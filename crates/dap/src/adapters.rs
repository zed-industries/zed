use ::fs::Fs;
use anyhow::{Context as _, Result, anyhow};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use collections::HashMap;
use dap_types::{StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest};
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
    collections::HashSet,
    ffi::OsStr,
    fmt::Debug,
    net::Ipv4Addr,
    ops::Deref,
    path::{Path, PathBuf},
    str::FromStr,
    sync::Arc,
};
use task::{AttachRequest, DebugRequest, DebugScenario, LaunchRequest, TcpArgumentsTemplate};
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

impl TcpArguments {
    pub fn from_proto(proto: proto::TcpHost) -> anyhow::Result<Self> {
        let host = TcpArgumentsTemplate::from_proto(proto)?;
        Ok(TcpArguments {
            host: host.host.ok_or_else(|| anyhow!("missing host"))?,
            port: host.port.ok_or_else(|| anyhow!("missing port"))?,
            timeout: host.timeout,
        })
    }

    pub fn to_proto(&self) -> proto::TcpHost {
        TcpArgumentsTemplate {
            host: Some(self.host),
            port: Some(self.port),
            timeout: self.timeout,
        }
        .to_proto()
    }
}

/// Represents a debuggable binary/process (what process is going to be debugged and with what arguments).
///
/// We start off with a [DebugScenario], a user-facing type that additionally defines how a debug target is built; once
/// an optional build step is completed, we turn it's result into a DebugTaskDefinition by running a locator (or using a user-provided task) and resolving task variables.
/// Finally, a [DebugTaskDefinition] has to be turned into a concrete debugger invokation ([DebugAdapterBinary]).
#[derive(Clone, Debug)]
pub struct DebugTaskDefinition {
    pub label: SharedString,
    pub adapter: SharedString,
    pub request: DebugRequest,
    /// Additional initialization arguments to be sent on DAP initialization
    pub initialize_args: Option<serde_json::Value>,
    /// Whether to tell the debug adapter to stop on entry
    pub stop_on_entry: Option<bool>,
    /// Optional TCP connection information
    ///
    /// If provided, this will be used to connect to the debug adapter instead of
    /// spawning a new debug adapter process. This is useful for connecting to a debug adapter
    /// that is already running or is started by another process.
    pub tcp_connection: Option<TcpArgumentsTemplate>,
}

impl TryFrom<DebugScenario> for DebugTaskDefinition {
    type Error = anyhow::Error;

    fn try_from(value: DebugScenario) -> Result<Self, Self::Error> {
        let DebugScenario {
            label,
            adapter,
            request: Some(request),
            initialize_args,
            tcp_connection,
            stop_on_entry,
            ..
        } = value
        else {
            return Err(anyhow!(
                "DebugScenario is expected to have a concrete binary to spawn/attach to."
            ));
        };

        Ok(Self {
            label,
            adapter,
            request,
            initialize_args,
            tcp_connection,
            stop_on_entry,
        })
    }
}

impl From<DebugTaskDefinition> for DebugScenario {
    fn from(value: DebugTaskDefinition) -> Self {
        let DebugTaskDefinition {
            label,
            adapter,
            request,
            initialize_args,
            tcp_connection,
            stop_on_entry,
            ..
        } = value;

        Self {
            label,
            adapter,
            request: Some(request),
            initialize_args,
            tcp_connection,
            stop_on_entry,
            build: None,
        }
    }
}

impl DebugTaskDefinition {
    pub fn from_proto(payload: proto::DebugScenario) -> Result<Self> {
        let Some(request) = payload.request else {
            return Err(anyhow!("Missing request in debug scenario"));
        };
        let request = match request {
            proto::debug_scenario::Request::DebugAttachRequest(config) => {
                DebugRequest::Attach(AttachRequest {
                    process_id: Some(config.process_id),
                })
            }

            proto::debug_scenario::Request::DebugLaunchRequest(config) => {
                DebugRequest::Launch(LaunchRequest {
                    program: config.program,
                    cwd: config.cwd.map(|cwd| cwd.into()),
                    args: config.args,
                })
            }
        };
        Ok(Self {
            adapter: payload.adapter.into(),
            label: payload.label.into(),
            request,
            initialize_args: payload
                .configuration
                .as_deref()
                .map(serde_json::Value::from_str)
                .transpose()?,
            tcp_connection: payload
                .connection
                .map(TcpArgumentsTemplate::from_proto)
                .transpose()?,
            stop_on_entry: payload.stop_on_entry,
        })
    }

    pub fn to_proto(&self) -> proto::DebugScenario {
        todo!()
    }
    pub fn cwd(&self) -> Option<&Path> {
        if let DebugRequest::Launch(config) = &self.request {
            config.cwd.as_ref().map(Path::new)
        } else {
            None
        }
    }
}

/// Created from a [DebugTaskDefinition], this struct describes how to spawn the debugger to create a previously-configured debug session.
#[derive(Debug, Clone)]
pub struct DebugAdapterBinary {
    pub command: String,
    pub arguments: Vec<String>,
    pub envs: HashMap<String, String>,
    pub cwd: Option<PathBuf>,
    pub connection: Option<TcpArguments>,
    pub request_args: StartDebuggingRequestArguments,
}

impl DebugAdapterBinary {
    pub fn from_proto(binary: proto::DebugAdapterBinary) -> anyhow::Result<Self> {
        let request = match binary.launch_type() {
            proto::debug_adapter_binary::LaunchType::Launch => {
                StartDebuggingRequestArgumentsRequest::Launch
            }
            proto::debug_adapter_binary::LaunchType::Attach => {
                StartDebuggingRequestArgumentsRequest::Attach
            }
        };

        Ok(DebugAdapterBinary {
            command: binary.command,
            arguments: binary.arguments,
            envs: binary.envs.into_iter().collect(),
            connection: binary
                .connection
                .map(TcpArguments::from_proto)
                .transpose()?,
            request_args: StartDebuggingRequestArguments {
                configuration: serde_json::from_str(&binary.configuration)?,
                request,
            },
            cwd: binary.cwd.map(|cwd| cwd.into()),
        })
    }

    pub fn to_proto(&self) -> proto::DebugAdapterBinary {
        proto::DebugAdapterBinary {
            command: self.command.clone(),
            arguments: self.arguments.clone(),
            envs: self
                .envs
                .iter()
                .map(|(k, v)| (k.clone(), v.clone()))
                .collect(),
            cwd: self
                .cwd
                .as_ref()
                .map(|cwd| cwd.to_string_lossy().to_string()),
            connection: self.connection.as_ref().map(|c| c.to_proto()),
            launch_type: match self.request_args.request {
                StartDebuggingRequestArgumentsRequest::Launch => {
                    proto::debug_adapter_binary::LaunchType::Launch.into()
                }
                StartDebuggingRequestArgumentsRequest::Attach => {
                    proto::debug_adapter_binary::LaunchType::Attach.into()
                }
            },
            configuration: self.request_args.configuration.to_string(),
        }
    }
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

    fn request_args(&self, config: &DebugScenario) -> StartDebuggingRequestArguments {
        use serde_json::json;
        use task::DebugRequest;

        let value = json!({
            "request": match config.request {
                DebugRequest::Launch(_) => "launch",
                DebugRequest::Attach(_) => "attach",
            },
            "process_id": if let DebugRequest::Attach(attach_config) = &config.request {
                attach_config.process_id
            } else {
                None
            },
        });
        let request = match config.request {
            DebugRequest::Launch(_) => dap_types::StartDebuggingRequestArgumentsRequest::Launch,
            DebugRequest::Attach(_) => dap_types::StartDebuggingRequestArgumentsRequest::Attach,
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
            command: "command".into(),
            arguments: vec![],
            connection: None,
            envs: HashMap::default(),
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
