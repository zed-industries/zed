use anyhow::Context;
use gpui::AppContext;
pub use language::*;
use node_runtime::NodeRuntime;
use rust_embed::RustEmbed;
use settings::Settings;
use std::{borrow::Cow, fs, path::Path, str, sync::Arc};
use util::{asset_str, paths::PLUGINS_DIR, ResultExt};

use self::{deno::DenoSettings, elixir::ElixirSettings};

mod c;
mod csharp;
mod css;
mod deno;
mod elixir;
mod elm;
mod erlang;
mod gleam;
mod go;
mod haskell;
mod html;
mod json;
#[cfg(feature = "plugin_runtime")]
mod language_plugin;
mod lua;
mod nu;
mod php;
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
#[folder = "src/languages"]
#[exclude = "*.rs"]
struct LanguageDir;

pub fn init(
    languages: Arc<LanguageRegistry>,
    node_runtime: Arc<dyn NodeRuntime>,
    cx: &mut AppContext,
) {
    ElixirSettings::register(cx);
    DenoSettings::register(cx);

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
    language(
        "csharp",
        tree_sitter_c_sharp::language(),
        vec![Arc::new(csharp::OmniSharpAdapter {})],
    );
    language(
        "css",
        tree_sitter_css::language(),
        vec![
            Arc::new(css::CssLspAdapter::new(node_runtime.clone())),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ],
    );

    match &ElixirSettings::get(None, cx).lsp {
        elixir::ElixirLspSetting::ElixirLs => language(
            "elixir",
            tree_sitter_elixir::language(),
            vec![
                Arc::new(elixir::ElixirLspAdapter),
                Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
            ],
        ),
        elixir::ElixirLspSetting::NextLs => language(
            "elixir",
            tree_sitter_elixir::language(),
            vec![Arc::new(elixir::NextLspAdapter)],
        ),
        elixir::ElixirLspSetting::Local { path, arguments } => language(
            "elixir",
            tree_sitter_elixir::language(),
            vec![Arc::new(elixir::LocalLspAdapter {
                path: path.clone(),
                arguments: arguments.clone(),
            })],
        ),
    }
    language("gitcommit", tree_sitter_gitcommit::language(), vec![]);
    language(
        "erlang",
        tree_sitter_erlang::language(),
        vec![Arc::new(erlang::ErlangLspAdapter)],
    );

    language(
        "gleam",
        tree_sitter_gleam::language(),
        vec![Arc::new(gleam::GleamLspAdapter)],
    );
    language(
        "go",
        tree_sitter_go::language(),
        vec![Arc::new(go::GoLspAdapter)],
    );
    language("gomod", tree_sitter_gomod::language(), vec![]);
    language("gowork", tree_sitter_gowork::language(), vec![]);
    language(
        "zig",
        tree_sitter_zig::language(),
        vec![Arc::new(zig::ZlsAdapter)],
    );
    language(
        "heex",
        tree_sitter_heex::language(),
        vec![
            Arc::new(elixir::ElixirLspAdapter),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ],
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
    language(
        "toml",
        tree_sitter_toml::language(),
        vec![Arc::new(toml::TaploLspAdapter)],
    );
    match &DenoSettings::get(None, cx).enable {
        true => {
            language(
                "tsx",
                tree_sitter_typescript::language_tsx(),
                vec![
                    Arc::new(deno::DenoLspAdapter::new()),
                    Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
                ],
            );
            language(
                "typescript",
                tree_sitter_typescript::language_typescript(),
                vec![Arc::new(deno::DenoLspAdapter::new())],
            );
            language(
                "javascript",
                tree_sitter_typescript::language_tsx(),
                vec![
                    Arc::new(deno::DenoLspAdapter::new()),
                    Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
                ],
            );
        }
        false => {
            language(
                "tsx",
                tree_sitter_typescript::language_tsx(),
                vec![
                    Arc::new(typescript::TypeScriptLspAdapter::new(node_runtime.clone())),
                    Arc::new(typescript::EsLintLspAdapter::new(node_runtime.clone())),
                    Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
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
                    Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
                ],
            );
        }
    }

    language(
        "haskell",
        tree_sitter_haskell::language(),
        vec![Arc::new(haskell::HaskellLanguageServer {})],
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
        vec![
            Arc::new(ruby::RubyLanguageServer),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ],
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
        vec![
            Arc::new(svelte::SvelteLspAdapter::new(node_runtime.clone())),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ],
    );
    language(
        "php",
        tree_sitter_php::language_php(),
        vec![
            Arc::new(php::IntelephenseLspAdapter::new(node_runtime.clone())),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        ],
    );

    language(
        "purescript",
        tree_sitter_purescript::language(),
        vec![Arc::new(purescript::PurescriptLspAdapter::new(
            node_runtime.clone(),
        ))],
    );
    language(
        "elm",
        tree_sitter_elm::language(),
        vec![Arc::new(elm::ElmLspAdapter::new(node_runtime.clone()))],
    );
    language("glsl", tree_sitter_glsl::language(), vec![]);
    language("nix", tree_sitter_nix::language(), vec![]);
    language(
        "nu",
        tree_sitter_nu::language(),
        vec![Arc::new(nu::NuLanguageServer {})],
    );
    language(
        "vue",
        tree_sitter_vue::language(),
        vec![Arc::new(vue::VueLspAdapter::new(node_runtime))],
    );
    language(
        "uiua",
        tree_sitter_uiua::language(),
        vec![Arc::new(uiua::UiuaLanguageServer {})],
    );
    language("proto", tree_sitter_proto::language(), vec![]);

    if let Ok(children) = std::fs::read_dir(&*PLUGINS_DIR) {
        for child in children {
            if let Ok(child) = child {
                let path = child.path();
                let config_path = path.join("config.toml");
                if let Ok(config) = std::fs::read(&config_path) {
                    languages.register_wasm(
                        path.into(),
                        ::toml::from_slice(&config).unwrap(),
                        load_plugin_queries,
                    );
                }
            }
        }
    }
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
    ::toml::from_slice(
        &LanguageDir::get(&format!("{}/config.toml", name))
            .unwrap()
            .data,
    )
    .with_context(|| format!("failed to load config.toml for language {name:?}"))
    .unwrap()
}

const QUERY_FILENAME_PREFIXES: &[(
    &str,
    fn(&mut LanguageQueries) -> &mut Option<Cow<'static, str>>,
)] = &[
    ("highlights", |q| &mut q.highlights),
    ("brackets", |q| &mut q.brackets),
    ("outline", |q| &mut q.outline),
    ("indents", |q| &mut q.indents),
    ("embedding", |q| &mut q.embedding),
    ("injections", |q| &mut q.injections),
    ("overrides", |q| &mut q.overrides),
    ("redactions", |q| &mut q.redactions),
];

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

fn load_plugin_queries(root_path: &Path) -> LanguageQueries {
    let mut result = LanguageQueries::default();
    if let Some(entries) = fs::read_dir(root_path).log_err() {
        for entry in entries {
            let Some(entry) = entry.log_err() else {
                continue;
            };
            let path = entry.path();
            if let Some(remainder) = path.strip_prefix(root_path).ok().and_then(|p| p.to_str()) {
                if !remainder.ends_with(".scm") {
                    continue;
                }
                for (name, query) in QUERY_FILENAME_PREFIXES {
                    if remainder.starts_with(name) {
                        if let Some(contents) = fs::read_to_string(&path).log_err() {
                            match query(&mut result) {
                                None => *query(&mut result) = Some(contents.into()),
                                Some(r) => r.to_mut().push_str(contents.as_ref()),
                            }
                        }
                        break;
                    }
                }
            }
        }
    }
    result
}
