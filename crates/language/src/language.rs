//! The `language` crate provides a large chunk of Zed's language-related
//! features (the other big contributors being project and lsp crates that revolve around LSP features).
//! Namely, this crate:
//! - Provides [`Language`], [`Grammar`] and [`LanguageRegistry`] types that
//!   use Tree-sitter to provide syntax highlighting to the editor; note though that `language` doesn't perform the highlighting by itself. It only maps ranges in a buffer to colors. Treesitter is also used for buffer outlines (lists of symbols in a buffer)
//! - Exposes [`LanguageConfig`] that describes how constructs (like brackets or line comments) should be handled by the editor for a source file of a particular language.
//!
//! Notably we do *not* assign a single language to a single file; in real world a single file can consist of multiple programming languages - HTML is a good example of that - and `language` crate tends to reflect that status quo in its API.
mod buffer;
mod diagnostic;
mod diagnostic_set;
mod file_content;
mod language_registry;

pub mod language_settings;
mod manifest;
pub mod modeline;
mod outline;
pub mod proto;
mod syntax_map;
mod task_context;
mod text_diff;
mod toolchain;

#[cfg(test)]
pub mod buffer_tests;

pub use crate::language_settings::{AutoIndentMode, EditPredictionsMode, IndentGuideSettings};
use anyhow::{Context as _, Result};
use async_trait::async_trait;
use collections::{HashMap, HashSet};
use futures::Future;
use futures::future::LocalBoxFuture;
use futures::lock::OwnedMutexGuard;
use gpui::{App, AsyncApp, Entity};
use http_client::HttpClient;

pub use language_core::highlight_map::{HighlightId, HighlightMap};

pub use language_core::{
    BlockCommentConfig, BracketPair, BracketPairConfig, BracketPairContent, BracketsConfig,
    BracketsPatternConfig, CodeLabel, CodeLabelBuilder, DebugVariablesConfig, DebuggerTextObject,
    DecreaseIndentConfig, Grammar, GrammarId, HighlightsConfig, IndentConfig, InjectionConfig,
    InjectionPatternConfig, JsxTagAutoCloseConfig, LanguageConfig, LanguageConfigOverride,
    LanguageId, LanguageMatcher, OrderedListConfig, OutlineConfig, Override, OverrideConfig,
    OverrideEntry, PromptResponseContext, RedactionConfig, RunnableCapture, RunnableConfig,
    SoftWrap, Symbol, TaskListConfig, TextObject, TextObjectConfig, ToLspPosition,
    WrapCharactersConfig, auto_indent_using_last_non_empty_line_default, deserialize_regex,
    deserialize_regex_vec, regex_json_schema, regex_vec_json_schema, serialize_regex,
};
pub use language_registry::{
    LanguageName, LanguageServerStatusUpdate, LoadedLanguage, ServerHealth,
};
use lsp::{
    CodeActionKind, InitializeParams, LanguageServerBinary, LanguageServerBinaryOptions, Uri,
};
pub use manifest::{ManifestDelegate, ManifestName, ManifestProvider, ManifestQuery};
pub use modeline::{ModelineSettings, parse_modeline};
use parking_lot::Mutex;
use regex::Regex;
use semver::Version;
use serde_json::Value;
use settings::WorktreeId;
use smol::future::FutureExt as _;
use std::{
    ffi::OsStr,
    fmt::Debug,
    hash::Hash,
    mem,
    ops::{DerefMut, Range},
    path::{Path, PathBuf},
    str,
    sync::{Arc, LazyLock},
};
use syntax_map::{QueryCursorHandle, SyntaxSnapshot};
use task::RunnableTag;
pub use task_context::{ContextLocation, ContextProvider, RunnableRange};
pub use text_diff::{
    DiffOptions, apply_diff_patch, apply_reversed_diff_patch, char_diff, line_diff, text_diff,
    text_diff_with_options, unified_diff, unified_diff_with_context, unified_diff_with_offsets,
    word_diff_ranges,
};
use theme::SyntaxTheme;
pub use toolchain::{
    LanguageToolchainStore, LocalLanguageToolchainStore, Toolchain, ToolchainList, ToolchainLister,
    ToolchainMetadata, ToolchainScope,
};
use tree_sitter::{self, QueryCursor, WasmStore, wasmtime};
use util::rel_path::RelPath;

pub use buffer::Operation;
pub use buffer::*;
pub use diagnostic::{Diagnostic, DiagnosticSourceKind};
pub use diagnostic_set::{DiagnosticEntry, DiagnosticEntryRef, DiagnosticGroup};
pub use file_content::{ByteContent, FILE_ANALYSIS_BYTES, analyze_byte_content};
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

pub(crate) fn to_settings_soft_wrap(value: language_core::SoftWrap) -> settings::SoftWrap {
    match value {
        language_core::SoftWrap::None => settings::SoftWrap::None,
        language_core::SoftWrap::PreferLine => settings::SoftWrap::PreferLine,
        language_core::SoftWrap::EditorWidth => settings::SoftWrap::EditorWidth,
        language_core::SoftWrap::Bounded => settings::SoftWrap::Bounded,
    }
}

static QUERY_CURSORS: Mutex<Vec<QueryCursor>> = Mutex::new(vec![]);
static PARSERS: Mutex<Vec<Parser>> = Mutex::new(vec![]);

#[ztracing::instrument(skip_all)]
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
                modeline_aliases: vec!["text".to_owned(), "txt".to_owned()],
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

/// Commands that the client (editor) handles locally rather than forwarding
/// to the language server. Servers embed these in code lens and code action
/// responses when they want the editor to perform a well-known UI action.
#[derive(Debug, Clone)]
pub enum ClientCommand {
    /// Open a location list (references panel / peek view).
    ShowLocations,
    /// Schedule a task from an LSP command's arguments.
    ScheduleTask(task::TaskTemplate),
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
    ) {
        self.adapter.process_diagnostics(params, server_id)
    }

    pub fn retain_old_diagnostic(&self, previous_diagnostic: &Diagnostic) -> bool {
        self.adapter.retain_old_diagnostic(previous_diagnostic)
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
        symbols: &[Symbol],
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

    pub async fn settings_schema(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncApp,
    ) -> Option<serde_json::Value> {
        self.adapter
            .clone()
            .settings_schema(delegate, self.cached_binary.clone().lock_owned().await, cx)
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
    fn resolve_relative_path(&self, path: PathBuf) -> PathBuf;
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

#[async_trait(?Send)]
pub trait LspAdapter: 'static + Send + Sync + DynLspInstaller {
    fn name(&self) -> LanguageServerName;

    fn process_diagnostics(&self, _: &mut lsp::PublishDiagnosticsParams, _: LanguageServerId) {}

    /// When processing new `lsp::PublishDiagnosticsParams` diagnostics, whether to retain previous one(s) or not.
    fn retain_old_diagnostic(&self, _previous_diagnostic: &Diagnostic) -> bool {
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
        symbols: &[Symbol],
        language: &Arc<Language>,
    ) -> Result<Vec<Option<CodeLabel>>> {
        let mut labels = Vec::new();
        for (ix, symbol) in symbols.iter().enumerate() {
            let label = self.label_for_symbol(symbol, language).await;
            if let Some(label) = label {
                labels.resize(ix + 1, None);
                *labels.last_mut().unwrap() = Some(label);
            }
        }
        Ok(labels)
    }

    async fn label_for_symbol(
        &self,
        _symbol: &Symbol,
        _language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        None
    }

    /// Returns initialization options that are going to be sent to a LSP server as a part of [`lsp::InitializeParams`]
    async fn initialization_options(
        self: Arc<Self>,
        _: &Arc<dyn LspAdapterDelegate>,
        _cx: &mut AsyncApp,
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

    /// Returns the JSON schema of the settings for the language server.
    /// This corresponds to the `settings` field in `LspSettings`, which is used
    /// to respond to `workspace/configuration` requests from the language server.
    async fn settings_schema(
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

    fn client_command(
        &self,
        _command_name: &str,
        _arguments: &[serde_json::Value],
    ) -> Option<ClientCommand> {
        None
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

/// Represents a language for the given range. Some languages (e.g. HTML)
/// interleave several languages together, thus a single buffer might actually contain
/// several nested scopes.
#[derive(Clone, Debug)]
pub struct LanguageScope {
    language: Arc<Language>,
    override_id: Option<u32>,
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

pub struct Language {
    pub(crate) id: LanguageId,
    pub(crate) config: LanguageConfig,
    pub(crate) grammar: Option<Arc<Grammar>>,
    pub(crate) context_provider: Option<Arc<dyn ContextProvider>>,
    pub(crate) toolchain: Option<Arc<dyn ToolchainLister>>,
    pub(crate) manifest_name: Option<ManifestName>,
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
            grammar: ts_language.map(|ts_language| Arc::new(Grammar::new(ts_language))),
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
        if let Some(grammar) = self.grammar.take() {
            let grammar =
                Arc::try_unwrap(grammar).map_err(|_| anyhow::anyhow!("cannot mutate grammar"))?;
            let grammar = grammar.with_queries(queries, &mut self.config)?;
            self.grammar = Some(Arc::new(grammar));
        }
        Ok(self)
    }

    pub fn with_highlights_query(self, source: &str) -> Result<Self> {
        self.with_grammar_query(|grammar| grammar.with_highlights_query(source))
    }

    pub fn with_runnable_query(self, source: &str) -> Result<Self> {
        self.with_grammar_query(|grammar| grammar.with_runnable_query(source))
    }

    pub fn with_outline_query(self, source: &str) -> Result<Self> {
        self.with_grammar_query_and_name(|grammar, name| grammar.with_outline_query(source, name))
    }

    pub fn with_text_object_query(self, source: &str) -> Result<Self> {
        self.with_grammar_query_and_name(|grammar, name| {
            grammar.with_text_object_query(source, name)
        })
    }

    pub fn with_debug_variables_query(self, source: &str) -> Result<Self> {
        self.with_grammar_query_and_name(|grammar, name| {
            grammar.with_debug_variables_query(source, name)
        })
    }

    pub fn with_brackets_query(self, source: &str) -> Result<Self> {
        self.with_grammar_query_and_name(|grammar, name| grammar.with_brackets_query(source, name))
    }

    pub fn with_indents_query(self, source: &str) -> Result<Self> {
        self.with_grammar_query_and_name(|grammar, name| grammar.with_indents_query(source, name))
    }

    pub fn with_injection_query(self, source: &str) -> Result<Self> {
        self.with_grammar_query_and_name(|grammar, name| grammar.with_injection_query(source, name))
    }

    pub fn with_override_query(mut self, source: &str) -> Result<Self> {
        if let Some(grammar_arc) = self.grammar.take() {
            let grammar = Arc::try_unwrap(grammar_arc)
                .map_err(|_| anyhow::anyhow!("cannot mutate grammar"))?;
            let grammar = grammar.with_override_query(
                source,
                &self.config.name,
                &self.config.overrides,
                &mut self.config.brackets,
                &self.config.scope_opt_in_language_servers,
            )?;
            self.grammar = Some(Arc::new(grammar));
        }
        Ok(self)
    }

    pub fn with_redaction_query(self, source: &str) -> Result<Self> {
        self.with_grammar_query_and_name(|grammar, name| grammar.with_redaction_query(source, name))
    }

    fn with_grammar_query(
        mut self,
        build: impl FnOnce(Grammar) -> Result<Grammar>,
    ) -> Result<Self> {
        if let Some(grammar_arc) = self.grammar.take() {
            let grammar = Arc::try_unwrap(grammar_arc)
                .map_err(|_| anyhow::anyhow!("cannot mutate grammar"))?;
            self.grammar = Some(Arc::new(build(grammar)?));
        }
        Ok(self)
    }

    fn with_grammar_query_and_name(
        mut self,
        build: impl FnOnce(Grammar, &LanguageName) -> Result<Grammar>,
    ) -> Result<Self> {
        if let Some(grammar_arc) = self.grammar.take() {
            let grammar = Arc::try_unwrap(grammar_arc)
                .map_err(|_| anyhow::anyhow!("cannot mutate grammar"))?;
            self.grammar = Some(Arc::new(build(grammar, &self.config.name)?));
        }
        Ok(self)
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

    pub fn matches_kernel_language(&self, kernel_language: &str) -> bool {
        let kernel_language_lower = kernel_language.to_lowercase();

        if self.code_fence_block_name().to_lowercase() == kernel_language_lower {
            return true;
        }

        if self.config.name.as_ref().to_lowercase() == kernel_language_lower {
            return true;
        }

        self.config
            .kernel_language_names
            .iter()
            .any(|name| name.to_lowercase() == kernel_language_lower)
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
            let tree = parse_text(grammar, text, None);
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
                if let Some(highlight_id) = chunk.syntax_highlight_id {
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
                build_highlight_map(highlights_config.query.capture_names(), theme);
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

#[inline]
pub fn build_highlight_map(capture_names: &[&str], theme: &SyntaxTheme) -> HighlightMap {
    HighlightMap::from_ids(
        capture_names
            .iter()
            .map(|capture_name| theme.highlight_id(capture_name).map(HighlightId::new)),
    )
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

pub(crate) fn parse_text(grammar: &Grammar, text: &Rope, old_tree: Option<Tree>) -> Tree {
    with_parser(|parser| {
        parser
            .set_language(&grammar.ts_language)
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

pub trait CodeLabelExt {
    fn fallback_for_completion(
        item: &lsp::CompletionItem,
        language: Option<&Language>,
    ) -> CodeLabel;
}

impl CodeLabelExt for CodeLabel {
    fn fallback_for_completion(
        item: &lsp::CompletionItem,
        language: Option<&Language>,
    ) -> CodeLabel {
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
        CodeLabel {
            text,
            runs,
            filter_range,
        }
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
        _cx: &mut AsyncApp,
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
            brackets: BracketPairConfig {
                pairs: vec![
                    BracketPair {
                        start: "{".into(),
                        end: "}".into(),
                        close: true,
                        surround: false,
                        newline: true,
                    },
                    BracketPair {
                        start: "[".into(),
                        end: "]".into(),
                        close: true,
                        surround: false,
                        newline: true,
                    },
                    BracketPair {
                        start: "(".into(),
                        end: ")".into(),
                        close: true,
                        surround: false,
                        newline: true,
                    },
                    BracketPair {
                        start: "<".into(),
                        end: ">".into(),
                        close: false,
                        surround: false,
                        newline: true,
                    },
                    BracketPair {
                        start: "\"".into(),
                        end: "\"".into(),
                        close: true,
                        surround: false,
                        newline: false,
                    },
                ],
                ..Default::default()
            },
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    )
    .with_queries(LanguageQueries {
        outline: Some(Cow::from(include_str!(
            "../../grammars/src/rust/outline.scm"
        ))),
        indents: Some(Cow::from(include_str!(
            "../../grammars/src/rust/indents.scm"
        ))),
        brackets: Some(Cow::from(include_str!(
            "../../grammars/src/rust/brackets.scm"
        ))),
        text_objects: Some(Cow::from(include_str!(
            "../../grammars/src/rust/textobjects.scm"
        ))),
        highlights: Some(Cow::from(include_str!(
            "../../grammars/src/rust/highlights.scm"
        ))),
        injections: Some(Cow::from(include_str!(
            "../../grammars/src/rust/injections.scm"
        ))),
        overrides: Some(Cow::from(include_str!(
            "../../grammars/src/rust/overrides.scm"
        ))),
        redactions: None,
        runnables: Some(Cow::from(include_str!(
            "../../grammars/src/rust/runnables.scm"
        ))),
        debugger: Some(Cow::from(include_str!(
            "../../grammars/src/rust/debugger.scm"
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
            "../../grammars/src/markdown/brackets.scm"
        ))),
        injections: Some(Cow::from(include_str!(
            "../../grammars/src/markdown/injections.scm"
        ))),
        highlights: Some(Cow::from(include_str!(
            "../../grammars/src/markdown/highlights.scm"
        ))),
        indents: Some(Cow::from(include_str!(
            "../../grammars/src/markdown/indents.scm"
        ))),
        outline: Some(Cow::from(include_str!(
            "../../grammars/src/markdown/outline.scm"
        ))),
        ..LanguageQueries::default()
    })
    .expect("Could not parse markdown queries");
    Arc::new(language)
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::{TestAppContext, rgba};
    use pretty_assertions::assert_matches;

    #[test]
    fn test_highlight_map() {
        let theme = SyntaxTheme::new(
            [
                ("function", rgba(0x100000ff)),
                ("function.method", rgba(0x200000ff)),
                ("function.async", rgba(0x300000ff)),
                ("variable.builtin.self.rust", rgba(0x400000ff)),
                ("variable.builtin", rgba(0x500000ff)),
                ("variable", rgba(0x600000ff)),
            ]
            .iter()
            .map(|(name, color)| (name.to_string(), (*color).into())),
        );

        let capture_names = &[
            "function.special",
            "function.async.rust",
            "variable.builtin.self",
        ];

        let map = build_highlight_map(capture_names, &theme);
        assert_eq!(
            theme.get_capture_name(map.get(0).unwrap()),
            Some("function")
        );
        assert_eq!(
            theme.get_capture_name(map.get(1).unwrap()),
            Some("function.async")
        );
        assert_eq!(
            theme.get_capture_name(map.get(2).unwrap()),
            Some("variable.builtin")
        );
    }

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
