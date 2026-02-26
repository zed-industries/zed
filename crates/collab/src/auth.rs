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
///   <token> can be an access_token attached to that user, or an access token of an admin
///   or (in development) the string ADMIN:<config.api_token>.
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

        let user_id = UserId(response_body.user.id);

        let user = state
            .db
            .get_user_by_id(user_id)
            .await?
            .with_context(|| format!("user {user_id} not found"))?;

        req.extensions_mut().insert(Principal::User(user));
        return Ok::<_, Error>(next.run(req).await);
    }

    Err(Error::http(
        StatusCode::UNAUTHORIZED,
        "invalid credentials".to_string(),
    ))
}
