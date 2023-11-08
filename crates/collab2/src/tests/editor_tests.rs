// use editor::{
//     test::editor_test_context::EditorTestContext, ConfirmCodeAction, ConfirmCompletion,
//     ConfirmRename, Editor, Redo, Rename, ToggleCodeActions, Undo,
// };

//todo!(editor)
// #[gpui::test(iterations = 10)]
// async fn test_host_disconnect(
//     executor: BackgroundExecutor,
//     cx_a: &mut TestAppContext,
//     cx_b: &mut TestAppContext,
//     cx_c: &mut TestAppContext,
// ) {
//     let mut server = TestServer::start(&executor).await;
//     let client_a = server.create_client(cx_a, "user_a").await;
//     let client_b = server.create_client(cx_b, "user_b").await;
//     let client_c = server.create_client(cx_c, "user_c").await;
//     server
//         .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
//         .await;

//     cx_b.update(editor::init);

//     client_a
//         .fs()
//         .insert_tree(
//             "/a",
//             json!({
//                 "a.txt": "a-contents",
//                 "b.txt": "b-contents",
//             }),
//         )
//         .await;

//     let active_call_a = cx_a.read(ActiveCall::global);
//     let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;

//     let worktree_a = project_a.read_with(cx_a, |project, cx| project.worktrees(cx).next().unwrap());
//     let project_id = active_call_a
//         .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
//         .await
//         .unwrap();

//     let project_b = client_b.build_remote_project(project_id, cx_b).await;
//     executor.run_until_parked();

//     assert!(worktree_a.read_with(cx_a, |tree, _| tree.as_local().unwrap().is_shared()));

//     let window_b =
//         cx_b.add_window(|cx| Workspace::new(0, project_b.clone(), client_b.app_state.clone(), cx));
//     let workspace_b = window_b.root(cx_b);
//     let editor_b = workspace_b
//         .update(cx_b, |workspace, cx| {
//             workspace.open_path((worktree_id, "b.txt"), None, true, cx)
//         })
//         .await
//         .unwrap()
//         .downcast::<Editor>()
//         .unwrap();

//     assert!(window_b.read_with(cx_b, |cx| editor_b.is_focused(cx)));
//     editor_b.update(cx_b, |editor, cx| editor.insert("X", cx));
//     assert!(window_b.is_edited(cx_b));

//     // Drop client A's connection. Collaborators should disappear and the project should not be shown as shared.
//     server.forbid_connections();
//     server.disconnect_client(client_a.peer_id().unwrap());
//     executor.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);

//     project_a.read_with(cx_a, |project, _| project.collaborators().is_empty());

//     project_a.read_with(cx_a, |project, _| assert!(!project.is_shared()));

//     project_b.read_with(cx_b, |project, _| project.is_read_only());

//     assert!(worktree_a.read_with(cx_a, |tree, _| !tree.as_local().unwrap().is_shared()));

//     // Ensure client B's edited state is reset and that the whole window is blurred.

//     window_b.read_with(cx_b, |cx| {
//         assert_eq!(cx.focused_view_id(), None);
//     });
//     assert!(!window_b.is_edited(cx_b));

//     // Ensure client B is not prompted to save edits when closing window after disconnecting.
//     let can_close = workspace_b
//         .update(cx_b, |workspace, cx| workspace.prepare_to_close(true, cx))
//         .await
//         .unwrap();
//     assert!(can_close);

//     // Allow client A to reconnect to the server.
//     server.allow_connections();
//     executor.advance_clock(RECEIVE_TIMEOUT);

//     // Client B calls client A again after they reconnected.
//     let active_call_b = cx_b.read(ActiveCall::global);
//     active_call_b
//         .update(cx_b, |call, cx| {
//             call.invite(client_a.user_id().unwrap(), None, cx)
//         })
//         .await
//         .unwrap();
//     executor.run_until_parked();
//     active_call_a
//         .update(cx_a, |call, cx| call.accept_incoming(cx))
//         .await
//         .unwrap();

//     active_call_a
//         .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
//         .await
//         .unwrap();

//     // Drop client A's connection again. We should still unshare it successfully.
//     server.forbid_connections();
//     server.disconnect_client(client_a.peer_id().unwrap());
//     executor.advance_clock(RECEIVE_TIMEOUT + RECONNECT_TIMEOUT);

//     project_a.read_with(cx_a, |project, _| assert!(!project.is_shared()));
// }

//todo!(editor)
// #[gpui::test]
// async fn test_newline_above_or_below_does_not_move_guest_cursor(
//     executor: BackgroundExecutor,
//     cx_a: &mut TestAppContext,
//     cx_b: &mut TestAppContext,
// ) {
//     let mut server = TestServer::start(&executor).await;
//     let client_a = server.create_client(cx_a, "user_a").await;
//     let client_b = server.create_client(cx_b, "user_b").await;
//     server
//         .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
//         .await;
//     let active_call_a = cx_a.read(ActiveCall::global);

//     client_a
//         .fs()
//         .insert_tree("/dir", json!({ "a.txt": "Some text\n" }))
//         .await;
//     let (project_a, worktree_id) = client_a.build_local_project("/dir", cx_a).await;
//     let project_id = active_call_a
//         .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
//         .await
//         .unwrap();

//     let project_b = client_b.build_remote_project(project_id, cx_b).await;

//     // Open a buffer as client A
//     let buffer_a = project_a
//         .update(cx_a, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
//         .await
//         .unwrap();
//     let window_a = cx_a.add_window(|_| EmptyView);
//     let editor_a = window_a.add_view(cx_a, |cx| Editor::for_buffer(buffer_a, Some(project_a), cx));
//     let mut editor_cx_a = EditorTestContext {
//         cx: cx_a,
//         window: window_a.into(),
//         editor: editor_a,
//     };

//     // Open a buffer as client B
//     let buffer_b = project_b
//         .update(cx_b, |p, cx| p.open_buffer((worktree_id, "a.txt"), cx))
//         .await
//         .unwrap();
//     let window_b = cx_b.add_window(|_| EmptyView);
//     let editor_b = window_b.add_view(cx_b, |cx| Editor::for_buffer(buffer_b, Some(project_b), cx));
//     let mut editor_cx_b = EditorTestContext {
//         cx: cx_b,
//         window: window_b.into(),
//         editor: editor_b,
//     };

//     // Test newline above
//     editor_cx_a.set_selections_state(indoc! {"
//         Some textˇ
//     "});
//     editor_cx_b.set_selections_state(indoc! {"
//         Some textˇ
//     "});
//     editor_cx_a.update_editor(|editor, cx| editor.newline_above(&editor::NewlineAbove, cx));
//     executor.run_until_parked();
//     editor_cx_a.assert_editor_state(indoc! {"
//         ˇ
//         Some text
//     "});
//     editor_cx_b.assert_editor_state(indoc! {"

//         Some textˇ
//     "});

//     // Test newline below
//     editor_cx_a.set_selections_state(indoc! {"

//         Some textˇ
//     "});
//     editor_cx_b.set_selections_state(indoc! {"

//         Some textˇ
//     "});
//     editor_cx_a.update_editor(|editor, cx| editor.newline_below(&editor::NewlineBelow, cx));
//     executor.run_until_parked();
//     editor_cx_a.assert_editor_state(indoc! {"

//         Some text
//         ˇ
//     "});
//     editor_cx_b.assert_editor_state(indoc! {"

//         Some textˇ

//     "});
// }

//todo!(editor)
// #[gpui::test(iterations = 10)]
// async fn test_collaborating_with_completion(
//     executor: BackgroundExecutor,
//     cx_a: &mut TestAppContext,
//     cx_b: &mut TestAppContext,
// ) {
//     let mut server = TestServer::start(&executor).await;
//     let client_a = server.create_client(cx_a, "user_a").await;
//     let client_b = server.create_client(cx_b, "user_b").await;
//     server
//         .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
//         .await;
//     let active_call_a = cx_a.read(ActiveCall::global);

//     // Set up a fake language server.
//     let mut language = Language::new(
//         LanguageConfig {
//             name: "Rust".into(),
//             path_suffixes: vec!["rs".to_string()],
//             ..Default::default()
//         },
//         Some(tree_sitter_rust::language()),
//     );
//     let mut fake_language_servers = language
//         .set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
//             capabilities: lsp::ServerCapabilities {
//                 completion_provider: Some(lsp::CompletionOptions {
//                     trigger_characters: Some(vec![".".to_string()]),
//                     resolve_provider: Some(true),
//                     ..Default::default()
//                 }),
//                 ..Default::default()
//             },
//             ..Default::default()
//         }))
//         .await;
//     client_a.language_registry().add(Arc::new(language));

//     client_a
//         .fs()
//         .insert_tree(
//             "/a",
//             json!({
//                 "main.rs": "fn main() { a }",
//                 "other.rs": "",
//             }),
//         )
//         .await;
//     let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
//     let project_id = active_call_a
//         .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
//         .await
//         .unwrap();
//     let project_b = client_b.build_remote_project(project_id, cx_b).await;

//     // Open a file in an editor as the guest.
//     let buffer_b = project_b
//         .update(cx_b, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
//         .await
//         .unwrap();
//     let window_b = cx_b.add_window(|_| EmptyView);
//     let editor_b = window_b.add_view(cx_b, |cx| {
//         Editor::for_buffer(buffer_b.clone(), Some(project_b.clone()), cx)
//     });

//     let fake_language_server = fake_language_servers.next().await.unwrap();
//     cx_a.foreground().run_until_parked();

//     buffer_b.read_with(cx_b, |buffer, _| {
//         assert!(!buffer.completion_triggers().is_empty())
//     });

//     // Type a completion trigger character as the guest.
//     editor_b.update(cx_b, |editor, cx| {
//         editor.change_selections(None, cx, |s| s.select_ranges([13..13]));
//         editor.handle_input(".", cx);
//         cx.focus(&editor_b);
//     });

//     // Receive a completion request as the host's language server.
//     // Return some completions from the host's language server.
//     cx_a.foreground().start_waiting();
//     fake_language_server
//         .handle_request::<lsp::request::Completion, _, _>(|params, _| async move {
//             assert_eq!(
//                 params.text_document_position.text_document.uri,
//                 lsp::Url::from_file_path("/a/main.rs").unwrap(),
//             );
//             assert_eq!(
//                 params.text_document_position.position,
//                 lsp::Position::new(0, 14),
//             );

//             Ok(Some(lsp::CompletionResponse::Array(vec![
//                 lsp::CompletionItem {
//                     label: "first_method(…)".into(),
//                     detail: Some("fn(&mut self, B) -> C".into()),
//                     text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
//                         new_text: "first_method($1)".to_string(),
//                         range: lsp::Range::new(
//                             lsp::Position::new(0, 14),
//                             lsp::Position::new(0, 14),
//                         ),
//                     })),
//                     insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
//                     ..Default::default()
//                 },
//                 lsp::CompletionItem {
//                     label: "second_method(…)".into(),
//                     detail: Some("fn(&mut self, C) -> D<E>".into()),
//                     text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
//                         new_text: "second_method()".to_string(),
//                         range: lsp::Range::new(
//                             lsp::Position::new(0, 14),
//                             lsp::Position::new(0, 14),
//                         ),
//                     })),
//                     insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
//                     ..Default::default()
//                 },
//             ])))
//         })
//         .next()
//         .await
//         .unwrap();
//     cx_a.foreground().finish_waiting();

//     // Open the buffer on the host.
//     let buffer_a = project_a
//         .update(cx_a, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
//         .await
//         .unwrap();
//     cx_a.foreground().run_until_parked();

//     buffer_a.read_with(cx_a, |buffer, _| {
//         assert_eq!(buffer.text(), "fn main() { a. }")
//     });

//     // Confirm a completion on the guest.

//     editor_b.read_with(cx_b, |editor, _| assert!(editor.context_menu_visible()));
//     editor_b.update(cx_b, |editor, cx| {
//         editor.confirm_completion(&ConfirmCompletion { item_ix: Some(0) }, cx);
//         assert_eq!(editor.text(cx), "fn main() { a.first_method() }");
//     });

//     // Return a resolved completion from the host's language server.
//     // The resolved completion has an additional text edit.
//     fake_language_server.handle_request::<lsp::request::ResolveCompletionItem, _, _>(
//         |params, _| async move {
//             assert_eq!(params.label, "first_method(…)");
//             Ok(lsp::CompletionItem {
//                 label: "first_method(…)".into(),
//                 detail: Some("fn(&mut self, B) -> C".into()),
//                 text_edit: Some(lsp::CompletionTextEdit::Edit(lsp::TextEdit {
//                     new_text: "first_method($1)".to_string(),
//                     range: lsp::Range::new(lsp::Position::new(0, 14), lsp::Position::new(0, 14)),
//                 })),
//                 additional_text_edits: Some(vec![lsp::TextEdit {
//                     new_text: "use d::SomeTrait;\n".to_string(),
//                     range: lsp::Range::new(lsp::Position::new(0, 0), lsp::Position::new(0, 0)),
//                 }]),
//                 insert_text_format: Some(lsp::InsertTextFormat::SNIPPET),
//                 ..Default::default()
//             })
//         },
//     );

//     // The additional edit is applied.
//     cx_a.foreground().run_until_parked();

//     buffer_a.read_with(cx_a, |buffer, _| {
//         assert_eq!(
//             buffer.text(),
//             "use d::SomeTrait;\nfn main() { a.first_method() }"
//         );
//     });

//     buffer_b.read_with(cx_b, |buffer, _| {
//         assert_eq!(
//             buffer.text(),
//             "use d::SomeTrait;\nfn main() { a.first_method() }"
//         );
//     });
// }
//todo!(editor)
// #[gpui::test(iterations = 10)]
// async fn test_collaborating_with_code_actions(
//     executor: BackgroundExecutor,
//     cx_a: &mut TestAppContext,
//     cx_b: &mut TestAppContext,
// ) {
//     let mut server = TestServer::start(&executor).await;
//     let client_a = server.create_client(cx_a, "user_a").await;
//     //
//     let client_b = server.create_client(cx_b, "user_b").await;
//     server
//         .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
//         .await;
//     let active_call_a = cx_a.read(ActiveCall::global);

//     cx_b.update(editor::init);

//     // Set up a fake language server.
//     let mut language = Language::new(
//         LanguageConfig {
//             name: "Rust".into(),
//             path_suffixes: vec!["rs".to_string()],
//             ..Default::default()
//         },
//         Some(tree_sitter_rust::language()),
//     );
//     let mut fake_language_servers = language.set_fake_lsp_adapter(Default::default()).await;
//     client_a.language_registry().add(Arc::new(language));

//     client_a
//         .fs()
//         .insert_tree(
//             "/a",
//             json!({
//                 "main.rs": "mod other;\nfn main() { let foo = other::foo(); }",
//                 "other.rs": "pub fn foo() -> usize { 4 }",
//             }),
//         )
//         .await;
//     let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
//     let project_id = active_call_a
//         .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
//         .await
//         .unwrap();

//     // Join the project as client B.
//     let project_b = client_b.build_remote_project(project_id, cx_b).await;
//     let window_b =
//         cx_b.add_window(|cx| Workspace::new(0, project_b.clone(), client_b.app_state.clone(), cx));
//     let workspace_b = window_b.root(cx_b);
//     let editor_b = workspace_b
//         .update(cx_b, |workspace, cx| {
//             workspace.open_path((worktree_id, "main.rs"), None, true, cx)
//         })
//         .await
//         .unwrap()
//         .downcast::<Editor>()
//         .unwrap();

//     let mut fake_language_server = fake_language_servers.next().await.unwrap();
//     let mut requests = fake_language_server
//         .handle_request::<lsp::request::CodeActionRequest, _, _>(|params, _| async move {
//             assert_eq!(
//                 params.text_document.uri,
//                 lsp::Url::from_file_path("/a/main.rs").unwrap(),
//             );
//             assert_eq!(params.range.start, lsp::Position::new(0, 0));
//             assert_eq!(params.range.end, lsp::Position::new(0, 0));
//             Ok(None)
//         });
//     executor.advance_clock(editor::CODE_ACTIONS_DEBOUNCE_TIMEOUT * 2);
//     requests.next().await;

//     // Move cursor to a location that contains code actions.
//     editor_b.update(cx_b, |editor, cx| {
//         editor.change_selections(None, cx, |s| {
//             s.select_ranges([Point::new(1, 31)..Point::new(1, 31)])
//         });
//         cx.focus(&editor_b);
//     });

//     let mut requests = fake_language_server
//         .handle_request::<lsp::request::CodeActionRequest, _, _>(|params, _| async move {
//             assert_eq!(
//                 params.text_document.uri,
//                 lsp::Url::from_file_path("/a/main.rs").unwrap(),
//             );
//             assert_eq!(params.range.start, lsp::Position::new(1, 31));
//             assert_eq!(params.range.end, lsp::Position::new(1, 31));

//             Ok(Some(vec![lsp::CodeActionOrCommand::CodeAction(
//                 lsp::CodeAction {
//                     title: "Inline into all callers".to_string(),
//                     edit: Some(lsp::WorkspaceEdit {
//                         changes: Some(
//                             [
//                                 (
//                                     lsp::Url::from_file_path("/a/main.rs").unwrap(),
//                                     vec![lsp::TextEdit::new(
//                                         lsp::Range::new(
//                                             lsp::Position::new(1, 22),
//                                             lsp::Position::new(1, 34),
//                                         ),
//                                         "4".to_string(),
//                                     )],
//                                 ),
//                                 (
//                                     lsp::Url::from_file_path("/a/other.rs").unwrap(),
//                                     vec![lsp::TextEdit::new(
//                                         lsp::Range::new(
//                                             lsp::Position::new(0, 0),
//                                             lsp::Position::new(0, 27),
//                                         ),
//                                         "".to_string(),
//                                     )],
//                                 ),
//                             ]
//                             .into_iter()
//                             .collect(),
//                         ),
//                         ..Default::default()
//                     }),
//                     data: Some(json!({
//                         "codeActionParams": {
//                             "range": {
//                                 "start": {"line": 1, "column": 31},
//                                 "end": {"line": 1, "column": 31},
//                             }
//                         }
//                     })),
//                     ..Default::default()
//                 },
//             )]))
//         });
//     executor.advance_clock(editor::CODE_ACTIONS_DEBOUNCE_TIMEOUT * 2);
//     requests.next().await;

//     // Toggle code actions and wait for them to display.
//     editor_b.update(cx_b, |editor, cx| {
//         editor.toggle_code_actions(
//             &ToggleCodeActions {
//                 deployed_from_indicator: false,
//             },
//             cx,
//         );
//     });
//     cx_a.foreground().run_until_parked();

//     editor_b.read_with(cx_b, |editor, _| assert!(editor.context_menu_visible()));

//     fake_language_server.remove_request_handler::<lsp::request::CodeActionRequest>();

//     // Confirming the code action will trigger a resolve request.
//     let confirm_action = workspace_b
//         .update(cx_b, |workspace, cx| {
//             Editor::confirm_code_action(workspace, &ConfirmCodeAction { item_ix: Some(0) }, cx)
//         })
//         .unwrap();
//     fake_language_server.handle_request::<lsp::request::CodeActionResolveRequest, _, _>(
//         |_, _| async move {
//             Ok(lsp::CodeAction {
//                 title: "Inline into all callers".to_string(),
//                 edit: Some(lsp::WorkspaceEdit {
//                     changes: Some(
//                         [
//                             (
//                                 lsp::Url::from_file_path("/a/main.rs").unwrap(),
//                                 vec![lsp::TextEdit::new(
//                                     lsp::Range::new(
//                                         lsp::Position::new(1, 22),
//                                         lsp::Position::new(1, 34),
//                                     ),
//                                     "4".to_string(),
//                                 )],
//                             ),
//                             (
//                                 lsp::Url::from_file_path("/a/other.rs").unwrap(),
//                                 vec![lsp::TextEdit::new(
//                                     lsp::Range::new(
//                                         lsp::Position::new(0, 0),
//                                         lsp::Position::new(0, 27),
//                                     ),
//                                     "".to_string(),
//                                 )],
//                             ),
//                         ]
//                         .into_iter()
//                         .collect(),
//                     ),
//                     ..Default::default()
//                 }),
//                 ..Default::default()
//             })
//         },
//     );

//     // After the action is confirmed, an editor containing both modified files is opened.
//     confirm_action.await.unwrap();

//     let code_action_editor = workspace_b.read_with(cx_b, |workspace, cx| {
//         workspace
//             .active_item(cx)
//             .unwrap()
//             .downcast::<Editor>()
//             .unwrap()
//     });
//     code_action_editor.update(cx_b, |editor, cx| {
//         assert_eq!(editor.text(cx), "mod other;\nfn main() { let foo = 4; }\n");
//         editor.undo(&Undo, cx);
//         assert_eq!(
//             editor.text(cx),
//             "mod other;\nfn main() { let foo = other::foo(); }\npub fn foo() -> usize { 4 }"
//         );
//         editor.redo(&Redo, cx);
//         assert_eq!(editor.text(cx), "mod other;\nfn main() { let foo = 4; }\n");
//     });
// }

//todo!(editor)
// #[gpui::test(iterations = 10)]
// async fn test_collaborating_with_renames(
//     executor: BackgroundExecutor,
//     cx_a: &mut TestAppContext,
//     cx_b: &mut TestAppContext,
// ) {
//     let mut server = TestServer::start(&executor).await;
//     let client_a = server.create_client(cx_a, "user_a").await;
//     let client_b = server.create_client(cx_b, "user_b").await;
//     server
//         .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
//         .await;
//     let active_call_a = cx_a.read(ActiveCall::global);

//     cx_b.update(editor::init);

//     // Set up a fake language server.
//     let mut language = Language::new(
//         LanguageConfig {
//             name: "Rust".into(),
//             path_suffixes: vec!["rs".to_string()],
//             ..Default::default()
//         },
//         Some(tree_sitter_rust::language()),
//     );
//     let mut fake_language_servers = language
//         .set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
//             capabilities: lsp::ServerCapabilities {
//                 rename_provider: Some(lsp::OneOf::Right(lsp::RenameOptions {
//                     prepare_provider: Some(true),
//                     work_done_progress_options: Default::default(),
//                 })),
//                 ..Default::default()
//             },
//             ..Default::default()
//         }))
//         .await;
//     client_a.language_registry().add(Arc::new(language));

//     client_a
//         .fs()
//         .insert_tree(
//             "/dir",
//             json!({
//                 "one.rs": "const ONE: usize = 1;",
//                 "two.rs": "const TWO: usize = one::ONE + one::ONE;"
//             }),
//         )
//         .await;
//     let (project_a, worktree_id) = client_a.build_local_project("/dir", cx_a).await;
//     let project_id = active_call_a
//         .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
//         .await
//         .unwrap();
//     let project_b = client_b.build_remote_project(project_id, cx_b).await;

//     let window_b =
//         cx_b.add_window(|cx| Workspace::new(0, project_b.clone(), client_b.app_state.clone(), cx));
//     let workspace_b = window_b.root(cx_b);
//     let editor_b = workspace_b
//         .update(cx_b, |workspace, cx| {
//             workspace.open_path((worktree_id, "one.rs"), None, true, cx)
//         })
//         .await
//         .unwrap()
//         .downcast::<Editor>()
//         .unwrap();
//     let fake_language_server = fake_language_servers.next().await.unwrap();

//     // Move cursor to a location that can be renamed.
//     let prepare_rename = editor_b.update(cx_b, |editor, cx| {
//         editor.change_selections(None, cx, |s| s.select_ranges([7..7]));
//         editor.rename(&Rename, cx).unwrap()
//     });

//     fake_language_server
//         .handle_request::<lsp::request::PrepareRenameRequest, _, _>(|params, _| async move {
//             assert_eq!(params.text_document.uri.as_str(), "file:///dir/one.rs");
//             assert_eq!(params.position, lsp::Position::new(0, 7));
//             Ok(Some(lsp::PrepareRenameResponse::Range(lsp::Range::new(
//                 lsp::Position::new(0, 6),
//                 lsp::Position::new(0, 9),
//             ))))
//         })
//         .next()
//         .await
//         .unwrap();
//     prepare_rename.await.unwrap();
//     editor_b.update(cx_b, |editor, cx| {
//         use editor::ToOffset;
//         let rename = editor.pending_rename().unwrap();
//         let buffer = editor.buffer().read(cx).snapshot(cx);
//         assert_eq!(
//             rename.range.start.to_offset(&buffer)..rename.range.end.to_offset(&buffer),
//             6..9
//         );
//         rename.editor.update(cx, |rename_editor, cx| {
//             rename_editor.buffer().update(cx, |rename_buffer, cx| {
//                 rename_buffer.edit([(0..3, "THREE")], None, cx);
//             });
//         });
//     });

//     let confirm_rename = workspace_b.update(cx_b, |workspace, cx| {
//         Editor::confirm_rename(workspace, &ConfirmRename, cx).unwrap()
//     });
//     fake_language_server
//         .handle_request::<lsp::request::Rename, _, _>(|params, _| async move {
//             assert_eq!(
//                 params.text_document_position.text_document.uri.as_str(),
//                 "file:///dir/one.rs"
//             );
//             assert_eq!(
//                 params.text_document_position.position,
//                 lsp::Position::new(0, 6)
//             );
//             assert_eq!(params.new_name, "THREE");
//             Ok(Some(lsp::WorkspaceEdit {
//                 changes: Some(
//                     [
//                         (
//                             lsp::Url::from_file_path("/dir/one.rs").unwrap(),
//                             vec![lsp::TextEdit::new(
//                                 lsp::Range::new(lsp::Position::new(0, 6), lsp::Position::new(0, 9)),
//                                 "THREE".to_string(),
//                             )],
//                         ),
//                         (
//                             lsp::Url::from_file_path("/dir/two.rs").unwrap(),
//                             vec![
//                                 lsp::TextEdit::new(
//                                     lsp::Range::new(
//                                         lsp::Position::new(0, 24),
//                                         lsp::Position::new(0, 27),
//                                     ),
//                                     "THREE".to_string(),
//                                 ),
//                                 lsp::TextEdit::new(
//                                     lsp::Range::new(
//                                         lsp::Position::new(0, 35),
//                                         lsp::Position::new(0, 38),
//                                     ),
//                                     "THREE".to_string(),
//                                 ),
//                             ],
//                         ),
//                     ]
//                     .into_iter()
//                     .collect(),
//                 ),
//                 ..Default::default()
//             }))
//         })
//         .next()
//         .await
//         .unwrap();
//     confirm_rename.await.unwrap();

//     let rename_editor = workspace_b.read_with(cx_b, |workspace, cx| {
//         workspace
//             .active_item(cx)
//             .unwrap()
//             .downcast::<Editor>()
//             .unwrap()
//     });
//     rename_editor.update(cx_b, |editor, cx| {
//         assert_eq!(
//             editor.text(cx),
//             "const THREE: usize = 1;\nconst TWO: usize = one::THREE + one::THREE;"
//         );
//         editor.undo(&Undo, cx);
//         assert_eq!(
//             editor.text(cx),
//             "const ONE: usize = 1;\nconst TWO: usize = one::ONE + one::ONE;"
//         );
//         editor.redo(&Redo, cx);
//         assert_eq!(
//             editor.text(cx),
//             "const THREE: usize = 1;\nconst TWO: usize = one::THREE + one::THREE;"
//         );
//     });

//     // Ensure temporary rename edits cannot be undone/redone.
//     editor_b.update(cx_b, |editor, cx| {
//         editor.undo(&Undo, cx);
//         assert_eq!(editor.text(cx), "const ONE: usize = 1;");
//         editor.undo(&Undo, cx);
//         assert_eq!(editor.text(cx), "const ONE: usize = 1;");
//         editor.redo(&Redo, cx);
//         assert_eq!(editor.text(cx), "const THREE: usize = 1;");
//     })
// }

//todo!(editor)
// #[gpui::test(iterations = 10)]
// async fn test_language_server_statuses(
//     executor: BackgroundExecutor,
//     cx_a: &mut TestAppContext,
//     cx_b: &mut TestAppContext,
// ) {
//     let mut server = TestServer::start(&executor).await;
//     let client_a = server.create_client(cx_a, "user_a").await;
//     let client_b = server.create_client(cx_b, "user_b").await;
//     server
//         .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
//         .await;
//     let active_call_a = cx_a.read(ActiveCall::global);

//     cx_b.update(editor::init);

//     // Set up a fake language server.
//     let mut language = Language::new(
//         LanguageConfig {
//             name: "Rust".into(),
//             path_suffixes: vec!["rs".to_string()],
//             ..Default::default()
//         },
//         Some(tree_sitter_rust::language()),
//     );
//     let mut fake_language_servers = language
//         .set_fake_lsp_adapter(Arc::new(FakeLspAdapter {
//             name: "the-language-server",
//             ..Default::default()
//         }))
//         .await;
//     client_a.language_registry().add(Arc::new(language));

//     client_a
//         .fs()
//         .insert_tree(
//             "/dir",
//             json!({
//                 "main.rs": "const ONE: usize = 1;",
//             }),
//         )
//         .await;
//     let (project_a, worktree_id) = client_a.build_local_project("/dir", cx_a).await;

//     let _buffer_a = project_a
//         .update(cx_a, |p, cx| p.open_buffer((worktree_id, "main.rs"), cx))
//         .await
//         .unwrap();

//     let fake_language_server = fake_language_servers.next().await.unwrap();
//     fake_language_server.start_progress("the-token").await;
//     fake_language_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
//         token: lsp::NumberOrString::String("the-token".to_string()),
//         value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::Report(
//             lsp::WorkDoneProgressReport {
//                 message: Some("the-message".to_string()),
//                 ..Default::default()
//             },
//         )),
//     });
//     executor.run_until_parked();

//     project_a.read_with(cx_a, |project, _| {
//         let status = project.language_server_statuses().next().unwrap();
//         assert_eq!(status.name, "the-language-server");
//         assert_eq!(status.pending_work.len(), 1);
//         assert_eq!(
//             status.pending_work["the-token"].message.as_ref().unwrap(),
//             "the-message"
//         );
//     });

//     let project_id = active_call_a
//         .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
//         .await
//         .unwrap();
//     executor.run_until_parked();
//     let project_b = client_b.build_remote_project(project_id, cx_b).await;

//     project_b.read_with(cx_b, |project, _| {
//         let status = project.language_server_statuses().next().unwrap();
//         assert_eq!(status.name, "the-language-server");
//     });

//     fake_language_server.notify::<lsp::notification::Progress>(lsp::ProgressParams {
//         token: lsp::NumberOrString::String("the-token".to_string()),
//         value: lsp::ProgressParamsValue::WorkDone(lsp::WorkDoneProgress::Report(
//             lsp::WorkDoneProgressReport {
//                 message: Some("the-message-2".to_string()),
//                 ..Default::default()
//             },
//         )),
//     });
//     executor.run_until_parked();

//     project_a.read_with(cx_a, |project, _| {
//         let status = project.language_server_statuses().next().unwrap();
//         assert_eq!(status.name, "the-language-server");
//         assert_eq!(status.pending_work.len(), 1);
//         assert_eq!(
//             status.pending_work["the-token"].message.as_ref().unwrap(),
//             "the-message-2"
//         );
//     });

//     project_b.read_with(cx_b, |project, _| {
//         let status = project.language_server_statuses().next().unwrap();
//         assert_eq!(status.name, "the-language-server");
//         assert_eq!(status.pending_work.len(), 1);
//         assert_eq!(
//             status.pending_work["the-token"].message.as_ref().unwrap(),
//             "the-message-2"
//         );
//     });
// }

// #[gpui::test(iterations = 10)]
// async fn test_share_project(
//     executor: BackgroundExecutor,
//     cx_a: &mut TestAppContext,
//     cx_b: &mut TestAppContext,
//     cx_c: &mut TestAppContext,
// ) {
//     let window_b = cx_b.add_window(|_| EmptyView);
//     let mut server = TestServer::start(&executor).await;
//     let client_a = server.create_client(cx_a, "user_a").await;
//     let client_b = server.create_client(cx_b, "user_b").await;
//     let client_c = server.create_client(cx_c, "user_c").await;
//     server
//         .make_contacts(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
//         .await;
//     let active_call_a = cx_a.read(ActiveCall::global);
//     let active_call_b = cx_b.read(ActiveCall::global);
//     let active_call_c = cx_c.read(ActiveCall::global);

//     client_a
//         .fs()
//         .insert_tree(
//             "/a",
//             json!({
//                 ".gitignore": "ignored-dir",
//                 "a.txt": "a-contents",
//                 "b.txt": "b-contents",
//                 "ignored-dir": {
//                     "c.txt": "",
//                     "d.txt": "",
//                 }
//             }),
//         )
//         .await;

//     // Invite client B to collaborate on a project
//     let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
//     active_call_a
//         .update(cx_a, |call, cx| {
//             call.invite(client_b.user_id().unwrap(), Some(project_a.clone()), cx)
//         })
//         .await
//         .unwrap();

//     // Join that project as client B

//     let incoming_call_b = active_call_b.read_with(cx_b, |call, _| call.incoming());
//     executor.run_until_parked();
//     let call = incoming_call_b.borrow().clone().unwrap();
//     assert_eq!(call.calling_user.github_login, "user_a");
//     let initial_project = call.initial_project.unwrap();
//     active_call_b
//         .update(cx_b, |call, cx| call.accept_incoming(cx))
//         .await
//         .unwrap();
//     let client_b_peer_id = client_b.peer_id().unwrap();
//     let project_b = client_b
//         .build_remote_project(initial_project.id, cx_b)
//         .await;

//     let replica_id_b = project_b.read_with(cx_b, |project, _| project.replica_id());

//     executor.run_until_parked();

//     project_a.read_with(cx_a, |project, _| {
//         let client_b_collaborator = project.collaborators().get(&client_b_peer_id).unwrap();
//         assert_eq!(client_b_collaborator.replica_id, replica_id_b);
//     });

//     project_b.read_with(cx_b, |project, cx| {
//         let worktree = project.worktrees().next().unwrap().read(cx);
//         assert_eq!(
//             worktree.paths().map(AsRef::as_ref).collect::<Vec<_>>(),
//             [
//                 Path::new(".gitignore"),
//                 Path::new("a.txt"),
//                 Path::new("b.txt"),
//                 Path::new("ignored-dir"),
//             ]
//         );
//     });

//     project_b
//         .update(cx_b, |project, cx| {
//             let worktree = project.worktrees().next().unwrap();
//             let entry = worktree.read(cx).entry_for_path("ignored-dir").unwrap();
//             project.expand_entry(worktree_id, entry.id, cx).unwrap()
//         })
//         .await
//         .unwrap();

//     project_b.read_with(cx_b, |project, cx| {
//         let worktree = project.worktrees().next().unwrap().read(cx);
//         assert_eq!(
//             worktree.paths().map(AsRef::as_ref).collect::<Vec<_>>(),
//             [
//                 Path::new(".gitignore"),
//                 Path::new("a.txt"),
//                 Path::new("b.txt"),
//                 Path::new("ignored-dir"),
//                 Path::new("ignored-dir/c.txt"),
//                 Path::new("ignored-dir/d.txt"),
//             ]
//         );
//     });

//     // Open the same file as client B and client A.
//     let buffer_b = project_b
//         .update(cx_b, |p, cx| p.open_buffer((worktree_id, "b.txt"), cx))
//         .await
//         .unwrap();

//     buffer_b.read_with(cx_b, |buf, _| assert_eq!(buf.text(), "b-contents"));

//     project_a.read_with(cx_a, |project, cx| {
//         assert!(project.has_open_buffer((worktree_id, "b.txt"), cx))
//     });
//     let buffer_a = project_a
//         .update(cx_a, |p, cx| p.open_buffer((worktree_id, "b.txt"), cx))
//         .await
//         .unwrap();

//     let editor_b = window_b.add_view(cx_b, |cx| Editor::for_buffer(buffer_b, None, cx));

//     // Client A sees client B's selection
//     executor.run_until_parked();

//     buffer_a.read_with(cx_a, |buffer, _| {
//         buffer
//             .snapshot()
//             .remote_selections_in_range(Anchor::MIN..Anchor::MAX)
//             .count()
//             == 1
//     });

//     // Edit the buffer as client B and see that edit as client A.
//     editor_b.update(cx_b, |editor, cx| editor.handle_input("ok, ", cx));
//     executor.run_until_parked();

//     buffer_a.read_with(cx_a, |buffer, _| {
//         assert_eq!(buffer.text(), "ok, b-contents")
//     });

//     // Client B can invite client C on a project shared by client A.
//     active_call_b
//         .update(cx_b, |call, cx| {
//             call.invite(client_c.user_id().unwrap(), Some(project_b.clone()), cx)
//         })
//         .await
//         .unwrap();

//     let incoming_call_c = active_call_c.read_with(cx_c, |call, _| call.incoming());
//     executor.run_until_parked();
//     let call = incoming_call_c.borrow().clone().unwrap();
//     assert_eq!(call.calling_user.github_login, "user_b");
//     let initial_project = call.initial_project.unwrap();
//     active_call_c
//         .update(cx_c, |call, cx| call.accept_incoming(cx))
//         .await
//         .unwrap();
//     let _project_c = client_c
//         .build_remote_project(initial_project.id, cx_c)
//         .await;

//     // Client B closes the editor, and client A sees client B's selections removed.
//     cx_b.update(move |_| drop(editor_b));
//     executor.run_until_parked();

//     buffer_a.read_with(cx_a, |buffer, _| {
//         buffer
//             .snapshot()
//             .remote_selections_in_range(Anchor::MIN..Anchor::MAX)
//             .count()
//             == 0
//     });
// }
