use std::{sync::Arc, time::Duration};

use crate::lsp_log_view::LogMenuItem;

use super::*;
use futures::StreamExt;
use gpui::{AppContext as _, TestAppContext, VisualTestContext};
use language::{FakeLspAdapter, Language, LanguageConfig, LanguageMatcher, tree_sitter_rust};
use lsp::{LanguageServerId, LanguageServerName};
use project::{
    FakeFs, Project,
    lsp_store::log_store::{LanguageServerKind, LogKind, LogStore},
};
use serde_json::json;
use settings::SettingsStore;
use util::path;

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
            matcher: LanguageMatcher {
                path_suffixes: vec!["rs".to_string()],
                ..Default::default()
            },
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
    cx.executor().advance_clock(Duration::from_millis(60));
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
                }
            }]
        );
        assert_eq!(view.editor.read(cx).text(cx), "hello from the server\n");
    });
}

#[gpui::test]
async fn test_lsp_log_view_batches_log_entries(cx: &mut TestAppContext) {
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

    let log_store = cx.new(|cx| LogStore::new(false, cx));
    log_store.update(cx, |store, cx| {
        store.add_project(&project, cx);
        store.add_language_server(
            LanguageServerKind::Local {
                project: project.downgrade(),
            },
            LanguageServerId(1),
            Some(LanguageServerName::new_static("test-language-server")),
            None,
            None,
            cx,
        );
    });

    let window =
        cx.add_window(|window, cx| LspLogView::new(project.clone(), log_store.clone(), window, cx));
    let log_view = window.root(cx).unwrap();
    let mut cx = VisualTestContext::from_window(*window, cx);

    log_store.update(&mut cx, |store, cx| {
        store.add_language_server_log(
            LanguageServerId(1),
            lsp::MessageType::INFO,
            "batched message",
            cx,
        );
    });

    log_view.update(&mut cx, |view, cx| {
        assert_eq!(view.editor.read(cx).text(cx), "");
    });

    cx.executor().advance_clock(Duration::from_millis(60));
    cx.executor().run_until_parked();

    log_view.update(&mut cx, |view, cx| {
        assert_eq!(view.editor.read(cx).text(cx), "batched message\n");
    });
}

/// Tests that calling `clear_pending_log_append` while a pending log task is running
/// properly cancels the pending append and prevents stale data from being written.
/// This happens when switching between servers before the log flush timer fires.
#[gpui::test]
async fn test_clear_pending_log_append_while_task_pending(cx: &mut TestAppContext) {
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

    let log_store = cx.new(|cx| LogStore::new(false, cx));
    log_store.update(cx, |store, cx| {
        store.add_project(&project, cx);
        // Add two language servers so we can switch between them
        store.add_language_server(
            LanguageServerKind::Local {
                project: project.downgrade(),
            },
            LanguageServerId(1),
            Some(LanguageServerName::new_static("server-one")),
            None,
            None,
            cx,
        );
        store.add_language_server(
            LanguageServerKind::Local {
                project: project.downgrade(),
            },
            LanguageServerId(2),
            Some(LanguageServerName::new_static("server-two")),
            None,
            None,
            cx,
        );
    });

    let window =
        cx.add_window(|window, cx| LspLogView::new(project.clone(), log_store.clone(), window, cx));
    let log_view = window.root(cx).unwrap();
    let mut cx = VisualTestContext::from_window(*window, cx);

    // First show server 1's logs so the view is subscribed to it
    log_view.update_in(&mut cx, |view, window, cx| {
        view.show_logs_for_server(LanguageServerId(1), window, cx);
    });

    // Queue a log entry for server 1. This starts a 50ms timer before flush.
    log_store.update(&mut cx, |store, cx| {
        store.add_language_server_log(
            LanguageServerId(1),
            lsp::MessageType::INFO,
            "message for server one",
            cx,
        );
    });

    // Verify the message is NOT yet written to editor (still pending)
    log_view.update(&mut cx, |view, cx| {
        assert_eq!(view.editor.read(cx).text(cx), "");
    });

    // Before the timer fires, switch to server 2's logs. This calls clear_pending_log_append
    // internally, which should clear the pending message for server 1.
    log_view.update_in(&mut cx, |view, window, cx| {
        view.show_logs_for_server(LanguageServerId(2), window, cx);
    });

    // Verify editor is empty since server 2 has no logs
    log_view.update(&mut cx, |view, cx| {
        assert_eq!(view.editor.read(cx).text(cx), "");
    });

    // Now let the original timer fire (advance past the 50ms log batching delay)
    cx.executor().advance_clock(Duration::from_millis(60));
    cx.executor().run_until_parked();

    // Verify that no stale data from server 1 appeared in the editor.
    // If clear_pending_log_append didn't work correctly, we'd see "message for server one" here.
    log_view.update(&mut cx, |view, cx| {
        assert_eq!(view.editor.read(cx).text(cx), "");
    });
}

fn init_test(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme::init(theme::LoadThemes::JustBase, cx);
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });
}
