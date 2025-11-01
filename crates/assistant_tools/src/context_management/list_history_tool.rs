use std::sync::Arc;

use crate::schema::json_schema_for;
use action_log::ActionLog;
use anyhow::{Result, anyhow};
use assistant_tool::{Tool, ToolResult, ToolResultOutput};
use gpui::{AnyWindowHandle, App, AppContext, Entity, Task};
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelToolSchemaFormat};

use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use ui::IconName;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListHistoryToolInput {
    /// Inclusive starting message index (default 0)
    #[serde(default)]
    start: usize,

    /// Number of messages to enumerate (default 40, clamped 1..500)
    #[serde(default = "default_limit")]
    limit: usize,

    /// Preview character cap per message (default 160, clamped 16..4096)
    #[serde(default = "default_max_chars")]
    max_chars_per_message: usize,

    /// If true, appends full text of each listed message after the table
    #[serde(default)]
    include_full_markdown: bool,
}

fn default_limit() -> usize {
    40
}

fn default_max_chars() -> usize {
    160
}

pub struct ListHistoryTool;

impl Tool for ListHistoryTool {
    fn name(&self) -> String {
        "list_history".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &Entity<Project>, _: &App) -> bool {
        false
    }

    fn may_perform_edits(&self) -> bool {
        false
    }

    fn description(&self) -> String {
        "Enumerate a slice of the thread's messages with stable indices, lightweight previews, and optional full markdown. Used for planning context reduction.".into()
    }

    fn icon(&self) -> IconName {
        IconName::ListTree
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<ListHistoryToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        let input: ListHistoryToolInput =
            serde_json::from_value(input.clone()).unwrap_or(ListHistoryToolInput {
                start: 0,
                limit: default_limit(),
                max_chars_per_message: default_max_chars(),
                include_full_markdown: false,
            });
        format!(
            "List history (start: {}, limit: {})",
            input.start, input.limit
        )
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        request: Arc<LanguageModelRequest>,
        _project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        _model: Arc<dyn LanguageModel>,
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let mut input: ListHistoryToolInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        // Clamp values to valid ranges
        input.limit = input.limit.clamp(1, 500);
        input.max_chars_per_message = input.max_chars_per_message.clamp(16, 4096);

        // Get the messages from the request
        let messages = request.messages.clone();

        let task = cx.background_spawn(async move {
            let mut output = String::new();

            // Header
            output.push_str("# Conversation History\n\n");

            // Get messages from the request
            let total_messages = messages.len();
            let end_index = (input.start + input.limit).min(total_messages);

            if input.start >= total_messages {
                output.push_str(&format!(
                    "No messages found starting from index {}\n",
                    input.start
                ));
                output.push_str(&format!(
                    "Total messages in conversation: {}\n",
                    total_messages
                ));
                return Ok(ToolResultOutput::from(output));
            }

            // JSON summary block
            output.push_str("```json\n");
            output.push_str(
                &serde_json::to_string_pretty(&serde_json::json!({
                    "total_messages": total_messages,
                    "showing_range": format!("{}..{}", input.start, end_index),
                    "messages_shown": end_index - input.start,
                }))
                .unwrap_or_default(),
            );
            output.push_str("\n```\n\n");

            // Table header
            output.push_str("| Idx | Role | Kind | Chars | Preview |\n");
            output.push_str("|-----|------|------|-------|----------|\n");

            // Build table rows
            let messages_slice = &messages[input.start..end_index];
            for (offset, message) in messages_slice.iter().enumerate() {
                let idx = input.start + offset;
                let role = format!("{:?}", message.role);

                // Convert message content to string
                let content_str = message
                    .content
                    .iter()
                    .map(|c| match c {
                        language_model::MessageContent::Text(text) => text.clone(),
                        language_model::MessageContent::Thinking { text, .. } => text.clone(),
                        language_model::MessageContent::RedactedThinking(text) => text.clone(),
                        language_model::MessageContent::Image(_) => "[Image]".to_string(),
                        language_model::MessageContent::ToolUse(_) => "[Tool Use]".to_string(),
                        language_model::MessageContent::ToolResult(_) => {
                            "[Tool Result]".to_string()
                        }
                    })
                    .collect::<Vec<_>>()
                    .join("\n");
                let chars = content_str.len();

                // Determine message kind based on content
                let has_tool_use = message
                    .content
                    .iter()
                    .any(|c| matches!(c, language_model::MessageContent::ToolUse(_)));
                let has_tool_result = message
                    .content
                    .iter()
                    .any(|c| matches!(c, language_model::MessageContent::ToolResult(_)));

                let kind = if has_tool_use {
                    "tool_use"
                } else if has_tool_result {
                    "tool_result"
                } else {
                    "message"
                };

                // Create preview
                let preview = if content_str.len() <= input.max_chars_per_message {
                    content_str.clone()
                } else {
                    format!("{}...", &content_str[..input.max_chars_per_message])
                };

                // Escape pipe characters in preview for markdown table
                let preview = preview.replace('|', "\\|").replace('\n', " ");

                output.push_str(&format!(
                    "| {} | {} | {} | {} | {} |\n",
                    idx, role, kind, chars, preview
                ));
            }

            // Optionally include full markdown
            if input.include_full_markdown {
                output.push_str("\n## Full Message Content\n\n");

                for (offset, message) in messages_slice.iter().enumerate() {
                    let idx = input.start + offset;
                    output.push_str(&format!(
                        "### Message {} ({})\n\n",
                        idx,
                        format!("{:?}", message.role)
                    ));

                    let content_str = message
                        .content
                        .iter()
                        .map(|c| match c {
                            language_model::MessageContent::Text(text) => text.clone(),
                            language_model::MessageContent::Thinking { text, .. } => text.clone(),
                            language_model::MessageContent::RedactedThinking(text) => text.clone(),
                            language_model::MessageContent::Image(_) => "[Image]".to_string(),
                            language_model::MessageContent::ToolUse(_) => "[Tool Use]".to_string(),
                            language_model::MessageContent::ToolResult(_) => {
                                "[Tool Result]".to_string()
                            }
                        })
                        .collect::<Vec<_>>()
                        .join("\n");
                    output.push_str(&content_str);
                    output.push_str("\n\n");
                }
            }

            Ok(ToolResultOutput::from(output))
        });

        ToolResult {
            output: task,
            card: None,
        }
    }
}
