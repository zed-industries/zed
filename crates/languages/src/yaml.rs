use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::StreamExt;
use gpui::AsyncAppContext;
use language::{
    language_settings::AllLanguageSettings, LanguageServerName, LspAdapter, LspAdapterDelegate,
};
use lsp::LanguageServerBinary;
use node_runtime::NodeRuntime;
use serde_json::Value;
use settings::{Settings, SettingsLocation};
use smol::fs;
use std::{
    any::Any,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{maybe, ResultExt};

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
    ) -> Result<Box<dyn 'static + Any + Send>> {
        Ok(Box::new(
            self.node
                .npm_package_latest_version("yaml-language-server")
                .await?,
        ) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        latest_version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let latest_version = latest_version.downcast::<String>().unwrap();
        let server_path = container_dir.join(SERVER_PATH);
        let package_name = "yaml-language-server";

        let should_install_language_server = self
            .node
            .should_install_npm_package(package_name, &server_path, &container_dir, &latest_version)
            .await;

        if should_install_language_server {
            self.node
                .npm_install_packages(&container_dir, &[(package_name, latest_version.as_str())])
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
        delegate: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<Value> {
        let location = SettingsLocation {
            worktree_id: delegate.worktree_id() as usize,
            path: delegate.worktree_root_path(),
        };

        let tab_size = cx.update(|cx| {
            AllLanguageSettings::get(Some(location), cx)
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
