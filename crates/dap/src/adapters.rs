use crate::transport::Transport;
use ::fs::Fs;
use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use http_client::{github::latest_github_release, HttpClient};
use node_runtime::NodeRuntime;
use serde_json::Value;
use smol::{self, fs::File, process};
use std::{
    collections::HashMap,
    ffi::OsString,
    fmt::Debug,
    path::{Path, PathBuf},
    sync::Arc,
};
use task::DebugAdapterConfig;

pub trait DapDelegate {
    fn http_client(&self) -> Option<Arc<dyn HttpClient>>;
    fn node_runtime(&self) -> Option<NodeRuntime>;
    fn fs(&self) -> Arc<dyn Fs>;
}

pub struct DebugAdapterName(pub Arc<str>);

impl AsRef<Path> for DebugAdapterName {
    fn as_ref(&self) -> &Path {
        Path::new(&*self.0)
    }
}

impl std::fmt::Display for DebugAdapterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

#[derive(Debug, Clone)]
pub struct DebugAdapterBinary {
    pub command: String,
    pub arguments: Option<Vec<OsString>>,
    pub envs: Option<HashMap<String, String>>,
}

pub async fn download_adapter_from_github(
    adapter_name: DebugAdapterName,
    github_repo: GithubRepo,
    delegate: &dyn DapDelegate,
) -> Result<PathBuf> {
    let adapter_path = paths::debug_adapters_dir().join(&adapter_name);
    let fs = delegate.fs();

    if let Some(http_client) = delegate.http_client() {
        if !adapter_path.exists() {
            fs.create_dir(&adapter_path.as_path()).await?;
        }

        let repo_name_with_owner = format!("{}/{}", github_repo.repo_owner, github_repo.repo_name);
        let release =
            latest_github_release(&repo_name_with_owner, false, false, http_client.clone()).await?;

        let asset_name = format!("{}_{}.zip", &adapter_name, release.tag_name);
        let zip_path = adapter_path.join(&asset_name);

        if smol::fs::metadata(&zip_path).await.is_err() {
            let mut response = http_client
                .get(&release.zipball_url, Default::default(), true)
                .await
                .context("Error downloading release")?;

            let mut file = File::create(&zip_path).await?;
            futures::io::copy(response.body_mut(), &mut file).await?;

            let _unzip_status = process::Command::new("unzip")
                .current_dir(&adapter_path)
                .arg(&zip_path)
                .output()
                .await?
                .status;

            fs.remove_file(&zip_path.as_path(), Default::default())
                .await?;

            let file_name = util::fs::find_file_name_in_dir(&adapter_path.as_path(), |file_name| {
                file_name.contains(&adapter_name.to_string())
            })
            .await
            .ok_or_else(|| anyhow!("Unzipped directory not found"));

            let file_name = file_name?;
            let downloaded_path = adapter_path
                .join(format!("{}_{}", adapter_name, release.tag_name))
                .to_owned();

            fs.rename(
                file_name.as_path(),
                downloaded_path.as_path(),
                Default::default(),
            )
            .await?;

            // if !unzip_status.success() {
            //     dbg!(unzip_status);
            //     Err(anyhow!("failed to unzip downloaded dap archive"))?;
            // }

            return Ok(downloaded_path);
        }
    }

    bail!("Install failed to download & counldn't preinstalled dap")
}

pub struct GithubRepo {
    pub repo_name: String,
    pub repo_owner: String,
}

#[async_trait(?Send)]
pub trait DebugAdapter: 'static + Send + Sync {
    fn id(&self) -> String {
        "".to_string()
    }

    fn name(&self) -> DebugAdapterName;

    fn transport(&self) -> Box<dyn Transport>;

    /// Installs the binary for the debug adapter.
    /// This method is called when the adapter binary is not found or needs to be updated.
    /// It should download and install the necessary files for the debug adapter to function.
    async fn install_binary(&self, delegate: &dyn DapDelegate) -> Result<()>;

    async fn fetch_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugAdapterConfig,
    ) -> Result<DebugAdapterBinary>;

    /// Should return base configuration to make the debug adapter work
    fn request_args(&self, config: &DebugAdapterConfig) -> Value;
}
