use crate::*;
use dap::{
    client::SessionId,
    requests::{
        Continue, Disconnect, Launch, Next, RunInTerminal, SetBreakpoints, StackTrace,
        StartDebugging, StepBack, StepIn, StepOut, Threads,
    },
    DebugRequestType, ErrorResponse, RunInTerminalRequestArguments, SourceBreakpoint,
    StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest,
};
use editor::{
    actions::{self},
    Editor, EditorMode, MultiBuffer,
};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use project::{
    debugger::session::{ThreadId, ThreadStatus},
    FakeFs, Project,
};
use serde_json::json;
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use terminal_view::{terminal_panel::TerminalPanel, TerminalView};
use tests::{active_debug_session_panel, init_test, init_test_workspace};
use util::path;
use workspace::{dock::Panel, Item};

#[gpui::test]
async fn test_basic_show_debug_panel(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        "/project",
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, ["/project".as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.start_debug_session(dap::test_config(DebugRequestType::Launch, None, None), cx)
    });

    let session = task.await.unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client
        .on_request::<Threads, _>(move |_, _| {
            Ok(dap::ThreadsResponse {
                threads: vec![dap::Thread {
                    id: 1,
                    name: "Thread 1".into(),
                }],
            })
        })
        .await;

    client
        .on_request::<StackTrace, _>(move |_, _| {
            Ok(dap::StackTraceResponse {
                stack_frames: Vec::default(),
                total_frames: None,
            })
        })
        .await;

    // assert we have a debug panel item before the session has stopped
    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_session = debug_panel.update(cx, |debug_panel, cx| {
                debug_panel.active_session(cx).unwrap()
            });

            let running_state = active_session.update(cx, |active_session, _| {
                active_session
                    .mode()
                    .as_running()
                    .expect("Session should be running by this point")
                    .clone()
            });

            debug_panel.update(cx, |this, cx| {
                assert!(this.active_session(cx).is_some());
                // we have one active session and one inert item
                assert_eq!(2, this.pane().unwrap().read(cx).items_len());
                assert!(running_state.read(cx).selected_thread_id().is_none());
            });
        })
        .unwrap();

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
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_session = debug_panel
                .update(cx, |this, cx| this.active_session(cx))
                .unwrap();

            let running_state = active_session.update(cx, |active_session, _| {
                active_session
                    .mode()
                    .as_running()
                    .expect("Session should be running by this point")
                    .clone()
            });

            // we have one active session and one inert item
            assert_eq!(
                2,
                debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
            );
            assert_eq!(client.id(), running_state.read(cx).session_id());
            assert_eq!(
                ThreadId(1),
                running_state.read(cx).selected_thread_id().unwrap()
            );
        })
        .unwrap();

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();

    // assert we still have a debug panel item after the client shutdown
    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

            let active_session = debug_panel
                .update(cx, |this, cx| this.active_session(cx))
                .unwrap();

            let running_state = active_session.update(cx, |active_session, _| {
                active_session
                    .mode()
                    .as_running()
                    .expect("Session should be running by this point")
                    .clone()
            });

            debug_panel.update(cx, |this, cx| {
                assert!(this.active_session(cx).is_some());
                assert_eq!(2, this.pane().unwrap().read(cx).items_len());
                assert_eq!(
                    ThreadId(1),
                    running_state.read(cx).selected_thread_id().unwrap()
                );
            });
        })
        .unwrap();
}

#[gpui::test]
async fn test_we_can_only_have_one_panel_per_debug_session(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        "/project",
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, ["/project".as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.start_debug_session(dap::test_config(DebugRequestType::Launch, None, None), cx)
    });

    let session = task.await.unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client
        .on_request::<Threads, _>(move |_, _| {
            Ok(dap::ThreadsResponse {
                threads: vec![dap::Thread {
                    id: 1,
                    name: "Thread 1".into(),
                }],
            })
        })
        .await;

    client
        .on_request::<StackTrace, _>(move |_, _| {
            Ok(dap::StackTraceResponse {
                stack_frames: Vec::default(),
                total_frames: None,
            })
        })
        .await;

    // assert we have a debug panel item before the session has stopped
    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

            debug_panel.update(cx, |this, cx| {
                assert!(this.active_session(cx).is_some());
                // we have one active session and one inert item
                assert_eq!(2, this.pane().unwrap().read(cx).items_len());
            });
        })
        .unwrap();

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

    // assert we added a debug panel item
    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_session = debug_panel
                .update(cx, |this, cx| this.active_session(cx))
                .unwrap();

            let running_state = active_session.update(cx, |active_session, _| {
                active_session
                    .mode()
                    .as_running()
                    .expect("Session should be running by this point")
                    .clone()
            });

            // we have one active session and one inert item
            assert_eq!(
                2,
                debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
            );
            assert_eq!(client.id(), active_session.read(cx).session_id(cx).unwrap());
            assert_eq!(
                ThreadId(1),
                running_state.read(cx).selected_thread_id().unwrap()
            );
        })
        .unwrap();

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Pause,
            description: None,
            thread_id: Some(2),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_session = debug_panel
                .update(cx, |this, cx| this.active_session(cx))
                .unwrap();

            let running_state = active_session.update(cx, |active_session, _| {
                active_session
                    .mode()
                    .as_running()
                    .expect("Session should be running by this point")
                    .clone()
            });

            // we have one active session and one inert item
            assert_eq!(
                2,
                debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
            );
            assert_eq!(client.id(), active_session.read(cx).session_id(cx).unwrap());
            assert_eq!(
                ThreadId(1),
                running_state.read(cx).selected_thread_id().unwrap()
            );
        })
        .unwrap();

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();

    // assert we still have a debug panel item after the client shutdown
    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_session = debug_panel
                .update(cx, |this, cx| this.active_session(cx))
                .unwrap();

            let running_state = active_session.update(cx, |active_session, _| {
                active_session
                    .mode()
                    .as_running()
                    .expect("Session should be running by this point")
                    .clone()
            });

            debug_panel.update(cx, |this, cx| {
                assert!(this.active_session(cx).is_some());
                assert_eq!(2, this.pane().unwrap().read(cx).items_len());
                assert_eq!(
                    ThreadId(1),
                    running_state.read(cx).selected_thread_id().unwrap()
                );
            });
        })
        .unwrap();
}

#[gpui::test]
async fn test_handle_successful_run_in_terminal_reverse_request(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let send_response = Arc::new(AtomicBool::new(false));

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        "/project",
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, ["/project".as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.start_debug_session(dap::test_config(DebugRequestType::Launch, None, None), cx)
    });

    let session = task.await.unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client
        .on_response::<RunInTerminal, _>({
            let send_response = send_response.clone();
            move |response| {
                send_response.store(true, Ordering::SeqCst);

                assert!(response.success);
                assert!(response.body.is_some());
            }
        })
        .await;

    client
        .fake_reverse_request::<RunInTerminal>(RunInTerminalRequestArguments {
            kind: None,
            title: None,
            cwd: std::env::temp_dir().to_string_lossy().to_string(),
            args: vec![],
            env: None,
            args_can_be_interpreted_by_shell: None,
        })
        .await;

    cx.run_until_parked();

    assert!(
        send_response.load(std::sync::atomic::Ordering::SeqCst),
        "Expected to receive response from reverse request"
    );

    workspace
        .update(cx, |workspace, _window, cx| {
            let terminal_panel = workspace.panel::<TerminalPanel>(cx).unwrap();

            let panel = terminal_panel.read(cx).pane().unwrap().read(cx);

            assert_eq!(1, panel.items_len());
            assert!(panel
                .active_item()
                .unwrap()
                .downcast::<TerminalView>()
                .unwrap()
                .read(cx)
                .terminal()
                .read(cx)
                .debug_terminal());
        })
        .unwrap();

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}

// // covers that we always send a response back, if something when wrong,
// // while spawning the terminal
#[gpui::test]
async fn test_handle_error_run_in_terminal_reverse_request(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let send_response = Arc::new(AtomicBool::new(false));

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        "/project",
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, ["/project".as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.start_debug_session(dap::test_config(DebugRequestType::Launch, None, None), cx)
    });

    let session = task.await.unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client
        .on_response::<RunInTerminal, _>({
            let send_response = send_response.clone();
            move |response| {
                send_response.store(true, Ordering::SeqCst);

                assert!(!response.success);
                assert!(response.body.is_some());
            }
        })
        .await;

    client
        .fake_reverse_request::<RunInTerminal>(RunInTerminalRequestArguments {
            kind: None,
            title: None,
            cwd: "/non-existing/path".into(), // invalid/non-existing path will cause the terminal spawn to fail
            args: vec![],
            env: None,
            args_can_be_interpreted_by_shell: None,
        })
        .await;

    cx.run_until_parked();

    assert!(
        send_response.load(std::sync::atomic::Ordering::SeqCst),
        "Expected to receive response from reverse request"
    );

    workspace
        .update(cx, |workspace, _window, cx| {
            let terminal_panel = workspace.panel::<TerminalPanel>(cx).unwrap();

            assert_eq!(
                0,
                terminal_panel.read(cx).pane().unwrap().read(cx).items_len()
            );
        })
        .unwrap();

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}

#[gpui::test]
async fn test_handle_start_debugging_reverse_request(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let send_response = Arc::new(AtomicBool::new(false));

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        "/project",
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, ["/project".as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.start_debug_session(dap::test_config(DebugRequestType::Launch, None, None), cx)
    });

    let session = task.await.unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    client
        .on_request::<dap::requests::Threads, _>(move |_, _| {
            Ok(dap::ThreadsResponse {
                threads: vec![dap::Thread {
                    id: 1,
                    name: "Thread 1".into(),
                }],
            })
        })
        .await;

    client
        .on_response::<StartDebugging, _>({
            let send_response = send_response.clone();
            move |response| {
                send_response.store(true, Ordering::SeqCst);

                assert!(response.success);
                assert!(response.body.is_some());
            }
        })
        .await;

    client
        .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
            configuration: json!({}),
            request: StartDebuggingRequestArgumentsRequest::Launch,
        })
        .await;

    cx.run_until_parked();

    let child_session = project.update(cx, |project, cx| {
        project
            .dap_store()
            .read(cx)
            .session_by_id(SessionId(1))
            .unwrap()
    });
    let child_client = child_session.update(cx, |session, _| session.adapter_client().unwrap());

    client
        .on_request::<dap::requests::Threads, _>(move |_, _| {
            Ok(dap::ThreadsResponse {
                threads: vec![dap::Thread {
                    id: 1,
                    name: "Thread 1".into(),
                }],
            })
        })
        .await;

    child_client
        .on_request::<Disconnect, _>(move |_, _| Ok(()))
        .await;

    child_client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Pause,
            description: None,
            thread_id: Some(2),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    assert!(
        send_response.load(std::sync::atomic::Ordering::SeqCst),
        "Expected to receive response from reverse request"
    );

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(child_session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}

#[gpui::test]
async fn test_debug_panel_item_thread_status_reset_on_failure(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        "/project",
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, ["/project".as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.start_debug_session(
            dap::test_config(
                DebugRequestType::Launch,
                None,
                Some(dap::Capabilities {
                    supports_step_back: Some(true),
                    ..Default::default()
                }),
            ),
            cx,
        )
    });

    let session = task.await.unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());
    const THREAD_ID_NUM: u64 = 1;

    client
        .on_request::<dap::requests::Threads, _>(move |_, _| {
            Ok(dap::ThreadsResponse {
                threads: vec![dap::Thread {
                    id: THREAD_ID_NUM,
                    name: "Thread 1".into(),
                }],
            })
        })
        .await;

    client.on_request::<Launch, _>(move |_, _| Ok(())).await;

    client
        .on_request::<StackTrace, _>(move |_, _| {
            Ok(dap::StackTraceResponse {
                stack_frames: Vec::default(),
                total_frames: None,
            })
        })
        .await;

    client
        .on_request::<Next, _>(move |_, _| {
            Err(ErrorResponse {
                error: Some(dap::Message {
                    id: 1,
                    format: "error".into(),
                    variables: None,
                    send_telemetry: None,
                    show_user: None,
                    url: None,
                    url_label: None,
                }),
            })
        })
        .await;

    client
        .on_request::<StepOut, _>(move |_, _| {
            Err(ErrorResponse {
                error: Some(dap::Message {
                    id: 1,
                    format: "error".into(),
                    variables: None,
                    send_telemetry: None,
                    show_user: None,
                    url: None,
                    url_label: None,
                }),
            })
        })
        .await;

    client
        .on_request::<StepIn, _>(move |_, _| {
            Err(ErrorResponse {
                error: Some(dap::Message {
                    id: 1,
                    format: "error".into(),
                    variables: None,
                    send_telemetry: None,
                    show_user: None,
                    url: None,
                    url_label: None,
                }),
            })
        })
        .await;

    client
        .on_request::<StepBack, _>(move |_, _| {
            Err(ErrorResponse {
                error: Some(dap::Message {
                    id: 1,
                    format: "error".into(),
                    variables: None,
                    send_telemetry: None,
                    show_user: None,
                    url: None,
                    url_label: None,
                }),
            })
        })
        .await;

    client
        .on_request::<Continue, _>(move |_, _| {
            Err(ErrorResponse {
                error: Some(dap::Message {
                    id: 1,
                    format: "error".into(),
                    variables: None,
                    send_telemetry: None,
                    show_user: None,
                    url: None,
                    url_label: None,
                }),
            })
        })
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

    let running_state = active_debug_session_panel(workspace, cx).update_in(cx, |item, _, _| {
        item.mode()
            .as_running()
            .expect("Session should be running by this point")
            .clone()
    });

    cx.run_until_parked();
    let thread_id = ThreadId(1);

    for operation in &[
        "step_over",
        "continue_thread",
        "step_back",
        "step_in",
        "step_out",
    ] {
        running_state.update(cx, |running_state, cx| match *operation {
            "step_over" => running_state.step_over(cx),
            "continue_thread" => running_state.continue_thread(cx),
            "step_back" => running_state.step_back(cx),
            "step_in" => running_state.step_in(cx),
            "step_out" => running_state.step_out(cx),
            _ => unreachable!(),
        });

        // Check that we step the thread status to the correct intermediate state
        running_state.update(cx, |running_state, cx| {
            assert_eq!(
                running_state
                    .thread_status(cx)
                    .expect("There should be an active thread selected"),
                match *operation {
                    "continue_thread" => ThreadStatus::Running,
                    _ => ThreadStatus::Stepping,
                },
                "Thread status was not set to correct intermediate state after {} request",
                operation
            );
        });

        cx.run_until_parked();

        running_state.update(cx, |running_state, cx| {
            assert_eq!(
                running_state
                    .thread_status(cx)
                    .expect("There should be an active thread selected"),
                ThreadStatus::Stopped,
                "Thread status not reset to Stopped after failed {}",
                operation
            );

            // update state to running, so we can test it actually changes the status back to stopped
            running_state
                .session()
                .update(cx, |session, cx| session.continue_thread(thread_id, cx));
        });
    }

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}

#[gpui::test]
async fn test_send_breakpoints_when_editor_has_been_saved(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
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
    let project_path = Path::new(path!("/project"));
    let worktree = project
        .update(cx, |project, cx| project.find_worktree(project_path, cx))
        .expect("This worktree should exist in project")
        .0;

    let worktree_id = workspace
        .update(cx, |_, _, cx| worktree.read(cx).id())
        .unwrap();

    let task = project.update(cx, |project, cx| {
        project.start_debug_session(dap::test_config(DebugRequestType::Launch, None, None), cx)
    });

    let session = task.await.unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "main.rs"), cx)
        })
        .await
        .unwrap();

    let (editor, cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::Full,
            MultiBuffer::build_from_buffer(buffer, cx),
            Some(project.clone()),
            window,
            cx,
        )
    });

    client.on_request::<Launch, _>(move |_, _| Ok(())).await;

    client
        .on_request::<StackTrace, _>(move |_, _| {
            Ok(dap::StackTraceResponse {
                stack_frames: Vec::default(),
                total_frames: None,
            })
        })
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

    let called_set_breakpoints = Arc::new(AtomicBool::new(false));
    client
        .on_request::<SetBreakpoints, _>({
            let called_set_breakpoints = called_set_breakpoints.clone();
            move |_, args| {
                assert_eq!(path!("/project/main.rs"), args.source.path.unwrap());
                assert_eq!(
                    vec![SourceBreakpoint {
                        line: 2,
                        column: None,
                        condition: None,
                        hit_condition: None,
                        log_message: None,
                        mode: None
                    }],
                    args.breakpoints.unwrap()
                );
                assert!(!args.source_modified.unwrap());

                called_set_breakpoints.store(true, Ordering::SeqCst);

                Ok(dap::SetBreakpointsResponse {
                    breakpoints: Vec::default(),
                })
            }
        })
        .await;

    editor.update_in(cx, |editor, window, cx| {
        editor.move_down(&actions::MoveDown, window, cx);
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
    });

    cx.run_until_parked();

    assert!(
        called_set_breakpoints.load(std::sync::atomic::Ordering::SeqCst),
        "SetBreakpoint request must be called"
    );

    let called_set_breakpoints = Arc::new(AtomicBool::new(false));
    client
        .on_request::<SetBreakpoints, _>({
            let called_set_breakpoints = called_set_breakpoints.clone();
            move |_, args| {
                assert_eq!(path!("/project/main.rs"), args.source.path.unwrap());
                assert_eq!(
                    vec![SourceBreakpoint {
                        line: 3,
                        column: None,
                        condition: None,
                        hit_condition: None,
                        log_message: None,
                        mode: None
                    }],
                    args.breakpoints.unwrap()
                );
                assert!(args.source_modified.unwrap());

                called_set_breakpoints.store(true, Ordering::SeqCst);

                Ok(dap::SetBreakpointsResponse {
                    breakpoints: Vec::default(),
                })
            }
        })
        .await;

    editor.update_in(cx, |editor, window, cx| {
        editor.move_up(&actions::MoveUp, window, cx);
        editor.insert("new text\n", window, cx);
    });

    editor
        .update_in(cx, |editor, window, cx| {
            editor.save(true, project.clone(), window, cx)
        })
        .await
        .unwrap();

    cx.run_until_parked();

    assert!(
        called_set_breakpoints.load(std::sync::atomic::Ordering::SeqCst),
        "SetBreakpoint request must be called after editor is saved"
    );

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}

#[gpui::test]
async fn test_debug_session_is_shutdown_when_attach_and_launch_request_fails(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        "/project",
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, ["/project".as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.start_debug_session(
            dap::test_config(DebugRequestType::Launch, Some(true), None),
            cx,
        )
    });

    assert!(
        task.await.is_err(),
        "Session should failed to start if launch request fails"
    );

    cx.run_until_parked();

    project.update(cx, |project, cx| {
        assert!(
            project.dap_store().read(cx).sessions().count() == 0,
            "Session wouldn't exist if it was shutdown"
        );
    });
}
