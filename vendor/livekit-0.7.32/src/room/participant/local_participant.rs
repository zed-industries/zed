// Copyright 2025 LiveKit, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use std::{
    collections::HashMap,
    fmt::Debug,
    future::Future,
    path::Path,
    pin::Pin,
    sync::{Arc, Weak},
    time::Duration,
};

use super::{
    ConnectionQuality, ParticipantInner, ParticipantKind, ParticipantKindDetail,
    ParticipantTrackPermission,
};
use crate::{
    data_stream::{
        ByteStreamInfo, ByteStreamWriter, StreamByteOptions, StreamResult, StreamTextOptions,
        TextStreamInfo, TextStreamWriter,
    },
    e2ee::EncryptionType,
    options::{self, compute_video_encodings, video_layers_from_encodings, TrackPublishOptions},
    prelude::*,
    room::participant::rpc::{RpcError, RpcErrorCode, RpcInvocationData, MAX_PAYLOAD_BYTES},
    rtc_engine::{EngineError, RtcEngine},
    ChatMessage, DataPacket, RoomSession, RpcAck, RpcRequest, RpcResponse, SipDTMF, Transcription,
};
use chrono::Utc;
use libwebrtc::{native::create_random_uuid, rtp_parameters::RtpEncodingParameters};
use livekit_api::signal_client::SignalError;
use livekit_protocol as proto;
use livekit_runtime::timeout;
use parking_lot::{Mutex, RwLock};
use proto::request_response::Reason;
use semver::Version;
use tokio::sync::oneshot;

type RpcHandler = Arc<
    dyn Fn(RpcInvocationData) -> Pin<Box<dyn Future<Output = Result<String, RpcError>> + Send>>
        + Send
        + Sync,
>;

const REQUEST_TIMEOUT: Duration = Duration::from_secs(5);

type LocalTrackPublishedHandler = Box<dyn Fn(LocalParticipant, LocalTrackPublication) + Send>;
type LocalTrackUnpublishedHandler = Box<dyn Fn(LocalParticipant, LocalTrackPublication) + Send>;

#[derive(Default)]
struct LocalEvents {
    local_track_published: Mutex<Option<LocalTrackPublishedHandler>>,
    local_track_unpublished: Mutex<Option<LocalTrackUnpublishedHandler>>,
}

struct RpcState {
    pending_acks: HashMap<String, oneshot::Sender<()>>,
    pending_responses: HashMap<String, oneshot::Sender<Result<String, RpcError>>>,
    handlers: HashMap<String, RpcHandler>,
}

impl RpcState {
    fn new() -> Self {
        Self {
            pending_acks: HashMap::new(),
            pending_responses: HashMap::new(),
            handlers: HashMap::new(),
        }
    }
}
struct LocalInfo {
    events: LocalEvents,
    encryption_type: EncryptionType,
    rpc_state: Mutex<RpcState>,
    all_participants_allowed: Mutex<bool>,
    track_permissions: Mutex<Vec<ParticipantTrackPermission>>,
    session: RwLock<Option<Weak<RoomSession>>>,
}

#[derive(Clone)]
pub struct LocalParticipant {
    inner: Arc<ParticipantInner>,
    local: Arc<LocalInfo>,
}

impl Debug for LocalParticipant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LocalParticipant")
            .field("sid", &self.sid())
            .field("identity", &self.identity())
            .field("name", &self.name())
            .finish()
    }
}

impl LocalParticipant {
    pub(crate) fn new(
        rtc_engine: Arc<RtcEngine>,
        kind: ParticipantKind,
        kind_details: Vec<ParticipantKindDetail>,
        sid: ParticipantSid,
        identity: ParticipantIdentity,
        name: String,
        metadata: String,
        attributes: HashMap<String, String>,
        encryption_type: EncryptionType,
        permission: Option<proto::ParticipantPermission>,
    ) -> Self {
        Self {
            inner: super::new_inner(
                rtc_engine,
                sid,
                identity,
                name,
                metadata,
                attributes,
                kind,
                kind_details,
                permission,
            ),
            local: Arc::new(LocalInfo {
                events: LocalEvents::default(),
                encryption_type,
                rpc_state: Mutex::new(RpcState::new()),
                all_participants_allowed: Mutex::new(true),
                track_permissions: Mutex::new(vec![]),
                session: Default::default(),
            }),
        }
    }

    pub(crate) fn set_session(&self, session: Weak<RoomSession>) {
        *self.local.session.write() = Some(session);
    }

    pub(crate) fn session(&self) -> Option<Arc<RoomSession>> {
        self.local.session.read().as_ref().and_then(|s| s.upgrade())
    }

    pub(crate) fn internal_track_publications(&self) -> HashMap<TrackSid, TrackPublication> {
        self.inner.track_publications.read().clone()
    }

    pub(crate) fn update_info(&self, info: proto::ParticipantInfo) {
        super::update_info(&self.inner, &Participant::Local(self.clone()), info);
    }

    pub(crate) fn set_speaking(&self, speaking: bool) {
        super::set_speaking(&self.inner, &Participant::Local(self.clone()), speaking);
    }

    pub(crate) fn set_audio_level(&self, level: f32) {
        super::set_audio_level(&self.inner, &Participant::Local(self.clone()), level);
    }

    pub(crate) fn set_connection_quality(&self, quality: ConnectionQuality) {
        super::set_connection_quality(&self.inner, &Participant::Local(self.clone()), quality);
    }

    pub(crate) fn on_local_track_published(
        &self,
        handler: impl Fn(LocalParticipant, LocalTrackPublication) + Send + 'static,
    ) {
        *self.local.events.local_track_published.lock() = Some(Box::new(handler));
    }

    pub(crate) fn on_local_track_unpublished(
        &self,
        handler: impl Fn(LocalParticipant, LocalTrackPublication) + Send + 'static,
    ) {
        *self.local.events.local_track_unpublished.lock() = Some(Box::new(handler));
    }

    pub(crate) fn on_track_muted(
        &self,
        handler: impl Fn(Participant, TrackPublication) + Send + 'static,
    ) {
        super::on_track_muted(&self.inner, handler)
    }

    pub(crate) fn on_track_unmuted(
        &self,
        handler: impl Fn(Participant, TrackPublication) + Send + 'static,
    ) {
        super::on_track_unmuted(&self.inner, handler)
    }

    pub(crate) fn on_metadata_changed(
        &self,
        handler: impl Fn(Participant, String, String) + Send + 'static,
    ) {
        super::on_metadata_changed(&self.inner, handler)
    }

    pub(crate) fn on_name_changed(
        &self,
        handler: impl Fn(Participant, String, String) + Send + 'static,
    ) {
        super::on_name_changed(&self.inner, handler)
    }

    pub(crate) fn on_attributes_changed(
        &self,
        handler: impl Fn(Participant, HashMap<String, String>) + Send + 'static,
    ) {
        super::on_attributes_changed(&self.inner, handler)
    }

    pub(crate) fn on_permission_changed(
        &self,
        handler: impl Fn(Participant, Option<proto::ParticipantPermission>) + Send + 'static,
    ) {
        super::on_permission_changed(&self.inner, handler)
    }

    pub(crate) fn add_publication(&self, publication: TrackPublication) {
        super::add_publication(&self.inner, &Participant::Local(self.clone()), publication);
    }

    pub(crate) fn remove_publication(&self, sid: &TrackSid) -> Option<TrackPublication> {
        super::remove_publication(&self.inner, &Participant::Local(self.clone()), sid)
    }

    pub(crate) fn published_tracks_info(&self) -> Vec<proto::TrackPublishedResponse> {
        let tracks = self.track_publications();
        let mut vec = Vec::with_capacity(tracks.len());

        for p in tracks.values() {
            if let Some(track) = p.track() {
                vec.push(proto::TrackPublishedResponse {
                    cid: track.rtc_track().id(),
                    track: Some(p.proto_info()),
                });
            }
        }

        vec
    }

    pub async fn publish_track(
        &self,
        track: LocalTrack,
        options: TrackPublishOptions,
    ) -> RoomResult<LocalTrackPublication> {
        let disable_red = self.local.encryption_type != EncryptionType::None || !options.red;

        let mut req = proto::AddTrackRequest {
            cid: track.rtc_track().id(),
            name: track.name(),
            r#type: proto::TrackType::from(track.kind()) as i32,
            muted: track.is_muted(),
            source: proto::TrackSource::from(options.source) as i32,
            disable_dtx: !options.dtx,
            disable_red,
            encryption: proto::encryption::Type::from(self.local.encryption_type) as i32,
            stream: options.stream.clone(),
            ..Default::default()
        };

        if options.preconnect_buffer {
            req.audio_features.push(proto::AudioTrackFeature::TfPreconnectBuffer as i32);
        }

        let mut encodings = Vec::default();
        match &track {
            LocalTrack::Video(video_track) => {
                // Get the video dimension
                // TODO(theomonnom): Use MediaStreamTrack::getSettings() on web
                let resolution = video_track.rtc_source().video_resolution();
                req.width = resolution.width;
                req.height = resolution.height;

                encodings = compute_video_encodings(req.width, req.height, &options);
                req.layers = video_layers_from_encodings(req.width, req.height, &encodings);

                // Populate simulcast_codecs so the server knows this track is simulcasted
                if options.simulcast && encodings.len() > 1 {
                    req.simulcast_codecs = vec![proto::SimulcastCodec {
                        codec: options.video_codec.as_str().to_string(),
                        cid: track.rtc_track().id(),
                        layers: req.layers.clone(),
                        ..Default::default()
                    }];
                }
            }
            LocalTrack::Audio(_audio_track) => {
                // Setup audio encoding
                let audio_encoding =
                    options.audio_encoding.as_ref().unwrap_or(&options::audio::MUSIC.encoding);

                encodings.push(RtpEncodingParameters {
                    max_bitrate: Some(audio_encoding.max_bitrate),
                    ..Default::default()
                });
            }
        }
        let track_info = self.inner.rtc_engine.add_track(req).await?;
        let publication = LocalTrackPublication::new(track_info.clone(), track.clone());
        track.update_info(track_info); // Update sid + source

        // set track for publication to listen mute/unmute events
        publication.set_track(Some(track.clone().into()));

        let transceiver =
            self.inner.rtc_engine.create_sender(track.clone(), options.clone(), encodings).await?;

        track.set_transceiver(Some(transceiver));

        self.inner.rtc_engine.publisher_negotiation_needed();

        publication.update_publish_options(options);
        self.add_publication(TrackPublication::Local(publication.clone()));

        if let Some(local_track_published) = self.local.events.local_track_published.lock().as_ref()
        {
            local_track_published(self.clone(), publication.clone());
        }
        track.enable();

        Ok(publication)
    }

    pub async fn set_metadata(&self, metadata: String) -> RoomResult<()> {
        if let Ok(response) = timeout(REQUEST_TIMEOUT, {
            let request_id = self.inner.rtc_engine.session().signal_client().next_request_id();
            self.inner
                .rtc_engine
                .send_request(proto::signal_request::Message::UpdateMetadata(
                    proto::UpdateParticipantMetadata {
                        metadata,
                        name: self.name(),
                        attributes: Default::default(),
                        request_id,
                        ..Default::default()
                    },
                ))
                .await;
            self.inner.rtc_engine.get_response(request_id)
        })
        .await
        {
            match response.reason() {
                Reason::Ok => Ok(()),
                reason => Err(RoomError::Request { reason, message: response.message }),
            }
        } else {
            Err(RoomError::Engine(EngineError::Signal(SignalError::Timeout(
                "request timeout".into(),
            ))))
        }
    }

    pub async fn set_attributes(&self, attributes: HashMap<String, String>) -> RoomResult<()> {
        if let Ok(response) = timeout(REQUEST_TIMEOUT, {
            let request_id = self.inner.rtc_engine.session().signal_client().next_request_id();
            self.inner
                .rtc_engine
                .send_request(proto::signal_request::Message::UpdateMetadata(
                    proto::UpdateParticipantMetadata {
                        attributes,
                        metadata: self.metadata(),
                        name: self.name(),
                        request_id,
                        ..Default::default()
                    },
                ))
                .await;
            self.inner.rtc_engine.get_response(request_id)
        })
        .await
        {
            match response.reason() {
                Reason::Ok => Ok(()),
                reason => Err(RoomError::Request { reason, message: response.message }),
            }
        } else {
            Err(RoomError::Engine(EngineError::Signal(SignalError::Timeout(
                "request timeout".into(),
            ))))
        }
    }

    pub async fn set_name(&self, name: String) -> RoomResult<()> {
        if let Ok(response) = timeout(REQUEST_TIMEOUT, {
            let request_id = self.inner.rtc_engine.session().signal_client().next_request_id();
            self.inner
                .rtc_engine
                .send_request(proto::signal_request::Message::UpdateMetadata(
                    proto::UpdateParticipantMetadata {
                        name,
                        metadata: self.metadata(),
                        attributes: Default::default(),
                        request_id,
                        ..Default::default()
                    },
                ))
                .await;
            self.inner.rtc_engine.get_response(request_id)
        })
        .await
        {
            match response.reason() {
                Reason::Ok => Ok(()),
                reason => Err(RoomError::Request { reason, message: response.message }),
            }
        } else {
            Err(RoomError::Engine(EngineError::Signal(SignalError::Timeout(
                "request timeout".into(),
            ))))
        }
    }

    pub async fn send_chat_message(
        &self,
        text: String,
        destination_identities: Option<Vec<String>>,
        sender_identity: Option<String>,
    ) -> RoomResult<ChatMessage> {
        let chat_message = proto::ChatMessage {
            id: create_random_uuid(),
            timestamp: Utc::now().timestamp_millis(),
            message: text,
            ..Default::default()
        };

        let data = proto::DataPacket {
            value: Some(proto::data_packet::Value::ChatMessage(chat_message.clone())),
            participant_identity: sender_identity.unwrap_or_default(),
            destination_identities: destination_identities.unwrap_or_default(),
            ..Default::default()
        };

        match self.inner.rtc_engine.publish_data(data, DataPacketKind::Reliable, false).await {
            Ok(_) => Ok(ChatMessage::from(chat_message)),
            Err(e) => Err(Into::into(e)),
        }
    }

    pub async fn edit_chat_message(
        &self,
        edit_text: String,
        original_message: ChatMessage,
        destination_identities: Option<Vec<String>>,
        sender_identity: Option<String>,
    ) -> RoomResult<ChatMessage> {
        let edited_message = ChatMessage {
            message: edit_text,
            edit_timestamp: Utc::now().timestamp_millis().into(),
            ..original_message
        };
        let proto_msg = proto::ChatMessage::from(edited_message);
        let data = proto::DataPacket {
            value: Some(proto::data_packet::Value::ChatMessage(proto_msg.clone())),
            participant_identity: sender_identity.unwrap_or_default(),
            destination_identities: destination_identities.unwrap_or_default(),
            ..Default::default()
        };

        match self.inner.rtc_engine.publish_data(data, DataPacketKind::Reliable, false).await {
            Ok(_) => Ok(ChatMessage::from(proto_msg)),
            Err(e) => Err(Into::into(e)),
        }
    }

    pub async fn unpublish_track(
        &self,
        sid: &TrackSid,
        // _stop_on_unpublish: bool,
    ) -> RoomResult<LocalTrackPublication> {
        let publication = self.remove_publication(sid);
        if let Some(TrackPublication::Local(publication)) = publication {
            let track = publication.track().unwrap();
            let sender = track.transceiver().unwrap().sender();

            self.inner.rtc_engine.remove_track(sender)?;
            track.set_transceiver(None);

            if let Some(local_track_unpublished) =
                self.local.events.local_track_unpublished.lock().as_ref()
            {
                local_track_unpublished(self.clone(), publication.clone());
            }

            publication.set_track(None);
            self.inner.rtc_engine.publisher_negotiation_needed();

            Ok(publication)
        } else {
            Err(RoomError::Internal("track not found".to_string()))
        }
    }

    /** internal */
    pub async fn publish_raw_data(
        self,
        packet: proto::DataPacket,
        reliable: bool,
    ) -> RoomResult<()> {
        let kind = match reliable {
            true => DataPacketKind::Reliable,
            false => DataPacketKind::Lossy,
        };
        self.inner.rtc_engine.publish_data(packet, kind, true).await.map_err(Into::into)
    }

    pub async fn publish_data(&self, packet: DataPacket) -> RoomResult<()> {
        let kind = match packet.reliable {
            true => DataPacketKind::Reliable,
            false => DataPacketKind::Lossy,
        };
        let destination_identities: Vec<String> =
            packet.destination_identities.into_iter().map(Into::into).collect();
        let data = proto::DataPacket {
            kind: kind as i32,
            destination_identities: destination_identities.clone(),
            value: Some(proto::data_packet::Value::User(proto::UserPacket {
                payload: packet.payload,
                topic: packet.topic,
                ..Default::default()
            })),
            ..Default::default()
        };

        self.inner.rtc_engine.publish_data(data, kind, false).await.map_err(Into::into)
    }

    pub fn set_data_channel_buffered_amount_low_threshold(
        &self,
        threshold: u64,
        kind: DataPacketKind,
    ) -> RoomResult<()> {
        self.inner
            .rtc_engine
            .session()
            .set_data_channel_buffered_amount_low_threshold(threshold, kind);
        Ok(())
    }

    pub fn data_channel_buffered_amount_low_threshold(
        &self,
        kind: DataPacketKind,
    ) -> RoomResult<u64> {
        Ok(self.inner.rtc_engine.session().data_channel_buffered_amount_low_threshold(kind))
    }

    pub async fn set_track_subscription_permissions(
        &self,
        all_participants_allowed: bool,
        permissions: Vec<ParticipantTrackPermission>,
    ) -> RoomResult<()> {
        *self.local.track_permissions.lock() = permissions;
        *self.local.all_participants_allowed.lock() = all_participants_allowed;
        self.update_track_subscription_permissions().await;
        Ok(())
    }

    pub async fn publish_transcription(&self, packet: Transcription) -> RoomResult<()> {
        let segments: Vec<proto::TranscriptionSegment> = packet
            .segments
            .into_iter()
            .map(|segment| proto::TranscriptionSegment {
                id: segment.id,
                start_time: segment.start_time,
                end_time: segment.end_time,
                text: segment.text,
                r#final: segment.r#final,
                language: segment.language,
            })
            .collect();
        let transcription_packet = proto::Transcription {
            transcribed_participant_identity: packet.participant_identity,
            segments: segments,
            track_id: packet.track_id,
        };
        let data = proto::DataPacket {
            value: Some(proto::data_packet::Value::Transcription(transcription_packet)),
            ..Default::default()
        };
        self.inner
            .rtc_engine
            .publish_data(data, DataPacketKind::Reliable, false)
            .await
            .map_err(Into::into)
    }

    pub async fn publish_dtmf(&self, dtmf: SipDTMF) -> RoomResult<()> {
        let destination_identities: Vec<String> =
            dtmf.destination_identities.into_iter().map(Into::into).collect();
        let dtmf_message = proto::SipDtmf { code: dtmf.code, digit: dtmf.digit };

        let data = proto::DataPacket {
            value: Some(proto::data_packet::Value::SipDtmf(dtmf_message)),
            destination_identities: destination_identities.clone(),
            ..Default::default()
        };

        self.inner
            .rtc_engine
            .publish_data(data, DataPacketKind::Reliable, false)
            .await
            .map_err(Into::into)
    }

    async fn publish_rpc_request(&self, rpc_request: RpcRequest) -> RoomResult<()> {
        let destination_identities = vec![rpc_request.destination_identity];
        let rpc_request_message = proto::RpcRequest {
            id: rpc_request.id,
            method: rpc_request.method,
            payload: rpc_request.payload,
            response_timeout_ms: rpc_request.response_timeout.as_millis() as u32,
            version: rpc_request.version,
            ..Default::default()
        };

        let data = proto::DataPacket {
            value: Some(proto::data_packet::Value::RpcRequest(rpc_request_message)),
            destination_identities,
            ..Default::default()
        };

        self.inner
            .rtc_engine
            .publish_data(data, DataPacketKind::Reliable, false)
            .await
            .map_err(Into::into)
    }

    async fn publish_rpc_response(&self, rpc_response: RpcResponse) -> RoomResult<()> {
        let destination_identities = vec![rpc_response.destination_identity];
        let rpc_response_message = proto::RpcResponse {
            request_id: rpc_response.request_id,
            value: Some(match rpc_response.error {
                Some(error) => proto::rpc_response::Value::Error(proto::RpcError {
                    code: error.code,
                    message: error.message,
                    data: error.data,
                }),
                None => proto::rpc_response::Value::Payload(rpc_response.payload.unwrap()),
            }),
            ..Default::default()
        };

        let data = proto::DataPacket {
            value: Some(proto::data_packet::Value::RpcResponse(rpc_response_message)),
            destination_identities: destination_identities.clone(),
            ..Default::default()
        };

        self.inner
            .rtc_engine
            .publish_data(data, DataPacketKind::Reliable, false)
            .await
            .map_err(Into::into)
    }

    async fn publish_rpc_ack(&self, rpc_ack: RpcAck) -> RoomResult<()> {
        let destination_identities = vec![rpc_ack.destination_identity];
        let rpc_ack_message =
            proto::RpcAck { request_id: rpc_ack.request_id, ..Default::default() };

        let data = proto::DataPacket {
            value: Some(proto::data_packet::Value::RpcAck(rpc_ack_message)),
            destination_identities: destination_identities.clone(),
            ..Default::default()
        };

        self.inner
            .rtc_engine
            .publish_data(data, DataPacketKind::Reliable, false)
            .await
            .map_err(Into::into)
    }

    pub(crate) async fn update_track_subscription_permissions(&self) {
        let all_participants_allowed = *self.local.all_participants_allowed.lock();
        let track_permissions = self
            .local
            .track_permissions
            .lock()
            .iter()
            .map(|p| proto::TrackPermission::from(p.clone()))
            .collect();

        self.inner
            .rtc_engine
            .send_request(proto::signal_request::Message::SubscriptionPermission(
                proto::SubscriptionPermission {
                    all_participants: all_participants_allowed,
                    track_permissions,
                },
            ))
            .await;
    }

    pub fn get_track_publication(&self, sid: &TrackSid) -> Option<LocalTrackPublication> {
        self.inner.track_publications.read().get(sid).map(|track| {
            if let TrackPublication::Local(local) = track {
                return local.clone();
            }

            unreachable!()
        })
    }

    pub fn sid(&self) -> ParticipantSid {
        self.inner.info.read().sid.clone()
    }

    pub fn identity(&self) -> ParticipantIdentity {
        self.inner.info.read().identity.clone()
    }

    pub fn name(&self) -> String {
        self.inner.info.read().name.clone()
    }

    pub fn metadata(&self) -> String {
        self.inner.info.read().metadata.clone()
    }

    pub fn attributes(&self) -> HashMap<String, String> {
        self.inner.info.read().attributes.clone()
    }

    pub fn is_speaking(&self) -> bool {
        self.inner.info.read().speaking
    }

    pub fn track_publications(&self) -> HashMap<TrackSid, LocalTrackPublication> {
        self.inner
            .track_publications
            .read()
            .clone()
            .into_iter()
            .map(|(sid, track)| {
                if let TrackPublication::Local(local) = track {
                    return (sid, local);
                }

                unreachable!()
            })
            .collect()
    }

    pub fn audio_level(&self) -> f32 {
        self.inner.info.read().audio_level
    }

    pub fn connection_quality(&self) -> ConnectionQuality {
        self.inner.info.read().connection_quality
    }

    pub fn kind(&self) -> ParticipantKind {
        self.inner.info.read().kind
    }

    pub fn kind_details(&self) -> Vec<ParticipantKindDetail> {
        self.inner.info.read().kind_details.clone()
    }

    pub fn disconnect_reason(&self) -> DisconnectReason {
        self.inner.info.read().disconnect_reason
    }

    pub fn permission(&self) -> Option<proto::ParticipantPermission> {
        self.inner.info.read().permission.clone()
    }

    pub async fn perform_rpc(&self, data: PerformRpcData) -> Result<String, RpcError> {
        // Maximum amount of time it should ever take for an RPC request to reach the destination, and the ACK to come back
        // This is set to 7 seconds to account for various relay timeouts and retries in LiveKit Cloud that occur in rare cases

        let max_round_trip_latency = Duration::from_millis(7000);
        let min_effective_timeout = Duration::from_millis(1000);

        if data.payload.len() > MAX_PAYLOAD_BYTES {
            return Err(RpcError::built_in(RpcErrorCode::RequestPayloadTooLarge, None));
        }

        if let Some(server_info) =
            self.inner.rtc_engine.session().signal_client().join_response().server_info
        {
            if !server_info.version.is_empty() {
                let server_version = Version::parse(&server_info.version).unwrap();
                let min_required_version = Version::parse("1.8.0").unwrap();
                if server_version < min_required_version {
                    return Err(RpcError::built_in(RpcErrorCode::UnsupportedServer, None));
                }
            }
        }

        let id = create_random_uuid();
        let (ack_tx, ack_rx) = oneshot::channel();
        let (response_tx, response_rx) = oneshot::channel();
        let effective_timeout = std::cmp::max(
            data.response_timeout.saturating_sub(max_round_trip_latency),
            min_effective_timeout,
        );

        // Register channels BEFORE sending the request to avoid race condition
        // where the response arrives before we've registered the handlers
        {
            let mut rpc_state = self.local.rpc_state.lock();
            rpc_state.pending_acks.insert(id.clone(), ack_tx);
            rpc_state.pending_responses.insert(id.clone(), response_tx);
        }

        if let Err(e) = self
            .publish_rpc_request(RpcRequest {
                destination_identity: data.destination_identity.clone(),
                id: id.clone(),
                method: data.method.clone(),
                payload: data.payload.clone(),
                response_timeout: effective_timeout,
                version: 1,
            })
            .await
        {
            // Clean up on failure
            let mut rpc_state = self.local.rpc_state.lock();
            rpc_state.pending_acks.remove(&id);
            rpc_state.pending_responses.remove(&id);
            log::error!("Failed to publish RPC request: {}", e);
            return Err(RpcError::built_in(RpcErrorCode::SendFailed, Some(e.to_string())));
        }

        // Wait for ack timeout
        match tokio::time::timeout(max_round_trip_latency, ack_rx).await {
            Err(_) => {
                let mut rpc_state = self.local.rpc_state.lock();
                rpc_state.pending_acks.remove(&id);
                rpc_state.pending_responses.remove(&id);
                return Err(RpcError::built_in(RpcErrorCode::ConnectionTimeout, None));
            }
            Ok(_) => {
                // Ack received, continue to wait for response
            }
        }

        // Wait for response timout
        let response = match tokio::time::timeout(data.response_timeout, response_rx).await {
            Err(_) => {
                self.local.rpc_state.lock().pending_responses.remove(&id);
                return Err(RpcError::built_in(RpcErrorCode::ResponseTimeout, None));
            }
            Ok(result) => result,
        };

        match response {
            Err(_) => {
                // Something went wrong locally
                Err(RpcError::built_in(RpcErrorCode::RecipientDisconnected, None))
            }
            Ok(Err(e)) => {
                // RPC error from remote, forward it
                Err(e)
            }
            Ok(Ok(payload)) => {
                // Successful response
                Ok(payload)
            }
        }
    }

    pub fn register_rpc_method(
        &self,
        method: String,
        handler: impl Fn(RpcInvocationData) -> Pin<Box<dyn Future<Output = Result<String, RpcError>> + Send>>
            + Send
            + Sync
            + 'static,
    ) {
        self.local.rpc_state.lock().handlers.insert(method, Arc::new(handler));

        // Pre-connect the publisher PC so ACKs can be sent immediately when requests arrive.
        // Without this, the first RPC request would trigger publisher negotiation, causing
        // a ~300-500ms delay before the ACK can be sent (ICE negotiation time).
        self.inner.rtc_engine.publisher_negotiation_needed();
    }

    pub fn unregister_rpc_method(&self, method: String) {
        self.local.rpc_state.lock().handlers.remove(&method);
    }

    pub(crate) fn handle_incoming_rpc_ack(&self, request_id: String) {
        let mut rpc_state = self.local.rpc_state.lock();
        if let Some(tx) = rpc_state.pending_acks.remove(&request_id) {
            let _ = tx.send(());
        } else {
            log::error!("Ack received for unexpected RPC request: {}", request_id);
        }
    }

    pub(crate) fn handle_incoming_rpc_response(
        &self,
        request_id: String,
        payload: Option<String>,
        error: Option<proto::RpcError>,
    ) {
        let mut rpc_state = self.local.rpc_state.lock();
        if let Some(tx) = rpc_state.pending_responses.remove(&request_id) {
            let _ = tx.send(match error {
                Some(e) => Err(RpcError::from_proto(e)),
                None => Ok(payload.unwrap_or_default()),
            });
        } else {
            log::error!("Response received for unexpected RPC request: {}", request_id);
        }
    }

    pub(crate) async fn handle_incoming_rpc_request(
        &self,
        caller_identity: ParticipantIdentity,
        request_id: String,
        method: String,
        payload: String,
        response_timeout: Duration,
        version: u32,
    ) {
        if let Err(e) = self
            .publish_rpc_ack(RpcAck {
                destination_identity: caller_identity.to_string(),
                request_id: request_id.clone(),
            })
            .await
        {
            log::error!("Failed to publish RPC ACK: {:?}", e);
        }

        let caller_identity_2 = caller_identity.clone();
        let request_id_2 = request_id.clone();

        let response = if version != 1 {
            Err(RpcError::built_in(RpcErrorCode::UnsupportedVersion, None))
        } else {
            let handler = self.local.rpc_state.lock().handlers.get(&method).cloned();

            match handler {
                Some(handler) => {
                    match tokio::task::spawn(async move {
                        handler(RpcInvocationData {
                            request_id: request_id.clone(),
                            caller_identity: caller_identity.clone(),
                            payload: payload.clone(),
                            response_timeout,
                        })
                        .await
                    })
                    .await
                    {
                        Ok(result) => result,
                        Err(e) => {
                            log::error!("RPC method handler returned an error: {:?}", e);
                            Err(RpcError::built_in(RpcErrorCode::ApplicationError, None))
                        }
                    }
                }
                None => Err(RpcError::built_in(RpcErrorCode::UnsupportedMethod, None)),
            }
        };

        let (payload, error) = match response {
            Ok(response_payload) if response_payload.len() <= MAX_PAYLOAD_BYTES => {
                (Some(response_payload), None)
            }
            Ok(_) => (None, Some(RpcError::built_in(RpcErrorCode::ResponsePayloadTooLarge, None))),
            Err(e) => (None, Some(e.into())),
        };

        if let Err(e) = self
            .publish_rpc_response(RpcResponse {
                destination_identity: caller_identity_2.to_string(),
                request_id: request_id_2,
                payload,
                error: error.map(|e| e.to_proto()),
            })
            .await
        {
            log::error!("Failed to publish RPC response: {:?}", e);
        }
    }

    /// Send text to participants in the room.
    ///
    /// This method sends a complete text string to participants in the room as a text stream.
    /// The text is sent in a single operation, and the method returns information about the
    /// stream used to send the text.
    ///
    /// # Arguments
    ///
    /// * `text` - The text content to send.
    /// * `options` - Configuration options for the text stream, including topic and
    ///   destination participants.
    ///
    pub async fn send_text(
        &self,
        text: &str,
        options: StreamTextOptions,
    ) -> StreamResult<TextStreamInfo> {
        self.session().unwrap().outgoing_stream_manager.send_text(text, options).await
    }

    /// Send a file on disk to participants in the room.
    ///
    /// This method reads a file from the specified path and sends its contents
    /// to participants in the room as a byte stream, and the method returns information
    /// the stream used to send the file.
    ///
    /// # Arguments
    ///
    /// * `path` - Path to the file to be sent.
    /// * `options` - Configuration options for the byte stream, including topic and
    ///   destination participants.
    ///
    pub async fn send_file(
        &self,
        path: impl AsRef<Path>,
        options: StreamByteOptions,
    ) -> StreamResult<ByteStreamInfo> {
        self.session().unwrap().outgoing_stream_manager.send_file(path, options).await
    }

    /// Send an in-memory blob of bytes to participants in the room.
    ///
    /// This method sends a provided byte slice as a byte stream.
    ///
    /// # Arguments
    ///
    /// * `data` - The bytes to send.
    /// * `options` - Configuration options for the byte stream, including topic and
    ///   destination participants.
    pub async fn send_bytes(
        &self,
        data: impl AsRef<[u8]>,
        options: StreamByteOptions,
    ) -> StreamResult<ByteStreamInfo> {
        self.session().unwrap().outgoing_stream_manager.send_bytes(data, options).await
    }

    /// Stream text incrementally to participants in the room.
    ///
    /// This method allows sending text data in chunks as it becomes available.
    /// Unlike `send_text`, which sends the entire text at once, this method returns
    /// a writer that can be used to send text incrementally.
    ///
    /// # Arguments
    ///
    /// * `options` - Configuration options for the text stream, including topic and
    ///   destination participants.
    ///
    pub async fn stream_text(&self, options: StreamTextOptions) -> StreamResult<TextStreamWriter> {
        self.session().unwrap().outgoing_stream_manager.stream_text(options).await
    }

    /// Stream bytes incrementally to participants in the room.
    ///
    /// This method allows sending binary data in chunks as it becomes available.
    /// Unlike `send_file`, which sends the entire file at once, this method returns
    /// a writer that can be used to send binary data incrementally.
    ///
    /// # Arguments
    ///
    /// * `options` - Configuration options for the byte stream, including topic and
    ///   destination participants.
    ///
    pub async fn stream_bytes(&self, options: StreamByteOptions) -> StreamResult<ByteStreamWriter> {
        self.session().unwrap().outgoing_stream_manager.stream_bytes(options).await
    }

    pub fn is_encrypted(&self) -> bool {
        *self.inner.is_encrypted.read()
    }

    #[doc(hidden)]
    pub fn update_data_encryption_status(&self, _is_encrypted: bool) {
        // Local participants don't receive data messages, so this is a no-op
    }
}
