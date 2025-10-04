use std::sync::Arc;

use crate::lsp_log_view::LogMenuItem;

use super::*;
use futures::StreamExt;
use gpui::{AppContext as _, SemanticVersion, TestAppContext, VisualTestContext};
use language::{FakeLspAdapter, Language, LanguageConfig, LanguageMatcher, tree_sitter_rust};
use lsp::LanguageServerName;
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
        workspace::init_settings(cx);
        theme::init(theme::LoadThemes::JustBase, cx);
        release_channel::init(SemanticVersion::default(), cx);
        language::init(cx);
        client::init_settings(cx);
        Project::init_settings(cx);
        editor::init_settings(cx);
    });
}
