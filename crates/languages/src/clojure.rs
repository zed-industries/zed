use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
pub use language::*;
use lsp::LanguageServerBinary;
use smol::fs::{self, File};
use std::{any::Any, env::consts, path::PathBuf};
use util::{
    fs::remove_matching,
    github::{latest_github_release, GitHubLspBinaryVersion},
};

#[derive(Copy, Clone)]
pub struct ClojureLspAdapter;

#[async_trait]
impl super::LspAdapter for ClojureLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("clojure-lsp".into())
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release = latest_github_release(
            "clojure-lsp/clojure-lsp",
            true,
            false,
            delegate.http_client(),
        )
        .await?;
        let os = match consts::OS {
            "macos" => "macos",
            "linux" => "linux",
            "windows" => "windows",
            other => bail!("Running on unsupported os: {other}"),
        };
        let platform = match consts::ARCH {
            "x86_64" => "amd64",
            "aarch64" => "aarch64",
            other => bail!("Running on unsupported platform: {other}"),
        };
        let asset_name = format!("clojure-lsp-native-{os}-{platform}.zip");
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;
        let version = GitHubLspBinaryVersion {
            name: release.tag_name.clone(),
            url: asset.browser_download_url.clone(),
        };
        Ok(Box::new(version) as Box<_>)
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let zip_path = container_dir.join(format!("clojure-lsp_{}.zip", version.name));
        let folder_path = container_dir.join("bin");
        let binary_path = folder_path.join("clojure-lsp");

        if fs::metadata(&binary_path).await.is_err() {
            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .context("error downloading release")?;
            let mut file = File::create(&zip_path)
                .await
                .with_context(|| format!("failed to create file {}", zip_path.display()))?;
            if !response.status().is_success() {
                return Err(anyhow!(
                    "download failed with status {}",
                    response.status().to_string()
                ))?;
            }
            futures::io::copy(response.body_mut(), &mut file).await?;

            fs::create_dir_all(&folder_path)
                .await
                .with_context(|| format!("failed to create directory {}", folder_path.display()))?;

            let unzip_status = smol::process::Command::new("unzip")
                .arg(&zip_path)
                .arg("-d")
                .arg(&folder_path)
                .output()
                .await?
                .status;
            if !unzip_status.success() {
                return Err(anyhow!("failed to unzip elixir-ls archive"))?;
            }

            remove_matching(&container_dir, |entry| entry != folder_path).await;
        }

        Ok(LanguageServerBinary {
            path: binary_path,
            env: None,
            arguments: vec![],
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        let binary_path = container_dir.join("bin").join("clojure-lsp");
        if binary_path.exists() {
            Some(LanguageServerBinary {
                path: binary_path,
                env: None,
                arguments: vec![],
            })
        } else {
            None
        }
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        let binary_path = container_dir.join("bin").join("clojure-lsp");
        if binary_path.exists() {
            Some(LanguageServerBinary {
                path: binary_path,
                env: None,
                arguments: vec!["--version".into()],
            })
        } else {
            None
        }
    }
}
