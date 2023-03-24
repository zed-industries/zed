use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use client::http::HttpClient;
use futures::{future::BoxFuture, FutureExt, StreamExt};
use gpui::MutableAppContext;
use language::{LanguageServerBinary, LanguageServerName, LspAdapter, ServerExecutionKind};
use serde_json::Value;
use settings::Settings;
use smol::fs;
use std::{any::Any, future, path::PathBuf, sync::Arc};
use util::ResultExt;

use super::installation::{npm_install_packages, npm_package_latest_version};

fn server_binary_arguments() -> Vec<String> {
    vec!["--stdio".into()]
}

pub struct YamlLspAdapter;

impl YamlLspAdapter {
    const BIN_PATH: &'static str = "node_modules/yaml-language-server/bin/yaml-language-server";
}

#[async_trait]
impl LspAdapter for YamlLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("yaml-language-server".into())
    }

    async fn server_execution_kind(&self) -> ServerExecutionKind {
        ServerExecutionKind::Node
    }

    async fn fetch_latest_server_version(
        &self,
        http: Arc<dyn HttpClient>,
    ) -> Result<Box<dyn 'static + Any + Send>> {
        Ok(Box::new(npm_package_latest_version(http, "yaml-language-server").await?) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        http: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<String>().unwrap();
        let version_dir = container_dir.join(version.as_str());
        fs::create_dir_all(&version_dir)
            .await
            .context("failed to create version directory")?;
        let binary_path = version_dir.join(Self::BIN_PATH);

        if fs::metadata(&binary_path).await.is_err() {
            npm_install_packages(
                http,
                [("yaml-language-server", version.as_str())],
                &version_dir,
            )
            .await?;

            if let Some(mut entries) = fs::read_dir(&container_dir).await.log_err() {
                while let Some(entry) = entries.next().await {
                    if let Some(entry) = entry.log_err() {
                        let entry_path = entry.path();
                        if entry_path.as_path() != version_dir {
                            fs::remove_dir_all(&entry_path).await.log_err();
                        }
                    }
                }
            }
        }

        Ok(LanguageServerBinary {
            path: binary_path,
            arguments: server_binary_arguments(),
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
            let bin_path = last_version_dir.join(Self::BIN_PATH);
            if bin_path.exists() {
                Ok(LanguageServerBinary {
                    path: bin_path,
                    arguments: server_binary_arguments(),
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

    fn workspace_configuration(
        &self,
        cx: &mut MutableAppContext,
    ) -> Option<BoxFuture<'static, Value>> {
        let settings = cx.global::<Settings>();
        Some(
            future::ready(serde_json::json!({
                "[yaml]": {
                    "editor.tabSize": settings.tab_size(Some("YAML"))
                }
            }))
            .boxed(),
        )
    }
}
