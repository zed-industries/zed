use std::sync::Arc;

use crate::{
    tests::{active_debug_panel_item, init_test, init_test_workspace},
    variable_list::{CollapseSelectedEntry, ExpandSelectedEntry, VariableContainer},
};
use collections::HashMap;
use dap::{
    requests::{Disconnect, Initialize, Launch, Scopes, StackTrace, Variables},
    Scope, StackFrame, Variable,
};
use gpui::{BackgroundExecutor, Focusable, TestAppContext, VisualTestContext};
use menu::{SelectFirst, SelectNext};
use project::{FakeFs, Project};
use serde_json::json;
use unindent::Unindent as _;

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
        "/project",
        json!({
           "src": {
               "test.js": test_file_content,
           }
        }),
    )
    .await;

    let project = Project::test(fs, ["/project".as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.start_debug_session(
            task::DebugAdapterConfig {
                label: "test config".into(),
                kind: task::DebugAdapterKind::Fake,
                request: task::DebugRequestType::Launch,
                program: None,
                cwd: None,
                initialize_args: None,
            },
            cx,
        )
    });

    let (session, client) = task.await.unwrap();

    client
        .on_request::<Initialize, _>(move |_, _| {
            Ok(dap::Capabilities {
                supports_step_back: Some(false),
                ..Default::default()
            })
        })
        .await;

    client.on_request::<Launch, _>(move |_, _| Ok(())).await;

    let stack_frames = vec![StackFrame {
        id: 1,
        name: "Stack Frame 1".into(),
        source: Some(dap::Source {
            name: Some("test.js".into()),
            path: Some("/project/src/test.js".into()),
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

    client
        .on_request::<StackTrace, _>({
            let stack_frames = Arc::new(stack_frames.clone());
            move |_, args| {
                assert_eq!(1, args.thread_id);

                Ok(dap::StackTraceResponse {
                    stack_frames: (*stack_frames).clone(),
                    total_frames: None,
                })
            }
        })
        .await;

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

    client
        .on_request::<Scopes, _>({
            let scopes = Arc::new(scopes.clone());
            move |_, args| {
                assert_eq!(1, args.frame_id);

                Ok(dap::ScopesResponse {
                    scopes: (*scopes).clone(),
                })
            }
        })
        .await;

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
        },
    ];

    client
        .on_request::<Variables, _>({
            let variables = Arc::new(variables.clone());
            move |_, args| {
                assert_eq!(2, args.variables_reference);

                Ok(dap::VariablesResponse {
                    variables: (*variables).clone(),
                })
            }
        })
        .await;

    client.on_request::<Disconnect, _>(move |_, _| Ok(())).await;

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

    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        let stack_frame_list = debug_panel_item.stack_frame_list().read(cx);

        assert_eq!(1, stack_frame_list.current_stack_frame_id());
        assert_eq!(stack_frames, stack_frame_list.stack_frames().clone());

        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                assert_eq!(1, variable_list.scopes().len());
                assert_eq!(scopes, variable_list.scopes().get(&1).unwrap().clone());
                assert_eq!(
                    vec![
                        VariableContainer {
                            container_reference: scopes[0].variables_reference,
                            variable: variables[0].clone(),
                            depth: 1,
                        },
                        VariableContainer {
                            container_reference: scopes[0].variables_reference,
                            variable: variables[1].clone(),
                            depth: 1,
                        },
                    ],
                    variable_list.variables_by_stack_frame_id(1)
                );

                variable_list.assert_visual_entries(
                    vec!["v Scope 1", "    > variable1", "    > variable2"],
                    cx,
                );
            });
    });

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
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
        "/project",
        json!({
           "src": {
               "test.js": test_file_content,
           }
        }),
    )
    .await;

    let project = Project::test(fs, ["/project".as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.start_debug_session(
            task::DebugAdapterConfig {
                label: "test config".into(),
                kind: task::DebugAdapterKind::Fake,
                request: task::DebugRequestType::Launch,
                program: None,
                cwd: None,
                initialize_args: None,
            },
            cx,
        )
    });

    let (session, client) = task.await.unwrap();

    client
        .on_request::<Initialize, _>(move |_, _| {
            Ok(dap::Capabilities {
                supports_step_back: Some(false),
                ..Default::default()
            })
        })
        .await;

    client.on_request::<Launch, _>(move |_, _| Ok(())).await;

    let stack_frames = vec![StackFrame {
        id: 1,
        name: "Stack Frame 1".into(),
        source: Some(dap::Source {
            name: Some("test.js".into()),
            path: Some("/project/src/test.js".into()),
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

    client
        .on_request::<StackTrace, _>({
            let stack_frames = Arc::new(stack_frames.clone());
            move |_, args| {
                assert_eq!(1, args.thread_id);

                Ok(dap::StackTraceResponse {
                    stack_frames: (*stack_frames).clone(),
                    total_frames: None,
                })
            }
        })
        .await;

    let scopes = vec![
        Scope {
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

    client
        .on_request::<Scopes, _>({
            let scopes = Arc::new(scopes.clone());
            move |_, args| {
                assert_eq!(1, args.frame_id);

                Ok(dap::ScopesResponse {
                    scopes: (*scopes).clone(),
                })
            }
        })
        .await;

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
        }],
    );

    client
        .on_request::<Variables, _>({
            let variables = Arc::new(variables.clone());
            move |_, args| {
                Ok(dap::VariablesResponse {
                    variables: variables.get(&args.variables_reference).unwrap().clone(),
                })
            }
        })
        .await;

    client.on_request::<Disconnect, _>(move |_, _| Ok(())).await;

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

    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        let stack_frame_list = debug_panel_item.stack_frame_list().read(cx);

        assert_eq!(1, stack_frame_list.current_stack_frame_id());
        assert_eq!(stack_frames, stack_frame_list.stack_frames().clone());

        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                assert_eq!(1, variable_list.scopes().len());
                assert_eq!(scopes, variable_list.scopes().get(&1).unwrap().clone());

                // scope 1
                assert_eq!(
                    vec![
                        VariableContainer {
                            container_reference: scopes[0].variables_reference,
                            variable: variables.get(&2).unwrap()[0].clone(),
                            depth: 1,
                        },
                        VariableContainer {
                            container_reference: scopes[0].variables_reference,
                            variable: variables.get(&2).unwrap()[1].clone(),
                            depth: 1,
                        },
                    ],
                    variable_list.variables_by_scope(1, 2).unwrap().variables()
                );

                // scope 2
                assert_eq!(
                    vec![VariableContainer {
                        container_reference: scopes[1].variables_reference,
                        variable: variables.get(&3).unwrap()[0].clone(),
                        depth: 1,
                    }],
                    variable_list.variables_by_scope(1, 3).unwrap().variables()
                );

                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1",
                        "    > variable1",
                        "    > variable2",
                        "> Scope 2",
                    ],
                    cx,
                );
            });
    });

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
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
        "/project",
        json!({
           "src": {
               "test.js": test_file_content,
           }
        }),
    )
    .await;

    let project = Project::test(fs, ["/project".as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.start_debug_session(
            task::DebugAdapterConfig {
                label: "test config".into(),
                kind: task::DebugAdapterKind::Fake,
                request: task::DebugRequestType::Launch,
                program: None,
                cwd: None,
                initialize_args: None,
            },
            cx,
        )
    });

    let (session, client) = task.await.unwrap();

    client
        .on_request::<Initialize, _>(move |_, _| {
            Ok(dap::Capabilities {
                supports_step_back: Some(false),
                ..Default::default()
            })
        })
        .await;

    client.on_request::<Launch, _>(move |_, _| Ok(())).await;

    let stack_frames = vec![StackFrame {
        id: 1,
        name: "Stack Frame 1".into(),
        source: Some(dap::Source {
            name: Some("test.js".into()),
            path: Some("/project/src/test.js".into()),
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

    client
        .on_request::<StackTrace, _>({
            let stack_frames = Arc::new(stack_frames.clone());
            move |_, args| {
                assert_eq!(1, args.thread_id);

                Ok(dap::StackTraceResponse {
                    stack_frames: (*stack_frames).clone(),
                    total_frames: None,
                })
            }
        })
        .await;

    let scopes = vec![
        Scope {
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

    client
        .on_request::<Scopes, _>({
            let scopes = Arc::new(scopes.clone());
            move |_, args| {
                assert_eq!(1, args.frame_id);

                Ok(dap::ScopesResponse {
                    scopes: (*scopes).clone(),
                })
            }
        })
        .await;

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
    }];

    client
        .on_request::<Variables, _>({
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
        })
        .await;

    client.on_request::<Disconnect, _>(move |_, _| Ok(())).await;

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

    active_debug_panel_item(workspace, cx).update_in(cx, |debug_panel_item, window, cx| {
        debug_panel_item
            .variable_list()
            .focus_handle(cx)
            .focus(window);
    });

    cx.dispatch_action(SelectFirst);
    cx.run_until_parked();

    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1 <=== selected",
                        "    > variable1",
                        "    > variable2",
                        "> Scope 2",
                    ],
                    cx,
                );
            });
    });

    // select the first variable of scope 1
    cx.dispatch_action(SelectNext);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1",
                        "    > variable1 <=== selected",
                        "    > variable2",
                        "> Scope 2",
                    ],
                    cx,
                );
            });
    });

    // expand the nested variables of variable 1
    cx.dispatch_action(ExpandSelectedEntry);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1",
                        "    v variable1 <=== selected",
                        "        > nested1",
                        "        > nested2",
                        "    > variable2",
                        "> Scope 2",
                    ],
                    cx,
                );
            });
    });

    // select the first nested variable of variable 1
    cx.dispatch_action(SelectNext);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1",
                        "    v variable1",
                        "        > nested1 <=== selected",
                        "        > nested2",
                        "    > variable2",
                        "> Scope 2",
                    ],
                    cx,
                );
            });
    });

    // select the second nested variable of variable 1
    cx.dispatch_action(SelectNext);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1",
                        "    v variable1",
                        "        > nested1",
                        "        > nested2 <=== selected",
                        "    > variable2",
                        "> Scope 2",
                    ],
                    cx,
                );
            });
    });

    // select variable 2 of scope 1
    cx.dispatch_action(SelectNext);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1",
                        "    v variable1",
                        "        > nested1",
                        "        > nested2",
                        "    > variable2 <=== selected",
                        "> Scope 2",
                    ],
                    cx,
                );
            });
    });

    // select scope 2
    cx.dispatch_action(SelectNext);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1",
                        "    v variable1",
                        "        > nested1",
                        "        > nested2",
                        "    > variable2",
                        "> Scope 2 <=== selected",
                    ],
                    cx,
                );
            });
    });

    // expand the nested variables of scope 2
    cx.dispatch_action(ExpandSelectedEntry);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1",
                        "    v variable1",
                        "        > nested1",
                        "        > nested2",
                        "    > variable2",
                        "v Scope 2 <=== selected",
                        "    > variable3",
                    ],
                    cx,
                );
            });
    });

    // select variable 3 of scope 2
    cx.dispatch_action(SelectNext);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1",
                        "    v variable1",
                        "        > nested1",
                        "        > nested2",
                        "    > variable2",
                        "v Scope 2",
                        "    > variable3 <=== selected",
                    ],
                    cx,
                );
            });
    });

    // select scope 2
    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1",
                        "    v variable1",
                        "        > nested1",
                        "        > nested2",
                        "    > variable2",
                        "v Scope 2 <=== selected",
                        "    > variable3",
                    ],
                    cx,
                );
            });
    });

    // collapse variables of scope 2
    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1",
                        "    v variable1",
                        "        > nested1",
                        "        > nested2",
                        "    > variable2",
                        "> Scope 2 <=== selected",
                    ],
                    cx,
                );
            });
    });

    // select variable 2 of scope 1
    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1",
                        "    v variable1",
                        "        > nested1",
                        "        > nested2",
                        "    > variable2 <=== selected",
                        "> Scope 2",
                    ],
                    cx,
                );
            });
    });

    // select nested2 of variable 1
    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1",
                        "    v variable1",
                        "        > nested1",
                        "        > nested2 <=== selected",
                        "    > variable2",
                        "> Scope 2",
                    ],
                    cx,
                );
            });
    });

    // select nested1 of variable 1
    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1",
                        "    v variable1",
                        "        > nested1 <=== selected",
                        "        > nested2",
                        "    > variable2",
                        "> Scope 2",
                    ],
                    cx,
                );
            });
    });

    // select variable 1 of scope 1
    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1",
                        "    v variable1 <=== selected",
                        "        > nested1",
                        "        > nested2",
                        "    > variable2",
                        "> Scope 2",
                    ],
                    cx,
                );
            });
    });

    // collapse variables of variable 1
    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1",
                        "    > variable1 <=== selected",
                        "    > variable2",
                        "> Scope 2",
                    ],
                    cx,
                );
            });
    });

    // select scope 1
    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list.assert_visual_entries(
                    vec![
                        "v Scope 1 <=== selected",
                        "    > variable1",
                        "    > variable2",
                        "> Scope 2",
                    ],
                    cx,
                );
            });
    });

    // collapse variables of scope 1
    cx.dispatch_action(CollapseSelectedEntry);
    cx.run_until_parked();
    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        debug_panel_item
            .variable_list()
            .update(cx, |variable_list, cx| {
                variable_list
                    .assert_visual_entries(vec!["> Scope 1 <=== selected", "> Scope 2"], cx);
            });
    });

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}

#[gpui::test]
async fn test_it_only_fetches_scopes_and_variables_for_the_first_stack_frame(
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
        "/project",
        json!({
           "src": {
               "test.js": test_file_content,
               "module.js": module_file_content,
           }
        }),
    )
    .await;

    let project = Project::test(fs, ["/project".as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.start_debug_session(
            task::DebugAdapterConfig {
                label: "test config".into(),
                kind: task::DebugAdapterKind::Fake,
                request: task::DebugRequestType::Launch,
                program: None,
                cwd: None,
                initialize_args: None,
            },
            cx,
        )
    });

    let (session, client) = task.await.unwrap();

    client
        .on_request::<Initialize, _>(move |_, _| {
            Ok(dap::Capabilities {
                supports_step_back: Some(false),
                ..Default::default()
            })
        })
        .await;

    client.on_request::<Launch, _>(move |_, _| Ok(())).await;

    let stack_frames = vec![
        StackFrame {
            id: 1,
            name: "Stack Frame 1".into(),
            source: Some(dap::Source {
                name: Some("test.js".into()),
                path: Some("/project/src/test.js".into()),
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
                path: Some("/project/src/module.js".into()),
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

    client
        .on_request::<StackTrace, _>({
            let stack_frames = Arc::new(stack_frames.clone());
            move |_, args| {
                assert_eq!(1, args.thread_id);

                Ok(dap::StackTraceResponse {
                    stack_frames: (*stack_frames).clone(),
                    total_frames: None,
                })
            }
        })
        .await;

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

    client
        .on_request::<Scopes, _>({
            let frame_1_scopes = Arc::new(frame_1_scopes.clone());
            move |_, args| {
                assert_eq!(1, args.frame_id);

                Ok(dap::ScopesResponse {
                    scopes: (*frame_1_scopes).clone(),
                })
            }
        })
        .await;

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
        },
    ];

    client
        .on_request::<Variables, _>({
            let frame_1_variables = Arc::new(frame_1_variables.clone());
            move |_, args| {
                assert_eq!(2, args.variables_reference);

                Ok(dap::VariablesResponse {
                    variables: (*frame_1_variables).clone(),
                })
            }
        })
        .await;

    client.on_request::<Disconnect, _>(move |_, _| Ok(())).await;

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

    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        let stack_frame_list = debug_panel_item.stack_frame_list().read(cx);
        let variable_list = debug_panel_item.variable_list().read(cx);

        assert_eq!(1, stack_frame_list.current_stack_frame_id());
        assert_eq!(stack_frames, stack_frame_list.stack_frames().clone());

        assert_eq!(
            frame_1_variables
                .clone()
                .into_iter()
                .map(|variable| VariableContainer {
                    container_reference: 2,
                    variable,
                    depth: 1
                })
                .collect::<Vec<_>>(),
            variable_list.variables_by_stack_frame_id(1)
        );
        assert!(variable_list.variables_by_stack_frame_id(2).is_empty());
    });

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
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
        "/project",
        json!({
           "src": {
               "test.js": test_file_content,
               "module.js": module_file_content,
           }
        }),
    )
    .await;

    let project = Project::test(fs, ["/project".as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.start_debug_session(
            task::DebugAdapterConfig {
                label: "test config".into(),
                kind: task::DebugAdapterKind::Fake,
                request: task::DebugRequestType::Launch,
                program: None,
                cwd: None,
                initialize_args: None,
            },
            cx,
        )
    });

    let (session, client) = task.await.unwrap();

    client
        .on_request::<Initialize, _>(move |_, _| {
            Ok(dap::Capabilities {
                supports_step_back: Some(false),
                ..Default::default()
            })
        })
        .await;

    client.on_request::<Launch, _>(move |_, _| Ok(())).await;

    let stack_frames = vec![
        StackFrame {
            id: 1,
            name: "Stack Frame 1".into(),
            source: Some(dap::Source {
                name: Some("test.js".into()),
                path: Some("/project/src/test.js".into()),
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
                path: Some("/project/src/module.js".into()),
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

    client
        .on_request::<StackTrace, _>({
            let stack_frames = Arc::new(stack_frames.clone());
            move |_, args| {
                assert_eq!(1, args.thread_id);

                Ok(dap::StackTraceResponse {
                    stack_frames: (*stack_frames).clone(),
                    total_frames: None,
                })
            }
        })
        .await;

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

    client
        .on_request::<Scopes, _>({
            let frame_1_scopes = Arc::new(frame_1_scopes.clone());
            move |_, args| {
                assert_eq!(1, args.frame_id);

                Ok(dap::ScopesResponse {
                    scopes: (*frame_1_scopes).clone(),
                })
            }
        })
        .await;

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
        },
    ];

    client
        .on_request::<Variables, _>({
            let frame_1_variables = Arc::new(frame_1_variables.clone());
            move |_, args| {
                assert_eq!(2, args.variables_reference);

                Ok(dap::VariablesResponse {
                    variables: (*frame_1_variables).clone(),
                })
            }
        })
        .await;

    client.on_request::<Disconnect, _>(move |_, _| Ok(())).await;

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

    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        let stack_frame_list = debug_panel_item.stack_frame_list().read(cx);
        let variable_list = debug_panel_item.variable_list().read(cx);

        assert_eq!(1, stack_frame_list.current_stack_frame_id());
        assert_eq!(stack_frames, stack_frame_list.stack_frames().clone());

        assert_eq!(
            frame_1_variables
                .clone()
                .into_iter()
                .map(|variable| VariableContainer {
                    container_reference: 2,
                    variable,
                    depth: 1
                })
                .collect::<Vec<_>>(),
            variable_list.variables_by_stack_frame_id(1)
        );
        assert!(variable_list.variables_by_stack_frame_id(2).is_empty());
    });

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

    client
        .on_request::<Scopes, _>({
            let frame_2_scopes = Arc::new(frame_2_scopes.clone());
            move |_, args| {
                assert_eq!(2, args.frame_id);

                Ok(dap::ScopesResponse {
                    scopes: (*frame_2_scopes).clone(),
                })
            }
        })
        .await;

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
        },
    ];

    client
        .on_request::<Variables, _>({
            let frame_2_variables = Arc::new(frame_2_variables.clone());
            move |_, args| {
                assert_eq!(3, args.variables_reference);

                Ok(dap::VariablesResponse {
                    variables: (*frame_2_variables).clone(),
                })
            }
        })
        .await;

    active_debug_panel_item(workspace, cx)
        .update_in(cx, |debug_panel_item, window, cx| {
            debug_panel_item
                .stack_frame_list()
                .update(cx, |stack_frame_list, cx| {
                    stack_frame_list.select_stack_frame(&stack_frames[1], true, window, cx)
                })
        })
        .await
        .unwrap();

    cx.run_until_parked();

    active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
        let stack_frame_list = debug_panel_item.stack_frame_list().read(cx);
        let variable_list = debug_panel_item.variable_list().read(cx);

        assert_eq!(2, stack_frame_list.current_stack_frame_id());
        assert_eq!(stack_frames, stack_frame_list.stack_frames().clone());

        assert_eq!(
            frame_1_variables
                .into_iter()
                .map(|variable| VariableContainer {
                    container_reference: 2,
                    variable,
                    depth: 1
                })
                .collect::<Vec<_>>(),
            variable_list.variables_by_stack_frame_id(1)
        );
        assert_eq!(
            frame_2_variables
                .into_iter()
                .map(|variable| VariableContainer {
                    container_reference: 3,
                    variable,
                    depth: 1
                })
                .collect::<Vec<_>>(),
            variable_list.variables_by_stack_frame_id(2)
        );
    });

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}
