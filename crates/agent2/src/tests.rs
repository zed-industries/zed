use super::*;
use assistant_tool::{IconName, Project, ToolResult};
use client::{Client, UserStore};
use fs::FakeFs;
use gpui::{AppContext, TestAppContext};
use language_model::LanguageModelRegistry;
use reqwest_client::ReqwestClient;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::time::Duration;

mod tools;
use tools::*;

#[gpui::test]
async fn test_echo(cx: &mut TestAppContext) {
    let AgentTest { model, agent, .. } = agent_test(cx).await;

    let events = agent
        .update(cx, |agent, cx| {
            agent.send(model.clone(), "Testing: Reply with 'Hello'", cx)
        })
        .collect()
        .await;
    agent.update(cx, |agent, _cx| {
        assert_eq!(
            agent.messages.last().unwrap().content,
            vec![MessageContent::Text("Hello".to_string())]
        );
    });
    assert_eq!(stop_events(events), vec![StopReason::EndTurn]);
}

#[gpui::test]
async fn test_tool_calls(cx: &mut TestAppContext) {
    let AgentTest { model, agent, .. } = agent_test(cx).await;

    // Test a tool calls that's likely to complete before streaming stops.
    let events = agent
        .update(cx, |agent, cx| {
            agent.add_tool(Arc::new(EchoTool));
            agent.send(
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

    // Test a tool calls that's likely to complete after streaming stops.
    let events = agent
        .update(cx, |agent, cx| {
            agent.remove_tool(&Tool::name(&EchoTool));
            agent.add_tool(Arc::new(DelayTool));
            agent.send(
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
    agent.update(cx, |agent, _cx| {
        assert!(agent
            .messages
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
async fn test_concurrent_tool_calls(cx: &mut TestAppContext) {
    let AgentTest { model, agent, .. } = agent_test(cx).await;

    // Test concurrent tool calls with different delay times
    let events = agent
        .update(cx, |agent, cx| {
            agent.add_tool(Arc::new(DelayTool));
            agent.send(
                model.clone(),
                "Call the delay tool twice in the same message. Once with 100ms. Once with 300ms. When both timers are complete, describe the outputs.",
                cx,
            )
        })
        .map(|event| dbg!(event))
        .collect()
        .await;

    let stop_reasons = stop_events(events);
    assert_eq!(stop_reasons, vec![StopReason::ToolUse, StopReason::EndTurn]);

    agent.update(cx, |agent, _cx| {
        let last_message = agent.messages.last().unwrap();
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

fn stop_events(result_events: Vec<Result<AgentResponseEvent>>) -> Vec<StopReason> {
    result_events
        .into_iter()
        .filter_map(|event| match event.unwrap() {
            LanguageModelCompletionEvent::Stop(stop_reason) => Some(stop_reason),
            _ => None,
        })
        .collect()
}

struct AgentTest {
    model: Arc<dyn LanguageModel>,
    agent: Entity<Agent>,
}

async fn agent_test(cx: &mut TestAppContext) -> AgentTest {
    cx.executor().allow_parking();
    cx.update(settings::init);
    let fs = FakeFs::new(cx.executor().clone());
    // let project = Project::test(fs.clone(), [], cx).await;
    // let action_log = cx.new(|_| ActionLog::new(project.clone()));
    let templates = Templates::new();
    let agent = cx.new(|_| Agent::new(templates));

    let model = cx
        .update(|cx| {
            gpui_tokio::init(cx);
            let http_client = ReqwestClient::user_agent("agent tests").unwrap();
            cx.set_http_client(Arc::new(http_client));

            client::init_settings(cx);
            let client = Client::production(cx);
            let user_store = cx.new(|cx| UserStore::new(client.clone(), cx));
            language_model::init(client.clone(), cx);
            language_models::init(user_store.clone(), client.clone(), fs.clone(), cx);

            let models = LanguageModelRegistry::read_global(cx);
            let model = models
                .available_models(cx)
                .find(|model| model.id().0 == "claude-3-7-sonnet-latest")
                .unwrap();

            let provider = models.provider(&model.provider_id()).unwrap();
            let authenticated = provider.authenticate(cx);

            cx.spawn(async move |cx| {
                authenticated.await.unwrap();
                model
            })
        })
        .await;

    AgentTest { model, agent }
}

#[cfg(test)]
#[ctor::ctor]
fn init_logger() {
    if std::env::var("RUST_LOG").is_ok() {
        env_logger::init();
    }
}
