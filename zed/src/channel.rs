use crate::rpc::{self, Client};
use anyhow::{anyhow, Result};
use futures::StreamExt;
use gpui::{
    AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext, Task, WeakModelHandle,
};
use std::{
    collections::{HashMap, VecDeque},
    sync::Arc,
};
use zrpc::{proto::ChannelMessageSent, ForegroundRouter, Router, TypedEnvelope};

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
enum Event {}

impl Entity for ChannelList {
    type Event = Event;
}

impl ChannelList {
    fn new(
        rpc: Arc<rpc::Client>,
        router: &mut ForegroundRouter,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        // Subscribe to messages.
        let this = cx.handle().downgrade();

        // rpc.on_message(
        //     router,
        //     |envelope, rpc, cx: &mut AsyncAppContext| async move {
        //         cx.update(|cx| {
        //             if let Some(this) = this.upgrade(cx) {
        //                 this.update(cx, |this, cx| this.receive_message(envelope, cx))
        //             } else {
        //                 Err(anyhow!("can't upgrade ChannelList handle"))
        //             }
        //         })
        //     },
        //     cx,
        // );

        Self {
            available_channels: Default::default(),
            channels: Default::default(),
            rpc,
        }
    }

    fn receive_message(
        &mut self,
        envelope: TypedEnvelope<ChannelMessageSent>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        Ok(())
    }
}

impl Entity for Channel {
    type Event = ();
}

impl Channel {
    pub fn new(details: ChannelDetails, rpc: Arc<Client>, cx: &mut ModelContext<Self>) -> Self {
        let messages = rpc.subscribe();
        let receive_messages = cx.spawn_weak(|this, cx| async move {
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
