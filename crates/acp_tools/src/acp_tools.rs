use std::{
    cell::{Cell, RefCell},
    collections::{HashSet, VecDeque},
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
use markdown::{CodeBlockRenderer, CopyButtonVisibility, Markdown, MarkdownElement, MarkdownStyle};
use project::{AgentId, Project};
use settings::Settings;
use theme_settings::ThemeSettings;
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

/// Maximum number of messages retained in a connection's [`MessageLog`] ring buffer.
///
/// Once this cap is hit, the oldest message is dropped as new ones arrive. The
/// monotonic counter returned by [`MessageLog::snapshot`] still reflects the
/// true total, so readers can detect when the ring has rolled past unseen
/// messages.
const MESSAGE_LOG_CAPACITY: usize = 500;

/// A bounded ring buffer of ACP stream messages for a single connection.
///
/// The buffer is populated by a background task owned by the connection itself,
/// so entries are captured from the moment the connection is created —
/// independently of whether any UI is observing. Readers open the log on
/// demand, take a snapshot, then subscribe to the [`watch::Receiver`] returned
/// by [`Self::subscribe`] to be notified of new entries.
pub struct MessageLog {
    messages: RefCell<VecDeque<acp::StreamMessage>>,
    total_count: Cell<u64>,
    notifier: RefCell<watch::Sender<u64>>,
}

/// A point-in-time view of a [`MessageLog`].
///
/// `total_count` is the number of messages the log has ever received, including
/// any that have rolled out of the ring.
///
/// `skipped` has two subtly different meanings depending on how the snapshot
/// was produced:
///
/// - From [`MessageLog::snapshot`]: the number of messages that have rolled
///   out of the ring over the log's entire lifetime (i.e. `total_count -
///   messages.len()`).
/// - From [`MessageLog::read_since`]: the number of messages the caller missed
///   because the ring rolled past their `last_seen_total`. If `last_seen_total`
///   still lies within the ring, this is `0` even if older messages have
///   previously been evicted.
pub struct MessageLogSnapshot {
    pub messages: Vec<acp::StreamMessage>,
    pub total_count: u64,
    pub skipped: u64,
}

impl MessageLog {
    pub fn new() -> Rc<Self> {
        // `watch::channel` takes an initial value; we never observe it because
        // readers always call `subscribe()` (which syncs to the current
        // version) and then treat the received value as a wakeup signal.
        let (tx, _) = watch::channel(0u64);
        Rc::new(Self {
            messages: RefCell::new(VecDeque::with_capacity(MESSAGE_LOG_CAPACITY)),
            total_count: Cell::new(0),
            notifier: RefCell::new(tx),
        })
    }

    pub fn push(&self, message: acp::StreamMessage) {
        {
            let mut messages = self.messages.borrow_mut();
            if messages.len() == MESSAGE_LOG_CAPACITY {
                messages.pop_front();
            }
            messages.push_back(message);
        }
        let new_total = self.total_count.get().saturating_add(1);
        self.total_count.set(new_total);
        // A dropped receiver is not an error — it just means nobody is watching
        // right now. The message is still in the ring for the next reader.
        self.notifier.borrow_mut().send(new_total).ok();
    }

    pub fn snapshot(&self) -> MessageLogSnapshot {
        let messages = self.messages.borrow();
        let total_count = self.total_count.get();
        let skipped = total_count.saturating_sub(messages.len() as u64);
        MessageLogSnapshot {
            messages: messages.iter().cloned().collect(),
            total_count,
            skipped,
        }
    }

    /// Return every message with a monotonic index `>= last_seen_total`.
    ///
    /// If the ring has rolled past `last_seen_total` since the caller's last
    /// read, the returned snapshot's `skipped` field will be non-zero to
    /// indicate how many messages were lost.
    pub fn read_since(&self, last_seen_total: u64) -> MessageLogSnapshot {
        let messages = self.messages.borrow();
        let total_count = self.total_count.get();
        let oldest_index = total_count.saturating_sub(messages.len() as u64);

        let (start_offset, skipped) = if last_seen_total >= oldest_index {
            ((last_seen_total - oldest_index) as usize, 0)
        } else {
            (0, oldest_index - last_seen_total)
        };

        MessageLogSnapshot {
            messages: messages.iter().skip(start_offset).cloned().collect(),
            total_count,
            skipped,
        }
    }

    /// Subscribe for notifications that new messages have been pushed.
    ///
    /// The returned receiver is synced to the current version, so the first
    /// `recv().await` will block until the next `push`. The `u64` carried by
    /// the channel is the current `total_count` but should be treated as an
    /// opaque wakeup signal — callers must still use [`Self::read_since`] to
    /// atomically get the new messages alongside the count.
    pub fn subscribe(&self) -> watch::Receiver<u64> {
        self.notifier.borrow().receiver()
    }
}

struct GlobalAcpConnectionRegistry(Entity<AcpConnectionRegistry>);

impl Global for GlobalAcpConnectionRegistry {}

#[derive(Default)]
pub struct AcpConnectionRegistry {
    active_connection: RefCell<Option<ActiveConnection>>,
}

struct ActiveConnection {
    agent_id: AgentId,
    message_log: Weak<MessageLog>,
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
        agent_id: AgentId,
        message_log: &Rc<MessageLog>,
        cx: &mut Context<Self>,
    ) {
        self.active_connection.replace(Some(ActiveConnection {
            agent_id,
            message_log: Rc::downgrade(message_log),
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
    agent_id: AgentId,
    messages: Vec<WatchedConnectionMessage>,
    list_state: ListState,
    message_log: Weak<MessageLog>,
    last_seen_total: u64,
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
        let (agent_id, log_weak) = {
            let registry = self.connection_registry.read(cx);
            let active_connection = registry.active_connection.borrow();
            let Some(active_connection) = active_connection.as_ref() else {
                return;
            };

            if let Some(watched_connection) = self.watched_connection.as_ref() {
                if Weak::ptr_eq(
                    &watched_connection.message_log,
                    &active_connection.message_log,
                ) {
                    return;
                }
            }

            (
                active_connection.agent_id.clone(),
                active_connection.message_log.clone(),
            )
        };

        let Some(message_log) = log_weak.upgrade() else {
            return;
        };

        let snapshot = message_log.snapshot();
        let mut log_rx = message_log.subscribe();

        let task = cx.spawn({
            let log_weak = log_weak.clone();
            async move |this, cx| {
                while log_rx.recv().await.is_ok() {
                    let Some(log) = log_weak.upgrade() else {
                        break;
                    };
                    let Ok(Some(last_seen)) = this.read_with(cx, |this, _| {
                        this.watched_connection.as_ref().map(|c| c.last_seen_total)
                    }) else {
                        break;
                    };
                    let new_entries = log.read_since(last_seen);
                    drop(log);
                    if this
                        .update(cx, |this, cx| {
                            this.ingest_log_snapshot(new_entries, cx);
                        })
                        .is_err()
                    {
                        break;
                    }
                }
            }
        });

        self.watched_connection = Some(WatchedConnection {
            agent_id,
            messages: Vec::with_capacity(snapshot.messages.len()),
            list_state: ListState::new(0, ListAlignment::Bottom, px(2048.)),
            message_log: log_weak,
            last_seen_total: 0,
            incoming_request_methods: HashMap::default(),
            outgoing_request_methods: HashMap::default(),
            _task: task,
        });
        self.ingest_log_snapshot(snapshot, cx);
    }

    fn ingest_log_snapshot(&mut self, snapshot: MessageLogSnapshot, cx: &mut Context<Self>) {
        if snapshot.messages.is_empty() && snapshot.skipped == 0 {
            if let Some(connection) = self.watched_connection.as_mut() {
                connection.last_seen_total = snapshot.total_count;
            }
            return;
        }

        for message in snapshot.messages {
            self.push_stream_message(message, cx);
        }

        if let Some(connection) = self.watched_connection.as_mut() {
            connection.last_seen_total = snapshot.total_count;
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
