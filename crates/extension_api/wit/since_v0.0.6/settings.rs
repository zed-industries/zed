use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageSettings {
    pub tab_size: NonZeroU32,
}

#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LspSettings {
    pub binary: Option<BinarySettings>,
    pub initialization_options: Option<serde_json::Value>,
    pub settings: Option<serde_json::Value>,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BinarySettings {
    pub path: Option<String>,
    pub arguments: Option<Vec<String>>,
}
