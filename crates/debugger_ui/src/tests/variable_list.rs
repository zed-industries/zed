use std::sync::{
    Arc,
    atomic::{AtomicBool, Ordering},
};

use crate::{
    DebugPanel,
    session::running::variable_list::{CollapseSelectedEntry, ExpandSelectedEntry},
    tests::{active_debug_session_panel, init_test, init_test_workspace, start_debug_session},
};
use collections::HashMap;
use dap::{
    Scope, StackFrame, Variable,
    requests::{Initialize, Launch, Scopes, StackTrace, Variables},
};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use menu::{SelectFirst, SelectNext, SelectPrevious};
use project::{FakeFs, Project};
use serde_json::json;
use unindent::Unindent as _;
use util::path;

/// This only tests fetching one scope and 2 variables for a single stackframe
#[gpui::test]
async fn test_basic_fetch_initial_scope_and_variables(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    let test_file_content = r#"
        const variable1 = "Value 1";
        const variable2 = "Value 2";
    "#
    .unindent();

    fs.insert_tree(
        path!("/project"),
        json!({
           "src": {
               "test.js": test_file_content,
           }
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    workspace
        .update(cx, |workspace, window, cx| {
            workspace.focus_panel::<DebugPanel>(window, cx);
        })
        .unwrap();
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client.on_request::<dap::requests::Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

    let stack_frames = vec![StackFrame {
        id: 1,
        name: "Stack Frame 1".into(),
        source: Some(dap::Source {
            name: Some("test.js".into()),
            path: Some(path!("/project/src/test.js").into()),
            source_reference: None,
            presentation_hint: None,
            origin: None,
            sources: None,
            adapter_data: None,
            checksums: None,
        }),
        line: 1,
        column: 1,
        end_line: None,
        end_column: None,
        can_restart: None,
        instruction_pointer_reference: None,
        module_id: None,
        presentation_hint: None,
    }];

    client.on_request::<StackTrace, _>({
        let stack_frames = Arc::new(stack_frames.clone());
        move |_, args| {
            assert_eq!(1, args.thread_id);

            Ok(dap::StackTraceResponse {
                stack_frames: (*stack_frames).clone(),
                total_frames: None,
            })
        }
    });

    let scopes = vec![Scope {
        name: "Scope 1".into(),
        presentation_hint: None,
        variables_reference: 2,
        named_variables: None,
        indexed_variables: None,
        expensive: false,
        source: None,
        line: None,
        column: None,
        end_line: None,
        end_column: None,
    }];

    client.on_request::<Scopes, _>({
        let scopes = Arc::new(scopes.clone());
        move |_, args| {
            assert_eq!(1, args.frame_id);

            Ok(dap::ScopesResponse {
                scopes: (*scopes).clone(),
            })
        }
    });

    let variables = vec![
        Variable {
            name: "variable1".into(),
            value: "value 1".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "variable2".into(),
            value: "value 2".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
    ];

    client.on_request::<Variables, _>({
        let variables = Arc::new(variables.clone());
        move |_, args| {
            assert_eq!(2, args.variables_reference);

            Ok(dap::VariablesResponse {
                variables: (*variables).clone(),
            })
        }
    });

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
            item.mode()
                .as_running()
                .expect("Session should be running by this point")
                .clone()
        });
    cx.run_until_parked();

    running_state.update(cx, |running_state, cx| {
        let (stack_frame_list, stack_frame_id) =
            running_state.stack_frame_list().update(cx, |list, _| {
                (list.flatten_entries(), list.selected_stack_frame_id())
            });

        assert_eq!(stack_frames, stack_frame_list);
        assert_eq!(Some(1), stack_frame_id);

        running_state
            .variable_list()
            .update(cx, |variable_list, _| {
                assert_eq!(scopes, variable_list.scopes());
                assert_eq!(
                    vec![variables[0].clone(), variables[1].clone(),],
                    variable_list.variables()
                );

                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    > variable1",
                    "    > variable2",
                ]);
            });
    });

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}

/// This tests fetching multiple scopes and variables for them with a single stackframe
#[gpui::test]
async fn test_fetch_variables_for_multiple_scopes(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    let test_file_content = r#"
        const variable1 = {
            nested1: "Nested 1",
            nested2: "Nested 2",
        };
        const variable2 = "Value 2";
        const variable3 = "Value 3";
    "#
    .unindent();

    fs.insert_tree(
        path!("/project"),
        json!({
           "src": {
               "test.js": test_file_content,
           }
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    workspace
        .update(cx, |workspace, window, cx| {
            workspace.focus_panel::<DebugPanel>(window, cx);
        })
        .unwrap();
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client.on_request::<dap::requests::Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

    client.on_request::<Initialize, _>(move |_, _| {
        Ok(dap::Capabilities {
            supports_step_back: Some(false),
            ..Default::default()
        })
    });

    client.on_request::<Launch, _>(move |_, _| Ok(()));

    let stack_frames = vec![StackFrame {
        id: 1,
        name: "Stack Frame 1".into(),
        source: Some(dap::Source {
            name: Some("test.js".into()),
            path: Some(path!("/project/src/test.js").into()),
            source_reference: None,
            presentation_hint: None,
            origin: None,
            sources: None,
            adapter_data: None,
            checksums: None,
        }),
        line: 1,
        column: 1,
        end_line: None,
        end_column: None,
        can_restart: None,
        instruction_pointer_reference: None,
        module_id: None,
        presentation_hint: None,
    }];

    client.on_request::<StackTrace, _>({
        let stack_frames = Arc::new(stack_frames.clone());
        move |_, args| {
            assert_eq!(1, args.thread_id);

            Ok(dap::StackTraceResponse {
                stack_frames: (*stack_frames).clone(),
                total_frames: None,
            })
        }
    });

    let scopes = vec![
        Scope {
            name: "Scope 1".into(),
            presentation_hint: Some(dap::ScopePresentationHint::Locals),
            variables_reference: 2,
            named_variables: None,
            indexed_variables: None,
            expensive: false,
            source: None,
            line: None,
            column: None,
            end_line: None,
            end_column: None,
        },
        Scope {
            name: "Scope 2".into(),
            presentation_hint: None,
            variables_reference: 3,
            named_variables: None,
            indexed_variables: None,
            expensive: false,
            source: None,
            line: None,
            column: None,
            end_line: None,
            end_column: None,
        },
    ];

    client.on_request::<Scopes, _>({
        let scopes = Arc::new(scopes.clone());
        move |_, args| {
            assert_eq!(1, args.frame_id);

            Ok(dap::ScopesResponse {
                scopes: (*scopes).clone(),
            })
        }
    });

    let mut variables = HashMap::default();
    variables.insert(
        2,
        vec![
            Variable {
                name: "variable1".into(),
                value: "{nested1: \"Nested 1\", nested2: \"Nested 2\"}".into(),
                type_: None,
                presentation_hint: None,
                evaluate_name: None,
                variables_reference: 0,
                named_variables: None,
                indexed_variables: None,
                memory_reference: None,
                declaration_location_reference: None,
                value_location_reference: None,
            },
            Variable {
                name: "variable2".into(),
                value: "Value 2".into(),
                type_: None,
                presentation_hint: None,
                evaluate_name: None,
                variables_reference: 0,
                named_variables: None,
                indexed_variables: None,
                memory_reference: None,
                declaration_location_reference: None,
                value_location_reference: None,
            },
        ],
    );
    variables.insert(
        3,
        vec![Variable {
            name: "variable3".into(),
            value: "Value 3".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        }],
    );

    client.on_request::<Variables, _>({
        let variables = Arc::new(variables.clone());
        move |_, args| {
            Ok(dap::VariablesResponse {
                variables: variables.get(&args.variables_reference).unwrap().clone(),
            })
        }
    });

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
            item.mode()
                .as_running()
                .expect("Session should be running by this point")
                .clone()
        });
    cx.run_until_parked();

    running_state.update(cx, |running_state, cx| {
        let (stack_frame_list, stack_frame_id) =
            running_state.stack_frame_list().update(cx, |list, _| {
                (list.flatten_entries(), list.selected_stack_frame_id())
            });

        assert_eq!(Some(1), stack_frame_id);
        assert_eq!(stack_frames, stack_frame_list);

        running_state
            .variable_list()
            .update(cx, |variable_list, _| {
                assert_eq!(2, variable_list.scopes().len());
                assert_eq!(scopes, variable_list.scopes());
                let variables_by_scope = variable_list.variables_per_scope();

                // scope 1
                assert_eq!(
                    vec![
                        variables.get(&2).unwrap()[0].clone(),
                        variables.get(&2).unwrap()[1].clone(),
                    ],
                    variables_by_scope[0].1
                );

                // scope 2
                let empty_vec: Vec<dap::Variable> = vec![];
                assert_eq!(empty_vec, variables_by_scope[1].1);

                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    > variable1",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}

// tests that toggling a variable will fetch its children and shows it
#[gpui::test]
async fn test_keyboard_navigation(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    let test_file_content = r#"
        const variable1 = {
            nested1: "Nested 1",
            nested2: "Nested 2",
        };
        const variable2 = "Value 2";
        const variable3 = "Value 3";
    "#
    .unindent();

    fs.insert_tree(
        path!("/project"),
        json!({
           "src": {
               "test.js": test_file_content,
           }
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    workspace
        .update(cx, |workspace, window, cx| {
            workspace.focus_panel::<DebugPanel>(window, cx);
        })
        .unwrap();
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client.on_request::<dap::requests::Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

    client.on_request::<Initialize, _>(move |_, _| {
        Ok(dap::Capabilities {
            supports_step_back: Some(false),
            ..Default::default()
        })
    });

    client.on_request::<Launch, _>(move |_, _| Ok(()));

    let stack_frames = vec![StackFrame {
        id: 1,
        name: "Stack Frame 1".into(),
        source: Some(dap::Source {
            name: Some("test.js".into()),
            path: Some(path!("/project/src/test.js").into()),
            source_reference: None,
            presentation_hint: None,
            origin: None,
            sources: None,
            adapter_data: None,
            checksums: None,
        }),
        line: 1,
        column: 1,
        end_line: None,
        end_column: None,
        can_restart: None,
        instruction_pointer_reference: None,
        module_id: None,
        presentation_hint: None,
    }];

    client.on_request::<StackTrace, _>({
        let stack_frames = Arc::new(stack_frames.clone());
        move |_, args| {
            assert_eq!(1, args.thread_id);

            Ok(dap::StackTraceResponse {
                stack_frames: (*stack_frames).clone(),
                total_frames: None,
            })
        }
    });

    let scopes = vec![
        Scope {
            name: "Scope 1".into(),
            presentation_hint: Some(dap::ScopePresentationHint::Locals),
            variables_reference: 2,
            named_variables: None,
            indexed_variables: None,
            expensive: false,
            source: None,
            line: None,
            column: None,
            end_line: None,
            end_column: None,
        },
        Scope {
            name: "Scope 2".into(),
            presentation_hint: None,
            variables_reference: 4,
            named_variables: None,
            indexed_variables: None,
            expensive: false,
            source: None,
            line: None,
            column: None,
            end_line: None,
            end_column: None,
        },
    ];

    client.on_request::<Scopes, _>({
        let scopes = Arc::new(scopes.clone());
        move |_, args| {
            assert_eq!(1, args.frame_id);

            Ok(dap::ScopesResponse {
                scopes: (*scopes).clone(),
            })
        }
    });

    let scope1_variables = vec![
        Variable {
            name: "variable1".into(),
            value: "{nested1: \"Nested 1\", nested2: \"Nested 2\"}".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 3,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "variable2".into(),
            value: "Value 2".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
    ];

    let nested_variables = vec![
        Variable {
            name: "nested1".into(),
            value: "Nested 1".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "nested2".into(),
            value: "Nested 2".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
    ];

    let scope2_variables = vec![Variable {
        name: "variable3".into(),
        value: "Value 3".into(),
        type_: None,
        presentation_hint: None,
        evaluate_name: None,
        variables_reference: 0,
        named_variables: None,
        indexed_variables: None,
        memory_reference: None,
        declaration_location_reference: None,
        value_location_reference: None,
    }];

    client.on_request::<Variables, _>({
        let scope1_variables = Arc::new(scope1_variables.clone());
        let nested_variables = Arc::new(nested_variables.clone());
        let scope2_variables = Arc::new(scope2_variables.clone());
        move |_, args| match args.variables_reference {
            4 => Ok(dap::VariablesResponse {
                variables: (*scope2_variables).clone(),
            }),
            3 => Ok(dap::VariablesResponse {
                variables: (*nested_variables).clone(),
            }),
            2 => Ok(dap::VariablesResponse {
                variables: (*scope1_variables).clone(),
            }),
            id => unreachable!("unexpected variables reference {id}"),
        }
    });

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
            let running = item
                .mode()
                .as_running()
                .expect("Session should be running by this point")
                .clone();

            let variable_list = running.read_with(cx, |state, _| state.variable_list().clone());
            variable_list.update(cx, |_, cx| cx.focus_self(window));
            running
        });
    cx.dispatch_action(SelectFirst);
    cx.dispatch_action(SelectFirst);
    cx.run_until_parked();

    running_state.update(cx, |running_state, cx| {
        running_state
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1 <=== selected",
                    "    > variable1",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    cx.dispatch_action(SelectNext);
    cx.run_until_parked();

    running_state.update(cx, |running_state, cx| {
        running_state
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    > variable1 <=== selected",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    // expand the nested variables of variable 1
    cx.dispatch_action(ExpandSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |running_state, cx| {
        running_state
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1 <=== selected",
                    "        > nested1",
                    "        > nested2",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    // select the first nested variable of variable 1
    cx.dispatch_action(SelectNext);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1",
                    "        > nested1 <=== selected",
                    "        > nested2",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    // select the second nested variable of variable 1
    cx.dispatch_action(SelectNext);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1",
                    "        > nested1",
                    "        > nested2 <=== selected",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    // select variable 2 of scope 1
    cx.dispatch_action(SelectNext);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1",
                    "        > nested1",
                    "        > nested2",
                    "    > variable2 <=== selected",
                    "> Scope 2",
                ]);
            });
    });

    // select scope 2
    cx.dispatch_action(SelectNext);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1",
                    "        > nested1",
                    "        > nested2",
                    "    > variable2",
                    "> Scope 2 <=== selected",
                ]);
            });
    });

    // expand the nested variables of scope 2
    cx.dispatch_action(ExpandSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1",
                    "        > nested1",
                    "        > nested2",
                    "    > variable2",
                    "v Scope 2 <=== selected",
                    "    > variable3",
                ]);
            });
    });

    // select variable 3 of scope 2
    cx.dispatch_action(SelectNext);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1",
                    "        > nested1",
                    "        > nested2",
                    "    > variable2",
                    "v Scope 2",
                    "    > variable3 <=== selected",
                ]);
            });
    });

    // select scope 2
    cx.dispatch_action(SelectPrevious);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1",
                    "        > nested1",
                    "        > nested2",
                    "    > variable2",
                    "v Scope 2 <=== selected",
                    "    > variable3",
                ]);
            });
    });

    // collapse variables of scope 2
    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1",
                    "        > nested1",
                    "        > nested2",
                    "    > variable2",
                    "> Scope 2 <=== selected",
                ]);
            });
    });

    // select variable 2 of scope 1
    cx.dispatch_action(SelectPrevious);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1",
                    "        > nested1",
                    "        > nested2",
                    "    > variable2 <=== selected",
                    "> Scope 2",
                ]);
            });
    });

    // select nested2 of variable 1
    cx.dispatch_action(SelectPrevious);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1",
                    "        > nested1",
                    "        > nested2 <=== selected",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    // select nested1 of variable 1
    cx.dispatch_action(SelectPrevious);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1",
                    "        > nested1 <=== selected",
                    "        > nested2",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    // select variable 1 of scope 1
    cx.dispatch_action(SelectPrevious);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1 <=== selected",
                    "        > nested1",
                    "        > nested2",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    // collapse variables of variable 1
    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    > variable1 <=== selected",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    // select scope 1
    cx.dispatch_action(SelectPrevious);
    cx.run_until_parked();
    running_state.update(cx, |running_state, cx| {
        running_state
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1 <=== selected",
                    "    > variable1",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    // collapse variables of scope 1
    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec!["> Scope 1 <=== selected", "> Scope 2"]);
            });
    });

    // select scope 2 backwards
    cx.dispatch_action(SelectPrevious);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec!["> Scope 1", "> Scope 2 <=== selected"]);
            });
    });

    // select scope 1 backwards
    cx.dispatch_action(SelectNext);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec!["> Scope 1 <=== selected", "> Scope 2"]);
            });
    });

    // test stepping through nested with ExpandSelectedEntry/CollapseSelectedEntry actions

    cx.dispatch_action(ExpandSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1 <=== selected",
                    "    > variable1",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    cx.dispatch_action(ExpandSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    > variable1 <=== selected",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    cx.dispatch_action(ExpandSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1 <=== selected",
                    "        > nested1",
                    "        > nested2",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    cx.dispatch_action(ExpandSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1",
                    "        > nested1 <=== selected",
                    "        > nested2",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    cx.dispatch_action(ExpandSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1",
                    "        > nested1",
                    "        > nested2 <=== selected",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    cx.dispatch_action(ExpandSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1",
                    "        > nested1",
                    "        > nested2",
                    "    > variable2 <=== selected",
                    "> Scope 2",
                ]);
            });
    });

    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1",
                    "        > nested1",
                    "        > nested2 <=== selected",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1",
                    "        > nested1 <=== selected",
                    "        > nested2",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    v variable1 <=== selected",
                    "        > nested1",
                    "        > nested2",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1",
                    "    > variable1 <=== selected",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec![
                    "v Scope 1 <=== selected",
                    "    > variable1",
                    "    > variable2",
                    "> Scope 2",
                ]);
            });
    });

    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    running_state.update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, _| {
                variable_list.assert_visual_entries(vec!["> Scope 1 <=== selected", "> Scope 2"]);
            });
    });

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}

#[gpui::test]
async fn test_variable_list_only_sends_requests_when_rendering(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    let test_file_content = r#"
        import { SOME_VALUE } './module.js';

        console.log(SOME_VALUE);
    "#
    .unindent();

    let module_file_content = r#"
        export SOME_VALUE = 'some value';
    "#
    .unindent();

    fs.insert_tree(
        path!("/project"),
        json!({
           "src": {
               "test.js": test_file_content,
               "module.js": module_file_content,
           }
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client.on_request::<dap::requests::Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

    client.on_request::<Initialize, _>(move |_, _| {
        Ok(dap::Capabilities {
            supports_step_back: Some(false),
            ..Default::default()
        })
    });

    client.on_request::<Launch, _>(move |_, _| Ok(()));

    let stack_frames = vec![
        StackFrame {
            id: 1,
            name: "Stack Frame 1".into(),
            source: Some(dap::Source {
                name: Some("test.js".into()),
                path: Some(path!("/project/src/test.js").into()),
                source_reference: None,
                presentation_hint: None,
                origin: None,
                sources: None,
                adapter_data: None,
                checksums: None,
            }),
            line: 3,
            column: 1,
            end_line: None,
            end_column: None,
            can_restart: None,
            instruction_pointer_reference: None,
            module_id: None,
            presentation_hint: None,
        },
        StackFrame {
            id: 2,
            name: "Stack Frame 2".into(),
            source: Some(dap::Source {
                name: Some("module.js".into()),
                path: Some(path!("/project/src/module.js").into()),
                source_reference: None,
                presentation_hint: None,
                origin: None,
                sources: None,
                adapter_data: None,
                checksums: None,
            }),
            line: 1,
            column: 1,
            end_line: None,
            end_column: None,
            can_restart: None,
            instruction_pointer_reference: None,
            module_id: None,
            presentation_hint: None,
        },
    ];

    client.on_request::<StackTrace, _>({
        let stack_frames = Arc::new(stack_frames.clone());
        move |_, args| {
            assert_eq!(1, args.thread_id);

            Ok(dap::StackTraceResponse {
                stack_frames: (*stack_frames).clone(),
                total_frames: None,
            })
        }
    });

    let frame_1_scopes = vec![Scope {
        name: "Frame 1 Scope 1".into(),
        presentation_hint: None,
        variables_reference: 2,
        named_variables: None,
        indexed_variables: None,
        expensive: false,
        source: None,
        line: None,
        column: None,
        end_line: None,
        end_column: None,
    }];

    let made_scopes_request = Arc::new(AtomicBool::new(false));

    client.on_request::<Scopes, _>({
        let frame_1_scopes = Arc::new(frame_1_scopes.clone());
        let made_scopes_request = made_scopes_request.clone();
        move |_, args| {
            assert_eq!(1, args.frame_id);
            assert!(
                !made_scopes_request.load(Ordering::SeqCst),
                "We should be caching the scope request"
            );

            made_scopes_request.store(true, Ordering::SeqCst);

            Ok(dap::ScopesResponse {
                scopes: (*frame_1_scopes).clone(),
            })
        }
    });

    let frame_1_variables = vec![
        Variable {
            name: "variable1".into(),
            value: "value 1".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "variable2".into(),
            value: "value 2".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
    ];

    client.on_request::<Variables, _>({
        let frame_1_variables = Arc::new(frame_1_variables.clone());
        move |_, args| {
            assert_eq!(2, args.variables_reference);

            Ok(dap::VariablesResponse {
                variables: (*frame_1_variables).clone(),
            })
        }
    });

    cx.run_until_parked();

    let running_state = active_debug_session_panel(workspace, cx).update_in(cx, |item, _, _| {
        let state = item
            .mode()
            .as_running()
            .expect("Session should be running by this point")
            .clone();

        state
    });

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

    running_state.update(cx, |running_state, cx| {
        let (stack_frame_list, stack_frame_id) =
            running_state.stack_frame_list().update(cx, |list, _| {
                (list.flatten_entries(), list.selected_stack_frame_id())
            });

        assert_eq!(Some(1), stack_frame_id);
        assert_eq!(stack_frames, stack_frame_list);

        let variable_list = running_state.variable_list().read(cx);

        assert_eq!(frame_1_variables, variable_list.variables());
        assert!(made_scopes_request.load(Ordering::SeqCst));
    });

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}

#[gpui::test]
async fn test_it_fetches_scopes_variables_when_you_select_a_stack_frame(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    let test_file_content = r#"
        import { SOME_VALUE } './module.js';

        console.log(SOME_VALUE);
    "#
    .unindent();

    let module_file_content = r#"
        export SOME_VALUE = 'some value';
    "#
    .unindent();

    fs.insert_tree(
        path!("/project"),
        json!({
           "src": {
               "test.js": test_file_content,
               "module.js": module_file_content,
           }
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    workspace
        .update(cx, |workspace, window, cx| {
            workspace.focus_panel::<DebugPanel>(window, cx);
        })
        .unwrap();
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client.on_request::<dap::requests::Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

    client.on_request::<Initialize, _>(move |_, _| {
        Ok(dap::Capabilities {
            supports_step_back: Some(false),
            ..Default::default()
        })
    });

    client.on_request::<Launch, _>(move |_, _| Ok(()));

    let stack_frames = vec![
        StackFrame {
            id: 1,
            name: "Stack Frame 1".into(),
            source: Some(dap::Source {
                name: Some("test.js".into()),
                path: Some(path!("/project/src/test.js").into()),
                source_reference: None,
                presentation_hint: None,
                origin: None,
                sources: None,
                adapter_data: None,
                checksums: None,
            }),
            line: 3,
            column: 1,
            end_line: None,
            end_column: None,
            can_restart: None,
            instruction_pointer_reference: None,
            module_id: None,
            presentation_hint: None,
        },
        StackFrame {
            id: 2,
            name: "Stack Frame 2".into(),
            source: Some(dap::Source {
                name: Some("module.js".into()),
                path: Some(path!("/project/src/module.js").into()),
                source_reference: None,
                presentation_hint: None,
                origin: None,
                sources: None,
                adapter_data: None,
                checksums: None,
            }),
            line: 1,
            column: 1,
            end_line: None,
            end_column: None,
            can_restart: None,
            instruction_pointer_reference: None,
            module_id: None,
            presentation_hint: None,
        },
    ];

    client.on_request::<StackTrace, _>({
        let stack_frames = Arc::new(stack_frames.clone());
        move |_, args| {
            assert_eq!(1, args.thread_id);

            Ok(dap::StackTraceResponse {
                stack_frames: (*stack_frames).clone(),
                total_frames: None,
            })
        }
    });

    let frame_1_scopes = vec![Scope {
        name: "Frame 1 Scope 1".into(),
        presentation_hint: None,
        variables_reference: 2,
        named_variables: None,
        indexed_variables: None,
        expensive: false,
        source: None,
        line: None,
        column: None,
        end_line: None,
        end_column: None,
    }];

    // add handlers for fetching the second stack frame's scopes and variables
    // after the user clicked the stack frame
    let frame_2_scopes = vec![Scope {
        name: "Frame 2 Scope 1".into(),
        presentation_hint: None,
        variables_reference: 3,
        named_variables: None,
        indexed_variables: None,
        expensive: false,
        source: None,
        line: None,
        column: None,
        end_line: None,
        end_column: None,
    }];

    let called_second_stack_frame = Arc::new(AtomicBool::new(false));
    let called_first_stack_frame = Arc::new(AtomicBool::new(false));

    client.on_request::<Scopes, _>({
        let frame_1_scopes = Arc::new(frame_1_scopes.clone());
        let frame_2_scopes = Arc::new(frame_2_scopes.clone());
        let called_first_stack_frame = called_first_stack_frame.clone();
        let called_second_stack_frame = called_second_stack_frame.clone();
        move |_, args| match args.frame_id {
            1 => {
                called_first_stack_frame.store(true, Ordering::SeqCst);
                Ok(dap::ScopesResponse {
                    scopes: (*frame_1_scopes).clone(),
                })
            }
            2 => {
                called_second_stack_frame.store(true, Ordering::SeqCst);

                Ok(dap::ScopesResponse {
                    scopes: (*frame_2_scopes).clone(),
                })
            }
            _ => panic!("Made a scopes request with an invalid frame id"),
        }
    });

    let frame_1_variables = vec![
        Variable {
            name: "variable1".into(),
            value: "value 1".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "variable2".into(),
            value: "value 2".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
    ];

    let frame_2_variables = vec![
        Variable {
            name: "variable3".into(),
            value: "old value 1".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
        Variable {
            name: "variable4".into(),
            value: "old value 2".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
            declaration_location_reference: None,
            value_location_reference: None,
        },
    ];

    client.on_request::<Variables, _>({
        let frame_1_variables = Arc::new(frame_1_variables.clone());
        move |_, args| {
            assert_eq!(2, args.variables_reference);

            Ok(dap::VariablesResponse {
                variables: (*frame_1_variables).clone(),
            })
        }
    });

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
            item.mode()
                .as_running()
                .expect("Session should be running by this point")
                .clone()
        });

    running_state.update(cx, |running_state, cx| {
        let (stack_frame_list, stack_frame_id) =
            running_state.stack_frame_list().update(cx, |list, _| {
                (list.flatten_entries(), list.selected_stack_frame_id())
            });

        let variable_list = running_state.variable_list().read(cx);
        let variables = variable_list.variables();

        assert_eq!(Some(1), stack_frame_id);
        assert_eq!(
            running_state
                .stack_frame_list()
                .read(cx)
                .selected_stack_frame_id(),
            Some(1)
        );

        assert!(
            called_first_stack_frame.load(std::sync::atomic::Ordering::SeqCst),
            "Request scopes shouldn't be called before it's needed"
        );
        assert!(
            !called_second_stack_frame.load(std::sync::atomic::Ordering::SeqCst),
            "Request scopes shouldn't be called before it's needed"
        );

        assert_eq!(stack_frames, stack_frame_list);
        assert_eq!(frame_1_variables, variables);
    });

    client.on_request::<Variables, _>({
        let frame_2_variables = Arc::new(frame_2_variables.clone());
        move |_, args| {
            assert_eq!(3, args.variables_reference);

            Ok(dap::VariablesResponse {
                variables: (*frame_2_variables).clone(),
            })
        }
    });

    running_state
        .update_in(cx, |running_state, window, cx| {
            running_state
                .stack_frame_list()
                .update(cx, |stack_frame_list, cx| {
                    stack_frame_list.select_stack_frame(&stack_frames[1], true, window, cx)
                })
        })
        .await
        .unwrap();

    cx.run_until_parked();

    running_state.update(cx, |running_state, cx| {
        let (stack_frame_list, stack_frame_id) =
            running_state.stack_frame_list().update(cx, |list, _| {
                (list.flatten_entries(), list.selected_stack_frame_id())
            });

        let variable_list = running_state.variable_list().read(cx);
        let variables = variable_list.variables();

        assert_eq!(Some(2), stack_frame_id);
        assert!(
            called_second_stack_frame.load(std::sync::atomic::Ordering::SeqCst),
            "Request scopes shouldn't be called before it's needed"
        );

        assert_eq!(stack_frames, stack_frame_list);

        assert_eq!(variables, frame_2_variables,);
    });

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}
