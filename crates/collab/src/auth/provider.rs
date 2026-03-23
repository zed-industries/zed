use crate::{Error, db::UserId};
use anyhow::Context as _;
use async_trait::async_trait;
use axum::http::StatusCode;
use cloud_api_types::GetAuthenticatedUserResponse;

#[async_trait]
pub trait AuthProvider: Send + Sync {
    async fn authenticate(&self, header: &str) -> Result<UserId, Error>;
    fn provider_name(&self) -> &'static str;
}

// -------------------------------- ZedCloud -------------------------------- //

pub struct ZedCloudAuthProvider {
    http_client: Option<reqwest::Client>,
    zed_cloud_url: String,
}

impl ZedCloudAuthProvider {
    pub fn new(http_client: Option<reqwest::Client>, zed_cloud_url: String) -> Self {
        Self {
            http_client,
            zed_cloud_url,
        }
    }
}

#[async_trait]
impl AuthProvider for ZedCloudAuthProvider {
    fn provider_name(&self) -> &'static str {
        "zed_cloud"
    }

    async fn authenticate(&self, header: &str) -> Result<UserId, Error> {
        let mut parts = header.split_whitespace();

        let first = parts.next().unwrap_or("");
        if first == "dev-server-token" {
            Err(Error::http(
                StatusCode::UNAUTHORIZED,
                "Dev servers were removed in Zed 0.157 please upgrade to SSH remoting".to_string(),
            ))?;
        }

        let user_id = UserId(first.parse().map_err(|_| {
            Error::http(
                StatusCode::BAD_REQUEST,
                "missing user id in authorization header".to_string(),
            )
        })?);

        let access_token = parts.next().ok_or_else(|| {
            Error::http(
                StatusCode::BAD_REQUEST,
                "missing access token in authorization header".to_string(),
            )
        })?;

        let http_client = self.http_client.clone().expect("no HTTP client");

        let response = http_client
            .get(format!("{}/client/users/me", self.zed_cloud_url))
            .header("Content-Type", "application/json")
            .header("Authorization", format!("{user_id} {access_token}"))
            .send()
            .await
            .context("failed to validate access token")?;

        if let Ok(response) = response.error_for_status() {
            let response_body: GetAuthenticatedUserResponse = response
                .json()
                .await
                .context("failed to parse response body")?;

            return Ok(UserId(response_body.user.id));
        }

        Err(Error::http(
            StatusCode::UNAUTHORIZED,
            "invalid credentials".to_string(),
        ))
    }
}
