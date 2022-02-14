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
use gpui::{action, AsyncAppContext, Entity, ModelContext, ModelHandle, MutableAppContext, Task};
use http::HttpClient;
use lazy_static::lazy_static;
use parking_lot::RwLock;
use postage::watch;
use rand::prelude::*;
use rpc::proto::{AnyTypedEnvelope, EntityMessage, EnvelopedMessage, RequestMessage};
use std::{
    any::{type_name, TypeId},
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
    static ref IMPERSONATE_LOGIN: Option<String> = std::env::var("ZED_IMPERSONATE")
        .ok()
        .and_then(|s| if s.is_empty() { None } else { Some(s) });
}

action!(Authenticate);

pub fn init(rpc: Arc<Client>, cx: &mut MutableAppContext) {
    cx.add_global_action(move |_: &Authenticate, cx| {
        let rpc = rpc.clone();
        cx.spawn(|cx| async move { rpc.authenticate_and_connect(&cx).log_err().await })
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

type ModelHandler = Box<
    dyn Send
        + Sync
        + FnMut(Box<dyn AnyTypedEnvelope>, &AsyncAppContext) -> LocalBoxFuture<'static, Result<()>>,
>;

struct ClientState {
    credentials: Option<Credentials>,
    status: (watch::Sender<Status>, watch::Receiver<Status>),
    entity_id_extractors: HashMap<TypeId, Box<dyn Send + Sync + Fn(&dyn AnyTypedEnvelope) -> u64>>,
    model_handlers: HashMap<(TypeId, Option<u64>), Option<ModelHandler>>,
    _maintain_connection: Option<Task<()>>,
    heartbeat_interval: Duration,
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
            model_handlers: Default::default(),
            _maintain_connection: None,
            heartbeat_interval: Duration::from_secs(5),
        }
    }
}

pub struct Subscription {
    client: Weak<Client>,
    id: (TypeId, Option<u64>),
}

impl Drop for Subscription {
    fn drop(&mut self) {
        if let Some(client) = self.client.upgrade() {
            let mut state = client.state.write();
            let _ = state.model_handlers.remove(&self.id).unwrap();
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
                let heartbeat_interval = state.heartbeat_interval;
                let this = self.clone();
                let foreground = cx.foreground();
                state._maintain_connection = Some(cx.foreground().spawn(async move {
                    loop {
                        foreground.timer(heartbeat_interval).await;
                        let _ = this.request(proto::Ping {}).await;
                    }
                }));
            }
            Status::ConnectionLost => {
                let this = self.clone();
                let foreground = cx.foreground();
                let heartbeat_interval = state.heartbeat_interval;
                state._maintain_connection = Some(cx.spawn(|cx| async move {
                    let mut rng = StdRng::from_entropy();
                    let mut delay = Duration::from_millis(100);
                    while let Err(error) = this.authenticate_and_connect(&cx).await {
                        log::error!("failed to connect {}", error);
                        this.set_status(
                            Status::ReconnectionError {
                                next_reconnection: Instant::now() + delay,
                            },
                            &cx,
                        );
                        foreground.timer(delay).await;
                        delay = delay
                            .mul_f32(rng.gen_range(1.0..=2.0))
                            .min(heartbeat_interval);
                    }
                }));
            }
            Status::SignedOut | Status::UpgradeRequired => {
                state._maintain_connection.take();
            }
            _ => {}
        }
    }

    pub fn add_message_handler<T, M, F, Fut>(
        self: &Arc<Self>,
        cx: &mut ModelContext<M>,
        mut handler: F,
    ) -> Subscription
    where
        T: EnvelopedMessage,
        M: Entity,
        F: 'static
            + Send
            + Sync
            + FnMut(ModelHandle<M>, TypedEnvelope<T>, Arc<Self>, AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = Result<()>>,
    {
        let subscription_id = (TypeId::of::<T>(), None);
        let client = self.clone();
        let mut state = self.state.write();
        let model = cx.weak_handle();
        let prev_handler = state.model_handlers.insert(
            subscription_id,
            Some(Box::new(move |envelope, cx| {
                if let Some(model) = model.upgrade(cx) {
                    let envelope = envelope.into_any().downcast::<TypedEnvelope<T>>().unwrap();
                    handler(model, *envelope, client.clone(), cx.clone()).boxed_local()
                } else {
                    async move {
                        Err(anyhow!(
                            "received message for {:?} but model was dropped",
                            type_name::<M>()
                        ))
                    }
                    .boxed_local()
                }
            })),
        );
        if prev_handler.is_some() {
            panic!("registered handler for the same message twice");
        }

        Subscription {
            client: Arc::downgrade(self),
            id: subscription_id,
        }
    }

    pub fn add_entity_message_handler<T, M, F, Fut>(
        self: &Arc<Self>,
        remote_id: u64,
        cx: &mut ModelContext<M>,
        mut handler: F,
    ) -> Subscription
    where
        T: EntityMessage,
        M: Entity,
        F: 'static
            + Send
            + Sync
            + FnMut(ModelHandle<M>, TypedEnvelope<T>, Arc<Self>, AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = Result<()>>,
    {
        let subscription_id = (TypeId::of::<T>(), Some(remote_id));
        let client = self.clone();
        let mut state = self.state.write();
        let model = cx.weak_handle();
        state
            .entity_id_extractors
            .entry(subscription_id.0)
            .or_insert_with(|| {
                Box::new(|envelope| {
                    let envelope = envelope
                        .as_any()
                        .downcast_ref::<TypedEnvelope<T>>()
                        .unwrap();
                    envelope.payload.remote_entity_id()
                })
            });
        let prev_handler = state.model_handlers.insert(
            subscription_id,
            Some(Box::new(move |envelope, cx| {
                if let Some(model) = model.upgrade(cx) {
                    let envelope = envelope.into_any().downcast::<TypedEnvelope<T>>().unwrap();
                    handler(model, *envelope, client.clone(), cx.clone()).boxed_local()
                } else {
                    async move {
                        Err(anyhow!(
                            "received message for {:?} but model was dropped",
                            type_name::<M>()
                        ))
                    }
                    .boxed_local()
                }
            })),
        );
        if prev_handler.is_some() {
            panic!("registered a handler for the same entity twice")
        }

        Subscription {
            client: Arc::downgrade(self),
            id: subscription_id,
        }
    }

    pub fn add_entity_request_handler<T, M, F, Fut>(
        self: &Arc<Self>,
        remote_id: u64,
        cx: &mut ModelContext<M>,
        mut handler: F,
    ) -> Subscription
    where
        T: EntityMessage + RequestMessage,
        M: Entity,
        F: 'static
            + Send
            + Sync
            + FnMut(ModelHandle<M>, TypedEnvelope<T>, Arc<Self>, AsyncAppContext) -> Fut,
        Fut: 'static + Future<Output = Result<T::Response>>,
    {
        self.add_entity_message_handler(remote_id, cx, move |model, envelope, client, cx| {
            let receipt = envelope.receipt();
            let response = handler(model, envelope, client.clone(), cx);
            async move {
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
        })
    }

    pub fn has_keychain_credentials(&self, cx: &AsyncAppContext) -> bool {
        read_credentials_from_keychain(cx).is_some()
    }

    #[async_recursion(?Send)]
    pub async fn authenticate_and_connect(
        self: &Arc<Self>,
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

        let mut used_keychain = false;
        let credentials = self.state.read().credentials.clone();
        let credentials = if let Some(credentials) = credentials {
            credentials
        } else if let Some(credentials) = read_credentials_from_keychain(cx) {
            used_keychain = true;
            credentials
        } else {
            let credentials = match self.authenticate(&cx).await {
                Ok(credentials) => credentials,
                Err(err) => {
                    self.set_status(Status::ConnectionError, cx);
                    return Err(err);
                }
            };
            credentials
        };

        if was_disconnected {
            self.set_status(Status::Connecting, cx);
        } else {
            self.set_status(Status::Reconnecting, cx);
        }

        match self.establish_connection(&credentials, cx).await {
            Ok(conn) => {
                self.state.write().credentials = Some(credentials.clone());
                if !used_keychain && IMPERSONATE_LOGIN.is_none() {
                    write_credentials_to_keychain(&credentials, cx).log_err();
                }
                self.set_connection(conn, cx).await;
                Ok(())
            }
            Err(EstablishConnectionError::Unauthorized) => {
                self.state.write().credentials.take();
                if used_keychain {
                    cx.platform().delete_credentials(&ZED_SERVER_URL).log_err();
                    self.set_status(Status::SignedOut, cx);
                    self.authenticate_and_connect(cx).await
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
        let (connection_id, handle_io, mut incoming) = self.peer.add_connection(conn).await;
        cx.foreground()
            .spawn({
                let cx = cx.clone();
                let this = self.clone();
                async move {
                    while let Some(message) = incoming.next().await {
                        let mut state = this.state.write();
                        let payload_type_id = message.payload_type_id();
                        let entity_id = if let Some(extract_entity_id) =
                            state.entity_id_extractors.get(&message.payload_type_id())
                        {
                            Some((extract_entity_id)(message.as_ref()))
                        } else {
                            None
                        };

                        let type_name = message.payload_type_name();

                        let handler_key = (payload_type_id, entity_id);
                        if let Some(handler) = state.model_handlers.get_mut(&handler_key) {
                            let mut handler = handler.take().unwrap();
                            drop(state); // Avoid deadlocks if the handler interacts with rpc::Client
                            let future = (handler)(message, &cx);
                            {
                                let mut state = this.state.write();
                                if state.model_handlers.contains_key(&handler_key) {
                                    state.model_handlers.insert(handler_key, Some(handler));
                                }
                            }

                            let client_id = this.id;
                            log::debug!(
                                "rpc message received. client_id:{}, name:{}",
                                client_id,
                                type_name
                            );
                            cx.foreground()
                                .spawn(async move {
                                    match future.await {
                                        Ok(()) => {
                                            log::debug!(
                                                "{}: rpc message '{}' handled",
                                                client_id,
                                                type_name
                                            );
                                        }
                                        Err(error) => {
                                            log::error!(
                                                "{}: error handling rpc message '{}', {}",
                                                client_id,
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

    pub async fn request<T: RequestMessage>(&self, request: T) -> Result<T::Response> {
        log::debug!(
            "rpc request start. client_id: {}. name:{}",
            self.id,
            T::NAME
        );
        let response = self.peer.request(self.connection_id()?, request).await;
        log::debug!(
            "rpc request finish. client_id: {}. name:{}",
            self.id,
            T::NAME
        );
        response
    }

    fn respond<T: RequestMessage>(&self, receipt: Receipt<T>, response: T::Response) -> Result<()> {
        log::debug!("rpc respond. client_id: {}. name:{}", self.id, T::NAME);
        self.peer.respond(receipt, response)
    }

    fn respond_with_error<T: RequestMessage>(
        &self,
        receipt: Receipt<T>,
        error: proto::Error,
    ) -> Result<()> {
        log::debug!("rpc respond. client_id: {}. name:{}", self.id, T::NAME);
        self.peer.respond_with_error(receipt, error)
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
    async fn test_heartbeat(cx: TestAppContext) {
        cx.foreground().forbid_parking();

        let user_id = 5;
        let mut client = Client::new(FakeHttpClient::with_404_response());
        let server = FakeServer::for_client(user_id, &mut client, &cx).await;

        cx.foreground().advance_clock(Duration::from_secs(10));
        let ping = server.receive::<proto::Ping>().await.unwrap();
        server.respond(ping.receipt(), proto::Ack {}).await;

        cx.foreground().advance_clock(Duration::from_secs(10));
        let ping = server.receive::<proto::Ping>().await.unwrap();
        server.respond(ping.receipt(), proto::Ack {}).await;

        client.disconnect(&cx.to_async()).unwrap();
        assert!(server.receive::<proto::Ping>().await.is_err());
    }

    #[gpui::test(iterations = 10)]
    async fn test_reconnection(cx: TestAppContext) {
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
        assert_eq!(server.auth_count(), 1);
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
    async fn test_subscribing_to_entity(mut cx: TestAppContext) {
        cx.foreground().forbid_parking();

        let user_id = 5;
        let mut client = Client::new(FakeHttpClient::with_404_response());
        let server = FakeServer::for_client(user_id, &mut client, &cx).await;

        let model = cx.add_model(|_| Model { subscription: None });
        let (mut done_tx1, mut done_rx1) = postage::oneshot::channel();
        let (mut done_tx2, mut done_rx2) = postage::oneshot::channel();
        let _subscription1 = model.update(&mut cx, |_, cx| {
            client.add_entity_message_handler(
                1,
                cx,
                move |_, _: TypedEnvelope<proto::UnshareProject>, _, _| {
                    postage::sink::Sink::try_send(&mut done_tx1, ()).unwrap();
                    async { Ok(()) }
                },
            )
        });
        let _subscription2 = model.update(&mut cx, |_, cx| {
            client.add_entity_message_handler(
                2,
                cx,
                move |_, _: TypedEnvelope<proto::UnshareProject>, _, _| {
                    postage::sink::Sink::try_send(&mut done_tx2, ()).unwrap();
                    async { Ok(()) }
                },
            )
        });

        // Ensure dropping a subscription for the same entity type still allows receiving of
        // messages for other entity IDs of the same type.
        let subscription3 = model.update(&mut cx, |_, cx| {
            client.add_entity_message_handler(
                3,
                cx,
                |_, _: TypedEnvelope<proto::UnshareProject>, _, _| async { Ok(()) },
            )
        });
        drop(subscription3);

        server.send(proto::UnshareProject { project_id: 1 });
        server.send(proto::UnshareProject { project_id: 2 });
        done_rx1.next().await.unwrap();
        done_rx2.next().await.unwrap();
    }

    #[gpui::test]
    async fn test_subscribing_after_dropping_subscription(mut cx: TestAppContext) {
        cx.foreground().forbid_parking();

        let user_id = 5;
        let mut client = Client::new(FakeHttpClient::with_404_response());
        let server = FakeServer::for_client(user_id, &mut client, &cx).await;

        let model = cx.add_model(|_| Model { subscription: None });
        let (mut done_tx1, _done_rx1) = postage::oneshot::channel();
        let (mut done_tx2, mut done_rx2) = postage::oneshot::channel();
        let subscription1 = model.update(&mut cx, |_, cx| {
            client.add_message_handler(cx, move |_, _: TypedEnvelope<proto::Ping>, _, _| {
                postage::sink::Sink::try_send(&mut done_tx1, ()).unwrap();
                async { Ok(()) }
            })
        });
        drop(subscription1);
        let _subscription2 = model.update(&mut cx, |_, cx| {
            client.add_message_handler(cx, move |_, _: TypedEnvelope<proto::Ping>, _, _| {
                postage::sink::Sink::try_send(&mut done_tx2, ()).unwrap();
                async { Ok(()) }
            })
        });
        server.send(proto::Ping {});
        done_rx2.next().await.unwrap();
    }

    #[gpui::test]
    async fn test_dropping_subscription_in_handler(mut cx: TestAppContext) {
        cx.foreground().forbid_parking();

        let user_id = 5;
        let mut client = Client::new(FakeHttpClient::with_404_response());
        let server = FakeServer::for_client(user_id, &mut client, &cx).await;

        let model = cx.add_model(|_| Model { subscription: None });
        let (mut done_tx, mut done_rx) = postage::oneshot::channel();
        model.update(&mut cx, |model, cx| {
            model.subscription = Some(client.add_message_handler(
                cx,
                move |model, _: TypedEnvelope<proto::Ping>, _, mut cx| {
                    model.update(&mut cx, |model, _| model.subscription.take());
                    postage::sink::Sink::try_send(&mut done_tx, ()).unwrap();
                    async { Ok(()) }
                },
            ));
        });
        server.send(proto::Ping {});
        done_rx.next().await.unwrap();
    }

    struct Model {
        subscription: Option<Subscription>,
    }

    impl Entity for Model {
        type Event = ();
    }
}
