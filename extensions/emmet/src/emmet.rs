use std::{env, fs};
use codeorbit_extension_api::{self as CodeOrbit, Result};

struct EmmetExtension {
    did_find_server: bool,
}

const SERVER_PATH: &str = "node_modules/.bin/emmet-language-server";
const PACKAGE_NAME: &str = "@olrtg/emmet-language-server";

impl EmmetExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(&mut self, language_server_id: &CodeOrbit::LanguageServerId) -> Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.to_string());
        }

        CodeOrbit::set_language_server_installation_status(
            language_server_id,
            &CodeOrbit::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = CodeOrbit::npm_package_latest_version(PACKAGE_NAME)?;

        if !server_exists
            || CodeOrbit::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            CodeOrbit::set_language_server_installation_status(
                language_server_id,
                &CodeOrbit::LanguageServerInstallationStatus::Downloading,
            );
            let result = CodeOrbit::npm_install_package(PACKAGE_NAME, &version);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                            "installed package '{PACKAGE_NAME}' did not contain expected path '{SERVER_PATH}'",
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.server_exists() {
                        Err(error)?;
                    }
                }
            }
        }

        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }
}

impl CodeOrbit::Extension for EmmetExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &CodeOrbit::LanguageServerId,
        _worktree: &CodeOrbit::Worktree,
    ) -> Result<CodeOrbit::Command> {
        let server_path = self.server_script_path(language_server_id)?;
        Ok(CodeOrbit::Command {
            command: CodeOrbit::node_binary_path()?,
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
}

CodeOrbit::register_extension!(EmmetExtension);
