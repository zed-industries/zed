use anyhow::Result;
use buffer_diff::BufferDiff;
use collections::HashMap;
use futures::{future::Shared, FutureExt};
use gpui::{prelude::*, Entity, Task};
use language::Buffer;
use project::{
    git_store::{
        GitStore, GitStoreCheckpoint, GitStoreVirtualBranch, GitStoreVirtualBranchChanges,
    },
    Project,
};
use util::TryFutureExt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ThreadDiffSource {
    User,
    Assistant,
}

pub struct ThreadDiff {
    changes: GitStoreVirtualBranchChanges,
    diffs_by_buffer: HashMap<Entity<Buffer>, Entity<BufferDiff>>,
    branch_without_assistant_changes: Shared<Task<Option<GitStoreVirtualBranch>>>,
    last_checkpoint: Option<Task<Result<GitStoreCheckpoint>>>,
    project: Entity<Project>,
    git_store: Entity<GitStore>,
}

impl ThreadDiff {
    pub fn new(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        let mut this = Self {
            changes: GitStoreVirtualBranchChanges::default(),
            diffs_by_buffer: HashMap::default(),
            branch_without_assistant_changes: cx
                .background_spawn(
                    project
                        .read(cx)
                        .git_store()
                        .read(cx)
                        .create_virtual_branch(cx)
                        .log_err(),
                )
                .shared(),
            last_checkpoint: None,
            git_store: project.read(cx).git_store().clone(),
            project,
        };
        this.compute_changes(ThreadDiffSource::User, cx);
        this
    }

    pub fn compute_changes(&mut self, source: ThreadDiffSource, cx: &mut Context<Self>) {
        let last_checkpoint = self.last_checkpoint.take();
        let git_store = self.project.read(cx).git_store().clone();
        let checkpoint = git_store.read(cx).checkpoint(cx);
        let virtual_branch = self.branch_without_assistant_changes.clone();
        self.last_checkpoint = Some(cx.spawn(async move |this, cx| {
            let checkpoint = checkpoint.await?;

            if let Some(virtual_branch) = virtual_branch.await {
                if let Some(last_checkpoint) = last_checkpoint {
                    if let Ok(last_checkpoint) = last_checkpoint.await {
                        if source == ThreadDiffSource::User {
                            let diff = git_store
                                .read_with(cx, |store, cx| {
                                    store.diff_checkpoints(last_checkpoint, checkpoint.clone(), cx)
                                })?
                                .await;

                            if let Ok(diff) = diff {
                                _ = git_store
                                    .read_with(cx, |store, cx| {
                                        store.apply_diff_to_virtual_branch(
                                            virtual_branch.clone(),
                                            diff,
                                            cx,
                                        )
                                    })?
                                    .await;
                            }
                        }
                    }
                }

                let changes = git_store
                    .read_with(cx, |store, cx| {
                        store.changes_for_virtual_branch(virtual_branch, cx)
                    })?
                    .await
                    .unwrap_or_default();
                this.update(cx, |this, cx| this.set_changes(changes, cx))?;
            }

            Ok(checkpoint)
        }));
    }

    fn set_changes(&mut self, changes: GitStoreVirtualBranchChanges, cx: &mut Context<Self>) {
        self.changes = changes;
        cx.notify();
    }
}
