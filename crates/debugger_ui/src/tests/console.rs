use crate::{
    tests::{active_debug_session_panel, start_debug_session},
    *,
};
use dap::requests::StackTrace;
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use project::{FakeFs, Project};
use serde_json::json;
use tests::{init_test, init_test_workspace};
use util::path;

#[gpui::test]
async fn test_handle_output_event(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    workspace
        .update(cx, |workspace, window, cx| {
            workspace.focus_panel::<DebugPanel>(window, cx);
        })
        .unwrap();

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client.on_request::<StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: Vec::default(),
            total_frames: None,
        })
    });

    client
        .fake_event(dap::messages::Events::Output(dap::OutputEvent {
            category: None,
            output: "First console output line before thread stopped!".to_string(),
            data: None,
            variables_reference: None,
            source: None,
            line: None,
            column: None,
            group: None,
            location_reference: None,
        }))
        .await;

    client
        .fake_event(dap::messages::Events::Output(dap::OutputEvent {
            category: Some(dap::OutputEventCategory::Stdout),
            output: "First output line before thread stopped!".to_string(),
            data: None,
            variables_reference: None,
            source: None,
            line: None,
            column: None,
            group: None,
            location_reference: None,
        }))
        .await;

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Pause,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    let running_state =
        active_debug_session_panel(workspace, cx).update_in(cx, |item, window, cx| {
            cx.focus_self(window);
            item.running_state().clone()
        });

    cx.run_until_parked();

    // assert we have output from before the thread stopped
    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_debug_session_panel = debug_panel
                .update(cx, |this, _| this.active_session())
                .unwrap();

            assert_eq!(
                "First console output line before thread stopped!\nFirst output line before thread stopped!\n",
                active_debug_session_panel.read(cx).running_state().read(cx).console().read(cx).editor().read(cx).text(cx).as_str()
            );
        })
        .unwrap();

    client
        .fake_event(dap::messages::Events::Output(dap::OutputEvent {
            category: Some(dap::OutputEventCategory::Stdout),
            output: "Second output line after thread stopped!".to_string(),
            data: None,
            variables_reference: None,
            source: None,
            line: None,
            column: None,
            group: None,
            location_reference: None,
        }))
        .await;

    client
        .fake_event(dap::messages::Events::Output(dap::OutputEvent {
            category: Some(dap::OutputEventCategory::Console),
            output: "Second console output line after thread stopped!".to_string(),
            data: None,
            variables_reference: None,
            source: None,
            line: None,
            column: None,
            group: None,
            location_reference: None,
        }))
        .await;

    cx.run_until_parked();
    running_state.update(cx, |_, cx| {
        cx.refresh_windows();
    });
    cx.run_until_parked();

    // assert we have output from before and after the thread stopped
    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_session_panel = debug_panel
                .update(cx, |this, _| this.active_session())
                .unwrap();

            assert_eq!(
                "First console output line before thread stopped!\nFirst output line before thread stopped!\nSecond output line after thread stopped!\nSecond console output line after thread stopped!\n",
                active_session_panel.read(cx).running_state().read(cx).console().read(cx).editor().read(cx).text(cx).as_str()
            );
        })
        .unwrap();
}

// #[gpui::test]
// async fn test_grouped_output(executor: BackgroundExecutor, cx: &mut TestAppContext) {
//     init_test(cx);

//     let fs = FakeFs::new(executor.clone());

//     fs.insert_tree(
//         "/project",
//         json!({
//             "main.rs": "First line\nSecond line\nThird line\nFourth line",
//         }),
//     )
//     .await;

//     let project = Project::test(fs, ["/project".as_ref()], cx).await;
//     let workspace = init_test_workspace(&project, cx).await;
//     let cx = &mut VisualTestContext::from_window(*workspace, cx);

//     let task = project.update(cx, |project, cx| {
//         project.start_debug_session(
//             dap::test_config(dap::DebugRequestType::Launch, None, None),
//             cx,
//         )
//     });

//     let session = task.await.unwrap();
//     let client = session.update(cx, |session, _| session.adapter_client().unwrap());

//     client
//         .on_request::<StackTrace, _>(move |_, _| {
//             Ok(dap::StackTraceResponse {
//                 stack_frames: Vec::default(),
//                 total_frames: None,
//             })
//         })
//         .await;

//     client
//         .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
//             reason: dap::StoppedEventReason::Pause,
//             description: None,
//             thread_id: Some(1),
//             preserve_focus_hint: None,
//             text: None,
//             all_threads_stopped: None,
//             hit_breakpoint_ids: None,
//         }))
//         .await;

//     client
//         .fake_event(dap::messages::Events::Output(dap::OutputEvent {
//             category: None,
//             output: "First line".to_string(),
//             data: None,
//             variables_reference: None,
//             source: None,
//             line: None,
//             column: None,
//             group: None,
//             location_reference: None,
//         }))
//         .await;

//     client
//         .fake_event(dap::messages::Events::Output(dap::OutputEvent {
//             category: Some(dap::OutputEventCategory::Stdout),
//             output: "First group".to_string(),
//             data: None,
//             variables_reference: None,
//             source: None,
//             line: None,
//             column: None,
//             group: Some(dap::OutputEventGroup::Start),
//             location_reference: None,
//         }))
//         .await;

//     client
//         .fake_event(dap::messages::Events::Output(dap::OutputEvent {
//             category: Some(dap::OutputEventCategory::Stdout),
//             output: "First item in group 1".to_string(),
//             data: None,
//             variables_reference: None,
//             source: None,
//             line: None,
//             column: None,
//             group: None,
//             location_reference: None,
//         }))
//         .await;

//     client
//         .fake_event(dap::messages::Events::Output(dap::OutputEvent {
//             category: Some(dap::OutputEventCategory::Stdout),
//             output: "Second item in group 1".to_string(),
//             data: None,
//             variables_reference: None,
//             source: None,
//             line: None,
//             column: None,
//             group: None,
//             location_reference: None,
//         }))
//         .await;

//     client
//         .fake_event(dap::messages::Events::Output(dap::OutputEvent {
//             category: Some(dap::OutputEventCategory::Stdout),
//             output: "Second group".to_string(),
//             data: None,
//             variables_reference: None,
//             source: None,
//             line: None,
//             column: None,
//             group: Some(dap::OutputEventGroup::Start),
//             location_reference: None,
//         }))
//         .await;

//     client
//         .fake_event(dap::messages::Events::Output(dap::OutputEvent {
//             category: Some(dap::OutputEventCategory::Stdout),
//             output: "First item in group 2".to_string(),
//             data: None,
//             variables_reference: None,
//             source: None,
//             line: None,
//             column: None,
//             group: None,
//             location_reference: None,
//         }))
//         .await;

//     client
//         .fake_event(dap::messages::Events::Output(dap::OutputEvent {
//             category: Some(dap::OutputEventCategory::Stdout),
//             output: "Second item in group 2".to_string(),
//             data: None,
//             variables_reference: None,
//             source: None,
//             line: None,
//             column: None,
//             group: None,
//             location_reference: None,
//         }))
//         .await;

//     client
//         .fake_event(dap::messages::Events::Output(dap::OutputEvent {
//             category: Some(dap::OutputEventCategory::Stdout),
//             output: "End group 2".to_string(),
//             data: None,
//             variables_reference: None,
//             source: None,
//             line: None,
//             column: None,
//             group: Some(dap::OutputEventGroup::End),
//             location_reference: None,
//         }))
//         .await;

//     client
//         .fake_event(dap::messages::Events::Output(dap::OutputEvent {
//             category: Some(dap::OutputEventCategory::Stdout),
//             output: "Third group".to_string(),
//             data: None,
//             variables_reference: None,
//             source: None,
//             line: None,
//             column: None,
//             group: Some(dap::OutputEventGroup::StartCollapsed),
//             location_reference: None,
//         }))
//         .await;

//     client
//         .fake_event(dap::messages::Events::Output(dap::OutputEvent {
//             category: Some(dap::OutputEventCategory::Stdout),
//             output: "First item in group 3".to_string(),
//             data: None,
//             variables_reference: None,
//             source: None,
//             line: None,
//             column: None,
//             group: None,
//             location_reference: None,
//         }))
//         .await;

//     client
//         .fake_event(dap::messages::Events::Output(dap::OutputEvent {
//             category: Some(dap::OutputEventCategory::Stdout),
//             output: "Second item in group 3".to_string(),
//             data: None,
//             variables_reference: None,
//             source: None,
//             line: None,
//             column: None,
//             group: None,
//             location_reference: None,
//         }))
//         .await;

//     client
//         .fake_event(dap::messages::Events::Output(dap::OutputEvent {
//             category: Some(dap::OutputEventCategory::Stdout),
//             output: "End group 3".to_string(),
//             data: None,
//             variables_reference: None,
//             source: None,
//             line: None,
//             column: None,
//             group: Some(dap::OutputEventGroup::End),
//             location_reference: None,
//         }))
//         .await;

//     client
//         .fake_event(dap::messages::Events::Output(dap::OutputEvent {
//             category: Some(dap::OutputEventCategory::Stdout),
//             output: "Third item in group 1".to_string(),
//             data: None,
//             variables_reference: None,
//             source: None,
//             line: None,
//             column: None,
//             group: None,
//             location_reference: None,
//         }))
//         .await;

//     client
//         .fake_event(dap::messages::Events::Output(dap::OutputEvent {
//             category: Some(dap::OutputEventCategory::Stdout),
//             output: "Second item".to_string(),
//             data: None,
//             variables_reference: None,
//             source: None,
//             line: None,
//             column: None,
//             group: Some(dap::OutputEventGroup::End),
//             location_reference: None,
//         }))
//         .await;

//     cx.run_until_parked();

//     active_debug_session_panel(workspace, cx).update(cx, |debug_panel_item, cx| {
//         debug_panel_item
//             .mode()
//             .as_running()
//             .unwrap()
//             .update(cx, |running_state, cx| {
//                 running_state.console().update(cx, |console, cx| {
//                     console.editor().update(cx, |editor, cx| {
//                         pretty_assertions::assert_eq!(
//                             "
//                         First line
//                         First group
//                             First item in group 1
//                             Second item in group 1
//                             Second group
//                                 First item in group 2
//                                 Second item in group 2
//                             End group 2
//                         ⋯    End group 3
//                             Third item in group 1
//                         Second item
//                     "
//                             .unindent(),
//                             editor.display_text(cx)
//                         );
//                     })
//                 });
//             });
//     });

//     let shutdown_session = project.update(cx, |project, cx| {
//         project.dap_store().update(cx, |dap_store, cx| {
//             dap_store.shutdown_session(session.read(cx).session_id(), cx)
//         })
//     });

//     shutdown_session.await.unwrap();
// }

// todo(debugger): enable this again
// #[gpui::test]
// async fn test_evaluate_expression(executor: BackgroundExecutor, cx: &mut TestAppContext) {
//     init_test(cx);

//     const NEW_VALUE: &str = "{nested1: \"Nested 1 updated\", nested2: \"Nested 2 updated\"}";

//     let called_evaluate = Arc::new(AtomicBool::new(false));

//     let fs = FakeFs::new(executor.clone());

//     let test_file_content = r#"
//         const variable1 = {
//             nested1: "Nested 1",
//             nested2: "Nested 2",
//         };
//         const variable2 = "Value 2";
//         const variable3 = "Value 3";
//     "#
//     .unindent();

//     fs.insert_tree(
//         "/project",
//         json!({
//            "src": {
//                "test.js": test_file_content,
//            }
//         }),
//     )
//     .await;

//     let project = Project::test(fs, ["/project".as_ref()], cx).await;
//     let workspace = init_test_workspace(&project, cx).await;
//     let cx = &mut VisualTestContext::from_window(*workspace, cx);

//     let task = project.update(cx, |project, cx| {
//         project.start_debug_session(dap::test_config(None), cx)
//     });

//     let session = task.await.unwrap();
//     let client = session.update(cx, |session, _| session.adapter_client().unwrap());

//     client
//         .on_request::<Threads, _>(move |_, _| {
//             Ok(dap::ThreadsResponse {
//                 threads: vec![dap::Thread {
//                     id: 1,
//                     name: "Thread 1".into(),
//                 }],
//             })
//         })
//         .await;

//     let stack_frames = vec![StackFrame {
//         id: 1,
//         name: "Stack Frame 1".into(),
//         source: Some(dap::Source {
//             name: Some("test.js".into()),
//             path: Some("/project/src/test.js".into()),
//             source_reference: None,
//             presentation_hint: None,
//             origin: None,
//             sources: None,
//             adapter_data: None,
//             checksums: None,
//         }),
//         line: 3,
//         column: 1,
//         end_line: None,
//         end_column: None,
//         can_restart: None,
//         instruction_pointer_reference: None,
//         module_id: None,
//         presentation_hint: None,
//     }];

//     client
//         .on_request::<StackTrace, _>({
//             let stack_frames = Arc::new(stack_frames.clone());
//             move |_, args| {
//                 assert_eq!(1, args.thread_id);

//                 Ok(dap::StackTraceResponse {
//                     stack_frames: (*stack_frames).clone(),
//                     total_frames: None,
//                 })
//             }
//         })
//         .await;

//     let scopes = vec![
//         Scope {
//             name: "Scope 1".into(),
//             presentation_hint: None,
//             variables_reference: 2,
//             named_variables: None,
//             indexed_variables: None,
//             expensive: false,
//             source: None,
//             line: None,
//             column: None,
//             end_line: None,
//             end_column: None,
//         },
//         Scope {
//             name: "Scope 2".into(),
//             presentation_hint: None,
//             variables_reference: 4,
//             named_variables: None,
//             indexed_variables: None,
//             expensive: false,
//             source: None,
//             line: None,
//             column: None,
//             end_line: None,
//             end_column: None,
//         },
//     ];

//     client
//         .on_request::<Scopes, _>({
//             let scopes = Arc::new(scopes.clone());
//             move |_, args| {
//                 assert_eq!(1, args.frame_id);

//                 Ok(dap::ScopesResponse {
//                     scopes: (*scopes).clone(),
//                 })
//             }
//         })
//         .await;

//     let scope1_variables = Arc::new(Mutex::new(vec![
//         Variable {
//             name: "variable1".into(),
//             value: "{nested1: \"Nested 1\", nested2: \"Nested 2\"}".into(),
//             type_: None,
//             presentation_hint: None,
//             evaluate_name: None,
//             variables_reference: 3,
//             named_variables: None,
//             indexed_variables: None,
//             memory_reference: None,
//             declaration_location_reference: None,
//             value_location_reference: None,
//         },
//         Variable {
//             name: "variable2".into(),
//             value: "Value 2".into(),
//             type_: None,
//             presentation_hint: None,
//             evaluate_name: None,
//             variables_reference: 0,
//             named_variables: None,
//             indexed_variables: None,
//             memory_reference: None,
//             declaration_location_reference: None,
//             value_location_reference: None,
//         },
//     ]));

//     let nested_variables = vec![
//         Variable {
//             name: "nested1".into(),
//             value: "Nested 1".into(),
//             type_: None,
//             presentation_hint: None,
//             evaluate_name: None,
//             variables_reference: 0,
//             named_variables: None,
//             indexed_variables: None,
//             memory_reference: None,
//             declaration_location_reference: None,
//             value_location_reference: None,
//         },
//         Variable {
//             name: "nested2".into(),
//             value: "Nested 2".into(),
//             type_: None,
//             presentation_hint: None,
//             evaluate_name: None,
//             variables_reference: 0,
//             named_variables: None,
//             indexed_variables: None,
//             memory_reference: None,
//             declaration_location_reference: None,
//             value_location_reference: None,
//         },
//     ];

//     let scope2_variables = vec![Variable {
//         name: "variable3".into(),
//         value: "Value 3".into(),
//         type_: None,
//         presentation_hint: None,
//         evaluate_name: None,
//         variables_reference: 0,
//         named_variables: None,
//         indexed_variables: None,
//         memory_reference: None,
//         declaration_location_reference: None,
//         value_location_reference: None,
//     }];

//     client
//         .on_request::<Variables, _>({
//             let scope1_variables = scope1_variables.clone();
//             let nested_variables = Arc::new(nested_variables.clone());
//             let scope2_variables = Arc::new(scope2_variables.clone());
//             move |_, args| match args.variables_reference {
//                 4 => Ok(dap::VariablesResponse {
//                     variables: (*scope2_variables).clone(),
//                 }),
//                 3 => Ok(dap::VariablesResponse {
//                     variables: (*nested_variables).clone(),
//                 }),
//                 2 => Ok(dap::VariablesResponse {
//                     variables: scope1_variables.lock().unwrap().clone(),
//                 }),
//                 id => unreachable!("unexpected variables reference {id}"),
//             }
//         })
//         .await;

//     client
//         .on_request::<Evaluate, _>({
//             let called_evaluate = called_evaluate.clone();
//             let scope1_variables = scope1_variables.clone();
//             move |_, args| {
//                 called_evaluate.store(true, Ordering::SeqCst);

//                 assert_eq!(format!("$variable1 = {}", NEW_VALUE), args.expression);
//                 assert_eq!(Some(1), args.frame_id);
//                 assert_eq!(Some(dap::EvaluateArgumentsContext::Variables), args.context);

//                 scope1_variables.lock().unwrap()[0].value = NEW_VALUE.to_string();

//                 Ok(dap::EvaluateResponse {
//                     result: NEW_VALUE.into(),
//                     type_: None,
//                     presentation_hint: None,
//                     variables_reference: 0,
//                     named_variables: None,
//                     indexed_variables: None,
//                     memory_reference: None,
//                     value_location_reference: None,
//                 })
//             }
//         })
//         .await;

//     client
//         .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
//             reason: dap::StoppedEventReason::Pause,
//             description: None,
//             thread_id: Some(1),
//             preserve_focus_hint: None,
//             text: None,
//             all_threads_stopped: None,
//             hit_breakpoint_ids: None,
//         }))
//         .await;

//     cx.run_until_parked();

//     // toggle nested variables for scope 1
//     active_debug_session_panel(workspace, cx).update(cx, |debug_panel_item, cx| {
//         debug_panel_item
//             .mode()
//             .as_running()
//             .unwrap()
//             .update(cx, |running_state, cx| {
//                 running_state
//                     .variable_list()
//                     .update(cx, |variable_list, cx| {
//                         variable_list.toggle_variable(
//                             &VariablePath {
//                                 indices: Arc::from([scopes[0].variables_reference]),
//                             },
//                             cx,
//                         );
//                     });
//             });
//     });

//     cx.run_until_parked();

//     active_debug_session_panel(workspace, cx).update_in(cx, |debug_panel_item, window, cx| {
//         debug_panel_item
//             .mode()
//             .as_running()
//             .unwrap()
//             .update(cx, |running_state, cx| {
//                 running_state.console().update(cx, |console, item_cx| {
//                     console
//                         .query_bar()
//                         .update(item_cx, |query_bar, console_cx| {
//                             query_bar.set_text(
//                                 format!("$variable1 = {}", NEW_VALUE),
//                                 window,
//                                 console_cx,
//                             );
//                         });

//                     console.evaluate(&menu::Confirm, window, item_cx);
//                 });
//             });
//     });

//     cx.run_until_parked();

//     active_debug_session_panel(workspace, cx).update(cx, |debug_panel_item, cx| {
//         assert_eq!(
//             "",
//             debug_panel_item
//                 .mode()
//                 .as_running()
//                 .unwrap()
//                 .read(cx)
//                 .console()
//                 .read(cx)
//                 .query_bar()
//                 .read(cx)
//                 .text(cx)
//                 .as_str()
//         );

//         assert_eq!(
//             format!("{}\n", NEW_VALUE),
//             debug_panel_item
//                 .mode()
//                 .as_running()
//                 .unwrap()
//                 .read(cx)
//                 .console()
//                 .read(cx)
//                 .editor()
//                 .read(cx)
//                 .text(cx)
//                 .as_str()
//         );

//         debug_panel_item
//             .mode()
//             .as_running()
//             .unwrap()
//             .update(cx, |running_state, cx| {
//                 running_state
//                     .variable_list()
//                     .update(cx, |variable_list, _| {
//                         let scope1_variables = scope1_variables.lock().unwrap().clone();

//                         // scope 1
//                         // assert_eq!(
//                         //     vec![
//                         //         VariableContainer {
//                         //             container_reference: scopes[0].variables_reference,
//                         //             variable: scope1_variables[0].clone(),
//                         //             depth: 1,
//                         //         },
//                         //         VariableContainer {
//                         //             container_reference: scope1_variables[0].variables_reference,
//                         //             variable: nested_variables[0].clone(),
//                         //             depth: 2,
//                         //         },
//                         //         VariableContainer {
//                         //             container_reference: scope1_variables[0].variables_reference,
//                         //             variable: nested_variables[1].clone(),
//                         //             depth: 2,
//                         //         },
//                         //         VariableContainer {
//                         //             container_reference: scopes[0].variables_reference,
//                         //             variable: scope1_variables[1].clone(),
//                         //             depth: 1,
//                         //         },
//                         //     ],
//                         //     variable_list.variables_by_scope(1, 2).unwrap().variables()
//                         // );

//                         // scope 2
//                         // assert_eq!(
//                         //     vec![VariableContainer {
//                         //         container_reference: scopes[1].variables_reference,
//                         //         variable: scope2_variables[0].clone(),
//                         //         depth: 1,
//                         //     }],
//                         //     variable_list.variables_by_scope(1, 4).unwrap().variables()
//                         // );

//                         variable_list.assert_visual_entries(vec![
//                             "v Scope 1",
//                             "    v variable1",
//                             "       > nested1",
//                             "       > nested2",
//                             "    > variable2",
//                         ]);

//                         // assert visual entries
//                         // assert_eq!(
//                         //     vec![
//                         //         VariableListEntry::Scope(scopes[0].clone()),
//                         //         VariableListEntry::Variable {
//                         //             depth: 1,
//                         //             scope: Arc::new(scopes[0].clone()),
//                         //             has_children: true,
//                         //             variable: Arc::new(scope1_variables[0].clone()),
//                         //             container_reference: scopes[0].variables_reference,
//                         //         },
//                         //         VariableListEntry::Variable {
//                         //             depth: 2,
//                         //             scope: Arc::new(scopes[0].clone()),
//                         //             has_children: false,
//                         //             variable: Arc::new(nested_variables[0].clone()),
//                         //             container_reference: scope1_variables[0].variables_reference,
//                         //         },
//                         //         VariableListEntry::Variable {
//                         //             depth: 2,
//                         //             scope: Arc::new(scopes[0].clone()),
//                         //             has_children: false,
//                         //             variable: Arc::new(nested_variables[1].clone()),
//                         //             container_reference: scope1_variables[0].variables_reference,
//                         //         },
//                         //         VariableListEntry::Variable {
//                         //             depth: 1,
//                         //             scope: Arc::new(scopes[0].clone()),
//                         //             has_children: false,
//                         //             variable: Arc::new(scope1_variables[1].clone()),
//                         //             container_reference: scopes[0].variables_reference,
//                         //         },
//                         //         VariableListEntry::Scope(scopes[1].clone()),
//                         //     ],
//                         //     variable_list.entries().get(&1).unwrap().clone()
//                         // );
//                     });
//             });
//     });

//     assert!(
//         called_evaluate.load(std::sync::atomic::Ordering::SeqCst),
//         "Expected evaluate request to be called"
//     );

//     let shutdown_session = project.update(cx, |project, cx| {
//         project.dap_store().update(cx, |dap_store, cx| {
//             dap_store.shutdown_session(&session.read(cx).session_id(), cx)
//         })
//     });

//     shutdown_session.await.unwrap();
// }
