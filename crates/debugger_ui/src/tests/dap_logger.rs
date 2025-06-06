use crate::tests::{init_test, init_test_workspace, start_debug_session};
use dap::requests::{StackTrace, Threads};
use debugger_tools::LogStore;
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use project::Project;
use serde_json::json;
use std::cell::OnceCell;
use util::path;

#[gpui::test]
async fn test_dap_logger_captures_all_session_rpc_messages(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    let log_store_cell = std::rc::Rc::new(OnceCell::new());

    cx.update(|cx| {
        let log_store_cell = log_store_cell.clone();
        cx.observe_new::<LogStore>(move |_, _, cx| {
            log_store_cell.set(cx.entity()).unwrap();
        })
        .detach();
        debugger_tools::init(cx);
    });
    init_test(cx);

    let log_store = log_store_cell.get().unwrap().clone();

    // Create a filesystem with a simple project
    let fs = project::FakeFs::new(executor.clone());
    fs.insert_tree(
        path!("/project"),
        json!({
            "main.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"
        }),
    )
    .await;

    assert!(
        log_store.read_with(cx, |log_store, _| log_store
            .contained_session_ids()
            .is_empty()),
        "log_store shouldn't contain any session IDs before any sessions were created"
    );

    let project = Project::test(fs, [path!("/project").as_ref()], cx).await;

    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    // Start a debug session
    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let session_id = session.read_with(cx, |session, _| session.session_id());
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

    assert_eq!(
        log_store.read_with(cx, |log_store, _| log_store.contained_session_ids().len()),
        1,
    );

    assert!(
        log_store.read_with(cx, |log_store, _| log_store
            .contained_session_ids()
            .contains(&session_id)),
        "log_store should contain the session IDs of the started session"
    );

    assert!(
        !log_store.read_with(cx, |log_store, _| log_store
            .rpc_messages_for_session_id(session_id)
            .is_empty()),
        "We should have the initialization sequence in the log store"
    );

    // Set up basic responses for common requests
    client.on_request::<Threads, _>(move |_, _| {
        Ok(dap::ThreadsResponse {
            threads: vec![dap::Thread {
                id: 1,
                name: "Thread 1".into(),
            }],
        })
    });

    client.on_request::<StackTrace, _>(move |_, _| {
        Ok(dap::StackTraceResponse {
            stack_frames: Vec::default(),
            total_frames: None,
        })
    });

    // Run until all pending tasks are executed
    cx.run_until_parked();

    // Simulate a stopped event to generate more DAP messages
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
}
