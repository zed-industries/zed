use crate::{
    persistence::DebuggerPaneItem,
    tests::{start_debug_session, start_debug_session_with},
    *,
};
use dap::{
    ErrorResponse, Message, RunInTerminalRequestArguments, SourceBreakpoint,
    StartDebuggingRequestArguments, StartDebuggingRequestArgumentsRequest,
    adapters::DebugTaskDefinition,
    client::SessionId,
    requests::{
        Continue, Disconnect, Launch, Next, RunInTerminal, SetBreakpoints, StackTrace,
        StartDebugging, StepBack, StepIn, StepOut, Threads,
    },
};
use editor::{
    ActiveDebugLine, Editor, EditorMode, MultiBuffer,
    actions::{self},
};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use project::{
    FakeFs, Project,
    debugger::session::{ThreadId, ThreadStatus},
};
use serde_json::json;
use std::{
    path::Path,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
};
use terminal_view::terminal_panel::TerminalPanel;
use tests::{active_debug_session_panel, init_test, init_test_workspace};
use util::{path, rel_path::rel_path};
use workspace::item::SaveOptions;
use workspace::pane_group::SplitDirection;
use workspace::{Item, dock::Panel, move_active_item};

#[gpui::test]
async fn test_basic_show_debug_panel(executor: BackgroundExecutor, cx: &mut TestAppContext) {
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

    client.on_request::<StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: Vec::default(),
            total_frames: None,
        })
    });

    cx.run_until_parked();

    // assert we have a debug panel item before the session has stopped
    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_session =
                debug_panel.update(cx, |debug_panel, _| debug_panel.active_session().unwrap());

            let running_state = active_session.update(cx, |active_session, _| {
                active_session.running_state().clone()
            });

            debug_panel.update(cx, |this, cx| {
                assert!(this.active_session().is_some());
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
                .update(cx, |this, _| this.active_session())
                .unwrap();

            let running_state = active_session.update(cx, |active_session, _| {
                active_session.running_state().clone()
            });

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
                .update(cx, |this, _| this.active_session())
                .unwrap();

            let running_state = active_session.update(cx, |active_session, _| {
                active_session.running_state().clone()
            });

            debug_panel.update(cx, |this, cx| {
                assert!(this.active_session().is_some());
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
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
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

    client.on_request::<StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: Vec::default(),
            total_frames: None,
        })
    });

    cx.run_until_parked();

    // assert we have a debug panel item before the session has stopped
    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

            debug_panel.update(cx, |this, _| {
                assert!(this.active_session().is_some());
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
                .update(cx, |this, _| this.active_session())
                .unwrap();

            let running_state = active_session.update(cx, |active_session, _| {
                active_session.running_state().clone()
            });

            assert_eq!(client.id(), active_session.read(cx).session_id(cx));
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
                .update(cx, |this, _| this.active_session())
                .unwrap();

            let running_state = active_session.update(cx, |active_session, _| {
                active_session.running_state().clone()
            });

            assert_eq!(client.id(), active_session.read(cx).session_id(cx));
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
                .update(cx, |this, _| this.active_session())
                .unwrap();

            let running_state = active_session.update(cx, |active_session, _| {
                active_session.running_state().clone()
            });

            debug_panel.update(cx, |this, cx| {
                assert!(this.active_session().is_some());
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
    // needed because the debugger launches a terminal which starts a background PTY
    cx.executor().allow_parking();
    init_test(cx);

    let send_response = Arc::new(AtomicBool::new(false));

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

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
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
            cwd: std::env::temp_dir().to_string_lossy().into_owned(),
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
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let session = debug_panel.read(cx).active_session().unwrap();
            let running = session.read(cx).running_state();
            assert_eq!(
                running
                    .read(cx)
                    .pane_items_status(cx)
                    .get(&DebuggerPaneItem::Terminal),
                Some(&true)
            );
            assert!(running.read(cx).debug_terminal.read(cx).terminal.is_some());
        })
        .unwrap();
}

#[gpui::test]
async fn test_handle_start_debugging_request(
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

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    let fake_config = json!({"one": "two"});
    let launched_with = Arc::new(parking_lot::Mutex::new(None));

    let _subscription = project::debugger::test::intercept_debug_sessions(cx, {
        let launched_with = launched_with.clone();
        move |client| {
            let launched_with = launched_with.clone();
            client.on_request::<dap::requests::Launch, _>(move |_, args| {
                launched_with.lock().replace(args.raw);
                Ok(())
            });
            client.on_request::<dap::requests::Attach, _>(move |_, _| {
                assert!(false, "should not get attach request");
                Ok(())
            });
        }
    });

    let sessions = workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            debug_panel.read(cx).sessions().collect::<Vec<_>>()
        })
        .unwrap();
    assert_eq!(sessions.len(), 1);
    client
        .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
            request: StartDebuggingRequestArgumentsRequest::Launch,
            configuration: fake_config.clone(),
        })
        .await;

    cx.run_until_parked();

    workspace
        .update(cx, |workspace, _window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

            // Active session changes on spawn, as the parent has never stopped.
            let active_session = debug_panel
                .read(cx)
                .active_session()
                .unwrap()
                .read(cx)
                .session(cx);
            let current_sessions = debug_panel.read(cx).sessions().collect::<Vec<_>>();
            assert_eq!(active_session, current_sessions[1].read(cx).session(cx));
            assert_eq!(
                active_session.read(cx).parent_session(),
                Some(&current_sessions[0].read(cx).session(cx))
            );

            assert_eq!(current_sessions.len(), 2);
            assert_eq!(current_sessions[0], sessions[0]);

            let parent_session = current_sessions[1]
                .read(cx)
                .session(cx)
                .read(cx)
                .parent_session()
                .unwrap();
            assert_eq!(parent_session, &sessions[0].read(cx).session(cx));

            // We should preserve the original binary (params to spawn process etc.) except for launch params
            // (as they come from reverse spawn request).
            let mut original_binary = parent_session.read(cx).binary().cloned().unwrap();
            original_binary.request_args = StartDebuggingRequestArguments {
                request: StartDebuggingRequestArgumentsRequest::Launch,
                configuration: fake_config.clone(),
            };

            assert_eq!(
                current_sessions[1]
                    .read(cx)
                    .session(cx)
                    .read(cx)
                    .binary()
                    .unwrap(),
                &original_binary
            );
        })
        .unwrap();

    assert_eq!(&fake_config, launched_with.lock().as_ref().unwrap());
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
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
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
            cwd: "".into(),
            args: vec!["oops".into(), "oops".into()],
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
}

#[gpui::test]
async fn test_handle_start_debugging_reverse_request(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    cx.executor().allow_parking();
    init_test(cx);

    let send_response = Arc::new(AtomicBool::new(false));

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
    // Set up handlers for sessions spawned with reverse request too.
    let _reverse_request_subscription =
        project::debugger::test::intercept_debug_sessions(cx, |_| {});
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

    child_client.on_request::<dap::requests::Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

    child_client.on_request::<Disconnect, _>(move |_, _| Ok(()));

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
}

#[gpui::test]
async fn test_shutdown_children_when_parent_session_shutdown(
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
    let dap_store = project.update(cx, |project, _| project.dap_store());
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let parent_session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = parent_session.update(cx, |session, _| session.adapter_client().unwrap());

    client.on_request::<dap::requests::Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

    client.on_response::<StartDebugging, _>(move |_| {}).await;
    // Set up handlers for sessions spawned with reverse request too.
    let _reverse_request_subscription =
        project::debugger::test::intercept_debug_sessions(cx, |_| {});
    // start first child session
    client
        .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
            configuration: json!({}),
            request: StartDebuggingRequestArgumentsRequest::Launch,
        })
        .await;

    cx.run_until_parked();

    // start second child session
    client
        .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
            configuration: json!({}),
            request: StartDebuggingRequestArgumentsRequest::Launch,
        })
        .await;

    cx.run_until_parked();

    // configure first child session
    let first_child_session = dap_store.read_with(cx, |dap_store, _| {
        dap_store.session_by_id(SessionId(1)).unwrap()
    });
    let first_child_client =
        first_child_session.update(cx, |session, _| session.adapter_client().unwrap());

    first_child_client.on_request::<Disconnect, _>(move |_, _| Ok(()));

    // configure second child session
    let second_child_session = dap_store.read_with(cx, |dap_store, _| {
        dap_store.session_by_id(SessionId(2)).unwrap()
    });
    let second_child_client =
        second_child_session.update(cx, |session, _| session.adapter_client().unwrap());

    second_child_client.on_request::<Disconnect, _>(move |_, _| Ok(()));

    cx.run_until_parked();

    // shutdown parent session
    dap_store
        .update(cx, |dap_store, cx| {
            dap_store.shutdown_session(parent_session.read(cx).session_id(), cx)
        })
        .await
        .unwrap();

    // assert parent session and all children sessions are shutdown
    dap_store.update(cx, |dap_store, cx| {
        assert!(
            dap_store
                .session_by_id(parent_session.read(cx).session_id())
                .is_none()
        );
        assert!(
            dap_store
                .session_by_id(first_child_session.read(cx).session_id())
                .is_none()
        );
        assert!(
            dap_store
                .session_by_id(second_child_session.read(cx).session_id())
                .is_none()
        );
    });
}

#[gpui::test]
async fn test_shutdown_parent_session_if_all_children_are_shutdown(
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
    let dap_store = project.update(cx, |project, _| project.dap_store());
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let parent_session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = parent_session.update(cx, |session, _| session.adapter_client().unwrap());

    client.on_response::<StartDebugging, _>(move |_| {}).await;
    // Set up handlers for sessions spawned with reverse request too.
    let _reverse_request_subscription =
        project::debugger::test::intercept_debug_sessions(cx, |_| {});
    // start first child session
    client
        .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
            configuration: json!({}),
            request: StartDebuggingRequestArgumentsRequest::Launch,
        })
        .await;

    cx.run_until_parked();

    // start second child session
    client
        .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
            configuration: json!({}),
            request: StartDebuggingRequestArgumentsRequest::Launch,
        })
        .await;

    cx.run_until_parked();

    // configure first child session
    let first_child_session = dap_store.read_with(cx, |dap_store, _| {
        dap_store.session_by_id(SessionId(1)).unwrap()
    });
    let first_child_client =
        first_child_session.update(cx, |session, _| session.adapter_client().unwrap());

    first_child_client.on_request::<Disconnect, _>(move |_, _| Ok(()));

    // configure second child session
    let second_child_session = dap_store.read_with(cx, |dap_store, _| {
        dap_store.session_by_id(SessionId(2)).unwrap()
    });
    let second_child_client =
        second_child_session.update(cx, |session, _| session.adapter_client().unwrap());

    second_child_client.on_request::<Disconnect, _>(move |_, _| Ok(()));

    cx.run_until_parked();

    // shutdown first child session
    dap_store
        .update(cx, |dap_store, cx| {
            dap_store.shutdown_session(first_child_session.read(cx).session_id(), cx)
        })
        .await
        .unwrap();

    // assert parent session and second child session still exist
    dap_store.update(cx, |dap_store, cx| {
        assert!(
            dap_store
                .session_by_id(parent_session.read(cx).session_id())
                .is_some()
        );
        assert!(
            dap_store
                .session_by_id(first_child_session.read(cx).session_id())
                .is_none()
        );
        assert!(
            dap_store
                .session_by_id(second_child_session.read(cx).session_id())
                .is_some()
        );
    });

    // shutdown first child session
    dap_store
        .update(cx, |dap_store, cx| {
            dap_store.shutdown_session(second_child_session.read(cx).session_id(), cx)
        })
        .await
        .unwrap();

    // assert parent session got shutdown by second child session
    // because it was the last child
    dap_store.update(cx, |dap_store, cx| {
        assert!(
            dap_store
                .session_by_id(parent_session.read(cx).session_id())
                .is_none()
        );
        assert!(
            dap_store
                .session_by_id(second_child_session.read(cx).session_id())
                .is_none()
        );
    });
}

#[gpui::test]
async fn test_debug_panel_item_thread_status_reset_on_failure(
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

    let session = start_debug_session(&workspace, cx, |client| {
        client.on_request::<dap::requests::Initialize, _>(move |_, _| {
            Ok(dap::Capabilities {
                supports_step_back: Some(true),
                ..Default::default()
            })
        });
    })
    .unwrap();

    let client = session.update(cx, |session, _| session.adapter_client().unwrap());
    const THREAD_ID_NUM: i64 = 1;

    client.on_request::<dap::requests::Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: THREAD_ID_NUM,
                name: "Thread 1".into(),
            }],
        })
    });

    client.on_request::<Launch, _>(move |_, _| Ok(()));

    client.on_request::<StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: Vec::default(),
            total_frames: None,
        })
    });

    client.on_request::<Next, _>(move |_, _| {
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
    });

    client.on_request::<StepOut, _>(move |_, _| {
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
    });

    client.on_request::<StepIn, _>(move |_, _| {
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
    });

    client.on_request::<StepBack, _>(move |_, _| {
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
    });

    client.on_request::<Continue, _>(move |_, _| {
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

    let running_state = active_debug_session_panel(workspace, cx)
        .read_with(cx, |item, _| item.running_state().clone());

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

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();

    let (editor, cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(buffer, cx),
            Some(project.clone()),
            window,
            cx,
        )
    });

    client.on_request::<Launch, _>(move |_, _| Ok(()));

    client.on_request::<StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: Vec::default(),
            total_frames: None,
        })
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

    let called_set_breakpoints = Arc::new(AtomicBool::new(false));
    client.on_request::<SetBreakpoints, _>({
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
    });

    editor.update_in(cx, |editor, window, cx| {
        editor.move_down(&zed_actions::editor::MoveDown, window, cx);
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
    });

    cx.run_until_parked();

    assert!(
        called_set_breakpoints.load(std::sync::atomic::Ordering::SeqCst),
        "SetBreakpoint request must be called"
    );

    let called_set_breakpoints = Arc::new(AtomicBool::new(false));
    client.on_request::<SetBreakpoints, _>({
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
    });

    editor.update_in(cx, |editor, window, cx| {
        editor.move_up(&zed_actions::editor::MoveUp, window, cx);
        editor.insert("new text\n", window, cx);
    });

    editor
        .update_in(cx, |editor, window, cx| {
            editor.save(
                SaveOptions {
                    format: true,
                    autosave: false,
                },
                project.clone(),
                window,
                cx,
            )
        })
        .await
        .unwrap();

    cx.run_until_parked();

    assert!(
        called_set_breakpoints.load(std::sync::atomic::Ordering::SeqCst),
        "SetBreakpoint request must be called after editor is saved"
    );
}

#[gpui::test]
async fn test_unsetting_breakpoints_on_clear_breakpoint_action(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
            "second.rs": "First line\nSecond line\nThird line\nFourth line",
            "no_breakpoints.rs": "Used to ensure that we don't unset breakpoint in files with no breakpoints"
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

    let first = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();

    let second = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, rel_path("second.rs")), cx)
        })
        .await
        .unwrap();

    let (first_editor, cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(first, cx),
            Some(project.clone()),
            window,
            cx,
        )
    });

    let (second_editor, cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(second, cx),
            Some(project.clone()),
            window,
            cx,
        )
    });

    first_editor.update_in(cx, |editor, window, cx| {
        editor.move_down(&zed_actions::editor::MoveDown, window, cx);
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
        editor.move_down(&zed_actions::editor::MoveDown, window, cx);
        editor.move_down(&zed_actions::editor::MoveDown, window, cx);
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
    });

    second_editor.update_in(cx, |editor, window, cx| {
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
        editor.move_down(&zed_actions::editor::MoveDown, window, cx);
        editor.move_down(&zed_actions::editor::MoveDown, window, cx);
        editor.move_down(&zed_actions::editor::MoveDown, window, cx);
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, window, cx);
    });

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    let called_set_breakpoints = Arc::new(AtomicBool::new(false));

    client.on_request::<SetBreakpoints, _>({
        move |_, args| {
            assert!(
                args.breakpoints.is_none_or(|bps| bps.is_empty()),
                "Send empty breakpoint sets to clear them from DAP servers"
            );

            match args
                .source
                .path
                .expect("We should always send a breakpoint's path")
                .as_str()
            {
                path!("/project/main.rs") | path!("/project/second.rs") => {}
                _ => {
                    panic!("Unset breakpoints for path that doesn't have any")
                }
            }

            called_set_breakpoints.store(true, Ordering::SeqCst);

            Ok(dap::SetBreakpointsResponse {
                breakpoints: Vec::default(),
            })
        }
    });

    cx.dispatch_action(crate::ClearAllBreakpoints);
    cx.run_until_parked();
}

#[gpui::test]
async fn test_debug_session_is_shutdown_when_attach_and_launch_request_fails(
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

    start_debug_session(&workspace, cx, |client| {
        client.on_request::<dap::requests::Initialize, _>(|_, _| {
            Err(ErrorResponse {
                error: Some(Message {
                    format: "failed to launch".to_string(),
                    id: 1,
                    variables: None,
                    send_telemetry: None,
                    show_user: None,
                    url: None,
                    url_label: None,
                }),
            })
        });
    })
    .ok();

    cx.run_until_parked();

    project.update(cx, |project, cx| {
        assert!(
            project.dap_store().read(cx).sessions().count() == 0,
            "Session wouldn't exist if it was shutdown"
        );
    });
}

#[gpui::test]
async fn test_we_send_arguments_from_user_config(
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
    let debug_definition = DebugTaskDefinition {
        adapter: "fake-adapter".into(),
        config: json!({
            "request": "launch",
            "program": "main.rs".to_owned(),
            "args": vec!["arg1".to_owned(), "arg2".to_owned()],
            "cwd": path!("/Random_path"),
            "env": json!({ "KEY": "VALUE" }),
        }),
        label: "test".into(),
        tcp_connection: None,
    };

    let launch_handler_called = Arc::new(AtomicBool::new(false));

    start_debug_session_with(&workspace, cx, debug_definition.clone(), {
        let launch_handler_called = launch_handler_called.clone();

        move |client| {
            let debug_definition = debug_definition.clone();
            let launch_handler_called = launch_handler_called.clone();

            client.on_request::<dap::requests::Launch, _>(move |_, args| {
                launch_handler_called.store(true, Ordering::SeqCst);

                assert_eq!(args.raw, debug_definition.config);

                Ok(())
            });
        }
    })
    .ok();

    cx.run_until_parked();

    assert!(
        launch_handler_called.load(Ordering::SeqCst),
        "Launch request handler was not called"
    );
}

#[gpui::test]
async fn test_active_debug_line_setting(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
            "second.rs": "First line\nSecond line\nThird line\nFourth line",
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

    let main_buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, rel_path("main.rs")), cx)
        })
        .await
        .unwrap();

    let second_buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, rel_path("second.rs")), cx)
        })
        .await
        .unwrap();

    let (main_editor, cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(main_buffer, cx),
            Some(project.clone()),
            window,
            cx,
        )
    });

    let (second_editor, cx) = cx.add_window_view(|window, cx| {
        Editor::new(
            EditorMode::full(),
            MultiBuffer::build_from_buffer(second_buffer, cx),
            Some(project.clone()),
            window,
            cx,
        )
    });

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

    client.on_request::<dap::requests::Scopes, _>(move |_, _| {
        Ok(dap::ScopesResponse {
            scopes: Vec::default(),
        })
    });

    client.on_request::<StackTrace, _>(move |_, args| {
        assert_eq!(args.thread_id, 1);

        Ok(dap::StackTraceResponse {
            stack_frames: vec![dap::StackFrame {
                id: 1,
                name: "frame 1".into(),
                source: Some(dap::Source {
                    name: Some("main.rs".into()),
                    path: Some(path!("/project/main.rs").into()),
                    source_reference: None,
                    presentation_hint: None,
                    origin: None,
                    sources: None,
                    adapter_data: None,
                    checksums: None,
                }),
                line: 2,
                column: 0,
                end_line: None,
                end_column: None,
                can_restart: None,
                instruction_pointer_reference: None,
                module_id: None,
                presentation_hint: None,
            }],
            total_frames: None,
        })
    });

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Breakpoint,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    main_editor.update_in(cx, |editor, window, cx| {
        let active_debug_lines: Vec<_> = editor.highlighted_rows::<ActiveDebugLine>().collect();

        assert_eq!(
            active_debug_lines.len(),
            1,
            "There should be only one active debug line"
        );

        let point = editor
            .snapshot(window, cx)
            .buffer_snapshot()
            .summary_for_anchor::<language::Point>(&active_debug_lines.first().unwrap().0.start);

        assert_eq!(point.row, 1);
    });

    second_editor.update(cx, |editor, _| {
        let active_debug_lines: Vec<_> = editor.highlighted_rows::<ActiveDebugLine>().collect();

        assert!(
            active_debug_lines.is_empty(),
            "There shouldn't be any active debug lines"
        );
    });

    let handled_second_stacktrace = Arc::new(AtomicBool::new(false));
    client.on_request::<StackTrace, _>({
        let handled_second_stacktrace = handled_second_stacktrace.clone();
        move |_, args| {
            handled_second_stacktrace.store(true, Ordering::SeqCst);
            assert_eq!(args.thread_id, 1);

            Ok(dap::StackTraceResponse {
                stack_frames: vec![dap::StackFrame {
                    id: 2,
                    name: "frame 2".into(),
                    source: Some(dap::Source {
                        name: Some("second.rs".into()),
                        path: Some(path!("/project/second.rs").into()),
                        source_reference: None,
                        presentation_hint: None,
                        origin: None,
                        sources: None,
                        adapter_data: None,
                        checksums: None,
                    }),
                    line: 3,
                    column: 0,
                    end_line: None,
                    end_column: None,
                    can_restart: None,
                    instruction_pointer_reference: None,
                    module_id: None,
                    presentation_hint: None,
                }],
                total_frames: None,
            })
        }
    });

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Breakpoint,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    second_editor.update_in(cx, |editor, window, cx| {
        let active_debug_lines: Vec<_> = editor.highlighted_rows::<ActiveDebugLine>().collect();

        assert_eq!(
            active_debug_lines.len(),
            1,
            "There should be only one active debug line"
        );

        let point = editor
            .snapshot(window, cx)
            .buffer_snapshot()
            .summary_for_anchor::<language::Point>(&active_debug_lines.first().unwrap().0.start);

        assert_eq!(point.row, 2);
    });

    main_editor.update(cx, |editor, _| {
        let active_debug_lines: Vec<_> = editor.highlighted_rows::<ActiveDebugLine>().collect();

        assert!(
            active_debug_lines.is_empty(),
            "There shouldn't be any active debug lines"
        );
    });

    assert!(
        handled_second_stacktrace.load(Ordering::SeqCst),
        "Second stacktrace request handler was not called"
    );

    client
        .fake_event(dap::messages::Events::Continued(dap::ContinuedEvent {
            thread_id: 0,
            all_threads_continued: Some(true),
        }))
        .await;

    cx.run_until_parked();

    second_editor.update(cx, |editor, _| {
        let active_debug_lines: Vec<_> = editor.highlighted_rows::<ActiveDebugLine>().collect();

        assert!(
            active_debug_lines.is_empty(),
            "There shouldn't be any active debug lines"
        );
    });

    main_editor.update(cx, |editor, _| {
        let active_debug_lines: Vec<_> = editor.highlighted_rows::<ActiveDebugLine>().collect();

        assert!(
            active_debug_lines.is_empty(),
            "There shouldn't be any active debug lines"
        );
    });

    // Clean up
    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();

    main_editor.update(cx, |editor, _| {
        let active_debug_lines: Vec<_> = editor.highlighted_rows::<ActiveDebugLine>().collect();

        assert!(
            active_debug_lines.is_empty(),
            "There shouldn't be any active debug lines after session shutdown"
        );
    });

    second_editor.update(cx, |editor, _| {
        let active_debug_lines: Vec<_> = editor.highlighted_rows::<ActiveDebugLine>().collect();

        assert!(
            active_debug_lines.is_empty(),
            "There shouldn't be any active debug lines after session shutdown"
        );
    });
}

#[gpui::test]
async fn test_debug_adapters_shutdown_on_app_quit(
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

    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    let disconnect_request_received = Arc::new(AtomicBool::new(false));
    let disconnect_clone = disconnect_request_received.clone();

    client.on_request::<Disconnect, _>(move |_, _| {
        disconnect_clone.store(true, Ordering::SeqCst);
        Ok(())
    });

    executor.run_until_parked();

    workspace
        .update(cx, |workspace, _, cx| {
            let panel = workspace.panel::<DebugPanel>(cx).unwrap();
            panel.read_with(cx, |panel, _| {
                assert!(
                    panel.sessions().next().is_some(),
                    "Debug session should be active"
                );
            });
        })
        .unwrap();

    cx.update(|_, cx| cx.defer(|cx| cx.shutdown()));

    executor.run_until_parked();

    assert!(
        disconnect_request_received.load(Ordering::SeqCst),
        "Disconnect request should have been sent to the adapter on app shutdown"
    );
}

#[gpui::test]
async fn test_breakpoint_jumps_only_in_proper_split_view(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
            "second.rs": "First line\nSecond line\nThird line\nFourth line",
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

    // Open main.rs in pane A (the initial pane)
    let pane_a = workspace
        .update(cx, |multi, _window, cx| {
            multi.workspace().read(cx).active_pane().clone()
        })
        .unwrap();

    let open_main = workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.open_path((worktree_id, rel_path("main.rs")), None, true, window, cx)
            })
        })
        .unwrap();
    open_main.await.unwrap();

    cx.run_until_parked();

    // Split pane A to the right, creating pane B
    let pane_b = workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.split_pane(pane_a.clone(), SplitDirection::Right, window, cx)
            })
        })
        .unwrap();

    cx.run_until_parked();

    // Open main.rs in pane B
    let weak_pane_b = pane_b.downgrade();
    let open_main_in_b = workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("main.rs")),
                    Some(weak_pane_b),
                    true,
                    window,
                    cx,
                )
            })
        })
        .unwrap();
    open_main_in_b.await.unwrap();

    cx.run_until_parked();

    // Also open second.rs in pane B as an inactive tab
    let weak_pane_b = pane_b.downgrade();
    let open_second_in_b = workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("second.rs")),
                    Some(weak_pane_b),
                    true,
                    window,
                    cx,
                )
            })
        })
        .unwrap();
    open_second_in_b.await.unwrap();

    cx.run_until_parked();

    // Switch pane B back to main.rs so second.rs is inactive there
    let weak_pane_b = pane_b.downgrade();
    let reactivate_main_in_b = workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("main.rs")),
                    Some(weak_pane_b),
                    true,
                    window,
                    cx,
                )
            })
        })
        .unwrap();
    reactivate_main_in_b.await.unwrap();

    cx.run_until_parked();

    // Now open second.rs in pane A, making main.rs an inactive tab there
    let weak_pane_a = pane_a.downgrade();
    let open_second = workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.open_path(
                    (worktree_id, rel_path("second.rs")),
                    Some(weak_pane_a),
                    true,
                    window,
                    cx,
                )
            })
        })
        .unwrap();
    open_second.await.unwrap();

    cx.run_until_parked();

    // Layout:
    //   Pane A: second.rs (active), main.rs (inactive tab)
    //   Pane B: main.rs (active), second.rs (inactive tab)

    // Verify pane A's active item is second.rs (main.rs is an inactive tab)
    workspace
        .read_with(cx, |_multi, cx| {
            let active = pane_a.read(cx).active_item().unwrap();
            let editor = active.to_any_view().downcast::<Editor>().unwrap();
            let path = editor.read(cx).project_path(cx).unwrap();
            assert_eq!(
                path.path.file_name().unwrap(),
                "second.rs",
                "Pane A should have second.rs active",
            );
        })
        .unwrap();

    // Verify pane B's active item is main.rs
    workspace
        .read_with(cx, |_multi, cx| {
            let active = pane_b.read(cx).active_item().unwrap();
            let editor = active.to_any_view().downcast::<Editor>().unwrap();
            let path = editor.read(cx).project_path(cx).unwrap();
            assert_eq!(
                path.path.file_name().unwrap(),
                "main.rs",
                "Pane B should have main.rs active",
            );
        })
        .unwrap();

    // Start a debug session and trigger a breakpoint stop on main.rs line 2
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

    client.on_request::<dap::requests::Scopes, _>(move |_, _| {
        Ok(dap::ScopesResponse {
            scopes: Vec::default(),
        })
    });

    client.on_request::<StackTrace, _>(move |_, args| {
        assert_eq!(args.thread_id, 1);

        Ok(dap::StackTraceResponse {
            stack_frames: vec![dap::StackFrame {
                id: 1,
                name: "frame 1".into(),
                source: Some(dap::Source {
                    name: Some("main.rs".into()),
                    path: Some(path!("/project/main.rs").into()),
                    source_reference: None,
                    presentation_hint: None,
                    origin: None,
                    sources: None,
                    adapter_data: None,
                    checksums: None,
                }),
                line: 2,
                column: 0,
                end_line: None,
                end_column: None,
                can_restart: None,
                instruction_pointer_reference: None,
                module_id: None,
                presentation_hint: None,
            }],
            total_frames: None,
        })
    });

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Breakpoint,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    // After first breakpoint stop on main.rs:
    // Pane A should still have second.rs as its active item because
    // main.rs was only an inactive tab there. The debugger should have jumped
    // to main.rs only in pane B where it was already the active tab.
    workspace
        .read_with(cx, |_multi, cx| {
            let pane_a_active = pane_a.read(cx).active_item().unwrap();
            let pane_a_editor = pane_a_active.to_any_view().downcast::<Editor>().unwrap();
            let pane_a_path = pane_a_editor.read(cx).project_path(cx).unwrap();
            assert_eq!(
                pane_a_path.path.file_name().unwrap(),
                "second.rs",
                "Pane A should still have second.rs as active item. \
                 The debugger should not switch active tabs in panes where the \
                 breakpoint file is not the active tab (issue #40602)",
            );
        })
        .unwrap();

    // There should be exactly one active debug line across all editors in all panes
    workspace
        .read_with(cx, |_multi, cx| {
            let mut total_active_debug_lines = 0;
            for pane in [&pane_a, &pane_b] {
                for item in pane.read(cx).items() {
                    if let Some(editor) = item.to_any_view().downcast::<Editor>().ok() {
                        total_active_debug_lines += editor
                            .read(cx)
                            .highlighted_rows::<ActiveDebugLine>()
                            .count();
                    }
                }
            }
            assert_eq!(
                total_active_debug_lines, 1,
                "There should be exactly one active debug line across all editors in all panes"
            );
        })
        .unwrap();

    // Pane B should show the debug highlight on main.rs
    workspace
        .read_with(cx, |_multi, cx| {
            let pane_b_active = pane_b.read(cx).active_item().unwrap();
            let pane_b_editor = pane_b_active.to_any_view().downcast::<Editor>().unwrap();

            let active_debug_lines: Vec<_> = pane_b_editor
                .read(cx)
                .highlighted_rows::<ActiveDebugLine>()
                .collect();

            assert_eq!(
                active_debug_lines.len(),
                1,
                "Pane B's main.rs editor should have the active debug line"
            );
        })
        .unwrap();

    // Second breakpoint stop: now on second.rs line 3.
    // Even though pane A has second.rs as its active tab, the debug line
    // should open in pane B (the persistent debug pane) because pane B
    // had the last active debug line.
    client.on_request::<StackTrace, _>(move |_, args| {
        assert_eq!(args.thread_id, 1);

        Ok(dap::StackTraceResponse {
            stack_frames: vec![dap::StackFrame {
                id: 2,
                name: "frame 2".into(),
                source: Some(dap::Source {
                    name: Some("second.rs".into()),
                    path: Some(path!("/project/second.rs").into()),
                    source_reference: None,
                    presentation_hint: None,
                    origin: None,
                    sources: None,
                    adapter_data: None,
                    checksums: None,
                }),
                line: 3,
                column: 0,
                end_line: None,
                end_column: None,
                can_restart: None,
                instruction_pointer_reference: None,
                module_id: None,
                presentation_hint: None,
            }],
            total_frames: None,
        })
    });

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Breakpoint,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    // Pane B should now have second.rs as the active tab with the debug line,
    // because pane B was the last pane that had the debug line (persistent debug pane).
    workspace
        .read_with(cx, |_multi, cx| {
            let pane_b_active = pane_b.read(cx).active_item().unwrap();
            let pane_b_editor = pane_b_active.to_any_view().downcast::<Editor>().unwrap();
            let pane_b_path = pane_b_editor.read(cx).project_path(cx).unwrap();
            assert_eq!(
                pane_b_path.path.file_name().unwrap(),
                "second.rs",
                "Pane B should have switched to second.rs because it is the persistent debug pane",
            );

            let active_debug_lines: Vec<_> = pane_b_editor
                .read(cx)
                .highlighted_rows::<ActiveDebugLine>()
                .collect();

            assert_eq!(
                active_debug_lines.len(),
                1,
                "Pane B's second.rs editor should have the active debug line"
            );
        })
        .unwrap();

    // There should still be exactly one active debug line across all editors
    workspace
        .read_with(cx, |_multi, cx| {
            let mut total_active_debug_lines = 0;
            for pane in [&pane_a, &pane_b] {
                for item in pane.read(cx).items() {
                    if let Some(editor) = item.to_any_view().downcast::<Editor>().ok() {
                        total_active_debug_lines += editor
                            .read(cx)
                            .highlighted_rows::<ActiveDebugLine>()
                            .count();
                    }
                }
            }
            assert_eq!(
                total_active_debug_lines, 1,
                "There should be exactly one active debug line across all editors after second stop"
            );
        })
        .unwrap();

    // === New case: Move the debug pane (pane B) active item to a new pane C ===
    // This simulates a user dragging the tab with the active debug line to a new split.
    // The debugger should track that the debug line moved to pane C and use pane C
    // for subsequent debug stops.

    // Split pane B to create pane C
    let pane_c = workspace
        .update(cx, |multi, window, cx| {
            multi.workspace().update(cx, |workspace, cx| {
                workspace.split_pane(pane_b.clone(), SplitDirection::Right, window, cx)
            })
        })
        .unwrap();

    cx.run_until_parked();

    // Move the active item (second.rs with debug line) from pane B to pane C
    workspace
        .update(cx, |_multi, window, cx| {
            move_active_item(&pane_b, &pane_c, true, false, window, cx);
        })
        .unwrap();

    cx.run_until_parked();

    // Verify pane C now has second.rs as active item
    workspace
        .read_with(cx, |_multi, cx| {
            let pane_c_active = pane_c.read(cx).active_item().unwrap();
            let pane_c_editor = pane_c_active.to_any_view().downcast::<Editor>().unwrap();
            let pane_c_path = pane_c_editor.read(cx).project_path(cx).unwrap();
            assert_eq!(
                pane_c_path.path.file_name().unwrap(),
                "second.rs",
                "Pane C should have second.rs after moving it from pane B",
            );
        })
        .unwrap();

    // Third breakpoint stop: back on main.rs line 2.
    // The debug line should appear in pane C because that's where the debug line
    // was moved to. The debugger should track pane moves.
    client.on_request::<StackTrace, _>(move |_, args| {
        assert_eq!(args.thread_id, 1);

        Ok(dap::StackTraceResponse {
            stack_frames: vec![dap::StackFrame {
                id: 3,
                name: "frame 3".into(),
                source: Some(dap::Source {
                    name: Some("main.rs".into()),
                    path: Some(path!("/project/main.rs").into()),
                    source_reference: None,
                    presentation_hint: None,
                    origin: None,
                    sources: None,
                    adapter_data: None,
                    checksums: None,
                }),
                line: 2,
                column: 0,
                end_line: None,
                end_column: None,
                can_restart: None,
                instruction_pointer_reference: None,
                module_id: None,
                presentation_hint: None,
            }],
            total_frames: None,
        })
    });

    client
        .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
            reason: dap::StoppedEventReason::Breakpoint,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    // Pane C should now have main.rs as the active tab with the debug line,
    // because pane C is where the debug line was moved to from pane B.
    workspace
        .read_with(cx, |_multi, cx| {
            let pane_c_active = pane_c.read(cx).active_item().unwrap();
            let pane_c_editor = pane_c_active.to_any_view().downcast::<Editor>().unwrap();
            let pane_c_path = pane_c_editor.read(cx).project_path(cx).unwrap();
            assert_eq!(
                pane_c_path.path.file_name().unwrap(),
                "main.rs",
                "Pane C should have switched to main.rs because it is now the persistent debug pane \
                 (the debug line was moved here from pane B)",
            );

            let active_debug_lines: Vec<_> = pane_c_editor
                .read(cx)
                .highlighted_rows::<ActiveDebugLine>()
                .collect();

            assert_eq!(
                active_debug_lines.len(),
                1,
                "Pane C's main.rs editor should have the active debug line"
            );
        })
        .unwrap();

    // There should still be exactly one active debug line across all editors
    workspace
        .read_with(cx, |_multi, cx| {
            let mut total_active_debug_lines = 0;
            for pane in [&pane_a, &pane_b, &pane_c] {
                for item in pane.read(cx).items() {
                    if let Some(editor) = item.to_any_view().downcast::<Editor>().ok() {
                        total_active_debug_lines += editor
                            .read(cx)
                            .highlighted_rows::<ActiveDebugLine>()
                            .count();
                    }
                }
            }
            assert_eq!(
                total_active_debug_lines, 1,
                "There should be exactly one active debug line across all editors after third stop"
            );
        })
        .unwrap();

    // Clean up
    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}

#[gpui::test]
async fn test_adapter_shutdown_with_child_sessions_on_app_quit(
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

    let parent_session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let parent_session_id = cx.read(|cx| parent_session.read(cx).session_id());
    let parent_client = parent_session.update(cx, |session, _| session.adapter_client().unwrap());

    let disconnect_count = Arc::new(std::sync::atomic::AtomicUsize::new(0));
    let parent_disconnect_called = Arc::new(AtomicBool::new(false));
    let parent_disconnect_clone = parent_disconnect_called.clone();
    let disconnect_count_clone = disconnect_count.clone();

    parent_client.on_request::<Disconnect, _>(move |_, _| {
        parent_disconnect_clone.store(true, Ordering::SeqCst);
        disconnect_count_clone.fetch_add(1, Ordering::SeqCst);

        for _ in 0..50 {
            if disconnect_count_clone.load(Ordering::SeqCst) >= 2 {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        Ok(())
    });

    parent_client
        .on_response::<StartDebugging, _>(move |_| {})
        .await;
    let _subscription = project::debugger::test::intercept_debug_sessions(cx, |_| {});

    parent_client
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
    let child_session_id = cx.read(|cx| child_session.read(cx).session_id());
    let child_client = child_session.update(cx, |session, _| session.adapter_client().unwrap());

    let child_disconnect_called = Arc::new(AtomicBool::new(false));
    let child_disconnect_clone = child_disconnect_called.clone();
    let disconnect_count_clone = disconnect_count.clone();

    child_client.on_request::<Disconnect, _>(move |_, _| {
        child_disconnect_clone.store(true, Ordering::SeqCst);
        disconnect_count_clone.fetch_add(1, Ordering::SeqCst);

        for _ in 0..50 {
            if disconnect_count_clone.load(Ordering::SeqCst) >= 2 {
                break;
            }
            std::thread::sleep(std::time::Duration::from_millis(1));
        }

        Ok(())
    });

    executor.run_until_parked();

    project.update(cx, |project, cx| {
        let store = project.dap_store().read(cx);
        assert!(store.session_by_id(parent_session_id).is_some());
        assert!(store.session_by_id(child_session_id).is_some());
    });

    cx.update(|_, cx| cx.defer(|cx| cx.shutdown()));

    executor.run_until_parked();

    let parent_disconnect_check = parent_disconnect_called.clone();
    let child_disconnect_check = child_disconnect_called.clone();
    let executor_clone = executor.clone();
    let both_disconnected = executor
        .spawn(async move {
            let parent_disconnect = parent_disconnect_check;
            let child_disconnect = child_disconnect_check;

            // We only have 100ms to shutdown the app
            for _ in 0..100 {
                if parent_disconnect.load(Ordering::SeqCst)
                    && child_disconnect.load(Ordering::SeqCst)
                {
                    return true;
                }

                executor_clone
                    .timer(std::time::Duration::from_millis(1))
                    .await;
            }

            false
        })
        .await;

    assert!(
        both_disconnected,
        "Both parent and child sessions should receive disconnect requests"
    );

    assert!(
        parent_disconnect_called.load(Ordering::SeqCst),
        "Parent session should have received disconnect request"
    );
    assert!(
        child_disconnect_called.load(Ordering::SeqCst),
        "Child session should have received disconnect request"
    );
}
