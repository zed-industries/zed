use super::*;
use crate::templates::Templates;
use client::{Client, UserStore};
use gpui::{AppContext, Entity, TestAppContext};
use language_model::{
    LanguageModel, LanguageModelCompletionError, LanguageModelCompletionEvent,
    LanguageModelRegistry, MessageContent, StopReason,
};
use reqwest_client::ReqwestClient;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use smol::stream::StreamExt;
use std::{sync::Arc, time::Duration};

mod test_tools;
use test_tools::*;

#[gpui::test]
async fn test_echo(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx).await;

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
async fn test_basic_tool_calls(cx: &mut TestAppContext) {
    let ThreadTest { model, thread, .. } = setup(cx).await;

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
    let ThreadTest { model, thread, .. } = setup(cx).await;

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
    let ThreadTest { model, thread, .. } = setup(cx).await;

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

async fn setup(cx: &mut TestAppContext) -> ThreadTest {
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
                .find(|model| model.id().0 == "claude-3-7-sonnet-latest")
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
