use std::fs;

use zed::lsp::{Completion, Symbol};
use zed::{CodeLabel, LanguageServerId};
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
            &language_server_id,
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
        let binary_path = format!(
            "{version_dir}/language_server.{extension}",
            extension = match platform {
                zed::Os::Mac | zed::Os::Linux => "sh",
                zed::Os::Windows => "bat",
            }
        );

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

    pub fn label_for_completion(&self, _completion: Completion) -> Option<CodeLabel> {
        None
    }

    pub fn label_for_symbol(&self, _symbol: Symbol) -> Option<CodeLabel> {
        None
    }
}
