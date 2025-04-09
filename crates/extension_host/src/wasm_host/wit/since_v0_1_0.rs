use crate::wasm_host::{WasmState, wit::ToWasmtimeResult};
use ::http_client::{AsyncBody, HttpRequestExt};
use ::settings::{Settings, WorktreeId};
use anyhow::{Context, Result, anyhow, bail};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use extension::{ExtensionLanguageServerProxy, KeyValueStoreDelegate, WorktreeDelegate};
use futures::{AsyncReadExt, lock::Mutex};
use futures::{FutureExt as _, io::BufReader};
use language::LanguageName;
use language::{BinaryStatus, language_settings::AllLanguageSettings};
use project::project_settings::ProjectSettings;
use semantic_version::SemanticVersion;
use std::{
    path::{Path, PathBuf},
    sync::{Arc, OnceLock},
};
use util::maybe;
use wasmtime::component::{Linker, Resource};

use super::latest;

pub const MIN_VERSION: SemanticVersion = SemanticVersion::new(0, 1, 0);

wasmtime::component::bindgen!({
    async: true,
    trappable_imports: true,
    path: "../extension_api/wit/since_v0.1.0",
    with: {
         "worktree": ExtensionWorktree,
         "key-value-store": ExtensionKeyValueStore,
         "zed:extension/http-client/http-response-stream": ExtensionHttpResponseStream,
         "zed:extension/github": latest::zed::extension::github,
         "zed:extension/nodejs": latest::zed::extension::nodejs,
         "zed:extension/platform": latest::zed::extension::platform,
         "zed:extension/slash-command": latest::zed::extension::slash_command,
    },
});

pub use self::zed::extension::*;

mod settings {
    include!(concat!(env!("OUT_DIR"), "/since_v0.1.0/settings.rs"));
}

pub type ExtensionWorktree = Arc<dyn WorktreeDelegate>;
pub type ExtensionKeyValueStore = Arc<dyn KeyValueStoreDelegate>;
pub type ExtensionHttpResponseStream = Arc<Mutex<::http_client::Response<AsyncBody>>>;

pub fn linker() -> &'static Linker<WasmState> {
    static LINKER: OnceLock<Linker<WasmState>> = OnceLock::new();
    LINKER.get_or_init(|| super::new_linker(Extension::add_to_linker))
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

impl From<Range> for latest::Range {
    fn from(value: Range) -> Self {
        Self {
            start: value.start,
            end: value.end,
        }
    }
}

impl From<CodeLabelSpan> for latest::CodeLabelSpan {
    fn from(value: CodeLabelSpan) -> Self {
        match value {
            CodeLabelSpan::CodeRange(range) => Self::CodeRange(range.into()),
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

impl From<CodeLabel> for latest::CodeLabel {
    fn from(value: CodeLabel) -> Self {
        Self {
            code: value.code,
            spans: value.spans.into_iter().map(Into::into).collect(),
            filter_range: value.filter_range.into(),
        }
    }
}

impl From<latest::Completion> for Completion {
    fn from(value: latest::Completion) -> Self {
        Self {
            label: value.label,
            detail: value.detail,
            kind: value.kind.map(Into::into),
            insert_text_format: value.insert_text_format.map(Into::into),
        }
    }
}

impl From<latest::lsp::CompletionKind> for lsp::CompletionKind {
    fn from(value: latest::lsp::CompletionKind) -> Self {
        match value {
            latest::lsp::CompletionKind::Text => Self::Text,
            latest::lsp::CompletionKind::Method => Self::Method,
            latest::lsp::CompletionKind::Function => Self::Function,
            latest::lsp::CompletionKind::Constructor => Self::Constructor,
            latest::lsp::CompletionKind::Field => Self::Field,
            latest::lsp::CompletionKind::Variable => Self::Variable,
            latest::lsp::CompletionKind::Class => Self::Class,
            latest::lsp::CompletionKind::Interface => Self::Interface,
            latest::lsp::CompletionKind::Module => Self::Module,
            latest::lsp::CompletionKind::Property => Self::Property,
            latest::lsp::CompletionKind::Unit => Self::Unit,
            latest::lsp::CompletionKind::Value => Self::Value,
            latest::lsp::CompletionKind::Enum => Self::Enum,
            latest::lsp::CompletionKind::Keyword => Self::Keyword,
            latest::lsp::CompletionKind::Snippet => Self::Snippet,
            latest::lsp::CompletionKind::Color => Self::Color,
            latest::lsp::CompletionKind::File => Self::File,
            latest::lsp::CompletionKind::Reference => Self::Reference,
            latest::lsp::CompletionKind::Folder => Self::Folder,
            latest::lsp::CompletionKind::EnumMember => Self::EnumMember,
            latest::lsp::CompletionKind::Constant => Self::Constant,
            latest::lsp::CompletionKind::Struct => Self::Struct,
            latest::lsp::CompletionKind::Event => Self::Event,
            latest::lsp::CompletionKind::Operator => Self::Operator,
            latest::lsp::CompletionKind::TypeParameter => Self::TypeParameter,
            latest::lsp::CompletionKind::Other(kind) => Self::Other(kind),
        }
    }
}

impl From<latest::lsp::InsertTextFormat> for lsp::InsertTextFormat {
    fn from(value: latest::lsp::InsertTextFormat) -> Self {
        match value {
            latest::lsp::InsertTextFormat::PlainText => Self::PlainText,
            latest::lsp::InsertTextFormat::Snippet => Self::Snippet,
            latest::lsp::InsertTextFormat::Other(value) => Self::Other(value),
        }
    }
}

impl From<latest::lsp::Symbol> for lsp::Symbol {
    fn from(value: latest::lsp::Symbol) -> Self {
        Self {
            name: value.name,
            kind: value.kind.into(),
        }
    }
}

impl From<latest::lsp::SymbolKind> for lsp::SymbolKind {
    fn from(value: latest::lsp::SymbolKind) -> Self {
        match value {
            latest::lsp::SymbolKind::File => Self::File,
            latest::lsp::SymbolKind::Module => Self::Module,
            latest::lsp::SymbolKind::Namespace => Self::Namespace,
            latest::lsp::SymbolKind::Package => Self::Package,
            latest::lsp::SymbolKind::Class => Self::Class,
            latest::lsp::SymbolKind::Method => Self::Method,
            latest::lsp::SymbolKind::Property => Self::Property,
            latest::lsp::SymbolKind::Field => Self::Field,
            latest::lsp::SymbolKind::Constructor => Self::Constructor,
            latest::lsp::SymbolKind::Enum => Self::Enum,
            latest::lsp::SymbolKind::Interface => Self::Interface,
            latest::lsp::SymbolKind::Function => Self::Function,
            latest::lsp::SymbolKind::Variable => Self::Variable,
            latest::lsp::SymbolKind::Constant => Self::Constant,
            latest::lsp::SymbolKind::String => Self::String,
            latest::lsp::SymbolKind::Number => Self::Number,
            latest::lsp::SymbolKind::Boolean => Self::Boolean,
            latest::lsp::SymbolKind::Array => Self::Array,
            latest::lsp::SymbolKind::Object => Self::Object,
            latest::lsp::SymbolKind::Key => Self::Key,
            latest::lsp::SymbolKind::Null => Self::Null,
            latest::lsp::SymbolKind::EnumMember => Self::EnumMember,
            latest::lsp::SymbolKind::Struct => Self::Struct,
            latest::lsp::SymbolKind::Event => Self::Event,
            latest::lsp::SymbolKind::Operator => Self::Operator,
            latest::lsp::SymbolKind::TypeParameter => Self::TypeParameter,
            latest::lsp::SymbolKind::Other(kind) => Self::Other(kind),
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
        let kv_store = self.table.get(&kv_store)?;
        kv_store.insert(key, value).await.to_wasmtime_result()
    }

    async fn drop(&mut self, _worktree: Resource<ExtensionKeyValueStore>) -> Result<()> {
        // We only ever hand out borrows of key-value stores.
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

impl common::Host for WasmState {}

impl http_client::Host for WasmState {
    async fn fetch(
        &mut self,
        request: http_client::HttpRequest,
    ) -> wasmtime::Result<Result<http_client::HttpResponse, String>> {
        maybe!(async {
            let url = &request.url;
            let request = convert_request(&request)?;
            let mut response = self.host.http_client.send(request).await?;

            if response.status().is_client_error() || response.status().is_server_error() {
                bail!("failed to fetch '{url}': status code {}", response.status())
            }
            convert_response(&mut response).await
        })
        .await
        .to_wasmtime_result()
    }

    async fn fetch_stream(
        &mut self,
        request: http_client::HttpRequest,
    ) -> wasmtime::Result<Result<Resource<ExtensionHttpResponseStream>, String>> {
        let request = convert_request(&request)?;
        let response = self.host.http_client.send(request);
        maybe!(async {
            let response = response.await?;
            let stream = Arc::new(Mutex::new(response));
            let resource = self.table.push(stream)?;
            Ok(resource)
        })
        .await
        .to_wasmtime_result()
    }
}

impl http_client::HostHttpResponseStream for WasmState {
    async fn next_chunk(
        &mut self,
        resource: Resource<ExtensionHttpResponseStream>,
    ) -> wasmtime::Result<Result<Option<Vec<u8>>, String>> {
        let stream = self.table.get(&resource)?.clone();
        maybe!(async move {
            let mut response = stream.lock().await;
            let mut buffer = vec![0; 8192]; // 8KB buffer
            let bytes_read = response.body_mut().read(&mut buffer).await?;
            if bytes_read == 0 {
                Ok(None)
            } else {
                buffer.truncate(bytes_read);
                Ok(Some(buffer))
            }
        })
        .await
        .to_wasmtime_result()
    }

    async fn drop(&mut self, _resource: Resource<ExtensionHttpResponseStream>) -> Result<()> {
        Ok(())
    }
}

impl From<http_client::HttpMethod> for ::http_client::Method {
    fn from(value: http_client::HttpMethod) -> Self {
        match value {
            http_client::HttpMethod::Get => Self::GET,
            http_client::HttpMethod::Post => Self::POST,
            http_client::HttpMethod::Put => Self::PUT,
            http_client::HttpMethod::Delete => Self::DELETE,
            http_client::HttpMethod::Head => Self::HEAD,
            http_client::HttpMethod::Options => Self::OPTIONS,
            http_client::HttpMethod::Patch => Self::PATCH,
        }
    }
}

fn convert_request(
    extension_request: &http_client::HttpRequest,
) -> Result<::http_client::Request<AsyncBody>, anyhow::Error> {
    let mut request = ::http_client::Request::builder()
        .method(::http_client::Method::from(extension_request.method))
        .uri(&extension_request.url)
        .follow_redirects(match extension_request.redirect_policy {
            http_client::RedirectPolicy::NoFollow => ::http_client::RedirectPolicy::NoFollow,
            http_client::RedirectPolicy::FollowLimit(limit) => {
                ::http_client::RedirectPolicy::FollowLimit(limit)
            }
            http_client::RedirectPolicy::FollowAll => ::http_client::RedirectPolicy::FollowAll,
        });
    for (key, value) in &extension_request.headers {
        request = request.header(key, value);
    }
    let body = extension_request
        .body
        .clone()
        .map(AsyncBody::from)
        .unwrap_or_default();
    request.body(body).map_err(anyhow::Error::from)
}

async fn convert_response(
    response: &mut ::http_client::Response<AsyncBody>,
) -> Result<http_client::HttpResponse, anyhow::Error> {
    let mut extension_response = http_client::HttpResponse {
        body: Vec::new(),
        headers: Vec::new(),
    };

    for (key, value) in response.headers() {
        extension_response
            .headers
            .push((key.to_string(), value.to_str().unwrap_or("").to_string()));
    }

    response
        .body_mut()
        .read_to_end(&mut extension_response.body)
        .await?;

    Ok(extension_response)
}

impl lsp::Host for WasmState {}

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
                        worktree_id: WorktreeId::from_proto(location.worktree_id),
                        path: Path::new(&location.path),
                    });

                cx.update(|cx| match category.as_str() {
                    "language" => {
                        let key = key.map(|k| LanguageName::new(&k));
                        let settings = AllLanguageSettings::get(location, cx).language(
                            location,
                            key.as_ref(),
                            cx,
                        );
                        Ok(serde_json::to_string(&settings::LanguageSettings {
                            tab_size: settings.tab_size,
                        })?)
                    }
                    "lsp" => {
                        let settings = key
                            .and_then(|key| {
                                ProjectSettings::get(location, cx)
                                    .lsp
                                    .get(&::lsp::LanguageServerName(key.into()))
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
            LanguageServerInstallationStatus::CheckingForUpdate => BinaryStatus::CheckingForUpdate,
            LanguageServerInstallationStatus::Downloading => BinaryStatus::Downloading,
            LanguageServerInstallationStatus::None => BinaryStatus::None,
            LanguageServerInstallationStatus::Failed(error) => BinaryStatus::Failed { error },
        };

        self.host
            .proxy
            .update_language_server_status(::lsp::LanguageServerName(server_name.into()), status);

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
                    futures::pin_mut!(body);
                    node_runtime::extract_zip(&destination_path, body)
                        .await
                        .with_context(|| format!("failed to unzip {} archive", path.display()))?;
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
