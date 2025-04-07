use crate::{
    rpc::RECONNECT_TIMEOUT,
    tests::{TestServer, rust_lang},
};
use call::ActiveCall;
use editor::{
    Editor, RowInfo,
    actions::{
        ConfirmCodeAction, ConfirmCompletion, ConfirmRename, ContextMenuFirst,
        ExpandMacroRecursively, Redo, Rename, ToggleCodeActions, Undo,
    },
    test::{
        editor_test_context::{AssertionContextManager, EditorTestContext},
        expand_macro_recursively,
    },
};
use fs::Fs;
use futures::StreamExt;
use gpui::{TestAppContext, UpdateGlobal, VisualContext, VisualTestContext};
use indoc::indoc;
use language::{
    FakeLspAdapter,
    language_settings::{AllLanguageSettings, InlayHintSettings},
};
use project::{
    ProjectPath, SERVER_PROGRESS_THROTTLE_TIMEOUT,
    lsp_store::{
        lsp_ext_command::{ExpandedMacro, LspExpandMacro},
        rust_analyzer_ext::RUST_ANALYZER_NAME,
    },
    project_settings::{InlineBlameSettings, ProjectSettings},
};
use recent_projects::disconnected_overlay::DisconnectedOverlay;
use rpc::RECEIVE_TIMEOUT;
use serde_json::json;
use settings::SettingsStore;
use std::{
    ops::Range,
    path::{Path, PathBuf},
    sync::{
        Arc,
        atomic::{self, AtomicBool, AtomicUsize},
    },
};
use text::Point;
use util::{path, uri};
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
            workspace.open_path((worktree_id, "b.txt"), None, true, window, cx)
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
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
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
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
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

#[gpui::test(iterations = 10)]
async fn test_collaborating_with_completion(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let mut server = TestServer::start(cx_a.executor()).await;
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
                completion_provider: Some(lsp::CompletionOptions {
                    trigger_characters: Some(vec![".".to_string()]),
                    resolve_provider: Some(true),
                    ..Default::default()
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
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
        .await
        .unwrap();
    let cx_b = cx_b.add_empty_window();
    let editor_b = cx_b.new_window_entity(|window, cx| {
        Editor::for_buffer(buffer_b.clone(), Some(project_b.clone()), window, cx)
    });

    let fake_language_server = fake_language_servers.next().await.unwrap();
    cx_a.background_executor.run_until_parked();

    buffer_b.read_with(cx_b, |buffer, _| {
        assert!(!buffer.completion_triggers().is_empty())
    });

    // Type a completion trigger character as the guest.
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| s.select_ranges([13..13]));
        editor.handle_input(".", window, cx);
    });
    cx_b.focus(&editor_b);

    // Receive a completion request as the host's language server.
    // Return some completions from the host's language server.
    cx_a.executor().start_waiting();
    fake_language_server
        .set_request_handler::<lsp::request::Completion, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri,
                lsp::Url::from_file_path(path!("/a/main.rs")).unwrap(),
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
        })
        .next()
        .await
        .unwrap();
    cx_a.executor().finish_waiting();

    // Open the buffer on the host.
    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
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
        editor.change_selections(None, window, cx, |s| s.select_ranges([46..46]));
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
                lsp::Url::from_file_path(path!("/a/main.rs")).unwrap(),
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
                        // no snippet placehodlers
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

    // Set up a fake language server.
    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a
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
            workspace.open_path((worktree_id, "main.rs"), None, true, window, cx)
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
                lsp::Url::from_file_path(path!("/a/main.rs")).unwrap(),
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
        editor.change_selections(None, window, cx, |s| {
            s.select_ranges([Point::new(1, 31)..Point::new(1, 31)])
        });
    });
    cx_b.focus(&editor_b);

    let mut requests = fake_language_server
        .set_request_handler::<lsp::request::CodeActionRequest, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path(path!("/a/main.rs")).unwrap(),
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
                                    lsp::Url::from_file_path(path!("/a/main.rs")).unwrap(),
                                    vec![lsp::TextEdit::new(
                                        lsp::Range::new(
                                            lsp::Position::new(1, 22),
                                            lsp::Position::new(1, 34),
                                        ),
                                        "4".to_string(),
                                    )],
                                ),
                                (
                                    lsp::Url::from_file_path(path!("/a/other.rs")).unwrap(),
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
                deployed_from_indicator: None,
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
                                lsp::Url::from_file_path(path!("/a/main.rs")).unwrap(),
                                vec![lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(1, 22),
                                        lsp::Position::new(1, 34),
                                    ),
                                    "4".to_string(),
                                )],
                            ),
                            (
                                lsp::Url::from_file_path(path!("/a/other.rs")).unwrap(),
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

    // Set up a fake language server.
    client_a.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                rename_provider: Some(lsp::OneOf::Right(lsp::RenameOptions {
                    prepare_provider: Some(true),
                    work_done_progress_options: Default::default(),
                })),
                ..Default::default()
            },
            ..Default::default()
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
            workspace.open_path((worktree_id, "one.rs"), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    let fake_language_server = fake_language_servers.next().await.unwrap();

    // Move cursor to a location that can be renamed.
    let prepare_rename = editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| s.select_ranges([7..7]));
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
            6..9
        );
        rename.editor.update(cx, |rename_editor, cx| {
            let rename_selection = rename_editor.selections.newest::<usize>(cx);
            assert_eq!(
                rename_selection.range(),
                0..3,
                "Rename that was triggered from zero selection caret, should propose the whole word."
            );
            rename_editor.buffer().update(cx, |rename_buffer, cx| {
                rename_buffer.edit([(0..3, "THREE")], None, cx);
            });
        });
    });

    // Cancel the rename, and repeat the same, but use selections instead of cursor movement
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.cancel(&editor::actions::Cancel, window, cx);
    });
    let prepare_rename = editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| s.select_ranges([7..8]));
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
        assert_eq!(lsp_rename_start..lsp_rename_end, 6..9);
        rename.editor.update(cx, |rename_editor, cx| {
            let rename_selection = rename_editor.selections.newest::<usize>(cx);
            assert_eq!(
                rename_selection.range(),
                1..2,
                "Rename that was triggered from a selection, should have the same selection range in the rename proposal"
            );
            rename_editor.buffer().update(cx, |rename_buffer, cx| {
                rename_buffer.edit([(0..lsp_rename_end - lsp_rename_start, "THREE")], None, cx);
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
                            lsp::Url::from_file_path(path!("/dir/one.rs")).unwrap(),
                            vec![lsp::TextEdit::new(
                                lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
                                "THREE".to_string(),
                            )],
                        ),
                        (
                            lsp::Url::from_file_path(path!("/dir/two.rs")).unwrap(),
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
    fake_language_server.notify::<lsp::notification::Progress>(&lsp::ProgressParams {
        token: lsp::NumberOrString::String("the-token".to_string()),
        value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::Report(
            lsp::WorkDoneProgressReport {
                message: Some("the-message".to_string()),
                ..Default::default()
            },
        )),
    });
    executor.run_until_parked();

    project_a.read_with(cx_a, |project, cx| {
        let status = project.language_server_statuses(cx).next().unwrap().1;
        assert_eq!(status.name, "the-language-server");
        assert_eq!(status.pending_work.len(), 1);
        assert_eq!(
            status.pending_work["the-token"].message.as_ref().unwrap(),
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
        assert_eq!(status.name, "the-language-server");
    });

    executor.advance_clock(SERVER_PROGRESS_THROTTLE_TIMEOUT);
    fake_language_server.notify::<lsp::notification::Progress>(&lsp::ProgressParams {
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
        assert_eq!(status.name, "the-language-server");
        assert_eq!(status.pending_work.len(), 1);
        assert_eq!(
            status.pending_work["the-token"].message.as_ref().unwrap(),
            "the-message-2"
        );
    });

    project_b.read_with(cx_b, |project, cx| {
        let status = project.language_server_statuses(cx).next().unwrap().1;
        assert_eq!(status.name, "the-language-server");
        assert_eq!(status.pending_work.len(), 1);
        assert_eq!(
            status.pending_work["the-token"].message.as_ref().unwrap(),
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
            worktree.paths().map(AsRef::as_ref).collect::<Vec<_>>(),
            [
                Path::new(".gitignore"),
                Path::new("a.txt"),
                Path::new("b.txt"),
                Path::new("ignored-dir"),
            ]
        );
    });

    project_b
        .update(cx_b, |project, cx| {
            let worktree = project.worktrees(cx).next().unwrap();
            let entry = worktree.read(cx).entry_for_path("ignored-dir").unwrap();
            project.expand_entry(worktree_id, entry.id, cx).unwrap()
        })
        .await
        .unwrap();

    project_b.read_with(cx_b, |project, cx| {
        let worktree = project.worktrees(cx).next().unwrap().read(cx);
        assert_eq!(
            worktree.paths().map(AsRef::as_ref).collect::<Vec<_>>(),
            [
                Path::new(".gitignore"),
                Path::new("a.txt"),
                Path::new("b.txt"),
                Path::new("ignored-dir"),
                Path::new("ignored-dir/c.txt"),
                Path::new("ignored-dir/d.txt"),
            ]
        );
    });

    // Open the same file as client B and client A.
    let buffer_b = project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "b.txt"), cx))
        .await
        .unwrap();

    buffer_b.read_with(cx_b, |buf, _| assert_eq!(buf.text(), "b-contents"));

    project_a.read_with(cx_a, |project, cx| {
        assert!(project.has_open_buffer((worktree_id, "b.txt"), cx))
    });
    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "b.txt"), cx))
        .await
        .unwrap();

    let editor_b =
        cx_b.new_window_entity(|window, cx| Editor::for_buffer(buffer_b, None, window, cx));

    // Client A sees client B's selection
    executor.run_until_parked();

    buffer_a.read_with(cx_a, |buffer, _| {
        buffer
            .snapshot()
            .selections_in_range(text::Anchor::MIN..text::Anchor::MAX, false)
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
            .selections_in_range(text::Anchor::MIN..text::Anchor::MAX, false)
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
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
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
                lsp::Url::from_file_path(path!("/a/main.rs")).unwrap(),
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
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
        .await
        .unwrap();

    // Type a on type formatting trigger character as the guest.
    cx_a.focus(&editor_a);
    editor_a.update_in(cx_a, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| s.select_ranges([13..13]));
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

    // Open a file in an editor as the guest.
    let buffer_b = project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
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
        editor.change_selections(None, window, cx, |s| s.select_ranges([13..13]));
        editor.handle_input(":", window, cx);
    });

    // Receive an OnTypeFormatting request as the host's language server.
    // Return some formatting from the host's language server.
    executor.start_waiting();
    fake_language_server
        .set_request_handler::<lsp::request::OnTypeFormatting, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri,
                lsp::Url::from_file_path(path!("/a/main.rs")).unwrap(),
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
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
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
            store.update_user_settings::<AllLanguageSettings>(cx, |settings| {
                settings.defaults.inlay_hints = Some(InlayHintSettings {
                    enabled: true,
                    edit_debounce_ms: 0,
                    scroll_debounce_ms: 0,
                    show_type_hints: true,
                    show_parameter_hints: false,
                    show_other_hints: true,
                    show_background: false,
                    toggle_on_modifiers_press: None,
                })
            });
        });
    });
    cx_b.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, |settings| {
                settings.defaults.inlay_hints = Some(InlayHintSettings {
                    enabled: true,
                    edit_debounce_ms: 0,
                    scroll_debounce_ms: 0,
                    show_type_hints: true,
                    show_parameter_hints: false,
                    show_other_hints: true,
                    show_background: false,
                    toggle_on_modifiers_press: None,
                })
            });
        });
    });

    client_a.language_registry().add(rust_lang());
    client_b.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                inlay_hint_provider: Some(lsp::OneOf::Left(true)),
                ..Default::default()
            },
            ..Default::default()
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
    executor.start_waiting();

    // The host opens a rust file.
    let _buffer_a = project_a
        .update(cx_a, |project, cx| {
            project.open_local_buffer(path!("/a/main.rs"), cx)
        })
        .await
        .unwrap();
    let editor_a = workspace_a
        .update_in(cx_a, |workspace, window, cx| {
            workspace.open_path((worktree_id, "main.rs"), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let fake_language_server = fake_language_servers.next().await.unwrap();

    // Set up the language server to return an additional inlay hint on each request.
    let edits_made = Arc::new(AtomicUsize::new(0));
    let closure_edits_made = Arc::clone(&edits_made);
    fake_language_server
        .set_request_handler::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
            let task_edits_made = Arc::clone(&closure_edits_made);
            async move {
                assert_eq!(
                    params.text_document.uri,
                    lsp::Url::from_file_path(path!("/a/main.rs")).unwrap(),
                );
                let edits_made = task_edits_made.load(atomic::Ordering::Acquire);
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
        })
        .next()
        .await
        .unwrap();

    executor.run_until_parked();

    let initial_edit = edits_made.load(atomic::Ordering::Acquire);
    editor_a.update(cx_a, |editor, _| {
        assert_eq!(
            vec![initial_edit.to_string()],
            extract_hint_labels(editor),
            "Host should get its first hints when opens an editor"
        );
    });
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    let editor_b = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, "main.rs"), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    executor.run_until_parked();
    editor_b.update(cx_b, |editor, _| {
        assert_eq!(
            vec![initial_edit.to_string()],
            extract_hint_labels(editor),
            "Client should get its first hints when opens an editor"
        );
    });

    let after_client_edit = edits_made.fetch_add(1, atomic::Ordering::Release) + 1;
    editor_b.update_in(cx_b, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| s.select_ranges([13..13].clone()));
        editor.handle_input(":", window, cx);
    });
    cx_b.focus(&editor_b);

    executor.run_until_parked();
    editor_a.update(cx_a, |editor, _| {
        assert_eq!(
            vec![after_client_edit.to_string()],
            extract_hint_labels(editor),
        );
    });
    editor_b.update(cx_b, |editor, _| {
        assert_eq!(
            vec![after_client_edit.to_string()],
            extract_hint_labels(editor),
        );
    });

    let after_host_edit = edits_made.fetch_add(1, atomic::Ordering::Release) + 1;
    editor_a.update_in(cx_a, |editor, window, cx| {
        editor.change_selections(None, window, cx, |s| s.select_ranges([13..13]));
        editor.handle_input("a change to increment both buffers' versions", window, cx);
    });
    cx_a.focus(&editor_a);

    executor.run_until_parked();
    editor_a.update(cx_a, |editor, _| {
        assert_eq!(
            vec![after_host_edit.to_string()],
            extract_hint_labels(editor),
        );
    });
    editor_b.update(cx_b, |editor, _| {
        assert_eq!(
            vec![after_host_edit.to_string()],
            extract_hint_labels(editor),
        );
    });

    let after_special_edit_for_refresh = edits_made.fetch_add(1, atomic::Ordering::Release) + 1;
    fake_language_server
        .request::<lsp::request::InlayHintRefreshRequest>(())
        .await
        .expect("inlay refresh request failed");

    executor.run_until_parked();
    editor_a.update(cx_a, |editor, _| {
        assert_eq!(
            vec![after_special_edit_for_refresh.to_string()],
            extract_hint_labels(editor),
            "Host should react to /refresh LSP request"
        );
    });
    editor_b.update(cx_b, |editor, _| {
        assert_eq!(
            vec![after_special_edit_for_refresh.to_string()],
            extract_hint_labels(editor),
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
            store.update_user_settings::<AllLanguageSettings>(cx, |settings| {
                settings.defaults.inlay_hints = Some(InlayHintSettings {
                    enabled: false,
                    edit_debounce_ms: 0,
                    scroll_debounce_ms: 0,
                    show_type_hints: false,
                    show_parameter_hints: false,
                    show_other_hints: false,
                    show_background: false,
                    toggle_on_modifiers_press: None,
                })
            });
        });
    });
    cx_b.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings::<AllLanguageSettings>(cx, |settings| {
                settings.defaults.inlay_hints = Some(InlayHintSettings {
                    enabled: true,
                    edit_debounce_ms: 0,
                    scroll_debounce_ms: 0,
                    show_type_hints: true,
                    show_parameter_hints: true,
                    show_other_hints: true,
                    show_background: false,
                    toggle_on_modifiers_press: None,
                })
            });
        });
    });

    client_a.language_registry().add(rust_lang());
    client_b.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            capabilities: lsp::ServerCapabilities {
                inlay_hint_provider: Some(lsp::OneOf::Left(true)),
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
            workspace.open_path((worktree_id, "main.rs"), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let editor_b = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, "main.rs"), None, true, window, cx)
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
                    lsp::Url::from_file_path(path!("/a/main.rs")).unwrap(),
                );
                let other_hints = task_other_hints.load(atomic::Ordering::Acquire);
                let character = if other_hints { 0 } else { 2 };
                let label = if other_hints {
                    "other hint"
                } else {
                    "initial hint"
                };
                Ok(Some(vec![lsp::InlayHint {
                    position: lsp::Position::new(0, character),
                    label: lsp::InlayHintLabel::String(label.to_string()),
                    kind: None,
                    text_edits: None,
                    tooltip: None,
                    padding_left: None,
                    padding_right: None,
                    data: None,
                }]))
            }
        })
        .next()
        .await
        .unwrap();
    executor.finish_waiting();

    executor.run_until_parked();
    editor_a.update(cx_a, |editor, _| {
        assert!(
            extract_hint_labels(editor).is_empty(),
            "Host should get no hints due to them turned off"
        );
    });

    executor.run_until_parked();
    editor_b.update(cx_b, |editor, _| {
        assert_eq!(
            vec!["initial hint".to_string()],
            extract_hint_labels(editor),
            "Client should get its first hints when opens an editor"
        );
    });

    other_hints.fetch_or(true, atomic::Ordering::Release);
    fake_language_server
        .request::<lsp::request::InlayHintRefreshRequest>(())
        .await
        .expect("inlay refresh request failed");
    executor.run_until_parked();
    editor_a.update(cx_a, |editor, _| {
        assert!(
            extract_hint_labels(editor).is_empty(),
            "Host should get no hints due to them turned off, even after the /refresh"
        );
    });

    executor.run_until_parked();
    editor_b.update(cx_b, |editor, _| {
        assert_eq!(
            vec!["other hint".to_string()],
            extract_hint_labels(editor),
            "Guest should get a /refresh LSP request propagated by host despite host hints are off"
        );
    });
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
        enabled: false,
        delay_ms: None,
        min_column: None,
        show_commit_summary: false,
    });
    cx_a.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings::<ProjectSettings>(cx, |settings| {
                settings.git.inline_blame = inline_blame_off_settings;
            });
        });
    });
    cx_b.update(|cx| {
        SettingsStore::update_global(cx, |store, cx| {
            store.update_user_settings::<ProjectSettings>(cx, |settings| {
                settings.git.inline_blame = inline_blame_off_settings;
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
        remote_url: Some("git@github.com:zed-industries/zed.git".to_string()),
    };
    client_a.fs().set_blame_for_repo(
        Path::new(path!("/my-repo/.git")),
        vec![("file.txt".into(), blame)],
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
            workspace.open_path((worktree_id, "file.txt"), None, true, window, cx)
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
            workspace.open_path((worktree_id, "file.txt"), None, true, window, cx)
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
                Some(blame_entry("1b1b1b", 0..1)),
                Some(blame_entry("0d0d0d", 1..2)),
                Some(blame_entry("3a3a3a", 2..3)),
                Some(blame_entry("4c4c4c", 3..4)),
            ]
        );

        blame.update(cx, |blame, _| {
            for (idx, entry) in entries.iter().flatten().enumerate() {
                let details = blame.details_for_entry(entry).unwrap();
                assert_eq!(details.message, format!("message for idx-{}", idx));
                assert_eq!(
                    details.permalink.unwrap().to_string(),
                    format!("https://github.com/zed-industries/zed/commit/{}", entry.sha)
                );
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
                Some(blame_entry("0d0d0d", 1..2)),
                Some(blame_entry("3a3a3a", 2..3)),
                Some(blame_entry("4c4c4c", 3..4)),
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
                Some(blame_entry("3a3a3a", 2..3)),
                Some(blame_entry("4c4c4c", 3..4)),
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
            p.open_buffer((worktree_id, "src/main.rs"), cx)
        })
        .await
        .unwrap();
    let other_buffer_a = project_a
        .update(cx_a, |p, cx| {
            p.open_buffer((worktree_id, "src/other_mod/other.rs"), cx)
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
            p.open_buffer((worktree_id, "src/main.rs"), cx)
        })
        .await
        .unwrap();
    let other_buffer_b = project_b
        .update(cx_b, |p, cx| {
            p.open_buffer((worktree_id, "src/other_mod/other.rs"), cx)
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
            p.open_buffer((worktree_id, "src/other_mod/.editorconfig"), cx)
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
        path: Arc::from(Path::new(&"test.txt")),
    };
    let abs_path = project_a.read_with(cx_a, |project, cx| {
        project
            .absolute_path(&project_path, cx)
            .map(|path_buf| Arc::from(path_buf.to_owned()))
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
            .clone()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
    });
    let breakpoints_b = editor_b.update(cx_b, |editor, cx| {
        editor
            .breakpoint_store()
            .clone()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
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
            .clone()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
    });
    let breakpoints_b = editor_b.update(cx_b, |editor, cx| {
        editor
            .breakpoint_store()
            .clone()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
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
            .clone()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
    });
    let breakpoints_b = editor_b.update(cx_b, |editor, cx| {
        editor
            .breakpoint_store()
            .clone()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
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
            .clone()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
    });
    let breakpoints_b = editor_b.update(cx_b, |editor, cx| {
        editor
            .breakpoint_store()
            .clone()
            .unwrap()
            .read(cx)
            .all_breakpoints(cx)
            .clone()
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
    client_b.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp(
        "Rust",
        FakeLspAdapter {
            name: RUST_ANALYZER_NAME,
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
            workspace.open_path((worktree_id, "main.rs"), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let editor_b = workspace_b
        .update_in(cx_b, |workspace, window, cx| {
            workspace.open_path((worktree_id, "main.rs"), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let fake_language_server = fake_language_servers.next().await.unwrap();

    // host
    let mut expand_request_a =
        fake_language_server.set_request_handler::<LspExpandMacro, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path(path!("/a/main.rs")).unwrap(),
            );
            assert_eq!(params.position, lsp::Position::new(0, 0),);
            Ok(Some(ExpandedMacro {
                name: "test_macro_name".to_string(),
                expansion: "test_macro_expansion on the host".to_string(),
            }))
        });

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
    let mut expand_request_b =
        fake_language_server.set_request_handler::<LspExpandMacro, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path(path!("/a/main.rs")).unwrap(),
            );
            assert_eq!(params.position, lsp::Position::new(0, 0),);
            Ok(Some(ExpandedMacro {
                name: "test_macro_name".to_string(),
                expansion: "test_macro_expansion on the client".to_string(),
            }))
        });

    editor_b.update_in(cx_b, |editor, window, cx| {
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

fn extract_hint_labels(editor: &Editor) -> Vec<String> {
    let mut labels = Vec::new();
    for hint in editor.inlay_hint_cache().hints() {
        match hint.label {
            project::InlayHintLabel::String(s) => labels.push(s),
            _ => unreachable!(),
        }
    }
    labels
}

fn blame_entry(sha: &str, range: Range<u32>) -> git::blame::BlameEntry {
    git::blame::BlameEntry {
        sha: sha.parse().unwrap(),
        range,
        ..Default::default()
    }
}
