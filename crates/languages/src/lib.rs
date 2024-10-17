use anyhow::Context;
use gpui::{AppContext, UpdateGlobal};
use json::json_task_context;
pub use language::*;
use node_runtime::NodeRuntime;
use python::PythonContextProvider;
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

mod dsl {
    use std::sync::Arc;

    use language::{ContextProvider, LanguageRegistry, LspAdapter};

    use crate::{load_config, load_queries};

    pub(super) struct LanguageBootstrapRecipe<'a> {
        name: &'static str,
        adapters: &'a [Arc<dyn LspAdapter>],
        context_provider: Option<Box<dyn Fn() -> Arc<dyn ContextProvider> + 'static + Send + Sync>>,
    }
    impl From<&'static str> for LanguageBootstrapRecipe<'static> {
        fn from(name: &'static str) -> Self {
            Self {
                name,
                adapters: &[],
                context_provider: None,
            }
        }
    }

    impl<'a, const ADAPTER_COUNT: usize>
        From<(&'static str, &'a [Arc<dyn LspAdapter>; ADAPTER_COUNT])>
        for LanguageBootstrapRecipe<'a>
    {
        fn from(value: (&'static str, &'a [Arc<dyn LspAdapter>; ADAPTER_COUNT])) -> Self {
            Self {
                name: value.0,
                adapters: value.1.as_ref(),
                context_provider: None,
            }
        }
    }
    impl<
            'a,
            const ADAPTER_COUNT: usize,
            T: ContextProvider + 'static,
            Callback: Fn() -> T + 'static + Send + Sync,
        >
        From<(
            &'static str,
            &'a [Arc<dyn LspAdapter>; ADAPTER_COUNT],
            Callback,
        )> for LanguageBootstrapRecipe<'a>
    {
        fn from(
            value: (
                &'static str,
                &'a [Arc<dyn LspAdapter>; ADAPTER_COUNT],
                Callback,
            ),
        ) -> Self {
            Self {
                name: value.0,
                adapters: value.1.as_ref(),
                context_provider: Some(Box::new(move || Arc::new((value.2)()))),
            }
        }
    }
    impl<T: ContextProvider + 'static, Callback: Fn() -> T + 'static + Send + Sync>
        From<(&'static str, Callback)> for LanguageBootstrapRecipe<'static>
    {
        fn from(value: (&'static str, Callback)) -> Self {
            Self {
                name: value.0,
                adapters: &[],
                context_provider: Some(Box::new(move || Arc::new((value.1)()))),
            }
        }
    }

    pub(super) fn language<'a>(
        languages: &LanguageRegistry,
        config: impl Into<LanguageBootstrapRecipe<'a>>,
    ) {
        let config = config.into();
        language_impl(languages, config)
    }
    fn language_impl<'a>(
        languages: &LanguageRegistry,
        bootstrap_config: LanguageBootstrapRecipe<'a>,
    ) {
        let config = load_config(bootstrap_config.name);
        for adapter in bootstrap_config.adapters {
            languages.register_lsp_adapter(config.name.clone(), adapter.clone());
        }
        languages.register_language(
            config.name.clone(),
            config.grammar.clone(),
            config.matcher.clone(),
            move || {
                let context_provider = if let Some(factory) = &bootstrap_config.context_provider {
                    Some(factory())
                } else {
                    None
                };
                Ok((
                    config.clone(),
                    load_queries(bootstrap_config.name),
                    context_provider,
                ))
            },
        );
    }
}
type Adapter = Arc<dyn LspAdapter>;
trait DynAdapter {
    fn create<T: LspAdapter>(inner: T) -> Adapter;
}
impl DynAdapter for Adapter {
    fn create<T: LspAdapter>(inner: T) -> Adapter {
        Arc::new(inner)
    }
}
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
        ($($arg:expr), *) => {
            dsl::language(&languages, ($($arg), *))
        }

    }
    language!("bash", bash_task_context);
    language!("c", &[Adapter::create(c::CLspAdapter)]);
    language!("cpp", &[Adapter::create(c::CLspAdapter)]);
    language!(
        "css",
        &[Adapter::create(css::CssLspAdapter::new(
            node_runtime.clone()
        )),]
    );
    language!("diff");
    language!(
        "go",
        &[Adapter::create(go::GoLspAdapter)],
        GoContextProvider::default
    );
    language!(
        "gomod",
        &[Adapter::create(go::GoLspAdapter)],
        GoContextProvider::default
    );
    language!(
        "gowork",
        &[Adapter::create(go::GoLspAdapter)],
        GoContextProvider::default
    );

    language!(
        "json",
        &[
            Adapter::create(json::JsonLspAdapter::new(
                node_runtime.clone(),
                languages.clone(),
            )),
            Adapter::create(json::NodeVersionAdapter)
        ],
        json_task_context
    );
    language!(
        "jsonc",
        &[Adapter::create(json::JsonLspAdapter::new(
            node_runtime.clone(),
            languages.clone(),
        ))],
        json_task_context
    );
    language!("markdown");
    language!("markdown-inline");
    language!(
        "python",
        &[Adapter::create(python::PythonLspAdapter::new(
            node_runtime.clone(),
        ))],
        PythonContextProvider::default
    );
    language!(
        "rust",
        &[Adapter::create(rust::RustLspAdapter)],
        RustContextProvider::default
    );
    language!(
        "tsx",
        &[
            Adapter::create(typescript::TypeScriptLspAdapter::new(node_runtime.clone())),
            Adapter::create(vtsls::VtslsLspAdapter::new(node_runtime.clone()))
        ],
        typescript_task_context
    );
    language!(
        "typescript",
        &[
            Adapter::create(typescript::TypeScriptLspAdapter::new(node_runtime.clone())),
            Adapter::create(vtsls::VtslsLspAdapter::new(node_runtime.clone()))
        ],
        typescript_task_context
    );
    language!(
        "javascript",
        &[
            Adapter::create(typescript::TypeScriptLspAdapter::new(node_runtime.clone())),
            Adapter::create(vtsls::VtslsLspAdapter::new(node_runtime.clone()))
        ],
        typescript_task_context
    );
    language!(
        "jsdoc",
        &[
            Adapter::create(typescript::TypeScriptLspAdapter::new(node_runtime.clone(),)),
            Adapter::create(vtsls::VtslsLspAdapter::new(node_runtime.clone()))
        ]
    );
    language!("regex");
    language!(
        "yaml",
        &[Adapter::create(yaml::YamlLspAdapter::new(
            node_runtime.clone()
        ))]
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
