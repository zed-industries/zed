use std::{env, fs, path::PathBuf};
use zed_extension_api::{self as zed, Result};

const PACKAGE_NAME: &str = "vscode-language-server";

struct HtmlExtension {
    path: Option<PathBuf>,
}

impl HtmlExtension {
    fn server_script_path(&self, language_server_id: &zed::LanguageServerId) -> Result<PathBuf> {
        if let Some(path) = self.path.as_ref() {
            if fs::metadata(path).map_or(false, |stat| stat.is_dir()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &zed::LanguageServerInstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            "zed-industries/vscode-langservers-extracted",
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;

        let asset_name = "vscode-language-server.tar.gz";

        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("no asset found matching {:?}", asset_name))?;
        let version_dir = format!("{}-{}", PACKAGE_NAME, release.version);
        if !fs::metadata(&version_dir).map_or(false, |stat| stat.is_dir()) {
            zed::set_language_server_installation_status(
                &language_server_id,
                &zed::LanguageServerInstallationStatus::Downloading,
            );

            zed::download_file(
                &asset.download_url,
                &version_dir,
                zed::DownloadedFileType::GzipTar,
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
        Ok(PathBuf::from(version_dir)
            .join("bin")
            .join("vscode-html-language-server"))
    }
}

impl zed::Extension for HtmlExtension {
    fn new() -> Self {
        Self { path: None }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let path = match &self.path {
            Some(path) => path,
            None => {
                let path = self.server_script_path(language_server_id)?;
                self.path = Some(path);
                self.path.as_ref().unwrap()
            }
        };

        Ok(zed::Command {
            command: zed::node_binary_path()?,
            args: vec![
                env::current_dir()
                    .unwrap()
                    .join(path)
                    .to_string_lossy()
                    .to_string(),
                "--stdio".to_string(),
            ],
            env: Default::default(),
        })
    }
}

zed::register_extension!(HtmlExtension);
