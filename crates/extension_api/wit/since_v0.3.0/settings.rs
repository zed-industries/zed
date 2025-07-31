use serde::{Deserialize, Serialize};
use std::{collections::HashMap, num::NonZeroU32};

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
    pub binary: Option<CommandSettings>,
    /// The initialization options to pass to the language server.
    pub initialization_options: Option<serde_json::Value>,
    /// The settings to pass to language server.
    pub settings: Option<serde_json::Value>,
}

/// The settings for a particular context server.
#[derive(Default, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct ContextServerSettings {
    /// The settings for the context server binary.
    pub command: Option<CommandSettings>,
    /// The settings to pass to the context server.
    pub settings: Option<serde_json::Value>,
}

/// The settings for a command.
#[derive(Debug, Serialize, Deserialize, PartialEq, Eq)]
pub struct CommandSettings {
    /// The path to the command.
    pub path: Option<String>,
    /// The arguments to pass to the command.
    pub arguments: Option<Vec<String>>,
    /// The environment variables.
    pub env: Option<HashMap<String, String>>,
}
