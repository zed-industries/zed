use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use language::{LanguageServerBinary, LanguageServerName, LspAdapter};
use node_runtime::NodeRuntime;
use serde_json::json;
use smol::fs;
use std::ffi::OsString;
use std::path::Path;
use std::{any::Any, path::PathBuf, sync::Arc};
use util::fs::remove_matching;
use util::http::HttpClient;
use util::ResultExt;

fn server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct HtmlLspAdapter {
    node: Arc<NodeRuntime>,
}

impl HtmlLspAdapter {
    const SERVER_PATH: &'static str =
        "node_modules/vscode-langservers-extracted/bin/vscode-html-language-server";

    pub fn new(node: Arc<NodeRuntime>) -> Self {
        HtmlLspAdapter { node }
    }
}

#[async_trait]
impl LspAdapter for HtmlLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName("vscode-html-language-server".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: Arc<dyn HttpClient>,
    ) -> Result<Box<dyn 'static + Any + Send>> {
        Ok(Box::new(
            self.node
                .npm_package_latest_version("vscode-langservers-extracted")
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
        let server_path = version_dir.join(Self::SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            self.node
                .npm_install_packages(
                    [("vscode-langservers-extracted", version.as_str())],
                    &version_dir,
                )
                .await?;

            remove_matching(container_dir.as_path(), |entry| entry != version_dir).await;
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
            let server_path = last_version_dir.join(Self::SERVER_PATH);
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
}
