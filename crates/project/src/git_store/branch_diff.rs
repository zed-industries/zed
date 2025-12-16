use anyhow::Result;
use buffer_diff::BufferDiff;
use collections::HashSet;
use futures::StreamExt;
use git::{
    repository::RepoPath,
    status::{DiffTreeType, FileStatus, StatusCode, TrackedStatus, TreeDiff, TreeDiffStatus},
};
use gpui::{
    App, AsyncWindowContext, Context, Entity, EventEmitter, SharedString, Subscription, Task,
    WeakEntity, Window,
};

use language::Buffer;
use text::BufferId;
use util::ResultExt;
use ztracing::instrument;

use crate::{
    Project,
    git_store::{GitStoreEvent, Repository, RepositoryEvent},
};

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
pub enum DiffBase {
    Head,
    Merge { base_ref: SharedString },
}

impl DiffBase {
    pub fn is_merge_base(&self) -> bool {
        matches!(self, DiffBase::Merge { .. })
    }
}

pub struct BranchDiff {
    diff_base: DiffBase,
    repo: Option<Entity<Repository>>,
    project: Entity<Project>,
    base_commit: Option<SharedString>,
    head_commit: Option<SharedString>,
    tree_diff: Option<TreeDiff>,
    _subscription: Subscription,
    update_needed: postage::watch::Sender<()>,
    _task: Task<()>,
}

pub enum BranchDiffEvent {
    FileListChanged,
}

impl EventEmitter<BranchDiffEvent> for BranchDiff {}

impl BranchDiff {
    pub fn new(
        source: DiffBase,
        project: Entity<Project>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let git_store = project.read(cx).git_store().clone();
        let git_store_subscription = cx.subscribe_in(
            &git_store,
            window,
            move |this, _git_store, event, _window, cx| match event {
                GitStoreEvent::ActiveRepositoryChanged(_)
                | GitStoreEvent::RepositoryUpdated(_, RepositoryEvent::StatusesChanged, true)
                | GitStoreEvent::ConflictsUpdated => {
                    cx.emit(BranchDiffEvent::FileListChanged);
                    *this.update_needed.borrow_mut() = ();
                }
                _ => {}
            },
        );

        let (send, recv) = postage::watch::channel::<()>();
        let worker = window.spawn(cx, {
            let this = cx.weak_entity();
            async |cx| Self::handle_status_updates(this, recv, cx).await
        });
        let repo = git_store.read(cx).active_repository();

        Self {
            diff_base: source,
            repo,
            project,
            tree_diff: None,
            base_commit: None,
            head_commit: None,
            _subscription: git_store_subscription,
            _task: worker,
            update_needed: send,
        }
    }

    pub fn diff_base(&self) -> &DiffBase {
        &self.diff_base
    }

    pub async fn handle_status_updates(
        this: WeakEntity<Self>,
        mut recv: postage::watch::Receiver<()>,
        cx: &mut AsyncWindowContext,
    ) {
        Self::reload_tree_diff(this.clone(), cx).await.log_err();
        while recv.next().await.is_some() {
            let Ok(needs_update) = this.update(cx, |this, cx| {
                let mut needs_update = false;
                let active_repo = this
                    .project
                    .read(cx)
                    .git_store()
                    .read(cx)
                    .active_repository();
                if active_repo != this.repo {
                    needs_update = true;
                    this.repo = active_repo;
                } else if let Some(repo) = this.repo.as_ref() {
                    repo.update(cx, |repo, _| {
                        if let Some(branch) = &repo.branch
                            && let DiffBase::Merge { base_ref } = &this.diff_base
                            && let Some(commit) = branch.most_recent_commit.as_ref()
                            && &branch.ref_name == base_ref
                            && this.base_commit.as_ref() != Some(&commit.sha)
                        {
                            this.base_commit = Some(commit.sha.clone());
                            needs_update = true;
                        }

                        if repo.head_commit.as_ref().map(|c| &c.sha) != this.head_commit.as_ref() {
                            this.head_commit = repo.head_commit.as_ref().map(|c| c.sha.clone());
                            needs_update = true;
                        }
                    })
                }
                needs_update
            }) else {
                return;
            };

            if needs_update {
                Self::reload_tree_diff(this.clone(), cx).await.log_err();
            }
        }
    }

    pub fn status_for_buffer_id(&self, buffer_id: BufferId, cx: &App) -> Option<FileStatus> {
        let (repo, path) = self
            .project
            .read(cx)
            .git_store()
            .read(cx)
            .repository_and_path_for_buffer_id(buffer_id, cx)?;
        if self.repo() == Some(&repo) {
            return self.merge_statuses(
                repo.read(cx)
                    .status_for_path(&path)
                    .map(|status| status.status),
                self.tree_diff
                    .as_ref()
                    .and_then(|diff| diff.entries.get(&path)),
            );
        }
        None
    }

    pub fn merge_statuses(
        &self,
        diff_from_head: Option<FileStatus>,
        diff_from_merge_base: Option<&TreeDiffStatus>,
    ) -> Option<FileStatus> {
        match (diff_from_head, diff_from_merge_base) {
            (None, None) => None,
            (Some(diff_from_head), None) => Some(diff_from_head),
            (Some(diff_from_head @ FileStatus::Unmerged(_)), _) => Some(diff_from_head),

            // file does not exist in HEAD
            // but *does* exist in work-tree
            // and *does* exist in merge-base
            (
                Some(FileStatus::Untracked)
                | Some(FileStatus::Tracked(TrackedStatus {
                    index_status: StatusCode::Added,
                    worktree_status: _,
                })),
                Some(_),
            ) => Some(FileStatus::Tracked(TrackedStatus {
                index_status: StatusCode::Modified,
                worktree_status: StatusCode::Modified,
            })),

            // file exists in HEAD
            // but *does not* exist in work-tree
            (Some(diff_from_head), Some(diff_from_merge_base)) if diff_from_head.is_deleted() => {
                match diff_from_merge_base {
                    TreeDiffStatus::Added => None, // unchanged, didn't exist in merge base or worktree
                    _ => Some(diff_from_head),
                }
            }

            // file exists in HEAD
            // and *does* exist in work-tree
            (Some(FileStatus::Tracked(_)), Some(tree_status)) => {
                Some(FileStatus::Tracked(TrackedStatus {
                    index_status: match tree_status {
                        TreeDiffStatus::Added { .. } => StatusCode::Added,
                        _ => StatusCode::Modified,
                    },
                    worktree_status: match tree_status {
                        TreeDiffStatus::Added => StatusCode::Added,
                        _ => StatusCode::Modified,
                    },
                }))
            }

            (_, Some(diff_from_merge_base)) => {
                Some(diff_status_to_file_status(diff_from_merge_base))
            }
        }
    }

    pub async fn reload_tree_diff(
        this: WeakEntity<Self>,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        let task = this.update(cx, |this, cx| {
            let DiffBase::Merge { base_ref } = this.diff_base.clone() else {
                return None;
            };
            let Some(repo) = this.repo.as_ref() else {
                this.tree_diff.take();
                return None;
            };
            repo.update(cx, |repo, cx| {
                Some(repo.diff_tree(
                    DiffTreeType::MergeBase {
                        base: base_ref,
                        head: "HEAD".into(),
                    },
                    cx,
                ))
            })
        })?;
        let Some(task) = task else { return Ok(()) };

        let diff = task.await??;
        this.update(cx, |this, cx| {
            this.tree_diff = Some(diff);
            cx.emit(BranchDiffEvent::FileListChanged);
            cx.notify();
        })
    }

    pub fn repo(&self) -> Option<&Entity<Repository>> {
        self.repo.as_ref()
    }

    #[instrument(skip_all)]
    pub fn load_buffers(&mut self, cx: &mut Context<Self>) -> Vec<DiffBuffer> {
        let mut output = Vec::default();
        let Some(repo) = self.repo.clone() else {
            return output;
        };

        self.project.update(cx, |_project, cx| {
            let mut seen = HashSet::default();

            for item in repo.read(cx).cached_status() {
                seen.insert(item.repo_path.clone());
                let branch_diff = self
                    .tree_diff
                    .as_ref()
                    .and_then(|t| t.entries.get(&item.repo_path))
                    .cloned();
                let Some(status) = self.merge_statuses(Some(item.status), branch_diff.as_ref())
                else {
                    continue;
                };
                if !status.has_changes() {
                    continue;
                }

                let Some(project_path) =
                    repo.read(cx).repo_path_to_project_path(&item.repo_path, cx)
                else {
                    continue;
                };
                let task = Self::load_buffer(branch_diff, project_path, repo.clone(), cx);

                output.push(DiffBuffer {
                    repo_path: item.repo_path.clone(),
                    load: task,
                    file_status: item.status,
                });
            }
            let Some(tree_diff) = self.tree_diff.as_ref() else {
                return;
            };

            for (path, branch_diff) in tree_diff.entries.iter() {
                if seen.contains(&path) {
                    continue;
                }

                let Some(project_path) = repo.read(cx).repo_path_to_project_path(&path, cx) else {
                    continue;
                };
                let task =
                    Self::load_buffer(Some(branch_diff.clone()), project_path, repo.clone(), cx);

                let file_status = diff_status_to_file_status(branch_diff);

                output.push(DiffBuffer {
                    repo_path: path.clone(),
                    load: task,
                    file_status,
                });
            }
        });
        output
    }

    #[instrument(skip_all)]
    fn load_buffer(
        branch_diff: Option<git::status::TreeDiffStatus>,
        project_path: crate::ProjectPath,
        repo: Entity<Repository>,
        cx: &Context<'_, Project>,
    ) -> Task<Result<(Entity<Buffer>, Entity<BufferDiff>)>> {
        let task = cx.spawn(async move |project, cx| {
            let buffer = project
                .update(cx, |project, cx| project.open_buffer(project_path, cx))?
                .await?;

            let languages = project.update(cx, |project, _cx| project.languages().clone())?;

            let changes = if let Some(entry) = branch_diff {
                let oid = match entry {
                    git::status::TreeDiffStatus::Added { .. } => None,
                    git::status::TreeDiffStatus::Modified { old, .. }
                    | git::status::TreeDiffStatus::Deleted { old } => Some(old),
                };
                project
                    .update(cx, |project, cx| {
                        project.git_store().update(cx, |git_store, cx| {
                            git_store.open_diff_since(oid, buffer.clone(), repo, languages, cx)
                        })
                    })?
                    .await?
            } else {
                project
                    .update(cx, |project, cx| {
                        project.open_uncommitted_diff(buffer.clone(), cx)
                    })?
                    .await?
            };
            Ok((buffer, changes))
        });
        task
    }
}

fn diff_status_to_file_status(branch_diff: &git::status::TreeDiffStatus) -> FileStatus {
    let file_status = match branch_diff {
        git::status::TreeDiffStatus::Added { .. } => FileStatus::Tracked(TrackedStatus {
            index_status: StatusCode::Added,
            worktree_status: StatusCode::Added,
        }),
        git::status::TreeDiffStatus::Modified { .. } => FileStatus::Tracked(TrackedStatus {
            index_status: StatusCode::Modified,
            worktree_status: StatusCode::Modified,
        }),
        git::status::TreeDiffStatus::Deleted { .. } => FileStatus::Tracked(TrackedStatus {
            index_status: StatusCode::Deleted,
            worktree_status: StatusCode::Deleted,
        }),
    };
    file_status
}

#[derive(Debug)]
pub struct DiffBuffer {
    pub repo_path: RepoPath,
    pub file_status: FileStatus,
    pub load: Task<Result<(Entity<Buffer>, Entity<BufferDiff>)>>,
}
