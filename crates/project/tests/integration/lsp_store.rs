use std::{
    path::Path,
    sync::Arc,
    time::{Duration, Instant},
};

use collections::HashMap;
use fs::FakeFs;
use futures::StreamExt;
use gpui::{AppContext, TestAppContext, WeakEntity};
use language::{CodeLabel, FakeLspAdapter, HighlightId, rust_lang};
use lsp::{LanguageServerId, LanguageServerName, LanguageServerSelector, MessageType, Uri};
use parking_lot::Mutex;
use project::{
    Project,
    lsp_store::{log_store::*, *},
};
use serde_json::json;
use util::path;

use crate::init_test;

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

/// A stopped server keeps its entry in LogStore. This can then be reused later on
#[gpui::test]
async fn test_stopped_server_logs_retained_until_restart(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        let log_store = cx.new(|cx| LogStore::new(false, cx));
        let project = WeakEntity::new_invalid();
        let lsp_store = WeakEntity::new_invalid();
        let name = LanguageServerName("rust-analyzer".into());
        let worktree_id = WorktreeId::from_usize(1);
        let first_id = LanguageServerId(1);
        let kind = LanguageServerKind::Supplementary {
            project: project.clone(),
        };

        log_store.update(cx, |store, cx| {
            store.add_language_server(
                kind.clone(),
                first_id,
                Some(name.clone()),
                Some(worktree_id),
                None,
                cx,
            );
            let key = LanguageServerLogKey::new(kind.clone(), first_id);
            store.add_language_server_log(&key, MessageType::LOG, "hello from the server", cx);

            store.remove_language_server(&key, cx);

            assert!(
                store.contains_language_server(first_id),
                "the stopped server stays tracked",
            );
            assert!(
                !store.language_servers.contains_key(&key)
                    && store
                        .stopped_language_servers
                        .contains_key(&StoppedServerKey {
                            kind: kind.clone(),
                            name: name.clone(),
                            worktree_id: Some(worktree_id),
                        }),
                "the entry moves from running to stopped",
            );
            assert_eq!(
                store.language_server_id_for_name_and_worktree(
                    &name,
                    worktree_id,
                    &project,
                    &lsp_store,
                ),
                Some(first_id),
                "the stopped server can still be looked up by name and worktree",
            );
            let selector_id = LanguageServerSelector::Id(first_id);
            let selector_name = LanguageServerSelector::Name(name.clone());
            assert!(store.has_server_logs(&selector_id, &project, &lsp_store),);
            assert!(store.has_server_logs(&selector_name, &project, &lsp_store),);
            assert_eq!(
                store.server_logs(&key).map(|logs| logs.len()),
                Some(1),
                "logs recorded before the stop are retained",
            );
            assert_eq!(
                store
                    .server_logs(&key)
                    .and_then(|logs| logs.front())
                    .map(|log| log.message.as_str()),
                Some("hello from the server"),
            );
            assert!(
                store.get_language_server_state(&key).is_some(),
                "mutable state access still works for the stopped server",
            );
            assert!(
                store.enable_rpc_trace_for_language_server(&key).is_some(),
                "rpc tracing can still be enabled for the stopped server",
            );

            let restarted_id = LanguageServerId(2);
            store.add_language_server(
                kind.clone(),
                restarted_id,
                Some(name.clone()),
                Some(worktree_id),
                None,
                cx,
            );
            let restarted_key = LanguageServerLogKey::new(kind.clone(), restarted_id);

            assert!(
                !store.contains_language_server(first_id),
                "the stopped server's id is no longer tracked after the restart",
            );
            assert!(
                store.stopped_language_servers.is_empty(),
                "no stopped entries linger after the restart",
            );
            assert_eq!(
                store.language_server_id_for_name_and_worktree(
                    &name,
                    worktree_id,
                    &project,
                    &lsp_store,
                ),
                Some(restarted_id),
                "the lookup now points at the running instance",
            );
            assert_eq!(
                store.server_logs(&restarted_key).map(|logs| logs
                    .iter()
                    .map(|log| log.message.as_str())
                    .collect::<Vec<_>>()),
                Some(vec!["hello from the server"]),
                "the retained logs carry over to the restarted server",
            );
            store.add_language_server_log(&restarted_key, MessageType::LOG, "hello again", cx);
            assert_eq!(
                store.server_logs(&restarted_key).map(|logs| logs
                    .iter()
                    .map(|log| log.message.as_str())
                    .collect::<Vec<_>>()),
                Some(vec!["hello from the server", "hello again"]),
                "new logs are appended after the carried-over logs",
            );
        });
    });
}

/// A supplementary (worktree-less) stopped server merges its logs into the
/// restarted instance, since stopped entries are keyed by
/// (kind, name, Option<WorktreeId>).
#[gpui::test]
async fn test_stopped_global_server_logs_retained_until_restart(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        let log_store = cx.new(|cx| LogStore::new(false, cx));
        let name = LanguageServerName("global-server".into());
        let first_id = LanguageServerId(1);
        let kind = LanguageServerKind::Supplementary {
            project: WeakEntity::new_invalid(),
        };

        log_store.update(cx, |store, cx| {
            store.add_language_server(kind.clone(), first_id, Some(name.clone()), None, None, cx);
            let key = LanguageServerLogKey::new(kind.clone(), first_id);
            store.add_language_server_log(&key, MessageType::LOG, "from the global server", cx);

            store.remove_language_server(&key, cx);

            assert!(
                store
                    .stopped_language_servers
                    .contains_key(&StoppedServerKey {
                        kind: kind.clone(),
                        name: name.clone(),
                        worktree_id: None,
                    }),
                "the global server is stored under a None worktree key",
            );
            assert_eq!(
                store.server_logs(&key).map(|logs| logs.len()),
                Some(1),
                "the global server's logs are retained while stopped",
            );

            let restarted_id = LanguageServerId(2);
            store.add_language_server(
                kind.clone(),
                restarted_id,
                Some(name.clone()),
                None,
                None,
                cx,
            );
            let restarted_key = LanguageServerLogKey::new(kind.clone(), restarted_id);

            assert!(
                store.stopped_language_servers.is_empty(),
                "the global stopped entry is claimed on restart",
            );
            assert_eq!(
                store.server_logs(&restarted_key).map(|logs| logs
                    .iter()
                    .map(|log| log.message.as_str())
                    .collect::<Vec<_>>()),
                Some(vec!["from the global server"]),
                "the global server's logs carry over to the restarted instance",
            );
        });
    });
}

/// Two projects running the same-named server for the same worktree do not
/// leak logs or RPC traces into each other.
#[gpui::test]
async fn test_stopped_server_logs_are_not_transferred_between_projects(
    cx: &mut gpui::TestAppContext,
) {
    cx.update(|cx| {
        let log_store = cx.new(|cx| LogStore::new(false, cx));

        let project_a = WeakEntity::new_invalid();
        let project_b = WeakEntity::new_invalid();
        let name = LanguageServerName("rust-analyzer".into());
        let worktree_id = WorktreeId::from_usize(1);

        let kind_a = LanguageServerKind::Supplementary {
            project: project_a.clone(),
        };
        let kind_b = LanguageServerKind::Supplementary {
            project: project_b.clone(),
        };

        log_store.update(cx, |store, cx| {
            store.add_language_server(
                kind_a.clone(),
                LanguageServerId(1),
                Some(name.clone()),
                Some(worktree_id),
                None,
                cx,
            );
            let key_a = LanguageServerLogKey::new(kind_a.clone(), LanguageServerId(1));
            store.add_language_server_log(&key_a, MessageType::LOG, "hello from project A", cx);

            store.remove_language_server(&key_a, cx);

            assert!(
                store
                    .stopped_language_servers
                    .contains_key(&StoppedServerKey {
                        kind: kind_a.clone(),
                        name: name.clone(),
                        worktree_id: Some(worktree_id),
                    }),
                "project A's server is in the stopped map",
            );
            assert!(
                !store
                    .stopped_language_servers
                    .contains_key(&StoppedServerKey {
                        kind: kind_b.clone(),
                        name: name.clone(),
                        worktree_id: Some(worktree_id),
                    }),
                "project B has no entry yet",
            );

            store.add_language_server(
                kind_b.clone(),
                LanguageServerId(2),
                Some(name.clone()),
                Some(worktree_id),
                None,
                cx,
            );
            let key_b = LanguageServerLogKey::new(kind_b.clone(), LanguageServerId(2));

            assert!(
                store
                    .stopped_language_servers
                    .contains_key(&StoppedServerKey {
                        kind: kind_a.clone(),
                        name: name.clone(),
                        worktree_id: Some(worktree_id),
                    }),
                "project A's stopped entry is untouched by project B's start",
            );
            assert!(
                store
                    .server_logs(&key_b)
                    .is_some_and(|logs| logs.is_empty()),
                "project B does not inherit project A's logs",
            );

            store.add_language_server_log(&key_b, MessageType::LOG, "hello from project B", cx);
            assert_eq!(
                store.server_logs(&key_b).map(|logs| logs
                    .iter()
                    .map(|log| log.message.as_str())
                    .collect::<Vec<_>>()),
                Some(vec!["hello from project B"]),
                "project B has only its own logs",
            );

            let lsp_store_a = WeakEntity::new_invalid();
            let lsp_store_b = WeakEntity::new_invalid();

            assert_eq!(
                store.language_server_id_for_name_and_worktree(
                    &name,
                    worktree_id,
                    &project_a,
                    &lsp_store_a,
                ),
                Some(LanguageServerId(1)),
                "project A lookup returns project A's stopped server ID",
            );

            assert_eq!(
                store.language_server_id_for_name_and_worktree(
                    &name,
                    worktree_id,
                    &project_b,
                    &lsp_store_b,
                ),
                Some(LanguageServerId(2)),
                "project B lookup returns project B's running server ID",
            );
        });
    });
}
