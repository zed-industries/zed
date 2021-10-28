use crate::HighlightMap;
use anyhow::Result;
use gpui::AppContext;
use parking_lot::Mutex;
use serde::Deserialize;
use std::{collections::HashSet, path::Path, str, sync::Arc};
use theme::SyntaxTheme;
use tree_sitter::{Language as Grammar, Query};
pub use tree_sitter::{Parser, Tree};

#[derive(Default, Deserialize)]
pub struct LanguageConfig {
    pub name: String,
    pub path_suffixes: Vec<String>,
    pub brackets: Vec<BracketPair>,
    pub language_server: Option<LanguageServerConfig>,
}

#[derive(Deserialize)]
pub struct LanguageServerConfig {
    pub binary: String,
    pub disk_based_diagnostic_sources: HashSet<String>,
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
    pub(crate) grammar: Grammar,
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
    pub fn new(config: LanguageConfig, grammar: Grammar) -> Self {
        Self {
            config,
            brackets_query: Query::new(grammar, "").unwrap(),
            highlights_query: Query::new(grammar, "").unwrap(),
            indents_query: Query::new(grammar, "").unwrap(),
            grammar,
            highlight_map: Default::default(),
        }
    }

    pub fn with_highlights_query(mut self, source: &str) -> Result<Self> {
        self.highlights_query = Query::new(self.grammar, source)?;
        Ok(self)
    }

    pub fn with_brackets_query(mut self, source: &str) -> Result<Self> {
        self.brackets_query = Query::new(self.grammar, source)?;
        Ok(self)
    }

    pub fn with_indents_query(mut self, source: &str) -> Result<Self> {
        self.indents_query = Query::new(self.grammar, source)?;
        Ok(self)
    }

    pub fn name(&self) -> &str {
        self.config.name.as_str()
    }

    pub fn start_server(
        &self,
        root_path: &Path,
        cx: &AppContext,
    ) -> Result<Option<Arc<lsp::LanguageServer>>> {
        if let Some(config) = &self.config.language_server {
            const ZED_BUNDLE: Option<&'static str> = option_env!("ZED_BUNDLE");
            let binary_path = if ZED_BUNDLE.map_or(Ok(false), |b| b.parse())? {
                cx.platform()
                    .path_for_resource(Some(&config.binary), None)?
            } else {
                Path::new(&config.binary).to_path_buf()
            };
            lsp::LanguageServer::new(&binary_path, root_path, cx.background()).map(Some)
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

    pub fn brackets(&self) -> &[BracketPair] {
        &self.config.brackets
    }

    pub fn highlight_map(&self) -> HighlightMap {
        self.highlight_map.lock().clone()
    }

    pub fn set_theme(&self, theme: &SyntaxTheme) {
        *self.highlight_map.lock() =
            HighlightMap::new(self.highlights_query.capture_names(), theme);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_select_language() {
        let grammar = tree_sitter_rust::language();
        let registry = LanguageRegistry {
            languages: vec![
                Arc::new(Language::new(
                    LanguageConfig {
                        name: "Rust".to_string(),
                        path_suffixes: vec!["rs".to_string()],
                        ..Default::default()
                    },
                    grammar,
                )),
                Arc::new(Language::new(
                    LanguageConfig {
                        name: "Make".to_string(),
                        path_suffixes: vec!["Makefile".to_string(), "mk".to_string()],
                        ..Default::default()
                    },
                    grammar,
                )),
            ],
        };

        // matching file extension
        assert_eq!(
            registry.select_language("zed/lib.rs").map(|l| l.name()),
            Some("Rust")
        );
        assert_eq!(
            registry.select_language("zed/lib.mk").map(|l| l.name()),
            Some("Make")
        );

        // matching filename
        assert_eq!(
            registry.select_language("zed/Makefile").map(|l| l.name()),
            Some("Make")
        );

        // matching suffix that is not the full file extension or filename
        assert_eq!(registry.select_language("zed/cars").map(|l| l.name()), None);
        assert_eq!(
            registry.select_language("zed/a.cars").map(|l| l.name()),
            None
        );
        assert_eq!(registry.select_language("zed/sumk").map(|l| l.name()), None);
    }
}
