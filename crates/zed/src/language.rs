pub use language::*;
use rust_embed::RustEmbed;
use std::borrow::Cow;
use std::{str, sync::Arc};

#[derive(RustEmbed)]
#[folder = "languages"]
struct LanguageDir;

pub fn build_language_registry() -> LanguageRegistry {
    let mut languages = LanguageRegistry::default();
    languages.add(Arc::new(rust()));
    languages.add(Arc::new(markdown()));
    languages
}

fn rust() -> Language {
    let grammar = tree_sitter_rust::language();
    let config = toml::from_slice(&LanguageDir::get("rust/config.toml").unwrap().data).unwrap();
    Language::new(config, grammar)
        .with_highlights_query(load_query("rust/highlights.scm").as_ref())
        .unwrap()
        .with_brackets_query(load_query("rust/brackets.scm").as_ref())
        .unwrap()
        .with_indents_query(load_query("rust/indents.scm").as_ref())
        .unwrap()
}

fn markdown() -> Language {
    let grammar = tree_sitter_markdown::language();
    let config = toml::from_slice(&LanguageDir::get("markdown/config.toml").unwrap().data).unwrap();
    Language::new(config, grammar)
}

fn load_query(path: &str) -> Cow<'static, str> {
    match LanguageDir::get(path).unwrap().data {
        Cow::Borrowed(s) => Cow::Borrowed(str::from_utf8(s).unwrap()),
        Cow::Owned(s) => Cow::Owned(String::from_utf8(s).unwrap()),
    }
}
