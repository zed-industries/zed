use zed_extension_api::{self as zed, Result};

struct GleamExtension {
    cached_binary_path: Option<String>,
}

impl zed::Extension for GleamExtension {
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
        let binary_path = if let Some(path) = &self.cached_binary_path {
            zed::set_language_server_installation_status(
                &config.name,
                &zed::LanguageServerInstallationStatus::Cached,
            );

            path.clone()
        } else {
            zed::set_language_server_installation_status(
                &config.name,
                &zed::LanguageServerInstallationStatus::CheckingForUpdate,
            );
            let release = zed::latest_github_release(
                "gleam-lang/gleam",
                zed::GithubReleaseOptions {
                    require_assets: true,
                    pre_release: false,
                },
            )?;

            let (platform, arch) = zed::current_platform();
            let asset_name = format!(
                "gleam-{version}-{arch}-{os}.tar.gz",
                version = release.version,
                arch = match arch {
                    zed::Architecture::Aarch64 => "aarch64",
                    zed::Architecture::X86 => "x86",
                    zed::Architecture::X8664 => "x86_64",
                },
                os = match platform {
                    zed::Os::Mac => "apple-darwin",
                    zed::Os::Linux => "unknown-linux-musl",
                    zed::Os::Windows => "pc-windows-msvc",
                },
            );

            let asset = release
                .assets
                .iter()
                .find(|asset| asset.name == asset_name)
                .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;

            zed::set_language_server_installation_status(
                &config.name,
                &zed::LanguageServerInstallationStatus::Downloading,
            );
            let version_dir = format!("gleam-{}", release.version);
            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::GzipTar,
            )
            .map_err(|e| format!("failed to download file: {e}"))?;

            zed::set_language_server_installation_status(
                &config.name,
                &zed::LanguageServerInstallationStatus::Downloaded,
            );

            let binary_path = format!("{version_dir}/gleam");
            self.cached_binary_path = Some(binary_path.clone());
            binary_path
        };

        Ok(zed::Command {
            command: binary_path,
            args: vec!["lsp".to_string()],
            env: Default::default(),
        })
    }
}

zed::register_extension!(GleamExtension);
