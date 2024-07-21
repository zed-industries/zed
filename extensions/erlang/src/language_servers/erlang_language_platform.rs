use zed_extension_api::{self as zed, LanguageServerId, Result};

pub struct ErlangLanguagePlatform;

impl ErlangLanguagePlatform {
    pub const LANGUAGE_SERVER_ID: &'static str = "elp";

    pub fn new() -> Self {
        Self
    }

    pub fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec!["server".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_binary_path(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        worktree
            .which("elp")
            .ok_or_else(|| "elp must be installed and available on your $PATH".to_string())
    }
}
