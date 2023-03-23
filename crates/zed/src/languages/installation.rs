use anyhow::{anyhow, Context, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use client::http::HttpClient;
use futures::{io::BufReader, StreamExt};
use serde::Deserialize;
use smol::fs::{self, File};
use smol::io::AsyncReadExt;
use std::{
    path::{Path, PathBuf},
    sync::Arc,
};

pub struct GitHubLspBinaryVersion {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "kebab-case")]
struct NpmInfo {
    #[serde(default)]
    dist_tags: NpmInfoDistTags,
    versions: Vec<String>,
}

#[derive(Deserialize, Default)]
struct NpmInfoDistTags {
    latest: Option<String>,
}

#[derive(Deserialize)]
pub(crate) struct GithubRelease {
    pub name: String,
    pub assets: Vec<GithubReleaseAsset>,
}

#[derive(Deserialize)]
pub(crate) struct GithubReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
}

pub async fn ensure_node_installation_dir(http: Arc<dyn HttpClient>) -> Result<PathBuf> {
    eprintln!("ensure_node_installation_dir");

    let version = "v18.15.0";
    let arch = "arm64";

    let folder_name = format!("node-{version}-darwin-{arch}");
    let node_containing_dir = dbg!(util::paths::SUPPORT_DIR.join("node"));
    let node_dir = dbg!(node_containing_dir.join(folder_name));
    let node_binary = node_dir.join("bin/node");

    if fs::metadata(&node_binary).await.is_err() {
        _ = fs::remove_dir_all(&node_containing_dir).await;
        fs::create_dir(&node_containing_dir)
            .await
            .context("error creating node containing dir")?;

        let url = format!("https://nodejs.org/dist/{version}/node-{version}-darwin-{arch}.tar.gz");
        dbg!(&url);
        let mut response = http
            .get(&url, Default::default(), true)
            .await
            .context("error downloading Node binary tarball")?;

        let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
        let archive = Archive::new(decompressed_bytes);
        archive.unpack(&node_containing_dir).await?;
        eprintln!("unpacked");
    }

    eprintln!("returning");
    Ok(dbg!(node_dir))
}

pub async fn npm_package_latest_version(name: &str) -> Result<String> {
    let output = smol::process::Command::new("npm")
        .args(["-fetch-retry-mintimeout", "2000"])
        .args(["-fetch-retry-maxtimeout", "5000"])
        .args(["info", name, "--json"])
        .output()
        .await
        .context("failed to run npm info")?;
    if !output.status.success() {
        Err(anyhow!(
            "failed to execute npm info:\nstdout: {:?}\nstderr: {:?}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ))?;
    }
    let mut info: NpmInfo = serde_json::from_slice(&output.stdout)?;
    info.dist_tags
        .latest
        .or_else(|| info.versions.pop())
        .ok_or_else(|| anyhow!("no version found for npm package {}", name))
}

pub async fn npm_install_packages(
    packages: impl IntoIterator<Item = (&str, &str)>,
    directory: &Path,
) -> Result<()> {
    let output = smol::process::Command::new("npm")
        .args(["-fetch-retry-mintimeout", "2000"])
        .args(["-fetch-retry-maxtimeout", "5000"])
        .arg("install")
        .arg("--prefix")
        .arg(directory)
        .args(
            packages
                .into_iter()
                .map(|(name, version)| format!("{name}@{version}")),
        )
        .output()
        .await
        .context("failed to run npm install")?;
    if !output.status.success() {
        Err(anyhow!(
            "failed to execute npm install:\nstdout: {:?}\nstderr: {:?}",
            String::from_utf8_lossy(&output.stdout),
            String::from_utf8_lossy(&output.stderr)
        ))?;
    }
    Ok(())
}

pub(crate) async fn latest_github_release(
    repo_name_with_owner: &str,
    http: Arc<dyn HttpClient>,
) -> Result<GithubRelease, anyhow::Error> {
    let mut response = http
        .get(
            &format!("https://api.github.com/repos/{repo_name_with_owner}/releases/latest"),
            Default::default(),
            true,
        )
        .await
        .context("error fetching latest release")?;
    let mut body = Vec::new();
    response
        .body_mut()
        .read_to_end(&mut body)
        .await
        .context("error reading latest release")?;
    let release: GithubRelease =
        serde_json::from_slice(body.as_slice()).context("error deserializing latest release")?;
    Ok(release)
}
