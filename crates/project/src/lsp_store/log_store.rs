use std::{
    collections::VecDeque,
    sync::{Arc, Weak},
    time::{Duration, Instant},
};

use collections::{HashMap, HashSet};
use futures::{StreamExt, channel::mpsc};
use gpui::{
    App, AppContext as _, Context, Entity, EventEmitter, Global, Subscription, TaskExt, WeakEntity,
};
use lsp::{
    IoKind, LanguageServer, LanguageServerId, LanguageServerName, LanguageServerSelector,
    MessageType, RequestId, TraceValue,
};
use rpc::proto;
use serde::Deserialize;
use settings::WorktreeId;
use util::ResultExt as _;

use crate::{LanguageServerLogType, LspStore, Project, ProjectItem as _};

const MAX_STORED_LOG_ENTRIES: usize = 2000;
const MAX_PENDING_REQUESTS: usize = MAX_STORED_LOG_ENTRIES;

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
        key: LanguageServerLogKey,
        kind: LanguageServerLogType,
        text: String,
    },
}

impl EventEmitter<Event> for LogStore {}

pub struct LogStore {
    on_headless_host: bool,
    projects: HashMap<WeakEntity<Project>, ProjectState>,
    pub language_servers: HashMap<LanguageServerLogKey, LanguageServerState>,
    io_tx: mpsc::UnboundedSender<(LanguageServerLogKey, IoKind, String, Instant)>,
}

struct ProjectState {
    _subscriptions: [Subscription; 2],
    copilot_server_id: Option<LanguageServerId>,
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
    pub name: Option<LanguageServerName>,
    pub worktree_id: Option<WorktreeId>,
    server: Option<Weak<LanguageServer>>,
    log_messages: VecDeque<LogMessage>,
    trace_messages: VecDeque<TraceMessage>,
    pub rpc_state: Option<LanguageServerRpcState>,
    pub trace_level: TraceValue,
    pub log_level: MessageType,
    io_logs_subscription: Option<lsp::Subscription>,
    view_log_stream_refcounts: HashMap<LogKind, usize>,
    downstream_log_streams: HashMap<proto::PeerId, HashSet<LogKind>>,
}

impl LanguageServerState {
    pub fn server(&self) -> Option<Arc<LanguageServer>> {
        self.server.as_ref()?.upgrade()
    }

    fn has_view_log_stream(&self, log_kind: LogKind) -> bool {
        self.view_log_stream_refcounts.contains_key(&log_kind)
    }

    fn has_downstream_log_stream(&self, log_kind: LogKind) -> bool {
        self.downstream_log_streams
            .values()
            .any(|log_kinds| log_kinds.contains(&log_kind))
    }

    fn has_log_stream(&self, log_kind: LogKind) -> bool {
        self.has_view_log_stream(log_kind) || self.has_downstream_log_stream(log_kind)
    }

    fn synchronize_rpc_state(&mut self) {
        if self.has_log_stream(LogKind::Rpc) {
            self.rpc_state
                .get_or_insert_with(|| LanguageServerRpcState {
                    rpc_messages: VecDeque::with_capacity(MAX_STORED_LOG_ENTRIES),
                    header_state: RpcLogHeaderState::default(),
                    request_tracker: RpcRequestTracker::default(),
                });
        } else {
            self.rpc_state.take();
        }
    }
}

impl std::fmt::Debug for LanguageServerState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("LanguageServerState")
            .field("name", &self.name)
            .field("worktree_id", &self.worktree_id)
            .field("log_messages", &self.log_messages)
            .field("trace_messages", &self.trace_messages)
            .field("rpc_state", &self.rpc_state)
            .field("trace_level", &self.trace_level)
            .field("log_level", &self.log_level)
            .field("view_log_stream_refcounts", &self.view_log_stream_refcounts)
            .field("downstream_log_streams", &self.downstream_log_streams)
            .finish_non_exhaustive()
    }
}

#[derive(PartialEq, Eq, Hash, Clone)]
pub enum LanguageServerKind {
    Local { project: WeakEntity<Project> },
    Remote { project: WeakEntity<Project> },
    LocalSsh { lsp_store: WeakEntity<LspStore> },
    Supplementary { project: WeakEntity<Project> },
}

impl std::fmt::Debug for LanguageServerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageServerKind::Local { .. } => write!(f, "LanguageServerKind::Local"),
            LanguageServerKind::Remote { .. } => write!(f, "LanguageServerKind::Remote"),
            LanguageServerKind::LocalSsh { .. } => write!(f, "LanguageServerKind::LocalSsh"),
            LanguageServerKind::Supplementary { .. } => {
                write!(f, "LanguageServerKind::Supplementary")
            }
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LanguageServerLogKey {
    pub kind: LanguageServerKind,
    pub server_id: LanguageServerId,
}

impl LanguageServerLogKey {
    pub fn new(kind: LanguageServerKind, server_id: LanguageServerId) -> Self {
        Self { kind, server_id }
    }

    pub fn is_for_project(
        &self,
        project: &WeakEntity<Project>,
        lsp_store: &WeakEntity<LspStore>,
    ) -> bool {
        self.kind.is_for_project(project, lsp_store)
    }
}

impl LanguageServerKind {
    pub fn project(&self) -> Option<&WeakEntity<Project>> {
        match self {
            Self::Local { project }
            | Self::Remote { project }
            | Self::Supplementary { project } => Some(project),
            Self::LocalSsh { .. } => None,
        }
    }

    pub fn is_for_project(
        &self,
        project: &WeakEntity<Project>,
        lsp_store: &WeakEntity<LspStore>,
    ) -> bool {
        match self {
            Self::Local {
                project: server_project,
            }
            | Self::Remote {
                project: server_project,
            }
            | Self::Supplementary {
                project: server_project,
            } => server_project == project,
            Self::LocalSsh {
                lsp_store: server_lsp_store,
            } => server_lsp_store == lsp_store,
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

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Hash)]
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

    fn to_proto(self) -> Option<proto::toggle_lsp_logs::LogType> {
        match self {
            Self::Rpc => Some(proto::toggle_lsp_logs::LogType::Rpc),
            Self::Trace => Some(proto::toggle_lsp_logs::LogType::Trace),
            Self::Logs => Some(proto::toggle_lsp_logs::LogType::Log),
            Self::ServerInfo => None,
        }
    }
}

impl LogStore {
    pub fn new(on_headless_host: bool, cx: &mut Context<Self>) -> Self {
        let (io_tx, mut io_rx) = mpsc::unbounded();

        let log_store = Self {
            projects: HashMap::default(),
            language_servers: HashMap::default(),

            on_headless_host,
            io_tx,
        };
        cx.spawn(async move |log_store, cx| {
            while let Some((server_key, io_kind, message, observed_at)) = io_rx.next().await {
                if let Some(log_store) = log_store.upgrade() {
                    log_store.update(cx, |log_store, cx| {
                        log_store.on_io(&server_key, io_kind, &message, observed_at, cx);
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
                            .retain(|key, _| key.kind.project() != Some(&weak_project));
                    }),
                    cx.subscribe(project, move |log_store, project, event, cx| {
                        let is_local = project.read(cx).is_local();
                        let primary_server_kind = if is_local {
                            LanguageServerKind::Local {
                                project: project.downgrade(),
                            }
                        } else {
                            LanguageServerKind::Remote {
                                project: project.downgrade(),
                            }
                        };
                        let server_kind_for_id = |log_store: &LogStore, server_id| {
                            // Remote project events carry host-side server IDs, which may
                            // collide numerically with locally allocated supplementary
                            // server IDs, so only local projects may resolve an ID to a
                            // supplementary server.
                            if is_local {
                                let supplementary_server_kind = LanguageServerKind::Supplementary {
                                    project: project.downgrade(),
                                };
                                let supplementary_server_key = LanguageServerLogKey::new(
                                    supplementary_server_kind.clone(),
                                    server_id,
                                );
                                if log_store
                                    .language_servers
                                    .contains_key(&supplementary_server_key)
                                {
                                    return supplementary_server_kind;
                                }
                            }
                            primary_server_kind.clone()
                        };
                        match event {
                            crate::Event::LanguageServerAdded(id, name, worktree_id) => {
                                log_store.add_language_server(
                                    primary_server_kind,
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
                            crate::Event::SupplementaryLanguageServerAdded(id, name) => {
                                log_store.add_language_server(
                                    LanguageServerKind::Supplementary {
                                        project: project.downgrade(),
                                    },
                                    *id,
                                    Some(name.clone()),
                                    None,
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
                                    server_kind_for_id(log_store, *server_id),
                                    *server_id,
                                    name,
                                    worktree_id,
                                    None,
                                    cx,
                                );
                            }
                            crate::Event::LanguageServerRemoved(id) => {
                                let server_key =
                                    LanguageServerLogKey::new(primary_server_kind, *id);
                                log_store.remove_language_server(&server_key, cx);
                            }
                            crate::Event::SupplementaryLanguageServerRemoved(id) => {
                                let server_key = LanguageServerLogKey::new(
                                    LanguageServerKind::Supplementary {
                                        project: project.downgrade(),
                                    },
                                    *id,
                                );
                                log_store.remove_language_server(&server_key, cx);
                            }
                            crate::Event::LanguageServerLog(id, typ, message) => {
                                let server_kind = server_kind_for_id(log_store, *id);
                                let server_key =
                                    LanguageServerLogKey::new(server_kind.clone(), *id);
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
                                        log_store.add_language_server_log(
                                            &server_key,
                                            *typ,
                                            message,
                                            cx,
                                        );
                                    }
                                    crate::LanguageServerLogType::Trace { verbose_info } => {
                                        log_store.add_language_server_trace(
                                            &server_key,
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
                                            &server_key,
                                            kind,
                                            message,
                                            RpcTiming::Forwarded(*elapsed),
                                            cx,
                                        );
                                    }
                                }
                            }
                            crate::Event::ToggleLspLogs {
                                peer_id,
                                server_id,
                                enabled,
                                toggled_log_kind,
                            } => {
                                let server_key = LanguageServerLogKey::new(
                                    server_kind_for_id(log_store, *server_id),
                                    *server_id,
                                );
                                log_store.set_downstream_log_stream(
                                    &server_key,
                                    *peer_id,
                                    *toggled_log_kind,
                                    *enabled,
                                    cx,
                                );
                            }
                            crate::Event::CollaboratorLeft(peer_id) => {
                                log_store.release_downstream_log_streams_for_peer(
                                    &project.downgrade(),
                                    *peer_id,
                                    cx,
                                );
                            }
                            crate::Event::CollaboratorUpdated {
                                old_peer_id,
                                new_peer_id,
                            } => {
                                log_store.update_downstream_peer_id(
                                    &project.downgrade(),
                                    *old_peer_id,
                                    *new_peer_id,
                                );
                            }
                            _ => {}
                        }
                    }),
                ],
                copilot_server_id: None,
                copilot_log_subscription: None,
            },
        );
    }

    pub fn get_language_server_state(
        &mut self,
        key: &LanguageServerLogKey,
    ) -> Option<&mut LanguageServerState> {
        self.language_servers.get_mut(key)
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
        let server_key = LanguageServerLogKey::new(kind, server_id);
        let server_state = self
            .language_servers
            .entry(server_key.clone())
            .or_insert_with(|| {
                cx.notify();
                LanguageServerState {
                    name: None,
                    worktree_id: None,
                    server: server.as_ref().map(Arc::downgrade),
                    rpc_state: None,
                    log_messages: VecDeque::with_capacity(MAX_STORED_LOG_ENTRIES),
                    trace_messages: VecDeque::with_capacity(MAX_STORED_LOG_ENTRIES),
                    trace_level: TraceValue::Off,
                    log_level: MessageType::LOG,
                    io_logs_subscription: None,
                    view_log_stream_refcounts: HashMap::default(),
                    downstream_log_streams: HashMap::default(),
                }
            });

        if let Some(name) = name {
            server_state.name = Some(name);
        }
        if let Some(worktree_id) = worktree_id {
            server_state.worktree_id = Some(worktree_id);
        }

        if let Some(server) = server {
            let is_new_server = match server_state.server() {
                Some(current_server) => !Arc::ptr_eq(&current_server, &server),
                None => true,
            };
            if is_new_server {
                server_state.server = Some(Arc::downgrade(&server));
                server_state.io_logs_subscription = None;
            }
            if server_state.io_logs_subscription.is_none() {
                let io_tx = self.io_tx.clone();
                server_state.io_logs_subscription = Some(server.on_io(move |io_kind, message| {
                    let observed_at = Instant::now();
                    io_tx
                        .unbounded_send((
                            server_key.clone(),
                            io_kind,
                            message.to_string(),
                            observed_at,
                        ))
                        .ok();
                }));
            }
        }

        Some(server_state)
    }

    pub fn add_language_server_log(
        &mut self,
        key: &LanguageServerLogKey,
        typ: MessageType,
        message: &str,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let store_logs = !self.on_headless_host;
        let language_server_state = self.get_language_server_state(key)?;

        let log_lines = &mut language_server_state.log_messages;
        let message = message.trim_end().to_string();
        if !store_logs {
            // Send all messages regardless of the visibility in case of not storing, to notify the receiver anyway
            self.emit_event(
                Event::NewServerLogEntry {
                    key: key.clone(),
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
                    key: key.clone(),
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
        key: &LanguageServerLogKey,
        message: &str,
        verbose_info: Option<String>,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let store_logs = !self.on_headless_host;
        let language_server_state = self.get_language_server_state(key)?;

        let log_lines = &mut language_server_state.trace_messages;
        if !store_logs {
            // Send all messages regardless of the visibility in case of not storing, to notify the receiver anyway
            self.emit_event(
                Event::NewServerLogEntry {
                    key: key.clone(),
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
                    key: key.clone(),
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

    fn add_language_server_rpc(
        &mut self,
        key: &LanguageServerLogKey,
        kind: MessageKind,
        message: &str,
        timing: RpcTiming,
        cx: &mut Context<'_, Self>,
    ) {
        let store_logs = !self.on_headless_host;
        let Some(state) = self
            .get_language_server_state(key)
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
                key: key.clone(),
                kind: LanguageServerLogType::Rpc {
                    received,
                    elapsed: None,
                },
                text: header,
            });
        }

        self.emit_event(
            Event::NewServerLogEntry {
                key: key.clone(),
                kind: LanguageServerLogType::Rpc { received, elapsed },
                text: message.to_owned(),
            },
            cx,
        );
    }

    pub fn remove_language_server(&mut self, key: &LanguageServerLogKey, cx: &mut Context<Self>) {
        self.language_servers.remove(key);
        cx.notify();
    }

    pub fn server_logs(&self, key: &LanguageServerLogKey) -> Option<&VecDeque<LogMessage>> {
        Some(&self.language_servers.get(key)?.log_messages)
    }

    pub fn server_trace(&self, key: &LanguageServerLogKey) -> Option<&VecDeque<TraceMessage>> {
        Some(&self.language_servers.get(key)?.trace_messages)
    }

    pub fn server_keys_for_project<'a>(
        &'a self,
        project: &'a WeakEntity<Project>,
        lsp_store: &'a WeakEntity<LspStore>,
    ) -> impl Iterator<Item = LanguageServerLogKey> + 'a {
        self.language_servers
            .keys()
            .filter(move |key| key.is_for_project(project, lsp_store))
            .cloned()
    }

    pub fn has_server_logs(
        &self,
        server: &LanguageServerSelector,
        project: &WeakEntity<Project>,
        lsp_store: &WeakEntity<LspStore>,
    ) -> bool {
        self.language_servers.iter().any(|(key, state)| {
            key.is_for_project(project, lsp_store)
                && match server {
                    LanguageServerSelector::Id(id) => key.server_id == *id,
                    LanguageServerSelector::Name(name) => state.name.as_ref() == Some(name),
                }
        })
    }

    fn on_io(
        &mut self,
        key: &LanguageServerLogKey,
        io_kind: IoKind,
        message: &str,
        observed_at: Instant,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let is_received = match io_kind {
            IoKind::StdOut => true,
            IoKind::StdIn => false,
            IoKind::StdErr => {
                self.add_language_server_log(key, MessageType::LOG, message, cx);
                return Some(());
            }
        };

        let kind = if is_received {
            MessageKind::Receive
        } else {
            MessageKind::Send
        };

        self.add_language_server_rpc(key, kind, message, RpcTiming::ObservedAt(observed_at), cx);
        cx.notify();
        Some(())
    }

    fn emit_event(&mut self, e: Event, cx: &mut Context<Self>) {
        match &e {
            Event::NewServerLogEntry { key, kind, text } => {
                if let Some(state) = self.get_language_server_state(key) {
                    let downstream_client = match &key.kind {
                        LanguageServerKind::Remote { project }
                        | LanguageServerKind::Local { project } => project
                            .upgrade()
                            .map(|project| project.read(cx).lsp_store()),
                        LanguageServerKind::LocalSsh { lsp_store } => lsp_store.upgrade(),
                        LanguageServerKind::Supplementary { .. } => None,
                    }
                    .and_then(|lsp_store| lsp_store.read(cx).downstream_client());
                    if let Some((client, project_id)) = downstream_client {
                        if state.has_downstream_log_stream(LogKind::from_server_log_type(kind)) {
                            client
                                .send(proto::LanguageServerLog {
                                    project_id,
                                    language_server_id: key.server_id.to_proto(),
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

    pub fn retain_view_log_stream(
        &mut self,
        key: &LanguageServerLogKey,
        log_kind: LogKind,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        self.update_log_stream_ownership(
            key,
            log_kind,
            |state| {
                *state.view_log_stream_refcounts.entry(log_kind).or_default() += 1;
            },
            cx,
        )
    }

    pub fn release_view_log_stream(
        &mut self,
        key: &LanguageServerLogKey,
        log_kind: LogKind,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        self.update_log_stream_ownership(
            key,
            log_kind,
            |state| match state.view_log_stream_refcounts.get_mut(&log_kind) {
                Some(refcount) if *refcount > 1 => *refcount -= 1,
                Some(_) => {
                    state.view_log_stream_refcounts.remove(&log_kind);
                }
                None => {}
            },
            cx,
        )
    }

    pub fn set_downstream_log_stream(
        &mut self,
        key: &LanguageServerLogKey,
        peer_id: proto::PeerId,
        log_kind: LogKind,
        enabled: bool,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        self.update_log_stream_ownership(
            key,
            log_kind,
            |state| {
                if enabled {
                    state
                        .downstream_log_streams
                        .entry(peer_id)
                        .or_default()
                        .insert(log_kind);
                } else if let Some(log_kinds) = state.downstream_log_streams.get_mut(&peer_id) {
                    log_kinds.remove(&log_kind);
                    if log_kinds.is_empty() {
                        state.downstream_log_streams.remove(&peer_id);
                    }
                }
            },
            cx,
        )
    }

    fn release_downstream_log_streams_for_peer(
        &mut self,
        project: &WeakEntity<Project>,
        peer_id: proto::PeerId,
        cx: &mut Context<Self>,
    ) {
        let mut streams = Vec::new();
        for (key, state) in &self.language_servers {
            if key.kind.project() == Some(project)
                && let Some(log_kinds) = state.downstream_log_streams.get(&peer_id)
            {
                streams.extend(log_kinds.iter().map(|log_kind| (key.clone(), *log_kind)));
            }
        }

        for (key, log_kind) in streams {
            self.set_downstream_log_stream(&key, peer_id, log_kind, false, cx);
        }
    }

    fn update_downstream_peer_id(
        &mut self,
        project: &WeakEntity<Project>,
        old_peer_id: proto::PeerId,
        new_peer_id: proto::PeerId,
    ) {
        for (key, state) in &mut self.language_servers {
            if key.kind.project() == Some(project)
                && let Some(log_kinds) = state.downstream_log_streams.remove(&old_peer_id)
            {
                state
                    .downstream_log_streams
                    .entry(new_peer_id)
                    .or_default()
                    .extend(log_kinds);
            }
        }
    }

    fn update_log_stream_ownership(
        &mut self,
        key: &LanguageServerLogKey,
        log_kind: LogKind,
        update: impl FnOnce(&mut LanguageServerState),
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let (was_enabled, is_enabled) = {
            let state = self.get_language_server_state(key)?;
            let was_enabled = state.has_log_stream(log_kind);
            update(state);
            let is_enabled = state.has_log_stream(log_kind);
            if log_kind == LogKind::Rpc {
                state.synchronize_rpc_state();
            }
            (was_enabled, is_enabled)
        };

        if was_enabled != is_enabled {
            Self::send_toggle_log_message(key, is_enabled, log_kind, cx);
        }
        Some(())
    }

    fn send_toggle_log_message(
        key: &LanguageServerLogKey,
        enabled: bool,
        log_kind: LogKind,
        cx: &mut App,
    ) {
        let LanguageServerKind::Remote { project } = &key.kind else {
            return;
        };
        let Some(log_type) = log_kind.to_proto() else {
            return;
        };
        let Some(project) = project.upgrade() else {
            return;
        };
        let lsp_store = project.read(cx).lsp_store();
        let Some((client, project_id)) = lsp_store.read(cx).upstream_client() else {
            return;
        };
        client
            .send(proto::ToggleLspLogs {
                project_id,
                log_type: log_type as i32,
                server_id: key.server_id.to_proto(),
                enabled,
            })
            .log_err();
    }

    pub fn sync_copilot_for_project(
        &mut self,
        project: &WeakEntity<Project>,
        server: Option<Arc<LanguageServer>>,
        cx: &mut Context<Self>,
    ) -> Option<()> {
        let server_kind = LanguageServerKind::Supplementary {
            project: project.clone(),
        };
        let current_server_id = server.as_ref().map(|server| server.server_id());
        let current_server_matches = server.as_ref().is_some_and(|server| {
            let server_key = LanguageServerLogKey::new(server_kind.clone(), server.server_id());
            self.language_servers
                .get(&server_key)
                .and_then(|state| state.server())
                .is_some_and(|current_server| Arc::ptr_eq(&current_server, server))
        });
        let project_state = self.projects.get_mut(project)?;
        if project_state.copilot_server_id == current_server_id && current_server_matches {
            return Some(());
        }

        project_state.copilot_log_subscription = None;
        let previous_server_id = project_state.copilot_server_id.take();
        if previous_server_id != current_server_id
            && let Some(previous_server_id) = previous_server_id
        {
            let previous_server_key =
                LanguageServerLogKey::new(server_kind.clone(), previous_server_id);
            self.remove_language_server(&previous_server_key, cx);
        }

        let Some(server) = server else {
            return Some(());
        };
        let server_id = server.server_id();
        let server_key = LanguageServerLogKey::new(server_kind.clone(), server_id);
        let weak_log_store = cx.weak_entity();
        let log_subscription =
            server.on_notification::<lsp::notification::LogMessage, _>(move |params, cx| {
                weak_log_store
                    .update(cx, |log_store, cx| {
                        log_store.add_language_server_log(
                            &server_key,
                            MessageType::LOG,
                            &params.message,
                            cx,
                        );
                    })
                    .ok();
            });
        self.add_language_server(
            server_kind,
            server_id,
            Some(LanguageServerName::new_static("copilot")),
            None,
            Some(server),
            cx,
        );

        let project_state = self.projects.get_mut(project)?;
        project_state.copilot_server_id = Some(server_id);
        project_state.copilot_log_subscription = Some(log_subscription);
        Some(())
    }
}
