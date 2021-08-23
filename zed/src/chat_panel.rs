use crate::Settings;

use super::channel::{Channel, ChannelList};
use gpui::{elements::*, Entity, ModelHandle, RenderContext, View, ViewContext};
use postage::watch;

pub struct ChatPanel {
    // channel_list: ModelHandle<ChannelList>,
    // active_channel: Option<ModelHandle<Channel>>,
    messages: ListState,
}

pub enum Event {}

impl ChatPanel {
    pub fn new(settings: watch::Receiver<Settings>) -> Self {
        let settings = settings.borrow();
        let mut messages = Vec::new();
        for i in 0..1000 {
            messages.push(
                Container::new(
                    Label::new(
                        format!("This is message {}", i),
                        settings.ui_font_family,
                        settings.ui_font_size,
                    )
                    .with_style(&settings.theme.selector.label)
                    .boxed(),
                )
                .boxed(),
            );
        }
        Self {
            messages: ListState::new(messages),
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

    fn render(&self, cx: &RenderContext<Self>) -> gpui::ElementBox {
        List::new(self.messages.clone()).boxed()
    }
}
