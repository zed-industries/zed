use std::fs;

use zed::lsp::{Completion, CompletionKind, Symbol, SymbolKind};
use zed::settings::LspSettings;
use zed::{serde_json, CodeLabel, CodeLabelSpan, LanguageServerId};
use zed_extension_api::{self as zed, Result};

pub struct ElixirLs {
    cached_binary_path: Option<String>,
}

impl ElixirLs {
    pub const LANGUAGE_SERVER_ID: &'static str = "elixir-ls";

    pub fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    pub fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("elixir-ls") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "elixir-lsp/elixir-ls",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset_name = format!("elixir-ls-{version}.zip", version = release.version,);

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let (platform, _arch) = zed::current_platform();
        let version_dir = format!("elixir-ls-{}", release.version);
        let extension = match platform {
            zed::Os::Mac | zed::Os::Linux => "sh",
            zed::Os::Windows => "bat",
        };
        let binary_path = format!("{version_dir}/language_server.{extension}");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;
            zed::make_file_executable(&format!("{version_dir}/launch.{extension}"))?;
            zed::make_file_executable(&format!("{version_dir}/debug_adapter.{extension}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    pub fn language_server_workspace_configuration(
        &mut self,
        worktree: &zed::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("elixir-ls", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(serde_json::json!({
            "elixirLS": settings
        })))
    }

    pub fn label_for_completion(&self, completion: Completion) -> Option<CodeLabel> {
        let name = &completion.label;
        let detail = completion
            .detail
            .filter(|detail| detail != "alias")
            .map(|detail| format!(": {detail}"))
            .unwrap_or("".to_string());

        let detail_span = CodeLabelSpan::literal(detail, Some("comment.unused".to_string()));

        match completion.kind? {
            CompletionKind::Module | CompletionKind::Class | CompletionKind::Struct => {
                let defmodule = "defmodule ";
                let alias = completion
                    .label_details
                    .and_then(|details| details.description)
                    .filter(|description| description.starts_with("alias"))
                    .map(|description| format!(" ({description})"))
                    .unwrap_or("".to_string());

                let code = format!("{defmodule}{name}{alias}");
                let name_start = defmodule.len();
                let name_end = name_start + name.len();

                Some(CodeLabel {
                    code,
                    spans: vec![
                        CodeLabelSpan::code_range(name_start..name_end),
                        detail_span,
                        CodeLabelSpan::code_range(name_end..(name_end + alias.len())),
                    ],
                    filter_range: (0..name.len()).into(),
                })
            }
            CompletionKind::Interface => Some(CodeLabel {
                code: name.to_string(),
                spans: vec![CodeLabelSpan::code_range(0..name.len()), detail_span],
                filter_range: (0..name.len()).into(),
            }),
            CompletionKind::Field => Some(CodeLabel {
                code: name.to_string(),
                spans: vec![
                    CodeLabelSpan::literal(name, Some("function".to_string())),
                    detail_span,
                ],
                filter_range: (0..name.len()).into(),
            }),
            CompletionKind::Function | CompletionKind::Constant => {
                let detail = completion
                    .label_details
                    .clone()
                    .and_then(|details| details.detail)
                    .unwrap_or("".to_string());

                let description = completion
                    .label_details
                    .clone()
                    .and_then(|details| details.description)
                    .map(|description| format!(" ({description})"))
                    .unwrap_or("".to_string());

                let def = "def ";
                let code = format!("{def}{name}{detail}{description}");

                let name_start = def.len();
                let name_end = name_start + name.len();
                let detail_end = name_end + detail.len();
                let description_end = detail_end + description.len();

                Some(CodeLabel {
                    code,
                    spans: vec![
                        CodeLabelSpan::code_range(name_start..name_end),
                        CodeLabelSpan::code_range(name_end..detail_end),
                        CodeLabelSpan::code_range(detail_end..description_end),
                    ],
                    filter_range: (0..name.len()).into(),
                })
            }
            CompletionKind::Operator => {
                let def_a = "def a ";
                let code = format!("{def_a}{name} b");

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(
                        def_a.len()..def_a.len() + name.len(),
                    )],
                    filter_range: (0..name.len()).into(),
                })
            }
            _ => None,
        }
    }

    pub fn label_for_symbol(&self, symbol: Symbol) -> Option<CodeLabel> {
        let name = &symbol.name;

        let (code, filter_range, display_range) = match symbol.kind {
            SymbolKind::Module | SymbolKind::Class | SymbolKind::Interface | SymbolKind::Struct => {
                let defmodule = "defmodule ";
                let code = format!("{defmodule}{name}");
                let filter_range = 0..name.len();
                let display_range = defmodule.len()..defmodule.len() + name.len();
                (code, filter_range, display_range)
            }
            SymbolKind::Function | SymbolKind::Constant => {
                let def = "def ";
                let code = format!("{def}{name}");
                let filter_range = 0..name.len();
                let display_range = def.len()..def.len() + name.len();
                (code, filter_range, display_range)
            }
            _ => return None,
        };

        Some(CodeLabel {
            spans: vec![CodeLabelSpan::code_range(display_range)],
            filter_range: filter_range.into(),
            code,
        })
    }
}
