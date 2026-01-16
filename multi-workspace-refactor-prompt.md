# MultiWorkspace Architecture Refactor

## Overview

We are refactoring Zed to support multiple Workspaces that share worktrees. The key change is moving from "Project owns WorktreeStore" to "WorktreeStore is shared, Project is a view over it."

### Current Architecture
```
Workspace (being renamed to MultiWorkspace)
└── Entity<Project>
    └── Entity<WorktreeStore>  // Project creates and owns this
        └── Entity<Worktree>...
    └── Entity<BufferStore>
    └── Entity<GitStore>
    └── Entity<LspStore>
    └── ... other stores
```

### Target Architecture
```
MultiWorkspace (window-level container)
├── Entity<WorktreeStore>     // Shared, eventually contains mix of local AND remote worktrees
├── Entity<BufferStore>       // Shared - same file = same buffer
├── Entity<GitStore>          // Shared - git state is per-repo
├── Entity<LspStore>          // Shared - one LSP instance per worktree
├── Entity<ImageStore>        // Shared
├── Entity<BreakpointStore>   // Shared - breakpoints are on files
│
├── Workspace A
│   └── Entity<Project>
│       ├── worktree_ids: HashSet<WorktreeId>     // Which worktrees this Project "views"
│       ├── client_state: ProjectClientState      // Collaboration state (Local/Shared/Remote)
│       ├── Entity<DapStore>                      // Per-workspace (debugging sessions)
│       ├── Entity<TaskStore>                     // Per-workspace (task execution context)
│       └── (references to shared stores)
│   └── Panes, Panels, Docks (UI)
│
└── Workspace B
    └── Entity<Project>
        ├── worktree_ids: HashSet<WorktreeId>     // Can overlap with Workspace A
        ├── client_state: ProjectClientState
        ├── Entity<DapStore>
        └── Entity<TaskStore>
    └── Panes, Panels, Docks (UI)
```

## Implementation Strategy

The key insight is: **defer store complexity until we actually have two workspaces trying to share them**. This gives us concrete test cases and clearer requirements.

### Phase A: Structural Rename (Ship First)

**Goal**: Extract `Workspace` from `MultiWorkspace` as a mechanical refactor with no behavior change.

**Current State**:
- `Workspace` has been renamed to `MultiWorkspace` ✅
- Stub `single_workspace::Workspace` module created ✅

**What Moves to `Workspace`** (the new inner struct):
- `Entity<Project>` - workspace owns its project
- `center: PaneGroup` - pane layout
- `panes: Vec<Entity<Pane>>` - all panes
- `active_pane: Entity<Pane>`
- `left_dock`, `right_dock`, `bottom_dock` - dock UI
- `status_bar` - per-workspace status
- `modal_layer`, `toast_layer` - per-workspace overlays
- `notifications` - per-workspace notifications
- Most UI rendering logic

**What Stays in `MultiWorkspace`**:
- `Entity<WorktreeStore>` - shared (passed down to Projects)
- `Entity<Workspace>` - just one for now
- `weak_self`, `app_state` - window-level concerns
- `workspace_actions` - action registration
- `database_id`, serialization concerns
- `WorkspaceStore` registration
- Window title management

**This is "extract class" refactoring** - moving fields/methods down without changing behavior. Risk is low because it's mechanical.

**Merge Point**: Ship this rename, get it tested in the wild.

### Phase B: Enable Second Workspace (Real Refactor)

**Goal**: Actually support multiple Workspace instances, refactoring stores as needed.

**Why This Order?**: You can't write integration tests for shared state without two workspaces. Once you have:

```rust
struct MultiWorkspace {
    worktree_store: Entity<WorktreeStore>,
    workspaces: Vec<Entity<Workspace>>,
}
```

Then you can write tests like:

```rust
#[gpui::test]
async fn test_two_workspaces_share_buffer(cx: &mut TestAppContext) {
    let multi = cx.new(|cx| MultiWorkspace::new(...));
    
    // Add a worktree to the shared store
    let worktree_id = multi.update(cx, |m, cx| m.add_worktree("/path", cx)).await;
    
    // Create two workspaces viewing the same worktree
    let ws_a = multi.update(cx, |m, cx| m.add_workspace(cx));
    let ws_b = multi.update(cx, |m, cx| m.add_workspace(cx));
    
    // Open same file in both, verify it's the same buffer
    // Edit in one, verify it appears in the other
}
```

**Store Refactors** (driven by test failures):
1. `WorktreeStore` - Remove `WorktreeStoreState` enum, push local/remote distinction into individual worktrees
2. `BufferStore` - Lift to MultiWorkspace so same file = same buffer
3. `GitStore` - Lift to MultiWorkspace (git state is per-repo)
4. `LspStore` - Lift to MultiWorkspace (one LSP per worktree)
5. Keep `DapStore`, `TaskStore` per-workspace

**Project Changes**:
- Add `worktree_ids: HashSet<WorktreeId>` to filter which worktrees it "views"
- `Project::worktrees()` filters by this set
- Project receives shared store references from MultiWorkspace

## Key Design Decisions

### 1. WorktreeStore Will Contain Mixed Local/Remote Worktrees

Eventually the shared WorktreeStore can contain both local and remote worktrees:
- `LocalWorktree` for `~/projects/foo` (local filesystem)
- `RemoteWorktree` for `server-a:/home/user/bar` (SSH connection)

This works because **transport lives at the Worktree level**:
- `LocalWorktree` has `fs: Arc<dyn Fs>`
- `RemoteWorktree` has `client: AnyProtoClient` and `project_id: u64`

**Required change** (Phase B): Remove `WorktreeStoreState` enum. Add explicit methods:
```rust
impl WorktreeStore {
    pub fn add_local_worktree(&mut self, path: &Path, fs: Arc<dyn Fs>, ...) -> Task<...>
    pub fn add_remote_worktree(&mut self, client: AnyProtoClient, project_id: u64, ...) -> Task<...>
}
```

### 2. Project Becomes a "View" Over Shared Worktrees

```rust
struct Project {
    worktree_ids: HashSet<WorktreeId>,
    worktree_store: Entity<WorktreeStore>,  // Reference to shared store
}

impl Project {
    pub fn worktrees(&self, cx: &App) -> impl Iterator<Item = Entity<Worktree>> {
        self.worktree_store.read(cx)
            .worktrees()
            .filter(|wt| self.worktree_ids.contains(&wt.read(cx).id()))
    }
}
```

Worktrees are **oblivious to Projects** - they don't track which Projects reference them.

### 3. Shared Stores vs Per-Workspace Stores

**Eventually Shared** (same file = same entity everywhere):
- `BufferStore` - Edits in Workspace A visible in Workspace B
- `GitStore` - Git state is per-repository
- `LspStore` - One language server instance per worktree
- `ImageStore` - Same as BufferStore
- `BreakpointStore` - Breakpoints are on files

**Per-Workspace** (workspace-specific context):
- `DapStore` - Debugging sessions are focused activities
- `TaskStore` - Tasks run in a specific workspace context

### 4. Collaboration Constraint

A worktree can only be in ONE shared Project at a time. Validate when sharing:
```rust
impl Project {
    pub fn share(&mut self, cx: &mut Context<Self>) -> Task<Result<u64>> {
        // Check if any of our worktrees are already in a shared project
        for worktree_id in &self.worktree_ids {
            if let Some(other_project) = multi_workspace.find_sharing_project_for_worktree(*worktree_id, cx) {
                if other_project != cx.entity() {
                    return Task::ready(Err(anyhow!(
                        "Worktree {} is already shared by another project",
                        worktree_id
                    )));
                }
            }
        }
        // Proceed with sharing...
    }
}
```

## Key Files

- `crates/workspace/src/workspace.rs` - Contains `MultiWorkspace`, will contain `Workspace`
- `crates/workspace/src/single_workspace.rs` - Stub module (created)
- `crates/project/src/project.rs` - Will add `worktree_ids` filter
- `crates/project/src/worktree_store.rs` - Will remove `WorktreeStoreState` enum

## Current Status

- [x] Rename `Workspace` → `MultiWorkspace` (done)
- [x] Create stub `single_workspace::Workspace` module (done)
- [ ] Extract UI/pane/panel ownership from `MultiWorkspace` into `Workspace`
- [ ] Pass `WorktreeStore` from `MultiWorkspace` down to `Project`
- [ ] **MERGE POINT**
- [ ] Add second `Workspace` support
- [ ] Refactor stores as needed for sharing