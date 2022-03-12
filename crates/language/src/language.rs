mod buffer;
mod diagnostic_set;
mod highlight_map;
mod outline;
pub mod proto;
#[cfg(test)]
mod tests;

use anyhow::{anyhow, Context, Result};
use client::http::{self, HttpClient};
use collections::HashSet;
use futures::{
    future::{BoxFuture, Shared},
    FutureExt, TryFutureExt,
};
use gpui::{MutableAppContext, Task};
use highlight_map::HighlightMap;
use lazy_static::lazy_static;
use parking_lot::{Mutex, RwLock};
use serde::Deserialize;
use serde_json::Value;
use std::{
    cell::RefCell,
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
            path_suffixes: Default::default(),
            brackets: Default::default(),
            line_comment: None,
            language_server: None,
        },
        None,
    ));
}

pub trait ToLspPosition {
    fn to_lsp_position(self) -> lsp::Position;
}

pub struct LspBinaryVersion {
    pub name: String,
    pub url: Option<http::Url>,
}

pub trait LspAdapter: 'static + Send + Sync {
    fn name(&self) -> &'static str;
    fn fetch_latest_server_version(
        &self,
        http: Arc<dyn HttpClient>,
    ) -> BoxFuture<'static, Result<LspBinaryVersion>>;
    fn fetch_server_binary(
        &self,
        version: LspBinaryVersion,
        http: Arc<dyn HttpClient>,
        container_dir: PathBuf,
    ) -> BoxFuture<'static, Result<PathBuf>>;
    fn cached_server_binary(&self, container_dir: PathBuf) -> BoxFuture<'static, Option<PathBuf>>;
    fn process_diagnostics(&self, diagnostics: &mut lsp::PublishDiagnosticsParams);

    fn label_for_completion(&self, _: &lsp::CompletionItem, _: &Language) -> Option<CodeLabel> {
        None
    }

    fn label_for_symbol(&self, _: &str, _: lsp::SymbolKind, _: &Language) -> Option<CodeLabel> {
        None
    }

    fn server_args(&self) -> &[&str] {
        &[]
    }

    fn initialization_options(&self) -> Option<Value> {
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
    pub line_comment: Option<String>,
    pub language_server: Option<LanguageServerConfig>,
}

impl Default for LanguageConfig {
    fn default() -> Self {
        Self {
            name: "".into(),
            path_suffixes: Default::default(),
            brackets: Default::default(),
            line_comment: Default::default(),
            language_server: Default::default(),
        }
    }
}

#[derive(Default, Deserialize)]
pub struct LanguageServerConfig {
    pub disk_based_diagnostic_sources: HashSet<String>,
    pub disk_based_diagnostics_progress_token: Option<String>,
    #[cfg(any(test, feature = "test-support"))]
    #[serde(skip)]
    fake_config: Option<FakeLanguageServerConfig>,
}

#[cfg(any(test, feature = "test-support"))]
struct FakeLanguageServerConfig {
    servers_tx: mpsc::UnboundedSender<lsp::FakeLanguageServer>,
    capabilities: lsp::ServerCapabilities,
    initializer: Option<Box<dyn 'static + Send + Sync + Fn(&mut lsp::FakeLanguageServer)>>,
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
    lsp_binary_path: Mutex<Option<Shared<BoxFuture<'static, Result<PathBuf, Arc<anyhow::Error>>>>>>,
}

pub struct Grammar {
    pub(crate) ts_language: tree_sitter::Language,
    pub(crate) highlights_query: Query,
    pub(crate) brackets_query: Query,
    pub(crate) indents_query: Query,
    pub(crate) outline_query: Query,
    pub(crate) highlight_map: Mutex<HighlightMap>,
}

#[derive(Clone)]
pub enum LanguageServerBinaryStatus {
    CheckingForUpdate,
    Downloading,
    Downloaded,
    Cached,
    Failed,
}

pub struct LanguageRegistry {
    languages: RwLock<Vec<Arc<Language>>>,
    language_server_download_dir: Option<Arc<Path>>,
    lsp_binary_statuses_tx: async_broadcast::Sender<(Arc<Language>, LanguageServerBinaryStatus)>,
    lsp_binary_statuses_rx: async_broadcast::Receiver<(Arc<Language>, LanguageServerBinaryStatus)>,
    login_shell_env_loaded: Shared<Task<()>>,
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
            .find(|language| language.name().as_ref() == name)
            .cloned()
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
                    .any(|suffix| path_suffixes.contains(&Some(suffix.as_str())))
            })
            .cloned()
    }

    pub fn start_language_server(
        &self,
        language: Arc<Language>,
        root_path: Arc<Path>,
        http_client: Arc<dyn HttpClient>,
        cx: &mut MutableAppContext,
    ) -> Option<Task<Result<lsp::LanguageServer>>> {
        #[cfg(any(test, feature = "test-support"))]
        if language
            .config
            .language_server
            .as_ref()
            .and_then(|config| config.fake_config.as_ref())
            .is_some()
        {
            let language = language.clone();
            return Some(cx.spawn(|mut cx| async move {
                let fake_config = language
                    .config
                    .language_server
                    .as_ref()
                    .unwrap()
                    .fake_config
                    .as_ref()
                    .unwrap();
                let (server, mut fake_server) = cx.update(|cx| {
                    lsp::LanguageServer::fake_with_capabilities(
                        fake_config.capabilities.clone(),
                        cx,
                    )
                });
                if let Some(initializer) = &fake_config.initializer {
                    initializer(&mut fake_server);
                }

                let servers_tx = fake_config.servers_tx.clone();
                cx.background()
                    .spawn(async move {
                        fake_server
                            .receive_notification::<lsp::notification::Initialized>()
                            .await;
                        servers_tx.unbounded_send(fake_server).ok();
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

        let adapter = language.adapter.clone()?;
        let background = cx.background().clone();
        let lsp_binary_statuses = self.lsp_binary_statuses_tx.clone();
        let login_shell_env_loaded = self.login_shell_env_loaded.clone();
        Some(cx.background().spawn(async move {
            login_shell_env_loaded.await;
            let server_binary_path = language
                .lsp_binary_path
                .lock()
                .get_or_insert_with(|| {
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
            let server_args = adapter.server_args();
            lsp::LanguageServer::new(
                &server_binary_path,
                server_args,
                &root_path,
                adapter.initialization_options(),
                background,
            )
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
    let container_dir = download_dir.join(adapter.name());
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
    if path.is_err() {
        if let Some(cached_path) = adapter.cached_server_binary(container_dir).await {
            statuses
                .broadcast((language.clone(), LanguageServerBinaryStatus::Cached))
                .await?;
            return Ok(cached_path);
        } else {
            statuses
                .broadcast((language.clone(), LanguageServerBinaryStatus::Failed))
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
        .fetch_server_binary(version_info, http_client, container_dir.to_path_buf())
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
                    brackets_query: Query::new(ts_language, "").unwrap(),
                    highlights_query: Query::new(ts_language, "").unwrap(),
                    indents_query: Query::new(ts_language, "").unwrap(),
                    outline_query: Query::new(ts_language, "").unwrap(),
                    ts_language,
                    highlight_map: Default::default(),
                })
            }),
            adapter: None,
            lsp_binary_path: Default::default(),
        }
    }

    pub fn with_highlights_query(mut self, source: &str) -> Result<Self> {
        let grammar = self
            .grammar
            .as_mut()
            .and_then(Arc::get_mut)
            .ok_or_else(|| anyhow!("grammar does not exist or is already being used"))?;
        grammar.highlights_query = Query::new(grammar.ts_language, source)?;
        Ok(self)
    }

    pub fn with_brackets_query(mut self, source: &str) -> Result<Self> {
        let grammar = self
            .grammar
            .as_mut()
            .and_then(Arc::get_mut)
            .ok_or_else(|| anyhow!("grammar does not exist or is already being used"))?;
        grammar.brackets_query = Query::new(grammar.ts_language, source)?;
        Ok(self)
    }

    pub fn with_indents_query(mut self, source: &str) -> Result<Self> {
        let grammar = self
            .grammar
            .as_mut()
            .and_then(Arc::get_mut)
            .ok_or_else(|| anyhow!("grammar does not exist or is already being used"))?;
        grammar.indents_query = Query::new(grammar.ts_language, source)?;
        Ok(self)
    }

    pub fn with_outline_query(mut self, source: &str) -> Result<Self> {
        let grammar = self
            .grammar
            .as_mut()
            .and_then(Arc::get_mut)
            .ok_or_else(|| anyhow!("grammar does not exist or is already being used"))?;
        grammar.outline_query = Query::new(grammar.ts_language, source)?;
        Ok(self)
    }

    pub fn with_lsp_adapter(mut self, lsp_adapter: impl LspAdapter) -> Self {
        self.adapter = Some(Arc::new(lsp_adapter));
        self
    }

    pub fn name(&self) -> Arc<str> {
        self.config.name.clone()
    }

    pub fn line_comment_prefix(&self) -> Option<&str> {
        self.config.line_comment.as_deref()
    }

    pub fn disk_based_diagnostic_sources(&self) -> Option<&HashSet<String>> {
        self.config
            .language_server
            .as_ref()
            .map(|config| &config.disk_based_diagnostic_sources)
    }

    pub fn disk_based_diagnostics_progress_token(&self) -> Option<&String> {
        self.config
            .language_server
            .as_ref()
            .and_then(|config| config.disk_based_diagnostics_progress_token.as_ref())
    }

    pub fn process_diagnostics(&self, diagnostics: &mut lsp::PublishDiagnosticsParams) {
        if let Some(processor) = self.adapter.as_ref() {
            processor.process_diagnostics(diagnostics);
        }
    }

    pub fn label_for_completion(&self, completion: &lsp::CompletionItem) -> Option<CodeLabel> {
        self.adapter
            .as_ref()?
            .label_for_completion(completion, self)
    }

    pub fn label_for_symbol(&self, name: &str, kind: lsp::SymbolKind) -> Option<CodeLabel> {
        self.adapter.as_ref()?.label_for_symbol(name, kind, self)
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
                    result.push((offset..end_offset, highlight_id));
                }
                offset = end_offset;
            }
        }
        result
    }

    pub fn brackets(&self) -> &[BracketPair] {
        &self.config.brackets
    }

    pub fn set_theme(&self, theme: &SyntaxTheme) {
        if let Some(grammar) = self.grammar.as_ref() {
            *grammar.highlight_map.lock() =
                HighlightMap::new(grammar.highlights_query.capture_names(), theme);
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
        let capture_id = self.highlights_query.capture_index_for_name(name)?;
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
impl LanguageServerConfig {
    pub fn fake() -> (Self, mpsc::UnboundedReceiver<lsp::FakeLanguageServer>) {
        let (servers_tx, servers_rx) = mpsc::unbounded();
        (
            Self {
                fake_config: Some(FakeLanguageServerConfig {
                    servers_tx,
                    capabilities: lsp::LanguageServer::full_capabilities(),
                    initializer: None,
                }),
                disk_based_diagnostics_progress_token: Some("fakeServer/check".to_string()),
                ..Default::default()
            },
            servers_rx,
        )
    }

    pub fn set_fake_capabilities(&mut self, capabilities: lsp::ServerCapabilities) {
        self.fake_config.as_mut().unwrap().capabilities = capabilities;
    }

    pub fn set_fake_initializer(
        &mut self,
        initializer: impl 'static + Send + Sync + Fn(&mut lsp::FakeLanguageServer),
    ) {
        self.fake_config.as_mut().unwrap().initializer = Some(Box::new(initializer));
    }
}

impl ToLspPosition for PointUtf16 {
    fn to_lsp_position(self) -> lsp::Position {
        lsp::Position::new(self.row, self.column)
    }
}

pub fn point_from_lsp(point: lsp::Position) -> PointUtf16 {
    PointUtf16::new(point.line, point.character)
}

pub fn range_from_lsp(range: lsp::Range) -> Range<PointUtf16> {
    let start = PointUtf16::new(range.start.line, range.start.character);
    let end = PointUtf16::new(range.end.line, range.end.character);
    start..end
}
