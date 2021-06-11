use anyhow::{anyhow, Context, Result};
use gpui::{AsyncAppContext, MutableAppContext, Task};
use smol::io::{AsyncBufReadExt, AsyncWriteExt};
use std::convert::TryFrom;
use url::Url;
use util::SurfResultExt;

pub mod assets;
pub mod editor;
pub mod file_finder;
pub mod language;
pub mod menus;
mod operation_queue;
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
    cx.spawn::<_, _, surf::Result<()>>(|cx| async move {
        let (user_id, access_token) = login(zed_url.clone(), &cx).await?;

        let mut response = surf::post(format!("{}/api/worktrees", &zed_url))
            .header(
                "Authorization",
                http_auth_basic::Credentials::new(&user_id, &access_token).as_http_header(),
            )
            .await
            .context("")?;

        let body = response
            .body_json::<zed_rpc::rest::CreateWorktreeResponse>()
            .await?;

        // TODO - If the `ZED_SERVER_URL` uses https, then wrap this stream in
        // a TLS stream using `native-tls`.
        let stream = smol::net::TcpStream::connect(body.rpc_address).await?;

        let mut message_stream = zed_rpc::proto::MessageStream::new(stream);
        message_stream
            .write_message(&zed_rpc::proto::FromClient {
                id: 0,
                variant: Some(zed_rpc::proto::from_client::Variant::Auth(
                    zed_rpc::proto::from_client::Auth {
                        user_id: user_id.parse::<i32>()?,
                        access_token,
                    },
                )),
            })
            .await?;

        Ok(())
    })
    .detach();
}

fn login(zed_url: String, cx: &AsyncAppContext) -> Task<Result<(String, String)>> {
    let platform = cx.platform();
    cx.background_executor().spawn(async move {
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

        // Listen on an open TCP port. This port will be used by the web browser to notify the
        // application that the login is complete, and to send the user's id and access token.
        let listener = smol::net::TcpListener::bind("127.0.0.1:0").await?;
        let port = listener.local_addr()?.port();

        // Open the Zed sign-in page in the user's browser, with query parameters that indicate
        // that the user is signing in from a Zed app running on the same device.
        platform.open_url(&format!(
            "{}/sign_in?native_app_port={}&native_app_public_key={}",
            zed_url, port, public_key_string
        ));

        // Receive the HTTP request from the user's browser. Parse the first line, which contains
        // the HTTP method and path.
        let (mut stream, _) = listener.accept().await?;
        let mut reader = smol::io::BufReader::new(&mut stream);
        let mut line = String::new();
        reader.read_line(&mut line).await?;
        let mut parts = line.split(" ");
        let http_method = parts.next();
        if http_method != Some("GET") {
            return Err(anyhow!(
                "unexpected http method {:?} in request from zed web app",
                http_method
            ));
        }
        let path = parts.next().ok_or_else(|| {
            anyhow!("failed to parse http request from zed login redirect - missing path")
        })?;

        // Parse the query parameters from the HTTP request.
        let mut user_id = None;
        let mut access_token = None;
        let url = Url::parse(&format!("http://example.com{}", path))
            .context("failed to parse login notification url")?;
        for (key, value) in url.query_pairs() {
            if key == "access_token" {
                access_token = Some(value);
            } else if key == "user_id" {
                user_id = Some(value);
            }
        }

        // Write an HTTP response to the user's browser, instructing it to close the tab.
        // Then transfer focus back to the application.
        stream
            .write_all(LOGIN_RESPONSE.as_bytes())
            .await
            .context("failed to write login response")?;
        stream.flush().await.context("failed to flush tcp stream")?;
        platform.activate(true);

        // If login succeeded, then store the credentials in the keychain.
        let user_id = user_id.ok_or_else(|| anyhow!("missing user_id in login request"))?;
        let access_token =
            access_token.ok_or_else(|| anyhow!("missing access_token in login request"))?;
        let access_token = private_key
            .decrypt_string(&access_token)
            .context("failed to decrypt access token")?;
        platform.write_credentials(&zed_url, &user_id, access_token.as_bytes());
        log::info!("successfully signed in. user_id: {}", user_id);

        Ok((user_id.to_string(), access_token))
    })
}

fn quit(_: &(), cx: &mut MutableAppContext) {
    cx.platform().quit();
}

const LOGIN_RESPONSE: &'static str = "
HTTP/1.1 200 OK\r
Content-Length: 64\r
Content-Type: text/html\r
\r
<!DOCTYPE html>
<html>
<script>window.close();</script>
</html>
";
