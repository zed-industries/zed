# Active Projects Implementation Plan (Draft)

This is a high-level sketch — phases and scope will shift after reviewing the architecture doc with fresh eyes. The goal is to show the rough shape of the work, not to commit to specifics.

## Phase 1: The Work Registry ✅

Extend the existing `WorkspaceStore` (in `crates/workspace/src/workspace.rs`) with an `active_projects: Vec<Entity<Workspace>>` field and the persistence/lifecycle methods.

- `WorkspaceStore` already exists as an app-global entity tracking all workspaces with their window handles.
- Add the `active_projects` field — the append-mostly list of workspaces that have had threads. (Not yet persisted to DB — in-memory only for now.)
- Add `persist_project()`, `remove_project()`, `detach_window()` methods.
- Add `active_projects()` and `windows_for_path_list()` queries.
- The store emits change notifications so observers (the sidebar) can react.

`WorkspaceStore` already tracks window↔workspace associations. The new `active_projects` field is the only addition to stored state.

**Implementation notes:**
- `persist_project()` adds to the in-memory `active_projects` list. A FIXME marks where DB persistence should go.
- `remove_project()` takes `&Entity<Workspace>` (entity identity) rather than an index for robustness.
- `detach_window()` removes the window-to-workspace association; the project stays in `active_projects` if present.
- All mutation methods only call `cx.notify()` when state actually changes.

## Phase 2: Hollow Out MultiWorkspace ✅

Narrow `MultiWorkspace` down to a window shell (rename to `WindowRoot` deferred to Phase 8).

- ✅ Replaced `workspaces: Vec<Entity<Workspace>>` and `active_workspace_index: usize` with a single `workspace: Entity<Workspace>`.
- ✅ `activate()` now simply swaps the single workspace field. It does **not** add to `active_projects` — workspaces are ephemeral until a thread is created. They are already tracked in `WorkspaceStore.workspaces` via `Workspace::new`.
- ✅ `open_sidebar()` / `close_sidebar()` operate on the single workspace directly.
- ✅ Removed `add_workspace()` — workspaces are tracked by `WorkspaceStore` at creation time.
- ✅ Removed `activate_next_workspace()` / `activate_previous_workspace()` — no-ops with single workspace.
- ✅ `remove_workspace()` is a no-op shim (a window must always have one workspace). Kept as a signal for future work.

**Compatibility shims kept for now (cleanup in Phase 8):**
- `workspaces()` → returns `std::slice::from_ref(&self.workspace)`.
- `active_workspace_index()` → always returns `0`.
- `activate_index()` → ignores index, just serializes and focuses.

**Known temporary regression:** The sidebar only sees this window's single workspace until Phase 3 wires it to `WorkspaceStore`. Ephemeral workspaces in other windows don't appear in the sidebar yet.

## Phase 3: Sidebar Reads From WorkspaceStore ✅

Rewrite the sidebar's data flow to use `WorkspaceStore` as its source.

- ✅ The sidebar observes `WorkspaceStore` for changes (in addition to `MultiWorkspace` and `ThreadStore`).
- ✅ `Sidebar::collect_all_workspaces()` computes the union of windowed workspaces (`workspaces_with_windows()`) and active projects (`active_projects()`), deduplicating by entity ID.
- ✅ `update_entries()` reads the workspace list from `WorkspaceStore` and the active workspace from `MultiWorkspace`.
- ✅ Removed the `workspaces: &[Entity<Workspace>]` parameter from `Sidebar::new` — no longer needed since `update_entries()` derives everything from `WorkspaceStore`.
- ✅ `Sidebar::new` takes `workspace_store: Entity<WorkspaceStore>` as an explicit parameter (required because the constructor is called inside `observe_new` where `MultiWorkspace` is already mutably borrowed — reading through the entity handle would cause a GPUI re-entrancy panic).
- ✅ `ActiveProjectsDelegate::new` simplified to just take `multi_workspace` and `workspace_store` — initial state is empty, immediately populated by `update_entries()`.

**Architecture of the sidebar's data sources:**
- `multi_workspace: Entity<MultiWorkspace>` — window-local operations (activate workspace in this window, create workspace, read which workspace is active in this window).
- `workspace_store: Entity<WorkspaceStore>` — global workspace list (all windowed workspaces + persisted active projects).
- `ThreadStore` (global) — thread metadata (titles, timestamps, etc.).

Thread data is still read from the existing `AgentPanel` / live thread entities (same as before). The `derive_project_groups()` function from the architecture doc is effectively what `ActiveProjects::from_workspaces()` + `collect_all_workspaces()` do today, though the thread-enrichment part (Phase 5) is not yet wired in.

## Phase 4: Active Projects Persistence

Add the persisted active projects list — the append-mostly database table.

- Define the DB schema: folder paths + workspace ID per entry.
- When a thread is created in a workspace, call `WorkspaceStore::persist_project()` which should write to the DB.
- On startup, deserialize the active list and eagerly rehydrate all entries into live `Entity<Workspace>` instances. (Note: we expect this to eventually need lazy loading as the list grows — see architecture doc addendum.)
- Auto-remove entries whose folders no longer exist on disk (stale detection).
- "Remove Project" action deletes from the persisted list and kills any running threads.

After this phase, projects survive window close and app restart. The sidebar shows the union of persisted entries and ephemeral windowed workspaces.

## Phase 5: Thread Database Integration

Enrich the sidebar with thread history from the thread database.

- Add/extend the query to fetch threads by folder paths from the thread DB. **Note:** `DbThreadMetadata` currently has `worktree_branch` but no `folder_paths` field — this schema work is needed before threads can be grouped under projects.
- Wire thread metadata into `derive_project_groups()` so threads appear under their project groups.
- Extend `SidebarThreadInfo` with the richer fields (agent name, diff stats, timestamp).
- Handle the case where a thread's folder paths don't match any active project (orphaned — just don't show it).

## Phase 6: PathList Canonicalization and Git Worktrees

Make git worktrees group correctly.

- Implement git-aware canonicalization in `path_list_from_workspace`.
- Worktrees at different physical paths but same git repo resolve to the same `PathList`.
- Add worktree annotations (e.g., "in olivetti") to `DerivedEntry`.
- Likely needs work in the git integration layer to expose "main repo path" for a worktree.

## Phase 7: Thread DB Path Reconciliation

Keep the thread database consistent when folder paths change.

- When a workspace detects a path rename (via file watcher), propagate the old→new mapping to the thread DB.
- Update stored folder paths in thread records so grouping stays correct.
- Update the persisted active projects list entries too.

## Phase 8: Polish

- Collapsible project groups with "View More" truncation.
- Context menu on project group headers (Remove Project, Collapse, New Thread, New Thread in...).
- Search/filter across the grouped view.
- Agent selector in the header.
- Notification dots for completed threads (infrastructure exists — `agent_thread_status(cx)`, `test_statuses` — but tracking logic for Running → Completed transitions is not yet implemented; 2 sidebar tests are failing on this).
- Keyboard navigation across the grouped structure.
- Remove the defunct `NextWorkspaceInWindow` and `PreviousWorkspaceInWindow` actions (currently registered as no-ops so existing keybindings don't break).
- Rename `MultiWorkspace` to `WindowRoot` (or another name TBD) and clean up remaining compatibility shims (`workspaces()`, `active_workspace_index()`, `remove_workspace()`, `activate_index()`).
- Delete the orphaned `crates/workspace/src/window_root.rs` file (leftover from a reverted rename).

## Open Items Not Yet Phased

- How does "New Thread in... > New Worktree" work mechanically? (Creating a git worktree, opening it as a workspace, starting a thread.)
- What exactly does the window indicator look like? (Icon, badge, color?) Waiting on design updates from Danilo.
- How does the sidebar interact with session restore? (`WorkspaceStore` loads first, then windows restore and register themselves.)
- Multiple windows showing the same project — same `Entity<Workspace>` or different instances? (Needs a decision before Phase 4.)
- Lazy loading of active projects at startup — the architecture doc sketches a `WorkspaceEntry` enum (`Loaded` / `Unloaded`) but this is deferred until the active projects list grows large enough to be a problem. The current design supports it cleanly.