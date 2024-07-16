mod language_servers;

use std::env;

use zed_extension_api::{self as zed, LanguageServerId, Result};

use crate::language_servers::{Intelephense, Phpactor};

struct PhpExtension {
    intelephense: Option<Intelephense>,
    phpactor: Option<Phpactor>,
}

impl zed::Extension for PhpExtension {
    fn new() -> Self {
        Self {
            intelephense: None,
            phpactor: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            Intelephense::LANGUAGE_SERVER_ID => {
                let intelephense = self.intelephense.get_or_insert_with(|| Intelephense::new());

                let server_path = intelephense.server_script_path(language_server_id)?;
                Ok(zed::Command {
                    command: zed::node_binary_path()?,
                    args: vec![
                        env::current_dir()
                            .unwrap()
                            .join(&server_path)
                            .to_string_lossy()
                            .to_string(),
                        "--stdio".to_string(),
                    ],
                    env: Default::default(),
                })
            }
            Phpactor::LANGUAGE_SERVER_ID => {
                let phpactor = self.phpactor.get_or_insert_with(|| Phpactor::new());

                Ok(zed::Command {
                    command: phpactor.language_server_binary_path(language_server_id, worktree)?,
                    args: vec!["language-server".into()],
                    env: Default::default(),
                })
            }
            language_server_id => Err(format!("unknown language server: {language_server_id}")),
        }
    }
}

zed::register_extension!(PhpExtension);
