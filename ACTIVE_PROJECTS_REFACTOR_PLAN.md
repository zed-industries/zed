# Active Projects Refactor Plan

## Goal

Extract `ActiveProjects` out of `WorkspaceStore` and into the `sidebar` crate as its own inert data structure. It owns a persisted `Vec<PathList>` — nothing more. No live workspaces, no entities, no language servers. The sidebar becomes the sole composition point that merges three independent data sources into the grouped UI.

## Motivation

The architecture doc originally called for `WorkspaceStore` to hold `active_projects: Vec<Entity<Workspace>>` — live, eagerly-rehydrated workspace entities. This creates two problems:

1. **Duplicated state management.** Both `WorkspaceStore` and the sidebar's internal `ActiveProjects` struct maintain lists of workspaces and have to stay in sync.
2. **Unnecessary coupling.** The active projects list is conceptually just "which folder sets does the user care about?" — it doesn't need live workspaces, language servers, or file watchers. Making it inert simplifies persistence (just serialize `PathList`s), simplifies startup (load from DB, done), and eliminates an entire category of lifecycle bugs.

The three data sources the sidebar composes become cleanly separated:

| Source | What it knows | Owner |
|--------|--------------|-------|
| **`ActiveProjects`** | Which folder sets the user cares about | `sidebar` crate (`active_projects.rs`) |
| **`WorkspaceStore`** | Which windows are open, which live workspace each one shows | `workspace` crate (existing) |
| **`ThreadStore`** | What threads exist, their metadata, which folder paths they ran against | `agent` crate (existing) |

## What Changes

### 1. New file: `crates/sidebar/src/active_projects.rs`

A GPUI entity with a simple API:

```rust
use workspace::PathList;

pub struct ActiveProjects {
    projects: Vec<PathList>,
}

impl ActiveProjects {
    // --- Queries ---
    pub fn projects(&self) -> &[PathList] { ... }
    pub fn contains(&self, path_list: &PathList) -> bool { ... }

    // --- Mutations (all call cx.notify()) ---
    pub fn add(&mut self, path_list: PathList, cx: &mut Context<Self>) { ... }
    pub fn remove(&mut self, path_list: &PathList, cx: &mut Context<Self>) { ... }
}
```

Key properties:
- **Inert.** No `Entity<Workspace>`, no live state. Just `Vec<PathList>`.
- **Persisted.** Reads from / writes to the KVP store (or its own DB table). Serialization is trivial since `PathList` already has `serialize()`/`deserialize()`.
- **Global entity.** Registered on the app, accessed via `ActiveProjects::global(cx)`. Same pattern as `ThreadStore`.
- **Append-mostly.** `add()` is called when a thread is created in a workspace. `remove()` is called when the user clicks "Remove Project." No automatic removal.
- **Idempotent adds.** If the `PathList` is already present, `add()` is a no-op.

### 2. Remove `active_projects` from `WorkspaceStore` ✅ Done

Already removed in the "Make WorkspaceStore a GPUI global" commit.

### 3. Update `crates/sidebar/src/sidebar.rs`

The sidebar's internal `ActiveProjects` struct (the grouping/derivation logic around `Vec<ProjectGroup>`) should be renamed to avoid confusion — something like `ProjectGroups` or `DerivedGroups`. It's the *derived view*, not the *source of truth*.

The sidebar's `update_entries` method changes from:

```
read workspaces from MultiWorkspace → build groups
```

to:

```
read PathLists from ActiveProjects (global entity)
read live workspaces from WorkspaceStore (global entity)
merge: active PathLists ∪ windowed workspaces' PathLists
for each PathList, find live workspaces (if any) and thread metadata
build ProjectGroups for rendering
```

The sidebar observes:
- `ActiveProjects` — project added/removed
- `WorkspaceStore` — window opened/closed, workspace changed
- `ThreadStore` — thread metadata changes
- Individual projects/threads — worktree changes, agent panel events (existing subscriptions)

### 4. Wire up the "persist on thread creation" trigger

When a thread is created in a workspace, the code that currently would call `WorkspaceStore::persist_project()` instead does:

```rust
let path_list = PathList::new(&workspace.read(cx).root_paths(cx));
ActiveProjects::global(cx).update(cx, |ap, cx| ap.add(path_list, cx));
```

This likely lives in the agent panel or thread creation codepath. Since nobody was calling `persist_project` yet (it was never wired up), this is greenfield.

### 5. Add `db` dependency to sidebar crate

Add `db.workspace = true` to `crates/sidebar/Cargo.toml` so `ActiveProjects` can use `KEY_VALUE_STORE` for persistence.

Persistence approach: store the entire list as a single JSON blob under a KVP key like `"active_projects"`. Each entry is a serialized `PathList`. This is simple and the list will be small (single digits to low tens of entries).

```rust
const ACTIVE_PROJECTS_KEY: &str = "active_projects";

// Save
let serialized: Vec<SerializedPathList> = self.projects.iter().map(|p| p.serialize()).collect();
let json = serde_json::to_string(&serialized)?;
KEY_VALUE_STORE.write_kvp(ACTIVE_PROJECTS_KEY.into(), json).await?;

// Load
let json = KEY_VALUE_STORE.read_kvp(ACTIVE_PROJECTS_KEY)?.unwrap_or_default();
let serialized: Vec<SerializedPathList> = serde_json::from_str(&json)?;
let projects = serialized.iter().map(|s| PathList::deserialize(s)).collect();
```

Note: `SerializedPathList` will need `Serialize`/`Deserialize` derives added (it currently only has `Debug`). Alternatively, we can use our own serialization struct for the JSON representation.

## File Inventory

| File | Action |
|------|--------|
| `crates/sidebar/src/active_projects.rs` | **Create** — the new `ActiveProjects` entity |
| `crates/sidebar/src/sidebar.rs` | **Edit** — rename internal `ActiveProjects` → `ProjectGroups`, update `update_entries` to read from the new global entity, add observation |
| `crates/sidebar/Cargo.toml` | **Edit** — add `db` dependency, add `serde`/`serde_json` if not already present |
| `ACTIVE_PROJECTS_ARCHITECTURE.md` | **Edit** — update to reflect the inert model |
| `ACTIVE_PROJECTS_PLAN.md` | **Edit** — update Phase 1 and Phase 4 descriptions |
| `MULTIPROJECT_SIDEBAR_NOTES.md` | **Edit** — update architecture section |

## Sequencing

This work can be split into two parallel tasks:

### Task A: Create `ActiveProjects` entity (greenfield, no conflicts)

1. Create `crates/sidebar/src/active_projects.rs` with the struct, GPUI entity impl, add/remove/contains/projects API.
2. Add persistence (load on init, save on mutation).
3. Add `db` dependency to sidebar's `Cargo.toml`.
4. Register as a global entity (add an `init()` function or similar pattern).
5. Write unit tests: add, remove, contains, idempotent add, persistence round-trip.

### Task B: Remove from `WorkspaceStore` and rewire sidebar (depends on Task A)

1. ✅ Already done: removed `active_projects`, `persist_project()`, `remove_project()`, `windows_for_path_list()` from `WorkspaceStore`.
2. Rename sidebar's internal `ActiveProjects` → `ProjectGroups` (or similar).
3. Update `Sidebar::update_entries` to read from the new `ActiveProjects` global entity + `WorkspaceStore` for live windows.
4. Add observation of `ActiveProjects` entity in sidebar.
5. Update any doc files.

## Non-Goals (for this refactor)

- **Thread database integration** — enriching sidebar entries with thread history. That's Phase 5 work.
- **Persistence of the "persist on thread creation" trigger** — wiring the actual callsite. The entity API will be ready, but the trigger can be wired separately.
- **Git worktree canonicalization** — `PathList` grouping by git repo identity. Phase 6.
- **Lazy loading / `WorkspaceEntry` enum** — the architecture doc's optimization addendum. Not needed when the list is inert.
- **Renaming `MultiWorkspace` → `WindowRoot`** — Phase 8 polish.