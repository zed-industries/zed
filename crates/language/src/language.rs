mod buffer;
mod diagnostic_set;
mod highlight_map;
mod outline;
pub mod proto;
#[cfg(test)]
mod tests;

use anyhow::{anyhow, Context, Result};
use client::http::HttpClient;
use collections::HashMap;
use futures::{
    future::{BoxFuture, Shared},
    FutureExt, TryFutureExt,
};
use gpui::{MutableAppContext, Task};
use highlight_map::HighlightMap;
use lazy_static::lazy_static;
use parking_lot::{Mutex, RwLock};
use regex::Regex;
use serde::{de, Deserialize, Deserializer};
use serde_json::Value;
use std::{
    any::Any,
    cell::RefCell,
    mem,
    ops::Range,
    path::{Path, PathBuf},
    str,
    sync::Arc,
};
use theme::SyntaxTheme;
use tree_sitter::{self, Query};
use util::ResultExt;

#[cfg(any(test, feature = "test-support"))]
use futures::channel::mpsc;

pub use buffer::Operation;
pub use buffer::*;
pub use diagnostic_set::DiagnosticEntry;
pub use outline::{Outline, OutlineItem};
pub use tree_sitter::{Parser, Tree};

thread_local! {
    static PARSER: RefCell<Parser>  = RefCell::new(Parser::new());
}

lazy_static! {
    pub static ref PLAIN_TEXT: Arc<Language> = Arc::new(Language::new(
        LanguageConfig {
            name: "Plain Text".into(),
            ..Default::default()
        },
        None,
    ));
}

pub trait ToLspPosition {
    fn to_lsp_position(self) -> lsp::Position;
}

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct LanguageServerName(pub Arc<str>);

use async_trait::async_trait;

#[async_trait]
pub trait LspAdapter: 'static + Send + Sync {
    async fn name(&self) -> LanguageServerName;
    async fn fetch_latest_server_version(
        &self,
        http: Arc<dyn HttpClient>,
    ) -> Result<Box<dyn 'static + Send + Any>>;
    async fn fetch_server_binary(
        &self,
        version: Box<dyn 'static + Send + Any>,
        http: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> Result<PathBuf>;
    async fn cached_server_binary(&self, container_dir: PathBuf) -> Option<PathBuf>;

    async fn process_diagnostics(&self, _: &mut lsp::PublishDiagnosticsParams) {}

    async fn label_for_completion(
        &self,
        _: &lsp::CompletionItem,
        _: &Language,
    ) -> Option<CodeLabel> {
        None
    }

    async fn label_for_symbol(
        &self,
        _: &str,
        _: lsp::SymbolKind,
        _: &Language,
    ) -> Option<CodeLabel> {
        None
    }

    async fn server_args(&self) -> Vec<String> {
        Vec::new()
    }

    async fn initialization_options(&self) -> Option<Value> {
        None
    }

    async fn disk_based_diagnostic_sources(&self) -> Vec<String> {
        Default::default()
    }

    async fn disk_based_diagnostics_progress_token(&self) -> Option<String> {
        None
    }

    async fn id_for_language(&self, _name: &str) -> Option<String> {
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CodeLabel {
    pub text: String,
    pub runs: Vec<(Range<usize>, HighlightId)>,
    pub filter_range: Range<usize>,
}

#[derive(Deserialize)]
pub struct LanguageConfig {
    pub name: Arc<str>,
    pub path_suffixes: Vec<String>,
    pub brackets: Vec<BracketPair>,
    #[serde(default = "auto_indent_using_last_non_empty_line_default")]
    pub auto_indent_using_last_non_empty_line: bool,
    #[serde(default, deserialize_with = "deserialize_regex")]
    pub increase_indent_pattern: Option<Regex>,
    #[serde(default, deserialize_with = "deserialize_regex")]
    pub decrease_indent_pattern: Option<Regex>,
    #[serde(default)]
    pub autoclose_before: String,
    pub line_comment: Option<String>,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            name: "".into(),
            path_suffixes: Default::default(),
            brackets: Default::default(),
            auto_indent_using_last_non_empty_line: auto_indent_using_last_non_empty_line_default(),
            increase_indent_pattern: Default::default(),
            decrease_indent_pattern: Default::default(),
            autoclose_before: Default::default(),
            line_comment: Default::default(),
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

#[cfg(any(test, feature = "test-support"))]
pub struct FakeLspAdapter {
    pub name: &'static str,
    pub capabilities: lsp::ServerCapabilities,
    pub initializer: Option<Box<dyn 'static + Send + Sync + Fn(&mut lsp::FakeLanguageServer)>>,
    pub disk_based_diagnostics_progress_token: Option<String>,
    pub disk_based_diagnostics_sources: Vec<String>,
}

#[derive(Clone, Debug, Deserialize)]
pub struct BracketPair {
    pub start: String,
    pub end: String,
    pub close: bool,
    pub newline: bool,
}

pub struct Language {
    pub(crate) config: LanguageConfig,
    pub(crate) grammar: Option<Arc<Grammar>>,
    pub(crate) adapter: Option<Arc<dyn LspAdapter>>,

    #[cfg(any(test, feature = "test-support"))]
    fake_adapter: Option<(
        mpsc::UnboundedSender<lsp::FakeLanguageServer>,
        Arc<FakeLspAdapter>,
    )>,
}

pub struct Grammar {
    pub(crate) ts_language: tree_sitter::Language,
    pub(crate) highlights_query: Option<Query>,
    pub(crate) brackets_query: Option<Query>,
    pub(crate) indents_query: Option<Query>,
    pub(crate) outline_query: Option<Query>,
    pub(crate) highlight_map: Mutex<HighlightMap>,
}

#[derive(Clone)]
pub enum LanguageServerBinaryStatus {
    CheckingForUpdate,
    Downloading,
    Downloaded,
    Cached,
    Failed { error: String },
}

pub struct LanguageRegistry {
    languages: RwLock<Vec<Arc<Language>>>,
    language_server_download_dir: Option<Arc<Path>>,
    lsp_binary_statuses_tx: async_broadcast::Sender<(Arc<Language>, LanguageServerBinaryStatus)>,
    lsp_binary_statuses_rx: async_broadcast::Receiver<(Arc<Language>, LanguageServerBinaryStatus)>,
    login_shell_env_loaded: Shared<Task<()>>,
    lsp_binary_paths: Mutex<
        HashMap<
            LanguageServerName,
            Shared<BoxFuture<'static, Result<PathBuf, Arc<anyhow::Error>>>>,
        >,
    >,
}

impl LanguageRegistry {
    pub fn new(login_shell_env_loaded: Task<()>) -> Self {
        let (lsp_binary_statuses_tx, lsp_binary_statuses_rx) = async_broadcast::broadcast(16);
        Self {
            language_server_download_dir: None,
            languages: Default::default(),
            lsp_binary_statuses_tx,
            lsp_binary_statuses_rx,
            login_shell_env_loaded: login_shell_env_loaded.shared(),
            lsp_binary_paths: Default::default(),
        }
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn test() -> Self {
        Self::new(Task::ready(()))
    }

    pub fn add(&self, language: Arc<Language>) {
        self.languages.write().push(language.clone());
    }

    pub fn set_theme(&self, theme: &SyntaxTheme) {
        for language in self.languages.read().iter() {
            language.set_theme(theme);
        }
    }

    pub fn set_language_server_download_dir(&mut self, path: impl Into<Arc<Path>>) {
        self.language_server_download_dir = Some(path.into());
    }

    pub fn get_language(&self, name: &str) -> Option<Arc<Language>> {
        self.languages
            .read()
            .iter()
            .find(|language| language.name().to_lowercase() == name.to_lowercase())
            .cloned()
    }

    pub fn to_vec(&self) -> Vec<Arc<Language>> {
        self.languages.read().iter().cloned().collect()
    }

    pub fn language_names(&self) -> Vec<String> {
        self.languages
            .read()
            .iter()
            .map(|language| language.name().to_string())
            .collect()
    }

    pub fn select_language(&self, path: impl AsRef<Path>) -> Option<Arc<Language>> {
        let path = path.as_ref();
        let filename = path.file_name().and_then(|name| name.to_str());
        let extension = path.extension().and_then(|name| name.to_str());
        let path_suffixes = [extension, filename];
        self.languages
            .read()
            .iter()
            .find(|language| {
                language
                    .config
                    .path_suffixes
                    .iter()
                    .any(|suffix| dbg!(path_suffixes.contains(&Some(dbg!(suffix.as_str())))))
            })
            .cloned()
    }

    pub fn start_language_server(
        self: &Arc<Self>,
        server_id: usize,
        language: Arc<Language>,
        root_path: Arc<Path>,
        http_client: Arc<dyn HttpClient>,
        cx: &mut MutableAppContext,
    ) -> Option<Task<Result<lsp::LanguageServer>>> {
        #[cfg(any(test, feature = "test-support"))]
        if language.fake_adapter.is_some() {
            let language = language.clone();
            return Some(cx.spawn(|cx| async move {
                let (servers_tx, fake_adapter) = language.fake_adapter.as_ref().unwrap();
                let (server, mut fake_server) = lsp::LanguageServer::fake(
                    fake_adapter.name.to_string(),
                    fake_adapter.capabilities.clone(),
                    cx.clone(),
                );

                if let Some(initializer) = &fake_adapter.initializer {
                    initializer(&mut fake_server);
                }

                let servers_tx = servers_tx.clone();
                cx.background()
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
            }));
        }

        let download_dir = self
            .language_server_download_dir
            .clone()
            .ok_or_else(|| anyhow!("language server download directory has not been assigned"))
            .log_err()?;

        let this = self.clone();
        let adapter = language.adapter.clone()?;
        let lsp_binary_statuses = self.lsp_binary_statuses_tx.clone();
        let login_shell_env_loaded = self.login_shell_env_loaded.clone();
        Some(cx.spawn(|cx| async move {
            login_shell_env_loaded.await;
            let server_binary_path = this
                .lsp_binary_paths
                .lock()
                .entry(adapter.name().await)
                .or_insert_with(|| {
                    get_server_binary_path(
                        adapter.clone(),
                        language.clone(),
                        http_client,
                        download_dir,
                        lsp_binary_statuses,
                    )
                    .map_err(Arc::new)
                    .boxed()
                    .shared()
                })
                .clone()
                .map_err(|e| anyhow!(e));

            let server_binary_path = server_binary_path.await?;
            let server_args = adapter.server_args().await;
            let server = lsp::LanguageServer::new(
                server_id,
                &server_binary_path,
                &server_args,
                &root_path,
                cx,
            )?;
            Ok(server)
        }))
    }

    pub fn language_server_binary_statuses(
        &self,
    ) -> async_broadcast::Receiver<(Arc<Language>, LanguageServerBinaryStatus)> {
        self.lsp_binary_statuses_rx.clone()
    }
}

async fn get_server_binary_path(
    adapter: Arc<dyn LspAdapter>,
    language: Arc<Language>,
    http_client: Arc<dyn HttpClient>,
    download_dir: Arc<Path>,
    statuses: async_broadcast::Sender<(Arc<Language>, LanguageServerBinaryStatus)>,
) -> Result<PathBuf> {
    let container_dir = download_dir.join(adapter.name().await.0.as_ref());
    if !container_dir.exists() {
        smol::fs::create_dir_all(&container_dir)
            .await
            .context("failed to create container directory")?;
    }

    let path = fetch_latest_server_binary_path(
        adapter.clone(),
        language.clone(),
        http_client,
        &container_dir,
        statuses.clone(),
    )
    .await;
    if let Err(error) = path.as_ref() {
        if let Some(cached_path) = adapter.cached_server_binary(container_dir).await {
            statuses
                .broadcast((language.clone(), LanguageServerBinaryStatus::Cached))
                .await?;
            return Ok(cached_path);
        } else {
            statuses
                .broadcast((
                    language.clone(),
                    LanguageServerBinaryStatus::Failed {
                        error: format!("{:?}", error),
                    },
                ))
                .await?;
        }
    }
    path
}

async fn fetch_latest_server_binary_path(
    adapter: Arc<dyn LspAdapter>,
    language: Arc<Language>,
    http_client: Arc<dyn HttpClient>,
    container_dir: &Path,
    lsp_binary_statuses_tx: async_broadcast::Sender<(Arc<Language>, LanguageServerBinaryStatus)>,
) -> Result<PathBuf> {
    let container_dir: Arc<Path> = container_dir.into();
    lsp_binary_statuses_tx
        .broadcast((
            language.clone(),
            LanguageServerBinaryStatus::CheckingForUpdate,
        ))
        .await?;
    let version_info = adapter
        .fetch_latest_server_version(http_client.clone())
        .await?;
    lsp_binary_statuses_tx
        .broadcast((language.clone(), LanguageServerBinaryStatus::Downloading))
        .await?;
    let path = adapter
        .fetch_server_binary(version_info, http_client, container_dir.clone())
        .await?;
    lsp_binary_statuses_tx
        .broadcast((language.clone(), LanguageServerBinaryStatus::Downloaded))
        .await?;
    Ok(path)
}

impl Language {
    pub fn new(config: LanguageConfig, ts_language: Option<tree_sitter::Language>) -> Self {
        Self {
            config,
            grammar: ts_language.map(|ts_language| {
                Arc::new(Grammar {
                    highlights_query: None,
                    brackets_query: None,
                    indents_query: None,
                    outline_query: None,
                    ts_language,
                    highlight_map: Default::default(),
                })
            }),
            adapter: None,

            #[cfg(any(test, feature = "test-support"))]
            fake_adapter: None,
        }
    }

    pub fn lsp_adapter(&self) -> Option<Arc<dyn LspAdapter>> {
        self.adapter.clone()
    }

    pub fn with_highlights_query(mut self, source: &str) -> Result<Self> {
        let grammar = self.grammar_mut();
        grammar.highlights_query = Some(Query::new(grammar.ts_language, source)?);
        Ok(self)
    }

    pub fn with_brackets_query(mut self, source: &str) -> Result<Self> {
        let grammar = self.grammar_mut();
        grammar.brackets_query = Some(Query::new(grammar.ts_language, source)?);
        Ok(self)
    }

    pub fn with_indents_query(mut self, source: &str) -> Result<Self> {
        let grammar = self.grammar_mut();
        grammar.indents_query = Some(Query::new(grammar.ts_language, source)?);
        Ok(self)
    }

    pub fn with_outline_query(mut self, source: &str) -> Result<Self> {
        let grammar = self.grammar_mut();
        grammar.outline_query = Some(Query::new(grammar.ts_language, source)?);
        Ok(self)
    }

    fn grammar_mut(&mut self) -> &mut Grammar {
        Arc::get_mut(self.grammar.as_mut().unwrap()).unwrap()
    }

    pub fn with_lsp_adapter(mut self, lsp_adapter: Arc<dyn LspAdapter>) -> Self {
        self.adapter = Some(lsp_adapter);
        self
    }

    #[cfg(any(test, feature = "test-support"))]
    pub fn set_fake_lsp_adapter(
        &mut self,
        fake_lsp_adapter: FakeLspAdapter,
    ) -> mpsc::UnboundedReceiver<lsp::FakeLanguageServer> {
        let (servers_tx, servers_rx) = mpsc::unbounded();
        let adapter = Arc::new(fake_lsp_adapter);
        self.fake_adapter = Some((servers_tx, adapter.clone()));
        self.adapter = Some(adapter);
        servers_rx
    }

    pub fn name(&self) -> Arc<str> {
        self.config.name.clone()
    }

    pub fn line_comment_prefix(&self) -> Option<&str> {
        self.config.line_comment.as_deref()
    }

    pub async fn disk_based_diagnostic_sources(&self) -> Vec<String> {
        match self.adapter.as_ref() {
            Some(adapter) => adapter.disk_based_diagnostic_sources().await,
            None => Vec::new(),
        }
    }

    pub async fn disk_based_diagnostics_progress_token(&self) -> Option<String> {
        if let Some(adapter) = self.adapter.as_ref() {
            adapter.disk_based_diagnostics_progress_token().await
        } else {
            None
        }
    }

    pub async fn process_diagnostics(&self, diagnostics: &mut lsp::PublishDiagnosticsParams) {
        if let Some(processor) = self.adapter.as_ref() {
            processor.process_diagnostics(diagnostics).await;
        }
    }

    pub async fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
    ) -> Option<CodeLabel> {
        self.adapter
            .as_ref()?
            .label_for_completion(completion, self)
            .await
    }

    pub async fn label_for_symbol(&self, name: &str, kind: lsp::SymbolKind) -> Option<CodeLabel> {
        self.adapter
            .as_ref()?
            .label_for_symbol(name, kind, self)
            .await
    }

    pub fn highlight_text<'a>(
        &'a self,
        text: &'a Rope,
        range: Range<usize>,
    ) -> Vec<(Range<usize>, HighlightId)> {
        let mut result = Vec::new();
        if let Some(grammar) = &self.grammar {
            let tree = grammar.parse_text(text, None);
            let mut offset = 0;
            for chunk in BufferChunks::new(text, range, Some(&tree), self.grammar.as_ref(), vec![])
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

    pub fn brackets(&self) -> &[BracketPair] {
        &self.config.brackets
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
}

impl Grammar {
    fn parse_text(&self, text: &Rope, old_tree: Option<Tree>) -> Tree {
        PARSER.with(|parser| {
            let mut parser = parser.borrow_mut();
            parser
                .set_language(self.ts_language)
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
            disk_based_diagnostics_sources: Vec::new(),
        }
    }
}

#[cfg(any(test, feature = "test-support"))]
#[async_trait]
impl LspAdapter for FakeLspAdapter {
    async fn name(&self) -> LanguageServerName {
        LanguageServerName(self.name.into())
    }

    async fn fetch_latest_server_version(
        &self,
        _: Arc<dyn HttpClient>,
    ) -> Result<Box<dyn 'static + Send + Any>> {
        unreachable!();
    }

    async fn fetch_server_binary(
        &self,
        _: Box<dyn 'static + Send + Any>,
        _: Arc<dyn HttpClient>,
        _: PathBuf,
    ) -> Result<PathBuf> {
        unreachable!();
    }

    async fn cached_server_binary(&self, _: PathBuf) -> Option<PathBuf> {
        unreachable!();
    }

    async fn process_diagnostics(&self, _: &mut lsp::PublishDiagnosticsParams) {}

    async fn disk_based_diagnostic_sources(&self) -> Vec<String> {
        self.disk_based_diagnostics_sources.clone()
    }

    async fn disk_based_diagnostics_progress_token(&self) -> Option<String> {
        self.disk_based_diagnostics_progress_token.clone()
    }
}

pub fn point_to_lsp(point: PointUtf16) -> lsp::Position {
    lsp::Position::new(point.row, point.column)
}

pub fn point_from_lsp(point: lsp::Position) -> PointUtf16 {
    PointUtf16::new(point.line, point.character)
}

pub fn range_to_lsp(range: Range<PointUtf16>) -> lsp::Range {
    lsp::Range {
        start: point_to_lsp(range.start),
        end: point_to_lsp(range.end),
    }
}

pub fn range_from_lsp(range: lsp::Range) -> Range<PointUtf16> {
    let mut start = point_from_lsp(range.start);
    let mut end = point_from_lsp(range.end);
    if start > end {
        mem::swap(&mut start, &mut end);
    }
    start..end
}
