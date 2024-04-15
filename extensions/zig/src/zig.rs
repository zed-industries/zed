use std::fs;
use zed::LanguageServerId;
use zed_extension_api::{self as zed, Result};

struct ZigExtension {
    cached_binary: Option<ZlsBinary>,
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
        if let Some(zls_binary) = &self.cached_binary {
            if fs::metadata(&zls_binary.path).map_or(false, |stat| stat.is_file()) {
                return Ok(zls_binary.clone());
            }
        }

        if let Some(path) = worktree.which("zls") {
            let environment = worktree.shell_env();
            let zls_binary = ZlsBinary {
                path,
                environment: Some(environment),
            };
            self.cached_binary = Some(zls_binary.clone());
            return Ok(zls_binary);
        }

        zed::set_language_server_installation_status(
            &language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "zigtools/zls",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

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

        let zls_binary = ZlsBinary {
            path: binary_path,
            environment: None,
        };
        self.cached_binary = Some(zls_binary.clone());
        Ok(zls_binary)
    }
}

impl zed::Extension for ZigExtension {
    fn new() -> Self {
        Self {
            cached_binary: None,
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
