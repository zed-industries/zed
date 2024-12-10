use std::{env, fs};

use zed_extension_api::{self as zed, Result};

use super::{utils::install_typescript_if_needed, TYPESCRIPT_TSDK_PATH};

const ASTRO_SERVER_PATH: &str = "node_modules/@astrojs/language-server/bin/nodeServer.js";
const ASTRO_PACKAGE_NAME: &str = "@astrojs/language-server";

pub struct AstroLanguageServer {
    did_find_server: bool,
    typescript_tsdk_path: String,
}

impl AstroLanguageServer {
    pub const LANGUAGE_SERVER_ID: &'static str = "astro-language-server";

    pub fn new() -> Self {
        Self {
            did_find_server: false,
            typescript_tsdk_path: TYPESCRIPT_TSDK_PATH.to_owned(),
        }
    }

    fn server_exists(&self) -> bool {
        fs::metadata(ASTRO_SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    pub fn language_server_command(
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

    fn server_script_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
            install_typescript_if_needed(worktree)?;
            return Ok(ASTRO_SERVER_PATH.to_string());
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let version = zed::npm_package_latest_version(ASTRO_PACKAGE_NAME)?;

        if !server_exists
            || zed::npm_package_installed_version(ASTRO_PACKAGE_NAME)?.as_ref() != Some(&version)
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let result = zed::npm_install_package(ASTRO_PACKAGE_NAME, &version);
            match result {
                Ok(()) => {
                    if !self.server_exists() {
                        Err(format!(
                            "installed package '{ASTRO_PACKAGE_NAME}' did not contain expected path '{ASTRO_SERVER_PATH}'",
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

        install_typescript_if_needed(worktree)?;
        self.typescript_tsdk_path = env::current_dir()
            .unwrap()
            .join(TYPESCRIPT_TSDK_PATH)
            .to_string_lossy()
            .to_string();
        self.did_find_server = true;
        Ok(ASTRO_SERVER_PATH.to_string())
    }

    pub fn typescript_tsdk_path(&self) -> &str {
        &self.typescript_tsdk_path
    }
}
