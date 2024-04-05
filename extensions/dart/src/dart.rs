use zed_extension_api::{self as zed, Result};

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

    // Previous implementation:
    // fn workspace_configuration(
    //     self: Arc<Self>,
    //     _workspace_root: &Path,
    //     cx: &mut AppContext,
    // ) -> Value {
    //     let settings = ProjectSettings::get_global(cx)
    //         .lsp
    //         .get("dart")
    //         .and_then(|s| s.settings.clone())
    //         .unwrap_or_default();

    //     serde_json::json!({
    //         "dart": settings
    //     })
    // }
}

zed::register_extension!(DartExtension);
