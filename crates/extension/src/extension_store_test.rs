use crate::{
    ExtensionStore, GrammarManifestEntry, LanguageManifestEntry, Manifest, ThemeManifestEntry,
};
use fs::FakeFs;
use gpui::{Context, TestAppContext};
use language::{LanguageMatcher, LanguageRegistry};
use serde_json::json;
use settings::SettingsStore;
use std::{path::PathBuf, sync::Arc};
use theme::ThemeRegistry;
use util::http::FakeHttpClient;

#[gpui::test]
async fn test_extension_store(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        theme::init(theme::LoadThemes::JustBase, cx);
    });

    let fs = FakeFs::new(cx.executor());
    let http_client = FakeHttpClient::with_200_response();

    fs.insert_tree(
        "/the-extension-dir",
        json!({
            "installed": {
                "zed-monokai": {
                    "extension.json": r#"{
                        "id": "zed-monokai",
                        "name": "Zed Monokai",
                        "version": "2.0.0"
                    }"#,
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
                    "extension.json": r#"{
                        "id": "zed-ruby",
                        "name": "Zed Ruby",
                        "version": "1.0.0"
                    }"#,
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

    let mut expected_manifest = Manifest {
        extensions: [
            ("zed-ruby".into(), "1.0.0".into()),
            ("zed-monokai".into(), "2.0.0".into()),
        ]
        .into_iter()
        .collect(),
        grammars: [
            (
                "embedded_template".into(),
                GrammarManifestEntry {
                    extension: "zed-ruby".into(),
                    path: "grammars/embedded_template.wasm".into(),
                },
            ),
            (
                "ruby".into(),
                GrammarManifestEntry {
                    extension: "zed-ruby".into(),
                    path: "grammars/ruby.wasm".into(),
                },
            ),
        ]
        .into_iter()
        .collect(),
        languages: [
            (
                "ERB".into(),
                LanguageManifestEntry {
                    extension: "zed-ruby".into(),
                    path: "languages/erb".into(),
                    grammar: Some("embedded_template".into()),
                    matcher: LanguageMatcher {
                        path_suffixes: vec!["erb".into()],
                        first_line_pattern: None,
                    },
                },
            ),
            (
                "Ruby".into(),
                LanguageManifestEntry {
                    extension: "zed-ruby".into(),
                    path: "languages/ruby".into(),
                    grammar: Some("ruby".into()),
                    matcher: LanguageMatcher {
                        path_suffixes: vec!["rb".into()],
                        first_line_pattern: None,
                    },
                },
            ),
        ]
        .into_iter()
        .collect(),
        themes: [
            (
                "Monokai Dark".into(),
                ThemeManifestEntry {
                    extension: "zed-monokai".into(),
                    path: "themes/monokai.json".into(),
                },
            ),
            (
                "Monokai Light".into(),
                ThemeManifestEntry {
                    extension: "zed-monokai".into(),
                    path: "themes/monokai.json".into(),
                },
            ),
            (
                "Monokai Pro Dark".into(),
                ThemeManifestEntry {
                    extension: "zed-monokai".into(),
                    path: "themes/monokai-pro.json".into(),
                },
            ),
            (
                "Monokai Pro Light".into(),
                ThemeManifestEntry {
                    extension: "zed-monokai".into(),
                    path: "themes/monokai-pro.json".into(),
                },
            ),
        ]
        .into_iter()
        .collect(),
    };

    let language_registry = Arc::new(LanguageRegistry::test());
    let theme_registry = Arc::new(ThemeRegistry::new(Box::new(())));

    let store = cx.new_model(|cx| {
        ExtensionStore::new(
            PathBuf::from("/the-extension-dir"),
            fs.clone(),
            http_client.clone(),
            language_registry.clone(),
            theme_registry.clone(),
            cx,
        )
    });

    cx.executor().run_until_parked();
    store.read_with(cx, |store, _| {
        let manifest = store.manifest.read();
        assert_eq!(manifest.grammars, expected_manifest.grammars);
        assert_eq!(manifest.languages, expected_manifest.languages);
        assert_eq!(manifest.themes, expected_manifest.themes);

        assert_eq!(
            language_registry.language_names(),
            ["ERB", "Plain Text", "Ruby"]
        );
        assert_eq!(
            theme_registry.list_names(false),
            [
                "Monokai Dark",
                "Monokai Light",
                "Monokai Pro Dark",
                "Monokai Pro Light",
                "One Dark",
            ]
        );
    });

    fs.insert_tree(
        "/the-extension-dir/installed/zed-gruvbox",
        json!({
            "extension.json": r#"{
                "id": "zed-gruvbox",
                "name": "Zed Gruvbox",
                "version": "1.0.0"
            }"#,
            "themes": {
                "gruvbox.json": r#"{
                    "name": "Gruvbox",
                    "author": "Someone Else",
                    "themes": [
                        {
                            "name": "Gruvbox",
                            "appearance": "dark",
                            "style": {}
                        }
                    ]
                }"#,
            }
        }),
    )
    .await;

    expected_manifest.themes.insert(
        "Gruvbox".into(),
        ThemeManifestEntry {
            extension: "zed-gruvbox".into(),
            path: "themes/gruvbox.json".into(),
        },
    );

    store.update(cx, |store, cx| store.reload(cx));

    cx.executor().run_until_parked();
    store.read_with(cx, |store, _| {
        let manifest = store.manifest.read();
        assert_eq!(manifest.grammars, expected_manifest.grammars);
        assert_eq!(manifest.languages, expected_manifest.languages);
        assert_eq!(manifest.themes, expected_manifest.themes);

        assert_eq!(
            theme_registry.list_names(false),
            [
                "Gruvbox",
                "Monokai Dark",
                "Monokai Light",
                "Monokai Pro Dark",
                "Monokai Pro Light",
                "One Dark",
            ]
        );
    });

    let prev_fs_metadata_call_count = fs.metadata_call_count();
    let prev_fs_read_dir_call_count = fs.read_dir_call_count();

    // Create new extension store, as if Zed were restarting.
    drop(store);
    let store = cx.new_model(|cx| {
        ExtensionStore::new(
            PathBuf::from("/the-extension-dir"),
            fs.clone(),
            http_client.clone(),
            language_registry.clone(),
            theme_registry.clone(),
            cx,
        )
    });

    cx.executor().run_until_parked();
    store.read_with(cx, |store, _| {
        let manifest = store.manifest.read();
        assert_eq!(manifest.grammars, expected_manifest.grammars);
        assert_eq!(manifest.languages, expected_manifest.languages);
        assert_eq!(manifest.themes, expected_manifest.themes);

        assert_eq!(
            language_registry.language_names(),
            ["ERB", "Plain Text", "Ruby"]
        );
        assert_eq!(
            language_registry.grammar_names(),
            ["embedded_template".into(), "ruby".into()]
        );
        assert_eq!(
            theme_registry.list_names(false),
            [
                "Gruvbox",
                "Monokai Dark",
                "Monokai Light",
                "Monokai Pro Dark",
                "Monokai Pro Light",
                "One Dark",
            ]
        );

        // The on-disk manifest limits the number of FS calls that need to be made
        // on startup.
        assert_eq!(fs.read_dir_call_count(), prev_fs_read_dir_call_count);
        assert_eq!(fs.metadata_call_count(), prev_fs_metadata_call_count + 2);
    });

    store.update(cx, |store, cx| {
        store.uninstall_extension("zed-ruby".into(), cx)
    });

    cx.executor().run_until_parked();
    expected_manifest.extensions.remove("zed-ruby");
    expected_manifest.languages.remove("Ruby");
    expected_manifest.languages.remove("ERB");
    expected_manifest.grammars.remove("ruby");
    expected_manifest.grammars.remove("embedded_template");

    store.read_with(cx, |store, _| {
        let manifest = store.manifest.read();
        assert_eq!(manifest.grammars, expected_manifest.grammars);
        assert_eq!(manifest.languages, expected_manifest.languages);
        assert_eq!(manifest.themes, expected_manifest.themes);

        assert_eq!(language_registry.language_names(), ["Plain Text"]);
        assert_eq!(language_registry.grammar_names(), []);
    });
}
