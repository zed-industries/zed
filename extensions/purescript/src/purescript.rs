use std::{env, fs};
use zed_extension_api::{self as zed, Result};

const SERVER_PATH: &str = "node_modules/.bin/purescript-language-server";
const SERVER_PATH_WIN: &str = "node_modules/.bin/purescript-language-server.ps1";
const PACKAGE_NAME: &str = "purescript-language-server";

struct PurescriptExtension {
    did_find_server: bool,
    server_path: String,
}

impl PurescriptExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(&self.server_path).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(&mut self, config: &zed::LanguageServerId) -> Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
            return Ok(self.server_path.clone());
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
                            "installed package '{PACKAGE_NAME}' did not contain expected path '{}'",
                            self.server_path,
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
        Ok(self.server_path.clone())
    }
}

impl zed::Extension for PurescriptExtension {
    fn new() -> Self {
        let server_path = match zed::current_platform().0 {
            zed_extension_api::Os::Windows => SERVER_PATH_WIN,
            _ => SERVER_PATH,
        }
        .to_string();
        Self {
            did_find_server: false,
            server_path,
        }
    }

    fn language_server_command(
        &mut self,
        config: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(config)?;
        let command = "powershell.exe".into();
        let env = vec![("PATH".to_string(), zed::node_environment_path()?)];
        println!("==> Env: {:?}", env);
        Ok(zed::Command {
            command,
            args: vec![
                zed_extension_api::current_dir()
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
