use anyhow::{Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::{Stream, StreamExt};
use gpui::BackgroundExecutor;
use http_client::{
    AsyncBody, HttpClient, Request, Response,
    http::{self, Method},
};
use parking_lot::Mutex as SyncMutex;
use smol::channel;
use std::{fmt, pin::Pin, sync::Arc};
use url::Url;

use crate::oauth::OAuthManager;
use crate::transport::Transport;

// Constants from MCP spec
const HEADER_SESSION_ID: &str = "Mcp-Session-Id";
const EVENT_STREAM_MIME_TYPE: &str = "text/event-stream";
const JSON_MIME_TYPE: &str = "application/json";

#[derive(Debug, Clone)]
pub enum HttpTransportError {
    AuthenticationRequired {
        server_name: String,
    },
    AuthenticationExpiredManual {
        server_name: String,
    },
    UnauthorizedMissingWwwAuthenticate {
        server_name: String,
    },
    AuthenticationFailed {
        server_name: String,
        status: u16,
        error_body: String,
    },
}

impl fmt::Display for HttpTransportError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AuthenticationRequired { server_name } => write!(
                f,
                "Authentication required for '{server_name}'. Please click the Authenticate button in Agent Settings."
            ),
            Self::AuthenticationExpiredManual { server_name } => write!(
                f,
                "Authentication expired for '{server_name}'. Please reauthenticate manually from Agent Settings."
            ),
            Self::UnauthorizedMissingWwwAuthenticate { server_name } => write!(
                f,
                "Received 401 for '{server_name}' but no WWW-Authenticate header - cannot retry"
            ),
            Self::AuthenticationFailed {
                server_name,
                status,
                error_body,
            } => write!(
                f,
                "Authentication failed for '{server_name}': {status} - {error_body}"
            ),
        }
    }
}

impl std::error::Error for HttpTransportError {}

impl HttpTransportError {
    /// Returns true if this error indicates that authentication is required
    /// (either initial auth or re-authentication after expiry)
    pub fn is_auth_required(&self) -> bool {
        matches!(
            self,
            Self::AuthenticationRequired { .. } | Self::AuthenticationExpiredManual { .. }
        )
    }

    /// Check if an anyhow::Error is an auth-required HttpTransportError
    pub fn is_auth_required_error(err: &anyhow::Error) -> bool {
        err.downcast_ref::<Self>()
            .is_some_and(|e| e.is_auth_required())
    }
}

/// HTTP Transport with session management and SSE support
pub struct HttpTransport {
    server_name: String,
    http_client: Arc<dyn HttpClient>,
    endpoint: String,
    session_id: Arc<SyncMutex<Option<String>>>,
    executor: BackgroundExecutor,
    response_tx: channel::Sender<String>,
    response_rx: channel::Receiver<String>,
    error_tx: channel::Sender<String>,
    error_rx: channel::Receiver<String>,
    // Authentication headers to include in requests
    headers: HashMap<String, String>,
    oauth: Option<OAuthManager>,
    /// Track if we've successfully completed initial authentication
    initial_auth_done: Arc<SyncMutex<bool>>,
    /// Whether automatic reauthentication is allowed (opening browser without user action)
    allow_auto_reauthentication: bool,
}

impl HttpTransport {
    /// Manually trigger OAuth authentication. Returns Ok(true) if auth was successful.
    pub async fn authenticate(&self) -> Result<bool> {
        let Some(manager) = &self.oauth else {
            log::debug!(
                "No OAuth manager for '{}', authentication not needed",
                self.server_name
            );
            return Ok(true);
        };

        manager.authenticate().await?;

        // Clear the old session ID since we have a new OAuth token
        // The old session is invalidated when we re-authenticate
        let old_session = self.session_id.lock().take();
        if old_session.is_some() {
            log::info!(
                "Cleared old session ID for '{}' after re-authentication",
                self.server_name
            );
        }

        // Mark initial auth as done after successful authentication
        *self.initial_auth_done.lock() = true;
        log::info!(
            "Manual authentication completed successfully for '{}'",
            self.server_name
        );
        Ok(true)
    }

    /// Check if this transport needs authentication
    pub fn needs_authentication(&self) -> bool {
        self.oauth
            .as_ref()
            .is_some_and(|m| m.needs_authentication())
    }

    /// Check if this transport is authenticated
    pub fn is_authenticated(&self) -> bool {
        self.oauth.as_ref().is_some_and(|m| m.is_authenticated())
    }

    /// Logout - clear tokens and reset auth state
    pub fn logout(&self) {
        if let Some(manager) = &self.oauth {
            manager.logout();
            *self.initial_auth_done.lock() = false;
            log::info!("Logged out from '{}'", self.server_name);
        }
    }

    pub fn new(
        server_name: String,
        http_client: Arc<dyn HttpClient>,
        endpoint: Url,
        headers: HashMap<String, String>,
        allow_auto_reauthentication: bool,
        executor: BackgroundExecutor,
    ) -> Self {
        let (response_tx, response_rx) = channel::unbounded();
        let (error_tx, error_rx) = channel::unbounded();
        let has_manual_auth_header = headers
            .keys()
            .any(|key| key.eq_ignore_ascii_case(http::header::AUTHORIZATION.as_str()));

        let oauth = (!has_manual_auth_header).then(|| {
            OAuthManager::new(
                server_name.clone(),
                endpoint.clone(),
                headers.clone(),
                http_client.clone(),
            )
        });

        Self {
            server_name,
            http_client,
            executor,
            endpoint: endpoint.to_string(),
            session_id: Arc::new(SyncMutex::new(None)),
            response_tx,
            response_rx,
            error_tx,
            error_rx,
            headers,
            oauth,
            initial_auth_done: Arc::new(SyncMutex::new(false)),
            allow_auto_reauthentication,
        }
    }

    /// Send a message and handle the response based on content type
    async fn send_message(&self, message: String) -> Result<()> {
        let is_notification =
            !message.contains("\"id\":") || message.contains("notifications/initialized");

        // Extract request ID for logging
        let request_id = if let Some(start) = message.find("\"id\":") {
            message[start..]
                .split(',')
                .next()
                .and_then(|s| s.split(':').nth(1))
                .and_then(|s| s.trim().parse::<i32>().ok())
        } else {
            None
        };

        log::trace!(
            "Sending message to '{}', request_id={:?}, is_notification={}",
            self.server_name,
            request_id,
            is_notification
        );

        let mut retry_auth = false;
        let request_bytes = message.clone().into_bytes();

        loop {
            let auth_header = self.oauth_header().await?;

            let mut request_builder = Request::builder()
                .method(Method::POST)
                .uri(&self.endpoint)
                .header("Content-Type", JSON_MIME_TYPE)
                .header(
                    "Accept",
                    format!("{}, {}", JSON_MIME_TYPE, EVENT_STREAM_MIME_TYPE),
                );

            if let Some(auth) = auth_header.as_ref() {
                request_builder =
                    request_builder.header(http::header::AUTHORIZATION, auth.as_str());
            }

            for (key, value) in &self.headers {
                request_builder = request_builder.header(key.as_str(), value.as_str());
            }

            // Add session ID if we have one (except for initialize)
            if let Some(ref session_id) = *self.session_id.lock() {
                request_builder = request_builder.header(HEADER_SESSION_ID, session_id.as_str());
            }

            let request = request_builder.body(AsyncBody::from(request_bytes.clone()))?;

            let mut response = self.http_client.send(request).await?;
            log::debug!(
                "Received response from '{}': status={}",
                self.server_name,
                response.status()
            );

            match response.status() {
                status if status.is_success() => {
                    // Mark that we've successfully completed at least one request
                    // This is used to differentiate initial auth from re-auth scenarios
                    if self.oauth.is_some() {
                        let was_done = *self.initial_auth_done.lock();
                        *self.initial_auth_done.lock() = true;
                        if !was_done {
                            log::info!(
                                "Initial authentication completed successfully for '{}' - connection established",
                                self.server_name
                            );
                        }
                    }

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
                status if status.as_u16() == 401 && !retry_auth => {
                    // Token expired - need to re-authenticate
                    log::warn!(
                        "Received 401 for '{}' - performing OAuth re-authentication",
                        self.server_name
                    );

                    if self.handle_unauthorized(&response).await? {
                        log::info!(
                            "OAuth completed successfully for '{}', retrying request...",
                            self.server_name
                        );

                        // Set flag to prevent infinite retry loop
                        retry_auth = true;

                        // Continue loop to retry the request with new token
                        continue;
                    } else {
                        let mut error_body = String::new();
                        futures::AsyncReadExt::read_to_string(response.body_mut(), &mut error_body)
                            .await?;
                        return Err(HttpTransportError::AuthenticationFailed {
                            server_name: self.server_name.clone(),
                            status: 401,
                            error_body,
                        }
                        .into());
                    }
                }
                status if status.as_u16() == 422 && retry_auth => {
                    // Server requires re-initialization after token change
                    let mut error_body = String::new();
                    futures::AsyncReadExt::read_to_string(response.body_mut(), &mut error_body)
                        .await?;

                    log::warn!(
                        "Server requires re-initialization after OAuth refresh for '{}': {}",
                        self.server_name,
                        error_body
                    );

                    // Perform re-initialization internally
                    log::info!(
                        "Performing internal re-initialization for '{}'...",
                        self.server_name
                    );
                    self.perform_reinitialize().await?;

                    // Now retry the original request
                    log::info!(
                        "Re-initialization complete, retrying original request for '{}'...",
                        self.server_name
                    );
                    continue;
                }
                _ => {
                    let status = response.status();
                    let mut error_body = String::new();
                    futures::AsyncReadExt::read_to_string(response.body_mut(), &mut error_body)
                        .await?;

                    log::error!(
                        "Request failed for '{}': {} - {}",
                        self.server_name,
                        status,
                        error_body
                    );

                    // Return error immediately so it propagates to the request handler
                    // instead of timing out waiting for a response that will never come
                    return Err(anyhow!(
                        "HTTP request failed for '{}': {} - {}",
                        self.server_name,
                        status,
                        error_body
                    ));
                }
            }

            break;
        }

        Ok(())
    }

    async fn handle_unauthorized(&self, response: &Response<AsyncBody>) -> Result<bool> {
        let Some(manager) = &self.oauth else {
            log::debug!("No OAuth manager for '{}'", self.server_name);
            return Ok(false);
        };

        let header = response
            .headers()
            .get(http::header::WWW_AUTHENTICATE)
            .and_then(|h| h.to_str().ok())
            .unwrap_or_default();

        if header.is_empty() {
            return Err(HttpTransportError::UnauthorizedMissingWwwAuthenticate {
                server_name: self.server_name.clone(),
            }
            .into());
        }

        log::info!(
            "Found WWW-Authenticate header for '{}': {}",
            self.server_name,
            header
        );

        let is_initial_auth = !*self.initial_auth_done.lock();
        if !self.allow_auto_reauthentication {
            if is_initial_auth {
                log::info!(
                    "Authentication required for '{}'. User must click Authenticate button.",
                    self.server_name
                );
                return Err(HttpTransportError::AuthenticationRequired {
                    server_name: self.server_name.clone(),
                }
                .into());
            }

            log::warn!(
                "Reauthentication required for '{}' but auto-reauthentication is disabled. User must manually trigger authentication.",
                self.server_name
            );
            return Err(HttpTransportError::AuthenticationExpiredManual {
                server_name: self.server_name.clone(),
            }
            .into());
        }

        // Auto-reauthentication is enabled. First attempt a silent refresh if possible, even on the
        // first request after restart (initial_auth_done=false). This avoids forcing manual auth
        // when a persisted access token is expired/invalid but a refresh token exists.
        if manager.can_refresh() {
            match manager.refresh_access_token().await {
                Ok(true) => {
                    log::info!(
                        "Refreshed OAuth token for '{}' after 401, retrying request...",
                        self.server_name
                    );
                    self.on_new_oauth_token().await;
                    return Ok(true);
                }
                Ok(false) => {}
                Err(err) => {
                    log::warn!(
                        "OAuth token refresh failed for '{}' after 401: {}",
                        self.server_name,
                        err
                    );
                }
            }
        }

        // Never automatically open a browser for initial authentication.
        // User must explicitly trigger authentication from the UI.
        if is_initial_auth {
            log::info!(
                "Authentication required for '{}'. User must click Authenticate button.",
                self.server_name
            );
            return Err(HttpTransportError::AuthenticationRequired {
                server_name: self.server_name.clone(),
            }
            .into());
        }

        log::info!(
            "Auto-reauthentication enabled for '{}', proceeding with OAuth...",
            self.server_name
        );
        log::trace!("OAuth manager address: {:p}", manager as *const _);

        // Perform OAuth login (only for auto-reauthentication)
        let result = manager.handle_www_authenticate(header).await;

        if result.is_ok() {
            self.on_new_oauth_token().await;
        }

        result.map(|_| true)
    }

    async fn on_new_oauth_token(&self) {
        // Clear the old session ID since we have a new OAuth token
        // The old session is invalidated when we re-authenticate
        let old_session = self.session_id.lock().take();
        if old_session.is_some() {
            log::info!(
                "Cleared old session ID for '{}' after re-authentication",
                self.server_name
            );
        }

        // Proactively re-initialize the session after OAuth token changes.
        // This avoids getting 422 "expect initialize request" on the retry.
        log::info!(
            "Proactively re-initializing session for '{}' after OAuth token change...",
            self.server_name
        );
        if let Err(err) = self.perform_reinitialize().await {
            log::warn!(
                "Proactive re-initialization failed for '{}': {} - will retry on 422",
                self.server_name,
                err
            );
        } else {
            log::info!(
                "Proactive re-initialization succeeded for '{}'",
                self.server_name
            );
        }
    }

    /// Builds an HTTP request with common headers (auth, custom headers, session ID).
    async fn build_request(
        &self,
        body: &str,
        include_session_id: bool,
    ) -> Result<Request<AsyncBody>> {
        let auth_header = self.oauth_header().await?;

        let mut request_builder = Request::builder()
            .method(Method::POST)
            .uri(&self.endpoint)
            .header("Content-Type", JSON_MIME_TYPE)
            .header(
                "Accept",
                format!("{}, {}", JSON_MIME_TYPE, EVENT_STREAM_MIME_TYPE),
            );

        if let Some(auth) = auth_header.as_ref() {
            request_builder = request_builder.header(http::header::AUTHORIZATION, auth.as_str());
        }

        for (key, value) in &self.headers {
            request_builder = request_builder.header(key.as_str(), value.as_str());
        }

        if include_session_id {
            if let Some(ref session_id) = *self.session_id.lock() {
                request_builder = request_builder.header(HEADER_SESSION_ID, session_id.as_str());
            }
        }

        Ok(request_builder.body(AsyncBody::from(body.to_string().into_bytes()))?)
    }

    /// Performs MCP re-initialization after OAuth re-authentication.
    /// This sends initialize + notifications/initialized directly without going through
    /// the normal request/response flow, since we need to establish a new session.
    async fn perform_reinitialize(&self) -> Result<()> {
        // Build and send initialize request
        let init_request = serde_json::json!({
            "jsonrpc": "2.0",
            "id": -1,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": {
                    "name": "Zed",
                    "version": env!("CARGO_PKG_VERSION")
                }
            }
        });

        let request = self
            .build_request(&serde_json::to_string(&init_request)?, false)
            .await?;
        log::debug!(
            "Sending re-initialize request for '{}'...",
            self.server_name
        );
        let mut response = self.http_client.send(request).await?;

        if !response.status().is_success() {
            let mut error_body = String::new();
            futures::AsyncReadExt::read_to_string(response.body_mut(), &mut error_body).await?;
            return Err(anyhow!(
                "Re-initialization failed for '{}': {} - {}",
                self.server_name,
                response.status(),
                error_body
            ));
        }

        // Extract new session ID from response
        if let Some(session_id) = response
            .headers()
            .get(HEADER_SESSION_ID)
            .and_then(|v| v.to_str().ok())
        {
            *self.session_id.lock() = Some(session_id.to_string());
            log::info!("New session ID for '{}': {}", self.server_name, session_id);
        }

        // Read and discard initialize response
        let mut response_body = String::new();
        futures::AsyncReadExt::read_to_string(response.body_mut(), &mut response_body).await?;
        log::debug!(
            "Re-initialize response for '{}': {}",
            self.server_name,
            response_body
        );

        // Send notifications/initialized
        let initialized_notification = serde_json::json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        });

        let request = self
            .build_request(&serde_json::to_string(&initialized_notification)?, true)
            .await?;
        log::debug!(
            "Sending initialized notification for '{}'...",
            self.server_name
        );
        let response = self.http_client.send(request).await?;

        if response.status().as_u16() != 202 && !response.status().is_success() {
            log::warn!(
                "Initialized notification got unexpected status for '{}': {}",
                self.server_name,
                response.status()
            );
        }

        log::info!(
            "Re-initialization completed successfully for '{}'",
            self.server_name
        );
        Ok(())
    }

    async fn oauth_header(&self) -> Result<Option<String>> {
        let Some(manager) = &self.oauth else {
            return Ok(None);
        };

        // Try to get existing valid token (includes refresh if needed)
        match manager.access_token().await {
            Ok(Some(token)) => {
                log::trace!("Using existing token for '{}'", self.server_name);
                return Ok(Some(format!("Bearer {}", token)));
            }
            Ok(None) => {
                log::debug!("No token available for '{}'", self.server_name);
            }
            Err(e) => {
                log::error!("Error getting token for '{}': {}", self.server_name, e);
                return Err(e);
            }
        }

        // No valid token available - don't attempt proactive login.
        // Let the request go through without auth; if the server requires OAuth,
        // it will respond with 401 + WWW-Authenticate header which triggers
        // handle_unauthorized() to perform the OAuth flow.
        log::debug!(
            "No valid token for '{}', proceeding without auth header",
            self.server_name
        );
        Ok(None)
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

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

impl Drop for HttpTransport {
    fn drop(&mut self) {
        // Try to cleanup session on drop
        let http_client = self.http_client.clone();
        let endpoint = self.endpoint.clone();
        let session_id = self.session_id.lock().clone();
        let headers = self.headers.clone();

        if let Some(session_id) = session_id {
            self.executor
                .spawn(async move {
                    let mut request_builder = Request::builder()
                        .method(Method::DELETE)
                        .uri(&endpoint)
                        .header(HEADER_SESSION_ID, &session_id);

                    // Add authentication headers if present
                    for (key, value) in headers {
                        request_builder = request_builder.header(key.as_str(), value.as_str());
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
    use gpui::TestAppContext;
    use http_client::FakeHttpClient;
    use serde_json::json;
    use std::collections::HashMap as StdHashMap;
    use std::sync::atomic::{AtomicUsize, Ordering};

    fn create_json_response(body: serde_json::Value) -> Response<AsyncBody> {
        Response::builder()
            .status(200)
            .header("Content-Type", "application/json")
            .body(AsyncBody::from(serde_json::to_string(&body).unwrap()))
            .unwrap()
    }

    fn create_mcp_response(id: i64, result: serde_json::Value) -> Response<AsyncBody> {
        create_json_response(json!({
            "jsonrpc": "2.0",
            "id": id,
            "result": result
        }))
    }

    #[gpui::test]
    async fn test_http_transport_no_auth_required(cx: &mut TestAppContext) {
        let request_count = Arc::new(AtomicUsize::new(0));
        let request_count_clone = request_count.clone();

        let client = FakeHttpClient::create(move |_req| {
            let count = request_count_clone.fetch_add(1, Ordering::SeqCst);
            async move {
                Ok(create_mcp_response(
                    count as i64,
                    json!({
                        "protocolVersion": "2024-11-05",
                        "capabilities": {},
                        "serverInfo": {
                            "name": "test-server",
                            "version": "1.0.0"
                        }
                    }),
                ))
            }
        });

        let executor = cx.executor();
        let transport = HttpTransport::new(
            "test-server".to_string(),
            client,
            Url::parse("http://example.com/mcp").unwrap(),
            HashMap::default(),
            false,
            executor,
        );

        let message = json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "initialize",
            "params": {
                "protocolVersion": "2024-11-05",
                "capabilities": {},
                "clientInfo": { "name": "test", "version": "1.0" }
            }
        });

        transport
            .send(serde_json::to_string(&message).unwrap())
            .await
            .expect("send should succeed");

        assert_eq!(request_count.load(Ordering::SeqCst), 1);
    }

    #[gpui::test]
    async fn test_http_transport_with_manual_auth_header(cx: &mut TestAppContext) {
        let auth_header_seen = Arc::new(SyncMutex::new(None::<String>));

        let client = {
            let auth_header_seen = auth_header_seen.clone();
            FakeHttpClient::create(move |req| {
                let auth = req
                    .headers()
                    .get(http::header::AUTHORIZATION)
                    .and_then(|v| v.to_str().ok())
                    .map(String::from);
                *auth_header_seen.lock() = auth;

                async move {
                    Ok(create_mcp_response(
                        0,
                        json!({
                            "protocolVersion": "2024-11-05",
                            "capabilities": {},
                            "serverInfo": { "name": "test", "version": "1.0" }
                        }),
                    ))
                }
            })
        };

        let executor = cx.executor();
        let mut headers = HashMap::default();
        headers.insert("Authorization".to_string(), "Bearer my-token".to_string());

        let transport = HttpTransport::new(
            "test-server".to_string(),
            client,
            Url::parse("http://example.com/mcp").unwrap(),
            headers,
            false,
            executor,
        );

        let message = json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "initialize",
            "params": {}
        });

        transport
            .send(serde_json::to_string(&message).unwrap())
            .await
            .expect("send should succeed");

        let seen_auth = auth_header_seen.lock().clone();
        assert_eq!(seen_auth, Some("Bearer my-token".to_string()));
    }

    #[gpui::test]
    async fn test_http_transport_401_without_www_authenticate_fails(cx: &mut TestAppContext) {
        let client = FakeHttpClient::create(|_req| async move {
            Ok(Response::builder()
                .status(401)
                .body(AsyncBody::from("Unauthorized"))
                .unwrap())
        });

        let executor = cx.executor();
        let transport = HttpTransport::new(
            "test-server".to_string(),
            client,
            Url::parse("http://example.com/mcp").unwrap(),
            HashMap::default(),
            false,
            executor,
        );

        let message = json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "initialize",
            "params": {}
        });

        let result = transport
            .send(serde_json::to_string(&message).unwrap())
            .await;

        assert!(result.is_err());
        let err = result.unwrap_err();
        let transport_err = err
            .downcast_ref::<HttpTransportError>()
            .expect("expected HttpTransportError");
        assert!(
            matches!(
                transport_err,
                HttpTransportError::UnauthorizedMissingWwwAuthenticate { .. }
            ),
            "Expected UnauthorizedMissingWwwAuthenticate, got: {:?}",
            transport_err
        );
    }

    #[gpui::test]
    async fn test_http_transport_session_id_handling(cx: &mut TestAppContext) {
        let request_count = Arc::new(AtomicUsize::new(0));
        let session_ids_seen = Arc::new(SyncMutex::new(Vec::<Option<String>>::new()));

        let client = {
            let request_count = request_count.clone();
            let session_ids_seen = session_ids_seen.clone();
            FakeHttpClient::create(move |req| {
                let count = request_count.fetch_add(1, Ordering::SeqCst);
                let session_id = req
                    .headers()
                    .get("Mcp-Session-Id")
                    .and_then(|v| v.to_str().ok())
                    .map(String::from);
                session_ids_seen.lock().push(session_id);

                async move {
                    let mut response = create_mcp_response(
                        count as i64,
                        json!({
                            "protocolVersion": "2024-11-05",
                            "capabilities": {},
                            "serverInfo": { "name": "test", "version": "1.0" }
                        }),
                    );
                    if count == 0 {
                        response
                            .headers_mut()
                            .insert("Mcp-Session-Id", "session-123".parse().unwrap());
                    }
                    Ok(response)
                }
            })
        };

        let executor = cx.executor();
        let transport = HttpTransport::new(
            "test-server".to_string(),
            client,
            Url::parse("http://example.com/mcp").unwrap(),
            HashMap::default(),
            false,
            executor,
        );

        let message = json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "initialize",
            "params": {}
        });

        transport
            .send(serde_json::to_string(&message).unwrap())
            .await
            .unwrap();

        transport
            .send(serde_json::to_string(&message).unwrap())
            .await
            .unwrap();

        let seen = session_ids_seen.lock().clone();
        assert_eq!(seen.len(), 2);
        assert_eq!(seen[0], None);
        assert_eq!(seen[1], Some("session-123".to_string()));
    }

    #[gpui::test]
    async fn test_http_transport_custom_headers(cx: &mut TestAppContext) {
        let headers_seen = Arc::new(SyncMutex::new(StdHashMap::<String, String>::new()));

        let client = {
            let headers_seen = headers_seen.clone();
            FakeHttpClient::create(move |req| {
                let mut seen = StdHashMap::new();
                for (name, value) in req.headers() {
                    if let Ok(v) = value.to_str() {
                        seen.insert(name.to_string(), v.to_string());
                    }
                }
                *headers_seen.lock() = seen;

                async move {
                    Ok(create_mcp_response(
                        0,
                        json!({
                            "protocolVersion": "2024-11-05",
                            "capabilities": {},
                            "serverInfo": { "name": "test", "version": "1.0" }
                        }),
                    ))
                }
            })
        };

        let executor = cx.executor();
        let mut headers = HashMap::default();
        headers.insert("X-Custom-Header".to_string(), "custom-value".to_string());
        headers.insert("X-Another".to_string(), "another-value".to_string());

        let transport = HttpTransport::new(
            "test-server".to_string(),
            client,
            Url::parse("http://example.com/mcp").unwrap(),
            headers,
            false,
            executor,
        );

        let message = json!({ "jsonrpc": "2.0", "id": 0, "method": "test" });
        transport
            .send(serde_json::to_string(&message).unwrap())
            .await
            .unwrap();

        let seen = headers_seen.lock();
        assert_eq!(
            seen.get("x-custom-header"),
            Some(&"custom-value".to_string())
        );
        assert_eq!(seen.get("x-another"), Some(&"another-value".to_string()));
    }

    #[gpui::test]
    async fn test_http_transport_notification_accepted(cx: &mut TestAppContext) {
        let client = FakeHttpClient::create(|_req| async move {
            Ok(Response::builder()
                .status(202)
                .body(AsyncBody::empty())
                .unwrap())
        });

        let executor = cx.executor();
        let transport = HttpTransport::new(
            "test-server".to_string(),
            client,
            Url::parse("http://example.com/mcp").unwrap(),
            HashMap::default(),
            false,
            executor,
        );

        let notification = json!({
            "jsonrpc": "2.0",
            "method": "notifications/initialized"
        });

        let result = transport
            .send(serde_json::to_string(&notification).unwrap())
            .await;

        assert!(result.is_ok());
    }

    #[gpui::test]
    async fn test_http_transport_server_error(cx: &mut TestAppContext) {
        let client = FakeHttpClient::create(|_req| async move {
            Ok(Response::builder()
                .status(500)
                .body(AsyncBody::from("Internal Server Error"))
                .unwrap())
        });

        let executor = cx.executor();
        let transport = HttpTransport::new(
            "test-server".to_string(),
            client,
            Url::parse("http://example.com/mcp").unwrap(),
            HashMap::default(),
            false,
            executor,
        );

        let message = json!({ "jsonrpc": "2.0", "id": 0, "method": "test" });
        let result = transport
            .send(serde_json::to_string(&message).unwrap())
            .await;

        assert!(result.is_err());
        let err = result.unwrap_err().to_string();
        assert!(err.contains("500"), "Expected 500 error, got: {}", err);
    }

    #[gpui::test]
    async fn test_http_transport_refreshes_token_on_initial_401_when_auto_enabled(
        cx: &mut TestAppContext,
    ) {
        let request_count = Arc::new(AtomicUsize::new(0));
        let refresh_count = Arc::new(AtomicUsize::new(0));
        let seen_auth_headers = Arc::new(SyncMutex::new(Vec::<Option<String>>::new()));

        let client = {
            FakeHttpClient::create({
                let request_count = request_count.clone();
                let refresh_count = refresh_count.clone();
                let seen_auth_headers = seen_auth_headers.clone();
                move |req| {
                    let request_count = request_count.clone();
                    let refresh_count = refresh_count.clone();
                    let seen_auth_headers = seen_auth_headers.clone();
                    let uri = req.uri().to_string();
                    let auth = req
                        .headers()
                        .get(http::header::AUTHORIZATION)
                        .and_then(|v| v.to_str().ok())
                        .map(String::from);

                    async move {
                        if uri == "http://example.com/token" || uri.ends_with("/token") {
                            refresh_count.fetch_add(1, Ordering::SeqCst);
                            return Ok(Response::builder()
                                .status(200)
                                .header("Content-Type", "application/json")
                                .body(AsyncBody::from(
                                    json!({
                                        "access_token": "new-token",
                                        "token_type": "Bearer",
                                        "expires_in": 3600u64,
                                        "refresh_token": "refresh-2"
                                    })
                                    .to_string(),
                                ))
                                .unwrap());
                        }

                        let count = request_count.fetch_add(1, Ordering::SeqCst);
                        seen_auth_headers.lock().push(auth);

                        if count == 0 {
                            return Ok(Response::builder()
                                .status(401)
                                .header(http::header::WWW_AUTHENTICATE, "Bearer scope=\"mcp\"")
                                .body(AsyncBody::from("Unauthorized"))
                                .unwrap());
                        }

                        Ok(create_mcp_response(
                            count as i64,
                            json!({
                                "protocolVersion": "2024-11-05",
                                "capabilities": {},
                                "serverInfo": { "name": "test", "version": "1.0" }
                            }),
                        ))
                    }
                }
            })
        };

        let executor = cx.executor();
        let transport = HttpTransport::new(
            "test-server".to_string(),
            client.clone(),
            Url::parse("http://example.com/mcp").unwrap(),
            HashMap::default(),
            true,
            executor,
        );

        let oauth_tokens = crate::oauth::StoredOAuthTokens {
            server_name: "test-server".to_string(),
            url: "http://example.com/mcp".to_string(),
            client_id: "client-id".to_string(),
            token_endpoint: "http://example.com/token".to_string(),
            access_token: "old-token".to_string(),
            refresh_token: Some("refresh-1".to_string()),
            expires_at: Some(u64::MAX),
            scopes: Vec::new(),
            client_secret: None,
        };

        transport
            .oauth
            .as_ref()
            .expect("oauth manager should be present")
            .set_tokens_for_test(oauth_tokens);

        let message = json!({
            "jsonrpc": "2.0",
            "id": 0,
            "method": "initialize",
            "params": {}
        });

        transport
            .send(serde_json::to_string(&message).unwrap())
            .await
            .expect("send should succeed after refresh");

        assert_eq!(request_count.load(Ordering::SeqCst), 4);
        assert_eq!(refresh_count.load(Ordering::SeqCst), 1);

        let headers = seen_auth_headers.lock().clone();
        assert_eq!(
            headers.first().cloned(),
            Some(Some("Bearer old-token".to_string()))
        );
        assert!(
            headers
                .iter()
                .skip(1)
                .all(|h| h.as_deref() == Some("Bearer new-token")),
            "Expected all requests after refresh to use new token, got: {:?}",
            headers
        );
    }
}
