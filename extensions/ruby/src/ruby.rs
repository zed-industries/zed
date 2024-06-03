mod language_servers;

use zed::lsp::{Completion, Symbol};
use zed::settings::LspSettings;
use zed::{serde_json, CodeLabel, LanguageServerId};
use zed_extension_api::{self as zed, Result};

use crate::language_servers::{RubyLsp, Solargraph};

struct RubyExtension {
    solargraph: Option<Solargraph>,
    ruby_lsp: Option<RubyLsp>,
}

impl zed::Extension for RubyExtension {
    fn new() -> Self {
        Self {
            solargraph: None,
            ruby_lsp: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            Solargraph::LANGUAGE_SERVER_ID => {
                let solargraph = self.solargraph.get_or_insert_with(|| Solargraph::new());

                Ok(zed::Command {
                    command: solargraph.server_script_path(worktree)?,
                    args: vec!["stdio".into()],
                    env: worktree.shell_env(),
                })
            }
            RubyLsp::LANGUAGE_SERVER_ID => {
                let ruby_lsp = self.ruby_lsp.get_or_insert_with(|| RubyLsp::new());

                Ok(zed::Command {
                    command: ruby_lsp.server_script_path(worktree)?,
                    args: vec![],
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
            Solargraph::LANGUAGE_SERVER_ID => self.solargraph.as_ref()?.label_for_symbol(symbol),
            RubyLsp::LANGUAGE_SERVER_ID => self.ruby_lsp.as_ref()?.label_for_symbol(symbol),
            _ => None,
        }
    }

    fn label_for_completion(
        &self,
        language_server_id: &LanguageServerId,
        completion: Completion,
    ) -> Option<CodeLabel> {
        match language_server_id.as_ref() {
            Solargraph::LANGUAGE_SERVER_ID => {
                self.solargraph.as_ref()?.label_for_completion(completion)
            }
            RubyLsp::LANGUAGE_SERVER_ID => self.ruby_lsp.as_ref()?.label_for_completion(completion),
            _ => None,
        }
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

zed::register_extension!(RubyExtension);
