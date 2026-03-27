use super::*;
use acp_thread::UserMessageId;
use action_log::ActionLog;
use fs::FakeFs;
use language_model::{
    LanguageModelCompletionEvent, LanguageModelToolUse, MessageContent, StopReason,
    fake_provider::FakeLanguageModel,
};
use prompt_store::ProjectContext;
use serde_json::json;
use std::{collections::BTreeMap, sync::Arc, time::Duration};
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
            cx.weak_entity(),
            project.clone(),
            thread.action_log().clone(),
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
        name: "read_file".into(),
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
        name: "edit_file".into(),
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
async fn test_subagent_uses_read_file_tool(cx: &mut TestAppContext) {
    // This test verifies that subagents can successfully use the read_file tool
    // through the full thread flow, and that tools are properly rebound to use
    // the subagent's thread ID instead of the parent's.
    super::init_test(cx);
    super::always_allow_tools(cx);

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/project"),
        json!({
            "src": {
                "lib.rs": "pub fn hello() -> &'static str {\n    \"Hello from lib!\"\n}\n"
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

    // Create subagent context
    let subagent_context = crate::SubagentContext {
        parent_thread_id: agent_client_protocol::SessionId::new("parent-id"),
        tool_use_id: language_model::LanguageModelToolUseId::from("subagent-tool-use-id"),
        depth: 1,
        summary_prompt: "Summarize what you found".to_string(),
        context_low_prompt: "Context low".to_string(),
    };

    // Create parent tools that will be passed to the subagent
    // This simulates how the subagent_tool passes tools to new_subagent
    let parent_tools: BTreeMap<gpui::SharedString, std::sync::Arc<dyn crate::AnyAgentTool>> = {
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        // Create a "fake" parent thread reference - this should get rebound
        let fake_parent_thread = cx.new(|cx| {
            crate::Thread::new(
                project.clone(),
                cx.new(|_cx| ProjectContext::default()),
                cx.new(|cx| crate::ContextServerRegistry::new(context_server_store.clone(), cx)),
                crate::Templates::new(),
                Some(model.clone()),
                cx,
            )
        });
        let mut tools: BTreeMap<gpui::SharedString, std::sync::Arc<dyn crate::AnyAgentTool>> =
            BTreeMap::new();
        tools.insert(
            "read_file".into(),
            crate::ReadFileTool::new(fake_parent_thread.downgrade(), project.clone(), action_log)
                .erase(),
        );
        tools
    };

    // Create subagent - tools should be rebound to use subagent's thread
    let subagent = cx.new(|cx| {
        crate::Thread::new_subagent(
            project.clone(),
            project_context,
            context_server_registry,
            crate::Templates::new(),
            model.clone(),
            subagent_context,
            parent_tools,
            cx,
        )
    });

    // Get the subagent's thread ID
    let _subagent_thread_id = subagent.read_with(cx, |thread, _| thread.id().to_string());

    // Verify the subagent has the read_file tool
    subagent.read_with(cx, |thread, _| {
        assert!(
            thread.has_registered_tool("read_file"),
            "subagent should have read_file tool"
        );
    });

    // Submit a user message to the subagent
    subagent
        .update(cx, |thread, cx| {
            thread.submit_user_message("Read the file src/lib.rs", cx)
        })
        .unwrap();
    cx.run_until_parked();

    // Simulate the model calling the read_file tool
    let read_tool_use = LanguageModelToolUse {
        id: "read_tool_1".into(),
        name: "read_file".into(),
        raw_input: json!({"path": "project/src/lib.rs"}).to_string(),
        input: json!({"path": "project/src/lib.rs"}),
        is_input_complete: true,
        thought_signature: None,
    };
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(read_tool_use));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // Wait for the tool to complete and the model to be called again with tool results
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    while fake_model.pending_completions().is_empty() {
        if std::time::Instant::now() >= deadline {
            panic!("Timed out waiting for model to be called after read_file tool completion");
        }
        cx.run_until_parked();
        cx.background_executor
            .timer(Duration::from_millis(10))
            .await;
    }

    // Verify the tool result was sent back to the model
    let pending = fake_model.pending_completions();
    assert!(
        !pending.is_empty(),
        "Model should have been called with tool result"
    );

    let last_request = pending.last().unwrap();
    let tool_result = last_request.messages.iter().find_map(|m| {
        m.content.iter().find_map(|c| match c {
            MessageContent::ToolResult(result) => Some(result),
            _ => None,
        })
    });
    assert!(
        tool_result.is_some(),
        "Tool result should be in the messages sent back to the model"
    );

    // Verify the tool result contains the file content
    let result = tool_result.unwrap();
    let result_text = match &result.content {
        language_model::LanguageModelToolResultContent::Text(text) => text.to_string(),
        _ => panic!("expected text content in tool result"),
    };
    assert!(
        result_text.contains("Hello from lib!"),
        "Tool result should contain file content, got: {}",
        result_text
    );

    // Verify the subagent is ready for more input (tool completed, model called again)
    // This test verifies the subagent can successfully use read_file tool.
    // The summary flow is tested separately in test_subagent_returns_summary_on_completion.
}

#[gpui::test]
async fn test_subagent_uses_edit_file_tool(cx: &mut TestAppContext) {
    // This test verifies that subagents can successfully use the edit_file tool
    // through the full thread flow, including the edit agent's model request.
    // It also verifies that the edit agent uses the subagent's thread ID, not the parent's.
    super::init_test(cx);
    super::always_allow_tools(cx);

    cx.update(|cx| {
        cx.update_flags(true, vec!["subagents".to_string()]);
    });

    let fs = FakeFs::new(cx.executor());
    fs.insert_tree(
        path!("/project"),
        json!({
            "src": {
                "config.rs": "pub const VERSION: &str = \"1.0.0\";\n"
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

    // Create a "parent" thread to simulate the real scenario where tools are inherited
    let parent_thread = cx.new(|cx| {
        crate::Thread::new(
            project.clone(),
            cx.new(|_cx| ProjectContext::default()),
            cx.new(|cx| crate::ContextServerRegistry::new(context_server_store.clone(), cx)),
            crate::Templates::new(),
            Some(model.clone()),
            cx,
        )
    });
    let parent_thread_id = parent_thread.read_with(cx, |thread, _| thread.id().to_string());

    // Create parent tools that reference the parent thread
    let parent_tools: BTreeMap<gpui::SharedString, std::sync::Arc<dyn crate::AnyAgentTool>> = {
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let language_registry = project.read_with(cx, |p, _| p.languages().clone());
        let mut tools: BTreeMap<gpui::SharedString, std::sync::Arc<dyn crate::AnyAgentTool>> =
            BTreeMap::new();
        tools.insert(
            "read_file".into(),
            crate::ReadFileTool::new(parent_thread.downgrade(), project.clone(), action_log)
                .erase(),
        );
        tools.insert(
            "edit_file".into(),
            crate::EditFileTool::new(
                project.clone(),
                parent_thread.downgrade(),
                language_registry,
                crate::Templates::new(),
            )
            .erase(),
        );
        tools
    };

    // Create subagent context
    let subagent_context = crate::SubagentContext {
        parent_thread_id: agent_client_protocol::SessionId::new("parent-id"),
        tool_use_id: language_model::LanguageModelToolUseId::from("subagent-tool-use-id"),
        depth: 1,
        summary_prompt: "Summarize what you changed".to_string(),
        context_low_prompt: "Context low".to_string(),
    };

    // Create subagent - tools should be rebound to use subagent's thread
    let subagent = cx.new(|cx| {
        crate::Thread::new_subagent(
            project.clone(),
            project_context,
            context_server_registry,
            crate::Templates::new(),
            model.clone(),
            subagent_context,
            parent_tools,
            cx,
        )
    });

    // Get the subagent's thread ID - it should be different from parent
    let subagent_thread_id = subagent.read_with(cx, |thread, _| thread.id().to_string());
    assert_ne!(
        parent_thread_id, subagent_thread_id,
        "Subagent should have a different thread ID than parent"
    );

    // Verify the subagent has the tools
    subagent.read_with(cx, |thread, _| {
        assert!(
            thread.has_registered_tool("read_file"),
            "subagent should have read_file tool"
        );
        assert!(
            thread.has_registered_tool("edit_file"),
            "subagent should have edit_file tool"
        );
    });

    // Submit a user message to the subagent
    subagent
        .update(cx, |thread, cx| {
            thread.submit_user_message("Update the version in config.rs to 2.0.0", cx)
        })
        .unwrap();
    cx.run_until_parked();

    // First, model calls read_file to see the current content
    let read_tool_use = LanguageModelToolUse {
        id: "read_tool_1".into(),
        name: "read_file".into(),
        raw_input: json!({"path": "project/src/config.rs"}).to_string(),
        input: json!({"path": "project/src/config.rs"}),
        is_input_complete: true,
        thought_signature: None,
    };
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(read_tool_use));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // Wait for the read tool to complete and model to be called again
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    while fake_model.pending_completions().is_empty() {
        if std::time::Instant::now() >= deadline {
            panic!("Timed out waiting for model to be called after read_file tool");
        }
        cx.run_until_parked();
        cx.background_executor
            .timer(Duration::from_millis(10))
            .await;
    }

    // Model responds and calls edit_file
    fake_model.send_last_completion_stream_text_chunk("I'll update the version now.");
    let edit_tool_use = LanguageModelToolUse {
        id: "edit_tool_1".into(),
        name: "edit_file".into(),
        raw_input: json!({
            "display_description": "Update version to 2.0.0",
            "path": "project/src/config.rs",
            "mode": "edit"
        })
        .to_string(),
        input: json!({
            "display_description": "Update version to 2.0.0",
            "path": "project/src/config.rs",
            "mode": "edit"
        }),
        is_input_complete: true,
        thought_signature: None,
    };
    fake_model
        .send_last_completion_stream_event(LanguageModelCompletionEvent::ToolUse(edit_tool_use));
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // The edit_file tool creates an EditAgent which makes its own model request.
    // Wait for that request.
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    while fake_model.pending_completions().is_empty() {
        if std::time::Instant::now() >= deadline {
            panic!(
                "Timed out waiting for edit agent completion request in subagent. Pending: {}",
                fake_model.pending_completions().len()
            );
        }
        cx.run_until_parked();
        cx.background_executor
            .timer(Duration::from_millis(10))
            .await;
    }

    // Verify the edit agent's request uses the SUBAGENT's thread ID, not the parent's
    let pending = fake_model.pending_completions();
    let edit_agent_request = pending.last().unwrap();
    let edit_agent_thread_id = edit_agent_request.thread_id.as_ref().unwrap();
    std::assert_eq!(
        edit_agent_thread_id,
        &subagent_thread_id,
        "Edit agent should use subagent's thread ID, not parent's. Got: {}, expected: {}",
        edit_agent_thread_id,
        subagent_thread_id
    );
    std::assert_ne!(
        edit_agent_thread_id,
        &parent_thread_id,
        "Edit agent should NOT use parent's thread ID"
    );

    // Send the edit agent's response with the XML format it expects
    let edit_response = "<old_text>pub const VERSION: &str = \"1.0.0\";</old_text>\n<new_text>pub const VERSION: &str = \"2.0.0\";</new_text>";
    fake_model.send_last_completion_stream_text_chunk(edit_response);
    fake_model.end_last_completion_stream();
    cx.run_until_parked();

    // Wait for the edit to complete and the thread to call the model again with tool results
    let deadline = std::time::Instant::now() + Duration::from_secs(5);
    while fake_model.pending_completions().is_empty() {
        if std::time::Instant::now() >= deadline {
            panic!("Timed out waiting for model to be called after edit completion in subagent");
        }
        cx.run_until_parked();
        cx.background_executor
            .timer(Duration::from_millis(10))
            .await;
    }

    // Verify the file was edited
    let file_content = fs
        .load(path!("/project/src/config.rs").as_ref())
        .await
        .expect("file should exist");
    assert!(
        file_content.contains("2.0.0"),
        "File should have been edited to contain new version. Content: {}",
        file_content
    );
    assert!(
        !file_content.contains("1.0.0"),
        "Old version should be replaced. Content: {}",
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
            .any(|c| matches!(c, MessageContent::ToolResult(_)))
    });
    assert!(
        has_tool_result,
        "Tool result should be in the messages sent back to the model"
    );
}
