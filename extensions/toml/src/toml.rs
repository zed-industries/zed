use std::fs;
use zed_extension_api::{self as zed, Result};

struct TomlExtension {
    cached_binary_path: Option<String>,
}

impl TomlExtension {
    fn language_server_binary_path(&mut self, config: zed::LanguageServerConfig) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).map_or(false, |stat| stat.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            &config.name,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "tamasfe/taplo",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (platform, arch) = zed::current_platform();
        let asset_name = format!(
            "taplo-full-{os}-{arch}.{extension}",
            arch = match arch {
                zed::Architecture::Aarch64 => "aarch64",
                zed::Architecture::X86 => "x86",
                zed::Architecture::X8664 => "x86_64",
            },
            os = match platform {
                zed::Os::Mac => "darwin",
                zed::Os::Linux => "linux",
                zed::Os::Windows => "windows",
            },
            extension = match platform {
                zed::Os::Mac | zed::Os::Linux => "gz",
                zed::Os::Windows => "zip",
            }
        );

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

        let version_dir = format!("taplo-{}", release.version);
        fs::create_dir_all(&version_dir)
            .map_err(|err| format!("failed to create directory '{version_dir}': {err}"))?;

        let binary_path = format!("{version_dir}/taplo");

        if !fs::metadata(&binary_path).map_or(false, |stat| stat.is_file()) {
            zed::set_language_server_installation_status(
                &config.name,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &binary_path,
                match platform {
                    zed::Os::Mac | zed::Os::Linux => zed::DownloadedFileType::Gzip,
                    zed::Os::Windows => zed::DownloadedFileType::Zip,
                },
            )
            .map_err(|err| format!("failed to download file: {err}"))?;

            let entries = fs::read_dir(".")
                .map_err(|err| format!("failed to list working directory {err}"))?;
            for entry in entries {
                let entry = entry.map_err(|err| format!("failed to load directory entry {err}"))?;
                if entry.file_name().to_str() != Some(&version_dir) {
                    fs::remove_dir_all(&entry.path()).ok();
                }
            }
        }

        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for TomlExtension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        config: zed::LanguageServerConfig,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        Ok(zed::Command {
            command: self.language_server_binary_path(config)?,
            args: vec!["lsp".to_string(), "stdio".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(TomlExtension);
