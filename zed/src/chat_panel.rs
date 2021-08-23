use super::channel::{Channel, ChannelList};
use gpui::{elements::*, Entity, ModelHandle, View};

pub struct ChatPanel {
    channel_list: ModelHandle<ChannelList>,
    active_channel: Option<ModelHandle<Channel>>,
    messages: ListState,
}

pub enum Event {}

impl Entity for ChatPanel {
    type Event = Event;
}

impl View for ChatPanel {
    fn ui_name() -> &'static str {
        "ChatPanel"
    }

    fn render(&self, cx: &gpui::RenderContext<'_, Self>) -> gpui::ElementBox {
        todo!()
    }
}
