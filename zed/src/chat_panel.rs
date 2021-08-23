use super::{
    channel::{Channel, ChannelList},
    Settings,
};
use gpui::{elements::*, Entity, ModelHandle, RenderContext, View, ViewContext};
use postage::watch;

pub struct ChatPanel {
    channel_list: ModelHandle<ChannelList>,
    active_channel: Option<ModelHandle<Channel>>,
    // active_channel_subscription: Subscription,
    messages: ListState,
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
        };
        let channel = this.channel_list.update(cx, |list, cx| {
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
            this.set_active_channel(channel);
        }
        this
    }

    pub fn set_active_channel(&mut self, channel: ModelHandle<Channel>) {
        //
    }
}

impl Entity for ChatPanel {
    type Event = Event;
}

impl View for ChatPanel {
    fn ui_name() -> &'static str {
        "ChatPanel"
    }

    fn render(&self, cx: &RenderContext<Self>) -> gpui::ElementBox {
        List::new(self.messages.clone()).boxed()
    }
}
