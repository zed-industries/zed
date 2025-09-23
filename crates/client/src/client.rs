#[cfg(any(test, feature = "test-support"))]
pub mod test;

mod proxy;
pub mod telemetry;
pub mod user;
pub mod zed_urls;

use anyhow::{Context as _, Result, anyhow};
use async_tungstenite::tungstenite::{
    client::IntoClientRequest,
    error::Error as WebsocketError,
    http::{HeaderValue, Request, StatusCode},
};
use clock::SystemClock;
use cloud_api_client::CloudApiClient;
use cloud_api_client::websocket_protocol::MessageToClient;
use credentials_provider::CredentialsProvider;
use feature_flags::FeatureFlagAppExt as _;
use futures::{
    AsyncReadExt, FutureExt, SinkExt, Stream, StreamExt, TryFutureExt as _, TryStreamExt,
    channel::oneshot, future::BoxFuture,
};
use gpui::{App, AsyncApp, Entity, Global, Task, WeakEntity, actions};
use http_client::{HttpClient, HttpClientWithUrl, http, read_proxy_from_env};
use parking_lot::RwLock;
use postage::watch;
use proxy::connect_proxy_stream;
use rand::prelude::*;
use release_channel::{AppVersion, ReleaseChannel};
use rpc::proto::{AnyTypedEnvelope, EnvelopedMessage, PeerId, RequestMessage};
use serde::{Deserialize, Serialize};
use settings::{Settings, SettingsContent};
use std::{
    any::TypeId,
    convert::TryFrom,
    fmt::Write as _,
    future::Future,
    marker::PhantomData,
    path::PathBuf,
    sync::{
        Arc, LazyLock, Weak,
        atomic::{AtomicU64, Ordering},
    },
    time::{Duration, Instant},
};
use std::{cmp, pin::Pin};
use telemetry::Telemetry;
use thiserror::Error;
use tokio::net::TcpStream;
use url::Url;
use util::{ConnectionResult, ResultExt};

pub use rpc::*;
pub use telemetry_events::Event;
pub use user::*;

static ZED_SERVER_URL: LazyLock<Option<String>> =
    LazyLock::new(|| std::env::var("ZED_SERVER_URL").ok());
static ZED_RPC_URL: LazyLock<Option<String>> = LazyLock::new(|| std::env::var("ZED_RPC_URL").ok());

pub static IMPERSONATE_LOGIN: LazyLock<Option<String>> = LazyLock::new(|| {
    std::env::var("ZED_IMPERSONATE")
        .ok()
        .and_then(|s| if s.is_empty() { None } else { Some(s) })
});

pub static USE_WEB_LOGIN: LazyLock<bool> = LazyLock::new(|| std::env::var("ZED_WEB_LOGIN").is_ok());

pub static ADMIN_API_TOKEN: LazyLock<Option<String>> = LazyLock::new(|| {
    std::env::var("ZED_ADMIN_API_TOKEN")
        .ok()
        .and_then(|s| if s.is_empty() { None } else { Some(s) })
});

pub static ZED_APP_PATH: LazyLock<Option<PathBuf>> =
    LazyLock::new(|| std::env::var("ZED_APP_PATH").ok().map(PathBuf::from));

pub static ZED_ALWAYS_ACTIVE: LazyLock<bool> =
    LazyLock::new(|| std::env::var("ZED_ALWAYS_ACTIVE").is_ok_and(|e| !e.is_empty()));

pub const INITIAL_RECONNECTION_DELAY: Duration = Duration::from_millis(500);
pub const MAX_RECONNECTION_DELAY: Duration = Duration::from_secs(30);
pub const CONNECTION_TIMEOUT: Duration = Duration::from_secs(20);

actions!(
    client,
    [
        /// Signs in to Zed account.
        SignIn,
        /// Signs out of Zed account.
        SignOut,
        /// Reconnects to the collaboration server.
        Reconnect
    ]
);

#[derive(Deserialize)]
pub struct ClientSettings {
    pub server_url: String,
}

impl Settings for ClientSettings {
    fn from_settings(content: &settings::SettingsContent, _cx: &mut App) -> Self {
        if let Some(server_url) = &*ZED_SERVER_URL {
            return Self {
                server_url: server_url.clone(),
            };
        }
        Self {
            server_url: content.server_url.clone().unwrap(),
        }
    }
}

#[derive(Deserialize, Default)]
pub struct ProxySettings {
    pub proxy: Option<String>,
}

impl ProxySettings {
    pub fn proxy_url(&self) -> Option<Url> {
        self.proxy
            .as_ref()
            .and_then(|input| {
                input
                    .parse::<Url>()
                    .inspect_err(|e| log::error!("Error parsing proxy settings: {}", e))
                    .ok()
            })
            .or_else(read_proxy_from_env)
    }
}

impl Settings for ProxySettings {
    fn from_settings(content: &settings::SettingsContent, _cx: &mut App) -> Self {
        Self {
            proxy: content.proxy.clone(),
        }
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut SettingsContent) {
        vscode.string_setting("http.proxy", &mut current.proxy);
    }
}

pub fn init_settings(cx: &mut App) {
    TelemetrySettings::register(cx);
    ClientSettings::register(cx);
    ProxySettings::register(cx);
}

pub fn init(client: &Arc<Client>, cx: &mut App) {
    let client = Arc::downgrade(client);
    cx.on_action({
        let client = client.clone();
        move |_: &SignIn, cx| {
            if let Some(client) = client.upgrade() {
                cx.spawn(async move |cx| client.sign_in_with_optional_connect(true, cx).await)
                    .detach_and_log_err(cx);
            }
        }
    });

    cx.on_action({
        let client = client.clone();
        move |_: &SignOut, cx| {
            if let Some(client) = client.upgrade() {
                cx.spawn(async move |cx| {
                    client.sign_out(cx).await;
                })
                .detach();
            }
        }
    });

    cx.on_action({
        let client = client;
        move |_: &Reconnect, cx| {
            if let Some(client) = client.upgrade() {
                cx.spawn(async move |cx| {
                    client.reconnect(cx);
                })
                .detach();
            }
        }
    });
}

pub type MessageToClientHandler = Box<dyn Fn(&MessageToClient, &mut App) + Send + Sync + 'static>;

struct GlobalClient(Arc<Client>);

impl Global for GlobalClient {}

pub struct Client {
    id: AtomicU64,
    peer: Arc<Peer>,
    http: Arc<HttpClientWithUrl>,
    cloud_client: Arc<CloudApiClient>,
    telemetry: Arc<Telemetry>,
    credentials_provider: ClientCredentialsProvider,
    state: RwLock<ClientState>,
    handler_set: parking_lot::Mutex<ProtoMessageHandlerSet>,
    message_to_client_handlers: parking_lot::Mutex<Vec<MessageToClientHandler>>,

    #[allow(clippy::type_complexity)]
    #[cfg(any(test, feature = "test-support"))]
    authenticate:
        RwLock<Option<Box<dyn 'static + Send + Sync + Fn(&AsyncApp) -> Task<Result<Credentials>>>>>,

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
                        &AsyncApp,
                    ) -> Task<Result<Connection, EstablishConnectionError>>,
            >,
        >,
    >,

    #[cfg(any(test, feature = "test-support"))]
    rpc_url: RwLock<Option<Url>>,
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
    InvalidHeaderValue(#[from] async_tungstenite::tungstenite::http::header::InvalidHeaderValue),
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
    Authenticated,
    AuthenticationError,
    Connecting,
    ConnectionError,
    Connected {
        peer_id: PeerId,
        connection_id: ConnectionId,
    },
    ConnectionLost,
    Reauthenticating,
    Reauthenticated,
    Reconnecting,
    ReconnectionError {
        next_reconnection: Instant,
    },
}

impl Status {
    pub fn is_connected(&self) -> bool {
        matches!(self, Self::Connected { .. })
    }

    pub fn was_connected(&self) -> bool {
        matches!(
            self,
            Self::ConnectionLost
                | Self::Reauthenticating
                | Self::Reauthenticated
                | Self::Reconnecting
        )
    }

    /// Returns whether the client is currently connected or was connected at some point.
    pub fn is_or_was_connected(&self) -> bool {
        self.is_connected() || self.was_connected()
    }

    pub fn is_signing_in(&self) -> bool {
        matches!(
            self,
            Self::Authenticating | Self::Reauthenticating | Self::Connecting | Self::Reconnecting
        )
    }

    pub fn is_signed_out(&self) -> bool {
        matches!(self, Self::SignedOut | Self::UpgradeRequired)
    }
}

struct ClientState {
    credentials: Option<Credentials>,
    status: (watch::Sender<Status>, watch::Receiver<Status>),
    _reconnect_task: Option<Task<()>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct Credentials {
    pub user_id: u64,
    pub access_token: String,
}

impl Credentials {
    pub fn authorization_header(&self) -> String {
        format!("{} {}", self.user_id, self.access_token)
    }
}

pub struct ClientCredentialsProvider {
    provider: Arc<dyn CredentialsProvider>,
}

impl ClientCredentialsProvider {
    pub fn new(cx: &App) -> Self {
        Self {
            provider: <dyn CredentialsProvider>::global(cx),
        }
    }

    fn server_url(&self, cx: &AsyncApp) -> Result<String> {
        cx.update(|cx| ClientSettings::get_global(cx).server_url.clone())
    }

    /// Reads the credentials from the provider.
    fn read_credentials<'a>(
        &'a self,
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Option<Credentials>> + 'a>> {
        async move {
            if IMPERSONATE_LOGIN.is_some() {
                return None;
            }

            let server_url = self.server_url(cx).ok()?;
            let (user_id, access_token) = self
                .provider
                .read_credentials(&server_url, cx)
                .await
                .log_err()
                .flatten()?;

            Some(Credentials {
                user_id: user_id.parse().ok()?,
                access_token: String::from_utf8(access_token).ok()?,
            })
        }
        .boxed_local()
    }

    /// Writes the credentials to the provider.
    fn write_credentials<'a>(
        &'a self,
        user_id: u64,
        access_token: String,
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
        async move {
            let server_url = self.server_url(cx)?;
            self.provider
                .write_credentials(
                    &server_url,
                    &user_id.to_string(),
                    access_token.as_bytes(),
                    cx,
                )
                .await
        }
        .boxed_local()
    }

    /// Deletes the credentials from the provider.
    fn delete_credentials<'a>(
        &'a self,
        cx: &'a AsyncApp,
    ) -> Pin<Box<dyn Future<Output = Result<()>> + 'a>> {
        async move {
            let server_url = self.server_url(cx)?;
            self.provider.delete_credentials(&server_url, cx).await
        }
        .boxed_local()
    }
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            credentials: None,
            status: watch::channel_with(Status::SignedOut),
            _reconnect_task: None,
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
                    let mut state = client.handler_set.lock();
                    let _ = state.entities_by_type_and_remote_id.remove(id);
                }
            }
            Subscription::Message { client, id } => {
                if let Some(client) = client.upgrade() {
                    let mut state = client.handler_set.lock();
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
    pub fn set_entity(mut self, entity: &Entity<T>, cx: &AsyncApp) -> Subscription {
        self.consumed = true;
        let mut handlers = self.client.handler_set.lock();
        let id = (TypeId::of::<T>(), self.remote_id);
        let Some(EntityMessageSubscriber::Pending(messages)) =
            handlers.entities_by_type_and_remote_id.remove(&id)
        else {
            unreachable!()
        };

        handlers.entities_by_type_and_remote_id.insert(
            id,
            EntityMessageSubscriber::Entity {
                handle: entity.downgrade().into(),
            },
        );
        drop(handlers);
        for message in messages {
            let client_id = self.client.id();
            let type_name = message.payload_type_name();
            let sender_id = message.original_sender_id();
            log::debug!(
                "handling queued rpc message. client_id:{}, sender_id:{:?}, type:{}",
                client_id,
                sender_id,
                type_name
            );
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
            let mut state = self.client.handler_set.lock();
            if let Some(EntityMessageSubscriber::Pending(messages)) = state
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

#[derive(Copy, Clone, Deserialize, Debug)]
pub struct TelemetrySettings {
    pub diagnostics: bool,
    pub metrics: bool,
}

impl settings::Settings for TelemetrySettings {
    fn from_settings(content: &SettingsContent, _cx: &mut App) -> Self {
        Self {
            diagnostics: content.telemetry.as_ref().unwrap().diagnostics.unwrap(),
            metrics: content.telemetry.as_ref().unwrap().metrics.unwrap(),
        }
    }

    fn import_from_vscode(vscode: &settings::VsCodeSettings, current: &mut SettingsContent) {
        let mut telemetry = settings::TelemetrySettingsContent::default();
        vscode.enum_setting("telemetry.telemetryLevel", &mut telemetry.metrics, |s| {
            Some(s == "all")
        });
        vscode.enum_setting(
            "telemetry.telemetryLevel",
            &mut telemetry.diagnostics,
            |s| Some(matches!(s, "all" | "error" | "crash")),
        );
        // we could translate telemetry.telemetryLevel, but just because users didn't want
        // to send microsoft telemetry doesn't mean they don't want to send it to zed. their
        // all/error/crash/off correspond to combinations of our "diagnostics" and "metrics".
        if let Some(diagnostics) = telemetry.diagnostics {
            current.telemetry.get_or_insert_default().diagnostics = Some(diagnostics)
        }
        if let Some(metrics) = telemetry.metrics {
            current.telemetry.get_or_insert_default().metrics = Some(metrics)
        }
    }
}

impl Client {
    pub fn new(
        clock: Arc<dyn SystemClock>,
        http: Arc<HttpClientWithUrl>,
        cx: &mut App,
    ) -> Arc<Self> {
        Arc::new(Self {
            id: AtomicU64::new(0),
            peer: Peer::new(0),
            telemetry: Telemetry::new(clock, http.clone(), cx),
            cloud_client: Arc::new(CloudApiClient::new(http.clone())),
            http,
            credentials_provider: ClientCredentialsProvider::new(cx),
            state: Default::default(),
            handler_set: Default::default(),
            message_to_client_handlers: parking_lot::Mutex::new(Vec::new()),

            #[cfg(any(test, feature = "test-support"))]
            authenticate: Default::default(),
            #[cfg(any(test, feature = "test-support"))]
            establish_connection: Default::default(),
            #[cfg(any(test, feature = "test-support"))]
            rpc_url: RwLock::default(),
        })
    }

    pub fn production(cx: &mut App) -> Arc<Self> {
        let clock = Arc::new(clock::RealSystemClock);
        let http = Arc::new(HttpClientWithUrl::new_url(
            cx.http_client(),
            &ClientSettings::get_global(cx).server_url,
            cx.http_client().proxy().cloned(),
        ));
        Self::new(clock, http, cx)
    }

    pub fn id(&self) -> u64 {
        self.id.load(Ordering::SeqCst)
    }

    pub fn http_client(&self) -> Arc<HttpClientWithUrl> {
        self.http.clone()
    }

    pub fn cloud_client(&self) -> Arc<CloudApiClient> {
        self.cloud_client.clone()
    }

    pub fn set_id(&self, id: u64) -> &Self {
        self.id.store(id, Ordering::SeqCst);
        self
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn teardown(&self) {
        let mut state = self.state.write();
        state._reconnect_task.take();
        self.handler_set.lock().clear();
        self.peer.teardown();
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn override_authenticate<F>(&self, authenticate: F) -> &Self
    where
        F: 'static + Send + Sync + Fn(&AsyncApp) -> Task<Result<Credentials>>,
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
            + Fn(&Credentials, &AsyncApp) -> Task<Result<Connection, EstablishConnectionError>>,
    {
        *self.establish_connection.write() = Some(Box::new(connect));
        self
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn override_rpc_url(&self, url: Url) -> &Self {
        *self.rpc_url.write() = Some(url);
        self
    }

    pub fn global(cx: &App) -> Arc<Self> {
        cx.global::<GlobalClient>().0.clone()
    }
    pub fn set_global(client: Arc<Client>, cx: &mut App) {
        cx.set_global(GlobalClient(client))
    }

    pub fn user_id(&self) -> Option<u64> {
        self.state
            .read()
            .credentials
            .as_ref()
            .map(|credentials| credentials.user_id)
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

    fn set_status(self: &Arc<Self>, status: Status, cx: &AsyncApp) {
        log::info!("set status on client {}: {:?}", self.id(), status);
        let mut state = self.state.write();
        *state.status.0.borrow_mut() = status;

        match status {
            Status::Connected { .. } => {
                state._reconnect_task = None;
            }
            Status::ConnectionLost => {
                let client = self.clone();
                state._reconnect_task = Some(cx.spawn(async move |cx| {
                    #[cfg(any(test, feature = "test-support"))]
                    let mut rng = StdRng::seed_from_u64(0);
                    #[cfg(not(any(test, feature = "test-support")))]
                    let mut rng = StdRng::from_os_rng();

                    let mut delay = INITIAL_RECONNECTION_DELAY;
                    loop {
                        match client.connect(true, cx).await {
                            ConnectionResult::Timeout => {
                                log::error!("client connect attempt timed out")
                            }
                            ConnectionResult::ConnectionReset => {
                                log::error!("client connect attempt reset")
                            }
                            ConnectionResult::Result(r) => {
                                if let Err(error) = r {
                                    log::error!("failed to connect: {error}");
                                } else {
                                    break;
                                }
                            }
                        }

                        if matches!(
                            *client.status().borrow(),
                            Status::AuthenticationError | Status::ConnectionError
                        ) {
                            client.set_status(
                                Status::ReconnectionError {
                                    next_reconnection: Instant::now() + delay,
                                },
                                cx,
                            );
                            let jitter = Duration::from_millis(
                                rng.random_range(0..delay.as_millis() as u64),
                            );
                            cx.background_executor().timer(delay + jitter).await;
                            delay = cmp::min(delay * 2, MAX_RECONNECTION_DELAY);
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

        let mut state = self.handler_set.lock();
        anyhow::ensure!(
            !state.entities_by_type_and_remote_id.contains_key(&id),
            "already subscribed to entity"
        );

        state
            .entities_by_type_and_remote_id
            .insert(id, EntityMessageSubscriber::Pending(Default::default()));

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
        entity: WeakEntity<E>,
        handler: H,
    ) -> Subscription
    where
        M: EnvelopedMessage,
        E: 'static,
        H: 'static + Sync + Fn(Entity<E>, TypedEnvelope<M>, AsyncApp) -> F + Send + Sync,
        F: 'static + Future<Output = Result<()>>,
    {
        self.add_message_handler_impl(entity, move |entity, message, _, cx| {
            handler(entity, message, cx)
        })
    }

    fn add_message_handler_impl<M, E, H, F>(
        self: &Arc<Self>,
        entity: WeakEntity<E>,
        handler: H,
    ) -> Subscription
    where
        M: EnvelopedMessage,
        E: 'static,
        H: 'static
            + Sync
            + Fn(Entity<E>, TypedEnvelope<M>, AnyProtoClient, AsyncApp) -> F
            + Send
            + Sync,
        F: 'static + Future<Output = Result<()>>,
    {
        let message_type_id = TypeId::of::<M>();
        let mut state = self.handler_set.lock();
        state
            .entities_by_message_type
            .insert(message_type_id, entity.into());

        let prev_handler = state.message_handlers.insert(
            message_type_id,
            Arc::new(move |subscriber, envelope, client, cx| {
                let subscriber = subscriber.downcast::<E>().unwrap();
                let envelope = envelope.into_any().downcast::<TypedEnvelope<M>>().unwrap();
                handler(subscriber, *envelope, client, cx).boxed_local()
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
        entity: WeakEntity<E>,
        handler: H,
    ) -> Subscription
    where
        M: RequestMessage,
        E: 'static,
        H: 'static + Sync + Fn(Entity<E>, TypedEnvelope<M>, AsyncApp) -> F + Send + Sync,
        F: 'static + Future<Output = Result<M::Response>>,
    {
        self.add_message_handler_impl(entity, move |handle, envelope, this, cx| {
            Self::respond_to_request(envelope.receipt(), handler(handle, envelope, cx), this)
        })
    }

    async fn respond_to_request<T: RequestMessage, F: Future<Output = Result<T::Response>>>(
        receipt: Receipt<T>,
        response: F,
        client: AnyProtoClient,
    ) -> Result<()> {
        match response.await {
            Ok(response) => {
                client.send_response(receipt.message_id, response)?;
                Ok(())
            }
            Err(error) => {
                client.send_response(receipt.message_id, error.to_proto())?;
                Err(error)
            }
        }
    }

    pub async fn has_credentials(&self, cx: &AsyncApp) -> bool {
        self.credentials_provider
            .read_credentials(cx)
            .await
            .is_some()
    }

    pub async fn sign_in(
        self: &Arc<Self>,
        try_provider: bool,
        cx: &AsyncApp,
    ) -> Result<Credentials> {
        let is_reauthenticating = if self.status().borrow().is_signed_out() {
            self.set_status(Status::Authenticating, cx);
            false
        } else {
            self.set_status(Status::Reauthenticating, cx);
            true
        };

        let mut credentials = None;

        let old_credentials = self.state.read().credentials.clone();
        if let Some(old_credentials) = old_credentials
            && self.validate_credentials(&old_credentials, cx).await?
        {
            credentials = Some(old_credentials);
        }

        if credentials.is_none()
            && try_provider
            && let Some(stored_credentials) = self.credentials_provider.read_credentials(cx).await
        {
            if self.validate_credentials(&stored_credentials, cx).await? {
                credentials = Some(stored_credentials);
            } else {
                self.credentials_provider
                    .delete_credentials(cx)
                    .await
                    .log_err();
            }
        }

        if credentials.is_none() {
            let mut status_rx = self.status();
            let _ = status_rx.next().await;
            futures::select_biased! {
                authenticate = self.authenticate(cx).fuse() => {
                    match authenticate {
                        Ok(creds) => {
                            if IMPERSONATE_LOGIN.is_none() {
                                self.credentials_provider
                                    .write_credentials(creds.user_id, creds.access_token.clone(), cx)
                                    .await
                                    .log_err();
                            }

                            credentials = Some(creds);
                        },
                        Err(err) => {
                            self.set_status(Status::AuthenticationError, cx);
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
        self.set_id(credentials.user_id);
        self.cloud_client
            .set_credentials(credentials.user_id as u32, credentials.access_token.clone());
        self.state.write().credentials = Some(credentials.clone());
        self.set_status(
            if is_reauthenticating {
                Status::Reauthenticated
            } else {
                Status::Authenticated
            },
            cx,
        );

        Ok(credentials)
    }

    async fn validate_credentials(
        self: &Arc<Self>,
        credentials: &Credentials,
        cx: &AsyncApp,
    ) -> Result<bool> {
        match self
            .cloud_client
            .validate_credentials(credentials.user_id as u32, &credentials.access_token)
            .await
        {
            Ok(valid) => Ok(valid),
            Err(err) => {
                self.set_status(Status::AuthenticationError, cx);
                Err(anyhow!("failed to validate credentials: {}", err))
            }
        }
    }

    /// Establishes a WebSocket connection with Cloud for receiving updates from the server.
    async fn connect_to_cloud(self: &Arc<Self>, cx: &AsyncApp) -> Result<()> {
        let connect_task = cx.update({
            let cloud_client = self.cloud_client.clone();
            move |cx| cloud_client.connect(cx)
        })??;
        let connection = connect_task.await?;

        let (mut messages, task) = cx.update(|cx| connection.spawn(cx))?;
        task.detach();

        cx.spawn({
            let this = self.clone();
            async move |cx| {
                while let Some(message) = messages.next().await {
                    if let Some(message) = message.log_err() {
                        this.handle_message_to_client(message, cx);
                    }
                }
            }
        })
        .detach();

        Ok(())
    }

    /// Performs a sign-in and also (optionally) connects to Collab.
    ///
    /// Only Zed staff automatically connect to Collab.
    pub async fn sign_in_with_optional_connect(
        self: &Arc<Self>,
        try_provider: bool,
        cx: &AsyncApp,
    ) -> Result<()> {
        // Don't try to sign in again if we're already connected to Collab, as it will temporarily disconnect us.
        if self.status().borrow().is_connected() {
            return Ok(());
        }

        let (is_staff_tx, is_staff_rx) = oneshot::channel::<bool>();
        let mut is_staff_tx = Some(is_staff_tx);
        cx.update(|cx| {
            cx.on_flags_ready(move |state, _cx| {
                if let Some(is_staff_tx) = is_staff_tx.take() {
                    is_staff_tx.send(state.is_staff).log_err();
                }
            })
            .detach();
        })
        .log_err();

        let credentials = self.sign_in(try_provider, cx).await?;

        self.connect_to_cloud(cx).await.log_err();

        cx.update(move |cx| {
            cx.spawn({
                let client = self.clone();
                async move |cx| {
                    let is_staff = is_staff_rx.await?;
                    if is_staff {
                        match client.connect_with_credentials(credentials, cx).await {
                            ConnectionResult::Timeout => Err(anyhow!("connection timed out")),
                            ConnectionResult::ConnectionReset => Err(anyhow!("connection reset")),
                            ConnectionResult::Result(result) => {
                                result.context("client auth and connect")
                            }
                        }
                    } else {
                        Ok(())
                    }
                }
            })
            .detach_and_log_err(cx);
        })
        .log_err();

        Ok(())
    }

    pub async fn connect(
        self: &Arc<Self>,
        try_provider: bool,
        cx: &AsyncApp,
    ) -> ConnectionResult<()> {
        let was_disconnected = match *self.status().borrow() {
            Status::SignedOut | Status::Authenticated => true,
            Status::ConnectionError
            | Status::ConnectionLost
            | Status::Authenticating
            | Status::AuthenticationError
            | Status::Reauthenticating
            | Status::Reauthenticated
            | Status::ReconnectionError { .. } => false,
            Status::Connected { .. } | Status::Connecting | Status::Reconnecting => {
                return ConnectionResult::Result(Ok(()));
            }
            Status::UpgradeRequired => {
                return ConnectionResult::Result(
                    Err(EstablishConnectionError::UpgradeRequired)
                        .context("client auth and connect"),
                );
            }
        };
        let credentials = match self.sign_in(try_provider, cx).await {
            Ok(credentials) => credentials,
            Err(err) => return ConnectionResult::Result(Err(err)),
        };

        if was_disconnected {
            self.set_status(Status::Connecting, cx);
        } else {
            self.set_status(Status::Reconnecting, cx);
        }

        self.connect_with_credentials(credentials, cx).await
    }

    async fn connect_with_credentials(
        self: &Arc<Self>,
        credentials: Credentials,
        cx: &AsyncApp,
    ) -> ConnectionResult<()> {
        let mut timeout =
            futures::FutureExt::fuse(cx.background_executor().timer(CONNECTION_TIMEOUT));
        futures::select_biased! {
            connection = self.establish_connection(&credentials, cx).fuse() => {
                match connection {
                    Ok(conn) => {
                        futures::select_biased! {
                            result = self.set_connection(conn, cx).fuse() => {
                                match result.context("client auth and connect") {
                                    Ok(()) => ConnectionResult::Result(Ok(())),
                                    Err(err) => {
                                        self.set_status(Status::ConnectionError, cx);
                                        ConnectionResult::Result(Err(err))
                                    },
                                }
                            },
                            _ = timeout => {
                                self.set_status(Status::ConnectionError, cx);
                                ConnectionResult::Timeout
                            }
                        }
                    }
                    Err(EstablishConnectionError::Unauthorized) => {
                        self.set_status(Status::ConnectionError, cx);
                        ConnectionResult::Result(Err(EstablishConnectionError::Unauthorized).context("client auth and connect"))
                    }
                    Err(EstablishConnectionError::UpgradeRequired) => {
                        self.set_status(Status::UpgradeRequired, cx);
                        ConnectionResult::Result(Err(EstablishConnectionError::UpgradeRequired).context("client auth and connect"))
                    }
                    Err(error) => {
                        self.set_status(Status::ConnectionError, cx);
                        ConnectionResult::Result(Err(error).context("client auth and connect"))
                    }
                }
            }
            _ = &mut timeout => {
                self.set_status(Status::ConnectionError, cx);
                ConnectionResult::Timeout
            }
        }
    }

    async fn set_connection(self: &Arc<Self>, conn: Connection, cx: &AsyncApp) -> Result<()> {
        let executor = cx.background_executor();
        log::debug!("add connection to peer");
        let (connection_id, handle_io, mut incoming) = self.peer.add_connection(conn, {
            let executor = executor.clone();
            move |duration| executor.timer(duration)
        });
        let handle_io = executor.spawn(handle_io);

        let peer_id = async {
            log::debug!("waiting for server hello");
            let message = incoming.next().await.context("no hello message received")?;
            log::debug!("got server hello");
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
            let peer_id = hello.payload.peer_id.context("invalid peer id")?;
            Ok(peer_id)
        };

        let peer_id = match peer_id.await {
            Ok(peer_id) => peer_id,
            Err(error) => {
                self.peer.disconnect(connection_id);
                return Err(error);
            }
        };

        log::debug!(
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
            async move |cx| {
                while let Some(message) = incoming.next().await {
                    this.handle_message(message, cx);
                    // Don't starve the main thread when receiving lots of messages at once.
                    smol::future::yield_now().await;
                }
            }
        })
        .detach();

        cx.spawn({
            let this = self.clone();
            async move |cx| match handle_io.await {
                Ok(()) => {
                    if *this.status().borrow()
                        == (Status::Connected {
                            connection_id,
                            peer_id,
                        })
                    {
                        this.set_status(Status::SignedOut, cx);
                    }
                }
                Err(err) => {
                    log::error!("connection error: {:?}", err);
                    this.set_status(Status::ConnectionLost, cx);
                }
            }
        })
        .detach();

        Ok(())
    }

    fn authenticate(self: &Arc<Self>, cx: &AsyncApp) -> Task<Result<Credentials>> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(callback) = self.authenticate.read().as_ref() {
            return callback(cx);
        }

        self.authenticate_with_browser(cx)
    }

    fn establish_connection(
        self: &Arc<Self>,
        credentials: &Credentials,
        cx: &AsyncApp,
    ) -> Task<Result<Connection, EstablishConnectionError>> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(callback) = self.establish_connection.read().as_ref() {
            return callback(credentials, cx);
        }

        self.establish_websocket_connection(credentials, cx)
    }

    fn rpc_url(
        &self,
        http: Arc<HttpClientWithUrl>,
        release_channel: Option<ReleaseChannel>,
    ) -> impl Future<Output = Result<url::Url>> + use<> {
        #[cfg(any(test, feature = "test-support"))]
        let url_override = self.rpc_url.read().clone();

        async move {
            #[cfg(any(test, feature = "test-support"))]
            if let Some(url) = url_override {
                return Ok(url);
            }

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
            anyhow::ensure!(
                response.status().is_redirection(),
                "unexpected /rpc response status {}",
                response.status()
            );
            let collab_url = response
                .headers()
                .get("Location")
                .context("missing location header in /rpc response")?
                .to_str()
                .map_err(EstablishConnectionError::other)?
                .to_string();
            Url::parse(&collab_url).with_context(|| format!("parsing collab rpc url {collab_url}"))
        }
    }

    fn establish_websocket_connection(
        self: &Arc<Self>,
        credentials: &Credentials,
        cx: &AsyncApp,
    ) -> Task<Result<Connection, EstablishConnectionError>> {
        let release_channel = cx
            .update(|cx| ReleaseChannel::try_global(cx))
            .ok()
            .flatten();
        let app_version = cx
            .update(|cx| AppVersion::global(cx).to_string())
            .ok()
            .unwrap_or_default();

        let http = self.http.clone();
        let proxy = http.proxy().cloned();
        let user_agent = http.user_agent().cloned();
        let credentials = credentials.clone();
        let rpc_url = self.rpc_url(http, release_channel);
        let system_id = self.telemetry.system_id();
        let metrics_id = self.telemetry.metrics_id();
        cx.spawn(async move |cx| {
            use HttpOrHttps::*;

            #[derive(Debug)]
            enum HttpOrHttps {
                Http,
                Https,
            }

            let mut rpc_url = rpc_url.await?;
            let url_scheme = match rpc_url.scheme() {
                "https" => Https,
                "http" => Http,
                _ => Err(anyhow!("invalid rpc url: {}", rpc_url))?,
            };

            let stream = gpui_tokio::Tokio::spawn_result(cx, {
                let rpc_url = rpc_url.clone();
                async move {
                    let rpc_host = rpc_url
                        .host_str()
                        .zip(rpc_url.port_or_known_default())
                        .context("missing host in rpc url")?;
                    Ok(match proxy {
                        Some(proxy) => connect_proxy_stream(&proxy, rpc_host).await?,
                        None => Box::new(TcpStream::connect(rpc_host).await?),
                    })
                }
            })?
            .await?;

            log::info!("connected to rpc endpoint {}", rpc_url);

            rpc_url
                .set_scheme(match url_scheme {
                    Https => "wss",
                    Http => "ws",
                })
                .unwrap();

            // We call `into_client_request` to let `tungstenite` construct the WebSocket request
            // for us from the RPC URL.
            //
            // Among other things, it will generate and set a `Sec-WebSocket-Key` header for us.
            let mut request = IntoClientRequest::into_client_request(rpc_url.as_str())?;

            // We then modify the request to add our desired headers.
            let request_headers = request.headers_mut();
            request_headers.insert(
                http::header::AUTHORIZATION,
                HeaderValue::from_str(&credentials.authorization_header())?,
            );
            request_headers.insert(
                "x-zed-protocol-version",
                HeaderValue::from_str(&rpc::PROTOCOL_VERSION.to_string())?,
            );
            request_headers.insert("x-zed-app-version", HeaderValue::from_str(&app_version)?);
            request_headers.insert(
                "x-zed-release-channel",
                HeaderValue::from_str(release_channel.map(|r| r.dev_name()).unwrap_or("unknown"))?,
            );
            if let Some(user_agent) = user_agent {
                request_headers.insert(http::header::USER_AGENT, user_agent);
            }
            if let Some(system_id) = system_id {
                request_headers.insert("x-zed-system-id", HeaderValue::from_str(&system_id)?);
            }
            if let Some(metrics_id) = metrics_id {
                request_headers.insert("x-zed-metrics-id", HeaderValue::from_str(&metrics_id)?);
            }

            let (stream, _) = async_tungstenite::tokio::client_async_tls_with_connector_and_config(
                request,
                stream,
                Some(Arc::new(http_client_tls::tls_config()).into()),
                None,
            )
            .await?;

            Ok(Connection::new(
                stream
                    .map_err(|error| anyhow!(error))
                    .sink_map_err(|error| anyhow!(error)),
            ))
        })
    }

    pub fn authenticate_with_browser(self: &Arc<Self>, cx: &AsyncApp) -> Task<Result<Credentials>> {
        let http = self.http.clone();
        let this = self.clone();
        cx.spawn(async move |cx| {
            let background = cx.background_executor().clone();

            let (open_url_tx, open_url_rx) = oneshot::channel::<String>();
            cx.update(|cx| {
                cx.spawn(async move |cx| {
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
                        if !*USE_WEB_LOGIN {
                            eprintln!("authenticate as admin {login}, {token}");

                            return this
                                .authenticate_as_admin(http, login.clone(), token.clone())
                                .await;
                        }
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

                    #[derive(Deserialize)]
                    struct CallbackParams {
                        pub user_id: String,
                        pub access_token: String,
                    }

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
                                    let url = Url::parse(&format!("http://example.com{}", path))
                                        .context("failed to parse login notification url")?;
                                    let callback_params: CallbackParams =
                                        serde_urlencoded::from_str(url.query().unwrap_or_default())
                                            .context(
                                                "failed to parse sign-in callback query parameters",
                                            )?;

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
                                        callback_params.user_id,
                                        callback_params.access_token,
                                    ));
                                }
                            }

                            anyhow::bail!("didn't receive login redirect");
                        })
                        .await?;

                    let access_token = private_key
                        .decrypt_string(&access_token)
                        .context("failed to decrypt access token")?;

                    Ok(Credentials {
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
        self: &Arc<Self>,
        http: Arc<HttpClientWithUrl>,
        login: String,
        api_token: String,
    ) -> Result<Credentials> {
        #[derive(Serialize)]
        struct ImpersonateUserBody {
            github_login: String,
        }

        #[derive(Deserialize)]
        struct ImpersonateUserResponse {
            user_id: u64,
            access_token: String,
        }

        let url = self
            .http
            .build_zed_cloud_url("/internal/users/impersonate", &[])?;
        let request = Request::post(url.as_str())
            .header("Content-Type", "application/json")
            .header("Authorization", format!("Bearer {api_token}"))
            .body(
                serde_json::to_string(&ImpersonateUserBody {
                    github_login: login,
                })?
                .into(),
            )?;

        let mut response = http.send(request).await?;
        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        anyhow::ensure!(
            response.status().is_success(),
            "admin user request failed {} - {}",
            response.status().as_u16(),
            body,
        );
        let response: ImpersonateUserResponse = serde_json::from_str(&body)?;

        Ok(Credentials {
            user_id: response.user_id,
            access_token: response.access_token,
        })
    }

    pub async fn sign_out(self: &Arc<Self>, cx: &AsyncApp) {
        self.state.write().credentials = None;
        self.cloud_client.clear_credentials();
        self.disconnect(cx);

        if self.has_credentials(cx).await {
            self.credentials_provider
                .delete_credentials(cx)
                .await
                .log_err();
        }
    }

    pub fn disconnect(self: &Arc<Self>, cx: &AsyncApp) {
        self.peer.teardown();
        self.set_status(Status::SignedOut, cx);
    }

    pub fn reconnect(self: &Arc<Self>, cx: &AsyncApp) {
        self.peer.teardown();
        self.set_status(Status::ConnectionLost, cx);
    }

    fn connection_id(&self) -> Result<ConnectionId> {
        if let Status::Connected { connection_id, .. } = *self.status().borrow() {
            Ok(connection_id)
        } else {
            anyhow::bail!("not connected");
        }
    }

    pub fn send<T: EnvelopedMessage>(&self, message: T) -> Result<()> {
        log::debug!("rpc send. client_id:{}, name:{}", self.id(), T::NAME);
        self.peer.send(self.connection_id()?, message)
    }

    pub fn request<T: RequestMessage>(
        &self,
        request: T,
    ) -> impl Future<Output = Result<T::Response>> + use<T> {
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
    ) -> impl Future<Output = Result<TypedEnvelope<T::Response>>> + use<T> {
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

    pub fn request_dynamic(
        &self,
        envelope: proto::Envelope,
        request_type: &'static str,
    ) -> impl Future<Output = Result<proto::Envelope>> + use<> {
        let client_id = self.id();
        log::debug!(
            "rpc request start. client_id:{}. name:{}",
            client_id,
            request_type
        );
        let response = self
            .connection_id()
            .map(|conn_id| self.peer.request_dynamic(conn_id, envelope, request_type));
        async move {
            let response = response?.await;
            log::debug!(
                "rpc request finish. client_id:{}. name:{}",
                client_id,
                request_type
            );
            Ok(response?.0)
        }
    }

    fn handle_message(self: &Arc<Client>, message: Box<dyn AnyTypedEnvelope>, cx: &AsyncApp) {
        let sender_id = message.sender_id();
        let request_id = message.message_id();
        let type_name = message.payload_type_name();
        let original_sender_id = message.original_sender_id();

        if let Some(future) = ProtoMessageHandlerSet::handle_message(
            &self.handler_set,
            message,
            self.clone().into(),
            cx.clone(),
        ) {
            let client_id = self.id();
            log::debug!(
                "rpc message received. client_id:{}, sender_id:{:?}, type:{}",
                client_id,
                original_sender_id,
                type_name
            );
            cx.spawn(async move |_| match future.await {
                Ok(()) => {
                    log::debug!("rpc message handled. client_id:{client_id}, sender_id:{original_sender_id:?}, type:{type_name}");
                }
                Err(error) => {
                    log::error!("error handling message. client_id:{client_id}, sender_id:{original_sender_id:?}, type:{type_name}, error:{error:#}");
                }
            })
            .detach();
        } else {
            log::info!("unhandled message {}", type_name);
            self.peer
                .respond_with_unhandled_message(sender_id.into(), request_id, type_name)
                .log_err();
        }
    }

    pub fn add_message_to_client_handler(
        self: &Arc<Client>,
        handler: impl Fn(&MessageToClient, &mut App) + Send + Sync + 'static,
    ) {
        self.message_to_client_handlers
            .lock()
            .push(Box::new(handler));
    }

    fn handle_message_to_client(self: &Arc<Client>, message: MessageToClient, cx: &AsyncApp) {
        cx.update(|cx| {
            for handler in self.message_to_client_handlers.lock().iter() {
                handler(&message, cx);
            }
        })
        .ok();
    }

    pub fn telemetry(&self) -> &Arc<Telemetry> {
        &self.telemetry
    }
}

impl ProtoClient for Client {
    fn request(
        &self,
        envelope: proto::Envelope,
        request_type: &'static str,
    ) -> BoxFuture<'static, Result<proto::Envelope>> {
        self.request_dynamic(envelope, request_type).boxed()
    }

    fn send(&self, envelope: proto::Envelope, message_type: &'static str) -> Result<()> {
        log::debug!("rpc send. client_id:{}, name:{}", self.id(), message_type);
        let connection_id = self.connection_id()?;
        self.peer.send_dynamic(connection_id, envelope)
    }

    fn send_response(&self, envelope: proto::Envelope, message_type: &'static str) -> Result<()> {
        log::debug!(
            "rpc respond. client_id:{}, name:{}",
            self.id(),
            message_type
        );
        let connection_id = self.connection_id()?;
        self.peer.send_dynamic(connection_id, envelope)
    }

    fn message_handler_set(&self) -> &parking_lot::Mutex<ProtoMessageHandlerSet> {
        &self.handler_set
    }

    fn is_via_collab(&self) -> bool {
        true
    }
}

/// prefix for the zed:// url scheme
pub const ZED_URL_SCHEME: &str = "zed";

/// Parses the given link into a Zed link.
///
/// Returns a [`Some`] containing the unprefixed link if the link is a Zed link.
/// Returns [`None`] otherwise.
pub fn parse_zed_link<'a>(link: &'a str, cx: &App) -> Option<&'a str> {
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
    use crate::test::{FakeServer, parse_authorization_header};

    use clock::FakeSystemClock;
    use gpui::{AppContext as _, BackgroundExecutor, TestAppContext};
    use http_client::FakeHttpClient;
    use parking_lot::Mutex;
    use proto::TypedEnvelope;
    use settings::SettingsStore;
    use std::future;

    #[gpui::test(iterations = 10)]
    async fn test_reconnection(cx: &mut TestAppContext) {
        init_test(cx);
        let user_id = 5;
        let client = cx.update(|cx| {
            Client::new(
                Arc::new(FakeSystemClock::new()),
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
    async fn test_auth_failure_during_reconnection(cx: &mut TestAppContext) {
        init_test(cx);
        let http_client = FakeHttpClient::with_200_response();
        let client =
            cx.update(|cx| Client::new(Arc::new(FakeSystemClock::new()), http_client.clone(), cx));
        let server = FakeServer::for_client(42, &client, cx).await;
        let mut status = client.status();
        assert!(matches!(
            status.next().await,
            Some(Status::Connected { .. })
        ));
        assert_eq!(server.auth_count(), 1);

        // Simulate an auth failure during reconnection.
        http_client
            .as_fake()
            .replace_handler(|_, _request| async move {
                Ok(http_client::Response::builder()
                    .status(503)
                    .body("".into())
                    .unwrap())
            });
        server.disconnect();
        while !matches!(status.next().await, Some(Status::ReconnectionError { .. })) {}

        // Restore the ability to authenticate.
        http_client
            .as_fake()
            .replace_handler(|_, _request| async move {
                Ok(http_client::Response::builder()
                    .status(200)
                    .body("".into())
                    .unwrap())
            });
        cx.executor().advance_clock(Duration::from_secs(10));
        while !matches!(status.next().await, Some(Status::Connected { .. })) {}
        assert_eq!(server.auth_count(), 1); // Client reused the cached credentials when reconnecting
    }

    #[gpui::test(iterations = 10)]
    async fn test_connection_timeout(executor: BackgroundExecutor, cx: &mut TestAppContext) {
        init_test(cx);
        let user_id = 5;
        let client = cx.update(|cx| {
            Client::new(
                Arc::new(FakeSystemClock::new()),
                FakeHttpClient::with_404_response(),
                cx,
            )
        });
        let mut status = client.status();

        // Time out when client tries to connect.
        client.override_authenticate(move |cx| {
            cx.background_spawn(async move {
                Ok(Credentials {
                    user_id,
                    access_token: "token".into(),
                })
            })
        });
        client.override_establish_connection(|_, cx| {
            cx.background_spawn(async move {
                future::pending::<()>().await;
                unreachable!()
            })
        });
        let auth_and_connect = cx.spawn({
            let client = client.clone();
            |cx| async move { client.connect(false, &cx).await }
        });
        executor.run_until_parked();
        assert!(matches!(status.next().await, Some(Status::Connecting)));

        executor.advance_clock(CONNECTION_TIMEOUT);
        assert!(matches!(status.next().await, Some(Status::ConnectionError)));
        auth_and_connect.await.into_response().unwrap_err();

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
            cx.background_spawn(async move {
                future::pending::<()>().await;
                unreachable!()
            })
        });
        executor.advance_clock(2 * INITIAL_RECONNECTION_DELAY);
        assert!(matches!(status.next().await, Some(Status::Reconnecting)));

        executor.advance_clock(CONNECTION_TIMEOUT);
        assert!(matches!(
            status.next().await,
            Some(Status::ReconnectionError { .. })
        ));
    }

    #[gpui::test(iterations = 10)]
    async fn test_reauthenticate_only_if_unauthorized(cx: &mut TestAppContext) {
        init_test(cx);
        let auth_count = Arc::new(Mutex::new(0));
        let http_client = FakeHttpClient::create(|_request| async move {
            Ok(http_client::Response::builder()
                .status(200)
                .body("".into())
                .unwrap())
        });
        let client =
            cx.update(|cx| Client::new(Arc::new(FakeSystemClock::new()), http_client.clone(), cx));
        client.override_authenticate({
            let auth_count = auth_count.clone();
            move |cx| {
                let auth_count = auth_count.clone();
                cx.background_spawn(async move {
                    *auth_count.lock() += 1;
                    Ok(Credentials {
                        user_id: 1,
                        access_token: auth_count.lock().to_string(),
                    })
                })
            }
        });

        let credentials = client.sign_in(false, &cx.to_async()).await.unwrap();
        assert_eq!(*auth_count.lock(), 1);
        assert_eq!(credentials.access_token, "1");

        // If credentials are still valid, signing in doesn't trigger authentication.
        let credentials = client.sign_in(false, &cx.to_async()).await.unwrap();
        assert_eq!(*auth_count.lock(), 1);
        assert_eq!(credentials.access_token, "1");

        // If the server is unavailable, signing in doesn't trigger authentication.
        http_client
            .as_fake()
            .replace_handler(|_, _request| async move {
                Ok(http_client::Response::builder()
                    .status(503)
                    .body("".into())
                    .unwrap())
            });
        client.sign_in(false, &cx.to_async()).await.unwrap_err();
        assert_eq!(*auth_count.lock(), 1);

        // If credentials became invalid, signing in triggers authentication.
        http_client
            .as_fake()
            .replace_handler(|_, request| async move {
                let credentials = parse_authorization_header(&request).unwrap();
                if credentials.access_token == "2" {
                    Ok(http_client::Response::builder()
                        .status(200)
                        .body("".into())
                        .unwrap())
                } else {
                    Ok(http_client::Response::builder()
                        .status(401)
                        .body("".into())
                        .unwrap())
                }
            });
        let credentials = client.sign_in(false, &cx.to_async()).await.unwrap();
        assert_eq!(*auth_count.lock(), 2);
        assert_eq!(credentials.access_token, "2");
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
                Arc::new(FakeSystemClock::new()),
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
                cx.background_spawn(async move {
                    *auth_count.lock() += 1;
                    let _drop = util::defer(move || *dropped_auth_count.lock() += 1);
                    future::pending::<()>().await;
                    unreachable!()
                })
            }
        });

        let _authenticate = cx.spawn({
            let client = client.clone();
            move |cx| async move { client.connect(false, &cx).await }
        });
        executor.run_until_parked();
        assert_eq!(*auth_count.lock(), 1);
        assert_eq!(*dropped_auth_count.lock(), 0);

        let _authenticate = cx.spawn(|cx| async move { client.connect(false, &cx).await });
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
                Arc::new(FakeSystemClock::new()),
                FakeHttpClient::with_404_response(),
                cx,
            )
        });
        let server = FakeServer::for_client(user_id, &client, cx).await;

        let (done_tx1, done_rx1) = smol::channel::unbounded();
        let (done_tx2, done_rx2) = smol::channel::unbounded();
        AnyProtoClient::from(client.clone()).add_entity_message_handler(
            move |entity: Entity<TestEntity>, _: TypedEnvelope<proto::JoinProject>, cx| {
                match entity.read_with(&cx, |entity, _| entity.id).unwrap() {
                    1 => done_tx1.try_send(()).unwrap(),
                    2 => done_tx2.try_send(()).unwrap(),
                    _ => unreachable!(),
                }
                async { Ok(()) }
            },
        );
        let entity1 = cx.new(|_| TestEntity {
            id: 1,
            subscription: None,
        });
        let entity2 = cx.new(|_| TestEntity {
            id: 2,
            subscription: None,
        });
        let entity3 = cx.new(|_| TestEntity {
            id: 3,
            subscription: None,
        });

        let _subscription1 = client
            .subscribe_to_entity(1)
            .unwrap()
            .set_entity(&entity1, &cx.to_async());
        let _subscription2 = client
            .subscribe_to_entity(2)
            .unwrap()
            .set_entity(&entity2, &cx.to_async());
        // Ensure dropping a subscription for the same entity type still allows receiving of
        // messages for other entity IDs of the same type.
        let subscription3 = client
            .subscribe_to_entity(3)
            .unwrap()
            .set_entity(&entity3, &cx.to_async());
        drop(subscription3);

        server.send(proto::JoinProject {
            project_id: 1,
            committer_name: None,
            committer_email: None,
        });
        server.send(proto::JoinProject {
            project_id: 2,
            committer_name: None,
            committer_email: None,
        });
        done_rx1.recv().await.unwrap();
        done_rx2.recv().await.unwrap();
    }

    #[gpui::test]
    async fn test_subscribing_after_dropping_subscription(cx: &mut TestAppContext) {
        init_test(cx);
        let user_id = 5;
        let client = cx.update(|cx| {
            Client::new(
                Arc::new(FakeSystemClock::new()),
                FakeHttpClient::with_404_response(),
                cx,
            )
        });
        let server = FakeServer::for_client(user_id, &client, cx).await;

        let entity = cx.new(|_| TestEntity::default());
        let (done_tx1, _done_rx1) = smol::channel::unbounded();
        let (done_tx2, done_rx2) = smol::channel::unbounded();
        let subscription1 = client.add_message_handler(
            entity.downgrade(),
            move |_, _: TypedEnvelope<proto::Ping>, _| {
                done_tx1.try_send(()).unwrap();
                async { Ok(()) }
            },
        );
        drop(subscription1);
        let _subscription2 = client.add_message_handler(
            entity.downgrade(),
            move |_, _: TypedEnvelope<proto::Ping>, _| {
                done_tx2.try_send(()).unwrap();
                async { Ok(()) }
            },
        );
        server.send(proto::Ping {});
        done_rx2.recv().await.unwrap();
    }

    #[gpui::test]
    async fn test_dropping_subscription_in_handler(cx: &mut TestAppContext) {
        init_test(cx);
        let user_id = 5;
        let client = cx.update(|cx| {
            Client::new(
                Arc::new(FakeSystemClock::new()),
                FakeHttpClient::with_404_response(),
                cx,
            )
        });
        let server = FakeServer::for_client(user_id, &client, cx).await;

        let entity = cx.new(|_| TestEntity::default());
        let (done_tx, done_rx) = smol::channel::unbounded();
        let subscription = client.add_message_handler(
            entity.clone().downgrade(),
            move |entity: Entity<TestEntity>, _: TypedEnvelope<proto::Ping>, mut cx| {
                entity
                    .update(&mut cx, |entity, _| entity.subscription.take())
                    .unwrap();
                done_tx.try_send(()).unwrap();
                async { Ok(()) }
            },
        );
        entity.update(cx, |entity, _| {
            entity.subscription = Some(subscription);
        });
        server.send(proto::Ping {});
        done_rx.recv().await.unwrap();
    }

    #[derive(Default)]
    struct TestEntity {
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
