use anyhow::{Result, anyhow};
use async_trait::async_trait;
use futures::{AsyncReadExt, Stream, StreamExt, stream::BoxStream};
use http_client::{AsyncBody, HttpClient, Request, Response, http::Method};
use smol::channel;
use std::{
    pin::Pin,
    sync::{Arc, Mutex},
};

use crate::transport::Transport;

// Constants from MCP spec
const HEADER_SESSION_ID: &str = "x-mcp-session-id";
const HEADER_LAST_EVENT_ID: &str = "Last-Event-ID";
const EVENT_STREAM_MIME_TYPE: &str = "text/event-stream";
const JSON_MIME_TYPE: &str = "application/json";

/// HTTP Transport with session management and SSE support
///
/// This implementation follows the MCP HTTP transport spec:
/// 1. POST to endpoint with JSON-RPC message
/// 2. Response can be either:
///    - JSON response (Content-Type: application/json)
///    - SSE stream (Content-Type: text/event-stream)
///    - 202 Accepted (for notifications)
/// 3. Session management via x-mcp-session-id header
pub struct HttpTransport {
    http_client: Arc<dyn HttpClient>,
    endpoint: String,
    session_id: Option<String>,
    response_tx: channel::Sender<String>,
    response_rx: channel::Receiver<String>,
    error_tx: channel::Sender<String>,
    error_rx: channel::Receiver<String>,
    active_streams: Arc<Mutex<Vec<BoxStream<'static, Result<String>>>>>,
}

impl HttpTransport {
    pub fn new(http_client: Arc<dyn HttpClient>, endpoint: String) -> Self {
        let (response_tx, response_rx) = channel::unbounded();
        let (error_tx, error_rx) = channel::unbounded();

        Self {
            http_client,
            endpoint,
            session_id: None,
            response_tx,
            response_rx,
            error_tx,
            error_rx,
            active_streams: Arc::new(Mutex::new(Vec::new())),
        }
    }

    /// Send a message and handle the response based on content type
    async fn send_message(&mut self, message: String) -> Result<()> {
        let mut request_builder = Request::builder()
            .method(Method::POST)
            .uri(&self.endpoint)
            .header("Content-Type", "application/json")
            // Accept both JSON and SSE responses
            .header(
                "Accept",
                format!("{}, {}", JSON_MIME_TYPE, EVENT_STREAM_MIME_TYPE),
            );

        // Add session ID if we have one
        if let Some(ref session_id) = self.session_id {
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

                // Extract session ID from response if present
                if let Some(session_id) = response
                    .headers()
                    .get(HEADER_SESSION_ID)
                    .and_then(|v| v.to_str().ok())
                {
                    self.session_id = Some(session_id.to_string());
                }

                match content_type {
                    Some(ct) if ct.starts_with(JSON_MIME_TYPE) => {
                        // JSON response - read and forward immediately
                        let mut body = String::new();
                        AsyncReadExt::read_to_string(response.body_mut(), &mut body).await?;
                        self.response_tx
                            .send(body)
                            .await
                            .map_err(|_| anyhow!("Failed to send JSON response"))?;
                    }
                    Some(ct) if ct.starts_with(EVENT_STREAM_MIME_TYPE) => {
                        // SSE stream - set up streaming
                        self.setup_sse_stream(response).await?;
                    }
                    _ => {
                        return Err(anyhow!("Unexpected content type: {:?}", content_type));
                    }
                }
            }
            status if status.as_u16() == 202 => {
                // Accepted - notification acknowledged, no response needed
                log::debug!("Notification accepted");
            }
            _ => {
                let mut error_body = String::new();
                AsyncReadExt::read_to_string(response.body_mut(), &mut error_body).await?;
                self.error_tx
                    .send(format!("HTTP error {}: {}", response.status(), error_body))
                    .await
                    .map_err(|_| anyhow!("Failed to send error"))?;
            }
        }

        Ok(())
    }

    /// Set up SSE streaming from the response
    async fn setup_sse_stream(&mut self, mut response: Response<AsyncBody>) -> Result<()> {
        let response_tx = self.response_tx.clone();
        let error_tx = self.error_tx.clone();

        // Spawn a task to handle the SSE stream
        smol::spawn(async move {
            let reader = futures::io::BufReader::new(response.body_mut());
            let mut lines = futures::AsyncBufReadExt::lines(reader);

            let mut event_buffer = String::new();
            let mut data_buffer = Vec::new();

            while let Some(line_result) = lines.next().await {
                match line_result {
                    Ok(line) => {
                        if line.is_empty() {
                            // Empty line signals end of event
                            if !data_buffer.is_empty() {
                                let message = data_buffer.join("\n");
                                if let Err(e) = response_tx.send(message).await {
                                    log::error!("Failed to send SSE message: {}", e);
                                    break;
                                }
                                data_buffer.clear();
                            }
                            event_buffer.clear();
                        } else if let Some(data) = line.strip_prefix("data: ") {
                            // Accumulate data lines
                            data_buffer.push(data.to_string());
                        } else if let Some(event) = line.strip_prefix("event: ") {
                            event_buffer = event.to_string();
                        }
                        // Ignore other SSE fields like id:, retry:, etc.
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

    /// Connect to SSE endpoint for continuous updates (if needed)
    pub async fn connect_sse(&mut self) -> Result<()> {
        if self.session_id.is_none() {
            return Err(anyhow!("No session ID available for SSE connection"));
        }

        let request = Request::builder()
            .method(Method::GET)
            .uri(&self.endpoint)
            .header("Accept", EVENT_STREAM_MIME_TYPE)
            .header(HEADER_SESSION_ID, self.session_id.as_ref().unwrap());

        let request = request.body(AsyncBody::empty())?;
        let response = self.http_client.send(request).await?;

        if response.status().is_success() {
            self.setup_sse_stream(response).await?;
        } else {
            return Err(anyhow!("Failed to connect SSE: {}", response.status()));
        }

        Ok(())
    }

    /// Clean up session when transport is closed
    pub async fn cleanup_session(&mut self) -> Result<()> {
        if let Some(ref session_id) = self.session_id {
            let request = Request::builder()
                .method(Method::DELETE)
                .uri(&self.endpoint)
                .header(HEADER_SESSION_ID, session_id)
                .body(AsyncBody::empty())?;

            let response = self.http_client.send(request).await?;

            // 405 Method Not Allowed means the server doesn't support session cleanup
            if response.status().as_u16() != 405 && !response.status().is_success() {
                log::warn!("Failed to cleanup session: {}", response.status());
            }
        }
        Ok(())
    }
}

#[async_trait]
impl Transport for HttpTransport {
    async fn send(&self, message: String) -> Result<()> {
        // Clone self to get mutable access in async context
        let mut transport = Self {
            http_client: self.http_client.clone(),
            endpoint: self.endpoint.clone(),
            session_id: self.session_id.clone(),
            response_tx: self.response_tx.clone(),
            response_rx: self.response_rx.clone(),
            error_tx: self.error_tx.clone(),
            error_rx: self.error_rx.clone(),
            active_streams: self.active_streams.clone(),
        };

        transport.send_message(message).await
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
        let session_id = self.session_id.clone();

        if let Some(session_id) = session_id {
            smol::spawn(async move {
                let request = Request::builder()
                    .method(Method::DELETE)
                    .uri(&endpoint)
                    .header(HEADER_SESSION_ID, &session_id)
                    .body(AsyncBody::empty());

                if let Ok(request) = request {
                    let _ = http_client.send(request).await;
                }
            })
            .detach();
        }
    }
}
