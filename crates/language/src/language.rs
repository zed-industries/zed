//! The `language` crate provides a large chunk of Zed's language-related
//! features (the other big contributors being project and lsp crates that revolve around LSP features).
//! Namely, this crate:
//! - Provides [`Language`], [`Grammar`] and [`LanguageRegistry`] types that
//! use Tree-sitter to provide syntax highlighting to the editor; note though that `language` doesn't perform the highlighting by itself. It only maps ranges in a buffer to colors. Treesitter is also used for buffer outlines (lists of symbols in a buffer)
//! - Exposes [`LanguageConfig`] that describes how constructs (like brackets or line comments) should be handled by the editor for a source file of a particular language.
//!
//! Notably we do *not* assign a single language to a single file; in real world a single file can consist of multiple programming languages - HTML is a good example of that - and `language` crate tends to reflect that status quo in its API.
mod buffer;
mod diagnostic_set;
mod highlight_map;
mod language_registry;
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
use futures::Future;
use gpui::{AppContext, AsyncAppContext, Model, Task};
pub use highlight_map::HighlightMap;
use lazy_static::lazy_static;
use lsp::{CodeActionKind, LanguageServerBinary};
use parking_lot::Mutex;
use regex::Regex;
use schemars::{
    gen::SchemaGenerator,
    schema::{InstanceType, Schema, SchemaObject},
    JsonSchema,
};
use serde::{de, Deserialize, Deserializer, Serialize, Serializer};
use serde_json::Value;
use smol::future::FutureExt as _;
use std::{
    any::Any,
    cell::RefCell,
    ffi::OsString,
    fmt::Debug,
    hash::Hash,
    mem,
    ops::Range,
    path::{Path, PathBuf},
    pin::Pin,
    str,
    sync::{
        atomic::{AtomicU64, AtomicUsize, Ordering::SeqCst},
        Arc,
    },
};
use syntax_map::SyntaxSnapshot;
use theme::SyntaxTheme;
use tree_sitter::{self, wasmtime, Query, WasmStore};
use util::http::HttpClient;

pub use buffer::Operation;
pub use buffer::*;
pub use diagnostic_set::DiagnosticEntry;
pub use language_registry::{
    LanguageQueries, LanguageRegistry, LanguageServerBinaryStatus, PendingLanguageServer,
    QUERY_FILENAME_PREFIXES,
};
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

thread_local! {
    static PARSER: RefCell<Parser> = {
        let mut parser = Parser::new();
        parser.set_wasm_store(WasmStore::new(WASM_ENGINE.clone()).unwrap()).unwrap();
        RefCell::new(parser)
    };
}

lazy_static! {
    static ref NEXT_LANGUAGE_ID: AtomicUsize = Default::default();
    static ref NEXT_GRAMMAR_ID: AtomicUsize = Default::default();
    static ref WASM_ENGINE: wasmtime::Engine = {
        wasmtime::Engine::new(&wasmtime::Config::new()).unwrap()
    };

    /// A shared grammar for plain text, exposed for reuse by downstream crates.
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
#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Deserialize, Serialize)]
pub struct LanguageServerName(pub Arc<str>);

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Location {
    pub buffer: Model<Buffer>,
    pub range: Range<Anchor>,
}

pub struct LanguageContext {
    pub package: Option<String>,
    pub symbol: Option<String>,
}

pub trait LanguageContextProvider: Send + Sync {
    fn build_context(&self, location: Location, cx: &mut AppContext) -> Result<LanguageContext>;
}

/// A context provider that fills out LanguageContext without inspecting the contents.
pub struct DefaultContextProvider;

impl LanguageContextProvider for DefaultContextProvider {
    fn build_context(
        &self,
        location: Location,
        cx: &mut AppContext,
    ) -> gpui::Result<LanguageContext> {
        let symbols = location
            .buffer
            .read(cx)
            .snapshot()
            .symbols_containing(location.range.start, None);
        let symbol = symbols.and_then(|symbols| {
            symbols.last().map(|symbol| {
                let range = symbol
                    .name_ranges
                    .last()
                    .cloned()
                    .unwrap_or(0..symbol.text.len());
                symbol.text[range].to_string()
            })
        });
        Ok(LanguageContext {
            package: None,
            symbol,
        })
    }
}

/// Represents a Language Server, with certain cached sync properties.
/// Uses [`LspAdapter`] under the hood, but calls all 'static' methods
/// once at startup, and caches the results.
pub struct CachedLspAdapter {
    pub name: LanguageServerName,
    pub disk_based_diagnostic_sources: Vec<String>,
    pub disk_based_diagnostics_progress_token: Option<String>,
    pub language_ids: HashMap<String, String>,
    pub adapter: Arc<dyn LspAdapter>,
    pub reinstall_attempt_count: AtomicU64,
    cached_binary: futures::lock::Mutex<Option<LanguageServerBinary>>,
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
        })
    }

    pub async fn get_language_server_command(
        self: Arc<Self>,
        language: Arc<Language>,
        container_dir: Arc<Path>,
        delegate: Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Result<LanguageServerBinary> {
        let cached_binary = self.cached_binary.lock().await;
        self.adapter
            .clone()
            .get_language_server_command(language, container_dir, delegate, cached_binary, cx)
            .await
    }

    pub fn will_start_server(
        &self,
        delegate: &Arc<dyn LspAdapterDelegate>,
        cx: &mut AsyncAppContext,
    ) -> Option<Task<Result<()>>> {
        self.adapter.will_start_server(delegate, cx)
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

    #[cfg(any(test, feature = "test-support"))]
    fn as_fake(&self) -> Option<&FakeLspAdapter> {
        self.adapter.as_fake()
    }
}

/// [`LspAdapterDelegate`] allows [`LspAdapter]` implementations to interface with the application
// e.g. to display a notification or fetch data from the web.
#[async_trait]
pub trait LspAdapterDelegate: Send + Sync {
    fn show_notification(&self, message: &str, cx: &mut AppContext);
    fn http_client(&self) -> Arc<dyn HttpClient>;
    fn update_status(&self, language: LanguageServerName, status: LanguageServerBinaryStatus);

    async fn which_command(&self, command: OsString) -> Option<(PathBuf, HashMap<String, String>)>;
    async fn read_text_file(&self, path: PathBuf) -> Result<String>;
}

#[async_trait]
pub trait LspAdapter: 'static + Send + Sync {
    fn name(&self) -> LanguageServerName;

    fn get_language_server_command<'a>(
        self: Arc<Self>,
        language: Arc<Language>,
        container_dir: Arc<Path>,
        delegate: Arc<dyn LspAdapterDelegate>,
        mut cached_binary: futures::lock::MutexGuard<'a, Option<LanguageServerBinary>>,
        cx: &'a mut AsyncAppContext,
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
            if let Some(binary) = self.check_if_user_installed(delegate.as_ref()).await {
                log::info!(
                    "found user-installed language server for {}. path: {:?}, arguments: {:?}",
                    language.name(),
                    binary.path,
                    binary.arguments
                );
                return Ok(binary);
            }

            if let Some(cached_binary) = cached_binary.as_ref() {
                return Ok(cached_binary.clone());
            }

            if !container_dir.exists() {
                smol::fs::create_dir_all(&container_dir)
                    .await
                    .context("failed to create container directory")?;
            }

            if let Some(task) = self.will_fetch_server(&delegate, cx) {
                task.await?;
            }

            let name = self.name();
            log::info!("fetching latest version of language server {:?}", name.0);
            delegate.update_status(
                name.clone(),
                LanguageServerBinaryStatus::CheckingForUpdate,
            );
            let version_info = self.fetch_latest_server_version(delegate.as_ref()).await?;

            log::info!("downloading language server {:?}", name.0);
            delegate.update_status(self.name(), LanguageServerBinaryStatus::Downloading);
            let mut binary = self
                .fetch_server_binary(version_info, container_dir.to_path_buf(), delegate.as_ref())
                .await;

            delegate.update_status(name.clone(), LanguageServerBinaryStatus::Downloaded);

            if let Err(error) = binary.as_ref() {
                if let Some(prev_downloaded_binary) = self
                    .cached_server_binary(container_dir.to_path_buf(), delegate.as_ref())
                    .await
                {
                    delegate.update_status(name.clone(), LanguageServerBinaryStatus::Cached);
                    log::info!(
                        "failed to fetch newest version of language server {:?}. falling back to using {:?}",
                        name.clone(),
                        prev_downloaded_binary.path.display()
                    );
                    binary = Ok(prev_downloaded_binary);
                } else {
                    delegate.update_status(
                        name.clone(),
                        LanguageServerBinaryStatus::Failed {
                            error: format!("{:?}", error),
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

    #[cfg(any(test, feature = "test-support"))]
    fn as_fake(&self) -> Option<&FakeLspAdapter> {
        None
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

#[derive(Clone, Deserialize, JsonSchema)]
pub struct LanguageConfig {
    /// Human-readable name of the language.
    pub name: Arc<str>,
    // The name of the grammar in a WASM bundle (experimental).
    pub grammar: Option<Arc<str>>,
    /// The criteria for matching this language to a given file.
    #[serde(flatten)]
    pub matcher: LanguageMatcher,
    /// List of bracket types in a language.
    #[serde(default)]
    #[schemars(schema_with = "bracket_pair_config_json_schema")]
    pub brackets: BracketPairConfig,
    /// If set to true, auto indentation uses last non empty line to determine
    /// the indentation level for a new line.
    #[serde(default = "auto_indent_using_last_non_empty_line_default")]
    pub auto_indent_using_last_non_empty_line: bool,
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
    #[serde(skip_deserializing)]
    #[schemars(skip)]
    pub disabled_bracket_ixs: Vec<u16>,
    #[serde(default)]
    pub word_characters: Override<HashSet<char>>,
    #[serde(default)]
    pub opt_into_language_servers: Vec<String>,
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
            name: "".into(),
            grammar: None,
            matcher: LanguageMatcher::default(),
            brackets: Default::default(),
            auto_indent_using_last_non_empty_line: auto_indent_using_last_non_empty_line_default(),
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
    pub capabilities: lsp::ServerCapabilities,
    pub initializer: Option<Box<dyn 'static + Send + Sync + Fn(&mut lsp::FakeLanguageServer)>>,
    pub disk_based_diagnostics_progress_token: Option<String>,
    pub disk_based_diagnostics_sources: Vec<String>,
    pub prettier_plugins: Vec<&'static str>,
    pub language_server_binary: LanguageServerBinary,
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
    #[schemars(skip)]
    pub disabled_scopes_by_bracket_ix: Vec<Vec<String>>,
}

fn bracket_pair_config_json_schema(gen: &mut SchemaGenerator) -> Schema {
    Option::<Vec<BracketPairContent>>::json_schema(gen)
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
    /// True if an extra newline should be inserted while the cursor is in the middle
    /// of that bracket pair.
    pub newline: bool,
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
pub(crate) struct LanguageId(usize);

impl LanguageId {
    pub(crate) fn new() -> Self {
        Self(NEXT_LANGUAGE_ID.fetch_add(1, SeqCst))
    }
}

pub struct Language {
    pub(crate) id: LanguageId,
    pub(crate) config: LanguageConfig,
    pub(crate) grammar: Option<Arc<Grammar>>,
    pub(crate) context_provider: Option<Arc<dyn LanguageContextProvider>>,
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

impl Language {
    pub fn new(config: LanguageConfig, ts_language: Option<tree_sitter::Language>) -> Self {
        Self::new_with_id(
            LanguageId(NEXT_LANGUAGE_ID.fetch_add(1, SeqCst)),
            config,
            ts_language,
        )
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
            context_provider: None,
        }
    }

    pub fn with_context_provider(
        mut self,
        provider: Option<Arc<dyn LanguageContextProvider>>,
    ) -> Self {
        self.context_provider = provider;
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

    pub fn name(&self) -> Arc<str> {
        self.config.name.clone()
    }

    pub fn context_provider(&self) -> Option<Arc<dyn LanguageContextProvider>> {
        self.context_provider.clone()
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
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
#[async_trait]
impl LspAdapter for FakeLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName(self.name.into())
    }

    fn get_language_server_command<'a>(
        self: Arc<Self>,
        _: Arc<Language>,
        _: Arc<Path>,
        _: Arc<dyn LspAdapterDelegate>,
        _: futures::lock::MutexGuard<'a, Option<LanguageServerBinary>>,
        _: &'a mut AsyncAppContext,
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

    fn as_fake(&self) -> Option<&FakeLspAdapter> {
        Some(self)
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
        languages.register_test_language(LanguageConfig {
            name: "JavaScript".into(),
            matcher: LanguageMatcher {
                path_suffixes: vec!["js".into()],
                first_line_pattern: Some(Regex::new(r"\bnode\b").unwrap()),
            },
            ..Default::default()
        });

        languages
            .language_for_file("the/script".as_ref(), None)
            .await
            .unwrap_err();
        languages
            .language_for_file("the/script".as_ref(), Some(&"nothing".into()))
            .await
            .unwrap_err();
        assert_eq!(
            languages
                .language_for_file("the/script".as_ref(), Some(&"#!/bin/env node".into()))
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
        languages.register_native_grammars([
            ("json", tree_sitter_json::language()),
            ("rust", tree_sitter_rust::language()),
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
}
