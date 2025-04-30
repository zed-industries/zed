use crate::schema::json_schema_for;
use anyhow::{Context as _, Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use gpui::{AnyWindowHandle, App, AppContext, Entity, Task};
use language_model::{LanguageModelRequestMessage, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{path::PathBuf, sync::Arc};
use ui::IconName;
use util::markdown::MarkdownEscaped;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct OpenToolInput {
    /// The path or URL to open with the default application.
    path_or_url: String,
}

pub struct OpenTool;

impl Tool for OpenTool {
    fn name(&self) -> String {
        "open".to_string()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        true
    }

    fn description(&self) -> String {
        include_str!("./open_tool/description.md").to_string()
    }

    fn icon(&self) -> IconName {
        IconName::ArrowUpRight
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<OpenToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<OpenToolInput>(input.clone()) {
            Ok(input) => format!("Open `{}`", MarkdownEscaped(&input.path_or_url)),
            Err(_) => "Open file or URL".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let input: OpenToolInput = match serde_json::from_value(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        // If path_or_url turns out to be a path in the project, make it absolute.
        let abs_path = to_absolute_path(&input.path_or_url, project, cx);

        cx.background_spawn(async move {
            match abs_path {
                Some(path) => open::that(path),
                None => open::that(&input.path_or_url),
            }
            .context("Failed to open URL or file path")?;

            Ok(format!("Successfully opened {}", input.path_or_url))
        })
        .into()
    }
}

fn to_absolute_path(
    potential_path: &str,
    project: Entity<Project>,
    cx: &mut App,
) -> Option<PathBuf> {
    let project = project.read(cx);
    project
        .find_project_path(PathBuf::from(potential_path), cx)
        .and_then(|project_path| project.absolute_path(&project_path, cx))
}

#[cfg(test)]
mod tests {
    use super::*;
    use gpui::TestAppContext;
    use project::{FakeFs, Project};
    use settings::SettingsStore;
    use util::path;

    #[gpui::test]
    async fn test_to_absolute_path(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/root",
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
        let project = Project::test(fs.clone(), [path!("/root").as_ref()], cx).await;

        // Test cases where the function should return Some
        cx.update(|cx| {
            // Project-relative paths should return Some
            let result = to_absolute_path("root/src/main.rs", project.clone(), cx);
            assert!(result.is_some());
            assert_eq!(result.unwrap().to_string_lossy(), "/root/src/main.rs");

            let result = to_absolute_path("root/docs/readme.md", project.clone(), cx);
            assert!(result.is_some());
            assert_eq!(result.unwrap().to_string_lossy(), "/root/docs/readme.md");

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
