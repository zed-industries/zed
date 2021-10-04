use buffer::{HighlightMap, Language, LanguageRegistry};
use parking_lot::Mutex;
use rust_embed::RustEmbed;
use std::{str, sync::Arc};
use tree_sitter::Query;

#[derive(RustEmbed)]
#[folder = "languages"]
struct LanguageDir;

pub fn build_language_registry() -> LanguageRegistry {
    let mut languages = LanguageRegistry::default();
    languages.add(Arc::new(rust()));
    languages
}

pub fn rust() -> Language {
    let grammar = tree_sitter_rust::language();
    let rust_config =
        toml::from_slice(&LanguageDir::get("rust/config.toml").unwrap().data).unwrap();
    Language {
        config: rust_config,
        grammar,
        highlight_query: load_query(grammar, "rust/highlights.scm"),
        brackets_query: load_query(grammar, "rust/brackets.scm"),
        highlight_map: Mutex::new(HighlightMap::default()),
    }
}

fn load_query(grammar: tree_sitter::Language, path: &str) -> Query {
    Query::new(
        grammar,
        str::from_utf8(&LanguageDir::get(path).unwrap().data).unwrap(),
    )
    .unwrap()
}
