use anyhow::{anyhow, bail, Context, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_zip::base::read::stream::ZipFileReader;
use async_zip::ZipString;
use futures::{AsyncBufRead, AsyncReadExt};
use semver::Version;
use serde::Deserialize;
use smol::io;
use smol::{fs, io::BufReader, lock::Mutex, process::Command};
use std::ffi::{OsStr, OsString};
use std::process::Output;
use std::str::from_utf8;
use std::{
    env::consts,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::http::HttpClient;
use util::ResultExt;

const VERSION: &str = "v18.15.0";

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
pub trait NodeRuntime: Send + Sync {
    async fn binary_path(&self) -> Result<PathBuf>;

    async fn run_npm_subcommand(
        &self,
        directory: Option<&Path>,
        subcommand: &str,
        args: &[&str],
    ) -> Result<Output>;

    async fn npm_package_latest_version(&self, name: &str) -> Result<String>;

    async fn npm_install_packages(&self, directory: &Path, packages: &[(&str, &str)])
        -> Result<()>;

    async fn npm_package_installed_version(
        &self,
        local_package_directory: &PathBuf,
        name: &str,
    ) -> Result<Option<String>>;

    async fn should_install_npm_package(
        &self,
        package_name: &str,
        local_executable_path: &Path,
        local_package_directory: &PathBuf,
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
        let Some(latest_version) = Version::parse(&latest_version).log_err() else {
            return true;
        };

        installed_version < latest_version
    }
}

pub struct RealNodeRuntime {
    http: Arc<dyn HttpClient>,
    installation_lock: Mutex<()>,
}

impl RealNodeRuntime {
    pub fn new(http: Arc<dyn HttpClient>) -> Arc<dyn NodeRuntime> {
        Arc::new(RealNodeRuntime {
            http,
            installation_lock: Mutex::new(()),
        })
    }

    fn get_node_bin(node_dir: &Path) -> PathBuf {
        if consts::OS != "windows" {
            node_dir.join("bin")
        } else {
            node_dir.to_path_buf()
        }
    }

    fn get_node_executable(node_dir: &Path) -> PathBuf {
        Self::get_node_bin(node_dir).join(if consts::OS != "windows" {
            "node"
        } else {
            "node.exe"
        })
    }

    fn get_npm_executable(node_dir: &Path) -> PathBuf {
        Self::get_node_bin(node_dir).join(if consts::OS != "windows" {
            "npm"
        } else {
            "node_modules/npm/bin/npm-cli.js"
        })
    }

    fn get_node_env_path(node_dir: &Path) -> OsString {
        let mut env_path = Self::get_node_bin(node_dir).into_os_string();

        if let Some(existing_path) = std::env::var_os("PATH") {
            if !existing_path.is_empty() {
                env_path.push(if consts::OS != "windows" { ":" } else { ";" });
                env_path.push(existing_path);
            }
        }

        env_path
    }

    fn create_command(node_dir: &Path, program: impl AsRef<OsStr>) -> Command {
        let mut command = Command::new(program);
        command.env_clear();
        command.env("PATH", Self::get_node_env_path(node_dir));

        #[cfg(target_os = "windows")]
        {
            use smol::process::windows::CommandExt;
            const CREATE_NO_WINDOW: u32 = 0x08000000;
            command.creation_flags(CREATE_NO_WINDOW);
        }

        command
    }

    fn create_node_command(node_dir: &Path) -> Command {
        Self::create_command(node_dir, Self::get_node_executable(node_dir))
    }

    fn create_npm_command(node_dir: &Path) -> Command {
        let mut command = Self::create_command(node_dir, Self::get_node_executable(node_dir));

        command
            .arg(Self::get_npm_executable(node_dir))
            .arg("--cache")
            .arg(node_dir.join("cache"))
            .arg("--userconfig")
            .arg(node_dir.join("blank_user_npmrc"))
            .arg("--globalconfig")
            .arg(node_dir.join("blank_global_npmrc"));

        command
    }

    async fn install_if_needed(&self) -> Result<PathBuf> {
        let _lock = self.installation_lock.lock().await;
        log::info!("checking if Node is installed...");

        let os = match consts::OS {
            "macos" => "darwin",
            "linux" => "linux",
            "windows" => "win",
            other => bail!("unsupported operating system: {other}"),
        };

        let arch = match consts::ARCH {
            "x86_64" => "x64",
            "aarch64" => "arm64",
            other => bail!("unsupported CPU architecture: {other}"),
        };

        let nodes_dir = util::paths::SUPPORT_DIR.join("node");
        let node_dir = nodes_dir.join(format!("node-{VERSION}-{os}-{arch}"));
        log::info!("node directory: {}", node_dir.display());

        let node_valid = match Self::create_node_command(&node_dir)
            .arg("--version")
            .output()
            .await
        {
            Ok(output) => {
                if output.status.success() {
                    match from_utf8(&output.stdout).ok() {
                        Some(version) => {
                            log::info!("`node --version`: {}", version.trim());
                            true
                        }
                        None => {
                            log::warn!("`node --version` succeeded, but returned invalid UTF-8");
                            false
                        }
                    }
                } else {
                    log::warn!("node returned non-zero exit code: {}", output.status);
                    false
                }
            }

            Err(error) => {
                log::warn!("failed to execute node subprocess: {}", error);
                false
            }
        };

        let npm_valid = match Self::create_npm_command(&node_dir)
            .arg("--version")
            .output()
            .await
        {
            Ok(output) => {
                if output.status.success() {
                    match from_utf8(&output.stdout).ok() {
                        Some(version) => {
                            log::info!("`npm --version`: {}", version.trim());
                            true
                        }
                        None => {
                            log::warn!("`npm --version` succeeded, but returned invalid UTF-8");
                            false
                        }
                    }
                } else {
                    log::warn!("npm returned non-zero exit code: {}", output.status);
                    false
                }
            }

            Err(error) => {
                log::warn!("failed to execute npm subprocess: {}", error);
                false
            }
        };

        if !node_valid || !npm_valid {
            log::info!("maybe node needs a reinstall...");

            // nuke from orbit and reinstall
            _ = fs::remove_dir_all(&nodes_dir).await;

            fs::create_dir(&nodes_dir)
                .await
                .context("creating node versions directory")?;

            let is_windows = os == "win";
            let archive_is_zip = is_windows;
            let archive_ext = if is_windows { ".zip" } else { ".tar.gz" };
            let archive_name = format!("node-{VERSION}-{os}-{arch}{archive_ext}");
            let archive_url = format!("https://nodejs.org/dist/{VERSION}/{archive_name}");
            log::info!("fetching node distribution from {}", archive_url);

            let stream = self
                .http
                .get(&archive_url, Default::default(), true)
                .await
                .context("fetching node distribution")?
                .into_body();

            log::info!(
                "extracting node distribution {} to {}",
                archive_name,
                nodes_dir.display()
            );

            if !archive_is_zip {
                let decompressed_bytes = GzipDecoder::new(BufReader::new(stream));
                let archive = Archive::new(decompressed_bytes);
                archive
                    .unpack(&nodes_dir)
                    .await
                    .context("extracting node distribution tar archive")?;
            } else {
                Self::extract_zip_stream_to_directory(BufReader::new(stream), &nodes_dir)
                    .await
                    .context("extracting node distribution zip archive")?;
            }
        }

        // Note: Not in the `if !valid {}` so we can populate these for existing installations
        _ = fs::create_dir(node_dir.join("cache")).await;
        _ = fs::write(node_dir.join("blank_user_npmrc"), []).await;
        _ = fs::write(node_dir.join("blank_global_npmrc"), []).await;

        anyhow::Ok(node_dir)
    }

    fn resolve_zip_path(path: &ZipString, base: &mut PathBuf) -> Result<usize> {
        const WINDOWS_ILLEGAL_CHARS: &[char] = &['<', '>', ':', '"', '|', '?', '*'];
        const WINDOWS_ILLEGAL_NAMES: &[&str] = &[
            "CON", "PRN", "AUX", "NUL", "COM1", "COM2", "COM3", "COM4", "COM5", "COM6", "COM7",
            "COM8", "COM9", "LPT1", "LPT2", "LPT3", "LPT4", "LPT5", "LPT6", "LPT7", "LPT8", "LPT9",
        ];

        let path = path
            .as_str()
            .ok()
            .or_else(|| from_utf8(path.as_bytes()).ok())
            .context("encountered non-UTF-8 path")?;

        let mut depth = 0usize;

        for component in path.split(&['/', '\\']) {
            match component {
                "" | "." => {}

                ".." => {
                    if depth > 0 {
                        depth -= 1;
                        base.pop();
                    }
                }

                component => {
                    depth = depth
                        .checked_add(1)
                        .ok_or(anyhow!("path is too long: {}", path))?;

                    for illegal in WINDOWS_ILLEGAL_NAMES.into_iter() {
                        if component.eq_ignore_ascii_case(illegal) {
                            bail!("path contains illegal component {}: {}", illegal, path);
                        }
                    }

                    base.push(component.replace(WINDOWS_ILLEGAL_CHARS, "_"));
                }
            }
        }

        Ok(depth)
    }

    async fn extract_zip_stream_to_directory(
        stream: impl AsyncBufRead + Unpin,
        destination: &Path,
    ) -> Result<()> {
        let destination = destination
            .to_owned()
            .canonicalize()
            .context("resolving destination directory")?;

        let mut stream = ZipFileReader::new(stream);

        while let Some(mut file_reader) = stream
            .next_with_entry()
            .await
            .context("reading zip stream")?
        {
            let entry_reader = file_reader.reader_mut();
            let entry = entry_reader.entry();

            let mut path = destination.clone();
            let depth = Self::resolve_zip_path(entry.filename(), &mut path)
                .context("resolving item path")?;

            if entry
                .dir()
                .context(anyhow!("checking if item is directory: {}", path.display()))?
            {
                log::info!("creating directory from zip archive: {}", path.display());
                fs::DirBuilder::new()
                    .recursive(true)
                    .create(&path)
                    .await
                    .context(anyhow!("creating directory item: {}", path.display()))?;
                stream = file_reader.skip().await.context("reading zip archive")?;
            } else {
                if depth < 1 {
                    stream = file_reader.skip().await?;
                } else {
                    log::info!("extracting file from zip archive: {}", path.display());

                    if let Some(parent) = (depth > 1).then(|| path.parent()).flatten() {
                        fs::DirBuilder::new()
                            .recursive(true)
                            .create(parent)
                            .await
                            .context(anyhow!(
                                "creating parent directories: {}",
                                parent.display()
                            ))?;
                    }

                    match fs::OpenOptions::new()
                        .create_new(true)
                        .write(true)
                        .open(&path)
                        .await
                    {
                        Err(e) if e.kind() == io::ErrorKind::AlreadyExists => {
                            log::info!("destination file already exists, skipping");
                            stream = file_reader.skip().await.context("reading zip archive")?;
                        }

                        result => {
                            let mut file = result
                                .context(anyhow!("creating file item: {}", path.display()))?;

                            io::copy(entry_reader, &mut file)
                                .await
                                .context(anyhow!("writing file content: {}", path.display()))?;

                            stream = file_reader.done().await.context("reading zip archive")?;
                        }
                    };
                }
            }
        }

        Ok(())
    }
}

#[async_trait::async_trait]
impl NodeRuntime for RealNodeRuntime {
    async fn binary_path(&self) -> Result<PathBuf> {
        let node_dir = self.install_if_needed().await?;
        Ok(Self::get_node_executable(&node_dir))
    }

    async fn run_npm_subcommand(
        &self,
        directory: Option<&Path>,
        subcommand: &str,
        args: &[&str],
    ) -> Result<Output> {
        let attempt = || async move {
            let node_dir = self
                .install_if_needed()
                .await
                .context("install node if needed")?;

            let node_executable = Self::get_node_executable(&node_dir);
            let npm_executable = Self::get_npm_executable(&node_dir);

            if let Err(e) = smol::fs::metadata(&node_executable).await {
                return Err(
                    anyhow!("missing node executable: {}", node_executable.display()).context(e),
                );
            }

            if let Err(e) = smol::fs::metadata(&npm_executable).await {
                return Err(
                    anyhow!("missing npm executable: {}", npm_executable.display()).context(e),
                );
            }

            let mut command = Self::create_npm_command(&node_dir);

            if let Some(directory) = directory {
                fs::DirBuilder::new()
                    .recursive(true)
                    .create(directory)
                    .await
                    .context(anyhow!(
                        "creating working directory: {}",
                        directory.display()
                    ))?;

                command.current_dir(directory);
                command.args([OsStr::new("--prefix"), directory.as_os_str()]);
            }

            command.arg(subcommand);
            command.args(args);
            log::info!("executing command {command:?}");

            match command.output().await.context("executing npm subprocess")? {
                output if !output.status.success() => {
                    log::error!(
                        "{} returned from command {:?}\nstdout: {:?}\nstderr: {:?}",
                        output.status,
                        command,
                        String::from_utf8_lossy(&output.stdout),
                        String::from_utf8_lossy(&output.stderr)
                    );

                    bail!("subprocess returned {}", output.status)
                }

                output => {
                    log::info!(
                        "{} returned from command {:?}\nstderr: {:?}",
                        output.status,
                        command,
                        String::from_utf8_lossy(&output.stderr)
                    );

                    Ok(output)
                }
            }
        };

        let mut output = attempt()
            .await
            .context("first attempt to execute the command");

        if let Err(e) = output {
            output = attempt()
                .await
                .context(e)
                .context("second and final attempt to execute the command");
        }

        output.context(anyhow!(
            "executing `npm {:?}` with args {:?}",
            subcommand,
            args
        ))
    }

    async fn npm_package_latest_version(&self, name: &str) -> Result<String> {
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
        info.dist_tags
            .latest
            .or_else(|| info.versions.pop())
            .ok_or_else(|| anyhow!("no version found for npm package {}", name))
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
                if err.kind() == std::io::ErrorKind::NotFound {
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
        packages: &[(&str, &str)],
    ) -> Result<()> {
        let packages: Vec<_> = packages
            .into_iter()
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

    async fn npm_package_latest_version(&self, name: &str) -> anyhow::Result<String> {
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
        packages: &[(&str, &str)],
    ) -> anyhow::Result<()> {
        unreachable!("Should not install packages {packages:?}")
    }
}
