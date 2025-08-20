use crate::{AgentTool, ToolCallEventStream};
use agent_client_protocol::ToolKind;
use anyhow::{Context as _, Result, anyhow};
use gpui::{App, AppContext, Entity, SharedString, Task};
use project::Project;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::{path::Path, sync::Arc};
use util::markdown::MarkdownInlineCode;

/// Moves or rename a file or directory in the project, and returns confirmation that the move succeeded.
///
/// If the source and destination directories are the same, but the filename is different, this performs a rename. Otherwise, it performs a move.
///
/// This tool should be used when it's desirable to move or rename a file or directory without changing its contents at all.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct MovePathToolInput {
    /// The source path of the file or directory to move/rename.
    ///
    /// <example>
    /// If the project has the following files:
    ///
    /// - directory1/a/something.txt
    /// - directory2/a/things.txt
    /// - directory3/a/other.txt
    ///
    /// You can move the first file by providing a source_path of "directory1/a/something.txt"
    /// </example>
    pub source_path: String,

    /// The destination path where the file or directory should be moved/renamed to.
    /// If the paths are the same except for the filename, then this will be a rename.
    ///
    /// <example>
    /// To move "directory1/a/something.txt" to "directory2/b/renamed.txt",
    /// provide a destination_path of "directory2/b/renamed.txt"
    /// </example>
    pub destination_path: String,
}

pub struct MovePathTool {
    project: Entity<Project>,
}

impl MovePathTool {
    pub fn new(project: Entity<Project>) -> Self {
        Self { project }
    }
}

impl AgentTool for MovePathTool {
    type Input = MovePathToolInput;
    type Output = String;

    fn name(&self) -> SharedString {
        "move_path".into()
    }

    fn kind(&self) -> ToolKind {
        ToolKind::Move
    }

    fn initial_title(&self, input: Result<Self::Input, serde_json::Value>) -> SharedString {
        if let Ok(input) = input {
            let src = MarkdownInlineCode(&input.source_path);
            let dest = MarkdownInlineCode(&input.destination_path);
            let src_path = Path::new(&input.source_path);
            let dest_path = Path::new(&input.destination_path);

            match dest_path
                .file_name()
                .and_then(|os_str| os_str.to_os_string().into_string().ok())
            {
                Some(filename) if src_path.parent() == dest_path.parent() => {
                    let filename = MarkdownInlineCode(&filename);
                    format!("Rename {src} to {filename}").into()
                }
                _ => format!("Move {src} to {dest}").into(),
            }
        } else {
            "Move path".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        let rename_task = self.project.update(cx, |project, cx| {
            match project
                .find_project_path(&input.source_path, cx)
                .and_then(|project_path| project.entry_for_path(&project_path, cx))
            {
                Some(entity) => match project.find_project_path(&input.destination_path, cx) {
                    Some(project_path) => project.rename_entry(entity.id, project_path.path, cx),
                    None => Task::ready(Err(anyhow!(
                        "Destination path {} was outside the project.",
                        input.destination_path
                    ))),
                },
                None => Task::ready(Err(anyhow!(
                    "Source path {} was not found in the project.",
                    input.source_path
                ))),
            }
        });

        cx.background_spawn(async move {
            let _ = rename_task.await.with_context(|| {
                format!("Moving {} to {}", input.source_path, input.destination_path)
            })?;
            Ok(format!(
                "Moved {} to {}",
                input.source_path, input.destination_path
            ))
        })
    }
}
