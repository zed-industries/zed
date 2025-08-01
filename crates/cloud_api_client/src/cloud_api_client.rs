use std::sync::Arc;

use anyhow::{Result, anyhow};
pub use cloud_api_types::*;
use futures::AsyncReadExt as _;
use http_client::http::request;
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

    pub fn has_credentials(&self) -> bool {
        self.credentials.read().is_some()
    }

    pub fn set_credentials(&self, user_id: u32, access_token: String) {
        *self.credentials.write() = Some(Credentials {
            user_id,
            access_token,
        });
    }

    pub fn clear_credentials(&self) {
        *self.credentials.write() = None;
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

    fn build_request(
        &self,
        req: request::Builder,
        body: impl Into<AsyncBody>,
    ) -> Result<Request<AsyncBody>> {
        Ok(req
            .header("Content-Type", "application/json")
            .header("Authorization", self.authorization_header()?)
            .body(body.into())?)
    }

    pub async fn get_authenticated_user(&self) -> Result<GetAuthenticatedUserResponse> {
        let request = self.build_request(
            Request::builder().method(Method::GET).uri(
                self.http_client
                    .build_zed_cloud_url("/client/users/me", &[])?
                    .as_ref(),
            ),
            AsyncBody::default(),
        )?;

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

        Ok(serde_json::from_str(&body)?)
    }

    pub async fn accept_terms_of_service(&self) -> Result<AcceptTermsOfServiceResponse> {
        let request = self.build_request(
            Request::builder().method(Method::POST).uri(
                self.http_client
                    .build_zed_cloud_url("/client/terms_of_service/accept", &[])?
                    .as_ref(),
            ),
            AsyncBody::default(),
        )?;

        let mut response = self.http_client.send(request).await?;

        if !response.status().is_success() {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;

            anyhow::bail!(
                "Failed to accept terms of service.\nStatus: {:?}\nBody: {body}",
                response.status()
            )
        }

        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        Ok(serde_json::from_str(&body)?)
    }

    pub async fn create_llm_token(
        &self,
        system_id: Option<String>,
    ) -> Result<CreateLlmTokenResponse> {
        let mut request_builder = Request::builder().method(Method::POST).uri(
            self.http_client
                .build_zed_cloud_url("/client/llm_tokens", &[])?
                .as_ref(),
        );

        if let Some(system_id) = system_id {
            request_builder = request_builder.header(ZED_SYSTEM_ID_HEADER_NAME, system_id);
        }

        let request = self.build_request(request_builder, AsyncBody::default())?;

        let mut response = self.http_client.send(request).await?;

        if !response.status().is_success() {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;

            anyhow::bail!(
                "Failed to create LLM token.\nStatus: {:?}\nBody: {body}",
                response.status()
            )
        }

        let mut body = String::new();
        response.body_mut().read_to_string(&mut body).await?;

        Ok(serde_json::from_str(&body)?)
    }
}
