use std::iter::zip;

use crate::{
    debugger_panel::DebugPanel,
    persistence::SerializedPaneLayout,
    tests::{init_test, init_test_workspace, start_debug_session},
};
use dap::{StoppedEvent, StoppedEventReason, messages::Events};
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use project::{FakeFs, Project};
use serde_json::json;
use util::path;
use workspace::{Panel, dock::DockPosition};

#[gpui::test]
async fn test_invert_axis_on_panel_position_change(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    init_test(cx);

    let fs = FakeFs::new(executor.clone());
    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "fn main() {\n    println!(\"Hello, world!\");\n}",
        }),
    )
    .await;

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;
    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    // Start a debug session
    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    // Setup thread response
    client.on_request::<dap::requests::Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse { threads: vec![] })
    });

    cx.run_until_parked();

    client
        .fake_event(Events::Stopped(StoppedEvent {
            reason: StoppedEventReason::Pause,
            description: None,
            thread_id: Some(1),
            preserve_focus_hint: None,
            text: None,
            all_threads_stopped: None,
            hit_breakpoint_ids: None,
        }))
        .await;

    cx.run_until_parked();

    let (debug_panel, dock_position) = workspace
        .update(cx, |workspace, window, cx| {
            let debug_panel = workspace.panel::<DebugPanel>(cx).unwrap();
            let dock_position = debug_panel.read(cx).position(window, cx);
            (debug_panel, dock_position)
        })
        .unwrap();

    assert_eq!(
        dock_position,
        DockPosition::Bottom,
        "Default dock position should be bottom for debug panel"
    );

    let pre_serialized_layout = debug_panel
        .read_with(cx, |panel, cx| {
            panel
                .active_session()
                .unwrap()
                .read(cx)
                .running_state()
                .read(cx)
                .serialized_layout(cx)
        })
        .panes;

    let post_serialized_layout = debug_panel
        .update_in(cx, |panel, window, cx| {
            panel.set_position(DockPosition::Right, window, cx);

            panel
                .active_session()
                .unwrap()
                .read(cx)
                .running_state()
                .read(cx)
                .serialized_layout(cx)
        })
        .panes;

    let pre_panes = pre_serialized_layout.in_order();
    let post_panes = post_serialized_layout.in_order();

    assert_eq!(pre_panes.len(), post_panes.len());

    for (pre, post) in zip(pre_panes, post_panes) {
        match (pre, post) {
            (
                SerializedPaneLayout::Group {
                    axis: pre_axis,
                    flexes: pre_flexes,
                    children: _,
                },
                SerializedPaneLayout::Group {
                    axis: post_axis,
                    flexes: post_flexes,
                    children: _,
                },
            ) => {
                assert_ne!(pre_axis, post_axis);
                assert_eq!(pre_flexes, post_flexes);
            }
            (SerializedPaneLayout::Pane(pre_pane), SerializedPaneLayout::Pane(post_pane)) => {
                assert_eq!(pre_pane.children, post_pane.children);
                assert_eq!(pre_pane.active_item, post_pane.active_item);
            }
            _ => {
                panic!("Variants don't match")
            }
        }
    }
}
