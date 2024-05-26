use crate::wasm_host::{wit::ToWasmtimeResult, WasmState};
use ::settings::Settings;
use anyhow::{anyhow, bail, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use futures::{io::BufReader, FutureExt as _};
use language::{
    language_settings::AllLanguageSettings, LanguageServerBinaryStatus, LspAdapterDelegate,
};
use project::project_settings::ProjectSettings;
use semantic_version::SemanticVersion;
use std::{
    env,
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};
use util::maybe;
use wasmtime::component::{Linker, Resource};

pub const MIN_VERSION: SemanticVersion = SemanticVersion::new(0, 0, 7);
pub const MAX_VERSION: SemanticVersion = SemanticVersion::new(0, 0, 7);

wasmtime::component::bindgen!({
    async: true,
    path: "../extension_api/wit/since_v0.0.7",
    with: {
         "worktree": ExtensionWorktree,
    },
});

pub use self::zed::extension::*;

mod settings {
    include!("../../../../extension_api/wit/since_v0.0.7/settings.rs");
}

pub type ExtensionWorktree = Arc<dyn LspAdapterDelegate>;

pub fn linker() -> &'static Linker<WasmState> {
    static LINKER: OnceLock<Linker<WasmState>> = OnceLock::new();
    LINKER.get_or_init(|| super::new_linker(Extension::add_to_linker))
}

#[async_trait]
impl HostWorktree for WasmState {
    async fn id(
        &mut self,
        delegate: Resource<Arc<dyn LspAdapterDelegate>>,
    ) -> wasmtime::Result<u64> {
        let delegate = self.table.get(&delegate)?;
        Ok(delegate.worktree_id())
    }

    async fn root_path(
        &mut self,
        delegate: Resource<Arc<dyn LspAdapterDelegate>>,
    ) -> wasmtime::Result<String> {
        let delegate = self.table.get(&delegate)?;
        Ok(delegate.worktree_root_path().to_string_lossy().to_string())
    }

    async fn read_text_file(
        &mut self,
        delegate: Resource<Arc<dyn LspAdapterDelegate>>,
        path: String,
    ) -> wasmtime::Result<Result<String, String>> {
        let delegate = self.table.get(&delegate)?;
        Ok(delegate
            .read_text_file(path.into())
            .await
            .map_err(|error| error.to_string()))
    }

    async fn shell_env(
        &mut self,
        delegate: Resource<Arc<dyn LspAdapterDelegate>>,
    ) -> wasmtime::Result<EnvVars> {
        let delegate = self.table.get(&delegate)?;
        Ok(delegate.shell_env().await.into_iter().collect())
    }

    async fn which(
        &mut self,
        delegate: Resource<Arc<dyn LspAdapterDelegate>>,
        binary_name: String,
    ) -> wasmtime::Result<Option<String>> {
        let delegate = self.table.get(&delegate)?;
        Ok(delegate
            .which(binary_name.as_ref())
            .await
            .map(|path| path.to_string_lossy().to_string()))
    }

    fn drop(&mut self, _worktree: Resource<Worktree>) -> Result<()> {
        // We only ever hand out borrows of worktrees.
        Ok(())
    }
}

#[async_trait]
impl nodejs::Host for WasmState {
    async fn node_binary_path(&mut self) -> wasmtime::Result<Result<String, String>> {
        self.host
            .node_runtime
            .binary_path()
            .await
            .map(|path| path.to_string_lossy().to_string())
            .to_wasmtime_result()
    }

    async fn npm_package_latest_version(
        &mut self,
        package_name: String,
    ) -> wasmtime::Result<Result<String, String>> {
        self.host
            .node_runtime
            .npm_package_latest_version(&package_name)
            .await
            .to_wasmtime_result()
    }

    async fn npm_package_installed_version(
        &mut self,
        package_name: String,
    ) -> wasmtime::Result<Result<Option<String>, String>> {
        self.host
            .node_runtime
            .npm_package_installed_version(&self.work_dir(), &package_name)
            .await
            .to_wasmtime_result()
    }

    async fn npm_install_package(
        &mut self,
        package_name: String,
        version: String,
    ) -> wasmtime::Result<Result<(), String>> {
        self.host
            .node_runtime
            .npm_install_packages(&self.work_dir(), &[(&package_name, &version)])
            .await
            .to_wasmtime_result()
    }
}

#[async_trait]
impl lsp::Host for WasmState {}

impl From<http::github::GithubRelease> for github::GithubRelease {
    fn from(value: http::github::GithubRelease) -> Self {
        Self {
            version: value.tag_name,
            assets: value.assets.into_iter().map(Into::into).collect(),
        }
    }
}

impl From<http::github::GithubReleaseAsset> for github::GithubReleaseAsset {
    fn from(value: http::github::GithubReleaseAsset) -> Self {
        Self {
            name: value.name,
            download_url: value.browser_download_url,
        }
    }
}

#[async_trait]
impl github::Host for WasmState {
    async fn latest_github_release(
        &mut self,
        repo: String,
        options: github::GithubReleaseOptions,
    ) -> wasmtime::Result<Result<github::GithubRelease, String>> {
        maybe!(async {
            let release = http::github::latest_github_release(
                &repo,
                options.require_assets,
                options.pre_release,
                self.host.http_client.clone(),
            )
            .await?;
            Ok(release.into())
        })
        .await
        .to_wasmtime_result()
    }

    async fn github_release_by_tag_name(
        &mut self,
        repo: String,
        tag: String,
    ) -> wasmtime::Result<Result<github::GithubRelease, String>> {
        maybe!(async {
            let release =
                http::github::get_release_by_tag_name(&repo, &tag, self.host.http_client.clone())
                    .await?;
            Ok(release.into())
        })
        .await
        .to_wasmtime_result()
    }
}

#[async_trait]
impl platform::Host for WasmState {
    async fn current_platform(&mut self) -> Result<(platform::Os, platform::Architecture)> {
        Ok((
            match env::consts::OS {
                "macos" => platform::Os::Mac,
                "linux" => platform::Os::Linux,
                "windows" => platform::Os::Windows,
                _ => panic!("unsupported os"),
            },
            match env::consts::ARCH {
                "aarch64" => platform::Architecture::Aarch64,
                "x86" => platform::Architecture::X86,
                "x86_64" => platform::Architecture::X8664,
                _ => panic!("unsupported architecture"),
            },
        ))
    }
}

#[async_trait]
impl slash_command::Host for WasmState {}

#[async_trait]
impl ExtensionImports for WasmState {
    async fn get_settings(
        &mut self,
        location: Option<self::SettingsLocation>,
        category: String,
        key: Option<String>,
    ) -> wasmtime::Result<Result<String, String>> {
        self.on_main_thread(|cx| {
            async move {
                let location = location
                    .as_ref()
                    .map(|location| ::settings::SettingsLocation {
                        worktree_id: location.worktree_id as usize,
                        path: Path::new(&location.path),
                    });

                cx.update(|cx| match category.as_str() {
                    "language" => {
                        let settings =
                            AllLanguageSettings::get(location, cx).language(key.as_deref());
                        Ok(serde_json::to_string(&settings::LanguageSettings {
                            tab_size: settings.tab_size,
                        })?)
                    }
                    "lsp" => {
                        let settings = key
                            .and_then(|key| {
                                ProjectSettings::get(location, cx)
                                    .lsp
                                    .get(&Arc::<str>::from(key))
                            })
                            .cloned()
                            .unwrap_or_default();
                        Ok(serde_json::to_string(&settings::LspSettings {
                            binary: settings.binary.map(|binary| settings::BinarySettings {
                                path: binary.path,
                                arguments: binary.arguments,
                            }),
                            settings: settings.settings,
                            initialization_options: settings.initialization_options,
                        })?)
                    }
                    _ => {
                        bail!("Unknown settings category: {}", category);
                    }
                })
            }
            .boxed_local()
        })
        .await?
        .to_wasmtime_result()
    }

    async fn set_language_server_installation_status(
        &mut self,
        server_name: String,
        status: LanguageServerInstallationStatus,
    ) -> wasmtime::Result<()> {
        let status = match status {
            LanguageServerInstallationStatus::CheckingForUpdate => {
                LanguageServerBinaryStatus::CheckingForUpdate
            }
            LanguageServerInstallationStatus::Downloading => {
                LanguageServerBinaryStatus::Downloading
            }
            LanguageServerInstallationStatus::None => LanguageServerBinaryStatus::None,
            LanguageServerInstallationStatus::Failed(error) => {
                LanguageServerBinaryStatus::Failed { error }
            }
        };

        self.host
            .language_registry
            .update_lsp_status(language::LanguageServerName(server_name.into()), status);
        Ok(())
    }

    async fn download_file(
        &mut self,
        url: String,
        path: String,
        file_type: DownloadedFileType,
    ) -> wasmtime::Result<Result<(), String>> {
        maybe!(async {
            let path = PathBuf::from(path);
            let extension_work_dir = self.host.work_dir.join(self.manifest.id.as_ref());

            self.host.fs.create_dir(&extension_work_dir).await?;

            let destination_path = self
                .host
                .writeable_path_from_extension(&self.manifest.id, &path)?;

            let mut response = self
                .host
                .http_client
                .get(&url, Default::default(), true)
                .await
                .map_err(|err| anyhow!("error downloading release: {}", err))?;

            if !response.status().is_success() {
                Err(anyhow!(
                    "download failed with status {}",
                    response.status().to_string()
                ))?;
            }
            let body = BufReader::new(response.body_mut());

            match file_type {
                DownloadedFileType::Uncompressed => {
                    futures::pin_mut!(body);
                    self.host
                        .fs
                        .create_file_with(&destination_path, body)
                        .await?;
                }
                DownloadedFileType::Gzip => {
                    let body = GzipDecoder::new(body);
                    futures::pin_mut!(body);
                    self.host
                        .fs
                        .create_file_with(&destination_path, body)
                        .await?;
                }
                DownloadedFileType::GzipTar => {
                    let body = GzipDecoder::new(body);
                    futures::pin_mut!(body);
                    self.host
                        .fs
                        .extract_tar_file(&destination_path, Archive::new(body))
                        .await?;
                }
                DownloadedFileType::Zip => {
                    let file_name = destination_path
                        .file_name()
                        .ok_or_else(|| anyhow!("invalid download path"))?
                        .to_string_lossy();
                    let zip_filename = format!("{file_name}.zip");
                    let mut zip_path = destination_path.clone();
                    zip_path.set_file_name(zip_filename);

                    futures::pin_mut!(body);
                    self.host.fs.create_file_with(&zip_path, body).await?;

                    let unzip_status = std::process::Command::new("unzip")
                        .current_dir(&extension_work_dir)
                        .arg("-d")
                        .arg(&destination_path)
                        .arg(&zip_path)
                        .output()?
                        .status;
                    if !unzip_status.success() {
                        Err(anyhow!("failed to unzip {} archive", path.display()))?;
                    }
                }
            }

            Ok(())
        })
        .await
        .to_wasmtime_result()
    }

    async fn make_file_executable(&mut self, path: String) -> wasmtime::Result<Result<(), String>> {
        #[allow(unused)]
        let path = self
            .host
            .writeable_path_from_extension(&self.manifest.id, Path::new(&path))?;

        #[cfg(unix)]
        {
            use std::fs::{self, Permissions};
            use std::os::unix::fs::PermissionsExt;

            return fs::set_permissions(&path, Permissions::from_mode(0o755))
                .map_err(|error| anyhow!("failed to set permissions for path {path:?}: {error}"))
                .to_wasmtime_result();
        }

        #[cfg(not(unix))]
        Ok(Ok(()))
    }
}
