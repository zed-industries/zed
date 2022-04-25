use crate::{
    auth,
    db::{User, UserId},
    AppState, Error, Result,
};
use anyhow::anyhow;
use axum::{
    body::Body,
    extract::{Path, Query},
    http::StatusCode,
    routing::{delete, get, post, put},
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

pub fn add_routes(router: Router<Body>, app: Arc<AppState>) -> Router<Body> {
    router
        .route("/users", {
            let app = app.clone();
            get(move || get_users(app))
        })
        .route("/users", {
            let app = app.clone();
            post(move |params| create_user(params, app))
        })
        .route("/users/:id", {
            let app = app.clone();
            put(move |user_id, params| update_user(user_id, params, app))
        })
        .route("/users/:id", {
            let app = app.clone();
            delete(move |user_id| destroy_user(user_id, app))
        })
        .route("/users/:github_login", {
            let app = app.clone();
            get(move |github_login| get_user(github_login, app))
        })
        .route("/users/:github_login/access_tokens", {
            let app = app.clone();
            post(move |github_login, params| create_access_token(github_login, params, app))
        })
}

async fn get_users(app: Arc<AppState>) -> Result<Json<Vec<User>>> {
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
    app: Arc<AppState>,
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
    app: Arc<AppState>,
) -> Result<()> {
    app.db
        .set_user_is_admin(UserId(user_id), params.admin)
        .await?;
    Ok(())
}

async fn destroy_user(Path(user_id): Path<i32>, app: Arc<AppState>) -> Result<()> {
    app.db.destroy_user(UserId(user_id)).await?;
    Ok(())
}

async fn get_user(Path(login): Path<String>, app: Arc<AppState>) -> Result<Json<User>> {
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
    app: Arc<AppState>,
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

// #[async_trait]
// pub trait RequestExt {
//     async fn require_token(&self) -> tide::Result<()>;
// }

// #[async_trait]
// impl RequestExt for Request {
//     async fn require_token(&self) -> tide::Result<()> {
//         let token = self
//             .header("Authorization")
//             .and_then(|header| header.get(0))
//             .and_then(|header| header.as_str().strip_prefix("token "))
//             .ok_or_else(|| surf::Error::from_str(403, "invalid authorization header"))?;

//         if token == self.state().config.api_token {
//             Ok(())
//         } else {
//             Err(tide::Error::from_str(403, "invalid authorization token"))
//         }
//     }
// }
