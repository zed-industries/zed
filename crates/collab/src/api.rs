use crate::{
    auth,
    db::{User, UserId},
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
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tower::ServiceBuilder;

pub fn routes(state: Arc<AppState>) -> Router<Body> {
    Router::new()
        .route("/users", get(get_users).post(create_user))
        .route("/users/:id", put(update_user).delete(destroy_user))
        .route("/users/:gh_login", get(get_user))
        .route("/users/:gh_login/access_tokens", post(create_access_token))
        .layer(
            ServiceBuilder::new()
                .layer(Extension(state))
                .layer(middleware::from_fn(validate_api_token)),
        )
    // TODO: Compression on API routes?
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

    if token != state.api_token {
        Err(Error::Http(
            StatusCode::UNAUTHORIZED,
            "invalid authorization token".to_string(),
        ))?
    }

    Ok::<_, Error>(next.run(req).await)
}

async fn get_users(Extension(app): Extension<Arc<AppState>>) -> Result<Json<Vec<User>>> {
    let users = app.db.get_all_users().await?;
    Ok(Json(users))
}

#[derive(Deserialize)]
struct CreateUserParams {
    github_login: String,
    admin: bool,
}

async fn create_user(
    Json(params): Json<CreateUserParams>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<User>> {
    let user_id = app
        .db
        .create_user(&params.github_login, params.admin)
        .await?;

    let user = app
        .db
        .get_user_by_id(user_id)
        .await?
        .ok_or_else(|| anyhow!("couldn't find the user we just created"))?;

    Ok(Json(user))
}

#[derive(Deserialize)]
struct UpdateUserParams {
    admin: bool,
}

async fn update_user(
    Path(user_id): Path<i32>,
    Json(params): Json<UpdateUserParams>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<()> {
    app.db
        .set_user_is_admin(UserId(user_id), params.admin)
        .await?;
    Ok(())
}

async fn destroy_user(
    Path(user_id): Path<i32>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<()> {
    app.db.destroy_user(UserId(user_id)).await?;
    Ok(())
}

async fn get_user(
    Path(login): Path<String>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<User>> {
    let user = app
        .db
        .get_user_by_github_login(&login)
        .await?
        .ok_or_else(|| anyhow!("user not found"))?;
    Ok(Json(user))
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
    Path(login): Path<String>,
    Query(params): Query<CreateAccessTokenQueryParams>,
    Extension(app): Extension<Arc<AppState>>,
) -> Result<Json<CreateAccessTokenResponse>> {
    //     request.require_token().await?;

    let user = app
        .db
        .get_user_by_github_login(&login)
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
                format!("you do not have permission to impersonate other users"),
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
