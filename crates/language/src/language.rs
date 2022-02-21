mod buffer;
mod diagnostic_set;
mod highlight_map;
mod outline;
pub mod proto;
#[cfg(test)]
mod tests;

use anyhow::{anyhow, Result};
use client::http::HttpClient;
use collections::HashSet;
use futures::{
    future::{BoxFuture, Shared},
    FutureExt, TryFutureExt,
};
use gpui::{AppContext, Task};
use highlight_map::HighlightMap;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use postage::watch;
use serde::Deserialize;
use std::{
    cell::RefCell,
    ops::Range,
    path::{Path, PathBuf},
    str,
    sync::Arc,
};
use theme::SyntaxTheme;
use tree_sitter::{self, Query};

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
            name: "Plain Text".to_string(),
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

pub trait LspExt: 'static + Send + Sync {
    fn fetch_latest_language_server(
        &self,
        http: Arc<dyn HttpClient>,
    ) -> BoxFuture<'static, Result<PathBuf>>;
    fn process_diagnostics(&self, diagnostics: &mut lsp::PublishDiagnosticsParams);
    fn label_for_completion(
        &self,
        _: &lsp::CompletionItem,
        _: &Language,
    ) -> Option<CompletionLabel> {
        None
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct CompletionLabel {
    pub text: String,
    pub runs: Vec<(Range<usize>, HighlightId)>,
    pub filter_range: Range<usize>,
    pub left_aligned_len: usize,
}

#[derive(Default, Deserialize)]
pub struct LanguageConfig {
    pub name: String,
    pub path_suffixes: Vec<String>,
    pub brackets: Vec<BracketPair>,
    pub line_comment: Option<String>,
    pub language_server: Option<LanguageServerConfig>,
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
    pub(crate) lsp_ext: Option<Box<dyn LspExt>>,
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

pub struct LanguageRegistry {
    languages: Vec<Arc<Language>>,
    pending_lsp_binaries_tx: Arc<Mutex<watch::Sender<usize>>>,
    pending_lsp_binaries_rx: watch::Receiver<usize>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        let (pending_lsp_binaries_tx, pending_lsp_binaries_rx) = watch::channel();
        Self {
            languages: Default::default(),
            pending_lsp_binaries_tx: Arc::new(Mutex::new(pending_lsp_binaries_tx)),
            pending_lsp_binaries_rx,
        }
    }

    pub fn add(&mut self, language: Arc<Language>) {
        self.languages.push(language.clone());
    }

    pub fn set_theme(&self, theme: &SyntaxTheme) {
        for language in &self.languages {
            language.set_theme(theme);
        }
    }

    pub fn get_language(&self, name: &str) -> Option<&Arc<Language>> {
        self.languages
            .iter()
            .find(|language| language.name() == name)
    }

    pub fn select_language(&self, path: impl AsRef<Path>) -> Option<&Arc<Language>> {
        let path = path.as_ref();
        let filename = path.file_name().and_then(|name| name.to_str());
        let extension = path.extension().and_then(|name| name.to_str());
        let path_suffixes = [extension, filename];
        self.languages.iter().find(|language| {
            language
                .config
                .path_suffixes
                .iter()
                .any(|suffix| path_suffixes.contains(&Some(suffix.as_str())))
        })
    }

    pub fn start_language_server(
        &self,
        language: &Arc<Language>,
        root_path: Arc<Path>,
        http_client: Arc<dyn HttpClient>,
        cx: &AppContext,
    ) -> Option<Task<Result<Arc<lsp::LanguageServer>>>> {
        #[cfg(any(test, feature = "test-support"))]
        if let Some(config) = &language.config.language_server {
            if let Some(fake_config) = &config.fake_config {
                use postage::prelude::Stream;

                let (server, mut fake_server) = lsp::LanguageServer::fake_with_capabilities(
                    fake_config.capabilities.clone(),
                    cx.background().clone(),
                );

                if let Some(initalizer) = &fake_config.initializer {
                    initalizer(&mut fake_server);
                }

                let servers_tx = fake_config.servers_tx.clone();
                let mut initialized = server.capabilities();
                cx.background()
                    .spawn(async move {
                        while initialized.recv().await.is_none() {}
                        servers_tx.unbounded_send(fake_server).ok();
                    })
                    .detach();

                return Some(Task::ready(Ok(server.clone())));
            }
        }

        let lsp_ext = language.lsp_ext.as_ref()?;
        let background = cx.background().clone();
        let server_binary_path = {
            Some(
                language
                    .lsp_binary_path
                    .lock()
                    .get_or_insert_with(|| {
                        let pending_lsp_binaries_tx = self.pending_lsp_binaries_tx.clone();
                        let language_server_path =
                            lsp_ext.fetch_latest_language_server(http_client);
                        async move {
                            *pending_lsp_binaries_tx.lock().borrow_mut() += 1;
                            let path = language_server_path.map_err(Arc::new).await;
                            *pending_lsp_binaries_tx.lock().borrow_mut() -= 1;
                            path
                        }
                        .boxed()
                        .shared()
                    })
                    .clone()
                    .map_err(|e| anyhow!(e)),
            )
        }?;
        Some(cx.background().spawn(async move {
            let server_binary_path = server_binary_path.await?;
            let server = lsp::LanguageServer::new(&server_binary_path, &root_path, background)?;
            Ok(server)
        }))
    }

    pub fn pending_lsp_binaries(&self) -> watch::Receiver<usize> {
        self.pending_lsp_binaries_rx.clone()
    }
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
            lsp_ext: None,
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

    pub fn with_lsp_ext(mut self, processor: impl LspExt) -> Self {
        self.lsp_ext = Some(Box::new(processor));
        self
    }

    pub fn name(&self) -> &str {
        self.config.name.as_str()
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
        if let Some(processor) = self.lsp_ext.as_ref() {
            processor.process_diagnostics(diagnostics);
        }
    }

    pub fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
    ) -> Option<CompletionLabel> {
        self.lsp_ext
            .as_ref()?
            .label_for_completion(completion, self)
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
                if let Some(highlight_id) = chunk.highlight_id {
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

impl CompletionLabel {
    pub fn plain(completion: &lsp::CompletionItem) -> Self {
        let mut result = Self {
            text: completion.label.clone(),
            runs: Vec::new(),
            left_aligned_len: completion.label.len(),
            filter_range: 0..completion.label.len(),
        };
        if let Some(filter_text) = &completion.filter_text {
            if let Some(ix) = completion.label.find(filter_text) {
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
                    capabilities: Default::default(),
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
