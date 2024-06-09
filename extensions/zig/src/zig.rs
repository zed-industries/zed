use std::fs;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

struct ZigExtension {
    cached_binary_path: Option<String>,
}

#[derive(Clone)]
struct ZlsBinary {
    path: String,
    environment: Option<Vec<(String, String)>>,
}

impl ZigExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<ZlsBinary> {
        if let Some(path) = worktree.which("zls") {
            let environment = worktree.shell_env();
            return Ok(ZlsBinary {
                path,
                environment: Some(environment),
            });
        }

        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(&path).map_or(false, |stat| stat.is_file()) {
                return Ok(ZlsBinary {
                    path: path.clone(),
                    environment: None,
                });
            }
        }

        zed::set_language_server_installation_status(
            &language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        // We're pinning ZLS to a release that has `.tar.gz` assets, since the latest release does not have
        // them, at time of writing.
        //
        // ZLS tracking issue: https://github.com/zigtools/zls/issues/1879
        let release = zed::github_release_by_tag_name("zigtools/zls", "0.11.0")?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "zls-{arch}-{os}.{extension}",
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X86 => "x86",
                zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "macos",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "windows",
            },
            extension = match platform {
                zed::Os::Mac | zed::Os::Linux => "tar.gz",
                zed::Os::Windows => "zip",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("zls-{}", release.version);
        let binary_path = format!("{version_dir}/bin/zls");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                &language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                match platform {
                    zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::GzipTar,
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                },
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::make_file_executable(&binary_path)?;

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
        Ok(ZlsBinary {
            path: binary_path,
            environment: None,
        })
    }
}

impl zed::Extension for ZigExtension {
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
        let zls_binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: zls_binary.path,
            args: vec![],
            env: zls_binary.environment.unwrap_or_default(),
        })
    }
}

zed::register_extension!(ZigExtension);
