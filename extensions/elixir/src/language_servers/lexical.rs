use std::fs;

use zed::lsp::{Completion, CompletionKind, Symbol};
use zed::{CodeLabel, CodeLabelSpan, LanguageServerId};
use zed_extension_api::{self as zed, Result};

pub struct Lexical {
    cached_binary_path: Option<String>,
}

impl Lexical {
    pub const LANGUAGE_SERVER_ID: &'static str = "lexical";

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
        if let Some(path) = worktree.which("lexical") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            &language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "lexical-lsp/lexical",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset_name = format!("lexical-{version}.zip", version = release.version);

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("lexical-{}", release.version);
        let binary_path = format!("{version_dir}/lexical/bin/start_lexical.sh");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                &language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::Zip,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            let entries =
                fs::read_dir(".").map_err(|e| format!("failed to list working directory {e}"))?;
            for entry in entries {
                let entry = entry.map_err(|e| format!("failed to load directory entry {e}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(&entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    pub fn label_for_completion(&self, completion: Completion) -> Option<CodeLabel> {
        match completion.kind? {
            CompletionKind::Module
            | CompletionKind::Class
            | CompletionKind::Interface
            | CompletionKind::Struct => {
                let name = completion.label;
                let defmodule = "defmodule ";
                let code = format!("{defmodule}{name}");

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(
                        defmodule.len()..defmodule.len() + name.len(),
                    )],
                    filter_range: (0..name.len()).into(),
                })
            }
            CompletionKind::Function | CompletionKind::Constant => {
                let name = completion.label;
                let def = "def ";
                let code = format!("{def}{name}");

                Some(CodeLabel {
                    code,
                    spans: vec![CodeLabelSpan::code_range(def.len()..def.len() + name.len())],
                    filter_range: (0..name.len()).into(),
                })
            }
            CompletionKind::Operator => {
                let name = completion.label;
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

    pub fn label_for_symbol(&self, _symbol: Symbol) -> Option<CodeLabel> {
        None
    }
}
