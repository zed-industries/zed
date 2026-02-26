# Finishing the MultiWorkspace Refactor

## Problem

MultiWorkspace has been hollowed out to hold a single `workspace: Entity<Workspace>` instead of `Vec<Entity<Workspace>>`. But `workspaces()` still exists as a compatibility shim returning a one-element slice, and many callers still iterate it. These callers depend on `workspaces()` for different reasons, and those reasons need different solutions.

There are two remaining problems:

1. **Compatibility shim callers.** `MultiWorkspace::workspaces()` returns a one-element slice. Callers iterate it for various reasons — broadcasting operations, finding workspaces across windows, collecting projects. Each category needs a different migration to the new model.

2. **The 4th data source: headless workspaces.** When a thread is running and the user closes the window, the workspace must stay alive (threads need a live `Entity<Workspace>` for tool execution — file reads, writes, terminal commands). This workspace has no window. Currently `WorkspaceStore` only tracks `(AnyWindowHandle, WeakEntity<Workspace>)` pairs — headless workspaces don't fit. These need to live in `WorkspaceStore` so the sidebar can discover and manage them.

## The Headless Workspace Concept

A **headless workspace** is a live `Entity<Workspace>` that has no window. It exists because:

- A thread was running when the user closed its window. The workspace stays alive so the thread can finish.
- The sidebar opened a project from the active projects list to inspect its threads, but hasn't assigned it to a window yet.

Headless workspaces are the bridge between the inert `PathList` entries in `ActiveProjects` and the fully windowed workspaces in `WorkspaceStore`. The sidebar drives their lifecycle:

- **Create:** sidebar needs a live workspace for a `PathList` entry (e.g., to show live thread status, or because a thread is running).
- **Promote to windowed:** user clicks an entry → the headless workspace gets assigned to a window.
- **Demote from windowed:** user closes a window with a running thread → the workspace becomes headless.
- **Destroy:** thread finishes and no window is showing it → the headless workspace is dropped.

## WorkspaceStore Changes

`WorkspaceStore` gains a second collection for headless workspaces:

```rust
pub struct WorkspaceStore {
    /// Windowed workspaces: live in a window, visible to the user.
    /// Existing field, unchanged.
    workspaces: HashSet<(AnyWindowHandle, WeakEntity<Workspace>)>,

    /// Headless workspaces: no window, kept alive for running threads.
    /// Strong refs because the whole point is to keep them alive.
    headless: Vec<Entity<Workspace>>,

    client: Arc<Client>,
    _subscriptions: Vec<client::Subscription>,
}
```

New API:

```rust
impl WorkspaceStore {
    /// All live workspaces — both windowed and headless.
    fn all_workspaces(&self, cx: &App) -> Vec<Entity<Workspace>>;

    /// Add a headless workspace (no window).
    fn add_headless(&mut self, workspace: Entity<Workspace>, cx: &mut Context<Self>);

    /// Remove a headless workspace (thread finished, no longer needed).
    fn remove_headless(&mut self, workspace: &Entity<Workspace>, cx: &mut Context<Self>);

    /// Promote a headless workspace to windowed (user clicked it in sidebar).
    fn assign_window(&mut self, workspace: Entity<Workspace>, window: AnyWindowHandle, cx: &mut Context<Self>);

    /// Demote a windowed workspace to headless (window closed, thread still running).
    /// Returns true if the workspace was kept as headless, false if it was just removed.
    fn detach_window(&mut self, window: AnyWindowHandle, keep_alive: bool, cx: &mut Context<Self>) -> bool;
}
```

The sidebar observes `WorkspaceStore` and sees the union of windowed + headless workspaces. The `all_workspaces()` method gives it everything it needs for derivation.

## Callers of `multi_workspace.workspaces()` — What Each Actually Needs

Every caller of `MultiWorkspace::workspaces()` falls into one of a few categories. Since MultiWorkspace now holds a single workspace, `workspaces()` is a compatibility shim returning a one-element slice. Each category needs a different migration path.

### Category 1: "Do something to every workspace in this window"

These callers iterate workspaces to broadcast an operation to all of them. Since there's only one workspace per window, these become `multi_workspace.workspace()` (singular).

| Caller | File | What it does |
|--------|------|-------------|
| **Vim register** | `crates/vim/src/state.rs` L734-742 | Registers workspace with vim globals for every workspace in a window. |
| **Show app notification** | `crates/workspace/src/notifications.rs` L1042-1052 | Shows a notification in every workspace in a window. |
| **Dismiss app notification** | `crates/workspace/src/notifications.rs` L1068-1074 | Dismisses a notification from every workspace in a window. |
| **Open/close sidebar** | `crates/workspace/src/multi_workspace.rs` | Sets sidebar_open on every workspace. |
| **Quit flush** | `crates/zed/src/zed.rs` L1356-1364 | Flushes serialization for every workspace in a window. |
| **Quit collect** | `crates/zed/src/zed.rs` L1325-1327 | Collects all workspaces to prepare for quit. |

**Migration:** Replace `for workspace in multi_workspace.workspaces()` with just `multi_workspace.workspace()`. These are trivial mechanical changes.

For the quit handler specifically: it needs to flush _all_ workspaces, including headless ones. This should iterate `WorkspaceStore::all_workspaces()` instead of going window-by-window.

### Category 2: "Find a workspace matching some criteria across all windows"

These callers search across all windows to find a workspace that matches a project, location, or set of paths.

| Caller | File | What it does |
|--------|------|-------------|
| **Find existing workspace** | `crates/workspace/src/workspace.rs` L8552-8562 | Finds a window whose workspace matches a set of paths (for reuse when opening). |
| **workspace_windows_for_location** | `crates/workspace/src/workspace.rs` L8512-8522 | Finds windows showing a specific workspace location. |
| **Join room project** | `crates/workspace/src/workspace.rs` L9181-9190 | Finds a workspace with a specific remote project ID. |
| **LSP access** | `crates/zed/src/main.rs` L539-550 | Collects all LSP stores across all workspaces for extension language support. |

**Migration:** These should use `WorkspaceStore::all_workspaces()` (which includes headless) or `WorkspaceStore::workspaces_with_windows()` depending on whether they need window handles. The MultiWorkspace indirection is unnecessary — they're already reaching through to `WorkspaceStore` or iterating windows.

For LSP access specifically: headless workspaces also have LSP stores, so `all_workspaces()` is correct.

### Category 3: "Get all projects for settings/configuration"

| Caller | File | What it does |
|--------|------|-------------|
| **Settings window init** | `crates/settings_ui/src/settings_ui.rs` L1542-1545 | Gets all workspaces from WorkspaceStore to observe their projects. |
| **all_projects** | `crates/settings_ui/src/settings_ui.rs` L3745-3748 | Gets all projects from WorkspaceStore + current window's MultiWorkspace. |

**Migration:** Use `WorkspaceStore::all_workspaces()` directly. The settings UI currently does a union of `workspace_store.workspaces()` and `multi_workspace.workspaces()` with dedup — `WorkspaceStore` should be the single source. Headless workspaces should be included (they have projects with settings too).

### Category 4: Sidebar (will be rewritten separately)

| Caller | File | What it does |
|--------|------|-------------|
| **build_workspace_thread_entries** | `crates/sidebar/src/sidebar.rs` L1040-1041 | Builds sidebar entries from workspaces. |
| **subscribe_to_projects** | `crates/sidebar/src/sidebar.rs` L1007-1010 | Subscribes to worktree events. |
| **subscribe_to_agent_panels** | `crates/sidebar/src/sidebar.rs` L1106-1116 | Subscribes to agent panel events. |
| **subscribe_to_threads** | `crates/sidebar/src/sidebar.rs` L1135-1145 | Subscribes to thread entity changes. |
| **open_workspace_path_sets** | `crates/sidebar/src/sidebar.rs` L472-474 | Gets open workspace path sets for dedup. |

**Migration:** The sidebar will be rewritten to read from `ActiveProjects` + `WorkspaceStore` + `ThreadStore` as described in the Active Projects refactor plan. These callers are replaced wholesale, not migrated one-by-one.

### Category 5: Restore/serialization

| Caller | File | What it does |
|--------|------|-------------|
| **restore_multiworkspace** | `crates/workspace/src/workspace.rs` L8129-8146 | Restores workspaces from DB on session restore. |

**Migration:** Session restore creates a single workspace per window and assigns it to MultiWorkspace. No iteration needed. The restore logic for _active projects_ is separate (loading the inert `PathList` values from `ActiveProjects`).

### Category 6: Tests

| Caller | File | What it does |
|--------|------|-------------|
| **Various test assertions** | `recent_projects/tests`, `visual_test_runner`, `zed/tests` | Assert workspace counts, access by index. |

**Migration:** Update tests to use `multi_workspace.workspace()` (singular). Tests that assert `workspaces().len() >= 2` are testing the old multi-workspace-per-window model and need rethinking.

## Execution Plan

### Task A: Add headless workspace support to WorkspaceStore

1. Add `headless: Vec<Entity<Workspace>>` field to `WorkspaceStore`.
2. Add `all_workspaces()` that returns windowed + headless.
3. Add `add_headless()`, `remove_headless()`.
4. Extend `detach_window()` with a `keep_alive` parameter (or split into two methods).
5. Add `assign_window()` for promoting headless → windowed.
6. `WorkspaceStore` calls `cx.notify()` on all mutations so observers (the sidebar) react.

### Task B: Migrate Category 1 callers (broadcast to all workspaces in a window)

For each caller:
- Replace `for workspace in multi_workspace.workspaces()` with `let workspace = multi_workspace.workspace()`.
- This is mechanical and safe — there's only ever one workspace per window now.

Callers:
- [ ] `crates/vim/src/state.rs` — vim register
- [ ] `crates/workspace/src/notifications.rs` — show/dismiss app notification (2 sites)
- [ ] `crates/zed/src/zed.rs` — quit handler (2 sites)

### Task C: Migrate Category 2 callers (find workspace across all windows)

For each caller:
- Replace the pattern of iterating windows → reading MultiWorkspace → iterating workspaces with a single call to `WorkspaceStore::all_workspaces()` or `workspaces_with_windows()`.

Callers:
- [ ] `crates/workspace/src/workspace.rs` — `find_existing_workspace`
- [ ] `crates/workspace/src/workspace.rs` — `workspace_windows_for_location`
- [ ] `crates/workspace/src/workspace.rs` — `join_in_room_project`
- [ ] `crates/zed/src/main.rs` — LSP access for extensions

### Task D: Migrate Category 3 callers (settings/configuration)

- [ ] `crates/settings_ui/src/settings_ui.rs` — `SettingsWindow::new` and `all_projects`
- Replace the union-with-dedup pattern with a single `WorkspaceStore::all_workspaces()` call.

### Task E: Redo restore/serialization for single-workspace-per-window

The entire `restore_multiworkspace` function and `MultiWorkspaceState` serialization model are shaped around the old "N workspaces per window" world. They need to be simplified for the new model where one window == one workspace.

What exists today:
- `MultiWorkspaceState` stores `active_workspace_id` and `sidebar_open`, serialized to KVP keyed by `WindowId`.
- `restore_multiworkspace` restores N workspaces, finds which one matches `active_workspace_id`, and calls `activate_index`.
- The `activate_index` / `workspaces().iter().position()` dance is all ceremony for a model that no longer exists.

What it should become:
- Restore creates one workspace, puts it in the window. Done.
- `MultiWorkspaceState` simplifies to just `sidebar_open` (and maybe window bounds). The `active_workspace_id` concept is meaningless when there's only one workspace.
- The `serialize` method on MultiWorkspace simplifies correspondingly.

Open question for headless workspaces: **all callers of `workspaces_with_windows()`** only see windowed workspaces. Once headless workspaces exist, any caller searching for a workspace by some criteria (location, remote project ID, path set, etc.) could miss a match that has no window. Each caller will need to decide: do I need a window handle, or do I just need the workspace? Callers that just need the workspace should switch to `all_workspaces()`. Callers that genuinely need the window handle should check headless workspaces as a fallback (e.g., promote one to windowed if found).

Callers:
- [ ] `crates/workspace/src/workspace.rs` — `restore_multiworkspace`: rewrite for single workspace
- [ ] `crates/workspace/src/persistence.rs` — `MultiWorkspaceState`: simplify struct, remove `active_workspace_id`
- [ ] `crates/workspace/src/multi_workspace.rs` — `serialize`: simplify to just persist sidebar state
- [ ] All callers of `workspaces_with_windows()`: revisit once headless workspaces land — audit whether each caller should use `all_workspaces()` instead

### Task F: Remove compatibility shims

Once all callers are migrated:
- [ ] Remove `MultiWorkspace::workspaces()` (the one-element-slice shim).
- [ ] Remove `MultiWorkspace::active_workspace_index()` (the always-0 shim).
- [ ] Remove `MultiWorkspace::activate_index()`.
- [ ] Remove `MultiWorkspace::activate_next_workspace()` / `activate_previous_workspace()`.
- [ ] Consider renaming `MultiWorkspace` → `WindowRoot`.

### Task G: Update tests

- [ ] Update tests in `recent_projects`, `visual_test_runner`, `zed/tests` to use the new model.
- [ ] Tests that create multiple workspaces in one window need to be rethought (they may test headless workspace scenarios instead).

## Sequencing

Task A (headless workspaces) is independent and can start immediately. Tasks B, C, D, E are independent of each other and can be parallelized. Task F is cleanup after B-E are done. Task G can be done incrementally alongside B-E.

The sidebar rewrite (Category 4) is covered by the Active Projects refactor plan and depends on Task A (headless workspaces in WorkspaceStore) and the `ActiveProjects` entity being in place.

## Open Question: Can Workspaces Move Between Windows?

Today, a workspace is bound to a specific window at construction time — `Workspace::new` registers itself with a specific `AnyWindowHandle` in `WorkspaceStore`. `MultiWorkspace::activate()` swaps which `Entity<Workspace>` a window displays, but it does **not** update `WorkspaceStore`'s window↔workspace mapping. That's a known gap.

For the active projects model to work (clicking a sidebar entry opens a project in the current window), we need workspaces to be movable between windows. This needs investigation:

- **WorkspaceStore mapping** — `activate()` must update the `(AnyWindowHandle, WeakEntity<Workspace>)` entry in `WorkspaceStore` when a workspace moves to a different window.
- **`cx.spawn_in(window, ...)`** — tasks spawned this way are tied to the original window. Do they survive if the workspace is displayed in a different window? Do they need to be re-spawned?
- **Focus handles** — are they window-scoped? Will a workspace's focus handles work after moving to a new window?
- **Pane layout** — pane splits, dimensions, zoom state — are these tied to the window that created them?
- **`on_release` callback** — the workspace registers an `on_release` that removes itself from `WorkspaceStore` using the original window handle. If the workspace moved, this cleans up the wrong entry.

This likely works *mostly* because `MultiWorkspace::activate()` has been used to swap workspaces in practice, but it may have subtle bugs. Needs a focused pass before headless→windowed promotion is reliable.

## Non-Goals

- **Sidebar rewrite** — covered by `ACTIVE_PROJECTS_REFACTOR_PLAN.md`.
- **ActiveProjects entity** — covered by `ACTIVE_PROJECTS_REFACTOR_PLAN.md`.
- **Thread database integration** — later phase.
- **Renaming MultiWorkspace → WindowRoot** — polish, after everything else lands.