use zed_extension_api::{self as zed, LanguageServerId, Result};

pub struct ErlangLs;

impl ErlangLs {
    pub const LANGUAGE_SERVER_ID: &'static str = "erlang-ls";

    pub fn new() -> Self {
        Self
    }

    pub fn language_server_binary_path(
        &mut self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        worktree
            .which("erlang_ls")
            .ok_or_else(|| "erlang_ls must be installed and available on your $PATH".to_string())
    }
}
