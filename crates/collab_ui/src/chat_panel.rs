use crate::{channel_view::ChannelView, ChatPanelSettings};
use anyhow::Result;
use call::ActiveCall;
use channel::{ChannelChat, ChannelChatEvent, ChannelMessageId, ChannelStore};
use client::{Client, UserStore};
use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use feature_flags::{ChannelsAlpha, FeatureFlagAppExt};
use gpui::{
    actions,
    elements::*,
    platform::{CursorStyle, MouseButton},
    serde_json,
    views::{ItemType, Select, SelectStyle},
    AnyViewHandle, AppContext, AsyncAppContext, Entity, ModelHandle, Subscription, Task, View,
    ViewContext, ViewHandle, WeakViewHandle,
};
use language::language_settings::SoftWrap;
use menu::Confirm;
use project::Fs;
use serde::{Deserialize, Serialize};
use settings::SettingsStore;
use std::sync::Arc;
use theme::{IconButton, Theme};
use time::{OffsetDateTime, UtcOffset};
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel},
    Workspace,
};

const MESSAGE_LOADING_THRESHOLD: usize = 50;
const CHAT_PANEL_KEY: &'static str = "ChatPanel";

pub struct ChatPanel {
    client: Arc<Client>,
    channel_store: ModelHandle<ChannelStore>,
    user_store: ModelHandle<UserStore>,
    active_chat: Option<(ModelHandle<ChannelChat>, Subscription)>,
    message_list: ListState<ChatPanel>,
    input_editor: ViewHandle<Editor>,
    channel_select: ViewHandle<Select>,
    local_timezone: UtcOffset,
    fs: Arc<dyn Fs>,
    width: Option<f32>,
    active: bool,
    pending_serialization: Task<Option<()>>,
    subscriptions: Vec<gpui::Subscription>,
    workspace: WeakViewHandle<Workspace>,
    has_focus: bool,
}

#[derive(Serialize, Deserialize)]
struct SerializedChatPanel {
    width: Option<f32>,
}

#[derive(Debug)]
pub enum Event {
    DockPositionChanged,
    Focus,
    Dismissed,
}

actions!(
    chat_panel,
    [LoadMoreMessages, ToggleFocus, OpenChannelNotes, JoinCall]
);

pub fn init(cx: &mut AppContext) {
    cx.add_action(ChatPanel::send);
    cx.add_action(ChatPanel::load_more_messages);
    cx.add_action(ChatPanel::open_notes);
    cx.add_action(ChatPanel::join_call);
}

impl ChatPanel {
    pub fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> ViewHandle<Self> {
        let fs = workspace.app_state().fs.clone();
        let client = workspace.app_state().client.clone();
        let channel_store = workspace.app_state().channel_store.clone();
        let user_store = workspace.app_state().user_store.clone();

        let input_editor = cx.add_view(|cx| {
            let mut editor = Editor::auto_height(
                4,
                Some(Arc::new(|theme| theme.chat_panel.input_editor.clone())),
                cx,
            );
            editor.set_soft_wrap_mode(SoftWrap::EditorWidth, cx);
            editor
        });

        let workspace_handle = workspace.weak_handle();

        let channel_select = cx.add_view(|cx| {
            let channel_store = channel_store.clone();
            let workspace = workspace_handle.clone();
            Select::new(0, cx, {
                move |ix, item_type, is_hovered, cx| {
                    Self::render_channel_name(
                        &channel_store,
                        ix,
                        item_type,
                        is_hovered,
                        workspace,
                        cx,
                    )
                }
            })
            .with_style(move |cx| {
                let style = &theme::current(cx).chat_panel.channel_select;
                SelectStyle {
                    header: Default::default(),
                    menu: style.menu,
                }
            })
        });

        let mut message_list =
            ListState::<Self>::new(0, Orientation::Bottom, 1000., move |this, ix, cx| {
                this.render_message(ix, cx)
            });
        message_list.set_scroll_handler(|visible_range, this, cx| {
            if visible_range.start < MESSAGE_LOADING_THRESHOLD {
                this.load_more_messages(&LoadMoreMessages, cx);
            }
        });

        cx.add_view(|cx| {
            let mut this = Self {
                fs,
                client,
                channel_store,
                user_store,
                active_chat: Default::default(),
                pending_serialization: Task::ready(None),
                message_list,
                input_editor,
                channel_select,
                local_timezone: cx.platform().local_timezone(),
                has_focus: false,
                subscriptions: Vec::new(),
                workspace: workspace_handle,
                active: false,
                width: None,
            };

            let mut old_dock_position = this.position(cx);
            this.subscriptions
                .push(
                    cx.observe_global::<SettingsStore, _>(move |this: &mut Self, cx| {
                        let new_dock_position = this.position(cx);
                        if new_dock_position != old_dock_position {
                            old_dock_position = new_dock_position;
                            cx.emit(Event::DockPositionChanged);
                        }
                        cx.notify();
                    }),
                );

            this.update_channel_count(cx);
            cx.observe(&this.channel_store, |this, _, cx| {
                this.update_channel_count(cx)
            })
            .detach();

            cx.observe(&this.channel_select, |this, channel_select, cx| {
                let selected_ix = channel_select.read(cx).selected_index();

                let selected_channel_id = this
                    .channel_store
                    .read(cx)
                    .channel_at(selected_ix)
                    .map(|e| e.id);
                if let Some(selected_channel_id) = selected_channel_id {
                    this.select_channel(selected_channel_id, cx)
                        .detach_and_log_err(cx);
                }
            })
            .detach();

            this
        })
    }

    pub fn active_chat(&self) -> Option<ModelHandle<ChannelChat>> {
        self.active_chat.as_ref().map(|(chat, _)| chat.clone())
    }

    pub fn load(
        workspace: WeakViewHandle<Workspace>,
        cx: AsyncAppContext,
    ) -> Task<Result<ViewHandle<Self>>> {
        cx.spawn(|mut cx| async move {
            let serialized_panel = if let Some(panel) = cx
                .background()
                .spawn(async move { KEY_VALUE_STORE.read_kvp(CHAT_PANEL_KEY) })
                .await
                .log_err()
                .flatten()
            {
                Some(serde_json::from_str::<SerializedChatPanel>(&panel)?)
            } else {
                None
            };

            workspace.update(&mut cx, |workspace, cx| {
                let panel = Self::new(workspace, cx);
                if let Some(serialized_panel) = serialized_panel {
                    panel.update(cx, |panel, cx| {
                        panel.width = serialized_panel.width;
                        cx.notify();
                    });
                }
                panel
            })
        })
    }

    fn serialize(&mut self, cx: &mut ViewContext<Self>) {
        let width = self.width;
        self.pending_serialization = cx.background().spawn(
            async move {
                KEY_VALUE_STORE
                    .write_kvp(
                        CHAT_PANEL_KEY.into(),
                        serde_json::to_string(&SerializedChatPanel { width })?,
                    )
                    .await?;
                anyhow::Ok(())
            }
            .log_err(),
        );
    }

    fn update_channel_count(&mut self, cx: &mut ViewContext<Self>) {
        let channel_count = self.channel_store.read(cx).channel_count();
        self.channel_select.update(cx, |select, cx| {
            select.set_item_count(channel_count, cx);
        });
    }

    fn set_active_chat(&mut self, chat: ModelHandle<ChannelChat>, cx: &mut ViewContext<Self>) {
        if self.active_chat.as_ref().map(|e| &e.0) != Some(&chat) {
            let id = chat.read(cx).channel().id;
            {
                let chat = chat.read(cx);
                self.message_list.reset(chat.message_count());
                let placeholder = format!("Message #{}", chat.channel().name);
                self.input_editor.update(cx, move |editor, cx| {
                    editor.set_placeholder_text(placeholder, cx);
                });
            }
            let subscription = cx.subscribe(&chat, Self::channel_did_change);
            self.active_chat = Some((chat, subscription));
            self.acknowledge_last_message(cx);
            self.channel_select.update(cx, |select, cx| {
                if let Some(ix) = self.channel_store.read(cx).index_of_channel(id) {
                    select.set_selected_index(ix, cx);
                }
            });
            cx.notify();
        }
    }

    fn channel_did_change(
        &mut self,
        _: ModelHandle<ChannelChat>,
        event: &ChannelChatEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            ChannelChatEvent::MessagesUpdated {
                old_range,
                new_count,
            } => {
                self.message_list.splice(old_range.clone(), *new_count);
                if self.active {
                    self.acknowledge_last_message(cx);
                }
            }
            ChannelChatEvent::NewMessage {
                channel_id,
                message_id,
            } => {
                if !self.active {
                    self.channel_store.update(cx, |store, cx| {
                        store.new_message(*channel_id, *message_id, cx)
                    })
                }
            }
        }
        cx.notify();
    }

    fn acknowledge_last_message(&mut self, cx: &mut ViewContext<'_, '_, ChatPanel>) {
        if self.active {
            if let Some((chat, _)) = &self.active_chat {
                chat.update(cx, |chat, cx| {
                    chat.acknowledge_last_message(cx);
                });
            }
        }
    }

    fn render_channel(&self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = theme::current(cx);
        Flex::column()
            .with_child(
                ChildView::new(&self.channel_select, cx)
                    .contained()
                    .with_style(theme.chat_panel.channel_select.container),
            )
            .with_child(self.render_active_channel_messages(&theme))
            .with_child(self.render_input_box(&theme, cx))
            .into_any()
    }

    fn render_active_channel_messages(&self, theme: &Arc<Theme>) -> AnyElement<Self> {
        let messages = if self.active_chat.is_some() {
            List::new(self.message_list.clone())
                .contained()
                .with_style(theme.chat_panel.list)
                .into_any()
        } else {
            Empty::new().into_any()
        };

        messages.flex(1., true).into_any()
    }

    fn render_message(&self, ix: usize, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let message = self.active_chat.as_ref().unwrap().0.read(cx).message(ix);

        let now = OffsetDateTime::now_utc();
        let theme = theme::current(cx);
        let style = if message.is_pending() {
            &theme.chat_panel.pending_message
        } else {
            &theme.chat_panel.message
        };

        let belongs_to_user = Some(message.sender.id) == self.client.user_id();
        let message_id_to_remove =
            if let (ChannelMessageId::Saved(id), true) = (message.id, belongs_to_user) {
                Some(id)
            } else {
                None
            };

        enum DeleteMessage {}

        let body = message.body.clone();
        Flex::column()
            .with_child(
                Flex::row()
                    .with_child(
                        message
                            .sender
                            .avatar
                            .clone()
                            .map(|avatar| {
                                Image::from_data(avatar)
                                    .with_style(theme.collab_panel.channel_avatar)
                                    .into_any()
                            })
                            .unwrap_or_else(|| {
                                Empty::new()
                                    .constrained()
                                    .with_width(
                                        theme.collab_panel.channel_avatar.width.unwrap_or(12.),
                                    )
                                    .into_any()
                            })
                            .contained()
                            .with_margin_right(4.),
                    )
                    .with_child(
                        Label::new(
                            message.sender.github_login.clone(),
                            style.sender.text.clone(),
                        )
                        .contained()
                        .with_style(style.sender.container),
                    )
                    .with_child(
                        Label::new(
                            format_timestamp(message.timestamp, now, self.local_timezone),
                            style.timestamp.text.clone(),
                        )
                        .contained()
                        .with_style(style.timestamp.container),
                    )
                    .with_children(message_id_to_remove.map(|id| {
                        MouseEventHandler::new::<DeleteMessage, _>(
                            id as usize,
                            cx,
                            |mouse_state, _| {
                                let button_style =
                                    theme.chat_panel.icon_button.style_for(mouse_state);
                                render_icon_button(button_style, "icons/x.svg")
                                    .aligned()
                                    .into_any()
                            },
                        )
                        .with_padding(Padding::uniform(2.))
                        .with_cursor_style(CursorStyle::PointingHand)
                        .on_click(MouseButton::Left, move |_, this, cx| {
                            this.remove_message(id, cx);
                        })
                        .flex_float()
                    }))
                    .align_children_center(),
            )
            .with_child(Text::new(body, style.body.clone()))
            .contained()
            .with_style(style.container)
            .into_any()
    }

    fn render_input_box(&self, theme: &Arc<Theme>, cx: &AppContext) -> AnyElement<Self> {
        ChildView::new(&self.input_editor, cx)
            .contained()
            .with_style(theme.chat_panel.input_editor.container)
            .into_any()
    }

    fn render_channel_name(
        channel_store: &ModelHandle<ChannelStore>,
        ix: usize,
        item_type: ItemType,
        is_hovered: bool,
        workspace: WeakViewHandle<Workspace>,
        cx: &mut ViewContext<Select>,
    ) -> AnyElement<Select> {
        let theme = theme::current(cx);
        let tooltip_style = &theme.tooltip;
        let theme = &theme.chat_panel;
        let style = match (&item_type, is_hovered) {
            (ItemType::Header, _) => &theme.channel_select.header,
            (ItemType::Selected, _) => &theme.channel_select.active_item,
            (ItemType::Unselected, false) => &theme.channel_select.item,
            (ItemType::Unselected, true) => &theme.channel_select.hovered_item,
        };

        let channel = &channel_store.read(cx).channel_at(ix).unwrap();
        let channel_id = channel.id;

        let mut row = Flex::row()
            .with_child(
                Label::new("#".to_string(), style.hash.text.clone())
                    .contained()
                    .with_style(style.hash.container),
            )
            .with_child(Label::new(channel.name.clone(), style.name.clone()));

        if matches!(item_type, ItemType::Header) {
            row.add_children([
                MouseEventHandler::new::<OpenChannelNotes, _>(0, cx, |mouse_state, _| {
                    render_icon_button(theme.icon_button.style_for(mouse_state), "icons/file.svg")
                })
                .on_click(MouseButton::Left, move |_, _, cx| {
                    if let Some(workspace) = workspace.upgrade(cx) {
                        ChannelView::open(channel_id, workspace, cx).detach();
                    }
                })
                .with_tooltip::<OpenChannelNotes>(
                    channel_id as usize,
                    "Open Notes",
                    Some(Box::new(OpenChannelNotes)),
                    tooltip_style.clone(),
                    cx,
                )
                .flex_float(),
                MouseEventHandler::new::<ActiveCall, _>(0, cx, |mouse_state, _| {
                    render_icon_button(
                        theme.icon_button.style_for(mouse_state),
                        "icons/speaker-loud.svg",
                    )
                })
                .on_click(MouseButton::Left, move |_, _, cx| {
                    ActiveCall::global(cx)
                        .update(cx, |call, cx| call.join_channel(channel_id, cx))
                        .detach_and_log_err(cx);
                })
                .with_tooltip::<ActiveCall>(
                    channel_id as usize,
                    "Join Call",
                    Some(Box::new(JoinCall)),
                    tooltip_style.clone(),
                    cx,
                )
                .flex_float(),
            ]);
        }

        row.align_children_center()
            .contained()
            .with_style(style.container)
            .into_any()
    }

    fn render_sign_in_prompt(
        &self,
        theme: &Arc<Theme>,
        cx: &mut ViewContext<Self>,
    ) -> AnyElement<Self> {
        enum SignInPromptLabel {}

        MouseEventHandler::new::<SignInPromptLabel, _>(0, cx, |mouse_state, _| {
            Label::new(
                "Sign in to use chat".to_string(),
                theme
                    .chat_panel
                    .sign_in_prompt
                    .style_for(mouse_state)
                    .clone(),
            )
        })
        .with_cursor_style(CursorStyle::PointingHand)
        .on_click(MouseButton::Left, move |_, this, cx| {
            let client = this.client.clone();
            cx.spawn(|this, mut cx| async move {
                if client
                    .authenticate_and_connect(true, &cx)
                    .log_err()
                    .await
                    .is_some()
                {
                    this.update(&mut cx, |this, cx| {
                        if cx.handle().is_focused(cx) {
                            cx.focus(&this.input_editor);
                        }
                    })
                    .ok();
                }
            })
            .detach();
        })
        .aligned()
        .into_any()
    }

    fn send(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if let Some((chat, _)) = self.active_chat.as_ref() {
            let body = self.input_editor.update(cx, |editor, cx| {
                let body = editor.text(cx);
                editor.clear(cx);
                body
            });

            if let Some(task) = chat
                .update(cx, |chat, cx| chat.send_message(body, cx))
                .log_err()
            {
                task.detach();
            }
        }
    }

    fn remove_message(&mut self, id: u64, cx: &mut ViewContext<Self>) {
        if let Some((chat, _)) = self.active_chat.as_ref() {
            chat.update(cx, |chat, cx| chat.remove_message(id, cx).detach())
        }
    }

    fn load_more_messages(&mut self, _: &LoadMoreMessages, cx: &mut ViewContext<Self>) {
        if let Some((chat, _)) = self.active_chat.as_ref() {
            chat.update(cx, |channel, cx| {
                channel.load_more_messages(cx);
            })
        }
    }

    pub fn select_channel(
        &mut self,
        selected_channel_id: u64,
        cx: &mut ViewContext<ChatPanel>,
    ) -> Task<Result<()>> {
        if let Some((chat, _)) = &self.active_chat {
            if chat.read(cx).channel().id == selected_channel_id {
                return Task::ready(Ok(()));
            }
        }

        let open_chat = self.channel_store.update(cx, |store, cx| {
            store.open_channel_chat(selected_channel_id, cx)
        });
        cx.spawn(|this, mut cx| async move {
            let chat = open_chat.await?;
            this.update(&mut cx, |this, cx| {
                this.set_active_chat(chat, cx);
            })
        })
    }

    fn open_notes(&mut self, _: &OpenChannelNotes, cx: &mut ViewContext<Self>) {
        if let Some((chat, _)) = &self.active_chat {
            let channel_id = chat.read(cx).channel().id;
            if let Some(workspace) = self.workspace.upgrade(cx) {
                ChannelView::open(channel_id, workspace, cx).detach();
            }
        }
    }

    fn join_call(&mut self, _: &JoinCall, cx: &mut ViewContext<Self>) {
        if let Some((chat, _)) = &self.active_chat {
            let channel_id = chat.read(cx).channel().id;
            ActiveCall::global(cx)
                .update(cx, |call, cx| call.join_channel(channel_id, cx))
                .detach_and_log_err(cx);
        }
    }
}

impl Entity for ChatPanel {
    type Event = Event;
}

impl View for ChatPanel {
    fn ui_name() -> &'static str {
        "ChatPanel"
    }

    fn render(&mut self, cx: &mut ViewContext<Self>) -> AnyElement<Self> {
        let theme = theme::current(cx);
        let element = if self.client.user_id().is_some() {
            self.render_channel(cx)
        } else {
            self.render_sign_in_prompt(&theme, cx)
        };
        element
            .contained()
            .with_style(theme.chat_panel.container)
            .constrained()
            .with_min_width(150.)
            .into_any()
    }

    fn focus_in(&mut self, _: AnyViewHandle, cx: &mut ViewContext<Self>) {
        self.has_focus = true;
        if matches!(
            *self.client.status().borrow(),
            client::Status::Connected { .. }
        ) {
            cx.focus(&self.input_editor);
        }
    }

    fn focus_out(&mut self, _: AnyViewHandle, _: &mut ViewContext<Self>) {
        self.has_focus = false;
    }
}

impl Panel for ChatPanel {
    fn position(&self, cx: &gpui::WindowContext) -> DockPosition {
        settings::get::<ChatPanelSettings>(cx).dock
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<ChatPanelSettings>(self.fs.clone(), cx, move |settings| {
            settings.dock = Some(position)
        });
    }

    fn size(&self, cx: &gpui::WindowContext) -> f32 {
        self.width
            .unwrap_or_else(|| settings::get::<ChatPanelSettings>(cx).default_width)
    }

    fn set_size(&mut self, size: Option<f32>, cx: &mut ViewContext<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        self.active = active;
        if active {
            self.acknowledge_last_message(cx);
            if !is_chat_feature_enabled(cx) {
                cx.emit(Event::Dismissed);
            }
        }
    }

    fn icon_path(&self, cx: &gpui::WindowContext) -> Option<&'static str> {
        (settings::get::<ChatPanelSettings>(cx).button && is_chat_feature_enabled(cx))
            .then(|| "icons/conversations.svg")
    }

    fn icon_tooltip(&self) -> (String, Option<Box<dyn gpui::Action>>) {
        ("Chat Panel".to_string(), Some(Box::new(ToggleFocus)))
    }

    fn should_change_position_on_event(event: &Self::Event) -> bool {
        matches!(event, Event::DockPositionChanged)
    }

    fn should_close_on_event(event: &Self::Event) -> bool {
        matches!(event, Event::Dismissed)
    }

    fn has_focus(&self, _cx: &gpui::WindowContext) -> bool {
        self.has_focus
    }

    fn is_focus_event(event: &Self::Event) -> bool {
        matches!(event, Event::Focus)
    }
}

fn is_chat_feature_enabled(cx: &gpui::WindowContext<'_>) -> bool {
    cx.is_staff() || cx.has_flag::<ChannelsAlpha>()
}

fn format_timestamp(
    mut timestamp: OffsetDateTime,
    mut now: OffsetDateTime,
    local_timezone: UtcOffset,
) -> String {
    timestamp = timestamp.to_offset(local_timezone);
    now = now.to_offset(local_timezone);

    let today = now.date();
    let date = timestamp.date();
    let mut hour = timestamp.hour();
    let mut part = "am";
    if hour > 12 {
        hour -= 12;
        part = "pm";
    }
    if date == today {
        format!("{:02}:{:02}{}", hour, timestamp.minute(), part)
    } else if date.next_day() == Some(today) {
        format!("yesterday at {:02}:{:02}{}", hour, timestamp.minute(), part)
    } else {
        format!("{:02}/{}/{}", date.month() as u32, date.day(), date.year())
    }
}

fn render_icon_button<V: View>(style: &IconButton, svg_path: &'static str) -> impl Element<V> {
    Svg::new(svg_path)
        .with_color(style.color)
        .constrained()
        .with_width(style.icon_width)
        .aligned()
        .constrained()
        .with_width(style.button_width)
        .with_height(style.button_width)
        .contained()
        .with_style(style.container)
}
