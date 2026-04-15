use super::*;
use crate::{AgentTool, EditFileTool, ReadFileTool};
use acp_thread::UserMessageId;
use fs::FakeFs;
use language_model::{
    LanguageModelCompletionEvent, LanguageModelToolUse, StopReason,
    fake_provider::FakeLanguageModel,
};
use prompt_store::ProjectContext;
use serde_json::json;
use std::{sync::Arc, time::Duration};
use util::path;

#[gpui::test]
async fn test_edit_file_tool_in_thread_context(cx: &mut TestAppContext) {
    // This test verifies that the edit_file tool works correctly when invoked
    // through the full thread flow (model sends ToolUse event -> tool runs -> result sent back).
    // This is different from tests that call tool.run() directly.
    super::init_test(cx);
    super::always_allow_tools(cx);

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/project"),
        json!({
            "src": {
                "main.rs": "fn main() {\n    println!(\"Hello, world!\");\n}\n"
            }
        }),
    )
    .await;

    let project = project::Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| crate::ContextServerRegistry::new(context_server_store.clone(), cx));
    let model = Arc::new(FakeLanguageModel::default());
    let fake_model = model.as_fake();

    let thread = cx.new(|cx| {
        let mut thread = crate::Thread::new(
            project.clone(),
            project_context,
            context_server_registry,
            crate::Templates::new(),
            Some(model.clone()),
            cx,
        );
        // Add just the tools we need for this test
        let language_registry = project.read(cx).languages().clone();
        thread.add_tool(crate::ReadFileTool::new(
            project.clone(),
            thread.action_log().clone(),
            true,
        ));
        thread.add_tool(crate::EditFileTool::new(
            project.clone(),
            cx.weak_entity(),
            language_registry,
            crate::Templates::new(),
        ));
        thread
    });

    // First, read the file so the thread knows about its contents
    let _events = thread
        .update(cx, |thread, cx| {
            thread.send(UserMessageId::new(), ["Read the file src/main.rs"], cx)
        })
        .unwrap();
    cx.run_until_parked();

    // Model calls read_file tool
    let read_tool_use = LanguageModelToolUse {
        id: "read_tool_1".into(),
        name: ReadFileTool::NAME.into(),
        raw_input: json!({"path": "project/src/main.rs"}).to_string(),
        input: json!({"path": "project/src/main.rs"}),
        is_input_complete: true,
        thought_signature: None,
    };
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(read_tool_use));
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::ToolUse));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // Wait for the read tool to complete and model to be called again
    while fake_model.pending_completions().is_empty() {
        cx.run_until_parked();
    }

    // Model responds after seeing the file content, then calls edit_file
    fake_model.send_last_completion_stream_text_chunk("I'll edit the file now.");
    let edit_tool_use = LanguageModelToolUse {
        id: "edit_tool_1".into(),
        name: EditFileTool::NAME.into(),
        raw_input: json!({
            "display_description": "Change greeting message",
            "path": "project/src/main.rs",
            "mode": "edit"
        })
        .to_string(),
        input: json!({
            "display_description": "Change greeting message",
            "path": "project/src/main.rs",
            "mode": "edit"
        }),
        is_input_complete: true,
        thought_signature: None,
    };
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(edit_tool_use));
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::ToolUse));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // The edit_file tool creates an EditAgent which makes its own model request.
    // We need to respond to that request with the edit instructions.
    // Wait for the edit agent's completion request
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    while fake_model.pending_completions().is_empty() {
        if std::time::Instant::now() >= deadline {
            panic!(
                "Timed out waiting for edit agent completion request. Pending: {}",
                fake_model.pending_completions().len()
            );
        }
        cx.run_until_parked();
        cx.background_executor
            .timer(Duration::from_millis(10))
            .await;
    }

    // Send the edit agent's response with the XML format it expects
    let edit_response = "<old_text>println!(\"Hello, world!\");</old_text>\n<new_text>println!(\"Hello, Zed!\");</new_text>";
    fake_model.send_last_completion_stream_text_chunk(edit_response);
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // Wait for the edit to complete and the thread to call the model again with tool results
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    while fake_model.pending_completions().is_empty() {
        if std::time::Instant::now() >= deadline {
            panic!("Timed out waiting for model to be called after edit completion");
        }
        cx.run_until_parked();
        cx.background_executor
            .timer(Duration::from_millis(10))
            .await;
    }

    // Verify the file was edited
    let file_content = fs
        .load(path!("/project/src/main.rs").as_ref())
        .await
        .expect("file should exist");
    assert!(
        file_content.contains("Hello, Zed!"),
        "File should have been edited. Content: {}",
        file_content
    );
    assert!(
        !file_content.contains("Hello, world!"),
        "Old content should be replaced. Content: {}",
        file_content
    );

    // Verify the tool result was sent back to the model
    let pending = fake_model.pending_completions();
    assert!(
        !pending.is_empty(),
        "Model should have been called with tool result"
    );

    let last_request = pending.last().unwrap();
    let has_tool_result = last_request.messages.iter().any(|m| {
        m.content
            .iter()
            .any(|c| matches!(c, language_model::MessageContent::ToolResult(_)))
    });
    assert!(
        has_tool_result,
        "Tool result should be in the messages sent back to the model"
    );

    // Complete the turn
    fake_model.send_last_completion_stream_text_chunk("I've updated the greeting message.");
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::EndTurn));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // Verify the thread completed successfully
    thread.update(cx, |thread, _cx| {
        assert!(
            thread.is_turn_complete(),
            "Thread should be complete after the turn ends"
        );
    });
}

#[gpui::test]
async fn test_streaming_edit_json_parse_error_does_not_cause_unsaved_changes(
    cx: &mut TestAppContext,
) {
    super::init_test(cx);
    super::always_allow_tools(cx);

    // Enable the streaming edit file tool feature flag.
    cx.update(|cx| {
        cx.update_flags(true, vec!["streaming-edit-file-tool".to_string()]);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/project"),
        json!({
            "src": {
                "main.rs": "fn main() {\n    println!(\"Hello, world!\");\n}\n"
            }
        }),
    )
    .await;

    let project = project::Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
    let project_context = cx.new(|_cx| ProjectContext::default());
    let context_server_store = project.read_with(cx, |project, _| project.context_server_store());
    let context_server_registry =
        cx.new(|cx| crate::ContextServerRegistry::new(context_server_store.clone(), cx));
    let model = Arc::new(FakeLanguageModel::default());
    model.as_fake().set_supports_streaming_tools(true);
    let fake_model = model.as_fake();

    let thread = cx.new(|cx| {
        let mut thread = crate::Thread::new(
            project.clone(),
            project_context,
            context_server_registry,
            crate::Templates::new(),
            Some(model.clone()),
            cx,
        );
        let language_registry = project.read(cx).languages().clone();
        thread.add_tool(crate::StreamingEditFileTool::new(
            project.clone(),
            cx.weak_entity(),
            thread.action_log().clone(),
            language_registry,
        ));
        thread
    });

    let _events = thread
        .update(cx, |thread, cx| {
            thread.send(
                UserMessageId::new(),
                ["Write new content to src/main.rs"],
                cx,
            )
        })
        .unwrap();
    cx.run_until_parked();

    let tool_use_id = "edit_1";
    let partial_1 = LanguageModelToolUse {
        id: tool_use_id.into(),
        name: EditFileTool::NAME.into(),
        raw_input: json!({
            "display_description": "Rewrite main.rs",
            "path": "project/src/main.rs",
            "mode": "write"
        })
        .to_string(),
        input: json!({
            "display_description": "Rewrite main.rs",
            "path": "project/src/main.rs",
            "mode": "write"
        }),
        is_input_complete: false,
        thought_signature: None,
    };
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(partial_1));
    cx.run_until_parked();

    let partial_2 = LanguageModelToolUse {
        id: tool_use_id.into(),
        name: EditFileTool::NAME.into(),
        raw_input: json!({
            "display_description": "Rewrite main.rs",
            "path": "project/src/main.rs",
            "mode": "write",
            "content": "fn main() { /* rewritten */ }"
        })
        .to_string(),
        input: json!({
            "display_description": "Rewrite main.rs",
            "path": "project/src/main.rs",
            "mode": "write",
            "content": "fn main() { /* rewritten */ }"
        }),
        is_input_complete: false,
        thought_signature: None,
    };
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(partial_2));
    cx.run_until_parked();

    // Now send a json parse error. At this point we have started writing content to the buffer.
    fake_model.send_last_completion_stream_event(
        LanguageModelCompletionEvent::ToolUseJsonParseError {
            id: tool_use_id.into(),
            tool_name: EditFileTool::NAME.into(),
            raw_input: r#"{"display_description":"Rewrite main.rs","path":"project/src/main.rs","mode":"write","content":"fn main() { /* rewritten "#.into(),
            json_parse_error: "EOF while parsing a string at line 1 column 95".into(),
        },
    );
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::ToolUse));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // cx.executor().advance_clock(Duration::from_secs(5));
    // cx.run_until_parked();

    assert!(
        !fake_model.pending_completions().is_empty(),
        "Thread should have retried after the error"
    );

    // Respond with a new, well-formed, complete edit_file tool use.
    let tool_use = LanguageModelToolUse {
        id: "edit_2".into(),
        name: EditFileTool::NAME.into(),
        raw_input: json!({
            "display_description": "Rewrite main.rs",
            "path": "project/src/main.rs",
            "mode": "write",
            "content": "fn main() {\n    println!(\"Hello, rewritten!\");\n}\n"
        })
        .to_string(),
        input: json!({
            "display_description": "Rewrite main.rs",
            "path": "project/src/main.rs",
            "mode": "write",
            "content": "fn main() {\n    println!(\"Hello, rewritten!\");\n}\n"
        }),
        is_input_complete: true,
        thought_signature: None,
    };
    fake_model.send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(tool_use));
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::Stop(StopReason::ToolUse));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    let pending_completions = fake_model.pending_completions();
    assert!(
        pending_completions.len() == 1,
        "Expected only the follow-up completion containing the successful tool result"
    );

    let completion = pending_completions
        .into_iter()
        .last()
        .expect("Expected a completion containing the tool result for edit_2");

    let tool_result = completion
        .messages
        .iter()
        .flat_map(|msg| &msg.content)
        .find_map(|content| match content {
            language_model::MessageContent::ToolResult(result)
                if result.tool_use_id == language_model::LanguageModelToolUseId::from("edit_2") =>
            {
                Some(result)
            }
            _ => None,
        })
        .expect("Should have a tool result for edit_2");

    // Ensure that the second tool call completed successfully and edits were applied.
    assert!(
        !tool_result.is_error,
        "Tool result should succeed, got: {:?}",
        tool_result
    );
    let content_text = match &tool_result.content {
        language_model::LanguageModelToolResultContent::Text(t) => t.to_string(),
        other => panic!("Expected text content, got: {:?}", other),
    };
    assert!(
        !content_text.contains("file has been modified since you last read it"),
        "Did not expect a stale last-read error, got: {content_text}"
    );
    assert!(
        !content_text.contains("This file has unsaved changes"),
        "Did not expect an unsaved-changes error, got: {content_text}"
    );

    let file_content = fs
        .load(path!("/project/src/main.rs").as_ref())
        .await
        .expect("file should exist");
    super::assert_eq!(
        file_content,
        "fn main() {\n    println!(\"Hello, rewritten!\");\n}\n",
        "The second edit should be applied and saved gracefully"
    );

    fake_model.end_last_completion_stream();
    cx.run_until_parked();
}
