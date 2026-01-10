mod auth;
mod www_authenticate;

use anyhow::{Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::{Stream, StreamExt, lock::Mutex};
use gpui::{AsyncApp, BackgroundExecutor};
use http_client::{AsyncBody, HttpClient, Request, Response, http::Method};
use parking_lot::Mutex as SyncMutex;
use smol::channel;
use std::{pin::Pin, sync::Arc};

use crate::transport::Transport;
use auth::OAuthClient;
use www_authenticate::WwwAuthenticate;

pub use auth::{AuthorizeUrl, OAuthCallback};

#[derive(Debug)]
pub struct AuthRequired {
    pub www_authenticate_header: Option<String>,
}

pub type AuthRequiredCallback = Arc<dyn Fn(AuthRequired) + Send + Sync>;

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
    // Authentication headers to include in requests
    headers: HashMap<String, String>,
    on_auth_required: AuthRequiredCallback,
    oauth_client: Arc<Mutex<Option<OAuthClient>>>,
}

impl HttpTransport {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        endpoint: String,
        headers: HashMap<String, String>,
        executor: BackgroundExecutor,
        on_auth_required: AuthRequiredCallback,
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
            on_auth_required,
            oauth_client: Arc::new(Mutex::new(None)),
        }
    }

    /// Send a message and handle the response based on content type
    async fn send_message(&self, message: String) -> Result<()> {
        let is_notification =
            !message.contains("\"id\":") || message.contains("notifications/initialized");

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

        let access_token: Option<String> = {
            let mut oauth_client_guard = self.oauth_client.lock().await;
            if let Some(oauth_client) = oauth_client_guard.as_mut() {
                match oauth_client.access_token().await {
                    Ok(Some(access_token)) => Some(access_token.to_owned()),
                    Ok(None) => None,
                    Err(error) => {
                        (self.on_auth_required)(AuthRequired {
                            www_authenticate_header: None,
                        });
                        return Err(error);
                    }
                }
            } else {
                None
            }
        };

        if let Some(access_token) = access_token {
            request_builder =
                request_builder.header("Authorization", format!("Bearer {}", access_token));
        }

        // Add session ID if we have one (except for initialize)
        if let Some(ref session_id) = *self.session_id.lock() {
            request_builder = request_builder.header(HEADER_SESSION_ID, session_id.as_str());
        }

        let request = request_builder.body(AsyncBody::from(message.into_bytes()))?;
        let mut response = self.http_client.send(request).await?;

        // Handle different response types based on status and content-type
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
            status if status.as_u16() == 401 => {
                let www_authenticate_header = response
                    .headers()
                    .get("WWW-Authenticate")
                    .and_then(|value| Some(value.to_str().ok()?.to_string()));

                (self.on_auth_required)(AuthRequired {
                    www_authenticate_header,
                });

                anyhow::bail!("Unauthorized");
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

    pub async fn restore_credentials(&self, cx: &AsyncApp) -> Result<()> {
        let mut client_guard = self.oauth_client.lock().await;

        if client_guard.is_some() {
            return Ok(());
        }

        if let Some(restored) =
            OAuthClient::load_from_keychain(&self.endpoint, &self.http_client, cx).await?
        {
            client_guard.replace(restored);
        };

        Ok(())
    }

    pub async fn start_auth(&self, www_auth_header: Option<&str>) -> Result<AuthorizeUrl> {
        let mut client_guard = self.oauth_client.lock().await;

        let www_authenticate = www_auth_header.and_then(WwwAuthenticate::parse);

        let client = match client_guard.as_mut() {
            Some(client) => client,
            None => {
                let new_client =
                    OAuthClient::init(&self.endpoint, www_authenticate.as_ref(), &self.http_client)
                        .await?;
                client_guard.replace(new_client);
                client_guard.as_mut().unwrap()
            }
        };

        let url = client.authorize_url()?;

        Ok(url)
    }

    pub async fn handle_oauth_callback(
        &self,
        callback: &OAuthCallback,
        cx: &AsyncApp,
    ) -> Result<()> {
        let mut client_guard = self.oauth_client.lock().await;
        let client = match client_guard.as_mut() {
            Some(client) => client,
            None => return Err(anyhow!("oauth client is not initialized; start auth first")),
        };

        client.exchange_token(&callback.code, cx).await?;

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
