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
//!  Trash(ProjectPath)                →  Trashed(TrashedEntry)
//!  Rename(ProjectPath, ProjectPath)  →  Renamed(ProjectPath, ProjectPath)
//!  Restore(TrashedEntry)             →  Restored(ProjectPath)
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
//! Record          Trashed(TrashedEntry(1))
//! History
//! 	0 Created(src/main.rs)
//! 	1 Renamed(README.md, readme.md) ─┐
//!     2 +++cursor+++                   │(before the cursor)
//! 	2 Trashed(TrashedEntry(1))       │
//!                                      │
//! User Operation  Undo                 v
//! Execute         Renamed(README.md, readme.md) ───> Rename(readme.md, README.md)
//! Record          Renamed(readme.md, README.md)
//! History
//! 	0 Created(src/main.rs)
//!     1 +++cursor+++
//! 	1 Renamed(readme.md, README.md) ─┐ (at the cursor)
//! 	2 Trashed(TrashedEntry(1))       │
//!                                      │
//!   ┌──────────────────────────────────┴─────────────────────────────────────────┐
//!     Redoing will take the result at the cursor position, convert that into the
//!     operation that can revert that result, execute that operation and replace
//!     the result in the history with the new result, obtained from running the
//!     inverse operation, advancing the cursor position.
//!   └──────────────────────────────────┬─────────────────────────────────────────┘
//!                                      │
//!                                      │
//! User Operation  Redo                 v
//! Execute         Renamed(readme.md, README.md) ───> Rename(README.md, readme.md)
//! Record          Renamed(README.md, readme.md)
//! History
//! 	0 Created(src/main.rs)
//! 	1 Renamed(README.md, readme.md)
//!     2 +++cursor+++
//! 	2 Trashed(TrashedEntry(1))────┐ (at the cursor)
//!                                   │
//! User Operation  Redo              v
//! Execute         Trashed(TrashedEntry(1)) ────────> Restore(TrashedEntry(1))
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

use crate::ProjectPanel;
use anyhow::{Context, Result, anyhow};
use fs::TrashedEntry;
use futures::channel::mpsc;
use gpui::{AppContext, AsyncApp, SharedString, Task, WeakEntity};
use project::{ProjectPath, WorktreeId};
use std::sync::atomic::{AtomicBool, Ordering};
use std::{collections::VecDeque, sync::Arc};
use ui::App;
use workspace::{
    Workspace,
    notifications::{NotificationId, simple_message_notification::MessageNotification},
};
use worktree::CreatedEntry;

enum Operation {
    Trash(ProjectPath),
    Rename(ProjectPath, ProjectPath),
    Restore(WorktreeId, TrashedEntry),
    Batch(Vec<Operation>),
}

impl Operation {
    async fn execute(self, undo_manager: &Inner, cx: &mut AsyncApp) -> Result<Change> {
        Ok(match self {
            Operation::Trash(project_path) => {
                let trash_entry = undo_manager.trash(&project_path, cx).await?;
                Change::Trashed(project_path.worktree_id, trash_entry)
            }
            Operation::Rename(from, to) => {
                undo_manager.rename(&from, &to, cx).await?;
                Change::Renamed(from, to)
            }
            Operation::Restore(worktree_id, trashed_entry) => {
                let project_path = undo_manager.restore(worktree_id, trashed_entry, cx).await?;
                Change::Restored(project_path)
            }
            Operation::Batch(operations) => {
                let mut res = Vec::new();
                for op in operations {
                    res.push(Box::pin(op.execute(undo_manager, cx)).await?);
                }
                Change::Batched(res)
            }
        })
    }
}

#[derive(Clone, Debug)]
pub(crate) enum Change {
    Created(ProjectPath),
    Trashed(WorktreeId, TrashedEntry),
    Renamed(ProjectPath, ProjectPath),
    Restored(ProjectPath),
    Batched(Vec<Change>),
}

impl Change {
    fn to_inverse(self) -> Operation {
        match self {
            Change::Created(project_path) => Operation::Trash(project_path),
            Change::Trashed(worktree_id, trashed_entry) => {
                Operation::Restore(worktree_id, trashed_entry)
            }
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
    can_undo: Arc<AtomicBool>,
    can_redo: Arc<AtomicBool>,
}

impl UndoManager {
    pub fn new(
        workspace: WeakEntity<Workspace>,
        panel: WeakEntity<ProjectPanel>,
        cx: &App,
    ) -> Self {
        let (tx, rx) = mpsc::channel(1024);
        let inner = Inner::new(workspace, panel, rx);

        let this = Self {
            tx,
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
            UndoMessage::Undo => "Undo failed",
            UndoMessage::Redo => "Redo failed",
        }
    }
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
                Self::show_error(error_title, self.workspace.clone(), e.to_string(), &mut cx);
            }

            self.can_undo.store(self.can_undo(), Ordering::Relaxed);
            self.can_redo.store(self.can_redo(), Ordering::Relaxed);
        }
    }
}

impl Inner {
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
        // 	2 Trashed(TrashedEntry(1))       │
        //                                   │
        // User Operation  Undo              v
        // Failed execute  Renamed(README.md, readme.md) ───> Rename(readme.md, README.md)
        // Record nothing
        // History
        // 	0 Created(src/main.rs)
        //     1 +++cursor+++
        // 	1 Trashed(TrashedEntry(1)) -----
        //                                  |(at the cursor)
        // User Operation  Redo             v
        // Execute         Trashed(TrashedEntry(1)) ────────> Restore(TrashedEntry(1))
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
        let undo_change = self
            .history
            .remove(before_cursor)
            .expect("we can undo")
            .to_inverse()
            .execute(self, cx)
            .await?;
        self.history.insert(before_cursor, undo_change);
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
        let redo_change = self
            .history
            .remove(self.cursor)
            .expect("we can redo")
            .to_inverse()
            .execute(self, cx)
            .await?;
        self.history.insert(self.cursor, redo_change);
        self.cursor += 1;
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

        let res: Result<Task<Result<CreatedEntry>>> = workspace.update(cx, |workspace, cx| {
            workspace.project().update(cx, |project, cx| {
                let entry_id = project
                    .entry_for_path(from, cx)
                    .map(|entry| entry.id)
                    .ok_or_else(|| anyhow!("No entry for path."))?;

                Ok(project.rename_entry(entry_id, to.clone(), cx))
            })
        });

        res?.await
    }

    async fn trash(&self, project_path: &ProjectPath, cx: &mut AsyncApp) -> Result<TrashedEntry> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Err(anyhow!("Failed to obtain workspace."));
        };

        workspace
            .update(cx, |workspace, cx| {
                workspace.project().update(cx, |project, cx| {
                    let entry_id = project
                        .entry_for_path(&project_path, cx)
                        .map(|entry| entry.id)
                        .ok_or_else(|| anyhow!("No entry for path."))?;

                    project
                        .delete_entry(entry_id, true, cx)
                        .ok_or_else(|| anyhow!("Worktree entry should exist"))
                })
            })?
            .await
            .and_then(|entry| {
                entry.ok_or_else(|| anyhow!("When trashing we should always get a trashentry"))
            })
    }

    async fn restore(
        &self,
        worktree_id: WorktreeId,
        trashed_entry: TrashedEntry,
        cx: &mut AsyncApp,
    ) -> Result<ProjectPath> {
        let Some(workspace) = self.workspace.upgrade() else {
            return Err(anyhow!("Failed to obtain workspace."));
        };

        workspace
            .update(cx, |workspace, cx| {
                workspace.project().update(cx, |project, cx| {
                    project.restore_entry(worktree_id, trashed_entry, cx)
                })
            })
            .await
    }

    /// Displays a notification with the provided `title` and `error`.
    fn show_error(
        title: impl Into<SharedString>,
        workspace: WeakEntity<Workspace>,
        error: String,
        cx: &mut AsyncApp,
    ) {
        workspace
            .update(cx, move |workspace, cx| {
                let notification_id =
                    NotificationId::Named(SharedString::new_static("project_panel_undo"));

                workspace.show_notification(notification_id, cx, move |cx| {
                    cx.new(|cx| MessageNotification::new(error, cx).with_title(title))
                })
            })
            .ok();
    }
}
