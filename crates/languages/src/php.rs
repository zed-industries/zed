use anyhow::{anyhow, Result};

use async_trait::async_trait;
use collections::HashMap;

use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use node_runtime::NodeRuntime;

use smol::{fs, stream::StreamExt};
use std::{
    any::Any,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{async_maybe, ResultExt};

fn intelephense_server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct IntelephenseVersion(String);

pub struct IntelephenseLspAdapter {
    node: Arc<dyn NodeRuntime>,
}

impl IntelephenseLspAdapter {
    const SERVER_PATH: &'static str = "node_modules/intelephense/lib/intelephense.js";

    pub fn new(node: Arc<dyn NodeRuntime>) -> Self {
        Self { node }
    }
}

#[async_trait]
impl LspAdapter for IntelephenseLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("intelephense".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        Ok(Box::new(IntelephenseVersion(
            self.node.npm_package_latest_version("intelephense").await?,
        )) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<IntelephenseVersion>().unwrap();
        let server_path = container_dir.join(Self::SERVER_PATH);

        if fs::metadata(&server_path).await.is_err() {
            self.node
                .npm_install_packages(&container_dir, &[("intelephense", version.0.as_str())])
                .await?;
        }
        Ok(LanguageServerBinary {
            path: self.node.binary_path().await?,
            env: None,
            arguments: intelephense_server_binary_arguments(&server_path),
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

    async fn label_for_completion(
        &self,
        _item: &lsp::CompletionItem,
        _language: &Arc<language::Language>,
    ) -> Option<language::CodeLabel> {
        None
    }

    fn initialization_options(&self) -> Option<serde_json::Value> {
        None
    }
    fn language_ids(&self) -> HashMap<String, String> {
        HashMap::from_iter([("PHP".into(), "php".into())])
    }

    fn prettier_plugins(&self) -> &[&'static str] {
        &["@prettier/plugin-php"]
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
        let server_path = last_version_dir.join(IntelephenseLspAdapter::SERVER_PATH);
        if server_path.exists() {
            Ok(LanguageServerBinary {
                path: node.binary_path().await?,
                env: None,
                arguments: intelephense_server_binary_arguments(&server_path),
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
