use crate::transport::Transport;
use ::fs::Fs;
use anyhow::{anyhow, Context as _, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use futures::io::BufReader;
use gpui::SharedString;
pub use http_client::{github::latest_github_release, HttpClient};
use node_runtime::NodeRuntime;
use serde::{Deserialize, Serialize};
use serde_json::Value;
use smol::{self, fs::File, lock::Mutex, process};
use std::{
    collections::{HashMap, HashSet},
    ffi::{OsStr, OsString},
    fmt::Debug,
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};
use sysinfo::{Pid, Process};
use task::DebugAdapterConfig;
use util::ResultExt;

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DapStatus {
    None,
    CheckingForUpdate,
    Downloading,
    Failed { error: String },
}

#[async_trait(?Send)]
pub trait DapDelegate {
    fn http_client(&self) -> Option<Arc<dyn HttpClient>>;
    fn node_runtime(&self) -> Option<NodeRuntime>;
    fn fs(&self) -> Arc<dyn Fs>;
    fn updated_adapters(&self) -> Arc<Mutex<HashSet<DebugAdapterName>>>;
    fn update_status(&self, dap_name: DebugAdapterName, status: DapStatus);
    fn which(&self, command: &OsStr) -> Option<PathBuf>;
    async fn shell_env(&self) -> collections::HashMap<String, String>;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
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

impl From<DebugAdapterName> for SharedString {
    fn from(name: DebugAdapterName) -> Self {
        SharedString::from(name.0)
    }
}

impl<'a> From<&'a str> for DebugAdapterName {
    fn from(str: &'a str) -> DebugAdapterName {
        DebugAdapterName(str.to_string().into())
    }
}

#[derive(Debug, Clone)]
pub struct DebugAdapterBinary {
    pub command: String,
    pub arguments: Option<Vec<OsString>>,
    pub envs: Option<HashMap<String, String>>,
    pub cwd: Option<PathBuf>,
}

pub struct AdapterVersion {
    pub tag_name: String,
    pub url: String,
}

pub enum DownloadedFileType {
    Vsix,
    GzipTar,
    Zip,
}

pub struct GithubRepo {
    pub repo_name: String,
    pub repo_owner: String,
}

pub async fn download_adapter_from_github(
    adapter_name: DebugAdapterName,
    github_version: AdapterVersion,
    file_type: DownloadedFileType,
    delegate: &dyn DapDelegate,
) -> Result<PathBuf> {
    let adapter_path = paths::debug_adapters_dir().join(&adapter_name);
    let version_path = adapter_path.join(format!("{}_{}", adapter_name, github_version.tag_name));
    let fs = delegate.fs();

    if version_path.exists() {
        return Ok(version_path);
    }

    if !adapter_path.exists() {
        fs.create_dir(&adapter_path.as_path())
            .await
            .context("Failed creating adapter path")?;
    }

    log::debug!(
        "Downloading adapter {} from {}",
        adapter_name,
        &github_version.url,
    );

    let http_client = delegate
        .http_client()
        .ok_or_else(|| anyhow!("Failed to download adapter: couldn't connect to GitHub"))?;
    let mut response = http_client
        .get(&github_version.url, Default::default(), true)
        .await
        .context("Error downloading release")?;
    if !response.status().is_success() {
        Err(anyhow!(
            "download failed with status {}",
            response.status().to_string()
        ))?;
    }

    match file_type {
        DownloadedFileType::GzipTar => {
            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let archive = Archive::new(decompressed_bytes);
            archive.unpack(&version_path).await?;
        }
        DownloadedFileType::Zip | DownloadedFileType::Vsix => {
            let zip_path = version_path.with_extension("zip");

            let mut file = File::create(&zip_path).await?;
            futures::io::copy(response.body_mut(), &mut file).await?;

            // we cannot check the status as some adapter include files with names that trigger `Illegal byte sequence`
            process::Command::new("unzip")
                .arg(&zip_path)
                .arg("-d")
                .arg(&version_path)
                .output()
                .await?;

            util::fs::remove_matching(&adapter_path, |entry| {
                entry
                    .file_name()
                    .is_some_and(|file| file.to_string_lossy().ends_with(".zip"))
            })
            .await;
        }
    }

    // remove older versions
    util::fs::remove_matching(&adapter_path, |entry| {
        entry.to_string_lossy() != version_path.to_string_lossy()
    })
    .await;

    Ok(version_path)
}

pub async fn fetch_latest_adapter_version_from_github(
    github_repo: GithubRepo,
    delegate: &dyn DapDelegate,
) -> Result<AdapterVersion> {
    let http_client = delegate
        .http_client()
        .ok_or_else(|| anyhow!("Failed to download adapter: couldn't connect to GitHub"))?;

    let release = latest_github_release(
        &format!("{}/{}", github_repo.repo_owner, github_repo.repo_name),
        false,
        false,
        http_client,
    )
    .await?;

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
        user_installed_path: Option<PathBuf>,
    ) -> Result<DebugAdapterBinary> {
        if delegate
            .updated_adapters()
            .lock()
            .await
            .contains(&self.name())
        {
            log::info!("Using cached debug adapter binary {}", self.name());

            if let Some(binary) = self
                .get_installed_binary(delegate, &config, user_installed_path.clone())
                .await
                .log_err()
            {
                return Ok(binary);
            }

            log::info!(
                "Cached binary {} is corrupt falling back to install",
                self.name()
            );
        }

        log::info!("Getting latest version of debug adapter {}", self.name());
        delegate.update_status(self.name(), DapStatus::CheckingForUpdate);
        if let Some(version) = self.fetch_latest_adapter_version(delegate).await.log_err() {
            log::info!(
                "Installiing latest version of debug adapter {}",
                self.name()
            );
            delegate.update_status(self.name(), DapStatus::Downloading);
            self.install_binary(version, delegate).await?;

            delegate
                .updated_adapters()
                .lock_arc()
                .await
                .insert(self.name());
        }

        self.get_installed_binary(delegate, &config, user_installed_path)
            .await
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
        user_installed_path: Option<PathBuf>,
    ) -> Result<DebugAdapterBinary>;

    /// Should return base configuration to make the debug adapter work
    fn request_args(&self, config: &DebugAdapterConfig) -> Value;

    /// Whether the adapter supports `attach` request,
    /// if not support and the request is selected we will show an error message
    fn supports_attach(&self) -> bool {
        false
    }

    /// Filters out the processes that the adapter can attach to for debugging
    fn attach_processes<'a>(
        &self,
        _: &'a HashMap<Pid, Process>,
    ) -> Option<Vec<(&'a Pid, &'a Process)>> {
        None
    }
}
