mod language_servers;

use zed::lsp::{Completion, Symbol};
use zed::{CodeLabel, LanguageServerId};
use zed_extension_api::{self as zed, Result};

use crate::language_servers::Solargraph;

struct RubyExtension {
    solargraph: Option<Solargraph>,
}

impl zed::Extension for RubyExtension {
    fn new() -> Self {
        Self { solargraph: None }
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
            _ => None,
        }
    }
}

zed::register_extension!(RubyExtension);
