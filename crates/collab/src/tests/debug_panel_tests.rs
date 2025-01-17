use call::ActiveCall;
use dap::{
    requests::{Disconnect, Initialize, Launch, RestartFrame, SetBreakpoints, StackTrace},
    SourceBreakpoint, StackFrame,
};
use debugger_ui::debugger_panel::DebugPanel;
use editor::Editor;
use gpui::{TestAppContext, View, VisualTestContext};
use project::ProjectPath;
use serde_json::json;
use std::{
    path::Path,
    sync::{
        atomic::{AtomicBool, Ordering},
        Arc,
    },
};
use workspace::{dock::Panel, Workspace};

use super::TestServer;

pub fn init_test(cx: &mut gpui::TestAppContext) {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::try_init().ok();
    }

    cx.update(|cx| {
        theme::init(theme::LoadThemes::JustBase, cx);
        command_palette_hooks::init(cx);
        language::init(cx);
        workspace::init_settings(cx);
        project::Project::init_settings(cx);
        debugger_ui::init(cx);
        editor::init(cx);
    });
}

pub async fn add_debugger_panel(workspace: &View<Workspace>, cx: &mut VisualTestContext) {
    let debugger_panel = workspace
        .update(cx, |_, cx| cx.spawn(DebugPanel::load))
        .await
        .unwrap();

    workspace.update(cx, |workspace, cx| {
        workspace.add_panel(debugger_panel, cx);
    });
}

#[gpui::test]
async fn test_debug_panel_item_opens_on_remote(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let executor = cx_a.executor();
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    init_test(cx_a);
    init_test(cx_b);

    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    let (project_a, _worktree_id) = client_a.build_local_project("/a", cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    add_debugger_panel(&workspace_a, cx_a).await;
    add_debugger_panel(&workspace_b, cx_b).await;

    let task = project_a.update(cx_a, |project, cx| {
        project.dap_store().update(cx, |store, cx| {
            store.start_debug_session(
                dap::DebugAdapterConfig {
                    label: "test config".into(),
                    kind: dap::DebugAdapterKind::Fake,
                    request: dap::DebugRequestType::Launch,
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

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    workspace_b.update(cx_b, |workspace, cx| {
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
    });

    let shutdown_client = project_a.update(cx_a, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
        })
    });

    shutdown_client.await.unwrap();
}

#[gpui::test]
async fn test_active_debug_panel_item_set_on_join_project(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let executor = cx_a.executor();
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    init_test(cx_a);
    init_test(cx_b);

    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    let (project_a, _worktree_id) = client_a.build_local_project("/a", cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);

    add_debugger_panel(&workspace_a, cx_a).await;

    let task = project_a.update(cx_a, |project, cx| {
        project.dap_store().update(cx, |store, cx| {
            store.start_debug_session(
                dap::DebugAdapterConfig {
                    label: "test config".into(),
                    kind: dap::DebugAdapterKind::Fake,
                    request: dap::DebugRequestType::Launch,
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

    // Give client_a time to send a debug panel item to collab server
    cx_a.run_until_parked();

    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);
    add_debugger_panel(&workspace_b, cx_b).await;

    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    workspace_b.update(cx_b, |workspace, cx| {
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
    });

    let shutdown_client = project_a.update(cx_a, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
        })
    });

    shutdown_client.await.unwrap();

    cx_b.run_until_parked();

    // assert we don't have a debug panel item anymore because the client shutdown
    workspace_b.update(cx_b, |workspace, cx| {
        let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

        debug_panel.update(cx, |this, cx| {
            assert!(this.active_debug_panel_item(cx).is_none());
            assert_eq!(0, this.pane().unwrap().read(cx).items_len());
        });
    });
}

#[gpui::test]
async fn test_debug_panel_remote_button_presses(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let executor = cx_a.executor();
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    init_test(cx_a);
    init_test(cx_b);

    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    let (project_a, _worktree_id) = client_a.build_local_project("/a", cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    add_debugger_panel(&workspace_a, cx_a).await;
    add_debugger_panel(&workspace_b, cx_b).await;

    let task = project_a.update(cx_a, |project, cx| {
        project.dap_store().update(cx, |store, cx| {
            store.start_debug_session(
                dap::DebugAdapterConfig {
                    label: "test config".into(),
                    kind: dap::DebugAdapterKind::Fake,
                    request: dap::DebugRequestType::Launch,
                    program: None,
                    cwd: None,
                    initialize_args: None,
                },
                cx,
            )
        })
    });

    let (_, client) = task.await.unwrap();

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

    client
        .on_request::<dap::requests::Continue, _>(move |_, _| {
            Ok(dap::ContinueResponse {
                all_threads_continued: Some(true),
            })
        })
        .await;

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    let remote_debug_item = workspace_b.update(cx_b, |workspace, cx| {
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
        active_debug_panel_item
    });

    let local_debug_item = workspace_a.update(cx_a, |workspace, cx| {
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
        active_debug_panel_item
    });

    remote_debug_item.update(cx_b, |this, cx| {
        this.continue_thread(cx);
    });

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    local_debug_item.update(cx_a, |debug_panel_item, cx| {
        assert_eq!(
            debugger_ui::debugger_panel::ThreadStatus::Running,
            debug_panel_item.thread_state().read(cx).status,
        );
    });

    remote_debug_item.update(cx_b, |debug_panel_item, cx| {
        assert_eq!(
            debugger_ui::debugger_panel::ThreadStatus::Running,
            debug_panel_item.thread_state().read(cx).status,
        );
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

    client
        .on_request::<StackTrace, _>(move |_, _| {
            Ok(dap::StackTraceResponse {
                stack_frames: Vec::default(),
                total_frames: None,
            })
        })
        .await;

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    local_debug_item.update(cx_a, |debug_panel_item, cx| {
        assert_eq!(
            debugger_ui::debugger_panel::ThreadStatus::Stopped,
            debug_panel_item.thread_state().read(cx).status,
        );
    });

    remote_debug_item.update(cx_b, |debug_panel_item, cx| {
        assert_eq!(
            debugger_ui::debugger_panel::ThreadStatus::Stopped,
            debug_panel_item.thread_state().read(cx).status,
        );
    });

    client
        .on_request::<dap::requests::Continue, _>(move |_, _| {
            Ok(dap::ContinueResponse {
                all_threads_continued: Some(true),
            })
        })
        .await;

    local_debug_item.update(cx_a, |this, cx| {
        this.continue_thread(cx);
    });

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    local_debug_item.update(cx_a, |debug_panel_item, cx| {
        assert_eq!(
            debugger_ui::debugger_panel::ThreadStatus::Running,
            debug_panel_item.thread_state().read(cx).status,
        );
    });

    remote_debug_item.update(cx_b, |debug_panel_item, cx| {
        assert_eq!(
            debugger_ui::debugger_panel::ThreadStatus::Running,
            debug_panel_item.thread_state().read(cx).status,
        );
    });

    client
        .on_request::<dap::requests::Pause, _>(move |_, _| Ok(()))
        .await;

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

    remote_debug_item.update(cx_b, |this, cx| {
        this.pause_thread(cx);
    });

    cx_b.run_until_parked();
    cx_a.run_until_parked();

    client
        .on_request::<dap::requests::StepOut, _>(move |_, _| Ok(()))
        .await;

    remote_debug_item.update(cx_b, |this, cx| {
        this.step_out(cx);
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

    cx_b.run_until_parked();
    cx_a.run_until_parked();

    client
        .on_request::<dap::requests::Next, _>(move |_, _| Ok(()))
        .await;

    remote_debug_item.update(cx_b, |this, cx| {
        this.step_over(cx);
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

    cx_b.run_until_parked();
    cx_a.run_until_parked();

    client
        .on_request::<dap::requests::StepIn, _>(move |_, _| Ok(()))
        .await;

    remote_debug_item.update(cx_b, |this, cx| {
        this.step_in(cx);
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

    cx_b.run_until_parked();
    cx_a.run_until_parked();

    client
        .on_request::<dap::requests::StepBack, _>(move |_, _| Ok(()))
        .await;

    remote_debug_item.update(cx_b, |this, cx| {
        this.step_back(cx);
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

    cx_b.run_until_parked();
    cx_a.run_until_parked();

    remote_debug_item.update(cx_b, |this, cx| {
        this.stop_thread(cx);
    });

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    // assert we don't have a debug panel item anymore because the client shutdown
    workspace_b.update(cx_b, |workspace, cx| {
        let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

        debug_panel.update(cx, |this, cx| {
            assert!(this.active_debug_panel_item(cx).is_none());
            assert_eq!(0, this.pane().unwrap().read(cx).items_len());
        });
    });
}

#[gpui::test]
async fn test_restart_stack_frame(cx_a: &mut TestAppContext, cx_b: &mut TestAppContext) {
    let executor = cx_a.executor();
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    init_test(cx_a);
    init_test(cx_b);

    let called_restart_frame = Arc::new(AtomicBool::new(false));

    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    let (project_a, _worktree_id) = client_a.build_local_project("/a", cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    add_debugger_panel(&workspace_a, cx_a).await;
    add_debugger_panel(&workspace_b, cx_b).await;

    let task = project_a.update(cx_a, |project, cx| {
        project.dap_store().update(cx, |store, cx| {
            store.start_debug_session(
                dap::DebugAdapterConfig {
                    label: "test config".into(),
                    kind: dap::DebugAdapterKind::Fake,
                    request: dap::DebugRequestType::Launch,
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
                supports_restart_frame: Some(true),
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

    client
        .on_request::<RestartFrame, _>({
            let called_restart_frame = called_restart_frame.clone();
            move |_, args| {
                assert_eq!(1, args.frame_id);

                called_restart_frame.store(true, Ordering::SeqCst);

                Ok(())
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

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    // try to restart stack frame 1 from the guest side
    workspace_b.update(cx_b, |workspace, cx| {
        let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
        let active_debug_panel_item = debug_panel
            .update(cx, |this, cx| this.active_debug_panel_item(cx))
            .unwrap();

        active_debug_panel_item.update(cx, |debug_panel_item, cx| {
            debug_panel_item
                .stack_frame_list()
                .update(cx, |stack_frame_list, cx| {
                    stack_frame_list.restart_stack_frame(1, cx);
                });
        });
    });

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    assert!(
        called_restart_frame.load(std::sync::atomic::Ordering::SeqCst),
        "Restart stack frame was not called"
    );

    let shutdown_client = project_a.update(cx_a, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
        })
    });

    shutdown_client.await.unwrap();
}

#[gpui::test]
async fn test_updated_breakpoints_send_to_dap(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
) {
    let executor = cx_a.executor();
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;

    client_a
        .fs()
        .insert_tree(
            "/a",
            json!({
                "test.txt": "one\ntwo\nthree\nfour\nfive",
            }),
        )
        .await;

    init_test(cx_a);
    init_test(cx_b);

    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);

    let (project_a, worktree_id) = client_a.build_local_project("/a", cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    let project_path = ProjectPath {
        worktree_id,
        path: Arc::from(Path::new(&"test.txt")),
    };

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    add_debugger_panel(&workspace_a, cx_a).await;
    add_debugger_panel(&workspace_b, cx_b).await;

    let task = project_a.update(cx_a, |project, cx| {
        project.dap_store().update(cx, |store, cx| {
            store.start_debug_session(
                dap::DebugAdapterConfig {
                    label: "test config".into(),
                    kind: dap::DebugAdapterKind::Fake,
                    request: dap::DebugRequestType::Launch,
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
                supports_restart_frame: Some(true),
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

    let called_set_breakpoints = Arc::new(AtomicBool::new(false));
    client
        .on_request::<SetBreakpoints, _>({
            let called_set_breakpoints = called_set_breakpoints.clone();
            move |_, args| {
                assert_eq!("/a/test.txt", args.source.path.unwrap());
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

                called_set_breakpoints.store(true, Ordering::SeqCst);

                Ok(dap::SetBreakpointsResponse {
                    breakpoints: Vec::default(),
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

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    // Client B opens an editor.
    let editor_b = workspace_b
        .update(cx_b, |workspace, cx| {
            workspace.open_path(project_path.clone(), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    editor_b.update(cx_b, |editor, cx| {
        editor.move_down(&editor::actions::MoveDown, cx);
        editor.move_down(&editor::actions::MoveDown, cx);
        editor.toggle_breakpoint(&editor::actions::ToggleBreakpoint, cx);
    });

    // Client A opens an editor.
    let editor_a = workspace_a
        .update(cx_a, |workspace, cx| {
            workspace.open_path(project_path.clone(), None, true, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    let called_set_breakpoints = Arc::new(AtomicBool::new(false));
    client
        .on_request::<SetBreakpoints, _>({
            let called_set_breakpoints = called_set_breakpoints.clone();
            move |_, args| {
                assert_eq!("/a/test.txt", args.source.path.unwrap());
                assert!(args.breakpoints.unwrap().is_empty());

                called_set_breakpoints.store(true, Ordering::SeqCst);

                Ok(dap::SetBreakpointsResponse {
                    breakpoints: Vec::default(),
                })
            }
        })
        .await;

    // remove the breakpoint that client B added
    editor_a.update(cx_a, |editor, cx| {
        editor.move_down(&editor::actions::MoveDown, cx);
        editor.move_down(&editor::actions::MoveDown, cx);
        editor.toggle_breakpoint(&editor::actions::ToggleBreakpoint, cx);
    });

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    assert!(
        called_set_breakpoints.load(std::sync::atomic::Ordering::SeqCst),
        "SetBreakpoint request must be called"
    );

    let called_set_breakpoints = Arc::new(AtomicBool::new(false));
    client
        .on_request::<SetBreakpoints, _>({
            let called_set_breakpoints = called_set_breakpoints.clone();
            move |_, args| {
                assert_eq!("/a/test.txt", args.source.path.unwrap());
                assert_eq!(
                    vec![
                        SourceBreakpoint {
                            line: 3,
                            column: None,
                            condition: None,
                            hit_condition: None,
                            log_message: None,
                            mode: None
                        },
                        SourceBreakpoint {
                            line: 2,
                            column: None,
                            condition: None,
                            hit_condition: None,
                            log_message: None,
                            mode: None
                        }
                    ],
                    args.breakpoints.unwrap()
                );

                called_set_breakpoints.store(true, Ordering::SeqCst);

                Ok(dap::SetBreakpointsResponse {
                    breakpoints: Vec::default(),
                })
            }
        })
        .await;

    // Add our own breakpoint now
    editor_a.update(cx_a, |editor, cx| {
        editor.toggle_breakpoint(&editor::actions::ToggleBreakpoint, cx);
        editor.move_up(&editor::actions::MoveUp, cx);
        editor.toggle_breakpoint(&editor::actions::ToggleBreakpoint, cx);
    });

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    assert!(
        called_set_breakpoints.load(std::sync::atomic::Ordering::SeqCst),
        "SetBreakpoint request must be called"
    );

    let shutdown_client = project_a.update(cx_a, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
        })
    });

    shutdown_client.await.unwrap();
}

#[gpui::test]
async fn test_module_list(
    cx_a: &mut TestAppContext,
    cx_b: &mut TestAppContext,
    cx_c: &mut TestAppContext,
) {
    let executor = cx_a.executor();
    let mut server = TestServer::start(executor.clone()).await;
    let client_a = server.create_client(cx_a, "user_a").await;
    let client_b = server.create_client(cx_b, "user_b").await;
    let client_c = server.create_client(cx_c, "user_c").await;

    init_test(cx_a);
    init_test(cx_b);
    init_test(cx_c);

    server
        .create_room(&mut [(&client_a, cx_a), (&client_b, cx_b), (&client_c, cx_c)])
        .await;
    let active_call_a = cx_a.read(ActiveCall::global);
    let active_call_b = cx_b.read(ActiveCall::global);
    let active_call_c = cx_c.read(ActiveCall::global);

    let (project_a, _worktree_id) = client_a.build_local_project("/a", cx_a).await;
    active_call_a
        .update(cx_a, |call, cx| call.set_location(Some(&project_a), cx))
        .await
        .unwrap();

    let project_id = active_call_a
        .update(cx_a, |call, cx| call.share_project(project_a.clone(), cx))
        .await
        .unwrap();
    let project_b = client_b.join_remote_project(project_id, cx_b).await;
    active_call_b
        .update(cx_b, |call, cx| call.set_location(Some(&project_b), cx))
        .await
        .unwrap();

    let (workspace_a, cx_a) = client_a.build_workspace(&project_a, cx_a);
    let (workspace_b, cx_b) = client_b.build_workspace(&project_b, cx_b);

    add_debugger_panel(&workspace_a, cx_a).await;
    add_debugger_panel(&workspace_b, cx_b).await;

    let task = project_a.update(cx_a, |project, cx| {
        project.dap_store().update(cx, |store, cx| {
            store.start_debug_session(
                dap::DebugAdapterConfig {
                    label: "test config".into(),
                    kind: dap::DebugAdapterKind::Fake,
                    request: dap::DebugRequestType::Launch,
                    program: None,
                    cwd: None,
                    initialize_args: None,
                },
                cx,
            )
        })
    });

    let (session, client) = task.await.unwrap();

    let called_initialize = Arc::new(AtomicBool::new(false));

    client
        .on_request::<Initialize, _>({
            let called_initialize = called_initialize.clone();
            move |_, _| {
                called_initialize.store(true, Ordering::SeqCst);
                Ok(dap::Capabilities {
                    supports_restart_frame: Some(true),
                    supports_modules_request: Some(true),
                    ..Default::default()
                })
            }
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

    let called_modules = Arc::new(AtomicBool::new(false));
    let modules = vec![
        dap::Module {
            id: dap::ModuleId::Number(1),
            name: "First Module".into(),
            address_range: None,
            date_time_stamp: None,
            path: None,
            symbol_file_path: None,
            symbol_status: None,
            version: None,
            is_optimized: None,
            is_user_code: None,
        },
        dap::Module {
            id: dap::ModuleId::Number(2),
            name: "Second Module".into(),
            address_range: None,
            date_time_stamp: None,
            path: None,
            symbol_file_path: None,
            symbol_status: None,
            version: None,
            is_optimized: None,
            is_user_code: None,
        },
    ];

    client
        .on_request::<dap::requests::Modules, _>({
            let called_modules = called_modules.clone();
            let modules = modules.clone();
            move |_, _| unsafe {
                static mut REQUEST_COUNT: i32 = 1;
                assert_eq!(
                    1, REQUEST_COUNT,
                    "This request should only be called once from the host"
                );
                REQUEST_COUNT += 1;
                called_modules.store(true, Ordering::SeqCst);

                Ok(dap::ModulesResponse {
                    modules: modules.clone(),
                    total_modules: Some(2u64),
                })
            }
        })
        .await;

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    assert!(
        called_initialize.load(std::sync::atomic::Ordering::SeqCst),
        "Request Initialize must be called"
    );

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

    cx_a.run_until_parked();
    cx_b.run_until_parked();

    assert!(
        called_modules.load(std::sync::atomic::Ordering::SeqCst),
        "Request Modules must be called"
    );

    workspace_a.update(cx_a, |workspace, cx| {
        let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
        let debug_panel_item = debug_panel
            .update(cx, |this, cx| this.active_debug_panel_item(cx))
            .unwrap();

        debug_panel_item.update(cx, |item, cx| {
            assert_eq!(
                true,
                item.capabilities(cx).supports_modules_request.unwrap(),
                "Local supports modules request should be true"
            );

            let local_module_list = item.module_list().read(cx).modules();

            assert_eq!(
                2usize,
                local_module_list.len(),
                "Local module list should have two items in it"
            );
            assert_eq!(
                &modules.clone(),
                local_module_list,
                "Local module list should match module list from response"
            );
        })
    });

    workspace_b.update(cx_b, |workspace, cx| {
        let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
        let debug_panel_item = debug_panel
            .update(cx, |this, cx| this.active_debug_panel_item(cx))
            .unwrap();

        debug_panel_item.update(cx, |item, cx| {
            assert_eq!(
                true,
                item.capabilities(cx).supports_modules_request.unwrap(),
                "Remote capabilities supports modules request should be true"
            );
            let remote_module_list = item.module_list().read(cx).modules();

            assert_eq!(
                2usize,
                remote_module_list.len(),
                "Remote module list should have two items in it"
            );
            assert_eq!(
                &modules.clone(),
                remote_module_list,
                "Remote module list should match module list from response"
            );
        })
    });

    let project_c = client_c.join_remote_project(project_id, cx_c).await;
    active_call_c
        .update(cx_c, |call, cx| call.set_location(Some(&project_c), cx))
        .await
        .unwrap();

    let (workspace_c, cx_c) = client_c.build_workspace(&project_c, cx_c);
    add_debugger_panel(&workspace_c, cx_c).await;
    cx_c.run_until_parked();

    workspace_c.update(cx_c, |workspace, cx| {
        let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
        let debug_panel_item = debug_panel
            .update(cx, |this, cx| this.active_debug_panel_item(cx))
            .unwrap();

        debug_panel_item.update(cx, |item, cx| {
            assert_eq!(
                true,
                item.capabilities(cx).supports_modules_request.unwrap(),
                "Remote (mid session join) capabilities supports modules request should be true"
            );
            let remote_module_list = item.module_list().read(cx).modules();

            assert_eq!(
                2usize,
                remote_module_list.len(),
                "Remote (mid session join) module list should have two items in it"
            );
            assert_eq!(
                &modules.clone(),
                remote_module_list,
                "Remote (mid session join) module list should match module list from response"
            );
        })
    });

    client.on_request::<Disconnect, _>(move |_, _| Ok(())).await;

    let shutdown_client = project_a.update(cx_a, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
        })
    });

    shutdown_client.await.unwrap();
}
