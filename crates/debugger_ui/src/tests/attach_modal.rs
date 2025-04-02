use crate::{attach_modal::Candidate, *};
use attach_modal::AttachModal;
use dap::{FakeAdapter, client::SessionId};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use menu::Confirm;
use project::{FakeFs, Project};
use serde_json::json;
use task::{AttachConfig, DebugTaskDefinition, TCPHost};
use tests::{init_test, init_test_workspace};

#[gpui::test]
async fn test_direct_attach_to_process(executor: BackgroundExecutor, cx: &mut TestAppContext) {
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
        project.fake_debug_session(
            dap::DebugRequestType::Attach(AttachConfig {
                process_id: Some(10),
            }),
            None,
            false,
            cx,
        )
    });

    let session = task.await.unwrap();

    cx.run_until_parked();

    // assert we didn't show the attach modal
    workspace
        .update(cx, |workspace, _window, cx| {
            assert!(workspace.active_modal::<AttachModal>(cx).is_none());
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
async fn test_show_attach_modal_and_select_process(
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

    let attach_modal = workspace
        .update(cx, |workspace, window, cx| {
            workspace.toggle_modal(window, cx, |window, cx| {
                AttachModal::with_processes(
                    project.clone(),
                    DebugTaskDefinition {
                        adapter: FakeAdapter::ADAPTER_NAME.into(),
                        request: dap::DebugRequestType::Attach(AttachConfig::default()),
                        label: "attach example".into(),
                        initialize_args: None,
                        tcp_connection: Some(TCPHost::default()),
                        locator: None,
                        stop_on_entry: None,
                    },
                    vec![
                        Candidate {
                            pid: 0,
                            name: "fake-binary-1".into(),
                            command: vec![],
                        },
                        Candidate {
                            pid: 3,
                            name: "non-fake-binary-1".into(),
                            command: vec![],
                        },
                        Candidate {
                            pid: 1,
                            name: "fake-binary-2".into(),
                            command: vec![],
                        },
                    ],
                    window,
                    cx,
                )
            });

            workspace.active_modal::<AttachModal>(cx).unwrap()
        })
        .unwrap();

    cx.run_until_parked();

    // assert we got the expected processes
    workspace
        .update(cx, |_, _, cx| {
            let names =
                attach_modal.update(cx, |modal, cx| attach_modal::_process_names(&modal, cx));

            // we filtered out all processes that are not starting with `fake-binary`
            assert_eq!(2, names.len());
        })
        .unwrap();

    // select the only existing process
    cx.dispatch_action(Confirm);

    cx.run_until_parked();

    // assert attach modal was dismissed
    workspace
        .update(cx, |workspace, _window, cx| {
            assert!(workspace.active_modal::<AttachModal>(cx).is_none());
        })
        .unwrap();

    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            let session = dap_store.session_by_id(SessionId(0)).unwrap();

            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();
}
