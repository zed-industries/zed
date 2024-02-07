use crate::{ExtensionStore, GrammarManifestEntry, LanguageManifestEntry, ThemeManifestEntry};
use fs::FakeFs;
use gpui::{Context, TestAppContext};
use language::{LanguageMatcher, LanguageRegistry};
use serde_json::json;
use std::{path::PathBuf, sync::Arc};
use theme::ThemeRegistry;

#[gpui::test]
async fn test_extension_store(cx: &mut TestAppContext) {
    let fs = FakeFs::new(cx.executor());

    fs.insert_tree(
        "/the-extension-dir",
        json!({
            "zed-monokai": {
                "themes": {
                    "monokai.json": r#"{
                        "name": "Monokai",
                        "author": "Someone",
                        "themes": [
                            {
                                "name": "Monokai Dark",
                                "appearance": "dark",
                                "style": {}
                            },
                            {
                                "name": "Monokai Light",
                                "appearance": "light",
                                "style": {}
                            }
                        ]
                    }"#,
                    "monokai-pro.json": r#"{
                        "name": "Monokai Pro",
                        "author": "Someone",
                        "themes": [
                            {
                                "name": "Monokai Pro Dark",
                                "appearance": "dark",
                                "style": {}
                            },
                            {
                                "name": "Monokai Pro Light",
                                "appearance": "light",
                                "style": {}
                            }
                        ]
                    }"#,
                }
            },
            "zed-ruby": {
                "grammars": {
                    "ruby.wasm": "",
                    "embedded_template.wasm": "",
                },
                "languages": {
                    "ruby": {
                        "config.toml": r#"
                            name = "Ruby"
                            grammar = "ruby"
                            path_suffixes = ["rb"]
                        "#,
                        "highlights.scm": "",
                    },
                    "erb": {
                        "config.toml": r#"
                            name = "ERB"
                            grammar = "embedded_template"
                            path_suffixes = ["erb"]
                        "#,
                        "highlights.scm": "",
                    }
                },
            }
        }),
    )
    .await;

    let language_registry = Arc::new(LanguageRegistry::test());
    let theme_registry = Arc::new(ThemeRegistry::new(Box::new(())));

    let store = cx.new_model(|cx| {
        ExtensionStore::new(
            PathBuf::from("/the-extension-dir"),
            PathBuf::from("/the-extension-manifest-path.json"),
            fs,
            language_registry,
            theme_registry,
            cx,
        )
    });

    store
        .update(cx, |store, cx| store.rebuild_manifest(cx))
        .await
        .unwrap();

    store.read_with(cx, |store, _| {
        let manifest = store.manifest.read();
        assert_eq!(
            manifest.grammars,
            [
                (
                    "embedded_template".into(),
                    GrammarManifestEntry {
                        extension: "zed-ruby".into(),
                        path: "grammars/embedded_template.wasm".into(),
                    }
                ),
                (
                    "ruby".into(),
                    GrammarManifestEntry {
                        extension: "zed-ruby".into(),
                        path: "grammars/ruby.wasm".into(),
                    }
                ),
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(
            manifest.languages,
            [
                (
                    "ERB".into(),
                    LanguageManifestEntry {
                        extension: "zed-ruby".into(),
                        path: "languages/erb".into(),
                        matcher: LanguageMatcher {
                            path_suffixes: vec!["erb".into()],
                            first_line_pattern: None,
                        }
                    }
                ),
                (
                    "Ruby".into(),
                    LanguageManifestEntry {
                        extension: "zed-ruby".into(),
                        path: "languages/ruby".into(),
                        matcher: LanguageMatcher {
                            path_suffixes: vec!["rb".into()],
                            first_line_pattern: None,
                        }
                    },
                )
            ]
            .into_iter()
            .collect(),
        );
        assert_eq!(
            manifest.themes,
            [
                (
                    "Monokai Dark".into(),
                    ThemeManifestEntry {
                        extension: "zed-monokai".into(),
                        path: "themes/monokai.json".into(),
                    }
                ),
                (
                    "Monokai Light".into(),
                    ThemeManifestEntry {
                        extension: "zed-monokai".into(),
                        path: "themes/monokai.json".into(),
                    }
                ),
                (
                    "Monokai Pro Dark".into(),
                    ThemeManifestEntry {
                        extension: "zed-monokai".into(),
                        path: "themes/monokai-pro.json".into(),
                    }
                ),
                (
                    "Monokai Pro Light".into(),
                    ThemeManifestEntry {
                        extension: "zed-monokai".into(),
                        path: "themes/monokai-pro.json".into(),
                    }
                )
            ]
            .into_iter()
            .collect(),
        );
    })
}
