use anyhow::{Context as _, Result};
use futures::AsyncReadExt;
use futures::io::BufReader;
use http_client::{AsyncBody, HttpClient, Request as HttpRequest};
use paths::supermaven_dir;
use serde::Deserialize;
use smol::fs::{self, File};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use util::fs::{make_file_executable, remove_matching};

#[derive(Deserialize)]
pub struct SupermavenApiError {
    pub message: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupermavenDownloadResponse {
    pub download_url: String,
    pub version: u64,
    pub sha256_hash: String,
}

pub async fn latest_release(
    client: Arc<dyn HttpClient>,
    platform: &str,
    arch: &str,
) -> Result<SupermavenDownloadResponse> {
    let uri = format!(
        "https://supermaven.com/api/download-path?platform={}&arch={}",
        platform, arch
    );

    // Download is not authenticated
    let request = HttpRequest::get(&uri);

    let mut response = client
        .send(request.body(AsyncBody::default())?)
        .await
        .with_context(|| "Unable to acquire Supermaven Agent".to_string())?;

    let mut body = Vec::new();
    response.body_mut().read_to_end(&mut body).await?;

    if response.status().is_client_error() || response.status().is_server_error() {
        let body_str = std::str::from_utf8(&body)?;
        let error: SupermavenApiError = serde_json::from_str(body_str)?;
        anyhow::bail!("Supermaven API error: {}", error.message);
    }

    serde_json::from_slice::<SupermavenDownloadResponse>(&body)
        .with_context(|| "Unable to parse Supermaven Agent response".to_string())
}

pub fn version_path(version: u64) -> PathBuf {
    supermaven_dir().join(format!(
        "sm-agent-{}{}",
        version,
        std::env::consts::EXE_SUFFIX
    ))
}

pub async fn has_version(version_path: &Path) -> bool {
    fs::metadata(version_path).await.is_ok_and(|m| m.is_file())
}

pub async fn get_supermaven_agent_path(client: Arc<dyn HttpClient>) -> Result<PathBuf> {
    fs::create_dir_all(supermaven_dir())
        .await
        .with_context(|| {
            format!(
                "Could not create Supermaven Agent Directory at {:?}",
                supermaven_dir()
            )
        })?;

    let platform = match std::env::consts::OS {
        "macos" => "darwin",
        "windows" => "windows",
        "linux" => "linux",
        unsupported => anyhow::bail!("unsupported platform {unsupported}"),
    };

    let arch = match std::env::consts::ARCH {
        "x86_64" => "amd64",
        "aarch64" => "arm64",
        unsupported => anyhow::bail!("unsupported architecture {unsupported}"),
    };

    let download_info = latest_release(client.clone(), platform, arch).await?;

    let binary_path = version_path(download_info.version);

    if has_version(&binary_path).await {
        // Due to an issue with the Supermaven binary not being made executable on
        // earlier Zed versions and Supermaven releases not occurring that frequently,
        // we ensure here that the found binary is actually executable.
        make_file_executable(&binary_path).await?;

        return Ok(binary_path);
    }

    let request = HttpRequest::get(&download_info.download_url);

    let mut response = client
        .send(request.body(AsyncBody::default())?)
        .await
        .with_context(|| "Unable to download Supermaven Agent".to_string())?;

    let mut file = File::create(&binary_path)
        .await
        .with_context(|| format!("Unable to create file at {:?}", binary_path))?;

    futures::io::copy(BufReader::new(response.body_mut()), &mut file)
        .await
        .with_context(|| format!("Unable to write binary to file at {:?}", binary_path))?;

    make_file_executable(&binary_path).await?;

    remove_matching(supermaven_dir(), |file| file != binary_path).await;

    Ok(binary_path)
}
