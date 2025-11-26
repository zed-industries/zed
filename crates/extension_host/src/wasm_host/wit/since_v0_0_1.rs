use super::latest;
use crate::wasm_host::WasmState;
use crate::wasm_host::wit::since_v0_0_4;
use anyhow::Result;
use extension::{ExtensionLanguageServerProxy, WorktreeDelegate};
use gpui::BackgroundExecutor;
use language::BinaryStatus;
use semver::Version;
use std::sync::{Arc, OnceLock};
use wasmtime::component::{Linker, Resource};

pub const MIN_VERSION: Version = Version::new(0, 0, 1);

wasmtime::component::bindgen!({
    async: true,
    trappable_imports: true,
    path: "../extension_api/wit/since_v0.0.1",
    with: {
         "worktree": ExtensionWorktree,
         "zed:extension/github": latest::zed::extension::github,
         "zed:extension/platform": latest::zed::extension::platform,
    },
});

pub type ExtensionWorktree = Arc<dyn WorktreeDelegate>;

pub fn linker(executor: &BackgroundExecutor) -> &'static Linker<WasmState> {
    static LINKER: OnceLock<Linker<WasmState>> = OnceLock::new();
    LINKER.get_or_init(|| super::new_linker(executor, Extension::add_to_linker))
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

impl From<since_v0_0_4::LanguageServerConfig> for LanguageServerConfig {
    fn from(value: since_v0_0_4::LanguageServerConfig) -> Self {
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

impl HostWorktree for WasmState {
    async fn read_text_file(
        &mut self,
        delegate: Resource<Arc<dyn WorktreeDelegate>>,
        path: String,
    ) -> wasmtime::Result<Result<String, String>> {
        latest::HostWorktree::read_text_file(self, delegate, path).await
    }

    async fn shell_env(
        &mut self,
        delegate: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> wasmtime::Result<EnvVars> {
        latest::HostWorktree::shell_env(self, delegate).await
    }

    async fn which(
        &mut self,
        delegate: Resource<Arc<dyn WorktreeDelegate>>,
        binary_name: String,
    ) -> wasmtime::Result<Option<String>> {
        latest::HostWorktree::which(self, delegate, binary_name).await
    }

    async fn drop(&mut self, _worktree: Resource<Worktree>) -> Result<()> {
        Ok(())
    }
}

impl ExtensionImports for WasmState {
    async fn node_binary_path(&mut self) -> wasmtime::Result<Result<String, String>> {
        latest::nodejs::Host::node_binary_path(self).await
    }

    async fn npm_package_latest_version(
        &mut self,
        package_name: String,
    ) -> wasmtime::Result<Result<String, String>> {
        latest::nodejs::Host::npm_package_latest_version(self, package_name).await
    }

    async fn npm_package_installed_version(
        &mut self,
        package_name: String,
    ) -> wasmtime::Result<Result<Option<String>, String>> {
        latest::nodejs::Host::npm_package_installed_version(self, package_name).await
    }

    async fn npm_install_package(
        &mut self,
        package_name: String,
        version: String,
    ) -> wasmtime::Result<Result<(), String>> {
        latest::nodejs::Host::npm_install_package(self, package_name, version).await
    }

    async fn latest_github_release(
        &mut self,
        repo: String,
        options: GithubReleaseOptions,
    ) -> wasmtime::Result<Result<GithubRelease, String>> {
        latest::zed::extension::github::Host::latest_github_release(self, repo, options).await
    }

    async fn current_platform(&mut self) -> Result<(Os, Architecture)> {
        latest::zed::extension::platform::Host::current_platform(self).await
    }

    async fn set_language_server_installation_status(
        &mut self,
        server_name: String,
        status: LanguageServerInstallationStatus,
    ) -> wasmtime::Result<()> {
        let status = match status {
            LanguageServerInstallationStatus::CheckingForUpdate => BinaryStatus::CheckingForUpdate,
            LanguageServerInstallationStatus::Downloading => BinaryStatus::Downloading,
            LanguageServerInstallationStatus::Cached
            | LanguageServerInstallationStatus::Downloaded => BinaryStatus::None,
            LanguageServerInstallationStatus::Failed(error) => BinaryStatus::Failed { error },
        };

        self.host
            .proxy
            .update_language_server_status(lsp::LanguageServerName(server_name.into()), status);

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
