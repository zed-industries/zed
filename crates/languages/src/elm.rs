use anyhow::{anyhow, Result};
use async_trait::async_trait;
use futures::StreamExt;
use gpui::AppContext;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use node_runtime::NodeRuntime;
use project::project_settings::ProjectSettings;
use serde_json::Value;
use settings::Settings;
use smol::fs;
use std::{
    any::Any,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{async_maybe, ResultExt};

const SERVER_NAME: &str = "elm-language-server";
const SERVER_PATH: &str = "node_modules/@elm-tooling/elm-language-server/out/node/index.js";

fn server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct ElmLspAdapter {
    node: Arc<dyn NodeRuntime>,
}

impl ElmLspAdapter {
    pub fn new(node: Arc<dyn NodeRuntime>) -> Self {
        ElmLspAdapter { node }
    }
}

#[async_trait]
impl LspAdapter for ElmLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName(SERVER_NAME.into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Any + Send>> {
        Ok(Box::new(
            self.node
                .npm_package_latest_version("@elm-tooling/elm-language-server")
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
                    &[("@elm-tooling/elm-language-server", version.as_str())],
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

    fn workspace_configuration(&self, _workspace_root: &Path, cx: &mut AppContext) -> Value {
        // elm-language-server expects workspace didChangeConfiguration notification
        // params to be the same as lsp initialization_options
        let override_options = ProjectSettings::get_global(cx)
            .lsp
            .get(SERVER_NAME)
            .and_then(|s| s.initialization_options.clone())
            .unwrap_or_default();

        match override_options.clone().as_object_mut() {
            Some(op) => {
                // elm-language-server requests workspace configuration
                // for the `elmLS` section, so we have to nest
                // another copy of initialization_options there
                op.insert("elmLS".into(), override_options);
                serde_json::to_value(op).unwrap_or_default()
            }
            None => override_options,
        }
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
