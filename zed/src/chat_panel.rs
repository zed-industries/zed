use crate::{
    channel::{Channel, ChannelEvent, ChannelList, ChannelMessage},
    Settings,
};
use gpui::{elements::*, Entity, ModelHandle, RenderContext, Subscription, View, ViewContext};
use postage::watch;

pub struct ChatPanel {
    channel_list: ModelHandle<ChannelList>,
    active_channel: Option<(ModelHandle<Channel>, Subscription)>,
    messages: ListState,
    settings: watch::Receiver<Settings>,
}

pub enum Event {}

impl ChatPanel {
    pub fn new(
        channel_list: ModelHandle<ChannelList>,
        settings: watch::Receiver<Settings>,
        cx: &mut ViewContext<Self>,
    ) -> Self {
        let mut this = Self {
            channel_list,
            messages: ListState::new(Vec::new()),
            active_channel: None,
            settings,
        };

        this.assign_active_channel(cx);
        cx.observe(&this.channel_list, |this, _, cx| {
            this.assign_active_channel(cx);
        })
        .detach();

        this
    }

    pub fn assign_active_channel(&mut self, cx: &mut ViewContext<Self>) {
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
            if self.active_channel.as_ref().map(|e| &e.0) != Some(&channel) {
                let subscription = cx.subscribe(&channel, Self::channel_did_change);
                self.messages = ListState::new(
                    channel
                        .read(cx)
                        .messages()
                        .cursor::<(), ()>()
                        .map(|m| self.render_message(m))
                        .collect(),
                );
                self.active_channel = Some((channel, subscription));
            }
        } else {
            self.active_channel = None;
        }
    }

    fn channel_did_change(
        &mut self,
        _: ModelHandle<Channel>,
        event: &ChannelEvent,
        cx: &mut ViewContext<Self>,
    ) {
        match event {
            ChannelEvent::Message { old_range, message } => {
                self.messages
                    .splice(old_range.clone(), Some(self.render_message(message)));
            }
        }
        cx.notify();
    }

    fn render_active_channel_messages(&self) -> ElementBox {
        Expanded::new(0.8, List::new(self.messages.clone()).boxed()).boxed()
    }

    fn render_message(&self, message: &ChannelMessage) -> ElementBox {
        let settings = self.settings.borrow();
        Flex::column()
            .with_child(
                Label::new(
                    message.body.clone(),
                    settings.ui_font_family,
                    settings.ui_font_size,
                )
                .boxed(),
            )
            .boxed()
    }

    fn render_input_box(&self) -> ElementBox {
        Empty::new().boxed()
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
        Flex::column()
            .with_child(self.render_active_channel_messages())
            .with_child(self.render_input_box())
            .boxed()
    }
}
