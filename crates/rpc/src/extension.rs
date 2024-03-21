use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize)]
pub struct ExtensionApiManifest {
    pub name: String,
    pub version: String,
    pub description: Option<String>,
    pub authors: Vec<String>,
    pub repository: String,
    pub schema_version: Option<i32>,
}
