use collections::{HashMap, VecDeque};
use copilot::Copilot;
use editor::{actions::MoveToEnd, Editor, EditorEvent};
use futures::{channel::mpsc, StreamExt};
use gpui::{
    actions, div, AnchorCorner, AppContext, Context, EventEmitter, FocusHandle, FocusableView,
    IntoElement, Model, ModelContext, ParentElement, Render, Styled, Subscription, View,
    ViewContext, VisualContext, WeakModel, WindowContext,
};
use language::LanguageServerId;
use lsp::{
    notification::SetTrace, IoKind, LanguageServer, LanguageServerName, MessageType,
    ServerCapabilities, SetTraceParams, TraceValue,
};
use project::{search::SearchQuery, Project, WorktreeId};
use std::{borrow::Cow, sync::Arc};
use ui::{prelude::*, Button, Checkbox, ContextMenu, Label, PopoverMenu, Selection};
use workspace::{
    item::{Item, ItemHandle},
    searchable::{SearchEvent, SearchableItem, SearchableItemHandle},
    SplitDirection, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace, WorkspaceId,
};

const SEND_LINE: &str = "// Send:";
const RECEIVE_LINE: &str = "// Receive:";
const MAX_STORED_LOG_ENTRIES: usize = 2000;

pub struct LogStore {
    projects: HashMap<WeakModel<Project>, ProjectState>,
    language_servers: HashMap<LanguageServerId, LanguageServerState>,
    copilot_log_subscription: Option<lsp::Subscription>,
    _copilot_subscription: Option<gpui::Subscription>,
    io_tx: mpsc::UnboundedSender<(LanguageServerId, IoKind, String)>,
}

struct ProjectState {
    _subscriptions: [gpui::Subscription; 2],
}

trait Message: AsRef<str> {
    type Level: Copy + std::fmt::Debug;
    fn should_include(&self, _: Self::Level) -> bool {
        true
    }
}

struct LogMessage {
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

struct TraceMessage {
    message: String,
}

impl AsRef<str> for TraceMessage {
    fn as_ref(&self) -> &str {
        &self.message
    }
}

impl Message for TraceMessage {
    type Level = ();
}

struct RpcMessage {
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

struct LanguageServerState {
    name: Option<LanguageServerName>,
    worktree_id: Option<WorktreeId>,
    kind: LanguageServerKind,
    log_messages: VecDeque<LogMessage>,
    trace_messages: VecDeque<TraceMessage>,
    rpc_state: Option<LanguageServerRpcState>,
    trace_level: TraceValue,
    log_level: MessageType,
    capabilities: ServerCapabilities,
    io_logs_subscription: Option<lsp::Subscription>,
}

#[derive(PartialEq, Clone)]
pub enum LanguageServerKind {
    Local { project: WeakModel<Project> },
    Remote { project: WeakModel<Project> },
    Global,
}

impl LanguageServerKind {
    fn is_remote(&self) -> bool {
        matches!(self, LanguageServerKind::Remote { .. })
    }
}

impl std::fmt::Debug for LanguageServerKind {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            LanguageServerKind::Local { .. } => write!(f, "LanguageServerKind::Local"),
            LanguageServerKind::Remote { .. } => write!(f, "LanguageServerKind::Remote"),
            LanguageServerKind::Global => write!(f, "LanguageServerKind::Global"),
        }
    }
}

impl LanguageServerKind {
    fn project(&self) -> Option<&WeakModel<Project>> {
        match self {
            Self::Local { project } => Some(project),
            Self::Remote { project } => Some(project),
            Self::Global { .. } => None,
        }
    }
}

struct LanguageServerRpcState {
    rpc_messages: VecDeque<RpcMessage>,
    last_message_kind: Option<MessageKind>,
}

pub struct LspLogView {
    pub(crate) editor: View<Editor>,
    editor_subscriptions: Vec<Subscription>,
    log_store: Model<LogStore>,
    current_server_id: Option<LanguageServerId>,
    active_entry_kind: LogKind,
    project: Model<Project>,
    focus_handle: FocusHandle,
    _log_store_subscriptions: Vec<Subscription>,
}

pub struct LspLogToolbarItemView {
    log_view: Option<View<LspLogView>>,
    _log_view_subscription: Option<Subscription>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
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
    Capabilities,
}

impl LogKind {
    fn label(&self) -> &'static str {
        match self {
            LogKind::Rpc => RPC_MESSAGES,
            LogKind::Trace => SERVER_TRACE,
            LogKind::Logs => SERVER_LOGS,
            LogKind::Capabilities => SERVER_CAPABILITIES,
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LogMenuItem {
    pub server_id: LanguageServerId,
    pub server_name: LanguageServerName,
    pub worktree_root_name: String,
    pub rpc_trace_enabled: bool,
    pub selected_entry: LogKind,
    pub trace_level: lsp::TraceValue,
    pub server_kind: LanguageServerKind,
}

actions!(debug, [OpenLanguageServerLogs]);

pub fn init(cx: &mut AppContext) {
    let log_store = cx.new_model(LogStore::new);

    cx.observe_new_views(move |workspace: &mut Workspace, cx| {
        let project = workspace.project();
        if project.read(cx).is_local() || project.read(cx).is_via_ssh() {
            log_store.update(cx, |store, cx| {
                store.add_project(project, cx);
            });
        }

        let log_store = log_store.clone();
        workspace.register_action(move |workspace, _: &OpenLanguageServerLogs, cx| {
            let project = workspace.project().read(cx);
            if project.is_local() || project.is_via_ssh() {
                workspace.split_item(
                    SplitDirection::Right,
                    Box::new(cx.new_view(|cx| {
                        LspLogView::new(workspace.project().clone(), log_store.clone(), cx)
                    })),
                    cx,
                );
            }
        });
    })
    .detach();
}

impl LogStore {
    pub fn new(cx: &mut ModelContext<Self>) -> Self {
        let (io_tx, mut io_rx) = mpsc::unbounded();

        let copilot_subscription = Copilot::global(cx).map(|copilot| {
            let copilot = &copilot;
            cx.subscribe(copilot, |this, copilot, inline_completion_event, cx| {
                if let copilot::Event::CopilotLanguageServerStarted = inline_completion_event {
                    if let Some(server) = copilot.read(cx).language_server() {
                        let server_id = server.server_id();
                        let weak_this = cx.weak_model();
                        this.copilot_log_subscription =
                            Some(server.on_notification::<copilot::request::LogMessage, _>(
                                move |params, mut cx| {
                                    weak_this
                                        .update(&mut cx, |this, cx| {
                                            this.add_language_server_log(
                                                server_id,
                                                MessageType::LOG,
                                                &params.message,
                                                cx,
                                            );
                                        })
                                        .ok();
                                },
                            ));
                        let name = LanguageServerName::new_static("copilot");
                        this.add_language_server(
                            LanguageServerKind::Global,
                            server.server_id(),
                            Some(name),
                            None,
                            Some(server.clone()),
                            cx,
                        );
                    }
                }
            })
        });

        let this = Self {
            copilot_log_subscription: None,
            _copilot_subscription: copilot_subscription,
            projects: HashMap::default(),
            language_servers: HashMap::default(),
            io_tx,
        };

        cx.spawn(|this, mut cx| async move {
            while let Some((server_id, io_kind, message)) = io_rx.next().await {
                if let Some(this) = this.upgrade() {
                    this.update(&mut cx, |this, cx| {
                        this.on_io(server_id, io_kind, &message, cx);
                    })?;
                }
            }
            anyhow::Ok(())
        })
        .detach_and_log_err(cx);
        this
    }

    pub fn add_project(&mut self, project: &Model<Project>, cx: &mut ModelContext<Self>) {
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
                    cx.subscribe(project, |this, project, event, cx| {
                        let server_kind = if project.read(cx).is_via_ssh() {
                            LanguageServerKind::Remote {
                                project: project.downgrade(),
                            }
                        } else {
                            LanguageServerKind::Local {
                                project: project.downgrade(),
                            }
                        };

                        match event {
                            project::Event::LanguageServerAdded(id, name, worktree_id) => {
                                this.add_language_server(
                                    server_kind,
                                    *id,
                                    Some(name.clone()),
                                    *worktree_id,
                                    project.read(cx).language_server_for_id(*id, cx),
                                    cx,
                                );
                            }
                            project::Event::LanguageServerRemoved(id) => {
                                this.remove_language_server(*id, cx);
                            }
                            project::Event::LanguageServerLog(id, typ, message) => {
                                this.add_language_server(server_kind, *id, None, None, None, cx);
                                match typ {
                                    project::LanguageServerLogType::Log(typ) => {
                                        this.add_language_server_log(*id, *typ, message, cx);
                                    }
                                    project::LanguageServerLogType::Trace(_) => {
                                        this.add_language_server_trace(*id, message, cx);
                                    }
                                }
                            }
                            _ => {}
                        }
                    }),
                ],
            },
        );
    }

    fn get_language_server_state(
        &mut self,
        id: LanguageServerId,
    ) -> Option<&mut LanguageServerState> {
        self.language_servers.get_mut(&id)
    }

    fn add_language_server(
        &mut self,
        kind: LanguageServerKind,
        server_id: LanguageServerId,
        name: Option<LanguageServerName>,
        worktree_id: Option<WorktreeId>,
        server: Option<Arc<LanguageServer>>,
        cx: &mut ModelContext<Self>,
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
                capabilities: ServerCapabilities::default(),
            }
        });

        if let Some(name) = name {
            server_state.name = Some(name);
        }
        if let Some(worktree_id) = worktree_id {
            server_state.worktree_id = Some(worktree_id);
        }

        if let Some(server) = server
            .clone()
            .filter(|_| server_state.io_logs_subscription.is_none())
        {
            let io_tx = self.io_tx.clone();
            let server_id = server.server_id();
            server_state.io_logs_subscription = Some(server.on_io(move |io_kind, message| {
                io_tx
                    .unbounded_send((server_id, io_kind, message.to_string()))
                    .ok();
            }));
        }

        if let Some(server) = server {
            server_state.capabilities = server.capabilities();
        }

        Some(server_state)
    }

    fn add_language_server_log(
        &mut self,
        id: LanguageServerId,
        typ: MessageType,
        message: &str,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        let language_server_state = self.get_language_server_state(id)?;

        let log_lines = &mut language_server_state.log_messages;
        Self::add_language_server_message(
            log_lines,
            id,
            LogMessage {
                message: message.trim_end().to_string(),
                typ,
            },
            language_server_state.log_level,
            LogKind::Logs,
            cx,
        );
        Some(())
    }

    fn add_language_server_trace(
        &mut self,
        id: LanguageServerId,
        message: &str,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        let language_server_state = self.get_language_server_state(id)?;

        let log_lines = &mut language_server_state.trace_messages;
        Self::add_language_server_message(
            log_lines,
            id,
            TraceMessage {
                message: message.trim_end().to_string(),
            },
            (),
            LogKind::Trace,
            cx,
        );
        Some(())
    }

    fn add_language_server_message<T: Message>(
        log_lines: &mut VecDeque<T>,
        id: LanguageServerId,
        message: T,
        current_severity: <T as Message>::Level,
        kind: LogKind,
        cx: &mut ModelContext<Self>,
    ) {
        while log_lines.len() >= MAX_STORED_LOG_ENTRIES {
            log_lines.pop_front();
        }
        let entry: &str = message.as_ref();
        let entry = entry.to_string();
        let visible = message.should_include(current_severity);
        log_lines.push_back(message);

        if visible {
            cx.emit(Event::NewServerLogEntry { id, entry, kind });
            cx.notify();
        }
    }

    fn remove_language_server(&mut self, id: LanguageServerId, cx: &mut ModelContext<Self>) {
        self.language_servers.remove(&id);
        cx.notify();
    }

    fn server_logs(&self, server_id: LanguageServerId) -> Option<&VecDeque<LogMessage>> {
        Some(&self.language_servers.get(&server_id)?.log_messages)
    }

    fn server_trace(&self, server_id: LanguageServerId) -> Option<&VecDeque<TraceMessage>> {
        Some(&self.language_servers.get(&server_id)?.trace_messages)
    }

    fn server_capabilities(&self, server_id: LanguageServerId) -> Option<&ServerCapabilities> {
        Some(&self.language_servers.get(&server_id)?.capabilities)
    }

    fn server_ids_for_project<'a>(
        &'a self,
        lookup_project: &'a WeakModel<Project>,
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
                LanguageServerKind::Global => Some(*id),
            })
    }

    fn enable_rpc_trace_for_language_server(
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

    fn on_io(
        &mut self,
        language_server_id: LanguageServerId,
        io_kind: IoKind,
        message: &str,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        let is_received = match io_kind {
            IoKind::StdOut => true,
            IoKind::StdIn => false,
            IoKind::StdErr => {
                let message = format!("stderr: {}", message.trim());
                self.add_language_server_log(language_server_id, MessageType::LOG, &message, cx);
                return Some(());
            }
        };

        let state = self
            .get_language_server_state(language_server_id)?
            .rpc_state
            .as_mut()?;
        let kind = if is_received {
            MessageKind::Receive
        } else {
            MessageKind::Send
        };

        let rpc_log_lines = &mut state.rpc_messages;
        if state.last_message_kind != Some(kind) {
            let line_before_message = match kind {
                MessageKind::Send => SEND_LINE,
                MessageKind::Receive => RECEIVE_LINE,
            };
            rpc_log_lines.push_back(RpcMessage {
                message: line_before_message.to_string(),
            });
            cx.emit(Event::NewServerLogEntry {
                id: language_server_id,
                entry: line_before_message.to_string(),
                kind: LogKind::Rpc,
            });
        }

        while rpc_log_lines.len() >= MAX_STORED_LOG_ENTRIES {
            rpc_log_lines.pop_front();
        }
        let message = message.trim();
        rpc_log_lines.push_back(RpcMessage {
            message: message.to_string(),
        });
        cx.emit(Event::NewServerLogEntry {
            id: language_server_id,
            entry: message.to_string(),
            kind: LogKind::Rpc,
        });
        cx.notify();
        Some(())
    }
}

impl LspLogView {
    pub fn new(
        project: Model<Project>,
        log_store: Model<LogStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let server_id = log_store
            .read(cx)
            .language_servers
            .iter()
            .find(|(_, server)| server.kind.project() == Some(&project.downgrade()))
            .map(|(id, _)| *id);

        let weak_project = project.downgrade();
        let model_changes_subscription = cx.observe(&log_store, move |this, store, cx| {
            let first_server_id_for_project =
                store.read(cx).server_ids_for_project(&weak_project).next();
            if let Some(current_lsp) = this.current_server_id {
                if !store.read(cx).language_servers.contains_key(&current_lsp) {
                    if let Some(server_id) = first_server_id_for_project {
                        match this.active_entry_kind {
                            LogKind::Rpc => this.show_rpc_trace_for_server(server_id, cx),
                            LogKind::Trace => this.show_trace_for_server(server_id, cx),
                            LogKind::Logs => this.show_logs_for_server(server_id, cx),
                            LogKind::Capabilities => {
                                this.show_capabilities_for_server(server_id, cx)
                            }
                        }
                    } else {
                        this.current_server_id = None;
                        this.editor.update(cx, |editor, cx| {
                            editor.set_read_only(false);
                            editor.clear(cx);
                            editor.set_read_only(true);
                        });
                        cx.notify();
                    }
                }
            } else if let Some(server_id) = first_server_id_for_project {
                match this.active_entry_kind {
                    LogKind::Rpc => this.show_rpc_trace_for_server(server_id, cx),
                    LogKind::Trace => this.show_trace_for_server(server_id, cx),
                    LogKind::Logs => this.show_logs_for_server(server_id, cx),
                    LogKind::Capabilities => this.show_capabilities_for_server(server_id, cx),
                }
            }

            cx.notify();
        });
        let events_subscriptions = cx.subscribe(&log_store, |log_view, _, e, cx| match e {
            Event::NewServerLogEntry { id, entry, kind } => {
                if log_view.current_server_id == Some(*id) && *kind == log_view.active_entry_kind {
                    log_view.editor.update(cx, |editor, cx| {
                        editor.set_read_only(false);
                        let last_point = editor.buffer().read(cx).len(cx);
                        editor.edit(
                            vec![
                                (last_point..last_point, entry.trim()),
                                (last_point..last_point, "\n"),
                            ],
                            cx,
                        );
                        editor.set_read_only(true);
                    });
                }
            }
        });
        let (editor, editor_subscriptions) = Self::editor_for_logs(String::new(), cx);

        let focus_handle = cx.focus_handle();
        let focus_subscription = cx.on_focus(&focus_handle, |log_view, cx| {
            cx.focus_view(&log_view.editor);
        });

        let mut this = Self {
            focus_handle,
            editor,
            editor_subscriptions,
            project,
            log_store,
            current_server_id: None,
            active_entry_kind: LogKind::Logs,
            _log_store_subscriptions: vec![
                model_changes_subscription,
                events_subscriptions,
                focus_subscription,
            ],
        };
        if let Some(server_id) = server_id {
            this.show_logs_for_server(server_id, cx);
        }
        this
    }

    fn editor_for_logs(
        log_contents: String,
        cx: &mut ViewContext<Self>,
    ) -> (View<Editor>, Vec<Subscription>) {
        let editor = cx.new_view(|cx| {
            let mut editor = Editor::multi_line(cx);
            editor.set_text(log_contents, cx);
            editor.move_to_end(&MoveToEnd, cx);
            editor.set_read_only(true);
            editor.set_show_inline_completions(Some(false), cx);
            editor
        });
        let editor_subscription = cx.subscribe(
            &editor,
            |_, _, event: &EditorEvent, cx: &mut ViewContext<'_, LspLogView>| {
                cx.emit(event.clone())
            },
        );
        let search_subscription = cx.subscribe(
            &editor,
            |_, _, event: &SearchEvent, cx: &mut ViewContext<'_, LspLogView>| {
                cx.emit(event.clone())
            },
        );
        (editor, vec![editor_subscription, search_subscription])
    }

    fn editor_for_capabilities(
        capabilities: ServerCapabilities,
        cx: &mut ViewContext<Self>,
    ) -> (View<Editor>, Vec<Subscription>) {
        let editor = cx.new_view(|cx| {
            let mut editor = Editor::multi_line(cx);
            editor.set_text(serde_json::to_string_pretty(&capabilities).unwrap(), cx);
            editor.move_to_end(&MoveToEnd, cx);
            editor.set_read_only(true);
            editor.set_show_inline_completions(Some(false), cx);
            editor
        });
        let editor_subscription = cx.subscribe(
            &editor,
            |_, _, event: &EditorEvent, cx: &mut ViewContext<'_, LspLogView>| {
                cx.emit(event.clone())
            },
        );
        let search_subscription = cx.subscribe(
            &editor,
            |_, _, event: &SearchEvent, cx: &mut ViewContext<'_, LspLogView>| {
                cx.emit(event.clone())
            },
        );
        (editor, vec![editor_subscription, search_subscription])
    }

    pub(crate) fn menu_items<'a>(&'a self, cx: &'a AppContext) -> Option<Vec<LogMenuItem>> {
        let log_store = self.log_store.read(cx);

        let unknown_server = LanguageServerName::new_static("unknown server");

        let mut rows = log_store
            .language_servers
            .iter()
            .map(|(server_id, state)| match &state.kind {
                LanguageServerKind::Local { .. } | LanguageServerKind::Remote { .. } => {
                    let worktree_root_name = state
                        .worktree_id
                        .and_then(|id| self.project.read(cx).worktree_for_id(id, cx))
                        .map(|worktree| worktree.read(cx).root_name().to_string())
                        .unwrap_or_else(|| "Unknown worktree".to_string());

                    LogMenuItem {
                        server_id: *server_id,
                        server_name: state.name.clone().unwrap_or(unknown_server.clone()),
                        server_kind: state.kind.clone(),
                        worktree_root_name,
                        rpc_trace_enabled: state.rpc_state.is_some(),
                        selected_entry: self.active_entry_kind,
                        trace_level: lsp::TraceValue::Off,
                    }
                }

                LanguageServerKind::Global => LogMenuItem {
                    server_id: *server_id,
                    server_name: state.name.clone().unwrap_or(unknown_server.clone()),
                    server_kind: state.kind.clone(),
                    worktree_root_name: "supplementary".to_string(),
                    rpc_trace_enabled: state.rpc_state.is_some(),
                    selected_entry: self.active_entry_kind,
                    trace_level: lsp::TraceValue::Off,
                },
            })
            .chain(
                self.project
                    .read(cx)
                    .supplementary_language_servers(cx)
                    .filter_map(|(server_id, name)| {
                        let state = log_store.language_servers.get(&server_id)?;
                        Some(LogMenuItem {
                            server_id,
                            server_name: name.clone(),
                            server_kind: state.kind.clone(),
                            worktree_root_name: "supplementary".to_string(),
                            rpc_trace_enabled: state.rpc_state.is_some(),
                            selected_entry: self.active_entry_kind,
                            trace_level: lsp::TraceValue::Off,
                        })
                    }),
            )
            .collect::<Vec<_>>();
        rows.sort_by_key(|row| row.server_id);
        rows.dedup_by_key(|row| row.server_id);
        Some(rows)
    }

    fn show_logs_for_server(&mut self, server_id: LanguageServerId, cx: &mut ViewContext<Self>) {
        let typ = self
            .log_store
            .read_with(cx, |v, _| {
                v.language_servers.get(&server_id).map(|v| v.log_level)
            })
            .unwrap_or(MessageType::LOG);
        let log_contents = self
            .log_store
            .read(cx)
            .server_logs(server_id)
            .map(|v| log_contents(v, typ));
        if let Some(log_contents) = log_contents {
            self.current_server_id = Some(server_id);
            self.active_entry_kind = LogKind::Logs;
            let (editor, editor_subscriptions) = Self::editor_for_logs(log_contents, cx);
            self.editor = editor;
            self.editor_subscriptions = editor_subscriptions;
            cx.notify();
        }
        cx.focus(&self.focus_handle);
    }

    fn update_log_level(
        &self,
        server_id: LanguageServerId,
        level: MessageType,
        cx: &mut ViewContext<Self>,
    ) {
        let log_contents = self.log_store.update(cx, |this, _| {
            if let Some(state) = this.get_language_server_state(server_id) {
                state.log_level = level;
            }

            this.server_logs(server_id).map(|v| log_contents(v, level))
        });

        if let Some(log_contents) = log_contents {
            self.editor.update(cx, move |editor, cx| {
                editor.set_text(log_contents, cx);
                editor.move_to_end(&MoveToEnd, cx);
            });
            cx.notify();
        }

        cx.focus(&self.focus_handle);
    }

    fn show_trace_for_server(&mut self, server_id: LanguageServerId, cx: &mut ViewContext<Self>) {
        let log_contents = self
            .log_store
            .read(cx)
            .server_trace(server_id)
            .map(|v| log_contents(v, ()));
        if let Some(log_contents) = log_contents {
            self.current_server_id = Some(server_id);
            self.active_entry_kind = LogKind::Trace;
            let (editor, editor_subscriptions) = Self::editor_for_logs(log_contents, cx);
            self.editor = editor;
            self.editor_subscriptions = editor_subscriptions;
            cx.notify();
        }
        cx.focus(&self.focus_handle);
    }

    fn show_rpc_trace_for_server(
        &mut self,
        server_id: LanguageServerId,
        cx: &mut ViewContext<Self>,
    ) {
        let rpc_log = self.log_store.update(cx, |log_store, _| {
            log_store
                .enable_rpc_trace_for_language_server(server_id)
                .map(|state| log_contents(&state.rpc_messages, ()))
        });
        if let Some(rpc_log) = rpc_log {
            self.current_server_id = Some(server_id);
            self.active_entry_kind = LogKind::Rpc;
            let (editor, editor_subscriptions) = Self::editor_for_logs(rpc_log, cx);
            let language = self.project.read(cx).languages().language_for_name("JSON");
            editor
                .read(cx)
                .buffer()
                .read(cx)
                .as_singleton()
                .expect("log buffer should be a singleton")
                .update(cx, |_, cx| {
                    cx.spawn({
                        let buffer = cx.handle();
                        |_, mut cx| async move {
                            let language = language.await.ok();
                            buffer.update(&mut cx, |buffer, cx| {
                                buffer.set_language(language, cx);
                            })
                        }
                    })
                    .detach_and_log_err(cx);
                });

            self.editor = editor;
            self.editor_subscriptions = editor_subscriptions;
            cx.notify();
        }

        cx.focus(&self.focus_handle);
    }

    fn toggle_rpc_trace_for_server(
        &mut self,
        server_id: LanguageServerId,
        enabled: bool,
        cx: &mut ViewContext<Self>,
    ) {
        self.log_store.update(cx, |log_store, _| {
            if enabled {
                log_store.enable_rpc_trace_for_language_server(server_id);
            } else {
                log_store.disable_rpc_trace_for_language_server(server_id);
            }
        });
        if !enabled && Some(server_id) == self.current_server_id {
            self.show_logs_for_server(server_id, cx);
            cx.notify();
        }
    }

    fn update_trace_level(
        &self,
        server_id: LanguageServerId,
        level: TraceValue,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(server) = self.project.read(cx).language_server_for_id(server_id, cx) {
            self.log_store.update(cx, |this, _| {
                if let Some(state) = this.get_language_server_state(server_id) {
                    state.trace_level = level;
                }
            });

            server
                .notify::<SetTrace>(SetTraceParams { value: level })
                .ok();
        }
    }

    fn show_capabilities_for_server(
        &mut self,
        server_id: LanguageServerId,
        cx: &mut ViewContext<Self>,
    ) {
        let capabilities = self.log_store.read(cx).server_capabilities(server_id);

        if let Some(capabilities) = capabilities {
            self.current_server_id = Some(server_id);
            self.active_entry_kind = LogKind::Capabilities;
            let (editor, editor_subscriptions) =
                Self::editor_for_capabilities(capabilities.clone(), cx);
            self.editor = editor;
            self.editor_subscriptions = editor_subscriptions;
            cx.notify();
        }
        cx.focus(&self.focus_handle);
    }
}

fn log_filter<T: Message>(line: &T, cmp: <T as Message>::Level) -> Option<&str> {
    if line.should_include(cmp) {
        Some(line.as_ref())
    } else {
        None
    }
}

fn log_contents<T: Message>(lines: &VecDeque<T>, cmp: <T as Message>::Level) -> String {
    let (a, b) = lines.as_slices();
    let a = a.iter().filter_map(move |v| log_filter(v, cmp));
    let b = b.iter().filter_map(move |v| log_filter(v, cmp));
    a.chain(b).fold(String::new(), |mut acc, el| {
        acc.push_str(el);
        acc.push('\n');
        acc
    })
}

impl Render for LspLogView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        self.editor
            .update(cx, |editor, cx| editor.render(cx).into_any_element())
    }
}

impl FocusableView for LspLogView {
    fn focus_handle(&self, _: &AppContext) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Item for LspLogView {
    type Event = EditorEvent;

    fn to_item_events(event: &Self::Event, f: impl FnMut(workspace::item::ItemEvent)) {
        Editor::to_item_events(event, f)
    }

    fn tab_content_text(&self, _cx: &WindowContext) -> Option<SharedString> {
        Some("LSP Logs".into())
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn as_searchable(&self, handle: &View<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }

    fn clone_on_split(
        &self,
        _workspace_id: Option<WorkspaceId>,
        cx: &mut ViewContext<Self>,
    ) -> Option<View<Self>>
    where
        Self: Sized,
    {
        Some(cx.new_view(|cx| {
            let mut new_view = Self::new(self.project.clone(), self.log_store.clone(), cx);
            if let Some(server_id) = self.current_server_id {
                match self.active_entry_kind {
                    LogKind::Rpc => new_view.show_rpc_trace_for_server(server_id, cx),
                    LogKind::Trace => new_view.show_trace_for_server(server_id, cx),
                    LogKind::Logs => new_view.show_logs_for_server(server_id, cx),
                    LogKind::Capabilities => new_view.show_capabilities_for_server(server_id, cx),
                }
            }
            new_view
        }))
    }
}

impl SearchableItem for LspLogView {
    type Match = <Editor as SearchableItem>::Match;

    fn clear_matches(&mut self, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |e, cx| e.clear_matches(cx))
    }

    fn update_matches(&mut self, matches: &[Self::Match], cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |e, cx| e.update_matches(matches, cx))
    }

    fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> String {
        self.editor.update(cx, |e, cx| e.query_suggestion(cx))
    }

    fn activate_match(
        &mut self,
        index: usize,
        matches: &[Self::Match],
        cx: &mut ViewContext<Self>,
    ) {
        self.editor
            .update(cx, |e, cx| e.activate_match(index, matches, cx))
    }

    fn select_matches(&mut self, matches: &[Self::Match], cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |e, cx| e.select_matches(matches, cx))
    }

    fn find_matches(
        &mut self,
        query: Arc<project::search::SearchQuery>,
        cx: &mut ViewContext<Self>,
    ) -> gpui::Task<Vec<Self::Match>> {
        self.editor.update(cx, |e, cx| e.find_matches(query, cx))
    }

    fn replace(&mut self, _: &Self::Match, _: &SearchQuery, _: &mut ViewContext<Self>) {
        // Since LSP Log is read-only, it doesn't make sense to support replace operation.
    }
    fn supported_options() -> workspace::searchable::SearchOptions {
        workspace::searchable::SearchOptions {
            case: true,
            word: true,
            regex: true,
            // LSP log is read-only.
            replacement: false,
            selection: false,
        }
    }
    fn active_match_index(
        &mut self,
        matches: &[Self::Match],
        cx: &mut ViewContext<Self>,
    ) -> Option<usize> {
        self.editor
            .update(cx, |e, cx| e.active_match_index(matches, cx))
    }
}

impl EventEmitter<ToolbarItemEvent> for LspLogToolbarItemView {}

impl ToolbarItemView for LspLogToolbarItemView {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
    ) -> workspace::ToolbarItemLocation {
        if let Some(item) = active_pane_item {
            if let Some(log_view) = item.downcast::<LspLogView>() {
                self.log_view = Some(log_view.clone());
                self._log_view_subscription = Some(cx.observe(&log_view, |_, _, cx| {
                    cx.notify();
                }));
                return ToolbarItemLocation::PrimaryLeft;
            }
        }
        self.log_view = None;
        self._log_view_subscription = None;
        ToolbarItemLocation::Hidden
    }
}

impl Render for LspLogToolbarItemView {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let Some(log_view) = self.log_view.clone() else {
            return div();
        };
        let (menu_rows, current_server_id) = log_view.update(cx, |log_view, cx| {
            let menu_rows = log_view.menu_items(cx).unwrap_or_default();
            let current_server_id = log_view.current_server_id;
            (menu_rows, current_server_id)
        });

        let current_server = current_server_id.and_then(|current_server_id| {
            if let Ok(ix) = menu_rows.binary_search_by_key(&current_server_id, |e| e.server_id) {
                Some(menu_rows[ix].clone())
            } else {
                None
            }
        });
        let available_language_servers: Vec<_> = menu_rows
            .iter()
            .map(|row| {
                (
                    row.server_id,
                    row.server_name.clone(),
                    row.worktree_root_name.clone(),
                    row.selected_entry,
                )
            })
            .collect();
        let log_toolbar_view = cx.view().clone();
        let lsp_menu = PopoverMenu::new("LspLogView")
            .anchor(AnchorCorner::TopLeft)
            .trigger(Button::new(
                "language_server_menu_header",
                current_server
                    .as_ref()
                    .map(|row| {
                        Cow::Owned(format!(
                            "{} ({})",
                            row.server_name.0, row.worktree_root_name,
                        ))
                    })
                    .unwrap_or_else(|| "No server selected".into()),
            ))
            .menu({
                let log_view = log_view.clone();
                move |cx| {
                    let log_view = log_view.clone();
                    ContextMenu::build(cx, |mut menu, cx| {
                        for (server_id, name, worktree_root, active_entry_kind) in
                            available_language_servers.iter()
                        {
                            let label = format!("{} ({})", name, worktree_root);
                            let server_id = *server_id;
                            let active_entry_kind = *active_entry_kind;
                            menu = menu.entry(
                                label,
                                None,
                                cx.handler_for(&log_view, move |view, cx| {
                                    view.current_server_id = Some(server_id);
                                    view.active_entry_kind = active_entry_kind;
                                    match view.active_entry_kind {
                                        LogKind::Rpc => {
                                            view.toggle_rpc_trace_for_server(server_id, true, cx);
                                            view.show_rpc_trace_for_server(server_id, cx);
                                        }
                                        LogKind::Trace => view.show_trace_for_server(server_id, cx),
                                        LogKind::Logs => view.show_logs_for_server(server_id, cx),
                                        LogKind::Capabilities => {
                                            view.show_capabilities_for_server(server_id, cx)
                                        }
                                    }
                                    cx.notify();
                                }),
                            );
                        }
                        menu
                    })
                    .into()
                }
            });
        let view_selector = current_server.map(|server| {
            let server_id = server.server_id;
            let is_remote = server.server_kind.is_remote();
            let rpc_trace_enabled = server.rpc_trace_enabled;
            let log_view = log_view.clone();
            PopoverMenu::new("LspViewSelector")
                .anchor(AnchorCorner::TopLeft)
                .trigger(Button::new(
                    "language_server_menu_header",
                    server.selected_entry.label(),
                ))
                .menu(move |cx| {
                    let log_toolbar_view = log_toolbar_view.clone();
                    let log_view = log_view.clone();
                    Some(ContextMenu::build(cx, move |this, cx| {
                        this.entry(
                            SERVER_LOGS,
                            None,
                            cx.handler_for(&log_view, move |view, cx| {
                                view.show_logs_for_server(server_id, cx);
                            }),
                        )
                        .when(!is_remote, |this| {
                            this.entry(
                                SERVER_TRACE,
                                None,
                                cx.handler_for(&log_view, move |view, cx| {
                                    view.show_trace_for_server(server_id, cx);
                                }),
                            )
                            .custom_entry(
                                {
                                    let log_toolbar_view = log_toolbar_view.clone();
                                    move |cx| {
                                        h_flex()
                                            .w_full()
                                            .justify_between()
                                            .child(Label::new(RPC_MESSAGES))
                                            .child(
                                                div().child(
                                                    Checkbox::new(
                                                        "LspLogEnableRpcTrace",
                                                        if rpc_trace_enabled {
                                                            Selection::Selected
                                                        } else {
                                                            Selection::Unselected
                                                        },
                                                    )
                                                    .on_click(cx.listener_for(
                                                        &log_toolbar_view,
                                                        move |view, selection, cx| {
                                                            let enabled = matches!(
                                                                selection,
                                                                Selection::Selected
                                                            );
                                                            view.toggle_rpc_logging_for_server(
                                                                server_id, enabled, cx,
                                                            );
                                                            cx.stop_propagation();
                                                        },
                                                    )),
                                                ),
                                            )
                                            .into_any_element()
                                    }
                                },
                                cx.handler_for(&log_view, move |view, cx| {
                                    view.show_rpc_trace_for_server(server_id, cx);
                                }),
                            )
                        })
                        .entry(
                            SERVER_CAPABILITIES,
                            None,
                            cx.handler_for(&log_view, move |view, cx| {
                                view.show_capabilities_for_server(server_id, cx);
                            }),
                        )
                    }))
                })
        });
        h_flex()
            .size_full()
            .justify_between()
            .child(
                h_flex()
                    .child(lsp_menu)
                    .children(view_selector)
                    .child(log_view.update(cx, |this, _| match this.active_entry_kind {
                        LogKind::Trace => {
                            let log_view = log_view.clone();
                            div().child(
                                PopoverMenu::new("lsp-trace-level-menu")
                                    .anchor(AnchorCorner::TopLeft)
                                    .trigger(Button::new(
                                        "language_server_trace_level_selector",
                                        "Trace level",
                                    ))
                                    .menu({
                                        let log_view = log_view.clone();

                                        move |cx| {
                                            let id = log_view.read(cx).current_server_id?;

                                            let trace_level = log_view.update(cx, |this, cx| {
                                                this.log_store.update(cx, |this, _| {
                                                    Some(
                                                        this.get_language_server_state(id)?
                                                            .trace_level,
                                                    )
                                                })
                                            })?;

                                            ContextMenu::build(cx, |mut menu, _| {
                                                let log_view = log_view.clone();

                                                for (option, label) in [
                                                    (TraceValue::Off, "Off"),
                                                    (TraceValue::Messages, "Messages"),
                                                    (TraceValue::Verbose, "Verbose"),
                                                ] {
                                                    menu = menu.entry(label, None, {
                                                        let log_view = log_view.clone();
                                                        move |cx| {
                                                            log_view.update(cx, |this, cx| {
                                                                if let Some(id) =
                                                                    this.current_server_id
                                                                {
                                                                    this.update_trace_level(
                                                                        id, option, cx,
                                                                    );
                                                                }
                                                            });
                                                        }
                                                    });
                                                    if option == trace_level {
                                                        menu.select_last();
                                                    }
                                                }

                                                menu
                                            })
                                            .into()
                                        }
                                    }),
                            )
                        }
                        LogKind::Logs => {
                            let log_view = log_view.clone();
                            div().child(
                                PopoverMenu::new("lsp-log-level-menu")
                                    .anchor(AnchorCorner::TopLeft)
                                    .trigger(Button::new(
                                        "language_server_log_level_selector",
                                        "Log level",
                                    ))
                                    .menu({
                                        let log_view = log_view.clone();

                                        move |cx| {
                                            let id = log_view.read(cx).current_server_id?;

                                            let log_level = log_view.update(cx, |this, cx| {
                                                this.log_store.update(cx, |this, _| {
                                                    Some(
                                                        this.get_language_server_state(id)?
                                                            .log_level,
                                                    )
                                                })
                                            })?;

                                            ContextMenu::build(cx, |mut menu, _| {
                                                let log_view = log_view.clone();

                                                for (option, label) in [
                                                    (MessageType::LOG, "Log"),
                                                    (MessageType::INFO, "Info"),
                                                    (MessageType::WARNING, "Warning"),
                                                    (MessageType::ERROR, "Error"),
                                                ] {
                                                    menu = menu.entry(label, None, {
                                                        let log_view = log_view.clone();
                                                        move |cx| {
                                                            log_view.update(cx, |this, cx| {
                                                                if let Some(id) =
                                                                    this.current_server_id
                                                                {
                                                                    this.update_log_level(
                                                                        id, option, cx,
                                                                    );
                                                                }
                                                            });
                                                        }
                                                    });
                                                    if option == log_level {
                                                        menu.select_last();
                                                    }
                                                }

                                                menu
                                            })
                                            .into()
                                        }
                                    }),
                            )
                        }
                        _ => div(),
                    })),
            )
            .child(
                div()
                    .child(
                        Button::new("clear_log_button", "Clear").on_click(cx.listener(
                            |this, _, cx| {
                                if let Some(log_view) = this.log_view.as_ref() {
                                    log_view.update(cx, |log_view, cx| {
                                        log_view.editor.update(cx, |editor, cx| {
                                            editor.set_read_only(false);
                                            editor.clear(cx);
                                            editor.set_read_only(true);
                                        });
                                    })
                                }
                            },
                        )),
                    )
                    .ml_2(),
            )
    }
}

const RPC_MESSAGES: &str = "RPC Messages";
const SERVER_LOGS: &str = "Server Logs";
const SERVER_TRACE: &str = "Server Trace";
const SERVER_CAPABILITIES: &str = "Server Capabilities";

impl Default for LspLogToolbarItemView {
    fn default() -> Self {
        Self::new()
    }
}

impl LspLogToolbarItemView {
    pub fn new() -> Self {
        Self {
            log_view: None,
            _log_view_subscription: None,
        }
    }

    fn toggle_rpc_logging_for_server(
        &mut self,
        id: LanguageServerId,
        enabled: bool,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(log_view) = &self.log_view {
            log_view.update(cx, |log_view, cx| {
                log_view.toggle_rpc_trace_for_server(id, enabled, cx);
                if !enabled && Some(id) == log_view.current_server_id {
                    log_view.show_logs_for_server(id, cx);
                    cx.notify();
                } else if enabled {
                    log_view.show_rpc_trace_for_server(id, cx);
                    cx.notify();
                }
                cx.focus(&log_view.focus_handle);
            });
        }
        cx.notify();
    }
}

pub enum Event {
    NewServerLogEntry {
        id: LanguageServerId,
        entry: String,
        kind: LogKind,
    },
}

impl EventEmitter<Event> for LogStore {}
impl EventEmitter<Event> for LspLogView {}
impl EventEmitter<EditorEvent> for LspLogView {}
impl EventEmitter<SearchEvent> for LspLogView {}
