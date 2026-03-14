//! Types for Helix integration

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use schemars::JsonSchema;
use std::collections::HashMap;
use std::rc::Rc;
use std::sync::Arc;
use gpui::SharedString;
use project::agent_server_store::AgentServerCommand;

/// External agent type for thread creation
#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize, JsonSchema)]
#[serde(rename_all = "snake_case")]
pub enum ExternalAgent {
    #[default]
    Gemini,
    ClaudeCode,
    NativeAgent,
    Custom {
        name: SharedString,
        command: AgentServerCommand,
    },
}

impl ExternalAgent {
    pub fn name(&self) -> &'static str {
        match self {
            Self::NativeAgent => "zed",
            Self::Gemini => "gemini-cli",
            Self::ClaudeCode => "claude-code",
            Self::Custom { .. } => "custom",
        }
    }

    pub fn server(
        &self,
        fs: Arc<dyn fs::Fs>,
        history: gpui::Entity<agent::ThreadStore>,
    ) -> Rc<dyn agent_servers::AgentServer> {
        match self {
            Self::Gemini => Rc::new(agent_servers::Gemini),
            Self::ClaudeCode => Rc::new(agent_servers::ClaudeCode),
            Self::NativeAgent => Rc::new(agent::NativeAgentServer::new(fs, history)),
            Self::Custom { name, command: _ } => {
                Rc::new(agent_servers::CustomAgentServer::new(name.clone()))
            }
        }
    }
}

/// Information about the current session
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SessionInfo {
    pub session_id: String,
    pub last_session_id: Option<String>,
    pub active_contexts: usize,
    pub websocket_connected: bool,
    pub sync_clients: usize,
}

/// Information about a conversation context
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ContextInfo {
    pub id: String,
    pub title: String,
    pub message_count: usize,
    pub last_message_at: DateTime<Utc>,
    pub status: String,
}

/// Information about a message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MessageInfo {
    pub id: u64,
    pub context_id: String,
    pub role: String,
    pub content: String,
    pub created_at: DateTime<Utc>,
    pub status: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Request to create a new context
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateContextRequest {
    pub title: Option<String>,
    pub initial_message: Option<String>,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Response when creating a context
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CreateContextResponse {
    pub context_id: String,
    pub title: String,
    pub created_at: DateTime<Utc>,
}

/// Request to add a message to a context
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddMessageRequest {
    pub content: String,
    pub role: String,
    pub metadata: Option<HashMap<String, serde_json::Value>>,
}

/// Response when adding a message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct AddMessageResponse {
    pub message_id: u64,
    pub context_id: String,
    pub created_at: DateTime<Utc>,
}

/// Configuration for sync service
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct SyncConfig {
    pub enabled: bool,
    pub helix_api_url: String,
    pub sync_interval_seconds: u64,
}

impl Default for SyncConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            helix_api_url: "http://localhost:8080".to_string(),
            sync_interval_seconds: 5,
        }
    }
}

/// Configuration for MCP integration
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpConfig {
    pub enabled: bool,
    pub server_configs: Vec<McpServerConfig>,
}

impl Default for McpConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            server_configs: Vec::new(),
        }
    }
}

/// Configuration for an MCP server
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpServerConfig {
    pub name: String,
    pub command: String,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
}

/// Wrapper for outgoing WebSocket messages that matches API's SyncMessage format
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct OutgoingMessage {
    pub event_type: String,
    pub data: serde_json::Value,
}

/// Events that Zed sends to external system via WebSocket
/// Per WEBSOCKET_PROTOCOL_SPEC.md - Zed is stateless and only knows about acp_thread_id
#[derive(Clone, Debug, Serialize, Deserialize)]
pub enum SyncEvent {
    /// Sent when Zed creates a new ACP thread (in response to Helix message)
    #[serde(rename = "thread_created")]
    ThreadCreated {
        acp_thread_id: String,
        request_id: String,
    },
    /// Sent when user creates a new thread in Zed UI (should create new Helix session)
    #[serde(rename = "user_created_thread")]
    UserCreatedThread {
        acp_thread_id: String,
        title: Option<String>,
    },
    /// Sent when thread title changes in Zed (sync to Helix session name)
    #[serde(rename = "thread_title_changed")]
    ThreadTitleChanged {
        acp_thread_id: String,
        title: String,
    },
    /// Sent while AI is streaming response (same message_id, progressively longer content)
    /// entry_type distinguishes "text" (assistant prose) from "tool_call" (tool invocation)
    #[serde(rename = "message_added")]
    MessageAdded {
        acp_thread_id: String,
        message_id: String,
        role: String,
        content: String,
        /// "text" for assistant prose, "tool_call" for tool invocations
        #[serde(default)]
        entry_type: String,
        /// For tool_call entries: the tool name (e.g. "Read file `foo.rs`")
        #[serde(default, skip_serializing_if = "String::is_empty")]
        tool_name: String,
        /// For tool_call entries: status string (e.g. "Completed", "In Progress")
        #[serde(default, skip_serializing_if = "String::is_empty")]
        tool_status: String,
        timestamp: i64,
    },

    /// Sent when AI finishes responding
    #[serde(rename = "message_completed")]
    MessageCompleted {
        acp_thread_id: String,
        message_id: String,
        request_id: String,
    },
    /// Sent when thread loading fails (e.g., session already active via UI)
    #[serde(rename = "thread_load_error")]
    ThreadLoadError {
        acp_thread_id: String,
        request_id: String,
        error: String,
    },
    /// Sent when the agent (e.g., qwen-code) has finished initialization and is ready to receive prompts
    /// This prevents race conditions where Helix sends prompts before the agent is ready
    #[serde(rename = "agent_ready")]
    AgentReady {
        /// Name of the agent that became ready (e.g., "qwen", "zed-agent")
        agent_name: String,
        /// Optional thread ID if a thread was loaded from session
        thread_id: Option<String>,
    },
    /// Response to query_ui_state command — reports current agent panel UI state
    /// Used by E2E tests to verify that threads are correctly displayed
    #[serde(rename = "ui_state_response")]
    UiStateResponse {
        /// Echoed back from the query to correlate request/response
        query_id: String,
        /// Current active view: "agent_thread", "history", "uninitialized", "agent_thread_loading", "other"
        active_view: String,
        /// Session ID of the currently displayed thread (if active_view == "agent_thread")
        thread_id: Option<String>,
        /// Number of entries in the displayed thread
        entry_count: usize,
        /// MCP context server statuses: server name -> status string ("running", "starting", "stopped", "error")
        mcp_servers: HashMap<String, String>,
        /// Currently selected model ID for the active thread (if available)
        active_model: Option<String>,
    },
}

impl SyncEvent {
    /// Convert to OutgoingMessage format expected by API
    pub fn to_outgoing_message(&self) -> Result<OutgoingMessage, serde_json::Error> {
        let (event_type, data) = match self {
            SyncEvent::ThreadCreated { acp_thread_id, request_id } => (
                "thread_created".to_string(),
                serde_json::json!({
                    "acp_thread_id": acp_thread_id,
                    "request_id": request_id,
                })
            ),
            SyncEvent::UserCreatedThread { acp_thread_id, title } => (
                "user_created_thread".to_string(),
                serde_json::json!({
                    "acp_thread_id": acp_thread_id,
                    "title": title,
                })
            ),
            SyncEvent::ThreadTitleChanged { acp_thread_id, title } => (
                "thread_title_changed".to_string(),
                serde_json::json!({
                    "acp_thread_id": acp_thread_id,
                    "title": title,
                })
            ),
            SyncEvent::MessageAdded { acp_thread_id, message_id, role, content, entry_type, tool_name, tool_status, timestamp } => (
                "message_added".to_string(),
                serde_json::json!({
                    "acp_thread_id": acp_thread_id,
                    "message_id": message_id,
                    "role": role,
                    "content": content,
                    "timestamp": timestamp,
                    "entry_type": entry_type,
                    "tool_name": tool_name,
                    "tool_status": tool_status,
                })
            ),
            SyncEvent::MessageCompleted { acp_thread_id, message_id, request_id } => (
                "message_completed".to_string(),
                serde_json::json!({
                    "acp_thread_id": acp_thread_id,
                    "message_id": message_id,
                    "request_id": request_id,
                })
            ),
            SyncEvent::ThreadLoadError { acp_thread_id, request_id, error } => (
                "thread_load_error".to_string(),
                serde_json::json!({
                    "acp_thread_id": acp_thread_id,
                    "request_id": request_id,
                    "error": error,
                })
            ),
            SyncEvent::AgentReady { agent_name, thread_id } => (
                "agent_ready".to_string(),
                serde_json::json!({
                    "agent_name": agent_name,
                    "thread_id": thread_id,
                })
            ),
            SyncEvent::UiStateResponse { query_id, active_view, thread_id, entry_count, mcp_servers, active_model } => (
                "ui_state_response".to_string(),
                serde_json::json!({
                    "query_id": query_id,
                    "active_view": active_view,
                    "thread_id": thread_id,
                    "entry_count": entry_count,
                    "mcp_servers": mcp_servers,
                    "active_model": active_model,
                })
            ),
        };

        Ok(OutgoingMessage { event_type, data })
    }
}

/// Incoming command from external system to Zed
/// Per WEBSOCKET_PROTOCOL_SPEC.md - external system sends chat_message
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct IncomingChatMessage {
    pub acp_thread_id: Option<String>,  // null = create new thread, Some(id) = use existing
    pub message: String,
    pub request_id: String,
    #[serde(default)]
    pub role: Option<String>,  // Optional role field from API (can be ignored)
    #[serde(default)]
    pub session_id: Option<String>,  // Optional session_id field from API (can be ignored)
    #[serde(default)]
    pub agent_name: Option<String>,  // Which agent to use (zed-agent or qwen) - defaults to zed-agent
}

/// Response for health check endpoint
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct HealthResponse {
    pub status: String,
    pub version: String,
    pub session_id: String,
    pub uptime_seconds: u64,
    pub active_contexts: usize,
    pub sync_clients: usize,
}

/// Error response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ErrorResponse {
    pub error: String,
    pub code: Option<String>,
    pub details: Option<HashMap<String, serde_json::Value>>,
}

/// WebSocket message types (DEPRECATED - use SyncEvent directly)
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum WebSocketMessage {
    /// Sync event from Zed to external system
    SyncEvent(SyncEvent),
    /// Ping message
    Ping { id: String },
    /// Pong response
    Pong { id: String },
    /// Error message
    Error(ErrorResponse),
    /// Subscribe to events
    Subscribe {
        events: Vec<String>,
    },
    /// Unsubscribe from events
    Unsubscribe {
        events: Vec<String>,
    },
}

/// MCP tool call request
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpToolCallRequest {
    pub tool_name: String,
    pub arguments: HashMap<String, serde_json::Value>,
    pub context_id: Option<String>,
}

/// MCP tool call response
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpToolCallResponse {
    pub success: bool,
    pub result: Option<serde_json::Value>,
    pub error: Option<String>,
}

/// Available MCP tools
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpTool {
    pub name: String,
    pub description: String,
    pub parameters: serde_json::Value, // JSON Schema
    pub server: String,
}

/// List of available MCP tools
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct McpToolsResponse {
    pub tools: Vec<McpTool>,
}

/// Stream response for real-time updates
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct StreamResponse<T> {
    pub id: String,
    pub data: T,
    pub timestamp: DateTime<Utc>,
    pub sequence: u64,
}

/// Conversation thread summary for Helix sync
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThreadSummary {
    pub thread_id: String,
    pub title: String,
    pub message_count: usize,
    pub last_message_at: DateTime<Utc>,
    pub participants: Vec<String>,
    pub status: String,
    pub metadata: HashMap<String, serde_json::Value>,
}

/// Full thread data for initial sync
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ThreadData {
    pub thread_id: String,
    pub title: String,
    pub messages: Vec<MessageInfo>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub metadata: HashMap<String, serde_json::Value>,
}