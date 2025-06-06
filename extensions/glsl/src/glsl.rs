use std::fs;
use CodeOrbit::settings::LspSettings;
use codeorbit_extension_api::{self as CodeOrbit, LanguageServerId, Result, serde_json};

struct GlslExtension {
    cached_binary_path: Option<String>,
}

impl GlslExtension {
    fn language_server_binary_path(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &CodeOrbit::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("glsl_analyzer") {
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
            "nolanderc/glsl_analyzer",
            CodeOrbit::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = CodeOrbit::current_platform();
        let asset_name = format!(
            "{arch}-{os}.zip",
            arch = match arch {
                CodeOrbit::Architecture::Aarch64 => "aarch64",
                CodeOrbit::Architecture::X86 => "x86",
                CodeOrbit::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                CodeOrbit::Os::Mac => "macos",
                CodeOrbit::Os::Linux => "linux-musl",
                CodeOrbit::Os::Windows => "windows",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("glsl_analyzer-{}", release.version);
        fs::create_dir_all(&version_dir)
            .map_err(|err| format!("failed to create directory '{version_dir}': {err}"))?;
        let binary_path = format!("{version_dir}/bin/glsl_analyzer");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            CodeOrbit::set_language_server_installation_status(
                language_server_id,
                &CodeOrbit::LanguageServerInstallationStatus::Downloading,
            );

            CodeOrbit::download_file(
                &asset.download_url,
                &version_dir,
                match platform {
                    CodeOrbit::Os::Mac | CodeOrbit::Os::Linux => CodeOrbit::DownloadedFileType::Zip,
                    CodeOrbit::Os::Windows => CodeOrbit::DownloadedFileType::Zip,
                },
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            CodeOrbit::make_file_executable(&binary_path)?;

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

impl CodeOrbit::Extension for GlslExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &CodeOrbit::LanguageServerId,
        worktree: &CodeOrbit::Worktree,
    ) -> Result<CodeOrbit::Command> {
        Ok(CodeOrbit::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args: vec![],
            env: Default::default(),
        })
    }

    fn language_server_workspace_configuration(
        &mut self,
        _language_server_id: &CodeOrbit::LanguageServerId,
        worktree: &CodeOrbit::Worktree,
    ) -> Result<Option<serde_json::Value>> {
        let settings = LspSettings::for_worktree("glsl_analyzer", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.settings.clone())
            .unwrap_or_default();

        Ok(Some(serde_json::json!({
            "glsl_analyzer": settings
        })))
    }
}

CodeOrbit::register_extension!(GlslExtension);
