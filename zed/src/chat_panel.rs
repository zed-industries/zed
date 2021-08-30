use crate::{
    channel::{Channel, ChannelEvent, ChannelList, ChannelMessage},
    editor::Editor,
    util::ResultExt,
    Settings,
};
use gpui::{
    action, elements::*, keymap::Binding, Entity, ModelHandle, MutableAppContext, RenderContext,
    Subscription, View, ViewContext, ViewHandle,
};
use postage::watch;
use time::{OffsetDateTime, UtcOffset};

pub struct ChatPanel {
    channel_list: ModelHandle<ChannelList>,
    active_channel: Option<(ModelHandle<Channel>, Subscription)>,
    message_list: ListState,
    input_editor: ViewHandle<Editor>,
    settings: watch::Receiver<Settings>,
}

pub enum Event {}

action!(Send);
action!(LoadMoreMessages);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ChatPanel::send);
    cx.add_action(ChatPanel::load_more_messages);

    cx.add_bindings(vec![Binding::new("enter", Send, Some("ChatPanel"))]);
}

impl ChatPanel {
    pub fn new(
        channel_list: ModelHandle<ChannelList>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let input_editor = cx.add_view(|cx| Editor::auto_height(settings.clone(), cx));
        let mut this = Self {
            channel_list,
            active_channel: Default::default(),
            message_list: ListState::new(0, Orientation::Bottom),
            input_editor,
            settings,
        };

        this.init_active_channel(cx);
        cx.observe(&this.channel_list, |this, _, cx| {
            this.init_active_channel(cx);
        })
        .detach();

        this
    }

    fn init_active_channel(&mut self, cx: &mut ViewContext<Self>) {
        if self.active_channel.is_none() {
            let channel = self.channel_list.update(cx, |list, cx| {
                if let Some(channel_id) = list
                    .available_channels()
                    .and_then(|channels| channels.first())
                    .map(|details| details.id)
                {
                    return list.get_channel(channel_id, cx);
                }
                None
            });
            if let Some(channel) = channel {
                self.set_active_channel(channel, cx);
            }
        } else if self.channel_list.read(cx).available_channels().is_none() {
            self.active_channel = None;
        }
    }

    fn set_active_channel(&mut self, channel: ModelHandle<Channel>, cx: &mut ViewContext<Self>) {
        if self.active_channel.as_ref().map(|e| &e.0) != Some(&channel) {
            let subscription = cx.subscribe(&channel, Self::channel_did_change);
            self.message_list =
                ListState::new(channel.read(cx).message_count(), Orientation::Bottom);
            self.message_list.set_scroll_handler(|visible_range, cx| {
                if visible_range.start < 5 {
                    cx.dispatch_action(LoadMoreMessages);
                }
            });
            self.active_channel = Some((channel, subscription));
        }
    }

    fn channel_did_change(
        &mut self,
        _: ModelHandle<Channel>,
        event: &ChannelEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            ChannelEvent::MessagesAdded {
                old_range,
                new_count,
            } => {
                self.message_list.splice(old_range.clone(), *new_count);
            }
        }
        cx.notify();
    }

    fn render_channel_name(&self, cx: &mut RenderContext<Self>) -> ElementBox {
        let settings = self.settings.borrow();
        let theme = &settings.theme.chat_panel;
        if let Some((channel, _)) = self.active_channel.as_ref() {
            let channel = channel.read(cx);
            Flex::row()
                .with_child(
                    Container::new(
                        Label::new("#".to_string(), theme.channel_name_hash.label.clone()).boxed(),
                    )
                    .with_style(&theme.channel_name_hash.container)
                    .boxed(),
                )
                .with_child(
                    Label::new(channel.name().to_string(), theme.channel_name.clone()).boxed(),
                )
                .boxed()
        } else {
            Empty::new().boxed()
        }
    }

    fn render_active_channel_messages(&self, cx: &mut RenderContext<Self>) -> ElementBox {
        let messages = if let Some((channel, _)) = self.active_channel.as_ref() {
            let channel = channel.read(cx);
            let now = OffsetDateTime::now_utc();
            List::new(self.message_list.clone(), cx, |range| {
                channel
                    .messages_in_range(range)
                    .map(|message| self.render_message(message, now))
            })
            .boxed()
        } else {
            Empty::new().boxed()
        };

        Expanded::new(1., messages).boxed()
    }

    fn render_message(&self, message: &ChannelMessage, now: OffsetDateTime) -> ElementBox {
        let settings = self.settings.borrow();
        let theme = &settings.theme.chat_panel.message;
        Flex::column()
            .with_child(
                Flex::row()
                    .with_child(
                        Container::new(
                            Label::new(
                                message.sender.github_login.clone(),
                                theme.sender.label.clone(),
                            )
                            .boxed(),
                        )
                        .with_style(&theme.sender.container)
                        .boxed(),
                    )
                    .with_child(
                        Container::new(
                            Label::new(
                                format_timestamp(message.timestamp, now),
                                theme.timestamp.label.clone(),
                            )
                            .boxed(),
                        )
                        .with_style(&theme.timestamp.container)
                        .boxed(),
                    )
                    .boxed(),
            )
            .with_child(Text::new(message.body.clone(), theme.body.clone()).boxed())
            .boxed()
    }

    fn render_input_box(&self) -> ElementBox {
        ConstrainedBox::new(ChildView::new(self.input_editor.id()).boxed())
            .with_max_height(100.)
            .boxed()
    }

    fn send(&mut self, _: &Send, cx: &mut ViewContext<Self>) {
        if let Some((channel, _)) = self.active_channel.as_ref() {
            let body = self.input_editor.update(cx, |editor, cx| {
                let body = editor.text(cx);
                editor.clear(cx);
                body
            });

            if let Some(task) = channel
                .update(cx, |channel, cx| channel.send_message(body, cx))
                .log_err()
            {
                task.detach();
            }
        }
    }

    fn load_more_messages(&mut self, _: &LoadMoreMessages, cx: &mut ViewContext<Self>) {
        if let Some((channel, _)) = self.active_channel.as_ref() {
            channel.update(cx, |channel, cx| {
                channel.load_more_messages(cx);
            })
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

    fn render(&mut self, cx: &mut RenderContext<Self>) -> ElementBox {
        let theme = &self.settings.borrow().theme;
        Container::new(
            Flex::column()
                .with_child(self.render_channel_name(cx))
                .with_child(self.render_active_channel_messages(cx))
                .with_child(self.render_input_box())
                .boxed(),
        )
        .with_style(&theme.chat_panel.container)
        .boxed()
    }

    fn on_focus(&mut self, cx: &mut ViewContext<Self>) {
        cx.focus(&self.input_editor);
    }
}

fn format_timestamp(mut timestamp: OffsetDateTime, mut now: OffsetDateTime) -> String {
    let local_offset = UtcOffset::current_local_offset().unwrap_or(UtcOffset::UTC);
    timestamp = timestamp.to_offset(local_offset);
    now = now.to_offset(local_offset);

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
