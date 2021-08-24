use anyhow::{anyhow, Context, Result};
use async_tungstenite::tungstenite::http::Request;
use async_tungstenite::tungstenite::{Error as WebSocketError, Message as WebSocketMessage};
use gpui::{AsyncAppContext, Entity, ModelContext, Task};
use lazy_static::lazy_static;
use parking_lot::RwLock;
use postage::prelude::Stream;
use postage::sink::Sink;
use postage::watch;
use std::any::TypeId;
use std::collections::HashMap;
use std::sync::Weak;
use std::time::{Duration, Instant};
use std::{convert::TryFrom, future::Future, sync::Arc};
use surf::Url;
use zrpc::proto::{AnyTypedEnvelope, EntityMessage};
pub use zrpc::{proto, ConnectionId, PeerId, TypedEnvelope};
use zrpc::{
    proto::{EnvelopedMessage, RequestMessage},
    Peer, Receipt,
};

lazy_static! {
    static ref ZED_SERVER_URL: String =
        std::env::var("ZED_SERVER_URL").unwrap_or("https://zed.dev:443".to_string());
}

pub struct Client {
    peer: Arc<Peer>,
    state: RwLock<ClientState>,
}

struct ClientState {
    connection_id: Option<ConnectionId>,
    user_id: (watch::Sender<Option<u64>>, watch::Receiver<Option<u64>>),
    entity_id_extractors: HashMap<TypeId, Box<dyn Send + Sync + Fn(&dyn AnyTypedEnvelope) -> u64>>,
    model_handlers: HashMap<
        (TypeId, u64),
        Box<dyn Send + Sync + FnMut(Box<dyn AnyTypedEnvelope>, &mut AsyncAppContext)>,
    >,
}

impl Default for ClientState {
    fn default() -> Self {
        Self {
            connection_id: Default::default(),
            user_id: watch::channel(),
            entity_id_extractors: Default::default(),
            model_handlers: Default::default(),
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
        })
    }

    pub fn user_id(&self) -> watch::Receiver<Option<u64>> {
        self.state.read().user_id.1.clone()
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
        cx: AsyncAppContext,
    ) -> anyhow::Result<()> {
        if self.state.read().connection_id.is_some() {
            return Ok(());
        }

        let (user_id, access_token) = Self::login(cx.platform(), &cx.background()).await?;
        let user_id = user_id.parse::<u64>()?;
        let request =
            Request::builder().header("Authorization", format!("{} {}", user_id, access_token));

        if let Some(host) = ZED_SERVER_URL.strip_prefix("https://") {
            let stream = smol::net::TcpStream::connect(host).await?;
            let request = request.uri(format!("wss://{}/rpc", host)).body(())?;
            let (stream, _) = async_tungstenite::async_tls::client_async_tls(request, stream)
                .await
                .context("websocket handshake")?;
            self.add_connection(user_id, stream, cx).await?;
        } else if let Some(host) = ZED_SERVER_URL.strip_prefix("http://") {
            let stream = smol::net::TcpStream::connect(host).await?;
            let request = request.uri(format!("ws://{}/rpc", host)).body(())?;
            let (stream, _) = async_tungstenite::client_async(request, stream)
                .await
                .context("websocket handshake")?;
            self.add_connection(user_id, stream, cx).await?;
        } else {
            return Err(anyhow!("invalid server url: {}", *ZED_SERVER_URL))?;
        };

        log::info!("connected to rpc address {}", *ZED_SERVER_URL);
        Ok(())
    }

    pub async fn add_connection<Conn>(
        self: &Arc<Self>,
        user_id: u64,
        conn: Conn,
        cx: AsyncAppContext,
    ) -> anyhow::Result<()>
    where
        Conn: 'static
            + futures::Sink<WebSocketMessage, Error = WebSocketError>
            + futures::Stream<Item = Result<WebSocketMessage, WebSocketError>>
            + Unpin
            + Send,
    {
        let (connection_id, handle_io, mut incoming) = self.peer.add_connection(conn).await;
        {
            let mut cx = cx.clone();
            let this = self.clone();
            cx.foreground()
                .spawn(async move {
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
                })
                .detach();
        }
        cx.background()
            .spawn(async move {
                if let Err(error) = handle_io.await {
                    log::error!("connection error: {:?}", error);
                }
            })
            .detach();
        let mut state = self.state.write();
        state.connection_id = Some(connection_id);
        state.user_id.0.send(Some(user_id)).await?;
        Ok(())
    }

    pub fn login(
        platform: Arc<dyn gpui::Platform>,
        executor: &Arc<gpui::executor::Background>,
    ) -> Task<Result<(String, String)>> {
        let executor = executor.clone();
        executor.clone().spawn(async move {
            if let Some((user_id, access_token)) = platform.read_credentials(&ZED_SERVER_URL) {
                log::info!("already signed in. user_id: {}", user_id);
                return Ok((user_id, String::from_utf8(access_token).unwrap()));
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
            platform.write_credentials(&ZED_SERVER_URL, &user_id, access_token.as_bytes());
            Ok((user_id.to_string(), access_token))
        })
    }

    pub async fn disconnect(&self) -> Result<()> {
        let conn_id = self.connection_id()?;
        self.peer.disconnect(conn_id).await;
        Ok(())
    }

    fn connection_id(&self) -> Result<ConnectionId> {
        self.state
            .read()
            .connection_id
            .ok_or_else(|| anyhow!("not connected"))
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

pub trait MessageHandler<'a, M: proto::EnvelopedMessage>: Clone {
    type Output: 'a + Future<Output = anyhow::Result<()>>;

    fn handle(
        &self,
        message: TypedEnvelope<M>,
        rpc: &'a Client,
        cx: &'a mut gpui::AsyncAppContext,
    ) -> Self::Output;
}

impl<'a, M, F, Fut> MessageHandler<'a, M> for F
where
    M: proto::EnvelopedMessage,
    F: Clone + Fn(TypedEnvelope<M>, &'a Client, &'a mut gpui::AsyncAppContext) -> Fut,
    Fut: 'a + Future<Output = anyhow::Result<()>>,
{
    type Output = Fut;

    fn handle(
        &self,
        message: TypedEnvelope<M>,
        rpc: &'a Client,
        cx: &'a mut gpui::AsyncAppContext,
    ) -> Self::Output {
        (self)(message, rpc, cx)
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
