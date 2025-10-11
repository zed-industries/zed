use anyhow::Result;
use buffer_diff::BufferDiff;
use futures::StreamExt;
use git::{
    repository::RepoPath,
    status::{DiffTreeType, FileStatus, TreeDiff},
};
use gpui::{
    AsyncWindowContext, Context, Entity, SharedString, Subscription, Task, WeakEntity, Window,
};

use language::Buffer;
use util::ResultExt;

use crate::{
    Project,
    git_store::{Repository, RepositoryEvent},
};

pub struct BranchDiff {
    repo: Entity<Repository>,
    project: Entity<Project>,
    base_branch: SharedString,
    head_branch: SharedString,
    base_commit: Option<SharedString>,
    head_commit: Option<SharedString>,
    tree_diff: Option<TreeDiff>,
    _subscription: Subscription,
    update_needed: postage::watch::Sender<()>,
    _task: Task<()>,
}

impl BranchDiff {
    pub fn new(
        project: Entity<Project>,
        repo: Entity<Repository>,
        base_branch: SharedString,
        head_branch: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let repo_subscription = cx.subscribe_in(
            &repo,
            window,
            // todo!() when do we actually need to fire this?
            move |this, _git_store, event, _window, _cx| match event {
                RepositoryEvent::MergeHeadsChanged
                | RepositoryEvent::Updated { .. }
                | RepositoryEvent::PathsChanged => {
                    *this.update_needed.borrow_mut() = ();
                }
            },
        );

        let (send, recv) = postage::watch::channel::<()>();
        let worker = window.spawn(cx, {
            let this = cx.weak_entity();
            async |cx| Self::handle_status_updates(this, recv, cx).await
        });

        Self {
            repo,
            base_branch,
            head_branch,
            project,
            tree_diff: None,
            base_commit: None,
            head_commit: None,
            _subscription: repo_subscription,
            _task: worker,
            update_needed: send,
        }
    }

    pub async fn handle_status_updates(
        this: WeakEntity<Self>,
        mut recv: postage::watch::Receiver<()>,
        cx: &mut AsyncWindowContext,
    ) {
        Self::reload_diff(this.clone(), cx).await.log_err();
        while recv.next().await.is_some() {
            let Ok(needs_update) = this.update(cx, |this, cx| {
                this.repo.update(cx, |repo, _| {
                    let mut needs_update = false;
                    if let Some(branch) = &repo.branch
                        && (branch.ref_name == this.base_branch
                            || branch.ref_name == this.head_branch)
                    {
                        let most_recent_commit = branch
                            .most_recent_commit
                            .as_ref()
                            .map(|commit| commit.sha.clone());

                        if branch.ref_name == this.base_branch
                            && most_recent_commit != this.base_commit
                        {
                            this.base_commit = most_recent_commit;
                            needs_update = true;
                        } else if branch.ref_name == this.head_branch
                            && most_recent_commit != this.head_commit
                        {
                            this.head_commit = most_recent_commit;
                            needs_update = true;
                        }
                    }
                    needs_update
                })
            }) else {
                return;
            };

            if needs_update {
                Self::reload_diff(this.clone(), cx).await.log_err();
            }
        }
    }

    pub async fn reload_diff(this: WeakEntity<Self>, cx: &mut AsyncWindowContext) -> Result<()> {
        let task = this.update(cx, |this, cx| {
            this.repo.update(cx, |repo, cx| {
                repo.diff_tree(
                    DiffTreeType::MergeBase {
                        base: this.base_branch.clone(),
                        head: this.head_branch.clone(),
                    },
                    cx,
                )
            })
        })?;

        let diff = task.await??;
        this.update(cx, |this, cx| {
            this.tree_diff = Some(diff);
            cx.notify();
        })
    }

    pub fn load_buffers(&mut self, cx: &mut Context<Self>) -> Vec<DiffBuffer> {
        let mut output = Vec::default();
        self.project.update(cx, |_project, cx| {
            for item in self.repo.read(cx).cached_status() {
                let branch_diff = self
                    .tree_diff
                    .as_ref()
                    .and_then(|t| t.entries.get(&item.repo_path));
                // todo! exclude mode change?
                if !item.status.has_changes() && branch_diff.is_none() {
                    continue;
                }

                let Some(project_path) = self
                    .repo
                    .read(cx)
                    .repo_path_to_project_path(&item.repo_path, cx)
                else {
                    continue;
                };
                let task = cx.spawn(async move |project, cx| {
                    let buffer = project
                        .update(cx, |project, cx| project.open_buffer(project_path, cx))?
                        .await?;
                    let changes = project
                        .update(cx, |project, cx| {
                            project.open_uncommitted_diff(buffer.clone(), cx)
                        })?
                        .await?;

                    Ok((buffer, changes))
                });

                output.push(DiffBuffer {
                    repo_path: item.repo_path.clone(),
                    load: task,
                    file_status: item.status,
                });
            }
        });
        output
    }
}

#[derive(Debug)]
pub struct DiffBuffer {
    pub repo_path: RepoPath,
    pub file_status: FileStatus,
    pub load: Task<Result<(Entity<Buffer>, Entity<BufferDiff>)>>,
}
