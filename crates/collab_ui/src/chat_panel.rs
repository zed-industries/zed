use crate::{collab_panel, ChatPanelSettings};
use anyhow::Result;
use call::{room, ActiveCall};
use channel::{ChannelChat, ChannelChatEvent, ChannelMessageId, ChannelStore};
use client::Client;
use collections::HashMap;
use db::kvp::KEY_VALUE_STORE;
use editor::Editor;
use gpui::{
    actions, div, list, prelude::*, px, Action, AppContext, AsyncWindowContext, DismissEvent,
    ElementId, EventEmitter, FocusHandle, FocusableView, FontWeight, ListOffset, ListScrollEvent,
    ListState, Model, Render, Subscription, Task, View, ViewContext, VisualContext, WeakView,
};
use language::LanguageRegistry;
use menu::Confirm;
use message_editor::MessageEditor;
use project::Fs;
use rich_text::RichText;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::{sync::Arc, time::Duration};
use time::{OffsetDateTime, UtcOffset};
use ui::{
    popover_menu, prelude::*, Avatar, Button, ContextMenu, IconButton, IconName, KeyBinding, Label,
    TabBar,
};
use util::{ResultExt, TryFutureExt};
use workspace::{
    dock::{DockPosition, Panel, PanelEvent},
    Workspace,
};

mod message_editor;

const MESSAGE_LOADING_THRESHOLD: usize = 50;
const CHAT_PANEL_KEY: &'static str = "ChatPanel";

pub fn init(cx: &mut AppContext) {
    cx.observe_new_views(|workspace: &mut Workspace, _| {
        workspace.register_action(|workspace, _: &ToggleFocus, cx| {
            workspace.toggle_panel_focus::<ChatPanel>(cx);
        });
    })
    .detach();
}

pub struct ChatPanel {
    client: Arc<Client>,
    channel_store: Model<ChannelStore>,
    languages: Arc<LanguageRegistry>,
    message_list: ListState,
    active_chat: Option<(Model<ChannelChat>, Subscription)>,
    message_editor: View<MessageEditor>,
    local_timezone: UtcOffset,
    fs: Arc<dyn Fs>,
    width: Option<Pixels>,
    active: bool,
    pending_serialization: Task<Option<()>>,
    subscriptions: Vec<gpui::Subscription>,
    is_scrolled_to_bottom: bool,
    markdown_data: HashMap<ChannelMessageId, RichText>,
    focus_handle: FocusHandle,
    open_context_menu: Option<(u64, Subscription)>,
}

#[derive(Serialize, Deserialize)]
struct SerializedChatPanel {
    width: Option<Pixels>,
}

actions!(chat_panel, [ToggleFocus]);

impl ChatPanel {
    pub fn new(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) -> View<Self> {
        let fs = workspace.app_state().fs.clone();
        let client = workspace.app_state().client.clone();
        let channel_store = ChannelStore::global(cx);
        let languages = workspace.app_state().languages.clone();

        let input_editor = cx.new_view(|cx| {
            MessageEditor::new(
                languages.clone(),
                channel_store.clone(),
                cx.new_view(|cx| Editor::auto_height(4, cx)),
                cx,
            )
        });

        cx.new_view(|cx: &mut ViewContext<Self>| {
            let view = cx.view().downgrade();
            let message_list =
                ListState::new(0, gpui::ListAlignment::Bottom, px(1000.), move |ix, cx| {
                    if let Some(view) = view.upgrade() {
                        view.update(cx, |view, cx| {
                            view.render_message(ix, cx).into_any_element()
                        })
                    } else {
                        div().into_any()
                    }
                });

            message_list.set_scroll_handler(cx.listener(|this, event: &ListScrollEvent, cx| {
                if event.visible_range.start < MESSAGE_LOADING_THRESHOLD {
                    this.load_more_messages(cx);
                }
                this.is_scrolled_to_bottom = !event.is_scrolled;
            }));

            let mut this = Self {
                fs,
                client,
                channel_store,
                languages,
                message_list,
                active_chat: Default::default(),
                pending_serialization: Task::ready(None),
                message_editor: input_editor,
                local_timezone: cx.local_timezone(),
                subscriptions: Vec::new(),
                is_scrolled_to_bottom: true,
                active: false,
                width: None,
                markdown_data: Default::default(),
                focus_handle: cx.focus_handle(),
                open_context_menu: None,
            };

            if let Some(channel_id) = ActiveCall::global(cx)
                .read(cx)
                .room()
                .and_then(|room| room.read(cx).channel_id())
            {
                this.select_channel(channel_id, None, cx)
                    .detach_and_log_err(cx);

                if ActiveCall::global(cx)
                    .read(cx)
                    .room()
                    .is_some_and(|room| room.read(cx).contains_guests())
                {
                    cx.emit(PanelEvent::Activate)
                }
            }

            this.subscriptions.push(cx.subscribe(
                &ActiveCall::global(cx),
                move |this: &mut Self, call, event: &room::Event, cx| match event {
                    room::Event::RoomJoined { channel_id } => {
                        if let Some(channel_id) = channel_id {
                            this.select_channel(*channel_id, None, cx)
                                .detach_and_log_err(cx);

                            if call
                                .read(cx)
                                .room()
                                .is_some_and(|room| room.read(cx).contains_guests())
                            {
                                cx.emit(PanelEvent::Activate)
                            }
                        }
                    }
                    room::Event::Left { channel_id } => {
                        if channel_id == &this.channel_id(cx) {
                            cx.emit(PanelEvent::Close)
                        }
                    }
                    _ => {}
                },
            ));

            this
        })
    }

    pub fn channel_id(&self, cx: &AppContext) -> Option<u64> {
        self.active_chat
            .as_ref()
            .map(|(chat, _)| chat.read(cx).channel_id)
    }

    pub fn is_scrolled_to_bottom(&self) -> bool {
        self.is_scrolled_to_bottom
    }

    pub fn active_chat(&self) -> Option<Model<ChannelChat>> {
        self.active_chat.as_ref().map(|(chat, _)| chat.clone())
    }

    pub fn load(
        workspace: WeakView<Workspace>,
        cx: AsyncWindowContext,
    ) -> Task<Result<View<Self>>> {
        cx.spawn(|mut cx| async move {
            let serialized_panel = if let Some(panel) = cx
                .background_executor()
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
        self.pending_serialization = cx.background_executor().spawn(
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

    fn set_active_chat(&mut self, chat: Model<ChannelChat>, cx: &mut ViewContext<Self>) {
        if self.active_chat.as_ref().map(|e| &e.0) != Some(&chat) {
            let channel_id = chat.read(cx).channel_id;
            {
                self.markdown_data.clear();
                let chat = chat.read(cx);
                self.message_list.reset(chat.message_count());

                let channel_name = chat.channel(cx).map(|channel| channel.name.clone());
                self.message_editor.update(cx, |editor, cx| {
                    editor.set_channel(channel_id, channel_name, cx);
                });
            };
            let subscription = cx.subscribe(&chat, Self::channel_did_change);
            self.active_chat = Some((chat, subscription));
            self.acknowledge_last_message(cx);
            cx.notify();
        }
    }

    fn channel_did_change(
        &mut self,
        _: Model<ChannelChat>,
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

    fn acknowledge_last_message(&mut self, cx: &mut ViewContext<Self>) {
        if self.active && self.is_scrolled_to_bottom {
            if let Some((chat, _)) = &self.active_chat {
                chat.update(cx, |chat, cx| {
                    chat.acknowledge_last_message(cx);
                });
            }
        }
    }

    fn render_message(&mut self, ix: usize, cx: &mut ViewContext<Self>) -> impl IntoElement {
        let active_chat = &self.active_chat.as_ref().unwrap().0;
        let (message, is_continuation_from_previous, is_admin) =
            active_chat.update(cx, |active_chat, cx| {
                let is_admin = self
                    .channel_store
                    .read(cx)
                    .is_channel_admin(active_chat.channel_id);

                let last_message = active_chat.message(ix.saturating_sub(1));
                let this_message = active_chat.message(ix).clone();

                let duration_since_last_message = this_message.timestamp - last_message.timestamp;
                let is_continuation_from_previous = last_message.sender.id
                    == this_message.sender.id
                    && last_message.id != this_message.id
                    && duration_since_last_message < Duration::from_secs(5 * 60);

                if let ChannelMessageId::Saved(id) = this_message.id {
                    if this_message
                        .mentions
                        .iter()
                        .any(|(_, user_id)| Some(*user_id) == self.client.user_id())
                    {
                        active_chat.acknowledge_message(id);
                    }
                }

                (this_message, is_continuation_from_previous, is_admin)
            });

        let _is_pending = message.is_pending();
        let text = self.markdown_data.entry(message.id).or_insert_with(|| {
            Self::render_markdown_with_mentions(&self.languages, self.client.id(), &message)
        });

        let belongs_to_user = Some(message.sender.id) == self.client.user_id();
        let message_id_to_remove = if let (ChannelMessageId::Saved(id), true) =
            (message.id, belongs_to_user || is_admin)
        {
            Some(id)
        } else {
            None
        };

        let element_id: ElementId = match message.id {
            ChannelMessageId::Saved(id) => ("saved-message", id).into(),
            ChannelMessageId::Pending(id) => ("pending-message", id).into(),
        };
        let this = cx.view().clone();

        v_flex()
            .w_full()
            .relative()
            .overflow_hidden()
            .when(!is_continuation_from_previous, |this| {
                this.pt_3().child(
                    h_flex()
                        .text_ui_sm()
                        .child(div().absolute().child(
                            Avatar::new(message.sender.avatar_uri.clone()).size(cx.rem_size()),
                        ))
                        .child(
                            div()
                                .pl(cx.rem_size() + px(6.0))
                                .pr(px(8.0))
                                .font_weight(FontWeight::BOLD)
                                .child(Label::new(message.sender.github_login.clone())),
                        )
                        .child(
                            Label::new(format_timestamp(
                                OffsetDateTime::now_utc(),
                                message.timestamp,
                                self.local_timezone,
                            ))
                            .size(LabelSize::Small)
                            .color(Color::Muted),
                        ),
                )
            })
            .when(is_continuation_from_previous, |this| this.pt_1())
            .child(
                v_flex()
                    .w_full()
                    .text_ui_sm()
                    .id(element_id)
                    .group("")
                    .child(text.element("body".into(), cx))
                    .child(
                        div()
                            .absolute()
                            .z_index(1)
                            .right_0()
                            .w_6()
                            .bg(cx.theme().colors().panel_background)
                            .when(!self.has_open_menu(message_id_to_remove), |el| {
                                el.visible_on_hover("")
                            })
                            .children(message_id_to_remove.map(|message_id| {
                                popover_menu(("menu", message_id))
                                    .trigger(IconButton::new(
                                        ("trigger", message_id),
                                        IconName::Ellipsis,
                                    ))
                                    .menu(move |cx| {
                                        Some(Self::render_message_menu(&this, message_id, cx))
                                    })
                            })),
                    ),
            )
    }

    fn has_open_menu(&self, message_id: Option<u64>) -> bool {
        match self.open_context_menu.as_ref() {
            Some((id, _)) => Some(*id) == message_id,
            None => false,
        }
    }

    fn render_message_menu(
        this: &View<Self>,
        message_id: u64,
        cx: &mut WindowContext,
    ) -> View<ContextMenu> {
        let menu = {
            let this = this.clone();
            ContextMenu::build(cx, move |menu, _| {
                menu.entry("Delete message", None, move |cx| {
                    this.update(cx, |this, cx| this.remove_message(message_id, cx))
                })
            })
        };
        this.update(cx, |this, cx| {
            let subscription = cx.subscribe(&menu, |this: &mut Self, _, _: &DismissEvent, _| {
                this.open_context_menu = None;
            });
            this.open_context_menu = Some((message_id, subscription));
        });
        menu
    }

    fn render_markdown_with_mentions(
        language_registry: &Arc<LanguageRegistry>,
        current_user_id: u64,
        message: &channel::ChannelMessage,
    ) -> RichText {
        let mentions = message
            .mentions
            .iter()
            .map(|(range, user_id)| rich_text::Mention {
                range: range.clone(),
                is_self_mention: *user_id == current_user_id,
            })
            .collect::<Vec<_>>();

        rich_text::render_markdown(message.body.clone(), &mentions, language_registry, None)
    }

    fn send(&mut self, _: &Confirm, cx: &mut ViewContext<Self>) {
        if let Some((chat, _)) = self.active_chat.as_ref() {
            let message = self
                .message_editor
                .update(cx, |editor, cx| editor.take_message(cx));

            if let Some(task) = chat
                .update(cx, |chat, cx| chat.send_message(message, cx))
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

    fn load_more_messages(&mut self, cx: &mut ViewContext<Self>) {
        if let Some((chat, _)) = self.active_chat.as_ref() {
            chat.update(cx, |channel, cx| {
                if let Some(task) = channel.load_more_messages(cx) {
                    task.detach();
                }
            })
        }
    }

    pub fn select_channel(
        &mut self,
        selected_channel_id: u64,
        scroll_to_message_id: Option<u64>,
        cx: &mut ViewContext<ChatPanel>,
    ) -> Task<Result<()>> {
        let open_chat = self
            .active_chat
            .as_ref()
            .and_then(|(chat, _)| {
                (chat.read(cx).channel_id == selected_channel_id)
                    .then(|| Task::ready(anyhow::Ok(chat.clone())))
            })
            .unwrap_or_else(|| {
                self.channel_store.update(cx, |store, cx| {
                    store.open_channel_chat(selected_channel_id, cx)
                })
            });

        cx.spawn(|this, mut cx| async move {
            let chat = open_chat.await?;
            this.update(&mut cx, |this, cx| {
                this.set_active_chat(chat.clone(), cx);
            })?;

            if let Some(message_id) = scroll_to_message_id {
                if let Some(item_ix) =
                    ChannelChat::load_history_since_message(chat.clone(), message_id, (*cx).clone())
                        .await
                {
                    this.update(&mut cx, |this, cx| {
                        if this.active_chat.as_ref().map_or(false, |(c, _)| *c == chat) {
                            this.message_list.scroll_to(ListOffset {
                                item_ix,
                                offset_in_item: px(0.0),
                            });
                            cx.notify();
                        }
                    })?;
                }
            }

            Ok(())
        })
    }
}

impl Render for ChatPanel {
    fn render(&mut self, cx: &mut ViewContext<Self>) -> impl IntoElement {
        v_flex()
            .track_focus(&self.focus_handle)
            .full()
            .on_action(cx.listener(Self::send))
            .child(
                h_flex().z_index(1).child(
                    TabBar::new("chat_header").child(
                        h_flex()
                            .w_full()
                            .h(rems(ui::Tab::CONTAINER_HEIGHT_IN_REMS))
                            .px_2()
                            .child(Label::new(
                                self.active_chat
                                    .as_ref()
                                    .and_then(|c| {
                                        Some(format!("#{}", c.0.read(cx).channel(cx)?.name))
                                    })
                                    .unwrap_or("Chat".to_string()),
                            )),
                    ),
                ),
            )
            .child(div().flex_grow().px_2().pt_1().map(|this| {
                if self.active_chat.is_some() {
                    this.child(list(self.message_list.clone()).full())
                } else {
                    this.child(
                        div()
                            .full()
                            .p_4()
                            .child(
                                Label::new("Select a channel to chat in.")
                                    .size(LabelSize::Small)
                                    .color(Color::Muted),
                            )
                            .child(
                                div().pt_1().w_full().items_center().child(
                                    Button::new("toggle-collab", "Open")
                                        .full_width()
                                        .key_binding(KeyBinding::for_action(
                                            &collab_panel::ToggleFocus,
                                            cx,
                                        ))
                                        .on_click(|_, cx| {
                                            cx.dispatch_action(
                                                collab_panel::ToggleFocus.boxed_clone(),
                                            )
                                        }),
                                ),
                            ),
                    )
                }
            }))
            .child(
                h_flex()
                    .when(!self.is_scrolled_to_bottom, |el| {
                        el.border_t_1().border_color(cx.theme().colors().border)
                    })
                    .p_2()
                    .map(|el| {
                        if self.active_chat.is_some() {
                            el.child(self.message_editor.clone())
                        } else {
                            el.child(
                                div()
                                    .rounded_md()
                                    .h_6()
                                    .w_full()
                                    .bg(cx.theme().colors().editor_background),
                            )
                        }
                    }),
            )
            .into_any()
    }
}

impl FocusableView for ChatPanel {
    fn focus_handle(&self, cx: &AppContext) -> gpui::FocusHandle {
        if self.active_chat.is_some() {
            self.message_editor.read(cx).focus_handle(cx)
        } else {
            self.focus_handle.clone()
        }
    }
}

impl Panel for ChatPanel {
    fn position(&self, cx: &gpui::WindowContext) -> DockPosition {
        ChatPanelSettings::get_global(cx).dock
    }

    fn position_is_valid(&self, position: DockPosition) -> bool {
        matches!(position, DockPosition::Left | DockPosition::Right)
    }

    fn set_position(&mut self, position: DockPosition, cx: &mut ViewContext<Self>) {
        settings::update_settings_file::<ChatPanelSettings>(self.fs.clone(), cx, move |settings| {
            settings.dock = Some(position)
        });
    }

    fn size(&self, cx: &gpui::WindowContext) -> Pixels {
        self.width
            .unwrap_or_else(|| ChatPanelSettings::get_global(cx).default_width)
    }

    fn set_size(&mut self, size: Option<Pixels>, cx: &mut ViewContext<Self>) {
        self.width = size;
        self.serialize(cx);
        cx.notify();
    }

    fn set_active(&mut self, active: bool, cx: &mut ViewContext<Self>) {
        self.active = active;
        if active {
            self.acknowledge_last_message(cx);
        }
    }

    fn persistent_name() -> &'static str {
        "ChatPanel"
    }

    fn icon(&self, cx: &WindowContext) -> Option<ui::IconName> {
        Some(ui::IconName::MessageBubbles).filter(|_| ChatPanelSettings::get_global(cx).button)
    }

    fn icon_tooltip(&self, _cx: &WindowContext) -> Option<&'static str> {
        Some("Chat Panel")
    }

    fn toggle_action(&self) -> Box<dyn gpui::Action> {
        Box::new(ToggleFocus)
    }
}

impl EventEmitter<PanelEvent> for ChatPanel {}

fn format_timestamp(
    reference: OffsetDateTime,
    timestamp: OffsetDateTime,
    timezone: UtcOffset,
) -> String {
    let timestamp_local = timestamp.to_offset(timezone);
    let timestamp_local_hour = timestamp_local.hour();

    let hour_12 = match timestamp_local_hour {
        0 => 12,                              // Midnight
        13..=23 => timestamp_local_hour - 12, // PM hours
        _ => timestamp_local_hour,            // AM hours
    };
    let meridiem = if timestamp_local_hour >= 12 {
        "pm"
    } else {
        "am"
    };
    let timestamp_local_minute = timestamp_local.minute();
    let formatted_time = format!("{:02}:{:02} {}", hour_12, timestamp_local_minute, meridiem);

    let reference_local = reference.to_offset(timezone);
    let reference_local_date = reference_local.date();
    let timestamp_local_date = timestamp_local.date();

    if timestamp_local_date == reference_local_date {
        return formatted_time;
    }

    if reference_local_date.previous_day() == Some(timestamp_local_date) {
        return format!("yesterday at {}", formatted_time);
    }

    format!(
        "{:02}/{:02}/{}",
        timestamp_local_date.month() as u32,
        timestamp_local_date.day(),
        timestamp_local_date.year()
    )
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::HighlightStyle;
    use pretty_assertions::assert_eq;
    use rich_text::Highlight;
    use time::{Date, OffsetDateTime, Time, UtcOffset};
    use util::test::marked_text_ranges;

    #[gpui::test]
    fn test_render_markdown_with_mentions() {
        let language_registry = Arc::new(LanguageRegistry::test());
        let (body, ranges) = marked_text_ranges("*hi*, «@abc», let's **call** «@fgh»", false);
        let message = channel::ChannelMessage {
            id: ChannelMessageId::Saved(0),
            body,
            timestamp: OffsetDateTime::now_utc(),
            sender: Arc::new(client::User {
                github_login: "fgh".into(),
                avatar_uri: "avatar_fgh".into(),
                id: 103,
            }),
            nonce: 5,
            mentions: vec![(ranges[0].clone(), 101), (ranges[1].clone(), 102)],
        };

        let message = ChatPanel::render_markdown_with_mentions(&language_registry, 102, &message);

        // Note that the "'" was replaced with ’ due to smart punctuation.
        let (body, ranges) = marked_text_ranges("«hi», «@abc», let’s «call» «@fgh»", false);
        assert_eq!(message.text, body);
        assert_eq!(
            message.highlights,
            vec![
                (
                    ranges[0].clone(),
                    HighlightStyle {
                        font_style: Some(gpui::FontStyle::Italic),
                        ..Default::default()
                    }
                    .into()
                ),
                (ranges[1].clone(), Highlight::Mention),
                (
                    ranges[2].clone(),
                    HighlightStyle {
                        font_weight: Some(gpui::FontWeight::BOLD),
                        ..Default::default()
                    }
                    .into()
                ),
                (ranges[3].clone(), Highlight::SelfMention)
            ]
        );
    }

    #[test]
    fn test_format_today() {
        let reference = create_offset_datetime(1990, 4, 12, 16, 45, 0);
        let timestamp = create_offset_datetime(1990, 4, 12, 15, 30, 0);

        assert_eq!(
            format_timestamp(reference, timestamp, test_timezone()),
            "03:30 pm"
        );
    }

    #[test]
    fn test_format_yesterday() {
        let reference = create_offset_datetime(1990, 4, 12, 10, 30, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 9, 0, 0);

        assert_eq!(
            format_timestamp(reference, timestamp, test_timezone()),
            "yesterday at 09:00 am"
        );
    }

    #[test]
    fn test_format_yesterday_less_than_24_hours_ago() {
        let reference = create_offset_datetime(1990, 4, 12, 19, 59, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 20, 0, 0);

        assert_eq!(
            format_timestamp(reference, timestamp, test_timezone()),
            "yesterday at 08:00 pm"
        );
    }

    #[test]
    fn test_format_yesterday_more_than_24_hours_ago() {
        let reference = create_offset_datetime(1990, 4, 12, 19, 59, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 18, 0, 0);

        assert_eq!(
            format_timestamp(reference, timestamp, test_timezone()),
            "yesterday at 06:00 pm"
        );
    }

    #[test]
    fn test_format_yesterday_over_midnight() {
        let reference = create_offset_datetime(1990, 4, 12, 0, 5, 0);
        let timestamp = create_offset_datetime(1990, 4, 11, 23, 55, 0);

        assert_eq!(
            format_timestamp(reference, timestamp, test_timezone()),
            "yesterday at 11:55 pm"
        );
    }

    #[test]
    fn test_format_yesterday_over_month() {
        let reference = create_offset_datetime(1990, 4, 2, 9, 0, 0);
        let timestamp = create_offset_datetime(1990, 4, 1, 20, 0, 0);

        assert_eq!(
            format_timestamp(reference, timestamp, test_timezone()),
            "yesterday at 08:00 pm"
        );
    }

    #[test]
    fn test_format_before_yesterday() {
        let reference = create_offset_datetime(1990, 4, 12, 10, 30, 0);
        let timestamp = create_offset_datetime(1990, 4, 10, 20, 20, 0);

        assert_eq!(
            format_timestamp(reference, timestamp, test_timezone()),
            "04/10/1990"
        );
    }

    fn test_timezone() -> UtcOffset {
        UtcOffset::from_hms(0, 0, 0).expect("Valid timezone offset")
    }

    fn create_offset_datetime(
        year: i32,
        month: u8,
        day: u8,
        hour: u8,
        minute: u8,
        second: u8,
    ) -> OffsetDateTime {
        let date =
            Date::from_calendar_date(year, time::Month::try_from(month).unwrap(), day).unwrap();
        let time = Time::from_hms(hour, minute, second).unwrap();
        date.with_time(time).assume_utc() // Assume UTC for simplicity
    }
}
