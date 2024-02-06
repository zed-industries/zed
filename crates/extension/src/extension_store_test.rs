use crate::{ExtensionStore, GrammarManifestEntry, LanguageManifestEntry, ThemeLocation};
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
        "/the-extension-path",
        json!({
            "extensions": {
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
            }
        }),
    )
    .await;

    let language_registry = Arc::new(LanguageRegistry::test());
    let theme_registry = Arc::new(ThemeRegistry::new(Box::new(())));

    let store = cx.new_model(|_| {
        ExtensionStore::new(
            PathBuf::from("/the-extension-path"),
            fs,
            language_registry,
            theme_registry,
        )
    });

    store
        .update(cx, |store, cx| store.rebuild_manifest(cx))
        .await
        .unwrap();

    store.read_with(cx, |store, _| {
        let manifest = &store.manifest;
        assert_eq!(
            manifest.grammars,
            &[
                GrammarManifestEntry {
                    extension: "zed-ruby".into(),
                    grammar_name: "embedded_template".into(),
                },
                GrammarManifestEntry {
                    extension: "zed-ruby".into(),
                    grammar_name: "ruby".into(),
                },
            ],
        );
        assert_eq!(
            manifest.languages,
            &[
                LanguageManifestEntry {
                    extension: "zed-ruby".into(),
                    language_dir: "erb".into(),
                    name: "ERB".into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec!["erb".into()],
                        first_line_pattern: None,
                    }
                },
                LanguageManifestEntry {
                    extension: "zed-ruby".into(),
                    language_dir: "ruby".into(),
                    name: "Ruby".into(),
                    matcher: LanguageMatcher {
                        path_suffixes: vec!["rb".into()],
                        first_line_pattern: None,
                    }
                },
            ],
        );
        assert_eq!(
            manifest.themes_by_name.get("Monokai Dark"),
            Some(&ThemeLocation {
                extension: "zed-monokai".into(),
                filename: "monokai.json".into(),
            })
        );
        assert_eq!(
            manifest.themes_by_name.get("Monokai Light"),
            Some(&ThemeLocation {
                extension: "zed-monokai".into(),
                filename: "monokai.json".into(),
            })
        );
        assert_eq!(
            manifest.themes_by_name.get("Monokai Pro Dark"),
            Some(&ThemeLocation {
                extension: "zed-monokai".into(),
                filename: "monokai-pro.json".into(),
            })
        );
        assert_eq!(
            manifest.themes_by_name.get("Monokai Pro Light"),
            Some(&ThemeLocation {
                extension: "zed-monokai".into(),
                filename: "monokai-pro.json".into(),
            })
        );
    })
}
