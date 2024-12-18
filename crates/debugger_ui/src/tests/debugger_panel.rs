use crate::*;
use dap::requests::{Disconnect, Initialize, Launch, StackTrace};
use gpui::{BackgroundExecutor, Model, TestAppContext, VisualTestContext, WindowHandle};
use project::{FakeFs, Project};
use serde_json::json;
use settings::SettingsStore;
use unindent::Unindent as _;
use workspace::{dock::Panel, Workspace};

pub fn init_test(cx: &mut gpui::TestAppContext) {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::try_init().ok();
    }

    cx.update(|cx| {
        let settings = SettingsStore::test(cx);
        cx.set_global(settings);
        theme::init(theme::LoadThemes::JustBase, cx);
        command_palette_hooks::init(cx);
        language::init(cx);
        workspace::init_settings(cx);
        Project::init_settings(cx);
        crate::init(cx);
        editor::init(cx);
    });
}

async fn add_debugger_panel(
    project: &Model<Project>,
    cx: &mut TestAppContext,
) -> WindowHandle<Workspace> {
    let window = cx.add_window(|cx| Workspace::test_new(project.clone(), cx));

    let debugger_panel = window
        .update(cx, |_, cx| cx.spawn(DebugPanel::load))
        .unwrap()
        .await
        .expect("Failed to load debug panel");

    window
        .update(cx, |workspace, cx| {
            workspace.add_panel(debugger_panel, cx);
        })
        .unwrap();
    window
}

#[gpui::test]
async fn test_basic_show_debug_panel(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());

    let file_contents = r#"
        // print goodbye
        fn main() {
            println!("goodbye world");
        }
    "#
    .unindent();

    fs.insert_tree(
        "/dir",
        json!({
           "src": {
               "main.rs": file_contents,
           }
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
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

    let shutdown_client = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_client(&client.id(), cx)
        })
    });

    // If we don't end session client will still be awaiting to recv messages
    // from fake transport that will never be transmitted, thus resulting in
    // a "panic: parked with nothing to run"
    shutdown_client.await.unwrap();

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

    let file_contents = r#"
        // print goodbye
        fn main() {
            println!("goodbye world");
        }
    "#
    .unindent();

    fs.insert_tree(
        "/dir",
        json!({
           "src": {
               "main.rs": file_contents,
           }
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
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

    let shutdown_client = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_client(&client.id(), cx)
        })
    });

    // If we don't end session client will still be awaiting to recv messages
    // from fake transport that will never be transmitted, thus resulting in
    // a "panic: parked with nothing to run"
    shutdown_client.await.unwrap();

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

    let file_contents = r#"
        // print goodbye
        fn main() {
            println!("goodbye world");
        }
    "#
    .unindent();

    fs.insert_tree(
        "/dir",
        json!({
           "src": {
               "main.rs": file_contents,
           }
        }),
    )
    .await;

    let project = Project::test(fs, ["/dir".as_ref()], cx).await;
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

    let shutdown_client = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_client(&client.id(), cx)
        })
    });

    // If we don't end session client will still be awaiting to recv messages
    // from fake transport that will never be transmitted, thus resulting in
    // a "panic: parked with nothing to run"
    shutdown_client.await.unwrap();

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
