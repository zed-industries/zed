use std::sync::Arc;

use crate::{
    debugger_panel::DebugPanel,
    tests::{add_debugger_panel, init_test},
    variable_list::VariableContainer,
};
use collections::HashMap;
use dap::{
    requests::{Disconnect, Initialize, Launch, Scopes, StackTrace, Variables},
    Scope, StackFrame, Variable,
};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
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
        console.log("Some value");
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
    let workspace = add_debugger_panel(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |store, cx| {
            store.start_test_client(
                task::DebugAdapterConfig {
                    kind: task::DebugAdapterKind::Fake,
                    request: task::DebugRequestType::Launch,
                    program: None,
                    cwd: None,
                    initialize_args: None,
                },
                cx,
            )
        })
    });

    let client = task.await.unwrap();

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
        line: 3,
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
            name: "Variable 1".into(),
            value: "Value 1".into(),
            type_: None,
            presentation_hint: None,
            evaluate_name: None,
            variables_reference: 0,
            named_variables: None,
            indexed_variables: None,
            memory_reference: None,
        },
        Variable {
            name: "Variable 2".into(),
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

    workspace
        .update(cx, |workspace, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_debug_panel_item = debug_panel
                .update(cx, |this, cx| this.active_debug_panel_item(cx))
                .unwrap();

            active_debug_panel_item.update(cx, |debug_panel_item, cx| {
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
                                    container_reference: 2,
                                    variable: variables[0].clone(),
                                    depth: 1,
                                },
                                VariableContainer {
                                    container_reference: 2,
                                    variable: variables[1].clone(),
                                    depth: 1,
                                },
                            ],
                            variable_list.variables(cx)
                        );
                    });
            });
        })
        .unwrap();

    let shutdown_client = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_client(&client.id(), cx)
        })
    });

    shutdown_client.await.unwrap();
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
        console.log("Some value");
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
    let workspace = add_debugger_panel(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |store, cx| {
            store.start_test_client(
                task::DebugAdapterConfig {
                    kind: task::DebugAdapterKind::Fake,
                    request: task::DebugRequestType::Launch,
                    program: None,
                    cwd: None,
                    initialize_args: None,
                },
                cx,
            )
        })
    });

    let client = task.await.unwrap();

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
        line: 3,
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
                name: "Variable 1".into(),
                value: "Value 1".into(),
                type_: None,
                presentation_hint: None,
                evaluate_name: None,
                variables_reference: 0,
                named_variables: None,
                indexed_variables: None,
                memory_reference: None,
            },
            Variable {
                name: "Variable 2".into(),
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
            name: "Variable 3".into(),
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

    workspace
        .update(cx, |workspace, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_debug_panel_item = debug_panel
                .update(cx, |this, cx| this.active_debug_panel_item(cx))
                .unwrap();

            active_debug_panel_item.update(cx, |debug_panel_item, cx| {
                let stack_frame_list = debug_panel_item.stack_frame_list().read(cx);

                assert_eq!(1, stack_frame_list.current_stack_frame_id());
                assert_eq!(stack_frames, stack_frame_list.stack_frames().clone());

                debug_panel_item
                    .variable_list()
                    .update(cx, |variable_list, _| {
                        assert_eq!(1, variable_list.scopes().len());
                        assert_eq!(scopes, variable_list.scopes().get(&1).unwrap().clone());

                        // scope 1
                        assert_eq!(
                            [
                                VariableContainer {
                                    container_reference: 2,
                                    variable: variables.get(&2).unwrap()[0].clone(),
                                    depth: 1,
                                },
                                VariableContainer {
                                    container_reference: 2,
                                    variable: variables.get(&2).unwrap()[1].clone(),
                                    depth: 1,
                                },
                            ],
                            variable_list.variables_by_scope(1, 2).unwrap().variables()
                        );

                        // scope 2
                        assert_eq!(
                            [VariableContainer {
                                container_reference: 3,
                                variable: variables.get(&3).unwrap()[0].clone(),
                                depth: 1,
                            }],
                            variable_list.variables_by_scope(1, 3).unwrap().variables()
                        );
                    });
            });
        })
        .unwrap();

    let shutdown_client = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_client(&client.id(), cx)
        })
    });

    shutdown_client.await.unwrap();
}
