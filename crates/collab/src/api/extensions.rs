use std::sync::Arc;

use axum::extract::Query;
use axum::routing::get;
use axum::{Extension, Json, Router};
use collections::BTreeMap;
use serde::{Deserialize, Serialize};

use crate::{AppState, Result};

pub fn router() -> Router {
    Router::new().route("/extensions", get(get_extensions))
}

#[derive(Debug, Serialize)]
struct ExtensionJson {
    pub id: String,
    pub name: String,
    pub version: String,
    pub authors: Vec<String>,
    pub repository: String,
    pub languages: Option<BTreeMap<String, String>>,
    pub grammars: Option<BTreeMap<String, String>>,
    pub themes: Option<BTreeMap<String, String>>,
    pub published_at: String,
}

#[derive(Debug, Deserialize)]
struct GetExtensionsParams {
    filter: Option<String>,
}

#[derive(Debug, Serialize)]
struct GetExtensionsResponse {
    pub data: Vec<ExtensionJson>,
}

async fn get_extensions(
    Extension(app): Extension<Arc<AppState>>,
    Query(params): Query<GetExtensionsParams>,
) -> Result<Json<GetExtensionsResponse>> {
    let extensions = app.db.get_extensions(params.filter.as_deref(), 30).await?;

    Ok(Json(GetExtensionsResponse {
        data: extensions
            .into_iter()
            .map(|(extension, version)| ExtensionJson {
                id: extension.external_id,
                name: extension.name,
                version: version.version,
                authors: version
                    .authors
                    .split(',')
                    .map(|author| author.trim().to_string())
                    .collect::<Vec<_>>(),
                repository: version.repository,
                grammars: None,
                languages: None,
                themes: None,
                published_at: "todo!()".into(),
            })
            .collect(),
    }))
}
