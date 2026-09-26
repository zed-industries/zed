use anyhow::{Context as _, Result, bail};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use chrono::{DateTime, Utc};
use futures::{AsyncReadExt, FutureExt as _, channel::oneshot, future::BoxFuture, future::Shared};
use http_client::{Host, HttpClient, Url};
use semver::{Version, VersionReq};
use serde::Deserialize;
use smol::io::BufReader;
use smol::{fs, lock::Mutex};
use std::collections::HashMap;

use std::{
    env::{self, consts},
    ffi::OsString,
    io,
    net::{IpAddr, Ipv4Addr},
    path::{Path, PathBuf},
    process::Output,
    sync::Arc,
};
use util::archive::extract_zip;
use util::{ResultExt, ToolPermissionDenied};

const NODE_CA_CERTS_ENV_VAR: &str = "NODE_EXTRA_CA_CERTS";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NodeBinaryOptions {
    pub allow_path_lookup: bool,
    pub allow_binary_downloads: bool,
    pub use_paths: Option<(PathBuf, PathBuf)>,
}

pub type NpmInstallGate = Arc<dyn Fn(String) -> BoxFuture<'static, bool> + Send + Sync>;

/// Use this when you need to launch npm as a long-lived process (for example, an agent server),
/// so the invocation and environment stay consistent with the Node runtime's proxy and CA setup.
#[derive(Clone, Debug)]
pub struct NpmCommand {
    pub path: PathBuf,
    pub args: Vec<String>,
    pub env: HashMap<String, String>,
}

pub enum VersionStrategy<'a> {
    /// Install if current version doesn't match pinned version
    Pin(&'a Version),
    /// Install if current version is older than latest version
    Latest(&'a Version),
}

#[derive(Clone)]
pub struct NodeRuntime {
    state: Arc<Mutex<NodeRuntimeState>>,
    install_gate: Option<NpmInstallGate>,
}

struct NodeRuntimeState {
    http: Arc<dyn HttpClient>,
    instance: Option<Box<dyn NodeRuntimeTrait>>,
    last_options: Option<NodeBinaryOptions>,
    options: watch::Receiver<Option<NodeBinaryOptions>>,
    shell_env_loaded: Shared<oneshot::Receiver<()>>,
}

impl NodeRuntime {
    pub fn new(
        http: Arc<dyn HttpClient>,
        shell_env_loaded: Option<oneshot::Receiver<()>>,
        options: watch::Receiver<Option<NodeBinaryOptions>>,
        install_gate: Option<NpmInstallGate>,
    ) -> Self {
        Self {
            state: Arc::new(Mutex::new(NodeRuntimeState {
                http,
                instance: None,
                last_options: None,
                options,
                shell_env_loaded: shell_env_loaded.unwrap_or(oneshot::channel().1).shared(),
            })),
            install_gate,
        }
    }

    pub fn unavailable() -> Self {
        Self {
            state: Arc::new(Mutex::new(NodeRuntimeState {
                http: Arc::new(http_client::BlockedHttpClient),
                instance: Some(Box::new(UnavailableNodeRuntime)),
                last_options: Some(NodeBinaryOptions::default()),
                options: watch::channel(Some(NodeBinaryOptions::default())).1,
                shell_env_loaded: oneshot::channel().1.shared(),
            })),
            install_gate: None,
        }
    }

    pub fn with_install_gate(&self, gate: Option<NpmInstallGate>) -> Self {
        Self {
            state: self.state.clone(),
            install_gate: gate,
        }
    }

    async fn instance(&self) -> Result<Box<dyn NodeRuntimeTrait>> {
        self.instance_with_managed_dir(&paths::data_dir().join("node"))
            .await
    }

    async fn instance_with_managed_dir(
        &self,
        managed_dir: &Path,
    ) -> Result<Box<dyn NodeRuntimeTrait>> {
        let mut state = self.state.lock().await;

        let options = loop {
            if let Some(options) = state.options.borrow().as_ref() {
                break options.clone();
            }
            match state.options.changed().await {
                Ok(()) => {}
                // failure case not cached
                Err(err) => {
                    return Err(err.into());
                }
            }
        };

        if state.last_options.as_ref() != Some(&options) {
            state.instance.take();
        }
        if let Some(instance) = state.instance.as_ref() {
            return Ok(instance.boxed_clone());
        }

        if let Some((node, npm)) = options.use_paths.as_ref() {
            let instance =
                match SystemNodeRuntime::new(node.clone(), npm.clone(), self.install_gate.as_ref())
                    .await
                {
                    Ok(instance) => {
                        log::info!("using Node.js from `node.path` in settings: {:?}", instance);
                        Box::new(instance)
                    }
                    Err(err) => {
                        // failure case not cached, since it's cheap to check again
                        return Err(err.context(format!(
                            "failure checking Node.js from `node.path` in settings ({})",
                            node.display()
                        )));
                    }
                };
            state.instance = Some(instance.boxed_clone());
            state.last_options = Some(options);
            return Ok(instance);
        }

        let system_node_error = if options.allow_path_lookup {
            state.shell_env_loaded.clone().await.ok();
            match SystemNodeRuntime::detect(self.install_gate.as_ref()).await {
                Ok(instance) => {
                    log::info!("using Node.js found on PATH: {:?}", instance);
                    state.instance = Some(instance.boxed_clone());
                    state.last_options = Some(options);
                    return Ok(Box::new(instance));
                }
                Err(err) => Some(err),
            }
        } else {
            None
        };

        let instance = match ManagedNodeRuntime::install_if_needed(
            &state.http,
            managed_dir,
            self.install_gate.as_ref(),
        )
        .await
        {
            Ok(instance) => Box::new(instance) as Box<dyn NodeRuntimeTrait>,
            Err(error) => {
                if let Some(system_error) = system_node_error {
                    if system_error.is::<ToolPermissionDenied>() {
                        return Err(system_error.context(format!(
                            "cannot use Zed managed Node.js: {error:#}; system Node.js unavailable"
                        )));
                    }
                    return Err(error.context(format!(
                        "cannot use Zed managed Node.js; system Node.js: {system_error:#}"
                    )));
                }
                return Err(error.context("cannot use Zed managed Node.js"));
            }
        };

        state.instance = Some(instance.boxed_clone());
        state.last_options = Some(options);
        Ok(instance)
    }

    pub async fn binary_path(&self) -> Result<PathBuf> {
        let instance = self.instance().await?;
        let path = instance.binary_path()?;
        require_install_gate(self.install_gate.as_ref(), &["Node.js"]).await?;
        Ok(path)
    }

    pub async fn run_npm_subcommand(
        &self,
        directory: Option<&Path>,
        subcommand: &str,
        args: &[&str],
    ) -> Result<Output> {
        self.run_npm_subcommand_for_tools(directory, subcommand, args, &["npm"])
            .await
    }

    pub async fn npm_package_installed_version(
        &self,
        local_package_directory: &Path,
        name: &str,
    ) -> Result<Option<Version>> {
        read_package_installed_version(local_package_directory.join("node_modules"), name).await
    }

    pub async fn npm_command(
        &self,
        prefix_dir: Option<&Path>,
        subcommand: &str,
        args: &[&str],
    ) -> Result<NpmCommand> {
        self.prepare_npm_command(prefix_dir, subcommand, args, &["npm"])
            .await
            .map(|(command, _)| command)
    }

    pub async fn npm_package_latest_version(&self, name: &str) -> Result<Version> {
        self.npm_package_latest_version_with_requirement(name, None)
            .await
    }

    pub async fn npm_package_latest_version_with_requirement(
        &self,
        name: &str,
        version_requirement: Option<&VersionReq>,
    ) -> Result<Version> {
        let output = self
            .run_npm_subcommand_for_tools(
                None,
                "info",
                &[
                    name,
                    "--json",
                    "--fetch-retry-mintimeout",
                    "2000",
                    "--fetch-retry-maxtimeout",
                    "5000",
                    "--fetch-timeout",
                    "5000",
                ],
                &[name],
            )
            .await?;

        let info: NpmInfo = deserialize_npm_info_from_response(&output.stdout).map_err(|e| {
            anyhow::anyhow!(
                "failed to parse npm info response: {e}\nstdout: {}",
                String::from_utf8_lossy(&output.stdout)
            )
        })?;
        let before = match npm_config_before(self, name)
            .await
            .context("getting npm before config")
        {
            Ok(before) => before,
            Err(error) if error.is::<ToolPermissionDenied>() => return Err(error),
            Err(error) => {
                log::error!("{error:#}");
                None
            }
        };
        let latest_dist_tag = info.dist_tags.latest.clone();
        let selected_version =
            select_npm_package_version(name, info, before.as_deref(), version_requirement)?;
        log::debug!(
            "selected latest npm package version package={name:?} version_requirement={version_requirement:?} before={before:?} dist_tag_latest={latest_dist_tag:?} selected={selected_version}"
        );
        Ok(selected_version)
    }

    pub async fn npm_install_packages(
        &self,
        directory: &Path,
        packages: &[(&str, &str)],
    ) -> Result<()> {
        if packages.is_empty() {
            return Ok(());
        }

        log::debug!(
            "installing npm packages directory={} packages={packages:?}",
            directory.display()
        );

        let arguments = build_npm_install_args(packages);
        let arguments = arguments.iter().map(String::as_str).collect::<Vec<_>>();

        let tools = packages.iter().map(|(name, _)| *name).collect::<Vec<_>>();
        self.run_npm_subcommand_for_tools(Some(directory), "install", &arguments, &tools)
            .await?;
        Ok(())
    }

    pub async fn npm_install_latest_packages(
        &self,
        directory: &Path,
        package_names: &[&str],
    ) -> Result<()> {
        // Let npm apply user config such as `before` and `min-release-age` during resolution.
        log::debug!(
            "installing latest npm packages directory={} packages={package_names:?}",
            directory.display()
        );
        let packages = package_names
            .iter()
            .map(|package_name| (*package_name, "latest"))
            .collect::<Vec<_>>();
        self.npm_install_packages(directory, &packages).await
    }

    pub async fn should_install_npm_package(
        &self,
        package_name: &str,
        local_executable_path: &Path,
        local_package_directory: &Path,
        version_strategy: VersionStrategy<'_>,
    ) -> bool {
        // In the case of the local system not having the package installed,
        // or in the instances where we fail to parse package.json data,
        // we attempt to install the package.
        if fs::metadata(local_executable_path).await.is_err() {
            log::debug!(
                "npm package cache miss package={package_name:?} reason=missing-executable executable={}",
                local_executable_path.display()
            );
            return true;
        }

        let Some(installed_version) = self
            .npm_package_installed_version(local_package_directory, package_name)
            .await
            .log_err()
            .flatten()
        else {
            log::debug!(
                "npm package cache miss package={package_name:?} reason=missing-installed-version package_dir={}",
                local_package_directory.display()
            );
            return true;
        };

        let version_strategy_label = match &version_strategy {
            VersionStrategy::Pin(version) => format!("pin:{version}"),
            VersionStrategy::Latest(version) => format!("latest:{version}"),
        };
        let should_install =
            should_install_npm_package_version(&installed_version, version_strategy);
        log::debug!(
            "npm package cache check package={package_name:?} installed={installed_version} strategy={version_strategy_label} should_install={should_install}"
        );
        should_install
    }

    async fn prepare_npm_command(
        &self,
        directory: Option<&Path>,
        subcommand: &str,
        args: &[&str],
        tools: &[&str],
    ) -> Result<(NpmCommand, bool)> {
        require_install_gate(self.install_gate.as_ref(), tools).await?;
        let http = self.state.lock().await.http.clone();
        let instance = self.instance().await?;
        let command = instance
            .npm_command(directory, http.proxy(), subcommand, args)
            .await?;
        require_install_gate(self.install_gate.as_ref(), tools).await?;
        Ok((command, instance.is_managed()))
    }

    async fn run_npm_subcommand_for_tools(
        &self,
        directory: Option<&Path>,
        subcommand: &str,
        args: &[&str],
        tools: &[&str],
    ) -> Result<Output> {
        let mut retry = false;
        loop {
            let (npm_command, managed) = self
                .prepare_npm_command(directory, subcommand, args, tools)
                .await?;
            let mut command = util::command::new_command(npm_command.path);
            command.kill_on_drop(true);
            command.args(npm_command.args);
            command.envs(npm_command.env);
            if let Some(directory) = directory {
                command.current_dir(directory);
            }
            let output = match command.output().await {
                Ok(output) => output,
                Err(_) if managed && !retry => {
                    retry = true;
                    continue;
                }
                Err(error) => return Err(error.into()),
            };
            anyhow::ensure!(
                output.status.success(),
                "failed to execute npm {subcommand} subcommand:\nstdout: {:?}\nstderr: {:?}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
            return Ok(output);
        }
    }
}

async fn require_install_gate(gate: Option<&NpmInstallGate>, tools: &[&str]) -> Result<()> {
    for tool in tools {
        anyhow::ensure!(
            match gate {
                Some(gate) => gate(tool.to_string()).await,
                None => false,
            },
            ToolPermissionDenied(tool.to_string())
        );
    }
    Ok(())
}

fn should_install_npm_package_version(
    installed_version: &Version,
    version_strategy: VersionStrategy<'_>,
) -> bool {
    match version_strategy {
        VersionStrategy::Pin(pinned_version) => installed_version != pinned_version,
        VersionStrategy::Latest(latest_version) => installed_version < latest_version,
    }
}

enum ArchiveType {
    TarGz,
    Zip,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub struct NpmInfo {
    #[serde(default)]
    dist_tags: NpmInfoDistTags,
    versions: Vec<Version>,
    #[serde(default, deserialize_with = "deserialize_npm_info_time")]
    time: HashMap<String, String>,
}

/// Parse NpmInfo from npm info --json output, handling both v11 and >= v12 formats.
fn deserialize_npm_info_from_response(data: &[u8]) -> Result<NpmInfo, serde_json::Error> {
    let value: serde_json::Value = serde_json::from_slice(data)?;

    // npm >= 12 returns an array with one object: [ { ... } ]
    if let serde_json::Value::Array(arr) = &value {
        if arr.len() == 1 {
            return NpmInfo::deserialize(&arr[0]);
        }
    }

    // npm <= v11 returns a bare JSON object: { ... }
    NpmInfo::deserialize(value)
}

#[derive(Debug, Deserialize, Default)]
pub struct NpmInfoDistTags {
    latest: Option<Version>,
}

// Some registries put non-string values in the `time` map: JFrog Artifactory emits
// `"unpublished": null`, and npm itself reports `unpublished` as an object when a
// package has had versions unpublished. Only version keys map to the RFC 3339 strings
// we read, so keep the string entries and drop the rest rather than failing to parse
// the entire `npm info` response (which would block language server installation).
fn deserialize_npm_info_time<'de, D>(deserializer: D) -> Result<HashMap<String, String>, D::Error>
where
    D: serde::Deserializer<'de>,
{
    let entries = HashMap::<String, serde_json::Value>::deserialize(deserializer)?;
    Ok(entries
        .into_iter()
        .filter_map(|(key, value)| match value {
            serde_json::Value::String(value) => Some((key, value)),
            _ => None,
        })
        .collect())
}

#[derive(Debug, Deserialize)]
struct NpmConfig {
    #[serde(default)]
    before: Option<String>,
}

async fn npm_config_before(node_runtime: &NodeRuntime, package: &str) -> Result<Option<String>> {
    // `npm config get before` renders Date values for display. The JSON config output keeps the
    // computed cutoff in the same ISO format used by `npm info --json` release times.
    let output = node_runtime
        .run_npm_subcommand_for_tools(None, "config", &["list", "--json"], &[package])
        .await?;
    let config: NpmConfig = serde_json::from_slice(&output.stdout)?;
    Ok(config
        .before
        .filter(|before| !before.trim().is_empty() && before != "null"))
}

fn select_npm_package_version(
    package_name: &str,
    mut info: NpmInfo,
    before: Option<&str>,
    version_requirement: Option<&VersionReq>,
) -> Result<Version> {
    if let Some(version_requirement) = version_requirement {
        info.versions
            .retain(|version| version_requirement.matches(version));
        info.versions.sort();
        info.dist_tags.latest = info
            .dist_tags
            .latest
            .take()
            .filter(|version| version_requirement.matches(version));
    }

    if let Some(before) = before
        && !info.time.is_empty()
    {
        let before_timestamp = DateTime::parse_from_rfc3339(before)
            .with_context(|| format!("parsing npm before config timestamp {before:?}"))?
            .with_timezone(&Utc);
        let latest_version = info.dist_tags.latest.as_ref();

        if let Some(version) = latest_version
            && npm_version_was_published_before(version, &info.time, &before_timestamp)?
        {
            return Ok(version.clone());
        }

        for version in info.versions.iter().rev() {
            if is_allowed_npm_version_before(
                version,
                latest_version,
                &info.time,
                &before_timestamp,
                version_requirement.is_some(),
            )? {
                return Ok(version.clone());
            }
        }

        bail!("no version found for npm package {package_name} before {before}");
    }

    info.dist_tags
        .latest
        .or_else(|| info.versions.pop())
        .with_context(|| format!("no version found for npm package {package_name}"))
}

fn is_allowed_npm_version_before(
    version: &Version,
    latest_version: Option<&Version>,
    published_at_by_version: &HashMap<String, String>,
    before: &DateTime<Utc>,
    allow_prereleases: bool,
) -> Result<bool> {
    if (!allow_prereleases && !version.pre.is_empty())
        || latest_version.is_some_and(|latest_version| version > latest_version)
    {
        return Ok(false);
    }

    npm_version_was_published_before(version, published_at_by_version, before)
}

fn npm_version_was_published_before(
    version: &Version,
    published_at_by_version: &HashMap<String, String>,
    before: &DateTime<Utc>,
) -> Result<bool> {
    let Some(published_at) = published_at_by_version.get(&version.to_string()) else {
        return Ok(false);
    };
    let published_at = DateTime::parse_from_rfc3339(published_at)
        .with_context(|| format!("parsing npm release timestamp for version {version}"))?
        .with_timezone(&Utc);
    Ok(&published_at <= before)
}

#[async_trait::async_trait]
trait NodeRuntimeTrait: Send + Sync {
    fn boxed_clone(&self) -> Box<dyn NodeRuntimeTrait>;
    fn binary_path(&self) -> Result<PathBuf>;

    fn is_managed(&self) -> bool {
        false
    }

    async fn npm_command(
        &self,
        prefix_dir: Option<&Path>,
        proxy: Option<&Url>,
        subcommand: &str,
        args: &[&str],
    ) -> Result<NpmCommand>;
}

#[derive(Clone)]
struct ManagedNodeRuntime {
    installation_path: PathBuf,
}

impl ManagedNodeRuntime {
    const VERSION: &str = "v24.11.0";

    #[cfg(not(windows))]
    const NODE_PATH: &str = "bin/node";
    #[cfg(windows)]
    const NODE_PATH: &str = "node.exe";

    #[cfg(not(windows))]
    const NPM_PATH: &str = "bin/npm";
    #[cfg(windows)]
    const NPM_PATH: &str = "node_modules/npm/bin/npm-cli.js";

    async fn install_if_needed(
        http: &Arc<dyn HttpClient>,
        node_containing_dir: &Path,
        install_gate: Option<&NpmInstallGate>,
    ) -> Result<Self> {
        log::info!("Node runtime install_if_needed");

        let os = match consts::OS {
            "macos" => "darwin",
            "linux" => "linux",
            "windows" => "win",
            other => bail!("Running on unsupported os: {other}"),
        };

        let arch = match consts::ARCH {
            "x86_64" => "x64",
            "aarch64" => "arm64",
            other => bail!("Running on unsupported architecture: {other}"),
        };

        let version = Self::VERSION;
        let folder_name = format!("node-{version}-{os}-{arch}");
        let node_dir = node_containing_dir.join(folder_name);
        let node_binary = node_dir.join(Self::NODE_PATH);
        let npm_file = node_dir.join(Self::NPM_PATH);
        let node_ca_certs = env::var(NODE_CA_CERTS_ENV_VAR).unwrap_or_else(|_| String::new());

        let valid = if fs::metadata(&node_binary)
            .await
            .is_ok_and(|metadata| metadata.is_file())
            && fs::metadata(&npm_file)
                .await
                .is_ok_and(|metadata| metadata.is_file())
        {
            require_install_gate(install_gate, &["Node.js"]).await?;
            let result = util::command::new_command(&node_binary)
                .env(NODE_CA_CERTS_ENV_VAR, node_ca_certs)
                .arg(npm_file)
                .arg("--version")
                .args(["--cache".into(), node_dir.join("cache")])
                .args(["--userconfig".into(), node_dir.join("blank_user_npmrc")])
                .args(["--globalconfig".into(), node_dir.join("blank_global_npmrc")])
                .output()
                .await;
            match result {
                Ok(output) => {
                    if output.status.success() {
                        true
                    } else {
                        log::warn!(
                            "Zed managed Node.js binary at {} failed check with output: {:?}",
                            node_binary.display(),
                            output
                        );
                        false
                    }
                }
                Err(err) => {
                    log::warn!(
                        "Zed managed Node.js binary at {} failed check. Error: {}",
                        node_binary.display(),
                        err
                    );
                    false
                }
            }
        } else {
            false
        };

        if valid {
            return Ok(Self {
                installation_path: node_dir,
            });
        }

        require_install_gate(install_gate, &["Node.js"]).await?;
        {
            _ = fs::remove_dir_all(&node_containing_dir).await;
            fs::create_dir(&node_containing_dir)
                .await
                .context("error creating node containing dir")?;

            let archive_type = match consts::OS {
                "macos" | "linux" => ArchiveType::TarGz,
                "windows" => ArchiveType::Zip,
                other => bail!("Running on unsupported os: {other}"),
            };

            let version = Self::VERSION;
            let file_name = format!(
                "node-{version}-{os}-{arch}.{extension}",
                extension = match archive_type {
                    ArchiveType::TarGz => "tar.gz",
                    ArchiveType::Zip => "zip",
                }
            );

            let url = format!("https://nodejs.org/dist/{version}/{file_name}");
            log::info!("Downloading Node.js binary from {url}");
            require_install_gate(install_gate, &["Node.js"]).await?;
            let mut response = http
                .get(&url, Default::default(), true)
                .await
                .context("error downloading Node binary tarball")?;
            log::info!("Download of Node.js complete, extracting...");

            let body = response.body_mut();
            match archive_type {
                ArchiveType::TarGz => {
                    let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
                    let archive = Archive::new(decompressed_bytes);
                    archive.unpack(&node_containing_dir).await?;
                }
                ArchiveType::Zip => extract_zip(&node_containing_dir, body).await?,
            }
            log::info!("Extracted Node.js to {}", node_containing_dir.display())
        }

        _ = fs::remove_dir_all(node_dir.join("cache")).await;

        _ = fs::create_dir(node_dir.join("cache")).await;
        _ = fs::write(node_dir.join("blank_user_npmrc"), []).await;
        _ = fs::write(node_dir.join("blank_global_npmrc"), []).await;

        anyhow::Ok(ManagedNodeRuntime {
            installation_path: node_dir,
        })
    }
}

fn path_with_node_binary_prepended(node_binary: &Path) -> Option<OsString> {
    let existing_path = env::var_os("PATH");
    let node_bin_dir = node_binary.parent().map(|dir| dir.as_os_str());
    match (existing_path, node_bin_dir) {
        (Some(existing_path), Some(node_bin_dir)) => {
            if let Ok(joined) = env::join_paths(
                [PathBuf::from(node_bin_dir)]
                    .into_iter()
                    .chain(env::split_paths(&existing_path)),
            ) {
                Some(joined)
            } else {
                Some(existing_path)
            }
        }
        (Some(existing_path), None) => Some(existing_path),
        (None, Some(node_bin_dir)) => Some(node_bin_dir.to_owned()),
        _ => None,
    }
}

#[async_trait::async_trait]
impl NodeRuntimeTrait for ManagedNodeRuntime {
    fn boxed_clone(&self) -> Box<dyn NodeRuntimeTrait> {
        Box::new(self.clone())
    }

    fn binary_path(&self) -> Result<PathBuf> {
        Ok(self.installation_path.join(Self::NODE_PATH))
    }

    fn is_managed(&self) -> bool {
        true
    }

    async fn npm_command(
        &self,
        prefix_dir: Option<&Path>,
        proxy: Option<&Url>,
        subcommand: &str,
        args: &[&str],
    ) -> Result<NpmCommand> {
        let node_binary = self.installation_path.join(Self::NODE_PATH);
        let npm_file = self.installation_path.join(Self::NPM_PATH);

        anyhow::ensure!(
            smol::fs::metadata(&node_binary).await.is_ok(),
            "missing node binary file"
        );
        anyhow::ensure!(
            smol::fs::metadata(&npm_file).await.is_ok(),
            "missing npm file"
        );

        let command_args = build_npm_command_args(
            Some(&npm_file),
            prefix_dir,
            &self.installation_path.join("cache"),
            Some(&self.installation_path.join("blank_user_npmrc")),
            Some(&self.installation_path.join("blank_global_npmrc")),
            proxy,
            subcommand,
            args,
        );
        let command_env = npm_command_env(&node_binary);

        Ok(NpmCommand {
            path: node_binary,
            args: command_args,
            env: command_env,
        })
    }
}

#[derive(Debug, Clone)]
pub struct SystemNodeRuntime {
    node: PathBuf,
    npm: PathBuf,
    scratch_dir: PathBuf,
}

impl SystemNodeRuntime {
    const MIN_VERSION: semver::Version = Version::new(22, 0, 0);
    async fn new(
        node: PathBuf,
        npm: PathBuf,
        install_gate: Option<&NpmInstallGate>,
    ) -> Result<Self> {
        require_install_gate(install_gate, &["Node.js"]).await?;
        let output = util::command::new_command(&node)
            .kill_on_drop(true)
            .arg("--version")
            .output()
            .await
            .with_context(|| format!("running node from {:?}", node))?;
        if !output.status.success() {
            anyhow::bail!(
                "failed to run node --version. stdout: {}, stderr: {}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr),
            );
        }
        let version_str = String::from_utf8_lossy(&output.stdout);
        let version = semver::Version::parse(version_str.trim().trim_start_matches('v'))?;
        if version < Self::MIN_VERSION {
            anyhow::bail!(
                "node at {} is too old. want: {}, got: {}",
                node.to_string_lossy(),
                Self::MIN_VERSION,
                version
            )
        }

        let scratch_dir = paths::data_dir().join("node");
        fs::create_dir(&scratch_dir).await.ok();
        _ = fs::remove_dir_all(scratch_dir.join("cache")).await;
        fs::create_dir(scratch_dir.join("cache")).await.ok();

        Ok(Self {
            node,
            npm,
            scratch_dir,
        })
    }

    async fn detect(install_gate: Option<&NpmInstallGate>) -> Result<Self> {
        let node = which::which("node").context("system Node.js wasn't found on PATH")?;
        let npm = which::which("npm").context("system npm wasn't found on PATH")?;
        Self::new(node, npm, install_gate)
            .await
            .context("checking system Node.js")
    }
}

#[async_trait::async_trait]
impl NodeRuntimeTrait for SystemNodeRuntime {
    fn boxed_clone(&self) -> Box<dyn NodeRuntimeTrait> {
        Box::new(self.clone())
    }

    fn binary_path(&self) -> Result<PathBuf> {
        Ok(self.node.clone())
    }

    async fn npm_command(
        &self,
        prefix_dir: Option<&Path>,
        proxy: Option<&Url>,
        subcommand: &str,
        args: &[&str],
    ) -> Result<NpmCommand> {
        let command_args = build_npm_command_args(
            None,
            prefix_dir,
            &self.scratch_dir.join("cache"),
            None,
            None,
            proxy,
            subcommand,
            args,
        );
        let command_env = npm_command_env(&self.node);

        Ok(NpmCommand {
            path: self.npm.clone(),
            args: command_args,
            env: command_env,
        })
    }
}

pub async fn read_package_installed_version(
    node_module_directory: PathBuf,
    name: &str,
) -> Result<Option<Version>> {
    let package_json_path = node_module_directory.join(name).join("package.json");

    let mut file = match fs::File::open(package_json_path).await {
        Ok(file) => file,
        Err(err) => {
            if err.kind() == io::ErrorKind::NotFound {
                return Ok(None);
            }

            Err(err)?
        }
    };

    #[derive(Deserialize)]
    struct PackageJson {
        version: Version,
    }

    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let package_json: PackageJson = serde_json::from_str(&contents)?;
    Ok(Some(package_json.version))
}

pub async fn read_package_executable(
    node_module_directory: PathBuf,
    name: &str,
) -> Result<PathBuf> {
    let package_directory = node_module_directory.join(name);

    #[derive(Deserialize)]
    #[serde(untagged)]
    enum Bin {
        Path(String),
        Named(HashMap<String, String>),
    }

    #[derive(Deserialize)]
    struct PackageJson {
        bin: Option<Bin>,
    }

    let package_json_path = package_directory.join("package.json");
    let mut file = fs::File::open(&package_json_path)
        .await
        .with_context(|| format!("opening {}", package_json_path.display()))?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let package_json: PackageJson = serde_json::from_str(&contents)
        .with_context(|| format!("parsing {}", package_json_path.display()))?;

    let relative_path = match package_json.bin {
        Some(Bin::Path(path)) => path,
        Some(Bin::Named(bins)) => {
            let unscoped_name = name.rsplit('/').next().unwrap_or(name);
            let path = if bins.len() == 1 {
                bins.values().next()
            } else {
                bins.get(unscoped_name)
            };
            path.with_context(|| {
                format!("npm package {name} declares no executable named {unscoped_name}")
            })?
            .clone()
        }
        None => bail!("npm package {name} declares no executable"),
    };

    Ok(package_directory.join(relative_path))
}

#[derive(Clone)]
pub struct UnavailableNodeRuntime;

#[async_trait::async_trait]
impl NodeRuntimeTrait for UnavailableNodeRuntime {
    fn boxed_clone(&self) -> Box<dyn NodeRuntimeTrait> {
        Box::new(self.clone())
    }
    fn binary_path(&self) -> Result<PathBuf> {
        bail!("`node` settings do not allow any way to use Node.js")
    }

    async fn npm_command(
        &self,
        _: Option<&Path>,
        _proxy: Option<&Url>,
        _subcommand: &str,
        _args: &[&str],
    ) -> Result<NpmCommand> {
        bail!("`node` settings do not allow any way to use Node.js")
    }
}

fn proxy_argument(proxy: Option<&Url>) -> Option<String> {
    let mut proxy = proxy.cloned()?;
    // Map proxy settings from `http://localhost:10809` to `http://127.0.0.1:10809`
    // NodeRuntime without environment information can not parse `localhost`
    // correctly.
    // TODO: map to `[::1]` if we are using ipv6
    if matches!(proxy.host(), Some(Host::Domain(domain)) if domain.eq_ignore_ascii_case("localhost"))
    {
        // When localhost is a valid Host, so is `127.0.0.1`
        let _ = proxy.set_ip_host(IpAddr::V4(Ipv4Addr::LOCALHOST));
    }

    Some(proxy.as_str().to_string())
}

fn build_npm_install_args(packages: &[(&str, &str)]) -> Vec<String> {
    packages
        .iter()
        .map(|(name, version)| format!("{name}@{version}"))
        .chain(
            [
                "--fetch-retry-mintimeout",
                "2000",
                "--fetch-retry-maxtimeout",
                "5000",
                "--fetch-timeout",
                "5000",
            ]
            .into_iter()
            .map(String::from),
        )
        .collect()
}

fn build_npm_command_args(
    entrypoint: Option<&Path>,
    prefix_dir: Option<&Path>,
    cache_dir: &Path,
    user_config: Option<&Path>,
    global_config: Option<&Path>,
    proxy: Option<&Url>,
    subcommand: &str,
    args: &[&str],
) -> Vec<String> {
    let mut command_args = Vec::new();
    if let Some(entrypoint) = entrypoint {
        command_args.push(entrypoint.to_string_lossy().into_owned());
    }
    if let Some(prefix_dir) = prefix_dir {
        command_args.push("--prefix".into());
        command_args.push(prefix_dir.to_string_lossy().into_owned());
    }
    command_args.push(subcommand.to_string());
    command_args.push(format!("--cache={}", cache_dir.display()));
    if let Some(user_config) = user_config {
        command_args.push("--userconfig".into());
        command_args.push(user_config.to_string_lossy().into_owned());
    }
    if let Some(global_config) = global_config {
        command_args.push("--globalconfig".into());
        command_args.push(global_config.to_string_lossy().into_owned());
    }
    if let Some(proxy_arg) = proxy_argument(proxy) {
        command_args.push("--proxy".into());
        command_args.push(proxy_arg);
    }
    let (options, positional) = args.split_at(
        args.iter()
            .position(|arg| *arg == "--")
            .unwrap_or(args.len()),
    );
    command_args.extend(options.iter().map(|arg| arg.to_string()));
    if matches!(
        subcommand,
        "install"
            | "add"
            | "i"
            | "in"
            | "ins"
            | "inst"
            | "insta"
            | "instal"
            | "isnt"
            | "isnta"
            | "isntal"
            | "isntall"
    ) {
        command_args
            .extend(["--no-package-lock", "--save-exact", "--ignore-scripts"].map(String::from));
    }
    command_args.extend(positional.iter().map(|arg| arg.to_string()));
    command_args
}

pub fn npm_command_env(node_binary: &Path) -> HashMap<String, String> {
    let mut command_env = HashMap::new();
    let env_path = path_with_node_binary_prepended(node_binary).unwrap_or_default();
    command_env.insert("PATH".into(), env_path.to_string_lossy().into_owned());

    if let Ok(node_ca_certs) = env::var(NODE_CA_CERTS_ENV_VAR) {
        if !node_ca_certs.is_empty() {
            command_env.insert(NODE_CA_CERTS_ENV_VAR.to_string(), node_ca_certs);
        }
    }

    #[cfg(windows)]
    {
        if let Some(val) = env::var("SYSTEMROOT")
            .context("Missing environment variable: SYSTEMROOT!")
            .log_err()
        {
            command_env.insert("SYSTEMROOT".into(), val);
        }
        if let Some(val) = env::var("ComSpec")
            .context("Missing environment variable: ComSpec!")
            .log_err()
        {
            command_env.insert("ComSpec".into(), val);
        }
    }

    command_env
}

#[cfg(test)]
mod tests {
    use std::path::Path;
    use std::sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    };
    #[cfg(unix)]
    use std::{
        os::unix::fs::symlink,
        path::PathBuf,
        task::Poll,
        time::{Duration, Instant},
    };

    use anyhow::{Result, bail};
    use futures::FutureExt as _;
    use http_client::Url;
    #[cfg(unix)]
    use http_client::http::HeaderValue;
    use semver::{Version, VersionReq};
    #[cfg(unix)]
    use std::sync::atomic::AtomicBool;
    #[cfg(unix)]
    use util::{ResultExt as _, ToolPermissionDenied, command::new_command};

    use super::{
        NodeBinaryOptions, NodeRuntime, NpmInfo, NpmInstallGate, VersionStrategy,
        build_npm_command_args, build_npm_install_args, deserialize_npm_info_from_response,
        proxy_argument, select_npm_package_version, should_install_npm_package_version,
    };

    // Map localhost to 127.0.0.1
    // NodeRuntime without environment information can not parse `localhost` correctly.
    #[test]
    fn test_proxy_argument_map_localhost_proxy() {
        const CASES: [(&str, &str); 4] = [
            // Map localhost to 127.0.0.1
            ("http://localhost:9090/", "http://127.0.0.1:9090/"),
            ("https://google.com/", "https://google.com/"),
            (
                "http://username:password@proxy.thing.com:8080/",
                "http://username:password@proxy.thing.com:8080/",
            ),
            // Test when localhost is contained within a different part of the URL
            (
                "http://username:localhost@localhost:8080/",
                "http://username:localhost@127.0.0.1:8080/",
            ),
        ];

        for (proxy, mapped_proxy) in CASES {
            let proxy = Url::parse(proxy).unwrap();
            let proxy = proxy_argument(Some(&proxy)).expect("Proxy was not passed correctly");
            assert_eq!(
                proxy, mapped_proxy,
                "Incorrectly mapped localhost to 127.0.0.1"
            );
        }
    }

    #[test]
    fn test_build_npm_command_args_inserts_prefix_before_subcommand() {
        let args = build_npm_command_args(
            None,
            Some(Path::new("/tmp/zed-prefix")),
            Path::new("/tmp/cache"),
            None,
            None,
            None,
            "exec",
            &["--yes", "--", "agent-package"],
        );

        assert_eq!(
            args,
            vec![
                "--prefix".to_string(),
                "/tmp/zed-prefix".to_string(),
                "exec".to_string(),
                "--cache=/tmp/cache".to_string(),
                "--yes".to_string(),
                "--".to_string(),
                "agent-package".to_string(),
            ]
        );
    }

    #[test]
    fn test_build_npm_command_args_keeps_entrypoint_before_prefix() {
        let args = build_npm_command_args(
            Some(Path::new("/tmp/npm-cli.js")),
            Some(Path::new("/tmp/zed-prefix")),
            Path::new("/tmp/cache"),
            None,
            None,
            None,
            "exec",
            &["--yes"],
        );

        assert_eq!(
            args,
            vec![
                "/tmp/npm-cli.js".to_string(),
                "--prefix".to_string(),
                "/tmp/zed-prefix".to_string(),
                "exec".to_string(),
                "--cache=/tmp/cache".to_string(),
                "--yes".to_string(),
            ]
        );
    }

    #[test]
    fn test_build_npm_command_args_secures_raw_installs() {
        let args = build_npm_command_args(
            None,
            None,
            Path::new("cache"),
            None,
            None,
            None,
            "install",
            &["agent-package@0.0.0 - 1.2.3", "--before=2026-01-01"],
        );
        assert_eq!(
            args,
            [
                "install",
                "--cache=cache",
                "agent-package@0.0.0 - 1.2.3",
                "--before=2026-01-01",
                "--no-package-lock",
                "--save-exact",
                "--ignore-scripts",
            ]
        );
        for subcommand in ["install", "i", "add"] {
            let args = build_npm_command_args(
                None,
                None,
                Path::new("cache"),
                None,
                None,
                None,
                subcommand,
                &[
                    "--ignore-scripts=false",
                    "--package-lock",
                    "--save-exact=false",
                    "--",
                    "agent-package@0.0.0 - 1.2.3",
                ],
            );
            assert_eq!(
                args,
                [
                    subcommand,
                    "--cache=cache",
                    "--ignore-scripts=false",
                    "--package-lock",
                    "--save-exact=false",
                    "--no-package-lock",
                    "--save-exact",
                    "--ignore-scripts",
                    "--",
                    "agent-package@0.0.0 - 1.2.3",
                ]
            );
        }
    }

    #[test]
    fn test_latest_version_strategy_accepts_newer_installed_versions() -> Result<()> {
        let target_version = Version::parse("2.0.0")?;

        assert!(!should_install_npm_package_version(
            &Version::parse("2.0.0")?,
            VersionStrategy::Latest(&target_version)
        ));
        assert!(should_install_npm_package_version(
            &Version::parse("1.0.0")?,
            VersionStrategy::Latest(&target_version)
        ));
        assert!(!should_install_npm_package_version(
            &Version::parse("3.0.0")?,
            VersionStrategy::Latest(&target_version)
        ));

        Ok(())
    }

    #[test]
    fn test_select_npm_package_version_uses_dist_tag_without_before() -> Result<()> {
        let info: NpmInfo = serde_json::from_str(
            r#"{
                "dist-tags": { "latest": "3.0.0" },
                "versions": ["1.0.0", "2.0.0", "3.0.0"],
                "time": {
                    "1.0.0": "2024-01-01T00:00:00.000Z",
                    "2.0.0": "2024-02-01T00:00:00.000Z",
                    "3.0.0": "2024-03-01T00:00:00.000Z"
                }
            }"#,
        )?;

        assert_eq!(
            select_npm_package_version("test-package", info, None, None)?,
            Version::parse("3.0.0")?
        );
        Ok(())
    }

    #[test]
    fn test_npm_info_skips_non_string_time_entries() -> Result<()> {
        // Registries such as JFrog Artifactory include `"unpublished": null` in `time`;
        // parsing must tolerate this rather than rejecting the whole response.
        let info: NpmInfo = serde_json::from_str(
            r#"{
                "dist-tags": { "latest": "2.0.0" },
                "versions": ["1.0.0", "2.0.0"],
                "time": {
                    "unpublished": null,
                    "created": "2024-01-01T00:00:00.000Z",
                    "modified": "2024-02-01T00:00:00.000Z",
                    "1.0.0": "2024-01-01T00:00:00.000Z",
                    "2.0.0": "2024-02-01T00:00:00.000Z"
                }
            }"#,
        )?;

        assert_eq!(
            select_npm_package_version(
                "test-package",
                info,
                Some("2024-02-15T00:00:00.000Z"),
                None
            )?,
            Version::parse("2.0.0")?
        );
        Ok(())
    }

    #[test]
    fn test_select_npm_package_version_uses_latest_before_npm_before_config() -> Result<()> {
        let info: NpmInfo = serde_json::from_str(
            r#"{
                "dist-tags": { "latest": "3.0.0" },
                "versions": ["1.0.0", "2.0.0", "3.0.0"],
                "time": {
                    "1.0.0": "2024-01-01T00:00:00.000Z",
                    "2.0.0": "2024-02-01T00:00:00.000Z",
                    "3.0.0": "2024-03-01T00:00:00.000Z"
                }
            }"#,
        )?;

        assert_eq!(
            select_npm_package_version(
                "test-package",
                info,
                Some("2024-02-15T00:00:00.000Z"),
                None
            )?,
            Version::parse("2.0.0")?
        );
        Ok(())
    }

    #[test]
    fn test_select_npm_package_version_keeps_allowed_latest_dist_tag() -> Result<()> {
        let info: NpmInfo = serde_json::from_str(
            r#"{
                "dist-tags": { "latest": "2.0.0" },
                "versions": ["1.0.0", "2.0.0", "3.0.0"],
                "time": {
                    "1.0.0": "2024-01-01T00:00:00.000Z",
                    "2.0.0": "2024-02-01T00:00:00.000Z",
                    "3.0.0": "2024-03-01T00:00:00.000Z"
                }
            }"#,
        )?;

        assert_eq!(
            select_npm_package_version(
                "test-package",
                info,
                Some("2024-02-15T00:00:00.000Z"),
                None
            )?,
            Version::parse("2.0.0")?
        );
        Ok(())
    }

    #[test]
    fn test_select_npm_package_version_keeps_allowed_prerelease_latest_dist_tag() -> Result<()> {
        let info: NpmInfo = serde_json::from_str(
            r#"{
                "dist-tags": { "latest": "2.0.0-beta.1" },
                "versions": ["1.0.0", "2.0.0-beta.1"],
                "time": {
                    "1.0.0": "2024-01-01T00:00:00.000Z",
                    "2.0.0-beta.1": "2024-02-01T00:00:00.000Z"
                }
            }"#,
        )?;

        assert_eq!(
            select_npm_package_version(
                "test-package",
                info,
                Some("2024-02-15T00:00:00.000Z"),
                None
            )?,
            Version::parse("2.0.0-beta.1")?
        );
        Ok(())
    }

    #[test]
    fn test_select_npm_package_version_ignores_prereleases_before_cutoff() -> Result<()> {
        let info: NpmInfo = serde_json::from_str(
            r#"{
                "dist-tags": { "latest": "2.0.0" },
                "versions": ["1.0.0", "2.0.0-beta.1", "2.0.0"],
                "time": {
                    "1.0.0": "2024-01-01T00:00:00.000Z",
                    "2.0.0-beta.1": "2024-02-01T00:00:00.000Z",
                    "2.0.0": "2024-03-01T00:00:00.000Z"
                }
            }"#,
        )?;

        assert_eq!(
            select_npm_package_version(
                "test-package",
                info,
                Some("2024-02-15T00:00:00.000Z"),
                None
            )?,
            Version::parse("1.0.0")?
        );
        Ok(())
    }

    #[test]
    fn test_select_npm_package_version_ignores_versions_above_latest_dist_tag() -> Result<()> {
        let info: NpmInfo = serde_json::from_str(
            r#"{
                "dist-tags": { "latest": "2.0.0" },
                "versions": ["1.0.0", "2.0.0", "3.0.0"],
                "time": {
                    "1.0.0": "2024-01-01T00:00:00.000Z",
                    "2.0.0": "2024-03-01T00:00:00.000Z",
                    "3.0.0": "2024-02-01T00:00:00.000Z"
                }
            }"#,
        )?;

        assert_eq!(
            select_npm_package_version(
                "test-package",
                info,
                Some("2024-02-15T00:00:00.000Z"),
                None
            )?,
            Version::parse("1.0.0")?
        );
        Ok(())
    }

    #[test]
    fn test_select_npm_package_version_errors_when_no_version_matches_before() -> Result<()> {
        let info: NpmInfo = serde_json::from_str(
            r#"{
                "dist-tags": { "latest": "2.0.0" },
                "versions": ["1.0.0", "2.0.0"],
                "time": {
                    "1.0.0": "2024-01-01T00:00:00.000Z",
                    "2.0.0": "2024-02-01T00:00:00.000Z"
                }
            }"#,
        )?;

        let Err(error) = select_npm_package_version(
            "test-package",
            info,
            Some("2023-12-01T00:00:00.000Z"),
            None,
        ) else {
            bail!("expected cutoff to reject all package versions");
        };
        assert_eq!(
            error.to_string(),
            "no version found for npm package test-package before 2023-12-01T00:00:00.000Z"
        );
        Ok(())
    }

    #[test]
    fn test_select_npm_package_version_selects_latest_matching_requirement() -> Result<()> {
        let info: NpmInfo = serde_json::from_str(
            r#"{
                "dist-tags": { "latest": "7.0.0" },
                "versions": ["6.0.3", "7.0.0", "5.9.3", "6.0.2"]
            }"#,
        )?;
        let version_requirement = VersionReq::parse("^6")?;

        assert_eq!(
            select_npm_package_version("test-package", info, None, Some(&version_requirement))?,
            Version::parse("6.0.3")?
        );
        Ok(())
    }

    #[test]
    fn test_select_npm_package_version_applies_before_to_matching_versions() -> Result<()> {
        let info: NpmInfo = serde_json::from_str(
            r#"{
                "dist-tags": { "latest": "7.0.0" },
                "versions": ["6.0.3", "7.0.0", "6.0.2"],
                "time": {
                    "6.0.2": "2024-02-01T00:00:00.000Z",
                    "6.0.3": "2024-03-01T00:00:00.000Z",
                    "7.0.0": "2024-04-01T00:00:00.000Z"
                }
            }"#,
        )?;
        let version_requirement = VersionReq::parse("^6")?;

        assert_eq!(
            select_npm_package_version(
                "test-package",
                info,
                Some("2024-02-15T00:00:00.000Z"),
                Some(&version_requirement),
            )?,
            Version::parse("6.0.2")?
        );
        Ok(())
    }

    #[test]
    fn test_select_npm_package_version_allows_requested_prerelease_before_cutoff() -> Result<()> {
        let info: NpmInfo = serde_json::from_str(
            r#"{
                "dist-tags": { "latest": "7.0.0" },
                "versions": ["7.1.0-beta.1", "7.1.0-beta.2", "7.0.0"],
                "time": {
                    "7.0.0": "2024-01-01T00:00:00.000Z",
                    "7.1.0-beta.1": "2024-02-01T00:00:00.000Z",
                    "7.1.0-beta.2": "2024-03-01T00:00:00.000Z"
                }
            }"#,
        )?;
        let version_requirement = VersionReq::parse(">=7.1.0-beta.1, <7.1.0")?;

        assert_eq!(
            select_npm_package_version(
                "test-package",
                info,
                Some("2024-02-15T00:00:00.000Z"),
                Some(&version_requirement),
            )?,
            Version::parse("7.1.0-beta.1")?
        );
        Ok(())
    }

    #[test]
    fn test_select_npm_package_version_errors_without_matching_version() -> Result<()> {
        let info: NpmInfo = serde_json::from_str(
            r#"{
                "dist-tags": { "latest": "7.0.0" },
                "versions": ["5.9.3", "7.0.0"]
            }"#,
        )?;
        let version_requirement = VersionReq::parse("^6")?;

        let error =
            select_npm_package_version("test-package", info, None, Some(&version_requirement))
                .expect_err("expected version requirement to reject all package versions");
        assert_eq!(
            error.to_string(),
            "no version found for npm package test-package"
        );
        Ok(())
    }

    #[test]
    fn test_pinned_version_strategy_replaces_different_installed_version() -> Result<()> {
        let pinned_version = Version::parse("6.0.3")?;

        assert!(!should_install_npm_package_version(
            &pinned_version,
            VersionStrategy::Pin(&pinned_version)
        ));
        assert!(should_install_npm_package_version(
            &Version::parse("7.0.0")?,
            VersionStrategy::Pin(&pinned_version)
        ));
        Ok(())
    }

    #[test]
    fn test_deserialize_npm_info_npm11_format() -> Result<()> {
        let json = r#"{
            "dist-tags": { "latest": "3.0.0" },
            "versions": ["1.0.0", "2.0.0", "3.0.0"]
        }"#;

        let info = deserialize_npm_info_from_response(json.as_bytes())?;
        assert_eq!(info.dist_tags.latest, Some(Version::parse("3.0.0")?));
        assert_eq!(
            info.versions,
            vec![
                Version::parse("1.0.0")?,
                Version::parse("2.0.0")?,
                Version::parse("3.0.0")?
            ]
        );
        Ok(())
    }

    #[test]
    fn test_deserialize_npm_v12_format() -> Result<()> {
        let json = r#"[
            {
                "dist-tags": { "latest": "3.0.0" },
                "versions": ["1.0.0", "2.0.0", "3.0.0"]
            }
        ]"#;

        let info = deserialize_npm_info_from_response(json.as_bytes())?;
        assert_eq!(info.dist_tags.latest, Some(Version::parse("3.0.0")?));
        assert_eq!(
            info.versions,
            vec![
                Version::parse("1.0.0")?,
                Version::parse("2.0.0")?,
                Version::parse("3.0.0")?
            ]
        );
        Ok(())
    }

    #[test]
    fn test_npm_install_args_disable_lifecycle_scripts_and_pin_versions() {
        let args = build_npm_install_args(&[("prettier", "3.0.0"), ("typescript", "5.4.2")]);
        let args = args.iter().map(String::as_str).collect::<Vec<_>>();
        assert_eq!(
            build_npm_command_args(
                None,
                None,
                Path::new("cache"),
                None,
                None,
                None,
                "install",
                &args
            ),
            [
                "install",
                "--cache=cache",
                "prettier@3.0.0",
                "typescript@5.4.2",
                "--fetch-retry-mintimeout",
                "2000",
                "--fetch-retry-maxtimeout",
                "5000",
                "--fetch-timeout",
                "5000",
                "--no-package-lock",
                "--save-exact",
                "--ignore-scripts",
            ]
        );
    }

    #[cfg(unix)]
    #[test]
    fn test_managed_node_reuses_approved_cache_without_repairs() -> Result<()> {
        smol::block_on(async {
            let directory = TestDirectory::new()?;
            let os = if cfg!(target_os = "macos") {
                "darwin"
            } else {
                "linux"
            };
            let arch = if cfg!(target_arch = "aarch64") {
                "arm64"
            } else {
                "x64"
            };
            let installation = directory.0.join(format!("node-v24.11.0-{os}-{arch}"));
            std::fs::create_dir_all(installation.join("bin"))?;
            let node = installation.join("bin/node");
            let npm = installation.join("bin/npm");
            symlink("/bin/sh", &node)?;
            let marker = directory.0.join("probe");
            std::fs::write(
                &npm,
                format!("printf '10.0.0\\n'\nprintf probe > {marker:?}\n"),
            )?;
            let http = Arc::new(NoNetworkHttpClient) as Arc<dyn http_client::HttpClient>;
            let mut options = NodeBinaryOptions {
                allow_binary_downloads: true,
                ..NodeBinaryOptions::default()
            };
            let (mut sender, receiver) = watch::channel(Some(options.clone()));
            let runtime = NodeRuntime::new(http.clone(), None, receiver, None);
            assert!(
                runtime
                    .instance_with_managed_dir(&directory.0)
                    .await
                    .err()
                    .unwrap()
                    .is::<ToolPermissionDenied>()
            );
            assert!(!marker.exists());
            let runtime =
                runtime.with_install_gate(Some(Arc::new(|_| futures::future::ready(true).boxed())));
            assert_eq!(
                runtime
                    .instance_with_managed_dir(&directory.0)
                    .await?
                    .binary_path()?,
                node
            );

            std::fs::create_dir(installation.join("cache"))?;
            std::fs::write(installation.join("cache/keep"), "cached")?;
            std::fs::write(installation.join("blank_user_npmrc"), "user")?;
            std::fs::write(installation.join("blank_global_npmrc"), "global")?;
            options.allow_binary_downloads = false;
            sender.send(Some(options.clone()))?;
            assert_eq!(
                runtime
                    .instance_with_managed_dir(&directory.0)
                    .await?
                    .binary_path()?,
                node
            );

            options.use_paths = Some((directory.0.join("missing-node"), npm.clone()));
            sender.send(Some(options.clone()))?;
            assert!(
                runtime
                    .instance_with_managed_dir(&directory.0)
                    .await
                    .and_then(|instance| instance.binary_path())
                    .is_err()
            );
            options.use_paths = None;
            sender.send(Some(options.clone()))?;
            assert_eq!(
                runtime
                    .instance_with_managed_dir(&directory.0)
                    .await?
                    .binary_path()?,
                node
            );

            let cold_runtime = NodeRuntime::new(http, None, watch::channel(Some(options)).1, None);
            std::fs::remove_file(&npm)?;
            assert!(
                cold_runtime
                    .instance_with_managed_dir(&directory.0)
                    .await
                    .and_then(|instance| instance.binary_path())
                    .is_err()
            );
            std::fs::write(&npm, "exit 1\n")?;
            assert!(
                cold_runtime
                    .instance_with_managed_dir(&directory.0)
                    .await
                    .and_then(|instance| instance.binary_path())
                    .is_err()
            );
            assert_eq!(std::fs::read_to_string(&npm)?, "exit 1\n");
            std::fs::write(&npm, "printf '10.0.0\\n'\n")?;
            let cold_runtime = cold_runtime.with_install_gate(runtime.install_gate.clone());
            assert_eq!(
                cold_runtime
                    .instance_with_managed_dir(&directory.0)
                    .await?
                    .binary_path()?,
                node
            );
            assert_eq!(
                std::fs::read_to_string(installation.join("cache/keep"))?,
                "cached"
            );
            assert_eq!(
                std::fs::read_to_string(installation.join("blank_user_npmrc"))?,
                "user"
            );
            assert_eq!(
                std::fs::read_to_string(installation.join("blank_global_npmrc"))?,
                "global"
            );
            assert!(
                NodeRuntime::unavailable()
                    .instance_with_managed_dir(&directory.0)
                    .await
                    .and_then(|instance| instance.binary_path())
                    .is_err()
            );
            Ok(())
        })
    }

    #[cfg(unix)]
    #[test]
    fn test_managed_npm_cancellation_kills_child() -> Result<()> {
        smol::block_on(assert_npm_cancellation(true))
    }

    #[cfg(unix)]
    #[test]
    fn test_system_npm_cancellation_kills_child() -> Result<()> {
        smol::block_on(assert_npm_cancellation(false))
    }

    #[cfg(unix)]
    #[test]
    fn test_npm_authorization_is_rechecked_after_preparation() -> Result<()> {
        smol::block_on(async {
            let directory = TestDirectory::new()?;
            let admitted = directory.0.join("admitted");
            let allowed = Arc::new(AtomicBool::new(true));
            let revoke = Arc::new(AtomicBool::new(true));
            let runtime = gated_node_runtime(Some(Arc::new({
                let allowed = allowed.clone();
                move |_| futures::future::ready(allowed.load(Ordering::SeqCst)).boxed()
            })));
            runtime.state.lock().await.instance = Some(Box::new(PreparingNodeRuntime {
                admitted: admitted.clone(),
                allowed: allowed.clone(),
                revoke: revoke.clone(),
            }));

            for subcommand in ["install", "exec", "run-script", "config"] {
                allowed.store(true, Ordering::SeqCst);
                revoke.store(true, Ordering::SeqCst);
                let error = runtime
                    .run_npm_subcommand(None, subcommand, &[])
                    .await
                    .unwrap_err();
                assert!(error.is::<ToolPermissionDenied>());
                assert_eq!(error.to_string(), util::downloads_disabled_error("npm"));
                assert!(!admitted.exists());
            }
            allowed.store(true, Ordering::SeqCst);
            revoke.store(true, Ordering::SeqCst);
            let error = runtime.npm_command(None, "exec", &[]).await.unwrap_err();
            assert_eq!(error.to_string(), util::downloads_disabled_error("npm"));
            assert!(!admitted.exists());

            allowed.store(true, Ordering::SeqCst);
            revoke.store(true, Ordering::SeqCst);
            let error = runtime
                .npm_install_packages(&directory.0, &[("package", "1.0.0")])
                .await
                .unwrap_err();
            assert_eq!(error.to_string(), util::downloads_disabled_error("package"));
            assert!(!admitted.exists());

            allowed.store(true, Ordering::SeqCst);
            let denied = runtime.with_install_gate(None);
            assert_eq!(
                denied.binary_path().await.unwrap_err().to_string(),
                util::downloads_disabled_error("Node.js")
            );
            assert_eq!(
                denied
                    .run_npm_subcommand(None, "exec", &[])
                    .await
                    .unwrap_err()
                    .to_string(),
                util::downloads_disabled_error("npm")
            );
            assert!(!admitted.exists());
            runtime.run_npm_subcommand(None, "exec", &[]).await?;
            assert_eq!(std::fs::read_to_string(&admitted)?, "admitted");
            Ok(())
        })
    }

    #[cfg(unix)]
    #[test]
    fn test_managed_node_acquisition_requires_live_gate_without_caching_denial() -> Result<()> {
        smol::block_on(async {
            let directory = TestDirectory::new()?;
            let http = Arc::new(RecordingHttpClient(AtomicUsize::new(0)));
            let (mut options, receiver) = watch::channel(Some(NodeBinaryOptions {
                allow_binary_downloads: true,
                ..NodeBinaryOptions::default()
            }));
            let runtime = NodeRuntime::new(http.clone(), None, receiver, None);
            let denied = format!(
                "cannot use Zed managed Node.js: {}",
                util::downloads_disabled_error("Node.js")
            );
            let error = runtime
                .instance_with_managed_dir(&directory.0)
                .await
                .and_then(|instance| instance.binary_path())
                .unwrap_err();
            assert!(error.is::<ToolPermissionDenied>());
            assert_eq!(format!("{error:#}"), denied);
            assert_eq!(http.0.load(Ordering::SeqCst), 0);
            assert!(runtime.state.lock().await.instance.is_none());

            let checks = Arc::new(AtomicUsize::new(0));
            let revoked = runtime.with_install_gate(Some(Arc::new({
                let checks = checks.clone();
                move |_| futures::future::ready(checks.fetch_add(1, Ordering::SeqCst) == 0).boxed()
            })));
            let error = revoked
                .instance_with_managed_dir(&directory.0)
                .await
                .and_then(|instance| instance.binary_path())
                .unwrap_err();
            assert!(error.is::<ToolPermissionDenied>());
            assert_eq!(format!("{error:#}"), denied);
            assert_eq!(checks.load(Ordering::SeqCst), 2);
            assert_eq!(http.0.load(Ordering::SeqCst), 0);

            options.send(Some(NodeBinaryOptions::default()))?;
            let approved =
                runtime.with_install_gate(Some(Arc::new(|_| futures::future::ready(true).boxed())));
            assert!(
                approved
                    .instance_with_managed_dir(&directory.0)
                    .await
                    .and_then(|instance| instance.binary_path())
                    .is_err()
            );
            assert_eq!(http.0.load(Ordering::SeqCst), 1);
            let error = runtime
                .instance_with_managed_dir(&directory.0)
                .await
                .and_then(|instance| instance.binary_path())
                .unwrap_err();
            assert!(error.is::<ToolPermissionDenied>());
            assert_eq!(format!("{error:#}"), denied);
            assert_eq!(http.0.load(Ordering::SeqCst), 1);
            assert!(runtime.state.lock().await.instance.is_none());
            Ok(())
        })
    }

    #[cfg(unix)]
    #[test]
    fn test_system_node_probe_requires_execution_consent() -> Result<()> {
        smol::block_on(async {
            let directory = TestDirectory::new()?;
            let error = super::SystemNodeRuntime::new(
                directory.0.join("untrusted-wrapper"),
                directory.0.join("npm"),
                None,
            )
            .await
            .unwrap_err();
            assert_eq!(error.to_string(), util::downloads_disabled_error("Node.js"));
            Ok(())
        })
    }

    #[cfg(unix)]
    #[test]
    fn test_installed_version_query_never_resolves_node() -> Result<()> {
        smol::block_on(async {
            let directory = TestDirectory::new()?;
            let package = directory.0.join("node_modules/package");
            std::fs::create_dir_all(&package)?;
            std::fs::write(package.join("package.json"), r#"{"version":"1.2.3"}"#)?;
            let runtime = NodeRuntime::new(
                Arc::new(NoNetworkHttpClient),
                None,
                watch::channel(Some(NodeBinaryOptions {
                    allow_binary_downloads: true,
                    use_paths: Some((
                        directory.0.join("untrusted-wrapper"),
                        directory.0.join("npm"),
                    )),
                    ..NodeBinaryOptions::default()
                }))
                .1,
                Some(Arc::new(|_| {
                    panic!("filesystem queries must not request execution consent")
                })),
            );
            assert_eq!(
                runtime
                    .npm_package_installed_version(&directory.0, "package")
                    .await?,
                Some(Version::new(1, 2, 3))
            );
            assert_eq!(
                runtime
                    .npm_package_installed_version(&directory.0, "missing")
                    .await?,
                None
            );
            assert!(runtime.state.lock().await.instance.is_none());
            Ok(())
        })
    }

    #[cfg(unix)]
    async fn assert_npm_cancellation(managed: bool) -> Result<()> {
        let directory = TestDirectory::new()?;
        std::fs::create_dir(directory.0.join("bin"))?;
        let node = directory.0.join("bin/node");
        let npm = directory.0.join("bin/npm");
        let pid_file = directory.0.join("bin/npm.pid");
        symlink("/bin/sh", &node)?;
        std::fs::write(
            &npm,
            "printf '%s\\n' \"$$\" > \"$0.pid\"\nexec /bin/sleep 60\n",
        )?;
        let instance: Box<dyn super::NodeRuntimeTrait> = if managed {
            Box::new(super::ManagedNodeRuntime {
                installation_path: directory.0.clone(),
            })
        } else {
            Box::new(super::SystemNodeRuntime {
                node,
                npm: PathBuf::from("/bin/sh"),
                scratch_dir: directory.0.clone(),
            })
        };
        let runtime = gated_node_runtime(Some(Arc::new(|_| futures::future::ready(true).boxed())));
        runtime.state.lock().await.instance = Some(instance);
        let npm = npm.to_string_lossy();
        let mut command = runtime.run_npm_subcommand(None, &npm, &[]).boxed();
        let deadline = Instant::now() + Duration::from_secs(5);
        let pid = loop {
            if let Poll::Ready(output) = futures::poll!(command.as_mut()) {
                bail!("fake npm exited before cancellation: {output:?}");
            }
            match std::fs::read_to_string(&pid_file) {
                Ok(contents) if contents.ends_with('\n') => {
                    break contents.trim().parse::<u32>()?;
                }
                Ok(_) => {}
                Err(error) if error.kind() == std::io::ErrorKind::NotFound => {}
                Err(error) => return Err(error.into()),
            }
            anyhow::ensure!(Instant::now() < deadline, "fake npm did not start");
            std::thread::sleep(Duration::from_millis(10));
        };
        drop(command);

        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            let output = new_command("/bin/kill")
                .args(["-0", &pid.to_string()])
                .output()
                .await?;
            if !output.status.success() {
                return Ok(());
            }
            if Instant::now() >= deadline {
                new_command("/bin/kill")
                    .args(["-KILL", &pid.to_string()])
                    .output()
                    .await?;
                bail!("npm child {pid} survived cancellation");
            }
            std::thread::sleep(Duration::from_millis(10));
        }
    }

    fn gated_node_runtime(install_gate: Option<NpmInstallGate>) -> NodeRuntime {
        NodeRuntime::unavailable().with_install_gate(install_gate)
    }

    #[cfg(unix)]
    struct TestDirectory(PathBuf);

    #[cfg(unix)]
    impl TestDirectory {
        fn new() -> Result<Self> {
            static NEXT_ID: AtomicUsize = AtomicUsize::new(0);
            let path = std::env::temp_dir().join(format!(
                "zed-node-runtime-{}-{}",
                std::process::id(),
                NEXT_ID.fetch_add(1, Ordering::SeqCst),
            ));
            std::fs::create_dir(&path)?;
            Ok(Self(path))
        }
    }

    #[cfg(unix)]
    impl Drop for TestDirectory {
        fn drop(&mut self) {
            std::fs::remove_dir_all(&self.0).log_err();
        }
    }

    #[cfg(unix)]
    #[derive(Clone)]
    struct PreparingNodeRuntime {
        admitted: PathBuf,
        allowed: Arc<AtomicBool>,
        revoke: Arc<AtomicBool>,
    }

    #[cfg(unix)]
    #[async_trait::async_trait]
    impl super::NodeRuntimeTrait for PreparingNodeRuntime {
        fn boxed_clone(&self) -> Box<dyn super::NodeRuntimeTrait> {
            Box::new(self.clone())
        }

        fn binary_path(&self) -> Result<PathBuf> {
            Ok(PathBuf::from("/bin/sh"))
        }

        async fn npm_command(
            &self,
            _: Option<&Path>,
            _: Option<&Url>,
            _: &str,
            _: &[&str],
        ) -> Result<super::NpmCommand> {
            smol::future::yield_now().await;
            if self.revoke.swap(false, Ordering::SeqCst) {
                self.allowed.store(false, Ordering::SeqCst);
            }
            Ok(super::NpmCommand {
                path: PathBuf::from("/bin/sh"),
                args: vec![
                    "-c".to_string(),
                    "printf admitted > \"$1\"".to_string(),
                    "probe".to_string(),
                    self.admitted.to_string_lossy().into_owned(),
                ],
                env: std::collections::HashMap::new(),
            })
        }
    }

    #[cfg(unix)]
    struct RecordingHttpClient(AtomicUsize);

    #[cfg(unix)]
    impl http_client::HttpClient for RecordingHttpClient {
        fn user_agent(&self) -> Option<&HeaderValue> {
            None
        }

        fn proxy(&self) -> Option<&Url> {
            None
        }

        fn send(
            &self,
            _: http_client::Request<http_client::AsyncBody>,
        ) -> futures::future::BoxFuture<
            'static,
            Result<http_client::Response<http_client::AsyncBody>>,
        > {
            self.0.fetch_add(1, Ordering::SeqCst);
            futures::future::ready(Err(anyhow::anyhow!("HTTP request admitted"))).boxed()
        }
    }

    #[cfg(unix)]
    struct NoNetworkHttpClient;

    #[cfg(unix)]
    impl http_client::HttpClient for NoNetworkHttpClient {
        fn user_agent(&self) -> Option<&HeaderValue> {
            None
        }

        fn proxy(&self) -> Option<&Url> {
            None
        }

        fn send(
            &self,
            _: http_client::Request<http_client::AsyncBody>,
        ) -> futures::future::BoxFuture<
            'static,
            Result<http_client::Response<http_client::AsyncBody>>,
        > {
            panic!("managed Node validation must not send HTTP requests")
        }
    }
}
