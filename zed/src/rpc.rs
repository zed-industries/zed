use crate::util::ResultExt;
use anyhow::{anyhow, Context, Result};
use async_tungstenite::tungstenite::http::Request;
use gpui::{AsyncAppContext, Entity, ModelContext, Task};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use postage::{prelude::Stream, watch};
use rand::prelude::*;
use std::{
    any::TypeId,
    collections::HashMap,
    convert::TryFrom,
    future::Future,
    sync::{Arc, Weak},
    time::{Duration, Instant},
};
use surf::Url;
pub use zrpc::{proto, ConnectionId, PeerId, TypedEnvelope};
use zrpc::{
    proto::{AnyTypedEnvelope, EntityMessage, EnvelopedMessage, RequestMessage},
    Conn, Peer, Receipt,
};

lazy_static! {
    static ref ZED_SERVER_URL: String =
        std::env::var("ZED_SERVER_URL").unwrap_or("https://zed.dev:443".to_string());
}

pub struct Client {
    peer: Arc<Peer>,
    state: RwLock<ClientState>,
    auth_callback: Option<
        Box<dyn 'static + Send + Sync + Fn(&AsyncAppContext) -> Task<Result<(u64, String)>>>,
    >,
    connect_callback: Option<
        Box<dyn 'static + Send + Sync + Fn(u64, &str, &AsyncAppContext) -> Task<Result<Conn>>>,
    >,
}

#[derive(Copy, Clone, Debug)]
pub enum Status {
    Disconnected,
    Authenticating,
    Connecting {
        user_id: u64,
    },
    ConnectionError,
    Connected {
        connection_id: ConnectionId,
        user_id: u64,
    },
    ConnectionLost,
    Reauthenticating,
    Reconnecting {
        user_id: u64,
    },
    ReconnectionError {
        next_reconnection: Instant,
    },
}

struct ClientState {
    status: (watch::Sender<Status>, watch::Receiver<Status>),
    entity_id_extractors: HashMap<TypeId, Box<dyn Send + Sync + Fn(&dyn AnyTypedEnvelope) -> u64>>,
    model_handlers: HashMap<
        (TypeId, u64),
        Box<dyn Send + Sync + FnMut(Box<dyn AnyTypedEnvelope>, &mut AsyncAppContext)>,
    >,
    _maintain_connection: Option<Task<()>>,
    heartbeat_interval: Duration,
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            status: watch::channel_with(Status::Disconnected),
            entity_id_extractors: Default::default(),
            model_handlers: Default::default(),
            _maintain_connection: None,
            heartbeat_interval: Duration::from_secs(5),
        }
    }
}

pub struct Subscription {
    client: Weak<Client>,
    id: (TypeId, u64),
}

impl Drop for Subscription {
    fn drop(&mut self) {
        if let Some(client) = self.client.upgrade() {
            drop(
                client
                    .state
                    .write()
                    .model_handlers
                    .remove(&self.id)
                    .unwrap(),
            );
        }
    }
}

impl Client {
    pub fn new() -> Arc<Self> {
        Arc::new(Self {
            peer: Peer::new(),
            state: Default::default(),
            auth_callback: None,
            connect_callback: None,
        })
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_login_and_connect_callbacks<Login, Connect>(
        &mut self,
        login: Login,
        connect: Connect,
    ) where
        Login: 'static + Send + Sync + Fn(&AsyncAppContext) -> Task<Result<(u64, String)>>,
        Connect: 'static + Send + Sync + Fn(u64, &str, &AsyncAppContext) -> Task<Result<Conn>>,
    {
        self.auth_callback = Some(Box::new(login));
        self.connect_callback = Some(Box::new(connect));
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
                    let mut next_ping_id = 0;
                    loop {
                        foreground.timer(heartbeat_interval).await;
                        this.request(proto::Ping { id: next_ping_id })
                            .await
                            .unwrap();
                        next_ping_id += 1;
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
            Status::Disconnected => {
                state._maintain_connection.take();
            }
            _ => {}
        }
    }

    pub fn subscribe_from_model<T, M, F>(
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
            + FnMut(&mut M, TypedEnvelope<T>, Arc<Self>, &mut ModelContext<M>) -> Result<()>,
    {
        let subscription_id = (TypeId::of::<T>(), remote_id);
        let client = self.clone();
        let mut state = self.state.write();
        let model = cx.handle().downgrade();
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
            Box::new(move |envelope, cx| {
                if let Some(model) = model.upgrade(cx) {
                    let envelope = envelope.into_any().downcast::<TypedEnvelope<T>>().unwrap();
                    model.update(cx, |model, cx| {
                        if let Err(error) = handler(model, *envelope, client.clone(), cx) {
                            log::error!("error handling message: {}", error)
                        }
                    });
                }
            }),
        );
        if prev_handler.is_some() {
            panic!("registered a handler for the same entity twice")
        }

        Subscription {
            client: Arc::downgrade(self),
            id: subscription_id,
        }
    }

    pub async fn authenticate_and_connect(
        self: &Arc<Self>,
        cx: &AsyncAppContext,
    ) -> anyhow::Result<()> {
        let was_disconnected = match *self.status().borrow() {
            Status::Disconnected => true,
            Status::ConnectionError | Status::ConnectionLost | Status::ReconnectionError { .. } => {
                false
            }
            Status::Connected { .. }
            | Status::Connecting { .. }
            | Status::Reconnecting { .. }
            | Status::Authenticating
            | Status::Reauthenticating => return Ok(()),
        };

        if was_disconnected {
            self.set_status(Status::Authenticating, cx);
        } else {
            self.set_status(Status::Reauthenticating, cx)
        }

        let (user_id, access_token) = match self.authenticate(&cx).await {
            Ok(result) => result,
            Err(err) => {
                self.set_status(Status::ConnectionError, cx);
                return Err(err);
            }
        };

        if was_disconnected {
            self.set_status(Status::Connecting { user_id }, cx);
        } else {
            self.set_status(Status::Reconnecting { user_id }, cx);
        }
        match self.connect(user_id, &access_token, cx).await {
            Ok(conn) => {
                log::info!("connected to rpc address {}", *ZED_SERVER_URL);
                self.set_connection(user_id, conn, cx).await;
                Ok(())
            }
            Err(err) => {
                self.set_status(Status::ConnectionError, cx);
                Err(err)
            }
        }
    }

    async fn set_connection(self: &Arc<Self>, user_id: u64, conn: Conn, cx: &AsyncAppContext) {
        let (connection_id, handle_io, mut incoming) = self.peer.add_connection(conn).await;
        cx.foreground()
            .spawn({
                let mut cx = cx.clone();
                let this = self.clone();
                async move {
                    while let Some(message) = incoming.recv().await {
                        let mut state = this.state.write();
                        if let Some(extract_entity_id) =
                            state.entity_id_extractors.get(&message.payload_type_id())
                        {
                            let entity_id = (extract_entity_id)(message.as_ref());
                            if let Some(handler) = state
                                .model_handlers
                                .get_mut(&(message.payload_type_id(), entity_id))
                            {
                                let start_time = Instant::now();
                                log::info!("RPC client message {}", message.payload_type_name());
                                (handler)(message, &mut cx);
                                log::info!(
                                    "RPC message handled. duration:{:?}",
                                    start_time.elapsed()
                                );
                            } else {
                                log::info!("unhandled message {}", message.payload_type_name());
                            }
                        } else {
                            log::info!("unhandled message {}", message.payload_type_name());
                        }
                    }
                }
            })
            .detach();

        self.set_status(
            Status::Connected {
                connection_id,
                user_id,
            },
            cx,
        );

        let handle_io = cx.background().spawn(handle_io);
        let this = self.clone();
        let cx = cx.clone();
        cx.foreground()
            .spawn(async move {
                match handle_io.await {
                    Ok(()) => this.set_status(Status::Disconnected, &cx),
                    Err(err) => {
                        log::error!("connection error: {:?}", err);
                        this.set_status(Status::ConnectionLost, &cx);
                    }
                }
            })
            .detach();
    }

    fn authenticate(self: &Arc<Self>, cx: &AsyncAppContext) -> Task<Result<(u64, String)>> {
        if let Some(callback) = self.auth_callback.as_ref() {
            callback(cx)
        } else {
            self.authenticate_with_browser(cx)
        }
    }

    fn connect(
        self: &Arc<Self>,
        user_id: u64,
        access_token: &str,
        cx: &AsyncAppContext,
    ) -> Task<Result<Conn>> {
        if let Some(callback) = self.connect_callback.as_ref() {
            callback(user_id, access_token, cx)
        } else {
            self.connect_with_websocket(user_id, access_token, cx)
        }
    }

    fn connect_with_websocket(
        self: &Arc<Self>,
        user_id: u64,
        access_token: &str,
        cx: &AsyncAppContext,
    ) -> Task<Result<Conn>> {
        let request =
            Request::builder().header("Authorization", format!("{} {}", user_id, access_token));
        cx.background().spawn(async move {
            if let Some(host) = ZED_SERVER_URL.strip_prefix("https://") {
                let stream = smol::net::TcpStream::connect(host).await?;
                let request = request.uri(format!("wss://{}/rpc", host)).body(())?;
                let (stream, _) = async_tungstenite::async_tls::client_async_tls(request, stream)
                    .await
                    .context("websocket handshake")?;
                Ok(Conn::new(stream))
            } else if let Some(host) = ZED_SERVER_URL.strip_prefix("http://") {
                let stream = smol::net::TcpStream::connect(host).await?;
                let request = request.uri(format!("ws://{}/rpc", host)).body(())?;
                let (stream, _) = async_tungstenite::client_async(request, stream)
                    .await
                    .context("websocket handshake")?;
                Ok(Conn::new(stream))
            } else {
                Err(anyhow!("invalid server url: {}", *ZED_SERVER_URL))
            }
        })
    }

    pub fn authenticate_with_browser(
        self: &Arc<Self>,
        cx: &AsyncAppContext,
    ) -> Task<Result<(u64, String)>> {
        let platform = cx.platform();
        let executor = cx.background();
        executor.clone().spawn(async move {
            if let Some((user_id, access_token)) = platform
                .read_credentials(&ZED_SERVER_URL)
                .log_err()
                .flatten()
            {
                log::info!("already signed in. user_id: {}", user_id);
                return Ok((user_id.parse()?, String::from_utf8(access_token).unwrap()));
            }

            // Generate a pair of asymmetric encryption keys. The public key will be used by the
            // zed server to encrypt the user's access token, so that it can'be intercepted by
            // any other app running on the user's device.
            let (public_key, private_key) =
                zrpc::auth::keypair().expect("failed to generate keypair for auth");
            let public_key_string =
                String::try_from(public_key).expect("failed to serialize public key for auth");

            // Start an HTTP server to receive the redirect from Zed's sign-in page.
            let server = tiny_http::Server::http("127.0.0.1:0").expect("failed to find open port");
            let port = server.server_addr().port();

            // Open the Zed sign-in page in the user's browser, with query parameters that indicate
            // that the user is signing in from a Zed app running on the same device.
            platform.open_url(&format!(
                "{}/sign_in?native_app_port={}&native_app_public_key={}",
                *ZED_SERVER_URL, port, public_key_string
            ));

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
                        req.respond(
                            tiny_http::Response::from_string(LOGIN_RESPONSE).with_header(
                                tiny_http::Header::from_bytes("Content-Type", "text/html").unwrap(),
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
            platform
                .write_credentials(&ZED_SERVER_URL, &user_id, access_token.as_bytes())
                .log_err();
            Ok((user_id.parse()?, access_token))
        })
    }

    pub async fn disconnect(self: &Arc<Self>, cx: &AsyncAppContext) -> Result<()> {
        let conn_id = self.connection_id()?;
        self.peer.disconnect(conn_id).await;
        self.set_status(Status::Disconnected, cx);
        Ok(())
    }

    fn connection_id(&self) -> Result<ConnectionId> {
        if let Status::Connected { connection_id, .. } = *self.status().borrow() {
            Ok(connection_id)
        } else {
            Err(anyhow!("not connected"))
        }
    }

    pub async fn send<T: EnvelopedMessage>(&self, message: T) -> Result<()> {
        self.peer.send(self.connection_id()?, message).await
    }

    pub async fn request<T: RequestMessage>(&self, request: T) -> Result<T::Response> {
        self.peer.request(self.connection_id()?, request).await
    }

    pub fn respond<T: RequestMessage>(
        &self,
        receipt: Receipt<T>,
        response: T::Response,
    ) -> impl Future<Output = Result<()>> {
        self.peer.respond(receipt, response)
    }
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

const LOGIN_RESPONSE: &'static str = "
<!DOCTYPE html>
<html>
<script>window.close();</script>
</html>
";

#[cfg(test)]
mod tests {
    use super::*;
    use crate::test::FakeServer;
    use gpui::TestAppContext;

    #[gpui::test(iterations = 10)]
    async fn test_heartbeat(cx: TestAppContext) {
        cx.foreground().forbid_parking();

        let user_id = 5;
        let mut client = Client::new();
        let server = FakeServer::for_client(user_id, &mut client, &cx).await;

        cx.foreground().advance_clock(Duration::from_secs(10));
        let ping = server.receive::<proto::Ping>().await.unwrap();
        assert_eq!(ping.payload.id, 0);
        server.respond(ping.receipt(), proto::Pong { id: 0 }).await;

        cx.foreground().advance_clock(Duration::from_secs(10));
        let ping = server.receive::<proto::Ping>().await.unwrap();
        assert_eq!(ping.payload.id, 1);
        server.respond(ping.receipt(), proto::Pong { id: 1 }).await;

        client.disconnect(&cx.to_async()).await.unwrap();
        assert!(server.receive::<proto::Ping>().await.is_err());
    }

    #[gpui::test(iterations = 10)]
    async fn test_reconnection(cx: TestAppContext) {
        cx.foreground().forbid_parking();

        let user_id = 5;
        let mut client = Client::new();
        let server = FakeServer::for_client(user_id, &mut client, &cx).await;
        let mut status = client.status();
        assert!(matches!(
            status.recv().await,
            Some(Status::Connected { .. })
        ));

        server.forbid_connections();
        server.disconnect().await;
        while !matches!(status.recv().await, Some(Status::ReconnectionError { .. })) {}

        server.allow_connections();
        cx.foreground().advance_clock(Duration::from_secs(10));
        while !matches!(status.recv().await, Some(Status::Connected { .. })) {}
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
}
