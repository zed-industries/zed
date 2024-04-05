use zed::settings::LspSettings;
use zed_extension_api::{self as zed, serde_json, Result};

struct DartExtension;

impl zed::Extension for DartExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("dart")
            .ok_or_else(|| "dart must me installed from dart.dev/get-dart".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec!["language-server".to_string(), "--protocol=lsp".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("dart", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(serde_json::json!({
            "dart": settings
        })))
    }
}

zed::register_extension!(DartExtension);
