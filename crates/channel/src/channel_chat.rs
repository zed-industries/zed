use crate::{Channel, ChannelStore};
use anyhow::{anyhow, Result};
use client::{
    proto,
    user::{User, UserStore},
    ChannelId, Client, Subscription, TypedEnvelope, UserId,
};
use collections::HashSet;
use futures::lock::Mutex;
use gpui::{
    AppContext, AsyncAppContext, Context, EventEmitter, Model, ModelContext, Task, WeakModel,
};
use rand::prelude::*;
use std::{
    ops::{ControlFlow, Range},
    sync::Arc,
};
use sum_tree::{Bias, SumTree};
use time::OffsetDateTime;
use util::{post_inc, ResultExt as _, TryFutureExt};

pub struct ChannelChat {
    pub channel_id: ChannelId,
    messages: SumTree<ChannelMessage>,
    acknowledged_message_ids: HashSet<u64>,
    channel_store: Model<ChannelStore>,
    loaded_all_messages: bool,
    last_acknowledged_id: Option<u64>,
    next_pending_message_id: usize,
    first_loaded_message_id: Option<u64>,
    user_store: Model<UserStore>,
    rpc: Arc<Client>,
    outgoing_messages_lock: Arc<Mutex<()>>,
    rng: StdRng,
    _subscription: Subscription,
}

#[derive(Debug, PartialEq, Eq)]
pub struct MessageParams {
    pub text: String,
    pub mentions: Vec<(Range<usize>, UserId)>,
    pub reply_to_message_id: Option<u64>,
}

#[derive(Clone, Debug)]
pub struct ChannelMessage {
    pub id: ChannelMessageId,
    pub body: String,
    pub timestamp: OffsetDateTime,
    pub sender: Arc<User>,
    pub nonce: u128,
    pub mentions: Vec<(Range<usize>, UserId)>,
    pub reply_to_message_id: Option<u64>,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ChannelMessageId {
    Saved(u64),
    Pending(usize),
}

impl Into<Option<u64>> for ChannelMessageId {
    fn into(self) -> Option<u64> {
        match self {
            ChannelMessageId::Saved(id) => Some(id),
            ChannelMessageId::Pending(_) => None,
        }
    }
}

#[derive(Clone, Debug, Default)]
pub struct ChannelMessageSummary {
    max_id: ChannelMessageId,
    count: usize,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Count(usize);

#[derive(Clone, Debug, PartialEq)]
pub enum ChannelChatEvent {
    MessagesUpdated {
        old_range: Range<usize>,
        new_count: usize,
    },
    NewMessage {
        channel_id: ChannelId,
        message_id: u64,
    },
}

impl EventEmitter<ChannelChatEvent> for ChannelChat {}
pub fn init(client: &Arc<Client>) {
    client.add_model_message_handler(ChannelChat::handle_message_sent);
    client.add_model_message_handler(ChannelChat::handle_message_removed);
}

impl ChannelChat {
    pub async fn new(
        channel: Arc<Channel>,
        channel_store: Model<ChannelStore>,
        user_store: Model<UserStore>,
        client: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<Model<Self>> {
        let channel_id = channel.id;
        let subscription = client.subscribe_to_entity(channel_id.0).unwrap();

        let response = client
            .request(proto::JoinChannelChat {
                channel_id: channel_id.0,
            })
            .await?;

        let handle = cx.new_model(|cx| {
            cx.on_release(Self::release).detach();
            Self {
                channel_id: channel.id,
                user_store: user_store.clone(),
                channel_store,
                rpc: client.clone(),
                outgoing_messages_lock: Default::default(),
                messages: Default::default(),
                acknowledged_message_ids: Default::default(),
                loaded_all_messages: false,
                next_pending_message_id: 0,
                last_acknowledged_id: None,
                rng: StdRng::from_entropy(),
                first_loaded_message_id: None,
                _subscription: subscription.set_model(&cx.handle(), &mut cx.to_async()),
            }
        })?;
        Self::handle_loaded_messages(
            handle.downgrade(),
            user_store,
            client,
            response.messages,
            response.done,
            &mut cx,
        )
        .await?;
        Ok(handle)
    }

    fn release(&mut self, _: &mut AppContext) {
        self.rpc
            .send(proto::LeaveChannelChat {
                channel_id: self.channel_id.0,
            })
            .log_err();
    }

    pub fn channel(&self, cx: &AppContext) -> Option<Arc<Channel>> {
        self.channel_store
            .read(cx)
            .channel_for_id(self.channel_id)
            .cloned()
    }

    pub fn client(&self) -> &Arc<Client> {
        &self.rpc
    }

    pub fn send_message(
        &mut self,
        message: MessageParams,
        cx: &mut ModelContext<Self>,
    ) -> Result<Task<Result<u64>>> {
        if message.text.trim().is_empty() {
            Err(anyhow!("message body can't be empty"))?;
        }

        let current_user = self
            .user_store
            .read(cx)
            .current_user()
            .ok_or_else(|| anyhow!("current_user is not present"))?;

        let channel_id = self.channel_id;
        let pending_id = ChannelMessageId::Pending(post_inc(&mut self.next_pending_message_id));
        let nonce = self.rng.gen();
        self.insert_messages(
            SumTree::from_item(
                ChannelMessage {
                    id: pending_id,
                    body: message.text.clone(),
                    sender: current_user,
                    timestamp: OffsetDateTime::now_utc(),
                    mentions: message.mentions.clone(),
                    nonce,
                    reply_to_message_id: message.reply_to_message_id,
                },
                &(),
            ),
            cx,
        );
        let user_store = self.user_store.clone();
        let rpc = self.rpc.clone();
        let outgoing_messages_lock = self.outgoing_messages_lock.clone();

        // todo - handle messages that fail to send (e.g. >1024 chars)
        Ok(cx.spawn(move |this, mut cx| async move {
            let outgoing_message_guard = outgoing_messages_lock.lock().await;
            let request = rpc.request(proto::SendChannelMessage {
                channel_id: channel_id.0,
                body: message.text,
                nonce: Some(nonce.into()),
                mentions: mentions_to_proto(&message.mentions),
                reply_to_message_id: message.reply_to_message_id,
            });
            let response = request.await?;
            drop(outgoing_message_guard);
            let response = response.message.ok_or_else(|| anyhow!("invalid message"))?;
            let id = response.id;
            let message = ChannelMessage::from_proto(response, &user_store, &mut cx).await?;
            this.update(&mut cx, |this, cx| {
                this.insert_messages(SumTree::from_item(message, &()), cx);
            })?;
            Ok(id)
        }))
    }

    pub fn remove_message(&mut self, id: u64, cx: &mut ModelContext<Self>) -> Task<Result<()>> {
        let response = self.rpc.request(proto::RemoveChannelMessage {
            channel_id: self.channel_id.0,
            message_id: id,
        });
        cx.spawn(move |this, mut cx| async move {
            response.await?;
            this.update(&mut cx, |this, cx| {
                this.message_removed(id, cx);
            })?;
            Ok(())
        })
    }

    pub fn load_more_messages(&mut self, cx: &mut ModelContext<Self>) -> Option<Task<Option<()>>> {
        if self.loaded_all_messages {
            return None;
        }

        let rpc = self.rpc.clone();
        let user_store = self.user_store.clone();
        let channel_id = self.channel_id;
        let before_message_id = self.first_loaded_message_id()?;
        Some(cx.spawn(move |this, mut cx| {
            async move {
                let response = rpc
                    .request(proto::GetChannelMessages {
                        channel_id: channel_id.0,
                        before_message_id,
                    })
                    .await?;
                Self::handle_loaded_messages(
                    this,
                    user_store,
                    rpc,
                    response.messages,
                    response.done,
                    &mut cx,
                )
                .await?;

                anyhow::Ok(())
            }
            .log_err()
        }))
    }

    pub fn first_loaded_message_id(&mut self) -> Option<u64> {
        self.first_loaded_message_id
    }

    /// Load a message by its id, if it's already stored locally.
    pub fn find_loaded_message(&self, id: u64) -> Option<&ChannelMessage> {
        self.messages.iter().find(|message| match message.id {
            ChannelMessageId::Saved(message_id) => message_id == id,
            ChannelMessageId::Pending(_) => false,
        })
    }

    /// Load all of the chat messages since a certain message id.
    ///
    /// For now, we always maintain a suffix of the channel's messages.
    pub async fn load_history_since_message(
        chat: Model<Self>,
        message_id: u64,
        mut cx: AsyncAppContext,
    ) -> Option<usize> {
        loop {
            let step = chat
                .update(&mut cx, |chat, cx| {
                    if let Some(first_id) = chat.first_loaded_message_id() {
                        if first_id <= message_id {
                            let mut cursor = chat.messages.cursor::<(ChannelMessageId, Count)>();
                            let message_id = ChannelMessageId::Saved(message_id);
                            cursor.seek(&message_id, Bias::Left, &());
                            return ControlFlow::Break(
                                if cursor
                                    .item()
                                    .map_or(false, |message| message.id == message_id)
                                {
                                    Some(cursor.start().1 .0)
                                } else {
                                    None
                                },
                            );
                        }
                    }
                    ControlFlow::Continue(chat.load_more_messages(cx))
                })
                .log_err()?;
            match step {
                ControlFlow::Break(ix) => return ix,
                ControlFlow::Continue(task) => task?.await?,
            }
        }
    }

    pub fn acknowledge_last_message(&mut self, cx: &mut ModelContext<Self>) {
        if let ChannelMessageId::Saved(latest_message_id) = self.messages.summary().max_id {
            if self
                .last_acknowledged_id
                .map_or(true, |acknowledged_id| acknowledged_id < latest_message_id)
            {
                self.rpc
                    .send(proto::AckChannelMessage {
                        channel_id: self.channel_id.0,
                        message_id: latest_message_id,
                    })
                    .ok();
                self.last_acknowledged_id = Some(latest_message_id);
                self.channel_store.update(cx, |store, cx| {
                    store.acknowledge_message_id(self.channel_id, latest_message_id, cx);
                });
            }
        }
    }

    async fn handle_loaded_messages(
        this: WeakModel<Self>,
        user_store: Model<UserStore>,
        rpc: Arc<Client>,
        proto_messages: Vec<proto::ChannelMessage>,
        loaded_all_messages: bool,
        cx: &mut AsyncAppContext,
    ) -> Result<()> {
        let loaded_messages = messages_from_proto(proto_messages, &user_store, cx).await?;

        let first_loaded_message_id = loaded_messages.first().map(|m| m.id);
        let loaded_message_ids = this.update(cx, |this, _| {
            let mut loaded_message_ids: HashSet<u64> = HashSet::default();
            for message in loaded_messages.iter() {
                if let Some(saved_message_id) = message.id.into() {
                    loaded_message_ids.insert(saved_message_id);
                }
            }
            for message in this.messages.iter() {
                if let Some(saved_message_id) = message.id.into() {
                    loaded_message_ids.insert(saved_message_id);
                }
            }
            loaded_message_ids
        })?;

        let missing_ancestors = loaded_messages
            .iter()
            .filter_map(|message| {
                if let Some(ancestor_id) = message.reply_to_message_id {
                    if !loaded_message_ids.contains(&ancestor_id) {
                        return Some(ancestor_id);
                    }
                }
                None
            })
            .collect::<Vec<_>>();

        let loaded_ancestors = if missing_ancestors.is_empty() {
            None
        } else {
            let response = rpc
                .request(proto::GetChannelMessagesById {
                    message_ids: missing_ancestors,
                })
                .await?;
            Some(messages_from_proto(response.messages, &user_store, cx).await?)
        };
        this.update(cx, |this, cx| {
            this.first_loaded_message_id = first_loaded_message_id.and_then(|msg_id| msg_id.into());
            this.loaded_all_messages = loaded_all_messages;
            this.insert_messages(loaded_messages, cx);
            if let Some(loaded_ancestors) = loaded_ancestors {
                this.insert_messages(loaded_ancestors, cx);
            }
        })?;

        Ok(())
    }

    pub fn rejoin(&mut self, cx: &mut ModelContext<Self>) {
        let user_store = self.user_store.clone();
        let rpc = self.rpc.clone();
        let channel_id = self.channel_id;
        cx.spawn(move |this, mut cx| {
            async move {
                let response = rpc
                    .request(proto::JoinChannelChat {
                        channel_id: channel_id.0,
                    })
                    .await?;
                Self::handle_loaded_messages(
                    this.clone(),
                    user_store.clone(),
                    rpc.clone(),
                    response.messages,
                    response.done,
                    &mut cx,
                )
                .await?;

                let pending_messages = this.update(&mut cx, |this, _| {
                    this.pending_messages().cloned().collect::<Vec<_>>()
                })?;

                for pending_message in pending_messages {
                    let request = rpc.request(proto::SendChannelMessage {
                        channel_id: channel_id.0,
                        body: pending_message.body,
                        mentions: mentions_to_proto(&pending_message.mentions),
                        nonce: Some(pending_message.nonce.into()),
                        reply_to_message_id: pending_message.reply_to_message_id,
                    });
                    let response = request.await?;
                    let message = ChannelMessage::from_proto(
                        response.message.ok_or_else(|| anyhow!("invalid message"))?,
                        &user_store,
                        &mut cx,
                    )
                    .await?;
                    this.update(&mut cx, |this, cx| {
                        this.insert_messages(SumTree::from_item(message, &()), cx);
                    })?;
                }

                anyhow::Ok(())
            }
            .log_err()
        })
        .detach();
    }

    pub fn message_count(&self) -> usize {
        self.messages.summary().count
    }

    pub fn messages(&self) -> &SumTree<ChannelMessage> {
        &self.messages
    }

    pub fn message(&self, ix: usize) -> &ChannelMessage {
        let mut cursor = self.messages.cursor::<Count>();
        cursor.seek(&Count(ix), Bias::Right, &());
        cursor.item().unwrap()
    }

    pub fn acknowledge_message(&mut self, id: u64) {
        if self.acknowledged_message_ids.insert(id) {
            self.rpc
                .send(proto::AckChannelMessage {
                    channel_id: self.channel_id.0,
                    message_id: id,
                })
                .ok();
        }
    }

    pub fn messages_in_range(&self, range: Range<usize>) -> impl Iterator<Item = &ChannelMessage> {
        let mut cursor = self.messages.cursor::<Count>();
        cursor.seek(&Count(range.start), Bias::Right, &());
        cursor.take(range.len())
    }

    pub fn pending_messages(&self) -> impl Iterator<Item = &ChannelMessage> {
        let mut cursor = self.messages.cursor::<ChannelMessageId>();
        cursor.seek(&ChannelMessageId::Pending(0), Bias::Left, &());
        cursor
    }

    async fn handle_message_sent(
        this: Model<Self>,
        message: TypedEnvelope<proto::ChannelMessageSent>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let user_store = this.update(&mut cx, |this, _| this.user_store.clone())?;
        let message = message
            .payload
            .message
            .ok_or_else(|| anyhow!("empty message"))?;
        let message_id = message.id;

        let message = ChannelMessage::from_proto(message, &user_store, &mut cx).await?;
        this.update(&mut cx, |this, cx| {
            this.insert_messages(SumTree::from_item(message, &()), cx);
            cx.emit(ChannelChatEvent::NewMessage {
                channel_id: this.channel_id,
                message_id,
            })
        })?;

        Ok(())
    }

    async fn handle_message_removed(
        this: Model<Self>,
        message: TypedEnvelope<proto::RemoveChannelMessage>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        this.update(&mut cx, |this, cx| {
            this.message_removed(message.payload.message_id, cx)
        })?;
        Ok(())
    }

    fn insert_messages(&mut self, messages: SumTree<ChannelMessage>, cx: &mut ModelContext<Self>) {
        if let Some((first_message, last_message)) = messages.first().zip(messages.last()) {
            let nonces = messages
                .cursor::<()>()
                .map(|m| m.nonce)
                .collect::<HashSet<_>>();

            let mut old_cursor = self.messages.cursor::<(ChannelMessageId, Count)>();
            let mut new_messages = old_cursor.slice(&first_message.id, Bias::Left, &());
            let start_ix = old_cursor.start().1 .0;
            let removed_messages = old_cursor.slice(&last_message.id, Bias::Right, &());
            let removed_count = removed_messages.summary().count;
            let new_count = messages.summary().count;
            let end_ix = start_ix + removed_count;

            new_messages.append(messages, &());

            let mut ranges = Vec::<Range<usize>>::new();
            if new_messages.last().unwrap().is_pending() {
                new_messages.append(old_cursor.suffix(&()), &());
            } else {
                new_messages.append(
                    old_cursor.slice(&ChannelMessageId::Pending(0), Bias::Left, &()),
                    &(),
                );

                while let Some(message) = old_cursor.item() {
                    let message_ix = old_cursor.start().1 .0;
                    if nonces.contains(&message.nonce) {
                        if ranges.last().map_or(false, |r| r.end == message_ix) {
                            ranges.last_mut().unwrap().end += 1;
                        } else {
                            ranges.push(message_ix..message_ix + 1);
                        }
                    } else {
                        new_messages.push(message.clone(), &());
                    }
                    old_cursor.next(&());
                }
            }

            drop(old_cursor);
            self.messages = new_messages;

            for range in ranges.into_iter().rev() {
                cx.emit(ChannelChatEvent::MessagesUpdated {
                    old_range: range,
                    new_count: 0,
                });
            }
            cx.emit(ChannelChatEvent::MessagesUpdated {
                old_range: start_ix..end_ix,
                new_count,
            });

            cx.notify();
        }
    }

    fn message_removed(&mut self, id: u64, cx: &mut ModelContext<Self>) {
        let mut cursor = self.messages.cursor::<ChannelMessageId>();
        let mut messages = cursor.slice(&ChannelMessageId::Saved(id), Bias::Left, &());
        if let Some(item) = cursor.item() {
            if item.id == ChannelMessageId::Saved(id) {
                let ix = messages.summary().count;
                cursor.next(&());
                messages.append(cursor.suffix(&()), &());
                drop(cursor);
                self.messages = messages;
                cx.emit(ChannelChatEvent::MessagesUpdated {
                    old_range: ix..ix + 1,
                    new_count: 0,
                });
            }
        }
    }
}

async fn messages_from_proto(
    proto_messages: Vec<proto::ChannelMessage>,
    user_store: &Model<UserStore>,
    cx: &mut AsyncAppContext,
) -> Result<SumTree<ChannelMessage>> {
    let messages = ChannelMessage::from_proto_vec(proto_messages, user_store, cx).await?;
    let mut result = SumTree::new();
    result.extend(messages, &());
    Ok(result)
}

impl ChannelMessage {
    pub async fn from_proto(
        message: proto::ChannelMessage,
        user_store: &Model<UserStore>,
        cx: &mut AsyncAppContext,
    ) -> Result<Self> {
        let sender = user_store
            .update(cx, |user_store, cx| {
                user_store.get_user(message.sender_id, cx)
            })?
            .await?;
        Ok(ChannelMessage {
            id: ChannelMessageId::Saved(message.id),
            body: message.body,
            mentions: message
                .mentions
                .into_iter()
                .filter_map(|mention| {
                    let range = mention.range?;
                    Some((range.start as usize..range.end as usize, mention.user_id))
                })
                .collect(),
            timestamp: OffsetDateTime::from_unix_timestamp(message.timestamp as i64)?,
            sender,
            nonce: message
                .nonce
                .ok_or_else(|| anyhow!("nonce is required"))?
                .into(),
            reply_to_message_id: message.reply_to_message_id,
        })
    }

    pub fn is_pending(&self) -> bool {
        matches!(self.id, ChannelMessageId::Pending(_))
    }

    pub async fn from_proto_vec(
        proto_messages: Vec<proto::ChannelMessage>,
        user_store: &Model<UserStore>,
        cx: &mut AsyncAppContext,
    ) -> Result<Vec<Self>> {
        let unique_user_ids = proto_messages
            .iter()
            .map(|m| m.sender_id)
            .collect::<HashSet<_>>()
            .into_iter()
            .collect();
        user_store
            .update(cx, |user_store, cx| {
                user_store.get_users(unique_user_ids, cx)
            })?
            .await?;

        let mut messages = Vec::with_capacity(proto_messages.len());
        for message in proto_messages {
            messages.push(ChannelMessage::from_proto(message, user_store, cx).await?);
        }
        Ok(messages)
    }
}

pub fn mentions_to_proto(mentions: &[(Range<usize>, UserId)]) -> Vec<proto::ChatMention> {
    mentions
        .iter()
        .map(|(range, user_id)| proto::ChatMention {
            range: Some(proto::Range {
                start: range.start as u64,
                end: range.end as u64,
            }),
            user_id: *user_id,
        })
        .collect()
}

impl sum_tree::Item for ChannelMessage {
    type Summary = ChannelMessageSummary;

    fn summary(&self) -> Self::Summary {
        ChannelMessageSummary {
            max_id: self.id,
            count: 1,
        }
    }
}

impl Default for ChannelMessageId {
    fn default() -> Self {
        Self::Saved(0)
    }
}

impl sum_tree::Summary for ChannelMessageSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        self.max_id = summary.max_id;
        self.count += summary.count;
    }
}

impl<'a> sum_tree::Dimension<'a, ChannelMessageSummary> for ChannelMessageId {
    fn add_summary(&mut self, summary: &'a ChannelMessageSummary, _: &()) {
        debug_assert!(summary.max_id > *self);
        *self = summary.max_id;
    }
}

impl<'a> sum_tree::Dimension<'a, ChannelMessageSummary> for Count {
    fn add_summary(&mut self, summary: &'a ChannelMessageSummary, _: &()) {
        self.0 += summary.count;
    }
}

impl<'a> From<&'a str> for MessageParams {
    fn from(value: &'a str) -> Self {
        Self {
            text: value.into(),
            mentions: Vec::new(),
            reply_to_message_id: None,
        }
    }
}
