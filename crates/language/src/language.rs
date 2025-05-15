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

pub use crate::language_settings::EditPredictionsMode;
use crate::language_settings::SoftWrap;
use anyhow::{Context as _, Result, anyhow};
use async_trait::async_trait;
use collections::{HashMap, HashSet, IndexSet};
use fs::Fs;
use futures::Future;
use gpui::{App, AsyncApp, Entity, SharedString, Task};
pub use highlight_map::HighlightMap;
use http_client::HttpClient;
pub use language_registry::{LanguageName, LoadedLanguage};
use lsp::{CodeActionKind, InitializeParams, LanguageServerBinary, LanguageServerBinaryOptions};
pub use manifest::{ManifestName, ManifestProvider, ManifestQuery};
use parking_lot::Mutex;
use regex::Regex;
use schemars::{
    JsonSchema,
    r#gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject},
};
use serde::{Deserialize, Deserializer, Serialize, Serializer, de};
use serde_json::Value;
use settings::WorktreeId;
use smol::future::FutureExt as _;
use std::{
    any::Any,
    ffi::OsStr,
    fmt::Debug,
    hash::Hash,
    mem,
    ops::{DerefMut, Range},
    path::{Path, PathBuf},
    pin::Pin,
    str,
    sync::{
        Arc, LazyLock,
        atomic::{AtomicU64, AtomicUsize, Ordering::SeqCst},
    },
};
use std::{num::NonZeroU32, sync::OnceLock};
use syntax_map::{QueryCursorHandle, SyntaxSnapshot};
use task::RunnableTag;
pub use task_context::{ContextProvider, RunnableRange};
pub use text_diff::{DiffOptions, line_diff, text_diff, text_diff_with_options, unified_diff};
use theme::SyntaxTheme;
pub use toolchain::{LanguageToolchainStore, Toolchain, ToolchainList, ToolchainLister};
use tree_sitter::{self, Query, QueryCursor, WasmStore, wasmtime};
use util::serde::default_true;

pub use buffer::Operation;
pub use buffer::*;
pub use diagnostic_set::{DiagnosticEntry, DiagnosticGroup};
pub use language_registry::{
    AvailableLanguage, BinaryStatus, LanguageNotFound, LanguageQueries, LanguageRegistry,
    QUERY_FILENAME_PREFIXES,
};
pub use lsp::{LanguageServerId, LanguageServerName};
pub use outline::*;
pub use syntax_map::{OwnedSyntaxLayer, SyntaxLayer, ToTreeSitterPoint, TreeSitterOptions};
pub use text::{AnchorRangeExt, LineEnding};
pub use tree_sitter::{Node, Parser, Tree, TreeCursor};

/// Initializes the `language` crate.
///
/// This should be called before making use of items from the create.
pub fn init(cx: &mut App) {
    language_settings::init(cx);
}

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

static NEXT_LANGUAGE_ID: LazyLock<AtomicUsize> = LazyLock::new(Default::default);
static NEXT_GRAMMAR_ID: LazyLock<AtomicUsize> = LazyLock::new(Default::default);
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

/// Represents a Language Server, with certain cached sync properties.
/// Uses [`LspAdapter`] under the hood, but calls all 'static' methods
/// once at startup, and caches the results.
pub struct CachedLspAdapter {
    pub name: LanguageServerName,
    pub disk_based_diagnostic_sources: Vec<String>,
    pub disk_based_diagnostics_progress_token: Option<String>,
    language_ids: HashMap<String, String>,
    pub adapter: Arc<dyn LspAdapter>,
    pub reinstall_attempt_count: AtomicU64,
    cached_binary: futures::lock::Mutex<Option<LanguageServerBinary>>,
    manifest_name: OnceLock<Option<ManifestName>>,
    attach_kind: OnceLock<Attach>,
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
            .field("reinstall_attempt_count", &self.reinstall_attempt_count)
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
            reinstall_attempt_count: AtomicU64::new(0),
            attach_kind: Default::default(),
            manifest_name: Default::default(),
        })
    }

    pub fn name(&self) -> LanguageServerName {
        self.adapter.name().clone()
    }

    pub async fn get_language_server_command(
        self: Arc<Self>,
        delegate: Arc<dyn LspAdapterDelegate>,
        toolchains: Arc<dyn LanguageToolchainStore>,
        binary_options: LanguageServerBinaryOptions,
        cx: &mut AsyncApp,
    ) -> Result<LanguageServerBinary> {
        let cached_binary = self.cached_binary.lock().await;
        self.adapter
            .clone()
            .get_language_server_command(delegate, toolchains, binary_options, cached_binary, cx)
            .await
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
            .get(language_name.as_ref())
            .cloned()
            .unwrap_or_else(|| language_name.lsp_id())
    }
    pub fn manifest_name(&self) -> Option<ManifestName> {
        self.manifest_name
            .get_or_init(|| self.adapter.manifest_name())
            .clone()
    }
    pub fn attach_kind(&self) -> Attach {
        *self.attach_kind.get_or_init(|| self.adapter.attach_kind())
    }
}

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Attach {
    /// Create a single language server instance per subproject root.
    InstancePerRoot,
    /// Use one shared language server instance for all subprojects within a project.
    Shared,
}

impl Attach {
    pub fn root_path(
        &self,
        root_subproject_path: (WorktreeId, Arc<Path>),
    ) -> (WorktreeId, Arc<Path>) {
        match self {
            Attach::InstancePerRoot => root_subproject_path,
            Attach::Shared => (root_subproject_path.0, Arc::from(Path::new(""))),
        }
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
    fn exists(&self, path: &Path, is_dir: Option<bool>) -> bool;
    fn update_status(&self, language: LanguageServerName, status: BinaryStatus);
    fn registered_lsp_adapters(&self) -> Vec<Arc<dyn LspAdapter>>;
    async fn language_server_download_dir(&self, name: &LanguageServerName) -> Option<Arc<Path>>;

    async fn npm_package_installed_version(
        &self,
        package_name: &str,
    ) -> Result<Option<(PathBuf, String)>>;
    async fn which(&self, command: &OsStr) -> Option<PathBuf>;
    async fn shell_env(&self) -> HashMap<String, String>;
    async fn read_text_file(&self, path: PathBuf) -> Result<String>;
    async fn try_exec(&self, binary: LanguageServerBinary) -> Result<()>;
}

#[async_trait(?Send)]
pub trait LspAdapter: 'static + Send + Sync {
    fn name(&self) -> LanguageServerName;

    fn get_language_server_command<'a>(
        self: Arc<Self>,
        delegate: Arc<dyn LspAdapterDelegate>,
        toolchains: Arc<dyn LanguageToolchainStore>,
        binary_options: LanguageServerBinaryOptions,
        mut cached_binary: futures::lock::MutexGuard<'a, Option<LanguageServerBinary>>,
        cx: &'a mut AsyncApp,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<LanguageServerBinary>>>> {
        async move {
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
            if binary_options.allow_path_lookup {
                if let Some(binary) = self.check_if_user_installed(delegate.as_ref(), toolchains, cx).await {
                    log::info!(
                        "found user-installed language server for {}. path: {:?}, arguments: {:?}",
                        self.name().0,
                        binary.path,
                        binary.arguments
                    );
                    return Ok(binary);
                }
            }

            if !binary_options.allow_binary_download {
                return Err(anyhow!("downloading language servers disabled"));
            }

            if let Some(cached_binary) = cached_binary.as_ref() {
                return Ok(cached_binary.clone());
            }

            let Some(container_dir) = delegate.language_server_download_dir(&self.name()).await else {
                anyhow::bail!("no language server download dir defined")
            };

            let mut binary = try_fetch_server_binary(self.as_ref(), &delegate, container_dir.to_path_buf(), cx).await;

            if let Err(error) = binary.as_ref() {
                if let Some(prev_downloaded_binary) = self
                    .cached_server_binary(container_dir.to_path_buf(), delegate.as_ref())
                    .await
                {
                    log::info!(
                        "failed to fetch newest version of language server {:?}. error: {:?}, falling back to using {:?}",
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
                *cached_binary = Some(binary.clone());
            }

            binary
        }
        .boxed_local()
    }

    async fn check_if_user_installed(
        &self,
        _: &dyn LspAdapterDelegate,
        _: Arc<dyn LanguageToolchainStore>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        None
    }

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>>;

    fn will_fetch_server(
        &self,
        _: &Arc<dyn LspAdapterDelegate>,
        _: &mut AsyncApp,
    ) -> Option<Task<Result<()>>> {
        None
    }

    async fn check_if_version_installed(
        &self,
        _version: &(dyn 'static + Send + Any),
        _container_dir: &PathBuf,
        _delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        None
    }

    async fn fetch_server_binary(
        &self,
        latest_version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary>;

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary>;

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
        _: &dyn Fs,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<Value>> {
        Ok(None)
    }

    async fn workspace_configuration(
        self: Arc<Self>,
        _: &dyn Fs,
        _: &Arc<dyn LspAdapterDelegate>,
        _: Arc<dyn LanguageToolchainStore>,
        _cx: &mut AsyncApp,
    ) -> Result<Value> {
        Ok(serde_json::json!({}))
    }

    async fn additional_initialization_options(
        self: Arc<Self>,
        _target_language_server_id: LanguageServerName,
        _: &dyn Fs,
        _: &Arc<dyn LspAdapterDelegate>,
    ) -> Result<Option<Value>> {
        Ok(None)
    }

    async fn additional_workspace_configuration(
        self: Arc<Self>,
        _target_language_server_id: LanguageServerName,
        _: &dyn Fs,
        _: &Arc<dyn LspAdapterDelegate>,
        _: Arc<dyn LanguageToolchainStore>,
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

    fn language_ids(&self) -> HashMap<String, String> {
        Default::default()
    }

    /// Support custom initialize params.
    fn prepare_initialize_params(
        &self,
        original: InitializeParams,
        _: &App,
    ) -> Result<InitializeParams> {
        Ok(original)
    }

    fn attach_kind(&self) -> Attach {
        Attach::Shared
    }

    fn manifest_name(&self) -> Option<ManifestName> {
        None
    }

    /// Method only implemented by the default JSON language server adapter.
    /// Used to provide dynamic reloading of the JSON schemas used to
    /// provide autocompletion and diagnostics in Zed setting and keybind
    /// files
    fn is_primary_zed_json_schema_adapter(&self) -> bool {
        false
    }

    /// Method only implemented by the default JSON language server adapter.
    /// Used to clear the cache of JSON schemas that are used to provide
    /// autocompletion and diagnostics in Zed settings and keybinds files.
    /// Should not be called unless the callee is sure that
    /// `Self::is_primary_zed_json_schema_adapter` returns `true`
    async fn clear_zed_json_schema_cache(&self) {
        unreachable!(
            "Not implemented for this adapter. This method should only be called on the default JSON language server adapter"
        );
    }
}

async fn try_fetch_server_binary<L: LspAdapter + 'static + Send + Sync + ?Sized>(
    adapter: &L,
    delegate: &Arc<dyn LspAdapterDelegate>,
    container_dir: PathBuf,
    cx: &mut AsyncApp,
) -> Result<LanguageServerBinary> {
    if let Some(task) = adapter.will_fetch_server(delegate, cx) {
        task.await?;
    }

    let name = adapter.name();
    log::info!("fetching latest version of language server {:?}", name.0);
    delegate.update_status(name.clone(), BinaryStatus::CheckingForUpdate);

    let latest_version = adapter
        .fetch_latest_server_version(delegate.as_ref())
        .await?;

    if let Some(binary) = adapter
        .check_if_version_installed(latest_version.as_ref(), &container_dir, delegate.as_ref())
        .await
    {
        log::info!("language server {:?} is already installed", name.0);
        delegate.update_status(name.clone(), BinaryStatus::None);
        Ok(binary)
    } else {
        log::info!("downloading language server {:?}", name.0);
        delegate.update_status(adapter.name(), BinaryStatus::Downloading);
        let binary = adapter
            .fetch_server_binary(latest_version, container_dir, delegate.as_ref())
            .await;

        delegate.update_status(name.clone(), BinaryStatus::None);
        binary
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

#[derive(Clone, Deserialize, JsonSchema)]
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
    #[schemars(schema_with = "bracket_pair_config_json_schema")]
    pub brackets: BracketPairConfig,
    /// If set to true, indicates the language uses significant whitespace/indentation
    /// for syntax structure (like Python) rather than brackets/braces for code blocks.
    #[serde(default)]
    pub significant_indentation: bool,
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
    /// Starting and closing characters of a block comment.
    #[serde(default)]
    pub block_comment: Option<(Arc<str>, Arc<str>)>,
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
    pub tab_size: Option<NonZeroU32>,
    /// How to soft-wrap long lines of text.
    #[serde(default)]
    pub soft_wrap: Option<SoftWrap>,
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
    /// A list of preferred debuggers for this language.
    #[serde(default)]
    pub debuggers: IndexSet<SharedString>,
    /// A character to add as a prefix when a new line is added to a documentation block.
    #[serde(default)]
    pub documentation_line_prefix: Option<Arc<str>>,
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
#[derive(Clone, Deserialize, JsonSchema)]
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
    pub block_comment: Override<(Arc<str>, Arc<str>)>,
    #[serde(skip)]
    pub disabled_bracket_ixs: Vec<u16>,
    #[serde(default)]
    pub word_characters: Override<HashSet<char>>,
    #[serde(default)]
    pub completion_query_characters: Override<HashSet<char>>,
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
            name: LanguageName::new(""),
            code_fence_block_name: None,
            grammar: None,
            matcher: LanguageMatcher::default(),
            brackets: Default::default(),
            auto_indent_using_last_non_empty_line: auto_indent_using_last_non_empty_line_default(),
            auto_indent_on_paste: None,
            increase_indent_pattern: Default::default(),
            decrease_indent_pattern: Default::default(),
            autoclose_before: Default::default(),
            line_comments: Default::default(),
            block_comment: Default::default(),
            scope_opt_in_language_servers: Default::default(),
            overrides: Default::default(),
            word_characters: Default::default(),
            collapsed_placeholder: Default::default(),
            hard_tabs: None,
            tab_size: None,
            soft_wrap: None,
            prettier_parser_name: None,
            hidden: false,
            jsx_tag_auto_close: None,
            completion_query_characters: Default::default(),
            debuggers: Default::default(),
            significant_indentation: Default::default(),
            documentation_line_prefix: None,
        }
    }
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

fn regex_json_schema(_: &mut SchemaGenerator) -> Schema {
    Schema::Object(SchemaObject {
        instance_type: Some(InstanceType::String.into()),
        ..Default::default()
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
pub struct BracketPairConfig {
    /// A list of character pairs that should be treated as brackets in the context of a given language.
    pub pairs: Vec<BracketPair>,
    /// A list of tree-sitter scopes for which a given bracket should not be active.
    /// N-th entry in `[Self::disabled_scopes_by_bracket_ix]` contains a list of disabled scopes for an n-th entry in `[Self::pairs]`
    #[serde(skip)]
    pub disabled_scopes_by_bracket_ix: Vec<Vec<String>>,
}

impl BracketPairConfig {
    pub fn is_closing_brace(&self, c: char) -> bool {
        self.pairs.iter().any(|pair| pair.end.starts_with(c))
    }
}

fn bracket_pair_config_json_schema(r#gen: &mut SchemaGenerator) -> Schema {
    Option::<Vec<BracketPairContent>>::json_schema(r#gen)
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
        let mut brackets = Vec::with_capacity(result.len());
        let mut disabled_scopes_by_bracket_ix = Vec::with_capacity(result.len());
        for entry in result {
            brackets.push(entry.bracket_pair);
            disabled_scopes_by_bracket_ix.push(entry.not_in);
        }

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
    pub(crate) highlights_query: Option<Query>,
    pub(crate) brackets_config: Option<BracketsConfig>,
    pub(crate) redactions_config: Option<RedactionConfig>,
    pub(crate) runnable_config: Option<RunnableConfig>,
    pub(crate) indents_config: Option<IndentConfig>,
    pub outline_config: Option<OutlineConfig>,
    pub text_object_config: Option<TextObjectConfig>,
    pub embedding_config: Option<EmbeddingConfig>,
    pub(crate) injection_config: Option<InjectionConfig>,
    pub(crate) override_config: Option<OverrideConfig>,
    pub(crate) highlight_map: Mutex<HighlightMap>,
}

struct IndentConfig {
    query: Query,
    indent_capture_ix: u32,
    start_capture_ix: Option<u32>,
    end_capture_ix: Option<u32>,
    outdent_capture_ix: Option<u32>,
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

#[derive(Debug)]
pub struct EmbeddingConfig {
    pub query: Query,
    pub item_capture_ix: u32,
    pub name_capture_ix: Option<u32>,
    pub context_capture_ix: Option<u32>,
    pub collapse_capture_ix: Option<u32>,
    pub keep_capture_ix: Option<u32>,
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

struct BracketsConfig {
    query: Query,
    open_capture_ix: u32,
    close_capture_ix: u32,
    patterns: Vec<BracketsPatternConfig>,
}

#[derive(Clone, Debug, Default)]
struct BracketsPatternConfig {
    newline_only: bool,
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
                    highlights_query: None,
                    brackets_config: None,
                    outline_config: None,
                    text_object_config: None,
                    embedding_config: None,
                    indents_config: None,
                    injection_config: None,
                    override_config: None,
                    redactions_config: None,
                    runnable_config: None,
                    error_query: Query::new(&ts_language, "(ERROR) @error").ok(),
                    ts_language,
                    highlight_map: Default::default(),
                })
            }),
            context_provider: None,
            toolchain: None,
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
        if let Some(query) = queries.embedding {
            self = self
                .with_embedding_query(query.as_ref())
                .context("Error loading embedding query")?;
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
        Ok(self)
    }

    pub fn with_highlights_query(mut self, source: &str) -> Result<Self> {
        let grammar = self
            .grammar_mut()
            .ok_or_else(|| anyhow!("cannot mutate grammar"))?;
        grammar.highlights_query = Some(Query::new(&grammar.ts_language, source)?);
        Ok(self)
    }

    pub fn with_runnable_query(mut self, source: &str) -> Result<Self> {
        let grammar = self
            .grammar_mut()
            .ok_or_else(|| anyhow!("cannot mutate grammar"))?;

        let query = Query::new(&grammar.ts_language, source)?;
        let mut extra_captures = Vec::with_capacity(query.capture_names().len());

        for name in query.capture_names().iter() {
            let kind = if *name == "run" {
                RunnableCapture::Run
            } else {
                RunnableCapture::Named(name.to_string().into())
            };
            extra_captures.push(kind);
        }

        grammar.runnable_config = Some(RunnableConfig {
            extra_captures,
            query,
        });

        Ok(self)
    }

    pub fn with_outline_query(mut self, source: &str) -> Result<Self> {
        let grammar = self
            .grammar_mut()
            .ok_or_else(|| anyhow!("cannot mutate grammar"))?;
        let query = Query::new(&grammar.ts_language, source)?;
        let mut item_capture_ix = None;
        let mut name_capture_ix = None;
        let mut context_capture_ix = None;
        let mut extra_context_capture_ix = None;
        let mut open_capture_ix = None;
        let mut close_capture_ix = None;
        let mut annotation_capture_ix = None;
        get_capture_indices(
            &query,
            &mut [
                ("item", &mut item_capture_ix),
                ("name", &mut name_capture_ix),
                ("context", &mut context_capture_ix),
                ("context.extra", &mut extra_context_capture_ix),
                ("open", &mut open_capture_ix),
                ("close", &mut close_capture_ix),
                ("annotation", &mut annotation_capture_ix),
            ],
        );
        if let Some((item_capture_ix, name_capture_ix)) = item_capture_ix.zip(name_capture_ix) {
            grammar.outline_config = Some(OutlineConfig {
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
        let grammar = self
            .grammar_mut()
            .ok_or_else(|| anyhow!("cannot mutate grammar"))?;
        let query = Query::new(&grammar.ts_language, source)?;

        let mut text_objects_by_capture_ix = Vec::new();
        for (ix, name) in query.capture_names().iter().enumerate() {
            if let Some(text_object) = TextObject::from_capture_name(name) {
                text_objects_by_capture_ix.push((ix as u32, text_object));
            }
        }

        grammar.text_object_config = Some(TextObjectConfig {
            query,
            text_objects_by_capture_ix,
        });
        Ok(self)
    }

    pub fn with_embedding_query(mut self, source: &str) -> Result<Self> {
        let grammar = self
            .grammar_mut()
            .ok_or_else(|| anyhow!("cannot mutate grammar"))?;
        let query = Query::new(&grammar.ts_language, source)?;
        let mut item_capture_ix = None;
        let mut name_capture_ix = None;
        let mut context_capture_ix = None;
        let mut collapse_capture_ix = None;
        let mut keep_capture_ix = None;
        get_capture_indices(
            &query,
            &mut [
                ("item", &mut item_capture_ix),
                ("name", &mut name_capture_ix),
                ("context", &mut context_capture_ix),
                ("keep", &mut keep_capture_ix),
                ("collapse", &mut collapse_capture_ix),
            ],
        );
        if let Some(item_capture_ix) = item_capture_ix {
            grammar.embedding_config = Some(EmbeddingConfig {
                query,
                item_capture_ix,
                name_capture_ix,
                context_capture_ix,
                collapse_capture_ix,
                keep_capture_ix,
            });
        }
        Ok(self)
    }

    pub fn with_brackets_query(mut self, source: &str) -> Result<Self> {
        let grammar = self
            .grammar_mut()
            .ok_or_else(|| anyhow!("cannot mutate grammar"))?;
        let query = Query::new(&grammar.ts_language, source)?;
        let mut open_capture_ix = None;
        let mut close_capture_ix = None;
        get_capture_indices(
            &query,
            &mut [
                ("open", &mut open_capture_ix),
                ("close", &mut close_capture_ix),
            ],
        );
        let patterns = (0..query.pattern_count())
            .map(|ix| {
                let mut config = BracketsPatternConfig::default();
                for setting in query.property_settings(ix) {
                    match setting.key.as_ref() {
                        "newline.only" => config.newline_only = true,
                        _ => {}
                    }
                }
                config
            })
            .collect();
        if let Some((open_capture_ix, close_capture_ix)) = open_capture_ix.zip(close_capture_ix) {
            grammar.brackets_config = Some(BracketsConfig {
                query,
                open_capture_ix,
                close_capture_ix,
                patterns,
            });
        }
        Ok(self)
    }

    pub fn with_indents_query(mut self, source: &str) -> Result<Self> {
        let grammar = self
            .grammar_mut()
            .ok_or_else(|| anyhow!("cannot mutate grammar"))?;
        let query = Query::new(&grammar.ts_language, source)?;
        let mut indent_capture_ix = None;
        let mut start_capture_ix = None;
        let mut end_capture_ix = None;
        let mut outdent_capture_ix = None;
        get_capture_indices(
            &query,
            &mut [
                ("indent", &mut indent_capture_ix),
                ("start", &mut start_capture_ix),
                ("end", &mut end_capture_ix),
                ("outdent", &mut outdent_capture_ix),
            ],
        );
        if let Some(indent_capture_ix) = indent_capture_ix {
            grammar.indents_config = Some(IndentConfig {
                query,
                indent_capture_ix,
                start_capture_ix,
                end_capture_ix,
                outdent_capture_ix,
            });
        }
        Ok(self)
    }

    pub fn with_injection_query(mut self, source: &str) -> Result<Self> {
        let grammar = self
            .grammar_mut()
            .ok_or_else(|| anyhow!("cannot mutate grammar"))?;
        let query = Query::new(&grammar.ts_language, source)?;
        let mut language_capture_ix = None;
        let mut injection_language_capture_ix = None;
        let mut content_capture_ix = None;
        let mut injection_content_capture_ix = None;
        get_capture_indices(
            &query,
            &mut [
                ("language", &mut language_capture_ix),
                ("injection.language", &mut injection_language_capture_ix),
                ("content", &mut content_capture_ix),
                ("injection.content", &mut injection_content_capture_ix),
            ],
        );
        language_capture_ix = match (language_capture_ix, injection_language_capture_ix) {
            (None, Some(ix)) => Some(ix),
            (Some(_), Some(_)) => {
                return Err(anyhow!(
                    "both language and injection.language captures are present"
                ));
            }
            _ => language_capture_ix,
        };
        content_capture_ix = match (content_capture_ix, injection_content_capture_ix) {
            (None, Some(ix)) => Some(ix),
            (Some(_), Some(_)) => {
                return Err(anyhow!(
                    "both content and injection.content captures are present"
                ));
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
            grammar.injection_config = Some(InjectionConfig {
                query,
                language_capture_ix,
                content_capture_ix,
                patterns,
            });
        }
        Ok(self)
    }

    pub fn with_override_query(mut self, source: &str) -> anyhow::Result<Self> {
        let query = {
            let grammar = self
                .grammar
                .as_ref()
                .ok_or_else(|| anyhow!("no grammar for language"))?;
            Query::new(&grammar.ts_language, source)?
        };

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
                Err(anyhow!(
                    "language {:?} has overrides in config not in query: {referenced_name:?}",
                    self.config.name
                ))?;
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

        let grammar = self
            .grammar_mut()
            .ok_or_else(|| anyhow!("cannot mutate grammar"))?;
        grammar.override_config = Some(OverrideConfig {
            query,
            values: override_configs_by_id,
        });
        Ok(self)
    }

    pub fn with_redaction_query(mut self, source: &str) -> anyhow::Result<Self> {
        let grammar = self
            .grammar_mut()
            .ok_or_else(|| anyhow!("cannot mutate grammar"))?;

        let query = Query::new(&grammar.ts_language, source)?;
        let mut redaction_capture_ix = None;
        get_capture_indices(&query, &mut [("redact", &mut redaction_capture_ix)]);

        if let Some(redaction_capture_ix) = redaction_capture_ix {
            grammar.redactions_config = Some(RedactionConfig {
                query,
                redaction_capture_ix,
            });
        }

        Ok(self)
    }

    fn grammar_mut(&mut self) -> Option<&mut Grammar> {
        Arc::get_mut(self.grammar.as_mut()?)
    }

    pub fn name(&self) -> LanguageName {
        self.config.name.clone()
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
                    grammar.highlights_query.as_ref()
                });
            let highlight_maps = vec![grammar.highlight_map()];
            let mut offset = 0;
            for chunk in
                BufferChunks::new(text, range, Some((captures, highlight_maps)), false, None)
            {
                let end_offset = offset + chunk.text.len();
                if let Some(highlight_id) = chunk.syntax_highlight_id {
                    if !highlight_id.is_default() {
                        result.push((offset..end_offset, highlight_id));
                    }
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
        if let Some(grammar) = self.grammar.as_ref() {
            if let Some(highlights_query) = &grammar.highlights_query {
                *grammar.highlight_map.lock() =
                    HighlightMap::new(highlights_query.capture_names(), theme);
            }
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
        &self.language.path_suffixes()
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

    pub fn block_comment_delimiters(&self) -> Option<(&Arc<str>, &Arc<str>)> {
        Override::as_option(
            self.config_override().map(|o| &o.block_comment),
            self.language.config.block_comment.as_ref(),
        )
        .map(|e| (&e.0, &e.1))
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

    /// A character to add as a prefix when a new line is added to a documentation block.
    ///
    /// Used for documentation styles that require a leading character on each line,
    /// such as the asterisk in JSDoc, Javadoc, etc.
    pub fn documentation_line_prefix(&self) -> Option<&Arc<str>> {
        self.language.config.documentation_line_prefix.as_ref()
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
                if let Some(next_disabled_ix) = disabled_ids.first() {
                    if ix == *next_disabled_ix as usize {
                        disabled_ids = &disabled_ids[1..];
                        is_enabled = false;
                    }
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
        if opt_in_servers.iter().any(|o| *o == *name) {
            if let Some(over) = self.config_override() {
                over.opt_into_language_servers.iter().any(|o| *o == *name)
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
            .highlights_query
            .as_ref()?
            .capture_index_for_name(name)?;
        Some(self.highlight_map.lock().get(capture_id))
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
        Self {
            text,
            runs,
            filter_range: 0..label_length,
        }
    }

    pub fn plain(text: String, filter_text: Option<&str>) -> Self {
        let mut result = Self {
            runs: Vec::new(),
            filter_range: 0..text.len(),
            text,
        };
        if let Some(filter_text) = filter_text {
            if let Some(ix) = result.text.find(filter_text) {
                result.filter_range = ix..ix + filter_text.len();
            }
        }
        result
    }

    pub fn push_str(&mut self, text: &str, highlight: Option<HighlightId>) {
        let start_ix = self.text.len();
        self.text.push_str(text);
        let end_ix = self.text.len();
        if let Some(highlight) = highlight {
            self.runs.push((start_ix..end_ix, highlight));
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
#[async_trait(?Send)]
impl LspAdapter for FakeLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName(self.name.into())
    }

    async fn check_if_user_installed(
        &self,
        _: &dyn LspAdapterDelegate,
        _: Arc<dyn LanguageToolchainStore>,
        _: &AsyncApp,
    ) -> Option<LanguageServerBinary> {
        Some(self.language_server_binary.clone())
    }

    fn get_language_server_command<'a>(
        self: Arc<Self>,
        _: Arc<dyn LspAdapterDelegate>,
        _: Arc<dyn LanguageToolchainStore>,
        _: LanguageServerBinaryOptions,
        _: futures::lock::MutexGuard<'a, Option<LanguageServerBinary>>,
        _: &'a mut AsyncApp,
    ) -> Pin<Box<dyn 'a + Future<Output = Result<LanguageServerBinary>>>> {
        async move { Ok(self.language_server_binary.clone()) }.boxed_local()
    }

    async fn fetch_latest_server_version(
        &self,
        _: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        unreachable!();
    }

    async fn fetch_server_binary(
        &self,
        _: Box<dyn 'static + Send + Any>,
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

    fn disk_based_diagnostic_sources(&self) -> Vec<String> {
        self.disk_based_diagnostics_sources.clone()
    }

    fn disk_based_diagnostics_progress_token(&self) -> Option<String> {
        self.disk_based_diagnostics_progress_token.clone()
    }

    async fn initialization_options(
        self: Arc<Self>,
        _: &dyn Fs,
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
}

fn get_capture_indices(query: &Query, captures: &mut [(&str, &mut Option<u32>)]) {
    for (ix, name) in query.capture_names().iter().enumerate() {
        for (capture_name, index) in captures.iter_mut() {
            if capture_name == name {
                **index = Some(ix as u32);
                break;
            }
        }
    }
}

pub fn point_to_lsp(point: PointUtf16) -> lsp::Position {
    lsp::Position::new(point.row, point.column)
}

pub fn point_from_lsp(point: lsp::Position) -> Unclipped<PointUtf16> {
    Unclipped(PointUtf16::new(point.line, point.character))
}

pub fn range_to_lsp(range: Range<PointUtf16>) -> Result<lsp::Range> {
    if range.start > range.end {
        Err(anyhow!(
            "Inverted range provided to an LSP request: {:?}-{:?}",
            range.start,
            range.end
        ))
    } else {
        Ok(lsp::Range {
            start: point_to_lsp(range.start),
            end: point_to_lsp(range.end),
        })
    }
}

pub fn range_from_lsp(range: lsp::Range) -> Range<Unclipped<PointUtf16>> {
    let mut start = point_from_lsp(range.start);
    let mut end = point_from_lsp(range.end);
    if start > end {
        log::warn!("range_from_lsp called with inverted range {start:?}-{end:?}");
        mem::swap(&mut start, &mut end);
    }
    start..end
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

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
                "JSON".to_string(),
                "Plain Text".to_string(),
                "Rust".to_string(),
            ]
        );

        let rust1 = languages.language_for_name("Rust");
        let rust2 = languages.language_for_name("Rust");

        // Ensure language is still listed even if it's being loaded.
        assert_eq!(
            languages.language_names(),
            &[
                "JSON".to_string(),
                "Plain Text".to_string(),
                "Rust".to_string(),
            ]
        );

        let (rust1, rust2) = futures::join!(rust1, rust2);
        assert!(Arc::ptr_eq(&rust1.unwrap(), &rust2.unwrap()));

        // Ensure language is still listed even after loading it.
        assert_eq!(
            languages.language_names(),
            &[
                "JSON".to_string(),
                "Plain Text".to_string(),
                "Rust".to_string(),
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
}
