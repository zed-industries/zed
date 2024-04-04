#[path = "../wit/since_v0.0.6/settings.rs"]
pub mod types;

pub use types::*;

use serde_json;

use crate::{wit, Result};

impl LanguageSettings {
    const SETTINGS_KEY: &'static str = "language";

    pub fn get() -> Result<Self> {
        let settings_json = wit::get_settings(Self::SETTINGS_KEY)?;
        let settings: Self = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;
        Ok(settings)
    }
}
