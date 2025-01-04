use crate::*;
use attach_modal::AttachModal;
use dap::requests::{Attach, Disconnect, Initialize};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use menu::{Cancel, Confirm};
use project::{FakeFs, Project};
use serde_json::json;
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use task::AttachConfig;
use tests::{init_test, init_test_workspace};

#[gpui::test]
async fn test_direct_attach_to_process(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let send_attach_request = Arc::new(AtomicBool::new(false));

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
                    request: task::DebugRequestType::Attach(AttachConfig {
                        process_id: Some(10),
                    }),
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

    client
        .on_request::<Attach, _>({
            let send_attach_request = send_attach_request.clone();
            move |_, args| {
                send_attach_request.store(true, Ordering::SeqCst);

                assert_eq!(json!({"request": "attach", "process_id": 10}), args.raw);

                Ok(())
            }
        })
        .await;

    client.on_request::<Disconnect, _>(move |_, _| Ok(())).await;

    cx.run_until_parked();

    assert!(
        send_attach_request.load(std::sync::atomic::Ordering::SeqCst),
        "Expected to send attach request, because we passed in the processId"
    );

    // assert we didn't show the attach modal
    workspace
        .update(cx, |workspace, cx| {
            assert!(workspace.active_modal::<AttachModal>(cx).is_none());
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
async fn test_show_attach_modal_and_select_process(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let send_attach_request = Arc::new(AtomicBool::new(false));

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
                    request: task::DebugRequestType::Attach(AttachConfig { process_id: None }),
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

    client
        .on_request::<Attach, _>({
            let send_attach_request = send_attach_request.clone();
            move |_, args| {
                send_attach_request.store(true, Ordering::SeqCst);

                assert_eq!(
                    json!({
                        "request": "attach",
                        // note we filtered out all processes in FakeAdapter::attach_processes,
                        // that is not equal to the current process id
                        "process_id": std::process::id(),
                    }),
                    args.raw
                );

                Ok(())
            }
        })
        .await;

    client.on_request::<Disconnect, _>(move |_, _| Ok(())).await;

    cx.run_until_parked();

    // assert we show the attach modal
    workspace
        .update(cx, |workspace, cx| {
            let attach_modal = workspace.active_modal::<AttachModal>(cx).unwrap();

            let names = attach_modal.update(cx, |modal, cx| attach_modal::procss_names(&modal, cx));

            // we filtered out all processes that are not the current process(zed itself)
            assert_eq!(1, names.len());
        })
        .unwrap();

    // select the only existing process
    cx.dispatch_action(Confirm);

    cx.run_until_parked();

    // assert attach modal was dismissed
    workspace
        .update(cx, |workspace, cx| {
            assert!(workspace.active_modal::<AttachModal>(cx).is_none());
        })
        .unwrap();

    assert!(
        send_attach_request.load(std::sync::atomic::Ordering::SeqCst),
        "Expected to send attach request, because we passed in the processId"
    );

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(&session.read(cx).id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}

#[gpui::test]
async fn test_shutdown_session_when_modal_is_dismissed(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let send_attach_request = Arc::new(AtomicBool::new(false));

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
                    request: task::DebugRequestType::Attach(AttachConfig { process_id: None }),
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

    client
        .on_request::<Attach, _>({
            let send_attach_request = send_attach_request.clone();
            move |_, _| {
                send_attach_request.store(true, Ordering::SeqCst);

                Ok(())
            }
        })
        .await;

    client.on_request::<Disconnect, _>(move |_, _| Ok(())).await;

    cx.run_until_parked();

    // assert we show the attach modal
    workspace
        .update(cx, |workspace, cx| {
            let attach_modal = workspace.active_modal::<AttachModal>(cx).unwrap();

            let names = attach_modal.update(cx, |modal, cx| attach_modal::procss_names(&modal, cx));

            // we filtered out all processes that are not the current process(zed itself)
            assert_eq!(1, names.len());
        })
        .unwrap();

    // close the modal
    cx.dispatch_action(Cancel);

    cx.run_until_parked();

    // assert attach modal was dismissed
    workspace
        .update(cx, |workspace, cx| {
            assert!(workspace.active_modal::<AttachModal>(cx).is_none());
        })
        .unwrap();

    assert!(
        !send_attach_request.load(std::sync::atomic::Ordering::SeqCst),
        "Didn't expected to send attach request, because we closed the modal"
    );

    // assert debug session is shutdown
    project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            assert!(dap_store.session_by_id(&session.read(cx).id()).is_none())
        });
    });
}
