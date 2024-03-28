use anyhow::Context;
use gpui::AppContext;
pub use language::*;
use node_runtime::NodeRuntime;
use rust_embed::RustEmbed;
use settings::Settings;
use std::{str, sync::Arc};
use util::asset_str;

use crate::{elixir::elixir_task_context, rust::RustContextProvider};

use self::{deno::DenoSettings, elixir::ElixirSettings};

mod c;
mod clojure;
mod csharp;
mod css;
mod dart;
mod deno;
mod elixir;
mod elm;
mod erlang;
mod go;
mod html;
mod json;
mod lua;
mod nu;
mod ocaml;
mod php;
mod python;
mod ruby;
mod rust;
mod tailwind;
mod terraform;
mod toml;
mod typescript;
mod vue;
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
#[folder = "src/"]
#[exclude = "*.rs"]
struct LanguageDir;

pub fn init(
    languages: Arc<LanguageRegistry>,
    node_runtime: Arc<dyn NodeRuntime>,
    cx: &mut AppContext,
) {
    ElixirSettings::register(cx);
    DenoSettings::register(cx);

    languages.register_native_grammars([
        ("bash", tree_sitter_bash::language()),
        ("c", tree_sitter_c::language()),
        ("c_sharp", tree_sitter_c_sharp::language()),
        ("clojure", tree_sitter_clojure::language()),
        ("cpp", tree_sitter_cpp::language()),
        ("css", tree_sitter_css::language()),
        ("elixir", tree_sitter_elixir::language()),
        ("elm", tree_sitter_elm::language()),
        (
            "embedded_template",
            tree_sitter_embedded_template::language(),
        ),
        ("erlang", tree_sitter_erlang::language()),
        ("glsl", tree_sitter_glsl::language()),
        ("go", tree_sitter_go::language()),
        ("gomod", tree_sitter_gomod::language()),
        ("gowork", tree_sitter_gowork::language()),
        ("hcl", tree_sitter_hcl::language()),
        ("heex", tree_sitter_heex::language()),
        ("html", tree_sitter_html::language()),
        ("jsdoc", tree_sitter_jsdoc::language()),
        ("json", tree_sitter_json::language()),
        ("lua", tree_sitter_lua::language()),
        ("markdown", tree_sitter_markdown::language()),
        ("nix", tree_sitter_nix::language()),
        ("nu", tree_sitter_nu::language()),
        ("ocaml", tree_sitter_ocaml::language_ocaml()),
        (
            "ocaml_interface",
            tree_sitter_ocaml::language_ocaml_interface(),
        ),
        ("php", tree_sitter_php::language_php()),
        ("proto", tree_sitter_proto::language()),
        ("python", tree_sitter_python::language()),
        ("racket", tree_sitter_racket::language()),
        ("regex", tree_sitter_regex::language()),
        ("ruby", tree_sitter_ruby::language()),
        ("rust", tree_sitter_rust::language()),
        ("scheme", tree_sitter_scheme::language()),
        ("toml", tree_sitter_toml::language()),
        ("tsx", tree_sitter_typescript::language_tsx()),
        ("typescript", tree_sitter_typescript::language_typescript()),
        ("vue", tree_sitter_vue::language()),
        ("yaml", tree_sitter_yaml::language()),
        ("dart", tree_sitter_dart::language()),
    ]);

    macro_rules! language {
        ($name:literal) => {
            let config = load_config($name);
            languages.register_language(
                config.name.clone(),
                config.grammar.clone(),
                config.matcher.clone(),
                move || {
                    Ok((
                        config.clone(),
                        load_queries($name),
                        Some(Arc::new(language::SymbolContextProvider)),
                    ))
                },
            );
        };
        ($name:literal, $adapters:expr) => {
            let config = load_config($name);
            // typeck helper
            let adapters: Vec<Arc<dyn LspAdapter>> = $adapters;
            for adapter in adapters {
                languages.register_lsp_adapter(config.name.clone(), adapter);
            }
            languages.register_language(
                config.name.clone(),
                config.grammar.clone(),
                config.matcher.clone(),
                move || {
                    Ok((
                        config.clone(),
                        load_queries($name),
                        Some(Arc::new(language::SymbolContextProvider)),
                    ))
                },
            );
        };
        ($name:literal, $adapters:expr, $context_provider:expr) => {
            let config = load_config($name);
            // typeck helper
            let adapters: Vec<Arc<dyn LspAdapter>> = $adapters;
            for adapter in adapters {
                languages.register_lsp_adapter(config.name.clone(), adapter);
            }
            languages.register_language(
                config.name.clone(),
                config.grammar.clone(),
                config.matcher.clone(),
                move || {
                    Ok((
                        config.clone(),
                        load_queries($name),
                        Some(Arc::new($context_provider)),
                    ))
                },
            );
        };
    }
    language!("bash");
    language!("c", vec![Arc::new(c::CLspAdapter) as Arc<dyn LspAdapter>]);
    language!("clojure", vec![Arc::new(clojure::ClojureLspAdapter)]);
    language!("cpp", vec![Arc::new(c::CLspAdapter)]);
    language!("csharp", vec![Arc::new(csharp::OmniSharpAdapter {})]);
    language!(
        "css",
        vec![
            Arc::new(css::CssLspAdapter::new(node_runtime.clone())),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ]
    );

    match &ElixirSettings::get(None, cx).lsp {
        elixir::ElixirLspSetting::ElixirLs => {
            language!(
                "elixir",
                vec![
                    Arc::new(elixir::ElixirLspAdapter),
                    Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
                ],
                elixir_task_context()
            );
        }
        elixir::ElixirLspSetting::NextLs => {
            language!(
                "elixir",
                vec![Arc::new(elixir::NextLspAdapter)],
                elixir_task_context()
            );
        }
        elixir::ElixirLspSetting::Local { path, arguments } => {
            language!(
                "elixir",
                vec![Arc::new(elixir::LocalLspAdapter {
                    path: path.clone(),
                    arguments: arguments.clone(),
                })],
                elixir_task_context()
            );
        }
    }
    language!("erlang", vec![Arc::new(erlang::ErlangLspAdapter)]);
    language!("go", vec![Arc::new(go::GoLspAdapter)]);
    language!("gomod");
    language!("gowork");
    language!(
        "heex",
        vec![
            Arc::new(elixir::ElixirLspAdapter),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ]
    );
    language!(
        "json",
        vec![Arc::new(json::JsonLspAdapter::new(
            node_runtime.clone(),
            languages.clone(),
        ))]
    );
    language!("markdown");
    language!(
        "python",
        vec![Arc::new(python::PythonLspAdapter::new(
            node_runtime.clone(),
        ))]
    );
    language!(
        "rust",
        vec![Arc::new(rust::RustLspAdapter)],
        RustContextProvider
    );
    language!("toml", vec![Arc::new(toml::TaploLspAdapter)]);
    match &DenoSettings::get(None, cx).enable {
        true => {
            language!(
                "tsx",
                vec![
                    Arc::new(deno::DenoLspAdapter::new()),
                    Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
                ]
            );
            language!("typescript", vec![Arc::new(deno::DenoLspAdapter::new())]);
            language!(
                "javascript",
                vec![
                    Arc::new(deno::DenoLspAdapter::new()),
                    Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
                ]
            );
            language!("jsdoc", vec![Arc::new(deno::DenoLspAdapter::new())]);
        }
        false => {
            language!(
                "tsx",
                vec![
                    Arc::new(typescript::TypeScriptLspAdapter::new(node_runtime.clone())),
                    Arc::new(typescript::EsLintLspAdapter::new(node_runtime.clone())),
                    Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
                ]
            );
            language!(
                "typescript",
                vec![
                    Arc::new(typescript::TypeScriptLspAdapter::new(node_runtime.clone())),
                    Arc::new(typescript::EsLintLspAdapter::new(node_runtime.clone())),
                ]
            );
            language!(
                "javascript",
                vec![
                    Arc::new(typescript::TypeScriptLspAdapter::new(node_runtime.clone())),
                    Arc::new(typescript::EsLintLspAdapter::new(node_runtime.clone())),
                    Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
                ]
            );
            language!(
                "jsdoc",
                vec![Arc::new(typescript::TypeScriptLspAdapter::new(
                    node_runtime.clone(),
                ))]
            );
        }
    }

    language!(
        "html",
        vec![
            Arc::new(html::HtmlLspAdapter::new(node_runtime.clone())),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ]
    );
    language!("ruby", vec![Arc::new(ruby::RubyLanguageServer)]);
    language!(
        "erb",
        vec![
            Arc::new(ruby::RubyLanguageServer),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ]
    );
    language!("scheme");
    language!("racket");
    language!("regex");
    language!("lua", vec![Arc::new(lua::LuaLspAdapter)]);
    language!(
        "yaml",
        vec![Arc::new(yaml::YamlLspAdapter::new(node_runtime.clone()))]
    );
    language!(
        "php",
        vec![
            Arc::new(php::IntelephenseLspAdapter::new(node_runtime.clone())),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ]
    );
    language!(
        "elm",
        vec![Arc::new(elm::ElmLspAdapter::new(node_runtime.clone()))]
    );
    language!("glsl");
    language!("nix");
    language!("nu", vec![Arc::new(nu::NuLanguageServer {})]);
    language!("ocaml", vec![Arc::new(ocaml::OCamlLspAdapter)]);
    language!("ocaml-interface", vec![Arc::new(ocaml::OCamlLspAdapter)]);
    language!(
        "vue",
        vec![Arc::new(vue::VueLspAdapter::new(node_runtime.clone()))]
    );
    language!("proto");
    language!("terraform", vec![Arc::new(terraform::TerraformLspAdapter)]);
    language!(
        "terraform-vars",
        vec![Arc::new(terraform::TerraformLspAdapter)]
    );
    language!("hcl", vec![]);
    language!("dart", vec![Arc::new(dart::DartLanguageServer {})]);

    languages.register_secondary_lsp_adapter(
        "Astro".into(),
        Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
    );
    languages.register_secondary_lsp_adapter(
        "Svelte".into(),
        Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
    );
}

#[cfg(any(test, feature = "test-support"))]
pub fn language(name: &str, grammar: tree_sitter::Language) -> Arc<Language> {
    Arc::new(
        Language::new(load_config(name), Some(grammar))
            .with_queries(load_queries(name))
            .unwrap(),
    )
}

fn load_config(name: &str) -> LanguageConfig {
    let config_toml = String::from_utf8(
        LanguageDir::get(&format!("{}/config.toml", name))
            .unwrap()
            .data
            .to_vec(),
    )
    .unwrap();

    ::toml::from_str(&config_toml)
        .with_context(|| format!("failed to load config.toml for language {name:?}"))
        .unwrap()
}

fn load_queries(name: &str) -> LanguageQueries {
    let mut result = LanguageQueries::default();
    for path in LanguageDir::iter() {
        if let Some(remainder) = path.strip_prefix(name).and_then(|p| p.strip_prefix('/')) {
            if !remainder.ends_with(".scm") {
                continue;
            }
            for (name, query) in QUERY_FILENAME_PREFIXES {
                if remainder.starts_with(name) {
                    let contents = asset_str::<LanguageDir>(path.as_ref());
                    match query(&mut result) {
                        None => *query(&mut result) = Some(contents),
                        Some(r) => r.to_mut().push_str(contents.as_ref()),
                    }
                }
            }
        }
    }
    result
}
