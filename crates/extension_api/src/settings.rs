#[path = "../wit/since_v0.0.6/settings.rs"]
pub mod types;

use crate::{wit, Result};
use serde_json;
pub use types::*;

impl LanguageSettings {
    pub fn get(language: Option<&str>) -> Result<Self> {
        let settings_json = wit::get_settings("language", language)?;
        let settings: Self = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;
        Ok(settings)
    }
}

impl LspSettings {
    pub fn get(language_server_name: &str) -> Result<Self> {
        let settings_json = wit::get_settings("lsp", Some(language_server_name))?;
        let settings: Self = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;
        Ok(settings)
    }
}
