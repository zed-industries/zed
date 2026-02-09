use crate::{
    debugger_panel::DebugPanel,
    session::running::stack_frame_list::{
        StackFrameEntry, StackFrameFilter, stack_frame_filter_key,
    },
    tests::{active_debug_session_panel, init_test, init_test_workspace, start_debug_session},
};
use dap::{
    StackFrame,
    requests::{Scopes, StackTrace, Threads},
};
use db::kvp::KEY_VALUE_STORE;
use editor::{Editor, ToPoint as _};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use project::{FakeFs, Project};
use serde_json::json;
use std::sync::Arc;
use unindent::Unindent as _;
use util::{path, rel_path::rel_path};

#[gpui::test]
async fn test_fetch_initial_stack_frames_and_go_to_stack_frame(
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
    client.on_request::<Scopes, _>(move |_, _| Ok(dap::ScopesResponse { scopes: vec![] }));

    client.on_request::<Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

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

    // trigger to load threads
    active_debug_session_panel(workspace, cx).update(cx, |session, cx| {
        session.running_state().update(cx, |running_state, cx| {
            running_state
                .session()
                .update(cx, |session, cx| session.threads(cx));
        });
    });

    cx.run_until_parked();

    // select first thread
    active_debug_session_panel(workspace, cx).update_in(cx, |session, window, cx| {
        session.running_state().update(cx, |running_state, cx| {
            running_state.select_current_thread(
                &running_state
                    .session()
                    .update(cx, |session, cx| session.threads(cx)),
                window,
                cx,
            );
        });
    });

    cx.run_until_parked();

    active_debug_session_panel(workspace, cx).update(cx, |session, cx| {
        let stack_frame_list = session
            .running_state()
            .update(cx, |state, _| state.stack_frame_list().clone());

        stack_frame_list.update(cx, |stack_frame_list, cx| {
            assert_eq!(Some(1), stack_frame_list.opened_stack_frame_id());
            assert_eq!(stack_frames, stack_frame_list.dap_stack_frames(cx));
        });
    });
}

#[gpui::test]
async fn test_select_stack_frame(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    cx.executor().allow_parking();
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
    let _ = workspace.update(cx, |workspace, window, cx| {
        workspace.toggle_dock(workspace::dock::DockPosition::Bottom, window, cx);
    });

    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client.on_request::<Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

    client.on_request::<Scopes, _>(move |_, _| Ok(dap::ScopesResponse { scopes: vec![] }));

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

    // trigger threads to load
    active_debug_session_panel(workspace, cx).update(cx, |session, cx| {
        session.running_state().update(cx, |running_state, cx| {
            running_state
                .session()
                .update(cx, |session, cx| session.threads(cx));
        });
    });

    cx.run_until_parked();

    // select first thread
    active_debug_session_panel(workspace, cx).update_in(cx, |session, window, cx| {
        session.running_state().update(cx, |running_state, cx| {
            running_state.select_current_thread(
                &running_state
                    .session()
                    .update(cx, |session, cx| session.threads(cx)),
                window,
                cx,
            );
        });
    });

    cx.run_until_parked();

    workspace
        .update(cx, |workspace, window, cx| {
            let editors = workspace.items_of_type::<Editor>(cx).collect::<Vec<_>>();
            assert_eq!(1, editors.len());

            let project_path = editors[0]
                .update(cx, |editor, cx| editor.project_path(cx))
                .unwrap();
            assert_eq!(rel_path("src/test.js"), project_path.path.as_ref());
            assert_eq!(test_file_content, editors[0].read(cx).text(cx));
            assert_eq!(
                vec![2..3],
                editors[0].update(cx, |editor, cx| {
                    let snapshot = editor.snapshot(window, cx);

                    editor
                        .highlighted_rows::<editor::ActiveDebugLine>()
                        .map(|(range, _)| {
                            let start = range.start.to_point(&snapshot.buffer_snapshot());
                            let end = range.end.to_point(&snapshot.buffer_snapshot());
                            start.row..end.row
                        })
                        .collect::<Vec<_>>()
                })
            );
        })
        .unwrap();

    let stack_frame_list = workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_debug_panel_item = debug_panel
                .update(cx, |this, _| this.active_session())
                .unwrap();

            active_debug_panel_item
                .read(cx)
                .running_state()
                .read(cx)
                .stack_frame_list()
                .clone()
        })
        .unwrap();

    stack_frame_list.update(cx, |stack_frame_list, cx| {
        assert_eq!(Some(1), stack_frame_list.opened_stack_frame_id());
        assert_eq!(stack_frames, stack_frame_list.dap_stack_frames(cx));
    });

    // select second stack frame
    stack_frame_list
        .update_in(cx, |stack_frame_list, window, cx| {
            stack_frame_list.go_to_stack_frame(stack_frames[1].id, window, cx)
        })
        .await
        .unwrap();

    cx.run_until_parked();

    stack_frame_list.update(cx, |stack_frame_list, cx| {
        assert_eq!(Some(2), stack_frame_list.opened_stack_frame_id());
        assert_eq!(stack_frames, stack_frame_list.dap_stack_frames(cx));
    });

    let _ = workspace.update(cx, |workspace, window, cx| {
        let editors = workspace.items_of_type::<Editor>(cx).collect::<Vec<_>>();
        assert_eq!(1, editors.len());

        let project_path = editors[0]
            .update(cx, |editor, cx| editor.project_path(cx))
            .unwrap();
        assert_eq!(rel_path("src/module.js"), project_path.path.as_ref());
        assert_eq!(module_file_content, editors[0].read(cx).text(cx));
        assert_eq!(
            vec![0..1],
            editors[0].update(cx, |editor, cx| {
                let snapshot = editor.snapshot(window, cx);

                editor
                    .highlighted_rows::<editor::ActiveDebugLine>()
                    .map(|(range, _)| {
                        let start = range.start.to_point(&snapshot.buffer_snapshot());
                        let end = range.end.to_point(&snapshot.buffer_snapshot());
                        start.row..end.row
                    })
                    .collect::<Vec<_>>()
            })
        );
    });
}

#[gpui::test]
async fn test_collapsed_entries(executor: BackgroundExecutor, cx: &mut TestAppContext) {
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

    client.on_request::<Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

    client.on_request::<Scopes, _>(move |_, _| Ok(dap::ScopesResponse { scopes: vec![] }));

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
                origin: Some("ignored".into()),
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
            presentation_hint: Some(dap::StackFramePresentationHint::Deemphasize),
        },
        StackFrame {
            id: 3,
            name: "Stack Frame 3".into(),
            source: Some(dap::Source {
                name: Some("module.js".into()),
                path: Some(path!("/project/src/module.js").into()),
                source_reference: None,
                presentation_hint: None,
                origin: Some("ignored".into()),
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
            presentation_hint: Some(dap::StackFramePresentationHint::Deemphasize),
        },
        StackFrame {
            id: 4,
            name: "Stack Frame 4".into(),
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
        StackFrame {
            id: 5,
            name: "Stack Frame 5".into(),
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
            presentation_hint: Some(dap::StackFramePresentationHint::Deemphasize),
        },
        StackFrame {
            id: 6,
            name: "Stack Frame 6".into(),
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
            presentation_hint: Some(dap::StackFramePresentationHint::Deemphasize),
        },
        StackFrame {
            id: 7,
            name: "Stack Frame 7".into(),
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

    // trigger threads to load
    active_debug_session_panel(workspace, cx).update(cx, |session, cx| {
        session.running_state().update(cx, |running_state, cx| {
            running_state
                .session()
                .update(cx, |session, cx| session.threads(cx));
        });
    });

    cx.run_until_parked();

    // select first thread
    active_debug_session_panel(workspace, cx).update_in(cx, |session, window, cx| {
        session.running_state().update(cx, |running_state, cx| {
            running_state.select_current_thread(
                &running_state
                    .session()
                    .update(cx, |session, cx| session.threads(cx)),
                window,
                cx,
            );
        });
    });

    cx.run_until_parked();

    // trigger stack frames to loaded
    active_debug_session_panel(workspace, cx).update(cx, |debug_panel_item, cx| {
        let stack_frame_list = debug_panel_item
            .running_state()
            .update(cx, |state, _| state.stack_frame_list().clone());

        stack_frame_list.update(cx, |stack_frame_list, cx| {
            stack_frame_list.dap_stack_frames(cx);
        });
    });

    cx.run_until_parked();

    active_debug_session_panel(workspace, cx).update_in(cx, |debug_panel_item, window, cx| {
        let stack_frame_list = debug_panel_item
            .running_state()
            .update(cx, |state, _| state.stack_frame_list().clone());

        stack_frame_list.update(cx, |stack_frame_list, cx| {
            stack_frame_list.build_entries(true, window, cx);

            assert_eq!(
                &vec![
                    StackFrameEntry::Normal(stack_frames[0].clone()),
                    StackFrameEntry::Collapsed(vec![
                        stack_frames[1].clone(),
                        stack_frames[2].clone()
                    ]),
                    StackFrameEntry::Normal(stack_frames[3].clone()),
                    StackFrameEntry::Collapsed(vec![
                        stack_frames[4].clone(),
                        stack_frames[5].clone()
                    ]),
                    StackFrameEntry::Normal(stack_frames[6].clone()),
                ],
                stack_frame_list.entries()
            );

            stack_frame_list.expand_collapsed_entry(1, cx);

            assert_eq!(
                &vec![
                    StackFrameEntry::Normal(stack_frames[0].clone()),
                    StackFrameEntry::Normal(stack_frames[1].clone()),
                    StackFrameEntry::Normal(stack_frames[2].clone()),
                    StackFrameEntry::Normal(stack_frames[3].clone()),
                    StackFrameEntry::Collapsed(vec![
                        stack_frames[4].clone(),
                        stack_frames[5].clone()
                    ]),
                    StackFrameEntry::Normal(stack_frames[6].clone()),
                ],
                stack_frame_list.entries()
            );

            stack_frame_list.expand_collapsed_entry(4, cx);

            assert_eq!(
                &vec![
                    StackFrameEntry::Normal(stack_frames[0].clone()),
                    StackFrameEntry::Normal(stack_frames[1].clone()),
                    StackFrameEntry::Normal(stack_frames[2].clone()),
                    StackFrameEntry::Normal(stack_frames[3].clone()),
                    StackFrameEntry::Normal(stack_frames[4].clone()),
                    StackFrameEntry::Normal(stack_frames[5].clone()),
                    StackFrameEntry::Normal(stack_frames[6].clone()),
                ],
                stack_frame_list.entries()
            );
        });
    });
}

#[gpui::test]
async fn test_stack_frame_filter(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    let test_file_content = r#"
        function main() {
            doSomething();
        }

        function doSomething() {
            console.log('doing something');
        }
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
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client.on_request::<Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

    client.on_request::<Scopes, _>(move |_, _| Ok(dap::ScopesResponse { scopes: vec![] }));

    let stack_frames = vec![
        StackFrame {
            id: 1,
            name: "main".into(),
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
            line: 2,
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
            name: "node:internal/modules/cjs/loader".into(),
            source: Some(dap::Source {
                name: Some("loader.js".into()),
                path: Some(path!("/usr/lib/node/internal/modules/cjs/loader.js").into()),
                source_reference: None,
                presentation_hint: None,
                origin: None,
                sources: None,
                adapter_data: None,
                checksums: None,
            }),
            line: 100,
            column: 1,
            end_line: None,
            end_column: None,
            can_restart: None,
            instruction_pointer_reference: None,
            module_id: None,
            presentation_hint: Some(dap::StackFramePresentationHint::Deemphasize),
        },
        StackFrame {
            id: 3,
            name: "node:internal/modules/run_main".into(),
            source: Some(dap::Source {
                name: Some("run_main.js".into()),
                path: Some(path!("/usr/lib/node/internal/modules/run_main.js").into()),
                source_reference: None,
                presentation_hint: None,
                origin: None,
                sources: None,
                adapter_data: None,
                checksums: None,
            }),
            line: 50,
            column: 1,
            end_line: None,
            end_column: None,
            can_restart: None,
            instruction_pointer_reference: None,
            module_id: None,
            presentation_hint: Some(dap::StackFramePresentationHint::Deemphasize),
        },
        StackFrame {
            id: 4,
            name: "node:internal/modules/run_main2".into(),
            source: Some(dap::Source {
                name: Some("run_main.js".into()),
                path: Some(path!("/usr/lib/node/internal/modules/run_main2.js").into()),
                source_reference: None,
                presentation_hint: None,
                origin: None,
                sources: None,
                adapter_data: None,
                checksums: None,
            }),
            line: 50,
            column: 1,
            end_line: None,
            end_column: None,
            can_restart: None,
            instruction_pointer_reference: None,
            module_id: None,
            presentation_hint: Some(dap::StackFramePresentationHint::Deemphasize),
        },
        StackFrame {
            id: 5,
            name: "doSomething".into(),
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
    ];

    // Store a copy for assertions
    let stack_frames_for_assertions = stack_frames.clone();

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

    // trigger threads to load
    active_debug_session_panel(workspace, cx).update(cx, |session, cx| {
        session.running_state().update(cx, |running_state, cx| {
            running_state
                .session()
                .update(cx, |session, cx| session.threads(cx));
        });
    });

    cx.run_until_parked();

    // select first thread
    active_debug_session_panel(workspace, cx).update_in(cx, |session, window, cx| {
        session.running_state().update(cx, |running_state, cx| {
            running_state.select_current_thread(
                &running_state
                    .session()
                    .update(cx, |session, cx| session.threads(cx)),
                window,
                cx,
            );
        });
    });

    cx.run_until_parked();

    // trigger stack frames to load
    active_debug_session_panel(workspace, cx).update(cx, |debug_panel_item, cx| {
        let stack_frame_list = debug_panel_item
            .running_state()
            .update(cx, |state, _| state.stack_frame_list().clone());

        stack_frame_list.update(cx, |stack_frame_list, cx| {
            stack_frame_list.dap_stack_frames(cx);
        });
    });

    cx.run_until_parked();

    let stack_frame_list =
        active_debug_session_panel(workspace, cx).update_in(cx, |debug_panel_item, window, cx| {
            let stack_frame_list = debug_panel_item
                .running_state()
                .update(cx, |state, _| state.stack_frame_list().clone());

            stack_frame_list.update(cx, |stack_frame_list, cx| {
                stack_frame_list.build_entries(true, window, cx);

                // Verify we have the expected collapsed structure
                assert_eq!(
                    stack_frame_list.entries(),
                    &vec![
                        StackFrameEntry::Normal(stack_frames_for_assertions[0].clone()),
                        StackFrameEntry::Collapsed(vec![
                            stack_frames_for_assertions[1].clone(),
                            stack_frames_for_assertions[2].clone(),
                            stack_frames_for_assertions[3].clone()
                        ]),
                        StackFrameEntry::Normal(stack_frames_for_assertions[4].clone()),
                    ]
                );
            });

            stack_frame_list
        });

    stack_frame_list.update(cx, |stack_frame_list, cx| {
        let all_frames = stack_frame_list.flatten_entries(true, false);
        assert_eq!(all_frames.len(), 5, "Should see all 5 frames initially");

        stack_frame_list
            .toggle_frame_filter(Some(project::debugger::session::ThreadStatus::Stopped), cx);
        assert_eq!(
            stack_frame_list.list_filter(),
            StackFrameFilter::OnlyUserFrames
        );
    });

    stack_frame_list.update(cx, |stack_frame_list, cx| {
        let user_frames = stack_frame_list.dap_stack_frames(cx);
        assert_eq!(user_frames.len(), 2, "Should only see 2 user frames");
        assert_eq!(user_frames[0].name, "main");
        assert_eq!(user_frames[1].name, "doSomething");

        // Toggle back to all frames
        stack_frame_list
            .toggle_frame_filter(Some(project::debugger::session::ThreadStatus::Stopped), cx);
        assert_eq!(stack_frame_list.list_filter(), StackFrameFilter::All);
    });

    stack_frame_list.update(cx, |stack_frame_list, cx| {
        let all_frames_again = stack_frame_list.flatten_entries(true, false);
        assert_eq!(
            all_frames_again.len(),
            5,
            "Should see all 5 frames after toggling back"
        );

        // Test 3: Verify collapsed entries stay expanded
        stack_frame_list.expand_collapsed_entry(1, cx);
        assert_eq!(
            stack_frame_list.entries(),
            &vec![
                StackFrameEntry::Normal(stack_frames_for_assertions[0].clone()),
                StackFrameEntry::Normal(stack_frames_for_assertions[1].clone()),
                StackFrameEntry::Normal(stack_frames_for_assertions[2].clone()),
                StackFrameEntry::Normal(stack_frames_for_assertions[3].clone()),
                StackFrameEntry::Normal(stack_frames_for_assertions[4].clone()),
            ]
        );

        stack_frame_list
            .toggle_frame_filter(Some(project::debugger::session::ThreadStatus::Stopped), cx);
        assert_eq!(
            stack_frame_list.list_filter(),
            StackFrameFilter::OnlyUserFrames
        );
    });

    stack_frame_list.update(cx, |stack_frame_list, cx| {
        stack_frame_list
            .toggle_frame_filter(Some(project::debugger::session::ThreadStatus::Stopped), cx);
        assert_eq!(stack_frame_list.list_filter(), StackFrameFilter::All);
    });

    stack_frame_list.update(cx, |stack_frame_list, cx| {
        stack_frame_list
            .toggle_frame_filter(Some(project::debugger::session::ThreadStatus::Stopped), cx);
        assert_eq!(
            stack_frame_list.list_filter(),
            StackFrameFilter::OnlyUserFrames
        );

        assert_eq!(
            stack_frame_list.dap_stack_frames(cx).as_slice(),
            &[
                stack_frames_for_assertions[0].clone(),
                stack_frames_for_assertions[4].clone()
            ]
        );

        // Verify entries remain expanded
        assert_eq!(
            stack_frame_list.entries(),
            &vec![
                StackFrameEntry::Normal(stack_frames_for_assertions[0].clone()),
                StackFrameEntry::Normal(stack_frames_for_assertions[1].clone()),
                StackFrameEntry::Normal(stack_frames_for_assertions[2].clone()),
                StackFrameEntry::Normal(stack_frames_for_assertions[3].clone()),
                StackFrameEntry::Normal(stack_frames_for_assertions[4].clone()),
            ],
            "Expanded entries should remain expanded after toggling filter"
        );
    });
}

#[gpui::test]
async fn test_stack_frame_filter_persistence(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
           "src": {
               "test.js": "function main() { console.log('hello'); }",
           }
        }),
    )
    .await;

    let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    workspace
        .update(cx, |workspace, _, _| {
            workspace.set_random_database_id();
        })
        .unwrap();

    let threads_response = dap::ThreadsResponse {
        threads: vec![dap::Thread {
            id: 1,
            name: "Thread 1".into(),
        }],
    };

    let stack_trace_response = dap::StackTraceResponse {
        stack_frames: vec![StackFrame {
            id: 1,
            name: "main".into(),
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
        }],
        total_frames: None,
    };

    let stopped_event = dap::StoppedEvent {
        reason: dap::StoppedEventReason::Pause,
        description: None,
        thread_id: Some(1),
        preserve_focus_hint: None,
        text: None,
        all_threads_stopped: None,
        hit_breakpoint_ids: None,
    };

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());
    let adapter_name = session.update(cx, |session, _| session.adapter());

    client.on_request::<Threads, _>({
        let threads_response = threads_response.clone();
        move |_, _| Ok(threads_response.clone())
    });

    client.on_request::<Scopes, _>(move |_, _| Ok(dap::ScopesResponse { scopes: vec![] }));

    client.on_request::<StackTrace, _>({
        let stack_trace_response = stack_trace_response.clone();
        move |_, _| Ok(stack_trace_response.clone())
    });

    client
        .fake_event(dap::messages::Events::Stopped(stopped_event.clone()))
        .await;

    cx.run_until_parked();

    let stack_frame_list =
        active_debug_session_panel(workspace, cx).update(cx, |debug_panel_item, cx| {
            debug_panel_item
                .running_state()
                .update(cx, |state, _| state.stack_frame_list().clone())
        });

    stack_frame_list.update(cx, |stack_frame_list, _cx| {
        assert_eq!(
            stack_frame_list.list_filter(),
            StackFrameFilter::All,
            "Initial filter should be All"
        );
    });

    stack_frame_list.update(cx, |stack_frame_list, cx| {
        stack_frame_list
            .toggle_frame_filter(Some(project::debugger::session::ThreadStatus::Stopped), cx);
        assert_eq!(
            stack_frame_list.list_filter(),
            StackFrameFilter::OnlyUserFrames,
            "Filter should be OnlyUserFrames after toggle"
        );
    });

    cx.run_until_parked();

    let workspace_id = workspace
        .update(cx, |workspace, _window, _cx| workspace.database_id())
        .ok()
        .flatten()
        .expect("workspace id has to be some for this test to work properly");

    let key = stack_frame_filter_key(&adapter_name, workspace_id);
    let stored_value = KEY_VALUE_STORE.read_kvp(&key).unwrap();
    assert_eq!(
        stored_value,
        Some(StackFrameFilter::OnlyUserFrames.into()),
        "Filter should be persisted in KVP store with key: {}",
        key
    );

    client
        .fake_event(dap::messages::Events::Terminated(None))
        .await;
    cx.run_until_parked();

    let session2 = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client2 = session2.update(cx, |session, _| session.adapter_client().unwrap());

    client2.on_request::<Threads, _>({
        let threads_response = threads_response.clone();
        move |_, _| Ok(threads_response.clone())
    });

    client2.on_request::<Scopes, _>(move |_, _| Ok(dap::ScopesResponse { scopes: vec![] }));

    client2.on_request::<StackTrace, _>({
        let stack_trace_response = stack_trace_response.clone();
        move |_, _| Ok(stack_trace_response.clone())
    });

    client2
        .fake_event(dap::messages::Events::Stopped(stopped_event.clone()))
        .await;

    cx.run_until_parked();

    let stack_frame_list2 =
        active_debug_session_panel(workspace, cx).update(cx, |debug_panel_item, cx| {
            debug_panel_item
                .running_state()
                .update(cx, |state, _| state.stack_frame_list().clone())
        });

    stack_frame_list2.update(cx, |stack_frame_list, _cx| {
        assert_eq!(
            stack_frame_list.list_filter(),
            StackFrameFilter::OnlyUserFrames,
            "Filter should be restored from KVP store in new session"
        );
    });
}
