use collections::HashMap;
use editor::Editor;
use futures::{channel::mpsc, StreamExt};
use gpui::{
    actions,
    elements::{
        AnchorCorner, ChildView, Empty, Flex, Label, MouseEventHandler, Overlay, OverlayFitMode,
        ParentElement, Stack,
    },
    impl_internal_actions,
    platform::MouseButton,
    AppContext, Element, ElementBox, Entity, ModelHandle, RenderContext, View, ViewContext,
    ViewHandle,
};
use language::{Buffer, LanguageServerId, LanguageServerName};
use project::{Project, WorktreeId};
use settings::Settings;
use std::{borrow::Cow, sync::Arc};
use theme::Theme;
use workspace::{
    item::{Item, ItemHandle},
    ToolbarItemLocation, ToolbarItemView, Workspace,
};

const SEND_LINE: &str = "// Send:\n";
const RECEIVE_LINE: &str = "// Receive:\n";

pub struct LspLogView {
    enabled_logs: HashMap<LanguageServerId, LogState>,
    current_server_id: Option<LanguageServerId>,
    project: ModelHandle<Project>,
    io_tx: mpsc::UnboundedSender<(LanguageServerId, bool, String)>,
}

pub struct LspLogToolbarItemView {
    log_view: Option<ViewHandle<LspLogView>>,
    menu_open: bool,
    project: ModelHandle<Project>,
}

struct LogState {
    buffer: ModelHandle<Buffer>,
    editor: ViewHandle<Editor>,
    last_message_kind: Option<MessageKind>,
    _subscription: lsp::Subscription,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum MessageKind {
    Send,
    Receive,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct ActivateLog {
    server_id: LanguageServerId,
}

#[derive(Clone, Copy, PartialEq, Eq)]
struct ToggleMenu;

impl_internal_actions!(log, [ActivateLog, ToggleMenu]);
actions!(log, [OpenLanguageServerLogs]);

pub fn init(cx: &mut AppContext) {
    cx.add_action(LspLogView::deploy);
    cx.add_action(LspLogToolbarItemView::toggle_menu);
    cx.add_action(LspLogToolbarItemView::activate_log_for_server);
}

impl LspLogView {
    pub fn new(project: ModelHandle<Project>, cx: &mut ViewContext<Self>) -> Self {
        let (io_tx, mut io_rx) = mpsc::unbounded();
        let this = Self {
            enabled_logs: HashMap::default(),
            current_server_id: None,
            io_tx,
            project,
        };
        cx.spawn_weak(|this, mut cx| async move {
            while let Some((language_server_id, is_output, mut message)) = io_rx.next().await {
                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| {
                        message.push('\n');
                        this.on_io(language_server_id, is_output, &message, cx);
                    })
                }
            }
        })
        .detach();
        this
    }

    fn deploy(
        workspace: &mut Workspace,
        _: &OpenLanguageServerLogs,
        cx: &mut ViewContext<Workspace>,
    ) {
        let project = workspace.project().read(cx);
        if project.is_remote() {
            return;
        }

        let log_view = cx.add_view(|cx| Self::new(workspace.project().clone(), cx));
        workspace.add_item(Box::new(log_view), cx);
    }

    fn activate_log(&mut self, action: &ActivateLog, cx: &mut ViewContext<Self>) {
        self.enable_logs_for_language_server(action.server_id, cx);
        self.current_server_id = Some(action.server_id);
        cx.notify();
    }

    fn on_io(
        &mut self,
        language_server_id: LanguageServerId,
        is_received: bool,
        message: &str,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(state) = self.enabled_logs.get_mut(&language_server_id) {
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
        }
    }

    pub fn enable_logs_for_language_server(
        &mut self,
        server_id: LanguageServerId,
        cx: &mut ViewContext<Self>,
    ) {
        if let Some(server) = self.project.read(cx).language_server_for_id(server_id) {
            self.enabled_logs.entry(server_id).or_insert_with(|| {
                let project = self.project.read(cx);
                let io_tx = self.io_tx.clone();
                let language = project.languages().language_for_name("JSON");
                let buffer = cx.add_model(|cx| Buffer::new(0, "", cx));
                cx.spawn({
                    let buffer = buffer.clone();
                    |_, mut cx| async move {
                        let language = language.await.ok();
                        buffer.update(&mut cx, |buffer, cx| {
                            buffer.set_language(language, cx);
                        });
                    }
                })
                .detach();
                let editor = cx.add_view(|cx| {
                    let mut editor =
                        Editor::for_buffer(buffer.clone(), Some(self.project.clone()), cx);
                    editor.set_read_only(true);
                    editor
                });

                LogState {
                    buffer,
                    editor,
                    last_message_kind: None,
                    _subscription: server.on_io(move |is_received, json| {
                        io_tx
                            .unbounded_send((server_id, is_received, json.to_string()))
                            .ok();
                    }),
                }
            });
        }
    }
}

impl View for LspLogView {
    fn ui_name() -> &'static str {
        "LspLogView"
    }

    fn render(&mut self, cx: &mut gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        if let Some(id) = self.current_server_id {
            if let Some(log) = self.enabled_logs.get_mut(&id) {
                return ChildView::new(&log.editor, cx).boxed();
            }
        }
        Empty::new().boxed()
    }
}

impl Item for LspLogView {
    fn tab_content(&self, _: Option<usize>, style: &theme::Tab, _: &AppContext) -> ElementBox {
        Label::new("Logs", style.label.clone()).boxed()
    }
}

impl ToolbarItemView for LspLogToolbarItemView {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        cx: &mut ViewContext<Self>,
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

    fn render(&mut self, cx: &mut RenderContext<'_, Self>) -> ElementBox {
        let theme = cx.global::<Settings>().theme.clone();
        let Some(log_view) = self.log_view.as_ref() else { return Empty::new().boxed() };
        let project = self.project.read(cx);
        let mut language_servers = project.language_servers().collect::<Vec<_>>();
        language_servers.sort_by_key(|a| a.0);

        let current_server_id = log_view.read(cx).current_server_id;
        let current_server = current_server_id.and_then(|current_server_id| {
            if let Ok(ix) = language_servers.binary_search_by_key(&current_server_id, |e| e.0) {
                Some(language_servers[ix].clone())
            } else {
                None
            }
        });

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
                        Flex::column()
                            .with_children(language_servers.into_iter().filter_map(
                                |(id, name, worktree_id)| {
                                    Self::render_language_server_menu_item(
                                        id,
                                        name,
                                        worktree_id,
                                        &self.project,
                                        &theme,
                                        cx,
                                    )
                                },
                            ))
                            .contained()
                            .with_style(theme.contacts_popover.container)
                            .constrained()
                            .with_width(200.)
                            .with_height(400.)
                            .boxed(),
                    )
                    .with_fit_mode(OverlayFitMode::SwitchAnchor)
                    .with_anchor_corner(AnchorCorner::TopRight)
                    .with_z_index(999)
                    .aligned()
                    .bottom()
                    .right()
                    .boxed(),
                )
            } else {
                None
            })
            .boxed()
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

    fn toggle_menu(&mut self, _: &ToggleMenu, cx: &mut ViewContext<Self>) {
        self.menu_open = !self.menu_open;
        cx.notify();
    }

    fn activate_log_for_server(&mut self, action: &ActivateLog, cx: &mut ViewContext<Self>) {
        if let Some(log_view) = &self.log_view {
            log_view.update(cx, |log_view, cx| {
                log_view.activate_log(action, cx);
            });
            self.menu_open = false;
        }
        cx.notify();
    }

    fn render_language_server_menu_header(
        current_server: Option<(LanguageServerId, LanguageServerName, WorktreeId)>,
        project: &ModelHandle<Project>,
        theme: &Arc<Theme>,
        cx: &mut RenderContext<Self>,
    ) -> ElementBox {
        MouseEventHandler::<ToggleMenu>::new(0, cx, move |state, cx| {
            let project = project.read(cx);
            let label: Cow<str> = current_server
                .and_then(|(_, server_name, worktree_id)| {
                    let worktree = project.worktree_for_id(worktree_id, cx)?;
                    let worktree = &worktree.read(cx);
                    Some(format!("{} - ({})", server_name.0, worktree.root_name()).into())
                })
                .unwrap_or_else(|| "No server selected".into());
            Label::new(label, theme.context_menu.item.default.label.clone()).boxed()
        })
        .on_click(MouseButton::Left, move |_, cx| {
            cx.dispatch_action(ToggleMenu);
        })
        .boxed()
    }

    fn render_language_server_menu_item(
        id: LanguageServerId,
        name: LanguageServerName,
        worktree_id: WorktreeId,
        project: &ModelHandle<Project>,
        theme: &Arc<Theme>,
        cx: &mut RenderContext<Self>,
    ) -> Option<ElementBox> {
        let project = project.read(cx);
        let worktree = project.worktree_for_id(worktree_id, cx)?;
        let worktree = &worktree.read(cx);
        if !worktree.is_visible() {
            return None;
        }
        let label = format!("{} - ({})", name.0, worktree.root_name());

        Some(
            MouseEventHandler::<ActivateLog>::new(id.0, cx, move |state, cx| {
                Label::new(label, theme.context_menu.item.default.label.clone()).boxed()
            })
            .on_click(MouseButton::Left, move |_, cx| {
                cx.dispatch_action(ActivateLog { server_id: id })
            })
            .boxed(),
        )
    }
}

impl Entity for LspLogView {
    type Event = ();
}

impl Entity for LspLogToolbarItemView {
    type Event = ();
}
