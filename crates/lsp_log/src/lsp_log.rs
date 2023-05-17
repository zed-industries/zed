use collections::{hash_map, HashMap};
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
use project::{Project, WorktreeId};
use std::{borrow::Cow, sync::Arc};
use theme::{ui, Theme};
use workspace::{
    item::{Item, ItemHandle},
    ToolbarItemLocation, ToolbarItemView, Workspace,
};

const SEND_LINE: &str = "// Send:\n";
const RECEIVE_LINE: &str = "// Receive:\n";

struct LogStore {
    projects: HashMap<WeakModelHandle<Project>, LogStoreProject>,
    io_tx: mpsc::UnboundedSender<(WeakModelHandle<Project>, LanguageServerId, bool, String)>,
}

struct LogStoreProject {
    servers: HashMap<LanguageServerId, LogStoreLanguageServer>,
    _subscription: gpui::Subscription,
}

struct LogStoreLanguageServer {
    buffer: ModelHandle<Buffer>,
    last_message_kind: Option<MessageKind>,
    _subscription: lsp::Subscription,
}

pub struct LspLogView {
    log_store: ModelHandle<LogStore>,
    current_server_id: Option<LanguageServerId>,
    editor: Option<ViewHandle<Editor>>,
    project: ModelHandle<Project>,
}

pub struct LspLogToolbarItemView {
    log_view: Option<ViewHandle<LspLogView>>,
    menu_open: bool,
    project: ModelHandle<Project>,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum MessageKind {
    Send,
    Receive,
}

actions!(log, [OpenLanguageServerLogs]);

pub fn init(cx: &mut AppContext) {
    let log_set = cx.add_model(|cx| LogStore::new(cx));

    cx.add_action(
        move |workspace: &mut Workspace, _: &OpenLanguageServerLogs, cx: _| {
            let project = workspace.project().read(cx);
            if project.is_local() {
                workspace.add_item(
                    Box::new(cx.add_view(|cx| {
                        LspLogView::new(workspace.project().clone(), log_set.clone(), cx)
                    })),
                    cx,
                );
            }
        },
    );
}

impl LogStore {
    fn new(cx: &mut ModelContext<Self>) -> Self {
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

    pub fn has_enabled_logs_for_language_server(
        &self,
        project: &ModelHandle<Project>,
        server_id: LanguageServerId,
    ) -> bool {
        self.projects
            .get(&project.downgrade())
            .map_or(false, |store| store.servers.contains_key(&server_id))
    }

    pub fn enable_logs_for_language_server(
        &mut self,
        project: &ModelHandle<Project>,
        server_id: LanguageServerId,
        cx: &mut ModelContext<Self>,
    ) -> Option<ModelHandle<Buffer>> {
        let server = project.read(cx).language_server_for_id(server_id)?;
        let weak_project = project.downgrade();
        let project_logs = match self.projects.entry(weak_project) {
            hash_map::Entry::Occupied(entry) => entry.into_mut(),
            hash_map::Entry::Vacant(entry) => entry.insert(LogStoreProject {
                servers: HashMap::default(),
                _subscription: cx.observe_release(&project, move |this, _, _| {
                    this.projects.remove(&weak_project);
                }),
            }),
        };
        let server_log_state = project_logs.servers.entry(server_id).or_insert_with(|| {
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

            let project = project.downgrade();
            LogStoreLanguageServer {
                buffer,
                last_message_kind: None,
                _subscription: server.on_io(move |is_received, json| {
                    io_tx
                        .unbounded_send((project, server_id, is_received, json.to_string()))
                        .ok();
                }),
            }
        });
        Some(server_log_state.buffer.clone())
    }

    pub fn disable_logs_for_language_server(
        &mut self,
        project: &ModelHandle<Project>,
        server_id: LanguageServerId,
        _: &mut ModelContext<Self>,
    ) {
        let project = project.downgrade();
        if let Some(store) = self.projects.get_mut(&project) {
            store.servers.remove(&server_id);
            if store.servers.is_empty() {
                self.projects.remove(&project);
            }
        }
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
            .get_mut(&language_server_id)?;
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
    fn new(
        project: ModelHandle<Project>,
        log_set: ModelHandle<LogStore>,
        _: &mut ViewContext<Self>,
    ) -> Self {
        Self {
            project,
            log_store: log_set,
            editor: None,
            current_server_id: None,
        }
    }

    fn show_logs_for_server(&mut self, server_id: LanguageServerId, cx: &mut ViewContext<Self>) {
        let buffer = self.log_store.update(cx, |log_set, cx| {
            log_set.enable_logs_for_language_server(&self.project, server_id, cx)
        });
        if let Some(buffer) = buffer {
            self.current_server_id = Some(server_id);
            self.editor = Some(cx.add_view(|cx| {
                let mut editor = Editor::for_buffer(buffer, Some(self.project.clone()), cx);
                editor.set_read_only(true);
                editor.move_to_end(&Default::default(), cx);
                editor
            }));
            cx.notify();
        }
    }

    fn toggle_logging_for_server(
        &mut self,
        server_id: LanguageServerId,
        enabled: bool,
        cx: &mut ViewContext<Self>,
    ) {
        self.log_store.update(cx, |log_store, cx| {
            if enabled {
                log_store.enable_logs_for_language_server(&self.project, server_id, cx);
            } else {
                log_store.disable_logs_for_language_server(&self.project, server_id, cx);
            }
        });
    }
}

impl View for LspLogView {
    fn ui_name() -> &'static str {
        "LspLogView"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        if let Some(editor) = &self.editor {
            ChildView::new(&editor, cx).into_any()
        } else {
            Empty::new().into_any()
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
        let project = self.project.read(cx);
        let log_view = log_view.read(cx);
        let log_store = log_view.log_store.read(cx);

        let mut language_servers = project
            .language_servers()
            .map(|(id, name, worktree)| {
                (
                    id,
                    name,
                    worktree,
                    log_store.has_enabled_logs_for_language_server(&self.project, id),
                )
            })
            .collect::<Vec<_>>();
        language_servers.sort_by_key(|a| (a.0, a.2));
        language_servers.dedup_by_key(|a| a.0);

        let current_server_id = log_view.current_server_id;
        let current_server = current_server_id.and_then(|current_server_id| {
            if let Ok(ix) = language_servers.binary_search_by_key(&current_server_id, |e| e.0) {
                Some(language_servers[ix].clone())
            } else {
                None
            }
        });

        enum Menu {}

        Stack::new()
            .with_child(Self::render_language_server_menu_header(
                current_server,
                &self.project,
                &theme,
                cx,
            ))
            .with_children(if self.menu_open {
                Some(
                    Overlay::new(
                        MouseEventHandler::<Menu, _>::new(0, cx, move |_, cx| {
                            Flex::column()
                                .with_children(language_servers.into_iter().filter_map(
                                    |(id, name, worktree_id, logging_enabled)| {
                                        Self::render_language_server_menu_item(
                                            id,
                                            name,
                                            worktree_id,
                                            logging_enabled,
                                            Some(id) == current_server_id,
                                            &self.project,
                                            &theme,
                                            cx,
                                        )
                                    },
                                ))
                                .contained()
                                .with_style(theme.context_menu.container)
                                .constrained()
                                .with_width(400.)
                                .with_height(400.)
                        })
                        .on_down_out(MouseButton::Left, |_, this, cx| {
                            this.menu_open = false;
                            cx.notify()
                        }),
                    )
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

impl LspLogToolbarItemView {
    pub fn new(project: ModelHandle<Project>) -> Self {
        Self {
            menu_open: false,
            log_view: None,
            project,
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
                log_view.toggle_logging_for_server(id, enabled, cx);
                if !enabled && Some(id) == log_view.current_server_id {
                    log_view.current_server_id = None;
                    log_view.editor = None;
                    cx.notify();
                }
            });
        }
        cx.notify();
    }

    fn show_logs_for_server(&mut self, id: LanguageServerId, cx: &mut ViewContext<Self>) {
        if let Some(log_view) = &self.log_view {
            log_view.update(cx, |log_view, cx| {
                log_view.show_logs_for_server(id, cx);
            });
            self.menu_open = false;
        }
        cx.notify();
    }

    fn render_language_server_menu_header(
        current_server: Option<(LanguageServerId, LanguageServerName, WorktreeId, bool)>,
        project: &ModelHandle<Project>,
        theme: &Arc<Theme>,
        cx: &mut ViewContext<Self>,
    ) -> impl Element<Self> {
        enum ToggleMenu {}
        MouseEventHandler::<ToggleMenu, Self>::new(0, cx, move |state, cx| {
            let project = project.read(cx);
            let label: Cow<str> = current_server
                .and_then(|(_, server_name, worktree_id, _)| {
                    let worktree = project.worktree_for_id(worktree_id, cx)?;
                    let worktree = &worktree.read(cx);
                    Some(format!("{} - ({})", server_name.0, worktree.root_name()).into())
                })
                .unwrap_or_else(|| "No server selected".into());
            Label::new(
                label,
                theme
                    .context_menu
                    .item
                    .style_for(state, false)
                    .label
                    .clone(),
            )
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, view, cx| {
            view.toggle_menu(cx);
        })
    }

    fn render_language_server_menu_item(
        id: LanguageServerId,
        name: LanguageServerName,
        worktree_id: WorktreeId,
        logging_enabled: bool,
        is_selected: bool,
        project: &ModelHandle<Project>,
        theme: &Arc<Theme>,
        cx: &mut ViewContext<Self>,
    ) -> Option<impl Element<Self>> {
        enum ActivateLog {}
        let project = project.read(cx);
        let worktree = project.worktree_for_id(worktree_id, cx)?;
        let worktree = &worktree.read(cx);
        if !worktree.is_visible() {
            return None;
        }
        let label = format!("{} - ({})", name.0, worktree.root_name());

        Some(
            MouseEventHandler::<ActivateLog, _>::new(id.0, cx, move |state, cx| {
                let item_style = theme.context_menu.item.style_for(state, is_selected);
                Flex::row()
                    .with_child(ui::checkbox_with_label::<Self, _, Self, _>(
                        Empty::new(),
                        &theme.welcome.checkbox,
                        logging_enabled,
                        id.0,
                        cx,
                        move |this, enabled, cx| {
                            this.toggle_logging_for_server(id, enabled, cx);
                        },
                    ))
                    .with_child(Label::new(label, item_style.label.clone()).aligned().left())
                    .align_children_center()
                    .contained()
                    .with_style(item_style.container)
            })
            .with_cursor_style(CursorStyle::PointingHand)
            .on_click(MouseButton::Left, move |_, view, cx| {
                view.show_logs_for_server(id, cx);
            }),
        )
    }
}

impl Entity for LogStore {
    type Event = ();
}

impl Entity for LspLogView {
    type Event = ();
}

impl Entity for LspLogToolbarItemView {
    type Event = ();
}
