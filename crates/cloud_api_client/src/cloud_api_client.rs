mod llm_token;
mod websocket;

use std::sync::Arc;

use anyhow::{Context, Result, anyhow};
use cloud_api_types::websocket_protocol::{PROTOCOL_VERSION, PROTOCOL_VERSION_HEADER_NAME};
pub use cloud_api_types::*;
use futures::AsyncReadExt as _;
use gpui::{App, Task};
use gpui_tokio::Tokio;
use http_client::http::request;
use http_client::{
    AsyncBody, HttpClientWithUrl, HttpRequestExt, Json, Method, Request, Response, StatusCode,
};
use parking_lot::RwLock;
use serde::de::DeserializeOwned;
use thiserror::Error;
use yawc::WebSocket;

use crate::websocket::Connection;

pub use llm_token::LlmApiToken;

struct Credentials {
    user_id: u32,
    access_token: String,
}

#[derive(Debug, Error)]
pub enum ClientApiError {
    /// 401 — credentials are invalid or expired.
    #[error("Unauthorized")]
    Unauthorized,
    /// No credentials have been set on the client.
    #[error("not signed in")]
    NotSignedIn,
    /// Connection-level failure: DNS, TCP, TLS, timeout, etc.
    /// The HTTP request never received a response.
    #[error("connection to {host} failed")]
    ConnectionFailed {
        host: String,
        #[source]
        source: anyhow::Error,
    },
    /// Server returned a non-success HTTP status (other than 401).
    #[error("{host} returned {status}")]
    ServerError {
        host: String,
        status: StatusCode,
        body: String,
    },
    /// Failed to read or parse the response body after a successful HTTP status.
    #[error("invalid response")]
    InvalidResponse(#[source] anyhow::Error),
    /// Failed to build the HTTP request (URL construction, serialization, etc.).
    /// This typically indicates a programming error.
    #[error("failed to build request")]
    RequestBuildFailed(#[source] anyhow::Error),
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

    fn cloud_host(&self) -> String {
        self.http_client
            .build_zed_cloud_url("/")
            .ok()
            .and_then(|url| url.host_str().map(String::from))
            .unwrap_or_else(|| "cloud.zed.dev".into())
    }

    fn build_request(
        &self,
        req: request::Builder,
        body: impl Into<AsyncBody>,
    ) -> Result<Request<AsyncBody>, ClientApiError> {
        let credentials = self.credentials.read();
        let credentials = credentials.as_ref().ok_or(ClientApiError::NotSignedIn)?;
        build_request(req, body, credentials).map_err(ClientApiError::RequestBuildFailed)
    }

    pub async fn get_authenticated_user(
        &self,
        system_id: Option<String>,
    ) -> Result<GetAuthenticatedUserResponse, ClientApiError> {
        let request_builder = Request::builder()
            .method(Method::GET)
            .uri(
                self.http_client
                    .build_zed_cloud_url("/client/users/me")
                    .map_err(ClientApiError::RequestBuildFailed)?
                    .as_ref(),
            )
            .when_some(system_id, |builder, system_id| {
                builder.header(ZED_SYSTEM_ID_HEADER_NAME, system_id)
            });

        let request = self.build_request(request_builder, AsyncBody::default())?;
        self.send_authenticated_json_request(request).await
    }

    pub fn connect(&self, cx: &App) -> Result<Task<Result<Connection>>> {
        let mut connect_url = self
            .http_client
            .build_zed_cloud_url("/client/users/connect")?;
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

        Ok(Tokio::spawn_result(cx, async move {
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

    async fn create_llm_token(
        &self,
        system_id: Option<String>,
        organization_id: OrganizationId,
    ) -> Result<CreateLlmTokenResponse, ClientApiError> {
        let request_builder = Request::builder()
            .method(Method::POST)
            .uri(
                self.http_client
                    .build_zed_cloud_url("/client/llm_tokens")
                    .map_err(ClientApiError::RequestBuildFailed)?
                    .as_ref(),
            )
            .when_some(system_id, |builder, system_id| {
                builder.header(ZED_SYSTEM_ID_HEADER_NAME, system_id)
            });

        let request = self.build_request(
            request_builder,
            Json(CreateLlmTokenBody { organization_id }),
        )?;
        self.send_authenticated_json_request(request).await
    }

    pub async fn update_system_settings(
        &self,
        system_id: String,
        body: UpdateSystemSettingsBody,
    ) -> Result<SystemSettings, ClientApiError> {
        let request_builder = Request::builder()
            .method(Method::PATCH)
            .uri(
                self.http_client
                    .build_zed_cloud_url("/client/system_settings")
                    .map_err(ClientApiError::RequestBuildFailed)?
                    .as_ref(),
            )
            .header(ZED_SYSTEM_ID_HEADER_NAME, system_id);

        let request = self.build_request(request_builder, Json(body))?;
        self.send_authenticated_json_request(request).await
    }

    async fn send_authenticated_json_request<T: DeserializeOwned>(
        &self,
        request: Request<AsyncBody>,
    ) -> Result<T, ClientApiError> {
        let mut response = self.send_authenticated_request(request).await?;
        Self::read_response_json(&mut response).await
    }

    async fn send_authenticated_request(
        &self,
        request: Request<AsyncBody>,
    ) -> Result<Response<AsyncBody>, ClientApiError> {
        let host = self.cloud_host();
        let mut response = self.http_client.send(request).await.map_err(|source| {
            ClientApiError::ConnectionFailed {
                host: host.clone(),
                source,
            }
        })?;

        let status = response.status();
        if status.is_success() {
            return Ok(response);
        }

        if status == StatusCode::UNAUTHORIZED {
            return Err(ClientApiError::Unauthorized);
        }

        let body = match Self::read_response_body(&mut response).await {
            Ok(body) => body,
            Err(error) => format!("failed to read response body: {error}"),
        };
        Err(ClientApiError::ServerError { host, status, body })
    }

    async fn read_response_json<T: DeserializeOwned>(
        response: &mut Response<AsyncBody>,
    ) -> Result<T, ClientApiError> {
        let body = Self::read_response_body(response).await?;
        serde_json::from_str(&body).map_err(|error| ClientApiError::InvalidResponse(error.into()))
    }

    async fn read_response_body(
        response: &mut Response<AsyncBody>,
    ) -> Result<String, ClientApiError> {
        let mut body = String::new();
        response
            .body_mut()
            .read_to_string(&mut body)
            .await
            .map_err(|error| ClientApiError::InvalidResponse(error.into()))?;
        Ok(body)
    }

    pub async fn validate_credentials(&self, user_id: u32, access_token: &str) -> Result<bool> {
        let request = build_request(
            Request::builder().method(Method::GET).uri(
                self.http_client
                    .build_zed_cloud_url("/client/users/me")?
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

    pub async fn submit_agent_feedback(&self, body: SubmitAgentThreadFeedbackBody) -> Result<()> {
        let request = self.build_request(
            Request::builder().method(Method::POST).uri(
                self.http_client
                    .build_zed_cloud_url("/client/feedback/agent_thread")?
                    .as_ref(),
            ),
            AsyncBody::from(serde_json::to_string(&body)?),
        )?;

        self.send_authenticated_request(request).await?;
        Ok(())
    }

    pub async fn submit_agent_feedback_comments(
        &self,
        body: SubmitAgentThreadFeedbackCommentsBody,
    ) -> Result<()> {
        let request = self.build_request(
            Request::builder().method(Method::POST).uri(
                self.http_client
                    .build_zed_cloud_url("/client/feedback/agent_thread_comments")?
                    .as_ref(),
            ),
            AsyncBody::from(serde_json::to_string(&body)?),
        )?;

        self.send_authenticated_request(request).await?;
        Ok(())
    }

    pub async fn submit_edit_prediction_feedback(
        &self,
        body: SubmitEditPredictionFeedbackBody,
    ) -> Result<()> {
        let request = self.build_request(
            Request::builder().method(Method::POST).uri(
                self.http_client
                    .build_zed_cloud_url("/client/feedback/edit_prediction")?
                    .as_ref(),
            ),
            AsyncBody::from(serde_json::to_string(&body)?),
        )?;

        self.send_authenticated_request(request).await?;
        Ok(())
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
