use anyhow::{anyhow, Context, Result};
use gpui::{AsyncAppContext, MutableAppContext, Task};
use rpc_client::RpcClient;
use std::{convert::TryFrom, net::Shutdown, time::Duration};
use tiny_http::{Header, Response, Server};
use url::Url;
use util::SurfResultExt;
use zed_rpc::{proto, rest::CreateWorktreeResponse};

pub mod assets;
pub mod editor;
pub mod file_finder;
pub mod language;
pub mod menus;
mod operation_queue;
mod rpc_client;
pub mod settings;
mod sum_tree;
#[cfg(test)]
mod test;
mod time;
mod util;
pub mod workspace;
mod worktree;

#[derive(Clone)]
pub struct AppState {
    pub settings: postage::watch::Receiver<settings::Settings>,
    pub language_registry: std::sync::Arc<language::LanguageRegistry>,
}

pub fn init(cx: &mut MutableAppContext) {
    cx.add_global_action("app:share_worktree", share_worktree);
    cx.add_global_action("app:quit", quit);
}

fn share_worktree(_: &(), cx: &mut MutableAppContext) {
    let zed_url = std::env::var("ZED_SERVER_URL").unwrap_or("https://zed.dev".to_string());
    let executor = cx.background_executor().clone();

    let task = cx.spawn::<_, _, surf::Result<()>>(|cx| async move {
        let (user_id, access_token) = login(zed_url.clone(), &cx).await?;

        let mut response = surf::post(format!("{}/api/worktrees", &zed_url))
            .header(
                "Authorization",
                http_auth_basic::Credentials::new(&user_id, &access_token).as_http_header(),
            )
            .await
            .context("")?;

        let CreateWorktreeResponse {
            worktree_id,
            rpc_address,
        } = response.body_json().await?;

        eprintln!("got worktree response: {:?} {:?}", worktree_id, rpc_address);

        // TODO - If the `ZED_SERVER_URL` uses https, then wrap this stream in
        // a TLS stream using `native-tls`.
        let stream = smol::net::TcpStream::connect(rpc_address).await?;

        let mut rpc_client = RpcClient::new(stream, executor, |stream| {
            stream.shutdown(Shutdown::Read).ok();
        });

        let auth_response = rpc_client
            .request(proto::from_client::Auth {
                user_id: user_id.parse::<i32>()?,
                access_token,
            })
            .await?;
        if !auth_response.credentials_valid {
            Err(anyhow!("failed to authenticate with RPC server"))?;
        }

        let share_response = rpc_client
            .request(proto::from_client::ShareWorktree {
                worktree_id: worktree_id as u64,
                files: Vec::new(),
            })
            .await?;

        log::info!("sharing worktree {:?}", share_response);

        Ok(())
    });

    cx.spawn(|_| async move {
        if let Err(e) = task.await {
            log::error!("sharing failed: {}", e);
        }
    })
    .detach();
}

fn login(zed_url: String, cx: &AsyncAppContext) -> Task<Result<(String, String)>> {
    let platform = cx.platform();
    let executor = cx.background_executor();
    executor.clone().spawn(async move {
        if let Some((user_id, access_token)) = platform.read_credentials(&zed_url) {
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
        let server = Server::http("127.0.0.1:0").expect("failed to find open port");
        let port = server.server_addr().port();

        // Open the Zed sign-in page in the user's browser, with query parameters that indicate
        // that the user is signing in from a Zed app running on the same device.
        platform.open_url(&format!(
            "{}/sign_in?native_app_port={}&native_app_public_key={}",
            zed_url, port, public_key_string
        ));

        // Receive the HTTP request from the user's browser. Retrieve the user id and encrypted
        // access token from the query params.
        //
        // TODO - Avoid ever starting more than one HTTP server. Maybe switch to using a
        // custom URL scheme instead of this local HTTP server.
        let (user_id, access_token) = executor
            .spawn::<anyhow::Result<_>, _>(async move {
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
                        Response::from_string(LOGIN_RESPONSE)
                            .with_header(Header::from_bytes("Content-Type", "text/html").unwrap()),
                    )
                    .context("failed to respond to login http request")?;
                    Ok(user_id.zip(access_token))
                } else {
                    Ok(None)
                }
            })
            .await?
            .ok_or_else(|| anyhow!(""))?;

        let access_token = private_key
            .decrypt_string(&access_token)
            .context("failed to decrypt access token")?;
        platform.activate(true);
        platform.write_credentials(&zed_url, &user_id, access_token.as_bytes());
        Ok((user_id.to_string(), access_token))
    })
}

fn quit(_: &(), cx: &mut MutableAppContext) {
    cx.platform().quit();
}

const LOGIN_RESPONSE: &'static str = "
<!DOCTYPE html>
<html>
<script>window.close();</script>
</html>
";
