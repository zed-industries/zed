# Active Projects Implementation Plan

This is a high-level sketch — phases and scope will shift as work progresses. The goal is to show the rough shape of the work, not to commit to specifics.

## Phase 1: The Inert Active Projects Entity ← **current work**

Create `ActiveProjects` as a standalone entity in the `sidebar` crate (`crates/sidebar/src/active_projects.rs`). It owns a persisted `Vec<PathList>` — nothing more.

- `ActiveProjects` is a GPUI global entity, same pattern as `ThreadStore`.
- It stores an inert list of `PathList` values — no live workspaces, no entities, no language servers.
- API: `projects()`, `contains()`, `add()`, `remove()`. All mutations call `cx.notify()`.
- Persisted to the KVP store. Loaded on startup as-is — no rehydration step.
- Idempotent adds: if the `PathList` is already present, `add()` is a no-op.
- Append-mostly: entries are only removed by explicit user action.

## Phase 2: Hollow Out MultiWorkspace ✅ **done**

Narrowed `MultiWorkspace` down to a window shell holding a single `Entity<Workspace>` (rename to `WindowRoot` deferred to Phase 9).

- Removed `Vec<Entity<Workspace>>` and `active_workspace_index`.
- Holds a single `workspace: Entity<Workspace>`.
- `activate()` now swaps the single workspace and rebinds the window's mapping in `WorkspaceStore` via `bind_window_to_workspace()`. It does **not** add to `active_projects` — workspaces are ephemeral until a thread is created.
- `open_sidebar()` / `close_sidebar()` operate on the single workspace directly.
- Removed `add_workspace()` — workspaces are tracked by `WorkspaceStore` at creation time.
- Removed `activate_next_workspace()` / `activate_previous_workspace()` — no-ops with single workspace.
- `remove_workspace()` is a no-op shim (a window must always have one workspace). Kept as a signal for future work.

**Compatibility shims kept for now (cleanup in Phase 9):**
- `workspaces()` → returns `std::slice::from_ref(&self.workspace)`.
- `active_workspace_index()` → always returns `0`.
- `activate_index()` → ignores index, just serializes and focuses.

## Phase 3: Make WorkspaceStore a GPUI Global ✅ **done**

- `WorkspaceStore` registered as a GPUI global (same pattern as `ThreadStore`).
- All callers migrated from iterating windows → MultiWorkspace → workspaces() to using `WorkspaceStore::global(cx)` directly.
- Removed `active_projects` field and related methods from `WorkspaceStore`.

## Phase 4: Sidebar Reads From ActiveProjects + WorkspaceStore

Rewrite the sidebar's data flow to compose the three independent sources.

- The sidebar observes `ActiveProjects`, `WorkspaceStore`, and `ThreadStore` for changes.
- Rename the sidebar's internal `ActiveProjects` struct (the grouping logic) to `ProjectGroups` to avoid confusion with the new entity.
- Build the `derive_project_groups()` function:
  - Start with inert `PathList` values from `ActiveProjects`.
  - Add any windowed workspaces not yet in the active list (ephemeral entries).
  - For each `PathList`, find live workspaces from `WorkspaceStore` (for window indicators and live path data).
- Clicking an entry with no window creates a workspace on demand.
- Clicking an entry with a window focuses that window or swaps the current window to it.
- Show window indicators for entries that have a live workspace.

At this point the sidebar shows the new grouped UI with both persisted and ephemeral entries. No thread history yet.

**Architecture of the sidebar's data sources:**
- `multi_workspace: Entity<MultiWorkspace>` — window-local operations (activate workspace in this window, create workspace, read which workspace is active in this window).
- `workspace_store: Entity<WorkspaceStore>` — global workspace list (all windowed workspaces + persisted active projects).
- `ThreadStore` (global) — thread metadata (titles, timestamps, etc.).

**Implementation notes from prior work:**
- `Sidebar::new` takes `workspace_store: Entity<WorkspaceStore>` as an explicit parameter (required because the constructor is called inside `observe_new` where `MultiWorkspace` is already mutably borrowed — reading through the entity handle would cause a GPUI re-entrancy panic).
- `ActiveProjectsDelegate::new` simplified to just take `multi_workspace` and `workspace_store` — initial state is empty, immediately populated by `update_entries()`.
- Added a cross-window affordance in the sidebar row UI: a monitor icon + tooltip (`"This workspace is open in another window."`) for workspaces currently shown by a different window.
- Fixed indicator over-reporting by keying cross-window detection on workspace entity identity (not just shared `PathList`).

## Next Immediate Work: Populate Active Threads From DB

Before continuing deeper into persistence phases, the next priority is to populate sidebar thread rows from thread database history for each active project/workspace group.

- Reuse the existing thread persistence foundation (`ThreadStore` + thread DB metadata/restore flows) rather than introducing a brand-new persistence layer.
- Focus on wiring the query/join path into sidebar derivation and rendering first.
- Defer schema expansion (for richer grouping fields) to the formal Phase 6 work items below.

## Phase 5: Active Projects Persistence

Wire up the database persistence for `ActiveProjects`.

- Serialize `Vec<PathList>` to the KVP store (single JSON blob). `PathList` already has `serialize()`/`deserialize()`.
- Load on startup, save on every mutation.
- The "persist on thread creation" trigger: when a thread is created in a workspace, compute its `PathList` and call `ActiveProjects::add()`.
- "Remove Project" action calls `ActiveProjects::remove()`.
- Auto-remove entries whose folders no longer exist on disk (optional — could defer to polish phase as a lightweight path-existence check).

After this phase, projects survive window close and app restart. The sidebar shows the union of persisted entries and ephemeral windowed workspaces.

## Phase 6: Thread Database Integration

Enrich the sidebar with thread history from the thread database.

- Add/extend the query to fetch threads by folder paths from the thread DB. **Note:** `DbThreadMetadata` currently has `worktree_branch` but no `folder_paths` field — this schema work is needed before threads can be grouped under projects.
- Wire thread metadata into `derive_project_groups()` so threads appear under their project groups.
- Extend `SidebarThreadInfo` with the richer fields (agent name, diff stats, timestamp).
- Handle the case where a thread's folder paths don't match any active project (orphaned — don't show it).

## Phase 7: PathList Canonicalization and Git Worktrees

Make git worktrees group correctly.

- Implement git-aware canonicalization in `path_list_from_workspace`.
- Worktrees at different physical paths but same git repo resolve to the same `PathList`.
- Add worktree annotations (e.g., "in olivetti") to derived entries.
- Likely needs work in the git integration layer to expose "main repo path" for a worktree.

## Phase 8: Thread DB Path Reconciliation

Keep the thread database consistent when folder paths change.

- When a live workspace detects a path rename (via file watcher), propagate the old→new mapping to the thread DB.
- Update stored folder paths in thread records so grouping stays correct.
- Update the stored `PathList` in `ActiveProjects` to match the live workspace's current paths.

## Phase 9: Polish

- Collapsible project groups with "View More" truncation.
- Context menu on project group headers (Remove Project, Collapse, New Thread, New Thread in...).
- Search/filter across the grouped view.
- Agent selector in the header.
- Notification dots for completed threads (infrastructure exists — `agent_thread_status(cx)`, `test_statuses` — but tracking logic for Running → Completed transitions is not yet implemented; 2 sidebar tests are failing on this).
- Keyboard navigation across the grouped structure.
- Visual indication of stale entries (folders that no longer exist on disk).
- Remove the defunct `NextWorkspaceInWindow` and `PreviousWorkspaceInWindow` actions (currently registered as no-ops so existing keybindings don't break).
- Rename `MultiWorkspace` to `WindowRoot` (or another name TBD) and clean up remaining compatibility shims (`workspaces()`, `active_workspace_index()`, `remove_workspace()`, `activate_index()`).
- Delete the orphaned `crates/workspace/src/window_root.rs` file (leftover from a reverted rename).

## Open Items Not Yet Phased

- How does "New Thread in... > New Worktree" work mechanically? (Creating a git worktree, opening it as a workspace, starting a thread.)
- Fine-tune the window indicator design (iconography, prominence, placement, and tooltip copy) based on team feedback from dogfooding.
- How does the sidebar interact with session restore? (`ActiveProjects` loads its inert list first, then windows restore and `WorkspaceStore` populates.)
- Multiple windows showing the same project — same `Entity<Workspace>` or different instances? (Needs a decision before Phase 5.)
- When the user adds/removes folders from a workspace, should `ActiveProjects` update its stored `PathList`? (Yes, eventually — but can defer to Phase 8.)
- Lazy loading of active projects at startup — the architecture doc sketches a `WorkspaceEntry` enum (`Loaded` / `Unloaded`) but this is deferred until the active projects list grows large enough to be a problem. The current design supports it cleanly.