use std::{
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{AtomicUsize, Ordering},
    },
    time::{Duration, Instant},
};

use collections::HashMap;
use fs::FakeFs;
use futures::{FutureExt, StreamExt, lock::OwnedMutexGuard};
use gpui::{AsyncApp, TestAppContext, UpdateGlobal};
use language::{
    BinaryStatus, CodeLabel, DynLspInstaller, FakeLspAdapter, HighlightId, LanguageName,
    LanguageServerBinaryLocations, LspAdapter, LspAdapterDelegate, Toolchain, rust_lang,
};
use lsp::{LanguageServerBinary, LanguageServerBinaryOptions, LanguageServerName, Uri};
use parking_lot::Mutex;
use project::{
    Project,
    lsp_store::{
        log_store::{TestRpcLogHeaderState, TestRpcRequestTracker},
        *,
    },
};
use serde_json::json;
use settings::{LocalSettingsKind, LocalSettingsPath, Settings, SettingsStore};
use util::{path, rel_path::RelPath};

use crate::init_test;

#[derive(Clone, Default)]
struct DownloadOnlyLspAdapter {
    fetch_count: Arc<AtomicUsize>,
}

#[async_trait::async_trait(?Send)]
impl DynLspInstaller for DownloadOnlyLspAdapter {
    async fn try_fetch_server_binary(
        &self,
        _: &Arc<dyn LspAdapterDelegate>,
        _: PathBuf,
        _: bool,
        _: &mut AsyncApp,
    ) -> anyhow::Result<LanguageServerBinary> {
        unreachable!()
    }

    fn get_language_server_command(
        self: Arc<Self>,
        delegate: Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
        binary_options: LanguageServerBinaryOptions,
        _: OwnedMutexGuard<Option<(bool, LanguageServerBinary)>>,
        _: AsyncApp,
    ) -> LanguageServerBinaryLocations {
        async move {
            if !binary_options.fetch_when_missing {
                let reason = util::downloads_disabled_error(self.name().0);
                delegate.update_status(
                    self.name(),
                    BinaryStatus::DownloadBlocked {
                        reason: reason.clone(),
                    },
                );
                return (Err(anyhow::anyhow!(reason)), None);
            }

            self.fetch_count.fetch_add(1, Ordering::SeqCst);
            (
                Ok(LanguageServerBinary {
                    path: "/downloaded/lsp".into(),
                    arguments: Vec::new(),
                    env: None,
                }),
                None,
            )
        }
        .boxed_local()
    }
}

impl LspAdapter for DownloadOnlyLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName::new_static("download-only-language-server")
    }

    fn language_ids(&self) -> collections::HashMap<LanguageName, String> {
        collections::HashMap::from_iter([("Rust".into(), "rust".to_string())])
    }
}

#[derive(Clone, Default)]
struct DiskBinaryWithDownloaderLspAdapter {
    fetch_count: Arc<AtomicUsize>,
}

#[async_trait::async_trait(?Send)]
impl DynLspInstaller for DiskBinaryWithDownloaderLspAdapter {
    async fn try_fetch_server_binary(
        &self,
        _: &Arc<dyn LspAdapterDelegate>,
        _: PathBuf,
        _: bool,
        _: &mut AsyncApp,
    ) -> anyhow::Result<LanguageServerBinary> {
        unreachable!()
    }

    fn get_language_server_command(
        self: Arc<Self>,
        _: Arc<dyn LspAdapterDelegate>,
        _: Option<Toolchain>,
        _: LanguageServerBinaryOptions,
        _: OwnedMutexGuard<Option<(bool, LanguageServerBinary)>>,
        _: AsyncApp,
    ) -> LanguageServerBinaryLocations {
        async move {
            let fetch_count = self.fetch_count.clone();
            (
                Ok(LanguageServerBinary {
                    path: "/existing/lsp".into(),
                    arguments: Vec::new(),
                    env: None,
                }),
                Some(
                    async move {
                        fetch_count.fetch_add(1, Ordering::SeqCst);
                        Ok(LanguageServerBinary {
                            path: "/downloaded/lsp".into(),
                            arguments: Vec::new(),
                            env: None,
                        })
                    }
                    .boxed_local(),
                ),
            )
        }
        .boxed_local()
    }
}

impl LspAdapter for DiskBinaryWithDownloaderLspAdapter {
    fn name(&self) -> LanguageServerName {
        LanguageServerName::new_static("disk-binary-language-server")
    }

    fn language_ids(&self) -> collections::HashMap<LanguageName, String> {
        collections::HashMap::from_iter([("Rust".into(), "rust".to_string())])
    }

    fn is_extension(&self) -> bool {
        true
    }
}

#[gpui::test]
async fn test_allow_binary_downloads_false_holds_lsp_until_allowed(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    cx.update(|cx| project::binary_downloads::init(cx));

    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(false);
            });
        });
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/the-root"), json!({ "main.rs": "fn main() {}" }))
        .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let adapter = DownloadOnlyLspAdapter::default();
    let adapter_name = adapter.name();
    language_registry.register_lsp_adapter("Rust".into(), Arc::new(adapter));
    let mut fake_servers = language_registry.register_fake_lsp_server(
        adapter_name.clone(),
        lsp::ServerCapabilities::default(),
        None,
    );

    let (_buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();

    let mut next_server = fake_servers.next().fuse();
    let mut timeout = cx.executor().timer(Duration::from_secs(1)).fuse();
    futures::select! {
        _ = next_server => panic!("language server started while downloads were disabled"),
        _ = timeout => {}
    }

    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(true);
            });
        });
    });

    let mut next_server = fake_servers.next().fuse();
    let mut timeout = cx.executor().timer(Duration::from_secs(1)).fuse();
    futures::select! {
        server = next_server => assert_eq!(server.is_some(), true),
        _ = timeout => panic!("timed out waiting for language server after enabling downloads"),
    }
}

#[gpui::test]
async fn test_allow_binary_downloads_can_be_enabled_for_a_project(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    cx.update(|cx| project::binary_downloads::init(cx));

    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(false);
            });
        });
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/the-root"), json!({ "main.rs": "fn main() {}" }))
        .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let worktree_id = project.update(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    cx.update_global::<SettingsStore, _>(|store, cx| {
        store
            .set_local_settings(
                worktree_id,
                LocalSettingsPath::InWorktree(Arc::from(RelPath::empty())),
                LocalSettingsKind::Settings,
                Some(r#"{ "allow_binary_downloads": true }"#),
                cx,
            )
            .unwrap();
    });
    project.read_with(cx, |_, cx| {
        assert_eq!(
            project::project_settings::ProjectSettings::get(
                Some(settings::SettingsLocation {
                    worktree_id,
                    path: RelPath::empty(),
                }),
                cx,
            )
            .allow_binary_downloads,
            true,
        );
    });

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let adapter = DownloadOnlyLspAdapter::default();
    let adapter_name = adapter.name();
    language_registry.register_lsp_adapter("Rust".into(), Arc::new(adapter));
    let mut fake_servers = language_registry.register_fake_lsp_server(
        adapter_name,
        lsp::ServerCapabilities::default(),
        None,
    );

    let (_buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();

    let mut next_server = fake_servers.next().fuse();
    let mut timeout = cx.executor().timer(Duration::from_secs(1)).fuse();
    futures::select! {
        server = next_server => assert_eq!(server.is_some(), true),
        _ = timeout => panic!("timed out waiting for language server"),
    }
}

#[gpui::test]
async fn test_user_installed_lsp_starts_with_downloads_disabled(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();
    cx.update(|cx| project::binary_downloads::init(cx));

    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(false);
            });
        });
    });

    let settings_json = json!({
        "languages": { "Rust": { "language_servers": ["user-installed-language-server"] } },
        "lsp": {
            "user-installed-language-server": {
                "binary": { "path": path!(".bin/user-installed-language-server.exe").to_string() }
            }
        },
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/the-root"),
        json!({
            ".zed": { "settings.json": settings_json.to_string() },
            ".bin": { "user-installed-language-server.exe": "" },
            "main.rs": "fn main() {}",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "user-installed-language-server",
            ..Default::default()
        },
    );

    let (_buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();

    let mut next_server = fake_servers.next().fuse();
    let mut timeout = cx.executor().timer(Duration::from_secs(1)).fuse();
    futures::select! {
        server = next_server => assert_eq!(server.is_some(), true),
        _ = timeout => {
            panic!("user-installed language server should start even while downloads are disabled")
        }
    }
}

#[gpui::test]
async fn test_disk_binary_starts_without_download_and_refreshes_when_downloads_enabled(
    cx: &mut TestAppContext,
) {
    init_test(cx);
    cx.executor().allow_parking();
    cx.update(|cx| project::binary_downloads::init(cx));

    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(false);
            });
        });
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/the-root"), json!({ "main.rs": "fn main() {}" }))
        .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let adapter = DiskBinaryWithDownloaderLspAdapter::default();
    let fetch_count = adapter.fetch_count.clone();
    let adapter_name = adapter.name();
    language_registry.register_lsp_adapter("Rust".into(), Arc::new(adapter));
    let mut fake_servers = language_registry.register_fake_lsp_server(
        adapter_name,
        lsp::ServerCapabilities::default(),
        None,
    );

    let (_buffer, _handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();

    let mut next_server = fake_servers.next().fuse();
    let mut timeout = cx.executor().timer(Duration::from_secs(1)).fuse();
    futures::select! {
        server = next_server => assert_eq!(server.is_some(), true),
        _ = timeout => {
            panic!("server should start from the existing binary while downloads are disabled")
        }
    }
    assert_eq!(
        fetch_count.load(Ordering::SeqCst),
        0,
        "the downloader must not run while downloads are disabled"
    );
    let store =
        cx.update(|cx| project::binary_downloads::BinaryDownloads::try_get_global(cx).unwrap());
    assert_eq!(
        store.read_with(cx, |store, _| store.pending_tool_installs()),
        Vec::new(),
        "starting from a disk binary must not register a pending install"
    );

    cx.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.allow_binary_downloads = Some(true);
            });
        });
    });
    cx.run_until_parked();

    assert_eq!(
        fetch_count.load(Ordering::SeqCst),
        1,
        "enabling downloads must run the deferred downloader once"
    );
}

#[gpui::test]
async fn test_removing_invisible_worktree_cleans_reused_lsp_bookkeeping(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/the-root"), json!({ "main.rs": "fn main() {}" }))
        .await;
    fs.insert_tree(
        path!("/the-registry"),
        json!({ "dep": { "src": { "dep.rs": "pub fn dep() {}" } } }),
    )
    .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp("Rust", FakeLspAdapter::default());

    let (_visible_buffer, _visible_handle) = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();
    fake_servers.next().await.unwrap();
    cx.run_until_parked();

    let server_id = project.read_with(cx, |project, cx| {
        project
            .lsp_store()
            .read(cx)
            .language_server_statuses()
            .next()
            .unwrap()
            .0
    });
    let external_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer_via_lsp(
                Uri::from_file_path(path!("/the-registry/dep/src/dep.rs")).unwrap(),
                server_id,
                cx,
            )
        })
        .await
        .unwrap();
    cx.run_until_parked();

    let invisible_worktree_id =
        external_buffer.read_with(cx, |buffer, cx| buffer.file().unwrap().worktree_id(cx));
    project.read_with(cx, |project, cx| {
        let worktree = project.worktree_for_id(invisible_worktree_id, cx).unwrap();
        assert!(!worktree.read(cx).is_visible());
        assert!(
            project
                .lsp_store()
                .read(cx)
                .has_language_server_seed_for_worktree(invisible_worktree_id)
        );
    });

    project.update(cx, |project, cx| {
        project.remove_worktree(invisible_worktree_id, cx);
    });
    cx.run_until_parked();

    project.read_with(cx, |project, cx| {
        let lsp_store = project.lsp_store();
        let lsp_store = lsp_store.read(cx);
        assert!(
            lsp_store
                .language_server_statuses()
                .any(|(status_server_id, _)| status_server_id == server_id)
        );
        assert!(!lsp_store.has_language_server_seed_for_worktree(invisible_worktree_id));
    });
}

#[gpui::test]
async fn test_open_buffer_via_lsp_case_variant_no_duplicate(cx: &mut TestAppContext) {
    init_test(cx);
    cx.executor().allow_parking();

    let fs = FakeFs::new(cx.executor());
    fs.set_case_sensitive(false);
    fs.insert_tree(
        path!("/root"),
        json!({ "src": { "main.rs": "fn main() {}" } }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());
    let mut fake_servers = language_registry.register_fake_lsp("Rust", FakeLspAdapter::default());

    project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/root/src/main.rs"), cx)
        })
        .await
        .unwrap();
    fake_servers.next().await.unwrap();
    cx.run_until_parked();

    let server_id = project.read_with(cx, |project, cx| {
        project
            .lsp_store()
            .read(cx)
            .language_server_statuses()
            .next()
            .unwrap()
            .0
    });

    project
        .update(cx, |project, cx| {
            project.open_local_buffer_via_lsp(
                Uri::from_file_path(path!("/root/SRC/main.rs")).unwrap(),
                server_id,
                cx,
            )
        })
        .await
        .unwrap();
    cx.run_until_parked();

    project.read_with(cx, |project, cx| {
        let worktree = project.worktrees(cx).next().unwrap();
        let entries: Vec<_> = worktree
            .read(cx)
            .snapshot()
            .entries(true, 0)
            .map(|entry| entry.path.as_unix_str().to_string())
            .collect();
        assert_eq!(entries, vec!["", "src", "src/main.rs"]);
    });
}

#[test]
fn test_rpc_log_grouping_separates_timed_messages() {
    for (received, direction) in [(false, "Send"), (true, "Receive")] {
        let mut header_state = TestRpcLogHeaderState::new();

        assert_eq!(
            header_state.header_for_message(received, None),
            Some(format!("\n// {direction}:"))
        );
        assert_eq!(header_state.header_for_message(received, None), None);
        assert_eq!(
            header_state.header_for_message(received, Some(Duration::from_millis(53))),
            Some(format!("\n// {direction} (took 53.0ms):"))
        );
        assert_eq!(
            header_state.header_for_message(received, None),
            Some(format!("\n// {direction}:"))
        );
        assert_eq!(header_state.header_for_message(received, None), None);
    }
}

#[test]
fn test_rpc_request_tracker_distinguishes_request_directions() {
    let mut tracker = TestRpcRequestTracker::new();
    let started_at = Instant::now();

    assert_eq!(
        tracker.observe(
            false,
            r#"{"jsonrpc":"2.0","id":1,"method":"textDocument/hover"}"#,
            started_at,
        ),
        None
    );
    assert_eq!(
        tracker.observe(
            true,
            r#"{"jsonrpc":"2.0","id":1,"method":"workspace/configuration"}"#,
            started_at + Duration::from_millis(10),
        ),
        None
    );
    assert_eq!(
        tracker.observe(
            false,
            r#"{"jsonrpc":"2.0","id":1,"result":[]}"#,
            started_at + Duration::from_millis(30),
        ),
        Some(Duration::from_millis(20))
    );
    assert_eq!(
        tracker.observe(
            true,
            r#"{"jsonrpc":"2.0","id":1,"result":null}"#,
            started_at + Duration::from_millis(50),
        ),
        Some(Duration::from_millis(50))
    );
}

#[test]
fn test_rpc_request_tracker_decodes_ids_and_times_cancelled_requests() {
    let mut tracker = TestRpcRequestTracker::new();
    let started_at = Instant::now();

    tracker.observe(
        true,
        r#"{"jsonrpc":"2.0","id":"foo\u002fbar","method":"workspace/configuration"}"#,
        started_at,
    );
    assert_eq!(
        tracker.observe(
            false,
            r#"{"jsonrpc":"2.0","id":"foo/bar","result":[]}"#,
            started_at + Duration::from_millis(25),
        ),
        Some(Duration::from_millis(25))
    );

    tracker.observe(
        false,
        r#"{"jsonrpc":"2.0","id":7,"method":"textDocument/hover"}"#,
        started_at,
    );
    tracker.observe(
        false,
        r#"{"jsonrpc":"2.0","method":"$/cancelRequest","params":{"id":7}}"#,
        started_at + Duration::from_millis(1),
    );
    assert_eq!(tracker.pending_request_count(), 1);
    assert_eq!(
        tracker.observe(
            true,
            r#"{"jsonrpc":"2.0","id":7,"error":{"code":-32800,"message":"Request was cancelled"}}"#,
            started_at + Duration::from_millis(10),
        ),
        Some(Duration::from_millis(10))
    );
    assert_eq!(tracker.pending_request_count(), 0);
}

#[test]
fn test_rpc_request_tracker_bounds_unanswered_requests() {
    let mut tracker = TestRpcRequestTracker::new();
    let started_at = Instant::now();
    let max_pending_requests = TestRpcRequestTracker::max_pending_requests();

    for id in 0..=max_pending_requests {
        tracker.observe(
            false,
            &format!(r#"{{"jsonrpc":"2.0","id":{id},"method":"textDocument/hover"}}"#),
            started_at + Duration::from_nanos(id as u64),
        );
    }

    assert_eq!(tracker.pending_request_count(), max_pending_requests);
    assert_eq!(
        tracker.observe(
            true,
            r#"{"jsonrpc":"2.0","id":0,"result":null}"#,
            started_at + Duration::from_secs(1),
        ),
        None
    );
    assert!(
        tracker
            .observe(
                true,
                r#"{"jsonrpc":"2.0","id":1,"result":null}"#,
                started_at + Duration::from_secs(1),
            )
            .is_some()
    );
}

#[test]
fn test_rpc_log_duration_proto_roundtrip() {
    let log_type = LanguageServerLogType::Rpc {
        received: true,
        elapsed: Some(Duration::from_micros(1234)),
    };

    assert_eq!(
        LanguageServerLogType::from_proto(log_type.to_proto()),
        log_type
    );
}

#[test]
fn test_glob_literal_prefix() {
    assert_eq!(glob_literal_prefix(Path::new("**/*.js")), Path::new(""));
    assert_eq!(
        glob_literal_prefix(Path::new("node_modules/**/*.js")),
        Path::new("node_modules")
    );
    assert_eq!(
        glob_literal_prefix(Path::new("foo/{bar,baz}.js")),
        Path::new("foo")
    );
    assert_eq!(
        glob_literal_prefix(Path::new("foo/bar/baz.js")),
        Path::new("foo/bar/baz.js")
    );

    #[cfg(target_os = "windows")]
    {
        assert_eq!(glob_literal_prefix(Path::new("**\\*.js")), Path::new(""));
        assert_eq!(
            glob_literal_prefix(Path::new("node_modules\\**/*.js")),
            Path::new("node_modules")
        );
        assert_eq!(
            glob_literal_prefix(Path::new("foo/{bar,baz}.js")),
            Path::new("foo")
        );
        assert_eq!(
            glob_literal_prefix(Path::new("foo\\bar\\baz.js")),
            Path::new("foo/bar/baz.js")
        );
    }
}

#[test]
fn test_multi_len_chars_normalization() {
    let mut label = CodeLabel::new(
        "myElˇ (parameter) myElˇ: {\n    foo: string;\n}".to_string(),
        0..6,
        vec![(0..6, HighlightId::new(1))],
    );
    ensure_uniform_list_compatible_label(&mut label);
    assert_eq!(
        label,
        CodeLabel::new(
            "myElˇ (parameter) myElˇ: { foo: string; }".to_string(),
            0..6,
            vec![(0..6, HighlightId::new(1))],
        )
    );
}

#[test]
fn test_trailing_newline_in_completion_documentation() {
    let doc =
        lsp::Documentation::String("Inappropriate argument value (of correct type).\n".to_string());
    let completion_doc: CompletionDocumentation = doc.into();
    assert!(
        matches!(completion_doc, CompletionDocumentation::SingleLine(s) if s == "Inappropriate argument value (of correct type).")
    );

    let doc = lsp::Documentation::String("  some value  \n".to_string());
    let completion_doc: CompletionDocumentation = doc.into();
    assert!(matches!(
        completion_doc,
        CompletionDocumentation::SingleLine(s) if s == "some value"
    ));
}

#[gpui::test]
async fn test_user_initialization_options_override_adapter_arrays(cx: &mut TestAppContext) {
    init_test(cx);

    let user_settings = serde_json::json!({
        "lsp": {
            "the-fake-language-server": {
                "initialization_options": {
                    "preview": {
                        "background": {
                            "enabled": true,
                            "args": ["--data-plane-host=127.0.0.1:23635", "--invert-colors=never"],
                        },
                    },
                    "plugins": ["user-plugin"],
                    "userOnly": ["user"],
                },
            },
        },
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/the-root"),
        json!({
            ".zed": {
                "settings.json": user_settings.to_string(),
            },
            "main.rs": "fn main() {}",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());

    let sent_initialization_options = Arc::new(Mutex::new(None));
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "the-fake-language-server",
            initialization_options: Some(json!({
                "preview": {
                    "background": {
                        "args": ["--data-plane-host=127.0.0.1:23635", "--invert-colors=never"],
                        "partialRendering": true,
                    },
                },
                "plugins": ["default-plugin", "user-plugin"],
                "adapterOnly": [1, 2],
            })),
            initializer: Some(Box::new({
                let sent_initialization_options = sent_initialization_options.clone();
                move |fake_server| {
                    let sent_initialization_options = sent_initialization_options.clone();
                    fake_server.set_request_handler::<lsp::request::Initialize, _, _>(
                        move |params, _| {
                            *sent_initialization_options.lock() = params.initialization_options;
                            async move { Ok(lsp::InitializeResult::default()) }
                        },
                    );
                }
            })),
            ..FakeLspAdapter::default()
        },
    );
    cx.run_until_parked();

    project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();
    fake_servers.next().await.unwrap();
    cx.run_until_parked();

    assert_eq!(
        sent_initialization_options.lock().take(),
        Some(json!({
            "preview": {
                "background": {
                    "enabled": true,
                    "args": ["--data-plane-host=127.0.0.1:23635", "--invert-colors=never"],
                    "partialRendering": true,
                },
            },
            "plugins": ["user-plugin"],
            "adapterOnly": [1, 2],
            "userOnly": ["user"],
        })),
    );
}

#[gpui::test]
async fn test_other_adapters_lsp_configuration_contributions_are_unioned(cx: &mut TestAppContext) {
    init_test(cx);

    let user_settings = serde_json::json!({
        "lsp": {
            "the-fake-language-server": {
                "initialization_options": {
                    "languages": ["user-lang"],
                    "userOnly": true,
                },
            },
        },
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/the-root"),
        json!({
            ".zed": {
                "settings.json": user_settings.to_string(),
            },
            "main.rs": "fn main() {}",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());

    let main_server_name = LanguageServerName("the-fake-language-server".into());
    for (language, server_name, plugin, lang, memory) in [
        ("Vue", "vue-language-server", "vue-plugin", "vue", 4096),
        (
            "Astro",
            "astro-language-server",
            "astro-plugin",
            "astro",
            2048,
        ),
    ] {
        let contribution = json!({
            "tsserver": {
                "globalPlugins": ["shared-plugin", plugin],
                "maxMemory": memory,
            },
            "languages": [lang],
        });
        language_registry.register_fake_lsp_adapter(
            language,
            FakeLspAdapter {
                name: server_name,
                additional_initialization_options: HashMap::from_iter([(
                    main_server_name.clone(),
                    contribution.clone(),
                )]),
                additional_workspace_configuration: HashMap::from_iter([(
                    main_server_name.clone(),
                    contribution,
                )]),
                ..FakeLspAdapter::default()
            },
        );
    }

    let sent_initialization_options = Arc::new(Mutex::new(None));
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "the-fake-language-server",
            initialization_options: Some(json!({
                "tsserver": {
                    "globalPlugins": ["default-plugin"],
                },
                "languages": ["default-lang"],
            })),
            initializer: Some(Box::new({
                let sent_initialization_options = sent_initialization_options.clone();
                move |fake_server| {
                    let sent_initialization_options = sent_initialization_options.clone();
                    fake_server.set_request_handler::<lsp::request::Initialize, _, _>(
                        move |params, _| {
                            *sent_initialization_options.lock() = params.initialization_options;
                            async move { Ok(lsp::InitializeResult::default()) }
                        },
                    );
                }
            })),
            ..FakeLspAdapter::default()
        },
    );
    cx.run_until_parked();

    project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();
    let mut fake_server = fake_servers.next().await.unwrap();
    let workspace_configuration = fake_server
        .receive_notification::<lsp::notification::DidChangeConfiguration>()
        .await
        .settings;
    cx.run_until_parked();

    assert_eq!(
        sent_initialization_options.lock().take(),
        Some(json!({
            "tsserver": {
                "globalPlugins": ["default-plugin", "shared-plugin", "astro-plugin", "vue-plugin"],
                "maxMemory": 4096,
            },
            "languages": ["user-lang"],
            "userOnly": true,
        })),
    );
    assert_eq!(
        workspace_configuration,
        json!({
            "tsserver": {
                "globalPlugins": ["shared-plugin", "astro-plugin", "vue-plugin"],
                "maxMemory": 4096,
            },
            "languages": ["astro", "vue"],
        }),
    );
}

#[gpui::test]
async fn test_initialization_options_contributions_without_own_options(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/the-root"), json!({ "main.rs": "fn main() {}" }))
        .await;

    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(rust_lang());

    let contribution = json!({
        "tsserver": {
            "globalPlugins": ["vue-plugin"],
        },
    });
    language_registry.register_fake_lsp_adapter(
        "Vue",
        FakeLspAdapter {
            name: "vue-language-server",
            additional_initialization_options: HashMap::from_iter([(
                LanguageServerName("the-fake-language-server".into()),
                contribution.clone(),
            )]),
            ..FakeLspAdapter::default()
        },
    );

    let sent_initialization_options = Arc::new(Mutex::new(None));
    let mut fake_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "the-fake-language-server",
            initialization_options: None,
            initializer: Some(Box::new({
                let sent_initialization_options = sent_initialization_options.clone();
                move |fake_server| {
                    let sent_initialization_options = sent_initialization_options.clone();
                    fake_server.set_request_handler::<lsp::request::Initialize, _, _>(
                        move |params, _| {
                            *sent_initialization_options.lock() =
                                Some(params.initialization_options);
                            async move { Ok(lsp::InitializeResult::default()) }
                        },
                    );
                }
            })),
            ..FakeLspAdapter::default()
        },
    );
    cx.run_until_parked();

    project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/main.rs"), cx)
        })
        .await
        .unwrap();
    fake_servers.next().await.unwrap();
    cx.run_until_parked();

    assert_eq!(
        sent_initialization_options.lock().take(),
        Some(Some(contribution)),
    );
}
