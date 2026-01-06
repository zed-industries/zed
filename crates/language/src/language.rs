//! The `language` crate provides a large chunk of Zed's language-related
//! features (the other big contributors being project and lsp crates that revolve around LSP features).
//! Namely, this crate:
//! - Provides [`Language`], [`Grammar`] and [`LanguageRegistry`] types that
//!   use Tree-sitter to provide syntax highlighting to the editor; note though that `language` doesn't perform the highlighting by itself. It only maps ranges in a buffer to colors. Treesitter is also used for buffer outlines (lists of symbols in a buffer)
//! - Exposes [`LanguageConfig`] that describes how constructs (like brackets or line comments) should be handled by the editor for a source file of a particular language.
//!
//! Notably we do *not* assign a single language to a single file; in real world a single file can consist of multiple programming languages - HTML is a good example of that - and `language` crate tends to reflect that status quo in its API.
mod buffer;
mod diagnostic_set;
mod highlight_map;
mod language_registry;
pub mod language_settings;
mod manifest;
mod outline;
pub mod proto;
mod syntax_map;
mod task_context;
mod text_diff;
mod toolchain;

#[cfg(test)]
pub mod buffer_tests;

use crate::language_settings::SoftWrap;
pub use crate::language_settings::{EditPredictionsMode, IndentGuideSettings};
use anyhow::{Context as _, Result};
use async_trait::async_trait;
use collections::{HashMap, HashSet, IndexSet};
use futures::Future;
use futures::future::LocalBoxFuture;
use futures::lock::OwnedMutexGuard;
use gpui::{App, AsyncApp, Entity, SharedString};
pub use highlight_map::HighlightMap;
use http_client::HttpClient;
pub use language_registry::{
    LanguageName, LanguageServerStatusUpdate, LoadedLanguage, ServerHealth,
};
use lsp::{
    CodeActionKind, InitializeParams, LanguageServerBinary, LanguageServerBinaryOptions, Uri,
};
pub use manifest::{ManifestDelegate, ManifestName, ManifestProvider, ManifestQuery};
use parking_lot::Mutex;
use regex::Regex;
use schemars::{JsonSchema, SchemaGenerator, json_schema};
use semver::Version;
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use serde_json::Value;
use settings::WorktreeId;
use smol::future::FutureExt as _;
use std::num::NonZeroU32;
use std::{
    ffi::OsStr,
    fmt::Debug,
    hash::Hash,
    mem,
    ops::{DerefMut, Range},
    path::{Path, PathBuf},
    str,
    sync::{
        Arc, LazyLock,
        atomic::{AtomicUsize, Ordering::SeqCst},
    },
};
use syntax_map::{QueryCursorHandle, SyntaxSnapshot};
use task::RunnableTag;
pub use task_context::{ContextLocation, ContextProvider, RunnableRange};
pub use text_diff::{
    DiffOptions, apply_diff_patch, line_diff, text_diff, text_diff_with_options, unified_diff,
    unified_diff_with_offsets, word_diff_ranges,
};
use theme::SyntaxTheme;
pub use toolchain::{
    LanguageToolchainStore, LocalLanguageToolchainStore, Toolchain, ToolchainList, ToolchainLister,
    ToolchainMetadata, ToolchainScope,
};
use tree_sitter::{self, Query, QueryCursor, WasmStore, wasmtime};
use util::rel_path::RelPath;
use util::serde::default_true;

pub use buffer::Operation;
pub use buffer::*;
pub use diagnostic_set::{DiagnosticEntry, DiagnosticEntryRef, DiagnosticGroup};
pub use language_registry::{
    AvailableLanguage, BinaryStatus, LanguageNotFound, LanguageQueries, LanguageRegistry,
    QUERY_FILENAME_PREFIXES,
};
pub use lsp::{LanguageServerId, LanguageServerName};
pub use outline::*;
pub use syntax_map::{
    OwnedSyntaxLayer, SyntaxLayer, SyntaxMapMatches, ToTreeSitterPoint, TreeSitterOptions,
};
pub use text::{AnchorRangeExt, LineEnding};
pub use tree_sitter::{Node, Parser, Tree, TreeCursor};

static QUERY_CURSORS: Mutex<Vec<QueryCursor>> = Mutex::new(vec![]);
static PARSERS: Mutex<Vec<Parser>> = Mutex::new(vec![]);

pub fn with_parser<F, R>(func: F) -> R
where
    F: FnOnce(&mut Parser) -> R,
{
    let mut parser = PARSERS.lock().pop().unwrap_or_else(|| {
        let mut parser = Parser::new();
        parser
            .set_wasm_store(WasmStore::new(&WASM_ENGINE).unwrap())
            .unwrap();
        parser
    });
    parser.set_included_ranges(&[]).unwrap();
    let result = func(&mut parser);
    PARSERS.lock().push(parser);
    result
}

pub fn with_query_cursor<F, R>(func: F) -> R
where
    F: FnOnce(&mut QueryCursor) -> R,
{
    let mut cursor = QueryCursorHandle::new();
    func(cursor.deref_mut())
}

static NEXT_LANGUAGE_ID: AtomicUsize = AtomicUsize::new(0);
static NEXT_GRAMMAR_ID: AtomicUsize = AtomicUsize::new(0);
static WASM_ENGINE: LazyLock<wasmtime::Engine> = LazyLock::new(|| {
    wasmtime::Engine::new(&wasmtime::Config::new()).expect("Failed to create Wasmtime engine")
});

/// A shared grammar for plain text, exposed for reuse by downstream crates.
pub static PLAIN_TEXT: LazyLock<Arc<Language>> = LazyLock::new(|| {
    Arc::new(Language::new(
        LanguageConfig {
            name: "Plain Text".into(),
            soft_wrap: Some(SoftWrap::EditorWidth),
            matcher: LanguageMatcher {
                path_suffixes: vec!["txt".to_owned()],
                first_line_pattern: None,
            },
            brackets: BracketPairConfig {
                pairs: vec![
                    BracketPair {
                        start: "(".to_string(),
                        end: ")".to_string(),
                        close: true,
                        surround: true,
                        newline: false,
                    },
                    BracketPair {
                        start: "[".to_string(),
                        end: "]".to_string(),
                        close: true,
                        surround: true,
                        newline: false,
                    },
                    BracketPair {
                        start: "{".to_string(),
                        end: "}".to_string(),
                        close: true,
                        surround: true,
                        newline: false,
                    },
                    BracketPair {
                        start: "\"".to_string(),
                        end: "\"".to_string(),
                        close: true,
                        surround: true,
                        newline: false,
                    },
                    BracketPair {
                        start: "'".to_string(),
                        end: "'".to_string(),
                        close: true,
                        surround: true,
                        newline: false,
                    },
                ],
                disabled_scopes_by_bracket_ix: Default::default(),
            },
            ..Default::default()
        },
        None,
    ))
});

/// Types that represent a position in a buffer, and can be converted into
/// an LSP position, to send to a language server.
pub trait ToLspPosition {
    /// Converts the value into an LSP position.
    fn to_lsp_position(self) -> lsp::Position;
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    pub buffer: Entity<Buffer>,
    pub range: Range<Anchor>,
}

type ServerBinaryCache = futures::lock::Mutex<Option<(bool, LanguageServerBinary)>>;
type DownloadableLanguageServerBinary = LocalBoxFuture<'static, Result<LanguageServerBinary>>;
pub type LanguageServerBinaryLocations = LocalBoxFuture<
    'static,
    (
        Result<LanguageServerBinary>,
        Option<DownloadableLanguageServerBinary>,
    ),
>;
/// Represents a Language Server, with certain cached sync properties.
/// Uses [`LspAdapter`] under the hood, but calls all 'static' methods
/// once at startup, and caches the results.
pub struct CachedLspAdapter {
    pub name: LanguageServerName,
    pub disk_based_diagnostic_sources: Vec<String>,
    pub disk_based_diagnostics_progress_token: Option<String>,
    language_ids: HashMap<LanguageName, String>,
    pub adapter: Arc<dyn LspAdapter>,
    cached_binary: Arc<ServerBinaryCache>,
}

impl Debug for CachedLspAdapter {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("CachedLspAdapter")
            .field("name", &self.name)
            .field(
                "disk_based_diagnostic_sources",
                &self.disk_based_diagnostic_sources,
            )
            .field(
                "disk_based_diagnostics_progress_token",
                &self.disk_based_diagnostics_progress_token,
            )
            .field("language_ids", &self.language_ids)
            .finish_non_exhaustive()
    }
}

impl CachedLspAdapter {
    pub fn new(adapter: Arc<dyn LspAdapter>) -> Arc<Self> {
        let name = adapter.name();
        let disk_based_diagnostic_sources = adapter.disk_based_diagnostic_sources();
        let disk_based_diagnostics_progress_token = adapter.disk_based_diagnostics_progress_token();
        let language_ids = adapter.language_ids();

        Arc::new(CachedLspAdapter {
            name,
            disk_based_diagnostic_sources,
            disk_based_diagnostics_progress_token,
            language_ids,
            adapter,
            cached_binary: Default::default(),
        })
    }

    pub fn name(&self) -> LanguageServerName {
        self.adapter.name()
    }

    pub async fn get_language_server_command(
        self: Arc<Self>,
        delegate: Arc<dyn LspAdapterDelegate>,
        toolchains: Option<Toolchain>,
        binary_options: LanguageServerBinaryOptions,
        cx: &mut AsyncApp,
    ) -> LanguageServerBinaryLocations {
        let cached_binary = self.cached_binary.clone().lock_owned().await;
        self.adapter.clone().get_language_server_command(
            delegate,
            toolchains,
            binary_options,
            cached_binary,
            cx.clone(),
        )
    }

    pub fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        self.adapter.code_action_kinds()
    }

    pub fn process_diagnostics(
        &self,
        params: &mut lsp::PublishDiagnosticsParams,
        server_id: LanguageServerId,
        existing_diagnostics: Option<&'_ Buffer>,
    ) {
        self.adapter
            .process_diagnostics(params, server_id, existing_diagnostics)
    }

    pub fn retain_old_diagnostic(&self, previous_diagnostic: &Diagnostic, cx: &App) -> bool {
        self.adapter.retain_old_diagnostic(previous_diagnostic, cx)
    }

    pub fn underline_diagnostic(&self, diagnostic: &lsp::Diagnostic) -> bool {
        self.adapter.underline_diagnostic(diagnostic)
    }

    pub fn diagnostic_message_to_markdown(&self, message: &str) -> Option<String> {
        self.adapter.diagnostic_message_to_markdown(message)
    }

    pub async fn process_completions(&self, completion_items: &mut [lsp::CompletionItem]) {
        self.adapter.process_completions(completion_items).await
    }

    pub async fn labels_for_completions(
        &self,
        completion_items: &[lsp::CompletionItem],
        language: &Arc<Language>,
    ) -> Result<Vec<Option<CodeLabel>>> {
        self.adapter
            .clone()
            .labels_for_completions(completion_items, language)
            .await
    }

    pub async fn labels_for_symbols(
        &self,
        symbols: &[(String, lsp::SymbolKind)],
        language: &Arc<Language>,
    ) -> Result<Vec<Option<CodeLabel>>> {
        self.adapter
            .clone()
            .labels_for_symbols(symbols, language)
            .await
    }

    pub fn language_id(&self, language_name: &LanguageName) -> String {
        self.language_ids
            .get(language_name)
            .cloned()
            .unwrap_or_else(|| language_name.lsp_id())
    }

    pub async fn initialization_options_schema(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncApp,
    ) -> Option<serde_json::Value> {
        self.adapter
            .clone()
            .initialization_options_schema(
                delegate,
                self.cached_binary.clone().lock_owned().await,
                cx,
            )
            .await
    }

    pub fn process_prompt_response(&self, context: &PromptResponseContext, cx: &mut AsyncApp) {
        self.adapter.process_prompt_response(context, cx)
    }
}

/// [`LspAdapterDelegate`] allows [`LspAdapter]` implementations to interface with the application
// e.g. to display a notification or fetch data from the web.
#[async_trait]
pub trait LspAdapterDelegate: Send + Sync {
    fn show_notification(&self, message: &str, cx: &mut App);
    fn http_client(&self) -> Arc<dyn HttpClient>;
    fn worktree_id(&self) -> WorktreeId;
    fn worktree_root_path(&self) -> &Path;
    fn resolve_executable_path(&self, path: PathBuf) -> PathBuf;
    fn update_status(&self, language: LanguageServerName, status: BinaryStatus);
    fn registered_lsp_adapters(&self) -> Vec<Arc<dyn LspAdapter>>;
    async fn language_server_download_dir(&self, name: &LanguageServerName) -> Option<Arc<Path>>;

    async fn npm_package_installed_version(
        &self,
        package_name: &str,
    ) -> Result<Option<(PathBuf, Version)>>;
    async fn which(&self, command: &OsStr) -> Option<PathBuf>;
    async fn shell_env(&self) -> HashMap<String, String>;
    async fn read_text_file(&self, path: &RelPath) -> Result<String>;
    async fn try_exec(&self, binary: LanguageServerBinary) -> Result<()>;
}

/// Context provided to LSP adapters when a user responds to a ShowMessageRequest prompt.
/// This allows adapters to intercept preference selections (like "Always" or "Never")
/// and potentially persist them to Zed's settings.
#[derive(Debug, Clone)]
pub struct PromptResponseContext {
    /// The original message shown to the user
    pub message: String,
    /// The action (button) the user selected
    pub selected_action: lsp::MessageActionItem,
}

#[async_trait(?Send)]
pub trait LspAdapter: 'static + Send + Sync + DynLspInstaller {
    fn name(&self) -> LanguageServerName;

    fn process_diagnostics(
        &self,
        _: &mut lsp::PublishDiagnosticsParams,
        _: LanguageServerId,
        _: Option<&'_ Buffer>,
    ) {
    }

    /// When processing new `lsp::PublishDiagnosticsParams` diagnostics, whether to retain previous one(s) or not.
    fn retain_old_diagnostic(&self, _previous_diagnostic: &Diagnostic, _cx: &App) -> bool {
        false
    }

    /// Whether to underline a given diagnostic or not, when rendering in the editor.
    ///
    /// https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#diagnosticTag
    /// states that
    /// > Clients are allowed to render diagnostics with this tag faded out instead of having an error squiggle.
    /// for the unnecessary diagnostics, so do not underline them.
    fn underline_diagnostic(&self, _diagnostic: &lsp::Diagnostic) -> bool {
        true
    }

    /// Post-processes completions provided by the language server.
    async fn process_completions(&self, _: &mut [lsp::CompletionItem]) {}

    fn diagnostic_message_to_markdown(&self, _message: &str) -> Option<String> {
        None
    }

    async fn labels_for_completions(
        self: Arc<Self>,
        completions: &[lsp::CompletionItem],
        language: &Arc<Language>,
    ) -> Result<Vec<Option<CodeLabel>>> {
        let mut labels = Vec::new();
        for (ix, completion) in completions.iter().enumerate() {
            let label = self.label_for_completion(completion, language).await;
            if let Some(label) = label {
                labels.resize(ix + 1, None);
                *labels.last_mut().unwrap() = Some(label);
            }
        }
        Ok(labels)
    }

    async fn label_for_completion(
        &self,
        _: &lsp::CompletionItem,
        _: &Arc<Language>,
    ) -> Option<CodeLabel> {
        None
    }

    async fn labels_for_symbols(
        self: Arc<Self>,
        symbols: &[(String, lsp::SymbolKind)],
        language: &Arc<Language>,
    ) -> Result<Vec<Option<CodeLabel>>> {
        let mut labels = Vec::new();
        for (ix, (name, kind)) in symbols.iter().enumerate() {
            let label = self.label_for_symbol(name, *kind, language).await;
            if let Some(label) = label {
                labels.resize(ix + 1, None);
                *labels.last_mut().unwrap() = Some(label);
            }
        }
        Ok(labels)
    }

    async fn label_for_symbol(
        &self,
        _: &str,
        _: lsp::SymbolKind,
        _: &Arc<Language>,
    ) -> Option<CodeLabel> {
        None
    }

    /// Returns initialization options that are going to be sent to a LSP server as a part of [`lsp::InitializeParams`]
    async fn initialization_options(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<Value>> {
        Ok(None)
    }

    /// Returns the JSON schema of the initialization_options for the language server.
    async fn initialization_options_schema(
        self: Arc<Self>,
        _delegate: &Arc<dyn LspAdapterDelegate>,
        _cached_binary: OwnedMutexGuard<Option<(bool, LanguageServerBinary)>>,
        _cx: &mut AsyncApp,
    ) -> Option<serde_json::Value> {
        None
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
        _: Option<Uri>,
        _cx: &mut AsyncApp,
    ) -> Result<Value> {
        Ok(serde_json::json!({}))
    }

    async fn additional_initialization_options(
        self: Arc<Self>,
        _target_language_server_id: LanguageServerName,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<Value>> {
        Ok(None)
    }

    async fn additional_workspace_configuration(
        self: Arc<Self>,
        _target_language_server_id: LanguageServerName,
        _: &Arc<dyn LspAdapterDelegate>,
        _cx: &mut AsyncApp,
    ) -> Result<Option<Value>> {
        Ok(None)
    }

    /// Returns a list of code actions supported by a given LspAdapter
    fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        None
    }

    fn disk_based_diagnostic_sources(&self) -> Vec<String> {
        Default::default()
    }

    fn disk_based_diagnostics_progress_token(&self) -> Option<String> {
        None
    }

    fn language_ids(&self) -> HashMap<LanguageName, String> {
        HashMap::default()
    }

    /// Support custom initialize params.
    fn prepare_initialize_params(
        &self,
        original: InitializeParams,
        _: &App,
    ) -> Result<InitializeParams> {
        Ok(original)
    }

    /// Method only implemented by the default JSON language server adapter.
    /// Used to provide dynamic reloading of the JSON schemas used to
    /// provide autocompletion and diagnostics in Zed setting and keybind
    /// files
    fn is_primary_zed_json_schema_adapter(&self) -> bool {
        false
    }

    /// True for the extension adapter and false otherwise.
    fn is_extension(&self) -> bool {
        false
    }

    /// Called when a user responds to a ShowMessageRequest from this language server.
    /// This allows adapters to intercept preference selections (like "Always" or "Never")
    /// for settings that should be persisted to Zed's settings file.
    fn process_prompt_response(&self, _context: &PromptResponseContext, _cx: &mut AsyncApp) {}
}

pub trait LspInstaller {
    type BinaryVersion;
    fn check_if_user_installed(
        &self,
        _: &dyn LspAdapterDelegate,
        _: Option<Toolchain>,
        _: &AsyncApp,
    ) -> impl Future<Output = Option<LanguageServerBinary>> {
        async { None }
    }

    fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
        pre_release: bool,
        cx: &mut AsyncApp,
    ) -> impl Future<Output = Result<Self::BinaryVersion>>;

    fn check_if_version_installed(
        &self,
        _version: &Self::BinaryVersion,
        _container_dir: &PathBuf,
        _delegate: &dyn LspAdapterDelegate,
    ) -> impl Send + Future<Output = Option<LanguageServerBinary>> {
        async { None }
    }

    fn fetch_server_binary(
        &self,
        latest_version: Self::BinaryVersion,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> impl Send + Future<Output = Result<LanguageServerBinary>>;

    fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> impl Future<Output = Option<LanguageServerBinary>>;
}

#[async_trait(?Send)]
pub trait DynLspInstaller {
    async fn try_fetch_server_binary(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        container_dir: PathBuf,
        pre_release: bool,
        cx: &mut AsyncApp,
    ) -> Result<LanguageServerBinary>;

    fn get_language_server_command(
        self: Arc<Self>,
        delegate: Arc<dyn LspAdapterDelegate>,
        toolchains: Option<Toolchain>,
        binary_options: LanguageServerBinaryOptions,
        cached_binary: OwnedMutexGuard<Option<(bool, LanguageServerBinary)>>,
        cx: AsyncApp,
    ) -> LanguageServerBinaryLocations;
}

#[async_trait(?Send)]
impl<LI, BinaryVersion> DynLspInstaller for LI
where
    BinaryVersion: Send + Sync,
    LI: LspInstaller<BinaryVersion = BinaryVersion> + LspAdapter,
{
    async fn try_fetch_server_binary(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        container_dir: PathBuf,
        pre_release: bool,
        cx: &mut AsyncApp,
    ) -> Result<LanguageServerBinary> {
        let name = self.name();

        log::debug!("fetching latest version of language server {:?}", name.0);
        delegate.update_status(name.clone(), BinaryStatus::CheckingForUpdate);

        let latest_version = self
            .fetch_latest_server_version(delegate.as_ref(), pre_release, cx)
            .await?;

        if let Some(binary) = cx
            .background_executor()
            .await_on_background(self.check_if_version_installed(
                &latest_version,
                &container_dir,
                delegate.as_ref(),
            ))
            .await
        {
            log::debug!("language server {:?} is already installed", name.0);
            delegate.update_status(name.clone(), BinaryStatus::None);
            Ok(binary)
        } else {
            log::debug!("downloading language server {:?}", name.0);
            delegate.update_status(name.clone(), BinaryStatus::Downloading);
            let binary = cx
                .background_executor()
                .await_on_background(self.fetch_server_binary(
                    latest_version,
                    container_dir,
                    delegate.as_ref(),
                ))
                .await;

            delegate.update_status(name.clone(), BinaryStatus::None);
            binary
        }
    }
    fn get_language_server_command(
        self: Arc<Self>,
        delegate: Arc<dyn LspAdapterDelegate>,
        toolchain: Option<Toolchain>,
        binary_options: LanguageServerBinaryOptions,
        mut cached_binary: OwnedMutexGuard<Option<(bool, LanguageServerBinary)>>,
        mut cx: AsyncApp,
    ) -> LanguageServerBinaryLocations {
        async move {
            let cached_binary_deref = cached_binary.deref_mut();
            // First we check whether the adapter can give us a user-installed binary.
            // If so, we do *not* want to cache that, because each worktree might give us a different
            // binary:
            //
            //      worktree 1: user-installed at `.bin/gopls`
            //      worktree 2: user-installed at `~/bin/gopls`
            //      worktree 3: no gopls found in PATH -> fallback to Zed installation
            //
            // We only want to cache when we fall back to the global one,
            // because we don't want to download and overwrite our global one
            // for each worktree we might have open.
            if binary_options.allow_path_lookup
                && let Some(binary) = self
                    .check_if_user_installed(delegate.as_ref(), toolchain, &mut cx)
                    .await
            {
                log::info!(
                    "found user-installed language server for {}. path: {:?}, arguments: {:?}",
                    self.name().0,
                    binary.path,
                    binary.arguments
                );
                return (Ok(binary), None);
            }

            if let Some((pre_release, cached_binary)) = cached_binary_deref
                && *pre_release == binary_options.pre_release
            {
                return (Ok(cached_binary.clone()), None);
            }

            if !binary_options.allow_binary_download {
                return (
                    Err(anyhow::anyhow!("downloading language servers disabled")),
                    None,
                );
            }

            let Some(container_dir) = delegate.language_server_download_dir(&self.name()).await
            else {
                return (
                    Err(anyhow::anyhow!("no language server download dir defined")),
                    None,
                );
            };

            let last_downloaded_binary = self
                .cached_server_binary(container_dir.to_path_buf(), delegate.as_ref())
                .await
                .context(
                    "did not find existing language server binary, falling back to downloading",
                );
            let download_binary = async move {
                let mut binary = self
                    .try_fetch_server_binary(
                        &delegate,
                        container_dir.to_path_buf(),
                        binary_options.pre_release,
                        &mut cx,
                    )
                    .await;

                if let Err(error) = binary.as_ref() {
                    if let Some(prev_downloaded_binary) = self
                        .cached_server_binary(container_dir.to_path_buf(), delegate.as_ref())
                        .await
                    {
                        log::info!(
                            "failed to fetch newest version of language server {:?}. \
                            error: {:?}, falling back to using {:?}",
                            self.name(),
                            error,
                            prev_downloaded_binary.path
                        );
                        binary = Ok(prev_downloaded_binary);
                    } else {
                        delegate.update_status(
                            self.name(),
                            BinaryStatus::Failed {
                                error: format!("{error:?}"),
                            },
                        );
                    }
                }

                if let Ok(binary) = &binary {
                    *cached_binary = Some((binary_options.pre_release, binary.clone()));
                }

                binary
            }
            .boxed_local();
            (last_downloaded_binary, Some(download_binary))
        }
        .boxed_local()
    }
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CodeLabel {
    /// The text to display.
    pub text: String,
    /// Syntax highlighting runs.
    pub runs: Vec<(Range<usize>, HighlightId)>,
    /// The portion of the text that should be used in fuzzy filtering.
    pub filter_range: Range<usize>,
}

#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct CodeLabelBuilder {
    /// The text to display.
    text: String,
    /// Syntax highlighting runs.
    runs: Vec<(Range<usize>, HighlightId)>,
    /// The portion of the text that should be used in fuzzy filtering.
    filter_range: Range<usize>,
}

#[derive(Clone, Deserialize, JsonSchema, Debug)]
pub struct LanguageConfig {
    /// Human-readable name of the language.
    pub name: LanguageName,
    /// The name of this language for a Markdown code fence block
    pub code_fence_block_name: Option<Arc<str>>,
    // The name of the grammar in a WASM bundle (experimental).
    pub grammar: Option<Arc<str>>,
    /// The criteria for matching this language to a given file.
    #[serde(flatten)]
    pub matcher: LanguageMatcher,
    /// List of bracket types in a language.
    #[serde(default)]
    pub brackets: BracketPairConfig,
    /// If set to true, auto indentation uses last non empty line to determine
    /// the indentation level for a new line.
    #[serde(default = "auto_indent_using_last_non_empty_line_default")]
    pub auto_indent_using_last_non_empty_line: bool,
    // Whether indentation of pasted content should be adjusted based on the context.
    #[serde(default)]
    pub auto_indent_on_paste: Option<bool>,
    /// A regex that is used to determine whether the indentation level should be
    /// increased in the following line.
    #[serde(default, deserialize_with = "deserialize_regex")]
    #[schemars(schema_with = "regex_json_schema")]
    pub increase_indent_pattern: Option<Regex>,
    /// A regex that is used to determine whether the indentation level should be
    /// decreased in the following line.
    #[serde(default, deserialize_with = "deserialize_regex")]
    #[schemars(schema_with = "regex_json_schema")]
    pub decrease_indent_pattern: Option<Regex>,
    /// A list of rules for decreasing indentation. Each rule pairs a regex with a set of valid
    /// "block-starting" tokens. When a line matches a pattern, its indentation is aligned with
    /// the most recent line that began with a corresponding token. This enables context-aware
    /// outdenting, like aligning an `else` with its `if`.
    #[serde(default)]
    pub decrease_indent_patterns: Vec<DecreaseIndentConfig>,
    /// A list of characters that trigger the automatic insertion of a closing
    /// bracket when they immediately precede the point where an opening
    /// bracket is inserted.
    #[serde(default)]
    pub autoclose_before: String,
    /// A placeholder used internally by Semantic Index.
    #[serde(default)]
    pub collapsed_placeholder: String,
    /// A line comment string that is inserted in e.g. `toggle comments` action.
    /// A language can have multiple flavours of line comments. All of the provided line comments are
    /// used for comment continuations on the next line, but only the first one is used for Editor::ToggleComments.
    #[serde(default)]
    pub line_comments: Vec<Arc<str>>,
    /// Delimiters and configuration for recognizing and formatting block comments.
    #[serde(default)]
    pub block_comment: Option<BlockCommentConfig>,
    /// Delimiters and configuration for recognizing and formatting documentation comments.
    #[serde(default, alias = "documentation")]
    pub documentation_comment: Option<BlockCommentConfig>,
    /// List markers that are inserted unchanged on newline (e.g., `- `, `* `, `+ `).
    #[serde(default)]
    pub unordered_list: Vec<Arc<str>>,
    /// Configuration for ordered lists with auto-incrementing numbers on newline (e.g., `1. ` becomes `2. `).
    #[serde(default)]
    pub ordered_list: Vec<OrderedListConfig>,
    /// Configuration for task lists where multiple markers map to a single continuation prefix (e.g., `- [x] ` continues as `- [ ] `).
    #[serde(default)]
    pub task_list: Option<TaskListConfig>,
    /// A list of additional regex patterns that should be treated as prefixes
    /// for creating boundaries during rewrapping, ensuring content from one
    /// prefixed section doesn't merge with another (e.g., markdown list items).
    /// By default, Zed treats as paragraph and comment prefixes as boundaries.
    #[serde(default, deserialize_with = "deserialize_regex_vec")]
    #[schemars(schema_with = "regex_vec_json_schema")]
    pub rewrap_prefixes: Vec<Regex>,
    /// A list of language servers that are allowed to run on subranges of a given language.
    #[serde(default)]
    pub scope_opt_in_language_servers: Vec<LanguageServerName>,
    #[serde(default)]
    pub overrides: HashMap<String, LanguageConfigOverride>,
    /// A list of characters that Zed should treat as word characters for the
    /// purpose of features that operate on word boundaries, like 'move to next word end'
    /// or a whole-word search in buffer search.
    #[serde(default)]
    pub word_characters: HashSet<char>,
    /// Whether to indent lines using tab characters, as opposed to multiple
    /// spaces.
    #[serde(default)]
    pub hard_tabs: Option<bool>,
    /// How many columns a tab should occupy.
    #[serde(default)]
    #[schemars(range(min = 1, max = 128))]
    pub tab_size: Option<NonZeroU32>,
    /// How to soft-wrap long lines of text.
    #[serde(default)]
    pub soft_wrap: Option<SoftWrap>,
    /// When set, selections can be wrapped using prefix/suffix pairs on both sides.
    #[serde(default)]
    pub wrap_characters: Option<WrapCharactersConfig>,
    /// The name of a Prettier parser that will be used for this language when no file path is available.
    /// If there's a parser name in the language settings, that will be used instead.
    #[serde(default)]
    pub prettier_parser_name: Option<String>,
    /// If true, this language is only for syntax highlighting via an injection into other
    /// languages, but should not appear to the user as a distinct language.
    #[serde(default)]
    pub hidden: bool,
    /// If configured, this language contains JSX style tags, and should support auto-closing of those tags.
    #[serde(default)]
    pub jsx_tag_auto_close: Option<JsxTagAutoCloseConfig>,
    /// A list of characters that Zed should treat as word characters for completion queries.
    #[serde(default)]
    pub completion_query_characters: HashSet<char>,
    /// A list of characters that Zed should treat as word characters for linked edit operations.
    #[serde(default)]
    pub linked_edit_characters: HashSet<char>,
    /// A list of preferred debuggers for this language.
    #[serde(default)]
    pub debuggers: IndexSet<SharedString>,
    /// A list of import namespace segments that aren't expected to appear in file paths. For
    /// example, "super" and "crate" in Rust.
    #[serde(default)]
    pub ignored_import_segments: HashSet<Arc<str>>,
    /// Regular expression that matches substrings to omit from import paths, to make the paths more
    /// similar to how they are specified when imported. For example, "/mod\.rs$" or "/__init__\.py$".
    #[serde(default, deserialize_with = "deserialize_regex")]
    #[schemars(schema_with = "regex_json_schema")]
    pub import_path_strip_regex: Option<Regex>,
}

#[derive(Clone, Debug, Deserialize, Default, JsonSchema)]
pub struct DecreaseIndentConfig {
    #[serde(default, deserialize_with = "deserialize_regex")]
    #[schemars(schema_with = "regex_json_schema")]
    pub pattern: Option<Regex>,
    #[serde(default)]
    pub valid_after: Vec<String>,
}

/// Configuration for continuing ordered lists with auto-incrementing numbers.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct OrderedListConfig {
    /// A regex pattern with a capture group for the number portion (e.g., `(\\d+)\\. `).
    pub pattern: String,
    /// A format string where `{1}` is replaced with the incremented number (e.g., `{1}. `).
    pub format: String,
}

/// Configuration for continuing task lists on newline.
#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct TaskListConfig {
    /// The list markers to match (e.g., `- [ ] `, `- [x] `).
    pub prefixes: Vec<Arc<str>>,
    /// The marker to insert when continuing the list on a new line (e.g., `- [ ] `).
    pub continuation: Arc<str>,
}

#[derive(Clone, Debug, Serialize, Deserialize, Default, JsonSchema)]
pub struct LanguageMatcher {
    /// Given a list of `LanguageConfig`'s, the language of a file can be determined based on the path extension matching any of the `path_suffixes`.
    #[serde(default)]
    pub path_suffixes: Vec<String>,
    /// A regex pattern that determines whether the language should be assigned to a file or not.
    #[serde(
        default,
        serialize_with = "serialize_regex",
        deserialize_with = "deserialize_regex"
    )]
    #[schemars(schema_with = "regex_json_schema")]
    pub first_line_pattern: Option<Regex>,
}

/// The configuration for JSX tag auto-closing.
#[derive(Clone, Deserialize, JsonSchema, Debug)]
pub struct JsxTagAutoCloseConfig {
    /// The name of the node for a opening tag
    pub open_tag_node_name: String,
    /// The name of the node for an closing tag
    pub close_tag_node_name: String,
    /// The name of the node for a complete element with children for open and close tags
    pub jsx_element_node_name: String,
    /// The name of the node found within both opening and closing
    /// tags that describes the tag name
    pub tag_name_node_name: String,
    /// Alternate Node names for tag names.
    /// Specifically needed as TSX represents the name in `<Foo.Bar>`
    /// as `member_expression` rather than `identifier` as usual
    #[serde(default)]
    pub tag_name_node_name_alternates: Vec<String>,
    /// Some grammars are smart enough to detect a closing tag
    /// that is not valid i.e. doesn't match it's corresponding
    /// opening tag or does not have a corresponding opening tag
    /// This should be set to the name of the node for invalid
    /// closing tags if the grammar contains such a node, otherwise
    /// detecting already closed tags will not work properly
    #[serde(default)]
    pub erroneous_close_tag_node_name: Option<String>,
    /// See above for erroneous_close_tag_node_name for details
    /// This should be set if the node used for the tag name
    /// within erroneous closing tags is different from the
    /// normal tag name node name
    #[serde(default)]
    pub erroneous_close_tag_name_node_name: Option<String>,
}

/// The configuration for block comments for this language.
#[derive(Clone, Debug, JsonSchema, PartialEq)]
pub struct BlockCommentConfig {
    /// A start tag of block comment.
    pub start: Arc<str>,
    /// A end tag of block comment.
    pub end: Arc<str>,
    /// A character to add as a prefix when a new line is added to a block comment.
    pub prefix: Arc<str>,
    /// A indent to add for prefix and end line upon new line.
    #[schemars(range(min = 1, max = 128))]
    pub tab_size: u32,
}

impl<'de> Deserialize<'de> for BlockCommentConfig {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        #[serde(untagged)]
        enum BlockCommentConfigHelper {
            New {
                start: Arc<str>,
                end: Arc<str>,
                prefix: Arc<str>,
                tab_size: u32,
            },
            Old([Arc<str>; 2]),
        }

        match BlockCommentConfigHelper::deserialize(deserializer)? {
            BlockCommentConfigHelper::New {
                start,
                end,
                prefix,
                tab_size,
            } => Ok(BlockCommentConfig {
                start,
                end,
                prefix,
                tab_size,
            }),
            BlockCommentConfigHelper::Old([start, end]) => Ok(BlockCommentConfig {
                start,
                end,
                prefix: "".into(),
                tab_size: 0,
            }),
        }
    }
}

/// Represents a language for the given range. Some languages (e.g. HTML)
/// interleave several languages together, thus a single buffer might actually contain
/// several nested scopes.
#[derive(Clone, Debug)]
pub struct LanguageScope {
    language: Arc<Language>,
    override_id: Option<u32>,
}

#[derive(Clone, Deserialize, Default, Debug, JsonSchema)]
pub struct LanguageConfigOverride {
    #[serde(default)]
    pub line_comments: Override<Vec<Arc<str>>>,
    #[serde(default)]
    pub block_comment: Override<BlockCommentConfig>,
    #[serde(skip)]
    pub disabled_bracket_ixs: Vec<u16>,
    #[serde(default)]
    pub word_characters: Override<HashSet<char>>,
    #[serde(default)]
    pub completion_query_characters: Override<HashSet<char>>,
    #[serde(default)]
    pub linked_edit_characters: Override<HashSet<char>>,
    #[serde(default)]
    pub opt_into_language_servers: Vec<LanguageServerName>,
    #[serde(default)]
    pub prefer_label_for_snippet: Option<bool>,
}

#[derive(Clone, Deserialize, Debug, Serialize, JsonSchema)]
#[serde(untagged)]
pub enum Override<T> {
    Remove { remove: bool },
    Set(T),
}

impl<T> Default for Override<T> {
    fn default() -> Self {
        Override::Remove { remove: false }
    }
}

impl<T> Override<T> {
    fn as_option<'a>(this: Option<&'a Self>, original: Option<&'a T>) -> Option<&'a T> {
        match this {
            Some(Self::Set(value)) => Some(value),
            Some(Self::Remove { remove: true }) => None,
            Some(Self::Remove { remove: false }) | None => original,
        }
    }
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            name: LanguageName::new_static(""),
            code_fence_block_name: None,
            grammar: None,
            matcher: LanguageMatcher::default(),
            brackets: Default::default(),
            auto_indent_using_last_non_empty_line: auto_indent_using_last_non_empty_line_default(),
            auto_indent_on_paste: None,
            increase_indent_pattern: Default::default(),
            decrease_indent_pattern: Default::default(),
            decrease_indent_patterns: Default::default(),
            autoclose_before: Default::default(),
            line_comments: Default::default(),
            block_comment: Default::default(),
            documentation_comment: Default::default(),
            unordered_list: Default::default(),
            ordered_list: Default::default(),
            task_list: Default::default(),
            rewrap_prefixes: Default::default(),
            scope_opt_in_language_servers: Default::default(),
            overrides: Default::default(),
            word_characters: Default::default(),
            collapsed_placeholder: Default::default(),
            hard_tabs: None,
            tab_size: None,
            soft_wrap: None,
            wrap_characters: None,
            prettier_parser_name: None,
            hidden: false,
            jsx_tag_auto_close: None,
            completion_query_characters: Default::default(),
            linked_edit_characters: Default::default(),
            debuggers: Default::default(),
            ignored_import_segments: Default::default(),
            import_path_strip_regex: None,
        }
    }
}

#[derive(Clone, Debug, Deserialize, JsonSchema)]
pub struct WrapCharactersConfig {
    /// Opening token split into a prefix and suffix. The first caret goes
    /// after the prefix (i.e., between prefix and suffix).
    pub start_prefix: String,
    pub start_suffix: String,
    /// Closing token split into a prefix and suffix. The second caret goes
    /// after the prefix (i.e., between prefix and suffix).
    pub end_prefix: String,
    pub end_suffix: String,
}

fn auto_indent_using_last_non_empty_line_default() -> bool {
    true
}

fn deserialize_regex<'de, D: Deserializer<'de>>(d: D) -> Result<Option<Regex>, D::Error> {
    let source = Option::<String>::deserialize(d)?;
    if let Some(source) = source {
        Ok(Some(regex::Regex::new(&source).map_err(de::Error::custom)?))
    } else {
        Ok(None)
    }
}

fn regex_json_schema(_: &mut schemars::SchemaGenerator) -> schemars::Schema {
    json_schema!({
        "type": "string"
    })
}

fn serialize_regex<S>(regex: &Option<Regex>, serializer: S) -> Result<S::Ok, S::Error>
where
    S: Serializer,
{
    match regex {
        Some(regex) => serializer.serialize_str(regex.as_str()),
        None => serializer.serialize_none(),
    }
}

fn deserialize_regex_vec<'de, D: Deserializer<'de>>(d: D) -> Result<Vec<Regex>, D::Error> {
    let sources = Vec::<String>::deserialize(d)?;
    sources
        .into_iter()
        .map(|source| regex::Regex::new(&source))
        .collect::<Result<_, _>>()
        .map_err(de::Error::custom)
}

fn regex_vec_json_schema(_: &mut SchemaGenerator) -> schemars::Schema {
    json_schema!({
        "type": "array",
        "items": { "type": "string" }
    })
}

#[doc(hidden)]
#[cfg(any(test, feature = "test-support"))]
pub struct FakeLspAdapter {
    pub name: &'static str,
    pub initialization_options: Option<Value>,
    pub prettier_plugins: Vec<&'static str>,
    pub disk_based_diagnostics_progress_token: Option<String>,
    pub disk_based_diagnostics_sources: Vec<String>,
    pub language_server_binary: LanguageServerBinary,

    pub capabilities: lsp::ServerCapabilities,
    pub initializer: Option<Box<dyn 'static + Send + Sync + Fn(&mut lsp::FakeLanguageServer)>>,
    pub label_for_completion: Option<
        Box<
            dyn 'static
                + Send
                + Sync
                + Fn(&lsp::CompletionItem, &Arc<Language>) -> Option<CodeLabel>,
        >,
    >,
}

/// Configuration of handling bracket pairs for a given language.
///
/// This struct includes settings for defining which pairs of characters are considered brackets and
/// also specifies any language-specific scopes where these pairs should be ignored for bracket matching purposes.
#[derive(Clone, Debug, Default, JsonSchema)]
#[schemars(with = "Vec::<BracketPairContent>")]
pub struct BracketPairConfig {
    /// A list of character pairs that should be treated as brackets in the context of a given language.
    pub pairs: Vec<BracketPair>,
    /// A list of tree-sitter scopes for which a given bracket should not be active.
    /// N-th entry in `[Self::disabled_scopes_by_bracket_ix]` contains a list of disabled scopes for an n-th entry in `[Self::pairs]`
    pub disabled_scopes_by_bracket_ix: Vec<Vec<String>>,
}

impl BracketPairConfig {
    pub fn is_closing_brace(&self, c: char) -> bool {
        self.pairs.iter().any(|pair| pair.end.starts_with(c))
    }
}

#[derive(Deserialize, JsonSchema)]
pub struct BracketPairContent {
    #[serde(flatten)]
    pub bracket_pair: BracketPair,
    #[serde(default)]
    pub not_in: Vec<String>,
}

impl<'de> Deserialize<'de> for BracketPairConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        let result = Vec::<BracketPairContent>::deserialize(deserializer)?;
        let (brackets, disabled_scopes_by_bracket_ix) = result
            .into_iter()
            .map(|entry| (entry.bracket_pair, entry.not_in))
            .unzip();

        Ok(BracketPairConfig {
            pairs: brackets,
            disabled_scopes_by_bracket_ix,
        })
    }
}

/// Describes a single bracket pair and how an editor should react to e.g. inserting
/// an opening bracket or to a newline character insertion in between `start` and `end` characters.
#[derive(Clone, Debug, Default, Deserialize, PartialEq, JsonSchema)]
pub struct BracketPair {
    /// Starting substring for a bracket.
    pub start: String,
    /// Ending substring for a bracket.
    pub end: String,
    /// True if `end` should be automatically inserted right after `start` characters.
    pub close: bool,
    /// True if selected text should be surrounded by `start` and `end` characters.
    #[serde(default = "default_true")]
    pub surround: bool,
    /// True if an extra newline should be inserted while the cursor is in the middle
    /// of that bracket pair.
    pub newline: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct LanguageId(usize);

impl LanguageId {
    pub(crate) fn new() -> Self {
        Self(NEXT_LANGUAGE_ID.fetch_add(1, SeqCst))
    }
}

pub struct Language {
    pub(crate) id: LanguageId,
    pub(crate) config: LanguageConfig,
    pub(crate) grammar: Option<Arc<Grammar>>,
    pub(crate) context_provider: Option<Arc<dyn ContextProvider>>,
    pub(crate) toolchain: Option<Arc<dyn ToolchainLister>>,
    pub(crate) manifest_name: Option<ManifestName>,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub struct GrammarId(pub usize);

impl GrammarId {
    pub(crate) fn new() -> Self {
        Self(NEXT_GRAMMAR_ID.fetch_add(1, SeqCst))
    }
}

pub struct Grammar {
    id: GrammarId,
    pub ts_language: tree_sitter::Language,
    pub(crate) error_query: Option<Query>,
    pub highlights_config: Option<HighlightsConfig>,
    pub(crate) brackets_config: Option<BracketsConfig>,
    pub(crate) redactions_config: Option<RedactionConfig>,
    pub(crate) runnable_config: Option<RunnableConfig>,
    pub(crate) indents_config: Option<IndentConfig>,
    pub outline_config: Option<OutlineConfig>,
    pub text_object_config: Option<TextObjectConfig>,
    pub(crate) injection_config: Option<InjectionConfig>,
    pub(crate) override_config: Option<OverrideConfig>,
    pub(crate) debug_variables_config: Option<DebugVariablesConfig>,
    pub(crate) imports_config: Option<ImportsConfig>,
    pub(crate) highlight_map: Mutex<HighlightMap>,
}

pub struct HighlightsConfig {
    pub query: Query,
    pub identifier_capture_indices: Vec<u32>,
}

struct IndentConfig {
    query: Query,
    indent_capture_ix: u32,
    start_capture_ix: Option<u32>,
    end_capture_ix: Option<u32>,
    outdent_capture_ix: Option<u32>,
    suffixed_start_captures: HashMap<u32, SharedString>,
}

pub struct OutlineConfig {
    pub query: Query,
    pub item_capture_ix: u32,
    pub name_capture_ix: u32,
    pub context_capture_ix: Option<u32>,
    pub extra_context_capture_ix: Option<u32>,
    pub open_capture_ix: Option<u32>,
    pub close_capture_ix: Option<u32>,
    pub annotation_capture_ix: Option<u32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum DebuggerTextObject {
    Variable,
    Scope,
}

impl DebuggerTextObject {
    pub fn from_capture_name(name: &str) -> Option<DebuggerTextObject> {
        match name {
            "debug-variable" => Some(DebuggerTextObject::Variable),
            "debug-scope" => Some(DebuggerTextObject::Scope),
            _ => None,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum TextObject {
    InsideFunction,
    AroundFunction,
    InsideClass,
    AroundClass,
    InsideComment,
    AroundComment,
}

impl TextObject {
    pub fn from_capture_name(name: &str) -> Option<TextObject> {
        match name {
            "function.inside" => Some(TextObject::InsideFunction),
            "function.around" => Some(TextObject::AroundFunction),
            "class.inside" => Some(TextObject::InsideClass),
            "class.around" => Some(TextObject::AroundClass),
            "comment.inside" => Some(TextObject::InsideComment),
            "comment.around" => Some(TextObject::AroundComment),
            _ => None,
        }
    }

    pub fn around(&self) -> Option<Self> {
        match self {
            TextObject::InsideFunction => Some(TextObject::AroundFunction),
            TextObject::InsideClass => Some(TextObject::AroundClass),
            TextObject::InsideComment => Some(TextObject::AroundComment),
            _ => None,
        }
    }
}

pub struct TextObjectConfig {
    pub query: Query,
    pub text_objects_by_capture_ix: Vec<(u32, TextObject)>,
}

struct InjectionConfig {
    query: Query,
    content_capture_ix: u32,
    language_capture_ix: Option<u32>,
    patterns: Vec<InjectionPatternConfig>,
}

struct RedactionConfig {
    pub query: Query,
    pub redaction_capture_ix: u32,
}

#[derive(Clone, Debug, PartialEq)]
enum RunnableCapture {
    Named(SharedString),
    Run,
}

struct RunnableConfig {
    pub query: Query,
    /// A mapping from capture indice to capture kind
    pub extra_captures: Vec<RunnableCapture>,
}

struct OverrideConfig {
    query: Query,
    values: HashMap<u32, OverrideEntry>,
}

#[derive(Debug)]
struct OverrideEntry {
    name: String,
    range_is_inclusive: bool,
    value: LanguageConfigOverride,
}

#[derive(Default, Clone)]
struct InjectionPatternConfig {
    language: Option<Box<str>>,
    combined: bool,
}

#[derive(Debug)]
struct BracketsConfig {
    query: Query,
    open_capture_ix: u32,
    close_capture_ix: u32,
    patterns: Vec<BracketsPatternConfig>,
}

#[derive(Clone, Debug, Default)]
struct BracketsPatternConfig {
    newline_only: bool,
    rainbow_exclude: bool,
}

pub struct DebugVariablesConfig {
    pub query: Query,
    pub objects_by_capture_ix: Vec<(u32, DebuggerTextObject)>,
}

pub struct ImportsConfig {
    pub query: Query,
    pub import_ix: u32,
    pub name_ix: Option<u32>,
    pub namespace_ix: Option<u32>,
    pub source_ix: Option<u32>,
    pub list_ix: Option<u32>,
    pub wildcard_ix: Option<u32>,
    pub alias_ix: Option<u32>,
}

impl Language {
    pub fn new(config: LanguageConfig, ts_language: Option<tree_sitter::Language>) -> Self {
        Self::new_with_id(LanguageId::new(), config, ts_language)
    }

    pub fn id(&self) -> LanguageId {
        self.id
    }

    fn new_with_id(
        id: LanguageId,
        config: LanguageConfig,
        ts_language: Option<tree_sitter::Language>,
    ) -> Self {
        Self {
            id,
            config,
            grammar: ts_language.map(|ts_language| {
                Arc::new(Grammar {
                    id: GrammarId::new(),
                    highlights_config: None,
                    brackets_config: None,
                    outline_config: None,
                    text_object_config: None,
                    indents_config: None,
                    injection_config: None,
                    override_config: None,
                    redactions_config: None,
                    runnable_config: None,
                    error_query: Query::new(&ts_language, "(ERROR) @error").ok(),
                    debug_variables_config: None,
                    imports_config: None,
                    ts_language,
                    highlight_map: Default::default(),
                })
            }),
            context_provider: None,
            toolchain: None,
            manifest_name: None,
        }
    }

    pub fn with_context_provider(mut self, provider: Option<Arc<dyn ContextProvider>>) -> Self {
        self.context_provider = provider;
        self
    }

    pub fn with_toolchain_lister(mut self, provider: Option<Arc<dyn ToolchainLister>>) -> Self {
        self.toolchain = provider;
        self
    }

    pub fn with_manifest(mut self, name: Option<ManifestName>) -> Self {
        self.manifest_name = name;
        self
    }

    pub fn with_queries(mut self, queries: LanguageQueries) -> Result<Self> {
        if let Some(query) = queries.highlights {
            self = self
                .with_highlights_query(query.as_ref())
                .context("Error loading highlights query")?;
        }
        if let Some(query) = queries.brackets {
            self = self
                .with_brackets_query(query.as_ref())
                .context("Error loading brackets query")?;
        }
        if let Some(query) = queries.indents {
            self = self
                .with_indents_query(query.as_ref())
                .context("Error loading indents query")?;
        }
        if let Some(query) = queries.outline {
            self = self
                .with_outline_query(query.as_ref())
                .context("Error loading outline query")?;
        }
        if let Some(query) = queries.injections {
            self = self
                .with_injection_query(query.as_ref())
                .context("Error loading injection query")?;
        }
        if let Some(query) = queries.overrides {
            self = self
                .with_override_query(query.as_ref())
                .context("Error loading override query")?;
        }
        if let Some(query) = queries.redactions {
            self = self
                .with_redaction_query(query.as_ref())
                .context("Error loading redaction query")?;
        }
        if let Some(query) = queries.runnables {
            self = self
                .with_runnable_query(query.as_ref())
                .context("Error loading runnables query")?;
        }
        if let Some(query) = queries.text_objects {
            self = self
                .with_text_object_query(query.as_ref())
                .context("Error loading textobject query")?;
        }
        if let Some(query) = queries.debugger {
            self = self
                .with_debug_variables_query(query.as_ref())
                .context("Error loading debug variables query")?;
        }
        if let Some(query) = queries.imports {
            self = self
                .with_imports_query(query.as_ref())
                .context("Error loading imports query")?;
        }
        Ok(self)
    }

    pub fn with_highlights_query(mut self, source: &str) -> Result<Self> {
        let grammar = self.grammar_mut()?;
        let query = Query::new(&grammar.ts_language, source)?;

        let mut identifier_capture_indices = Vec::new();
        for name in [
            "variable",
            "constant",
            "constructor",
            "function",
            "function.method",
            "function.method.call",
            "function.special",
            "property",
            "type",
            "type.interface",
        ] {
            identifier_capture_indices.extend(query.capture_index_for_name(name));
        }

        grammar.highlights_config = Some(HighlightsConfig {
            query,
            identifier_capture_indices,
        });

        Ok(self)
    }

    pub fn with_runnable_query(mut self, source: &str) -> Result<Self> {
        let grammar = self.grammar_mut()?;

        let query = Query::new(&grammar.ts_language, source)?;
        let extra_captures: Vec<_> = query
            .capture_names()
            .iter()
            .map(|&name| match name {
                "run" => RunnableCapture::Run,
                name => RunnableCapture::Named(name.to_string().into()),
            })
            .collect();

        grammar.runnable_config = Some(RunnableConfig {
            extra_captures,
            query,
        });

        Ok(self)
    }

    pub fn with_outline_query(mut self, source: &str) -> Result<Self> {
        let query = Query::new(&self.expect_grammar()?.ts_language, source)?;
        let mut item_capture_ix = 0;
        let mut name_capture_ix = 0;
        let mut context_capture_ix = None;
        let mut extra_context_capture_ix = None;
        let mut open_capture_ix = None;
        let mut close_capture_ix = None;
        let mut annotation_capture_ix = None;
        if populate_capture_indices(
            &query,
            &self.config.name,
            "outline",
            &[],
            &mut [
                Capture::Required("item", &mut item_capture_ix),
                Capture::Required("name", &mut name_capture_ix),
                Capture::Optional("context", &mut context_capture_ix),
                Capture::Optional("context.extra", &mut extra_context_capture_ix),
                Capture::Optional("open", &mut open_capture_ix),
                Capture::Optional("close", &mut close_capture_ix),
                Capture::Optional("annotation", &mut annotation_capture_ix),
            ],
        ) {
            self.grammar_mut()?.outline_config = Some(OutlineConfig {
                query,
                item_capture_ix,
                name_capture_ix,
                context_capture_ix,
                extra_context_capture_ix,
                open_capture_ix,
                close_capture_ix,
                annotation_capture_ix,
            });
        }
        Ok(self)
    }

    pub fn with_text_object_query(mut self, source: &str) -> Result<Self> {
        let query = Query::new(&self.expect_grammar()?.ts_language, source)?;

        let mut text_objects_by_capture_ix = Vec::new();
        for (ix, name) in query.capture_names().iter().enumerate() {
            if let Some(text_object) = TextObject::from_capture_name(name) {
                text_objects_by_capture_ix.push((ix as u32, text_object));
            } else {
                log::warn!(
                    "unrecognized capture name '{}' in {} textobjects TreeSitter query",
                    name,
                    self.config.name,
                );
            }
        }

        self.grammar_mut()?.text_object_config = Some(TextObjectConfig {
            query,
            text_objects_by_capture_ix,
        });
        Ok(self)
    }

    pub fn with_debug_variables_query(mut self, source: &str) -> Result<Self> {
        let query = Query::new(&self.expect_grammar()?.ts_language, source)?;

        let mut objects_by_capture_ix = Vec::new();
        for (ix, name) in query.capture_names().iter().enumerate() {
            if let Some(text_object) = DebuggerTextObject::from_capture_name(name) {
                objects_by_capture_ix.push((ix as u32, text_object));
            } else {
                log::warn!(
                    "unrecognized capture name '{}' in {} debugger TreeSitter query",
                    name,
                    self.config.name,
                );
            }
        }

        self.grammar_mut()?.debug_variables_config = Some(DebugVariablesConfig {
            query,
            objects_by_capture_ix,
        });
        Ok(self)
    }

    pub fn with_imports_query(mut self, source: &str) -> Result<Self> {
        let query = Query::new(&self.expect_grammar()?.ts_language, source)?;

        let mut import_ix = 0;
        let mut name_ix = None;
        let mut namespace_ix = None;
        let mut source_ix = None;
        let mut list_ix = None;
        let mut wildcard_ix = None;
        let mut alias_ix = None;
        if populate_capture_indices(
            &query,
            &self.config.name,
            "imports",
            &[],
            &mut [
                Capture::Required("import", &mut import_ix),
                Capture::Optional("name", &mut name_ix),
                Capture::Optional("namespace", &mut namespace_ix),
                Capture::Optional("source", &mut source_ix),
                Capture::Optional("list", &mut list_ix),
                Capture::Optional("wildcard", &mut wildcard_ix),
                Capture::Optional("alias", &mut alias_ix),
            ],
        ) {
            self.grammar_mut()?.imports_config = Some(ImportsConfig {
                query,
                import_ix,
                name_ix,
                namespace_ix,
                source_ix,
                list_ix,
                wildcard_ix,
                alias_ix,
            });
        }
        return Ok(self);
    }

    pub fn with_brackets_query(mut self, source: &str) -> Result<Self> {
        let query = Query::new(&self.expect_grammar()?.ts_language, source)?;
        let mut open_capture_ix = 0;
        let mut close_capture_ix = 0;
        if populate_capture_indices(
            &query,
            &self.config.name,
            "brackets",
            &[],
            &mut [
                Capture::Required("open", &mut open_capture_ix),
                Capture::Required("close", &mut close_capture_ix),
            ],
        ) {
            let patterns = (0..query.pattern_count())
                .map(|ix| {
                    let mut config = BracketsPatternConfig::default();
                    for setting in query.property_settings(ix) {
                        let setting_key = setting.key.as_ref();
                        if setting_key == "newline.only" {
                            config.newline_only = true
                        }
                        if setting_key == "rainbow.exclude" {
                            config.rainbow_exclude = true
                        }
                    }
                    config
                })
                .collect();
            self.grammar_mut()?.brackets_config = Some(BracketsConfig {
                query,
                open_capture_ix,
                close_capture_ix,
                patterns,
            });
        }
        Ok(self)
    }

    pub fn with_indents_query(mut self, source: &str) -> Result<Self> {
        let query = Query::new(&self.expect_grammar()?.ts_language, source)?;
        let mut indent_capture_ix = 0;
        let mut start_capture_ix = None;
        let mut end_capture_ix = None;
        let mut outdent_capture_ix = None;
        if populate_capture_indices(
            &query,
            &self.config.name,
            "indents",
            &["start."],
            &mut [
                Capture::Required("indent", &mut indent_capture_ix),
                Capture::Optional("start", &mut start_capture_ix),
                Capture::Optional("end", &mut end_capture_ix),
                Capture::Optional("outdent", &mut outdent_capture_ix),
            ],
        ) {
            let mut suffixed_start_captures = HashMap::default();
            for (ix, name) in query.capture_names().iter().enumerate() {
                if let Some(suffix) = name.strip_prefix("start.") {
                    suffixed_start_captures.insert(ix as u32, suffix.to_owned().into());
                }
            }

            self.grammar_mut()?.indents_config = Some(IndentConfig {
                query,
                indent_capture_ix,
                start_capture_ix,
                end_capture_ix,
                outdent_capture_ix,
                suffixed_start_captures,
            });
        }
        Ok(self)
    }

    pub fn with_injection_query(mut self, source: &str) -> Result<Self> {
        let query = Query::new(&self.expect_grammar()?.ts_language, source)?;
        let mut language_capture_ix = None;
        let mut injection_language_capture_ix = None;
        let mut content_capture_ix = None;
        let mut injection_content_capture_ix = None;
        if populate_capture_indices(
            &query,
            &self.config.name,
            "injections",
            &[],
            &mut [
                Capture::Optional("language", &mut language_capture_ix),
                Capture::Optional("injection.language", &mut injection_language_capture_ix),
                Capture::Optional("content", &mut content_capture_ix),
                Capture::Optional("injection.content", &mut injection_content_capture_ix),
            ],
        ) {
            language_capture_ix = match (language_capture_ix, injection_language_capture_ix) {
                (None, Some(ix)) => Some(ix),
                (Some(_), Some(_)) => {
                    anyhow::bail!("both language and injection.language captures are present");
                }
                _ => language_capture_ix,
            };
            content_capture_ix = match (content_capture_ix, injection_content_capture_ix) {
                (None, Some(ix)) => Some(ix),
                (Some(_), Some(_)) => {
                    anyhow::bail!("both content and injection.content captures are present")
                }
                _ => content_capture_ix,
            };
            let patterns = (0..query.pattern_count())
                .map(|ix| {
                    let mut config = InjectionPatternConfig::default();
                    for setting in query.property_settings(ix) {
                        match setting.key.as_ref() {
                            "language" | "injection.language" => {
                                config.language.clone_from(&setting.value);
                            }
                            "combined" | "injection.combined" => {
                                config.combined = true;
                            }
                            _ => {}
                        }
                    }
                    config
                })
                .collect();
            if let Some(content_capture_ix) = content_capture_ix {
                self.grammar_mut()?.injection_config = Some(InjectionConfig {
                    query,
                    language_capture_ix,
                    content_capture_ix,
                    patterns,
                });
            } else {
                log::error!(
                    "missing required capture in injections {} TreeSitter query: \
                    content or injection.content",
                    &self.config.name,
                );
            }
        }
        Ok(self)
    }

    pub fn with_override_query(mut self, source: &str) -> anyhow::Result<Self> {
        let query = Query::new(&self.expect_grammar()?.ts_language, source)?;

        let mut override_configs_by_id = HashMap::default();
        for (ix, mut name) in query.capture_names().iter().copied().enumerate() {
            let mut range_is_inclusive = false;
            if name.starts_with('_') {
                continue;
            }
            if let Some(prefix) = name.strip_suffix(".inclusive") {
                name = prefix;
                range_is_inclusive = true;
            }

            let value = self.config.overrides.get(name).cloned().unwrap_or_default();
            for server_name in &value.opt_into_language_servers {
                if !self
                    .config
                    .scope_opt_in_language_servers
                    .contains(server_name)
                {
                    util::debug_panic!(
                        "Server {server_name:?} has been opted-in by scope {name:?} but has not been marked as an opt-in server"
                    );
                }
            }

            override_configs_by_id.insert(
                ix as u32,
                OverrideEntry {
                    name: name.to_string(),
                    range_is_inclusive,
                    value,
                },
            );
        }

        let referenced_override_names = self.config.overrides.keys().chain(
            self.config
                .brackets
                .disabled_scopes_by_bracket_ix
                .iter()
                .flatten(),
        );

        for referenced_name in referenced_override_names {
            if !override_configs_by_id
                .values()
                .any(|entry| entry.name == *referenced_name)
            {
                anyhow::bail!(
                    "language {:?} has overrides in config not in query: {referenced_name:?}",
                    self.config.name
                );
            }
        }

        for entry in override_configs_by_id.values_mut() {
            entry.value.disabled_bracket_ixs = self
                .config
                .brackets
                .disabled_scopes_by_bracket_ix
                .iter()
                .enumerate()
                .filter_map(|(ix, disabled_scope_names)| {
                    if disabled_scope_names.contains(&entry.name) {
                        Some(ix as u16)
                    } else {
                        None
                    }
                })
                .collect();
        }

        self.config.brackets.disabled_scopes_by_bracket_ix.clear();

        let grammar = self.grammar_mut()?;
        grammar.override_config = Some(OverrideConfig {
            query,
            values: override_configs_by_id,
        });
        Ok(self)
    }

    pub fn with_redaction_query(mut self, source: &str) -> anyhow::Result<Self> {
        let query = Query::new(&self.expect_grammar()?.ts_language, source)?;
        let mut redaction_capture_ix = 0;
        if populate_capture_indices(
            &query,
            &self.config.name,
            "redactions",
            &[],
            &mut [Capture::Required("redact", &mut redaction_capture_ix)],
        ) {
            self.grammar_mut()?.redactions_config = Some(RedactionConfig {
                query,
                redaction_capture_ix,
            });
        }
        Ok(self)
    }

    fn expect_grammar(&self) -> Result<&Grammar> {
        self.grammar
            .as_ref()
            .map(|grammar| grammar.as_ref())
            .context("no grammar for language")
    }

    fn grammar_mut(&mut self) -> Result<&mut Grammar> {
        Arc::get_mut(self.grammar.as_mut().context("no grammar for language")?)
            .context("cannot mutate grammar")
    }

    pub fn name(&self) -> LanguageName {
        self.config.name.clone()
    }
    pub fn manifest(&self) -> Option<&ManifestName> {
        self.manifest_name.as_ref()
    }

    pub fn code_fence_block_name(&self) -> Arc<str> {
        self.config
            .code_fence_block_name
            .clone()
            .unwrap_or_else(|| self.config.name.as_ref().to_lowercase().into())
    }

    pub fn context_provider(&self) -> Option<Arc<dyn ContextProvider>> {
        self.context_provider.clone()
    }

    pub fn toolchain_lister(&self) -> Option<Arc<dyn ToolchainLister>> {
        self.toolchain.clone()
    }

    pub fn highlight_text<'a>(
        self: &'a Arc<Self>,
        text: &'a Rope,
        range: Range<usize>,
    ) -> Vec<(Range<usize>, HighlightId)> {
        let mut result = Vec::new();
        if let Some(grammar) = &self.grammar {
            let tree = grammar.parse_text(text, None);
            let captures =
                SyntaxSnapshot::single_tree_captures(range.clone(), text, &tree, self, |grammar| {
                    grammar
                        .highlights_config
                        .as_ref()
                        .map(|config| &config.query)
                });
            let highlight_maps = vec![grammar.highlight_map()];
            let mut offset = 0;
            for chunk in
                BufferChunks::new(text, range, Some((captures, highlight_maps)), false, None)
            {
                let end_offset = offset + chunk.text.len();
                if let Some(highlight_id) = chunk.syntax_highlight_id
                    && !highlight_id.is_default()
                {
                    result.push((offset..end_offset, highlight_id));
                }
                offset = end_offset;
            }
        }
        result
    }

    pub fn path_suffixes(&self) -> &[String] {
        &self.config.matcher.path_suffixes
    }

    pub fn should_autoclose_before(&self, c: char) -> bool {
        c.is_whitespace() || self.config.autoclose_before.contains(c)
    }

    pub fn set_theme(&self, theme: &SyntaxTheme) {
        if let Some(grammar) = self.grammar.as_ref()
            && let Some(highlights_config) = &grammar.highlights_config
        {
            *grammar.highlight_map.lock() =
                HighlightMap::new(highlights_config.query.capture_names(), theme);
        }
    }

    pub fn grammar(&self) -> Option<&Arc<Grammar>> {
        self.grammar.as_ref()
    }

    pub fn default_scope(self: &Arc<Self>) -> LanguageScope {
        LanguageScope {
            language: self.clone(),
            override_id: None,
        }
    }

    pub fn lsp_id(&self) -> String {
        self.config.name.lsp_id()
    }

    pub fn prettier_parser_name(&self) -> Option<&str> {
        self.config.prettier_parser_name.as_deref()
    }

    pub fn config(&self) -> &LanguageConfig {
        &self.config
    }
}

impl LanguageScope {
    pub fn path_suffixes(&self) -> &[String] {
        self.language.path_suffixes()
    }

    pub fn language_name(&self) -> LanguageName {
        self.language.config.name.clone()
    }

    pub fn collapsed_placeholder(&self) -> &str {
        self.language.config.collapsed_placeholder.as_ref()
    }

    /// Returns line prefix that is inserted in e.g. line continuations or
    /// in `toggle comments` action.
    pub fn line_comment_prefixes(&self) -> &[Arc<str>] {
        Override::as_option(
            self.config_override().map(|o| &o.line_comments),
            Some(&self.language.config.line_comments),
        )
        .map_or([].as_slice(), |e| e.as_slice())
    }

    /// Config for block comments for this language.
    pub fn block_comment(&self) -> Option<&BlockCommentConfig> {
        Override::as_option(
            self.config_override().map(|o| &o.block_comment),
            self.language.config.block_comment.as_ref(),
        )
    }

    /// Config for documentation-style block comments for this language.
    pub fn documentation_comment(&self) -> Option<&BlockCommentConfig> {
        self.language.config.documentation_comment.as_ref()
    }

    /// Returns list markers that are inserted unchanged on newline (e.g., `- `, `* `, `+ `).
    pub fn unordered_list(&self) -> &[Arc<str>] {
        &self.language.config.unordered_list
    }

    /// Returns configuration for ordered lists with auto-incrementing numbers (e.g., `1. ` becomes `2. `).
    pub fn ordered_list(&self) -> &[OrderedListConfig] {
        &self.language.config.ordered_list
    }

    /// Returns configuration for task list continuation, if any (e.g., `- [x] ` continues as `- [ ] `).
    pub fn task_list(&self) -> Option<&TaskListConfig> {
        self.language.config.task_list.as_ref()
    }

    /// Returns additional regex patterns that act as prefix markers for creating
    /// boundaries during rewrapping.
    ///
    /// By default, Zed treats as paragraph and comment prefixes as boundaries.
    pub fn rewrap_prefixes(&self) -> &[Regex] {
        &self.language.config.rewrap_prefixes
    }

    /// Returns a list of language-specific word characters.
    ///
    /// By default, Zed treats alphanumeric characters (and '_') as word characters for
    /// the purpose of actions like 'move to next word end` or whole-word search.
    /// It additionally accounts for language's additional word characters.
    pub fn word_characters(&self) -> Option<&HashSet<char>> {
        Override::as_option(
            self.config_override().map(|o| &o.word_characters),
            Some(&self.language.config.word_characters),
        )
    }

    /// Returns a list of language-specific characters that are considered part of
    /// a completion query.
    pub fn completion_query_characters(&self) -> Option<&HashSet<char>> {
        Override::as_option(
            self.config_override()
                .map(|o| &o.completion_query_characters),
            Some(&self.language.config.completion_query_characters),
        )
    }

    /// Returns a list of language-specific characters that are considered part of
    /// identifiers during linked editing operations.
    pub fn linked_edit_characters(&self) -> Option<&HashSet<char>> {
        Override::as_option(
            self.config_override().map(|o| &o.linked_edit_characters),
            Some(&self.language.config.linked_edit_characters),
        )
    }

    /// Returns whether to prefer snippet `label` over `new_text` to replace text when
    /// completion is accepted.
    ///
    /// In cases like when cursor is in string or renaming existing function,
    /// you don't want to expand function signature instead just want function name
    /// to replace existing one.
    pub fn prefers_label_for_snippet_in_completion(&self) -> bool {
        self.config_override()
            .and_then(|o| o.prefer_label_for_snippet)
            .unwrap_or(false)
    }

    /// Returns a list of bracket pairs for a given language with an additional
    /// piece of information about whether the particular bracket pair is currently active for a given language.
    pub fn brackets(&self) -> impl Iterator<Item = (&BracketPair, bool)> {
        let mut disabled_ids = self
            .config_override()
            .map_or(&[] as _, |o| o.disabled_bracket_ixs.as_slice());
        self.language
            .config
            .brackets
            .pairs
            .iter()
            .enumerate()
            .map(move |(ix, bracket)| {
                let mut is_enabled = true;
                if let Some(next_disabled_ix) = disabled_ids.first()
                    && ix == *next_disabled_ix as usize
                {
                    disabled_ids = &disabled_ids[1..];
                    is_enabled = false;
                }
                (bracket, is_enabled)
            })
    }

    pub fn should_autoclose_before(&self, c: char) -> bool {
        c.is_whitespace() || self.language.config.autoclose_before.contains(c)
    }

    pub fn language_allowed(&self, name: &LanguageServerName) -> bool {
        let config = &self.language.config;
        let opt_in_servers = &config.scope_opt_in_language_servers;
        if opt_in_servers.contains(name) {
            if let Some(over) = self.config_override() {
                over.opt_into_language_servers.contains(name)
            } else {
                false
            }
        } else {
            true
        }
    }

    pub fn override_name(&self) -> Option<&str> {
        let id = self.override_id?;
        let grammar = self.language.grammar.as_ref()?;
        let override_config = grammar.override_config.as_ref()?;
        override_config.values.get(&id).map(|e| e.name.as_str())
    }

    fn config_override(&self) -> Option<&LanguageConfigOverride> {
        let id = self.override_id?;
        let grammar = self.language.grammar.as_ref()?;
        let override_config = grammar.override_config.as_ref()?;
        override_config.values.get(&id).map(|e| &e.value)
    }
}

impl Hash for Language {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id.hash(state)
    }
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.id.eq(&other.id)
    }
}

impl Eq for Language {}

impl Debug for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Language")
            .field("name", &self.config.name)
            .finish()
    }
}

impl Grammar {
    pub fn id(&self) -> GrammarId {
        self.id
    }

    fn parse_text(&self, text: &Rope, old_tree: Option<Tree>) -> Tree {
        with_parser(|parser| {
            parser
                .set_language(&self.ts_language)
                .expect("incompatible grammar");
            let mut chunks = text.chunks_in_range(0..text.len());
            parser
                .parse_with_options(
                    &mut move |offset, _| {
                        chunks.seek(offset);
                        chunks.next().unwrap_or("").as_bytes()
                    },
                    old_tree.as_ref(),
                    None,
                )
                .unwrap()
        })
    }

    pub fn highlight_map(&self) -> HighlightMap {
        self.highlight_map.lock().clone()
    }

    pub fn highlight_id_for_name(&self, name: &str) -> Option<HighlightId> {
        let capture_id = self
            .highlights_config
            .as_ref()?
            .query
            .capture_index_for_name(name)?;
        Some(self.highlight_map.lock().get(capture_id))
    }

    pub fn debug_variables_config(&self) -> Option<&DebugVariablesConfig> {
        self.debug_variables_config.as_ref()
    }

    pub fn imports_config(&self) -> Option<&ImportsConfig> {
        self.imports_config.as_ref()
    }
}

impl CodeLabelBuilder {
    pub fn respan_filter_range(&mut self, filter_text: Option<&str>) {
        self.filter_range = filter_text
            .and_then(|filter| self.text.find(filter).map(|ix| ix..ix + filter.len()))
            .unwrap_or(0..self.text.len());
    }

    pub fn push_str(&mut self, text: &str, highlight: Option<HighlightId>) {
        let start_ix = self.text.len();
        self.text.push_str(text);
        if let Some(highlight) = highlight {
            let end_ix = self.text.len();
            self.runs.push((start_ix..end_ix, highlight));
        }
    }

    pub fn build(mut self) -> CodeLabel {
        if self.filter_range.end == 0 {
            self.respan_filter_range(None);
        }
        CodeLabel {
            text: self.text,
            runs: self.runs,
            filter_range: self.filter_range,
        }
    }
}

impl CodeLabel {
    pub fn fallback_for_completion(
        item: &lsp::CompletionItem,
        language: Option<&Language>,
    ) -> Self {
        let highlight_id = item.kind.and_then(|kind| {
            let grammar = language?.grammar()?;
            use lsp::CompletionItemKind as Kind;
            match kind {
                Kind::CLASS => grammar.highlight_id_for_name("type"),
                Kind::CONSTANT => grammar.highlight_id_for_name("constant"),
                Kind::CONSTRUCTOR => grammar.highlight_id_for_name("constructor"),
                Kind::ENUM => grammar
                    .highlight_id_for_name("enum")
                    .or_else(|| grammar.highlight_id_for_name("type")),
                Kind::ENUM_MEMBER => grammar
                    .highlight_id_for_name("variant")
                    .or_else(|| grammar.highlight_id_for_name("property")),
                Kind::FIELD => grammar.highlight_id_for_name("property"),
                Kind::FUNCTION => grammar.highlight_id_for_name("function"),
                Kind::INTERFACE => grammar.highlight_id_for_name("type"),
                Kind::METHOD => grammar
                    .highlight_id_for_name("function.method")
                    .or_else(|| grammar.highlight_id_for_name("function")),
                Kind::OPERATOR => grammar.highlight_id_for_name("operator"),
                Kind::PROPERTY => grammar.highlight_id_for_name("property"),
                Kind::STRUCT => grammar.highlight_id_for_name("type"),
                Kind::VARIABLE => grammar.highlight_id_for_name("variable"),
                Kind::KEYWORD => grammar.highlight_id_for_name("keyword"),
                _ => None,
            }
        });

        let label = &item.label;
        let label_length = label.len();
        let runs = highlight_id
            .map(|highlight_id| vec![(0..label_length, highlight_id)])
            .unwrap_or_default();
        let text = if let Some(detail) = item.detail.as_deref().filter(|detail| detail != label) {
            format!("{label} {detail}")
        } else if let Some(description) = item
            .label_details
            .as_ref()
            .and_then(|label_details| label_details.description.as_deref())
            .filter(|description| description != label)
        {
            format!("{label} {description}")
        } else {
            label.clone()
        };
        let filter_range = item
            .filter_text
            .as_deref()
            .and_then(|filter| text.find(filter).map(|ix| ix..ix + filter.len()))
            .unwrap_or(0..label_length);
        Self {
            text,
            runs,
            filter_range,
        }
    }

    pub fn plain(text: String, filter_text: Option<&str>) -> Self {
        Self::filtered(text.clone(), text.len(), filter_text, Vec::new())
    }

    pub fn filtered(
        text: String,
        label_len: usize,
        filter_text: Option<&str>,
        runs: Vec<(Range<usize>, HighlightId)>,
    ) -> Self {
        assert!(label_len <= text.len());
        let filter_range = filter_text
            .and_then(|filter| text.find(filter).map(|ix| ix..ix + filter.len()))
            .unwrap_or(0..label_len);
        Self::new(text, filter_range, runs)
    }

    pub fn new(
        text: String,
        filter_range: Range<usize>,
        runs: Vec<(Range<usize>, HighlightId)>,
    ) -> Self {
        assert!(
            text.get(filter_range.clone()).is_some(),
            "invalid filter range"
        );
        runs.iter().for_each(|(range, _)| {
            assert!(
                text.get(range.clone()).is_some(),
                "invalid run range with inputs. Requested range {range:?} in text '{text}'",
            );
        });
        Self {
            runs,
            filter_range,
            text,
        }
    }

    pub fn text(&self) -> &str {
        self.text.as_str()
    }

    pub fn filter_text(&self) -> &str {
        &self.text[self.filter_range.clone()]
    }
}

impl From<String> for CodeLabel {
    fn from(value: String) -> Self {
        Self::plain(value, None)
    }
}

impl From<&str> for CodeLabel {
    fn from(value: &str) -> Self {
        Self::plain(value.to_string(), None)
    }
}

impl Ord for LanguageMatcher {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.path_suffixes.cmp(&other.path_suffixes).then_with(|| {
            self.first_line_pattern
                .as_ref()
                .map(Regex::as_str)
                .cmp(&other.first_line_pattern.as_ref().map(Regex::as_str))
        })
    }
}

impl PartialOrd for LanguageMatcher {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Eq for LanguageMatcher {}

impl PartialEq for LanguageMatcher {
    fn eq(&self, other: &Self) -> bool {
        self.path_suffixes == other.path_suffixes
            && self.first_line_pattern.as_ref().map(Regex::as_str)
                == other.first_line_pattern.as_ref().map(Regex::as_str)
    }
}

#[cfg(any(test, feature = "test-support"))]
impl Default for FakeLspAdapter {
    fn default() -> Self {
        Self {
            name: "the-fake-language-server",
            capabilities: lsp::LanguageServer::full_capabilities(),
            initializer: None,
            disk_based_diagnostics_progress_token: None,
            initialization_options: None,
            disk_based_diagnostics_sources: Vec::new(),
            prettier_plugins: Vec::new(),
            language_server_binary: LanguageServerBinary {
                path: "/the/fake/lsp/path".into(),
                arguments: vec![],
                env: Default::default(),
            },
            label_for_completion: None,
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl LspInstaller for FakeLspAdapter {
    type BinaryVersion = ();

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
        _: bool,
        _: &mut AsyncApp,
    ) -> Result<Self::BinaryVersion> {
        unreachable!()
    }

    async fn check_if_user_installed(
        &self,
        _: &dyn LspAdapterDelegate,
        _: Option<Toolchain>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        Some(self.language_server_binary.clone())
    }

    async fn fetch_server_binary(
        &self,
        _: (),
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        unreachable!();
    }

    async fn cached_server_binary(
        &self,
        _: PathBuf,
        _: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        unreachable!();
    }
}

#[cfg(any(test, feature = "test-support"))]
#[async_trait(?Send)]
impl LspAdapter for FakeLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName(self.name.into())
    }

    fn disk_based_diagnostic_sources(&self) -> Vec<String> {
        self.disk_based_diagnostics_sources.clone()
    }

    fn disk_based_diagnostics_progress_token(&self) -> Option<String> {
        self.disk_based_diagnostics_progress_token.clone()
    }

    async fn initialization_options(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<Value>> {
        Ok(self.initialization_options.clone())
    }

    async fn label_for_completion(
        &self,
        item: &lsp::CompletionItem,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        let label_for_completion = self.label_for_completion.as_ref()?;
        label_for_completion(item, language)
    }

    fn is_extension(&self) -> bool {
        false
    }
}

enum Capture<'a> {
    Required(&'static str, &'a mut u32),
    Optional(&'static str, &'a mut Option<u32>),
}

fn populate_capture_indices(
    query: &Query,
    language_name: &LanguageName,
    query_type: &str,
    expected_prefixes: &[&str],
    captures: &mut [Capture<'_>],
) -> bool {
    let mut found_required_indices = Vec::new();
    'outer: for (ix, name) in query.capture_names().iter().enumerate() {
        for (required_ix, capture) in captures.iter_mut().enumerate() {
            match capture {
                Capture::Required(capture_name, index) if capture_name == name => {
                    **index = ix as u32;
                    found_required_indices.push(required_ix);
                    continue 'outer;
                }
                Capture::Optional(capture_name, index) if capture_name == name => {
                    **index = Some(ix as u32);
                    continue 'outer;
                }
                _ => {}
            }
        }
        if !name.starts_with("_")
            && !expected_prefixes
                .iter()
                .any(|&prefix| name.starts_with(prefix))
        {
            log::warn!(
                "unrecognized capture name '{}' in {} {} TreeSitter query \
                (suppress this warning by prefixing with '_')",
                name,
                language_name,
                query_type
            );
        }
    }
    let mut missing_required_captures = Vec::new();
    for (capture_ix, capture) in captures.iter().enumerate() {
        if let Capture::Required(capture_name, _) = capture
            && !found_required_indices.contains(&capture_ix)
        {
            missing_required_captures.push(*capture_name);
        }
    }
    let success = missing_required_captures.is_empty();
    if !success {
        log::error!(
            "missing required capture(s) in {} {} TreeSitter query: {}",
            language_name,
            query_type,
            missing_required_captures.join(", ")
        );
    }
    success
}

pub fn point_to_lsp(point: PointUtf16) -> lsp::Position {
    lsp::Position::new(point.row, point.column)
}

pub fn point_from_lsp(point: lsp::Position) -> Unclipped<PointUtf16> {
    Unclipped(PointUtf16::new(point.line, point.character))
}

pub fn range_to_lsp(range: Range<PointUtf16>) -> Result<lsp::Range> {
    anyhow::ensure!(
        range.start <= range.end,
        "Inverted range provided to an LSP request: {:?}-{:?}",
        range.start,
        range.end
    );
    Ok(lsp::Range {
        start: point_to_lsp(range.start),
        end: point_to_lsp(range.end),
    })
}

pub fn range_from_lsp(range: lsp::Range) -> Range<Unclipped<PointUtf16>> {
    let mut start = point_from_lsp(range.start);
    let mut end = point_from_lsp(range.end);
    if start > end {
        // We debug instead of warn so that this is not logged by default unless explicitly requested.
        // Using warn would write to the log file, and since we receive an enormous amount of
        // range_from_lsp calls (especially during completions), that can hang the main thread.
        //
        // See issue #36223.
        zlog::debug!("range_from_lsp called with inverted range {start:?}-{end:?}");
        mem::swap(&mut start, &mut end);
    }
    start..end
}

#[doc(hidden)]
#[cfg(any(test, feature = "test-support"))]
pub fn rust_lang() -> Arc<Language> {
    use std::borrow::Cow;

    let language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
            line_comments: vec!["// ".into(), "/// ".into(), "//! ".into()],
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    )
    .with_queries(LanguageQueries {
        outline: Some(Cow::from(include_str!(
            "../../languages/src/rust/outline.scm"
        ))),
        indents: Some(Cow::from(include_str!(
            "../../languages/src/rust/indents.scm"
        ))),
        brackets: Some(Cow::from(include_str!(
            "../../languages/src/rust/brackets.scm"
        ))),
        text_objects: Some(Cow::from(include_str!(
            "../../languages/src/rust/textobjects.scm"
        ))),
        highlights: Some(Cow::from(include_str!(
            "../../languages/src/rust/highlights.scm"
        ))),
        injections: Some(Cow::from(include_str!(
            "../../languages/src/rust/injections.scm"
        ))),
        overrides: Some(Cow::from(include_str!(
            "../../languages/src/rust/overrides.scm"
        ))),
        redactions: None,
        runnables: Some(Cow::from(include_str!(
            "../../languages/src/rust/runnables.scm"
        ))),
        debugger: Some(Cow::from(include_str!(
            "../../languages/src/rust/debugger.scm"
        ))),
        imports: Some(Cow::from(include_str!(
            "../../languages/src/rust/imports.scm"
        ))),
    })
    .expect("Could not parse queries");
    Arc::new(language)
}

#[doc(hidden)]
#[cfg(any(test, feature = "test-support"))]
pub fn markdown_lang() -> Arc<Language> {
    use std::borrow::Cow;

    let language = Language::new(
        LanguageConfig {
            name: "Markdown".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["md".into()],
                ..Default::default()
            },
            ..LanguageConfig::default()
        },
        Some(tree_sitter_md::LANGUAGE.into()),
    )
    .with_queries(LanguageQueries {
        brackets: Some(Cow::from(include_str!(
            "../../languages/src/markdown/brackets.scm"
        ))),
        injections: Some(Cow::from(include_str!(
            "../../languages/src/markdown/injections.scm"
        ))),
        highlights: Some(Cow::from(include_str!(
            "../../languages/src/markdown/highlights.scm"
        ))),
        indents: Some(Cow::from(include_str!(
            "../../languages/src/markdown/indents.scm"
        ))),
        outline: Some(Cow::from(include_str!(
            "../../languages/src/markdown/outline.scm"
        ))),
        ..LanguageQueries::default()
    })
    .expect("Could not parse markdown queries");
    Arc::new(language)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use pretty_assertions::assert_matches;

    #[gpui::test(iterations = 10)]
    async fn test_language_loading(cx: &mut TestAppContext) {
        let languages = LanguageRegistry::test(cx.executor());
        let languages = Arc::new(languages);
        languages.register_native_grammars([
            ("json", tree_sitter_json::LANGUAGE),
            ("rust", tree_sitter_rust::LANGUAGE),
        ]);
        languages.register_test_language(LanguageConfig {
            name: "JSON".into(),
            grammar: Some("json".into()),
            matcher: LanguageMatcher {
                path_suffixes: vec!["json".into()],
                ..Default::default()
            },
            ..Default::default()
        });
        languages.register_test_language(LanguageConfig {
            name: "Rust".into(),
            grammar: Some("rust".into()),
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".into()],
                ..Default::default()
            },
            ..Default::default()
        });
        assert_eq!(
            languages.language_names(),
            &[
                LanguageName::new_static("JSON"),
                LanguageName::new_static("Plain Text"),
                LanguageName::new_static("Rust"),
            ]
        );

        let rust1 = languages.language_for_name("Rust");
        let rust2 = languages.language_for_name("Rust");

        // Ensure language is still listed even if it's being loaded.
        assert_eq!(
            languages.language_names(),
            &[
                LanguageName::new_static("JSON"),
                LanguageName::new_static("Plain Text"),
                LanguageName::new_static("Rust"),
            ]
        );

        let (rust1, rust2) = futures::join!(rust1, rust2);
        assert!(Arc::ptr_eq(&rust1.unwrap(), &rust2.unwrap()));

        // Ensure language is still listed even after loading it.
        assert_eq!(
            languages.language_names(),
            &[
                LanguageName::new_static("JSON"),
                LanguageName::new_static("Plain Text"),
                LanguageName::new_static("Rust"),
            ]
        );

        // Loading an unknown language returns an error.
        assert!(languages.language_for_name("Unknown").await.is_err());
    }

    #[gpui::test]
    async fn test_completion_label_omits_duplicate_data() {
        let regular_completion_item_1 = lsp::CompletionItem {
            label: "regular1".to_string(),
            detail: Some("detail1".to_string()),
            label_details: Some(lsp::CompletionItemLabelDetails {
                detail: None,
                description: Some("description 1".to_string()),
            }),
            ..lsp::CompletionItem::default()
        };

        let regular_completion_item_2 = lsp::CompletionItem {
            label: "regular2".to_string(),
            label_details: Some(lsp::CompletionItemLabelDetails {
                detail: None,
                description: Some("description 2".to_string()),
            }),
            ..lsp::CompletionItem::default()
        };

        let completion_item_with_duplicate_detail_and_proper_description = lsp::CompletionItem {
            detail: Some(regular_completion_item_1.label.clone()),
            ..regular_completion_item_1.clone()
        };

        let completion_item_with_duplicate_detail = lsp::CompletionItem {
            detail: Some(regular_completion_item_1.label.clone()),
            label_details: None,
            ..regular_completion_item_1.clone()
        };

        let completion_item_with_duplicate_description = lsp::CompletionItem {
            label_details: Some(lsp::CompletionItemLabelDetails {
                detail: None,
                description: Some(regular_completion_item_2.label.clone()),
            }),
            ..regular_completion_item_2.clone()
        };

        assert_eq!(
            CodeLabel::fallback_for_completion(&regular_completion_item_1, None).text,
            format!(
                "{} {}",
                regular_completion_item_1.label,
                regular_completion_item_1.detail.unwrap()
            ),
            "LSP completion items with both detail and label_details.description should prefer detail"
        );
        assert_eq!(
            CodeLabel::fallback_for_completion(&regular_completion_item_2, None).text,
            format!(
                "{} {}",
                regular_completion_item_2.label,
                regular_completion_item_2
                    .label_details
                    .as_ref()
                    .unwrap()
                    .description
                    .as_ref()
                    .unwrap()
            ),
            "LSP completion items without detail but with label_details.description should use that"
        );
        assert_eq!(
            CodeLabel::fallback_for_completion(
                &completion_item_with_duplicate_detail_and_proper_description,
                None
            )
            .text,
            format!(
                "{} {}",
                regular_completion_item_1.label,
                regular_completion_item_1
                    .label_details
                    .as_ref()
                    .unwrap()
                    .description
                    .as_ref()
                    .unwrap()
            ),
            "LSP completion items with both detail and label_details.description should prefer description only if the detail duplicates the completion label"
        );
        assert_eq!(
            CodeLabel::fallback_for_completion(&completion_item_with_duplicate_detail, None).text,
            regular_completion_item_1.label,
            "LSP completion items with duplicate label and detail, should omit the detail"
        );
        assert_eq!(
            CodeLabel::fallback_for_completion(&completion_item_with_duplicate_description, None)
                .text,
            regular_completion_item_2.label,
            "LSP completion items with duplicate label and detail, should omit the detail"
        );
    }

    #[test]
    fn test_deserializing_comments_backwards_compat() {
        // current version of `block_comment` and `documentation_comment` work
        {
            let config: LanguageConfig = ::toml::from_str(
                r#"
                name = "Foo"
                block_comment = { start = "a", end = "b", prefix = "c", tab_size = 1 }
                documentation_comment = { start = "d", end = "e", prefix = "f", tab_size = 2 }
                "#,
            )
            .unwrap();
            assert_matches!(config.block_comment, Some(BlockCommentConfig { .. }));
            assert_matches!(
                config.documentation_comment,
                Some(BlockCommentConfig { .. })
            );

            let block_config = config.block_comment.unwrap();
            assert_eq!(block_config.start.as_ref(), "a");
            assert_eq!(block_config.end.as_ref(), "b");
            assert_eq!(block_config.prefix.as_ref(), "c");
            assert_eq!(block_config.tab_size, 1);

            let doc_config = config.documentation_comment.unwrap();
            assert_eq!(doc_config.start.as_ref(), "d");
            assert_eq!(doc_config.end.as_ref(), "e");
            assert_eq!(doc_config.prefix.as_ref(), "f");
            assert_eq!(doc_config.tab_size, 2);
        }

        // former `documentation` setting is read into `documentation_comment`
        {
            let config: LanguageConfig = ::toml::from_str(
                r#"
                name = "Foo"
                documentation = { start = "a", end = "b", prefix = "c", tab_size = 1}
                "#,
            )
            .unwrap();
            assert_matches!(
                config.documentation_comment,
                Some(BlockCommentConfig { .. })
            );

            let config = config.documentation_comment.unwrap();
            assert_eq!(config.start.as_ref(), "a");
            assert_eq!(config.end.as_ref(), "b");
            assert_eq!(config.prefix.as_ref(), "c");
            assert_eq!(config.tab_size, 1);
        }

        // old block_comment format is read into BlockCommentConfig
        {
            let config: LanguageConfig = ::toml::from_str(
                r#"
                name = "Foo"
                block_comment = ["a", "b"]
                "#,
            )
            .unwrap();
            assert_matches!(config.block_comment, Some(BlockCommentConfig { .. }));

            let config = config.block_comment.unwrap();
            assert_eq!(config.start.as_ref(), "a");
            assert_eq!(config.end.as_ref(), "b");
            assert_eq!(config.prefix.as_ref(), "");
            assert_eq!(config.tab_size, 0);
        }
    }
}
