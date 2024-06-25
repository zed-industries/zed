#[path = "../wit/since_v0.0.7/settings.rs"]
mod types;

use crate::{wit, Result, SettingsLocation, Worktree};
use serde_json;
pub use types::*;

impl LanguageSettings {
    /// Returns the [`LanguageSettings`] for the given language.
    pub fn for_worktree(language: Option<&str>, worktree: &Worktree) -> Result<Self> {
        let location = SettingsLocation {
            worktree_id: worktree.id(),
            path: worktree.root_path(),
        };
        let settings_json = wit::get_settings(Some(&location), "language", language)?;
        let settings: Self = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;
        Ok(settings)
    }
}

impl LspSettings {
    /// Returns the [`LspSettings`] for the given language server.
    pub fn for_worktree(language_server_name: &str, worktree: &Worktree) -> Result<Self> {
        let location = SettingsLocation {
            worktree_id: worktree.id(),
            path: worktree.root_path(),
        };
        let settings_json = wit::get_settings(Some(&location), "lsp", Some(language_server_name))?;
        let settings: Self = serde_json::from_str(&settings_json).map_err(|err| err.to_string())?;
        Ok(settings)
    }
}
