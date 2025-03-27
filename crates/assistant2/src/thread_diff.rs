use anyhow::{Context as _, Result};
use buffer_diff::BufferDiff;
use collections::HashMap;
use futures::{channel::mpsc, future::Shared, FutureExt, StreamExt};
use gpui::{prelude::*, App, Entity, Task};
use language::{Buffer, LanguageRegistry};
use project::{
    git_store::{GitStore, GitStoreCheckpoint, GitStoreIndex, GitStoreStatus},
    Project,
};
use std::sync::Arc;
use util::TryFutureExt;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub enum ChangeAuthor {
    User,
    Agent,
}

pub struct ThreadDiff {
    base: Shared<Task<Option<GitStoreIndex>>>,
    diffs_by_buffer: HashMap<Entity<Buffer>, Entity<BufferDiff>>,
    status: GitStoreStatus,
    project: Entity<Project>,
    git_store: Entity<GitStore>,
    checkpoints_tx: mpsc::UnboundedSender<(ChangeAuthor, GitStoreCheckpoint)>,
    _maintain_diff: Task<Result<()>>,
}

impl ThreadDiff {
    pub fn new(project: Entity<Project>, cx: &mut Context<Self>) -> Self {
        let git_store = project.read(cx).git_store().clone();
        let (checkpoints_tx, mut checkpoints_rx) = mpsc::unbounded();
        let checkpoint = git_store.read(cx).checkpoint(cx);
        let base = cx
            .background_spawn(
                project
                    .read(cx)
                    .git_store()
                    .read(cx)
                    .create_index(cx)
                    .log_err(),
            )
            .shared();
        Self {
            base: base.clone(),
            status: GitStoreStatus::default(),
            diffs_by_buffer: HashMap::default(),
            git_store: git_store.clone(),
            project,
            checkpoints_tx,
            _maintain_diff: cx.spawn(async move |this, cx| {
                let mut last_checkpoint = checkpoint.await.ok();
                let base = base.await.context("failed to create base")?;
                while let Some((author, checkpoint)) = checkpoints_rx.next().await {
                    if let Some(last_checkpoint) = last_checkpoint {
                        if author == ChangeAuthor::User {
                            let diff = git_store
                                .read_with(cx, |store, cx| {
                                    store.diff_checkpoints(last_checkpoint, checkpoint.clone(), cx)
                                })?
                                .await;

                            if let Ok(diff) = diff {
                                _ = git_store
                                    .read_with(cx, |store, cx| {
                                        store.apply_diff(base.clone(), diff, cx)
                                    })?
                                    .await;
                            }
                        }

                        let status = git_store
                            .read_with(cx, |store, cx| store.status(Some(base.clone()), cx))?
                            .await
                            .unwrap_or_default();
                        this.update(cx, |this, cx| this.set_status(status, cx))?;
                    }

                    last_checkpoint = Some(checkpoint);
                }
                Ok(())
            }),
        }
    }

    pub fn compute_changes(&mut self, author: ChangeAuthor, checkpoint: GitStoreCheckpoint) {
        _ = self.checkpoints_tx.unbounded_send((author, checkpoint));
    }

    fn set_status(&mut self, status: GitStoreStatus, cx: &mut Context<Self>) {
        self.status = status;
        cx.notify();
    }
}

pub struct ThreadDiffSource {
    thread_diff: Entity<ThreadDiff>,
    git_store: Entity<GitStore>,
    language_registry: Arc<LanguageRegistry>,
}

impl ThreadDiffSource {
    pub fn new(
        thread_diff: Entity<ThreadDiff>,
        git_store: Entity<GitStore>,
        language_registry: Arc<LanguageRegistry>,
    ) -> Self {
        Self {
            thread_diff,
            git_store,
            language_registry,
        }
    }
}

impl git_ui::project_diff::DiffSource for ThreadDiffSource {
    fn status(&self, cx: &App) -> Vec<git_ui::project_diff::StatusEntry> {
        let mut results = Vec::new();

        for (repo, repo_path, status) in self
            .thread_diff
            .read(cx)
            .status
            .entries(&self.git_store, cx)
        {
            let Some(project_path) = repo.read(cx).repo_path_to_project_path(repo_path, cx) else {
                continue;
            };

            let status = match *status {
                git::status::FileStatus::Tracked(mut tracked_status) => {
                    if tracked_status.worktree_status == git::status::StatusCode::Unmodified {
                        continue;
                    } else {
                        tracked_status.index_status = git::status::StatusCode::Unmodified;
                        git::status::FileStatus::Tracked(tracked_status)
                    }
                }
                status @ _ => status,
            };

            results.push(git_ui::project_diff::StatusEntry {
                project_path,
                status,
                has_conflict: false,
            });
        }

        results
    }

    fn open_uncommitted_diff(
        &self,
        buffer: Entity<Buffer>,
        cx: &mut App,
    ) -> Task<Result<Entity<BufferDiff>>> {
        let base = self.thread_diff.read(cx).base.clone();
        let git_store = self.git_store.clone();
        let language_registry = self.language_registry.clone();
        let thread_diff = self.thread_diff.clone();
        cx.spawn(async move |cx| {
            let base = base.await.context("failed to load diff base")?;
            let base_text = git_store
                .read_with(cx, |git, cx| git.load_index_text(Some(base), &buffer, cx))?
                .await;
            let snapshot = buffer.read_with(cx, |buffer, _cx| buffer.snapshot())?;

            let diff = thread_diff.update(cx, |thread_diff, cx| {
                thread_diff
                    .diffs_by_buffer
                    .entry(buffer.clone())
                    .or_insert_with(|| cx.new(|cx| BufferDiff::new(&snapshot, cx)))
                    .clone()
            })?;
            let base_text = Arc::new(base_text.unwrap_or_default());
            let diff_snapshot = BufferDiff::update_diff(
                diff.clone(),
                snapshot.text.clone(),
                Some(base_text),
                true,
                false,
                snapshot.language().cloned(),
                Some(language_registry),
                cx,
            )
            .await?;
            diff.update(cx, |diff, cx| {
                diff.set_snapshot(&snapshot, diff_snapshot, false, None, cx);
            })?;
            Ok(diff)
        })
    }
}
