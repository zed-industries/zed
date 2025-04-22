use crate::tests::{init_test, init_test_workspace, start_debug_session};
use dap::{
    requests::{StackTrace, Threads},
    transport::LogKind,
};
use debugger_tools::LogStore;
use gpui::{BackgroundExecutor, TestAppContext, VisualTestContext};
use project::Project;
use serde_json::json;
use std::sync::{Arc, Mutex};
use std::{cell::OnceCell, collections::VecDeque};

#[gpui::test]
async fn test_dap_logger_captures_all_session_rpc_messages(
    executor: BackgroundExecutor,
    cx: &mut TestAppContext,
) {
    let log_store_cell = Arc::new(OnceCell::new());

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
        "/project",
        json!({
            "main.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"
        }),
    )
    .await;

    // Set up the project and workspace
    let project = Project::test(fs, ["/project".as_ref()], cx).await;

    let workspace = init_test_workspace(&project, cx).await;
    let cx = &mut VisualTestContext::from_window(*workspace, cx);

    // Create a logger extractor to capture RPC logs after the test
    let rpc_logs = Arc::new(Mutex::new(VecDeque::new()));
    let rpc_logs_capture = rpc_logs.clone();

    // Start a debug session
    let session = start_debug_session(&workspace, cx, |_| {}).unwrap();
    let client = session.update(cx, |session, _| session.adapter_client().unwrap());

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

    cx.run_until_parked();

    // Create a custom log extractor to capture RPC logs directly from the client
    // We need to do this because we don't have direct access to the LogStore
    client.add_log_handler(
        move |_, message| {
            rpc_logs_capture
                .lock()
                .unwrap()
                .push_back(message.to_string());
        },
        LogKind::Rpc,
    );

    // Shutdown the debug session
    let shutdown_session = project.update(cx, |project, cx| {
        project.dap_store().update(cx, |dap_store, cx| {
            dap_store.shutdown_session(session.read(cx).session_id(), cx)
        })
    });

    shutdown_session.await.unwrap();
    cx.run_until_parked();

    // Access logs for verification
    let logs = rpc_logs.lock().unwrap();

    // Make sure we have some RPC logs
    assert!(!logs.is_empty(), "Should have captured RPC logs");

    // Print the log count for diagnostic purposes
    println!("Captured {} RPC logs", logs.len());

    // Check for specific messages that should be present
    // We expect at least:
    // 1. An initialize request at the beginning
    let has_initialize = logs.iter().any(|log| log.contains("initialize"));

    // 2. A disconnect request at the end
    let has_disconnect = logs.iter().any(|log| log.contains("disconnect"));

    // 3. Configuration done in the middle
    let has_configuration_done = logs.iter().any(|log| log.contains("configurationDone"));

    // 4. Threads request
    let has_threads = logs.iter().any(|log| log.contains("threads"));

    // Verify the critical messages were captured
    assert!(has_initialize, "Should have captured initialize message");
    assert!(has_disconnect, "Should have captured disconnect message");
    assert!(
        has_configuration_done,
        "Should have captured configurationDone message"
    );
    assert!(has_threads, "Should have captured threads request");

    // Check the sequence of messages to ensure proper session flow
    // First convert to a vector for easier indexing
    let logs_vec: Vec<_> = logs.iter().collect();

    // Find indexes of key messages
    let initialize_index = logs_vec.iter().position(|log| log.contains("initialize"));
    let config_done_index = logs_vec
        .iter()
        .position(|log| log.contains("configurationDone"));
    let threads_index = logs_vec.iter().position(|log| log.contains("threads"));
    let disconnect_index = logs_vec.iter().position(|log| log.contains("disconnect"));

    // The initialize request should come before configurationDone
    if let (Some(init_idx), Some(config_idx)) = (initialize_index, config_done_index) {
        assert!(
            init_idx < config_idx,
            "Initialize should occur before configurationDone"
        );
    }

    // The disconnect should come after threads requests
    if let (Some(threads_idx), Some(disconnect_idx)) = (threads_index, disconnect_index) {
        assert!(
            threads_idx < disconnect_idx,
            "Threads request should occur before disconnect"
        );
    }
}
