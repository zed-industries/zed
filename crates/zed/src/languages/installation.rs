use anyhow::{anyhow, Context, Result};
use client::http::{self, HttpClient, Method};
use serde::Deserialize;
use std::{path::Path, sync::Arc};

pub struct GitHubLspBinaryVersion {
    pub name: String,
    pub url: http::Url,
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
    browser_download_url: http::Url,
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
    let release = http
        .send(
            surf::RequestBuilder::new(
                Method::Get,
                http::Url::parse(&format!(
                    "https://api.github.com/repos/{repo_name_with_owner}/releases/latest"
                ))
                .unwrap(),
            )
            .middleware(surf::middleware::Redirect::default())
            .build(),
        )
        .await
        .map_err(|err| anyhow!("error fetching latest release: {}", err))?
        .body_json::<GithubRelease>()
        .await
        .map_err(|err| anyhow!("error parsing latest release: {}", err))?;
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
