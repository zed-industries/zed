mod language_servers;

use zed::lsp::{Completion, Symbol};
use zed::settings::LspSettings;
use zed::{serde_json, CodeLabel, LanguageServerId};
use zed_extension_api::{self as zed, Result};

use crate::language_servers::{BasedPyright, Pyright};

struct PythonExtension {
    basedpyright: Option<BasedPyright>,
    pyright: Option<Pyright>,
}

impl zed::Extension for PythonExtension {
    fn new() -> Self {
        Self {
            basedpyright: None,
            pyright: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            BasedPyright::LANGUAGE_SERVER_ID => {
                let basedpyright = self.basedpyright.get_or_insert_with(|| BasedPyright::new());

                Ok(zed::Command {
                    command: basedpyright.server_script_path(worktree)?,
                    args: vec!["--stdio".into()],
                    env: worktree.shell_env(),
                })
            }
            Pyright::LANGUAGE_SERVER_ID => {
                let pyright = self.pyright.get_or_insert_with(|| Pyright::new());

                Ok(zed::Command {
                    command: pyright.server_script_path(worktree)?,
                    args: vec!["--stdio".into()],
                    env: worktree.shell_env(),
                })
            }
            language_server_id => Err(format!("unknown language server: {language_server_id}")),
        }
    }

    fn label_for_symbol(
        &self,
        language_server_id: &LanguageServerId,
        symbol: Symbol,
    ) -> Option<CodeLabel> {
        match language_server_id.as_ref() {
            BasedPyright::LANGUAGE_SERVER_ID => {
                self.basedpyright.as_ref()?.label_for_symbol(symbol)
            }
            Pyright::LANGUAGE_SERVER_ID => self.pyright.as_ref()?.label_for_symbol(symbol),
            _ => None,
        }
    }

    fn label_for_completion(
        &self,
        language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        match language_server_id.as_ref() {
            BasedPyright::LANGUAGE_SERVER_ID => {
                self.basedpyright.as_ref()?.label_for_completion(completion)
            }
            Pyright::LANGUAGE_SERVER_ID => self.pyright.as_ref()?.label_for_completion(completion),
            _ => None,
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        match language_server_id.as_ref() {
            BasedPyright::LANGUAGE_SERVER_ID => {
                if let Some(basedpyright) = self.basedpyright.as_mut() {
                    return basedpyright.workspace_configuration(worktree);
                }
            }
            Pyright::LANGUAGE_SERVER_ID => {
                if let Some(pyright) = self.pyright.as_mut() {
                    return pyright.workspace_configuration(worktree);
                }
            }
            _ => (),
        }

        Ok(None)
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let initialization_options =
            LspSettings::for_worktree(language_server_id.as_ref(), worktree)
                .ok()
                .and_then(|lsp_settings| lsp_settings.initialization_options.clone())
                .unwrap_or_default();

        Ok(Some(serde_json::json!(initialization_options)))
    }
}

zed::register_extension!(PythonExtension);
