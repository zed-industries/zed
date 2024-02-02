//! The `language` crate provides a large chunk of Zed's language-related
//! features (the other big contributors being project and lsp crates that revolve around LSP features).
//! Namely, this crate:
//! - Provides [`Language`], [`Grammar`] and [`LanguageRegistry`] types that
//! use Tree-sitter to provide syntax highlighting to the editor; note though that `language` doesn't perform the highlighting by itself. It only maps ranges in a buffer to colors. Treesitter is also used for buffer outlines (lists of symbols in a buffer)
//! - Exposes [`LanguageConfig`] that describes how constructs (like brackets or line comments) should be handled by the editor for a source file of a particular language.
//!
//! Notably we do *not* assign a single language to a single file; in real world a single file can consist of multiple programming languages - HTML is a good example of that - and `language` crate tends to reflect that status quo in it's API.
mod buffer;
mod diagnostic_set;
mod highlight_map;
pub mod language_settings;
mod outline;
pub mod proto;
mod syntax_map;

#[cfg(test)]
mod buffer_tests;
pub mod markdown;

use anyhow::{anyhow, Context, Result};
use async_trait::async_trait;
use collections::{HashMap, HashSet};
use futures::{
    channel::{mpsc, oneshot},
    future::Shared,
    FutureExt, TryFutureExt as _,
};
use gpui::{AppContext, AsyncAppContext, BackgroundExecutor, Task};
pub use highlight_map::HighlightMap;
use lazy_static::lazy_static;
use lsp::{CodeActionKind, LanguageServerBinary};
use parking_lot::{Mutex, RwLock};
use postage::watch;
use regex::Regex;
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::{
    any::Any,
    borrow::Cow,
    cell::RefCell,
    fmt::Debug,
    hash::Hash,
    mem,
    ops::{Not, Range},
    path::{Path, PathBuf},
    str,
    sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};
use syntax_map::SyntaxSnapshot;
use theme::{SyntaxTheme, Theme};
use tree_sitter::{self, wasmtime, Query, WasmStore};
use unicase::UniCase;
use util::{http::HttpClient, paths::PathExt};
use util::{post_inc, ResultExt, TryFutureExt as _, UnwrapFuture};

pub use buffer::Operation;
pub use buffer::*;
pub use diagnostic_set::DiagnosticEntry;
pub use lsp::LanguageServerId;
pub use outline::{Outline, OutlineItem};
pub use syntax_map::{OwnedSyntaxLayer, SyntaxLayer};
pub use text::LineEnding;
pub use tree_sitter::{Parser, Tree};

/// Initializes the `language` crate.
///
/// This should be called before making use of items from the create.
pub fn init(cx: &mut AppContext) {
    language_settings::init(cx);
}

#[derive(Clone, Default)]
struct LspBinaryStatusSender {
    txs: Arc<Mutex<Vec<mpsc::UnboundedSender<(Arc<Language>, LanguageServerBinaryStatus)>>>>,
}

impl LspBinaryStatusSender {
    fn subscribe(&self) -> mpsc::UnboundedReceiver<(Arc<Language>, LanguageServerBinaryStatus)> {
        let (tx, rx) = mpsc::unbounded();
        self.txs.lock().push(tx);
        rx
    }

    fn send(&self, language: Arc<Language>, status: LanguageServerBinaryStatus) {
        let mut txs = self.txs.lock();
        txs.retain(|tx| {
            tx.unbounded_send((language.clone(), status.clone()))
                .is_ok()
        });
    }
}

thread_local! {
    static PARSER: RefCell<Parser> = {
        let mut parser = Parser::new();
        parser.set_wasm_store(WasmStore::new(WASM_ENGINE.clone()).unwrap()).unwrap();
        RefCell::new(parser)
    };
}

lazy_static! {
    pub(crate) static ref NEXT_GRAMMAR_ID: AtomicUsize = Default::default();
    /// A shared grammar for plain text, exposed for reuse by downstream crates.
    #[doc(hidden)]
    pub static ref WASM_ENGINE: wasmtime::Engine = wasmtime::Engine::default();
    pub static ref PLAIN_TEXT: Arc<Language> = Arc::new(Language::new(
        LanguageConfig {
            name: "Plain Text".into(),
            ..Default::default()
        },
        None,
    ));
}

/// Types that represent a position in a buffer, and can be converted into
/// an LSP position, to send to a language server.
pub trait ToLspPosition {
    /// Converts the value into an LSP position.
    fn to_lsp_position(self) -> lsp::Position;
}

/// A name of a language server.
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LanguageServerName(pub Arc<str>);

/// Represents a Language Server, with certain cached sync properties.
/// Uses [`LspAdapter`] under the hood, but calls all 'static' methods
/// once at startup, and caches the results.
pub struct CachedLspAdapter {
    pub name: LanguageServerName,
    pub short_name: &'static str,
    pub disk_based_diagnostic_sources: Vec<String>,
    pub disk_based_diagnostics_progress_token: Option<String>,
    pub language_ids: HashMap<String, String>,
    pub adapter: Arc<dyn LspAdapter>,
    pub reinstall_attempt_count: AtomicU64,
}

impl CachedLspAdapter {
    pub async fn new(adapter: Arc<dyn LspAdapter>) -> Arc<Self> {
        let name = adapter.name();
        let short_name = adapter.short_name();
        let disk_based_diagnostic_sources = adapter.disk_based_diagnostic_sources();
        let disk_based_diagnostics_progress_token = adapter.disk_based_diagnostics_progress_token();
        let language_ids = adapter.language_ids();

        Arc::new(CachedLspAdapter {
            name,
            short_name,
            disk_based_diagnostic_sources,
            disk_based_diagnostics_progress_token,
            language_ids,
            adapter,
            reinstall_attempt_count: AtomicU64::new(0),
        })
    }

    pub async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        self.adapter.fetch_latest_server_version(delegate).await
    }

    pub fn will_fetch_server(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Option<Task<Result<()>>> {
        self.adapter.will_fetch_server(delegate, cx)
    }

    pub fn will_start_server(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Option<Task<Result<()>>> {
        self.adapter.will_start_server(delegate, cx)
    }

    pub async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary> {
        self.adapter
            .fetch_server_binary(version, container_dir, delegate)
            .await
    }

    pub async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary> {
        self.adapter
            .cached_server_binary(container_dir, delegate)
            .await
    }

    pub fn can_be_reinstalled(&self) -> bool {
        self.adapter.can_be_reinstalled()
    }

    pub async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary> {
        self.adapter.installation_test_binary(container_dir).await
    }

    pub fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        self.adapter.code_action_kinds()
    }

    pub fn workspace_configuration(&self, workspace_root: &Path, cx: &mut AppContext) -> Value {
        self.adapter.workspace_configuration(workspace_root, cx)
    }

    pub fn process_diagnostics(&self, params: &mut lsp::PublishDiagnosticsParams) {
        self.adapter.process_diagnostics(params)
    }

    pub async fn process_completion(&self, completion_item: &mut lsp::CompletionItem) {
        self.adapter.process_completion(completion_item).await
    }

    pub async fn label_for_completion(
        &self,
        completion_item: &lsp::CompletionItem,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        self.adapter
            .label_for_completion(completion_item, language)
            .await
    }

    pub async fn label_for_symbol(
        &self,
        name: &str,
        kind: lsp::SymbolKind,
        language: &Arc<Language>,
    ) -> Option<CodeLabel> {
        self.adapter.label_for_symbol(name, kind, language).await
    }

    pub fn prettier_plugins(&self) -> &[&'static str] {
        self.adapter.prettier_plugins()
    }
}

/// [`LspAdapterDelegate`] allows [`LspAdapter]` implementations to interface with the application
// e.g. to display a notification or fetch data from the web.
pub trait LspAdapterDelegate: Send + Sync {
    fn show_notification(&self, message: &str, cx: &mut AppContext);
    fn http_client(&self) -> Arc<dyn HttpClient>;
}

#[async_trait]
pub trait LspAdapter: 'static + Send + Sync {
    fn name(&self) -> LanguageServerName;

    fn short_name(&self) -> &'static str;

    async fn fetch_latest_server_version(
        &self,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<Box<dyn 'static + Send + Any>>;

    fn will_fetch_server(
        &self,
        _: &Arc<dyn LspAdapterDelegate>,
        _: &mut AsyncAppContext,
    ) -> Option<Task<Result<()>>> {
        None
    }

    fn will_start_server(
        &self,
        _: &Arc<dyn LspAdapterDelegate>,
        _: &mut AsyncAppContext,
    ) -> Option<Task<Result<()>>> {
        None
    }

    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Result<LanguageServerBinary>;

    async fn cached_server_binary(
        &self,
        container_dir: PathBuf,
        delegate: &dyn LspAdapterDelegate,
    ) -> Option<LanguageServerBinary>;

    /// Returns `true` if a language server can be reinstalled.
    ///
    /// If language server initialization fails, a reinstallation will be attempted unless the value returned from this method is `false`.
    ///
    /// Implementations that rely on software already installed on user's system
    /// should have [`can_be_reinstalled`](Self::can_be_reinstalled) return `false`.
    fn can_be_reinstalled(&self) -> bool {
        true
    }

    async fn installation_test_binary(
        &self,
        container_dir: PathBuf,
    ) -> Option<LanguageServerBinary>;

    fn process_diagnostics(&self, _: &mut lsp::PublishDiagnosticsParams) {}

    /// A callback called for each [`lsp::CompletionItem`] obtained from LSP server.
    /// Some LspAdapter implementations might want to modify the obtained item to
    /// change how it's displayed.
    async fn process_completion(&self, _: &mut lsp::CompletionItem) {}

    async fn label_for_completion(
        &self,
        _: &lsp::CompletionItem,
        _: &Arc<Language>,
    ) -> Option<CodeLabel> {
        None
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
    fn initialization_options(&self) -> Option<Value> {
        None
    }

    fn workspace_configuration(&self, _workspace_root: &Path, _cx: &mut AppContext) -> Value {
        serde_json::json!({})
    }

    /// Returns a list of code actions supported by a given LspAdapter
    fn code_action_kinds(&self) -> Option<Vec<CodeActionKind>> {
        Some(vec![
            CodeActionKind::EMPTY,
            CodeActionKind::QUICKFIX,
            CodeActionKind::REFACTOR,
            CodeActionKind::REFACTOR_EXTRACT,
            CodeActionKind::SOURCE,
        ])
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

    fn prettier_plugins(&self) -> &[&'static str] {
        &[]
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeLabel {
    /// The text to display.
    pub text: String,
    /// Syntax highlighting runs.
    pub runs: Vec<(Range<usize>, HighlightId)>,
    /// The portion of the text that should be used in fuzzy filtering.
    pub filter_range: Range<usize>,
}

#[derive(Clone, Deserialize)]
pub struct LanguageConfig {
    /// Human-readable name of the language.
    pub name: Arc<str>,
    // The name of the grammar in a WASM bundle (experimental).
    pub grammar_name: Option<Arc<str>>,
    /// Given a list of `LanguageConfig`'s, the language of a file can be determined based on the path extension matching any of the `path_suffixes`.
    pub path_suffixes: Vec<String>,
    /// List of bracket types in a language.
    pub brackets: BracketPairConfig,
    /// A regex pattern that determines whether the language should be assigned to a file or not.
    #[serde(default, deserialize_with = "deserialize_regex")]
    pub first_line_pattern: Option<Regex>,
    /// If set to true, auto indentation uses last non empty line to determine
    /// the indentation level for a new line.
    #[serde(default = "auto_indent_using_last_non_empty_line_default")]
    pub auto_indent_using_last_non_empty_line: bool,
    /// A regex that is used to determine whether the indentation level should be
    /// increased in the following line.
    #[serde(default, deserialize_with = "deserialize_regex")]
    pub increase_indent_pattern: Option<Regex>,
    /// A regex that is used to determine whether the indentation level should be
    /// decreased in the following line.
    #[serde(default, deserialize_with = "deserialize_regex")]
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
    pub scope_opt_in_language_servers: Vec<String>,
    #[serde(default)]
    pub overrides: HashMap<String, LanguageConfigOverride>,
    /// A list of characters that Zed should treat as word characters for the
    /// purpose of features that operate on word boundaries, like 'move to next word end'
    /// or a whole-word search in buffer search.
    #[serde(default)]
    pub word_characters: HashSet<char>,
    /// The name of a Prettier parser that should be used for this language.
    #[serde(default)]
    pub prettier_parser_name: Option<String>,
}

/// Tree-sitter language queries for a given language.
#[derive(Debug, Default)]
pub struct LanguageQueries {
    pub highlights: Option<Cow<'static, str>>,
    pub brackets: Option<Cow<'static, str>>,
    pub indents: Option<Cow<'static, str>>,
    pub outline: Option<Cow<'static, str>>,
    pub embedding: Option<Cow<'static, str>>,
    pub injections: Option<Cow<'static, str>>,
    pub overrides: Option<Cow<'static, str>>,
    pub redactions: Option<Cow<'static, str>>,
}

/// Represents a language for the given range. Some languages (e.g. HTML)
/// interleave several languages together, thus a single buffer might actually contain
/// several nested scopes.
#[derive(Clone, Debug)]
pub struct LanguageScope {
    language: Arc<Language>,
    override_id: Option<u32>,
}

#[derive(Clone, Deserialize, Default, Debug)]
pub struct LanguageConfigOverride {
    #[serde(default)]
    pub line_comments: Override<Vec<Arc<str>>>,
    #[serde(default)]
    pub block_comment: Override<(Arc<str>, Arc<str>)>,
    #[serde(skip_deserializing)]
    pub disabled_bracket_ixs: Vec<u16>,
    #[serde(default)]
    pub word_characters: Override<HashSet<char>>,
    #[serde(default)]
    pub opt_into_language_servers: Vec<String>,
}

#[derive(Clone, Deserialize, Debug)]
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
            name: "".into(),
            grammar_name: None,
            path_suffixes: Default::default(),
            brackets: Default::default(),
            auto_indent_using_last_non_empty_line: auto_indent_using_last_non_empty_line_default(),
            first_line_pattern: Default::default(),
            increase_indent_pattern: Default::default(),
            decrease_indent_pattern: Default::default(),
            autoclose_before: Default::default(),
            line_comments: Default::default(),
            block_comment: Default::default(),
            scope_opt_in_language_servers: Default::default(),
            overrides: Default::default(),
            word_characters: Default::default(),
            prettier_parser_name: None,
            collapsed_placeholder: Default::default(),
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

#[doc(hidden)]
#[cfg(any(test, feature = "test-support"))]
pub struct FakeLspAdapter {
    pub name: &'static str,
    pub initialization_options: Option<Value>,
    pub capabilities: lsp::ServerCapabilities,
    pub initializer: Option<Box<dyn 'static + Send + Sync + Fn(&mut lsp::FakeLanguageServer)>>,
    pub disk_based_diagnostics_progress_token: Option<String>,
    pub disk_based_diagnostics_sources: Vec<String>,
    pub prettier_plugins: Vec<&'static str>,
}

/// Configuration of handling bracket pairs for a given language.
///
/// This struct includes settings for defining which pairs of characters are considered brackets and
/// also specifies any language-specific scopes where these pairs should be ignored for bracket matching purposes.
#[derive(Clone, Debug, Default)]
pub struct BracketPairConfig {
    /// A list of character pairs that should be treated as brackets in the context of a given language.
    pub pairs: Vec<BracketPair>,
    /// A list of tree-sitter scopes for which a given bracket should not be active.
    /// N-th entry in `[Self::disabled_scopes_by_bracket_ix]` contains a list of disabled scopes for an n-th entry in `[Self::pairs]`
    pub disabled_scopes_by_bracket_ix: Vec<Vec<String>>,
}

impl<'de> Deserialize<'de> for BracketPairConfig {
    fn deserialize<D>(deserializer: D) -> std::result::Result<Self, D::Error>
    where
        D: Deserializer<'de>,
    {
        #[derive(Deserialize)]
        pub struct Entry {
            #[serde(flatten)]
            pub bracket_pair: BracketPair,
            #[serde(default)]
            pub not_in: Vec<String>,
        }

        let result = Vec::<Entry>::deserialize(deserializer)?;
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
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
pub struct BracketPair {
    /// Starting substring for a bracket.
    pub start: String,
    /// Ending substring for a bracket.
    pub end: String,
    /// True if `end` should be automatically inserted right after `start` characters.
    pub close: bool,
    /// True if an extra newline should be inserted while the cursor is in the middle
    /// of that bracket pair.
    pub newline: bool,
}

pub struct Language {
    pub(crate) config: LanguageConfig,
    pub(crate) grammar: Option<Arc<Grammar>>,
    pub(crate) adapters: Vec<Arc<CachedLspAdapter>>,

    #[cfg(any(test, feature = "test-support"))]
    fake_adapter: Option<(
        mpsc::UnboundedSender<lsp::FakeLanguageServer>,
        Arc<FakeLspAdapter>,
    )>,
}

pub struct Grammar {
    id: usize,
    pub ts_language: tree_sitter::Language,
    pub(crate) error_query: Query,
    pub(crate) highlights_query: Option<Query>,
    pub(crate) brackets_config: Option<BracketConfig>,
    pub(crate) redactions_config: Option<RedactionConfig>,
    pub(crate) indents_config: Option<IndentConfig>,
    pub outline_config: Option<OutlineConfig>,
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

struct OverrideConfig {
    query: Query,
    values: HashMap<u32, (String, LanguageConfigOverride)>,
}

#[derive(Default, Clone)]
struct InjectionPatternConfig {
    language: Option<Box<str>>,
    combined: bool,
}

struct BracketConfig {
    query: Query,
    open_capture_ix: u32,
    close_capture_ix: u32,
}

#[derive(Clone)]
pub enum LanguageServerBinaryStatus {
    CheckingForUpdate,
    Downloading,
    Downloaded,
    Cached,
    Failed { error: String },
}

type AvailableLanguageId = usize;

#[derive(Clone)]
struct AvailableLanguage {
    id: AvailableLanguageId,
    config: LanguageConfig,
    grammar: AvailableGrammar,
    lsp_adapters: Vec<Arc<dyn LspAdapter>>,
    loaded: bool,
}

#[derive(Clone)]
enum AvailableGrammar {
    Native {
        grammar: tree_sitter::Language,
        asset_dir: &'static str,
        get_queries: fn(&str) -> LanguageQueries,
    },
    Wasm {
        path: Arc<Path>,
        get_queries: fn(&Path) -> LanguageQueries,
    },
}

pub struct LanguageRegistry {
    state: RwLock<LanguageRegistryState>,
    language_server_download_dir: Option<Arc<Path>>,
    login_shell_env_loaded: Shared<Task<()>>,
    #[allow(clippy::type_complexity)]
    lsp_binary_paths: Mutex<
        HashMap<LanguageServerName, Shared<Task<Result<LanguageServerBinary, Arc<anyhow::Error>>>>>,
    >,
    executor: Option<BackgroundExecutor>,
    lsp_binary_status_tx: LspBinaryStatusSender,
}

struct LanguageRegistryState {
    next_language_server_id: usize,
    languages: Vec<Arc<Language>>,
    available_languages: Vec<AvailableLanguage>,
    next_available_language_id: AvailableLanguageId,
    loading_languages: HashMap<AvailableLanguageId, Vec<oneshot::Sender<Result<Arc<Language>>>>>,
    subscription: (watch::Sender<()>, watch::Receiver<()>),
    theme: Option<Arc<Theme>>,
    version: usize,
    reload_count: usize,
}

pub struct PendingLanguageServer {
    pub server_id: LanguageServerId,
    pub task: Task<Result<lsp::LanguageServer>>,
    pub container_dir: Option<Arc<Path>>,
}

impl LanguageRegistry {
    pub fn new(login_shell_env_loaded: Task<()>) -> Self {
        Self {
            state: RwLock::new(LanguageRegistryState {
                next_language_server_id: 0,
                languages: vec![PLAIN_TEXT.clone()],
                available_languages: Default::default(),
                next_available_language_id: 0,
                loading_languages: Default::default(),
                subscription: watch::channel(),
                theme: Default::default(),
                version: 0,
                reload_count: 0,
            }),
            language_server_download_dir: None,
            login_shell_env_loaded: login_shell_env_loaded.shared(),
            lsp_binary_paths: Default::default(),
            executor: None,
            lsp_binary_status_tx: Default::default(),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test() -> Self {
        Self::new(Task::ready(()))
    }

    pub fn set_executor(&mut self, executor: BackgroundExecutor) {
        self.executor = Some(executor);
    }

    /// Clear out all of the loaded languages and reload them from scratch.
    pub fn reload(&self) {
        self.state.write().reload();
    }

    pub fn register(
        &self,
        asset_dir: &'static str,
        config: LanguageConfig,
        grammar: tree_sitter::Language,
        lsp_adapters: Vec<Arc<dyn LspAdapter>>,
        get_queries: fn(&str) -> LanguageQueries,
    ) {
        let state = &mut *self.state.write();
        state.available_languages.push(AvailableLanguage {
            id: post_inc(&mut state.next_available_language_id),
            config,
            grammar: AvailableGrammar::Native {
                grammar,
                get_queries,
                asset_dir,
            },
            lsp_adapters,
            loaded: false,
        });
    }

    pub fn register_wasm(
        &self,
        path: Arc<Path>,
        config: LanguageConfig,
        get_queries: fn(&Path) -> LanguageQueries,
    ) {
        let state = &mut *self.state.write();
        state.available_languages.push(AvailableLanguage {
            id: post_inc(&mut state.next_available_language_id),
            config,
            grammar: AvailableGrammar::Wasm { path, get_queries },
            lsp_adapters: Vec::new(),
            loaded: false,
        });
    }

    pub fn language_names(&self) -> Vec<String> {
        let state = self.state.read();
        let mut result = state
            .available_languages
            .iter()
            .filter_map(|l| l.loaded.not().then_some(l.config.name.to_string()))
            .chain(state.languages.iter().map(|l| l.config.name.to_string()))
            .collect::<Vec<_>>();
        result.sort_unstable_by_key(|language_name| language_name.to_lowercase());
        result
    }

    pub fn add(&self, language: Arc<Language>) {
        self.state.write().add(language);
    }

    pub fn subscribe(&self) -> watch::Receiver<()> {
        self.state.read().subscription.1.clone()
    }

    /// The number of times that the registry has been changed,
    /// by adding languages or reloading.
    pub fn version(&self) -> usize {
        self.state.read().version
    }

    /// The number of times that the registry has been reloaded.
    pub fn reload_count(&self) -> usize {
        self.state.read().reload_count
    }

    pub fn set_theme(&self, theme: Arc<Theme>) {
        let mut state = self.state.write();
        state.theme = Some(theme.clone());
        for language in &state.languages {
            language.set_theme(theme.syntax());
        }
    }

    pub fn set_language_server_download_dir(&mut self, path: impl Into<Arc<Path>>) {
        self.language_server_download_dir = Some(path.into());
    }

    pub fn language_for_name(
        self: &Arc<Self>,
        name: &str,
    ) -> UnwrapFuture<oneshot::Receiver<Result<Arc<Language>>>> {
        let name = UniCase::new(name);
        self.get_or_load_language(|config| UniCase::new(config.name.as_ref()) == name)
    }

    pub fn language_for_name_or_extension(
        self: &Arc<Self>,
        string: &str,
    ) -> UnwrapFuture<oneshot::Receiver<Result<Arc<Language>>>> {
        let string = UniCase::new(string);
        self.get_or_load_language(|config| {
            UniCase::new(config.name.as_ref()) == string
                || config
                    .path_suffixes
                    .iter()
                    .any(|suffix| UniCase::new(suffix) == string)
        })
    }

    pub fn language_for_file(
        self: &Arc<Self>,
        path: impl AsRef<Path>,
        content: Option<&Rope>,
    ) -> UnwrapFuture<oneshot::Receiver<Result<Arc<Language>>>> {
        let path = path.as_ref();
        let filename = path.file_name().and_then(|name| name.to_str());
        let extension = path.extension_or_hidden_file_name();
        let path_suffixes = [extension, filename];
        self.get_or_load_language(|config| {
            let path_matches = config
                .path_suffixes
                .iter()
                .any(|suffix| path_suffixes.contains(&Some(suffix.as_str())));
            let content_matches = content.zip(config.first_line_pattern.as_ref()).map_or(
                false,
                |(content, pattern)| {
                    let end = content.clip_point(Point::new(0, 256), Bias::Left);
                    let end = content.point_to_offset(end);
                    let text = content.chunks_in_range(0..end).collect::<String>();
                    pattern.is_match(&text)
                },
            );
            path_matches || content_matches
        })
    }

    fn get_or_load_language(
        self: &Arc<Self>,
        callback: impl Fn(&LanguageConfig) -> bool,
    ) -> UnwrapFuture<oneshot::Receiver<Result<Arc<Language>>>> {
        let (tx, rx) = oneshot::channel();

        let mut state = self.state.write();
        if let Some(language) = state
            .languages
            .iter()
            .find(|language| callback(&language.config))
        {
            let _ = tx.send(Ok(language.clone()));
        } else if let Some(executor) = self.executor.clone() {
            if let Some(language) = state
                .available_languages
                .iter()
                .find(|l| !l.loaded && callback(&l.config))
                .cloned()
            {
                let txs = state
                    .loading_languages
                    .entry(language.id)
                    .or_insert_with(|| {
                        let this = self.clone();
                        executor
                            .spawn(async move {
                                let id = language.id;
                                let name = language.config.name.clone();
                                let language = async {
                                    let (grammar, queries) = match language.grammar {
                                        AvailableGrammar::Native {
                                            grammar,
                                            asset_dir,
                                            get_queries,
                                        } => (grammar, (get_queries)(asset_dir)),
                                        AvailableGrammar::Wasm { path, get_queries } => {
                                            let grammar_name =
                                                &language.config.grammar_name.as_ref().ok_or_else(
                                                    || anyhow!("missing grammar name"),
                                                )?;
                                            let mut wasm_path = path.join(grammar_name.as_ref());
                                            wasm_path.set_extension("wasm");
                                            let wasm_bytes = std::fs::read(&wasm_path)?;
                                            let grammar = PARSER.with(|parser| {
                                                let mut parser = parser.borrow_mut();
                                                let mut store = parser.take_wasm_store().unwrap();
                                                let grammar =
                                                    store.load_language(&grammar_name, &wasm_bytes);
                                                parser.set_wasm_store(store).unwrap();
                                                grammar
                                            })?;
                                            (grammar, get_queries(path.as_ref()))
                                        }
                                    };
                                    Language::new(language.config, Some(grammar))
                                        .with_lsp_adapters(language.lsp_adapters)
                                        .await
                                        .with_queries(queries)
                                }
                                .await;

                                match language {
                                    Ok(language) => {
                                        let language = Arc::new(language);
                                        let mut state = this.state.write();

                                        state.add(language.clone());
                                        state.mark_language_loaded(id);
                                        if let Some(mut txs) = state.loading_languages.remove(&id) {
                                            for tx in txs.drain(..) {
                                                let _ = tx.send(Ok(language.clone()));
                                            }
                                        }
                                    }
                                    Err(e) => {
                                        log::error!("failed to load language {name}:\n{:?}", e);
                                        let mut state = this.state.write();
                                        state.mark_language_loaded(id);
                                        if let Some(mut txs) = state.loading_languages.remove(&id) {
                                            for tx in txs.drain(..) {
                                                let _ = tx.send(Err(anyhow!(
                                                    "failed to load language {}: {}",
                                                    name,
                                                    e
                                                )));
                                            }
                                        }
                                    }
                                };
                            })
                            .detach();

                        Vec::new()
                    });
                txs.push(tx);
            } else {
                let _ = tx.send(Err(anyhow!("language not found")));
            }
        } else {
            let _ = tx.send(Err(anyhow!("executor does not exist")));
        }

        rx.unwrap()
    }

    pub fn to_vec(&self) -> Vec<Arc<Language>> {
        self.state.read().languages.iter().cloned().collect()
    }

    pub fn create_pending_language_server(
        self: &Arc<Self>,
        stderr_capture: Arc<Mutex<Option<String>>>,
        language: Arc<Language>,
        adapter: Arc<CachedLspAdapter>,
        root_path: Arc<Path>,
        delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut AppContext,
    ) -> Option<PendingLanguageServer> {
        let server_id = self.state.write().next_language_server_id();
        log::info!(
            "starting language server {:?}, path: {root_path:?}, id: {server_id}",
            adapter.name.0
        );

        #[cfg(any(test, feature = "test-support"))]
        if language.fake_adapter.is_some() {
            let task = cx.spawn(|cx| async move {
                let (servers_tx, fake_adapter) = language.fake_adapter.as_ref().unwrap();
                let (server, mut fake_server) = lsp::FakeLanguageServer::new(
                    fake_adapter.name.to_string(),
                    fake_adapter.capabilities.clone(),
                    cx.clone(),
                );

                if let Some(initializer) = &fake_adapter.initializer {
                    initializer(&mut fake_server);
                }

                let servers_tx = servers_tx.clone();
                cx.background_executor()
                    .spawn(async move {
                        if fake_server
                            .try_receive_notification::<lsp::notification::Initialized>()
                            .await
                            .is_some()
                        {
                            servers_tx.unbounded_send(fake_server).ok();
                        }
                    })
                    .detach();

                Ok(server)
            });

            return Some(PendingLanguageServer {
                server_id,
                task,
                container_dir: None,
            });
        }

        let download_dir = self
            .language_server_download_dir
            .clone()
            .ok_or_else(|| anyhow!("language server download directory has not been assigned before starting server"))
            .log_err()?;
        let this = self.clone();
        let language = language.clone();
        let container_dir: Arc<Path> = Arc::from(download_dir.join(adapter.name.0.as_ref()));
        let root_path = root_path.clone();
        let adapter = adapter.clone();
        let login_shell_env_loaded = self.login_shell_env_loaded.clone();
        let lsp_binary_statuses = self.lsp_binary_status_tx.clone();

        let task = {
            let container_dir = container_dir.clone();
            cx.spawn(move |mut cx| async move {
                login_shell_env_loaded.await;

                let entry = this
                    .lsp_binary_paths
                    .lock()
                    .entry(adapter.name.clone())
                    .or_insert_with(|| {
                        let adapter = adapter.clone();
                        let language = language.clone();
                        let delegate = delegate.clone();
                        cx.spawn(|cx| {
                            get_binary(
                                adapter,
                                language,
                                delegate,
                                container_dir,
                                lsp_binary_statuses,
                                cx,
                            )
                            .map_err(Arc::new)
                        })
                        .shared()
                    })
                    .clone();

                let binary = match entry.await {
                    Ok(binary) => binary,
                    Err(err) => anyhow::bail!("{err}"),
                };

                if let Some(task) = adapter.will_start_server(&delegate, &mut cx) {
                    task.await?;
                }

                lsp::LanguageServer::new(
                    stderr_capture,
                    server_id,
                    binary,
                    &root_path,
                    adapter.code_action_kinds(),
                    cx,
                )
            })
        };

        Some(PendingLanguageServer {
            server_id,
            task,
            container_dir: Some(container_dir),
        })
    }

    pub fn language_server_binary_statuses(
        &self,
    ) -> mpsc::UnboundedReceiver<(Arc<Language>, LanguageServerBinaryStatus)> {
        self.lsp_binary_status_tx.subscribe()
    }

    pub fn delete_server_container(
        &self,
        adapter: Arc<CachedLspAdapter>,
        cx: &mut AppContext,
    ) -> Task<()> {
        log::info!("deleting server container");

        let mut lock = self.lsp_binary_paths.lock();
        lock.remove(&adapter.name);

        let download_dir = self
            .language_server_download_dir
            .clone()
            .expect("language server download directory has not been assigned before deleting server container");

        cx.spawn(|_| async move {
            let container_dir = download_dir.join(adapter.name.0.as_ref());
            smol::fs::remove_dir_all(container_dir)
                .await
                .context("server container removal")
                .log_err();
        })
    }

    pub fn next_language_server_id(&self) -> LanguageServerId {
        self.state.write().next_language_server_id()
    }
}

impl LanguageRegistryState {
    fn next_language_server_id(&mut self) -> LanguageServerId {
        LanguageServerId(post_inc(&mut self.next_language_server_id))
    }

    fn add(&mut self, language: Arc<Language>) {
        if let Some(theme) = self.theme.as_ref() {
            language.set_theme(theme.syntax());
        }
        self.languages.push(language);
        self.version += 1;
        *self.subscription.0.borrow_mut() = ();
    }

    fn reload(&mut self) {
        self.languages.clear();
        self.version += 1;
        self.reload_count += 1;
        for language in &mut self.available_languages {
            language.loaded = false;
        }
        *self.subscription.0.borrow_mut() = ();
    }

    /// Mark the given language a having been loaded, so that the
    /// language registry won't try to load it again.
    fn mark_language_loaded(&mut self, id: AvailableLanguageId) {
        for language in &mut self.available_languages {
            if language.id == id {
                language.loaded = true;
                break;
            }
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::test()
    }
}

async fn get_binary(
    adapter: Arc<CachedLspAdapter>,
    language: Arc<Language>,
    delegate: Arc<dyn LspAdapterDelegate>,
    container_dir: Arc<Path>,
    statuses: LspBinaryStatusSender,
    mut cx: AsyncAppContext,
) -> Result<LanguageServerBinary> {
    if !container_dir.exists() {
        smol::fs::create_dir_all(&container_dir)
            .await
            .context("failed to create container directory")?;
    }

    if let Some(task) = adapter.will_fetch_server(&delegate, &mut cx) {
        task.await?;
    }

    let binary = fetch_latest_binary(
        adapter.clone(),
        language.clone(),
        delegate.as_ref(),
        &container_dir,
        statuses.clone(),
    )
    .await;

    if let Err(error) = binary.as_ref() {
        if let Some(binary) = adapter
            .cached_server_binary(container_dir.to_path_buf(), delegate.as_ref())
            .await
        {
            statuses.send(language.clone(), LanguageServerBinaryStatus::Cached);
            return Ok(binary);
        } else {
            statuses.send(
                language.clone(),
                LanguageServerBinaryStatus::Failed {
                    error: format!("{:?}", error),
                },
            );
        }
    }

    binary
}

async fn fetch_latest_binary(
    adapter: Arc<CachedLspAdapter>,
    language: Arc<Language>,
    delegate: &dyn LspAdapterDelegate,
    container_dir: &Path,
    lsp_binary_statuses_tx: LspBinaryStatusSender,
) -> Result<LanguageServerBinary> {
    let container_dir: Arc<Path> = container_dir.into();
    lsp_binary_statuses_tx.send(
        language.clone(),
        LanguageServerBinaryStatus::CheckingForUpdate,
    );

    let version_info = adapter.fetch_latest_server_version(delegate).await?;
    lsp_binary_statuses_tx.send(language.clone(), LanguageServerBinaryStatus::Downloading);

    let binary = adapter
        .fetch_server_binary(version_info, container_dir.to_path_buf(), delegate)
        .await?;
    lsp_binary_statuses_tx.send(language.clone(), LanguageServerBinaryStatus::Downloaded);

    Ok(binary)
}

impl Language {
    pub fn new(config: LanguageConfig, ts_language: Option<tree_sitter::Language>) -> Self {
        Self {
            config,
            grammar: ts_language.map(|ts_language| {
                Arc::new(Grammar {
                    id: NEXT_GRAMMAR_ID.fetch_add(1, SeqCst),
                    highlights_query: None,
                    brackets_config: None,
                    outline_config: None,
                    embedding_config: None,
                    indents_config: None,
                    injection_config: None,
                    override_config: None,
                    redactions_config: None,
                    error_query: Query::new(&ts_language, "(ERROR) @error").unwrap(),
                    ts_language,
                    highlight_map: Default::default(),
                })
            }),
            adapters: Vec::new(),

            #[cfg(any(test, feature = "test-support"))]
            fake_adapter: None,
        }
    }

    pub fn lsp_adapters(&self) -> &[Arc<CachedLspAdapter>] {
        &self.adapters
    }

    pub fn id(&self) -> Option<usize> {
        self.grammar.as_ref().map(|g| g.id)
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
        Ok(self)
    }

    pub fn with_highlights_query(mut self, source: &str) -> Result<Self> {
        let grammar = self.grammar_mut();
        grammar.highlights_query = Some(Query::new(&grammar.ts_language, source)?);
        Ok(self)
    }

    pub fn with_outline_query(mut self, source: &str) -> Result<Self> {
        let grammar = self.grammar_mut();
        let query = Query::new(&grammar.ts_language, source)?;
        let mut item_capture_ix = None;
        let mut name_capture_ix = None;
        let mut context_capture_ix = None;
        let mut extra_context_capture_ix = None;
        get_capture_indices(
            &query,
            &mut [
                ("item", &mut item_capture_ix),
                ("name", &mut name_capture_ix),
                ("context", &mut context_capture_ix),
                ("context.extra", &mut extra_context_capture_ix),
            ],
        );
        if let Some((item_capture_ix, name_capture_ix)) = item_capture_ix.zip(name_capture_ix) {
            grammar.outline_config = Some(OutlineConfig {
                query,
                item_capture_ix,
                name_capture_ix,
                context_capture_ix,
                extra_context_capture_ix,
            });
        }
        Ok(self)
    }

    pub fn with_embedding_query(mut self, source: &str) -> Result<Self> {
        let grammar = self.grammar_mut();
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
        let grammar = self.grammar_mut();
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
        if let Some((open_capture_ix, close_capture_ix)) = open_capture_ix.zip(close_capture_ix) {
            grammar.brackets_config = Some(BracketConfig {
                query,
                open_capture_ix,
                close_capture_ix,
            });
        }
        Ok(self)
    }

    pub fn with_indents_query(mut self, source: &str) -> Result<Self> {
        let grammar = self.grammar_mut();
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
        let grammar = self.grammar_mut();
        let query = Query::new(&grammar.ts_language, source)?;
        let mut language_capture_ix = None;
        let mut content_capture_ix = None;
        get_capture_indices(
            &query,
            &mut [
                ("language", &mut language_capture_ix),
                ("content", &mut content_capture_ix),
            ],
        );
        let patterns = (0..query.pattern_count())
            .map(|ix| {
                let mut config = InjectionPatternConfig::default();
                for setting in query.property_settings(ix) {
                    match setting.key.as_ref() {
                        "language" => {
                            config.language = setting.value.clone();
                        }
                        "combined" => {
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
        let query = Query::new(&self.grammar_mut().ts_language, source)?;

        let mut override_configs_by_id = HashMap::default();
        for (ix, name) in query.capture_names().iter().enumerate() {
            if !name.starts_with('_') {
                let value = self.config.overrides.remove(*name).unwrap_or_default();
                for server_name in &value.opt_into_language_servers {
                    if !self
                        .config
                        .scope_opt_in_language_servers
                        .contains(server_name)
                    {
                        util::debug_panic!("Server {server_name:?} has been opted-in by scope {name:?} but has not been marked as an opt-in server");
                    }
                }

                override_configs_by_id.insert(ix as u32, (name.to_string(), value));
            }
        }

        if !self.config.overrides.is_empty() {
            let keys = self.config.overrides.keys().collect::<Vec<_>>();
            Err(anyhow!(
                "language {:?} has overrides in config not in query: {keys:?}",
                self.config.name
            ))?;
        }

        for disabled_scope_name in self
            .config
            .brackets
            .disabled_scopes_by_bracket_ix
            .iter()
            .flatten()
        {
            if !override_configs_by_id
                .values()
                .any(|(scope_name, _)| scope_name == disabled_scope_name)
            {
                Err(anyhow!(
                    "language {:?} has overrides in config not in query: {disabled_scope_name:?}",
                    self.config.name
                ))?;
            }
        }

        for (name, override_config) in override_configs_by_id.values_mut() {
            override_config.disabled_bracket_ixs = self
                .config
                .brackets
                .disabled_scopes_by_bracket_ix
                .iter()
                .enumerate()
                .filter_map(|(ix, disabled_scope_names)| {
                    if disabled_scope_names.contains(name) {
                        Some(ix as u16)
                    } else {
                        None
                    }
                })
                .collect();
        }

        self.config.brackets.disabled_scopes_by_bracket_ix.clear();
        self.grammar_mut().override_config = Some(OverrideConfig {
            query,
            values: override_configs_by_id,
        });
        Ok(self)
    }

    pub fn with_redaction_query(mut self, source: &str) -> anyhow::Result<Self> {
        let grammar = self.grammar_mut();
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

    fn grammar_mut(&mut self) -> &mut Grammar {
        Arc::get_mut(self.grammar.as_mut().unwrap()).unwrap()
    }

    pub async fn with_lsp_adapters(mut self, lsp_adapters: Vec<Arc<dyn LspAdapter>>) -> Self {
        for adapter in lsp_adapters {
            self.adapters.push(CachedLspAdapter::new(adapter).await);
        }
        self
    }

    #[cfg(any(test, feature = "test-support"))]
    pub async fn set_fake_lsp_adapter(
        &mut self,
        fake_lsp_adapter: Arc<FakeLspAdapter>,
    ) -> mpsc::UnboundedReceiver<lsp::FakeLanguageServer> {
        let (servers_tx, servers_rx) = mpsc::unbounded();
        self.fake_adapter = Some((servers_tx, fake_lsp_adapter.clone()));
        let adapter = CachedLspAdapter::new(Arc::new(fake_lsp_adapter)).await;
        self.adapters = vec![adapter];
        servers_rx
    }

    pub fn name(&self) -> Arc<str> {
        self.config.name.clone()
    }

    pub async fn disk_based_diagnostic_sources(&self) -> &[String] {
        match self.adapters.first().as_ref() {
            Some(adapter) => &adapter.disk_based_diagnostic_sources,
            None => &[],
        }
    }

    pub async fn disk_based_diagnostics_progress_token(&self) -> Option<&str> {
        for adapter in &self.adapters {
            let token = adapter.disk_based_diagnostics_progress_token.as_deref();
            if token.is_some() {
                return token;
            }
        }

        None
    }

    pub async fn process_completion(self: &Arc<Self>, completion: &mut lsp::CompletionItem) {
        for adapter in &self.adapters {
            adapter.process_completion(completion).await;
        }
    }

    pub async fn label_for_completion(
        self: &Arc<Self>,
        completion: &lsp::CompletionItem,
    ) -> Option<CodeLabel> {
        self.adapters
            .first()
            .as_ref()?
            .label_for_completion(completion, self)
            .await
    }

    pub async fn label_for_symbol(
        self: &Arc<Self>,
        name: &str,
        kind: lsp::SymbolKind,
    ) -> Option<CodeLabel> {
        self.adapters
            .first()
            .as_ref()?
            .label_for_symbol(name, kind, self)
            .await
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
            for chunk in BufferChunks::new(text, range, Some((captures, highlight_maps)), vec![]) {
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
        &self.config.path_suffixes
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

    pub fn prettier_parser_name(&self) -> Option<&str> {
        self.config.prettier_parser_name.as_deref()
    }
}

impl LanguageScope {
    pub fn collapsed_placeholder(&self) -> &str {
        self.language.config.collapsed_placeholder.as_ref()
    }

    /// Returns line prefix that is inserted in e.g. line continuations or
    /// in `toggle comments` action.
    pub fn line_comment_prefixes(&self) -> Option<&Vec<Arc<str>>> {
        Override::as_option(
            self.config_override().map(|o| &o.line_comments),
            Some(&self.language.config.line_comments),
        )
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
        if opt_in_servers.iter().any(|o| *o == *name.0) {
            if let Some(over) = self.config_override() {
                over.opt_into_language_servers.iter().any(|o| *o == *name.0)
            } else {
                false
            }
        } else {
            true
        }
    }

    fn config_override(&self) -> Option<&LanguageConfigOverride> {
        let id = self.override_id?;
        let grammar = self.language.grammar.as_ref()?;
        let override_config = grammar.override_config.as_ref()?;
        override_config.values.get(&id).map(|e| &e.1)
    }
}

impl Hash for Language {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.id().hash(state)
    }
}

impl PartialEq for Language {
    fn eq(&self, other: &Self) -> bool {
        self.id().eq(&other.id())
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
    pub fn id(&self) -> usize {
        self.id
    }

    fn parse_text(&self, text: &Rope, old_tree: Option<Tree>) -> Tree {
        PARSER.with(|parser| {
            let mut parser = parser.borrow_mut();
            parser
                .set_language(&self.ts_language)
                .expect("incompatible grammar");
            let mut chunks = text.chunks_in_range(0..text.len());
            parser
                .parse_with(
                    &mut move |offset, _| {
                        chunks.seek(offset);
                        chunks.next().unwrap_or("").as_bytes()
                    },
                    old_tree.as_ref(),
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
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
#[async_trait]
impl LspAdapter for Arc<FakeLspAdapter> {
    fn name(&self) -> LanguageServerName {
        LanguageServerName(self.name.into())
    }

    fn short_name(&self) -> &'static str {
        "FakeLspAdapter"
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

    async fn installation_test_binary(&self, _: PathBuf) -> Option<LanguageServerBinary> {
        unreachable!();
    }

    fn process_diagnostics(&self, _: &mut lsp::PublishDiagnosticsParams) {}

    fn disk_based_diagnostic_sources(&self) -> Vec<String> {
        self.disk_based_diagnostics_sources.clone()
    }

    fn disk_based_diagnostics_progress_token(&self) -> Option<String> {
        self.disk_based_diagnostics_progress_token.clone()
    }

    fn initialization_options(&self) -> Option<Value> {
        self.initialization_options.clone()
    }

    fn prettier_plugins(&self) -> &[&'static str] {
        &self.prettier_plugins
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

pub fn range_to_lsp(range: Range<PointUtf16>) -> lsp::Range {
    lsp::Range {
        start: point_to_lsp(range.start),
        end: point_to_lsp(range.end),
    }
}

pub fn range_from_lsp(range: lsp::Range) -> Range<Unclipped<PointUtf16>> {
    let mut start = point_from_lsp(range.start);
    let mut end = point_from_lsp(range.end);
    if start > end {
        mem::swap(&mut start, &mut end);
    }
    start..end
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;

    #[gpui::test(iterations = 10)]
    async fn test_first_line_pattern(cx: &mut TestAppContext) {
        let mut languages = LanguageRegistry::test();

        languages.set_executor(cx.executor());
        let languages = Arc::new(languages);
        languages.register(
            "/javascript",
            LanguageConfig {
                name: "JavaScript".into(),
                path_suffixes: vec!["js".into()],
                first_line_pattern: Some(Regex::new(r"\bnode\b").unwrap()),
                ..Default::default()
            },
            tree_sitter_typescript::language_tsx(),
            vec![],
            |_| Default::default(),
        );

        languages
            .language_for_file("the/script", None)
            .await
            .unwrap_err();
        languages
            .language_for_file("the/script", Some(&"nothing".into()))
            .await
            .unwrap_err();
        assert_eq!(
            languages
                .language_for_file("the/script", Some(&"#!/bin/env node".into()))
                .await
                .unwrap()
                .name()
                .as_ref(),
            "JavaScript"
        );
    }

    #[gpui::test(iterations = 10)]
    async fn test_language_loading(cx: &mut TestAppContext) {
        let mut languages = LanguageRegistry::test();
        languages.set_executor(cx.executor());
        let languages = Arc::new(languages);
        languages.register(
            "/JSON",
            LanguageConfig {
                name: "JSON".into(),
                path_suffixes: vec!["json".into()],
                ..Default::default()
            },
            tree_sitter_json::language(),
            vec![],
            |_| Default::default(),
        );
        languages.register(
            "/rust",
            LanguageConfig {
                name: "Rust".into(),
                path_suffixes: vec!["rs".into()],
                ..Default::default()
            },
            tree_sitter_rust::language(),
            vec![],
            |_| Default::default(),
        );
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
}
