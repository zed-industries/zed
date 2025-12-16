use crate::{rpc::RECONNECT_TIMEOUT, tests::TestServer};
use call::ActiveCall;
use editor::{
    DocumentColorsRenderMode, Editor, FETCH_COLORS_DEBOUNCE_TIMEOUT, MultiBufferOffset, RowInfo,
    SelectionEffects,
    actions::{
        ConfirmCodeAction, ConfirmCompletion, ConfirmRename, ContextMenuFirst, CopyFileLocation,
        CopyFileName, CopyFileNameWithoutExtension, ExpandMacroRecursively, MoveToEnd, Redo,
        Rename, SelectAll, ToggleCodeActions, Undo,
    },
    test::{
        editor_test_context::{AssertionContextManager, EditorTestContext},
        expand_macro_recursively,
    },
};
use fs::Fs;
use futures::{SinkExt, StreamExt, channel::mpsc, lock::Mutex};
use git::repository::repo_path;
use gpui::{
    App, Rgba, SharedString, TestAppContext, UpdateGlobal, VisualContext, VisualTestContext,
};
use indoc::indoc;
use language::{FakeLspAdapter, rust_lang};
use lsp::LSP_REQUEST_TIMEOUT;
use pretty_assertions::assert_eq;
use project::{
    ProgressToken, ProjectPath, SERVER_PROGRESS_THROTTLE_TIMEOUT,
    lsp_store::lsp_ext_command::{ExpandedMacro, LspExtExpandMacro},
};
use recent_projects::disconnected_overlay::DisconnectedOverlay;
use rpc::RECEIVE_TIMEOUT;
use serde_json::json;
use settings::{InlayHintSettingsContent, InlineBlameSettings, SettingsStore};
use std::{
    collections::BTreeSet,
    ops::{Deref as _, Range},
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{self, AtomicBool, AtomicUsize},
    },
    time::Duration,
};
use text::Point;
use util::{path, rel_path::rel_path, uri};
use workspace::{CloseIntent, Workspace};

#[gpui::test(iterations = 10)]
async fn test_host_disconnect(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;

    cx_b.update(editor::init);
    cx_b.update(recent_projects::init);

    client_a
        .fs()
        .insert_tree(
            "/a",
            json!({
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;

    let worktree_a = project_a.read_with(cx_a, |project, cx| project.worktrees(cx).next().unwrap());
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    cx_a.background_executor.run_until_parked();

    assert!(worktree_a.read_with(cx_a, |tree, _| tree.has_update_observer()));

    let workspace_b = cx_b.add_window(|window, cx| {
        Workspace::new(
            None,
            project_b.clone(),
            client_b.app_state.clone(),
            window,
            cx,
        )
    });
    let cx_b = &mut VisualTestContext::from_window(*workspace_b, cx_b);
    let workspace_b_view = workspace_b.root(cx_b).unwrap();

    let editor_b = workspace_b
        .update(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("b.txt")), None, true, window, cx)
        })
        .unwrap()
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    //TODO: focus
    assert!(cx_b.update_window_entity(&editor_b, |editor, window, _| editor.is_focused(window)));
    editor_b.update_in(cx_b, |editor, window, cx| editor.insert("X", window, cx));

    cx_b.update(|_, cx| {
        assert!(workspace_b_view.read(cx).is_edited());
    });

    // Drop client A's connection. Collaborators should disappear and the project should not be shown as shared.
    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    cx_a.background_executor
        .advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);

    project_a.read_with(cx_a, |project, _| project.collaborators().is_empty());

    project_a.read_with(cx_a, |project, _| assert!(!project.is_shared()));

    project_b.read_with(cx_b, |project, cx| project.is_read_only(cx));

    assert!(worktree_a.read_with(cx_a, |tree, _| !tree.has_update_observer()));

    // Ensure client B's edited state is reset and that the whole window is blurred.
    workspace_b
        .update(cx_b, |workspace, _, cx| {
            assert!(workspace.active_modal::<DisconnectedOverlay>(cx).is_some());
            assert!(!workspace.is_edited());
        })
        .unwrap();

    // Ensure client B is not prompted to save edits when closing window after disconnecting.
    let can_close = workspace_b
        .update(cx_b, |workspace, window, cx| {
            workspace.prepare_to_close(CloseIntent::Quit, window, cx)
        })
        .unwrap()
        .await
        .unwrap();
    assert!(can_close);

    // Allow client A to reconnect to the server.
    server.allow_connections();
    cx_a.background_executor.advance_clock(RECEIVE_TIMEOUT);

    // Client B calls client A again after they reconnected.
    let active_call_b = cx_b.read(ActiveCall::global);
    active_call_b
        .update(cx_b, |call, cx| {
            call.invite(client_a.user_id().unwrap(), None, cx)
        })
        .await
        .unwrap();
    cx_a.background_executor.run_until_parked();
    active_call_a
        .update(cx_a, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();

    active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Drop client A's connection again. We should still unshare it successfully.
    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    cx_a.background_executor
        .advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);

    project_a.read_with(cx_a, |project, _| assert!(!project.is_shared()));
}

#[gpui::test]
async fn test_newline_above_or_below_does_not_move_guest_cursor(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let executor = cx_a.executor();
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(path!("/dir"), json!({ "a.txt": "Some text\n" }))
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/dir"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Open a buffer as client A
    let buffer_a = project_a
        .update(cx_a, |p, cx| {
            p.open_buffer((worktree_id, rel_path("a.txt")), cx)
        })
        .await
        .unwrap();
    let cx_a = cx_a.add_empty_window();
    let editor_a = cx_a
        .new_window_entity(|window, cx| Editor::for_buffer(buffer_a, Some(project_a), window, cx));

    let mut editor_cx_a = EditorTestContext {
        cx: cx_a.clone(),
        window: cx_a.window_handle(),
        editor: editor_a,
        assertion_cx: AssertionContextManager::new(),
    };

    let cx_b = cx_b.add_empty_window();
    // Open a buffer as client B
    let buffer_b = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer((worktree_id, rel_path("a.txt")), cx)
        })
        .await
        .unwrap();
    let editor_b = cx_b
        .new_window_entity(|window, cx| Editor::for_buffer(buffer_b, Some(project_b), window, cx));

    let mut editor_cx_b = EditorTestContext {
        cx: cx_b.clone(),
        window: cx_b.window_handle(),
        editor: editor_b,
        assertion_cx: AssertionContextManager::new(),
    };

    // Test newline above
    editor_cx_a.set_selections_state(indoc! {"
        Some textˇ
    "});
    editor_cx_b.set_selections_state(indoc! {"
        Some textˇ
    "});
    editor_cx_a.update_editor(|editor, window, cx| {
        editor.newline_above(&editor::actions::NewlineAbove, window, cx)
    });
    executor.run_until_parked();
    editor_cx_a.assert_editor_state(indoc! {"
        ˇ
        Some text
    "});
    editor_cx_b.assert_editor_state(indoc! {"

        Some textˇ
    "});

    // Test newline below
    editor_cx_a.set_selections_state(indoc! {"

        Some textˇ
    "});
    editor_cx_b.set_selections_state(indoc! {"

        Some textˇ
    "});
    editor_cx_a.update_editor(|editor, window, cx| {
        editor.newline_below(&editor::actions::NewlineBelow, window, cx)
    });
    executor.run_until_parked();
    editor_cx_a.assert_editor_state(indoc! {"

        Some text
        ˇ
    "});
    editor_cx_b.assert_editor_state(indoc! {"

        Some textˇ

    "});
}

#[gpui::test]
async fn test_collaborating_with_completion(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    let capabilities = lsp::ServerCapabilities {
        completion_provider: Some(lsp::CompletionOptions {
            trigger_characters: Some(vec![".".to_string()]),
            resolve_provider: Some(true),
            ..lsp::CompletionOptions::default()
        }),
        ..lsp::ServerCapabilities::default()
    };
    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = [
        client_a.language_registry().register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                capabilities: capabilities.clone(),
                initializer: Some(Box::new(|fake_server| {
                    fake_server.set_request_handler::<lsp::request::Completion, _, _>(
                        |params, _| async move {
                            assert_eq!(
                                params.text_document_position.text_document.uri,
                                lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
                            );
                            assert_eq!(
                                params.text_document_position.position,
                                lsp::Position::new(0, 14),
                            );

                            Ok(Some(lsp::CompletionResponse::Array(vec![
                                lsp::CompletionItem {
                                    label: "first_method(…)".into(),
                                    detail: Some("fn(&mut self, B) -> C".into()),
                                    text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                                        new_text: "first_method($1)".to_string(),
                                        range: lsp::Range::new(
                                            lsp::Position::new(0, 14),
                                            lsp::Position::new(0, 14),
                                        ),
                                    })),
                                    insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                                    ..Default::default()
                                },
                                lsp::CompletionItem {
                                    label: "second_method(…)".into(),
                                    detail: Some("fn(&mut self, C) -> D<E>".into()),
                                    text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                                        new_text: "second_method()".to_string(),
                                        range: lsp::Range::new(
                                            lsp::Position::new(0, 14),
                                            lsp::Position::new(0, 14),
                                        ),
                                    })),
                                    insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                                    ..Default::default()
                                },
                            ])))
                        },
                    );
                })),
                ..FakeLspAdapter::default()
            },
        ),
        client_a.language_registry().register_fake_lsp(
            "Rust",
            FakeLspAdapter {
                name: "fake-analyzer",
                capabilities: capabilities.clone(),
                initializer: Some(Box::new(|fake_server| {
                    fake_server.set_request_handler::<lsp::request::Completion, _, _>(
                        |_, _| async move { Ok(None) },
                    );
                })),
                ..FakeLspAdapter::default()
            },
        ),
    ];
    client_b.language_registry().add(rust_lang());
    client_b.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            capabilities: capabilities.clone(),
            ..FakeLspAdapter::default()
        },
    );
    client_b.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            name: "fake-analyzer",
            capabilities,
            ..FakeLspAdapter::default()
        },
    );

    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() { a }",
                "other.rs": "",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Open a file in an editor as the guest.
    let buffer_b = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer((worktree_id, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();
    let cx_b = cx_b.add_empty_window();
    let editor_b = cx_b.new_window_entity(|window, cx| {
        Editor::for_buffer(buffer_b.clone(), Some(project_b.clone()), window, cx)
    });

    let fake_language_server = fake_language_servers[0].next().await.unwrap();
    let second_fake_language_server = fake_language_servers[1].next().await.unwrap();
    cx_a.background_executor.run_until_parked();
    cx_b.background_executor.run_until_parked();

    buffer_b.read_with(cx_b, |buffer, _| {
        assert!(!buffer.completion_triggers().is_empty())
    });

    // Type a completion trigger character as the guest.
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges([MultiBufferOffset(13)..MultiBufferOffset(13)])
        });
        editor.handle_input(".", window, cx);
    });
    cx_b.focus(&editor_b);

    // Allow the completion request to propagate from guest to host to LSP.
    cx_b.background_executor.run_until_parked();
    cx_a.background_executor.run_until_parked();

    // Open the buffer on the host.
    let buffer_a = project_a
        .update(cx_a, |p, cx| {
            p.open_buffer((worktree_id, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();
    cx_a.executor().run_until_parked();

    buffer_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a. }")
    });

    // Confirm a completion on the guest.
    editor_b.update_in(cx_b, |editor, window, cx| {
        assert!(editor.context_menu_visible());
        editor.confirm_completion(&ConfirmCompletion { item_ix: Some(0) }, window, cx);
        assert_eq!(editor.text(cx), "fn main() { a.first_method() }");
    });

    // Return a resolved completion from the host's language server.
    // The resolved completion has an additional text edit.
    fake_language_server.set_request_handler::<lsp::request::ResolveCompletionItem, _, _>(
        |params, _| async move {
            assert_eq!(params.label, "first_method(…)");
            Ok(lsp::CompletionItem {
                label: "first_method(…)".into(),
                detail: Some("fn(&mut self, B) -> C".into()),
                text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                    new_text: "first_method($1)".to_string(),
                    range: lsp::Range::new(lsp::Position::new(0, 14), lsp::Position::new(0, 14)),
                })),
                additional_text_edits: Some(vec![lsp::TextEdit {
                    new_text: "use d::SomeTrait;\n".to_string(),
                    range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 0)),
                }]),
                insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                ..Default::default()
            })
        },
    );

    // The additional edit is applied.
    cx_a.executor().run_until_parked();
    cx_b.executor().run_until_parked();

    buffer_a.read_with(cx_a, |buffer, _| {
        assert_eq!(
            buffer.text(),
            "use d::SomeTrait;\nfn main() { a.first_method() }"
        );
    });

    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(
            buffer.text(),
            "use d::SomeTrait;\nfn main() { a.first_method() }"
        );
    });

    // Now we do a second completion, this time to ensure that documentation/snippets are
    // resolved
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges([MultiBufferOffset(46)..MultiBufferOffset(46)])
        });
        editor.handle_input("; a", window, cx);
        editor.handle_input(".", window, cx);
    });

    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(
            buffer.text(),
            "use d::SomeTrait;\nfn main() { a.first_method(); a. }"
        );
    });

    let mut completion_response = fake_language_server
        .set_request_handler::<lsp::request::Completion, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri,
                lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
            );
            assert_eq!(
                params.text_document_position.position,
                lsp::Position::new(1, 32),
            );

            Ok(Some(lsp::CompletionResponse::Array(vec![
                lsp::CompletionItem {
                    label: "third_method(…)".into(),
                    detail: Some("fn(&mut self, B, C, D) -> E".into()),
                    text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                        // no snippet placeholders
                        new_text: "third_method".to_string(),
                        range: lsp::Range::new(
                            lsp::Position::new(1, 32),
                            lsp::Position::new(1, 32),
                        ),
                    })),
                    insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                    documentation: None,
                    ..Default::default()
                },
            ])))
        });

    // Second language server also needs to handle the request (returns None)
    let mut second_completion_response = second_fake_language_server
        .set_request_handler::<lsp::request::Completion, _, _>(|_, _| async move { Ok(None) });

    // The completion now gets a new `text_edit.new_text` when resolving the completion item
    let mut resolve_completion_response = fake_language_server
        .set_request_handler::<lsp::request::ResolveCompletionItem, _, _>(|params, _| async move {
            assert_eq!(params.label, "third_method(…)");
            Ok(lsp::CompletionItem {
                label: "third_method(…)".into(),
                detail: Some("fn(&mut self, B, C, D) -> E".into()),
                text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                    // Now it's a snippet
                    new_text: "third_method($1, $2, $3)".to_string(),
                    range: lsp::Range::new(lsp::Position::new(1, 32), lsp::Position::new(1, 32)),
                })),
                insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                documentation: Some(lsp::Documentation::String(
                    "this is the documentation".into(),
                )),
                ..Default::default()
            })
        });

    cx_b.executor().run_until_parked();

    completion_response.next().await.unwrap();
    second_completion_response.next().await.unwrap();

    editor_b.update_in(cx_b, |editor, window, cx| {
        assert!(editor.context_menu_visible());
        editor.context_menu_first(&ContextMenuFirst {}, window, cx);
    });

    resolve_completion_response.next().await.unwrap();
    cx_b.executor().run_until_parked();

    // When accepting the completion, the snippet is insert.
    editor_b.update_in(cx_b, |editor, window, cx| {
        assert!(editor.context_menu_visible());
        editor.confirm_completion(&ConfirmCompletion { item_ix: Some(0) }, window, cx);
        assert_eq!(
            editor.text(cx),
            "use d::SomeTrait;\nfn main() { a.first_method(); a.third_method(, , ) }"
        );
    });

    // Ensure buffer is synced before proceeding with the next test
    cx_a.executor().run_until_parked();
    cx_b.executor().run_until_parked();

    // Test completions from the second fake language server
    // Add another completion trigger to test the second language server
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges([MultiBufferOffset(68)..MultiBufferOffset(68)])
        });
        editor.handle_input("; b", window, cx);
        editor.handle_input(".", window, cx);
    });

    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(
            buffer.text(),
            "use d::SomeTrait;\nfn main() { a.first_method(); a.third_method(, , ); b. }"
        );
    });

    // Set up completion handlers for both language servers
    let mut first_lsp_completion = fake_language_server
        .set_request_handler::<lsp::request::Completion, _, _>(|_, _| async move { Ok(None) });

    let mut second_lsp_completion = second_fake_language_server
        .set_request_handler::<lsp::request::Completion, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri,
                lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
            );
            assert_eq!(
                params.text_document_position.position,
                lsp::Position::new(1, 54),
            );

            Ok(Some(lsp::CompletionResponse::Array(vec![
                lsp::CompletionItem {
                    label: "analyzer_method(…)".into(),
                    detail: Some("fn(&self) -> Result<T>".into()),
                    text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
                        new_text: "analyzer_method()".to_string(),
                        range: lsp::Range::new(
                            lsp::Position::new(1, 54),
                            lsp::Position::new(1, 54),
                        ),
                    })),
                    insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
                    ..lsp::CompletionItem::default()
                },
            ])))
        });

    // Await both language server responses
    first_lsp_completion.next().await.unwrap();
    second_lsp_completion.next().await.unwrap();

    cx_b.executor().run_until_parked();

    // Confirm the completion from the second language server works
    editor_b.update_in(cx_b, |editor, window, cx| {
        assert!(editor.context_menu_visible());
        editor.confirm_completion(&ConfirmCompletion { item_ix: Some(0) }, window, cx);
        assert_eq!(
            editor.text(cx),
            "use d::SomeTrait;\nfn main() { a.first_method(); a.third_method(, , ); b.analyzer_method() }"
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_collaborating_with_code_actions(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    //
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    cx_b.update(editor::init);

    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a
        .language_registry()
        .register_fake_lsp("Rust", FakeLspAdapter::default());
    client_b.language_registry().add(rust_lang());
    client_b
        .language_registry()
        .register_fake_lsp("Rust", FakeLspAdapter::default());

    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "main.rs": "mod other;\nfn main() { let foo = other::foo(); }",
                "other.rs": "pub fn foo() -> usize { 4 }",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Join the project as client B.
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    let editor_b = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("main.rs")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let mut fake_language_server = fake_language_servers.next().await.unwrap();
    let mut requests = fake_language_server
        .set_request_handler::<lsp::request::CodeActionRequest, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
            );
            assert_eq!(params.range.start, lsp::Position::new(0, 0));
            assert_eq!(params.range.end, lsp::Position::new(0, 0));
            Ok(None)
        });
    cx_a.background_executor
        .advance_clock(editor::CODE_ACTIONS_DEBOUNCE_TIMEOUT * 2);
    requests.next().await;

    // Move cursor to a location that contains code actions.
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges([Point::new(1, 31)..Point::new(1, 31)])
        });
    });
    cx_b.focus(&editor_b);

    let mut requests = fake_language_server
        .set_request_handler::<lsp::request::CodeActionRequest, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
            );
            assert_eq!(params.range.start, lsp::Position::new(1, 31));
            assert_eq!(params.range.end, lsp::Position::new(1, 31));

            Ok(Some(vec![lsp::CodeActionOrCommand::CodeAction(
                lsp::CodeAction {
                    title: "Inline into all callers".to_string(),
                    edit: Some(lsp::WorkspaceEdit {
                        changes: Some(
                            [
                                (
                                    lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
                                    vec![lsp::TextEdit::new(
                                        lsp::Range::new(
                                            lsp::Position::new(1, 22),
                                            lsp::Position::new(1, 34),
                                        ),
                                        "4".to_string(),
                                    )],
                                ),
                                (
                                    lsp::Uri::from_file_path(path!("/a/other.rs")).unwrap(),
                                    vec![lsp::TextEdit::new(
                                        lsp::Range::new(
                                            lsp::Position::new(0, 0),
                                            lsp::Position::new(0, 27),
                                        ),
                                        "".to_string(),
                                    )],
                                ),
                            ]
                            .into_iter()
                            .collect(),
                        ),
                        ..Default::default()
                    }),
                    data: Some(json!({
                        "codeActionParams": {
                            "range": {
                                "start": {"line": 1, "column": 31},
                                "end": {"line": 1, "column": 31},
                            }
                        }
                    })),
                    ..Default::default()
                },
            )]))
        });
    cx_a.background_executor
        .advance_clock(editor::CODE_ACTIONS_DEBOUNCE_TIMEOUT * 2);
    requests.next().await;

    // Toggle code actions and wait for them to display.
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.toggle_code_actions(
            &ToggleCodeActions {
                deployed_from: None,
                quick_launch: false,
            },
            window,
            cx,
        );
    });
    cx_a.background_executor.run_until_parked();

    editor_b.update(cx_b, |editor, _| assert!(editor.context_menu_visible()));

    fake_language_server.remove_request_handler::<lsp::request::CodeActionRequest>();

    // Confirming the code action will trigger a resolve request.
    let confirm_action = editor_b
        .update_in(cx_b, |editor, window, cx| {
            Editor::confirm_code_action(editor, &ConfirmCodeAction { item_ix: Some(0) }, window, cx)
        })
        .unwrap();
    fake_language_server.set_request_handler::<lsp::request::CodeActionResolveRequest, _, _>(
        |_, _| async move {
            Ok(lsp::CodeAction {
                title: "Inline into all callers".to_string(),
                edit: Some(lsp::WorkspaceEdit {
                    changes: Some(
                        [
                            (
                                lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
                                vec![lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(1, 22),
                                        lsp::Position::new(1, 34),
                                    ),
                                    "4".to_string(),
                                )],
                            ),
                            (
                                lsp::Uri::from_file_path(path!("/a/other.rs")).unwrap(),
                                vec![lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(0, 0),
                                        lsp::Position::new(0, 27),
                                    ),
                                    "".to_string(),
                                )],
                            ),
                        ]
                        .into_iter()
                        .collect(),
                    ),
                    ..Default::default()
                }),
                ..Default::default()
            })
        },
    );

    // After the action is confirmed, an editor containing both modified files is opened.
    confirm_action.await.unwrap();

    let code_action_editor = workspace_b.update(cx_b, |workspace, cx| {
        workspace
            .active_item(cx)
            .unwrap()
            .downcast::<Editor>()
            .unwrap()
    });
    code_action_editor.update_in(cx_b, |editor, window, cx| {
        assert_eq!(editor.text(cx), "mod other;\nfn main() { let foo = 4; }\n");
        editor.undo(&Undo, window, cx);
        assert_eq!(
            editor.text(cx),
            "mod other;\nfn main() { let foo = other::foo(); }\npub fn foo() -> usize { 4 }"
        );
        editor.redo(&Redo, window, cx);
        assert_eq!(editor.text(cx), "mod other;\nfn main() { let foo = 4; }\n");
    });
}

#[gpui::test(iterations = 10)]
async fn test_collaborating_with_renames(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    cx_b.update(editor::init);

    let capabilities = lsp::ServerCapabilities {
        rename_provider: Some(lsp::OneOf::Right(lsp::RenameOptions {
            prepare_provider: Some(true),
            work_done_progress_options: Default::default(),
        })),
        ..lsp::ServerCapabilities::default()
    };
    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: capabilities.clone(),
            ..FakeLspAdapter::default()
        },
    );
    client_b.language_registry().add(rust_lang());
    client_b.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            capabilities,
            ..FakeLspAdapter::default()
        },
    );

    client_a
        .fs()
        .insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;"
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/dir"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    let editor_b = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("one.rs")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    let fake_language_server = fake_language_servers.next().await.unwrap();
    cx_a.run_until_parked();
    cx_b.run_until_parked();

    // Move cursor to a location that can be renamed.
    let prepare_rename = editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges([MultiBufferOffset(7)..MultiBufferOffset(7)])
        });
        editor.rename(&Rename, window, cx).unwrap()
    });

    fake_language_server
        .set_request_handler::<lsp::request::PrepareRenameRequest, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document.uri.as_str(),
                uri!("file:///dir/one.rs")
            );
            assert_eq!(params.position, lsp::Position::new(0, 7));
            Ok(Some(lsp::PrepareRenameResponse::Range(lsp::Range::new(
                lsp::Position::new(0, 6),
                lsp::Position::new(0, 9),
            ))))
        })
        .next()
        .await
        .unwrap();
    prepare_rename.await.unwrap();
    editor_b.update(cx_b, |editor, cx| {
        use editor::ToOffset;
        let rename = editor.pending_rename().unwrap();
        let buffer = editor.buffer().read(cx).snapshot(cx);
        assert_eq!(
            rename.range.start.to_offset(&buffer)..rename.range.end.to_offset(&buffer),
            MultiBufferOffset(6)..MultiBufferOffset(9)
        );
        rename.editor.update(cx, |rename_editor, cx| {
            let rename_selection = rename_editor.selections.newest::<MultiBufferOffset>(&rename_editor.display_snapshot(cx));
            assert_eq!(
                rename_selection.range(),
                MultiBufferOffset(0)..MultiBufferOffset(3),
                "Rename that was triggered from zero selection caret, should propose the whole word."
            );
            rename_editor.buffer().update(cx, |rename_buffer, cx| {
                rename_buffer.edit([(MultiBufferOffset(0)..MultiBufferOffset(3), "THREE")], None, cx);
            });
        });
    });

    // Cancel the rename, and repeat the same, but use selections instead of cursor movement
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.cancel(&editor::actions::Cancel, window, cx);
    });
    let prepare_rename = editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges([MultiBufferOffset(7)..MultiBufferOffset(8)])
        });
        editor.rename(&Rename, window, cx).unwrap()
    });

    fake_language_server
        .set_request_handler::<lsp::request::PrepareRenameRequest, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document.uri.as_str(),
                uri!("file:///dir/one.rs")
            );
            assert_eq!(params.position, lsp::Position::new(0, 8));
            Ok(Some(lsp::PrepareRenameResponse::Range(lsp::Range::new(
                lsp::Position::new(0, 6),
                lsp::Position::new(0, 9),
            ))))
        })
        .next()
        .await
        .unwrap();
    prepare_rename.await.unwrap();
    editor_b.update(cx_b, |editor, cx| {
        use editor::ToOffset;
        let rename = editor.pending_rename().unwrap();
        let buffer = editor.buffer().read(cx).snapshot(cx);
        let lsp_rename_start = rename.range.start.to_offset(&buffer);
        let lsp_rename_end = rename.range.end.to_offset(&buffer);
        assert_eq!(lsp_rename_start..lsp_rename_end, MultiBufferOffset(6)..MultiBufferOffset(9));
        rename.editor.update(cx, |rename_editor, cx| {
            let rename_selection = rename_editor.selections.newest::<MultiBufferOffset>(&rename_editor.display_snapshot(cx));
            assert_eq!(
                rename_selection.range(),
                MultiBufferOffset(1)..MultiBufferOffset(2),
                "Rename that was triggered from a selection, should have the same selection range in the rename proposal"
            );
            rename_editor.buffer().update(cx, |rename_buffer, cx| {
                rename_buffer.edit([(MultiBufferOffset(0)..MultiBufferOffset(lsp_rename_end - lsp_rename_start), "THREE")], None, cx);
            });
        });
    });

    let confirm_rename = editor_b.update_in(cx_b, |editor, window, cx| {
        Editor::confirm_rename(editor, &ConfirmRename, window, cx).unwrap()
    });
    fake_language_server
        .set_request_handler::<lsp::request::Rename, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri.as_str(),
                uri!("file:///dir/one.rs")
            );
            assert_eq!(
                params.text_document_position.position,
                lsp::Position::new(0, 6)
            );
            assert_eq!(params.new_name, "THREE");
            Ok(Some(lsp::WorkspaceEdit {
                changes: Some(
                    [
                        (
                            lsp::Uri::from_file_path(path!("/dir/one.rs")).unwrap(),
                            vec![lsp::TextEdit::new(
                                lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
                                "THREE".to_string(),
                            )],
                        ),
                        (
                            lsp::Uri::from_file_path(path!("/dir/two.rs")).unwrap(),
                            vec![
                                lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(0, 24),
                                        lsp::Position::new(0, 27),
                                    ),
                                    "THREE".to_string(),
                                ),
                                lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(0, 35),
                                        lsp::Position::new(0, 38),
                                    ),
                                    "THREE".to_string(),
                                ),
                            ],
                        ),
                    ]
                    .into_iter()
                    .collect(),
                ),
                ..Default::default()
            }))
        })
        .next()
        .await
        .unwrap();
    confirm_rename.await.unwrap();

    let rename_editor = workspace_b.update(cx_b, |workspace, cx| {
        workspace.active_item_as::<Editor>(cx).unwrap()
    });

    rename_editor.update_in(cx_b, |editor, window, cx| {
        assert_eq!(
            editor.text(cx),
            "const THREE: usize = 1;\nconst TWO: usize = one::THREE + one::THREE;"
        );
        editor.undo(&Undo, window, cx);
        assert_eq!(
            editor.text(cx),
            "const ONE: usize = 1;\nconst TWO: usize = one::ONE + one::ONE;"
        );
        editor.redo(&Redo, window, cx);
        assert_eq!(
            editor.text(cx),
            "const THREE: usize = 1;\nconst TWO: usize = one::THREE + one::THREE;"
        );
    });

    // Ensure temporary rename edits cannot be undone/redone.
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.undo(&Undo, window, cx);
        assert_eq!(editor.text(cx), "const ONE: usize = 1;");
        editor.undo(&Undo, window, cx);
        assert_eq!(editor.text(cx), "const ONE: usize = 1;");
        editor.redo(&Redo, window, cx);
        assert_eq!(editor.text(cx), "const THREE: usize = 1;");
    })
}

#[gpui::test]
async fn test_slow_lsp_server(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    cx_b.update(editor::init);

    let command_name = "test_command";
    let capabilities = lsp::ServerCapabilities {
        code_lens_provider: Some(lsp::CodeLensOptions {
            resolve_provider: None,
        }),
        execute_command_provider: Some(lsp::ExecuteCommandOptions {
            commands: vec![command_name.to_string()],
            ..lsp::ExecuteCommandOptions::default()
        }),
        ..lsp::ServerCapabilities::default()
    };
    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: capabilities.clone(),
            ..FakeLspAdapter::default()
        },
    );
    client_b.language_registry().add(rust_lang());
    client_b.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            capabilities,
            ..FakeLspAdapter::default()
        },
    );

    client_a
        .fs()
        .insert_tree(
            path!("/dir"),
            json!({
                "one.rs": "const ONE: usize = 1;"
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/dir"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    let editor_b = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("one.rs")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    let (lsp_store_b, buffer_b) = editor_b.update(cx_b, |editor, cx| {
        let lsp_store = editor.project().unwrap().read(cx).lsp_store();
        let buffer = editor.buffer().read(cx).as_singleton().unwrap();
        (lsp_store, buffer)
    });
    let fake_language_server = fake_language_servers.next().await.unwrap();
    cx_a.run_until_parked();
    cx_b.run_until_parked();

    let long_request_time = LSP_REQUEST_TIMEOUT / 2;
    let (request_started_tx, mut request_started_rx) = mpsc::unbounded();
    let requests_started = Arc::new(AtomicUsize::new(0));
    let requests_completed = Arc::new(AtomicUsize::new(0));
    let _lens_requests = fake_language_server
        .set_request_handler::<lsp::request::CodeLensRequest, _, _>({
            let request_started_tx = request_started_tx.clone();
            let requests_started = requests_started.clone();
            let requests_completed = requests_completed.clone();
            move |params, cx| {
                let mut request_started_tx = request_started_tx.clone();
                let requests_started = requests_started.clone();
                let requests_completed = requests_completed.clone();
                async move {
                    assert_eq!(
                        params.text_document.uri.as_str(),
                        uri!("file:///dir/one.rs")
                    );
                    requests_started.fetch_add(1, atomic::Ordering::Release);
                    request_started_tx.send(()).await.unwrap();
                    cx.background_executor().timer(long_request_time).await;
                    let i = requests_completed.fetch_add(1, atomic::Ordering::Release) + 1;
                    Ok(Some(vec![lsp::CodeLens {
                        range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 9)),
                        command: Some(lsp::Command {
                            title: format!("LSP Command {i}"),
                            command: command_name.to_string(),
                            arguments: None,
                        }),
                        data: None,
                    }]))
                }
            }
        });

    // Move cursor to a location, this should trigger the code lens call.
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges([MultiBufferOffset(7)..MultiBufferOffset(7)])
        });
    });
    let () = request_started_rx.next().await.unwrap();
    assert_eq!(
        requests_started.load(atomic::Ordering::Acquire),
        1,
        "Selection change should have initiated the first request"
    );
    assert_eq!(
        requests_completed.load(atomic::Ordering::Acquire),
        0,
        "Slow requests should be running still"
    );
    let _first_task = lsp_store_b.update(cx_b, |lsp_store, cx| {
        lsp_store
            .forget_code_lens_task(buffer_b.read(cx).remote_id())
            .expect("Should have the fetch task started")
    });

    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges([MultiBufferOffset(1)..MultiBufferOffset(1)])
        });
    });
    let () = request_started_rx.next().await.unwrap();
    assert_eq!(
        requests_started.load(atomic::Ordering::Acquire),
        2,
        "Selection change should have initiated the second request"
    );
    assert_eq!(
        requests_completed.load(atomic::Ordering::Acquire),
        0,
        "Slow requests should be running still"
    );
    let _second_task = lsp_store_b.update(cx_b, |lsp_store, cx| {
        lsp_store
            .forget_code_lens_task(buffer_b.read(cx).remote_id())
            .expect("Should have the fetch task started for the 2nd time")
    });

    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges([MultiBufferOffset(2)..MultiBufferOffset(2)])
        });
    });
    let () = request_started_rx.next().await.unwrap();
    assert_eq!(
        requests_started.load(atomic::Ordering::Acquire),
        3,
        "Selection change should have initiated the third request"
    );
    assert_eq!(
        requests_completed.load(atomic::Ordering::Acquire),
        0,
        "Slow requests should be running still"
    );

    _first_task.await.unwrap();
    _second_task.await.unwrap();
    cx_b.run_until_parked();
    assert_eq!(
        requests_started.load(atomic::Ordering::Acquire),
        3,
        "No selection changes should trigger no more code lens requests"
    );
    assert_eq!(
        requests_completed.load(atomic::Ordering::Acquire),
        3,
        "After enough time, all 3 LSP requests should have been served by the language server"
    );
    let resulting_lens_actions = editor_b
        .update(cx_b, |editor, cx| {
            let lsp_store = editor.project().unwrap().read(cx).lsp_store();
            lsp_store.update(cx, |lsp_store, cx| {
                lsp_store.code_lens_actions(&buffer_b, cx)
            })
        })
        .await
        .unwrap()
        .unwrap();
    assert_eq!(
        resulting_lens_actions.len(),
        1,
        "Should have fetched one code lens action, but got: {resulting_lens_actions:?}"
    );
    assert_eq!(
        resulting_lens_actions.first().unwrap().lsp_action.title(),
        "LSP Command 3",
        "Only the final code lens action should be in the data"
    )
}

#[gpui::test(iterations = 10)]
async fn test_language_server_statuses(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let executor = cx_a.executor();
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    cx_b.update(editor::init);

    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "the-language-server",
            ..Default::default()
        },
    );

    client_a
        .fs()
        .insert_tree(
            path!("/dir"),
            json!({
                "main.rs": "const ONE: usize = 1;",
            }),
        )
        .await;
    let (project_a, _) = client_a.build_local_project(path!("/dir"), cx_a).await;

    let _buffer_a = project_a
        .update(cx_a, |p, cx| {
            p.open_local_buffer_with_lsp(path!("/dir/main.rs"), cx)
        })
        .await
        .unwrap();

    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.start_progress("the-token").await;

    executor.advance_clock(SERVER_PROGRESS_THROTTLE_TIMEOUT);
    fake_language_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
        token: lsp::NumberOrString::String("the-token".to_string()),
        value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::Report(
            lsp::WorkDoneProgressReport {
                message: Some("the-message".to_string()),
                ..Default::default()
            },
        )),
    });
    executor.run_until_parked();

    let token = ProgressToken::String(SharedString::from("the-token"));

    project_a.read_with(cx_a, |project, cx| {
        let status = project.language_server_statuses(cx).next().unwrap().1;
        assert_eq!(status.name.0, "the-language-server");
        assert_eq!(status.pending_work.len(), 1);
        assert_eq!(
            status.pending_work[&token].message.as_ref().unwrap(),
            "the-message"
        );
    });

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    executor.run_until_parked();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    project_b.read_with(cx_b, |project, cx| {
        let status = project.language_server_statuses(cx).next().unwrap().1;
        assert_eq!(status.name.0, "the-language-server");
    });

    executor.advance_clock(SERVER_PROGRESS_THROTTLE_TIMEOUT);
    fake_language_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
        token: lsp::NumberOrString::String("the-token".to_string()),
        value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::Report(
            lsp::WorkDoneProgressReport {
                message: Some("the-message-2".to_string()),
                ..Default::default()
            },
        )),
    });
    executor.run_until_parked();

    project_a.read_with(cx_a, |project, cx| {
        let status = project.language_server_statuses(cx).next().unwrap().1;
        assert_eq!(status.name.0, "the-language-server");
        assert_eq!(status.pending_work.len(), 1);
        assert_eq!(
            status.pending_work[&token].message.as_ref().unwrap(),
            "the-message-2"
        );
    });

    project_b.read_with(cx_b, |project, cx| {
        let status = project.language_server_statuses(cx).next().unwrap().1;
        assert_eq!(status.name.0, "the-language-server");
        assert_eq!(status.pending_work.len(), 1);
        assert_eq!(
            status.pending_work[&token].message.as_ref().unwrap(),
            "the-message-2"
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_share_project(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    let executor = cx_a.executor();
    let cx_b = cx_b.add_empty_window();
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;
    server
        .make_contacts(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);
    let active_call_c = cx_c.read(ActiveCall::global);

    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                ".gitignore": "ignored-dir",
                "a.txt": "a-contents",
                "b.txt": "b-contents",
                "ignored-dir": {
                    "c.txt": "",
                    "d.txt": "",
                }
            }),
        )
        .await;

    // Invite client B to collaborate on a project
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| {
            call.invite(client_b.user_id().unwrap(), Some(project_a.clone()), cx)
        })
        .await
        .unwrap();

    // Join that project as client B

    let incoming_call_b = active_call_b.read_with(cx_b, |call, _| call.incoming());
    executor.run_until_parked();
    let call = incoming_call_b.borrow().clone().unwrap();
    assert_eq!(call.calling_user.github_login, "user_a");
    let initial_project = call.initial_project.unwrap();
    active_call_b
        .update(cx_b, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();
    let client_b_peer_id = client_b.peer_id().unwrap();
    let project_b = client_b.join_remote_project(initial_project.id, cx_b).await;

    let replica_id_b = project_b.read_with(cx_b, |project, _| project.replica_id());

    executor.run_until_parked();

    project_a.read_with(cx_a, |project, _| {
        let client_b_collaborator = project.collaborators().get(&client_b_peer_id).unwrap();
        assert_eq!(client_b_collaborator.replica_id, replica_id_b);
    });

    project_b.read_with(cx_b, |project, cx| {
        let worktree = project.worktrees(cx).next().unwrap().read(cx);
        assert_eq!(
            worktree.paths().collect::<Vec<_>>(),
            [
                rel_path(".gitignore"),
                rel_path("a.txt"),
                rel_path("b.txt"),
                rel_path("ignored-dir"),
            ]
        );
    });

    project_b
        .update(cx_b, |project, cx| {
            let worktree = project.worktrees(cx).next().unwrap();
            let entry = worktree
                .read(cx)
                .entry_for_path(rel_path("ignored-dir"))
                .unwrap();
            project.expand_entry(worktree_id, entry.id, cx).unwrap()
        })
        .await
        .unwrap();

    project_b.read_with(cx_b, |project, cx| {
        let worktree = project.worktrees(cx).next().unwrap().read(cx);
        assert_eq!(
            worktree.paths().collect::<Vec<_>>(),
            [
                rel_path(".gitignore"),
                rel_path("a.txt"),
                rel_path("b.txt"),
                rel_path("ignored-dir"),
                rel_path("ignored-dir/c.txt"),
                rel_path("ignored-dir/d.txt"),
            ]
        );
    });

    // Open the same file as client B and client A.
    let buffer_b = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer((worktree_id, rel_path("b.txt")), cx)
        })
        .await
        .unwrap();

    buffer_b.read_with(cx_b, |buf, _| assert_eq!(buf.text(), "b-contents"));

    project_a.read_with(cx_a, |project, cx| {
        assert!(project.has_open_buffer((worktree_id, rel_path("b.txt")), cx))
    });
    let buffer_a = project_a
        .update(cx_a, |p, cx| {
            p.open_buffer((worktree_id, rel_path("b.txt")), cx)
        })
        .await
        .unwrap();

    let editor_b =
        cx_b.new_window_entity(|window, cx| Editor::for_buffer(buffer_b, None, window, cx));

    // Client A sees client B's selection
    executor.run_until_parked();

    buffer_a.read_with(cx_a, |buffer, _| {
        buffer
            .snapshot()
            .selections_in_range(
                text::Anchor::min_max_range_for_buffer(buffer.remote_id()),
                false,
            )
            .count()
            == 1
    });

    // Edit the buffer as client B and see that edit as client A.
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.handle_input("ok, ", window, cx)
    });
    executor.run_until_parked();

    buffer_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.text(), "ok, b-contents")
    });

    // Client B can invite client C on a project shared by client A.
    active_call_b
        .update(cx_b, |call, cx| {
            call.invite(client_c.user_id().unwrap(), Some(project_b.clone()), cx)
        })
        .await
        .unwrap();

    let incoming_call_c = active_call_c.read_with(cx_c, |call, _| call.incoming());
    executor.run_until_parked();
    let call = incoming_call_c.borrow().clone().unwrap();
    assert_eq!(call.calling_user.github_login, "user_b");
    let initial_project = call.initial_project.unwrap();
    active_call_c
        .update(cx_c, |call, cx| call.accept_incoming(cx))
        .await
        .unwrap();
    let _project_c = client_c.join_remote_project(initial_project.id, cx_c).await;

    // Client B closes the editor, and client A sees client B's selections removed.
    cx_b.update(move |_, _| drop(editor_b));
    executor.run_until_parked();

    buffer_a.read_with(cx_a, |buffer, _| {
        buffer
            .snapshot()
            .selections_in_range(
                text::Anchor::min_max_range_for_buffer(buffer.remote_id()),
                false,
            )
            .count()
            == 0
    });
}

#[gpui::test(iterations = 10)]
async fn test_on_input_format_from_host_to_guest(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let executor = cx_a.executor();
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                document_on_type_formatting_provider: Some(lsp::DocumentOnTypeFormattingOptions {
                    first_trigger_character: ":".to_string(),
                    more_trigger_character: Some(vec![">".to_string()]),
                }),
                ..Default::default()
            },
            ..Default::default()
        },
    );

    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() { a }",
                "other.rs": "// Test file",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Open a file in an editor as the host.
    let buffer_a = project_a
        .update(cx_a, |p, cx| {
            p.open_buffer((worktree_id, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();
    let cx_a = cx_a.add_empty_window();
    let editor_a = cx_a.new_window_entity(|window, cx| {
        Editor::for_buffer(buffer_a, Some(project_a.clone()), window, cx)
    });

    let fake_language_server = fake_language_servers.next().await.unwrap();
    executor.run_until_parked();

    // Receive an OnTypeFormatting request as the host's language server.
    // Return some formatting from the host's language server.
    fake_language_server.set_request_handler::<lsp::request::OnTypeFormatting, _, _>(
        |params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri,
                lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
            );
            assert_eq!(
                params.text_document_position.position,
                lsp::Position::new(0, 14),
            );

            Ok(Some(vec![lsp::TextEdit {
                new_text: "~<".to_string(),
                range: lsp::Range::new(lsp::Position::new(0, 14), lsp::Position::new(0, 14)),
            }]))
        },
    );

    // Open the buffer on the guest and see that the formatting worked
    let buffer_b = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer((worktree_id, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();

    // Type a on type formatting trigger character as the guest.
    cx_a.focus(&editor_a);
    editor_a.update_in(cx_a, |editor, window, cx| {
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges([MultiBufferOffset(13)..MultiBufferOffset(13)])
        });
        editor.handle_input(">", window, cx);
    });

    executor.run_until_parked();

    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a>~< }")
    });

    // Undo should remove LSP edits first
    editor_a.update_in(cx_a, |editor, window, cx| {
        assert_eq!(editor.text(cx), "fn main() { a>~< }");
        editor.undo(&Undo, window, cx);
        assert_eq!(editor.text(cx), "fn main() { a> }");
    });
    executor.run_until_parked();

    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a> }")
    });

    editor_a.update_in(cx_a, |editor, window, cx| {
        assert_eq!(editor.text(cx), "fn main() { a> }");
        editor.undo(&Undo, window, cx);
        assert_eq!(editor.text(cx), "fn main() { a }");
    });
    executor.run_until_parked();

    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a }")
    });
}

#[gpui::test(iterations = 10)]
async fn test_on_input_format_from_guest_to_host(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let executor = cx_a.executor();
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    let capabilities = lsp::ServerCapabilities {
        document_on_type_formatting_provider: Some(lsp::DocumentOnTypeFormattingOptions {
            first_trigger_character: ":".to_string(),
            more_trigger_character: Some(vec![">".to_string()]),
        }),
        ..lsp::ServerCapabilities::default()
    };
    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: capabilities.clone(),
            ..FakeLspAdapter::default()
        },
    );
    client_b.language_registry().add(rust_lang());
    client_b.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            capabilities,
            ..FakeLspAdapter::default()
        },
    );

    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() { a }",
                "other.rs": "// Test file",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    // Open a file in an editor as the guest.
    let buffer_b = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer((worktree_id, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();
    let cx_b = cx_b.add_empty_window();
    let editor_b = cx_b.new_window_entity(|window, cx| {
        Editor::for_buffer(buffer_b, Some(project_b.clone()), window, cx)
    });

    let fake_language_server = fake_language_servers.next().await.unwrap();
    executor.run_until_parked();

    // Type a on type formatting trigger character as the guest.
    cx_b.focus(&editor_b);
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges([MultiBufferOffset(13)..MultiBufferOffset(13)])
        });
        editor.handle_input(":", window, cx);
    });

    // Receive an OnTypeFormatting request as the host's language server.
    // Return some formatting from the host's language server.
    executor.start_waiting();
    fake_language_server
        .set_request_handler::<lsp::request::OnTypeFormatting, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri,
                lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
            );
            assert_eq!(
                params.text_document_position.position,
                lsp::Position::new(0, 14),
            );

            Ok(Some(vec![lsp::TextEdit {
                new_text: "~:".to_string(),
                range: lsp::Range::new(lsp::Position::new(0, 14), lsp::Position::new(0, 14)),
            }]))
        })
        .next()
        .await
        .unwrap();
    executor.finish_waiting();

    // Open the buffer on the host and see that the formatting worked
    let buffer_a = project_a
        .update(cx_a, |p, cx| {
            p.open_buffer((worktree_id, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();
    executor.run_until_parked();

    buffer_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a:~: }")
    });

    // Undo should remove LSP edits first
    editor_b.update_in(cx_b, |editor, window, cx| {
        assert_eq!(editor.text(cx), "fn main() { a:~: }");
        editor.undo(&Undo, window, cx);
        assert_eq!(editor.text(cx), "fn main() { a: }");
    });
    executor.run_until_parked();

    buffer_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a: }")
    });

    editor_b.update_in(cx_b, |editor, window, cx| {
        assert_eq!(editor.text(cx), "fn main() { a: }");
        editor.undo(&Undo, window, cx);
        assert_eq!(editor.text(cx), "fn main() { a }");
    });
    executor.run_until_parked();

    buffer_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a }")
    });
}

#[gpui::test(iterations = 10)]
async fn test_mutual_editor_inlay_hint_cache_update(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let executor = cx_a.executor();
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    cx_a.update(editor::init);
    cx_b.update(editor::init);

    cx_a.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.all_languages.defaults.inlay_hints =
                    Some(InlayHintSettingsContent {
                        enabled: Some(true),
                        ..InlayHintSettingsContent::default()
                    })
            });
        });
    });
    cx_b.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.all_languages.defaults.inlay_hints =
                    Some(InlayHintSettingsContent {
                        enabled: Some(true),
                        ..InlayHintSettingsContent::default()
                    })
            });
        });
    });

    let capabilities = lsp::ServerCapabilities {
        inlay_hint_provider: Some(lsp::OneOf::Left(true)),
        ..lsp::ServerCapabilities::default()
    };
    client_a.language_registry().add(rust_lang());

    // Set up the language server to return an additional inlay hint on each request.
    let edits_made = Arc::new(AtomicUsize::new(0));
    let closure_edits_made = Arc::clone(&edits_made);
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: capabilities.clone(),
            initializer: Some(Box::new(move |fake_language_server| {
                let closure_edits_made = closure_edits_made.clone();
                fake_language_server.set_request_handler::<lsp::request::InlayHintRequest, _, _>(
                    move |params, _| {
                        let edits_made_2 = Arc::clone(&closure_edits_made);
                        async move {
                            assert_eq!(
                                params.text_document.uri,
                                lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
                            );
                            let edits_made =
                                AtomicUsize::load(&edits_made_2, atomic::Ordering::Acquire);
                            Ok(Some(vec![lsp::InlayHint {
                                position: lsp::Position::new(0, edits_made as u32),
                                label: lsp::InlayHintLabel::String(edits_made.to_string()),
                                kind: None,
                                text_edits: None,
                                tooltip: None,
                                padding_left: None,
                                padding_right: None,
                                data: None,
                            }]))
                        }
                    },
                );
            })),
            ..FakeLspAdapter::default()
        },
    );
    client_b.language_registry().add(rust_lang());
    client_b.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            capabilities,
            ..FakeLspAdapter::default()
        },
    );

    // Client A opens a project.
    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() { a } // and some long comment to ensure inlay hints are not trimmed out",
                "other.rs": "// Test file",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Client B joins the project
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);

    // The host opens a rust file.
    let file_a = workspace_a.update_in(cx_a, |workspace, window, cx| {
        workspace.open_path((worktree_id, rel_path("main.rs")), None, true, window, cx)
    });
    let fake_language_server = fake_language_servers.next().await.unwrap();
    let editor_a = file_a.await.unwrap().downcast::<Editor>().unwrap();
    executor.advance_clock(Duration::from_millis(100));
    executor.run_until_parked();

    let initial_edit = edits_made.load(atomic::Ordering::Acquire);
    editor_a.update(cx_a, |editor, cx| {
        assert_eq!(
            vec![initial_edit.to_string()],
            extract_hint_labels(editor, cx),
            "Host should get its first hints when opens an editor"
        );
    });
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    let editor_b = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("main.rs")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    executor.advance_clock(Duration::from_millis(100));
    executor.run_until_parked();
    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(
            vec![initial_edit.to_string()],
            extract_hint_labels(editor, cx),
            "Client should get its first hints when opens an editor"
        );
    });

    let after_client_edit = edits_made.fetch_add(1, atomic::Ordering::Release) + 1;
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges([MultiBufferOffset(13)..MultiBufferOffset(13)].clone())
        });
        editor.handle_input(":", window, cx);
    });
    cx_b.focus(&editor_b);

    executor.advance_clock(Duration::from_secs(1));
    executor.run_until_parked();
    editor_a.update(cx_a, |editor, cx| {
        assert_eq!(
            vec![after_client_edit.to_string()],
            extract_hint_labels(editor, cx),
        );
    });
    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(
            vec![after_client_edit.to_string()],
            extract_hint_labels(editor, cx),
        );
    });

    let after_host_edit = edits_made.fetch_add(1, atomic::Ordering::Release) + 1;
    editor_a.update_in(cx_a, |editor, window, cx| {
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges([MultiBufferOffset(13)..MultiBufferOffset(13)])
        });
        editor.handle_input("a change to increment both buffers' versions", window, cx);
    });
    cx_a.focus(&editor_a);

    executor.advance_clock(Duration::from_secs(1));
    executor.run_until_parked();
    editor_a.update(cx_a, |editor, cx| {
        assert_eq!(
            vec![after_host_edit.to_string()],
            extract_hint_labels(editor, cx),
        );
    });
    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(
            vec![after_host_edit.to_string()],
            extract_hint_labels(editor, cx),
        );
    });

    let after_special_edit_for_refresh = edits_made.fetch_add(1, atomic::Ordering::Release) + 1;
    fake_language_server
        .request::<lsp::request::InlayHintRefreshRequest>(())
        .await
        .into_response()
        .expect("inlay refresh request failed");

    executor.advance_clock(Duration::from_secs(1));
    executor.run_until_parked();
    editor_a.update(cx_a, |editor, cx| {
        assert_eq!(
            vec![after_special_edit_for_refresh.to_string()],
            extract_hint_labels(editor, cx),
            "Host should react to /refresh LSP request"
        );
    });
    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(
            vec![after_special_edit_for_refresh.to_string()],
            extract_hint_labels(editor, cx),
            "Guest should get a /refresh LSP request propagated by host"
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_inlay_hint_refresh_is_forwarded(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let executor = cx_a.executor();
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    cx_a.update(editor::init);
    cx_b.update(editor::init);

    cx_a.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.all_languages.defaults.inlay_hints =
                    Some(InlayHintSettingsContent {
                        show_value_hints: Some(true),
                        enabled: Some(false),
                        edit_debounce_ms: Some(0),
                        scroll_debounce_ms: Some(0),
                        show_type_hints: Some(false),
                        show_parameter_hints: Some(false),
                        show_other_hints: Some(false),
                        show_background: Some(false),
                        toggle_on_modifiers_press: None,
                    })
            });
        });
    });
    cx_b.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.project.all_languages.defaults.inlay_hints =
                    Some(InlayHintSettingsContent {
                        show_value_hints: Some(true),
                        enabled: Some(true),
                        edit_debounce_ms: Some(0),
                        scroll_debounce_ms: Some(0),
                        show_type_hints: Some(true),
                        show_parameter_hints: Some(true),
                        show_other_hints: Some(true),
                        show_background: Some(false),
                        toggle_on_modifiers_press: None,
                    })
            });
        });
    });

    let capabilities = lsp::ServerCapabilities {
        inlay_hint_provider: Some(lsp::OneOf::Left(true)),
        ..lsp::ServerCapabilities::default()
    };
    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: capabilities.clone(),
            ..FakeLspAdapter::default()
        },
    );
    client_b.language_registry().add(rust_lang());
    client_b.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            capabilities,
            ..FakeLspAdapter::default()
        },
    );

    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() { a } // and some long comment to ensure inlay hints are not trimmed out",
                "other.rs": "// Test file",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    cx_a.background_executor.start_waiting();

    let editor_a = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("main.rs")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let editor_b = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("main.rs")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let other_hints = Arc::new(AtomicBool::new(false));
    let fake_language_server = fake_language_servers.next().await.unwrap();
    let closure_other_hints = Arc::clone(&other_hints);
    fake_language_server
        .set_request_handler::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
            let task_other_hints = Arc::clone(&closure_other_hints);
            async move {
                assert_eq!(
                    params.text_document.uri,
                    lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
                );
                let other_hints = task_other_hints.load(atomic::Ordering::Acquire);
                let character = if other_hints { 0 } else { 2 };
                let label = if other_hints {
                    "other hint"
                } else {
                    "initial hint"
                };
                Ok(Some(vec![
                    lsp::InlayHint {
                        position: lsp::Position::new(0, character),
                        label: lsp::InlayHintLabel::String(label.to_string()),
                        kind: None,
                        text_edits: None,
                        tooltip: None,
                        padding_left: None,
                        padding_right: None,
                        data: None,
                    },
                    lsp::InlayHint {
                        position: lsp::Position::new(1090, 1090),
                        label: lsp::InlayHintLabel::String("out-of-bounds hint".to_string()),
                        kind: None,
                        text_edits: None,
                        tooltip: None,
                        padding_left: None,
                        padding_right: None,
                        data: None,
                    },
                ]))
            }
        })
        .next()
        .await
        .unwrap();
    executor.finish_waiting();

    executor.run_until_parked();
    editor_a.update(cx_a, |editor, cx| {
        assert!(
            extract_hint_labels(editor, cx).is_empty(),
            "Host should get no hints due to them turned off"
        );
    });

    executor.run_until_parked();
    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(
            vec!["initial hint".to_string()],
            extract_hint_labels(editor, cx),
            "Client should get its first hints when opens an editor"
        );
    });

    other_hints.fetch_or(true, atomic::Ordering::Release);
    fake_language_server
        .request::<lsp::request::InlayHintRefreshRequest>(())
        .await
        .into_response()
        .expect("inlay refresh request failed");
    executor.run_until_parked();
    editor_a.update(cx_a, |editor, cx| {
        assert!(
            extract_hint_labels(editor, cx).is_empty(),
            "Host should get no hints due to them turned off, even after the /refresh"
        );
    });

    executor.run_until_parked();
    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(
            vec!["other hint".to_string()],
            extract_hint_labels(editor, cx),
            "Guest should get a /refresh LSP request propagated by host despite host hints are off"
        );
    });
}

#[gpui::test(iterations = 10)]
async fn test_lsp_document_color(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let expected_color = Rgba {
        r: 0.33,
        g: 0.33,
        b: 0.33,
        a: 0.33,
    };
    let mut server = TestServer::start(cx_a.executor()).await;
    let executor = cx_a.executor();
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    cx_a.update(editor::init);
    cx_b.update(editor::init);

    cx_a.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.editor.lsp_document_colors = Some(DocumentColorsRenderMode::None);
            });
        });
    });
    cx_b.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.editor.lsp_document_colors = Some(DocumentColorsRenderMode::Inlay);
            });
        });
    });

    let capabilities = lsp::ServerCapabilities {
        color_provider: Some(lsp::ColorProviderCapability::Simple(true)),
        ..lsp::ServerCapabilities::default()
    };
    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: capabilities.clone(),
            ..FakeLspAdapter::default()
        },
    );
    client_b.language_registry().add(rust_lang());
    client_b.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            capabilities,
            ..FakeLspAdapter::default()
        },
    );

    // Client A opens a project.
    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() { a }",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Client B joins the project
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);

    // The host opens a rust file.
    let _buffer_a = project_a
        .update(cx_a, |project, cx| {
            project.open_local_buffer(path!("/a/main.rs"), cx)
        })
        .await
        .unwrap();
    let editor_a = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("main.rs")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let fake_language_server = fake_language_servers.next().await.unwrap();
    cx_a.run_until_parked();
    cx_b.run_until_parked();

    let requests_made = Arc::new(AtomicUsize::new(0));
    let closure_requests_made = Arc::clone(&requests_made);
    let mut color_request_handle = fake_language_server
        .set_request_handler::<lsp::request::DocumentColor, _, _>(move |params, _| {
            let requests_made = Arc::clone(&closure_requests_made);
            async move {
                assert_eq!(
                    params.text_document.uri,
                    lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
                );
                requests_made.fetch_add(1, atomic::Ordering::Release);
                Ok(vec![lsp::ColorInformation {
                    range: lsp::Range {
                        start: lsp::Position {
                            line: 0,
                            character: 0,
                        },
                        end: lsp::Position {
                            line: 0,
                            character: 1,
                        },
                    },
                    color: lsp::Color {
                        red: 0.33,
                        green: 0.33,
                        blue: 0.33,
                        alpha: 0.33,
                    },
                }])
            }
        });
    executor.run_until_parked();

    assert_eq!(
        0,
        requests_made.load(atomic::Ordering::Acquire),
        "Host did not enable document colors, hence should query for none"
    );
    editor_a.update(cx_a, |editor, cx| {
        assert_eq!(
            Vec::<Rgba>::new(),
            extract_color_inlays(editor, cx),
            "No query colors should result in no hints"
        );
    });

    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    let editor_b = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("main.rs")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    color_request_handle.next().await.unwrap();
    executor.advance_clock(FETCH_COLORS_DEBOUNCE_TIMEOUT);
    executor.run_until_parked();

    assert_eq!(
        1,
        requests_made.load(atomic::Ordering::Acquire),
        "The client opened the file and got its first colors back"
    );
    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(
            vec![expected_color],
            extract_color_inlays(editor, cx),
            "With document colors as inlays, color inlays should be pushed"
        );
    });

    editor_a.update_in(cx_a, |editor, window, cx| {
        editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
            s.select_ranges([MultiBufferOffset(13)..MultiBufferOffset(13)].clone())
        });
        editor.handle_input(":", window, cx);
    });
    color_request_handle.next().await.unwrap();
    executor.run_until_parked();
    assert_eq!(
        2,
        requests_made.load(atomic::Ordering::Acquire),
        "After the host edits his file, the client should request the colors again"
    );
    editor_a.update(cx_a, |editor, cx| {
        assert_eq!(
            Vec::<Rgba>::new(),
            extract_color_inlays(editor, cx),
            "Host has no colors still"
        );
    });
    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(vec![expected_color], extract_color_inlays(editor, cx),);
    });

    cx_b.update(|_, cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.editor.lsp_document_colors = Some(DocumentColorsRenderMode::Background);
            });
        });
    });
    executor.run_until_parked();
    assert_eq!(
        2,
        requests_made.load(atomic::Ordering::Acquire),
        "After the client have changed the colors settings, no extra queries should happen"
    );
    editor_a.update(cx_a, |editor, cx| {
        assert_eq!(
            Vec::<Rgba>::new(),
            extract_color_inlays(editor, cx),
            "Host is unaffected by the client's settings changes"
        );
    });
    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(
            Vec::<Rgba>::new(),
            extract_color_inlays(editor, cx),
            "Client should have no colors hints, as in the settings"
        );
    });

    cx_b.update(|_, cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.editor.lsp_document_colors = Some(DocumentColorsRenderMode::Inlay);
            });
        });
    });
    executor.run_until_parked();
    assert_eq!(
        2,
        requests_made.load(atomic::Ordering::Acquire),
        "After falling back to colors as inlays, no extra LSP queries are made"
    );
    editor_a.update(cx_a, |editor, cx| {
        assert_eq!(
            Vec::<Rgba>::new(),
            extract_color_inlays(editor, cx),
            "Host is unaffected by the client's settings changes, again"
        );
    });
    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(
            vec![expected_color],
            extract_color_inlays(editor, cx),
            "Client should have its color hints back"
        );
    });

    cx_a.update(|_, cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.editor.lsp_document_colors = Some(DocumentColorsRenderMode::Border);
            });
        });
    });
    color_request_handle.next().await.unwrap();
    executor.run_until_parked();
    assert_eq!(
        3,
        requests_made.load(atomic::Ordering::Acquire),
        "After the host enables document colors, another LSP query should be made"
    );
    editor_a.update(cx_a, |editor, cx| {
        assert_eq!(
            Vec::<Rgba>::new(),
            extract_color_inlays(editor, cx),
            "Host did not configure document colors as hints hence gets nothing"
        );
    });
    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(
            vec![expected_color],
            extract_color_inlays(editor, cx),
            "Client should be unaffected by the host's settings changes"
        );
    });
}

async fn test_lsp_pull_diagnostics(
    should_stream_workspace_diagnostic: bool,
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let executor = cx_a.executor();
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    cx_a.update(editor::init);
    cx_b.update(editor::init);

    let expected_push_diagnostic_main_message = "pushed main diagnostic";
    let expected_push_diagnostic_lib_message = "pushed lib diagnostic";
    let expected_pull_diagnostic_main_message = "pulled main diagnostic";
    let expected_pull_diagnostic_lib_message = "pulled lib diagnostic";
    let expected_workspace_pull_diagnostics_main_message = "pulled workspace main diagnostic";
    let expected_workspace_pull_diagnostics_lib_message = "pulled workspace lib diagnostic";

    let diagnostics_pulls_result_ids = Arc::new(Mutex::new(BTreeSet::<Option<String>>::new()));
    let workspace_diagnostics_pulls_result_ids = Arc::new(Mutex::new(BTreeSet::<String>::new()));
    let diagnostics_pulls_made = Arc::new(AtomicUsize::new(0));
    let closure_diagnostics_pulls_made = diagnostics_pulls_made.clone();
    let closure_diagnostics_pulls_result_ids = diagnostics_pulls_result_ids.clone();
    let workspace_diagnostics_pulls_made = Arc::new(AtomicUsize::new(0));
    let closure_workspace_diagnostics_pulls_made = workspace_diagnostics_pulls_made.clone();
    let closure_workspace_diagnostics_pulls_result_ids =
        workspace_diagnostics_pulls_result_ids.clone();
    let (workspace_diagnostic_cancel_tx, closure_workspace_diagnostic_cancel_rx) =
        smol::channel::bounded::<()>(1);
    let (closure_workspace_diagnostic_received_tx, workspace_diagnostic_received_rx) =
        smol::channel::bounded::<()>(1);

    let capabilities = lsp::ServerCapabilities {
        diagnostic_provider: Some(lsp::DiagnosticServerCapabilities::Options(
            lsp::DiagnosticOptions {
                identifier: Some("test-pulls".to_string()),
                inter_file_dependencies: true,
                workspace_diagnostics: true,
                work_done_progress_options: lsp::WorkDoneProgressOptions {
                    work_done_progress: None,
                },
            },
        )),
        ..lsp::ServerCapabilities::default()
    };
    client_a.language_registry().add(rust_lang());

    let pull_diagnostics_handle = Arc::new(parking_lot::Mutex::new(None));
    let workspace_diagnostics_pulls_handle = Arc::new(parking_lot::Mutex::new(None));

    let closure_pull_diagnostics_handle = pull_diagnostics_handle.clone();
    let closure_workspace_diagnostics_pulls_handle = workspace_diagnostics_pulls_handle.clone();
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: capabilities.clone(),
            initializer: Some(Box::new(move |fake_language_server| {
                let expected_workspace_diagnostic_token = lsp::ProgressToken::String(format!(
                    "workspace/diagnostic/{}/1",
                    fake_language_server.server.server_id()
                ));
                let closure_workspace_diagnostics_pulls_result_ids = closure_workspace_diagnostics_pulls_result_ids.clone();
                let diagnostics_pulls_made = closure_diagnostics_pulls_made.clone();
                let diagnostics_pulls_result_ids = closure_diagnostics_pulls_result_ids.clone();
                let closure_pull_diagnostics_handle = closure_pull_diagnostics_handle.clone();
                let closure_workspace_diagnostics_pulls_handle = closure_workspace_diagnostics_pulls_handle.clone();
                let closure_workspace_diagnostic_cancel_rx = closure_workspace_diagnostic_cancel_rx.clone();
                let closure_workspace_diagnostic_received_tx = closure_workspace_diagnostic_received_tx.clone();
                let pull_diagnostics_handle = fake_language_server
                    .set_request_handler::<lsp::request::DocumentDiagnosticRequest, _, _>(
                        move |params, _| {
                            let requests_made = diagnostics_pulls_made.clone();
                            let diagnostics_pulls_result_ids =
                                diagnostics_pulls_result_ids.clone();
                            async move {
                                let message = if lsp::Uri::from_file_path(path!("/a/main.rs"))
                                    .unwrap()
                                    == params.text_document.uri
                                {
                                    expected_pull_diagnostic_main_message.to_string()
                                } else if lsp::Uri::from_file_path(path!("/a/lib.rs")).unwrap()
                                    == params.text_document.uri
                                {
                                    expected_pull_diagnostic_lib_message.to_string()
                                } else {
                                    panic!("Unexpected document: {}", params.text_document.uri)
                                };
                                {
                                    diagnostics_pulls_result_ids
                                        .lock()
                                        .await
                                        .insert(params.previous_result_id);
                                }
                                let new_requests_count =
                                    requests_made.fetch_add(1, atomic::Ordering::Release) + 1;
                                Ok(lsp::DocumentDiagnosticReportResult::Report(
                                    lsp::DocumentDiagnosticReport::Full(
                                        lsp::RelatedFullDocumentDiagnosticReport {
                                            related_documents: None,
                                            full_document_diagnostic_report:
                                                lsp::FullDocumentDiagnosticReport {
                                                    result_id: Some(format!(
                                                        "pull-{new_requests_count}"
                                                    )),
                                                    items: vec![lsp::Diagnostic {
                                                        range: lsp::Range {
                                                            start: lsp::Position {
                                                                line: 0,
                                                                character: 0,
                                                            },
                                                            end: lsp::Position {
                                                                line: 0,
                                                                character: 2,
                                                            },
                                                        },
                                                        severity: Some(
                                                            lsp::DiagnosticSeverity::ERROR,
                                                        ),
                                                        message,
                                                        ..lsp::Diagnostic::default()
                                                    }],
                                                },
                                        },
                                    ),
                                ))
                            }
                        },
                    );
                let _ = closure_pull_diagnostics_handle.lock().insert(pull_diagnostics_handle);

                let closure_workspace_diagnostics_pulls_made = closure_workspace_diagnostics_pulls_made.clone();
                let workspace_diagnostics_pulls_handle = fake_language_server.set_request_handler::<lsp::request::WorkspaceDiagnosticRequest, _, _>(
                    move |params, _| {
                        let workspace_requests_made = closure_workspace_diagnostics_pulls_made.clone();
                        let workspace_diagnostics_pulls_result_ids =
                            closure_workspace_diagnostics_pulls_result_ids.clone();
                        let workspace_diagnostic_cancel_rx = closure_workspace_diagnostic_cancel_rx.clone();
                        let workspace_diagnostic_received_tx = closure_workspace_diagnostic_received_tx.clone();
                        let expected_workspace_diagnostic_token = expected_workspace_diagnostic_token.clone();
                        async move {
                            let workspace_request_count =
                                workspace_requests_made.fetch_add(1, atomic::Ordering::Release) + 1;
                            {
                                workspace_diagnostics_pulls_result_ids
                                    .lock()
                                    .await
                                    .extend(params.previous_result_ids.into_iter().map(|id| id.value));
                            }
                            if should_stream_workspace_diagnostic && !workspace_diagnostic_cancel_rx.is_closed()
                            {
                                assert_eq!(
                                    params.partial_result_params.partial_result_token,
                                    Some(expected_workspace_diagnostic_token)
                                );
                                workspace_diagnostic_received_tx.send(()).await.unwrap();
                                workspace_diagnostic_cancel_rx.recv().await.unwrap();
                                workspace_diagnostic_cancel_rx.close();
                                // https://microsoft.github.io/language-server-protocol/specifications/lsp/3.17/specification/#partialResults
                                // > The final response has to be empty in terms of result values.
                                return Ok(lsp::WorkspaceDiagnosticReportResult::Report(
                                    lsp::WorkspaceDiagnosticReport { items: Vec::new() },
                                ));
                            }
                            Ok(lsp::WorkspaceDiagnosticReportResult::Report(
                                lsp::WorkspaceDiagnosticReport {
                                    items: vec![
                                        lsp::WorkspaceDocumentDiagnosticReport::Full(
                                            lsp::WorkspaceFullDocumentDiagnosticReport {
                                                uri: lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
                                                version: None,
                                                full_document_diagnostic_report:
                                                    lsp::FullDocumentDiagnosticReport {
                                                        result_id: Some(format!(
                                                            "workspace_{workspace_request_count}"
                                                        )),
                                                        items: vec![lsp::Diagnostic {
                                                            range: lsp::Range {
                                                                start: lsp::Position {
                                                                    line: 0,
                                                                    character: 1,
                                                                },
                                                                end: lsp::Position {
                                                                    line: 0,
                                                                    character: 3,
                                                                },
                                                            },
                                                            severity: Some(lsp::DiagnosticSeverity::WARNING),
                                                            message:
                                                                expected_workspace_pull_diagnostics_main_message
                                                                    .to_string(),
                                                            ..lsp::Diagnostic::default()
                                                        }],
                                                    },
                                            },
                                        ),
                                        lsp::WorkspaceDocumentDiagnosticReport::Full(
                                            lsp::WorkspaceFullDocumentDiagnosticReport {
                                                uri: lsp::Uri::from_file_path(path!("/a/lib.rs")).unwrap(),
                                                version: None,
                                                full_document_diagnostic_report:
                                                    lsp::FullDocumentDiagnosticReport {
                                                        result_id: Some(format!(
                                                            "workspace_{workspace_request_count}"
                                                        )),
                                                        items: vec![lsp::Diagnostic {
                                                            range: lsp::Range {
                                                                start: lsp::Position {
                                                                    line: 0,
                                                                    character: 1,
                                                                },
                                                                end: lsp::Position {
                                                                    line: 0,
                                                                    character: 3,
                                                                },
                                                            },
                                                            severity: Some(lsp::DiagnosticSeverity::WARNING),
                                                            message:
                                                                expected_workspace_pull_diagnostics_lib_message
                                                                    .to_string(),
                                                            ..lsp::Diagnostic::default()
                                                        }],
                                                    },
                                            },
                                        ),
                                    ],
                                },
                            ))
                        }
                    });
                let _ = closure_workspace_diagnostics_pulls_handle.lock().insert(workspace_diagnostics_pulls_handle);
            })),
            ..FakeLspAdapter::default()
        },
    );

    client_b.language_registry().add(rust_lang());
    client_b.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            capabilities,
            ..FakeLspAdapter::default()
        },
    );

    // Client A opens a project.
    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() { a }",
                "lib.rs": "fn other() {}",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Client B joins the project
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    executor.start_waiting();

    // The host opens a rust file.
    let _buffer_a = project_a
        .update(cx_a, |project, cx| {
            project.open_local_buffer(path!("/a/main.rs"), cx)
        })
        .await
        .unwrap();
    let editor_a_main = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("main.rs")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let fake_language_server = fake_language_servers.next().await.unwrap();
    let expected_workspace_diagnostic_token = lsp::ProgressToken::String(format!(
        "workspace/diagnostic-{}-1",
        fake_language_server.server.server_id()
    ));
    cx_a.run_until_parked();
    cx_b.run_until_parked();
    let mut pull_diagnostics_handle = pull_diagnostics_handle.lock().take().unwrap();
    let mut workspace_diagnostics_pulls_handle =
        workspace_diagnostics_pulls_handle.lock().take().unwrap();

    if should_stream_workspace_diagnostic {
        workspace_diagnostic_received_rx.recv().await.unwrap();
    } else {
        workspace_diagnostics_pulls_handle.next().await.unwrap();
    }
    assert_eq!(
        1,
        workspace_diagnostics_pulls_made.load(atomic::Ordering::Acquire),
        "Workspace diagnostics should be pulled initially on a server startup"
    );
    pull_diagnostics_handle.next().await.unwrap();
    assert_eq!(
        1,
        diagnostics_pulls_made.load(atomic::Ordering::Acquire),
        "Host should query pull diagnostics when the editor is opened"
    );
    executor.run_until_parked();
    editor_a_main.update(cx_a, |editor, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let all_diagnostics = snapshot
            .diagnostics_in_range(MultiBufferOffset(0)..snapshot.len())
            .collect::<Vec<_>>();
        assert_eq!(
            all_diagnostics.len(),
            1,
            "Expected single diagnostic, but got: {all_diagnostics:?}"
        );
        let diagnostic = &all_diagnostics[0];
        let mut expected_messages = vec![expected_pull_diagnostic_main_message];
        if !should_stream_workspace_diagnostic {
            expected_messages.push(expected_workspace_pull_diagnostics_main_message);
        }
        assert!(
            expected_messages.contains(&diagnostic.diagnostic.message.as_str()),
            "Expected {expected_messages:?} on the host, but got: {}",
            diagnostic.diagnostic.message
        );
    });

    fake_language_server.notify::<lsp::notification::PublishDiagnostics>(
        lsp::PublishDiagnosticsParams {
            uri: lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
            diagnostics: vec![lsp::Diagnostic {
                range: lsp::Range {
                    start: lsp::Position {
                        line: 0,
                        character: 3,
                    },
                    end: lsp::Position {
                        line: 0,
                        character: 4,
                    },
                },
                severity: Some(lsp::DiagnosticSeverity::INFORMATION),
                message: expected_push_diagnostic_main_message.to_string(),
                ..lsp::Diagnostic::default()
            }],
            version: None,
        },
    );
    fake_language_server.notify::<lsp::notification::PublishDiagnostics>(
        lsp::PublishDiagnosticsParams {
            uri: lsp::Uri::from_file_path(path!("/a/lib.rs")).unwrap(),
            diagnostics: vec![lsp::Diagnostic {
                range: lsp::Range {
                    start: lsp::Position {
                        line: 0,
                        character: 3,
                    },
                    end: lsp::Position {
                        line: 0,
                        character: 4,
                    },
                },
                severity: Some(lsp::DiagnosticSeverity::INFORMATION),
                message: expected_push_diagnostic_lib_message.to_string(),
                ..lsp::Diagnostic::default()
            }],
            version: None,
        },
    );

    if should_stream_workspace_diagnostic {
        fake_language_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
            token: expected_workspace_diagnostic_token.clone(),
            value: lsp::ProgressParamsValue::WorkspaceDiagnostic(
                lsp::WorkspaceDiagnosticReportResult::Report(lsp::WorkspaceDiagnosticReport {
                    items: vec![
                        lsp::WorkspaceDocumentDiagnosticReport::Full(
                            lsp::WorkspaceFullDocumentDiagnosticReport {
                                uri: lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
                                version: None,
                                full_document_diagnostic_report:
                                    lsp::FullDocumentDiagnosticReport {
                                        result_id: Some(format!(
                                            "workspace_{}",
                                            workspace_diagnostics_pulls_made
                                                .fetch_add(1, atomic::Ordering::Release)
                                                + 1
                                        )),
                                        items: vec![lsp::Diagnostic {
                                            range: lsp::Range {
                                                start: lsp::Position {
                                                    line: 0,
                                                    character: 1,
                                                },
                                                end: lsp::Position {
                                                    line: 0,
                                                    character: 2,
                                                },
                                            },
                                            severity: Some(lsp::DiagnosticSeverity::ERROR),
                                            message:
                                                expected_workspace_pull_diagnostics_main_message
                                                    .to_string(),
                                            ..lsp::Diagnostic::default()
                                        }],
                                    },
                            },
                        ),
                        lsp::WorkspaceDocumentDiagnosticReport::Full(
                            lsp::WorkspaceFullDocumentDiagnosticReport {
                                uri: lsp::Uri::from_file_path(path!("/a/lib.rs")).unwrap(),
                                version: None,
                                full_document_diagnostic_report:
                                    lsp::FullDocumentDiagnosticReport {
                                        result_id: Some(format!(
                                            "workspace_{}",
                                            workspace_diagnostics_pulls_made
                                                .fetch_add(1, atomic::Ordering::Release)
                                                + 1
                                        )),
                                        items: Vec::new(),
                                    },
                            },
                        ),
                    ],
                }),
            ),
        });
    };

    let mut workspace_diagnostic_start_count =
        workspace_diagnostics_pulls_made.load(atomic::Ordering::Acquire);

    executor.run_until_parked();
    editor_a_main.update(cx_a, |editor, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let all_diagnostics = snapshot
            .diagnostics_in_range(MultiBufferOffset(0)..snapshot.len())
            .collect::<Vec<_>>();
        assert_eq!(
            all_diagnostics.len(),
            2,
            "Expected pull and push diagnostics, but got: {all_diagnostics:?}"
        );
        let expected_messages = [
            expected_workspace_pull_diagnostics_main_message,
            expected_pull_diagnostic_main_message,
            expected_push_diagnostic_main_message,
        ];
        for diagnostic in all_diagnostics {
            assert!(
                expected_messages.contains(&diagnostic.diagnostic.message.as_str()),
                "Expected push and pull messages on the host: {expected_messages:?}, but got: {}",
                diagnostic.diagnostic.message
            );
        }
    });

    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    let editor_b_main = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("main.rs")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    cx_b.run_until_parked();

    pull_diagnostics_handle.next().await.unwrap();
    assert_eq!(
        2,
        diagnostics_pulls_made.load(atomic::Ordering::Acquire),
        "Client should query pull diagnostics when its editor is opened"
    );
    executor.run_until_parked();
    assert_eq!(
        workspace_diagnostic_start_count,
        workspace_diagnostics_pulls_made.load(atomic::Ordering::Acquire),
        "Workspace diagnostics should not be changed as the remote client does not initialize the workspace diagnostics pull"
    );
    editor_b_main.update(cx_b, |editor, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let all_diagnostics = snapshot
            .diagnostics_in_range(MultiBufferOffset(0)..snapshot.len())
            .collect::<Vec<_>>();
        assert_eq!(
            all_diagnostics.len(),
            2,
            "Expected pull and push diagnostics, but got: {all_diagnostics:?}"
        );

        // Despite the workspace diagnostics not re-initialized for the remote client, we can still expect its message synced from the host.
        let expected_messages = [
            expected_workspace_pull_diagnostics_main_message,
            expected_pull_diagnostic_main_message,
            expected_push_diagnostic_main_message,
        ];
        for diagnostic in all_diagnostics {
            assert!(
                expected_messages.contains(&diagnostic.diagnostic.message.as_str()),
                "The client should get both push and pull messages: {expected_messages:?}, but got: {}",
                diagnostic.diagnostic.message
            );
        }
    });

    let editor_b_lib = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("lib.rs")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    pull_diagnostics_handle.next().await.unwrap();
    assert_eq!(
        3,
        diagnostics_pulls_made.load(atomic::Ordering::Acquire),
        "Client should query pull diagnostics when its another editor is opened"
    );
    executor.run_until_parked();
    assert_eq!(
        workspace_diagnostic_start_count,
        workspace_diagnostics_pulls_made.load(atomic::Ordering::Acquire),
        "The remote client still did not anything to trigger the workspace diagnostics pull"
    );
    editor_b_lib.update(cx_b, |editor, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let all_diagnostics = snapshot
            .diagnostics_in_range(MultiBufferOffset(0)..snapshot.len())
            .collect::<Vec<_>>();
        let expected_messages = [
            expected_pull_diagnostic_lib_message,
            expected_push_diagnostic_lib_message,
        ];
        assert_eq!(
            all_diagnostics.len(),
            2,
            "Expected pull and push diagnostics, but got: {all_diagnostics:?}"
        );
        for diagnostic in all_diagnostics {
            assert!(
                expected_messages.contains(&diagnostic.diagnostic.message.as_str()),
                "The client should get both push and pull messages: {expected_messages:?}, but got: {}",
                diagnostic.diagnostic.message
            );
        }
    });

    if should_stream_workspace_diagnostic {
        fake_language_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
            token: expected_workspace_diagnostic_token.clone(),
            value: lsp::ProgressParamsValue::WorkspaceDiagnostic(
                lsp::WorkspaceDiagnosticReportResult::Report(lsp::WorkspaceDiagnosticReport {
                    items: vec![lsp::WorkspaceDocumentDiagnosticReport::Full(
                        lsp::WorkspaceFullDocumentDiagnosticReport {
                            uri: lsp::Uri::from_file_path(path!("/a/lib.rs")).unwrap(),
                            version: None,
                            full_document_diagnostic_report: lsp::FullDocumentDiagnosticReport {
                                result_id: Some(format!(
                                    "workspace_{}",
                                    workspace_diagnostics_pulls_made
                                        .fetch_add(1, atomic::Ordering::Release)
                                        + 1
                                )),
                                items: vec![lsp::Diagnostic {
                                    range: lsp::Range {
                                        start: lsp::Position {
                                            line: 0,
                                            character: 1,
                                        },
                                        end: lsp::Position {
                                            line: 0,
                                            character: 2,
                                        },
                                    },
                                    severity: Some(lsp::DiagnosticSeverity::ERROR),
                                    message: expected_workspace_pull_diagnostics_lib_message
                                        .to_string(),
                                    ..lsp::Diagnostic::default()
                                }],
                            },
                        },
                    )],
                }),
            ),
        });
        workspace_diagnostic_start_count =
            workspace_diagnostics_pulls_made.load(atomic::Ordering::Acquire);
        workspace_diagnostic_cancel_tx.send(()).await.unwrap();
        workspace_diagnostics_pulls_handle.next().await.unwrap();
        executor.run_until_parked();
        editor_b_lib.update(cx_b, |editor, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let all_diagnostics = snapshot
                .diagnostics_in_range(MultiBufferOffset(0)..snapshot.len())
                .collect::<Vec<_>>();
            let expected_messages = [
                // Despite workspace diagnostics provided,
                // the currently open file's diagnostics should be preferred, as LSP suggests.
                expected_pull_diagnostic_lib_message,
                expected_push_diagnostic_lib_message,
            ];
            assert_eq!(
                all_diagnostics.len(),
                2,
                "Expected pull and push diagnostics, but got: {all_diagnostics:?}"
            );
            for diagnostic in all_diagnostics {
                assert!(
                    expected_messages.contains(&diagnostic.diagnostic.message.as_str()),
                    "The client should get both push and pull messages: {expected_messages:?}, but got: {}",
                    diagnostic.diagnostic.message
                );
            }
        });
    };

    {
        assert!(
            !diagnostics_pulls_result_ids.lock().await.is_empty(),
            "Initial diagnostics pulls should report None at least"
        );
        assert_eq!(
            0,
            workspace_diagnostics_pulls_result_ids
                .lock()
                .await
                .deref()
                .len(),
            "After the initial workspace request, opening files should not reuse any result ids"
        );
    }

    editor_b_lib.update_in(cx_b, |editor, window, cx| {
        editor.move_to_end(&MoveToEnd, window, cx);
        editor.handle_input(":", window, cx);
    });
    pull_diagnostics_handle.next().await.unwrap();
    assert_eq!(
        4,
        diagnostics_pulls_made.load(atomic::Ordering::Acquire),
        "Client lib.rs edits should trigger another diagnostics pull for a buffer"
    );
    workspace_diagnostics_pulls_handle.next().await.unwrap();
    assert_eq!(
        workspace_diagnostic_start_count + 1,
        workspace_diagnostics_pulls_made.load(atomic::Ordering::Acquire),
        "After client lib.rs edits, the workspace diagnostics request should follow"
    );
    executor.run_until_parked();

    editor_b_main.update_in(cx_b, |editor, window, cx| {
        editor.move_to_end(&MoveToEnd, window, cx);
        editor.handle_input(":", window, cx);
    });
    pull_diagnostics_handle.next().await.unwrap();
    pull_diagnostics_handle.next().await.unwrap();
    assert_eq!(
        6,
        diagnostics_pulls_made.load(atomic::Ordering::Acquire),
        "Client main.rs edits should trigger another diagnostics pull by both client and host as they share the buffer"
    );
    workspace_diagnostics_pulls_handle.next().await.unwrap();
    assert_eq!(
        workspace_diagnostic_start_count + 2,
        workspace_diagnostics_pulls_made.load(atomic::Ordering::Acquire),
        "After client main.rs edits, the workspace diagnostics pull should follow"
    );
    executor.run_until_parked();

    editor_a_main.update_in(cx_a, |editor, window, cx| {
        editor.move_to_end(&MoveToEnd, window, cx);
        editor.handle_input(":", window, cx);
    });
    pull_diagnostics_handle.next().await.unwrap();
    pull_diagnostics_handle.next().await.unwrap();
    assert_eq!(
        8,
        diagnostics_pulls_made.load(atomic::Ordering::Acquire),
        "Host main.rs edits should trigger another diagnostics pull by both client and host as they share the buffer"
    );
    workspace_diagnostics_pulls_handle.next().await.unwrap();
    assert_eq!(
        workspace_diagnostic_start_count + 3,
        workspace_diagnostics_pulls_made.load(atomic::Ordering::Acquire),
        "After host main.rs edits, the workspace diagnostics pull should follow"
    );
    executor.run_until_parked();
    let diagnostic_pulls_result_ids = diagnostics_pulls_result_ids.lock().await.len();
    let workspace_pulls_result_ids = workspace_diagnostics_pulls_result_ids.lock().await.len();
    {
        assert!(
            diagnostic_pulls_result_ids > 1,
            "Should have sent result ids when pulling diagnostics"
        );
        assert!(
            workspace_pulls_result_ids > 1,
            "Should have sent result ids when pulling workspace diagnostics"
        );
    }

    fake_language_server
        .request::<lsp::request::WorkspaceDiagnosticRefresh>(())
        .await
        .into_response()
        .expect("workspace diagnostics refresh request failed");
    assert_eq!(
        8,
        diagnostics_pulls_made.load(atomic::Ordering::Acquire),
        "No single file pulls should happen after the diagnostics refresh server request"
    );
    workspace_diagnostics_pulls_handle.next().await.unwrap();
    assert_eq!(
        workspace_diagnostic_start_count + 4,
        workspace_diagnostics_pulls_made.load(atomic::Ordering::Acquire),
        "Another workspace diagnostics pull should happen after the diagnostics refresh server request"
    );
    {
        assert_eq!(
            diagnostics_pulls_result_ids.lock().await.len(),
            diagnostic_pulls_result_ids,
            "Pulls should not happen hence no extra ids should appear"
        );
        assert!(
            workspace_diagnostics_pulls_result_ids.lock().await.len() > workspace_pulls_result_ids,
            "More workspace diagnostics should be pulled"
        );
    }
    editor_b_lib.update(cx_b, |editor, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let all_diagnostics = snapshot
            .diagnostics_in_range(MultiBufferOffset(0)..snapshot.len())
            .collect::<Vec<_>>();
        let expected_messages = [
            expected_workspace_pull_diagnostics_lib_message,
            expected_pull_diagnostic_lib_message,
            expected_push_diagnostic_lib_message,
        ];
        assert_eq!(all_diagnostics.len(), 2);
        for diagnostic in &all_diagnostics {
            assert!(
                expected_messages.contains(&diagnostic.diagnostic.message.as_str()),
                "Unexpected diagnostics: {all_diagnostics:?}"
            );
        }
    });
    editor_b_main.update(cx_b, |editor, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let all_diagnostics = snapshot
            .diagnostics_in_range(MultiBufferOffset(0)..snapshot.len())
            .collect::<Vec<_>>();
        assert_eq!(all_diagnostics.len(), 2);

        let expected_messages = [
            expected_workspace_pull_diagnostics_main_message,
            expected_pull_diagnostic_main_message,
            expected_push_diagnostic_main_message,
        ];
        for diagnostic in &all_diagnostics {
            assert!(
                expected_messages.contains(&diagnostic.diagnostic.message.as_str()),
                "Unexpected diagnostics: {all_diagnostics:?}"
            );
        }
    });
    editor_a_main.update(cx_a, |editor, cx| {
        let snapshot = editor.buffer().read(cx).snapshot(cx);
        let all_diagnostics = snapshot
            .diagnostics_in_range(MultiBufferOffset(0)..snapshot.len())
            .collect::<Vec<_>>();
        assert_eq!(all_diagnostics.len(), 2);
        let expected_messages = [
            expected_workspace_pull_diagnostics_main_message,
            expected_pull_diagnostic_main_message,
            expected_push_diagnostic_main_message,
        ];
        for diagnostic in &all_diagnostics {
            assert!(
                expected_messages.contains(&diagnostic.diagnostic.message.as_str()),
                "Unexpected diagnostics: {all_diagnostics:?}"
            );
        }
    });
}

#[gpui::test(iterations = 10)]
async fn test_non_streamed_lsp_pull_diagnostics(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    test_lsp_pull_diagnostics(false, cx_a, cx_b).await;
}

#[gpui::test(iterations = 10)]
async fn test_streamed_lsp_pull_diagnostics(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    test_lsp_pull_diagnostics(true, cx_a, cx_b).await;
}

#[gpui::test(iterations = 10)]
async fn test_git_blame_is_forwarded(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    cx_a.update(editor::init);
    cx_b.update(editor::init);
    // Turn inline-blame-off by default so no state is transferred without us explicitly doing so
    let inline_blame_off_settings = Some(InlineBlameSettings {
        enabled: Some(false),
        ..Default::default()
    });
    cx_a.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.git.get_or_insert_default().inline_blame = inline_blame_off_settings;
            });
        });
    });
    cx_b.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings(cx, |settings| {
                settings.git.get_or_insert_default().inline_blame = inline_blame_off_settings;
            });
        });
    });

    client_a
        .fs()
        .insert_tree(
            path!("/my-repo"),
            json!({
                ".git": {},
                "file.txt": "line1\nline2\nline3\nline\n",
            }),
        )
        .await;

    let blame = git::blame::Blame {
        entries: vec![
            blame_entry("1b1b1b", 0..1),
            blame_entry("0d0d0d", 1..2),
            blame_entry("3a3a3a", 2..3),
            blame_entry("4c4c4c", 3..4),
        ],
        messages: [
            ("1b1b1b", "message for idx-0"),
            ("0d0d0d", "message for idx-1"),
            ("3a3a3a", "message for idx-2"),
            ("4c4c4c", "message for idx-3"),
        ]
        .into_iter()
        .map(|(sha, message)| (sha.parse().unwrap(), message.into()))
        .collect(),
    };
    client_a.fs().set_blame_for_repo(
        Path::new(path!("/my-repo/.git")),
        vec![(repo_path("file.txt"), blame)],
    );

    let (project_a, worktree_id) = client_a.build_local_project(path!("/my-repo"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Create editor_a
    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let editor_a = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("file.txt")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Join the project as client B.
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    let editor_b = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("file.txt")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    let buffer_id_b = editor_b.update(cx_b, |editor_b, cx| {
        editor_b
            .buffer()
            .read(cx)
            .as_singleton()
            .unwrap()
            .read(cx)
            .remote_id()
    });

    // client_b now requests git blame for the open buffer
    editor_b.update_in(cx_b, |editor_b, window, cx| {
        assert!(editor_b.blame().is_none());
        editor_b.toggle_git_blame(&git::Blame {}, window, cx);
    });

    cx_a.executor().run_until_parked();
    cx_b.executor().run_until_parked();

    editor_b.update(cx_b, |editor_b, cx| {
        let blame = editor_b.blame().expect("editor_b should have blame now");
        let entries = blame.update(cx, |blame, cx| {
            blame
                .blame_for_rows(
                    &(0..4)
                        .map(|row| RowInfo {
                            buffer_row: Some(row),
                            buffer_id: Some(buffer_id_b),
                            ..Default::default()
                        })
                        .collect::<Vec<_>>(),
                    cx,
                )
                .collect::<Vec<_>>()
        });

        assert_eq!(
            entries,
            vec![
                Some((buffer_id_b, blame_entry("1b1b1b", 0..1))),
                Some((buffer_id_b, blame_entry("0d0d0d", 1..2))),
                Some((buffer_id_b, blame_entry("3a3a3a", 2..3))),
                Some((buffer_id_b, blame_entry("4c4c4c", 3..4))),
            ]
        );

        blame.update(cx, |blame, _| {
            for (idx, (buffer, entry)) in entries.iter().flatten().enumerate() {
                let details = blame.details_for_entry(*buffer, entry).unwrap();
                assert_eq!(details.message, format!("message for idx-{}", idx));
            }
        });
    });

    // editor_b updates the file, which gets sent to client_a, which updates git blame,
    // which gets back to client_b.
    editor_b.update_in(cx_b, |editor_b, _, cx| {
        editor_b.edit([(Point::new(0, 3)..Point::new(0, 3), "FOO")], cx);
    });

    cx_a.executor().run_until_parked();
    cx_b.executor().run_until_parked();

    editor_b.update(cx_b, |editor_b, cx| {
        let blame = editor_b.blame().expect("editor_b should have blame now");
        let entries = blame.update(cx, |blame, cx| {
            blame
                .blame_for_rows(
                    &(0..4)
                        .map(|row| RowInfo {
                            buffer_row: Some(row),
                            buffer_id: Some(buffer_id_b),
                            ..Default::default()
                        })
                        .collect::<Vec<_>>(),
                    cx,
                )
                .collect::<Vec<_>>()
        });

        assert_eq!(
            entries,
            vec![
                None,
                Some((buffer_id_b, blame_entry("0d0d0d", 1..2))),
                Some((buffer_id_b, blame_entry("3a3a3a", 2..3))),
                Some((buffer_id_b, blame_entry("4c4c4c", 3..4))),
            ]
        );
    });

    // Now editor_a also updates the file
    editor_a.update_in(cx_a, |editor_a, _, cx| {
        editor_a.edit([(Point::new(1, 3)..Point::new(1, 3), "FOO")], cx);
    });

    cx_a.executor().run_until_parked();
    cx_b.executor().run_until_parked();

    editor_b.update(cx_b, |editor_b, cx| {
        let blame = editor_b.blame().expect("editor_b should have blame now");
        let entries = blame.update(cx, |blame, cx| {
            blame
                .blame_for_rows(
                    &(0..4)
                        .map(|row| RowInfo {
                            buffer_row: Some(row),
                            buffer_id: Some(buffer_id_b),
                            ..Default::default()
                        })
                        .collect::<Vec<_>>(),
                    cx,
                )
                .collect::<Vec<_>>()
        });

        assert_eq!(
            entries,
            vec![
                None,
                None,
                Some((buffer_id_b, blame_entry("3a3a3a", 2..3))),
                Some((buffer_id_b, blame_entry("4c4c4c", 3..4))),
            ]
        );
    });
}

#[gpui::test(iterations = 30)]
async fn test_collaborating_with_editorconfig(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);

    cx_b.update(editor::init);

    // Set up a fake language server.
    client_a.language_registry().add(rust_lang());
    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "src": {
                    "main.rs": "mod other;\nfn main() { let foo = other::foo(); }",
                    "other_mod": {
                        "other.rs": "pub fn foo() -> usize {\n    4\n}",
                        ".editorconfig": "",
                    },
                },
                ".editorconfig": "[*]\ntab_width = 2\n",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let main_buffer_a = project_a
        .update(cx_a, |p, cx| {
            p.open_buffer((worktree_id, rel_path("src/main.rs")), cx)
        })
        .await
        .unwrap();
    let other_buffer_a = project_a
        .update(cx_a, |p, cx| {
            p.open_buffer((worktree_id, rel_path("src/other_mod/other.rs")), cx)
        })
        .await
        .unwrap();
    let cx_a = cx_a.add_empty_window();
    let main_editor_a = cx_a.new_window_entity(|window, cx| {
        Editor::for_buffer(main_buffer_a, Some(project_a.clone()), window, cx)
    });
    let other_editor_a = cx_a.new_window_entity(|window, cx| {
        Editor::for_buffer(other_buffer_a, Some(project_a), window, cx)
    });
    let mut main_editor_cx_a = EditorTestContext {
        cx: cx_a.clone(),
        window: cx_a.window_handle(),
        editor: main_editor_a,
        assertion_cx: AssertionContextManager::new(),
    };
    let mut other_editor_cx_a = EditorTestContext {
        cx: cx_a.clone(),
        window: cx_a.window_handle(),
        editor: other_editor_a,
        assertion_cx: AssertionContextManager::new(),
    };

    // Join the project as client B.
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    let main_buffer_b = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer((worktree_id, rel_path("src/main.rs")), cx)
        })
        .await
        .unwrap();
    let other_buffer_b = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer((worktree_id, rel_path("src/other_mod/other.rs")), cx)
        })
        .await
        .unwrap();
    let cx_b = cx_b.add_empty_window();
    let main_editor_b = cx_b.new_window_entity(|window, cx| {
        Editor::for_buffer(main_buffer_b, Some(project_b.clone()), window, cx)
    });
    let other_editor_b = cx_b.new_window_entity(|window, cx| {
        Editor::for_buffer(other_buffer_b, Some(project_b.clone()), window, cx)
    });
    let mut main_editor_cx_b = EditorTestContext {
        cx: cx_b.clone(),
        window: cx_b.window_handle(),
        editor: main_editor_b,
        assertion_cx: AssertionContextManager::new(),
    };
    let mut other_editor_cx_b = EditorTestContext {
        cx: cx_b.clone(),
        window: cx_b.window_handle(),
        editor: other_editor_b,
        assertion_cx: AssertionContextManager::new(),
    };

    let initial_main = indoc! {"
ˇmod other;
fn main() { let foo = other::foo(); }"};
    let initial_other = indoc! {"
ˇpub fn foo() -> usize {
    4
}"};

    let first_tabbed_main = indoc! {"
  ˇmod other;
fn main() { let foo = other::foo(); }"};
    tab_undo_assert(
        &mut main_editor_cx_a,
        &mut main_editor_cx_b,
        initial_main,
        first_tabbed_main,
        true,
    );
    tab_undo_assert(
        &mut main_editor_cx_a,
        &mut main_editor_cx_b,
        initial_main,
        first_tabbed_main,
        false,
    );

    let first_tabbed_other = indoc! {"
  ˇpub fn foo() -> usize {
    4
}"};
    tab_undo_assert(
        &mut other_editor_cx_a,
        &mut other_editor_cx_b,
        initial_other,
        first_tabbed_other,
        true,
    );
    tab_undo_assert(
        &mut other_editor_cx_a,
        &mut other_editor_cx_b,
        initial_other,
        first_tabbed_other,
        false,
    );

    client_a
        .fs()
        .atomic_write(
            PathBuf::from(path!("/a/src/.editorconfig")),
            "[*]\ntab_width = 3\n".to_owned(),
        )
        .await
        .unwrap();
    cx_a.run_until_parked();
    cx_b.run_until_parked();

    let second_tabbed_main = indoc! {"
   ˇmod other;
fn main() { let foo = other::foo(); }"};
    tab_undo_assert(
        &mut main_editor_cx_a,
        &mut main_editor_cx_b,
        initial_main,
        second_tabbed_main,
        true,
    );
    tab_undo_assert(
        &mut main_editor_cx_a,
        &mut main_editor_cx_b,
        initial_main,
        second_tabbed_main,
        false,
    );

    let second_tabbed_other = indoc! {"
   ˇpub fn foo() -> usize {
    4
}"};
    tab_undo_assert(
        &mut other_editor_cx_a,
        &mut other_editor_cx_b,
        initial_other,
        second_tabbed_other,
        true,
    );
    tab_undo_assert(
        &mut other_editor_cx_a,
        &mut other_editor_cx_b,
        initial_other,
        second_tabbed_other,
        false,
    );

    let editorconfig_buffer_b = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer((worktree_id, rel_path("src/other_mod/.editorconfig")), cx)
        })
        .await
        .unwrap();
    editorconfig_buffer_b.update(cx_b, |buffer, cx| {
        buffer.set_text("[*.rs]\ntab_width = 6\n", cx);
    });
    project_b
        .update(cx_b, |project, cx| {
            project.save_buffer(editorconfig_buffer_b.clone(), cx)
        })
        .await
        .unwrap();
    cx_a.run_until_parked();
    cx_b.run_until_parked();

    tab_undo_assert(
        &mut main_editor_cx_a,
        &mut main_editor_cx_b,
        initial_main,
        second_tabbed_main,
        true,
    );
    tab_undo_assert(
        &mut main_editor_cx_a,
        &mut main_editor_cx_b,
        initial_main,
        second_tabbed_main,
        false,
    );

    let third_tabbed_other = indoc! {"
      ˇpub fn foo() -> usize {
    4
}"};
    tab_undo_assert(
        &mut other_editor_cx_a,
        &mut other_editor_cx_b,
        initial_other,
        third_tabbed_other,
        true,
    );

    tab_undo_assert(
        &mut other_editor_cx_a,
        &mut other_editor_cx_b,
        initial_other,
        third_tabbed_other,
        false,
    );
}

#[gpui::test]
async fn test_add_breakpoints(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let executor = cx_a.executor();
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);
    cx_a.update(editor::init);
    cx_b.update(editor::init);
    client_a
        .fs()
        .insert_tree(
            "/a",
            json!({
                "test.txt": "one\ntwo\nthree\nfour\nfive",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    let project_path = ProjectPath {
        worktree_id,
        path: rel_path(&"test.txt").into(),
    };
    let abs_path = project_a.read_with(cx_a, |project, cx| {
        project
            .absolute_path(&project_path, cx)
            .map(Arc::from)
            .unwrap()
    });

    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();
    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    // Client A opens an editor.
    let editor_a = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path(project_path.clone(), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Client B opens same editor as A.
    let editor_b = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path(project_path.clone(), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    // Client A adds breakpoint on line (1)
    editor_a.update_in(cx_a, |editor, window, cx| {
        editor.toggle_breakpoint(&editor::actions::ToggleBreakpoint, window, cx);
    });

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    let breakpoints_a = editor_a.update(cx_a, |editor, cx| {
        editor
            .breakpoint_store()
            .unwrap()
            .read(cx)
            .all_source_breakpoints(cx)
    });
    let breakpoints_b = editor_b.update(cx_b, |editor, cx| {
        editor
            .breakpoint_store()
            .unwrap()
            .read(cx)
            .all_source_breakpoints(cx)
    });

    assert_eq!(1, breakpoints_a.len());
    assert_eq!(1, breakpoints_a.get(&abs_path).unwrap().len());
    assert_eq!(breakpoints_a, breakpoints_b);

    // Client B adds breakpoint on line(2)
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.move_down(&editor::actions::MoveDown, window, cx);
        editor.move_down(&editor::actions::MoveDown, window, cx);
        editor.toggle_breakpoint(&editor::actions::ToggleBreakpoint, window, cx);
    });

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    let breakpoints_a = editor_a.update(cx_a, |editor, cx| {
        editor
            .breakpoint_store()
            .unwrap()
            .read(cx)
            .all_source_breakpoints(cx)
    });
    let breakpoints_b = editor_b.update(cx_b, |editor, cx| {
        editor
            .breakpoint_store()
            .unwrap()
            .read(cx)
            .all_source_breakpoints(cx)
    });

    assert_eq!(1, breakpoints_a.len());
    assert_eq!(breakpoints_a, breakpoints_b);
    assert_eq!(2, breakpoints_a.get(&abs_path).unwrap().len());

    // Client A removes last added breakpoint from client B
    editor_a.update_in(cx_a, |editor, window, cx| {
        editor.move_down(&editor::actions::MoveDown, window, cx);
        editor.move_down(&editor::actions::MoveDown, window, cx);
        editor.toggle_breakpoint(&editor::actions::ToggleBreakpoint, window, cx);
    });

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    let breakpoints_a = editor_a.update(cx_a, |editor, cx| {
        editor
            .breakpoint_store()
            .unwrap()
            .read(cx)
            .all_source_breakpoints(cx)
    });
    let breakpoints_b = editor_b.update(cx_b, |editor, cx| {
        editor
            .breakpoint_store()
            .unwrap()
            .read(cx)
            .all_source_breakpoints(cx)
    });

    assert_eq!(1, breakpoints_a.len());
    assert_eq!(breakpoints_a, breakpoints_b);
    assert_eq!(1, breakpoints_a.get(&abs_path).unwrap().len());

    // Client B removes first added breakpoint by client A
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.move_up(&editor::actions::MoveUp, window, cx);
        editor.move_up(&editor::actions::MoveUp, window, cx);
        editor.toggle_breakpoint(&editor::actions::ToggleBreakpoint, window, cx);
    });

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    let breakpoints_a = editor_a.update(cx_a, |editor, cx| {
        editor
            .breakpoint_store()
            .unwrap()
            .read(cx)
            .all_source_breakpoints(cx)
    });
    let breakpoints_b = editor_b.update(cx_b, |editor, cx| {
        editor
            .breakpoint_store()
            .unwrap()
            .read(cx)
            .all_source_breakpoints(cx)
    });

    assert_eq!(0, breakpoints_a.len());
    assert_eq!(breakpoints_a, breakpoints_b);
}

#[gpui::test]
async fn test_client_can_query_lsp_ext(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    cx_a.update(editor::init);
    cx_b.update(editor::init);

    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: "rust-analyzer",
            ..FakeLspAdapter::default()
        },
    );
    client_b.language_registry().add(rust_lang());
    client_b.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            name: "rust-analyzer",
            ..FakeLspAdapter::default()
        },
    );

    client_a
        .fs()
        .insert_tree(
            path!("/a"),
            json!({
                "main.rs": "fn main() {}",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project(path!("/a"), cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    let editor_a = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("main.rs")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let editor_b = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, rel_path("main.rs")), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let fake_language_server = fake_language_servers.next().await.unwrap();

    // host
    let mut expand_request_a = fake_language_server.set_request_handler::<LspExtExpandMacro, _, _>(
        |params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
            );
            assert_eq!(params.position, lsp::Position::new(0, 0));
            Ok(Some(ExpandedMacro {
                name: "test_macro_name".to_string(),
                expansion: "test_macro_expansion on the host".to_string(),
            }))
        },
    );

    editor_a.update_in(cx_a, |editor, window, cx| {
        expand_macro_recursively(editor, &ExpandMacroRecursively, window, cx)
    });
    expand_request_a.next().await.unwrap();
    cx_a.run_until_parked();

    workspace_a.update(cx_a, |workspace, cx| {
        workspace.active_pane().update(cx, |pane, cx| {
            assert_eq!(
                pane.items_len(),
                2,
                "Should have added a macro expansion to the host's pane"
            );
            let new_editor = pane.active_item().unwrap().downcast::<Editor>().unwrap();
            new_editor.update(cx, |editor, cx| {
                assert_eq!(editor.text(cx), "test_macro_expansion on the host");
            });
        })
    });

    // client
    let mut expand_request_b = fake_language_server.set_request_handler::<LspExtExpandMacro, _, _>(
        |params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Uri::from_file_path(path!("/a/main.rs")).unwrap(),
            );
            assert_eq!(
                params.position,
                lsp::Position::new(0, 12),
                "editor_b has selected the entire text and should query for a different position"
            );
            Ok(Some(ExpandedMacro {
                name: "test_macro_name".to_string(),
                expansion: "test_macro_expansion on the client".to_string(),
            }))
        },
    );

    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.select_all(&SelectAll, window, cx);
        expand_macro_recursively(editor, &ExpandMacroRecursively, window, cx)
    });
    expand_request_b.next().await.unwrap();
    cx_b.run_until_parked();

    workspace_b.update(cx_b, |workspace, cx| {
        workspace.active_pane().update(cx, |pane, cx| {
            assert_eq!(
                pane.items_len(),
                2,
                "Should have added a macro expansion to the client's pane"
            );
            let new_editor = pane.active_item().unwrap().downcast::<Editor>().unwrap();
            new_editor.update(cx, |editor, cx| {
                assert_eq!(editor.text(cx), "test_macro_expansion on the client");
            });
        })
    });
}

#[gpui::test]
async fn test_copy_file_name_without_extension(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    cx_b.update(editor::init);

    client_a
        .fs()
        .insert_tree(
            path!("/root"),
            json!({
                "src": {
                    "main.rs": indoc! {"
                        fn main() {
                            println!(\"Hello, world!\");
                        }
                    "},
                }
            }),
        )
        .await;

    let (project_a, worktree_id) = client_a.build_local_project(path!("/root"), cx_a).await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    let editor_a = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id, rel_path("src/main.rs")),
                None,
                true,
                window,
                cx,
            )
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let editor_b = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id, rel_path("src/main.rs")),
                None,
                true,
                window,
                cx,
            )
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    editor_a.update_in(cx_a, |editor, window, cx| {
        editor.copy_file_name_without_extension(&CopyFileNameWithoutExtension, window, cx);
    });

    assert_eq!(
        cx_a.read_from_clipboard().and_then(|item| item.text()),
        Some("main".to_string())
    );

    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.copy_file_name_without_extension(&CopyFileNameWithoutExtension, window, cx);
    });

    assert_eq!(
        cx_b.read_from_clipboard().and_then(|item| item.text()),
        Some("main".to_string())
    );
}

#[gpui::test]
async fn test_copy_file_name(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    cx_b.update(editor::init);

    client_a
        .fs()
        .insert_tree(
            path!("/root"),
            json!({
                "src": {
                    "main.rs": indoc! {"
                        fn main() {
                            println!(\"Hello, world!\");
                        }
                    "},
                }
            }),
        )
        .await;

    let (project_a, worktree_id) = client_a.build_local_project(path!("/root"), cx_a).await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    let editor_a = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id, rel_path("src/main.rs")),
                None,
                true,
                window,
                cx,
            )
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let editor_b = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id, rel_path("src/main.rs")),
                None,
                true,
                window,
                cx,
            )
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    editor_a.update_in(cx_a, |editor, window, cx| {
        editor.copy_file_name(&CopyFileName, window, cx);
    });

    assert_eq!(
        cx_a.read_from_clipboard().and_then(|item| item.text()),
        Some("main.rs".to_string())
    );

    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.copy_file_name(&CopyFileName, window, cx);
    });

    assert_eq!(
        cx_b.read_from_clipboard().and_then(|item| item.text()),
        Some("main.rs".to_string())
    );
}

#[gpui::test]
async fn test_copy_file_location(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let mut server = TestServer::start(cx_a.executor()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;

    cx_b.update(editor::init);

    client_a
        .fs()
        .insert_tree(
            path!("/root"),
            json!({
                "src": {
                    "main.rs": indoc! {"
                        fn main() {
                            println!(\"Hello, world!\");
                        }
                    "},
                }
            }),
        )
        .await;

    let (project_a, worktree_id) = client_a.build_local_project(path!("/root"), cx_a).await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.join_remote_project(project_id, cx_b).await;

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    let editor_a = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id, rel_path("src/main.rs")),
                None,
                true,
                window,
                cx,
            )
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let editor_b = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path(
                (worktree_id, rel_path("src/main.rs")),
                None,
                true,
                window,
                cx,
            )
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    editor_a.update_in(cx_a, |editor, window, cx| {
        editor.change_selections(Default::default(), window, cx, |s| {
            s.select_ranges([MultiBufferOffset(16)..MultiBufferOffset(16)]);
        });
        editor.copy_file_location(&CopyFileLocation, window, cx);
    });

    assert_eq!(
        cx_a.read_from_clipboard().and_then(|item| item.text()),
        Some(format!("{}:2", path!("src/main.rs")))
    );

    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(Default::default(), window, cx, |s| {
            s.select_ranges([MultiBufferOffset(16)..MultiBufferOffset(16)]);
        });
        editor.copy_file_location(&CopyFileLocation, window, cx);
    });

    assert_eq!(
        cx_b.read_from_clipboard().and_then(|item| item.text()),
        Some(format!("{}:2", path!("src/main.rs")))
    );
}

#[track_caller]
fn tab_undo_assert(
    cx_a: &mut EditorTestContext,
    cx_b: &mut EditorTestContext,
    expected_initial: &str,
    expected_tabbed: &str,
    a_tabs: bool,
) {
    cx_a.assert_editor_state(expected_initial);
    cx_b.assert_editor_state(expected_initial);

    if a_tabs {
        cx_a.update_editor(|editor, window, cx| {
            editor.tab(&editor::actions::Tab, window, cx);
        });
    } else {
        cx_b.update_editor(|editor, window, cx| {
            editor.tab(&editor::actions::Tab, window, cx);
        });
    }

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    cx_a.assert_editor_state(expected_tabbed);
    cx_b.assert_editor_state(expected_tabbed);

    if a_tabs {
        cx_a.update_editor(|editor, window, cx| {
            editor.undo(&editor::actions::Undo, window, cx);
        });
    } else {
        cx_b.update_editor(|editor, window, cx| {
            editor.undo(&editor::actions::Undo, window, cx);
        });
    }
    cx_a.run_until_parked();
    cx_b.run_until_parked();
    cx_a.assert_editor_state(expected_initial);
    cx_b.assert_editor_state(expected_initial);
}

fn extract_hint_labels(editor: &Editor, cx: &mut App) -> Vec<String> {
    let lsp_store = editor.project().unwrap().read(cx).lsp_store();

    let mut all_cached_labels = Vec::new();
    let mut all_fetched_hints = Vec::new();
    for buffer in editor.buffer().read(cx).all_buffers() {
        lsp_store.update(cx, |lsp_store, cx| {
            let hints = &lsp_store.latest_lsp_data(&buffer, cx).inlay_hints();
            all_cached_labels.extend(hints.all_cached_hints().into_iter().map(|hint| {
                let mut label = hint.text().to_string();
                if hint.padding_left {
                    label.insert(0, ' ');
                }
                if hint.padding_right {
                    label.push_str(" ");
                }
                label
            }));
            all_fetched_hints.extend(hints.all_fetched_hints());
        });
    }

    assert!(
        all_fetched_hints.is_empty(),
        "Did not expect background hints fetch tasks, but got {} of them",
        all_fetched_hints.len()
    );

    all_cached_labels
}

#[track_caller]
fn extract_color_inlays(editor: &Editor, cx: &App) -> Vec<Rgba> {
    editor
        .all_inlays(cx)
        .into_iter()
        .filter_map(|inlay| inlay.get_color())
        .map(Rgba::from)
        .collect()
}

fn blame_entry(sha: &str, range: Range<u32>) -> git::blame::BlameEntry {
    git::blame::BlameEntry {
        sha: sha.parse().unwrap(),
        range,
        ..Default::default()
    }
}
