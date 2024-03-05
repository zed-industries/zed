use std::any::Any;
use std::env::consts;
use std::ffi::OsString;
use std::path::PathBuf;

use anyhow::{anyhow, bail, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use futures::io::BufReader;
use futures::StreamExt;
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use smol::fs;
use util::github::{latest_github_release, GitHubLspBinaryVersion};
use util::{async_maybe, ResultExt};

fn server_binary_arguments() -> Vec<OsString> {
    vec!["lsp".into()]
}

pub struct GleamLspAdapter;

#[async_trait]
impl LspAdapter for GleamLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("gleam".into())
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let release =
            latest_github_release("gleam-lang/gleam", true, false, delegate.http_client()).await?;
        let asset_name = format!(
            "gleam-{version}-{arch}-{os}.tar.gz",
            version = release.tag_name,
            arch = std::env::consts::ARCH,
            os = match consts::OS {
                "macos" => "apple-darwin",
                "linux" => "unknown-linux-musl",
                "windows" => "pc-windows-msvc",
                other => bail!("Running on unsupported os: {other}"),
            },
        );
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;
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
        let binary_path = container_dir.join("gleam");

        if fs::metadata(&binary_path).await.is_err() {
            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading release: {}", err))?;
            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let archive = Archive::new(decompressed_bytes);
            archive.unpack(container_dir).await?;
        }

        Ok(LanguageServerBinary {
            path: binary_path,
            env: None,
            arguments: server_binary_arguments(),
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
                binary.arguments = vec!["--version".into()];
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
            path: last.ok_or_else(|| anyhow!("no cached binary"))?,
            env: None,
            arguments: server_binary_arguments(),
        })
    })
    .await
    .log_err()
}
