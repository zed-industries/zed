use crate::{
    channel::{Channel, ChannelEvent, ChannelList, ChannelMessage},
    editor::Editor,
    theme,
    util::ResultExt,
    Settings,
};
use gpui::{
    action,
    elements::*,
    keymap::Binding,
    views::{ItemType, Select, SelectStyle},
    AppContext, Entity, ModelHandle, MutableAppContext, RenderContext, Subscription, View,
    ViewContext, ViewHandle,
};
use postage::watch;
use time::{OffsetDateTime, UtcOffset};

const MESSAGE_LOADING_THRESHOLD: usize = 50;

pub struct ChatPanel {
    channel_list: ModelHandle<ChannelList>,
    active_channel: Option<(ModelHandle<Channel>, Subscription)>,
    message_list: ListState,
    input_editor: ViewHandle<Editor>,
    channel_select: ViewHandle<Select>,
    settings: watch::Receiver<Settings>,
    local_timezone: UtcOffset,
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
        let input_editor = cx.add_view(|cx| {
            Editor::auto_height(settings.clone(), cx).with_style({
                let settings = settings.clone();
                move |_| settings.borrow().theme.chat_panel.input_editor.as_editor()
            })
        });
        let channel_select = cx.add_view(|cx| {
            let channel_list = channel_list.clone();
            Select::new(0, cx, {
                let settings = settings.clone();
                move |ix, item_type, is_hovered, cx| {
                    Self::render_channel_name(
                        &channel_list,
                        ix,
                        item_type,
                        is_hovered,
                        &settings.borrow().theme.chat_panel.channel_select,
                        cx,
                    )
                }
            })
            .with_style({
                let settings = settings.clone();
                move |_| {
                    let theme = &settings.borrow().theme.chat_panel.channel_select;
                    SelectStyle {
                        header: theme.header.container.clone(),
                        menu: theme.menu.clone(),
                    }
                }
            })
        });

        let mut message_list = ListState::new(0, Orientation::Bottom, 1000., {
            let this = cx.handle().downgrade();
            move |ix, cx| {
                let this = this.upgrade(cx).unwrap().read(cx);
                let message = this.active_channel.as_ref().unwrap().0.read(cx).message(ix);
                this.render_message(message)
            }
        });
        message_list.set_scroll_handler(|visible_range, cx| {
            if visible_range.start < MESSAGE_LOADING_THRESHOLD {
                cx.dispatch_action(LoadMoreMessages);
            }
        });

        let mut this = Self {
            channel_list,
            active_channel: Default::default(),
            message_list,
            input_editor,
            channel_select,
            settings,
            local_timezone: cx.platform().local_timezone(),
        };

        this.init_active_channel(cx);
        cx.observe(&this.channel_list, |this, _, cx| {
            this.init_active_channel(cx);
        })
        .detach();
        cx.observe(&this.channel_select, |this, channel_select, cx| {
            let selected_ix = channel_select.read(cx).selected_index();
            let selected_channel = this.channel_list.update(cx, |channel_list, cx| {
                let available_channels = channel_list.available_channels()?;
                let channel_id = available_channels.get(selected_ix)?.id;
                channel_list.get_channel(channel_id, cx)
            });
            if let Some(selected_channel) = selected_channel {
                this.set_active_channel(selected_channel, cx);
            }
        })
        .detach();

        this
    }

    fn init_active_channel(&mut self, cx: &mut ViewContext<Self>) {
        let (active_channel, channel_count) = self.channel_list.update(cx, |list, cx| {
            let channel_count;
            let mut active_channel = None;

            if let Some(available_channels) = list.available_channels() {
                channel_count = available_channels.len();
                if self.active_channel.is_none() {
                    if let Some(channel_id) = available_channels.first().map(|channel| channel.id) {
                        active_channel = list.get_channel(channel_id, cx);
                    }
                }
            } else {
                channel_count = 0;
            }

            (active_channel, channel_count)
        });

        if let Some(active_channel) = active_channel {
            self.set_active_channel(active_channel, cx);
        } else {
            self.active_channel = None;
        }

        self.channel_select.update(cx, |select, cx| {
            select.set_item_count(channel_count, cx);
        });
    }

    fn set_active_channel(&mut self, channel: ModelHandle<Channel>, cx: &mut ViewContext<Self>) {
        if self.active_channel.as_ref().map(|e| &e.0) != Some(&channel) {
            {
                let channel = channel.read(cx);
                self.message_list.reset(channel.message_count());
                let placeholder = format!("Message #{}", channel.name());
                self.input_editor.update(cx, move |editor, cx| {
                    editor.set_placeholder_text(placeholder, cx);
                });
            }
            let subscription = cx.subscribe(&channel, Self::channel_did_change);
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

    fn render_active_channel_messages(&self) -> ElementBox {
        let messages = if self.active_channel.is_some() {
            List::new(self.message_list.clone()).boxed()
        } else {
            Empty::new().boxed()
        };

        Expanded::new(1., messages).boxed()
    }

    fn render_message(&self, message: &ChannelMessage) -> ElementBox {
        let now = OffsetDateTime::now_utc();
        let settings = self.settings.borrow();
        let theme = &settings.theme.chat_panel.message;
        Container::new(
            Flex::column()
                .with_child(
                    Flex::row()
                        .with_child(
                            Container::new(
                                Label::new(
                                    message.sender.github_login.clone(),
                                    theme.sender.text.clone(),
                                )
                                .boxed(),
                            )
                            .with_style(&theme.sender.container)
                            .boxed(),
                        )
                        .with_child(
                            Container::new(
                                Label::new(
                                    format_timestamp(message.timestamp, now, self.local_timezone),
                                    theme.timestamp.text.clone(),
                                )
                                .boxed(),
                            )
                            .with_style(&theme.timestamp.container)
                            .boxed(),
                        )
                        .boxed(),
                )
                .with_child(Text::new(message.body.clone(), theme.body.clone()).boxed())
                .boxed(),
        )
        .with_style(&theme.container)
        .boxed()
    }

    fn render_input_box(&self) -> ElementBox {
        let theme = &self.settings.borrow().theme;
        Container::new(
            ConstrainedBox::new(ChildView::new(self.input_editor.id()).boxed())
                .with_max_height(100.)
                .boxed(),
        )
        .with_style(&theme.chat_panel.input_editor_container)
        .boxed()
    }

    fn render_channel_name(
        channel_list: &ModelHandle<ChannelList>,
        ix: usize,
        item_type: ItemType,
        is_hovered: bool,
        theme: &theme::ChannelSelect,
        cx: &AppContext,
    ) -> ElementBox {
        let channel = &channel_list.read(cx).available_channels().unwrap()[ix];
        let theme = match (item_type, is_hovered) {
            (ItemType::Header, _) => &theme.header,
            (ItemType::Selected, false) => &theme.active_item,
            (ItemType::Selected, true) => &theme.hovered_active_item,
            (ItemType::Unselected, false) => &theme.item,
            (ItemType::Unselected, true) => &theme.hovered_item,
        };
        Container::new(
            Flex::row()
                .with_child(
                    Container::new(Label::new("#".to_string(), theme.hash.text.clone()).boxed())
                        .with_style(&theme.hash.container)
                        .boxed(),
                )
                .with_child(Label::new(channel.name.clone(), theme.name.clone()).boxed())
                .boxed(),
        )
        .with_style(&theme.container)
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

    fn render(&mut self, _: &mut RenderContext<Self>) -> ElementBox {
        let theme = &self.settings.borrow().theme;
        Container::new(
            Flex::column()
                .with_child(
                    Container::new(ChildView::new(self.channel_select.id()).boxed())
                        .with_style(&theme.chat_panel.channel_select.container)
                        .boxed(),
                )
                .with_child(self.render_active_channel_messages())
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
