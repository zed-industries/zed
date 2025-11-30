use crate::{
    Event, ExtensionIndex, ExtensionIndexEntry, ExtensionIndexLanguageEntry,
    ExtensionIndexThemeEntry, ExtensionManifest, ExtensionStore, GrammarManifestEntry,
    RELOAD_DEBOUNCE_DURATION, SchemaVersion,
};
use async_compression::futures::bufread::GzipEncoder;
use collections::{BTreeMap, HashSet};
use extension::ExtensionHostProxy;
use fs::{FakeFs, Fs, RealFs};
use futures::{AsyncReadExt, StreamExt, io::BufReader};
use gpui::{AppContext as _, TestAppContext};
use http_client::{FakeHttpClient, Response};
use language::{BinaryStatus, LanguageMatcher, LanguageName, LanguageRegistry};
use language_extension::LspAccess;
use lsp::LanguageServerName;
use node_runtime::NodeRuntime;
use parking_lot::Mutex;
use project::{DEFAULT_COMPLETION_CONTEXT, Project};
use release_channel::AppVersion;
use reqwest_client::ReqwestClient;
use serde_json::json;
use settings::SettingsStore;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    sync::Arc,
};
use theme::ThemeRegistry;
use util::test::TempTree;

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    zlog::init_test();
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
                        icon_themes: Vec::new(),
                        lib: Default::default(),
                        languages: vec!["languages/erb".into(), "languages/ruby".into()],
                        grammars: [
                            ("embedded_template".into(), GrammarManifestEntry::default()),
                            ("ruby".into(), GrammarManifestEntry::default()),
                        ]
                        .into_iter()
                        .collect(),
                        language_servers: BTreeMap::default(),
                        context_servers: BTreeMap::default(),
                        agent_servers: BTreeMap::default(),
                        slash_commands: BTreeMap::default(),
                        snippets: None,
                        capabilities: Vec::new(),
                        debug_adapters: Default::default(),
                        debug_locators: Default::default(),
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
                        icon_themes: Vec::new(),
                        lib: Default::default(),
                        languages: Default::default(),
                        grammars: BTreeMap::default(),
                        language_servers: BTreeMap::default(),
                        context_servers: BTreeMap::default(),
                        agent_servers: BTreeMap::default(),
                        slash_commands: BTreeMap::default(),
                        snippets: None,
                        capabilities: Vec::new(),
                        debug_adapters: Default::default(),
                        debug_locators: Default::default(),
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
                    hidden: false,
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
                    hidden: false,
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
        icon_themes: BTreeMap::default(),
    };

    let proxy = Arc::new(ExtensionHostProxy::new());
    let theme_registry = Arc::new(ThemeRegistry::new(Box::new(())));
    theme_extension::init(proxy.clone(), theme_registry.clone(), cx.executor());
    let language_registry = Arc::new(LanguageRegistry::test(cx.executor()));
    language_extension::init(LspAccess::Noop, proxy.clone(), language_registry.clone());
    let node_runtime = NodeRuntime::unavailable();

    let store = cx.new(|cx| {
        ExtensionStore::new(
            PathBuf::from("/the-extension-dir"),
            None,
            proxy.clone(),
            fs.clone(),
            http_client.clone(),
            http_client.clone(),
            None,
            node_runtime.clone(),
            cx,
        )
    });

    cx.executor().advance_clock(RELOAD_DEBOUNCE_DURATION);
    store.read_with(cx, |store, _| {
        let index = &store.extension_index;
        assert_eq!(index.extensions, expected_index.extensions);

        for ((actual_key, actual_language), (expected_key, expected_language)) in
            index.languages.iter().zip(expected_index.languages.iter())
        {
            assert_eq!(actual_key, expected_key);
            assert_eq!(actual_language.grammar, expected_language.grammar);
            assert_eq!(actual_language.matcher, expected_language.matcher);
            assert_eq!(actual_language.hidden, expected_language.hidden);
        }
        assert_eq!(index.themes, expected_index.themes);

        assert_eq!(
            language_registry.language_names(),
            [
                LanguageName::new("ERB"),
                LanguageName::new("Plain Text"),
                LanguageName::new("Ruby"),
            ]
        );
        assert_eq!(
            theme_registry.list_names(),
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
                icon_themes: Vec::new(),
                lib: Default::default(),
                languages: Default::default(),
                grammars: BTreeMap::default(),
                language_servers: BTreeMap::default(),
                context_servers: BTreeMap::default(),
                agent_servers: BTreeMap::default(),
                slash_commands: BTreeMap::default(),
                snippets: None,
                capabilities: Vec::new(),
                debug_adapters: Default::default(),
                debug_locators: Default::default(),
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

    #[allow(clippy::let_underscore_future)]
    let _ = store.update(cx, |store, cx| store.reload(None, cx));

    cx.executor().advance_clock(RELOAD_DEBOUNCE_DURATION);
    store.read_with(cx, |store, _| {
        let index = &store.extension_index;

        for ((actual_key, actual_language), (expected_key, expected_language)) in
            index.languages.iter().zip(expected_index.languages.iter())
        {
            assert_eq!(actual_key, expected_key);
            assert_eq!(actual_language.grammar, expected_language.grammar);
            assert_eq!(actual_language.matcher, expected_language.matcher);
            assert_eq!(actual_language.hidden, expected_language.hidden);
        }

        assert_eq!(index.extensions, expected_index.extensions);
        assert_eq!(index.themes, expected_index.themes);

        assert_eq!(
            theme_registry.list_names(),
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
    let store = cx.new(|cx| {
        ExtensionStore::new(
            PathBuf::from("/the-extension-dir"),
            None,
            proxy,
            fs.clone(),
            http_client.clone(),
            http_client.clone(),
            None,
            node_runtime.clone(),
            cx,
        )
    });

    cx.executor().run_until_parked();
    store.read_with(cx, |store, _| {
        assert_eq!(store.extension_index.extensions, expected_index.extensions);
        assert_eq!(store.extension_index.themes, expected_index.themes);
        assert_eq!(
            store.extension_index.icon_themes,
            expected_index.icon_themes
        );

        for ((actual_key, actual_language), (expected_key, expected_language)) in store
            .extension_index
            .languages
            .iter()
            .zip(expected_index.languages.iter())
        {
            assert_eq!(actual_key, expected_key);
            assert_eq!(actual_language.grammar, expected_language.grammar);
            assert_eq!(actual_language.matcher, expected_language.matcher);
            assert_eq!(actual_language.hidden, expected_language.hidden);
        }

        assert_eq!(
            language_registry.language_names(),
            [
                LanguageName::new("ERB"),
                LanguageName::new("Plain Text"),
                LanguageName::new("Ruby"),
            ]
        );
        assert_eq!(
            language_registry.grammar_names(),
            ["embedded_template".into(), "ruby".into()]
        );
        assert_eq!(
            theme_registry.list_names(),
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
        store
            .uninstall_extension("zed-ruby".into(), cx)
            .detach_and_log_err(cx);
    });

    cx.executor().advance_clock(RELOAD_DEBOUNCE_DURATION);
    expected_index.extensions.remove("zed-ruby");
    expected_index.languages.remove("Ruby");
    expected_index.languages.remove("ERB");

    store.read_with(cx, |store, _| {
        assert_eq!(store.extension_index.extensions, expected_index.extensions);
        assert_eq!(store.extension_index.themes, expected_index.themes);
        assert_eq!(
            store.extension_index.icon_themes,
            expected_index.icon_themes
        );

        for ((actual_key, actual_language), (expected_key, expected_language)) in store
            .extension_index
            .languages
            .iter()
            .zip(expected_index.languages.iter())
        {
            assert_eq!(actual_key, expected_key);
            assert_eq!(actual_language.grammar, expected_language.grammar);
            assert_eq!(actual_language.matcher, expected_language.matcher);
            assert_eq!(actual_language.hidden, expected_language.hidden);
        }

        assert_eq!(
            language_registry.language_names(),
            [LanguageName::new("Plain Text")]
        );
        assert_eq!(language_registry.grammar_names(), []);
    });
}

#[gpui::test]
async fn test_extension_store_with_test_extension(cx: &mut TestAppContext) {
    log::info!("Initializing test");
    init_test(cx);
    cx.executor().allow_parking();

    let root_dir = Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .unwrap()
        .parent()
        .unwrap();
    let cache_dir = root_dir.join("target");
    let test_extension_id = "test-extension";
    let test_extension_dir = root_dir.join("extensions").join(test_extension_id);

    let fs = Arc::new(RealFs::new(None, cx.executor()));
    let extensions_tree = TempTree::new(json!({
        "installed": {},
        "work": {}
    }));
    let project_dir = TempTree::new(json!({
        "test.gleam": ""
    }));

    let extensions_dir = extensions_tree.path().canonicalize().unwrap();
    let project_dir = project_dir.path().canonicalize().unwrap();

    log::info!("Setting up test");

    let project = Project::test(fs.clone(), [project_dir.as_path()], cx).await;

    let proxy = Arc::new(ExtensionHostProxy::new());
    let theme_registry = Arc::new(ThemeRegistry::new(Box::new(())));
    theme_extension::init(proxy.clone(), theme_registry.clone(), cx.executor());
    let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
    language_extension::init(
        LspAccess::ViaLspStore(project.update(cx, |project, _| project.lsp_store())),
        proxy.clone(),
        language_registry.clone(),
    );
    let node_runtime = NodeRuntime::unavailable();

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

    let extension_client = FakeHttpClient::create({
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
                                    },
                                    {
                                        "name": format!("gleam-{version}-x86_64-unknown-linux-musl.tar.gz"),
                                        "browser_download_url": asset_download_uri
                                    },
                                    {
                                        "name": format!("gleam-{version}-aarch64-unknown-linux-musl.tar.gz"),
                                        "browser_download_url": asset_download_uri
                                    },
                                    {
                                        "name": format!("gleam-{version}-x86_64-pc-windows-msvc.tar.gz"),
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
    let user_agent = cx.update(|cx| {
        format!(
            "Zed/{} ({}; {})",
            AppVersion::global(cx),
            std::env::consts::OS,
            std::env::consts::ARCH
        )
    });
    let builder_client =
        Arc::new(ReqwestClient::user_agent(&user_agent).expect("Could not create HTTP client"));

    let extension_store = cx.new(|cx| {
        ExtensionStore::new(
            extensions_dir.clone(),
            Some(cache_dir),
            proxy,
            fs.clone(),
            extension_client.clone(),
            builder_client,
            None,
            node_runtime,
            cx,
        )
    });

    log::info!("Flushing events");

    // Ensure that debounces fire.
    let mut events = cx.events(&extension_store);
    let executor = cx.executor();
    let _task = cx.executor().spawn(async move {
        while let Some(event) = events.next().await {
            if let Event::StartedReloading = event {
                executor.advance_clock(RELOAD_DEBOUNCE_DURATION);
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
            store.install_dev_extension(test_extension_dir.clone(), cx)
        })
        .await
        .unwrap();

    let mut fake_servers = language_registry.register_fake_language_server(
        LanguageServerName("gleam".into()),
        lsp::ServerCapabilities {
            completion_provider: Some(Default::default()),
            ..Default::default()
        },
        None,
    );

    let (buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(project_dir.join("test.gleam"), cx)
        })
        .await
        .unwrap();

    let fake_server = fake_servers.next().await.unwrap();
    let work_dir = extensions_dir.join(format!("work/{test_extension_id}"));
    let expected_server_path = work_dir.join("gleam-v1.2.3/gleam");
    let expected_binary_contents = language_server_version.lock().binary_contents.clone();

    // check that IO operations in extension work correctly
    assert!(work_dir.join("dir-created-with-rel-path").exists());
    assert!(work_dir.join("dir-created-with-abs-path").exists());
    assert!(work_dir.join("file-created-with-abs-path").exists());
    assert!(work_dir.join("file-created-with-rel-path").exists());

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
            status_updates.next().await.unwrap(),
        ],
        [
            (
                LanguageServerName::new_static("gleam"),
                BinaryStatus::Starting
            ),
            (
                LanguageServerName::new_static("gleam"),
                BinaryStatus::CheckingForUpdate
            ),
            (
                LanguageServerName::new_static("gleam"),
                BinaryStatus::Downloading
            ),
            (LanguageServerName::new_static("gleam"), BinaryStatus::None)
        ]
    );

    // The extension creates custom labels for completion items.
    fake_server.set_request_handler::<lsp::request::Completion, _, _>(|_, _| async move {
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
        .update(cx, |project, cx| {
            project.completions(&buffer, 0, DEFAULT_COMPLETION_CONTEXT, cx)
        })
        .await
        .unwrap()
        .into_iter()
        .flat_map(|response| response.completions)
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
        project.restart_language_servers_for_buffers(vec![buffer.clone()], HashSet::default(), cx)
    });
    cx.executor().run_until_parked();

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
        .update(cx, |store, cx| {
            store.reload(Some("test-extension".into()), cx)
        })
        .await;
    cx.executor().run_until_parked();
    project.update(cx, |project, cx| {
        project.restart_language_servers_for_buffers(vec![buffer.clone()], HashSet::default(), cx)
    });

    // The extension re-fetches the latest version of the language server.
    let fake_server = fake_servers.next().await.unwrap();
    let new_expected_server_path =
        extensions_dir.join(format!("work/{test_extension_id}/gleam-v2.0.0/gleam"));
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
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        extension::init(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        gpui_tokio::init(cx);
    });
}
