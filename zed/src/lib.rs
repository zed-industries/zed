use std::convert::TryInto;

use anyhow::{anyhow, Context};
use gpui::MutableAppContext;
use smol::io::{AsyncBufReadExt, AsyncWriteExt};
use url::Url;

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
    cx.add_global_action("app:authenticate", authenticate);
    cx.add_global_action("app:quit", quit);
}

fn authenticate(_: &(), cx: &mut MutableAppContext) {
    let zed_url = std::env::var("ZED_SERVER_URL").unwrap_or("https://zed.dev".to_string());
    let platform = cx.platform().clone();

    let task = cx.background_executor().spawn(async move {
        let listener = smol::net::TcpListener::bind("127.0.0.1:0").await?;
        let port = listener.local_addr()?.port();

        let (public_key, private_key) =
            zed_rpc::auth::keypair().expect("failed to generate keypair for auth");

        let public_key_string: String = public_key.try_into().unwrap();

        platform.open_url(&format!(
            "{}/sign_in?native_app_port={}&native_app_public_key={}",
            zed_url, port, public_key_string
        ));

        let (mut stream, _) = listener.accept().await?;
        let mut reader = smol::io::BufReader::new(&mut stream);
        let mut line = String::new();
        reader.read_line(&mut line).await?;

        let mut parts = line.split(" ");
        if parts.next() == Some("GET") {
            if let Some(path) = parts.next() {
                let url = Url::parse(&format!("http://example.com{}", path))
                    .context("failed to parse login notification url")?;
                let mut user_id = None;
                let mut access_token = None;
                for (key, value) in url.query_pairs() {
                    if key == "access_token" {
                        access_token = Some(value);
                    } else if key == "user_id" {
                        user_id = Some(value);
                    }
                }
                stream
                    .write_all(LOGIN_RESPONSE.as_bytes())
                    .await
                    .context("failed to write login response")?;
                stream.flush().await.context("failed to flush tcp stream")?;

                if let Some((user_id, access_token)) = user_id.zip(access_token) {
                    let access_token = private_key.decrypt_string(&access_token);
                    eprintln!(
                        "logged in. user_id: {}, access_token: {:?}",
                        user_id, access_token
                    );
                }

                platform.activate(true);
                return Ok(());
            }
        }
        Err(anyhow!("failed to parse http request from zed web app"))
    });

    cx.spawn(|_| async move {
        if let Err(e) = task.await {
            log::error!("failed to login {:?}", e)
        }
    })
    .detach();
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
