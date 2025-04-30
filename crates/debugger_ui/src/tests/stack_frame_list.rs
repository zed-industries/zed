use crate::{
    debugger_panel::DebugPanel,
    session::running::stack_frame_list::StackFrameEntry,
    tests::{active_debug_session_panel, init_test, init_test_workspace, start_debug_session},
};
use dap::{
    StackFrame,
    requests::{Scopes, StackTrace, Threads},
};
use editor::{Editor, ToPoint as _};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use project::{FakeFs, Project};
use serde_json::json;
use std::sync::Arc;
use unindent::Unindent as _;
use util::path;

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
            assert_eq!(Some(1), stack_frame_list.selected_stack_frame_id());
            assert_eq!(stack_frames, stack_frame_list.dap_stack_frames(cx));
        });
    });
}

#[gpui::test]
async fn test_select_stack_frame(executor: BackgroundExecutor, cx: &mut TestAppContext) {
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
            let expected = if cfg!(target_os = "windows") {
                "src\\test.js"
            } else {
                "src/test.js"
            };
            assert_eq!(expected, project_path.path.to_string_lossy());
            assert_eq!(test_file_content, editors[0].read(cx).text(cx));
            assert_eq!(
                vec![2..3],
                editors[0].update(cx, |editor, cx| {
                    let snapshot = editor.snapshot(window, cx);

                    editor
                        .highlighted_rows::<editor::ActiveDebugLine>()
                        .map(|(range, _)| {
                            let start = range.start.to_point(&snapshot.buffer_snapshot);
                            let end = range.end.to_point(&snapshot.buffer_snapshot);
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
        assert_eq!(Some(1), stack_frame_list.selected_stack_frame_id());
        assert_eq!(stack_frames, stack_frame_list.dap_stack_frames(cx));
    });

    // select second stack frame
    stack_frame_list
        .update_in(cx, |stack_frame_list, window, cx| {
            stack_frame_list.select_stack_frame(&stack_frames[1], true, window, cx)
        })
        .await
        .unwrap();

    cx.run_until_parked();

    stack_frame_list.update(cx, |stack_frame_list, cx| {
        assert_eq!(Some(2), stack_frame_list.selected_stack_frame_id());
        assert_eq!(stack_frames, stack_frame_list.dap_stack_frames(cx));
    });

    let _ = workspace.update(cx, |workspace, window, cx| {
        let editors = workspace.items_of_type::<Editor>(cx).collect::<Vec<_>>();
        assert_eq!(1, editors.len());

        let project_path = editors[0]
            .update(cx, |editor, cx| editor.project_path(cx))
            .unwrap();
        let expected = if cfg!(target_os = "windows") {
            "src\\module.js"
        } else {
            "src/module.js"
        };
        assert_eq!(expected, project_path.path.to_string_lossy());
        assert_eq!(module_file_content, editors[0].read(cx).text(cx));
        assert_eq!(
            vec![0..1],
            editors[0].update(cx, |editor, cx| {
                let snapshot = editor.snapshot(window, cx);

                editor
                    .highlighted_rows::<editor::ActiveDebugLine>()
                    .map(|(range, _)| {
                        let start = range.start.to_point(&snapshot.buffer_snapshot);
                        let end = range.end.to_point(&snapshot.buffer_snapshot);
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

            stack_frame_list.expand_collapsed_entry(
                1,
                &vec![stack_frames[1].clone(), stack_frames[2].clone()],
                cx,
            );

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

            stack_frame_list.expand_collapsed_entry(
                4,
                &vec![stack_frames[4].clone(), stack_frames[5].clone()],
                cx,
            );

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
