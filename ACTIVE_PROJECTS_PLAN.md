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

Narrowed `MultiWorkspace` down to a window shell holding a single `Entity<Workspace>`.

- Removed `Vec<Entity<Workspace>>` and `active_workspace_index`.
- Holds a single `workspace: Entity<Workspace>`.
- Kept sidebar layout, resize handle, open/close/focus logic.
- Compatibility shims (`workspaces()`, `active_workspace_index()`) in place for callers that haven't been updated.

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

- Add/extend the query to fetch threads by folder paths from the thread DB.
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
- Notification dots for completed threads.
- Keyboard navigation across the grouped structure.
- Visual indication of stale entries (folders that no longer exist on disk).
- Remove compatibility shims from MultiWorkspace (`workspaces()`, `active_workspace_index()`).
- Consider renaming `MultiWorkspace` to `WindowRoot`.

## Open Items Not Yet Phased

- How does "New Thread in... > New Worktree" work mechanically? (Creating a git worktree, opening it as a workspace, starting a thread.)
- What exactly does the window indicator look like? (Icon, badge, color?)
- How does the sidebar interact with session restore? (`ActiveProjects` loads its inert list first, then windows restore and `WorkspaceStore` populates.)
- Multiple windows showing the same project — same `Entity<Workspace>` or different instances?
- When the user adds/removes folders from a workspace, should `ActiveProjects` update its stored `PathList`? (Yes, eventually — but can defer to Phase 8.)