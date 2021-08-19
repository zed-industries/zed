use crate::rpc::{self, Client};
use anyhow::Result;
use gpui::{Entity, ModelContext, Task, WeakModelHandle};
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use zrpc::{proto::ChannelMessageSent, TypedEnvelope};

pub struct ChannelList {
    available_channels: Vec<ChannelDetails>,
    channels: HashMap<u64, WeakModelHandle<Channel>>,
    rpc: Arc<Client>,
}

pub struct ChannelDetails {
    id: u64,
    name: String,
}
pub struct Channel {
    details: ChannelDetails,
    first_message_id: Option<u64>,
    messages: Option<VecDeque<ChannelMessage>>,
    rpc: Arc<Client>,
    _message_handler: Task<()>,
}

pub struct ChannelMessage {
    id: u64,
}
pub enum Event {}

impl Entity for ChannelList {
    type Event = Event;
}

impl ChannelList {
    fn new(rpc: Arc<rpc::Client>) -> Self {
        Self {
            available_channels: Default::default(),
            channels: Default::default(),
            rpc,
        }
    }
}

impl Entity for Channel {
    type Event = ();
}

impl Channel {
    pub fn new(details: ChannelDetails, rpc: Arc<Client>, cx: &mut ModelContext<Self>) -> Self {
        let _message_handler = rpc.subscribe_from_model(details.id, cx, Self::handle_message_sent);

        Self {
            details,
            rpc,
            first_message_id: None,
            messages: None,
            _message_handler,
        }
    }

    fn handle_message_sent(
        &mut self,
        message: &TypedEnvelope<ChannelMessageSent>,
        rpc: rpc::Client,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        Ok(())
    }
}
