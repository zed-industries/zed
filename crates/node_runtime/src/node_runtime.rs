use anyhow::{Context as _, Result, anyhow, bail};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use futures::{AsyncReadExt, FutureExt as _, channel::oneshot, future::Shared};
use http_client::{HttpClient, Url};
use log::Level;
use semver::Version;
use serde::Deserialize;
use smol::io::BufReader;
use smol::{fs, lock::Mutex};
use std::fmt::Display;
use std::{
    env::{self, consts},
    ffi::OsString,
    io,
    path::{Path, PathBuf},
    process::Output,
    sync::Arc,
};
use util::ResultExt;
use util::archive::extract_zip;

const NODE_CA_CERTS_ENV_VAR: &str = "NODE_EXTRA_CA_CERTS";

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NodeBinaryOptions {
    pub allow_path_lookup: bool,
    pub allow_binary_download: bool,
    pub use_paths: Option<(PathBuf, PathBuf)>,
}

pub enum VersionStrategy<'a> {
    /// Install if current version doesn't match pinned version
    Pin(&'a str),
    /// Install if current version is older than latest version
    Latest(&'a str),
}

#[derive(Clone)]
pub struct NodeRuntime(Arc<Mutex<NodeRuntimeState>>);

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
    ) -> Self {
        NodeRuntime(Arc::new(Mutex::new(NodeRuntimeState {
            http,
            instance: None,
            last_options: None,
            options,
            shell_env_loaded: shell_env_loaded.unwrap_or(oneshot::channel().1).shared(),
        })))
    }

    pub fn unavailable() -> Self {
        NodeRuntime(Arc::new(Mutex::new(NodeRuntimeState {
            http: Arc::new(http_client::BlockedHttpClient),
            instance: None,
            last_options: None,
            options: watch::channel(Some(NodeBinaryOptions::default())).1,
            shell_env_loaded: oneshot::channel().1.shared(),
        })))
    }

    async fn instance(&self) -> Box<dyn NodeRuntimeTrait> {
        let mut state = self.0.lock().await;

        let options = loop {
            if let Some(options) = state.options.borrow().as_ref() {
                break options.clone();
            }
            match state.options.changed().await {
                Ok(()) => {}
                // failure case not cached
                Err(err) => {
                    return Box::new(UnavailableNodeRuntime {
                        error_message: err.to_string().into(),
                    });
                }
            }
        };

        if state.last_options.as_ref() != Some(&options) {
            state.instance.take();
        }
        if let Some(instance) = state.instance.as_ref() {
            return instance.boxed_clone();
        }

        if let Some((node, npm)) = options.use_paths.as_ref() {
            let instance = match SystemNodeRuntime::new(node.clone(), npm.clone()).await {
                Ok(instance) => {
                    log::info!("using Node.js from `node.path` in settings: {:?}", instance);
                    Box::new(instance)
                }
                Err(err) => {
                    // failure case not cached, since it's cheap to check again
                    return Box::new(UnavailableNodeRuntime {
                        error_message: format!(
                            "failure checking Node.js from `node.path` in settings ({}): {:?}",
                            node.display(),
                            err
                        )
                        .into(),
                    });
                }
            };
            state.instance = Some(instance.boxed_clone());
            state.last_options = Some(options);
            return instance;
        }

        let system_node_error = if options.allow_path_lookup {
            state.shell_env_loaded.clone().await.ok();
            match SystemNodeRuntime::detect().await {
                Ok(instance) => {
                    log::info!("using Node.js found on PATH: {:?}", instance);
                    state.instance = Some(instance.boxed_clone());
                    state.last_options = Some(options);
                    return Box::new(instance);
                }
                Err(err) => Some(err),
            }
        } else {
            None
        };

        let instance = if options.allow_binary_download {
            let (log_level, why_using_managed) = match system_node_error {
                Some(err @ DetectError::Other(_)) => (Level::Warn, err.to_string()),
                Some(err @ DetectError::NotInPath(_)) => (Level::Info, err.to_string()),
                None => (
                    Level::Info,
                    "`node.ignore_system_version` is `true` in settings".to_string(),
                ),
            };
            match ManagedNodeRuntime::install_if_needed(&state.http).await {
                Ok(instance) => {
                    log::log!(
                        log_level,
                        "using Zed managed Node.js at {} since {}",
                        instance.installation_path.display(),
                        why_using_managed
                    );
                    Box::new(instance) as Box<dyn NodeRuntimeTrait>
                }
                Err(err) => {
                    // failure case is cached, since downloading + installing may be expensive. The
                    // downside of this is that it may fail due to an intermittent network issue.
                    //
                    // TODO: Have `install_if_needed` indicate which failure cases are retryable
                    // and/or have shared tracking of when internet is available.
                    Box::new(UnavailableNodeRuntime {
                        error_message: format!(
                            "failure while downloading and/or installing Zed managed Node.js, \
                            restart Zed to retry: {}",
                            err
                        )
                        .into(),
                    }) as Box<dyn NodeRuntimeTrait>
                }
            }
        } else if let Some(system_node_error) = system_node_error {
            // failure case not cached, since it's cheap to check again
            //
            // TODO: When support is added for setting `options.allow_binary_download`, update this
            // error message.
            return Box::new(UnavailableNodeRuntime {
                error_message: format!(
                    "failure while checking system Node.js from PATH: {}",
                    system_node_error
                )
                .into(),
            });
        } else {
            // failure case is cached because it will always happen with these options
            //
            // TODO: When support is added for setting `options.allow_binary_download`, update this
            // error message.
            Box::new(UnavailableNodeRuntime {
                error_message: "`node` settings do not allow any way to use Node.js"
                    .to_string()
                    .into(),
            })
        };

        state.instance = Some(instance.boxed_clone());
        state.last_options = Some(options);
        instance
    }

    pub async fn binary_path(&self) -> Result<PathBuf> {
        self.instance().await.binary_path()
    }

    pub async fn run_npm_subcommand(
        &self,
        directory: &Path,
        subcommand: &str,
        args: &[&str],
    ) -> Result<Output> {
        let http = self.0.lock().await.http.clone();
        self.instance()
            .await
            .run_npm_subcommand(Some(directory), http.proxy(), subcommand, args)
            .await
    }

    pub async fn npm_package_installed_version(
        &self,
        local_package_directory: &Path,
        name: &str,
    ) -> Result<Option<String>> {
        self.instance()
            .await
            .npm_package_installed_version(local_package_directory, name)
            .await
    }

    pub async fn npm_package_latest_version(&self, name: &str) -> Result<String> {
        let http = self.0.lock().await.http.clone();
        let output = self
            .instance()
            .await
            .run_npm_subcommand(
                None,
                http.proxy(),
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
            )
            .await?;

        let mut info: NpmInfo = serde_json::from_slice(&output.stdout)?;
        info.dist_tags
            .latest
            .or_else(|| info.versions.pop())
            .with_context(|| format!("no version found for npm package {name}"))
    }

    pub async fn npm_install_packages(
        &self,
        directory: &Path,
        packages: &[(&str, &str)],
    ) -> Result<()> {
        if packages.is_empty() {
            return Ok(());
        }

        let packages: Vec<_> = packages
            .iter()
            .map(|(name, version)| format!("{name}@{version}"))
            .collect();

        let mut arguments: Vec<_> = packages.iter().map(|p| p.as_str()).collect();
        arguments.extend_from_slice(&[
            "--save-exact",
            "--fetch-retry-mintimeout",
            "2000",
            "--fetch-retry-maxtimeout",
            "5000",
            "--fetch-timeout",
            "5000",
        ]);

        // This is also wrong because the directory is wrong.
        self.run_npm_subcommand(directory, "install", &arguments)
            .await?;
        Ok(())
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
            return true;
        }

        let Some(installed_version) = self
            .npm_package_installed_version(local_package_directory, package_name)
            .await
            .log_err()
            .flatten()
        else {
            return true;
        };

        let Some(installed_version) = Version::parse(&installed_version).log_err() else {
            return true;
        };

        match version_strategy {
            VersionStrategy::Pin(pinned_version) => {
                let Some(pinned_version) = Version::parse(pinned_version).log_err() else {
                    return true;
                };
                installed_version != pinned_version
            }
            VersionStrategy::Latest(latest_version) => {
                let Some(latest_version) = Version::parse(latest_version).log_err() else {
                    return true;
                };
                installed_version < latest_version
            }
        }
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
    versions: Vec<String>,
}

#[derive(Debug, Deserialize, Default)]
pub struct NpmInfoDistTags {
    latest: Option<String>,
}

#[async_trait::async_trait]
trait NodeRuntimeTrait: Send + Sync {
    fn boxed_clone(&self) -> Box<dyn NodeRuntimeTrait>;
    fn binary_path(&self) -> Result<PathBuf>;

    async fn run_npm_subcommand(
        &self,
        directory: Option<&Path>,
        proxy: Option<&Url>,
        subcommand: &str,
        args: &[&str],
    ) -> Result<Output>;

    async fn npm_package_installed_version(
        &self,
        local_package_directory: &Path,
        name: &str,
    ) -> Result<Option<String>>;
}

#[derive(Clone)]
struct ManagedNodeRuntime {
    installation_path: PathBuf,
}

impl ManagedNodeRuntime {
    const VERSION: &str = "v22.5.1";

    #[cfg(not(windows))]
    const NODE_PATH: &str = "bin/node";
    #[cfg(windows)]
    const NODE_PATH: &str = "node.exe";

    #[cfg(not(windows))]
    const NPM_PATH: &str = "bin/npm";
    #[cfg(windows)]
    const NPM_PATH: &str = "node_modules/npm/bin/npm-cli.js";

    async fn install_if_needed(http: &Arc<dyn HttpClient>) -> Result<Self> {
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
        let node_containing_dir = paths::data_dir().join("node");
        let node_dir = node_containing_dir.join(folder_name);
        let node_binary = node_dir.join(Self::NODE_PATH);
        let npm_file = node_dir.join(Self::NPM_PATH);
        let node_ca_certs = env::var(NODE_CA_CERTS_ENV_VAR).unwrap_or_else(|_| String::new());

        let valid = if fs::metadata(&node_binary).await.is_ok() {
            let result = util::command::new_smol_command(&node_binary)
                .env_clear()
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
                        "Zed managed Node.js binary at {} failed check, so re-downloading it. \
                        Error: {}",
                        node_binary.display(),
                        err
                    );
                    false
                }
            }
        } else {
            false
        };

        if !valid {
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

        // Note: Not in the `if !valid {}` so we can populate these for existing installations
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

    async fn run_npm_subcommand(
        &self,
        directory: Option<&Path>,
        proxy: Option<&Url>,
        subcommand: &str,
        args: &[&str],
    ) -> Result<Output> {
        let attempt = || async move {
            let node_binary = self.installation_path.join(Self::NODE_PATH);
            let npm_file = self.installation_path.join(Self::NPM_PATH);
            let env_path = path_with_node_binary_prepended(&node_binary).unwrap_or_default();

            anyhow::ensure!(
                smol::fs::metadata(&node_binary).await.is_ok(),
                "missing node binary file"
            );
            anyhow::ensure!(
                smol::fs::metadata(&npm_file).await.is_ok(),
                "missing npm file"
            );

            let node_ca_certs = env::var(NODE_CA_CERTS_ENV_VAR).unwrap_or_else(|_| String::new());

            let mut command = util::command::new_smol_command(node_binary);
            command.env_clear();
            command.env("PATH", env_path);
            command.env(NODE_CA_CERTS_ENV_VAR, node_ca_certs);
            command.arg(npm_file).arg(subcommand);
            command.args(["--cache".into(), self.installation_path.join("cache")]);
            command.args([
                "--userconfig".into(),
                self.installation_path.join("blank_user_npmrc"),
            ]);
            command.args([
                "--globalconfig".into(),
                self.installation_path.join("blank_global_npmrc"),
            ]);
            command.args(args);
            configure_npm_command(&mut command, directory, proxy);
            command.output().await.map_err(|e| anyhow!("{e}"))
        };

        let mut output = attempt().await;
        if output.is_err() {
            output = attempt().await;
            anyhow::ensure!(
                output.is_ok(),
                "failed to launch npm subcommand {subcommand} subcommand\nerr: {:?}",
                output.err()
            );
        }

        if let Ok(output) = &output {
            anyhow::ensure!(
                output.status.success(),
                "failed to execute npm {subcommand} subcommand:\nstdout: {:?}\nstderr: {:?}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            );
        }

        output.map_err(|e| anyhow!("{e}"))
    }
    async fn npm_package_installed_version(
        &self,
        local_package_directory: &Path,
        name: &str,
    ) -> Result<Option<String>> {
        read_package_installed_version(local_package_directory.join("node_modules"), name).await
    }
}

#[derive(Debug, Clone)]
pub struct SystemNodeRuntime {
    node: PathBuf,
    npm: PathBuf,
    global_node_modules: PathBuf,
    scratch_dir: PathBuf,
}

impl SystemNodeRuntime {
    const MIN_VERSION: semver::Version = Version::new(20, 0, 0);
    async fn new(node: PathBuf, npm: PathBuf) -> Result<Self> {
        let output = util::command::new_smol_command(&node)
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
        fs::create_dir(scratch_dir.join("cache")).await.ok();

        let mut this = Self {
            node,
            npm,
            global_node_modules: PathBuf::default(),
            scratch_dir,
        };
        let output = this.run_npm_subcommand(None, None, "root", &["-g"]).await?;
        this.global_node_modules =
            PathBuf::from(String::from_utf8_lossy(&output.stdout).to_string());

        Ok(this)
    }

    async fn detect() -> std::result::Result<Self, DetectError> {
        let node = which::which("node").map_err(DetectError::NotInPath)?;
        let npm = which::which("npm").map_err(DetectError::NotInPath)?;
        Self::new(node, npm).await.map_err(DetectError::Other)
    }
}

enum DetectError {
    NotInPath(which::Error),
    Other(anyhow::Error),
}

impl Display for DetectError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            DetectError::NotInPath(err) => {
                write!(f, "system Node.js wasn't found on PATH: {}", err)
            }
            DetectError::Other(err) => {
                write!(f, "checking system Node.js failed with error: {}", err)
            }
        }
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

    async fn run_npm_subcommand(
        &self,
        directory: Option<&Path>,
        proxy: Option<&Url>,
        subcommand: &str,
        args: &[&str],
    ) -> anyhow::Result<Output> {
        let node_ca_certs = env::var(NODE_CA_CERTS_ENV_VAR).unwrap_or_else(|_| String::new());
        let mut command = util::command::new_smol_command(self.npm.clone());
        let path = path_with_node_binary_prepended(&self.node).unwrap_or_default();
        command
            .env_clear()
            .env("PATH", path)
            .env(NODE_CA_CERTS_ENV_VAR, node_ca_certs)
            .arg(subcommand)
            .args(["--cache".into(), self.scratch_dir.join("cache")])
            .args(args);
        configure_npm_command(&mut command, directory, proxy);
        let output = command.output().await?;
        anyhow::ensure!(
            output.status.success(),
            "failed to execute npm {subcommand} subcommand:\nstdout: {:?}\nstderr: {:?}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        );
        Ok(output)
    }

    async fn npm_package_installed_version(
        &self,
        local_package_directory: &Path,
        name: &str,
    ) -> Result<Option<String>> {
        read_package_installed_version(local_package_directory.join("node_modules"), name).await
        // todo: allow returning a globally installed version (requires callers not to hard-code the path)
    }
}

pub async fn read_package_installed_version(
    node_module_directory: PathBuf,
    name: &str,
) -> Result<Option<String>> {
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
        version: String,
    }

    let mut contents = String::new();
    file.read_to_string(&mut contents).await?;
    let package_json: PackageJson = serde_json::from_str(&contents)?;
    Ok(Some(package_json.version))
}

#[derive(Clone)]
pub struct UnavailableNodeRuntime {
    error_message: Arc<String>,
}

#[async_trait::async_trait]
impl NodeRuntimeTrait for UnavailableNodeRuntime {
    fn boxed_clone(&self) -> Box<dyn NodeRuntimeTrait> {
        Box::new(self.clone())
    }
    fn binary_path(&self) -> Result<PathBuf> {
        bail!("{}", self.error_message)
    }

    async fn run_npm_subcommand(
        &self,
        _: Option<&Path>,
        _: Option<&Url>,
        _: &str,
        _: &[&str],
    ) -> anyhow::Result<Output> {
        bail!("{}", self.error_message)
    }

    async fn npm_package_installed_version(
        &self,
        _local_package_directory: &Path,
        _: &str,
    ) -> Result<Option<String>> {
        bail!("{}", self.error_message)
    }
}

fn configure_npm_command(
    command: &mut smol::process::Command,
    directory: Option<&Path>,
    proxy: Option<&Url>,
) {
    if let Some(directory) = directory {
        command.current_dir(directory);
        command.args(["--prefix".into(), directory.to_path_buf()]);
    }

    if let Some(proxy) = proxy {
        // Map proxy settings from `http://localhost:10809` to `http://127.0.0.1:10809`
        // NodeRuntime without environment information can not parse `localhost`
        // correctly.
        // TODO: map to `[::1]` if we are using ipv6
        let proxy = proxy
            .to_string()
            .to_ascii_lowercase()
            .replace("localhost", "127.0.0.1");

        command.args(["--proxy", &proxy]);
    }

    #[cfg(windows)]
    {
        // SYSTEMROOT is a critical environment variables for Windows.
        if let Some(val) = env::var("SYSTEMROOT")
            .context("Missing environment variable: SYSTEMROOT!")
            .log_err()
        {
            command.env("SYSTEMROOT", val);
        }
        // Without ComSpec, the post-install will always fail.
        if let Some(val) = env::var("ComSpec")
            .context("Missing environment variable: ComSpec!")
            .log_err()
        {
            command.env("ComSpec", val);
        }
    }
}
