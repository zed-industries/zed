# Active Projects Implementation Plan (Draft)

This is a high-level sketch — phases and scope will shift after reviewing the architecture doc with fresh eyes. The goal is to show the rough shape of the work, not to commit to specifics.

## Phase 1: The Work Registry

Extend the existing `WorkspaceStore` (in `crates/workspace/src/workspace.rs`) with an `active_projects: Vec<Entity<Workspace>>` field and the persistence/lifecycle methods.

- `WorkspaceStore` already exists as an app-global entity tracking all workspaces with their window handles.
- Add the `active_projects` field — the persisted, append-mostly list of workspaces that have had threads.
- Add `persist_project()`, `remove_project()`, `detach_window()` methods.
- Add `active_projects()` and `windows_for_folder_set()` queries.
- The store emits change notifications so observers (the sidebar) can react.

`WorkspaceStore` already tracks window↔workspace associations. The new `active_projects` field is the only addition to stored state.

## Phase 2: Hollow Out MultiWorkspace

Narrow `MultiWorkspace` down to a window shell (maybe rename to `WindowRoot`).

- Remove `Vec<Entity<Workspace>>` and `active_workspace_index` from `MultiWorkspace`.
- It holds a single `Entity<Workspace>` — whatever `WorkspaceStore` says this window should display.
- Keep the sidebar layout, resize handle, open/close/focus logic.
- The sidebar reads from the global `WorkspaceStore` instead of from `MultiWorkspace.workspaces()`.
- Window-switching actions (`NextWorkspaceInWindow`, etc.) go through `WorkspaceStore`.

This is mostly deletion and re-wiring. The sidebar infrastructure stays intact.

## Phase 3: Sidebar Reads From WorkspaceStore

Rewrite the sidebar's data flow to use `WorkspaceStore` as its source.

- The sidebar observes `WorkspaceStore` for changes.
- Build the `derive_project_groups()` function that groups workspaces by `PathList`.
- Render project group headers, workspace entries underneath.
- Show window indicators for workspaces that have a window assigned.
- Thread data is still read from the existing `AgentPanel` / live thread entities for now (same as today).

At this point the sidebar shows the new grouped UI but is still only showing live/windowed workspaces. No persistence yet.

## Phase 4: Active Projects Persistence

Add the persisted active projects list — the append-mostly database table.

- Define the DB schema: folder paths + workspace ID per entry.
- When a thread is created in a workspace, persist that workspace to the active list.
- On startup, deserialize the active list and eagerly rehydrate all entries into live `Entity<Workspace>` instances.
- Auto-remove entries whose folders no longer exist on disk (stale detection).
- "Remove Project" action deletes from the persisted list and kills any running threads.

After this phase, projects survive window close and app restart. The sidebar shows the union of persisted entries and ephemeral windowed workspaces.

## Phase 5: Thread Database Integration

Enrich the sidebar with thread history from the thread database.

- Add/extend the query to fetch threads by folder paths from the thread DB.
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
- Notification dots for completed threads.
- Keyboard navigation across the grouped structure.
- Remove the defunct `NextWorkspaceInWindow` and `PreviousWorkspaceInWindow` actions (currently registered as no-ops so existing keybindings don't break).
- Rename `MultiWorkspace` to `WindowRoot` and clean up remaining compatibility shims (`workspaces()`, `active_workspace_index()`, `remove_workspace()`, `activate_index()`).

## Open Items Not Yet Phased

- How does "New Thread in... > New Worktree" work mechanically? (Creating a git worktree, opening it as a workspace, starting a thread.)
- What exactly does the window indicator look like? (Icon, badge, color?)
- How does the sidebar interact with session restore? (`WorkspaceStore` loads first, then windows restore and register themselves.)
- Multiple windows showing the same project — same `Entity<Workspace>` or different instances?