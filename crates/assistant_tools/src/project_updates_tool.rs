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

pub struct ProjectUpdatesTool;

impl Tool for ProjectUpdatesTool {
    fn name(&self) -> String {
        "project_updates".to_string()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }
    fn may_perform_edits(&self) -> bool {
        false
    }
    fn description(&self) -> String {
        include_str!("./project_updates_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::Update
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<ProjectUpdatesToolInput>(format)
    }

    fn ui_text(&self, _input: &serde_json::Value) -> String {
        "Check for project updates".into()
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

        let action_log = action_log.read(cx);

        for stale_file in action_log.stale_buffers(cx) {
            if let Some(file) = stale_file.read(cx).file() {
                writeln!(&mut stale_files, "- {}", file.path().display()).ok();
            }
        }

        let response = if stale_files.is_empty() {
            "Files up-to-date".to_string()
        } else {
            // NOTE: Changes to this prompt require a symmetric update in the LLM Worker
            const HEADER: &str = include_str!("./project_updates_tool/prompt_header.txt");
            format!("{HEADER}{stale_files}").replace("\r\n", "\n")
        };

        Task::ready(Ok(response.into())).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use project::{FakeFs, Project};
    use settings::SettingsStore;
    use std::path::Path;
    use tempfile::TempDir;

    #[gpui::test]
    async fn test_to_absolute_path(cx: &mut TestAppContext) {
        init_test(cx);
        let temp_dir = TempDir::new().expect("Failed to create temp directory");
        let temp_path = temp_dir.path().to_string_lossy().to_string();

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            &temp_path,
            serde_json::json!({
                "src": {
                    "main.rs": "fn main() {}",
                    "lib.rs": "pub fn lib_fn() {}"
                },
                "docs": {
                    "readme.md": "# Project Documentation"
                }
            }),
        )
        .await;

        // Use the temp_path as the root directory, not just its filename
        let project = Project::test(fs.clone(), [temp_dir.path()], cx).await;

        // Test cases where the function should return Some
        cx.update(|cx| {
            // Project-relative paths should return Some
            // Create paths using the last segment of the temp path to simulate a project-relative path
            let root_dir_name = Path::new(&temp_path)
                .file_name()
                .unwrap_or_else(|| std::ffi::OsStr::new("temp"))
                .to_string_lossy();

            assert!(
                to_absolute_path(&format!("{root_dir_name}/src/main.rs"), project.clone(), cx)
                    .is_some(),
                "Failed to resolve main.rs path"
            );

            assert!(
                to_absolute_path(
                    &format!("{root_dir_name}/docs/readme.md",),
                    project.clone(),
                    cx,
                )
                .is_some(),
                "Failed to resolve readme.md path"
            );

            // External URL should return None
            let result = to_absolute_path("https://example.com", project.clone(), cx);
            assert_eq!(result, None, "External URLs should return None");

            // Path outside project
            let result = to_absolute_path("../invalid/path", project.clone(), cx);
            assert_eq!(result, None, "Paths outside the project should return None");
        });
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
        });
    }
}
