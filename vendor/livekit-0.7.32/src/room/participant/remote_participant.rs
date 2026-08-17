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
    collections::{HashMap, HashSet},
    fmt::Debug,
    sync::Arc,
    time::Duration,
};

use libwebrtc::prelude::*;
use livekit_protocol as proto;
use livekit_runtime::timeout;
use parking_lot::Mutex;

use super::{
    ConnectionQuality, ParticipantInner, ParticipantKind, ParticipantKindDetail, TrackKind,
};
use crate::{
    prelude::*,
    rtc_engine::RtcEngine,
    track::{TrackError, VideoQuality},
};

const ADD_TRACK_TIMEOUT: Duration = Duration::from_secs(5);

type TrackPublishedHandler = Box<dyn Fn(RemoteParticipant, RemoteTrackPublication) + Send>;
type TrackUnpublishedHandler = Box<dyn Fn(RemoteParticipant, RemoteTrackPublication) + Send>;
type TrackSubscribedHandler =
    Box<dyn Fn(RemoteParticipant, RemoteTrackPublication, RemoteTrack) + Send>;
type TrackUnsubscribedHandler =
    Box<dyn Fn(RemoteParticipant, RemoteTrackPublication, RemoteTrack) + Send>;
type TrackSubscriptionFailedHandler = Box<dyn Fn(RemoteParticipant, TrackSid, TrackError) + Send>;

#[derive(Default)]
struct RemoteEvents {
    track_published: Mutex<Option<TrackPublishedHandler>>,
    track_unpublished: Mutex<Option<TrackUnpublishedHandler>>,
    track_subscribed: Mutex<Option<TrackSubscribedHandler>>,
    track_unsubscribed: Mutex<Option<TrackUnsubscribedHandler>>,
    track_subscription_failed: Mutex<Option<TrackSubscriptionFailedHandler>>,
}

struct RemoteInfo {
    events: Arc<RemoteEvents>,
    auto_subscribe: bool, // better way to access this from room?
}

#[derive(Clone)]
pub struct RemoteParticipant {
    inner: Arc<ParticipantInner>,
    remote: Arc<RemoteInfo>,
}

impl Debug for RemoteParticipant {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RemoteParticipant")
            .field("sid", &self.sid())
            .field("identity", &self.identity())
            .field("name", &self.name())
            .finish()
    }
}

impl RemoteParticipant {
    pub(crate) fn new(
        rtc_engine: Arc<RtcEngine>,
        kind: ParticipantKind,
        kind_details: Vec<ParticipantKindDetail>,
        sid: ParticipantSid,
        identity: ParticipantIdentity,
        name: String,
        metadata: String,
        attributes: HashMap<String, String>,
        auto_subscribe: bool,
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
            remote: Arc::new(RemoteInfo { events: Default::default(), auto_subscribe }),
        }
    }

    pub(crate) fn internal_track_publications(&self) -> HashMap<TrackSid, TrackPublication> {
        self.inner.track_publications.read().clone()
    }

    pub(crate) async fn add_subscribed_media_track(
        &self,
        sid: TrackSid,
        media_track: MediaStreamTrack,
        transceiver: RtpTransceiver,
    ) {
        let wait_publication = {
            let participant = self.clone();
            let sid = sid.clone();
            async move {
                loop {
                    let publication = participant.get_track_publication(&sid);
                    if let Some(publication) = publication {
                        return publication;
                    }

                    livekit_runtime::sleep(Duration::from_millis(50)).await;
                }
            }
        };

        if let Ok(remote_publication) = timeout(ADD_TRACK_TIMEOUT, wait_publication).await {
            let track = match remote_publication.kind() {
                TrackKind::Audio => {
                    if let MediaStreamTrack::Audio(rtc_track) = media_track {
                        let audio_track = RemoteAudioTrack::new(
                            remote_publication.sid(),
                            remote_publication.name(),
                            rtc_track,
                        );
                        RemoteTrack::Audio(audio_track)
                    } else {
                        unreachable!();
                    }
                }
                TrackKind::Video => {
                    if let MediaStreamTrack::Video(rtc_track) = media_track {
                        let video_track = RemoteVideoTrack::new(
                            remote_publication.sid(),
                            remote_publication.name(),
                            rtc_track,
                        );
                        RemoteTrack::Video(video_track)
                    } else {
                        unreachable!()
                    }
                }
            };

            track.set_transceiver(Some(transceiver));

            //track.set_muted(remote_publication.is_muted());
            track.update_info(proto::TrackInfo {
                sid: remote_publication.sid().to_string(),
                name: remote_publication.name(),
                r#type: proto::TrackType::from(remote_publication.kind()) as i32,
                source: proto::TrackSource::from(remote_publication.source()) as i32,
                muted: remote_publication.is_muted(),
                ..Default::default()
            });

            self.add_publication(TrackPublication::Remote(remote_publication.clone()));
            track.enable();

            remote_publication.set_track(Some(track)); // This will fire TrackSubscribed on the
                                                       // publication
        } else {
            log::error!("could not find published track with sid: {:?}", sid);

            if let Some(track_subscription_failed) =
                self.remote.events.track_subscription_failed.lock().as_ref()
            {
                track_subscription_failed(
                    self.clone(),
                    sid.clone(),
                    TrackError::TrackNotFound(sid),
                );
            }
        }
    }

    pub(crate) fn unpublish_track(&self, sid: &TrackSid) {
        if let Some(publication) = self.get_track_publication(sid) {
            // Unsubscribe to the track if needed
            if let Some(track) = publication.track() {
                track.disable();
                publication.set_track(None); // This will fire TrackUnsubscribed on the publication
            }

            self.remove_publication(sid);

            if let Some(track_unpublished) = self.remote.events.track_unpublished.lock().as_ref() {
                track_unpublished(self.clone(), publication);
            }
        }
    }

    pub(crate) fn update_info(&self, info: proto::ParticipantInfo) {
        super::update_info(&self.inner, &Participant::Remote(self.clone()), info.clone());

        let mut valid_tracks = HashSet::<TrackSid>::new();
        for track in info.tracks {
            let track_sid = track.sid.clone().try_into().unwrap();
            if let Some(publication) = self.get_track_publication(&track_sid) {
                publication.update_info(track.clone());
            } else {
                let publication =
                    RemoteTrackPublication::new(track.clone(), None, self.remote.auto_subscribe);

                self.add_publication(TrackPublication::Remote(publication.clone()));

                // This is a new track, dispatch publish event
                if let Some(track_published) = self.remote.events.track_published.lock().as_ref() {
                    track_published(self.clone(), publication);
                }
            }

            valid_tracks.insert(track_sid);
        }

        // remove tracks that are no longer valid
        let tracks = self.inner.track_publications.read().clone();
        for sid in tracks.keys() {
            if valid_tracks.contains(sid) {
                continue;
            }

            self.unpublish_track(sid);
        }
    }

    pub(crate) fn on_track_published(
        &self,
        track_published: impl Fn(RemoteParticipant, RemoteTrackPublication) + Send + 'static,
    ) {
        *self.remote.events.track_published.lock() = Some(Box::new(track_published));
    }

    pub(crate) fn on_track_unpublished(
        &self,
        track_unpublished: impl Fn(RemoteParticipant, RemoteTrackPublication) + Send + 'static,
    ) {
        *self.remote.events.track_unpublished.lock() = Some(Box::new(track_unpublished));
    }

    pub(crate) fn on_track_subscribed(
        &self,
        track_subscribed: impl Fn(RemoteParticipant, RemoteTrackPublication, RemoteTrack)
            + Send
            + 'static,
    ) {
        *self.remote.events.track_subscribed.lock() = Some(Box::new(track_subscribed));
    }

    pub(crate) fn on_track_unsubscribed(
        &self,
        track_unsubscribed: impl Fn(RemoteParticipant, RemoteTrackPublication, RemoteTrack)
            + Send
            + 'static,
    ) {
        *self.remote.events.track_unsubscribed.lock() = Some(Box::new(track_unsubscribed));
    }

    pub(crate) fn on_track_subscription_failed(
        &self,
        track_subscription_failed: impl Fn(RemoteParticipant, TrackSid, TrackError) + Send + 'static,
    ) {
        *self.remote.events.track_subscription_failed.lock() =
            Some(Box::new(track_subscription_failed));
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

    pub(crate) fn on_encryption_status_changed(
        &self,
        handler: impl Fn(Participant, bool) + Send + 'static,
    ) {
        super::on_encryption_status_changed(&self.inner, handler);
    }

    pub(crate) fn set_speaking(&self, speaking: bool) {
        super::set_speaking(&self.inner, &Participant::Remote(self.clone()), speaking);
    }

    pub(crate) fn set_audio_level(&self, level: f32) {
        super::set_audio_level(&self.inner, &Participant::Remote(self.clone()), level);
    }

    pub(crate) fn set_connection_quality(&self, quality: ConnectionQuality) {
        super::set_connection_quality(&self.inner, &Participant::Remote(self.clone()), quality);
    }

    pub(crate) fn add_publication(&self, publication: TrackPublication) {
        super::add_publication(
            &self.inner,
            &Participant::Remote(self.clone()),
            publication.clone(),
        );

        let TrackPublication::Remote(publication) = publication else {
            panic!("expected remote publication");
        };

        publication.on_subscription_update_needed({
            let rtc_engine = self.inner.rtc_engine.clone();
            let psid = self.sid();
            move |publication, subscribed| {
                let rtc_engine = rtc_engine.clone();
                let psid = psid.clone();
                livekit_runtime::spawn(async move {
                    let tsid: String = publication.sid().into();
                    let update_subscription = proto::UpdateSubscription {
                        track_sids: vec![tsid.clone()],
                        subscribe: subscribed,
                        participant_tracks: vec![proto::ParticipantTracks {
                            participant_sid: psid.into(),
                            track_sids: vec![tsid],
                        }],
                    };

                    let _ = rtc_engine
                        .send_request(proto::signal_request::Message::Subscription(
                            update_subscription,
                        ))
                        .await;
                });
            }
        });

        publication.on_subscribed({
            let events = self.remote.events.clone();
            let participant = self.clone();
            move |publication, track| {
                if let Some(track_subscribed) = events.track_subscribed.lock().as_ref() {
                    track_subscribed(participant.clone(), publication, track);
                }
            }
        });

        publication.on_unsubscribed({
            let events = self.remote.events.clone();
            let participant = self.clone();
            move |publication, track| {
                if let Some(track_unsubscribed) = events.track_unsubscribed.lock().as_ref() {
                    track_unsubscribed(participant.clone(), publication, track);
                }
            }
        });

        publication.on_enabled_status_changed({
            let rtc_engine = self.inner.rtc_engine.clone();
            move |publication, enabled| {
                let rtc_engine = rtc_engine.clone();
                livekit_runtime::spawn(async move {
                    let tsid: String = publication.sid().into();
                    let TrackDimension(width, height) = publication.dimension();
                    let update_track_settings = proto::UpdateTrackSettings {
                        track_sids: vec![tsid.clone()],
                        disabled: !enabled,
                        width,
                        height,
                        ..Default::default()
                    };

                    rtc_engine
                        .send_request(proto::signal_request::Message::TrackSetting(
                            update_track_settings,
                        ))
                        .await
                });
            }
        });

        publication.on_video_dimensions_changed({
            let rtc_engine = self.inner.rtc_engine.clone();
            move |publication, dimension| {
                let rtc_engine = rtc_engine.clone();
                livekit_runtime::spawn(async move {
                    let tsid: String = publication.sid().into();
                    let TrackDimension(width, height) = dimension;
                    let enabled = publication.is_enabled();
                    let update_track_settings = proto::UpdateTrackSettings {
                        track_sids: vec![tsid.clone()],
                        disabled: !enabled,
                        width,
                        height,
                        ..Default::default()
                    };

                    rtc_engine
                        .send_request(proto::signal_request::Message::TrackSetting(
                            update_track_settings,
                        ))
                        .await
                });
            }
        });

        publication.on_video_quality_changed({
            let rtc_engine = self.inner.rtc_engine.clone();
            move |publication, quality| {
                let rtc_engine = rtc_engine.clone();
                livekit_runtime::spawn(async move {
                    let tsid: String = publication.sid().into();
                    let quality = match quality {
                        VideoQuality::Low => proto::VideoQuality::Low,
                        VideoQuality::Medium => proto::VideoQuality::Medium,
                        VideoQuality::High => proto::VideoQuality::High,
                    }
                    .into();
                    let update_track_settings = proto::UpdateTrackSettings {
                        track_sids: vec![tsid.clone()],
                        quality,
                        ..Default::default()
                    };

                    rtc_engine
                        .send_request(proto::signal_request::Message::TrackSetting(
                            update_track_settings,
                        ))
                        .await
                });
            }
        });
    }

    pub(crate) fn remove_publication(&self, sid: &TrackSid) -> Option<TrackPublication> {
        let publication =
            super::remove_publication(&self.inner, &Participant::Remote(self.clone()), sid);

        if let Some(publication) = publication.clone() {
            let TrackPublication::Remote(publication) = publication else {
                panic!("expected remote publication");
            };

            publication.on_subscription_update_needed(|_, _| {});
            publication.on_subscribed(|_, _| {});
            publication.on_unsubscribed(|_, _| {});
        }

        publication
    }

    pub fn get_track_publication(&self, sid: &TrackSid) -> Option<RemoteTrackPublication> {
        self.inner.track_publications.read().get(sid).map(|track| {
            if let TrackPublication::Remote(remote) = track {
                return remote.clone();
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

    pub fn track_publications(&self) -> HashMap<TrackSid, RemoteTrackPublication> {
        self.inner
            .track_publications
            .read()
            .clone()
            .into_iter()
            .map(|(sid, track)| {
                if let TrackPublication::Remote(remote) = track {
                    return (sid, remote);
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

    pub fn is_encrypted(&self) -> bool {
        *self.inner.is_encrypted.read()
    }

    #[doc(hidden)]
    pub fn update_data_encryption_status(&self, is_encrypted: bool) {
        super::update_data_encryption_status(
            &self.inner,
            &super::Participant::Remote(self.clone()),
            is_encrypted,
        );
    }
}
