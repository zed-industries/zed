use crate::schema::json_schema_for;
use anyhow::Result;
use assistant_tool::{ActionLog, Tool, ToolResult};
use gpui::{AnyWindowHandle, App, Entity, Task};
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::fmt::Write as _;
use std::sync::Arc;
use ui::IconName;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ProjectUpdatesToolInput {}

pub struct ProjectNotificationsTool;

impl Tool for ProjectNotificationsTool {
    fn name(&self) -> String {
        "project_notifications".to_string()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }
    fn may_perform_edits(&self) -> bool {
        false
    }
    fn description(&self) -> String {
        include_str!("./project_notifications_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::Envelope
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<ProjectUpdatesToolInput>(format)
    }

    fn ui_text(&self, _input: &serde_json::Value) -> String {
        "Check project notifications".into()
    }

    fn run(
        self: Arc<Self>,
        _input: serde_json::Value,
        _request: Arc<LanguageModelRequest>,
        _project: Entity<Project>,
        action_log: Entity<ActionLog>,
        _model: Arc<dyn LanguageModel>,
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let mut stale_files = String::new();
        let mut notified_buffers = Vec::new();

        for stale_file in action_log.read(cx).unnotified_stale_buffers(cx) {
            if let Some(file) = stale_file.read(cx).file() {
                writeln!(&mut stale_files, "- {}", file.path().display()).ok();
                notified_buffers.push(stale_file.clone());
            }
        }

        if !notified_buffers.is_empty() {
            action_log.update(cx, |log, cx| {
                log.mark_buffers_as_notified(notified_buffers, cx);
            });
        }

        let response = if stale_files.is_empty() {
            "No new notifications".to_string()
        } else {
            // NOTE: Changes to this prompt require a symmetric update in the LLM Worker
            const HEADER: &str = include_str!("./project_notifications_tool/prompt_header.txt");
            format!("{HEADER}{stale_files}").replace("\r\n", "\n")
        };

        Task::ready(Ok(response.into())).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assistant_tool::ToolResultContent;
    use gpui::{AppContext, TestAppContext};
    use language_model::{LanguageModelRequest, fake_provider::FakeLanguageModelProvider};
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use std::sync::Arc;
    use util::path;

    #[gpui::test]
    async fn test_stale_buffer_notification(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            path!("/test"),
            json!({"code.rs": "fn main() {\n    println!(\"Hello, world!\");\n}"}),
        )
        .await;

        let project = Project::test(fs, [path!("/test").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));

        let buffer_path = project
            .read_with(cx, |project, cx| {
                project.find_project_path("test/code.rs", cx)
            })
            .unwrap();

        let buffer = project
            .update(cx, |project, cx| {
                project.open_buffer(buffer_path.clone(), cx)
            })
            .await
            .unwrap();

        // Start tracking the buffer
        action_log.update(cx, |log, cx| {
            log.buffer_read(buffer.clone(), cx);
        });

        // Run the tool before any changes
        let tool = Arc::new(ProjectNotificationsTool);
        let provider = Arc::new(FakeLanguageModelProvider);
        let model: Arc<dyn LanguageModel> = Arc::new(provider.test_model());
        let request = Arc::new(LanguageModelRequest::default());
        let tool_input = json!({});

        let result = cx.update(|cx| {
            tool.clone().run(
                tool_input.clone(),
                request.clone(),
                project.clone(),
                action_log.clone(),
                model.clone(),
                None,
                cx,
            )
        });

        let response = result.output.await.unwrap();
        let response_text = match &response.content {
            ToolResultContent::Text(text) => text.clone(),
            _ => panic!("Expected text response"),
        };
        assert_eq!(
            response_text.as_str(),
            "No new notifications",
            "Tool should return 'No new notifications' when no stale buffers"
        );

        // Modify the buffer (makes it stale)
        buffer.update(cx, |buffer, cx| {
            buffer.edit([(1..1, "\nChange!\n")], None, cx);
        });

        // Run the tool again
        let result = cx.update(|cx| {
            tool.clone().run(
                tool_input.clone(),
                request.clone(),
                project.clone(),
                action_log.clone(),
                model.clone(),
                None,
                cx,
            )
        });

        // This time the buffer is stale, so the tool should return a notification
        let response = result.output.await.unwrap();
        let response_text = match &response.content {
            ToolResultContent::Text(text) => text.clone(),
            _ => panic!("Expected text response"),
        };

        let expected_content = "[The following is an auto-generated notification; do not reply]\n\nThese files have changed since the last read:\n- code.rs\n";
        assert_eq!(
            response_text.as_str(),
            expected_content,
            "Tool should return the stale buffer notification"
        );

        // Run the tool once more without any changes - should get no new notifications
        let result = cx.update(|cx| {
            tool.run(
                tool_input.clone(),
                request.clone(),
                project.clone(),
                action_log,
                model.clone(),
                None,
                cx,
            )
        });

        let response = result.output.await.unwrap();
        let response_text = match &response.content {
            ToolResultContent::Text(text) => text.clone(),
            _ => panic!("Expected text response"),
        };

        assert_eq!(
            response_text.as_str(),
            "No new notifications",
            "Tool should return 'No new notifications' when running again without changes"
        );
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
            assistant_tool::init(cx);
        });
    }
}
