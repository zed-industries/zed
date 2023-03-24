use super::installation::{latest_github_release, GitHubLspBinaryVersion};
use anyhow::{anyhow, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_trait::async_trait;
use client::http::HttpClient;
use collections::HashMap;
use futures::{future::BoxFuture, io::BufReader, FutureExt, StreamExt};
use gpui::MutableAppContext;
use language::{
    LanguageRegistry, LanguageServerBinary, LanguageServerName, LspAdapter, ServerExecutionKind,
};
use serde_json::json;
use settings::{keymap_file_json_schema, settings_file_json_schema};
use smol::fs::{self, File};
use std::{
    any::Any,
    env::consts,
    future,
    path::{Path, PathBuf},
    sync::Arc,
};
use theme::ThemeRegistry;
use util::{paths, ResultExt, StaffMode};

fn server_binary_arguments() -> Vec<String> {
    vec!["--stdio".into()]
}

pub struct JsonLspAdapter {
    languages: Arc<LanguageRegistry>,
    themes: Arc<ThemeRegistry>,
}

impl JsonLspAdapter {
    pub fn new(languages: Arc<LanguageRegistry>, themes: Arc<ThemeRegistry>) -> Self {
        Self { languages, themes }
    }
}

#[async_trait]
impl LspAdapter for JsonLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("json-language-server".into())
    }

    async fn server_execution_kind(&self) -> ServerExecutionKind {
        ServerExecutionKind::Node
    }

    async fn fetch_latest_server_version(
        &self,
        http: Arc<dyn HttpClient>,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release = latest_github_release("zed-industries/json-language-server", http).await?;
        let asset_name = format!("json-language-server-darwin-{}.gz", consts::ARCH);
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
    ) -> Result<LanguageServerBinary> {
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
            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let mut file = File::create(&destination_path).await?;
            futures::io::copy(decompressed_bytes, &mut file).await?;
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

        Ok(LanguageServerBinary {
            path: destination_path,
            arguments: server_binary_arguments(),
        })
    }

    async fn cached_server_binary(&self, container_dir: PathBuf) -> Option<LanguageServerBinary> {
        (|| async move {
            let mut last = None;
            let mut entries = fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                last = Some(entry?.path());
            }
            anyhow::Ok(LanguageServerBinary {
                path: last.ok_or_else(|| anyhow!("no cached binary"))?,
                arguments: server_binary_arguments(),
            })
        })()
        .await
        .log_err()
    }

    async fn initialization_options(&self) -> Option<serde_json::Value> {
        Some(json!({
            "provideFormatter": true
        }))
    }

    fn workspace_configuration(
        &self,
        cx: &mut MutableAppContext,
    ) -> Option<BoxFuture<'static, serde_json::Value>> {
        let action_names = cx.all_action_names().collect::<Vec<_>>();
        let theme_names = self
            .themes
            .list(**cx.default_global::<StaffMode>())
            .map(|meta| meta.name)
            .collect();
        let language_names = self.languages.language_names();
        Some(
            future::ready(serde_json::json!({
                "json": {
                    "format": {
                        "enable": true,
                    },
                    "schemas": [
                        {
                            "fileMatch": [schema_file_match(&paths::SETTINGS)],
                            "schema": settings_file_json_schema(theme_names, &language_names),
                        },
                        {
                            "fileMatch": [schema_file_match(&paths::KEYMAP)],
                            "schema": keymap_file_json_schema(&action_names),
                        }
                    ]
                }
            }))
            .boxed(),
        )
    }

    async fn language_ids(&self) -> HashMap<String, String> {
        [("JSON".into(), "jsonc".into())].into_iter().collect()
    }
}

fn schema_file_match(path: &Path) -> &Path {
    path.strip_prefix(path.parent().unwrap().parent().unwrap())
        .unwrap()
}
