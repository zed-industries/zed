use std::sync::Arc;

use axum::{
    Extension, Json, Router,
    extract::{self},
    routing::get,
};
use serde::Deserialize;

use crate::{AppState, Result};

pub fn router() -> Router {
    Router::new().route("/contributors", get(get_contributors).post(add_contributor))
}

async fn get_contributors(Extension(app): Extension<Arc<AppState>>) -> Result<Json<Vec<String>>> {
    Ok(Json(app.db.get_contributors().await?))
}

#[derive(Debug, Deserialize)]
struct AddContributorBody {
    github_user_id: i32,
    github_login: String,
    github_email: Option<String>,
    github_name: Option<String>,
    github_user_created_at: chrono::DateTime<chrono::Utc>,
}

async fn add_contributor(
    Extension(app): Extension<Arc<AppState>>,
    extract::Json(params): extract::Json<AddContributorBody>,
) -> Result<()> {
    let initial_channel_id = app.config.auto_join_channel_id;
    app.db
        .add_contributor(
            &params.github_login,
            params.github_user_id,
            params.github_email.as_deref(),
            params.github_name.as_deref(),
            params.github_user_created_at,
            initial_channel_id,
        )
        .await
}
