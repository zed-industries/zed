use anyhow::Context;
use gpui::{AppContext, UpdateGlobal};
use json::json_task_context;
pub use language::*;
use node_runtime::NodeRuntime;
use python::{PythonContextProvider, PythonToolchainProvider};
use rust_embed::RustEmbed;
use settings::SettingsStore;
use smol::stream::StreamExt;
use std::{str, sync::Arc};
use typescript::typescript_task_context;
use util::{asset_str, ResultExt};

use crate::{bash::bash_task_context, go::GoContextProvider, rust::RustContextProvider};

mod bash;
mod c;
mod css;
mod go;
mod json;
mod python;
mod rust;
mod tailwind;
mod typescript;
mod vtsls;
mod yaml;

#[derive(RustEmbed)]
#[folder = "src/"]
#[exclude = "*.rs"]
struct LanguageDir;

pub fn init(languages: Arc<LanguageRegistry>, node_runtime: NodeRuntime, cx: &mut AppContext) {
    #[cfg(feature = "load-grammars")]
    languages.register_native_grammars([
        ("bash", tree_sitter_bash::LANGUAGE),
        ("c", tree_sitter_c::LANGUAGE),
        ("cpp", tree_sitter_cpp::LANGUAGE),
        ("css", tree_sitter_css::LANGUAGE),
        ("diff", tree_sitter_diff::LANGUAGE),
        ("go", tree_sitter_go::LANGUAGE),
        ("gomod", tree_sitter_go_mod::LANGUAGE),
        ("gowork", tree_sitter_gowork::LANGUAGE),
        ("jsdoc", tree_sitter_jsdoc::LANGUAGE),
        ("json", tree_sitter_json::LANGUAGE),
        ("jsonc", tree_sitter_json::LANGUAGE),
        ("markdown", tree_sitter_md::LANGUAGE),
        ("markdown-inline", tree_sitter_md::INLINE_LANGUAGE),
        ("python", tree_sitter_python::LANGUAGE),
        ("regex", tree_sitter_regex::LANGUAGE),
        ("rust", tree_sitter_rust::LANGUAGE),
        ("tsx", tree_sitter_typescript::LANGUAGE_TSX),
        ("typescript", tree_sitter_typescript::LANGUAGE_TYPESCRIPT),
        ("yaml", tree_sitter_yaml::LANGUAGE),
    ]);

    macro_rules! language {
        ($name:literal) => {
            let config = load_config($name);
            languages.register_language(
                config.name.clone(),
                config.grammar.clone(),
                config.matcher.clone(),
                move || {
                    Ok(LoadedLanguage {
                        config: config.clone(),
                        queries: load_queries($name),
                        context_provider: None,
                        toolchain_provider: None,
                    })
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
                    Ok(LoadedLanguage {
                        config: config.clone(),
                        queries: load_queries($name),
                        context_provider: None,
                        toolchain_provider: None,
                    })
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
                    Ok(LoadedLanguage {
                        config: config.clone(),
                        queries: load_queries($name),
                        context_provider: Some(Arc::new($context_provider)),
                        toolchain_provider: None,
                    })
                },
            );
        };
        ($name:literal, $adapters:expr, $context_provider:expr, $toolchain_provider:expr) => {
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
                    Ok(LoadedLanguage {
                        config: config.clone(),
                        queries: load_queries($name),
                        context_provider: Some(Arc::new($context_provider)),
                        toolchain_provider: Some($toolchain_provider),
                    })
                },
            );
        };
    }
    language!("bash", Vec::new(), bash_task_context());
    language!("c", vec![Arc::new(c::CLspAdapter) as Arc<dyn LspAdapter>]);
    language!("cpp", vec![Arc::new(c::CLspAdapter)]);
    language!(
        "css",
        vec![Arc::new(css::CssLspAdapter::new(node_runtime.clone())),]
    );
    language!("diff");
    language!("go", vec![Arc::new(go::GoLspAdapter)], GoContextProvider);
    language!("gomod", vec![Arc::new(go::GoLspAdapter)], GoContextProvider);
    language!(
        "gowork",
        vec![Arc::new(go::GoLspAdapter)],
        GoContextProvider
    );

    language!(
        "json",
        vec![
            Arc::new(json::JsonLspAdapter::new(
                node_runtime.clone(),
                languages.clone(),
            )),
            Arc::new(json::NodeVersionAdapter)
        ],
        json_task_context()
    );
    language!(
        "jsonc",
        vec![Arc::new(json::JsonLspAdapter::new(
            node_runtime.clone(),
            languages.clone(),
        ))],
        json_task_context()
    );
    language!("markdown");
    language!("markdown-inline");
    language!(
        "python",
        vec![Arc::new(python::PythonLspAdapter::new(
            node_runtime.clone(),
        ))],
        PythonContextProvider,
        Arc::new(PythonToolchainProvider::default()) as Arc<dyn ToolchainLister>
    );
    language!(
        "rust",
        vec![Arc::new(rust::RustLspAdapter)],
        RustContextProvider
    );
    language!(
        "tsx",
        vec![
            Arc::new(typescript::TypeScriptLspAdapter::new(node_runtime.clone())),
            Arc::new(vtsls::VtslsLspAdapter::new(node_runtime.clone()))
        ],
        typescript_task_context()
    );
    language!(
        "typescript",
        vec![
            Arc::new(typescript::TypeScriptLspAdapter::new(node_runtime.clone())),
            Arc::new(vtsls::VtslsLspAdapter::new(node_runtime.clone()))
        ],
        typescript_task_context()
    );
    language!(
        "javascript",
        vec![
            Arc::new(typescript::TypeScriptLspAdapter::new(node_runtime.clone())),
            Arc::new(vtsls::VtslsLspAdapter::new(node_runtime.clone()))
        ],
        typescript_task_context()
    );
    language!(
        "jsdoc",
        vec![
            Arc::new(typescript::TypeScriptLspAdapter::new(node_runtime.clone(),)),
            Arc::new(vtsls::VtslsLspAdapter::new(node_runtime.clone()))
        ]
    );
    language!("regex");
    language!(
        "yaml",
        vec![Arc::new(yaml::YamlLspAdapter::new(node_runtime.clone()))]
    );

    // Register globally available language servers.
    //
    // This will allow users to add support for a built-in language server (e.g., Tailwind)
    // for a given language via the `language_servers` setting:
    //
    // ```json
    // {
    //   "languages": {
    //     "My Language": {
    //       "language_servers": ["tailwindcss-language-server", "..."]
    //     }
    //   }
    // }
    // ```
    languages.register_available_lsp_adapter(
        LanguageServerName("tailwindcss-language-server".into()),
        {
            let node_runtime = node_runtime.clone();
            move || Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone()))
        },
    );
    languages.register_available_lsp_adapter(LanguageServerName("eslint".into()), {
        let node_runtime = node_runtime.clone();
        move || Arc::new(typescript::EsLintLspAdapter::new(node_runtime.clone()))
    });

    // Register Tailwind for the existing languages that should have it by default.
    //
    // This can be driven by the `language_servers` setting once we have a way for
    // extensions to provide their own default value for that setting.
    let tailwind_languages = [
        "Astro",
        "CSS",
        "ERB",
        "HEEX",
        "HTML",
        "JavaScript",
        "PHP",
        "Svelte",
        "TSX",
        "Vue.js",
    ];

    for language in tailwind_languages {
        languages.register_lsp_adapter(
            language.into(),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        );
    }

    let eslint_languages = ["TSX", "TypeScript", "JavaScript", "Vue.js", "Svelte"];
    for language in eslint_languages {
        languages.register_lsp_adapter(
            language.into(),
            Arc::new(typescript::EsLintLspAdapter::new(node_runtime.clone())),
        );
    }

    let mut subscription = languages.subscribe();
    let mut prev_language_settings = languages.language_settings();

    cx.spawn(|cx| async move {
        while subscription.next().await.is_some() {
            let language_settings = languages.language_settings();
            if language_settings != prev_language_settings {
                cx.update(|cx| {
                    SettingsStore::update_global(cx, |settings, cx| {
                        settings
                            .set_extension_settings(language_settings.clone(), cx)
                            .log_err();
                    });
                })?;
                prev_language_settings = language_settings;
            }
        }
        anyhow::Ok(())
    })
    .detach();
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
            .unwrap_or_else(|| panic!("missing config for language {:?}", name))
            .data
            .to_vec(),
    )
    .unwrap();

    #[allow(unused_mut)]
    let mut config: LanguageConfig = ::toml::from_str(&config_toml)
        .with_context(|| format!("failed to load config.toml for language {name:?}"))
        .unwrap();

    #[cfg(not(feature = "load-grammars"))]
    {
        config = LanguageConfig {
            name: config.name,
            matcher: config.matcher,
            ..Default::default()
        }
    }

    config
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
