use anyhow::{anyhow, bail, Context, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use futures::lock::Mutex;
use futures::{future::Shared, FutureExt};
use gpui::{executor::Background, Task};
use serde::Deserialize;
use smol::{fs, io::BufReader, process::Command};
use std::process::Output;
use std::{
    env::consts,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};
use util::{http::HttpClient, ResultExt};

const VERSION: &str = "v18.15.0";

static RUNTIME_INSTANCE: OnceLock<Arc<NodeRuntime>> = OnceLock::new();

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

pub struct NodeRuntime {
    http: Arc<dyn HttpClient>,
    background: Arc<Background>,
    installation_path: Mutex<Option<Shared<Task<Result<PathBuf, Arc<anyhow::Error>>>>>>,
}

impl NodeRuntime {
    pub fn instance(http: Arc<dyn HttpClient>, background: Arc<Background>) -> Arc<NodeRuntime> {
        RUNTIME_INSTANCE
            .get_or_init(|| {
                Arc::new(NodeRuntime {
                    http,
                    background,
                    installation_path: Mutex::new(None),
                })
            })
            .clone()
    }

    pub async fn binary_path(&self) -> Result<PathBuf> {
        let installation_path = self.install_if_needed().await?;
        Ok(installation_path.join("bin/node"))
    }

    pub async fn run_npm_subcommand(
        &self,
        directory: Option<&Path>,
        subcommand: &str,
        args: &[&str],
    ) -> Result<Output> {
        let attempt = |installation_path: PathBuf| async move {
            let node_binary = installation_path.join("bin/node");
            let npm_file = installation_path.join("bin/npm");

            if smol::fs::metadata(&node_binary).await.is_err() {
                return Err(anyhow!("missing node binary file"));
            }

            if smol::fs::metadata(&npm_file).await.is_err() {
                return Err(anyhow!("missing npm file"));
            }

            let mut command = Command::new(node_binary);
            command.arg(npm_file).arg(subcommand).args(args);

            if let Some(directory) = directory {
                command.current_dir(directory);
            }

            command.output().await.map_err(|e| anyhow!("{e}"))
        };

        let installation_path = self.install_if_needed().await?;
        let mut output = attempt(installation_path).await;
        if output.is_err() {
            let installation_path = self.reinstall().await?;
            output = attempt(installation_path).await;
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

    pub async fn npm_package_latest_version(&self, name: &str) -> Result<String> {
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

    pub async fn npm_install_packages(
        &self,
        directory: &Path,
        packages: impl IntoIterator<Item = (&str, &str)>,
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

    async fn reinstall(&self) -> Result<PathBuf> {
        log::info!("beginnning to reinstall Node runtime");
        let mut installation_path = self.installation_path.lock().await;

        if let Some(task) = installation_path.as_ref().cloned() {
            if let Ok(installation_path) = task.await {
                smol::fs::remove_dir_all(&installation_path)
                    .await
                    .context("node dir removal")
                    .log_err();
            }
        }

        let http = self.http.clone();
        let task = self
            .background
            .spawn(async move { Self::install(http).await.map_err(Arc::new) })
            .shared();

        *installation_path = Some(task.clone());
        task.await.map_err(|e| anyhow!("{}", e))
    }

    async fn install_if_needed(&self) -> Result<PathBuf> {
        let task = self
            .installation_path
            .lock()
            .await
            .get_or_insert_with(|| {
                let http = self.http.clone();
                self.background
                    .spawn(async move { Self::install(http).await.map_err(Arc::new) })
                    .shared()
            })
            .clone();

        task.await.map_err(|e| anyhow!("{}", e))
    }

    async fn install(http: Arc<dyn HttpClient>) -> Result<PathBuf> {
        log::info!("installing Node runtime");
        let arch = match consts::ARCH {
            "x86_64" => "x64",
            "aarch64" => "arm64",
            other => bail!("Running on unsupported platform: {other}"),
        };

        let folder_name = format!("node-{VERSION}-darwin-{arch}");
        let node_containing_dir = util::paths::SUPPORT_DIR.join("node");
        let node_dir = node_containing_dir.join(folder_name);
        let node_binary = node_dir.join("bin/node");

        if fs::metadata(&node_binary).await.is_err() {
            _ = fs::remove_dir_all(&node_containing_dir).await;
            fs::create_dir(&node_containing_dir)
                .await
                .context("error creating node containing dir")?;

            let file_name = format!("node-{VERSION}-darwin-{arch}.tar.gz");
            let url = format!("https://nodejs.org/dist/{VERSION}/{file_name}");
            let mut response = http
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
