use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use collections::HashMap;
use futures::{future::BoxFuture, FutureExt, StreamExt};
use gpui::MutableAppContext;
use language::{LanguageRegistry, LanguageServerBinary, LanguageServerName, LspAdapter};
use node_runtime::NodeRuntime;
use serde_json::json;
use settings::{keymap_file_json_schema, settings_file_json_schema};
use smol::fs;
use std::{
    any::Any,
    ffi::OsString,
    future,
    path::{Path, PathBuf},
    sync::Arc,
};
use theme::ThemeRegistry;
use util::{fs::remove_matching, http::HttpClient};
use util::{paths, ResultExt, StaffMode};

const SERVER_PATH: &'static str =
    "node_modules/vscode-json-languageserver/bin/vscode-json-languageserver";

fn server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct JsonLspAdapter {
    node: Arc<NodeRuntime>,
    languages: Arc<LanguageRegistry>,
    themes: Arc<ThemeRegistry>,
}

impl JsonLspAdapter {
    pub fn new(
        node: Arc<NodeRuntime>,
        languages: Arc<LanguageRegistry>,
        themes: Arc<ThemeRegistry>,
    ) -> Self {
        JsonLspAdapter {
            node,
            languages,
            themes,
        }
    }
}

#[async_trait]
impl LspAdapter for JsonLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("json-language-server".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: Arc<dyn HttpClient>,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(
            self.node
                .npm_package_latest_version("vscode-json-languageserver")
                .await?,
        ) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        _: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<String>().unwrap();
        let version_dir = container_dir.join(version.as_str());
        fs::create_dir_all(&version_dir)
            .await
            .context("failed to create version directory")?;
        let server_path = version_dir.join(SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            self.node
                .npm_install_packages(
                    [("vscode-json-languageserver", version.as_str())],
                    &version_dir,
                )
                .await?;

            remove_matching(&container_dir, |entry| entry != server_path).await;
        }

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            arguments: server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(&self, container_dir: PathBuf) -> Option<LanguageServerBinary> {
        (|| async move {
            let mut last_version_dir = None;
            let mut entries = fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                let entry = entry?;
                if entry.file_type().await?.is_dir() {
                    last_version_dir = Some(entry.path());
                }
            }

            let last_version_dir = last_version_dir.ok_or_else(|| anyhow!("no cached binary"))?;
            let server_path = last_version_dir.join(SERVER_PATH);
            if server_path.exists() {
                Ok(LanguageServerBinary {
                    path: self.node.binary_path().await?,
                    arguments: server_binary_arguments(&server_path),
                })
            } else {
                Err(anyhow!(
                    "missing executable in directory {:?}",
                    last_version_dir
                ))
            }
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
