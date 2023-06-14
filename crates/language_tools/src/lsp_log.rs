use collections::HashMap;
use editor::Editor;
use futures::{channel::mpsc, StreamExt};
use gpui::{
    actions,
    elements::{
        AnchorCorner, ChildView, Empty, Flex, Label, MouseEventHandler, Overlay, OverlayFitMode,
        ParentElement, Stack,
    },
    platform::{CursorStyle, MouseButton},
    AnyElement, AppContext, Element, Entity, ModelContext, ModelHandle, View, ViewContext,
    ViewHandle, WeakModelHandle,
};
use language::{Buffer, LanguageServerId, LanguageServerName};
use project::{Project, Worktree};
use std::{borrow::Cow, sync::Arc};
use theme::{ui, Theme};
use workspace::{
    item::{Item, ItemHandle},
    searchable::{SearchableItem, SearchableItemHandle},
    ToolbarItemLocation, ToolbarItemView, Workspace, WorkspaceCreated,
};

const SEND_LINE: &str = "// Send:\n";
const RECEIVE_LINE: &str = "// Receive:\n";

pub struct LogStore {
    projects: HashMap<WeakModelHandle<Project>, ProjectState>,
    io_tx: mpsc::UnboundedSender<(WeakModelHandle<Project>, LanguageServerId, bool, String)>,
}

struct ProjectState {
    servers: HashMap<LanguageServerId, LanguageServerState>,
    _subscriptions: [gpui::Subscription; 2],
}

struct LanguageServerState {
    log_buffer: ModelHandle<Buffer>,
    rpc_state: Option<LanguageServerRpcState>,
}

struct LanguageServerRpcState {
    buffer: ModelHandle<Buffer>,
    last_message_kind: Option<MessageKind>,
    _subscription: lsp::Subscription,
}

pub struct LspLogView {
    pub(crate) editor: ViewHandle<Editor>,
    log_store: ModelHandle<LogStore>,
    current_server_id: Option<LanguageServerId>,
    is_showing_rpc_trace: bool,
    project: ModelHandle<Project>,
}

pub struct LspLogToolbarItemView {
    log_view: Option<ViewHandle<LspLogView>>,
    menu_open: bool,
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
    pub worktree: ModelHandle<Worktree>,
    pub rpc_trace_enabled: bool,
    pub rpc_trace_selected: bool,
    pub logs_selected: bool,
}

actions!(debug, [OpenLanguageServerLogs]);

pub fn init(cx: &mut AppContext) {
    let log_store = cx.add_model(|cx| LogStore::new(cx));

    cx.subscribe_global::<WorkspaceCreated, _>({
        let log_store = log_store.clone();
        move |event, cx| {
            let workspace = &event.0;
            if let Some(workspace) = workspace.upgrade(cx) {
                let project = workspace.read(cx).project().clone();
                if project.read(cx).is_local() {
                    log_store.update(cx, |store, cx| {
                        store.add_project(&project, cx);
                    });
                }
            }
        }
    })
    .detach();

    cx.add_action(
        move |workspace: &mut Workspace, _: &OpenLanguageServerLogs, cx: _| {
            let project = workspace.project().read(cx);
            if project.is_local() {
                workspace.add_item(
                    Box::new(cx.add_view(|cx| {
                        LspLogView::new(workspace.project().clone(), log_store.clone(), cx)
                    })),
                    cx,
                );
            }
        },
    );
}

impl LogStore {
    pub fn new(cx: &mut ModelContext<Self>) -> Self {
        let (io_tx, mut io_rx) = mpsc::unbounded();
        let this = Self {
            projects: HashMap::default(),
            io_tx,
        };
        cx.spawn_weak(|this, mut cx| async move {
            while let Some((project, server_id, is_output, mut message)) = io_rx.next().await {
                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| {
                        message.push('\n');
                        this.on_io(project, server_id, is_output, &message, cx);
                    });
                }
            }
            anyhow::Ok(())
        })
        .detach();
        this
    }

    pub fn add_project(&mut self, project: &ModelHandle<Project>, cx: &mut ModelContext<Self>) {
        use project::Event::*;

        let weak_project = project.downgrade();
        self.projects.insert(
            weak_project,
            ProjectState {
                servers: HashMap::default(),
                _subscriptions: [
                    cx.observe_release(&project, move |this, _, _| {
                        this.projects.remove(&weak_project);
                    }),
                    cx.subscribe(project, |this, project, event, cx| match event {
                        LanguageServerAdded(id) => {
                            this.add_language_server(&project, *id, cx);
                        }
                        LanguageServerRemoved(id) => {
                            this.remove_language_server(&project, *id, cx);
                        }
                        LanguageServerLog(id, message) => {
                            this.add_language_server_log(&project, *id, message, cx);
                        }
                        _ => {}
                    }),
                ],
            },
        );
    }

    fn add_language_server(
        &mut self,
        project: &ModelHandle<Project>,
        id: LanguageServerId,
        cx: &mut ModelContext<Self>,
    ) -> Option<ModelHandle<Buffer>> {
        let project_state = self.projects.get_mut(&project.downgrade())?;
        Some(
            project_state
                .servers
                .entry(id)
                .or_insert_with(|| {
                    cx.notify();
                    LanguageServerState {
                        rpc_state: None,
                        log_buffer: cx.add_model(|cx| Buffer::new(0, "", cx)).clone(),
                    }
                })
                .log_buffer
                .clone(),
        )
    }

    fn add_language_server_log(
        &mut self,
        project: &ModelHandle<Project>,
        id: LanguageServerId,
        message: &str,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        let buffer = self.add_language_server(&project, id, cx)?;
        buffer.update(cx, |buffer, cx| {
            let len = buffer.len();
            let has_newline = message.ends_with("\n");
            buffer.edit([(len..len, message)], None, cx);
            if !has_newline {
                let len = buffer.len();
                buffer.edit([(len..len, "\n")], None, cx);
            }
        });
        cx.notify();
        Some(())
    }

    fn remove_language_server(
        &mut self,
        project: &ModelHandle<Project>,
        id: LanguageServerId,
        cx: &mut ModelContext<Self>,
    ) -> Option<()> {
        let project_state = self.projects.get_mut(&project.downgrade())?;
        project_state.servers.remove(&id);
        cx.notify();
        Some(())
    }

    pub fn log_buffer_for_server(
        &self,
        project: &ModelHandle<Project>,
        server_id: LanguageServerId,
    ) -> Option<ModelHandle<Buffer>> {
        let weak_project = project.downgrade();
        let project_state = self.projects.get(&weak_project)?;
        let server_state = project_state.servers.get(&server_id)?;
        Some(server_state.log_buffer.clone())
    }

    pub fn enable_rpc_trace_for_language_server(
        &mut self,
        project: &ModelHandle<Project>,
        server_id: LanguageServerId,
        cx: &mut ModelContext<Self>,
    ) -> Option<ModelHandle<Buffer>> {
        let weak_project = project.downgrade();
        let project_state = self.projects.get_mut(&weak_project)?;
        let server_state = project_state.servers.get_mut(&server_id)?;
        let server = project.read(cx).language_server_for_id(server_id)?;
        let rpc_state = server_state.rpc_state.get_or_insert_with(|| {
            let io_tx = self.io_tx.clone();
            let language = project.read(cx).languages().language_for_name("JSON");
            let buffer = cx.add_model(|cx| Buffer::new(0, "", cx));
            cx.spawn_weak({
                let buffer = buffer.clone();
                |_, mut cx| async move {
                    let language = language.await.ok();
                    buffer.update(&mut cx, |buffer, cx| {
                        buffer.set_language(language, cx);
                    });
                }
            })
            .detach();

            LanguageServerRpcState {
                buffer,
                last_message_kind: None,
                _subscription: server.on_io(move |is_received, json| {
                    io_tx
                        .unbounded_send((weak_project, server_id, is_received, json.to_string()))
                        .ok();
                }),
            }
        });
        Some(rpc_state.buffer.clone())
    }

    pub fn disable_rpc_trace_for_language_server(
        &mut self,
        project: &ModelHandle<Project>,
        server_id: LanguageServerId,
        _: &mut ModelContext<Self>,
    ) -> Option<()> {
        let project = project.downgrade();
        let project_state = self.projects.get_mut(&project)?;
        let server_state = project_state.servers.get_mut(&server_id)?;
        server_state.rpc_state.take();
        Some(())
    }

    fn on_io(
        &mut self,
        project: WeakModelHandle<Project>,
        language_server_id: LanguageServerId,
        is_received: bool,
        message: &str,
        cx: &mut AppContext,
    ) -> Option<()> {
        let state = self
            .projects
            .get_mut(&project)?
            .servers
            .get_mut(&language_server_id)?
            .rpc_state
            .as_mut()?;
        state.buffer.update(cx, |buffer, cx| {
            let kind = if is_received {
                MessageKind::Receive
            } else {
                MessageKind::Send
            };
            if state.last_message_kind != Some(kind) {
                let len = buffer.len();
                let line = match kind {
                    MessageKind::Send => SEND_LINE,
                    MessageKind::Receive => RECEIVE_LINE,
                };
                buffer.edit([(len..len, line)], None, cx);
                state.last_message_kind = Some(kind);
            }
            let len = buffer.len();
            buffer.edit([(len..len, message)], None, cx);
        });
        Some(())
    }
}

impl LspLogView {
    pub fn new(
        project: ModelHandle<Project>,
        log_store: ModelHandle<LogStore>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let server_id = log_store
            .read(cx)
            .projects
            .get(&project.downgrade())
            .and_then(|project| project.servers.keys().copied().next());
        let buffer = cx.add_model(|cx| Buffer::new(0, "", cx));
        let mut this = Self {
            editor: Self::editor_for_buffer(project.clone(), buffer, cx),
            project,
            log_store,
            current_server_id: None,
            is_showing_rpc_trace: false,
        };
        if let Some(server_id) = server_id {
            this.show_logs_for_server(server_id, cx);
        }
        this
    }

    fn editor_for_buffer(
        project: ModelHandle<Project>,
        buffer: ModelHandle<Buffer>,
        cx: &mut ViewContext<Self>,
    ) -> ViewHandle<Editor> {
        let editor = cx.add_view(|cx| {
            let mut editor = Editor::for_buffer(buffer, Some(project), cx);
            editor.set_read_only(true);
            editor.move_to_end(&Default::default(), cx);
            editor
        });
        cx.subscribe(&editor, |_, _, event, cx| cx.emit(event.clone()))
            .detach();
        editor
    }

    pub(crate) fn menu_items<'a>(&'a self, cx: &'a AppContext) -> Option<Vec<LogMenuItem>> {
        let log_store = self.log_store.read(cx);
        let state = log_store.projects.get(&self.project.downgrade())?;
        let mut rows = self
            .project
            .read(cx)
            .language_servers()
            .filter_map(|(server_id, language_server_name, worktree_id)| {
                let worktree = self.project.read(cx).worktree_for_id(worktree_id, cx)?;
                let state = state.servers.get(&server_id)?;
                Some(LogMenuItem {
                    server_id,
                    server_name: language_server_name,
                    worktree,
                    rpc_trace_enabled: state.rpc_state.is_some(),
                    rpc_trace_selected: self.is_showing_rpc_trace
                        && self.current_server_id == Some(server_id),
                    logs_selected: !self.is_showing_rpc_trace
                        && self.current_server_id == Some(server_id),
                })
            })
            .collect::<Vec<_>>();
        rows.sort_by_key(|row| row.server_id);
        rows.dedup_by_key(|row| row.server_id);
        Some(rows)
    }

    fn show_logs_for_server(&mut self, server_id: LanguageServerId, cx: &mut ViewContext<Self>) {
        let buffer = self
            .log_store
            .read(cx)
            .log_buffer_for_server(&self.project, server_id);
        if let Some(buffer) = buffer {
            self.current_server_id = Some(server_id);
            self.is_showing_rpc_trace = false;
            self.editor = Self::editor_for_buffer(self.project.clone(), buffer, cx);
            cx.notify();
        }
    }

    fn show_rpc_trace_for_server(
        &mut self,
        server_id: LanguageServerId,
        cx: &mut ViewContext<Self>,
    ) {
        let buffer = self.log_store.update(cx, |log_set, cx| {
            log_set.enable_rpc_trace_for_language_server(&self.project, server_id, cx)
        });
        if let Some(buffer) = buffer {
            self.current_server_id = Some(server_id);
            self.is_showing_rpc_trace = true;
            self.editor = Self::editor_for_buffer(self.project.clone(), buffer, cx);
            cx.notify();
        }
    }

    fn toggle_rpc_trace_for_server(
        &mut self,
        server_id: LanguageServerId,
        enabled: bool,
        cx: &mut ViewContext<Self>,
    ) {
        self.log_store.update(cx, |log_store, cx| {
            if enabled {
                log_store.enable_rpc_trace_for_language_server(&self.project, server_id, cx);
            } else {
                log_store.disable_rpc_trace_for_language_server(&self.project, server_id, cx);
            }
        });
        if !enabled && Some(server_id) == self.current_server_id {
            self.show_logs_for_server(server_id, cx);
            cx.notify();
        }
    }
}

impl View for LspLogView {
    fn ui_name() -> &'static str {
        "LspLogView"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        ChildView::new(&self.editor, cx).into_any()
    }

    fn focus_in(&mut self, _: gpui::AnyViewHandle, cx: &mut ViewContext<Self>) {
        if cx.is_self_focused() {
            cx.focus(&self.editor);
        }
    }
}

impl Item for LspLogView {
    fn tab_content<V: View>(
        &self,
        _: Option<usize>,
        style: &theme::Tab,
        _: &AppContext,
    ) -> AnyElement<V> {
        Label::new("LSP Logs", style.label.clone()).into_any()
    }

    fn as_searchable(&self, handle: &ViewHandle<Self>) -> Option<Box<dyn SearchableItemHandle>> {
        Some(Box::new(handle.clone()))
    }
}

impl SearchableItem for LspLogView {
    type Match = <Editor as SearchableItem>::Match;

    fn to_search_event(event: &Self::Event) -> Option<workspace::searchable::SearchEvent> {
        Editor::to_search_event(event)
    }

    fn clear_matches(&mut self, cx: &mut ViewContext<Self>) {
        self.editor.update(cx, |e, cx| e.clear_matches(cx))
    }

    fn update_matches(&mut self, matches: Vec<Self::Match>, cx: &mut ViewContext<Self>) {
        self.editor
            .update(cx, |e, cx| e.update_matches(matches, cx))
    }

    fn query_suggestion(&mut self, cx: &mut ViewContext<Self>) -> String {
        self.editor.update(cx, |e, cx| e.query_suggestion(cx))
    }

    fn activate_match(
        &mut self,
        index: usize,
        matches: Vec<Self::Match>,
        cx: &mut ViewContext<Self>,
    ) {
        self.editor
            .update(cx, |e, cx| e.activate_match(index, matches, cx))
    }

    fn find_matches(
        &mut self,
        query: project::search::SearchQuery,
        cx: &mut ViewContext<Self>,
    ) -> gpui::Task<Vec<Self::Match>> {
        self.editor.update(cx, |e, cx| e.find_matches(query, cx))
    }

    fn active_match_index(
        &mut self,
        matches: Vec<Self::Match>,
        cx: &mut ViewContext<Self>,
    ) -> Option<usize> {
        self.editor
            .update(cx, |e, cx| e.active_match_index(matches, cx))
    }
}

impl ToolbarItemView for LspLogToolbarItemView {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _: &mut ViewContext<Self>,
    ) -> workspace::ToolbarItemLocation {
        self.menu_open = false;
        if let Some(item) = active_pane_item {
            if let Some(log_view) = item.downcast::<LspLogView>() {
                self.log_view = Some(log_view.clone());
                return ToolbarItemLocation::PrimaryLeft {
                    flex: Some((1., false)),
                };
            }
        }
        self.log_view = None;
        ToolbarItemLocation::Hidden
    }
}

impl View for LspLogToolbarItemView {
    fn ui_name() -> &'static str {
        "LspLogView"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = theme::current(cx).clone();
        let Some(log_view) = self.log_view.as_ref() else { return Empty::new().into_any() };
        let log_view = log_view.read(cx);
        let menu_rows = log_view.menu_items(cx).unwrap_or_default();

        let current_server_id = log_view.current_server_id;
        let current_server = current_server_id.and_then(|current_server_id| {
            if let Ok(ix) = menu_rows.binary_search_by_key(&current_server_id, |e| e.server_id) {
                Some(menu_rows[ix].clone())
            } else {
                None
            }
        });

        enum Menu {}

        Stack::new()
            .with_child(Self::render_language_server_menu_header(
                current_server,
                &theme,
                cx,
            ))
            .with_children(if self.menu_open {
                Some(
                    Overlay::new(
                        MouseEventHandler::<Menu, _>::new(0, cx, move |_, cx| {
                            Flex::column()
                                .with_children(menu_rows.into_iter().map(|row| {
                                    Self::render_language_server_menu_item(
                                        row.server_id,
                                        row.server_name,
                                        row.worktree,
                                        row.rpc_trace_enabled,
                                        row.logs_selected,
                                        row.rpc_trace_selected,
                                        &theme,
                                        cx,
                                    )
                                }))
                                .contained()
                                .with_style(theme.toolbar_dropdown_menu.container)
                                .constrained()
                                .with_width(400.)
                                .with_height(400.)
                        })
                        .on_down_out(MouseButton::Left, |_, this, cx| {
                            this.menu_open = false;
                            cx.notify()
                        }),
                    )
                    .with_hoverable(true)
                    .with_fit_mode(OverlayFitMode::SwitchAnchor)
                    .with_anchor_corner(AnchorCorner::TopLeft)
                    .with_z_index(999)
                    .aligned()
                    .bottom()
                    .left(),
                )
            } else {
                None
            })
            .aligned()
            .left()
            .clipped()
            .into_any()
    }
}

const RPC_MESSAGES: &str = "RPC Messages";
const SERVER_LOGS: &str = "Server Logs";

impl LspLogToolbarItemView {
    pub fn new() -> Self {
        Self {
            menu_open: false,
            log_view: None,
        }
    }

    fn toggle_menu(&mut self, cx: &mut ViewContext<Self>) {
        self.menu_open = !self.menu_open;
        cx.notify();
    }

    fn toggle_logging_for_server(
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
                }
            });
        }
        cx.notify();
    }

    fn show_logs_for_server(&mut self, id: LanguageServerId, cx: &mut ViewContext<Self>) {
        if let Some(log_view) = &self.log_view {
            log_view.update(cx, |view, cx| view.show_logs_for_server(id, cx));
            self.menu_open = false;
            cx.notify();
        }
    }

    fn show_rpc_trace_for_server(&mut self, id: LanguageServerId, cx: &mut ViewContext<Self>) {
        if let Some(log_view) = &self.log_view {
            log_view.update(cx, |view, cx| view.show_rpc_trace_for_server(id, cx));
            self.menu_open = false;
            cx.notify();
        }
    }

    fn render_language_server_menu_header(
        current_server: Option<LogMenuItem>,
        theme: &Arc<Theme>,
        cx: &mut ViewContext<Self>,
    ) -> impl Element<Self> {
        enum ToggleMenu {}
        MouseEventHandler::<ToggleMenu, Self>::new(0, cx, move |state, cx| {
            let label: Cow<str> = current_server
                .and_then(|row| {
                    let worktree = row.worktree.read(cx);
                    Some(
                        format!(
                            "{} ({}) - {}",
                            row.server_name.0,
                            worktree.root_name(),
                            if row.rpc_trace_selected {
                                RPC_MESSAGES
                            } else {
                                SERVER_LOGS
                            },
                        )
                        .into(),
                    )
                })
                .unwrap_or_else(|| "No server selected".into());
            let style = theme.toolbar_dropdown_menu.header.style_for(state, false);
            Label::new(label, style.text.clone())
                .contained()
                .with_style(style.container)
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, view, cx| {
            view.toggle_menu(cx);
        })
    }

    fn render_language_server_menu_item(
        id: LanguageServerId,
        name: LanguageServerName,
        worktree: ModelHandle<Worktree>,
        rpc_trace_enabled: bool,
        logs_selected: bool,
        rpc_trace_selected: bool,
        theme: &Arc<Theme>,
        cx: &mut ViewContext<Self>,
    ) -> impl Element<Self> {
        enum ActivateLog {}
        enum ActivateRpcTrace {}

        Flex::column()
            .with_child({
                let style = &theme.toolbar_dropdown_menu.section_header;
                Label::new(
                    format!("{} ({})", name.0, worktree.read(cx).root_name()),
                    style.text.clone(),
                )
                .contained()
                .with_style(style.container)
                .constrained()
                .with_height(theme.toolbar_dropdown_menu.row_height)
            })
            .with_child(
                MouseEventHandler::<ActivateLog, _>::new(id.0, cx, move |state, _| {
                    let style = theme
                        .toolbar_dropdown_menu
                        .item
                        .style_for(state, logs_selected);
                    Label::new(SERVER_LOGS, style.text.clone())
                        .contained()
                        .with_style(style.container)
                        .constrained()
                        .with_height(theme.toolbar_dropdown_menu.row_height)
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, view, cx| {
                    view.show_logs_for_server(id, cx);
                }),
            )
            .with_child(
                MouseEventHandler::<ActivateRpcTrace, _>::new(id.0, cx, move |state, cx| {
                    let style = theme
                        .toolbar_dropdown_menu
                        .item
                        .style_for(state, rpc_trace_selected);
                    Flex::row()
                        .with_child(
                            Label::new(RPC_MESSAGES, style.text.clone())
                                .constrained()
                                .with_height(theme.toolbar_dropdown_menu.row_height),
                        )
                        .with_child(
                            ui::checkbox_with_label::<Self, _, Self, _>(
                                Empty::new(),
                                &theme.welcome.checkbox,
                                rpc_trace_enabled,
                                id.0,
                                cx,
                                move |this, enabled, cx| {
                                    this.toggle_logging_for_server(id, enabled, cx);
                                },
                            )
                            .flex_float(),
                        )
                        .align_children_center()
                        .contained()
                        .with_style(style.container)
                        .constrained()
                        .with_height(theme.toolbar_dropdown_menu.row_height)
                })
                .with_cursor_style(CursorStyle::PointingHand)
                .on_click(MouseButton::Left, move |_, view, cx| {
                    view.show_rpc_trace_for_server(id, cx);
                }),
            )
    }
}

impl Entity for LogStore {
    type Event = ();
}

impl Entity for LspLogView {
    type Event = editor::Event;
}

impl Entity for LspLogToolbarItemView {
    type Event = ();
}
