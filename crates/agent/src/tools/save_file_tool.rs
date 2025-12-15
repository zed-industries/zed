use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

use crate::{AgentTool, ToolCallEventStream};

/// Saves a file that has unsaved changes.
///
/// Use this tool when you need to edit a file but it has unsaved changes that must be saved first.
/// Only use this tool after asking the user for permission to save their unsaved changes.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct SaveFileToolInput {
    /// The path of the file to save.
    pub path: PathBuf,
}

pub struct SaveFileTool {
    project: Entity<Project>,
}

impl SaveFileTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for SaveFileTool {
    type Input = SaveFileToolInput;
    type Output = String;

    fn name() -> &'static str {
        "save_file"
    }

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Other
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input
            && let Some(project_path) = self.project.read(cx).find_project_path(&input.path, cx)
            && let Some(path) = self
                .project
                .read(cx)
                .short_full_path_for_project_path(&project_path, cx)
        {
            format!("Save file `{path}`").into()
        } else {
            "Save file".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<String>> {
        let project = self.project.clone();

        let Some(project_path) = project.read(cx).find_project_path(&input.path, cx) else {
            return Task::ready(Err(anyhow!(
                "Path {} not found in project",
                input.path.display()
            )));
        };

        let open_buffer = project.update(cx, |project, cx| {
            project.open_buffer(project_path.clone(), cx)
        });

        cx.spawn(async move |cx| {
            let buffer = open_buffer.await?;

            let is_dirty = buffer.read_with(cx, |buffer, _| buffer.is_dirty())?;

            if !is_dirty {
                return Ok(format!(
                    "File {} has no unsaved changes.",
                    input.path.display()
                ));
            }

            project
                .update(cx, |project, cx| project.save_buffer(buffer, cx))?
                .await?;

            Ok(format!("Saved {}.", input.path.display()))
        })
    }
}
