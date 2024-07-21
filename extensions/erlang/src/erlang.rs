mod language_servers;

use zed_extension_api::{self as zed, Result};

use crate::language_servers::ErlangLs;

struct ErlangExtension {
    erlang_ls: Option<ErlangLs>,
}

impl zed::Extension for ErlangExtension {
    fn new() -> Self {
        Self { erlang_ls: None }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            ErlangLs::LANGUAGE_SERVER_ID => {
                let erlang_ls = self.erlang_ls.get_or_insert_with(|| ErlangLs::new());

                Ok(zed::Command {
                    command: erlang_ls.language_server_binary_path(language_server_id, worktree)?,
                    args: vec![],
                    env: Default::default(),
                })
            }
            language_server_id => Err(format!("unknown language server: {language_server_id}")),
        }
    }
}

zed::register_extension!(ErlangExtension);
