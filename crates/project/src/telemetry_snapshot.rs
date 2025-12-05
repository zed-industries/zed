use git::repository::DiffType;
use gpui::{App, Entity, Task};
use serde::{Deserialize, Serialize};
use worktree::Worktree;

use crate::{
    Project,
    git_store::{GitStore, LocalRepositoryState, RepositoryState},
};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TelemetrySnapshot {
    pub worktree_snapshots: Vec<TelemetryWorktreeSnapshot>,
}

impl TelemetrySnapshot {
    pub fn new(project: &Entity<Project>, cx: &mut App) -> Task<TelemetrySnapshot> {
        let git_store = project.read(cx).git_store().clone();
        let worktree_snapshots: Vec<_> = project
            .read(cx)
            .visible_worktrees(cx)
            .map(|worktree| TelemetryWorktreeSnapshot::new(worktree, git_store.clone(), cx))
            .collect();

        cx.spawn(async move |_| {
            let worktree_snapshots = futures::future::join_all(worktree_snapshots).await;

            Self { worktree_snapshots }
        })
    }
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct TelemetryWorktreeSnapshot {
    pub worktree_path: String,
    pub git_state: Option<GitState>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct GitState {
    pub remote_url: Option<String>,
    pub head_sha: Option<String>,
    pub current_branch: Option<String>,
    pub diff: Option<String>,
}

impl TelemetryWorktreeSnapshot {
    fn new(
        worktree: Entity<Worktree>,
        git_store: Entity<GitStore>,
        cx: &App,
    ) -> Task<TelemetryWorktreeSnapshot> {
        cx.spawn(async move |cx| {
            // Get worktree path and snapshot
            let worktree_info = cx.update(|app_cx| {
                let worktree = worktree.read(app_cx);
                let path = worktree.abs_path().to_string_lossy().into_owned();
                let snapshot = worktree.snapshot();
                (path, snapshot)
            });

            let Ok((worktree_path, _snapshot)) = worktree_info else {
                return TelemetryWorktreeSnapshot {
                    worktree_path: String::new(),
                    git_state: None,
                };
            };

            let git_state = git_store
                .update(cx, |git_store, cx| {
                    git_store
                        .repositories()
                        .values()
                        .find(|repo| {
                            repo.read(cx)
                                .abs_path_to_repo_path(&worktree.read(cx).abs_path())
                                .is_some()
                        })
                        .cloned()
                })
                .ok()
                .flatten()
                .map(|repo| {
                    repo.update(cx, |repo, _| {
                        let current_branch =
                            repo.branch.as_ref().map(|branch| branch.name().to_owned());
                        repo.send_job(None, |state, _| async move {
                            let RepositoryState::Local(LocalRepositoryState { backend, .. }) =
                                state
                            else {
                                return GitState {
                                    remote_url: None,
                                    head_sha: None,
                                    current_branch,
                                    diff: None,
                                };
                            };

                            let remote_url = backend.remote_url("origin").await;
                            let head_sha = backend.head_sha().await;
                            let diff = backend.diff(DiffType::HeadToWorktree).await.ok();

                            GitState {
                                remote_url,
                                head_sha,
                                current_branch,
                                diff,
                            }
                        })
                    })
                });

            let git_state = match git_state {
                Some(git_state) => match git_state.ok() {
                    Some(git_state) => git_state.await.ok(),
                    None => None,
                },
                None => None,
            };

            TelemetryWorktreeSnapshot {
                worktree_path,
                git_state,
            }
        })
    }
}
