pub mod provider;

use crate::{AppState, Error, rpc::Principal};

use axum::{
    http::{self, Request, StatusCode},
    middleware::Next,
    response::IntoResponse,
};
pub use rpc::auth::random_token;
use std::sync::Arc;

/// Validates the authorization header using the configured `AuthProvider` and adds
/// an `Extension<Principal>` to the request.
///
/// The actual authentication logic is delegated to `AppState::auth_provider`, allowing
/// for different authentication mechanisms to be plugged in.
pub async fn validate_header<B>(mut req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let auth_header = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| {
            Error::http(
                StatusCode::UNAUTHORIZED,
                "missing authorization header".to_string(),
            )
        })?;

    let state = req.extensions().get::<Arc<AppState>>().unwrap();

    let user_id = state.auth_provider.authenticate(auth_header).await?;

    let user = state.db.get_user_by_id(user_id).await?.ok_or_else(|| {
        Error::http(
            StatusCode::UNAUTHORIZED,
            format!("user {user_id} not found"),
        )
    })?;

    req.extensions_mut().insert(Principal::User(user));

    return Ok::<_, Error>(next.run(req).await);
}
