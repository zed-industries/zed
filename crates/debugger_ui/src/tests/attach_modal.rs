use crate::{attach_modal::Candidate, tests::start_debug_session_with, *};
use attach_modal::AttachModal;
use dap::{FakeAdapter, adapters::DebugTaskDefinition};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use menu::Confirm;
use project::{FakeFs, Project};
use serde_json::json;
use task::{AttachRequest, TcpArgumentsTemplate};
use tests::{init_test, init_test_workspace};
use util::path;

#[gpui::test]
async fn test_direct_attach_to_process(executor: BackgroundExecutor, cx: &mut TestAppContext) {
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

    let session = start_debug_session_with(
        &workspace,
        cx,
        DebugTaskDefinition {
            adapter: "fake-adapter".into(),
            request: dap::DebugRequest::Attach(AttachRequest {
                process_id: Some(10),
            }),
            label: "label".into(),
            initialize_args: None,
            tcp_connection: None,
            stop_on_entry: None,
        },
        |client| {
            client.on_request::<dap::requests::Attach, _>(move |_, args| {
                let raw = &args.raw;
                assert_eq!(raw["request"], "attach");
                assert_eq!(raw["process_id"], 10);

                Ok(())
            });
        },
    )
    .unwrap();

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
        path!("/project"),
        json!({
            "main.rs": "First line\nSecond line\nThird line\nFourth line",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);
    // Set up handlers for sessions spawned via modal.
    let _initialize_subscription =
        project::debugger::test::intercept_debug_sessions(cx, |client| {
            client.on_request::<dap::requests::Attach, _>(move |_, args| {
                let raw = &args.raw;
                assert_eq!(raw["request"], "attach");
                assert_eq!(raw["process_id"], 1);

                Ok(())
            });
        });
    let attach_modal = workspace
        .update(cx, |workspace, window, cx| {
            let workspace_handle = cx.weak_entity();
            workspace.toggle_modal(window, cx, |window, cx| {
                AttachModal::with_processes(
                    workspace_handle,
                    DebugTaskDefinition {
                        adapter: FakeAdapter::ADAPTER_NAME.into(),

                        request: dap::DebugRequest::Attach(AttachRequest::default()),
                        label: "attach example".into(),
                        initialize_args: None,
                        tcp_connection: Some(TcpArgumentsTemplate::default()),
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
                            name: "real-binary-1".into(),
                            command: vec![],
                        },
                        Candidate {
                            pid: 1,
                            name: "fake-binary-2".into(),
                            command: vec![],
                        },
                    ]
                    .into_iter()
                    .collect(),
                    true,
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
        .update(cx, |_, window, cx| {
            let names =
                attach_modal.update(cx, |modal, cx| attach_modal::_process_names(&modal, cx));
            // Initially all processes are visible.
            assert_eq!(3, names.len());
            attach_modal.update(cx, |this, cx| {
                this.picker.update(cx, |this, cx| {
                    this.set_query("fakb", window, cx);
                })
            })
        })
        .unwrap();
    cx.run_until_parked();
    // assert we got the expected processes
    workspace
        .update(cx, |_, _, cx| {
            let names =
                attach_modal.update(cx, |modal, cx| attach_modal::_process_names(&modal, cx));
            // Initially all processes are visible.
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
}
