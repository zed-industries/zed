# MultiWorkspace Architecture Refactor

## Overview

We are refactoring Zed to support multiple Workspaces that share worktrees. The key change is moving from "Project owns WorktreeStore" to "WorktreeStore is shared, Project is a view over it."

### Current Architecture
```
Workspace
└── Entity<Project>
    └── Entity<WorktreeStore>  // Project creates and owns this
        └── Entity<Worktree>...
    └── Entity<BufferStore>
    └── Entity<GitStore>
    └── Entity<LspStore>
    └── ... other stores
```

### New Architecture
```
MultiWorkspace (new top-level container)
├── Entity<WorktreeStore>     // Shared, contains mix of local AND remote worktrees
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
│
└── Workspace B
    └── Entity<Project>
        ├── worktree_ids: HashSet<WorktreeId>     // Can overlap with Workspace A
        ├── client_state: ProjectClientState
        ├── Entity<DapStore>
        └── Entity<TaskStore>
```

## Key Design Decisions

### 1. WorktreeStore Contains Mixed Local/Remote Worktrees

The shared WorktreeStore can contain both local and remote worktrees simultaneously:
- `LocalWorktree` for `~/projects/foo` (local filesystem)
- `RemoteWorktree` for `server-a:/home/user/bar` (SSH connection)
- `RemoteWorktree` for `server-b:/work/baz` (different SSH connection)

This works because **transport lives at the Worktree level**, not WorktreeStore level:
- `LocalWorktree` has `fs: Arc<dyn Fs>`
- `RemoteWorktree` has `client: AnyProtoClient` and `project_id: u64`

When BufferStore opens a file, it delegates to the Worktree via `worktree.load_file()`, so the Worktree handles its own transport.

**Required change**: Remove `WorktreeStoreState` enum (currently Local vs Remote). Instead, provide explicit methods for adding local vs remote worktrees:
```rust
impl WorktreeStore {
    pub fn add_local_worktree(&mut self, path: &Path, fs: Arc<dyn Fs>, cx: ...) -> Task<...>
    pub fn add_remote_worktree(&mut self, client: AnyProtoClient, project_id: u64, ...) -> Task<...>
}
```

### 2. Project Becomes a "View" Over Shared Worktrees

Project tracks which worktrees it contains via `worktree_ids: HashSet<WorktreeId>`:

```rust
struct Project {
    worktree_ids: HashSet<WorktreeId>,
    worktree_store: Entity<WorktreeStore>,  // Reference to shared store
    // ...
}

impl Project {
    pub fn worktrees(&self, cx: &App) -> impl Iterator<Item = Entity<Worktree>> {
        self.worktree_store.read(cx)
            .worktrees()
            .filter(|wt| self.worktree_ids.contains(&wt.read(cx).id()))
    }
}
```

Worktrees are **oblivious to Projects** - they don't track which Projects reference them. This keeps the relationship clean and unidirectional.

### 3. Shared Stores vs Per-Workspace Stores

**Shared at MultiWorkspace level** (same file = same entity everywhere):
- `BufferStore` - Edits in Workspace A are immediately visible in Workspace B
- `GitStore` - Git state is per-repository
- `LspStore` - One language server instance per worktree, shared across workspaces
- `ImageStore` - Same as BufferStore
- `BreakpointStore` - Breakpoints are on files

**Per-Workspace** (workspace-specific context):
- `DapStore` - Debugging sessions are focused activities
- `TaskStore` - Tasks run in a specific workspace context
- `ContextServerStore` - Context servers may be workspace-scoped

### 4. Collaboration Model (Option C: Share at Project Level)

Collaboration happens at the **Project level**, not WorktreeStore level:
- A Project can be shared (gets a `remote_id` in `ProjectClientState::Shared`)
- Collaborators join a specific Project and see its worktrees
- The same worktree can be in multiple Projects, but...

**Constraint**: A worktree can only be in ONE shared Project at a time.


When sharing, validate:
```rust
impl Project {
    pub fn share(&mut self, cx: &mut Context<Self>) -> Task<Result<u64>> {
        // Check if any of our worktrees are already in a shared project
        let multi_workspace = MultiWorkspace::global(cx);
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

### 5. RemoteWorktree.project_id Clarification

There are two different IDs to understand:
1. **`RemoteWorktree.project_id`** - The upstream source ID (e.g., SSH server's project ID). This is for RPC with the server hosting the files.
2. **`Project.client_state.remote_id`** - The collaboration ID when this Project is shared with others.

These are orthogonal. A RemoteWorktree connected via SSH has a fixed `project_id` for that connection. That worktree can be part of multiple local Projects with different collaboration states.

### 6. Worktree Lifecycle

Worktrees are reference-counted in the shared WorktreeStore:
- When a Workspace closes, its Project removes its worktree references
- The worktree stays alive if another Project still references it
- When the last reference is removed, the worktree is dropped

## Implementation Plan

### Phase 1: Create MultiWorkspace Container

1. Create new `MultiWorkspace` struct that holds:
   - `Entity<WorktreeStore>`
   - `Entity<BufferStore>`
   - `Entity<GitStore>`
   - `Entity<LspStore>`
   - `Entity<ImageStore>`
   - `Entity<BreakpointStore>`
   - `Vec<Entity<Workspace>>` or similar

2. Make MultiWorkspace global or per-window (TBD based on window model)

3. Update Workspace creation to:
   - Get/create MultiWorkspace
   - Create Project with reference to shared WorktreeStore
   - Pass shared stores to Project

### Phase 2: Refactor WorktreeStore

1. Remove `WorktreeStoreState` enum
2. Add explicit `add_local_worktree` and `add_remote_worktree` methods
3. Update `find_or_create_worktree` to take connection info parameter

### Phase 3: Refactor Project

1. Add `worktree_ids: HashSet<WorktreeId>` field
2. Change `worktree_store` from owned to shared reference
3. Update all methods that iterate worktrees to filter by `worktree_ids`
4. Move DapStore and TaskStore creation into Project (keep them per-workspace)
5. Add references to shared stores (BufferStore, GitStore, etc.)

### Phase 4: Refactor Store Creation

1. Move BufferStore, GitStore, LspStore, ImageStore, BreakpointStore creation to MultiWorkspace
2. Update these stores to work with the shared WorktreeStore
3. Ensure stores handle worktrees being added/removed dynamically

### Phase 5: Update Collaboration

1. Add sharing validation (prevent double-sharing of worktrees)
2. Update `share_project` to work with the new architecture
3. Test collaboration scenarios with multiple workspaces

### Phase 6: Update Consumers

Many places call `workspace.project()` and expect to access stores. Audit and update:
- Panels (AgentPanel, GitPanel, etc.)
- Activity indicator
- File opening code paths
- Search functionality

## Key Files to Modify

- `crates/project/src/project.rs` - Major refactor
- `crates/project/src/worktree_store.rs` - Remove state enum, add explicit methods
- `crates/project/src/buffer_store.rs` - May need state simplification
- `crates/workspace/src/workspace.rs` - Update to use MultiWorkspace
- `crates/call/src/call_impl/room.rs` - Update sharing logic
- New file: `crates/workspace/src/multi_workspace.rs` or similar

## Testing Considerations

1. **Multi-workspace scenarios**: Create tests where two workspaces share worktrees
2. **Edit propagation**: Verify edits in one workspace appear in another
3. **Collaboration with shared worktrees**: Test sharing when worktrees overlap
4. **Mixed local/remote**: Test WorktreeStore with both local and SSH worktrees
5. **Worktree lifecycle**: Test closing workspaces and worktree cleanup

## Open Questions for Implementation

1. Should MultiWorkspace be global (one per app) or per-window?
2. How should we handle the transition for existing serialized workspaces?
3. Should there be UI for managing worktrees across workspaces?

## References

- Original detailed plan: `threads-sidebar-plan.md` (some concepts superseded)
- Comment in `crates/project/src/project.rs` lines 162-172 showing target architecture
- Current Project::local() and Project::remote() constructors show store creation patterns
