use zed_extension_api::{self as zed, Result};

struct OcamlExtension;

impl zed::Extension for OcamlExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _config: zed::LanguageServerConfig,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("ocamllsp")
            .ok_or_else(|| "ocamllsp is not installed".to_string())?;

        Ok(zed::Command {
            command: path,
            args: Vec::new(),
            env: Default::default(),
        })
    }
}

zed::register_extension!(OcamlExtension);
