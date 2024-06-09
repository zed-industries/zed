use crate::extension_manifest::SchemaVersion;
use crate::extension_settings::ExtensionSettings;
use crate::{
    Event, ExtensionIndex, ExtensionIndexEntry, ExtensionIndexLanguageEntry,
    ExtensionIndexThemeEntry, ExtensionManifest, ExtensionStore, GrammarManifestEntry,
    RELOAD_DEBOUNCE_DURATION,
};
use assistant_slash_command::SlashCommandRegistry;
use async_compression::futures::bufread::GzipEncoder;
use collections::BTreeMap;
use fs::{FakeFs, Fs, RealFs};
use futures::{io::BufReader, AsyncReadExt, StreamExt};
use gpui::{Context, TestAppContext};
use http::{FakeHttpClient, Response};
use language::{LanguageMatcher, LanguageRegistry, LanguageServerBinaryStatus, LanguageServerName};
use node_runtime::FakeNodeRuntime;
use parking_lot::Mutex;
use project::Project;
use serde_json::json;
use settings::{Settings as _, SettingsStore};
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use theme::ThemeRegistry;
use util::test::temp_tree;

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

#[gpui::test]
async fn test_extension_store(cx: &mut TestAppContext) {
    init_test(cx);

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
                        "version": "2.0.0",
                        "themes": {
                            "Monokai Dark": "themes/monokai.json",
                            "Monokai Light": "themes/monokai.json",
                            "Monokai Pro Dark": "themes/monokai-pro.json",
                            "Monokai Pro Light": "themes/monokai-pro.json"
                        }
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
                        "version": "1.0.0",
                        "grammars": {
                            "ruby": "grammars/ruby.wasm",
                            "embedded_template": "grammars/embedded_template.wasm"
                        },
                        "languages": {
                            "ruby": "languages/ruby",
                            "erb": "languages/erb"
                        }
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

    let mut expected_index = ExtensionIndex {
        extensions: [
            (
                "zed-ruby".into(),
                ExtensionIndexEntry {
                    manifest: Arc::new(ExtensionManifest {
                        id: "zed-ruby".into(),
                        name: "Zed Ruby".into(),
                        version: "1.0.0".into(),
                        schema_version: SchemaVersion::ZERO,
                        description: None,
                        authors: Vec::new(),
                        repository: None,
                        themes: Default::default(),
                        lib: Default::default(),
                        languages: vec!["languages/erb".into(), "languages/ruby".into()],
                        grammars: [
                            ("embedded_template".into(), GrammarManifestEntry::default()),
                            ("ruby".into(), GrammarManifestEntry::default()),
                        ]
                        .into_iter()
                        .collect(),
                        language_servers: BTreeMap::default(),
                        slash_commands: BTreeMap::default(),
                    }),
                    dev: false,
                },
            ),
            (
                "zed-monokai".into(),
                ExtensionIndexEntry {
                    manifest: Arc::new(ExtensionManifest {
                        id: "zed-monokai".into(),
                        name: "Zed Monokai".into(),
                        version: "2.0.0".into(),
                        schema_version: SchemaVersion::ZERO,
                        description: None,
                        authors: vec![],
                        repository: None,
                        themes: vec![
                            "themes/monokai-pro.json".into(),
                            "themes/monokai.json".into(),
                        ],
                        lib: Default::default(),
                        languages: Default::default(),
                        grammars: BTreeMap::default(),
                        language_servers: BTreeMap::default(),
                        slash_commands: BTreeMap::default(),
                    }),
                    dev: false,
                },
            ),
        ]
        .into_iter()
        .collect(),
        languages: [
            (
                "ERB".into(),
                ExtensionIndexLanguageEntry {
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
                ExtensionIndexLanguageEntry {
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
                ExtensionIndexThemeEntry {
                    extension: "zed-monokai".into(),
                    path: "themes/monokai.json".into(),
                },
            ),
            (
                "Monokai Light".into(),
                ExtensionIndexThemeEntry {
                    extension: "zed-monokai".into(),
                    path: "themes/monokai.json".into(),
                },
            ),
            (
                "Monokai Pro Dark".into(),
                ExtensionIndexThemeEntry {
                    extension: "zed-monokai".into(),
                    path: "themes/monokai-pro.json".into(),
                },
            ),
            (
                "Monokai Pro Light".into(),
                ExtensionIndexThemeEntry {
                    extension: "zed-monokai".into(),
                    path: "themes/monokai-pro.json".into(),
                },
            ),
        ]
        .into_iter()
        .collect(),
    };

    let language_registry = Arc::new(LanguageRegistry::test(cx.executor()));
    let theme_registry = Arc::new(ThemeRegistry::new(Box::new(())));
    let slash_command_registry = SlashCommandRegistry::new();
    let node_runtime = FakeNodeRuntime::new();

    let store = cx.new_model(|cx| {
        ExtensionStore::new(
            PathBuf::from("/the-extension-dir"),
            None,
            fs.clone(),
            http_client.clone(),
            None,
            node_runtime.clone(),
            language_registry.clone(),
            theme_registry.clone(),
            slash_command_registry.clone(),
            cx,
        )
    });

    cx.executor().advance_clock(super::RELOAD_DEBOUNCE_DURATION);
    store.read_with(cx, |store, _| {
        let index = &store.extension_index;
        assert_eq!(index.extensions, expected_index.extensions);
        assert_eq!(index.languages, expected_index.languages);
        assert_eq!(index.themes, expected_index.themes);

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
                "version": "1.0.0",
                "themes": {
                    "Gruvbox": "themes/gruvbox.json"
                }
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

    expected_index.extensions.insert(
        "zed-gruvbox".into(),
        ExtensionIndexEntry {
            manifest: Arc::new(ExtensionManifest {
                id: "zed-gruvbox".into(),
                name: "Zed Gruvbox".into(),
                version: "1.0.0".into(),
                schema_version: SchemaVersion::ZERO,
                description: None,
                authors: vec![],
                repository: None,
                themes: vec!["themes/gruvbox.json".into()],
                lib: Default::default(),
                languages: Default::default(),
                grammars: BTreeMap::default(),
                language_servers: BTreeMap::default(),
                slash_commands: BTreeMap::default(),
            }),
            dev: false,
        },
    );
    expected_index.themes.insert(
        "Gruvbox".into(),
        ExtensionIndexThemeEntry {
            extension: "zed-gruvbox".into(),
            path: "themes/gruvbox.json".into(),
        },
    );

    let _ = store.update(cx, |store, cx| store.reload(None, cx));

    cx.executor().advance_clock(RELOAD_DEBOUNCE_DURATION);
    store.read_with(cx, |store, _| {
        let index = &store.extension_index;
        assert_eq!(index.extensions, expected_index.extensions);
        assert_eq!(index.languages, expected_index.languages);
        assert_eq!(index.themes, expected_index.themes);

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
            None,
            fs.clone(),
            http_client.clone(),
            None,
            node_runtime.clone(),
            language_registry.clone(),
            theme_registry.clone(),
            slash_command_registry,
            cx,
        )
    });

    cx.executor().run_until_parked();
    store.read_with(cx, |store, _| {
        assert_eq!(store.extension_index, expected_index);
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

    cx.executor().advance_clock(RELOAD_DEBOUNCE_DURATION);
    expected_index.extensions.remove("zed-ruby");
    expected_index.languages.remove("Ruby");
    expected_index.languages.remove("ERB");

    store.read_with(cx, |store, _| {
        assert_eq!(store.extension_index, expected_index);
        assert_eq!(language_registry.language_names(), ["Plain Text"]);
        assert_eq!(language_registry.grammar_names(), []);
    });
}

#[gpui::test]
async fn test_extension_store_with_gleam_extension(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let cache_dir = root_dir.join("target");
    let gleam_extension_dir = root_dir.join("extensions").join("gleam");

    let fs = Arc::new(RealFs::default());
    let extensions_dir = temp_tree(json!({
        "installed": {},
        "work": {}
    }));
    let project_dir = temp_tree(json!({
        "test.gleam": ""
    }));

    let extensions_dir = extensions_dir.path().canonicalize().unwrap();
    let project_dir = project_dir.path().canonicalize().unwrap();

    let project = Project::test(fs.clone(), [project_dir.as_path()], cx).await;

    let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
    let theme_registry = Arc::new(ThemeRegistry::new(Box::new(())));
    let slash_command_registry = SlashCommandRegistry::new();
    let node_runtime = FakeNodeRuntime::new();

    let mut status_updates = language_registry.language_server_binary_statuses();

    struct FakeLanguageServerVersion {
        version: String,
        binary_contents: String,
        http_request_count: usize,
    }

    let language_server_version = Arc::new(Mutex::new(FakeLanguageServerVersion {
        version: "v1.2.3".into(),
        binary_contents: "the-binary-contents".into(),
        http_request_count: 0,
    }));

    let http_client = FakeHttpClient::create({
        let language_server_version = language_server_version.clone();
        move |request| {
            let language_server_version = language_server_version.clone();
            async move {
                let version = language_server_version.lock().version.clone();
                let binary_contents = language_server_version.lock().binary_contents.clone();

                let github_releases_uri = "https://api.github.com/repos/gleam-lang/gleam/releases";
                let asset_download_uri =
                    format!("https://fake-download.example.com/gleam-{version}");

                let uri = request.uri().to_string();
                if uri == github_releases_uri {
                    language_server_version.lock().http_request_count += 1;
                    Ok(Response::new(
                        json!([
                            {
                                "tag_name": version,
                                "prerelease": false,
                                "tarball_url": "",
                                "zipball_url": "",
                                "assets": [
                                    {
                                        "name": format!("gleam-{version}-aarch64-apple-darwin.tar.gz"),
                                        "browser_download_url": asset_download_uri
                                    }
                                ]
                            }
                        ])
                        .to_string()
                        .into(),
                    ))
                } else if uri == asset_download_uri {
                    language_server_version.lock().http_request_count += 1;
                    let mut bytes = Vec::<u8>::new();
                    let mut archive = async_tar::Builder::new(&mut bytes);
                    let mut header = async_tar::Header::new_gnu();
                    header.set_size(binary_contents.len() as u64);
                    archive
                        .append_data(&mut header, "gleam", binary_contents.as_bytes())
                        .await
                        .unwrap();
                    archive.into_inner().await.unwrap();
                    let mut gzipped_bytes = Vec::new();
                    let mut encoder = GzipEncoder::new(BufReader::new(bytes.as_slice()));
                    encoder.read_to_end(&mut gzipped_bytes).await.unwrap();
                    Ok(Response::new(gzipped_bytes.into()))
                } else {
                    Ok(Response::builder().status(404).body("not found".into())?)
                }
            }
        }
    });

    let extension_store = cx.new_model(|cx| {
        ExtensionStore::new(
            extensions_dir.clone(),
            Some(cache_dir),
            fs.clone(),
            http_client.clone(),
            None,
            node_runtime,
            language_registry.clone(),
            theme_registry.clone(),
            slash_command_registry,
            cx,
        )
    });

    // Ensure that debounces fire.
    let mut events = cx.events(&extension_store);
    let executor = cx.executor();
    let _task = cx.executor().spawn(async move {
        while let Some(event) = events.next().await {
            match event {
                crate::Event::StartedReloading => {
                    executor.advance_clock(RELOAD_DEBOUNCE_DURATION);
                }
                _ => (),
            }
        }
    });

    extension_store.update(cx, |_, cx| {
        cx.subscribe(&extension_store, |_, _, event, _| {
            if matches!(event, Event::ExtensionFailedToLoad(_)) {
                panic!("extension failed to load");
            }
        })
        .detach();
    });

    extension_store
        .update(cx, |store, cx| {
            store.install_dev_extension(gleam_extension_dir.clone(), cx)
        })
        .await
        .unwrap();

    let mut fake_servers = language_registry.fake_language_servers("Gleam");

    let buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer(project_dir.join("test.gleam"), cx)
        })
        .await
        .unwrap();

    let fake_server = fake_servers.next().await.unwrap();
    let expected_server_path = extensions_dir.join("work/gleam/gleam-v1.2.3/gleam");
    let expected_binary_contents = language_server_version.lock().binary_contents.clone();

    assert_eq!(fake_server.binary.path, expected_server_path);
    assert_eq!(fake_server.binary.arguments, [OsString::from("lsp")]);
    assert_eq!(
        fs.load(&expected_server_path).await.unwrap(),
        expected_binary_contents
    );
    assert_eq!(language_server_version.lock().http_request_count, 2);
    assert_eq!(
        [
            status_updates.next().await.unwrap(),
            status_updates.next().await.unwrap(),
            status_updates.next().await.unwrap(),
        ],
        [
            (
                LanguageServerName("gleam".into()),
                LanguageServerBinaryStatus::CheckingForUpdate
            ),
            (
                LanguageServerName("gleam".into()),
                LanguageServerBinaryStatus::Downloading
            ),
            (
                LanguageServerName("gleam".into()),
                LanguageServerBinaryStatus::None
            )
        ]
    );

    // The extension creates custom labels for completion items.
    fake_server.handle_request::<lsp::request::Completion, _, _>(|_, _| async move {
        Ok(Some(lsp::CompletionResponse::Array(vec![
            lsp::CompletionItem {
                label: "foo".into(),
                kind: Some(lsp::CompletionItemKind::FUNCTION),
                detail: Some("fn() -> Result(Nil, Error)".into()),
                ..Default::default()
            },
            lsp::CompletionItem {
                label: "bar.baz".into(),
                kind: Some(lsp::CompletionItemKind::FUNCTION),
                detail: Some("fn(List(a)) -> a".into()),
                ..Default::default()
            },
            lsp::CompletionItem {
                label: "Quux".into(),
                kind: Some(lsp::CompletionItemKind::CONSTRUCTOR),
                detail: Some("fn(String) -> T".into()),
                ..Default::default()
            },
            lsp::CompletionItem {
                label: "my_string".into(),
                kind: Some(lsp::CompletionItemKind::CONSTANT),
                detail: Some("String".into()),
                ..Default::default()
            },
        ])))
    });

    let completion_labels = project
        .update(cx, |project, cx| project.completions(&buffer, 0, cx))
        .await
        .unwrap()
        .into_iter()
        .map(|c| c.label.text)
        .collect::<Vec<_>>();
    assert_eq!(
        completion_labels,
        [
            "foo: fn() -> Result(Nil, Error)".to_string(),
            "bar.baz: fn(List(a)) -> a".to_string(),
            "Quux: fn(String) -> T".to_string(),
            "my_string: String".to_string(),
        ]
    );

    // Simulate a new version of the language server being released
    language_server_version.lock().version = "v2.0.0".into();
    language_server_version.lock().binary_contents = "the-new-binary-contents".into();
    language_server_version.lock().http_request_count = 0;

    // Start a new instance of the language server.
    project.update(cx, |project, cx| {
        project.restart_language_servers_for_buffers([buffer.clone()], cx)
    });

    // The extension has cached the binary path, and does not attempt
    // to reinstall it.
    let fake_server = fake_servers.next().await.unwrap();
    assert_eq!(fake_server.binary.path, expected_server_path);
    assert_eq!(
        fs.load(&expected_server_path).await.unwrap(),
        expected_binary_contents
    );
    assert_eq!(language_server_version.lock().http_request_count, 0);

    // Reload the extension, clearing its cache.
    // Start a new instance of the language server.
    extension_store
        .update(cx, |store, cx| store.reload(Some("gleam".into()), cx))
        .await;

    cx.executor().run_until_parked();
    project.update(cx, |project, cx| {
        project.restart_language_servers_for_buffers([buffer.clone()], cx)
    });

    // The extension re-fetches the latest version of the language server.
    let fake_server = fake_servers.next().await.unwrap();
    let new_expected_server_path = extensions_dir.join("work/gleam/gleam-v2.0.0/gleam");
    let expected_binary_contents = language_server_version.lock().binary_contents.clone();
    assert_eq!(fake_server.binary.path, new_expected_server_path);
    assert_eq!(fake_server.binary.arguments, [OsString::from("lsp")]);
    assert_eq!(
        fs.load(&new_expected_server_path).await.unwrap(),
        expected_binary_contents
    );

    // The old language server directory has been cleaned up.
    assert!(fs.metadata(&expected_server_path).await.unwrap().is_none());
}

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        release_channel::init("0.0.0", cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        Project::init_settings(cx);
        ExtensionSettings::register(cx);
        language::init(cx);
    });
}
