use crate::{
    rpc::{self, Client},
    util::log_async_errors,
};
use anyhow::{anyhow, Context, Result};
use gpui::{
    AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext, WeakModelHandle,
};
use std::{
    collections::{hash_map, HashMap, VecDeque},
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

#[derive(Clone, Debug, PartialEq)]
pub struct ChannelDetails {
    pub id: u64,
    pub name: String,
}

pub struct Channel {
    details: ChannelDetails,
    first_message_id: Option<u64>,
    messages: VecDeque<ChannelMessage>,
    pending_messages: Vec<PendingChannelMessage>,
    next_local_message_id: u64,
    rpc: Arc<Client>,
    _subscription: rpc::Subscription,
}

pub struct ChannelMessage {
    pub id: u64,
    pub sender_id: u64,
    pub body: String,
}

pub struct PendingChannelMessage {
    pub body: String,
    local_id: u64,
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

    pub fn get_channel(
        &mut self,
        id: u64,
        cx: &mut MutableAppContext,
    ) -> Option<ModelHandle<Channel>> {
        match self.channels.entry(id) {
            hash_map::Entry::Occupied(entry) => entry.get().upgrade(cx),
            hash_map::Entry::Vacant(entry) => {
                if let Some(details) = self
                    .available_channels
                    .iter()
                    .find(|details| details.id == id)
                {
                    let rpc = self.rpc.clone();
                    let channel = cx.add_model(|cx| Channel::new(details.clone(), rpc, cx));
                    entry.insert(channel.downgrade());
                    Some(channel)
                } else {
                    None
                }
            }
        }
    }
}

impl Entity for Channel {
    type Event = ();

    // TODO: Implement the server side of leaving a channel
    fn release(&mut self, cx: &mut MutableAppContext) {
        let rpc = self.rpc.clone();
        let channel_id = self.details.id;
        cx.foreground()
            .spawn(async move {
                if let Err(error) = rpc.send(proto::LeaveChannel { channel_id }).await {
                    log::error!("error leaving channel: {}", error);
                };
            })
            .detach()
    }
}

impl Channel {
    pub fn new(details: ChannelDetails, rpc: Arc<Client>, cx: &mut ModelContext<Self>) -> Self {
        let _subscription = rpc.subscribe_from_model(details.id, cx, Self::handle_message_sent);

        {
            let rpc = rpc.clone();
            let channel_id = details.id;
            cx.spawn(|channel, mut cx| async move {
                match rpc.request(proto::JoinChannel { channel_id }).await {
                    Ok(response) => channel.update(&mut cx, |channel, cx| {
                        channel.messages = response.messages.into_iter().map(Into::into).collect();
                        cx.notify();
                    }),
                    Err(error) => log::error!("error joining channel: {}", error),
                }
            })
            .detach();
        }

        Self {
            details,
            rpc,
            first_message_id: None,
            messages: Default::default(),
            pending_messages: Default::default(),
            next_local_message_id: 0,
            _subscription,
        }
    }

    pub fn send_message(&mut self, body: String, cx: &mut ModelContext<Self>) -> Result<()> {
        let channel_id = self.details.id;
        let current_user_id = self.rpc.user_id().ok_or_else(|| anyhow!("not logged in"))?;
        let local_id = self.next_local_message_id;
        self.next_local_message_id += 1;
        self.pending_messages.push(PendingChannelMessage {
            local_id,
            body: body.clone(),
        });
        let rpc = self.rpc.clone();
        cx.spawn(|this, mut cx| {
            log_async_errors(async move {
                let request = rpc.request(proto::SendChannelMessage { channel_id, body });
                let response = request.await?;
                this.update(&mut cx, |this, cx| {
                    if let Ok(i) = this
                        .pending_messages
                        .binary_search_by_key(&local_id, |msg| msg.local_id)
                    {
                        let body = this.pending_messages.remove(i).body;
                        this.messages.push_back(ChannelMessage {
                            id: response.message_id,
                            sender_id: current_user_id,
                            body,
                        });
                        cx.notify();
                    }
                });
                Ok(())
            })
        })
        .detach();
        Ok(())
    }

    pub fn messages(&self) -> &VecDeque<ChannelMessage> {
        &self.messages
    }

    pub fn pending_messages(&self) -> &[PendingChannelMessage] {
        &self.pending_messages
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

impl From<proto::ChannelMessage> for ChannelMessage {
    fn from(message: proto::ChannelMessage) -> Self {
        ChannelMessage {
            id: message.id,
            sender_id: message.sender_id,
            body: message.body,
        }
    }
}
