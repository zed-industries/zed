use anyhow::{anyhow, Context, Result};
use futures::{io::BufReader, stream::BoxStream, AsyncBufReadExt, AsyncReadExt, StreamExt};
use serde::{Deserialize, Serialize};
use serde_json::{Map, Value};
use std::sync::Arc;
use std::{convert::TryFrom, future::Future};
use util::http::HttpClient;
use util::http::{AsyncBody, HttpClient, Method, Request as HttpRequest};

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
enum SupermavenUser {
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
            // todo!(): Double check that the response body is "User not found".
            return Ok(SupermavenUser::NotFound);
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
}

pub fn download_binary(
    http_client: Arc<dyn HttpClient>,
    platform: String,
    arch: String,
) -> impl Future<Output = Result<BoxStream<'static, Result<Vec<u8>>>>> {
    let uri = format!(
        "https://supermaven.com/api/download-path?platform={}&arch={}",
        platform, arch
    );

    let mut response = http
        .get(url, Default::default(), true)
        .await
        .context("error downloading copilot release")?;
    let decompressed_bytes = GzipDecoder::new(BufReader::new(response.body_mut()));
    let archive = Archive::new(decompressed_bytes);
    archive.unpack(dist_dir).await?;
}
