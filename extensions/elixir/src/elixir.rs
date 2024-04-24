mod language_servers;

use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

use crate::language_servers::{ElixirLs, Lexical, NextLs};

struct ElixirExtension {
    elixir_ls: Option<ElixirLs>,
    next_ls: Option<NextLs>,
    lexical: Option<Lexical>,
}

impl zed::Extension for ElixirExtension {
    fn new() -> Self {
        Self {
            elixir_ls: None,
            next_ls: None,
            lexical: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            "elixir-ls" => {
                let elixir_ls = self.elixir_ls.get_or_insert_with(|| ElixirLs::new());

                Ok(zed::Command {
                    command: elixir_ls.language_server_binary_path(language_server_id, worktree)?,
                    args: vec![],
                    env: Default::default(),
                })
            }
            "next-ls" => {
                let next_ls = self.next_ls.get_or_insert_with(|| NextLs::new());

                Ok(zed::Command {
                    command: next_ls.language_server_binary_path(language_server_id, worktree)?,
                    args: vec!["--stdio".to_string()],
                    env: Default::default(),
                })
            }
            "lexical" => {
                let lexical = self.lexical.get_or_insert_with(|| Lexical::new());

                Ok(zed::Command {
                    command: lexical.language_server_binary_path(language_server_id, worktree)?,
                    args: vec![],
                    env: Default::default(),
                })
            }
            language_server_id => Err(format!("unknown language server: {language_server_id}")),
        }
    }
}

zed::register_extension!(ElixirExtension);
