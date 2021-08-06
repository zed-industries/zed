use crate::{language::LanguageRegistry, worktree::Worktree};
use anyhow::{anyhow, Context, Result};
use async_tungstenite::tungstenite::http::Request;
use async_tungstenite::tungstenite::{Error as WebSocketError, Message as WebSocketMessage};
use gpui::{AsyncAppContext, ModelHandle, Task, WeakModelHandle};
use lazy_static::lazy_static;
use smol::lock::RwLock;
use std::collections::HashMap;
use std::time::Duration;
use std::{convert::TryFrom, future::Future, sync::Arc};
use surf::Url;
pub use zrpc::{proto, ConnectionId, PeerId, TypedEnvelope};
use zrpc::{
    proto::{EnvelopedMessage, RequestMessage},
    ForegroundRouter, Peer, Receipt,
};

lazy_static! {
    static ref ZED_SERVER_URL: String =
        std::env::var("ZED_SERVER_URL").unwrap_or("https://zed.dev:443".to_string());
}

#[derive(Clone)]
pub struct Client {
    peer: Arc<Peer>,
    pub state: Arc<RwLock<ClientState>>,
}

pub struct ClientState {
    connection_id: Option<ConnectionId>,
    pub shared_worktrees: HashMap<u64, WeakModelHandle<Worktree>>,
    pub channel_list: Option<WeakModelHandle<ChannelList>>,
    pub languages: Arc<LanguageRegistry>,
}

impl ClientState {
    pub fn shared_worktree(
        &self,
        id: u64,
        cx: &mut AsyncAppContext,
    ) -> Result<ModelHandle<Worktree>> {
        if let Some(worktree) = self.shared_worktrees.get(&id) {
            if let Some(worktree) = cx.read(|cx| worktree.upgrade(cx)) {
                Ok(worktree)
            } else {
                Err(anyhow!("worktree {} was dropped", id))
            }
        } else {
            Err(anyhow!("worktree {} does not exist", id))
        }
    }
}

impl Client {
    pub fn new(languages: Arc<LanguageRegistry>) -> Self {
        Self {
            peer: Peer::new(),
            state: Arc::new(RwLock::new(ClientState {
                connection_id: None,
                shared_worktrees: Default::default(),
                languages,
            })),
        }
    }

    pub fn on_message<H, M>(
        &self,
        router: &mut ForegroundRouter,
        handler: H,
        cx: &mut gpui::MutableAppContext,
    ) where
        H: 'static + Clone + for<'a> MessageHandler<'a, M>,
        M: proto::EnvelopedMessage,
    {
        let this = self.clone();
        let cx = cx.to_async();
        router.add_message_handler(move |message| {
            let this = this.clone();
            let mut cx = cx.clone();
            let handler = handler.clone();
            async move { handler.handle(message, &this, &mut cx).await }
        });
    }

    pub async fn log_in_and_connect(
        &self,
        router: Arc<ForegroundRouter>,
        cx: AsyncAppContext,
    ) -> surf::Result<()> {
        if self.state.read().await.connection_id.is_some() {
            return Ok(());
        }

        let (user_id, access_token) = Self::login(cx.platform(), &cx.background()).await?;
        let user_id: i32 = user_id.parse()?;
        let request =
            Request::builder().header("Authorization", format!("{} {}", user_id, access_token));

        if let Some(host) = ZED_SERVER_URL.strip_prefix("https://") {
            let stream = smol::net::TcpStream::connect(host).await?;
            let request = request.uri(format!("wss://{}/rpc", host)).body(())?;
            let (stream, _) = async_tungstenite::async_tls::client_async_tls(request, stream)
                .await
                .context("websocket handshake")?;
            log::info!("connected to rpc address {}", *ZED_SERVER_URL);
            self.add_connection(stream, router, cx).await?;
        } else if let Some(host) = ZED_SERVER_URL.strip_prefix("http://") {
            let stream = smol::net::TcpStream::connect(host).await?;
            let request = request.uri(format!("ws://{}/rpc", host)).body(())?;
            let (stream, _) = async_tungstenite::client_async(request, stream).await?;
            log::info!("connected to rpc address {}", *ZED_SERVER_URL);
            self.add_connection(stream, router, cx).await?;
        } else {
            return Err(anyhow!("invalid server url: {}", *ZED_SERVER_URL))?;
        };

        Ok(())
    }

    pub async fn add_connection<Conn>(
        &self,
        conn: Conn,
        router: Arc<ForegroundRouter>,
        cx: AsyncAppContext,
    ) -> surf::Result<()>
    where
        Conn: 'static
            + futures::Sink<WebSocketMessage, Error = WebSocketError>
            + futures::Stream<Item = Result<WebSocketMessage, WebSocketError>>
            + Unpin
            + Send,
    {
        let (connection_id, handle_io, handle_messages) =
            self.peer.add_connection(conn, router).await;
        cx.foreground().spawn(handle_messages).detach();
        cx.background()
            .spawn(async move {
                if let Err(error) = handle_io.await {
                    log::error!("connection error: {:?}", error);
                }
            })
            .detach();
        self.state.write().await.connection_id = Some(connection_id);
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
        let conn_id = self.connection_id().await?;
        self.peer.disconnect(conn_id).await;
        Ok(())
    }

    async fn connection_id(&self) -> Result<ConnectionId> {
        self.state
            .read()
            .await
            .connection_id
            .ok_or_else(|| anyhow!("not connected"))
    }

    pub async fn send<T: EnvelopedMessage>(&self, message: T) -> Result<()> {
        self.peer.send(self.connection_id().await?, message).await
    }

    pub async fn request<T: RequestMessage>(&self, request: T) -> Result<T::Response> {
        self.peer
            .request(self.connection_id().await?, request)
            .await
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
