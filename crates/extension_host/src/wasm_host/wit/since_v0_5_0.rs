use crate::wasm_host::WasmState;
use anyhow::Result;
use extension::{KeyValueStoreDelegate, ProjectDelegate, WorktreeDelegate};
use gpui::BackgroundExecutor;
use semantic_version::SemanticVersion;
use std::sync::{Arc, OnceLock};
use wasmtime::component::{Linker, Resource};

use super::latest;

pub const MIN_VERSION: SemanticVersion = SemanticVersion::new(0, 5, 0);

wasmtime::component::bindgen!({
    async: true,
    trappable_imports: true,
    path: "../extension_api/wit/since_v0.5.0",
    with: {
        "worktree": ExtensionWorktree,
        "project": ExtensionProject,
        "key-value-store": ExtensionKeyValueStore,
        "zed:extension/common": latest::zed::extension::common,
        "zed:extension/github": latest::zed::extension::github,
        "zed:extension/http-client": latest::zed::extension::http_client,
        "zed:extension/lsp": latest::zed::extension::lsp,
        "zed:extension/nodejs": latest::zed::extension::nodejs,
        "zed:extension/platform": latest::zed::extension::platform,
        "zed:extension/process": latest::zed::extension::process,
        "zed:extension/slash-command": latest::zed::extension::slash_command,
        "zed:extension/context-server": latest::zed::extension::context_server,
    },
});

mod settings {
    #![allow(dead_code)]
    include!(concat!(env!("OUT_DIR"), "/since_v0.5.0/settings.rs"));
}

pub type ExtensionWorktree = Arc<dyn WorktreeDelegate>;
pub type ExtensionProject = Arc<dyn ProjectDelegate>;
pub type ExtensionKeyValueStore = Arc<dyn KeyValueStoreDelegate>;

pub fn linker(executor: &BackgroundExecutor) -> &'static Linker<WasmState> {
    static LINKER: OnceLock<Linker<WasmState>> = OnceLock::new();
    LINKER.get_or_init(|| super::new_linker(executor, Extension::add_to_linker))
}

impl From<CodeLabel> for latest::CodeLabel {
    fn from(value: CodeLabel) -> Self {
        Self {
            code: value.code,
            spans: value.spans.into_iter().map(Into::into).collect(),
            filter_range: value.filter_range,
        }
    }
}

impl From<CodeLabelSpan> for latest::CodeLabelSpan {
    fn from(value: CodeLabelSpan) -> Self {
        match value {
            CodeLabelSpan::CodeRange(range) => Self::CodeRange(range),
            CodeLabelSpan::Literal(literal) => Self::Literal(literal.into()),
        }
    }
}

impl From<CodeLabelSpanLiteral> for latest::CodeLabelSpanLiteral {
    fn from(value: CodeLabelSpanLiteral) -> Self {
        Self {
            text: value.text,
            highlight_name: value.highlight_name,
        }
    }
}

impl From<SettingsLocation> for latest::SettingsLocation {
    fn from(value: SettingsLocation) -> Self {
        Self {
            worktree_id: value.worktree_id,
            path: value.path,
        }
    }
}

impl From<LanguageServerInstallationStatus> for latest::LanguageServerInstallationStatus {
    fn from(value: LanguageServerInstallationStatus) -> Self {
        match value {
            LanguageServerInstallationStatus::None => Self::None,
            LanguageServerInstallationStatus::Downloading => Self::Downloading,
            LanguageServerInstallationStatus::CheckingForUpdate => Self::CheckingForUpdate,
            LanguageServerInstallationStatus::Failed(message) => Self::Failed(message),
        }
    }
}

impl From<DownloadedFileType> for latest::DownloadedFileType {
    fn from(value: DownloadedFileType) -> Self {
        match value {
            DownloadedFileType::Gzip => Self::Gzip,
            DownloadedFileType::GzipTar => Self::GzipTar,
            DownloadedFileType::Zip => Self::Zip,
            DownloadedFileType::Uncompressed => Self::Uncompressed,
        }
    }
}

impl HostKeyValueStore for WasmState {
    async fn insert(
        &mut self,
        kv_store: Resource<ExtensionKeyValueStore>,
        key: String,
        value: String,
    ) -> wasmtime::Result<Result<(), String>> {
        latest::HostKeyValueStore::insert(self, kv_store, key, value).await
    }

    async fn drop(&mut self, _worktree: Resource<ExtensionKeyValueStore>) -> Result<()> {
        // We only ever hand out borrows of key-value stores.
        Ok(())
    }
}

impl HostProject for WasmState {
    async fn worktree_ids(
        &mut self,
        project: Resource<ExtensionProject>,
    ) -> wasmtime::Result<Vec<u64>> {
        latest::HostProject::worktree_ids(self, project).await
    }

    async fn drop(&mut self, _project: Resource<Project>) -> Result<()> {
        // We only ever hand out borrows of projects.
        Ok(())
    }
}

impl HostWorktree for WasmState {
    async fn id(&mut self, delegate: Resource<Arc<dyn WorktreeDelegate>>) -> wasmtime::Result<u64> {
        latest::HostWorktree::id(self, delegate).await
    }

    async fn root_path(
        &mut self,
        delegate: Resource<Arc<dyn WorktreeDelegate>>,
    ) -> wasmtime::Result<String> {
        latest::HostWorktree::root_path(self, delegate).await
    }

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
        // We only ever hand out borrows of worktrees.
        Ok(())
    }
}

impl ExtensionImports for WasmState {
    async fn get_settings(
        &mut self,
        location: Option<self::SettingsLocation>,
        category: String,
        key: Option<String>,
    ) -> wasmtime::Result<Result<String, String>> {
        latest::ExtensionImports::get_settings(
            self,
            location.map(|location| location.into()),
            category,
            key,
        )
        .await
    }

    async fn set_language_server_installation_status(
        &mut self,
        server_name: String,
        status: LanguageServerInstallationStatus,
    ) -> wasmtime::Result<()> {
        latest::ExtensionImports::set_language_server_installation_status(
            self,
            server_name,
            status.into(),
        )
        .await
    }

    async fn download_file(
        &mut self,
        url: String,
        path: String,
        file_type: DownloadedFileType,
    ) -> wasmtime::Result<Result<(), String>> {
        latest::ExtensionImports::download_file(self, url, path, file_type.into()).await
    }

    async fn make_file_executable(&mut self, path: String) -> wasmtime::Result<Result<(), String>> {
        latest::ExtensionImports::make_file_executable(self, path).await
    }
}
