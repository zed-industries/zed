use anyhow::{anyhow, Context, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use futures::{io::BufReader, StreamExt};
use language::{LanguageServerName, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use smol::fs;
use std::env::consts::ARCH;
use std::ffi::OsString;
use std::{any::Any, path::PathBuf};
use util::async_maybe;
use util::github::latest_github_release;
use util::{github::GitHubLspBinaryVersion, ResultExt};

pub struct OmniSharpAdapter;

#[async_trait]
impl super::LspAdapter for OmniSharpAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("OmniSharp".into())
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        let mapped_arch = match ARCH {
            "aarch64" => Some("arm64"),
            "x86_64" => Some("x64"),
            _ => None,
        };

        match mapped_arch {
            None => Ok(Box::new(())),
            Some(arch) => {
                let release = latest_github_release(
                    "OmniSharp/omnisharp-roslyn",
                    true,
                    false,
                    delegate.http_client(),
                )
                .await?;
                let asset_name = format!("omnisharp-osx-{}-net6.0.tar.gz", arch);
                let asset = release
                    .assets
                    .iter()
                    .find(|asset| asset.name == asset_name)
                    .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;
                let version = GitHubLspBinaryVersion {
                    name: release.tag_name,
                    url: asset.browser_download_url.clone(),
                };

                Ok(Box::new(version) as Box<_>)
            }
        }
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();
        let binary_path = container_dir.join("omnisharp");

        if fs::metadata(&binary_path).await.is_err() {
            let mut response = delegate
                .http_client()
                .get(&version.url, Default::default(), true)
                .await
                .context("error downloading release")?;
            let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
            let archive = Archive::new(decompressed_bytes);
            archive.unpack(container_dir).await?;
        }

        // todo("windows")
        #[cfg(not(windows))]
        {
            fs::set_permissions(
                &binary_path,
                <fs::Permissions as fs::unix::PermissionsExt>::from_mode(0o755),
            )
            .await?;
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
                binary.arguments = vec!["--help".into()];
                binary
            })
    }
}

async fn get_cached_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
    async_maybe!({
        let mut last_binary_path = None;
        let mut entries = fs::read_dir(&container_dir).await?;
        while let Some(entry) = entries.next().await {
            let entry = entry?;
            if entry.file_type().await?.is_file()
                && entry
                    .file_name()
                    .to_str()
                    .map_or(false, |name| name == "omnisharp")
            {
                last_binary_path = Some(entry.path());
            }
        }

        if let Some(path) = last_binary_path {
            Ok(LanguageServerBinary {
                path,
                env: None,
                arguments: server_binary_arguments(),
            })
        } else {
            Err(anyhow!("no cached binary"))
        }
    })
    .await
    .log_err()
}

fn server_binary_arguments() -> Vec<OsString> {
    vec!["-lsp".into()]
}
