use crate::{AgentTool, ToolCallEventStream};
use action_log::ActionLog;
use agent_client_protocol::ToolKind;
use anyhow::{Context as _, Result, anyhow};
use futures::{SinkExt, StreamExt, channel::mpsc};
use gpui::{App, AppContext, Entity, SharedString, Task};
use project::{Project, ProjectPath};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

/// Deletes the file or directory (and the directory's contents, recursively) at the specified path in the project, and returns confirmation of the deletion.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct DeletePathToolInput {
    /// The path of the file or directory to delete.
    ///
    /// <example>
    /// If the project has the following files:
    ///
    /// - directory1/a/something.txt
    /// - directory2/a/things.txt
    /// - directory3/a/other.txt
    ///
    /// You can delete the first file by providing a path of "directory1/a/something.txt"
    /// </example>
    pub path: String,
}

pub struct DeletePathTool {
    project: Entity<Project>,
    action_log: Entity<ActionLog>,
}

impl DeletePathTool {
    pub fn new(project: Entity<Project>, action_log: Entity<ActionLog>) -> Self {
        Self {
            project,
            action_log,
        }
    }
}

impl AgentTool for DeletePathTool {
    type Input = DeletePathToolInput;
    type Output = String;

    fn name() -> &'static str {
        "delete_path"
    }

    fn kind() -> ToolKind {
        ToolKind::Delete
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("Delete “`{}`”", input.path).into()
        } else {
            "Delete path".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: Self::Input,
        _event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        let path = input.path;
        let Some(project_path) = self.project.read(cx).find_project_path(&path, cx) else {
            return Task::ready(Err(anyhow!(
                "Couldn't delete {path} because that path isn't in this project."
            )));
        };

        let Some(worktree) = self
            .project
            .read(cx)
            .worktree_for_id(project_path.worktree_id, cx)
        else {
            return Task::ready(Err(anyhow!(
                "Couldn't delete {path} because that path isn't in this project."
            )));
        };

        let worktree_snapshot = worktree.read(cx).snapshot();
        let (mut paths_tx, mut paths_rx) = mpsc::channel(256);
        cx.background_spawn({
            let project_path = project_path.clone();
            async move {
                for entry in
                    worktree_snapshot.traverse_from_path(true, false, false, &project_path.path)
                {
                    if !entry.path.starts_with(&project_path.path) {
                        break;
                    }
                    paths_tx
                        .send(ProjectPath {
                            worktree_id: project_path.worktree_id,
                            path: entry.path.clone(),
                        })
                        .await?;
                }
                anyhow::Ok(())
            }
        })
        .detach();

        let project = self.project.clone();
        let action_log = self.action_log.clone();
        cx.spawn(async move |cx| {
            while let Some(path) = paths_rx.next().await {
                if let Ok(buffer) = project
                    .update(cx, |project, cx| project.open_buffer(path, cx))
                    .await
                {
                    action_log.update(cx, |action_log, cx| {
                        action_log.will_delete_buffer(buffer.clone(), cx)
                    });
                }
            }

            let deletion_task = project
                .update(cx, |project, cx| {
                    project.delete_file(project_path, false, cx)
                })
                .with_context(|| {
                    format!("Couldn't delete {path} because that path isn't in this project.")
                })?;
            deletion_task
                .await
                .with_context(|| format!("Deleting {path}"))?;
            Ok(format!("Deleted {path}"))
        })
    }
}
