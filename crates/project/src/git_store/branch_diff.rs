use anyhow::Result;
use buffer_diff::BufferDiff;
use futures::StreamExt;
use git::{
    repository::RepoPath,
    status::{DiffTreeType, FileStatus, TreeDiff},
};
use gpui::{
    AsyncWindowContext, Context, Entity, EventEmitter, SharedString, Subscription, Task,
    WeakEntity, Window,
};

use language::Buffer;
use util::ResultExt;

use crate::{
    Project,
    git_store::{GitStoreEvent, Repository},
};

#[derive(Debug, Clone)]
pub enum DiffBase {
    Head,
    Merge { base_ref: SharedString },
}

pub struct BranchDiff {
    source: DiffBase,
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
                | GitStoreEvent::RepositoryUpdated(_, _, true)
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
            source,
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
                            && let DiffBase::Merge { base_ref } = &this.source
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

    pub async fn reload_tree_diff(
        this: WeakEntity<Self>,
        cx: &mut AsyncWindowContext,
    ) -> Result<()> {
        let task = this.update(cx, |this, cx| {
            let DiffBase::Merge { base_ref } = this.source.clone() else {
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
            cx.notify();
        })
    }

    pub fn repo(&self) -> Option<&Entity<Repository>> {
        self.repo.as_ref()
    }

    pub fn load_buffers(&mut self, cx: &mut Context<Self>) -> Vec<DiffBuffer> {
        let mut output = Vec::default();
        let Some(repo) = self.repo.clone() else {
            return output;
        };
        self.project.update(cx, |_project, cx| {
            for item in repo.read(cx).cached_status() {
                let branch_diff = self
                    .tree_diff
                    .as_ref()
                    .and_then(|t| t.entries.get(&item.repo_path));
                // todo! exclude mode change?
                if !item.status.has_changes() && branch_diff.is_none() {
                    continue;
                }

                let Some(project_path) =
                    repo.read(cx).repo_path_to_project_path(&item.repo_path, cx)
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
