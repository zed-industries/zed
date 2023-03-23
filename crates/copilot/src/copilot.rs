mod request;

use anyhow::{anyhow, Result};
use async_compression::futures::bufread::GzipDecoder;
use client::Client;
use gpui::{actions, AppContext, Entity, ModelContext, ModelHandle, MutableAppContext};
use lsp::LanguageServer;
use smol::{fs, io::BufReader, stream::StreamExt};
use std::{
    env::consts,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{
    fs::remove_matching, github::latest_github_release, http::HttpClient, paths, ResultExt,
};

actions!(copilot, [SignIn]);

pub fn init(client: Arc<Client>, cx: &mut MutableAppContext) {
    let copilot = cx.add_model(|cx| Copilot::start(client.http_client(), cx));
    cx.set_global(copilot);
}

enum CopilotServer {
    Downloading,
    Error(String),
    Started {
        server: Arc<LanguageServer>,
        status: SignInStatus,
    },
}

enum SignInStatus {
    Authorized,
    Unauthorized,
    SignedOut,
}

struct Copilot {
    server: CopilotServer,
}

impl Entity for Copilot {
    type Event = ();
}

impl Copilot {
    fn global(cx: &AppContext) -> Option<ModelHandle<Self>> {
        if cx.has_global::<ModelHandle<Self>>() {
            Some(cx.global::<ModelHandle<Self>>().clone())
        } else {
            None
        }
    }

    fn start(http: Arc<dyn HttpClient>, cx: &mut ModelContext<Self>) -> Self {
        let copilot = Self {
            server: CopilotServer::Downloading,
        };
        cx.spawn(|this, mut cx| async move {
            let start_language_server = async {
                let server_path = get_lsp_binary(http).await?;
                let server =
                    LanguageServer::new(0, &server_path, &["--stdio"], Path::new("/"), cx.clone())?;
                let server = server.initialize(Default::default()).await?;
                let status = server
                    .request::<request::CheckStatus>(request::CheckStatusParams {
                        local_checks_only: false,
                    })
                    .await?;
                let status = match status.status.as_str() {
                    "OK" | "MaybeOk" => SignInStatus::Authorized,
                    "NotAuthorized" => SignInStatus::Unauthorized,
                    _ => SignInStatus::SignedOut,
                };
                anyhow::Ok((server, status))
            };

            let server = start_language_server.await;
            this.update(&mut cx, |this, cx| {
                cx.notify();
                match server {
                    Ok((server, status)) => {
                        this.server = CopilotServer::Started { server, status };
                        Ok(())
                    }
                    Err(error) => {
                        this.server = CopilotServer::Error(error.to_string());
                        Err(error)
                    }
                }
            })
        })
        .detach_and_log_err(cx);
        copilot
    }
}

async fn get_lsp_binary(http: Arc<dyn HttpClient>) -> anyhow::Result<PathBuf> {
    ///Check for the latest copilot language server and download it if we haven't already
    async fn fetch_latest(http: Arc<dyn HttpClient>) -> anyhow::Result<PathBuf> {
        let release = latest_github_release("zed-industries/copilot", http.clone()).await?;
        let asset_name = format!("copilot-darwin-{}.gz", consts::ARCH);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;

        fs::create_dir_all(&*paths::COPILOT_DIR).await?;
        let destination_path =
            paths::COPILOT_DIR.join(format!("copilot-{}-{}", release.name, consts::ARCH));

        if fs::metadata(&destination_path).await.is_err() {
            let mut response = http
                .get(&asset.browser_download_url, Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading release: {}", err))?;
            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let mut file = fs::File::create(&destination_path).await?;
            futures::io::copy(decompressed_bytes, &mut file).await?;
            fs::set_permissions(
                &destination_path,
                <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
            )
            .await?;

            remove_matching(&paths::COPILOT_DIR, |entry| entry != destination_path).await;
        }

        Ok(destination_path)
    }

    match fetch_latest(http).await {
        ok @ Result::Ok(..) => ok,
        e @ Err(..) => {
            e.log_err();
            // Fetch a cached binary, if it exists
            (|| async move {
                let mut last = None;
                let mut entries = fs::read_dir(paths::COPILOT_DIR.as_path()).await?;
                while let Some(entry) = entries.next().await {
                    last = Some(entry?.path());
                }
                last.ok_or_else(|| anyhow!("no cached binary"))
            })()
            .await
        }
    }
}
