use super::*;
use crate::templates::Templates;
use acp_thread::AgentConnection as _;
use agent_client_protocol as acp;
use client::{Client, UserStore};
use gpui::{AppContext, Entity, TestAppContext};
use indoc::indoc;
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelRegistry, MessageContent, StopReason,
};
use project::Project;
use reqwest_client::ReqwestClient;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use smol::stream::StreamExt;
use std::{path::Path, rc::Rc, sync::Arc, time::Duration};

mod test_tools;
use test_tools::*;

const SONNET_4: &str = "claude-sonnet-4-latest";
const SONNET_4_THINKING: &str = "claude-sonnet-4-thinking-latest";

#[gpui::test]
async fn test_echo(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, SONNET_4).await;

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
    assert_eq!(stop_events(events), vec![StopReason::EndTurn]);
}

#[gpui::test]
async fn test_thinking(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, SONNET_4_THINKING).await;

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
    assert_eq!(stop_events(events), vec![StopReason::EndTurn]);
}

#[gpui::test]
async fn test_basic_tool_calls(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, SONNET_4).await;

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
    assert_eq!(
        stop_events(events),
        vec![StopReason::ToolUse, StopReason::EndTurn]
    );

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
    assert_eq!(
        stop_events(events),
        vec![StopReason::ToolUse, StopReason::EndTurn]
    );
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
async fn test_streaming_tool_calls(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, SONNET_4).await;

    // Test a tool call that's likely to complete *before* streaming stops.
    let mut events = thread.update(cx, |thread, cx| {
        thread.add_tool(WordListTool);
        thread.send(model.clone(), "Test the word_list tool.", cx)
    });

    let mut saw_partial_tool_use = false;
    while let Some(event) = events.next().await {
        if let Ok(LanguageModelCompletionEvent::ToolUse(tool_use_event)) = event {
            thread.update(cx, |thread, _cx| {
                // Look for a tool use in the thread's last message
                let last_content = thread.messages().last().unwrap().content.last().unwrap();
                if let MessageContent::ToolUse(last_tool_use) = last_content {
                    assert_eq!(last_tool_use.name.as_ref(), "word_list");
                    if tool_use_event.is_input_complete {
                        last_tool_use
                            .input
                            .get("a")
                            .expect("'a' has streamed because input is now complete");
                        last_tool_use
                            .input
                            .get("g")
                            .expect("'g' has streamed because input is now complete");
                    } else {
                        if !last_tool_use.is_input_complete
                            && last_tool_use.input.get("g").is_none()
                        {
                            saw_partial_tool_use = true;
                        }
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
async fn test_concurrent_tool_calls(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx, SONNET_4).await;

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
    if stop_reasons.len() == 2 {
        assert_eq!(stop_reasons, vec![StopReason::ToolUse, StopReason::EndTurn]);
    } else if stop_reasons.len() == 3 {
        assert_eq!(
            stop_reasons,
            vec![
                StopReason::ToolUse,
                StopReason::ToolUse,
                StopReason::EndTurn
            ]
        );
    } else {
        panic!("Expected either 1 or 2 tool uses followed by end turn");
    }

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

    cx.update(|cx| connection.prompt(prompt_request, cx))
        .await
        .expect("prompt should succeed");

    // The prompt was sent successfully

    // Test cancel
    cx.update(|cx| connection.cancel(&session_id, cx));

    // After cancel, selected_model should fail
    let result = cx
        .update(|cx| {
            let mut async_cx = cx.to_async();
            selector.selected_model(&session_id, &mut async_cx)
        })
        .await;
    assert!(result.is_err(), "selected_model should fail after cancel");

    // Test error case: invalid session
    let invalid_session = acp::SessionId("invalid".into());
    let result = cx
        .update(|cx| {
            let mut async_cx = cx.to_async();
            selector.selected_model(&invalid_session, &mut async_cx)
        })
        .await;
    assert!(result.is_err(), "should fail for invalid session");
    if let Err(e) = result {
        assert!(
            e.to_string().contains("Session not found"),
            "should have correct error message"
        );
    }
}

/// Filters out the stop events for asserting against in tests
fn stop_events(
    result_events: Vec<Result<AgentResponseEvent, LanguageModelCompletionError>>,
) -> Vec<StopReason> {
    result_events
        .into_iter()
        .filter_map(|event| match event.unwrap() {
            LanguageModelCompletionEvent::Stop(stop_reason) => Some(stop_reason),
            _ => None,
        })
        .collect()
}

struct ThreadTest {
    model: Arc<dyn LanguageModel>,
    thread: Entity<Thread>,
}

async fn setup(cx: &mut TestAppContext, model_name: &'static str) -> ThreadTest {
    cx.executor().allow_parking();
    cx.update(settings::init);
    let templates = Templates::new();

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

            let models = LanguageModelRegistry::read_global(cx);
            let model = models
                .available_models(cx)
                .find(|model| model.id().0 == model_name)
                .unwrap();

            let provider = models.provider(&model.provider_id()).unwrap();
            let authenticated = provider.authenticate(cx);

            cx.spawn(async move |_cx| {
                authenticated.await.unwrap();
                model
            })
        })
        .await;

    let thread = cx.new(|_| Thread::new(templates, model.clone()));

    ThreadTest { model, thread }
}

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}
