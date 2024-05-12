use std::{env, fs};
use zed_extension_api::{self as zed, serde_json, Result};

struct SvelteExtension {
    did_find_server: bool,
}

const SERVER_PATH: &str = "node_modules/svelte-language-server/bin/server.js";
const PACKAGE_NAME: &str = "svelte-language-server";

impl SvelteExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(&mut self, id: &zed::LanguageServerId) -> Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let installed_version = zed::npm_package_installed_version(PACKAGE_NAME).ok().flatten();
        let latest_version = zed::npm_package_latest_version(PACKAGE_NAME);
        let should_reinstall = !server_exists || match (installed_version.as_deref(), latest_version.as_deref().ok()) {
            (Some(installed_version), Some(latest_version)) => installed_version != latest_version,
            (Some(_), None) => false,
            _ => true
        };

        if should_reinstall {
            zed::set_language_server_installation_status(
                id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            let result = zed::npm_install_package(PACKAGE_NAME, &latest_version?);
            if !self.server_exists() {
                return Err(result.err().unwrap_or_else(|| format!(
                    "installed package '{PACKAGE_NAME}' did not contain expected path '{SERVER_PATH}'",
                )));
            }
        }

        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }
}

impl zed::Extension for SvelteExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    fn language_server_command(
        &mut self,
        id: &zed::LanguageServerId,
        _: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(id)?;
        Ok(zed::Command {
            command: zed::node_binary_path()?,
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

    fn language_server_initialization_options(
        &mut self,
        _: &zed::LanguageServerId,
        _: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let config = serde_json::json!({
          "inlayHints": {
            "parameterNames": {
              "enabled": "all",
              "suppressWhenArgumentMatchesName": false
            },
            "parameterTypes": {
              "enabled": true
            },
            "variableTypes": {
              "enabled": true,
              "suppressWhenTypeMatchesName": false
            },
            "propertyDeclarationTypes": {
              "enabled": true
            },
            "functionLikeReturnType": {
              "enabled": true
            },
            "enumMemberValues": {
              "enabled": true
            }
          }
        });

        Ok(Some(serde_json::json!({
            "provideFormatter": true,
            "configuration": {
                "typescript": config,
                "javascript": config
            }
        })))
    }
}

zed::register_extension!(SvelteExtension);
