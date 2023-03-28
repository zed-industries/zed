use crate::http::HttpClient;
use anyhow::{Context, Result};
use futures::AsyncReadExt;
use serde::Deserialize;
use std::sync::Arc;

pub struct GitHubLspBinaryVersion {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize)]
pub struct GithubRelease {
    pub name: String,
    pub assets: Vec<GithubReleaseAsset>,
}

#[derive(Deserialize)]
pub struct GithubReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
}

pub async fn latest_github_release(
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
