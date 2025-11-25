use std::{collections::VecDeque, sync::Arc};

use collections::HashMap;
use futures::{StreamExt, channel::mpsc};
use gpui::{App, AppContext as _, Context, Entity, EventEmitter, Global, Subscription, WeakEntity};
use lsp::{
    IoKind, LanguageServer, LanguageServerId, LanguageServerName, LanguageServerSelector,
    MessageType, TraceValue,
};
use rpc::proto;
use settings::WorktreeId;

use crate::{LanguageServerLogType, LspStore, Project, ProjectItem as _};

const SEND_LINE: &str = "\n// Send:";
const RECEIVE_LINE: &str = "\n// Receive:";
const MAX_STORED_LOG_ENTRIES: usize = 2000;

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
    pub copilot_log_subscription: Option<lsp::Subscription>,
    pub language_servers: HashMap<LanguageServerId, LanguageServerState>,
    io_tx: mpsc::UnboundedSender<(LanguageServerId, IoKind, String)>,
}

struct ProjectState {
    _subscriptions: [Subscription; 2],
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
    last_message_kind: Option<MessageKind>,
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum MessageKind {
    Send,
    Receive,
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
            copilot_log_subscription: None,
            on_headless_host,
            io_tx,
        };
        cx.spawn(async move |log_store, cx| {
            while let Some((server_id, io_kind, message)) = io_rx.next().await {
                if let Some(log_store) = log_store.upgrade() {
                    log_store.update(cx, |log_store, cx| {
                        log_store.on_io(server_id, io_kind, &message, cx);
                    })?;
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
                                    crate::LanguageServerLogType::Rpc { received } => {
                                        let kind = if *received {
                                            MessageKind::Receive
                                        } else {
                                            MessageKind::Send
                                        };
                                        log_store.add_language_server_rpc(*id, kind, message, cx);
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
            },
        );
    }

    pub fn get_language_server_state(
        &mut self,
        id: LanguageServerId,
    ) -> Option<&mut LanguageServerState> {
        self.language_servers.get_mut(&id)
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
        let server_state = self.language_servers.entry(server_id).or_insert_with(|| {
            cx.notify();
            LanguageServerState {
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
                io_tx
                    .unbounded_send((server_id, io_kind, message.to_string()))
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

    fn add_language_server_rpc(
        &mut self,
        language_server_id: LanguageServerId,
        kind: MessageKind,
        message: &str,
        cx: &mut Context<'_, Self>,
    ) {
        let store_logs = !self.on_headless_host;
        let Some(state) = self
            .get_language_server_state(language_server_id)
            .and_then(|state| state.rpc_state.as_mut())
        else {
            return;
        };

        let received = kind == MessageKind::Receive;
        let rpc_log_lines = &mut state.rpc_messages;
        if state.last_message_kind != Some(kind) {
            while rpc_log_lines.len() + 1 >= MAX_STORED_LOG_ENTRIES {
                rpc_log_lines.pop_front();
            }
            let line_before_message = match kind {
                MessageKind::Send => SEND_LINE,
                MessageKind::Receive => RECEIVE_LINE,
            };
            if store_logs {
                rpc_log_lines.push_back(RpcMessage {
                    message: line_before_message.to_string(),
                });
            }
            // Do not send a synthetic message over the wire, it will be derived from the actual RPC message
            cx.emit(Event::NewServerLogEntry {
                id: language_server_id,
                kind: LanguageServerLogType::Rpc { received },
                text: line_before_message.to_string(),
            });
        }

        while rpc_log_lines.len() + 1 >= MAX_STORED_LOG_ENTRIES {
            rpc_log_lines.pop_front();
        }

        if store_logs {
            rpc_log_lines.push_back(RpcMessage {
                message: message.trim().to_owned(),
            });
        }

        self.emit_event(
            Event::NewServerLogEntry {
                id: language_server_id,
                kind: LanguageServerLogType::Rpc { received },
                text: message.to_owned(),
            },
            cx,
        );
    }

    pub fn remove_language_server(&mut self, id: LanguageServerId, cx: &mut Context<Self>) {
        self.language_servers.remove(&id);
        cx.notify();
    }

    pub fn server_logs(&self, server_id: LanguageServerId) -> Option<&VecDeque<LogMessage>> {
        Some(&self.language_servers.get(&server_id)?.log_messages)
    }

    pub fn server_trace(&self, server_id: LanguageServerId) -> Option<&VecDeque<TraceMessage>> {
        Some(&self.language_servers.get(&server_id)?.trace_messages)
    }

    pub fn server_ids_for_project<'a>(
        &'a self,
        lookup_project: &'a WeakEntity<Project>,
    ) -> impl Iterator<Item = LanguageServerId> + 'a {
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
                LanguageServerKind::Global | LanguageServerKind::LocalSsh { .. } => Some(*id),
            })
    }

    pub fn enable_rpc_trace_for_language_server(
        &mut self,
        server_id: LanguageServerId,
    ) -> Option<&mut LanguageServerRpcState> {
        let rpc_state = self
            .language_servers
            .get_mut(&server_id)?
            .rpc_state
            .get_or_insert_with(|| LanguageServerRpcState {
                rpc_messages: VecDeque::with_capacity(MAX_STORED_LOG_ENTRIES),
                last_message_kind: None,
            });
        Some(rpc_state)
    }

    pub fn disable_rpc_trace_for_language_server(
        &mut self,
        server_id: LanguageServerId,
    ) -> Option<()> {
        self.language_servers.get_mut(&server_id)?.rpc_state.take();
        Some(())
    }

    pub fn has_server_logs(&self, server: &LanguageServerSelector) -> bool {
        match server {
            LanguageServerSelector::Id(id) => self.language_servers.contains_key(id),
            LanguageServerSelector::Name(name) => self
                .language_servers
                .iter()
                .any(|(_, state)| state.name.as_ref() == Some(name)),
        }
    }

    fn on_io(
        &mut self,
        language_server_id: LanguageServerId,
        io_kind: IoKind,
        message: &str,
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

        self.add_language_server_rpc(language_server_id, kind, message, cx);
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
}
