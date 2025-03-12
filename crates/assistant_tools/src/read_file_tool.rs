use std::path::Path;
use std::sync::Arc;

use anyhow::{anyhow, Result};
use assistant_tool::Tool;
use gpui::{App, Entity, Task};
use language_model::LanguageModelRequestMessage;
use project::{Project, ProjectPath, WorktreeId};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct ReadFileToolInput {
    /// The ID of the worktree in which the file resides.
    pub worktree_id: usize,
    /// The path to the file to read.
    ///
    /// This path is relative to the worktree root, it must not be an absolute path.
    pub path: Arc<Path>,
}

pub struct ReadFileTool;

impl Tool for ReadFileTool {
    fn name(&self) -> String {
        "read-file".into()
    }

    fn description(&self) -> String {
        "Reads the content of a file specified by a worktree ID and path. Use this tool when you need to access the contents of a file in the project.".into()
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

        let project_path = ProjectPath {
            worktree_id: WorktreeId::from_usize(input.worktree_id),
            path: input.path,
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
