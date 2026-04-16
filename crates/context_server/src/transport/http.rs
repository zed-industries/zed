use anyhow::{Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::{Stream, StreamExt};
use gpui::BackgroundExecutor;
use http_client::{AsyncBody, HttpClient, Request, Response, http::Method};
use parking_lot::Mutex as SyncMutex;
use smol::channel;
use std::{pin::Pin, sync::Arc};

use crate::oauth::{self, OAuthTokenProvider, WwwAuthenticate};
use crate::transport::Transport;

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
const EVENT_STREAM_MIME_TYPE: &str = "text/event-stream";
const JSON_MIME_TYPE: &str = "application/json";

/// HTTP Transport with session management and SSE support
pub struct HttpTransport {
    http_client: Arc<dyn HttpClient>,
    endpoint: String,
    session_id: Arc<SyncMutex<Option<String>>>,
    executor: BackgroundExecutor,
    response_tx: channel::Sender<String>,
    response_rx: channel::Receiver<String>,
    error_tx: channel::Sender<String>,
    error_rx: channel::Receiver<String>,
    /// Static headers to include in every request (e.g. from server config).
    headers: HashMap<String, String>,
    /// When set, the transport attaches `Authorization: Bearer` headers and
    /// handles 401 responses with token refresh + retry.
    token_provider: Option<Arc<dyn OAuthTokenProvider>>,
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
        let (response_tx, response_rx) = channel::unbounded();
        let (error_tx, error_rx) = channel::unbounded();

        Self {
            http_client,
            executor,
            endpoint,
            session_id: Arc::new(SyncMutex::new(None)),
            response_tx,
            response_rx,
            error_tx,
            error_rx,
            headers,
            token_provider,
        }
    }

    /// Build a POST request for the given message body, attaching all standard
    /// headers (content-type, accept, session ID, static headers, and bearer
    /// token if available).
    fn build_request(&self, message: &[u8]) -> Result<http_client::Request<AsyncBody>> {
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
        if let Some(ref session_id) = *self.session_id.lock() {
            request_builder = request_builder.header(HEADER_SESSION_ID, session_id.as_str());
        }

        Ok(request_builder.body(AsyncBody::from(message.to_vec()))?)
    }

    /// Send a message and handle the response based on content type.
    async fn send_message(&self, message: String) -> Result<()> {
        let is_notification =
            !message.contains("\"id\":") || message.contains("notifications/initialized");

        // If we currently have no access token, try refreshing before sending
        // the request so restored but expired sessions do not need an initial
        // 401 round-trip before they can recover.
        if let Some(ref provider) = self.token_provider {
            if provider.access_token().is_none() {
                provider.try_refresh().await.unwrap_or(false);
            }
        }

        let request = self.build_request(message.as_bytes())?;
        let mut response = self.http_client.send(request).await?;

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
                    let retry_request = self.build_request(message.as_bytes())?;
                    response = self.http_client.send(retry_request).await?;

                    // If still 401 after refresh, give up.
                    if response.status().as_u16() == 401 {
                        return Err(TransportError::AuthRequired { www_authenticate }.into());
                    }
                } else {
                    return Err(TransportError::AuthRequired { www_authenticate }.into());
                }
            } else {
                return Err(TransportError::AuthRequired { www_authenticate }.into());
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
                        // SSE stream - set up streaming
                        self.setup_sse_stream(response).await?;
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
                let mut error_body = String::new();
                futures::AsyncReadExt::read_to_string(response.body_mut(), &mut error_body).await?;

                self.error_tx
                    .send(format!("HTTP {}: {}", response.status(), error_body))
                    .await
                    .map_err(|_| anyhow!("Failed to send error"))?;
            }
        }

        Ok(())
    }

    /// Set up SSE streaming from the response
    async fn setup_sse_stream(&self, mut response: Response<AsyncBody>) -> Result<()> {
        let response_tx = self.response_tx.clone();
        let error_tx = self.error_tx.clone();

        // Spawn a task to handle the SSE stream
        smol::spawn(async move {
            let reader = futures::io::BufReader::new(response.body_mut());
            let mut lines = futures::AsyncBufReadExt::lines(reader);

            let mut data_buffer = Vec::new();
            let mut in_message = false;

            while let Some(line_result) = lines.next().await {
                match line_result {
                    Ok(line) => {
                        if line.is_empty() {
                            // Empty line signals end of event
                            if !data_buffer.is_empty() {
                                let message = data_buffer.join("\n");

                                // Filter out ping messages and empty data
                                if !message.trim().is_empty() && message != "ping" {
                                    if let Err(e) = response_tx.send(message).await {
                                        log::error!("Failed to send SSE message: {}", e);
                                        break;
                                    }
                                }
                                data_buffer.clear();
                            }
                            in_message = false;
                        } else if let Some(data) = line.strip_prefix("data: ") {
                            // Handle data lines
                            let data = data.trim();
                            if !data.is_empty() {
                                // Check if this is a ping message
                                if data == "ping" {
                                    log::trace!("Received SSE ping");
                                    continue;
                                }
                                data_buffer.push(data.to_string());
                                in_message = true;
                            }
                        } else if line.starts_with("event:")
                            || line.starts_with("id:")
                            || line.starts_with("retry:")
                        {
                            // Ignore other SSE fields
                            continue;
                        } else if in_message {
                            // Continuation of data
                            data_buffer.push(line);
                        }
                    }
                    Err(e) => {
                        let _ = error_tx.send(format!("SSE stream error: {}", e)).await;
                        break;
                    }
                }
            }
        })
        .detach();

        Ok(())
    }
}

#[async_trait]
impl Transport for HttpTransport {
    async fn send(&self, message: String) -> Result<()> {
        self.send_message(message).await
    }

    fn receive(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(self.response_rx.clone())
    }

    fn receive_err(&self) -> Pin<Box<dyn Stream<Item = String> + Send>> {
        Box::pin(self.error_rx.clone())
    }
}

impl Drop for HttpTransport {
    fn drop(&mut self) {
        // Try to cleanup session on drop
        let http_client = self.http_client.clone();
        let endpoint = self.endpoint.clone();
        let session_id = self.session_id.lock().clone();
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
    use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};

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
