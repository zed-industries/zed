use super::installation::latest_github_release;
use anyhow::{anyhow, Result};
use client::http::HttpClient;
use futures::{future::BoxFuture, FutureExt, StreamExt};
pub use language::*;
use lazy_static::lazy_static;
use regex::Regex;
use smol::{fs, process};
use std::{
    any::Any,
    path::{Path, PathBuf},
    str,
    sync::Arc,
};
use util::{ResultExt, TryFutureExt};

#[derive(Copy, Clone)]
pub struct GoLspAdapter;

lazy_static! {
    static ref GOPLS_VERSION_REGEX: Regex = Regex::new(r"\d+\.\d+\.\d+").unwrap();
}

impl super::LspAdapter for GoLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("gopls".into())
    }

    fn server_args(&self) -> &[&str] {
        &["-mode=stdio"]
    }

    fn fetch_latest_server_version(
        &self,
        http: Arc<dyn HttpClient>,
    ) -> BoxFuture<'static, Result<Box<dyn 'static + Send + Any>>> {
        async move {
            let release = latest_github_release("golang/tools", http).await?;
            let version: Option<String> = release.name.strip_prefix("gopls/v").map(str::to_string);
            if version.is_none() {
                log::warn!(
                    "couldn't infer gopls version from github release name '{}'",
                    release.name
                );
            }
            Ok(Box::new(version) as Box<_>)
        }
        .boxed()
    }

    fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        _: Arc<dyn HttpClient>,
        container_dir: Arc<Path>,
    ) -> BoxFuture<'static, Result<PathBuf>> {
        let version = version.downcast::<Option<String>>().unwrap();
        let this = *self;

        async move {
            if let Some(version) = *version {
                let binary_path = container_dir.join(&format!("gopls_{version}"));
                if let Ok(metadata) = fs::metadata(&binary_path).await {
                    if metadata.is_file() {
                        if let Some(mut entries) = fs::read_dir(&container_dir).await.log_err() {
                            while let Some(entry) = entries.next().await {
                                if let Some(entry) = entry.log_err() {
                                    let entry_path = entry.path();
                                    if entry_path.as_path() != binary_path
                                        && entry.file_name() != "gobin"
                                    {
                                        fs::remove_file(&entry_path).await.log_err();
                                    }
                                }
                            }
                        }

                        return Ok(binary_path.to_path_buf());
                    }
                }
            } else if let Some(path) = this.cached_server_binary(container_dir.clone()).await {
                return Ok(path.to_path_buf());
            }

            let gobin_dir = container_dir.join("gobin");
            fs::create_dir_all(&gobin_dir).await?;
            let install_output = process::Command::new("go")
                .env("GO111MODULE", "on")
                .env("GOBIN", &gobin_dir)
                .args(["install", "golang.org/x/tools/gopls@latest"])
                .output()
                .await?;
            if !install_output.status.success() {
                Err(anyhow!("failed to install gopls"))?;
            }

            let installed_binary_path = gobin_dir.join("gopls");
            let version_output = process::Command::new(&installed_binary_path)
                .arg("version")
                .output()
                .await
                .map_err(|e| anyhow!("failed to run installed gopls binary {:?}", e))?;
            let version_stdout = str::from_utf8(&version_output.stdout)
                .map_err(|_| anyhow!("gopls version produced invalid utf8"))?;
            let version = GOPLS_VERSION_REGEX
                .find(version_stdout)
                .ok_or_else(|| anyhow!("failed to parse gopls version output"))?
                .as_str();
            let binary_path = container_dir.join(&format!("gopls_{version}"));
            fs::rename(&installed_binary_path, &binary_path).await?;

            Ok(binary_path.to_path_buf())
        }
        .boxed()
    }

    fn cached_server_binary(
        &self,
        container_dir: Arc<Path>,
    ) -> BoxFuture<'static, Option<PathBuf>> {
        async move {
            let mut last_binary_path = None;
            let mut entries = fs::read_dir(&container_dir).await?;
            while let Some(entry) = entries.next().await {
                let entry = entry?;
                if entry.file_type().await?.is_file()
                    && entry
                        .file_name()
                        .to_str()
                        .map_or(false, |name| name.starts_with("gopls_"))
                {
                    last_binary_path = Some(entry.path());
                }
            }

            if let Some(path) = last_binary_path {
                Ok(path.to_path_buf())
            } else {
                Err(anyhow!("no cached binary"))
            }
        }
        .log_err()
        .boxed()
    }
}
