use crate::{
    rpc::{self, Client},
    user::{User, UserStore},
    util::TryFutureExt,
};
use anyhow::{anyhow, Context, Result};
use gpui::{
    sum_tree::{self, Bias, SumTree},
    Entity, ModelContext, ModelHandle, MutableAppContext, Task, WeakModelHandle,
};
use postage::prelude::Stream;
use std::{
    collections::{HashMap, HashSet},
    ops::Range,
    sync::Arc,
};
use time::OffsetDateTime;
use zrpc::{
    proto::{self, ChannelMessageSent},
    TypedEnvelope,
};

pub struct ChannelList {
    available_channels: Option<Vec<ChannelDetails>>,
    channels: HashMap<u64, WeakModelHandle<Channel>>,
    rpc: Arc<Client>,
    user_store: Arc<UserStore>,
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
    pending_messages: Vec<PendingChannelMessage>,
    next_local_message_id: u64,
    user_store: Arc<UserStore>,
    rpc: Arc<Client>,
    _subscription: rpc::Subscription,
}

#[derive(Clone, Debug, PartialEq)]
pub struct ChannelMessage {
    pub id: u64,
    pub body: String,
    pub timestamp: OffsetDateTime,
    pub sender: Arc<User>,
}

pub struct PendingChannelMessage {
    pub body: String,
    local_id: u64,
}

#[derive(Clone, Debug, Default)]
pub struct ChannelMessageSummary {
    max_id: u64,
    count: usize,
}

#[derive(Copy, Clone, Debug, Default)]
struct Count(usize);

pub enum ChannelListEvent {}

#[derive(Clone, Debug, PartialEq)]
pub enum ChannelEvent {
    MessagesAdded {
        old_range: Range<usize>,
        new_count: usize,
    },
}

impl Entity for ChannelList {
    type Event = ChannelListEvent;
}

impl ChannelList {
    pub fn new(
        user_store: Arc<UserStore>,
        rpc: Arc<rpc::Client>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let _task = cx.spawn(|this, mut cx| {
            let rpc = rpc.clone();
            async move {
                let mut user_id = rpc.user_id();
                loop {
                    let available_channels = if user_id.recv().await.unwrap().is_some() {
                        Some(
                            rpc.request(proto::GetChannels {})
                                .await
                                .context("failed to fetch available channels")?
                                .channels
                                .into_iter()
                                .map(Into::into)
                                .collect(),
                        )
                    } else {
                        None
                    };

                    this.update(&mut cx, |this, cx| {
                        if available_channels.is_none() {
                            if this.available_channels.is_none() {
                                return;
                            }
                            this.channels.clear();
                        }
                        this.available_channels = available_channels;
                        cx.notify();
                    });
                }
            }
            .log_err()
        });

        Self {
            available_channels: None,
            channels: Default::default(),
            user_store,
            rpc,
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
        let channel =
            cx.add_model(|cx| Channel::new(details, self.user_store.clone(), self.rpc.clone(), cx));
        self.channels.insert(id, channel.downgrade());
        Some(channel)
    }
}

impl Entity for Channel {
    type Event = ChannelEvent;

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
    pub fn new(
        details: ChannelDetails,
        user_store: Arc<UserStore>,
        rpc: Arc<Client>,
        cx: &mut ModelContext<Self>,
    ) -> Self {
        let _subscription = rpc.subscribe_from_model(details.id, cx, Self::handle_message_sent);

        {
            let user_store = user_store.clone();
            let rpc = rpc.clone();
            let channel_id = details.id;
            cx.spawn(|channel, mut cx| {
                async move {
                    let response = rpc.request(proto::JoinChannel { channel_id }).await?;
                    let messages = messages_from_proto(response.messages, &user_store).await?;
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
            messages: Default::default(),
            pending_messages: Default::default(),
            loaded_all_messages: false,
            next_local_message_id: 0,
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

        let channel_id = self.details.id;
        let current_user_id = self.current_user_id()?;
        let local_id = self.next_local_message_id;
        self.next_local_message_id += 1;
        self.pending_messages.push(PendingChannelMessage {
            local_id,
            body: body.clone(),
        });
        let user_store = self.user_store.clone();
        let rpc = self.rpc.clone();
        Ok(cx.spawn(|this, mut cx| async move {
            let request = rpc.request(proto::SendChannelMessage { channel_id, body });
            let response = request.await?;
            let sender = user_store.get_user(current_user_id).await?;

            this.update(&mut cx, |this, cx| {
                if let Ok(i) = this
                    .pending_messages
                    .binary_search_by_key(&local_id, |msg| msg.local_id)
                {
                    let body = this.pending_messages.remove(i).body;
                    this.insert_messages(
                        SumTree::from_item(
                            ChannelMessage {
                                id: response.message_id,
                                timestamp: OffsetDateTime::from_unix_timestamp(
                                    response.timestamp as i64,
                                )?,
                                body,
                                sender,
                            },
                            &(),
                        ),
                        cx,
                    );
                }
                Ok(())
            })
        }))
    }

    pub fn load_more_messages(&mut self, cx: &mut ModelContext<Self>) -> bool {
        if !self.loaded_all_messages {
            let rpc = self.rpc.clone();
            let user_store = self.user_store.clone();
            let channel_id = self.details.id;
            if let Some(before_message_id) = self.messages.first().map(|message| message.id) {
                cx.spawn(|this, mut cx| {
                    async move {
                        let response = rpc
                            .request(proto::GetChannelMessages {
                                channel_id,
                                before_message_id,
                            })
                            .await?;
                        let loaded_all_messages = response.done;
                        let messages = messages_from_proto(response.messages, &user_store).await?;
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

    pub fn message_count(&self) -> usize {
        self.messages.summary().count
    }

    pub fn messages(&self) -> &SumTree<ChannelMessage> {
        &self.messages
    }

    pub fn message(&self, ix: usize) -> &ChannelMessage {
        let mut cursor = self.messages.cursor::<Count, ()>();
        cursor.seek(&Count(ix), Bias::Right, &());
        cursor.item().unwrap()
    }

    pub fn messages_in_range(&self, range: Range<usize>) -> impl Iterator<Item = &ChannelMessage> {
        let mut cursor = self.messages.cursor::<Count, ()>();
        cursor.seek(&Count(range.start), Bias::Right, &());
        cursor.take(range.len())
    }

    pub fn pending_messages(&self) -> &[PendingChannelMessage] {
        &self.pending_messages
    }

    fn current_user_id(&self) -> Result<u64> {
        self.rpc
            .user_id()
            .borrow()
            .ok_or_else(|| anyhow!("not logged in"))
    }

    fn handle_message_sent(
        &mut self,
        message: TypedEnvelope<ChannelMessageSent>,
        _: Arc<rpc::Client>,
        cx: &mut ModelContext<Self>,
    ) -> Result<()> {
        let user_store = self.user_store.clone();
        let message = message
            .payload
            .message
            .ok_or_else(|| anyhow!("empty message"))?;

        cx.spawn(|this, mut cx| {
            async move {
                let message = ChannelMessage::from_proto(message, &user_store).await?;
                this.update(&mut cx, |this, cx| {
                    this.insert_messages(SumTree::from_item(message, &()), cx)
                });
                Ok(())
            }
            .log_err()
        })
        .detach();
        Ok(())
    }

    fn insert_messages(&mut self, messages: SumTree<ChannelMessage>, cx: &mut ModelContext<Self>) {
        if let Some((first_message, last_message)) = messages.first().zip(messages.last()) {
            let mut old_cursor = self.messages.cursor::<u64, Count>();
            let mut new_messages = old_cursor.slice(&first_message.id, Bias::Left, &());
            let start_ix = old_cursor.sum_start().0;
            let removed_messages = old_cursor.slice(&last_message.id, Bias::Right, &());
            let removed_count = removed_messages.summary().count;
            let new_count = messages.summary().count;
            let end_ix = start_ix + removed_count;

            new_messages.push_tree(messages, &());
            new_messages.push_tree(old_cursor.suffix(&()), &());
            drop(old_cursor);
            self.messages = new_messages;

            cx.emit(ChannelEvent::MessagesAdded {
                old_range: start_ix..end_ix,
                new_count,
            });
            cx.notify();
        }
    }
}

async fn messages_from_proto(
    proto_messages: Vec<proto::ChannelMessage>,
    user_store: &UserStore,
) -> Result<SumTree<ChannelMessage>> {
    let unique_user_ids = proto_messages
        .iter()
        .map(|m| m.sender_id)
        .collect::<HashSet<_>>()
        .into_iter()
        .collect();
    user_store.load_users(unique_user_ids).await?;

    let mut messages = Vec::with_capacity(proto_messages.len());
    for message in proto_messages {
        messages.push(ChannelMessage::from_proto(message, &user_store).await?);
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
        user_store: &UserStore,
    ) -> Result<Self> {
        let sender = user_store.get_user(message.sender_id).await?;
        Ok(ChannelMessage {
            id: message.id,
            body: message.body,
            timestamp: OffsetDateTime::from_unix_timestamp(message.timestamp as i64)?,
            sender,
        })
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

impl sum_tree::Summary for ChannelMessageSummary {
    type Context = ();

    fn add_summary(&mut self, summary: &Self, _: &()) {
        self.max_id = summary.max_id;
        self.count += summary.count;
    }
}

impl<'a> sum_tree::Dimension<'a, ChannelMessageSummary> for u64 {
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

impl<'a> sum_tree::SeekDimension<'a, ChannelMessageSummary> for Count {
    fn cmp(&self, other: &Self, _: &()) -> std::cmp::Ordering {
        Ord::cmp(&self.0, &other.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use postage::mpsc::Receiver;
    use zrpc::{test::Channel, ConnectionId, Peer, Receipt};

    #[gpui::test]
    async fn test_channel_messages(mut cx: TestAppContext) {
        let user_id = 5;
        let client = Client::new();
        let mut server = FakeServer::for_client(user_id, &client, &cx).await;
        let user_store = Arc::new(UserStore::new(client.clone()));

        let channel_list = cx.add_model(|cx| ChannelList::new(user_store, client.clone(), cx));
        channel_list.read_with(&cx, |list, _| assert_eq!(list.available_channels(), None));

        // Get the available channels.
        let get_channels = server.receive::<proto::GetChannels>().await;
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

        // Join a channel and populate its existing messages.
        let channel = channel_list
            .update(&mut cx, |list, cx| {
                let channel_id = list.available_channels().unwrap()[0].id;
                list.get_channel(channel_id, cx)
            })
            .unwrap();
        channel.read_with(&cx, |channel, _| assert!(channel.messages().is_empty()));
        let join_channel = server.receive::<proto::JoinChannel>().await;
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
                        },
                        proto::ChannelMessage {
                            id: 11,
                            body: "b".into(),
                            timestamp: 1001,
                            sender_id: 6,
                        },
                    ],
                    done: false,
                },
            )
            .await;

        // Client requests all users for the received messages
        let mut get_users = server.receive::<proto::GetUsers>().await;
        get_users.payload.user_ids.sort();
        assert_eq!(get_users.payload.user_ids, vec![5, 6]);
        server
            .respond(
                get_users.receipt(),
                proto::GetUsersResponse {
                    users: vec![
                        proto::User {
                            id: 5,
                            github_login: "nathansobo".into(),
                            avatar_url: "http://avatar.com/nathansobo".into(),
                        },
                        proto::User {
                            id: 6,
                            github_login: "maxbrunsfeld".into(),
                            avatar_url: "http://avatar.com/maxbrunsfeld".into(),
                        },
                    ],
                },
            )
            .await;

        assert_eq!(
            channel.next_event(&cx).await,
            ChannelEvent::MessagesAdded {
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
        server
            .send(proto::ChannelMessageSent {
                channel_id: channel.read_with(&cx, |channel, _| channel.details.id),
                message: Some(proto::ChannelMessage {
                    id: 12,
                    body: "c".into(),
                    timestamp: 1002,
                    sender_id: 7,
                }),
            })
            .await;

        // Client requests user for message since they haven't seen them yet
        let get_users = server.receive::<proto::GetUsers>().await;
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
            ChannelEvent::MessagesAdded {
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
        let get_messages = server.receive::<proto::GetChannelMessages>().await;
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
                        },
                        proto::ChannelMessage {
                            id: 9,
                            body: "z".into(),
                            timestamp: 999,
                            sender_id: 6,
                        },
                    ],
                },
            )
            .await;

        assert_eq!(
            channel.next_event(&cx).await,
            ChannelEvent::MessagesAdded {
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

    struct FakeServer {
        peer: Arc<Peer>,
        incoming: Receiver<Box<dyn proto::AnyTypedEnvelope>>,
        connection_id: ConnectionId,
    }

    impl FakeServer {
        async fn for_client(user_id: u64, client: &Arc<Client>, cx: &TestAppContext) -> Self {
            let (client_conn, server_conn) = Channel::bidirectional();
            let peer = Peer::new();
            let (connection_id, io, incoming) = peer.add_connection(server_conn).await;
            cx.background().spawn(io).detach();

            client
                .add_connection(user_id, client_conn, cx.to_async())
                .await
                .unwrap();

            Self {
                peer,
                incoming,
                connection_id,
            }
        }

        async fn send<T: proto::EnvelopedMessage>(&self, message: T) {
            self.peer.send(self.connection_id, message).await.unwrap();
        }

        async fn receive<M: proto::EnvelopedMessage>(&mut self) -> TypedEnvelope<M> {
            *self
                .incoming
                .recv()
                .await
                .unwrap()
                .into_any()
                .downcast::<TypedEnvelope<M>>()
                .unwrap()
        }

        async fn respond<T: proto::RequestMessage>(
            &self,
            receipt: Receipt<T>,
            response: T::Response,
        ) {
            self.peer.respond(receipt, response).await.unwrap()
        }
    }
}
