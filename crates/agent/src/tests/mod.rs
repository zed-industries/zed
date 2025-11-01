use super::*;
use acp_thread::{AgentConnection, AgentModelGroupName, AgentModelList, UserMessageId};
use agent_client_protocol::{self as acp};
use agent_settings::AgentProfileId;
use anyhow::Result;
use client::{Client, UserStore};
use cloud_llm_client::CompletionIntent;
use collections::IndexMap;
use context_server::{ContextServer, ContextServerCommand, ContextServerId};
use fs::{FakeFs, Fs};
use futures::{
    StreamExt,
    channel::{
        mpsc::{self, UnboundedReceiver},
        oneshot,
    },
};
use gpui::{
    App, AppContext, Entity, Task, TestAppContext, UpdateGlobal, http_client::FakeHttpClient,
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
use std::{path::Path, rc::Rc, sync::Arc, time::Duration};
use util::path;

mod test_tools;
use test_tools::*;

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
            cache: true
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
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec!["Response to Message 1".into()],
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["Message 2".into()],
                cache: true
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
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec!["Response to Message 1".into()],
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["Message 2".into()],
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec!["Response to Message 2".into()],
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["Use the echo tool".into()],
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![MessageContent::ToolUse(tool_use)],
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::ToolResult(tool_result)],
                cache: true
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
        },
    ));
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "tool_id_2".into(),
            name: ToolRequiringPermission::name().into(),
            raw_input: "{}".into(),
            input: json!({}),
            is_input_complete: true,
        },
    ));
    fake_model.end_last_completion_stream();
    let tool_call_auth_1 = next_tool_call_authorization(&mut events).await;
    let tool_call_auth_2 = next_tool_call_authorization(&mut events).await;

    // Approve the first
    tool_call_auth_1
        .response
        .send(tool_call_auth_1.options[1].id.clone())
        .unwrap();
    cx.run_until_parked();

    // Reject the second
    tool_call_auth_2
        .response
        .send(tool_call_auth_1.options[2].id.clone())
        .unwrap();
    cx.run_until_parked();

    let completion = fake_model.pending_completions().pop().unwrap();
    let message = completion.messages.last().unwrap();
    assert_eq!(
        message.content,
        vec![
            language_model::MessageContent::ToolResult(LanguageModelToolResult {
                tool_use_id: tool_call_auth_1.tool_call.id.0.to_string().into(),
                tool_name: ToolRequiringPermission::name().into(),
                is_error: false,
                content: "Allowed".into(),
                output: Some("Allowed".into())
            }),
            language_model::MessageContent::ToolResult(LanguageModelToolResult {
                tool_use_id: tool_call_auth_2.tool_call.id.0.to_string().into(),
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
        },
    ));
    fake_model.end_last_completion_stream();

    // Respond by always allowing tools.
    let tool_call_auth_3 = next_tool_call_authorization(&mut events).await;
    tool_call_auth_3
        .response
        .send(tool_call_auth_3.options[0].id.clone())
        .unwrap();
    cx.run_until_parked();
    let completion = fake_model.pending_completions().pop().unwrap();
    let message = completion.messages.last().unwrap();
    assert_eq!(
        message.content,
        vec![language_model::MessageContent::ToolResult(
            LanguageModelToolResult {
                tool_use_id: tool_call_auth_3.tool_call.id.0.to_string().into(),
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
        },
    ));
    fake_model.end_last_completion_stream();

    let tool_call = expect_tool_call(&mut events).await;
    assert_eq!(tool_call.title, "nonexistent_tool");
    assert_eq!(tool_call.status, acp::ToolCallStatus::Pending);
    let update = expect_tool_call_update_fields(&mut events).await;
    assert_eq!(update.fields.status, Some(acp::ToolCallStatus::Failed));
}

#[gpui::test]
async fn test_resume_after_tool_use_limit(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(EchoTool);
            thread.send(UserMessageId::new(), ["abc"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    let tool_use = LanguageModelToolUse {
        id: "tool_id_1".into(),
        name: EchoTool::name().into(),
        raw_input: "{}".into(),
        input: serde_json::to_value(&EchoToolInput { text: "def".into() }).unwrap(),
        is_input_complete: true,
    };
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(tool_use.clone()));
    fake_model.end_last_completion_stream();

    cx.run_until_parked();
    let completion = fake_model.pending_completions().pop().unwrap();
    let tool_result = LanguageModelToolResult {
        tool_use_id: "tool_id_1".into(),
        tool_name: EchoTool::name().into(),
        is_error: false,
        content: "def".into(),
        output: Some("def".into()),
    };
    assert_eq!(
        completion.messages[1..],
        vec![
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["abc".into()],
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![MessageContent::ToolUse(tool_use.clone())],
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::ToolResult(tool_result.clone())],
                cache: true
            },
        ]
    );

    // Simulate reaching tool use limit.
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::StatusUpdate(
        cloud_llm_client::CompletionRequestStatus::ToolUseLimitReached,
    ));
    fake_model.end_last_completion_stream();
    let last_event = events.collect::<Vec<_>>().await.pop().unwrap();
    assert!(
        last_event
            .unwrap_err()
            .is::<language_model::ToolUseLimitReachedError>()
    );

    let events = thread.update(cx, |thread, cx| thread.resume(cx)).unwrap();
    cx.run_until_parked();
    let completion = fake_model.pending_completions().pop().unwrap();
    assert_eq!(
        completion.messages[1..],
        vec![
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["abc".into()],
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![MessageContent::ToolUse(tool_use)],
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::ToolResult(tool_result)],
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["Continue where you left off".into()],
                cache: true
            }
        ]
    );

    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::Text("Done".into()));
    fake_model.end_last_completion_stream();
    events.collect::<Vec<_>>().await;
    thread.read_with(cx, |thread, _cx| {
        assert_eq!(
            thread.last_message().unwrap().to_markdown(),
            indoc! {"
                ## Assistant

                Done
            "}
        )
    });
}

#[gpui::test]
async fn test_send_after_tool_use_limit(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(EchoTool);
            thread.send(UserMessageId::new(), ["abc"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    let tool_use = LanguageModelToolUse {
        id: "tool_id_1".into(),
        name: EchoTool::name().into(),
        raw_input: "{}".into(),
        input: serde_json::to_value(&EchoToolInput { text: "def".into() }).unwrap(),
        is_input_complete: true,
    };
    let tool_result = LanguageModelToolResult {
        tool_use_id: "tool_id_1".into(),
        tool_name: EchoTool::name().into(),
        is_error: false,
        content: "def".into(),
        output: Some("def".into()),
    };
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(tool_use.clone()));
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::StatusUpdate(
        cloud_llm_client::CompletionRequestStatus::ToolUseLimitReached,
    ));
    fake_model.end_last_completion_stream();
    let last_event = events.collect::<Vec<_>>().await.pop().unwrap();
    assert!(
        last_event
            .unwrap_err()
            .is::<language_model::ToolUseLimitReachedError>()
    );

    thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), vec!["ghi"], cx)
        })
        .unwrap();
    cx.run_until_parked();
    let completion = fake_model.pending_completions().pop().unwrap();
    assert_eq!(
        completion.messages[1..],
        vec![
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["abc".into()],
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![MessageContent::ToolUse(tool_use)],
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec![MessageContent::ToolResult(tool_result)],
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::User,
                content: vec!["ghi".into()],
                cache: true
            }
        ]
    );
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
                .iter()
                .map(|o| o.kind)
                .collect::<Vec<_>>();
            assert_eq!(
                permission_kinds,
                vec![
                    acp::PermissionOptionKind::AllowAlways,
                    acp::PermissionOptionKind::AllowOnce,
                    acp::PermissionOptionKind::RejectOnce,
                ]
            );
            return tool_call_authorization;
        }
    }
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
            thread.set_profile(AgentProfileId("test-1".into()));
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
            thread.set_profile(AgentProfileId("test-2".into()));
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
    thread.update(cx, |thread, _| {
        thread.set_profile(AgentProfileId("test".into()))
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
        },
    ));
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "tool_3".into(),
            name: "echo".into(),
            raw_input: json!({"text": "native"}).to_string(),
            input: json!({"text": "native"}),
            is_input_complete: true,
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

    thread.update(cx, |thread, _| {
        thread.set_profile(AgentProfileId("test".into()));
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
                    echo_id = Some(tool_call.id);
                }
            }
            ThreadEvent::ToolCallUpdate(acp_thread::ToolCallUpdate::UpdateFields(
                acp::ToolCallUpdate {
                    id,
                    fields:
                        acp::ToolCallUpdateFields {
                            status: Some(acp::ToolCallStatus::Completed),
                            ..
                        },
                    meta: None,
                },
            )) if Some(&id) == echo_id.as_ref() => {
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
    thread.update(cx, |thread, cx| thread.cancel(cx));
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
    };
    let echo_tool_use = LanguageModelToolUse {
        id: "tool_id_2".into(),
        name: EchoTool::name().into(),
        raw_input: json!({"text": "test"}).to_string(),
        input: json!({"text": "test"}),
        is_input_complete: true,
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
                cache: true
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![
                    MessageContent::Text("Hi!".into()),
                    MessageContent::ToolUse(echo_tool_use.clone())
                ],
                cache: false
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
                cache: false
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
        client::init_settings(cx);

        let http_client = FakeHttpClient::with_404_response();
        let clock = Arc::new(clock::FakeSystemClock::new());
        let client = Client::new(clock, http_client, cx);
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        language_model::init(client.clone(), cx);
        language_models::init(user_store, client.clone(), cx);
        Project::init_settings(cx);
        LanguageModelRegistry::test(cx);
        agent_settings::init(cx);
    });
    cx.executor().forbid_parking();

    // Create a project for new_thread
    let fake_fs = cx.update(|cx| fs::FakeFs::new(cx.background_executor().clone()));
    fake_fs.insert_tree(path!("/test"), json!({})).await;
    let project = Project::test(fake_fs.clone(), [Path::new("/test")], cx).await;
    let cwd = Path::new("/test");
    let text_thread_store =
        cx.new(|cx| assistant_text_thread::TextThreadStore::fake(project.clone(), cx));
    let history_store = cx.new(|cx| HistoryStore::new(text_thread_store, cx));

    // Create agent and connection
    let agent = NativeAgent::new(
        project.clone(),
        history_store,
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
                acp::PromptRequest {
                    session_id: session_id.clone(),
                    prompt: vec!["ghi".into()],
                    meta: None,
                },
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
        },
    ));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let tool_call = expect_tool_call(&mut events).await;
    assert_eq!(
        tool_call,
        acp::ToolCall {
            id: acp::ToolCallId("1".into()),
            title: "Thinking".into(),
            kind: acp::ToolKind::Think,
            status: acp::ToolCallStatus::Pending,
            content: vec![],
            locations: vec![],
            raw_input: Some(json!({})),
            raw_output: None,
            meta: Some(json!({ "tool_name": "thinking" })),
        }
    );
    let update = expect_tool_call_update_fields(&mut events).await;
    assert_eq!(
        update,
        acp::ToolCallUpdate {
            id: acp::ToolCallId("1".into()),
            fields: acp::ToolCallUpdateFields {
                title: Some("Thinking".into()),
                kind: Some(acp::ToolKind::Think),
                raw_input: Some(json!({ "content": "Thinking hard!" })),
                ..Default::default()
            },
            meta: None,
        }
    );
    let update = expect_tool_call_update_fields(&mut events).await;
    assert_eq!(
        update,
        acp::ToolCallUpdate {
            id: acp::ToolCallId("1".into()),
            fields: acp::ToolCallUpdateFields {
                status: Some(acp::ToolCallStatus::InProgress),
                ..Default::default()
            },
            meta: None,
        }
    );
    let update = expect_tool_call_update_fields(&mut events).await;
    assert_eq!(
        update,
        acp::ToolCallUpdate {
            id: acp::ToolCallId("1".into()),
            fields: acp::ToolCallUpdateFields {
                content: Some(vec!["Thinking hard!".into()]),
                ..Default::default()
            },
            meta: None,
        }
    );
    let update = expect_tool_call_update_fields(&mut events).await;
    assert_eq!(
        update,
        acp::ToolCallUpdate {
            id: acp::ToolCallId("1".into()),
            fields: acp::ToolCallUpdateFields {
                status: Some(acp::ToolCallStatus::Completed),
                raw_output: Some("Finished thinking.".into()),
                ..Default::default()
            },
            meta: None,
        }
    );
}

#[gpui::test]
async fn test_send_no_retry_on_success(cx: &mut TestAppContext) {
    let ThreadTest { thread, model, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let mut events = thread
        .update(cx, |thread, cx| {
            thread.set_completion_mode(agent_settings::CompletionMode::Burn, cx);
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
            thread.set_completion_mode(agent_settings::CompletionMode::Burn, cx);
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
            thread.set_completion_mode(agent_settings::CompletionMode::Burn, cx);
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
                cache: false
            },
            LanguageModelRequestMessage {
                role: Role::Assistant,
                content: vec![language_model::MessageContent::ToolUse(tool_use_1.clone())],
                cache: false
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
                cache: true
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
                tool_results: IndexMap::default()
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
            thread.set_completion_mode(agent_settings::CompletionMode::Burn, cx);
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
                            ThinkingTool::name(): true,
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
        Project::init_settings(cx);
        agent_settings::init(cx);

        match model {
            TestModel::Fake => {}
            TestModel::Sonnet4 => {
                gpui_tokio::init(cx);
                let http_client = ReqwestClient::user_agent("agent tests").unwrap();
                cx.set_http_client(Arc::new(http_client));
                client::init_settings(cx);
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
            project::project_settings::ContextServerSettings::Custom {
                enabled: true,
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
