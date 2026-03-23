# Sidebar / Workspace Refactor — Implementation Plan v2

**Reference spec:** `PARALLEL_AGENTS_SPEC.md` (adjacent)

**Starting point:** Clean codebase, no changes from v1 applied. MultiWorkspace keeps its
`workspaces: Vec<Entity<Workspace>>` design — that model is correct.

---

## What This Plan Covers

1. **Fix persistence** so multi-workspace state survives restarts
2. **Add WorkspaceStore as a global** for cross-window workspace lookup
3. **Migrate "all workspaces" queries** from window iteration to WorkspaceStore

## What This Plan Does NOT Cover

- Sidebar project list initialization (works today, keep as-is)
- Thread execution independence (not needed — workspaces always live in a window)
- Folder add/remove splitting behavior

---

## Problem 1: Persistence Is Broken

### Current Data Flow

**Saving (while running):**
- Each `Workspace` serializes itself to the `workspaces` SQLite table via `save_workspace()`
  - Stores: `workspace_id`, `paths`, `session_id`, `window_id`, docks, pane layout, etc.
  - `window_id` is `window.window_handle().window_id().as_u64()` — the GPUI window ID
  - `session_id` is the current app session UUID
- `MultiWorkspace::serialize()` writes `MultiWorkspaceState` to KVP store
  - Keyed by `WindowId.as_u64().to_string()`
  - Contains: `active_workspace_id: Option<WorkspaceId>`, `sidebar_open: bool`

**Restoring (on startup):**
1. `restorable_workspace_locations()` calls `DB.last_session_workspace_locations(last_session_id, ...)`
2. This queries `workspaces WHERE session_id = ?` → returns `(workspace_id, paths, window_id, ...)`
3. `read_serialized_multi_workspaces()` groups these by `window_id` into `SerializedMultiWorkspace` groups
4. For each group, it reads `MultiWorkspaceState` from KVP using the old `window_id`
5. `restore_multiworkspace()` opens a window for each group, creates all workspaces, activates the one from `MultiWorkspaceState.active_workspace_id`

### What's Broken

The `window_id` is a GPUI `WindowId` — an opaque integer assigned by the windowing system.
**These change between app launches.** So:

- Step 3 groups workspaces by their old `window_id` — **this part works** because the grouping
  is relative (workspaces with the same old `window_id` end up in the same group)
- Step 4 reads `MultiWorkspaceState` from KVP keyed by the old `window_id` — **this works
  by coincidence** because the KVP key matches the `window_id` stored in the workspace rows.
  Both came from the same session, so they agree.

**The actual bug:** After restart, the *new* window gets a *new* `WindowId`. When `MultiWorkspace::serialize()`
writes state, it uses the new `WindowId` as the KVP key. But the workspace rows still have the *old*
`window_id`. If you quit and restart again, the grouping uses the old-old `window_id`, but the KVP
has the old (not old-old) `window_id`. They diverge.

**In practice:** The `active_workspace_id` and `sidebar_open` state from `MultiWorkspaceState` may
not be restored correctly after multiple restart cycles. The workspace grouping (which workspaces
share a window) does work because `save_workspace` updates `window_id` on each save, and
`session_workspaces` queries by `session_id` which is fresh each session.

### Fix

**Option A: Store MultiWorkspaceState alongside workspace rows, not in KVP.**
Add `active` boolean and `sidebar_open` boolean columns to the `workspaces` table (or a new
`multi_workspace_state` table keyed by `workspace_id` rather than `window_id`). On save, mark
the active workspace's row. On restore, read it from the same query.

**Option B: Re-key MultiWorkspaceState by something stable.**
Instead of keying by `WindowId`, key by a stable window identifier (e.g., a UUID generated
when the window is first created, stored on `MultiWorkspace`, and persisted). On restore, the
workspace rows carry this UUID, and the KVP state is read using it.

**Recommendation: Option A.** It's simpler — no new identifier to manage. The `workspaces` table
already has all the data we need. We just need to know which workspace in a group was active
and whether the sidebar was open.

### Tasks

1. **Add migration**: Add `active` (boolean, default false) column to `workspaces` table
2. **On save**: In `save_workspace()`, set `active = true` for the active workspace, `active = false`
   for others in the same window. Store `sidebar_open` on the workspace row too (or in a
   per-window-group column).
3. **On restore**: `session_workspaces` query already returns all workspaces grouped by `window_id`.
   Read the `active` flag to know which one to activate. Read `sidebar_open` from the group.
4. **Remove KVP-based MultiWorkspaceState**: Delete `read_multi_workspace_state`,
   `write_multi_workspace_state`, and the `MultiWorkspaceState` struct.
5. **Update `MultiWorkspace::serialize()`**: Instead of writing to KVP, trigger workspace saves
   that include the active/sidebar state.

### Files to Change

| File | What |
|------|------|
| `persistence.rs` | Add migration, update `save_workspace`, update `session_workspaces` query |
| `persistence/model.rs` | Remove `MultiWorkspaceState`, update `SerializedMultiWorkspace` |
| `multi_workspace.rs` | Update `serialize()` to write to DB instead of KVP |
| `workspace.rs` | Update `restore_multiworkspace` to read active state from DB |

---

## Problem 2: No Global Cross-Window Workspace Lookup

### Current State

`WorkspaceStore` exists on `AppState` and tracks all workspaces via
`HashSet<(AnyWindowHandle, WeakEntity<Workspace>)>`. But:
- It's only used for collab follow/unfollow protocol
- There's no way to ask "is this project open in any window?"
- Callers that need "all workspaces" iterate windows and call `multi_workspace.workspaces()`

### What We Need

When clicking a thread in the sidebar whose project lives in another window, we need to find
that window and focus it. The spec says: "If you have that project opened in a different window
then we will focus that window for you."

### Tasks

1. **Add `WorkspaceStore::global(cx)`**: Register WorkspaceStore as a GPUI global so it can be
   accessed without going through `AppState`. Add `global()`, `set_global()`, `test()` methods.
   Set the global in `WorkspaceStore::new()`.

2. **Add `find_workspace_for_paths(path_list, cx)`**: Search all registered workspaces for one
   matching the given paths. Returns `Option<(AnyWindowHandle, Entity<Workspace>)>`.

3. **Wire into sidebar**: When clicking a project header or thread, check
   `WorkspaceStore::find_workspace_for_paths()` first. If found in a different window, focus
   that window. If found in the current window, activate it. If not found, create it.

### Files to Change

| File | What |
|------|------|
| `workspace.rs` (WorkspaceStore) | Add global registration, `find_workspace_for_paths()` |
| `sidebar.rs` | Use WorkspaceStore for cross-window lookup before activating |

---

## Problem 3: "All Workspaces" Queries Go Through Windows

### Current State

Several places iterate `cx.windows()` and call `multi_workspace.workspaces()` to do something
to every workspace. This is indirect and fragile.

### Callers to Migrate

| File | What it does | Migration |
|------|-------------|-----------|
| `vim/state.rs` | Register keybindings on all workspaces | Use `WorkspaceStore::global(cx).read(cx).workspaces()` |
| `notifications.rs` | Show/dismiss notification on all workspaces | Same |
| `settings_ui.rs` (`all_projects`) | Collect projects from all workspaces | Same |
| `workspace.rs` (`find_existing_workspace`) | Find workspace matching paths | Use `find_workspace_for_paths()` |
| `workspace.rs` (`workspace_windows_for_location`) | Find windows with matching workspace | Use WorkspaceStore |
| `workspace.rs` (`join_in_room_project`) | Find workspace with matching remote project | Use WorkspaceStore |

### Tasks

1. Add `WorkspaceStore::global()` (from Problem 2)
2. Migrate each caller to use WorkspaceStore instead of window iteration
3. Ensure test `init` functions set up WorkspaceStore (add `WorkspaceStore::test(cx)` helper)

### Files to Change

| File | What |
|------|------|
| `vim/src/state.rs` | Replace window iteration with WorkspaceStore |
| `workspace/src/notifications.rs` | Replace window iteration with WorkspaceStore |
| `settings_ui/src/settings_ui.rs` | Replace AppState access with WorkspaceStore |
| `workspace/src/workspace.rs` | Replace window iteration in find/join functions |
| `vim/src/test/vim_test_context.rs` | Add WorkspaceStore::test() to init |

---

## Suggested Ordering

| Phase | Problem | Why this order |
|-------|---------|---------------|
| 1 | P2: WorkspaceStore global + lookup | Foundation — other work uses this |
| 2 | P3: Migrate "all workspaces" queries | Uses WorkspaceStore global, cleans up patterns |
| 3 | P1: Fix persistence | Independent, can be done in parallel with P2/P3 |

Phase 1 and 2 are small and mechanical. Phase 3 is the meatiest but well-scoped.

---

## Workspace Lifetime Rules (For Reference)

**Created when:**
- User opens a folder (cmd-O, CLI, recent projects picker)
- Session restore on startup

**Destroyed when:**
- User explicitly removes it from sidebar (click X on project header)
- Its worktree gets pruned (git worktree no longer exists)
- Zed restarts and session restore doesn't include it (paths no longer exist on disk)

**Never destroyed by:**
- Switching to a different project in the same window
- Opening a new project in the same window
- A thread finishing
- Closing the agent panel
