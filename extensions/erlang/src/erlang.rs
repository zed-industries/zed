use zed_extension_api::{self as zed, Result};

struct ErlangExtension;

impl zed::Extension for ErlangExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _config: zed::LanguageServerConfig,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("erlang_ls")
            .ok_or_else(|| "erlang_ls must be installed and available on your $PATH".to_string())?;

        Ok(zed::Command {
            command: path,
            args: Vec::new(),
            env: Default::default(),
        })
    }
}

zed::register_extension!(ErlangExtension);
