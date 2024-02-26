use anyhow::Context;
use gpui::AppContext;
pub use language::*;
use node_runtime::NodeRuntime;
use rust_embed::RustEmbed;
use settings::Settings;
use std::{str, sync::Arc};
use util::asset_str;

use self::{deno::DenoSettings, elixir::ElixirSettings};

mod astro;
mod c;
mod clojure;
mod csharp;
mod css;
mod dart;
mod deno;
mod dockerfile;
mod elixir;
mod elm;
mod erlang;
mod gleam;
mod go;
mod haskell;
mod html;
mod json;
mod lua;
mod nu;
mod ocaml;
mod php;
mod prisma;
mod purescript;
mod python;
mod ruby;
mod rust;
mod svelte;
mod tailwind;
mod toml;
mod typescript;
mod uiua;
mod vue;
mod yaml;
mod zig;

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
        ("astro", tree_sitter_astro::language()),
        ("bash", tree_sitter_bash::language()),
        ("c", tree_sitter_c::language()),
        ("c_sharp", tree_sitter_c_sharp::language()),
        ("clojure", tree_sitter_clojure::language()),
        ("cpp", tree_sitter_cpp::language()),
        ("css", tree_sitter_css::language()),
        ("dockerfile", tree_sitter_dockerfile::language()),
        ("elixir", tree_sitter_elixir::language()),
        ("elm", tree_sitter_elm::language()),
        (
            "embedded_template",
            tree_sitter_embedded_template::language(),
        ),
        ("erlang", tree_sitter_erlang::language()),
        ("git_commit", tree_sitter_gitcommit::language()),
        ("gleam", tree_sitter_gleam::language()),
        ("glsl", tree_sitter_glsl::language()),
        ("go", tree_sitter_go::language()),
        ("gomod", tree_sitter_gomod::language()),
        ("gowork", tree_sitter_gowork::language()),
        ("haskell", tree_sitter_haskell::language()),
        ("hcl", tree_sitter_hcl::language()),
        ("heex", tree_sitter_heex::language()),
        ("html", tree_sitter_html::language()),
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
        ("prisma", tree_sitter_prisma_io::language()),
        ("proto", tree_sitter_proto::language()),
        ("purescript", tree_sitter_purescript::language()),
        ("python", tree_sitter_python::language()),
        ("racket", tree_sitter_racket::language()),
        ("ruby", tree_sitter_ruby::language()),
        ("rust", tree_sitter_rust::language()),
        ("scheme", tree_sitter_scheme::language()),
        ("svelte", tree_sitter_svelte::language()),
        ("toml", tree_sitter_toml::language()),
        ("tsx", tree_sitter_typescript::language_tsx()),
        ("typescript", tree_sitter_typescript::language_typescript()),
        ("uiua", tree_sitter_uiua::language()),
        ("vue", tree_sitter_vue::language()),
        ("yaml", tree_sitter_yaml::language()),
        ("zig", tree_sitter_zig::language()),
        ("dart", tree_sitter_dart::language()),
    ]);

    let language = |asset_dir_name: &'static str, adapters| {
        let config = load_config(asset_dir_name);
        languages.register_language(
            config.name.clone(),
            config.grammar.clone(),
            config.matcher.clone(),
            adapters,
            move || Ok((config.clone(), load_queries(asset_dir_name))),
        )
    };

    language(
        "astro",
        vec![
            Arc::new(astro::AstroLspAdapter::new(node_runtime.clone())),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ],
    );
    language("bash", vec![]);
    language("c", vec![Arc::new(c::CLspAdapter) as Arc<dyn LspAdapter>]);
    language("clojure", vec![Arc::new(clojure::ClojureLspAdapter)]);
    language("cpp", vec![Arc::new(c::CLspAdapter)]);
    language("csharp", vec![Arc::new(csharp::OmniSharpAdapter {})]);
    language(
        "css",
        vec![
            Arc::new(css::CssLspAdapter::new(node_runtime.clone())),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ],
    );

    language(
        "dockerfile",
        vec![Arc::new(dockerfile::DockerfileLspAdapter::new(
            node_runtime.clone(),
        ))],
    );

    match &ElixirSettings::get(None, cx).lsp {
        elixir::ElixirLspSetting::ElixirLs => language(
            "elixir",
            vec![
                Arc::new(elixir::ElixirLspAdapter),
                Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
            ],
        ),
        elixir::ElixirLspSetting::NextLs => {
            language("elixir", vec![Arc::new(elixir::NextLspAdapter)])
        }
        elixir::ElixirLspSetting::Local { path, arguments } => language(
            "elixir",
            vec![Arc::new(elixir::LocalLspAdapter {
                path: path.clone(),
                arguments: arguments.clone(),
            })],
        ),
    }
    language("gitcommit", vec![]);
    language("erlang", vec![Arc::new(erlang::ErlangLspAdapter)]);

    language("gleam", vec![Arc::new(gleam::GleamLspAdapter)]);
    language("go", vec![Arc::new(go::GoLspAdapter)]);
    language("gomod", vec![]);
    language("gowork", vec![]);
    language("zig", vec![Arc::new(zig::ZlsAdapter)]);
    language(
        "heex",
        vec![
            Arc::new(elixir::ElixirLspAdapter),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ],
    );
    language(
        "json",
        vec![Arc::new(json::JsonLspAdapter::new(
            node_runtime.clone(),
            languages.clone(),
        ))],
    );
    language("markdown", vec![]);
    language(
        "python",
        vec![Arc::new(python::PythonLspAdapter::new(
            node_runtime.clone(),
        ))],
    );
    language("rust", vec![Arc::new(rust::RustLspAdapter)]);
    language("toml", vec![Arc::new(toml::TaploLspAdapter)]);
    match &DenoSettings::get(None, cx).enable {
        true => {
            language(
                "tsx",
                vec![
                    Arc::new(deno::DenoLspAdapter::new()),
                    Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
                ],
            );
            language("typescript", vec![Arc::new(deno::DenoLspAdapter::new())]);
            language(
                "javascript",
                vec![
                    Arc::new(deno::DenoLspAdapter::new()),
                    Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
                ],
            );
        }
        false => {
            language(
                "tsx",
                vec![
                    Arc::new(typescript::TypeScriptLspAdapter::new(node_runtime.clone())),
                    Arc::new(typescript::EsLintLspAdapter::new(node_runtime.clone())),
                    Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
                ],
            );
            language(
                "typescript",
                vec![
                    Arc::new(typescript::TypeScriptLspAdapter::new(node_runtime.clone())),
                    Arc::new(typescript::EsLintLspAdapter::new(node_runtime.clone())),
                ],
            );
            language(
                "javascript",
                vec![
                    Arc::new(typescript::TypeScriptLspAdapter::new(node_runtime.clone())),
                    Arc::new(typescript::EsLintLspAdapter::new(node_runtime.clone())),
                    Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
                ],
            );
        }
    }

    language("haskell", vec![Arc::new(haskell::HaskellLanguageServer {})]);
    language(
        "html",
        vec![
            Arc::new(html::HtmlLspAdapter::new(node_runtime.clone())),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ],
    );
    language("ruby", vec![Arc::new(ruby::RubyLanguageServer)]);
    language(
        "erb",
        vec![
            Arc::new(ruby::RubyLanguageServer),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ],
    );
    language("scheme", vec![]);
    language("racket", vec![]);
    language("lua", vec![Arc::new(lua::LuaLspAdapter)]);
    language(
        "yaml",
        vec![Arc::new(yaml::YamlLspAdapter::new(node_runtime.clone()))],
    );
    language(
        "svelte",
        vec![
            Arc::new(svelte::SvelteLspAdapter::new(node_runtime.clone())),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ],
    );
    language(
        "php",
        vec![
            Arc::new(php::IntelephenseLspAdapter::new(node_runtime.clone())),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ],
    );
    language(
        "purescript",
        vec![Arc::new(purescript::PurescriptLspAdapter::new(
            node_runtime.clone(),
        ))],
    );
    language(
        "elm",
        vec![Arc::new(elm::ElmLspAdapter::new(node_runtime.clone()))],
    );
    language("glsl", vec![]);
    language("nix", vec![]);
    language("nu", vec![Arc::new(nu::NuLanguageServer {})]);
    language("ocaml", vec![Arc::new(ocaml::OCamlLspAdapter)]);
    language("ocaml-interface", vec![Arc::new(ocaml::OCamlLspAdapter)]);
    language(
        "vue",
        vec![Arc::new(vue::VueLspAdapter::new(node_runtime.clone()))],
    );
    language("uiua", vec![Arc::new(uiua::UiuaLanguageServer {})]);
    language("proto", vec![]);
    language("terraform", vec![]);
    language("terraform-vars", vec![]);
    language("hcl", vec![]);
    language(
        "prisma",
        vec![Arc::new(prisma::PrismaLspAdapter::new(
            node_runtime.clone(),
        ))],
    );
    language("dart", vec![Arc::new(dart::DartLanguageServer {})]);
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
