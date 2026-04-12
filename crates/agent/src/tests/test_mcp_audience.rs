//! Tests for MCP tool response audience annotations (`annotations.audience`).
//!
//! # MCP spec reference
//!
//! The `Annotations` type is defined in the MCP JSON Schema (all versions
//! since 2024-11-05).  Each content block in a `tools/call` response may
//! carry an `annotations` object whose `audience` field is an array of
//! `Role` values (`"user"` and/or `"assistant"`).
//!
//! Spec: <https://modelcontextprotocol.io/specification/2025-03-26/server/tools>
//! Schema: <https://github.com/modelcontextprotocol/modelcontextprotocol/blob/main/schema/2025-03-26/schema.json>
//!
//! # ACP mapping
//!
//! Zed maps MCP audience annotations to ACP (`agent-client-protocol`)
//! ToolCallContent updates.  User-only blocks are pushed to the event
//! stream as `ToolCallContent::Content` display updates (shown in the
//! tool call card) but excluded from the `AgentToolOutput` sent to the
//! model.  See `ContextServerTool::run()` in
//! `crates/agent/src/tools/context_server_registry.rs`.
//!
//! # Routing rules tested here
//!
//! | `annotations.audience`      | Sent to model? | User display update? |
//! |-----------------------------|----------------|----------------------|
//! | absent / `null`             | yes            | no                   |
//! | `["user"]`                  | **no**         | **yes**              |
//! | `["assistant"]`             | yes            | no                   |
//! | `["user", "assistant"]`     | yes            | no                   |
//!
//! When *all* blocks are user-only the model receives the placeholder
//! `"[output displayed to user]"`.

use super::*;
use pretty_assertions::assert_eq;
use acp_thread::UserMessageId;
use agent_client_protocol as acp;
use agent_settings::AgentProfileId;
use context_server::types::{
    CallToolResponse, MessageAnnotations, Role as McpRole, ToolResponseContent,
};
use futures::StreamExt;
use language_model::{
    LanguageModelCompletionEvent, LanguageModelToolResult, LanguageModelToolUse, MessageContent,
};
use serde_json::json;

// ---------------------------------------------------------------------------
// Helpers
// ---------------------------------------------------------------------------

fn user_ann() -> Option<MessageAnnotations> {
    Some(MessageAnnotations {
        audience: Some(vec![McpRole::User]),
        priority: None,
    })
}

fn assistant_ann() -> Option<MessageAnnotations> {
    Some(MessageAnnotations {
        audience: Some(vec![McpRole::Assistant]),
        priority: None,
    })
}

fn both_ann() -> Option<MessageAnnotations> {
    Some(MessageAnnotations {
        audience: Some(vec![McpRole::User, McpRole::Assistant]),
        priority: None,
    })
}

fn text_block(text: &str, annotations: Option<MessageAnnotations>) -> ToolResponseContent {
    ToolResponseContent::Text {
        text: text.into(),
        annotations,
    }
}

fn tool_response(content: Vec<ToolResponseContent>) -> CallToolResponse {
    CallToolResponse {
        content,
        is_error: None,
        meta: None,
        structured_content: None,
    }
}

/// Collects any text pushed as user-only display content via ToolCallUpdate.
fn collect_display_texts(
    events: &mut (impl futures::Stream<Item = Result<ThreadEvent>> + Unpin),
) -> Vec<String> {
    let mut texts = Vec::new();
    while let Some(event) = events.next().now_or_never().flatten() {
        if let Ok(ThreadEvent::ToolCallUpdate(acp_thread::ToolCallUpdate::UpdateFields(update))) =
            &event
        {
            if let Some(content) = &update.fields.content {
                for item in content {
                    if let acp::ToolCallContent::Content(c) = item {
                        if let acp::ContentBlock::Text(t) = &c.content {
                            texts.push(t.text.clone());
                        }
                    }
                }
            }
        }
    }
    texts
}

/// Result of [`run_audience_test`] — the bits each test needs to assert on.
struct AudienceResult {
    /// The tool result content the model received.
    model_content: Vec<MessageContent>,
    /// Any text emitted as user-only display updates.
    display_texts: Vec<String>,
}

/// Sets up a fake MCP server with one tool, sends a user message, has the
/// model invoke the tool, delivers `response_blocks` as the tool output,
/// and returns what the model received plus any display-only content.
///
/// Every audience test follows this exact sequence — only the response
/// blocks and assertions differ.
async fn run_audience_test(
    server_name: &'static str,
    tool_name: &'static str,
    response_blocks: Vec<ToolResponseContent>,
    cx: &mut TestAppContext,
) -> AudienceResult {
    let ThreadTest {
        model,
        thread,
        context_server_store,
        fs,
        ..
    } = setup(cx, TestModel::Fake).await;
    let fake_model = model.as_fake();

    fs.insert_file(
        paths::settings_file(),
        json!({
            "agent": {
                "always_allow_tool_actions": true,
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

    let mut mcp_tool_calls = setup_context_server(
        server_name,
        vec![context_server::types::Tool {
            name: tool_name.into(),
            description: Some("test".into()),
            input_schema: json!({"type": "object", "properties": {}}),
            output_schema: None,
            annotations: None,
        }],
        &context_server_store,
        cx,
    );

    let mut events = thread.update(cx, |thread, cx| {
        thread
            .send(UserMessageId::new(), ["call the tool"], cx)
            .unwrap()
    });
    cx.run_until_parked();

    let _initial = fake_model.pending_completions().pop().unwrap();
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(
        LanguageModelToolUse {
            id: "tool_1".into(),
            name: tool_name.into(),
            raw_input: json!({}).to_string(),
            input: json!({}),
            is_input_complete: true,
            thought_signature: None,
        },
    ));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let (_params, responder) = mcp_tool_calls.next().await.unwrap();
    responder.send(tool_response(response_blocks)).unwrap();
    cx.run_until_parked();

    let completion = fake_model
        .pending_completions()
        .pop()
        .expect("no pending completion after tool response");
    let model_content = completion.messages.last().unwrap().content.clone();
    let display_texts = collect_display_texts(&mut events);

    fake_model.send_last_completion_stream_text_chunk("ok");
    fake_model.end_last_completion_stream();

    AudienceResult {
        model_content,
        display_texts,
    }
}

fn expected_tool_result(tool_name: &str, text: &str) -> Vec<MessageContent> {
    vec![MessageContent::ToolResult(LanguageModelToolResult {
        tool_use_id: "tool_1".into(),
        tool_name: tool_name.into(),
        is_error: false,
        content: text.into(),
        output: Some(text.into()),
    })]
}

// ---------------------------------------------------------------------------
// Tests
// ---------------------------------------------------------------------------

/// Mixed response: one user-only block (displayed but hidden from model)
/// and one unannotated block (sent to model).
#[gpui::test]
async fn test_mixed_audience(cx: &mut TestAppContext) {
    let result = run_audience_test(
        "aud_mixed",
        "preview",
        vec![
            text_block("rich preview for the human", user_ann()),
            text_block("dimensions: 800x600", None),
        ],
        cx,
    )
    .await;

    assert_eq!(result.model_content, expected_tool_result("preview", "dimensions: 800x600"));
    assert!(
        result.display_texts.contains(&"rich preview for the human".into()),
        "user-only block should appear as display update, got: {:?}",
        result.display_texts
    );
}

/// Every block is user-only → model gets the placeholder.
#[gpui::test]
async fn test_all_user_only(cx: &mut TestAppContext) {
    let result = run_audience_test(
        "aud_user_only",
        "render",
        vec![
            text_block("image data", user_ann()),
            text_block("more display content", user_ann()),
        ],
        cx,
    )
    .await;

    assert_eq!(
        result.model_content,
        expected_tool_result("render", "[output displayed to user]")
    );
}

/// No annotations at all — baseline. Everything goes to both user and model,
/// no separate display update is emitted.
#[gpui::test]
async fn test_no_annotations(cx: &mut TestAppContext) {
    let result = run_audience_test(
        "aud_none",
        "plain",
        vec![text_block("plain output", None)],
        cx,
    )
    .await;

    assert_eq!(result.model_content, expected_tool_result("plain", "plain output"));
    assert!(
        result.display_texts.is_empty(),
        "no annotation should not produce display update, got: {:?}",
        result.display_texts
    );
}

/// audience: ["assistant"] — content goes to the model, no user display update.
#[gpui::test]
async fn test_model_only(cx: &mut TestAppContext) {
    let result = run_audience_test(
        "aud_model",
        "compute",
        vec![text_block("the answer is 4", assistant_ann())],
        cx,
    )
    .await;

    assert_eq!(result.model_content, expected_tool_result("compute", "the answer is 4"));
    assert!(
        result.display_texts.is_empty(),
        "model-only block should not produce display update, got: {:?}",
        result.display_texts
    );
}

/// audience: ["user","assistant"] — same as no annotation. Content goes to
/// the model and no separate display update is emitted.
#[gpui::test]
async fn test_both_audiences(cx: &mut TestAppContext) {
    let result = run_audience_test(
        "aud_both",
        "lookup",
        vec![text_block("shared content", both_ann())],
        cx,
    )
    .await;

    assert_eq!(result.model_content, expected_tool_result("lookup", "shared content"));
    assert!(
        result.display_texts.is_empty(),
        "both-audience block should not produce display update, got: {:?}",
        result.display_texts
    );
}