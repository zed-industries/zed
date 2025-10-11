use anyhow::Result;
use collections::{HashMap, HashSet};
use futures::future::Shared;
use git::{
    repository::RepoPath,
    status::{DiffTreeType, TreeDiff, TreeDiffStatus},
};
use gpui::{
    AsyncWindowContext, Context, Entity, SharedString, Subscription, Task, WeakEntity, Window,
};
use language::Buffer;
use smol::stream::StreamExt;
use util::ResultExt;

use crate::git_store::{GitStore, GitStoreEvent, Repository};

pub struct BranchDiff {
    repo: Entity<Repository>,
    base_branch: SharedString,
    head_branch: SharedString,
    base_commit: Option<SharedString>,
    head_commit: Option<SharedString>,
    tree_diff: Option<TreeDiff>,
    base_buffers: HashMap<RepoPath, BaseBuffer>,
    _subscription: Subscription,
    update_needed: postage::watch::Sender<()>,
    _task: Task<()>,
}

enum BaseBuffer {
    None,
    Loading {
        oid: git::Oid,
        task: Shared<Task<Entity<Buffer>>>,
    },
}

impl BranchDiff {
    pub fn new(
        git_store: &Entity<GitStore>,
        repo: Entity<Repository>,
        base_branch: SharedString,
        head_branch: SharedString,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let git_store_subscription = cx.subscribe_in(
            &git_store,
            window,
            move |this, _git_store, event, _window, _cx| match event {
                GitStoreEvent::ActiveRepositoryChanged(_)
                | GitStoreEvent::RepositoryUpdated(_, _, true)
                | GitStoreEvent::ConflictsUpdated => {
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

        Self {
            repo,
            base_branch,
            head_branch,
            tree_diff: None,
            base_buffers: HashMap::default(),
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
            this.tree_diff = Some(diff.clone());
            let mut new_paths = HashSet::default();
            for (path, status) in diff.entries.as_ref() {
                new_paths.insert(path.clone());
                let existing = this.base_buffers.get(&path);
                let old_sha = match status {
                    TreeDiffStatus::Modified { old, .. } | TreeDiffStatus::Deleted { old } => {
                        if let Some(BaseBuffer::Loading { oid, .. }) = this.base_buffers.get(&path)
                            && old == oid
                        {
                            continue;
                        }
                        this.base_buffers.insert(
                            path.clone(),
                            BaseBuffer::Loading {
                                oid: *old,
                                task: todo!(),
                            },
                        )
                    }
                    TreeDiffStatus::Added { .. } => {
                        this.base_buffers.insert(path.clone(), BaseBuffer::None)
                    }
                    TreeDiffStatus::TypeChanged {} => continue,
                };
            }
            this.base_buffers.retain(|path, _| new_paths.contains(path))
        })
    }
}
