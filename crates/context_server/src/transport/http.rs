use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::{FutureExt as _, Stream, StreamExt, channel::oneshot, select};
use gpui::BackgroundExecutor;
use http_client::{AsyncBody, HttpClient, Request, Response, http::Method};
use parking_lot::Mutex as SyncMutex;
use serde_json::Value;
use std::{
    pin::Pin,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};

use crate::oauth::{self, OAuthTokenProvider, WwwAuthenticate};
use crate::transport::{Transport, TransportShutdownReason};
use crate::types;

/// Typed errors returned by the HTTP transport that callers can downcast from
/// `anyhow::Error` to handle specific failure modes.
#[derive(Debug)]
pub enum TransportError {
    /// The server returned 401 and token refresh either wasn't possible or
    /// failed. The caller should initiate the OAuth authorization flow.
    AuthRequired { www_authenticate: WwwAuthenticate },
}

impl std::fmt::Display for TransportError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TransportError::AuthRequired { .. } => {
                write!(f, "OAuth authorization required")
            }
        }
    }
}

impl std::error::Error for TransportError {}

// Constants from MCP spec
const HEADER_SESSION_ID: &str = "Mcp-Session-Id";
const HEADER_PROTOCOL_VERSION: &str = "MCP-Protocol-Version";
const EVENT_STREAM_MIME_TYPE: &str = "text/event-stream";
const JSON_MIME_TYPE: &str = "application/json";
const MAX_HTTP_ERROR_BODY_BYTES: usize = 16 * 1024;

/// HTTP Transport with session management and SSE support
pub struct HttpTransport {
    http_client: Arc<dyn HttpClient>,
    endpoint: String,
    session_id: Arc<SyncMutex<Option<String>>>,
    /// Negotiated MCP protocol version, populated by `set_protocol_version`
    /// after the initialize handshake. From 2025-06-18 onward the server
    /// requires clients to echo this in the `MCP-Protocol-Version` header on
    /// every subsequent request.
    protocol_version: Arc<SyncMutex<Option<String>>>,
    executor: BackgroundExecutor,
    response_tx: async_channel::Sender<String>,
    response_rx: async_channel::Receiver<String>,
    error_tx: async_channel::Sender<String>,
    error_rx: async_channel::Receiver<String>,
    /// Static headers to include in every request (e.g. from server config).
    headers: HashMap<String, String>,
    /// When set, the transport attaches `Authorization: Bearer` headers and
    /// handles 401 responses with token refresh + retry.
    token_provider: Option<Arc<dyn OAuthTokenProvider>>,
    /// The challenge from the last 401 this transport gave up on; cleared at
    /// the start of each send so it always describes the most recent attempt.
    /// See [`Transport::auth_challenge`].
    auth_challenge: SyncMutex<Option<WwwAuthenticate>>,
    session_expired: AtomicBool,
}

impl HttpTransport {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        endpoint: String,
        headers: HashMap<String, String>,
        executor: BackgroundExecutor,
    ) -> Self {
        Self::new_with_token_provider(http_client, endpoint, headers, executor, None)
    }

    pub fn new_with_token_provider(
        http_client: Arc<dyn HttpClient>,
        endpoint: String,
        headers: HashMap<String, String>,
        executor: BackgroundExecutor,
        token_provider: Option<Arc<dyn OAuthTokenProvider>>,
    ) -> Self {
        let (response_tx, response_rx) = async_channel::unbounded();
        let (error_tx, error_rx) = async_channel::unbounded();

        Self {
            http_client,
            executor,
            endpoint,
            session_id: Arc::new(SyncMutex::new(None)),
            protocol_version: Arc::new(SyncMutex::new(None)),
            response_tx,
            response_rx,
            error_tx,
            error_rx,
            headers,
            token_provider,
            auth_challenge: SyncMutex::new(None),
            session_expired: AtomicBool::new(false),
        }
    }

    /// Build a POST request for the given message body, attaching all standard
    /// headers (content-type, accept, session ID, static headers, and bearer
    /// token if available).
    fn build_request(
        &self,
        message: &[u8],
        is_initialize: bool,
    ) -> Result<http_client::Request<AsyncBody>> {
        let mut request_builder = Request::builder()
            .method(Method::POST)
            .uri(&self.endpoint)
            .header("Content-Type", JSON_MIME_TYPE)
            .header(
                "Accept",
                format!("{}, {}", JSON_MIME_TYPE, EVENT_STREAM_MIME_TYPE),
            );

        for (key, value) in &self.headers {
            request_builder = request_builder.header(key.as_str(), value.as_str());
        }

        // Attach bearer token when a token provider is present.
        if let Some(token) = self.token_provider.as_ref().and_then(|p| p.access_token()) {
            request_builder = request_builder.header("Authorization", format!("Bearer {}", token));
        }

        // Add session ID if we have one (except for initialize).
        if !is_initialize && let Some(ref session_id) = *self.session_id.lock() {
            request_builder = request_builder.header(HEADER_SESSION_ID, session_id.as_str());
        }

        // Echo the negotiated protocol version once initialization has
        // completed. Required by servers speaking MCP 2025-06-18 or later.
        if !is_initialize
            && let Some(ref version) = *self.protocol_version.lock()
            && types::requires_protocol_version_header(version)
        {
            request_builder = request_builder.header(HEADER_PROTOCOL_VERSION, version.as_str());
        }

        Ok(request_builder.body(AsyncBody::from(message.to_vec()))?)
    }

    /// Record the challenge so it remains observable after the failed send
    /// tears down the client (see [`Transport::auth_challenge`]), and build
    /// the typed error for the send itself.
    fn auth_required(&self, www_authenticate: WwwAuthenticate) -> anyhow::Error {
        *self.auth_challenge.lock() = Some(www_authenticate.clone());
        TransportError::AuthRequired { www_authenticate }.into()
    }

    /// Send a message and handle the response based on content type.
    async fn send_message(
        &self,
        message: String,
        mut issued_tx: Option<oneshot::Sender<()>>,
    ) -> Result<()> {
        // The same server instance can be restarted over this transport; a
        // challenge recorded by a previous client generation must not be
        // observed by the current one.
        *self.auth_challenge.lock() = None;

        let outgoing_message = serde_json::from_str::<Value>(&message).ok();
        let request_id = outgoing_message
            .as_ref()
            .and_then(|message| message.get("id").cloned())
            .filter(|id| !id.is_null());
        let is_initialize = outgoing_message
            .as_ref()
            .and_then(|message| message.get("method"))
            .and_then(Value::as_str)
            == Some("initialize");
        if is_initialize {
            self.session_expired.store(false, Ordering::SeqCst);
        }
        let is_notification = request_id.is_none();

        // If we currently have no access token, try refreshing before sending
        // the request so restored but expired sessions do not need an initial
        // 401 round-trip before they can recover.
        if let Some(ref provider) = self.token_provider {
            if provider.access_token().is_none() {
                provider.try_refresh().await.unwrap_or(false);
            }
        }

        let request = self.build_request(message.as_bytes(), is_initialize)?;
        let mut response = self.send_http_request(request, &mut issued_tx).await?;

        // On 401, try refreshing the token and retry once.
        if response.status().as_u16() == 401 {
            let www_auth_header = response
                .headers()
                .get("www-authenticate")
                .and_then(|v| v.to_str().ok())
                .unwrap_or("Bearer");

            let www_authenticate =
                oauth::parse_www_authenticate(www_auth_header).unwrap_or(WwwAuthenticate {
                    resource_metadata: None,
                    scope: None,
                    error: None,
                    error_description: None,
                });

            if let Some(ref provider) = self.token_provider {
                if provider.try_refresh().await.unwrap_or(false) {
                    // Retry with the refreshed token.
                    let retry_request = self.build_request(message.as_bytes(), is_initialize)?;
                    response = self
                        .send_http_request(retry_request, &mut issued_tx)
                        .await?;

                    // If still 401 after refresh, give up.
                    if response.status().as_u16() == 401 {
                        return Err(self.auth_required(www_authenticate));
                    }
                } else {
                    return Err(self.auth_required(www_authenticate));
                }
            } else {
                return Err(self.auth_required(www_authenticate));
            }
        }

        // Handle different response types based on status and content-type.
        match response.status() {
            status if status.is_success() => {
                // Check content type
                let content_type = response
                    .headers()
                    .get("content-type")
                    .and_then(|v| v.to_str().ok());

                // Extract session ID from response headers if present
                if let Some(session_id) = response
                    .headers()
                    .get(HEADER_SESSION_ID)
                    .and_then(|v| v.to_str().ok())
                {
                    *self.session_id.lock() = Some(session_id.to_string());
                    log::debug!("Session ID set: {}", session_id);
                }

                match content_type {
                    Some(ct) if ct.starts_with(JSON_MIME_TYPE) => {
                        // JSON response - read and forward immediately
                        let mut body = String::new();
                        futures::AsyncReadExt::read_to_string(response.body_mut(), &mut body)
                            .await?;

                        // Only send non-empty responses
                        if !body.is_empty() {
                            self.response_tx
                                .send(body)
                                .await
                                .map_err(|_| anyhow!("Failed to send JSON response"))?;
                        }
                    }
                    Some(ct) if ct.starts_with(EVENT_STREAM_MIME_TYPE) => {
                        self.receive_sse_response(response, request_id.as_ref())
                            .await?;
                    }
                    _ => {
                        // For notifications, 202 Accepted with no content type is ok
                        if is_notification && status.as_u16() == 202 {
                            log::debug!("Notification accepted");
                        } else {
                            return Err(anyhow!("Unexpected content type: {:?}", content_type));
                        }
                    }
                }
            }
            status if status.as_u16() == 202 => {
                // Accepted - notification acknowledged, no response needed
                log::debug!("Notification accepted");
            }
            _ => {
                let status = response.status();
                let session_expired =
                    status.as_u16() == 404 && self.session_id.lock().take().is_some();
                if session_expired {
                    self.session_expired.store(true, Ordering::SeqCst);
                    *self.protocol_version.lock() = None;
                    return Err(anyhow!("MCP session expired: HTTP {status}"));
                }
                let mut error_body = Vec::new();
                let mut limited_body = futures::AsyncReadExt::take(
                    response.body_mut(),
                    (MAX_HTTP_ERROR_BODY_BYTES + 1) as u64,
                );
                futures::AsyncReadExt::read_to_end(&mut limited_body, &mut error_body).await?;
                let body_was_truncated = error_body.len() > MAX_HTTP_ERROR_BODY_BYTES;
                error_body.truncate(MAX_HTTP_ERROR_BODY_BYTES);

                let error_body_text = String::from_utf8_lossy(&error_body);
                let error_body_text = error_body_text.trim();
                let error_message = match (error_body_text.is_empty(), body_was_truncated) {
                    (true, false) => format!("HTTP {status}"),
                    (true, true) => format!("HTTP {status}: [response body truncated]"),
                    (false, false) => format!("HTTP {status}: {error_body_text}"),
                    (false, true) => {
                        format!("HTTP {status}: {error_body_text} [response body truncated]")
                    }
                };

                if let Some(request_id) = request_id {
                    let response_message = if !body_was_truncated
                        && serde_json::from_slice::<Value>(&error_body)
                            .ok()
                            .is_some_and(|body| {
                                body.get("jsonrpc").and_then(Value::as_str) == Some("2.0")
                                    && body.get("id") == Some(&request_id)
                                    && body.get("error").is_some_and(Value::is_object)
                            }) {
                        String::from_utf8(error_body)
                            .map_err(|_| anyhow!("HTTP error response was not valid UTF-8"))?
                    } else {
                        serde_json::json!({
                            "jsonrpc": "2.0",
                            "id": request_id,
                            "error": {
                                "code": crate::client::INTERNAL_ERROR,
                                "message": error_message,
                            }
                        })
                        .to_string()
                    };
                    self.response_tx
                        .send(response_message)
                        .await
                        .map_err(|_| anyhow!("Failed to send HTTP error response"))?;
                } else {
                    self.error_tx
                        .send(error_message)
                        .await
                        .map_err(|_| anyhow!("Failed to send HTTP notification error"))?;
                }
            }
        }

        Ok(())
    }

    async fn send_http_request(
        &self,
        request: http_client::Request<AsyncBody>,
        issued_tx: &mut Option<oneshot::Sender<()>>,
    ) -> Result<Response<AsyncBody>> {
        let mut send = self.http_client.send(request);
        futures::future::poll_fn(|cx| {
            let result = send.as_mut().poll(cx);
            if !matches!(&result, std::task::Poll::Ready(Err(_)))
                && let Some(issued_tx) = issued_tx.take()
                && issued_tx.send(()).is_err()
            {
                log::trace!("context server request-issued receiver was dropped");
            }
            result
        })
        .await
    }

    async fn receive_sse_response(
        &self,
        mut response: Response<AsyncBody>,
        request_id: Option<&Value>,
    ) -> Result<()> {
        let request_id = request_id.context("received an SSE response for a notification")?;
        let reader = futures::io::BufReader::new(response.body_mut());
        let mut lines = futures::AsyncBufReadExt::lines(reader);
        let mut data_buffer = Vec::new();
        let mut in_message = false;

        while let Some(line) = lines.next().await {
            let line = line.context("reading MCP SSE response")?;
            if line.is_empty() {
                if !data_buffer.is_empty()
                    && self
                        .forward_sse_message(&mut data_buffer, request_id)
                        .await?
                {
                    return Ok(());
                }
                in_message = false;
            } else if let Some(data) = line.strip_prefix("data:") {
                let data = data.trim();
                if !data.is_empty() && data != "ping" {
                    data_buffer.push(data.to_string());
                    in_message = true;
                }
            } else if line.starts_with("event:")
                || line.starts_with("id:")
                || line.starts_with("retry:")
            {
                continue;
            } else if in_message {
                data_buffer.push(line);
            }
        }

        if !data_buffer.is_empty()
            && self
                .forward_sse_message(&mut data_buffer, request_id)
                .await?
        {
            return Ok(());
        }

        anyhow::bail!(
            "SSE stream ended before the context server responded to request {request_id}"
        )
    }

    async fn forward_sse_message(
        &self,
        data_buffer: &mut Vec<String>,
        request_id: &Value,
    ) -> Result<bool> {
        let message = data_buffer.join("\n");
        data_buffer.clear();
        let completes_request =
            serde_json::from_str::<Value>(&message)
                .ok()
                .is_some_and(|message| {
                    message.get("id") == Some(request_id)
                        && message.get("method").is_none()
                        && (message.get("result").is_some() || message.get("error").is_some())
                });
        self.response_tx
            .send(message)
            .await
            .map_err(|_| anyhow!("Failed to send SSE response"))?;
        Ok(completes_request)
    }

    async fn forward_request_error(
        &self,
        request_id: Option<Value>,
        error: &anyhow::Error,
    ) -> Result<()> {
        let error_message = format!("{error:#}");
        if let Some(request_id) = request_id {
            self.response_tx
                .send(
                    serde_json::json!({
                        "jsonrpc": "2.0",
                        "id": request_id,
                        "error": {
                            "code": crate::client::INTERNAL_ERROR,
                            "message": error_message,
                        }
                    })
                    .to_string(),
                )
                .await
                .map_err(|_| anyhow!("Failed to send HTTP transport error response"))?;
        } else {
            self.error_tx
                .send(error_message)
                .await
                .map_err(|_| anyhow!("Failed to send HTTP notification error"))?;
        }
        Ok(())
    }
}

#[async_trait]
impl Transport for HttpTransport {
    async fn send(&self, message: String) -> Result<()> {
        self.send_message(message, None).await
    }

    fn supports_concurrent_sends(&self) -> bool {
        true
    }

    async fn send_cancellable(
        &self,
        message: String,
        cancel_rx: Option<oneshot::Receiver<()>>,
        issued_tx: Option<oneshot::Sender<()>>,
    ) -> Result<()> {
        let request_id = serde_json::from_str::<Value>(&message)
            .ok()
            .and_then(|message| message.get("id").cloned())
            .filter(|id| !id.is_null());
        let Some(cancel_rx) = cancel_rx else {
            return match self.send_message(message, issued_tx).await {
                Ok(()) => Ok(()),
                Err(error)
                    if error.downcast_ref::<TransportError>().is_some()
                        || self.session_expired.load(Ordering::SeqCst) =>
                {
                    Err(error)
                }
                Err(error) => self.forward_request_error(request_id, &error).await,
            };
        };
        let mut send = std::pin::pin!(self.send_message(message, issued_tx).fuse());
        let mut cancel_rx = std::pin::pin!(cancel_rx.fuse());
        let result = select! {
            result = send => result,
            _ = cancel_rx => Ok(()),
        };
        match result {
            Ok(()) => Ok(()),
            Err(error)
                if error.downcast_ref::<TransportError>().is_some()
                    || self.session_expired.load(Ordering::SeqCst) =>
            {
                Err(error)
            }
            Err(error) => self.forward_request_error(request_id, &error).await,
        }
    }

    fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(self.response_rx.clone())
    }

    fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(self.error_rx.clone())
    }

    fn set_protocol_version(&self, version: &str) {
        *self.protocol_version.lock() = Some(version.to_string());
    }

    fn auth_challenge(&self) -> Option<WwwAuthenticate> {
        self.auth_challenge.lock().clone()
    }

    fn shutdown_reason(&self) -> TransportShutdownReason {
        if let Some(www_authenticate) = self.auth_challenge() {
            TransportShutdownReason::AuthRequired(www_authenticate)
        } else if self.session_expired.load(Ordering::SeqCst) {
            TransportShutdownReason::SessionExpired
        } else {
            TransportShutdownReason::Other
        }
    }
}

impl Drop for HttpTransport {
    fn drop(&mut self) {
        // Try to cleanup session on drop
        let http_client = self.http_client.clone();
        let endpoint = self.endpoint.clone();
        let session_id = self.session_id.lock().clone();
        let protocol_version = self.protocol_version.lock().clone();
        let headers = self.headers.clone();
        let access_token = self.token_provider.as_ref().and_then(|p| p.access_token());

        if let Some(session_id) = session_id {
            self.executor
                .spawn(async move {
                    let mut request_builder = Request::builder()
                        .method(Method::DELETE)
                        .uri(&endpoint)
                        .header(HEADER_SESSION_ID, &session_id);

                    // Add static authentication headers.
                    for (key, value) in headers {
                        request_builder = request_builder.header(key.as_str(), value.as_str());
                    }

                    // Attach bearer token if available.
                    if let Some(token) = access_token {
                        request_builder =
                            request_builder.header("Authorization", format!("Bearer {}", token));
                    }

                    // Stamp the negotiated MCP protocol version on the DELETE
                    // too, matching what `build_request` does for POSTs.
                    if let Some(ref version) = protocol_version
                        && types::requires_protocol_version_header(version)
                    {
                        request_builder =
                            request_builder.header(HEADER_PROTOCOL_VERSION, version.as_str());
                    }

                    let request = request_builder.body(AsyncBody::empty());

                    if let Ok(request) = request {
                        let _ = http_client.send(request).await;
                    }
                })
                .detach();
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use async_trait::async_trait;
    use gpui::TestAppContext;
    use parking_lot::Mutex as SyncMutex;
    use std::{
        sync::atomic::{AtomicBool, AtomicUsize, Ordering},
        time::Duration,
    };

    struct PendingReader;

    impl futures::AsyncRead for PendingReader {
        fn poll_read(
            self: Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
            _buffer: &mut [u8],
        ) -> std::task::Poll<std::io::Result<usize>> {
            std::task::Poll::Pending
        }
    }

    struct TrackedPendingReader(Arc<AtomicUsize>);

    impl futures::AsyncRead for TrackedPendingReader {
        fn poll_read(
            self: Pin<&mut Self>,
            _cx: &mut std::task::Context<'_>,
            _buffer: &mut [u8],
        ) -> std::task::Poll<std::io::Result<usize>> {
            std::task::Poll::Pending
        }
    }

    impl Drop for TrackedPendingReader {
        fn drop(&mut self) {
            self.0.fetch_add(1, Ordering::SeqCst);
        }
    }

    /// A mock token provider that returns a configurable token and tracks
    /// refresh attempts.
    struct FakeTokenProvider {
        token: SyncMutex<Option<String>>,
        refreshed_token: SyncMutex<Option<String>>,
        refresh_succeeds: AtomicBool,
        refresh_count: AtomicUsize,
    }

    impl FakeTokenProvider {
        fn new(token: Option<&str>, refresh_succeeds: bool) -> Arc<Self> {
            Self::with_refreshed_token(token, None, refresh_succeeds)
        }

        fn with_refreshed_token(
            token: Option<&str>,
            refreshed_token: Option<&str>,
            refresh_succeeds: bool,
        ) -> Arc<Self> {
            Arc::new(Self {
                token: SyncMutex::new(token.map(String::from)),
                refreshed_token: SyncMutex::new(refreshed_token.map(String::from)),
                refresh_succeeds: AtomicBool::new(refresh_succeeds),
                refresh_count: AtomicUsize::new(0),
            })
        }

        fn set_token(&self, token: &str) {
            *self.token.lock() = Some(token.to_string());
        }

        fn refresh_count(&self) -> usize {
            self.refresh_count.load(Ordering::SeqCst)
        }
    }

    #[async_trait]
    impl OAuthTokenProvider for FakeTokenProvider {
        fn access_token(&self) -> Option<String> {
            self.token.lock().clone()
        }

        async fn try_refresh(&self) -> Result<bool> {
            self.refresh_count.fetch_add(1, Ordering::SeqCst);

            let refresh_succeeds = self.refresh_succeeds.load(Ordering::SeqCst);
            if refresh_succeeds {
                if let Some(token) = self.refreshed_token.lock().clone() {
                    *self.token.lock() = Some(token);
                }
            }

            Ok(refresh_succeeds)
        }
    }

    struct PendingRefreshTokenProvider {
        refresh_started: AtomicBool,
    }

    #[async_trait]
    impl OAuthTokenProvider for PendingRefreshTokenProvider {
        fn access_token(&self) -> Option<String> {
            None
        }

        async fn try_refresh(&self) -> Result<bool> {
            self.refresh_started.store(true, Ordering::SeqCst);
            futures::future::pending().await
        }
    }

    fn make_fake_http_client(
        handler: impl Fn(
            http_client::Request<AsyncBody>,
        ) -> std::pin::Pin<
            Box<dyn std::future::Future<Output = anyhow::Result<Response<AsyncBody>>> + Send>,
        > + Send
        + Sync
        + 'static,
    ) -> Arc<dyn HttpClient> {
        http_client::FakeHttpClient::create(handler) as Arc<dyn HttpClient>
    }

    fn json_response(status: u16, body: &str) -> anyhow::Result<Response<AsyncBody>> {
        Ok(Response::builder()
            .status(status)
            .header("Content-Type", "application/json")
            .body(AsyncBody::from(body.as_bytes().to_vec()))
            .unwrap())
    }

    #[gpui::test]
    async fn test_sse_data_field(cx: &mut TestAppContext) {
        for data_prefix in ["data:", "data: "] {
            let body = format!(
                "id:1\nevent:message\n{data_prefix}{{\"jsonrpc\":\"2.0\",\"id\":1,\"result\":{{}}}}\n\n"
            );
            let client = make_fake_http_client(move |_req| {
                let body = body.clone();
                Box::pin(async move {
                    Ok(Response::builder()
                        .status(200)
                        .header("Content-Type", "text/event-stream;charset=UTF-8")
                        .body(AsyncBody::from(body.into_bytes()))
                        .unwrap())
                })
            });

            let transport = HttpTransport::new(
                client,
                "http://mcp.example.com/mcp".to_string(),
                HashMap::default(),
                cx.background_executor.clone(),
            );

            transport
                .send(r#"{"jsonrpc":"2.0","id":1,"method":"initialize"}"#.to_string())
                .await
                .expect("send should succeed");

            let mut responses = transport.receive();
            let next_response = responses.next().fuse();
            let timeout = cx.background_executor.timer(Duration::from_secs(1)).fuse();
            futures::pin_mut!(next_response, timeout);

            let response = futures::select_biased! {
                response = next_response => response.expect("expected SSE response"),
                _ = timeout => panic!("timed out waiting for SSE response with {data_prefix:?}"),
            };

            assert_eq!(
                response, r#"{"jsonrpc":"2.0","id":1,"result":{}}"#,
                "unexpected SSE response for {data_prefix:?}",
            );
        }
    }

    #[gpui::test]
    async fn test_sse_continuation_ignores_event_fields(cx: &mut TestAppContext) {
        let client = make_fake_http_client(|_request| {
            Box::pin(async {
                Ok(Response::builder()
                    .status(200)
                    .header("Content-Type", "text/event-stream")
                    .body(AsyncBody::from(
                        b"data:ping\n\nid:1\ndata:{\"jsonrpc\":\"2.0\",\nevent:message\n\"id\":1,\"result\":{}}\n\n"
                            .to_vec(),
                    ))?)
            })
        });
        let transport = HttpTransport::new(
            client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
        );

        transport
            .send(r#"{"jsonrpc":"2.0","id":1,"method":"initialize"}"#.to_string())
            .await
            .expect("SSE response should complete the request");
        let response = transport
            .receive()
            .next()
            .await
            .expect("SSE response should be forwarded");
        let response: Value =
            serde_json::from_str(&response).expect("continuation should be valid JSON");
        assert_eq!(response["id"], 1);
        assert_eq!(response["result"], serde_json::json!({}));
    }

    #[gpui::test]
    async fn test_bearer_token_attached_to_requests(cx: &mut TestAppContext) {
        // Capture the Authorization header from the request.
        let captured_auth = Arc::new(SyncMutex::new(None::<String>));
        let captured_auth_clone = captured_auth.clone();

        let client = make_fake_http_client(move |req| {
            let auth = req
                .headers()
                .get("Authorization")
                .map(|v| v.to_str().unwrap().to_string());
            *captured_auth_clone.lock() = auth;
            Box::pin(async { json_response(200, r#"{"jsonrpc":"2.0","id":1,"result":{}}"#) })
        });

        let provider = FakeTokenProvider::new(Some("test-access-token"), false);
        let transport = HttpTransport::new_with_token_provider(
            client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
            Some(provider),
        );

        transport
            .send(r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#.to_string())
            .await
            .expect("send should succeed");

        assert_eq!(
            captured_auth.lock().as_deref(),
            Some("Bearer test-access-token"),
        );
    }

    #[gpui::test]
    async fn test_no_bearer_token_without_provider(cx: &mut TestAppContext) {
        let captured_auth = Arc::new(SyncMutex::new(None::<String>));
        let captured_auth_clone = captured_auth.clone();

        let client = make_fake_http_client(move |req| {
            let auth = req
                .headers()
                .get("Authorization")
                .map(|v| v.to_str().unwrap().to_string());
            *captured_auth_clone.lock() = auth;
            Box::pin(async { json_response(200, r#"{"jsonrpc":"2.0","id":1,"result":{}}"#) })
        });

        let transport = HttpTransport::new(
            client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
        );

        transport
            .send(r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#.to_string())
            .await
            .expect("send should succeed");

        assert!(captured_auth.lock().is_none());
    }

    #[gpui::test]
    async fn test_non_success_response_forwards_status_and_body(cx: &mut TestAppContext) {
        let client = make_fake_http_client(|_request| {
            Box::pin(async { json_response(429, "request quota exceeded") })
        });
        let transport = HttpTransport::new(
            client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
        );

        transport
            .send(r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#.to_string())
            .await
            .expect("non-success request response should be forwarded");
        let response = transport
            .receive()
            .next()
            .await
            .expect("HTTP error response should be forwarded");
        let response: Value =
            serde_json::from_str(&response).expect("forwarded response should be valid JSON");

        assert_eq!(
            response["error"]["message"],
            "HTTP 429 Too Many Requests: request quota exceeded"
        );
        assert_eq!(response["id"], 1);
    }

    #[gpui::test]
    async fn test_non_success_response_preserves_json_rpc_error(cx: &mut TestAppContext) {
        let error_response = serde_json::json!({
            "jsonrpc": "2.0",
            "id": "request-id",
            "error": {"code": -32001, "message": "server is busy"}
        });
        let response_body = error_response.to_string();
        let client = make_fake_http_client(move |_request| {
            let response_body = response_body.clone();
            Box::pin(async move { json_response(503, &response_body) })
        });
        let transport = HttpTransport::new(
            client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
        );

        transport
            .send(r#"{"jsonrpc":"2.0","id":"request-id","method":"tools/list"}"#.to_string())
            .await
            .expect("JSON-RPC error response should be forwarded");
        let response = transport
            .receive()
            .next()
            .await
            .expect("JSON-RPC error response should be available");

        assert_eq!(
            serde_json::from_str::<Value>(&response)
                .expect("forwarded response should be valid JSON"),
            error_response
        );
    }

    #[gpui::test]
    async fn test_http_error_body_is_truncated(cx: &mut TestAppContext) {
        let client = make_fake_http_client(|_request| {
            Box::pin(async {
                let body = "x".repeat(MAX_HTTP_ERROR_BODY_BYTES + 1024);
                json_response(500, &body)
            })
        });
        let transport = HttpTransport::new(
            client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
        );

        transport
            .send(r#"{"jsonrpc":"2.0","id":1,"method":"tools/list"}"#.to_string())
            .await
            .expect("HTTP error response should be forwarded");
        let response = transport
            .receive()
            .next()
            .await
            .expect("HTTP error response should be available");
        let response: Value =
            serde_json::from_str(&response).expect("forwarded response should be valid JSON");
        let message = response["error"]["message"]
            .as_str()
            .expect("forwarded error should have a message");

        assert!(message.ends_with("[response body truncated]"));
        assert!(message.len() < MAX_HTTP_ERROR_BODY_BYTES + 100);
    }

    #[gpui::test]
    async fn test_404_clears_expired_session(cx: &mut TestAppContext) {
        let request_count = Arc::new(AtomicUsize::new(0));
        let request_count_for_client = request_count.clone();
        let client = make_fake_http_client(move |_request| {
            let request_index = request_count_for_client.fetch_add(1, Ordering::SeqCst);
            Box::pin(async move {
                if request_index == 0 {
                    Ok(Response::builder()
                        .status(200)
                        .header("Content-Type", JSON_MIME_TYPE)
                        .header(HEADER_SESSION_ID, "expired-session")
                        .body(AsyncBody::from(
                            br#"{"jsonrpc":"2.0","id":1,"result":{}}"#.to_vec(),
                        ))
                        .expect("session response should build"))
                } else {
                    Ok(Response::builder()
                        .status(404)
                        .body(AsyncBody::from_reader(PendingReader))
                        .expect("expired session response should build"))
                }
            })
        });
        let transport = HttpTransport::new(
            client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
        );

        transport
            .send(r#"{"jsonrpc":"2.0","id":1,"method":"initialize"}"#.to_string())
            .await
            .expect("initial request should succeed");
        assert_eq!(
            transport.session_id.lock().as_deref(),
            Some("expired-session")
        );
        transport.set_protocol_version("2025-06-18");
        let error = transport
            .send(r#"{"jsonrpc":"2.0","id":2,"method":"tools/list"}"#.to_string())
            .await
            .expect_err("expired session should terminate the send");

        assert_eq!(error.to_string(), "MCP session expired: HTTP 404 Not Found");
        assert!(transport.session_id.lock().is_none());
        assert!(transport.protocol_version.lock().is_none());
        assert!(matches!(
            transport.shutdown_reason(),
            TransportShutdownReason::SessionExpired
        ));
    }

    #[gpui::test]
    async fn test_non_success_response_fails_promptly_and_keeps_client_running(
        cx: &mut TestAppContext,
    ) {
        let request_count = Arc::new(AtomicUsize::new(0));
        let request_count_for_client = request_count.clone();
        let http_client = make_fake_http_client(move |_request| {
            let request_index = request_count_for_client.fetch_add(1, Ordering::SeqCst);
            Box::pin(async move {
                if request_index == 0 {
                    json_response(503, "service is warming up")
                } else {
                    json_response(
                        200,
                        r#"{"jsonrpc":"2.0","id":1,"result":{"recovered":true}}"#,
                    )
                }
            })
        });
        let transport = Arc::new(HttpTransport::new(
            http_client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
        ));
        let client = Arc::new(
            crate::client::Client::new(
                crate::client::ContextServerId("test-server".into()),
                "test-server".into(),
                transport,
                Some(std::time::Duration::from_secs(60)),
                cx.to_async(),
            )
            .expect("client should be created"),
        );
        let requests = cx.spawn(move |_| async move {
            let first = client.request::<serde_json::Value>("tools/list", ()).await;
            let second = client.request::<serde_json::Value>("tools/list", ()).await;
            (first, second)
        });

        cx.executor().run_until_parked();
        let (first, second) = requests
            .now_or_never()
            .expect("both requests should resolve without advancing the request timeout");
        let error = first.expect_err("non-success response should fail the first request");

        assert_eq!(
            error.to_string(),
            "HTTP 503 Service Unavailable: service is warming up"
        );
        assert_eq!(
            second.expect("the second request should succeed"),
            serde_json::json!({"recovered": true})
        );
        assert_eq!(request_count.load(Ordering::SeqCst), 2);
    }

    #[gpui::test]
    async fn test_sse_eof_fails_client_request_promptly(cx: &mut TestAppContext) {
        let request_count = Arc::new(AtomicUsize::new(0));
        let request_count_for_client = request_count.clone();
        let http_client = make_fake_http_client(move |_request| {
            let request_index = request_count_for_client.fetch_add(1, Ordering::SeqCst);
            Box::pin(async move {
                if request_index == 0 {
                    Ok(Response::builder()
                        .status(200)
                        .header("Content-Type", EVENT_STREAM_MIME_TYPE)
                        .body(AsyncBody::empty())
                        .expect("SSE response should build"))
                } else {
                    json_response(
                        200,
                        r#"{"jsonrpc":"2.0","id":1,"result":{"recovered":true}}"#,
                    )
                }
            })
        });
        let transport = Arc::new(HttpTransport::new(
            http_client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
        ));
        let client = Arc::new(
            crate::client::Client::new(
                crate::client::ContextServerId("test-server".into()),
                "test-server".into(),
                transport,
                Some(std::time::Duration::from_secs(60)),
                cx.to_async(),
            )
            .expect("client should be created"),
        );
        let requests = cx.spawn(move |_| async move {
            let first = client.request::<serde_json::Value>("tools/list", ()).await;
            let second = client.request::<serde_json::Value>("tools/list", ()).await;
            (first, second)
        });

        cx.executor().run_until_parked();
        let (first, second) = requests
            .now_or_never()
            .expect("both requests should resolve without advancing the request timeout");
        let error = first.expect_err("SSE EOF should fail the client request");

        assert_eq!(
            error.to_string(),
            "SSE stream ended before the context server responded to request 0"
        );
        assert_eq!(
            second.expect("a request after SSE EOF should still succeed"),
            serde_json::json!({"recovered": true})
        );
        assert_eq!(request_count.load(Ordering::SeqCst), 2);
    }

    #[gpui::test]
    async fn test_sse_server_request_with_same_id_does_not_complete_response(
        cx: &mut TestAppContext,
    ) {
        let sse_body = concat!(
            "data: {\"jsonrpc\":\"2.0\",\"id\":7,\"method\":\"sampling/createMessage\",\"params\":{}}\n\n",
            "data: {\"jsonrpc\":\"2.0\",\"id\":7,\"result\":{\"complete\":true}}\n\n"
        );
        let http_client = make_fake_http_client(move |_request| {
            Box::pin(async move {
                Ok(Response::builder()
                    .status(200)
                    .header("Content-Type", EVENT_STREAM_MIME_TYPE)
                    .body(AsyncBody::from(sse_body.as_bytes().to_vec()))
                    .expect("SSE response should build"))
            })
        });
        let transport = HttpTransport::new(
            http_client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
        );

        transport
            .send(r#"{"jsonrpc":"2.0","id":7,"method":"tools/list"}"#.to_string())
            .await
            .expect("SSE response should be consumed through the matching response");
        let mut responses = transport.receive();
        let server_request = responses
            .next()
            .await
            .expect("server request should be forwarded");
        let response = responses
            .next()
            .now_or_never()
            .flatten()
            .expect("matching response should also be forwarded before the stream is dropped");

        assert_eq!(
            serde_json::from_str::<Value>(&server_request)
                .expect("server request should be valid JSON")["method"],
            "sampling/createMessage"
        );
        assert_eq!(
            serde_json::from_str::<Value>(&response).expect("response should be valid JSON")["result"]
                ["complete"],
            true
        );
    }

    #[gpui::test]
    async fn test_sse_timeout_aborts_body_and_unblocks_later_requests(cx: &mut TestAppContext) {
        let request_count = Arc::new(AtomicUsize::new(0));
        let request_count_for_client = request_count.clone();
        let cancellation_count = Arc::new(AtomicUsize::new(0));
        let cancellation_count_for_client = cancellation_count.clone();
        let http_client = make_fake_http_client(move |mut request| {
            let request_count_for_client = request_count_for_client.clone();
            let cancellation_count_for_client = cancellation_count_for_client.clone();
            Box::pin(async move {
                let mut body = String::new();
                futures::AsyncReadExt::read_to_string(request.body_mut(), &mut body).await?;
                if body.contains("notifications/cancelled") {
                    cancellation_count_for_client.fetch_add(1, Ordering::SeqCst);
                    return Ok(Response::builder()
                        .status(202)
                        .body(AsyncBody::empty())
                        .expect("cancellation response should build"));
                }
                let request_index = request_count_for_client.fetch_add(1, Ordering::SeqCst);
                match request_index {
                    0 => Ok(Response::builder()
                        .status(200)
                        .header("Content-Type", EVENT_STREAM_MIME_TYPE)
                        .body(AsyncBody::from_reader(PendingReader))
                        .expect("SSE response should build")),
                    _ => json_response(
                        200,
                        r#"{"jsonrpc":"2.0","id":1,"result":{"recovered":true}}"#,
                    ),
                }
            })
        });
        let transport = Arc::new(HttpTransport::new(
            http_client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
        ));
        let client = Arc::new(
            crate::client::Client::new(
                crate::client::ContextServerId("test-server".into()),
                "test-server".into(),
                transport,
                Some(std::time::Duration::from_secs(1)),
                cx.to_async(),
            )
            .expect("client should be created"),
        );
        let first_request = cx.spawn({
            let client = client.clone();
            move |_| async move { client.request::<serde_json::Value>("tools/list", ()).await }
        });

        cx.executor().run_until_parked();
        assert_eq!(request_count.load(Ordering::SeqCst), 1);
        cx.executor()
            .advance_clock(std::time::Duration::from_secs(2));
        let error = first_request
            .await
            .expect_err("the first HTTP request should time out");
        assert_eq!(error.to_string(), "Context server request timeout");
        cx.executor().run_until_parked();
        assert_eq!(request_count.load(Ordering::SeqCst), 1);
        assert_eq!(cancellation_count.load(Ordering::SeqCst), 1);

        let second_request = cx.spawn({
            let client = client.clone();
            move |_| async move { client.request::<serde_json::Value>("tools/list", ()).await }
        });
        cx.executor().run_until_parked();
        assert_eq!(
            second_request
                .await
                .expect("the later HTTP request should succeed"),
            serde_json::json!({"recovered": true})
        );
        assert_eq!(request_count.load(Ordering::SeqCst), 2);
    }

    #[gpui::test]
    async fn test_requests_wait_for_initialized_notification(cx: &mut TestAppContext) {
        let (initialized_sender, initialized_receiver) = oneshot::channel::<()>();
        let initialized_receiver = Arc::new(SyncMutex::new(Some(initialized_receiver)));
        let request_count = Arc::new(AtomicUsize::new(0));
        let http_client = make_fake_http_client({
            let request_count = request_count.clone();
            move |mut request| {
                let initialized_receiver = initialized_receiver.clone();
                let request_count = request_count.clone();
                Box::pin(async move {
                    let mut body = String::new();
                    futures::AsyncReadExt::read_to_string(request.body_mut(), &mut body).await?;
                    let message: Value = serde_json::from_str(&body)?;
                    if message["method"] == "notifications/initialized" {
                        let receiver = initialized_receiver
                            .lock()
                            .take()
                            .expect("initialized notification should only be sent once");
                        receiver.await?;
                        Ok(Response::builder().status(202).body(AsyncBody::empty())?)
                    } else {
                        request_count.fetch_add(1, Ordering::SeqCst);
                        json_response(200, r#"{"jsonrpc":"2.0","id":0,"result":{"tools":[]}}"#)
                    }
                })
            }
        });
        let transport = Arc::new(HttpTransport::new(
            http_client,
            "https://mcp.example.com/mcp".to_owned(),
            HashMap::default(),
            cx.background_executor.clone(),
        ));
        let client = Arc::new(
            crate::client::Client::new(
                crate::client::ContextServerId("test-server".into()),
                "test-server".into(),
                transport,
                Some(Duration::from_secs(60)),
                cx.to_async(),
            )
            .expect("client should be created"),
        );
        client
            .notify("notifications/initialized", ())
            .expect("initialized notification should enqueue");
        let request = cx.spawn({
            let client = client.clone();
            move |_| async move { client.request::<Value>("tools/list", ()).await }
        });
        cx.executor().run_until_parked();
        assert_eq!(request_count.load(Ordering::SeqCst), 0);
        initialized_sender
            .send(())
            .expect("notification should still be pending");
        cx.executor().run_until_parked();
        assert_eq!(request_count.load(Ordering::SeqCst), 1);
        assert!(
            request
                .now_or_never()
                .expect("request should complete")
                .is_ok()
        );
        client.stop();
    }

    #[gpui::test]
    async fn test_pending_sse_requests_do_not_block_requests_or_notifications(
        cx: &mut TestAppContext,
    ) {
        let seen = Arc::new(SyncMutex::new(Vec::<Value>::new()));
        let dropped_readers = Arc::new(AtomicUsize::new(0));
        let http_client = make_fake_http_client({
            let seen = seen.clone();
            let dropped_readers = dropped_readers.clone();
            move |mut request| {
                let seen = seen.clone();
                let dropped_readers = dropped_readers.clone();
                Box::pin(async move {
                    let mut body = String::new();
                    futures::AsyncReadExt::read_to_string(request.body_mut(), &mut body).await?;
                    let message: Value = serde_json::from_str(&body)?;
                    seen.lock().push(message.clone());
                    match message.get("id").and_then(Value::as_i64) {
                        Some(0 | 1) => Ok(Response::builder()
                            .status(200)
                            .header("Content-Type", EVENT_STREAM_MIME_TYPE)
                            .body(AsyncBody::from_reader(TrackedPendingReader(
                                dropped_readers,
                            )))?),
                        Some(2) => json_response(
                            200,
                            r#"{"jsonrpc":"2.0","id":2,"result":{"complete":true}}"#,
                        ),
                        _ => Ok(Response::builder().status(202).body(AsyncBody::empty())?),
                    }
                })
            }
        });
        let transport = Arc::new(HttpTransport::new(
            http_client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
        ));
        let client = Arc::new(
            crate::client::Client::new(
                crate::client::ContextServerId("test-server".into()),
                "test-server".into(),
                transport,
                Some(Duration::from_secs(60)),
                cx.to_async(),
            )
            .expect("client should be created"),
        );
        let (cancel_tx, cancel_rx) = oneshot::channel();
        let first = cx.spawn({
            let client = client.clone();
            move |_| async move {
                client
                    .request_with::<Value>("tools/call", (), Some(cancel_rx), None)
                    .await
            }
        });
        cx.executor().run_until_parked();
        let mut second = cx.spawn({
            let client = client.clone();
            move |_| async move { client.request::<Value>("tools/call", ()).await }
        });
        cx.executor().run_until_parked();
        assert_eq!(seen.lock().len(), 2);

        client
            .notify("notifications/roots/list_changed", ())
            .expect("notification should enqueue");
        let third = cx.spawn({
            let client = client.clone();
            move |_| async move { client.request::<Value>("tools/call", ()).await }
        });
        cx.executor().run_until_parked();
        assert_eq!(
            third
                .now_or_never()
                .expect("third request should complete while both SSE bodies are pending")
                .expect("third response should succeed"),
            serde_json::json!({"complete": true})
        );
        assert!(
            seen.lock()
                .iter()
                .any(|message| { message["method"] == "notifications/roots/list_changed" })
        );
        assert_eq!(dropped_readers.load(Ordering::SeqCst), 0);

        cancel_tx
            .send(())
            .expect("cancellation should be delivered");
        cx.executor().run_until_parked();
        assert_eq!(
            first
                .now_or_never()
                .expect("cancellation should complete promptly")
                .expect_err("first request should be canceled")
                .to_string(),
            crate::client::RequestCanceled.to_string()
        );
        assert_eq!(dropped_readers.load(Ordering::SeqCst), 1);
        assert!((&mut second).now_or_never().is_none());

        client.stop();
        cx.executor().run_until_parked();
        assert_eq!(
            second
                .now_or_never()
                .expect("stop should fail the outstanding request")
                .expect_err("second request should fail")
                .to_string(),
            "Context server stopped"
        );
        assert_eq!(dropped_readers.load(Ordering::SeqCst), 2);
    }

    #[gpui::test]
    async fn test_cancellation_during_token_refresh_does_not_notify_server(
        cx: &mut TestAppContext,
    ) {
        let request_count = Arc::new(AtomicUsize::new(0));
        let request_count_for_client = request_count.clone();
        let http_client = make_fake_http_client(move |_request| {
            request_count_for_client.fetch_add(1, Ordering::SeqCst);
            Box::pin(async { json_response(200, r#"{"jsonrpc":"2.0","id":0,"result":{}}"#) })
        });
        let token_provider = Arc::new(PendingRefreshTokenProvider {
            refresh_started: AtomicBool::new(false),
        });
        let transport = Arc::new(HttpTransport::new_with_token_provider(
            http_client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
            Some(token_provider.clone()),
        ));
        let client = Arc::new(
            crate::client::Client::new(
                crate::client::ContextServerId("test-server".into()),
                "test-server".into(),
                transport,
                Some(std::time::Duration::from_secs(60)),
                cx.to_async(),
            )
            .expect("client should be created"),
        );
        let (cancel_tx, cancel_rx) = oneshot::channel();
        let request = cx.spawn(move |_| async move {
            client
                .request_with::<serde_json::Value>("tools/list", (), Some(cancel_rx), None)
                .await
        });
        cx.executor().run_until_parked();
        assert!(token_provider.refresh_started.load(Ordering::SeqCst));

        cancel_tx
            .send(())
            .expect("request cancellation should be sent");
        cx.executor().run_until_parked();
        let error = request.await.expect_err("request should be canceled");

        assert_eq!(
            error.to_string(),
            crate::client::RequestCanceled.to_string()
        );
        assert_eq!(request_count.load(Ordering::SeqCst), 0);
    }

    #[gpui::test]
    async fn test_missing_token_triggers_refresh_before_first_request(cx: &mut TestAppContext) {
        let captured_auth = Arc::new(SyncMutex::new(None::<String>));
        let captured_auth_clone = captured_auth.clone();

        let client = make_fake_http_client(move |req| {
            let auth = req
                .headers()
                .get("Authorization")
                .map(|v| v.to_str().unwrap().to_string());
            *captured_auth_clone.lock() = auth;
            Box::pin(async { json_response(200, r#"{"jsonrpc":"2.0","id":1,"result":{}}"#) })
        });

        let provider = FakeTokenProvider::with_refreshed_token(None, Some("refreshed-token"), true);
        let transport = HttpTransport::new_with_token_provider(
            client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
            Some(provider.clone()),
        );

        transport
            .send(r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#.to_string())
            .await
            .expect("send should succeed after proactive refresh");

        assert_eq!(provider.refresh_count(), 1);
        assert_eq!(
            captured_auth.lock().as_deref(),
            Some("Bearer refreshed-token"),
        );
    }

    #[gpui::test]
    async fn test_invalid_token_still_triggers_refresh_and_retry(cx: &mut TestAppContext) {
        let request_count = Arc::new(AtomicUsize::new(0));
        let request_count_clone = request_count.clone();

        let client = make_fake_http_client(move |_req| {
            let count = request_count_clone.fetch_add(1, Ordering::SeqCst);
            Box::pin(async move {
                if count == 0 {
                    Ok(Response::builder()
                        .status(401)
                        .header(
                            "WWW-Authenticate",
                            r#"Bearer error="invalid_token", resource_metadata="https://mcp.example.com/.well-known/oauth-protected-resource""#,
                        )
                        .body(AsyncBody::from(b"Unauthorized".to_vec()))
                        .unwrap())
                } else {
                    json_response(200, r#"{"jsonrpc":"2.0","id":1,"result":{}}"#)
                }
            })
        });

        let provider = FakeTokenProvider::with_refreshed_token(
            Some("old-token"),
            Some("refreshed-token"),
            true,
        );
        let transport = HttpTransport::new_with_token_provider(
            client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
            Some(provider.clone()),
        );

        transport
            .send(r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#.to_string())
            .await
            .expect("send should succeed after refresh");

        assert_eq!(provider.refresh_count(), 1);
        assert_eq!(request_count.load(Ordering::SeqCst), 2);
    }

    #[gpui::test]
    async fn test_401_triggers_refresh_and_retry(cx: &mut TestAppContext) {
        let request_count = Arc::new(AtomicUsize::new(0));
        let request_count_clone = request_count.clone();

        let client = make_fake_http_client(move |_req| {
            let count = request_count_clone.fetch_add(1, Ordering::SeqCst);
            Box::pin(async move {
                if count == 0 {
                    // First request: 401.
                    Ok(Response::builder()
                        .status(401)
                        .header(
                            "WWW-Authenticate",
                            r#"Bearer resource_metadata="https://mcp.example.com/.well-known/oauth-protected-resource""#,
                        )
                        .body(AsyncBody::from(b"Unauthorized".to_vec()))
                        .unwrap())
                } else {
                    // Retry after refresh: 200.
                    json_response(200, r#"{"jsonrpc":"2.0","id":1,"result":{}}"#)
                }
            })
        });

        let provider = FakeTokenProvider::new(Some("old-token"), true);
        // Simulate the refresh updating the token.
        let provider_ref = provider.clone();
        let transport = HttpTransport::new_with_token_provider(
            client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
            Some(provider.clone()),
        );

        // Set the new token that will be used on retry.
        provider_ref.set_token("refreshed-token");

        transport
            .send(r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#.to_string())
            .await
            .expect("send should succeed after refresh");

        assert_eq!(provider_ref.refresh_count(), 1);
        assert_eq!(request_count.load(Ordering::SeqCst), 2);
    }

    #[gpui::test]
    async fn test_401_returns_auth_required_when_refresh_fails(cx: &mut TestAppContext) {
        let client = make_fake_http_client(|_req| {
            Box::pin(async {
                Ok(Response::builder()
                    .status(401)
                    .header(
                        "WWW-Authenticate",
                        r#"Bearer resource_metadata="https://mcp.example.com/.well-known/oauth-protected-resource", scope="read write""#,
                    )
                    .body(AsyncBody::from(b"Unauthorized".to_vec()))
                    .unwrap())
            })
        });

        // Refresh returns false — no new token available.
        let provider = FakeTokenProvider::new(Some("stale-token"), false);
        let transport = HttpTransport::new_with_token_provider(
            client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
            Some(provider.clone()),
        );

        let err = transport
            .send(r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#.to_string())
            .await
            .unwrap_err();

        let transport_err = err
            .downcast_ref::<TransportError>()
            .expect("error should be TransportError");
        match transport_err {
            TransportError::AuthRequired { www_authenticate } => {
                assert_eq!(
                    www_authenticate
                        .resource_metadata
                        .as_ref()
                        .map(|u| u.as_str()),
                    Some("https://mcp.example.com/.well-known/oauth-protected-resource"),
                );
                assert_eq!(
                    www_authenticate.scope,
                    Some(vec!["read".to_string(), "write".to_string()]),
                );
            }
        }
        assert_eq!(provider.refresh_count(), 1);
    }

    #[gpui::test]
    async fn test_401_returns_auth_required_without_provider(cx: &mut TestAppContext) {
        let client = make_fake_http_client(|_req| {
            Box::pin(async {
                Ok(Response::builder()
                    .status(401)
                    .header("WWW-Authenticate", "Bearer")
                    .body(AsyncBody::from(b"Unauthorized".to_vec()))
                    .unwrap())
            })
        });

        // No token provider at all.
        let transport = HttpTransport::new(
            client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
        );

        let err = transport
            .send(r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#.to_string())
            .await
            .unwrap_err();

        let transport_err = err
            .downcast_ref::<TransportError>()
            .expect("error should be TransportError");
        match transport_err {
            TransportError::AuthRequired { www_authenticate } => {
                assert!(www_authenticate.resource_metadata.is_none());
                assert!(www_authenticate.scope.is_none());
            }
        }
    }

    #[gpui::test]
    async fn test_401_after_successful_refresh_still_returns_auth_required(
        cx: &mut TestAppContext,
    ) {
        // Both requests return 401 — the server rejects the refreshed token too.
        let client = make_fake_http_client(|_req| {
            Box::pin(async {
                Ok(Response::builder()
                    .status(401)
                    .header("WWW-Authenticate", "Bearer")
                    .body(AsyncBody::from(b"Unauthorized".to_vec()))
                    .unwrap())
            })
        });

        let provider = FakeTokenProvider::new(Some("token"), true);
        let transport = HttpTransport::new_with_token_provider(
            client,
            "http://mcp.example.com/mcp".to_string(),
            HashMap::default(),
            cx.background_executor.clone(),
            Some(provider.clone()),
        );

        let err = transport
            .send(r#"{"jsonrpc":"2.0","id":1,"method":"ping"}"#.to_string())
            .await
            .unwrap_err();

        err.downcast_ref::<TransportError>()
            .expect("error should be TransportError");
        // Refresh was attempted exactly once.
        assert_eq!(provider.refresh_count(), 1);
    }
}
