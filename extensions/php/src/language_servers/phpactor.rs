use std::fs;

use zed_extension_api::{self as zed, LanguageServerId, Result};

pub struct Phpactor {
    cached_binary_path: Option<String>,
}

impl Phpactor {
    pub const LANGUAGE_SERVER_ID: &'static str = "phpactor";

    pub fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    pub fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("phpactor") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        Err("Oops".into())
    }
}
