use crate::worktree::{FileHandle, Worktree};

use super::util::SurfResultExt as _;
use anyhow::{anyhow, Context, Result};
use gpui::executor::Background;
use gpui::{AsyncAppContext, ModelHandle, Task};
use lazy_static::lazy_static;
use postage::prelude::Stream;
use smol::lock::Mutex;
use std::collections::{HashMap, HashSet};
use std::time::Duration;
use std::{convert::TryFrom, future::Future, sync::Arc};
use surf::Url;
use zed_rpc::proto::EnvelopedMessage;
use zed_rpc::{proto::RequestMessage, rest, Peer, TypedEnvelope};
use zed_rpc::{PeerId, Receipt};

pub use zed_rpc::{proto, ConnectionId};

lazy_static! {
    static ref ZED_SERVER_URL: String =
        std::env::var("ZED_SERVER_URL").unwrap_or("https://zed.dev".to_string());
}

#[derive(Clone)]
pub struct Client {
    peer: Arc<Peer>,
    pub state: Arc<Mutex<ClientState>>,
}

#[derive(Default)]
pub struct ClientState {
    connection_id: Option<ConnectionId>,
    pub shared_worktrees: HashSet<ModelHandle<Worktree>>,
    pub shared_files: HashMap<FileHandle, HashMap<PeerId, usize>>,
}

impl Client {
    pub fn new() -> Self {
        Self {
            peer: Peer::new(),
            state: Default::default(),
        }
    }

    pub fn on_message<H, M>(&self, handler: H, cx: &mut gpui::MutableAppContext)
    where
        H: 'static + for<'a> MessageHandler<'a, M>,
        M: proto::EnvelopedMessage,
    {
        let this = self.clone();
        let mut messages = smol::block_on(this.peer.add_message_handler::<M>());
        cx.spawn(|mut cx| async move {
            while let Some(message) = messages.recv().await {
                if let Err(err) = handler.handle(message, &this, &mut cx).await {
                    log::error!("error handling message: {:?}", err);
                }
            }
        })
        .detach();
    }

    pub async fn connect_to_server(
        &self,
        cx: &AsyncAppContext,
        executor: &Arc<Background>,
    ) -> surf::Result<ConnectionId> {
        if let Some(connection_id) = self.state.lock().await.connection_id {
            return Ok(connection_id);
        }

        let (user_id, access_token) = Self::login(cx.platform(), executor).await?;

        let mut response = surf::get(format!(
            "{}{}",
            *ZED_SERVER_URL,
            &rest::GET_RPC_ADDRESS_PATH
        ))
        .header(
            "Authorization",
            http_auth_basic::Credentials::new(&user_id, &access_token).as_http_header(),
        )
        .await
        .context("rpc address request failed")?;

        let rest::GetRpcAddressResponse { address } = response
            .body_json()
            .await
            .context("failed to parse rpc address response")?;

        // TODO - If the `ZED_SERVER_URL` uses https, then wrap this stream in
        // a TLS stream using `native-tls`.
        let stream = smol::net::TcpStream::connect(&address).await?;
        log::info!("connected to rpc address {}", address);

        let connection_id = self.peer.add_connection(stream).await;
        executor
            .spawn(self.peer.handle_messages(connection_id))
            .detach();

        let auth_response = self
            .peer
            .request(
                connection_id,
                proto::Auth {
                    user_id: user_id.parse()?,
                    access_token,
                },
            )
            .await
            .context("rpc auth request failed")?;
        if !auth_response.credentials_valid {
            Err(anyhow!("failed to authenticate with RPC server"))?;
        }

        Ok(connection_id)
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
                zed_rpc::auth::keypair().expect("failed to generate keypair for auth");
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

    pub fn send<T: EnvelopedMessage>(
        &self,
        connection_id: ConnectionId,
        message: T,
    ) -> impl Future<Output = Result<()>> {
        self.peer.send(connection_id, message)
    }

    pub fn request<T: RequestMessage>(
        &self,
        connection_id: ConnectionId,
        request: T,
    ) -> impl Future<Output = Result<T::Response>> {
        self.peer.request(connection_id, request)
    }

    pub fn respond<T: RequestMessage>(
        &self,
        receipt: Receipt<T>,
        response: T::Response,
    ) -> impl Future<Output = Result<()>> {
        self.peer.respond(receipt, response)
    }
}

pub trait MessageHandler<'a, M: proto::EnvelopedMessage> {
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
    F: Fn(TypedEnvelope<M>, &'a Client, &'a mut gpui::AsyncAppContext) -> Fut,
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
    let path = url.strip_prefix(WORKTREE_URL_PREFIX)?;
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
    assert_eq!(decode_worktree_url("not://the-right-format"), None);
}
