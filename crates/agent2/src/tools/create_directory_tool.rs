use agent_client_protocol::ToolKind;
use anyhow::{Context as _, Result, anyhow};
use gpui::{App, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use util::markdown::MarkdownInlineCode;

use crate::{AgentTool, ToolCallEventStream};

/// Creates a new directory at the specified path within the project. Returns
/// confirmation that the directory was created.
///
/// This tool creates a directory and all necessary parent directories (similar
/// to `mkdir -p`). It should be used whenever you need to create new
/// directories within the project.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct CreateDirectoryToolInput {
    /// The path of the new directory.
    ///
    /// <example>
    /// If the project has the following structure:
    ///
    /// - directory1/
    /// - directory2/
    ///
    /// You can create a new directory by providing a path of "directory1/new_directory"
    /// </example>
    pub path: String,
}

pub struct CreateDirectoryTool {
    project: Entity<Project>,
}

impl CreateDirectoryTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for CreateDirectoryTool {
    type Input = CreateDirectoryToolInput;
    type Output = String;

    fn name(&self) -> SharedString {
        "create_directory".into()
    }

    fn kind(&self) -> ToolKind {
        ToolKind::Read
    }

    fn initial_title(&self, input: Result<Self::Input, serde_json::Value>) -> SharedString {
        if let Ok(input) = input {
            format!("Create directory {}", MarkdownInlineCode(&input.path)).into()
        } else {
            "Create directory".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        let project_path = match self.project.read(cx).find_project_path(&input.path, cx) {
            Some(project_path) => project_path,
            None => {
                return Task::ready(Err(anyhow!("Path to create was outside the project")));
            }
        };
        let destination_path: Arc<str> = input.path.as_str().into();

        let create_entry = self.project.update(cx, |project, cx| {
            project.create_entry(project_path.clone(), true, cx)
        });

        cx.spawn(async move |_cx| {
            create_entry
                .await
                .with_context(|| format!("Creating directory {destination_path}"))?;

            Ok(format!("Created directory {destination_path}"))
        })
    }
}
