use zed_extension_api::{self as zed, Result};

struct ScalaExtension;

impl zed::Extension for ScalaExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _config: zed::LanguageServerConfig,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = worktree
            .which("metals")
            .ok_or_else(|| "Metals must be installed via coursier. Please install coursier (https://get-coursier.io/), and then run `cs install metals`.".to_string())?;

        Ok(zed::Command {
            command: path,
            args: vec![],
            env: Default::default(),
        })
    }
}

zed::register_extension!(ScalaExtension);
