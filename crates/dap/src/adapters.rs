use crate::transport::Transport;
use ::fs::Fs;
use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use http_client::{github::latest_github_release, HttpClient};
use node_runtime::NodeRuntime;
use serde_json::Value;
use smol::{self, fs::File, lock::Mutex, process};
use std::{
    collections::{HashMap, HashSet},
    ffi::OsString,
    fmt::Debug,
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};
use task::DebugAdapterConfig;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DapStatus {
    None,
    CheckingForUpdate,
    Downloading,
    Failed { error: String },
}

pub trait DapDelegate {
    fn http_client(&self) -> Option<Arc<dyn HttpClient>>;
    fn node_runtime(&self) -> Option<NodeRuntime>;
    fn fs(&self) -> Arc<dyn Fs>;
    fn updated_adapters(&self) -> Arc<Mutex<HashSet<DebugAdapterName>>>;
    fn update_status(&self, dap_name: DebugAdapterName, status: DapStatus);
}

#[derive(PartialEq, Eq, Hash, Debug)]
pub struct DebugAdapterName(pub Arc<str>);

impl Deref for DebugAdapterName {
    type Target = str;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl AsRef<str> for DebugAdapterName {
    fn as_ref(&self) -> &str {
        &self.0
    }
}

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
    pub version: String,
}

pub struct AdapterVersion {
    pub tag_name: String,
    pub url: String,
}

pub struct GithubRepo {
    pub repo_name: String,
    pub repo_owner: String,
}

pub async fn download_adapter_from_github(
    adapter_name: DebugAdapterName,
    github_version: AdapterVersion,
    delegate: &dyn DapDelegate,
) -> Result<PathBuf> {
    let adapter_path = paths::debug_adapters_dir().join(&adapter_name);
    let version_dir = adapter_path.join(format!("{}_{}", adapter_name, github_version.tag_name));
    let fs = delegate.fs();

    let http_client = delegate
        .http_client()
        .ok_or_else(|| anyhow!("Failed to download adapter: couldn't connect to GitHub"))?;

    if !adapter_path.exists() {
        fs.create_dir(&adapter_path.as_path()).await?;
    }

    if version_dir.exists() {
        return Ok(version_dir);
    }

    let asset_name = format!("{}_{}.zip", &adapter_name, github_version.tag_name);
    let zip_path = adapter_path.join(&asset_name);
    fs.remove_file(
        zip_path.as_path(),
        fs::RemoveOptions {
            recursive: true,
            ignore_if_not_exists: true,
        },
    )
    .await?;

    let mut response = http_client
        .get(&github_version.url, Default::default(), true)
        .await
        .context("Error downloading release")?;

    let mut file = File::create(&zip_path).await?;
    futures::io::copy(response.body_mut(), &mut file).await?;

    let old_files: HashSet<_> = util::fs::collect_matching(&adapter_path.as_path(), |file_path| {
        file_path != zip_path.as_path()
    })
    .await
    .into_iter()
    .filter_map(|file_path| {
        file_path
            .file_name()
            .and_then(|f| f.to_str())
            .map(|f| f.to_string())
    })
    .collect();

    let _unzip_status = process::Command::new("unzip")
        .current_dir(&adapter_path)
        .arg(&zip_path)
        .output()
        .await?
        .status;

    let file_name = util::fs::find_file_name_in_dir(&adapter_path.as_path(), |file_name| {
        !file_name.ends_with(".zip") && !old_files.contains(file_name)
    })
    .await
    .ok_or_else(|| anyhow!("Unzipped directory not found"));

    let file_name = file_name?;
    let downloaded_path = adapter_path
        .join(format!("{}_{}", adapter_name, github_version.tag_name))
        .to_owned();

    fs.rename(
        file_name.as_path(),
        downloaded_path.as_path(),
        Default::default(),
    )
    .await?;

    util::fs::remove_matching(&adapter_path, |entry| entry != version_dir).await;

    // if !unzip_status.success() {
    //     dbg!(unzip_status);
    //     Err(anyhow!("failed to unzip downloaded dap archive"))?;
    // }

    Ok(downloaded_path)
}

pub async fn fetch_latest_adapter_version_from_github(
    github_repo: GithubRepo,
    delegate: &dyn DapDelegate,
) -> Result<AdapterVersion> {
    let http_client = delegate
        .http_client()
        .ok_or_else(|| anyhow!("Failed to download adapter: couldn't connect to GitHub"))?;
    let repo_name_with_owner = format!("{}/{}", github_repo.repo_owner, github_repo.repo_name);
    let release = latest_github_release(&repo_name_with_owner, false, false, http_client).await?;

    Ok(AdapterVersion {
        tag_name: release.tag_name,
        url: release.zipball_url,
    })
}

#[async_trait(?Send)]
pub trait DebugAdapter: 'static + Send + Sync {
    fn name(&self) -> DebugAdapterName;

    async fn get_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugAdapterConfig,
    ) -> Result<DebugAdapterBinary> {
        if delegate
            .updated_adapters()
            .lock()
            .await
            .contains(&self.name())
        {
            log::info!("Using cached debug adapter binary {}", self.name());

            return self.get_installed_binary(delegate, config).await;
        }

        log::info!("Getting latest version of debug adapter {}", self.name());
        delegate.update_status(self.name(), DapStatus::CheckingForUpdate);
        let version = self.fetch_latest_adapter_version(delegate).await.ok();

        let mut binary = self.get_installed_binary(delegate, config).await;

        if let Some(version) = version {
            if binary
                .as_ref()
                .is_ok_and(|binary| binary.version == version.tag_name)
            {
                delegate
                    .updated_adapters()
                    .lock_arc()
                    .await
                    .insert(self.name());

                return Ok(binary?);
            }

            delegate.update_status(self.name(), DapStatus::Downloading);
            self.install_binary(version, delegate).await?;
            binary = self.get_installed_binary(delegate, config).await;
        } else {
            log::error!(
                "Failed getting latest version of debug adapter {}",
                self.name()
            );
        }

        if binary.is_err() {
            delegate.update_status(
                self.name(),
                DapStatus::Failed {
                    error: format!("Failed to download {}", self.name()),
                },
            );
        }
        let binary = binary?;

        delegate
            .updated_adapters()
            .lock_arc()
            .await
            .insert(self.name());

        Ok(binary)
    }

    fn transport(&self) -> Box<dyn Transport>;

    async fn fetch_latest_adapter_version(
        &self,
        delegate: &dyn DapDelegate,
    ) -> Result<AdapterVersion>;

    /// Installs the binary for the debug adapter.
    /// This method is called when the adapter binary is not found or needs to be updated.
    /// It should download and install the necessary files for the debug adapter to function.
    async fn install_binary(
        &self,
        version: AdapterVersion,
        delegate: &dyn DapDelegate,
    ) -> Result<()>;

    async fn get_installed_binary(
        &self,
        delegate: &dyn DapDelegate,
        config: &DebugAdapterConfig,
    ) -> Result<DebugAdapterBinary>;

    /// Should return base configuration to make the debug adapter work
    fn request_args(&self, config: &DebugAdapterConfig) -> Value;
}
