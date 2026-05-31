use crate::entities::User;
use crate::{AppState, Error, db::UserId, rpc::Principal};
use anyhow::Context as _;
use axum::{
    http::{self, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
use cloud_api_types::GetAuthenticatedUserResponse;
pub use rpc::auth::random_token;
use std::sync::Arc;

/// Validates the authorization header and adds an Extension<Principal> to the request.
/// Authorization: <user-id> <token>
///   <token> is the access_token attached to that user.
/// Authorization: "dev-server-token" <token>
pub async fn validate_header<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let mut auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| {
            Error::http(
                StatusCode::UNAUTHORIZED,
                "missing authorization header".to_string(),
            )
        })?
        .split_whitespace();

    let state = req.extensions().get::<Arc<AppState>>().unwrap();

    let first = auth_header.next().unwrap_or("");
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

    let access_token = auth_header.next().ok_or_else(|| {
        Error::http(
            StatusCode::BAD_REQUEST,
            "missing access token in authorization header".to_string(),
        )
    })?;

    let http_client = state.http_client.clone().expect("no HTTP client");

    let response = http_client
        .get(format!("{}/client/users/me", state.config.zed_cloud_url()))
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

        let user = User {
            id: UserId(response_body.user.id),
            github_login: response_body.user.github_login,
            avatar_url: response_body.user.avatar_url,
            name: response_body.user.name,
            admin: response_body.user.is_staff,
            connected_once: response_body.user.has_connected_to_collab_once,
        };

        req.extensions_mut().insert(Principal::User(user));
        return Ok::<_, Error>(next.run(req).await);
    }

    Err(Error::http(
        StatusCode::UNAUTHORIZED,
        "invalid credentials".to_string(),
    ))
}
