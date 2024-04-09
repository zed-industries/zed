use serde::{Deserialize, Serialize};
use std::num::NonZeroU32;

/// The settings for a particular language.
#[derive(Debug, Serialize, Deserialize)]
pub struct LanguageSettings {
    /// How many columns a tab should occupy.
    pub tab_size: NonZeroU32,
}

/// The settings for a particular language server.
#[derive(Default, Debug, Serialize, Deserialize)]
pub struct LspSettings {
    /// The settings for the language server binary.
    pub binary: Option<BinarySettings>,
    /// The initialization options to pass to the language server.
    pub initialization_options: Option<serde_json::Value>,
    /// The settings to pass to language server.
    pub settings: Option<serde_json::Value>,
}

/// The settings for a language server binary.
#[derive(Debug, Serialize, Deserialize)]
pub struct BinarySettings {
    /// The path to the binary.
    pub path: Option<String>,
    /// The arguments to pass to the binary.
    pub arguments: Option<Vec<String>>,
}
