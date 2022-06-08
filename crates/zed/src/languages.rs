use gpui::{
    executor::{self, Background},
    Task,
};
pub use language::*;
use rust_embed::RustEmbed;
use std::{borrow::Cow, str, sync::Arc};
use util::ResultExt;

mod c;
mod go;
mod installation;
mod language_plugin;
mod python;
mod rust;
mod typescript;

#[derive(RustEmbed)]
#[folder = "src/languages"]
#[exclude = "*.rs"]
struct LanguageDir;

pub async fn init(languages: Arc<LanguageRegistry>, executor: Arc<Background>) {
    for (name, grammar, lsp_adapter) in [
        (
            "c",
            tree_sitter_c::language(),
            Some(Arc::new(c::CLspAdapter) as Arc<dyn LspAdapter>),
        ),
        (
            "cpp",
            tree_sitter_cpp::language(),
            Some(Arc::new(c::CLspAdapter) as Arc<dyn LspAdapter>),
        ),
        (
            "go",
            tree_sitter_go::language(),
            Some(Arc::new(go::GoLspAdapter) as Arc<dyn LspAdapter>),
        ),
        (
            "json",
            tree_sitter_json::language(),
            // Some(Arc::new(json::JsonLspAdapter)),
            language_plugin::new_json(executor)
                .await
                .log_err()
                .map(|lang| Arc::new(lang) as Arc<_>),
        ),
        (
            "markdown",
            tree_sitter_markdown::language(),
            None, //
        ),
        (
            "python",
            tree_sitter_python::language(),
            Some(Arc::new(python::PythonLspAdapter)),
        ),
        (
            "rust",
            tree_sitter_rust::language(),
            Some(Arc::new(rust::RustLspAdapter)),
        ),
        (
            "toml",
            tree_sitter_toml::language(),
            None, //
        ),
        (
            "tsx",
            tree_sitter_typescript::language_tsx(),
            Some(Arc::new(typescript::TypeScriptLspAdapter)),
        ),
        (
            "typescript",
            tree_sitter_typescript::language_typescript(),
            Some(Arc::new(typescript::TypeScriptLspAdapter)),
        ),
        (
            "javascript",
            tree_sitter_typescript::language_tsx(),
            Some(Arc::new(typescript::TypeScriptLspAdapter)),
        ),
    ] {
        languages.add(Arc::new(language(name, grammar, lsp_adapter)));
    }
}

pub(crate) fn language(
    name: &str,
    grammar: tree_sitter::Language,
    lsp_adapter: Option<Arc<dyn LspAdapter>>,
) -> Language {
    let config = toml::from_slice(
        &LanguageDir::get(&format!("{}/config.toml", name))
            .unwrap()
            .data,
    )
    .unwrap();
    let mut language = Language::new(config, Some(grammar));

    if let Some(query) = load_query(name, "/highlights") {
        language = language
            .with_highlights_query(query.as_ref())
            .expect("failed to evaluate highlights query");
    }
    if let Some(query) = load_query(name, "/brackets") {
        language = language
            .with_brackets_query(query.as_ref())
            .expect("failed to load brackets query");
    }
    if let Some(query) = load_query(name, "/indents") {
        language = language
            .with_indents_query(query.as_ref())
            .expect("failed to load indents query");
    }
    if let Some(query) = load_query(name, "/outline") {
        language = language
            .with_outline_query(query.as_ref())
            .expect("failed to load outline query");
    }
    if let Some(lsp_adapter) = lsp_adapter {
        language = language.with_lsp_adapter(lsp_adapter)
    }
    language
}

fn load_query(name: &str, filename_prefix: &str) -> Option<Cow<'static, str>> {
    let mut result = None;
    for path in LanguageDir::iter() {
        if let Some(remainder) = path.strip_prefix(name) {
            if remainder.starts_with(filename_prefix) {
                let contents = match LanguageDir::get(path.as_ref()).unwrap().data {
                    Cow::Borrowed(s) => Cow::Borrowed(str::from_utf8(s).unwrap()),
                    Cow::Owned(s) => Cow::Owned(String::from_utf8(s).unwrap()),
                };
                match &mut result {
                    None => result = Some(contents),
                    Some(r) => r.to_mut().push_str(contents.as_ref()),
                }
            }
        }
    }
    result
}
