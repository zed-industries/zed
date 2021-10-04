use buffer::{HighlightMap, Language, SyntaxTheme};
use parking_lot::Mutex;
use rust_embed::RustEmbed;
use std::{path::Path, str, sync::Arc};
use tree_sitter::Query;
pub use tree_sitter::{Parser, Tree};

#[derive(RustEmbed)]
#[folder = "languages"]
pub struct LanguageDir;

pub struct LanguageRegistry {
    languages: Vec<Arc<Language>>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        let grammar = tree_sitter_rust::language();
        let rust_config =
            toml::from_slice(&LanguageDir::get("rust/config.toml").unwrap().data).unwrap();
        let rust_language = Language {
            config: rust_config,
            grammar,
            highlight_query: Self::load_query(grammar, "rust/highlights.scm"),
            brackets_query: Self::load_query(grammar, "rust/brackets.scm"),
            highlight_map: Mutex::new(HighlightMap::default()),
        };

        Self {
            languages: vec![Arc::new(rust_language)],
        }
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

    fn load_query(grammar: tree_sitter::Language, path: &str) -> Query {
        Query::new(
            grammar,
            str::from_utf8(&LanguageDir::get(path).unwrap().data).unwrap(),
        )
        .unwrap()
    }
}

impl Default for LanguageRegistry {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use buffer::LanguageConfig;

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
