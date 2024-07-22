use std::{env, fs};
use zed_extension_api::{self as zed, Result};

const SERVER_PATH: &str = "node_modules/.bin/purescript-language-server";
const PACKAGE_NAME: &str = "purescript-language-server";

struct PurescriptExtension {
    did_find_server: bool,
}

impl PurescriptExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(&mut self, config: &zed::LanguageServerId) -> Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            &config,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

        if !server_exists
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                &config,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result = zed::npm_install_package(PACKAGE_NAME, &version);
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

impl zed::Extension for PurescriptExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        config: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(config)?;
        let command = zed::node_binary_path()?;
        let env = Vec::new();
        Ok(zed::Command {
            command,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(&server_path)
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env,
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _config: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let initialization_options = r#"{
            "purescript": {
                "addSpagoSources": true
            }
        }"#;

        Ok(Some(initialization_options.into()))
    }
}

zed::register_extension!(PurescriptExtension);
