use std::{
    collections::{HashSet, VecDeque},
    fmt::Display,
    sync::Arc,
};

use agent_client_protocol::schema as acp;
use collections::HashMap;
use gpui::{
    App, Empty, Entity, EventEmitter, FocusHandle, Focusable, Global, ListAlignment, ListState,
    StyleRefinement, Subscription, Task, TextStyleRefinement, Window, actions, list, prelude::*,
};
use language::LanguageRegistry;
use markdown::{CodeBlockRenderer, CopyButtonVisibility, Markdown, MarkdownElement, MarkdownStyle};
use project::{AgentId, Project};
use settings::Settings;
use theme_settings::ThemeSettings;
use ui::{CopyButton, Tooltip, WithScrollbar, prelude::*};
use util::ResultExt as _;
use workspace::{
    Item, ItemHandle, ToolbarItemEvent, ToolbarItemLocation, ToolbarItemView, Workspace,
};

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum StreamMessageDirection {
    Incoming,
    Outgoing,
    /// Lines captured from the agent's stderr. These are not part of the
    /// JSON-RPC protocol, but agents often emit useful diagnostics there.
    Stderr,
}

#[derive(Clone)]
pub enum StreamMessageContent {
    Request {
        id: acp::RequestId,
        method: Arc<str>,
        params: Option<serde_json::Value>,
    },
    Response {
        id: acp::RequestId,
        result: Result<Option<serde_json::Value>, acp::Error>,
    },
    Notification {
        method: Arc<str>,
        params: Option<serde_json::Value>,
    },
    /// A raw stderr line from the agent process.
    Stderr { line: Arc<str> },
}

#[derive(Clone)]
pub struct StreamMessage {
    pub direction: StreamMessageDirection,
    pub message: StreamMessageContent,
}

impl StreamMessage {
    /// Build a `StreamMessage` from a raw line captured off the transport.
    ///
    /// For `Stderr`, the line is wrapped as-is (no JSON parsing). For
    /// `Incoming`/`Outgoing`, the line is parsed as JSON-RPC; returns `None`
    /// if it doesn't look like a valid JSON-RPC message.
    pub fn from_raw_line(direction: StreamMessageDirection, line: &str) -> Option<Self> {
        if direction == StreamMessageDirection::Stderr {
            return Some(StreamMessage {
                direction,
                message: StreamMessageContent::Stderr {
                    line: Arc::from(line),
                },
            });
        }

        let value: serde_json::Value = serde_json::from_str(line).ok()?;
        let obj = value.as_object()?;

        let parsed_id = obj
            .get("id")
            .map(|raw| serde_json::from_value::<acp::RequestId>(raw.clone()));

        let message = if let Some(method) = obj.get("method").and_then(|m| m.as_str()) {
            match parsed_id {
                Some(Ok(id)) => StreamMessageContent::Request {
                    id,
                    method: method.into(),
                    params: obj.get("params").cloned(),
                },
                Some(Err(err)) => {
                    log::warn!("Skipping JSON-RPC message with unparsable id: {err}");
                    return None;
                }
                None => StreamMessageContent::Notification {
                    method: method.into(),
                    params: obj.get("params").cloned(),
                },
            }
        } else if let Some(parsed_id) = parsed_id {
            let id = match parsed_id {
                Ok(id) => id,
                Err(err) => {
                    log::warn!("Skipping JSON-RPC response with unparsable id: {err}");
                    return None;
                }
            };
            if let Some(error) = obj.get("error") {
                let acp_err =
                    serde_json::from_value::<acp::Error>(error.clone()).unwrap_or_else(|err| {
                        log::warn!("Failed to deserialize ACP error: {err}");
                        acp::Error::internal_error().data(error.to_string())
                    });
                StreamMessageContent::Response {
                    id,
                    result: Err(acp_err),
                }
            } else {
                StreamMessageContent::Response {
                    id,
                    result: Ok(obj.get("result").cloned()),
                }
            }
        } else {
            return None;
        };

        Some(StreamMessage { direction, message })
    }
}

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

/// A raw line captured from the transport (or from stderr), tagged with
/// direction. Deserialization into [`StreamMessage`] happens on the
/// registry's foreground task so the ring buffer can be replayed to late
/// subscribers.
struct RawStreamLine {
    direction: StreamMessageDirection,
    line: Arc<str>,
}

/// Handle to an ACP connection's log tap. Passed back by
/// [`AcpConnectionRegistry::set_active_connection`] so that the connection
/// can publish transport and stderr lines without knowing anything about
/// the logs panel's channel.
///
/// Every line is buffered into the registry's ring, so opening the ACP logs
/// panel after the fact still shows history. The steady-state cost is
/// negligible compared to the JSON-RPC serialization that already happened
/// to produce the line.
#[derive(Clone)]
pub struct AcpLogTap {
    sender: smol::channel::Sender<RawStreamLine>,
}

impl AcpLogTap {
    fn emit(&self, direction: StreamMessageDirection, line: &str) {
        self.sender
            .try_send(RawStreamLine {
                direction,
                line: Arc::from(line),
            })
            .log_err();
    }

    /// Record a line read from the agent's stdout.
    pub fn emit_incoming(&self, line: &str) {
        self.emit(StreamMessageDirection::Incoming, line);
    }

    /// Record a line written to the agent's stdin.
    pub fn emit_outgoing(&self, line: &str) {
        self.emit(StreamMessageDirection::Outgoing, line);
    }

    /// Record a line read from the agent's stderr.
    pub fn emit_stderr(&self, line: &str) {
        self.emit(StreamMessageDirection::Stderr, line);
    }
}

/// Maximum number of messages retained in the registry's backlog.
///
/// Mirrors `MAX_STORED_LOG_ENTRIES` in the LSP log store, so that opening the
/// ACP logs panel after a session has been running for a while still shows
/// meaningful history.
const MAX_BACKLOG_MESSAGES: usize = 2000;

#[derive(Default)]
pub struct AcpConnectionRegistry {
    active_agent_id: Option<AgentId>,
    generation: u64,
    /// Bounded ring buffer of every message observed on the current connection.
    /// When a new connection is set, this is cleared.
    backlog: VecDeque<StreamMessage>,
    subscribers: Vec<smol::channel::Sender<StreamMessage>>,
    _broadcast_task: Option<Task<()>>,
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

    /// Register a new active connection and return an [`AcpLogTap`] that
    /// the connection should hand to its transport + stderr readers.
    ///
    /// The tap begins capturing immediately so that opening the ACP logs
    /// panel after something has already gone wrong still shows the
    /// leading history (up to [`MAX_BACKLOG_MESSAGES`]).
    pub fn set_active_connection(
        &mut self,
        agent_id: AgentId,
        cx: &mut Context<Self>,
    ) -> AcpLogTap {
        let (sender, raw_rx) = smol::channel::unbounded::<RawStreamLine>();
        let tap = AcpLogTap { sender };

        self.active_agent_id = Some(agent_id);
        self.generation += 1;
        self.backlog.clear();
        self.subscribers.clear();

        self._broadcast_task = Some(cx.spawn(async move |this, cx| {
            while let Ok(raw) = raw_rx.recv().await {
                this.update(cx, |this, _cx| {
                    let Some(message) = StreamMessage::from_raw_line(raw.direction, &raw.line)
                    else {
                        return;
                    };

                    if this.backlog.len() == MAX_BACKLOG_MESSAGES {
                        this.backlog.pop_front();
                    }
                    this.backlog.push_back(message.clone());

                    this.subscribers.retain(|sender| !sender.is_closed());
                    for sender in &this.subscribers {
                        sender.try_send(message.clone()).log_err();
                    }
                })
                .log_err();
            }

            // The transport closed — clear state so observers (e.g. the ACP
            // logs tab) can transition back to the disconnected state.
            this.update(cx, |this, cx| {
                this.active_agent_id = None;
                this.subscribers.clear();
                cx.notify();
            })
            .log_err();
        }));

        cx.notify();
        tap
    }

    /// Clear the retained message history for the current connection and force
    /// watchers to resubscribe so their local correlation state is reset too.
    pub fn clear_messages(&mut self, cx: &mut Context<Self>) {
        self.backlog.clear();
        self.generation += 1;
        self.subscribers.clear();
        cx.notify();
    }

    /// Subscribe to messages on the current connection.
    ///
    /// Returns the existing backlog (already-observed messages) together with
    /// a receiver for new messages. The caller is responsible for flushing the
    /// backlog into its local state before draining the receiver, so that no
    /// messages are dropped between the snapshot and live subscription.
    pub fn subscribe(&mut self) -> (Vec<StreamMessage>, smol::channel::Receiver<StreamMessage>) {
        let backlog = self.backlog.iter().cloned().collect();
        let (sender, receiver) = smol::channel::unbounded();
        self.subscribers.push(sender);
        (backlog, receiver)
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
    agent_id: AgentId,
    generation: u64,
    messages: Vec<WatchedConnectionMessage>,
    list_state: ListState,
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
        let (generation, agent_id) = {
            let registry = self.connection_registry.read(cx);
            (registry.generation, registry.active_agent_id.clone())
        };

        let Some(agent_id) = agent_id else {
            self.watched_connection = None;
            self.expanded.clear();
            return;
        };

        if let Some(watched) = self.watched_connection.as_ref() {
            if watched.generation == generation {
                return;
            }
        }

        self.expanded.clear();

        let (backlog, messages_rx) = self
            .connection_registry
            .update(cx, |registry, _cx| registry.subscribe());

        let task = cx.spawn(async move |this, cx| {
            while let Ok(message) = messages_rx.recv().await {
                this.update(cx, |this, cx| {
                    this.push_stream_message(message, cx);
                })
                .log_err();
            }
        });

        self.watched_connection = Some(WatchedConnection {
            agent_id,
            generation,
            messages: vec![],
            list_state: ListState::new(0, ListAlignment::Bottom, px(2048.)),
            incoming_request_methods: HashMap::default(),
            outgoing_request_methods: HashMap::default(),
            _task: task,
        });

        for message in backlog {
            self.push_stream_message(message, cx);
        }
    }

    fn push_stream_message(&mut self, stream_message: StreamMessage, cx: &mut Context<Self>) {
        let Some(connection) = self.watched_connection.as_mut() else {
            return;
        };
        let language_registry = self.project.read(cx).languages().clone();
        let index = connection.messages.len();

        let (request_id, method, message_type, params) = match stream_message.message {
            StreamMessageContent::Request { id, method, params } => {
                let method_map = match stream_message.direction {
                    StreamMessageDirection::Incoming => &mut connection.incoming_request_methods,
                    StreamMessageDirection::Outgoing => &mut connection.outgoing_request_methods,
                    // Stderr lines never carry request/response correlation.
                    StreamMessageDirection::Stderr => return,
                };

                method_map.insert(id.clone(), method.clone());
                (Some(id), method.into(), MessageType::Request, Ok(params))
            }
            StreamMessageContent::Response { id, result } => {
                let method_map = match stream_message.direction {
                    StreamMessageDirection::Incoming => &mut connection.outgoing_request_methods,
                    StreamMessageDirection::Outgoing => &mut connection.incoming_request_methods,
                    StreamMessageDirection::Stderr => return,
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
            StreamMessageContent::Notification { method, params } => {
                (None, method.into(), MessageType::Notification, Ok(params))
            }
            StreamMessageContent::Stderr { line } => {
                // Stderr is rendered as plain text inline with JSON-RPC traffic,
                // using `stderr` as the pseudo-method name so it shows up in the
                // header the same way real methods do.
                (
                    None,
                    "stderr".into(),
                    MessageType::Stderr,
                    Ok(Some(serde_json::Value::String(line.to_string()))),
                )
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
                        StreamMessageDirection::Incoming => "incoming",
                        StreamMessageDirection::Outgoing => "outgoing",
                        StreamMessageDirection::Stderr => "stderr",
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
                    .child(match message.direction {
                        StreamMessageDirection::Incoming => Icon::new(IconName::ArrowDown)
                            .color(Color::Error)
                            .size(IconSize::Small),
                        StreamMessageDirection::Outgoing => Icon::new(IconName::ArrowUp)
                            .color(Color::Success)
                            .size(IconSize::Small),
                        StreamMessageDirection::Stderr => Icon::new(IconName::Warning)
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

struct WatchedConnectionMessage {
    name: SharedString,
    request_id: Option<acp::RequestId>,
    direction: StreamMessageDirection,
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
            self.watched_connection
                .as_ref()
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
        let connection_registry = acp_tools.read(cx).connection_registry.clone();
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
                        connection_registry.update(cx, |registry, cx| {
                            registry.clear_messages(cx);
                        });
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
