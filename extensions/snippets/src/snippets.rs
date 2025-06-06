use serde_json::json;
use std::fs;
use CodeOrbit::LanguageServerId;
use codeorbit_extension_api::{self as CodeOrbit, Result, settings::LspSettings};

struct SnippetExtension {
    cached_binary_path: Option<String>,
}

impl SnippetExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &CodeOrbit::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("simple-completion-language-server") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        CodeOrbit::set_language_server_installation_status(
            language_server_id,
            &CodeOrbit::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = CodeOrbit::latest_github_release(
            "CodeOrbit-industries/simple-completion-language-server",
            CodeOrbit::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = CodeOrbit::current_platform();
        let asset_name = format!(
            "simple-completion-language-server-{arch}-{os}.tar.gz",
            arch = match arch {
                CodeOrbit::Architecture::Aarch64 => "aarch64",
                CodeOrbit::Architecture::X86 => "x86",
                CodeOrbit::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                CodeOrbit::Os::Mac => "apple-darwin",
                CodeOrbit::Os::Linux => "unknown-linux-gnu",
                CodeOrbit::Os::Windows => "pc-windows-msvc",
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("simple-completion-language-server-{}", release.version);
        let binary_path = format!("{version_dir}/simple-completion-language-server");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            CodeOrbit::set_language_server_installation_status(
                language_server_id,
                &CodeOrbit::LanguageServerInstallationStatus::Downloading,
            );

            CodeOrbit::download_file(
                &asset.download_url,
                &version_dir,
                CodeOrbit::DownloadedFileType::GzipTar,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

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
}

impl CodeOrbit::Extension for SnippetExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &CodeOrbit::Worktree,
    ) -> Result<CodeOrbit::Command> {
        Ok(CodeOrbit::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec![],
            env: vec![("SCLS_CONFIG_SUBDIRECTORY".to_owned(), "CodeOrbit".to_owned())],
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        server_id: &LanguageServerId,
        worktree: &codeorbit_extension_api::Worktree,
    ) -> Result<Option<codeorbit_extension_api::serde_json::Value>> {
        let settings = LspSettings::for_worktree(server_id.as_ref(), worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_else(|| {
                json!({
                    "max_completion_items": 20,
                    "snippets_first": true,
                    "feature_words": false,
                    "feature_snippets": true,
                    "feature_paths": true
                })
            });
        Ok(Some(settings))
    }
}

CodeOrbit::register_extension!(SnippetExtension);
