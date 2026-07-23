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
async fn test_local_views_and_downstream_requests_own_rpc_streams_independently(
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(path!("/stream-ownership"), json!({ "test.rs": "" }))
        .await;
    let project = Project::test(fs, [path!("/stream-ownership").as_ref()], cx).await;
    let server_id = LanguageServerId(100);
    let server_key = LanguageServerLogKey::new(
        LanguageServerKind::Local {
            project: project.downgrade(),
        },
        server_id,
    );
    let log_store = cx.new(|cx| LogStore::new(false, cx));

    log_store.update(cx, |store, cx| {
        store.add_language_server(
            server_key.kind.clone(),
            server_id,
            Some(LanguageServerName::new_static("test-server")),
            None,
            None,
            cx,
        );

        store.toggle_lsp_logs(&server_key, true, LogKind::Rpc);
        assert!(
            store
                .language_servers
                .get(&server_key)
                .is_some_and(|state| state.rpc_state.is_some())
        );
        assert_eq!(
            store.retain_view_log_stream(&server_key, LogKind::Rpc),
            Some(true)
        );
        assert_eq!(
            store.release_view_log_stream(&server_key, LogKind::Rpc),
            Some(true)
        );
        assert!(
            store
                .language_servers
                .get(&server_key)
                .is_some_and(|state| state.rpc_state.is_some()),
            "releasing the final local view must preserve a downstream RPC request"
        );
        store.toggle_lsp_logs(&server_key, false, LogKind::Rpc);
        assert!(
            store
                .language_servers
                .get(&server_key)
                .is_some_and(|state| state.rpc_state.is_none())
        );

        assert_eq!(
            store.retain_view_log_stream(&server_key, LogKind::Rpc),
            Some(true)
        );
        store.toggle_lsp_logs(&server_key, true, LogKind::Rpc);
        store.toggle_lsp_logs(&server_key, false, LogKind::Rpc);
        assert!(
            store
                .language_servers
                .get(&server_key)
                .is_some_and(|state| state.rpc_state.is_some()),
            "disabling a downstream RPC request must preserve a local view's stream"
        );
        assert_eq!(
            store.release_view_log_stream(&server_key, LogKind::Rpc),
            Some(true)
        );
        assert!(
            store
                .language_servers
                .get(&server_key)
                .is_some_and(|state| state.rpc_state.is_none())
        );
    });
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

fn init_test(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme_settings::init(theme::LoadThemes::JustBase, cx);
        release_channel::init(semver::Version::new(0, 0, 0), cx);
    });
}
