use anyhow::Context;
pub use language::*;
use node_runtime::NodeRuntime;
use rust_embed::RustEmbed;
use std::{borrow::Cow, str, sync::Arc};
use util::asset_str;

mod c;
mod elixir;
mod go;
mod html;
mod json;
#[cfg(feature = "plugin_runtime")]
mod language_plugin;
mod lua;
mod php;
mod python;
mod ruby;
mod rust;
mod svelte;
mod tailwind;
mod typescript;
mod yaml;

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

pub fn init(languages: Arc<LanguageRegistry>, node_runtime: Arc<NodeRuntime>) {
    let language = |name, grammar, adapters| {
        languages.register(name, load_config(name), grammar, adapters, load_queries)
    };

    language("bash", tree_sitter_bash::language(), vec![]);
    language(
        "c",
        tree_sitter_c::language(),
        vec![Arc::new(c::CLspAdapter) as Arc<dyn LspAdapter>],
    );
    language(
        "cpp",
        tree_sitter_cpp::language(),
        vec![Arc::new(c::CLspAdapter)],
    );
    language("css", tree_sitter_css::language(), vec![]);
    language(
        "elixir",
        tree_sitter_elixir::language(),
        vec![Arc::new(elixir::ElixirLspAdapter)],
    );
    language(
        "go",
        tree_sitter_go::language(),
        vec![Arc::new(go::GoLspAdapter)],
    );
    language(
        "heex",
        tree_sitter_heex::language(),
        vec![Arc::new(elixir::ElixirLspAdapter)],
    );
    language(
        "json",
        tree_sitter_json::language(),
        vec![Arc::new(json::JsonLspAdapter::new(
            node_runtime.clone(),
            languages.clone(),
        ))],
    );
    language("markdown", tree_sitter_markdown::language(), vec![]);
    language(
        "python",
        tree_sitter_python::language(),
        vec![Arc::new(python::PythonLspAdapter::new(
            node_runtime.clone(),
        ))],
    );
    language(
        "rust",
        tree_sitter_rust::language(),
        vec![Arc::new(rust::RustLspAdapter)],
    );
    language("toml", tree_sitter_toml::language(), vec![]);
    language(
        "tsx",
        tree_sitter_typescript::language_tsx(),
        vec![
            Arc::new(typescript::TypeScriptLspAdapter::new(node_runtime.clone())),
            Arc::new(typescript::EsLintLspAdapter::new(node_runtime.clone())),
        ],
    );
    language(
        "typescript",
        tree_sitter_typescript::language_typescript(),
        vec![
            Arc::new(typescript::TypeScriptLspAdapter::new(node_runtime.clone())),
            Arc::new(typescript::EsLintLspAdapter::new(node_runtime.clone())),
        ],
    );
    language(
        "javascript",
        tree_sitter_typescript::language_tsx(),
        vec![
            Arc::new(typescript::TypeScriptLspAdapter::new(node_runtime.clone())),
            Arc::new(typescript::EsLintLspAdapter::new(node_runtime.clone())),
        ],
    );
    language(
        "html",
        tree_sitter_html::language(),
        vec![
            Arc::new(html::HtmlLspAdapter::new(node_runtime.clone())),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ],
    );
    language(
        "ruby",
        tree_sitter_ruby::language(),
        vec![Arc::new(ruby::RubyLanguageServer)],
    );
    language(
        "erb",
        tree_sitter_embedded_template::language(),
        vec![Arc::new(ruby::RubyLanguageServer)],
    );
    language("scheme", tree_sitter_scheme::language(), vec![]);
    language("racket", tree_sitter_racket::language(), vec![]);
    language(
        "lua",
        tree_sitter_lua::language(),
        vec![Arc::new(lua::LuaLspAdapter)],
    );
    language(
        "yaml",
        tree_sitter_yaml::language(),
        vec![Arc::new(yaml::YamlLspAdapter::new(node_runtime.clone()))],
    );
    language(
        "svelte",
        tree_sitter_svelte::language(),
        vec![Arc::new(svelte::SvelteLspAdapter::new(
            node_runtime.clone(),
        ))],
    );
    language(
        "php",
        tree_sitter_php::language(),
        vec![Arc::new(php::IntelephenseLspAdapter::new(node_runtime))],
    );

    language("elm", tree_sitter_elm::language(), vec![]);
    language("glsl", tree_sitter_glsl::language(), vec![]);
    language("nix", tree_sitter_nix::language(), vec![]);
}

#[cfg(any(test, feature = "test-support"))]
pub async fn language(
    name: &str,
    grammar: tree_sitter::Language,
    lsp_adapter: Option<Arc<dyn LspAdapter>>,
) -> Arc<Language> {
    Arc::new(
        Language::new(load_config(name), Some(grammar))
            .with_lsp_adapters(lsp_adapter.into_iter().collect())
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
        embedding: load_query(name, "/embedding"),
        injections: load_query(name, "/injections"),
        overrides: load_query(name, "/overrides"),
    }
}

fn load_query(name: &str, filename_prefix: &str) -> Option<Cow<'static, str>> {
    let mut result = None;
    for path in LanguageDir::iter() {
        if let Some(remainder) = path.strip_prefix(name) {
            if remainder.starts_with(filename_prefix) {
                let contents = asset_str::<LanguageDir>(path.as_ref());
                match &mut result {
                    None => result = Some(contents),
                    Some(r) => r.to_mut().push_str(contents.as_ref()),
                }
            }
        }
    }
    result
}
