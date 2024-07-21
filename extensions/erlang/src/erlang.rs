mod language_servers;

use zed_extension_api::{self as zed, Result};

use crate::language_servers::{ErlangLanguagePlatform, ErlangLs};

struct ErlangExtension {
    erlang_ls: Option<ErlangLs>,
    erlang_language_platform: Option<ErlangLanguagePlatform>,
}

impl zed::Extension for ErlangExtension {
    fn new() -> Self {
        Self {
            erlang_ls: None,
            erlang_language_platform: None,
        }
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
            ErlangLanguagePlatform::LANGUAGE_SERVER_ID => {
                let erlang_language_platform = self
                    .erlang_language_platform
                    .get_or_insert_with(|| ErlangLanguagePlatform::new());
                erlang_language_platform.language_server_command(language_server_id, worktree)
            }
            language_server_id => Err(format!("unknown language server: {language_server_id}")),
        }
    }
}

zed::register_extension!(ErlangExtension);
