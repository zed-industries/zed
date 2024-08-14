use std::collections::HashMap;
use std::{env, fs};

use serde::Deserialize;
use zed_extension_api::{self as zed, serde_json, Result};

const SERVER_PATH: &str = "node_modules/@astrojs/language-server/bin/nodeServer.js";
const PACKAGE_NAME: &str = "@astrojs/language-server";

const TYPESCRIPT_PACKAGE_NAME: &str = "typescript";

/// The relative path to TypeScript's SDK.
const TYPESCRIPT_TSDK_PATH: &str = "node_modules/typescript/lib";

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
    typescript_tsdk_path: String,
}

impl AstroExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
            self.install_typescript_if_needed(worktree)?;
            return Ok(SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::npm_package_latest_version(PACKAGE_NAME)?;

        if !server_exists
            || zed::npm_package_installed_version(PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                language_server_id,
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

        self.install_typescript_if_needed(worktree)?;
        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }

    /// Returns whether a local copy of TypeScript exists in the worktree.
    fn typescript_exists_for_worktree(&self, worktree: &zed::Worktree) -> Result<bool> {
        let package_json = worktree.read_text_file("package.json")?;
        let package_json: PackageJson = serde_json::from_str(&package_json)
            .map_err(|err| format!("failed to parse package.json: {err}"))?;

        let dev_dependencies = &package_json.dev_dependencies;
        let dependencies = &package_json.dependencies;

        // Since the extension is not allowed to read the filesystem within the project
        // except through the worktree (which does not contains `node_modules`), we check
        // the `package.json` to see if `typescript` is listed in the dependencies.
        Ok(dev_dependencies.contains_key(TYPESCRIPT_PACKAGE_NAME)
            || dependencies.contains_key(TYPESCRIPT_PACKAGE_NAME))
    }

    fn install_typescript_if_needed(&mut self, worktree: &zed::Worktree) -> Result<()> {
        if self
            .typescript_exists_for_worktree(worktree)
            .unwrap_or_default()
        {
            println!("found local TypeScript installation at '{TYPESCRIPT_TSDK_PATH}'");
            return Ok(());
        }

        let installed_typescript_version =
            zed::npm_package_installed_version(TYPESCRIPT_PACKAGE_NAME)?;
        let latest_typescript_version = zed::npm_package_latest_version(TYPESCRIPT_PACKAGE_NAME)?;

        if installed_typescript_version.as_ref() != Some(&latest_typescript_version) {
            println!("installing {TYPESCRIPT_PACKAGE_NAME}@{latest_typescript_version}");
            zed::npm_install_package(TYPESCRIPT_PACKAGE_NAME, &latest_typescript_version)?;
        } else {
            println!("typescript already installed");
        }

        self.typescript_tsdk_path = env::current_dir()
            .unwrap()
            .join(TYPESCRIPT_TSDK_PATH)
            .to_string_lossy()
            .to_string();

        Ok(())
    }
}

impl zed::Extension for AstroExtension {
    fn new() -> Self {
        Self {
            did_find_server: false,
            typescript_tsdk_path: TYPESCRIPT_TSDK_PATH.to_owned(),
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
                "tsdk": self.typescript_tsdk_path
            }
        })))
    }
}

zed::register_extension!(AstroExtension);
