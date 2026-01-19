use super::*;
use acp_thread::{
    AgentConnection, AgentModelGroupName, AgentModelList, PermissionOptions, UserMessageId,
};
use agent_client_protocol::{self as acp};
use agent_settings::AgentProfileId;
use anyhow::Result;
use client::{Client, UserStore};
use cloud_llm_client::CompletionIntent;
use collections::IndexMap;
use context_server::{ContextServer, ContextServerCommand, ContextServerId};
use feature_flags::FeatureFlagAppExt as _;
use fs::{FakeFs, Fs};
use futures::{
    FutureExt as _, StreamExt,
    channel::{
        mpsc::{self, UnboundedReceiver},
        oneshot,
    },
    future::{Fuse, Shared},
};
use gpui::{
    App, AppContext, AsyncApp, Entity, Task, TestAppContext, UpdateGlobal,
    http_client::FakeHttpClient,
};
use indoc::indoc;
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelId,
    LanguageModelProviderName, LanguageModelRegistry, LanguageModelRequest,
    LanguageModelRequestMessage, LanguageModelToolResult, LanguageModelToolSchemaFormat,
    LanguageModelToolUse, MessageContent, Role, StopReason, fake_provider::FakeLanguageModel,
};
use pretty_assertions::assert_eq;
use project::{
    Project, context_server_store::ContextServerStore, project_settings::ProjectSettings,
};
use prompt_store::ProjectContext;
use reqwest_client::ReqwestClient;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use settings::{Settings, SettingsStore};
use std::{
    path::Path,
    pin::Pin,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::Duration,
};
use util::path;

mod test_tools;
use test_tools::*;

fn init_test(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let settings_store = SettingsStore::test(cx);
        cx.set_global(settings_store);
    });
}

struct FakeTerminalHandle {
    killed: Arc<AtomicBool>,
    stopped_by_user: Arc<AtomicBool>,
    exit_sender: std::cell::RefCell<Option<futures::channel::oneshot::Sender<()>>>,
    wait_for_exit: Shared<Task<acp::TerminalExitStatus>>,
    output: acp::TerminalOutputResponse,
    id: acp::TerminalId,
}

impl FakeTerminalHandle {
    fn new_never_exits(cx: &mut App) -> Self {
        let killed = Arc::new(AtomicBool::new(false));
        let stopped_by_user = Arc::new(AtomicBool::new(false));

        let (exit_sender, exit_receiver) = futures::channel::oneshot::channel();

        let wait_for_exit = cx
            .spawn(async move |_cx| {
                // Wait for the exit signal (sent when kill() is called)
                let _ = exit_receiver.await;
                acp::TerminalExitStatus::new()
            })
            .shared();

        Self {
            killed,
            stopped_by_user,
            exit_sender: std::cell::RefCell::new(Some(exit_sender)),
            wait_for_exit,
            output: acp::TerminalOutputResponse::new("partial output".to_string(), false),
            id: acp::TerminalId::new("fake_terminal".to_string()),
        }
    }

    fn new_with_immediate_exit(cx: &mut App, exit_code: u32) -> Self {
        let killed = Arc::new(AtomicBool::new(false));
        let stopped_by_user = Arc::new(AtomicBool::new(false));
        let (exit_sender, _exit_receiver) = futures::channel::oneshot::channel();

        let wait_for_exit = cx
            .spawn(async move |_cx| acp::TerminalExitStatus::new().exit_code(exit_code))
            .shared();

        Self {
            killed,
            stopped_by_user,
            exit_sender: std::cell::RefCell::new(Some(exit_sender)),
            wait_for_exit,
            output: acp::TerminalOutputResponse::new("command output".to_string(), false),
            id: acp::TerminalId::new("fake_terminal".to_string()),
        }
    }

    fn was_killed(&self) -> bool {
        self.killed.load(Ordering::SeqCst)
    }

    fn set_stopped_by_user(&self, stopped: bool) {
        self.stopped_by_user.store(stopped, Ordering::SeqCst);
    }

    fn signal_exit(&self) {
        if let Some(sender) = self.exit_sender.borrow_mut().take() {
            let _ = sender.send(());
        }
    }
}

impl crate::TerminalHandle for FakeTerminalHandle {
    fn id(&self, _cx: &AsyncApp) -> Result<acp::TerminalId> {
        Ok(self.id.clone())
    }

    fn current_output(&self, _cx: &AsyncApp) -> Result<acp::TerminalOutputResponse> {
        Ok(self.output.clone())
    }

    fn wait_for_exit(&self, _cx: &AsyncApp) -> Result<Shared<Task<acp::TerminalExitStatus>>> {
        Ok(self.wait_for_exit.clone())
    }

    fn kill(&self, _cx: &AsyncApp) -> Result<()> {
        self.killed.store(true, Ordering::SeqCst);
        self.signal_exit();
        Ok(())
    }

    fn was_stopped_by_user(&self, _cx: &AsyncApp) -> Result<bool> {
        Ok(self.stopped_by_user.load(Ordering::SeqCst))
    }
}

struct FakeThreadEnvironment {
    handle: Rc<FakeTerminalHandle>,
}

impl crate::ThreadEnvironment for FakeThreadEnvironment {
    fn create_terminal(
        &self,
        _command: String,
        _cwd: Option<std::path::PathBuf>,
        _output_byte_limit: Option<u64>,
        _cx: &mut AsyncApp,
    ) -> Task<Result<Rc<dyn crate::TerminalHandle>>> {
        Task::ready(Ok(self.handle.clone() as Rc<dyn crate::TerminalHandle>))
    }
}

/// Environment that creates multiple independent terminal handles for testing concurrent terminals.
struct MultiTerminalEnvironment {
    handles: std::cell::RefCell<Vec<Rc<FakeTerminalHandle>>>,
}

impl MultiTerminalEnvironment {
    fn new() -> Self {
        Self {
            handles: std::cell::RefCell::new(Vec::new()),
        }
    }

    fn handles(&self) -> Vec<Rc<FakeTerminalHandle>> {
        self.handles.borrow().clone()
    }
}

impl crate::ThreadEnvironment for MultiTerminalEnvironment {
    fn create_terminal(
        &self,
        _command: String,
        _cwd: Option<std::path::PathBuf>,
        _output_byte_limit: Option<u64>,
        cx: &mut AsyncApp,
    ) -> Task<Result<Rc<dyn crate::TerminalHandle>>> {
        let handle = Rc::new(cx.update(|cx| FakeTerminalHandle::new_never_exits(cx)));
        self.handles.borrow_mut().push(handle.clone());
        Task::ready(Ok(handle as Rc<dyn crate::TerminalHandle>))
    }
}

fn always_allow_tools(cx: &mut TestAppContext) {
    cx.update(|cx| {
        let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
        settings.always_allow_tool_actions = true;
        agent_settings::AgentSettings::override_global(settings, cx);
    });
}

#[gpui::test]
async fn test_echo(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let events = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Testing: Reply with 'Hello'"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    fake_model.send_last_completion_stream_text_chunk("Hello");
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();

    let events = events.collect().await;
    thread.update(cx, |thread, _cx| {
        assert_eq!(
            thread.last_message().unwrap().to_markdown(),
            indoc! {"
                ## Assistant

                Hello
            "}
        )
    });
    assert_eq!(stop_events(events), vec![acp::StopReason::EndTurn]);
}

#[gpui::test]
async fn test_terminal_tool_timeout_kills_handle(cx: &mut TestAppContext) {
    init_test(cx);
    always_allow_tools(cx);

    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, [], cx).await;

    let handle = Rc::new(cx.update(|cx| FakeTerminalHandle::new_never_exits(cx)));
    let environment = Rc::new(FakeThreadEnvironment {
        handle: handle.clone(),
    });

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(crate::TerminalTool::new(project, environment));
    let (event_stream, mut rx) = crate::ToolCallEventStream::test();

    let task = cx.update(|cx| {
        tool.run(
            crate::TerminalToolInput {
                command: "sleep 1000".to_string(),
                cd: ".".to_string(),
                timeout_ms: Some(5),
            },
            event_stream,
            cx,
        )
    });

    let update = rx.expect_update_fields().await;
    assert!(
        update.content.iter().any(|blocks| {
            blocks
                .iter()
                .any(|c| matches!(c, acp::ToolCallContent::Terminal(_)))
        }),
        "expected tool call update to include terminal content"
    );

    let mut task_future: Pin<Box<Fuse<Task<Result<String>>>>> = Box::pin(task.fuse());

    let deadline = std::time::Instant::now() + Duration::from_millis(500);
    loop {
        if let Some(result) = task_future.as_mut().now_or_never() {
            let result = result.expect("terminal tool task should complete");

            assert!(
                handle.was_killed(),
                "expected terminal handle to be killed on timeout"
            );
            assert!(
                result.contains("partial output"),
                "expected result to include terminal output, got: {result}"
            );
            return;
        }

        if std::time::Instant::now() >= deadline {
            panic!("timed out waiting for terminal tool task to complete");
        }

        cx.run_until_parked();
        cx.background_executor.timer(Duration::from_millis(1)).await;
    }
}

#[gpui::test]
#[ignore]
async fn test_terminal_tool_without_timeout_does_not_kill_handle(cx: &mut TestAppContext) {
    init_test(cx);
    always_allow_tools(cx);

    let fs = FakeFs::new(cx.executor());
    let project = Project::test(fs, [], cx).await;

    let handle = Rc::new(cx.update(|cx| FakeTerminalHandle::new_never_exits(cx)));
    let environment = Rc::new(FakeThreadEnvironment {
        handle: handle.clone(),
    });

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(crate::TerminalTool::new(project, environment));
    let (event_stream, mut rx) = crate::ToolCallEventStream::test();

    let _task = cx.update(|cx| {
        tool.run(
            crate::TerminalToolInput {
                command: "sleep 1000".to_string(),
                cd: ".".to_string(),
                timeout_ms: None,
            },
            event_stream,
            cx,
        )
    });

    let update = rx.expect_update_fields().await;
    assert!(
        update.content.iter().any(|blocks| {
            blocks
                .iter()
                .any(|c| matches!(c, acp::ToolCallContent::Terminal(_)))
        }),
        "expected tool call update to include terminal content"
    );

    cx.background_executor
        .timer(Duration::from_millis(25))
        .await;

    assert!(
        !handle.was_killed(),
        "did not expect terminal handle to be killed without a timeout"
    );
}

#[gpui::test]
async fn test_thinking(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let events = thread
        .update(cx, |thread, cx| {
            thread.send(
                UserMessageId::new(),
                [indoc! {"
                    Testing:

                    Generate a thinking step where you just think the word 'Think',
                    and have your final answer be 'Hello'
                "}],
                cx,
            )
        })
        .unwrap();
    cx.run_until_parked();
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::Thinking {
        text: "Think".to_string(),
        signature: None,
    });
    fake_model.send_last_completion_stream_text_chunk("Hello");
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();

    let events = events.collect().await;
    thread.update(cx, |thread, _cx| {
        assert_eq!(
            thread.last_message().unwrap().to_markdown(),
            indoc! {"
                ## Assistant

                <think>Think</think>
                Hello
            "}
        )
    });
    assert_eq!(stop_events(events), vec![acp::StopReason::EndTurn]);
}

#[gpui::test]
async fn test_system_prompt(cx: &mut TestAppContext) {
    let ThreadTest {
        model,
        thread,
        project_context,
        ..
    } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    project_context.update(cx, |project_context, _cx| {
        project_context.shell = "test-shell".into()
    });
    thread.update(cx, |thread, _| thread.add_tool(EchoTool));
    thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["abc"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    let mut pending_completions = fake_model.pending_completions();
    assert_eq!(
        pending_completions.len(),
        1,
        "unexpected pending completions: {:?}",
        pending_completions
    );

    let pending_completion = pending_completions.pop().unwrap();
    assert_eq!(pending_completion.messages[0].role, Role::System);

    let system_message = &pending_completion.messages[0];
    let system_prompt = system_message.content[0].to_str().unwrap();
    assert!(
        system_prompt.contains("test-shell"),
        "unexpected system message: {:?}",
        system_message
    );
    assert!(
        system_prompt.contains("## Fixing Diagnostics"),
        "unexpected system message: {:?}",
        system_message
    );
}

#[gpui::test]
async fn test_system_prompt_without_tools(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["abc"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    let mut pending_completions = fake_model.pending_completions();
    assert_eq!(
        pending_completions.len(),
        1,
        "unexpected pending completions: {:?}",
        pending_completions
    );

    let pending_completion = pending_completions.pop().unwrap();
    assert_eq!(pending_completion.messages[0].role, Role::System);

    let system_message = &pending_completion.messages[0];
    let system_prompt = system_message.content[0].to_str().unwrap();
    assert!(
        !system_prompt.contains("## Tool Use"),
        "unexpected system message: {:?}",
        system_message
    );
    assert!(
        !system_prompt.contains("## Fixing Diagnostics"),
        "unexpected system message: {:?}",
        system_message
    );
}

#[gpui::test]
async fn test_prompt_caching(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    // Send initial user message and verify it's cached
    thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Message 1"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    let completion = fake_model.pending_completions().pop().unwrap();
    assert_eq!(
        completion.messages[1..],
        vec![LanguageModelRequestMessage {
            role: Role::User,
            content: vec!["Message 1".into()],
            cache: true,
            reasoning_details: None,
        }]
    );
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::Text(
        "Response to Message 1".into(),
    ));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // Send another user message and verify only the latest is cached
    thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Message 2"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    let completion = fake_model.pending_completions().pop().unwrap();
    assert_eq!(
        completion.messages[1..],
        vec![
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["Message 1".into()],
                cache: false,
                reasoning_details: None,
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec!["Response to Message 1".into()],
                cache: false,
                reasoning_details: None,
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["Message 2".into()],
                cache: true,
                reasoning_details: None,
            }
        ]
    );
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::Text(
        "Response to Message 2".into(),
    ));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // Simulate a tool call and verify that the latest tool result is cached
    thread.update(cx, |thread, _| thread.add_tool(EchoTool));
    thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Use the echo tool"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    let tool_use = LanguageModelToolUse {
        id: "tool_1".into(),
        name: EchoTool::name().into(),
        raw_input: json!({"text": "test"}).to_string(),
        input: json!({"text": "test"}),
        is_input_complete: true,
        thought_signature: None,
    };
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(tool_use.clone()));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let completion = fake_model.pending_completions().pop().unwrap();
    let tool_result = LanguageModelToolResult {
        tool_use_id: "tool_1".into(),
        tool_name: EchoTool::name().into(),
        is_error: false,
        content: "test".into(),
        output: Some("test".into()),
    };
    assert_eq!(
        completion.messages[1..],
        vec![
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["Message 1".into()],
                cache: false,
                reasoning_details: None,
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec!["Response to Message 1".into()],
                cache: false,
                reasoning_details: None,
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["Message 2".into()],
                cache: false,
                reasoning_details: None,
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec!["Response to Message 2".into()],
                cache: false,
                reasoning_details: None,
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["Use the echo tool".into()],
                cache: false,
                reasoning_details: None,
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![MessageContent::ToolUse(tool_use)],
                cache: false,
                reasoning_details: None,
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::ToolResult(tool_result)],
                cache: true,
                reasoning_details: None,
            }
        ]
    );
}

#[gpui::test]
#[cfg_attr(not(feature = "e2e"), ignore)]
async fn test_basic_tool_calls(cx: &mut TestAppContext) {
    let ThreadTest { thread, .. } = setup(cx, TestModel::Sonnet4).await;

    // Test a tool call that's likely to complete *before* streaming stops.
    let events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(EchoTool);
            thread.send(
                UserMessageId::new(),
                ["Now test the echo tool with 'Hello'. Does it work? Say 'Yes' or 'No'."],
                cx,
            )
        })
        .unwrap()
        .collect()
        .await;
    assert_eq!(stop_events(events), vec![acp::StopReason::EndTurn]);

    // Test a tool calls that's likely to complete *after* streaming stops.
    let events = thread
        .update(cx, |thread, cx| {
            thread.remove_tool(&EchoTool::name());
            thread.add_tool(DelayTool);
            thread.send(
                UserMessageId::new(),
                [
                    "Now call the delay tool with 200ms.",
                    "When the timer goes off, then you echo the output of the tool.",
                ],
                cx,
            )
        })
        .unwrap()
        .collect()
        .await;
    assert_eq!(stop_events(events), vec![acp::StopReason::EndTurn]);
    thread.update(cx, |thread, _cx| {
        assert!(
            thread
                .last_message()
                .unwrap()
                .as_agent_message()
                .unwrap()
                .content
                .iter()
                .any(|content| {
                    if let AgentMessageContent::Text(text) = content {
                        text.contains("Ding")
                    } else {
                        false
                    }
                }),
            "{}",
            thread.to_markdown()
        );
    });
}

#[gpui::test]
#[cfg_attr(not(feature = "e2e"), ignore)]
async fn test_streaming_tool_calls(cx: &mut TestAppContext) {
    let ThreadTest { thread, .. } = setup(cx, TestModel::Sonnet4).await;

    // Test a tool call that's likely to complete *before* streaming stops.
    let mut events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(WordListTool);
            thread.send(UserMessageId::new(), ["Test the word_list tool."], cx)
        })
        .unwrap();

    let mut saw_partial_tool_use = false;
    while let Some(event) = events.next().await {
        if let Ok(ThreadEvent::ToolCall(tool_call)) = event {
            thread.update(cx, |thread, _cx| {
                // Look for a tool use in the thread's last message
                let message = thread.last_message().unwrap();
                let agent_message = message.as_agent_message().unwrap();
                let last_content = agent_message.content.last().unwrap();
                if let AgentMessageContent::ToolUse(last_tool_use) = last_content {
                    assert_eq!(last_tool_use.name.as_ref(), "word_list");
                    if tool_call.status == acp::ToolCallStatus::Pending {
                        if !last_tool_use.is_input_complete
                            && last_tool_use.input.get("g").is_none()
                        {
                            saw_partial_tool_use = true;
                        }
                    } else {
                        last_tool_use
                            .input
                            .get("a")
                            .expect("'a' has streamed because input is now complete");
                        last_tool_use
                            .input
                            .get("g")
                            .expect("'g' has streamed because input is now complete");
                    }
                } else {
                    panic!("last content should be a tool use");
                }
            });
        }
    }

    assert!(
        saw_partial_tool_use,
        "should see at least one partially streamed tool use in the history"
    );
}

#[gpui::test]
async fn test_tool_authorization(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(ToolRequiringPermission);
            thread.send(UserMessageId::new(), ["abc"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "tool_id_1".into(),
            name: ToolRequiringPermission::name().into(),
            raw_input: "{}".into(),
            input: json!({}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "tool_id_2".into(),
            name: ToolRequiringPermission::name().into(),
            raw_input: "{}".into(),
            input: json!({}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.end_last_completion_stream();
    let tool_call_auth_1 = next_tool_call_authorization(&mut events).await;
    let tool_call_auth_2 = next_tool_call_authorization(&mut events).await;

    // Approve the first - send "allow" option_id (UI transforms "once" to "allow")
    tool_call_auth_1
        .response
        .send(acp::PermissionOptionId::new("allow"))
        .unwrap();
    cx.run_until_parked();

    // Reject the second - send "deny" option_id directly since Deny is now a button
    tool_call_auth_2
        .response
        .send(acp::PermissionOptionId::new("deny"))
        .unwrap();
    cx.run_until_parked();

    let completion = fake_model.pending_completions().pop().unwrap();
    let message = completion.messages.last().unwrap();
    assert_eq!(
        message.content,
        vec![
            language_model::MessageContent::ToolResult(LanguageModelToolResult {
                tool_use_id: tool_call_auth_1.tool_call.tool_call_id.0.to_string().into(),
                tool_name: ToolRequiringPermission::name().into(),
                is_error: false,
                content: "Allowed".into(),
                output: Some("Allowed".into())
            }),
            language_model::MessageContent::ToolResult(LanguageModelToolResult {
                tool_use_id: tool_call_auth_2.tool_call.tool_call_id.0.to_string().into(),
                tool_name: ToolRequiringPermission::name().into(),
                is_error: true,
                content: "Permission to run tool denied by user".into(),
                output: Some("Permission to run tool denied by user".into())
            })
        ]
    );

    // Simulate yet another tool call.
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "tool_id_3".into(),
            name: ToolRequiringPermission::name().into(),
            raw_input: "{}".into(),
            input: json!({}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.end_last_completion_stream();

    // Respond by always allowing tools - send transformed option_id
    // (UI transforms "always:tool_requiring_permission" to "always_allow:tool_requiring_permission")
    let tool_call_auth_3 = next_tool_call_authorization(&mut events).await;
    tool_call_auth_3
        .response
        .send(acp::PermissionOptionId::new(
            "always_allow:tool_requiring_permission",
        ))
        .unwrap();
    cx.run_until_parked();
    let completion = fake_model.pending_completions().pop().unwrap();
    let message = completion.messages.last().unwrap();
    assert_eq!(
        message.content,
        vec![language_model::MessageContent::ToolResult(
            LanguageModelToolResult {
                tool_use_id: tool_call_auth_3.tool_call.tool_call_id.0.to_string().into(),
                tool_name: ToolRequiringPermission::name().into(),
                is_error: false,
                content: "Allowed".into(),
                output: Some("Allowed".into())
            }
        )]
    );

    // Simulate a final tool call, ensuring we don't trigger authorization.
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "tool_id_4".into(),
            name: ToolRequiringPermission::name().into(),
            raw_input: "{}".into(),
            input: json!({}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();
    let completion = fake_model.pending_completions().pop().unwrap();
    let message = completion.messages.last().unwrap();
    assert_eq!(
        message.content,
        vec![language_model::MessageContent::ToolResult(
            LanguageModelToolResult {
                tool_use_id: "tool_id_4".into(),
                tool_name: ToolRequiringPermission::name().into(),
                is_error: false,
                content: "Allowed".into(),
                output: Some("Allowed".into())
            }
        )]
    );
}

#[gpui::test]
async fn test_tool_hallucination(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["abc"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "tool_id_1".into(),
            name: "nonexistent_tool".into(),
            raw_input: "{}".into(),
            input: json!({}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.end_last_completion_stream();

    let tool_call = expect_tool_call(&mut events).await;
    assert_eq!(tool_call.title, "nonexistent_tool");
    assert_eq!(tool_call.status, acp::ToolCallStatus::Pending);
    let update = expect_tool_call_update_fields(&mut events).await;
    assert_eq!(update.fields.status, Some(acp::ToolCallStatus::Failed));
}

async fn expect_tool_call(events: &mut UnboundedReceiver<Result<ThreadEvent>>) -> acp::ToolCall {
    let event = events
        .next()
        .await
        .expect("no tool call authorization event received")
        .unwrap();
    match event {
        ThreadEvent::ToolCall(tool_call) => tool_call,
        event => {
            panic!("Unexpected event {event:?}");
        }
    }
}

async fn expect_tool_call_update_fields(
    events: &mut UnboundedReceiver<Result<ThreadEvent>>,
) -> acp::ToolCallUpdate {
    let event = events
        .next()
        .await
        .expect("no tool call authorization event received")
        .unwrap();
    match event {
        ThreadEvent::ToolCallUpdate(acp_thread::ToolCallUpdate::UpdateFields(update)) => update,
        event => {
            panic!("Unexpected event {event:?}");
        }
    }
}

async fn next_tool_call_authorization(
    events: &mut UnboundedReceiver<Result<ThreadEvent>>,
) -> ToolCallAuthorization {
    loop {
        let event = events
            .next()
            .await
            .expect("no tool call authorization event received")
            .unwrap();
        if let ThreadEvent::ToolCallAuthorization(tool_call_authorization) = event {
            let permission_kinds = tool_call_authorization
                .options
                .first_option_of_kind(acp::PermissionOptionKind::AllowAlways)
                .map(|option| option.kind);
            let allow_once = tool_call_authorization
                .options
                .first_option_of_kind(acp::PermissionOptionKind::AllowOnce)
                .map(|option| option.kind);

            assert_eq!(
                permission_kinds,
                Some(acp::PermissionOptionKind::AllowAlways)
            );
            assert_eq!(allow_once, Some(acp::PermissionOptionKind::AllowOnce));
            return tool_call_authorization;
        }
    }
}

#[test]
fn test_permission_options_terminal_with_pattern() {
    let permission_options =
        ToolPermissionContext::new("terminal", "cargo build --release").build_permission_options();

    let PermissionOptions::Dropdown(choices) = permission_options else {
        panic!("Expected dropdown permission options");
    };

    assert_eq!(choices.len(), 3);
    let labels: Vec<&str> = choices
        .iter()
        .map(|choice| choice.allow.name.as_ref())
        .collect();
    assert!(labels.contains(&"Always for terminal"));
    assert!(labels.contains(&"Always for `cargo` commands"));
    assert!(labels.contains(&"Only this time"));
}

#[test]
fn test_permission_options_edit_file_with_path_pattern() {
    let permission_options =
        ToolPermissionContext::new("edit_file", "src/main.rs").build_permission_options();

    let PermissionOptions::Dropdown(choices) = permission_options else {
        panic!("Expected dropdown permission options");
    };

    let labels: Vec<&str> = choices
        .iter()
        .map(|choice| choice.allow.name.as_ref())
        .collect();
    assert!(labels.contains(&"Always for edit file"));
    assert!(labels.contains(&"Always for `src/`"));
}

#[test]
fn test_permission_options_fetch_with_domain_pattern() {
    let permission_options =
        ToolPermissionContext::new("fetch", "https://docs.rs/gpui").build_permission_options();

    let PermissionOptions::Dropdown(choices) = permission_options else {
        panic!("Expected dropdown permission options");
    };

    let labels: Vec<&str> = choices
        .iter()
        .map(|choice| choice.allow.name.as_ref())
        .collect();
    assert!(labels.contains(&"Always for fetch"));
    assert!(labels.contains(&"Always for `docs.rs`"));
}

#[test]
fn test_permission_options_without_pattern() {
    let permission_options = ToolPermissionContext::new("terminal", "./deploy.sh --production")
        .build_permission_options();

    let PermissionOptions::Dropdown(choices) = permission_options else {
        panic!("Expected dropdown permission options");
    };

    assert_eq!(choices.len(), 2);
    let labels: Vec<&str> = choices
        .iter()
        .map(|choice| choice.allow.name.as_ref())
        .collect();
    assert!(labels.contains(&"Always for terminal"));
    assert!(labels.contains(&"Only this time"));
    assert!(!labels.iter().any(|label| label.contains("commands")));
}

#[test]
fn test_permission_option_ids_for_terminal() {
    let permission_options =
        ToolPermissionContext::new("terminal", "cargo build --release").build_permission_options();

    let PermissionOptions::Dropdown(choices) = permission_options else {
        panic!("Expected dropdown permission options");
    };

    let allow_ids: Vec<String> = choices
        .iter()
        .map(|choice| choice.allow.option_id.0.to_string())
        .collect();
    let deny_ids: Vec<String> = choices
        .iter()
        .map(|choice| choice.deny.option_id.0.to_string())
        .collect();

    assert!(allow_ids.contains(&"always_allow:terminal".to_string()));
    assert!(allow_ids.contains(&"allow".to_string()));
    assert!(
        allow_ids
            .iter()
            .any(|id| id.starts_with("always_allow_pattern:terminal:")),
        "Missing allow pattern option"
    );

    assert!(deny_ids.contains(&"always_deny:terminal".to_string()));
    assert!(deny_ids.contains(&"deny".to_string()));
    assert!(
        deny_ids
            .iter()
            .any(|id| id.starts_with("always_deny_pattern:terminal:")),
        "Missing deny pattern option"
    );
}

#[gpui::test]
#[cfg_attr(not(feature = "e2e"), ignore)]
async fn test_concurrent_tool_calls(cx: &mut TestAppContext) {
    let ThreadTest { thread, .. } = setup(cx, TestModel::Sonnet4).await;

    // Test concurrent tool calls with different delay times
    let events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(DelayTool);
            thread.send(
                UserMessageId::new(),
                [
                    "Call the delay tool twice in the same message.",
                    "Once with 100ms. Once with 300ms.",
                    "When both timers are complete, describe the outputs.",
                ],
                cx,
            )
        })
        .unwrap()
        .collect()
        .await;

    let stop_reasons = stop_events(events);
    assert_eq!(stop_reasons, vec![acp::StopReason::EndTurn]);

    thread.update(cx, |thread, _cx| {
        let last_message = thread.last_message().unwrap();
        let agent_message = last_message.as_agent_message().unwrap();
        let text = agent_message
            .content
            .iter()
            .filter_map(|content| {
                if let AgentMessageContent::Text(text) = content {
                    Some(text.as_str())
                } else {
                    None
                }
            })
            .collect::<String>();

        assert!(text.contains("Ding"));
    });
}

#[gpui::test]
async fn test_profiles(cx: &mut TestAppContext) {
    let ThreadTest {
        model, thread, fs, ..
    } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    thread.update(cx, |thread, _cx| {
        thread.add_tool(DelayTool);
        thread.add_tool(EchoTool);
        thread.add_tool(InfiniteTool);
    });

    // Override profiles and wait for settings to be loaded.
    fs.insert_file(
        paths::settings_file(),
        json!({
            "agent": {
                "profiles": {
                    "test-1": {
                        "name": "Test Profile 1",
                        "tools": {
                            EchoTool::name(): true,
                            DelayTool::name(): true,
                        }
                    },
                    "test-2": {
                        "name": "Test Profile 2",
                        "tools": {
                            InfiniteTool::name(): true,
                        }
                    }
                }
            }
        })
        .to_string()
        .into_bytes(),
    )
    .await;
    cx.run_until_parked();

    // Test that test-1 profile (default) has echo and delay tools
    thread
        .update(cx, |thread, cx| {
            thread.set_profile(AgentProfileId("test-1".into()), cx);
            thread.send(UserMessageId::new(), ["test"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    let mut pending_completions = fake_model.pending_completions();
    assert_eq!(pending_completions.len(), 1);
    let completion = pending_completions.pop().unwrap();
    let tool_names: Vec<String> = completion
        .tools
        .iter()
        .map(|tool| tool.name.clone())
        .collect();
    assert_eq!(tool_names, vec![DelayTool::name(), EchoTool::name()]);
    fake_model.end_last_completion_stream();

    // Switch to test-2 profile, and verify that it has only the infinite tool.
    thread
        .update(cx, |thread, cx| {
            thread.set_profile(AgentProfileId("test-2".into()), cx);
            thread.send(UserMessageId::new(), ["test2"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    let mut pending_completions = fake_model.pending_completions();
    assert_eq!(pending_completions.len(), 1);
    let completion = pending_completions.pop().unwrap();
    let tool_names: Vec<String> = completion
        .tools
        .iter()
        .map(|tool| tool.name.clone())
        .collect();
    assert_eq!(tool_names, vec![InfiniteTool::name()]);
}

#[gpui::test]
async fn test_mcp_tools(cx: &mut TestAppContext) {
    let ThreadTest {
        model,
        thread,
        context_server_store,
        fs,
        ..
    } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    // Override profiles and wait for settings to be loaded.
    fs.insert_file(
        paths::settings_file(),
        json!({
            "agent": {
                "always_allow_tool_actions": true,
                "profiles": {
                    "test": {
                        "name": "Test Profile",
                        "enable_all_context_servers": true,
                        "tools": {
                            EchoTool::name(): true,
                        }
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

    let mut mcp_tool_calls = setup_context_server(
        "test_server",
        vec![context_server::types::Tool {
            name: "echo".into(),
            description: None,
            input_schema: serde_json::to_value(EchoTool::input_schema(
                LanguageModelToolSchemaFormat::JsonSchema,
            ))
            .unwrap(),
            output_schema: None,
            annotations: None,
        }],
        &context_server_store,
        cx,
    );

    let events = thread.update(cx, |thread, cx| {
        thread.send(UserMessageId::new(), ["Hey"], cx).unwrap()
    });
    cx.run_until_parked();

    // Simulate the model calling the MCP tool.
    let completion = fake_model.pending_completions().pop().unwrap();
    assert_eq!(tool_names_for_completion(&completion), vec!["echo"]);
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "tool_1".into(),
            name: "echo".into(),
            raw_input: json!({"text": "test"}).to_string(),
            input: json!({"text": "test"}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let (tool_call_params, tool_call_response) = mcp_tool_calls.next().await.unwrap();
    assert_eq!(tool_call_params.name, "echo");
    assert_eq!(tool_call_params.arguments, Some(json!({"text": "test"})));
    tool_call_response
        .send(context_server::types::CallToolResponse {
            content: vec![context_server::types::ToolResponseContent::Text {
                text: "test".into(),
            }],
            is_error: None,
            meta: None,
            structured_content: None,
        })
        .unwrap();
    cx.run_until_parked();

    assert_eq!(tool_names_for_completion(&completion), vec!["echo"]);
    fake_model.send_last_completion_stream_text_chunk("Done!");
    fake_model.end_last_completion_stream();
    events.collect::<Vec<_>>().await;

    // Send again after adding the echo tool, ensuring the name collision is resolved.
    let events = thread.update(cx, |thread, cx| {
        thread.add_tool(EchoTool);
        thread.send(UserMessageId::new(), ["Go"], cx).unwrap()
    });
    cx.run_until_parked();
    let completion = fake_model.pending_completions().pop().unwrap();
    assert_eq!(
        tool_names_for_completion(&completion),
        vec!["echo", "test_server_echo"]
    );
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "tool_2".into(),
            name: "test_server_echo".into(),
            raw_input: json!({"text": "mcp"}).to_string(),
            input: json!({"text": "mcp"}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "tool_3".into(),
            name: "echo".into(),
            raw_input: json!({"text": "native"}).to_string(),
            input: json!({"text": "native"}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let (tool_call_params, tool_call_response) = mcp_tool_calls.next().await.unwrap();
    assert_eq!(tool_call_params.name, "echo");
    assert_eq!(tool_call_params.arguments, Some(json!({"text": "mcp"})));
    tool_call_response
        .send(context_server::types::CallToolResponse {
            content: vec![context_server::types::ToolResponseContent::Text { text: "mcp".into() }],
            is_error: None,
            meta: None,
            structured_content: None,
        })
        .unwrap();
    cx.run_until_parked();

    // Ensure the tool results were inserted with the correct names.
    let completion = fake_model.pending_completions().pop().unwrap();
    assert_eq!(
        completion.messages.last().unwrap().content,
        vec![
            MessageContent::ToolResult(LanguageModelToolResult {
                tool_use_id: "tool_3".into(),
                tool_name: "echo".into(),
                is_error: false,
                content: "native".into(),
                output: Some("native".into()),
            },),
            MessageContent::ToolResult(LanguageModelToolResult {
                tool_use_id: "tool_2".into(),
                tool_name: "test_server_echo".into(),
                is_error: false,
                content: "mcp".into(),
                output: Some("mcp".into()),
            },),
        ]
    );
    fake_model.end_last_completion_stream();
    events.collect::<Vec<_>>().await;
}

#[gpui::test]
async fn test_mcp_tool_truncation(cx: &mut TestAppContext) {
    let ThreadTest {
        model,
        thread,
        context_server_store,
        fs,
        ..
    } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    // Set up a profile with all tools enabled
    fs.insert_file(
        paths::settings_file(),
        json!({
            "agent": {
                "profiles": {
                    "test": {
                        "name": "Test Profile",
                        "enable_all_context_servers": true,
                        "tools": {
                            EchoTool::name(): true,
                            DelayTool::name(): true,
                            WordListTool::name(): true,
                            ToolRequiringPermission::name(): true,
                            InfiniteTool::name(): true,
                        }
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
        thread.set_profile(AgentProfileId("test".into()), cx);
        thread.add_tool(EchoTool);
        thread.add_tool(DelayTool);
        thread.add_tool(WordListTool);
        thread.add_tool(ToolRequiringPermission);
        thread.add_tool(InfiniteTool);
    });

    // Set up multiple context servers with some overlapping tool names
    let _server1_calls = setup_context_server(
        "xxx",
        vec![
            context_server::types::Tool {
                name: "echo".into(), // Conflicts with native EchoTool
                description: None,
                input_schema: serde_json::to_value(EchoTool::input_schema(
                    LanguageModelToolSchemaFormat::JsonSchema,
                ))
                .unwrap(),
                output_schema: None,
                annotations: None,
            },
            context_server::types::Tool {
                name: "unique_tool_1".into(),
                description: None,
                input_schema: json!({"type": "object", "properties": {}}),
                output_schema: None,
                annotations: None,
            },
        ],
        &context_server_store,
        cx,
    );

    let _server2_calls = setup_context_server(
        "yyy",
        vec![
            context_server::types::Tool {
                name: "echo".into(), // Also conflicts with native EchoTool
                description: None,
                input_schema: serde_json::to_value(EchoTool::input_schema(
                    LanguageModelToolSchemaFormat::JsonSchema,
                ))
                .unwrap(),
                output_schema: None,
                annotations: None,
            },
            context_server::types::Tool {
                name: "unique_tool_2".into(),
                description: None,
                input_schema: json!({"type": "object", "properties": {}}),
                output_schema: None,
                annotations: None,
            },
            context_server::types::Tool {
                name: "a".repeat(MAX_TOOL_NAME_LENGTH - 2),
                description: None,
                input_schema: json!({"type": "object", "properties": {}}),
                output_schema: None,
                annotations: None,
            },
            context_server::types::Tool {
                name: "b".repeat(MAX_TOOL_NAME_LENGTH - 1),
                description: None,
                input_schema: json!({"type": "object", "properties": {}}),
                output_schema: None,
                annotations: None,
            },
        ],
        &context_server_store,
        cx,
    );
    let _server3_calls = setup_context_server(
        "zzz",
        vec![
            context_server::types::Tool {
                name: "a".repeat(MAX_TOOL_NAME_LENGTH - 2),
                description: None,
                input_schema: json!({"type": "object", "properties": {}}),
                output_schema: None,
                annotations: None,
            },
            context_server::types::Tool {
                name: "b".repeat(MAX_TOOL_NAME_LENGTH - 1),
                description: None,
                input_schema: json!({"type": "object", "properties": {}}),
                output_schema: None,
                annotations: None,
            },
            context_server::types::Tool {
                name: "c".repeat(MAX_TOOL_NAME_LENGTH + 1),
                description: None,
                input_schema: json!({"type": "object", "properties": {}}),
                output_schema: None,
                annotations: None,
            },
        ],
        &context_server_store,
        cx,
    );

    thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Go"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    let completion = fake_model.pending_completions().pop().unwrap();
    assert_eq!(
        tool_names_for_completion(&completion),
        vec![
            "bbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbbb",
            "cccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccccc",
            "delay",
            "echo",
            "infinite",
            "tool_requiring_permission",
            "unique_tool_1",
            "unique_tool_2",
            "word_list",
            "xxx_echo",
            "y_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
            "yyy_echo",
            "z_aaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaaa",
        ]
    );
}

#[gpui::test]
#[cfg_attr(not(feature = "e2e"), ignore)]
async fn test_cancellation(cx: &mut TestAppContext) {
    let ThreadTest { thread, .. } = setup(cx, TestModel::Sonnet4).await;

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(InfiniteTool);
            thread.add_tool(EchoTool);
            thread.send(
                UserMessageId::new(),
                ["Call the echo tool, then call the infinite tool, then explain their output"],
                cx,
            )
        })
        .unwrap();

    // Wait until both tools are called.
    let mut expected_tools = vec!["Echo", "Infinite Tool"];
    let mut echo_id = None;
    let mut echo_completed = false;
    while let Some(event) = events.next().await {
        match event.unwrap() {
            ThreadEvent::ToolCall(tool_call) => {
                assert_eq!(tool_call.title, expected_tools.remove(0));
                if tool_call.title == "Echo" {
                    echo_id = Some(tool_call.tool_call_id);
                }
            }
            ThreadEvent::ToolCallUpdate(acp_thread::ToolCallUpdate::UpdateFields(
                acp::ToolCallUpdate {
                    tool_call_id,
                    fields:
                        acp::ToolCallUpdateFields {
                            status: Some(acp::ToolCallStatus::Completed),
                            ..
                        },
                    ..
                },
            )) if Some(&tool_call_id) == echo_id.as_ref() => {
                echo_completed = true;
            }
            _ => {}
        }

        if expected_tools.is_empty() && echo_completed {
            break;
        }
    }

    // Cancel the current send and ensure that the event stream is closed, even
    // if one of the tools is still running.
    thread.update(cx, |thread, cx| thread.cancel(cx)).await;
    let events = events.collect::<Vec<_>>().await;
    let last_event = events.last();
    assert!(
        matches!(
            last_event,
            Some(Ok(ThreadEvent::Stop(acp::StopReason::Cancelled)))
        ),
        "unexpected event {last_event:?}"
    );

    // Ensure we can still send a new message after cancellation.
    let events = thread
        .update(cx, |thread, cx| {
            thread.send(
                UserMessageId::new(),
                ["Testing: reply with 'Hello' then stop."],
                cx,
            )
        })
        .unwrap()
        .collect::<Vec<_>>()
        .await;
    thread.update(cx, |thread, _cx| {
        let message = thread.last_message().unwrap();
        let agent_message = message.as_agent_message().unwrap();
        assert_eq!(
            agent_message.content,
            vec![AgentMessageContent::Text("Hello".to_string())]
        );
    });
    assert_eq!(stop_events(events), vec![acp::StopReason::EndTurn]);
}

#[gpui::test]
async fn test_terminal_tool_cancellation_captures_output(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    always_allow_tools(cx);
    let fake_model = model.as_fake();

    let handle = Rc::new(cx.update(|cx| FakeTerminalHandle::new_never_exits(cx)));
    let environment = Rc::new(FakeThreadEnvironment {
        handle: handle.clone(),
    });

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(crate::TerminalTool::new(
                thread.project().clone(),
                environment,
            ));
            thread.send(UserMessageId::new(), ["run a command"], cx)
        })
        .unwrap();

    cx.run_until_parked();

    // Simulate the model calling the terminal tool
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "terminal_tool_1".into(),
            name: "terminal".into(),
            raw_input: r#"{"command": "sleep 1000", "cd": "."}"#.into(),
            input: json!({"command": "sleep 1000", "cd": "."}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.end_last_completion_stream();

    // Wait for the terminal tool to start running
    wait_for_terminal_tool_started(&mut events, cx).await;

    // Cancel the thread while the terminal is running
    thread.update(cx, |thread, cx| thread.cancel(cx)).detach();

    // Collect remaining events, driving the executor to let cancellation complete
    let remaining_events = collect_events_until_stop(&mut events, cx).await;

    // Verify the terminal was killed
    assert!(
        handle.was_killed(),
        "expected terminal handle to be killed on cancellation"
    );

    // Verify we got a cancellation stop event
    assert_eq!(
        stop_events(remaining_events),
        vec![acp::StopReason::Cancelled],
    );

    // Verify the tool result contains the terminal output, not just "Tool canceled by user"
    thread.update(cx, |thread, _cx| {
        let message = thread.last_message().unwrap();
        let agent_message = message.as_agent_message().unwrap();

        let tool_use = agent_message
            .content
            .iter()
            .find_map(|content| match content {
                AgentMessageContent::ToolUse(tool_use) => Some(tool_use),
                _ => None,
            })
            .expect("expected tool use in agent message");

        let tool_result = agent_message
            .tool_results
            .get(&tool_use.id)
            .expect("expected tool result");

        let result_text = match &tool_result.content {
            language_model::LanguageModelToolResultContent::Text(text) => text.to_string(),
            _ => panic!("expected text content in tool result"),
        };

        // "partial output" comes from FakeTerminalHandle's output field
        assert!(
            result_text.contains("partial output"),
            "expected tool result to contain terminal output, got: {result_text}"
        );
        // Match the actual format from process_content in terminal_tool.rs
        assert!(
            result_text.contains("The user stopped this command"),
            "expected tool result to indicate user stopped, got: {result_text}"
        );
    });

    // Verify we can send a new message after cancellation
    verify_thread_recovery(&thread, &fake_model, cx).await;
}

#[gpui::test]
async fn test_cancellation_aware_tool_responds_to_cancellation(cx: &mut TestAppContext) {
    // This test verifies that tools which properly handle cancellation via
    // `event_stream.cancelled_by_user()` (like edit_file_tool) respond promptly
    // to cancellation and report that they were cancelled.
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    always_allow_tools(cx);
    let fake_model = model.as_fake();

    let (tool, was_cancelled) = CancellationAwareTool::new();

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(tool);
            thread.send(
                UserMessageId::new(),
                ["call the cancellation aware tool"],
                cx,
            )
        })
        .unwrap();

    cx.run_until_parked();

    // Simulate the model calling the cancellation-aware tool
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "cancellation_aware_1".into(),
            name: "cancellation_aware".into(),
            raw_input: r#"{}"#.into(),
            input: json!({}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.end_last_completion_stream();

    cx.run_until_parked();

    // Wait for the tool call to be reported
    let mut tool_started = false;
    let deadline = cx.executor().num_cpus() * 100;
    for _ in 0..deadline {
        cx.run_until_parked();

        while let Some(Some(event)) = events.next().now_or_never() {
            if let Ok(ThreadEvent::ToolCall(tool_call)) = &event {
                if tool_call.title == "Cancellation Aware Tool" {
                    tool_started = true;
                    break;
                }
            }
        }

        if tool_started {
            break;
        }

        cx.background_executor
            .timer(Duration::from_millis(10))
            .await;
    }
    assert!(tool_started, "expected cancellation aware tool to start");

    // Cancel the thread and wait for it to complete
    let cancel_task = thread.update(cx, |thread, cx| thread.cancel(cx));

    // The cancel task should complete promptly because the tool handles cancellation
    let timeout = cx.background_executor.timer(Duration::from_secs(5));
    futures::select! {
        _ = cancel_task.fuse() => {}
        _ = timeout.fuse() => {
            panic!("cancel task timed out - tool did not respond to cancellation");
        }
    }

    // Verify the tool detected cancellation via its flag
    assert!(
        was_cancelled.load(std::sync::atomic::Ordering::SeqCst),
        "tool should have detected cancellation via event_stream.cancelled_by_user()"
    );

    // Collect remaining events
    let remaining_events = collect_events_until_stop(&mut events, cx).await;

    // Verify we got a cancellation stop event
    assert_eq!(
        stop_events(remaining_events),
        vec![acp::StopReason::Cancelled],
    );

    // Verify we can send a new message after cancellation
    verify_thread_recovery(&thread, &fake_model, cx).await;
}

/// Helper to verify thread can recover after cancellation by sending a simple message.
async fn verify_thread_recovery(
    thread: &Entity<Thread>,
    fake_model: &FakeLanguageModel,
    cx: &mut TestAppContext,
) {
    let events = thread
        .update(cx, |thread, cx| {
            thread.send(
                UserMessageId::new(),
                ["Testing: reply with 'Hello' then stop."],
                cx,
            )
        })
        .unwrap();
    cx.run_until_parked();
    fake_model.send_last_completion_stream_text_chunk("Hello");
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();

    let events = events.collect::<Vec<_>>().await;
    thread.update(cx, |thread, _cx| {
        let message = thread.last_message().unwrap();
        let agent_message = message.as_agent_message().unwrap();
        assert_eq!(
            agent_message.content,
            vec![AgentMessageContent::Text("Hello".to_string())]
        );
    });
    assert_eq!(stop_events(events), vec![acp::StopReason::EndTurn]);
}

/// Waits for a terminal tool to start by watching for a ToolCallUpdate with terminal content.
async fn wait_for_terminal_tool_started(
    events: &mut mpsc::UnboundedReceiver<Result<ThreadEvent>>,
    cx: &mut TestAppContext,
) {
    let deadline = cx.executor().num_cpus() * 100; // Scale with available parallelism
    for _ in 0..deadline {
        cx.run_until_parked();

        while let Some(Some(event)) = events.next().now_or_never() {
            if let Ok(ThreadEvent::ToolCallUpdate(acp_thread::ToolCallUpdate::UpdateFields(
                update,
            ))) = &event
            {
                if update.fields.content.as_ref().is_some_and(|content| {
                    content
                        .iter()
                        .any(|c| matches!(c, acp::ToolCallContent::Terminal(_)))
                }) {
                    return;
                }
            }
        }

        cx.background_executor
            .timer(Duration::from_millis(10))
            .await;
    }
    panic!("terminal tool did not start within the expected time");
}

/// Collects events until a Stop event is received, driving the executor to completion.
async fn collect_events_until_stop(
    events: &mut mpsc::UnboundedReceiver<Result<ThreadEvent>>,
    cx: &mut TestAppContext,
) -> Vec<Result<ThreadEvent>> {
    let mut collected = Vec::new();
    let deadline = cx.executor().num_cpus() * 200;

    for _ in 0..deadline {
        cx.executor().advance_clock(Duration::from_millis(10));
        cx.run_until_parked();

        while let Some(Some(event)) = events.next().now_or_never() {
            let is_stop = matches!(&event, Ok(ThreadEvent::Stop(_)));
            collected.push(event);
            if is_stop {
                return collected;
            }
        }
    }
    panic!(
        "did not receive Stop event within the expected time; collected {} events",
        collected.len()
    );
}

#[gpui::test]
async fn test_truncate_while_terminal_tool_running(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    always_allow_tools(cx);
    let fake_model = model.as_fake();

    let handle = Rc::new(cx.update(|cx| FakeTerminalHandle::new_never_exits(cx)));
    let environment = Rc::new(FakeThreadEnvironment {
        handle: handle.clone(),
    });

    let message_id = UserMessageId::new();
    let mut events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(crate::TerminalTool::new(
                thread.project().clone(),
                environment,
            ));
            thread.send(message_id.clone(), ["run a command"], cx)
        })
        .unwrap();

    cx.run_until_parked();

    // Simulate the model calling the terminal tool
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "terminal_tool_1".into(),
            name: "terminal".into(),
            raw_input: r#"{"command": "sleep 1000", "cd": "."}"#.into(),
            input: json!({"command": "sleep 1000", "cd": "."}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.end_last_completion_stream();

    // Wait for the terminal tool to start running
    wait_for_terminal_tool_started(&mut events, cx).await;

    // Truncate the thread while the terminal is running
    thread
        .update(cx, |thread, cx| thread.truncate(message_id, cx))
        .unwrap();

    // Drive the executor to let cancellation complete
    let _ = collect_events_until_stop(&mut events, cx).await;

    // Verify the terminal was killed
    assert!(
        handle.was_killed(),
        "expected terminal handle to be killed on truncate"
    );

    // Verify the thread is empty after truncation
    thread.update(cx, |thread, _cx| {
        assert_eq!(
            thread.to_markdown(),
            "",
            "expected thread to be empty after truncating the only message"
        );
    });

    // Verify we can send a new message after truncation
    verify_thread_recovery(&thread, &fake_model, cx).await;
}

#[gpui::test]
async fn test_cancel_multiple_concurrent_terminal_tools(cx: &mut TestAppContext) {
    // Tests that cancellation properly kills all running terminal tools when multiple are active.
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    always_allow_tools(cx);
    let fake_model = model.as_fake();

    let environment = Rc::new(MultiTerminalEnvironment::new());

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(crate::TerminalTool::new(
                thread.project().clone(),
                environment.clone(),
            ));
            thread.send(UserMessageId::new(), ["run multiple commands"], cx)
        })
        .unwrap();

    cx.run_until_parked();

    // Simulate the model calling two terminal tools
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "terminal_tool_1".into(),
            name: "terminal".into(),
            raw_input: r#"{"command": "sleep 1000", "cd": "."}"#.into(),
            input: json!({"command": "sleep 1000", "cd": "."}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "terminal_tool_2".into(),
            name: "terminal".into(),
            raw_input: r#"{"command": "sleep 2000", "cd": "."}"#.into(),
            input: json!({"command": "sleep 2000", "cd": "."}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.end_last_completion_stream();

    // Wait for both terminal tools to start by counting terminal content updates
    let mut terminals_started = 0;
    let deadline = cx.executor().num_cpus() * 100;
    for _ in 0..deadline {
        cx.run_until_parked();

        while let Some(Some(event)) = events.next().now_or_never() {
            if let Ok(ThreadEvent::ToolCallUpdate(acp_thread::ToolCallUpdate::UpdateFields(
                update,
            ))) = &event
            {
                if update.fields.content.as_ref().is_some_and(|content| {
                    content
                        .iter()
                        .any(|c| matches!(c, acp::ToolCallContent::Terminal(_)))
                }) {
                    terminals_started += 1;
                    if terminals_started >= 2 {
                        break;
                    }
                }
            }
        }
        if terminals_started >= 2 {
            break;
        }

        cx.background_executor
            .timer(Duration::from_millis(10))
            .await;
    }
    assert!(
        terminals_started >= 2,
        "expected 2 terminal tools to start, got {terminals_started}"
    );

    // Cancel the thread while both terminals are running
    thread.update(cx, |thread, cx| thread.cancel(cx)).detach();

    // Collect remaining events
    let remaining_events = collect_events_until_stop(&mut events, cx).await;

    // Verify both terminal handles were killed
    let handles = environment.handles();
    assert_eq!(
        handles.len(),
        2,
        "expected 2 terminal handles to be created"
    );
    assert!(
        handles[0].was_killed(),
        "expected first terminal handle to be killed on cancellation"
    );
    assert!(
        handles[1].was_killed(),
        "expected second terminal handle to be killed on cancellation"
    );

    // Verify we got a cancellation stop event
    assert_eq!(
        stop_events(remaining_events),
        vec![acp::StopReason::Cancelled],
    );
}

#[gpui::test]
async fn test_terminal_tool_stopped_via_terminal_card_button(cx: &mut TestAppContext) {
    // Tests that clicking the stop button on the terminal card (as opposed to the main
    // cancel button) properly reports user stopped via the was_stopped_by_user path.
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    always_allow_tools(cx);
    let fake_model = model.as_fake();

    let handle = Rc::new(cx.update(|cx| FakeTerminalHandle::new_never_exits(cx)));
    let environment = Rc::new(FakeThreadEnvironment {
        handle: handle.clone(),
    });

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(crate::TerminalTool::new(
                thread.project().clone(),
                environment,
            ));
            thread.send(UserMessageId::new(), ["run a command"], cx)
        })
        .unwrap();

    cx.run_until_parked();

    // Simulate the model calling the terminal tool
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "terminal_tool_1".into(),
            name: "terminal".into(),
            raw_input: r#"{"command": "sleep 1000", "cd": "."}"#.into(),
            input: json!({"command": "sleep 1000", "cd": "."}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.end_last_completion_stream();

    // Wait for the terminal tool to start running
    wait_for_terminal_tool_started(&mut events, cx).await;

    // Simulate user clicking stop on the terminal card itself.
    // This sets the flag and signals exit (simulating what the real UI would do).
    handle.set_stopped_by_user(true);
    handle.killed.store(true, Ordering::SeqCst);
    handle.signal_exit();

    // Wait for the tool to complete
    cx.run_until_parked();

    // The thread continues after tool completion - simulate the model ending its turn
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();

    // Collect remaining events
    let remaining_events = collect_events_until_stop(&mut events, cx).await;

    // Verify we got an EndTurn (not Cancelled, since we didn't cancel the thread)
    assert_eq!(
        stop_events(remaining_events),
        vec![acp::StopReason::EndTurn],
    );

    // Verify the tool result indicates user stopped
    thread.update(cx, |thread, _cx| {
        let message = thread.last_message().unwrap();
        let agent_message = message.as_agent_message().unwrap();

        let tool_use = agent_message
            .content
            .iter()
            .find_map(|content| match content {
                AgentMessageContent::ToolUse(tool_use) => Some(tool_use),
                _ => None,
            })
            .expect("expected tool use in agent message");

        let tool_result = agent_message
            .tool_results
            .get(&tool_use.id)
            .expect("expected tool result");

        let result_text = match &tool_result.content {
            language_model::LanguageModelToolResultContent::Text(text) => text.to_string(),
            _ => panic!("expected text content in tool result"),
        };

        assert!(
            result_text.contains("The user stopped this command"),
            "expected tool result to indicate user stopped, got: {result_text}"
        );
    });
}

#[gpui::test]
async fn test_terminal_tool_timeout_expires(cx: &mut TestAppContext) {
    // Tests that when a timeout is configured and expires, the tool result indicates timeout.
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    always_allow_tools(cx);
    let fake_model = model.as_fake();

    let handle = Rc::new(cx.update(|cx| FakeTerminalHandle::new_never_exits(cx)));
    let environment = Rc::new(FakeThreadEnvironment {
        handle: handle.clone(),
    });

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(crate::TerminalTool::new(
                thread.project().clone(),
                environment,
            ));
            thread.send(UserMessageId::new(), ["run a command with timeout"], cx)
        })
        .unwrap();

    cx.run_until_parked();

    // Simulate the model calling the terminal tool with a short timeout
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "terminal_tool_1".into(),
            name: "terminal".into(),
            raw_input: r#"{"command": "sleep 1000", "cd": ".", "timeout_ms": 100}"#.into(),
            input: json!({"command": "sleep 1000", "cd": ".", "timeout_ms": 100}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.end_last_completion_stream();

    // Wait for the terminal tool to start running
    wait_for_terminal_tool_started(&mut events, cx).await;

    // Advance clock past the timeout
    cx.executor().advance_clock(Duration::from_millis(200));
    cx.run_until_parked();

    // The thread continues after tool completion - simulate the model ending its turn
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();

    // Collect remaining events
    let remaining_events = collect_events_until_stop(&mut events, cx).await;

    // Verify the terminal was killed due to timeout
    assert!(
        handle.was_killed(),
        "expected terminal handle to be killed on timeout"
    );

    // Verify we got an EndTurn (the tool completed, just with timeout)
    assert_eq!(
        stop_events(remaining_events),
        vec![acp::StopReason::EndTurn],
    );

    // Verify the tool result indicates timeout, not user stopped
    thread.update(cx, |thread, _cx| {
        let message = thread.last_message().unwrap();
        let agent_message = message.as_agent_message().unwrap();

        let tool_use = agent_message
            .content
            .iter()
            .find_map(|content| match content {
                AgentMessageContent::ToolUse(tool_use) => Some(tool_use),
                _ => None,
            })
            .expect("expected tool use in agent message");

        let tool_result = agent_message
            .tool_results
            .get(&tool_use.id)
            .expect("expected tool result");

        let result_text = match &tool_result.content {
            language_model::LanguageModelToolResultContent::Text(text) => text.to_string(),
            _ => panic!("expected text content in tool result"),
        };

        assert!(
            result_text.contains("timed out"),
            "expected tool result to indicate timeout, got: {result_text}"
        );
        assert!(
            !result_text.contains("The user stopped"),
            "tool result should not mention user stopped when it timed out, got: {result_text}"
        );
    });
}

#[gpui::test]
async fn test_in_progress_send_canceled_by_next_send(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let events_1 = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Hello 1"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    fake_model.send_last_completion_stream_text_chunk("Hey 1!");
    cx.run_until_parked();

    let events_2 = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Hello 2"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    fake_model.send_last_completion_stream_text_chunk("Hey 2!");
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();

    let events_1 = events_1.collect::<Vec<_>>().await;
    assert_eq!(stop_events(events_1), vec![acp::StopReason::Cancelled]);
    let events_2 = events_2.collect::<Vec<_>>().await;
    assert_eq!(stop_events(events_2), vec![acp::StopReason::EndTurn]);
}

#[gpui::test]
async fn test_subsequent_successful_sends_dont_cancel(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let events_1 = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Hello 1"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    fake_model.send_last_completion_stream_text_chunk("Hey 1!");
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();
    let events_1 = events_1.collect::<Vec<_>>().await;

    let events_2 = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Hello 2"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    fake_model.send_last_completion_stream_text_chunk("Hey 2!");
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();
    let events_2 = events_2.collect::<Vec<_>>().await;

    assert_eq!(stop_events(events_1), vec![acp::StopReason::EndTurn]);
    assert_eq!(stop_events(events_2), vec![acp::StopReason::EndTurn]);
}

#[gpui::test]
async fn test_refusal(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let events = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Hello"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    thread.read_with(cx, |thread, _| {
        assert_eq!(
            thread.to_markdown(),
            indoc! {"
                ## User

                Hello
            "}
        );
    });

    fake_model.send_last_completion_stream_text_chunk("Hey!");
    cx.run_until_parked();
    thread.read_with(cx, |thread, _| {
        assert_eq!(
            thread.to_markdown(),
            indoc! {"
                ## User

                Hello

                ## Assistant

                Hey!
            "}
        );
    });

    // If the model refuses to continue, the thread should remove all the messages after the last user message.
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::Refusal));
    let events = events.collect::<Vec<_>>().await;
    assert_eq!(stop_events(events), vec![acp::StopReason::Refusal]);
    thread.read_with(cx, |thread, _| {
        assert_eq!(thread.to_markdown(), "");
    });
}

#[gpui::test]
async fn test_truncate_first_message(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let message_id = UserMessageId::new();
    thread
        .update(cx, |thread, cx| {
            thread.send(message_id.clone(), ["Hello"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    thread.read_with(cx, |thread, _| {
        assert_eq!(
            thread.to_markdown(),
            indoc! {"
                ## User

                Hello
            "}
        );
        assert_eq!(thread.latest_token_usage(), None);
    });

    fake_model.send_last_completion_stream_text_chunk("Hey!");
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::UsageUpdate(
        language_model::TokenUsage {
            input_tokens: 32_000,
            output_tokens: 16_000,
            cache_creation_input_tokens: 0,
            cache_read_input_tokens: 0,
        },
    ));
    cx.run_until_parked();
    thread.read_with(cx, |thread, _| {
        assert_eq!(
            thread.to_markdown(),
            indoc! {"
                ## User

                Hello

                ## Assistant

                Hey!
            "}
        );
        assert_eq!(
            thread.latest_token_usage(),
            Some(acp_thread::TokenUsage {
                used_tokens: 32_000 + 16_000,
                max_tokens: 1_000_000,
                input_tokens: 32_000,
                output_tokens: 16_000,
            })
        );
    });

    thread
        .update(cx, |thread, cx| thread.truncate(message_id, cx))
        .unwrap();
    cx.run_until_parked();
    thread.read_with(cx, |thread, _| {
        assert_eq!(thread.to_markdown(), "");
        assert_eq!(thread.latest_token_usage(), None);
    });

    // Ensure we can still send a new message after truncation.
    thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Hi"], cx)
        })
        .unwrap();
    thread.update(cx, |thread, _cx| {
        assert_eq!(
            thread.to_markdown(),
            indoc! {"
                ## User

                Hi
            "}
        );
    });
    cx.run_until_parked();
    fake_model.send_last_completion_stream_text_chunk("Ahoy!");
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::UsageUpdate(
        language_model::TokenUsage {
            input_tokens: 40_000,
            output_tokens: 20_000,
            cache_creation_input_tokens: 0,
            cache_read_input_tokens: 0,
        },
    ));
    cx.run_until_parked();
    thread.read_with(cx, |thread, _| {
        assert_eq!(
            thread.to_markdown(),
            indoc! {"
                ## User

                Hi

                ## Assistant

                Ahoy!
            "}
        );

        assert_eq!(
            thread.latest_token_usage(),
            Some(acp_thread::TokenUsage {
                used_tokens: 40_000 + 20_000,
                max_tokens: 1_000_000,
                input_tokens: 40_000,
                output_tokens: 20_000,
            })
        );
    });
}

#[gpui::test]
async fn test_truncate_second_message(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Message 1"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    fake_model.send_last_completion_stream_text_chunk("Message 1 response");
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::UsageUpdate(
        language_model::TokenUsage {
            input_tokens: 32_000,
            output_tokens: 16_000,
            cache_creation_input_tokens: 0,
            cache_read_input_tokens: 0,
        },
    ));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let assert_first_message_state = |cx: &mut TestAppContext| {
        thread.clone().read_with(cx, |thread, _| {
            assert_eq!(
                thread.to_markdown(),
                indoc! {"
                    ## User

                    Message 1

                    ## Assistant

                    Message 1 response
                "}
            );

            assert_eq!(
                thread.latest_token_usage(),
                Some(acp_thread::TokenUsage {
                    used_tokens: 32_000 + 16_000,
                    max_tokens: 1_000_000,
                    input_tokens: 32_000,
                    output_tokens: 16_000,
                })
            );
        });
    };

    assert_first_message_state(cx);

    let second_message_id = UserMessageId::new();
    thread
        .update(cx, |thread, cx| {
            thread.send(second_message_id.clone(), ["Message 2"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    fake_model.send_last_completion_stream_text_chunk("Message 2 response");
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::UsageUpdate(
        language_model::TokenUsage {
            input_tokens: 40_000,
            output_tokens: 20_000,
            cache_creation_input_tokens: 0,
            cache_read_input_tokens: 0,
        },
    ));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    thread.read_with(cx, |thread, _| {
        assert_eq!(
            thread.to_markdown(),
            indoc! {"
                ## User

                Message 1

                ## Assistant

                Message 1 response

                ## User

                Message 2

                ## Assistant

                Message 2 response
            "}
        );

        assert_eq!(
            thread.latest_token_usage(),
            Some(acp_thread::TokenUsage {
                used_tokens: 40_000 + 20_000,
                max_tokens: 1_000_000,
                input_tokens: 40_000,
                output_tokens: 20_000,
            })
        );
    });

    thread
        .update(cx, |thread, cx| thread.truncate(second_message_id, cx))
        .unwrap();
    cx.run_until_parked();

    assert_first_message_state(cx);
}

#[gpui::test]
async fn test_title_generation(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let summary_model = Arc::new(FakeLanguageModel::default());
    thread.update(cx, |thread, cx| {
        thread.set_summarization_model(Some(summary_model.clone()), cx)
    });

    let send = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Hello"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    fake_model.send_last_completion_stream_text_chunk("Hey!");
    fake_model.end_last_completion_stream();
    cx.run_until_parked();
    thread.read_with(cx, |thread, _| assert_eq!(thread.title(), "New Thread"));

    // Ensure the summary model has been invoked to generate a title.
    summary_model.send_last_completion_stream_text_chunk("Hello ");
    summary_model.send_last_completion_stream_text_chunk("world\nG");
    summary_model.send_last_completion_stream_text_chunk("oodnight Moon");
    summary_model.end_last_completion_stream();
    send.collect::<Vec<_>>().await;
    cx.run_until_parked();
    thread.read_with(cx, |thread, _| assert_eq!(thread.title(), "Hello world"));

    // Send another message, ensuring no title is generated this time.
    let send = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Hello again"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    fake_model.send_last_completion_stream_text_chunk("Hey again!");
    fake_model.end_last_completion_stream();
    cx.run_until_parked();
    assert_eq!(summary_model.pending_completions(), Vec::new());
    send.collect::<Vec<_>>().await;
    thread.read_with(cx, |thread, _| assert_eq!(thread.title(), "Hello world"));
}

#[gpui::test]
async fn test_building_request_with_pending_tools(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let _events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(ToolRequiringPermission);
            thread.add_tool(EchoTool);
            thread.send(UserMessageId::new(), ["Hey!"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    let permission_tool_use = LanguageModelToolUse {
        id: "tool_id_1".into(),
        name: ToolRequiringPermission::name().into(),
        raw_input: "{}".into(),
        input: json!({}),
        is_input_complete: true,
        thought_signature: None,
    };
    let echo_tool_use = LanguageModelToolUse {
        id: "tool_id_2".into(),
        name: EchoTool::name().into(),
        raw_input: json!({"text": "test"}).to_string(),
        input: json!({"text": "test"}),
        is_input_complete: true,
        thought_signature: None,
    };
    fake_model.send_last_completion_stream_text_chunk("Hi!");
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        permission_tool_use,
    ));
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        echo_tool_use.clone(),
    ));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // Ensure pending tools are skipped when building a request.
    let request = thread
        .read_with(cx, |thread, cx| {
            thread.build_completion_request(CompletionIntent::EditFile, cx)
        })
        .unwrap();
    assert_eq!(
        request.messages[1..],
        vec![
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["Hey!".into()],
                cache: true,
                reasoning_details: None,
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![
                    MessageContent::Text("Hi!".into()),
                    MessageContent::ToolUse(echo_tool_use.clone())
                ],
                cache: false,
                reasoning_details: None,
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::ToolResult(LanguageModelToolResult {
                    tool_use_id: echo_tool_use.id.clone(),
                    tool_name: echo_tool_use.name,
                    is_error: false,
                    content: "test".into(),
                    output: Some("test".into())
                })],
                cache: false,
                reasoning_details: None,
            },
        ],
    );
}

#[gpui::test]
async fn test_agent_connection(cx: &mut TestAppContext) {
    cx.update(settings::init);
    let templates = Templates::new();

    // Initialize language model system with test provider
    cx.update(|cx| {
        gpui_tokio::init(cx);

        let http_client = FakeHttpClient::with_404_response();
        let clock = Arc::new(clock::FakeSystemClock::new());
        let client = Client::new(clock, http_client, cx);
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        language_model::init(client.clone(), cx);
        language_models::init(user_store, client.clone(), cx);
        LanguageModelRegistry::test(cx);
    });
    cx.executor().forbid_parking();

    // Create a project for new_thread
    let fake_fs = cx.update(|cx| fs::FakeFs::new(cx.background_executor().clone()));
    fake_fs.insert_tree(path!("/test"), json!({})).await;
    let project = Project::test(fake_fs.clone(), [Path::new("/test")], cx).await;
    let cwd = Path::new("/test");
    let thread_store = cx.new(|cx| ThreadStore::new(cx));

    // Create agent and connection
    let agent = NativeAgent::new(
        project.clone(),
        thread_store,
        templates.clone(),
        None,
        fake_fs.clone(),
        &mut cx.to_async(),
    )
    .await
    .unwrap();
    let connection = NativeAgentConnection(agent.clone());

    // Create a thread using new_thread
    let connection_rc = Rc::new(connection.clone());
    let acp_thread = cx
        .update(|cx| connection_rc.new_thread(project, cwd, cx))
        .await
        .expect("new_thread should succeed");

    // Get the session_id from the AcpThread
    let session_id = acp_thread.read_with(cx, |thread, _| thread.session_id().clone());

    // Test model_selector returns Some
    let selector_opt = connection.model_selector(&session_id);
    assert!(
        selector_opt.is_some(),
        "agent should always support ModelSelector"
    );
    let selector = selector_opt.unwrap();

    // Test list_models
    let listed_models = cx
        .update(|cx| selector.list_models(cx))
        .await
        .expect("list_models should succeed");
    let AgentModelList::Grouped(listed_models) = listed_models else {
        panic!("Unexpected model list type");
    };
    assert!(!listed_models.is_empty(), "should have at least one model");
    assert_eq!(
        listed_models[&AgentModelGroupName("Fake".into())][0]
            .id
            .0
            .as_ref(),
        "fake/fake"
    );

    // Test selected_model returns the default
    let model = cx
        .update(|cx| selector.selected_model(cx))
        .await
        .expect("selected_model should succeed");
    let model = cx
        .update(|cx| agent.read(cx).models().model_from_id(&model.id))
        .unwrap();
    let model = model.as_fake();
    assert_eq!(model.id().0, "fake", "should return default model");

    let request = acp_thread.update(cx, |thread, cx| thread.send(vec!["abc".into()], cx));
    cx.run_until_parked();
    model.send_last_completion_stream_text_chunk("def");
    cx.run_until_parked();
    acp_thread.read_with(cx, |thread, cx| {
        assert_eq!(
            thread.to_markdown(cx),
            indoc! {"
                ## User

                abc

                ## Assistant

                def

            "}
        )
    });

    // Test cancel
    cx.update(|cx| connection.cancel(&session_id, cx));
    request.await.expect("prompt should fail gracefully");

    // Ensure that dropping the ACP thread causes the native thread to be
    // dropped as well.
    cx.update(|_| drop(acp_thread));
    let result = cx
        .update(|cx| {
            connection.prompt(
                Some(acp_thread::UserMessageId::new()),
                acp::PromptRequest::new(session_id.clone(), vec!["ghi".into()]),
                cx,
            )
        })
        .await;
    assert_eq!(
        result.as_ref().unwrap_err().to_string(),
        "Session not found",
        "unexpected result: {:?}",
        result
    );
}

#[gpui::test]
async fn test_tool_updates_to_completion(cx: &mut TestAppContext) {
    let ThreadTest { thread, model, .. } = setup(cx, TestModel::Fake).await;
    thread.update(cx, |thread, _cx| thread.add_tool(ThinkingTool));
    let fake_model = model.as_fake();

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Think"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    // Simulate streaming partial input.
    let input = json!({});
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "1".into(),
            name: ThinkingTool::name().into(),
            raw_input: input.to_string(),
            input,
            is_input_complete: false,
            thought_signature: None,
        },
    ));

    // Input streaming completed
    let input = json!({ "content": "Thinking hard!" });
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "1".into(),
            name: "thinking".into(),
            raw_input: input.to_string(),
            input,
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let tool_call = expect_tool_call(&mut events).await;
    assert_eq!(
        tool_call,
        acp::ToolCall::new("1", "Thinking")
            .kind(acp::ToolKind::Think)
            .raw_input(json!({}))
            .meta(acp::Meta::from_iter([(
                "tool_name".into(),
                "thinking".into()
            )]))
    );
    let update = expect_tool_call_update_fields(&mut events).await;
    assert_eq!(
        update,
        acp::ToolCallUpdate::new(
            "1",
            acp::ToolCallUpdateFields::new()
                .title("Thinking")
                .kind(acp::ToolKind::Think)
                .raw_input(json!({ "content": "Thinking hard!"}))
        )
    );
    let update = expect_tool_call_update_fields(&mut events).await;
    assert_eq!(
        update,
        acp::ToolCallUpdate::new(
            "1",
            acp::ToolCallUpdateFields::new().status(acp::ToolCallStatus::InProgress)
        )
    );
    let update = expect_tool_call_update_fields(&mut events).await;
    assert_eq!(
        update,
        acp::ToolCallUpdate::new(
            "1",
            acp::ToolCallUpdateFields::new().content(vec!["Thinking hard!".into()])
        )
    );
    let update = expect_tool_call_update_fields(&mut events).await;
    assert_eq!(
        update,
        acp::ToolCallUpdate::new(
            "1",
            acp::ToolCallUpdateFields::new()
                .status(acp::ToolCallStatus::Completed)
                .raw_output("Finished thinking.")
        )
    );
}

#[gpui::test]
async fn test_send_no_retry_on_success(cx: &mut TestAppContext) {
    let ThreadTest { thread, model, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Hello!"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    fake_model.send_last_completion_stream_text_chunk("Hey!");
    fake_model.end_last_completion_stream();

    let mut retry_events = Vec::new();
    while let Some(Ok(event)) = events.next().await {
        match event {
            ThreadEvent::Retry(retry_status) => {
                retry_events.push(retry_status);
            }
            ThreadEvent::Stop(..) => break,
            _ => {}
        }
    }

    assert_eq!(retry_events.len(), 0);
    thread.read_with(cx, |thread, _cx| {
        assert_eq!(
            thread.to_markdown(),
            indoc! {"
                ## User

                Hello!

                ## Assistant

                Hey!
            "}
        )
    });
}

#[gpui::test]
async fn test_send_retry_on_error(cx: &mut TestAppContext) {
    let ThreadTest { thread, model, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Hello!"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    fake_model.send_last_completion_stream_text_chunk("Hey,");
    fake_model.send_last_completion_stream_error(LanguageModelCompletionError::ServerOverloaded {
        provider: LanguageModelProviderName::new("Anthropic"),
        retry_after: Some(Duration::from_secs(3)),
    });
    fake_model.end_last_completion_stream();

    cx.executor().advance_clock(Duration::from_secs(3));
    cx.run_until_parked();

    fake_model.send_last_completion_stream_text_chunk("there!");
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let mut retry_events = Vec::new();
    while let Some(Ok(event)) = events.next().await {
        match event {
            ThreadEvent::Retry(retry_status) => {
                retry_events.push(retry_status);
            }
            ThreadEvent::Stop(..) => break,
            _ => {}
        }
    }

    assert_eq!(retry_events.len(), 1);
    assert!(matches!(
        retry_events[0],
        acp_thread::RetryStatus { attempt: 1, .. }
    ));
    thread.read_with(cx, |thread, _cx| {
        assert_eq!(
            thread.to_markdown(),
            indoc! {"
                ## User

                Hello!

                ## Assistant

                Hey,

                [resume]

                ## Assistant

                there!
            "}
        )
    });
}

#[gpui::test]
async fn test_send_retry_finishes_tool_calls_on_error(cx: &mut TestAppContext) {
    let ThreadTest { thread, model, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(EchoTool);
            thread.send(UserMessageId::new(), ["Call the echo tool!"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    let tool_use_1 = LanguageModelToolUse {
        id: "tool_1".into(),
        name: EchoTool::name().into(),
        raw_input: json!({"text": "test"}).to_string(),
        input: json!({"text": "test"}),
        is_input_complete: true,
        thought_signature: None,
    };
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        tool_use_1.clone(),
    ));
    fake_model.send_last_completion_stream_error(LanguageModelCompletionError::ServerOverloaded {
        provider: LanguageModelProviderName::new("Anthropic"),
        retry_after: Some(Duration::from_secs(3)),
    });
    fake_model.end_last_completion_stream();

    cx.executor().advance_clock(Duration::from_secs(3));
    let completion = fake_model.pending_completions().pop().unwrap();
    assert_eq!(
        completion.messages[1..],
        vec![
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["Call the echo tool!".into()],
                cache: false,
                reasoning_details: None,
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![language_model::MessageContent::ToolUse(tool_use_1.clone())],
                cache: false,
                reasoning_details: None,
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec![language_model::MessageContent::ToolResult(
                    LanguageModelToolResult {
                        tool_use_id: tool_use_1.id.clone(),
                        tool_name: tool_use_1.name.clone(),
                        is_error: false,
                        content: "test".into(),
                        output: Some("test".into())
                    }
                )],
                cache: true,
                reasoning_details: None,
            },
        ]
    );

    fake_model.send_last_completion_stream_text_chunk("Done");
    fake_model.end_last_completion_stream();
    cx.run_until_parked();
    events.collect::<Vec<_>>().await;
    thread.read_with(cx, |thread, _cx| {
        assert_eq!(
            thread.last_message(),
            Some(Message::Agent(AgentMessage {
                content: vec![AgentMessageContent::Text("Done".into())],
                tool_results: IndexMap::default(),
                reasoning_details: None,
            }))
        );
    })
}

#[gpui::test]
async fn test_send_max_retries_exceeded(cx: &mut TestAppContext) {
    let ThreadTest { thread, model, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Hello!"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    for _ in 0..crate::thread::MAX_RETRY_ATTEMPTS + 1 {
        fake_model.send_last_completion_stream_error(
            LanguageModelCompletionError::ServerOverloaded {
                provider: LanguageModelProviderName::new("Anthropic"),
                retry_after: Some(Duration::from_secs(3)),
            },
        );
        fake_model.end_last_completion_stream();
        cx.executor().advance_clock(Duration::from_secs(3));
        cx.run_until_parked();
    }

    let mut errors = Vec::new();
    let mut retry_events = Vec::new();
    while let Some(event) = events.next().await {
        match event {
            Ok(ThreadEvent::Retry(retry_status)) => {
                retry_events.push(retry_status);
            }
            Ok(ThreadEvent::Stop(..)) => break,
            Err(error) => errors.push(error),
            _ => {}
        }
    }

    assert_eq!(
        retry_events.len(),
        crate::thread::MAX_RETRY_ATTEMPTS as usize
    );
    for i in 0..crate::thread::MAX_RETRY_ATTEMPTS as usize {
        assert_eq!(retry_events[i].attempt, i + 1);
    }
    assert_eq!(errors.len(), 1);
    let error = errors[0]
        .downcast_ref::<LanguageModelCompletionError>()
        .unwrap();
    assert!(matches!(
        error,
        LanguageModelCompletionError::ServerOverloaded { .. }
    ));
}

/// Filters out the stop events for asserting against in tests
fn stop_events(result_events: Vec<Result<ThreadEvent>>) -> Vec<acp::StopReason> {
    result_events
        .into_iter()
        .filter_map(|event| match event.unwrap() {
            ThreadEvent::Stop(stop_reason) => Some(stop_reason),
            _ => None,
        })
        .collect()
}

struct ThreadTest {
    model: Arc<dyn LanguageModel>,
    thread: Entity<Thread>,
    project_context: Entity<ProjectContext>,
    context_server_store: Entity<ContextServerStore>,
    fs: Arc<FakeFs>,
}

enum TestModel {
    Sonnet4,
    Fake,
}

impl TestModel {
    fn id(&self) -> LanguageModelId {
        match self {
            TestModel::Sonnet4 => LanguageModelId("claude-sonnet-4-latest".into()),
            TestModel::Fake => unreachable!(),
        }
    }
}

async fn setup(cx: &mut TestAppContext, model: TestModel) -> ThreadTest {
    cx.executor().allow_parking();

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.create_dir(paths::settings_file().parent().unwrap())
        .await
        .unwrap();
    fs.insert_file(
        paths::settings_file(),
        json!({
            "agent": {
                "default_profile": "test-profile",
                "profiles": {
                    "test-profile": {
                        "name": "Test Profile",
                        "tools": {
                            EchoTool::name(): true,
                            DelayTool::name(): true,
                            WordListTool::name(): true,
                            ToolRequiringPermission::name(): true,
                            InfiniteTool::name(): true,
                            CancellationAwareTool::name(): true,
                            ThinkingTool::name(): true,
                            "terminal": true,
                        }
                    }
                }
            }
        })
        .to_string()
        .into_bytes(),
    )
    .await;

    cx.update(|cx| {
        settings::init(cx);

        match model {
            TestModel::Fake => {}
            TestModel::Sonnet4 => {
                gpui_tokio::init(cx);
                let http_client = ReqwestClient::user_agent("agent tests").unwrap();
                cx.set_http_client(Arc::new(http_client));
                let client = Client::production(cx);
                let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
                language_model::init(client.clone(), cx);
                language_models::init(user_store, client.clone(), cx);
            }
        };

        watch_settings(fs.clone(), cx);
    });

    let templates = Templates::new();

    fs.insert_tree(path!("/test"), json!({})).await;
    let project = Project::test(fs.clone(), [path!("/test").as_ref()], cx).await;

    let model = cx
        .update(|cx| {
            if let TestModel::Fake = model {
                Task::ready(Arc::new(FakeLanguageModel::default()) as Arc<_>)
            } else {
                let model_id = model.id();
                let models = LanguageModelRegistry::read_global(cx);
                let model = models
                    .available_models(cx)
                    .find(|model| model.id() == model_id)
                    .unwrap();

                let provider = models.provider(&model.provider_id()).unwrap();
                let authenticated = provider.authenticate(cx);

                cx.spawn(async move |_cx| {
                    authenticated.await.unwrap();
                    model
                })
            }
        })
        .await;

    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));
    let thread = cx.new(|cx| {
        Thread::new(
            project,
            project_context.clone(),
            context_server_registry,
            templates,
            Some(model.clone()),
            cx,
        )
    });
    ThreadTest {
        model,
        thread,
        project_context,
        context_server_store,
        fs,
    }
}

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}

fn watch_settings(fs: Arc<dyn Fs>, cx: &mut App) {
    let fs = fs.clone();
    cx.spawn({
        async move |cx| {
            let mut new_settings_content_rx = settings::watch_config_file(
                cx.background_executor(),
                fs,
                paths::settings_file().clone(),
            );

            while let Some(new_settings_content) = new_settings_content_rx.next().await {
                cx.update(|cx| {
                    SettingsStore::update_global(cx, |settings, cx| {
                        settings.set_user_settings(&new_settings_content, cx)
                    })
                })
                .ok();
            }
        }
    })
    .detach();
}

fn tool_names_for_completion(completion: &LanguageModelRequest) -> Vec<String> {
    completion
        .tools
        .iter()
        .map(|tool| tool.name.clone())
        .collect()
}

fn setup_context_server(
    name: &'static str,
    tools: Vec<context_server::types::Tool>,
    context_server_store: &Entity<ContextServerStore>,
    cx: &mut TestAppContext,
) -> mpsc::UnboundedReceiver<(
    context_server::types::CallToolParams,
    oneshot::Sender<context_server::types::CallToolResponse>,
)> {
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

    let (mcp_tool_calls_tx, mcp_tool_calls_rx) = mpsc::unbounded();
    let fake_transport = context_server::test::create_fake_transport(name, cx.executor())
        .on_request::<context_server::types::requests::Initialize, _>(move |_params| async move {
            context_server::types::InitializeResponse {
                protocol_version: context_server::types::ProtocolVersion(
                    context_server::types::LATEST_PROTOCOL_VERSION.to_string(),
                ),
                server_info: context_server::types::Implementation {
                    name: name.into(),
                    version: "1.0.0".to_string(),
                },
                capabilities: context_server::types::ServerCapabilities {
                    tools: Some(context_server::types::ToolsCapabilities {
                        list_changed: Some(true),
                    }),
                    ..Default::default()
                },
                meta: None,
            }
        })
        .on_request::<context_server::types::requests::ListTools, _>(move |_params| {
            let tools = tools.clone();
            async move {
                context_server::types::ListToolsResponse {
                    tools,
                    next_cursor: None,
                    meta: None,
                }
            }
        })
        .on_request::<context_server::types::requests::CallTool, _>(move |params| {
            let mcp_tool_calls_tx = mcp_tool_calls_tx.clone();
            async move {
                let (response_tx, response_rx) = oneshot::channel();
                mcp_tool_calls_tx
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
    mcp_tool_calls_rx
}

#[gpui::test]
async fn test_tokens_before_message(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    // First message
    let message_1_id = UserMessageId::new();
    thread
        .update(cx, |thread, cx| {
            thread.send(message_1_id.clone(), ["First message"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    // Before any response, tokens_before_message should return None for first message
    thread.read_with(cx, |thread, _| {
        assert_eq!(
            thread.tokens_before_message(&message_1_id),
            None,
            "First message should have no tokens before it"
        );
    });

    // Complete first message with usage
    fake_model.send_last_completion_stream_text_chunk("Response 1");
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::UsageUpdate(
        language_model::TokenUsage {
            input_tokens: 100,
            output_tokens: 50,
            cache_creation_input_tokens: 0,
            cache_read_input_tokens: 0,
        },
    ));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // First message still has no tokens before it
    thread.read_with(cx, |thread, _| {
        assert_eq!(
            thread.tokens_before_message(&message_1_id),
            None,
            "First message should still have no tokens before it after response"
        );
    });

    // Second message
    let message_2_id = UserMessageId::new();
    thread
        .update(cx, |thread, cx| {
            thread.send(message_2_id.clone(), ["Second message"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    // Second message should have first message's input tokens before it
    thread.read_with(cx, |thread, _| {
        assert_eq!(
            thread.tokens_before_message(&message_2_id),
            Some(100),
            "Second message should have 100 tokens before it (from first request)"
        );
    });

    // Complete second message
    fake_model.send_last_completion_stream_text_chunk("Response 2");
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::UsageUpdate(
        language_model::TokenUsage {
            input_tokens: 250, // Total for this request (includes previous context)
            output_tokens: 75,
            cache_creation_input_tokens: 0,
            cache_read_input_tokens: 0,
        },
    ));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // Third message
    let message_3_id = UserMessageId::new();
    thread
        .update(cx, |thread, cx| {
            thread.send(message_3_id.clone(), ["Third message"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    // Third message should have second message's input tokens (250) before it
    thread.read_with(cx, |thread, _| {
        assert_eq!(
            thread.tokens_before_message(&message_3_id),
            Some(250),
            "Third message should have 250 tokens before it (from second request)"
        );
        // Second message should still have 100
        assert_eq!(
            thread.tokens_before_message(&message_2_id),
            Some(100),
            "Second message should still have 100 tokens before it"
        );
        // First message still has none
        assert_eq!(
            thread.tokens_before_message(&message_1_id),
            None,
            "First message should still have no tokens before it"
        );
    });
}

#[gpui::test]
async fn test_tokens_before_message_after_truncate(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    // Set up three messages with responses
    let message_1_id = UserMessageId::new();
    thread
        .update(cx, |thread, cx| {
            thread.send(message_1_id.clone(), ["Message 1"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    fake_model.send_last_completion_stream_text_chunk("Response 1");
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::UsageUpdate(
        language_model::TokenUsage {
            input_tokens: 100,
            output_tokens: 50,
            cache_creation_input_tokens: 0,
            cache_read_input_tokens: 0,
        },
    ));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let message_2_id = UserMessageId::new();
    thread
        .update(cx, |thread, cx| {
            thread.send(message_2_id.clone(), ["Message 2"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    fake_model.send_last_completion_stream_text_chunk("Response 2");
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::UsageUpdate(
        language_model::TokenUsage {
            input_tokens: 250,
            output_tokens: 75,
            cache_creation_input_tokens: 0,
            cache_read_input_tokens: 0,
        },
    ));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // Verify initial state
    thread.read_with(cx, |thread, _| {
        assert_eq!(thread.tokens_before_message(&message_2_id), Some(100));
    });

    // Truncate at message 2 (removes message 2 and everything after)
    thread
        .update(cx, |thread, cx| thread.truncate(message_2_id.clone(), cx))
        .unwrap();
    cx.run_until_parked();

    // After truncation, message_2_id no longer exists, so lookup should return None
    thread.read_with(cx, |thread, _| {
        assert_eq!(
            thread.tokens_before_message(&message_2_id),
            None,
            "After truncation, message 2 no longer exists"
        );
        // Message 1 still exists but has no tokens before it
        assert_eq!(
            thread.tokens_before_message(&message_1_id),
            None,
            "First message still has no tokens before it"
        );
    });
}

#[gpui::test]
async fn test_terminal_tool_permission_rules(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/root", json!({})).await;
    let project = Project::test(fs, ["/root".as_ref()], cx).await;

    // Test 1: Deny rule blocks command
    {
        let handle = Rc::new(cx.update(|cx| FakeTerminalHandle::new_never_exits(cx)));
        let environment = Rc::new(FakeThreadEnvironment {
            handle: handle.clone(),
        });

        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.tool_permissions.tools.insert(
                "terminal".into(),
                agent_settings::ToolRules {
                    default_mode: settings::ToolPermissionMode::Confirm,
                    always_allow: vec![],
                    always_deny: vec![
                        agent_settings::CompiledRegex::new(r"rm\s+-rf", false).unwrap(),
                    ],
                    always_confirm: vec![],
                    invalid_patterns: vec![],
                },
            );
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        #[allow(clippy::arc_with_non_send_sync)]
        let tool = Arc::new(crate::TerminalTool::new(project.clone(), environment));
        let (event_stream, _rx) = crate::ToolCallEventStream::test();

        let task = cx.update(|cx| {
            tool.run(
                crate::TerminalToolInput {
                    command: "rm -rf /".to_string(),
                    cd: ".".to_string(),
                    timeout_ms: None,
                },
                event_stream,
                cx,
            )
        });

        let result = task.await;
        assert!(
            result.is_err(),
            "expected command to be blocked by deny rule"
        );
        assert!(
            result.unwrap_err().to_string().contains("blocked"),
            "error should mention the command was blocked"
        );
    }

    // Test 2: Allow rule skips confirmation (and overrides default_mode: Deny)
    {
        let handle = Rc::new(cx.update(|cx| FakeTerminalHandle::new_with_immediate_exit(cx, 0)));
        let environment = Rc::new(FakeThreadEnvironment {
            handle: handle.clone(),
        });

        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.always_allow_tool_actions = false;
            settings.tool_permissions.tools.insert(
                "terminal".into(),
                agent_settings::ToolRules {
                    default_mode: settings::ToolPermissionMode::Deny,
                    always_allow: vec![
                        agent_settings::CompiledRegex::new(r"^echo\s", false).unwrap(),
                    ],
                    always_deny: vec![],
                    always_confirm: vec![],
                    invalid_patterns: vec![],
                },
            );
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        #[allow(clippy::arc_with_non_send_sync)]
        let tool = Arc::new(crate::TerminalTool::new(project.clone(), environment));
        let (event_stream, mut rx) = crate::ToolCallEventStream::test();

        let task = cx.update(|cx| {
            tool.run(
                crate::TerminalToolInput {
                    command: "echo hello".to_string(),
                    cd: ".".to_string(),
                    timeout_ms: None,
                },
                event_stream,
                cx,
            )
        });

        let update = rx.expect_update_fields().await;
        assert!(
            update.content.iter().any(|blocks| {
                blocks
                    .iter()
                    .any(|c| matches!(c, acp::ToolCallContent::Terminal(_)))
            }),
            "expected terminal content (allow rule should skip confirmation and override default deny)"
        );

        let result = task.await;
        assert!(
            result.is_ok(),
            "expected command to succeed without confirmation"
        );
    }

    // Test 3: always_allow_tool_actions=true overrides always_confirm patterns
    {
        let handle = Rc::new(cx.update(|cx| FakeTerminalHandle::new_with_immediate_exit(cx, 0)));
        let environment = Rc::new(FakeThreadEnvironment {
            handle: handle.clone(),
        });

        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.always_allow_tool_actions = true;
            settings.tool_permissions.tools.insert(
                "terminal".into(),
                agent_settings::ToolRules {
                    default_mode: settings::ToolPermissionMode::Allow,
                    always_allow: vec![],
                    always_deny: vec![],
                    always_confirm: vec![
                        agent_settings::CompiledRegex::new(r"sudo", false).unwrap(),
                    ],
                    invalid_patterns: vec![],
                },
            );
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        #[allow(clippy::arc_with_non_send_sync)]
        let tool = Arc::new(crate::TerminalTool::new(project.clone(), environment));
        let (event_stream, _rx) = crate::ToolCallEventStream::test();

        let task = cx.update(|cx| {
            tool.run(
                crate::TerminalToolInput {
                    command: "sudo rm file".to_string(),
                    cd: ".".to_string(),
                    timeout_ms: None,
                },
                event_stream,
                cx,
            )
        });

        // With always_allow_tool_actions=true, confirm patterns are overridden
        task.await
            .expect("command should be allowed with always_allow_tool_actions=true");
    }

    // Test 4: always_allow_tool_actions=true overrides default_mode: Deny
    {
        let handle = Rc::new(cx.update(|cx| FakeTerminalHandle::new_with_immediate_exit(cx, 0)));
        let environment = Rc::new(FakeThreadEnvironment {
            handle: handle.clone(),
        });

        cx.update(|cx| {
            let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
            settings.always_allow_tool_actions = true;
            settings.tool_permissions.tools.insert(
                "terminal".into(),
                agent_settings::ToolRules {
                    default_mode: settings::ToolPermissionMode::Deny,
                    always_allow: vec![],
                    always_deny: vec![],
                    always_confirm: vec![],
                    invalid_patterns: vec![],
                },
            );
            agent_settings::AgentSettings::override_global(settings, cx);
        });

        #[allow(clippy::arc_with_non_send_sync)]
        let tool = Arc::new(crate::TerminalTool::new(project.clone(), environment));
        let (event_stream, _rx) = crate::ToolCallEventStream::test();

        let task = cx.update(|cx| {
            tool.run(
                crate::TerminalToolInput {
                    command: "echo hello".to_string(),
                    cd: ".".to_string(),
                    timeout_ms: None,
                },
                event_stream,
                cx,
            )
        });

        // With always_allow_tool_actions=true, even default_mode: Deny is overridden
        task.await
            .expect("command should be allowed with always_allow_tool_actions=true");
    }
}

#[gpui::test]
async fn test_subagent_tool_is_present_when_feature_flag_enabled(cx: &mut TestAppContext) {
    init_test(cx);

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/test"), json!({})).await;
    let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));
    let model = Arc::new(FakeLanguageModel::default());

    let handle = Rc::new(cx.update(|cx| FakeTerminalHandle::new_never_exits(cx)));
    let environment = Rc::new(FakeThreadEnvironment { handle });

    let thread = cx.new(|cx| {
        let mut thread = Thread::new(
            project.clone(),
            project_context,
            context_server_registry,
            Templates::new(),
            Some(model),
            cx,
        );
        thread.add_default_tools(environment, cx);
        thread
    });

    thread.read_with(cx, |thread, _| {
        assert!(
            thread.has_registered_tool("subagent"),
            "subagent tool should be present when feature flag is enabled"
        );
    });
}

#[gpui::test]
async fn test_subagent_thread_inherits_parent_model(cx: &mut TestAppContext) {
    init_test(cx);

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/test"), json!({})).await;
    let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));
    let model = Arc::new(FakeLanguageModel::default());

    let subagent_context = SubagentContext {
        parent_thread_id: agent_client_protocol::SessionId::new("parent-id"),
        tool_use_id: language_model::LanguageModelToolUseId::from("tool-use-id"),
        depth: 1,
        summary_prompt: "Summarize".to_string(),
        context_low_prompt: "Context low".to_string(),
    };

    let subagent = cx.new(|cx| {
        Thread::new_subagent(
            project.clone(),
            project_context,
            context_server_registry,
            Templates::new(),
            model.clone(),
            subagent_context,
            std::collections::BTreeMap::new(),
            cx,
        )
    });

    subagent.read_with(cx, |thread, _| {
        assert!(thread.is_subagent());
        assert_eq!(thread.depth(), 1);
        assert!(thread.model().is_some());
    });
}

#[gpui::test]
async fn test_max_subagent_depth_prevents_tool_registration(cx: &mut TestAppContext) {
    init_test(cx);

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/test"), json!({})).await;
    let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));
    let model = Arc::new(FakeLanguageModel::default());

    let subagent_context = SubagentContext {
        parent_thread_id: agent_client_protocol::SessionId::new("parent-id"),
        tool_use_id: language_model::LanguageModelToolUseId::from("tool-use-id"),
        depth: MAX_SUBAGENT_DEPTH,
        summary_prompt: "Summarize".to_string(),
        context_low_prompt: "Context low".to_string(),
    };

    let handle = Rc::new(cx.update(|cx| FakeTerminalHandle::new_never_exits(cx)));
    let environment = Rc::new(FakeThreadEnvironment { handle });

    let deep_subagent = cx.new(|cx| {
        let mut thread = Thread::new_subagent(
            project.clone(),
            project_context,
            context_server_registry,
            Templates::new(),
            model.clone(),
            subagent_context,
            std::collections::BTreeMap::new(),
            cx,
        );
        thread.add_default_tools(environment, cx);
        thread
    });

    deep_subagent.read_with(cx, |thread, _| {
        assert_eq!(thread.depth(), MAX_SUBAGENT_DEPTH);
        assert!(
            !thread.has_registered_tool("subagent"),
            "subagent tool should not be present at max depth"
        );
    });
}

#[gpui::test]
async fn test_subagent_receives_task_prompt(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let subagent_context = SubagentContext {
        parent_thread_id: agent_client_protocol::SessionId::new("parent-id"),
        tool_use_id: language_model::LanguageModelToolUseId::from("tool-use-id"),
        depth: 1,
        summary_prompt: "Summarize your work".to_string(),
        context_low_prompt: "Context low, wrap up".to_string(),
    };

    let project = thread.read_with(cx, |t, _| t.project.clone());
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));

    let subagent = cx.new(|cx| {
        Thread::new_subagent(
            project.clone(),
            project_context,
            context_server_registry,
            Templates::new(),
            model.clone(),
            subagent_context,
            std::collections::BTreeMap::new(),
            cx,
        )
    });

    let task_prompt = "Find all TODO comments in the codebase";
    subagent
        .update(cx, |thread, cx| thread.submit_user_message(task_prompt, cx))
        .unwrap();
    cx.run_until_parked();

    let pending = fake_model.pending_completions();
    assert_eq!(pending.len(), 1, "should have one pending completion");

    let messages = &pending[0].messages;
    let user_messages: Vec<_> = messages
        .iter()
        .filter(|m| m.role == language_model::Role::User)
        .collect();
    assert_eq!(user_messages.len(), 1, "should have one user message");

    let content = &user_messages[0].content[0];
    assert!(
        content.to_str().unwrap().contains("TODO"),
        "task prompt should be in user message"
    );
}

#[gpui::test]
async fn test_subagent_returns_summary_on_completion(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let subagent_context = SubagentContext {
        parent_thread_id: agent_client_protocol::SessionId::new("parent-id"),
        tool_use_id: language_model::LanguageModelToolUseId::from("tool-use-id"),
        depth: 1,
        summary_prompt: "Please summarize what you found".to_string(),
        context_low_prompt: "Context low, wrap up".to_string(),
    };

    let project = thread.read_with(cx, |t, _| t.project.clone());
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));

    let subagent = cx.new(|cx| {
        Thread::new_subagent(
            project.clone(),
            project_context,
            context_server_registry,
            Templates::new(),
            model.clone(),
            subagent_context,
            std::collections::BTreeMap::new(),
            cx,
        )
    });

    subagent
        .update(cx, |thread, cx| {
            thread.submit_user_message("Do some work", cx)
        })
        .unwrap();
    cx.run_until_parked();

    fake_model.send_last_completion_stream_text_chunk("I did the work");
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    subagent
        .update(cx, |thread, cx| thread.request_final_summary(cx))
        .unwrap();
    cx.run_until_parked();

    let pending = fake_model.pending_completions();
    assert!(
        !pending.is_empty(),
        "should have pending completion for summary"
    );

    let messages = &pending.last().unwrap().messages;
    let user_messages: Vec<_> = messages
        .iter()
        .filter(|m| m.role == language_model::Role::User)
        .collect();

    let last_user = user_messages.last().unwrap();
    assert!(
        last_user.content[0].to_str().unwrap().contains("summarize"),
        "summary prompt should be sent"
    );
}

#[gpui::test]
async fn test_allowed_tools_restricts_subagent_capabilities(cx: &mut TestAppContext) {
    init_test(cx);

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/test"), json!({})).await;
    let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));
    let model = Arc::new(FakeLanguageModel::default());

    let subagent_context = SubagentContext {
        parent_thread_id: agent_client_protocol::SessionId::new("parent-id"),
        tool_use_id: language_model::LanguageModelToolUseId::from("tool-use-id"),
        depth: 1,
        summary_prompt: "Summarize".to_string(),
        context_low_prompt: "Context low".to_string(),
    };

    let subagent = cx.new(|cx| {
        let mut thread = Thread::new_subagent(
            project.clone(),
            project_context,
            context_server_registry,
            Templates::new(),
            model.clone(),
            subagent_context,
            std::collections::BTreeMap::new(),
            cx,
        );
        thread.add_tool(EchoTool);
        thread.add_tool(DelayTool);
        thread.add_tool(WordListTool);
        thread
    });

    subagent.read_with(cx, |thread, _| {
        assert!(thread.has_registered_tool("echo"));
        assert!(thread.has_registered_tool("delay"));
        assert!(thread.has_registered_tool("word_list"));
    });

    let allowed: collections::HashSet<gpui::SharedString> =
        vec!["echo".into()].into_iter().collect();

    subagent.update(cx, |thread, _cx| {
        thread.restrict_tools(&allowed);
    });

    subagent.read_with(cx, |thread, _| {
        assert!(
            thread.has_registered_tool("echo"),
            "echo should still be available"
        );
        assert!(
            !thread.has_registered_tool("delay"),
            "delay should be removed"
        );
        assert!(
            !thread.has_registered_tool("word_list"),
            "word_list should be removed"
        );
    });
}

#[gpui::test]
async fn test_parent_cancel_stops_subagent(cx: &mut TestAppContext) {
    init_test(cx);

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/test"), json!({})).await;
    let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));
    let model = Arc::new(FakeLanguageModel::default());

    let parent = cx.new(|cx| {
        Thread::new(
            project.clone(),
            project_context.clone(),
            context_server_registry.clone(),
            Templates::new(),
            Some(model.clone()),
            cx,
        )
    });

    let subagent_context = SubagentContext {
        parent_thread_id: agent_client_protocol::SessionId::new("parent-id"),
        tool_use_id: language_model::LanguageModelToolUseId::from("tool-use-id"),
        depth: 1,
        summary_prompt: "Summarize".to_string(),
        context_low_prompt: "Context low".to_string(),
    };

    let subagent = cx.new(|cx| {
        Thread::new_subagent(
            project.clone(),
            project_context.clone(),
            context_server_registry.clone(),
            Templates::new(),
            model.clone(),
            subagent_context,
            std::collections::BTreeMap::new(),
            cx,
        )
    });

    parent.update(cx, |thread, _cx| {
        thread.register_running_subagent(subagent.downgrade());
    });

    subagent
        .update(cx, |thread, cx| thread.submit_user_message("Do work", cx))
        .unwrap();
    cx.run_until_parked();

    subagent.read_with(cx, |thread, _| {
        assert!(!thread.is_turn_complete(), "subagent should be running");
    });

    parent.update(cx, |thread, cx| {
        thread.cancel(cx).detach();
    });

    subagent.read_with(cx, |thread, _| {
        assert!(
            thread.is_turn_complete(),
            "subagent should be cancelled when parent cancels"
        );
    });
}

#[gpui::test]
async fn test_subagent_tool_cancellation(cx: &mut TestAppContext) {
    // This test verifies that the subagent tool properly handles user cancellation
    // via `event_stream.cancelled_by_user()` and stops all running subagents.
    init_test(cx);
    always_allow_tools(cx);

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/test"), json!({})).await;
    let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));
    let model = Arc::new(FakeLanguageModel::default());

    let parent = cx.new(|cx| {
        Thread::new(
            project.clone(),
            project_context.clone(),
            context_server_registry.clone(),
            Templates::new(),
            Some(model.clone()),
            cx,
        )
    });

    let parent_tools: std::collections::BTreeMap<gpui::SharedString, Arc<dyn crate::AnyAgentTool>> =
        std::collections::BTreeMap::new();

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(SubagentTool::new(
        parent.downgrade(),
        project.clone(),
        project_context,
        context_server_registry,
        Templates::new(),
        0,
        parent_tools,
    ));

    let (event_stream, _rx, mut cancellation_tx) =
        crate::ToolCallEventStream::test_with_cancellation();

    // Start the subagent tool
    let task = cx.update(|cx| {
        tool.run(
            SubagentToolInput {
                subagents: vec![crate::SubagentConfig {
                    label: "Long running task".to_string(),
                    task_prompt: "Do a very long task that takes forever".to_string(),
                    summary_prompt: "Summarize".to_string(),
                    context_low_prompt: "Context low".to_string(),
                    timeout_ms: None,
                    allowed_tools: None,
                }],
            },
            event_stream.clone(),
            cx,
        )
    });

    cx.run_until_parked();

    // Signal cancellation via the event stream
    crate::ToolCallEventStream::signal_cancellation_with_sender(&mut cancellation_tx);

    // The task should complete promptly with a cancellation error
    let timeout = cx.background_executor.timer(Duration::from_secs(5));
    let result = futures::select! {
        result = task.fuse() => result,
        _ = timeout.fuse() => {
            panic!("subagent tool did not respond to cancellation within timeout");
        }
    };

    // Verify we got a cancellation error
    let err = result.unwrap_err();
    assert!(
        err.to_string().contains("cancelled by user"),
        "expected cancellation error, got: {}",
        err
    );
}

#[gpui::test]
async fn test_subagent_model_error_returned_as_tool_error(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let subagent_context = SubagentContext {
        parent_thread_id: agent_client_protocol::SessionId::new("parent-id"),
        tool_use_id: language_model::LanguageModelToolUseId::from("tool-use-id"),
        depth: 1,
        summary_prompt: "Summarize".to_string(),
        context_low_prompt: "Context low".to_string(),
    };

    let project = thread.read_with(cx, |t, _| t.project.clone());
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));

    let subagent = cx.new(|cx| {
        Thread::new_subagent(
            project.clone(),
            project_context,
            context_server_registry,
            Templates::new(),
            model.clone(),
            subagent_context,
            std::collections::BTreeMap::new(),
            cx,
        )
    });

    subagent
        .update(cx, |thread, cx| thread.submit_user_message("Do work", cx))
        .unwrap();
    cx.run_until_parked();

    subagent.read_with(cx, |thread, _| {
        assert!(!thread.is_turn_complete(), "turn should be in progress");
    });

    fake_model.send_last_completion_stream_error(LanguageModelCompletionError::NoApiKey {
        provider: LanguageModelProviderName::from("Fake".to_string()),
    });
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    subagent.read_with(cx, |thread, _| {
        assert!(
            thread.is_turn_complete(),
            "turn should be complete after non-retryable error"
        );
    });
}

#[gpui::test]
async fn test_subagent_timeout_triggers_early_summary(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let subagent_context = SubagentContext {
        parent_thread_id: agent_client_protocol::SessionId::new("parent-id"),
        tool_use_id: language_model::LanguageModelToolUseId::from("tool-use-id"),
        depth: 1,
        summary_prompt: "Summarize your work".to_string(),
        context_low_prompt: "Context low, stop and summarize".to_string(),
    };

    let project = thread.read_with(cx, |t, _| t.project.clone());
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));

    let subagent = cx.new(|cx| {
        Thread::new_subagent(
            project.clone(),
            project_context.clone(),
            context_server_registry.clone(),
            Templates::new(),
            model.clone(),
            subagent_context.clone(),
            std::collections::BTreeMap::new(),
            cx,
        )
    });

    subagent.update(cx, |thread, _| {
        thread.add_tool(EchoTool);
    });

    subagent
        .update(cx, |thread, cx| {
            thread.submit_user_message("Do some work", cx)
        })
        .unwrap();
    cx.run_until_parked();

    fake_model.send_last_completion_stream_text_chunk("Working on it...");
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let interrupt_result = subagent.update(cx, |thread, cx| thread.interrupt_for_summary(cx));
    assert!(
        interrupt_result.is_ok(),
        "interrupt_for_summary should succeed"
    );

    cx.run_until_parked();

    let pending = fake_model.pending_completions();
    assert!(
        !pending.is_empty(),
        "should have pending completion for interrupted summary"
    );

    let messages = &pending.last().unwrap().messages;
    let user_messages: Vec<_> = messages
        .iter()
        .filter(|m| m.role == language_model::Role::User)
        .collect();

    let last_user = user_messages.last().unwrap();
    let content_str = last_user.content[0].to_str().unwrap();
    assert!(
        content_str.contains("Context low") || content_str.contains("stop and summarize"),
        "context_low_prompt should be sent when interrupting: got {:?}",
        content_str
    );
}

#[gpui::test]
async fn test_context_low_check_returns_true_when_usage_high(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let subagent_context = SubagentContext {
        parent_thread_id: agent_client_protocol::SessionId::new("parent-id"),
        tool_use_id: language_model::LanguageModelToolUseId::from("tool-use-id"),
        depth: 1,
        summary_prompt: "Summarize".to_string(),
        context_low_prompt: "Context low".to_string(),
    };

    let project = thread.read_with(cx, |t, _| t.project.clone());
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));

    let subagent = cx.new(|cx| {
        Thread::new_subagent(
            project.clone(),
            project_context,
            context_server_registry,
            Templates::new(),
            model.clone(),
            subagent_context,
            std::collections::BTreeMap::new(),
            cx,
        )
    });

    subagent
        .update(cx, |thread, cx| thread.submit_user_message("Do work", cx))
        .unwrap();
    cx.run_until_parked();

    let max_tokens = model.max_token_count();
    let high_usage = language_model::TokenUsage {
        input_tokens: (max_tokens as f64 * 0.80) as u64,
        output_tokens: 0,
        cache_creation_input_tokens: 0,
        cache_read_input_tokens: 0,
    };

    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::UsageUpdate(high_usage));
    fake_model.send_last_completion_stream_text_chunk("Working...");
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let usage = subagent.read_with(cx, |thread, _| thread.latest_token_usage());
    assert!(usage.is_some(), "should have token usage after completion");

    let usage = usage.unwrap();
    let remaining_ratio = 1.0 - (usage.used_tokens as f32 / usage.max_tokens as f32);
    assert!(
        remaining_ratio <= 0.25,
        "remaining ratio should be at or below 25% (got {}%), indicating context is low",
        remaining_ratio * 100.0
    );
}

#[gpui::test]
async fn test_allowed_tools_rejects_unknown_tool(cx: &mut TestAppContext) {
    init_test(cx);

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/test"), json!({})).await;
    let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));
    let model = Arc::new(FakeLanguageModel::default());

    let parent = cx.new(|cx| {
        let mut thread = Thread::new(
            project.clone(),
            project_context.clone(),
            context_server_registry.clone(),
            Templates::new(),
            Some(model.clone()),
            cx,
        );
        thread.add_tool(EchoTool);
        thread
    });

    let mut parent_tools: std::collections::BTreeMap<
        gpui::SharedString,
        Arc<dyn crate::AnyAgentTool>,
    > = std::collections::BTreeMap::new();
    parent_tools.insert("echo".into(), EchoTool.erase());

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(SubagentTool::new(
        parent.downgrade(),
        project,
        project_context,
        context_server_registry,
        Templates::new(),
        0,
        parent_tools,
    ));

    let subagent_configs = vec![crate::SubagentConfig {
        label: "Test".to_string(),
        task_prompt: "Do something".to_string(),
        summary_prompt: "Summarize".to_string(),
        context_low_prompt: "Context low".to_string(),
        timeout_ms: None,
        allowed_tools: Some(vec!["nonexistent_tool".to_string()]),
    }];
    let result = tool.validate_subagents(&subagent_configs);
    assert!(result.is_err(), "should reject unknown tool");
    let err_msg = result.unwrap_err().to_string();
    assert!(
        err_msg.contains("nonexistent_tool"),
        "error should mention the invalid tool name: {}",
        err_msg
    );
    assert!(
        err_msg.contains("do not exist"),
        "error should explain the tool does not exist: {}",
        err_msg
    );
}

#[gpui::test]
async fn test_subagent_empty_response_handled(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let subagent_context = SubagentContext {
        parent_thread_id: agent_client_protocol::SessionId::new("parent-id"),
        tool_use_id: language_model::LanguageModelToolUseId::from("tool-use-id"),
        depth: 1,
        summary_prompt: "Summarize".to_string(),
        context_low_prompt: "Context low".to_string(),
    };

    let project = thread.read_with(cx, |t, _| t.project.clone());
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));

    let subagent = cx.new(|cx| {
        Thread::new_subagent(
            project.clone(),
            project_context,
            context_server_registry,
            Templates::new(),
            model.clone(),
            subagent_context,
            std::collections::BTreeMap::new(),
            cx,
        )
    });

    subagent
        .update(cx, |thread, cx| thread.submit_user_message("Do work", cx))
        .unwrap();
    cx.run_until_parked();

    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    subagent.read_with(cx, |thread, _| {
        assert!(
            thread.is_turn_complete(),
            "turn should complete even with empty response"
        );
    });
}

#[gpui::test]
async fn test_nested_subagent_at_depth_2_succeeds(cx: &mut TestAppContext) {
    init_test(cx);

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/test"), json!({})).await;
    let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));
    let model = Arc::new(FakeLanguageModel::default());

    let depth_1_context = SubagentContext {
        parent_thread_id: agent_client_protocol::SessionId::new("root-id"),
        tool_use_id: language_model::LanguageModelToolUseId::from("tool-use-1"),
        depth: 1,
        summary_prompt: "Summarize".to_string(),
        context_low_prompt: "Context low".to_string(),
    };

    let depth_1_subagent = cx.new(|cx| {
        Thread::new_subagent(
            project.clone(),
            project_context.clone(),
            context_server_registry.clone(),
            Templates::new(),
            model.clone(),
            depth_1_context,
            std::collections::BTreeMap::new(),
            cx,
        )
    });

    depth_1_subagent.read_with(cx, |thread, _| {
        assert_eq!(thread.depth(), 1);
        assert!(thread.is_subagent());
    });

    let depth_2_context = SubagentContext {
        parent_thread_id: agent_client_protocol::SessionId::new("depth-1-id"),
        tool_use_id: language_model::LanguageModelToolUseId::from("tool-use-2"),
        depth: 2,
        summary_prompt: "Summarize depth 2".to_string(),
        context_low_prompt: "Context low depth 2".to_string(),
    };

    let depth_2_subagent = cx.new(|cx| {
        Thread::new_subagent(
            project.clone(),
            project_context.clone(),
            context_server_registry.clone(),
            Templates::new(),
            model.clone(),
            depth_2_context,
            std::collections::BTreeMap::new(),
            cx,
        )
    });

    depth_2_subagent.read_with(cx, |thread, _| {
        assert_eq!(thread.depth(), 2);
        assert!(thread.is_subagent());
    });

    depth_2_subagent
        .update(cx, |thread, cx| {
            thread.submit_user_message("Nested task", cx)
        })
        .unwrap();
    cx.run_until_parked();

    let pending = model.as_fake().pending_completions();
    assert!(
        !pending.is_empty(),
        "depth-2 subagent should be able to submit messages"
    );
}

#[gpui::test]
async fn test_subagent_uses_tool_and_returns_result(cx: &mut TestAppContext) {
    init_test(cx);
    always_allow_tools(cx);

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/test"), json!({})).await;
    let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));
    let model = Arc::new(FakeLanguageModel::default());
    let fake_model = model.as_fake();

    let subagent_context = SubagentContext {
        parent_thread_id: agent_client_protocol::SessionId::new("parent-id"),
        tool_use_id: language_model::LanguageModelToolUseId::from("tool-use-id"),
        depth: 1,
        summary_prompt: "Summarize what you did".to_string(),
        context_low_prompt: "Context low".to_string(),
    };

    let subagent = cx.new(|cx| {
        let mut thread = Thread::new_subagent(
            project.clone(),
            project_context,
            context_server_registry,
            Templates::new(),
            model.clone(),
            subagent_context,
            std::collections::BTreeMap::new(),
            cx,
        );
        thread.add_tool(EchoTool);
        thread
    });

    subagent.read_with(cx, |thread, _| {
        assert!(
            thread.has_registered_tool("echo"),
            "subagent should have echo tool"
        );
    });

    subagent
        .update(cx, |thread, cx| {
            thread.submit_user_message("Use the echo tool to echo 'hello world'", cx)
        })
        .unwrap();
    cx.run_until_parked();

    let tool_use = LanguageModelToolUse {
        id: "tool_call_1".into(),
        name: EchoTool::name().into(),
        raw_input: json!({"text": "hello world"}).to_string(),
        input: json!({"text": "hello world"}),
        is_input_complete: true,
        thought_signature: None,
    };
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(tool_use));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let pending = fake_model.pending_completions();
    assert!(
        !pending.is_empty(),
        "should have pending completion after tool use"
    );

    let last_completion = pending.last().unwrap();
    let has_tool_result = last_completion.messages.iter().any(|m| {
        m.content
            .iter()
            .any(|c| matches!(c, MessageContent::ToolResult(_)))
    });
    assert!(
        has_tool_result,
        "tool result should be in the messages sent back to the model"
    );
}

#[gpui::test]
async fn test_max_parallel_subagents_enforced(cx: &mut TestAppContext) {
    init_test(cx);

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/test"), json!({})).await;
    let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));
    let model = Arc::new(FakeLanguageModel::default());

    let parent = cx.new(|cx| {
        Thread::new(
            project.clone(),
            project_context.clone(),
            context_server_registry.clone(),
            Templates::new(),
            Some(model.clone()),
            cx,
        )
    });

    let mut subagents = Vec::new();
    for i in 0..MAX_PARALLEL_SUBAGENTS {
        let subagent_context = SubagentContext {
            parent_thread_id: agent_client_protocol::SessionId::new("parent-id"),
            tool_use_id: language_model::LanguageModelToolUseId::from(format!("tool-use-{}", i)),
            depth: 1,
            summary_prompt: "Summarize".to_string(),
            context_low_prompt: "Context low".to_string(),
        };

        let subagent = cx.new(|cx| {
            Thread::new_subagent(
                project.clone(),
                project_context.clone(),
                context_server_registry.clone(),
                Templates::new(),
                model.clone(),
                subagent_context,
                std::collections::BTreeMap::new(),
                cx,
            )
        });

        parent.update(cx, |thread, _cx| {
            thread.register_running_subagent(subagent.downgrade());
        });
        subagents.push(subagent);
    }

    parent.read_with(cx, |thread, _| {
        assert_eq!(
            thread.running_subagent_count(),
            MAX_PARALLEL_SUBAGENTS,
            "should have MAX_PARALLEL_SUBAGENTS registered"
        );
    });

    let parent_tools: std::collections::BTreeMap<gpui::SharedString, Arc<dyn crate::AnyAgentTool>> =
        std::collections::BTreeMap::new();

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(SubagentTool::new(
        parent.downgrade(),
        project.clone(),
        project_context,
        context_server_registry,
        Templates::new(),
        0,
        parent_tools,
    ));

    let (event_stream, _rx) = crate::ToolCallEventStream::test();

    let result = cx.update(|cx| {
        tool.run(
            SubagentToolInput {
                subagents: vec![crate::SubagentConfig {
                    label: "Test".to_string(),
                    task_prompt: "Do something".to_string(),
                    summary_prompt: "Summarize".to_string(),
                    context_low_prompt: "Context low".to_string(),
                    timeout_ms: None,
                    allowed_tools: None,
                }],
            },
            event_stream,
            cx,
        )
    });

    let err = result.await.unwrap_err();
    assert!(
        err.to_string().contains("Maximum parallel subagents"),
        "should reject when max parallel subagents reached: {}",
        err
    );

    drop(subagents);
}

#[gpui::test]
async fn test_subagent_tool_end_to_end(cx: &mut TestAppContext) {
    init_test(cx);
    always_allow_tools(cx);

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(path!("/test"), json!({})).await;
    let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(context_server_store.clone(), cx));
    let model = Arc::new(FakeLanguageModel::default());
    let fake_model = model.as_fake();

    let parent = cx.new(|cx| {
        let mut thread = Thread::new(
            project.clone(),
            project_context.clone(),
            context_server_registry.clone(),
            Templates::new(),
            Some(model.clone()),
            cx,
        );
        thread.add_tool(EchoTool);
        thread
    });

    let mut parent_tools: std::collections::BTreeMap<
        gpui::SharedString,
        Arc<dyn crate::AnyAgentTool>,
    > = std::collections::BTreeMap::new();
    parent_tools.insert("echo".into(), EchoTool.erase());

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(SubagentTool::new(
        parent.downgrade(),
        project.clone(),
        project_context,
        context_server_registry,
        Templates::new(),
        0,
        parent_tools,
    ));

    let (event_stream, _rx) = crate::ToolCallEventStream::test();

    let task = cx.update(|cx| {
        tool.run(
            SubagentToolInput {
                subagents: vec![crate::SubagentConfig {
                    label: "Research task".to_string(),
                    task_prompt: "Find all TODOs in the codebase".to_string(),
                    summary_prompt: "Summarize what you found".to_string(),
                    context_low_prompt: "Context low, wrap up".to_string(),
                    timeout_ms: None,
                    allowed_tools: None,
                }],
            },
            event_stream,
            cx,
        )
    });

    cx.run_until_parked();

    let pending = fake_model.pending_completions();
    assert!(
        !pending.is_empty(),
        "subagent should have started and sent a completion request"
    );

    let first_completion = &pending[0];
    let has_task_prompt = first_completion.messages.iter().any(|m| {
        m.role == language_model::Role::User
            && m.content
                .iter()
                .any(|c| c.to_str().map(|s| s.contains("TODO")).unwrap_or(false))
    });
    assert!(has_task_prompt, "task prompt should be sent to subagent");

    fake_model.send_last_completion_stream_text_chunk("I found 5 TODOs in the codebase.");
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let pending = fake_model.pending_completions();
    assert!(
        !pending.is_empty(),
        "should have pending completion for summary request"
    );

    let last_completion = pending.last().unwrap();
    let has_summary_prompt = last_completion.messages.iter().any(|m| {
        m.role == language_model::Role::User
            && m.content.iter().any(|c| {
                c.to_str()
                    .map(|s| s.contains("Summarize") || s.contains("summarize"))
                    .unwrap_or(false)
            })
    });
    assert!(
        has_summary_prompt,
        "summary prompt should be sent after task completion"
    );

    fake_model.send_last_completion_stream_text_chunk("Summary: Found 5 TODOs across 3 files.");
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let result = task.await;
    assert!(result.is_ok(), "subagent tool should complete successfully");

    let summary = result.unwrap();
    assert!(
        summary.contains("Summary") || summary.contains("TODO") || summary.contains("5"),
        "summary should contain subagent's response: {}",
        summary
    );
}

#[gpui::test]
async fn test_edit_file_tool_deny_rule_blocks_edit(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/root", json!({"sensitive_config.txt": "secret data"}))
        .await;
    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;

    cx.update(|cx| {
        let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
        settings.tool_permissions.tools.insert(
            "edit_file".into(),
            agent_settings::ToolRules {
                default_mode: settings::ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![agent_settings::CompiledRegex::new(r"sensitive", false).unwrap()],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        agent_settings::AgentSettings::override_global(settings, cx);
    });

    let context_server_registry =
        cx.new(|cx| crate::ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
    let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
    let templates = crate::Templates::new();
    let thread = cx.new(|cx| {
        crate::Thread::new(
            project.clone(),
            cx.new(|_cx| prompt_store::ProjectContext::default()),
            context_server_registry,
            templates.clone(),
            None,
            cx,
        )
    });

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(crate::EditFileTool::new(
        project.clone(),
        thread.downgrade(),
        language_registry,
        templates,
    ));
    let (event_stream, _rx) = crate::ToolCallEventStream::test();

    let task = cx.update(|cx| {
        tool.run(
            crate::EditFileToolInput {
                display_description: "Edit sensitive file".to_string(),
                path: "root/sensitive_config.txt".into(),
                mode: crate::EditFileMode::Edit,
            },
            event_stream,
            cx,
        )
    });

    let result = task.await;
    assert!(result.is_err(), "expected edit to be blocked");
    assert!(
        result.unwrap_err().to_string().contains("blocked"),
        "error should mention the edit was blocked"
    );
}

#[gpui::test]
async fn test_delete_path_tool_deny_rule_blocks_deletion(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/root", json!({"important_data.txt": "critical info"}))
        .await;
    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;

    cx.update(|cx| {
        let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
        settings.tool_permissions.tools.insert(
            "delete_path".into(),
            agent_settings::ToolRules {
                default_mode: settings::ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![agent_settings::CompiledRegex::new(r"important", false).unwrap()],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        agent_settings::AgentSettings::override_global(settings, cx);
    });

    let action_log = cx.new(|_cx| action_log::ActionLog::new(project.clone()));

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(crate::DeletePathTool::new(project, action_log));
    let (event_stream, _rx) = crate::ToolCallEventStream::test();

    let task = cx.update(|cx| {
        tool.run(
            crate::DeletePathToolInput {
                path: "root/important_data.txt".to_string(),
            },
            event_stream,
            cx,
        )
    });

    let result = task.await;
    assert!(result.is_err(), "expected deletion to be blocked");
    assert!(
        result.unwrap_err().to_string().contains("blocked"),
        "error should mention the deletion was blocked"
    );
}

#[gpui::test]
async fn test_move_path_tool_denies_if_destination_denied(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "safe.txt": "content",
            "protected": {}
        }),
    )
    .await;
    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;

    cx.update(|cx| {
        let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
        settings.tool_permissions.tools.insert(
            "move_path".into(),
            agent_settings::ToolRules {
                default_mode: settings::ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![agent_settings::CompiledRegex::new(r"protected", false).unwrap()],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        agent_settings::AgentSettings::override_global(settings, cx);
    });

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(crate::MovePathTool::new(project));
    let (event_stream, _rx) = crate::ToolCallEventStream::test();

    let task = cx.update(|cx| {
        tool.run(
            crate::MovePathToolInput {
                source_path: "root/safe.txt".to_string(),
                destination_path: "root/protected/safe.txt".to_string(),
            },
            event_stream,
            cx,
        )
    });

    let result = task.await;
    assert!(
        result.is_err(),
        "expected move to be blocked due to destination path"
    );
    assert!(
        result.unwrap_err().to_string().contains("blocked"),
        "error should mention the move was blocked"
    );
}

#[gpui::test]
async fn test_move_path_tool_denies_if_source_denied(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "secret.txt": "secret content",
            "public": {}
        }),
    )
    .await;
    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;

    cx.update(|cx| {
        let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
        settings.tool_permissions.tools.insert(
            "move_path".into(),
            agent_settings::ToolRules {
                default_mode: settings::ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![agent_settings::CompiledRegex::new(r"secret", false).unwrap()],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        agent_settings::AgentSettings::override_global(settings, cx);
    });

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(crate::MovePathTool::new(project));
    let (event_stream, _rx) = crate::ToolCallEventStream::test();

    let task = cx.update(|cx| {
        tool.run(
            crate::MovePathToolInput {
                source_path: "root/secret.txt".to_string(),
                destination_path: "root/public/not_secret.txt".to_string(),
            },
            event_stream,
            cx,
        )
    });

    let result = task.await;
    assert!(
        result.is_err(),
        "expected move to be blocked due to source path"
    );
    assert!(
        result.unwrap_err().to_string().contains("blocked"),
        "error should mention the move was blocked"
    );
}

#[gpui::test]
async fn test_copy_path_tool_deny_rule_blocks_copy(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "confidential.txt": "confidential data",
            "dest": {}
        }),
    )
    .await;
    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;

    cx.update(|cx| {
        let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
        settings.tool_permissions.tools.insert(
            "copy_path".into(),
            agent_settings::ToolRules {
                default_mode: settings::ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![
                    agent_settings::CompiledRegex::new(r"confidential", false).unwrap(),
                ],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        agent_settings::AgentSettings::override_global(settings, cx);
    });

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(crate::CopyPathTool::new(project));
    let (event_stream, _rx) = crate::ToolCallEventStream::test();

    let task = cx.update(|cx| {
        tool.run(
            crate::CopyPathToolInput {
                source_path: "root/confidential.txt".to_string(),
                destination_path: "root/dest/copy.txt".to_string(),
            },
            event_stream,
            cx,
        )
    });

    let result = task.await;
    assert!(result.is_err(), "expected copy to be blocked");
    assert!(
        result.unwrap_err().to_string().contains("blocked"),
        "error should mention the copy was blocked"
    );
}

#[gpui::test]
async fn test_save_file_tool_denies_if_any_path_denied(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        "/root",
        json!({
            "normal.txt": "normal content",
            "readonly": {
                "config.txt": "readonly content"
            }
        }),
    )
    .await;
    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;

    cx.update(|cx| {
        let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
        settings.tool_permissions.tools.insert(
            "save_file".into(),
            agent_settings::ToolRules {
                default_mode: settings::ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![agent_settings::CompiledRegex::new(r"readonly", false).unwrap()],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        agent_settings::AgentSettings::override_global(settings, cx);
    });

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(crate::SaveFileTool::new(project));
    let (event_stream, _rx) = crate::ToolCallEventStream::test();

    let task = cx.update(|cx| {
        tool.run(
            crate::SaveFileToolInput {
                paths: vec![
                    std::path::PathBuf::from("root/normal.txt"),
                    std::path::PathBuf::from("root/readonly/config.txt"),
                ],
            },
            event_stream,
            cx,
        )
    });

    let result = task.await;
    assert!(
        result.is_err(),
        "expected save to be blocked due to denied path"
    );
    assert!(
        result.unwrap_err().to_string().contains("blocked"),
        "error should mention the save was blocked"
    );
}

#[gpui::test]
async fn test_save_file_tool_respects_deny_rules(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/root", json!({"config.secret": "secret config"}))
        .await;
    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;

    cx.update(|cx| {
        let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
        settings.always_allow_tool_actions = false;
        settings.tool_permissions.tools.insert(
            "save_file".into(),
            agent_settings::ToolRules {
                default_mode: settings::ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![agent_settings::CompiledRegex::new(r"\.secret$", false).unwrap()],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        agent_settings::AgentSettings::override_global(settings, cx);
    });

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(crate::SaveFileTool::new(project));
    let (event_stream, _rx) = crate::ToolCallEventStream::test();

    let task = cx.update(|cx| {
        tool.run(
            crate::SaveFileToolInput {
                paths: vec![std::path::PathBuf::from("root/config.secret")],
            },
            event_stream,
            cx,
        )
    });

    let result = task.await;
    assert!(result.is_err(), "expected save to be blocked");
    assert!(
        result.unwrap_err().to_string().contains("blocked"),
        "error should mention the save was blocked"
    );
}

#[gpui::test]
async fn test_web_search_tool_deny_rule_blocks_search(cx: &mut TestAppContext) {
    init_test(cx);

    cx.update(|cx| {
        let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
        settings.tool_permissions.tools.insert(
            "web_search".into(),
            agent_settings::ToolRules {
                default_mode: settings::ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![
                    agent_settings::CompiledRegex::new(r"internal\.company", false).unwrap(),
                ],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        agent_settings::AgentSettings::override_global(settings, cx);
    });

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(crate::WebSearchTool);
    let (event_stream, _rx) = crate::ToolCallEventStream::test();

    let input: crate::WebSearchToolInput =
        serde_json::from_value(json!({"query": "internal.company.com secrets"})).unwrap();

    let task = cx.update(|cx| tool.run(input, event_stream, cx));

    let result = task.await;
    assert!(result.is_err(), "expected search to be blocked");
    assert!(
        result.unwrap_err().to_string().contains("blocked"),
        "error should mention the search was blocked"
    );
}

#[gpui::test]
async fn test_edit_file_tool_allow_rule_skips_confirmation(cx: &mut TestAppContext) {
    init_test(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree("/root", json!({"README.md": "# Hello"}))
        .await;
    let project = Project::test(fs.clone(), ["/root".as_ref()], cx).await;

    cx.update(|cx| {
        let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
        settings.always_allow_tool_actions = false;
        settings.tool_permissions.tools.insert(
            "edit_file".into(),
            agent_settings::ToolRules {
                default_mode: settings::ToolPermissionMode::Confirm,
                always_allow: vec![agent_settings::CompiledRegex::new(r"\.md$", false).unwrap()],
                always_deny: vec![],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        agent_settings::AgentSettings::override_global(settings, cx);
    });

    let context_server_registry =
        cx.new(|cx| crate::ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
    let language_registry = project.read_with(cx, |project, _cx| project.languages().clone());
    let templates = crate::Templates::new();
    let thread = cx.new(|cx| {
        crate::Thread::new(
            project.clone(),
            cx.new(|_cx| prompt_store::ProjectContext::default()),
            context_server_registry,
            templates.clone(),
            None,
            cx,
        )
    });

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(crate::EditFileTool::new(
        project,
        thread.downgrade(),
        language_registry,
        templates,
    ));
    let (event_stream, mut rx) = crate::ToolCallEventStream::test();

    let _task = cx.update(|cx| {
        tool.run(
            crate::EditFileToolInput {
                display_description: "Edit README".to_string(),
                path: "root/README.md".into(),
                mode: crate::EditFileMode::Edit,
            },
            event_stream,
            cx,
        )
    });

    cx.run_until_parked();

    let event = rx.try_next();
    assert!(
        !matches!(event, Ok(Some(Ok(ThreadEvent::ToolCallAuthorization(_))))),
        "expected no authorization request for allowed .md file"
    );
}

#[gpui::test]
async fn test_fetch_tool_deny_rule_blocks_url(cx: &mut TestAppContext) {
    init_test(cx);

    cx.update(|cx| {
        let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
        settings.tool_permissions.tools.insert(
            "fetch".into(),
            agent_settings::ToolRules {
                default_mode: settings::ToolPermissionMode::Allow,
                always_allow: vec![],
                always_deny: vec![
                    agent_settings::CompiledRegex::new(r"internal\.company\.com", false).unwrap(),
                ],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        agent_settings::AgentSettings::override_global(settings, cx);
    });

    let http_client = gpui::http_client::FakeHttpClient::with_200_response();

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(crate::FetchTool::new(http_client));
    let (event_stream, _rx) = crate::ToolCallEventStream::test();

    let input: crate::FetchToolInput =
        serde_json::from_value(json!({"url": "https://internal.company.com/api"})).unwrap();

    let task = cx.update(|cx| tool.run(input, event_stream, cx));

    let result = task.await;
    assert!(result.is_err(), "expected fetch to be blocked");
    assert!(
        result.unwrap_err().to_string().contains("blocked"),
        "error should mention the fetch was blocked"
    );
}

#[gpui::test]
async fn test_fetch_tool_allow_rule_skips_confirmation(cx: &mut TestAppContext) {
    init_test(cx);

    cx.update(|cx| {
        let mut settings = agent_settings::AgentSettings::get_global(cx).clone();
        settings.always_allow_tool_actions = false;
        settings.tool_permissions.tools.insert(
            "fetch".into(),
            agent_settings::ToolRules {
                default_mode: settings::ToolPermissionMode::Confirm,
                always_allow: vec![agent_settings::CompiledRegex::new(r"docs\.rs", false).unwrap()],
                always_deny: vec![],
                always_confirm: vec![],
                invalid_patterns: vec![],
            },
        );
        agent_settings::AgentSettings::override_global(settings, cx);
    });

    let http_client = gpui::http_client::FakeHttpClient::with_200_response();

    #[allow(clippy::arc_with_non_send_sync)]
    let tool = Arc::new(crate::FetchTool::new(http_client));
    let (event_stream, mut rx) = crate::ToolCallEventStream::test();

    let input: crate::FetchToolInput =
        serde_json::from_value(json!({"url": "https://docs.rs/some-crate"})).unwrap();

    let _task = cx.update(|cx| tool.run(input, event_stream, cx));

    cx.run_until_parked();

    let event = rx.try_next();
    assert!(
        !matches!(event, Ok(Some(Ok(ThreadEvent::ToolCallAuthorization(_))))),
        "expected no authorization request for allowed docs.rs URL"
    );
}

#[gpui::test]
async fn test_queued_message_ends_turn_at_boundary(cx: &mut TestAppContext) {
    init_test(cx);
    always_allow_tools(cx);

    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    // Add a tool so we can simulate tool calls
    thread.update(cx, |thread, _cx| {
        thread.add_tool(EchoTool);
    });

    // Start a turn by sending a message
    let mut events = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Use the echo tool"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    // Simulate the model making a tool call
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "tool_1".into(),
            name: "echo".into(),
            raw_input: r#"{"text": "hello"}"#.into(),
            input: json!({"text": "hello"}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::ToolUse));

    // Queue a message before ending the stream
    thread.update(cx, |thread, _cx| {
        thread.queue_message(
            vec![acp::ContentBlock::Text(acp::TextContent::new(
                "This is my queued message".to_string(),
            ))],
            vec![],
        );
    });

    // Now end the stream - tool will run, and the boundary check should see the queue
    fake_model.end_last_completion_stream();

    // Collect all events until the turn stops
    let all_events = collect_events_until_stop(&mut events, cx).await;

    // Verify we received the tool call event
    let tool_call_ids: Vec<_> = all_events
        .iter()
        .filter_map(|e| match e {
            Ok(ThreadEvent::ToolCall(tc)) => Some(tc.tool_call_id.to_string()),
            _ => None,
        })
        .collect();
    assert_eq!(
        tool_call_ids,
        vec!["tool_1"],
        "Should have received a tool call event for our echo tool"
    );

    // The turn should have stopped with EndTurn
    let stop_reasons = stop_events(all_events);
    assert_eq!(
        stop_reasons,
        vec![acp::StopReason::EndTurn],
        "Turn should have ended after tool completion due to queued message"
    );

    // Verify the queued message is still there
    thread.update(cx, |thread, _cx| {
        let queued = thread.queued_messages();
        assert_eq!(queued.len(), 1, "Should still have one queued message");
        assert!(matches!(
            &queued[0].content[0],
            acp::ContentBlock::Text(t) if t.text == "This is my queued message"
        ));
    });

    // Thread should be idle now
    thread.update(cx, |thread, _cx| {
        assert!(
            thread.is_turn_complete(),
            "Thread should not be running after turn ends"
        );
    });
}
