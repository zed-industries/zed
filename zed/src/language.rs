use rust_embed::RustEmbed;
use std::{path::Path, sync::Arc};
use tree_sitter::{Language as Grammar, Query};

pub use tree_sitter::{Parser, Tree};

#[derive(RustEmbed)]
#[folder = "languages"]
pub struct LanguageDir;

pub struct Language {
    pub name: String,
    pub grammar: Grammar,
    pub highlight_query: Query,
    path_suffixes: Vec<String>,
}

pub struct LanguageRegistry {
    languages: Vec<Arc<Language>>,
}

impl LanguageRegistry {
    pub fn new() -> Self {
        let grammar = tree_sitter_rust::language();
        let rust_language = Language {
            name: "Rust".to_string(),
            grammar,
            highlight_query: Query::new(
                grammar,
                std::str::from_utf8(LanguageDir::get("rust/highlights.scm").unwrap().as_ref())
                    .unwrap(),
            )
            .unwrap(),
            path_suffixes: vec!["rs".to_string()],
        };

        Self {
            languages: vec![Arc::new(rust_language)],
        }
    }

    pub fn select_language(&self, path: impl AsRef<Path>) -> Option<&Arc<Language>> {
        let path = path.as_ref();
        let filename = path.file_name().and_then(|name| name.to_str());
        let extension = path.extension().and_then(|name| name.to_str());
        let path_suffixes = [extension, filename];
        self.languages.iter().find(|language| {
            language
                .path_suffixes
                .iter()
                .any(|suffix| path_suffixes.contains(&Some(suffix.as_str())))
        })
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
                    name: "Rust".to_string(),
                    grammar,
                    highlight_query: Query::new(grammar, "").unwrap(),
                    path_suffixes: vec!["rs".to_string()],
                }),
                Arc::new(Language {
                    name: "Make".to_string(),
                    grammar,
                    highlight_query: Query::new(grammar, "").unwrap(),
                    path_suffixes: vec!["Makefile".to_string(), "mk".to_string()],
                }),
            ],
        };

        // matching file extension
        assert_eq!(
            registry.select_language("zed/lib.rs").map(get_name),
            Some("Rust")
        );
        assert_eq!(
            registry.select_language("zed/lib.mk").map(get_name),
            Some("Make")
        );

        // matching filename
        assert_eq!(
            registry.select_language("zed/Makefile").map(get_name),
            Some("Make")
        );

        // matching suffix that is not the full file extension or filename
        assert_eq!(registry.select_language("zed/cars").map(get_name), None);
        assert_eq!(registry.select_language("zed/a.cars").map(get_name), None);
        assert_eq!(registry.select_language("zed/sumk").map(get_name), None);

        fn get_name(language: &Arc<Language>) -> &str {
            language.name.as_str()
        }
    }
}
