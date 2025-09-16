pub mod contributors;
pub mod events;
pub mod extensions;
pub mod ips_file;
pub mod slack;

use crate::{AppState, Error, Result, auth, db::UserId, rpc};
use anyhow::Context as _;
use axum::{
    Extension, Json, Router,
    body::Body,
    extract::{Path, Query},
    headers::Header,
    http::{self, HeaderName, Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
};
use axum_extra::response::ErasedJson;
use serde::{Deserialize, Serialize};
use std::sync::{Arc, OnceLock};
use tower::ServiceBuilder;

pub use extensions::fetch_extensions_from_blob_store_periodically;

pub struct CloudflareIpCountryHeader(String);

impl Header for CloudflareIpCountryHeader {
    fn name() -> &'static HeaderName {
        static CLOUDFLARE_IP_COUNTRY_HEADER: OnceLock<HeaderName> = OnceLock::new();
        CLOUDFLARE_IP_COUNTRY_HEADER.get_or_init(|| HeaderName::from_static("cf-ipcountry"))
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let country_code = values
            .next()
            .ok_or_else(axum::headers::Error::invalid)?
            .to_str()
            .map_err(|_| axum::headers::Error::invalid())?;

        Ok(Self(country_code.to_string()))
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, _values: &mut E) {
        unimplemented!()
    }
}

impl std::fmt::Display for CloudflareIpCountryHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub struct SystemIdHeader(String);

impl Header for SystemIdHeader {
    fn name() -> &'static HeaderName {
        static SYSTEM_ID_HEADER: OnceLock<HeaderName> = OnceLock::new();
        SYSTEM_ID_HEADER.get_or_init(|| HeaderName::from_static("x-zed-system-id"))
    }

    fn decode<'i, I>(values: &mut I) -> Result<Self, axum::headers::Error>
    where
        Self: Sized,
        I: Iterator<Item = &'i axum::http::HeaderValue>,
    {
        let system_id = values
            .next()
            .ok_or_else(axum::headers::Error::invalid)?
            .to_str()
            .map_err(|_| axum::headers::Error::invalid())?;

        Ok(Self(system_id.to_string()))
    }

    fn encode<E: Extend<axum::http::HeaderValue>>(&self, _values: &mut E) {
        unimplemented!()
    }
}

impl std::fmt::Display for SystemIdHeader {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub fn routes(rpc_server: Arc<rpc::Server>) -> Router<(), Body> {
    Router::new()
        .route("/users/:id/access_tokens", post(create_access_token))
        .route("/rpc_server_snapshot", get(get_rpc_server_snapshot))
        .merge(contributors::router())
        .layer(
            ServiceBuilder::new()
                .layer(Extension(rpc_server))
                .layer(middleware::from_fn(validate_api_token)),
        )
}

pub async fn validate_api_token<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| {
            Error::http(
                StatusCode::BAD_REQUEST,
                "missing authorization header".to_string(),
            )
        })?
        .strip_prefix("token ")
        .ok_or_else(|| {
            Error::http(
                StatusCode::BAD_REQUEST,
                "invalid authorization header".to_string(),
            )
        })?;

    let state = req.extensions().get::<Arc<AppState>>().unwrap();

    if token != state.config.api_token {
        Err(Error::http(
            StatusCode::UNAUTHORIZED,
            "invalid authorization token".to_string(),
        ))?
    }

    Ok::<_, Error>(next.run(req).await)
}

async fn get_rpc_server_snapshot(
    Extension(rpc_server): Extension<Arc<rpc::Server>>,
) -> Result<ErasedJson> {
    Ok(ErasedJson::pretty(rpc_server.snapshot().await))
}

#[derive(Deserialize)]
struct CreateAccessTokenQueryParams {
    public_key: String,
    impersonate: Option<String>,
}

#[derive(Serialize)]
struct CreateAccessTokenResponse {
    user_id: UserId,
    encrypted_access_token: String,
}

async fn create_access_token(
    Path(user_id): Path<UserId>,
    Query(params): Query<CreateAccessTokenQueryParams>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<CreateAccessTokenResponse>> {
    let user = app
        .db
        .get_user_by_id(user_id)
        .await?
        .context("user not found")?;

    let mut impersonated_user_id = None;
    if let Some(impersonate) = params.impersonate {
        if user.admin {
            if let Some(impersonated_user) = app.db.get_user_by_github_login(&impersonate).await? {
                impersonated_user_id = Some(impersonated_user.id);
            } else {
                return Err(Error::http(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    format!("user {impersonate} does not exist"),
                ));
            }
        } else {
            return Err(Error::http(
                StatusCode::UNAUTHORIZED,
                "you do not have permission to impersonate other users".to_string(),
            ));
        }
    }

    let access_token =
        auth::create_access_token(app.db.as_ref(), user_id, impersonated_user_id).await?;
    let encrypted_access_token =
        auth::encrypt_access_token(&access_token, params.public_key.clone())?;

    Ok(Json(CreateAccessTokenResponse {
        user_id: impersonated_user_id.unwrap_or(user_id),
        encrypted_access_token,
    }))
}
