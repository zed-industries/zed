use std::sync::Arc;

use anyhow::{Result, anyhow};
pub use cloud_api_types::*;
use futures::AsyncReadExt as _;
use http_client::{AsyncBody, HttpClientWithUrl, Method, Request};
use parking_lot::RwLock;

struct Credentials {
    user_id: u32,
    access_token: String,
}

pub struct CloudApiClient {
    credentials: RwLock<Option<Credentials>>,
    http_client: Arc<HttpClientWithUrl>,
}

impl CloudApiClient {
    pub fn new(http_client: Arc<HttpClientWithUrl>) -> Self {
        Self {
            credentials: RwLock::new(None),
            http_client,
        }
    }

    pub fn set_credentials(&self, user_id: u32, access_token: String) {
        *self.credentials.write() = Some(Credentials {
            user_id,
            access_token,
        });
    }

    fn authorization_header(&self) -> Result<String> {
        let guard = self.credentials.read();
        let credentials = guard
            .as_ref()
            .ok_or_else(|| anyhow!("No credentials provided"))?;

        Ok(format!(
            "{} {}",
            credentials.user_id, credentials.access_token
        ))
    }

    pub async fn get_authenticated_user(&self) -> Result<AuthenticatedUser> {
        let request = Request::builder()
            .method(Method::GET)
            .uri(
                self.http_client
                    .build_zed_cloud_url("/client/users/me", &[])?
                    .as_ref(),
            )
            .header("Content-Type", "application/json")
            .header("Authorization", self.authorization_header()?)
            .body(AsyncBody::default())?;

        let mut response = self.http_client.send(request).await?;

        if !response.status().is_success() {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;

            anyhow::bail!(
                "Failed to get authenticated user.\nStatus: {:?}\nBody: {body}",
                response.status()
            )
        }

        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;
        let response: GetAuthenticatedUserResponse = serde_json::from_str(&body)?;

        Ok(response.user)
    }
}
