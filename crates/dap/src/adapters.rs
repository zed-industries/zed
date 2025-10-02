use anyhow::{Context as _, Result, anyhow};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use collections::HashMap;
pub use dap_types::{StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest};
use fs::Fs;
use futures::io::BufReader;
use gpui::{AsyncApp, SharedString};
pub use http_client::{HttpClient, github::latest_github_release};
use language::{LanguageName, LanguageToolchainStore};
use node_runtime::NodeRuntime;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::WorktreeId;
use smol::fs::File;
use std::{
    borrow::Borrow,
    ffi::OsStr,
    fmt::Debug,
    net::Ipv4Addr,
    ops::Deref,
    path::{Path, PathBuf},
    sync::Arc,
};
use task::{DebugScenario, TcpArgumentsTemplate, ZedDebugConfig};
use util::{archive::extract_zip, rel_path::RelPath};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DapStatus {
    None,
    CheckingForUpdate,
    Downloading,
    Failed { error: String },
}

#[async_trait]
pub trait DapDelegate: Send + Sync + 'static {
    fn worktree_id(&self) -> WorktreeId;
    fn worktree_root_path(&self) -> &Path;
    fn http_client(&self) -> Arc<dyn HttpClient>;
    fn node_runtime(&self) -> NodeRuntime;
    fn toolchain_store(&self) -> Arc<dyn LanguageToolchainStore>;
    fn fs(&self) -> Arc<dyn Fs>;
    fn output_to_console(&self, msg: String);
    async fn which(&self, command: &OsStr) -> Option<PathBuf>;
    async fn read_text_file(&self, path: &RelPath) -> Result<String>;
    async fn shell_env(&self) -> collections::HashMap<String, String>;
}

#[derive(
    Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize, JsonSchema,
)]
#[serde(transparent)]
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

impl Borrow<SharedString> for DebugAdapterName {
    fn borrow(&self) -> &SharedString {
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
impl From<SharedString> for DebugAdapterName {
    fn from(name: SharedString) -> Self {
        DebugAdapterName(name)
    }
}

impl<'a> From<&'a str> for DebugAdapterName {
    fn from(str: &'a str) -> DebugAdapterName {
        DebugAdapterName(str.to_string().into())
    }
}

#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct TcpArguments {
    pub host: Ipv4Addr,
    pub port: u16,
    pub timeout: Option<u64>,
}

impl TcpArguments {
    pub fn from_proto(proto: proto::TcpHost) -> anyhow::Result<Self> {
        let host = TcpArgumentsTemplate::from_proto(proto)?;
        Ok(TcpArguments {
            host: host.host.context("missing host")?,
            port: host.port.context("missing port")?,
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
/// Finally, a [DebugTaskDefinition] has to be turned into a concrete debugger invocation ([DebugAdapterBinary]).
#[derive(Clone, Debug, PartialEq)]
#[cfg_attr(
    any(feature = "test-support", test),
    derive(serde::Deserialize, serde::Serialize)
)]
pub struct DebugTaskDefinition {
    /// The name of this debug task
    pub label: SharedString,
    /// The debug adapter to use
    pub adapter: DebugAdapterName,
    /// The configuration to send to the debug adapter
    pub config: serde_json::Value,
    /// Optional TCP connection information
    ///
    /// If provided, this will be used to connect to the debug adapter instead of
    /// spawning a new debug adapter process. This is useful for connecting to a debug adapter
    /// that is already running or is started by another process.
    pub tcp_connection: Option<TcpArgumentsTemplate>,
}

impl DebugTaskDefinition {
    pub fn to_scenario(&self) -> DebugScenario {
        DebugScenario {
            label: self.label.clone(),
            adapter: self.adapter.clone().into(),
            build: None,
            tcp_connection: self.tcp_connection.clone(),
            config: self.config.clone(),
        }
    }

    pub fn to_proto(&self) -> proto::DebugTaskDefinition {
        proto::DebugTaskDefinition {
            label: self.label.clone().into(),
            config: self.config.to_string(),
            tcp_connection: self.tcp_connection.clone().map(|v| v.to_proto()),
            adapter: self.adapter.clone().0.into(),
        }
    }

    pub fn from_proto(proto: proto::DebugTaskDefinition) -> Result<Self> {
        Ok(Self {
            label: proto.label.into(),
            config: serde_json::from_str(&proto.config)?,
            tcp_connection: proto
                .tcp_connection
                .map(TcpArgumentsTemplate::from_proto)
                .transpose()?,
            adapter: DebugAdapterName(proto.adapter.into()),
        })
    }
}

/// Created from a [DebugTaskDefinition], this struct describes how to spawn the debugger to create a previously-configured debug session.
#[derive(Debug, Clone, PartialEq, Serialize)]
pub struct DebugAdapterBinary {
    pub command: Option<String>,
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
                .map(|cwd| cwd.to_string_lossy().into_owned()),
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

#[derive(Debug, Clone)]
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
        fs.create_dir(adapter_path.as_path())
            .await
            .context("Failed creating adapter path")?;
    }

    log::debug!(
        "Downloading adapter {} from {}",
        adapter_name,
        &github_version.url,
    );
    delegate.output_to_console(format!("Downloading from {}...", github_version.url));

    let mut response = delegate
        .http_client()
        .get(&github_version.url, Default::default(), true)
        .await
        .context("Error downloading release")?;
    anyhow::ensure!(
        response.status().is_success(),
        "download failed with status {}",
        response.status().to_string()
    );

    delegate.output_to_console("Download complete".to_owned());
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
            let file = File::open(&zip_path).await?;
            extract_zip(&version_path, file)
                .await
                // we cannot check the status as some adapter include files with names that trigger `Illegal byte sequence`
                .ok();

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

#[async_trait(?Send)]
pub trait DebugAdapter: 'static + Send + Sync {
    fn name(&self) -> DebugAdapterName;

    async fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario>;

    async fn get_binary(
        &self,
        delegate: &Arc<dyn DapDelegate>,
        config: &DebugTaskDefinition,
        user_installed_path: Option<PathBuf>,
        user_args: Option<Vec<String>>,
        cx: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary>;

    /// Returns the language name of an adapter if it only supports one language
    fn adapter_language_name(&self) -> Option<LanguageName> {
        None
    }

    /// Extracts the kind (attach/launch) of debug configuration from the given JSON config.
    /// This method should only return error when the kind cannot be determined for a given configuration;
    /// in particular, it *should not* validate whether the request as a whole is valid, because that's best left to the debug adapter itself to decide.
    async fn request_kind(
        &self,
        config: &serde_json::Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest> {
        match config.get("request") {
            Some(val) if val == "launch" => Ok(StartDebuggingRequestArgumentsRequest::Launch),
            Some(val) if val == "attach" => Ok(StartDebuggingRequestArgumentsRequest::Attach),
            _ => Err(anyhow!(
                "missing or invalid `request` field in config. Expected 'launch' or 'attach'"
            )),
        }
    }

    fn dap_schema(&self) -> serde_json::Value;

    fn label_for_child_session(&self, _args: &StartDebuggingRequestArguments) -> Option<String> {
        None
    }

    fn compact_child_session(&self) -> bool {
        false
    }

    fn prefer_thread_name(&self) -> bool {
        false
    }
}

#[cfg(any(test, feature = "test-support"))]
pub struct FakeAdapter {}

#[cfg(any(test, feature = "test-support"))]
impl FakeAdapter {
    pub const ADAPTER_NAME: &'static str = "fake-adapter";

    pub fn new() -> Self {
        Self {}
    }
}

#[cfg(any(test, feature = "test-support"))]
#[async_trait(?Send)]
impl DebugAdapter for FakeAdapter {
    fn name(&self) -> DebugAdapterName {
        DebugAdapterName(Self::ADAPTER_NAME.into())
    }

    fn dap_schema(&self) -> serde_json::Value {
        serde_json::Value::Null
    }

    async fn request_kind(
        &self,
        config: &serde_json::Value,
    ) -> Result<StartDebuggingRequestArgumentsRequest> {
        let request = config.as_object().unwrap()["request"].as_str().unwrap();

        let request = match request {
            "launch" => dap_types::StartDebuggingRequestArgumentsRequest::Launch,
            "attach" => dap_types::StartDebuggingRequestArgumentsRequest::Attach,
            _ => unreachable!("Wrong fake adapter input for request field"),
        };

        Ok(request)
    }

    fn adapter_language_name(&self) -> Option<LanguageName> {
        None
    }

    async fn config_from_zed_format(&self, zed_scenario: ZedDebugConfig) -> Result<DebugScenario> {
        let config = serde_json::to_value(zed_scenario.request).unwrap();

        Ok(DebugScenario {
            adapter: zed_scenario.adapter,
            label: zed_scenario.label,
            build: None,
            config,
            tcp_connection: None,
        })
    }

    async fn get_binary(
        &self,
        _: &Arc<dyn DapDelegate>,
        task_definition: &DebugTaskDefinition,
        _: Option<PathBuf>,
        _: Option<Vec<String>>,
        _: &mut AsyncApp,
    ) -> Result<DebugAdapterBinary> {
        let connection = task_definition
            .tcp_connection
            .as_ref()
            .map(|connection| TcpArguments {
                host: connection.host(),
                port: connection.port.unwrap_or(17),
                timeout: connection.timeout,
            });
        Ok(DebugAdapterBinary {
            command: Some("command".into()),
            arguments: vec![],
            connection,
            envs: HashMap::default(),
            cwd: None,
            request_args: StartDebuggingRequestArguments {
                request: self.request_kind(&task_definition.config).await?,
                configuration: task_definition.config.clone(),
            },
        })
    }
}
