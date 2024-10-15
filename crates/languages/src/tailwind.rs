use anyhow::{anyhow, Result};
use async_trait::async_trait;
use collections::HashMap;
use futures::StreamExt;
use gpui::AsyncAppContext;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use node_runtime::NodeRuntime;
use project::lsp_store::language_server_settings;
use serde_json::{json, Value};
use smol::fs;
use std::{
    any::Any,
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{maybe, ResultExt};

#[cfg(target_os = "windows")]
const SERVER_PATH: &str =
    "node_modules/@tailwindcss/language-server/bin/tailwindcss-language-server";
#[cfg(not(target_os = "windows"))]
const SERVER_PATH: &str = "node_modules/.bin/tailwindcss-language-server";

fn server_binary_arguments(server_path: &Path) -> Vec<OsString> {
    vec![server_path.into(), "--stdio".into()]
}

pub struct TailwindLspAdapter {
    node: NodeRuntime,
}

impl TailwindLspAdapter {
    const SERVER_NAME: LanguageServerName =
        LanguageServerName::new_static("tailwindcss-language-server");

    pub fn new(node: NodeRuntime) -> Self {
        TailwindLspAdapter { node }
    }
}

#[async_trait(?Send)]
impl LspAdapter for TailwindLspAdapter {
    fn name(&self) -> LanguageServerName {
        Self::SERVER_NAME.clone()
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
        latest_version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let latest_version = latest_version.downcast::<String>().unwrap();
        let server_path = container_dir.join(SERVER_PATH);
        let package_name = "@tailwindcss/language-server";

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

    async fn initialization_options(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(json!({
            "provideFormatter": true,
            "userLanguages": {
                "html": "html",
                "css": "css",
                "javascript": "javascript",
                "typescriptreact": "typescriptreact",
            },
        })))
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        delegate: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<Value> {
        let tailwind_user_settings = cx.update(|cx| {
            language_server_settings(delegate.as_ref(), &Self::SERVER_NAME, cx)
                .and_then(|s| s.settings.clone())
                .unwrap_or_default()
        })?;

        let mut configuration = json!({
            "tailwindCSS": {
                "emmetCompletions": true,
            }
        });

        if let Some(experimental) = tailwind_user_settings.get("experimental").cloned() {
            configuration["tailwindCSS"]["experimental"] = experimental;
        }

        if let Some(class_attributes) = tailwind_user_settings.get("classAttributes").cloned() {
            configuration["tailwindCSS"]["classAttributes"] = class_attributes;
        }

        if let Some(include_languages) = tailwind_user_settings.get("includeLanguages").cloned() {
            configuration["tailwindCSS"]["includeLanguages"] = include_languages;
        }

        Ok(configuration)
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
            ("Vue.js".to_string(), "vue".to_string()),
        ])
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
