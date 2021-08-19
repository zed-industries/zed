use crate::rpc::{self, Client};
use anyhow::{Context, Result};
use gpui::{AppContext, AsyncAppContext, Entity, ModelContext, ModelHandle, WeakModelHandle};
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use zrpc::{
    proto::{self, ChannelMessageSent},
    TypedEnvelope,
};

pub struct ChannelList {
    available_channels: Vec<ChannelDetails>,
    channels: HashMap<u64, WeakModelHandle<Channel>>,
    rpc: Arc<Client>,
}

#[derive(Debug, PartialEq)]
pub struct ChannelDetails {
    pub id: u64,
    pub name: String,
}

pub struct Channel {
    details: ChannelDetails,
    first_message_id: Option<u64>,
    messages: Option<VecDeque<ChannelMessage>>,
    rpc: Arc<Client>,
    _subscription: rpc::Subscription,
}

pub struct ChannelMessage {
    id: u64,
}
pub enum Event {}

impl Entity for ChannelList {
    type Event = Event;
}

impl ChannelList {
    pub async fn new(rpc: Arc<rpc::Client>, cx: &mut AsyncAppContext) -> Result<ModelHandle<Self>> {
        let response = rpc
            .request(proto::GetChannels {})
            .await
            .context("failed to fetch available channels")?;

        Ok(cx.add_model(|_| Self {
            available_channels: response.channels.into_iter().map(Into::into).collect(),
            channels: Default::default(),
            rpc,
        }))
    }

    pub fn available_channels(&self) -> &[ChannelDetails] {
        &self.available_channels
    }

    pub fn get_channel(&self, id: u64, cx: &AppContext) -> Option<ModelHandle<Channel>> {
        self.channels
            .get(&id)
            .cloned()
            .and_then(|handle| handle.upgrade(cx))
    }
}

impl Entity for Channel {
    type Event = ();
}

impl Channel {
    pub fn new(details: ChannelDetails, rpc: Arc<Client>, cx: &mut ModelContext<Self>) -> Self {
        let _subscription = rpc.subscribe_from_model(details.id, cx, Self::handle_message_sent);

        Self {
            details,
            rpc,
            first_message_id: None,
            messages: None,
            _subscription,
        }
    }

    fn handle_message_sent(
        &mut self,
        message: TypedEnvelope<ChannelMessageSent>,
        rpc: Arc<rpc::Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        Ok(())
    }
}

impl From<proto::Channel> for ChannelDetails {
    fn from(message: proto::Channel) -> Self {
        Self {
            id: message.id,
            name: message.name,
        }
    }
}
