use crate::wasm_host::WasmState;
use anyhow::{anyhow, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use futures::io::BufReader;
use language::{LanguageServerBinaryStatus, LspAdapterDelegate};
use std::{
    env,
    path::PathBuf,
    sync::{Arc, OnceLock},
};
use util::{maybe, SemanticVersion};
use wasmtime::component::{Linker, Resource};

pub const VERSION: SemanticVersion = SemanticVersion {
    major: 0,
    minor: 0,
    patch: 4,
};

wasmtime::component::bindgen!({
    async: true,
    path: "../extension_api/wit/0.0.4",
    with: {
         "worktree": ExtensionWorktree,
    },
});

pub type ExtensionWorktree = Arc<dyn LspAdapterDelegate>;

pub fn linker() -> &'static Linker<WasmState> {
    static LINKER: OnceLock<Linker<WasmState>> = OnceLock::new();
    LINKER.get_or_init(|| super::new_linker(Extension::add_to_linker))
}

#[async_trait]
impl HostWorktree for WasmState {
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
        // we only ever hand out borrows of worktrees
        Ok(())
    }
}

#[async_trait]
impl ExtensionImports for WasmState {
    async fn node_binary_path(&mut self) -> wasmtime::Result<Result<String, String>> {
        convert_result(
            self.host
                .node_runtime
                .binary_path()
                .await
                .map(|path| path.to_string_lossy().to_string()),
        )
    }

    async fn npm_package_latest_version(
        &mut self,
        package_name: String,
    ) -> wasmtime::Result<Result<String, String>> {
        convert_result(
            self.host
                .node_runtime
                .npm_package_latest_version(&package_name)
                .await,
        )
    }

    async fn npm_package_installed_version(
        &mut self,
        package_name: String,
    ) -> wasmtime::Result<Result<Option<String>, String>> {
        convert_result(
            self.host
                .node_runtime
                .npm_package_installed_version(&self.work_dir(), &package_name)
                .await,
        )
    }

    async fn npm_install_package(
        &mut self,
        package_name: String,
        version: String,
    ) -> wasmtime::Result<Result<(), String>> {
        convert_result(
            self.host
                .node_runtime
                .npm_install_packages(&self.work_dir(), &[(&package_name, &version)])
                .await,
        )
    }

    async fn latest_github_release(
        &mut self,
        repo: String,
        options: GithubReleaseOptions,
    ) -> wasmtime::Result<Result<GithubRelease, String>> {
        convert_result(
            maybe!(async {
                let release = util::github::latest_github_release(
                    &repo,
                    options.require_assets,
                    options.pre_release,
                    self.host.http_client.clone(),
                )
                .await?;
                Ok(GithubRelease {
                    version: release.tag_name,
                    assets: release
                        .assets
                        .into_iter()
                        .map(|asset| GithubReleaseAsset {
                            name: asset.name,
                            download_url: asset.browser_download_url,
                        })
                        .collect(),
                })
            })
            .await,
        )
    }

    async fn current_platform(&mut self) -> Result<(Os, Architecture)> {
        Ok((
            match env::consts::OS {
                "macos" => Os::Mac,
                "linux" => Os::Linux,
                "windows" => Os::Windows,
                _ => panic!("unsupported os"),
            },
            match env::consts::ARCH {
                "aarch64" => Architecture::Aarch64,
                "x86" => Architecture::X86,
                "x86_64" => Architecture::X8664,
                _ => panic!("unsupported architecture"),
            },
        ))
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
        let result = maybe!(async {
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
        .await;
        convert_result(result)
    }
}

fn convert_result<T>(result: Result<T>) -> wasmtime::Result<Result<T, String>> {
    Ok(result.map_err(|error| error.to_string()))
}
