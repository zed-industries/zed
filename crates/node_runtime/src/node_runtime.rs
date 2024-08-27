mod archive;

use anyhow::{anyhow, bail, Context, Result};
pub use archive::extract_zip;
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use futures::channel::mpsc;
use futures::channel::oneshot;
use futures::AsyncReadExt;
use gpui::SemanticVersion;
use http_client::HttpClient;
use semver::Version;
use serde::Deserialize;
use smol::io::BufReader;
use smol::{fs, lock::Mutex, process::Command};
use std::fmt::Display;
use std::io;
use std::io::ErrorKind;
use std::process::{Output, Stdio};
use std::str::from_utf8;
use std::str::FromStr;
use std::{
    env::consts,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::{AssetVersion, ResultExt};

#[cfg(windows)]
use smol::process::windows::CommandExt;

const VERSION: SemanticVersion = SemanticVersion::new(22, 5, 1);

#[cfg(not(windows))]
const NODE_PATH: &str = "bin/node";
#[cfg(windows)]
const NODE_PATH: &str = "node.exe";

#[cfg(not(windows))]
const NPM_PATH: &str = "bin/npm";
#[cfg(windows)]
const NPM_PATH: &str = "node_modules/npm/bin/npm-cli.js";

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

#[derive(Debug, Clone)]
pub struct NodeAssetVersion {
    pub name: String,
    pub version: String,
}

impl Display for NodeAssetVersion {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_fmt(format_args!("{}@{}", self.name, self.version))
    }
}

impl util::AssetVersion for NodeAssetVersion {
    fn description(&self) -> String {
        format!("node module {}@{}", self.name, self.version)
    }

    fn as_any(&self) -> &dyn std::any::Any {
        self
    }
}

#[async_trait::async_trait]
pub trait NodeRuntime: Send + Sync {
    async fn binary_path(&self) -> Result<PathBuf>;

    async fn run_npm_subcommand(
        &self,
        directory: Option<&Path>,
        subcommand: &str,
        args: &[&str],
    ) -> Result<Output>;

    async fn npm_package_latest_version(&self, name: &str) -> Result<NodeAssetVersion>;

    async fn npm_install_packages(
        &self,
        directory: &Path,
        packages: &[NodeAssetVersion],
    ) -> Result<()>;

    async fn npm_package_installed_version(
        &self,
        local_package_directory: &PathBuf,
        name: &str,
    ) -> Result<Option<String>>;

    async fn should_install_npm_package(
        &self,
        package: &NodeAssetVersion,
        local_executable_path: &Path,
        local_package_directory: &PathBuf,
    ) -> bool {
        // In the case of the local system not having the package installed,
        // or in the instances where we fail to parse package.json data,
        // we attempt to install the package.
        if fs::metadata(local_executable_path).await.is_err() {
            return true;
        }

        let Some(installed_version) = self
            .npm_package_installed_version(local_package_directory, &package.name)
            .await
            .log_err()
            .flatten()
        else {
            return true;
        };

        let Some(installed_version) = Version::parse(&installed_version).log_err() else {
            return true;
        };
        let Some(latest_version) = Version::parse(&package.version).log_err() else {
            return true;
        };

        installed_version < latest_version
    }
}

pub struct RealNodeRuntime {
    http: Arc<dyn HttpClient>,
    installation_lock: Mutex<()>,
    check_can_install: mpsc::UnboundedSender<(Box<dyn AssetVersion>, oneshot::Sender<Result<()>>)>,

    can_use_dependencies: mpsc::UnboundedSender<oneshot::Sender<bool>>,
}

impl RealNodeRuntime {
    pub fn new(
        http: Arc<dyn HttpClient>,
        check_can_install: mpsc::UnboundedSender<(
            Box<dyn AssetVersion>,
            oneshot::Sender<Result<()>>,
        )>,
        can_use_dependencies: mpsc::UnboundedSender<oneshot::Sender<bool>>,
    ) -> Arc<dyn NodeRuntime> {
        Arc::new(RealNodeRuntime {
            http,
            installation_lock: Mutex::new(()),
            check_can_install,
            can_use_dependencies,
        })
    }

    async fn check_dependency(&self, asset_version: impl AssetVersion) -> Result<()> {
        let (tx, rx) = oneshot::channel();
        self.check_can_install
            .unbounded_send((Box::new(asset_version), tx))
            .ok();
        rx.await?
    }

    async fn can_install_dependencies(&self) -> bool {
        let (tx, rx) = oneshot::channel();
        self.can_use_dependencies.unbounded_send(tx).ok();
        rx.await.unwrap_or(false)
    }

    async fn install_if_needed(&self) -> Result<(PathBuf, PathBuf, PathBuf)> {
        log::info!("Node runtime install_if_needed");
        let _lock = self.installation_lock.lock().await;

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

        let mut found_node_version: Option<SemanticVersion> = None;

        #[cfg(not(windows))]
        let node_path_binary = "node";
        #[cfg(not(windows))]
        let npm_path_binary = "npm";

        #[cfg(windows)]
        let node_path_binary = "node.exe";
        #[cfg(windows)]
        let npm_path_binary = "npm.exe"; // ???

        if let Some(node_path) = which::which(node_path_binary).log_err() {
            if let Some(npm_path) = which::which(npm_path_binary).log_err() {
                let mut path_command = Command::new(dbg!(&node_path));
                path_command
                    .env_clear()
                    .arg("--version")
                    .stdin(Stdio::null())
                    .stderr(Stdio::null());

                #[cfg(windows)]
                path_command.creation_flags(windows::Win32::System::Threading::CREATE_NO_WINDOW.0);

                let output = path_command.output().await;

                if let Some(output) = output.log_err() {
                    if output.status.success() {
                        if let Ok(output) = from_utf8(&output.stdout) {
                            let output = output.trim();
                            let output = if let Some(output) = output.strip_prefix("v") {
                                output
                            } else {
                                output
                            };

                            if let Some(node_version) = SemanticVersion::from_str(output).log_err()
                            {
                                found_node_version = Some(node_version);
                                if node_version >= VERSION {
                                    let folder_name = format!("node-v{node_version}-{os}-{arch}");
                                    let node_containing_dir = paths::support_dir().join("node");
                                    let node_dir = node_containing_dir.join(folder_name);

                                    // Make sure the proper file structure is setup
                                    if fs::metadata(&node_dir)
                                        .await
                                        .is_err_and(|e| e.kind() == ErrorKind::NotFound)
                                    {
                                        _ = fs::create_dir_all(node_dir.join("cache")).await;
                                        _ = fs::write(node_dir.join("blank_user_npmrc"), []).await;
                                        _ = fs::write(node_dir.join("blank_global_npmrc"), [])
                                            .await;
                                    }

                                    return Ok((node_path, npm_path, node_dir));
                                } else {
                                    log::error!(
                                        "node version on PATH is too old: v{}, Zed requires: v{}",
                                        node_version,
                                        VERSION
                                    )
                                }
                            }
                        }
                    }
                }
            }
        }

        if !self.can_install_dependencies().await {
            let err = if let Some(node_version) = found_node_version {
                anyhow!(
                    "Node version on $PATH is too old. Verion required: {}, version found: {}",
                    VERSION,
                    node_version
                )
            } else {
                anyhow!("Could not find or use node on $PATH")
            };
            return Err(err);
        }

        let folder_name = format!("node-v{VERSION}-{os}-{arch}");
        let node_containing_dir = paths::support_dir().join("node");
        let node_dir = node_containing_dir.join(folder_name);
        let node_binary = node_dir.join(NODE_PATH);

        let mut command = Command::new(&node_binary);
        command
            .env_clear()
            .arg("--version")
            .stdin(Stdio::null())
            .stderr(Stdio::null())
            .stdout(Stdio::null());

        #[cfg(windows)]
        command.creation_flags(windows::Win32::System::Threading::CREATE_NO_WINDOW.0);

        let result = command.status().await;
        let valid = matches!(result, Ok(status) if status.success());

        if !valid {
            self.check_dependency(NodeAssetVersion {
                name: "node".to_string(),
                version: format!("{}", VERSION),
            })
            .await?;

            _ = fs::remove_dir_all(&node_containing_dir).await;
            fs::create_dir(&node_containing_dir)
                .await
                .context("error creating node containing dir")?;

            let archive_type = match consts::OS {
                "macos" | "linux" => ArchiveType::TarGz,
                "windows" => ArchiveType::Zip,
                other => bail!("Running on unsupported os: {other}"),
            };

            let file_name = format!(
                "node-v{VERSION}-{os}-{arch}.{extension}",
                extension = match archive_type {
                    ArchiveType::TarGz => "tar.gz",
                    ArchiveType::Zip => "zip",
                }
            );
            let url = format!("https://nodejs.org/dist/v{VERSION}/{file_name}");
            let mut response = self
                .http
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

            _ = fs::create_dir(node_dir.join("cache")).await;
            _ = fs::write(node_dir.join("blank_user_npmrc"), []).await;
            _ = fs::write(node_dir.join("blank_global_npmrc"), []).await;
        }

        anyhow::Ok((node_dir.join(NODE_PATH), node_dir.join(NPM_PATH), node_dir))
    }
}

#[async_trait::async_trait]
impl NodeRuntime for RealNodeRuntime {
    async fn binary_path(&self) -> Result<PathBuf> {
        let (binary_path, _, _) = self.install_if_needed().await?;
        Ok(binary_path)
    }

    async fn run_npm_subcommand(
        &self,
        directory: Option<&Path>,
        subcommand: &str,
        args: &[&str],
    ) -> Result<Output> {
        let attempt = || async move {
            let (node_binary, npm_path, node_dir) = self.install_if_needed().await?;

            let mut env_path = vec![node_binary
                .parent()
                .expect("invalid node binary path")
                .to_path_buf()];

            if let Some(existing_path) = std::env::var_os("PATH") {
                let mut paths = std::env::split_paths(&existing_path).collect::<Vec<_>>();
                env_path.append(&mut paths);
            }

            let env_path =
                std::env::join_paths(env_path).context("failed to create PATH env variable")?;

            if smol::fs::metadata(&node_binary).await.is_err() {
                return Err(anyhow!("missing node binary file"));
            }

            if smol::fs::metadata(&npm_path).await.is_err() {
                return Err(anyhow!("missing npm file"));
            }

            let mut command = Command::new(node_binary);
            command.env_clear();
            command.env("PATH", env_path);
            command.arg(&npm_path).arg(subcommand);
            command.args(["--cache".into(), node_dir.join("cache")]);
            command.args(["--userconfig".into(), node_dir.join("blank_user_npmrc")]);
            command.args(["--globalconfig".into(), node_dir.join("blank_global_npmrc")]);
            command.args(args);

            if let Some(directory) = directory {
                command.current_dir(directory);
                command.args(["--prefix".into(), directory.to_path_buf()]);
            }

            if let Some(proxy) = self.http.proxy() {
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
                if let Some(val) = std::env::var("SYSTEMROOT")
                    .context("Missing environment variable: SYSTEMROOT!")
                    .log_err()
                {
                    command.env("SYSTEMROOT", val);
                }
                // Without ComSpec, the post-install will always fail.
                if let Some(val) = std::env::var("ComSpec")
                    .context("Missing environment variable: ComSpec!")
                    .log_err()
                {
                    command.env("ComSpec", val);
                }
                command.creation_flags(windows::Win32::System::Threading::CREATE_NO_WINDOW.0);
            }

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

    async fn npm_package_latest_version(&self, name: &str) -> Result<NodeAssetVersion> {
        let output = self
            .run_npm_subcommand(
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
            )
            .await?;

        let mut info: NpmInfo = serde_json::from_slice(&output.stdout)?;
        let version = info
            .dist_tags
            .latest
            .or_else(|| info.versions.pop())
            .ok_or_else(|| anyhow!("no version found for npm package {}", name))?;

        Ok(NodeAssetVersion {
            name: name.to_string(),
            version,
        })
    }

    async fn npm_package_installed_version(
        &self,
        local_package_directory: &PathBuf,
        name: &str,
    ) -> Result<Option<String>> {
        let mut package_json_path = local_package_directory.clone();
        package_json_path.extend(["node_modules", name, "package.json"]);

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

    async fn npm_install_packages(
        &self,
        directory: &Path,
        packages: &[NodeAssetVersion],
    ) -> Result<()> {
        if packages.len() == 0 {
            return Ok(());
        }

        self.check_dependency(packages[0].clone()).await?;

        let packages: Vec<_> = packages
            .into_iter()
            .map(|node_asset_version| {
                format!("{}@{}", node_asset_version.name, node_asset_version.version)
            })
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

        self.run_npm_subcommand(Some(directory), "install", &arguments)
            .await?;
        Ok(())
    }
}

pub struct FakeNodeRuntime;

impl FakeNodeRuntime {
    pub fn new() -> Arc<dyn NodeRuntime> {
        Arc::new(Self)
    }
}

#[async_trait::async_trait]
impl NodeRuntime for FakeNodeRuntime {
    async fn binary_path(&self) -> anyhow::Result<PathBuf> {
        unreachable!()
    }

    async fn run_npm_subcommand(
        &self,
        _: Option<&Path>,
        subcommand: &str,
        args: &[&str],
    ) -> anyhow::Result<Output> {
        unreachable!("Should not run npm subcommand '{subcommand}' with args {args:?}")
    }

    async fn npm_package_latest_version(&self, name: &str) -> anyhow::Result<NodeAssetVersion> {
        unreachable!("Should not query npm package '{name}' for latest version")
    }

    async fn npm_package_installed_version(
        &self,
        _local_package_directory: &PathBuf,
        name: &str,
    ) -> Result<Option<String>> {
        unreachable!("Should not query npm package '{name}' for installed version")
    }

    async fn npm_install_packages(
        &self,
        _: &Path,
        packages: &[NodeAssetVersion],
    ) -> anyhow::Result<()> {
        unreachable!("Should not install packages {packages:?}")
    }
}
