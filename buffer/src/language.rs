use crate::{HighlightMap, SyntaxTheme};
use parking_lot::Mutex;
use serde::Deserialize;
use std::{path::Path, str, sync::Arc};
use tree_sitter::{Language as Grammar, Query};
pub use tree_sitter::{Parser, Tree};

#[derive(Default, Deserialize)]
pub struct LanguageConfig {
    pub name: String,
    pub path_suffixes: Vec<String>,
}

#[derive(Deserialize)]
pub struct BracketPair {
    pub start: String,
    pub end: String,
}

pub struct Language {
    pub config: LanguageConfig,
    pub grammar: Grammar,
    pub highlight_query: Query,
    pub brackets_query: Query,
    pub highlight_map: Mutex<HighlightMap>,
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
    pub fn name(&self) -> &str {
        self.config.name.as_str()
    }

    pub fn highlight_map(&self) -> HighlightMap {
        self.highlight_map.lock().clone()
    }

    pub fn set_theme(&self, theme: &SyntaxTheme) {
        *self.highlight_map.lock() = HighlightMap::new(self.highlight_query.capture_names(), theme);
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
                Arc::new(Language {
                    config: LanguageConfig {
                        name: "Rust".to_string(),
                        path_suffixes: vec!["rs".to_string()],
                        ..Default::default()
                    },
                    grammar,
                    highlight_query: Query::new(grammar, "").unwrap(),
                    brackets_query: Query::new(grammar, "").unwrap(),
                    highlight_map: Default::default(),
                }),
                Arc::new(Language {
                    config: LanguageConfig {
                        name: "Make".to_string(),
                        path_suffixes: vec!["Makefile".to_string(), "mk".to_string()],
                        ..Default::default()
                    },
                    grammar,
                    highlight_query: Query::new(grammar, "").unwrap(),
                    brackets_query: Query::new(grammar, "").unwrap(),
                    highlight_map: Default::default(),
                }),
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
