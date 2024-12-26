use call::ActiveCall;
use dap::requests::{Disconnect, Initialize, Launch, StackTrace};
use debugger_ui::debugger_panel::DebugPanel;
use gpui::{TestAppContext, View, VisualTestContext};
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
