use crate::*;
use dap::{
    client::DebugAdapterClientId,
    requests::{
        Continue, Disconnect, Initialize, Launch, Next, RunInTerminal, SetBreakpoints, StackTrace,
        StartDebugging, StepBack, StepIn, StepOut,
    },
    ErrorResponse, RunInTerminalRequestArguments, SourceBreakpoint, StartDebuggingRequestArguments,
    StartDebuggingRequestArgumentsRequest,
};
use debugger_panel::ThreadStatus;
use editor::{
    actions::{self},
    Editor, EditorMode, MultiBuffer,
};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use project::{FakeFs, Project};
use serde_json::json;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use terminal_view::{terminal_panel::TerminalPanel, TerminalView};
use tests::{active_debug_panel_item, init_test, init_test_workspace};
use workspace::{dock::Panel, Item};

#[gpui::test]
async fn test_basic_show_debug_panel(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    let project = Project::test(fs, [], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |store, cx| {
            store.start_debug_session(
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
        })
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

    client
        .on_request::<StackTrace, _>(move |_, _| {
            Ok(dap::StackTraceResponse {
                stack_frames: Vec::default(),
                total_frames: None,
            })
        })
        .await;

    client.on_request::<Disconnect, _>(move |_, _| Ok(())).await;

    // assert we don't have a debug panel item yet
    workspace
        .update(cx, |workspace, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

            debug_panel.update(cx, |this, cx| {
                assert!(this.active_debug_panel_item(cx).is_none());
                assert_eq!(0, this.pane().unwrap().read(cx).items_len());
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
        .update(cx, |workspace, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_debug_panel_item = debug_panel
                .update(cx, |this, cx| this.active_debug_panel_item(cx))
                .unwrap();

            assert_eq!(
                1,
                debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
            );
            assert_eq!(client.id(), active_debug_panel_item.read(cx).client_id());
            assert_eq!(1, active_debug_panel_item.read(cx).thread_id());
        })
        .unwrap();

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
        })
    });

    shutdown_session.await.unwrap();

    // assert we don't have a debug panel item anymore because the client shutdown
    workspace
        .update(cx, |workspace, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

            debug_panel.update(cx, |this, cx| {
                assert!(this.active_debug_panel_item(cx).is_none());
                assert_eq!(0, this.pane().unwrap().read(cx).items_len());
            });
        })
        .unwrap();
}

#[gpui::test]
async fn test_we_can_only_have_one_panel_per_debug_thread(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    let project = Project::test(fs, [], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |store, cx| {
            store.start_debug_session(
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
        })
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

    client
        .on_request::<StackTrace, _>(move |_, _| {
            Ok(dap::StackTraceResponse {
                stack_frames: Vec::default(),
                total_frames: None,
            })
        })
        .await;

    client.on_request::<Disconnect, _>(move |_, _| Ok(())).await;

    // assert we don't have a debug panel item yet
    workspace
        .update(cx, |workspace, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

            debug_panel.update(cx, |this, cx| {
                assert!(this.active_debug_panel_item(cx).is_none());
                assert_eq!(0, this.pane().unwrap().read(cx).items_len());
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
        .update(cx, |workspace, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_debug_panel_item = debug_panel
                .update(cx, |this, cx| this.active_debug_panel_item(cx))
                .unwrap();

            assert_eq!(
                1,
                debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
            );
            assert_eq!(client.id(), active_debug_panel_item.read(cx).client_id());
            assert_eq!(1, active_debug_panel_item.read(cx).thread_id());
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
        .update(cx, |workspace, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_debug_panel_item = debug_panel
                .update(cx, |this, cx| this.active_debug_panel_item(cx))
                .unwrap();

            assert_eq!(
                1,
                debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
            );
            assert_eq!(client.id(), active_debug_panel_item.read(cx).client_id());
            assert_eq!(1, active_debug_panel_item.read(cx).thread_id());
        })
        .unwrap();

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
        })
    });

    shutdown_session.await.unwrap();

    // assert we don't have a debug panel item anymore because the client shutdown
    workspace
        .update(cx, |workspace, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

            debug_panel.update(cx, |this, cx| {
                assert!(this.active_debug_panel_item(cx).is_none());
                assert_eq!(0, this.pane().unwrap().read(cx).items_len());
            });
        })
        .unwrap();
}

#[gpui::test]
async fn test_client_can_open_multiple_thread_panels(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    let project = Project::test(fs, [], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |store, cx| {
            store.start_debug_session(
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
        })
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

    client
        .on_request::<StackTrace, _>(move |_, _| {
            Ok(dap::StackTraceResponse {
                stack_frames: Vec::default(),
                total_frames: None,
            })
        })
        .await;

    client.on_request::<Disconnect, _>(move |_, _| Ok(())).await;

    // assert we don't have a debug panel item yet
    workspace
        .update(cx, |workspace, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

            debug_panel.update(cx, |this, cx| {
                assert!(this.active_debug_panel_item(cx).is_none());
                assert_eq!(0, this.pane().unwrap().read(cx).items_len());
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
        .update(cx, |workspace, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_debug_panel_item = debug_panel
                .update(cx, |this, cx| this.active_debug_panel_item(cx))
                .unwrap();

            assert_eq!(
                1,
                debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
            );
            assert_eq!(client.id(), active_debug_panel_item.read(cx).client_id());
            assert_eq!(1, active_debug_panel_item.read(cx).thread_id());
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

    // assert we added a debug panel item and the new one is active
    workspace
        .update(cx, |workspace, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_debug_panel_item = debug_panel
                .update(cx, |this, cx| this.active_debug_panel_item(cx))
                .unwrap();

            assert_eq!(
                2,
                debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
            );
            assert_eq!(client.id(), active_debug_panel_item.read(cx).client_id());
            assert_eq!(2, active_debug_panel_item.read(cx).thread_id());
        })
        .unwrap();

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
        })
    });

    shutdown_session.await.unwrap();

    // assert we don't have a debug panel item anymore because the client shutdown
    workspace
        .update(cx, |workspace, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

            debug_panel.update(cx, |this, cx| {
                assert!(this.active_debug_panel_item(cx).is_none());
                assert_eq!(0, this.pane().unwrap().read(cx).items_len());
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

    let project = Project::test(fs, [], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |store, cx| {
            store.start_debug_session(
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
        })
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

    client.on_request::<Disconnect, _>(move |_, _| Ok(())).await;

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
        .update(cx, |workspace, cx| {
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
            dap_store.shutdown_session(&session.read(cx).id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}

// covers that we always send a response back, if something when wrong,
// while spawning the terminal
#[gpui::test]
async fn test_handle_error_run_in_terminal_reverse_request(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let send_response = Arc::new(AtomicBool::new(false));

    let fs = FakeFs::new(executor.clone());

    let project = Project::test(fs, [], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |store, cx| {
            store.start_debug_session(
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
        })
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

    client.on_request::<Disconnect, _>(move |_, _| Ok(())).await;

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
        .update(cx, |workspace, cx| {
            let terminal_panel = workspace.panel::<TerminalPanel>(cx).unwrap();

            assert_eq!(
                0,
                terminal_panel.read(cx).pane().unwrap().read(cx).items_len()
            );
        })
        .unwrap();

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
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
    let send_launch = Arc::new(AtomicBool::new(false));

    let fs = FakeFs::new(executor.clone());

    let project = Project::test(fs, [], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |store, cx| {
            store.start_debug_session(
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
        })
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

    client.on_request::<Disconnect, _>(move |_, _| Ok(())).await;

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

    cx.run_until_parked();

    client
        .fake_reverse_request::<StartDebugging>(StartDebuggingRequestArguments {
            configuration: json!({}),
            request: StartDebuggingRequestArgumentsRequest::Launch,
        })
        .await;

    cx.run_until_parked();

    project.update(cx, |_, cx| {
        assert_eq!(2, session.read(cx).clients_len());
    });
    assert!(
        send_response.load(std::sync::atomic::Ordering::SeqCst),
        "Expected to receive response from reverse request"
    );

    let second_client = project.update(cx, |_, cx| {
        session
            .read(cx)
            .client_by_id(&DebugAdapterClientId(1))
            .unwrap()
    });

    project.update(cx, |_, cx| {
        cx.emit(project::Event::DebugClientStarted((
            session.read(cx).id(),
            second_client.id(),
        )));
    });

    second_client
        .on_request::<Initialize, _>(move |_, _| {
            Ok(dap::Capabilities {
                supports_step_back: Some(false),
                ..Default::default()
            })
        })
        .await;
    second_client
        .on_request::<Launch, _>({
            let send_launch = send_launch.clone();
            move |_, _| {
                send_launch.store(true, Ordering::SeqCst);

                Ok(())
            }
        })
        .await;
    second_client
        .on_request::<Disconnect, _>(move |_, _| Ok(()))
        .await;

    cx.run_until_parked();

    assert!(
        send_launch.load(std::sync::atomic::Ordering::SeqCst),
        "Expected to send launch request on second client"
    );

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
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

    let project = Project::test(fs, [], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    let task = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |store, cx| {
            store.start_debug_session(
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
        })
    });

    let (session, client) = task.await.unwrap();

    client
        .on_request::<Initialize, _>(move |_, _| {
            Ok(dap::Capabilities {
                supports_step_back: Some(true),
                ..Default::default()
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

    client.on_request::<Disconnect, _>(move |_, _| Ok(())).await;

    cx.run_until_parked();

    for operation in &[
        "step_over",
        "continue_thread",
        "step_back",
        "step_in",
        "step_out",
    ] {
        active_debug_panel_item(workspace, cx).update(
            cx,
            |debug_panel_item, cx| match *operation {
                "step_over" => debug_panel_item.step_over(cx),
                "continue_thread" => debug_panel_item.continue_thread(cx),
                "step_back" => debug_panel_item.step_back(cx),
                "step_in" => debug_panel_item.step_in(cx),
                "step_out" => debug_panel_item.step_out(cx),
                _ => unreachable!(),
            },
        );

        cx.run_until_parked();

        active_debug_panel_item(workspace, cx).update(cx, |debug_panel_item, cx| {
            assert_eq!(
                debug_panel_item.thread_state().read(cx).status,
                debugger_panel::ThreadStatus::Stopped,
                "Thread status not reset to Stopped after failed {}",
                operation
            );

            // update state to running, so we can test it actually changes the status back to stopped
            debug_panel_item
                .thread_state()
                .update(cx, |thread_state, cx| {
                    thread_state.status = ThreadStatus::Running;
                    cx.notify();
                });
        });
    }

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
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
        "/a",
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, ["/a".as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    let worktree_id = workspace
        .update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                project.worktrees(cx).next().unwrap().read(cx).id()
            })
        })
        .unwrap();

    let task = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |store, cx| {
            store.start_debug_session(
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
        })
    });

    let (session, client) = task.await.unwrap();

    let buffer = project
        .update(cx, |project, cx| {
            project.open_buffer((worktree_id, "main.rs"), cx)
        })
        .await
        .unwrap();

    let (editor, cx) = cx.add_window_view(|cx| {
        Editor::new(
            EditorMode::Full,
            MultiBuffer::build_from_buffer(buffer, cx),
            Some(project.clone()),
            true,
            cx,
        )
    });

    client
        .on_request::<Initialize, _>(move |_, _| {
            Ok(dap::Capabilities {
                supports_step_back: Some(true),
                ..Default::default()
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

    let called_set_breakpoints = Arc::new(AtomicBool::new(false));
    client
        .on_request::<SetBreakpoints, _>({
            let called_set_breakpoints = called_set_breakpoints.clone();
            move |_, args| {
                assert_eq!("/a/main.rs", args.source.path.unwrap());
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

    editor.update(cx, |editor, cx| {
        editor.move_down(&actions::MoveDown, cx);
        editor.toggle_breakpoint(&actions::ToggleBreakpoint, cx);
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
                assert_eq!("/a/main.rs", args.source.path.unwrap());
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

    editor.update(cx, |editor, cx| {
        editor.move_up(&actions::MoveUp, cx);
        editor.insert("new text\n", cx);
    });

    editor
        .update(cx, |editor, cx| editor.save(true, project.clone(), cx))
        .await
        .unwrap();

    cx.run_until_parked();

    assert!(
        called_set_breakpoints.load(std::sync::atomic::Ordering::SeqCst),
        "SetBreakpoint request must be called after editor is saved"
    );

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}
