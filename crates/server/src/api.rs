use crate::{auth, AppState, Request, RequestExt as _};
use async_trait::async_trait;
use serde_json::json;
use std::sync::Arc;

pub fn add_routes(app: &mut tide::Server<Arc<AppState>>) {
    app.at("/users/:github_login").get(get_user);
    app.at("/users/:github_login/access_tokens")
        .post(create_access_token);
}

async fn get_user(request: Request) -> tide::Result {
    request.require_token().await?;

    let user = request
        .db()
        .get_user_by_github_login(request.param("github_login")?)
        .await?
        .ok_or_else(|| surf::Error::from_str(404, "user not found"))?;

    Ok(tide::Response::builder(200)
        .body(tide::Body::from_json(&user)?)
        .build())
}

async fn create_access_token(request: Request) -> tide::Result {
    request.require_token().await?;

    let user = request
        .db()
        .get_user_by_github_login(request.param("github_login")?)
        .await?
        .ok_or_else(|| surf::Error::from_str(404, "user not found"))?;
    let token = auth::create_access_token(request.db(), user.id).await?;

    Ok(tide::Response::builder(200)
        .body(json!({"user_id": user.id, "access_token": token}))
        .build())
}

#[async_trait]
pub trait RequestExt {
    async fn require_token(&self) -> tide::Result<()>;
}

#[async_trait]
impl RequestExt for Request {
    async fn require_token(&self) -> tide::Result<()> {
        let token = self
            .header("Authorization")
            .and_then(|header| header.get(0))
            .and_then(|header| header.as_str().strip_prefix("token "))
            .ok_or_else(|| surf::Error::from_str(403, "invalid authorization header"))?;

        if token == self.state().config.api_token {
            Ok(())
        } else {
            Err(tide::Error::from_str(403, "invalid authorization token"))
        }
    }
}
