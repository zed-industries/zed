use crate::HttpClient;
use anyhow::{anyhow, Context, Result};
use futures::AsyncReadExt;
use serde::Deserialize;
use std::sync::Arc;

pub struct GitHubLspBinaryVersion {
    pub name: String,
    pub url: String,
}

#[derive(Deserialize, Debug)]
pub struct GithubRelease {
    pub name: String,
    #[serde(rename = "prerelease")]
    pub pre_release: bool,
    pub assets: Vec<GithubReleaseAsset>,
    pub tarball_url: String,
}

#[derive(Deserialize, Debug)]
pub struct GithubReleaseAsset {
    pub name: String,
    pub browser_download_url: String,
}

pub async fn latest_github_release(
    repo_name_with_owner: &str,
    pre_release: bool,
    http: Arc<dyn HttpClient>,
) -> Result<GithubRelease, anyhow::Error> {
    let mut response = http
        .get(
            &format!("https://api.github.com/repos/{repo_name_with_owner}/releases"),
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

    let releases = match serde_json::from_slice::<Vec<GithubRelease>>(body.as_slice()) {
        Ok(releases) => releases,

        Err(_) => {
            log::error!(
                "Error deserializing Github API response text: {:?}",
                String::from_utf8_lossy(body.as_slice())
            );
            return Err(anyhow!("error deserializing latest release"));
        }
    };

    releases
        .into_iter()
        .find(|release| release.pre_release == pre_release)
        .ok_or(anyhow!("Failed to find a release"))
}
