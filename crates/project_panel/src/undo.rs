//! # Undo Manager
//!
//! ## Operations and Results
//!
//! Undo and Redo actions execute an operation against the filesystem, producing
//! a result that is recorded back into the history in place of the original
//! entry. Each result is the semantic inverse of its paired operation, so the
//! cycle can repeat for continued undo and redo.
//!
//!  Operations                            Results
//!  ─────────────────────────────────  ──────────────────────────────────────
//!  Create(ProjectPath)               →  Created(ProjectPath)
//!  Trash(ProjectPath)                →  Trashed(WorktreeId, TrashId)
//!  Rename(ProjectPath, ProjectPath)  →  Renamed(ProjectPath, ProjectPath)
//!  Restore(WorktreeId, TrashId)      →  Restored(ProjectPath)
//!  Batch(Vec<Operation>)             →  Batch(Vec<Result>)
//!
//!
//! ## History and Cursor
//!
//! The undo manager maintains an operation history with a cursor position (↑).
//! Recording an operation appends it to the history and advances the cursor to
//! the end. The cursor separates past entries (left of ↑) from future entries
//! (right of ↑).
//!
//! ─ **Undo**: Takes the history entry just *before* ↑, executes its inverse,
//!   records the result back in its place, and moves ↑ one step to the left.
//! ─ **Redo**: Takes the history entry just *at* ↑, executes its inverse,
//!   records the result back in its place, and advances ↑ one step to the right.
//!
//!
//! ## Example
//!
//! User Operation  Create(src/main.rs)
//! History
//! 	0 Created(src/main.rs)
//!     1 +++cursor+++
//!
//! User Operation  Rename(README.md, readme.md)
//! History
//! 	0 Created(src/main.rs)
//! 	1 Renamed(README.md, readme.md)
//!     2 +++cursor+++
//!
//! User Operation  Create(CONTRIBUTING.md)
//! History
//! 	0 Created(src/main.rs)
//!     1 Renamed(README.md, readme.md)
//! 	2 Created(CONTRIBUTING.md) ──┐
//!     3 +++cursor+++               │(before the cursor)
//!                                  │
//!   ┌──────────────────────────────┴─────────────────────────────────────────────┐
//!     Redoing will take the result at the cursor position, convert that into the
//!     operation that can revert that result, execute that operation and replace
//!     the result in the history with the new result, obtained from running the
//!     inverse operation, advancing the cursor position.
//!   └──────────────────────────────┬─────────────────────────────────────────────┘
//!                                  │
//!                                  │
//! User Operation  Undo             v
//! Execute         Created(CONTRIBUTING.md) ────────> Trash(CONTRIBUTING.md)
//! Record          Trashed(WorktreeId(1), TrashId(1))
//! History
//! 	0 Created(src/main.rs)
//! 	1 Renamed(README.md, readme.md) ─────┐
//!     2 +++cursor+++                       │(before the cursor)
//! 	2 Trashed(WorktreeId(1), TrashId(1)) │
//!                                          │
//! User Operation  Undo                     v
//! Execute         Renamed(README.md, readme.md) ───> Rename(readme.md, README.md)
//! Record          Renamed(readme.md, README.md)
//! History
//! 	0 Created(src/main.rs)
//!     1 +++cursor+++
//! 	1 Renamed(readme.md, README.md) ─────┐ (at the cursor)
//! 	2 Trashed(WorktreeId(1), TrashId(1)) │
//!                                          │
//!   ┌──────────────────────────────────────┴─────────────────────────────────────┐
//!     Redoing will take the result at the cursor position, convert that into the
//!     operation that can revert that result, execute that operation and replace
//!     the result in the history with the new result, obtained from running the
//!     inverse operation, advancing the cursor position.
//!   └─────────────────────────────────────┬──────────────────────────────────────┘
//!                                         │
//!                                         │
//! User Operation  Redo                    v
//! Execute         Renamed(readme.md, README.md) ───> Rename(README.md, readme.md)
//! Record          Renamed(README.md, readme.md)
//! History
//! 	0 Created(src/main.rs)
//! 	1 Renamed(README.md, readme.md)
//!     2 +++cursor+++
//! 	2 Trashed(WorktreeId(1), TrashId(1)) ─┐ (at the cursor)
//!                                 │
//! User Operation  Redo            v
//! Execute         Trashed(WorktreeId(1), TrashId(1)) ─> Restore(WorktreeId(1), TrashId(1))
//! Record          Restored(ProjectPath)
//! History
//! 	0 Created(src/main.rs)
//! 	1 Renamed(README.md, readme.md)
//! 	2 Restored(ProjectPath)
//!     2 +++cursor+++

//!
//! create A;                                                      A
//! rename A -> B;                                                 B
//! undo (rename B -> A)       (takes 10s for some reason)         B (still b cause it's hanging for 10s)
//! remove B                                                       _
//! create B                                                       B
//! put important content in B                                     B
//! undo manger renames (does not hang)                            A
//! remove A                                                       _
//! user sad

//!
//! create A;                                                      A
//! rename A -> B;                                                 B
//! undo (rename B -> A)       (takes 10s for some reason)         B (still b cause it's hanging for 10s)
//! create C                                                       B
//! -- src/c.rs
//!    --

//!
//! create docs/files/ directory                                   docs/files/
//! create docs/files/a.txt                                        docs/files/
//! undo (rename B -> A)       (takes 10s for some reason)         B (still b cause it's hanging for 10s)
//! create C                                                       B
//! -- src/c.rs
//!    --

//! List of "tainted files" that the user may not operate on

use crate::{ProjectPanel, RemovalKind};
use anyhow::{Context, Result, anyhow};
use fs::{TrashId, TrashRestoreError};
use futures::channel::mpsc;
use gpui::{
    AppContext, AsyncApp, IntoElement, PromptLevel, SharedString, Styled, Task, WeakEntity,
};
use markdown::{Markdown, MarkdownElement};
use project::Project;
use project::{ProjectPath, WorktreeId};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{collections::VecDeque, sync::Arc};
use ui::{App, TextSize};
use util::{paths::PathStyle, rel_path::RelPath};
use workspace::{
    Pane, SaveIntent, Workspace,
    notifications::{
        NotificationId, markdown_style, simple_message_notification::MessageNotification,
    },
};
use worktree::CreatedEntry;

enum Operation {
    Trash(ProjectPath),
    Rename(ProjectPath, ProjectPath),
    Restore(WorktreeId, TrashId),
    Batch(Vec<Operation>),
}

impl Operation {
    /// Attempts to execute the operation, returning an `Err` if the operation
    /// failed to execute or `Ok(None)` if the operation was cancelled by the
    /// user, for example, cancelling trashing a file with unsaved edits.
    async fn execute(self, undo_manager: &Inner, cx: &mut AsyncApp) -> Result<Option<Change>> {
        let change = match self {
            Operation::Trash(project_path) => {
                let Some(trash_id) = undo_manager.trash(&project_path, cx).await? else {
                    return Ok(None);
                };
                Change::Trashed(project_path.worktree_id, trash_id)
            }
            Operation::Batch(operations) => {
                let mut trash_paths = Vec::new();

                for operation in &operations {
                    operation.trash_paths(&mut trash_paths);
                }

                if !trash_paths.is_empty()
                    && !undo_manager.confirm_batch_trash(trash_paths, cx).await?
                {
                    return Ok(None);
                }

                let mut changes = Vec::new();

                for operation in operations {
                    changes.push(Box::pin(operation.execute_confirmed(undo_manager, cx)).await?);
                }

                Change::Batched(changes)
            }
            operation => {
                return operation
                    .execute_confirmed(undo_manager, cx)
                    .await
                    .map(Some);
            }
        };

        Ok(Some(change))
    }

    /// Same as [`Self::execute`], but assumes the operation has already been
    /// confirmed in order to avoid showing confirmation modals to the user.
    ///
    /// Useful in scenarios where undoing a batch of operations would lead to
    /// multiple files being trashed, in which case we only ask the user once
    /// whether they'd like to trash the files, and then execute each trash
    /// operation without confirming each file one by one.
    async fn execute_confirmed(self, undo_manager: &Inner, cx: &mut AsyncApp) -> Result<Change> {
        let change = match self {
            Operation::Trash(project_path) => {
                let trash_id = undo_manager
                    .trash_without_confirmation(&project_path, cx)
                    .await?;

                Change::Trashed(project_path.worktree_id, trash_id)
            }
            Operation::Rename(from, to) => {
                undo_manager.rename(&from, &to, cx).await?;
                Change::Renamed(from, to)
            }
            Operation::Restore(worktree_id, trash_id) => {
                let project_path = undo_manager.restore(worktree_id, trash_id, cx).await?;
                Change::Restored(project_path)
            }
            Operation::Batch(operations) => {
                let mut changes = Vec::new();

                for operation in operations {
                    changes.push(Box::pin(operation.execute_confirmed(undo_manager, cx)).await?);
                }

                Change::Batched(changes)
            }
        };

        Ok(change)
    }

    fn trash_paths<'a>(&'a self, paths: &mut Vec<&'a ProjectPath>) {
        match self {
            Operation::Trash(path) => paths.push(path),
            Operation::Batch(operations) => {
                for operation in operations {
                    operation.trash_paths(paths);
                }
            }
            Operation::Rename(..) | Operation::Restore(..) => {}
        }
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Change {
    Created(ProjectPath),
    Trashed(WorktreeId, TrashId),
    Renamed(ProjectPath, ProjectPath),
    Restored(ProjectPath),
    Batched(Vec<Change>),
}

impl Change {
    fn to_inverse(self) -> Operation {
        match self {
            Change::Created(project_path) => Operation::Trash(project_path),
            Change::Trashed(worktree_id, trash_id) => Operation::Restore(worktree_id, trash_id),
            Change::Renamed(from, to) => Operation::Rename(to, from),
            Change::Restored(project_path) => Operation::Trash(project_path),
            // When inverting a batch of operations, we reverse the order of
            // operations to handle dependencies between them. For example, if a
            // batch contains the following order of operations:
            //
            // 1. Create `src/`
            // 2. Create `src/main.rs`
            //
            // If we first tried to revert the directory creation, it would fail
            // because there's still files inside the directory.
            Change::Batched(changes) => {
                Operation::Batch(changes.into_iter().rev().map(Change::to_inverse).collect())
            }
        }
    }
}

// Imagine pressing undo 10000+ times?!
const MAX_UNDO_OPERATIONS: usize = 10_000;

struct Inner {
    workspace: WeakEntity<Workspace>,
    panel: WeakEntity<ProjectPanel>,
    history: VecDeque<Change>,
    cursor: usize,
    /// Maximum number of operations to keep on the undo history.
    limit: usize,
    can_undo: Arc<AtomicBool>,
    can_redo: Arc<AtomicBool>,
    rx: mpsc::Receiver<UndoMessage>,
}

/// pls arc this
#[derive(Clone)]
pub struct UndoManager {
    tx: mpsc::Sender<UndoMessage>,
    is_via_collab: bool,
    can_undo: Arc<AtomicBool>,
    can_redo: Arc<AtomicBool>,
}

impl UndoManager {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        panel: WeakEntity<ProjectPanel>,
        is_via_collab: bool,
        cx: &App,
    ) -> Self {
        let (tx, rx) = mpsc::channel(1024);
        let inner = Inner::new(workspace, panel, rx);

        let this = Self {
            tx,
            is_via_collab,
            can_undo: Arc::clone(&inner.can_undo),
            can_redo: Arc::clone(&inner.can_redo),
        };

        cx.spawn(async move |cx| inner.manage_undo_and_redo(cx.clone()).await)
            .detach();

        this
    }

    pub fn undo(&mut self) -> Result<()> {
        self.tx
            .try_send(UndoMessage::Undo)
            .context("Undo and redo task can not keep up")
    }
    pub fn redo(&mut self) -> Result<()> {
        self.tx
            .try_send(UndoMessage::Redo)
            .context("Undo and redo task can not keep up")
    }
    pub fn record(&mut self, changes: impl IntoIterator<Item = Change>) -> Result<()> {
        // In a collab session, undoing or redoing can send `TrashProjectEntry`
        // or `RestoreProjectEntry`, for example, undoing a create or undoing a
        // trash.
        // Since older hosts can't decode those messages, which would
        // silently never complete, besides disabling the `Undo`/`Redo` actions
        // for collab, we also avoid recording history here so there's nothing
        // that could later trigger those messages.
        if self.is_via_collab {
            return Ok(());
        }

        self.tx
            .try_send(UndoMessage::Changed(changes.into_iter().collect()))
            .context("Undo and redo task can not keep up")
    }
    /// just for the UI, an undo may still fail if there are concurrent file
    /// operations happening.
    pub fn can_undo(&self) -> bool {
        self.can_undo.load(Ordering::Relaxed)
    }
    /// just for the UI, an undo may still fail if there are concurrent file
    /// operations happening.
    pub fn can_redo(&self) -> bool {
        self.can_redo.load(Ordering::Relaxed)
    }

    #[cfg(test)]
    pub(crate) fn set_is_via_collab(&mut self, is_via_collab: bool) {
        self.is_via_collab = is_via_collab;
    }
}

#[derive(Debug)]
enum UndoMessage {
    Changed(Vec<Change>),
    Undo,
    Redo,
}

impl UndoMessage {
    fn error_title(&self) -> &'static str {
        match self {
            UndoMessage::Changed(_) => {
                "this is a bug in the manage_undo_and_redo task please report"
            }
            UndoMessage::Undo => "Undo Failed",
            UndoMessage::Redo => "Redo Failed",
        }
    }
}

fn project_path_display(
    project: &Project,
    project_path: &ProjectPath,
    path_style: PathStyle,
    cx: &App,
) -> String {
    project
        .short_full_path_for_project_path(project_path, cx)
        .unwrap_or_else(|| project_path.path.display(path_style).to_string())
}

impl Inner {
    async fn manage_undo_and_redo(mut self, mut cx: AsyncApp) {
        loop {
            let Ok(new) = self.rx.recv().await else {
                // project panel got closed
                return;
            };

            let error_title = new.error_title();
            let res = match new {
                UndoMessage::Changed(changes) => {
                    self.record(changes);
                    Ok(())
                }
                UndoMessage::Undo => {
                    let res = self.undo(&mut cx).await;
                    let _ = self.panel.update(&mut cx, |_, cx| cx.notify());
                    res
                }
                UndoMessage::Redo => {
                    let res = self.redo(&mut cx).await;
                    let _ = self.panel.update(&mut cx, |_, cx| cx.notify());
                    res
                }
            };

            if let Err(e) = res {
                Self::show_error(
                    error_title,
                    self.workspace.clone(),
                    format!("{e:#}"),
                    &mut cx,
                );
            }

            self.can_undo.store(self.can_undo(), Ordering::Relaxed);
            self.can_redo.store(self.can_redo(), Ordering::Relaxed);
        }
    }

    pub fn new(
        workspace: WeakEntity<Workspace>,
        panel: WeakEntity<ProjectPanel>,
        rx: mpsc::Receiver<UndoMessage>,
    ) -> Self {
        Self::new_with_limit(workspace, panel, MAX_UNDO_OPERATIONS, rx)
    }

    pub fn new_with_limit(
        workspace: WeakEntity<Workspace>,
        panel: WeakEntity<ProjectPanel>,
        limit: usize,
        rx: mpsc::Receiver<UndoMessage>,
    ) -> Self {
        Self {
            workspace,
            panel,
            history: VecDeque::new(),
            cursor: 0usize,
            limit,
            can_undo: Arc::new(AtomicBool::new(false)),
            can_redo: Arc::new(AtomicBool::new(false)),
            rx,
        }
    }

    pub fn can_undo(&self) -> bool {
        self.cursor > 0
    }

    pub fn can_redo(&self) -> bool {
        self.cursor < self.history.len()
    }

    pub async fn undo(&mut self, cx: &mut AsyncApp) -> Result<()> {
        if !self.can_undo() {
            return Ok(());
        }

        // Undo failure:
        //
        // History
        // 	0 Created(src/main.rs)
        // 	1 Renamed(README.md, readme.md) ─┐
        //     2 +++cursor+++                │(before the cursor)
        // 	2 Trashed(WorktreeId(1), TrashId(1)) │
        //                                   │
        // User Operation  Undo              v
        // Failed execute  Renamed(README.md, readme.md) ───> Rename(readme.md, README.md)
        // Record nothing
        // History
        // 	0 Created(src/main.rs)
        //     1 +++cursor+++
        // 	1 Trashed(WorktreeId(1), TrashId(1)) ---------
        //                                             |(at the cursor)
        // User Operation  Redo                        v
        // Execute         Trashed(WorktreeId(1), TrashId(1)) ─> Restore(WorktreeId(1), TrashId(1))
        // Record          Restored(ProjectPath)
        // History
        // 	0 Created(src/main.rs)
        // 	1 Restored(ProjectPath)
        //  1 +++cursor+++

        // We always want to move the cursor back regardless of whether undoing
        // succeeds or fails, otherwise the cursor could end up pointing to a
        // position outside of the history, as we remove the change before the
        // cursor, in case undo fails.
        let before_cursor = self.cursor - 1; // see docs above
        self.cursor -= 1; // take a step back into the past

        // If undoing fails, the user would be in a stuck state from which
        // manual intervention would likely be needed in order to undo. As such,
        // we remove the change from the `history` even before attempting to
        // execute its inversion.
        let change = self.history.remove(before_cursor).expect("we can undo");
        let operation = change.clone().to_inverse();

        match operation.execute(self, cx).await? {
            Some(undo_change) => self.history.insert(before_cursor, undo_change),
            None => {
                self.history.insert(before_cursor, change);
                self.cursor += 1;
            }
        };

        Ok(())
    }

    pub async fn redo(&mut self, cx: &mut AsyncApp) -> Result<()> {
        if !self.can_redo() {
            return Ok(());
        }

        // If redoing fails, the user would be in a stuck state from which
        // manual intervention would likely be needed in order to redo. As such,
        // we remove the change from the `history` even before attempting to
        // execute its inversion.
        let change = self.history.remove(self.cursor).expect("we can redo");
        let operation = change.clone().to_inverse();
        match operation.execute(self, cx).await? {
            Some(redo_change) => {
                self.history.insert(self.cursor, redo_change);
                self.cursor += 1;
            }
            None => self.history.insert(self.cursor, change),
        }
        Ok(())
    }

    /// Passed in changes will always be performed as a single step
    pub fn record(&mut self, mut changes: Vec<Change>) {
        let change = match changes.len() {
            0 => return,
            1 => changes.remove(0),
            _ => Change::Batched(changes),
        };

        // When recording a new change, discard any changes that could still be
        // redone.
        if self.cursor < self.history.len() {
            self.history.drain(self.cursor..);
        }

        // Ensure that the number of recorded changes does not exceed the
        // maximum amount of tracked changes.
        if self.history.len() >= self.limit {
            self.history.pop_front();
        } else {
            self.cursor += 1;
        }

        self.history.push_back(change);
    }

    async fn rename(
        &self,
        from: &ProjectPath,
        to: &ProjectPath,
        cx: &mut AsyncApp,
    ) -> Result<CreatedEntry> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Err(anyhow!("Failed to obtain workspace."));
        };

        let (from_name, to_name) = workspace.update(cx, |workspace, cx| {
            let project = workspace.project().read(cx);
            let path_style = project.path_style(cx);

            (
                project_path_display(project, from, path_style, cx),
                project_path_display(project, to, path_style, cx),
            )
        });

        // Since the Project Panel's rename operation is used for both renaming
        // and moving files and directories, we'll assume that, if both paths
        // share the parent folder, then it was a simple rename, otherwise it
        // was a move.
        let operation = if from.path.parent() == to.path.parent() {
            "rename"
        } else {
            "move"
        };

        let res: Result<Task<Result<CreatedEntry>>> = workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                let entry_id = project
                    .entry_for_path(from, cx)
                    .map(|entry| entry.id)
                    .with_context(|| {
                        format!("Failed to {operation} `{from_name}`. It no longer exists.")
                    })?;

                Ok(project.rename_entry(entry_id, to.clone(), cx))
            })
        });

        res?.await.map_err(|err| {
            // It is possible for `RealFs::rename` to return an error other than
            // `io::Error` when the file already exists, hence why we're also
            // checking if the error contains the "already exists" string.
            let already_exists = err.chain().any(|cause| {
                cause
                    .downcast_ref::<std::io::Error>()
                    .is_some_and(|io_err| io_err.kind() == std::io::ErrorKind::AlreadyExists)
            }) || format!("{err:#}").contains("already exists");

            if already_exists {
                anyhow!(
                    "Failed to {operation} `{from_name}` to `{to_name}`. A file or folder already exists there."
                )
            } else {
                err
            }
        })
    }

    /// Trashes the specified `project_path` after prompting the user for
    /// confirmation. If the path has unsaved changes, the prompt also lets the
    /// user save or discard them.
    ///
    /// Returns `Ok(None)` if the user cancels the operation.
    async fn trash(
        &self,
        project_path: &ProjectPath,
        cx: &mut AsyncApp,
    ) -> Result<Option<TrashId>> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Err(anyhow!("Failed to obtain workspace."));
        };

        let name = workspace.update(cx, |workspace, cx| {
            let project = workspace.project().read(cx);
            let path_style = project.path_style(cx);

            project_path_display(project, project_path, path_style, cx)
        });

        if !self.confirm_trash(project_path, &name, cx).await? {
            return Ok(None);
        }

        self.trash_without_confirmation(project_path, cx)
            .await
            .map(Some)
    }

    /// Same as [`Self::trash`] but proceeds without confirming if the user
    /// wishes to trash the file.
    async fn trash_without_confirmation(
        &self,
        project_path: &ProjectPath,
        cx: &mut AsyncApp,
    ) -> Result<TrashId> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Err(anyhow!("Failed to obtain workspace."));
        };

        let name = workspace.update(cx, |workspace, cx| {
            let project = workspace.project().read(cx);
            let path_style = project.path_style(cx);

            project_path_display(project, project_path, path_style, cx)
        });

        let task = workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                let entry_id = project
                    .entry_for_path(project_path, cx)
                    .map(|entry| entry.id)
                    .with_context(|| format!("Failed to trash `{name}`. It no longer exists."))?;

                project
                    .trash_entry(entry_id, cx)
                    .with_context(|| format!("Failed to trash `{name}`."))
            })
        })?;

        match task.await {
            Ok(trash_id) => Ok(trash_id),
            Err(err) => Err(err).context(format!("Failed to trash `{name}`.")),
        }
    }

    async fn restore(
        &self,
        worktree_id: WorktreeId,
        trash_id: TrashId,
        cx: &mut AsyncApp,
    ) -> Result<ProjectPath> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Err(anyhow!("Failed to obtain workspace."));
        };

        let name = workspace
            .update(cx, |workspace, cx| {
                let project = workspace.project().read(cx);
                let path_style = project.path_style(cx);
                let original_path = project.fs().original_path_for_trash_id(trash_id)?;
                let original_name = original_path
                    .file_name()
                    .map(|name| name.to_string_lossy().into_owned())
                    .unwrap_or_else(|| original_path.display().to_string());

                let project_path = (|| {
                    let worktree = project.worktree_for_id(worktree_id, cx)?;
                    let worktree_abs_path = worktree.read(cx).abs_path();
                    let relative_path = original_path
                        .strip_prefix(worktree_abs_path.as_ref())
                        .ok()?;
                    let relative_path = RelPath::new(relative_path, path_style).ok()?;
                    Some(ProjectPath {
                        worktree_id,
                        path: relative_path.into_arc(),
                    })
                })();

                Some(project_path.map_or(original_name, |project_path| {
                    project_path_display(project, &project_path, path_style, cx)
                }))
            })
            .unwrap_or_else(|| "item".to_string());

        workspace
            .update(cx, |workspace, cx| {
                workspace.project().update(cx, |project, cx| {
                    project.restore_entry(worktree_id, trash_id, cx)
                })
            })
            .await
            .map_err(|err| match err.downcast_ref::<TrashRestoreError>() {
                Some(TrashRestoreError::Collision { .. }) => anyhow!(
                    "Failed to restore `{name}`. Something already exists at its original location."
                ),
                _ => anyhow!("Failed to restore `{name}`. It may have been permanently deleted."),
            })
    }

    /// Displays a notification with the provided `title` and `error`. The
    /// `error` is rendered as markdown, so file names wrapped in backticks show
    /// up as inline code.
    fn show_error(
        title: impl Into<SharedString>,
        workspace: WeakEntity<Workspace>,
        error: String,
        cx: &mut AsyncApp,
    ) {
        let title = title.into();
        workspace
            .update(cx, move |workspace, cx| {
                let notification_id =
                    NotificationId::Named(SharedString::new_static("project_panel_undo"));

                workspace.show_notification(notification_id, cx, move |cx| {
                    cx.new(move |cx| {
                        let markdown = cx.new(|cx| Markdown::new(error.into(), None, None, cx));
                        MessageNotification::new_from_builder(cx, move |window, cx| {
                            MarkdownElement::new(markdown.clone(), markdown_style(window, cx))
                                .text_size(TextSize::Default.rems(cx))
                                .into_any_element()
                        })
                        .with_title(title)
                    })
                })
            })
            .ok();
    }

    /// Prompts the user to confirm whether they really want to trash the file.
    /// In the case the file has unsaved changes, the prompt will ask whether
    /// the user wants to save or discard these changes.
    ///
    /// Returns `true` if the user confirmed they want to trash the file,
    /// `false` otherwise.
    async fn confirm_trash(
        &self,
        project_path: &ProjectPath,
        name: &str,
        cx: &mut AsyncApp,
    ) -> Result<bool> {
        let open_item = self.workspace.update(cx, |workspace, cx| {
            workspace.panes().iter().find_map(|pane| {
                pane.read(cx).items().find_map(|item| {
                    (item.is_dirty(cx)
                        && item
                            .project_path(cx)
                            .iter()
                            .any(|item_path| item_path == project_path))
                    .then(|| (pane.clone(), item.boxed_clone()))
                })
            })
        })?;

        let Some((pane, item)) = open_item else {
            return self.trash_prompt(&[name], 0, cx).await;
        };

        let mut async_window_cx = self
            .panel
            .update_in(cx, |_panel, window, cx| window.to_async(cx))?;
        let project = self
            .panel
            .read_with(cx, |panel, _cx| panel.project.clone())?;

        Pane::save_item(
            project,
            pane,
            item.as_ref(),
            SaveIntent::Close,
            &mut async_window_cx,
        )
        .await
    }

    /// Prompts the user to confirm whether they really want to trash all of the
    /// provided `trash_paths`.
    ///
    /// Meant to be used when executing a batch of operations that will lead to
    /// one or more paths being trashed.
    ///
    /// Returns `true` if the user confirmed they want to trash the files,
    /// `false` otherwise.
    async fn confirm_batch_trash(
        &self,
        trash_paths: Vec<&ProjectPath>,
        cx: &mut AsyncApp,
    ) -> Result<bool> {
        let (names, dirty_buffers) = self.workspace.update(cx, |workspace, cx| {
            let project = workspace.project().read(cx);
            let path_style = project.path_style(cx);
            let dirty_buffers = project
                .dirty_buffers(cx)
                .filter(|dirty_path| trash_paths.contains(&dirty_path))
                .count();
            let names = trash_paths
                .iter()
                .map(|project_path| project_path_display(project, project_path, path_style, cx))
                .collect::<Vec<_>>();

            (names, dirty_buffers)
        })?;

        self.trash_prompt(&names, dirty_buffers, cx).await
    }

    /// Prompts the user to confirm whether they actually want to trash the
    /// file.
    ///
    /// Returns `true` if the user confirms, `false` otherwise.
    async fn trash_prompt<S>(
        &self,
        names: &[S],
        dirty_buffers: usize,
        cx: &mut AsyncApp,
    ) -> Result<bool>
    where
        S: AsRef<str>,
    {
        let prompt = ProjectPanel::build_removal_prompt(RemovalKind::Trash, names, dirty_buffers);
        let answer = self
            .panel
            .update_in(cx, |_panel, window, cx| {
                window.prompt(
                    PromptLevel::Info,
                    &prompt.message,
                    prompt.detail,
                    &[prompt.confirmation_label, "Cancel"],
                    cx,
                )
            })?
            .await?;

        Ok(answer == 0)
    }
}
