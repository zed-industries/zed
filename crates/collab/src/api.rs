use crate::{
    db::{Db, User, UserId},
    AppState, Result,
};
use anyhow::anyhow;
use axum::{
    body::Body,
    extract::Path,
    http::{Request, StatusCode},
    response::{IntoResponse, Response},
    routing::{get, put},
    Json, Router,
};
use serde::Deserialize;
use std::sync::Arc;

pub fn add_routes(router: Router<Body>, app: Arc<AppState>) -> Router<Body> {
    router
        .route("/users", {
            let app = app.clone();
            get(move |req| get_users(req, app))
        })
        .route("/users", {
            let app = app.clone();
            get(move |params| create_user(params, app))
        })
        .route("/users/:id", {
            let app = app.clone();
            put(move |user_id, params| update_user(user_id, params, app))
        })
}

// pub fn add_routes(app: &mut tide::Server<Arc<AppState>>) {
//     app.at("/users").get(get_users);
//     app.at("/users").post(create_user);
//     app.at("/users/:id").put(update_user);
//     app.at("/users/:id").delete(destroy_user);
//     app.at("/users/:github_login").get(get_user);
//     app.at("/users/:github_login/access_tokens")
//         .post(create_access_token);
// }

async fn get_users(request: Request<Body>, app: Arc<AppState>) -> Result<Json<Vec<User>>> {
    // request.require_token().await?;

    let users = app.db.get_all_users().await?;
    Ok(Json(users))
}

#[derive(Deserialize)]
struct CreateUser {
    github_login: String,
    admin: bool,
}

async fn create_user(Json(params): Json<CreateUser>, app: Arc<AppState>) -> Result<Json<User>> {
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
struct UpdateUser {
    admin: bool,
}

async fn update_user(
    Path(user_id): Path<i32>,
    Json(params): Json<UpdateUser>,
    app: Arc<AppState>,
) -> Result<impl IntoResponse> {
    let user_id = UserId(user_id);
    app.db.set_user_is_admin(user_id, params.admin).await?;
    Ok(())
}

// async fn update_user(mut request: Request) -> tide::Result {
//     request.require_token().await?;

//     #[derive(Deserialize)]
//     struct Params {
//         admin: bool,
//     }

//     request
//         .db()
//         .set_user_is_admin(user_id, params.admin)
//         .await?;

//     Ok(tide::Response::builder(StatusCode::Ok).build())
// }

// async fn destroy_user(request: Request) -> tide::Result {
//     request.require_token().await?;
//     let user_id = UserId(
//         request
//             .param("id")?
//             .parse::<i32>()
//             .map_err(|error| surf::Error::from_str(StatusCode::BadRequest, error.to_string()))?,
//     );

//     request.db().destroy_user(user_id).await?;

//     Ok(tide::Response::builder(StatusCode::Ok).build())
// }

// async fn create_access_token(request: Request) -> tide::Result {
//     request.require_token().await?;

//     let user = request
//         .db()
//         .get_user_by_github_login(request.param("github_login")?)
//         .await?
//         .ok_or_else(|| surf::Error::from_str(StatusCode::NotFound, "user not found"))?;

//     #[derive(Deserialize)]
//     struct QueryParams {
//         public_key: String,
//         impersonate: Option<String>,
//     }

//     let query_params: QueryParams = request.query().map_err(|_| {
//         surf::Error::from_str(StatusCode::UnprocessableEntity, "invalid query params")
//     })?;

//     let mut user_id = user.id;
//     if let Some(impersonate) = query_params.impersonate {
//         if user.admin {
//             if let Some(impersonated_user) =
//                 request.db().get_user_by_github_login(&impersonate).await?
//             {
//                 user_id = impersonated_user.id;
//             } else {
//                 return Ok(tide::Response::builder(StatusCode::UnprocessableEntity)
//                     .body(format!(
//                         "Can't impersonate non-existent user {}",
//                         impersonate
//                     ))
//                     .build());
//             }
//         } else {
//             return Ok(tide::Response::builder(StatusCode::Unauthorized)
//                 .body(format!(
//                     "Can't impersonate user {} because the real user isn't an admin",
//                     impersonate
//                 ))
//                 .build());
//         }
//     }

//     let access_token = auth::create_access_token(request.db().as_ref(), user_id).await?;
//     let encrypted_access_token =
//         auth::encrypt_access_token(&access_token, query_params.public_key.clone())?;

//     Ok(tide::Response::builder(StatusCode::Ok)
//         .body(json!({"user_id": user_id, "encrypted_access_token": encrypted_access_token}))
//         .build())
// }

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
