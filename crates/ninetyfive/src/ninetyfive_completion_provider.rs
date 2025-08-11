use crate::NinetyFive;
use anyhow::Result;
use async_tungstenite::{
    tokio::client_async_tls_with_connector_and_config, tungstenite::Message, WebSocketStream,
};
use chrono::{DateTime, Duration, Utc};
use edit_prediction::{Direction, EditPrediction, EditPredictionProvider};
use futures::{
    stream::{SplitSink, SplitStream},
    SinkExt, StreamExt,
};
use gpui::{App, Context, Entity, Task};
use gpui_tokio::Tokio;
use http_client_tls;
use language::{Anchor, Buffer, BufferSnapshot, EditPreview, ToOffset};
use project::Project;
use serde_json;
use std::{
    ops::Range,
    sync::{Arc, OnceLock},
};
use tokio::{
    net::TcpStream,
    sync::{mpsc, Mutex},
    time::sleep,
};

const NINETYFIVE_API_URL: &str = "wss://api.ninetyfive.gg";

type WebSocketConnection = WebSocketStream<async_tungstenite::tokio::ConnectStream>;

#[derive(Clone)]
struct CurrentCompletion {
    snapshot: BufferSnapshot,
    edits: Arc<[(Range<Anchor>, String)]>,
    edit_preview: EditPreview,
    cursor_offset: usize,
}

impl CurrentCompletion {
    fn interpolate(&self, new_snapshot: &BufferSnapshot) -> Option<Vec<(Range<Anchor>, String)>> {
        interpolate(&self.snapshot, new_snapshot, self.edits.clone())
    }
}

pub struct NinetyFiveCompletionProvider {
    ninetyfive: Entity<NinetyFive>,
    current_completion: Option<CurrentCompletion>,
    pending_refresh: Option<Task<Result<()>>>,
}

static WEBSOCKET_CLIENT: OnceLock<Arc<WebSocketClient>> = OnceLock::new();

#[derive(Debug)]
enum WebSocketMessage {
    FileContent {
        path: String,
        content: String,
    },
    CompletionRequest {
        pos: usize,
        repo: String,
        request_id: String,
    },
}

#[derive(Clone)]
pub struct WebSocketClient {
    api_url: String,
    message_sender: Arc<Mutex<Option<mpsc::UnboundedSender<WebSocketMessage>>>>,
    current_request_id: Arc<Mutex<Option<String>>>,
    current_completion_text: Arc<Mutex<String>>,
    current_completion_offset: Arc<Mutex<Option<usize>>>,
}

impl WebSocketClient {
    fn new(api_url: String) -> Self {
        Self {
            api_url,
            message_sender: Arc::new(Mutex::new(None)),
            current_request_id: Arc::new(Mutex::new(None)),
            current_completion_text: Arc::new(Mutex::new(String::new())),
            current_completion_offset: Arc::new(Mutex::new(None)),
        }
    }

    pub fn get_singleton(cx: &App) -> Arc<WebSocketClient> {
        WEBSOCKET_CLIENT
            .get_or_init(|| {
                let client = Arc::new(Self::new(NINETYFIVE_API_URL.to_string()));

                // Start the connection manager immediately
                let client_clone = client.clone();
                let task = Tokio::spawn(cx, async move {
                    client_clone.start_connection_manager().await;
                });
                task.detach();

                client
            })
            .clone()
    }

    async fn create_connection(&self) -> Result<WebSocketConnection> {
        log::debug!(
            "NinetyFive: Creating websocket connection to {}",
            self.api_url
        );

        // Parse URL and connect to TCP stream first
        let url = url::Url::parse(&self.api_url)?;
        let host = url
            .host_str()
            .ok_or_else(|| anyhow::anyhow!("Invalid host in URL"))?;
        let port = url.port().unwrap_or(443);

        log::debug!("NinetyFive: Connecting to TCP {}:{}", host, port);

        // Create TCP connection
        let tcp_stream = TcpStream::connect((host, port)).await?;
        log::debug!("NinetyFive: TCP connection successful");

        // Create websocket connection with TLS
        log::debug!(
            "NinetyFive: Attempting websocket handshake to {}",
            self.api_url
        );
        let (ws_stream, _) = client_async_tls_with_connector_and_config(
            &self.api_url,
            tcp_stream,
            Some(Arc::new(http_client_tls::tls_config()).into()),
            None,
        )
        .await?;

        log::info!("NinetyFive: Websocket connection established");
        Ok(ws_stream)
    }

    async fn start_connection_manager(&self) {
        loop {
            match self.create_connection().await {
                Ok(connection) => {
                    log::info!("NinetyFive: Connection established, starting manager");

                    let (tx, mut rx) = mpsc::unbounded_channel();

                    // Store the sender
                    {
                        let mut sender_guard = self.message_sender.lock().await;
                        *sender_guard = Some(tx);
                    }

                    let (mut sink, mut stream) = connection.split();
                    let current_request_id = self.current_request_id.clone();
                    let current_completion_text = self.current_completion_text.clone();

                    // Spawn message sender task
                    let sender_task = tokio::spawn(async move {
                        while let Some(msg) = rx.recv().await {
                            let json_msg = match msg {
                                WebSocketMessage::FileContent { path, content } => {
                                    serde_json::json!({
                                        "type": "file-content",
                                        "path": path,
                                        "text": content
                                    })
                                }
                                WebSocketMessage::CompletionRequest {
                                    pos,
                                    repo,
                                    request_id,
                                } => {
                                    serde_json::json!({
                                        "type": "delta-completion-request",
                                        "requestId": request_id,
                                        "repo": repo,
                                        "pos": pos
                                    })
                                }
                            };

                            if let Err(e) =
                                sink.send(Message::Text(json_msg.to_string().into())).await
                            {
                                log::error!("NinetyFive: Failed to send message: {}", e);
                                break;
                            }
                        }
                    });

                    // Spawn message receiver task
                    let receiver_task = tokio::spawn(async move {
                        while let Some(msg) = stream.next().await {
                            match msg {
                                Ok(Message::Text(text)) => {
                                    let now = Utc::now();
                                    println!("Response {}", now);
                                    log::info!("NinetyFive: Received: {}", text);

                                    if let Ok(response) =
                                        serde_json::from_str::<serde_json::Value>(&text)
                                    {
                                        if let Some(response_id) =
                                            response.get("r").and_then(|r| r.as_str())
                                        {
                                            // Check if this is for the current request (non-blocking)
                                            if let Ok(current_id_guard) =
                                                current_request_id.try_lock()
                                            {
                                                if let Some(ref current_id) = *current_id_guard {
                                                    if response_id == current_id {
                                                        if let Some(value) = response
                                                            .get("v")
                                                            .and_then(|v| v.as_str())
                                                        {
                                                            // Append to current completion (non-blocking)
                                                            if let Ok(mut completion_guard) =
                                                                current_completion_text.try_lock()
                                                            {
                                                                completion_guard.push_str(value);
                                                                log::info!(
                                                                    "NinetyFive: Updated completion: '{}'",
                                                                    *completion_guard
                                                                );
                                                            }
                                                        }
                                                    }
                                                }
                                            }
                                        }
                                    }
                                }
                                Ok(Message::Close(_)) => {
                                    log::info!("NinetyFive: Connection closed");
                                    break;
                                }
                                Err(e) => {
                                    log::error!("NinetyFive: Connection error: {}", e);
                                    break;
                                }
                                _ => {}
                            }
                        }
                    });

                    // Wait for either task to complete (indicating connection loss)
                    tokio::select! {
                        _ = sender_task => {
                            log::warn!("NinetyFive: Sender task ended");
                        }
                        _ = receiver_task => {
                            log::warn!("NinetyFive: Receiver task ended");
                        }
                    }

                    // Clear the sender
                    {
                        let mut sender_guard = self.message_sender.lock().await;
                        *sender_guard = None;
                    }
                }
                Err(e) => {
                    log::error!("NinetyFive: Failed to create connection: {}", e);
                }
            }

            // Wait before reconnecting
            tokio::time::sleep(std::time::Duration::from_secs(5)).await;
        }
    }

    pub async fn send_file_content(&self, path: &str, content: &str) -> Result<()> {
        let sender_guard = self.message_sender.lock().await;
        if let Some(ref sender) = *sender_guard {
            sender.send(WebSocketMessage::FileContent {
                path: path.to_string(),
                content: content.to_string(),
            })?;
            log::debug!("NinetyFive: Queued file content for {}", path);
            Ok(())
        } else {
            Err(anyhow::anyhow!("No websocket connection available"))
        }
    }

    pub async fn send_delta_completion_request(
        &self,
        pos: usize,
        repo: &str,
        _file_path: Option<&str>,
        file_content: Option<&str>,
    ) -> Result<()> {
        // Always send file content before completion request if provided
        if let Some(content) = file_content {
            self.send_file_content("Untitled-1", content).await?;
        } else {
            log::info!("NinetyFive: No file content to send");
            return Ok(());
        }

        let request_id = generate_request_id();

        // Clear current completion and set new request ID (non-blocking)
        // This ensures we don't show stale completions from previous requests
        if let Ok(mut current_id_guard) = self.current_request_id.try_lock() {
            *current_id_guard = Some(request_id.clone());
        }
        if let Ok(mut completion_guard) = self.current_completion_text.try_lock() {
            completion_guard.clear();
        }
        if let Ok(mut offset_guard) = self.current_completion_offset.try_lock() {
            *offset_guard = Some(pos);
        }

        let sender_guard = self.message_sender.lock().await;
        if let Some(ref sender) = *sender_guard {
            sender.send(WebSocketMessage::CompletionRequest {
                pos,
                repo: repo.to_string(),
                request_id: request_id.clone(),
            })?;

            let now = Utc::now();
            println!("Request{}", now);
            log::info!(
                "NinetyFive: Queued completion request {} at pos {}",
                request_id,
                pos
            );
            Ok(())
        } else {
            Err(anyhow::anyhow!("No websocket connection available"))
        }
    }

    pub async fn get_current_completion(&self) -> String {
        if let Ok(completion_guard) = self.current_completion_text.try_lock() {
            completion_guard.clone()
        } else {
            String::new()
        }
    }
}

fn generate_request_id() -> String {
    use std::collections::hash_map::DefaultHasher;
    use std::hash::{Hash, Hasher};
    use std::time::{SystemTime, UNIX_EPOCH};

    let mut hasher = DefaultHasher::new();
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap()
        .as_nanos()
        .hash(&mut hasher);
    format!("{:x}", hasher.finish())[..6].to_string()
}

impl NinetyFiveCompletionProvider {
    pub fn new(ninetyfive: Entity<NinetyFive>, _cx: &App) -> Self {
        Self {
            ninetyfive,
            current_completion: None,
            pending_refresh: None,
        }
    }
}

impl EditPredictionProvider for NinetyFiveCompletionProvider {
    fn name() -> &'static str {
        "ninetyfive"
    }

    fn display_name() -> &'static str {
        "NinetyFive"
    }

    fn show_completions_in_menu() -> bool {
        true
    }

    fn is_enabled(&self, _buffer: &Entity<Buffer>, _cursor_position: Anchor, cx: &App) -> bool {
        log::debug!("NinetyFive: is enabled enter");
        let enabled = self.ninetyfive.read(cx).is_enabled();
        log::debug!("NinetyFive: Provider enabled: {}", enabled);
        enabled
    }

    fn is_refreshing(&self) -> bool {
        self.pending_refresh.is_some()
    }

    fn refresh(
        &mut self,
        _project: Option<Entity<Project>>,
        buffer_handle: Entity<Buffer>,
        cursor_position: Anchor,
        debounce: bool,
        cx: &mut Context<Self>,
    ) {
        log::info!("NinetyFive: Refresh called (debounce: {})", debounce);

        // Get repo name (fallback to "unknown")
        let buffer = buffer_handle.read(cx);
        let repo = buffer
            .file()
            .and_then(|file| {
                file.path()
                    .ancestors()
                    .find(|p| p.join(".git").exists())
                    .and_then(|p| p.file_name())
                    .map(|name| name.to_string_lossy().to_string())
            })
            .unwrap_or_else(|| "unknown".to_string());

        // Get file information for the completion request
        let (file_path, file_content) = if let Some(file) = buffer.file() {
            let path = file.path().to_string_lossy().to_string();
            let content = buffer.text();
            (Some(path), Some(content))
        } else {
            (None, None)
        };

        let cursor_offset = cursor_position.to_offset(&buffer);
        let client = WebSocketClient::get_singleton(cx);
        let client_clone = client.clone();

        // Send the completion request first using Tokio::spawn
        let task = Tokio::spawn(cx, async move {
            if debounce {
                sleep(std::time::Duration::from_millis(15)).await;
            }

            // Send the completion request
            if let Err(e) = client_clone
                .send_delta_completion_request(
                    cursor_offset,
                    &repo,
                    file_path.as_deref(),
                    file_content.as_deref(),
                )
                .await
            {
                log::error!("NinetyFive: Completion request failed: {}", e);
                return;
            }

            log::info!("NinetyFive: Completion request sent successfully");
        });

        // Now set the pending refresh task to wait for completion
        self.pending_refresh = Some(cx.spawn(async move |this, cx| {
            // Use Tokio::spawn for the polling loop that needs sleep
            let polling_task = Tokio::spawn(cx, async move {
                // Wait for completion to be ready by polling the client
                let mut attempts = 0;
                let max_attempts = 100; // 5 seconds max wait time

                loop {
                    let completion_text = client.get_current_completion().await;

                    if !completion_text.is_empty() {
                        log::info!("NinetyFive: Completion ready: '{}'", completion_text);
                        return true; // Completion found
                    }

                    attempts += 1;
                    if attempts >= max_attempts {
                        log::warn!("NinetyFive: Timeout waiting for completion");
                        return false; // Timeout
                    }

                    sleep(std::time::Duration::from_millis(10)).await;
                }
            });

            // Wait for the polling task to complete
            let completion_ready = match polling_task {
                Ok(task) => match task.await {
                    Ok(result) => result,
                    Err(e) => {
                        log::error!("NinetyFive: Polling task failed: {}", e);
                        false
                    }
                },
                Err(e) => {
                    log::error!("NinetyFive: Failed to spawn polling task: {}", e);
                    false
                }
            };

            // Update the provider state and notify
            this.update(cx, |this, cx| {
                this.pending_refresh = None;
                cx.notify();
            })?;

            if completion_ready {
                log::info!("NinetyFive: Completion polling completed successfully");
            } else {
                log::warn!("NinetyFive: Completion polling timed out");
            }

            Ok(())
        }));

        task.detach();
    }

    fn cycle(
        &mut self,
        _buffer: Entity<Buffer>,
        _cursor_position: language::Anchor,
        _direction: Direction,
        _cx: &mut Context<Self>,
    ) {
        // Does nothing
    }

    fn accept(&mut self, _cx: &mut Context<Self>) {
        log::debug!("NinetyFive: Completion accepted");
        self.pending_refresh = None;
        self.current_completion = None;
    }

    fn discard(&mut self, _cx: &mut Context<Self>) {
        log::debug!("NinetyFive: Completion discarded");
        self.pending_refresh = None;
        self.current_completion = None;
    }

    fn suggest(
        &mut self,
        buffer: &Entity<Buffer>,
        cursor_position: language::Anchor,
        cx: &mut Context<Self>,
    ) -> Option<EditPrediction> {
        let now = Utc::now();
        log::info!("NinetyFive: Suggest called {}", now);

        // Get current buffer snapshot
        let buffer_snapshot = buffer.read(cx);
        let snapshot = buffer_snapshot.snapshot();
        let cursor_offset = cursor_position.to_offset(&buffer_snapshot);

        // Check if we have a current completion and if it's still valid
        if let Some(current_completion) = &self.current_completion {
            // Check if the completion is still valid for the current position and buffer state
            if current_completion.snapshot.version() == snapshot.version()
                && current_completion.cursor_offset == cursor_offset
            {
                if let Some(edits) = current_completion.interpolate(&snapshot) {
                    if !edits.is_empty() {
                        log::info!(
                            "NinetyFive: Reusing existing completion {} {}",
                            current_completion.cursor_offset,
                            cursor_offset
                        );
                        return Some(EditPrediction {
                            id: None,
                            edits,
                            edit_preview: Some(current_completion.edit_preview.clone()),
                        });
                    }
                }
            } else {
                // Buffer has changed or cursor moved, invalidate current completion
                log::info!(
                    "NinetyFive: Buffer changed or cursor moved, invalidating current completion"
                );
                self.current_completion = None;
            }
        }

        // Check if we have any completion text from the shared state
        let client = WebSocketClient::get_singleton(cx);
        let (completion_text, completion_offset) = if let (Ok(completion_guard), Ok(offset_guard)) = (
            client.current_completion_text.try_lock(),
            client.current_completion_offset.try_lock(),
        ) {
            (completion_guard.clone(), *offset_guard)
        } else {
            (String::new(), None)
        };

        // If we have completion text and no current completion, check if it's for the current position
        if !completion_text.is_empty() && self.current_completion.is_none() {
            // Only use the completion if it's for the current cursor position
            if completion_offset == Some(cursor_offset) {
                log::info!(
                    "NinetyFive: Found completion text for current position: '{}'",
                    completion_text
                );

                let position = cursor_position.bias_right(&buffer_snapshot);
                let edits: Arc<[(Range<Anchor>, String)]> =
                    Arc::from([(position..position, completion_text.clone())]);

                // Create edit preview using the buffer's preview_edits method
                let edit_preview_task = buffer_snapshot.preview_edits(edits.clone(), cx);
                let edit_preview = cx.background_executor().block(edit_preview_task);

                let current_completion = CurrentCompletion {
                    snapshot: snapshot.clone(),
                    edits: edits.clone(),
                    edit_preview,
                    cursor_offset,
                };

                self.current_completion = Some(current_completion.clone());

                log::info!("should show?");
                if let Some(edits) = current_completion.interpolate(&snapshot) {
                    if !edits.is_empty() {
                        log::info!("madaskdfjasdfk");
                        return Some(EditPrediction {
                            id: None,
                            edits,
                            edit_preview: Some(current_completion.edit_preview.clone()),
                        });
                    }
                }
            } else {
                log::info!("NinetyFive: Ignoring completion text for different position: expected {}, got {:?}", cursor_offset, completion_offset);
            }
        }

        // No completion available yet
        None
    }
}

fn interpolate(
    old_snapshot: &BufferSnapshot,
    new_snapshot: &BufferSnapshot,
    current_edits: Arc<[(Range<Anchor>, String)]>,
) -> Option<Vec<(Range<Anchor>, String)>> {
    // We should only have one edit (cursor insertion) in the simplified model
    if current_edits.len() != 1 {
        return None;
    }

    let (edit_range, completion_text) = &current_edits[0];
    let cursor_offset = edit_range.start.to_offset(old_snapshot);

    // Check what the user has typed since the prediction
    for user_edit in new_snapshot.edits_since::<usize>(&old_snapshot.version) {
        // If the user edit is at our cursor position
        if user_edit.old.start == cursor_offset && user_edit.old.end == cursor_offset {
            let user_typed = new_snapshot
                .text_for_range(user_edit.new.clone())
                .collect::<String>();

            // Check if what the user typed matches the beginning of our completion
            if let Some(remaining) = completion_text.strip_prefix(&user_typed) {
                if remaining.is_empty() {
                    // User typed the entire completion
                    return None;
                }
                // Adjust to insert only the remaining part
                let new_cursor = new_snapshot.anchor_after(user_edit.new.end);
                return Some(vec![(new_cursor..new_cursor, remaining.to_string())]);
            } else if !user_typed.is_empty() {
                // User typed something different
                return None;
            }
        } else if user_edit.old.contains(&cursor_offset) || cursor_offset > user_edit.old.end {
            // User made an edit that affects our insertion point
            return None;
        }
    }

    // No conflicting edits, return original completion
    Some(vec![(edit_range.clone(), completion_text.clone())])
}
