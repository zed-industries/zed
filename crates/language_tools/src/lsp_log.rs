use collections::{HashMap, VecDeque};
use copilot::Copilot;
use editor::{actions::MoveToEnd, Editor, EditorEvent};
use futures::{channel::mpsc, StreamExt};
use gpui::{
    actions, div, AnchorCorner, AnyElement, AppContext, Context, EventEmitter, FocusHandle,
    FocusableView, IntoElement, Model, ModelContext, ParentElement, Render, Styled, Subscription,
    View, ViewContext, VisualContext, WeakModel, WindowContext,
};
use language::{LanguageServerId, LanguageServerName};
use lsp::{IoKind, LanguageServer};
use project::{search::SearchQuery, Project};
use std::{borrow::Cow, sync::Arc};
use ui::{popover_menu, prelude::*, Button, Checkbox, ContextMenu, Label, Selection};
use workspace::{
    item::{Item, ItemHandle, TabContentParams},
    searchable::{SearchEvent, SearchableItem, SearchableItemHandle},
    ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
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

struct LanguageServerState {
    name: LanguageServerName,
    log_messages: VecDeque<String>,
    rpc_state: Option<LanguageServerRpcState>,
    project: Option<WeakModel<Project>>,
    _io_logs_subscription: Option<lsp::Subscription>,
    _lsp_logs_subscription: Option<lsp::Subscription>,
}

struct LanguageServerRpcState {
    rpc_messages: VecDeque<String>,
    last_message_kind: Option<MessageKind>,
}

pub struct LspLogView {
    pub(crate) editor: View<Editor>,
    editor_subscriptions: Vec<Subscription>,
    log_store: Model<LogStore>,
    current_server_id: Option<LanguageServerId>,
    is_showing_rpc_trace: bool,
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

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct LogMenuItem {
    pub server_id: LanguageServerId,
    pub server_name: LanguageServerName,
    pub worktree_root_name: String,
    pub rpc_trace_enabled: bool,
    pub rpc_trace_selected: bool,
    pub logs_selected: bool,
}

actions!(debug, [OpenLanguageServerLogs]);

pub fn init(cx: &mut AppContext) {
    let log_store = cx.new_model(|cx| LogStore::new(cx));

    cx.observe_new_views(move |workspace: &mut Workspace, cx| {
        let project = workspace.project();
        if project.read(cx).is_local() {
            log_store.update(cx, |store, cx| {
                store.add_project(&project, cx);
            });
        }

        let log_store = log_store.clone();
        workspace.register_action(move |workspace, _: &OpenLanguageServerLogs, cx| {
            let project = workspace.project().read(cx);
            if project.is_local() {
                workspace.add_item_to_active_pane(
                    Box::new(cx.new_view(|cx| {
                        LspLogView::new(workspace.project().clone(), log_store.clone(), cx)
                    })),
                    None,
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
            cx.subscribe(
                copilot,
                |this, copilot, copilot_event, cx| match copilot_event {
                    copilot::Event::CopilotLanguageServerStarted => {
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
                                                    &params.message,
                                                    cx,
                                                );
                                            })
                                            .ok();
                                    },
                                ));
                            this.add_language_server(
                                None,
                                LanguageServerName(Arc::from("copilot")),
                                server.clone(),
                                cx,
                            );
                        }
                    }
                },
            )
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
                            .retain(|_, state| state.project.as_ref() != Some(&weak_project));
                    }),
                    cx.subscribe(project, |this, project, event, cx| match event {
                        project::Event::LanguageServerAdded(id) => {
                            let read_project = project.read(cx);
                            if let Some((server, adapter)) = read_project
                                .language_server_for_id(*id)
                                .zip(read_project.language_server_adapter_for_id(*id))
                            {
                                this.add_language_server(
                                    Some(&project.downgrade()),
                                    adapter.name.clone(),
                                    server,
                                    cx,
                                );
                            }
                        }
                        project::Event::LanguageServerRemoved(id) => {
                            this.remove_language_server(*id, cx);
                        }
                        project::Event::LanguageServerLog(id, message) => {
                            this.add_language_server_log(*id, message, cx);
                        }
                        _ => {}
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
        project: Option<&WeakModel<Project>>,
        name: LanguageServerName,
        server: Arc<LanguageServer>,
        cx: &mut ModelContext<Self>,
    ) -> Option<&mut LanguageServerState> {
        let server_state = self
            .language_servers
            .entry(server.server_id())
            .or_insert_with(|| {
                cx.notify();
                LanguageServerState {
                    name,
                    rpc_state: None,
                    project: project.cloned(),
                    log_messages: VecDeque::with_capacity(MAX_STORED_LOG_ENTRIES),
                    _io_logs_subscription: None,
                    _lsp_logs_subscription: None,
                }
            });

        if server.has_notification_handler::<lsp::notification::LogMessage>() {
            // Another event wants to re-add the server that was already added and subscribed to, avoid doing it again.
            return Some(server_state);
        }

        let io_tx = self.io_tx.clone();
        let server_id = server.server_id();
        server_state._io_logs_subscription = Some(server.on_io(move |io_kind, message| {
            io_tx
                .unbounded_send((server_id, io_kind, message.to_string()))
                .ok();
        }));
        let this = cx.handle().downgrade();
        server_state._lsp_logs_subscription =
            Some(server.on_notification::<lsp::notification::LogMessage, _>({
                move |params, mut cx| {
                    if let Some(this) = this.upgrade() {
                        this.update(&mut cx, |this, cx| {
                            this.add_language_server_log(server_id, &params.message, cx);
                        })
                        .ok();
                    }
                }
            }));
        Some(server_state)
    }

    fn add_language_server_log(
        &mut self,
        id: LanguageServerId,
        message: &str,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        let language_server_state = self.get_language_server_state(id)?;

        let log_lines = &mut language_server_state.log_messages;
        while log_lines.len() >= MAX_STORED_LOG_ENTRIES {
            log_lines.pop_front();
        }
        let message = message.trim();
        log_lines.push_back(message.to_string());
        cx.emit(Event::NewServerLogEntry {
            id,
            entry: message.to_string(),
            is_rpc: false,
        });
        cx.notify();
        Some(())
    }

    fn remove_language_server(&mut self, id: LanguageServerId, cx: &mut ModelContext<Self>) {
        self.language_servers.remove(&id);
        cx.notify();
    }

    fn server_logs(&self, server_id: LanguageServerId) -> Option<&VecDeque<String>> {
        Some(&self.language_servers.get(&server_id)?.log_messages)
    }

    fn server_ids_for_project<'a>(
        &'a self,
        project: &'a WeakModel<Project>,
    ) -> impl Iterator<Item = LanguageServerId> + 'a {
        [].into_iter()
            .chain(self.language_servers.iter().filter_map(|(id, state)| {
                if state.project.as_ref() == Some(project) {
                    return Some(*id);
                } else {
                    None
                }
            }))
            .chain(self.language_servers.iter().filter_map(|(id, state)| {
                if state.project.is_none() {
                    return Some(*id);
                } else {
                    None
                }
            }))
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
                self.add_language_server_log(language_server_id, &message, cx);
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
            rpc_log_lines.push_back(line_before_message.to_string());
            cx.emit(Event::NewServerLogEntry {
                id: language_server_id,
                entry: line_before_message.to_string(),
                is_rpc: true,
            });
        }

        while rpc_log_lines.len() >= MAX_STORED_LOG_ENTRIES {
            rpc_log_lines.pop_front();
        }
        let message = message.trim();
        rpc_log_lines.push_back(message.to_string());
        cx.emit(Event::NewServerLogEntry {
            id: language_server_id,
            entry: message.to_string(),
            is_rpc: true,
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
            .find(|(_, server)| server.project == Some(project.downgrade()))
            .map(|(id, _)| *id);

        let weak_project = project.downgrade();
        let model_changes_subscription = cx.observe(&log_store, move |this, store, cx| {
            let first_server_id_for_project =
                store.read(cx).server_ids_for_project(&weak_project).next();
            if let Some(current_lsp) = this.current_server_id {
                if !store.read(cx).language_servers.contains_key(&current_lsp) {
                    if let Some(server_id) = first_server_id_for_project {
                        if this.is_showing_rpc_trace {
                            this.show_rpc_trace_for_server(server_id, cx)
                        } else {
                            this.show_logs_for_server(server_id, cx)
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
                if this.is_showing_rpc_trace {
                    this.show_rpc_trace_for_server(server_id, cx)
                } else {
                    this.show_logs_for_server(server_id, cx)
                }
            }

            cx.notify();
        });
        let events_subscriptions = cx.subscribe(&log_store, |log_view, _, e, cx| match e {
            Event::NewServerLogEntry { id, entry, is_rpc } => {
                if log_view.current_server_id == Some(*id) {
                    if (*is_rpc && log_view.is_showing_rpc_trace)
                        || (!*is_rpc && !log_view.is_showing_rpc_trace)
                    {
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
            is_showing_rpc_trace: false,
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
            editor.set_show_inline_completions(false);
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

        let mut rows = self
            .project
            .read(cx)
            .language_servers()
            .filter_map(|(server_id, language_server_name, worktree_id)| {
                let worktree = self.project.read(cx).worktree_for_id(worktree_id, cx)?;
                let state = log_store.language_servers.get(&server_id)?;
                Some(LogMenuItem {
                    server_id,
                    server_name: language_server_name,
                    worktree_root_name: worktree.read(cx).root_name().to_string(),
                    rpc_trace_enabled: state.rpc_state.is_some(),
                    rpc_trace_selected: self.is_showing_rpc_trace
                        && self.current_server_id == Some(server_id),
                    logs_selected: !self.is_showing_rpc_trace
                        && self.current_server_id == Some(server_id),
                })
            })
            .chain(
                self.project
                    .read(cx)
                    .supplementary_language_servers()
                    .filter_map(|(&server_id, (name, _))| {
                        let state = log_store.language_servers.get(&server_id)?;
                        Some(LogMenuItem {
                            server_id,
                            server_name: name.clone(),
                            worktree_root_name: "supplementary".to_string(),
                            rpc_trace_enabled: state.rpc_state.is_some(),
                            rpc_trace_selected: self.is_showing_rpc_trace
                                && self.current_server_id == Some(server_id),
                            logs_selected: !self.is_showing_rpc_trace
                                && self.current_server_id == Some(server_id),
                        })
                    }),
            )
            .chain(
                log_store
                    .language_servers
                    .iter()
                    .filter_map(|(server_id, state)| {
                        if state.project.is_none() {
                            Some(LogMenuItem {
                                server_id: *server_id,
                                server_name: state.name.clone(),
                                worktree_root_name: "supplementary".to_string(),
                                rpc_trace_enabled: state.rpc_state.is_some(),
                                rpc_trace_selected: self.is_showing_rpc_trace
                                    && self.current_server_id == Some(*server_id),
                                logs_selected: !self.is_showing_rpc_trace
                                    && self.current_server_id == Some(*server_id),
                            })
                        } else {
                            None
                        }
                    }),
            )
            .collect::<Vec<_>>();
        rows.sort_by_key(|row| row.server_id);
        rows.dedup_by_key(|row| row.server_id);
        Some(rows)
    }

    fn show_logs_for_server(&mut self, server_id: LanguageServerId, cx: &mut ViewContext<Self>) {
        let log_contents = self
            .log_store
            .read(cx)
            .server_logs(server_id)
            .map(log_contents);
        if let Some(log_contents) = log_contents {
            self.current_server_id = Some(server_id);
            self.is_showing_rpc_trace = false;
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
                .map(|state| log_contents(&state.rpc_messages))
        });
        if let Some(rpc_log) = rpc_log {
            self.current_server_id = Some(server_id);
            self.is_showing_rpc_trace = true;
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
}

fn log_contents(lines: &VecDeque<String>) -> String {
    let (a, b) = lines.as_slices();
    let log_contents = a.join("\n");
    if b.is_empty() {
        log_contents
    } else {
        log_contents + "\n" + &b.join("\n")
    }
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

    fn tab_content(&self, params: TabContentParams, _: &WindowContext<'_>) -> AnyElement {
        Label::new("LSP Logs")
            .color(if params.selected {
                Color::Default
            } else {
                Color::Muted
            })
            .into_any_element()
    }

    fn telemetry_event_text(&self) -> Option<&'static str> {
        None
    }

    fn as_searchable(&self, handle: &View<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
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

        let log_toolbar_view = cx.view().clone();
        let lsp_menu = popover_menu("LspLogView")
            .anchor(AnchorCorner::TopLeft)
            .trigger(Button::new(
                "language_server_menu_header",
                current_server
                    .map(|row| {
                        Cow::Owned(format!(
                            "{} ({}) - {}",
                            row.server_name.0,
                            row.worktree_root_name,
                            if row.rpc_trace_selected {
                                RPC_MESSAGES
                            } else {
                                SERVER_LOGS
                            },
                        ))
                    })
                    .unwrap_or_else(|| "No server selected".into()),
            ))
            .menu(move |cx| {
                let menu_rows = menu_rows.clone();
                let log_view = log_view.clone();
                let log_toolbar_view = log_toolbar_view.clone();
                ContextMenu::build(cx, move |mut menu, cx| {
                    for (ix, row) in menu_rows.into_iter().enumerate() {
                        let server_selected = Some(row.server_id) == current_server_id;
                        menu = menu
                            .header(format!(
                                "{} ({})",
                                row.server_name.0, row.worktree_root_name
                            ))
                            .entry(
                                SERVER_LOGS,
                                None,
                                cx.handler_for(&log_view, move |view, cx| {
                                    view.show_logs_for_server(row.server_id, cx);
                                }),
                            );
                        if server_selected && row.logs_selected {
                            let selected_ix = menu.select_last();
                            debug_assert_eq!(
                                Some(ix * 3 + 1),
                                selected_ix,
                                "Could not scroll to a just added LSP menu item"
                            );
                        }

                        menu = menu.custom_entry(
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
                                                    ix,
                                                    if row.rpc_trace_enabled {
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
                                                            row.server_id,
                                                            enabled,
                                                            cx,
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
                                view.show_rpc_trace_for_server(row.server_id, cx);
                            }),
                        );
                        if server_selected && row.rpc_trace_selected {
                            let selected_ix = menu.select_last();
                            debug_assert_eq!(
                                Some(ix * 3 + 2),
                                selected_ix,
                                "Could not scroll to a just added LSP menu item"
                            );
                        }
                    }
                    menu
                })
                .into()
            });

        h_flex().size_full().child(lsp_menu).child(
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
        is_rpc: bool,
    },
}

impl EventEmitter<Event> for LogStore {}
impl EventEmitter<Event> for LspLogView {}
impl EventEmitter<EditorEvent> for LspLogView {}
impl EventEmitter<SearchEvent> for LspLogView {}
