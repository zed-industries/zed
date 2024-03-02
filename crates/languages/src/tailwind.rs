use anyhow::{anyhow, Result};
use async_trait::async_trait;
use collections::HashMap;
use futures::StreamExt;
use gpui::AppContext;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use node_runtime::NodeRuntime;
use serde_json::{json, Value};
use smol::fs;
use std::{
    any::Any,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{async_maybe, ResultExt};

const SERVER_PATH: &str = "node_modules/.bin/tailwindcss-language-server";

fn server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct TailwindLspAdapter {
    node: Arc<dyn NodeRuntime>,
}

impl TailwindLspAdapter {
    pub fn new(node: Arc<dyn NodeRuntime>) -> Self {
        TailwindLspAdapter { node }
    }
}

#[async_trait]
impl LspAdapter for TailwindLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("tailwindcss-language-server".into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Any + Send>> {
        Ok(Box::new(
            self.node
                .npm_package_latest_version("@tailwindcss/language-server")
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
                    &[("@tailwindcss/language-server", version.as_str())],
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
            "provideFormatter": true,
            "userLanguages": {
                "html": "html",
                "css": "css",
                "javascript": "javascript",
                "typescriptreact": "typescriptreact",
            },
        }))
    }

    fn workspace_configuration(&self, _workspace_root: &Path, _: &mut AppContext) -> Value {
        json!({
            "tailwindCSS": {
                "emmetCompletions": true,
            }
        })
    }

    fn language_ids(&self) -> HashMap<String, String> {
        HashMap::from_iter([
            ("Astro".to_string(), "astro".to_string()),
            ("HTML".to_string(), "html".to_string()),
            ("CSS".to_string(), "css".to_string()),
            ("JavaScript".to_string(), "javascript".to_string()),
            ("TSX".to_string(), "typescriptreact".to_string()),
            ("Svelte".to_string(), "svelte".to_string()),
            ("Elixir".to_string(), "phoenix-heex".to_string()),
            ("HEEX".to_string(), "phoenix-heex".to_string()),
            ("ERB".to_string(), "erb".to_string()),
            ("PHP".to_string(), "php".to_string()),
        ])
    }

    fn prettier_plugins(&self) -> &[&'static str] {
        &["prettier-plugin-tailwindcss"]
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
