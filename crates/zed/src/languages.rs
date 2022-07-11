use gpui::executor::Background;
pub use language::*;
use rust_embed::RustEmbed;
use std::{borrow::Cow, str, sync::Arc};
use util::ResultExt;

mod c;
mod go;
mod installation;
mod json;
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
            Some(LspAdapter::new(c::CLspAdapter).await),
        ),
        (
            "cpp",
            tree_sitter_cpp::language(),
            Some(LspAdapter::new(c::CLspAdapter).await),
        ),
        (
            "go",
            tree_sitter_go::language(),
            Some(LspAdapter::new(go::GoLspAdapter).await),
        ),
        (
            "json",
            tree_sitter_json::language(),
            // Some(LspAdapter::new(json::JsonLspAdapter).await),
            match language_plugin::new_json(executor).await.log_err() {
                Some(lang) => Some(LspAdapter::new(lang).await),
                None => None,
            },
        ),
        (
            "markdown",
            tree_sitter_markdown::language(),
            None, //
        ),
        (
            "python",
            tree_sitter_python::language(),
            Some(LspAdapter::new(python::PythonLspAdapter).await),
        ),
        (
            "rust",
            tree_sitter_rust::language(),
            Some(LspAdapter::new(rust::RustLspAdapter).await),
        ),
        (
            "toml",
            tree_sitter_toml::language(),
            None, //
        ),
        (
            "tsx",
            tree_sitter_typescript::language_tsx(),
            Some(LspAdapter::new(typescript::TypeScriptLspAdapter).await),
        ),
        (
            "typescript",
            tree_sitter_typescript::language_typescript(),
            Some(LspAdapter::new(typescript::TypeScriptLspAdapter).await),
        ),
        (
            "javascript",
            tree_sitter_typescript::language_tsx(),
            Some(LspAdapter::new(typescript::TypeScriptLspAdapter).await),
        ),
    ] {
        languages.add(Arc::new(language(name, grammar, lsp_adapter)));
    }
}

pub(crate) fn language(
    name: &str,
    grammar: tree_sitter::Language,
    lsp_adapter: Option<Arc<LspAdapter>>,
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
