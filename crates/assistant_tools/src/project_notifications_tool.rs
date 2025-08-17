use crate::schema::json_schema_for;
use action_log::ActionLog;
use anyhow::Result;
use assistant_tool::{Tool, ToolResult};
use gpui::{AnyWindowHandle, App, Entity, Task};
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt::Write, sync::Arc};
use ui::IconName;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ProjectUpdatesToolInput {}

pub struct ProjectNotificationsTool;

impl Tool for ProjectNotificationsTool {
    fn name(&self) -> String {
        "project_notifications".to_string()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &Entity<Project>, _: &App) -> bool {
        false
    }
    fn may_perform_edits(&self) -> bool {
        false
    }
    fn description(&self) -> String {
        include_str!("./project_notifications_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::ToolNotification
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
        let Some(user_edits_diff) =
            action_log.update(cx, |log, cx| log.flush_unnotified_user_edits(cx))
        else {
            return result("No new notifications");
        };

        // NOTE: Changes to this prompt require a symmetric update in the LLM Worker
        const HEADER: &str = include_str!("./project_notifications_tool/prompt_header.txt");
        const MAX_BYTES: usize = 8000;
        let diff = fit_patch_to_size(&user_edits_diff, MAX_BYTES);
        result(&format!("{HEADER}\n\n```diff\n{diff}\n```\n").replace("\r\n", "\n"))
    }
}

fn result(response: &str) -> ToolResult {
    Task::ready(Ok(response.to_string().into())).into()
}

/// Make sure that the patch fits into the size limit (in bytes).
/// Compress the patch by omitting some parts if needed.
/// Unified diff format is assumed.
fn fit_patch_to_size(patch: &str, max_size: usize) -> String {
    if patch.len() <= max_size {
        return patch.to_string();
    }

    // Compression level 1: remove context lines in diff bodies, but
    // leave the counts and positions of inserted/deleted lines
    let mut current_size = patch.len();
    let mut file_patches = split_patch(&patch);
    file_patches.sort_by_key(|patch| patch.len());
    let compressed_patches = file_patches
        .iter()
        .rev()
        .map(|patch| {
            if current_size > max_size {
                let compressed = compress_patch(patch).unwrap_or_else(|_| patch.to_string());
                current_size -= patch.len() - compressed.len();
                compressed
            } else {
                patch.to_string()
            }
        })
        .collect::<Vec<_>>();

    if current_size <= max_size {
        return compressed_patches.join("\n\n");
    }

    // Compression level 2: list paths of the changed files only
    let filenames = file_patches
        .iter()
        .map(|patch| {
            let patch = diffy::Patch::from_str(patch).unwrap();
            let path = patch
                .modified()
                .and_then(|path| path.strip_prefix("b/"))
                .unwrap_or_default();
            format!("- {path}\n")
        })
        .collect::<Vec<_>>();

    filenames.join("")
}

/// Split a potentially multi-file patch into multiple single-file patches
fn split_patch(patch: &str) -> Vec<String> {
    let mut result = Vec::new();
    let mut current_patch = String::new();

    for line in patch.lines() {
        if line.starts_with("---") && !current_patch.is_empty() {
            result.push(current_patch.trim_end_matches('\n').into());
            current_patch = String::new();
        }
        current_patch.push_str(line);
        current_patch.push('\n');
    }

    if !current_patch.is_empty() {
        result.push(current_patch.trim_end_matches('\n').into());
    }

    result
}

fn compress_patch(patch: &str) -> anyhow::Result<String> {
    let patch = diffy::Patch::from_str(patch)?;
    let mut out = String::new();

    writeln!(out, "--- {}", patch.original().unwrap_or("a"))?;
    writeln!(out, "+++ {}", patch.modified().unwrap_or("b"))?;

    for hunk in patch.hunks() {
        writeln!(out, "@@ -{} +{} @@", hunk.old_range(), hunk.new_range())?;
        writeln!(out, "[...skipped...]")?;
    }

    Ok(out)
}

#[cfg(test)]
mod tests {
    use super::*;
    use assistant_tool::ToolResultContent;
    use gpui::{AppContext, TestAppContext};
    use indoc::indoc;
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
        cx.run_until_parked();

        // Run the tool before any changes
        let tool = Arc::new(ProjectNotificationsTool);
        let provider = Arc::new(FakeLanguageModelProvider::default());
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
        cx.run_until_parked();

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
        cx.run_until_parked();

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
        cx.run_until_parked();

        // This time the buffer is stale, so the tool should return a notification
        let response = result.output.await.unwrap();
        let response_text = match &response.content {
            ToolResultContent::Text(text) => text.clone(),
            _ => panic!("Expected text response"),
        };

        assert!(
            response_text.contains("These files have changed"),
            "Tool should return the stale buffer notification"
        );
        assert!(
            response_text.contains("test/code.rs"),
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
        cx.run_until_parked();

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

    #[test]
    fn test_patch_compression() {
        // Given a patch that doesn't fit into the size budget
        let patch = indoc! {"
       --- a/dir/test.txt
       +++ b/dir/test.txt
       @@ -1,3 +1,3 @@
        line 1
       -line 2
       +CHANGED
        line 3
       @@ -10,2 +10,2 @@
        line 10
       -line 11
       +line eleven


       --- a/dir/another.txt
       +++ b/dir/another.txt
       @@ -100,1 +1,1 @@
       -before
       +after
       "};

        // When the size deficit can be compensated by dropping the body,
        // then the body should be trimmed for larger files first
        let limit = patch.len() - 10;
        let compressed = fit_patch_to_size(patch, limit);
        let expected = indoc! {"
       --- a/dir/test.txt
       +++ b/dir/test.txt
       @@ -1,3 +1,3 @@
       [...skipped...]
       @@ -10,2 +10,2 @@
       [...skipped...]


       --- a/dir/another.txt
       +++ b/dir/another.txt
       @@ -100,1 +1,1 @@
       -before
       +after"};
        assert_eq!(compressed, expected);

        // When the size deficit is too large, then only file paths
        // should be returned
        let limit = 10;
        let compressed = fit_patch_to_size(patch, limit);
        let expected = indoc! {"
       - dir/another.txt
       - dir/test.txt
       "};
        assert_eq!(compressed, expected);
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
