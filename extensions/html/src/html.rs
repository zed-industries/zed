use std::{env, fs};
use zed::settings::LspSettings;
use zed_extension_api::{self as zed, LanguageServerId, Result, serde_json::json};

const BINARY_NAME: &str = "vscode-html-language-server";
const SERVER_PATH: &str =
    "node_modules/@zed-industries/vscode-langservers-extracted/bin/vscode-html-language-server";
const WRAPPER_PATH: &str = "node_modules/server-wrapper.js";
const PACKAGE_NAME: &str = "@zed-industries/vscode-langservers-extracted";

struct HtmlExtension {
    cached_binary_path: Option<String>,
}

impl HtmlExtension {
    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).is_ok_and(|stat| stat.is_file())
    }

    fn server_script_path(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        let server_exists = self.server_exists();
        if self.cached_binary_path.is_some() && server_exists {
            return Ok(WRAPPER_PATH.to_string());
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

        self.write_wrapper_script()?;

        Ok(WRAPPER_PATH.to_string())
    }

    fn write_wrapper_script(&self) -> Result<()> {
        // The vscode-html-language-server's built-in JavaScript mode loads TypeScript
        // type definitions (lib.dom.d.ts, lib.es5.d.ts) using a hardcoded relative path
        // that resolves incorrectly for the extracted package. This wrapper patches
        // fs.readFileSync to redirect those reads to the actual TypeScript lib location.
        let wrapper_content = r#"const path = require('path');
const fs = require('fs');
const origReadFileSync = fs.readFileSync;
const tsLibPath = path.dirname(require.resolve('typescript/lib/lib.d.ts'));
const pkgRoot = path.join(
  __dirname,
  '@zed-industries/vscode-langservers-extracted'
);
const brokenBase = path.join(pkgRoot, 'node_modules/typescript/lib');
fs.readFileSync = function(filePath) {
  if (typeof filePath === 'string' && filePath.startsWith(brokenBase)) {
    arguments[0] = filePath.replace(brokenBase, tsLibPath);
  }
  return origReadFileSync.apply(this, arguments);
};
require('./@zed-industries/vscode-langservers-extracted/packages/html/lib/node/htmlServerMain.js');
"#;
        fs::write(WRAPPER_PATH, wrapper_content)
            .map_err(|error| format!("failed to write server wrapper script: {error}"))?;
        Ok(())
    }
}

impl zed::Extension for HtmlExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let server_path = if let Some(path) = worktree.which(BINARY_NAME) {
            return Ok(zed::Command {
                command: path,
                args: vec!["--stdio".to_string()],
                env: Default::default(),
            });
        } else {
            let server_path = self.server_script_path(language_server_id)?;
            env::current_dir()
                .map_err(|error| format!("failed to get current directory: {error}"))?
                .join(&server_path)
                .to_string_lossy()
                .to_string()
        };
        self.cached_binary_path = Some(server_path.clone());

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![server_path, "--stdio".to_string()],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings)
            .unwrap_or_default();
        Ok(Some(settings))
    }

    fn language_server_initialization_options(
        &mut self,
        _server_id: &LanguageServerId,
        _worktree: &zed_extension_api::Worktree,
    ) -> Result<Option<zed_extension_api::serde_json::Value>> {
        let initialization_options = json!({
            "provideFormatter": true,
            "embeddedLanguages": { "css": true, "javascript": true }
        });
        Ok(Some(initialization_options))
    }
}

zed::register_extension!(HtmlExtension);
