use anyhow::{anyhow, Context, Result};
use futures::AsyncReadExt;
use serde::{Deserialize, Serialize};
use smol::fs::unix::PermissionsExt as _;
use smol::fs::{self, File};
use std::path::PathBuf;
use std::sync::Arc;
use util::http::{AsyncBody, HttpClient, Request as HttpRequest};
use util::paths::SUPERMAVEN_DIR;

#[derive(Serialize)]
pub struct GetApiKeyRequest {
    pub user_id: String,
}

#[derive(Serialize)]
pub struct CreateApiKeyRequest {
    pub user_id: String,
    pub email: String,
}

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyResponse {
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

#[derive(Deserialize)]
pub struct SupermavenDownloadResponse {
    pub download_url: String,
    pub version: u64,
    pub sha256_hash: String,
}

#[derive(Deserialize)]
pub enum SupermavenUser {
    NotFound,
    Found {
        id: String,
        email: String,
        api_key: String,
    },
}

impl SupermavenAdminApi {
    pub fn new(admin_api_key: String, http_client: Arc<dyn HttpClient>) -> Self {
        Self {
            admin_api_key,
            api_url: "https://supermaven.com/api/".to_string(),
            http_client,
        }
    }

    pub async fn try_get_user(&self, request: GetApiKeyRequest) -> Result<SupermavenUser> {
        let uri = format!("{}external-user/{}", &self.api_url, &request.user_id);

        let request = HttpRequest::get(&uri).header("Authorization", self.admin_api_key.clone());

        let mut response = self
            .http_client
            .send(request.body(AsyncBody::default())?)
            .await
            .with_context(|| format!("Unable to get Supermaven API Key"))?;

        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        if response.status().is_client_error() {
            let error: SupermavenApiError = serde_json::from_slice(&body)?;
            if error.message == "User not found" {
                return Ok(SupermavenUser::NotFound);
            } else {
                return Err(anyhow!("Supermaven API error: {}", error.message));
            }
        } else if response.status().is_server_error() {
            let error: SupermavenApiError = serde_json::from_slice(&body)?;
            return Err(anyhow!("Supermaven API server error").context(error.message));
        }

        let body_str = std::str::from_utf8(&body)?;
        serde_json::from_str::<SupermavenUser>(body_str)
            .with_context(|| format!("Unable to parse Supermaven API Key response"))
    }

    pub async fn try_create_api_key(
        &self,
        request: CreateApiKeyRequest,
    ) -> Result<CreateApiKeyResponse> {
        let uri = format!("{}external-user", &self.api_url);

        let request = HttpRequest::post(&uri)
            .header("Authorization", self.admin_api_key.clone())
            .body(AsyncBody::from(serde_json::to_vec(&request)?))?;

        let mut response = self
            .http_client
            .send(request)
            .await
            .with_context(|| format!("Unable to create Supermaven API Key"))?;

        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        let body_str = std::str::from_utf8(&body)?;
        serde_json::from_str::<CreateApiKeyResponse>(body_str)
            .with_context(|| format!("Unable to parse Supermaven API Key response"))
    }

    pub async fn agent_binary_info(
        &self,
        platform: String,
        arch: String,
    ) -> Result<SupermavenDownloadResponse> {
        let uri = format!(
            "https://supermaven.com/api/download-path?platform={}&arch={}",
            platform, arch
        );

        // Download is not authenticated
        let request = HttpRequest::get(&uri);

        let mut response = self
            .http_client
            .send(request.body(AsyncBody::default())?)
            .await
            .with_context(|| format!("Unable to acquire Supermaven Agent"))?;

        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;

        let body_str = std::str::from_utf8(&body)?;
        serde_json::from_str::<SupermavenDownloadResponse>(body_str)
            .with_context(|| format!("Unable to parse Supermaven Agent response"))
    }

    pub async fn download_binary(&self, platform: String, arch: String) -> Result<PathBuf> {
        let download_info = self.agent_binary_info(platform, arch).await?;

        let request = HttpRequest::get(&download_info.download_url);

        let mut response = self
            .http_client
            .send(request.body(AsyncBody::default())?)
            .await
            .with_context(|| format!("Unable to download Supermaven Agent"))?;

        let version_dir = SUPERMAVEN_DIR.join(format!("sm-agent-{}", download_info.version));
        fs::create_dir_all(&version_dir)
            .await
            .with_context(|| format!("Could not create version directory at {:?}", version_dir))?;

        let binary_path = version_dir.join("sm-agent");

        let mut file = File::create(&binary_path)
            .await
            .with_context(|| format!("Unable to create file at {:?}", binary_path))?;

        smol::io::copy(response.body_mut(), &mut file)
            .await
            .with_context(|| format!("Unable to write binary to file at {:?}", binary_path))?;

        let mut permissions = file.metadata().await?.permissions();
        permissions.set_mode(0o755);
        file.set_permissions(permissions).await?;

        Ok(binary_path)
    }
}
