use crate::{
    auth,
    db::{User, UserId},
    rpc, AppState, Error, Result,
};
use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Path, Query},
    http::{self, Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post},
    Extension, Json, Router,
};
use axum_extra::response::ErasedJson;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower::ServiceBuilder;
use tracing::instrument;
use util::{async_maybe, http::AsyncBody, ResultExt};

pub fn routes(rpc_server: Arc<rpc::Server>, state: Arc<AppState>) -> Router<Body> {
    let called_from_website = Router::new()
        .route("/user", get(get_authenticated_user))
        .route("/users/:id/access_tokens", post(create_access_token))
        .route("/panic", post(trace_panic))
        .route("/rpc_server_snapshot", get(get_rpc_server_snapshot))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(state.clone()))
                .layer(Extension(rpc_server))
                .layer(middleware::from_fn(validate_api_token)),
        );

    let called_from_client = Router::new().route("/crash", post(trace_crash)).layer(
        ServiceBuilder::new()
            .layer(Extension(state))
            .layer(middleware::from_fn(validate_client_secret)),
    );

    called_from_website.merge(called_from_client)
}

pub async fn validate_api_token<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| {
            Error::Http(
                StatusCode::BAD_REQUEST,
                "missing authorization header".to_string(),
            )
        })?
        .strip_prefix("token ")
        .ok_or_else(|| {
            Error::Http(
                StatusCode::BAD_REQUEST,
                "invalid authorization header".to_string(),
            )
        })?;

    let state = req.extensions().get::<Arc<AppState>>().unwrap();

    if token != state.config.api_token {
        Err(Error::Http(
            StatusCode::UNAUTHORIZED,
            "invalid authorization token".to_string(),
        ))?
    }

    Ok::<_, Error>(next.run(req).await)
}

pub async fn validate_client_secret<B>(req: Request<B>, next: Next<B>) -> impl IntoResponse {
    let token = req
        .headers()
        .get(http::header::AUTHORIZATION)
        .and_then(|header| header.to_str().ok())
        .ok_or_else(|| {
            Error::Http(
                StatusCode::BAD_REQUEST,
                "missing authorization header".to_string(),
            )
        })?
        .strip_prefix("token ")
        .ok_or_else(|| {
            Error::Http(
                StatusCode::BAD_REQUEST,
                "invalid authorization header".to_string(),
            )
        })?;

    let state = req.extensions().get::<Arc<AppState>>().unwrap();

    if token != state.config.client_token {
        Err(Error::Http(
            StatusCode::UNAUTHORIZED,
            "invalid client secret".to_string(),
        ))?
    }

    Ok::<_, Error>(next.run(req).await)
}

#[derive(Debug, Deserialize)]
struct AuthenticatedUserParams {
    github_user_id: Option<i32>,
    github_login: String,
    github_email: Option<String>,
}

#[derive(Debug, Serialize)]
struct AuthenticatedUserResponse {
    user: User,
    metrics_id: String,
}

async fn get_authenticated_user(
    Query(params): Query<AuthenticatedUserParams>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<AuthenticatedUserResponse>> {
    let user = app
        .db
        .get_or_create_user_by_github_account(
            &params.github_login,
            params.github_user_id,
            params.github_email.as_deref(),
        )
        .await?
        .ok_or_else(|| Error::Http(StatusCode::NOT_FOUND, "user not found".into()))?;
    let metrics_id = app.db.get_user_metrics_id(user.id).await?;
    return Ok(Json(AuthenticatedUserResponse { user, metrics_id }));
}

#[derive(Deserialize, Debug)]
struct CreateUserParams {
    github_user_id: i32,
    github_login: String,
    email_address: String,
    email_confirmation_code: Option<String>,
    #[serde(default)]
    admin: bool,
    #[serde(default)]
    invite_count: i32,
}

#[derive(Serialize, Debug)]
struct CreateUserResponse {
    user: User,
    signup_device_id: Option<String>,
    metrics_id: String,
}

#[derive(Debug, Deserialize)]
struct Panic {
    version: String,
    release_channel: String,
    backtrace_hash: String,
    text: String,
}

#[instrument(skip(panic))]
async fn trace_panic(panic: Json<Panic>) -> Result<()> {
    tracing::error!(version = %panic.version, release_channel = %panic.release_channel, backtrace_hash = %panic.backtrace_hash, text = %panic.text, "panic report");
    Ok(())
}

/// IPSHeader is the first line of an .ips file (in JSON format)
/// https://developer.apple.com/documentation/xcode/interpreting-the-json-format-of-a-crash-report
#[derive(Debug, Serialize, Deserialize)]
struct IPSHeader {
    timestamp: Option<String>,
    name: Option<String>,
    app_name: Option<String>,
    app_version: Option<String>,
    slice_uuid: Option<String>,
    build_version: Option<String>,
    platform: Option<i32>,
    #[serde(rename = "bundleID")]
    bundle_id: Option<String>,
    share_with_app_devs: Option<i32>,
    is_first_party: Option<i32>,
    bug_type: Option<String>,
    os_version: Option<String>,
    roots_installed: Option<i32>,
    incident_id: Option<String>,
}

#[instrument(skip(content, app))]
async fn trace_crash(content: String, Extension(app): Extension<Arc<AppState>>) -> Result<()> {
    let Some(header) = content.split("\n").next() else {
        return Err(Error::Http(
            StatusCode::BAD_REQUEST,
            "invalid .ips file".to_string(),
        ));
    };
    let header: IPSHeader = serde_json::from_slice(&header.as_bytes())?;
    let text = content.as_str();

    tracing::error!(app_version = %header.app_version.clone().unwrap_or_default(),
        build_version = %header.build_version.unwrap_or_default(),
        os_version = %header.os_version.unwrap_or_default(),
        bundle_id = %header.bundle_id.clone().unwrap_or_default(),
        text = %text,
    "crash report");

    async_maybe!({
        let api_key = app.config.slack_api_key.clone()?;
        let channel = app.config.slack_panic_channel.clone()?;

        let mut body = form_data_builder::FormData::new(Vec::new());
        body.write_field("content", text).log_err()?;
        body.write_field("channels", channel.as_str()).log_err()?;
        body.write_field(
            "filename",
            format!("zed-crash-{}.ips", header.incident_id.unwrap_or_default()).as_str(),
        )
        .log_err()?;
        body.write_field(
            "initial_comment",
            format!(
                "New crash in {} ({})",
                header.bundle_id.unwrap_or_default(),
                header.app_version.unwrap_or_default()
            )
            .as_str(),
        )
        .log_err()?;
        let content_type = body.content_type_header();
        let body = AsyncBody::from(body.finish().log_err()?);

        let request = Request::post("https://slack.com/api/files.upload")
            .header("Content-Type", content_type)
            .header("Authorization", format!("Bearer {}", api_key))
            .body(body)
            .log_err()?;

        let response = util::http::client().send(request).await.log_err()?;
        if !response.status().is_success() {
            tracing::error!(response = ?response, "failed to send crash report to slack");
        }

        Some(())
    })
    .await;
    Ok(())
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
        .ok_or_else(|| anyhow!("user not found"))?;

    let mut user_id = user.id;
    if let Some(impersonate) = params.impersonate {
        if user.admin {
            if let Some(impersonated_user) = app.db.get_user_by_github_login(&impersonate).await? {
                user_id = impersonated_user.id;
            } else {
                return Err(Error::Http(
                    StatusCode::UNPROCESSABLE_ENTITY,
                    format!("user {impersonate} does not exist"),
                ));
            }
        } else {
            return Err(Error::Http(
                StatusCode::UNAUTHORIZED,
                "you do not have permission to impersonate other users".to_string(),
            ));
        }
    }

    let access_token = auth::create_access_token(app.db.as_ref(), user_id).await?;
    let encrypted_access_token =
        auth::encrypt_access_token(&access_token, params.public_key.clone())?;

    Ok(Json(CreateAccessTokenResponse {
        user_id,
        encrypted_access_token,
    }))
}
