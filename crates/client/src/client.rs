#[cfg(any(test, feature = "test-support"))]
pub mod test;

pub mod channel;
pub mod http;
pub mod user;

use anyhow::{anyhow, Context, Result};
use async_recursion::async_recursion;
use async_tungstenite::tungstenite::{
    error::Error as WebsocketError,
    http::{Request, StatusCode},
};
use futures::{future::LocalBoxFuture, FutureExt, StreamExt};
use gpui::{
    action, AnyModelHandle, AnyViewHandle, AnyWeakModelHandle, AnyWeakViewHandle, AsyncAppContext,
    Entity, ModelContext, ModelHandle, MutableAppContext, Task, View, ViewContext, ViewHandle,
};
use http::HttpClient;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use postage::watch;
use rand::prelude::*;
use rpc::proto::{AnyTypedEnvelope, EntityMessage, EnvelopedMessage, RequestMessage};
use std::{
    any::TypeId,
    collections::HashMap,
    convert::TryFrom,
    fmt::Write as _,
    future::Future,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc, Weak,
    },
    time::{Duration, Instant},
};
use surf::{http::Method, Url};
use thiserror::Error;
use util::{ResultExt, TryFutureExt};

pub use channel::*;
pub use rpc::*;
pub use user::*;

lazy_static! {
    static ref ZED_SERVER_URL: String =
        std::env::var("ZED_SERVER_URL").unwrap_or("https://zed.dev".to_string());
    pub static ref IMPERSONATE_LOGIN: Option<String> = std::env::var("ZED_IMPERSONATE")
        .ok()
        .and_then(|s| if s.is_empty() { None } else { Some(s) });
}

action!(Authenticate);

pub fn init(rpc: Arc<Client>, cx: &mut MutableAppContext) {
    cx.add_global_action(move |_: &Authenticate, cx| {
        let rpc = rpc.clone();
        cx.spawn(|cx| async move { rpc.authenticate_and_connect(true, &cx).log_err().await })
            .detach();
    });
}

pub struct Client {
    id: usize,
    peer: Arc<Peer>,
    http: Arc<dyn HttpClient>,
    state: RwLock<ClientState>,
    authenticate:
        Option<Box<dyn 'static + Send + Sync + Fn(&AsyncAppContext) -> Task<Result<Credentials>>>>,
    establish_connection: Option<
        Box<
            dyn 'static
                + Send
                + Sync
                + Fn(
                    &Credentials,
                    &AsyncAppContext,
                ) -> Task<Result<Connection, EstablishConnectionError>>,
        >,
    >,
}

#[derive(Error, Debug)]
pub enum EstablishConnectionError {
    #[error("upgrade required")]
    UpgradeRequired,
    #[error("unauthorized")]
    Unauthorized,
    #[error("{0}")]
    Other(#[from] anyhow::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Http(#[from] async_tungstenite::tungstenite::http::Error),
}

impl From<WebsocketError> for EstablishConnectionError {
    fn from(error: WebsocketError) -> Self {
        if let WebsocketError::Http(response) = &error {
            match response.status() {
                StatusCode::UNAUTHORIZED => return EstablishConnectionError::Unauthorized,
                StatusCode::UPGRADE_REQUIRED => return EstablishConnectionError::UpgradeRequired,
                _ => {}
            }
        }
        EstablishConnectionError::Other(error.into())
    }
}

impl EstablishConnectionError {
    pub fn other(error: impl Into<anyhow::Error> + Send + Sync) -> Self {
        Self::Other(error.into())
    }
}

#[derive(Copy, Clone, Debug)]
pub enum Status {
    SignedOut,
    UpgradeRequired,
    Authenticating,
    Connecting,
    ConnectionError,
    Connected { connection_id: ConnectionId },
    ConnectionLost,
    Reauthenticating,
    Reconnecting,
    ReconnectionError { next_reconnection: Instant },
}

impl Status {
    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Connected { .. })
    }
}

struct ClientState {
    credentials: Option<Credentials>,
    status: (watch::Sender<Status>, watch::Receiver<Status>),
    entity_id_extractors: HashMap<TypeId, fn(&dyn AnyTypedEnvelope) -> u64>,
    _reconnect_task: Option<Task<()>>,
    reconnect_interval: Duration,
    entities_by_type_and_remote_id: HashMap<(TypeId, u64), AnyWeakEntityHandle>,
    models_by_message_type: HashMap<TypeId, AnyWeakModelHandle>,
    entity_types_by_message_type: HashMap<TypeId, TypeId>,
    message_handlers: HashMap<
        TypeId,
        Arc<
            dyn Send
                + Sync
                + Fn(
                    AnyEntityHandle,
                    Box<dyn AnyTypedEnvelope>,
                    &Arc<Client>,
                    AsyncAppContext,
                ) -> LocalBoxFuture<'static, Result<()>>,
        >,
    >,
}

enum AnyWeakEntityHandle {
    Model(AnyWeakModelHandle),
    View(AnyWeakViewHandle),
}

enum AnyEntityHandle {
    Model(AnyModelHandle),
    View(AnyViewHandle),
}

#[derive(Clone, Debug)]
pub struct Credentials {
    pub user_id: u64,
    pub access_token: String,
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            credentials: None,
            status: watch::channel_with(Status::SignedOut),
            entity_id_extractors: Default::default(),
            _reconnect_task: None,
            reconnect_interval: Duration::from_secs(5),
            models_by_message_type: Default::default(),
            entities_by_type_and_remote_id: Default::default(),
            entity_types_by_message_type: Default::default(),
            message_handlers: Default::default(),
        }
    }
}

pub enum Subscription {
    Entity {
        client: Weak<Client>,
        id: (TypeId, u64),
    },
    Message {
        client: Weak<Client>,
        id: TypeId,
    },
}

impl Drop for Subscription {
    fn drop(&mut self) {
        match self {
            Subscription::Entity { client, id } => {
                if let Some(client) = client.upgrade() {
                    let mut state = client.state.write();
                    let _ = state.entities_by_type_and_remote_id.remove(id);
                }
            }
            Subscription::Message { client, id } => {
                if let Some(client) = client.upgrade() {
                    let mut state = client.state.write();
                    let _ = state.entity_types_by_message_type.remove(id);
                    let _ = state.message_handlers.remove(id);
                }
            }
        }
    }
}

impl Client {
    pub fn new(http: Arc<dyn HttpClient>) -> Arc<Self> {
        lazy_static! {
            static ref NEXT_CLIENT_ID: AtomicUsize = AtomicUsize::default();
        }

        Arc::new(Self {
            id: NEXT_CLIENT_ID.fetch_add(1, Ordering::SeqCst),
            peer: Peer::new(),
            http,
            state: Default::default(),
            authenticate: None,
            establish_connection: None,
        })
    }

    pub fn id(&self) -> usize {
        self.id
    }

    pub fn http_client(&self) -> Arc<dyn HttpClient> {
        self.http.clone()
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn tear_down(&self) {
        let mut state = self.state.write();
        state._reconnect_task.take();
        state.message_handlers.clear();
        state.models_by_message_type.clear();
        state.entities_by_type_and_remote_id.clear();
        state.entity_id_extractors.clear();
        self.peer.reset();
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn override_authenticate<F>(&mut self, authenticate: F) -> &mut Self
    where
        F: 'static + Send + Sync + Fn(&AsyncAppContext) -> Task<Result<Credentials>>,
    {
        self.authenticate = Some(Box::new(authenticate));
        self
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn override_establish_connection<F>(&mut self, connect: F) -> &mut Self
    where
        F: 'static
            + Send
            + Sync
            + Fn(&Credentials, &AsyncAppContext) -> Task<Result<Connection, EstablishConnectionError>>,
    {
        self.establish_connection = Some(Box::new(connect));
        self
    }

    pub fn user_id(&self) -> Option<u64> {
        self.state
            .read()
            .credentials
            .as_ref()
            .map(|credentials| credentials.user_id)
    }

    pub fn status(&self) -> watch::Receiver<Status> {
        self.state.read().status.1.clone()
    }

    fn set_status(self: &Arc<Self>, status: Status, cx: &AsyncAppContext) {
        let mut state = self.state.write();
        *state.status.0.borrow_mut() = status;

        match status {
            Status::Connected { .. } => {
                state._reconnect_task = None;
            }
            Status::ConnectionLost => {
                let this = self.clone();
                let reconnect_interval = state.reconnect_interval;
                state._reconnect_task = Some(cx.spawn(|cx| async move {
                    let mut rng = StdRng::from_entropy();
                    let mut delay = Duration::from_millis(100);
                    while let Err(error) = this.authenticate_and_connect(true, &cx).await {
                        log::error!("failed to connect {}", error);
                        this.set_status(
                            Status::ReconnectionError {
                                next_reconnection: Instant::now() + delay,
                            },
                            &cx,
                        );
                        cx.background().timer(delay).await;
                        delay = delay
                            .mul_f32(rng.gen_range(1.0..=2.0))
                            .min(reconnect_interval);
                    }
                }));
            }
            Status::SignedOut | Status::UpgradeRequired => {
                state._reconnect_task.take();
            }
            _ => {}
        }
    }

    pub fn add_view_for_remote_entity<T: View>(
        self: &Arc<Self>,
        remote_id: u64,
        cx: &mut ViewContext<T>,
    ) -> Subscription {
        let id = (TypeId::of::<T>(), remote_id);
        self.state
            .write()
            .entities_by_type_and_remote_id
            .insert(id, AnyWeakEntityHandle::View(cx.weak_handle().into()));
        Subscription::Entity {
            client: Arc::downgrade(self),
            id,
        }
    }

    pub fn add_model_for_remote_entity<T: Entity>(
        self: &Arc<Self>,
        remote_id: u64,
        cx: &mut ModelContext<T>,
    ) -> Subscription {
        let id = (TypeId::of::<T>(), remote_id);
        self.state
            .write()
            .entities_by_type_and_remote_id
            .insert(id, AnyWeakEntityHandle::Model(cx.weak_handle().into()));
        Subscription::Entity {
            client: Arc::downgrade(self),
            id,
        }
    }

    pub fn add_message_handler<M, E, H, F>(
        self: &Arc<Self>,
        model: ModelHandle<E>,
        handler: H,
    ) -> Subscription
    where
        M: EnvelopedMessage,
        E: Entity,
        H: 'static
            + Send
            + Sync
            + Fn(ModelHandle<E>, TypedEnvelope<M>, Arc<Self>, AsyncAppContext) -> F,
        F: 'static + Future<Output = Result<()>>,
    {
        let message_type_id = TypeId::of::<M>();

        let mut state = self.state.write();
        state
            .models_by_message_type
            .insert(message_type_id, model.downgrade().into());

        let prev_handler = state.message_handlers.insert(
            message_type_id,
            Arc::new(move |handle, envelope, client, cx| {
                let handle = if let AnyEntityHandle::Model(handle) = handle {
                    handle
                } else {
                    unreachable!();
                };
                let model = handle.downcast::<E>().unwrap();
                let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                handler(model, *envelope, client.clone(), cx).boxed_local()
            }),
        );
        if prev_handler.is_some() {
            panic!("registered handler for the same message twice");
        }

        Subscription::Message {
            client: Arc::downgrade(self),
            id: message_type_id,
        }
    }

    pub fn add_view_message_handler<M, E, H, F>(self: &Arc<Self>, handler: H)
    where
        M: EntityMessage,
        E: View,
        H: 'static
            + Send
            + Sync
            + Fn(ViewHandle<E>, TypedEnvelope<M>, Arc<Self>, AsyncAppContext) -> F,
        F: 'static + Future<Output = Result<()>>,
    {
        self.add_entity_message_handler::<M, E, _, _>(move |handle, message, client, cx| {
            if let AnyEntityHandle::View(handle) = handle {
                handler(handle.downcast::<E>().unwrap(), message, client, cx)
            } else {
                unreachable!();
            }
        })
    }

    pub fn add_model_message_handler<M, E, H, F>(self: &Arc<Self>, handler: H)
    where
        M: EntityMessage,
        E: Entity,
        H: 'static
            + Send
            + Sync
            + Fn(ModelHandle<E>, TypedEnvelope<M>, Arc<Self>, AsyncAppContext) -> F,
        F: 'static + Future<Output = Result<()>>,
    {
        self.add_entity_message_handler::<M, E, _, _>(move |handle, message, client, cx| {
            if let AnyEntityHandle::Model(handle) = handle {
                handler(handle.downcast::<E>().unwrap(), message, client, cx)
            } else {
                unreachable!();
            }
        })
    }

    fn add_entity_message_handler<M, E, H, F>(self: &Arc<Self>, handler: H)
    where
        M: EntityMessage,
        E: Entity,
        H: 'static
            + Send
            + Sync
            + Fn(AnyEntityHandle, TypedEnvelope<M>, Arc<Self>, AsyncAppContext) -> F,
        F: 'static + Future<Output = Result<()>>,
    {
        let model_type_id = TypeId::of::<E>();
        let message_type_id = TypeId::of::<M>();

        let mut state = self.state.write();
        state
            .entity_types_by_message_type
            .insert(message_type_id, model_type_id);
        state
            .entity_id_extractors
            .entry(message_type_id)
            .or_insert_with(|| {
                |envelope| {
                    envelope
                        .as_any()
                        .downcast_ref::<TypedEnvelope<M>>()
                        .unwrap()
                        .payload
                        .remote_entity_id()
                }
            });
        let prev_handler = state.message_handlers.insert(
            message_type_id,
            Arc::new(move |handle, envelope, client, cx| {
                let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                handler(handle, *envelope, client.clone(), cx).boxed_local()
            }),
        );
        if prev_handler.is_some() {
            panic!("registered handler for the same message twice");
        }
    }

    pub fn add_model_request_handler<M, E, H, F>(self: &Arc<Self>, handler: H)
    where
        M: EntityMessage + RequestMessage,
        E: Entity,
        H: 'static
            + Send
            + Sync
            + Fn(ModelHandle<E>, TypedEnvelope<M>, Arc<Self>, AsyncAppContext) -> F,
        F: 'static + Future<Output = Result<M::Response>>,
    {
        self.add_model_message_handler(move |entity, envelope, client, cx| {
            Self::respond_to_request::<M, _>(
                envelope.receipt(),
                handler(entity, envelope, client.clone(), cx),
                client,
            )
        })
    }

    pub fn add_view_request_handler<M, E, H, F>(self: &Arc<Self>, handler: H)
    where
        M: EntityMessage + RequestMessage,
        E: View,
        H: 'static
            + Send
            + Sync
            + Fn(ViewHandle<E>, TypedEnvelope<M>, Arc<Self>, AsyncAppContext) -> F,
        F: 'static + Future<Output = Result<M::Response>>,
    {
        self.add_view_message_handler(move |entity, envelope, client, cx| {
            Self::respond_to_request::<M, _>(
                envelope.receipt(),
                handler(entity, envelope, client.clone(), cx),
                client,
            )
        })
    }

    async fn respond_to_request<T: RequestMessage, F: Future<Output = Result<T::Response>>>(
        receipt: Receipt<T>,
        response: F,
        client: Arc<Self>,
    ) -> Result<()> {
        match response.await {
            Ok(response) => {
                client.respond(receipt, response)?;
                Ok(())
            }
            Err(error) => {
                client.respond_with_error(
                    receipt,
                    proto::Error {
                        message: error.to_string(),
                    },
                )?;
                Err(error)
            }
        }
    }

    pub fn has_keychain_credentials(&self, cx: &AsyncAppContext) -> bool {
        read_credentials_from_keychain(cx).is_some()
    }

    #[async_recursion(?Send)]
    pub async fn authenticate_and_connect(
        self: &Arc<Self>,
        try_keychain: bool,
        cx: &AsyncAppContext,
    ) -> anyhow::Result<()> {
        let was_disconnected = match *self.status().borrow() {
            Status::SignedOut => true,
            Status::ConnectionError | Status::ConnectionLost | Status::ReconnectionError { .. } => {
                false
            }
            Status::Connected { .. }
            | Status::Connecting { .. }
            | Status::Reconnecting { .. }
            | Status::Authenticating
            | Status::Reauthenticating => return Ok(()),
            Status::UpgradeRequired => return Err(EstablishConnectionError::UpgradeRequired)?,
        };

        if was_disconnected {
            self.set_status(Status::Authenticating, cx);
        } else {
            self.set_status(Status::Reauthenticating, cx)
        }

        let mut read_from_keychain = false;
        let mut credentials = self.state.read().credentials.clone();
        if credentials.is_none() && try_keychain {
            credentials = read_credentials_from_keychain(cx);
            read_from_keychain = credentials.is_some();
        }
        if credentials.is_none() {
            credentials = Some(match self.authenticate(&cx).await {
                Ok(credentials) => credentials,
                Err(err) => {
                    self.set_status(Status::ConnectionError, cx);
                    return Err(err);
                }
            });
        }
        let credentials = credentials.unwrap();

        if was_disconnected {
            self.set_status(Status::Connecting, cx);
        } else {
            self.set_status(Status::Reconnecting, cx);
        }

        match self.establish_connection(&credentials, cx).await {
            Ok(conn) => {
                self.state.write().credentials = Some(credentials.clone());
                if !read_from_keychain && IMPERSONATE_LOGIN.is_none() {
                    write_credentials_to_keychain(&credentials, cx).log_err();
                }
                self.set_connection(conn, cx).await;
                Ok(())
            }
            Err(EstablishConnectionError::Unauthorized) => {
                self.state.write().credentials.take();
                if read_from_keychain {
                    cx.platform().delete_credentials(&ZED_SERVER_URL).log_err();
                    self.set_status(Status::SignedOut, cx);
                    self.authenticate_and_connect(false, cx).await
                } else {
                    self.set_status(Status::ConnectionError, cx);
                    Err(EstablishConnectionError::Unauthorized)?
                }
            }
            Err(EstablishConnectionError::UpgradeRequired) => {
                self.set_status(Status::UpgradeRequired, cx);
                Err(EstablishConnectionError::UpgradeRequired)?
            }
            Err(error) => {
                self.set_status(Status::ConnectionError, cx);
                Err(error)?
            }
        }
    }

    async fn set_connection(self: &Arc<Self>, conn: Connection, cx: &AsyncAppContext) {
        let executor = cx.background();
        let (connection_id, handle_io, mut incoming) = self
            .peer
            .add_connection(conn, move |duration| executor.timer(duration))
            .await;
        cx.foreground()
            .spawn({
                let cx = cx.clone();
                let this = self.clone();
                async move {
                    let mut message_id = 0_usize;
                    while let Some(message) = incoming.next().await {
                        let mut state = this.state.write();
                        message_id += 1;
                        let type_name = message.payload_type_name();
                        let payload_type_id = message.payload_type_id();
                        let sender_id = message.original_sender_id().map(|id| id.0);

                        let model = state
                            .models_by_message_type
                            .get(&payload_type_id)
                            .and_then(|model| model.upgrade(&cx))
                            .map(AnyEntityHandle::Model)
                            .or_else(|| {
                                let entity_type_id =
                                    *state.entity_types_by_message_type.get(&payload_type_id)?;
                                let entity_id = state
                                    .entity_id_extractors
                                    .get(&message.payload_type_id())
                                    .map(|extract_entity_id| {
                                        (extract_entity_id)(message.as_ref())
                                    })?;

                                let entity = state
                                    .entities_by_type_and_remote_id
                                    .get(&(entity_type_id, entity_id))?;
                                if let Some(entity) = entity.upgrade(&cx) {
                                    Some(entity)
                                } else {
                                    state
                                        .entities_by_type_and_remote_id
                                        .remove(&(entity_type_id, entity_id));
                                    None
                                }
                            });

                        let model = if let Some(model) = model {
                            model
                        } else {
                            log::info!("unhandled message {}", type_name);
                            continue;
                        };

                        if let Some(handler) = state.message_handlers.get(&payload_type_id).cloned()
                        {
                            drop(state); // Avoid deadlocks if the handler interacts with rpc::Client
                            let future = handler(model, message, &this, cx.clone());

                            let client_id = this.id;
                            log::debug!(
                                "rpc message received. client_id:{}, message_id:{}, sender_id:{:?}, type:{}",
                                client_id,
                                message_id,
                                sender_id,
                                type_name
                            );
                            cx.foreground()
                                .spawn(async move {
                                    match future.await {
                                        Ok(()) => {
                                            log::debug!(
                                                "rpc message handled. client_id:{}, message_id:{}, sender_id:{:?}, type:{}",
                                                client_id,
                                                message_id,
                                                sender_id,
                                                type_name
                                            );
                                        }
                                        Err(error) => {
                                            log::error!(
                                                "error handling message. client_id:{}, message_id:{}, sender_id:{:?}, type:{}, error:{:?}",
                                                client_id,
                                                message_id,
                                                sender_id,
                                                type_name,
                                                error
                                            );
                                        }
                                    }
                                })
                                .detach();
                        } else {
                            log::info!("unhandled message {}", type_name);
                        }

                        // Don't starve the main thread when receiving lots of messages at once.
                        smol::future::yield_now().await;
                    }
                }
            })
            .detach();

        self.set_status(Status::Connected { connection_id }, cx);

        let handle_io = cx.background().spawn(handle_io);
        let this = self.clone();
        let cx = cx.clone();
        cx.foreground()
            .spawn(async move {
                match handle_io.await {
                    Ok(()) => this.set_status(Status::SignedOut, &cx),
                    Err(err) => {
                        log::error!("connection error: {:?}", err);
                        this.set_status(Status::ConnectionLost, &cx);
                    }
                }
            })
            .detach();
    }

    fn authenticate(self: &Arc<Self>, cx: &AsyncAppContext) -> Task<Result<Credentials>> {
        if let Some(callback) = self.authenticate.as_ref() {
            callback(cx)
        } else {
            self.authenticate_with_browser(cx)
        }
    }

    fn establish_connection(
        self: &Arc<Self>,
        credentials: &Credentials,
        cx: &AsyncAppContext,
    ) -> Task<Result<Connection, EstablishConnectionError>> {
        if let Some(callback) = self.establish_connection.as_ref() {
            callback(credentials, cx)
        } else {
            self.establish_websocket_connection(credentials, cx)
        }
    }

    fn establish_websocket_connection(
        self: &Arc<Self>,
        credentials: &Credentials,
        cx: &AsyncAppContext,
    ) -> Task<Result<Connection, EstablishConnectionError>> {
        let request = Request::builder()
            .header(
                "Authorization",
                format!("{} {}", credentials.user_id, credentials.access_token),
            )
            .header("X-Zed-Protocol-Version", rpc::PROTOCOL_VERSION);

        let http = self.http.clone();
        cx.background().spawn(async move {
            let mut rpc_url = format!("{}/rpc", *ZED_SERVER_URL);
            let rpc_request = surf::Request::new(
                Method::Get,
                surf::Url::parse(&rpc_url).context("invalid ZED_SERVER_URL")?,
            );
            let rpc_response = http.send(rpc_request).await?;

            if rpc_response.status().is_redirection() {
                rpc_url = rpc_response
                    .header("Location")
                    .ok_or_else(|| anyhow!("missing location header in /rpc response"))?
                    .as_str()
                    .to_string();
            }
            // Until we switch the zed.dev domain to point to the new Next.js app, there
            // will be no redirect required, and the app will connect directly to
            // wss://zed.dev/rpc.
            else if rpc_response.status() != surf::StatusCode::UpgradeRequired {
                Err(anyhow!(
                    "unexpected /rpc response status {}",
                    rpc_response.status()
                ))?
            }

            let mut rpc_url = surf::Url::parse(&rpc_url).context("invalid rpc url")?;
            let rpc_host = rpc_url
                .host_str()
                .zip(rpc_url.port_or_known_default())
                .ok_or_else(|| anyhow!("missing host in rpc url"))?;
            let stream = smol::net::TcpStream::connect(rpc_host).await?;

            log::info!("connected to rpc endpoint {}", rpc_url);

            match rpc_url.scheme() {
                "https" => {
                    rpc_url.set_scheme("wss").unwrap();
                    let request = request.uri(rpc_url.as_str()).body(())?;
                    let (stream, _) =
                        async_tungstenite::async_tls::client_async_tls(request, stream).await?;
                    Ok(Connection::new(stream))
                }
                "http" => {
                    rpc_url.set_scheme("ws").unwrap();
                    let request = request.uri(rpc_url.as_str()).body(())?;
                    let (stream, _) = async_tungstenite::client_async(request, stream).await?;
                    Ok(Connection::new(stream))
                }
                _ => Err(anyhow!("invalid rpc url: {}", rpc_url))?,
            }
        })
    }

    pub fn authenticate_with_browser(
        self: &Arc<Self>,
        cx: &AsyncAppContext,
    ) -> Task<Result<Credentials>> {
        let platform = cx.platform();
        let executor = cx.background();
        executor.clone().spawn(async move {
            // Generate a pair of asymmetric encryption keys. The public key will be used by the
            // zed server to encrypt the user's access token, so that it can'be intercepted by
            // any other app running on the user's device.
            let (public_key, private_key) =
                rpc::auth::keypair().expect("failed to generate keypair for auth");
            let public_key_string =
                String::try_from(public_key).expect("failed to serialize public key for auth");

            // Start an HTTP server to receive the redirect from Zed's sign-in page.
            let server = tiny_http::Server::http("127.0.0.1:0").expect("failed to find open port");
            let port = server.server_addr().port();

            // Open the Zed sign-in page in the user's browser, with query parameters that indicate
            // that the user is signing in from a Zed app running on the same device.
            let mut url = format!(
                "{}/native_app_signin?native_app_port={}&native_app_public_key={}",
                *ZED_SERVER_URL, port, public_key_string
            );

            if let Some(impersonate_login) = IMPERSONATE_LOGIN.as_ref() {
                log::info!("impersonating user @{}", impersonate_login);
                write!(&mut url, "&impersonate={}", impersonate_login).unwrap();
            }

            platform.open_url(&url);

            // Receive the HTTP request from the user's browser. Retrieve the user id and encrypted
            // access token from the query params.
            //
            // TODO - Avoid ever starting more than one HTTP server. Maybe switch to using a
            // custom URL scheme instead of this local HTTP server.
            let (user_id, access_token) = executor
                .spawn(async move {
                    if let Some(req) = server.recv_timeout(Duration::from_secs(10 * 60))? {
                        let path = req.url();
                        let mut user_id = None;
                        let mut access_token = None;
                        let url = Url::parse(&format!("http://example.com{}", path))
                            .context("failed to parse login notification url")?;
                        for (key, value) in url.query_pairs() {
                            if key == "access_token" {
                                access_token = Some(value.to_string());
                            } else if key == "user_id" {
                                user_id = Some(value.to_string());
                            }
                        }

                        let post_auth_url =
                            format!("{}/native_app_signin_succeeded", *ZED_SERVER_URL);
                        req.respond(
                            tiny_http::Response::empty(302).with_header(
                                tiny_http::Header::from_bytes(
                                    &b"Location"[..],
                                    post_auth_url.as_bytes(),
                                )
                                .unwrap(),
                            ),
                        )
                        .context("failed to respond to login http request")?;
                        Ok((
                            user_id.ok_or_else(|| anyhow!("missing user_id parameter"))?,
                            access_token
                                .ok_or_else(|| anyhow!("missing access_token parameter"))?,
                        ))
                    } else {
                        Err(anyhow!("didn't receive login redirect"))
                    }
                })
                .await?;

            let access_token = private_key
                .decrypt_string(&access_token)
                .context("failed to decrypt access token")?;
            platform.activate(true);

            Ok(Credentials {
                user_id: user_id.parse()?,
                access_token,
            })
        })
    }

    pub fn disconnect(self: &Arc<Self>, cx: &AsyncAppContext) -> Result<()> {
        let conn_id = self.connection_id()?;
        self.peer.disconnect(conn_id);
        self.set_status(Status::SignedOut, cx);
        Ok(())
    }

    fn connection_id(&self) -> Result<ConnectionId> {
        if let Status::Connected { connection_id, .. } = *self.status().borrow() {
            Ok(connection_id)
        } else {
            Err(anyhow!("not connected"))
        }
    }

    pub fn send<T: EnvelopedMessage>(&self, message: T) -> Result<()> {
        log::debug!("rpc send. client_id:{}, name:{}", self.id, T::NAME);
        self.peer.send(self.connection_id()?, message)
    }

    pub fn request<T: RequestMessage>(
        &self,
        request: T,
    ) -> impl Future<Output = Result<T::Response>> {
        let client_id = self.id;
        log::debug!(
            "rpc request start. client_id:{}. name:{}",
            client_id,
            T::NAME
        );
        let response = self
            .connection_id()
            .map(|conn_id| self.peer.request(conn_id, request));
        async move {
            let response = response?.await;
            log::debug!(
                "rpc request finish. client_id:{}. name:{}",
                client_id,
                T::NAME
            );
            response
        }
    }

    fn respond<T: RequestMessage>(&self, receipt: Receipt<T>, response: T::Response) -> Result<()> {
        log::debug!("rpc respond. client_id:{}. name:{}", self.id, T::NAME);
        self.peer.respond(receipt, response)
    }

    fn respond_with_error<T: RequestMessage>(
        &self,
        receipt: Receipt<T>,
        error: proto::Error,
    ) -> Result<()> {
        log::debug!("rpc respond. client_id:{}. name:{}", self.id, T::NAME);
        self.peer.respond_with_error(receipt, error)
    }
}

impl AnyWeakEntityHandle {
    fn upgrade(&self, cx: &AsyncAppContext) -> Option<AnyEntityHandle> {
        match self {
            AnyWeakEntityHandle::Model(handle) => handle.upgrade(cx).map(AnyEntityHandle::Model),
            AnyWeakEntityHandle::View(handle) => handle.upgrade(cx).map(AnyEntityHandle::View),
        }
    }
}

fn read_credentials_from_keychain(cx: &AsyncAppContext) -> Option<Credentials> {
    if IMPERSONATE_LOGIN.is_some() {
        return None;
    }

    let (user_id, access_token) = cx
        .platform()
        .read_credentials(&ZED_SERVER_URL)
        .log_err()
        .flatten()?;
    Some(Credentials {
        user_id: user_id.parse().ok()?,
        access_token: String::from_utf8(access_token).ok()?,
    })
}

fn write_credentials_to_keychain(credentials: &Credentials, cx: &AsyncAppContext) -> Result<()> {
    cx.platform().write_credentials(
        &ZED_SERVER_URL,
        &credentials.user_id.to_string(),
        credentials.access_token.as_bytes(),
    )
}

const WORKTREE_URL_PREFIX: &'static str = "zed://worktrees/";

pub fn encode_worktree_url(id: u64, access_token: &str) -> String {
    format!("{}{}/{}", WORKTREE_URL_PREFIX, id, access_token)
}

pub fn decode_worktree_url(url: &str) -> Option<(u64, String)> {
    let path = url.trim().strip_prefix(WORKTREE_URL_PREFIX)?;
    let mut parts = path.split('/');
    let id = parts.next()?.parse::<u64>().ok()?;
    let access_token = parts.next()?;
    if access_token.is_empty() {
        return None;
    }
    Some((id, access_token.to_string()))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::{FakeHttpClient, FakeServer};
    use gpui::TestAppContext;

    #[gpui::test(iterations = 10)]
    async fn test_reconnection(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();

        let user_id = 5;
        let mut client = Client::new(FakeHttpClient::with_404_response());
        let server = FakeServer::for_client(user_id, &mut client, &cx).await;
        let mut status = client.status();
        assert!(matches!(
            status.next().await,
            Some(Status::Connected { .. })
        ));
        assert_eq!(server.auth_count(), 1);

        server.forbid_connections();
        server.disconnect();
        while !matches!(status.next().await, Some(Status::ReconnectionError { .. })) {}

        server.allow_connections();
        cx.foreground().advance_clock(Duration::from_secs(10));
        while !matches!(status.next().await, Some(Status::Connected { .. })) {}
        assert_eq!(server.auth_count(), 1); // Client reused the cached credentials when reconnecting

        server.forbid_connections();
        server.disconnect();
        while !matches!(status.next().await, Some(Status::ReconnectionError { .. })) {}

        // Clear cached credentials after authentication fails
        server.roll_access_token();
        server.allow_connections();
        cx.foreground().advance_clock(Duration::from_secs(10));
        while !matches!(status.next().await, Some(Status::Connected { .. })) {}
        assert_eq!(server.auth_count(), 2); // Client re-authenticated due to an invalid token
    }

    #[test]
    fn test_encode_and_decode_worktree_url() {
        let url = encode_worktree_url(5, "deadbeef");
        assert_eq!(decode_worktree_url(&url), Some((5, "deadbeef".to_string())));
        assert_eq!(
            decode_worktree_url(&format!("\n {}\t", url)),
            Some((5, "deadbeef".to_string()))
        );
        assert_eq!(decode_worktree_url("not://the-right-format"), None);
    }

    #[gpui::test]
    async fn test_subscribing_to_entity(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();

        let user_id = 5;
        let mut client = Client::new(FakeHttpClient::with_404_response());
        let server = FakeServer::for_client(user_id, &mut client, &cx).await;

        let (done_tx1, mut done_rx1) = smol::channel::unbounded();
        let (done_tx2, mut done_rx2) = smol::channel::unbounded();
        client.add_model_message_handler(
            move |model: ModelHandle<Model>, _: TypedEnvelope<proto::UnshareProject>, _, cx| {
                match model.read_with(&cx, |model, _| model.id) {
                    1 => done_tx1.try_send(()).unwrap(),
                    2 => done_tx2.try_send(()).unwrap(),
                    _ => unreachable!(),
                }
                async { Ok(()) }
            },
        );
        let model1 = cx.add_model(|_| Model {
            id: 1,
            subscription: None,
        });
        let model2 = cx.add_model(|_| Model {
            id: 2,
            subscription: None,
        });
        let model3 = cx.add_model(|_| Model {
            id: 3,
            subscription: None,
        });

        let _subscription1 = model1.update(cx, |_, cx| client.add_model_for_remote_entity(1, cx));
        let _subscription2 = model2.update(cx, |_, cx| client.add_model_for_remote_entity(2, cx));
        // Ensure dropping a subscription for the same entity type still allows receiving of
        // messages for other entity IDs of the same type.
        let subscription3 = model3.update(cx, |_, cx| client.add_model_for_remote_entity(3, cx));
        drop(subscription3);

        server.send(proto::UnshareProject { project_id: 1 });
        server.send(proto::UnshareProject { project_id: 2 });
        done_rx1.next().await.unwrap();
        done_rx2.next().await.unwrap();
    }

    #[gpui::test]
    async fn test_subscribing_after_dropping_subscription(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();

        let user_id = 5;
        let mut client = Client::new(FakeHttpClient::with_404_response());
        let server = FakeServer::for_client(user_id, &mut client, &cx).await;

        let model = cx.add_model(|_| Model::default());
        let (done_tx1, _done_rx1) = smol::channel::unbounded();
        let (done_tx2, mut done_rx2) = smol::channel::unbounded();
        let subscription1 = client.add_message_handler(
            model.clone(),
            move |_, _: TypedEnvelope<proto::Ping>, _, _| {
                done_tx1.try_send(()).unwrap();
                async { Ok(()) }
            },
        );
        drop(subscription1);
        let _subscription2 =
            client.add_message_handler(model, move |_, _: TypedEnvelope<proto::Ping>, _, _| {
                done_tx2.try_send(()).unwrap();
                async { Ok(()) }
            });
        server.send(proto::Ping {});
        done_rx2.next().await.unwrap();
    }

    #[gpui::test]
    async fn test_dropping_subscription_in_handler(cx: &mut TestAppContext) {
        cx.foreground().forbid_parking();

        let user_id = 5;
        let mut client = Client::new(FakeHttpClient::with_404_response());
        let server = FakeServer::for_client(user_id, &mut client, &cx).await;

        let model = cx.add_model(|_| Model::default());
        let (done_tx, mut done_rx) = smol::channel::unbounded();
        let subscription = client.add_message_handler(
            model.clone(),
            move |model, _: TypedEnvelope<proto::Ping>, _, mut cx| {
                model.update(&mut cx, |model, _| model.subscription.take());
                done_tx.try_send(()).unwrap();
                async { Ok(()) }
            },
        );
        model.update(cx, |model, _| {
            model.subscription = Some(subscription);
        });
        server.send(proto::Ping {});
        done_rx.next().await.unwrap();
    }

    #[derive(Default)]
    struct Model {
        id: usize,
        subscription: Option<Subscription>,
    }

    impl Entity for Model {
        type Event = ();
    }
}
