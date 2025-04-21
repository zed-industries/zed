use ::fs::Fs;
use anyhow::{Context as _, Result, anyhow};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use dap_types::StartDebuggingRequestArguments;
use futures::io::BufReader;
use gpui::{AsyncApp, SharedString};
pub use http_client::{HttpClient, github::latest_github_release};
use language::LanguageToolchainStore;
use node_runtime::NodeRuntime;
use serde::{Deserialize, Serialize};
use settings::WorktreeId;
use smol::{self, fs::File, lock::Mutex};
use std::{
    borrow::Borrow,
    collections::{HashMap, HashSet},
    ffi::{OsStr, OsString},
    fmt::Debug,
    net::Ipv4Addr,
    ops::Deref,
    path::PathBuf,
    sync::Arc,
};
use task::DebugTaskDefinition;
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
    fn worktree_id(&self) -> WorktreeId;
    fn http_client(&self) -> Arc<dyn HttpClient>;
    fn node_runtime(&self) -> NodeRuntime;
    fn toolchain_store(&self) -> Arc<dyn LanguageToolchainStore>;
    fn fs(&self) -> Arc<dyn Fs>;
    fn updated_adapters(&self) -> Arc<Mutex<HashSet<DebugAdapterName>>>;
    fn update_status(&self, dap_name: DebugAdapterName, status: DapStatus);
    fn which(&self, command: &OsStr) -> Option<PathBuf>;
    async fn shell_env(&self) -> collections::HashMap<String, String>;
}

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct DebugAdapterName(pub SharedString);

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

impl Borrow<str> for DebugAdapterName {
    fn borrow(&self) -> &str {
        &self.0
    }
}

impl std::fmt::Display for DebugAdapterName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        std::fmt::Display::fmt(&self.0, f)
    }
}

impl From<DebugAdapterName> for SharedString {
    fn from(name: DebugAdapterName) -> Self {
        name.0
    }
}

impl<'a> From<&'a str> for DebugAdapterName {
    fn from(str: &'a str) -> DebugAdapterName {
        DebugAdapterName(str.to_string().into())
    }
}

#[derive(Debug, Clone)]
pub struct TcpArguments {
    pub host: Ipv4Addr,
    pub port: u16,
    pub timeout: Option<u64>,
}
#[derive(Debug, Clone)]
pub struct DebugAdapterBinary {
    pub adapter_name: DebugAdapterName,
    pub command: String,
    pub arguments: Option<Vec<OsString>>,
    pub envs: Option<HashMap<String, String>>,
    pub cwd: Option<PathBuf>,
    pub connection: Option<TcpArguments>,
    pub request_args: StartDebuggingRequestArguments,
}

#[derive(Debug)]
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
    let adapter_path = paths::debug_adapters_dir().join(&adapter_name.as_ref());
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

    let mut response = delegate
        .http_client()
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
            util::command::new_smol_command("unzip")
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
    let release = latest_github_release(
        &format!("{}/{}", github_repo.repo_owner, github_repo.repo_name),
        false,
        false,
        delegate.http_client(),
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
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        if delegate
            .updated_adapters()
            .lock()
            .await
            .contains(&self.name())
        {
            log::info!("Using cached debug adapter binary {}", self.name());

            if let Some(binary) = self
                .get_installed_binary(delegate, &config, user_installed_path.clone(), cx)
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
            match self.install_binary(version, delegate).await {
                Ok(_) => {
                    delegate.update_status(self.name(), DapStatus::None);
                }
                Err(error) => {
                    delegate.update_status(
                        self.name(),
                        DapStatus::Failed {
                            error: error.to_string(),
                        },
                    );

                    return Err(error);
                }
            }

            delegate
                .updated_adapters()
                .lock_arc()
                .await
                .insert(self.name());
        }

        self.get_installed_binary(delegate, &config, user_installed_path, cx)
            .await
    }

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
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary>;
}
#[cfg(any(test, feature = "test-support"))]
pub struct FakeAdapter {}

#[cfg(any(test, feature = "test-support"))]
impl FakeAdapter {
    pub const ADAPTER_NAME: &'static str = "fake-adapter";

    pub fn new() -> Self {
        Self {}
    }

    fn request_args(&self, config: &DebugTaskDefinition) -> StartDebuggingRequestArguments {
        use serde_json::json;
        use task::DebugRequestType;

        let value = json!({
            "request": match config.request {
                DebugRequestType::Launch(_) => "launch",
                DebugRequestType::Attach(_) => "attach",
            },
            "process_id": if let DebugRequestType::Attach(attach_config) = &config.request {
                attach_config.process_id
            } else {
                None
            },
        });
        let request = match config.request {
            DebugRequestType::Launch(_) => dap_types::StartDebuggingRequestArgumentsRequest::Launch,
            DebugRequestType::Attach(_) => dap_types::StartDebuggingRequestArgumentsRequest::Attach,
        };
        StartDebuggingRequestArguments {
            configuration: value,
            request,
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
#[async_trait(?Send)]
impl DebugAdapter for FakeAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    async fn get_binary(
        &self,
        _: &dyn DapDelegate,
        config: &DebugTaskDefinition,
        _: Option<PathBuf>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        Ok(DebugAdapterBinary {
            adapter_name: Self::ADAPTER_NAME.into(),
            command: "command".into(),
            arguments: None,
            connection: None,
            envs: None,
            cwd: None,
            request_args: self.request_args(config),
        })
    }

    async fn fetch_latest_adapter_version(
        &self,
        _delegate: &dyn DapDelegate,
    ) -> Result<AdapterVersion> {
        unimplemented!("fetch latest adapter version");
    }

    async fn install_binary(
        &self,
        _version: AdapterVersion,
        _delegate: &dyn DapDelegate,
    ) -> Result<()> {
        unimplemented!("install binary");
    }

    async fn get_installed_binary(
        &self,
        _: &dyn DapDelegate,
        _: &DebugTaskDefinition,
        _: Option<PathBuf>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        unimplemented!("get installed binary");
    }
}
