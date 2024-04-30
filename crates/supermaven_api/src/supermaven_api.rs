use anyhow::{anyhow, Context, Result};
use futures::{AsyncReadExt, Future};
use serde::{Deserialize, Serialize};
use smol::fs::{self, File};
use std::path::PathBuf;
use std::sync::Arc;
use util::http::{AsyncBody, HttpClient, Request as HttpRequest};
use util::paths::SUPERMAVEN_DIR;

#[derive(Serialize)]
pub struct GetExternalUserRequest {
    pub user_id: String,
}

#[derive(Serialize)]
pub struct CreateExternalUserRequest {
    pub user_id: String,
    pub email: String,
}

#[derive(Serialize)]
pub struct DeleteExternalUserRequest {
    pub user_id: String,
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

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
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

    pub async fn try_get_user(&self, request: GetExternalUserRequest) -> Result<SupermavenUser> {
        let uri = format!("{}external-user/{}", &self.api_url, &request.user_id);

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
            .with_context(|| "Unable to parse Supermaven API Key response".to_string())
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
        serde_json::from_str::<CreateExternalUserResponse>(body_str)
            .with_context(|| "Unable to parse Supermaven API Key response".to_string())
    }

    pub async fn try_delete_user(&self, request: DeleteExternalUserRequest) -> Result<()> {
        let uri = format!("{}external-user/{}", &self.api_url, &request.user_id);

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
                return Err(anyhow!("Supermaven API error: {}", error.message));
            }
        } else if response.status().is_server_error() {
            let error: SupermavenApiError = serde_json::from_slice(&body)?;
            return Err(anyhow!("Supermaven API server error").context(error.message));
        }

        Ok(())
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
        return Err(anyhow!("Supermaven API error: {}", error.message));
    }

    serde_json::from_slice::<SupermavenDownloadResponse>(&body)
        .with_context(|| "Unable to parse Supermaven Agent response".to_string())
}

pub fn download_latest(client: Arc<dyn HttpClient>) -> impl Future<Output = Result<PathBuf>> {
    async move {
        let platform = match std::env::consts::OS {
            "macos" => "darwin",
            "windows" => "windows",
            "linux" => "linux",
            _ => return Err(anyhow!("unsupported platform")),
        };

        let arch = match std::env::consts::ARCH {
            "x86_64" => "amd64",
            "aarch64" => "arm64",
            _ => return Err(anyhow!("unsupported architecture")),
        };

        let download_info = latest_release(client.clone(), platform, arch).await?;
        let request = HttpRequest::get(&download_info.download_url);

        let mut response = client
            .send(request.body(AsyncBody::default())?)
            .await
            .with_context(|| "Unable to download Supermaven Agent".to_string())?;

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

        #[cfg(not(windows))]
        {
            let mut permissions = file.metadata().await?.permissions();
            permissions.set_mode(0o755);
            file.set_permissions(permissions).await?;
        }
        #[cfg(windows)]
        {
            use std::fs::OpenOptions;
            use std::os::windows::fs::OpenOptionsExt as _;

            let mut options = OpenOptions::new();
            options.write(true);
            options.read(true);
            options.custom_flags(libc::O_BINARY);
            let file = options.open(&binary_path)?;
        }

        Ok(binary_path)
    }
}
