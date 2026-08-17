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
    collections::{HashMap, VecDeque},
    convert::TryInto,
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, AtomicU32, AtomicU64, Ordering},
        Arc,
    },
    time::Duration,
};

use libwebrtc::{prelude::*, stats::RtcStats};
use livekit_api::signal_client::{SignalClient, SignalEvent, SignalEvents};
use livekit_protocol::{self as proto};
use livekit_runtime::{sleep, JoinHandle};
use parking_lot::Mutex;
use prost::Message;
use proto::{
    debouncer::{self, Debouncer},
    SignalTarget,
};
use serde::{Deserialize, Serialize};
use tokio::sync::{mpsc, oneshot, watch, Notify};

use super::{rtc_events, EngineError, EngineOptions, EngineResult, SimulateScenario};
use crate::{
    id::ParticipantIdentity,
    utils::{
        ttl_map::TtlMap,
        tx_queue::{TxQueue, TxQueueItem},
    },
    ChatMessage, TranscriptionSegment,
};
use crate::{
    id::ParticipantSid,
    options::TrackPublishOptions,
    prelude::TrackKind,
    room::{e2ee::manager::E2eeManager, DisconnectReason},
    rtc_engine::{
        lk_runtime::LkRuntime,
        peer_transport::PeerTransport,
        rtc_events::{RtcEvent, RtcEvents},
    },
    track::LocalTrack,
    DataPacketKind,
};

pub const ICE_CONNECT_TIMEOUT: Duration = Duration::from_secs(15);
pub const TRACK_PUBLISH_TIMEOUT: Duration = Duration::from_secs(10);
pub const LOSSY_DC_LABEL: &str = "_lossy";
pub const RELIABLE_DC_LABEL: &str = "_reliable";
pub const RELIABLE_RECEIVED_STATE_TTL: Duration = Duration::from_secs(30);
pub const PUBLISHER_NEGOTIATION_FREQUENCY: Duration = Duration::from_millis(150);
pub const INITIAL_BUFFERED_AMOUNT_LOW_THRESHOLD: u64 = 2 * 1024 * 1024;

#[derive(Debug)]
enum NegotiationState {
    Idle,
    InProgress,
    PendingRetry,
}

struct NegotiationQueue {
    state: Arc<Mutex<NegotiationState>>,
    waker: Arc<Notify>,
    task_running: AtomicBool,
    waiting_for_answer: AtomicBool,
}

impl NegotiationQueue {
    fn new() -> Self {
        Self {
            state: Arc::new(Mutex::new(NegotiationState::Idle)),
            waker: Arc::new(Notify::new()),
            task_running: AtomicBool::new(false),
            waiting_for_answer: AtomicBool::new(false),
        }
    }
}

pub type SessionEmitter = mpsc::UnboundedSender<SessionEvent>;
pub type SessionEvents = mpsc::UnboundedReceiver<SessionEvent>;

#[derive(Debug, Clone)]
pub struct SessionStats {
    pub publisher_stats: Vec<RtcStats>,
    pub subscriber_stats: Vec<RtcStats>,
}

#[derive(Debug)]
pub enum SessionEvent {
    ParticipantUpdate {
        updates: Vec<proto::ParticipantInfo>,
    },
    Data {
        // None when the data comes from the ServerSDK (So no real participant)
        participant_sid: Option<ParticipantSid>,
        participant_identity: Option<ParticipantIdentity>,
        payload: Vec<u8>,
        topic: Option<String>,
        kind: DataPacketKind,
        encryption_type: proto::encryption::Type,
    },
    ChatMessage {
        participant_identity: ParticipantIdentity,
        message: ChatMessage,
    },
    Transcription {
        participant_identity: ParticipantIdentity,
        track_sid: String,
        segments: Vec<TranscriptionSegment>,
    },
    SipDTMF {
        // None when the data comes from the ServerSDK (So no real participant)
        participant_identity: Option<ParticipantIdentity>,
        code: u32,
        digit: Option<String>,
    },
    RpcRequest {
        caller_identity: Option<ParticipantIdentity>,
        request_id: String,
        method: String,
        payload: String,
        response_timeout: Duration,
        version: u32,
    },
    RpcResponse {
        request_id: String,
        payload: Option<String>,
        error: Option<proto::RpcError>,
    },
    RpcAck {
        request_id: String,
    },
    MediaTrack {
        track: MediaStreamTrack,
        stream: MediaStream,
        transceiver: RtpTransceiver,
    },
    SpeakersChanged {
        speakers: Vec<proto::SpeakerInfo>,
    },
    ConnectionQuality {
        updates: Vec<proto::ConnectionQualityInfo>,
    },
    RoomUpdate {
        room: proto::Room,
    },
    RoomMoved {
        moved: proto::RoomMovedResponse,
    },
    LocalTrackSubscribed {
        track_sid: String,
    },
    Close {
        source: String,
        reason: DisconnectReason,
        action: proto::leave_request::Action,
        retry_now: bool,
    },
    DataStreamHeader {
        header: proto::data_stream::Header,
        participant_identity: String,
        encryption_type: proto::encryption::Type,
    },
    DataStreamChunk {
        chunk: proto::data_stream::Chunk,
        participant_identity: String,
        encryption_type: proto::encryption::Type,
    },
    DataStreamTrailer {
        trailer: proto::data_stream::Trailer,
        participant_identity: String,
    },
    DataChannelBufferedAmountLowThresholdChanged {
        kind: DataPacketKind,
        threshold: u64,
    },
    RefreshToken {
        url: String,
        token: String,
    },
    TrackMuted {
        sid: String,
        muted: bool,
    },
}

#[derive(Debug)]
struct DataChannelEvent {
    kind: DataPacketKind,
    detail: DataChannelEventDetail,
}

#[derive(Debug)]
enum DataChannelEventDetail {
    /// Publish data packet.
    PublishPacket(PublishPacketRequest),
    /// Publish data packet that has already been encoded.
    PublishData(PublishDataRequest),
    /// RTC buffered amount changed.
    BufferedAmountChange(u64),
    /// Enqueue reliable packets for retry starting from the given sequence number.
    RetryFrom(u32),
}

#[derive(Debug)]
struct PublishPacketRequest {
    /// Unencoded data packet.
    packet: proto::DataPacket,

    /// Notifies the caller once the request has been fulfilled.
    completion_tx: oneshot::Sender<Result<(), EngineError>>,
}

#[derive(Debug)]
struct PublishDataRequest {
    /// Encoded data packet.
    encoded_packet: EncodedPacket,

    /// Notifies the caller once the request has been fulfilled.
    ///
    /// For retries, this will be `None`.
    ///
    completion_tx: Option<oneshot::Sender<Result<(), EngineError>>>,
}

#[derive(Debug)]
struct EncodedPacket {
    /// Encoded packet data.
    data: Vec<u8>,
    /// Packet's sequence number from [`proto::DataPacket::sequence`].
    sequence: u32,
}

impl Into<EncodedPacket> for proto::DataPacket {
    fn into(self) -> EncodedPacket {
        EncodedPacket { data: self.encode_to_vec(), sequence: self.sequence }
    }
}

fn convert_encrypted_to_data_packet_value(
    encrypted_value: proto::encrypted_packet_payload::Value,
) -> proto::data_packet::Value {
    match encrypted_value {
        proto::encrypted_packet_payload::Value::User(user) => proto::data_packet::Value::User(user),
        proto::encrypted_packet_payload::Value::ChatMessage(msg) => {
            proto::data_packet::Value::ChatMessage(msg)
        }
        proto::encrypted_packet_payload::Value::RpcRequest(req) => {
            proto::data_packet::Value::RpcRequest(req)
        }
        proto::encrypted_packet_payload::Value::RpcResponse(resp) => {
            proto::data_packet::Value::RpcResponse(resp)
        }
        proto::encrypted_packet_payload::Value::RpcAck(ack) => {
            proto::data_packet::Value::RpcAck(ack)
        }
        proto::encrypted_packet_payload::Value::StreamHeader(header) => {
            proto::data_packet::Value::StreamHeader(header)
        }
        proto::encrypted_packet_payload::Value::StreamChunk(chunk) => {
            proto::data_packet::Value::StreamChunk(chunk)
        }
        proto::encrypted_packet_payload::Value::StreamTrailer(trailer) => {
            proto::data_packet::Value::StreamTrailer(trailer)
        }
    }
}

fn convert_data_packet_to_encrypted_value(
    value: proto::data_packet::Value,
) -> Option<proto::encrypted_packet_payload::Value> {
    match value {
        proto::data_packet::Value::User(user) => {
            Some(proto::encrypted_packet_payload::Value::User(user))
        }
        proto::data_packet::Value::ChatMessage(msg) => {
            Some(proto::encrypted_packet_payload::Value::ChatMessage(msg))
        }
        proto::data_packet::Value::RpcRequest(req) => {
            Some(proto::encrypted_packet_payload::Value::RpcRequest(req))
        }
        proto::data_packet::Value::RpcResponse(resp) => {
            Some(proto::encrypted_packet_payload::Value::RpcResponse(resp))
        }
        proto::data_packet::Value::RpcAck(ack) => {
            Some(proto::encrypted_packet_payload::Value::RpcAck(ack))
        }
        proto::data_packet::Value::StreamHeader(header) => {
            Some(proto::encrypted_packet_payload::Value::StreamHeader(header))
        }
        proto::data_packet::Value::StreamChunk(chunk) => {
            Some(proto::encrypted_packet_payload::Value::StreamChunk(chunk))
        }
        proto::data_packet::Value::StreamTrailer(trailer) => {
            Some(proto::encrypted_packet_payload::Value::StreamTrailer(trailer))
        }
        _ => None,
    }
}

impl TxQueueItem for EncodedPacket {
    fn buffered_size(&self) -> usize {
        self.data.len()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct IceCandidateJson {
    pub sdp_mid: String,
    pub sdp_m_line_index: i32,
    pub candidate: String,
}

/// Fields shared with rtc_task and signal_task
struct SessionInner {
    signal_client: Arc<SignalClient>,
    has_published: AtomicBool,
    fast_publish: AtomicBool,

    publisher_pc: PeerTransport,
    /// In single peer connection mode, this is None and publisher_pc handles both send/receive
    subscriber_pc: Option<PeerTransport>,
    /// Whether single peer connection mode is active
    single_pc_mode: bool,
    /// Mapping from SDP mid to track ID, used for track resolution in single PC mode
    mid_to_track_id: Mutex<HashMap<String, String>>,

    pending_tracks: Mutex<HashMap<String, oneshot::Sender<proto::TrackInfo>>>,

    // Publisher data channels
    // used to send data to other participants (The SFU forwards the messages)
    lossy_dc: DataChannel,
    lossy_dc_buffered_amount_low_threshold: AtomicU64,
    reliable_dc: DataChannel,
    reliable_dc_buffered_amount_low_threshold: AtomicU64,

    /// Next sequence number for reliable packets.
    next_packet_sequence: AtomicU32,

    /// Time to live (TTL) map between publisher SID and last sequence number.
    packet_rx_state: Mutex<TtlMap<String, u32>>,

    participant_info: SessionParticipantInfo,

    dc_emitter: mpsc::UnboundedSender<DataChannelEvent>,

    // Keep a strong reference to the subscriber datachannels,
    // so we can receive data from other participants
    sub_lossy_dc: Mutex<Option<DataChannel>>,
    sub_reliable_dc: Mutex<Option<DataChannel>>,

    closed: AtomicBool,
    emitter: SessionEmitter,

    options: EngineOptions,
    negotiation_debouncer: Mutex<Option<Debouncer>>,
    negotiation_queue: NegotiationQueue,

    pending_requests: Mutex<HashMap<u32, oneshot::Sender<proto::RequestResponse>>>,

    e2ee_manager: Option<E2eeManager>,
}

/// Information about the local participant needed for outgoing
/// data packets.
struct SessionParticipantInfo {
    sid: ParticipantSid,
    identity: ParticipantIdentity,
}

impl SessionParticipantInfo {
    /// Extracts participant info from a join response.
    fn from_join(join_response: &proto::JoinResponse) -> Option<Self> {
        let Some(info) = &join_response.participant else { None? };
        Some(Self {
            sid: info.sid.clone().try_into().ok()?,
            identity: info.identity.clone().try_into().ok()?,
        })
    }
}

/// This struct holds a WebRTC session
/// The session changes at every reconnection
///
/// RTCSession is also responsable for the signaling and the negotation
pub struct RtcSession {
    inner: Arc<SessionInner>,
    handle: Mutex<Option<SessionHandle>>,
}

impl Debug for RtcSession {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("RtcSession").finish()
    }
}

struct SessionHandle {
    close_tx: watch::Sender<bool>, // false = is_running
    signal_task: JoinHandle<()>,
    rtc_task: JoinHandle<()>,
    dc_task: JoinHandle<()>,
}

impl RtcSession {
    pub async fn connect(
        url: &str,
        token: &str,
        options: EngineOptions,
        e2ee_manager: Option<E2eeManager>,
    ) -> EngineResult<(Self, proto::JoinResponse, SessionEvents)> {
        let (emitter, session_events) = mpsc::unbounded_channel();

        let (signal_client, join_response, signal_events) =
            SignalClient::connect(url, token, options.signal_options.clone()).await?;
        let signal_client = Arc::new(signal_client);
        log::debug!("received JoinResponse: {:?}", join_response);

        // Determine if single PC mode is active based on the path used
        let single_pc_mode = signal_client.is_single_pc_mode_active();

        let Some(participant_info) = SessionParticipantInfo::from_join(&join_response) else {
            Err(EngineError::Internal("Join response missing participant info".into()))?
        };

        let (rtc_emitter, rtc_events) = mpsc::unbounded_channel();
        let rtc_config = make_rtc_config_join(join_response.clone(), options.rtc_config.clone());

        let (dc_emitter, dc_events) = mpsc::unbounded_channel();

        let lk_runtime = LkRuntime::instance();
        let mut publisher_pc = PeerTransport::new(
            lk_runtime.pc_factory().create_peer_connection(rtc_config.clone())?,
            proto::SignalTarget::Publisher,
            single_pc_mode,
        );

        // In single PC mode, subscriber_pc is None
        let mut subscriber_pc = if single_pc_mode {
            None
        } else {
            Some(PeerTransport::new(
                lk_runtime.pc_factory().create_peer_connection(rtc_config)?,
                proto::SignalTarget::Subscriber,
                false,
            ))
        };

        let mut lossy_dc = publisher_pc.peer_connection().create_data_channel(
            LOSSY_DC_LABEL,
            DataChannelInit {
                ordered: false,
                max_retransmits: Some(0),
                ..DataChannelInit::default()
            },
        )?;

        let mut reliable_dc = publisher_pc.peer_connection().create_data_channel(
            RELIABLE_DC_LABEL,
            // Use ordered: true for reliable delivery with ordering guarantees.
            DataChannelInit { ordered: true, ..DataChannelInit::default() },
        )?;

        // Forward events received inside the signaling thread to our rtc channel
        rtc_events::forward_pc_events(&mut publisher_pc, rtc_emitter.clone());
        if let Some(ref mut sub_pc) = subscriber_pc {
            rtc_events::forward_pc_events(sub_pc, rtc_emitter.clone());
        }
        rtc_events::forward_dc_events(&mut lossy_dc, DataPacketKind::Lossy, rtc_emitter.clone());
        rtc_events::forward_dc_events(&mut reliable_dc, DataPacketKind::Reliable, rtc_emitter);

        let (close_tx, close_rx) = watch::channel(false);

        let inner = Arc::new(SessionInner {
            has_published: Default::default(),
            fast_publish: AtomicBool::new(join_response.fast_publish),
            signal_client,
            publisher_pc,
            subscriber_pc,
            single_pc_mode,
            mid_to_track_id: Mutex::new(HashMap::new()),
            pending_tracks: Default::default(),
            lossy_dc,
            lossy_dc_buffered_amount_low_threshold: AtomicU64::new(
                INITIAL_BUFFERED_AMOUNT_LOW_THRESHOLD,
            ),
            reliable_dc,
            reliable_dc_buffered_amount_low_threshold: AtomicU64::new(
                INITIAL_BUFFERED_AMOUNT_LOW_THRESHOLD,
            ),
            next_packet_sequence: 1.into(),
            packet_rx_state: Mutex::new(TtlMap::new(RELIABLE_RECEIVED_STATE_TTL)),
            participant_info,
            dc_emitter,
            sub_lossy_dc: Mutex::new(None),
            sub_reliable_dc: Mutex::new(None),
            closed: Default::default(),
            emitter,
            options,
            negotiation_debouncer: Default::default(),
            negotiation_queue: NegotiationQueue::new(),
            pending_requests: Default::default(),
            e2ee_manager,
        });

        // Start session tasks
        let signal_task =
            livekit_runtime::spawn(inner.clone().signal_task(signal_events, close_rx.clone()));
        let rtc_task =
            livekit_runtime::spawn(inner.clone().rtc_session_task(rtc_events, close_rx.clone()));
        let dc_task = livekit_runtime::spawn(inner.clone().data_channel_task(dc_events, close_rx));

        let handle = Mutex::new(Some(SessionHandle { close_tx, signal_task, rtc_task, dc_task }));

        // In single PC mode (or with fast_publish), trigger initial negotiation
        // This matches JS SDK behavior: if (!this.subscriberPrimary || joinResponse.fastPublish) { this.negotiate(); }
        if single_pc_mode || join_response.fast_publish {
            inner.publisher_negotiation_needed();
        }

        Ok((Self { inner, handle }, join_response, session_events))
    }

    pub fn has_published(&self) -> bool {
        self.inner.has_published.load(Ordering::Acquire)
    }

    /// Returns whether single peer connection mode is active
    pub fn is_single_pc_mode(&self) -> bool {
        self.inner.single_pc_mode
    }

    /// Get the track ID for a given SDP mid (used in single PC mode)
    pub fn get_track_id_for_mid(&self, mid: &str) -> Option<String> {
        self.inner.mid_to_track_id.lock().get(mid).cloned()
    }

    pub fn remove_track(&self, sender: RtpSender) -> EngineResult<()> {
        self.inner.remove_track(sender)
    }

    pub fn publisher_negotiation_needed(&self) {
        self.inner.publisher_negotiation_needed()
    }

    pub async fn add_track(&self, req: proto::AddTrackRequest) -> EngineResult<proto::TrackInfo> {
        self.inner.add_track(req).await
    }

    pub async fn mute_track(&self, req: proto::MuteTrackRequest) -> EngineResult<()> {
        self.inner.mute_track(req).await
    }

    pub async fn create_sender(
        &self,
        track: LocalTrack,
        options: TrackPublishOptions,
        encodings: Vec<RtpEncodingParameters>,
    ) -> EngineResult<RtpTransceiver> {
        self.inner.create_sender(track, options, encodings).await
    }

    /// Close the PeerConnections and the SignalClient
    pub async fn close(&self) {
        // Close the tasks
        let handle = self.handle.lock().take();
        if let Some(handle) = handle {
            let _ = handle.close_tx.send(true);
            let _ = handle.rtc_task.await;
            let _ = handle.signal_task.await;
            let _ = handle.dc_task.await;
        }

        // Close the PeerConnections after the task
        // So if a sensitive operation is running, we can wait for it
        self.inner.close().await;
    }

    pub async fn publish_data(
        &self,
        data: proto::DataPacket,
        kind: DataPacketKind,
        is_raw_packet: bool,
    ) -> Result<(), EngineError> {
        self.inner.publish_data(data, kind, is_raw_packet).await
    }

    pub async fn restart(&self) -> EngineResult<proto::ReconnectResponse> {
        self.inner.restart().await
    }

    pub async fn restart_publisher(&self) -> EngineResult<()> {
        self.inner.restart_publisher().await
    }

    pub async fn wait_pc_connection(&self) -> EngineResult<()> {
        self.inner.wait_pc_connection().await
    }

    /// Ensure the publisher peer connection is connected and the data channel is open.
    /// This triggers negotiation if needed and waits for the connection to be established.
    pub async fn ensure_publisher_connected(&self) -> EngineResult<()> {
        self.inner.ensure_publisher_connected(DataPacketKind::Reliable).await
    }

    pub async fn simulate_scenario(&self, scenario: SimulateScenario) -> EngineResult<()> {
        self.inner.simulate_scenario(scenario).await
    }

    pub async fn get_stats(&self) -> EngineResult<SessionStats> {
        let publisher_stats = self.inner.publisher_pc.peer_connection().get_stats().await?;

        let subscriber_stats = if let Some(ref sub_pc) = self.inner.subscriber_pc {
            sub_pc.peer_connection().get_stats().await?
        } else {
            // In single PC mode, there's no separate subscriber stats
            Vec::new()
        };

        Ok(SessionStats { publisher_stats, subscriber_stats })
    }

    pub fn publisher(&self) -> &PeerTransport {
        &self.inner.publisher_pc
    }

    pub fn subscriber(&self) -> Option<&PeerTransport> {
        self.inner.subscriber_pc.as_ref()
    }

    pub fn signal_client(&self) -> &Arc<SignalClient> {
        &self.inner.signal_client
    }

    pub fn data_channel(&self, target: SignalTarget, kind: DataPacketKind) -> Option<DataChannel> {
        self.inner.data_channel(target, kind)
    }

    pub fn e2ee_manager(&self) -> Option<E2eeManager> {
        self.inner.e2ee_manager.clone()
    }

    pub fn data_channel_buffered_amount_low_threshold(&self, kind: DataPacketKind) -> u64 {
        match kind {
            DataPacketKind::Lossy => {
                self.inner.lossy_dc_buffered_amount_low_threshold.load(Ordering::Relaxed)
            }
            DataPacketKind::Reliable => {
                self.inner.reliable_dc_buffered_amount_low_threshold.load(Ordering::Relaxed)
            }
        }
    }

    pub fn set_data_channel_buffered_amount_low_threshold(
        &self,
        threshold: u64,
        kind: DataPacketKind,
    ) {
        match kind {
            DataPacketKind::Lossy => self
                .inner
                .lossy_dc_buffered_amount_low_threshold
                .store(threshold, Ordering::Relaxed),
            DataPacketKind::Reliable => self
                .inner
                .reliable_dc_buffered_amount_low_threshold
                .store(threshold, Ordering::Relaxed),
        }
        let _ = self
            .inner
            .emitter
            .send(SessionEvent::DataChannelBufferedAmountLowThresholdChanged { kind, threshold });
    }

    pub fn data_channel_receive_states(&self) -> Vec<proto::DataChannelReceiveState> {
        self.inner.data_channel_receive_states()
    }

    pub async fn get_response(&self, request_id: u32) -> proto::RequestResponse {
        self.inner.get_response(request_id).await
    }
}

impl SessionInner {
    async fn rtc_session_task(
        self: Arc<Self>,
        mut rtc_events: RtcEvents,
        mut close_rx: watch::Receiver<bool>,
    ) {
        loop {
            tokio::select! {
                Some(event) = rtc_events.recv() => {
                    let debug = format!("{:?}", event);
                    let inner = self.clone();
                    let (tx, rx) = oneshot::channel();
                    let task = livekit_runtime::spawn(async move {
                        if let Err(err) = inner.on_rtc_event(event).await {
                            log::error!("failed to handle rtc event: {:?}", err);
                        }
                        let _ = tx.send(());
                    });

                    // Monitor sync/async blockings
                    tokio::select! {
                        _ = rx => {},
                        _ = livekit_runtime::sleep(Duration::from_secs(10)) => {
                            log::error!("rtc_event is taking too much time: {}", debug);
                        }
                    }

                    task.await;
                },
                _ = close_rx.changed() => {
                    break;
                }
            }
        }

        log::debug!("rtc_session_task closed");
    }

    async fn signal_task(
        self: Arc<Self>,
        mut signal_events: SignalEvents,
        mut close_rx: watch::Receiver<bool>,
    ) {
        loop {
            tokio::select! {
                Some(signal) = signal_events.recv() => {
                    match signal {
                        SignalEvent::Message(signal) => {
                            let debug = format!("{:?}", signal);
                            let inner = self.clone();
                            let (tx, rx) = oneshot::channel();
                            let task = livekit_runtime::spawn(async move {
                                if let Err(err) = inner.on_signal_event(*signal).await {
                                    log::error!("failed to handle signal: {:?}", err);
                                }
                                let _ = tx.send(());
                            });

                            // Monitor sync/async blockings
                            tokio::select! {
                                _ = rx => {},
                                _ = livekit_runtime::sleep(Duration::from_secs(10)) => {
                                    log::error!("signal_event taking too much time: {}", debug);
                                }
                            }

                            task.await;
                        }
                        SignalEvent::Close(reason) => {
                            if !self.closed.load(Ordering::Acquire) {
                                // SignalClient has been closed unexpectedly
                                self.on_session_disconnected(
                                    format!("signal client closed: {:?}", reason).as_str(),
                                    DisconnectReason::UnknownReason,
                                    proto::leave_request::Action::Resume,
                                    false,
                                );
                            }
                        }
                    }
                },
                _ = close_rx.changed() => {
                    break;
                }
            }
        }

        log::debug!("closing signal_task");
    }

    async fn data_channel_task(
        self: Arc<Self>,
        mut dc_events: mpsc::UnboundedReceiver<DataChannelEvent>,
        mut close_rx: watch::Receiver<bool>,
    ) {
        let mut lossy_buffered_amount = 0;
        let mut reliable_buffered_amount = 0;
        let mut lossy_queue = VecDeque::new();
        let mut reliable_queue = VecDeque::new();
        let mut retry_queue = TxQueue::new();

        loop {
            tokio::select! {
                event = dc_events.recv() => {
                    let Some(event) = event else {
                        // tx closed
                        break;
                    };

                    match event.detail {
                        DataChannelEventDetail::PublishPacket(mut request) => {
                            if event.kind == DataPacketKind::Reliable {
                                request.packet.sequence = self.next_packet_sequence.fetch_add(1, Ordering::Relaxed);
                            }
                            let ev = DataChannelEvent {
                                kind: event.kind,
                                detail: DataChannelEventDetail::PublishData(PublishDataRequest {
                                    encoded_packet: request.packet.into(),
                                    completion_tx: request.completion_tx.into(),
                                })
                            };
                            if let Err(err) = self.dc_emitter.send(ev) {
                                log::error!("Failed to enqueue send data request: {}", err)
                            }
                        }
                        DataChannelEventDetail::PublishData(request) => {
                            match event.kind {
                                DataPacketKind::Lossy => {
                                    lossy_queue.push_back(request);
                                    let threshold = self.lossy_dc_buffered_amount_low_threshold.load(Ordering::Relaxed);
                                    self._send_until_threshold(DataPacketKind::Lossy, threshold, &mut lossy_buffered_amount, &mut lossy_queue, &mut retry_queue);
                                }
                                DataPacketKind::Reliable => {
                                    reliable_queue.push_back(request);
                                    let threshold = self.reliable_dc_buffered_amount_low_threshold.load(Ordering::Relaxed);
                                    self._send_until_threshold(DataPacketKind::Reliable, threshold, &mut reliable_buffered_amount, &mut reliable_queue, &mut retry_queue);
                                }
                            }
                        }
                        DataChannelEventDetail::BufferedAmountChange(sent) => {
                            match event.kind {
                                DataPacketKind::Lossy => {
                                    if lossy_buffered_amount < sent {
                                        // I believe never reach here but adding logs just in case
                                        log::error!("unexpected buffer size detected: lossy_buffered_amount={}, sent={}", lossy_buffered_amount, sent);
                                        lossy_buffered_amount = 0;
                                    } else {
                                        lossy_buffered_amount -= sent;
                                    }
                                    let threshold = self.lossy_dc_buffered_amount_low_threshold.load(Ordering::Relaxed);
                                    self._send_until_threshold(DataPacketKind::Lossy, threshold, &mut lossy_buffered_amount, &mut lossy_queue, &mut retry_queue);
                                }
                                DataPacketKind::Reliable => {
                                    if reliable_buffered_amount < sent {
                                        log::error!("unexpected buffer size detected: reliable_buffered_amount={}, sent={}", reliable_buffered_amount, sent);
                                        reliable_buffered_amount = 0;
                                    } else {
                                        reliable_buffered_amount -= sent;
                                    }
                                    let threshold = self.reliable_dc_buffered_amount_low_threshold.load(Ordering::Relaxed);
                                    self._send_until_threshold(DataPacketKind::Reliable, threshold, &mut reliable_buffered_amount, &mut reliable_queue, &mut retry_queue);
                                    retry_queue.trim(sent as usize);
                                }
                            }
                        }
                        DataChannelEventDetail::RetryFrom(last_sequence) => {
                            assert!(event.kind == DataPacketKind::Reliable);
                            self._enqueue_for_retry_from(last_sequence, &mut retry_queue);
                        }
                    }
                },

                _ = close_rx.changed() => {
                    break;
                },
            }
        }

        log::debug!("closing data_channel_task");
    }

    fn _send_until_threshold(
        self: &Arc<Self>,
        kind: DataPacketKind,
        threshold: u64,
        buffered_amount: &mut u64,
        request_queue: &mut VecDeque<PublishDataRequest>,
        retry_queue: &mut TxQueue<EncodedPacket>,
    ) {
        while *buffered_amount <= threshold {
            let Some(request) = request_queue.pop_front() else {
                break;
            };

            *buffered_amount += request.encoded_packet.data.len() as u64;
            let result = self
                .data_channel(SignalTarget::Publisher, kind)
                .unwrap()
                .send(&request.encoded_packet.data, true)
                .map_err(|err| {
                    EngineError::Internal(format!("failed to send data packet: {:?}", err).into())
                });

            if let Some(completion_tx) = request.completion_tx {
                _ = completion_tx.send(result);
            }
            if kind == DataPacketKind::Reliable {
                retry_queue.enqueue(request.encoded_packet);
            }
        }
    }

    fn _enqueue_for_retry_from(
        self: &Arc<Self>,
        last_sequence: u32,
        retry_queue: &mut TxQueue<EncodedPacket>,
    ) {
        if let Some(first) = retry_queue.peek() {
            if first.sequence > last_sequence + 1 {
                log::warn!(
                    "Wrong packet sequence while retrying: {} > {}, {} packets missing",
                    first.sequence,
                    last_sequence + 1,
                    first.sequence - last_sequence - 1
                );
            }
        }

        while let Some(encoded_packet) = retry_queue.dequeue() {
            if encoded_packet.sequence <= last_sequence {
                continue;
            };
            let ev = DataChannelEvent {
                kind: DataPacketKind::Reliable,
                detail: DataChannelEventDetail::PublishData(PublishDataRequest {
                    encoded_packet,
                    completion_tx: None,
                }),
            };
            if let Err(err) = self.dc_emitter.send(ev) {
                log::error!("Failed to enqueue data for retry: {}", err);
            }
        }
    }

    /// Updates the packet receive state (TTL map) for reliable packets.
    fn update_packet_rx_state(&self, packet: &proto::DataPacket) {
        if packet.sequence <= 0 || packet.participant_sid.is_empty() {
            return;
        };
        let mut rx_state = self.packet_rx_state.lock();
        if rx_state
            .get(&packet.participant_sid)
            .is_some_and(|&last_sequence| packet.sequence <= last_sequence)
        {
            log::warn!("Ignoring duplicate/out-of-order reliable data message");
            return;
        }
        rx_state.set(&packet.participant_sid, Some(packet.sequence));
    }

    async fn on_signal_event(
        self: &Arc<Self>,
        event: proto::signal_response::Message,
    ) -> EngineResult<()> {
        match event {
            proto::signal_response::Message::Answer(answer) => {
                log::debug!("received publisher answer: {:?}", answer);

                // Store mid_to_track_id mapping in single PC mode
                if self.single_pc_mode && !answer.mid_to_track_id.is_empty() {
                    let mut mapping = self.mid_to_track_id.lock();
                    for (mid, track_id) in &answer.mid_to_track_id {
                        mapping.insert(mid.clone(), track_id.clone());
                    }
                }

                let answer =
                    SessionDescription::parse(&answer.sdp, answer.r#type.parse().unwrap()).unwrap(); // Unwrap is ok, the server shouldn't give us an invalid sdp
                self.publisher_pc.set_remote_description(answer).await?;

                if self.fast_publish.load(Ordering::Acquire) {
                    if self.negotiation_queue.waiting_for_answer.swap(false, Ordering::AcqRel) {
                        log::debug!("answer received, notifying negotiation loop");
                        self.negotiation_queue.waker.notify_one();
                    }
                }
            }
            proto::signal_response::Message::Offer(offer) => {
                // In single PC mode, client always offers and server always answers,
                // so we should never receive an offer from the server.
                if self.single_pc_mode {
                    log::warn!("received unexpected offer in single PC mode, ignoring");
                    return Ok(());
                }

                // Dual PC mode: handle offer on subscriber PC
                log::debug!("received subscriber offer: {:?}", offer);
                let offer_sdp =
                    SessionDescription::parse(&offer.sdp, offer.r#type.parse().unwrap()).unwrap();

                let answer = self
                    .subscriber_pc
                    .as_ref()
                    .unwrap()
                    .create_anwser(offer_sdp, AnswerOptions::default())
                    .await?;

                self.signal_client
                    .send(proto::signal_request::Message::Answer(proto::SessionDescription {
                        r#type: "answer".to_string(),
                        sdp: answer.to_string(),
                        id: 0,
                        mid_to_track_id: Default::default(),
                    }))
                    .await;
            }
            proto::signal_response::Message::Trickle(trickle) => {
                let target = trickle.target();
                let ice_candidate = {
                    let json =
                        serde_json::from_str::<IceCandidateJson>(&trickle.candidate_init).unwrap();
                    IceCandidate::parse(&json.sdp_mid, json.sdp_m_line_index, &json.candidate)
                        .unwrap()
                };

                log::debug!("remote ice_candidate {:?} {:?}", ice_candidate, target);

                if target == proto::SignalTarget::Publisher {
                    self.publisher_pc.add_ice_candidate(ice_candidate).await?;
                } else if self.single_pc_mode {
                    // In single PC mode, all ICE candidates go to publisher
                    self.publisher_pc.add_ice_candidate(ice_candidate).await?;
                } else {
                    self.subscriber_pc.as_ref().unwrap().add_ice_candidate(ice_candidate).await?;
                }
            }
            proto::signal_response::Message::Leave(leave) => {
                log::debug!("received leave request: {:?}", leave);
                self.on_session_disconnected(
                    "server request to leave",
                    leave.reason(),
                    leave.action(),
                    true,
                );
            }
            proto::signal_response::Message::Update(update) => {
                let _ = self
                    .emitter
                    .send(SessionEvent::ParticipantUpdate { updates: update.participants });
            }
            proto::signal_response::Message::SpeakersChanged(speaker) => {
                let _ =
                    self.emitter.send(SessionEvent::SpeakersChanged { speakers: speaker.speakers });
            }
            proto::signal_response::Message::ConnectionQuality(quality) => {
                let _ =
                    self.emitter.send(SessionEvent::ConnectionQuality { updates: quality.updates });
            }
            proto::signal_response::Message::TrackPublished(publish_res) => {
                let mut pending_tracks = self.pending_tracks.lock();
                if let Some(tx) = pending_tracks.remove(&publish_res.cid) {
                    let _ = tx.send(publish_res.track.unwrap());
                }
            }
            proto::signal_response::Message::RoomUpdate(room_update) => {
                let _ =
                    self.emitter.send(SessionEvent::RoomUpdate { room: room_update.room.unwrap() });
            }
            proto::signal_response::Message::RoomMoved(room_moved) => {
                let _ = self.emitter.send(SessionEvent::RoomMoved { moved: room_moved });
            }
            proto::signal_response::Message::TrackSubscribed(track_subscribed) => {
                let _ = self.emitter.send(SessionEvent::LocalTrackSubscribed {
                    track_sid: track_subscribed.track_sid,
                });
            }
            proto::signal_response::Message::RequestResponse(request_response) => {
                let mut pending_requests = self.pending_requests.lock();
                if let Some(tx) = pending_requests.remove(&request_response.request_id) {
                    let _ = tx.send(request_response);
                }
            }
            proto::signal_response::Message::RefreshToken(ref token) => {
                let url = self.signal_client.url();
                let _ = self.emitter.send(SessionEvent::RefreshToken { url, token: token.clone() });
            }
            proto::signal_response::Message::Mute(req) => {
                let _ =
                    self.emitter.send(SessionEvent::TrackMuted { sid: req.sid, muted: req.muted });
            }
            proto::signal_response::Message::MediaSectionsRequirement(req) => {
                if self.single_pc_mode {
                    self.handle_media_sections_requirement(req)?;
                }
            }
            _ => {}
        }

        Ok(())
    }

    /// Handle MediaSectionsRequirement by adding recvonly transceivers to publisher PC.
    /// This matches the JS SDK behavior: add transceivers and trigger negotiation.
    fn handle_media_sections_requirement(
        self: &Arc<Self>,
        req: proto::MediaSectionsRequirement,
    ) -> EngineResult<()> {
        log::debug!(
            "MediaSectionsRequirement: adding {} audio and {} video recvonly transceivers",
            req.num_audios,
            req.num_videos
        );

        let recvonly_init = RtpTransceiverInit {
            direction: RtpTransceiverDirection::RecvOnly,
            stream_ids: Vec::new(),
            send_encodings: Vec::new(),
        };

        // Add audio transceivers
        for _ in 0..req.num_audios {
            self.publisher_pc
                .peer_connection()
                .add_transceiver_for_media(MediaType::Audio, recvonly_init.clone())?;
        }

        // Add video transceivers
        for _ in 0..req.num_videos {
            self.publisher_pc
                .peer_connection()
                .add_transceiver_for_media(MediaType::Video, recvonly_init.clone())?;
        }

        // Trigger renegotiation
        self.publisher_negotiation_needed();

        Ok(())
    }

    async fn on_rtc_event(&self, event: RtcEvent) -> EngineResult<()> {
        match event {
            RtcEvent::IceCandidate { ice_candidate, target } => {
                log::debug!("local ice_candidate {:?} {:?}", ice_candidate, target);
                self.signal_client
                    .send(proto::signal_request::Message::Trickle(proto::TrickleRequest {
                        candidate_init: serde_json::to_string(&IceCandidateJson {
                            sdp_mid: ice_candidate.sdp_mid(),
                            sdp_m_line_index: ice_candidate.sdp_mline_index(),
                            candidate: ice_candidate.candidate(),
                        })
                        .unwrap(),
                        target: target as i32,
                        ..Default::default()
                    }))
                    .await;
            }
            RtcEvent::ConnectionChange { state, target } => {
                log::debug!("connection change, {:?} {:?}", state, target);

                if state == PeerConnectionState::Failed {
                    log::error!("{:?} pc state failed", target);
                    self.on_session_disconnected(
                        "pc_state failed",
                        DisconnectReason::UnknownReason,
                        proto::leave_request::Action::Resume,
                        false,
                    );
                }
            }
            RtcEvent::DataChannel { data_channel, target } => {
                // In single PC mode, subscriber data channels come from publisher target
                let is_subscriber_dc = if self.single_pc_mode {
                    target == SignalTarget::Publisher
                } else {
                    target == SignalTarget::Subscriber
                };
                if is_subscriber_dc {
                    if data_channel.label() == LOSSY_DC_LABEL {
                        self.sub_lossy_dc.lock().replace(data_channel);
                    } else if data_channel.label() == RELIABLE_DC_LABEL {
                        self.sub_reliable_dc.lock().replace(data_channel);
                    }
                }
            }
            RtcEvent::Offer { offer, target: _ } => {
                // Send the publisher offer to the server
                log::debug!("sending publisher offer: {:?}", offer);
                self.signal_client
                    .send(proto::signal_request::Message::Offer(proto::SessionDescription {
                        r#type: "offer".to_string(),
                        sdp: offer.to_string(),
                        id: 0,
                        mid_to_track_id: Default::default(),
                    }))
                    .await;
            }
            RtcEvent::Track { mut streams, track, transceiver, target: _ } => {
                if !streams.is_empty() {
                    let _ = self.emitter.send(SessionEvent::MediaTrack {
                        stream: streams.remove(0),
                        track,
                        transceiver,
                    });
                }
            }
            RtcEvent::Data { data, binary, kind } => {
                if !binary {
                    Err(EngineError::Internal("text messages aren't supported".into()))?;
                }
                let mut packet = proto::DataPacket::decode(&*data).map_err(|err| {
                    EngineError::Internal(format!("failed to decode data packet: {}", err).into())
                })?;
                if kind == DataPacketKind::Reliable {
                    self.update_packet_rx_state(&packet);
                }
                if let Some(detail) = packet.value.take() {
                    let participant_sid: Option<ParticipantSid> =
                        packet.participant_sid.try_into().ok();
                    let participant_identity: Option<ParticipantIdentity> =
                        packet.participant_identity.try_into().ok();
                    self.emit_incoming_packet(
                        kind,
                        participant_sid,
                        participant_identity,
                        detail,
                        proto::encryption::Type::None,
                    );
                }
            }
            RtcEvent::DataChannelBufferedAmountChange { sent, amount: _, kind } => {
                let ev = DataChannelEvent {
                    kind,
                    detail: DataChannelEventDetail::BufferedAmountChange(sent),
                };
                if let Err(err) = self.dc_emitter.send(ev) {
                    log::error!("failed to send dc_event buffer_amount_change: {:?}", err);
                }
            }
        }

        Ok(())
    }

    fn emit_incoming_packet(
        &self,
        kind: DataPacketKind,
        participant_sid: Option<ParticipantSid>,
        participant_identity: Option<ParticipantIdentity>,
        value: proto::data_packet::Value,
        encryption_type: proto::encryption::Type,
    ) {
        let send_result = match value {
            proto::data_packet::Value::User(user) => {
                // Participant SID and identity used to be defined on user packet, but
                // they have been moved to the packet root. For backwards compatibility,
                // we take the user packet's values if the top-level fields are not set.
                let participant_sid =
                    participant_sid.or_else(|| user.participant_sid.try_into().ok());
                let participant_identity =
                    participant_identity.or_else(|| user.participant_identity.try_into().ok());
                self.emitter.send(SessionEvent::Data {
                    kind,
                    participant_sid,
                    participant_identity,
                    payload: user.payload,
                    topic: user.topic,
                    encryption_type,
                })
            }
            proto::data_packet::Value::SipDtmf(dtmf) => self.emitter.send(SessionEvent::SipDTMF {
                participant_identity,
                digit: (!dtmf.digit.is_empty()).then_some(dtmf.digit),
                code: dtmf.code,
            }),
            proto::data_packet::Value::Transcription(transcription) => {
                let segments = transcription
                    .segments
                    .into_iter()
                    .map(|s| TranscriptionSegment {
                        id: s.id,
                        start_time: s.start_time,
                        end_time: s.end_time,
                        text: s.text,
                        language: s.language,
                        r#final: s.r#final,
                    })
                    .collect();
                let participant_identity = transcription.transcribed_participant_identity.into();

                self.emitter.send(SessionEvent::Transcription {
                    participant_identity,
                    track_sid: transcription.track_id,
                    segments,
                })
            }
            proto::data_packet::Value::RpcRequest(rpc_request) => {
                let caller_identity = participant_identity;
                self.emitter.send(SessionEvent::RpcRequest {
                    caller_identity,
                    request_id: rpc_request.id.clone(),
                    method: rpc_request.method,
                    payload: rpc_request.payload,
                    response_timeout: Duration::from_millis(rpc_request.response_timeout_ms as u64),
                    version: rpc_request.version,
                })
            }
            proto::data_packet::Value::RpcResponse(rpc_response) => {
                let (payload, error) = match rpc_response.value {
                    None => (None, None),
                    Some(proto::rpc_response::Value::Payload(payload)) => (Some(payload), None),
                    Some(proto::rpc_response::Value::Error(err)) => (None, Some(err)),
                };
                self.emitter.send(SessionEvent::RpcResponse {
                    request_id: rpc_response.request_id,
                    payload,
                    error,
                })
            }
            proto::data_packet::Value::RpcAck(rpc_ack) => {
                self.emitter.send(SessionEvent::RpcAck { request_id: rpc_ack.request_id })
            }
            proto::data_packet::Value::ChatMessage(message) => {
                self.emitter.send(SessionEvent::ChatMessage {
                    participant_identity: participant_identity
                        .unwrap_or(ParticipantIdentity("".into())),
                    message: ChatMessage::from(message),
                })
            }
            proto::data_packet::Value::StreamHeader(header) => {
                let participant_identity =
                    participant_identity.map_or("".into(), |identity| identity.0);
                self.emitter.send(SessionEvent::DataStreamHeader {
                    header,
                    participant_identity,
                    encryption_type,
                })
            }
            proto::data_packet::Value::StreamChunk(chunk) => {
                let participant_identity =
                    participant_identity.map_or("".into(), |identity| identity.0);
                self.emitter.send(SessionEvent::DataStreamChunk {
                    chunk,
                    participant_identity,
                    encryption_type,
                })
            }
            proto::data_packet::Value::StreamTrailer(trailer) => {
                let participant_identity =
                    participant_identity.map_or("".into(), |identity| identity.0);
                self.emitter.send(SessionEvent::DataStreamTrailer { trailer, participant_identity })
            }
            proto::data_packet::Value::EncryptedPacket(encrypted_packet) => {
                // Handle encrypted data packets
                if let Some(e2ee_manager) = &self.e2ee_manager {
                    let participant_identity_str =
                        participant_identity.as_ref().map(|p| p.0.as_str()).unwrap_or("");

                    match e2ee_manager.handle_encrypted_data(
                        &encrypted_packet.encrypted_value,
                        &encrypted_packet.iv,
                        participant_identity_str,
                        encrypted_packet.key_index,
                    ) {
                        Some(decrypted_payload) => {
                            // Parse the decrypted payload as EncryptedPacketPayload
                            match proto::EncryptedPacketPayload::decode(&*decrypted_payload) {
                                Ok(encrypted_payload) => {
                                    if let Some(decrypted_value) = encrypted_payload.value {
                                        // Recursively call emit_incoming_packet with the decrypted value
                                        self.emit_incoming_packet(
                                            kind,
                                            participant_sid,
                                            participant_identity,
                                            convert_encrypted_to_data_packet_value(decrypted_value),
                                            encrypted_packet.encryption_type(),
                                        );
                                        Ok(())
                                    } else {
                                        log::warn!("Failed to decrypt encrypted payload");
                                        Ok(())
                                    }
                                }
                                Err(e) => {
                                    log::warn!("Failed to decode decrypted payload: {}", e);
                                    Ok(())
                                }
                            }
                        }
                        None => {
                            log::warn!(
                                "Failed to decrypt data packet from {}",
                                participant_identity_str
                            );
                            Ok(())
                        }
                    }
                } else {
                    log::warn!("Received encrypted data packet but E2EE is not enabled");
                    Ok(()) // Don't emit the event if E2EE is not available
                }
            }
            _ => Ok(()),
        };
        if let Err(err) = send_result {
            log::error!("failed to emit incoming data packet: {:?}", err);
        }
    }

    async fn add_track(&self, req: proto::AddTrackRequest) -> EngineResult<proto::TrackInfo> {
        let (tx, rx) = oneshot::channel();
        let cid = req.cid.clone();
        {
            let mut pendings_tracks = self.pending_tracks.lock();
            if pendings_tracks.contains_key(&req.cid) {
                Err(EngineError::Internal("track already published".into()))?;
            }

            pendings_tracks.insert(cid.clone(), tx);
        }

        self.signal_client.send(proto::signal_request::Message::AddTrack(req)).await;

        // Wait the result from the server (TrackInfo)
        tokio::select! {
            Ok(info) = rx => Ok(info),
            _ = sleep(TRACK_PUBLISH_TIMEOUT) => {
                self.pending_tracks.lock().remove(&cid);
                Err(EngineError::Internal("track publication timed out, no response received from the server".into()))
            },
            else => {
                Err(EngineError::Internal("track publication cancelled".into()))
            }
        }
    }

    fn remove_track(&self, sender: RtpSender) -> EngineResult<()> {
        if let Some(track) = sender.track() {
            let mut pending_tracks = self.pending_tracks.lock();
            pending_tracks.remove(&track.id());
        }

        self.publisher_pc.peer_connection().remove_track(sender)?;

        Ok(())
    }

    async fn mute_track(&self, req: proto::MuteTrackRequest) -> EngineResult<()> {
        self.signal_client.send(proto::signal_request::Message::Mute(req)).await;

        Ok(())
    }

    async fn create_sender(
        &self,
        track: LocalTrack,
        options: TrackPublishOptions,
        encodings: Vec<RtpEncodingParameters>,
    ) -> EngineResult<RtpTransceiver> {
        // If video track, derive "ultimate" bitrate from encodings and stash it for offer munging.
        // Must be done before encodings is moved into RtpTransceiverInit.
        if track.kind() == TrackKind::Video {
            let ultimate_bps: Option<u64> = {
                let sum: u64 = encodings.iter().filter_map(|e| e.max_bitrate).sum();
                (sum > 0).then_some(sum)
            };
            self.publisher_pc.set_max_send_bitrate_bps(ultimate_bps).await;
        }

        let init = RtpTransceiverInit {
            direction: RtpTransceiverDirection::SendOnly,
            stream_ids: Default::default(),
            send_encodings: encodings,
        };

        let transceiver =
            self.publisher_pc.peer_connection().add_transceiver(track.rtc_track(), init)?;

        if track.kind() == TrackKind::Video {
            let capabilities = LkRuntime::instance().pc_factory().get_rtp_sender_capabilities(
                match track.kind() {
                    TrackKind::Video => MediaType::Video,
                    TrackKind::Audio => MediaType::Audio,
                },
            );

            let mut matched = Vec::new();
            let mut partial_matched = Vec::new();
            let mut unmatched = Vec::new();

            for codec in capabilities.codecs {
                let mime_type = codec.mime_type.to_lowercase();
                if mime_type == format!("video/{}", options.video_codec.as_str()) {
                    if let Some(sdp_fmtp_line) = codec.sdp_fmtp_line.as_ref() {
                        // for h264 codecs that have sdpFmtpLine available, use only if the
                        // profile-level-id is 42e01f for cross-browser compatibility
                        if sdp_fmtp_line.contains("profile-level-id=42e01f") {
                            matched.push(codec);
                            continue;
                        }
                    }
                    partial_matched.push(codec);
                } else {
                    unmatched.push(codec);
                }
            }

            matched.append(&mut partial_matched);

            transceiver.set_codec_preferences(matched)?;
        }

        Ok(transceiver)
    }

    /// Called when the SignalClient or one of the PeerConnection has lost the connection
    /// The RTCEngine may try a reconnect.
    fn on_session_disconnected(
        &self,
        source: &str,
        reason: DisconnectReason,
        action: proto::leave_request::Action,
        retry_now: bool,
    ) {
        let _ = self.emitter.send(SessionEvent::Close {
            source: source.to_owned(),
            reason,
            action,
            retry_now,
        });
    }

    async fn close(&self) {
        self.closed.store(true, Ordering::Release);

        self.signal_client
            .send(proto::signal_request::Message::Leave(proto::LeaveRequest {
                action: proto::leave_request::Action::Disconnect.into(),
                reason: DisconnectReason::ClientInitiated as i32,
                ..Default::default()
            }))
            .await;

        self.signal_client.close().await;
        self.publisher_pc.close();
        if let Some(ref sub_pc) = self.subscriber_pc {
            sub_pc.close();
        }
    }

    async fn simulate_scenario(self: &Arc<Self>, scenario: SimulateScenario) -> EngineResult<()> {
        match scenario {
            SimulateScenario::SignalReconnect => {
                self.signal_client.close().await;
            }
            SimulateScenario::Speaker => {
                self.signal_client
                    .send(proto::signal_request::Message::Simulate(proto::SimulateScenario {
                        scenario: Some(proto::simulate_scenario::Scenario::SpeakerUpdate(3)),
                    }))
                    .await;
            }
            SimulateScenario::NodeFailure => {
                self.signal_client
                    .send(proto::signal_request::Message::Simulate(proto::SimulateScenario {
                        scenario: Some(proto::simulate_scenario::Scenario::NodeFailure(true)),
                    }))
                    .await;
            }
            SimulateScenario::ServerLeave => {
                self.signal_client
                    .send(proto::signal_request::Message::Simulate(proto::SimulateScenario {
                        scenario: Some(proto::simulate_scenario::Scenario::ServerLeave(true)),
                    }))
                    .await;
            }
            SimulateScenario::Migration => {
                self.signal_client
                    .send(proto::signal_request::Message::Simulate(proto::SimulateScenario {
                        scenario: Some(proto::simulate_scenario::Scenario::Migration(true)),
                    }))
                    .await;
            }
            SimulateScenario::ForceTcp => {
                self.signal_client
                    .send(proto::signal_request::Message::Simulate(proto::SimulateScenario {
                        scenario: Some(
                            proto::simulate_scenario::Scenario::SwitchCandidateProtocol(
                                proto::CandidateProtocol::Tcp as i32,
                            ),
                        ),
                    }))
                    .await;

                self.on_signal_event(proto::signal_response::Message::Leave(proto::LeaveRequest {
                    action: proto::leave_request::Action::Reconnect.into(),
                    reason: DisconnectReason::ClientInitiated as i32,
                    ..Default::default()
                }))
                .await?
            }
            SimulateScenario::ForceTls => {
                self.signal_client
                    .send(proto::signal_request::Message::Simulate(proto::SimulateScenario {
                        scenario: Some(
                            proto::simulate_scenario::Scenario::SwitchCandidateProtocol(
                                proto::CandidateProtocol::Tls as i32,
                            ),
                        ),
                    }))
                    .await;

                self.on_signal_event(proto::signal_response::Message::Leave(proto::LeaveRequest {
                    action: proto::leave_request::Action::Reconnect.into(),
                    reason: DisconnectReason::ClientInitiated as i32,
                    ..Default::default()
                }))
                .await?
            }
        }
        Ok(())
    }

    async fn publish_data(
        self: &Arc<Self>,
        mut packet: proto::DataPacket,
        kind: DataPacketKind,
        is_raw_packet: bool,
    ) -> Result<(), EngineError> {
        self.ensure_publisher_connected(kind).await?;

        // Populate local participant info fields
        if !is_raw_packet {
            packet.participant_identity = self.participant_info.identity.to_string();
            packet.participant_sid = self.participant_info.sid.to_string();
        }

        let packet_value = packet.value.take();

        // Handle encryption for all data packet types when DC encryption is enabled
        if let (Some(e2ee_manager), Some(value)) = (&self.e2ee_manager, packet_value.clone()) {
            if e2ee_manager.is_dc_encryption_enabled() {
                // Create EncryptedPacketPayload from the original value if it's an encryptable type
                let encrypted_payload_value = convert_data_packet_to_encrypted_value(value);

                if let Some(encrypted_payload_value) = encrypted_payload_value {
                    // Create EncryptedPacketPayload and encrypt it
                    let encrypted_payload =
                        proto::EncryptedPacketPayload { value: Some(encrypted_payload_value) };

                    // Encode the payload and encrypt it
                    let payload_bytes = encrypted_payload.encode_to_vec();
                    let key_index = e2ee_manager
                        .key_provider()
                        .map(|kp| kp.get_latest_key_index() as u32)
                        .unwrap_or(0);
                    match e2ee_manager.encrypt_data(
                        &payload_bytes,
                        &self.participant_info.identity.0,
                        key_index,
                    ) {
                        Ok(encrypted_data) => {
                            // Replace with EncryptedPacket variant
                            packet.value = Some(proto::data_packet::Value::EncryptedPacket(
                                proto::EncryptedPacket {
                                    encrypted_value: encrypted_data.data,
                                    iv: encrypted_data.iv,
                                    key_index: encrypted_data.key_index,
                                    encryption_type: e2ee_manager.encryption_type().into(),
                                },
                            ));
                        }
                        Err(e) => {
                            return Err(EngineError::Internal(
                                format!("Failed to encrypt data packet: {}", e).into(),
                            ));
                        }
                    }
                } else {
                    // Packet type shouldn't be encrypted, restore original
                    packet.value = packet_value;
                }
            } else {
                // DC encryption not enabled, restore original packet
                packet.value = packet_value;
            }
        } else {
            // No e2ee_manager or no packet value - restore original
            packet.value = packet_value;
        }

        // Send through the queue with backpressure
        let (completion_tx, completion_rx) = oneshot::channel();
        let ev = DataChannelEvent {
            kind,
            detail: DataChannelEventDetail::PublishPacket(PublishPacketRequest {
                packet,
                completion_tx,
            }),
        };
        if let Err(err) = self.dc_emitter.send(ev) {
            return Err(EngineError::Internal(
                format!("Failed to enqueue publish packet request: {:?}", err).into(),
            ));
        };
        completion_rx.await.map_err(|e| {
            EngineError::Internal(format!("failed to receive data from dc_task: {:?}", e).into())
        })?
    }

    /// This reconnection if more seemless compared to the full reconnection implemented in
    /// ['RTCEngine']
    async fn restart(&self) -> EngineResult<proto::ReconnectResponse> {
        let reconnect_response = self.signal_client.restart().await?;
        log::debug!("received reconnect response: {:?}", reconnect_response);

        let rtc_config =
            make_rtc_config_reconnect(reconnect_response.clone(), self.options.rtc_config.clone());
        self.publisher_pc.peer_connection().set_configuration(rtc_config.clone())?;
        if let Some(ref sub_pc) = self.subscriber_pc {
            sub_pc.peer_connection().set_configuration(rtc_config)?;
        }

        let ev = DataChannelEvent {
            kind: DataPacketKind::Reliable,
            detail: DataChannelEventDetail::RetryFrom(reconnect_response.last_message_seq),
        };
        if let Err(err) = self.dc_emitter.send(ev) {
            log::error!("Failed to request reliable retry: {:?}", err);
        }

        Ok(reconnect_response)
    }

    async fn restart_publisher(&self) -> EngineResult<()> {
        if self.has_published.load(Ordering::Acquire) {
            self.publisher_pc
                .create_and_send_offer(OfferOptions { ice_restart: true, ..Default::default() })
                .await?;
        }
        Ok(())
    }

    /// Timeout after ['MAX_ICE_CONNECT_TIMEOUT']
    async fn wait_pc_connection(&self) -> EngineResult<()> {
        let wait_connected = async move {
            loop {
                if self.closed.load(Ordering::Acquire) {
                    return Err(EngineError::Connection("closed".into()));
                }

                let publisher_connected = self.publisher_pc.is_connected();
                let subscriber_connected = if self.single_pc_mode {
                    true // No subscriber in single PC mode
                } else {
                    self.subscriber_pc.as_ref().map(|pc| pc.is_connected()).unwrap_or(true)
                };

                let need_publisher = self.has_published.load(Ordering::Acquire);

                if subscriber_connected && (!need_publisher || publisher_connected) {
                    break;
                }

                livekit_runtime::sleep(Duration::from_millis(50)).await;
            }

            Ok(())
        };

        tokio::select! {
            res = wait_connected => res,
            _ = sleep(ICE_CONNECT_TIMEOUT) => {
                let err = EngineError::Connection("wait_pc_connection timed out".into());
                Err(err)
            }
        }
    }

    /// Start publisher negotiation
    fn publisher_negotiation_needed(self: &Arc<Self>) {
        let fast_publish = self.fast_publish.load(Ordering::Acquire);
        self.has_published.store(true, Ordering::Release);

        log::debug!("publisher_negotiation_needed: fast_publish={}", fast_publish);

        if fast_publish {
            if self.negotiation_queue.waiting_for_answer.load(Ordering::Acquire) {
                log::debug!("already waiting for answer, marking for retry");
                let mut state = self.negotiation_queue.state.lock();
                *state = NegotiationState::PendingRetry;
                return;
            }
            self.queue_negotiation();
        } else {
            self.debounce_negotiation();
        }
    }

    fn queue_negotiation(self: &Arc<Self>) {
        let mut state = self.negotiation_queue.state.lock();

        match *state {
            NegotiationState::Idle => {
                if self.negotiation_queue.task_running.swap(true, Ordering::AcqRel) {
                    log::debug!("queue_negotiation: task already running, marking for retry");
                    *state = NegotiationState::PendingRetry;
                    return;
                }

                log::debug!("queue_negotiation: starting new negotiation");
                *state = NegotiationState::InProgress;
                drop(state);

                let session = self.clone();
                livekit_runtime::spawn(async move {
                    session.execute_negotiation_with_retry().await;
                    session.negotiation_queue.task_running.store(false, Ordering::Release);
                });
            }
            NegotiationState::InProgress => {
                log::debug!("queue_negotiation: marking for retry");
                *state = NegotiationState::PendingRetry;
            }
            NegotiationState::PendingRetry => {
                log::debug!("queue_negotiation: already pending retry");
            }
        }
    }

    async fn execute_negotiation_with_retry(self: &Arc<Self>) {
        loop {
            log::debug!("negotiating the publisher (fast mode)");

            self.negotiation_queue.waiting_for_answer.store(true, Ordering::Release);

            if let Err(err) = self.publisher_pc.create_and_send_offer(OfferOptions::default()).await
            {
                log::error!("failed to negotiate the publisher: {:?}", err);
                self.negotiation_queue.waiting_for_answer.store(false, Ordering::Release);
            } else {
                log::debug!("offer sent, waiting for answer...");

                let timeout = tokio::time::sleep(Duration::from_secs(10));
                tokio::pin!(timeout);

                tokio::select! {
                    _ = self.negotiation_queue.waker.notified() => {
                        log::debug!("answer received successfully");
                    }
                    _ = &mut timeout => {
                        log::debug!("timeout waiting for answer");
                        self.negotiation_queue.waiting_for_answer.store(false, Ordering::Release);
                    }
                }
            }

            let mut state = self.negotiation_queue.state.lock();
            match *state {
                NegotiationState::PendingRetry => {
                    log::debug!("retrying negotiation");
                    *state = NegotiationState::InProgress;
                    drop(state);
                    continue;
                }
                _ => {
                    log::debug!("negotiation completed");
                    *state = NegotiationState::Idle;
                    break;
                }
            }
        }
    }

    fn debounce_negotiation(self: &Arc<Self>) {
        let mut debouncer = self.negotiation_debouncer.lock();

        // call() returns an error if the debouncer has finished
        if debouncer.is_none() || debouncer.as_ref().unwrap().call().is_err() {
            let session = self.clone();

            *debouncer = Some(debouncer::debounce(PUBLISHER_NEGOTIATION_FREQUENCY, async move {
                log::debug!("negotiating the publisher (debounced)");
                if let Err(err) =
                    session.publisher_pc.create_and_send_offer(OfferOptions::default()).await
                {
                    log::error!("failed to negotiate the publisher: {:?}", err);
                }
            }));
        }
    }

    /// Ensure the Publisher PC is connected, if not, start the negotiation
    /// This is required when sending data to the server
    async fn ensure_publisher_connected(
        self: &Arc<Self>,
        kind: DataPacketKind,
    ) -> EngineResult<()> {
        if !self.has_published.load(Ordering::Acquire) {
            // The publisher has never been connected, start the negotiation
            // If the connection fails, the reconnection logic will be triggered
            self.publisher_negotiation_needed();
        }

        let dc = self.data_channel(SignalTarget::Publisher, kind).unwrap();
        if dc.state() == DataChannelState::Open {
            return Ok(());
        }

        // Wait until the PeerConnection is connected
        let wait_connected = async {
            while !self.publisher_pc.is_connected() || dc.state() != DataChannelState::Open {
                if self.closed.load(Ordering::Acquire) {
                    return Err(EngineError::Connection("closed".into()));
                }

                livekit_runtime::sleep(Duration::from_millis(50)).await;
            }

            Ok(())
        };

        let result = tokio::select! {
            res = wait_connected => res,
            _ = sleep(ICE_CONNECT_TIMEOUT) => {
                let err = EngineError::Connection("could not establish publisher connection: timeout".into());
                log::error!("{}", err);
                Err(err)
            }
        };

        result
    }

    fn data_channel(&self, target: SignalTarget, kind: DataPacketKind) -> Option<DataChannel> {
        if target == SignalTarget::Publisher {
            if kind == DataPacketKind::Reliable {
                Some(self.reliable_dc.clone())
            } else {
                Some(self.lossy_dc.clone())
            }
        } else if target == SignalTarget::Subscriber {
            if kind == DataPacketKind::Reliable {
                self.sub_reliable_dc.lock().clone()
            } else {
                self.sub_lossy_dc.lock().clone()
            }
        } else {
            unreachable!()
        }
    }

    fn data_channel_receive_states(self: &Arc<Self>) -> Vec<proto::DataChannelReceiveState> {
        let mut state = self.packet_rx_state.lock();
        state
            .iter()
            .map(|(publisher_sid, last_seq)| proto::DataChannelReceiveState {
                publisher_sid: publisher_sid.to_string(),
                last_seq: *last_seq,
            })
            .collect()
    }

    async fn get_response(&self, request_id: u32) -> proto::RequestResponse {
        let (tx, rx) = oneshot::channel();
        self.pending_requests.lock().insert(request_id, tx);
        rx.await.unwrap()
    }
}

macro_rules! make_rtc_config {
    ($fncname:ident, $proto:ty) => {
        fn $fncname(value: $proto, mut config: RtcConfiguration) -> RtcConfiguration {
            if config.ice_servers.is_empty() {
                for ice_server in value.ice_servers.clone() {
                    config.ice_servers.push(IceServer {
                        urls: ice_server.urls,
                        username: ice_server.username,
                        password: ice_server.credential,
                    })
                }
            }

            if let Some(client_configuration) = value.client_configuration {
                if client_configuration.force_relay() == proto::ClientConfigSetting::Enabled {
                    config.ice_transport_type = IceTransportsType::Relay;
                }
            }

            config
        }
    };
}

make_rtc_config!(make_rtc_config_join, proto::JoinResponse);
make_rtc_config!(make_rtc_config_reconnect, proto::ReconnectResponse);
