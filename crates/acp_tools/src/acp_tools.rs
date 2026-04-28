use std::{collections::HashSet, fmt::Display, rc::Rc, sync::Arc};

use agent_client_protocol::schema as acp;
use agent_servers::{AcpDebugMessage, AcpDebugMessageContent, AcpDebugMessageDirection};
use agent_ui::agent_connection_store::AgentConnectionStatus;
use agent_ui::{Agent, AgentConnectionStore, AgentPanel};
use collections::HashMap;
use gpui::{
    App, Empty, Entity, EventEmitter, FocusHandle, Focusable, ListAlignment, ListState,
    SharedString, StyleRefinement, Subscription, Task, TextStyleRefinement, WeakEntity, Window,
    actions, list, prelude::*,
};
use language::LanguageRegistry;
use markdown::{CodeBlockRenderer, CopyButtonVisibility, Markdown, MarkdownElement, MarkdownStyle};
use project::{AgentId, Project};
use settings::Settings;
use theme_settings::ThemeSettings;
use ui::{
    ContextMenu, CopyButton, DropdownMenu, DropdownStyle, IconPosition, Tooltip, WithScrollbar,
    prelude::*,
};
use util::ResultExt as _;
use workspace::{
    Item, ItemHandle, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
};

actions!(dev, [OpenAcpLogs]);

pub fn init(cx: &mut App) {
    cx.observe_new(
        |workspace: &mut Workspace, _window, _cx: &mut Context<Workspace>| {
            workspace.register_action(|workspace, _: &OpenAcpLogs, window, cx| {
                let connection_store = workspace
                    .panel::<AgentPanel>(cx)
                    .map(|panel| panel.read(cx).connection_store().clone());
                let acp_tools = Box::new(cx.new(|cx| {
                    AcpTools::new(
                        workspace.weak_handle(),
                        workspace.project().clone(),
                        connection_store,
                        cx,
                    )
                }));
                workspace.add_item_to_active_pane(acp_tools, None, true, window, cx);
            });
        },
    )
    .detach();
}

struct AcpTools {
    workspace: WeakEntity<Workspace>,
    project: Entity<Project>,
    focus_handle: FocusHandle,
    expanded: HashSet<usize>,
    watched_connections: HashMap<AgentId, WatchedConnection>,
    selected_connection: Option<AgentId>,
    connection_store: Option<Entity<AgentConnectionStore>>,
    _workspace_subscription: Option<Subscription>,
    _connection_store_subscription: Option<Subscription>,
}

struct WatchedConnection {
    agent_id: AgentId,
    connection: Rc<agent_servers::AcpConnection>,
    messages: Vec<WatchedConnectionMessage>,
    list_state: ListState,
    incoming_request_methods: HashMap<acp::RequestId, Arc<str>>,
    outgoing_request_methods: HashMap<acp::RequestId, Arc<str>>,
    _task: Task<()>,
}

impl AcpTools {
    fn new(
        workspace: WeakEntity<Workspace>,
        project: Entity<Project>,
        connection_store: Option<Entity<AgentConnectionStore>>,
        cx: &mut Context<Self>,
    ) -> Self {
        let workspace_subscription = workspace.upgrade().map(|workspace| {
            cx.observe(&workspace, |this, _, cx| {
                this.update_connection_store(cx);
            })
        });

        let mut acp_tools = Self {
            workspace,
            project,
            focus_handle: cx.focus_handle(),
            expanded: HashSet::default(),
            watched_connections: HashMap::default(),
            selected_connection: None,
            connection_store: None,
            _workspace_subscription: workspace_subscription,
            _connection_store_subscription: None,
        };
        acp_tools.set_connection_store(connection_store, cx);
        acp_tools
    }

    fn set_connection_store(
        &mut self,
        connection_store: Option<Entity<AgentConnectionStore>>,
        cx: &mut Context<Self>,
    ) {
        if self.connection_store == connection_store {
            return;
        }

        self.connection_store = connection_store.clone();
        self._connection_store_subscription = connection_store.as_ref().map(|connection_store| {
            cx.observe(connection_store, |this, _, cx| {
                this.refresh_connections(cx);
            })
        });
        self.refresh_connections(cx);
    }

    fn update_connection_store(&mut self, cx: &mut Context<Self>) {
        let connection_store = self.workspace.upgrade().and_then(|workspace| {
            workspace
                .read(cx)
                .panel::<AgentPanel>(cx)
                .map(|panel| panel.read(cx).connection_store().clone())
        });
        self.set_connection_store(connection_store, cx);
    }

    fn refresh_connections(&mut self, cx: &mut Context<Self>) {
        let active_connections = self
            .connection_store
            .as_ref()
            .map(|connection_store| connection_store.read(cx).active_acp_connections(cx))
            .unwrap_or_default();

        self.watched_connections
            .retain(|agent_id, watched_connection| {
                active_connections.iter().any(|active_connection| {
                    active_connection.agent_id == *agent_id
                        && Rc::ptr_eq(
                            &active_connection.connection,
                            &watched_connection.connection,
                        )
                })
            });

        for active_connection in active_connections {
            if self
                .watched_connections
                .get(&active_connection.agent_id)
                .is_some_and(|watched_connection| {
                    Rc::ptr_eq(
                        &active_connection.connection,
                        &watched_connection.connection,
                    )
                })
            {
                continue;
            }

            let (backlog, messages_rx) = active_connection.connection.subscribe_debug_messages();
            let agent_id = active_connection.agent_id.clone();
            let task = cx.spawn({
                let agent_id = agent_id.clone();
                async move |this, cx| {
                    while let Ok(message) = messages_rx.recv().await {
                        this.update(cx, |this, cx| {
                            this.push_stream_message(&agent_id, message, cx);
                        })
                        .log_err();
                    }
                }
            });

            let mut watched_connection = WatchedConnection {
                agent_id: agent_id.clone(),
                messages: Vec::new(),
                list_state: ListState::new(0, ListAlignment::Bottom, px(2048.)),
                connection: active_connection.connection.clone(),
                incoming_request_methods: HashMap::default(),
                outgoing_request_methods: HashMap::default(),
                _task: task,
            };

            for message in backlog {
                push_stream_message_for_connection(
                    &mut watched_connection,
                    &self.project,
                    message,
                    cx,
                );
            }

            self.watched_connections
                .insert(agent_id, watched_connection);
        }

        self.selected_connection = self
            .selected_connection
            .clone()
            .filter(|agent_id| self.should_keep_selected_connection(agent_id, cx))
            .or_else(|| self.watched_connections.keys().next().cloned());
        self.expanded.clear();
        cx.notify();
    }

    fn should_keep_selected_connection(&self, agent_id: &AgentId, cx: &App) -> bool {
        self.watched_connections.contains_key(agent_id)
            || self
                .connection_store
                .as_ref()
                .is_some_and(|connection_store| {
                    connection_store
                        .read(cx)
                        .connection_status(&Agent::from(agent_id.clone()), cx)
                        != AgentConnectionStatus::Disconnected
                })
    }

    fn select_connection(&mut self, agent_id: Option<AgentId>, cx: &mut Context<Self>) {
        if self.selected_connection == agent_id {
            return;
        }

        self.selected_connection = agent_id;
        self.expanded.clear();
        cx.notify();
    }

    fn restart_selected_connection(&mut self, cx: &mut Context<Self>) {
        let Some(agent_id) = self.selected_connection.clone() else {
            return;
        };
        let Some(workspace) = self.workspace.upgrade() else {
            return;
        };

        workspace.update(cx, |workspace, cx| {
            let Some(panel) = workspace.panel::<AgentPanel>(cx) else {
                return;
            };

            let fs = workspace.app_state().fs.clone();
            let (thread_store, connection_store) = {
                let panel = panel.read(cx);
                (
                    panel.thread_store().clone(),
                    panel.connection_store().clone(),
                )
            };
            let agent = Agent::from(agent_id);
            let server = agent.server(fs, thread_store);
            connection_store.update(cx, |store, cx| {
                store.restart_connection(agent, server, cx);
            });
        });
    }

    fn selected_connection_status(&self, cx: &App) -> Option<AgentConnectionStatus> {
        let agent = Agent::from(self.selected_connection.clone()?);
        Some(
            self.connection_store
                .as_ref()?
                .read(cx)
                .connection_status(&agent, cx),
        )
    }

    fn selected_watched_connection(&self) -> Option<&WatchedConnection> {
        let selected_connection = self.selected_connection.as_ref()?;
        self.watched_connections.get(selected_connection)
    }

    fn selected_watched_connection_mut(&mut self) -> Option<&mut WatchedConnection> {
        let selected_connection = self.selected_connection.clone()?;
        self.watched_connections.get_mut(&selected_connection)
    }

    fn connection_menu_entries(&self) -> Vec<SharedString> {
        let mut entries: Vec<_> = self
            .watched_connections
            .values()
            .map(|connection| connection.agent_id.0.clone())
            .collect();
        entries.sort();
        entries
    }

    fn selected_connection_label(&self) -> SharedString {
        self.selected_connection
            .as_ref()
            .map(|agent_id| agent_id.0.clone())
            .unwrap_or_else(|| SharedString::from("No connection selected"))
    }

    fn connection_menu(&self, window: &mut Window, cx: &mut Context<Self>) -> Entity<ContextMenu> {
        let entries = self.connection_menu_entries();
        let selected_connection = self.selected_connection.clone();
        let acp_tools = cx.entity().downgrade();

        ContextMenu::build(window, cx, move |mut menu, _window, _cx| {
            if entries.is_empty() {
                return menu.entry("No active connections", None, |_, _| {});
            }

            for entry in &entries {
                let label = entry.clone();
                let is_selected = selected_connection
                    .as_ref()
                    .is_some_and(|agent_id| agent_id.0.as_ref() == label.as_ref());
                let acp_tools = acp_tools.clone();
                menu = menu.toggleable_entry(
                    label.clone(),
                    is_selected,
                    IconPosition::Start,
                    None,
                    move |_window, cx| {
                        acp_tools
                            .update(cx, |this, cx| {
                                this.select_connection(Some(AgentId(label.clone())), cx);
                            })
                            .ok();
                    },
                );
            }

            menu
        })
    }

    fn push_stream_message(
        &mut self,
        agent_id: &AgentId,
        stream_message: AcpDebugMessage,
        cx: &mut Context<Self>,
    ) {
        let Some(connection) = self.watched_connections.get_mut(agent_id) else {
            return;
        };
        push_stream_message_for_connection(connection, &self.project, stream_message, cx);
        cx.notify();
    }

    fn serialize_observed_messages(&self) -> Option<String> {
        let connection = self.selected_watched_connection()?;

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
                        AcpDebugMessageDirection::Incoming => "incoming",
                        AcpDebugMessageDirection::Outgoing => "outgoing",
                        AcpDebugMessageDirection::Stderr => "stderr",
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
        if let Some(connection) = self.selected_watched_connection_mut() {
            connection.messages.clear();
            connection.list_state.reset(0);
            connection.incoming_request_methods.clear();
            connection.outgoing_request_methods.clear();
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
        let Some(connection) = self.selected_watched_connection() else {
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
            .child(
                h_flex()
                    .id(("acp-log-message-header", index))
                    .w_full()
                    .gap_2()
                    .flex_shrink_0()
                    .cursor_pointer()
                    .on_click(cx.listener(move |this, _, _, cx| {
                        if this.expanded.contains(&index) {
                            this.expanded.remove(&index);
                        } else {
                            this.expanded.insert(index);
                            let project = this.project.clone();
                            let Some(connection) = this.selected_watched_connection_mut() else {
                                return;
                            };
                            let Some(message) = connection.messages.get_mut(index) else {
                                return;
                            };
                            message.expanded(project.read(cx).languages().clone(), cx);
                            connection.list_state.scroll_to_reveal_item(index);
                        }
                        cx.notify()
                    }))
                    .child(match message.direction {
                        AcpDebugMessageDirection::Incoming => Icon::new(IconName::ArrowDown)
                            .color(Color::Error)
                            .size(IconSize::Small),
                        AcpDebugMessageDirection::Outgoing => Icon::new(IconName::ArrowUp)
                            .color(Color::Success)
                            .size(IconSize::Small),
                        AcpDebugMessageDirection::Stderr => Icon::new(IconName::Warning)
                            .color(Color::Warning)
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
                                    copy_button_visibility: if expanded {
                                        CopyButtonVisibility::VisibleOnHover
                                    } else {
                                        CopyButtonVisibility::Hidden
                                    },
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

fn push_stream_message_for_connection(
    connection: &mut WatchedConnection,
    project: &Entity<Project>,
    stream_message: AcpDebugMessage,
    cx: &mut App,
) {
    let language_registry = project.read(cx).languages().clone();
    let index = connection.messages.len();

    let (request_id, method, message_type, params) = match stream_message.message {
        AcpDebugMessageContent::Request { id, method, params } => {
            let method_map = match stream_message.direction {
                AcpDebugMessageDirection::Incoming => &mut connection.incoming_request_methods,
                AcpDebugMessageDirection::Outgoing => &mut connection.outgoing_request_methods,
                AcpDebugMessageDirection::Stderr => return,
            };

            method_map.insert(id.clone(), method.clone());
            (Some(id), method.into(), MessageType::Request, Ok(params))
        }
        AcpDebugMessageContent::Response { id, result } => {
            let method_map = match stream_message.direction {
                AcpDebugMessageDirection::Incoming => &mut connection.outgoing_request_methods,
                AcpDebugMessageDirection::Outgoing => &mut connection.incoming_request_methods,
                AcpDebugMessageDirection::Stderr => return,
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
        AcpDebugMessageContent::Notification { method, params } => {
            (None, method.into(), MessageType::Notification, Ok(params))
        }
        AcpDebugMessageContent::Stderr { line } => (
            None,
            "stderr".into(),
            MessageType::Stderr,
            Ok(Some(serde_json::Value::String(line.to_string()))),
        ),
    };

    let message = WatchedConnectionMessage {
        name: method,
        message_type,
        request_id,
        direction: stream_message.direction,
        collapsed_params_md: match &params {
            Ok(Some(params)) => Some(collapsed_params_md(params, &language_registry, cx)),
            Ok(None) => None,
            Err(err) => serde_json::to_value(err)
                .ok()
                .map(|err| collapsed_params_md(&err, &language_registry, cx)),
        },
        expanded_params_md: None,
        params,
    };

    connection.messages.push(message);
    connection.list_state.splice(index..index, 1);
}

struct WatchedConnectionMessage {
    name: SharedString,
    request_id: Option<acp::RequestId>,
    direction: AcpDebugMessageDirection,
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
    Stderr,
}

impl Display for MessageType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            MessageType::Request => write!(f, "Request"),
            MessageType::Response => write!(f, "Response"),
            MessageType::Notification => write!(f, "Notification"),
            MessageType::Stderr => write!(f, "Stderr"),
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
            self.selected_watched_connection()
                .map_or("Disconnected", |connection| connection.agent_id.0.as_ref())
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
        let has_messages = self
            .selected_watched_connection()
            .is_some_and(|connection| !connection.messages.is_empty());
        let can_restart = matches!(
            self.selected_connection_status(cx),
            Some(status) if status != AgentConnectionStatus::Connecting
        );
        let copied_messages = self.serialize_observed_messages().unwrap_or_default();

        v_flex()
            .track_focus(&self.focus_handle)
            .size_full()
            .bg(cx.theme().colors().editor_background)
            .child(
                h_flex()
                    .w_full()
                    .px_3()
                    .py_2()
                    .items_center()
                    .justify_between()
                    .gap_2()
                    .border_b_1()
                    .border_color(cx.theme().colors().border)
                    .child(
                        DropdownMenu::new(
                            "acp-connection-selector",
                            self.selected_connection_label(),
                            self.connection_menu(window, cx),
                        )
                        .style(DropdownStyle::Subtle)
                        .disabled(self.watched_connections.is_empty()),
                    )
                    .child(
                        h_flex()
                            .gap_2()
                            .child(
                                IconButton::new("restart_connection", IconName::RotateCw)
                                    .icon_size(IconSize::Small)
                                    .tooltip(Tooltip::text("Restart Connection"))
                                    .disabled(!can_restart)
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.restart_selected_connection(cx);
                                    })),
                            )
                            .child(
                                CopyButton::new("copy-all-messages", copied_messages)
                                    .tooltip_label("Copy All Messages")
                                    .disabled(!has_messages),
                            )
                            .child(
                                IconButton::new("clear_messages", IconName::Trash)
                                    .icon_size(IconSize::Small)
                                    .tooltip(Tooltip::text("Clear Messages"))
                                    .disabled(!has_messages)
                                    .on_click(cx.listener(|this, _, _window, cx| {
                                        this.clear_messages(cx);
                                    })),
                            ),
                    ),
            )
            .child(match self.selected_watched_connection() {
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
                None => match self.selected_connection_status(cx) {
                    Some(AgentConnectionStatus::Connecting) => h_flex()
                        .size_full()
                        .justify_center()
                        .items_center()
                        .child(format!(
                            "Reconnecting to {}",
                            self.selected_connection_label()
                        ))
                        .into_any(),
                    _ => h_flex()
                        .size_full()
                        .justify_center()
                        .items_center()
                        .child("No active connection")
                        .into_any(),
                },
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
        let _ = (&self.acp_tools, cx);
        Empty.into_any_element()
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
            return ToolbarItemLocation::Hidden;
        }
        if self.acp_tools.take().is_some() {
            cx.notify();
        }
        ToolbarItemLocation::Hidden
    }
}
