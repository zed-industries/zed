use std::{
    cell::RefCell,
    collections::HashSet,
    fmt::Display,
    rc::{Rc, Weak},
    sync::Arc,
};

use agent_client_protocol as acp;
use gpui::{
    App, Empty, Entity, EventEmitter, FocusHandle, Focusable, Global, ListAlignment, ListState,
    StyleRefinement, Subscription, Task, TextStyleRefinement, Window, actions, list, prelude::*,
};
use language::LanguageRegistry;
use markdown::{CodeBlockRenderer, Markdown, MarkdownElement, MarkdownStyle};
use project::Project;
use settings::Settings;
use theme::ThemeSettings;
use ui::prelude::*;
use util::ResultExt as _;
use workspace::{Item, Workspace};

actions!(acp, [OpenDebugTools]);

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace.register_action(|workspace, _: &OpenDebugTools, window, cx| {
                let acp_tools =
                    Box::new(cx.new(|cx| AcpTools::new(workspace.project().clone(), cx)));
                workspace.add_item_to_active_pane(acp_tools, None, true, window, cx);
            });
        },
    )
    .detach();
}

struct GlobalAcpConnectionRegistry(Entity<AcpConnectionRegistry>);

impl Global for GlobalAcpConnectionRegistry {}

#[derive(Default)]
pub struct AcpConnectionRegistry {
    active_connection: RefCell<Option<ActiveConnection>>,
}

struct ActiveConnection {
    server_name: &'static str,
    connection: Weak<acp::ClientSideConnection>,
}

impl AcpConnectionRegistry {
    pub fn default_global(cx: &mut App) -> Entity<Self> {
        if cx.has_global::<GlobalAcpConnectionRegistry>() {
            cx.global::<GlobalAcpConnectionRegistry>().0.clone()
        } else {
            let registry = cx.new(|_cx| AcpConnectionRegistry::default());
            cx.set_global(GlobalAcpConnectionRegistry(registry.clone()));
            registry
        }
    }

    pub fn set_active_connection(
        &self,
        server_name: &'static str,
        connection: &Rc<acp::ClientSideConnection>,
        cx: &mut Context<Self>,
    ) {
        self.active_connection.replace(Some(ActiveConnection {
            server_name,
            connection: Rc::downgrade(connection),
        }));
        cx.notify();
    }
}

struct AcpTools {
    project: Entity<Project>,
    focus_handle: FocusHandle,
    expanded: HashSet<usize>,
    watched_connection: Option<WatchedConnection>,
    connection_registry: Entity<AcpConnectionRegistry>,
    _subscription: Subscription,
}

struct WatchedConnection {
    server_name: &'static str,
    messages: Vec<WatchedConnectionMessage>,
    list_state: ListState,
    connection: Weak<acp::ClientSideConnection>,
    _task: Task<()>,
}

impl AcpTools {
    fn new(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        let connection_registry = AcpConnectionRegistry::default_global(cx);

        let subscription = cx.observe(&connection_registry, |this, _, cx| {
            dbg!();
            this.update_connection(cx);
            cx.notify();
        });

        let mut this = Self {
            project,
            focus_handle: cx.focus_handle(),
            expanded: HashSet::default(),
            watched_connection: None,
            connection_registry,
            _subscription: subscription,
        };
        this.update_connection(cx);
        this
    }

    fn update_connection(&mut self, cx: &mut Context<Self>) {
        let active_connection = self.connection_registry.read(cx).active_connection.borrow();
        let Some(active_connection) = active_connection.as_ref() else {
            return;
        };

        if let Some(watched_connection) = self.watched_connection.as_ref() {
            if Weak::ptr_eq(
                &watched_connection.connection,
                &active_connection.connection,
            ) {
                return;
            }
        }

        if let Some(connection) = active_connection.connection.upgrade() {
            let mut receiver = connection.subscribe();
            let language_registry = self.project.read(cx).languages().clone();
            let task = cx.spawn(async move |this, cx| {
                while let Ok(message) = receiver.recv().await {
                    this.update(cx, |this, cx| {
                        if let Some(connection) = &mut this.watched_connection {
                            let index = connection.messages.len();
                            connection.messages.push(
                                WatchedConnectionMessage::from_stream_message(
                                    message,
                                    &language_registry,
                                    cx,
                                ),
                            );
                            connection.list_state.splice(index..index, 1);
                            cx.notify();
                        }
                    })
                    .ok();
                }
            });

            self.watched_connection = Some(WatchedConnection {
                server_name: active_connection.server_name,
                messages: vec![],
                list_state: ListState::new(0, ListAlignment::Bottom, px(2048.)),
                connection: active_connection.connection.clone(),
                _task: task,
            });
        }
    }

    fn render_message(
        &mut self,
        index: usize,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> AnyElement {
        let Some(connection) = self.watched_connection.as_ref() else {
            return Empty.into_any();
        };

        let Some(message) = connection.messages.get(index) else {
            return Empty.into_any();
        };

        let base_size = TextSize::Editor.rems(cx);

        let theme_settings = ThemeSettings::get_global(cx);
        let text_style = window.text_style();

        let colors = cx.theme().colors();

        v_flex()
            .w_full()
            .px_4()
            .py_3()
            .border_color(colors.border)
            .border_b_1()
            .gap_2()
            .items_start()
            .font_buffer(cx)
            .text_size(base_size)
            .id(index)
            .group("message")
            .hover(|this| this.bg(colors.element_background.opacity(0.5)))
            .on_click(cx.listener(move |this, _, _, cx| {
                if this.expanded.contains(&index) {
                    this.expanded.remove(&index);
                } else {
                    this.expanded.insert(index);
                    let Some(connection) = &mut this.watched_connection else {
                        return;
                    };
                    let Some(message) = connection.messages.get_mut(index) else {
                        return;
                    };
                    message.expanded(this.project.read(cx).languages().clone(), cx);
                    connection.list_state.scroll_to_reveal_item(index);
                }
                cx.notify()
            }))
            .child(
                h_flex()
                    .w_full()
                    .gap_2()
                    .items_center()
                    .flex_shrink_0()
                    .child(match message.direction {
                        acp::StreamMessageDirection::Incoming => {
                            ui::Icon::new(ui::IconName::ArrowDown).color(Color::Error)
                        }
                        acp::StreamMessageDirection::Outgoing => {
                            ui::Icon::new(ui::IconName::ArrowUp).color(Color::Success)
                        }
                    })
                    .child(
                        div()
                            .children(message.name.clone())
                            .text_color(colors.text_muted),
                    )
                    .child(div().flex_1())
                    .child(
                        div()
                            .child(ui::Chip::new(message.message_type.to_string()))
                            .visible_on_hover("message"),
                    ),
            )
            // probably shouldn't use markdown for this
            .when_some(
                if self.expanded.contains(&index) {
                    message.expanded_params_md.clone()
                } else {
                    message.collapsed_params_md.clone()
                },
                |this, params| {
                    this.child(
                        div().ml_6().flex_1().child(
                            MarkdownElement::new(
                                params,
                                MarkdownStyle {
                                    base_text_style: text_style,
                                    selection_background_color: colors.element_selection_background,
                                    syntax: cx.theme().syntax().clone(),
                                    code_block_overflow_x_scroll: true,
                                    code_block: StyleRefinement {
                                        text: Some(TextStyleRefinement {
                                            font_family: Some(
                                                theme_settings.buffer_font.family.clone(),
                                            ),
                                            font_size: Some((base_size * 0.8).into()),
                                            ..Default::default()
                                        }),
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                            )
                            .code_block_renderer(
                                CodeBlockRenderer::Default {
                                    copy_button: false,
                                    copy_button_on_hover: false,
                                    border: false,
                                },
                            ),
                        ),
                    )
                },
            )
            .into_any()
    }
}

struct WatchedConnectionMessage {
    name: Option<SharedString>,
    direction: acp::StreamMessageDirection,
    message_type: MessageType,
    params: Result<Option<serde_json::Value>, acp::Error>,
    collapsed_params_md: Option<Entity<Markdown>>,
    expanded_params_md: Option<Entity<Markdown>>,
}

impl WatchedConnectionMessage {
    fn from_stream_message(
        stream_message: acp::StreamMessage,
        language_registry: &Arc<LanguageRegistry>,
        cx: &mut App,
    ) -> Self {
        let (name, message_type, params) = match stream_message.message {
            acp::StreamMessageContent::Request {
                id: _,
                method,
                params,
            } => (Some(method), MessageType::Request, Ok(params)),
            acp::StreamMessageContent::Response { id: _, result } => {
                // todo!
                (Some("response".into()), MessageType::Response, result)
            }
            acp::StreamMessageContent::Notification { method, params } => {
                (Some(method), MessageType::Notification, Ok(params))
            }
        };

        Self {
            name: name.map(|name| name.to_string().into()),
            message_type,
            direction: stream_message.direction,
            collapsed_params_md: match params.as_ref() {
                Ok(params) => params
                    .as_ref()
                    .map(|params| collapsed_params_md(params, language_registry, cx)),
                Err(err) => {
                    if let Ok(err) = &serde_json::to_value(err) {
                        Some(collapsed_params_md(&err, language_registry, cx))
                    } else {
                        None
                    }
                }
            },

            expanded_params_md: None,
            params,
        }
    }

    fn expanded(&mut self, language_registry: Arc<LanguageRegistry>, cx: &mut App) {
        let params_md = match &self.params {
            Ok(Some(params)) => Some(expanded_params_md(params, &language_registry, cx)),
            Err(err) => {
                if let Some(err) = &serde_json::to_value(err).log_err() {
                    Some(expanded_params_md(&err, &language_registry, cx))
                } else {
                    None
                }
            }
            _ => None,
        };
        self.expanded_params_md = params_md;
    }
}

fn collapsed_params_md(
    params: &serde_json::Value,
    language_registry: &Arc<LanguageRegistry>,
    cx: &mut App,
) -> Entity<Markdown> {
    let params_json = serde_json::to_string(params)
        .unwrap_or_default()
        .replace("{", "{ ")
        .replace("}", " }")
        .replace(":", ": ")
        .replace(",", ", ");

    let params_md = format!("```json\n{}\n```", params_json);
    cx.new(|cx| Markdown::new(params_md.into(), Some(language_registry.clone()), None, cx))
}

fn expanded_params_md(
    params: &serde_json::Value,
    language_registry: &Arc<LanguageRegistry>,
    cx: &mut App,
) -> Entity<Markdown> {
    let params_json = serde_json::to_string_pretty(params).unwrap_or_default();
    let params_md = format!("```json\n{}\n```", params_json);
    cx.new(|cx| Markdown::new(params_md.into(), Some(language_registry.clone()), None, cx))
}

enum MessageType {
    Request,
    Response,
    Notification,
}

impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Request => write!(f, "Request"),
            MessageType::Response => write!(f, "Response"),
            MessageType::Notification => write!(f, "Notification"),
        }
    }
}

enum AcpToolsEvent {}

impl EventEmitter<AcpToolsEvent> for AcpTools {}

impl Item for AcpTools {
    type Event = AcpToolsEvent;

    fn tab_content_text(&self, _detail: usize, _cx: &App) -> ui::SharedString {
        format!(
            "ACP: {}",
            self.watched_connection
                .as_ref()
                .map_or("Disconnected", |connection| connection.server_name)
        )
        .into()
    }

    fn tab_icon(&self, _window: &Window, _cx: &App) -> Option<Icon> {
        Some(ui::Icon::new(IconName::Thread))
    }
}

impl Focusable for AcpTools {
    fn focus_handle(&self, _cx: &App) -> FocusHandle {
        self.focus_handle.clone()
    }
}

impl Render for AcpTools {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        v_flex()
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(match self.watched_connection.as_ref() {
                Some(connection) => {
                    if connection.messages.is_empty() {
                        h_flex()
                            .size_full()
                            .justify_center()
                            .items_center()
                            .child("No messages recorded yet")
                            .into_any()
                    } else {
                        list(
                            connection.list_state.clone(),
                            cx.processor(Self::render_message),
                        )
                        .with_sizing_behavior(gpui::ListSizingBehavior::Auto)
                        .flex_grow()
                        .into_any()
                    }
                }
                None => h_flex()
                    .size_full()
                    .justify_center()
                    .items_center()
                    .child("No connection")
                    .into_any(),
            })
    }
}
