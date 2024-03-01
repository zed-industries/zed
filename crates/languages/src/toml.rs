use anyhow::{bail, Context, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_trait::async_trait;
use futures::{io::BufReader, StreamExt};
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use smol::fs::{self, File};
use std::{any::Any, path::PathBuf};
use util::async_maybe;
use util::github::latest_github_release;
use util::{github::GitHubLspBinaryVersion, ResultExt};

pub struct TaploLspAdapter;

#[async_trait]
impl LspAdapter for TaploLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("taplo-ls".into())
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release =
            latest_github_release("tamasfe/taplo", true, false, delegate.http_client()).await?;
        let asset_name = format!(
            "taplo-full-{os}-{arch}.gz",
            os = match std::env::consts::OS {
                "macos" => "darwin",
                "linux" => "linux",
                "windows" => "windows",
                other => bail!("Running on unsupported os: {other}"),
            },
            arch = std::env::consts::ARCH
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .context(format!("no asset found matching {asset_name:?}"))?;

        Ok(Box::new(GitHubLspBinaryVersion {
            name: release.tag_name,
            url: asset.browser_download_url.clone(),
        }))
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let binary_path = container_dir.join("taplo");

        if fs::metadata(&binary_path).await.is_err() {
            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .context("error downloading release")?;

            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let mut file = File::create(&binary_path).await?;

            futures::io::copy(decompressed_bytes, &mut file).await?;

            // todo("windows")
            #[cfg(not(windows))]
            {
                fs::set_permissions(
                    &binary_path,
                    <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
                )
                .await?;
            }
        }

        Ok(LanguageServerBinary {
            path: binary_path,
            env: None,
            arguments: vec!["lsp".into(), "stdio".into()],
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
        get_cached_server_binary(container_dir)
            .await
            .map(|mut binary| {
                binary.arguments = vec!["--help".into()];
                binary
            })
    }
}

async fn get_cached_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
    async_maybe!({
        let mut last = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            last = Some(entry?.path());
        }

        anyhow::Ok(LanguageServerBinary {
            path: last.context("no cached binary")?,
            env: None,
            arguments: Default::default(),
        })
    })
    .await
    .log_err()
}
