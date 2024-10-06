use std::{env, fs};

use zed::{CodeLabel, CodeLabelSpan};
use zed_extension_api::settings::LspSettings;
use zed_extension_api::{self as zed, serde_json, LanguageServerId, Result};

const SERVER_PATH: &str = "node_modules/intelephense/lib/intelephense.js";
const PACKAGE_NAME: &str = "intelephense";

pub struct Intelephense {
    did_find_server: bool,
}

impl Intelephense {
    pub const LANGUAGE_SERVER_ID: &'static str = "intelephense";

    pub fn new() -> Self {
        Self {
            did_find_server: false,
        }
    }

    pub fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if let Some(path) = worktree.which("intelephense") {
            return Ok(zed::Command {
                command: path,
                args: vec!["--stdio".to_string()],
                env: Default::default(),
            });
        }

        let server_path = self.server_script_path(language_server_id)?;
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

    fn server_exists(&self) -> bool {
        fs::metadata(SERVER_PATH).map_or(false, |stat| stat.is_file())
    }

    fn server_script_path(&mut self, language_server_id: &LanguageServerId) -> Result<String> {
        let server_exists = self.server_exists();
        if self.did_find_server && server_exists {
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

        self.did_find_server = true;
        Ok(SERVER_PATH.to_string())
    }

    pub fn language_server_workspace_configuration(
        &mut self,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("intelephense", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(serde_json::json!({
            "intelephense": settings
        })))
    }

    pub fn label_for_completion(&self, completion: zed::lsp::Completion) -> Option<CodeLabel> {
        let label = &completion.label;

        match completion.kind? {
            zed::lsp::CompletionKind::Method => {
                // __construct method doesn't have a detail
                if let Some(ref detail) = completion.detail {
                    if detail.is_empty() {
                        return Some(CodeLabel {
                            spans: vec![
                                CodeLabelSpan::literal(label, Some("function.method".to_string())),
                                CodeLabelSpan::literal("()", None),
                            ],
                            filter_range: (0..label.len()).into(),
                            code: completion.label,
                        });
                    }
                }

                let mut parts = completion.detail.as_ref()?.split(":");
                // E.g., `foo(string $var)`
                let name_and_params = parts.next()?;
                let return_type = parts.next()?.trim();

                let (_, params) = name_and_params.split_once("(")?;
                let params = params.trim_end_matches(")");

                Some(CodeLabel {
                    spans: vec![
                        CodeLabelSpan::literal(label, Some("function.method".to_string())),
                        CodeLabelSpan::literal("(", None),
                        CodeLabelSpan::literal(params, Some("comment".to_string())),
                        CodeLabelSpan::literal("): ", None),
                        CodeLabelSpan::literal(return_type, Some("type".to_string())),
                    ],
                    filter_range: (0..label.len()).into(),
                    code: completion.label,
                })
            }
            zed::lsp::CompletionKind::Constant | zed::lsp::CompletionKind::EnumMember => {
                if let Some(ref detail) = completion.detail {
                    if !detail.is_empty() {
                        return Some(CodeLabel {
                            spans: vec![
                                CodeLabelSpan::literal(label, Some("constant".to_string())),
                                CodeLabelSpan::literal(" ", None),
                                CodeLabelSpan::literal(detail, Some("comment".to_string())),
                            ],
                            filter_range: (0..label.len()).into(),
                            code: completion.label,
                        });
                    }
                }

                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::literal(label, Some("constant".to_string()))],
                    filter_range: (0..label.len()).into(),
                    code: completion.label,
                })
            }
            zed::lsp::CompletionKind::Property => {
                let return_type = completion.detail?;
                Some(CodeLabel {
                    spans: vec![
                        CodeLabelSpan::literal(label, Some("attribute".to_string())),
                        CodeLabelSpan::literal(": ", None),
                        CodeLabelSpan::literal(return_type, Some("type".to_string())),
                    ],
                    filter_range: (0..label.len()).into(),
                    code: completion.label,
                })
            }
            zed::lsp::CompletionKind::Variable => {
                // See https://www.php.net/manual/en/reserved.variables.php
                const SYSTEM_VAR_NAMES: &[&str] =
                    &["argc", "argv", "php_errormsg", "http_response_header"];

                let var_name = completion.label.trim_start_matches("$");
                let is_uppercase = var_name
                    .chars()
                    .filter(|c| c.is_alphabetic())
                    .all(|c| c.is_uppercase());
                let is_system_constant = var_name.starts_with("_");
                let is_reserved = SYSTEM_VAR_NAMES.contains(&var_name);

                let highlight = if is_uppercase || is_system_constant || is_reserved {
                    Some("comment".to_string())
                } else {
                    None
                };

                Some(CodeLabel {
                    spans: vec![CodeLabelSpan::literal(label, highlight)],
                    filter_range: (0..label.len()).into(),
                    code: completion.label,
                })
            }
            _ => None,
        }
    }
}
