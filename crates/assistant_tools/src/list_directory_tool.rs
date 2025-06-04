use crate::schema::json_schema_for;
use anyhow::{Result, anyhow};
use assistant_tool::{ActionLog, Tool, ToolResult};
use gpui::{AnyWindowHandle, App, Entity, Task};
use language_model::{LanguageModel, LanguageModelRequest, LanguageModelToolSchemaFormat};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt::Write, path::Path, sync::Arc};
use ui::IconName;
use util::markdown::MarkdownInlineCode;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListDirectoryToolInput {
    /// The fully-qualified path of the directory to list in the project.
    ///
    /// This path should never be absolute, and the first component
    /// of the path should always be a root directory in a project.
    ///
    /// <example>
    /// If the project has the following root directories:
    ///
    /// - directory1
    /// - directory2
    ///
    /// You can list the contents of `directory1` by using the path `directory1`.
    /// </example>
    ///
    /// <example>
    /// If the project has the following root directories:
    ///
    /// - foo
    /// - bar
    ///
    /// If you wanna list contents in the directory `foo/baz`, you should use the path `foo/baz`.
    /// </example>
    pub path: String,
}

pub struct ListDirectoryTool;

impl Tool for ListDirectoryTool {
    fn name(&self) -> String {
        "list_directory".into()
    }

    fn needs_confirmation(&self, _: &serde_json::Value, _: &App) -> bool {
        false
    }

    fn may_perform_edits(&self) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./list_directory_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Folder
    }

    fn input_schema(&self, format: LanguageModelToolSchemaFormat) -> Result<serde_json::Value> {
        json_schema_for::<ListDirectoryToolInput>(format)
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<ListDirectoryToolInput>(input.clone()) {
            Ok(input) => {
                let path = MarkdownInlineCode(&input.path);
                format!("List the {path} directory's contents")
            }
            Err(_) => "List directory".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _request: Arc<LanguageModelRequest>,
        project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        _model: Arc<dyn LanguageModel>,
        _window: Option<AnyWindowHandle>,
        cx: &mut App,
    ) -> ToolResult {
        let input = match serde_json::from_value::<ListDirectoryToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))).into(),
        };

        // Sometimes models will return these even though we tell it to give a path and not a glob.
        // When this happens, just list the root worktree directories.
        if matches!(input.path.as_str(), "." | "" | "./" | "*") {
            let output = project
                .read(cx)
                .worktrees(cx)
                .filter_map(|worktree| {
                    worktree.read(cx).root_entry().and_then(|entry| {
                        if entry.is_dir() {
                            entry.path.to_str()
                        } else {
                            None
                        }
                    })
                })
                .collect::<Vec<_>>()
                .join("\n");

            return Task::ready(Ok(output.into())).into();
        }

        let Some(project_path) = project.read(cx).find_project_path(&input.path, cx) else {
            return Task::ready(Err(anyhow!("Path {} not found in project", input.path))).into();
        };
        let Some(worktree) = project
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)
        else {
            return Task::ready(Err(anyhow!("Worktree not found"))).into();
        };
        let worktree = worktree.read(cx);

        let Some(entry) = worktree.entry_for_path(&project_path.path) else {
            return Task::ready(Err(anyhow!("Path not found: {}", input.path))).into();
        };

        if !entry.is_dir() {
            return Task::ready(Err(anyhow!("{} is not a directory.", input.path))).into();
        }

        let mut folders = Vec::new();
        let mut files = Vec::new();

        for entry in worktree.child_entries(&project_path.path) {
            let full_path = Path::new(worktree.root_name())
                .join(&entry.path)
                .display()
                .to_string();
            if entry.is_dir() {
                folders.push(full_path);
            } else {
                files.push(full_path);
            }
        }

        let mut output = String::new();

        if !folders.is_empty() {
            writeln!(output, "# Folders:\n{}", folders.join("\n")).unwrap();
        }

        if !files.is_empty() {
            writeln!(output, "\n# Files:\n{}", files.join("\n")).unwrap();
        }

        if output.is_empty() {
            writeln!(output, "{} is empty.", input.path).unwrap();
        }

        Task::ready(Ok(output.into())).into()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use assistant_tool::Tool;
    use gpui::{AppContext, TestAppContext};
    use indoc::indoc;
    use language_model::fake_provider::FakeLanguageModel;
    use project::{FakeFs, Project};
    use serde_json::json;
    use settings::SettingsStore;
    use util::path;

    fn platform_paths(path_str: &str) -> String {
        if cfg!(target_os = "windows") {
            path_str.replace("/", "\\")
        } else {
            path_str.to_string()
        }
    }

    fn init_test(cx: &mut TestAppContext) {
        cx.update(|cx| {
            let settings_store = SettingsStore::test(cx);
            cx.set_global(settings_store);
            language::init(cx);
            Project::init_settings(cx);
        });
    }

    #[gpui::test]
    async fn test_list_directory_separates_files_and_dirs(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                "src": {
                    "main.rs": "fn main() {}",
                    "lib.rs": "pub fn hello() {}",
                    "models": {
                        "user.rs": "struct User {}",
                        "post.rs": "struct Post {}"
                    },
                    "utils": {
                        "helper.rs": "pub fn help() {}"
                    }
                },
                "tests": {
                    "integration_test.rs": "#[test] fn test() {}"
                },
                "README.md": "# Project",
                "Cargo.toml": "[package]"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let model = Arc::new(FakeLanguageModel::default());
        let tool = Arc::new(ListDirectoryTool);

        // Test listing root directory
        let input = json!({
            "path": "project"
        });

        let result = cx
            .update(|cx| {
                tool.clone().run(
                    input,
                    Arc::default(),
                    project.clone(),
                    action_log.clone(),
                    model.clone(),
                    None,
                    cx,
                )
            })
            .output
            .await
            .unwrap();

        let content = result.content.as_str().unwrap();
        assert_eq!(
            content,
            platform_paths(indoc! {"
                # Folders:
                project/src
                project/tests

                # Files:
                project/Cargo.toml
                project/README.md
            "})
        );

        // Test listing src directory
        let input = json!({
            "path": "project/src"
        });

        let result = cx
            .update(|cx| {
                tool.clone().run(
                    input,
                    Arc::default(),
                    project.clone(),
                    action_log.clone(),
                    model.clone(),
                    None,
                    cx,
                )
            })
            .output
            .await
            .unwrap();

        let content = result.content.as_str().unwrap();
        assert_eq!(
            content,
            platform_paths(indoc! {"
                # Folders:
                project/src/models
                project/src/utils

                # Files:
                project/src/lib.rs
                project/src/main.rs
            "})
        );

        // Test listing directory with only files
        let input = json!({
            "path": "project/tests"
        });

        let result = cx
            .update(|cx| {
                tool.clone().run(
                    input,
                    Arc::default(),
                    project.clone(),
                    action_log.clone(),
                    model.clone(),
                    None,
                    cx,
                )
            })
            .output
            .await
            .unwrap();

        let content = result.content.as_str().unwrap();
        assert!(!content.contains("# Folders:"));
        assert!(content.contains("# Files:"));
        assert!(content.contains(&platform_paths("project/tests/integration_test.rs")));
    }

    #[gpui::test]
    async fn test_list_directory_empty_directory(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                "empty_dir": {}
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let model = Arc::new(FakeLanguageModel::default());
        let tool = Arc::new(ListDirectoryTool);

        let input = json!({
            "path": "project/empty_dir"
        });

        let result = cx
            .update(|cx| tool.run(input, Arc::default(), project, action_log, model, None, cx))
            .output
            .await
            .unwrap();

        let content = result.content.as_str().unwrap();
        assert_eq!(content, "project/empty_dir is empty.\n");
    }

    #[gpui::test]
    async fn test_list_directory_error_cases(cx: &mut TestAppContext) {
        init_test(cx);

        let fs = FakeFs::new(cx.executor());
        fs.insert_tree(
            "/project",
            json!({
                "file.txt": "content"
            }),
        )
        .await;

        let project = Project::test(fs.clone(), [path!("/project").as_ref()], cx).await;
        let action_log = cx.new(|_| ActionLog::new(project.clone()));
        let model = Arc::new(FakeLanguageModel::default());
        let tool = Arc::new(ListDirectoryTool);

        // Test non-existent path
        let input = json!({
            "path": "project/nonexistent"
        });

        let result = cx
            .update(|cx| {
                tool.clone().run(
                    input,
                    Arc::default(),
                    project.clone(),
                    action_log.clone(),
                    model.clone(),
                    None,
                    cx,
                )
            })
            .output
            .await;

        assert!(result.is_err());
        assert!(result.unwrap_err().to_string().contains("Path not found"));

        // Test trying to list a file instead of directory
        let input = json!({
            "path": "project/file.txt"
        });

        let result = cx
            .update(|cx| tool.run(input, Arc::default(), project, action_log, model, None, cx))
            .output
            .await;

        assert!(result.is_err());
        assert!(
            result
                .unwrap_err()
                .to_string()
                .contains("is not a directory")
        );
    }
}
