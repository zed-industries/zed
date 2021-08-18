use crate::rpc::{self, Client};
use futures::StreamExt;
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
    _receive_messages: Task<()>,
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
        let mut messages = rpc.subscribe();
        let receive_messages = cx.spawn_weak(|this, mut cx| async move {
            while let Some(message) = messages.next().await {
                if let Some(this) = this.upgrade(&cx) {
                    this.update(&mut cx, |this, cx| this.message_received(&message, cx));
                }
            }
        });

        Self {
            details,
            rpc,
            first_message_id: None,
            messages: None,
            _receive_messages: receive_messages,
        }
    }

    fn message_received(
        &mut self,
        message: &TypedEnvelope<ChannelMessageSent>,
        cx: &mut ModelContext<Self>,
    ) {
    }
}
