use std::{env, fs};
use zed_extension_api::{self as zed, Result};

use super::{utils::install_typescript_if_needed, TYPESCRIPT_TSDK_PATH};

const VTSLS_PACKAGE: &str = "@vtsls/language-server";
const VTSLS_SERVER_PATH: &str = "node_modules/@vtsls/language-server/bin/vtsls.js";
const ASTRO_TS_PLUGIN_PACKAGE: &str = "@astrojs/ts-plugin";

pub struct AstroTypeScriptServer {
    cached_binary_path: Option<String>,
    typescript_tsdk_path: String,
}

impl AstroTypeScriptServer {
    pub const LANGUAGE_SERVER_ID: &'static str = "astro-typescript";

    pub fn new() -> Self {
        Self {
            cached_binary_path: None,
            typescript_tsdk_path: TYPESCRIPT_TSDK_PATH.to_owned(),
        }
    }

    fn server_binary_path(&mut self, language_server_id: &zed::LanguageServerId) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let vtsls_version = zed::npm_package_latest_version(VTSLS_PACKAGE)?;
        if zed::npm_package_installed_version(VTSLS_PACKAGE)?.as_ref() != Some(&vtsls_version) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::npm_install_package(VTSLS_PACKAGE, &vtsls_version)?;
        }

        let plugin_version = zed::npm_package_latest_version(ASTRO_TS_PLUGIN_PACKAGE)?;
        if zed::npm_package_installed_version(ASTRO_TS_PLUGIN_PACKAGE)?.as_ref()
            != Some(&plugin_version)
        {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            zed::npm_install_package(ASTRO_TS_PLUGIN_PACKAGE, &plugin_version)?;
        }

        let server_path = VTSLS_SERVER_PATH.to_string();
        if !fs::metadata(&server_path).map_or(false, |stat| stat.is_file()) {
            return Err(format!("VTSLS server not found at expected path: {}", server_path).into());
        }

        self.typescript_tsdk_path = env::current_dir()
            .unwrap()
            .join(TYPESCRIPT_TSDK_PATH)
            .to_string_lossy()
            .to_string();
        self.cached_binary_path = Some(server_path.clone());
        Ok(server_path)
    }

    pub fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        install_typescript_if_needed(worktree)?;
        let server_path = self.server_binary_path(language_server_id)?;

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

    pub fn typescript_tsdk_path(&self) -> &str {
        &self.typescript_tsdk_path
    }
}
