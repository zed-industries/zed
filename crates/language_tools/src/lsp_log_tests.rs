use std::sync::Arc;

use crate::lsp_log::LogMenuItem;

use super::*;
use futures::StreamExt;
use gpui::{serde_json::json, TestAppContext};
use language::{tree_sitter_rust, FakeLspAdapter, Language, LanguageConfig, LanguageServerName};
use project::{FakeFs, Project};
use settings::SettingsStore;

#[gpui::test]
async fn test_lsp_logs(cx: &mut TestAppContext) {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }

    init_test(cx);

    let mut rust_language = Language::new(
        LanguageConfig {
            name: "Rust".into(),
            path_suffixes: vec!["rs".to_string()],
            ..Default::default()
        },
        Some(tree_sitter_rust::language()),
    );
    let mut fake_rust_servers = rust_language
        .set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
            name: "the-rust-language-server",
            ..Default::default()
        }))
        .await;

    let fs = FakeFs::new(cx.background());
    fs.insert_tree(
        "/the-root",
        json!({
            "test.rs": "",
            "package.json": "",
        }),
    )
    .await;
    let project = Project::test(fs.clone(), ["/the-root".as_ref()], cx).await;
    project.update(cx, |project, _| {
        project.languages().add(Arc::new(rust_language));
    });

    let log_store = cx.add_model(|cx| LogStore::new(cx));
    log_store.update(cx, |store, cx| store.add_project(&project, cx));

    let _rust_buffer = project
        .update(cx, |project, cx| {
            project.open_local_buffer("/the-root/test.rs", cx)
        })
        .await
        .unwrap();

    let mut language_server = fake_rust_servers.next().await.unwrap();
    language_server
        .receive_notification::<lsp::notification::DidOpenTextDocument>()
        .await;

    let (_, log_view) = cx.add_window(|cx| LspLogView::new(project.clone(), log_store.clone(), cx));

    language_server.notify::<lsp::notification::LogMessage>(lsp::LogMessageParams {
        message: "hello from the server".into(),
        typ: lsp::MessageType::INFO,
    });
    cx.foreground().run_until_parked();

    log_view.read_with(cx, |view, cx| {
        assert_eq!(
            view.menu_items(cx).unwrap(),
            &[LogMenuItem {
                server_id: language_server.server.server_id(),
                server_name: LanguageServerName("the-rust-language-server".into()),
                worktree: project.read(cx).worktrees(cx).next().unwrap(),
                rpc_trace_enabled: false,
                rpc_trace_selected: false,
                logs_selected: true,
            }]
        );
        assert_eq!(view.editor.read(cx).text(cx), "hello from the server\n");
    });
}

fn init_test(cx: &mut gpui::TestAppContext) {
    cx.foreground().forbid_parking();

    cx.update(|cx| {
        cx.set_global(SettingsStore::test(cx));
        theme::init((), cx);
        language::init(cx);
        client::init_settings(cx);
        Project::init_settings(cx);
        editor::init_settings(cx);
    });
}
