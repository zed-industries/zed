use anyhow::{anyhow, bail, Context, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use serde::Deserialize;
use smol::{fs, io::BufReader, process::Command};
use std::process::{Output, Stdio};
use std::{
    env::consts,
    path::{Path, PathBuf},
    sync::Arc,
};
use util::http::HttpClient;

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
}

pub struct RealNodeRuntime {
    http: Arc<dyn HttpClient>,
}

impl RealNodeRuntime {
    pub fn new(http: Arc<dyn HttpClient>) -> Arc<dyn NodeRuntime> {
        Arc::new(RealNodeRuntime { http })
    }

    async fn install_if_needed(&self) -> Result<PathBuf> {
        log::info!("Node runtime install_if_needed");

        let arch = match consts::ARCH {
            "x86_64" => "x64",
            "aarch64" => "arm64",
            other => bail!("Running on unsupported platform: {other}"),
        };

        let folder_name = format!("node-{VERSION}-darwin-{arch}");
        let node_containing_dir = util::paths::SUPPORT_DIR.join("node");
        let node_dir = node_containing_dir.join(folder_name);
        let node_binary = node_dir.join("bin/node");
        let npm_file = node_dir.join("bin/npm");

        let result = Command::new(&node_binary)
            .arg(npm_file)
            .arg("--version")
            .stdin(Stdio::null())
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .status()
            .await;
        let valid = matches!(result, Ok(status) if status.success());

        if !valid {
            _ = fs::remove_dir_all(&node_containing_dir).await;
            fs::create_dir(&node_containing_dir)
                .await
                .context("error creating node containing dir")?;

            let file_name = format!("node-{VERSION}-darwin-{arch}.tar.gz");
            let url = format!("https://nodejs.org/dist/{VERSION}/{file_name}");
            let mut response = self
                .http
                .get(&url, Default::default(), true)
                .await
                .context("error downloading Node binary tarball")?;

            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let archive = Archive::new(decompressed_bytes);
            archive.unpack(&node_containing_dir).await?;
        }

        anyhow::Ok(node_dir)
    }
}

#[async_trait::async_trait]
impl NodeRuntime for RealNodeRuntime {
    async fn binary_path(&self) -> Result<PathBuf> {
        let installation_path = self.install_if_needed().await?;
        Ok(installation_path.join("bin/node"))
    }

    async fn run_npm_subcommand(
        &self,
        directory: Option<&Path>,
        subcommand: &str,
        args: &[&str],
    ) -> Result<Output> {
        let attempt = || async move {
            let installation_path = self.install_if_needed().await?;

            let mut env_path = installation_path.join("bin").into_os_string();
            if let Some(existing_path) = std::env::var_os("PATH") {
                if !existing_path.is_empty() {
                    env_path.push(":");
                    env_path.push(&existing_path);
                }
            }

            let node_binary = installation_path.join("bin/node");
            let npm_file = installation_path.join("bin/npm");

            if smol::fs::metadata(&node_binary).await.is_err() {
                return Err(anyhow!("missing node binary file"));
            }

            if smol::fs::metadata(&npm_file).await.is_err() {
                return Err(anyhow!("missing npm file"));
            }

            let mut command = Command::new(node_binary);
            command.env("PATH", env_path);
            command.arg(npm_file).arg(subcommand).args(args);

            if let Some(directory) = directory {
                command.current_dir(directory);
            }

            command.output().await.map_err(|e| anyhow!("{e}"))
        };

        let mut output = attempt().await;
        if output.is_err() {
            output = attempt().await;
            if output.is_err() {
                return Err(anyhow!(
                    "failed to launch npm subcommand {subcommand} subcommand"
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

    async fn npm_package_latest_version(&self, name: &str) -> Result<String> {
        let output = self
            .run_npm_subcommand(
                None,
                "info",
                &[
                    name,
                    "--json",
                    "-fetch-retry-mintimeout",
                    "2000",
                    "-fetch-retry-maxtimeout",
                    "5000",
                    "-fetch-timeout",
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
            "-fetch-retry-mintimeout",
            "2000",
            "-fetch-retry-maxtimeout",
            "5000",
            "-fetch-timeout",
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
        Arc::new(FakeNodeRuntime)
    }
}

#[async_trait::async_trait]
impl NodeRuntime for FakeNodeRuntime {
    async fn binary_path(&self) -> anyhow::Result<PathBuf> {
        // TODO kb move away into a separate type + a Project's setter (for test code)
        Ok(PathBuf::from("prettier_fake_node"))
    }

    async fn run_npm_subcommand(&self, _: Option<&Path>, _: &str, _: &[&str]) -> Result<Output> {
        unreachable!()
    }

    async fn npm_package_latest_version(&self, name: &str) -> anyhow::Result<String> {
        if name == "prettier" {
            Ok("0.0.1".to_string())
        } else {
            unreachable!()
        }
    }

    async fn npm_install_packages(
        &self,
        _: &Path,
        packages: &[(&str, &str)],
    ) -> anyhow::Result<()> {
        if packages == [("prettier", "0.0.1")] {
            Ok(())
        } else {
            unreachable!()
        }
    }
}
