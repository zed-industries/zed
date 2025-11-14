use call::ActiveCall;
use dap::DebugRequestType;
use dap::requests::{Initialize, Launch, StackTrace};
use dap::{SourceBreakpoint, requests::SetBreakpoints};
use debugger_ui::debugger_panel::DebugPanel;
use debugger_ui::session::DebugSession;
use editor::Editor;
use gpui::{Entity, TestAppContext, VisualTestContext};
use project::{Project, ProjectPath, WorktreeId};
use serde_json::json;
use std::sync::Arc;
use std::{
    path::Path,
    sync::atomic::{AtomicBool, Ordering},
};
use workspace::{Workspace, dock::Panel};

use super::{TestClient, TestServer};

pub fn init_test(cx: &mut gpui::TestAppContext) {
    zlog::init_test();

    cx.update(|cx| {
        theme::init(theme::LoadThemes::JustBase, cx);
        command_palette_hooks::init(cx);
        debugger_ui::init(cx);
        editor::init(cx);
    });
}

async fn add_debugger_panel(workspace: &Entity<Workspace>, cx: &mut VisualTestContext) {
    let debugger_panel = workspace
        .update_in(cx, |_workspace, window, cx| {
            cx.spawn_in(window, DebugPanel::load)
        })
        .await
        .unwrap();

    workspace.update_in(cx, |workspace, window, cx| {
        workspace.add_panel(debugger_panel, window, cx);
    });
}

pub fn _active_session(
    workspace: Entity<Workspace>,
    cx: &mut VisualTestContext,
) -> Entity<DebugSession> {
    workspace.update_in(cx, |workspace, _window, cx| {
        let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
        debug_panel
            .update(cx, |this, cx| this.active_session(cx))
            .unwrap()
    })
}

struct ZedInstance<'a> {
    client: TestClient,
    project: Option<Entity<Project>>,
    active_call: Entity<ActiveCall>,
    cx: &'a mut TestAppContext,
}

impl<'a> ZedInstance<'a> {
    fn new(client: TestClient, cx: &'a mut TestAppContext) -> Self {
        ZedInstance {
            project: None,
            client,
            active_call: cx.read(ActiveCall::global),
            cx,
        }
    }

    async fn host_project(
        &mut self,
        project_files: Option<serde_json::Value>,
    ) -> (u64, WorktreeId) {
        let (project, worktree_id) = self.client.build_local_project("/project", self.cx).await;
        self.active_call
            .update(self.cx, |call, cx| call.set_location(Some(&project), cx))
            .await
            .unwrap();

        if let Some(tree) = project_files {
            self.client.fs().insert_tree("/project", tree).await;
        }

        self.project = Some(project.clone());

        let project_id = self
            .active_call
            .update(self.cx, |call, cx| call.share_project(project, cx))
            .await
            .unwrap();

        (project_id, worktree_id)
    }

    async fn join_project(&mut self, project_id: u64) {
        let remote_project = self.client.join_remote_project(project_id, self.cx).await;
        self.project = Some(remote_project);

        self.active_call
            .update(self.cx, |call, cx| {
                call.set_location(self.project.as_ref(), cx)
            })
            .await
            .unwrap();
    }

    async fn expand(
        &'a mut self,
    ) -> (
        &'a TestClient,
        Entity<Workspace>,
        Entity<Project>,
        &'a mut VisualTestContext,
    ) {
        let (workspace, cx) = self.client.build_workspace(
            self.project
                .as_ref()
                .expect("Project should be hosted or built before expanding"),
            self.cx,
        );
        add_debugger_panel(&workspace, cx).await;
        (&self.client, workspace, self.project.clone().unwrap(), cx)
    }
}

async fn _setup_three_member_test<'a, 'b, 'c>(
    server: &mut TestServer,
    host_cx: &'a mut TestAppContext,
    first_remote_cx: &'b mut TestAppContext,
    second_remote_cx: &'c mut TestAppContext,
) -> (ZedInstance<'a>, ZedInstance<'b>, ZedInstance<'c>) {
    let host_client = server.create_client(host_cx, "user_host").await;
    let first_remote_client = server.create_client(first_remote_cx, "user_remote_1").await;
    let second_remote_client = server
        .create_client(second_remote_cx, "user_remote_2")
        .await;

    init_test(host_cx);
    init_test(first_remote_cx);
    init_test(second_remote_cx);

    server
        .create_room(&mut [
            (&host_client, host_cx),
            (&first_remote_client, first_remote_cx),
            (&second_remote_client, second_remote_cx),
        ])
        .await;

    let host_zed = ZedInstance::new(host_client, host_cx);
    let first_remote_zed = ZedInstance::new(first_remote_client, first_remote_cx);
    let second_remote_zed = ZedInstance::new(second_remote_client, second_remote_cx);

    (host_zed, first_remote_zed, second_remote_zed)
}

async fn setup_two_member_test<'a, 'b>(
    server: &mut TestServer,
    host_cx: &'a mut TestAppContext,
    remote_cx: &'b mut TestAppContext,
) -> (ZedInstance<'a>, ZedInstance<'b>) {
    let host_client = server.create_client(host_cx, "user_host").await;
    let remote_client = server.create_client(remote_cx, "user_remote").await;

    init_test(host_cx);
    init_test(remote_cx);

    server
        .create_room(&mut [(&host_client, host_cx), (&remote_client, remote_cx)])
        .await;

    let host_zed = ZedInstance::new(host_client, host_cx);
    let remote_zed = ZedInstance::new(remote_client, remote_cx);

    (host_zed, remote_zed)
}

#[gpui::test]
async fn test_debug_panel_item_opens_on_remote(
    host_cx: &mut TestAppContext,
    remote_cx: &mut TestAppContext,
) {
    let executor = host_cx.executor();
    let mut server = TestServer::start(executor).await;

    let (mut host_zed, mut remote_zed) =
        setup_two_member_test(&mut server, host_cx, remote_cx).await;

    let (host_project_id, _) = host_zed.host_project(None).await;
    remote_zed.join_project(host_project_id).await;

    let (_client_host, _host_workspace, host_project, host_cx) = host_zed.expand().await;
    let (_client_remote, remote_workspace, _remote_project, remote_cx) = remote_zed.expand().await;

    remote_cx.run_until_parked();

    let task = host_project.update(host_cx, |project, cx| {
        project.start_debug_session(dap::test_config(DebugRequestType::Launch, None, None), cx)
    });

    let session = task.await.unwrap();
    let client = session.read_with(host_cx, |project, _| project.adapter_client().unwrap());

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

    host_cx.run_until_parked();
    remote_cx.run_until_parked();

    remote_workspace.update(remote_cx, |workspace, cx| {
        let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
        let _active_session = debug_panel
            .update(cx, |this, cx| this.active_session(cx))
            .unwrap();

        assert_eq!(
            1,
            debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
        );
        // assert_eq!(client.id(), active_session.read(cx).());
        // assert_eq!(1, active_session.read(cx).thread_id().0);
        // todo(debugger) check selected thread id
    });

    let shutdown_client = host_project.update(host_cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_client.await.unwrap();
}

#[gpui::test]
async fn test_active_debug_panel_item_set_on_join_project(
    host_cx: &mut TestAppContext,
    remote_cx: &mut TestAppContext,
) {
    let executor = host_cx.executor();
    let mut server = TestServer::start(executor).await;

    let (mut host_zed, mut remote_zed) =
        setup_two_member_test(&mut server, host_cx, remote_cx).await;

    let (host_project_id, _) = host_zed.host_project(None).await;

    let (_client_host, _host_workspace, host_project, host_cx) = host_zed.expand().await;

    host_cx.run_until_parked();

    let task = host_project.update(host_cx, |project, cx| {
        project.start_debug_session(dap::test_config(DebugRequestType::Launch, None, None), cx)
    });

    let session = task.await.unwrap();
    let client = session.read_with(host_cx, |project, _| project.adapter_client().unwrap());

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

    // Give host_client time to send a debug panel item to collab server
    host_cx.run_until_parked();

    remote_zed.join_project(host_project_id).await;
    let (_client_remote, remote_workspace, _remote_project, remote_cx) = remote_zed.expand().await;

    host_cx.run_until_parked();
    remote_cx.run_until_parked();

    remote_workspace.update(remote_cx, |workspace, cx| {
        let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
        let _active_session = debug_panel
            .update(cx, |this, cx| this.active_session(cx))
            .unwrap();

        assert_eq!(
            1,
            debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
        );
        // assert_eq!(cl, active_session.read(cx).client_id());
        // assert_eq!(1, active_session.read(cx).thread_id().0);
        // todo(debugger)
    });

    let shutdown_client = host_project.update(host_cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_client.await.unwrap();

    remote_cx.run_until_parked();

    // assert we don't have a debug panel item anymore because the client shutdown
    remote_workspace.update(remote_cx, |workspace, cx| {
        let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

        debug_panel.update(cx, |this, cx| {
            assert!(this.active_session(cx).is_none());
            assert_eq!(0, this.pane().unwrap().read(cx).items_len());
        });
    });
}

#[gpui::test]
async fn test_debug_panel_remote_button_presses(
    _host_cx: &mut TestAppContext,
    _remote_cx: &mut TestAppContext,
) {
    unimplemented!("Collab is still being refactored");
    // let executor = host_cx.executor();
    // let mut server = TestServer::start(executor).await;

    // let (mut host_zed, mut remote_zed) =
    //     setup_two_member_test(&mut server, host_cx, remote_cx).await;

    // let (host_project_id, _) = host_zed.host_project(None).await;
    // remote_zed.join_project(host_project_id).await;

    // let (_client_host, host_workspace, host_project, host_cx) = host_zed.expand().await;
    // let (_client_remote, remote_workspace, _remote_project, remote_cx) = remote_zed.expand().await;

    // let task = host_project.update(host_cx, |project, cx| {
    //     project.start_debug_session(dap::test_config(None), cx)
    // });

    // let session = task.await.unwrap();
    // let client = session.read_with(host_cx, |project, _| project.adapter_client().unwrap());

    // client
    //     .on_request::<Initialize, _>(move |_, _| {
    //         Ok(dap::Capabilities {
    //             supports_step_back: Some(true),
    //             ..Default::default()
    //         })
    //     })
    //     .await;

    // client.on_request::<Launch, _>(move |_, _| Ok(())).await;

    // client
    //     .on_request::<StackTrace, _>(move |_, _| {
    //         Ok(dap::StackTraceResponse {
    //             stack_frames: Vec::default(),
    //             total_frames: None,
    //         })
    //     })
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
    //         reason: dap::StoppedEventReason::Pause,
    //         description: None,
    //         thread_id: Some(1),
    //         preserve_focus_hint: None,
    //         text: None,
    //         all_threads_stopped: None,
    //         hit_breakpoint_ids: None,
    //     }))
    //     .await;

    // client
    //     .on_request::<dap::requests::Continue, _>(move |_, _| {
    //         Ok(dap::ContinueResponse {
    //             all_threads_continued: Some(true),
    //         })
    //     })
    //     .await;

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();

    // let remote_debug_item = remote_workspace.update(remote_cx, |workspace, cx| {
    //     let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
    //     let active_session = debug_panel
    //         .update(cx, |this, cx| this.active_session(cx))
    //         .unwrap();

    //     assert_eq!(
    //         1,
    //         debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
    //     );
    //     // assert_eq!(client.id(), active_session.read(cx).client_id());
    //     // assert_eq!(1, active_session.read(cx).thread_id().0);
    //     // todo(debugger)
    //     active_session
    // });

    // let local_debug_item = host_workspace.update(host_cx, |workspace, cx| {
    //     let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
    //     let active_session = debug_panel
    //         .update(cx, |this, cx| this.active_session(cx))
    //         .unwrap();

    //     assert_eq!(
    //         1,
    //         debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
    //     );
    //     // assert_eq!(client.id(), active_session.read(cx).client_id());
    //     // assert_eq!(1, active_session.read(cx).thread_id().0);
    //     // todo(debugger)
    //     active_session
    // });

    // remote_debug_item.update(remote_cx, |this, cx| {
    //     this.continue_thread(cx);
    // });

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();

    // local_debug_item.update(host_cx, |debug_panel_item, cx| {
    //     assert_eq!(
    //         debugger_ui::debugger_panel::ThreadStatus::Running,
    //         debug_panel_item.thread_state().read(cx).status,
    //     );
    // });

    // remote_debug_item.update(remote_cx, |debug_panel_item, cx| {
    //     assert_eq!(
    //         debugger_ui::debugger_panel::ThreadStatus::Running,
    //         debug_panel_item.thread_state().read(cx).status,
    //     );
    // });

    // client
    //     .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
    //         reason: dap::StoppedEventReason::Pause,
    //         description: None,
    //         thread_id: Some(1),
    //         preserve_focus_hint: None,
    //         text: None,
    //         all_threads_stopped: None,
    //         hit_breakpoint_ids: None,
    //     }))
    //     .await;

    // client
    //     .on_request::<StackTrace, _>(move |_, _| {
    //         Ok(dap::StackTraceResponse {
    //             stack_frames: Vec::default(),
    //             total_frames: None,
    //         })
    //     })
    //     .await;

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();

    // local_debug_item.update(host_cx, |debug_panel_item, cx| {
    //     assert_eq!(
    //         debugger_ui::debugger_panel::ThreadStatus::Stopped,
    //         debug_panel_item.thread_state().read(cx).status,
    //     );
    // });

    // remote_debug_item.update(remote_cx, |debug_panel_item, cx| {
    //     assert_eq!(
    //         debugger_ui::debugger_panel::ThreadStatus::Stopped,
    //         debug_panel_item.thread_state().read(cx).status,
    //     );
    // });

    // client
    //     .on_request::<dap::requests::Continue, _>(move |_, _| {
    //         Ok(dap::ContinueResponse {
    //             all_threads_continued: Some(true),
    //         })
    //     })
    //     .await;

    // local_debug_item.update(host_cx, |this, cx| {
    //     this.continue_thread(cx);
    // });

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();

    // local_debug_item.update(host_cx, |debug_panel_item, cx| {
    //     assert_eq!(
    //         debugger_ui::debugger_panel::ThreadStatus::Running,
    //         debug_panel_item.thread_state().read(cx).status,
    //     );
    // });

    // remote_debug_item.update(remote_cx, |debug_panel_item, cx| {
    //     assert_eq!(
    //         debugger_ui::debugger_panel::ThreadStatus::Running,
    //         debug_panel_item.thread_state().read(cx).status,
    //     );
    // });

    // client
    //     .on_request::<dap::requests::Pause, _>(move |_, _| Ok(()))
    //     .await;

    // client
    //     .on_request::<StackTrace, _>(move |_, _| {
    //         Ok(dap::StackTraceResponse {
    //             stack_frames: Vec::default(),
    //             total_frames: None,
    //         })
    //     })
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
    //         reason: dap::StoppedEventReason::Pause,
    //         description: None,
    //         thread_id: Some(1),
    //         preserve_focus_hint: None,
    //         text: None,
    //         all_threads_stopped: None,
    //         hit_breakpoint_ids: None,
    //     }))
    //     .await;

    // remote_debug_item.update(remote_cx, |this, cx| {
    //     this.pause_thread(cx);
    // });

    // remote_cx.run_until_parked();
    // host_cx.run_until_parked();

    // client
    //     .on_request::<dap::requests::StepOut, _>(move |_, _| Ok(()))
    //     .await;

    // remote_debug_item.update(remote_cx, |this, cx| {
    //     this.step_out(cx);
    // });

    // client
    //     .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
    //         reason: dap::StoppedEventReason::Pause,
    //         description: None,
    //         thread_id: Some(1),
    //         preserve_focus_hint: None,
    //         text: None,
    //         all_threads_stopped: None,
    //         hit_breakpoint_ids: None,
    //     }))
    //     .await;

    // remote_cx.run_until_parked();
    // host_cx.run_until_parked();

    // client
    //     .on_request::<dap::requests::Next, _>(move |_, _| Ok(()))
    //     .await;

    // remote_debug_item.update(remote_cx, |this, cx| {
    //     this.step_over(cx);
    // });

    // client
    //     .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
    //         reason: dap::StoppedEventReason::Pause,
    //         description: None,
    //         thread_id: Some(1),
    //         preserve_focus_hint: None,
    //         text: None,
    //         all_threads_stopped: None,
    //         hit_breakpoint_ids: None,
    //     }))
    //     .await;

    // remote_cx.run_until_parked();
    // host_cx.run_until_parked();

    // client
    //     .on_request::<dap::requests::StepIn, _>(move |_, _| Ok(()))
    //     .await;

    // remote_debug_item.update(remote_cx, |this, cx| {
    //     this.step_in(cx);
    // });

    // client
    //     .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
    //         reason: dap::StoppedEventReason::Pause,
    //         description: None,
    //         thread_id: Some(1),
    //         preserve_focus_hint: None,
    //         text: None,
    //         all_threads_stopped: None,
    //         hit_breakpoint_ids: None,
    //     }))
    //     .await;

    // remote_cx.run_until_parked();
    // host_cx.run_until_parked();

    // client
    //     .on_request::<dap::requests::StepBack, _>(move |_, _| Ok(()))
    //     .await;

    // remote_debug_item.update(remote_cx, |this, cx| {
    //     this.step_back(cx);
    // });

    // client
    //     .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
    //         reason: dap::StoppedEventReason::Pause,
    //         description: None,
    //         thread_id: Some(1),
    //         preserve_focus_hint: None,
    //         text: None,
    //         all_threads_stopped: None,
    //         hit_breakpoint_ids: None,
    //     }))
    //     .await;

    // remote_cx.run_until_parked();
    // host_cx.run_until_parked();

    // remote_debug_item.update(remote_cx, |this, cx| {
    //     this.stop_thread(cx);
    // });

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();

    // // assert we don't have a debug panel item anymore because the client shutdown
    // remote_workspace.update(remote_cx, |workspace, cx| {
    //     let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();

    //     debug_panel.update(cx, |this, cx| {
    //         assert!(this.active_session(cx).is_none());
    //         assert_eq!(0, this.pane().unwrap().read(cx).items_len());
    //     });
    // });
}

#[gpui::test]
async fn test_restart_stack_frame(_host_cx: &mut TestAppContext, _remote_cx: &mut TestAppContext) {
    unimplemented!("Collab is still being refactored");
    // let executor = host_cx.executor();
    // let mut server = TestServer::start(executor).await;

    // let (mut host_zed, mut remote_zed) =
    //     setup_two_member_test(&mut server, host_cx, remote_cx).await;

    // let (host_project_id, _) = host_zed.host_project(None).await;
    // remote_zed.join_project(host_project_id).await;

    // let (_client_host, _host_workspace, host_project, host_cx) = host_zed.expand().await;
    // let (_client_remote, remote_workspace, _remote_project, remote_cx) = remote_zed.expand().await;

    // let called_restart_frame = Arc::new(AtomicBool::new(false));

    // let task = host_project.update(host_cx, |project, cx| {
    //     project.start_debug_session(dap::test_config(None), cx)
    // });

    // let session = task.await.unwrap();
    // let client = session.read(cx).adapter_client().unwrap();

    // client
    //     .on_request::<Initialize, _>(move |_, _| {
    //         Ok(dap::Capabilities {
    //             supports_restart_frame: Some(true),
    //             ..Default::default()
    //         })
    //     })
    //     .await;

    // client.on_request::<Launch, _>(move |_, _| Ok(())).await;

    // let stack_frames = vec![StackFrame {
    //     id: 1,
    //     name: "Stack Frame 1".into(),
    //     source: Some(dap::Source {
    //         name: Some("test.js".into()),
    //         path: Some("/project/src/test.js".into()),
    //         source_reference: None,
    //         presentation_hint: None,
    //         origin: None,
    //         sources: None,
    //         adapter_data: None,
    //         checksums: None,
    //     }),
    //     line: 3,
    //     column: 1,
    //     end_line: None,
    //     end_column: None,
    //     can_restart: None,
    //     instruction_pointer_reference: None,
    //     module_id: None,
    //     presentation_hint: None,
    // }];

    // client
    //     .on_request::<StackTrace, _>({
    //         let stack_frames = Arc::new(stack_frames.clone());
    //         move |_, args| {
    //             assert_eq!(1, args.thread_id);

    //             Ok(dap::StackTraceResponse {
    //                 stack_frames: (*stack_frames).clone(),
    //                 total_frames: None,
    //             })
    //         }
    //     })
    //     .await;

    // client
    //     .on_request::<RestartFrame, _>({
    //         let called_restart_frame = called_restart_frame.clone();
    //         move |_, args| {
    //             assert_eq!(1, args.frame_id);

    //             called_restart_frame.store(true, Ordering::SeqCst);

    //             Ok(())
    //         }
    //     })
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
    //         reason: dap::StoppedEventReason::Pause,
    //         description: None,
    //         thread_id: Some(1),
    //         preserve_focus_hint: None,
    //         text: None,
    //         all_threads_stopped: None,
    //         hit_breakpoint_ids: None,
    //     }))
    //     .await;

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();

    // // try to restart stack frame 1 from the guest side
    // remote_workspace.update(remote_cx, |workspace, cx| {
    //     let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
    //     let active_session = debug_panel
    //         .update(cx, |this, cx| this.active_session(cx))
    //         .unwrap();

    //     active_session.update(cx, |debug_panel_item, cx| {
    //         debug_panel_item
    //             .stack_frame_list()
    //             .update(cx, |stack_frame_list, cx| {
    //                 stack_frame_list.restart_stack_frame(1, cx);
    //             });
    //     });
    // });

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();

    // assert!(
    //     called_restart_frame.load(std::sync::atomic::Ordering::SeqCst),
    //     "Restart stack frame was not called"
    // );

    // let shutdown_client = host_project.update(host_cx, |project, cx| {
    //     project.dap_store().update(cx, |dap_store, cx| {
    //         dap_store.shutdown_session(&session.read(cx).session_id(), cx)
    //     })
    // });

    // shutdown_client.await.unwrap();
}

#[gpui::test]
async fn test_updated_breakpoints_send_to_dap(
    host_cx: &mut TestAppContext,
    remote_cx: &mut TestAppContext,
) {
    let executor = host_cx.executor();
    let mut server = TestServer::start(executor).await;

    let (mut host_zed, mut remote_zed) =
        setup_two_member_test(&mut server, host_cx, remote_cx).await;

    let (host_project_id, worktree_id) = host_zed
        .host_project(Some(json!({"test.txt": "one\ntwo\nthree\nfour\nfive"})))
        .await;

    remote_zed.join_project(host_project_id).await;

    let (_client_host, host_workspace, host_project, host_cx) = host_zed.expand().await;
    let (_client_remote, remote_workspace, _remote_project, remote_cx) = remote_zed.expand().await;

    let project_path = ProjectPath {
        worktree_id,
        path: Arc::from(Path::new(&"test.txt")),
    };

    let task = host_project.update(host_cx, |project, cx| {
        project.start_debug_session(dap::test_config(DebugRequestType::Launch, None, None), cx)
    });

    let session = task.await.unwrap();
    let client = session.read_with(host_cx, |project, _| project.adapter_client().unwrap());

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
                assert_eq!("/project/test.txt", args.source.path.unwrap());
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
                // assert!(!args.source_modified.unwrap());
                // todo(debugger): Implement source_modified handling

                called_set_breakpoints.store(true, Ordering::SeqCst);

                Ok(dap::SetBreakpointsResponse {
                    breakpoints: Vec::default(),
                })
            }
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

    host_cx.run_until_parked();
    remote_cx.run_until_parked();

    // Client B opens an editor.
    let editor_b = remote_workspace
        .update_in(remote_cx, |workspace, window, cx| {
            workspace.open_path(project_path.clone(), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    editor_b.update_in(remote_cx, |editor, window, cx| {
        editor.move_down(&editor::actions::MoveDown, window, cx);
        editor.move_down(&editor::actions::MoveDown, window, cx);
        editor.toggle_breakpoint(&editor::actions::ToggleBreakpoint, window, cx);
    });

    // Client A opens an editor.
    let editor_a = host_workspace
        .update_in(host_cx, |workspace, window, cx| {
            workspace.open_path(project_path.clone(), None, true, window, cx)
        })
        .await
        .unwrap()
        .downcast::<Editor>()
        .unwrap();

    host_cx.run_until_parked();
    remote_cx.run_until_parked();

    let called_set_breakpoints = Arc::new(AtomicBool::new(false));
    client
        .on_request::<SetBreakpoints, _>({
            let called_set_breakpoints = called_set_breakpoints.clone();
            move |_, args| {
                assert_eq!("/project/test.txt", args.source.path.unwrap());
                assert!(args.breakpoints.unwrap().is_empty());
                // assert!(!args.source_modified.unwrap());
                // todo(debugger) Implement source modified support

                called_set_breakpoints.store(true, Ordering::SeqCst);

                Ok(dap::SetBreakpointsResponse {
                    breakpoints: Vec::default(),
                })
            }
        })
        .await;

    // remove the breakpoint that client B added
    editor_a.update_in(host_cx, |editor, window, cx| {
        editor.move_down(&editor::actions::MoveDown, window, cx);
        editor.move_down(&editor::actions::MoveDown, window, cx);
        editor.toggle_breakpoint(&editor::actions::ToggleBreakpoint, window, cx);
    });

    host_cx.run_until_parked();
    remote_cx.run_until_parked();

    assert!(
        called_set_breakpoints.load(std::sync::atomic::Ordering::SeqCst),
        "SetBreakpoint request must be called"
    );

    let called_set_breakpoints = Arc::new(AtomicBool::new(false));
    client
        .on_request::<SetBreakpoints, _>({
            let called_set_breakpoints = called_set_breakpoints.clone();
            move |_, args| {
                assert_eq!("/project/test.txt", args.source.path.unwrap());
                let mut breakpoints = args.breakpoints.unwrap();
                breakpoints.sort_by_key(|b| b.line);
                assert_eq!(
                    vec![
                        SourceBreakpoint {
                            line: 2,
                            column: None,
                            condition: None,
                            hit_condition: None,
                            log_message: None,
                            mode: None
                        },
                        SourceBreakpoint {
                            line: 3,
                            column: None,
                            condition: None,
                            hit_condition: None,
                            log_message: None,
                            mode: None
                        }
                    ],
                    breakpoints
                );
                // assert!(!args.source_modified.unwrap());
                // todo(debugger) Implement source modified support

                called_set_breakpoints.store(true, Ordering::SeqCst);

                Ok(dap::SetBreakpointsResponse {
                    breakpoints: Vec::default(),
                })
            }
        })
        .await;

    // Add our own breakpoint now
    editor_a.update_in(host_cx, |editor, window, cx| {
        editor.toggle_breakpoint(&editor::actions::ToggleBreakpoint, window, cx);
        editor.move_up(&editor::actions::MoveUp, window, cx);
        editor.toggle_breakpoint(&editor::actions::ToggleBreakpoint, window, cx);
    });

    host_cx.run_until_parked();
    remote_cx.run_until_parked();

    assert!(
        called_set_breakpoints.load(std::sync::atomic::Ordering::SeqCst),
        "SetBreakpoint request must be called"
    );

    let shutdown_client = host_project.update(host_cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_client.await.unwrap();
}

#[gpui::test]
async fn test_module_list(
    _host_cx: &mut TestAppContext,
    _remote_cx: &mut TestAppContext,
    _late_join_cx: &mut TestAppContext,
) {
    unimplemented!("Collab is still being refactored");
    // let executor = host_cx.executor();
    // let mut server = TestServer::start(executor).await;

    // let (mut host_zed, mut remote_zed, mut late_join_zed) =
    //     setup_three_member_test(&mut server, host_cx, remote_cx, late_join_cx).await;

    // let (host_project_id, _worktree_id) = host_zed.host_project(None).await;

    // remote_zed.join_project(host_project_id).await;

    // let (_client_host, host_workspace, host_project, host_cx) = host_zed.expand().await;
    // let (_client_remote, remote_workspace, _remote_project, remote_cx) = remote_zed.expand().await;

    // let task = host_project.update(host_cx, |project, cx| {
    //     project.start_debug_session(dap::test_config(None), cx)
    // });

    // let session = task.await.unwrap();
    // let client = session.read_with(host_cx, |project, _| project.adapter_client().unwrap());

    // let called_initialize = Arc::new(AtomicBool::new(false));

    // client
    //     .on_request::<Initialize, _>({
    //         let called_initialize = called_initialize.clone();
    //         move |_, _| {
    //             called_initialize.store(true, Ordering::SeqCst);
    //             Ok(dap::Capabilities {
    //                 supports_restart_frame: Some(true),
    //                 supports_modules_request: Some(true),
    //                 ..Default::default()
    //             })
    //         }
    //     })
    //     .await;

    // client.on_request::<Launch, _>(move |_, _| Ok(())).await;
    // client
    //     .on_request::<StackTrace, _>(move |_, _| {
    //         Ok(dap::StackTraceResponse {
    //             stack_frames: Vec::default(),
    //             total_frames: None,
    //         })
    //     })
    //     .await;

    // let called_modules = Arc::new(AtomicBool::new(false));
    // let modules = vec![
    //     dap::Module {
    //         id: dap::ModuleId::Number(1),
    //         name: "First Module".into(),
    //         address_range: None,
    //         date_time_stamp: None,
    //         path: None,
    //         symbol_file_path: None,
    //         symbol_status: None,
    //         version: None,
    //         is_optimized: None,
    //         is_user_code: None,
    //     },
    //     dap::Module {
    //         id: dap::ModuleId::Number(2),
    //         name: "Second Module".into(),
    //         address_range: None,
    //         date_time_stamp: None,
    //         path: None,
    //         symbol_file_path: None,
    //         symbol_status: None,
    //         version: None,
    //         is_optimized: None,
    //         is_user_code: None,
    //     },
    // ];

    // client
    //     .on_request::<dap::requests::Modules, _>({
    //         let called_modules = called_modules.clone();
    //         let modules = modules.clone();
    //         move |_, _| unsafe {
    //             static mut REQUEST_COUNT: i32 = 1;
    //             assert_eq!(
    //                 1, REQUEST_COUNT,
    //                 "This request should only be called once from the host"
    //             );
    //             REQUEST_COUNT += 1;
    //             called_modules.store(true, Ordering::SeqCst);

    //             Ok(dap::ModulesResponse {
    //                 modules: modules.clone(),
    //                 total_modules: Some(2u64),
    //             })
    //         }
    //     })
    //     .await;

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();

    // assert!(
    //     called_initialize.load(std::sync::atomic::Ordering::SeqCst),
    //     "Request Initialize must be called"
    // );

    // client
    //     .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
    //         reason: dap::StoppedEventReason::Pause,
    //         description: None,
    //         thread_id: Some(1),
    //         preserve_focus_hint: None,
    //         text: None,
    //         all_threads_stopped: None,
    //         hit_breakpoint_ids: None,
    //     }))
    //     .await;

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();

    // assert!(
    //     called_modules.load(std::sync::atomic::Ordering::SeqCst),
    //     "Request Modules must be called"
    // );

    // host_workspace.update(host_cx, |workspace, cx| {
    //     let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
    //     let debug_panel_item = debug_panel
    //         .update(cx, |this, cx| this.active_session(cx))
    //         .unwrap();

    //     debug_panel_item.update(cx, |item, cx| {
    //         assert_eq!(
    //             true,
    //             item.capabilities(cx).supports_modules_request.unwrap(),
    //             "Local supports modules request should be true"
    //         );

    //         let local_module_list = item.module_list().update(cx, |list, cx| list.modules(cx));

    //         assert_eq!(
    //             2usize,
    //             local_module_list.len(),
    //             "Local module list should have two items in it"
    //         );
    //         assert_eq!(
    //             modules.clone(),
    //             local_module_list,
    //             "Local module list should match module list from response"
    //         );
    //     })
    // });

    // remote_workspace.update(remote_cx, |workspace, cx| {
    //     let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
    //     let debug_panel_item = debug_panel
    //         .update(cx, |this, cx| this.active_session(cx))
    //         .unwrap();

    //     debug_panel_item.update(cx, |item, cx| {
    //         assert_eq!(
    //             true,
    //             item.capabilities(cx).supports_modules_request.unwrap(),
    //             "Remote capabilities supports modules request should be true"
    //         );
    //         let remote_module_list = item.module_list().update(cx, |list, cx| list.modules(cx));

    //         assert_eq!(
    //             2usize,
    //             remote_module_list.len(),
    //             "Remote module list should have two items in it"
    //         );
    //         assert_eq!(
    //             modules.clone(),
    //             remote_module_list,
    //             "Remote module list should match module list from response"
    //         );
    //     })
    // });

    // late_join_zed.join_project(host_project_id).await;
    // let (_late_join_client, late_join_workspace, _late_join_project, late_join_cx) =
    //     late_join_zed.expand().await;

    // late_join_workspace.update(late_join_cx, |workspace, cx| {
    //     let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
    //     let debug_panel_item = debug_panel
    //         .update(cx, |this, cx| this.active_session(cx))
    //         .unwrap();

    //     debug_panel_item.update(cx, |item, cx| {
    //         assert_eq!(
    //             true,
    //             item.capabilities(cx).supports_modules_request.unwrap(),
    //             "Remote (mid session join) capabilities supports modules request should be true"
    //         );
    //         let remote_module_list = item.module_list().update(cx, |list, cx| list.modules(cx));

    //         assert_eq!(
    //             2usize,
    //             remote_module_list.len(),
    //             "Remote (mid session join) module list should have two items in it"
    //         );
    //         assert_eq!(
    //             modules.clone(),
    //             remote_module_list,
    //             "Remote (mid session join) module list should match module list from response"
    //         );
    //     })
    // });

    // let shutdown_client = host_project.update(host_cx, |project, cx| {
    //     project.dap_store().update(cx, |dap_store, cx| {
    //         dap_store.shutdown_session(&session.read(cx).id(), cx)
    //     })
    // });

    // shutdown_client.await.unwrap();
}

// #[gpui::test]
// async fn test_variable_list(
//     host_cx: &mut TestAppContext,
//     remote_cx: &mut TestAppContext,
//     late_join_cx: &mut TestAppContext,
// ) {
//     let executor = host_cx.executor();
//     let mut server = TestServer::start(executor).await;

//     let (mut host_zed, mut remote_zed, mut late_join_zed) =
//         setup_three_member_test(&mut server, host_cx, remote_cx, late_join_cx).await;

//     let (host_project_id, _worktree_id) = host_zed
//         .host_project(Some(json!({"test.txt": "one\ntwo\nthree\nfour\nfive"})))
//         .await;

//     remote_zed.join_project(host_project_id).await;

//     let (_client_host, host_workspace, host_project, host_cx) = host_zed.expand().await;
//     let (_client_remote, remote_workspace, _remote_project, remote_cx) = remote_zed.expand().await;

//     let task = host_project.update(host_cx, |project, cx| {
//         project.start_debug_session(
//             dap::DebugAdapterConfig {
//                 label: "test config".into(),
//                 kind: dap::DebugAdapterKind::Fake,
//                 request: dap::DebugRequestType::Launch,
//                 program: None,
//                 cwd: None,
//                 initialize_args: None,
//             },
//             cx,
//         )
//     });

//     let (session, client) = task.await.unwrap();

//     client
//         .on_request::<Initialize, _>(move |_, _| {
//             Ok(dap::Capabilities {
//                 supports_step_back: Some(true),
//                 ..Default::default()
//             })
//         })
//         .await;

//     client.on_request::<Launch, _>(move |_, _| Ok(())).await;

//     let stack_frames = vec![dap::StackFrame {
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
//         line: 1,
//         column: 1,
//         end_line: None,
//         end_column: None,
//         can_restart: None,
//         instruction_pointer_reference: None,
//         module_id: None,
//         presentation_hint: None,
//     }];

//     let scopes = vec![Scope {
//         name: "Scope 1".into(),
//         presentation_hint: None,
//         variables_reference: 1,
//         named_variables: None,
//         indexed_variables: None,
//         expensive: false,
//         source: None,
//         line: None,
//         column: None,
//         end_line: None,
//         end_column: None,
//     }];

//     let variable_1 = Variable {
//         name: "variable 1".into(),
//         value: "1".into(),
//         type_: None,
//         presentation_hint: None,
//         evaluate_name: None,
//         variables_reference: 2,
//         named_variables: None,
//         indexed_variables: None,
//         memory_reference: None,
//     };

//     let variable_2 = Variable {
//         name: "variable 2".into(),
//         value: "2".into(),
//         type_: None,
//         presentation_hint: None,
//         evaluate_name: None,
//         variables_reference: 3,
//         named_variables: None,
//         indexed_variables: None,
//         memory_reference: None,
//     };

//     let variable_3 = Variable {
//         name: "variable 3".into(),
//         value: "hello world".into(),
//         type_: None,
//         presentation_hint: None,
//         evaluate_name: None,
//         variables_reference: 4,
//         named_variables: None,
//         indexed_variables: None,
//         memory_reference: None,
//     };

//     let variable_4 = Variable {
//         name: "variable 4".into(),
//         value: "hello world this is the final variable".into(),
//         type_: None,
//         presentation_hint: None,
//         evaluate_name: None,
//         variables_reference: 0,
//         named_variables: None,
//         indexed_variables: None,
//         memory_reference: None,
//     };

//     client
//         .on_request::<StackTrace, _>({
//             let stack_frames = std::sync::Arc::new(stack_frames.clone());
//             move |_, args| {
//                 assert_eq!(1, args.thread_id);

//                 Ok(dap::StackTraceResponse {
//                     stack_frames: (*stack_frames).clone(),
//                     total_frames: None,
//                 })
//             }
//         })
//         .await;

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

//     let first_variable_request = vec![variable_1.clone(), variable_2.clone()];

//     client
//         .on_request::<Variables, _>({
//             move |_, args| {
//                 assert_eq!(1, args.variables_reference);

//                 Ok(dap::VariablesResponse {
//                     variables: first_variable_request.clone(),
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

//     host_cx.run_until_parked();
//     remote_cx.run_until_parked();

//     let local_debug_item = host_workspace.update(host_cx, |workspace, cx| {
//         let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
//         let active_debug_panel_item = debug_panel
//             .update(cx, |this, cx| this.active_debug_panel_item(cx))
//             .unwrap();

//         assert_eq!(
//             1,
//             debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
//         );
//         assert_eq!(client.id(), active_debug_panel_item.read(cx).client_id());
//         assert_eq!(1, active_debug_panel_item.read(cx).thread_id());
//         active_debug_panel_item
//     });

//     let remote_debug_item = remote_workspace.update(remote_cx, |workspace, cx| {
//         let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
//         let active_debug_panel_item = debug_panel
//             .update(cx, |this, cx| this.active_debug_panel_item(cx))
//             .unwrap();

//         assert_eq!(
//             1,
//             debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
//         );
//         assert_eq!(client.id(), active_debug_panel_item.read(cx).client_id());
//         assert_eq!(1, active_debug_panel_item.read(cx).thread_id());
//         active_debug_panel_item
//     });

//     let first_visual_entries = vec!["v Scope 1", "    > variable 1", "    > variable 2"];
//     let first_variable_containers = vec![
//         VariableContainer {
//             container_reference: scopes[0].variables_reference,
//             variable: variable_1.clone(),
//             depth: 1,
//         },
//         VariableContainer {
//             container_reference: scopes[0].variables_reference,
//             variable: variable_2.clone(),
//             depth: 1,
//         },
//     ];

//     local_debug_item
//         .update(host_cx, |this, _| this.variable_list().clone())
//         .update(host_cx, |variable_list, cx| {
//             assert_eq!(1, variable_list.scopes().len());
//             assert_eq!(scopes, variable_list.scopes().get(&1).unwrap().clone());
//             assert_eq!(&first_variable_containers, &variable_list.variables());

//             variable_list.assert_visual_entries(first_visual_entries.clone(), cx);
//         });

//     client
//         .on_request::<Variables, _>({
//             let variables = Arc::new(vec![variable_3.clone()]);
//             move |_, args| {
//                 assert_eq!(2, args.variables_reference);

//                 Ok(dap::VariablesResponse {
//                     variables: (*variables).clone(),
//                 })
//             }
//         })
//         .await;

//     remote_debug_item
//         .update(remote_cx, |this, _| this.variable_list().clone())
//         .update(remote_cx, |variable_list, cx| {
//             assert_eq!(1, variable_list.scopes().len());
//             assert_eq!(scopes, variable_list.scopes().get(&1).unwrap().clone());
//             assert_eq!(&first_variable_containers, &variable_list.variables());

//             variable_list.assert_visual_entries(first_visual_entries.clone(), cx);

//             variable_list.toggle_variable(&scopes[0], &variable_1, 1, cx);
//         });

//     host_cx.run_until_parked();
//     remote_cx.run_until_parked();

//     let second_req_variable_list = vec![
//         VariableContainer {
//             container_reference: scopes[0].variables_reference,
//             variable: variable_1.clone(),
//             depth: 1,
//         },
//         VariableContainer {
//             container_reference: variable_1.variables_reference,
//             variable: variable_3.clone(),
//             depth: 2,
//         },
//         VariableContainer {
//             container_reference: scopes[0].variables_reference,
//             variable: variable_2.clone(),
//             depth: 1,
//         },
//     ];

//     remote_debug_item
//         .update(remote_cx, |this, _| this.variable_list().clone())
//         .update(remote_cx, |variable_list, cx| {
//             assert_eq!(1, variable_list.scopes().len());
//             assert_eq!(3, variable_list.variables().len());
//             assert_eq!(scopes, variable_list.scopes().get(&1).unwrap().clone());
//             assert_eq!(&second_req_variable_list, &variable_list.variables());

//             variable_list.assert_visual_entries(
//                 vec![
//                     "v Scope 1",
//                     "    v variable 1",
//                     "        > variable 3",
//                     "    > variable 2",
//                 ],
//                 cx,
//             );
//         });

//     client
//         .on_request::<Variables, _>({
//             let variables = Arc::new(vec![variable_4.clone()]);
//             move |_, args| {
//                 assert_eq!(3, args.variables_reference);

//                 Ok(dap::VariablesResponse {
//                     variables: (*variables).clone(),
//                 })
//             }
//         })
//         .await;

//     local_debug_item
//         .update(host_cx, |this, _| this.variable_list().clone())
//         .update(host_cx, |variable_list, cx| {
//             assert_eq!(1, variable_list.scopes().len());
//             assert_eq!(3, variable_list.variables().len());
//             assert_eq!(scopes, variable_list.scopes().get(&1).unwrap().clone());
//             assert_eq!(&second_req_variable_list, &variable_list.variables());

//             variable_list.assert_visual_entries(first_visual_entries.clone(), cx);

//             variable_list.toggle_variable(&scopes[0], &variable_2.clone(), 1, cx);
//         });

//     host_cx.run_until_parked();
//     remote_cx.run_until_parked();

//     let final_variable_containers: Vec<VariableContainer> = vec![
//         VariableContainer {
//             container_reference: scopes[0].variables_reference,
//             variable: variable_1.clone(),
//             depth: 1,
//         },
//         VariableContainer {
//             container_reference: variable_1.variables_reference,
//             variable: variable_3.clone(),
//             depth: 2,
//         },
//         VariableContainer {
//             container_reference: scopes[0].variables_reference,
//             variable: variable_2.clone(),
//             depth: 1,
//         },
//         VariableContainer {
//             container_reference: variable_2.variables_reference,
//             variable: variable_4.clone(),
//             depth: 2,
//         },
//     ];

//     remote_debug_item
//         .update(remote_cx, |this, _| this.variable_list().clone())
//         .update(remote_cx, |variable_list, cx| {
//             assert_eq!(1, variable_list.scopes().len());
//             assert_eq!(4, variable_list.variables().len());
//             assert_eq!(scopes, variable_list.scopes().get(&1).unwrap().clone());
//             assert_eq!(&final_variable_containers, &variable_list.variables());

//             variable_list.assert_visual_entries(
//                 vec![
//                     "v Scope 1",
//                     "    v variable 1",
//                     "        > variable 3",
//                     "    > variable 2",
//                 ],
//                 cx,
//             );
//         });

//     local_debug_item
//         .update(host_cx, |this, _| this.variable_list().clone())
//         .update(host_cx, |variable_list, cx| {
//             assert_eq!(1, variable_list.scopes().len());
//             assert_eq!(4, variable_list.variables().len());
//             assert_eq!(scopes, variable_list.scopes().get(&1).unwrap().clone());
//             assert_eq!(&final_variable_containers, &variable_list.variables());

//             variable_list.assert_visual_entries(
//                 vec![
//                     "v Scope 1",
//                     "    > variable 1",
//                     "    v variable 2",
//                     "        > variable 4",
//                 ],
//                 cx,
//             );
//         });

//     late_join_zed.join_project(host_project_id).await;
//     let (_late_join_client, late_join_workspace, _late_join_project, late_join_cx) =
//         late_join_zed.expand().await;

//     late_join_cx.run_until_parked();

//     let last_join_remote_item = late_join_workspace.update(late_join_cx, |workspace, cx| {
//         let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
//         let active_debug_panel_item = debug_panel
//             .update(cx, |this, cx| this.active_debug_panel_item(cx))
//             .unwrap();

//         assert_eq!(
//             1,
//             debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
//         );
//         assert_eq!(client.id(), active_debug_panel_item.read(cx).client_id());
//         assert_eq!(1, active_debug_panel_item.read(cx).thread_id());
//         active_debug_panel_item
//     });

//     last_join_remote_item
//         .update(late_join_cx, |this, _| this.variable_list().clone())
//         .update(late_join_cx, |variable_list, cx| {
//             assert_eq!(1, variable_list.scopes().len());
//             assert_eq!(4, variable_list.variables().len());
//             assert_eq!(scopes, variable_list.scopes().get(&1).unwrap().clone());
//             assert_eq!(final_variable_containers, variable_list.variables());

//             variable_list.assert_visual_entries(first_visual_entries, cx);
//         });

//     let shutdown_client = host_project.update(host_cx, |project, cx| {
//         project.dap_store().update(cx, |dap_store, cx| {
//             dap_store.shutdown_session(&session.read(cx).id(), cx)
//         })
//     });

//     shutdown_client.await.unwrap();
// }

#[gpui::test]
async fn test_ignore_breakpoints(
    _host_cx: &mut TestAppContext,
    _remote_cx: &mut TestAppContext,
    _cx_c: &mut TestAppContext,
) {
    unimplemented!("Collab is still being refactored");
    // let executor = host_cx.executor();
    // let mut server = TestServer::start(executor).await;

    // let (mut host_zed, mut remote_zed, mut late_join_zed) =
    //     setup_three_member_test(&mut server, host_cx, remote_cx, cx_c).await;

    // let (host_project_id, worktree_id) = host_zed
    //     .host_project(Some(json!({"test.txt": "one\ntwo\nthree\nfour\nfive"})))
    //     .await;

    // remote_zed.join_project(host_project_id).await;

    // let (_client_host, host_workspace, host_project, host_cx) = host_zed.expand().await;
    // let (_client_remote, remote_workspace, remote_project, remote_cx) = remote_zed.expand().await;

    // let project_path = ProjectPath {
    //     worktree_id,
    //     path: Arc::from(Path::new(&"test.txt")),
    // };

    // let local_editor = host_workspace
    //     .update_in(host_cx, |workspace, window, cx| {
    //         workspace.open_path(project_path.clone(), None, true, window, cx)
    //     })
    //     .await
    //     .unwrap()
    //     .downcast::<Editor>()
    //     .unwrap();

    // local_editor.update_in(host_cx, |editor, window, cx| {
    //     editor.move_down(&editor::actions::MoveDown, window, cx);
    //     editor.toggle_breakpoint(&editor::actions::ToggleBreakpoint, window, cx); // Line 2
    //     editor.move_down(&editor::actions::MoveDown, window, cx);
    //     editor.toggle_breakpoint(&editor::actions::ToggleBreakpoint, window, cx);
    //     // Line 3
    // });

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();

    // let task = host_project.update(host_cx, |project, cx| {
    //     project.start_debug_session(dap::test_config(None), cx)
    // });

    // let session = task.await.unwrap();
    // let client = session.read_with(host_cx, |project, _| project.adapter_client().unwrap());
    // let client_id = client.id();

    // client
    //     .on_request::<Initialize, _>(move |_, _| {
    //         Ok(dap::Capabilities {
    //             supports_configuration_done_request: Some(true),
    //             ..Default::default()
    //         })
    //     })
    //     .await;

    // let called_set_breakpoints = Arc::new(AtomicBool::new(false));
    // client
    //     .on_request::<SetBreakpoints, _>({
    //         let called_set_breakpoints = called_set_breakpoints.clone();
    //         move |_, args| {
    //             assert_eq!("/project/test.txt", args.source.path.unwrap());

    //             let mut actual_breakpoints = args.breakpoints.unwrap();
    //             actual_breakpoints.sort_by_key(|b| b.line);

    //             let expected_breakpoints = vec![
    //                 SourceBreakpoint {
    //                     line: 2,
    //                     column: None,
    //                     condition: None,
    //                     hit_condition: None,
    //                     log_message: None,
    //                     mode: None,
    //                 },
    //                 SourceBreakpoint {
    //                     line: 3,
    //                     column: None,
    //                     condition: None,
    //                     hit_condition: None,
    //                     log_message: None,
    //                     mode: None,
    //                 },
    //             ];

    //             assert_eq!(actual_breakpoints, expected_breakpoints);

    //             called_set_breakpoints.store(true, Ordering::SeqCst);

    //             Ok(dap::SetBreakpointsResponse {
    //                 breakpoints: Vec::default(),
    //             })
    //         }
    //     })
    //     .await;

    // client.on_request::<Launch, _>(move |_, _| Ok(())).await;
    // client
    //     .on_request::<StackTrace, _>(move |_, _| {
    //         Ok(dap::StackTraceResponse {
    //             stack_frames: Vec::default(),
    //             total_frames: None,
    //         })
    //     })
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Initialized(Some(
    //         dap::Capabilities {
    //             supports_configuration_done_request: Some(true),
    //             ..Default::default()
    //         },
    //     )))
    //     .await;

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();

    // assert!(
    //     called_set_breakpoints.load(std::sync::atomic::Ordering::SeqCst),
    //     "SetBreakpoint request must be called when starting debug session"
    // );

    // client
    //     .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
    //         reason: dap::StoppedEventReason::Pause,
    //         description: None,
    //         thread_id: Some(1),
    //         preserve_focus_hint: None,
    //         text: None,
    //         all_threads_stopped: None,
    //         hit_breakpoint_ids: None,
    //     }))
    //     .await;

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();

    // let remote_debug_item = remote_workspace.update(remote_cx, |workspace, cx| {
    //     let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
    //     let active_session = debug_panel
    //         .update(cx, |this, cx| this.active_session(cx))
    //         .unwrap();

    //     assert_eq!(
    //         1,
    //         debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
    //     );

    //     let session_id = debug_panel.update(cx, |this, cx| {
    //         this.dap_store()
    //             .read(cx)
    //             .session_by_client_id(client.id())
    //             .unwrap()
    //             .read(cx)
    //             .id()
    //     });

    //     let breakpoints_ignored = active_session.read(cx).are_breakpoints_ignored(cx);

    //     assert_eq!(session_id, active_session.read(cx).session().read(cx).id());
    //     assert_eq!(false, breakpoints_ignored);
    //     assert_eq!(client.id(), active_session.read(cx).client_id());
    //     assert_eq!(1, active_session.read(cx).thread_id().0);
    //     active_session
    // });

    // called_set_breakpoints.store(false, Ordering::SeqCst);

    // client
    //     .on_request::<SetBreakpoints, _>({
    //         let called_set_breakpoints = called_set_breakpoints.clone();
    //         move |_, args| {
    //             assert_eq!("/project/test.txt", args.source.path.unwrap());
    //             assert_eq!(args.breakpoints, Some(vec![]));

    //             called_set_breakpoints.store(true, Ordering::SeqCst);

    //             Ok(dap::SetBreakpointsResponse {
    //                 breakpoints: Vec::default(),
    //             })
    //         }
    //     })
    //     .await;

    // let local_debug_item = host_workspace.update(host_cx, |workspace, cx| {
    //     let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
    //     let active_session = debug_panel
    //         .update(cx, |this, cx| this.active_session(cx))
    //         .unwrap();

    //     assert_eq!(
    //         1,
    //         debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
    //     );

    //     assert_eq!(false, active_session.read(cx).are_breakpoints_ignored(cx));
    //     assert_eq!(client.id(), active_session.read(cx).client_id());
    //     assert_eq!(1, active_session.read(cx).thread_id().0);

    //     active_session
    // });

    // local_debug_item.update(host_cx, |item, cx| {
    //     item.toggle_ignore_breakpoints(cx); // Set to true
    //     assert_eq!(true, item.are_breakpoints_ignored(cx));
    // });

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();

    // assert!(
    //     called_set_breakpoints.load(std::sync::atomic::Ordering::SeqCst),
    //     "SetBreakpoint request must be called to ignore breakpoints"
    // );

    // client
    //     .on_request::<SetBreakpoints, _>({
    //         let called_set_breakpoints = called_set_breakpoints.clone();
    //         move |_, _args| {
    //             called_set_breakpoints.store(true, Ordering::SeqCst);

    //             Ok(dap::SetBreakpointsResponse {
    //                 breakpoints: Vec::default(),
    //             })
    //         }
    //     })
    //     .await;

    // let remote_editor = remote_workspace
    //     .update_in(remote_cx, |workspace, window, cx| {
    //         workspace.open_path(project_path.clone(), None, true, window, cx)
    //     })
    //     .await
    //     .unwrap()
    //     .downcast::<Editor>()
    //     .unwrap();

    // called_set_breakpoints.store(false, std::sync::atomic::Ordering::SeqCst);

    // remote_editor.update_in(remote_cx, |editor, window, cx| {
    //     // Line 1
    //     editor.toggle_breakpoint(&editor::actions::ToggleBreakpoint, window, cx);
    // });

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();

    // assert!(
    //     called_set_breakpoints.load(std::sync::atomic::Ordering::SeqCst),
    //     "SetBreakpoint request be called whenever breakpoints are toggled but with not breakpoints"
    // );

    // remote_debug_item.update(remote_cx, |debug_panel, cx| {
    //     let breakpoints_ignored = debug_panel.are_breakpoints_ignored(cx);

    //     assert_eq!(true, breakpoints_ignored);
    //     assert_eq!(client.id(), debug_panel.client_id());
    //     assert_eq!(1, debug_panel.thread_id().0);
    // });

    // client
    //     .on_request::<SetBreakpoints, _>({
    //         let called_set_breakpoints = called_set_breakpoints.clone();
    //         move |_, args| {
    //             assert_eq!("/project/test.txt", args.source.path.unwrap());

    //             let mut actual_breakpoints = args.breakpoints.unwrap();
    //             actual_breakpoints.sort_by_key(|b| b.line);

    //             let expected_breakpoints = vec![
    //                 SourceBreakpoint {
    //                     line: 1,
    //                     column: None,
    //                     condition: None,
    //                     hit_condition: None,
    //                     log_message: None,
    //                     mode: None,
    //                 },
    //                 SourceBreakpoint {
    //                     line: 2,
    //                     column: None,
    //                     condition: None,
    //                     hit_condition: None,
    //                     log_message: None,
    //                     mode: None,
    //                 },
    //                 SourceBreakpoint {
    //                     line: 3,
    //                     column: None,
    //                     condition: None,
    //                     hit_condition: None,
    //                     log_message: None,
    //                     mode: None,
    //                 },
    //             ];

    //             assert_eq!(actual_breakpoints, expected_breakpoints);

    //             called_set_breakpoints.store(true, Ordering::SeqCst);

    //             Ok(dap::SetBreakpointsResponse {
    //                 breakpoints: Vec::default(),
    //             })
    //         }
    //     })
    //     .await;

    // late_join_zed.join_project(host_project_id).await;
    // let (_late_join_client, late_join_workspace, late_join_project, late_join_cx) =
    //     late_join_zed.expand().await;

    // late_join_cx.run_until_parked();

    // let last_join_remote_item = late_join_workspace.update(late_join_cx, |workspace, cx| {
    //     let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
    //     let active_session = debug_panel
    //         .update(cx, |this, cx| this.active_session(cx))
    //         .unwrap();

    //     let breakpoints_ignored = active_session.read(cx).are_breakpoints_ignored(cx);

    //     assert_eq!(true, breakpoints_ignored);

    //     assert_eq!(
    //         1,
    //         debug_panel.update(cx, |this, cx| this.pane().unwrap().read(cx).items_len())
    //     );
    //     assert_eq!(client.id(), active_session.read(cx).client_id());
    //     assert_eq!(1, active_session.read(cx).thread_id().0);
    //     active_session
    // });

    // remote_debug_item.update(remote_cx, |item, cx| {
    //     item.toggle_ignore_breakpoints(cx);
    // });

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();
    // late_join_cx.run_until_parked();

    // assert!(
    //     called_set_breakpoints.load(std::sync::atomic::Ordering::SeqCst),
    //     "SetBreakpoint request should be called to update breakpoints"
    // );

    // client
    //     .on_request::<SetBreakpoints, _>({
    //         let called_set_breakpoints = called_set_breakpoints.clone();
    //         move |_, args| {
    //             assert_eq!("/project/test.txt", args.source.path.unwrap());
    //             assert_eq!(args.breakpoints, Some(vec![]));

    //             called_set_breakpoints.store(true, Ordering::SeqCst);

    //             Ok(dap::SetBreakpointsResponse {
    //                 breakpoints: Vec::default(),
    //             })
    //         }
    //     })
    //     .await;

    // local_debug_item.update(host_cx, |debug_panel_item, cx| {
    //     assert_eq!(
    //         false,
    //         debug_panel_item.are_breakpoints_ignored(cx),
    //         "Remote client set this to false"
    //     );
    // });

    // remote_debug_item.update(remote_cx, |debug_panel_item, cx| {
    //     assert_eq!(
    //         false,
    //         debug_panel_item.are_breakpoints_ignored(cx),
    //         "Remote client set this to false"
    //     );
    // });

    // last_join_remote_item.update(late_join_cx, |debug_panel_item, cx| {
    //     assert_eq!(
    //         false,
    //         debug_panel_item.are_breakpoints_ignored(cx),
    //         "Remote client set this to false"
    //     );
    // });

    // let shutdown_client = host_project.update(host_cx, |project, cx| {
    //     project.dap_store().update(cx, |dap_store, cx| {
    //         dap_store.shutdown_session(&session.read(cx).id(), cx)
    //     })
    // });

    // shutdown_client.await.unwrap();

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();

    // remote_project.update(remote_cx, |project, cx| {
    //     project.dap_store().update(cx, |dap_store, _cx| {
    //         let sessions = dap_store.sessions().collect::<Vec<_>>();

    //         assert_eq!(
    //             None,
    //             dap_store.session_by_client_id(&client_id),
    //             "No client_id to session mapping should exist after shutdown"
    //         );
    //         assert_eq!(
    //             0,
    //             sessions.len(),
    //             "No sessions should be left after shutdown"
    //         );
    //     })
    // });

    // late_join_project.update(late_join_cx, |project, cx| {
    //     project.dap_store().update(cx, |dap_store, _cx| {
    //         let sessions = dap_store.sessions().collect::<Vec<_>>();

    //         assert_eq!(
    //             None,
    //             dap_store.session_by_client_id(&client_id),
    //             "No client_id to session mapping should exist after shutdown"
    //         );
    //         assert_eq!(
    //             0,
    //             sessions.len(),
    //             "No sessions should be left after shutdown"
    //         );
    //     })
    // });
}

#[gpui::test]
async fn test_debug_panel_console(_host_cx: &mut TestAppContext, _remote_cx: &mut TestAppContext) {
    unimplemented!("Collab is still being refactored");
    // let executor = host_cx.executor();
    // let mut server = TestServer::start(executor).await;

    // let (mut host_zed, mut remote_zed) =
    //     setup_two_member_test(&mut server, host_cx, remote_cx).await;

    // let (host_project_id, _) = host_zed.host_project(None).await;
    // remote_zed.join_project(host_project_id).await;

    // let (_client_host, _host_workspace, host_project, host_cx) = host_zed.expand().await;
    // let (_client_remote, remote_workspace, _remote_project, remote_cx) = remote_zed.expand().await;

    // remote_cx.run_until_parked();

    // let task = host_project.update(host_cx, |project, cx| {
    //     project.start_debug_session(dap::test_config(None), cx)
    // });

    // let session = task.await.unwrap();
    // let client = session.read_with(host_cx, |project, _| project.adapter_client().unwrap());

    // client
    //     .on_request::<Initialize, _>(move |_, _| {
    //         Ok(dap::Capabilities {
    //             supports_step_back: Some(false),
    //             ..Default::default()
    //         })
    //     })
    //     .await;

    // client.on_request::<Launch, _>(move |_, _| Ok(())).await;

    // client
    //     .on_request::<StackTrace, _>(move |_, _| {
    //         Ok(dap::StackTraceResponse {
    //             stack_frames: Vec::default(),
    //             total_frames: None,
    //         })
    //     })
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Stopped(dap::StoppedEvent {
    //         reason: dap::StoppedEventReason::Pause,
    //         description: None,
    //         thread_id: Some(1),
    //         preserve_focus_hint: None,
    //         text: None,
    //         all_threads_stopped: None,
    //         hit_breakpoint_ids: None,
    //     }))
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Output(dap::OutputEvent {
    //         category: None,
    //         output: "First line".to_string(),
    //         data: None,
    //         variables_reference: None,
    //         source: None,
    //         line: None,
    //         column: None,
    //         group: None,
    //         location_reference: None,
    //     }))
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Output(dap::OutputEvent {
    //         category: Some(dap::OutputEventCategory::Stdout),
    //         output: "First group".to_string(),
    //         data: None,
    //         variables_reference: None,
    //         source: None,
    //         line: None,
    //         column: None,
    //         group: Some(dap::OutputEventGroup::Start),
    //         location_reference: None,
    //     }))
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Output(dap::OutputEvent {
    //         category: Some(dap::OutputEventCategory::Stdout),
    //         output: "First item in group 1".to_string(),
    //         data: None,
    //         variables_reference: None,
    //         source: None,
    //         line: None,
    //         column: None,
    //         group: None,
    //         location_reference: None,
    //     }))
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Output(dap::OutputEvent {
    //         category: Some(dap::OutputEventCategory::Stdout),
    //         output: "Second item in group 1".to_string(),
    //         data: None,
    //         variables_reference: None,
    //         source: None,
    //         line: None,
    //         column: None,
    //         group: None,
    //         location_reference: None,
    //     }))
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Output(dap::OutputEvent {
    //         category: Some(dap::OutputEventCategory::Stdout),
    //         output: "Second group".to_string(),
    //         data: None,
    //         variables_reference: None,
    //         source: None,
    //         line: None,
    //         column: None,
    //         group: Some(dap::OutputEventGroup::Start),
    //         location_reference: None,
    //     }))
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Output(dap::OutputEvent {
    //         category: Some(dap::OutputEventCategory::Stdout),
    //         output: "First item in group 2".to_string(),
    //         data: None,
    //         variables_reference: None,
    //         source: None,
    //         line: None,
    //         column: None,
    //         group: None,
    //         location_reference: None,
    //     }))
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Output(dap::OutputEvent {
    //         category: Some(dap::OutputEventCategory::Stdout),
    //         output: "Second item in group 2".to_string(),
    //         data: None,
    //         variables_reference: None,
    //         source: None,
    //         line: None,
    //         column: None,
    //         group: None,
    //         location_reference: None,
    //     }))
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Output(dap::OutputEvent {
    //         category: Some(dap::OutputEventCategory::Stdout),
    //         output: "End group 2".to_string(),
    //         data: None,
    //         variables_reference: None,
    //         source: None,
    //         line: None,
    //         column: None,
    //         group: Some(dap::OutputEventGroup::End),
    //         location_reference: None,
    //     }))
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Output(dap::OutputEvent {
    //         category: Some(dap::OutputEventCategory::Stdout),
    //         output: "Third group".to_string(),
    //         data: None,
    //         variables_reference: None,
    //         source: None,
    //         line: None,
    //         column: None,
    //         group: Some(dap::OutputEventGroup::StartCollapsed),
    //         location_reference: None,
    //     }))
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Output(dap::OutputEvent {
    //         category: Some(dap::OutputEventCategory::Stdout),
    //         output: "First item in group 3".to_string(),
    //         data: None,
    //         variables_reference: None,
    //         source: None,
    //         line: None,
    //         column: None,
    //         group: None,
    //         location_reference: None,
    //     }))
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Output(dap::OutputEvent {
    //         category: Some(dap::OutputEventCategory::Stdout),
    //         output: "Second item in group 3".to_string(),
    //         data: None,
    //         variables_reference: None,
    //         source: None,
    //         line: None,
    //         column: None,
    //         group: None,
    //         location_reference: None,
    //     }))
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Output(dap::OutputEvent {
    //         category: Some(dap::OutputEventCategory::Stdout),
    //         output: "End group 3".to_string(),
    //         data: None,
    //         variables_reference: None,
    //         source: None,
    //         line: None,
    //         column: None,
    //         group: Some(dap::OutputEventGroup::End),
    //         location_reference: None,
    //     }))
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Output(dap::OutputEvent {
    //         category: Some(dap::OutputEventCategory::Stdout),
    //         output: "Third item in group 1".to_string(),
    //         data: None,
    //         variables_reference: None,
    //         source: None,
    //         line: None,
    //         column: None,
    //         group: None,
    //         location_reference: None,
    //     }))
    //     .await;

    // client
    //     .fake_event(dap::messages::Events::Output(dap::OutputEvent {
    //         category: Some(dap::OutputEventCategory::Stdout),
    //         output: "Second item".to_string(),
    //         data: None,
    //         variables_reference: None,
    //         source: None,
    //         line: None,
    //         column: None,
    //         group: Some(dap::OutputEventGroup::End),
    //         location_reference: None,
    //     }))
    //     .await;

    // host_cx.run_until_parked();
    // remote_cx.run_until_parked();

    // active_session(remote_workspace, remote_cx).update(remote_cx, |session_item, cx| {
    //     session_item
    //         .mode()
    //         .as_running()
    //         .unwrap()
    //         .read(cx)
    //         .console()
    //         .update(cx, |console, cx| {
    //             console.editor().update(cx, |editor, cx| {
    //                 pretty_assertions::assert_eq!(
    //                     "
    //                     <First line
    //                     First group
    //                         First item in group 1
    //                         Second item in group 1
    //                         Second group
    //                             First item in group 2
    //                             Second item in group 2
    //                         End group 2
    //                     ⋯    End group 3
    //                         Third item in group 1
    //                     Second item
    //                 "
    //                     .unindent(),
    //                     editor.display_text(cx)
    //                 );
    //             })
    //         });
    // });

    // let shutdown_client = host_project.update(host_cx, |project, cx| {
    //     project.dap_store().update(cx, |dap_store, cx| {
    //         dap_store.shutdown_session(&session.read(cx).session_id(), cx)
    //     })
    // });

    // shutdown_client.await.unwrap();
}
