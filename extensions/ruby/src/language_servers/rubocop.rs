use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

pub struct RubocopBinary {
    pub path: String,
    pub args: Option<Vec<String>>,
}

pub struct Rubocop {}

impl Rubocop {
    pub const LANGUAGE_SERVER_ID: &'static str = "rubocop";

    pub fn new() -> Self {
        Self {}
    }

    pub fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary = self.language_server_binary(language_server_id, worktree)?;

        Ok(zed::Command {
            command: binary.path,
            args: binary.args.unwrap_or_else(|| vec!["--lsp".to_string()]),
            env: worktree.shell_env(),
        })
    }

    fn language_server_binary(
        &self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<RubocopBinary> {
        let binary_settings = LspSettings::for_worktree("rubocop", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(RubocopBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = worktree.which("rubocop") {
            return Ok(RubocopBinary {
                path,
                args: binary_args,
            });
        }

        Err("rubocop must be installed manually. Install it with `gem install rubocop` or specify the 'binary' path to it via local settings.".to_string())
    }
}
