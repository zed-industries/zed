use anyhow::{anyhow, Result};
use assistant_tool::Tool;
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{fmt::Write, path::Path, sync::Arc};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ListDirectoryToolInput {
    /// The relative path of the directory to list.
    ///
    /// This path should never be absolute, and the first component
    /// of the path should always be a top-level directory in a project.
    ///
    /// <example>
    /// If the project has the following top-level directories:
    ///
    /// - directory1
    /// - directory2
    ///
    /// You can list the contents of `directory1` by using the path `directory1`.
    /// </example>
    ///
    /// <example>
    /// If the project has the following top-level directories:
    ///
    /// - foo
    /// - bar
    ///
    /// If you wanna list contents in the directory `foo/baz`, you should use the path `foo/baz`.
    /// </example>
    pub path: Arc<Path>,
}

pub struct ListDirectoryTool;

impl Tool for ListDirectoryTool {
    fn name(&self) -> String {
        "list-directory".into()
    }

    fn description(&self) -> String {
        include_str!("./list_directory_tool/description.md").into()
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(ListDirectoryToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let input = match serde_json::from_value::<ListDirectoryToolInput>(input) {
            Ok(input) => input,
            Err(err) => return Task::ready(Err(anyhow!(err))),
        };

        let Some(worktree_root_name) = input.path.components().next() else {
            return Task::ready(Err(anyhow!("Invalid path")));
        };
        let Some(worktree) = project
            .read(cx)
            .worktree_for_root_name(&worktree_root_name.as_os_str().to_string_lossy(), cx)
        else {
            return Task::ready(Err(anyhow!("Directory not found in the project")));
        };
        let path = input.path.strip_prefix(worktree_root_name).unwrap();
        let mut output = String::new();
        for entry in worktree.read(cx).child_entries(path) {
            writeln!(
                output,
                "{}",
                Path::new(worktree_root_name.as_os_str())
                    .join(&entry.path)
                    .display(),
            )
            .unwrap();
        }
        Task::ready(Ok(output))
    }
}
