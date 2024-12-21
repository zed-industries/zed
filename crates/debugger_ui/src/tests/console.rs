use crate::*;
use dap::requests::{Disconnect, Evaluate, Initialize, Launch, StackTrace};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use project::{FakeFs, Project};
use std::sync::{
    atomic::{AtomicBool, Ordering},
    Arc,
};
use tests::{add_debugger_panel, init_test};

#[gpui::test]
async fn test_evaluate_expression(executor: BackgroundExecutor, cx: &mut TestAppContext) {
    init_test(cx);

    let was_called = Arc::new(AtomicBool::new(false));
    let was_called_clone = was_called.clone();

    let fs = FakeFs::new(executor.clone());

    let project = Project::test(fs, [], cx).await;
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

    client
        .on_request::<Evaluate, _>(move |_, args| {
            was_called_clone.store(true, Ordering::SeqCst);

            assert_eq!("print_r($variable, true);", args.expression);
            assert_eq!(Some(0), args.frame_id);
            assert_eq!(Some(dap::EvaluateArgumentsContext::Variables), args.context);

            Ok(dap::EvaluateResponse {
                result: "['key' => 'value']".into(),
                type_: None,
                presentation_hint: None,
                variables_reference: 0,
                named_variables: None,
                indexed_variables: None,
                memory_reference: None,
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

    cx.run_until_parked();

    workspace
        .update(cx, |workspace, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_debug_panel_item = debug_panel
                .update(cx, |this, cx| this.active_debug_panel_item(cx))
                .unwrap();

            active_debug_panel_item.update(cx, |item, cx| {
                item.console().update(cx, |console, cx| {
                    console.query_bar().update(cx, |query_bar, cx| {
                        query_bar.set_text("print_r($variable, true);", cx);
                    });

                    console.evaluate(&menu::Confirm, cx);
                });
            });
        })
        .unwrap();

    cx.run_until_parked();

    workspace
        .update(cx, |workspace, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let active_debug_panel_item = debug_panel
                .update(cx, |this, cx| this.active_debug_panel_item(cx))
                .unwrap();

            assert_eq!(
                "",
                active_debug_panel_item
                    .read(cx)
                    .console()
                    .read(cx)
                    .query_bar()
                    .read(cx)
                    .text(cx)
                    .as_str()
            );

            assert_eq!(
                "['key' => 'value']\n",
                active_debug_panel_item
                    .read(cx)
                    .console()
                    .read(cx)
                    .editor()
                    .read(cx)
                    .text(cx)
                    .as_str()
            );
        })
        .unwrap();

    assert!(
        was_called.load(std::sync::atomic::Ordering::SeqCst),
        "Expected evaluate request to be called"
    );

    let shutdown_client = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_client(&client.id(), cx)
        })
    });

    shutdown_client.await.unwrap();
}
