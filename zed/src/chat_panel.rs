use super::channel::{Channel, ChannelList};
use gpui::{elements::*, Entity, ModelHandle, RenderContext, Subscription, View, ViewContext};

pub struct ChatPanel {
    channel_list: ModelHandle<ChannelList>,
    active_channel: Option<(ModelHandle<Channel>, Subscription)>,
    messages: ListState,
}

pub enum Event {}

impl ChatPanel {
    pub fn new(channel_list: ModelHandle<ChannelList>, cx: &mut ViewContext<Self>) -> Self {
        let mut this = Self {
            channel_list,
            messages: ListState::new(Vec::new()),
            active_channel: None,
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
                let subscription = cx.observe(&channel, Self::channel_did_change);
                self.active_channel = Some((channel, subscription));
            }
        } else {
            self.active_channel = None;
        }
    }

    fn channel_did_change(&mut self, _: ModelHandle<Channel>, cx: &mut ViewContext<Self>) {
        cx.notify();
    }
}

impl Entity for ChatPanel {
    type Event = Event;
}

impl View for ChatPanel {
    fn ui_name() -> &'static str {
        "ChatPanel"
    }

    fn render(&self, _: &RenderContext<Self>) -> gpui::ElementBox {
        List::new(self.messages.clone()).boxed()
    }
}
