use std::collections::HashMap;
use std::{env, fs};

use serde::Deserialize;
use zed_extension_api::{self as zed, serde_json, Result};

const SERVER_PATH: &str = "node_modules/@astrojs/language-server/bin/nodeServer.js";
const SERVER_NAME: &str = "@astrojs/language-server";
const TSDK_PATH: &str = "node_modules/typescript/lib";
const TYPESCRIPT_PATH: &str = "node_modules/typescript/lib/tsserver.js";
const TYPESCRIPT_NAME: &str = "typescript";

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PackageJson {
    #[serde(default)]
    dependencies: HashMap<String, String>,
    #[serde(default)]
    dev_dependencies: HashMap<String, String>,
}

struct AstroExtension {
    did_find_server: bool,
    tsdk_path: String,
}

impl AstroExtension {
    fn file_path_exists(&self, path: &str) -> bool {
        fs::metadata(path).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        let server_exists = self.file_path_exists(SERVER_PATH);

        if self.did_find_server && server_exists {
            self.ensure_typescript(language_server_id, worktree)?;
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let server_version = zed::npm_package_latest_version(SERVER_NAME)?;
        if !server_exists
            || zed::npm_package_installed_version(SERVER_NAME)?.as_ref() != Some(&server_version)
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let server_result = zed::npm_install_package(SERVER_NAME, &server_version);
            match server_result {
                Ok(()) => {
                    if !self.file_path_exists(SERVER_PATH) {
                        Err(format!(
                            "installed package '{SERVER_NAME}' did not contain expected path '{SERVER_PATH}'",
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.file_path_exists(SERVER_PATH) {
                        Err(error)?;
                    }
                }
            }
        }

        self.ensure_typescript(language_server_id, worktree)?;
        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }

    fn worktree_has_typescript(&self, worktree: &zed::Worktree) -> Result<bool> {
        let package_json = worktree.read_text_file("package.json")?;
        let package_json: PackageJson = serde_json::from_str(&package_json)
            .map_err(|err| format!("failed to parse package.json: {err}"))?;

        let dev_dependencies = &package_json.dev_dependencies;
        let dependencies = &package_json.dependencies;

        Ok(dev_dependencies.contains_key(TYPESCRIPT_NAME)
            || dependencies.contains_key(TYPESCRIPT_NAME))
    }

    fn ensure_typescript(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<()> {
        if self.worktree_has_typescript(worktree).unwrap_or_default() {
            return Ok(());
        }
        zed::set_language_server_installation_status(
            &language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let typescript_version = zed::npm_package_latest_version(TYPESCRIPT_NAME)?;
        if zed::npm_package_installed_version(TYPESCRIPT_NAME)?.as_ref()
            != Some(&typescript_version)
        {
            zed::set_language_server_installation_status(
                &language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let server_result = zed::npm_install_package(TYPESCRIPT_NAME, &typescript_version);
            match server_result {
                Ok(()) => {
                    if !self.file_path_exists(TYPESCRIPT_PATH) {
                        Err(format!(
                            "installed package '{TYPESCRIPT_NAME}' did not contain expected path '{TYPESCRIPT_PATH}'",
                        ))?;
                    }
                }
                Err(error) => {
                    if !self.file_path_exists(TYPESCRIPT_PATH) {
                        Err(error)?;
                    }
                }
            }
        }

        self.tsdk_path = env::current_dir()
            .unwrap()
            .join(TSDK_PATH)
            .to_string_lossy()
            .to_string();

        Ok(())
    }
}

impl zed::Extension for AstroExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
            tsdk_path: TSDK_PATH.to_string(),
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = self.server_script_path(language_server_id, worktree)?;
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
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        Ok(Some(serde_json::json!({
            "provideFormatter": true,
            "typescript": {
                "tsdk": self.tsdk_path
            }
        })))
    }
}

zed::register_extension!(AstroExtension);
