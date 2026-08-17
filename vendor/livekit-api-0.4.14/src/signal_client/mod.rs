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
    borrow::Cow,
    fmt::Debug,
    sync::{
        atomic::{AtomicBool, AtomicU32, Ordering},
        Arc,
    },
    time::{Duration, SystemTime, UNIX_EPOCH},
};

use base64::{engine::general_purpose::STANDARD as BASE64_STANDARD, Engine};
use http::StatusCode;
use livekit_protocol as proto;
use livekit_runtime::{interval, sleep, Instant, JoinHandle};
use parking_lot::Mutex;
use prost::Message;
use thiserror::Error;
use tokio::sync::{mpsc, Mutex as AsyncMutex, RwLock as AsyncRwLock};

#[cfg(feature = "signal-client-tokio")]
use tokio_tungstenite::tungstenite::Error as WsError;

#[cfg(feature = "__signal-client-async-compatible")]
use async_tungstenite::tungstenite::Error as WsError;

use crate::{http_client, signal_client::signal_stream::SignalStream};

mod region;
mod signal_stream;

pub use region::RegionUrlProvider;

pub type SignalEmitter = mpsc::UnboundedSender<SignalEvent>;
pub type SignalEvents = mpsc::UnboundedReceiver<SignalEvent>;
pub type SignalResult<T> = Result<T, SignalError>;

pub const JOIN_RESPONSE_TIMEOUT: Duration = Duration::from_secs(5);
pub const SIGNAL_CONNECT_TIMEOUT: Duration = Duration::from_secs(5);
const REGION_FETCH_TIMEOUT: Duration = Duration::from_secs(3);
const VALIDATE_TIMEOUT: Duration = Duration::from_secs(3);
pub const PROTOCOL_VERSION: u32 = 16;

#[derive(Error, Debug)]
pub enum SignalError {
    #[error("ws failure: {0}")]
    WsError(#[from] WsError),
    #[error("failed to parse the url: {0}")]
    UrlParse(String),
    #[error("access token has invalid characters")]
    TokenFormat,
    #[error("client error: {0} - {1}")]
    Client(StatusCode, String),
    #[error("server error: {0} - {1}")]
    Server(StatusCode, String),
    #[error("failed to decode messages from server: {0}")]
    ProtoParse(#[from] prost::DecodeError),
    #[error("{0}")]
    Timeout(String),
    #[error("failed to send message to the server")]
    SendError,
    #[error("failed to retrieve region info: {0}")]
    RegionError(String),
    #[error("server sent leave during reconnect: reason={reason:?}, action={action:?}")]
    LeaveRequest { reason: proto::DisconnectReason, action: proto::leave_request::Action },
}

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SignalSdkOptions {
    pub sdk: String,
    pub sdk_version: Option<String>,
}

impl Default for SignalSdkOptions {
    fn default() -> Self {
        Self { sdk: "rust".to_string(), sdk_version: None }
    }
}

#[cfg(all(feature = "signal-client-tokio", feature = "rustls-tls-native-roots"))]
#[derive(Debug, Default, Clone)]
pub struct TlsConfig(pub Option<tokio_rustls::rustls::ClientConfig>);

#[cfg(not(all(feature = "signal-client-tokio", feature = "rustls-tls-native-roots")))]
#[derive(Debug, Default, Clone)]
pub struct TlsConfig;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub struct SignalOptions {
    pub auto_subscribe: bool,
    pub adaptive_stream: bool,
    pub sdk_options: SignalSdkOptions,
    /// Enable single peer connection mode
    pub single_peer_connection: bool,
    /// Timeout for each individual signal connection attempt
    pub connect_timeout: Duration,
    /// Custom TLS config
    pub tls_config: TlsConfig,
}

impl Default for SignalOptions {
    fn default() -> Self {
        Self {
            auto_subscribe: true,
            adaptive_stream: false,
            sdk_options: SignalSdkOptions::default(),
            single_peer_connection: false,
            connect_timeout: SIGNAL_CONNECT_TIMEOUT,
            tls_config: TlsConfig::default(),
        }
    }
}

pub enum SignalEvent {
    /// Received a message from the server
    Message(Box<proto::signal_response::Message>),

    /// Signal connection closed, SignalClient::restart() can be called to reconnect
    Close(Cow<'static, str>),
}

struct SignalInner {
    stream: AsyncRwLock<Option<SignalStream>>,
    token: Mutex<String>, // Token can be refreshed
    reconnecting: AtomicBool,
    queue: AsyncMutex<Vec<proto::signal_request::Message>>,
    url: String,
    options: SignalOptions,
    join_response: proto::JoinResponse,
    request_id: AtomicU32,
    /// Tracks whether single PC mode is active (v1 path succeeded)
    single_pc_mode_active: bool,
}

pub struct SignalClient {
    inner: Arc<SignalInner>,
    emitter: SignalEmitter,
    handle: Mutex<Option<JoinHandle<()>>>,
}

impl Debug for SignalClient {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("SignalClient")
            .field("url", &self.url())
            .field("join_response", &self.join_response())
            .field("options", &self.options())
            .finish()
    }
}

impl SignalClient {
    pub async fn connect(
        url: &str,
        token: &str,
        options: SignalOptions,
    ) -> SignalResult<(Self, proto::JoinResponse, SignalEvents)> {
        let handle_success = |inner: Arc<SignalInner>, join_response, stream_events| {
            let (emitter, events) = mpsc::unbounded_channel();
            let signal_task =
                livekit_runtime::spawn(signal_task(inner.clone(), emitter.clone(), stream_events));

            (Self { inner, emitter, handle: Mutex::new(Some(signal_task)) }, join_response, events)
        };

        match SignalInner::connect(url, token, options.clone()).await {
            Ok((inner, join_response, stream_events)) => {
                return Ok(handle_success(inner, join_response, stream_events))
            }
            Err(err) => {
                // fallback to region urls
                if matches!(&err, SignalError::WsError(WsError::Http(e)) if e.status() != 403) {
                    log::error!("unexpected signal error: {}", err.to_string());
                }
                let urls = RegionUrlProvider::fetch_region_urls(url.into(), token.into()).await?;
                let mut last_err = err;

                for url in urls.iter() {
                    log::info!("fallback connection to: {}", url);
                    match SignalInner::connect(url, token, options.clone()).await {
                        Ok((inner, join_response, stream_events)) => {
                            return Ok(handle_success(inner, join_response, stream_events))
                        }
                        Err(err) => last_err = err,
                    }
                }

                Err(last_err)
            }
        }
    }

    /// Restart the connection to the server
    /// This will automatically flush the queue
    pub async fn restart(&self) -> SignalResult<proto::ReconnectResponse> {
        self.close().await;

        let (reconnect_response, stream_events) = self.inner.restart().await?;
        let signal_task = livekit_runtime::spawn(signal_task(
            self.inner.clone(),
            self.emitter.clone(),
            stream_events,
        ));

        *self.handle.lock() = Some(signal_task);
        Ok(reconnect_response)
    }

    /// Send a signal to the server (e.g. publish, subscribe, etc.)
    /// This will automatically queue the message if the connection fails
    /// The queue is flushed on the next restart
    pub async fn send(&self, signal: proto::signal_request::Message) {
        self.inner.send(signal).await
    }

    /// Close the connection to the server
    pub async fn close(&self) {
        self.inner.close(true).await;

        let handle = self.handle.lock().take();
        if let Some(signal_task) = handle {
            let _ = signal_task.await;
        }
    }

    /// Returns Initial JoinResponse
    pub fn join_response(&self) -> proto::JoinResponse {
        self.inner.join_response.clone()
    }

    /// Returns the initial options
    pub fn options(&self) -> SignalOptions {
        self.inner.options.clone()
    }

    /// Returns the initial URL
    pub fn url(&self) -> String {
        self.inner.url.clone()
    }

    /// Returns the last refreshed token (Or initial token if not refreshed yet)
    pub fn token(&self) -> String {
        self.inner.token.lock().clone()
    }

    /// Increment request_id for user-initiated requests and [`RequestResponse`][`proto::RequestResponse`]s
    pub fn next_request_id(&self) -> u32 {
        self.inner.next_request_id().clone()
    }

    /// Returns whether single peer connection mode is active.
    /// This is determined by whether the /rtc/v1 path was used successfully.
    pub fn is_single_pc_mode_active(&self) -> bool {
        self.inner.is_single_pc_mode_active()
    }
}

impl SignalInner {
    pub async fn connect(
        url: &str,
        token: &str,
        options: SignalOptions,
    ) -> SignalResult<(
        Arc<Self>,
        proto::JoinResponse,
        mpsc::UnboundedReceiver<Box<proto::signal_response::Message>>,
    )> {
        // Try v1 path first if single_peer_connection is enabled
        let use_v1_path = options.single_peer_connection;
        // For initial connection: reconnect=false, reconnect_reason=None, participant_sid=""
        let lk_url = get_livekit_url(url, &options, use_v1_path, false, None, "")?;
        // Try to connect to the SignalClient
        let (stream, mut events, single_pc_mode_active) = match SignalStream::connect(
            lk_url.clone(),
            token,
            options.connect_timeout,
            options.tls_config.clone(),
        )
        .await
        {
            Ok((new_stream, stream_events)) => {
                log::debug!(
                    "signal connection successful: path={}, single_pc_mode={}",
                    if use_v1_path { "v1" } else { "v0" },
                    use_v1_path
                );
                (new_stream, stream_events, use_v1_path)
            }
            Err(err) => {
                log::warn!(
                    "signal connection failed on {} path: {:?}",
                    if use_v1_path { "v1" } else { "v0" },
                    err
                );

                if let SignalError::TokenFormat = err {
                    return Err(err);
                }

                // Only fallback to v0 if the v1 endpoint returned 404 (not found).
                // Other errors (401, 403, 500, etc.) indicate real issues that shouldn't
                // be masked by falling back to a different signaling mode.
                let is_not_found =
                    matches!(&err, SignalError::WsError(WsError::Http(e)) if e.status() == 404);

                if use_v1_path && is_not_found {
                    let lk_url_v0 = get_livekit_url(url, &options, false, false, None, "")?;
                    log::warn!("v1 path not found (404), falling back to v0 path");
                    match SignalStream::connect(
                        lk_url_v0.clone(),
                        token,
                        options.connect_timeout,
                        options.tls_config.clone(),
                    )
                    .await
                    {
                        Ok((new_stream, stream_events)) => (new_stream, stream_events, false),
                        Err(err) => {
                            log::error!("v0 fallback also failed: {:?}", err);
                            if let SignalError::TokenFormat = err {
                                return Err(err);
                            }
                            Self::validate(lk_url_v0).await?;
                            return Err(err);
                        }
                    }
                } else {
                    // Connection failed, try to retrieve more information
                    Self::validate(lk_url).await?;
                    return Err(err);
                }
            }
        };

        let join_response = get_join_response(&mut events).await?;

        // Successfully connected to the SignalClient
        let inner = Arc::new(SignalInner {
            stream: AsyncRwLock::new(Some(stream)),
            token: Mutex::new(token.to_owned()),
            reconnecting: AtomicBool::new(false),
            queue: Default::default(),
            options,
            url: url.to_string(),
            join_response: join_response.clone(),
            request_id: AtomicU32::new(1),
            single_pc_mode_active,
        });

        Ok((inner, join_response, events))
    }

    /// Validate the connection by calling rtc/validate
    async fn validate(ws_url: url::Url) -> SignalResult<()> {
        let validate_url = get_validate_url(ws_url);

        let validate_fut = async {
            if let Ok(res) = http_client::get(validate_url.as_str()).await {
                let status = res.status();
                let body = res.text().await.ok().unwrap_or_default();

                if status.is_client_error() {
                    return Err(SignalError::Client(status, body));
                } else if status.is_server_error() {
                    return Err(SignalError::Server(status, body));
                }
            }

            Ok(())
        };

        livekit_runtime::timeout(VALIDATE_TIMEOUT, validate_fut)
            .await
            .map_err(|_| SignalError::Timeout("validate request timed out".into()))?
    }

    /// Returns whether single peer connection mode is active
    pub fn is_single_pc_mode_active(&self) -> bool {
        self.single_pc_mode_active
    }

    /// Restart is called when trying to resume the room (RtcSession resume)
    pub async fn restart(
        self: &Arc<Self>,
    ) -> SignalResult<(
        proto::ReconnectResponse,
        mpsc::UnboundedReceiver<Box<proto::signal_response::Message>>,
    )> {
        self.close(false).await;

        // Lock while we are reconnecting
        let mut stream = self.stream.write().await;

        self.reconnecting.store(true, Ordering::Release);
        scopeguard::defer!(self.reconnecting.store(false, Ordering::Release));

        let sid = &self.join_response.participant.as_ref().unwrap().sid;
        let token = self.token.lock().clone();

        // Use the same path that succeeded during initial connection
        // For reconnects: reconnect=true, participant_sid=sid
        // For v1 path: reconnect and sid are encoded in the join_request protobuf
        // For v0 path: reconnect and sid are added as separate query parameters
        let lk_url =
            get_livekit_url(&self.url, &self.options, self.single_pc_mode_active, true, None, sid)
                .unwrap();

        let (new_stream, mut events) = SignalStream::connect(
            lk_url,
            &token,
            self.options.connect_timeout,
            self.options.tls_config.clone(),
        )
        .await?;
        let reconnect_response = get_reconnect_response(&mut events).await?;
        *stream = Some(new_stream);

        drop(stream);
        self.flush_queue().await;
        Ok((reconnect_response, events))
    }

    /// Close the connection
    pub async fn close(&self, notify_close: bool) {
        if let Some(stream) = self.stream.write().await.take() {
            stream.close(notify_close).await;
        }
    }

    /// Send a signal to the server
    pub async fn send(&self, signal: proto::signal_request::Message) {
        if self.reconnecting.load(Ordering::Acquire) {
            self.queue_message(signal).await;
            return;
        }

        self.flush_queue().await; // The queue must be flusehd before sending any new signal

        if let Some(stream) = self.stream.read().await.as_ref() {
            if let Err(SignalError::SendError) = stream.send(signal.clone()).await {
                self.queue_message(signal).await;
            }
        }
    }

    async fn queue_message(&self, signal: proto::signal_request::Message) {
        if is_queuable(&signal) {
            self.queue.lock().await.push(signal);
        }
    }

    pub async fn flush_queue(&self) {
        let mut queue = self.queue.lock().await;
        if queue.is_empty() {
            return;
        }

        if let Some(stream) = self.stream.read().await.as_ref() {
            for signal in queue.drain(..) {
                // log::warn!("sending queued signal: {:?}", signal);

                if let Err(err) = stream.send(signal).await {
                    log::error!("failed to send queued signal: {}", err); // Lost message
                }
            }
        }
    }

    /// Increment request_id for user-initiated requests and [`RequestResponse`][`proto::RequestResponse`]s
    pub fn next_request_id(&self) -> u32 {
        self.request_id.fetch_add(1, Ordering::SeqCst)
    }
}

/// Middleware task to receive SignalStream events and handle SignalClient specific logic
async fn signal_task(
    inner: Arc<SignalInner>,
    emitter: SignalEmitter, // Public emitter
    mut internal_events: mpsc::UnboundedReceiver<Box<proto::signal_response::Message>>,
) {
    let mut ping_interval = interval(Duration::from_secs(inner.join_response.ping_interval as u64));
    let timeout_duration = Duration::from_secs(inner.join_response.ping_timeout as u64);
    let ping_timeout = sleep(timeout_duration);
    tokio::pin!(ping_timeout);

    let mut rtt = 0; // TODO(theomonnom): Should we expose SignalClient rtt?

    loop {
        tokio::select! {
            signal = internal_events.recv() => {
                if let Some(signal) = signal {
                    // Received a message from the server
                    match signal.as_ref() {
                        proto::signal_response::Message::RefreshToken(ref token) => {
                            // Refresh the token so the client can still reconnect if the initial join token expired
                            *inner.token.lock() = token.clone();
                        }
                        proto::signal_response::Message::PongResp(ref pong) => {
                            // Reset the ping_timeout if we received a pong
                            let now = SystemTime::now()
                                .duration_since(UNIX_EPOCH)
                                .unwrap()
                                .as_millis() as i64;

                            rtt = now - pong.last_ping_timestamp;
                        }
                        _ => {}
                    }

                    ping_timeout.as_mut().reset(Instant::now() + timeout_duration);

                    let _ = emitter.send(SignalEvent::Message(signal));
                } else {
                    let _ = emitter.send(SignalEvent::Close("stream closed".into()));
                    break; // Stream closed
                }
            }
            _ = ping_interval.tick() => {
                let now = SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .unwrap()
                    .as_millis() as i64;

                let ping = proto::signal_request::Message::PingReq(proto::Ping{
                    timestamp: now,
                    rtt,
                });

                inner.send(ping).await;
            }
            _ = &mut ping_timeout => {
                let _ = emitter.send(SignalEvent::Close("ping timeout".into()));
                break;
            }
        }
    }

    inner.close(true).await; // Make sure to always close the ws connection when the loop is terminated
}

/// Check if the signal is queuable
/// Not every signal should be sent after signal reconnection
fn is_queuable(signal: &proto::signal_request::Message) -> bool {
    !matches!(
        signal,
        proto::signal_request::Message::SyncState(_)
            | proto::signal_request::Message::Trickle(_)
            | proto::signal_request::Message::Offer(_)
            | proto::signal_request::Message::Answer(_)
            | proto::signal_request::Message::Simulate(_)
            | proto::signal_request::Message::Leave(_)
    )
}

/// Create the base64-encoded WrappedJoinRequest parameter required for v1 path
///
/// Parameters:
/// - options: SignalOptions containing auto_subscribe, adaptive_stream, etc.
/// - reconnect: true if this is a reconnection attempt
/// - participant_sid: the participant SID (only used during reconnection)
fn create_join_request_param(
    options: &SignalOptions,
    reconnect: bool,
    reconnect_reason: Option<i32>,
    participant_sid: &str,
) -> String {
    let connection_settings = proto::ConnectionSettings {
        auto_subscribe: options.auto_subscribe,
        adaptive_stream: options.adaptive_stream,
        ..Default::default()
    };

    let client_info = proto::ClientInfo {
        sdk: proto::client_info::Sdk::Rust as i32,
        version: options.sdk_options.sdk_version.clone().unwrap_or_default(),
        protocol: PROTOCOL_VERSION as i32,
        os: std::env::consts::OS.to_string(),
        ..Default::default()
    };

    let mut join_request = proto::JoinRequest {
        client_info: Some(client_info),
        connection_settings: Some(connection_settings),
        reconnect,
        ..Default::default()
    };

    // Only set participant_sid if non-empty (for reconnects)
    if !participant_sid.is_empty() {
        join_request.participant_sid = participant_sid.to_string();
    }

    // Only set reconnect_reason if provided
    if let Some(reason) = reconnect_reason {
        join_request.reconnect_reason = reason;
    }

    // Serialize JoinRequest to bytes
    let join_request_bytes = join_request.encode_to_vec();

    // Create WrappedJoinRequest (JS doesn't explicitly set compression, so default is NONE)
    let wrapped_join_request =
        proto::WrappedJoinRequest { join_request: join_request_bytes, ..Default::default() };

    // Serialize WrappedJoinRequest to bytes and base64 encode
    let wrapped_bytes = wrapped_join_request.encode_to_vec();
    BASE64_STANDARD.encode(&wrapped_bytes)
}

/// Build the LiveKit WebSocket URL for connection
///
/// Parameters:
/// - url: the base server URL
/// - options: SignalOptions
/// - use_v1_path: if true, use /rtc/v1 (single PC mode), otherwise /rtc (dual PC mode)
/// - reconnect: true if this is a reconnection attempt
/// - reconnect_reason: reason for reconnection (only used when reconnect=true)
/// - participant_sid: the participant SID (only used during reconnection)
fn get_livekit_url(
    url: &str,
    options: &SignalOptions,
    use_v1_path: bool,
    reconnect: bool,
    reconnect_reason: Option<i32>,
    participant_sid: &str,
) -> SignalResult<url::Url> {
    let mut lk_url = url::Url::parse(url).map_err(|err| SignalError::UrlParse(err.to_string()))?;

    if !lk_url.has_host() {
        return Err(SignalError::UrlParse("missing host or scheme".into()));
    }

    // Automatically switch to websocket scheme when using user is providing http(s) scheme
    if lk_url.scheme() == "https" {
        lk_url.set_scheme("wss").unwrap();
    } else if lk_url.scheme() == "http" {
        lk_url.set_scheme("ws").unwrap();
    } else if lk_url.scheme() != "wss" && lk_url.scheme() != "ws" {
        return Err(SignalError::UrlParse(format!("unsupported scheme: {}", lk_url.scheme())));
    }

    if let Ok(mut segs) = lk_url.path_segments_mut() {
        segs.push("rtc");
        if use_v1_path {
            segs.push("v1");
        }
    }

    if use_v1_path {
        // For v1 path (single PC mode): only join_request param
        // All other info (sdk, protocol, auto_subscribe, etc.) is inside the JoinRequest protobuf
        let join_request_param =
            create_join_request_param(options, reconnect, reconnect_reason, participant_sid);
        lk_url.query_pairs_mut().append_pair("join_request", &join_request_param);
    } else {
        // For v0 path (dual PC mode): use URL query parameters
        lk_url
            .query_pairs_mut()
            .append_pair("sdk", options.sdk_options.sdk.as_str())
            .append_pair("protocol", PROTOCOL_VERSION.to_string().as_str())
            .append_pair("auto_subscribe", if options.auto_subscribe { "1" } else { "0" })
            .append_pair("adaptive_stream", if options.adaptive_stream { "1" } else { "0" });

        if let Some(sdk_version) = &options.sdk_options.sdk_version {
            lk_url.query_pairs_mut().append_pair("version", sdk_version.as_str());
        }

        // For reconnects in v0 path, add reconnect and sid as separate query parameters
        if reconnect {
            lk_url
                .query_pairs_mut()
                .append_pair("reconnect", "1")
                .append_pair("sid", participant_sid);
        }
    }

    Ok(lk_url)
}

/// Convert a WebSocket URL (with /rtc or /rtc/v1 path) to the validate endpoint URL
fn get_validate_url(mut ws_url: url::Url) -> url::Url {
    ws_url.set_scheme(if ws_url.scheme() == "wss" { "https" } else { "http" }).unwrap();
    // ws_url already has /rtc or /rtc/v1 from get_livekit_url, so only append /validate
    if let Ok(mut segs) = ws_url.path_segments_mut() {
        segs.push("validate");
    }
    ws_url
}

macro_rules! get_async_message {
    ($fnc:ident, $pattern:pat => $result:expr, $ty:ty) => {
        async fn $fnc(
            receiver: &mut mpsc::UnboundedReceiver<Box<proto::signal_response::Message>>,
        ) -> SignalResult<$ty> {
            let join = async {
                while let Some(event) = receiver.recv().await {
                    if let $pattern = *event {
                        return Ok($result);
                    }
                }

                Err(WsError::ConnectionClosed)?
            };

            livekit_runtime::timeout(JOIN_RESPONSE_TIMEOUT, join).await.map_err(|_| {
                SignalError::Timeout(format!("failed to receive {}", std::any::type_name::<$ty>()))
            })?
        }
    };
}

get_async_message!(
    get_join_response,
    proto::signal_response::Message::Join(msg) => msg,
    proto::JoinResponse
);

async fn get_reconnect_response(
    receiver: &mut mpsc::UnboundedReceiver<Box<proto::signal_response::Message>>,
) -> SignalResult<proto::ReconnectResponse> {
    let join = async {
        while let Some(event) = receiver.recv().await {
            match *event {
                proto::signal_response::Message::Reconnect(msg) => return Ok(msg),
                proto::signal_response::Message::Leave(leave) => {
                    return Err(SignalError::LeaveRequest {
                        reason: leave.reason(),
                        action: leave.action(),
                    });
                }
                _ => {}
            }
        }

        Err(WsError::ConnectionClosed)?
    };

    livekit_runtime::timeout(JOIN_RESPONSE_TIMEOUT, join).await.map_err(|_| {
        SignalError::Timeout(format!(
            "failed to receive {}",
            std::any::type_name::<proto::ReconnectResponse>()
        ))
    })?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn livekit_url_test() {
        let io = SignalOptions::default();

        assert!(get_livekit_url("localhost:7880", &io, false, false, None, "").is_err());
        assert_eq!(
            get_livekit_url("https://localhost:7880", &io, false, false, None, "")
                .unwrap()
                .scheme(),
            "wss"
        );
        assert_eq!(
            get_livekit_url("http://localhost:7880", &io, false, false, None, "").unwrap().scheme(),
            "ws"
        );
        assert_eq!(
            get_livekit_url("wss://localhost:7880", &io, false, false, None, "").unwrap().scheme(),
            "wss"
        );
        assert_eq!(
            get_livekit_url("ws://localhost:7880", &io, false, false, None, "").unwrap().scheme(),
            "ws"
        );
        assert!(get_livekit_url("ftp://localhost:7880", &io, false, false, None, "").is_err());
    }

    #[test]
    fn validate_url_test() {
        let io = SignalOptions::default();
        let lk_url = get_livekit_url("wss://localhost:7880", &io, false, false, None, "").unwrap();
        let validate_url = get_validate_url(lk_url);

        // Should be /rtc/validate, not /rtc/rtc/validate
        assert_eq!(validate_url.path(), "/rtc/validate");
        assert_eq!(validate_url.scheme(), "https");
    }

    #[cfg(feature = "signal-client-tokio")]
    #[tokio::test]
    async fn signal_stream_connect_timeout() {
        use tokio::net::TcpListener;

        // Bind a TCP listener that accepts connections but never sends data
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Spawn a task that accepts connections but does nothing (simulates a hanging server)
        let _accept_task = tokio::spawn(async move {
            loop {
                let Ok((_socket, _)) = listener.accept().await else {
                    break;
                };
                // Hold the connection open but never write anything
                tokio::time::sleep(Duration::from_secs(60)).await;
            }
        });

        let url = url::Url::parse(&format!("ws://127.0.0.1:{}", addr.port())).unwrap();
        let result = SignalStream::connect(
            url,
            "fake-token",
            Duration::from_millis(500),
            TlsConfig::default(),
        )
        .await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(matches!(err, SignalError::Timeout(_)), "expected Timeout error, got: {:?}", err);
    }

    #[cfg(feature = "signal-client-tokio")]
    #[tokio::test]
    async fn region_fetch_parses_response() {
        use tokio::io::AsyncWriteExt;
        use tokio::net::TcpListener;

        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        // Spawn a task that serves a hand-crafted HTTP response with region JSON
        tokio::spawn(async move {
            let (mut socket, _) = listener.accept().await.unwrap();

            // Read the request (consume it so the connection doesn't stall)
            let mut buf = [0u8; 4096];
            let _ = tokio::io::AsyncReadExt::read(&mut socket, &mut buf).await;

            let body = r#"{"regions":[{"region":"us-east-1","url":"wss://us-east.livekit.cloud","distance":"100"},{"region":"eu-west-1","url":"wss://eu-west.livekit.cloud","distance":"200"}]}"#;

            let response = format!(
                "HTTP/1.1 200 OK\r\nContent-Type: application/json\r\nContent-Length: {}\r\n\r\n{}",
                body.len(),
                body
            );
            socket.write_all(response.as_bytes()).await.unwrap();
        });

        let endpoint = format!("http://127.0.0.1:{}/settings/regions", addr.port());
        let result = region::fetch_from_endpoint(&endpoint, "fake-token").await;

        let urls = result.unwrap();
        assert_eq!(
            urls,
            vec![
                "wss://us-east.livekit.cloud".to_string(),
                "wss://eu-west.livekit.cloud".to_string(),
            ]
        );
    }

    #[cfg(feature = "signal-client-tokio")]
    #[tokio::test]
    async fn region_fetch_timeout() {
        use tokio::net::TcpListener;

        // Bind a listener that accepts but never responds
        let listener = TcpListener::bind("127.0.0.1:0").await.unwrap();
        let addr = listener.local_addr().unwrap();

        tokio::spawn(async move {
            loop {
                let Ok((_socket, _)) = listener.accept().await else {
                    break;
                };
                // Hold connection open, never write
                tokio::time::sleep(Duration::from_secs(60)).await;
            }
        });

        let endpoint = format!("http://127.0.0.1:{}/settings/regions", addr.port());
        let result = region::fetch_from_endpoint(&endpoint, "fake-token").await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        assert!(
            matches!(err, SignalError::RegionError(ref msg) if msg.contains("timed out")),
            "expected RegionError with 'timed out', got: {:?}",
            err
        );
    }
}
