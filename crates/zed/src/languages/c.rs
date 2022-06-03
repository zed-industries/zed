use super::installation::{latest_github_release, GitHubLspBinaryVersion};
use anyhow::{anyhow, Context, Result};
use client::http::HttpClient;
use futures::{future::BoxFuture, FutureExt, StreamExt};
pub use language::*;
use smol::fs::{self, File};
use std::{any::Any, path::PathBuf, sync::Arc};
use util::{ResultExt, TryFutureExt};

pub struct CLspAdapter;

impl super::LspAdapter for CLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("clangd".into())
    }

    fn fetch_latest_server_version(
        &self,
        http: Arc<dyn HttpClient>,
    ) -> BoxFuture<'static, Result<Box<dyn 'static + Send + Any>>> {
        async move {
            let version = latest_github_release("clangd/clangd", http, |release_name| {
                format!("clangd-mac-{release_name}.zip")
            })
            .await?;
            Ok(Box::new(version) as Box<_>)
        }
        .boxed()
    }

    fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        http: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> BoxFuture<'static, Result<PathBuf>> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        async move {
            let zip_path = container_dir.join(format!("clangd_{}.zip", version.name));
            let version_dir = container_dir.join(format!("clangd_{}", version.name));
            let binary_path = version_dir.join("bin/clangd");

            if fs::metadata(&binary_path).await.is_err() {
                let mut response = http
                    .get(&version.url, Default::default(), true)
                    .await
                    .context("error downloading release")?;
                let mut file = File::create(&zip_path).await?;
                if !response.status().is_success() {
                    Err(anyhow!(
                        "download failed with status {}",
                        response.status().to_string()
                    ))?;
                }
                futures::io::copy(response.body_mut(), &mut file).await?;

                let unzip_status = smol::process::Command::new("unzip")
                    .current_dir(&container_dir)
                    .arg(&zip_path)
                    .output()
                    .await?
                    .status;
                if !unzip_status.success() {
                    Err(anyhow!("failed to unzip clangd archive"))?;
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
            let mut last_clangd_dir = None;
            let mut entries = fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                let entry = entry?;
                if entry.file_type().await?.is_dir() {
                    last_clangd_dir = Some(entry.path());
                }
            }
            let clangd_dir = last_clangd_dir.ok_or_else(|| anyhow!("no cached binary"))?;
            let clangd_bin = clangd_dir.join("bin/clangd");
            if clangd_bin.exists() {
                Ok(clangd_bin)
            } else {
                Err(anyhow!(
                    "missing clangd binary in directory {:?}",
                    clangd_dir
                ))
            }
        }
        .log_err()
        .boxed()
    }
}
