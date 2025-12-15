use std::fs;

use zed_extension_api::{
    self as zed, Architecture, DownloadedFileType, GithubReleaseOptions, Os, Result,
    settings::LspSettings,
};

use crate::language_servers::util;

pub(crate) struct BufLsp {
    cached_binary_path: Option<String>,
}

impl BufLsp {
    pub(crate) const SERVER_NAME: &str = "buf";

    pub(crate) fn new() -> Self {
        BufLsp {
            cached_binary_path: None,
        }
    }

    pub(crate) fn language_server_binary(
        &mut self,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let binary_settings = LspSettings::for_worktree(Self::SERVER_NAME, worktree)
            .ok()
            .and_then(|lsp_settings| lsp_settings.binary);

        let args = binary_settings
            .as_ref()
            .and_then(|binary_settings| binary_settings.arguments.clone())
            .unwrap_or_else(|| ["lsp", "serve"].map(ToOwned::to_owned).into());

        if let Some(path) = binary_settings.and_then(|binary_settings| binary_settings.path) {
            return Ok(zed::Command {
                command: path,
                args,
                env: Default::default(),
            });
        } else if let Some(path) = self.cached_binary_path.clone() {
            return Ok(zed::Command {
                command: path,
                args,
                env: Default::default(),
            });
        } else if let Some(path) = worktree.which(Self::SERVER_NAME) {
            self.cached_binary_path = Some(path.clone());
            return Ok(zed::Command {
                command: path,
                args,
                env: Default::default(),
            });
        }

        let latest_release = zed::latest_github_release(
            "bufbuild/buf",
            GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let (os, arch) = zed::current_platform();

        let release_suffix = match (os, arch) {
            (Os::Mac, Architecture::Aarch64) => "Darwin-arm64",
            (Os::Mac, Architecture::X8664) => "Darwin-x86_64",
            (Os::Linux, Architecture::Aarch64) => "Linux-aarch64",
            (Os::Linux, Architecture::X8664) => "Linux-x86_64",
            (Os::Windows, Architecture::Aarch64) => "Windows-arm64.exe",
            (Os::Windows, Architecture::X8664) => "Windows-x86_64.exe",
            _ => {
                return Err("Platform and architecture not supported by buf CLI".to_string());
            }
        };

        let release_name = format!("buf-{release_suffix}");

        let version_dir = format!("{}-{}", Self::SERVER_NAME, latest_release.version);
        fs::create_dir_all(&version_dir).map_err(|_| "Could not create directory")?;

        let binary_path = format!("{version_dir}/buf");

        let download_target = latest_release
            .assets
            .into_iter()
            .find(|asset| asset.name == release_name)
            .ok_or_else(|| {
                format!(
                    "Could not find asset with name {} in buf CLI release",
                    &release_name
                )
            })?;

        zed::download_file(
            &download_target.download_url,
            &binary_path,
            DownloadedFileType::Uncompressed,
        )?;
        zed::make_file_executable(&binary_path)?;

        util::remove_outdated_versions(Self::SERVER_NAME, &version_dir)?;

        self.cached_binary_path = Some(binary_path.clone());

        Ok(zed::Command {
            command: binary_path,
            args,
            env: Default::default(),
        })
    }
}
