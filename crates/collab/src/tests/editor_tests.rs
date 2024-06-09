use crate::{
    rpc::RECONNECT_TIMEOUT,
    tests::{rust_lang, TestServer},
};
use call::ActiveCall;
use collections::HashMap;
use editor::{
    actions::{
        ConfirmCodeAction, ConfirmCompletion, ConfirmRename, ContextMenuFirst, Redo, Rename,
        RevertSelectedHunks, ToggleCodeActions, Undo,
    },
    display_map::DisplayRow,
    test::{
        editor_hunks,
        editor_test_context::{AssertionContextManager, EditorTestContext},
        expanded_hunks, expanded_hunks_background_highlights,
    },
    Editor,
};
use futures::StreamExt;
use git::diff::DiffHunkStatus;
use gpui::{TestAppContext, UpdateGlobal, VisualContext, VisualTestContext};
use indoc::indoc;
use language::{
    language_settings::{AllLanguageSettings, InlayHintSettings},
    FakeLspAdapter,
};
use multi_buffer::MultiBufferRow;
use project::{
    project_settings::{InlineBlameSettings, ProjectSettings},
    SERVER_PROGRESS_DEBOUNCE_TIMEOUT,
};
use rpc::RECEIVE_TIMEOUT;
use serde_json::json;
use settings::SettingsStore;
use std::{
    ops::Range,
    path::Path,
    sync::{
        atomic::{self, AtomicBool, AtomicUsize},
        Arc,
    },
};
use text::Point;
use workspace::Workspace;

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

    client_a
        .fs()
        .insert_tree(
            "/a",
            serde_json::json!({
                "a.txt": "a-contents",
                "b.txt": "b-contents",
            }),
        )
        .await;

    let active_call_a = cx_a.read(ActiveCall::global);
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;

    let worktree_a = project_a.read_with(cx_a, |project, _| project.worktrees().next().unwrap());
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.build_dev_server_project(project_id, cx_b).await;
    cx_a.background_executor.run_until_parked();

    assert!(worktree_a.read_with(cx_a, |tree, _| tree.as_local().unwrap().is_shared()));

    let workspace_b = cx_b
        .add_window(|cx| Workspace::new(None, project_b.clone(), client_b.app_state.clone(), cx));
    let cx_b = &mut VisualTestContext::from_window(*workspace_b, cx_b);
    let workspace_b_view = workspace_b.root_view(cx_b).unwrap();

    let editor_b = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "b.txt"), None, true, cx)
        })
        .unwrap()
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    //TODO: focus
    assert!(cx_b.update_view(&editor_b, |editor, cx| editor.is_focused(cx)));
    editor_b.update(cx_b, |editor, cx| editor.insert("X", cx));

    cx_b.update(|cx| {
        assert!(workspace_b_view.read(cx).is_edited());
    });

    // Drop client A's connection. Collaborators should disappear and the project should not be shown as shared.
    server.forbid_connections();
    server.disconnect_client(client_a.peer_id().unwrap());
    cx_a.background_executor
        .advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);

    project_a.read_with(cx_a, |project, _| project.collaborators().is_empty());

    project_a.read_with(cx_a, |project, _| assert!(!project.is_shared()));

    project_b.read_with(cx_b, |project, _| project.is_read_only());

    assert!(worktree_a.read_with(cx_a, |tree, _| !tree.as_local().unwrap().is_shared()));

    // Ensure client B's edited state is reset and that the whole window is blurred.

    workspace_b
        .update(cx_b, |workspace, cx| {
            assert_eq!(cx.focused(), None);
            assert!(!workspace.is_edited())
        })
        .unwrap();

    // Ensure client B is not prompted to save edits when closing window after disconnecting.
    let can_close = workspace_b
        .update(cx_b, |workspace, cx| workspace.prepare_to_close(true, cx))
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
        .insert_tree("/dir", json!({ "a.txt": "Some text\n" }))
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/dir", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.build_dev_server_project(project_id, cx_b).await;

    // Open a buffer as client A
    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();
    let cx_a = cx_a.add_empty_window();
    let editor_a = cx_a.new_view(|cx| Editor::for_buffer(buffer_a, Some(project_a), cx));

    let mut editor_cx_a = EditorTestContext {
        cx: cx_a.clone(),
        window: cx_a.handle(),
        editor: editor_a,
        assertion_cx: AssertionContextManager::new(),
    };

    let cx_b = cx_b.add_empty_window();
    // Open a buffer as client B
    let buffer_b = project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
        .await
        .unwrap();
    let editor_b = cx_b.new_view(|cx| Editor::for_buffer(buffer_b, Some(project_b), cx));

    let mut editor_cx_b = EditorTestContext {
        cx: cx_b.clone(),
        window: cx_b.handle(),
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
    editor_cx_a
        .update_editor(|editor, cx| editor.newline_above(&editor::actions::NewlineAbove, cx));
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
    editor_cx_a
        .update_editor(|editor, cx| editor.newline_below(&editor::actions::NewlineBelow, cx));
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
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp_adapter(
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
            "/a",
            json!({
                "main.rs": "fn main() { a }",
                "other.rs": "",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_dev_server_project(project_id, cx_b).await;

    // Open a file in an editor as the guest.
    let buffer_b = project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
        .await
        .unwrap();
    let cx_b = cx_b.add_empty_window();
    let editor_b =
        cx_b.new_view(|cx| Editor::for_buffer(buffer_b.clone(), Some(project_b.clone()), cx));

    let fake_language_server = fake_language_servers.next().await.unwrap();
    cx_a.background_executor.run_until_parked();

    buffer_b.read_with(cx_b, |buffer, _| {
        assert!(!buffer.completion_triggers().is_empty())
    });

    // Type a completion trigger character as the guest.
    editor_b.update(cx_b, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([13..13]));
        editor.handle_input(".", cx);
    });
    cx_b.focus_view(&editor_b);

    // Receive a completion request as the host's language server.
    // Return some completions from the host's language server.
    cx_a.executor().start_waiting();
    fake_language_server
        .handle_request::<lsp::request::Completion, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri,
                lsp::Url::from_file_path("/a/main.rs").unwrap(),
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
    editor_b.update(cx_b, |editor, cx| {
        assert!(editor.context_menu_visible());
        editor.confirm_completion(&ConfirmCompletion { item_ix: Some(0) }, cx);
        assert_eq!(editor.text(cx), "fn main() { a.first_method() }");
    });

    // Return a resolved completion from the host's language server.
    // The resolved completion has an additional text edit.
    fake_language_server.handle_request::<lsp::request::ResolveCompletionItem, _, _>(
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
    editor_b.update(cx_b, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([46..46]));
        editor.handle_input("; a", cx);
        editor.handle_input(".", cx);
    });

    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(
            buffer.text(),
            "use d::SomeTrait;\nfn main() { a.first_method(); a. }"
        );
    });

    let mut completion_response = fake_language_server
        .handle_request::<lsp::request::Completion, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri,
                lsp::Url::from_file_path("/a/main.rs").unwrap(),
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
        .handle_request::<lsp::request::ResolveCompletionItem, _, _>(|params, _| async move {
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

    editor_b.update(cx_b, |editor, cx| {
        assert!(editor.context_menu_visible());
        editor.context_menu_first(&ContextMenuFirst {}, cx);
    });

    resolve_completion_response.next().await.unwrap();
    cx_b.executor().run_until_parked();

    // When accepting the completion, the snippet is insert.
    editor_b.update(cx_b, |editor, cx| {
        assert!(editor.context_menu_visible());
        editor.confirm_completion(&ConfirmCompletion { item_ix: Some(0) }, cx);
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
        .register_fake_lsp_adapter("Rust", FakeLspAdapter::default());

    client_a
        .fs()
        .insert_tree(
            "/a",
            json!({
                "main.rs": "mod other;\nfn main() { let foo = other::foo(); }",
                "other.rs": "pub fn foo() -> usize { 4 }",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Join the project as client B.
    let project_b = client_b.build_dev_server_project(project_id, cx_b).await;
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    let editor_b = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "main.rs"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let mut fake_language_server = fake_language_servers.next().await.unwrap();
    let mut requests = fake_language_server
        .handle_request::<lsp::request::CodeActionRequest, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path("/a/main.rs").unwrap(),
            );
            assert_eq!(params.range.start, lsp::Position::new(0, 0));
            assert_eq!(params.range.end, lsp::Position::new(0, 0));
            Ok(None)
        });
    cx_a.background_executor
        .advance_clock(editor::CODE_ACTIONS_DEBOUNCE_TIMEOUT * 2);
    requests.next().await;

    // Move cursor to a location that contains code actions.
    editor_b.update(cx_b, |editor, cx| {
        editor.change_selections(None, cx, |s| {
            s.select_ranges([Point::new(1, 31)..Point::new(1, 31)])
        });
    });
    cx_b.focus_view(&editor_b);

    let mut requests = fake_language_server
        .handle_request::<lsp::request::CodeActionRequest, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document.uri,
                lsp::Url::from_file_path("/a/main.rs").unwrap(),
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
                                    lsp::Url::from_file_path("/a/main.rs").unwrap(),
                                    vec![lsp::TextEdit::new(
                                        lsp::Range::new(
                                            lsp::Position::new(1, 22),
                                            lsp::Position::new(1, 34),
                                        ),
                                        "4".to_string(),
                                    )],
                                ),
                                (
                                    lsp::Url::from_file_path("/a/other.rs").unwrap(),
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
    editor_b.update(cx_b, |editor, cx| {
        editor.toggle_code_actions(
            &ToggleCodeActions {
                deployed_from_indicator: None,
            },
            cx,
        );
    });
    cx_a.background_executor.run_until_parked();

    editor_b.update(cx_b, |editor, _| assert!(editor.context_menu_visible()));

    fake_language_server.remove_request_handler::<lsp::request::CodeActionRequest>();

    // Confirming the code action will trigger a resolve request.
    let confirm_action = editor_b
        .update(cx_b, |editor, cx| {
            Editor::confirm_code_action(editor, &ConfirmCodeAction { item_ix: Some(0) }, cx)
        })
        .unwrap();
    fake_language_server.handle_request::<lsp::request::CodeActionResolveRequest, _, _>(
        |_, _| async move {
            Ok(lsp::CodeAction {
                title: "Inline into all callers".to_string(),
                edit: Some(lsp::WorkspaceEdit {
                    changes: Some(
                        [
                            (
                                lsp::Url::from_file_path("/a/main.rs").unwrap(),
                                vec![lsp::TextEdit::new(
                                    lsp::Range::new(
                                        lsp::Position::new(1, 22),
                                        lsp::Position::new(1, 34),
                                    ),
                                    "4".to_string(),
                                )],
                            ),
                            (
                                lsp::Url::from_file_path("/a/other.rs").unwrap(),
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
    code_action_editor.update(cx_b, |editor, cx| {
        assert_eq!(editor.text(cx), "mod other;\nfn main() { let foo = 4; }\n");
        editor.undo(&Undo, cx);
        assert_eq!(
            editor.text(cx),
            "mod other;\nfn main() { let foo = other::foo(); }\npub fn foo() -> usize { 4 }"
        );
        editor.redo(&Redo, cx);
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
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp_adapter(
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
            "/dir",
            json!({
                "one.rs": "const ONE: usize = 1;",
                "two.rs": "const TWO: usize = one::ONE + one::ONE;"
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/dir", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_dev_server_project(project_id, cx_b).await;

    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    let editor_b = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "one.rs"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();
    let fake_language_server = fake_language_servers.next().await.unwrap();

    // Move cursor to a location that can be renamed.
    let prepare_rename = editor_b.update(cx_b, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([7..7]));
        editor.rename(&Rename, cx).unwrap()
    });

    fake_language_server
        .handle_request::<lsp::request::PrepareRenameRequest, _, _>(|params, _| async move {
            assert_eq!(params.text_document.uri.as_str(), "file:///dir/one.rs");
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
    editor_b.update(cx_b, |editor, cx| {
        editor.cancel(&editor::actions::Cancel, cx);
    });
    let prepare_rename = editor_b.update(cx_b, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([7..8]));
        editor.rename(&Rename, cx).unwrap()
    });

    fake_language_server
        .handle_request::<lsp::request::PrepareRenameRequest, _, _>(|params, _| async move {
            assert_eq!(params.text_document.uri.as_str(), "file:///dir/one.rs");
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

    let confirm_rename = editor_b.update(cx_b, |editor, cx| {
        Editor::confirm_rename(editor, &ConfirmRename, cx).unwrap()
    });
    fake_language_server
        .handle_request::<lsp::request::Rename, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri.as_str(),
                "file:///dir/one.rs"
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
                            lsp::Url::from_file_path("/dir/one.rs").unwrap(),
                            vec![lsp::TextEdit::new(
                                lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
                                "THREE".to_string(),
                            )],
                        ),
                        (
                            lsp::Url::from_file_path("/dir/two.rs").unwrap(),
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

    rename_editor.update(cx_b, |editor, cx| {
        assert_eq!(
            editor.text(cx),
            "const THREE: usize = 1;\nconst TWO: usize = one::THREE + one::THREE;"
        );
        editor.undo(&Undo, cx);
        assert_eq!(
            editor.text(cx),
            "const ONE: usize = 1;\nconst TWO: usize = one::ONE + one::ONE;"
        );
        editor.redo(&Redo, cx);
        assert_eq!(
            editor.text(cx),
            "const THREE: usize = 1;\nconst TWO: usize = one::THREE + one::THREE;"
        );
    });

    // Ensure temporary rename edits cannot be undone/redone.
    editor_b.update(cx_b, |editor, cx| {
        editor.undo(&Undo, cx);
        assert_eq!(editor.text(cx), "const ONE: usize = 1;");
        editor.undo(&Undo, cx);
        assert_eq!(editor.text(cx), "const ONE: usize = 1;");
        editor.redo(&Redo, cx);
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
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp_adapter(
        "Rust",
        FakeLspAdapter {
            name: "the-language-server",
            ..Default::default()
        },
    );

    client_a
        .fs()
        .insert_tree(
            "/dir",
            json!({
                "main.rs": "const ONE: usize = 1;",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/dir", cx_a).await;

    let _buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
        .await
        .unwrap();

    let fake_language_server = fake_language_servers.next().await.unwrap();
    fake_language_server.start_progress("the-token").await;
    fake_language_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
        token: lsp::NumberOrString::String("the-token".to_string()),
        value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::Report(
            lsp::WorkDoneProgressReport {
                message: Some("the-message".to_string()),
                ..Default::default()
            },
        )),
    });
    executor.advance_clock(SERVER_PROGRESS_DEBOUNCE_TIMEOUT);
    executor.run_until_parked();

    project_a.read_with(cx_a, |project, _| {
        let status = project.language_server_statuses().next().unwrap();
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
    let project_b = client_b.build_dev_server_project(project_id, cx_b).await;

    project_b.read_with(cx_b, |project, _| {
        let status = project.language_server_statuses().next().unwrap();
        assert_eq!(status.name, "the-language-server");
    });

    fake_language_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
        token: lsp::NumberOrString::String("the-token".to_string()),
        value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::Report(
            lsp::WorkDoneProgressReport {
                message: Some("the-message-2".to_string()),
                ..Default::default()
            },
        )),
    });
    executor.advance_clock(SERVER_PROGRESS_DEBOUNCE_TIMEOUT);
    executor.run_until_parked();

    project_a.read_with(cx_a, |project, _| {
        let status = project.language_server_statuses().next().unwrap();
        assert_eq!(status.name, "the-language-server");
        assert_eq!(status.pending_work.len(), 1);
        assert_eq!(
            status.pending_work["the-token"].message.as_ref().unwrap(),
            "the-message-2"
        );
    });

    project_b.read_with(cx_b, |project, _| {
        let status = project.language_server_statuses().next().unwrap();
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
            "/a",
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
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
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
    let project_b = client_b
        .build_dev_server_project(initial_project.id, cx_b)
        .await;

    let replica_id_b = project_b.read_with(cx_b, |project, _| project.replica_id());

    executor.run_until_parked();

    project_a.read_with(cx_a, |project, _| {
        let client_b_collaborator = project.collaborators().get(&client_b_peer_id).unwrap();
        assert_eq!(client_b_collaborator.replica_id, replica_id_b);
    });

    project_b.read_with(cx_b, |project, cx| {
        let worktree = project.worktrees().next().unwrap().read(cx);
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
            let worktree = project.worktrees().next().unwrap();
            let entry = worktree.read(cx).entry_for_path("ignored-dir").unwrap();
            project.expand_entry(worktree_id, entry.id, cx).unwrap()
        })
        .await
        .unwrap();

    project_b.read_with(cx_b, |project, cx| {
        let worktree = project.worktrees().next().unwrap().read(cx);
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

    let editor_b = cx_b.new_view(|cx| Editor::for_buffer(buffer_b, None, cx));

    // Client A sees client B's selection
    executor.run_until_parked();

    buffer_a.read_with(cx_a, |buffer, _| {
        buffer
            .snapshot()
            .remote_selections_in_range(text::Anchor::MIN..text::Anchor::MAX)
            .count()
            == 1
    });

    // Edit the buffer as client B and see that edit as client A.
    editor_b.update(cx_b, |editor, cx| editor.handle_input("ok, ", cx));
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
    let _project_c = client_c
        .build_dev_server_project(initial_project.id, cx_c)
        .await;

    // Client B closes the editor, and client A sees client B's selections removed.
    cx_b.update(move |_| drop(editor_b));
    executor.run_until_parked();

    buffer_a.read_with(cx_a, |buffer, _| {
        buffer
            .snapshot()
            .remote_selections_in_range(text::Anchor::MIN..text::Anchor::MAX)
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
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp_adapter(
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
            "/a",
            json!({
                "main.rs": "fn main() { a }",
                "other.rs": "// Test file",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_dev_server_project(project_id, cx_b).await;

    // Open a file in an editor as the host.
    let buffer_a = project_a
        .update(cx_a, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
        .await
        .unwrap();
    let cx_a = cx_a.add_empty_window();
    let editor_a = cx_a.new_view(|cx| Editor::for_buffer(buffer_a, Some(project_a.clone()), cx));

    let fake_language_server = fake_language_servers.next().await.unwrap();
    executor.run_until_parked();

    // Receive an OnTypeFormatting request as the host's language server.
    // Return some formatting from the host's language server.
    fake_language_server.handle_request::<lsp::request::OnTypeFormatting, _, _>(
        |params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri,
                lsp::Url::from_file_path("/a/main.rs").unwrap(),
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
    cx_a.focus_view(&editor_a);
    editor_a.update(cx_a, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([13..13]));
        editor.handle_input(">", cx);
    });

    executor.run_until_parked();

    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a>~< }")
    });

    // Undo should remove LSP edits first
    editor_a.update(cx_a, |editor, cx| {
        assert_eq!(editor.text(cx), "fn main() { a>~< }");
        editor.undo(&Undo, cx);
        assert_eq!(editor.text(cx), "fn main() { a> }");
    });
    executor.run_until_parked();

    buffer_b.read_with(cx_b, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a> }")
    });

    editor_a.update(cx_a, |editor, cx| {
        assert_eq!(editor.text(cx), "fn main() { a> }");
        editor.undo(&Undo, cx);
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
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp_adapter(
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
            "/a",
            json!({
                "main.rs": "fn main() { a }",
                "other.rs": "// Test file",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.build_dev_server_project(project_id, cx_b).await;

    // Open a file in an editor as the guest.
    let buffer_b = project_b
        .update(cx_b, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
        .await
        .unwrap();
    let cx_b = cx_b.add_empty_window();
    let editor_b = cx_b.new_view(|cx| Editor::for_buffer(buffer_b, Some(project_b.clone()), cx));

    let fake_language_server = fake_language_servers.next().await.unwrap();
    executor.run_until_parked();

    // Type a on type formatting trigger character as the guest.
    cx_b.focus_view(&editor_b);
    editor_b.update(cx_b, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([13..13]));
        editor.handle_input(":", cx);
    });

    // Receive an OnTypeFormatting request as the host's language server.
    // Return some formatting from the host's language server.
    executor.start_waiting();
    fake_language_server
        .handle_request::<lsp::request::OnTypeFormatting, _, _>(|params, _| async move {
            assert_eq!(
                params.text_document_position.text_document.uri,
                lsp::Url::from_file_path("/a/main.rs").unwrap(),
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
    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(editor.text(cx), "fn main() { a:~: }");
        editor.undo(&Undo, cx);
        assert_eq!(editor.text(cx), "fn main() { a: }");
    });
    executor.run_until_parked();

    buffer_a.read_with(cx_a, |buffer, _| {
        assert_eq!(buffer.text(), "fn main() { a: }")
    });

    editor_b.update(cx_b, |editor, cx| {
        assert_eq!(editor.text(cx), "fn main() { a: }");
        editor.undo(&Undo, cx);
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
                })
            });
        });
    });

    client_a.language_registry().add(rust_lang());
    client_b.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp_adapter(
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
            "/a",
            json!({
                "main.rs": "fn main() { a } // and some long comment to ensure inlay hints are not trimmed out",
                "other.rs": "// Test file",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Client B joins the project
    let project_b = client_b.build_dev_server_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    executor.start_waiting();

    // The host opens a rust file.
    let _buffer_a = project_a
        .update(cx_a, |project, cx| {
            project.open_local_buffer("/a/main.rs", cx)
        })
        .await
        .unwrap();
    let fake_language_server = fake_language_servers.next().await.unwrap();
    let editor_a = workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "main.rs"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Set up the language server to return an additional inlay hint on each request.
    let edits_made = Arc::new(AtomicUsize::new(0));
    let closure_edits_made = Arc::clone(&edits_made);
    fake_language_server
        .handle_request::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
            let task_edits_made = Arc::clone(&closure_edits_made);
            async move {
                assert_eq!(
                    params.text_document.uri,
                    lsp::Url::from_file_path("/a/main.rs").unwrap(),
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
        let inlay_cache = editor.inlay_hint_cache();
        assert_eq!(
            inlay_cache.version(),
            1,
            "Host editor update the cache version after every cache/view change",
        );
    });
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    let editor_b = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "main.rs"), None, true, cx)
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
        let inlay_cache = editor.inlay_hint_cache();
        assert_eq!(
            inlay_cache.version(),
            1,
            "Guest editor update the cache version after every cache/view change"
        );
    });

    let after_client_edit = edits_made.fetch_add(1, atomic::Ordering::Release) + 1;
    editor_b.update(cx_b, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([13..13].clone()));
        editor.handle_input(":", cx);
    });
    cx_b.focus_view(&editor_b);

    executor.run_until_parked();
    editor_a.update(cx_a, |editor, _| {
        assert_eq!(
            vec![after_client_edit.to_string()],
            extract_hint_labels(editor),
        );
        let inlay_cache = editor.inlay_hint_cache();
        assert_eq!(inlay_cache.version(), 2);
    });
    editor_b.update(cx_b, |editor, _| {
        assert_eq!(
            vec![after_client_edit.to_string()],
            extract_hint_labels(editor),
        );
        let inlay_cache = editor.inlay_hint_cache();
        assert_eq!(inlay_cache.version(), 2);
    });

    let after_host_edit = edits_made.fetch_add(1, atomic::Ordering::Release) + 1;
    editor_a.update(cx_a, |editor, cx| {
        editor.change_selections(None, cx, |s| s.select_ranges([13..13]));
        editor.handle_input("a change to increment both buffers' versions", cx);
    });
    cx_a.focus_view(&editor_a);

    executor.run_until_parked();
    editor_a.update(cx_a, |editor, _| {
        assert_eq!(
            vec![after_host_edit.to_string()],
            extract_hint_labels(editor),
        );
        let inlay_cache = editor.inlay_hint_cache();
        assert_eq!(inlay_cache.version(), 3);
    });
    editor_b.update(cx_b, |editor, _| {
        assert_eq!(
            vec![after_host_edit.to_string()],
            extract_hint_labels(editor),
        );
        let inlay_cache = editor.inlay_hint_cache();
        assert_eq!(inlay_cache.version(), 3);
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
        let inlay_cache = editor.inlay_hint_cache();
        assert_eq!(
            inlay_cache.version(),
            4,
            "Host should accepted all edits and bump its cache version every time"
        );
    });
    editor_b.update(cx_b, |editor, _| {
        assert_eq!(
            vec![after_special_edit_for_refresh.to_string()],
            extract_hint_labels(editor),
            "Guest should get a /refresh LSP request propagated by host"
        );
        let inlay_cache = editor.inlay_hint_cache();
        assert_eq!(
            inlay_cache.version(),
            4,
            "Guest should accepted all edits and bump its cache version every time"
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
                })
            });
        });
    });

    client_a.language_registry().add(rust_lang());
    client_b.language_registry().add(rust_lang());
    let mut fake_language_servers = client_a.language_registry().register_fake_lsp_adapter(
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
            "/a",
            json!({
                "main.rs": "fn main() { a } // and some long comment to ensure inlay hints are not trimmed out",
                "other.rs": "// Test file",
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.build_dev_server_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    cx_a.background_executor.start_waiting();

    let editor_a = workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "main.rs"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let editor_b = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "main.rs"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let other_hints = Arc::new(AtomicBool::new(false));
    let fake_language_server = fake_language_servers.next().await.unwrap();
    let closure_other_hints = Arc::clone(&other_hints);
    fake_language_server
        .handle_request::<lsp::request::InlayHintRequest, _, _>(move |params, _| {
            let task_other_hints = Arc::clone(&closure_other_hints);
            async move {
                assert_eq!(
                    params.text_document.uri,
                    lsp::Url::from_file_path("/a/main.rs").unwrap(),
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
        let inlay_cache = editor.inlay_hint_cache();
        assert_eq!(
            inlay_cache.version(),
            0,
            "Turned off hints should not generate version updates"
        );
    });

    executor.run_until_parked();
    editor_b.update(cx_b, |editor, _| {
        assert_eq!(
            vec!["initial hint".to_string()],
            extract_hint_labels(editor),
            "Client should get its first hints when opens an editor"
        );
        let inlay_cache = editor.inlay_hint_cache();
        assert_eq!(
            inlay_cache.version(),
            1,
            "Should update cache version after first hints"
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
            "Host should get nop hints due to them turned off, even after the /refresh"
        );
        let inlay_cache = editor.inlay_hint_cache();
        assert_eq!(
            inlay_cache.version(),
            0,
            "Turned off hints should not generate version updates, again"
        );
    });

    executor.run_until_parked();
    editor_b.update(cx_b, |editor, _| {
        assert_eq!(
            vec!["other hint".to_string()],
            extract_hint_labels(editor),
            "Guest should get a /refresh LSP request propagated by host despite host hints are off"
        );
        let inlay_cache = editor.inlay_hint_cache();
        assert_eq!(
            inlay_cache.version(),
            2,
            "Guest should accepted all edits and bump its cache version every time"
        );
    });
}

#[gpui::test]
async fn test_multiple_hunk_types_revert(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
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

    let base_text = indoc! {r#"struct Row;
struct Row1;
struct Row2;

struct Row4;
struct Row5;
struct Row6;

struct Row8;
struct Row9;
struct Row10;"#};

    client_a
        .fs()
        .insert_tree(
            "/a",
            json!({
                "main.rs": base_text,
            }),
        )
        .await;
    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let project_b = client_b.build_dev_server_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    let editor_a = workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "main.rs"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let editor_b = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "main.rs"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    let mut editor_cx_a = EditorTestContext {
        cx: cx_a.clone(),
        window: cx_a.handle(),
        editor: editor_a,
        assertion_cx: AssertionContextManager::new(),
    };
    let mut editor_cx_b = EditorTestContext {
        cx: cx_b.clone(),
        window: cx_b.handle(),
        editor: editor_b,
        assertion_cx: AssertionContextManager::new(),
    };

    // host edits the file, that differs from the base text, producing diff hunks
    editor_cx_a.set_state(indoc! {r#"struct Row;
        struct Row0.1;
        struct Row0.2;
        struct Row1;

        struct Row4;
        struct Row5444;
        struct Row6;

        struct Row9;
        struct Row1220;ˇ"#});
    editor_cx_a.update_editor(|editor, cx| {
        editor
            .buffer()
            .read(cx)
            .as_singleton()
            .unwrap()
            .update(cx, |buffer, cx| {
                buffer.set_diff_base(Some(base_text.into()), cx);
            });
    });
    editor_cx_b.update_editor(|editor, cx| {
        editor
            .buffer()
            .read(cx)
            .as_singleton()
            .unwrap()
            .update(cx, |buffer, cx| {
                buffer.set_diff_base(Some(base_text.into()), cx);
            });
    });
    cx_a.executor().run_until_parked();
    cx_b.executor().run_until_parked();

    // the client selects a range in the updated buffer, expands it to see the diff for each hunk in the selection
    // the host does not see the diffs toggled
    editor_cx_b.set_selections_state(indoc! {r#"«ˇstruct Row;
        struct Row0.1;
        struct Row0.2;
        struct Row1;

        struct Row4;
        struct Row5444;
        struct Row6;

        struct R»ow9;
        struct Row1220;"#});
    editor_cx_b
        .update_editor(|editor, cx| editor.toggle_hunk_diff(&editor::actions::ToggleHunkDiff, cx));
    cx_a.executor().run_until_parked();
    cx_b.executor().run_until_parked();
    editor_cx_a.update_editor(|editor, cx| {
        let snapshot = editor.snapshot(cx);
        let all_hunks = editor_hunks(editor, &snapshot, cx);
        let all_expanded_hunks = expanded_hunks(&editor, &snapshot, cx);
        assert_eq!(expanded_hunks_background_highlights(editor, cx), Vec::new());
        assert_eq!(
            all_hunks,
            vec![
                (
                    "".to_string(),
                    DiffHunkStatus::Added,
                    DisplayRow(1)..DisplayRow(3)
                ),
                (
                    "struct Row2;\n".to_string(),
                    DiffHunkStatus::Removed,
                    DisplayRow(4)..DisplayRow(4)
                ),
                (
                    "struct Row5;\n".to_string(),
                    DiffHunkStatus::Modified,
                    DisplayRow(6)..DisplayRow(7)
                ),
                (
                    "struct Row8;\n".to_string(),
                    DiffHunkStatus::Removed,
                    DisplayRow(9)..DisplayRow(9)
                ),
                (
                    "struct Row10;".to_string(),
                    DiffHunkStatus::Modified,
                    DisplayRow(10)..DisplayRow(10),
                ),
            ]
        );
        assert_eq!(all_expanded_hunks, Vec::new());
    });
    editor_cx_b.update_editor(|editor, cx| {
        let snapshot = editor.snapshot(cx);
        let all_hunks = editor_hunks(editor, &snapshot, cx);
        let all_expanded_hunks = expanded_hunks(&editor, &snapshot, cx);
        assert_eq!(
            expanded_hunks_background_highlights(editor, cx),
            vec![DisplayRow(1)..=DisplayRow(2), DisplayRow(8)..=DisplayRow(8)],
        );
        assert_eq!(
            all_hunks,
            vec![
                (
                    "".to_string(),
                    DiffHunkStatus::Added,
                    DisplayRow(1)..DisplayRow(3)
                ),
                (
                    "struct Row2;\n".to_string(),
                    DiffHunkStatus::Removed,
                    DisplayRow(5)..DisplayRow(5)
                ),
                (
                    "struct Row5;\n".to_string(),
                    DiffHunkStatus::Modified,
                    DisplayRow(8)..DisplayRow(9)
                ),
                (
                    "struct Row8;\n".to_string(),
                    DiffHunkStatus::Removed,
                    DisplayRow(12)..DisplayRow(12)
                ),
                (
                    "struct Row10;".to_string(),
                    DiffHunkStatus::Modified,
                    DisplayRow(13)..DisplayRow(13),
                ),
            ]
        );
        assert_eq!(all_expanded_hunks, &all_hunks[..all_hunks.len() - 1]);
    });

    // the client reverts the hunks, removing the expanded diffs too
    // both host and the client observe the reverted state (with one hunk left, not covered by client's selection)
    editor_cx_b.update_editor(|editor, cx| {
        editor.revert_selected_hunks(&RevertSelectedHunks, cx);
    });
    cx_a.executor().run_until_parked();
    cx_b.executor().run_until_parked();
    editor_cx_a.update_editor(|editor, cx| {
        let snapshot = editor.snapshot(cx);
        let all_hunks = editor_hunks(editor, &snapshot, cx);
        let all_expanded_hunks = expanded_hunks(&editor, &snapshot, cx);
        assert_eq!(expanded_hunks_background_highlights(editor, cx), Vec::new());
        assert_eq!(
            all_hunks,
            vec![(
                "struct Row10;".to_string(),
                DiffHunkStatus::Modified,
                DisplayRow(10)..DisplayRow(10),
            )]
        );
        assert_eq!(all_expanded_hunks, Vec::new());
    });
    editor_cx_b.update_editor(|editor, cx| {
        let snapshot = editor.snapshot(cx);
        let all_hunks = editor_hunks(editor, &snapshot, cx);
        let all_expanded_hunks = expanded_hunks(&editor, &snapshot, cx);
        assert_eq!(
            expanded_hunks_background_highlights(editor, cx),
            vec![DisplayRow(5)..=DisplayRow(5)]
        );
        assert_eq!(
            all_hunks,
            vec![(
                "struct Row10;".to_string(),
                DiffHunkStatus::Modified,
                DisplayRow(10)..DisplayRow(10),
            )]
        );
        assert_eq!(all_expanded_hunks, Vec::new());
    });
    editor_cx_a.assert_editor_state(indoc! {r#"struct Row;
        struct Row1;
        struct Row2;

        struct Row4;
        struct Row5;
        struct Row6;

        struct Row8;
        struct Row9;
        struct Row1220;ˇ"#});
    editor_cx_b.assert_editor_state(indoc! {r#"«ˇstruct Row;
        struct Row1;
        struct Row2;

        struct Row4;
        struct Row5;
        struct Row6;

        struct Row8;
        struct R»ow9;
        struct Row1220;"#});
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
            "/my-repo",
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
        permalinks: HashMap::default(), // This field is deprecrated
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
        Path::new("/my-repo/.git"),
        vec![(Path::new("file.txt"), blame)],
    );

    let (project_a, worktree_id) = client_a.build_local_project("/my-repo", cx_a).await;
    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    // Create editor_a
    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let editor_a = workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path((worktree_id, "file.txt"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // Join the project as client B.
    let project_b = client_b.build_dev_server_project(project_id, cx_b).await;
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    let editor_b = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path((worktree_id, "file.txt"), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    // client_b now requests git blame for the open buffer
    editor_b.update(cx_b, |editor_b, cx| {
        assert!(editor_b.blame().is_none());
        editor_b.toggle_git_blame(&editor::actions::ToggleGitBlame {}, cx);
    });

    cx_a.executor().run_until_parked();
    cx_b.executor().run_until_parked();

    editor_b.update(cx_b, |editor_b, cx| {
        let blame = editor_b.blame().expect("editor_b should have blame now");
        let entries = blame.update(cx, |blame, cx| {
            blame
                .blame_for_rows((0..4).map(MultiBufferRow).map(Some), cx)
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
    editor_b.update(cx_b, |editor_b, cx| {
        editor_b.edit([(Point::new(0, 3)..Point::new(0, 3), "FOO")], cx);
    });

    cx_a.executor().run_until_parked();
    cx_b.executor().run_until_parked();

    editor_b.update(cx_b, |editor_b, cx| {
        let blame = editor_b.blame().expect("editor_b should have blame now");
        let entries = blame.update(cx, |blame, cx| {
            blame
                .blame_for_rows((0..4).map(MultiBufferRow).map(Some), cx)
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
    editor_a.update(cx_a, |editor_a, cx| {
        editor_a.edit([(Point::new(1, 3)..Point::new(1, 3), "FOO")], cx);
    });

    cx_a.executor().run_until_parked();
    cx_b.executor().run_until_parked();

    editor_b.update(cx_b, |editor_b, cx| {
        let blame = editor_b.blame().expect("editor_b should have blame now");
        let entries = blame.update(cx, |blame, cx| {
            blame
                .blame_for_rows((0..4).map(MultiBufferRow).map(Some), cx)
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
