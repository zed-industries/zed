use super::{
    proto,
    user::{User, UserStore},
    Client, Status, Subscription, TypedEnvelope,
};
use anyhow::{anyhow, Context, Result};
use futures::lock::Mutex;
use gpui::{
    AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext, Task, WeakModelHandle,
};
use postage::prelude::Stream;
use rand::prelude::*;
use std::{
    collections::{HashMap, HashSet},
    mem,
    ops::Range,
    sync::Arc,
};
use sum_tree::{Bias, SumTree};
use time::OffsetDateTime;
use util::{post_inc, ResultExt as _, TryFutureExt};

pub struct ChannelList {
    available_channels: Option<Vec<ChannelDetails>>,
    channels: HashMap<u64, WeakModelHandle<Channel>>,
    client: Arc<Client>,
    user_store: ModelHandle<UserStore>,
    _task: Task<Option<()>>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChannelDetails {
    pub id: u64,
    pub name: String,
}

pub struct Channel {
    details: ChannelDetails,
    messages: SumTree<ChannelMessage>,
    loaded_all_messages: bool,
    next_pending_message_id: usize,
    user_store: ModelHandle<UserStore>,
    rpc: Arc<Client>,
    outgoing_messages_lock: Arc<Mutex<()>>,
    rng: StdRng,
    _subscription: Subscription,
}

#[derive(Clone, Debug)]
pub struct ChannelMessage {
    pub id: ChannelMessageId,
    pub body: String,
    pub timestamp: OffsetDateTime,
    pub sender: Arc<User>,
    pub nonce: u128,
}

#[derive(Copy, Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub enum ChannelMessageId {
    Saved(u64),
    Pending(usize),
}

#[derive(Clone, Debug, Default)]
pub struct ChannelMessageSummary {
    max_id: ChannelMessageId,
    count: usize,
}

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Count(usize);

pub enum ChannelListEvent {}

#[derive(Clone, Debug, PartialEq)]
pub enum ChannelEvent {
    MessagesUpdated {
        old_range: Range<usize>,
        new_count: usize,
    },
}

impl Entity for ChannelList {
    type Event = ChannelListEvent;
}

impl ChannelList {
    pub fn new(
        user_store: ModelHandle<UserStore>,
        rpc: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let _task = cx.spawn_weak(|this, mut cx| {
            let rpc = rpc.clone();
            async move {
                let mut status = rpc.status();
                while let Some((status, this)) = status.recv().await.zip(this.upgrade(&cx)) {
                    match status {
                        Status::Connected { .. } => {
                            let response = rpc
                                .request(proto::GetChannels {})
                                .await
                                .context("failed to fetch available channels")?;
                            this.update(&mut cx, |this, cx| {
                                this.available_channels =
                                    Some(response.channels.into_iter().map(Into::into).collect());

                                let mut to_remove = Vec::new();
                                for (channel_id, channel) in &this.channels {
                                    if let Some(channel) = channel.upgrade(cx) {
                                        channel.update(cx, |channel, cx| channel.rejoin(cx))
                                    } else {
                                        to_remove.push(*channel_id);
                                    }
                                }

                                for channel_id in to_remove {
                                    this.channels.remove(&channel_id);
                                }
                                cx.notify();
                            });
                        }
                        Status::SignedOut { .. } => {
                            this.update(&mut cx, |this, cx| {
                                this.available_channels = None;
                                this.channels.clear();
                                cx.notify();
                            });
                        }
                        _ => {}
                    }
                }
                Ok(())
            }
            .log_err()
        });

        Self {
            available_channels: None,
            channels: Default::default(),
            user_store,
            client: rpc,
            _task,
        }
    }

    pub fn available_channels(&self) -> Option<&[ChannelDetails]> {
        self.available_channels.as_ref().map(Vec::as_slice)
    }

    pub fn get_channel(
        &mut self,
        id: u64,
        cx: &mut MutableAppContext,
    ) -> Option<ModelHandle<Channel>> {
        if let Some(channel) = self.channels.get(&id).and_then(|c| c.upgrade(cx)) {
            return Some(channel);
        }

        let channels = self.available_channels.as_ref()?;
        let details = channels.iter().find(|details| details.id == id)?.clone();
        let channel = cx.add_model(|cx| {
            Channel::new(details, self.user_store.clone(), self.client.clone(), cx)
        });
        self.channels.insert(id, channel.downgrade());
        Some(channel)
    }
}

impl Entity for Channel {
    type Event = ChannelEvent;

    fn release(&mut self, _: &mut MutableAppContext) {
        self.rpc
            .send(proto::LeaveChannel {
                channel_id: self.details.id,
            })
            .log_err();
    }
}

impl Channel {
    pub fn init(rpc: &Arc<Client>) {
        rpc.add_entity_message_handler(Self::handle_message_sent);
    }

    pub fn new(
        details: ChannelDetails,
        user_store: ModelHandle<UserStore>,
        rpc: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let _subscription = rpc.add_model_for_remote_entity(cx.handle(), details.id);

        {
            let user_store = user_store.clone();
            let rpc = rpc.clone();
            let channel_id = details.id;
            cx.spawn(|channel, mut cx| {
                async move {
                    let response = rpc.request(proto::JoinChannel { channel_id }).await?;
                    let messages =
                        messages_from_proto(response.messages, &user_store, &mut cx).await?;
                    let loaded_all_messages = response.done;

                    channel.update(&mut cx, |channel, cx| {
                        channel.insert_messages(messages, cx);
                        channel.loaded_all_messages = loaded_all_messages;
                    });

                    Ok(())
                }
                .log_err()
            })
            .detach();
        }

        Self {
            details,
            user_store,
            rpc,
            outgoing_messages_lock: Default::default(),
            messages: Default::default(),
            loaded_all_messages: false,
            next_pending_message_id: 0,
            rng: StdRng::from_entropy(),
            _subscription,
        }
    }

    pub fn name(&self) -> &str {
        &self.details.name
    }

    pub fn send_message(
        &mut self,
        body: String,
        cx: &mut ModelContext<Self>,
    ) -> Result<Task<Result<()>>> {
        if body.is_empty() {
            Err(anyhow!("message body can't be empty"))?;
        }

        let current_user = self
            .user_store
            .read(cx)
            .current_user()
            .ok_or_else(|| anyhow!("current_user is not present"))?;

        let channel_id = self.details.id;
        let pending_id = ChannelMessageId::Pending(post_inc(&mut self.next_pending_message_id));
        let nonce = self.rng.gen();
        self.insert_messages(
            SumTree::from_item(
                ChannelMessage {
                    id: pending_id,
                    body: body.clone(),
                    sender: current_user,
                    timestamp: OffsetDateTime::now_utc(),
                    nonce,
                },
                &(),
            ),
            cx,
        );
        let user_store = self.user_store.clone();
        let rpc = self.rpc.clone();
        let outgoing_messages_lock = self.outgoing_messages_lock.clone();
        Ok(cx.spawn(|this, mut cx| async move {
            let outgoing_message_guard = outgoing_messages_lock.lock().await;
            let request = rpc.request(proto::SendChannelMessage {
                channel_id,
                body,
                nonce: Some(nonce.into()),
            });
            let response = request.await?;
            drop(outgoing_message_guard);
            let message = ChannelMessage::from_proto(
                response.message.ok_or_else(|| anyhow!("invalid message"))?,
                &user_store,
                &mut cx,
            )
            .await?;
            this.update(&mut cx, |this, cx| {
                this.insert_messages(SumTree::from_item(message, &()), cx);
                Ok(())
            })
        }))
    }

    pub fn load_more_messages(&mut self, cx: &mut ModelContext<Self>) -> bool {
        if !self.loaded_all_messages {
            let rpc = self.rpc.clone();
            let user_store = self.user_store.clone();
            let channel_id = self.details.id;
            if let Some(before_message_id) =
                self.messages.first().and_then(|message| match message.id {
                    ChannelMessageId::Saved(id) => Some(id),
                    ChannelMessageId::Pending(_) => None,
                })
            {
                cx.spawn(|this, mut cx| {
                    async move {
                        let response = rpc
                            .request(proto::GetChannelMessages {
                                channel_id,
                                before_message_id,
                            })
                            .await?;
                        let loaded_all_messages = response.done;
                        let messages =
                            messages_from_proto(response.messages, &user_store, &mut cx).await?;
                        this.update(&mut cx, |this, cx| {
                            this.loaded_all_messages = loaded_all_messages;
                            this.insert_messages(messages, cx);
                        });
                        Ok(())
                    }
                    .log_err()
                })
                .detach();
                return true;
            }
        }
        false
    }

    pub fn rejoin(&mut self, cx: &mut ModelContext<Self>) {
        let user_store = self.user_store.clone();
        let rpc = self.rpc.clone();
        let channel_id = self.details.id;
        cx.spawn(|this, mut cx| {
            async move {
                let response = rpc.request(proto::JoinChannel { channel_id }).await?;
                let messages = messages_from_proto(response.messages, &user_store, &mut cx).await?;
                let loaded_all_messages = response.done;

                let pending_messages = this.update(&mut cx, |this, cx| {
                    if let Some((first_new_message, last_old_message)) =
                        messages.first().zip(this.messages.last())
                    {
                        if first_new_message.id > last_old_message.id {
                            let old_messages = mem::take(&mut this.messages);
                            cx.emit(ChannelEvent::MessagesUpdated {
                                old_range: 0..old_messages.summary().count,
                                new_count: 0,
                            });
                            this.loaded_all_messages = loaded_all_messages;
                        }
                    }

                    this.insert_messages(messages, cx);
                    if loaded_all_messages {
                        this.loaded_all_messages = loaded_all_messages;
                    }

                    this.pending_messages().cloned().collect::<Vec<_>>()
                });

                for pending_message in pending_messages {
                    let request = rpc.request(proto::SendChannelMessage {
                        channel_id,
                        body: pending_message.body,
                        nonce: Some(pending_message.nonce.into()),
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
                    });
                }

                Ok(())
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
        this: ModelHandle<Self>,
        message: TypedEnvelope<proto::ChannelMessageSent>,
        _: Arc<Client>,
        mut cx: AsyncAppContext,
    ) -> Result<()> {
        let user_store = this.read_with(&cx, |this, _| this.user_store.clone());
        let message = message
            .payload
            .message
            .ok_or_else(|| anyhow!("empty message"))?;

        let message = ChannelMessage::from_proto(message, &user_store, &mut cx).await?;
        this.update(&mut cx, |this, cx| {
            this.insert_messages(SumTree::from_item(message, &()), cx)
        });

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

            new_messages.push_tree(messages, &());

            let mut ranges = Vec::<Range<usize>>::new();
            if new_messages.last().unwrap().is_pending() {
                new_messages.push_tree(old_cursor.suffix(&()), &());
            } else {
                new_messages.push_tree(
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
                cx.emit(ChannelEvent::MessagesUpdated {
                    old_range: range,
                    new_count: 0,
                });
            }
            cx.emit(ChannelEvent::MessagesUpdated {
                old_range: start_ix..end_ix,
                new_count,
            });
            cx.notify();
        }
    }
}

async fn messages_from_proto(
    proto_messages: Vec<proto::ChannelMessage>,
    user_store: &ModelHandle<UserStore>,
    cx: &mut AsyncAppContext,
) -> Result<SumTree<ChannelMessage>> {
    let unique_user_ids = proto_messages
        .iter()
        .map(|m| m.sender_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    user_store
        .update(cx, |user_store, cx| {
            user_store.load_users(unique_user_ids, cx)
        })
        .await?;

    let mut messages = Vec::with_capacity(proto_messages.len());
    for message in proto_messages {
        messages.push(ChannelMessage::from_proto(message, user_store, cx).await?);
    }
    let mut result = SumTree::new();
    result.extend(messages, &());
    Ok(result)
}

impl From<proto::Channel> for ChannelDetails {
    fn from(message: proto::Channel) -> Self {
        Self {
            id: message.id,
            name: message.name,
        }
    }
}

impl ChannelMessage {
    pub async fn from_proto(
        message: proto::ChannelMessage,
        user_store: &ModelHandle<UserStore>,
        cx: &mut AsyncAppContext,
    ) -> Result<Self> {
        let sender = user_store
            .update(cx, |user_store, cx| {
                user_store.fetch_user(message.sender_id, cx)
            })
            .await?;
        Ok(ChannelMessage {
            id: ChannelMessageId::Saved(message.id),
            body: message.body,
            timestamp: OffsetDateTime::from_unix_timestamp(message.timestamp as i64)?,
            sender,
            nonce: message
                .nonce
                .ok_or_else(|| anyhow!("nonce is required"))?
                .into(),
        })
    }

    pub fn is_pending(&self) -> bool {
        matches!(self.id, ChannelMessageId::Pending(_))
    }
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{FakeHttpClient, FakeServer};
    use gpui::TestAppContext;
    use surf::http::Response;

    #[gpui::test]
    async fn test_channel_messages(mut cx: TestAppContext) {
        let user_id = 5;
        let http_client = FakeHttpClient::new(|_| async move { Ok(Response::new(404)) });
        let mut client = Client::new(http_client.clone());
        let server = FakeServer::for_client(user_id, &mut client, &cx).await;
        let user_store = cx.add_model(|cx| UserStore::new(client.clone(), http_client, cx));

        let channel_list = cx.add_model(|cx| ChannelList::new(user_store, client.clone(), cx));
        channel_list.read_with(&cx, |list, _| assert_eq!(list.available_channels(), None));

        // Get the available channels.
        let get_channels = server.receive::<proto::GetChannels>().await.unwrap();
        server
            .respond(
                get_channels.receipt(),
                proto::GetChannelsResponse {
                    channels: vec![proto::Channel {
                        id: 5,
                        name: "the-channel".to_string(),
                    }],
                },
            )
            .await;
        channel_list.next_notification(&cx).await;
        channel_list.read_with(&cx, |list, _| {
            assert_eq!(
                list.available_channels().unwrap(),
                &[ChannelDetails {
                    id: 5,
                    name: "the-channel".into(),
                }]
            )
        });

        let get_users = server.receive::<proto::GetUsers>().await.unwrap();
        assert_eq!(get_users.payload.user_ids, vec![5]);
        server
            .respond(
                get_users.receipt(),
                proto::GetUsersResponse {
                    users: vec![proto::User {
                        id: 5,
                        github_login: "nathansobo".into(),
                        avatar_url: "http://avatar.com/nathansobo".into(),
                    }],
                },
            )
            .await;

        // Join a channel and populate its existing messages.
        let channel = channel_list
            .update(&mut cx, |list, cx| {
                let channel_id = list.available_channels().unwrap()[0].id;
                list.get_channel(channel_id, cx)
            })
            .unwrap();
        channel.read_with(&cx, |channel, _| assert!(channel.messages().is_empty()));
        let join_channel = server.receive::<proto::JoinChannel>().await.unwrap();
        server
            .respond(
                join_channel.receipt(),
                proto::JoinChannelResponse {
                    messages: vec![
                        proto::ChannelMessage {
                            id: 10,
                            body: "a".into(),
                            timestamp: 1000,
                            sender_id: 5,
                            nonce: Some(1.into()),
                        },
                        proto::ChannelMessage {
                            id: 11,
                            body: "b".into(),
                            timestamp: 1001,
                            sender_id: 6,
                            nonce: Some(2.into()),
                        },
                    ],
                    done: false,
                },
            )
            .await;

        // Client requests all users for the received messages
        let mut get_users = server.receive::<proto::GetUsers>().await.unwrap();
        get_users.payload.user_ids.sort();
        assert_eq!(get_users.payload.user_ids, vec![6]);
        server
            .respond(
                get_users.receipt(),
                proto::GetUsersResponse {
                    users: vec![proto::User {
                        id: 6,
                        github_login: "maxbrunsfeld".into(),
                        avatar_url: "http://avatar.com/maxbrunsfeld".into(),
                    }],
                },
            )
            .await;

        assert_eq!(
            channel.next_event(&cx).await,
            ChannelEvent::MessagesUpdated {
                old_range: 0..0,
                new_count: 2,
            }
        );
        channel.read_with(&cx, |channel, _| {
            assert_eq!(
                channel
                    .messages_in_range(0..2)
                    .map(|message| (message.sender.github_login.clone(), message.body.clone()))
                    .collect::<Vec<_>>(),
                &[
                    ("nathansobo".into(), "a".into()),
                    ("maxbrunsfeld".into(), "b".into())
                ]
            );
        });

        // Receive a new message.
        server.send(proto::ChannelMessageSent {
            channel_id: channel.read_with(&cx, |channel, _| channel.details.id),
            message: Some(proto::ChannelMessage {
                id: 12,
                body: "c".into(),
                timestamp: 1002,
                sender_id: 7,
                nonce: Some(3.into()),
            }),
        });

        // Client requests user for message since they haven't seen them yet
        let get_users = server.receive::<proto::GetUsers>().await.unwrap();
        assert_eq!(get_users.payload.user_ids, vec![7]);
        server
            .respond(
                get_users.receipt(),
                proto::GetUsersResponse {
                    users: vec![proto::User {
                        id: 7,
                        github_login: "as-cii".into(),
                        avatar_url: "http://avatar.com/as-cii".into(),
                    }],
                },
            )
            .await;

        assert_eq!(
            channel.next_event(&cx).await,
            ChannelEvent::MessagesUpdated {
                old_range: 2..2,
                new_count: 1,
            }
        );
        channel.read_with(&cx, |channel, _| {
            assert_eq!(
                channel
                    .messages_in_range(2..3)
                    .map(|message| (message.sender.github_login.clone(), message.body.clone()))
                    .collect::<Vec<_>>(),
                &[("as-cii".into(), "c".into())]
            )
        });

        // Scroll up to view older messages.
        channel.update(&mut cx, |channel, cx| {
            assert!(channel.load_more_messages(cx));
        });
        let get_messages = server.receive::<proto::GetChannelMessages>().await.unwrap();
        assert_eq!(get_messages.payload.channel_id, 5);
        assert_eq!(get_messages.payload.before_message_id, 10);
        server
            .respond(
                get_messages.receipt(),
                proto::GetChannelMessagesResponse {
                    done: true,
                    messages: vec![
                        proto::ChannelMessage {
                            id: 8,
                            body: "y".into(),
                            timestamp: 998,
                            sender_id: 5,
                            nonce: Some(4.into()),
                        },
                        proto::ChannelMessage {
                            id: 9,
                            body: "z".into(),
                            timestamp: 999,
                            sender_id: 6,
                            nonce: Some(5.into()),
                        },
                    ],
                },
            )
            .await;

        assert_eq!(
            channel.next_event(&cx).await,
            ChannelEvent::MessagesUpdated {
                old_range: 0..0,
                new_count: 2,
            }
        );
        channel.read_with(&cx, |channel, _| {
            assert_eq!(
                channel
                    .messages_in_range(0..2)
                    .map(|message| (message.sender.github_login.clone(), message.body.clone()))
                    .collect::<Vec<_>>(),
                &[
                    ("nathansobo".into(), "y".into()),
                    ("maxbrunsfeld".into(), "z".into())
                ]
            );
        });
    }
}
