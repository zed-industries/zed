use super::installation::{latest_github_release, GitHubLspBinaryVersion};
use anyhow::{anyhow, Result};
use async_trait::async_trait;
use client::http::HttpClient;
use collections::HashMap;
use futures::StreamExt;
use language::{LanguageServerName, LspAdapter};
use serde_json::json;
use smol::fs::{self, File};
use std::{any::Any, env::consts, path::PathBuf, sync::Arc};
use util::ResultExt;

pub struct JsonLspAdapter;

#[async_trait]
impl LspAdapter for JsonLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("json-language-server".into())
    }

    async fn server_args(&self) -> Vec<String> {
        vec!["--stdio".into()]
    }

    async fn fetch_latest_server_version(
        &self,
        http: Arc<dyn HttpClient>,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release = latest_github_release("zed-industries/json-language-server", http).await?;
        let asset_name = format!("json-language-server-darwin-{}", consts::ARCH);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;
        let version = GitHubLspBinaryVersion {
            name: release.name,
            url: asset.browser_download_url.clone(),
        };
        Ok(Box::new(version) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        http: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> Result<PathBuf> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let destination_path = container_dir.join(format!(
            "json-language-server-{}-{}",
            version.name,
            consts::ARCH
        ));
        if fs::metadata(&destination_path).await.is_err() {
            let mut response = http
                .get(&version.url, Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading release: {}", err))?;
            let mut file = File::create(&destination_path).await?;
            futures::io::copy(response.body_mut(), &mut file).await?;
            fs::set_permissions(
                &destination_path,
                <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
            )
            .await?;

            if let Some(mut entries) = fs::read_dir(&container_dir).await.log_err() {
                while let Some(entry) = entries.next().await {
                    if let Some(entry) = entry.log_err() {
                        let entry_path = entry.path();
                        if entry_path.as_path() != destination_path {
                            fs::remove_file(&entry_path).await.log_err();
                        }
                    }
                }
            }
        }

        Ok(destination_path)
    }

    async fn cached_server_binary(&self, container_dir: PathBuf) -> Option<PathBuf> {
        (|| async move {
            let mut last = None;
            let mut entries = fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                last = Some(entry?.path());
            }
            last.ok_or_else(|| anyhow!("no cached binary"))
        })()
        .await
        .log_err()
    }

    async fn initialization_options(&self) -> Option<serde_json::Value> {
        Some(json!({
            "provideFormatter": true
        }))
    }

    async fn language_ids(&self) -> HashMap<String, String> {
        [("JSON".into(), "jsonc".into())].into_iter().collect()
    }
}
