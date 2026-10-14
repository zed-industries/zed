//! EventRouter: the single fan-out point for OMP session events.
//!
//! Plan §§5, 71–73: `OMP stdout → background reader → frame decode →
//! EventRouter → channel → batched GPUI updates`. The vendored client's reader
//! thread decodes frames; this router classifies them into [`RouterEvent`]s,
//! keeps the append-only event log (crash-during-stream reconciliation reads
//! this log, plan §74 R3), and forwards to subscribers.
//!
//! Unknown event kinds are tolerated (forwarded as `Unknown`), never fatal —
//! protocol forward-compat. Classification is a pure typed `match`, never
//! Debug-string parsing.

use omp_rpc::wire::{
    AgentMessage, AssistantMessageEvent, ExtensionUiRequest, RpcAgentEvent, RpcNotification,
    SubagentLifecycleStatus,
};
use parking_lot::Mutex;
use std::collections::{BTreeSet, HashMap};
use std::sync::{Arc, mpsc};

/// Classified view of one OMP frame for cedian consumers.
#[derive(Debug, Clone)]
pub enum RouterEvent {
    /// Agent turn started.
    AgentStart,
    /// Agent turn ended (terminal or continuation — check flags).
    AgentEnd { yielded: bool, is_terminal: bool },
    /// Streaming text/thinking/toolcall delta with the RPC message id.
    /// `delta` carries the new text for `Text`/`Thinking` (empty otherwise).
    MessageDelta {
        message_id: String,
        kind: DeltaKind,
        delta: String,
    },
    /// One message completed (full content in payload).
    MessageEnd { message_id: String },
    /// Tool execution lifecycle from OMP's own tools. `args_preview` is a
    /// one-line human summary (never raw JSON); `result_summary` likewise,
    /// filled at end. Phase 3 registry renders both. `paths` are the files an
    /// edit-class tool is about to write, as OMP names them (workspace
    /// relative), so the host can open and mark them before the write lands.
    ToolStart {
        tool_call_id: String,
        tool_name: String,
        args_preview: String,
        paths: Vec<String>,
    },
    /// Tool execution finished. `before` is each file's text before the
    /// call as OMP's result reports it (`edit` results only, by path as
    /// OMP names it).
    ToolEnd {
        tool_call_id: String,
        tool_name: String,
        result_summary: String,
        is_error: bool,
        before: Vec<(String, TextBefore)>,
    },
    /// A prompt ticket completed.
    PromptResult {
        prompt_id: String,
        status: PromptStatus,
    },
    /// Session quiescent — safe to tear down / recycle.
    Settled,
    /// Queue snapshot changed (steering/follow-up chips).
    Queue {
        steering: Vec<String>,
        follow_up: Vec<String>,
    },
    /// An OMP subagent started or ended (ADR-0050). `parent_tool_call_id`
    /// is the `task` tool call that started it.
    SubagentLifecycle {
        id: String,
        agent: String,
        status: SubagentStatus,
        parent_tool_call_id: Option<String>,
        description: Option<String>,
    },
    /// A running subagent reported progress; its id is `progress.id`.
    SubagentProgress {
        id: String,
        agent: String,
        parent_tool_call_id: Option<String>,
        description: Option<String>,
    },
    /// A dialog or UI notice from OMP: approvals and `ask` wait for an
    /// answer (`crate::dialog`); the rest are fire-and-forget.
    UiRequest(ExtensionUiRequest),
    /// Compaction or auto-retry, for the thread; `retrying` is set by the
    /// retry events (`Some(true)` while OMP waits to retry).
    RunNote {
        text: String,
        retrying: Option<bool>,
    },
    /// OMP's model changed; the event names none, `get_state` does.
    ModelChanged,
    /// OMP's effective thinking level changed.
    ThinkingLevel(Option<omp_rpc::wire::ThinkingLevel>),
    /// One of OMP's notices for the person (ADR-0057 decision 3).
    Toast(Toast),
    /// OMP renamed session `session_id` (`session_info_update`).
    SessionInfo {
        session_id: String,
        title: Option<String>,
    },
    /// Forward-compat: recognized frame, unmodeled kind — never fatal.
    Unknown { frame_type: String },
    /// OMP's output closed without cedian shutting it down: the process
    /// crashed or was killed. Nothing follows; a restart is needed.
    Disconnected,
}

/// How loud a toast is; OMP's `NotifyType`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ToastLevel {
    Info,
    Warning,
    Error,
}

impl From<omp_rpc::wire::NotifyType> for ToastLevel {
    fn from(t: omp_rpc::wire::NotifyType) -> Self {
        match t {
            omp_rpc::wire::NotifyType::Info => Self::Info,
            omp_rpc::wire::NotifyType::Warning => Self::Warning,
            omp_rpc::wire::NotifyType::Error => Self::Error,
        }
    }
}

/// A notice from OMP: `notice`, `notify`, `extension_error`,
/// `config_warnings_changed` or `ttsr_triggered`, as one line of text.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Toast {
    pub level: ToastLevel,
    pub text: String,
}

/// Streaming delta kinds (coarse — Phase 2 panel refines rendering).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DeltaKind {
    Text,
    Thinking,
    ToolCall,
    Other,
}

/// Where an OMP subagent is: OMP's `started` is `Running`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SubagentStatus {
    Running,
    Completed,
    Failed,
    Aborted,
}

impl From<SubagentLifecycleStatus> for SubagentStatus {
    fn from(s: SubagentLifecycleStatus) -> Self {
        match s {
            SubagentLifecycleStatus::Started => Self::Running,
            SubagentLifecycleStatus::Completed => Self::Completed,
            SubagentLifecycleStatus::Failed => Self::Failed,
            SubagentLifecycleStatus::Aborted => Self::Aborted,
        }
    }
}

/// Terminal status of one prompt ticket. Mirrors the wire enum.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PromptStatus {
    Completed,
    Aborted,
    Error,
}

impl From<omp_rpc::wire::PromptStatus> for PromptStatus {
    fn from(s: omp_rpc::wire::PromptStatus) -> Self {
        match s {
            omp_rpc::wire::PromptStatus::Completed => Self::Completed,
            omp_rpc::wire::PromptStatus::Aborted => Self::Aborted,
            omp_rpc::wire::PromptStatus::Error => Self::Error,
        }
    }
}

/// Append-only router log entry: sequence number + event.
#[derive(Debug, Clone)]
pub struct LogEntry {
    /// Monotonic sequence within this router.
    pub seq: u64,
    /// Classified event.
    pub event: RouterEvent,
}

/// A tool call the log saw start and end; see [`EventRouter::finished_tool_calls`].
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct FinishedToolCall {
    pub tool_call_id: String,
    pub tool_name: String,
    pub args_preview: String,
    pub outcome: CallOutcome,
}

/// How a finished call ended. OMP marks both a refused approval and a bash
/// that ran and exited non-zero as `isError`; only the refusal never ran.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum CallOutcome {
    Ok,
    Failed,
    Denied,
}

/// The result text OMP's tool wrapper gives a call whose approval was
/// refused (`extensibility/extensions/wrapper.ts`, pinned 18.8.6).
const DENIED_PREFIX: &str = "Tool call denied by user: ";

/// Single fan-out point: classifies session events, appends to the log, and
/// broadcasts to subscribers. Reader-thread safe (`parking_lot::Mutex`, no await).
pub struct EventRouter {
    inner: Arc<Mutex<Inner>>,
}

struct Inner {
    log: Vec<LogEntry>,
    next_seq: u64,
    subscribers: HashMap<u64, mpsc::Sender<RouterEvent>>,
    next_sub: u64,
    /// `provider/model` of every assistant message this session finished.
    answered: BTreeSet<String>,
}

impl EventRouter {
    /// Empty router with no subscribers.
    pub fn new() -> Self {
        Self {
            inner: Arc::new(Mutex::new(Inner {
                log: Vec::new(),
                next_seq: 0,
                subscribers: HashMap::new(),
                next_sub: 0,
                answered: BTreeSet::new(),
            })),
        }
    }

    /// Classify one notification frame, append to the log, broadcast. Dead
    /// subscribers are pruned. Never fails on unknown input.
    pub fn dispatch_notification(&self, frame: &RpcNotification) {
        // From the message's start too: OMP's Cursor provider runs a whole
        // tool loop inside one assistant message, so its end comes after a
        // review the implementer asked for in that loop.
        let answering = match frame {
            RpcNotification::RpcAgentEvent(RpcAgentEvent::MessageStart(start)) => {
                Some(&start.message)
            }
            RpcNotification::RpcAgentEvent(RpcAgentEvent::MessageEnd(end)) => Some(&end.message),
            _ => None,
        };
        if let Some(AgentMessage::Assistant(message)) = answering {
            if let Some(model) = &message.model {
                let id = match &message.provider {
                    Some(provider) => format!("{provider}/{model}"),
                    None => model.clone(),
                };
                self.inner.lock().answered.insert(id);
            }
        }
        let classified = classify_notification(frame);
        self.push(classified);
    }

    /// The runtime saw OMP's output close without a shutdown.
    pub fn dispatch_disconnected(&self) {
        self.push(RouterEvent::Disconnected);
    }

    /// The models that actually answered in this session, as
    /// `provider/model`, sorted (ADR-0039: independence is measured).
    pub fn answered_models(&self) -> Vec<String> {
        self.inner.lock().answered.iter().cloned().collect()
    }

    /// Subscribe to classified events. Returns (id, receiver); drop the
    /// receiver to unsubscribe (pruned on next dispatch).
    pub fn subscribe(&self) -> (u64, mpsc::Receiver<RouterEvent>) {
        let (tx, rx) = mpsc::channel();
        let mut inner = self.inner.lock();
        let id = inner.next_sub;
        inner.next_sub += 1;
        inner.subscribers.insert(id, tx);
        (id, rx)
    }

    /// Explicit unsubscribe.
    pub fn unsubscribe(&self, id: u64) {
        self.inner.lock().subscribers.remove(&id);
    }

    /// Full log snapshot for crash reconciliation (plan §74: `tool.started`
    /// without `tool.completed` → `INTERRUPTED`).
    pub fn log_snapshot(&self) -> Vec<LogEntry> {
        self.inner.lock().log.clone()
    }

    /// Tool calls started but not ended in log order — the INTERRUPTED set.
    pub fn interrupted_tool_calls(&self) -> Vec<String> {
        let log = self.inner.lock().log.clone();
        let mut started: Vec<String> = Vec::new();
        let mut ended: Vec<String> = Vec::new();
        for entry in &log {
            match &entry.event {
                RouterEvent::ToolStart { tool_call_id, .. } => started.push(tool_call_id.clone()),
                RouterEvent::ToolEnd { tool_call_id, .. } => ended.push(tool_call_id.clone()),
                _ => {}
            }
        }
        started
            .into_iter()
            .filter(|id| !ended.contains(id))
            .collect()
    }

    /// Every call the log saw start and end, in completion order: name and
    /// args preview from its `ToolStart`, the outcome from its `ToolEnd`.
    /// Host-tool evidence binds to these (P5, ADR-0031) — the log is the
    /// only witness, never the agent.
    pub fn finished_tool_calls(&self) -> Vec<FinishedToolCall> {
        let inner = self.inner.lock();
        let mut started: HashMap<&str, (&str, &str)> = HashMap::new();
        let mut done = Vec::new();
        for entry in &inner.log {
            match &entry.event {
                RouterEvent::ToolStart {
                    tool_call_id,
                    tool_name,
                    args_preview,
                    ..
                } => {
                    started.insert(tool_call_id, (tool_name, args_preview));
                }
                RouterEvent::ToolEnd {
                    tool_call_id,
                    tool_name,
                    result_summary,
                    is_error,
                    ..
                } => {
                    let (name, preview) = started
                        .remove(tool_call_id.as_str())
                        .unwrap_or((tool_name, ""));
                    let outcome = match (*is_error, result_summary.starts_with(DENIED_PREFIX)) {
                        (false, _) => CallOutcome::Ok,
                        (true, true) => CallOutcome::Denied,
                        (true, false) => CallOutcome::Failed,
                    };
                    done.push(FinishedToolCall {
                        tool_call_id: tool_call_id.clone(),
                        tool_name: name.to_string(),
                        args_preview: preview.to_string(),
                        outcome,
                    });
                }
                _ => {}
            }
        }
        done
    }

    fn push(&self, event: RouterEvent) {
        let mut inner = self.inner.lock();
        let seq = inner.next_seq;
        inner.next_seq += 1;
        inner.log.push(LogEntry {
            seq,
            event: event.clone(),
        });
        inner
            .subscribers
            .retain(|_, tx| tx.send(event.clone()).is_ok());
    }
}

impl Default for EventRouter {
    fn default() -> Self {
        Self::new()
    }
}

/// Pure classifier: notification frame → router event. Typed `match` over the
/// generated unions; unknown variants stay `Unknown`, never panic.
fn classify_notification(frame: &RpcNotification) -> RouterEvent {
    match frame {
        RpcNotification::RpcAgentEvent(event) => classify_agent_event(event),
        RpcNotification::PromptResult(result) => RouterEvent::PromptResult {
            prompt_id: result.id.clone().unwrap_or_default(),
            status: result.status.into(),
        },
        RpcNotification::SessionSettled(_) => RouterEvent::Settled,
        RpcNotification::SessionInfoUpdate(info) => RouterEvent::SessionInfo {
            session_id: info.session_id.clone(),
            title: info.title.clone(),
        },
        RpcNotification::ExtensionUiRequest(ExtensionUiRequest::Notify(notify)) => {
            RouterEvent::Toast(Toast {
                level: notify.notify_type.map_or(ToastLevel::Info, Into::into),
                text: notify.message.clone(),
            })
        }
        RpcNotification::ExtensionUiRequest(request) => RouterEvent::UiRequest(request.clone()),
        RpcNotification::ExtensionError(e) => RouterEvent::Toast(Toast {
            level: ToastLevel::Error,
            text: format!(
                "extension {} failed on {}: {}",
                e.extension_path, e.event, e.error
            ),
        }),
        RpcNotification::SubagentLifecycle(event) => {
            let p = &event.payload;
            RouterEvent::SubagentLifecycle {
                id: p.id.clone(),
                agent: p.agent.clone(),
                status: p.status.into(),
                parent_tool_call_id: p.parent_tool_call_id.clone(),
                description: p.description.clone(),
            }
        }
        RpcNotification::SubagentProgress(event) => {
            let p = &event.payload;
            let text = |key: &str| p.progress.get(key).and_then(|v| v.as_str());
            match text("id") {
                Some(id) => RouterEvent::SubagentProgress {
                    id: id.to_string(),
                    agent: p.agent.clone(),
                    parent_tool_call_id: p.parent_tool_call_id.clone(),
                    description: text("description").map(str::to_string),
                },
                None => RouterEvent::Unknown {
                    frame_type: "subagent_progress".to_string(),
                },
            }
        }
        RpcNotification::Unknown(raw) => RouterEvent::Unknown {
            frame_type: raw
                .get("type")
                .and_then(|t| t.as_str())
                .unwrap_or("?")
                .to_string(),
        },
        other => RouterEvent::Unknown {
            frame_type: notification_name(other).to_string(),
        },
    }
}

fn notification_name(frame: &RpcNotification) -> &'static str {
    match frame {
        RpcNotification::Ready(_) => "ready",
        RpcNotification::ExtensionError(_) => "extension_error",
        RpcNotification::ExtensionUiRequest(_) => "extension_ui_request",
        RpcNotification::AvailableCommandsUpdate(_) => "available_commands_update",
        RpcNotification::SubagentLifecycle(_) => "subagent_lifecycle",
        RpcNotification::SubagentProgress(_) => "subagent_progress",
        RpcNotification::SubagentEvent(_) => "subagent_event",
        RpcNotification::CommandOutput(_) => "command_output",
        RpcNotification::SessionInfoUpdate(_) => "session_info_update",
        RpcNotification::ConfigUpdate(_) => "config_update",
        RpcNotification::RpcFrameError(_) => "rpc_frame_error",
        _ => "other",
    }
}

/// A file's text before an edit call, from OMP 18.6.1's result `details`:
/// `oldText` is the whole file before the call (OMP diffs it against
/// `newText` and re-parses it to catch a syntax regression), absent for a
/// file the call created; once a result's texts pass 32 KiB OMP drops them
/// and says `snapshotsPruned`.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum TextBefore {
    Text(String),
    NewFile,
    Pruned,
    /// A move: OMP's `oldText` is the source's text, so neither the source
    /// nor the destination has a baseline of its own.
    Moved,
}

/// The before-texts in an edit result's `details`: one file's fields, or
/// a `perFileResults` list of them.
pub fn texts_before(result: Option<&serde_json::Value>) -> Vec<(String, TextBefore)> {
    let Some(details) = result.and_then(|r| r.get("details")) else {
        return Vec::new();
    };
    let files: Vec<&serde_json::Value> = match details.get("perFileResults") {
        Some(serde_json::Value::Array(files)) => files.iter().collect(),
        _ => vec![details],
    };
    files
        .into_iter()
        .filter_map(|file| {
            let path = file.get("path").and_then(|p| p.as_str())?.to_string();
            if let Some(source) = file.get("sourcePath").and_then(|p| p.as_str()) {
                return Some(vec![
                    (path, TextBefore::Moved),
                    (source.to_string(), TextBefore::Moved),
                ]);
            }
            let before = if file.get("snapshotsPruned").and_then(|v| v.as_bool()) == Some(true) {
                TextBefore::Pruned
            } else if let Some(text) = file.get("oldText").and_then(|v| v.as_str()) {
                TextBefore::Text(text.to_string())
            } else if file.get("newText").is_some_and(|v| v.is_string()) {
                TextBefore::NewFile
            } else {
                return None;
            };
            Some(vec![(path, before)])
        })
        .flatten()
        .collect()
}

/// Pure classifier: session event → router event.
fn classify_agent_event(event: &RpcAgentEvent) -> RouterEvent {
    match event {
        RpcAgentEvent::AgentStart(_) => RouterEvent::AgentStart,
        RpcAgentEvent::AgentEnd(end) => RouterEvent::AgentEnd {
            yielded: end.yielded.unwrap_or(true),
            is_terminal: end.is_terminal.unwrap_or(true),
        },
        RpcAgentEvent::MessageUpdate(update) => {
            let (kind, delta) = match &update.assistant_message_event {
                AssistantMessageEvent::TextDelta(d) => {
                    (DeltaKind::Text, d.delta.clone().unwrap_or_default())
                }
                AssistantMessageEvent::TextStart(_) | AssistantMessageEvent::TextEnd(_) => {
                    (DeltaKind::Text, String::new())
                }
                AssistantMessageEvent::ThinkingDelta(d) => {
                    (DeltaKind::Thinking, d.delta.clone().unwrap_or_default())
                }
                AssistantMessageEvent::ThinkingStart(_) | AssistantMessageEvent::ThinkingEnd(_) => {
                    (DeltaKind::Thinking, String::new())
                }
                AssistantMessageEvent::ToolcallStart(_)
                | AssistantMessageEvent::ToolcallDelta(_)
                | AssistantMessageEvent::ToolcallEnd(_) => (DeltaKind::ToolCall, String::new()),
                _ => (DeltaKind::Other, String::new()),
            };
            RouterEvent::MessageDelta {
                message_id: update.message_id.clone().unwrap_or_default(),
                kind,
                delta,
            }
        }
        RpcAgentEvent::MessageEnd(end) => RouterEvent::MessageEnd {
            message_id: end.message_id.clone().unwrap_or_default(),
        },
        RpcAgentEvent::ToolExecutionStart(start) => RouterEvent::ToolStart {
            tool_call_id: start.tool_call_id.clone(),
            tool_name: start.tool_name.clone(),
            args_preview: summarize_args(&start.tool_name, start.args.as_ref()),
            paths: written_paths(&start.tool_name, start.args.as_ref()),
        },
        RpcAgentEvent::ToolExecutionEnd(end) => RouterEvent::ToolEnd {
            tool_call_id: end.tool_call_id.clone(),
            tool_name: end.tool_name.clone(),
            result_summary: summarize_result(&end.tool_name, end.result.as_ref()),
            is_error: end.is_error.unwrap_or(false),
            before: texts_before(end.result.as_ref()),
        },
        RpcAgentEvent::AutoCompactionStart(e) => RouterEvent::RunNote {
            text: format!("compacting ({}, {})", e.reason.as_str(), e.action.as_str()),
            retrying: None,
        },
        RpcAgentEvent::AutoCompactionEnd(e) => RouterEvent::RunNote {
            text: if e.aborted {
                "compaction aborted".to_string()
            } else if let Some(error) = &e.error_message {
                format!("compaction failed: {error}")
            } else if e.skipped == Some(true) {
                "compaction skipped".to_string()
            } else {
                let tokens = e.result.as_ref().map_or(0, |r| r.tokens_before);
                format!("compacted {tokens} tokens")
            },
            retrying: None,
        },
        RpcAgentEvent::AutoRetryStart(e) => RouterEvent::RunNote {
            text: format!(
                "retry {}/{} in {} ms: {}",
                e.attempt, e.max_attempts, e.delay_ms, e.error_message
            ),
            retrying: Some(true),
        },
        RpcAgentEvent::AutoRetryEnd(e) => RouterEvent::RunNote {
            text: match (&e.final_error, e.success) {
                (_, true) => format!("retry {} succeeded", e.attempt),
                (Some(error), false) => format!("retry gave up after {}: {error}", e.attempt),
                (None, false) => format!("retry gave up after {}", e.attempt),
            },
            retrying: Some(false),
        },
        RpcAgentEvent::RetryFallbackApplied(e) => RouterEvent::RunNote {
            text: format!(
                "{} falls back from {} to {}{}",
                e.role,
                e.from,
                e.to,
                e.reason
                    .as_ref()
                    .map(|r| format!(": {r}"))
                    .unwrap_or_default()
            ),
            retrying: None,
        },
        RpcAgentEvent::RetryFallbackSucceeded(e) => RouterEvent::RunNote {
            text: format!("{} answered on fallback {}", e.role, e.model),
            retrying: None,
        },
        RpcAgentEvent::ModelChanged(_) => RouterEvent::ModelChanged,
        RpcAgentEvent::ThinkingLevelChanged(event) => {
            RouterEvent::ThinkingLevel(event.thinking_level)
        }
        RpcAgentEvent::Notice(notice) => RouterEvent::Toast(Toast {
            level: notice.level.into(),
            text: match &notice.source {
                Some(source) => format!("{source}: {}", notice.message),
                None => notice.message.clone(),
            },
        }),
        // The event carries no warnings; OMP lists them in its own UI.
        RpcAgentEvent::ConfigWarningsChanged(_) => RouterEvent::Toast(Toast {
            level: ToastLevel::Warning,
            text: "OMP's config warnings changed (retry fallback chains); run omp to see them"
                .to_string(),
        }),
        RpcAgentEvent::TtsrTriggered(event) => {
            let names: Vec<&str> = event
                .rules
                .iter()
                .filter_map(|rule| rule.get("name").and_then(|n| n.as_str()))
                .collect();
            RouterEvent::Toast(Toast {
                level: ToastLevel::Info,
                text: format!("OMP rule triggered mid-stream: {}", names.join(", ")),
            })
        }
        RpcAgentEvent::QueueUpdate(queue) => RouterEvent::Queue {
            steering: queue.steering.clone(),
            follow_up: queue.follow_up.clone(),
        },
        _ => RouterEvent::Unknown {
            frame_type: "agent_event".to_string(),
        },
    }
}
/// One-line human summary of tool args — never raw JSON. Per-tool shapes from
/// `get_state.dumpTools` parameters; unknown tools fall back to key names.
/// The files an edit-class tool writes. `write` and `ast_edit` name one
/// `path`; `edit` takes a script whose `[path#hash]` headers name each file.
/// A URI is no file: OMP 18.6.1 mounts host tools as `xd://` devices that
/// a `read` or `write` calls.
pub fn written_paths(name: &str, args: Option<&serde_json::Value>) -> Vec<String> {
    let mut paths = edit_targets(name, args);
    paths.retain(|p| !p.starts_with("xd://"));
    paths
}

fn edit_targets(name: &str, args: Option<&serde_json::Value>) -> Vec<String> {
    let Some(args) = args else {
        return Vec::new();
    };
    let path = || {
        args.get("path")
            .and_then(|v| v.as_str())
            .map(str::to_string)
            .into_iter()
            .collect()
    };
    match name {
        "write" | "ast_edit" => path(),
        "edit" => {
            let input = args.get("input").and_then(|v| v.as_str()).unwrap_or("");
            let mut paths: Vec<String> = Vec::new();
            for line in input.lines() {
                let Some(body) = line.strip_prefix('[').and_then(|l| l.strip_suffix(']')) else {
                    continue;
                };
                match body.rsplit_once('#') {
                    Some((path, _hash)) if !paths.iter().any(|p| p == path) => {
                        paths.push(path.to_string())
                    }
                    _ => {}
                }
            }
            paths
        }
        _ => Vec::new(),
    }
}

fn summarize_args(name: &str, args: Option<&serde_json::Value>) -> String {
    let Some(args) = args else {
        return String::new();
    };
    let get = |key: &str| args.get(key).and_then(|v| v.as_str()).unwrap_or("");
    // One display line, ~120 chars: long paths (e.g. $TMPDIR) must survive.
    let first_line = |s: &str| {
        s.lines()
            .next()
            .unwrap_or("")
            .chars()
            .take(120)
            .collect::<String>()
    };
    let preview: String = match name {
        "read" | "write" | "glob" => get("path").to_string(),
        "edit" => get("input")
            .lines()
            .next()
            .unwrap_or("")
            .chars()
            .take(60)
            .collect(),
        "bash" => first_line(get("command")),
        "grep" => format!("{} in {}", get("pattern"), get("path")),
        "find" => get("query").chars().take(80).collect(),
        "eval" => format!("{}: {}", get("language"), first_line(get("code"))),
        "todo" => format!("{} {}", get("op"), get("task")),
        "web_search" => get("query").chars().take(80).collect(),
        _ => args
            .as_object()
            .map(|o| o.keys().take(3).cloned().collect::<Vec<_>>().join(", "))
            .unwrap_or_default(),
    };
    preview.trim().to_string()
}

/// One-line human summary of a tool result — never raw JSON. Prefers counts
/// and first lines over dumps; truncates to one line.
fn summarize_result(name: &str, result: Option<&serde_json::Value>) -> String {
    let Some(result) = result else {
        return String::new();
    };
    let content_texts: Vec<&str> = result
        .get("content")
        .and_then(|c| c.as_array())
        .map(|items| {
            items
                .iter()
                .filter_map(|i| i.get("text").and_then(|t| t.as_str()))
                .collect()
        })
        .unwrap_or_default();
    let joined = content_texts.join("\n");
    if joined.starts_with(DENIED_PREFIX) {
        return joined.lines().next().unwrap_or("").to_string();
    }
    let line_count = joined.lines().count();
    match name {
        "bash" | "eval" => {
            // OMP 18.6 ends the output with a blank-separated `Wall time:` line.
            let mut output: Vec<&str> = joined.lines().collect();
            if output.last().is_some_and(|l| l.starts_with("Wall time: ")) {
                output.pop();
            }
            while output.last().is_some_and(|l| l.trim().is_empty()) {
                output.pop();
            }
            match output.as_slice() {
                [] => String::new(),
                [only] => only.chars().take(100).collect(),
                [.., last] => format!("{} lines, last: {last}", output.len()),
            }
        }
        "grep" | "glob" | "find" => {
            if line_count > 1 {
                format!("{line_count} matches")
            } else {
                joined
                    .lines()
                    .next()
                    .unwrap_or("")
                    .chars()
                    .take(100)
                    .collect()
            }
        }
        _ => joined
            .lines()
            .next()
            .unwrap_or("")
            .chars()
            .take(100)
            .collect(),
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn an_edit_result_names_each_files_text_before_the_call() {
        use super::TextBefore;
        use serde_json::json;
        let one = json!({"details": {"path": "/ws/a.txt", "oldText": "a\n", "newText": "A\n"}});
        assert_eq!(
            super::texts_before(Some(&one)),
            vec![("/ws/a.txt".to_string(), TextBefore::Text("a\n".to_string()))]
        );
        let many = json!({"details": {"perFileResults": [
            {"path": "/ws/new.txt", "newText": "n\n"},
            {"path": "/ws/big.txt", "snapshotsPruned": true},
            {"path": "/ws/moved.txt", "diff": "-x"}
        ]}});
        assert_eq!(
            super::texts_before(Some(&many)),
            vec![
                ("/ws/new.txt".to_string(), TextBefore::NewFile),
                ("/ws/big.txt".to_string(), TextBefore::Pruned),
            ]
        );
    }

    #[test]
    fn written_paths_name_every_file_an_edit_tool_touches() {
        use serde_json::json;
        let edit = json!({"input": "[notes.txt#6193]\nPUT 2.=2:\n+BETA\n[src/a.rs#0585]\nPUT 1.=1:\n+x\n[notes.txt#6193]\n"});
        assert_eq!(
            super::written_paths("edit", Some(&edit)),
            vec!["notes.txt".to_string(), "src/a.rs".to_string()]
        );
        let write = json!({"path": "new.txt", "content": "x"});
        assert_eq!(
            super::written_paths("write", Some(&write)),
            vec!["new.txt".to_string()]
        );
        assert_eq!(
            super::written_paths("ast_edit", Some(&write)),
            vec!["new.txt".to_string()]
        );
        assert!(super::written_paths("bash", Some(&json!({"command": "rm notes.txt"}))).is_empty());
        assert!(super::written_paths("edit", None).is_empty());
        let device = json!({"path": "xd://cedian_complete", "content": "{}"});
        assert!(super::texts_before(Some(&json!({"content": []}))).is_empty());
        assert!(
            super::written_paths("write", Some(&device)).is_empty(),
            "a write to an xd:// device is a host tool call, not a file"
        );
        let file_uri = json!({"path": "file:///ws/a.txt", "content": "x"});
        assert_eq!(
            super::written_paths("write", Some(&file_uri)),
            vec!["file:///ws/a.txt".to_string()],
            "only xd:// is a device"
        );
    }

    /// A move's `oldText` is its source's text: neither path has a
    /// before-text of its own.
    #[test]
    fn a_moved_file_has_no_text_before_of_its_own() {
        use super::TextBefore;
        use serde_json::json;
        let moved = json!({"details": {"path": "/ws/b.txt", "move": "/ws/b.txt",
            "sourcePath": "/ws/a.txt", "oldText": "a\n", "newText": "a\n"}});
        assert_eq!(
            super::texts_before(Some(&moved)),
            vec![
                ("/ws/b.txt".to_string(), TextBefore::Moved),
                ("/ws/a.txt".to_string(), TextBefore::Moved),
            ]
        );
    }

    use super::*;
    use omp_rpc::wire::{PromptResultEvent, SessionSettledEvent};

    fn agent_event(json: serde_json::Value) -> RpcAgentEvent {
        RpcAgentEvent::from_value(json).expect("valid agent event")
    }

    #[test]
    fn finished_tool_call_needs_a_tool_end() {
        let router = EventRouter::new();
        let start = |id: &str, name: &str| RouterEvent::ToolStart {
            tool_call_id: id.into(),
            tool_name: name.into(),
            args_preview: format!("{name} args"),
            paths: Vec::new(),
        };
        let end = |id: &str, name: &str, is_error| RouterEvent::ToolEnd {
            tool_call_id: id.into(),
            tool_name: name.into(),
            result_summary: String::new(),
            is_error,
            before: Vec::new(),
        };
        router.push(start("ok", "bash"));
        router.push(start("bad", "bash"));
        router.push(start("open", "bash"));
        router.push(end("ok", "bash", false));
        router.push(end("bad", "bash", true));
        let done = router.finished_tool_calls();
        assert_eq!(
            done[0],
            FinishedToolCall {
                tool_call_id: "ok".into(),
                tool_name: "bash".into(),
                args_preview: "bash args".into(),
                outcome: CallOutcome::Ok,
            }
        );
        assert_eq!(done[1].outcome, CallOutcome::Failed);
        assert_eq!(done.len(), 2, "a started-only call is not finished");
    }

    #[test]
    fn a_refused_approval_is_denied_and_a_nonzero_exit_failed() {
        let end = |id: &str, tool: &str, text: &str| {
            classify_agent_event(&agent_event(serde_json::json!({
                "type": "tool_execution_end", "toolCallId": id, "toolName": tool,
                "result": {"content": [{"type": "text", "text": text}], "details": {}},
                "isError": true,
            })))
        };
        let router = EventRouter::new();
        router.push(end("denied", "edit", "Tool call denied by user: edit"));
        router.push(end(
            "failed",
            "bash",
            "wrote out.txt\n\nCommand exited with code 1",
        ));
        let outcomes: Vec<_> = router
            .finished_tool_calls()
            .into_iter()
            .map(|call| (call.tool_call_id, call.outcome))
            .collect();
        assert_eq!(
            outcomes,
            vec![
                ("denied".to_string(), CallOutcome::Denied),
                ("failed".to_string(), CallOutcome::Failed),
            ]
        );
    }

    #[test]
    fn text_delta_carries_text_and_message_end_classified() {
        let message = serde_json::json!({"role": "assistant", "content": [], "timestamp": 0});
        let update = agent_event(serde_json::json!({
            "type": "message_update",
            "messageId": "m1",
            "message": message,
            "assistantMessageEvent": {"type": "text_delta", "contentIndex": 0, "delta": "hel"},
        }));
        match classify_agent_event(&update) {
            RouterEvent::MessageDelta {
                message_id,
                kind,
                delta,
            } => {
                assert_eq!(message_id, "m1");
                assert_eq!(kind, DeltaKind::Text);
                assert_eq!(delta, "hel");
            }
            other => panic!("unexpected {other:?}"),
        }
        let end = agent_event(serde_json::json!({
            "type": "message_end",
            "messageId": "m1",
            "message": message,
        }));
        assert!(matches!(
            classify_agent_event(&end),
            RouterEvent::MessageEnd { message_id } if message_id == "m1"
        ));
    }

    #[test]
    fn models_that_answered_are_recorded_from_assistant_messages() {
        let router = EventRouter::new();
        let end = |role: &str, provider: &str, model: &str| {
            RpcNotification::RpcAgentEvent(agent_event(serde_json::json!({
                "type": "message_end",
                "messageId": "m1",
                "message": {"role": role, "content": [], "timestamp": 0,
                            "provider": provider, "model": model},
            })))
        };
        router.dispatch_notification(&end("assistant", "opencode-go", "muse-spark-1.3"));
        router.dispatch_notification(&end("assistant", "opencode-go", "muse-spark-1.3"));
        router.dispatch_notification(&end("assistant", "opencode-go", "glm-5.3"));
        assert_eq!(
            router.answered_models(),
            ["opencode-go/glm-5.3", "opencode-go/muse-spark-1.3"],
            "each model once, sorted"
        );
    }

    /// OMP's Cursor provider (`api: cursor-agent`) calls tools inside one
    /// assistant message: a review asked for mid-message must already know
    /// the model (tests/fixtures/s3_same_model.jsonl, records 24 and 237).
    #[test]
    fn a_model_answers_from_its_messages_start() {
        let router = EventRouter::new();
        router.dispatch_notification(&RpcNotification::RpcAgentEvent(agent_event(
            serde_json::json!({
                "type": "message_start",
                "message": {"role": "assistant", "content": [], "timestamp": 0,
                            "api": "cursor-agent", "provider": "cursor", "model": "default"},
            }),
        )));
        assert_eq!(router.answered_models(), ["cursor/default"]);
    }

    #[test]
    fn subagent_frames_are_typed_and_progress_takes_its_id_from_progress() {
        let frame = |json: serde_json::Value| RpcNotification::from_value(json).unwrap();
        let lifecycle = frame(serde_json::json!({
            "type": "subagent_lifecycle",
            "payload": {"id": "sa-1", "agent": "explore", "agentSource": "bundled",
                        "status": "started", "index": 0, "parentToolCallId": "call-task",
                        "description": "map the router"},
        }));
        match classify_notification(&lifecycle) {
            RouterEvent::SubagentLifecycle {
                id,
                agent,
                status,
                parent_tool_call_id,
                description,
            } => {
                assert_eq!((id.as_str(), agent.as_str()), ("sa-1", "explore"));
                assert_eq!(status, SubagentStatus::Running);
                assert_eq!(parent_tool_call_id.as_deref(), Some("call-task"));
                assert_eq!(description.as_deref(), Some("map the router"));
            }
            other => panic!("unexpected {other:?}"),
        }
        let progress = frame(serde_json::json!({
            "type": "subagent_progress",
            "payload": {"index": 0, "agent": "explore", "agentSource": "bundled",
                        "task": "long assignment", "parentToolCallId": "call-task",
                        "progress": {"id": "sa-1", "status": "running", "description": "reading"}},
        }));
        match classify_notification(&progress) {
            RouterEvent::SubagentProgress {
                id, description, ..
            } => {
                assert_eq!(id, "sa-1");
                assert_eq!(description.as_deref(), Some("reading"));
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn interrupted_set_empty_when_balanced() {
        let r = EventRouter::new();
        assert!(r.interrupted_tool_calls().is_empty());
    }

    #[test]
    fn prompt_result_classified() {
        let r = EventRouter::new();
        let (_id, rx) = r.subscribe();
        r.dispatch_notification(&RpcNotification::PromptResult(PromptResultEvent {
            agent_invoked: true,
            status: omp_rpc::wire::PromptStatus::Aborted,
            session_settled: true,
            id: Some("p1".to_string()),
            error: None,
        }));
        match rx.recv_timeout(std::time::Duration::from_secs(1)).unwrap() {
            RouterEvent::PromptResult { prompt_id, status } => {
                assert_eq!(prompt_id, "p1");
                assert_eq!(status, PromptStatus::Aborted);
            }
            other => panic!("unexpected {other:?}"),
        }
    }

    #[test]
    fn settled_classified() {
        let r = EventRouter::new();
        let (_id, rx) = r.subscribe();
        r.dispatch_notification(&RpcNotification::SessionSettled(SessionSettledEvent {}));
        assert!(matches!(
            rx.recv_timeout(std::time::Duration::from_secs(1)).unwrap(),
            RouterEvent::Settled
        ));
    }

    #[test]
    fn dead_subscriber_pruned() {
        let r = EventRouter::new();
        let (_id, rx) = r.subscribe();
        drop(rx);
        r.dispatch_notification(&RpcNotification::SessionSettled(SessionSettledEvent {}));
        assert_eq!(r.inner.lock().subscribers.len(), 0);
    }
    #[test]
    fn args_preview_never_raw_json() {
        let args = serde_json::json!({"command": "cargo test --workspace", "cwd": "/w"});
        assert_eq!(
            summarize_args("bash", Some(&args)),
            "cargo test --workspace"
        );
        let args = serde_json::json!({"path": "src/main.rs:10-20"});
        assert_eq!(summarize_args("read", Some(&args)), "src/main.rs:10-20");
        let args = serde_json::json!({"pattern": "TODO", "path": "src/"});
        assert_eq!(summarize_args("grep", Some(&args)), "TODO in src/");
        // Unknown tool: key names, never a JSON dump.
        let args = serde_json::json!({"zzz": 1, "aaa": 2});
        let preview = summarize_args("future_tool", Some(&args));
        assert!(!preview.contains('{'), "got {preview:?}");
        assert!(summarize_args("read", None).is_empty());
    }

    #[test]
    fn result_summary_prefers_counts() {
        let multi = serde_json::json!({"content": [{"type": "text", "text": "a\nb\nc"}]});
        assert_eq!(summarize_result("grep", Some(&multi)), "3 matches");
        let single = serde_json::json!({"content": [{"type": "text", "text": "hello"}]});
        assert_eq!(summarize_result("read", Some(&single)), "hello");
        assert!(summarize_result("bash", None).is_empty());
    }

    #[test]
    fn bash_summary_counts_blank_lines_and_skips_only_the_wall_time() {
        let out =
            serde_json::json!({"content": [{"type": "text", "text": "a\n\nb\n\nWall time: 0.1s"}]});
        assert_eq!(summarize_result("bash", Some(&out)), "3 lines, last: b");
    }
}
