use std::sync::Arc;

use crate::lsp_log_view::LogMenuItem;

use super::*;
use futures::StreamExt;
use gpui::{AppContext as _, TestAppContext, VisualTestContext};
use language::{
    FakeLspAdapter, Language, LanguageConfig, LanguageMatcher, LanguageServerId, tree_sitter_rust,
};
use lsp::LanguageServerName;
use project::{
    FakeFs, Project,
    lsp_store::log_store::{LanguageServerKind, LanguageServerLogKey, LogKind, LogStore},
};
use serde_json::json;
use settings::SettingsStore;
use util::path;

#[gpui::test]
async fn test_lsp_log_view_filters_servers_from_other_projects(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(path!("/current-project"), json!({ "test.rs": "" }))
        .await;
    fs.insert_tree(path!("/other-project"), json!({ "test.rs": "" }))
        .await;

    let project = Project::test(fs.clone(), [path!("/current-project").as_ref()], cx).await;
    let other_project = Project::test(fs.clone(), [path!("/other-project").as_ref()], cx).await;
    let worktree_id = project.read_with(cx, |project, cx| {
        project.worktrees(cx).next().unwrap().read(cx).id()
    });
    let current_lsp_store = project
        .read_with(cx, |project, _| project.lsp_store())
        .downgrade();
    let other_lsp_store = other_project
        .read_with(cx, |project, _| project.lsp_store())
        .downgrade();

    let current_ssh_server_id = LanguageServerId(99);
    let current_server_id = LanguageServerId(100);
    let log_store = cx.new(|cx| LogStore::new(false, cx));
    log_store.update(cx, |store, cx| {
        store.add_language_server(
            LanguageServerKind::LocalSsh {
                lsp_store: current_lsp_store,
            },
            current_ssh_server_id,
            Some(LanguageServerName::new_static("current-ssh-server")),
            Some(worktree_id),
            None,
            cx,
        );
        store.add_language_server(
            LanguageServerKind::Local {
                project: project.downgrade(),
            },
            current_server_id,
            Some(LanguageServerName::new_static("current-server")),
            Some(worktree_id),
            None,
            cx,
        );
        store.add_language_server(
            LanguageServerKind::Remote {
                project: other_project.downgrade(),
            },
            current_server_id,
            Some(LanguageServerName::new_static("other-remote-server")),
            None,
            None,
            cx,
        );
        store.add_language_server(
            LanguageServerKind::Supplementary {
                project: other_project.downgrade(),
            },
            current_server_id,
            Some(LanguageServerName::new_static("other-supplementary-server")),
            None,
            None,
            cx,
        );
        store.add_language_server(
            LanguageServerKind::LocalSsh {
                lsp_store: other_lsp_store,
            },
            current_ssh_server_id,
            Some(LanguageServerName::new_static("other-ssh-server")),
            None,
            None,
            cx,
        );
    });
    assert_eq!(
        log_store.read_with(cx, |store, _| store.language_servers.len()),
        5
    );

    let window =
        cx.add_window(|window, cx| LspLogView::new(project.clone(), log_store, window, cx));
    let log_view = window.root(cx).unwrap();
    let mut cx = VisualTestContext::from_window(*window, cx);

    log_view.update(&mut cx, |view, cx| {
        let visible_servers = view
            .menu_items(cx)
            .unwrap()
            .into_iter()
            .map(|item| (item.server_id, item.server_name))
            .collect::<Vec<_>>();
        assert_eq!(
            visible_servers,
            [
                (
                    current_ssh_server_id,
                    LanguageServerName::new_static("current-ssh-server"),
                ),
                (
                    current_server_id,
                    LanguageServerName::new_static("current-server"),
                ),
            ]
        );
    });
}

#[gpui::test]
async fn test_lsp_log_view_labels_registered_supplementary_servers(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(path!("/the-root"), json!({ "test.rs": "" }))
        .await;
    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let lsp_store = project.read_with(cx, |project, _| project.lsp_store());
    let log_store = cx.new(|cx| LogStore::new(false, cx));
    log_store.update(cx, |store, cx| store.add_project(&project, cx));

    let server_id = LanguageServerId(100);
    let (server, _fake_server) = lsp::FakeLanguageServer::new(
        server_id,
        lsp::LanguageServerBinary {
            path: "path/to/prettier".into(),
            arguments: Vec::new(),
            env: None,
        },
        "prettier".to_string(),
        Default::default(),
        &mut cx.to_async(),
    );
    lsp_store.update(cx, |lsp_store, cx| {
        lsp_store.register_supplementary_language_server_for_test(
            server_id,
            LanguageServerName::new_static("prettier (default)"),
            Arc::new(server),
            cx,
        );
    });

    let window =
        cx.add_window(|window, cx| LspLogView::new(project.clone(), log_store, window, cx));
    let log_view = window.root(cx).unwrap();
    let mut cx = VisualTestContext::from_window(*window, cx);

    log_view.update(&mut cx, |view, cx| {
        assert_eq!(
            view.menu_items(cx).unwrap(),
            &[LogMenuItem {
                server_id,
                server_name: LanguageServerName::new_static("prettier (default)"),
                worktree_root_name: "supplementary".to_string(),
                rpc_trace_enabled: false,
                selected_entry: LogKind::Logs,
                trace_level: lsp::TraceValue::Off,
                server_kind: LanguageServerKind::Supplementary {
                    project: project.downgrade(),
                },
                stopped: false,
            }]
        );
    });

    lsp_store.update(&mut cx, |lsp_store, cx| {
        lsp_store.unregister_supplementary_language_server_for_test(server_id, cx);
    });
    log_view.update(&mut cx, |view, cx| {
        assert!(view.menu_items(cx).unwrap().is_empty());
    });
}

#[gpui::test]
async fn test_log_store_does_not_retain_language_servers(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(path!("/the-root"), json!({ "test.rs": "" }))
        .await;
    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let server_id = LanguageServerId(100);
    let server_kind = LanguageServerKind::Local {
        project: project.downgrade(),
    };
    let server_key = LanguageServerLogKey::new(server_kind.clone(), server_id);
    let log_store = cx.new(|cx| LogStore::new(false, cx));

    let (first_server, _first_fake_server) = lsp::FakeLanguageServer::new(
        server_id,
        lsp::LanguageServerBinary {
            path: "path/to/first-language-server".into(),
            arguments: Vec::new(),
            env: None,
        },
        "first-language-server".to_string(),
        Default::default(),
        &mut cx.to_async(),
    );
    let first_server = Arc::new(first_server);
    let first_server_weak = Arc::downgrade(&first_server);
    log_store.update(cx, |store, cx| {
        store.add_language_server(
            server_kind.clone(),
            server_id,
            Some(LanguageServerName::new_static("first-language-server")),
            None,
            Some(first_server.clone()),
            cx,
        );
    });

    let (replacement_server, _replacement_fake_server) = lsp::FakeLanguageServer::new(
        server_id,
        lsp::LanguageServerBinary {
            path: "path/to/replacement-language-server".into(),
            arguments: Vec::new(),
            env: None,
        },
        "replacement-language-server".to_string(),
        Default::default(),
        &mut cx.to_async(),
    );
    let replacement_server = Arc::new(replacement_server);
    let replacement_server_weak = Arc::downgrade(&replacement_server);
    log_store.update(cx, |store, cx| {
        store.add_language_server(
            server_kind,
            server_id,
            Some(LanguageServerName::new_static(
                "replacement-language-server",
            )),
            None,
            Some(replacement_server.clone()),
            cx,
        );
    });

    let stored_server = log_store
        .read_with(cx, |store, _| {
            store
                .language_servers
                .get(&server_key)
                .and_then(|state| state.server())
        })
        .expect("replacement language server should be available");
    assert!(Arc::ptr_eq(&stored_server, &replacement_server));
    drop(stored_server);

    drop(first_server);
    assert!(first_server_weak.upgrade().is_none());
    drop(replacement_server);
    assert!(replacement_server_weak.upgrade().is_none());
    assert!(
        log_store
            .read_with(cx, |store, _| store
                .language_servers
                .get(&server_key)
                .and_then(|state| state.server()))
            .is_none()
    );
}

#[gpui::test]
async fn test_log_store_removes_unavailable_copilot_server(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(path!("/the-root"), json!({ "test.rs": "" }))
        .await;
    let project = Project::test(fs, [path!("/the-root").as_ref()], cx).await;
    let server_id = LanguageServerId(100);
    let server_kind = LanguageServerKind::Supplementary {
        project: project.downgrade(),
    };
    let server_key = LanguageServerLogKey::new(server_kind, server_id);
    let log_store = cx.new(|cx| LogStore::new(false, cx));
    log_store.update(cx, |log_store, cx| log_store.add_project(&project, cx));

    let (server, _fake_server) = lsp::FakeLanguageServer::new(
        server_id,
        lsp::LanguageServerBinary {
            path: "path/to/copilot-language-server".into(),
            arguments: Vec::new(),
            env: None,
        },
        "copilot".to_string(),
        Default::default(),
        &mut cx.to_async(),
    );
    log_store.update(cx, |log_store, cx| {
        log_store.sync_copilot_for_project(&project.downgrade(), Some(Arc::new(server)), cx);
    });
    assert!(log_store.read_with(cx, |log_store, _| {
        log_store.language_servers.contains_key(&server_key)
    }));

    log_store.update(cx, |log_store, cx| {
        log_store.sync_copilot_for_project(&project.downgrade(), None, cx);
    });
    assert!(!log_store.read_with(cx, |log_store, _| {
        log_store.language_servers.contains_key(&server_key)
    }));
}

#[gpui::test]
async fn test_lsp_log_view(cx: &mut TestAppContext) {
    zlog::init_test();

    init_test(cx);

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        path!("/the-root"),
        json!({
            "test.rs": "",
            "package.json": "",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/the-root").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(Arc::new(Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: (LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            })
            .into(),
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    )));
    let mut fake_rust_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "the-rust-language-server",
            ..Default::default()
        },
    );

    let log_store = cx.new(|cx| LogStore::new(false, cx));
    log_store.update(cx, |store, cx| store.add_project(&project, cx));

    let _rust_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/test.rs"), cx)
        })
        .await
        .unwrap();

    let mut language_server = fake_rust_servers.next().await.unwrap();
    language_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await;

    let window =
        cx.add_window(|window, cx| LspLogView::new(project.clone(), log_store.clone(), window, cx));
    let log_view = window.root(cx).unwrap();
    let mut cx = VisualTestContext::from_window(*window, cx);

    language_server.notify::<lsp::notification::LogMessage>(lsp::LogMessageParams {
        message: "hello from the server".into(),
        typ: lsp::MessageType::INFO,
    });
    cx.executor().run_until_parked();

    log_view.update(&mut cx, |view, cx| {
        assert_eq!(
            view.menu_items(cx).unwrap(),
            &[LogMenuItem {
                server_id: language_server.server.server_id(),
                server_name: LanguageServerName("the-rust-language-server".into()),
                worktree_root_name: project
                    .read(cx)
                    .worktrees(cx)
                    .next()
                    .unwrap()
                    .read(cx)
                    .root_name_str()
                    .to_string(),
                rpc_trace_enabled: false,
                selected_entry: LogKind::Logs,
                trace_level: lsp::TraceValue::Off,
                server_kind: LanguageServerKind::Local {
                    project: project.downgrade()
                },
                stopped: false
            }]
        );
        assert_eq!(view.editor.read(cx).text(cx), "hello from the server\n");
    });
}

/// A stopped server stays in the log view's menu and is marked as stopped
#[gpui::test]
async fn test_lsp_log_view_stopped_server_shows_retained_logs(cx: &mut TestAppContext) {
    zlog::init_test();

    init_test(cx);

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        path!("/the-root"),
        json!({
            "test.rs": "",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/the-root").as_ref()], cx).await;

    let language_registry = project.read_with(cx, |project, _| project.languages().clone());
    language_registry.add(Arc::new(Language::new(
        LanguageConfig {
            name: "Rust".into(),
            matcher: (LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            })
            .into(),
            ..Default::default()
        },
        Some(tree_sitter_rust::LANGUAGE.into()),
    )));
    let mut fake_rust_servers = language_registry.register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "the-rust-language-server",
            ..Default::default()
        },
    );

    let log_store = cx.new(|cx| LogStore::new(false, cx));
    log_store.update(cx, |store, cx| store.add_project(&project, cx));

    let _rust_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer_with_lsp(path!("/the-root/test.rs"), cx)
        })
        .await
        .unwrap();

    let mut language_server = fake_rust_servers.next().await.unwrap();
    language_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await;
    let server_id = language_server.server.server_id();

    let window =
        cx.add_window(|window, cx| LspLogView::new(project.clone(), log_store.clone(), window, cx));
    let log_view = window.root(cx).unwrap();
    let mut cx = VisualTestContext::from_window(*window, cx);

    language_server.notify::<lsp::notification::LogMessage>(lsp::LogMessageParams {
        message: "hello from the server".into(),
        typ: lsp::MessageType::INFO,
    });
    cx.executor().run_until_parked();

    log_view.update(&mut cx, |view, cx| {
        assert_eq!(view.editor.read(cx).text(cx), "hello from the server\n");
    });

    let lsp_store = project.read_with(&cx, |project, _| project.lsp_store());
    lsp_store.update(&mut cx, |lsp_store, cx| {
        lsp_store.stop_all_language_servers(cx)
    });
    cx.executor().run_until_parked();

    log_view.update(&mut cx, |view, cx| {
        assert_eq!(
            view.menu_items(cx).unwrap(),
            &[LogMenuItem {
                server_id,
                server_name: LanguageServerName("the-rust-language-server".into()),
                worktree_root_name: project
                    .read(cx)
                    .worktrees(cx)
                    .next()
                    .unwrap()
                    .read(cx)
                    .root_name_str()
                    .to_string(),
                rpc_trace_enabled: false,
                selected_entry: LogKind::Logs,
                trace_level: lsp::TraceValue::Off,
                server_kind: LanguageServerKind::Local {
                    project: project.downgrade()
                },
                stopped: true
            }]
        );
        assert_eq!(
            view.editor.read(cx).text(cx),
            "hello from the server\n",
            "the log view stays on the stopped server and keeps its logs",
        );
    });

    lsp_store.update(&mut cx, |lsp_store, cx| {
        lsp_store.restart_all_language_servers(cx)
    });
    let mut restarted_server = fake_rust_servers.next().await.unwrap();
    restarted_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await;
    cx.executor().run_until_parked();

    let restarted_id = restarted_server.server.server_id();
    log_view.update(&mut cx, |view, cx| {
        assert_eq!(
            view.menu_items(cx).unwrap(),
            &[LogMenuItem {
                server_id: restarted_id,
                server_name: LanguageServerName("the-rust-language-server".into()),
                worktree_root_name: project
                    .read(cx)
                    .worktrees(cx)
                    .next()
                    .unwrap()
                    .read(cx)
                    .root_name_str()
                    .to_string(),
                rpc_trace_enabled: false,
                selected_entry: LogKind::Logs,
                trace_level: lsp::TraceValue::Off,
                server_kind: LanguageServerKind::Local {
                    project: project.downgrade()
                },
                stopped: false
            }]
        );
        assert_eq!(
            view.editor.read(cx).text(cx),
            "hello from the server\n",
            "the log view follows the restarted server and keeps the retained logs",
        );
    });
}

fn init_test(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });
}
