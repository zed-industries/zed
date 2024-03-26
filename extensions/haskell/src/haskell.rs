use zed_extension_api::{self as zed, Result};

struct HaskellExtension;

impl zed::Extension for HaskellExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _config: zed::LanguageServerConfig,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("haskell-language-server-wrapper")
            .ok_or_else(|| "hls must be installed via ghcup".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec!["lsp".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(HaskellExtension);
