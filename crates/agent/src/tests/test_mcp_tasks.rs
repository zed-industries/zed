use super::*;
use context_server::types::{
    self, CallToolParams, CallToolResponse, CreateTaskResult, ServerCapabilities,
    ServerTaskRequestsCapabilities, ServerTaskToolsCapabilities, ServerTasksCapabilities, Task,
    TaskStatus, TaskSupport, TasksGetParams, TasksResultParams, Tool, ToolExecution,
    ToolResponseContent, MODEL_IMMEDIATE_RESPONSE_KEY,
};
use pretty_assertions::assert_eq;
use std::collections::VecDeque;
use std::sync::Mutex as StdMutex;

/// Shared state that tests use to control what the fake task server returns.
struct TaskServerState {
    /// Sequence of Task states that TasksGet will return, popped from front.
    task_get_responses: VecDeque<Task>,
    /// The value that TasksResult will return.
    task_result: Option<serde_json::Value>,
}

/// Sets up a fake MCP context server that advertises task capabilities.
///
/// Returns:
/// - A receiver for CallToolAsTask invocations (like `setup_context_server`'s CallTool receiver)
/// - An `Arc<StdMutex<TaskServerState>>` for controlling TasksGet / TasksResult responses
/// - An `Arc<FakeTransport>` for injecting notifications from the test body
fn setup_task_context_server(
    name: &'static str,
    tools: Vec<Tool>,
    context_server_store: &Entity<ContextServerStore>,
    cx: &mut TestAppContext,
) -> (
    mpsc::UnboundedReceiver<(CallToolParams, oneshot::Sender<CreateTaskResult>)>,
    Arc<StdMutex<TaskServerState>>,
    Arc<context_server::test::FakeTransport>,
) {
    cx.update(|cx| {
        let mut settings = ProjectSettings::get_global(cx).clone();
        settings.context_servers.insert(
            name.into(),
            project::project_settings::ContextServerSettings::Stdio {
                enabled: true,
                remote: false,
                command: ContextServerCommand {
                    path: "somebinary".into(),
                    args: Vec::new(),
                    env: None,
                    timeout: None,
                },
            },
        );
        ProjectSettings::override_global(settings, cx);
    });

    let state = Arc::new(StdMutex::new(TaskServerState {
        task_get_responses: VecDeque::new(),
        task_result: None,
    }));

    let (tool_calls_tx, tool_calls_rx) = mpsc::unbounded();

    let tasks_get_state = state.clone();
    let tasks_result_state = state.clone();

    let fake_transport = context_server::test::create_fake_transport(name, cx.executor())
        .on_request::<types::requests::Initialize, _>(move |_params| async move {
            types::InitializeResponse {
                protocol_version: types::ProtocolVersion(
                    types::LATEST_PROTOCOL_VERSION.to_string(),
                ),
                server_info: types::Implementation {
                    name: name.into(),
                    version: "1.0.0".to_string(),
                },
                capabilities: ServerCapabilities {
                    tools: Some(types::ToolsCapabilities {
                        list_changed: Some(true),
                    }),
                    tasks: Some(ServerTasksCapabilities {
                        list: None,
                        cancel: None,
                        requests: Some(ServerTaskRequestsCapabilities {
                            tools: Some(ServerTaskToolsCapabilities {
                                call: Some(serde_json::json!({})),
                            }),
                        }),
                    }),
                    ..Default::default()
                },
                meta: None,
            }
        })
        .on_request::<types::requests::ListTools, _>(move |_params| {
            let tools = tools.clone();
            async move {
                types::ListToolsResponse {
                    tools,
                    next_cursor: None,
                    meta: None,
                }
            }
        })
        .on_request::<types::requests::CallToolAsTask, _>(move |params| {
            let tool_calls_tx = tool_calls_tx.clone();
            async move {
                let (response_tx, response_rx) = oneshot::channel();
                tool_calls_tx
                    .unbounded_send((params, response_tx))
                    .unwrap();
                response_rx.await.unwrap()
            }
        })
        .on_request::<types::requests::TasksGet, _>(move |params: TasksGetParams| {
            let state = tasks_get_state.clone();
            async move {
                let mut guard = state.lock().unwrap();
                guard.task_get_responses.pop_front().unwrap_or_else(|| {
                    panic!(
                        "TasksGet called for task '{}' but no responses were queued",
                        params.task_id
                    )
                })
            }
        })
        .on_request::<types::requests::TasksResult, _>(move |params: TasksResultParams| {
            let state = tasks_result_state.clone();
            async move {
                let guard = state.lock().unwrap();
                guard.task_result.clone().unwrap_or_else(|| {
                    panic!(
                        "TasksResult called for task '{}' but no result was set",
                        params.task_id
                    )
                })
            }
        });

    let transport_arc = Arc::new(fake_transport);

    context_server_store.update(cx, |store, cx| {
        store.start_server(
            Arc::new(ContextServer::new(
                ContextServerId(name.into()),
                transport_arc.clone() as Arc<dyn context_server::transport::Transport>,
            )),
            cx,
        );
    });
    cx.run_until_parked();

    (tool_calls_rx, state, transport_arc)
}

/// Sets up a fake MCP context server that advertises task capabilities but
/// whose tool has `task_support: Forbidden`. This should cause the agent to
/// use the normal `CallTool` path instead of `CallToolAsTask`.
fn setup_non_task_context_server(
    name: &'static str,
    tools: Vec<Tool>,
    context_server_store: &Entity<ContextServerStore>,
    cx: &mut TestAppContext,
) -> mpsc::UnboundedReceiver<(CallToolParams, oneshot::Sender<CallToolResponse>)> {
    cx.update(|cx| {
        let mut settings = ProjectSettings::get_global(cx).clone();
        settings.context_servers.insert(
            name.into(),
            project::project_settings::ContextServerSettings::Stdio {
                enabled: true,
                remote: false,
                command: ContextServerCommand {
                    path: "somebinary".into(),
                    args: Vec::new(),
                    env: None,
                    timeout: None,
                },
            },
        );
        ProjectSettings::override_global(settings, cx);
    });

    let (tool_calls_tx, tool_calls_rx) = mpsc::unbounded();

    let fake_transport = context_server::test::create_fake_transport(name, cx.executor())
        .on_request::<types::requests::Initialize, _>(move |_params| async move {
            types::InitializeResponse {
                protocol_version: types::ProtocolVersion(
                    types::LATEST_PROTOCOL_VERSION.to_string(),
                ),
                server_info: types::Implementation {
                    name: name.into(),
                    version: "1.0.0".to_string(),
                },
                capabilities: ServerCapabilities {
                    tools: Some(types::ToolsCapabilities {
                        list_changed: Some(true),
                    }),
                    tasks: Some(ServerTasksCapabilities {
                        list: None,
                        cancel: None,
                        requests: Some(ServerTaskRequestsCapabilities {
                            tools: Some(ServerTaskToolsCapabilities {
                                call: Some(serde_json::json!({})),
                            }),
                        }),
                    }),
                    ..Default::default()
                },
                meta: None,
            }
        })
        .on_request::<types::requests::ListTools, _>(move |_params| {
            let tools = tools.clone();
            async move {
                types::ListToolsResponse {
                    tools,
                    next_cursor: None,
                    meta: None,
                }
            }
        })
        .on_request::<types::requests::CallTool, _>(move |params| {
            let tool_calls_tx = tool_calls_tx.clone();
            async move {
                let (response_tx, response_rx) = oneshot::channel();
                tool_calls_tx
                    .unbounded_send((params, response_tx))
                    .unwrap();
                response_rx.await.unwrap()
            }
        });

    context_server_store.update(cx, |store, cx| {
        store.start_server(
            Arc::new(ContextServer::new(
                ContextServerId(name.into()),
                Arc::new(fake_transport),
            )),
            cx,
        );
    });
    cx.run_until_parked();

    tool_calls_rx
}

fn make_task(
    task_id: &str,
    status: TaskStatus,
    message: Option<&str>,
    poll_interval: Option<u64>,
) -> Task {
    Task {
        task_id: task_id.to_string(),
        status,
        status_message: message.map(|s| s.to_string()),
        created_at: "2025-01-01T00:00:00Z".to_string(),
        last_updated_at: "2025-01-01T00:00:01Z".to_string(),
        ttl: Some(300_000),
        poll_interval,
    }
}

fn make_tool(name: &str, task_support: TaskSupport) -> Tool {
    Tool {
        name: name.into(),
        description: Some(format!("A tool named {name}")),
        input_schema: json!({"type": "object", "properties": {"text": {"type": "string"}}}),
        output_schema: None,
        annotations: None,
        execution: Some(ToolExecution {
            task_support: Some(task_support),
        }),
    }
}

/// Extract the text from a `LanguageModelToolResultContent`.
fn tool_result_text(content: &language_model::LanguageModelToolResultContent) -> &str {
    match content {
        language_model::LanguageModelToolResultContent::Text(text) => text.as_ref(),
        other => panic!("expected Text tool result content, got: {other:?}"),
    }
}

/// Configures settings so MCP tools are auto-allowed and the test profile
/// enables all context servers.
async fn configure_test_profile(
    fs: &Arc<FakeFs>,
    thread: &Entity<Thread>,
    cx: &mut TestAppContext,
) {
    fs.insert_file(
        paths::settings_file(),
        json!({
            "agent": {
                "tool_permissions": { "default": "allow" },
                "profiles": {
                    "test": {
                        "name": "Test Profile",
                        "enable_all_context_servers": true,
                        "tools": {}
                    },
                }
            }
        })
        .to_string()
        .into_bytes(),
    )
    .await;
    cx.run_until_parked();
    thread.update(cx, |thread, cx| {
        thread.set_profile(AgentProfileId("test".into()), cx)
    });
}

/// Simulate the model issuing a tool call, end the stream, and park.
fn model_call_tool(fake_model: &FakeLanguageModel, tool_name: &str, cx: &mut TestAppContext) {
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "tool_1".into(),
            name: tool_name.into(),
            raw_input: json!({"text": "hello"}).to_string(),
            input: json!({"text": "hello"}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();
}

/// Wait for the model to receive a new pending completion request.
///
/// The MCP task polling loop uses `smol::Timer::after` which requires
/// real wall-clock time to fire. The test executor's `run_until_parked()`
/// is synchronous and does not park for real time, so we cannot drive
/// smol timers with it. Instead, we await a short smol timer in the
/// test itself — the `block()` method in the test scheduler parks the
/// thread for real time, during which the tool's smol timers fire and
/// the polling loop progresses.
async fn wait_for_model_completion(
    fake_model: &FakeLanguageModel,
    cx: &mut TestAppContext,
) {
    for _ in 0..50 {
        // Yield real wall-clock time so smol timers in spawned tasks can fire.
        smol::Timer::after(Duration::from_millis(50)).await;
        cx.run_until_parked();
        if !fake_model.pending_completions().is_empty() {
            return;
        }
    }
    panic!("Timed out waiting for model to receive a new completion request");
}

#[gpui::test]
async fn test_mcp_task_basic_lifecycle(cx: &mut TestAppContext) {
    let ThreadTest {
        model,
        thread,
        context_server_store,
        fs,
        ..
    } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    configure_test_profile(&fs, &thread, cx).await;

    let (mut tool_calls, state, _transport) = setup_task_context_server(
        "task_server",
        vec![make_tool("slow_echo", TaskSupport::Required)],
        &context_server_store,
        cx,
    );

    // Queue up TasksGet responses: first "working", then "completed".
    {
        let mut guard = state.lock().unwrap();
        guard.task_get_responses.push_back(make_task(
            "task-1",
            TaskStatus::Working,
            Some("Still processing..."),
            Some(10),
        ));
        guard.task_get_responses.push_back(make_task(
            "task-1",
            TaskStatus::Completed,
            Some("Done!"),
            Some(10),
        ));
        guard.task_result = Some(
            serde_json::to_value(CallToolResponse {
                content: vec![ToolResponseContent::Text {
                    text: "echoed: hello".into(),
                    annotations: None,
                }],
                is_error: None,
                meta: None,
                structured_content: None,
            })
            .unwrap(),
        );
    }

    // Send a user message so the model gets a completion request.
    let events = thread.update(cx, |thread, cx| {
        thread
            .send(UserMessageId::new(), ["call the tool"], cx)
            .unwrap()
    });
    cx.run_until_parked();

    // Verify the tool is available.
    let completion = fake_model.pending_completions().pop().unwrap();
    assert!(
        tool_names_for_completion(&completion).contains(&"slow_echo".to_string()),
        "slow_echo tool should be available"
    );

    // Simulate the model calling the tool.
    model_call_tool(&fake_model, "slow_echo", cx);

    // The server receives the CallToolAsTask request — respond with CreateTaskResult.
    let (call_params, response_tx) = tool_calls.next().await.unwrap();
    assert_eq!(call_params.name, "slow_echo");
    assert_eq!(call_params.arguments, Some(json!({"text": "hello"})));
    assert!(
        call_params.task.is_some(),
        "task params should be set for task-augmented calls"
    );

    response_tx
        .send(CreateTaskResult {
            task: make_task("task-1", TaskStatus::Working, Some("Starting..."), Some(10)),
            meta: None,
        })
        .unwrap();

    // Wait for the polling loop to complete and the model to get a new
    // completion request with the tool result. The polling loop uses
    // smol::Timer::after which needs real wall-clock time.
    wait_for_model_completion(&fake_model, cx).await;

    // The model should now have a new completion request with the tool result.
    let completion = fake_model.pending_completions().pop().unwrap();
    let tool_result = completion
        .messages
        .last()
        .unwrap()
        .content
        .iter()
        .find_map(|c| match c {
            MessageContent::ToolResult(r) => Some(r),
            _ => None,
        })
        .expect("expected a tool result in the completion");

    assert_eq!(tool_result.tool_use_id.to_string(), "tool_1");
    assert!(!tool_result.is_error, "tool result should not be an error");
    let result_text = tool_result_text(&tool_result.content);
    assert!(
        result_text.contains("echoed: hello"),
        "tool result content should contain the echoed text, got: {result_text}",
    );

    // Finish the model turn.
    fake_model.send_last_completion_stream_text_chunk("All done!");
    fake_model.end_last_completion_stream();
    events.collect::<Vec<_>>().await;
}

#[gpui::test]
async fn test_mcp_task_failure(cx: &mut TestAppContext) {
    let ThreadTest {
        model,
        thread,
        context_server_store,
        fs,
        ..
    } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    configure_test_profile(&fs, &thread, cx).await;

    let (mut tool_calls, state, _transport) = setup_task_context_server(
        "fail_server",
        vec![make_tool("failing_tool", TaskSupport::Required)],
        &context_server_store,
        cx,
    );

    // Queue up TasksGet to return "failed" immediately.
    {
        let mut guard = state.lock().unwrap();
        guard.task_get_responses.push_back(make_task(
            "task-fail",
            TaskStatus::Failed,
            Some("Something went wrong on the server"),
            Some(10),
        ));
        guard.task_result = Some(
            serde_json::to_value(CallToolResponse {
                content: vec![ToolResponseContent::Text {
                    text: "Error: Something went wrong on the server".into(),
                    annotations: None,
                }],
                is_error: Some(true),
                meta: None,
                structured_content: None,
            })
            .unwrap(),
        );
    }

    let events = thread.update(cx, |thread, cx| {
        thread
            .send(UserMessageId::new(), ["call the failing tool"], cx)
            .unwrap()
    });
    cx.run_until_parked();

    model_call_tool(&fake_model, "failing_tool", cx);

    let (_call_params, response_tx) = tool_calls.next().await.unwrap();
    response_tx
        .send(CreateTaskResult {
            task: make_task(
                "task-fail",
                TaskStatus::Working,
                Some("Starting..."),
                Some(10),
            ),
            meta: None,
        })
        .unwrap();

    // Wait for the polling loop to detect the failure and return a result.
    wait_for_model_completion(&fake_model, cx).await;

    // The model should receive a tool result indicating failure.
    let completion = fake_model.pending_completions().pop().unwrap();
    let tool_result = completion
        .messages
        .last()
        .unwrap()
        .content
        .iter()
        .find_map(|c| match c {
            MessageContent::ToolResult(r) => Some(r),
            _ => None,
        })
        .expect("expected a tool result in the completion");

    assert_eq!(tool_result.tool_use_id.to_string(), "tool_1");
    assert!(
        tool_result.is_error,
        "tool result should be marked as an error"
    );
    let result_text = tool_result_text(&tool_result.content);
    assert!(
        result_text.contains("Something went wrong"),
        "tool result should contain the error message, got: {result_text}",
    );

    // Finish the model turn.
    fake_model.send_last_completion_stream_text_chunk("The tool failed.");
    fake_model.end_last_completion_stream();
    events.collect::<Vec<_>>().await;
}

#[gpui::test]
async fn test_mcp_task_progress_notifications(cx: &mut TestAppContext) {
    let ThreadTest {
        model,
        thread,
        context_server_store,
        fs,
        ..
    } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    configure_test_profile(&fs, &thread, cx).await;

    let (mut tool_calls, state, transport) = setup_task_context_server(
        "progress_server",
        vec![make_tool("progress_tool", TaskSupport::Required)],
        &context_server_store,
        cx,
    );

    // Queue up TasksGet responses. The task completes on first poll.
    {
        let mut guard = state.lock().unwrap();
        guard.task_get_responses.push_back(make_task(
            "task-progress",
            TaskStatus::Completed,
            Some("Finished processing"),
            Some(10),
        ));
        guard.task_result = Some(
            serde_json::to_value(CallToolResponse {
                content: vec![ToolResponseContent::Text {
                    text: "progress result".into(),
                    annotations: None,
                }],
                is_error: None,
                meta: None,
                structured_content: None,
            })
            .unwrap(),
        );
    }

    let mut events = thread.update(cx, |thread, cx| {
        thread
            .send(UserMessageId::new(), ["call the progress tool"], cx)
            .unwrap()
    });
    cx.run_until_parked();

    model_call_tool(&fake_model, "progress_tool", cx);

    // The server receives the CallToolAsTask request. Before responding,
    // extract the progress token from the params so we can send a matching
    // notifications/progress notification.
    let (call_params, response_tx) = tool_calls.next().await.unwrap();
    assert_eq!(call_params.name, "progress_tool");

    // Extract the progress token from _meta.progressToken.
    let progress_token = call_params
        .meta
        .as_ref()
        .and_then(|m| m.get("progressToken"))
        .cloned()
        .expect("CallToolParams should contain a progressToken in _meta");

    // Send a progress notification via the transport before returning
    // the CreateTaskResult.
    transport
        .send_notification(
            "notifications/progress",
            json!({
                "progressToken": progress_token,
                "progress": 0.5,
                "message": "Halfway there"
            }),
        )
        .expect("sending progress notification should succeed");
    cx.run_until_parked();

    // Now respond with the CreateTaskResult.
    response_tx
        .send(CreateTaskResult {
            task: make_task(
                "task-progress",
                TaskStatus::Working,
                Some("Processing..."),
                Some(10),
            ),
            meta: None,
        })
        .unwrap();

    // Wait for the polling loop to complete.
    wait_for_model_completion(&fake_model, cx).await;

    // Verify the model received the tool result.
    let completion = fake_model.pending_completions().pop().unwrap();
    let tool_result = completion
        .messages
        .last()
        .unwrap()
        .content
        .iter()
        .find_map(|c| match c {
            MessageContent::ToolResult(r) => Some(r),
            _ => None,
        })
        .expect("expected a tool result in the completion");

    let result_text = tool_result_text(&tool_result.content);
    assert!(
        result_text.contains("progress result"),
        "tool result should contain the expected output, got: {result_text}",
    );

    // Drain events and look for a ToolCallUpdate that contains one of
    // our progress/status messages (either "Halfway there" from the
    // notifications/progress notification, "Processing..." from the
    // CreateTaskResult, or "Finished processing" from the final TasksGet).
    let mut found_status_update = false;
    while let Some(Some(event)) = events.next().now_or_never() {
        if let Ok(ThreadEvent::ToolCallUpdate(acp_thread::ToolCallUpdate::UpdateFields(update))) =
            &event
        {
            if let Some(title) = &update.fields.title {
                if title == "Halfway there"
                    || title == "Processing..."
                    || title == "Finished processing"
                {
                    found_status_update = true;
                }
            }
        }
    }
    assert!(
        found_status_update,
        "Should have received a ToolCallUpdate with a progress or status message as the title"
    );

    fake_model.send_last_completion_stream_text_chunk("Done with progress!");
    fake_model.end_last_completion_stream();
}

#[gpui::test]
async fn test_mcp_task_non_task_tool_uses_normal_path(cx: &mut TestAppContext) {
    let ThreadTest {
        model,
        thread,
        context_server_store,
        fs,
        ..
    } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    configure_test_profile(&fs, &thread, cx).await;

    // The server has task capabilities, but this specific tool has Forbidden.
    let mut mcp_tool_calls = setup_non_task_context_server(
        "non_task_server",
        vec![make_tool("normal_tool", TaskSupport::Forbidden)],
        &context_server_store,
        cx,
    );

    let events = thread.update(cx, |thread, cx| {
        thread
            .send(UserMessageId::new(), ["call the normal tool"], cx)
            .unwrap()
    });
    cx.run_until_parked();

    let completion = fake_model.pending_completions().pop().unwrap();
    assert!(
        tool_names_for_completion(&completion).contains(&"normal_tool".to_string()),
        "normal_tool should be available"
    );

    model_call_tool(&fake_model, "normal_tool", cx);

    // The server should receive a regular CallTool (not CallToolAsTask).
    let (call_params, response_tx) = mcp_tool_calls.next().await.unwrap();
    assert_eq!(call_params.name, "normal_tool");
    assert_eq!(call_params.arguments, Some(json!({"text": "hello"})));
    // The task field should NOT be set for non-task calls.
    assert!(
        call_params.task.is_none(),
        "task params should not be set for Forbidden task_support tools"
    );

    response_tx
        .send(CallToolResponse {
            content: vec![ToolResponseContent::Text {
                text: "normal result".into(),
                annotations: None,
            }],
            is_error: None,
            meta: None,
            structured_content: None,
        })
        .unwrap();
    cx.run_until_parked();

    // The model should receive the tool result directly.
    let completion = fake_model.pending_completions().pop().unwrap();
    let tool_result = completion
        .messages
        .last()
        .unwrap()
        .content
        .iter()
        .find_map(|c| match c {
            MessageContent::ToolResult(r) => Some(r),
            _ => None,
        })
        .expect("expected a tool result in the completion");

    assert_eq!(tool_result.tool_use_id.to_string(), "tool_1");
    assert!(!tool_result.is_error);
    let result_text = tool_result_text(&tool_result.content);
    assert!(
        result_text.contains("normal result"),
        "tool result should contain the normal output, got: {result_text}",
    );

    // Finish the model turn.
    fake_model.send_last_completion_stream_text_chunk("Got it!");
    fake_model.end_last_completion_stream();
    events.collect::<Vec<_>>().await;
}

#[gpui::test]
async fn test_mcp_task_model_immediate_response(cx: &mut TestAppContext) {
    let ThreadTest {
        model,
        thread,
        context_server_store,
        fs,
        ..
    } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    configure_test_profile(&fs, &thread, cx).await;

    let (mut tool_calls, state, _transport) = setup_task_context_server(
        "immediate_server",
        vec![make_tool("background_tool", TaskSupport::Required)],
        &context_server_store,
        cx,
    );

    // Queue up TasksGet responses: the task stays "working" for one poll,
    // then completes. The model should NOT wait for these — it should
    // continue immediately with the provisional response.
    {
        let mut guard = state.lock().unwrap();
        guard.task_get_responses.push_back(make_task(
            "task-imm-1",
            TaskStatus::Working,
            Some("Still crunching..."),
            Some(10),
        ));
        guard.task_get_responses.push_back(make_task(
            "task-imm-1",
            TaskStatus::Completed,
            Some("All done in background"),
            Some(10),
        ));
        guard.task_result = Some(
            serde_json::to_value(CallToolResponse {
                content: vec![ToolResponseContent::Text {
                    text: "final background result".into(),
                    annotations: None,
                }],
                is_error: None,
                meta: None,
                structured_content: None,
            })
            .unwrap(),
        );
    }

    // Send a user message.
    let events = thread.update(cx, |thread, cx| {
        thread
            .send(UserMessageId::new(), ["call the background tool"], cx)
            .unwrap()
    });
    cx.run_until_parked();

    let completion = fake_model.pending_completions().pop().unwrap();
    assert!(
        tool_names_for_completion(&completion).contains(&"background_tool".to_string()),
        "background_tool should be available"
    );

    // Simulate the model calling the tool.
    model_call_tool(&fake_model, "background_tool", cx);

    // The server receives CallToolAsTask — respond with a CreateTaskResult
    // that includes a model-immediate-response in _meta.
    let (call_params, response_tx) = tool_calls.next().await.unwrap();
    assert_eq!(call_params.name, "background_tool");
    assert!(call_params.task.is_some());

    let provisional_response = CallToolResponse {
        content: vec![ToolResponseContent::Text {
            text: "provisional: task started, check back later".into(),
            annotations: None,
        }],
        is_error: None,
        meta: None,
        structured_content: None,
    };

    let mut meta_map = collections::HashMap::default();
    meta_map.insert(
        MODEL_IMMEDIATE_RESPONSE_KEY.to_string(),
        serde_json::to_value(&provisional_response).unwrap(),
    );

    response_tx
        .send(CreateTaskResult {
            task: make_task(
                "task-imm-1",
                TaskStatus::Working,
                Some("Starting background work..."),
                Some(10),
            ),
            meta: Some(meta_map),
        })
        .unwrap();

    // The model should receive the provisional result WITHOUT waiting for
    // the background task to complete. Because the provisional response is
    // returned immediately (no polling needed), run_until_parked is enough.
    cx.run_until_parked();

    // The model should now have a new completion request with the provisional
    // tool result.
    assert!(
        !fake_model.pending_completions().is_empty(),
        "model should have a pending completion with the provisional tool result"
    );
    let completion = fake_model.pending_completions().pop().unwrap();
    let tool_result = completion
        .messages
        .last()
        .unwrap()
        .content
        .iter()
        .find_map(|c| match c {
            MessageContent::ToolResult(r) => Some(r),
            _ => None,
        })
        .expect("expected a tool result in the completion");

    assert_eq!(tool_result.tool_use_id.to_string(), "tool_1");
    assert!(
        !tool_result.is_error,
        "provisional tool result should not be an error"
    );
    assert!(
        tool_result.is_provisional,
        "provisional tool result should have is_provisional=true"
    );
    let result_text = tool_result_text(&tool_result.content);
    assert!(
        result_text.contains("provisional: task started"),
        "model should receive the provisional response, got: {result_text}",
    );

    // Finish the model turn — the model continues immediately.
    fake_model.send_last_completion_stream_text_chunk("Got provisional result, moving on!");
    fake_model.end_last_completion_stream();
    // Don't use events.collect() here — the background poller holds a clone
    // of the ThreadEventStream sender, so the receiver never sees the stream
    // close. Instead, drop the receiver and let run_until_parked() drive the
    // turn task to completion.
    drop(events);
    cx.run_until_parked();

    // The background poller is still running (detached). Give it real
    // wall-clock time so its smol timers fire and it polls to completion.
    for _i in 0..50 {
        smol::Timer::after(Duration::from_millis(50)).await;
        cx.run_until_parked();
        // Check if the background poller has drained all TasksGet responses.
        let remaining = state.lock().unwrap().task_get_responses.len();
        if remaining == 0 {
            break;
        }
    }

    // All queued TasksGet responses should have been consumed by the
    // background poller.
    let remaining = state.lock().unwrap().task_get_responses.len();
    assert_eq!(
        remaining, 0,
        "background poller should have consumed all TasksGet responses, {} remain",
        remaining,
    );
}