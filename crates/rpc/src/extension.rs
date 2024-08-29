use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ExtensionApiManifest {
    pub name: String,
    pub version: Arc<str>,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub repository: String,
    pub schema_version: Option<i32>,
    pub wasm_api_version: Option<String>,
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq)]
pub struct ExtensionMetadata {
    pub id: Arc<str>,
    #[serde(flatten)]
    pub manifest: ExtensionApiManifest,
    pub published_at: DateTime<Utc>,
    pub download_count: u64,
}

#[derive(Serialize, Deserialize)]
pub struct GetExtensionsResponse {
    pub data: Vec<ExtensionMetadata>,
}
