use anyhow::{anyhow, Context, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use futures::{io::BufReader, StreamExt};
use language::{LanguageServerName, LspAdapter, LspAdapterDelegate};
use lsp::LanguageServerBinary;
use smol::fs;
use std::env::consts::ARCH;
use std::{any::Any, path::PathBuf};
use util::async_maybe;
use util::github::latest_github_release;
use util::{github::GitHubLspBinaryVersion, ResultExt};

pub struct RocLspAdapter;

// GithubRelease { name: "Nightly build", pre_release: true,
//  assets: [
//      GithubReleaseAsset { name: "roc_nightly-linux_arm64-latest.tar.gz", browser_download_url: "https://github.com/roc-lang/roc/releases/download/nightly/roc_nightly-linux_arm64-latest.tar.gz" },
//      GithubReleaseAsset { name: "roc_nightly-linux_x86_64-latest.tar.gz", browser_download_url: "https://github.com/roc-lang/roc/releases/download/nightly/roc_nightly-linux_x86_64-latest.tar.gz" },
//      GithubReleaseAsset { name: "roc_nightly-macos_apple_silicon-latest.tar.gz", browser_download_url: "https://github.com/roc-lang/roc/releases/download/nightly/roc_nightly-macos_apple_silicon-latest.tar.gz" },
//      GithubReleaseAsset { name: "roc_nightly-macos_x86_64-latest.tar.gz", browser_download_url: "https://github.com/roc-lang/roc/releases/download/nightly/roc_nightly-macos_x86_64-latest.tar.gz" },
//      GithubReleaseAsset { name: "roc_nightly-old_linux_arm64-latest.tar.gz", browser_download_url: "https://github.com/roc-lang/roc/releases/download/nightly/roc_nightly-old_linux_arm64-latest.tar.gz" },
//      GithubReleaseAsset { name: "roc_nightly-old_linux_x86_64-latest.tar.gz", browser_download_url: "https://github.com/roc-lang/roc/releases/download/nightly/roc_nightly-old_linux_x86_64-latest.tar.gz" },
//      GithubReleaseAsset { name: "roc_repl_wasm.tar.gz", browser_download_url: "https://github.com/roc-lang/roc/releases/download/nightly/roc_repl_wasm.tar.gz" }
//  ],
//  tarball_url: "https://api.github.com/repos/roc-lang/roc/tarball/nightly",
//  zipball_url: "https://api.github.com/repos/roc-lang/roc/zipball/nightly"
// }
fn select_archive_name_from_arch() -> &'static str {
    if ARCH == "x86_64" {
        "roc_nightly-macos_x86_64-latest.tar.gz"
    } else if ARCH == "aarch64" {
        "roc_nightly-macos_apple_silicon-latest.tar.gz"
    } else {
        // FIXME add better error handling
        panic!(
            "Unsupported architecture: {}, supported are macos intel 64 and apple silicon",
            ARCH
        );
    }
}

const SERVER_BINARY_NAME: &'static str = "roc_lang_server";

#[async_trait]
impl LspAdapter for RocLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName("roc_lang_server".into())
    }

    fn short_name(&self) -> &'static str {
        "roc_lang_server"
    }

    /// Fetch latest build url using github API
    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        log::error!(">> fetching latest server version for roc");

        // calls https://api.github.com/repos/roc-lang/roc/releases
        let release = latest_github_release("roc-lang/roc", true, delegate.http_client()).await?;
        log::error!("found release:\n{:?}", release);
        let archive_name = select_archive_name_from_arch();
        log::error!("searching for asset in release assets: {}", archive_name);
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == archive_name)
            .ok_or_else(|| anyhow!("no asset found matching {:?}", archive_name))?;

        let binary_url = GitHubLspBinaryVersion {
            name: release.name,
            url: asset.browser_download_url.clone(),
        };
        log::error!("found asset url: {}", asset.browser_download_url);
        Ok(Box::new(binary_url) as Box<_>)
    }

    /// Fetches the server binary from the given version (url) if not present in the container_dir,
    /// extracts the binary from the tarball, search for language server binary and
    /// returns the path to the binary
    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        log::error!(
            ">> fetching server binary (if not present) into: {} ",
            container_dir.display()
        );
        let version = version.downcast::<GitHubLspBinaryVersion>().unwrap();

        if let Some(sever_bin_path) = search_server_binary(&container_dir).await? {
            return Ok(LanguageServerBinary {
                path: sever_bin_path,
                arguments: Vec::new(),
            });
        };

        log::error!("downloading release from {}", version.url);
        let mut response = delegate
            .http_client()
            .get(&version.url, Default::default(), true)
            .await
            .context(
                "error downloading roc nightly release from ".to_string() + version.url.as_str(),
            )?;
        log::error!("downloaded release, decompressing...");
        let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
        let archive = Archive::new(decompressed_bytes);
        archive.unpack(&container_dir).await?;
        log::error!("unpacked archive");

        let server_bin_path = search_server_binary(&container_dir).await;
        return if let Ok(Some(bin_path)) = server_bin_path {
            Ok(LanguageServerBinary {
                path: bin_path,
                arguments: Vec::new(),
            })
        } else {
            log::error!("no binary found in {}", container_dir.display());
            Err(anyhow!("no binary found"))
        }
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
        // FIXME how to test if the binary is working?
        // .map(|mut binary| {
        //     binary.arguments = vec!["--help".into()];
        //     binary
        // })
    }
}

async fn get_cached_server_binary(container_dir: PathBuf) -> Option<LanguageServerBinary> {
    async_maybe!({
        log::error!(
            ">> getting cached server binary from {}",
            container_dir.display()
        );

        if let Some(path) = search_server_binary(&container_dir).await? {
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

/// Searches for the language server binary in the given directory
///
/// e.g. given the default container dir: "~/Library/Application Support/Zed/languages/roc_lang_server",
/// search for first roc_nightly-* dir, go into it,
/// and check that a file named "roc_lang_server" exists.
/// if not, an error Ok(None) is returned.
async fn search_server_binary(container_dir: &PathBuf) -> Result<Option<PathBuf>> {
    log::error!(
        ">> searching for server binary in {}",
        container_dir.display()
    );
    let mut entries = fs::read_dir(&container_dir).await?;

    while let Some(entry) = entries.next().await {
        let path = entry?.path();
        log::error!("checking path: {}", path.display());
        // if path is a directory and starts with "/roc_nightly-"
        if path.is_dir() && path.display().to_string().contains("/roc_nightly-") {
            let bin_path = path.join(SERVER_BINARY_NAME);
            if bin_path.exists() {
                if let Ok(metadata) = bin_path.metadata() {
                    if metadata.is_file() {
                        log::error!("found lang server binary: {}", bin_path.display());
                        return Ok(Some(bin_path));
                    } else {
                        log::error!("not a file: {}", bin_path.display());
                    }
                } else {
                    log::error!("could not read metadata: {}", bin_path.display());
                }
            }
        }
    }

    return Ok(None);
}
