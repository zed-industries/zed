use anyhow::Context;
use gpui::{AppContext, UpdateGlobal};
use json::json_task_context;
pub use language::*;
use node_runtime::NodeRuntime;
use rust_embed::RustEmbed;
use settings::SettingsStore;
use smol::stream::StreamExt;
use std::{str, sync::Arc};
use util::{asset_str, ResultExt};

use crate::{
    bash::bash_task_context, go::GoContextProvider, python::python_task_context,
    rust::RustContextProvider,
};

mod bash;
mod c;
mod css;
mod go;
mod json;
mod python;
mod rust;
mod tailwind;
mod typescript;
mod yaml;

#[derive(RustEmbed)]
#[folder = "src/"]
#[exclude = "*.rs"]
struct LanguageDir;

pub fn init(
    languages: Arc<LanguageRegistry>,
    node_runtime: Arc<dyn NodeRuntime>,
    cx: &mut AppContext,
) {
    languages.register_native_grammars([
        ("bash", tree_sitter_bash::language()),
        ("c", tree_sitter_c::language()),
        ("cpp", tree_sitter_cpp::language()),
        ("css", tree_sitter_css::language()),
        ("go", tree_sitter_go::language()),
        ("gomod", tree_sitter_gomod::language()),
        ("gowork", tree_sitter_gowork::language()),
        ("jsdoc", tree_sitter_jsdoc::language()),
        ("json", tree_sitter_json::language()),
        ("markdown", tree_sitter_markdown::language()),
        ("proto", tree_sitter_proto::language()),
        ("python", tree_sitter_python::language()),
        ("regex", tree_sitter_regex::language()),
        ("rust", tree_sitter_rust::language()),
        ("tsx", tree_sitter_typescript::language_tsx()),
        ("typescript", tree_sitter_typescript::language_typescript()),
        ("yaml", tree_sitter_yaml::language()),
    ]);

    macro_rules! language {
        ($name:literal) => {
            let config = load_config($name);
            languages.register_language(
                config.name.clone(),
                config.grammar.clone(),
                config.matcher.clone(),
                move || Ok((config.clone(), load_queries($name), None)),
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
                move || Ok((config.clone(), load_queries($name), None)),
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
    language!("bash", Vec::new(), bash_task_context());
    language!("c", vec![Arc::new(c::CLspAdapter) as Arc<dyn LspAdapter>]);
    language!("cpp", vec![Arc::new(c::CLspAdapter)]);
    language!(
        "css",
        vec![Arc::new(css::CssLspAdapter::new(node_runtime.clone())),]
    );
    language!("go", vec![Arc::new(go::GoLspAdapter)], GoContextProvider);
    language!("gomod", vec![Arc::new(go::GoLspAdapter)], GoContextProvider);
    language!(
        "gowork",
        vec![Arc::new(go::GoLspAdapter)],
        GoContextProvider
    );

    language!(
        "json",
        vec![Arc::new(json::JsonLspAdapter::new(
            node_runtime.clone(),
            languages.clone(),
        ))],
        json_task_context()
    );
    language!("markdown");
    language!(
        "python",
        vec![Arc::new(python::PythonLspAdapter::new(
            node_runtime.clone(),
        ))],
        python_task_context()
    );
    language!(
        "rust",
        vec![Arc::new(rust::RustLspAdapter)],
        RustContextProvider
    );
    language!(
        "tsx",
        vec![Arc::new(typescript::TypeScriptLspAdapter::new(
            node_runtime.clone()
        ))]
    );
    language!(
        "typescript",
        vec![Arc::new(typescript::TypeScriptLspAdapter::new(
            node_runtime.clone()
        ))]
    );
    language!(
        "javascript",
        vec![Arc::new(typescript::TypeScriptLspAdapter::new(
            node_runtime.clone()
        ))]
    );
    language!(
        "jsdoc",
        vec![Arc::new(typescript::TypeScriptLspAdapter::new(
            node_runtime.clone(),
        ))]
    );
    language!("regex");
    language!(
        "yaml",
        vec![Arc::new(yaml::YamlLspAdapter::new(node_runtime.clone()))]
    );
    language!("proto");

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
        languages.register_secondary_lsp_adapter(
            language.into(),
            Arc::new(tailwind::TailwindLspAdapter::new(node_runtime.clone())),
        );
    }

    let eslint_languages = ["TSX", "TypeScript", "JavaScript", "Vue.js", "Svelte"];
    for language in eslint_languages {
        languages.register_secondary_lsp_adapter(
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
