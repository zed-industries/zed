use zed_extension_api::{self as zed, settings::LspSettings, Result};

use crate::RubyLanguageServerCommand;

pub struct Rubocop {}

impl Rubocop {
    pub const LANGUAGE_SERVER_ID: &'static str = "rubocop";

    pub fn new() -> Self {
        Self {}
    }

    pub fn language_server_command(
        &self,
        worktree: &zed::Worktree,
    ) -> Result<RubyLanguageServerCommand> {
        let mut binary = None;
        let mut args = None;

        if let Some(binary_settings) =
            LspSettings::for_worktree(Rubocop::LANGUAGE_SERVER_ID, worktree)
                .ok()
                .and_then(|lsp_settings| lsp_settings.binary)
        {
            if let Some(bin_path) = binary_settings.path {
                binary = Some(bin_path);
            }
            if let Some(bin_args) = binary_settings.arguments {
                args = Some(bin_args);
            }
        }
        let command = if let Some(binary) = binary {
            binary
        } else {
            worktree.which("rubocop").ok_or_else(|| {
                "rubocop must be installed manually. Install it with `gem install rubocop` or specify the 'binary' path to it via local settings.".to_string()
            })?
        };
        let args = args.unwrap_or_else(|| vec!["--lsp".into()]);

        Ok(RubyLanguageServerCommand { command, args })
    }
}
