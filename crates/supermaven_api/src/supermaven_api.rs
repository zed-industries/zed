use anyhow::{Context as _, Result, anyhow};
use futures::AsyncReadExt;
use futures::io::BufReader;
use http_client::{AsyncBody, HttpClient, Request as HttpRequest};
use paths::supermaven_dir;
use serde::{Deserialize, Serialize};
use smol::fs::{self, File};
use std::path::{Path, PathBuf};
use std::sync::Arc;

use util::fs::{make_file_executable, remove_matching};

#[derive(Serialize)]
pub struct GetExternalUserRequest {
    pub id: String,
}

#[derive(Serialize)]
pub struct CreateExternalUserRequest {
    pub id: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct DeleteExternalUserRequest {
    pub id: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateExternalUserResponse {
    pub api_key: String,
}

#[derive(Deserialize)]
pub struct SupermavenApiError {
    pub message: String,
}

pub struct SupermavenBinary {}

pub struct SupermavenAdminApi {
    admin_api_key: String,
    api_url: String,
    http_client: Arc<dyn HttpClient>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupermavenDownloadResponse {
    pub download_url: String,
    pub version: u64,
    pub sha256_hash: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct SupermavenUser {
    id: String,
    email: String,
    api_key: String,
}

impl SupermavenAdminApi {
    pub fn new(admin_api_key: String, http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            admin_api_key,
            api_url: "https://supermaven.com/api/".to_string(),
            http_client,
        }
    }

    pub async fn try_get_user(
        &self,
        request: GetExternalUserRequest,
    ) -> Result<Option<SupermavenUser>> {
        let uri = format!("{}external-user/{}", &self.api_url, &request.id);

        let request = HttpRequest::get(&uri).header("Authorization", self.admin_api_key.clone());

        let mut response = self
            .http_client
            .send(request.body(AsyncBody::default())?)
            .await
            .with_context(|| "Unable to get Supermaven API Key".to_string())?;

        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        if response.status().is_client_error() {
            let error: SupermavenApiError = serde_json::from_slice(&body)?;
            if error.message == "User not found" {
                return Ok(None);
            } else {
                anyhow::bail!("Supermaven API error: {}", error.message);
            }
        } else if response.status().is_server_error() {
            let error: SupermavenApiError = serde_json::from_slice(&body)?;
            return Err(anyhow!("Supermaven API server error").context(error.message));
        }

        let body_str = std::str::from_utf8(&body)?;

        Ok(Some(
            serde_json::from_str::<SupermavenUser>(body_str)
                .with_context(|| "Unable to parse Supermaven user response".to_string())?,
        ))
    }

    pub async fn try_create_user(
        &self,
        request: CreateExternalUserRequest,
    ) -> Result<CreateExternalUserResponse> {
        let uri = format!("{}external-user", &self.api_url);

        let request = HttpRequest::post(&uri)
            .header("Authorization", self.admin_api_key.clone())
            .body(AsyncBody::from(serde_json::to_vec(&request)?))?;

        let mut response = self
            .http_client
            .send(request)
            .await
            .with_context(|| "Unable to create Supermaven API Key".to_string())?;

        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        let body_str = std::str::from_utf8(&body)?;

        if !response.status().is_success() {
            let error: SupermavenApiError = serde_json::from_slice(&body)?;
            return Err(anyhow!("Supermaven API server error").context(error.message));
        }

        serde_json::from_str::<CreateExternalUserResponse>(body_str)
            .with_context(|| "Unable to parse Supermaven API Key response".to_string())
    }

    pub async fn try_delete_user(&self, request: DeleteExternalUserRequest) -> Result<()> {
        let uri = format!("{}external-user/{}", &self.api_url, &request.id);

        let request = HttpRequest::delete(&uri).header("Authorization", self.admin_api_key.clone());

        let mut response = self
            .http_client
            .send(request.body(AsyncBody::default())?)
            .await
            .with_context(|| "Unable to delete Supermaven User".to_string())?;

        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        if response.status().is_client_error() {
            let error: SupermavenApiError = serde_json::from_slice(&body)?;
            if error.message == "User not found" {
                return Ok(());
            } else {
                anyhow::bail!("Supermaven API error: {}", error.message);
            }
        } else if response.status().is_server_error() {
            let error: SupermavenApiError = serde_json::from_slice(&body)?;
            return Err(anyhow!("Supermaven API server error").context(error.message));
        }

        Ok(())
    }

    pub async fn try_get_or_create_user(
        &self,
        request: CreateExternalUserRequest,
    ) -> Result<CreateExternalUserResponse> {
        let get_user_request = GetExternalUserRequest {
            id: request.id.clone(),
        };

        match self.try_get_user(get_user_request).await? {
            None => self.try_create_user(request).await,
            Some(SupermavenUser { api_key, .. }) => Ok(CreateExternalUserResponse { api_key }),
        }
    }
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
