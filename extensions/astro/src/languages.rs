mod language_servers;

use std::env;

use crate::language_servers::{AstroLanguageServer, AstroTypeScriptServer};
use language_servers::merge_json_value_into;
use zed_extension_api::{self as zed, serde_json, settings::LspSettings, LanguageServerId, Result};

pub struct AstroExtension {
    astro: Option<AstroLanguageServer>,
    typescript: Option<AstroTypeScriptServer>,
}

impl zed::Extension for AstroExtension {
    fn new() -> Self {
        Self {
            astro: None,
            typescript: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        match language_server_id.as_ref() {
            AstroLanguageServer::LANGUAGE_SERVER_ID => {
                let server = self.astro.get_or_insert_with(AstroLanguageServer::new);
                server.language_server_command(language_server_id, worktree)
            }
            AstroTypeScriptServer::LANGUAGE_SERVER_ID => {
                let server = self
                    .typescript
                    .get_or_insert_with(AstroTypeScriptServer::new);
                server.language_server_command(language_server_id, worktree)
            }
            id => Err(format!("Unknown Language Server: {id}")),
        }
    }
    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        match language_server_id.as_ref() {
            AstroLanguageServer::LANGUAGE_SERVER_ID => {
                let server = self.astro.as_ref().unwrap();
                Ok(Some(serde_json::json!({
                    "provideFormatter": true,
                    "typescript": {
                        "tsdk": server.typescript_tsdk_path()
                    }
                })))
            }
            AstroTypeScriptServer::LANGUAGE_SERVER_ID => {
                let server = self.typescript.as_ref().unwrap();
                Ok(Some(serde_json::json!({
                    "provideFormatter": true,
                    "typescript": {
                        "tsdk": server.typescript_tsdk_path()
                    }
                })))
            }
            _ => Ok(None),
        }
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree(language_server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();
        match language_server_id.as_ref() {
            AstroLanguageServer::LANGUAGE_SERVER_ID => Ok(Some(settings)),
            AstroTypeScriptServer::LANGUAGE_SERVER_ID => {
                let server = self.typescript.as_ref().unwrap();
                let current_dir = env::current_dir().unwrap().to_string_lossy().to_string();
                let config = serde_json::json!({
                    "tsdk": server.typescript_tsdk_path(),
                    "suggest": {
                        "completeFunctionCalls": true
                    },
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
                        "functionLikeReturnTypes": {
                            "enabled": true
                        },
                        "enumMemberValues": {
                            "enabled": true
                        }
                    },
                    "tsserver": {
                        "maxTsServerMemory": 8092
                    },
                });

                let mut workspace_config = serde_json::json!({
                    "typescript": config,
                    "javascript": config,
                    "vtsls": {
                        "experimental": {
                            "completion": {
                                "enableServerSideFuzzyMatch": true,
                                "entriesLimit": 5000,
                            }
                        },
                        "autoUseWorkspaceTsdk": true,
                        "tsserver": {
                            "globalPlugins": [{
                                "name": "@astrojs/ts-plugin",
                                "location": current_dir,
                                "enableForWorkspaceTypeScriptVersions": true
                            }]
                        }
                    },
                });

                if !settings.is_null() {
                    merge_json_value_into(settings, &mut workspace_config);
                }

                Ok(Some(workspace_config))
            }
            _ => Ok(None),
        }
    }
}

zed::register_extension!(AstroExtension);
