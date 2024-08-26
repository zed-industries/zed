use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::StreamExt;
use gpui::AsyncAppContext;
use language::{
    language_settings::all_language_settings, LanguageServerName, LspAdapter, LspAdapterDelegate,
};
use lsp::LanguageServerBinary;
use node_runtime::{NodeAssetVersion, NodeRuntime};
use serde_json::Value;
use smol::fs;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{maybe, AssetVersion, ResultExt};

const SERVER_PATH: &str = "node_modules/yaml-language-server/bin/yaml-language-server";

fn server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct YamlLspAdapter {
    node: Arc<dyn NodeRuntime>,
}

impl YamlLspAdapter {
    pub fn new(node: Arc<dyn NodeRuntime>) -> Self {
        YamlLspAdapter { node }
    }
}

#[async_trait(?Send)]
impl LspAdapter for YamlLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("yaml-language-server".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn AssetVersion>> {
        Ok(Box::new(
            self.node
                .npm_package_latest_version("yaml-language-server")
                .await?,
        ) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        latest_version: Box<dyn AssetVersion>,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let latest_version = &latest_version.as_any().downcast_ref::<String>().unwrap();
        let server_path = container_dir.join(SERVER_PATH);
        let package_name = "yaml-language-server";

        let node_asset = NodeAssetVersion {
            name: package_name.to_string(),
            version: latest_version.to_string(),
        };

        let should_install_language_server = self
            .node
            .should_install_npm_package(&node_asset, &server_path, &container_dir)
            .await;

        if should_install_language_server {
            self.node
                .npm_install_packages(&container_dir, &[node_asset])
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

    async fn workspace_configuration(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<Value> {
        let tab_size = cx.update(|cx| {
            all_language_settings(None, cx)
                .language(Some("YAML"))
                .tab_size
        })?;

        Ok(serde_json::json!({
            "yaml": {
                "keyOrdering": false
            },
            "[yaml]": {
                "editor.tabSize": tab_size
            }
        }))
    }
}

async fn get_cached_server_binary(
    container_dir: PathBuf,
    node: &dyn NodeRuntime,
) -> Option<LanguageServerBinary> {
    maybe!(async {
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
