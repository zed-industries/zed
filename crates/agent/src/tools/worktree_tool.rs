use crate::{AgentTool, ToolCallEventStream, ToolInput};
use agent_client_protocol as acp;
use anyhow::{Result, anyhow};
use collections::HashSet;
use futures::FutureExt as _;
use git::repository::{validate_worktree_directory, worktree_path_for_branch};
use gpui::{App, AsyncApp, Entity, SharedString, Task, WeakEntity};
use project::project_settings::ProjectSettings;
use project::{
    Project,
    git_store::Repository,
    trusted_worktrees::{PathTrust, TrustedWorktrees},
};
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use settings::Settings;
use std::{path::Path, path::PathBuf, sync::Arc};
use workspace::Workspace;

/// Creates or switches to a git worktree for a branch.
///
/// - If the branch already has a worktree, this switches the current thread/workspace context to it.
/// - Otherwise, this creates a new worktree for the branch using the repository's configured `git.worktree_directory`.
/// - Use this tool when you want isolated branch/task state before searching, editing, or running commands.
#[derive(Debug, Serialize, Deserialize, JsonSchema)]
pub struct WorktreeToolInput {
    /// The branch name to switch to or create a worktree for.
    pub branch: String,
    /// Optional base ref or commit to branch from when creating a new worktree.
    #[serde(default)]
    pub base_ref: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
#[serde(untagged)]
pub enum WorktreeToolOutput {
    Success {
        action: String,
        branch: String,
        path: PathBuf,
    },
    Error {
        error: String,
    },
}

impl From<WorktreeToolOutput> for language_model::LanguageModelToolResultContent {
    fn from(output: WorktreeToolOutput) -> Self {
        match output {
            WorktreeToolOutput::Success {
                action,
                branch,
                path,
            } => format!(
                "{action} worktree for branch `{branch}` at {}",
                path.display()
            )
            .into(),
            WorktreeToolOutput::Error { error } => error.into(),
        }
    }
}

pub struct WorktreeTool {
    project: Entity<Project>,
    workspace: WeakEntity<Workspace>,
}

impl WorktreeTool {
    pub fn new(project: Entity<Project>, workspace: WeakEntity<Workspace>) -> Self {
        Self { project, workspace }
    }

    fn authorize_title(
        action: &str,
        branch: &str,
        path: &Path,
        base_ref: Option<&str>,
    ) -> SharedString {
        let mut title = format!("{action} worktree for `{branch}`");
        if let Some(base_ref) = base_ref {
            title.push_str(&format!(" from `{base_ref}`"));
        }
        title.push_str(&format!(" at `{}`", path.display()));
        title.into()
    }
}

fn tool_error(error: impl ToString) -> WorktreeToolOutput {
    WorktreeToolOutput::Error {
        error: error.to_string(),
    }
}

impl AgentTool for WorktreeTool {
    type Input = WorktreeToolInput;
    type Output = WorktreeToolOutput;

    const NAME: &'static str = "worktree";

    fn kind() -> acp::ToolKind {
        acp::ToolKind::Execute
    }

    fn initial_title(
        &self,
        input: Result<Self::Input, serde_json::Value>,
        _cx: &mut App,
    ) -> SharedString {
        if let Ok(input) = input {
            format!("Prepare worktree for `{}`", input.branch).into()
        } else {
            "Prepare worktree".into()
        }
    }

    fn run(
        self: Arc<Self>,
        input: ToolInput<Self::Input>,
        event_stream: ToolCallEventStream,
        cx: &mut App,
    ) -> Task<Result<Self::Output, Self::Output>> {
        let project = self.project.clone();
        let workspace = self.workspace.clone();
        cx.spawn(async move |cx| {
            let input = input
                .recv()
                .await
                .map_err(|error| tool_error(format!("Failed to receive tool input: {error}")))?;

            let (workspace_entity, repository) = cx
                .update(|cx| {
                    let workspace_entity = workspace.upgrade().ok_or_else(|| {
                        anyhow!("No workspace is available for worktree operations")
                    })?;
                    let repository = workspace_entity
                        .read(cx)
                        .effective_active_repository(cx)
                        .ok_or_else(|| anyhow!("No active git repository is available"))?;
                    anyhow::Ok((workspace_entity, repository))
                })
                .map_err(tool_error)?;

            let existing_worktree = repository
                .update(cx, |repository, _cx| repository.worktrees())
                .await
                .map_err(tool_error)?
                .map_err(tool_error)?
                .into_iter()
                .find(|worktree| worktree.branch() == input.branch);

            let (action, target_path) = if let Some(worktree) = existing_worktree.as_ref() {
                ("Switch to".to_string(), worktree.path.clone())
            } else {
                let target_path = repository
                    .read_with(cx, |repository, cx| {
                        let worktree_directory_setting = ProjectSettings::get_global(cx)
                            .git
                            .worktree_directory
                            .clone();
                        validate_worktree_directory(
                            &repository.original_repo_abs_path,
                            &worktree_directory_setting,
                        )?;
                        anyhow::Ok(worktree_path_for_branch(
                            &repository.original_repo_abs_path,
                            &worktree_directory_setting,
                            &input.branch,
                        ))
                    })
                    .map_err(tool_error)?;
                ("Create".to_string(), target_path)
            };

            let authorize = cx.update(|cx| {
                let mut context_args =
                    vec![input.branch.clone(), target_path.display().to_string()];
                if let Some(base_ref) = input.base_ref.clone() {
                    context_args.push(base_ref);
                }
                let context = crate::ToolPermissionContext::new(Self::NAME, context_args);
                event_stream.authorize(
                    Self::authorize_title(
                        &action,
                        &input.branch,
                        &target_path,
                        input.base_ref.as_deref(),
                    ),
                    context,
                    cx,
                )
            });

            futures::select! {
                result = authorize.fuse() => {
                    result.map_err(tool_error)?;
                }
                _ = event_stream.cancelled_by_user().fuse() => {
                    return Err(WorktreeToolOutput::Error {
                        error: "Worktree operation cancelled by user".to_string(),
                    });
                }
            }

            let ensured_path = if existing_worktree.is_some() {
                target_path
            } else {
                let (receiver, new_worktree_path) = repository
                    .update(cx, |repository, cx| {
                        let worktree_directory_setting = ProjectSettings::get_global(cx)
                            .git
                            .worktree_directory
                            .clone();
                        let directory = validate_worktree_directory(
                            &repository.original_repo_abs_path,
                            &worktree_directory_setting,
                        )?;
                        let new_worktree_path = worktree_path_for_branch(
                            &repository.original_repo_abs_path,
                            &worktree_directory_setting,
                            &input.branch,
                        );
                        let receiver = repository.create_worktree(
                            input.branch.clone(),
                            directory,
                            input.base_ref.clone(),
                        );
                        anyhow::Ok((receiver, new_worktree_path))
                    })
                    .map_err(tool_error)?;
                let create_worktree = receiver.await.map_err(tool_error)?;
                create_worktree.map_err(tool_error)?;

                trust_new_worktree_if_parent_trusted(
                    &workspace_entity,
                    &repository,
                    &new_worktree_path,
                    cx,
                )
                .map_err(tool_error)?;

                new_worktree_path
            };

            let (worktree, _) = project
                .update(cx, |project, cx| {
                    project.find_or_create_worktree(ensured_path.as_path(), true, cx)
                })
                .await
                .map_err(tool_error)?;

            let worktree_id = worktree.read_with(cx, |worktree, _cx| worktree.id());
            workspace_entity.update(cx, |workspace, cx| {
                workspace.set_active_worktree_override(Some(worktree_id), cx);
            });

            Ok(WorktreeToolOutput::Success {
                action: if existing_worktree.is_some() {
                    "Switched to existing".into()
                } else {
                    "Created".into()
                },
                branch: input.branch,
                path: ensured_path,
            })
        })
    }
}

fn trust_new_worktree_if_parent_trusted(
    workspace: &Entity<Workspace>,
    repository: &Entity<Repository>,
    new_worktree_path: &PathBuf,
    cx: &mut AsyncApp,
) -> Result<()> {
    workspace.update(cx, |workspace, cx| -> Result<()> {
        let Some(trusted_worktrees) = TrustedWorktrees::try_get_global(cx) else {
            return anyhow::Ok(());
        };

        let repo_path = repository.read(cx).snapshot().work_directory_abs_path;
        let project = workspace.project();
        let Some((parent_worktree, _)) = project.read(cx).find_worktree(&repo_path, cx) else {
            return anyhow::Ok(());
        };
        let worktree_store = project.read(cx).worktree_store();

        trusted_worktrees.update(cx, |trusted_worktrees, cx| {
            if trusted_worktrees.can_trust(&worktree_store, parent_worktree.read(cx).id(), cx) {
                trusted_worktrees.trust(
                    &worktree_store,
                    HashSet::from_iter([PathTrust::AbsPath(new_worktree_path.clone())]),
                    cx,
                );
            }
        });

        anyhow::Ok(())
    })?;
    Ok(())
}
