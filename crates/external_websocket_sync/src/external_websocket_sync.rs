//! External WebSocket Thread Sync for Zed Editor
//! 
//! This crate provides APIs for synchronizing Zed editor conversation threads
//! with external services via WebSocket connections, enabling real-time collaboration
//! and integration with AI platforms and other external tools.

use anyhow::{Context, Result};
use assistant_text_thread::{TextThread, TextThreadId, TextThreadStore, MessageId};
use assistant_slash_command::SlashCommandWorkingSet;
use clock::ReplicaId;
use collections::HashMap;
use gpui::{App, Entity, EventEmitter, Global, Subscription, Task};
use tokio::sync::mpsc;

use language_model;
use parking_lot::RwLock;
use project::Project;
use prompt_store::PromptBuilder;
use serde::{Deserialize, Serialize};
use session::AppSession;
use std::sync::Arc;

mod websocket_sync;

mod thread_service;
pub use thread_service::*;

mod types;
pub use types::{ExternalAgent, *};

// mod sync;
// pub use sync::*;

mod mcp;
pub use mcp::*;

mod sync_settings;
pub use sync_settings::*;

// mod server;
// pub use server::*;

pub use websocket_sync::*;
pub use tungstenite;

/// Type alias for backwards compatibility with existing code
pub type ContextId = TextThreadId;

/// Global WebSocket sender for sending responses back to external system
#[derive(Clone)]
pub struct WebSocketSender {
    pub sender: Arc<RwLock<Option<tokio::sync::mpsc::UnboundedSender<tungstenite::Message>>>>,
}

impl Default for WebSocketSender {
    fn default() -> Self {
        Self {
            sender: Arc::new(RwLock::new(None)),
        }
    }
}

impl Global for WebSocketSender {}

/// Static global for thread creation callback
static GLOBAL_THREAD_CREATION_CALLBACK: parking_lot::Mutex<Option<mpsc::UnboundedSender<ThreadCreationRequest>>> =
    parking_lot::Mutex::new(None);

/// Pending thread creation requests that arrived before callback was initialized
/// These get replayed when init_thread_creation_callback is called
static PENDING_THREAD_CREATION_REQUESTS: parking_lot::Mutex<Vec<ThreadCreationRequest>> =
    parking_lot::Mutex::new(Vec::new());

/// Pending thread open requests that arrived before callback was initialized
static PENDING_THREAD_OPEN_REQUESTS: parking_lot::Mutex<Vec<ThreadOpenRequest>> =
    parking_lot::Mutex::new(Vec::new());

/// Static global for thread display callback (notifies AgentPanel to auto-select thread)
static GLOBAL_THREAD_DISPLAY_CALLBACK: parking_lot::Mutex<Option<mpsc::UnboundedSender<ThreadDisplayNotification>>> =
    parking_lot::Mutex::new(None);

/// Pending thread display notifications that arrived before AgentPanel was ready
/// These get replayed when init_thread_display_callback is called
static PENDING_THREAD_DISPLAY_NOTIFICATIONS: parking_lot::Mutex<Vec<ThreadDisplayNotification>> =
    parking_lot::Mutex::new(Vec::new());

/// Static global for thread open callback (loads thread from database and displays it)
static GLOBAL_THREAD_OPEN_CALLBACK: parking_lot::Mutex<Option<mpsc::UnboundedSender<ThreadOpenRequest>>> =
    parking_lot::Mutex::new(None);

/// Static global for UI state query callback (queries AgentPanel's active view for E2E testing)
static GLOBAL_UI_STATE_QUERY_CALLBACK: parking_lot::Mutex<Option<mpsc::UnboundedSender<UiStateQueryRequest>>> =
    parking_lot::Mutex::new(None);

/// Pending UI state queries that arrived before AgentPanel was ready
static PENDING_UI_STATE_QUERIES: parking_lot::Mutex<Vec<UiStateQueryRequest>> =
    parking_lot::Mutex::new(Vec::new());

/// Request to create ACP thread from external WebSocket message
#[derive(Clone, Debug)]
pub struct ThreadCreationRequest {
    pub acp_thread_id: Option<String>, // null = create new, Some(id) = use existing
    pub message: String,
    pub request_id: String,
    pub agent_name: Option<String>, // Which agent to use (zed-agent or qwen) - defaults to zed-agent
    /// When true, don't mark the entry as external-originated.
    /// This allows the NewEntry subscription to fire and sync the user message back to Helix,
    /// simulating a user typing directly in Zed's agent panel.
    pub simulate_input: bool,
}

/// Request to open existing ACP thread from database and display in UI
#[derive(Clone, Debug)]
pub struct ThreadOpenRequest {
    pub acp_thread_id: String,
    /// Which ACP agent to use (e.g., "qwen", "claude", "gemini", "codex").
    /// None or empty means use NativeAgent (Zed's built-in agent).
    pub agent_name: Option<String>,
}

/// Request to query UI state from AgentPanel (for E2E testing)
#[derive(Clone, Debug)]
pub struct UiStateQueryRequest {
    pub query_id: String,
}

/// Notification to display a thread in AgentPanel (for auto-select)
#[derive(Clone, Debug)]
pub struct ThreadDisplayNotification {
    pub thread_entity: gpui::Entity<acp_thread::AcpThread>,
    pub helix_session_id: String,
    pub agent_name: Option<String>, // Which agent created this thread (e.g., "qwen") - None means Zed Agent
}

/// Global callback for thread creation from WebSocket (set by agent_panel)
#[derive(Clone)]
pub struct ThreadCreationCallback {
    pub sender: mpsc::UnboundedSender<ThreadCreationRequest>,
}

impl Global for ThreadCreationCallback {}

/// Send thread creation request to agent_panel via callback
/// If callback is not yet initialized (Zed restart race condition), queue the request
/// and replay when init_thread_creation_callback is called
pub fn request_thread_creation(request: ThreadCreationRequest) -> Result<()> {
    log::info!("🔧 [CALLBACK] request_thread_creation() called: acp_thread_id={:?}, request_id={}",
               request.acp_thread_id, request.request_id);
    eprintln!("🔧 [CALLBACK] request_thread_creation() called: acp_thread_id={:?}, request_id={}",
               request.acp_thread_id, request.request_id);

    let sender = GLOBAL_THREAD_CREATION_CALLBACK.lock().clone();
    if let Some(sender) = sender {
        log::info!("✅ [CALLBACK] Found global callback sender, sending request...");
        sender.send(request)
            .map_err(|e| {
                log::error!("❌ [CALLBACK] Failed to send to channel: {:?}", e);
                anyhow::anyhow!("Failed to send thread creation request")
            })?;
        log::info!("✅ [CALLBACK] Request sent to callback channel successfully");
        Ok(())
    } else {
        // Queue the request for later - this handles Zed restart race condition
        // where WebSocket reconnects before workspace/thread_service is initialized
        log::warn!("⏳ [CALLBACK] Thread creation callback not yet initialized - queueing request for later replay");
        eprintln!("⏳ [CALLBACK] Thread creation callback not yet initialized - queueing request_id={} for later replay", request.request_id);
        PENDING_THREAD_CREATION_REQUESTS.lock().push(request);
        log::info!("✅ [CALLBACK] Request queued successfully (will replay when callback is registered)");
        eprintln!("✅ [CALLBACK] Request queued successfully (will replay when callback is registered)");
        Ok(())
    }
}

/// Initialize the global callback sender (called from thread_service or tests)
/// Also replays any pending requests that arrived before callback was initialized
pub fn init_thread_creation_callback(sender: mpsc::UnboundedSender<ThreadCreationRequest>) {
    log::info!("🔧 [CALLBACK] init_thread_creation_callback() called - registering global callback");
    eprintln!("🔧 [CALLBACK] init_thread_creation_callback() called - registering global callback");

    // Store the callback
    *GLOBAL_THREAD_CREATION_CALLBACK.lock() = Some(sender.clone());
    log::info!("✅ [CALLBACK] Global thread creation callback registered");
    eprintln!("✅ [CALLBACK] Global thread creation callback registered");

    // Replay any pending requests that arrived before callback was ready
    let pending: Vec<ThreadCreationRequest> = std::mem::take(&mut *PENDING_THREAD_CREATION_REQUESTS.lock());
    if !pending.is_empty() {
        log::info!("🔄 [CALLBACK] Replaying {} pending thread creation requests", pending.len());
        eprintln!("🔄 [CALLBACK] Replaying {} pending thread creation requests", pending.len());
        for request in pending {
            log::info!("🔄 [CALLBACK] Replaying request_id={}", request.request_id);
            eprintln!("🔄 [CALLBACK] Replaying request_id={}", request.request_id);
            if let Err(e) = sender.send(request) {
                log::error!("❌ [CALLBACK] Failed to replay pending request: {:?}", e);
                eprintln!("❌ [CALLBACK] Failed to replay pending request: {:?}", e);
            }
        }
        log::info!("✅ [CALLBACK] Finished replaying pending requests");
        eprintln!("✅ [CALLBACK] Finished replaying pending requests");
    }
}

/// Initialize the global thread display callback (called from agent_panel)
/// Also replays any pending notifications that arrived before AgentPanel was ready
pub fn init_thread_display_callback(sender: mpsc::UnboundedSender<ThreadDisplayNotification>) {
    log::info!("🔧 [CALLBACK] init_thread_display_callback() called - registering global callback");
    eprintln!("🔧 [CALLBACK] init_thread_display_callback() called - registering global callback");

    // Store the callback
    *GLOBAL_THREAD_DISPLAY_CALLBACK.lock() = Some(sender.clone());
    log::info!("✅ [CALLBACK] Global thread display callback registered");
    eprintln!("✅ [CALLBACK] Global thread display callback registered");

    // Replay any pending notifications that arrived before AgentPanel was ready
    let pending: Vec<ThreadDisplayNotification> = std::mem::take(&mut *PENDING_THREAD_DISPLAY_NOTIFICATIONS.lock());
    if !pending.is_empty() {
        log::info!("🔄 [CALLBACK] Replaying {} pending thread display notifications", pending.len());
        eprintln!("🔄 [CALLBACK] Replaying {} pending thread display notifications", pending.len());
        for notification in pending {
            log::info!("🔄 [CALLBACK] Replaying display notification for session: {}", notification.helix_session_id);
            eprintln!("🔄 [CALLBACK] Replaying display notification for session: {}", notification.helix_session_id);
            if let Err(e) = sender.send(notification) {
                log::error!("❌ [CALLBACK] Failed to replay display notification: {:?}", e);
                eprintln!("❌ [CALLBACK] Failed to replay display notification: {:?}", e);
            }
        }
        log::info!("✅ [CALLBACK] Finished replaying pending display notifications");
        eprintln!("✅ [CALLBACK] Finished replaying pending display notifications");
    }
}

/// Initialize the global thread open callback (called from thread_service)
/// Also replays any pending requests that arrived before callback was initialized
pub fn init_thread_open_callback(sender: mpsc::UnboundedSender<ThreadOpenRequest>) {
    log::info!("🔧 [CALLBACK] init_thread_open_callback() called - registering global callback");
    eprintln!("🔧 [CALLBACK] init_thread_open_callback() called - registering global callback");

    // Store the callback
    *GLOBAL_THREAD_OPEN_CALLBACK.lock() = Some(sender.clone());
    log::info!("✅ [CALLBACK] Global thread open callback registered");
    eprintln!("✅ [CALLBACK] Global thread open callback registered");

    // Replay any pending requests that arrived before callback was ready
    let pending: Vec<ThreadOpenRequest> = std::mem::take(&mut *PENDING_THREAD_OPEN_REQUESTS.lock());
    if !pending.is_empty() {
        log::info!("🔄 [CALLBACK] Replaying {} pending thread open requests", pending.len());
        eprintln!("🔄 [CALLBACK] Replaying {} pending thread open requests", pending.len());
        for request in pending {
            log::info!("🔄 [CALLBACK] Replaying thread open for acp_thread_id={}", request.acp_thread_id);
            eprintln!("🔄 [CALLBACK] Replaying thread open for acp_thread_id={}", request.acp_thread_id);
            if let Err(e) = sender.send(request) {
                log::error!("❌ [CALLBACK] Failed to replay pending open request: {:?}", e);
                eprintln!("❌ [CALLBACK] Failed to replay pending open request: {:?}", e);
            }
        }
        log::info!("✅ [CALLBACK] Finished replaying pending open requests");
        eprintln!("✅ [CALLBACK] Finished replaying pending open requests");
    }
}

/// Request opening a thread (called from WebSocket handler)
/// If callback is not yet initialized (Zed restart race condition), queue the request
pub fn request_thread_open(request: ThreadOpenRequest) -> Result<()> {
    log::info!("🔧 [CALLBACK] request_thread_open() called: acp_thread_id={}", request.acp_thread_id);
    eprintln!("🔧 [CALLBACK] request_thread_open() called: acp_thread_id={}", request.acp_thread_id);

    let sender = GLOBAL_THREAD_OPEN_CALLBACK.lock().clone();
    if let Some(sender) = sender {
        log::info!("✅ [CALLBACK] Found global callback sender, sending request...");
        sender.send(request)
            .map_err(|e| {
                log::error!("❌ [CALLBACK] Failed to send to channel: {:?}", e);
                anyhow::anyhow!("Failed to send thread open request")
            })?;
        log::info!("✅ [CALLBACK] Request sent to callback channel successfully");
        Ok(())
    } else {
        // Queue the request for later - this handles Zed restart race condition
        log::warn!("⏳ [CALLBACK] Thread open callback not yet initialized - queueing request for later replay");
        eprintln!("⏳ [CALLBACK] Thread open callback not yet initialized - queueing acp_thread_id={} for later replay", request.acp_thread_id);
        PENDING_THREAD_OPEN_REQUESTS.lock().push(request);
        log::info!("✅ [CALLBACK] Open request queued successfully (will replay when callback is registered)");
        eprintln!("✅ [CALLBACK] Open request queued successfully (will replay when callback is registered)");
        Ok(())
    }
}

/// Initialize the global UI state query callback (called from agent_panel)
/// Also replays any pending queries that arrived before AgentPanel was ready
pub fn init_ui_state_query_callback(sender: mpsc::UnboundedSender<UiStateQueryRequest>) {
    log::info!("🔧 [CALLBACK] init_ui_state_query_callback() called - registering global callback");
    eprintln!("🔧 [CALLBACK] init_ui_state_query_callback() called - registering global callback");

    *GLOBAL_UI_STATE_QUERY_CALLBACK.lock() = Some(sender.clone());

    let pending: Vec<UiStateQueryRequest> = std::mem::take(&mut *PENDING_UI_STATE_QUERIES.lock());
    if !pending.is_empty() {
        log::info!("🔄 [CALLBACK] Replaying {} pending UI state queries", pending.len());
        for request in pending {
            if let Err(e) = sender.send(request) {
                log::error!("❌ [CALLBACK] Failed to replay UI state query: {:?}", e);
            }
        }
    }
}

/// Request UI state from AgentPanel (called from WebSocket handler)
/// If AgentPanel isn't ready yet, the query is queued for later replay
pub fn request_ui_state_query(request: UiStateQueryRequest) -> Result<()> {
    log::info!("🔧 [CALLBACK] request_ui_state_query() called: query_id={}", request.query_id);
    eprintln!("🔧 [CALLBACK] request_ui_state_query() called: query_id={}", request.query_id);

    let sender = GLOBAL_UI_STATE_QUERY_CALLBACK.lock().clone();
    if let Some(sender) = sender {
        sender.send(request)
            .map_err(|_| anyhow::anyhow!("Failed to send UI state query"))?;
        Ok(())
    } else {
        PENDING_UI_STATE_QUERIES.lock().push(request);
        Ok(())
    }
}

/// Notify AgentPanel to display a thread (for auto-select)
/// If AgentPanel isn't ready yet, the notification is queued for later replay
pub fn notify_thread_display(notification: ThreadDisplayNotification) -> Result<()> {
    log::info!("🔧 [CALLBACK] notify_thread_display() called for session: {}", notification.helix_session_id);
    eprintln!("🔧 [CALLBACK] notify_thread_display() called for session: {}", notification.helix_session_id);

    let sender = GLOBAL_THREAD_DISPLAY_CALLBACK.lock().clone();
    if let Some(sender) = sender {
        log::info!("✅ [CALLBACK] Found global display callback sender, sending notification...");
        eprintln!("✅ [CALLBACK] Found global display callback sender, sending notification...");
        sender.send(notification)
            .map_err(|e| {
                log::error!("❌ [CALLBACK] Failed to send to channel: {:?}", e);
                eprintln!("❌ [CALLBACK] Failed to send to channel: {:?}", e);
                anyhow::anyhow!("Failed to send thread display notification")
            })?;
        log::info!("✅ [CALLBACK] Notification sent to callback channel successfully");
        eprintln!("✅ [CALLBACK] Notification sent to callback channel successfully");
        Ok(())
    } else {
        // Queue the notification for later replay when AgentPanel initializes
        log::warn!("⏳ [CALLBACK] Thread display callback not yet initialized - queueing notification for later replay");
        eprintln!("⏳ [CALLBACK] Thread display callback not yet initialized - queueing session={} for later replay", notification.helix_session_id);
        PENDING_THREAD_DISPLAY_NOTIFICATIONS.lock().push(notification);
        log::info!("✅ [CALLBACK] Display notification queued successfully (will replay when AgentPanel is ready)");
        eprintln!("✅ [CALLBACK] Display notification queued successfully (will replay when AgentPanel is ready)");
        Ok(())
    }
}

/// Type alias for compatibility with existing code
pub type HelixIntegration = ExternalWebSocketSync;

/// Main external WebSocket thread sync service
pub struct ExternalWebSocketSync {
    session: Arc<AppSession>,
    context_store: Option<Entity<TextThreadStore>>,
    project: Entity<Project>,
    prompt_builder: Arc<PromptBuilder>,
    active_contexts: Arc<RwLock<HashMap<TextThreadId, Entity<TextThread>>>>,
    websocket_sync: Option<WebSocketSync>,
    sync_clients: Arc<RwLock<Vec<String>>>,
    _subscriptions: Vec<Subscription>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ExternalSyncConfig {
    /// Enable external WebSocket sync
    pub enabled: bool,
    /// WebSocket sync configuration
    pub websocket_sync: WebSocketSyncConfig,
    /// MCP configuration
    pub mcp: McpConfig,
}

impl Default for ExternalSyncConfig {
    fn default() -> Self {
        Self {
            enabled: false,
            websocket_sync: WebSocketSyncConfig::default(),
            mcp: McpConfig::default(),
        }
    }
}

impl ExternalWebSocketSync {
    /// Initialize external WebSocket sync with project and prompt builder
    pub fn init_with_project(
        app: &mut App, 
        session: Arc<AppSession>, 
        project: Entity<Project>,
        prompt_builder: Arc<PromptBuilder>
    ) {
        let sync_service = Self::new(session, project, prompt_builder);
        app.set_global(sync_service);
        
        // Initialize sync service synchronously for now
        log::info!("External WebSocket sync initialized - async startup deferred");
    }

    /// Create new external WebSocket sync instance
    pub fn new(session: Arc<AppSession>, project: Entity<Project>, prompt_builder: Arc<PromptBuilder>) -> Self {
        log::warn!("External WebSocket sync creation - using placeholder values due to API changes");
        Self {
            session,
            context_store: None,
            project,
            prompt_builder,
            active_contexts: Arc::new(RwLock::new(HashMap::default())),
            websocket_sync: None,
            sync_clients: Arc::new(RwLock::new(Vec::new())),
            _subscriptions: Vec::new(),
        }
    }

    /// Subscribe to context changes and emit sync events to Helix
    pub fn subscribe_to_text_thread_changes(&mut self, text_thread: Entity<TextThread>, external_session_id: String, cx: &mut App) {
        let session_id_clone = external_session_id.clone();
        eprintln!("🔔 [SYNC] Subscribing to text thread changes for external session: {}", external_session_id);

        let subscription = cx.subscribe(&text_thread, move |_text_thread, _event, _cx| {
            eprintln!("🔔 [SYNC] Text thread changed for session: {}", session_id_clone);

            // TODO: Extract text thread content and send sync event to Helix
            eprintln!("✅ [SYNC] Sending text thread update to Helix for session: {}", session_id_clone);
            // This is where we'll implement the sync back to Helix
            // We'll send the complete thread state back to Helix
        });

        self._subscriptions.push(subscription);
        eprintln!("✅ [SYNC] Successfully subscribed to text thread changes for session: {}", external_session_id);
    }

    /// Initialize context store with project
    pub async fn init_context_store(&mut self, cx: &mut App) -> Result<()> {
        if self.context_store.is_some() {
            return Ok(()); // Already initialized
        }

        let project = self.project.clone();
        let prompt_builder = self.prompt_builder.clone();
        // let slash_commands = Arc::new(SlashCommandWorkingSet::default());

        log::info!("Initializing text thread store for Helix integration");

        let slash_commands = Arc::new(SlashCommandWorkingSet::default());
        let text_thread_store = TextThreadStore::new(project, prompt_builder, slash_commands, cx).await?;
        self.context_store = Some(text_thread_store);
        log::info!("Text thread store initialized successfully");
        Ok(())
    }

    /// Initialize text thread store and return a task
    pub fn init_text_thread_store_task(&self, cx: &mut App) -> Task<Result<Entity<TextThreadStore>>> {
        let project = self.project.clone();
        let prompt_builder = self.prompt_builder.clone();

        log::info!("Initializing text thread store for Helix integration");

        let slash_commands = Arc::new(SlashCommandWorkingSet::default());
        TextThreadStore::new(project, prompt_builder, slash_commands, cx)
    }

    /// Start the sync service (DEPRECATED - use init_websocket_service instead)
    #[allow(dead_code)]
    pub async fn start(&mut self, _cx: &mut App) -> Result<()> {
        log::warn!("ExternalWebSocketSync::start() is deprecated - use websocket_sync::init_websocket_service()");
        Ok(())
    }

    /// Stop the sync service
    pub async fn stop(&self) -> Result<()> {
        log::info!("Stopping external WebSocket sync service");

        // Note: This would need proper mutable access in a real implementation
        // For now, just log the stop request
        log::info!("Stop requested for external WebSocket sync service");

        // Clear active contexts
        self.active_contexts.write().clear();

        log::info!("External WebSocket sync service stopped");
        Ok(())
    }

    /// Load configuration from settings
    fn load_config(&self, cx: &App) -> Option<ExternalSyncConfig> {
        let settings = ExternalSyncSettings::get_global(cx);
        
        Some(ExternalSyncConfig {
            enabled: settings.enabled,
            websocket_sync: WebSocketSyncConfig {
                enabled: settings.websocket_sync.enabled,
                url: settings.websocket_sync.external_url.clone(),
                auth_token: settings.websocket_sync.auth_token.clone().unwrap_or_default(),
                use_tls: settings.websocket_sync.use_tls,
                skip_tls_verify: settings.websocket_sync.skip_tls_verify,
            },
            mcp: McpConfig {
                enabled: settings.mcp.enabled,
                server_configs: settings.mcp.servers.iter().map(|s| crate::types::McpServerConfig {
                    name: s.name.clone(),
                    command: s.command.clone(),
                    args: s.args.clone(),
                    env: s.env.clone(),
                }).collect(),
            },
        })
    }

    /// Get session information
    pub fn get_session_info(&self) -> SessionInfo {
        SessionInfo {
            session_id: self.session.id().to_string(),
            last_session_id: self.session.last_session_id().map(|s| s.to_string()),
            active_contexts: self.active_contexts.read().len(),
            websocket_connected: false, // TODO: check if websocket service is active
            sync_clients: self.sync_clients.read().len(),
        }
    }

    /// Get all active conversation contexts
    pub fn get_contexts(&self) -> Vec<ContextInfo> {
        let contexts = self.active_contexts.read();
        contexts
            .iter()
            .map(|(id, _context)| {
                // TODO: Read actual context data once assistant_context integration is complete
                ContextInfo {
                    id: id.to_proto(),
                    title: "Conversation".to_string(),
                    message_count: 0,
                    last_message_at: chrono::Utc::now(),
                    status: "active".to_string(),
                }
            })
            .collect()
    }

    /// Create a new conversation context
    pub fn create_context(&self, title: Option<String>, _cx: &mut App) -> Result<ContextId> {
        let context_id = ContextId::new();
        
        // TODO: Implement real context creation when API is fixed
        // For now, just create a placeholder context ID
        log::info!("Created new conversation context: {} ({})", 
                  context_id.to_proto(), 
                  title.as_deref().unwrap_or("Untitled"));
        
        Ok(context_id)

    }

    /// Delete a conversation context
    pub fn delete_context(&mut self, context_id: &ContextId) -> Result<()> {
        if self.active_contexts.write().remove(context_id).is_some() {
            self.notify_context_deleted(context_id);
            Ok(())
        } else {
            anyhow::bail!("Context not found: {}", context_id.to_proto())
        }
    }

    /// Get messages from a context
    pub fn get_context_messages(&self, context_id: &ContextId, cx: &App) -> Result<Vec<MessageInfo>> {
        let contexts = self.active_contexts.read();
        let context = contexts
            .get(context_id)
            .with_context(|| format!("Context not found: {}", context_id.to_proto()))?;

        // Read actual messages from the assistant context
        let messages: Vec<MessageInfo> = context.read(cx).messages(cx)
            .map(|message| MessageInfo {
                id: message.id.as_u64(),
                context_id: context_id.to_proto(),
                role: match message.role {
                    language_model::Role::User => "user".to_string(),
                    language_model::Role::Assistant => "assistant".to_string(),
                    language_model::Role::System => "system".to_string(),
                },
                content: "placeholder message content".to_string(), // TODO: Get actual message content from buffer
                created_at: chrono::Utc::now(), // TODO: Get actual timestamp from message
                status: match message.status {
                    assistant_text_thread::MessageStatus::Pending => "pending".to_string(),
                    assistant_text_thread::MessageStatus::Done => "done".to_string(),
                    assistant_text_thread::MessageStatus::Error(_) => "error".to_string(),
                    assistant_text_thread::MessageStatus::Canceled => "canceled".to_string(),
                },
                metadata: std::collections::HashMap::new(),
            })
            .collect();

        Ok(messages)
    }

    /// Add a message to a context
    pub fn add_message_to_context(
        &mut self,
        context_id: &ContextId,
        content: String,
        role: String,
        cx: &mut App,
    ) -> Result<MessageId> {
        let contexts = self.active_contexts.read();
        let context = contexts
            .get(context_id)
            .with_context(|| format!("Context not found: {}", context_id.to_proto()))?
            .clone();

        drop(contexts);

        // Add the actual message to the assistant context
        let message_id = context.update(cx, |context, cx| {
            // Add the user's message to the buffer (same as agent panel does)
            context.buffer().update(cx, |buffer, cx| {
                let end_offset = buffer.len();
                buffer.edit([(end_offset..end_offset, format!("{}\n", content))], None, cx);
            });

            // If this is a user message, trigger AI assistant response
            if role == "user" {
                log::info!("🤖 [WEBSOCKET_SYNC] Triggering AI assistant for user message: {}", content);
                context.assist(cx);
            }

            // Create a message ID (for now just use a placeholder)
            MessageId(clock::Lamport::new(ReplicaId::new(1)))
        });

        // Notify via WebSocket
        self.notify_message_added(context_id, &message_id);

        Ok(message_id)
    }

    /// Notify WebSocket of context creation (DEPRECATED - use thread_created)
    #[allow(dead_code)]
    fn notify_context_created(&self, _context_id: &ContextId) {
        // Context creation now sends thread_created event via the callback mechanism
    }

    /// Notify WebSocket of context deletion (DEPRECATED)
    #[allow(dead_code)]
    fn notify_context_deleted(&self, _context_id: &ContextId) {
        // Not part of the simplified protocol
    }

    /// Notify WebSocket of message addition
    fn notify_message_added(&self, context_id: &ContextId, message_id: &MessageId) {
        if let Some(websocket_sync) = &self.websocket_sync {
            let event = SyncEvent::MessageAdded {
                acp_thread_id: context_id.to_proto(),
                message_id: message_id.as_u64().to_string(),
                role: "assistant".to_string(),
                content: String::new(), // TODO: get actual content
                timestamp: chrono::Utc::now().timestamp(),
            };
            if let Err(e) = websocket_sync.send_event(event) {
                log::warn!("Failed to send message added event: {}", e);
            }
        }
    }

    /// Notify WebSocket of message completion
    pub fn notify_message_completed(&self, context_id: &ContextId, message_id: &MessageId) {
        if let Some(websocket_sync) = &self.websocket_sync {
            // Flush any pending throttled messages before completion
            flush_streaming_throttle(&context_id.to_proto());

            let event = SyncEvent::MessageCompleted {
                acp_thread_id: context_id.to_proto(),
                message_id: message_id.as_u64().to_string(),
                request_id: String::new(), // TODO: track request_id
            };
            if let Err(e) = websocket_sync.send_event(event) {
                log::warn!("Failed to send message completed event: {}", e);
            }
        }
    }
}

impl EventEmitter<ExternalSyncEvent> for ExternalWebSocketSync {}

impl Global for ExternalWebSocketSync {}

/// Events emitted by the external WebSocket sync
#[derive(Clone, Debug)]
pub enum ExternalSyncEvent {
    ContextCreated { context_id: String },
    ContextDeleted { context_id: String },
    MessageAdded { context_id: String, message_id: u64 },
    SyncClientConnected { client_id: String },
    SyncClientDisconnected { client_id: String },
    /// External system requests thread creation (e.g., Helix sends first message)
    ExternalThreadCreationRequested {
        helix_session_id: String,
        message: String,
        request_id: String,
    },
    /// Thread was created and should be displayed in UI (e.g., response to Helix message)
    ThreadCreatedForDisplay {
        thread_entity: gpui::Entity<acp_thread::AcpThread>,
        helix_session_id: String,
    },
}

/// Global access to external WebSocket sync
impl ExternalWebSocketSync {
    pub fn global(cx: &App) -> Option<&Self> {
        cx.try_global::<Self>()
    }

    pub fn global_mut(cx: &mut App) -> Option<&mut Self> {
        if cx.has_global::<Self>() {
            Some(cx.global_mut::<Self>())
        } else {
            log::error!("⚠️ [EXTERNAL_WEBSOCKET_SYNC] ExternalWebSocketSync global not available for mutable access");
            None
        }
    }
}

/// Initialize the external WebSocket sync module
pub fn init(cx: &mut App) {
    log::info!("Initializing external WebSocket sync module");

    // Initialize settings
    sync_settings::init(cx);

    // Create global WebSocket sender
    cx.set_global(WebSocketSender::default());

    // TODO: Auto-start WebSocket service when enabled
    // Currently disabled because tokio_tungstenite requires Tokio runtime
    // which isn't available during GPUI init. Need to either:
    // 1. Use smol-based WebSocket library (compatible with GPUI), or
    // 2. Create Tokio runtime wrapper, or
    // 3. Start WebSocket from workspace creation (has executor context)
    //
    // For now: WebSocket must be started manually via init_websocket_service()
    // or will be started when first workspace is created (if we add that)

    let settings = ExternalSyncSettings::get_global(cx);
    if settings.enabled && settings.websocket_sync.enabled {
        log::warn!("⚠️  [WEBSOCKET] WebSocket sync enabled in settings but auto-start not yet supported");
        log::warn!("⚠️  [WEBSOCKET] WebSocket requires Tokio runtime - incompatible with GPUI init");
        log::warn!("⚠️  [WEBSOCKET] Will start when workspace is created (has executor)");
    } else {
        log::info!("⚠️  [WEBSOCKET] WebSocket sync disabled in settings");
    }

    log::info!("External WebSocket sync module initialization completed");
}


/// Initialize the external WebSocket sync with full assistant support (DEPRECATED)
#[allow(dead_code)]
pub fn init_full(
    _session: Arc<AppSession>,
    _project: Entity<Project>,
    _prompt_builder: Arc<PromptBuilder>,
    _cx: &mut App
) -> Result<()> {
    log::warn!("init_full() is deprecated - use websocket_sync::init_websocket_service()");
    Ok(())
}

/// Initialize with session and prompt builder, store for later use
pub async fn init_with_session(
    _session: Arc<AppSession>,
    _prompt_builder: Arc<PromptBuilder>,
) -> Result<()> {
    log::info!("Session and prompt builder will be passed directly to initialization methods");
    Ok(())
}

/// Initialize with project when available (DEPRECATED)
#[allow(dead_code)]
pub fn init_with_project_when_available(
    _project: Entity<Project>,
    _session: Arc<AppSession>,
    _prompt_builder: Arc<PromptBuilder>,
    _cx: &mut App
) -> Result<()> {
    log::warn!("init_with_project_when_available() is deprecated");
    Ok(())
}

/// Get the global external WebSocket sync instance
pub fn get_global_sync_service(cx: &App) -> Option<&ExternalWebSocketSync> {
    cx.try_global::<ExternalWebSocketSync>()
}

#[cfg(any(test, feature = "test-support"))]
pub mod mock_helix_server;

#[cfg(test)]
mod protocol_test;

/// Execute a function with the global sync service if available
pub fn with_sync_service<T>(
    cx: &App,
    f: impl FnOnce(&ExternalWebSocketSync) -> T,
) -> Option<T> {
    get_global_sync_service(cx).map(f)
}

/// Execute an async function with the global sync service if available
pub async fn with_sync_service_async<T>(
    cx: &App,
    f: impl FnOnce(&ExternalWebSocketSync) -> T,
) -> Option<T> {
    get_global_sync_service(cx).map(f)
}

/// Subscribe to context changes for an external session (called from AgentPanel)
pub fn subscribe_to_context_changes_global(context: Entity<TextThread>, external_session_id: String, cx: &mut App) {
    let session_id_clone = external_session_id.clone();
    eprintln!("🔔 [SYNC_GLOBAL] Setting up global context subscription for session: {}", external_session_id);
    
    // Create a subscription that will send sync events when the context changes
    let _subscription = cx.subscribe(&context, move |context_entity, _event, cx| {
        eprintln!("🔔 [SYNC_GLOBAL] Context changed for session: {}", session_id_clone);
        
        // Extract the current context content
        let context_content = context_entity.read(cx);
        let messages = context_content.messages(cx);
        
        let messages: Vec<_> = messages.collect();
        eprintln!("🔔 [SYNC_GLOBAL] Context has {} messages, syncing to Helix...", messages.len());
        
        // TODO: Send sync event to Helix via WebSocket
        // This is where we'll implement the actual sync back to Helix
        // We need to:
        // 1. Extract all messages from the context
        // 2. Format them as Helix-compatible messages
        // 3. Send them via WebSocket to update the Helix session
        
        eprintln!("✅ [SYNC_GLOBAL] Context sync completed for session: {}", session_id_clone);
    });
    
    // Store the subscription in global state so it doesn't get dropped
    // TODO: We need a way to store these subscriptions globally
    eprintln!("✅ [SYNC_GLOBAL] Context subscription created for session: {}", external_session_id);
}


