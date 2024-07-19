mod language_servers;

use zed_extension_api::{self as zed, serde_json, LanguageServerId, Result};

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
                intelephense.language_server_command(language_server_id, worktree)
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

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        match language_server_id.as_ref() {
            Intelephense::LANGUAGE_SERVER_ID => {
                if let Some(intelephense) = self.intelephense.as_mut() {
                    return intelephense.language_server_workspace_configuration(worktree);
                }
            }
            _ => (),
        }

        Ok(None)
    }
}

zed::register_extension!(PhpExtension);
