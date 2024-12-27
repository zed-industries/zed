use crate::*;
use attach_modal::AttachModal;
use dap::requests::{Attach, Disconnect, Initialize};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
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
