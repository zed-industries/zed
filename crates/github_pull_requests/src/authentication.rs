use crate::{GITHUB_API_VERSION, GITHUB_APP_CLIENT_ID};
use anyhow::{Context as _, Result, anyhow, bail};
use credentials_provider::CredentialsProvider;
use futures::AsyncReadExt as _;
use gpui::AsyncApp;
use http_client::{AsyncBody, HttpClient, Method, Request};
use serde::{Deserialize, Serialize};
use std::{sync::Arc, time::Duration};
use url::form_urlencoded;

const DEVICE_CODE_URL: &str = "https://github.com/login/device/code";
const ACCESS_TOKEN_URL: &str = "https://github.com/login/oauth/access_token";
const CREDENTIALS_KEY: &str = "https://github.com/zed-pull-requests";

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct GitHubCredentials {
    pub access_token: String,
    pub refresh_token: Option<String>,
    pub expires_at_unix_seconds: Option<u64>,
    pub refresh_token_expires_at_unix_seconds: Option<u64>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct DeviceAuthorization {
    pub device_code: String,
    pub user_code: String,
    pub verification_uri: String,
    pub expires_in: Duration,
    pub interval: Duration,
}

#[derive(Debug)]
pub enum DeviceFlowPoll {
    Pending,
    SlowDown(Duration),
    Complete(GitHubCredentials),
    AccessDenied,
    Expired,
}

pub struct GitHubAuthentication {
    http_client: Arc<dyn HttpClient>,
    credentials_provider: Arc<dyn CredentialsProvider>,
    client_id: Arc<str>,
}

impl GitHubAuthentication {
    pub fn new(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
    ) -> Result<Self> {
        let client_id = GITHUB_APP_CLIENT_ID
            .filter(|client_id| !client_id.is_empty())
            .ok_or_else(|| {
                anyhow!(
                    "GitHub pull request sign-in is not configured; set ZED_GITHUB_APP_CLIENT_ID when building Zed"
                )
            })?;
        Ok(Self::with_client_id(
            http_client,
            credentials_provider,
            client_id,
        ))
    }

    pub fn with_client_id(
        http_client: Arc<dyn HttpClient>,
        credentials_provider: Arc<dyn CredentialsProvider>,
        client_id: impl Into<Arc<str>>,
    ) -> Self {
        Self {
            http_client,
            credentials_provider,
            client_id: client_id.into(),
        }
    }

    pub async fn request_device_authorization(&self) -> Result<DeviceAuthorization> {
        let body = form_urlencoded::Serializer::new(String::new())
            .append_pair("client_id", &self.client_id)
            .finish();
        let response: DeviceCodeResponse = self.post_form(DEVICE_CODE_URL, body).await?;
        Ok(DeviceAuthorization {
            device_code: response.device_code,
            user_code: response.user_code,
            verification_uri: response.verification_uri,
            expires_in: Duration::from_secs(response.expires_in),
            interval: Duration::from_secs(response.interval),
        })
    }

    pub async fn poll_device_authorization(&self, device_code: &str) -> Result<DeviceFlowPoll> {
        let body = form_urlencoded::Serializer::new(String::new())
            .append_pair("client_id", &self.client_id)
            .append_pair("device_code", device_code)
            .append_pair("grant_type", "urn:ietf:params:oauth:grant-type:device_code")
            .finish();
        let response: TokenResponse = self.post_form(ACCESS_TOKEN_URL, body).await?;
        match response.error.as_deref() {
            Some("authorization_pending") => Ok(DeviceFlowPoll::Pending),
            Some("slow_down") => Ok(DeviceFlowPoll::SlowDown(Duration::from_secs(5))),
            Some("access_denied") => Ok(DeviceFlowPoll::AccessDenied),
            Some("expired_token") => Ok(DeviceFlowPoll::Expired),
            Some(error) => bail!("GitHub device authorization failed: {error}"),
            None => Ok(DeviceFlowPoll::Complete(response.into_credentials()?)),
        }
    }

    pub async fn refresh(&self, refresh_token: &str) -> Result<GitHubCredentials> {
        let body = form_urlencoded::Serializer::new(String::new())
            .append_pair("client_id", &self.client_id)
            .append_pair("grant_type", "refresh_token")
            .append_pair("refresh_token", refresh_token)
            .finish();
        let response: TokenResponse = self.post_form(ACCESS_TOKEN_URL, body).await?;
        if let Some(error) = response.error {
            bail!("GitHub token refresh failed: {error}");
        }
        response.into_credentials()
    }

    pub async fn load(&self, cx: &AsyncApp) -> Result<Option<GitHubCredentials>> {
        let Some((_, bytes)) = self
            .credentials_provider
            .read_credentials(CREDENTIALS_KEY, cx)
            .await?
        else {
            return Ok(None);
        };
        serde_json::from_slice(&bytes).context("failed to read stored GitHub credentials")
    }

    pub async fn load_valid(&self, cx: &AsyncApp) -> Result<Option<GitHubCredentials>> {
        let Some(credentials) = self.load(cx).await? else {
            return Ok(None);
        };
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        let needs_refresh = credentials
            .expires_at_unix_seconds
            .is_some_and(|expires_at| expires_at <= now.saturating_add(60));
        if !needs_refresh {
            return Ok(Some(credentials));
        }
        let refresh_token = credentials
            .refresh_token
            .as_deref()
            .context("GitHub credentials expired and cannot be refreshed; sign in again")?;
        if credentials
            .refresh_token_expires_at_unix_seconds
            .is_some_and(|expires_at| expires_at <= now)
        {
            bail!("GitHub credentials expired; sign in again");
        }
        let refreshed = self.refresh(refresh_token).await?;
        self.store(&refreshed, cx).await?;
        Ok(Some(refreshed))
    }

    pub async fn store(&self, credentials: &GitHubCredentials, cx: &AsyncApp) -> Result<()> {
        let bytes = serde_json::to_vec(credentials)?;
        self.credentials_provider
            .write_credentials(CREDENTIALS_KEY, "Bearer", &bytes, cx)
            .await
    }

    pub async fn clear(&self, cx: &AsyncApp) -> Result<()> {
        self.credentials_provider
            .delete_credentials(CREDENTIALS_KEY, cx)
            .await
    }

    async fn post_form<T: for<'de> Deserialize<'de>>(&self, url: &str, body: String) -> Result<T> {
        let request = Request::builder()
            .method(Method::POST)
            .uri(url)
            .header("Accept", "application/json")
            .header("Content-Type", "application/x-www-form-urlencoded")
            .header("X-GitHub-Api-Version", GITHUB_API_VERSION)
            .body(AsyncBody::from(body))?;
        let mut response = self.http_client.send(request).await?;
        let status = response.status();
        let mut body = Vec::new();
        response.body_mut().read_to_end(&mut body).await?;
        if !status.is_success() {
            bail!("GitHub authentication request failed with HTTP {status}");
        }
        serde_json::from_slice(&body).context("failed to parse GitHub authentication response")
    }
}

#[derive(Deserialize)]
struct DeviceCodeResponse {
    device_code: String,
    user_code: String,
    verification_uri: String,
    expires_in: u64,
    interval: u64,
}

#[derive(Deserialize)]
struct TokenResponse {
    access_token: Option<String>,
    refresh_token: Option<String>,
    expires_in: Option<u64>,
    refresh_token_expires_in: Option<u64>,
    error: Option<String>,
}

impl TokenResponse {
    fn into_credentials(self) -> Result<GitHubCredentials> {
        let access_token = self
            .access_token
            .context("GitHub token response did not include an access token")?;
        let now = std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)?
            .as_secs();
        Ok(GitHubCredentials {
            access_token,
            refresh_token: self.refresh_token,
            expires_at_unix_seconds: self.expires_in.map(|seconds| now + seconds),
            refresh_token_expires_at_unix_seconds: self
                .refresh_token_expires_in
                .map(|seconds| now + seconds),
        })
    }
}
