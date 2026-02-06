use super::edit_file_tool::{
    SensitiveSettingsKind, is_sensitive_settings_path, sensitive_settings_kind,
};
use crate::{AgentTool, ToolCallEventStream, ToolPermissionDecision, decide_permission_for_path};
use action_log::ActionLog;
use agent_client_protocol::ToolKind;
use agent_settings::AgentSettings;
use anyhow::{Context as _, Result, anyhow};
use futures::{FutureExt as _, SinkExt, StreamExt, channel::mpsc};
use gpui::{App, AppContext, Entity, SharedString, Task};
use project::{Project, ProjectPath};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::path::Path;
use std::sync::Arc;
use util::markdown::MarkdownInlineCode;

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

    const NAME: &'static str = "delete_path";

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
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output>> {
        let path = input.path;

        let settings = AgentSettings::get_global(cx);
        let mut decision = decide_permission_for_path(Self::NAME, &path, settings);

        if matches!(decision, ToolPermissionDecision::Allow)
            && !settings.always_allow_tool_actions
            && is_sensitive_settings_path(Path::new(&path))
        {
            decision = ToolPermissionDecision::Confirm;
        }

        let authorize = match decision {
            ToolPermissionDecision::Allow => None,
            ToolPermissionDecision::Deny(reason) => {
                return Task::ready(Err(anyhow!("{}", reason)));
            }
            ToolPermissionDecision::Confirm => {
                let context = crate::ToolPermissionContext {
                    tool_name: Self::NAME.to_string(),
                    input_value: path.clone(),
                };
                let title = format!("Delete {}", MarkdownInlineCode(&path));
                let title = match sensitive_settings_kind(Path::new(&path)) {
                    Some(SensitiveSettingsKind::Local) => format!("{title} (local settings)"),
                    Some(SensitiveSettingsKind::Global) => format!("{title} (settings)"),
                    None => title,
                };
                Some(event_stream.authorize(title, context, cx))
            }
        };

        let project = self.project.clone();
        let action_log = self.action_log.clone();
        cx.spawn(async move |cx| {
            if let Some(authorize) = authorize {
                authorize.await?;
            }

            let (project_path, worktree_snapshot) = project.read_with(cx, |project, cx| {
                let project_path = project.find_project_path(&path, cx).ok_or_else(|| {
                    anyhow!("Couldn't delete {path} because that path isn't in this project.")
                })?;
                let worktree = project
                    .worktree_for_id(project_path.worktree_id, cx)
                    .ok_or_else(|| {
                        anyhow!("Couldn't delete {path} because that path isn't in this project.")
                    })?;
                let worktree_snapshot = worktree.read(cx).snapshot();
                anyhow::Ok((project_path, worktree_snapshot))
            })?;

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

            loop {
                let path_result = futures::select! {
                    path = paths_rx.next().fuse() => path,
                    _ = event_stream.cancelled_by_user().fuse() => {
                        anyhow::bail!("Delete cancelled by user");
                    }
                };
                let Some(path) = path_result else {
                    break;
                };
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

            futures::select! {
                result = deletion_task.fuse() => {
                    result.with_context(|| format!("Deleting {path}"))?;
                }
                _ = event_stream.cancelled_by_user().fuse() => {
                    anyhow::bail!("Delete cancelled by user");
                }
            }
            Ok(format!("Deleted {path}"))
        })
    }
}
