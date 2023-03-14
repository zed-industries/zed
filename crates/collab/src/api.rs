use crate::{
    auth,
    db::{Invite, NewSignup, NewUserParams, User, UserId, WaitlistSummary},
    rpc::{self, ResultExt},
    AppState, Error, Result,
};
use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Path, Query},
    http::{self, Request, StatusCode},
    middleware::{self, Next},
    response::IntoResponse,
    routing::{get, post, put},
    Extension, Json, Router,
};
use axum_extra::response::ErasedJson;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower::ServiceBuilder;
use tracing::instrument;

pub fn routes(rpc_server: Arc<rpc::Server>, state: Arc<AppState>) -> Router<Body> {
    Router::new()
        .route("/user", get(get_authenticated_user))
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", put(update_user).delete(destroy_user))
        .route("/users/:id/access_tokens", post(create_access_token))
        .route("/users_with_no_invites", get(get_users_with_no_invites))
        .route("/invite_codes/:code", get(get_user_for_invite_code))
        .route("/panic", post(trace_panic))
        .route("/rpc_server_snapshot", get(get_rpc_server_snapshot))
        .route("/signups", post(create_signup))
        .route("/signups_summary", get(get_waitlist_summary))
        .route("/user_invites", post(create_invite_from_code))
        .route("/unsent_invites", get(get_unsent_invites))
        .route("/sent_invites", post(record_sent_invites))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(state))
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

#[derive(Debug, Deserialize)]
struct GetUsersQueryParams {
    query: Option<String>,
    page: Option<u32>,
    limit: Option<u32>,
}

async fn get_users(
    Query(params): Query<GetUsersQueryParams>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<Vec<User>>> {
    let limit = params.limit.unwrap_or(100);
    let users = if let Some(query) = params.query {
        app.db.fuzzy_search_users(&query, limit).await?
    } else {
        app.db
            .get_all_users(params.page.unwrap_or(0), limit)
            .await?
    };
    Ok(Json(users))
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

async fn create_user(
    Json(params): Json<CreateUserParams>,
    Extension(app): Extension<Arc<AppState>>,
    Extension(rpc_server): Extension<Arc<rpc::Server>>,
) -> Result<Json<Option<CreateUserResponse>>> {
    let user = NewUserParams {
        github_login: params.github_login,
        github_user_id: params.github_user_id,
        invite_count: params.invite_count,
    };

    // Creating a user via the normal signup process
    let result = if let Some(email_confirmation_code) = params.email_confirmation_code {
        if let Some(result) = app
            .db
            .create_user_from_invite(
                &Invite {
                    email_address: params.email_address,
                    email_confirmation_code,
                },
                user,
            )
            .await?
        {
            result
        } else {
            return Ok(Json(None));
        }
    }
    // Creating a user as an admin
    else if params.admin {
        app.db
            .create_user(&params.email_address, false, user)
            .await?
    } else {
        Err(Error::Http(
            StatusCode::UNPROCESSABLE_ENTITY,
            "email confirmation code is required".into(),
        ))?
    };

    if let Some(inviter_id) = result.inviting_user_id {
        rpc_server
            .invite_code_redeemed(inviter_id, result.user_id)
            .await
            .trace_err();
    }

    let user = app
        .db
        .get_user_by_id(result.user_id)
        .await?
        .ok_or_else(|| anyhow!("couldn't find the user we just created"))?;

    Ok(Json(Some(CreateUserResponse {
        user,
        metrics_id: result.metrics_id,
        signup_device_id: result.signup_device_id,
    })))
}

#[derive(Deserialize)]
struct UpdateUserParams {
    admin: Option<bool>,
    invite_count: Option<i32>,
}

async fn update_user(
    Path(user_id): Path<i32>,
    Json(params): Json<UpdateUserParams>,
    Extension(app): Extension<Arc<AppState>>,
    Extension(rpc_server): Extension<Arc<rpc::Server>>,
) -> Result<()> {
    let user_id = UserId(user_id);

    if let Some(admin) = params.admin {
        app.db.set_user_is_admin(user_id, admin).await?;
    }

    if let Some(invite_count) = params.invite_count {
        app.db
            .set_invite_count_for_user(user_id, invite_count)
            .await?;
        rpc_server.invite_count_updated(user_id).await.trace_err();
    }

    Ok(())
}

async fn destroy_user(
    Path(user_id): Path<i32>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<()> {
    app.db.destroy_user(UserId(user_id)).await?;
    Ok(())
}

#[derive(Debug, Deserialize)]
struct GetUsersWithNoInvites {
    invited_by_another_user: bool,
}

async fn get_users_with_no_invites(
    Query(params): Query<GetUsersWithNoInvites>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<Vec<User>>> {
    Ok(Json(
        app.db
            .get_users_with_no_invites(params.invited_by_another_user)
            .await?,
    ))
}

#[derive(Debug, Deserialize)]
struct Panic {
    version: String,
    text: String,
}

#[instrument(skip(panic))]
async fn trace_panic(panic: Json<Panic>) -> Result<()> {
    tracing::error!(version = %panic.version, text = %panic.text, "panic report");
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

async fn get_user_for_invite_code(
    Path(code): Path<String>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<User>> {
    Ok(Json(app.db.get_user_for_invite_code(&code).await?))
}

async fn create_signup(
    Json(params): Json<NewSignup>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<()> {
    app.db.create_signup(&params).await?;
    Ok(())
}

async fn get_waitlist_summary(
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<WaitlistSummary>> {
    Ok(Json(app.db.get_waitlist_summary().await?))
}

#[derive(Deserialize)]
pub struct CreateInviteFromCodeParams {
    invite_code: String,
    email_address: String,
    device_id: Option<String>,
    #[serde(default)]
    added_to_mailing_list: bool,
}

async fn create_invite_from_code(
    Json(params): Json<CreateInviteFromCodeParams>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<Invite>> {
    Ok(Json(
        app.db
            .create_invite_from_code(
                &params.invite_code,
                &params.email_address,
                params.device_id.as_deref(),
                params.added_to_mailing_list,
            )
            .await?,
    ))
}

#[derive(Deserialize)]
pub struct GetUnsentInvitesParams {
    pub count: usize,
}

async fn get_unsent_invites(
    Query(params): Query<GetUnsentInvitesParams>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<Vec<Invite>>> {
    Ok(Json(app.db.get_unsent_invites(params.count).await?))
}

async fn record_sent_invites(
    Json(params): Json<Vec<Invite>>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<()> {
    app.db.record_sent_invites(&params).await?;
    Ok(())
}
