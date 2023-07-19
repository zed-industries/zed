mod channels_panel;
mod channels_panel_settings;

pub use channels_panel::*;
use gpui::{AppContext, Entity};

use std::sync::Arc;

use client::Client;

pub fn init(client: Arc<Client>, cx: &mut AppContext) {
    let channels = cx.add_model(|cx| Channels::new(client, cx));
    cx.set_global(channels);
    channels_panel::init(cx);
}

#[derive(Debug, Clone)]
struct Channel {
    id: u64,
    name: String,
    sub_channels: Vec<Channel>,
    _room: Option<()>,
}

impl Channel {
    fn new(id: u64, name: impl AsRef<str>, members: Vec<Channel>) -> Channel {
        Channel {
            name: name.as_ref().to_string(),
            id,
            sub_channels: members,
            _room: None,
        }
    }

    fn members(&self) -> &[Channel] {
        &self.sub_channels
    }

    fn name(&self) -> &str {
        &self.name
    }
}

struct Channels {
    channels: Vec<Channel>,
}

impl Channels {
    fn channels(&self) -> Vec<Channel> {
        self.channels.clone()
    }
}

enum ChannelEvents {}

impl Entity for Channels {
    type Event = ChannelEvents;
}

impl Channels {
    fn new(_client: Arc<Client>, _cx: &mut AppContext) -> Self {
        //TODO: Subscribe to channel updates from the server
        Channels {
            channels: vec![Channel::new(
                0,
                "Zed Industries",
                vec![
                    Channel::new(1, "#general", Vec::new()),
                    Channel::new(2, "#admiral", Vec::new()),
                    Channel::new(3, "#livestreaming", vec![]),
                    Channel::new(4, "#crdb", Vec::new()),
                    Channel::new(5, "#crdb-1", Vec::new()),
                    Channel::new(6, "#crdb-2", Vec::new()),
                    Channel::new(7, "#crdb-3", vec![]),
                    Channel::new(8, "#crdb-4", Vec::new()),
                    Channel::new(9, "#crdb-1", Vec::new()),
                    Channel::new(10, "#crdb-1", Vec::new()),
                    Channel::new(11, "#crdb-1", Vec::new()),
                    Channel::new(12, "#crdb-1", vec![]),
                    Channel::new(13, "#crdb-1", Vec::new()),
                    Channel::new(14, "#crdb-1", Vec::new()),
                    Channel::new(15, "#crdb-1", Vec::new()),
                    Channel::new(16, "#crdb-1", Vec::new()),
                    Channel::new(17, "#crdb", vec![]),
                ],
            ),
            Channel::new(
                18,
                "CRDB Consulting",
                vec![
                    Channel::new(19, "#crdb 😭", Vec::new()),
                    Channel::new(20, "#crdb 😌", Vec::new()),
                    Channel::new(21, "#crdb 🦀", vec![]),
                    Channel::new(22, "#crdb 😤", Vec::new()),
                    Channel::new(23, "#crdb 😤", Vec::new()),
                    Channel::new(24, "#crdb 😤", Vec::new()),
                    Channel::new(25, "#crdb 😤", vec![]),
                    Channel::new(26, "#crdb 😤", Vec::new()),
                ],
            )],
        }
    }
}
