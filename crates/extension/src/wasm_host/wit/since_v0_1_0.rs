use crate::wasm_host::{wit::ToWasmtimeResult, WasmState};
use ::http_client::{AsyncBody, HttpRequestExt};
use ::settings::{Settings, WorktreeId};
use anyhow::{anyhow, bail, Context, Result};
use async_compression::futures::bufread::GzipDecoder;
use async_tar::Archive;
use async_trait::async_trait;
use futures::{io::BufReader, FutureExt as _};
use futures::{lock::Mutex, AsyncReadExt};
use indexed_docs::IndexedDocsDatabase;
use language::{
    language_settings::AllLanguageSettings, LanguageServerBinaryStatus, LspAdapterDelegate,
};
use language::{LanguageName, LanguageServerName};
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
pub const MAX_VERSION: SemanticVersion = SemanticVersion::new(0, 1, 0);

wasmtime::component::bindgen!({
    async: true,
    trappable_imports: true,
    path: "../extension_api/wit/since_v0.1.0",
    with: {
         "worktree": ExtensionWorktree,
         "key-value-store": ExtensionKeyValueStore,
         "zed:extension/http-client/http-response-stream": ExtensionHttpResponseStream,
         "zed:extension/github": latest::zed::extension::github,
         "zed:extension/lsp": latest::zed::extension::lsp,
         "zed:extension/nodejs": latest::zed::extension::nodejs,
         "zed:extension/platform": latest::zed::extension::platform,
         "zed:extension/slash-command": latest::zed::extension::slash_command,
    },
});

pub use self::zed::extension::*;

mod settings {
    include!(concat!(env!("OUT_DIR"), "/since_v0.1.0/settings.rs"));
}

pub type ExtensionWorktree = Arc<dyn LspAdapterDelegate>;
pub type ExtensionKeyValueStore = Arc<IndexedDocsDatabase>;
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

#[async_trait]
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

    fn drop(&mut self, _worktree: Resource<ExtensionKeyValueStore>) -> Result<()> {
        // We only ever hand out borrows of key-value stores.
        Ok(())
    }
}

#[async_trait]
impl HostWorktree for WasmState {
    async fn id(
        &mut self,
        delegate: Resource<Arc<dyn LspAdapterDelegate>>,
    ) -> wasmtime::Result<u64> {
        let delegate = self.table.get(&delegate)?;
        Ok(delegate.worktree_id().to_proto())
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
impl common::Host for WasmState {}

#[async_trait]
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

#[async_trait]
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

    fn drop(&mut self, _resource: Resource<ExtensionHttpResponseStream>) -> Result<()> {
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
                        worktree_id: WorktreeId::from_proto(location.worktree_id),
                        path: Path::new(&location.path),
                    });

                cx.update(|cx| match category.as_str() {
                    "language" => {
                        let key = key.map(|k| LanguageName::new(&k));
                        let settings =
                            AllLanguageSettings::get(location, cx).language(key.as_ref());
                        Ok(serde_json::to_string(&settings::LanguageSettings {
                            tab_size: settings.tab_size,
                        })?)
                    }
                    "lsp" => {
                        let settings = key
                            .and_then(|key| {
                                ProjectSettings::get(location, cx)
                                    .lsp
                                    .get(&LanguageServerName(key.into()))
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
