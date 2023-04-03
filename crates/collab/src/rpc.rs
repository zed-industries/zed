mod connection_pool;

use crate::{
    auth,
    db::{self, Database, ProjectId, RoomId, ServerId, User, UserId},
    executor::Executor,
    AppState, Result,
};
use anyhow::anyhow;
use async_tungstenite::tungstenite::{
    protocol::CloseFrame as TungsteniteCloseFrame, Message as TungsteniteMessage,
};
use axum::{
    body::Body,
    extract::{
        ws::{CloseFrame as AxumCloseFrame, Message as AxumMessage},
        ConnectInfo, WebSocketUpgrade,
    },
    headers::{Header, HeaderName},
    http::StatusCode,
    middleware,
    response::IntoResponse,
    routing::get,
    Extension, Router, TypedHeader,
};
use collections::{HashMap, HashSet};
pub use connection_pool::ConnectionPool;
use futures::{
    channel::oneshot,
    future::{self, BoxFuture},
    stream::FuturesUnordered,
    FutureExt, SinkExt, StreamExt, TryStreamExt,
};
use lazy_static::lazy_static;
use prometheus::{register_int_gauge, IntGauge};
use rpc::{
    proto::{self, AnyTypedEnvelope, EntityMessage, EnvelopedMessage, RequestMessage},
    Connection, ConnectionId, Peer, Receipt, TypedEnvelope,
};
use serde::{Serialize, Serializer};
use std::{
    any::TypeId,
    fmt,
    future::Future,
    marker::PhantomData,
    mem,
    net::SocketAddr,
    ops::{Deref, DerefMut},
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering::SeqCst},
        Arc,
    },
    time::Duration,
};
use tokio::sync::{watch, Semaphore};
use tower::ServiceBuilder;
use tracing::{info_span, instrument, Instrument};

pub const RECONNECT_TIMEOUT: Duration = Duration::from_secs(30);
pub const CLEANUP_TIMEOUT: Duration = Duration::from_secs(10);

lazy_static! {
    static ref METRIC_CONNECTIONS: IntGauge =
        register_int_gauge!("connections", "number of connections").unwrap();
    static ref METRIC_SHARED_PROJECTS: IntGauge = register_int_gauge!(
        "shared_projects",
        "number of open projects with one or more guests"
    )
    .unwrap();
}

type MessageHandler =
    Box<dyn Send + Sync + Fn(Box<dyn AnyTypedEnvelope>, Session) -> BoxFuture<'static, ()>>;

struct Response<R> {
    peer: Arc<Peer>,
    receipt: Receipt<R>,
    responded: Arc<AtomicBool>,
}

impl<R: RequestMessage> Response<R> {
    fn send(self, payload: R::Response) -> Result<()> {
        self.responded.store(true, SeqCst);
        self.peer.respond(self.receipt, payload)?;
        Ok(())
    }
}

#[derive(Clone)]
struct Session {
    user_id: UserId,
    connection_id: ConnectionId,
    db: Arc<tokio::sync::Mutex<DbHandle>>,
    peer: Arc<Peer>,
    connection_pool: Arc<parking_lot::Mutex<ConnectionPool>>,
    live_kit_client: Option<Arc<dyn live_kit_server::api::Client>>,
    executor: Executor,
}

impl Session {
    async fn db(&self) -> tokio::sync::MutexGuard<DbHandle> {
        #[cfg(test)]
        tokio::task::yield_now().await;
        let guard = self.db.lock().await;
        #[cfg(test)]
        tokio::task::yield_now().await;
        guard
    }

    async fn connection_pool(&self) -> ConnectionPoolGuard<'_> {
        #[cfg(test)]
        tokio::task::yield_now().await;
        let guard = self.connection_pool.lock();
        ConnectionPoolGuard {
            guard,
            _not_send: PhantomData,
        }
    }
}

impl fmt::Debug for Session {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Session")
            .field("user_id", &self.user_id)
            .field("connection_id", &self.connection_id)
            .finish()
    }
}

struct DbHandle(Arc<Database>);

impl Deref for DbHandle {
    type Target = Database;

    fn deref(&self) -> &Self::Target {
        self.0.as_ref()
    }
}

pub struct Server {
    id: parking_lot::Mutex<ServerId>,
    peer: Arc<Peer>,
    pub(crate) connection_pool: Arc<parking_lot::Mutex<ConnectionPool>>,
    app_state: Arc<AppState>,
    executor: Executor,
    handlers: HashMap<TypeId, MessageHandler>,
    teardown: watch::Sender<()>,
}

pub(crate) struct ConnectionPoolGuard<'a> {
    guard: parking_lot::MutexGuard<'a, ConnectionPool>,
    _not_send: PhantomData<Rc<()>>,
}

#[derive(Serialize)]
pub struct ServerSnapshot<'a> {
    peer: &'a Peer,
    #[serde(serialize_with = "serialize_deref")]
    connection_pool: ConnectionPoolGuard<'a>,
}

pub fn serialize_deref<S, T, U>(value: &T, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
    T: Deref<Target = U>,
    U: Serialize,
{
    Serialize::serialize(value.deref(), serializer)
}

impl Server {
    pub fn new(id: ServerId, app_state: Arc<AppState>, executor: Executor) -> Arc<Self> {
        let mut server = Self {
            id: parking_lot::Mutex::new(id),
            peer: Peer::new(id.0 as u32),
            app_state,
            executor,
            connection_pool: Default::default(),
            handlers: Default::default(),
            teardown: watch::channel(()).0,
        };

        server
            .add_request_handler(ping)
            .add_request_handler(create_room)
            .add_request_handler(join_room)
            .add_request_handler(rejoin_room)
            .add_request_handler(leave_room)
            .add_request_handler(call)
            .add_request_handler(cancel_call)
            .add_message_handler(decline_call)
            .add_request_handler(update_participant_location)
            .add_request_handler(share_project)
            .add_message_handler(unshare_project)
            .add_request_handler(join_project)
            .add_message_handler(leave_project)
            .add_request_handler(update_project)
            .add_request_handler(update_worktree)
            .add_message_handler(start_language_server)
            .add_message_handler(update_language_server)
            .add_message_handler(update_diagnostic_summary)
            .add_request_handler(forward_project_request::<proto::GetHover>)
            .add_request_handler(forward_project_request::<proto::GetDefinition>)
            .add_request_handler(forward_project_request::<proto::GetTypeDefinition>)
            .add_request_handler(forward_project_request::<proto::GetReferences>)
            .add_request_handler(forward_project_request::<proto::SearchProject>)
            .add_request_handler(forward_project_request::<proto::GetDocumentHighlights>)
            .add_request_handler(forward_project_request::<proto::GetProjectSymbols>)
            .add_request_handler(forward_project_request::<proto::OpenBufferForSymbol>)
            .add_request_handler(forward_project_request::<proto::OpenBufferById>)
            .add_request_handler(forward_project_request::<proto::OpenBufferByPath>)
            .add_request_handler(forward_project_request::<proto::GetCompletions>)
            .add_request_handler(forward_project_request::<proto::ApplyCompletionAdditionalEdits>)
            .add_request_handler(forward_project_request::<proto::GetCodeActions>)
            .add_request_handler(forward_project_request::<proto::ApplyCodeAction>)
            .add_request_handler(forward_project_request::<proto::PrepareRename>)
            .add_request_handler(forward_project_request::<proto::PerformRename>)
            .add_request_handler(forward_project_request::<proto::ReloadBuffers>)
            .add_request_handler(forward_project_request::<proto::SynchronizeBuffers>)
            .add_request_handler(forward_project_request::<proto::FormatBuffers>)
            .add_request_handler(forward_project_request::<proto::CreateProjectEntry>)
            .add_request_handler(forward_project_request::<proto::RenameProjectEntry>)
            .add_request_handler(forward_project_request::<proto::CopyProjectEntry>)
            .add_request_handler(forward_project_request::<proto::DeleteProjectEntry>)
            .add_message_handler(create_buffer_for_peer)
            .add_request_handler(update_buffer)
            .add_message_handler(update_buffer_file)
            .add_message_handler(buffer_reloaded)
            .add_message_handler(buffer_saved)
            .add_request_handler(save_buffer)
            .add_request_handler(get_users)
            .add_request_handler(fuzzy_search_users)
            .add_request_handler(request_contact)
            .add_request_handler(remove_contact)
            .add_request_handler(respond_to_contact_request)
            .add_request_handler(follow)
            .add_message_handler(unfollow)
            .add_message_handler(update_followers)
            .add_message_handler(update_diff_base)
            .add_request_handler(get_private_user_info);

        Arc::new(server)
    }

    pub async fn start(&self) -> Result<()> {
        let server_id = *self.id.lock();
        let app_state = self.app_state.clone();
        let peer = self.peer.clone();
        let timeout = self.executor.sleep(CLEANUP_TIMEOUT);
        let pool = self.connection_pool.clone();
        let live_kit_client = self.app_state.live_kit_client.clone();

        let span = info_span!("start server");
        self.executor.spawn_detached(
            async move {
                tracing::info!("waiting for cleanup timeout");
                timeout.await;
                tracing::info!("cleanup timeout expired, retrieving stale rooms");
                if let Some(room_ids) = app_state
                    .db
                    .stale_room_ids(&app_state.config.zed_environment, server_id)
                    .await
                    .trace_err()
                {
                    tracing::info!(stale_room_count = room_ids.len(), "retrieved stale rooms");
                    for room_id in room_ids {
                        let mut contacts_to_update = HashSet::default();
                        let mut canceled_calls_to_user_ids = Vec::new();
                        let mut live_kit_room = String::new();
                        let mut delete_live_kit_room = false;

                        if let Some(mut refreshed_room) = app_state
                            .db
                            .refresh_room(room_id, server_id)
                            .await
                            .trace_err()
                        {
                            tracing::info!(
                                room_id = room_id.0,
                                new_participant_count = refreshed_room.room.participants.len(),
                                "refreshed room"
                            );
                            room_updated(&refreshed_room.room, &peer);
                            contacts_to_update
                                .extend(refreshed_room.stale_participant_user_ids.iter().copied());
                            contacts_to_update
                                .extend(refreshed_room.canceled_calls_to_user_ids.iter().copied());
                            canceled_calls_to_user_ids =
                                mem::take(&mut refreshed_room.canceled_calls_to_user_ids);
                            live_kit_room = mem::take(&mut refreshed_room.room.live_kit_room);
                            delete_live_kit_room = refreshed_room.room.participants.is_empty();
                        }

                        {
                            let pool = pool.lock();
                            for canceled_user_id in canceled_calls_to_user_ids {
                                for connection_id in pool.user_connection_ids(canceled_user_id) {
                                    peer.send(
                                        connection_id,
                                        proto::CallCanceled {
                                            room_id: room_id.to_proto(),
                                        },
                                    )
                                    .trace_err();
                                }
                            }
                        }

                        for user_id in contacts_to_update {
                            let busy = app_state.db.is_user_busy(user_id).await.trace_err();
                            let contacts = app_state.db.get_contacts(user_id).await.trace_err();
                            if let Some((busy, contacts)) = busy.zip(contacts) {
                                let pool = pool.lock();
                                let updated_contact = contact_for_user(user_id, false, busy, &pool);
                                for contact in contacts {
                                    if let db::Contact::Accepted {
                                        user_id: contact_user_id,
                                        ..
                                    } = contact
                                    {
                                        for contact_conn_id in
                                            pool.user_connection_ids(contact_user_id)
                                        {
                                            peer.send(
                                                contact_conn_id,
                                                proto::UpdateContacts {
                                                    contacts: vec![updated_contact.clone()],
                                                    remove_contacts: Default::default(),
                                                    incoming_requests: Default::default(),
                                                    remove_incoming_requests: Default::default(),
                                                    outgoing_requests: Default::default(),
                                                    remove_outgoing_requests: Default::default(),
                                                },
                                            )
                                            .trace_err();
                                        }
                                    }
                                }
                            }
                        }

                        if let Some(live_kit) = live_kit_client.as_ref() {
                            if delete_live_kit_room {
                                live_kit.delete_room(live_kit_room).await.trace_err();
                            }
                        }
                    }
                }

                app_state
                    .db
                    .delete_stale_servers(&app_state.config.zed_environment, server_id)
                    .await
                    .trace_err();
            }
            .instrument(span),
        );
        Ok(())
    }

    pub fn teardown(&self) {
        self.peer.teardown();
        self.connection_pool.lock().reset();
        let _ = self.teardown.send(());
    }

    #[cfg(test)]
    pub fn reset(&self, id: ServerId) {
        self.teardown();
        *self.id.lock() = id;
        self.peer.reset(id.0 as u32);
    }

    #[cfg(test)]
    pub fn id(&self) -> ServerId {
        *self.id.lock()
    }

    fn add_handler<F, Fut, M>(&mut self, handler: F) -> &mut Self
    where
        F: 'static + Send + Sync + Fn(TypedEnvelope<M>, Session) -> Fut,
        Fut: 'static + Send + Future<Output = Result<()>>,
        M: EnvelopedMessage,
    {
        let prev_handler = self.handlers.insert(
            TypeId::of::<M>(),
            Box::new(move |envelope, session| {
                let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                let span = info_span!(
                    "handle message",
                    payload_type = envelope.payload_type_name()
                );
                span.in_scope(|| {
                    tracing::info!(
                        payload_type = envelope.payload_type_name(),
                        "message received"
                    );
                });
                let future = (handler)(*envelope, session);
                async move {
                    if let Err(error) = future.await {
                        tracing::error!(%error, "error handling message");
                    }
                }
                .instrument(span)
                .boxed()
            }),
        );
        if prev_handler.is_some() {
            panic!("registered a handler for the same message twice");
        }
        self
    }

    fn add_message_handler<F, Fut, M>(&mut self, handler: F) -> &mut Self
    where
        F: 'static + Send + Sync + Fn(M, Session) -> Fut,
        Fut: 'static + Send + Future<Output = Result<()>>,
        M: EnvelopedMessage,
    {
        self.add_handler(move |envelope, session| handler(envelope.payload, session));
        self
    }

    fn add_request_handler<F, Fut, M>(&mut self, handler: F) -> &mut Self
    where
        F: 'static + Send + Sync + Fn(M, Response<M>, Session) -> Fut,
        Fut: Send + Future<Output = Result<()>>,
        M: RequestMessage,
    {
        let handler = Arc::new(handler);
        self.add_handler(move |envelope, session| {
            let receipt = envelope.receipt();
            let handler = handler.clone();
            async move {
                let peer = session.peer.clone();
                let responded = Arc::new(AtomicBool::default());
                let response = Response {
                    peer: peer.clone(),
                    responded: responded.clone(),
                    receipt,
                };
                match (handler)(envelope.payload, response, session).await {
                    Ok(()) => {
                        if responded.load(std::sync::atomic::Ordering::SeqCst) {
                            Ok(())
                        } else {
                            Err(anyhow!("handler did not send a response"))?
                        }
                    }
                    Err(error) => {
                        peer.respond_with_error(
                            receipt,
                            proto::Error {
                                message: error.to_string(),
                            },
                        )?;
                        Err(error)
                    }
                }
            }
        })
    }

    pub fn handle_connection(
        self: &Arc<Self>,
        connection: Connection,
        address: String,
        user: User,
        mut send_connection_id: Option<oneshot::Sender<ConnectionId>>,
        executor: Executor,
    ) -> impl Future<Output = Result<()>> {
        let this = self.clone();
        let user_id = user.id;
        let login = user.github_login;
        let span = info_span!("handle connection", %user_id, %login, %address);
        let mut teardown = self.teardown.subscribe();
        async move {
            let (connection_id, handle_io, mut incoming_rx) = this
                .peer
                .add_connection(connection, {
                    let executor = executor.clone();
                    move |duration| executor.sleep(duration)
                });

            tracing::info!(%user_id, %login, %connection_id, %address, "connection opened");
            this.peer.send(connection_id, proto::Hello { peer_id: Some(connection_id.into()) })?;
            tracing::info!(%user_id, %login, %connection_id, %address, "sent hello message");

            if let Some(send_connection_id) = send_connection_id.take() {
                let _ = send_connection_id.send(connection_id);
            }

            if !user.connected_once {
                this.peer.send(connection_id, proto::ShowContacts {})?;
                this.app_state.db.set_user_connected_once(user_id, true).await?;
            }

            let (contacts, invite_code) = future::try_join(
                this.app_state.db.get_contacts(user_id),
                this.app_state.db.get_invite_code_for_user(user_id)
            ).await?;

            {
                let mut pool = this.connection_pool.lock();
                pool.add_connection(connection_id, user_id, user.admin);
                this.peer.send(connection_id, build_initial_contacts_update(contacts, &pool))?;

                if let Some((code, count)) = invite_code {
                    this.peer.send(connection_id, proto::UpdateInviteInfo {
                        url: format!("{}{}", this.app_state.config.invite_link_prefix, code),
                        count: count as u32,
                    })?;
                }
            }

            if let Some(incoming_call) = this.app_state.db.incoming_call_for_user(user_id).await? {
                this.peer.send(connection_id, incoming_call)?;
            }

            let session = Session {
                user_id,
                connection_id,
                db: Arc::new(tokio::sync::Mutex::new(DbHandle(this.app_state.db.clone()))),
                peer: this.peer.clone(),
                connection_pool: this.connection_pool.clone(),
                live_kit_client: this.app_state.live_kit_client.clone(),
                executor: executor.clone(),
            };
            update_user_contacts(user_id, &session).await?;

            let handle_io = handle_io.fuse();
            futures::pin_mut!(handle_io);

            // Handlers for foreground messages are pushed into the following `FuturesUnordered`.
            // This prevents deadlocks when e.g., client A performs a request to client B and
            // client B performs a request to client A. If both clients stop processing further
            // messages until their respective request completes, they won't have a chance to
            // respond to the other client's request and cause a deadlock.
            //
            // This arrangement ensures we will attempt to process earlier messages first, but fall
            // back to processing messages arrived later in the spirit of making progress.
            let mut foreground_message_handlers = FuturesUnordered::new();
            let concurrent_handlers = Arc::new(Semaphore::new(256));
            loop {
                let next_message = async {
                    let permit = concurrent_handlers.clone().acquire_owned().await.unwrap();
                    let message = incoming_rx.next().await;
                    (permit, message)
                }.fuse();
                futures::pin_mut!(next_message);
                futures::select_biased! {
                    _ = teardown.changed().fuse() => return Ok(()),
                    result = handle_io => {
                        if let Err(error) = result {
                            tracing::error!(?error, %user_id, %login, %connection_id, %address, "error handling I/O");
                        }
                        break;
                    }
                    _ = foreground_message_handlers.next() => {}
                    next_message = next_message => {
                        let (permit, message) = next_message;
                        if let Some(message) = message {
                            let type_name = message.payload_type_name();
                            let span = tracing::info_span!("receive message", %user_id, %login, %connection_id, %address, type_name);
                            let span_enter = span.enter();
                            if let Some(handler) = this.handlers.get(&message.payload_type_id()) {
                                let is_background = message.is_background();
                                let handle_message = (handler)(message, session.clone());
                                drop(span_enter);

                                let handle_message = async move {
                                    handle_message.await;
                                    drop(permit);
                                }.instrument(span);
                                if is_background {
                                    executor.spawn_detached(handle_message);
                                } else {
                                    foreground_message_handlers.push(handle_message);
                                }
                            } else {
                                tracing::error!(%user_id, %login, %connection_id, %address, "no message handler");
                            }
                        } else {
                            tracing::info!(%user_id, %login, %connection_id, %address, "connection closed");
                            break;
                        }
                    }
                }
            }

            drop(foreground_message_handlers);
            tracing::info!(%user_id, %login, %connection_id, %address, "signing out");
            if let Err(error) = connection_lost(session, teardown, executor).await {
                tracing::error!(%user_id, %login, %connection_id, %address, ?error, "error signing out");
            }

            Ok(())
        }.instrument(span)
    }

    pub async fn invite_code_redeemed(
        self: &Arc<Self>,
        inviter_id: UserId,
        invitee_id: UserId,
    ) -> Result<()> {
        if let Some(user) = self.app_state.db.get_user_by_id(inviter_id).await? {
            if let Some(code) = &user.invite_code {
                let pool = self.connection_pool.lock();
                let invitee_contact = contact_for_user(invitee_id, true, false, &pool);
                for connection_id in pool.user_connection_ids(inviter_id) {
                    self.peer.send(
                        connection_id,
                        proto::UpdateContacts {
                            contacts: vec![invitee_contact.clone()],
                            ..Default::default()
                        },
                    )?;
                    self.peer.send(
                        connection_id,
                        proto::UpdateInviteInfo {
                            url: format!("{}{}", self.app_state.config.invite_link_prefix, &code),
                            count: user.invite_count as u32,
                        },
                    )?;
                }
            }
        }
        Ok(())
    }

    pub async fn invite_count_updated(self: &Arc<Self>, user_id: UserId) -> Result<()> {
        if let Some(user) = self.app_state.db.get_user_by_id(user_id).await? {
            if let Some(invite_code) = &user.invite_code {
                let pool = self.connection_pool.lock();
                for connection_id in pool.user_connection_ids(user_id) {
                    self.peer.send(
                        connection_id,
                        proto::UpdateInviteInfo {
                            url: format!(
                                "{}{}",
                                self.app_state.config.invite_link_prefix, invite_code
                            ),
                            count: user.invite_count as u32,
                        },
                    )?;
                }
            }
        }
        Ok(())
    }

    pub async fn snapshot<'a>(self: &'a Arc<Self>) -> ServerSnapshot<'a> {
        ServerSnapshot {
            connection_pool: ConnectionPoolGuard {
                guard: self.connection_pool.lock(),
                _not_send: PhantomData,
            },
            peer: &self.peer,
        }
    }
}

impl<'a> Deref for ConnectionPoolGuard<'a> {
    type Target = ConnectionPool;

    fn deref(&self) -> &Self::Target {
        &*self.guard
    }
}

impl<'a> DerefMut for ConnectionPoolGuard<'a> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.guard
    }
}

impl<'a> Drop for ConnectionPoolGuard<'a> {
    fn drop(&mut self) {
        #[cfg(test)]
        self.check_invariants();
    }
}

fn broadcast<F>(
    sender_id: Option<ConnectionId>,
    receiver_ids: impl IntoIterator<Item = ConnectionId>,
    mut f: F,
) where
    F: FnMut(ConnectionId) -> anyhow::Result<()>,
{
    for receiver_id in receiver_ids {
        if Some(receiver_id) != sender_id {
            if let Err(error) = f(receiver_id) {
                tracing::error!("failed to send to {:?} {}", receiver_id, error);
            }
        }
    }
}

lazy_static! {
    static ref ZED_PROTOCOL_VERSION: HeaderName = HeaderName::from_static("x-zed-protocol-version");
}

pub struct ProtocolVersion(u32);

impl Header for ProtocolVersion {
    fn name() -> &'static HeaderName {
        &ZED_PROTOCOL_VERSION
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let version = values
            .next()
            .ok_or_else(axum::headers::Error::invalid)?
            .to_str()
            .map_err(|_| axum::headers::Error::invalid())?
            .parse()
            .map_err(|_| axum::headers::Error::invalid())?;
        Ok(Self(version))
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, values: &mut E) {
        values.extend([self.0.to_string().parse().unwrap()]);
    }
}

pub fn routes(server: Arc<Server>) -> Router<Body> {
    Router::new()
        .route("/rpc", get(handle_websocket_request))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(server.app_state.clone()))
                .layer(middleware::from_fn(auth::validate_header)),
        )
        .route("/metrics", get(handle_metrics))
        .layer(Extension(server))
}

pub async fn handle_websocket_request(
    TypedHeader(ProtocolVersion(protocol_version)): TypedHeader<ProtocolVersion>,
    ConnectInfo(socket_address): ConnectInfo<SocketAddr>,
    Extension(server): Extension<Arc<Server>>,
    Extension(user): Extension<User>,
    ws: WebSocketUpgrade,
) -> axum::response::Response {
    if protocol_version != rpc::PROTOCOL_VERSION {
        return (
            StatusCode::UPGRADE_REQUIRED,
            "client must be upgraded".to_string(),
        )
            .into_response();
    }
    let socket_address = socket_address.to_string();
    ws.on_upgrade(move |socket| {
        use util::ResultExt;
        let socket = socket
            .map_ok(to_tungstenite_message)
            .err_into()
            .with(|message| async move { Ok(to_axum_message(message)) });
        let connection = Connection::new(Box::pin(socket));
        async move {
            server
                .handle_connection(connection, socket_address, user, None, Executor::Production)
                .await
                .log_err();
        }
    })
}

pub async fn handle_metrics(Extension(server): Extension<Arc<Server>>) -> Result<String> {
    let connections = server
        .connection_pool
        .lock()
        .connections()
        .filter(|connection| !connection.admin)
        .count();

    METRIC_CONNECTIONS.set(connections as _);

    let shared_projects = server.app_state.db.project_count_excluding_admins().await?;
    METRIC_SHARED_PROJECTS.set(shared_projects as _);

    let encoder = prometheus::TextEncoder::new();
    let metric_families = prometheus::gather();
    let encoded_metrics = encoder
        .encode_to_string(&metric_families)
        .map_err(|err| anyhow!("{}", err))?;
    Ok(encoded_metrics)
}

#[instrument(err, skip(executor))]
async fn connection_lost(
    session: Session,
    mut teardown: watch::Receiver<()>,
    executor: Executor,
) -> Result<()> {
    session.peer.disconnect(session.connection_id);
    session
        .connection_pool()
        .await
        .remove_connection(session.connection_id)?;

    session
        .db()
        .await
        .connection_lost(session.connection_id)
        .await
        .trace_err();

    futures::select_biased! {
        _ = executor.sleep(RECONNECT_TIMEOUT).fuse() => {
            leave_room_for_session(&session).await.trace_err();

            if !session
                .connection_pool()
                .await
                .is_user_online(session.user_id)
            {
                let db = session.db().await;
                if let Some(room) = db.decline_call(None, session.user_id).await.trace_err().flatten() {
                    room_updated(&room, &session.peer);
                }
            }
            update_user_contacts(session.user_id, &session).await?;
        }
        _ = teardown.changed().fuse() => {}
    }

    Ok(())
}

async fn ping(_: proto::Ping, response: Response<proto::Ping>, _session: Session) -> Result<()> {
    response.send(proto::Ack {})?;
    Ok(())
}

async fn create_room(
    _request: proto::CreateRoom,
    response: Response<proto::CreateRoom>,
    session: Session,
) -> Result<()> {
    let live_kit_room = nanoid::nanoid!(30);
    let live_kit_connection_info = if let Some(live_kit) = session.live_kit_client.as_ref() {
        if let Some(_) = live_kit
            .create_room(live_kit_room.clone())
            .await
            .trace_err()
        {
            if let Some(token) = live_kit
                .room_token(&live_kit_room, &session.user_id.to_string())
                .trace_err()
            {
                Some(proto::LiveKitConnectionInfo {
                    server_url: live_kit.url().into(),
                    token,
                })
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    {
        let room = session
            .db()
            .await
            .create_room(session.user_id, session.connection_id, &live_kit_room)
            .await?;

        response.send(proto::CreateRoomResponse {
            room: Some(room.clone()),
            live_kit_connection_info,
        })?;
    }

    update_user_contacts(session.user_id, &session).await?;
    Ok(())
}

async fn join_room(
    request: proto::JoinRoom,
    response: Response<proto::JoinRoom>,
    session: Session,
) -> Result<()> {
    let room_id = RoomId::from_proto(request.id);
    let room = {
        let room = session
            .db()
            .await
            .join_room(room_id, session.user_id, session.connection_id)
            .await?;
        room_updated(&room, &session.peer);
        room.clone()
    };

    for connection_id in session
        .connection_pool()
        .await
        .user_connection_ids(session.user_id)
    {
        session
            .peer
            .send(
                connection_id,
                proto::CallCanceled {
                    room_id: room_id.to_proto(),
                },
            )
            .trace_err();
    }

    let live_kit_connection_info = if let Some(live_kit) = session.live_kit_client.as_ref() {
        if let Some(token) = live_kit
            .room_token(&room.live_kit_room, &session.user_id.to_string())
            .trace_err()
        {
            Some(proto::LiveKitConnectionInfo {
                server_url: live_kit.url().into(),
                token,
            })
        } else {
            None
        }
    } else {
        None
    };

    response.send(proto::JoinRoomResponse {
        room: Some(room),
        live_kit_connection_info,
    })?;

    update_user_contacts(session.user_id, &session).await?;
    Ok(())
}

async fn rejoin_room(
    request: proto::RejoinRoom,
    response: Response<proto::RejoinRoom>,
    session: Session,
) -> Result<()> {
    {
        let mut rejoined_room = session
            .db()
            .await
            .rejoin_room(request, session.user_id, session.connection_id)
            .await?;

        response.send(proto::RejoinRoomResponse {
            room: Some(rejoined_room.room.clone()),
            reshared_projects: rejoined_room
                .reshared_projects
                .iter()
                .map(|project| proto::ResharedProject {
                    id: project.id.to_proto(),
                    collaborators: project
                        .collaborators
                        .iter()
                        .map(|collaborator| collaborator.to_proto())
                        .collect(),
                })
                .collect(),
            rejoined_projects: rejoined_room
                .rejoined_projects
                .iter()
                .map(|rejoined_project| proto::RejoinedProject {
                    id: rejoined_project.id.to_proto(),
                    worktrees: rejoined_project
                        .worktrees
                        .iter()
                        .map(|worktree| proto::WorktreeMetadata {
                            id: worktree.id,
                            root_name: worktree.root_name.clone(),
                            visible: worktree.visible,
                            abs_path: worktree.abs_path.clone(),
                        })
                        .collect(),
                    collaborators: rejoined_project
                        .collaborators
                        .iter()
                        .map(|collaborator| collaborator.to_proto())
                        .collect(),
                    language_servers: rejoined_project.language_servers.clone(),
                })
                .collect(),
        })?;
        room_updated(&rejoined_room.room, &session.peer);

        for project in &rejoined_room.reshared_projects {
            for collaborator in &project.collaborators {
                session
                    .peer
                    .send(
                        collaborator.connection_id,
                        proto::UpdateProjectCollaborator {
                            project_id: project.id.to_proto(),
                            old_peer_id: Some(project.old_connection_id.into()),
                            new_peer_id: Some(session.connection_id.into()),
                        },
                    )
                    .trace_err();
            }

            broadcast(
                Some(session.connection_id),
                project
                    .collaborators
                    .iter()
                    .map(|collaborator| collaborator.connection_id),
                |connection_id| {
                    session.peer.forward_send(
                        session.connection_id,
                        connection_id,
                        proto::UpdateProject {
                            project_id: project.id.to_proto(),
                            worktrees: project.worktrees.clone(),
                        },
                    )
                },
            );
        }

        for project in &rejoined_room.rejoined_projects {
            for collaborator in &project.collaborators {
                session
                    .peer
                    .send(
                        collaborator.connection_id,
                        proto::UpdateProjectCollaborator {
                            project_id: project.id.to_proto(),
                            old_peer_id: Some(project.old_connection_id.into()),
                            new_peer_id: Some(session.connection_id.into()),
                        },
                    )
                    .trace_err();
            }
        }

        for project in &mut rejoined_room.rejoined_projects {
            for worktree in mem::take(&mut project.worktrees) {
                #[cfg(any(test, feature = "test-support"))]
                const MAX_CHUNK_SIZE: usize = 2;
                #[cfg(not(any(test, feature = "test-support")))]
                const MAX_CHUNK_SIZE: usize = 256;

                // Stream this worktree's entries.
                let message = proto::UpdateWorktree {
                    project_id: project.id.to_proto(),
                    worktree_id: worktree.id,
                    abs_path: worktree.abs_path.clone(),
                    root_name: worktree.root_name,
                    updated_entries: worktree.updated_entries,
                    removed_entries: worktree.removed_entries,
                    scan_id: worktree.scan_id,
                    is_last_update: worktree.completed_scan_id == worktree.scan_id,
                };
                for update in proto::split_worktree_update(message, MAX_CHUNK_SIZE) {
                    session.peer.send(session.connection_id, update.clone())?;
                }

                // Stream this worktree's diagnostics.
                for summary in worktree.diagnostic_summaries {
                    session.peer.send(
                        session.connection_id,
                        proto::UpdateDiagnosticSummary {
                            project_id: project.id.to_proto(),
                            worktree_id: worktree.id,
                            summary: Some(summary),
                        },
                    )?;
                }
            }

            for language_server in &project.language_servers {
                session.peer.send(
                    session.connection_id,
                    proto::UpdateLanguageServer {
                        project_id: project.id.to_proto(),
                        language_server_id: language_server.id,
                        variant: Some(
                            proto::update_language_server::Variant::DiskBasedDiagnosticsUpdated(
                                proto::LspDiskBasedDiagnosticsUpdated {},
                            ),
                        ),
                    },
                )?;
            }
        }
    }

    update_user_contacts(session.user_id, &session).await?;
    Ok(())
}

async fn leave_room(
    _: proto::LeaveRoom,
    response: Response<proto::LeaveRoom>,
    session: Session,
) -> Result<()> {
    leave_room_for_session(&session).await?;
    response.send(proto::Ack {})?;
    Ok(())
}

async fn call(
    request: proto::Call,
    response: Response<proto::Call>,
    session: Session,
) -> Result<()> {
    let room_id = RoomId::from_proto(request.room_id);
    let calling_user_id = session.user_id;
    let calling_connection_id = session.connection_id;
    let called_user_id = UserId::from_proto(request.called_user_id);
    let initial_project_id = request.initial_project_id.map(ProjectId::from_proto);
    if !session
        .db()
        .await
        .has_contact(calling_user_id, called_user_id)
        .await?
    {
        return Err(anyhow!("cannot call a user who isn't a contact"))?;
    }

    let incoming_call = {
        let (room, incoming_call) = &mut *session
            .db()
            .await
            .call(
                room_id,
                calling_user_id,
                calling_connection_id,
                called_user_id,
                initial_project_id,
            )
            .await?;
        room_updated(&room, &session.peer);
        mem::take(incoming_call)
    };
    update_user_contacts(called_user_id, &session).await?;

    let mut calls = session
        .connection_pool()
        .await
        .user_connection_ids(called_user_id)
        .map(|connection_id| session.peer.request(connection_id, incoming_call.clone()))
        .collect::<FuturesUnordered<_>>();

    while let Some(call_response) = calls.next().await {
        match call_response.as_ref() {
            Ok(_) => {
                response.send(proto::Ack {})?;
                return Ok(());
            }
            Err(_) => {
                call_response.trace_err();
            }
        }
    }

    {
        let room = session
            .db()
            .await
            .call_failed(room_id, called_user_id)
            .await?;
        room_updated(&room, &session.peer);
    }
    update_user_contacts(called_user_id, &session).await?;

    Err(anyhow!("failed to ring user"))?
}

async fn cancel_call(
    request: proto::CancelCall,
    response: Response<proto::CancelCall>,
    session: Session,
) -> Result<()> {
    let called_user_id = UserId::from_proto(request.called_user_id);
    let room_id = RoomId::from_proto(request.room_id);
    {
        let room = session
            .db()
            .await
            .cancel_call(room_id, session.connection_id, called_user_id)
            .await?;
        room_updated(&room, &session.peer);
    }

    for connection_id in session
        .connection_pool()
        .await
        .user_connection_ids(called_user_id)
    {
        session
            .peer
            .send(
                connection_id,
                proto::CallCanceled {
                    room_id: room_id.to_proto(),
                },
            )
            .trace_err();
    }
    response.send(proto::Ack {})?;

    update_user_contacts(called_user_id, &session).await?;
    Ok(())
}

async fn decline_call(message: proto::DeclineCall, session: Session) -> Result<()> {
    let room_id = RoomId::from_proto(message.room_id);
    {
        let room = session
            .db()
            .await
            .decline_call(Some(room_id), session.user_id)
            .await?
            .ok_or_else(|| anyhow!("failed to decline call"))?;
        room_updated(&room, &session.peer);
    }

    for connection_id in session
        .connection_pool()
        .await
        .user_connection_ids(session.user_id)
    {
        session
            .peer
            .send(
                connection_id,
                proto::CallCanceled {
                    room_id: room_id.to_proto(),
                },
            )
            .trace_err();
    }
    update_user_contacts(session.user_id, &session).await?;
    Ok(())
}

async fn update_participant_location(
    request: proto::UpdateParticipantLocation,
    response: Response<proto::UpdateParticipantLocation>,
    session: Session,
) -> Result<()> {
    let room_id = RoomId::from_proto(request.room_id);
    let location = request
        .location
        .ok_or_else(|| anyhow!("invalid location"))?;
    let room = session
        .db()
        .await
        .update_room_participant_location(room_id, session.connection_id, location)
        .await?;
    room_updated(&room, &session.peer);
    response.send(proto::Ack {})?;
    Ok(())
}

async fn share_project(
    request: proto::ShareProject,
    response: Response<proto::ShareProject>,
    session: Session,
) -> Result<()> {
    let (project_id, room) = &*session
        .db()
        .await
        .share_project(
            RoomId::from_proto(request.room_id),
            session.connection_id,
            &request.worktrees,
        )
        .await?;
    response.send(proto::ShareProjectResponse {
        project_id: project_id.to_proto(),
    })?;
    room_updated(&room, &session.peer);

    Ok(())
}

async fn unshare_project(message: proto::UnshareProject, session: Session) -> Result<()> {
    let project_id = ProjectId::from_proto(message.project_id);

    let (room, guest_connection_ids) = &*session
        .db()
        .await
        .unshare_project(project_id, session.connection_id)
        .await?;

    broadcast(
        Some(session.connection_id),
        guest_connection_ids.iter().copied(),
        |conn_id| session.peer.send(conn_id, message.clone()),
    );
    room_updated(&room, &session.peer);

    Ok(())
}

async fn join_project(
    request: proto::JoinProject,
    response: Response<proto::JoinProject>,
    session: Session,
) -> Result<()> {
    let project_id = ProjectId::from_proto(request.project_id);
    let guest_user_id = session.user_id;

    tracing::info!(%project_id, "join project");

    let (project, replica_id) = &mut *session
        .db()
        .await
        .join_project(project_id, session.connection_id)
        .await?;

    let collaborators = project
        .collaborators
        .iter()
        .filter(|collaborator| collaborator.connection_id != session.connection_id)
        .map(|collaborator| collaborator.to_proto())
        .collect::<Vec<_>>();

    let worktrees = project
        .worktrees
        .iter()
        .map(|(id, worktree)| proto::WorktreeMetadata {
            id: *id,
            root_name: worktree.root_name.clone(),
            visible: worktree.visible,
            abs_path: worktree.abs_path.clone(),
        })
        .collect::<Vec<_>>();

    for collaborator in &collaborators {
        session
            .peer
            .send(
                collaborator.peer_id.unwrap().into(),
                proto::AddProjectCollaborator {
                    project_id: project_id.to_proto(),
                    collaborator: Some(proto::Collaborator {
                        peer_id: Some(session.connection_id.into()),
                        replica_id: replica_id.0 as u32,
                        user_id: guest_user_id.to_proto(),
                    }),
                },
            )
            .trace_err();
    }

    // First, we send the metadata associated with each worktree.
    response.send(proto::JoinProjectResponse {
        worktrees: worktrees.clone(),
        replica_id: replica_id.0 as u32,
        collaborators: collaborators.clone(),
        language_servers: project.language_servers.clone(),
    })?;

    for (worktree_id, worktree) in mem::take(&mut project.worktrees) {
        #[cfg(any(test, feature = "test-support"))]
        const MAX_CHUNK_SIZE: usize = 2;
        #[cfg(not(any(test, feature = "test-support")))]
        const MAX_CHUNK_SIZE: usize = 256;

        // Stream this worktree's entries.
        let message = proto::UpdateWorktree {
            project_id: project_id.to_proto(),
            worktree_id,
            abs_path: worktree.abs_path.clone(),
            root_name: worktree.root_name,
            updated_entries: worktree.entries,
            removed_entries: Default::default(),
            scan_id: worktree.scan_id,
            is_last_update: worktree.scan_id == worktree.completed_scan_id,
        };
        for update in proto::split_worktree_update(message, MAX_CHUNK_SIZE) {
            session.peer.send(session.connection_id, update.clone())?;
        }

        // Stream this worktree's diagnostics.
        for summary in worktree.diagnostic_summaries {
            session.peer.send(
                session.connection_id,
                proto::UpdateDiagnosticSummary {
                    project_id: project_id.to_proto(),
                    worktree_id: worktree.id,
                    summary: Some(summary),
                },
            )?;
        }
    }

    for language_server in &project.language_servers {
        session.peer.send(
            session.connection_id,
            proto::UpdateLanguageServer {
                project_id: project_id.to_proto(),
                language_server_id: language_server.id,
                variant: Some(
                    proto::update_language_server::Variant::DiskBasedDiagnosticsUpdated(
                        proto::LspDiskBasedDiagnosticsUpdated {},
                    ),
                ),
            },
        )?;
    }

    Ok(())
}

async fn leave_project(request: proto::LeaveProject, session: Session) -> Result<()> {
    let sender_id = session.connection_id;
    let project_id = ProjectId::from_proto(request.project_id);

    let (room, project) = &*session
        .db()
        .await
        .leave_project(project_id, sender_id)
        .await?;
    tracing::info!(
        %project_id,
        host_user_id = %project.host_user_id,
        host_connection_id = %project.host_connection_id,
        "leave project"
    );

    project_left(&project, &session);
    room_updated(&room, &session.peer);

    Ok(())
}

async fn update_project(
    request: proto::UpdateProject,
    response: Response<proto::UpdateProject>,
    session: Session,
) -> Result<()> {
    let project_id = ProjectId::from_proto(request.project_id);
    let (room, guest_connection_ids) = &*session
        .db()
        .await
        .update_project(project_id, session.connection_id, &request.worktrees)
        .await?;
    broadcast(
        Some(session.connection_id),
        guest_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())
        },
    );
    room_updated(&room, &session.peer);
    response.send(proto::Ack {})?;

    Ok(())
}

async fn update_worktree(
    request: proto::UpdateWorktree,
    response: Response<proto::UpdateWorktree>,
    session: Session,
) -> Result<()> {
    let guest_connection_ids = session
        .db()
        .await
        .update_worktree(&request, session.connection_id)
        .await?;

    broadcast(
        Some(session.connection_id),
        guest_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())
        },
    );
    response.send(proto::Ack {})?;
    Ok(())
}

async fn update_diagnostic_summary(
    message: proto::UpdateDiagnosticSummary,
    session: Session,
) -> Result<()> {
    let guest_connection_ids = session
        .db()
        .await
        .update_diagnostic_summary(&message, session.connection_id)
        .await?;

    broadcast(
        Some(session.connection_id),
        guest_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, message.clone())
        },
    );

    Ok(())
}

async fn start_language_server(
    request: proto::StartLanguageServer,
    session: Session,
) -> Result<()> {
    let guest_connection_ids = session
        .db()
        .await
        .start_language_server(&request, session.connection_id)
        .await?;

    broadcast(
        Some(session.connection_id),
        guest_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())
        },
    );
    Ok(())
}

async fn update_language_server(
    request: proto::UpdateLanguageServer,
    session: Session,
) -> Result<()> {
    session.executor.record_backtrace();
    let project_id = ProjectId::from_proto(request.project_id);
    let project_connection_ids = session
        .db()
        .await
        .project_connection_ids(project_id, session.connection_id)
        .await?;
    broadcast(
        Some(session.connection_id),
        project_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())
        },
    );
    Ok(())
}

async fn forward_project_request<T>(
    request: T,
    response: Response<T>,
    session: Session,
) -> Result<()>
where
    T: EntityMessage + RequestMessage,
{
    session.executor.record_backtrace();
    let project_id = ProjectId::from_proto(request.remote_entity_id());
    let host_connection_id = {
        let collaborators = session
            .db()
            .await
            .project_collaborators(project_id, session.connection_id)
            .await?;
        collaborators
            .iter()
            .find(|collaborator| collaborator.is_host)
            .ok_or_else(|| anyhow!("host not found"))?
            .connection_id
    };

    let payload = session
        .peer
        .forward_request(session.connection_id, host_connection_id, request)
        .await?;

    response.send(payload)?;
    Ok(())
}

async fn save_buffer(
    request: proto::SaveBuffer,
    response: Response<proto::SaveBuffer>,
    session: Session,
) -> Result<()> {
    let project_id = ProjectId::from_proto(request.project_id);
    let host_connection_id = {
        let collaborators = session
            .db()
            .await
            .project_collaborators(project_id, session.connection_id)
            .await?;
        collaborators
            .iter()
            .find(|collaborator| collaborator.is_host)
            .ok_or_else(|| anyhow!("host not found"))?
            .connection_id
    };
    let response_payload = session
        .peer
        .forward_request(session.connection_id, host_connection_id, request.clone())
        .await?;

    let mut collaborators = session
        .db()
        .await
        .project_collaborators(project_id, session.connection_id)
        .await?;
    collaborators.retain(|collaborator| collaborator.connection_id != session.connection_id);
    let project_connection_ids = collaborators
        .iter()
        .map(|collaborator| collaborator.connection_id);
    broadcast(
        Some(host_connection_id),
        project_connection_ids,
        |conn_id| {
            session
                .peer
                .forward_send(host_connection_id, conn_id, response_payload.clone())
        },
    );
    response.send(response_payload)?;
    Ok(())
}

async fn create_buffer_for_peer(
    request: proto::CreateBufferForPeer,
    session: Session,
) -> Result<()> {
    session.executor.record_backtrace();
    let peer_id = request.peer_id.ok_or_else(|| anyhow!("invalid peer id"))?;
    session
        .peer
        .forward_send(session.connection_id, peer_id.into(), request)?;
    Ok(())
}

async fn update_buffer(
    request: proto::UpdateBuffer,
    response: Response<proto::UpdateBuffer>,
    session: Session,
) -> Result<()> {
    session.executor.record_backtrace();
    let project_id = ProjectId::from_proto(request.project_id);
    let project_connection_ids = session
        .db()
        .await
        .project_connection_ids(project_id, session.connection_id)
        .await?;

    session.executor.record_backtrace();

    broadcast(
        Some(session.connection_id),
        project_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())
        },
    );
    response.send(proto::Ack {})?;
    Ok(())
}

async fn update_buffer_file(request: proto::UpdateBufferFile, session: Session) -> Result<()> {
    let project_id = ProjectId::from_proto(request.project_id);
    let project_connection_ids = session
        .db()
        .await
        .project_connection_ids(project_id, session.connection_id)
        .await?;

    broadcast(
        Some(session.connection_id),
        project_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())
        },
    );
    Ok(())
}

async fn buffer_reloaded(request: proto::BufferReloaded, session: Session) -> Result<()> {
    let project_id = ProjectId::from_proto(request.project_id);
    let project_connection_ids = session
        .db()
        .await
        .project_connection_ids(project_id, session.connection_id)
        .await?;
    broadcast(
        Some(session.connection_id),
        project_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())
        },
    );
    Ok(())
}

async fn buffer_saved(request: proto::BufferSaved, session: Session) -> Result<()> {
    let project_id = ProjectId::from_proto(request.project_id);
    let project_connection_ids = session
        .db()
        .await
        .project_connection_ids(project_id, session.connection_id)
        .await?;
    broadcast(
        Some(session.connection_id),
        project_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())
        },
    );
    Ok(())
}

async fn follow(
    request: proto::Follow,
    response: Response<proto::Follow>,
    session: Session,
) -> Result<()> {
    let project_id = ProjectId::from_proto(request.project_id);
    let leader_id = request
        .leader_id
        .ok_or_else(|| anyhow!("invalid leader id"))?
        .into();
    let follower_id = session.connection_id;

    {
        let project_connection_ids = session
            .db()
            .await
            .project_connection_ids(project_id, session.connection_id)
            .await?;

        if !project_connection_ids.contains(&leader_id) {
            Err(anyhow!("no such peer"))?;
        }
    }

    let mut response_payload = session
        .peer
        .forward_request(session.connection_id, leader_id, request)
        .await?;
    response_payload
        .views
        .retain(|view| view.leader_id != Some(follower_id.into()));
    response.send(response_payload)?;

    let room = session
        .db()
        .await
        .follow(project_id, leader_id, follower_id)
        .await?;
    room_updated(&room, &session.peer);

    Ok(())
}

async fn unfollow(request: proto::Unfollow, session: Session) -> Result<()> {
    let project_id = ProjectId::from_proto(request.project_id);
    let leader_id = request
        .leader_id
        .ok_or_else(|| anyhow!("invalid leader id"))?
        .into();
    let follower_id = session.connection_id;

    if !session
        .db()
        .await
        .project_connection_ids(project_id, session.connection_id)
        .await?
        .contains(&leader_id)
    {
        Err(anyhow!("no such peer"))?;
    }

    session
        .peer
        .forward_send(session.connection_id, leader_id, request)?;

    let room = session
        .db()
        .await
        .unfollow(project_id, leader_id, follower_id)
        .await?;
    room_updated(&room, &session.peer);

    Ok(())
}

async fn update_followers(request: proto::UpdateFollowers, session: Session) -> Result<()> {
    let project_id = ProjectId::from_proto(request.project_id);
    let project_connection_ids = session
        .db
        .lock()
        .await
        .project_connection_ids(project_id, session.connection_id)
        .await?;

    let leader_id = request.variant.as_ref().and_then(|variant| match variant {
        proto::update_followers::Variant::CreateView(payload) => payload.leader_id,
        proto::update_followers::Variant::UpdateView(payload) => payload.leader_id,
        proto::update_followers::Variant::UpdateActiveView(payload) => payload.leader_id,
    });
    for follower_peer_id in request.follower_ids.iter().copied() {
        let follower_connection_id = follower_peer_id.into();
        if project_connection_ids.contains(&follower_connection_id)
            && Some(follower_peer_id) != leader_id
        {
            session.peer.forward_send(
                session.connection_id,
                follower_connection_id,
                request.clone(),
            )?;
        }
    }
    Ok(())
}

async fn get_users(
    request: proto::GetUsers,
    response: Response<proto::GetUsers>,
    session: Session,
) -> Result<()> {
    let user_ids = request
        .user_ids
        .into_iter()
        .map(UserId::from_proto)
        .collect();
    let users = session
        .db()
        .await
        .get_users_by_ids(user_ids)
        .await?
        .into_iter()
        .map(|user| proto::User {
            id: user.id.to_proto(),
            avatar_url: format!("https://github.com/{}.png?size=128", user.github_login),
            github_login: user.github_login,
        })
        .collect();
    response.send(proto::UsersResponse { users })?;
    Ok(())
}

async fn fuzzy_search_users(
    request: proto::FuzzySearchUsers,
    response: Response<proto::FuzzySearchUsers>,
    session: Session,
) -> Result<()> {
    let query = request.query;
    let users = match query.len() {
        0 => vec![],
        1 | 2 => session
            .db()
            .await
            .get_user_by_github_login(&query)
            .await?
            .into_iter()
            .collect(),
        _ => session.db().await.fuzzy_search_users(&query, 10).await?,
    };
    let users = users
        .into_iter()
        .filter(|user| user.id != session.user_id)
        .map(|user| proto::User {
            id: user.id.to_proto(),
            avatar_url: format!("https://github.com/{}.png?size=128", user.github_login),
            github_login: user.github_login,
        })
        .collect();
    response.send(proto::UsersResponse { users })?;
    Ok(())
}

async fn request_contact(
    request: proto::RequestContact,
    response: Response<proto::RequestContact>,
    session: Session,
) -> Result<()> {
    let requester_id = session.user_id;
    let responder_id = UserId::from_proto(request.responder_id);
    if requester_id == responder_id {
        return Err(anyhow!("cannot add yourself as a contact"))?;
    }

    session
        .db()
        .await
        .send_contact_request(requester_id, responder_id)
        .await?;

    // Update outgoing contact requests of requester
    let mut update = proto::UpdateContacts::default();
    update.outgoing_requests.push(responder_id.to_proto());
    for connection_id in session
        .connection_pool()
        .await
        .user_connection_ids(requester_id)
    {
        session.peer.send(connection_id, update.clone())?;
    }

    // Update incoming contact requests of responder
    let mut update = proto::UpdateContacts::default();
    update
        .incoming_requests
        .push(proto::IncomingContactRequest {
            requester_id: requester_id.to_proto(),
            should_notify: true,
        });
    for connection_id in session
        .connection_pool()
        .await
        .user_connection_ids(responder_id)
    {
        session.peer.send(connection_id, update.clone())?;
    }

    response.send(proto::Ack {})?;
    Ok(())
}

async fn respond_to_contact_request(
    request: proto::RespondToContactRequest,
    response: Response<proto::RespondToContactRequest>,
    session: Session,
) -> Result<()> {
    let responder_id = session.user_id;
    let requester_id = UserId::from_proto(request.requester_id);
    let db = session.db().await;
    if request.response == proto::ContactRequestResponse::Dismiss as i32 {
        db.dismiss_contact_notification(responder_id, requester_id)
            .await?;
    } else {
        let accept = request.response == proto::ContactRequestResponse::Accept as i32;

        db.respond_to_contact_request(responder_id, requester_id, accept)
            .await?;
        let requester_busy = db.is_user_busy(requester_id).await?;
        let responder_busy = db.is_user_busy(responder_id).await?;

        let pool = session.connection_pool().await;
        // Update responder with new contact
        let mut update = proto::UpdateContacts::default();
        if accept {
            update
                .contacts
                .push(contact_for_user(requester_id, false, requester_busy, &pool));
        }
        update
            .remove_incoming_requests
            .push(requester_id.to_proto());
        for connection_id in pool.user_connection_ids(responder_id) {
            session.peer.send(connection_id, update.clone())?;
        }

        // Update requester with new contact
        let mut update = proto::UpdateContacts::default();
        if accept {
            update
                .contacts
                .push(contact_for_user(responder_id, true, responder_busy, &pool));
        }
        update
            .remove_outgoing_requests
            .push(responder_id.to_proto());
        for connection_id in pool.user_connection_ids(requester_id) {
            session.peer.send(connection_id, update.clone())?;
        }
    }

    response.send(proto::Ack {})?;
    Ok(())
}

async fn remove_contact(
    request: proto::RemoveContact,
    response: Response<proto::RemoveContact>,
    session: Session,
) -> Result<()> {
    let requester_id = session.user_id;
    let responder_id = UserId::from_proto(request.user_id);
    let db = session.db().await;
    let contact_accepted = db.remove_contact(requester_id, responder_id).await?;

    let pool = session.connection_pool().await;
    // Update outgoing contact requests of requester
    let mut update = proto::UpdateContacts::default();
    if contact_accepted {
        update.remove_contacts.push(responder_id.to_proto());
    } else {
        update
            .remove_outgoing_requests
            .push(responder_id.to_proto());
    }
    for connection_id in pool.user_connection_ids(requester_id) {
        session.peer.send(connection_id, update.clone())?;
    }

    // Update incoming contact requests of responder
    let mut update = proto::UpdateContacts::default();
    if contact_accepted {
        update.remove_contacts.push(requester_id.to_proto());
    } else {
        update
            .remove_incoming_requests
            .push(requester_id.to_proto());
    }
    for connection_id in pool.user_connection_ids(responder_id) {
        session.peer.send(connection_id, update.clone())?;
    }

    response.send(proto::Ack {})?;
    Ok(())
}

async fn update_diff_base(request: proto::UpdateDiffBase, session: Session) -> Result<()> {
    let project_id = ProjectId::from_proto(request.project_id);
    let project_connection_ids = session
        .db()
        .await
        .project_connection_ids(project_id, session.connection_id)
        .await?;
    broadcast(
        Some(session.connection_id),
        project_connection_ids.iter().copied(),
        |connection_id| {
            session
                .peer
                .forward_send(session.connection_id, connection_id, request.clone())
        },
    );
    Ok(())
}

async fn get_private_user_info(
    _request: proto::GetPrivateUserInfo,
    response: Response<proto::GetPrivateUserInfo>,
    session: Session,
) -> Result<()> {
    let metrics_id = session
        .db()
        .await
        .get_user_metrics_id(session.user_id)
        .await?;
    let user = session
        .db()
        .await
        .get_user_by_id(session.user_id)
        .await?
        .ok_or_else(|| anyhow!("user not found"))?;
    response.send(proto::GetPrivateUserInfoResponse {
        metrics_id,
        staff: user.admin,
    })?;
    Ok(())
}

fn to_axum_message(message: TungsteniteMessage) -> AxumMessage {
    match message {
        TungsteniteMessage::Text(payload) => AxumMessage::Text(payload),
        TungsteniteMessage::Binary(payload) => AxumMessage::Binary(payload),
        TungsteniteMessage::Ping(payload) => AxumMessage::Ping(payload),
        TungsteniteMessage::Pong(payload) => AxumMessage::Pong(payload),
        TungsteniteMessage::Close(frame) => AxumMessage::Close(frame.map(|frame| AxumCloseFrame {
            code: frame.code.into(),
            reason: frame.reason,
        })),
    }
}

fn to_tungstenite_message(message: AxumMessage) -> TungsteniteMessage {
    match message {
        AxumMessage::Text(payload) => TungsteniteMessage::Text(payload),
        AxumMessage::Binary(payload) => TungsteniteMessage::Binary(payload),
        AxumMessage::Ping(payload) => TungsteniteMessage::Ping(payload),
        AxumMessage::Pong(payload) => TungsteniteMessage::Pong(payload),
        AxumMessage::Close(frame) => {
            TungsteniteMessage::Close(frame.map(|frame| TungsteniteCloseFrame {
                code: frame.code.into(),
                reason: frame.reason,
            }))
        }
    }
}

fn build_initial_contacts_update(
    contacts: Vec<db::Contact>,
    pool: &ConnectionPool,
) -> proto::UpdateContacts {
    let mut update = proto::UpdateContacts::default();

    for contact in contacts {
        match contact {
            db::Contact::Accepted {
                user_id,
                should_notify,
                busy,
            } => {
                update
                    .contacts
                    .push(contact_for_user(user_id, should_notify, busy, &pool));
            }
            db::Contact::Outgoing { user_id } => update.outgoing_requests.push(user_id.to_proto()),
            db::Contact::Incoming {
                user_id,
                should_notify,
            } => update
                .incoming_requests
                .push(proto::IncomingContactRequest {
                    requester_id: user_id.to_proto(),
                    should_notify,
                }),
        }
    }

    update
}

fn contact_for_user(
    user_id: UserId,
    should_notify: bool,
    busy: bool,
    pool: &ConnectionPool,
) -> proto::Contact {
    proto::Contact {
        user_id: user_id.to_proto(),
        online: pool.is_user_online(user_id),
        busy,
        should_notify,
    }
}

fn room_updated(room: &proto::Room, peer: &Peer) {
    broadcast(
        None,
        room.participants
            .iter()
            .filter_map(|participant| Some(participant.peer_id?.into())),
        |peer_id| {
            peer.send(
                peer_id.into(),
                proto::RoomUpdated {
                    room: Some(room.clone()),
                },
            )
        },
    );
}

async fn update_user_contacts(user_id: UserId, session: &Session) -> Result<()> {
    let db = session.db().await;
    let contacts = db.get_contacts(user_id).await?;
    let busy = db.is_user_busy(user_id).await?;

    let pool = session.connection_pool().await;
    let updated_contact = contact_for_user(user_id, false, busy, &pool);
    for contact in contacts {
        if let db::Contact::Accepted {
            user_id: contact_user_id,
            ..
        } = contact
        {
            for contact_conn_id in pool.user_connection_ids(contact_user_id) {
                session
                    .peer
                    .send(
                        contact_conn_id,
                        proto::UpdateContacts {
                            contacts: vec![updated_contact.clone()],
                            remove_contacts: Default::default(),
                            incoming_requests: Default::default(),
                            remove_incoming_requests: Default::default(),
                            outgoing_requests: Default::default(),
                            remove_outgoing_requests: Default::default(),
                        },
                    )
                    .trace_err();
            }
        }
    }
    Ok(())
}

async fn leave_room_for_session(session: &Session) -> Result<()> {
    let mut contacts_to_update = HashSet::default();

    let room_id;
    let canceled_calls_to_user_ids;
    let live_kit_room;
    let delete_live_kit_room;
    if let Some(mut left_room) = session.db().await.leave_room(session.connection_id).await? {
        contacts_to_update.insert(session.user_id);

        for project in left_room.left_projects.values() {
            project_left(project, session);
        }

        room_updated(&left_room.room, &session.peer);
        room_id = RoomId::from_proto(left_room.room.id);
        canceled_calls_to_user_ids = mem::take(&mut left_room.canceled_calls_to_user_ids);
        live_kit_room = mem::take(&mut left_room.room.live_kit_room);
        delete_live_kit_room = left_room.room.participants.is_empty();
    } else {
        return Ok(());
    }

    {
        let pool = session.connection_pool().await;
        for canceled_user_id in canceled_calls_to_user_ids {
            for connection_id in pool.user_connection_ids(canceled_user_id) {
                session
                    .peer
                    .send(
                        connection_id,
                        proto::CallCanceled {
                            room_id: room_id.to_proto(),
                        },
                    )
                    .trace_err();
            }
            contacts_to_update.insert(canceled_user_id);
        }
    }

    for contact_user_id in contacts_to_update {
        update_user_contacts(contact_user_id, &session).await?;
    }

    if let Some(live_kit) = session.live_kit_client.as_ref() {
        live_kit
            .remove_participant(live_kit_room.clone(), session.user_id.to_string())
            .await
            .trace_err();

        if delete_live_kit_room {
            live_kit.delete_room(live_kit_room).await.trace_err();
        }
    }

    Ok(())
}

fn project_left(project: &db::LeftProject, session: &Session) {
    for connection_id in &project.connection_ids {
        if project.host_user_id == session.user_id {
            session
                .peer
                .send(
                    *connection_id,
                    proto::UnshareProject {
                        project_id: project.id.to_proto(),
                    },
                )
                .trace_err();
        } else {
            session
                .peer
                .send(
                    *connection_id,
                    proto::RemoveProjectCollaborator {
                        project_id: project.id.to_proto(),
                        peer_id: Some(session.connection_id.into()),
                    },
                )
                .trace_err();
        }
    }
}

pub trait ResultExt {
    type Ok;

    fn trace_err(self) -> Option<Self::Ok>;
}

impl<T, E> ResultExt for Result<T, E>
where
    E: std::fmt::Debug,
{
    type Ok = T;

    fn trace_err(self) -> Option<T> {
        match self {
            Ok(value) => Some(value),
            Err(error) => {
                tracing::error!("{:?}", error);
                None
            }
        }
    }
}
