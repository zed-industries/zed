use std::sync::Arc;

use axum::extract::Query;
use axum::routing::get;
use axum::{Extension, Json, Router};
use serde::{Deserialize, Serialize};

use crate::{AppState, Result};

pub fn router() -> Router {
    Router::new().route("/extensions", get(get_extensions))
}

#[derive(Serialize)]
struct ExtensionJson {}

#[derive(Debug, Deserialize)]
struct GetExtensionsParams {
    filter: Option<String>,
}

async fn get_extensions(
    Extension(app): Extension<Arc<AppState>>,
    Query(params): Query<GetExtensionsParams>,
) -> Result<Json<Vec<ExtensionJson>>> {
    let extensions = app.db.get_extensions(params.filter.as_deref(), 30).await?;

    todo!()
}
