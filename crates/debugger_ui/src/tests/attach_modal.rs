use crate::{
    attach_modal::{Candidate, ModalIntent},
    tests::start_debug_session_with,
    *,
};
use attach_modal::AttachModal;
use dap::{FakeAdapter, adapters::DebugTaskDefinition};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use menu::Confirm;
use project::{FakeFs, Project};
use serde_json::json;
use task::AttachRequest;
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

    let _session = start_debug_session_with(
        &workspace,
        cx,
        DebugTaskDefinition {
            adapter: "fake-adapter".into(),
            label: "label".into(),
            config: json!({
               "request": "attach",
              "process_id": 10,
            }),
            tcp_connection: None,
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
                    ModalIntent::AttachToProcess(task::ZedDebugConfig {
                        adapter: FakeAdapter::ADAPTER_NAME.into(),
                        request: dap::DebugRequest::Attach(AttachRequest::default()),
                        label: "attach example".into(),
                        stop_on_entry: None,
                    }),
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
            let names = attach_modal.update(cx, |modal, cx| attach_modal::process_names(modal, cx));
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
            let names = attach_modal.update(cx, |modal, cx| attach_modal::process_names(modal, cx));
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

#[gpui::test]
async fn test_attach_with_pick_pid_variable(executor: BackgroundExecutor, cx: &mut TestAppContext) {
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

    let _initialize_subscription =
        project::debugger::test::intercept_debug_sessions(cx, |client| {
            client.on_request::<dap::requests::Attach, _>(move |_, args| {
                let raw = &args.raw;
                assert_eq!(raw["request"], "attach");
                assert_eq!(
                    raw["process_id"], "42",
                    "verify process id has been replaced"
                );

                Ok(())
            });
        });

    let pick_pid_placeholder = task::VariableName::PickProcessId.template_value();
    workspace
        .update(cx, |workspace, window, cx| {
            workspace.start_debug_session(
                DebugTaskDefinition {
                    adapter: FakeAdapter::ADAPTER_NAME.into(),
                    label: "attach with picker".into(),
                    config: json!({
                        "request": "attach",
                        "process_id": pick_pid_placeholder,
                    }),
                    tcp_connection: None,
                }
                .to_scenario(),
                task::TaskContext::default(),
                None,
                None,
                window,
                cx,
            )
        })
        .unwrap();

    cx.run_until_parked();

    let attach_modal = workspace
        .update(cx, |workspace, _window, cx| {
            workspace.active_modal::<AttachModal>(cx)
        })
        .unwrap();

    assert!(
        attach_modal.is_some(),
        "Attach modal should open when config contains ZED_PICK_PID"
    );

    let attach_modal = attach_modal.unwrap();

    workspace
        .update(cx, |_, window, cx| {
            attach_modal.update(cx, |modal, cx| {
                attach_modal::set_candidates(
                    modal,
                    vec![
                        Candidate {
                            pid: 10,
                            name: "process-1".into(),
                            command: vec![],
                        },
                        Candidate {
                            pid: 42,
                            name: "target-process".into(),
                            command: vec![],
                        },
                        Candidate {
                            pid: 99,
                            name: "process-3".into(),
                            command: vec![],
                        },
                    ]
                    .into_iter()
                    .collect(),
                    window,
                    cx,
                )
            })
        })
        .unwrap();

    cx.run_until_parked();

    workspace
        .update(cx, |_, window, cx| {
            attach_modal.update(cx, |modal, cx| {
                modal.picker.update(cx, |picker, cx| {
                    picker.set_query("target", window, cx);
                })
            })
        })
        .unwrap();

    cx.run_until_parked();

    workspace
        .update(cx, |_, _, cx| {
            let names = attach_modal.update(cx, |modal, cx| attach_modal::process_names(modal, cx));
            assert_eq!(names.len(), 1);
            assert_eq!(names[0], " 42 target-process");
        })
        .unwrap();

    cx.dispatch_action(Confirm);
    cx.run_until_parked();

    workspace
        .update(cx, |workspace, _window, cx| {
            assert!(
                workspace.active_modal::<AttachModal>(cx).is_none(),
                "Attach modal should be dismissed after selection"
            );
        })
        .unwrap();
}
