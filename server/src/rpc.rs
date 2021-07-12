use crate::auth::{self, UserId};

use super::{auth::PeerExt as _, AppState};
use anyhow::anyhow;
use async_std::task;
use async_tungstenite::{
    tungstenite::{protocol::Role, Error as WebSocketError, Message as WebSocketMessage},
    WebSocketStream,
};
use sha1::{Digest as _, Sha1};
use std::{
    collections::{HashMap, HashSet},
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
use zed_rpc::{
    auth::random_token,
    proto::{self, EnvelopedMessage},
    ConnectionId, Peer, Router, TypedEnvelope,
};

type ReplicaId = u16;

#[derive(Default)]
pub struct State {
    connections: HashMap<ConnectionId, ConnectionState>,
    pub worktrees: HashMap<u64, WorktreeState>,
    next_worktree_id: u64,
}

struct ConnectionState {
    _user_id: i32,
    worktrees: HashSet<u64>,
}

pub struct WorktreeState {
    host_connection_id: Option<ConnectionId>,
    guest_connection_ids: HashMap<ConnectionId, ReplicaId>,
    active_replica_ids: HashSet<ReplicaId>,
    access_token: String,
    root_name: String,
    entries: HashMap<u64, proto::Entry>,
}

impl WorktreeState {
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

impl State {
    // Add a new connection associated with a given user.
    pub fn add_connection(&mut self, connection_id: ConnectionId, _user_id: i32) {
        self.connections.insert(
            connection_id,
            ConnectionState {
                _user_id,
                worktrees: Default::default(),
            },
        );
    }

    // Remove the given connection and its association with any worktrees.
    pub fn remove_connection(&mut self, connection_id: ConnectionId) -> Vec<u64> {
        let mut worktree_ids = Vec::new();
        if let Some(connection_state) = self.connections.remove(&connection_id) {
            for worktree_id in connection_state.worktrees {
                if let Some(worktree) = self.worktrees.get_mut(&worktree_id) {
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

    // Add the given connection as a guest of the given worktree
    pub fn join_worktree(
        &mut self,
        connection_id: ConnectionId,
        worktree_id: u64,
        access_token: &str,
    ) -> Option<(ReplicaId, &WorktreeState)> {
        if let Some(worktree_state) = self.worktrees.get_mut(&worktree_id) {
            if access_token == worktree_state.access_token {
                if let Some(connection_state) = self.connections.get_mut(&connection_id) {
                    connection_state.worktrees.insert(worktree_id);
                }

                let mut replica_id = 1;
                while worktree_state.active_replica_ids.contains(&replica_id) {
                    replica_id += 1;
                }
                worktree_state.active_replica_ids.insert(replica_id);
                worktree_state
                    .guest_connection_ids
                    .insert(connection_id, replica_id);
                Some((replica_id, worktree_state))
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
    ) -> tide::Result<&WorktreeState> {
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
    ) -> tide::Result<&mut WorktreeState> {
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

trait MessageHandler<'a, M: proto::EnvelopedMessage> {
    type Output: 'a + Send + Future<Output = tide::Result<()>>;

    fn handle(
        &self,
        message: TypedEnvelope<M>,
        rpc: &'a Arc<Peer>,
        app_state: &'a Arc<AppState>,
    ) -> Self::Output;
}

impl<'a, M, F, Fut> MessageHandler<'a, M> for F
where
    M: proto::EnvelopedMessage,
    F: Fn(TypedEnvelope<M>, &'a Arc<Peer>, &'a Arc<AppState>) -> Fut,
    Fut: 'a + Send + Future<Output = tide::Result<()>>,
{
    type Output = Fut;

    fn handle(
        &self,
        message: TypedEnvelope<M>,
        rpc: &'a Arc<Peer>,
        app_state: &'a Arc<AppState>,
    ) -> Self::Output {
        (self)(message, rpc, app_state)
    }
}

fn on_message<M, H>(router: &mut Router, rpc: &Arc<Peer>, app_state: &Arc<AppState>, handler: H)
where
    M: EnvelopedMessage,
    H: 'static + Clone + Send + Sync + for<'a> MessageHandler<'a, M>,
{
    let rpc = rpc.clone();
    let handler = handler.clone();
    let app_state = app_state.clone();
    router.add_message_handler(move |message| {
        let rpc = rpc.clone();
        let handler = handler.clone();
        let app_state = app_state.clone();
        async move {
            let sender_id = message.sender_id;
            let message_id = message.message_id;
            let start_time = Instant::now();
            log::info!(
                "RPC message received. id: {}.{}, type:{}",
                sender_id,
                message_id,
                M::NAME
            );
            if let Err(err) = handler.handle(message, &rpc, &app_state).await {
                log::error!("error handling message: {:?}", err);
            } else {
                log::info!(
                    "RPC message handled. id:{}.{}, duration:{:?}",
                    sender_id,
                    message_id,
                    start_time.elapsed()
                );
            }

            Ok(())
        }
    });
}

pub fn add_rpc_routes(router: &mut Router, state: &Arc<AppState>, rpc: &Arc<Peer>) {
    on_message(router, rpc, state, share_worktree);
    on_message(router, rpc, state, join_worktree);
    on_message(router, rpc, state, update_worktree);
    on_message(router, rpc, state, close_worktree);
    on_message(router, rpc, state, open_buffer);
    on_message(router, rpc, state, close_buffer);
    on_message(router, rpc, state, update_buffer);
    on_message(router, rpc, state, buffer_saved);
    on_message(router, rpc, state, save_buffer);
}

pub fn add_routes(app: &mut tide::Server<Arc<AppState>>, rpc: &Arc<Peer>) {
    let mut router = Router::new();
    add_rpc_routes(&mut router, app.state(), rpc);
    let router = Arc::new(router);

    let rpc = rpc.clone();
    app.at("/rpc").with(auth::VerifyToken).get(move |request: Request<Arc<AppState>>| {
        let user_id = request.ext::<UserId>().copied();
        let rpc = rpc.clone();
        let router = router.clone();
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
            let state = request.state().clone();
            let user_id = user_id.ok_or_else(|| anyhow!("user_id is not present on request. ensure auth::VerifyToken middleware is present"))?.0;
            task::spawn(async move {
                if let Some(stream) = upgrade_receiver.await {
                    let stream = WebSocketStream::from_raw_socket(stream, Role::Server, None).await;
                    handle_connection(rpc, router, state, addr, stream, user_id).await;
                }
            });

            Ok(response)
        }
    });
}

pub async fn handle_connection<Conn>(
    rpc: Arc<Peer>,
    router: Arc<Router>,
    state: Arc<AppState>,
    addr: String,
    stream: Conn,
    user_id: i32,
) where
    Conn: 'static
        + futures::Sink<WebSocketMessage, Error = WebSocketError>
        + futures::Stream<Item = Result<WebSocketMessage, WebSocketError>>
        + Send
        + Unpin,
{
    log::info!("accepted rpc connection: {:?}", addr);
    let (connection_id, handle_io, handle_messages) = rpc.add_connection(stream, router).await;
    state
        .rpc
        .write()
        .await
        .add_connection(connection_id, user_id);

    let handle_messages = async move {
        handle_messages.await;
        Ok(())
    };

    if let Err(e) = futures::try_join!(handle_messages, handle_io) {
        log::error!("error handling rpc connection {:?} - {:?}", addr, e);
    }

    log::info!("closing connection to {:?}", addr);
    if let Err(e) = rpc.sign_out(connection_id, &state).await {
        log::error!("error signing out connection {:?} - {:?}", addr, e);
    }
}

async fn share_worktree(
    mut request: TypedEnvelope<proto::ShareWorktree>,
    rpc: &Arc<Peer>,
    state: &Arc<AppState>,
) -> tide::Result<()> {
    let mut state = state.rpc.write().await;
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
        WorktreeState {
            host_connection_id: Some(request.sender_id),
            guest_connection_ids: Default::default(),
            active_replica_ids: Default::default(),
            access_token: access_token.clone(),
            root_name: mem::take(&mut worktree.root_name),
            entries,
        },
    );

    rpc.respond(
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
    request: TypedEnvelope<proto::OpenWorktree>,
    rpc: &Arc<Peer>,
    state: &Arc<AppState>,
) -> tide::Result<()> {
    let worktree_id = request.payload.worktree_id;
    let access_token = &request.payload.access_token;

    let mut state = state.rpc.write().await;
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
            rpc.send(
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
        rpc.respond(
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
        rpc.respond(
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
    request: TypedEnvelope<proto::UpdateWorktree>,
    rpc: &Arc<Peer>,
    state: &Arc<AppState>,
) -> tide::Result<()> {
    {
        let mut state = state.rpc.write().await;
        let worktree = state.write_worktree(request.payload.worktree_id, request.sender_id)?;
        for entry_id in &request.payload.removed_entries {
            worktree.entries.remove(&entry_id);
        }

        for entry in &request.payload.updated_entries {
            worktree.entries.insert(entry.id, entry.clone());
        }
    }

    broadcast_in_worktree(request.payload.worktree_id, request, rpc, state).await?;
    Ok(())
}

async fn close_worktree(
    request: TypedEnvelope<proto::CloseWorktree>,
    rpc: &Arc<Peer>,
    state: &Arc<AppState>,
) -> tide::Result<()> {
    let connection_ids;
    {
        let mut state = state.rpc.write().await;
        let worktree = state.write_worktree(request.payload.worktree_id, request.sender_id)?;
        connection_ids = worktree.connection_ids();
        if worktree.host_connection_id == Some(request.sender_id) {
            worktree.host_connection_id = None;
        } else if let Some(replica_id) = worktree.guest_connection_ids.remove(&request.sender_id) {
            worktree.active_replica_ids.remove(&replica_id);
        }
    }

    broadcast(request.sender_id, connection_ids, |conn_id| {
        rpc.send(
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
    request: TypedEnvelope<proto::OpenBuffer>,
    rpc: &Arc<Peer>,
    state: &Arc<AppState>,
) -> tide::Result<()> {
    let receipt = request.receipt();
    let worktree_id = request.payload.worktree_id;
    let host_connection_id = state
        .rpc
        .read()
        .await
        .read_worktree(worktree_id, request.sender_id)?
        .host_connection_id()?;

    let response = rpc
        .forward_request(request.sender_id, host_connection_id, request.payload)
        .await?;
    rpc.respond(receipt, response).await?;
    Ok(())
}

async fn close_buffer(
    request: TypedEnvelope<proto::CloseBuffer>,
    rpc: &Arc<Peer>,
    state: &Arc<AppState>,
) -> tide::Result<()> {
    let host_connection_id = state
        .rpc
        .read()
        .await
        .read_worktree(request.payload.worktree_id, request.sender_id)?
        .host_connection_id()?;

    rpc.forward_send(request.sender_id, host_connection_id, request.payload)
        .await?;

    Ok(())
}

async fn save_buffer(
    request: TypedEnvelope<proto::SaveBuffer>,
    rpc: &Arc<Peer>,
    state: &Arc<AppState>,
) -> tide::Result<()> {
    let host;
    let guests;
    {
        let state = state.rpc.read().await;
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
    let response = rpc
        .forward_request(sender, host, request.payload.clone())
        .await?;

    broadcast(host, guests, |conn_id| {
        let response = response.clone();
        async move {
            if conn_id == sender {
                rpc.respond(receipt, response).await
            } else {
                rpc.forward_send(host, conn_id, response).await
            }
        }
    })
    .await?;

    Ok(())
}

async fn update_buffer(
    request: TypedEnvelope<proto::UpdateBuffer>,
    rpc: &Arc<Peer>,
    state: &Arc<AppState>,
) -> tide::Result<()> {
    broadcast_in_worktree(request.payload.worktree_id, request, rpc, state).await
}

async fn buffer_saved(
    request: TypedEnvelope<proto::BufferSaved>,
    rpc: &Arc<Peer>,
    state: &Arc<AppState>,
) -> tide::Result<()> {
    broadcast_in_worktree(request.payload.worktree_id, request, rpc, state).await
}

async fn broadcast_in_worktree<T: proto::EnvelopedMessage>(
    worktree_id: u64,
    request: TypedEnvelope<T>,
    rpc: &Arc<Peer>,
    state: &Arc<AppState>,
) -> tide::Result<()> {
    let connection_ids = state
        .rpc
        .read()
        .await
        .read_worktree(worktree_id, request.sender_id)?
        .connection_ids();

    broadcast(request.sender_id, connection_ids, |conn_id| {
        rpc.forward_send(request.sender_id, conn_id, request.payload.clone())
    })
    .await?;

    Ok(())
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
