use anyhow::{anyhow, Context, Result};
use client::http::HttpClient;
use futures::{future::BoxFuture, FutureExt, StreamExt};
use language::{LanguageServerName, LspAdapter};
use serde::Deserialize;
use serde_json::json;
use smol::fs;
use std::{any::Any, path::PathBuf, sync::Arc};
use util::{ResultExt, TryFutureExt};

pub struct TypeScriptLspAdapter;

impl TypeScriptLspAdapter {
    const BIN_PATH: &'static str = "node_modules/typescript-language-server/lib/cli.js";
}

struct Versions {
    typescript_version: String,
    server_version: String,
}

impl LspAdapter for TypeScriptLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("typescript-language-server".into())
    }

    fn server_args(&self) -> &[&str] {
        &["--stdio", "--tsserver-path", "node_modules/typescript/lib"]
    }

    fn fetch_latest_server_version(
        &self,
        _: Arc<dyn HttpClient>,
    ) -> BoxFuture<'static, Result<Box<dyn 'static + Send + Any>>> {
        async move {
            #[derive(Deserialize)]
            struct NpmInfo {
                versions: Vec<String>,
            }

            let typescript_output = smol::process::Command::new("npm")
                .args(["info", "typescript", "--json"])
                .output()
                .await?;
            if !typescript_output.status.success() {
                Err(anyhow!("failed to execute npm info"))?;
            }
            let mut typescript_info: NpmInfo = serde_json::from_slice(&typescript_output.stdout)?;

            let server_output = smol::process::Command::new("npm")
                .args(["info", "typescript-language-server", "--json"])
                .output()
                .await?;
            if !server_output.status.success() {
                Err(anyhow!("failed to execute npm info"))?;
            }
            let mut server_info: NpmInfo = serde_json::from_slice(&server_output.stdout)?;

            Ok(Box::new(Versions {
                typescript_version: typescript_info
                    .versions
                    .pop()
                    .ok_or_else(|| anyhow!("no versions found in typescript npm info"))?,
                server_version: server_info.versions.pop().ok_or_else(|| {
                    anyhow!("no versions found in typescript language server npm info")
                })?,
            }) as Box<_>)
        }
        .boxed()
    }

    fn fetch_server_binary(
        &self,
        versions: Box<dyn 'static + Send + Any>,
        _: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> BoxFuture<'static, Result<PathBuf>> {
        let versions = versions.downcast::<Versions>().unwrap();
        async move {
            let version_dir = container_dir.join(&format!(
                "typescript-{}:server-{}",
                versions.typescript_version, versions.server_version
            ));
            fs::create_dir_all(&version_dir)
                .await
                .context("failed to create version directory")?;
            let binary_path = version_dir.join(Self::BIN_PATH);

            if fs::metadata(&binary_path).await.is_err() {
                let output = smol::process::Command::new("npm")
                    .current_dir(&version_dir)
                    .arg("install")
                    .arg(format!("typescript@{}", versions.typescript_version))
                    .arg(format!(
                        "typescript-language-server@{}",
                        versions.server_version
                    ))
                    .output()
                    .await
                    .context("failed to run npm install")?;
                if !output.status.success() {
                    Err(anyhow!("failed to install typescript-language-server"))?;
                }

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

    fn cached_server_binary(&self, container_dir: PathBuf) -> BoxFuture<'static, Option<PathBuf>> {
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

    fn process_diagnostics(&self, _: &mut lsp::PublishDiagnosticsParams) {}

    fn initialization_options(&self) -> Option<serde_json::Value> {
        Some(json!({
            "provideFormatter": true
        }))
    }
}
