mod buffer;
mod diagnostic_set;
mod highlight_map;
mod outline;
pub mod proto;
#[cfg(test)]
mod tests;

use anyhow::{anyhow, Result};
pub use buffer::Operation;
pub use buffer::*;
use collections::HashSet;
pub use diagnostic_set::DiagnosticEntry;
use gpui::AppContext;
use highlight_map::HighlightMap;
use lazy_static::lazy_static;
pub use outline::{Outline, OutlineItem};
use parking_lot::Mutex;
use serde::Deserialize;
use std::{cell::RefCell, ops::Range, path::Path, str, sync::Arc};
use theme::SyntaxTheme;
use tree_sitter::{self, Query};
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

pub trait ToPointUtf16 {
    fn to_point_utf16(self) -> PointUtf16;
}

pub trait ToLspPosition {
    fn to_lsp_position(self) -> lsp::Position;
}

pub trait LspPostProcessor: 'static + Send + Sync {
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
    pub binary: String,
    pub disk_based_diagnostic_sources: HashSet<String>,
    pub disk_based_diagnostics_progress_token: Option<String>,
    #[cfg(any(test, feature = "test-support"))]
    #[serde(skip)]
    pub fake_server: Option<(Arc<lsp::LanguageServer>, Arc<std::sync::atomic::AtomicBool>)>,
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
    pub(crate) lsp_post_processor: Option<Box<dyn LspPostProcessor>>,
}

pub struct Grammar {
    pub(crate) ts_language: tree_sitter::Language,
    pub(crate) highlights_query: Query,
    pub(crate) brackets_query: Query,
    pub(crate) indents_query: Query,
    pub(crate) outline_query: Query,
    pub(crate) highlight_map: Mutex<HighlightMap>,
}

#[derive(Default)]
pub struct LanguageRegistry {
    languages: Vec<Arc<Language>>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn add(&mut self, language: Arc<Language>) {
        self.languages.push(language);
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
            lsp_post_processor: None,
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

    pub fn with_lsp_post_processor(mut self, processor: impl LspPostProcessor) -> Self {
        self.lsp_post_processor = Some(Box::new(processor));
        self
    }

    pub fn name(&self) -> &str {
        self.config.name.as_str()
    }

    pub fn line_comment_prefix(&self) -> Option<&str> {
        self.config.line_comment.as_deref()
    }

    pub fn start_server(
        &self,
        root_path: &Path,
        cx: &AppContext,
    ) -> Result<Option<Arc<lsp::LanguageServer>>> {
        if let Some(config) = &self.config.language_server {
            #[cfg(any(test, feature = "test-support"))]
            if let Some((server, started)) = &config.fake_server {
                started.store(true, std::sync::atomic::Ordering::SeqCst);
                return Ok(Some(server.clone()));
            }

            const ZED_BUNDLE: Option<&'static str> = option_env!("ZED_BUNDLE");
            let binary_path = if ZED_BUNDLE.map_or(Ok(false), |b| b.parse())? {
                cx.platform()
                    .path_for_resource(Some(&config.binary), None)?
            } else {
                Path::new(&config.binary).to_path_buf()
            };
            lsp::LanguageServer::new(&binary_path, root_path, cx.background().clone()).map(Some)
        } else {
            Ok(None)
        }
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
        if let Some(processor) = self.lsp_post_processor.as_ref() {
            processor.process_diagnostics(diagnostics);
        }
    }

    pub fn label_for_completion(
        &self,
        completion: &lsp::CompletionItem,
    ) -> Option<CompletionLabel> {
        self.lsp_post_processor
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
    fn plain(completion: &lsp::CompletionItem) -> Self {
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
    pub async fn fake(
        executor: Arc<gpui::executor::Background>,
    ) -> (Self, lsp::FakeLanguageServer) {
        let (server, fake) = lsp::LanguageServer::fake(executor).await;
        fake.started
            .store(false, std::sync::atomic::Ordering::SeqCst);
        let started = fake.started.clone();
        (
            Self {
                fake_server: Some((server, started)),
                disk_based_diagnostics_progress_token: Some("fakeServer/check".to_string()),
                ..Default::default()
            },
            fake,
        )
    }
}

impl ToPointUtf16 for lsp::Position {
    fn to_point_utf16(self) -> PointUtf16 {
        PointUtf16::new(self.line, self.character)
    }
}

impl ToLspPosition for PointUtf16 {
    fn to_lsp_position(self) -> lsp::Position {
        lsp::Position::new(self.row, self.column)
    }
}

pub fn range_from_lsp(range: lsp::Range) -> Range<PointUtf16> {
    let start = PointUtf16::new(range.start.line, range.start.character);
    let end = PointUtf16::new(range.end.line, range.end.character);
    start..end
}
