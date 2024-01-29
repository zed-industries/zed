use anyhow::{anyhow, bail, Context, Result};
use async_trait::async_trait;
use futures::StreamExt;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use smol::fs::{self, File};
use std::env::consts::ARCH;
use std::{any::Any, path::PathBuf};
use util::async_maybe;
use util::github::latest_github_release;
use util::{github::GitHubLspBinaryVersion, ResultExt};

pub struct OlsAdapter;

#[async_trait]
impl LspAdapter for OlsAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("ols".into())
    }

    fn short_name(&self) -> &'static str {
        "ols"
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release = latest_github_release("DanielGavin/ols", true, delegate.http_client()).await?;
        let target_name = target_name().unwrap();
        let asset_name = format!("{}.zip", target_name);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;

        let version = GitHubLspBinaryVersion {
            name: release.name,
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
        let target_name = target_name().unwrap();
        let zip_path = container_dir.join(format!("{}.zip", target_name));
        let folder_path = container_dir.join(target_name);
        let binary_path = folder_path.join(target_name);

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
                Err(anyhow!(
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
                Err(anyhow!("failed to unzip ols archive"))?;
            }
        }

        Ok(LanguageServerBinary {
            path: binary_path,
            arguments: vec![],
        })
    }

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir).await
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        get_cached_server_binary(container_dir).await
    }
}

async fn get_cached_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
    async_maybe!({
        let target_name = target_name().unwrap();
        let mut last_binary_path = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            if entry.file_type().await?.is_file()
                && entry
                    .file_name()
                    .to_str()
                    .map_or(false, |name| name == target_name)
            {
                last_binary_path = Some(entry.path());
            }
        }

        if let Some(path) = last_binary_path {
            Ok(LanguageServerBinary {
                path,
                arguments: Vec::new(),
            })
        } else {
            Err(anyhow!("no cached binary"))
        }
    })
    .await
    .log_err()
}

fn target_name() -> Result<&'static str> {
    match ARCH {
        "x86_64"  => Ok("ols-x86_64-darwin"),
        "aarch64" => Ok("ols-arm64-darwin"),
        other     => bail!("Running on unsupported platform: {other}"),
    }
}
