// use crate::{auth, db::UserId, AppState, Request, RequestExt as _};
use async_trait::async_trait;
use serde::Deserialize;
use serde_json::json;
use std::sync::Arc;
// use surf::StatusCode;

// pub fn add_routes(app: &mut tide::Server<Arc<AppState>>) {
//     app.at("/users").get(get_users);
//     app.at("/users").post(create_user);
//     app.at("/users/:id").put(update_user);
//     app.at("/users/:id").delete(destroy_user);
//     app.at("/users/:github_login").get(get_user);
//     app.at("/users/:github_login/access_tokens")
//         .post(create_access_token);
// }

// async fn get_user(request: Request) -> tide::Result {
//     request.require_token().await?;

//     let user = request
//         .db()
//         .get_user_by_github_login(request.param("github_login")?)
//         .await?
//         .ok_or_else(|| surf::Error::from_str(404, "user not found"))?;

//     Ok(tide::Response::builder(StatusCode::Ok)
//         .body(tide::Body::from_json(&user)?)
//         .build())
// }

// async fn get_users(request: Request) -> tide::Result {
//     request.require_token().await?;

//     let users = request.db().get_all_users().await?;

//     Ok(tide::Response::builder(StatusCode::Ok)
//         .body(tide::Body::from_json(&users)?)
//         .build())
// }

// async fn create_user(mut request: Request) -> tide::Result {
//     request.require_token().await?;

//     #[derive(Deserialize)]
//     struct Params {
//         github_login: String,
//         admin: bool,
//     }
//     let params = request.body_json::<Params>().await?;

//     let user_id = request
//         .db()
//         .create_user(&params.github_login, params.admin)
//         .await?;

//     let user = request.db().get_user_by_id(user_id).await?.ok_or_else(|| {
//         surf::Error::from_str(
//             StatusCode::InternalServerError,
//             "couldn't find the user we just created",
//         )
//     })?;

//     Ok(tide::Response::builder(StatusCode::Ok)
//         .body(tide::Body::from_json(&user)?)
//         .build())
// }

// async fn update_user(mut request: Request) -> tide::Result {
//     request.require_token().await?;

//     #[derive(Deserialize)]
//     struct Params {
//         admin: bool,
//     }
//     let user_id = UserId(
//         request
//             .param("id")?
//             .parse::<i32>()
//             .map_err(|error| surf::Error::from_str(StatusCode::BadRequest, error.to_string()))?,
//     );
//     let params = request.body_json::<Params>().await?;

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
