use super::latest;
use crate::wasm_host::WasmState;
use anyhow::Result;
use async_trait::async_trait;
use language::{LanguageServerBinaryStatus, LspAdapterDelegate};
use std::sync::{Arc, OnceLock};
use util::SemanticVersion;
use wasmtime::component::{Linker, Resource};

pub const VERSION: SemanticVersion = SemanticVersion {
    major: 0,
    minor: 0,
    patch: 1,
};

wasmtime::component::bindgen!({
    async: true,
    path: "../extension_api/wit/0.0.1",
    with: {
         "worktree": ExtensionWorktree,
    },
});

pub type ExtensionWorktree = Arc<dyn LspAdapterDelegate>;

pub fn linker() -> &'static Linker<WasmState> {
    static LINKER: OnceLock<Linker<WasmState>> = OnceLock::new();
    LINKER.get_or_init(|| super::new_linker(Extension::add_to_linker))
}

impl From<latest::Os> for Os {
    fn from(value: latest::Os) -> Self {
        match value {
            latest::Os::Mac => Os::Mac,
            latest::Os::Linux => Os::Linux,
            latest::Os::Windows => Os::Windows,
        }
    }
}

impl From<latest::Architecture> for Architecture {
    fn from(value: latest::Architecture) -> Self {
        match value {
            latest::Architecture::Aarch64 => Self::Aarch64,
            latest::Architecture::X86 => Self::X86,
            latest::Architecture::X8664 => Self::X8664,
        }
    }
}

impl From<latest::GithubRelease> for GithubRelease {
    fn from(value: latest::GithubRelease) -> Self {
        Self {
            version: value.version,
            assets: value.assets.into_iter().map(|asset| asset.into()).collect(),
        }
    }
}

impl From<latest::GithubReleaseAsset> for GithubReleaseAsset {
    fn from(value: latest::GithubReleaseAsset) -> Self {
        Self {
            name: value.name,
            download_url: value.download_url,
        }
    }
}

impl From<GithubReleaseOptions> for latest::GithubReleaseOptions {
    fn from(value: GithubReleaseOptions) -> Self {
        Self {
            require_assets: value.require_assets,
            pre_release: value.pre_release,
        }
    }
}

impl From<DownloadedFileType> for latest::DownloadedFileType {
    fn from(value: DownloadedFileType) -> Self {
        match value {
            DownloadedFileType::Gzip => latest::DownloadedFileType::Gzip,
            DownloadedFileType::GzipTar => latest::DownloadedFileType::GzipTar,
            DownloadedFileType::Zip => latest::DownloadedFileType::Zip,
            DownloadedFileType::Uncompressed => latest::DownloadedFileType::Uncompressed,
        }
    }
}

impl From<latest::LanguageServerConfig> for LanguageServerConfig {
    fn from(value: latest::LanguageServerConfig) -> Self {
        Self {
            name: value.name,
            language_name: value.language_name,
        }
    }
}

impl From<Command> for latest::Command {
    fn from(value: Command) -> Self {
        Self {
            command: value.command,
            args: value.args,
            env: value.env,
        }
    }
}

#[async_trait]
impl HostWorktree for WasmState {
    async fn read_text_file(
        &mut self,
        delegate: Resource<Arc<dyn LspAdapterDelegate>>,
        path: String,
    ) -> wasmtime::Result<Result<String, String>> {
        latest::HostWorktree::read_text_file(self, delegate, path).await
    }

    async fn shell_env(
        &mut self,
        delegate: Resource<Arc<dyn LspAdapterDelegate>>,
    ) -> wasmtime::Result<EnvVars> {
        latest::HostWorktree::shell_env(self, delegate).await
    }

    async fn which(
        &mut self,
        delegate: Resource<Arc<dyn LspAdapterDelegate>>,
        binary_name: String,
    ) -> wasmtime::Result<Option<String>> {
        latest::HostWorktree::which(self, delegate, binary_name).await
    }

    fn drop(&mut self, _worktree: Resource<Worktree>) -> Result<()> {
        Ok(())
    }
}

#[async_trait]
impl ExtensionImports for WasmState {
    async fn node_binary_path(&mut self) -> wasmtime::Result<Result<String, String>> {
        latest::ExtensionImports::node_binary_path(self).await
    }

    async fn npm_package_latest_version(
        &mut self,
        package_name: String,
    ) -> wasmtime::Result<Result<String, String>> {
        latest::ExtensionImports::npm_package_latest_version(self, package_name).await
    }

    async fn npm_package_installed_version(
        &mut self,
        package_name: String,
    ) -> wasmtime::Result<Result<Option<String>, String>> {
        latest::ExtensionImports::npm_package_installed_version(self, package_name).await
    }

    async fn npm_install_package(
        &mut self,
        package_name: String,
        version: String,
    ) -> wasmtime::Result<Result<(), String>> {
        latest::ExtensionImports::npm_install_package(self, package_name, version).await
    }

    async fn latest_github_release(
        &mut self,
        repo: String,
        options: GithubReleaseOptions,
    ) -> wasmtime::Result<Result<GithubRelease, String>> {
        Ok(
            latest::ExtensionImports::latest_github_release(self, repo, options.into())
                .await?
                .map(|github| github.into()),
        )
    }

    async fn current_platform(&mut self) -> Result<(Os, Architecture)> {
        latest::ExtensionImports::current_platform(self)
            .await
            .map(|(os, arch)| (os.into(), arch.into()))
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
            LanguageServerInstallationStatus::Cached
            | LanguageServerInstallationStatus::Downloaded => LanguageServerBinaryStatus::None,
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
        latest::ExtensionImports::download_file(self, url, path, file_type.into()).await
    }
}
