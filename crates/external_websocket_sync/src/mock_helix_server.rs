//! Mock Helix WebSocket Server for Testing
//!
//! Provides a standalone mock of the Helix API's WebSocket sync endpoint
//! (`/api/v1/external-agents/sync`) for use in integration and protocol tests.
//!
//! The mock server implements the readiness protocol (queues commands until
//! `agent_ready` is received) and provides a clean API for scripting commands
//! and asserting on received events.
//!
//! # Usage
//!
//! ```rust,ignore
//! use external_websocket_sync::mock_helix_server::MockHelixServer;
//! use std::time::Duration;
//!
//! #[tokio::test]
//! async fn test_example() {
//!     let server = MockHelixServer::start().await;
//!     let url = server.url();
//!
//!     // ... connect a client ...
//!
//!     server.wait_for_connection("session-1", Duration::from_secs(5)).await.unwrap();
//!     server.wait_for_event("session-1", "agent_ready", Duration::from_secs(10)).await.unwrap();
//!     server.send_chat_message("session-1", "Hello", "req-1", None).await.unwrap();
//!
//!     let events = server.wait_for_event("session-1", "message_completed", Duration::from_secs(30)).await.unwrap();
//!     let all_events = server.get_events("session-1").await;
//! }
//! ```

use std::collections::HashMap;
use std::sync::Arc;
use std::time::Duration;

use futures::{SinkExt, StreamExt};
use serde::{Deserialize, Serialize};
use tokio::net::TcpListener;
use tokio::sync::{mpsc, Mutex, Notify, RwLock};
use tokio_tungstenite::tungstenite::Message;

// ---------------------------------------------------------------------------
// Public types matching the Helix API protocol
// ---------------------------------------------------------------------------

/// A SyncMessage received from Zed (Zed -> Helix direction).
/// Matches the Go `types.SyncMessage` struct.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ReceivedSyncMessage {
    /// The session_id field from the message (may be empty for some events)
    #[serde(default)]
    pub session_id: Option<String>,
    /// The event type (e.g., "agent_ready", "thread_created", "message_added", etc.)
    pub event_type: String,
    /// The event data payload
    #[serde(default)]
    pub data: serde_json::Value,
    /// Timestamp (optional in the wire format)
    #[serde(default)]
    pub timestamp: Option<String>,
}

/// An ExternalAgentCommand sent from the mock server to Zed (Helix -> Zed direction).
/// Matches the Go `types.ExternalAgentCommand` struct.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternalAgentCommand {
    #[serde(rename = "type")]
    pub command_type: String,
    pub data: serde_json::Value,
}

// ---------------------------------------------------------------------------
// Per-session state
// ---------------------------------------------------------------------------

/// Tracks the state of a single connected session.
struct SessionState {
    /// Whether the agent has sent `agent_ready`
    is_ready: bool,
    /// Commands queued before agent_ready (flushed when ready)
    pending_queue: Vec<ExternalAgentCommand>,
    /// All sync messages received from this session
    received_events: Vec<ReceivedSyncMessage>,
    /// Channel to send commands to this session's writer task
    command_tx: mpsc::UnboundedSender<ExternalAgentCommand>,
    /// Notifier for new events (wake up waiters)
    event_notify: Arc<Notify>,
}

// ---------------------------------------------------------------------------
// MockHelixServer
// ---------------------------------------------------------------------------

/// A mock Helix WebSocket server that listens on a random port and accepts
/// WebSocket connections from Zed clients.
///
/// Supports:
/// - Accepting connections on `/api/v1/external-agents/sync`
/// - Validating auth tokens
/// - Implementing the readiness protocol (queues until `agent_ready`)
/// - Sending scripted commands (`chat_message`, `open_thread`)
/// - Recording all received `SyncMessage`s for test assertions
/// - Multiple concurrent sessions
pub struct MockHelixServer {
    /// The port the server is listening on
    port: u16,
    /// Expected auth token (if set, connections without it are rejected)
    #[allow(dead_code)]
    expected_token: Option<String>,
    /// Per-session state, keyed by session_id from the query string
    sessions: Arc<RwLock<HashMap<String, SessionState>>>,
    /// Notifier for new connections (wake up `wait_for_connection`)
    connection_notify: Arc<Notify>,
    /// Handle to the server task (kept alive for the lifetime of the server)
    _server_handle: tokio::task::JoinHandle<()>,
}

impl MockHelixServer {
    /// Start a mock Helix server on a random available port.
    pub async fn start() -> Self {
        Self::start_with_token(None).await
    }

    /// Start a mock Helix server that requires a specific auth token.
    pub async fn start_with_token(expected_token: Option<String>) -> Self {
        let listener = TcpListener::bind("127.0.0.1:0")
            .await
            .expect("Failed to bind mock server");
        let port = listener.local_addr().unwrap().port();

        let sessions: Arc<RwLock<HashMap<String, SessionState>>> =
            Arc::new(RwLock::new(HashMap::new()));
        let connection_notify = Arc::new(Notify::new());

        let sessions_clone = sessions.clone();
        let connection_notify_clone = connection_notify.clone();
        let token_clone = expected_token.clone();

        let server_handle = tokio::spawn(async move {
            Self::accept_loop(listener, sessions_clone, connection_notify_clone, token_clone).await;
        });

        Self {
            port,
            expected_token,
            sessions,
            connection_notify,
            _server_handle: server_handle,
        }
    }

    /// Get the WebSocket URL for connecting to this mock server.
    /// Returns a URL like `ws://127.0.0.1:{port}`
    pub fn url(&self) -> String {
        format!("ws://127.0.0.1:{}", self.port)
    }

    /// Get just the host:port portion (for use with `WebSocketSyncConfig.url`).
    pub fn host_port(&self) -> String {
        format!("127.0.0.1:{}", self.port)
    }

    /// Get the port the server is listening on.
    pub fn port(&self) -> u16 {
        self.port
    }

    /// Wait for a session to connect within the given timeout.
    pub async fn wait_for_connection(
        &self,
        session_id: &str,
        timeout: Duration,
    ) -> Result<(), String> {
        let deadline = tokio::time::Instant::now() + timeout;

        loop {
            // Check if already connected
            if self.sessions.read().await.contains_key(session_id) {
                return Ok(());
            }

            // Wait for notification or timeout
            let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
            if remaining.is_zero() {
                return Err(format!(
                    "Timeout waiting for connection from session '{}'",
                    session_id
                ));
            }

            tokio::select! {
                _ = self.connection_notify.notified() => {
                    // Re-check in next loop iteration
                }
                _ = tokio::time::sleep(remaining) => {
                    return Err(format!(
                        "Timeout waiting for connection from session '{}'",
                        session_id
                    ));
                }
            }
        }
    }

    /// Wait for a specific event type from a session within the given timeout.
    /// Returns the matching event(s).
    pub async fn wait_for_event(
        &self,
        session_id: &str,
        event_type: &str,
        timeout: Duration,
    ) -> Result<Vec<ReceivedSyncMessage>, String> {
        let deadline = tokio::time::Instant::now() + timeout;

        // Get the notify handle for this session
        let notify = {
            let sessions = self.sessions.read().await;
            match sessions.get(session_id) {
                Some(state) => state.event_notify.clone(),
                None => {
                    return Err(format!("Session '{}' not connected", session_id));
                }
            }
        };

        loop {
            // Check if we already have matching events
            {
                let sessions = self.sessions.read().await;
                if let Some(state) = sessions.get(session_id) {
                    let matches: Vec<_> = state
                        .received_events
                        .iter()
                        .filter(|e| e.event_type == event_type)
                        .cloned()
                        .collect();
                    if !matches.is_empty() {
                        return Ok(matches);
                    }
                }
            }

            // Wait for notification or timeout
            let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
            if remaining.is_zero() {
                return Err(format!(
                    "Timeout waiting for event '{}' from session '{}'",
                    event_type, session_id
                ));
            }

            tokio::select! {
                _ = notify.notified() => {
                    // Re-check in next loop iteration
                }
                _ = tokio::time::sleep(remaining) => {
                    return Err(format!(
                        "Timeout waiting for event '{}' from session '{}'",
                        event_type, session_id
                    ));
                }
            }
        }
    }

    /// Wait until the session reaches the specified event count within the given timeout.
    /// Useful for waiting for an exact number of events (e.g., streaming chunks + completion).
    pub async fn wait_for_event_count(
        &self,
        session_id: &str,
        event_type: &str,
        count: usize,
        timeout: Duration,
    ) -> Result<Vec<ReceivedSyncMessage>, String> {
        let deadline = tokio::time::Instant::now() + timeout;

        let notify = {
            let sessions = self.sessions.read().await;
            match sessions.get(session_id) {
                Some(state) => state.event_notify.clone(),
                None => {
                    return Err(format!("Session '{}' not connected", session_id));
                }
            }
        };

        loop {
            {
                let sessions = self.sessions.read().await;
                if let Some(state) = sessions.get(session_id) {
                    let matches: Vec<_> = state
                        .received_events
                        .iter()
                        .filter(|e| e.event_type == event_type)
                        .cloned()
                        .collect();
                    if matches.len() >= count {
                        return Ok(matches);
                    }
                }
            }

            let remaining = deadline.saturating_duration_since(tokio::time::Instant::now());
            if remaining.is_zero() {
                return Err(format!(
                    "Timeout waiting for {} '{}' events from session '{}'",
                    count, event_type, session_id
                ));
            }

            tokio::select! {
                _ = notify.notified() => {}
                _ = tokio::time::sleep(remaining) => {
                    return Err(format!(
                        "Timeout waiting for {} '{}' events from session '{}'",
                        count, event_type, session_id
                    ));
                }
            }
        }
    }

    /// Send a `chat_message` command to the specified session.
    /// If the session is not yet ready (no `agent_ready` received), the command is queued.
    pub async fn send_chat_message(
        &self,
        session_id: &str,
        message: &str,
        request_id: &str,
        acp_thread_id: Option<&str>,
    ) -> Result<(), String> {
        let cmd = ExternalAgentCommand {
            command_type: "chat_message".to_string(),
            data: serde_json::json!({
                "message": message,
                "request_id": request_id,
                "acp_thread_id": acp_thread_id,
            }),
        };
        self.send_command(session_id, cmd).await
    }

    /// Send a `chat_message` command with an `agent_name` field.
    pub async fn send_chat_message_with_agent(
        &self,
        session_id: &str,
        message: &str,
        request_id: &str,
        acp_thread_id: Option<&str>,
        agent_name: &str,
    ) -> Result<(), String> {
        let cmd = ExternalAgentCommand {
            command_type: "chat_message".to_string(),
            data: serde_json::json!({
                "message": message,
                "request_id": request_id,
                "acp_thread_id": acp_thread_id,
                "agent_name": agent_name,
            }),
        };
        self.send_command(session_id, cmd).await
    }

    /// Send an `open_thread` command to the specified session.
    pub async fn send_open_thread(
        &self,
        session_id: &str,
        acp_thread_id: &str,
    ) -> Result<(), String> {
        let cmd = ExternalAgentCommand {
            command_type: "open_thread".to_string(),
            data: serde_json::json!({
                "acp_thread_id": acp_thread_id,
            }),
        };
        self.send_command(session_id, cmd).await
    }

    /// Send an `open_thread` command with an `agent_name` field.
    pub async fn send_open_thread_with_agent(
        &self,
        session_id: &str,
        acp_thread_id: &str,
        agent_name: &str,
    ) -> Result<(), String> {
        let cmd = ExternalAgentCommand {
            command_type: "open_thread".to_string(),
            data: serde_json::json!({
                "acp_thread_id": acp_thread_id,
                "agent_name": agent_name,
            }),
        };
        self.send_command(session_id, cmd).await
    }

    /// Send a raw `ExternalAgentCommand` to the specified session.
    /// Respects the readiness protocol: if the session is not ready, the command is queued.
    pub async fn send_command(
        &self,
        session_id: &str,
        cmd: ExternalAgentCommand,
    ) -> Result<(), String> {
        let mut sessions = self.sessions.write().await;
        let state = sessions
            .get_mut(session_id)
            .ok_or_else(|| format!("Session '{}' not connected", session_id))?;

        if state.is_ready {
            // Send immediately
            state
                .command_tx
                .send(cmd)
                .map_err(|e| format!("Failed to send command: {}", e))?;
        } else {
            // Queue for later (will be flushed when agent_ready arrives)
            state.pending_queue.push(cmd);
        }

        Ok(())
    }

    /// Send a command immediately, bypassing the readiness queue.
    /// Useful for testing edge cases.
    pub async fn send_command_immediate(
        &self,
        session_id: &str,
        cmd: ExternalAgentCommand,
    ) -> Result<(), String> {
        let sessions = self.sessions.read().await;
        let state = sessions
            .get(session_id)
            .ok_or_else(|| format!("Session '{}' not connected", session_id))?;

        state
            .command_tx
            .send(cmd)
            .map_err(|e| format!("Failed to send command: {}", e))
    }

    /// Get all events received from a session.
    pub async fn get_events(&self, session_id: &str) -> Vec<ReceivedSyncMessage> {
        let sessions = self.sessions.read().await;
        match sessions.get(session_id) {
            Some(state) => state.received_events.clone(),
            None => Vec::new(),
        }
    }

    /// Get events of a specific type received from a session.
    pub async fn get_events_of_type(
        &self,
        session_id: &str,
        event_type: &str,
    ) -> Vec<ReceivedSyncMessage> {
        let sessions = self.sessions.read().await;
        match sessions.get(session_id) {
            Some(state) => state
                .received_events
                .iter()
                .filter(|e| e.event_type == event_type)
                .cloned()
                .collect(),
            None => Vec::new(),
        }
    }

    /// Check if a session is currently connected.
    pub async fn is_connected(&self, session_id: &str) -> bool {
        self.sessions.read().await.contains_key(session_id)
    }

    /// Check if a session has sent `agent_ready`.
    pub async fn is_ready(&self, session_id: &str) -> bool {
        let sessions = self.sessions.read().await;
        sessions
            .get(session_id)
            .map(|s| s.is_ready)
            .unwrap_or(false)
    }

    /// Get the list of connected session IDs.
    pub async fn connected_sessions(&self) -> Vec<String> {
        self.sessions.read().await.keys().cloned().collect()
    }

    /// Clear all recorded events for a session (useful between test phases).
    pub async fn clear_events(&self, session_id: &str) {
        let mut sessions = self.sessions.write().await;
        if let Some(state) = sessions.get_mut(session_id) {
            state.received_events.clear();
        }
    }

    // -----------------------------------------------------------------------
    // Internal implementation
    // -----------------------------------------------------------------------

    /// Main accept loop: listens for incoming TCP connections and upgrades to WebSocket.
    async fn accept_loop(
        listener: TcpListener,
        sessions: Arc<RwLock<HashMap<String, SessionState>>>,
        connection_notify: Arc<Notify>,
        expected_token: Option<String>,
    ) {
        loop {
            let (stream, _addr) = match listener.accept().await {
                Ok(conn) => conn,
                Err(_) => break,
            };

            let sessions = sessions.clone();
            let connection_notify = connection_notify.clone();
            let expected_token = expected_token.clone();

            tokio::spawn(async move {
                // Use a callback to extract the session_id and validate auth from the HTTP upgrade.
                // NOTE: We use std::sync::Mutex here (not tokio::sync::Mutex) because the callback
                // passed to accept_hdr_async is synchronous and runs on the tokio runtime thread.
                // tokio::sync::Mutex::blocking_lock() panics when called from within a runtime.
                let session_id = Arc::new(std::sync::Mutex::new(String::new()));
                let session_id_for_callback = session_id.clone();
                let token_for_callback = expected_token.clone();

                let ws_stream = tokio_tungstenite::accept_hdr_async(
                    stream,
                    move |request: &tokio_tungstenite::tungstenite::handshake::server::Request,
                          response: tokio_tungstenite::tungstenite::handshake::server::Response| {
                        // Extract session_id from query string
                        let uri = request.uri().to_string();
                        let sid = Self::extract_session_id(&uri).unwrap_or_default();

                        // Validate auth token if required
                        if let Some(ref expected) = token_for_callback {
                            let auth_header = request
                                .headers()
                                .get("authorization")
                                .and_then(|v| v.to_str().ok())
                                .unwrap_or("");

                            let provided_token = auth_header.strip_prefix("Bearer ").unwrap_or("");
                            if provided_token != expected.as_str() {
                                // Reject the connection
                                return Err(tokio_tungstenite::tungstenite::handshake::server::ErrorResponse::new(None));
                            }
                        }

                        // Store session_id for use after handshake
                        *session_id_for_callback.lock().unwrap() = sid;

                        Ok(response)
                    },
                )
                .await;

                let ws_stream = match ws_stream {
                    Ok(ws) => ws,
                    Err(_e) => {
                        return;
                    }
                };

                let sid = session_id.lock().unwrap().clone();
                if sid.is_empty() {
                    // No session_id in URL, close connection
                    return;
                }

                let (command_tx, command_rx) = mpsc::unbounded_channel::<ExternalAgentCommand>();
                let event_notify = Arc::new(Notify::new());

                // Register the session
                {
                    let mut sessions_guard = sessions.write().await;
                    sessions_guard.insert(
                        sid.clone(),
                        SessionState {
                            is_ready: false,
                            pending_queue: Vec::new(),
                            received_events: Vec::new(),
                            command_tx: command_tx.clone(),
                            event_notify: event_notify.clone(),
                        },
                    );
                }

                // Notify waiters about the new connection
                connection_notify.notify_waiters();

                // Run the session handler
                Self::handle_session(ws_stream, sid.clone(), sessions.clone(), command_rx).await;

                // Clean up on disconnect
                sessions.write().await.remove(&sid);
            });
        }
    }

    /// Handle a single WebSocket session: read incoming messages and write outgoing commands.
    async fn handle_session(
        ws_stream: tokio_tungstenite::WebSocketStream<tokio::net::TcpStream>,
        session_id: String,
        sessions: Arc<RwLock<HashMap<String, SessionState>>>,
        mut command_rx: mpsc::UnboundedReceiver<ExternalAgentCommand>,
    ) {
        let (mut ws_sink, mut ws_stream) = ws_stream.split();

        loop {
            tokio::select! {
                // Handle incoming messages from the client (Zed)
                msg = ws_stream.next() => {
                    match msg {
                        Some(Ok(Message::Text(text))) => {
                            // Try to parse as SyncMessage (OutgoingMessage format from Zed)
                            if let Ok(sync_msg) = serde_json::from_str::<ReceivedSyncMessage>(&text) {
                                let is_agent_ready = sync_msg.event_type == "agent_ready";

                                // Record the event
                                let mut sessions_guard = sessions.write().await;
                                if let Some(state) = sessions_guard.get_mut(&session_id) {
                                    state.received_events.push(sync_msg);
                                    state.event_notify.notify_waiters();

                                    // Handle agent_ready: mark ready and flush queue
                                    if is_agent_ready && !state.is_ready {
                                        state.is_ready = true;
                                        let queued: Vec<_> = state.pending_queue.drain(..).collect();
                                        drop(sessions_guard);

                                        // Send all queued commands
                                        for cmd in queued {
                                            let json = serde_json::to_string(&cmd).unwrap();
                                            if ws_sink.send(Message::Text(json.into())).await.is_err() {
                                                return;
                                            }
                                        }
                                    }
                                }
                            }
                        }
                        Some(Ok(Message::Ping(data))) => {
                            if ws_sink.send(Message::Pong(data)).await.is_err() {
                                return;
                            }
                        }
                        Some(Ok(Message::Close(_))) | None => {
                            return;
                        }
                        Some(Ok(_)) => {
                            // Ignore binary, pong, etc.
                        }
                        Some(Err(_)) => {
                            return;
                        }
                    }
                }

                // Handle outgoing commands to the client (Zed)
                Some(cmd) = command_rx.recv() => {
                    let json = serde_json::to_string(&cmd).unwrap();
                    if ws_sink.send(Message::Text(json.into())).await.is_err() {
                        return;
                    }
                }
            }
        }
    }

    /// Extract session_id from a URI query string.
    /// e.g., "/api/v1/external-agents/sync?session_id=abc" -> "abc"
    fn extract_session_id(uri: &str) -> Option<String> {
        let query = uri.split('?').nth(1)?;
        for pair in query.split('&') {
            let mut parts = pair.splitn(2, '=');
            let key = parts.next()?;
            let value = parts.next()?;
            if key == "session_id" {
                return Some(value.to_string());
            }
        }
        None
    }
}

// ===========================================================================
// Tests
// ===========================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::types::{IncomingChatMessage, OutgoingMessage, SyncEvent};

    // -----------------------------------------------------------------------
    // Protocol serialization tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_sync_event_serialization_roundtrip_thread_created() {
        let event = SyncEvent::ThreadCreated {
            acp_thread_id: "thread-123".to_string(),
            request_id: "req-001".to_string(),
        };

        let outgoing = event.to_outgoing_message().unwrap();
        assert_eq!(outgoing.event_type, "thread_created");
        assert_eq!(outgoing.data["acp_thread_id"], "thread-123");
        assert_eq!(outgoing.data["request_id"], "req-001");

        // Roundtrip through JSON
        let json = serde_json::to_string(&outgoing).unwrap();
        let parsed: OutgoingMessage = serde_json::from_str(&json).unwrap();
        assert_eq!(parsed.event_type, "thread_created");
        assert_eq!(parsed.data["acp_thread_id"], "thread-123");
    }

    #[test]
    fn test_sync_event_serialization_roundtrip_message_added() {
        let event = SyncEvent::MessageAdded {
            acp_thread_id: "thread-123".to_string(),
            message_id: "msg-456".to_string(),
            role: "assistant".to_string(),
            content: "Hello, world!".to_string(),
            entry_type: "text".to_string(),
                tool_name: String::new(),
                tool_status: String::new(),
            timestamp: 1706000000,
        };

        let outgoing = event.to_outgoing_message().unwrap();
        assert_eq!(outgoing.event_type, "message_added");
        assert_eq!(outgoing.data["acp_thread_id"], "thread-123");
        assert_eq!(outgoing.data["message_id"], "msg-456");
        assert_eq!(outgoing.data["role"], "assistant");
        assert_eq!(outgoing.data["content"], "Hello, world!");
        assert_eq!(outgoing.data["timestamp"], 1706000000);
    }

    #[test]
    fn test_sync_event_serialization_roundtrip_message_completed() {
        let event = SyncEvent::MessageCompleted {
            acp_thread_id: "thread-123".to_string(),
            message_id: "msg-456".to_string(),
            request_id: "req-001".to_string(),
        };

        let outgoing = event.to_outgoing_message().unwrap();
        assert_eq!(outgoing.event_type, "message_completed");
        assert_eq!(outgoing.data["acp_thread_id"], "thread-123");
        assert_eq!(outgoing.data["message_id"], "msg-456");
        assert_eq!(outgoing.data["request_id"], "req-001");
    }

    #[test]
    fn test_sync_event_serialization_roundtrip_agent_ready() {
        let event = SyncEvent::AgentReady {
            agent_name: "qwen".to_string(),
            thread_id: Some("thread-existing".to_string()),
        };

        let outgoing = event.to_outgoing_message().unwrap();
        assert_eq!(outgoing.event_type, "agent_ready");
        assert_eq!(outgoing.data["agent_name"], "qwen");
        assert_eq!(outgoing.data["thread_id"], "thread-existing");
    }

    #[test]
    fn test_sync_event_serialization_agent_ready_null_thread() {
        let event = SyncEvent::AgentReady {
            agent_name: "zed-agent".to_string(),
            thread_id: None,
        };

        let outgoing = event.to_outgoing_message().unwrap();
        assert_eq!(outgoing.event_type, "agent_ready");
        assert_eq!(outgoing.data["agent_name"], "zed-agent");
        assert!(outgoing.data["thread_id"].is_null());
    }

    #[test]
    fn test_sync_event_serialization_user_created_thread() {
        let event = SyncEvent::UserCreatedThread {
            acp_thread_id: "thread-user-1".to_string(),
            title: Some("My Thread".to_string()),
        };

        let outgoing = event.to_outgoing_message().unwrap();
        assert_eq!(outgoing.event_type, "user_created_thread");
        assert_eq!(outgoing.data["acp_thread_id"], "thread-user-1");
        assert_eq!(outgoing.data["title"], "My Thread");
    }

    #[test]
    fn test_sync_event_serialization_thread_title_changed() {
        let event = SyncEvent::ThreadTitleChanged {
            acp_thread_id: "thread-123".to_string(),
            title: "Updated Title".to_string(),
        };

        let outgoing = event.to_outgoing_message().unwrap();
        assert_eq!(outgoing.event_type, "thread_title_changed");
        assert_eq!(outgoing.data["acp_thread_id"], "thread-123");
        assert_eq!(outgoing.data["title"], "Updated Title");
    }

    #[test]
    fn test_sync_event_serialization_thread_load_error() {
        let event = SyncEvent::ThreadLoadError {
            acp_thread_id: "thread-123".to_string(),
            request_id: "req-001".to_string(),
            error: "Thread already active".to_string(),
        };

        let outgoing = event.to_outgoing_message().unwrap();
        assert_eq!(outgoing.event_type, "thread_load_error");
        assert_eq!(outgoing.data["acp_thread_id"], "thread-123");
        assert_eq!(outgoing.data["request_id"], "req-001");
        assert_eq!(outgoing.data["error"], "Thread already active");
    }

    // -----------------------------------------------------------------------
    // ExternalAgentCommand serialization tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_external_agent_command_chat_message_serialization() {
        let cmd = ExternalAgentCommand {
            command_type: "chat_message".to_string(),
            data: serde_json::json!({
                "message": "Hello",
                "request_id": "req-001",
                "acp_thread_id": null,
            }),
        };

        let json = serde_json::to_string(&cmd).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["type"], "chat_message");
        assert_eq!(parsed["data"]["message"], "Hello");
        assert_eq!(parsed["data"]["request_id"], "req-001");
        assert!(parsed["data"]["acp_thread_id"].is_null());
    }

    #[test]
    fn test_external_agent_command_open_thread_serialization() {
        let cmd = ExternalAgentCommand {
            command_type: "open_thread".to_string(),
            data: serde_json::json!({
                "acp_thread_id": "thread-123",
            }),
        };

        let json = serde_json::to_string(&cmd).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        assert_eq!(parsed["type"], "open_thread");
        assert_eq!(parsed["data"]["acp_thread_id"], "thread-123");
    }

    #[test]
    fn test_external_agent_command_deserialization() {
        let json = r#"{"type": "chat_message", "data": {"message": "Hi", "request_id": "r1", "acp_thread_id": null}}"#;
        let cmd: ExternalAgentCommand = serde_json::from_str(json).unwrap();

        assert_eq!(cmd.command_type, "chat_message");
        assert_eq!(cmd.data["message"], "Hi");
        assert_eq!(cmd.data["request_id"], "r1");
        assert!(cmd.data["acp_thread_id"].is_null());
    }

    // -----------------------------------------------------------------------
    // OutgoingMessage format tests (matches what Helix expects)
    // -----------------------------------------------------------------------

    #[test]
    fn test_outgoing_message_format_matches_helix_sync_message() {
        // The OutgoingMessage is what Zed sends over the wire.
        // Helix expects: {"event_type": "...", "data": {...}}
        let outgoing = OutgoingMessage {
            event_type: "thread_created".to_string(),
            data: serde_json::json!({
                "acp_thread_id": "thread-1",
                "request_id": "req-1",
            }),
        };

        let json = serde_json::to_string(&outgoing).unwrap();
        let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();

        // Verify the wire format has the expected fields
        assert!(parsed.get("event_type").is_some(), "Must have event_type field");
        assert!(parsed.get("data").is_some(), "Must have data field");
        assert_eq!(parsed["event_type"], "thread_created");
        assert_eq!(parsed["data"]["acp_thread_id"], "thread-1");
        assert_eq!(parsed["data"]["request_id"], "req-1");
    }

    #[test]
    fn test_outgoing_message_all_event_types_have_correct_format() {
        // Verify all SyncEvent variants produce valid OutgoingMessage with correct event_type
        let events: Vec<(SyncEvent, &str)> = vec![
            (
                SyncEvent::AgentReady {
                    agent_name: "test".to_string(),
                    thread_id: None,
                },
                "agent_ready",
            ),
            (
                SyncEvent::ThreadCreated {
                    acp_thread_id: "t1".to_string(),
                    request_id: "r1".to_string(),
                },
                "thread_created",
            ),
            (
                SyncEvent::UserCreatedThread {
                    acp_thread_id: "t2".to_string(),
                    title: None,
                },
                "user_created_thread",
            ),
            (
                SyncEvent::ThreadTitleChanged {
                    acp_thread_id: "t3".to_string(),
                    title: "Title".to_string(),
                },
                "thread_title_changed",
            ),
            (
                SyncEvent::MessageAdded {
                    acp_thread_id: "t4".to_string(),
                    message_id: "m1".to_string(),
                    role: "assistant".to_string(),
                    content: "Hello".to_string(),
                    entry_type: "text".to_string(),
                tool_name: String::new(),
                tool_status: String::new(),
                    timestamp: 0,
                },
                "message_added",
            ),
            (
                SyncEvent::MessageCompleted {
                    acp_thread_id: "t5".to_string(),
                    message_id: "m2".to_string(),
                    request_id: "r2".to_string(),
                },
                "message_completed",
            ),
            (
                SyncEvent::ThreadLoadError {
                    acp_thread_id: "t6".to_string(),
                    request_id: "r3".to_string(),
                    error: "err".to_string(),
                },
                "thread_load_error",
            ),
        ];

        for (event, expected_type) in events {
            let outgoing = event.to_outgoing_message().unwrap();
            assert_eq!(
                outgoing.event_type, expected_type,
                "Event type mismatch for {:?}",
                expected_type
            );

            // Verify it serializes to valid JSON
            let json = serde_json::to_string(&outgoing).unwrap();
            let parsed: serde_json::Value = serde_json::from_str(&json).unwrap();
            assert!(parsed.get("event_type").is_some());
            assert!(parsed.get("data").is_some());
        }
    }

    // -----------------------------------------------------------------------
    // IncomingChatMessage parsing tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_incoming_chat_message_parsing_new_thread() {
        let json = r#"{
            "message": "What is the meaning of life?",
            "request_id": "req-001",
            "acp_thread_id": null
        }"#;

        let msg: IncomingChatMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.message, "What is the meaning of life?");
        assert_eq!(msg.request_id, "req-001");
        assert!(msg.acp_thread_id.is_none());
        assert!(msg.agent_name.is_none());
    }

    #[test]
    fn test_incoming_chat_message_parsing_existing_thread() {
        let json = r#"{
            "message": "Follow-up question",
            "request_id": "req-002",
            "acp_thread_id": "thread-existing-123"
        }"#;

        let msg: IncomingChatMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.message, "Follow-up question");
        assert_eq!(msg.request_id, "req-002");
        assert_eq!(msg.acp_thread_id.as_deref(), Some("thread-existing-123"));
    }

    #[test]
    fn test_incoming_chat_message_parsing_with_agent_name() {
        let json = r#"{
            "message": "Hello",
            "request_id": "req-003",
            "acp_thread_id": null,
            "agent_name": "qwen"
        }"#;

        let msg: IncomingChatMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.message, "Hello");
        assert_eq!(msg.agent_name.as_deref(), Some("qwen"));
    }

    #[test]
    fn test_incoming_chat_message_parsing_with_optional_fields() {
        let json = r#"{
            "message": "Hello",
            "request_id": "req-004",
            "acp_thread_id": null,
            "role": "user",
            "session_id": "ses-external-001"
        }"#;

        let msg: IncomingChatMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.role.as_deref(), Some("user"));
        assert_eq!(msg.session_id.as_deref(), Some("ses-external-001"));
    }

    #[test]
    fn test_incoming_chat_message_missing_optional_fields_default() {
        let json = r#"{
            "message": "Hi",
            "request_id": "req-005"
        }"#;

        let msg: IncomingChatMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.message, "Hi");
        assert_eq!(msg.request_id, "req-005");
        assert!(msg.acp_thread_id.is_none());
        assert!(msg.role.is_none());
        assert!(msg.session_id.is_none());
        assert!(msg.agent_name.is_none());
    }

    // -----------------------------------------------------------------------
    // ReceivedSyncMessage parsing tests
    // -----------------------------------------------------------------------

    #[test]
    fn test_received_sync_message_parsing() {
        let json = r#"{
            "event_type": "agent_ready",
            "data": {"agent_name": "qwen", "thread_id": null}
        }"#;

        let msg: ReceivedSyncMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.event_type, "agent_ready");
        assert_eq!(msg.data["agent_name"], "qwen");
        assert!(msg.data["thread_id"].is_null());
    }

    #[test]
    fn test_received_sync_message_with_session_id() {
        let json = r#"{
            "session_id": "ses-123",
            "event_type": "thread_created",
            "data": {"acp_thread_id": "thread-1", "request_id": "req-1"},
            "timestamp": "2026-01-01T00:00:00Z"
        }"#;

        let msg: ReceivedSyncMessage = serde_json::from_str(json).unwrap();
        assert_eq!(msg.session_id.as_deref(), Some("ses-123"));
        assert_eq!(msg.event_type, "thread_created");
        assert_eq!(msg.data["acp_thread_id"], "thread-1");
        assert_eq!(msg.timestamp.as_deref(), Some("2026-01-01T00:00:00Z"));
    }

    // -----------------------------------------------------------------------
    // Mock server integration tests
    // -----------------------------------------------------------------------

    #[tokio::test]
    async fn test_mock_server_starts_and_accepts_connection() {
        let server = MockHelixServer::start().await;
        let url = format!(
            "{}/api/v1/external-agents/sync?session_id=test-session-1",
            server.url()
        );

        // Connect a client
        let (ws_stream, _) = tokio_tungstenite::connect_async(&url).await.unwrap();
        let (_write, _read) = ws_stream.split();

        // Verify connection is registered
        server
            .wait_for_connection("test-session-1", Duration::from_secs(2))
            .await
            .unwrap();
        assert!(server.is_connected("test-session-1").await);
    }

    #[tokio::test]
    async fn test_mock_server_records_received_events() {
        let server = MockHelixServer::start().await;
        let url = format!(
            "{}/api/v1/external-agents/sync?session_id=test-session-2",
            server.url()
        );

        let (ws_stream, _) = tokio_tungstenite::connect_async(&url).await.unwrap();
        let (mut write, _read) = ws_stream.split();

        server
            .wait_for_connection("test-session-2", Duration::from_secs(2))
            .await
            .unwrap();

        // Send agent_ready
        let agent_ready = serde_json::json!({
            "event_type": "agent_ready",
            "data": {"agent_name": "test-agent", "thread_id": null}
        });
        write
            .send(Message::Text(agent_ready.to_string().into()))
            .await
            .unwrap();

        // Wait for the event to be recorded
        let events = server
            .wait_for_event("test-session-2", "agent_ready", Duration::from_secs(2))
            .await
            .unwrap();

        assert_eq!(events.len(), 1);
        assert_eq!(events[0].event_type, "agent_ready");
        assert_eq!(events[0].data["agent_name"], "test-agent");
    }

    #[tokio::test]
    async fn test_mock_server_readiness_protocol_queues_commands() {
        let server = MockHelixServer::start().await;
        let url = format!(
            "{}/api/v1/external-agents/sync?session_id=test-session-3",
            server.url()
        );

        let (ws_stream, _) = tokio_tungstenite::connect_async(&url).await.unwrap();
        let (mut write, mut read) = ws_stream.split();

        server
            .wait_for_connection("test-session-3", Duration::from_secs(2))
            .await
            .unwrap();

        // Send a chat_message BEFORE agent_ready -- should be queued
        server
            .send_chat_message("test-session-3", "Hello!", "req-1", None)
            .await
            .unwrap();

        // Verify no message received yet (short wait)
        let no_msg = tokio::time::timeout(Duration::from_millis(200), read.next()).await;
        assert!(
            no_msg.is_err(),
            "Should not receive message before agent_ready"
        );

        // Now send agent_ready
        let agent_ready = serde_json::json!({
            "event_type": "agent_ready",
            "data": {"agent_name": "test-agent", "thread_id": null}
        });
        write
            .send(Message::Text(agent_ready.to_string().into()))
            .await
            .unwrap();

        // Now the queued message should arrive
        let msg = tokio::time::timeout(Duration::from_secs(2), read.next())
            .await
            .expect("Should receive queued message after agent_ready")
            .unwrap()
            .unwrap();

        if let Message::Text(text) = msg {
            let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
            assert_eq!(parsed["type"], "chat_message");
            assert_eq!(parsed["data"]["message"], "Hello!");
            assert_eq!(parsed["data"]["request_id"], "req-1");
        } else {
            panic!("Expected text message, got {:?}", msg);
        }
    }

    #[tokio::test]
    async fn test_mock_server_sends_commands_immediately_when_ready() {
        let server = MockHelixServer::start().await;
        let url = format!(
            "{}/api/v1/external-agents/sync?session_id=test-session-4",
            server.url()
        );

        let (ws_stream, _) = tokio_tungstenite::connect_async(&url).await.unwrap();
        let (mut write, mut read) = ws_stream.split();

        server
            .wait_for_connection("test-session-4", Duration::from_secs(2))
            .await
            .unwrap();

        // Send agent_ready first
        let agent_ready = serde_json::json!({
            "event_type": "agent_ready",
            "data": {"agent_name": "test-agent", "thread_id": null}
        });
        write
            .send(Message::Text(agent_ready.to_string().into()))
            .await
            .unwrap();

        // Wait for ready state
        server
            .wait_for_event("test-session-4", "agent_ready", Duration::from_secs(2))
            .await
            .unwrap();

        // Now send chat_message -- should arrive immediately (no queuing)
        server
            .send_chat_message("test-session-4", "Hello!", "req-2", None)
            .await
            .unwrap();

        let msg = tokio::time::timeout(Duration::from_secs(2), read.next())
            .await
            .expect("Should receive message immediately when ready")
            .unwrap()
            .unwrap();

        if let Message::Text(text) = msg {
            let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
            assert_eq!(parsed["type"], "chat_message");
            assert_eq!(parsed["data"]["message"], "Hello!");
        } else {
            panic!("Expected text message");
        }
    }

    #[tokio::test]
    async fn test_mock_server_auth_token_validation_accepts_valid() {
        let server = MockHelixServer::start_with_token(Some("test-token-123".to_string())).await;
        let url = format!(
            "{}/api/v1/external-agents/sync?session_id=test-session-5",
            server.url()
        );

        // Connect with valid token
        let request = tokio_tungstenite::tungstenite::http::Request::builder()
            .uri(&url)
            .header("Host", server.host_port())
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header(
                "Sec-WebSocket-Key",
                tokio_tungstenite::tungstenite::handshake::client::generate_key(),
            )
            .header("Authorization", "Bearer test-token-123")
            .body(())
            .unwrap();

        let result = tokio_tungstenite::connect_async(request).await;
        assert!(result.is_ok(), "Should accept valid auth token");

        server
            .wait_for_connection("test-session-5", Duration::from_secs(2))
            .await
            .unwrap();
    }

    #[tokio::test]
    async fn test_mock_server_auth_token_validation_rejects_invalid() {
        let server = MockHelixServer::start_with_token(Some("correct-token".to_string())).await;
        let url = format!(
            "{}/api/v1/external-agents/sync?session_id=test-session-6",
            server.url()
        );

        // Connect with invalid token
        let request = tokio_tungstenite::tungstenite::http::Request::builder()
            .uri(&url)
            .header("Host", server.host_port())
            .header("Connection", "Upgrade")
            .header("Upgrade", "websocket")
            .header("Sec-WebSocket-Version", "13")
            .header(
                "Sec-WebSocket-Key",
                tokio_tungstenite::tungstenite::handshake::client::generate_key(),
            )
            .header("Authorization", "Bearer wrong-token")
            .body(())
            .unwrap();

        let result = tokio_tungstenite::connect_async(request).await;
        assert!(result.is_err(), "Should reject invalid auth token");
    }

    #[tokio::test]
    async fn test_mock_server_multiple_sessions() {
        let server = MockHelixServer::start().await;

        // Connect two sessions
        let url1 = format!(
            "{}/api/v1/external-agents/sync?session_id=session-A",
            server.url()
        );
        let url2 = format!(
            "{}/api/v1/external-agents/sync?session_id=session-B",
            server.url()
        );

        let (ws1, _) = tokio_tungstenite::connect_async(&url1).await.unwrap();
        let (ws2, _) = tokio_tungstenite::connect_async(&url2).await.unwrap();

        let (mut write1, _read1) = ws1.split();
        let (mut write2, _read2) = ws2.split();

        server
            .wait_for_connection("session-A", Duration::from_secs(2))
            .await
            .unwrap();
        server
            .wait_for_connection("session-B", Duration::from_secs(2))
            .await
            .unwrap();

        // Send different events from each session
        let ready_a = serde_json::json!({
            "event_type": "agent_ready",
            "data": {"agent_name": "agent-A", "thread_id": null}
        });
        let ready_b = serde_json::json!({
            "event_type": "agent_ready",
            "data": {"agent_name": "agent-B", "thread_id": null}
        });

        write1
            .send(Message::Text(ready_a.to_string().into()))
            .await
            .unwrap();
        write2
            .send(Message::Text(ready_b.to_string().into()))
            .await
            .unwrap();

        // Verify events are tracked independently
        let events_a = server
            .wait_for_event("session-A", "agent_ready", Duration::from_secs(2))
            .await
            .unwrap();
        let events_b = server
            .wait_for_event("session-B", "agent_ready", Duration::from_secs(2))
            .await
            .unwrap();

        assert_eq!(events_a[0].data["agent_name"], "agent-A");
        assert_eq!(events_b[0].data["agent_name"], "agent-B");

        // Verify both sessions are listed
        let sessions = server.connected_sessions().await;
        assert!(sessions.contains(&"session-A".to_string()));
        assert!(sessions.contains(&"session-B".to_string()));
    }

    #[tokio::test]
    async fn test_mock_server_send_open_thread() {
        let server = MockHelixServer::start().await;
        let url = format!(
            "{}/api/v1/external-agents/sync?session_id=test-session-7",
            server.url()
        );

        let (ws_stream, _) = tokio_tungstenite::connect_async(&url).await.unwrap();
        let (mut write, mut read) = ws_stream.split();

        server
            .wait_for_connection("test-session-7", Duration::from_secs(2))
            .await
            .unwrap();

        // Send agent_ready first
        let agent_ready = serde_json::json!({
            "event_type": "agent_ready",
            "data": {"agent_name": "test", "thread_id": null}
        });
        write
            .send(Message::Text(agent_ready.to_string().into()))
            .await
            .unwrap();

        server
            .wait_for_event("test-session-7", "agent_ready", Duration::from_secs(2))
            .await
            .unwrap();

        // Send open_thread command
        server
            .send_open_thread("test-session-7", "thread-to-open")
            .await
            .unwrap();

        let msg = tokio::time::timeout(Duration::from_secs(2), read.next())
            .await
            .expect("Should receive open_thread command")
            .unwrap()
            .unwrap();

        if let Message::Text(text) = msg {
            let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
            assert_eq!(parsed["type"], "open_thread");
            assert_eq!(parsed["data"]["acp_thread_id"], "thread-to-open");
        } else {
            panic!("Expected text message");
        }
    }

    #[tokio::test]
    async fn test_mock_server_get_events_of_type() {
        let server = MockHelixServer::start().await;
        let url = format!(
            "{}/api/v1/external-agents/sync?session_id=test-session-8",
            server.url()
        );

        let (ws_stream, _) = tokio_tungstenite::connect_async(&url).await.unwrap();
        let (mut write, _read) = ws_stream.split();

        server
            .wait_for_connection("test-session-8", Duration::from_secs(2))
            .await
            .unwrap();

        // Send multiple different events
        let events = vec![
            serde_json::json!({"event_type": "agent_ready", "data": {"agent_name": "test", "thread_id": null}}),
            serde_json::json!({"event_type": "thread_created", "data": {"acp_thread_id": "t1", "request_id": "r1"}}),
            serde_json::json!({"event_type": "message_added", "data": {"acp_thread_id": "t1", "message_id": "m1", "role": "assistant", "content": "Hi", "timestamp": 0}}),
            serde_json::json!({"event_type": "message_added", "data": {"acp_thread_id": "t1", "message_id": "m1", "role": "assistant", "content": "Hi there", "timestamp": 0}}),
            serde_json::json!({"event_type": "message_completed", "data": {"acp_thread_id": "t1", "message_id": "m1", "request_id": "r1"}}),
        ];

        for event in &events {
            write
                .send(Message::Text(event.to_string().into()))
                .await
                .unwrap();
        }

        // Wait for all events to arrive
        server
            .wait_for_event_count("test-session-8", "message_added", 2, Duration::from_secs(2))
            .await
            .unwrap();

        // Get events by type
        let added = server
            .get_events_of_type("test-session-8", "message_added")
            .await;
        assert_eq!(added.len(), 2);
        assert_eq!(added[0].data["content"], "Hi");
        assert_eq!(added[1].data["content"], "Hi there");

        let completed = server
            .get_events_of_type("test-session-8", "message_completed")
            .await;
        assert_eq!(completed.len(), 1);

        let all_events = server.get_events("test-session-8").await;
        assert_eq!(all_events.len(), 5);
    }

    #[tokio::test]
    async fn test_mock_server_clear_events() {
        let server = MockHelixServer::start().await;
        let url = format!(
            "{}/api/v1/external-agents/sync?session_id=test-session-9",
            server.url()
        );

        let (ws_stream, _) = tokio_tungstenite::connect_async(&url).await.unwrap();
        let (mut write, _read) = ws_stream.split();

        server
            .wait_for_connection("test-session-9", Duration::from_secs(2))
            .await
            .unwrap();

        // Send an event
        let agent_ready = serde_json::json!({
            "event_type": "agent_ready",
            "data": {"agent_name": "test", "thread_id": null}
        });
        write
            .send(Message::Text(agent_ready.to_string().into()))
            .await
            .unwrap();

        server
            .wait_for_event("test-session-9", "agent_ready", Duration::from_secs(2))
            .await
            .unwrap();

        assert_eq!(server.get_events("test-session-9").await.len(), 1);

        // Clear events
        server.clear_events("test-session-9").await;
        assert_eq!(server.get_events("test-session-9").await.len(), 0);
    }

    #[tokio::test]
    async fn test_mock_server_wait_for_event_count() {
        let server = MockHelixServer::start().await;
        let url = format!(
            "{}/api/v1/external-agents/sync?session_id=test-session-10",
            server.url()
        );

        let (ws_stream, _) = tokio_tungstenite::connect_async(&url).await.unwrap();
        let (mut write, _read) = ws_stream.split();

        server
            .wait_for_connection("test-session-10", Duration::from_secs(2))
            .await
            .unwrap();

        // Send 3 message_added events with a delay between them
        for i in 0..3 {
            let event = serde_json::json!({
                "event_type": "message_added",
                "data": {
                    "acp_thread_id": "t1",
                    "message_id": "m1",
                    "role": "assistant",
                    "content": format!("chunk {}", i),
                    "timestamp": i
                }
            });
            write
                .send(Message::Text(event.to_string().into()))
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_millis(50)).await;
        }

        // Wait for exactly 3
        let events = server
            .wait_for_event_count("test-session-10", "message_added", 3, Duration::from_secs(2))
            .await
            .unwrap();

        assert_eq!(events.len(), 3);
        assert_eq!(events[0].data["content"], "chunk 0");
        assert_eq!(events[1].data["content"], "chunk 1");
        assert_eq!(events[2].data["content"], "chunk 2");
    }

    #[tokio::test]
    async fn test_mock_server_full_protocol_flow() {
        // End-to-end test of the complete protocol flow:
        // 1. Client connects
        // 2. Server queues a chat_message
        // 3. Client sends agent_ready
        // 4. Server flushes queued chat_message
        // 5. Client receives chat_message and responds with thread_created, message_added, message_completed
        // 6. Server verifies all events

        let server = MockHelixServer::start().await;
        let url = format!(
            "{}/api/v1/external-agents/sync?session_id=flow-session",
            server.url()
        );

        let (ws_stream, _) = tokio_tungstenite::connect_async(&url).await.unwrap();
        let (mut write, mut read) = ws_stream.split();

        server
            .wait_for_connection("flow-session", Duration::from_secs(2))
            .await
            .unwrap();

        // Queue a chat_message before agent is ready
        server
            .send_chat_message("flow-session", "What is 2+2?", "req-flow-1", None)
            .await
            .unwrap();

        // Client sends agent_ready
        let agent_ready = serde_json::json!({
            "event_type": "agent_ready",
            "data": {"agent_name": "flow-agent", "thread_id": null}
        });
        write
            .send(Message::Text(agent_ready.to_string().into()))
            .await
            .unwrap();

        // Client receives the queued chat_message
        let msg = tokio::time::timeout(Duration::from_secs(2), read.next())
            .await
            .expect("Should receive queued chat_message")
            .unwrap()
            .unwrap();

        let chat_msg: serde_json::Value = match msg {
            Message::Text(text) => serde_json::from_str(&text).unwrap(),
            other => panic!("Expected text, got {:?}", other),
        };
        assert_eq!(chat_msg["type"], "chat_message");
        assert_eq!(chat_msg["data"]["message"], "What is 2+2?");
        assert_eq!(chat_msg["data"]["request_id"], "req-flow-1");

        // Client responds with thread_created
        let thread_created = serde_json::json!({
            "event_type": "thread_created",
            "data": {"acp_thread_id": "flow-thread-1", "request_id": "req-flow-1"}
        });
        write
            .send(Message::Text(thread_created.to_string().into()))
            .await
            .unwrap();

        // Client streams message_added (progressive content)
        for content in &["The", "The answer", "The answer is 4"] {
            let msg_added = serde_json::json!({
                "event_type": "message_added",
                "data": {
                    "acp_thread_id": "flow-thread-1",
                    "message_id": "flow-msg-1",
                    "role": "assistant",
                    "content": content,
                    "timestamp": 1706000000
                }
            });
            write
                .send(Message::Text(msg_added.to_string().into()))
                .await
                .unwrap();
            tokio::time::sleep(Duration::from_millis(20)).await;
        }

        // Client sends message_completed
        let msg_completed = serde_json::json!({
            "event_type": "message_completed",
            "data": {
                "acp_thread_id": "flow-thread-1",
                "message_id": "flow-msg-1",
                "request_id": "req-flow-1"
            }
        });
        write
            .send(Message::Text(msg_completed.to_string().into()))
            .await
            .unwrap();

        // Verify all events were recorded
        server
            .wait_for_event("flow-session", "message_completed", Duration::from_secs(2))
            .await
            .unwrap();

        let all_events = server.get_events("flow-session").await;
        // Expected: agent_ready + thread_created + 3 message_added + message_completed = 6
        assert_eq!(all_events.len(), 6, "Should have 6 events total");

        assert_eq!(all_events[0].event_type, "agent_ready");
        assert_eq!(all_events[1].event_type, "thread_created");
        assert_eq!(all_events[2].event_type, "message_added");
        assert_eq!(all_events[3].event_type, "message_added");
        assert_eq!(all_events[4].event_type, "message_added");
        assert_eq!(all_events[5].event_type, "message_completed");

        // Verify streaming content is progressive
        assert_eq!(all_events[2].data["content"], "The");
        assert_eq!(all_events[3].data["content"], "The answer");
        assert_eq!(all_events[4].data["content"], "The answer is 4");

        // Verify thread_created has correct request_id
        assert_eq!(all_events[1].data["request_id"], "req-flow-1");
        assert_eq!(all_events[1].data["acp_thread_id"], "flow-thread-1");

        // Verify message_completed has correct fields
        assert_eq!(all_events[5].data["acp_thread_id"], "flow-thread-1");
        assert_eq!(all_events[5].data["request_id"], "req-flow-1");
    }

    #[tokio::test]
    async fn test_mock_server_send_command_immediate_bypasses_queue() {
        let server = MockHelixServer::start().await;
        let url = format!(
            "{}/api/v1/external-agents/sync?session_id=test-session-11",
            server.url()
        );

        let (ws_stream, _) = tokio_tungstenite::connect_async(&url).await.unwrap();
        let (_write, mut read) = ws_stream.split();

        server
            .wait_for_connection("test-session-11", Duration::from_secs(2))
            .await
            .unwrap();

        // Session is NOT ready, but send_command_immediate should bypass the queue
        let cmd = ExternalAgentCommand {
            command_type: "chat_message".to_string(),
            data: serde_json::json!({"message": "Bypass!", "request_id": "req-bypass"}),
        };

        server
            .send_command_immediate("test-session-11", cmd)
            .await
            .unwrap();

        // Should receive immediately even though not ready
        let msg = tokio::time::timeout(Duration::from_secs(2), read.next())
            .await
            .expect("Should receive immediate command")
            .unwrap()
            .unwrap();

        if let Message::Text(text) = msg {
            let parsed: serde_json::Value = serde_json::from_str(&text).unwrap();
            assert_eq!(parsed["data"]["message"], "Bypass!");
        } else {
            panic!("Expected text message");
        }
    }
}

entry_type: "text".to_string(),
                tool_name: String::new(),
                tool_status: String::new(),
