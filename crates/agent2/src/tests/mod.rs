use super::*;
use crate::MessageContent;
use acp_thread::{AgentConnection, AgentModelGroupName, AgentModelList};
use action_log::ActionLog;
use agent_client_protocol::{self as acp};
use agent_settings::AgentProfileId;
use anyhow::Result;
use client::{Client, UserStore};
use fs::{FakeFs, Fs};
use futures::channel::mpsc::UnboundedReceiver;
use gpui::{
    App, AppContext, Entity, Task, TestAppContext, UpdateGlobal, http_client::FakeHttpClient,
};
use indoc::indoc;
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent, LanguageModelId,
    LanguageModelRegistry, LanguageModelToolResult, LanguageModelToolUse, Role, StopReason,
    fake_provider::FakeLanguageModel,
};
use project::Project;
use prompt_store::ProjectContext;
use reqwest_client::ReqwestClient;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use settings::SettingsStore;
use smol::stream::StreamExt;
use std::{cell::RefCell, path::Path, rc::Rc, sync::Arc, time::Duration};
use util::path;

mod test_tools;
use test_tools::*;

#[gpui::test]
#[ignore = "can't run on CI yet"]
async fn test_echo(cx: &mut TestAppContext) {
    let ThreadTest { thread, .. } = setup(cx, TestModel::Sonnet4).await;

    let events = thread
        .update(cx, |thread, cx| {
            thread.send("Testing: Reply with 'Hello'", cx)
        })
        .collect()
        .await;
    thread.update(cx, |thread, _cx| {
        assert_eq!(
            thread.messages().last().unwrap().content,
            vec![MessageContent::Text("Hello".to_string())]
        );
    });
    assert_eq!(stop_events(events), vec![acp::StopReason::EndTurn]);
}

#[gpui::test]
#[ignore = "can't run on CI yet"]
async fn test_thinking(cx: &mut TestAppContext) {
    let ThreadTest { thread, .. } = setup(cx, TestModel::Sonnet4Thinking).await;

    let events = thread
        .update(cx, |thread, cx| {
            thread.send(
                indoc! {"
                    Testing:

                    Generate a thinking step where you just think the word 'Think',
                    and have your final answer be 'Hello'
                "},
                cx,
            )
        })
        .collect()
        .await;
    thread.update(cx, |thread, _cx| {
        assert_eq!(
            thread.messages().last().unwrap().to_markdown(),
            indoc! {"
                ## assistant
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

    project_context.borrow_mut().shell = "test-shell".into();
    thread.update(cx, |thread, _| thread.add_tool(EchoTool));
    thread.update(cx, |thread, cx| thread.send("abc", cx));
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
#[ignore = "can't run on CI yet"]
async fn test_basic_tool_calls(cx: &mut TestAppContext) {
    let ThreadTest { thread, .. } = setup(cx, TestModel::Sonnet4).await;

    // Test a tool call that's likely to complete *before* streaming stops.
    let events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(EchoTool);
            thread.send(
                "Now test the echo tool with 'Hello'. Does it work? Say 'Yes' or 'No'.",
                cx,
            )
        })
        .collect()
        .await;
    assert_eq!(stop_events(events), vec![acp::StopReason::EndTurn]);

    // Test a tool calls that's likely to complete *after* streaming stops.
    let events = thread
        .update(cx, |thread, cx| {
            thread.remove_tool(&AgentTool::name(&EchoTool));
            thread.add_tool(DelayTool);
            thread.send(
                "Now call the delay tool with 200ms. When the timer goes off, then you echo the output of the tool.",
                cx,
            )
        })
        .collect()
        .await;
    assert_eq!(stop_events(events), vec![acp::StopReason::EndTurn]);
    thread.update(cx, |thread, _cx| {
        assert!(
            thread
                .messages()
                .last()
                .unwrap()
                .content
                .iter()
                .any(|content| {
                    if let MessageContent::Text(text) = content {
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
#[ignore = "can't run on CI yet"]
async fn test_streaming_tool_calls(cx: &mut TestAppContext) {
    let ThreadTest { thread, .. } = setup(cx, TestModel::Sonnet4).await;

    // Test a tool call that's likely to complete *before* streaming stops.
    let mut events = thread.update(cx, |thread, cx| {
        thread.add_tool(WordListTool);
        thread.send("Test the word_list tool.", cx)
    });

    let mut saw_partial_tool_use = false;
    while let Some(event) = events.next().await {
        if let Ok(AgentResponseEvent::ToolCall(tool_call)) = event {
            thread.update(cx, |thread, _cx| {
                // Look for a tool use in the thread's last message
                let last_content = thread.messages().last().unwrap().content.last().unwrap();
                if let MessageContent::ToolUse(last_tool_use) = last_content {
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

    let mut events = thread.update(cx, |thread, cx| {
        thread.add_tool(ToolRequiringPermission);
        thread.send("abc", cx)
    });
    cx.run_until_parked();
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "tool_id_1".into(),
            name: ToolRequiringPermission.name().into(),
            raw_input: "{}".into(),
            input: json!({}),
            is_input_complete: true,
        },
    ));
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "tool_id_2".into(),
            name: ToolRequiringPermission.name().into(),
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
                tool_name: ToolRequiringPermission.name().into(),
                is_error: false,
                content: "Allowed".into(),
                output: Some("Allowed".into())
            }),
            language_model::MessageContent::ToolResult(LanguageModelToolResult {
                tool_use_id: tool_call_auth_2.tool_call.id.0.to_string().into(),
                tool_name: ToolRequiringPermission.name().into(),
                is_error: true,
                content: "Permission to run tool denied by user".into(),
                output: None
            })
        ]
    );

    // Simulate yet another tool call.
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "tool_id_3".into(),
            name: ToolRequiringPermission.name().into(),
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
                tool_name: ToolRequiringPermission.name().into(),
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
            name: ToolRequiringPermission.name().into(),
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
                tool_name: ToolRequiringPermission.name().into(),
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

    let mut events = thread.update(cx, |thread, cx| thread.send("abc", cx));
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

async fn expect_tool_call(
    events: &mut UnboundedReceiver<Result<AgentResponseEvent, LanguageModelCompletionError>>,
) -> acp::ToolCall {
    let event = events
        .next()
        .await
        .expect("no tool call authorization event received")
        .unwrap();
    match event {
        AgentResponseEvent::ToolCall(tool_call) => return tool_call,
        event => {
            panic!("Unexpected event {event:?}");
        }
    }
}

async fn expect_tool_call_update_fields(
    events: &mut UnboundedReceiver<Result<AgentResponseEvent, LanguageModelCompletionError>>,
) -> acp::ToolCallUpdate {
    let event = events
        .next()
        .await
        .expect("no tool call authorization event received")
        .unwrap();
    match event {
        AgentResponseEvent::ToolCallUpdate(acp_thread::ToolCallUpdate::UpdateFields(update)) => {
            return update;
        }
        event => {
            panic!("Unexpected event {event:?}");
        }
    }
}

async fn next_tool_call_authorization(
    events: &mut UnboundedReceiver<Result<AgentResponseEvent, LanguageModelCompletionError>>,
) -> ToolCallAuthorization {
    loop {
        let event = events
            .next()
            .await
            .expect("no tool call authorization event received")
            .unwrap();
        if let AgentResponseEvent::ToolCallAuthorization(tool_call_authorization) = event {
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
#[ignore = "can't run on CI yet"]
async fn test_concurrent_tool_calls(cx: &mut TestAppContext) {
    let ThreadTest { thread, .. } = setup(cx, TestModel::Sonnet4).await;

    // Test concurrent tool calls with different delay times
    let events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(DelayTool);
            thread.send(
                "Call the delay tool twice in the same message. Once with 100ms. Once with 300ms. When both timers are complete, describe the outputs.",
                cx,
            )
        })
        .collect()
        .await;

    let stop_reasons = stop_events(events);
    assert_eq!(stop_reasons, vec![acp::StopReason::EndTurn]);

    thread.update(cx, |thread, _cx| {
        let last_message = thread.messages().last().unwrap();
        let text = last_message
            .content
            .iter()
            .filter_map(|content| {
                if let MessageContent::Text(text) = content {
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
                            EchoTool.name(): true,
                            DelayTool.name(): true,
                        }
                    },
                    "test-2": {
                        "name": "Test Profile 2",
                        "tools": {
                            InfiniteTool.name(): true,
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
    thread.update(cx, |thread, cx| {
        thread.set_profile(AgentProfileId("test-1".into()));
        thread.send("test", cx);
    });
    cx.run_until_parked();

    let mut pending_completions = fake_model.pending_completions();
    assert_eq!(pending_completions.len(), 1);
    let completion = pending_completions.pop().unwrap();
    let tool_names: Vec<String> = completion
        .tools
        .iter()
        .map(|tool| tool.name.clone())
        .collect();
    assert_eq!(tool_names, vec![DelayTool.name(), EchoTool.name()]);
    fake_model.end_last_completion_stream();

    // Switch to test-2 profile, and verify that it has only the infinite tool.
    thread.update(cx, |thread, cx| {
        thread.set_profile(AgentProfileId("test-2".into()));
        thread.send("test2", cx)
    });
    cx.run_until_parked();
    let mut pending_completions = fake_model.pending_completions();
    assert_eq!(pending_completions.len(), 1);
    let completion = pending_completions.pop().unwrap();
    let tool_names: Vec<String> = completion
        .tools
        .iter()
        .map(|tool| tool.name.clone())
        .collect();
    assert_eq!(tool_names, vec![InfiniteTool.name()]);
}

#[gpui::test]
#[ignore = "can't run on CI yet"]
async fn test_cancellation(cx: &mut TestAppContext) {
    let ThreadTest { thread, .. } = setup(cx, TestModel::Sonnet4).await;

    let mut events = thread.update(cx, |thread, cx| {
        thread.add_tool(InfiniteTool);
        thread.add_tool(EchoTool);
        thread.send(
            "Call the echo tool and then call the infinite tool, then explain their output",
            cx,
        )
    });

    // Wait until both tools are called.
    let mut expected_tools = vec!["Echo", "Infinite Tool"];
    let mut echo_id = None;
    let mut echo_completed = false;
    while let Some(event) = events.next().await {
        match event.unwrap() {
            AgentResponseEvent::ToolCall(tool_call) => {
                assert_eq!(tool_call.title, expected_tools.remove(0));
                if tool_call.title == "Echo" {
                    echo_id = Some(tool_call.id);
                }
            }
            AgentResponseEvent::ToolCallUpdate(acp_thread::ToolCallUpdate::UpdateFields(
                acp::ToolCallUpdate {
                    id,
                    fields:
                        acp::ToolCallUpdateFields {
                            status: Some(acp::ToolCallStatus::Completed),
                            ..
                        },
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
    thread.update(cx, |thread, _cx| thread.cancel());
    events.collect::<Vec<_>>().await;

    // Ensure we can still send a new message after cancellation.
    let events = thread
        .update(cx, |thread, cx| {
            thread.send("Testing: reply with 'Hello' then stop.", cx)
        })
        .collect::<Vec<_>>()
        .await;
    thread.update(cx, |thread, _cx| {
        assert_eq!(
            thread.messages().last().unwrap().content,
            vec![MessageContent::Text("Hello".to_string())]
        );
    });
    assert_eq!(stop_events(events), vec![acp::StopReason::EndTurn]);
}

#[gpui::test]
async fn test_refusal(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    let events = thread.update(cx, |thread, cx| thread.send("Hello", cx));
    cx.run_until_parked();
    thread.read_with(cx, |thread, _| {
        assert_eq!(
            thread.to_markdown(),
            indoc! {"
                ## user
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
                ## user
                Hello
                ## assistant
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
        language_models::init(user_store.clone(), client.clone(), cx);
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

    // Create agent and connection
    let agent = NativeAgent::new(
        project.clone(),
        templates.clone(),
        None,
        fake_fs.clone(),
        &mut cx.to_async(),
    )
    .await
    .unwrap();
    let connection = NativeAgentConnection(agent.clone());

    // Test model_selector returns Some
    let selector_opt = connection.model_selector();
    assert!(
        selector_opt.is_some(),
        "agent2 should always support ModelSelector"
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
        listed_models[&AgentModelGroupName("Fake".into())][0].id.0,
        "fake/fake"
    );

    // Create a thread using new_thread
    let connection_rc = Rc::new(connection.clone());
    let acp_thread = cx
        .update(|cx| connection_rc.new_thread(project, cwd, &mut cx.to_async()))
        .await
        .expect("new_thread should succeed");

    // Get the session_id from the AcpThread
    let session_id = acp_thread.read_with(cx, |thread, _| thread.session_id().clone());

    // Test selected_model returns the default
    let model = cx
        .update(|cx| selector.selected_model(&session_id, cx))
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
                acp::PromptRequest {
                    session_id: session_id.clone(),
                    prompt: vec!["ghi".into()],
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

    let mut events = thread.update(cx, |thread, cx| thread.send("Think", cx));
    cx.run_until_parked();

    // Simulate streaming partial input.
    let input = json!({});
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "1".into(),
            name: ThinkingTool.name().into(),
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
        }
    );
}

/// Filters out the stop events for asserting against in tests
fn stop_events(
    result_events: Vec<Result<AgentResponseEvent, LanguageModelCompletionError>>,
) -> Vec<acp::StopReason> {
    result_events
        .into_iter()
        .filter_map(|event| match event.unwrap() {
            AgentResponseEvent::Stop(stop_reason) => Some(stop_reason),
            _ => None,
        })
        .collect()
}

struct ThreadTest {
    model: Arc<dyn LanguageModel>,
    thread: Entity<Thread>,
    project_context: Rc<RefCell<ProjectContext>>,
    fs: Arc<FakeFs>,
}

enum TestModel {
    Sonnet4,
    Sonnet4Thinking,
    Fake,
}

impl TestModel {
    fn id(&self) -> LanguageModelId {
        match self {
            TestModel::Sonnet4 => LanguageModelId("claude-sonnet-4-latest".into()),
            TestModel::Sonnet4Thinking => LanguageModelId("claude-sonnet-4-thinking-latest".into()),
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
                            EchoTool.name(): true,
                            DelayTool.name(): true,
                            WordListTool.name(): true,
                            ToolRequiringPermission.name(): true,
                            InfiniteTool.name(): true,
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
        gpui_tokio::init(cx);
        let http_client = ReqwestClient::user_agent("agent tests").unwrap();
        cx.set_http_client(Arc::new(http_client));

        client::init_settings(cx);
        let client = Client::production(cx);
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        language_model::init(client.clone(), cx);
        language_models::init(user_store.clone(), client.clone(), cx);

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

    let project_context = Rc::new(RefCell::new(ProjectContext::default()));
    let context_server_registry =
        cx.new(|cx| ContextServerRegistry::new(project.read(cx).context_server_store(), cx));
    let action_log = cx.new(|_| ActionLog::new(project.clone()));
    let thread = cx.new(|cx| {
        Thread::new(
            project,
            project_context.clone(),
            context_server_registry,
            action_log,
            templates,
            model.clone(),
            cx,
        )
    });
    ThreadTest {
        model,
        thread,
        project_context,
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
