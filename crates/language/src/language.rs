mod buffer;
mod diagnostic_set;
mod highlight_map;
pub mod proto;
#[cfg(test)]
mod tests;

use anyhow::{anyhow, Result};
use async_trait::async_trait;
pub use buffer::Operation;
pub use buffer::*;
use collections::HashSet;
pub use diagnostic_set::DiagnosticEntry;
use gpui::AppContext;
use highlight_map::HighlightMap;
use lazy_static::lazy_static;
use parking_lot::Mutex;
use serde::Deserialize;
use std::{
    path::{Path, PathBuf},
    str,
    sync::Arc,
};
use theme::SyntaxTheme;
use tree_sitter::{self, Query};
pub use tree_sitter::{Parser, Tree};

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

#[async_trait]
pub trait DiagnosticSource: 'static + Send + Sync {
    fn name(&self) -> Arc<str>;

    async fn diagnose(
        &self,
        path: Arc<Path>,
    ) -> Result<Vec<(PathBuf, Vec<DiagnosticEntry<usize>>)>>;
}

pub struct Language {
    pub(crate) config: LanguageConfig,
    pub(crate) grammar: Option<Arc<Grammar>>,
    pub(crate) diagnostic_source: Option<Arc<dyn DiagnosticSource>>,
}

pub struct Grammar {
    pub(crate) ts_language: tree_sitter::Language,
    pub(crate) highlights_query: Query,
    pub(crate) brackets_query: Query,
    pub(crate) indents_query: Query,
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
                    ts_language,
                    highlight_map: Default::default(),
                })
            }),
            diagnostic_source: None,
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

    pub fn with_diagnostic_source(mut self, source: impl DiagnosticSource) -> Self {
        self.diagnostic_source = Some(Arc::new(source));
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

    pub fn diagnostic_source(&self) -> Option<&Arc<dyn DiagnosticSource>> {
        self.diagnostic_source.as_ref()
    }

    pub fn disk_based_diagnostic_sources(&self) -> Option<&HashSet<String>> {
        self.config
            .language_server
            .as_ref()
            .map(|config| &config.disk_based_diagnostic_sources)
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
}

impl Grammar {
    pub fn highlight_map(&self) -> HighlightMap {
        self.highlight_map.lock().clone()
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
                ..Default::default()
            },
            fake,
        )
    }
}
