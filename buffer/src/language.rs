use crate::{HighlightMap, SyntaxTheme};
use parking_lot::Mutex;
use serde::Deserialize;
use std::str;
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
