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
    messages: ListState,
    input_editor: ViewHandle<Editor>,
    settings: watch::Receiver<Settings>,
}

pub enum Event {}

action!(Send);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(ChatPanel::send);

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
            active_channel: None,
            messages: ListState::new(Vec::new(), Orientation::Bottom),
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
            let now = OffsetDateTime::now_utc();
            self.messages = ListState::new(
                channel
                    .read(cx)
                    .messages()
                    .cursor::<(), ()>()
                    .map(|m| self.render_message(m, now))
                    .collect(),
                Orientation::Bottom,
            );
            self.active_channel = Some((channel, subscription));
        }
    }

    fn channel_did_change(
        &mut self,
        channel: ModelHandle<Channel>,
        event: &ChannelEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            ChannelEvent::Message {
                old_range,
                new_count,
            } => {
                let now = OffsetDateTime::now_utc();
                self.messages.splice(
                    old_range.clone(),
                    channel
                        .read(cx)
                        .messages_in_range(old_range.start..(old_range.start + new_count))
                        .map(|message| self.render_message(message, now)),
                );
            }
        }
        cx.notify();
    }

    fn render_active_channel_messages(&self) -> ElementBox {
        Expanded::new(1., List::new(self.messages.clone()).boxed()).boxed()
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
                                settings.ui_font_family,
                                settings.ui_font_size,
                            )
                            .with_style(&theme.sender.label)
                            .boxed(),
                        )
                        .with_style(&theme.sender.container)
                        .boxed(),
                    )
                    .with_child(
                        Container::new(
                            Label::new(
                                format_timestamp(message.timestamp, now),
                                settings.ui_font_family,
                                settings.ui_font_size,
                            )
                            .with_style(&theme.timestamp.label)
                            .boxed(),
                        )
                        .with_style(&theme.timestamp.container)
                        .boxed(),
                    )
                    .boxed(),
            )
            .with_child(
                Text::new(
                    message.body.clone(),
                    settings.ui_font_family,
                    settings.ui_font_size,
                )
                .with_style(&theme.body)
                .boxed(),
            )
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

            channel
                .update(cx, |channel, cx| channel.send_message(body, cx))
                .log_err();
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

    fn render(&self, _: &RenderContext<Self>) -> ElementBox {
        let theme = &self.settings.borrow().theme;
        Container::new(
            Flex::column()
                .with_child(self.render_active_channel_messages())
                .with_child(self.render_input_box())
                .boxed(),
        )
        .with_style(&theme.chat_panel.container)
        .boxed()
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
        format!("{}:{}{}", hour, timestamp.minute(), part)
    } else if date.next_day() == Some(today) {
        format!("yesterday at {}:{}{}", hour, timestamp.minute(), part)
    } else {
        format!("{}/{}/{}", date.month(), date.day(), date.year())
    }
}
