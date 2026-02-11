use std::{
    cell::RefCell,
    collections::HashSet,
    fmt::Display,
    rc::{Rc, Weak},
    sync::Arc,
};

use agent_client_protocol as acp;
use collections::HashMap;
use gpui::{
    App, Empty, Entity, EventEmitter, FocusHandle, Focusable, Global, ListAlignment, ListState,
    StyleRefinement, Subscription, Task, TextStyleRefinement, Window, actions, list, prelude::*,
};
use language::LanguageRegistry;
use markdown::{CodeBlockRenderer, Markdown, MarkdownElement, MarkdownStyle};
use project::Project;
use settings::Settings;
use theme::ThemeSettings;
use ui::{CopyButton, Tooltip, WithScrollbar, prelude::*};
use util::ResultExt as _;
use workspace::{
    Item, ItemHandle, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
};

actions!(dev, [OpenAcpLogs]);

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace.register_action(|workspace, _: &OpenAcpLogs, window, cx| {
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
    server_name: SharedString,
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
        server_name: impl Into<SharedString>,
        connection: &Rc<acp::ClientSideConnection>,
        cx: &mut Context<Self>,
    ) {
        self.active_connection.replace(Some(ActiveConnection {
            server_name: server_name.into(),
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
    server_name: SharedString,
    messages: Vec<WatchedConnectionMessage>,
    list_state: ListState,
    connection: Weak<acp::ClientSideConnection>,
    incoming_request_methods: HashMap<acp::RequestId, Arc<str>>,
    outgoing_request_methods: HashMap<acp::RequestId, Arc<str>>,
    _task: Task<()>,
}

impl AcpTools {
    fn new(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        let connection_registry = AcpConnectionRegistry::default_global(cx);

        let subscription = cx.observe(&connection_registry, |this, _, cx| {
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
            let task = cx.spawn(async move |this, cx| {
                while let Ok(message) = receiver.recv().await {
                    this.update(cx, |this, cx| {
                        this.push_stream_message(message, cx);
                    })
                    .ok();
                }
            });

            self.watched_connection = Some(WatchedConnection {
                server_name: active_connection.server_name.clone(),
                messages: vec![],
                list_state: ListState::new(0, ListAlignment::Bottom, px(2048.)),
                connection: active_connection.connection.clone(),
                incoming_request_methods: HashMap::default(),
                outgoing_request_methods: HashMap::default(),
                _task: task,
            });
        }
    }

    fn push_stream_message(&mut self, stream_message: acp::StreamMessage, cx: &mut Context<Self>) {
        let Some(connection) = self.watched_connection.as_mut() else {
            return;
        };
        let language_registry = self.project.read(cx).languages().clone();
        let index = connection.messages.len();

        let (request_id, method, message_type, params) = match stream_message.message {
            acp::StreamMessageContent::Request { id, method, params } => {
                let method_map = match stream_message.direction {
                    acp::StreamMessageDirection::Incoming => {
                        &mut connection.incoming_request_methods
                    }
                    acp::StreamMessageDirection::Outgoing => {
                        &mut connection.outgoing_request_methods
                    }
                };

                method_map.insert(id.clone(), method.clone());
                (Some(id), method.into(), MessageType::Request, Ok(params))
            }
            acp::StreamMessageContent::Response { id, result } => {
                let method_map = match stream_message.direction {
                    acp::StreamMessageDirection::Incoming => {
                        &mut connection.outgoing_request_methods
                    }
                    acp::StreamMessageDirection::Outgoing => {
                        &mut connection.incoming_request_methods
                    }
                };

                if let Some(method) = method_map.remove(&id) {
                    (Some(id), method.into(), MessageType::Response, result)
                } else {
                    (
                        Some(id),
                        "[unrecognized response]".into(),
                        MessageType::Response,
                        result,
                    )
                }
            }
            acp::StreamMessageContent::Notification { method, params } => {
                (None, method.into(), MessageType::Notification, Ok(params))
            }
        };

        let message = WatchedConnectionMessage {
            name: method,
            message_type,
            request_id,
            direction: stream_message.direction,
            collapsed_params_md: match params.as_ref() {
                Ok(params) => params
                    .as_ref()
                    .map(|params| collapsed_params_md(params, &language_registry, cx)),
                Err(err) => {
                    if let Ok(err) = &serde_json::to_value(err) {
                        Some(collapsed_params_md(&err, &language_registry, cx))
                    } else {
                        None
                    }
                }
            },

            expanded_params_md: None,
            params,
        };

        connection.messages.push(message);
        connection.list_state.splice(index..index, 1);
        cx.notify();
    }

    fn serialize_observed_messages(&self) -> Option<String> {
        let connection = self.watched_connection.as_ref()?;

        let messages: Vec<serde_json::Value> = connection
            .messages
            .iter()
            .filter_map(|message| {
                let params = match &message.params {
                    Ok(Some(params)) => params.clone(),
                    Ok(None) => serde_json::Value::Null,
                    Err(err) => serde_json::to_value(err).ok()?,
                };
                Some(serde_json::json!({
                    "_direction": match message.direction {
                        acp::StreamMessageDirection::Incoming => "incoming",
                        acp::StreamMessageDirection::Outgoing => "outgoing",
                    },
                    "_type": message.message_type.to_string().to_lowercase(),
                    "id": message.request_id,
                    "method": message.name.to_string(),
                    "params": params,
                }))
            })
            .collect();

        serde_json::to_string_pretty(&messages).ok()
    }

    fn clear_messages(&mut self, cx: &mut Context<Self>) {
        if let Some(connection) = self.watched_connection.as_mut() {
            connection.messages.clear();
            connection.list_state.reset(0);
            self.expanded.clear();
            cx.notify();
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
        let expanded = self.expanded.contains(&index);

        v_flex()
            .id(index)
            .group("message")
            .cursor_pointer()
            .font_buffer(cx)
            .w_full()
            .py_3()
            .pl_4()
            .pr_5()
            .gap_2()
            .items_start()
            .text_size(base_size)
            .border_color(colors.border)
            .border_b_1()
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
                    .flex_shrink_0()
                    .child(match message.direction {
                        acp::StreamMessageDirection::Incoming => Icon::new(IconName::ArrowDown)
                            .color(Color::Error)
                            .size(IconSize::Small),
                        acp::StreamMessageDirection::Outgoing => Icon::new(IconName::ArrowUp)
                            .color(Color::Success)
                            .size(IconSize::Small),
                    })
                    .child(
                        Label::new(message.name.clone())
                            .buffer_font(cx)
                            .color(Color::Muted),
                    )
                    .child(div().flex_1())
                    .child(
                        div()
                            .child(ui::Chip::new(message.message_type.to_string()))
                            .visible_on_hover("message"),
                    )
                    .children(
                        message
                            .request_id
                            .as_ref()
                            .map(|req_id| div().child(ui::Chip::new(req_id.to_string()))),
                    ),
            )
            // I'm aware using markdown is a hack. Trying to get something working for the demo.
            // Will clean up soon!
            .when_some(
                if expanded {
                    message.expanded_params_md.clone()
                } else {
                    message.collapsed_params_md.clone()
                },
                |this, params| {
                    this.child(
                        div().pl_6().w_full().child(
                            MarkdownElement::new(
                                params,
                                MarkdownStyle {
                                    base_text_style: text_style,
                                    selection_background_color: colors.element_selection_background,
                                    syntax: cx.theme().syntax().clone(),
                                    code_block_overflow_x_scroll: true,
                                    code_block: StyleRefinement {
                                        text: TextStyleRefinement {
                                            font_family: Some(
                                                theme_settings.buffer_font.family.clone(),
                                            ),
                                            font_size: Some((base_size * 0.8).into()),
                                            ..Default::default()
                                        },
                                        ..Default::default()
                                    },
                                    ..Default::default()
                                },
                            )
                            .code_block_renderer(
                                CodeBlockRenderer::Default {
                                    copy_button: false,
                                    copy_button_on_hover: expanded,
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
    name: SharedString,
    request_id: Option<acp::RequestId>,
    direction: acp::StreamMessageDirection,
    message_type: MessageType,
    params: Result<Option<serde_json::Value>, acp::Error>,
    collapsed_params_md: Option<Entity<Markdown>>,
    expanded_params_md: Option<Entity<Markdown>>,
}

impl WatchedConnectionMessage {
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
    let params_json = serde_json::to_string(params).unwrap_or_default();
    let mut spaced_out_json = String::with_capacity(params_json.len() + params_json.len() / 4);

    for ch in params_json.chars() {
        match ch {
            '{' => spaced_out_json.push_str("{ "),
            '}' => spaced_out_json.push_str(" }"),
            ':' => spaced_out_json.push_str(": "),
            ',' => spaced_out_json.push_str(", "),
            c => spaced_out_json.push(c),
        }
    }

    let params_md = format!("```json\n{}\n```", spaced_out_json);
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
                .map_or("Disconnected", |connection| &connection.server_name)
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
    fn render(&mut self, window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
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
                        div()
                            .size_full()
                            .flex_grow()
                            .child(
                                list(
                                    connection.list_state.clone(),
                                    cx.processor(Self::render_message),
                                )
                                .with_sizing_behavior(gpui::ListSizingBehavior::Auto)
                                .size_full(),
                            )
                            .vertical_scrollbar_for(&connection.list_state, window, cx)
                            .into_any()
                    }
                }
                None => h_flex()
                    .size_full()
                    .justify_center()
                    .items_center()
                    .child("No active connection")
                    .into_any(),
            })
    }
}

pub struct AcpToolsToolbarItemView {
    acp_tools: Option<Entity<AcpTools>>,
}

impl AcpToolsToolbarItemView {
    pub fn new() -> Self {
        Self { acp_tools: None }
    }
}

impl Render for AcpToolsToolbarItemView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let Some(acp_tools) = self.acp_tools.as_ref() else {
            return Empty.into_any_element();
        };

        let acp_tools = acp_tools.clone();
        let has_messages = acp_tools
            .read(cx)
            .watched_connection
            .as_ref()
            .is_some_and(|connection| !connection.messages.is_empty());

        h_flex()
            .gap_2()
            .child({
                let message = acp_tools
                    .read(cx)
                    .serialize_observed_messages()
                    .unwrap_or_default();

                CopyButton::new("copy-all-messages", message)
                    .tooltip_label("Copy All Messages")
                    .disabled(!has_messages)
            })
            .child(
                IconButton::new("clear_messages", IconName::Trash)
                    .icon_size(IconSize::Small)
                    .tooltip(Tooltip::text("Clear Messages"))
                    .disabled(!has_messages)
                    .on_click(cx.listener(move |_this, _, _window, cx| {
                        acp_tools.update(cx, |acp_tools, cx| {
                            acp_tools.clear_messages(cx);
                        });
                    })),
            )
            .into_any()
    }
}

impl EventEmitter<ToolbarItemEvent> for AcpToolsToolbarItemView {}

impl ToolbarItemView for AcpToolsToolbarItemView {
    fn set_active_pane_item(
        &mut self,
        active_pane_item: Option<&dyn ItemHandle>,
        _window: &mut Window,
        cx: &mut Context<Self>,
    ) -> ToolbarItemLocation {
        if let Some(item) = active_pane_item
            && let Some(acp_tools) = item.downcast::<AcpTools>()
        {
            self.acp_tools = Some(acp_tools);
            cx.notify();
            return ToolbarItemLocation::PrimaryRight;
        }
        if self.acp_tools.take().is_some() {
            cx.notify();
        }
        ToolbarItemLocation::Hidden
    }
}
