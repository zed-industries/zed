use std::sync::Arc;

use crate::lsp_log::LogMenuItem;

use super::*;
use futures::StreamExt;
use gpui::{Context, TestAppContext, VisualTestContext};
use language::{
    tree_sitter_rust, FakeLspAdapter, Language, LanguageConfig, LanguageMatcher, LanguageServerName,
};
use project::{FakeFs, Project};
use serde_json::json;
use settings::SettingsStore;

#[gpui::test]
async fn test_lsp_logs(cx: &mut TestAppContext) {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }

    init_test(cx);

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(
        "/the-root",
        json!({
            "test.rs": "",
            "package.json": "",
        }),
    )
    .await;

    let project = Project::test(fs.clone(), ["/the-root".as_ref()], cx).await;

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
        Some(tree_sitter_rust::language()),
    )));
    let mut fake_rust_servers = language_registry.register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            name: "the-rust-language-server",
            ..Default::default()
        },
    );

    let log_store = cx.new_model(|cx| LogStore::new(cx));
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

    let window = cx.add_window(|cx| LspLogView::new(project.clone(), log_store.clone(), cx));
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
                    .worktrees()
                    .next()
                    .unwrap()
                    .read(cx)
                    .root_name()
                    .to_string(),
                rpc_trace_enabled: false,
                rpc_trace_selected: false,
                logs_selected: true,
            }]
        );
        assert_eq!(view.editor.read(cx).text(cx), "hello from the server\n");
    });
}

fn init_test(cx: &mut gpui::TestAppContext) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
        theme::init(theme::LoadThemes::JustBase, cx);
        release_channel::init("0.0.0", cx);
        language::init(cx);
        client::init_settings(cx);
        Project::init_settings(cx);
        editor::init_settings(cx);
    });
}
