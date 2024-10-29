use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::StreamExt;
use gpui::AsyncAppContext;
use language::{
    language_settings::AllLanguageSettings, LanguageServerName, LanguageToolchainStore, LspAdapter,
    LspAdapterDelegate,
};
use lsp::LanguageServerBinary;
use node_runtime::NodeRuntime;
use project::lsp_store::language_server_settings;
use serde_json::Value;
use settings::{Settings, SettingsLocation};
use smol::fs;
use std::{
    any::Any,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{maybe, merge_json_value_into, ResultExt};

const SERVER_PATH: &str = "node_modules/yaml-language-server/bin/yaml-language-server";

fn server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct YamlLspAdapter {
    node: NodeRuntime,
}

impl YamlLspAdapter {
    const SERVER_NAME: LanguageServerName = LanguageServerName::new_static("yaml-language-server");
    pub fn new(node: NodeRuntime) -> Self {
        YamlLspAdapter { node }
    }
}

#[async_trait(?Send)]
impl LspAdapter for YamlLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME.clone()
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
        get_cached_server_binary(container_dir, &self.node).await
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
        _: Arc<dyn LanguageToolchainStore>,
        cx: &mut AsyncAppContext,
    ) -> Result<Value> {
        let location = SettingsLocation {
            worktree_id: delegate.worktree_id(),
            path: delegate.worktree_root_path(),
        };

        let tab_size = cx.update(|cx| {
            AllLanguageSettings::get(Some(location), cx)
                .language(Some(location), Some(&"YAML".into()), cx)
                .tab_size
        })?;
        let mut options = serde_json::json!({"[yaml]": {"editor.tabSize": tab_size}});

        let project_options = cx.update(|cx| {
            language_server_settings(delegate.as_ref(), &Self::SERVER_NAME, cx)
                .and_then(|s| s.settings.clone())
        })?;
        if let Some(override_options) = project_options {
            merge_json_value_into(override_options, &mut options);
        }
        Ok(options)
    }
}

async fn get_cached_server_binary(
    container_dir: PathBuf,
    node: &NodeRuntime,
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
