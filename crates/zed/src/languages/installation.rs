use anyhow::{anyhow, Context, Result};
use client::http::HttpClient;

use serde::Deserialize;
use smol::io::AsyncReadExt;
use std::{path::Path, sync::Arc};

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
    name: String,
    assets: Vec<GithubReleaseAsset>,
}

#[derive(Deserialize)]
pub(crate) struct GithubReleaseAsset {
    name: String,
    browser_download_url: String,
}

pub async fn npm_package_latest_version(name: &str) -> Result<String> {
    let output = smol::process::Command::new("npm")
        .args(["info", name, "--json"])
        .output()
        .await?;
    if !output.status.success() {
        Err(anyhow!(
            "failed to execute npm info: {:?}",
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
            "failed to execute npm install: {:?}",
            String::from_utf8_lossy(&output.stderr)
        ))?;
    }
    Ok(())
}

pub async fn latest_github_release(
    repo_name_with_owner: &str,
    http: Arc<dyn HttpClient>,
    asset_name: impl Fn(&str) -> String,
) -> Result<GitHubLspBinaryVersion> {
    let mut response = http
        .get(
            &format!("https://api.github.com/repos/{repo_name_with_owner}/releases/latest"),
            Default::default(),
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
    let asset_name = asset_name(&release.name);
    let asset = release
        .assets
        .iter()
        .find(|asset| asset.name == asset_name)
        .ok_or_else(|| anyhow!("no asset found matching {:?}", asset_name))?;
    Ok(GitHubLspBinaryVersion {
        name: release.name,
        url: asset.browser_download_url.clone(),
    })
}
