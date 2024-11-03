use zed_extension_api::{self as zed, Result};

const SERVER_PATH: &str = "idris2-lsp";

struct Idris2Extension;
impl Idris2Extension {
    fn server_script_path(&mut self, worktree: &zed::Worktree) -> Result<String> {
        worktree
            .which(SERVER_PATH)
            .ok_or_else(|| "idris2-lsp not found in PATH".into())
    }
}

impl zed::Extension for Idris2Extension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(worktree)?;
        Ok(zed::Command {
            command: server_path,
            env: Default::default(),
            args: Default::default(),
        })
    }
}

zed::register_extension!(Idris2Extension);
