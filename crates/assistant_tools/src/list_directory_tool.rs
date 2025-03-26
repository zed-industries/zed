use anyhow::{anyhow, Result};
use assistant_tool::{ActionLog, Tool};
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt::Write, path::Path, sync::Arc};
use ui::IconName;
use util::markdown::MarkdownString;

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListDirectoryToolInput {
    /// The relative path of the directory to list.
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
        "list-directory".into()
    }

    fn needs_confirmation(&self) -> bool {
        false
    }

    fn description(&self) -> String {
        include_str!("./list_directory_tool/description.md").into()
    }

    fn icon(&self) -> IconName {
        IconName::Folder
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(ListDirectoryToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn ui_text(&self, input: &serde_json::Value) -> String {
        match serde_json::from_value::<ListDirectoryToolInput>(input.clone()) {
            Ok(input) => {
                let path = MarkdownString::inline_code(&input.path);
                format!("List the {path} directory's contents")
            }
            Err(_) => "List directory".to_string(),
        }
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        _action_log: Entity<ActionLog>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let input = match serde_json::from_value::<ListDirectoryToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
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

            return Task::ready(Ok(output));
        }

        let Some(project_path) = project.read(cx).find_project_path(&input.path, cx) else {
            return Task::ready(Err(anyhow!("Path {} not found in project", input.path)));
        };
        let Some(worktree) = project
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)
        else {
            return Task::ready(Err(anyhow!("Worktree not found")));
        };
        let worktree = worktree.read(cx);

        let Some(entry) = worktree.entry_for_path(&project_path.path) else {
            return Task::ready(Err(anyhow!("Path not found: {}", input.path)));
        };

        if !entry.is_dir() {
            return Task::ready(Err(anyhow!("{} is not a directory.", input.path)));
        }

        let mut output = String::new();
        for entry in worktree.child_entries(&project_path.path) {
            writeln!(
                output,
                "{}",
                Path::new(worktree.root_name()).join(&entry.path).display(),
            )
            .unwrap();
        }
        if output.is_empty() {
            return Task::ready(Ok(format!("{} is empty.", input.path)));
        }
        Task::ready(Ok(output))
    }
}
