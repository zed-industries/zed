use std::sync::Arc;

use anyhow::Result;
use buffer_diff::BufferDiff;
use futures::StreamExt;
use git::{
    repository::RepoPath,
    status::{DiffTreeType, FileStatus, TreeDiff},
};
use gpui::{
    AppContext, AsyncWindowContext, Context, Entity, EventEmitter, SharedString, Subscription,
    Task, WeakEntity, Window,
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

        // for file in diff.files {
        //     let Some(path) = repo.update(cx, |repo, cx| {
        //         repo.repo_path_to_project_path(&file.path, cx)
        //     })?
        //     else {
        //         continue;
        //     };
        //     let open_buffer = project
        //         .update(cx, |project, cx| project.open_buffer(path.clone(), cx))?
        //         .await;

        //     let mut status = FileStatus::Tracked(TrackedStatus {
        //         index_status: git::status::StatusCode::Unmodified,
        //         worktree_status: git::status::StatusCode::Modified,
        //     });
        //     let buffer = match open_buffer {
        //         Ok(buffer) => buffer,
        //         Err(err) => {
        //             let exists = project.read_with(cx, |project, cx| {
        //                 project.entry_for_path(&path, cx).is_some()
        //             })?;
        //             if exists {
        //                 return Err(err);
        //             }
        //             status = FileStatus::Tracked(TrackedStatus {
        //                 index_status: git::status::StatusCode::Unmodified,
        //                 worktree_status: git::status::StatusCode::Deleted,
        //             });
        //             cx.new(|cx| Buffer::local("", cx))?
        //             });
        //             cx.new(|cx| Buffer::local("", cx))?
        //         }
        //     };

        //     let buffer_snapshot = buffer.read_with(cx, |buffer, _| buffer.snapshot())?;
        //     let namespace = if file.old_text.is_none() {
        //         NEW_NAMESPACE
        //     } else {
        //         TRACKED_NAMESPACE
        //     };

        //     let buffer_diff = cx.new(|cx| BufferDiff::new(&buffer_snapshot, cx))?;
        //     buffer_diff
        //         .update(cx, |buffer_diff, cx| {
        //             buffer_diff.set_base_text(
        //                 file.old_text.map(Arc::new),
        //                 buffer_snapshot.language().cloned(),
        //                 Some(language_registry.clone()),
        //                 buffer_snapshot.text,
        //                 cx,
        //             )
        //         })?
        //         .await?;

        //     this.read_with(cx, |this, cx| {
        //         BufferDiffSnapshot::new_with_base_buffer(
        //             buffer.clone(),
        //             base_text,
        //             this.base_text().clone(),
        //             cx,
        //         )
        //     })?
        //     .await;

        //     this.update_in(cx, |this, window, cx| {
        //         this.multibuffer.update(cx, |multibuffer, cx| {
        //             multibuffer.add_diff(buffer_diff.clone(), cx);
        //         });
        //         this.register_buffer(
        //             DiffBuffer {
        //                 path_key: PathKey::namespaced(namespace, file.path.0),
        //                 buffer,
        //                 diff: buffer_diff,
        //                 file_status: status,
        //             },
        //             window,
        //             cx,
        //         );
        //     })?;
        // }

        // Ok(())
        let this = cx.weak_entity();

        self.project.update(cx, |_project, cx| {
            for item in repo.read(cx).cached_status() {
                let branch_diff = self
                    .tree_diff
                    .as_ref()
                    .and_then(|t| t.entries.get(&item.repo_path))
                    .cloned();
                if !item.status.has_changes() && branch_diff.is_none() {
                    continue;
                }

                let Some(project_path) =
                    repo.read(cx).repo_path_to_project_path(&item.repo_path, cx)
                else {
                    continue;
                };
                let repo = repo.clone();
                let task = cx.spawn(async move |project, cx| {
                    let buffer = project
                        .update(cx, |project, cx| project.open_buffer(project_path, cx))?
                        .await?;

                    let language_registry =
                        project.update(cx, |project, cx| project.languages().clone())?;

                    let changes;
                    if let Some(entry) = branch_diff {
                        let buffer_snapshot = buffer.update(cx, |buffer, cx| buffer.snapshot())?;
                        let content = match entry {
                            git::status::TreeDiffStatus::Added { .. } => None,
                            git::status::TreeDiffStatus::Modified { old, .. }
                            | git::status::TreeDiffStatus::Deleted { old } => Some(
                                repo.update(cx, |repo, cx| repo.load_blob_content(old, cx))?
                                    .await?,
                            ),
                        };

                        let buffer_diff = cx.new(|cx| BufferDiff::new(&buffer_snapshot, cx))?;
                        buffer_diff
                            .update(cx, |buffer_diff, cx| {
                                buffer_diff.set_base_text(
                                    content.map(Arc::new),
                                    buffer_snapshot.language().cloned(),
                                    Some(language_registry.clone()),
                                    buffer_snapshot.text,
                                    cx,
                                )
                            })?
                            .await?;
                        changes = buffer_diff;
                    } else {
                        changes = project
                            .update(cx, |project, cx| {
                                project.open_uncommitted_diff(buffer.clone(), cx)
                            })?
                            .await?;
                    }
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
