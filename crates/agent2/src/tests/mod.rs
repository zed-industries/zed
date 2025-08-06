use super::*;
use crate::templates::Templates;
use acp_thread::AgentConnection as _;
use agent_client_protocol as acp;
use client::{Client, UserStore};
use fs::FakeFs;
use gpui::{AppContext, Entity, Task, TestAppContext};
use indoc::indoc;
use language_model::{
    fake_provider::FakeLanguageModel, LanguageModel, LanguageModelCompletionError,
    LanguageModelCompletionEvent, LanguageModelId, LanguageModelRegistry, MessageContent,
    StopReason,
};
use project::Project;
use reqwest_client::ReqwestClient;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use serde_json::json;
use smol::stream::StreamExt;
use std::{path::Path, rc::Rc, sync::Arc, time::Duration};
use util::path;

mod test_tools;
use test_tools::*;

#[gpui::test]
#[ignore = "temporarily disabled until it can be run on CI"]
async fn test_echo(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Sonnet4).await;

    let events = thread
        .update(cx, |thread, cx| {
            thread.send(model.clone(), "Testing: Reply with 'Hello'", cx)
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
#[ignore = "temporarily disabled until it can be run on CI"]
async fn test_thinking(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Sonnet4Thinking).await;

    let events = thread
        .update(cx, |thread, cx| {
            thread.send(
                model.clone(),
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
#[ignore = "temporarily disabled until it can be run on CI"]
async fn test_basic_tool_calls(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Sonnet4).await;

    // Test a tool call that's likely to complete *before* streaming stops.
    let events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(EchoTool);
            thread.send(
                model.clone(),
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
                model.clone(),
                "Now call the delay tool with 200ms. When the timer goes off, then you echo the output of the tool.",
                cx,
            )
        })
        .collect()
        .await;
    assert_eq!(stop_events(events), vec![acp::StopReason::EndTurn]);
    thread.update(cx, |thread, _cx| {
        assert!(thread
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
            }));
    });
}

#[gpui::test]
#[ignore = "temporarily disabled until it can be run on CI"]
async fn test_streaming_tool_calls(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Sonnet4).await;

    // Test a tool call that's likely to complete *before* streaming stops.
    let mut events = thread.update(cx, |thread, cx| {
        thread.add_tool(WordListTool);
        thread.send(model.clone(), "Test the word_list tool.", cx)
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
#[ignore = "temporarily disabled until it can be run on CI"]
async fn test_concurrent_tool_calls(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Sonnet4).await;

    // Test concurrent tool calls with different delay times
    let events = thread
        .update(cx, |thread, cx| {
            thread.add_tool(DelayTool);
            thread.send(
                model.clone(),
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
#[ignore = "temporarily disabled until it can be run on CI"]
async fn test_cancellation(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, TestModel::Sonnet4).await;

    let mut events = thread.update(cx, |thread, cx| {
        thread.add_tool(InfiniteTool);
        thread.add_tool(EchoTool);
        thread.send(
            model.clone(),
            "Call the echo tool and then call the infinite tool, then explain their output",
            cx,
        )
    });

    // Wait until both tools are called.
    let mut expected_tool_calls = vec!["echo", "infinite"];
    let mut echo_id = None;
    let mut echo_completed = false;
    while let Some(event) = events.next().await {
        match event.unwrap() {
            AgentResponseEvent::ToolCall(tool_call) => {
                assert_eq!(tool_call.title, expected_tool_calls.remove(0));
                if tool_call.title == "echo" {
                    echo_id = Some(tool_call.id);
                }
            }
            AgentResponseEvent::ToolCallUpdate(acp::ToolCallUpdate {
                id,
                fields:
                    acp::ToolCallUpdateFields {
                        status: Some(acp::ToolCallStatus::Completed),
                        ..
                    },
            }) if Some(&id) == echo_id.as_ref() => {
                echo_completed = true;
            }
            _ => {}
        }

        if expected_tool_calls.is_empty() && echo_completed {
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
            thread.send(model.clone(), "Testing: reply with 'Hello' then stop.", cx)
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
    let fake_model = Arc::new(FakeLanguageModel::default());
    let ThreadTest { thread, .. } = setup(cx, TestModel::Fake(fake_model.clone())).await;

    let events = thread.update(cx, |thread, cx| {
        thread.send(fake_model.clone(), "Hello", cx)
    });
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

#[ignore = "temporarily disabled until it can be run on CI"]
#[gpui::test]
async fn test_agent_connection(cx: &mut TestAppContext) {
    cx.executor().allow_parking();
    cx.update(settings::init);
    let templates = Templates::new();

    // Initialize language model system with test provider
    cx.update(|cx| {
        gpui_tokio::init(cx);
        let http_client = ReqwestClient::user_agent("agent tests").unwrap();
        cx.set_http_client(Arc::new(http_client));

        client::init_settings(cx);
        let client = Client::production(cx);
        let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
        language_model::init(client.clone(), cx);
        language_models::init(user_store.clone(), client.clone(), cx);

        // Initialize project settings
        Project::init_settings(cx);

        // Use test registry with fake provider
        LanguageModelRegistry::test(cx);
    });

    // Create agent and connection
    let agent = cx.new(|_| NativeAgent::new(templates.clone()));
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
        .update(|cx| {
            let mut async_cx = cx.to_async();
            selector.list_models(&mut async_cx)
        })
        .await
        .expect("list_models should succeed");
    assert!(!listed_models.is_empty(), "should have at least one model");
    assert_eq!(listed_models[0].id().0, "fake");

    // Create a project for new_thread
    let fake_fs = cx.update(|cx| fs::FakeFs::new(cx.background_executor().clone()));
    let project = Project::test(fake_fs, [Path::new("/test")], cx).await;

    // Create a thread using new_thread
    let cwd = Path::new("/test");
    let connection_rc = Rc::new(connection.clone());
    let acp_thread = cx
        .update(|cx| {
            let mut async_cx = cx.to_async();
            connection_rc.new_thread(project, cwd, &mut async_cx)
        })
        .await
        .expect("new_thread should succeed");

    // Get the session_id from the AcpThread
    let session_id = acp_thread.read_with(cx, |thread, _| thread.session_id().clone());

    // Test selected_model returns the default
    let selected = cx
        .update(|cx| {
            let mut async_cx = cx.to_async();
            selector.selected_model(&session_id, &mut async_cx)
        })
        .await
        .expect("selected_model should succeed");
    assert_eq!(selected.id().0, "fake", "should return default model");

    // The thread was created via prompt with the default model
    // We can verify it through selected_model

    // Test prompt uses the selected model
    let prompt_request = acp::PromptRequest {
        session_id: session_id.clone(),
        prompt: vec![acp::ContentBlock::Text(acp::TextContent {
            text: "Test prompt".into(),
            annotations: None,
        })],
    };

    let request = cx.update(|cx| connection.prompt(prompt_request, cx));
    let request = cx.background_spawn(request);
    smol::Timer::after(Duration::from_millis(100)).await;

    // Test cancel
    cx.update(|cx| connection.cancel(&session_id, cx));
    request.await.expect("prompt should fail gracefully");
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
}

enum TestModel {
    Sonnet4,
    Sonnet4Thinking,
    Fake(Arc<FakeLanguageModel>),
}

impl TestModel {
    fn id(&self) -> LanguageModelId {
        match self {
            TestModel::Sonnet4 => LanguageModelId("claude-sonnet-4-latest".into()),
            TestModel::Sonnet4Thinking => LanguageModelId("claude-sonnet-4-thinking-latest".into()),
            TestModel::Fake(fake_model) => fake_model.id(),
        }
    }
}

async fn setup(cx: &mut TestAppContext, model: TestModel) -> ThreadTest {
    cx.executor().allow_parking();
    cx.update(|cx| {
        settings::init(cx);
        Project::init_settings(cx);
    });
    let templates = Templates::new();

    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree(path!("/test"), json!({})).await;
    let project = Project::test(fs, [path!("/test").as_ref()], cx).await;

    let model = cx
        .update(|cx| {
            gpui_tokio::init(cx);
            let http_client = ReqwestClient::user_agent("agent tests").unwrap();
            cx.set_http_client(Arc::new(http_client));

            client::init_settings(cx);
            let client = Client::production(cx);
            let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
            language_model::init(client.clone(), cx);
            language_models::init(user_store.clone(), client.clone(), cx);

            if let TestModel::Fake(model) = model {
                Task::ready(model as Arc<_>)
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

    let thread = cx.new(|_| Thread::new(project, templates, model.clone()));

    ThreadTest { model, thread }
}

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}
