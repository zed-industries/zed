use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::StreamExt;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use node_runtime::NodeRuntime;
use serde_json::json;
use smol::fs;
use std::{
    any::Any,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{async_maybe, ResultExt};

const SERVER_PATH: &str =
    "node_modules/vscode-langservers-extracted/bin/vscode-css-language-server";

fn server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct CssLspAdapter {
    node: Arc<dyn NodeRuntime>,
}

impl CssLspAdapter {
    pub fn new(node: Arc<dyn NodeRuntime>) -> Self {
        CssLspAdapter { node }
    }
}

#[async_trait]
impl LspAdapter for CssLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("vscode-css-language-server".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
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
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<String>().unwrap();
        let server_path = container_dir.join(SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            self.node
                .npm_install_packages(
                    &container_dir,
                    &[("vscode-langservers-extracted", version.as_str())],
                )
                .await?;
        }

        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: None,
            arguments: server_binary_arguments(&server_path),
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir, &*self.node).await
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir, &*self.node).await
    }

    fn initialization_options(&self) -> Option<serde_json::Value> {
        Some(json!({
            "provideFormatter": true
        }))
    }
}

async fn get_cached_server_binary(
    container_dir: PathBuf,
    node: &dyn NodeRuntime,
) -> Option<LanguageServerBinary> {
    async_maybe!({
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
                path: node.binary_path().await?,
                env: None,
                arguments: server_binary_arguments(&server_path),
            })
        } else {
            Err(anyhow!(
                "missing executable in directory {:?}",
                last_version_dir
            ))
        }
    })
    .await
    .log_err()
}
