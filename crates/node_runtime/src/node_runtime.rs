mod archive;

use anyhow::{Context, Result, anyhow, bail};
pub use archive::extract_zip;
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use collections::HashMap;
use futures::AsyncReadExt;
use http_client::{HttpClient, Url};
use semver::Version;
use serde::Deserialize;
use smol::io::BufReader;
use smol::{fs, lock::Mutex};
use std::{
    env::{self, consts},
    io,
    path::{Path, PathBuf},
    process::{Output, Stdio},
    sync::Arc,
};
use util::ResultExt;

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct NodeBinaryOptions {
    pub allow_path_lookup: bool,
    pub allow_binary_download: bool,
    pub use_paths: Option<(PathBuf, PathBuf)>,
}

#[derive(Clone)]
pub struct NodeRuntime(Arc<Mutex<NodeRuntimeState>>);

struct NodeRuntimeState {
    http: Arc<dyn HttpClient>,
    instance: Option<Box<dyn NodeRuntimeTrait>>,
    last_options: Option<NodeBinaryOptions>,
    options: async_watch::Receiver<Option<NodeBinaryOptions>>,
}

impl NodeRuntime {
    pub fn new(
        http: Arc<dyn HttpClient>,
        options: async_watch::Receiver<Option<NodeBinaryOptions>>,
    ) -> Self {
        NodeRuntime(Arc::new(Mutex::new(NodeRuntimeState {
            http,
            instance: None,
            last_options: None,
            options,
        })))
    }

    pub fn unavailable() -> Self {
        NodeRuntime(Arc::new(Mutex::new(NodeRuntimeState {
            http: Arc::new(http_client::BlockedHttpClient),
            instance: None,
            last_options: None,
            options: async_watch::channel(Some(NodeBinaryOptions::default())).1,
        })))
    }

    async fn instance(&self) -> Result<Box<dyn NodeRuntimeTrait>> {
        let mut state = self.0.lock().await;

        while state.options.borrow().is_none() {
            state.options.changed().await?;
        }
        let options = state.options.borrow().clone().unwrap();
        if state.last_options.as_ref() != Some(&options) {
            state.instance.take();
        }
        if let Some(instance) = state.instance.as_ref() {
            return Ok(instance.boxed_clone());
        }

        let home = paths::home_dir();
        let env = environment::in_dir(home, false).await?;

        if let Some((node, npm)) = options.use_paths.as_ref() {
            let instance = SystemNodeRuntime::new(env, node.clone(), npm.clone()).await?;
            state.instance = Some(instance.boxed_clone());
            return Ok(instance);
        }

        if options.allow_path_lookup {
            if let Some(instance) = SystemNodeRuntime::detect(env.clone()).await {
                state.instance = Some(instance.boxed_clone());
                return Ok(instance);
            }
        }

        let instance = if options.allow_binary_download {
            ManagedNodeRuntime::install_if_needed(env, &state.http).await?
        } else {
            Box::new(UnavailableNodeRuntime)
        };

        state.instance = Some(instance.boxed_clone());
        return Ok(instance);
    }

    pub async fn binary_path(&self) -> Result<PathBuf> {
        self.instance().await?.binary_path()
    }

    pub async fn run_npm_subcommand(
        &self,
        directory: &Path,
        subcommand: &str,
        args: &[&str],
    ) -> Result<Output> {
        let http = self.0.lock().await.http.clone();
        self.instance()
            .await?
            .run_npm_subcommand(Some(directory), http.proxy(), subcommand, args)
            .await
    }

    pub async fn npm_package_installed_version(
        &self,
        local_package_directory: &Path,
        name: &str,
    ) -> Result<Option<String>> {
        self.instance()
            .await?
            .npm_package_installed_version(local_package_directory, name)
            .await
    }

    pub async fn npm_package_latest_version(&self, name: &str) -> Result<String> {
        let http = self.0.lock().await.http.clone();
        let output = self
            .instance()
            .await?
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
            .ok_or_else(|| anyhow!("no version found for npm package {}", name))
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
        latest_version: &str,
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
        let Some(latest_version) = Version::parse(latest_version).log_err() else {
            return true;
        };

        installed_version < latest_version
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
    clean_env: HashMap<String, String>,
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

    async fn install_if_needed(
        env: HashMap<String, String>,
        http: &Arc<dyn HttpClient>,
    ) -> Result<Box<dyn NodeRuntimeTrait>> {
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

        let mut clean_env = HashMap::default();
        clean_env.insert(
            "PATH".to_string(),
            path_with_node_binary_prepended(
                env.get("PATH").cloned().unwrap_or_default(),
                &node_binary,
            ),
        );
        if let Ok(node_ca_certs) = env::var("NODE_EXTRA_CA_CERTS") {
            clean_env.insert("NODE_EXTRA_CA_CERTS".to_string(), node_ca_certs);
        }

        let result = util::command::new_smol_command(&node_binary, &env)
            .arg(npm_file)
            .arg("--version")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .args(["--cache".into(), node_dir.join("cache")])
            .args(["--userconfig".into(), node_dir.join("blank_user_npmrc")])
            .args(["--globalconfig".into(), node_dir.join("blank_global_npmrc")])
            .status()
            .await;
        let valid = matches!(result, Ok(status) if status.success());

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
            let mut response = http
                .get(&url, Default::default(), true)
                .await
                .context("error downloading Node binary tarball")?;

            let body = response.body_mut();
            match archive_type {
                ArchiveType::TarGz => {
                    let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
                    let archive = Archive::new(decompressed_bytes);
                    archive.unpack(&node_containing_dir).await?;
                }
                ArchiveType::Zip => archive::extract_zip(&node_containing_dir, body).await?,
            }
        }

        // Note: Not in the `if !valid {}` so we can populate these for existing installations
        _ = fs::create_dir(node_dir.join("cache")).await;
        _ = fs::write(node_dir.join("blank_user_npmrc"), []).await;
        _ = fs::write(node_dir.join("blank_global_npmrc"), []).await;

        anyhow::Ok(Box::new(ManagedNodeRuntime {
            clean_env,
            installation_path: node_dir,
        }))
    }
}

fn path_with_node_binary_prepended(existing_path: String, node_binary: &Path) -> String {
    let Some(node_bin_dir) = node_binary.parent().map(|dir| dir.as_os_str()) else {
        return existing_path;
    };

    let mut existing = env::split_paths(&existing_path).collect::<Vec<_>>();
    existing.insert(0, PathBuf::from(node_bin_dir));

    let joined = env::join_paths(existing)
        .ok()
        .and_then(|e| e.to_str().map(|s| s.to_string()));
    joined.unwrap_or(existing_path)
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

            if smol::fs::metadata(&node_binary).await.is_err() {
                return Err(anyhow!("missing node binary file"));
            }

            if smol::fs::metadata(&npm_file).await.is_err() {
                return Err(anyhow!("missing npm file"));
            }

            let mut command = util::command::new_smol_command(node_binary, &self.clean_env);
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
            if output.is_err() {
                return Err(anyhow!(
                    "failed to launch npm subcommand {subcommand} subcommand\nerr: {:?}",
                    output.err()
                ));
            }
        }

        if let Ok(output) = &output {
            if !output.status.success() {
                return Err(anyhow!(
                    "failed to execute npm {subcommand} subcommand:\nstdout: {:?}\nstderr: {:?}",
                    String::from_utf8_lossy(&output.stdout),
                    String::from_utf8_lossy(&output.stderr)
                ));
            }
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

#[derive(Clone)]
pub struct SystemNodeRuntime {
    node: PathBuf,
    npm: PathBuf,
    global_node_modules: PathBuf,
    scratch_dir: PathBuf,
    clean_env: HashMap<String, String>,
}

impl SystemNodeRuntime {
    const MIN_VERSION: semver::Version = Version::new(20, 0, 0);
    async fn new(
        env: HashMap<String, String>,
        node: PathBuf,
        npm: PathBuf,
    ) -> Result<Box<dyn NodeRuntimeTrait>> {
        let path =
            path_with_node_binary_prepended(env.get("PATH").cloned().unwrap_or_default(), &node);
        let mut clean_env = HashMap::default();
        clean_env.insert("PATH".to_string(), path);

        if let Ok(node_ca_certs) = env::var("NODE_EXTRA_CA_CERTS") {
            clean_env.insert("NODE_EXTRA_CA_CERTS".to_string(), node_ca_certs);
        }

        let output = util::command::new_smol_command(&node, &clean_env)
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
            clean_env,
        };
        let output = this.run_npm_subcommand(None, None, "root", &["-g"]).await?;
        this.global_node_modules =
            PathBuf::from(String::from_utf8_lossy(&output.stdout).to_string());

        Ok(Box::new(this))
    }

    async fn detect(env: HashMap<String, String>) -> Option<Box<dyn NodeRuntimeTrait>> {
        let path = env
            .get("PATH")
            .cloned()
            .or_else(|| std::env::var("PATH").ok());
        let node = which::which_in_global("node", path.as_ref()).ok()?.next()?;
        let npm = which::which_in_global("npm", path.as_ref()).ok()?.next()?;
        Self::new(env, node, npm).await.log_err()
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
        let mut command = util::command::new_smol_command(self.npm.clone(), &self.clean_env);
        command
            .arg(subcommand)
            .args(["--cache".into(), self.scratch_dir.join("cache")])
            .args(args);
        configure_npm_command(&mut command, directory, proxy);
        let output = command.output().await?;
        if !output.status.success() {
            return Err(anyhow!(
                "failed to execute npm {subcommand} subcommand:\nstdout: {:?}\nstderr: {:?}",
                String::from_utf8_lossy(&output.stdout),
                String::from_utf8_lossy(&output.stderr)
            ));
        }

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

pub struct UnavailableNodeRuntime;

#[async_trait::async_trait]
impl NodeRuntimeTrait for UnavailableNodeRuntime {
    fn boxed_clone(&self) -> Box<dyn NodeRuntimeTrait> {
        Box::new(UnavailableNodeRuntime)
    }
    fn binary_path(&self) -> Result<PathBuf> {
        bail!("binary_path: no node runtime available")
    }

    async fn run_npm_subcommand(
        &self,
        _: Option<&Path>,
        _: Option<&Url>,
        _: &str,
        _: &[&str],
    ) -> anyhow::Result<Output> {
        bail!("run_npm_subcommand: no node runtime available")
    }

    async fn npm_package_installed_version(
        &self,
        _local_package_directory: &Path,
        _: &str,
    ) -> Result<Option<String>> {
        bail!("npm_package_installed_version: no node runtime available")
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
