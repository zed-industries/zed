use crate::{
    Event, ExtensionIndex, ExtensionIndexEntry, ExtensionIndexLanguageEntry,
    ExtensionIndexThemeEntry, ExtensionManifest, ExtensionStore, GrammarManifestEntry,
    MAX_REMOTE_SYNC_ATTEMPTS, MAX_REMOTE_SYNC_RETRY_DELAY, RELOAD_DEBOUNCE_DURATION,
    REMOTE_SYNC_TIMEOUT, SchemaVersion,
    headless_host::{
        ExtensionVersion, HeadlessExtensionStore, LoadedExtension, STALE_UPLOAD_TTL,
        hash_directory_contents, remove_stale_uploads,
    },
    load_plugin_queries, remote_sync_retry_delay,
};
use async_compression::futures::bufread::GzipEncoder;
use async_trait::async_trait;
use client::{AnyProtoClient, TypedEnvelope, proto};
use collections::{BTreeMap, HashMap, HashSet};
use extension::{
    BuildTaskTemplate, CodeLabel, Command, Completion, ContextServerConfiguration,
    DebugAdapterBinary, DebugRequest, DebugScenario, DebugTaskDefinition, Extension,
    ExtensionHostProxy, KeyValueStoreDelegate, LibManifestEntry, ProjectDelegate, SlashCommand,
    SlashCommandArgumentCompletion, SlashCommandOutput, StartDebuggingRequestArgumentsRequest,
    Symbol, WorktreeDelegate,
};
use fs::{FakeFs, Fs, RealFs, RemoveOptions};
use futures::{AsyncReadExt, FutureExt, StreamExt, io::BufReader};
use gpui::{AppContext as _, BackgroundExecutor, Entity, TaskExt, TestAppContext};
use http_client::{FakeHttpClient, Response};
use language::{
    BinaryStatus, LanguageConfig, LanguageMatcher, LanguageName, LanguageRegistry, QueryFiles,
};
use language_extension::LspAccess;
use lsp::LanguageServerName;
use node_runtime::NodeRuntime;
use parking_lot::Mutex;
use project::{DEFAULT_COMPLETION_CONTEXT, Project};
use release_channel::AppVersion;
use remote::{ConnectionState, RemoteClient, RemoteClientEvent, RemoteConnectionOptions};
use reqwest_client::ReqwestClient;
use serde_json::json;
use settings::SettingsStore;
use std::{
    ffi::OsString,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, SystemTime},
};
use task::{SpawnInTerminal, ZedDebugConfig};
use theme::ThemeRegistry;
use util::{rel_path::rel_path_buf, test::TempTree};

#[cfg(test)]
#[ctor::ctor(unsafe)]
fn init_logger() {
    zlog::init_test();
}

#[gpui::test]
async fn test_load_plugin_queries(executor: BackgroundExecutor) {
    let fs = FakeFs::new(executor);
    fs.insert_tree(
        "/queries",
        json!({
            "highlights.scm": "highlight query",
            "outline.scm": "outline query",
            "highlights_extra.scm": "ignored query",
            "locals.scm": "unrelated query",
        }),
    )
    .await;

    let queries = load_plugin_queries(
        fs.clone(),
        Path::new("/queries"),
        Some(QueryFiles::HIGHLIGHTS),
    )
    .await;
    assert_eq!(queries.highlights.as_deref(), Some("highlight query"));
    assert!(queries.outline.is_none());
    assert!(queries.brackets.is_none());

    let queries = load_plugin_queries(fs, Path::new("/queries"), None).await;
    assert_eq!(queries.highlights.as_deref(), Some("highlight query"));
    assert_eq!(queries.outline.as_deref(), Some("outline query"));
    assert!(queries.brackets.is_none());
}

fn remote_sync_entry(id: &str, manifest_body: &str) -> ExtensionIndexEntry {
    let manifest = format!(
        r#"
        id = "{id}"
        name = "{id}"
        version = "1.0.0"
        schema_version = 0

        {manifest_body}
        "#
    );

    ExtensionIndexEntry {
        manifest: Arc::new(toml::from_str(&manifest).unwrap()),
        dev: false,
    }
}

fn remote_sync_language_entry(extension: &str, path: &str) -> ExtensionIndexLanguageEntry {
    ExtensionIndexLanguageEntry {
        extension: extension.into(),
        path: path.into(),
        matcher: LanguageMatcher::default().into(),
        hidden: false,
        grammar: None,
        query_files: None,
    }
}

fn remote_sync_extension_ids(index: &ExtensionIndex) -> Vec<String> {
    let mut extensions = index
        .extensions_to_sync_to_remote()
        .into_entries()
        .map(|(id, _)| id.to_string())
        .collect::<Vec<_>>();

    extensions.sort();

    extensions
}

#[test]
fn remote_sync_includes_language_dependencies() {
    let index = ExtensionIndex {
        extensions: [
            (
                "bar-language".into(),
                remote_sync_entry("bar-language", r#"languages = ["languages/bar"]"#),
            ),
            (
                "foo-lsp".into(),
                remote_sync_entry(
                    "foo-lsp",
                    r#"
                    [language_servers.foo]
                    language = "Foo"
                    "#,
                ),
            ),
            (
                "foo-language".into(),
                remote_sync_entry("foo-language", r#"languages = ["languages/foo"]"#),
            ),
        ]
        .into_iter()
        .collect(),
        languages: [
            (
                "Bar".into(),
                remote_sync_language_entry("bar-language", "languages/bar"),
            ),
            (
                "Foo".into(),
                remote_sync_language_entry("foo-language", "languages/foo"),
            ),
        ]
        .into_iter()
        .collect(),
        themes: BTreeMap::default(),
        icon_themes: BTreeMap::default(),
    };

    assert_eq!(
        remote_sync_extension_ids(&index),
        ["foo-language", "foo-lsp"]
    );
}

#[test]
fn remote_sync_keeps_shared_language_dependency_once() {
    let index = ExtensionIndex {
        extensions: [
            (
                "aaa-lsp".into(),
                remote_sync_entry(
                    "aaa-lsp",
                    r#"
                    [language_servers.aaa]
                    language = "Foo"
                    "#,
                ),
            ),
            (
                "bbb-lsp".into(),
                remote_sync_entry(
                    "bbb-lsp",
                    r#"
                    [language_servers.bbb]
                    language = "Foo"
                    "#,
                ),
            ),
            (
                "zzz-language".into(),
                remote_sync_entry("zzz-language", r#"languages = ["languages/foo"]"#),
            ),
        ]
        .into_iter()
        .collect(),
        languages: [(
            "Foo".into(),
            remote_sync_language_entry("zzz-language", "languages/foo"),
        )]
        .into_iter()
        .collect(),
        themes: BTreeMap::default(),
        icon_themes: BTreeMap::default(),
    };

    assert_eq!(
        remote_sync_extension_ids(&index),
        ["aaa-lsp", "bbb-lsp", "zzz-language"]
    );
}

#[test]
fn remote_sync_keeps_remote_loadable_extensions_without_language_dependency() {
    let index = ExtensionIndex {
        extensions: [(
            "foo".into(),
            remote_sync_entry(
                "foo",
                r#"
                [language_servers.foo]
                language = "Foo"
                "#,
            ),
        )]
        .into_iter()
        .collect(),
        languages: BTreeMap::default(),
        themes: BTreeMap::default(),
        icon_themes: BTreeMap::default(),
    };

    assert_eq!(remote_sync_extension_ids(&index), ["foo"]);
}

#[test]
fn remote_sync_keeps_debug_adapters() {
    let index = ExtensionIndex {
        extensions: [(
            "foo".into(),
            remote_sync_entry(
                "foo",
                r#"
                [debug_adapters.foo]
                "#,
            ),
        )]
        .into_iter()
        .collect(),
        languages: BTreeMap::default(),
        themes: BTreeMap::default(),
        icon_themes: BTreeMap::default(),
    };

    assert_eq!(remote_sync_extension_ids(&index), ["foo"]);
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
                            "outline.scm": "",
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
                        languages: vec![
                            rel_path_buf("languages/erb"),
                            rel_path_buf("languages/ruby"),
                        ],
                        grammars: [
                            ("embedded_template".into(), GrammarManifestEntry::default()),
                            ("ruby".into(), GrammarManifestEntry::default()),
                        ]
                        .into_iter()
                        .collect(),
                        language_servers: BTreeMap::default(),
                        context_servers: BTreeMap::default(),
                        slash_commands: BTreeMap::default(),
                        snippets: None,
                        capabilities: Vec::new(),
                        debug_adapters: Default::default(),
                        debug_locators: Default::default(),
                        language_model_providers: BTreeMap::default(),
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
                            rel_path_buf("themes/monokai-pro.json"),
                            rel_path_buf("themes/monokai.json"),
                        ],
                        icon_themes: Vec::new(),
                        lib: Default::default(),
                        languages: Default::default(),
                        grammars: BTreeMap::default(),
                        language_servers: BTreeMap::default(),
                        context_servers: BTreeMap::default(),
                        slash_commands: BTreeMap::default(),
                        snippets: None,
                        capabilities: Vec::new(),
                        debug_adapters: Default::default(),
                        debug_locators: Default::default(),
                        language_model_providers: BTreeMap::default(),
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
                    matcher: (LanguageMatcher {
                        path_suffixes: vec!["erb".into()],
                        first_line_pattern: None,
                        ..LanguageMatcher::default()
                    })
                    .into(),
                    query_files: Some(QueryFiles::HIGHLIGHTS),
                },
            ),
            (
                "Ruby".into(),
                ExtensionIndexLanguageEntry {
                    extension: "zed-ruby".into(),
                    path: "languages/ruby".into(),
                    grammar: Some("ruby".into()),
                    hidden: false,
                    matcher: (LanguageMatcher {
                        path_suffixes: vec!["rb".into()],
                        first_line_pattern: None,
                        ..LanguageMatcher::default()
                    })
                    .into(),
                    query_files: Some(QueryFiles::HIGHLIGHTS | QueryFiles::OUTLINE),
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
                LanguageName::new_static("ERB"),
                LanguageName::new_static("Plain Text"),
                LanguageName::new_static("Ruby"),
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
                themes: vec![rel_path_buf("themes/gruvbox.json")],
                icon_themes: Vec::new(),
                lib: Default::default(),
                languages: Default::default(),
                grammars: BTreeMap::default(),
                language_servers: BTreeMap::default(),
                context_servers: BTreeMap::default(),
                slash_commands: BTreeMap::default(),
                snippets: None,
                capabilities: Vec::new(),
                debug_adapters: Default::default(),
                debug_locators: Default::default(),
                language_model_providers: BTreeMap::default(),
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
                LanguageName::new_static("ERB"),
                LanguageName::new_static("Plain Text"),
                LanguageName::new_static("Ruby"),
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
            [LanguageName::new_static("Plain Text")]
        );
        assert_eq!(language_registry.grammar_names(), []);
    });
}

#[gpui::test]
async fn test_extension_store_with_test_extension(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let executor = cx.executor();
    async fn await_or_timeout<T>(
        executor: &BackgroundExecutor,
        what: &'static str,
        seconds: u64,
        future: impl std::future::Future<Output = T>,
    ) -> T {
        let timeout = executor.timer(std::time::Duration::from_secs(seconds));

        futures::select! {
            output = future.fuse() => output,
            _ = futures::FutureExt::fuse(timeout) => panic!(
            "[test_extension_store_with_test_extension] timed out after {seconds}s while {what}"
        )
        }
    }

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

    let project = await_or_timeout(
        &executor,
        "awaiting Project::test",
        5,
        Project::test(fs.clone(), [project_dir.as_path()], cx),
    )
    .await;

    let proxy = Arc::new(ExtensionHostProxy::new());
    let theme_registry = Arc::new(ThemeRegistry::new(Box::new(())));
    theme_extension::init(proxy.clone(), theme_registry.clone(), cx.executor());
    let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
    language_extension::init(
        LspAccess::ViaLspStore(
            project
                .update(cx, |project, _| project.lsp_store())
                .downgrade(),
        ),
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

    let mut extension_events = cx.events(&cx.update(|cx| {
        extension::ExtensionEvents::try_global(cx)
            .expect("ExtensionEvents should be initialized in tests")
    }));

    let executor = cx.executor();
    await_or_timeout(
        &executor,
        "awaiting install_dev_extension",
        60,
        extension_store.update(cx, |store, cx| {
            store.install_dev_extension(test_extension_dir.clone(), cx)
        }),
    )
    .await
    .unwrap();

    await_or_timeout(
        &executor,
        "awaiting ExtensionsInstalledChanged",
        10,
        async {
            while let Some(event) = extension_events.next().await {
                if matches!(event, extension::Event::ExtensionsInstalledChanged) {
                    return;
                }
            }

            panic!(
                "[test_extension_store_with_test_extension] extension event stream ended before ExtensionsInstalledChanged"
            );
        },
    )
    .await;

    let mut fake_servers = language_registry.register_fake_lsp_server(
        LanguageServerName("gleam".into()),
        lsp::ServerCapabilities {
            completion_provider: Some(Default::default()),
            ..Default::default()
        },
        None,
    );
    cx.executor().run_until_parked();

    let mut project_events = cx.events(&project);
    let buffer_path = project_dir.join("test.gleam");
    let (buffer, _handle) = await_or_timeout(
        &executor,
        "awaiting open_local_buffer_with_lsp",
        5,
        project.update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(buffer_path.clone(), cx)
        }),
    )
    .await
    .unwrap();
    cx.executor().run_until_parked();

    let buffer_remote_id = buffer.read_with(cx, |buffer, _cx| buffer.remote_id());

    let fake_server = await_or_timeout(
        &executor,
        "awaiting first fake server spawn",
        10,
        fake_servers.next(),
    )
    .await
    .unwrap();

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
        await_or_timeout(
            &executor,
            "awaiting fs.load(expected_server_path)",
            5,
            fs.load(&expected_server_path)
        )
        .await
        .unwrap(),
        expected_binary_contents
    );
    assert_eq!(language_server_version.lock().http_request_count, 2);
    assert_eq!(
        [
            await_or_timeout(
                &executor,
                "awaiting status_updates #1",
                5,
                status_updates.next()
            )
            .await
            .unwrap(),
            await_or_timeout(
                &executor,
                "awaiting status_updates #2",
                5,
                status_updates.next()
            )
            .await
            .unwrap(),
            await_or_timeout(
                &executor,
                "awaiting status_updates #3",
                5,
                status_updates.next()
            )
            .await
            .unwrap(),
            await_or_timeout(
                &executor,
                "awaiting status_updates #4",
                5,
                status_updates.next()
            )
            .await
            .unwrap(),
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

    // `register_fake_lsp_server` can yield a server instance before the client has fully registered
    // the buffer with the project LSP plumbing. Wait for the project to observe that registration
    // before issuing requests like completion.
    await_or_timeout(
        &executor,
        "awaiting LanguageServerBufferRegistered",
        5,
        async {
            while let Some(event) = project_events.next().await {
                if let project::Event::LanguageServerBufferRegistered { buffer_id, .. } = event {
                    if buffer_id == buffer_remote_id {
                        return;
                    }
                }
            }

            panic!(
                "[test_extension_store_with_test_extension] project event stream ended before buffer registration for {}",
                buffer_path.display()
            );
        },
    )
    .await;

    let completion_labels = await_or_timeout(
        &executor,
        "awaiting completions",
        5,
        project.update(cx, |project, cx| {
            project.completions(&buffer, 0, DEFAULT_COMPLETION_CONTEXT, cx)
        }),
    )
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
        project.restart_language_servers_for_buffers(
            vec![buffer.clone()],
            HashSet::default(),
            true,
            cx,
        )
    });
    cx.executor().run_until_parked();

    // The extension has cached the binary path, and does not attempt
    // to reinstall it.
    let fake_server = await_or_timeout(
        &executor,
        "awaiting second fake server spawn",
        5,
        fake_servers.next(),
    )
    .await
    .unwrap();
    assert_eq!(fake_server.binary.path, expected_server_path);
    assert_eq!(
        await_or_timeout(
            &executor,
            "awaiting fs.load(expected_server_path) after restart",
            5,
            fs.load(&expected_server_path)
        )
        .await
        .unwrap(),
        expected_binary_contents
    );
    assert_eq!(language_server_version.lock().http_request_count, 0);

    // Reload the extension, clearing its cache.
    // Start a new instance of the language server.
    await_or_timeout(
        &executor,
        "awaiting extension_store.reload(test-extension)",
        5,
        extension_store.update(cx, |store, cx| {
            store.reload(Some("test-extension".into()), cx)
        }),
    )
    .await;
    cx.executor().run_until_parked();
    project.update(cx, |project, cx| {
        project.restart_language_servers_for_buffers(
            vec![buffer.clone()],
            HashSet::default(),
            true,
            cx,
        )
    });

    // The extension re-fetches the latest version of the language server.
    let fake_server = await_or_timeout(
        &executor,
        "awaiting third fake server spawn",
        5,
        fake_servers.next(),
    )
    .await
    .unwrap();
    let new_expected_server_path =
        extensions_dir.join(format!("work/{test_extension_id}/gleam-v2.0.0/gleam"));
    let expected_binary_contents = language_server_version.lock().binary_contents.clone();
    assert_eq!(fake_server.binary.path, new_expected_server_path);
    assert_eq!(fake_server.binary.arguments, [OsString::from("lsp")]);
    assert_eq!(
        await_or_timeout(
            &executor,
            "awaiting fs.load(new_expected_server_path)",
            5,
            fs.load(&new_expected_server_path)
        )
        .await
        .unwrap(),
        expected_binary_contents
    );

    // The old language server directory has been cleaned up.
    assert!(
        await_or_timeout(
            &executor,
            "awaiting fs.metadata(expected_server_path)",
            5,
            fs.metadata(&expected_server_path)
        )
        .await
        .unwrap()
        .is_none()
    );
}

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let store = SettingsStore::test(cx);
        cx.set_global(store);
        release_channel::init(semver::Version::new(0, 0, 0), cx);
        extension::init(cx);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        gpui_tokio::init(cx);
    });
}

#[gpui::test]
async fn test_register_remote_client_syncs_only_the_new_client(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);
    let (store, _fs) = create_extension_store(cx);
    insert_remote_sync_index_entry(&store, cx);

    let (opts_a, _counter_a, sync_count_a) = setup_mock_remote(cx, server_cx);
    let (opts_b, _counter_b, sync_count_b) = setup_mock_remote(cx, server_cx);

    let client_a = RemoteClient::connect_mock(opts_a, cx).await;
    let client_b = RemoteClient::connect_mock(opts_b, cx).await;

    store.update(cx, |store, cx| {
        store.register_remote_client(client_a.clone(), cx)
    });
    cx.run_until_parked();
    assert_eq!(
        sync_count_a.load(Ordering::SeqCst),
        1,
        "registering a client should sync extensions to it once"
    );
    assert_eq!(sync_count_b.load(Ordering::SeqCst), 0);

    store.update(cx, |store, cx| {
        store.register_remote_client(client_b.clone(), cx)
    });
    cx.run_until_parked();
    assert_eq!(
        sync_count_a.load(Ordering::SeqCst),
        1,
        "registering a new client should not re-sync already-registered clients"
    );
    assert_eq!(
        sync_count_b.load(Ordering::SeqCst),
        1,
        "registering a client should sync extensions to it once"
    );

    store.update(cx, |store, cx| {
        store.register_remote_client(client_a.clone(), cx)
    });
    cx.run_until_parked();
    assert_eq!(
        sync_count_a.load(Ordering::SeqCst),
        1,
        "re-registering an already-registered client should be a no-op"
    );
}

#[gpui::test]
async fn test_register_remote_client_resyncs_extensions_on_reconnect(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);
    let (store, _fs) = create_extension_store(cx);
    insert_remote_sync_index_entry(&store, cx);

    let (opts, _counter, sync_count) = setup_mock_remote(cx, server_cx);
    let (other_opts, _other_counter, other_sync_count) = setup_mock_remote(cx, server_cx);
    let client = RemoteClient::connect_mock(opts, cx).await;
    let other_client = RemoteClient::connect_mock(other_opts, cx).await;
    store.update(cx, |store, cx| {
        store.register_remote_client(client.clone(), cx);
        store.register_remote_client(other_client.clone(), cx);
    });
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        1,
        "registering a remote client should sync extensions to it once"
    );
    assert_eq!(other_sync_count.load(Ordering::SeqCst), 1);

    client.update(cx, |_client, cx| {
        cx.emit(RemoteClientEvent::Reconnected);
    });
    cx.run_until_parked();

    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        2,
        "reconnecting should re-sync extensions to the remote client"
    );
    assert_eq!(
        other_sync_count.load(Ordering::SeqCst),
        1,
        "reconnecting one client should not re-sync other clients"
    );
}

#[gpui::test]
async fn test_register_remote_client_retries_failed_sync(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);
    let (store, _fs) = create_extension_store(cx);
    insert_remote_sync_index_entry(&store, cx);

    let (opts, _counter, sync_count) = setup_mock_remote_with_failures(cx, server_cx, 1);
    let client = RemoteClient::connect_mock(opts, cx).await;

    store.update(cx, |store, cx| {
        store.register_remote_client(client.clone(), cx)
    });
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        1,
        "registering a client should attempt an initial sync"
    );

    cx.executor().advance_clock(remote_sync_retry_delay(0));
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        2,
        "a failed sync should be retried after the retry delay"
    );

    cx.executor().advance_clock(remote_sync_retry_delay(1));
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        2,
        "a successful sync should not be retried"
    );
}

#[gpui::test]
async fn test_persistent_sync_failure_retries_with_capped_backoff(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);
    let (store, _fs) = create_extension_store(cx);
    insert_remote_sync_index_entry(&store, cx);

    let (opts, _counter, sync_count) = setup_mock_remote_with_failures(cx, server_cx, usize::MAX);
    let client = RemoteClient::connect_mock(opts, cx).await;

    store.update(cx, |store, cx| {
        store.register_remote_client(client.clone(), cx)
    });
    cx.run_until_parked();
    assert_eq!(sync_count.load(Ordering::SeqCst), 1);

    advance_through_sync_backoff(cx);
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        9,
        "every backoff period should trigger exactly one retry"
    );

    assert_eq!(remote_sync_retry_delay(8), MAX_REMOTE_SYNC_RETRY_DELAY);
    cx.executor().advance_clock(MAX_REMOTE_SYNC_RETRY_DELAY);
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        MAX_REMOTE_SYNC_ATTEMPTS,
        "a persistently failing sync should keep retrying once per capped backoff period until the attempt limit"
    );

    cx.executor().advance_clock(MAX_REMOTE_SYNC_RETRY_DELAY);
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        MAX_REMOTE_SYNC_ATTEMPTS,
        "after the attempt limit is reached, retries should pause until the next change"
    );

    client.update(cx, |_client, cx| {
        cx.emit(RemoteClientEvent::Reconnected);
    });
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        MAX_REMOTE_SYNC_ATTEMPTS + 1,
        "a connection change should resume syncing after retries were exhausted"
    );
}

#[gpui::test]
async fn test_register_remote_client_drops_stale_retry_after_successful_sync(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);
    let (store, _fs) = create_extension_store(cx);

    let (opts, _counter, sync_count) = setup_mock_remote_with_failures(cx, server_cx, 1);
    let client = RemoteClient::connect_mock(opts, cx).await;

    store.update(cx, |store, cx| {
        store.register_remote_client(client.clone(), cx)
    });
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        1,
        "registering a client should attempt an initial sync"
    );

    client.update(cx, |_client, cx| {
        cx.emit(RemoteClientEvent::Reconnected);
    });
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        2,
        "reconnecting should trigger a sync"
    );

    cx.executor().advance_clock(remote_sync_retry_delay(0));
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        2,
        "a pending retry should be dropped once a later sync succeeded"
    );
}

#[gpui::test]
async fn test_register_remote_client_keeps_subscription_across_disconnect(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);
    let (store, _fs) = create_extension_store(cx);

    let (opts, _counter, sync_count) = setup_mock_remote_with_failures(cx, server_cx, 1);
    let client = RemoteClient::connect_mock(opts, cx).await;

    store.update(cx, |store, cx| {
        store.register_remote_client(client.clone(), cx)
    });
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        1,
        "registering a client should attempt an initial sync"
    );

    client.update(cx, |_client, cx| {
        cx.emit(RemoteClientEvent::Disconnected {
            server_not_running: false,
        });
    });
    cx.run_until_parked();

    store.read_with(cx, |store, _cx| {
        assert_eq!(
            store.remote_clients.len(),
            1,
            "a disconnected client should stay registered so a later reconnect can resync"
        );
    });

    client.update(cx, |_client, cx| {
        cx.emit(RemoteClientEvent::Reconnected);
    });
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        2,
        "reconnecting after a disconnect should resync the client"
    );
}

#[gpui::test]
async fn test_register_remote_client_release_evicts_client(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);
    let (store, _fs) = create_extension_store(cx);

    let (opts, _counter, _sync_count) = setup_mock_remote(cx, server_cx);
    let client = RemoteClient::connect_mock(opts, cx).await;

    store.update(cx, |store, cx| {
        store.register_remote_client(client.clone(), cx)
    });
    cx.run_until_parked();
    assert_eq!(
        store.read_with(cx, |store, _cx| store.remote_clients.len()),
        1
    );

    cx.update(move |_cx| drop(client));
    cx.run_until_parked();

    store.read_with(cx, |store, _cx| {
        assert_eq!(
            store.remote_clients.len(),
            0,
            "a released client should be evicted along with its subscriptions and reconciler"
        );
    });
}

#[gpui::test]
async fn test_register_remote_client_resyncs_after_simulated_reconnect(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);
    let (store, _fs) = create_extension_store(cx);

    let (opts, _counter, sync_count) = setup_mock_remote(cx, server_cx);
    let client = RemoteClient::connect_mock(opts, cx).await;

    store.update(cx, |store, cx| {
        store.register_remote_client(client.clone(), cx)
    });
    cx.run_until_parked();
    assert_eq!(sync_count.load(Ordering::SeqCst), 1);

    client
        .update(cx, |client, cx| client.simulate_disconnect(cx))
        .detach();
    cx.run_until_parked();

    let mut reconnected_and_resynced = false;
    for _ in 0..30 {
        cx.executor().advance_clock(Duration::from_secs(10));
        cx.run_until_parked();
        let connected = client.read_with(cx, |client, _cx| {
            client.connection_state() == ConnectionState::Connected
        });
        if connected && sync_count.load(Ordering::SeqCst) >= 2 {
            reconnected_and_resynced = true;
            break;
        }
    }
    assert!(
        reconnected_and_resynced,
        "a simulated disconnect should reconnect and resync without deadlocking the reconciler"
    );
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        2,
        "a real reconnect should trigger exactly one resync"
    );
}

#[gpui::test]
async fn test_headless_sync_extensions_notifications(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/extensions/foo-dev",
        json!({
            "extension.toml": r#"
id = "foo-dev"
name = "foo-dev"
version = "1.0.0"
schema_version = 1
languages = ["languages/foo"]
"#,
            "languages": {
                "foo": {
                    "config.toml": r#"
name = "Foo"
grammar = "foo"
path_suffixes = ["foo"]
"#
                }
            }
        }),
    )
    .await;

    let store = cx.update(|cx| {
        HeadlessExtensionStore::new(
            fs.clone(),
            FakeHttpClient::with_200_response(),
            PathBuf::from("/extensions"),
            Arc::new(ExtensionHostProxy::new()),
            NodeRuntime::unavailable(),
            cx,
        )
    });

    let installed_changed_count = Arc::new(AtomicUsize::new(0));
    cx.update(|cx| {
        let extension_events = extension::ExtensionEvents::try_global(cx)
            .expect("ExtensionEvents should be initialized in tests");
        let installed_changed_count = installed_changed_count.clone();
        cx.subscribe(&extension_events, move |_, event, _cx| {
            if matches!(event, extension::Event::ExtensionsInstalledChanged) {
                installed_changed_count.fetch_add(1, Ordering::SeqCst);
            }
        })
        .detach();
    });

    let dev_extension = || {
        vec![ExtensionVersion {
            id: "foo-dev".to_string(),
            version: "1.0.0".to_string(),
            dev: true,
            content_fingerprint: None,
        }]
    };

    let missing = store
        .update(cx, |store, cx| store.sync_extensions(dev_extension(), cx))
        .await
        .unwrap();
    cx.run_until_parked();
    assert_eq!(missing.len(), 1, "dev extensions should always be re-sent");
    assert_eq!(
        installed_changed_count.load(Ordering::SeqCst),
        1,
        "loading a not-yet-loaded extension should notify"
    );

    let missing = store
        .update(cx, |store, cx| store.sync_extensions(dev_extension(), cx))
        .await
        .unwrap();
    cx.run_until_parked();
    assert_eq!(missing.len(), 1, "dev extensions should always be re-sent");
    assert_eq!(
        installed_changed_count.load(Ordering::SeqCst),
        1,
        "re-syncing an unchanged dev extension should not reload it or notify"
    );

    fs.insert_file(
        "/extensions/foo-dev/languages/foo/config.toml",
        br#"
name = "Foo"
grammar = "foo"
path_suffixes = ["foo", "foo2"]
"#
        .to_vec(),
    )
    .await;

    let missing = store
        .update(cx, |store, cx| store.sync_extensions(dev_extension(), cx))
        .await
        .unwrap();
    cx.run_until_parked();
    assert_eq!(missing.len(), 1, "dev extensions should always be re-sent");
    assert_eq!(
        installed_changed_count.load(Ordering::SeqCst),
        2,
        "reloading a dev extension whose content changed should notify"
    );

    store
        .update(cx, |store, cx| store.sync_extensions(Vec::new(), cx))
        .await
        .unwrap();
    cx.run_until_parked();
    assert_eq!(
        installed_changed_count.load(Ordering::SeqCst),
        3,
        "unloading an extension should notify"
    );
}

#[gpui::test]
async fn test_headless_sync_skips_dev_extension_with_matching_fingerprint(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/extensions/foo-dev",
        json!({
            "extension.toml": r#"
id = "foo-dev"
name = "foo-dev"
version = "1.0.0"
schema_version = 1
languages = ["languages/foo"]
"#,
            "languages": {
                "foo": {
                    "config.toml": "name = \"Foo\"\ngrammar = \"foo\"\npath_suffixes = [\"foo\"]\n"
                }
            }
        }),
    )
    .await;

    let store = cx.update(|cx| {
        HeadlessExtensionStore::new(
            fs.clone(),
            FakeHttpClient::with_200_response(),
            PathBuf::from("/extensions"),
            Arc::new(ExtensionHostProxy::new()),
            NodeRuntime::unavailable(),
            cx,
        )
    });

    let dev_extension = |content_fingerprint: Option<u64>| {
        vec![ExtensionVersion {
            id: "foo-dev".to_string(),
            version: "1.0.0".to_string(),
            dev: true,
            content_fingerprint,
        }]
    };

    let missing = store
        .update(cx, |store, cx| {
            store.sync_extensions(dev_extension(None), cx)
        })
        .await
        .unwrap();
    assert_eq!(
        missing.len(),
        1,
        "a dev extension without a client fingerprint should be re-requested"
    );

    let fingerprint = store
        .read_with(cx, |store, _cx| {
            store
                .loaded_extensions
                .get("foo-dev")
                .and_then(|loaded| loaded.content_fingerprint)
        })
        .expect("loading a dev extension should record its content fingerprint");

    let missing = store
        .update(cx, |store, cx| {
            store.sync_extensions(dev_extension(Some(fingerprint)), cx)
        })
        .await
        .unwrap();
    assert_eq!(
        missing.len(),
        0,
        "a dev extension whose fingerprint matches the loaded content should not be re-requested"
    );

    let missing = store
        .update(cx, |store, cx| {
            store.sync_extensions(dev_extension(Some(fingerprint.wrapping_add(1))), cx)
        })
        .await
        .unwrap();
    assert_eq!(
        missing.len(),
        1,
        "a dev extension whose fingerprint differs from the loaded content should be re-requested"
    );
}

#[gpui::test]
async fn test_headless_sync_extensions_reports_failed_load_as_missing_again(
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/extensions", json!({})).await;

    let store = cx.update(|cx| {
        HeadlessExtensionStore::new(
            fs.clone(),
            FakeHttpClient::with_200_response(),
            PathBuf::from("/extensions"),
            Arc::new(ExtensionHostProxy::new()),
            NodeRuntime::unavailable(),
            cx,
        )
    });

    let broken_extension = || {
        vec![ExtensionVersion {
            id: "broken".to_string(),
            version: "1.0.0".to_string(),
            dev: false,
            content_fingerprint: None,
        }]
    };

    let missing = store
        .update(cx, |store, cx| {
            store.sync_extensions(broken_extension(), cx)
        })
        .await
        .unwrap();
    assert_eq!(
        missing.len(),
        1,
        "a failed load should be reported as missing"
    );

    let loaded_count = store.read_with(cx, |store, _cx| store.loaded_extensions.len());
    assert_eq!(
        loaded_count, 0,
        "a failed load should not mark the extension as loaded"
    );

    let missing = store
        .update(cx, |store, cx| {
            store.sync_extensions(broken_extension(), cx)
        })
        .await
        .unwrap();
    assert_eq!(
        missing.len(),
        1,
        "an extension that failed to load should be reported as missing on the next sync too"
    );
}

#[gpui::test]
async fn test_headless_sync_uninstall_failure_does_not_block_other_extensions(
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let extension_files = |id: &str| {
        json!({
            "extension.toml": format!(
                "id = \"{id}\"\nname = \"{id}\"\nversion = \"1.0.0\"\nschema_version = 1\nlanguages = [\"languages/lang\"]\n"
            ),
            "languages": {
                "lang": {
                    "config.toml": format!(
                        "name = \"{id}-lang\"\ngrammar = \"{id}\"\npath_suffixes = [\"{id}\"]\n"
                    )
                }
            }
        })
    };

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/extensions",
        json!({
            "ext-a": extension_files("ext-a"),
            "ext-b": extension_files("ext-b"),
            "ext-c": extension_files("ext-c"),
        }),
    )
    .await;

    let store = cx.update(|cx| {
        HeadlessExtensionStore::new(
            fs.clone(),
            FakeHttpClient::with_200_response(),
            PathBuf::from("/extensions"),
            Arc::new(ExtensionHostProxy::new()),
            NodeRuntime::unavailable(),
            cx,
        )
    });

    let extension = |id: &str| ExtensionVersion {
        id: id.to_string(),
        version: "1.0.0".to_string(),
        dev: false,
        content_fingerprint: None,
    };

    store
        .update(cx, |store, cx| {
            store.sync_extensions(vec![extension("ext-a"), extension("ext-b")], cx)
        })
        .await
        .unwrap();

    fs.set_remove_dir_error("/extensions/ext-a", "simulated removal failure".to_string());

    let missing = store
        .update(cx, |store, cx| {
            store.sync_extensions(vec![extension("ext-b"), extension("ext-c")], cx)
        })
        .await
        .unwrap();
    assert_eq!(
        missing.len(),
        0,
        "a failed removal should not fail the sync or block loading new extensions"
    );

    store.read_with(cx, |store, _cx| {
        let mut ids = store.loaded_extensions.keys().cloned().collect::<Vec<_>>();
        ids.sort();
        assert_eq!(
            ids,
            vec![Arc::<str>::from("ext-b"), Arc::<str>::from("ext-c")],
            "the extension whose files failed to be removed should still be evicted, and the new extension loaded"
        );
    });
}

#[gpui::test]
async fn test_extension_index_change_sync_failure_is_retried(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);
    let (store, fs) = create_extension_store(cx);

    let (opts, counter, sync_count) = setup_mock_remote(cx, server_cx);
    let client = RemoteClient::connect_mock(opts, cx).await;

    store.update(cx, |store, cx| {
        store.register_remote_client(client.clone(), cx)
    });
    cx.run_until_parked();
    assert_eq!(sync_count.load(Ordering::SeqCst), 1);

    counter.update(server_cx, |counter, _cx| counter.failures_remaining = 1);

    insert_remote_relevant_extension(&fs, "sync-ext").await;
    store.update(cx, |store, cx| drop(store.reload(None, cx)));
    cx.executor().advance_clock(RELOAD_DEBOUNCE_DURATION);
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        2,
        "an extension index change should sync extensions to registered clients"
    );

    cx.executor().advance_clock(remote_sync_retry_delay(0));
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        3,
        "a failed index-change sync should be retried after the retry delay"
    );

    cx.executor().advance_clock(remote_sync_retry_delay(1));
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        3,
        "a successful retry should not be retried again"
    );
}

#[gpui::test]
async fn test_register_remote_client_reconnect_does_not_start_parallel_retry_chain(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);
    let (store, _fs) = create_extension_store(cx);

    let (opts, _counter, sync_count) = setup_mock_remote_with_failures(cx, server_cx, usize::MAX);
    let client = RemoteClient::connect_mock(opts, cx).await;

    store.update(cx, |store, cx| {
        store.register_remote_client(client.clone(), cx)
    });
    cx.run_until_parked();
    assert_eq!(sync_count.load(Ordering::SeqCst), 1);

    client.update(cx, |_client, cx| {
        cx.emit(RemoteClientEvent::Reconnected);
    });
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        2,
        "reconnecting should attempt a sync even while a retry is pending"
    );

    cx.executor().advance_clock(remote_sync_retry_delay(0));
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        3,
        "after a reconnect, each backoff period should trigger exactly one retry, not one per chain"
    );

    cx.executor().advance_clock(remote_sync_retry_delay(1));
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        4,
        "retries should stay serialized in a single chain"
    );
}

#[gpui::test]
async fn test_index_change_during_backoff_neither_preempts_nor_resets_it(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);
    let (store, fs) = create_extension_store(cx);

    let (opts, _counter, sync_count) = setup_mock_remote_with_failures(cx, server_cx, usize::MAX);
    let client = RemoteClient::connect_mock(opts, cx).await;

    store.update(cx, |store, cx| {
        store.register_remote_client(client.clone(), cx)
    });
    cx.run_until_parked();
    assert_eq!(sync_count.load(Ordering::SeqCst), 1);

    insert_remote_relevant_extension(&fs, "sync-ext").await;
    store.update(cx, |store, cx| drop(store.reload(None, cx)));
    cx.executor().advance_clock(RELOAD_DEBOUNCE_DURATION);
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        1,
        "an index change should not preempt the pending backoff timer"
    );

    cx.executor().advance_clock(remote_sync_retry_delay(0));
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        2,
        "the pending retry should fire on schedule and pick up the index change"
    );

    cx.executor().advance_clock(remote_sync_retry_delay(0));
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        2,
        "an index change should not reset the backoff chain to the initial delay"
    );

    cx.executor().advance_clock(remote_sync_retry_delay(1));
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        3,
        "the backoff chain should continue growing across index changes"
    );
}

#[gpui::test]
async fn test_index_change_after_exhaustion_triggers_single_attempt(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);
    let (store, fs) = create_extension_store(cx);

    let (opts, _counter, sync_count) = setup_mock_remote_with_failures(cx, server_cx, usize::MAX);
    let client = RemoteClient::connect_mock(opts, cx).await;

    store.update(cx, |store, cx| {
        store.register_remote_client(client.clone(), cx)
    });
    cx.run_until_parked();

    advance_through_sync_backoff(cx);
    cx.executor().advance_clock(MAX_REMOTE_SYNC_RETRY_DELAY);
    cx.run_until_parked();
    assert_eq!(sync_count.load(Ordering::SeqCst), MAX_REMOTE_SYNC_ATTEMPTS);

    insert_remote_relevant_extension(&fs, "sync-ext").await;
    store.update(cx, |store, cx| drop(store.reload(None, cx)));
    cx.executor().advance_clock(RELOAD_DEBOUNCE_DURATION);
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        MAX_REMOTE_SYNC_ATTEMPTS + 1,
        "after exhaustion, an index change should trigger exactly one attempt"
    );

    cx.executor().advance_clock(MAX_REMOTE_SYNC_RETRY_DELAY);
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        MAX_REMOTE_SYNC_ATTEMPTS + 1,
        "a failed post-exhaustion attempt should park again instead of restarting the chain"
    );

    client.update(cx, |_client, cx| {
        cx.emit(RemoteClientEvent::Reconnected);
    });
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        MAX_REMOTE_SYNC_ATTEMPTS + 2,
        "a reconnect should restart the retry chain after exhaustion"
    );

    cx.executor().advance_clock(remote_sync_retry_delay(0));
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        MAX_REMOTE_SYNC_ATTEMPTS + 3,
        "the restarted chain should retry with the initial delay"
    );
}

#[gpui::test]
async fn test_remote_sync_failed_install_is_retried(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);
    let (store, _fs) = create_extension_store(cx);

    store.update(cx, |store, _cx| {
        for id in ["bar-lsp", "foo-lsp"] {
            let mut entry = remote_sync_entry(
                id,
                r#"
                [language_servers.foo]
                language = "Foo"
                "#,
            );
            entry.dev = true;
            store
                .extension_index
                .extensions
                .insert(Arc::from(id), entry);
        }
    });

    let (opts, _counter, sync_count, install_count) =
        setup_mock_remote_with_install_failures(cx, server_cx, 1);
    let client = RemoteClient::connect_mock(opts, cx).await;

    store.update(cx, |store, cx| {
        store.register_remote_client(client.clone(), cx)
    });
    cx.run_until_parked();
    assert_eq!(sync_count.load(Ordering::SeqCst), 1);
    assert_eq!(
        install_count.load(Ordering::SeqCst),
        2,
        "one failed install should not prevent installing the remaining extensions"
    );

    cx.executor().advance_clock(remote_sync_retry_delay(0));
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        2,
        "a sync with a failed extension install should be retried"
    );
    assert_eq!(
        install_count.load(Ordering::SeqCst),
        4,
        "the retried sync should reattempt all missing installs"
    );

    cx.executor().advance_clock(remote_sync_retry_delay(1));
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        2,
        "a sync whose installs all succeeded should not be retried"
    );
}

#[gpui::test]
async fn test_unchanged_dev_extension_is_not_reuploaded_on_resync(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);
    let (store, _fs) = create_extension_store(cx);

    let insert_dev_entry = |store: &Entity<ExtensionStore>, cx: &mut TestAppContext, body: &str| {
        let mut entry = remote_sync_entry("foo-lsp", body);
        entry.dev = true;
        store.update(cx, |store, _cx| {
            store
                .extension_index
                .extensions
                .insert(Arc::from("foo-lsp"), entry);
        });
    };
    insert_dev_entry(&store, cx, "[language_servers.foo]\nlanguage = \"Foo\"");

    let (opts, _counter, sync_count, install_count) =
        setup_mock_remote_tracking_fingerprints(cx, server_cx);
    let client = RemoteClient::connect_mock(opts, cx).await;

    store.update(cx, |store, cx| {
        store.register_remote_client(client.clone(), cx)
    });
    cx.run_until_parked();
    assert_eq!(sync_count.load(Ordering::SeqCst), 1);
    assert_eq!(
        install_count.load(Ordering::SeqCst),
        1,
        "the initial sync should upload the dev extension"
    );

    client.update(cx, |_client, cx| {
        cx.emit(RemoteClientEvent::Reconnected);
    });
    cx.run_until_parked();
    assert_eq!(sync_count.load(Ordering::SeqCst), 2);
    assert_eq!(
        install_count.load(Ordering::SeqCst),
        1,
        "a resync with unchanged dev extension content should not re-upload it"
    );

    insert_dev_entry(
        &store,
        cx,
        "[language_servers.foo]\nlanguage = \"Foo\"\n\n[language_servers.bar]\nlanguage = \"Bar\"",
    );
    client.update(cx, |_client, cx| {
        cx.emit(RemoteClientEvent::Reconnected);
    });
    cx.run_until_parked();
    assert_eq!(sync_count.load(Ordering::SeqCst), 3);
    assert_eq!(
        install_count.load(Ordering::SeqCst),
        2,
        "changed dev extension content should be re-uploaded"
    );
}

#[gpui::test]
async fn test_remote_sync_hang_times_out_and_is_retried(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);
    let (store, _fs) = create_extension_store(cx);

    let (opts, _counter, sync_count) = setup_mock_remote_hanging(cx, server_cx);
    let client = RemoteClient::connect_mock(opts, cx).await;

    store.update(cx, |store, cx| {
        store.register_remote_client(client.clone(), cx)
    });
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        1,
        "registering a client should attempt an initial sync"
    );

    cx.executor().advance_clock(REMOTE_SYNC_TIMEOUT);
    cx.run_until_parked();
    cx.executor().advance_clock(remote_sync_retry_delay(0));
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        2,
        "a hung sync should time out and be retried"
    );
}

#[gpui::test]
async fn test_headless_failed_reload_keeps_other_extensions_languages(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/extensions",
        json!({
            "ext-a": {
                "extension.toml": r#"
id = "ext-a"
name = "ext-a"
version = "1.0.0"
schema_version = 1
languages = ["languages/shared"]
"#,
                "languages": {
                    "shared": {
                        "config.toml": "name = \"Shared\"\ngrammar = \"shared\"\npath_suffixes = [\"shared\"]\n"
                    }
                }
            },
            "ext-b": {
                "extension.toml": r#"
id = "ext-b"
name = "ext-b"
version = "1.0.0"
schema_version = 1
languages = ["languages/foo"]
"#,
                "languages": {
                    "foo": {
                        "config.toml": "name = \"Foo\"\ngrammar = \"foo\"\npath_suffixes = [\"foo\"]\n"
                    }
                }
            }
        }),
    )
    .await;

    let language_registry = Arc::new(LanguageRegistry::test(cx.executor()));
    let proxy = Arc::new(ExtensionHostProxy::new());
    language_extension::init(LspAccess::Noop, proxy.clone(), language_registry.clone());

    let store = cx.update(|cx| {
        HeadlessExtensionStore::new(
            fs.clone(),
            FakeHttpClient::with_200_response(),
            PathBuf::from("/extensions"),
            proxy,
            NodeRuntime::unavailable(),
            cx,
        )
    });

    let extensions = || {
        vec![
            ExtensionVersion {
                id: "ext-a".to_string(),
                version: "1.0.0".to_string(),
                dev: false,
                content_fingerprint: None,
            },
            ExtensionVersion {
                id: "ext-b".to_string(),
                version: "1.0.0".to_string(),
                dev: true,
                content_fingerprint: None,
            },
        ]
    };

    store
        .update(cx, |store, cx| store.sync_extensions(extensions(), cx))
        .await
        .unwrap();
    assert_eq!(
        language_registry.language_names(),
        vec![
            LanguageName::new("Foo"),
            LanguageName::new("Plain Text"),
            LanguageName::new("Shared"),
        ],
    );

    fs.insert_tree(
        "/extensions/ext-b/languages",
        json!({
            "shared": {
                "config.toml": "name = \"Shared\"\ngrammar = \"shared\"\npath_suffixes = [\"shared\"]\n"
            },
            "broken": {
                "config.toml": "not valid toml ["
            }
        }),
    )
    .await;
    fs.insert_file(
        "/extensions/ext-b/extension.toml",
        br#"
id = "ext-b"
name = "ext-b"
version = "1.0.0"
schema_version = 1
languages = ["languages/foo", "languages/shared", "languages/broken"]
"#
        .to_vec(),
    )
    .await;

    let missing = store
        .update(cx, |store, cx| store.sync_extensions(extensions(), cx))
        .await
        .unwrap();
    assert_eq!(
        missing.len(),
        1,
        "a failed dev extension reload should be reported as missing"
    );
    assert_eq!(
        language_registry.language_names(),
        vec![
            LanguageName::new("Foo"),
            LanguageName::new("Plain Text"),
            LanguageName::new("Shared"),
        ],
        "a failed reload of one extension should not deregister languages owned by other extensions"
    );
}

#[gpui::test]
async fn test_headless_dev_reload_deregisters_dropped_languages(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/extensions/foo-dev",
        json!({
            "extension.toml": r#"
id = "foo-dev"
name = "foo-dev"
version = "1.0.0"
schema_version = 1
languages = ["languages/foo", "languages/bar"]
"#,
            "languages": {
                "foo": {
                    "config.toml": "name = \"Foo\"\ngrammar = \"foo\"\npath_suffixes = [\"foo\"]\n"
                },
                "bar": {
                    "config.toml": "name = \"Bar\"\ngrammar = \"bar\"\npath_suffixes = [\"bar\"]\n"
                }
            }
        }),
    )
    .await;

    let language_registry = Arc::new(LanguageRegistry::test(cx.executor()));
    let proxy = Arc::new(ExtensionHostProxy::new());
    language_extension::init(LspAccess::Noop, proxy.clone(), language_registry.clone());

    let store = cx.update(|cx| {
        HeadlessExtensionStore::new(
            fs.clone(),
            FakeHttpClient::with_200_response(),
            PathBuf::from("/extensions"),
            proxy,
            NodeRuntime::unavailable(),
            cx,
        )
    });

    let dev_extension = || {
        vec![ExtensionVersion {
            id: "foo-dev".to_string(),
            version: "1.0.0".to_string(),
            dev: true,
            content_fingerprint: None,
        }]
    };

    store
        .update(cx, |store, cx| store.sync_extensions(dev_extension(), cx))
        .await
        .unwrap();
    assert_eq!(
        language_registry.language_names(),
        vec![
            LanguageName::new("Bar"),
            LanguageName::new("Foo"),
            LanguageName::new("Plain Text"),
        ],
    );

    fs.insert_file(
        "/extensions/foo-dev/extension.toml",
        br#"
id = "foo-dev"
name = "foo-dev"
version = "1.0.0"
schema_version = 1
languages = ["languages/foo"]
"#
        .to_vec(),
    )
    .await;

    store
        .update(cx, |store, cx| store.sync_extensions(dev_extension(), cx))
        .await
        .unwrap();
    assert_eq!(
        language_registry.language_names(),
        vec![LanguageName::new("Foo"), LanguageName::new("Plain Text")],
        "a dev reload that drops a language should deregister it"
    );
    store.read_with(cx, |store, _cx| {
        assert_eq!(
            store
                .loaded_extensions
                .get("foo-dev")
                .map(|extension| extension.languages.len()),
            Some(1)
        );
    });
}

#[gpui::test]
async fn test_headless_uninstall_restores_surviving_extensions_language_config(
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let shared_language_extension = |id: &str, suffix: &str| {
        json!({
            "extension.toml": format!(
                "id = \"{id}\"\nname = \"{id}\"\nversion = \"1.0.0\"\nschema_version = 1\nlanguages = [\"languages/shared\"]\n"
            ),
            "languages": {
                "shared": {
                    "config.toml": format!(
                        "name = \"Shared\"\ngrammar = \"shared\"\npath_suffixes = [\"{suffix}\"]\n"
                    )
                }
            }
        })
    };

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/extensions",
        json!({
            "ext-a": shared_language_extension("ext-a", "shared-a"),
            "ext-c": shared_language_extension("ext-c", "shared-c"),
        }),
    )
    .await;

    let language_registry = Arc::new(LanguageRegistry::test(cx.executor()));
    let proxy = Arc::new(ExtensionHostProxy::new());
    language_extension::init(LspAccess::Noop, proxy.clone(), language_registry.clone());

    let store = cx.update(|cx| {
        HeadlessExtensionStore::new(
            fs.clone(),
            FakeHttpClient::with_200_response(),
            PathBuf::from("/extensions"),
            proxy,
            NodeRuntime::unavailable(),
            cx,
        )
    });

    let extension = |id: &str| ExtensionVersion {
        id: id.to_string(),
        version: "1.0.0".to_string(),
        dev: false,
        content_fingerprint: None,
    };

    store
        .update(cx, |store, cx| {
            store.sync_extensions(vec![extension("ext-a"), extension("ext-c")], cx)
        })
        .await
        .unwrap();

    store
        .update(cx, |store, cx| {
            store.sync_extensions(vec![extension("ext-a")], cx)
        })
        .await
        .unwrap();
    assert_eq!(
        language_registry.language_name_for_extension("shared-a"),
        Some(LanguageName::new("Shared")),
        "uninstalling an extension should restore the surviving extension's config for a shared language"
    );
    assert_eq!(
        language_registry.language_name_for_extension("shared-c"),
        None,
        "the uninstalled extension's config should no longer be registered"
    );
}

#[gpui::test]
async fn test_headless_failed_reload_restores_previous_language_config(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/extensions/foo-dev",
        json!({
            "extension.toml": r#"
id = "foo-dev"
name = "foo-dev"
version = "1.0.0"
schema_version = 1
languages = ["languages/foo"]
"#,
            "languages": {
                "foo": {
                    "config.toml": "name = \"Foo\"\ngrammar = \"foo\"\npath_suffixes = [\"foo\"]\n"
                }
            }
        }),
    )
    .await;

    let language_registry = Arc::new(LanguageRegistry::test(cx.executor()));
    let proxy = Arc::new(ExtensionHostProxy::new());
    language_extension::init(LspAccess::Noop, proxy.clone(), language_registry.clone());

    let store = cx.update(|cx| {
        HeadlessExtensionStore::new(
            fs.clone(),
            FakeHttpClient::with_200_response(),
            PathBuf::from("/extensions"),
            proxy,
            NodeRuntime::unavailable(),
            cx,
        )
    });

    let dev_extension = || {
        vec![ExtensionVersion {
            id: "foo-dev".to_string(),
            version: "1.0.0".to_string(),
            dev: true,
            content_fingerprint: None,
        }]
    };

    store
        .update(cx, |store, cx| store.sync_extensions(dev_extension(), cx))
        .await
        .unwrap();
    assert_eq!(
        language_registry.language_name_for_extension("foo"),
        Some(LanguageName::new("Foo")),
    );

    fs.insert_tree(
        "/extensions/foo-dev/languages",
        json!({
            "foo": {
                "config.toml": "name = \"Foo\"\ngrammar = \"foo\"\npath_suffixes = [\"foo2\"]\n"
            },
            "broken": {
                "config.toml": "not valid toml ["
            }
        }),
    )
    .await;
    fs.insert_file(
        "/extensions/foo-dev/extension.toml",
        br#"
id = "foo-dev"
name = "foo-dev"
version = "1.0.0"
schema_version = 1
languages = ["languages/foo", "languages/broken"]
"#
        .to_vec(),
    )
    .await;

    let missing = store
        .update(cx, |store, cx| store.sync_extensions(dev_extension(), cx))
        .await
        .unwrap();
    assert_eq!(
        missing.len(),
        1,
        "a failed dev extension reload should be reported as missing"
    );
    assert_eq!(
        language_registry.language_name_for_extension("foo"),
        Some(LanguageName::new("Foo")),
        "a failed reload should restore the previously registered language config"
    );
    assert_eq!(
        language_registry.language_name_for_extension("foo2"),
        None,
        "the failed reload's partially applied language config should be rolled back"
    );
    store.read_with(cx, |store, _cx| {
        let loaded = store
            .loaded_extensions
            .get("foo-dev")
            .expect("a failed reload should keep the extension loaded");
        assert_eq!(
            loaded.version.as_ref(),
            "1.0.0",
            "a failed reload should keep the previously loaded version"
        );
        assert_eq!(
            loaded.languages.len(),
            1,
            "a failed reload should not duplicate or drop language registrations"
        );
    });
}

#[gpui::test]
async fn test_headless_failed_reinstall_restores_previous_registrations(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/extensions/foo-dev",
        json!({
            "extension.toml": r#"
id = "foo-dev"
name = "foo-dev"
version = "1.0.0"
schema_version = 1
languages = ["languages/foo"]
"#,
            "languages": {
                "foo": {
                    "config.toml": "name = \"Foo\"\ngrammar = \"foo\"\npath_suffixes = [\"foo\"]\n"
                }
            }
        }),
    )
    .await;
    fs.insert_tree(
        "/staged/foo-dev",
        json!({
            "extension.toml": r#"
id = "foo-dev"
name = "foo-dev"
version = "1.0.0"
schema_version = 1
languages = ["languages/foo"]
"#,
            "languages": {
                "foo": {
                    "config.toml": "not valid toml ["
                }
            }
        }),
    )
    .await;

    let language_registry = Arc::new(LanguageRegistry::test(cx.executor()));
    let proxy = Arc::new(ExtensionHostProxy::new());
    language_extension::init(LspAccess::Noop, proxy.clone(), language_registry.clone());

    let store = cx.update(|cx| {
        HeadlessExtensionStore::new(
            fs.clone(),
            FakeHttpClient::with_200_response(),
            PathBuf::from("/extensions"),
            proxy,
            NodeRuntime::unavailable(),
            cx,
        )
    });

    let dev_extension = ExtensionVersion {
        id: "foo-dev".to_string(),
        version: "1.0.0".to_string(),
        dev: true,
        content_fingerprint: None,
    };

    store
        .update(cx, |store, cx| {
            store.sync_extensions(vec![dev_extension.clone()], cx)
        })
        .await
        .unwrap();
    assert_eq!(
        language_registry.language_name_for_extension("foo"),
        Some(LanguageName::new("Foo")),
    );

    let install_result = store
        .update(cx, |store, cx| {
            store.install_extension(dev_extension, PathBuf::from("/staged/foo-dev"), cx)
        })
        .await;
    assert!(
        install_result.is_err(),
        "installing a broken extension should fail"
    );

    assert_eq!(
        language_registry.language_name_for_extension("foo"),
        Some(LanguageName::new("Foo")),
        "a failed reinstall should leave the previous registrations untouched"
    );
    store.read_with(cx, |store, _cx| {
        assert_eq!(
            store
                .loaded_extensions
                .get("foo-dev")
                .map(|extension| extension.languages.len()),
            Some(1),
            "a failed reinstall should keep the previously loaded languages"
        );
        assert_eq!(
            store
                .loaded_extensions
                .get("foo-dev")
                .map(|extension| extension.version.as_ref()),
            Some("1.0.0"),
            "a failed reinstall should keep the previous version loaded"
        );
    });
    assert!(
        fs.is_dir(Path::new("/extensions/foo-dev")).await,
        "a failed reinstall should not touch the installed files"
    );
    assert!(
        !fs.is_dir(Path::new("/staged/foo-dev")).await,
        "a failed reinstall should clean up the staged upload"
    );
}

#[gpui::test]
async fn test_remove_stale_uploads_removes_only_stale_entries(cx: &mut TestAppContext) {
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/uploads/stale-upload",
        json!({
            "extension.toml": "id = \"stale\""
        }),
    )
    .await;
    fs.set_next_mtime(SystemTime::now() + STALE_UPLOAD_TTL * 2);
    fs.insert_tree(
        "/uploads/far-future-upload",
        json!({
            "extension.toml": "id = \"future\""
        }),
    )
    .await;
    fs.set_next_mtime(SystemTime::now());
    fs.insert_tree(
        "/uploads/fresh-upload",
        json!({
            "extension.toml": "id = \"fresh\""
        }),
    )
    .await;

    let fs_trait_object = fs.clone() as Arc<dyn Fs>;
    remove_stale_uploads(&fs_trait_object, Path::new("/uploads")).await;

    assert!(
        !fs.is_dir(Path::new("/uploads/stale-upload")).await,
        "an upload directory older than the TTL should be removed"
    );
    assert!(
        !fs.is_dir(Path::new("/uploads/far-future-upload")).await,
        "an upload directory with an mtime far in the future should be treated as stale"
    );
    assert!(
        fs.is_dir(Path::new("/uploads/fresh-upload")).await,
        "a recently modified upload directory should be kept"
    );
}

#[gpui::test]
async fn test_headless_dev_reload_replaces_registrations(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/extensions", json!({})).await;

    let language_registry = Arc::new(LanguageRegistry::test(cx.executor()));
    let proxy = Arc::new(ExtensionHostProxy::new());
    language_extension::init(LspAccess::Noop, proxy.clone(), language_registry.clone());

    let store = cx.update(|cx| {
        HeadlessExtensionStore::new(
            fs.clone(),
            FakeHttpClient::with_200_response(),
            PathBuf::from("/extensions"),
            proxy,
            NodeRuntime::unavailable(),
            cx,
        )
    });

    let loaded_extension = || LoadedExtension {
        version: "1.0.0".into(),
        languages: vec![(
            LanguageName::new("Foo"),
            LanguageConfig {
                name: LanguageName::new("Foo"),
                ..LanguageConfig::default()
            },
        )],
        language_servers: vec![(
            LanguageServerName("foo-lsp".into()),
            LanguageName::new("Foo"),
        )],
        debug_adapters: Vec::new(),
        debug_locators: Vec::new(),
        wasm_extension: Some(Arc::new(FakeExtension)),
        content_fingerprint: None,
    };
    let foo = LanguageName::new("Foo");

    let removal_tasks = store.update(cx, |store, cx| {
        store.commit_extension("foo-ext".into(), Some(loaded_extension()), cx)
    });
    assert_eq!(
        removal_tasks.len(),
        0,
        "the first load should not stop any language servers"
    );
    assert_eq!(language_registry.lsp_adapters(&foo).len(), 1);
    assert_eq!(
        language_registry.language_names(),
        vec![LanguageName::new("Foo"), LanguageName::new("Plain Text")],
    );

    for _ in 0..2 {
        let removal_tasks = store.update(cx, |store, cx| {
            store.commit_extension("foo-ext".into(), Some(loaded_extension()), cx)
        });
        assert_eq!(
            removal_tasks.len(),
            1,
            "a reload should stop the previously running language server"
        );
        for removal in removal_tasks {
            removal.await.unwrap();
        }
        assert_eq!(
            language_registry.lsp_adapters(&foo).len(),
            1,
            "a reload should not accumulate duplicate language server registrations"
        );
        assert_eq!(
            language_registry.language_names(),
            vec![LanguageName::new("Foo"), LanguageName::new("Plain Text")],
            "a reload should not accumulate duplicate language registrations"
        );
    }

    let removal_tasks = store.update(cx, |store, cx| {
        store.commit_extension("foo-ext".into(), None, cx)
    });
    assert_eq!(removal_tasks.len(), 1);
    for removal in removal_tasks {
        removal.await.unwrap();
    }
    assert_eq!(
        language_registry.lsp_adapters(&foo).len(),
        0,
        "an uninstall should deregister the language server"
    );
    assert_eq!(
        language_registry.language_names(),
        vec![LanguageName::new("Plain Text")],
        "an uninstall should deregister the language"
    );
}

#[gpui::test]
async fn test_headless_install_failure_after_removing_old_files_evicts_extension(
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let extension_files = |suffix: &str| {
        json!({
            "extension.toml": r#"
id = "foo"
name = "foo"
version = "1.0.0"
schema_version = 1
languages = ["languages/foo"]
"#,
            "languages": {
                "foo": {
                    "config.toml": format!(
                        "name = \"Foo\"\ngrammar = \"foo\"\npath_suffixes = [\"{suffix}\"]\n"
                    )
                }
            }
        })
    };

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/extensions/foo", extension_files("foo"))
        .await;
    fs.insert_tree("/staged/foo", extension_files("foo2")).await;

    let store = cx.update(|cx| {
        HeadlessExtensionStore::new(
            fs.clone(),
            FakeHttpClient::with_200_response(),
            PathBuf::from("/extensions"),
            Arc::new(ExtensionHostProxy::new()),
            NodeRuntime::unavailable(),
            cx,
        )
    });

    let extension = || ExtensionVersion {
        id: "foo".to_string(),
        version: "1.0.0".to_string(),
        dev: false,
        content_fingerprint: None,
    };

    store
        .update(cx, |store, cx| store.sync_extensions(vec![extension()], cx))
        .await
        .unwrap();
    store.read_with(cx, |store, _cx| {
        assert_eq!(
            store
                .loaded_extensions
                .get("foo")
                .map(|extension| extension.version.as_ref()),
            Some("1.0.0")
        );
    });

    fs.set_remove_dir_error("/extensions/foo", "simulated removal failure".to_string());
    let install_result = store
        .update(cx, |store, cx| {
            store.install_extension(extension(), PathBuf::from("/staged/foo"), cx)
        })
        .await;
    assert!(
        install_result.is_err(),
        "an install whose removal step failed should report the error"
    );

    store.read_with(cx, |store, _cx| {
        assert_eq!(
            store
                .loaded_extensions
                .get("foo")
                .map(|extension| extension.version.as_ref()),
            None,
            "a failed install after destructive operations started should evict the extension"
        );
    });

    let missing = store
        .update(cx, |store, cx| store.sync_extensions(vec![extension()], cx))
        .await
        .unwrap();
    assert_eq!(
        missing.len(),
        0,
        "a later sync should reload the evicted extension from the intact installed files"
    );
    store.read_with(cx, |store, _cx| {
        assert_eq!(
            store
                .loaded_extensions
                .get("foo")
                .map(|extension| extension.version.as_ref()),
            Some("1.0.0"),
            "a later sync should repair the evicted extension"
        );
    });
}

#[gpui::test]
async fn test_headless_failed_install_cleanup_is_retried_on_later_syncs(cx: &mut TestAppContext) {
    init_test(cx);

    let extension_files = |suffix: &str| {
        json!({
            "extension.toml": r#"
id = "foo"
name = "foo"
version = "1.0.0"
schema_version = 1
languages = ["languages/foo"]
"#,
            "languages": {
                "foo": {
                    "config.toml": format!(
                        "name = \"Foo\"\ngrammar = \"foo\"\npath_suffixes = [\"{suffix}\"]\n"
                    )
                }
            }
        })
    };

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/extensions/foo", extension_files("foo"))
        .await;
    fs.insert_tree("/staged/foo", extension_files("foo2")).await;

    let store = cx.update(|cx| {
        HeadlessExtensionStore::new(
            fs.clone(),
            FakeHttpClient::with_200_response(),
            PathBuf::from("/extensions"),
            Arc::new(ExtensionHostProxy::new()),
            NodeRuntime::unavailable(),
            cx,
        )
    });

    let extension = || ExtensionVersion {
        id: "foo".to_string(),
        version: "1.0.0".to_string(),
        dev: false,
        content_fingerprint: None,
    };

    store
        .update(cx, |store, cx| store.sync_extensions(vec![extension()], cx))
        .await
        .unwrap();

    fs.set_remove_dir_error("/extensions/foo", "simulated removal failure".to_string());
    let install_result = store
        .update(cx, |store, cx| {
            store.install_extension(extension(), PathBuf::from("/staged/foo"), cx)
        })
        .await;
    assert!(
        install_result.is_err(),
        "an install whose removal step failed should report the error"
    );

    store
        .update(cx, |store, cx| store.sync_extensions(Vec::new(), cx))
        .await
        .unwrap();
    assert!(
        fs.is_dir(Path::new("/extensions/foo")).await,
        "the cleanup retry should leave the files while removal keeps failing"
    );

    fs.clear_remove_dir_error("/extensions/foo");
    store
        .update(cx, |store, cx| store.sync_extensions(Vec::new(), cx))
        .await
        .unwrap();
    assert!(
        !fs.is_dir(Path::new("/extensions/foo")).await,
        "a later sync should retry and complete the cleanup of a failed install"
    );
}

#[gpui::test]
async fn test_headless_unchanged_reinstall_skips_reload(cx: &mut TestAppContext) {
    init_test(cx);

    let extension_files = |suffix: &str| {
        json!({
            "extension.toml": r#"
id = "foo-dev"
name = "foo-dev"
version = "1.0.0"
schema_version = 1
languages = ["languages/foo"]
"#,
            "languages": {
                "foo": {
                    "config.toml": format!(
                        "name = \"Foo\"\ngrammar = \"foo\"\npath_suffixes = [\"{suffix}\"]\n"
                    )
                }
            }
        })
    };

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/extensions/foo-dev", extension_files("foo"))
        .await;
    fs.insert_tree("/staged/foo-dev", extension_files("foo"))
        .await;

    let store = cx.update(|cx| {
        HeadlessExtensionStore::new(
            fs.clone(),
            FakeHttpClient::with_200_response(),
            PathBuf::from("/extensions"),
            Arc::new(ExtensionHostProxy::new()),
            NodeRuntime::unavailable(),
            cx,
        )
    });

    let installed_changed_count = Arc::new(AtomicUsize::new(0));
    cx.update(|cx| {
        let extension_events = extension::ExtensionEvents::try_global(cx)
            .expect("ExtensionEvents should be initialized in tests");
        let installed_changed_count = installed_changed_count.clone();
        cx.subscribe(&extension_events, move |_, event, _cx| {
            if matches!(event, extension::Event::ExtensionsInstalledChanged) {
                installed_changed_count.fetch_add(1, Ordering::SeqCst);
            }
        })
        .detach();
    });

    let dev_extension = || ExtensionVersion {
        id: "foo-dev".to_string(),
        version: "1.0.0".to_string(),
        dev: true,
        content_fingerprint: None,
    };

    store
        .update(cx, |store, cx| {
            store.sync_extensions(vec![dev_extension()], cx)
        })
        .await
        .unwrap();
    cx.run_until_parked();
    assert_eq!(installed_changed_count.load(Ordering::SeqCst), 1);

    store
        .update(cx, |store, cx| {
            store.install_extension(dev_extension(), PathBuf::from("/staged/foo-dev"), cx)
        })
        .await
        .unwrap();
    cx.run_until_parked();
    assert_eq!(
        installed_changed_count.load(Ordering::SeqCst),
        1,
        "reinstalling identical content should not reload the extension or notify"
    );
    assert!(
        !fs.is_dir(Path::new("/staged/foo-dev")).await,
        "a skipped install should clean up the staged upload"
    );
    assert!(
        fs.is_dir(Path::new("/extensions/foo-dev")).await,
        "a skipped install should leave the installed files untouched"
    );
    store.read_with(cx, |store, _cx| {
        assert_eq!(
            store
                .loaded_extensions
                .get("foo-dev")
                .map(|extension| extension.version.as_ref()),
            Some("1.0.0"),
            "a skipped install should keep the extension loaded"
        );
    });

    fs.insert_tree("/staged/foo-dev", extension_files("foo2"))
        .await;
    store
        .update(cx, |store, cx| {
            store.install_extension(dev_extension(), PathBuf::from("/staged/foo-dev"), cx)
        })
        .await
        .unwrap();
    cx.run_until_parked();
    assert_eq!(
        installed_changed_count.load(Ordering::SeqCst),
        2,
        "reinstalling changed content should reload the extension and notify"
    );
}

#[gpui::test]
async fn test_headless_failed_uninstall_is_retried_on_later_syncs(cx: &mut TestAppContext) {
    init_test(cx);

    let extension_files = |id: &str| {
        json!({
            "extension.toml": format!(
                "id = \"{id}\"\nname = \"{id}\"\nversion = \"1.0.0\"\nschema_version = 1\nlanguages = [\"languages/lang\"]\n"
            ),
            "languages": {
                "lang": {
                    "config.toml": format!(
                        "name = \"{id}-lang\"\ngrammar = \"{id}\"\npath_suffixes = [\"{id}\"]\n"
                    )
                }
            }
        })
    };

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/extensions",
        json!({
            "ext-a": extension_files("ext-a"),
            "ext-b": extension_files("ext-b"),
            "ext-c": extension_files("ext-c"),
        }),
    )
    .await;

    let store = cx.update(|cx| {
        HeadlessExtensionStore::new(
            fs.clone(),
            FakeHttpClient::with_200_response(),
            PathBuf::from("/extensions"),
            Arc::new(ExtensionHostProxy::new()),
            NodeRuntime::unavailable(),
            cx,
        )
    });

    let extension = |id: &str| ExtensionVersion {
        id: id.to_string(),
        version: "1.0.0".to_string(),
        dev: false,
        content_fingerprint: None,
    };

    store
        .update(cx, |store, cx| {
            store.sync_extensions(
                vec![extension("ext-a"), extension("ext-b"), extension("ext-c")],
                cx,
            )
        })
        .await
        .unwrap();

    fs.set_remove_dir_error("/extensions/ext-a", "simulated removal failure".to_string());
    fs.set_remove_dir_error("/extensions/ext-c", "simulated removal failure".to_string());
    store
        .update(cx, |store, cx| {
            store.sync_extensions(vec![extension("ext-b")], cx)
        })
        .await
        .unwrap();
    store.read_with(cx, |store, _cx| {
        assert_eq!(
            store.loaded_extensions.keys().cloned().collect::<Vec<_>>(),
            vec![Arc::<str>::from("ext-b")],
            "failed removals should still evict the extensions"
        );
    });
    assert!(
        fs.is_dir(Path::new("/extensions/ext-a")).await,
        "a failed removal should leave the files on disk"
    );

    let missing = store
        .update(cx, |store, cx| {
            store.sync_extensions(vec![extension("ext-b"), extension("ext-c")], cx)
        })
        .await
        .unwrap();
    assert_eq!(
        missing.len(),
        0,
        "an extension pending removal that the client re-requests should be reloaded from the intact files instead"
    );
    store.read_with(cx, |store, _cx| {
        let mut ids = store.loaded_extensions.keys().cloned().collect::<Vec<_>>();
        ids.sort();
        assert_eq!(
            ids,
            vec![Arc::<str>::from("ext-b"), Arc::<str>::from("ext-c")]
        );
    });
    assert!(
        fs.is_dir(Path::new("/extensions/ext-a")).await,
        "a removal that keeps failing should keep the files on disk"
    );

    fs.clear_remove_dir_error("/extensions/ext-a");
    store
        .update(cx, |store, cx| {
            store.sync_extensions(vec![extension("ext-b"), extension("ext-c")], cx)
        })
        .await
        .unwrap();
    assert!(
        !fs.is_dir(Path::new("/extensions/ext-a")).await,
        "a later sync should retry and complete the failed removal"
    );
}

#[gpui::test]
async fn test_index_change_without_remote_relevant_extensions_does_not_sync(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) {
    init_test(cx);
    let (store, fs) = create_extension_store(cx);

    let (opts, _counter, sync_count) = setup_mock_remote(cx, server_cx);
    let client = RemoteClient::connect_mock(opts, cx).await;

    store.update(cx, |store, cx| {
        store.register_remote_client(client.clone(), cx)
    });
    cx.run_until_parked();
    assert_eq!(sync_count.load(Ordering::SeqCst), 1);

    fs.insert_tree(
        "/extensions/installed/plain-ext",
        json!({
            "extension.toml": "id = \"plain-ext\"\nname = \"plain-ext\"\nversion = \"1.0.0\"\nschema_version = 1\n",
        }),
    )
    .await;
    store.update(cx, |store, cx| drop(store.reload(None, cx)));
    cx.executor().advance_clock(RELOAD_DEBOUNCE_DURATION);
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        1,
        "an index change that does not affect remote-synced extensions should not trigger a sync"
    );

    insert_remote_relevant_extension(&fs, "sync-ext").await;
    store.update(cx, |store, cx| drop(store.reload(None, cx)));
    cx.executor().advance_clock(RELOAD_DEBOUNCE_DURATION);
    cx.run_until_parked();
    assert_eq!(
        sync_count.load(Ordering::SeqCst),
        2,
        "an index change that affects remote-synced extensions should trigger a sync"
    );
}

#[gpui::test]
async fn test_uninstalling_extension_restores_surviving_extensions_language(
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    let language_registry = Arc::new(LanguageRegistry::test(cx.executor()));
    let proxy = Arc::new(ExtensionHostProxy::new());
    language_extension::init(LspAccess::Noop, proxy.clone(), language_registry.clone());

    insert_language_extension(&fs, "ext-a", "Shared", "shared-a").await;
    insert_language_extension(&fs, "ext-b", "Shared", "shared-b").await;

    let store = create_extension_store_with(fs.clone(), proxy, cx);
    assert_eq!(
        language_registry.language_name_for_extension("shared-b"),
        Some(LanguageName::new("Shared")),
        "the extension scanned last should own the language in the index"
    );

    fs.remove_dir(
        Path::new("/extensions/installed/ext-b"),
        RemoveOptions {
            recursive: true,
            ignore_if_not_exists: false,
        },
    )
    .await
    .unwrap();
    store.update(cx, |store, cx| drop(store.reload(None, cx)));
    cx.executor().advance_clock(RELOAD_DEBOUNCE_DURATION);
    cx.run_until_parked();

    assert_eq!(
        language_registry.language_name_for_extension("shared-a"),
        Some(LanguageName::new("Shared")),
        "uninstalling the owning extension should re-register the language from the surviving extension"
    );
    assert_eq!(
        language_registry.language_name_for_extension("shared-b"),
        None,
        "the uninstalled extension's language config should be gone"
    );
}

#[gpui::test]
async fn test_extension_cannot_shadow_language_registered_outside_extensions(
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    let language_registry = Arc::new(LanguageRegistry::test(cx.executor()));
    let proxy = Arc::new(ExtensionHostProxy::new());
    language_extension::init(LspAccess::Noop, proxy.clone(), language_registry.clone());

    language_registry.register_test_language(LanguageConfig {
        name: LanguageName::new("Shared"),
        matcher: Arc::new(LanguageMatcher {
            path_suffixes: vec!["builtin".to_string()],
            ..LanguageMatcher::default()
        }),
        ..LanguageConfig::default()
    });

    let store = create_extension_store_with(fs.clone(), proxy, cx);

    insert_language_extension(&fs, "ext-a", "Shared", "shadowed").await;
    store.update(cx, |store, cx| drop(store.reload(None, cx)));
    cx.executor().advance_clock(RELOAD_DEBOUNCE_DURATION);
    cx.run_until_parked();

    assert_eq!(
        language_registry.language_name_for_extension("builtin"),
        Some(LanguageName::new("Shared")),
        "an extension must not replace a language registered outside of extensions"
    );
    assert_eq!(
        language_registry.language_name_for_extension("shadowed"),
        None,
        "the shadowing extension's language config should not be registered"
    );

    fs.remove_dir(
        Path::new("/extensions/installed/ext-a"),
        RemoveOptions {
            recursive: true,
            ignore_if_not_exists: false,
        },
    )
    .await
    .unwrap();
    store.update(cx, |store, cx| drop(store.reload(None, cx)));
    cx.executor().advance_clock(RELOAD_DEBOUNCE_DURATION);
    cx.run_until_parked();

    assert_eq!(
        language_registry.language_name_for_extension("builtin"),
        Some(LanguageName::new("Shared")),
        "uninstalling the shadowing extension must not remove the language it never registered"
    );
}

#[gpui::test]
async fn test_headless_extension_cannot_shadow_language_registered_outside_extensions(
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/extensions/ext-a",
        json!({
            "extension.toml": r#"
id = "ext-a"
name = "ext-a"
version = "1.0.0"
schema_version = 1
languages = ["languages/shared"]
"#,
            "languages": {
                "shared": {
                    "config.toml": "name = \"Shared\"\npath_suffixes = [\"shadowed\"]\n"
                }
            }
        }),
    )
    .await;

    let language_registry = Arc::new(LanguageRegistry::test(cx.executor()));
    let proxy = Arc::new(ExtensionHostProxy::new());
    language_extension::init(LspAccess::Noop, proxy.clone(), language_registry.clone());

    language_registry.register_test_language(LanguageConfig {
        name: LanguageName::new("Shared"),
        matcher: Arc::new(LanguageMatcher {
            path_suffixes: vec!["builtin".to_string()],
            ..LanguageMatcher::default()
        }),
        ..LanguageConfig::default()
    });

    let store = cx.update(|cx| {
        HeadlessExtensionStore::new(
            fs.clone(),
            FakeHttpClient::with_200_response(),
            PathBuf::from("/extensions"),
            proxy,
            NodeRuntime::unavailable(),
            cx,
        )
    });

    let extension = || {
        vec![ExtensionVersion {
            id: "ext-a".to_string(),
            version: "1.0.0".to_string(),
            dev: false,
            content_fingerprint: None,
        }]
    };

    store
        .update(cx, |store, cx| store.sync_extensions(extension(), cx))
        .await
        .unwrap();
    assert_eq!(
        language_registry.language_name_for_extension("builtin"),
        Some(LanguageName::new("Shared")),
        "an extension must not replace a language registered outside of extensions"
    );
    assert_eq!(
        language_registry.language_name_for_extension("shadowed"),
        None,
        "the shadowing extension's language config should not be registered"
    );

    store
        .update(cx, |store, cx| store.sync_extensions(Vec::new(), cx))
        .await
        .unwrap();
    assert_eq!(
        language_registry.language_name_for_extension("builtin"),
        Some(LanguageName::new("Shared")),
        "unloading the shadowing extension must not remove the language it never registered"
    );
}

#[gpui::test]
async fn test_hash_directory_contents_hashes_symlinks_by_target(cx: &mut TestAppContext) {
    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/targets",
        json!({
            "target-1": "one",
            "target-2": "two",
        }),
    )
    .await;
    fs.insert_tree(
        "/dir",
        json!({
            "file.txt": "contents",
        }),
    )
    .await;
    fs.insert_symlink("/dir/link", PathBuf::from("/targets/target-1"))
        .await;

    let fs_trait_object = fs.clone() as Arc<dyn Fs>;
    let first = hash_directory_contents(&fs_trait_object, Path::new("/dir"))
        .await
        .unwrap();
    let second = hash_directory_contents(&fs_trait_object, Path::new("/dir"))
        .await
        .unwrap();
    assert_eq!(
        first, second,
        "fingerprinting a directory containing a symlink should be deterministic"
    );

    fs.insert_symlink("/dir/link", PathBuf::from("/targets/target-2"))
        .await;
    let third = hash_directory_contents(&fs_trait_object, Path::new("/dir"))
        .await
        .unwrap();
    assert_ne!(
        first, third,
        "changing a symlink's target should change the fingerprint"
    );
}

struct SyncRequestCounter {
    sync_count: Arc<AtomicUsize>,
    failures_remaining: usize,
    installed_fingerprints: HashMap<String, Option<u64>>,
}

fn setup_mock_remote(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) -> (
    RemoteConnectionOptions,
    Entity<SyncRequestCounter>,
    Arc<AtomicUsize>,
) {
    setup_mock_remote_with_failures(cx, server_cx, 0)
}

fn setup_mock_remote_with_failures(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
    failures: usize,
) -> (
    RemoteConnectionOptions,
    Entity<SyncRequestCounter>,
    Arc<AtomicUsize>,
) {
    let sync_count = Arc::new(AtomicUsize::new(0));
    let counter = server_cx.new(|_| SyncRequestCounter {
        sync_count: sync_count.clone(),
        failures_remaining: failures,
        installed_fingerprints: HashMap::default(),
    });
    let (opts, server_client, _) = RemoteClient::fake_server(cx, server_cx);
    register_server_handlers(&server_client, counter.clone());
    (opts, counter, sync_count)
}

fn setup_mock_remote_with_install_failures(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
    failures: usize,
) -> (
    RemoteConnectionOptions,
    Entity<SyncRequestCounter>,
    Arc<AtomicUsize>,
    Arc<AtomicUsize>,
) {
    let sync_count = Arc::new(AtomicUsize::new(0));
    let install_count = Arc::new(AtomicUsize::new(0));
    let counter = server_cx.new(|_| SyncRequestCounter {
        sync_count: sync_count.clone(),
        failures_remaining: failures,
        installed_fingerprints: HashMap::default(),
    });
    let (opts, server_client, _) = RemoteClient::fake_server(cx, server_cx);
    register_ping_handler(&server_client, &counter);
    register_sync_handler(
        &server_client,
        &counter,
        SyncHandlerBehavior {
            fail_from_counter: false,
            echo_missing_extensions: true,
        },
    );
    server_client.add_request_handler::<proto::InstallExtension, SyncRequestCounter, _, _>(
        counter.downgrade(),
        {
            let install_count = install_count.clone();
            move |counter, _envelope: TypedEnvelope<proto::InstallExtension>, mut cx| {
                let install_count = install_count.clone();
                async move {
                    install_count.fetch_add(1, Ordering::SeqCst);
                    let should_fail = counter.update(&mut cx, |counter, _cx| {
                        if counter.failures_remaining > 0 {
                            counter.failures_remaining -= 1;
                            true
                        } else {
                            false
                        }
                    });
                    anyhow::ensure!(!should_fail, "simulated install failure");
                    Ok(proto::Ack {})
                }
            }
        },
    );
    (opts, counter, sync_count, install_count)
}

fn setup_mock_remote_tracking_fingerprints(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) -> (
    RemoteConnectionOptions,
    Entity<SyncRequestCounter>,
    Arc<AtomicUsize>,
    Arc<AtomicUsize>,
) {
    let sync_count = Arc::new(AtomicUsize::new(0));
    let install_count = Arc::new(AtomicUsize::new(0));
    let counter = server_cx.new(|_| SyncRequestCounter {
        sync_count: sync_count.clone(),
        failures_remaining: 0,
        installed_fingerprints: HashMap::default(),
    });
    let (opts, server_client, _) = RemoteClient::fake_server(cx, server_cx);
    register_ping_handler(&server_client, &counter);
    server_client.add_request_handler::<proto::SyncExtensions, SyncRequestCounter, _, _>(
        counter.downgrade(),
        |counter, envelope: TypedEnvelope<proto::SyncExtensions>, mut cx| async move {
            let missing_extensions = counter.update(&mut cx, |counter, _cx| {
                counter.sync_count.fetch_add(1, Ordering::SeqCst);
                envelope
                    .payload
                    .extensions
                    .into_iter()
                    .filter(|extension| {
                        let installed = counter
                            .installed_fingerprints
                            .get(&extension.id)
                            .copied()
                            .flatten();
                        extension.content_fingerprint.is_none()
                            || installed != extension.content_fingerprint
                    })
                    .collect()
            });
            Ok(proto::SyncExtensionsResponse {
                missing_extensions,
                tmp_dir: "/remote-tmp".to_string(),
            })
        },
    );
    server_client.add_request_handler::<proto::InstallExtension, SyncRequestCounter, _, _>(
        counter.downgrade(),
        {
            let install_count = install_count.clone();
            move |counter, envelope: TypedEnvelope<proto::InstallExtension>, mut cx| {
                let install_count = install_count.clone();
                async move {
                    install_count.fetch_add(1, Ordering::SeqCst);
                    if let Some(extension) = envelope.payload.extension {
                        counter.update(&mut cx, |counter, _cx| {
                            counter
                                .installed_fingerprints
                                .insert(extension.id, extension.content_fingerprint);
                        });
                    }
                    Ok(proto::Ack {})
                }
            }
        },
    );
    (opts, counter, sync_count, install_count)
}

fn setup_mock_remote_hanging(
    cx: &mut TestAppContext,
    server_cx: &mut TestAppContext,
) -> (
    RemoteConnectionOptions,
    Entity<SyncRequestCounter>,
    Arc<AtomicUsize>,
) {
    let sync_count = Arc::new(AtomicUsize::new(0));
    let counter = server_cx.new(|_| SyncRequestCounter {
        sync_count: sync_count.clone(),
        failures_remaining: 0,
        installed_fingerprints: HashMap::default(),
    });
    let (opts, server_client, _) = RemoteClient::fake_server(cx, server_cx);
    register_ping_handler(&server_client, &counter);
    server_client.add_request_handler::<proto::SyncExtensions, SyncRequestCounter, _, _>(
        counter.downgrade(),
        |counter, _envelope: TypedEnvelope<proto::SyncExtensions>, mut cx| async move {
            counter.update(&mut cx, |counter, _cx| {
                counter.sync_count.fetch_add(1, Ordering::SeqCst)
            });
            drop(counter);
            futures::future::pending::<()>().await;
            Ok(proto::SyncExtensionsResponse {
                missing_extensions: Vec::new(),
                tmp_dir: String::new(),
            })
        },
    );
    (opts, counter, sync_count)
}

fn register_ping_handler(server_client: &AnyProtoClient, counter: &Entity<SyncRequestCounter>) {
    server_client.add_request_handler::<proto::Ping, SyncRequestCounter, _, _>(
        counter.downgrade(),
        |_counter, _envelope: TypedEnvelope<proto::Ping>, _cx| async move { Ok(proto::Ack {}) },
    );
}

fn register_server_handlers(server_client: &AnyProtoClient, counter: Entity<SyncRequestCounter>) {
    register_ping_handler(server_client, &counter);
    register_sync_handler(
        server_client,
        &counter,
        SyncHandlerBehavior {
            fail_from_counter: true,
            echo_missing_extensions: false,
        },
    );
}

#[derive(Clone, Copy)]
struct SyncHandlerBehavior {
    fail_from_counter: bool,
    echo_missing_extensions: bool,
}

fn register_sync_handler(
    server_client: &AnyProtoClient,
    counter: &Entity<SyncRequestCounter>,
    behavior: SyncHandlerBehavior,
) {
    server_client.add_request_handler::<proto::SyncExtensions, SyncRequestCounter, _, _>(
        counter.downgrade(),
        move |counter, envelope: TypedEnvelope<proto::SyncExtensions>, mut cx| async move {
            let should_fail = counter.update(&mut cx, |counter, _cx| {
                counter.sync_count.fetch_add(1, Ordering::SeqCst);
                if behavior.fail_from_counter && counter.failures_remaining > 0 {
                    counter.failures_remaining -= 1;
                    true
                } else {
                    false
                }
            });
            anyhow::ensure!(!should_fail, "simulated sync failure");
            Ok(proto::SyncExtensionsResponse {
                missing_extensions: if behavior.echo_missing_extensions {
                    envelope.payload.extensions
                } else {
                    Vec::new()
                },
                tmp_dir: "/remote-tmp".to_string(),
            })
        },
    );
}

fn create_extension_store(cx: &mut TestAppContext) -> (Entity<ExtensionStore>, Arc<FakeFs>) {
    let fs = FakeFs::new(cx.executor());
    let store = create_extension_store_with(fs.clone(), Arc::new(ExtensionHostProxy::new()), cx);
    (store, fs)
}

fn insert_remote_sync_index_entry(store: &Entity<ExtensionStore>, cx: &mut TestAppContext) {
    store.update(cx, |store, _cx| {
        store.extension_index.extensions.insert(
            Arc::from("foo-lsp"),
            remote_sync_entry(
                "foo-lsp",
                r#"
                [language_servers.foo]
                language = "Foo"
                "#,
            ),
        );
    });
}

fn advance_through_sync_backoff(cx: &mut TestAppContext) {
    for attempts in 0..8 {
        cx.executor()
            .advance_clock(remote_sync_retry_delay(attempts));
        cx.run_until_parked();
    }
}

fn create_extension_store_with(
    fs: Arc<FakeFs>,
    proxy: Arc<ExtensionHostProxy>,
    cx: &mut TestAppContext,
) -> Entity<ExtensionStore> {
    let http_client = FakeHttpClient::with_200_response();
    let node_runtime = NodeRuntime::unavailable();

    let store = cx.new(|cx| {
        ExtensionStore::new(
            PathBuf::from("/extensions"),
            None,
            proxy,
            fs,
            http_client.clone(),
            http_client,
            None,
            node_runtime,
            cx,
        )
    });
    cx.run_until_parked();
    cx.executor().advance_clock(RELOAD_DEBOUNCE_DURATION);
    cx.run_until_parked();

    store
}

async fn insert_remote_relevant_extension(fs: &Arc<FakeFs>, id: &str) {
    fs.insert_tree(
        format!("/extensions/installed/{id}"),
        json!({
            "extension.toml": format!(
                "id = \"{id}\"\nname = \"{id}\"\nversion = \"1.0.0\"\nschema_version = 1\n\n[language_servers.{id}-lsp]\nlanguage = \"{id}-lang\"\n"
            ),
        }),
    )
    .await;
}

async fn insert_language_extension(fs: &Arc<FakeFs>, id: &str, language: &str, suffix: &str) {
    fs.insert_tree(
        format!("/extensions/installed/{id}"),
        json!({
            "extension.toml": format!(
                "id = \"{id}\"\nname = \"{id}\"\nversion = \"1.0.0\"\nschema_version = 1\nlanguages = [\"languages/lang\"]\n"
            ),
            "languages": {
                "lang": {
                    "config.toml": format!(
                        "name = \"{language}\"\npath_suffixes = [\"{suffix}\"]\n"
                    )
                }
            }
        }),
    )
    .await;
}

struct FakeExtension;

#[async_trait]
impl Extension for FakeExtension {
    fn manifest(&self) -> Arc<ExtensionManifest> {
        Arc::new(ExtensionManifest {
            id: "fake-extension".into(),
            name: "Fake Extension".to_string(),
            version: "1.0.0".into(),
            schema_version: SchemaVersion(1),
            description: None,
            repository: None,
            authors: Vec::new(),
            lib: LibManifestEntry::default(),
            themes: Vec::new(),
            icon_themes: Vec::new(),
            languages: Vec::new(),
            grammars: BTreeMap::default(),
            language_servers: BTreeMap::default(),
            context_servers: BTreeMap::default(),
            slash_commands: BTreeMap::default(),
            snippets: None,
            capabilities: Vec::new(),
            debug_adapters: BTreeMap::default(),
            debug_locators: BTreeMap::default(),
            language_model_providers: BTreeMap::default(),
        })
    }

    fn work_dir(&self) -> Arc<Path> {
        Arc::from(Path::new("/fake-extension-work-dir"))
    }

    async fn language_server_command(
        &self,
        _language_server_id: LanguageServerName,
        _language_name: LanguageName,
        _worktree: Arc<dyn WorktreeDelegate>,
    ) -> anyhow::Result<Command> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn language_server_initialization_options(
        &self,
        _language_server_id: LanguageServerName,
        _language_name: LanguageName,
        _worktree: Arc<dyn WorktreeDelegate>,
    ) -> anyhow::Result<Option<String>> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn language_server_workspace_configuration(
        &self,
        _language_server_id: LanguageServerName,
        _worktree: Arc<dyn WorktreeDelegate>,
    ) -> anyhow::Result<Option<String>> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn language_server_initialization_options_schema(
        &self,
        _language_server_id: LanguageServerName,
        _worktree: Arc<dyn WorktreeDelegate>,
    ) -> anyhow::Result<Option<String>> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn language_server_workspace_configuration_schema(
        &self,
        _language_server_id: LanguageServerName,
        _worktree: Arc<dyn WorktreeDelegate>,
    ) -> anyhow::Result<Option<String>> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn language_server_additional_initialization_options(
        &self,
        _language_server_id: LanguageServerName,
        _target_language_server_id: LanguageServerName,
        _worktree: Arc<dyn WorktreeDelegate>,
    ) -> anyhow::Result<Option<String>> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn language_server_additional_workspace_configuration(
        &self,
        _language_server_id: LanguageServerName,
        _target_language_server_id: LanguageServerName,
        _worktree: Arc<dyn WorktreeDelegate>,
    ) -> anyhow::Result<Option<String>> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn labels_for_completions(
        &self,
        _language_server_id: LanguageServerName,
        _completions: Vec<Completion>,
    ) -> anyhow::Result<Vec<Option<CodeLabel>>> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn labels_for_symbols(
        &self,
        _language_server_id: LanguageServerName,
        _symbols: Vec<Symbol>,
    ) -> anyhow::Result<Vec<Option<CodeLabel>>> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn complete_slash_command_argument(
        &self,
        _command: SlashCommand,
        _arguments: Vec<String>,
    ) -> anyhow::Result<Vec<SlashCommandArgumentCompletion>> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn run_slash_command(
        &self,
        _command: SlashCommand,
        _arguments: Vec<String>,
        _worktree: Option<Arc<dyn WorktreeDelegate>>,
    ) -> anyhow::Result<SlashCommandOutput> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn context_server_command(
        &self,
        _context_server_id: Arc<str>,
        _project: Arc<dyn ProjectDelegate>,
    ) -> anyhow::Result<Command> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn context_server_configuration(
        &self,
        _context_server_id: Arc<str>,
        _project: Arc<dyn ProjectDelegate>,
    ) -> anyhow::Result<Option<ContextServerConfiguration>> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn suggest_docs_packages(&self, _provider: Arc<str>) -> anyhow::Result<Vec<String>> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn index_docs(
        &self,
        _provider: Arc<str>,
        _package_name: Arc<str>,
        _kv_store: Arc<dyn KeyValueStoreDelegate>,
    ) -> anyhow::Result<()> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn get_dap_binary(
        &self,
        _dap_name: Arc<str>,
        _config: DebugTaskDefinition,
        _user_installed_path: Option<PathBuf>,
        _worktree: Arc<dyn WorktreeDelegate>,
    ) -> anyhow::Result<DebugAdapterBinary> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn dap_request_kind(
        &self,
        _dap_name: Arc<str>,
        _config: serde_json::Value,
    ) -> anyhow::Result<StartDebuggingRequestArgumentsRequest> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn dap_config_to_scenario(
        &self,
        _config: ZedDebugConfig,
    ) -> anyhow::Result<DebugScenario> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn dap_locator_create_scenario(
        &self,
        _locator_name: String,
        _build_config_template: BuildTaskTemplate,
        _resolved_label: String,
        _debug_adapter_name: String,
    ) -> anyhow::Result<Option<DebugScenario>> {
        anyhow::bail!("not supported by FakeExtension")
    }

    async fn run_dap_locator(
        &self,
        _locator_name: String,
        _config: SpawnInTerminal,
    ) -> anyhow::Result<DebugRequest> {
        anyhow::bail!("not supported by FakeExtension")
    }
}
