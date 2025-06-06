use std::fs;
use CodeOrbit::LanguageServerId;
use codeorbit_extension_api::settings::LspSettings;
use codeorbit_extension_api::{self as CodeOrbit, Result};

struct TaploBinary {
    path: String,
    args: Option<Vec<String>>,
}

struct TomlExtension {
    cached_binary_path: Option<String>,
}

impl TomlExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &CodeOrbit::Worktree,
    ) -> Result<TaploBinary> {
        let binary_settings = LspSettings::for_worktree("taplo", worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);
        let binary_args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(TaploBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = worktree.which("taplo") {
            return Ok(TaploBinary {
                path,
                args: binary_args,
            });
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(TaploBinary {
                    path: path.clone(),
                    args: binary_args,
                });
            }
        }

        CodeOrbit::set_language_server_installation_status(
            language_server_id,
            &CodeOrbit::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = CodeOrbit::latest_github_release(
            "tamasfe/taplo",
            CodeOrbit::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = CodeOrbit::current_platform();
        let asset_name = format!(
            "taplo-{os}-{arch}.gz",
            arch = match arch {
                CodeOrbit::Architecture::Aarch64 => "aarch64",
                CodeOrbit::Architecture::X86 => "x86",
                CodeOrbit::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                CodeOrbit::Os::Mac => "darwin",
                CodeOrbit::Os::Linux => "linux",
                CodeOrbit::Os::Windows => "windows",
            },
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("taplo-{}", release.version);
        fs::create_dir_all(&version_dir)
            .map_err(|err| format!("failed to create directory '{version_dir}': {err}"))?;

        let binary_path = format!(
            "{version_dir}/{bin_name}",
            bin_name = match platform {
                CodeOrbit::Os::Windows => "taplo.exe",
                CodeOrbit::Os::Mac | CodeOrbit::Os::Linux => "taplo",
            }
        );

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            CodeOrbit::set_language_server_installation_status(
                language_server_id,
                &CodeOrbit::LanguageServerInstallationStatus::Downloading,
            );

            CodeOrbit::download_file(
                &asset.download_url,
                &binary_path,
                CodeOrbit::DownloadedFileType::Gzip,
            )
            .map_err(|err| format!("failed to download file: {err}"))?;

            CodeOrbit::make_file_executable(&binary_path)?;

            let entries = fs::read_dir(".")
                .map_err(|err| format!("failed to list working directory {err}"))?;
            for entry in entries {
                let entry = entry.map_err(|err| format!("failed to load directory entry {err}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(TaploBinary {
            path: binary_path,
            args: binary_args,
        })
    }
}

impl CodeOrbit::Extension for TomlExtension {
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
        let taplo_binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(CodeOrbit::Command {
            command: taplo_binary.path,
            args: taplo_binary
                .args
                .unwrap_or_else(|| vec!["lsp".to_string(), "stdio".to_string()]),
            env: Default::default(),
        })
    }
}

CodeOrbit::register_extension!(TomlExtension);
