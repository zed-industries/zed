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
        thread.add_tool(
            crate::ReadFileTool::new(
                cx.weak_entity(),
                project.clone(),
                thread.action_log().clone(),
            ),
            None,
        );
        thread.add_tool(
            crate::EditFileTool::new(
                project.clone(),
                cx.weak_entity(),
                language_registry,
                crate::Templates::new(),
            ),
            None,
        );
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
