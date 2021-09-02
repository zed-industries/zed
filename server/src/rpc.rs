use super::{
    auth,
    db::{ChannelId, MessageId, UserId},
    AppState,
};
use anyhow::anyhow;
use async_std::{sync::RwLock, task};
use async_tungstenite::{
    tungstenite::{protocol::Role, Error as WebSocketError, Message as WebSocketMessage},
    WebSocketStream,
};
use futures::{future::BoxFuture, FutureExt};
use postage::{mpsc, prelude::Sink as _, prelude::Stream as _};
use sha1::{Digest as _, Sha1};
use std::{
    any::TypeId,
    collections::{hash_map, HashMap, HashSet},
    future::Future,
    mem,
    sync::Arc,
    time::Instant,
};
use surf::StatusCode;
use tide::log;
use tide::{
    http::headers::{HeaderName, CONNECTION, UPGRADE},
    Request, Response,
};
use time::OffsetDateTime;
use zrpc::{
    auth::random_token,
    proto::{self, AnyTypedEnvelope, EnvelopedMessage},
    ConnectionId, Peer, TypedEnvelope,
};

type ReplicaId = u16;

type MessageHandler = Box<
    dyn Send
        + Sync
        + Fn(Arc<Server>, Box<dyn AnyTypedEnvelope>) -> BoxFuture<'static, tide::Result<()>>,
>;

pub struct Server {
    peer: Arc<Peer>,
    state: RwLock<ServerState>,
    app_state: Arc<AppState>,
    handlers: HashMap<TypeId, MessageHandler>,
    notifications: Option<mpsc::Sender<()>>,
}

#[derive(Default)]
struct ServerState {
    connections: HashMap<ConnectionId, Connection>,
    pub worktrees: HashMap<u64, Worktree>,
    channels: HashMap<ChannelId, Channel>,
    next_worktree_id: u64,
}

struct Connection {
    user_id: UserId,
    worktrees: HashSet<u64>,
    channels: HashSet<ChannelId>,
}

struct Worktree {
    host_connection_id: Option<ConnectionId>,
    guest_connection_ids: HashMap<ConnectionId, ReplicaId>,
    active_replica_ids: HashSet<ReplicaId>,
    access_token: String,
    root_name: String,
    entries: HashMap<u64, proto::Entry>,
}

#[derive(Default)]
struct Channel {
    connection_ids: HashSet<ConnectionId>,
}

const MESSAGE_COUNT_PER_PAGE: usize = 100;
const MAX_MESSAGE_LEN: usize = 1024;

impl Server {
    pub fn new(
        app_state: Arc<AppState>,
        peer: Arc<Peer>,
        notifications: Option<mpsc::Sender<()>>,
    ) -> Arc<Self> {
        let mut server = Self {
            peer,
            app_state,
            state: Default::default(),
            handlers: Default::default(),
            notifications,
        };

        server
            .add_handler(Server::share_worktree)
            .add_handler(Server::join_worktree)
            .add_handler(Server::update_worktree)
            .add_handler(Server::close_worktree)
            .add_handler(Server::open_buffer)
            .add_handler(Server::close_buffer)
            .add_handler(Server::update_buffer)
            .add_handler(Server::buffer_saved)
            .add_handler(Server::save_buffer)
            .add_handler(Server::get_channels)
            .add_handler(Server::get_users)
            .add_handler(Server::join_channel)
            .add_handler(Server::leave_channel)
            .add_handler(Server::send_channel_message)
            .add_handler(Server::get_channel_messages);

        Arc::new(server)
    }

    fn add_handler<F, Fut, M>(&mut self, handler: F) -> &mut Self
    where
        F: 'static + Send + Sync + Fn(Arc<Self>, TypedEnvelope<M>) -> Fut,
        Fut: 'static + Send + Future<Output = tide::Result<()>>,
        M: EnvelopedMessage,
    {
        let prev_handler = self.handlers.insert(
            TypeId::of::<M>(),
            Box::new(move |server, envelope| {
                let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                (handler)(server, *envelope).boxed()
            }),
        );
        if prev_handler.is_some() {
            panic!("registered a handler for the same message twice");
        }
        self
    }

    pub fn handle_connection<Conn>(
        self: &Arc<Self>,
        connection: Conn,
        addr: String,
        user_id: UserId,
    ) -> impl Future<Output = ()>
    where
        Conn: 'static
            + futures::Sink<WebSocketMessage, Error = WebSocketError>
            + futures::Stream<Item = Result<WebSocketMessage, WebSocketError>>
            + Send
            + Unpin,
    {
        let this = self.clone();
        async move {
            let (connection_id, handle_io, mut incoming_rx) =
                this.peer.add_connection(connection).await;
            this.add_connection(connection_id, user_id).await;

            let handle_io = handle_io.fuse();
            futures::pin_mut!(handle_io);
            loop {
                let next_message = incoming_rx.recv().fuse();
                futures::pin_mut!(next_message);
                futures::select_biased! {
                    message = next_message => {
                        if let Some(message) = message {
                            let start_time = Instant::now();
                            log::info!("RPC message received: {}", message.payload_type_name());
                            if let Some(handler) = this.handlers.get(&message.payload_type_id()) {
                                if let Err(err) = (handler)(this.clone(), message).await {
                                    log::error!("error handling message: {:?}", err);
                                } else {
                                    log::info!("RPC message handled. duration:{:?}", start_time.elapsed());
                                }

                                if let Some(mut notifications) = this.notifications.clone() {
                                    let _ = notifications.send(()).await;
                                }
                            } else {
                                log::warn!("unhandled message: {}", message.payload_type_name());
                            }
                        } else {
                            log::info!("rpc connection closed {:?}", addr);
                            break;
                        }
                    }
                    handle_io = handle_io => {
                        if let Err(err) = handle_io {
                            log::error!("error handling rpc connection {:?} - {:?}", addr, err);
                        }
                        break;
                    }
                }
            }

            if let Err(err) = this.sign_out(connection_id).await {
                log::error!("error signing out connection {:?} - {:?}", addr, err);
            }
        }
    }

    async fn sign_out(self: &Arc<Self>, connection_id: zrpc::ConnectionId) -> tide::Result<()> {
        self.peer.disconnect(connection_id).await;
        let worktree_ids = self.remove_connection(connection_id).await;
        for worktree_id in worktree_ids {
            let state = self.state.read().await;
            if let Some(worktree) = state.worktrees.get(&worktree_id) {
                broadcast(connection_id, worktree.connection_ids(), |conn_id| {
                    self.peer.send(
                        conn_id,
                        proto::RemovePeer {
                            worktree_id,
                            peer_id: connection_id.0,
                        },
                    )
                })
                .await?;
            }
        }
        Ok(())
    }

    // Add a new connection associated with a given user.
    async fn add_connection(&self, connection_id: ConnectionId, user_id: UserId) {
        self.state.write().await.connections.insert(
            connection_id,
            Connection {
                user_id,
                worktrees: Default::default(),
                channels: Default::default(),
            },
        );
    }

    // Remove the given connection and its association with any worktrees.
    async fn remove_connection(&self, connection_id: ConnectionId) -> Vec<u64> {
        let mut worktree_ids = Vec::new();
        let mut state = self.state.write().await;
        if let Some(connection) = state.connections.remove(&connection_id) {
            for channel_id in connection.channels {
                if let Some(channel) = state.channels.get_mut(&channel_id) {
                    channel.connection_ids.remove(&connection_id);
                }
            }
            for worktree_id in connection.worktrees {
                if let Some(worktree) = state.worktrees.get_mut(&worktree_id) {
                    if worktree.host_connection_id == Some(connection_id) {
                        worktree_ids.push(worktree_id);
                    } else if let Some(replica_id) =
                        worktree.guest_connection_ids.remove(&connection_id)
                    {
                        worktree.active_replica_ids.remove(&replica_id);
                        worktree_ids.push(worktree_id);
                    }
                }
            }
        }
        worktree_ids
    }

    async fn share_worktree(
        self: Arc<Server>,
        mut request: TypedEnvelope<proto::ShareWorktree>,
    ) -> tide::Result<()> {
        let mut state = self.state.write().await;
        let worktree_id = state.next_worktree_id;
        state.next_worktree_id += 1;
        let access_token = random_token();
        let worktree = request
            .payload
            .worktree
            .as_mut()
            .ok_or_else(|| anyhow!("missing worktree"))?;
        let entries = mem::take(&mut worktree.entries)
            .into_iter()
            .map(|entry| (entry.id, entry))
            .collect();
        state.worktrees.insert(
            worktree_id,
            Worktree {
                host_connection_id: Some(request.sender_id),
                guest_connection_ids: Default::default(),
                active_replica_ids: Default::default(),
                access_token: access_token.clone(),
                root_name: mem::take(&mut worktree.root_name),
                entries,
            },
        );

        self.peer
            .respond(
                request.receipt(),
                proto::ShareWorktreeResponse {
                    worktree_id,
                    access_token,
                },
            )
            .await?;
        Ok(())
    }

    async fn join_worktree(
        self: Arc<Server>,
        request: TypedEnvelope<proto::OpenWorktree>,
    ) -> tide::Result<()> {
        let worktree_id = request.payload.worktree_id;
        let access_token = &request.payload.access_token;

        let mut state = self.state.write().await;
        if let Some((peer_replica_id, worktree)) =
            state.join_worktree(request.sender_id, worktree_id, access_token)
        {
            let mut peers = Vec::new();
            if let Some(host_connection_id) = worktree.host_connection_id {
                peers.push(proto::Peer {
                    peer_id: host_connection_id.0,
                    replica_id: 0,
                });
            }
            for (peer_conn_id, peer_replica_id) in &worktree.guest_connection_ids {
                if *peer_conn_id != request.sender_id {
                    peers.push(proto::Peer {
                        peer_id: peer_conn_id.0,
                        replica_id: *peer_replica_id as u32,
                    });
                }
            }

            broadcast(request.sender_id, worktree.connection_ids(), |conn_id| {
                self.peer.send(
                    conn_id,
                    proto::AddPeer {
                        worktree_id,
                        peer: Some(proto::Peer {
                            peer_id: request.sender_id.0,
                            replica_id: peer_replica_id as u32,
                        }),
                    },
                )
            })
            .await?;
            self.peer
                .respond(
                    request.receipt(),
                    proto::OpenWorktreeResponse {
                        worktree_id,
                        worktree: Some(proto::Worktree {
                            root_name: worktree.root_name.clone(),
                            entries: worktree.entries.values().cloned().collect(),
                        }),
                        replica_id: peer_replica_id as u32,
                        peers,
                    },
                )
                .await?;
        } else {
            self.peer
                .respond(
                    request.receipt(),
                    proto::OpenWorktreeResponse {
                        worktree_id,
                        worktree: None,
                        replica_id: 0,
                        peers: Vec::new(),
                    },
                )
                .await?;
        }

        Ok(())
    }

    async fn update_worktree(
        self: Arc<Server>,
        request: TypedEnvelope<proto::UpdateWorktree>,
    ) -> tide::Result<()> {
        {
            let mut state = self.state.write().await;
            let worktree = state.write_worktree(request.payload.worktree_id, request.sender_id)?;
            for entry_id in &request.payload.removed_entries {
                worktree.entries.remove(&entry_id);
            }

            for entry in &request.payload.updated_entries {
                worktree.entries.insert(entry.id, entry.clone());
            }
        }

        self.broadcast_in_worktree(request.payload.worktree_id, &request)
            .await?;
        Ok(())
    }

    async fn close_worktree(
        self: Arc<Server>,
        request: TypedEnvelope<proto::CloseWorktree>,
    ) -> tide::Result<()> {
        let connection_ids;
        {
            let mut state = self.state.write().await;
            let worktree = state.write_worktree(request.payload.worktree_id, request.sender_id)?;
            connection_ids = worktree.connection_ids();
            if worktree.host_connection_id == Some(request.sender_id) {
                worktree.host_connection_id = None;
            } else if let Some(replica_id) =
                worktree.guest_connection_ids.remove(&request.sender_id)
            {
                worktree.active_replica_ids.remove(&replica_id);
            }
        }

        broadcast(request.sender_id, connection_ids, |conn_id| {
            self.peer.send(
                conn_id,
                proto::RemovePeer {
                    worktree_id: request.payload.worktree_id,
                    peer_id: request.sender_id.0,
                },
            )
        })
        .await?;

        Ok(())
    }

    async fn open_buffer(
        self: Arc<Server>,
        request: TypedEnvelope<proto::OpenBuffer>,
    ) -> tide::Result<()> {
        let receipt = request.receipt();
        let worktree_id = request.payload.worktree_id;
        let host_connection_id = self
            .state
            .read()
            .await
            .read_worktree(worktree_id, request.sender_id)?
            .host_connection_id()?;

        let response = self
            .peer
            .forward_request(request.sender_id, host_connection_id, request.payload)
            .await?;
        self.peer.respond(receipt, response).await?;
        Ok(())
    }

    async fn close_buffer(
        self: Arc<Server>,
        request: TypedEnvelope<proto::CloseBuffer>,
    ) -> tide::Result<()> {
        let host_connection_id = self
            .state
            .read()
            .await
            .read_worktree(request.payload.worktree_id, request.sender_id)?
            .host_connection_id()?;

        self.peer
            .forward_send(request.sender_id, host_connection_id, request.payload)
            .await?;

        Ok(())
    }

    async fn save_buffer(
        self: Arc<Server>,
        request: TypedEnvelope<proto::SaveBuffer>,
    ) -> tide::Result<()> {
        let host;
        let guests;
        {
            let state = self.state.read().await;
            let worktree = state.read_worktree(request.payload.worktree_id, request.sender_id)?;
            host = worktree.host_connection_id()?;
            guests = worktree
                .guest_connection_ids
                .keys()
                .copied()
                .collect::<Vec<_>>();
        }

        let sender = request.sender_id;
        let receipt = request.receipt();
        let response = self
            .peer
            .forward_request(sender, host, request.payload.clone())
            .await?;

        broadcast(host, guests, |conn_id| {
            let response = response.clone();
            let peer = &self.peer;
            async move {
                if conn_id == sender {
                    peer.respond(receipt, response).await
                } else {
                    peer.forward_send(host, conn_id, response).await
                }
            }
        })
        .await?;

        Ok(())
    }

    async fn update_buffer(
        self: Arc<Server>,
        request: TypedEnvelope<proto::UpdateBuffer>,
    ) -> tide::Result<()> {
        self.broadcast_in_worktree(request.payload.worktree_id, &request)
            .await
    }

    async fn buffer_saved(
        self: Arc<Server>,
        request: TypedEnvelope<proto::BufferSaved>,
    ) -> tide::Result<()> {
        self.broadcast_in_worktree(request.payload.worktree_id, &request)
            .await
    }

    async fn get_channels(
        self: Arc<Server>,
        request: TypedEnvelope<proto::GetChannels>,
    ) -> tide::Result<()> {
        let user_id = self
            .state
            .read()
            .await
            .user_id_for_connection(request.sender_id)?;
        let channels = self.app_state.db.get_accessible_channels(user_id).await?;
        self.peer
            .respond(
                request.receipt(),
                proto::GetChannelsResponse {
                    channels: channels
                        .into_iter()
                        .map(|chan| proto::Channel {
                            id: chan.id.to_proto(),
                            name: chan.name,
                        })
                        .collect(),
                },
            )
            .await?;
        Ok(())
    }

    async fn get_users(
        self: Arc<Server>,
        request: TypedEnvelope<proto::GetUsers>,
    ) -> tide::Result<()> {
        let user_id = self
            .state
            .read()
            .await
            .user_id_for_connection(request.sender_id)?;
        let receipt = request.receipt();
        let user_ids = request.payload.user_ids.into_iter().map(UserId::from_proto);
        let users = self
            .app_state
            .db
            .get_users_by_ids(user_id, user_ids)
            .await?
            .into_iter()
            .map(|user| proto::User {
                id: user.id.to_proto(),
                github_login: user.github_login,
                avatar_url: String::new(),
            })
            .collect();
        self.peer
            .respond(receipt, proto::GetUsersResponse { users })
            .await?;
        Ok(())
    }

    async fn join_channel(
        self: Arc<Self>,
        request: TypedEnvelope<proto::JoinChannel>,
    ) -> tide::Result<()> {
        let user_id = self
            .state
            .read()
            .await
            .user_id_for_connection(request.sender_id)?;
        let channel_id = ChannelId::from_proto(request.payload.channel_id);
        if !self
            .app_state
            .db
            .can_user_access_channel(user_id, channel_id)
            .await?
        {
            Err(anyhow!("access denied"))?;
        }

        self.state
            .write()
            .await
            .join_channel(request.sender_id, channel_id);
        let messages = self
            .app_state
            .db
            .get_channel_messages(channel_id, MESSAGE_COUNT_PER_PAGE, None)
            .await?
            .into_iter()
            .map(|msg| proto::ChannelMessage {
                id: msg.id.to_proto(),
                body: msg.body,
                timestamp: msg.sent_at.unix_timestamp() as u64,
                sender_id: msg.sender_id.to_proto(),
            })
            .collect::<Vec<_>>();
        self.peer
            .respond(
                request.receipt(),
                proto::JoinChannelResponse {
                    done: messages.len() < MESSAGE_COUNT_PER_PAGE,
                    messages,
                },
            )
            .await?;
        Ok(())
    }

    async fn leave_channel(
        self: Arc<Self>,
        request: TypedEnvelope<proto::LeaveChannel>,
    ) -> tide::Result<()> {
        let user_id = self
            .state
            .read()
            .await
            .user_id_for_connection(request.sender_id)?;
        let channel_id = ChannelId::from_proto(request.payload.channel_id);
        if !self
            .app_state
            .db
            .can_user_access_channel(user_id, channel_id)
            .await?
        {
            Err(anyhow!("access denied"))?;
        }

        self.state
            .write()
            .await
            .leave_channel(request.sender_id, channel_id);

        Ok(())
    }

    async fn send_channel_message(
        self: Arc<Self>,
        request: TypedEnvelope<proto::SendChannelMessage>,
    ) -> tide::Result<()> {
        let receipt = request.receipt();
        let channel_id = ChannelId::from_proto(request.payload.channel_id);
        let user_id;
        let connection_ids;
        {
            let state = self.state.read().await;
            user_id = state.user_id_for_connection(request.sender_id)?;
            if let Some(channel) = state.channels.get(&channel_id) {
                connection_ids = channel.connection_ids();
            } else {
                return Ok(());
            }
        }

        // Validate the message body.
        let body = request.payload.body.trim().to_string();
        if body.len() > MAX_MESSAGE_LEN {
            self.peer
                .respond_with_error(
                    receipt,
                    proto::Error {
                        message: "message is too long".to_string(),
                    },
                )
                .await?;
            return Ok(());
        }
        if body.is_empty() {
            self.peer
                .respond_with_error(
                    receipt,
                    proto::Error {
                        message: "message can't be blank".to_string(),
                    },
                )
                .await?;
            return Ok(());
        }

        let timestamp = OffsetDateTime::now_utc();
        let message_id = self
            .app_state
            .db
            .create_channel_message(channel_id, user_id, &body, timestamp)
            .await?
            .to_proto();
        let message = proto::ChannelMessage {
            sender_id: user_id.to_proto(),
            id: message_id,
            body,
            timestamp: timestamp.unix_timestamp() as u64,
        };
        broadcast(request.sender_id, connection_ids, |conn_id| {
            self.peer.send(
                conn_id,
                proto::ChannelMessageSent {
                    channel_id: channel_id.to_proto(),
                    message: Some(message.clone()),
                },
            )
        })
        .await?;
        self.peer
            .respond(
                receipt,
                proto::SendChannelMessageResponse {
                    message: Some(message),
                },
            )
            .await?;
        Ok(())
    }

    async fn get_channel_messages(
        self: Arc<Self>,
        request: TypedEnvelope<proto::GetChannelMessages>,
    ) -> tide::Result<()> {
        let user_id = self
            .state
            .read()
            .await
            .user_id_for_connection(request.sender_id)?;
        let channel_id = ChannelId::from_proto(request.payload.channel_id);
        if !self
            .app_state
            .db
            .can_user_access_channel(user_id, channel_id)
            .await?
        {
            Err(anyhow!("access denied"))?;
        }

        let messages = self
            .app_state
            .db
            .get_channel_messages(
                channel_id,
                MESSAGE_COUNT_PER_PAGE,
                Some(MessageId::from_proto(request.payload.before_message_id)),
            )
            .await?
            .into_iter()
            .map(|msg| proto::ChannelMessage {
                id: msg.id.to_proto(),
                body: msg.body,
                timestamp: msg.sent_at.unix_timestamp() as u64,
                sender_id: msg.sender_id.to_proto(),
            })
            .collect::<Vec<_>>();
        self.peer
            .respond(
                request.receipt(),
                proto::GetChannelMessagesResponse {
                    done: messages.len() < MESSAGE_COUNT_PER_PAGE,
                    messages,
                },
            )
            .await?;
        Ok(())
    }

    async fn broadcast_in_worktree<T: proto::EnvelopedMessage>(
        &self,
        worktree_id: u64,
        message: &TypedEnvelope<T>,
    ) -> tide::Result<()> {
        let connection_ids = self
            .state
            .read()
            .await
            .read_worktree(worktree_id, message.sender_id)?
            .connection_ids();

        broadcast(message.sender_id, connection_ids, |conn_id| {
            self.peer
                .forward_send(message.sender_id, conn_id, message.payload.clone())
        })
        .await?;

        Ok(())
    }
}

pub async fn broadcast<F, T>(
    sender_id: ConnectionId,
    receiver_ids: Vec<ConnectionId>,
    mut f: F,
) -> anyhow::Result<()>
where
    F: FnMut(ConnectionId) -> T,
    T: Future<Output = anyhow::Result<()>>,
{
    let futures = receiver_ids
        .into_iter()
        .filter(|id| *id != sender_id)
        .map(|id| f(id));
    futures::future::try_join_all(futures).await?;
    Ok(())
}

impl ServerState {
    fn join_channel(&mut self, connection_id: ConnectionId, channel_id: ChannelId) {
        if let Some(connection) = self.connections.get_mut(&connection_id) {
            connection.channels.insert(channel_id);
            self.channels
                .entry(channel_id)
                .or_default()
                .connection_ids
                .insert(connection_id);
        }
    }

    fn leave_channel(&mut self, connection_id: ConnectionId, channel_id: ChannelId) {
        if let Some(connection) = self.connections.get_mut(&connection_id) {
            connection.channels.remove(&channel_id);
            if let hash_map::Entry::Occupied(mut entry) = self.channels.entry(channel_id) {
                entry.get_mut().connection_ids.remove(&connection_id);
                if entry.get_mut().connection_ids.is_empty() {
                    entry.remove();
                }
            }
        }
    }

    fn user_id_for_connection(&self, connection_id: ConnectionId) -> tide::Result<UserId> {
        Ok(self
            .connections
            .get(&connection_id)
            .ok_or_else(|| anyhow!("unknown connection"))?
            .user_id)
    }

    // Add the given connection as a guest of the given worktree
    fn join_worktree(
        &mut self,
        connection_id: ConnectionId,
        worktree_id: u64,
        access_token: &str,
    ) -> Option<(ReplicaId, &Worktree)> {
        if let Some(worktree) = self.worktrees.get_mut(&worktree_id) {
            if access_token == worktree.access_token {
                if let Some(connection) = self.connections.get_mut(&connection_id) {
                    connection.worktrees.insert(worktree_id);
                }

                let mut replica_id = 1;
                while worktree.active_replica_ids.contains(&replica_id) {
                    replica_id += 1;
                }
                worktree.active_replica_ids.insert(replica_id);
                worktree
                    .guest_connection_ids
                    .insert(connection_id, replica_id);
                Some((replica_id, worktree))
            } else {
                None
            }
        } else {
            None
        }
    }

    fn read_worktree(
        &self,
        worktree_id: u64,
        connection_id: ConnectionId,
    ) -> tide::Result<&Worktree> {
        let worktree = self
            .worktrees
            .get(&worktree_id)
            .ok_or_else(|| anyhow!("worktree not found"))?;

        if worktree.host_connection_id == Some(connection_id)
            || worktree.guest_connection_ids.contains_key(&connection_id)
        {
            Ok(worktree)
        } else {
            Err(anyhow!(
                "{} is not a member of worktree {}",
                connection_id,
                worktree_id
            ))?
        }
    }

    fn write_worktree(
        &mut self,
        worktree_id: u64,
        connection_id: ConnectionId,
    ) -> tide::Result<&mut Worktree> {
        let worktree = self
            .worktrees
            .get_mut(&worktree_id)
            .ok_or_else(|| anyhow!("worktree not found"))?;

        if worktree.host_connection_id == Some(connection_id)
            || worktree.guest_connection_ids.contains_key(&connection_id)
        {
            Ok(worktree)
        } else {
            Err(anyhow!(
                "{} is not a member of worktree {}",
                connection_id,
                worktree_id
            ))?
        }
    }
}

impl Worktree {
    pub fn connection_ids(&self) -> Vec<ConnectionId> {
        self.guest_connection_ids
            .keys()
            .copied()
            .chain(self.host_connection_id)
            .collect()
    }

    fn host_connection_id(&self) -> tide::Result<ConnectionId> {
        Ok(self
            .host_connection_id
            .ok_or_else(|| anyhow!("host disconnected from worktree"))?)
    }
}

impl Channel {
    fn connection_ids(&self) -> Vec<ConnectionId> {
        self.connection_ids.iter().copied().collect()
    }
}

pub fn add_routes(app: &mut tide::Server<Arc<AppState>>, rpc: &Arc<Peer>) {
    let server = Server::new(app.state().clone(), rpc.clone(), None);
    app.at("/rpc").with(auth::VerifyToken).get(move |request: Request<Arc<AppState>>| {
        let user_id = request.ext::<UserId>().copied();
        let server = server.clone();
        async move {
            const WEBSOCKET_GUID: &str = "258EAFA5-E914-47DA-95CA-C5AB0DC85B11";

            let connection_upgrade = header_contains_ignore_case(&request, CONNECTION, "upgrade");
            let upgrade_to_websocket = header_contains_ignore_case(&request, UPGRADE, "websocket");
            let upgrade_requested = connection_upgrade && upgrade_to_websocket;

            if !upgrade_requested {
                return Ok(Response::new(StatusCode::UpgradeRequired));
            }

            let header = match request.header("Sec-Websocket-Key") {
                Some(h) => h.as_str(),
                None => return Err(anyhow!("expected sec-websocket-key"))?,
            };

            let mut response = Response::new(StatusCode::SwitchingProtocols);
            response.insert_header(UPGRADE, "websocket");
            response.insert_header(CONNECTION, "Upgrade");
            let hash = Sha1::new().chain(header).chain(WEBSOCKET_GUID).finalize();
            response.insert_header("Sec-Websocket-Accept", base64::encode(&hash[..]));
            response.insert_header("Sec-Websocket-Version", "13");

            let http_res: &mut tide::http::Response = response.as_mut();
            let upgrade_receiver = http_res.recv_upgrade().await;
            let addr = request.remote().unwrap_or("unknown").to_string();
            let user_id = user_id.ok_or_else(|| anyhow!("user_id is not present on request. ensure auth::VerifyToken middleware is present"))?;
            task::spawn(async move {
                if let Some(stream) = upgrade_receiver.await {
                    let stream = WebSocketStream::from_raw_socket(stream, Role::Server, None).await;
                    server.handle_connection(stream, addr, user_id).await;
                }
            });

            Ok(response)
        }
    });
}

fn header_contains_ignore_case<T>(
    request: &tide::Request<T>,
    header_name: HeaderName,
    value: &str,
) -> bool {
    request
        .header(header_name)
        .map(|h| {
            h.as_str()
                .split(',')
                .any(|s| s.trim().eq_ignore_ascii_case(value.trim()))
        })
        .unwrap_or(false)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        auth,
        db::{tests::TestDb, UserId},
        github, AppState, Config,
    };
    use async_std::{sync::RwLockReadGuard, task};
    use gpui::TestAppContext;
    use postage::mpsc;
    use serde_json::json;
    use sqlx::types::time::OffsetDateTime;
    use std::{path::Path, sync::Arc, time::Duration};
    use zed::{
        channel::{Channel, ChannelDetails, ChannelList},
        editor::{Editor, Insert},
        fs::{FakeFs, Fs as _},
        language::LanguageRegistry,
        rpc::Client,
        settings, test,
        user::UserStore,
        worktree::Worktree,
    };
    use zrpc::Peer;

    #[gpui::test]
    async fn test_share_worktree(mut cx_a: TestAppContext, mut cx_b: TestAppContext) {
        let (window_b, _) = cx_b.add_window(|_| EmptyView);
        let settings = cx_b.read(settings::test).1;
        let lang_registry = Arc::new(LanguageRegistry::new());

        // Connect to a server as 2 clients.
        let mut server = TestServer::start().await;
        let (_, client_a) = server.create_client(&mut cx_a, "user_a").await;
        let (_, client_b) = server.create_client(&mut cx_b, "user_b").await;

        cx_a.foreground().forbid_parking();

        // Share a local worktree as client A
        let fs = Arc::new(FakeFs::new());
        fs.insert_tree(
            "/a",
            json!({
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;
        let worktree_a = Worktree::open_local(
            "/a".as_ref(),
            lang_registry.clone(),
            fs,
            &mut cx_a.to_async(),
        )
        .await
        .unwrap();
        worktree_a
            .read_with(&cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let (worktree_id, worktree_token) = worktree_a
            .update(&mut cx_a, |tree, cx| {
                tree.as_local_mut().unwrap().share(client_a.clone(), cx)
            })
            .await
            .unwrap();

        // Join that worktree as client B, and see that a guest has joined as client A.
        let worktree_b = Worktree::open_remote(
            client_b.clone(),
            worktree_id,
            worktree_token,
            lang_registry.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();
        let replica_id_b = worktree_b.read_with(&cx_b, |tree, _| tree.replica_id());
        worktree_a
            .condition(&cx_a, |tree, _| {
                tree.peers()
                    .values()
                    .any(|replica_id| *replica_id == replica_id_b)
            })
            .await;

        // Open the same file as client B and client A.
        let buffer_b = worktree_b
            .update(&mut cx_b, |worktree, cx| worktree.open_buffer("b.txt", cx))
            .await
            .unwrap();
        buffer_b.read_with(&cx_b, |buf, _| assert_eq!(buf.text(), "b-contents"));
        worktree_a.read_with(&cx_a, |tree, cx| assert!(tree.has_open_buffer("b.txt", cx)));
        let buffer_a = worktree_a
            .update(&mut cx_a, |tree, cx| tree.open_buffer("b.txt", cx))
            .await
            .unwrap();

        // Create a selection set as client B and see that selection set as client A.
        let editor_b = cx_b.add_view(window_b, |cx| Editor::for_buffer(buffer_b, settings, cx));
        buffer_a
            .condition(&cx_a, |buffer, _| buffer.selection_sets().count() == 1)
            .await;

        // Edit the buffer as client B and see that edit as client A.
        editor_b.update(&mut cx_b, |editor, cx| {
            editor.insert(&Insert("ok, ".into()), cx)
        });
        buffer_a
            .condition(&cx_a, |buffer, _| buffer.text() == "ok, b-contents")
            .await;

        // Remove the selection set as client B, see those selections disappear as client A.
        cx_b.update(move |_| drop(editor_b));
        buffer_a
            .condition(&cx_a, |buffer, _| buffer.selection_sets().count() == 0)
            .await;

        // Close the buffer as client A, see that the buffer is closed.
        drop(buffer_a);
        worktree_a
            .condition(&cx_a, |tree, cx| !tree.has_open_buffer("b.txt", cx))
            .await;

        // Dropping the worktree removes client B from client A's peers.
        cx_b.update(move |_| drop(worktree_b));
        worktree_a
            .condition(&cx_a, |tree, _| tree.peers().is_empty())
            .await;
    }

    #[gpui::test]
    async fn test_propagate_saves_and_fs_changes_in_shared_worktree(
        mut cx_a: TestAppContext,
        mut cx_b: TestAppContext,
        mut cx_c: TestAppContext,
    ) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::new());

        // Connect to a server as 3 clients.
        let mut server = TestServer::start().await;
        let (_, client_a) = server.create_client(&mut cx_a, "user_a").await;
        let (_, client_b) = server.create_client(&mut cx_b, "user_b").await;
        let (_, client_c) = server.create_client(&mut cx_c, "user_c").await;

        let fs = Arc::new(FakeFs::new());

        // Share a worktree as client A.
        fs.insert_tree(
            "/a",
            json!({
                "file1": "",
                "file2": ""
            }),
        )
        .await;

        let worktree_a = Worktree::open_local(
            "/a".as_ref(),
            lang_registry.clone(),
            fs.clone(),
            &mut cx_a.to_async(),
        )
        .await
        .unwrap();
        worktree_a
            .read_with(&cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let (worktree_id, worktree_token) = worktree_a
            .update(&mut cx_a, |tree, cx| {
                tree.as_local_mut().unwrap().share(client_a.clone(), cx)
            })
            .await
            .unwrap();

        // Join that worktree as clients B and C.
        let worktree_b = Worktree::open_remote(
            client_b.clone(),
            worktree_id,
            worktree_token.clone(),
            lang_registry.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();
        let worktree_c = Worktree::open_remote(
            client_c.clone(),
            worktree_id,
            worktree_token,
            lang_registry.clone(),
            &mut cx_c.to_async(),
        )
        .await
        .unwrap();

        // Open and edit a buffer as both guests B and C.
        let buffer_b = worktree_b
            .update(&mut cx_b, |tree, cx| tree.open_buffer("file1", cx))
            .await
            .unwrap();
        let buffer_c = worktree_c
            .update(&mut cx_c, |tree, cx| tree.open_buffer("file1", cx))
            .await
            .unwrap();
        buffer_b.update(&mut cx_b, |buf, cx| buf.edit([0..0], "i-am-b, ", cx));
        buffer_c.update(&mut cx_c, |buf, cx| buf.edit([0..0], "i-am-c, ", cx));

        // Open and edit that buffer as the host.
        let buffer_a = worktree_a
            .update(&mut cx_a, |tree, cx| tree.open_buffer("file1", cx))
            .await
            .unwrap();

        buffer_a
            .condition(&mut cx_a, |buf, _| buf.text() == "i-am-c, i-am-b, ")
            .await;
        buffer_a.update(&mut cx_a, |buf, cx| {
            buf.edit([buf.len()..buf.len()], "i-am-a", cx)
        });

        // Wait for edits to propagate
        buffer_a
            .condition(&mut cx_a, |buf, _| buf.text() == "i-am-c, i-am-b, i-am-a")
            .await;
        buffer_b
            .condition(&mut cx_b, |buf, _| buf.text() == "i-am-c, i-am-b, i-am-a")
            .await;
        buffer_c
            .condition(&mut cx_c, |buf, _| buf.text() == "i-am-c, i-am-b, i-am-a")
            .await;

        // Edit the buffer as the host and concurrently save as guest B.
        let save_b = buffer_b.update(&mut cx_b, |buf, cx| buf.save(cx).unwrap());
        buffer_a.update(&mut cx_a, |buf, cx| buf.edit([0..0], "hi-a, ", cx));
        save_b.await.unwrap();
        assert_eq!(
            fs.load("/a/file1".as_ref()).await.unwrap(),
            "hi-a, i-am-c, i-am-b, i-am-a"
        );
        buffer_a.read_with(&cx_a, |buf, _| assert!(!buf.is_dirty()));
        buffer_b.read_with(&cx_b, |buf, _| assert!(!buf.is_dirty()));
        buffer_c.condition(&cx_c, |buf, _| !buf.is_dirty()).await;

        // Make changes on host's file system, see those changes on the guests.
        fs.rename("/a/file2".as_ref(), "/a/file3".as_ref())
            .await
            .unwrap();
        fs.insert_file(Path::new("/a/file4"), "4".into())
            .await
            .unwrap();

        worktree_b
            .condition(&cx_b, |tree, _| tree.file_count() == 3)
            .await;
        worktree_c
            .condition(&cx_c, |tree, _| tree.file_count() == 3)
            .await;
        worktree_b.read_with(&cx_b, |tree, _| {
            assert_eq!(
                tree.paths()
                    .map(|p| p.to_string_lossy())
                    .collect::<Vec<_>>(),
                &["file1", "file3", "file4"]
            )
        });
        worktree_c.read_with(&cx_c, |tree, _| {
            assert_eq!(
                tree.paths()
                    .map(|p| p.to_string_lossy())
                    .collect::<Vec<_>>(),
                &["file1", "file3", "file4"]
            )
        });
    }

    #[gpui::test]
    async fn test_buffer_conflict_after_save(mut cx_a: TestAppContext, mut cx_b: TestAppContext) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::new());

        // Connect to a server as 2 clients.
        let mut server = TestServer::start().await;
        let (_, client_a) = server.create_client(&mut cx_a, "user_a").await;
        let (_, client_b) = server.create_client(&mut cx_b, "user_b").await;

        // Share a local worktree as client A
        let fs = Arc::new(FakeFs::new());
        fs.save(Path::new("/a.txt"), &"a-contents".into())
            .await
            .unwrap();
        let worktree_a = Worktree::open_local(
            "/".as_ref(),
            lang_registry.clone(),
            fs,
            &mut cx_a.to_async(),
        )
        .await
        .unwrap();
        worktree_a
            .read_with(&cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let (worktree_id, worktree_token) = worktree_a
            .update(&mut cx_a, |tree, cx| {
                tree.as_local_mut().unwrap().share(client_a.clone(), cx)
            })
            .await
            .unwrap();

        // Join that worktree as client B, and see that a guest has joined as client A.
        let worktree_b = Worktree::open_remote(
            client_b.clone(),
            worktree_id,
            worktree_token,
            lang_registry.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        let buffer_b = worktree_b
            .update(&mut cx_b, |worktree, cx| worktree.open_buffer("a.txt", cx))
            .await
            .unwrap();
        let mtime = buffer_b.read_with(&cx_b, |buf, _| buf.file().unwrap().mtime);

        buffer_b.update(&mut cx_b, |buf, cx| buf.edit([0..0], "world ", cx));
        buffer_b.read_with(&cx_b, |buf, _| {
            assert!(buf.is_dirty());
            assert!(!buf.has_conflict());
        });

        buffer_b
            .update(&mut cx_b, |buf, cx| buf.save(cx))
            .unwrap()
            .await
            .unwrap();
        worktree_b
            .condition(&cx_b, |_, cx| {
                buffer_b.read(cx).file().unwrap().mtime != mtime
            })
            .await;
        buffer_b.read_with(&cx_b, |buf, _| {
            assert!(!buf.is_dirty());
            assert!(!buf.has_conflict());
        });

        buffer_b.update(&mut cx_b, |buf, cx| buf.edit([0..0], "hello ", cx));
        buffer_b.read_with(&cx_b, |buf, _| {
            assert!(buf.is_dirty());
            assert!(!buf.has_conflict());
        });
    }

    #[gpui::test]
    async fn test_editing_while_guest_opens_buffer(
        mut cx_a: TestAppContext,
        mut cx_b: TestAppContext,
    ) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::new());

        // Connect to a server as 2 clients.
        let mut server = TestServer::start().await;
        let (_, client_a) = server.create_client(&mut cx_a, "user_a").await;
        let (_, client_b) = server.create_client(&mut cx_b, "user_b").await;

        // Share a local worktree as client A
        let fs = Arc::new(FakeFs::new());
        fs.save(Path::new("/a.txt"), &"a-contents".into())
            .await
            .unwrap();
        let worktree_a = Worktree::open_local(
            "/".as_ref(),
            lang_registry.clone(),
            fs,
            &mut cx_a.to_async(),
        )
        .await
        .unwrap();
        worktree_a
            .read_with(&cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let (worktree_id, worktree_token) = worktree_a
            .update(&mut cx_a, |tree, cx| {
                tree.as_local_mut().unwrap().share(client_a.clone(), cx)
            })
            .await
            .unwrap();

        // Join that worktree as client B, and see that a guest has joined as client A.
        let worktree_b = Worktree::open_remote(
            client_b.clone(),
            worktree_id,
            worktree_token,
            lang_registry.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();

        let buffer_a = worktree_a
            .update(&mut cx_a, |tree, cx| tree.open_buffer("a.txt", cx))
            .await
            .unwrap();
        let buffer_b = cx_b
            .background()
            .spawn(worktree_b.update(&mut cx_b, |worktree, cx| worktree.open_buffer("a.txt", cx)));

        task::yield_now().await;
        buffer_a.update(&mut cx_a, |buf, cx| buf.edit([0..0], "z", cx));

        let text = buffer_a.read_with(&cx_a, |buf, _| buf.text());
        let buffer_b = buffer_b.await.unwrap();
        buffer_b.condition(&cx_b, |buf, _| buf.text() == text).await;
    }

    #[gpui::test]
    async fn test_peer_disconnection(mut cx_a: TestAppContext, cx_b: TestAppContext) {
        cx_a.foreground().forbid_parking();
        let lang_registry = Arc::new(LanguageRegistry::new());

        // Connect to a server as 2 clients.
        let mut server = TestServer::start().await;
        let (_, client_a) = server.create_client(&mut cx_a, "user_a").await;
        let (_, client_b) = server.create_client(&mut cx_a, "user_b").await;

        // Share a local worktree as client A
        let fs = Arc::new(FakeFs::new());
        fs.insert_tree(
            "/a",
            json!({
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;
        let worktree_a = Worktree::open_local(
            "/a".as_ref(),
            lang_registry.clone(),
            fs,
            &mut cx_a.to_async(),
        )
        .await
        .unwrap();
        worktree_a
            .read_with(&cx_a, |tree, _| tree.as_local().unwrap().scan_complete())
            .await;
        let (worktree_id, worktree_token) = worktree_a
            .update(&mut cx_a, |tree, cx| {
                tree.as_local_mut().unwrap().share(client_a.clone(), cx)
            })
            .await
            .unwrap();

        // Join that worktree as client B, and see that a guest has joined as client A.
        let _worktree_b = Worktree::open_remote(
            client_b.clone(),
            worktree_id,
            worktree_token,
            lang_registry.clone(),
            &mut cx_b.to_async(),
        )
        .await
        .unwrap();
        worktree_a
            .condition(&cx_a, |tree, _| tree.peers().len() == 1)
            .await;

        // Drop client B's connection and ensure client A observes client B leaving the worktree.
        client_b.disconnect().await.unwrap();
        worktree_a
            .condition(&cx_a, |tree, _| tree.peers().len() == 0)
            .await;
    }

    #[gpui::test]
    async fn test_basic_chat(mut cx_a: TestAppContext, mut cx_b: TestAppContext) {
        cx_a.foreground().forbid_parking();

        // Connect to a server as 2 clients.
        let mut server = TestServer::start().await;
        let (user_id_a, client_a) = server.create_client(&mut cx_a, "user_a").await;
        let (user_id_b, client_b) = server.create_client(&mut cx_b, "user_b").await;

        // Create an org that includes these 2 users.
        let db = &server.app_state.db;
        let org_id = db.create_org("Test Org", "test-org").await.unwrap();
        db.add_org_member(org_id, user_id_a, false).await.unwrap();
        db.add_org_member(org_id, user_id_b, false).await.unwrap();

        // Create a channel that includes all the users.
        let channel_id = db.create_org_channel(org_id, "test-channel").await.unwrap();
        db.add_channel_member(channel_id, user_id_a, false)
            .await
            .unwrap();
        db.add_channel_member(channel_id, user_id_b, false)
            .await
            .unwrap();
        db.create_channel_message(
            channel_id,
            user_id_b,
            "hello A, it's B.",
            OffsetDateTime::now_utc(),
        )
        .await
        .unwrap();

        let user_store_a = Arc::new(UserStore::new(client_a.clone()));
        let channels_a = cx_a.add_model(|cx| ChannelList::new(user_store_a, client_a, cx));
        channels_a
            .condition(&mut cx_a, |list, _| list.available_channels().is_some())
            .await;
        channels_a.read_with(&cx_a, |list, _| {
            assert_eq!(
                list.available_channels().unwrap(),
                &[ChannelDetails {
                    id: channel_id.to_proto(),
                    name: "test-channel".to_string()
                }]
            )
        });
        let channel_a = channels_a.update(&mut cx_a, |this, cx| {
            this.get_channel(channel_id.to_proto(), cx).unwrap()
        });
        channel_a.read_with(&cx_a, |channel, _| assert!(channel.messages().is_empty()));
        channel_a
            .condition(&cx_a, |channel, _| {
                channel_messages(channel)
                    == [("user_b".to_string(), "hello A, it's B.".to_string())]
            })
            .await;

        let user_store_b = Arc::new(UserStore::new(client_b.clone()));
        let channels_b = cx_b.add_model(|cx| ChannelList::new(user_store_b, client_b, cx));
        channels_b
            .condition(&mut cx_b, |list, _| list.available_channels().is_some())
            .await;
        channels_b.read_with(&cx_b, |list, _| {
            assert_eq!(
                list.available_channels().unwrap(),
                &[ChannelDetails {
                    id: channel_id.to_proto(),
                    name: "test-channel".to_string()
                }]
            )
        });

        let channel_b = channels_b.update(&mut cx_b, |this, cx| {
            this.get_channel(channel_id.to_proto(), cx).unwrap()
        });
        channel_b.read_with(&cx_b, |channel, _| assert!(channel.messages().is_empty()));
        channel_b
            .condition(&cx_b, |channel, _| {
                channel_messages(channel)
                    == [("user_b".to_string(), "hello A, it's B.".to_string())]
            })
            .await;

        channel_a
            .update(&mut cx_a, |channel, cx| {
                channel
                    .send_message("oh, hi B.".to_string(), cx)
                    .unwrap()
                    .detach();
                let task = channel.send_message("sup".to_string(), cx).unwrap();
                assert_eq!(
                    channel
                        .pending_messages()
                        .iter()
                        .map(|m| &m.body)
                        .collect::<Vec<_>>(),
                    &["oh, hi B.", "sup"]
                );
                task
            })
            .await
            .unwrap();

        channel_a
            .condition(&cx_a, |channel, _| channel.pending_messages().is_empty())
            .await;
        channel_b
            .condition(&cx_b, |channel, _| {
                channel_messages(channel)
                    == [
                        ("user_b".to_string(), "hello A, it's B.".to_string()),
                        ("user_a".to_string(), "oh, hi B.".to_string()),
                        ("user_a".to_string(), "sup".to_string()),
                    ]
            })
            .await;

        assert_eq!(
            server.state().await.channels[&channel_id]
                .connection_ids
                .len(),
            2
        );
        cx_b.update(|_| drop(channel_b));
        server
            .condition(|state| state.channels[&channel_id].connection_ids.len() == 1)
            .await;

        cx_a.update(|_| drop(channel_a));
        server
            .condition(|state| !state.channels.contains_key(&channel_id))
            .await;

        fn channel_messages(channel: &Channel) -> Vec<(String, String)> {
            channel
                .messages()
                .cursor::<(), ()>()
                .map(|m| (m.sender.github_login.clone(), m.body.clone()))
                .collect()
        }
    }

    #[gpui::test]
    async fn test_chat_message_validation(mut cx_a: TestAppContext) {
        cx_a.foreground().forbid_parking();

        let mut server = TestServer::start().await;
        let (user_id_a, client_a) = server.create_client(&mut cx_a, "user_a").await;

        let db = &server.app_state.db;
        let org_id = db.create_org("Test Org", "test-org").await.unwrap();
        let channel_id = db.create_org_channel(org_id, "test-channel").await.unwrap();
        db.add_org_member(org_id, user_id_a, false).await.unwrap();
        db.add_channel_member(channel_id, user_id_a, false)
            .await
            .unwrap();

        let user_store_a = Arc::new(UserStore::new(client_a.clone()));
        let channels_a = cx_a.add_model(|cx| ChannelList::new(user_store_a, client_a, cx));
        channels_a
            .condition(&mut cx_a, |list, _| list.available_channels().is_some())
            .await;
        let channel_a = channels_a.update(&mut cx_a, |this, cx| {
            this.get_channel(channel_id.to_proto(), cx).unwrap()
        });

        // Messages aren't allowed to be too long.
        channel_a
            .update(&mut cx_a, |channel, cx| {
                let long_body = "this is long.\n".repeat(1024);
                channel.send_message(long_body, cx).unwrap()
            })
            .await
            .unwrap_err();

        // Messages aren't allowed to be blank.
        channel_a.update(&mut cx_a, |channel, cx| {
            channel.send_message(String::new(), cx).unwrap_err()
        });

        // Leading and trailing whitespace are trimmed.
        channel_a
            .update(&mut cx_a, |channel, cx| {
                channel
                    .send_message("\n surrounded by whitespace  \n".to_string(), cx)
                    .unwrap()
            })
            .await
            .unwrap();
        assert_eq!(
            db.get_channel_messages(channel_id, 10, None)
                .await
                .unwrap()
                .iter()
                .map(|m| &m.body)
                .collect::<Vec<_>>(),
            &["surrounded by whitespace"]
        );
    }

    struct TestServer {
        peer: Arc<Peer>,
        app_state: Arc<AppState>,
        server: Arc<Server>,
        notifications: mpsc::Receiver<()>,
        _test_db: TestDb,
    }

    impl TestServer {
        async fn start() -> Self {
            let test_db = TestDb::new();
            let app_state = Self::build_app_state(&test_db).await;
            let peer = Peer::new();
            let notifications = mpsc::channel(128);
            let server = Server::new(app_state.clone(), peer.clone(), Some(notifications.0));
            Self {
                peer,
                app_state,
                server,
                notifications: notifications.1,
                _test_db: test_db,
            }
        }

        async fn create_client(
            &mut self,
            cx: &mut TestAppContext,
            name: &str,
        ) -> (UserId, Arc<Client>) {
            let user_id = self.app_state.db.create_user(name, false).await.unwrap();
            let client = Client::new();
            let (client_conn, server_conn) = test::Channel::bidirectional();
            cx.background()
                .spawn(
                    self.server
                        .handle_connection(server_conn, name.to_string(), user_id),
                )
                .detach();
            client
                .add_connection(user_id.to_proto(), client_conn, cx.to_async())
                .await
                .unwrap();
            (user_id, client)
        }

        async fn build_app_state(test_db: &TestDb) -> Arc<AppState> {
            let mut config = Config::default();
            config.session_secret = "a".repeat(32);
            config.database_url = test_db.url.clone();
            let github_client = github::AppClient::test();
            Arc::new(AppState {
                db: test_db.db().clone(),
                handlebars: Default::default(),
                auth_client: auth::build_client("", ""),
                repo_client: github::RepoClient::test(&github_client),
                github_client,
                config,
            })
        }

        async fn state<'a>(&'a self) -> RwLockReadGuard<'a, ServerState> {
            self.server.state.read().await
        }

        async fn condition<F>(&mut self, mut predicate: F)
        where
            F: FnMut(&ServerState) -> bool,
        {
            async_std::future::timeout(Duration::from_millis(500), async {
                while !(predicate)(&*self.server.state.read().await) {
                    self.notifications.recv().await;
                }
            })
            .await
            .expect("condition timed out");
        }
    }

    impl Drop for TestServer {
        fn drop(&mut self) {
            task::block_on(self.peer.reset());
        }
    }

    struct EmptyView;

    impl gpui::Entity for EmptyView {
        type Event = ();
    }

    impl gpui::View for EmptyView {
        fn ui_name() -> &'static str {
            "empty view"
        }

        fn render(&mut self, _: &mut gpui::RenderContext<Self>) -> gpui::ElementBox {
            gpui::Element::boxed(gpui::elements::Empty)
        }
    }
}
