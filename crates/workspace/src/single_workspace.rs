//! Single workspace view over shared worktrees.
//!
//! This module contains the `Workspace` struct which represents a single workspace's
//! view over the shared `WorktreeStore`. Multiple `Workspace` instances can exist
//! within a `MultiWorkspace`, each with their own subset of worktrees.
//!
//! Key concepts:
//! - `Workspace` tracks which worktrees it "views" via `worktree_ids: HashSet<WorktreeId>`
//! - Worktrees are oblivious to which Workspaces reference them
//! - Per-workspace stores (DapStore, TaskStore) live here
//! - Shared stores (BufferStore, LspStore, etc.) are referenced from MultiWorkspace

use collections::HashSet;
use project::WorktreeId;

/// A single workspace's view over shared worktrees.
///
/// In the MultiWorkspace architecture, this struct represents one workspace
/// that can view a subset of all available worktrees. Multiple Workspaces
/// can reference the same worktree, enabling scenarios like having different
/// "views" of the same codebase.
pub struct Workspace {
    /// The set of worktree IDs this workspace views.
    /// This is a subset of the worktrees in the shared WorktreeStore.
    worktree_ids: HashSet<WorktreeId>,
}

impl Workspace {
    /// Creates a new empty workspace.
    pub fn new() -> Self {
        Self {
            worktree_ids: HashSet::default(),
        }
    }

    /// Returns the set of worktree IDs this workspace views.
    pub fn worktree_ids(&self) -> &HashSet<WorktreeId> {
        &self.worktree_ids
    }

    /// Adds a worktree to this workspace's view.
    pub fn add_worktree(&mut self, worktree_id: WorktreeId) {
        self.worktree_ids.insert(worktree_id);
    }

    /// Removes a worktree from this workspace's view.
    pub fn remove_worktree(&mut self, worktree_id: WorktreeId) {
        self.worktree_ids.remove(&worktree_id);
    }

    /// Returns whether this workspace views the given worktree.
    pub fn contains_worktree(&self, worktree_id: WorktreeId) -> bool {
        self.worktree_ids.contains(&worktree_id)
    }
}

impl Default for Workspace {
    fn default() -> Self {
        Self::new()
    }
}
