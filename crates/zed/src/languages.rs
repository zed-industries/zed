use gpui::executor::Background;
pub use language::*;
use lazy_static::lazy_static;
use rust_embed::RustEmbed;
use std::{borrow::Cow, str, sync::Arc};

mod c;
mod elixir;
mod go;
mod html;
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

// TODO - Remove this once the `init` function is synchronous again.
lazy_static! {
    pub static ref LANGUAGE_NAMES: Vec<String> = LanguageDir::iter()
        .filter_map(|path| {
            if path.ends_with("config.toml") {
                let config = LanguageDir::get(&path)?;
                let config = toml::from_slice::<LanguageConfig>(&config.data).ok()?;
                Some(config.name.to_string())
            } else {
                None
            }
        })
        .collect();
}

pub async fn init(languages: Arc<LanguageRegistry>, _executor: Arc<Background>) {
    for (name, grammar, lsp_adapter) in [
        (
            "c",
            tree_sitter_c::language(),
            Some(CachedLspAdapter::new(c::CLspAdapter).await),
        ),
        (
            "cpp",
            tree_sitter_cpp::language(),
            Some(CachedLspAdapter::new(c::CLspAdapter).await),
        ),
        (
            "css",
            tree_sitter_css::language(),
            None, //
        ),
        (
            "elixir",
            tree_sitter_elixir::language(),
            Some(CachedLspAdapter::new(elixir::ElixirLspAdapter).await),
        ),
        (
            "go",
            tree_sitter_go::language(),
            Some(CachedLspAdapter::new(go::GoLspAdapter).await),
        ),
        (
            "json",
            tree_sitter_json::language(),
            Some(CachedLspAdapter::new(json::JsonLspAdapter).await),
        ),
        (
            "markdown",
            tree_sitter_markdown::language(),
            None, //
        ),
        (
            "python",
            tree_sitter_python::language(),
            Some(CachedLspAdapter::new(python::PythonLspAdapter).await),
        ),
        (
            "rust",
            tree_sitter_rust::language(),
            Some(CachedLspAdapter::new(rust::RustLspAdapter).await),
        ),
        (
            "toml",
            tree_sitter_toml::language(),
            None, //
        ),
        (
            "tsx",
            tree_sitter_typescript::language_tsx(),
            Some(CachedLspAdapter::new(typescript::TypeScriptLspAdapter).await),
        ),
        (
            "typescript",
            tree_sitter_typescript::language_typescript(),
            Some(CachedLspAdapter::new(typescript::TypeScriptLspAdapter).await),
        ),
        (
            "javascript",
            tree_sitter_typescript::language_tsx(),
            Some(CachedLspAdapter::new(typescript::TypeScriptLspAdapter).await),
        ),
        (
            "html",
            tree_sitter_html::language(),
            Some(CachedLspAdapter::new(html::HtmlLspAdapter).await),
        ),
    ] {
        languages.add(language(name, grammar, lsp_adapter));
    }
}

pub(crate) fn language(
    name: &str,
    grammar: tree_sitter::Language,
    lsp_adapter: Option<Arc<CachedLspAdapter>>,
) -> Arc<Language> {
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
    if let Some(query) = load_query(name, "/injections") {
        language = language
            .with_injection_query(query.as_ref())
            .expect("failed to load injection query");
    }
    if let Some(lsp_adapter) = lsp_adapter {
        language = language.with_lsp_adapter(lsp_adapter)
    }
    Arc::new(language)
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
