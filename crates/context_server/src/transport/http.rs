// zed/crates/context_server/src/transport/http.rs
use anyhow::{Result, anyhow};
use async_trait::async_trait;
use collections::HashMap;
use futures::{Stream, StreamExt};
use gpui::{App, BackgroundExecutor};
use http_client::{AsyncBody, HttpClient, Request, Response, http::Method};
use parking_lot::Mutex as SyncMutex;
use smol::channel; // todo!() can this be futures::channel::mpsc()
use std::{pin::Pin, sync::Arc};

use crate::transport::Transport;

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
    // Track if we've sent the initialize response
    initialized: Arc<SyncMutex<bool>>,
    // Authentication headers to include in requests
    auth_headers: Option<HashMap<String, String>>,
}

impl HttpTransport {
    pub fn new(http_client: Arc<dyn HttpClient>, endpoint: String, cx: &App) -> Self {
        let (response_tx, response_rx) = channel::unbounded();
        let (error_tx, error_rx) = channel::unbounded();

        Self {
            http_client,
            executor: cx.background_executor().clone(),
            endpoint,
            session_id: Arc::new(SyncMutex::new(None)),
            response_tx,
            response_rx,
            error_tx,
            error_rx,
            initialized: Arc::new(SyncMutex::new(false)),
            auth_headers: None,
        }
    }

    /// Add authentication headers to this transport
    pub fn with_auth_headers(mut self, headers: HashMap<String, String>) -> Self {
        self.auth_headers = Some(headers);
        self
    }

    /// Send a message and handle the response based on content type
    async fn send_message(&self, message: String) -> Result<()> {
        // Check if this is an initialize request
        let is_initialize = message.contains("\"method\":\"initialize\"");
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

        // Add authentication headers if present
        if let Some(ref headers) = self.auth_headers {
            for (key, value) in headers {
                request_builder = request_builder.header(key.as_str(), value.as_str());
            }
        }

        // Add session ID if we have one (except for initialize)
        if !is_initialize {
            if let Some(ref session_id) = *self.session_id.lock() {
                request_builder = request_builder.header(HEADER_SESSION_ID, session_id.as_str());
            }
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

                        // Mark as initialized after setting up the first SSE stream
                        if is_initialize {
                            *self.initialized.lock() = true;
                        }
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

                // Log the error but don't propagate for notifications
                if is_notification {
                    log::warn!("Notification error {}: {}", response.status(), error_body);
                } else {
                    self.error_tx
                        .send(format!("HTTP error {}: {}", response.status(), error_body))
                        .await
                        .map_err(|_| anyhow!("Failed to send error"))?;
                }
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
                dbg!(&line_result); // do we see `data: `? or do we just get one JSON blob per line
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
        let auth_headers = self.auth_headers.clone();

        if let Some(session_id) = session_id {
            self.executor
                .spawn(async move {
                    let mut request_builder = Request::builder()
                        .method(Method::DELETE)
                        .uri(&endpoint)
                        .header(HEADER_SESSION_ID, &session_id);

                    // Add authentication headers if present
                    if let Some(ref headers) = auth_headers {
                        for (key, value) in headers {
                            request_builder = request_builder.header(key.as_str(), value.as_str());
                        }
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
