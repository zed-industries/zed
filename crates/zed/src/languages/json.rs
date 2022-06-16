use super::installation::{npm_install_packages, npm_package_latest_version};
use anyhow::{anyhow, Context, Result};
use client::http::HttpClient;
use futures::{future::BoxFuture, FutureExt, StreamExt};
use language::{LanguageServerName, LspAdapter};
use serde_json::json;
use smol::fs;
use std::{
    any::Any,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{ResultExt, TryFutureExt};

pub struct JsonLspAdapter;

impl JsonLspAdapter {
    const BIN_PATH: &'static str =
        "node_modules/vscode-json-languageserver/bin/vscode-json-languageserver";
}

impl LspAdapter for JsonLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("vscode-json-languageserver".into())
    }

    fn server_args(&self) -> &[&str] {
        &["--stdio"]
    }

    fn fetch_latest_server_version(
        &self,
        _: Arc<dyn HttpClient>,
    ) -> BoxFuture<'static, Result<Box<dyn 'static + Any + Send>>> {
        async move {
            Ok(Box::new(npm_package_latest_version("vscode-json-languageserver").await?) as Box<_>)
        }
        .boxed()
    }

    fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        _: Arc<dyn HttpClient>,
        container_dir: Arc<Path>,
    ) -> BoxFuture<'static, Result<PathBuf>> {
        let version = version.downcast::<String>().unwrap();
        async move {
            let version_dir = container_dir.join(version.as_str());
            fs::create_dir_all(&version_dir)
                .await
                .context("failed to create version directory")?;
            let binary_path = version_dir.join(Self::BIN_PATH);

            if fs::metadata(&binary_path).await.is_err() {
                npm_install_packages(
                    [("vscode-json-languageserver", version.as_str())],
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

            Ok(binary_path)
        }
        .boxed()
    }

    fn cached_server_binary(
        &self,
        container_dir: Arc<Path>,
    ) -> BoxFuture<'static, Option<PathBuf>> {
        async move {
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
                Ok(bin_path)
            } else {
                Err(anyhow!(
                    "missing executable in directory {:?}",
                    last_version_dir
                ))
            }
        }
        .log_err()
        .boxed()
    }

    fn initialization_options(&self) -> Option<serde_json::Value> {
        Some(json!({
            "provideFormatter": true
        }))
    }

    fn id_for_language(&self, name: &str) -> Option<String> {
        if name == "JSON" {
            Some("jsonc".into())
        } else {
            None
        }
    }
}
