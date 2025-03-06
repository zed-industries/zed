use anyhow::Context as _;
use gpui::{App, UpdateGlobal};
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

use crate::{bash::bash_task_context, rust::RustContextProvider};

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

/// A shared grammar for plain text, exposed for reuse by downstream crates.
#[cfg(feature = "tree-sitter-gitcommit")]
pub static LANGUAGE_GIT_COMMIT: std::sync::LazyLock<Arc<Language>> =
    std::sync::LazyLock::new(|| {
        Arc::new(Language::new(
            LanguageConfig {
                name: "Git Commit".into(),
                soft_wrap: Some(language::language_settings::SoftWrap::EditorWidth),
                matcher: LanguageMatcher {
                    path_suffixes: vec!["COMMIT_EDITMSG".to_owned()],
                    first_line_pattern: None,
                },
                line_comments: vec![Arc::from("#")],
                ..LanguageConfig::default()
            },
            Some(tree_sitter_gitcommit::LANGUAGE.into()),
        ))
    });

pub fn init(languages: Arc<LanguageRegistry>, node_runtime: NodeRuntime, cx: &mut App) {
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
        ("gitcommit", tree_sitter_gitcommit::LANGUAGE),
    ]);

    // Following are a series of helper macros for registering languages.
    // Macros are used instead of a function or for loop in order to avoid
    // code duplication and improve readability as the types get quite verbose
    // to type out in some cases.
    // Additionally, the `provider` fields in LoadedLanguage
    // would have be `Copy` if we were to use a function or for-loop to register the languages
    // due to the fact that we pass an `Arc<Fn>` to `languages.register_language`
    // that loads and initializes the language lazily.
    // We avoid this entirely by using a Macro

    macro_rules! context_provider {
        ($name:expr) => {
            Some(Arc::new($name) as Arc<dyn ContextProvider>)
        };
        () => {
            None
        };
    }

    macro_rules! toolchain_provider {
        ($name:expr) => {
            Some(Arc::new($name) as Arc<dyn ToolchainLister>)
        };
        () => {
            None
        };
    }

    macro_rules! adapters {
        ($($item:expr),+ $(,)?) => {
            vec![
                $(Arc::new($item) as Arc<dyn LspAdapter>,)*
            ]
        };
        () => {
            vec![]
        };
    }

    macro_rules! register_language {
        ($name:expr, adapters => $adapters:expr, context => $context:expr, toolchain => $toolchain:expr) => {
            let config = load_config($name);
            for adapter in $adapters {
                languages.register_lsp_adapter(config.name.clone(), adapter);
            }
            languages.register_language(
                config.name.clone(),
                config.grammar.clone(),
                config.matcher.clone(),
                config.hidden,
                Arc::new(move || {
                    Ok(LoadedLanguage {
                        config: config.clone(),
                        queries: load_queries($name),
                        context_provider: $context,
                        toolchain_provider: $toolchain,
                    })
                }),
            );
        };
        ($name:expr) => {
            register_language!($name, adapters => adapters![], context => context_provider!(), toolchain => toolchain_provider!())
        };
        ($name:expr, adapters => $adapters:expr, context => $context:expr, toolchain => $toolchain:expr) => {
            register_language!($name, adapters => $adapters, context => $context, toolchain => $toolchain)
        };
        ($name:expr, adapters => $adapters:expr, context => $context:expr) => {
            register_language!($name, adapters => $adapters, context => $context, toolchain => toolchain_provider!())
        };
        ($name:expr, adapters => $adapters:expr) => {
            register_language!($name, adapters => $adapters, context => context_provider!(), toolchain => toolchain_provider!())
        };
    }

    register_language!(
        "bash",
        adapters => adapters![],
        context => context_provider!(bash_task_context()),
        toolchain => toolchain_provider!()
    );

    register_language!(
        "c",
        adapters => adapters![c::CLspAdapter]
    );
    register_language!(
        "cpp",
        adapters => adapters![c::CLspAdapter]
    );

    register_language!(
        "css",
        adapters => adapters![css::CssLspAdapter::new(node_runtime.clone())]
    );

    register_language!("diff");

    register_language!(
        "go",
        adapters => adapters![go::GoLspAdapter],
        context => context_provider!(go::GoContextProvider)
    );
    register_language!(
        "gomod",
        adapters => adapters![go::GoLspAdapter],
        context => context_provider!(go::GoContextProvider)
    );
    register_language!(
        "gowork",
        adapters => adapters![go::GoLspAdapter],
        context => context_provider!(go::GoContextProvider)
    );

    register_language!(
        "json",
        adapters => adapters![
            json::JsonLspAdapter::new(node_runtime.clone(), languages.clone(),),
            json::NodeVersionAdapter,
        ],
        context => context_provider!(json_task_context())
    );
    register_language!(
        "jsonc",
        adapters => adapters![
            json::JsonLspAdapter::new(node_runtime.clone(), languages.clone(),),
        ],
        context => context_provider!(json_task_context())
    );

    register_language!("markdown");
    register_language!("markdown-inline");

    register_language!(
        "python",
        adapters => adapters![
            python::PythonLspAdapter::new(node_runtime.clone()),
            python::PyLspAdapter::new()
        ],
        context => context_provider!(PythonContextProvider),
        toolchain => toolchain_provider!(PythonToolchainProvider::default())
    );
    register_language!(
        "rust",
        adapters => adapters![rust::RustLspAdapter],
        context => context_provider!(RustContextProvider)
    );
    register_language!(
        "tsx",
        adapters => adapters![
            typescript::TypeScriptLspAdapter::new(node_runtime.clone()),
            vtsls::VtslsLspAdapter::new(node_runtime.clone()),
        ],
        context => context_provider!(typescript_task_context()),
        toolchain => toolchain_provider!()
    );
    register_language!(
        "typescript",
        adapters => adapters![
            typescript::TypeScriptLspAdapter::new(node_runtime.clone()),
            vtsls::VtslsLspAdapter::new(node_runtime.clone()),
        ],
        context => context_provider!(typescript_task_context())
    );
    register_language!(
        "javascript",
        adapters => adapters![
            typescript::TypeScriptLspAdapter::new(node_runtime.clone()),
            vtsls::VtslsLspAdapter::new(node_runtime.clone()),
        ],
        context => context_provider!(typescript_task_context())
    );
    register_language!(
        "jsdoc",
        adapters => adapters![
            typescript::TypeScriptLspAdapter::new(node_runtime.clone()),
            vtsls::VtslsLspAdapter::new(node_runtime.clone()),
        ]
    );

    register_language!("regex");

    register_language!("yaml",
        adapters => adapters![
            yaml::YamlLspAdapter::new(node_runtime.clone()),
        ]
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
    languages.register_available_lsp_adapter(LanguageServerName("vtsls".into()), {
        let node_runtime = node_runtime.clone();
        move || Arc::new(vtsls::VtslsLspAdapter::new(node_runtime.clone()))
    });
    languages.register_available_lsp_adapter(
        LanguageServerName("typescript-language-server".into()),
        {
            let node_runtime = node_runtime.clone();
            move || Arc::new(typescript::TypeScriptLspAdapter::new(node_runtime.clone()))
        },
    );

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

    #[cfg(not(any(feature = "load-grammars", test)))]
    {
        config = LanguageConfig {
            name: config.name,
            matcher: config.matcher,
            jsx_tag_auto_close: config.jsx_tag_auto_close,
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
