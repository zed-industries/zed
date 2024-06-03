#[cfg(any(test, feature = "test-support"))]
pub mod test;

pub mod telemetry;
pub mod user;

use anyhow::{anyhow, Context as _, Result};
use async_recursion::async_recursion;
use async_tungstenite::tungstenite::{
    error::Error as WebsocketError,
    http::{Request, StatusCode},
};
use clock::SystemClock;
use collections::HashMap;
use futures::{
    channel::oneshot, future::LocalBoxFuture, AsyncReadExt, FutureExt, SinkExt, Stream, StreamExt,
    TryFutureExt as _, TryStreamExt,
};
use gpui::{
    actions, AnyModel, AnyWeakModel, AppContext, AsyncAppContext, Global, Model, Task, WeakModel,
};
use http::{HttpClient, HttpClientWithUrl};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use postage::watch;
use rand::prelude::*;
use release_channel::{AppVersion, ReleaseChannel};
use rpc::proto::{AnyTypedEnvelope, EntityMessage, EnvelopedMessage, PeerId, RequestMessage};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsSources};
use std::fmt;
use std::pin::Pin;
use std::{
    any::TypeId,
    convert::TryFrom,
    fmt::Write as _,
    future::Future,
    marker::PhantomData,
    path::PathBuf,
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Weak,
    },
    time::{Duration, Instant},
};
use telemetry::Telemetry;
use thiserror::Error;
use url::Url;
use util::{ResultExt, TryFutureExt};

pub use rpc::*;
pub use telemetry_events::Event;
pub use user::*;

#[derive(Debug, Clone, Eq, PartialEq)]
pub struct DevServerToken(pub String);

impl fmt::Display for DevServerToken {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

lazy_static! {
    static ref ZED_SERVER_URL: Option<String> = std::env::var("ZED_SERVER_URL").ok();
    static ref ZED_RPC_URL: Option<String> = std::env::var("ZED_RPC_URL").ok();
    /// An environment variable whose presence indicates that the development auth
    /// provider should be used.
    ///
    /// Only works in development. Setting this environment variable in other release
    /// channels is a no-op.
    pub static ref ZED_DEVELOPMENT_AUTH: bool =
        std::env::var("ZED_DEVELOPMENT_AUTH").map_or(false, |value| !value.is_empty());
    pub static ref IMPERSONATE_LOGIN: Option<String> = std::env::var("ZED_IMPERSONATE")
        .ok()
        .and_then(|s| if s.is_empty() { None } else { Some(s) });
    pub static ref ADMIN_API_TOKEN: Option<String> = std::env::var("ZED_ADMIN_API_TOKEN")
        .ok()
        .and_then(|s| if s.is_empty() { None } else { Some(s) });
    pub static ref ZED_APP_PATH: Option<PathBuf> =
        std::env::var("ZED_APP_PATH").ok().map(PathBuf::from);
    pub static ref ZED_ALWAYS_ACTIVE: bool =
        std::env::var("ZED_ALWAYS_ACTIVE").map_or(false, |e| !e.is_empty());
}

pub const INITIAL_RECONNECTION_DELAY: Duration = Duration::from_millis(500);
pub const MAX_RECONNECTION_DELAY: Duration = Duration::from_secs(10);
pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(20);

actions!(client, [SignIn, SignOut, Reconnect]);

#[derive(Clone, Default, Serialize, Deserialize, JsonSchema)]
pub struct ClientSettingsContent {
    server_url: Option<String>,
}

#[derive(Deserialize)]
pub struct ClientSettings {
    pub server_url: String,
}

impl Settings for ClientSettings {
    const KEY: Option<&'static str> = None;

    type FileContent = ClientSettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        let mut result = sources.json_merge::<Self>()?;
        if let Some(server_url) = &*ZED_SERVER_URL {
            result.server_url.clone_from(&server_url)
        }
        Ok(result)
    }
}

#[derive(Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct ProxySettingsContent {
    proxy: Option<String>,
}

#[derive(Deserialize, Default)]
pub struct ProxySettings {
    pub proxy: Option<String>,
}

impl Settings for ProxySettings {
    const KEY: Option<&'static str> = None;

    type FileContent = ProxySettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        Ok(Self {
            proxy: sources
                .user
                .and_then(|value| value.proxy.clone())
                .or(sources.default.proxy.clone()),
        })
    }
}

pub fn init_settings(cx: &mut AppContext) {
    TelemetrySettings::register(cx);
    ClientSettings::register(cx);
    ProxySettings::register(cx);
}

pub fn init(client: &Arc<Client>, cx: &mut AppContext) {
    let client = Arc::downgrade(client);
    cx.on_action({
        let client = client.clone();
        move |_: &SignIn, cx| {
            if let Some(client) = client.upgrade() {
                cx.spawn(
                    |cx| async move { client.authenticate_and_connect(true, &cx).log_err().await },
                )
                .detach();
            }
        }
    });

    cx.on_action({
        let client = client.clone();
        move |_: &SignOut, cx| {
            if let Some(client) = client.upgrade() {
                cx.spawn(|cx| async move {
                    client.sign_out(&cx).await;
                })
                .detach();
            }
        }
    });

    cx.on_action({
        let client = client.clone();
        move |_: &Reconnect, cx| {
            if let Some(client) = client.upgrade() {
                cx.spawn(|cx| async move {
                    client.reconnect(&cx);
                })
                .detach();
            }
        }
    });
}

struct GlobalClient(Arc<Client>);

impl Global for GlobalClient {}

pub struct Client {
    id: AtomicU64,
    peer: Arc<Peer>,
    http: Arc<HttpClientWithUrl>,
    telemetry: Arc<Telemetry>,
    credentials_provider: Arc<dyn CredentialsProvider + Send + Sync + 'static>,
    state: RwLock<ClientState>,

    #[allow(clippy::type_complexity)]
    #[cfg(any(test, feature = "test-support"))]
    authenticate: RwLock<
        Option<Box<dyn 'static + Send + Sync + Fn(&AsyncAppContext) -> Task<Result<Credentials>>>>,
    >,

    #[allow(clippy::type_complexity)]
    #[cfg(any(test, feature = "test-support"))]
    establish_connection: RwLock<
        Option<
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
    Http(#[from] http::Error),
    #[error("{0}")]
    Io(#[from] std::io::Error),
    #[error("{0}")]
    Websocket(#[from] async_tungstenite::tungstenite::http::Error),
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

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum Status {
    SignedOut,
    UpgradeRequired,
    Authenticating,
    Connecting,
    ConnectionError,
    Connected {
        peer_id: PeerId,
        connection_id: ConnectionId,
    },
    ConnectionLost,
    Reauthenticating,
    Reconnecting,
    ReconnectionError {
        next_reconnection: Instant,
    },
}

impl Status {
    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Connected { .. })
    }

    pub fn is_signed_out(&self) -> bool {
        matches!(self, Self::SignedOut | Self::UpgradeRequired)
    }
}

struct ClientState {
    credentials: Option<Credentials>,
    status: (watch::Sender<Status>, watch::Receiver<Status>),
    entity_id_extractors: HashMap<TypeId, fn(&dyn AnyTypedEnvelope) -> u64>,
    _reconnect_task: Option<Task<()>>,
    entities_by_type_and_remote_id: HashMap<(TypeId, u64), WeakSubscriber>,
    models_by_message_type: HashMap<TypeId, AnyWeakModel>,
    entity_types_by_message_type: HashMap<TypeId, TypeId>,
    #[allow(clippy::type_complexity)]
    message_handlers: HashMap<
        TypeId,
        Arc<
            dyn Send
                + Sync
                + Fn(
                    AnyModel,
                    Box<dyn AnyTypedEnvelope>,
                    &Arc<Client>,
                    AsyncAppContext,
                ) -> LocalBoxFuture<'static, Result<()>>,
        >,
    >,
}

enum WeakSubscriber {
    Entity { handle: AnyWeakModel },
    Pending(Vec<Box<dyn AnyTypedEnvelope>>),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum Credentials {
    DevServer { token: DevServerToken },
    User { user_id: u64, access_token: String },
}

impl Credentials {
    pub fn authorization_header(&self) -> String {
        match self {
            Credentials::DevServer { token } => format!("dev-server-token {}", token),
            Credentials::User {
                user_id,
                access_token,
            } => format!("{} {}", user_id, access_token),
        }
    }
}

/// A provider for [`Credentials`].
///
/// Used to abstract over reading and writing credentials to some form of
/// persistence (like the system keychain).
trait CredentialsProvider {
    /// Reads the credentials from the provider.
    fn read_credentials<'a>(
        &'a self,
        cx: &'a AsyncAppContext,
    ) -> Pin<Box<dyn Future<Output = Option<Credentials>> + 'a>>;

    /// Writes the credentials to the provider.
    fn write_credentials<'a>(
        &'a self,
        user_id: u64,
        access_token: String,
        cx: &'a AsyncAppContext,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>>;

    /// Deletes the credentials from the provider.
    fn delete_credentials<'a>(
        &'a self,
        cx: &'a AsyncAppContext,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>>;
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            credentials: None,
            status: watch::channel_with(Status::SignedOut),
            entity_id_extractors: Default::default(),
            _reconnect_task: None,
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

pub struct PendingEntitySubscription<T: 'static> {
    client: Arc<Client>,
    remote_id: u64,
    _entity_type: PhantomData<T>,
    consumed: bool,
}

impl<T: 'static> PendingEntitySubscription<T> {
    pub fn set_model(mut self, model: &Model<T>, cx: &mut AsyncAppContext) -> Subscription {
        self.consumed = true;
        let mut state = self.client.state.write();
        let id = (TypeId::of::<T>(), self.remote_id);
        let Some(WeakSubscriber::Pending(messages)) =
            state.entities_by_type_and_remote_id.remove(&id)
        else {
            unreachable!()
        };

        state.entities_by_type_and_remote_id.insert(
            id,
            WeakSubscriber::Entity {
                handle: model.downgrade().into(),
            },
        );
        drop(state);
        for message in messages {
            self.client.handle_message(message, cx);
        }
        Subscription::Entity {
            client: Arc::downgrade(&self.client),
            id,
        }
    }
}

impl<T: 'static> Drop for PendingEntitySubscription<T> {
    fn drop(&mut self) {
        if !self.consumed {
            let mut state = self.client.state.write();
            if let Some(WeakSubscriber::Pending(messages)) = state
                .entities_by_type_and_remote_id
                .remove(&(TypeId::of::<T>(), self.remote_id))
            {
                for message in messages {
                    log::info!("unhandled message {}", message.payload_type_name());
                }
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct TelemetrySettings {
    pub diagnostics: bool,
    pub metrics: bool,
}

/// Control what info is collected by Zed.
#[derive(Default, Clone, Serialize, Deserialize, JsonSchema)]
pub struct TelemetrySettingsContent {
    /// Send debug info like crash reports.
    ///
    /// Default: true
    pub diagnostics: Option<bool>,
    /// Send anonymized usage data like what languages you're using Zed with.
    ///
    /// Default: true
    pub metrics: Option<bool>,
}

impl settings::Settings for TelemetrySettings {
    const KEY: Option<&'static str> = Some("telemetry");

    type FileContent = TelemetrySettingsContent;

    fn load(sources: SettingsSources<Self::FileContent>, _: &mut AppContext) -> Result<Self> {
        Ok(Self {
            diagnostics: sources.user.as_ref().and_then(|v| v.diagnostics).unwrap_or(
                sources
                    .default
                    .diagnostics
                    .ok_or_else(Self::missing_default)?,
            ),
            metrics: sources
                .user
                .as_ref()
                .and_then(|v| v.metrics)
                .unwrap_or(sources.default.metrics.ok_or_else(Self::missing_default)?),
        })
    }
}

impl Client {
    pub fn new(
        clock: Arc<dyn SystemClock>,
        http: Arc<HttpClientWithUrl>,
        cx: &mut AppContext,
    ) -> Arc<Self> {
        let use_zed_development_auth = match ReleaseChannel::try_global(cx) {
            Some(ReleaseChannel::Dev) => *ZED_DEVELOPMENT_AUTH,
            Some(ReleaseChannel::Nightly | ReleaseChannel::Preview | ReleaseChannel::Stable)
            | None => false,
        };

        let credentials_provider: Arc<dyn CredentialsProvider + Send + Sync + 'static> =
            if use_zed_development_auth {
                Arc::new(DevelopmentCredentialsProvider {
                    path: util::paths::CONFIG_DIR.join("development_auth"),
                })
            } else {
                Arc::new(KeychainCredentialsProvider)
            };

        Arc::new(Self {
            id: AtomicU64::new(0),
            peer: Peer::new(0),
            telemetry: Telemetry::new(clock, http.clone(), cx),
            http,
            credentials_provider,
            state: Default::default(),

            #[cfg(any(test, feature = "test-support"))]
            authenticate: Default::default(),
            #[cfg(any(test, feature = "test-support"))]
            establish_connection: Default::default(),
        })
    }

    pub fn production(cx: &mut AppContext) -> Arc<Self> {
        let clock = Arc::new(clock::RealSystemClock);
        let http = Arc::new(HttpClientWithUrl::new(
            &ClientSettings::get_global(cx).server_url,
            ProxySettings::get_global(cx).proxy.clone(),
        ));
        Self::new(clock, http.clone(), cx)
    }

    pub fn id(&self) -> u64 {
        self.id.load(Ordering::SeqCst)
    }

    pub fn http_client(&self) -> Arc<HttpClientWithUrl> {
        self.http.clone()
    }

    pub fn set_id(&self, id: u64) -> &Self {
        self.id.store(id, Ordering::SeqCst);
        self
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn teardown(&self) {
        let mut state = self.state.write();
        state._reconnect_task.take();
        state.message_handlers.clear();
        state.models_by_message_type.clear();
        state.entities_by_type_and_remote_id.clear();
        state.entity_id_extractors.clear();
        self.peer.teardown();
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn override_authenticate<F>(&self, authenticate: F) -> &Self
    where
        F: 'static + Send + Sync + Fn(&AsyncAppContext) -> Task<Result<Credentials>>,
    {
        *self.authenticate.write() = Some(Box::new(authenticate));
        self
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn override_establish_connection<F>(&self, connect: F) -> &Self
    where
        F: 'static
            + Send
            + Sync
            + Fn(&Credentials, &AsyncAppContext) -> Task<Result<Connection, EstablishConnectionError>>,
    {
        *self.establish_connection.write() = Some(Box::new(connect));
        self
    }

    pub fn global(cx: &AppContext) -> Arc<Self> {
        cx.global::<GlobalClient>().0.clone()
    }
    pub fn set_global(client: Arc<Client>, cx: &mut AppContext) {
        cx.set_global(GlobalClient(client))
    }

    pub fn user_id(&self) -> Option<u64> {
        if let Some(Credentials::User { user_id, .. }) = self.state.read().credentials.as_ref() {
            Some(*user_id)
        } else {
            None
        }
    }

    pub fn peer_id(&self) -> Option<PeerId> {
        if let Status::Connected { peer_id, .. } = &*self.status().borrow() {
            Some(*peer_id)
        } else {
            None
        }
    }

    pub fn status(&self) -> watch::Receiver<Status> {
        self.state.read().status.1.clone()
    }

    fn set_status(self: &Arc<Self>, status: Status, cx: &AsyncAppContext) {
        log::info!("set status on client {}: {:?}", self.id(), status);
        let mut state = self.state.write();
        *state.status.0.borrow_mut() = status;

        match status {
            Status::Connected { .. } => {
                state._reconnect_task = None;
            }
            Status::ConnectionLost => {
                let this = self.clone();
                state._reconnect_task = Some(cx.spawn(move |cx| async move {
                    #[cfg(any(test, feature = "test-support"))]
                    let mut rng = StdRng::seed_from_u64(0);
                    #[cfg(not(any(test, feature = "test-support")))]
                    let mut rng = StdRng::from_entropy();

                    let mut delay = INITIAL_RECONNECTION_DELAY;
                    while let Err(error) = this.authenticate_and_connect(true, &cx).await {
                        log::error!("failed to connect {}", error);
                        if matches!(*this.status().borrow(), Status::ConnectionError) {
                            this.set_status(
                                Status::ReconnectionError {
                                    next_reconnection: Instant::now() + delay,
                                },
                                &cx,
                            );
                            cx.background_executor().timer(delay).await;
                            delay = delay
                                .mul_f32(rng.gen_range(0.5..=2.5))
                                .max(INITIAL_RECONNECTION_DELAY)
                                .min(MAX_RECONNECTION_DELAY);
                        } else {
                            break;
                        }
                    }
                }));
            }
            Status::SignedOut | Status::UpgradeRequired => {
                self.telemetry.set_authenticated_user_info(None, false);
                state._reconnect_task.take();
            }
            _ => {}
        }
    }

    pub fn subscribe_to_entity<T>(
        self: &Arc<Self>,
        remote_id: u64,
    ) -> Result<PendingEntitySubscription<T>>
    where
        T: 'static,
    {
        let id = (TypeId::of::<T>(), remote_id);

        let mut state = self.state.write();
        if state.entities_by_type_and_remote_id.contains_key(&id) {
            return Err(anyhow!("already subscribed to entity"));
        }

        state
            .entities_by_type_and_remote_id
            .insert(id, WeakSubscriber::Pending(Default::default()));

        Ok(PendingEntitySubscription {
            client: self.clone(),
            remote_id,
            consumed: false,
            _entity_type: PhantomData,
        })
    }

    #[track_caller]
    pub fn add_message_handler<M, E, H, F>(
        self: &Arc<Self>,
        entity: WeakModel<E>,
        handler: H,
    ) -> Subscription
    where
        M: EnvelopedMessage,
        E: 'static,
        H: 'static
            + Sync
            + Fn(Model<E>, TypedEnvelope<M>, Arc<Self>, AsyncAppContext) -> F
            + Send
            + Sync,
        F: 'static + Future<Output = Result<()>>,
    {
        let message_type_id = TypeId::of::<M>();
        let mut state = self.state.write();
        state
            .models_by_message_type
            .insert(message_type_id, entity.into());

        let prev_handler = state.message_handlers.insert(
            message_type_id,
            Arc::new(move |subscriber, envelope, client, cx| {
                let subscriber = subscriber.downcast::<E>().unwrap();
                let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                handler(subscriber, *envelope, client.clone(), cx).boxed_local()
            }),
        );
        if prev_handler.is_some() {
            let location = std::panic::Location::caller();
            panic!(
                "{}:{} registered handler for the same message {} twice",
                location.file(),
                location.line(),
                std::any::type_name::<M>()
            );
        }

        Subscription::Message {
            client: Arc::downgrade(self),
            id: message_type_id,
        }
    }

    pub fn add_request_handler<M, E, H, F>(
        self: &Arc<Self>,
        model: WeakModel<E>,
        handler: H,
    ) -> Subscription
    where
        M: RequestMessage,
        E: 'static,
        H: 'static
            + Sync
            + Fn(Model<E>, TypedEnvelope<M>, Arc<Self>, AsyncAppContext) -> F
            + Send
            + Sync,
        F: 'static + Future<Output = Result<M::Response>>,
    {
        self.add_message_handler(model, move |handle, envelope, this, cx| {
            Self::respond_to_request(
                envelope.receipt(),
                handler(handle, envelope, this.clone(), cx),
                this,
            )
        })
    }

    pub fn add_model_message_handler<M, E, H, F>(self: &Arc<Self>, handler: H)
    where
        M: EntityMessage,
        E: 'static,
        H: 'static + Fn(Model<E>, TypedEnvelope<M>, Arc<Self>, AsyncAppContext) -> F + Send + Sync,
        F: 'static + Future<Output = Result<()>>,
    {
        self.add_entity_message_handler::<M, E, _, _>(move |subscriber, message, client, cx| {
            handler(subscriber.downcast::<E>().unwrap(), message, client, cx)
        })
    }

    fn add_entity_message_handler<M, E, H, F>(self: &Arc<Self>, handler: H)
    where
        M: EntityMessage,
        E: 'static,
        H: 'static + Fn(AnyModel, TypedEnvelope<M>, Arc<Self>, AsyncAppContext) -> F + Send + Sync,
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
        E: 'static,
        H: 'static + Fn(Model<E>, TypedEnvelope<M>, Arc<Self>, AsyncAppContext) -> F + Send + Sync,
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
                client.respond_with_error(receipt, error.to_proto())?;
                Err(error)
            }
        }
    }

    pub async fn has_credentials(&self, cx: &AsyncAppContext) -> bool {
        self.credentials_provider
            .read_credentials(cx)
            .await
            .is_some()
    }

    pub fn set_dev_server_token(&self, token: DevServerToken) -> &Self {
        self.state.write().credentials = Some(Credentials::DevServer { token });
        self
    }

    #[async_recursion(?Send)]
    pub async fn authenticate_and_connect(
        self: &Arc<Self>,
        try_provider: bool,
        cx: &AsyncAppContext,
    ) -> anyhow::Result<()> {
        let was_disconnected = match *self.status().borrow() {
            Status::SignedOut => true,
            Status::ConnectionError
            | Status::ConnectionLost
            | Status::Authenticating { .. }
            | Status::Reauthenticating { .. }
            | Status::ReconnectionError { .. } => false,
            Status::Connected { .. } | Status::Connecting { .. } | Status::Reconnecting { .. } => {
                return Ok(())
            }
            Status::UpgradeRequired => return Err(EstablishConnectionError::UpgradeRequired)?,
        };
        if was_disconnected {
            self.set_status(Status::Authenticating, cx);
        } else {
            self.set_status(Status::Reauthenticating, cx)
        }

        let mut read_from_provider = false;
        let mut credentials = self.state.read().credentials.clone();
        if credentials.is_none() && try_provider {
            credentials = self.credentials_provider.read_credentials(cx).await;
            read_from_provider = credentials.is_some();
        }

        if credentials.is_none() {
            let mut status_rx = self.status();
            let _ = status_rx.next().await;
            futures::select_biased! {
                authenticate = self.authenticate(cx).fuse() => {
                    match authenticate {
                        Ok(creds) => credentials = Some(creds),
                        Err(err) => {
                            self.set_status(Status::ConnectionError, cx);
                            return Err(err);
                        }
                    }
                }
                _ = status_rx.next().fuse() => {
                    return Err(anyhow!("authentication canceled"));
                }
            }
        }
        let credentials = credentials.unwrap();
        if let Credentials::User { user_id, .. } = &credentials {
            self.set_id(*user_id);
        }

        if was_disconnected {
            self.set_status(Status::Connecting, cx);
        } else {
            self.set_status(Status::Reconnecting, cx);
        }

        let mut timeout =
            futures::FutureExt::fuse(cx.background_executor().timer(CONNECTION_TIMEOUT));
        futures::select_biased! {
            connection = self.establish_connection(&credentials, cx).fuse() => {
                match connection {
                    Ok(conn) => {
                        self.state.write().credentials = Some(credentials.clone());
                        if !read_from_provider && IMPERSONATE_LOGIN.is_none() {
                            if let Credentials::User{user_id, access_token} = credentials {
                                self.credentials_provider.write_credentials(user_id, access_token, cx).await.log_err();
                            }
                        }

                        futures::select_biased! {
                            result = self.set_connection(conn, cx).fuse() => result,
                            _ = timeout => {
                                self.set_status(Status::ConnectionError, cx);
                                Err(anyhow!("timed out waiting on hello message from server"))
                            }
                        }
                    }
                    Err(EstablishConnectionError::Unauthorized) => {
                        self.state.write().credentials.take();
                        if read_from_provider {
                            self.credentials_provider.delete_credentials(cx).await.log_err();
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
            _ = &mut timeout => {
                self.set_status(Status::ConnectionError, cx);
                Err(anyhow!("timed out trying to establish connection"))
            }
        }
    }

    async fn set_connection(
        self: &Arc<Self>,
        conn: Connection,
        cx: &AsyncAppContext,
    ) -> Result<()> {
        let executor = cx.background_executor();
        log::info!("add connection to peer");
        let (connection_id, handle_io, mut incoming) = self.peer.add_connection(conn, {
            let executor = executor.clone();
            move |duration| executor.timer(duration)
        });
        let handle_io = executor.spawn(handle_io);

        let peer_id = async {
            log::info!("waiting for server hello");
            let message = incoming
                .next()
                .await
                .ok_or_else(|| anyhow!("no hello message received"))?;
            log::info!("got server hello");
            let hello_message_type_name = message.payload_type_name().to_string();
            let hello = message
                .into_any()
                .downcast::<TypedEnvelope<proto::Hello>>()
                .map_err(|_| {
                    anyhow!(
                        "invalid hello message received: {:?}",
                        hello_message_type_name
                    )
                })?;
            let peer_id = hello
                .payload
                .peer_id
                .ok_or_else(|| anyhow!("invalid peer id"))?;
            Ok(peer_id)
        };

        let peer_id = match peer_id.await {
            Ok(peer_id) => peer_id,
            Err(error) => {
                self.peer.disconnect(connection_id);
                return Err(error);
            }
        };

        log::info!(
            "set status to connected (connection id: {:?}, peer id: {:?})",
            connection_id,
            peer_id
        );
        self.set_status(
            Status::Connected {
                peer_id,
                connection_id,
            },
            cx,
        );

        cx.spawn({
            let this = self.clone();
            |cx| {
                async move {
                    while let Some(message) = incoming.next().await {
                        this.handle_message(message, &cx);
                        // Don't starve the main thread when receiving lots of messages at once.
                        smol::future::yield_now().await;
                    }
                }
            }
        })
        .detach();

        cx.spawn({
            let this = self.clone();
            move |cx| async move {
                match handle_io.await {
                    Ok(()) => {
                        if *this.status().borrow()
                            == (Status::Connected {
                                connection_id,
                                peer_id,
                            })
                        {
                            this.set_status(Status::SignedOut, &cx);
                        }
                    }
                    Err(err) => {
                        log::error!("connection error: {:?}", err);
                        this.set_status(Status::ConnectionLost, &cx);
                    }
                }
            }
        })
        .detach();

        Ok(())
    }

    fn authenticate(self: &Arc<Self>, cx: &AsyncAppContext) -> Task<Result<Credentials>> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(callback) = self.authenticate.read().as_ref() {
            return callback(cx);
        }

        self.authenticate_with_browser(cx)
    }

    fn establish_connection(
        self: &Arc<Self>,
        credentials: &Credentials,
        cx: &AsyncAppContext,
    ) -> Task<Result<Connection, EstablishConnectionError>> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(callback) = self.establish_connection.read().as_ref() {
            return callback(credentials, cx);
        }

        self.establish_websocket_connection(credentials, cx)
    }

    async fn get_rpc_url(
        http: Arc<HttpClientWithUrl>,
        release_channel: Option<ReleaseChannel>,
    ) -> Result<Url> {
        if let Some(url) = &*ZED_RPC_URL {
            return Url::parse(url).context("invalid rpc url");
        }

        let mut url = http.build_url("/rpc");
        if let Some(preview_param) =
            release_channel.and_then(|channel| channel.release_query_param())
        {
            url += "?";
            url += preview_param;
        }
        let response = http.get(&url, Default::default(), false).await?;
        let collab_url = if response.status().is_redirection() {
            response
                .headers()
                .get("Location")
                .ok_or_else(|| anyhow!("missing location header in /rpc response"))?
                .to_str()
                .map_err(EstablishConnectionError::other)?
                .to_string()
        } else {
            Err(anyhow!(
                "unexpected /rpc response status {}",
                response.status()
            ))?
        };

        Url::parse(&collab_url).context("invalid rpc url")
    }

    fn establish_websocket_connection(
        self: &Arc<Self>,
        credentials: &Credentials,
        cx: &AsyncAppContext,
    ) -> Task<Result<Connection, EstablishConnectionError>> {
        let release_channel = cx
            .update(|cx| ReleaseChannel::try_global(cx))
            .ok()
            .flatten();
        let app_version = cx
            .update(|cx| AppVersion::global(cx).to_string())
            .ok()
            .unwrap_or_default();

        let request = Request::builder()
            .header("Authorization", credentials.authorization_header())
            .header("x-zed-protocol-version", rpc::PROTOCOL_VERSION)
            .header("x-zed-app-version", app_version)
            .header(
                "x-zed-release-channel",
                release_channel.map(|r| r.dev_name()).unwrap_or("unknown"),
            );

        let http = self.http.clone();
        cx.background_executor().spawn(async move {
            let mut rpc_url = Self::get_rpc_url(http, release_channel).await?;
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
                        async_tungstenite::async_std::client_async_tls(request, stream).await?;
                    Ok(Connection::new(
                        stream
                            .map_err(|error| anyhow!(error))
                            .sink_map_err(|error| anyhow!(error)),
                    ))
                }
                "http" => {
                    rpc_url.set_scheme("ws").unwrap();
                    let request = request.uri(rpc_url.as_str()).body(())?;
                    let (stream, _) = async_tungstenite::client_async(request, stream).await?;
                    Ok(Connection::new(
                        stream
                            .map_err(|error| anyhow!(error))
                            .sink_map_err(|error| anyhow!(error)),
                    ))
                }
                _ => Err(anyhow!("invalid rpc url: {}", rpc_url))?,
            }
        })
    }

    pub fn authenticate_with_browser(
        self: &Arc<Self>,
        cx: &AsyncAppContext,
    ) -> Task<Result<Credentials>> {
        let http = self.http.clone();
        cx.spawn(|cx| async move {
            let background = cx.background_executor().clone();

            let (open_url_tx, open_url_rx) = oneshot::channel::<String>();
            cx.update(|cx| {
                cx.spawn(move |cx| async move {
                    let url = open_url_rx.await?;
                    cx.update(|cx| cx.open_url(&url))
                })
                .detach_and_log_err(cx);
            })
            .log_err();

            let credentials = background
                .clone()
                .spawn(async move {
                    // Generate a pair of asymmetric encryption keys. The public key will be used by the
                    // zed server to encrypt the user's access token, so that it can'be intercepted by
                    // any other app running on the user's device.
                    let (public_key, private_key) =
                        rpc::auth::keypair().expect("failed to generate keypair for auth");
                    let public_key_string = String::try_from(public_key)
                        .expect("failed to serialize public key for auth");

                    if let Some((login, token)) =
                        IMPERSONATE_LOGIN.as_ref().zip(ADMIN_API_TOKEN.as_ref())
                    {
                        eprintln!("authenticate as admin {login}, {token}");

                        return Self::authenticate_as_admin(http, login.clone(), token.clone())
                            .await;
                    }

                    // Start an HTTP server to receive the redirect from Zed's sign-in page.
                    let server =
                        tiny_http::Server::http("127.0.0.1:0").expect("failed to find open port");
                    let port = server.server_addr().port();

                    // Open the Zed sign-in page in the user's browser, with query parameters that indicate
                    // that the user is signing in from a Zed app running on the same device.
                    let mut url = http.build_url(&format!(
                        "/native_app_signin?native_app_port={}&native_app_public_key={}",
                        port, public_key_string
                    ));

                    if let Some(impersonate_login) = IMPERSONATE_LOGIN.as_ref() {
                        log::info!("impersonating user @{}", impersonate_login);
                        write!(&mut url, "&impersonate={}", impersonate_login).unwrap();
                    }

                    open_url_tx.send(url).log_err();

                    // Receive the HTTP request from the user's browser. Retrieve the user id and encrypted
                    // access token from the query params.
                    //
                    // TODO - Avoid ever starting more than one HTTP server. Maybe switch to using a
                    // custom URL scheme instead of this local HTTP server.
                    let (user_id, access_token) = background
                        .spawn(async move {
                            for _ in 0..100 {
                                if let Some(req) = server.recv_timeout(Duration::from_secs(1))? {
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
                                        http.build_url("/native_app_signin_succeeded");
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
                                    return Ok((
                                        user_id
                                            .ok_or_else(|| anyhow!("missing user_id parameter"))?,
                                        access_token.ok_or_else(|| {
                                            anyhow!("missing access_token parameter")
                                        })?,
                                    ));
                                }
                            }

                            Err(anyhow!("didn't receive login redirect"))
                        })
                        .await?;

                    let access_token = private_key
                        .decrypt_string(&access_token)
                        .context("failed to decrypt access token")?;

                    Ok(Credentials::User {
                        user_id: user_id.parse()?,
                        access_token,
                    })
                })
                .await?;

            cx.update(|cx| cx.activate(true))?;
            Ok(credentials)
        })
    }

    async fn authenticate_as_admin(
        http: Arc<HttpClientWithUrl>,
        login: String,
        mut api_token: String,
    ) -> Result<Credentials> {
        #[derive(Deserialize)]
        struct AuthenticatedUserResponse {
            user: User,
        }

        #[derive(Deserialize)]
        struct User {
            id: u64,
        }

        // Use the collab server's admin API to retrieve the id
        // of the impersonated user.
        let mut url = Self::get_rpc_url(http.clone(), None).await?;
        url.set_path("/user");
        url.set_query(Some(&format!("github_login={login}")));
        let request = Request::get(url.as_str())
            .header("Authorization", format!("token {api_token}"))
            .body("".into())?;

        let mut response = http.send(request).await?;
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        if !response.status().is_success() {
            Err(anyhow!(
                "admin user request failed {} - {}",
                response.status().as_u16(),
                body,
            ))?;
        }
        let response: AuthenticatedUserResponse = serde_json::from_str(&body)?;

        // Use the admin API token to authenticate as the impersonated user.
        api_token.insert_str(0, "ADMIN_TOKEN:");
        Ok(Credentials::User {
            user_id: response.user.id,
            access_token: api_token,
        })
    }

    pub async fn sign_out(self: &Arc<Self>, cx: &AsyncAppContext) {
        self.state.write().credentials = None;
        self.disconnect(&cx);

        if self.has_credentials(cx).await {
            self.credentials_provider
                .delete_credentials(cx)
                .await
                .log_err();
        }
    }

    pub fn disconnect(self: &Arc<Self>, cx: &AsyncAppContext) {
        self.peer.teardown();
        self.set_status(Status::SignedOut, cx);
    }

    pub fn reconnect(self: &Arc<Self>, cx: &AsyncAppContext) {
        self.peer.teardown();
        self.set_status(Status::ConnectionLost, cx);
    }

    fn connection_id(&self) -> Result<ConnectionId> {
        if let Status::Connected { connection_id, .. } = *self.status().borrow() {
            Ok(connection_id)
        } else {
            Err(anyhow!("not connected"))
        }
    }

    pub fn send<T: EnvelopedMessage>(&self, message: T) -> Result<()> {
        log::debug!("rpc send. client_id:{}, name:{}", self.id(), T::NAME);
        self.peer.send(self.connection_id()?, message)
    }

    pub fn request<T: RequestMessage>(
        &self,
        request: T,
    ) -> impl Future<Output = Result<T::Response>> {
        self.request_envelope(request)
            .map_ok(|envelope| envelope.payload)
    }

    pub fn request_stream<T: RequestMessage>(
        &self,
        request: T,
    ) -> impl Future<Output = Result<impl Stream<Item = Result<T::Response>>>> {
        let client_id = self.id.load(Ordering::SeqCst);
        log::debug!(
            "rpc request start. client_id:{}. name:{}",
            client_id,
            T::NAME
        );
        let response = self
            .connection_id()
            .map(|conn_id| self.peer.request_stream(conn_id, request));
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

    pub fn request_envelope<T: RequestMessage>(
        &self,
        request: T,
    ) -> impl Future<Output = Result<TypedEnvelope<T::Response>>> {
        let client_id = self.id();
        log::debug!(
            "rpc request start. client_id:{}. name:{}",
            client_id,
            T::NAME
        );
        let response = self
            .connection_id()
            .map(|conn_id| self.peer.request_envelope(conn_id, request));
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
        log::debug!("rpc respond. client_id:{}. name:{}", self.id(), T::NAME);
        self.peer.respond(receipt, response)
    }

    fn respond_with_error<T: RequestMessage>(
        &self,
        receipt: Receipt<T>,
        error: proto::Error,
    ) -> Result<()> {
        log::debug!("rpc respond. client_id:{}. name:{}", self.id(), T::NAME);
        self.peer.respond_with_error(receipt, error)
    }

    fn handle_message(
        self: &Arc<Client>,
        message: Box<dyn AnyTypedEnvelope>,
        cx: &AsyncAppContext,
    ) {
        let mut state = self.state.write();
        let type_name = message.payload_type_name();
        let payload_type_id = message.payload_type_id();
        let sender_id = message.original_sender_id();

        let mut subscriber = None;

        if let Some(handle) = state
            .models_by_message_type
            .get(&payload_type_id)
            .and_then(|handle| handle.upgrade())
        {
            subscriber = Some(handle);
        } else if let Some((extract_entity_id, entity_type_id)) =
            state.entity_id_extractors.get(&payload_type_id).zip(
                state
                    .entity_types_by_message_type
                    .get(&payload_type_id)
                    .copied(),
            )
        {
            let entity_id = (extract_entity_id)(message.as_ref());

            match state
                .entities_by_type_and_remote_id
                .get_mut(&(entity_type_id, entity_id))
            {
                Some(WeakSubscriber::Pending(pending)) => {
                    pending.push(message);
                    return;
                }
                Some(weak_subscriber) => match weak_subscriber {
                    WeakSubscriber::Entity { handle } => {
                        subscriber = handle.upgrade();
                    }

                    WeakSubscriber::Pending(_) => {}
                },
                _ => {}
            }
        }

        let subscriber = if let Some(subscriber) = subscriber {
            subscriber
        } else {
            log::info!("unhandled message {}", type_name);
            self.peer.respond_with_unhandled_message(message).log_err();
            return;
        };

        let handler = state.message_handlers.get(&payload_type_id).cloned();
        // Dropping the state prevents deadlocks if the handler interacts with rpc::Client.
        // It also ensures we don't hold the lock while yielding back to the executor, as
        // that might cause the executor thread driving this future to block indefinitely.
        drop(state);

        if let Some(handler) = handler {
            let future = handler(subscriber, message, self, cx.clone());
            let client_id = self.id();
            log::debug!(
                "rpc message received. client_id:{}, sender_id:{:?}, type:{}",
                client_id,
                sender_id,
                type_name
            );
            cx.spawn(move |_| async move {
                    match future.await {
                        Ok(()) => {
                            log::debug!(
                                "rpc message handled. client_id:{}, sender_id:{:?}, type:{}",
                                client_id,
                                sender_id,
                                type_name
                            );
                        }
                        Err(error) => {
                            log::error!(
                                "error handling message. client_id:{}, sender_id:{:?}, type:{}, error:{:?}",
                                client_id,
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
            self.peer.respond_with_unhandled_message(message).log_err();
        }
    }

    pub fn telemetry(&self) -> &Arc<Telemetry> {
        &self.telemetry
    }
}

#[derive(Serialize, Deserialize)]
struct DevelopmentCredentials {
    user_id: u64,
    access_token: String,
}

/// A credentials provider that stores credentials in a local file.
///
/// This MUST only be used in development, as this is not a secure way of storing
/// credentials on user machines.
///
/// Its existence is purely to work around the annoyance of having to constantly
/// re-allow access to the system keychain when developing Zed.
struct DevelopmentCredentialsProvider {
    path: PathBuf,
}

impl CredentialsProvider for DevelopmentCredentialsProvider {
    fn read_credentials<'a>(
        &'a self,
        _cx: &'a AsyncAppContext,
    ) -> Pin<Box<dyn Future<Output = Option<Credentials>> + 'a>> {
        async move {
            if IMPERSONATE_LOGIN.is_some() {
                return None;
            }

            let json = std::fs::read(&self.path).log_err()?;

            let credentials: DevelopmentCredentials = serde_json::from_slice(&json).log_err()?;

            Some(Credentials::User {
                user_id: credentials.user_id,
                access_token: credentials.access_token,
            })
        }
        .boxed_local()
    }

    fn write_credentials<'a>(
        &'a self,
        user_id: u64,
        access_token: String,
        _cx: &'a AsyncAppContext,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
        async move {
            let json = serde_json::to_string(&DevelopmentCredentials {
                user_id,
                access_token,
            })?;

            std::fs::write(&self.path, json)?;

            Ok(())
        }
        .boxed_local()
    }

    fn delete_credentials<'a>(
        &'a self,
        _cx: &'a AsyncAppContext,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
        async move { Ok(std::fs::remove_file(&self.path)?) }.boxed_local()
    }
}

/// A credentials provider that stores credentials in the system keychain.
struct KeychainCredentialsProvider;

impl CredentialsProvider for KeychainCredentialsProvider {
    fn read_credentials<'a>(
        &'a self,
        cx: &'a AsyncAppContext,
    ) -> Pin<Box<dyn Future<Output = Option<Credentials>> + 'a>> {
        async move {
            if IMPERSONATE_LOGIN.is_some() {
                return None;
            }

            let (user_id, access_token) = cx
                .update(|cx| cx.read_credentials(&ClientSettings::get_global(cx).server_url))
                .log_err()?
                .await
                .log_err()??;

            Some(Credentials::User {
                user_id: user_id.parse().ok()?,
                access_token: String::from_utf8(access_token).ok()?,
            })
        }
        .boxed_local()
    }

    fn write_credentials<'a>(
        &'a self,
        user_id: u64,
        access_token: String,
        cx: &'a AsyncAppContext,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
        async move {
            cx.update(move |cx| {
                cx.write_credentials(
                    &ClientSettings::get_global(cx).server_url,
                    &user_id.to_string(),
                    access_token.as_bytes(),
                )
            })?
            .await
        }
        .boxed_local()
    }

    fn delete_credentials<'a>(
        &'a self,
        cx: &'a AsyncAppContext,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
        async move {
            cx.update(move |cx| cx.delete_credentials(&ClientSettings::get_global(cx).server_url))?
                .await
        }
        .boxed_local()
    }
}

/// prefix for the zed:// url scheme
pub static ZED_URL_SCHEME: &str = "zed";

/// Parses the given link into a Zed link.
///
/// Returns a [`Some`] containing the unprefixed link if the link is a Zed link.
/// Returns [`None`] otherwise.
pub fn parse_zed_link<'a>(link: &'a str, cx: &AppContext) -> Option<&'a str> {
    let server_url = &ClientSettings::get_global(cx).server_url;
    if let Some(stripped) = link
        .strip_prefix(server_url)
        .and_then(|result| result.strip_prefix('/'))
    {
        return Some(stripped);
    }
    if let Some(stripped) = link
        .strip_prefix(ZED_URL_SCHEME)
        .and_then(|result| result.strip_prefix("://"))
    {
        return Some(stripped);
    }

    None
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::FakeServer;

    use clock::FakeSystemClock;
    use gpui::{BackgroundExecutor, Context, TestAppContext};
    use http::FakeHttpClient;
    use parking_lot::Mutex;
    use settings::SettingsStore;
    use std::future;

    #[gpui::test(iterations = 10)]
    async fn test_reconnection(cx: &mut TestAppContext) {
        init_test(cx);
        let user_id = 5;
        let client = cx.update(|cx| {
            Client::new(
                Arc::new(FakeSystemClock::default()),
                FakeHttpClient::with_404_response(),
                cx,
            )
        });
        let server = FakeServer::for_client(user_id, &client, cx).await;
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
        cx.executor().advance_clock(Duration::from_secs(10));
        while !matches!(status.next().await, Some(Status::Connected { .. })) {}
        assert_eq!(server.auth_count(), 1); // Client reused the cached credentials when reconnecting

        server.forbid_connections();
        server.disconnect();
        while !matches!(status.next().await, Some(Status::ReconnectionError { .. })) {}

        // Clear cached credentials after authentication fails
        server.roll_access_token();
        server.allow_connections();
        cx.executor().run_until_parked();
        cx.executor().advance_clock(Duration::from_secs(10));
        while !matches!(status.next().await, Some(Status::Connected { .. })) {}
        assert_eq!(server.auth_count(), 2); // Client re-authenticated due to an invalid token
    }

    #[gpui::test(iterations = 10)]
    async fn test_connection_timeout(executor: BackgroundExecutor, cx: &mut TestAppContext) {
        init_test(cx);
        let user_id = 5;
        let client = cx.update(|cx| {
            Client::new(
                Arc::new(FakeSystemClock::default()),
                FakeHttpClient::with_404_response(),
                cx,
            )
        });
        let mut status = client.status();

        // Time out when client tries to connect.
        client.override_authenticate(move |cx| {
            cx.background_executor().spawn(async move {
                Ok(Credentials::User {
                    user_id,
                    access_token: "token".into(),
                })
            })
        });
        client.override_establish_connection(|_, cx| {
            cx.background_executor().spawn(async move {
                future::pending::<()>().await;
                unreachable!()
            })
        });
        let auth_and_connect = cx.spawn({
            let client = client.clone();
            |cx| async move { client.authenticate_and_connect(false, &cx).await }
        });
        executor.run_until_parked();
        assert!(matches!(status.next().await, Some(Status::Connecting)));

        executor.advance_clock(CONNECTION_TIMEOUT);
        assert!(matches!(
            status.next().await,
            Some(Status::ConnectionError { .. })
        ));
        auth_and_connect.await.unwrap_err();

        // Allow the connection to be established.
        let server = FakeServer::for_client(user_id, &client, cx).await;
        assert!(matches!(
            status.next().await,
            Some(Status::Connected { .. })
        ));

        // Disconnect client.
        server.forbid_connections();
        server.disconnect();
        while !matches!(status.next().await, Some(Status::ReconnectionError { .. })) {}

        // Time out when re-establishing the connection.
        server.allow_connections();
        client.override_establish_connection(|_, cx| {
            cx.background_executor().spawn(async move {
                future::pending::<()>().await;
                unreachable!()
            })
        });
        executor.advance_clock(2 * INITIAL_RECONNECTION_DELAY);
        assert!(matches!(
            status.next().await,
            Some(Status::Reconnecting { .. })
        ));

        executor.advance_clock(CONNECTION_TIMEOUT);
        assert!(matches!(
            status.next().await,
            Some(Status::ReconnectionError { .. })
        ));
    }

    #[gpui::test(iterations = 10)]
    async fn test_authenticating_more_than_once(
        cx: &mut TestAppContext,
        executor: BackgroundExecutor,
    ) {
        init_test(cx);
        let auth_count = Arc::new(Mutex::new(0));
        let dropped_auth_count = Arc::new(Mutex::new(0));
        let client = cx.update(|cx| {
            Client::new(
                Arc::new(FakeSystemClock::default()),
                FakeHttpClient::with_404_response(),
                cx,
            )
        });
        client.override_authenticate({
            let auth_count = auth_count.clone();
            let dropped_auth_count = dropped_auth_count.clone();
            move |cx| {
                let auth_count = auth_count.clone();
                let dropped_auth_count = dropped_auth_count.clone();
                cx.background_executor().spawn(async move {
                    *auth_count.lock() += 1;
                    let _drop = util::defer(move || *dropped_auth_count.lock() += 1);
                    future::pending::<()>().await;
                    unreachable!()
                })
            }
        });

        let _authenticate = cx.spawn({
            let client = client.clone();
            move |cx| async move { client.authenticate_and_connect(false, &cx).await }
        });
        executor.run_until_parked();
        assert_eq!(*auth_count.lock(), 1);
        assert_eq!(*dropped_auth_count.lock(), 0);

        let _authenticate = cx.spawn({
            let client = client.clone();
            |cx| async move { client.authenticate_and_connect(false, &cx).await }
        });
        executor.run_until_parked();
        assert_eq!(*auth_count.lock(), 2);
        assert_eq!(*dropped_auth_count.lock(), 1);
    }

    #[gpui::test]
    async fn test_subscribing_to_entity(cx: &mut TestAppContext) {
        init_test(cx);
        let user_id = 5;
        let client = cx.update(|cx| {
            Client::new(
                Arc::new(FakeSystemClock::default()),
                FakeHttpClient::with_404_response(),
                cx,
            )
        });
        let server = FakeServer::for_client(user_id, &client, cx).await;

        let (done_tx1, mut done_rx1) = smol::channel::unbounded();
        let (done_tx2, mut done_rx2) = smol::channel::unbounded();
        client.add_model_message_handler(
            move |model: Model<TestModel>, _: TypedEnvelope<proto::JoinProject>, _, mut cx| {
                match model.update(&mut cx, |model, _| model.id).unwrap() {
                    1 => done_tx1.try_send(()).unwrap(),
                    2 => done_tx2.try_send(()).unwrap(),
                    _ => unreachable!(),
                }
                async { Ok(()) }
            },
        );
        let model1 = cx.new_model(|_| TestModel {
            id: 1,
            subscription: None,
        });
        let model2 = cx.new_model(|_| TestModel {
            id: 2,
            subscription: None,
        });
        let model3 = cx.new_model(|_| TestModel {
            id: 3,
            subscription: None,
        });

        let _subscription1 = client
            .subscribe_to_entity(1)
            .unwrap()
            .set_model(&model1, &mut cx.to_async());
        let _subscription2 = client
            .subscribe_to_entity(2)
            .unwrap()
            .set_model(&model2, &mut cx.to_async());
        // Ensure dropping a subscription for the same entity type still allows receiving of
        // messages for other entity IDs of the same type.
        let subscription3 = client
            .subscribe_to_entity(3)
            .unwrap()
            .set_model(&model3, &mut cx.to_async());
        drop(subscription3);

        server.send(proto::JoinProject { project_id: 1 });
        server.send(proto::JoinProject { project_id: 2 });
        done_rx1.next().await.unwrap();
        done_rx2.next().await.unwrap();
    }

    #[gpui::test]
    async fn test_subscribing_after_dropping_subscription(cx: &mut TestAppContext) {
        init_test(cx);
        let user_id = 5;
        let client = cx.update(|cx| {
            Client::new(
                Arc::new(FakeSystemClock::default()),
                FakeHttpClient::with_404_response(),
                cx,
            )
        });
        let server = FakeServer::for_client(user_id, &client, cx).await;

        let model = cx.new_model(|_| TestModel::default());
        let (done_tx1, _done_rx1) = smol::channel::unbounded();
        let (done_tx2, mut done_rx2) = smol::channel::unbounded();
        let subscription1 = client.add_message_handler(
            model.downgrade(),
            move |_, _: TypedEnvelope<proto::Ping>, _, _| {
                done_tx1.try_send(()).unwrap();
                async { Ok(()) }
            },
        );
        drop(subscription1);
        let _subscription2 = client.add_message_handler(
            model.downgrade(),
            move |_, _: TypedEnvelope<proto::Ping>, _, _| {
                done_tx2.try_send(()).unwrap();
                async { Ok(()) }
            },
        );
        server.send(proto::Ping {});
        done_rx2.next().await.unwrap();
    }

    #[gpui::test]
    async fn test_dropping_subscription_in_handler(cx: &mut TestAppContext) {
        init_test(cx);
        let user_id = 5;
        let client = cx.update(|cx| {
            Client::new(
                Arc::new(FakeSystemClock::default()),
                FakeHttpClient::with_404_response(),
                cx,
            )
        });
        let server = FakeServer::for_client(user_id, &client, cx).await;

        let model = cx.new_model(|_| TestModel::default());
        let (done_tx, mut done_rx) = smol::channel::unbounded();
        let subscription = client.add_message_handler(
            model.clone().downgrade(),
            move |model: Model<TestModel>, _: TypedEnvelope<proto::Ping>, _, mut cx| {
                model
                    .update(&mut cx, |model, _| model.subscription.take())
                    .unwrap();
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
    struct TestModel {
        id: usize,
        subscription: Option<Subscription>,
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            init_settings(cx);
        });
    }
}
