use anyhow::Context;
pub use language::*;
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
mod ruby;
mod rust;

mod typescript;

// 1. Add tree-sitter-{language} parser to zed crate
// 2. Create a language directory in zed/crates/zed/src/languages and add the language to init function below
// 3. Add config.toml to the newly created language directory using existing languages as a template
// 4. Copy highlights from tree sitter repo for the language into a highlights.scm file.
//      Note: github highlights take the last match while zed takes the first
// 5. Add indents.scm, outline.scm, and brackets.scm to implement indent on newline, outline/breadcrumbs,
//    and autoclosing brackets respectively
// 6. If the language has injections add an injections.scm query file

#[derive(RustEmbed)]
#[folder = "src/languages"]
#[exclude = "*.rs"]
struct LanguageDir;

pub fn init(languages: Arc<LanguageRegistry>) {
    for (name, grammar, lsp_adapter) in [
        (
            "c",
            tree_sitter_c::language(),
            Some(Box::new(c::CLspAdapter) as Box<dyn LspAdapter>),
        ),
        (
            "cpp",
            tree_sitter_cpp::language(),
            Some(Box::new(c::CLspAdapter)),
        ),
        (
            "css",
            tree_sitter_css::language(),
            None, //
        ),
        (
            "elixir",
            tree_sitter_elixir::language(),
            Some(Box::new(elixir::ElixirLspAdapter)),
        ),
        (
            "go",
            tree_sitter_go::language(),
            Some(Box::new(go::GoLspAdapter)),
        ),
        (
            "json",
            tree_sitter_json::language(),
            Some(Box::new(json::JsonLspAdapter)),
        ),
        (
            "markdown",
            tree_sitter_markdown::language(),
            None, //
        ),
        (
            "python",
            tree_sitter_python::language(),
            Some(Box::new(python::PythonLspAdapter)),
        ),
        (
            "rust",
            tree_sitter_rust::language(),
            Some(Box::new(rust::RustLspAdapter)),
        ),
        (
            "toml",
            tree_sitter_toml::language(),
            None, //
        ),
        (
            "tsx",
            tree_sitter_typescript::language_tsx(),
            Some(Box::new(typescript::TypeScriptLspAdapter)),
        ),
        (
            "typescript",
            tree_sitter_typescript::language_typescript(),
            Some(Box::new(typescript::TypeScriptLspAdapter)),
        ),
        (
            "javascript",
            tree_sitter_typescript::language_tsx(),
            Some(Box::new(typescript::TypeScriptLspAdapter)),
        ),
        (
            "html",
            tree_sitter_html::language(),
            Some(Box::new(html::HtmlLspAdapter)),
        ),
        (
            "ruby",
            tree_sitter_ruby::language(),
            Some(Box::new(ruby::RubyLanguageServer)),
        ),
        (
            "erb",
            tree_sitter_embedded_template::language(),
            Some(Box::new(ruby::RubyLanguageServer)),
        ),
        (
            "scheme",
            tree_sitter_scheme::language(),
            None, //
        ),
        (
            "racket",
            tree_sitter_racket::language(),
            None, //
        ),
    ] {
        languages.register(name, load_config(name), grammar, lsp_adapter, load_queries);
    }
}

#[cfg(any(test, feature = "test-support"))]
pub async fn language(
    name: &str,
    grammar: tree_sitter::Language,
    lsp_adapter: Option<Box<dyn LspAdapter>>,
) -> Arc<Language> {
    Arc::new(
        Language::new(load_config(name), Some(grammar))
            .with_lsp_adapter(lsp_adapter)
            .await
            .with_queries(load_queries(name))
            .unwrap(),
    )
}

fn load_config(name: &str) -> LanguageConfig {
    toml::from_slice(
        &LanguageDir::get(&format!("{}/config.toml", name))
            .unwrap()
            .data,
    )
    .with_context(|| format!("failed to load config.toml for language {name:?}"))
    .unwrap()
}

fn load_queries(name: &str) -> LanguageQueries {
    LanguageQueries {
        highlights: load_query(name, "/highlights"),
        brackets: load_query(name, "/brackets"),
        indents: load_query(name, "/indents"),
        outline: load_query(name, "/outline"),
        injections: load_query(name, "/injections"),
        overrides: load_query(name, "/overrides"),
    }
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
