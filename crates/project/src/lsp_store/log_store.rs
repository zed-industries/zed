use std::{
    collections::VecDeque,
    sync::Arc,
    time::{Duration, Instant},
};

use collections::HashMap;
use futures::{StreamExt, channel::mpsc};
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, Global, Subscription, TaskExt, WeakEntity,
};
use indexmap::IndexMap;
use lsp::{
    IoKind, LanguageServer, LanguageServerId, LanguageServerName, LanguageServerSelector,
    MessageType, RequestId, TraceValue,
};
use rpc::proto;
use serde::Deserialize;
use settings::WorktreeId;

use crate::{LanguageServerLogType, LspStore, Project, ProjectItem as _};

const MAX_STORED_LOG_ENTRIES: usize = 2000;
const MAX_PENDING_REQUESTS: usize = MAX_STORED_LOG_ENTRIES;
const MAX_RETAINED_STOPPED_SERVERS: usize = 16;

pub fn init(on_headless_host: bool, cx: &mut App) -> Entity<LogStore> {
    let log_store = cx.new(|cx| LogStore::new(on_headless_host, cx));
    cx.set_global(GlobalLogStore(log_store.clone()));
    log_store
}

pub struct GlobalLogStore(pub Entity<LogStore>);

impl Global for GlobalLogStore {}

#[derive(Debug)]
pub enum Event {
    NewServerLogEntry {
        id: LanguageServerId,
        kind: LanguageServerLogType,
        text: String,
    },
}

impl EventEmitter<Event> for LogStore {}

pub struct LogStore {
    on_headless_host: bool,
    projects: HashMap<WeakEntity<Project>, ProjectState>,
    pub language_servers: HashMap<LanguageServerId, LanguageServerState>,
    pub stopped_language_servers:
        IndexMap<(LanguageServerName, Option<WorktreeId>), LanguageServerState>,
    io_tx: mpsc::UnboundedSender<(LanguageServerId, IoKind, String, Instant)>,
}

struct ProjectState {
    _subscriptions: [Subscription; 2],
    copilot_log_subscription: Option<lsp::Subscription>,
}

pub trait Message: AsRef<str> {
    type Level: Copy + std::fmt::Debug;
    fn should_include(&self, _: Self::Level) -> bool {
        true
    }
}

#[derive(Debug)]
pub struct LogMessage {
    message: String,
    typ: MessageType,
}

impl AsRef<str> for LogMessage {
    fn as_ref(&self) -> &str {
        &self.message
    }
}

impl Message for LogMessage {
    type Level = MessageType;

    fn should_include(&self, level: Self::Level) -> bool {
        match (self.typ, level) {
            (MessageType::ERROR, _) => true,
            (_, MessageType::ERROR) => false,
            (MessageType::WARNING, _) => true,
            (_, MessageType::WARNING) => false,
            (MessageType::INFO, _) => true,
            (_, MessageType::INFO) => false,
            _ => true,
        }
    }
}

#[derive(Debug)]
pub struct TraceMessage {
    message: String,
    is_verbose: bool,
}

impl AsRef<str> for TraceMessage {
    fn as_ref(&self) -> &str {
        &self.message
    }
}

impl Message for TraceMessage {
    type Level = TraceValue;

    fn should_include(&self, level: Self::Level) -> bool {
        match level {
            TraceValue::Off => false,
            TraceValue::Messages => !self.is_verbose,
            TraceValue::Verbose => true,
        }
    }
}

#[derive(Debug)]
pub struct RpcMessage {
    message: String,
}

impl AsRef<str> for RpcMessage {
    fn as_ref(&self) -> &str {
        &self.message
    }
}

impl Message for RpcMessage {
    type Level = ();
}

pub struct LanguageServerState {
    pub server_id: LanguageServerId,
    pub name: Option<LanguageServerName>,
    pub worktree_id: Option<WorktreeId>,
    pub kind: LanguageServerKind,
    log_messages: VecDeque<LogMessage>,
    trace_messages: VecDeque<TraceMessage>,
    pub rpc_state: Option<LanguageServerRpcState>,
    pub trace_level: TraceValue,
    pub log_level: MessageType,
    io_logs_subscription: Option<lsp::Subscription>,
    pub toggled_log_kind: Option<LogKind>,
}

impl std::fmt::Debug for LanguageServerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LanguageServerState")
            .field("server_id", &self.server_id)
            .field("name", &self.name)
            .field("worktree_id", &self.worktree_id)
            .field("kind", &self.kind)
            .field("log_messages", &self.log_messages)
            .field("trace_messages", &self.trace_messages)
            .field("rpc_state", &self.rpc_state)
            .field("trace_level", &self.trace_level)
            .field("log_level", &self.log_level)
            .field("toggled_log_kind", &self.toggled_log_kind)
            .finish_non_exhaustive()
    }
}

#[derive(PartialEq, Clone)]
pub enum LanguageServerKind {
    Local { project: WeakEntity<Project> },
    Remote { project: WeakEntity<Project> },
    LocalSsh { lsp_store: WeakEntity<LspStore> },
    Global,
}

impl std::fmt::Debug for LanguageServerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageServerKind::Local { .. } => write!(f, "LanguageServerKind::Local"),
            LanguageServerKind::Remote { .. } => write!(f, "LanguageServerKind::Remote"),
            LanguageServerKind::LocalSsh { .. } => write!(f, "LanguageServerKind::LocalSsh"),
            LanguageServerKind::Global => write!(f, "LanguageServerKind::Global"),
        }
    }
}

impl LanguageServerKind {
    pub fn project(&self) -> Option<&WeakEntity<Project>> {
        match self {
            Self::Local { project } => Some(project),
            Self::Remote { project } => Some(project),
            Self::LocalSsh { .. } => None,
            Self::Global { .. } => None,
        }
    }
}

#[derive(Debug)]
pub struct LanguageServerRpcState {
    pub rpc_messages: VecDeque<RpcMessage>,
    header_state: RpcLogHeaderState,
    request_tracker: RpcRequestTracker,
}

#[derive(Debug, Default)]
struct RpcLogHeaderState {
    last_message_kind: Option<MessageKind>,
    last_message_had_elapsed: bool,
}

#[derive(Debug, Default)]
struct RpcRequestTracker {
    pending_requests: HashMap<PendingRequestKey, Instant>,
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct PendingRequestKey {
    kind: MessageKind,
    id: RequestId,
}

#[derive(Deserialize)]
struct RpcEnvelope<'a> {
    id: Option<RequestId>,
    method: Option<&'a str>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
enum MessageKind {
    Send,
    Receive,
}

impl MessageKind {
    fn opposite(self) -> Self {
        match self {
            Self::Send => Self::Receive,
            Self::Receive => Self::Send,
        }
    }
}

impl RpcLogHeaderState {
    fn header_for_message(
        &mut self,
        kind: MessageKind,
        elapsed: Option<Duration>,
    ) -> Option<String> {
        let starts_new_group = self.last_message_kind != Some(kind)
            || self.last_message_had_elapsed
            || elapsed.is_some();
        self.last_message_kind = Some(kind);
        self.last_message_had_elapsed = elapsed.is_some();

        starts_new_group.then(|| {
            let direction = if kind == MessageKind::Receive {
                "Receive"
            } else {
                "Send"
            };
            match elapsed {
                Some(elapsed) => format!("\n// {direction} (took {}):", format_duration(elapsed)),
                None => format!("\n// {direction}:"),
            }
        })
    }
}

#[cfg(feature = "test-support")]
#[derive(Default)]
pub struct TestRpcLogHeaderState(RpcLogHeaderState);

#[cfg(feature = "test-support")]
impl TestRpcLogHeaderState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn header_for_message(
        &mut self,
        received: bool,
        elapsed: Option<Duration>,
    ) -> Option<String> {
        let kind = if received {
            MessageKind::Receive
        } else {
            MessageKind::Send
        };
        self.0.header_for_message(kind, elapsed)
    }
}

impl RpcRequestTracker {
    fn observe(
        &mut self,
        kind: MessageKind,
        message: &str,
        observed_at: Instant,
    ) -> Option<Duration> {
        let envelope = serde_json::from_str::<RpcEnvelope>(message).ok()?;
        let id = envelope.id?;
        if envelope.method.is_some() {
            self.insert(PendingRequestKey { kind, id }, observed_at);
            None
        } else {
            self.pending_requests
                .remove(&PendingRequestKey {
                    kind: kind.opposite(),
                    id,
                })
                .and_then(|started_at| observed_at.checked_duration_since(started_at))
        }
    }

    fn insert(&mut self, key: PendingRequestKey, observed_at: Instant) {
        if self.pending_requests.len() >= MAX_PENDING_REQUESTS
            && !self.pending_requests.contains_key(&key)
            && let Some(oldest_key) = self
                .pending_requests
                .iter()
                .min_by_key(|(_, started_at)| **started_at)
                .map(|(key, _)| key.clone())
        {
            self.pending_requests.remove(&oldest_key);
        }
        self.pending_requests.insert(key, observed_at);
    }
}

#[cfg(feature = "test-support")]
#[derive(Default)]
pub struct TestRpcRequestTracker(RpcRequestTracker);

#[cfg(feature = "test-support")]
impl TestRpcRequestTracker {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn observe(
        &mut self,
        received: bool,
        message: &str,
        observed_at: Instant,
    ) -> Option<Duration> {
        let kind = if received {
            MessageKind::Receive
        } else {
            MessageKind::Send
        };
        self.0.observe(kind, message, observed_at)
    }

    pub fn pending_request_count(&self) -> usize {
        self.0.pending_requests.len()
    }

    pub fn max_pending_requests() -> usize {
        MAX_PENDING_REQUESTS
    }
}

enum RpcTiming {
    ObservedAt(Instant),
    Forwarded(Option<Duration>),
}

fn format_duration(duration: Duration) -> String {
    let seconds = duration.as_secs_f64();
    if seconds < 0.001 {
        format!("{:.0}µs", seconds * 1_000_000.0)
    } else if seconds < 1.0 {
        format!("{:.1}ms", seconds * 1_000.0)
    } else {
        format!("{seconds:.2}s")
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub enum LogKind {
    Rpc,
    Trace,
    #[default]
    Logs,
    ServerInfo,
}

impl LogKind {
    pub fn from_server_log_type(log_type: &LanguageServerLogType) -> Self {
        match log_type {
            LanguageServerLogType::Log(_) => Self::Logs,
            LanguageServerLogType::Trace { .. } => Self::Trace,
            LanguageServerLogType::Rpc { .. } => Self::Rpc,
        }
    }
}

impl LogStore {
    pub fn new(on_headless_host: bool, cx: &mut Context<Self>) -> Self {
        let (io_tx, mut io_rx) = mpsc::unbounded();

        let log_store = Self {
            projects: HashMap::default(),
            language_servers: HashMap::default(),
            stopped_language_servers: IndexMap::default(),

            on_headless_host,
            io_tx,
        };
        cx.spawn(async move |log_store, cx| {
            while let Some((server_id, io_kind, message, observed_at)) = io_rx.next().await {
                if let Some(log_store) = log_store.upgrade() {
                    log_store.update(cx, |log_store, cx| {
                        log_store.on_io(server_id, io_kind, &message, observed_at, cx);
                    });
                }
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);

        log_store
    }

    pub fn add_project(&mut self, project: &Entity<Project>, cx: &mut Context<Self>) {
        let weak_project = project.downgrade();
        self.projects.insert(
            project.downgrade(),
            ProjectState {
                _subscriptions: [
                    cx.observe_release(project, move |this, _, _| {
                        this.projects.remove(&weak_project);
                        this.language_servers
                            .retain(|_, state| state.kind.project() != Some(&weak_project));
                        this.stopped_language_servers
                            .retain(|(_, _), state| state.kind.project() != Some(&weak_project));
                    }),
                    cx.subscribe(project, move |log_store, project, event, cx| {
                        let server_kind = if project.read(cx).is_local() {
                            LanguageServerKind::Local {
                                project: project.downgrade(),
                            }
                        } else {
                            LanguageServerKind::Remote {
                                project: project.downgrade(),
                            }
                        };
                        match event {
                            crate::Event::LanguageServerAdded(id, name, worktree_id) => {
                                log_store.add_language_server(
                                    server_kind,
                                    *id,
                                    Some(name.clone()),
                                    *worktree_id,
                                    project
                                        .read(cx)
                                        .lsp_store()
                                        .read(cx)
                                        .language_server_for_id(*id),
                                    cx,
                                );
                            }
                            crate::Event::LanguageServerBufferRegistered {
                                server_id,
                                buffer_id,
                                name,
                                ..
                            } => {
                                let worktree_id = project
                                    .read(cx)
                                    .buffer_for_id(*buffer_id, cx)
                                    .and_then(|buffer| {
                                        Some(buffer.read(cx).project_path(cx)?.worktree_id)
                                    });
                                let name = name.clone().or_else(|| {
                                    project
                                        .read(cx)
                                        .lsp_store()
                                        .read(cx)
                                        .language_server_statuses
                                        .get(server_id)
                                        .map(|status| status.name.clone())
                                });
                                log_store.add_language_server(
                                    server_kind,
                                    *server_id,
                                    name,
                                    worktree_id,
                                    None,
                                    cx,
                                );
                            }
                            crate::Event::LanguageServerRemoved(id) => {
                                log_store.remove_language_server(*id, cx);
                            }
                            crate::Event::LanguageServerLog(id, typ, message) => {
                                log_store.add_language_server(
                                    server_kind,
                                    *id,
                                    None,
                                    None,
                                    None,
                                    cx,
                                );
                                match typ {
                                    crate::LanguageServerLogType::Log(typ) => {
                                        log_store.add_language_server_log(*id, *typ, message, cx);
                                    }
                                    crate::LanguageServerLogType::Trace { verbose_info } => {
                                        log_store.add_language_server_trace(
                                            *id,
                                            message,
                                            verbose_info.clone(),
                                            cx,
                                        );
                                    }
                                    crate::LanguageServerLogType::Rpc { received, elapsed } => {
                                        let kind = if *received {
                                            MessageKind::Receive
                                        } else {
                                            MessageKind::Send
                                        };
                                        log_store.add_language_server_rpc(
                                            *id,
                                            kind,
                                            message,
                                            RpcTiming::Forwarded(*elapsed),
                                            cx,
                                        );
                                    }
                                }
                            }
                            crate::Event::ToggleLspLogs {
                                server_id,
                                enabled,
                                toggled_log_kind,
                            } => {
                                log_store.toggle_lsp_logs(*server_id, *enabled, *toggled_log_kind);
                            }
                            _ => {}
                        }
                    }),
                ],
                copilot_log_subscription: None,
            },
        );
    }

    pub fn get_language_server_state(
        &mut self,
        id: LanguageServerId,
    ) -> Option<&mut LanguageServerState> {
        self.language_servers.get_mut(&id).or_else(|| {
            self.stopped_language_servers
                .values_mut()
                .find(|s| s.server_id == id)
        })
    }

    pub fn add_language_server(
        &mut self,
        kind: LanguageServerKind,
        server_id: LanguageServerId,
        name: Option<LanguageServerName>,
        worktree_id: Option<WorktreeId>,
        server: Option<Arc<LanguageServer>>,
        cx: &mut Context<Self>,
    ) -> Option<&mut LanguageServerState> {
        let stopped_state = name.as_ref().and_then(|name| {
            self.stopped_language_servers
                .shift_remove(&(name.clone(), worktree_id))
        });

        let server_state = self.language_servers.entry(server_id).or_insert_with(|| {
            cx.notify();
            LanguageServerState {
                server_id,
                name: None,
                worktree_id: None,
                kind,
                rpc_state: None,
                log_messages: VecDeque::with_capacity(MAX_STORED_LOG_ENTRIES),
                trace_messages: VecDeque::with_capacity(MAX_STORED_LOG_ENTRIES),
                trace_level: TraceValue::Off,
                log_level: MessageType::LOG,
                io_logs_subscription: None,
                toggled_log_kind: None,
            }
        });

        if let Some(stopped_state) = stopped_state {
            Self::merge_log_entries(&mut server_state.log_messages, stopped_state.log_messages);
            Self::merge_log_entries(
                &mut server_state.trace_messages,
                stopped_state.trace_messages,
            );
            match (&mut server_state.rpc_state, stopped_state.rpc_state) {
                (None, rpc_state) => server_state.rpc_state = rpc_state,
                (Some(current), Some(stopped)) => {
                    Self::merge_log_entries(&mut current.rpc_messages, stopped.rpc_messages);
                }
                (Some(_), None) => {}
            }
            server_state.trace_level = stopped_state.trace_level;
            server_state.log_level = stopped_state.log_level;
            server_state.toggled_log_kind = stopped_state.toggled_log_kind;
        }

        if let Some(name) = name {
            server_state.name = Some(name);
        }
        if let Some(worktree_id) = worktree_id {
            server_state.worktree_id = Some(worktree_id);
        }

        if let Some(server) = server.filter(|_| server_state.io_logs_subscription.is_none()) {
            let io_tx = self.io_tx.clone();
            let server_id = server.server_id();
            server_state.io_logs_subscription = Some(server.on_io(move |io_kind, message| {
                let observed_at = Instant::now();
                io_tx
                    .unbounded_send((server_id, io_kind, message.to_string(), observed_at))
                    .ok();
            }));
        }

        Some(server_state)
    }

    pub fn add_language_server_log(
        &mut self,
        id: LanguageServerId,
        typ: MessageType,
        message: &str,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let store_logs = !self.on_headless_host;
        let language_server_state = self.get_language_server_state(id)?;

        let log_lines = &mut language_server_state.log_messages;
        let message = message.trim_end().to_string();
        if !store_logs {
            // Send all messages regardless of the visibility in case of not storing, to notify the receiver anyway
            self.emit_event(
                Event::NewServerLogEntry {
                    id,
                    kind: LanguageServerLogType::Log(typ),
                    text: message,
                },
                cx,
            );
        } else if let Some(new_message) = Self::push_new_message(
            log_lines,
            LogMessage { message, typ },
            language_server_state.log_level,
        ) {
            self.emit_event(
                Event::NewServerLogEntry {
                    id,
                    kind: LanguageServerLogType::Log(typ),
                    text: new_message,
                },
                cx,
            );
        }
        Some(())
    }

    fn add_language_server_trace(
        &mut self,
        id: LanguageServerId,
        message: &str,
        verbose_info: Option<String>,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let store_logs = !self.on_headless_host;
        let language_server_state = self.get_language_server_state(id)?;

        let log_lines = &mut language_server_state.trace_messages;
        if !store_logs {
            // Send all messages regardless of the visibility in case of not storing, to notify the receiver anyway
            self.emit_event(
                Event::NewServerLogEntry {
                    id,
                    kind: LanguageServerLogType::Trace { verbose_info },
                    text: message.trim().to_string(),
                },
                cx,
            );
        } else if let Some(new_message) = Self::push_new_message(
            log_lines,
            TraceMessage {
                message: message.trim().to_string(),
                is_verbose: false,
            },
            TraceValue::Messages,
        ) {
            if let Some(verbose_message) = verbose_info.as_ref() {
                Self::push_new_message(
                    log_lines,
                    TraceMessage {
                        message: verbose_message.clone(),
                        is_verbose: true,
                    },
                    TraceValue::Verbose,
                );
            }
            self.emit_event(
                Event::NewServerLogEntry {
                    id,
                    kind: LanguageServerLogType::Trace { verbose_info },
                    text: new_message,
                },
                cx,
            );
        }
        Some(())
    }

    fn push_new_message<T: Message>(
        log_lines: &mut VecDeque<T>,
        message: T,
        current_severity: <T as Message>::Level,
    ) -> Option<String> {
        while log_lines.len() + 1 >= MAX_STORED_LOG_ENTRIES {
            log_lines.pop_front();
        }
        let visible = message.should_include(current_severity);

        let visible_message = visible.then(|| message.as_ref().to_string());
        log_lines.push_back(message);
        visible_message
    }

    fn merge_log_entries<T>(destination: &mut VecDeque<T>, source: VecDeque<T>) {
        let overflow = source
            .len()
            .saturating_add(destination.len())
            .saturating_sub(MAX_STORED_LOG_ENTRIES);
        let mut merged = source;
        for _ in 0..overflow {
            merged.pop_front();
        }
        merged.extend(destination.drain(..));
        *destination = merged;
    }

    fn add_language_server_rpc(
        &mut self,
        language_server_id: LanguageServerId,
        kind: MessageKind,
        message: &str,
        timing: RpcTiming,
        cx: &mut Context<'_, Self>,
    ) {
        let store_logs = !self.on_headless_host;
        let Some(state) = self
            .get_language_server_state(language_server_id)
            .and_then(|state| state.rpc_state.as_mut())
        else {
            return;
        };

        let elapsed = match timing {
            RpcTiming::ObservedAt(observed_at) => {
                state.request_tracker.observe(kind, message, observed_at)
            }
            RpcTiming::Forwarded(elapsed) => elapsed,
        };

        let received = kind == MessageKind::Receive;
        let header = state.header_state.header_for_message(kind, elapsed);

        if store_logs {
            let rpc_log_lines = &mut state.rpc_messages;
            while rpc_log_lines.len() + 1 >= MAX_STORED_LOG_ENTRIES {
                rpc_log_lines.pop_front();
            }
            let message = message.trim();
            rpc_log_lines.push_back(RpcMessage {
                message: match &header {
                    Some(header) => format!("{header}\n{message}"),
                    None => message.to_owned(),
                },
            });
        }

        if let Some(header) = header {
            // Do not send a synthetic message over the wire, it will be derived from the actual RPC message
            cx.emit(Event::NewServerLogEntry {
                id: language_server_id,
                kind: LanguageServerLogType::Rpc {
                    received,
                    elapsed: None,
                },
                text: header,
            });
        }

        self.emit_event(
            Event::NewServerLogEntry {
                id: language_server_id,
                kind: LanguageServerLogType::Rpc { received, elapsed },
                text: message.to_owned(),
            },
            cx,
        );
    }

    pub fn remove_language_server(&mut self, id: LanguageServerId, cx: &mut Context<Self>) {
        if let Some(mut state) = self.language_servers.remove(&id) {
            state.server_id = id;
            if let Some(name) = state.name.clone() {
                self.stopped_language_servers
                    .insert((name, state.worktree_id), state);
            }
        }

        while self.stopped_language_servers.len() > MAX_RETAINED_STOPPED_SERVERS {
            self.stopped_language_servers.shift_remove_index(0);
        }
        cx.notify();
    }

    pub fn server_logs(&self, server_id: LanguageServerId) -> Option<&VecDeque<LogMessage>> {
        self.language_server_state(server_id)
            .map(|state| &state.log_messages)
    }

    pub fn server_trace(&self, server_id: LanguageServerId) -> Option<&VecDeque<TraceMessage>> {
        self.language_server_state(server_id)
            .map(|state| &state.trace_messages)
    }

    pub fn language_server_state(
        &self,
        server_id: LanguageServerId,
    ) -> Option<&LanguageServerState> {
        self.language_servers.get(&server_id).or_else(|| {
            self.stopped_language_servers
                .values()
                .find(|s| s.server_id == server_id)
        })
    }

    pub fn server_ids_for_project<'a>(
        &'a self,
        lookup_project: &'a WeakEntity<Project>,
        cx: &App,
    ) -> impl Iterator<Item = LanguageServerId> + 'a {
        let lookup_project_lsp_store = lookup_project
            .upgrade()
            .map(|project| project.read(cx).lsp_store().downgrade());
        self.language_servers
            .iter()
            .filter_map(move |(id, state)| match &state.kind {
                LanguageServerKind::Local { project } | LanguageServerKind::Remote { project } => {
                    if project == lookup_project {
                        Some(*id)
                    } else {
                        None
                    }
                }
                LanguageServerKind::Global => Some(*id),
                LanguageServerKind::LocalSsh {
                    lsp_store: ssh_lsp_store,
                } => {
                    if lookup_project_lsp_store.as_ref() == Some(ssh_lsp_store) {
                        Some(*id)
                    } else {
                        None
                    }
                }
            })
    }

    pub fn enable_rpc_trace_for_language_server(
        &mut self,
        server_id: LanguageServerId,
    ) -> Option<&mut LanguageServerRpcState> {
        let rpc_state = self
            .get_language_server_state(server_id)?
            .rpc_state
            .get_or_insert_with(|| LanguageServerRpcState {
                rpc_messages: VecDeque::with_capacity(MAX_STORED_LOG_ENTRIES),
                header_state: RpcLogHeaderState::default(),
                request_tracker: RpcRequestTracker::default(),
            });
        Some(rpc_state)
    }

    pub fn disable_rpc_trace_for_language_server(
        &mut self,
        server_id: LanguageServerId,
    ) -> Option<()> {
        self.get_language_server_state(server_id)?.rpc_state.take();
        Some(())
    }

    pub fn has_server_logs(&self, server: &LanguageServerSelector) -> bool {
        match server {
            LanguageServerSelector::Id(id) => self.contains_language_server(*id),
            LanguageServerSelector::Name(name) => {
                self.language_servers
                    .values()
                    .any(|s| s.name.as_ref() == Some(name))
                    || self
                        .stopped_language_servers
                        .values()
                        .any(|s| s.name.as_ref() == Some(name))
            }
        }
    }

    pub fn contains_language_server(&self, id: LanguageServerId) -> bool {
        self.language_servers.contains_key(&id)
            || self
                .stopped_language_servers
                .values()
                .any(|s| s.server_id == id)
    }

    pub fn language_server_id_for_name_and_worktree(
        &self,
        name: &LanguageServerName,
        worktree_id: WorktreeId,
    ) -> Option<LanguageServerId> {
        self.stopped_language_servers
            .get(&(name.clone(), Some(worktree_id)))
            .map(|state| state.server_id)
            .or_else(|| {
                self.language_servers.iter().find_map(|(id, state)| {
                    (state.name.as_ref() == Some(name) && state.worktree_id == Some(worktree_id))
                        .then_some(*id)
                })
            })
    }

    fn on_io(
        &mut self,
        language_server_id: LanguageServerId,
        io_kind: IoKind,
        message: &str,
        observed_at: Instant,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let is_received = match io_kind {
            IoKind::StdOut => true,
            IoKind::StdIn => false,
            IoKind::StdErr => {
                self.add_language_server_log(language_server_id, MessageType::LOG, message, cx);
                return Some(());
            }
        };

        let kind = if is_received {
            MessageKind::Receive
        } else {
            MessageKind::Send
        };

        self.add_language_server_rpc(
            language_server_id,
            kind,
            message,
            RpcTiming::ObservedAt(observed_at),
            cx,
        );
        cx.notify();
        Some(())
    }

    fn emit_event(&mut self, e: Event, cx: &mut Context<Self>) {
        match &e {
            Event::NewServerLogEntry { id, kind, text } => {
                if let Some(state) = self.get_language_server_state(*id) {
                    let downstream_client = match &state.kind {
                        LanguageServerKind::Remote { project }
                        | LanguageServerKind::Local { project } => project
                            .upgrade()
                            .map(|project| project.read(cx).lsp_store()),
                        LanguageServerKind::LocalSsh { lsp_store } => lsp_store.upgrade(),
                        LanguageServerKind::Global => None,
                    }
                    .and_then(|lsp_store| lsp_store.read(cx).downstream_client());
                    if let Some((client, project_id)) = downstream_client {
                        if Some(LogKind::from_server_log_type(kind)) == state.toggled_log_kind {
                            client
                                .send(proto::LanguageServerLog {
                                    project_id,
                                    language_server_id: id.to_proto(),
                                    message: text.clone(),
                                    log_type: Some(kind.to_proto()),
                                })
                                .ok();
                        }
                    }
                }
            }
        }

        cx.emit(e);
    }

    pub fn toggle_lsp_logs(
        &mut self,
        server_id: LanguageServerId,
        enabled: bool,
        toggled_log_kind: LogKind,
    ) {
        if let Some(server_state) = self.get_language_server_state(server_id) {
            if enabled {
                server_state.toggled_log_kind = Some(toggled_log_kind);
            } else {
                server_state.toggled_log_kind = None;
            }
        }
        if LogKind::Rpc == toggled_log_kind {
            if enabled {
                self.enable_rpc_trace_for_language_server(server_id);
            } else {
                self.disable_rpc_trace_for_language_server(server_id);
            }
        }
    }
    pub fn copilot_state_for_project(
        &mut self,
        project: &WeakEntity<Project>,
    ) -> Option<&mut Option<lsp::Subscription>> {
        self.projects
            .get_mut(project)
            .map(|project| &mut project.copilot_log_subscription)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    /// A stopped server keeps its entry in LogStore. This can then be reused later on
    #[gpui::test]
    async fn test_stopped_server_logs_retained_until_restart(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let log_store = cx.new(|cx| LogStore::new(false, cx));
            let name = LanguageServerName("rust-analyzer".into());
            let worktree_id = WorktreeId::from_usize(1);
            let first_id = LanguageServerId(1);

            log_store.update(cx, |store, cx| {
                store.add_language_server(
                    LanguageServerKind::Global,
                    first_id,
                    Some(name.clone()),
                    Some(worktree_id),
                    None,
                    cx,
                );
                store.add_language_server_log(
                    first_id,
                    MessageType::LOG,
                    "hello from the server",
                    cx,
                );

                store.remove_language_server(first_id, cx);

                assert!(
                    store.contains_language_server(first_id),
                    "the stopped server stays tracked",
                );
                assert!(
                    !store.language_servers.contains_key(&first_id)
                        && store
                            .stopped_language_servers
                            .contains_key(&(name.clone(), Some(worktree_id))),
                    "the entry moves from running to stopped",
                );
                assert_eq!(
                    store.language_server_id_for_name_and_worktree(&name, worktree_id),
                    Some(first_id),
                    "the stopped server can still be looked up by name and worktree",
                );
                assert!(store.has_server_logs(&LanguageServerSelector::Id(first_id)));
                assert!(store.has_server_logs(&LanguageServerSelector::Name(name.clone())));
                assert_eq!(
                    store.server_logs(first_id).map(|logs| logs.len()),
                    Some(1),
                    "logs recorded before the stop are retained",
                );
                assert_eq!(
                    store
                        .server_logs(first_id)
                        .and_then(|logs| logs.front())
                        .map(|log| log.message.as_str()),
                    Some("hello from the server"),
                );
                assert!(
                    store.get_language_server_state(first_id).is_some(),
                    "mutable state access still works for the stopped server",
                );
                assert!(
                    store
                        .enable_rpc_trace_for_language_server(first_id)
                        .is_some(),
                    "rpc tracing can still be enabled for the stopped server",
                );

                let restarted_id = LanguageServerId(2);
                store.add_language_server(
                    LanguageServerKind::Global,
                    restarted_id,
                    Some(name.clone()),
                    Some(worktree_id),
                    None,
                    cx,
                );

                assert!(
                    !store.contains_language_server(first_id),
                    "the stopped server's id is no longer tracked after the restart",
                );
                assert!(
                    store.stopped_language_servers.is_empty(),
                    "no stopped entries linger after the restart",
                );
                assert_eq!(
                    store.language_server_id_for_name_and_worktree(&name, worktree_id),
                    Some(restarted_id),
                    "the lookup now points at the running instance",
                );
                assert_eq!(
                    store.server_logs(restarted_id).map(|logs| logs
                        .iter()
                        .map(|log| log.message.as_str())
                        .collect::<Vec<_>>()),
                    Some(vec!["hello from the server"]),
                    "the retained logs carry over to the restarted server",
                );
                store.add_language_server_log(restarted_id, MessageType::LOG, "hello again", cx);
                assert_eq!(
                    store.server_logs(restarted_id).map(|logs| logs
                        .iter()
                        .map(|log| log.message.as_str())
                        .collect::<Vec<_>>()),
                    Some(vec!["hello from the server", "hello again"]),
                    "new logs are appended after the carried-over logs",
                );
            });
        });
    }

    /// A Global (worktree-less) stopped server merges its logs into the
    /// restarted instance, since stopped entries are keyed by
    /// (name, Option<WorktreeId>).
    #[gpui::test]
    async fn test_stopped_global_server_logs_retained_until_restart(cx: &mut gpui::TestAppContext) {
        cx.update(|cx| {
            let log_store = cx.new(|cx| LogStore::new(false, cx));
            let name = LanguageServerName("global-server".into());
            let first_id = LanguageServerId(1);

            log_store.update(cx, |store, cx| {
                store.add_language_server(
                    LanguageServerKind::Global,
                    first_id,
                    Some(name.clone()),
                    None,
                    None,
                    cx,
                );
                store.add_language_server_log(
                    first_id,
                    MessageType::LOG,
                    "from the global server",
                    cx,
                );

                store.remove_language_server(first_id, cx);

                assert!(
                    store
                        .stopped_language_servers
                        .contains_key(&(name.clone(), None)),
                    "the global server is stored under a None worktree key",
                );
                assert_eq!(
                    store.server_logs(first_id).map(|logs| logs.len()),
                    Some(1),
                    "the global server's logs are retained while stopped",
                );

                let restarted_id = LanguageServerId(2);
                store.add_language_server(
                    LanguageServerKind::Global,
                    restarted_id,
                    Some(name.clone()),
                    None,
                    None,
                    cx,
                );

                assert!(
                    store.stopped_language_servers.is_empty(),
                    "the global stopped entry is claimed on restart",
                );
                assert_eq!(
                    store.server_logs(restarted_id).map(|logs| logs
                        .iter()
                        .map(|log| log.message.as_str())
                        .collect::<Vec<_>>()),
                    Some(vec!["from the global server"]),
                    "the global server's logs carry over to the restarted instance",
                );
            });
        });
    }
}
