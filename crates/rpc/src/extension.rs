use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, PartialEq)]
pub struct ExtensionApiManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub repository: String,
    pub schema_version: Option<i32>,
    pub wasm_api_version: Option<String>,
}

#[derive(Debug, Serialize, PartialEq)]
pub struct ExtensionMetadata {
    pub id: String,
    #[serde(flatten)]
    pub manifest: ExtensionApiManifest,
    pub published_at: DateTime<Utc>,
    pub download_count: u64,
}
