mod websocket;

use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
use cloud_api_types::websocket_protocol::{PROTOCOL_VERSION, PROTOCOL_VERSION_HEADER_NAME};
pub use cloud_api_types::*;
use futures::AsyncReadExt as _;
use gpui::{App, Task};
use gpui_tokio::Tokio;
use http_client::http::request;
use http_client::{AsyncBody, HttpClientWithUrl, Method, Request, StatusCode};
use parking_lot::RwLock;
use yawc::WebSocket;

use crate::websocket::Connection;

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

    fn build_request(
        &self,
        req: request::Builder,
        body: impl Into<AsyncBody>,
    ) -> Result<Request<AsyncBody>> {
        let credentials = self.credentials.read();
        let credentials = credentials.as_ref().context("no credentials provided")?;
        build_request(req, body, credentials)
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

    pub fn connect(&self, cx: &App) -> Result<Task<Result<Connection>>> {
        let mut connect_url = self
            .http_client
            .build_zed_cloud_url("/client/users/connect", &[])?;
        connect_url
            .set_scheme(match connect_url.scheme() {
                "https" => "wss",
                "http" => "ws",
                scheme => Err(anyhow!("invalid URL scheme: {scheme}"))?,
            })
            .map_err(|_| anyhow!("failed to set URL scheme"))?;

        let credentials = self.credentials.read();
        let credentials = credentials.as_ref().context("no credentials provided")?;
        let authorization_header = format!("{} {}", credentials.user_id, credentials.access_token);

        Ok(cx.spawn(async move |cx| {
            let handle = cx
                .update(|cx| Tokio::handle(cx))
                .ok()
                .context("failed to get Tokio handle")?;
            let _guard = handle.enter();

            let ws = WebSocket::connect(connect_url)
                .with_request(
                    request::Builder::new()
                        .header("Authorization", authorization_header)
                        .header(PROTOCOL_VERSION_HEADER_NAME, PROTOCOL_VERSION.to_string()),
                )
                .await?;

            Ok(Connection::new(ws))
        }))
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

    pub async fn validate_credentials(&self, user_id: u32, access_token: &str) -> Result<bool> {
        let request = build_request(
            Request::builder().method(Method::GET).uri(
                self.http_client
                    .build_zed_cloud_url("/client/users/me", &[])?
                    .as_ref(),
            ),
            AsyncBody::default(),
            &Credentials {
                user_id,
                access_token: access_token.into(),
            },
        )?;

        let mut response = self.http_client.send(request).await?;

        if response.status().is_success() {
            Ok(true)
        } else {
            let mut body = String::new();
            response.body_mut().read_to_string(&mut body).await?;
            if response.status() == StatusCode::UNAUTHORIZED {
                Ok(false)
            } else {
                Err(anyhow!(
                    "Failed to get authenticated user.\nStatus: {:?}\nBody: {body}",
                    response.status()
                ))
            }
        }
    }
}

fn build_request(
    req: request::Builder,
    body: impl Into<AsyncBody>,
    credentials: &Credentials,
) -> Result<Request<AsyncBody>> {
    Ok(req
        .header("Content-Type", "application/json")
        .header(
            "Authorization",
            format!("{} {}", credentials.user_id, credentials.access_token),
        )
        .body(body.into())?)
}
