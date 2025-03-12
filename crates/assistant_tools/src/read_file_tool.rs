use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_tool::Tool;
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::{Project, ProjectPath};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReadFileToolInput {
    /// The relative path of the file to read.
    ///
    /// This path should never be absolute, and the first component
    /// of the path should always be a top-level directory in a project.
    ///
    /// For example, if the project has the following top-level directories:
    ///
    /// - directory1
    /// - directory2
    ///
    /// If you wanna access `file.txt` in `directory1`, you should use the path `directory1/file.txt`.
    /// If you wanna access `file.txt` in `directory2`, you should use the path `directory2/file.txt`.
    pub path: Arc<Path>,
}

pub struct ReadFileTool;

impl Tool for ReadFileTool {
    fn name(&self) -> String {
        "read-file".into()
    }

    fn description(&self) -> String {
        include_str!("./read_file_tool/description.md").into()
    }

    fn input_schema(&self) -> serde_json::Value {
        let schema = schemars::schema_for!(ReadFileToolInput);
        serde_json::to_value(&schema).unwrap()
    }

    fn run(
        self: Arc<Self>,
        input: serde_json::Value,
        _messages: &[LanguageModelRequestMessage],
        project: Entity<Project>,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let input = match serde_json::from_value::<ReadFileToolInput>(input) {
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
        let project_path = ProjectPath {
            worktree_id: worktree.read(cx).id(),
            path: Arc::from(input.path.strip_prefix(worktree_root_name).unwrap()),
        };
        cx.spawn(|cx| async move {
            let buffer = cx
                .update(|cx| {
                    project.update(cx, |project, cx| project.open_buffer(project_path, cx))
                })?
                .await?;

            buffer.read_with(&cx, |buffer, _cx| {
                if buffer
                    .file()
                    .map_or(false, |file| file.disk_state().exists())
                {
                    Ok(buffer.text())
                } else {
                    Err(anyhow!("File does not exist"))
                }
            })?
        })
    }
}
