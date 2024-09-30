use zed_extension_api::{self as zed, settings::LspSettings, LanguageServerId, Result};

pub struct CustomLspBinary {
    pub path: String,
    pub args: Option<Vec<String>>,
}

pub struct CustomRubyLsp {}

impl CustomRubyLsp {
    pub const LANGUAGE_SERVER_ID: &'static str = "custom-ruby-lsp";

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
            args: binary.args.unwrap_or_default(),
            env: worktree.shell_env(),
        })
    }

    pub fn language_server_binary(
        &self,
        _language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<CustomLspBinary> {
        let binary_settings = LspSettings::for_worktree("custom-ruby-lsp", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(CustomLspBinary {
                path,
                args: binary_args,
            });
        }

        Err(format!(
            "Unable to start language server. Please verify if binary is installed correctly."
        ))
    }
}
