use crate::rpc::Client;
use gpui::{Entity, ModelHandle, WeakModelHandle};
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};

pub struct ChannelList {
    channels: HashMap<u64, WeakModelHandle<Channel>>,
    rpc: Arc<Client>,
}

pub struct Channel {
    id: u64,
    name: String,
    first_message_id: Option<u64>,
    messages: Option<VecDeque<ChannelMessage>>,
    rpc: Arc<Client>,
}

pub struct ChannelMessage {
    id: u64,
}

enum Event {}

impl Entity for ChannelList {
    type Event = Event;
}

impl ChannelList {}
