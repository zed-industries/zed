//! Thread management service for WebSocket integration
//!
//! This is the NON-UI service layer that manages ACP threads for external WebSocket control.
//! Called from workspace creation, contains all business logic.

use anyhow::Result;
use acp_thread::{AcpThread, AcpThreadEvent};
use agent::ThreadStore;
use agent_client_protocol::{self as acp, ContentBlock, TextContent};
use util::path_list::PathList;
use gpui::{App, Entity, WeakEntity};
use parking_lot::RwLock;
use std::collections::{HashMap, HashSet};
use std::sync::Arc;
use std::time::{Duration, Instant};
use fs::Fs;
use project::Project;
use tokio::sync::mpsc;
use util::ResultExt;
use crate::{ExternalAgent, ThreadCreationRequest, ThreadOpenRequest, SyncEvent};

/// Global registry of active ACP threads (service layer)
/// Stores STRONG references to keep threads alive for follow-up messages
static THREAD_REGISTRY: parking_lot::Mutex<Option<Arc<RwLock<HashMap<String, Entity<AcpThread>>>>>> =
    parking_lot::Mutex::new(None);

/// Keeps strong references to ALL threads ever created/loaded, preventing them
/// from being released when the UI switches to a different thread. Unlike
/// THREAD_REGISTRY (which gets cleaned up by unregister_thread on UI transitions),
/// this map is append-only. This ensures follow-up messages to non-visible threads
/// can find the thread entity without needing load_session.
static THREAD_KEEP_ALIVE: parking_lot::Mutex<Option<Arc<RwLock<HashMap<String, Entity<AcpThread>>>>>> =
    parking_lot::Mutex::new(None);

/// Global map of acp_thread_id -> agent_session_id
/// The agent (e.g. Claude Code) uses its own session IDs that differ from Zed's thread UUIDs.
/// We store this mapping when a thread is created so we can pass the correct session ID
/// to load_session when reloading a thread that was unloaded.
static THREAD_AGENT_SESSION_MAP: parking_lot::Mutex<Option<Arc<RwLock<HashMap<String, String>>>>> =
    parking_lot::Mutex::new(None);

/// Global map of thread_id -> current_request_id
/// Tracks the request_id for the CURRENT/LATEST message being processed by each thread
/// This ensures message_completed events use the correct request_id (not the first one)
static THREAD_REQUEST_MAP: parking_lot::Mutex<Option<Arc<RwLock<HashMap<String, String>>>>> =
    parking_lot::Mutex::new(None);

/// Global map of thread_id -> Set of entry indices that originated from external system
/// Prevents echoing external messages back (initial + follow-ups)
static EXTERNAL_ORIGINATED_ENTRIES: parking_lot::Mutex<Option<Arc<RwLock<HashMap<String, HashSet<usize>>>>>> =
    parking_lot::Mutex::new(None);

/// Set of thread_ids that already have a persistent event subscription
/// Prevents creating duplicate subscriptions when follow-up messages arrive
static PERSISTENT_SUBSCRIPTIONS: parking_lot::Mutex<Option<Arc<RwLock<HashSet<String>>>>> =
    parking_lot::Mutex::new(None);

/// Guards against concurrent thread loading. Only one thread load can be
/// in progress at a time (the UI only shows one thread anyway). Prevents
/// double-load when workspace restore and open_thread race.
static THREAD_LOAD_IN_PROGRESS: parking_lot::Mutex<Option<String>> =
    parking_lot::Mutex::new(None);

/// Streaming throttle state per message entry.
/// Keyed by "{thread_id}:{entry_idx}" to support multi-entry streaming.
static STREAMING_THROTTLE: parking_lot::Mutex<Option<Arc<RwLock<HashMap<String, StreamingThrottleState>>>>> =
    parking_lot::Mutex::new(None);

/// Minimum interval between message_added events for the same entry.
/// Reduces Zed→Go wire traffic by ~90% (10 events/sec instead of 100+).
const STREAMING_THROTTLE_INTERVAL: Duration = Duration::from_millis(100);

/// Per-entry throttle state for streaming events.
struct StreamingThrottleState {
    last_sent: Instant,
    pending_content: Option<PendingMessage>,
}

/// Content waiting to be sent when the throttle window expires.
struct PendingMessage {
    acp_thread_id: String,
    message_id: String,
    role: String,
    content: String,
    request_id: String,
    entry_type: String,
    tool_name: String,
    tool_status: String,
}

/// Initialize the thread registry
pub fn init_thread_registry() {
    let mut registry = THREAD_REGISTRY.lock();
    if registry.is_none() {
        *registry = Some(Arc::new(RwLock::new(HashMap::new())));
    }

    let mut keep_alive = THREAD_KEEP_ALIVE.lock();
    if keep_alive.is_none() {
        *keep_alive = Some(Arc::new(RwLock::new(HashMap::new())));
    }

    let mut session_map = THREAD_AGENT_SESSION_MAP.lock();
    if session_map.is_none() {
        *session_map = Some(Arc::new(RwLock::new(HashMap::new())));
    }

    let mut request_map = THREAD_REQUEST_MAP.lock();
    if request_map.is_none() {
        *request_map = Some(Arc::new(RwLock::new(HashMap::new())));
    }

    let mut external_map = EXTERNAL_ORIGINATED_ENTRIES.lock();
    if external_map.is_none() {
        *external_map = Some(Arc::new(RwLock::new(HashMap::new())));
    }

    let mut persistent_subs = PERSISTENT_SUBSCRIPTIONS.lock();
    if persistent_subs.is_none() {
        *persistent_subs = Some(Arc::new(RwLock::new(HashSet::new())));
    }

}

/// Mark an entry as originated from external system (won't be echoed back)
fn mark_external_originated_entry(thread_id: String, entry_idx: usize) {
    init_thread_registry();
    let map = EXTERNAL_ORIGINATED_ENTRIES.lock();
    if let Some(m) = map.as_ref() {
        m.write().entry(thread_id).or_insert_with(HashSet::new).insert(entry_idx);
    }
}

/// Check if entry originated from external system
pub fn is_external_originated_entry(thread_id: &str, entry_idx: usize) -> bool {
    let map = EXTERNAL_ORIGINATED_ENTRIES.lock();
    if let Some(m) = map.as_ref() {
        m.read().get(thread_id).map_or(false, |set| set.contains(&entry_idx))
    } else {
        false
    }
}

/// Check if a thread already has a persistent event subscription
fn has_persistent_subscription(thread_id: &str) -> bool {
    init_thread_registry();
    let subs = PERSISTENT_SUBSCRIPTIONS.lock();
    subs.as_ref().map_or(false, |s| s.read().contains(thread_id))
}

/// Mark a thread as having a persistent event subscription
fn mark_persistent_subscription(thread_id: String) {
    init_thread_registry();
    let subs = PERSISTENT_SUBSCRIPTIONS.lock();
    if let Some(s) = subs.as_ref() {
        s.write().insert(thread_id);
    }
}

/// Initialize the streaming throttle state
fn init_streaming_throttle() {
    let mut throttle = STREAMING_THROTTLE.lock();
    if throttle.is_none() {
        *throttle = Some(Arc::new(RwLock::new(HashMap::new())));
    }
}

/// Throttled send of message_added events. Only sends if enough time has passed
/// since the last send for this entry. Otherwise, stores the content as pending.
/// Returns true if the event was sent, false if throttled.
fn throttled_send_message_added(
    acp_thread_id: &str,
    entry_idx: usize,
    role: &str,
    content: String,
    request_id: &str,
    entry_type: &str,
    tool_name: &str,
    tool_status: &str,
) -> bool {
    init_streaming_throttle();
    let key = format!("{}:{}", acp_thread_id, entry_idx);
    let thread_prefix = format!("{}:", acp_thread_id);
    let now = Instant::now();

    // Hold locks only while reading/mutating state, collect messages to send after release.
    let mut stale_pending: Vec<PendingMessage> = Vec::new();
    let mut current_to_send: Option<PendingMessage> = None;
    let sent: bool;

    {
        let throttle_map = STREAMING_THROTTLE.lock();
        let Some(map) = throttle_map.as_ref() else { return false };
        let mut map = map.write();

        // Flush pending content for all OTHER entries in this thread.
        // This ensures each entry's final content (e.g. tool call
        // "Status: Completed") is sent before we move on, rather than
        // waiting for the end-of-turn flush.
        for (k, state) in map.iter_mut() {
            if k.starts_with(&thread_prefix) && *k != key {
                if let Some(pending) = state.pending_content.take() {
                    state.last_sent = now;
                    stale_pending.push(pending);
                }
            }
        }

        let state = map.entry(key).or_insert_with(|| StreamingThrottleState {
            last_sent: Instant::now() - STREAMING_THROTTLE_INTERVAL,
            pending_content: None,
        });

        if now.duration_since(state.last_sent) >= STREAMING_THROTTLE_INTERVAL {
            state.last_sent = now;
            state.pending_content = None;
            current_to_send = Some(PendingMessage {
                acp_thread_id: acp_thread_id.to_string(),
                message_id: entry_idx.to_string(),
                role: role.to_string(),
                content,
                request_id: request_id.to_string(),
                entry_type: entry_type.to_string(),
                tool_name: tool_name.to_string(),
                tool_status: tool_status.to_string(),
            });
            sent = true;
        } else {
            state.pending_content = Some(PendingMessage {
                acp_thread_id: acp_thread_id.to_string(),
                message_id: entry_idx.to_string(),
                role: role.to_string(),
                content,
                request_id: request_id.to_string(),
                entry_type: entry_type.to_string(),
                tool_name: tool_name.to_string(),
                tool_status: tool_status.to_string(),
            });
            sent = false;
        }
    } // locks released

    // Send stale pending messages from other entries first
    for pending in stale_pending {
        let _ = crate::send_websocket_event(SyncEvent::MessageAdded {
            acp_thread_id: pending.acp_thread_id,
            message_id: pending.message_id,
            role: pending.role,
            content: pending.content,
            request_id: pending.request_id,
            entry_type: pending.entry_type,
            tool_name: pending.tool_name,
            tool_status: pending.tool_status,
            timestamp: chrono::Utc::now().timestamp(),
        });
    }

    // Then send the current entry if not throttled
    if let Some(msg) = current_to_send {
        let _ = crate::send_websocket_event(SyncEvent::MessageAdded {
            acp_thread_id: msg.acp_thread_id,
            message_id: msg.message_id,
            role: msg.role,
            content: msg.content,
            request_id: msg.request_id,
            entry_type: msg.entry_type,
            tool_name: msg.tool_name,
            tool_status: msg.tool_status,
            timestamp: chrono::Utc::now().timestamp(),
        });
    }

    sent
}

/// Flush all pending throttled messages for a given thread and clean up throttle state.
/// Called before message_completed to ensure the final content is sent.
pub fn flush_streaming_throttle(acp_thread_id: &str) {
    init_streaming_throttle();

    // Collect pending messages under lock, then send after releasing
    let pending_messages: Vec<PendingMessage>;
    {
        let throttle_map = STREAMING_THROTTLE.lock();
        let Some(map) = throttle_map.as_ref() else { return };
        let mut map = map.write();

        let prefix = format!("{}:", acp_thread_id);
        let keys_to_remove: Vec<String> = map.keys()
            .filter(|k| k.starts_with(&prefix))
            .cloned()
            .collect();

        pending_messages = keys_to_remove.iter()
            .filter_map(|key| map.remove(key))
            .filter_map(|state| state.pending_content)
            .collect();
    }

    // Send all pending messages outside the lock
    for pending in pending_messages {
        let _ = crate::send_websocket_event(SyncEvent::MessageAdded {
            acp_thread_id: pending.acp_thread_id,
            message_id: pending.message_id,
            role: pending.role,
            content: pending.content,
            request_id: pending.request_id,
            entry_type: pending.entry_type,
            tool_name: pending.tool_name,
            tool_status: pending.tool_status,
            timestamp: chrono::Utc::now().timestamp(),
        });
    }
}

/// Set the current request_id for a thread (used when sending new message to thread)
pub fn set_thread_request_id(acp_thread_id: String, request_id: String) {
    init_thread_registry();
    let map = THREAD_REQUEST_MAP.lock();
    if let Some(m) = map.as_ref() {
        m.write().insert(acp_thread_id, request_id);
    }
}

/// Get the current request_id for a thread
pub fn get_thread_request_id(acp_thread_id: &str) -> Option<String> {
    let map = THREAD_REQUEST_MAP.lock();
    map.as_ref()?.read().get(acp_thread_id).cloned()
}

/// Store the agent's session ID for a thread, so we can pass it to load_session later.
/// The agent (e.g. Claude Code) assigns its own session ID which differs from the Zed thread UUID.
pub fn set_agent_session_id(acp_thread_id: &str, agent_session_id: String) {
    let map = THREAD_AGENT_SESSION_MAP.lock();
    if let Some(m) = map.as_ref() {
        m.write().insert(acp_thread_id.to_string(), agent_session_id);
    }
}

/// Get the agent's session ID for a thread (for passing to load_session).
pub fn get_agent_session_id(acp_thread_id: &str) -> Option<String> {
    let map = THREAD_AGENT_SESSION_MAP.lock();
    map.as_ref().and_then(|m| m.read().get(acp_thread_id).cloned())
}

/// Register an active thread (stores strong reference to keep thread alive)
pub fn register_thread(acp_thread_id: String, thread: Entity<AcpThread>) {
    init_thread_registry();
    let registry = THREAD_REGISTRY.lock();
    if let Some(reg) = registry.as_ref() {
        let mut map = reg.write();
        if let Some(existing) = map.get(&acp_thread_id) {
            if existing.entity_id() != thread.entity_id() {
                log::warn!(
                    "⚠️ [THREAD_SERVICE] register_thread: overwriting thread '{}' with different entity (old={:?}, new={:?})",
                    acp_thread_id,
                    existing.entity_id(),
                    thread.entity_id(),
                );
                eprintln!(
                    "⚠️ [THREAD_SERVICE] register_thread: overwriting thread '{}' with different entity (old={:?}, new={:?})",
                    acp_thread_id,
                    existing.entity_id(),
                    thread.entity_id(),
                );
            }
        }
        map.insert(acp_thread_id.clone(), thread.clone());
    }

    // Also keep a permanent strong reference so the entity survives UI transitions
    let keep_alive = THREAD_KEEP_ALIVE.lock();
    if let Some(ka) = keep_alive.as_ref() {
        ka.write().insert(acp_thread_id, thread);
    }
}

/// Remove a thread from the registry (e.g., when a headless view is dropped).
pub fn unregister_thread(acp_thread_id: &str) {
    let registry = THREAD_REGISTRY.lock();
    if let Some(reg) = registry.as_ref() {
        if reg.write().remove(acp_thread_id).is_some() {
            eprintln!("🗑️ [THREAD_SERVICE] unregister_thread: removed '{}'", acp_thread_id);
            log::info!("🗑️ [THREAD_SERVICE] unregister_thread: removed '{}'", acp_thread_id);
        }
    }

    // Clear persistent subscription so that if this thread is reloaded later
    // (e.g., follow-up to a non-visible thread), ensure_thread_subscription
    // will create a fresh subscription on the new Entity<AcpThread>.
    // Without this, the old subscription (attached to the dropped entity) is
    // gone but the flag remains, so no new subscription is created and events
    // like Stopped/message_completed are silently lost.
    let subs = PERSISTENT_SUBSCRIPTIONS.lock();
    if let Some(s) = subs.as_ref() {
        if s.write().remove(acp_thread_id) {
            eprintln!("🗑️ [THREAD_SERVICE] unregister_thread: cleared persistent subscription for '{}'", acp_thread_id);
        }
    }
}

/// Get an active thread as weak reference
pub fn get_thread(acp_thread_id: &str) -> Option<WeakEntity<AcpThread>> {
    // Check the active registry first
    let registry = THREAD_REGISTRY.lock();
    if let Some(entity) = registry.as_ref().and_then(|r| r.read().get(acp_thread_id).cloned()) {
        return Some(entity.downgrade());
    }
    drop(registry);

    // Fall back to the keep-alive map (threads survive UI transitions here)
    let keep_alive = THREAD_KEEP_ALIVE.lock();
    keep_alive.as_ref().and_then(|ka| ka.read().get(acp_thread_id).map(|e| e.downgrade()))
}

/// Ensure a thread has an event subscription for syncing to Helix.
///
/// This is the SINGLE source of truth for thread event subscriptions. All code paths
/// that create or load threads must call this to set up the subscription. It is
/// idempotent — if a persistent subscription already exists, it does nothing.
///
/// Handles three events:
/// - `NewEntry`: new user/assistant message → send `message_added`
/// - `EntryUpdated`: streaming tokens / tool call updates → throttled `message_added`
/// - `Stopped`: turn completed → flush throttle + send `message_completed`
fn ensure_thread_subscription(
    thread_entity: &Entity<AcpThread>,
    thread_id: &str,
    cx: &mut App,
) {
    if has_persistent_subscription(thread_id) {
        eprintln!("🔔 [THREAD_SERVICE] Thread {} already has persistent subscription, skipping", thread_id);
        return;
    }

    let thread_id_for_sub = thread_id.to_string();
    mark_persistent_subscription(thread_id.to_string());

    // Track the request_id that was active when the current turn started.
    // Used by the Stopped handler to flush with the correct request_id,
    // even if a follow-up/interrupt message has already updated the global
    // THREAD_REQUEST_MAP to the next turn's request_id.
    let turn_request_id: std::cell::RefCell<String> = std::cell::RefCell::new(
        crate::get_thread_request_id(thread_id).unwrap_or_default()
    );

    cx.subscribe(thread_entity, move |thread_entity, event, cx| {
        match event {
            AcpThreadEvent::NewEntry => {
                let thread = thread_entity.read(cx);
                let latest_idx = thread.entries().len().saturating_sub(1);
                if is_external_originated_entry(&thread_id_for_sub, latest_idx) {
                    return;
                }
                if let Some(entry) = thread.entries().get(latest_idx) {
                    let (role, content, entry_type) = match entry {
                        acp_thread::AgentThreadEntry::UserMessage(msg) => {
                            ("user", msg.content.to_markdown(cx).to_string(), "text")
                        }
                        acp_thread::AgentThreadEntry::AssistantMessage(msg) => {
                            ("assistant", msg.content_only(cx), "text")
                        }
                        acp_thread::AgentThreadEntry::ToolCall(tool_call) => {
                            ("assistant", tool_call.to_markdown(cx), "tool_call")
                        }
                        _ => return,
                    };
                    let (tool_name, tool_status) = match entry {
                        acp_thread::AgentThreadEntry::ToolCall(tool_call) => {
                            (tool_call.label.read(cx).source().to_string(), tool_call.status.to_string())
                        }
                        _ => (String::new(), String::new()),
                    };
                    let rid = crate::get_thread_request_id(&thread_id_for_sub)
                        .unwrap_or_default();
                    // Snapshot the request_id when the first assistant entry of a turn appears.
                    // This ensures the Stopped flush uses the turn's own request_id, not a
                    // later follow-up's.
                    if role == "assistant" {
                        *turn_request_id.borrow_mut() = rid.clone();
                    }
                    let _ = crate::send_websocket_event(SyncEvent::MessageAdded {
                        acp_thread_id: thread_id_for_sub.clone(),
                        message_id: latest_idx.to_string(),
                        role: role.to_string(),
                        content,
                        request_id: rid,
                        entry_type: entry_type.to_string(),
                        tool_name,
                        tool_status,
                        timestamp: chrono::Utc::now().timestamp(),
                    });
                }
            }
            AcpThreadEvent::EntryUpdated(entry_idx) => {
                let thread = thread_entity.read(cx);
                if let Some(entry) = thread.entries().get(*entry_idx) {
                    let (content, entry_type, tool_name, tool_status) = match entry {
                        acp_thread::AgentThreadEntry::AssistantMessage(msg) => {
                            (msg.content_only(cx), "text", String::new(), String::new())
                        }
                        acp_thread::AgentThreadEntry::ToolCall(tool_call) => {
                            let name = tool_call.label.read(cx).source().to_string();
                            let status = tool_call.status.to_string();
                            (tool_call.to_markdown(cx), "tool_call", name, status)
                        }
                        _ => return,
                    };
                    let rid = crate::get_thread_request_id(&thread_id_for_sub)
                        .unwrap_or_default();
                    throttled_send_message_added(
                        &thread_id_for_sub,
                        *entry_idx,
                        "assistant",
                        content,
                        &rid,
                        entry_type,
                        &tool_name,
                        &tool_status,
                    );
                }
            }
            AcpThreadEvent::Stopped(_) => {
                flush_streaming_throttle(&thread_id_for_sub);

                // AcpThread calls flush_streaming_text before emitting Stopped, so all
                // Markdown entities now have their complete buffered text. Send corrected
                // content for ALL entries — EntryUpdated events during streaming carried
                // incomplete content (text was still in the streaming buffer at that point),
                // so intermediate text entries were truncated. Re-sending with the now-complete
                // content is safe: the Go accumulator uses overwrite semantics for known message_ids.
                let thread = thread_entity.read(cx);
                let entries = thread.entries();
                // Use the request_id captured when this turn started, NOT the current
                // global request_id. If a follow-up/interrupt message arrived before
                // Stopped fires, the global ID already points to the next turn.
                let rid = turn_request_id.borrow().clone();

                // Find the start of the current turn: the entry AFTER the last UserMessage.
                // Only flush entries from the current turn — sending old entries would cause
                // them to leak into the current interaction's response_entries on the Go side.
                let turn_start = entries.iter().enumerate().rev()
                    .find_map(|(i, e)| matches!(e, acp_thread::AgentThreadEntry::UserMessage(_)).then_some(i + 1))
                    .unwrap_or(0);

                for (idx, entry) in entries.iter().enumerate().skip(turn_start) {
                    match entry {
                        acp_thread::AgentThreadEntry::AssistantMessage(msg) => {
                            let content = msg.content_only(cx);
                            if !content.is_empty() {
                                crate::send_websocket_event(SyncEvent::MessageAdded {
                                    acp_thread_id: thread_id_for_sub.clone(),
                                    message_id: idx.to_string(),
                                    role: "assistant".to_string(),
                                    content,
                                    request_id: rid.clone(),
                                    entry_type: "text".to_string(),
                                    tool_name: String::new(),
                                    tool_status: String::new(),
                                    timestamp: chrono::Utc::now().timestamp(),
                                }).log_err();
                            }
                        }
                        acp_thread::AgentThreadEntry::ToolCall(tool_call) => {
                            let content = tool_call.to_markdown(cx);
                            if !content.is_empty() {
                                let name = tool_call.label.read(cx).source().to_string();
                                let status = tool_call.status.to_string();
                                crate::send_websocket_event(SyncEvent::MessageAdded {
                                    acp_thread_id: thread_id_for_sub.clone(),
                                    message_id: idx.to_string(),
                                    role: "assistant".to_string(),
                                    content,
                                    request_id: rid.clone(),
                                    entry_type: "tool_call".to_string(),
                                    tool_name: name,
                                    tool_status: status,
                                    timestamp: chrono::Utc::now().timestamp(),
                                }).log_err();
                            }
                        }
                        _ => {}
                    }
                }

                // Use the turn's captured request_id for message_completed too
                let completed_rid = turn_request_id.borrow().clone();
                eprintln!(
                    "📤 [THREAD_SERVICE] Stopped event: sending message_completed for thread {} (request_id={})",
                    thread_id_for_sub, completed_rid
                );
                let _ = crate::send_websocket_event(SyncEvent::MessageCompleted {
                    acp_thread_id: thread_id_for_sub.clone(),
                    message_id: "0".to_string(),
                    request_id: completed_rid,
                });
            }
            _ => {}
        }
    }).detach();
}

/// Setup WebSocket thread handler for a workspace
///
/// Called during workspace creation from zed.rs.
/// Contains ALL the business logic for thread creation and management.
///
/// This is the NON-UI service layer that creates and manages ACP threads in response
/// to WebSocket messages from external systems (e.g., Helix).
pub fn setup_thread_handler(
    project: Entity<Project>,
    acp_history_store: Entity<ThreadStore>,
    fs: Arc<dyn Fs>,
    cx: &mut App,
) {
    log::info!("🔧 [THREAD_SERVICE] Setting up WebSocket thread handler");

    // Create callback channel for thread creation requests
    let (callback_tx, mut callback_rx) = mpsc::unbounded_channel::<ThreadCreationRequest>();

    // Register callback globally so WebSocket sync can send requests
    crate::init_thread_creation_callback(callback_tx);
    log::info!("✅ [THREAD_SERVICE] Thread creation callback registered");

    // Create callback channel for thread open requests
    let (open_callback_tx, mut open_callback_rx) = mpsc::unbounded_channel::<ThreadOpenRequest>();

    // Register open callback globally
    crate::init_thread_open_callback(open_callback_tx);
    log::info!("✅ [THREAD_SERVICE] Thread open callback registered");

    // Clone resources for both spawned tasks
    let project_for_create = project.clone();
    let acp_history_store_for_create = acp_history_store.clone();
    let fs_for_create = fs.clone();
    let project_for_open = project.clone();
    let acp_history_store_for_open = acp_history_store.clone();
    let fs_for_open = fs.clone();

    // Spawn handler task to process thread creation requests
    cx.spawn(async move |cx| {
        eprintln!("🔧 [THREAD_SERVICE] Handler task started, waiting for requests...");
        log::info!("🔧 [THREAD_SERVICE] Handler task started, waiting for requests...");

        while let Some(request) = callback_rx.recv().await {
            eprintln!(
                "📨 [THREAD_SERVICE] Received thread creation request: acp_thread_id={:?}, request_id={}",
                request.acp_thread_id,
                request.request_id
            );
            log::info!(
                "📨 [THREAD_SERVICE] Received thread creation request: acp_thread_id={:?}, request_id={}",
                request.acp_thread_id,
                request.request_id
            );

            // Check if this is a follow-up message to existing thread
            if let Some(existing_thread_id) = &request.acp_thread_id {
                eprintln!("🔍 [THREAD_SERVICE] Checking for existing thread: '{}'", existing_thread_id);
                log::info!("🔍 [THREAD_SERVICE] Checking for existing thread: '{}'", existing_thread_id);

                // Skip empty string thread IDs (these are new thread requests)
                if existing_thread_id.is_empty() {
                    eprintln!("⚠️ [THREAD_SERVICE] Empty thread ID, creating new thread");
                    log::warn!("⚠️ [THREAD_SERVICE] Empty thread ID, creating new thread");
                } else if let Some(thread) = get_thread(existing_thread_id) {
                    eprintln!(
                        "🔄 [THREAD_SERVICE] Sending to existing thread: {}",
                        existing_thread_id
                    );
                    log::info!(
                        "🔄 [THREAD_SERVICE] Sending to existing thread: {}",
                        existing_thread_id
                    );

                    // Notify AgentPanel to display this thread (it may not be currently visible)
                    // This ensures the UI switches to the correct thread before the message is sent
                    if let Some(thread_entity) = thread.upgrade() {
                        if let Err(e) = crate::notify_thread_display(crate::ThreadDisplayNotification {
                            thread_entity: thread_entity.clone(),
                            helix_session_id: existing_thread_id.clone(),
                            agent_name: request.agent_name.clone(),
                        }) {
                            eprintln!("⚠️ [THREAD_SERVICE] Failed to notify thread display for follow-up: {}", e);
                        }
                    }

                    if let Err(e) = handle_follow_up_message(
                        thread,
                        existing_thread_id.clone(),
                        request.request_id.clone(),
                        request.message,
                        request.simulate_input,
                        cx.clone()
                    ).await {
                        eprintln!("❌ [THREAD_SERVICE] Failed to send follow-up message: {}", e);
                        log::error!("❌ [THREAD_SERVICE] Failed to send follow-up message: {}", e);

                        // If follow-up failed (e.g., entity released), send error back to Helix
                        let error_event = SyncEvent::ThreadLoadError {
                            acp_thread_id: existing_thread_id.clone(),
                            request_id: request.request_id.clone(),
                            error: format!("Failed to send follow-up: {}", e),
                        };
                        if let Err(send_err) = crate::send_websocket_event(error_event) {
                            eprintln!("❌ [THREAD_SERVICE] Failed to send error event: {}", send_err);
                        }
                    }
                    continue;
                } else {
                    // Thread not in registry - try to load from agent first
                    eprintln!(
                        "🔄 [THREAD_SERVICE] Thread {} not in registry, attempting to load from agent...",
                        existing_thread_id
                    );
                    log::info!(
                        "🔄 [THREAD_SERVICE] Thread {} not in registry, attempting to load from agent...",
                        existing_thread_id
                    );

                    // Try to load the session from the agent
                    let load_result = load_thread_from_agent(
                        project_for_create.clone(),
                        acp_history_store_for_create.clone(),
                        fs_for_create.clone(),
                        existing_thread_id.clone(),
                        request.agent_name.clone(),
                        cx.clone(),
                    ).await;

                    match load_result {
                        Ok(thread) => {
                            eprintln!(
                                "✅ [THREAD_SERVICE] Successfully loaded thread {} from agent, sending message",
                                existing_thread_id
                            );
                            log::info!(
                                "✅ [THREAD_SERVICE] Successfully loaded thread {} from agent, sending message",
                                existing_thread_id
                            );
                            // Send the message to the loaded thread
                            if let Err(e) = handle_follow_up_message(
                                thread,
                                existing_thread_id.clone(),
                                request.request_id.clone(),
                                request.message,
                                request.simulate_input,
                                cx.clone()
                            ).await {
                                eprintln!("❌ [THREAD_SERVICE] Failed to send message to loaded thread: {}", e);
                                log::error!("❌ [THREAD_SERVICE] Failed to send message to loaded thread: {}", e);
                            }
                            continue;
                        }
                        Err(e) => {
                            // Thread can't be reloaded. This should be rare now that
                            // THREAD_KEEP_ALIVE keeps all thread entities alive.
                            // Report the error back to Helix rather than silently
                            // creating a new thread (which would lose conversation context).
                            eprintln!(
                                "❌ [THREAD_SERVICE] Failed to load thread {} from agent: {} - sending error to Helix",
                                existing_thread_id, e
                            );
                            log::error!(
                                "❌ [THREAD_SERVICE] Failed to load thread {} from agent: {}",
                                existing_thread_id, e
                            );

                            let error_event = SyncEvent::ThreadLoadError {
                                acp_thread_id: existing_thread_id.clone(),
                                request_id: request.request_id.clone(),
                                error: format!("Failed to load thread: {}", e),
                            };
                            if let Err(send_err) = crate::send_websocket_event(error_event) {
                                eprintln!("❌ [THREAD_SERVICE] Failed to send error event: {}", send_err);
                            }

                            continue;
                        }
                    }
                }
            }

            // Create new ACP thread (synchronously via cx.update to avoid async context issues)
            eprintln!("🆕 [THREAD_SERVICE] Creating new ACP thread for request: {}", request.request_id);
            log::info!("🆕 [THREAD_SERVICE] Creating new ACP thread for request: {}", request.request_id);
            if let Err(e) = cx.update(|cx| {
                create_new_thread_sync(
                    project_for_create.clone(),
                    acp_history_store_for_create.clone(),
                    fs_for_create.clone(),
                    request,
                    cx,
                )
            }) {
                log::error!("❌ [THREAD_SERVICE] Failed to create thread: {}", e);
            }
        }

        log::warn!("⚠️ [THREAD_SERVICE] Handler task exiting - callback channel closed");
        anyhow::Ok(())
    })
    .detach();

    // Spawn handler task to process thread open requests
    cx.spawn(async move |cx| {
        eprintln!("🔧 [THREAD_SERVICE] Open thread handler task started, waiting for requests...");
        log::info!("🔧 [THREAD_SERVICE] Open thread handler task started, waiting for requests...");

        while let Some(request) = open_callback_rx.recv().await {
            eprintln!(
                "📨 [THREAD_SERVICE] Received thread open request: acp_thread_id={}",
                request.acp_thread_id
            );
            log::info!(
                "📨 [THREAD_SERVICE] Received thread open request: acp_thread_id={}",
                request.acp_thread_id
            );

            // Open the thread via agent (loads from database)
            if let Err(e) = cx.update(|cx| {
                open_existing_thread_sync(
                    project_for_open.clone(),
                    acp_history_store_for_open.clone(),
                    fs_for_open.clone(),
                    request,
                    cx,
                )
            }) {
                log::error!("❌ [THREAD_SERVICE] Failed to open thread: {}", e);
            }
        }

        log::warn!("⚠️ [THREAD_SERVICE] Open thread handler task exiting - callback channel closed");
        anyhow::Ok(())
    })
    .detach();

    log::info!("✅ [THREAD_SERVICE] WebSocket thread handler initialized");
}

/// Create a new ACP thread and send the initial message (synchronous version)
fn create_new_thread_sync(
    project: Entity<Project>,
    acp_history_store: Entity<ThreadStore>,
    fs: Arc<dyn Fs>,
    request: ThreadCreationRequest,
    cx: &mut App,
) -> Result<()> {
    log::info!("[THREAD_SERVICE] Creating ACP thread with agent: {:?}", request.agent_name);

    let agent = match request.agent_name.as_deref() {
        Some("zed-agent") | None => ExternalAgent::NativeAgent,
        Some(name) => {
            // Map Helix agent names to Zed registry agent IDs.
            // Helix sends "claude" but the Zed registry uses "claude-acp".
            let zed_name = match name {
                "claude" => agent_servers::CLAUDE_AGENT_ID,
                other => other,
            };
            ExternalAgent::Custom {
                name: gpui::SharedString::from(zed_name.to_string()),
                command: project::agent_server_store::AgentServerCommand {
                    path: std::path::PathBuf::new(),
                    args: vec![],
                    env: None,
                },
            }
        }
    };

    // Spawn async task to complete the connection and create the thread
    let request_clone = request.clone();
    let project_clone = project.clone();
    cx.spawn(async move |cx| {
        eprintln!("🚀 [THREAD_SERVICE] Spawn task started for request: {}", request_clone.request_id);

        // Retry connecting up to 10 times with 1s delay when the agent is not yet registered.
        // This handles the race where the AgentRegistryStore's async network fetch (for registry
        // agents like claude-acp) hasn't completed by the time the first message arrives.
        let max_retries = 10u32;
        let connection: std::rc::Rc<dyn acp_thread::AgentConnection> = {
            let mut attempt = 0u32;
            loop {
                let connection_task = cx.update(|cx| {
                    let server = agent.server(fs.clone(), acp_history_store.clone());
                    let agent_server_store = project.read(cx).agent_server_store().clone();
                    let delegate = agent_servers::AgentServerDelegate::new(
                        agent_server_store,
                        None,
                    );
                    eprintln!("🔌 [THREAD_SERVICE] Calling server.connect() (attempt {}/{})...", attempt + 1, max_retries + 1);
                    server.connect(delegate, project.clone(), cx)
                });

                eprintln!("⏳ [THREAD_SERVICE] Awaiting connection task...");
                match connection_task.await {
                    Ok(conn) => {
                        eprintln!("✅ [THREAD_SERVICE] Connected to agent successfully");
                        break conn;
                    }
                    Err(e) if attempt < max_retries && e.to_string().contains("not registered") => {
                        eprintln!("⚠️ [THREAD_SERVICE] Agent not registered yet (attempt {}/{}), retrying in 1s: {}", attempt + 1, max_retries, e);
                        attempt += 1;
                        cx.background_executor().timer(Duration::from_secs(1)).await;
                    }
                    Err(e) => {
                        eprintln!("❌ [THREAD_SERVICE] Failed to connect to agent: {}", e);
                        return Err(e);
                    }
                }
            }
        };

        // Authenticate if required
        let auth_methods = connection.auth_methods();
        if let Some(first_method) = auth_methods.first() {
            let connection_for_auth = connection.clone();
            let auth_task = cx.update(|cx| {
                connection_for_auth.authenticate(first_method.id().clone(), cx)
            });
            if let Err(e) = auth_task.await {
                log::warn!("[THREAD_SERVICE] Authentication failed (continuing): {}", e);
            }
        }

        // Use ZED_WORK_DIR for consistency with agent_panel.rs and thread_view.rs
        // This ensures sessions created here can be found when listing/loading sessions
        // from the UI (which also uses ZED_WORK_DIR as the cwd for project hash calculation)
        let cwd = std::env::var("ZED_WORK_DIR")
            .ok()
            .map(|dir| std::path::PathBuf::from(dir))
            .unwrap_or_else(|| {
                // Fallback to first worktree if ZED_WORK_DIR not set
                cx.update(|cx| {
                    project_clone.read(cx).worktrees(cx).next()
                        .map(|wt| wt.read(cx).abs_path().to_path_buf())
                        .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
                })
            });
        let connection_for_tools = connection.clone();
        let connection_for_model = connection.clone();
        let work_dirs = PathList::new(&[cwd.clone()]);
        eprintln!("🧵 [THREAD_SERVICE] Calling new_session() with cwd={:?}", cwd);
        let thread_entity: Entity<AcpThread> = match cx.update(|cx| {
            connection.new_session(project_clone.clone(), work_dirs, cx)
        }).await {
            Ok(entity) => {
                eprintln!("✅ [THREAD_SERVICE] new_session() succeeded");
                entity
            }
            Err(e) => {
                eprintln!("❌ [THREAD_SERVICE] new_session() failed: {}", e);
                return Err(e);
            }
        };

        // Wait for the NativeAgent's model list to be populated.
        // NativeAgent::new() spawns authenticate_all_language_model_providers()
        // which runs asynchronously. When it completes, ProviderStateChanged
        // fires, NativeAgent refreshes its model list. We wait for that refresh.
        let session_id = cx.update(|cx| thread_entity.read(cx).session_id().clone());
        {
            let mut model_watch = cx.update(|cx| {
                connection_for_model.model_selector(&session_id)
                    .and_then(|selector| selector.watch(cx))
            });
            if let Some(ref mut watch_rx) = model_watch {
                let wait_for_models = async {
                    let _ = watch_rx.changed().await;
                };
                let timeout = async {
                    smol::Timer::after(std::time::Duration::from_secs(15)).await;
                    log::warn!("[THREAD_SERVICE] Timed out waiting for models (15s), proceeding");
                };
                futures::future::select(Box::pin(wait_for_models), Box::pin(timeout)).await;
            }
        }

        // The settings system may set the default model to zed.dev (from default.json),
        // which can't be resolved without a Zed account. The NativeAgent's auto-model
        // logic uses registry.default_model() which may return None in this case.
        // If the thread still has no model after the watch fired, explicitly select
        // the first available model from the authenticated providers.
        if let Some(selector) = cx.update(|_cx| connection_for_model.model_selector(&session_id)) {
            let has_model = cx.update(|cx| selector.selected_model(cx)).await;
            let needs_model = match has_model {
                Ok(_) => false,
                Err(_) => true,
            };
            if needs_model {
                let model_list_result = cx.update(|cx| selector.list_models(cx)).await;
                if let Ok(model_list) = model_list_result {
                    let first_model_id = match &model_list {
                        acp_thread::AgentModelList::Flat(models) => models.first().map(|m| m.id.clone()),
                        acp_thread::AgentModelList::Grouped(groups) => {
                            groups.values().flat_map(|v| v.iter()).next().map(|m| m.id.clone())
                        }
                    };
                    if let Some(model_id) = first_model_id {
                        if let Err(e) = cx.update(|cx| selector.select_model(model_id.clone(), cx)).await {
                            log::warn!("[THREAD_SERVICE] Failed to select model: {}", e);
                        }
                    }
                }
            }
        }

        // Wait for MCP context server tools to finish loading before sending
        // the first message, so the LLM request includes all available tools.
        let tools_ready_task = cx.update(|cx| connection_for_tools.wait_for_tools_ready(cx));
        tools_ready_task.await;

        let acp_thread_id = cx.update(|cx| {
            let thread_id = thread_entity.read(cx).session_id().to_string();
            log::info!("[THREAD_SERVICE] Created ACP thread: {}", thread_id);
            thread_id
        });

        // Keep thread entity alive for the duration of this task
        let _thread_keep_alive = thread_entity.clone();

        // Store the current request_id for this thread (so message_completed uses correct ID)
        set_thread_request_id(acp_thread_id.clone(), request_clone.request_id.clone());

        // NOTE: WebSocket event sending is now handled centrally in ThreadView.handle_thread_event
        // This avoids duplicate events when thread is both created here and displayed in UI via from_existing_thread

        // Register thread for follow-up messages (strong reference keeps it alive)
        register_thread(acp_thread_id.clone(), thread_entity.clone());

        // Store the agent's session ID so load_session uses the right ID later.
        // ACP agents like Claude Code assign their own session IDs that differ
        // from the Zed thread UUID.
        cx.update(|cx| {
            let agent_sid = thread_entity.read(cx).session_id().to_string();
            eprintln!("📋 [THREAD_SERVICE] Registered thread: {} → agent session: {}", acp_thread_id, agent_sid);
            log::info!("📋 [THREAD_SERVICE] Registered thread: {} → agent session: {}", acp_thread_id, agent_sid);
            set_agent_session_id(&acp_thread_id, agent_sid);
        });

        // Send agent_ready event to Helix (signals that agent is ready to receive prompts)
        // This prevents race conditions where Helix sends continue prompts before agent is initialized
        let agent_name_for_ready = request_clone.agent_name.clone().unwrap_or_else(|| "zed-agent".to_string());
        crate::send_agent_ready(agent_name_for_ready, Some(acp_thread_id.clone()));

        // Notify AgentPanel to display this thread (for auto-select in UI)
        if let Err(e) = crate::notify_thread_display(crate::ThreadDisplayNotification {
            thread_entity: thread_entity.clone(),
            helix_session_id: request_clone.request_id.clone(),
            agent_name: request_clone.agent_name.clone(), // Pass agent name for correct UI label
        }) {
            eprintln!("⚠️ [THREAD_SERVICE] Failed to notify thread display: {}", e);
            log::warn!("⚠️ [THREAD_SERVICE] Failed to notify thread display: {}", e);
        } else {
            eprintln!("📤 [THREAD_SERVICE] Notified AgentPanel to display thread");
            log::info!("📤 [THREAD_SERVICE] Notified AgentPanel to display thread");
        }

        // Send thread_created event via WebSocket
        let thread_created_event = SyncEvent::ThreadCreated {
            acp_thread_id: acp_thread_id.clone(),
            request_id: request_clone.request_id.clone(),
        };

        if let Err(e) = crate::send_websocket_event(thread_created_event) {
            eprintln!("❌ [THREAD_SERVICE] Failed to send thread_created event: {}", e);
            log::error!("❌ [THREAD_SERVICE] Failed to send thread_created event: {}", e);
        } else {
            eprintln!("📤 [THREAD_SERVICE] Sent thread_created: {}", acp_thread_id);
            log::info!("📤 [THREAD_SERVICE] Sent thread_created: {}", acp_thread_id);
        }

        // Mark the entry that will be created as external-originated (so we don't echo it back)
        // Unless simulate_input=true, in which case we want the sync to fire
        if !request_clone.simulate_input {
            let entry_idx_to_mark = cx.update(|cx| {
                thread_entity.read(cx).entries().len()
            });
            mark_external_originated_entry(acp_thread_id.clone(), entry_idx_to_mark);
            eprintln!("🏷️ [THREAD_SERVICE] Marked entry {} as external-originated (won't echo back)", entry_idx_to_mark);
        } else {
            eprintln!("🎭 [THREAD_SERVICE] simulate_input=true, NOT marking entry as external-originated (will sync back)");
        }

        // Subscribe to thread events PERSISTENTLY so that:
        // Subscribe to thread events so streaming responses sync to Helix
        // and future user-typed messages in Zed's UI also sync back.
        cx.update(|cx| {
            ensure_thread_subscription(&thread_entity, &acp_thread_id, cx);
        });

        // Send the initial message to the thread to trigger AI response
        eprintln!("🔧 [THREAD_SERVICE] About to send message to thread...");
        let send_task = cx.update(|cx| {
            thread_entity.update(cx, |thread: &mut AcpThread, cx| {
                let message = vec![ContentBlock::Text(
                    TextContent::new(request_clone.message.clone())
                )];
                eprintln!("🔧 [THREAD_SERVICE] Calling thread.send() with message: {}", request_clone.message);
                thread.send(message, cx)
            })
        });

        // Await the send task directly (don't spawn and detach)
        eprintln!("🔧 [THREAD_SERVICE] Awaiting send task...");
        match send_task.await {
            Ok(_) => {
                eprintln!("✅ [THREAD_SERVICE] Send task completed successfully - message sent to AI");
                log::info!("✅ [THREAD_SERVICE] Send task completed successfully");
            }
            Err(e) => {
                eprintln!("❌ [THREAD_SERVICE] Send task failed: {:#}", e);
                log::error!("❌ [THREAD_SERVICE] Send task failed: {:#}", e);
            }
        }

        eprintln!("✅ [THREAD_SERVICE] Message send awaited - AI response complete");
        log::info!("✅ [THREAD_SERVICE] Message send awaited - AI response complete");

        // NOTE: MessageCompleted is now sent by the persistent subscription's
        // Stopped event handler (above). This ensures ALL turn completions emit
        // MessageCompleted, whether initiated by Helix or by direct Zed UI input.
        // Previously, this code sent MessageCompleted here, but that missed turns
        // the user typed directly into Zed's agent panel.

        anyhow::Ok(())
    }).detach();

    Ok(())
}

/// Handle a follow-up message to an existing thread
async fn handle_follow_up_message(
    thread: WeakEntity<AcpThread>,
    thread_id: String,
    request_id: String,
    message: String,
    simulate_input: bool,
    cx: gpui::AsyncApp,
) -> Result<()> {
    log::info!("💬 [THREAD_SERVICE] Sending follow-up message: {} (simulate_input={})", message, simulate_input);

    // CRITICAL: Update the request_id for this thread so message_completed uses the correct ID!
    set_thread_request_id(thread_id.clone(), request_id.clone());
    eprintln!("🔄 [THREAD_SERVICE] Updated request_id for thread {} to {}", thread_id, request_id);
    log::info!("🔄 [THREAD_SERVICE] Updated request_id for thread {} to {}", thread_id, request_id);

    // Mark the entry that will be created as external-originated (unless simulating user input)
    // When simulate_input=true, we want the NewEntry subscription to fire so the user message
    // syncs back to Helix (testing the Zed → Helix direction)
    if !simulate_input {
        let entry_idx_to_mark = cx.update(|cx| {
            thread.update(cx, |thread, _| thread.entries().len())
        })?;
        mark_external_originated_entry(thread_id.clone(), entry_idx_to_mark);
        eprintln!("🏷️ [THREAD_SERVICE] Marked entry {} as external-originated (follow-up)", entry_idx_to_mark);
    } else {
        eprintln!("🎭 [THREAD_SERVICE] simulate_input=true, NOT marking entry as external-originated (will sync back)");
    }

    // Ensure subscription exists (idempotent — skips if already present)
    cx.update(|cx| {
        if let Some(thread_entity) = thread.upgrade() {
            ensure_thread_subscription(&thread_entity, &thread_id, cx);
        }
    });

    let send_task = cx.update(|cx| {
        thread.update(cx, |thread: &mut AcpThread, cx| {
            let message = vec![ContentBlock::Text(
                TextContent::new(message.clone())
            )];
            thread.send(message, cx)
        })
    })?;

    // Await the send task to completion (LLM response finishes)
    match send_task.await {
        Ok(_) => {
            eprintln!("✅ [THREAD_SERVICE] Follow-up send completed successfully");
        }
        Err(e) => {
            eprintln!("❌ [THREAD_SERVICE] Follow-up send failed: {}", e);
            return Err(e);
        }
    }

    // NOTE: MessageCompleted is now sent by the persistent subscription's
    // Stopped event handler. See create_new_thread_sync for rationale.

    log::info!("✅ [THREAD_SERVICE] Follow-up message sent successfully");
    Ok(())
}

/// Load an existing thread from the agent (async version for use in message handler)
/// This connects to the agent, loads the session via ACP protocol, registers it, and returns a weak reference.
async fn load_thread_from_agent(
    project: Entity<Project>,
    acp_history_store: Entity<ThreadStore>,
    fs: Arc<dyn Fs>,
    acp_thread_id: String,
    agent_name: Option<String>,
    cx: gpui::AsyncApp,
) -> Result<WeakEntity<AcpThread>> {
    eprintln!("📂 [THREAD_SERVICE] load_thread_from_agent: {} (agent: {:?})", acp_thread_id, agent_name);
    log::info!("📂 [THREAD_SERVICE] load_thread_from_agent: {} (agent: {:?})", acp_thread_id, agent_name);

    // Select agent based on agent_name
    let agent = match agent_name.as_deref() {
        Some("zed-agent") | Some("") | None => ExternalAgent::NativeAgent,
        Some(name) => {
            let zed_name = match name {
                "claude" => agent_servers::CLAUDE_AGENT_ID,
                other => other,
            };
            ExternalAgent::Custom {
                name: gpui::SharedString::from(zed_name.to_string()),
                command: project::agent_server_store::AgentServerCommand {
                    path: std::path::PathBuf::new(),
                    args: vec![],
                    env: None,
                },
            }
        }
    };

    let server = agent.server(fs, acp_history_store.clone());

    // Get agent server store and create connection
    let (connection_task, cwd) = cx.update(|cx| {
        let agent_server_store = project.read(cx).agent_server_store().clone();
        let delegate = agent_servers::AgentServerDelegate::new(
            agent_server_store,
            None,
        );
        let connection_task = server.connect(delegate, project.clone(), cx);
        // Use ZED_WORK_DIR for consistency with session storage
        let cwd = std::env::var("ZED_WORK_DIR")
            .ok()
            .map(|dir| std::path::PathBuf::from(dir))
            .unwrap_or_else(|| {
                project.read(cx).worktrees(cx).next()
                    .map(|wt| wt.read(cx).abs_path().to_path_buf())
                    .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
            });
        (connection_task, cwd)
    });

    let connection: std::rc::Rc<dyn acp_thread::AgentConnection> = connection_task.await?;

    eprintln!("✅ [THREAD_SERVICE] Connected to agent for loading thread");
    log::info!("✅ [THREAD_SERVICE] Connected to agent for loading thread");

    // Check if agent supports session loading
    {
        let connection = connection.clone();
        let supports_load = cx.update(|_cx| connection.supports_load_session());
        if !supports_load {
            let err = anyhow::anyhow!("Agent does not support session loading");
            eprintln!("⚠️ [THREAD_SERVICE] {}", err);
            log::warn!("⚠️ [THREAD_SERVICE] {}", err);
            return Err(err);
        }
    }

    // Load the thread from agent using the agent's own session ID (not the Zed thread UUID).
    // ACP agents like Claude Code assign their own session IDs during new_session.
    let agent_sid = get_agent_session_id(&acp_thread_id).unwrap_or_else(|| acp_thread_id.clone());
    eprintln!("📂 [THREAD_SERVICE] load_session: zed_thread={} agent_session={}", acp_thread_id, agent_sid);
    log::info!("📂 [THREAD_SERVICE] load_session: zed_thread={} agent_session={}", acp_thread_id, agent_sid);
    let session_id = acp::SessionId::new(agent_sid);
    let work_dirs = PathList::new(&[cwd.clone()]);
    let project_clone = project.clone();
    // Clone the connection before passing to load_session, which consumes its Rc<Self>.
    // We must keep a strong reference alive until load_task completes, because the
    // spawned tasks inside open_thread/load_thread only hold WeakEntity<NativeAgent>.
    // Without this, the NativeAgent entity is released and those tasks fail with
    // "entity released".
    let _connection_keepalive = connection.clone();
    let load_task = cx.update(|cx| {
        connection.load_session(session_id, project_clone, work_dirs, None, cx)
    });

    let thread_entity: Entity<AcpThread> = load_task.await?;

    let loaded_thread_id = cx.update(|cx| {
        thread_entity.read(cx).session_id().to_string()
    });

    eprintln!("✅ [THREAD_SERVICE] Loaded thread from agent: {}", loaded_thread_id);
    log::info!("✅ [THREAD_SERVICE] Loaded thread from agent: {}", loaded_thread_id);

    // Subscribe to thread events for streaming responses
    cx.update(|cx| {
        ensure_thread_subscription(&thread_entity, &loaded_thread_id, cx);
    });

    // Register thread for future access
    register_thread(loaded_thread_id.clone(), thread_entity.clone());
    set_agent_session_id(&acp_thread_id, loaded_thread_id.clone());
    eprintln!("📋 [THREAD_SERVICE] Registered loaded thread: {} → agent session: {}", acp_thread_id, loaded_thread_id);
    log::info!("📋 [THREAD_SERVICE] Registered loaded thread: {} → agent session: {}", acp_thread_id, loaded_thread_id);

    // Send agent_ready event to Helix (signals that agent is ready to receive prompts)
    let agent_name_for_ready = agent_name.clone().unwrap_or_else(|| "zed-agent".to_string());
    crate::send_agent_ready(agent_name_for_ready, Some(loaded_thread_id.clone()));

    // Notify AgentPanel to display this thread
    if let Err(e) = crate::notify_thread_display(crate::ThreadDisplayNotification {
        thread_entity: thread_entity.clone(),
        helix_session_id: loaded_thread_id.clone(),
        agent_name: agent_name.clone(),
    }) {
        eprintln!("⚠️ [THREAD_SERVICE] Failed to notify thread display: {}", e);
    }

    Ok(thread_entity.downgrade())
}

/// Open an existing ACP thread from database and display it (synchronous version)
fn open_existing_thread_sync(
    project: Entity<Project>,
    acp_history_store: Entity<ThreadStore>,
    fs: Arc<dyn Fs>,
    request: ThreadOpenRequest,
    cx: &mut App,
) -> Result<()> {
    eprintln!("📖 [THREAD_SERVICE] Opening existing ACP thread: {}, agent_name: {:?}",
              request.acp_thread_id, request.agent_name);
    log::info!("📖 [THREAD_SERVICE] Opening existing ACP thread: {}, agent_name: {:?}",
               request.acp_thread_id, request.agent_name);

    // Check if thread is already in registry — ensure it has a subscription
    // (subscription tracking resets on process restart even if thread entity survived)
    if let Some(thread_weak) = get_thread(&request.acp_thread_id) {
        eprintln!("✅ [THREAD_SERVICE] Thread already loaded in registry: {}", request.acp_thread_id);
        log::info!("✅ [THREAD_SERVICE] Thread already loaded in registry: {}", request.acp_thread_id);
        if let Some(thread_entity) = thread_weak.upgrade() {
            ensure_thread_subscription(&thread_entity, &request.acp_thread_id, cx);
        }
        // TODO: Still need to notify AgentPanel to display it
        return Ok(());
    }

    // Check if any thread is already being loaded (async load in progress from
    // workspace restore or a concurrent open_thread). Without this guard, two
    // async loads race: both pass the registry check above, both spawn load tasks,
    // and the second overwrites the first's entity in the registry — orphaning
    // the first entity's event subscriptions.
    {
        let mut loading = THREAD_LOAD_IN_PROGRESS.lock();
        if let Some(in_progress) = loading.as_ref() {
            eprintln!("⏳ [THREAD_SERVICE] DUPLICATE LOAD PREVENTED: {} is already loading, skipping load of {}",
                      in_progress, request.acp_thread_id);
            log::warn!("⏳ [THREAD_SERVICE] DUPLICATE LOAD PREVENTED: {} is already loading, skipping load of {}",
                       in_progress, request.acp_thread_id);
            return Ok(());
        }
        eprintln!("🔒 [THREAD_SERVICE] Acquired thread load lock for {}", request.acp_thread_id);
        log::info!("🔒 [THREAD_SERVICE] Acquired thread load lock for {}", request.acp_thread_id);
        *loading = Some(request.acp_thread_id.clone());
    }

    // Thread not in registry - need to load from agent
    // Select agent based on agent_name (same logic as create_new_thread_sync)
    let agent = match request.agent_name.as_deref() {
        Some("zed-agent") | Some("") | None => ExternalAgent::NativeAgent,
        Some(name) => {
            let zed_name = match name {
                "claude" => agent_servers::CLAUDE_AGENT_ID,
                other => other,
            };
            ExternalAgent::Custom {
                name: gpui::SharedString::from(zed_name.to_string()),
                command: project::agent_server_store::AgentServerCommand {
                    path: std::path::PathBuf::new(),
                    args: vec![],
                    env: None,
                },
            }
        }
    };
    eprintln!("🔧 [THREAD_SERVICE] Selected agent: {:?}", agent);
    log::info!("🔧 [THREAD_SERVICE] Selected agent: {:?}", agent);

    let server = agent.server(fs, acp_history_store.clone());

    // Get agent server store from project
    let agent_server_store = project.read(cx).agent_server_store().clone();

    // Create delegate for connection
    let delegate = agent_servers::AgentServerDelegate::new(
        agent_server_store,
        None,
    );

    // Connect to get AgentConnection
    let connection_task = server.connect(delegate, project.clone(), cx);

    // Use ZED_WORK_DIR for consistency with session storage
    let cwd = std::env::var("ZED_WORK_DIR")
        .ok()
        .map(|dir| std::path::PathBuf::from(dir))
        .unwrap_or_else(|| {
            project.read(cx).worktrees(cx).next()
                .map(|wt| wt.read(cx).abs_path().to_path_buf())
                .unwrap_or_else(|| std::env::current_dir().unwrap_or_default())
        });

    // Spawn async task to load the thread from agent
    let request_clone = request.clone();
    let project_clone = project.clone();
    cx.spawn(async move |cx| {
        // Drop guard: clear THREAD_LOAD_IN_PROGRESS on exit (success or error)
        struct ClearLoadingGuard;
        impl Drop for ClearLoadingGuard {
            fn drop(&mut self) {
                let mut loading = THREAD_LOAD_IN_PROGRESS.lock();
                eprintln!("🔓 [THREAD_SERVICE] Released thread load lock (was {:?})", loading);
                log::info!("🔓 [THREAD_SERVICE] Released thread load lock (was {:?})", loading);
                *loading = None;
            }
        }
        let _loading_guard = ClearLoadingGuard;

        let connection = match connection_task.await {
            Ok(result) => result,
            Err(e) => {
                eprintln!("❌ [THREAD_SERVICE] Failed to connect to agent: {}", e);
                log::error!("❌ [THREAD_SERVICE] Failed to connect to agent: {}", e);
                return Err(e);
            }
        };

        eprintln!("✅ [THREAD_SERVICE] Connected to agent server for thread loading");
        log::info!("✅ [THREAD_SERVICE] Connected to agent server for thread loading");

        // Check if agent supports session loading
        {
            let connection = connection.clone();
            if !cx.update(|_cx| connection.supports_load_session()) {
                eprintln!("⚠️ [THREAD_SERVICE] Agent does not support session loading");
                log::warn!("⚠️ [THREAD_SERVICE] Agent does not support session loading");
                return Err(anyhow::anyhow!("Agent does not support session loading"));
            }
        }

        eprintln!("🔨 [THREAD_SERVICE] Calling connection.load_session() to load from agent...");
        log::info!("🔨 [THREAD_SERVICE] Calling connection.load_session() to load from agent...");

        // Use the generic AgentConnection::load_session() method
        // This works for both NativeAgent (from local DB) and ACP agents (via session/load protocol)
        let session_id = acp::SessionId::new(request_clone.acp_thread_id.clone());
        let work_dirs = PathList::new(&[cwd.clone()]);
        // Clone the connection before passing to load_session, which consumes its Rc<Self>.
        // We must keep a strong reference alive until load_task completes, because the
        // spawned tasks inside open_thread/load_thread only hold WeakEntity<NativeAgent>.
        // Without this, the NativeAgent entity is released and those tasks fail with
        // "entity released".
        let _connection_keepalive = connection.clone();
        let load_task = cx.update(|cx| {
            connection.load_session(session_id, project_clone, work_dirs, None, cx)
        });

        let thread_entity: Entity<AcpThread> = match load_task.await {
            Ok(entity) => entity,
            Err(e) => {
                eprintln!("❌ [THREAD_SERVICE] connection.load_session() failed: {}", e);
                log::error!("❌ [THREAD_SERVICE] connection.load_session() failed: {}", e);
                return Err(e);
            }
        };

        let acp_thread_id = cx.update(|cx| {
            let thread_id = thread_entity.read(cx).session_id().to_string();
            eprintln!("✅ [THREAD_SERVICE] Loaded ACP thread from agent: {} (session_id)", thread_id);
            log::info!("✅ [THREAD_SERVICE] Loaded ACP thread from agent: {} (session_id)", thread_id);
            thread_id
        });

        // Register thread for future access (strong reference keeps it alive)
        register_thread(acp_thread_id.clone(), thread_entity.clone());
        if !request_clone.acp_thread_id.is_empty() {
            set_agent_session_id(&request_clone.acp_thread_id, acp_thread_id.clone());
            eprintln!("📋 [THREAD_SERVICE] Registered thread: {} → agent session: {}", request_clone.acp_thread_id, acp_thread_id);
            log::info!("📋 [THREAD_SERVICE] Registered thread: {} → agent session: {}", request_clone.acp_thread_id, acp_thread_id);
        } else {
            eprintln!("📋 [THREAD_SERVICE] Registered thread: {} (strong reference)", acp_thread_id);
            log::info!("📋 [THREAD_SERVICE] Registered thread: {} (strong reference)", acp_thread_id);
        }

        // Send agent_ready event to Helix (signals that agent is ready to receive prompts)
        // (THREAD_LOAD_IN_PROGRESS is cleared by the ClearLoadingGuard drop guard)
        let agent_name_for_ready = request_clone.agent_name.clone().unwrap_or_else(|| "zed-agent".to_string());
        crate::send_agent_ready(agent_name_for_ready, Some(acp_thread_id.clone()));

        // Notify AgentPanel to display this thread (for auto-select in UI)
        if let Err(e) = crate::notify_thread_display(crate::ThreadDisplayNotification {
            thread_entity: thread_entity.clone(),
            helix_session_id: acp_thread_id.clone(),
            agent_name: request_clone.agent_name.clone(),
        }) {
            eprintln!("⚠️ [THREAD_SERVICE] Failed to notify thread display: {}", e);
            log::warn!("⚠️ [THREAD_SERVICE] Failed to notify thread display: {}", e);
        } else {
            eprintln!("📤 [THREAD_SERVICE] Notified AgentPanel to display opened thread");
            log::info!("📤 [THREAD_SERVICE] Notified AgentPanel to display opened thread");
        }

        eprintln!("✅ [THREAD_SERVICE] Thread opened and displayed successfully");
        log::info!("✅ [THREAD_SERVICE] Thread opened and displayed successfully");

        anyhow::Ok(())
    }).detach();

    Ok(())
}
