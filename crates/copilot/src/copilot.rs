use anyhow::{anyhow, Ok};
use async_compression::futures::bufread::GzipDecoder;
use client::Client;
use gpui::{actions, MutableAppContext};
use smol::{fs, io::BufReader, stream::StreamExt};
use std::{env::consts, path::PathBuf, sync::Arc};
use util::{
    fs::remove_matching, github::latest_github_release, http::HttpClient, paths, ResultExt,
};

actions!(copilot, [SignIn]);

pub fn init(client: Arc<Client>, cx: &mut MutableAppContext) {
    cx.add_global_action(move |_: &SignIn, cx: &mut MutableAppContext| {
        Copilot::sign_in(client.http_client(), cx)
    });
}

#[derive(Debug)]
struct Copilot {
    copilot_server: PathBuf,
}

impl Copilot {
    fn sign_in(http: Arc<dyn HttpClient>, cx: &mut MutableAppContext) {
        let maybe_copilot = cx.default_global::<Option<Arc<Copilot>>>().clone();

        cx.spawn(|mut cx| async move {
            // Lazily download / initialize copilot LSP
            let copilot = if let Some(copilot) = maybe_copilot {
                copilot
            } else {
                let copilot_server = get_lsp_binary(http).await?; // TODO: Make this error user visible
                let new_copilot = Arc::new(Copilot { copilot_server });
                cx.update({
                    let new_copilot = new_copilot.clone();
                    move |cx| cx.set_global(Some(new_copilot.clone()))
                });
                new_copilot
            };

            dbg!(copilot);

            Ok(())
        })
        .detach();
    }
}

async fn get_lsp_binary(http: Arc<dyn HttpClient>) -> anyhow::Result<PathBuf> {
    ///Check for the latest copilot language server and download it if we haven't already
    async fn fetch_latest(http: Arc<dyn HttpClient>) -> anyhow::Result<PathBuf> {
        let release = latest_github_release("zed-industries/copilotserver", http.clone()).await?;
        let asset_name = format!("copilot-darwin-{}.gz", consts::ARCH);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;

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
