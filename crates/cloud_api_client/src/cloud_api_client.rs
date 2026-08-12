mod llm_token;
mod websocket;

use std::sync::Arc;

use anyhow::{Result, anyhow};
pub use cloud_api_types::*;
use futures::AsyncReadExt as _;
use http_client::http::request;
use http_client::{
    AsyncBody, HttpClientWithUrl, HttpRequestExt, Json, Method, Request, Response, StatusCode,
};
use parking_lot::RwLock;
use serde::de::DeserializeOwned;
use thiserror::Error;

pub use llm_token::LlmApiToken;

struct Credentials {
    user_id: u32,
    access_token: String,
}

#[derive(Clone, Copy)]
enum Authentication {
    Credentials,
    Session,
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
    authentication: Authentication,
}

#[derive(Debug)]
pub enum UpdateSyncedSettingsResult {
    Written(UpdateSyncedSettingsResponse),
    Conflict(GetSyncedSettingsResponse),
}

impl CloudApiClient {
    pub fn new(http_client: Arc<HttpClientWithUrl>) -> Self {
        Self {
            credentials: RwLock::new(None),
            http_client,
            authentication: Authentication::Credentials,
        }
    }

    pub fn new_with_session_authentication(http_client: Arc<HttpClientWithUrl>) -> Self {
        Self {
            credentials: RwLock::new(None),
            http_client,
            authentication: Authentication::Session,
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

    pub fn cloud_host(&self) -> String {
        self.http_client
            .build_zed_cloud_url("/")
            .ok()
            .and_then(|url| url.host_str().map(String::from))
            .unwrap_or_else(|| "cloud.zed.dev".into())
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

        self.send_authenticated_json_request(request_builder, AsyncBody::default())
            .await
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

        self.send_authenticated_json_request(
            request_builder,
            Json(CreateLlmTokenBody { organization_id }),
        )
        .await
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

        self.send_authenticated_json_request(request_builder, Json(body))
            .await
    }

    pub async fn send_authenticated_json_request<T: DeserializeOwned>(
        &self,
        request_builder: request::Builder,
        body: impl Into<AsyncBody>,
    ) -> Result<T, ClientApiError> {
        let mut response = self
            .send_authenticated_request(request_builder, body)
            .await?;
        Self::read_response_json(&mut response).await
    }

    async fn send_authenticated_request(
        &self,
        request_builder: request::Builder,
        body: impl Into<AsyncBody>,
    ) -> Result<Response<AsyncBody>, ClientApiError> {
        let request = if matches!(self.authentication, Authentication::Session) {
            build_request(request_builder, body, None)
                .map_err(ClientApiError::RequestBuildFailed)?
        } else {
            let credentials = self.credentials.read();
            let credentials = credentials.as_ref().ok_or(ClientApiError::NotSignedIn)?;
            build_request(request_builder, body, Some(credentials))
                .map_err(ClientApiError::RequestBuildFailed)?
        };

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
            Some(&Credentials {
                user_id,
                access_token: access_token.into(),
            }),
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

    // TODO kb cloud: the `/client/synced_settings` endpoints are not
    // implemented server-side yet.
    pub async fn get_synced_settings(
        &self,
        system_id: Option<String>,
        kind: &str,
    ) -> Result<GetSyncedSettingsResponse, ClientApiError> {
        let request_builder = Request::builder()
            .method(Method::GET)
            .uri(
                self.http_client
                    .build_zed_cloud_url_with_query("/client/synced_settings", [("kind", kind)])
                    .map_err(ClientApiError::RequestBuildFailed)?
                    .as_ref(),
            )
            .when_some(system_id, |builder, system_id| {
                builder.header(ZED_SYSTEM_ID_HEADER_NAME, system_id)
            });

        self.send_authenticated_json_request(request_builder, AsyncBody::default())
            .await
    }

    pub async fn update_synced_settings(
        &self,
        system_id: Option<String>,
        body: UpdateSyncedSettingsBody,
    ) -> Result<UpdateSyncedSettingsResult, ClientApiError> {
        let request_builder = Request::builder()
            .method(Method::PUT)
            .uri(
                self.http_client
                    .build_zed_cloud_url("/client/synced_settings")
                    .map_err(ClientApiError::RequestBuildFailed)?
                    .as_ref(),
            )
            .when_some(system_id, |builder, system_id| {
                builder.header(ZED_SYSTEM_ID_HEADER_NAME, system_id)
            });

        match self
            .send_authenticated_json_request::<UpdateSyncedSettingsResponse>(
                request_builder,
                Json(body),
            )
            .await
        {
            Ok(response) => Ok(UpdateSyncedSettingsResult::Written(response)),
            Err(ClientApiError::ServerError { status, body, .. })
                if status == StatusCode::CONFLICT =>
            {
                let current = serde_json::from_str::<GetSyncedSettingsResponse>(&body)
                    .map_err(|error| ClientApiError::InvalidResponse(error.into()))?;
                Ok(UpdateSyncedSettingsResult::Conflict(current))
            }
            Err(error) => Err(error),
        }
    }

    pub async fn get_synced_settings_hosts(
        &self,
        system_id: Option<String>,
    ) -> Result<GetSyncedSettingsHostsResponse, ClientApiError> {
        let request_builder = Request::builder()
            .method(Method::GET)
            .uri(
                self.http_client
                    .build_zed_cloud_url("/client/synced_settings/hosts")
                    .map_err(ClientApiError::RequestBuildFailed)?
                    .as_ref(),
            )
            .when_some(system_id, |builder, system_id| {
                builder.header(ZED_SYSTEM_ID_HEADER_NAME, system_id)
            });

        self.send_authenticated_json_request(request_builder, AsyncBody::default())
            .await
    }

    pub async fn update_synced_settings_host(
        &self,
        system_id: Option<String>,
        host_system_id: &str,
        body: UpdateSyncedSettingsHostBody,
    ) -> Result<SyncedSettingsHost, ClientApiError> {
        let request_builder = Request::builder()
            .method(Method::PATCH)
            .uri(
                self.http_client
                    .build_zed_cloud_url(&format!("/client/synced_settings/hosts/{host_system_id}"))
                    .map_err(ClientApiError::RequestBuildFailed)?
                    .as_ref(),
            )
            .when_some(system_id, |builder, system_id| {
                builder.header(ZED_SYSTEM_ID_HEADER_NAME, system_id)
            });

        self.send_authenticated_json_request(request_builder, Json(body))
            .await
    }

    pub async fn delete_synced_settings_host(
        &self,
        system_id: Option<String>,
        host_system_id: &str,
    ) -> Result<(), ClientApiError> {
        let request_builder = Request::builder()
            .method(Method::DELETE)
            .uri(
                self.http_client
                    .build_zed_cloud_url(&format!("/client/synced_settings/hosts/{host_system_id}"))
                    .map_err(ClientApiError::RequestBuildFailed)?
                    .as_ref(),
            )
            .when_some(system_id, |builder, system_id| {
                builder.header(ZED_SYSTEM_ID_HEADER_NAME, system_id)
            });

        self.send_authenticated_request(request_builder, AsyncBody::default())
            .await?;
        Ok(())
    }

    pub async fn get_synced_settings_history(
        &self,
        system_id: Option<String>,
        kind: &str,
    ) -> Result<GetSyncedSettingsHistoryResponse, ClientApiError> {
        let request_builder = Request::builder()
            .method(Method::GET)
            .uri(
                self.http_client
                    .build_zed_cloud_url_with_query(
                        "/client/synced_settings/history",
                        [("kind", kind)],
                    )
                    .map_err(ClientApiError::RequestBuildFailed)?
                    .as_ref(),
            )
            .when_some(system_id, |builder, system_id| {
                builder.header(ZED_SYSTEM_ID_HEADER_NAME, system_id)
            });

        self.send_authenticated_json_request(request_builder, AsyncBody::default())
            .await
    }

    // TODO kb cloud: erase is resurrection-prone — any still-enabled device in
    // the group re-pushes its local settings on its next change; needs a server
    // tombstone, or the docs must state that erase implies disabling the
    // group's hosts first.
    pub async fn delete_synced_settings(
        &self,
        system_id: Option<String>,
        all_groups: bool,
    ) -> Result<(), ClientApiError> {
        let url = if all_groups {
            self.http_client
                .build_zed_cloud_url_with_query("/client/synced_settings", [("all", "true")])
        } else {
            self.http_client
                .build_zed_cloud_url("/client/synced_settings")
        }
        .map_err(ClientApiError::RequestBuildFailed)?;
        let request_builder = Request::builder()
            .method(Method::DELETE)
            .uri(url.as_ref())
            .when_some(system_id, |builder, system_id| {
                builder.header(ZED_SYSTEM_ID_HEADER_NAME, system_id)
            });

        self.send_authenticated_request(request_builder, AsyncBody::default())
            .await?;
        Ok(())
    }

    pub async fn submit_agent_feedback(&self, body: SubmitAgentThreadFeedbackBody) -> Result<()> {
        let request = Request::builder().method(Method::POST).uri(
            self.http_client
                .build_zed_cloud_url("/client/feedback/agent_thread")?
                .as_ref(),
        );

        self.send_authenticated_request(request, AsyncBody::from(serde_json::to_string(&body)?))
            .await?;
        Ok(())
    }

    pub async fn submit_agent_feedback_comments(
        &self,
        body: SubmitAgentThreadFeedbackCommentsBody,
    ) -> Result<()> {
        let request = Request::builder().method(Method::POST).uri(
            self.http_client
                .build_zed_cloud_url("/client/feedback/agent_thread_comments")?
                .as_ref(),
        );

        self.send_authenticated_request(request, AsyncBody::from(serde_json::to_string(&body)?))
            .await?;
        Ok(())
    }

    pub async fn submit_edit_prediction_feedback(
        &self,
        body: SubmitEditPredictionFeedbackBody,
    ) -> Result<()> {
        let request = Request::builder().method(Method::POST).uri(
            self.http_client
                .build_zed_cloud_url("/client/feedback/edit_prediction")?
                .as_ref(),
        );

        self.send_authenticated_request(request, AsyncBody::from(serde_json::to_string(&body)?))
            .await?;
        Ok(())
    }
}

fn build_request(
    req: request::Builder,
    body: impl Into<AsyncBody>,
    credentials: Option<&Credentials>,
) -> Result<Request<AsyncBody>> {
    Ok(req
        .header("Content-Type", "application/json")
        .when_some(credentials, |request, credentials| {
            request.header(
                "Authorization",
                format!("{} {}", credentials.user_id, credentials.access_token),
            )
        })
        .body(body.into())?)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn build_session_authenticated_request_without_authorization_header() -> Result<()> {
        let request = build_request(
            Request::builder().uri("https://cloud.zed.dev/client/users/me"),
            AsyncBody::default(),
            None,
        )?;

        assert_eq!(
            request
                .headers()
                .get("Content-Type")
                .and_then(|value| value.to_str().ok()),
            Some("application/json")
        );
        assert!(!request.headers().contains_key("Authorization"));
        Ok(())
    }

    #[test]
    fn build_credentials_authenticated_request_with_authorization_header() -> Result<()> {
        let request = build_request(
            Request::builder().uri("https://cloud.zed.dev/client/users/me"),
            AsyncBody::default(),
            Some(&Credentials {
                user_id: 123,
                access_token: "token".into(),
            }),
        )?;

        assert_eq!(
            request
                .headers()
                .get("Authorization")
                .and_then(|value| value.to_str().ok()),
            Some("123 token")
        );
        Ok(())
    }
}
