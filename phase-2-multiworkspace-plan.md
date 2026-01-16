# Phase 2: MultiWorkspace Implementation Plan

This document details Phase 2 of the MultiWorkspace Agent V2 Refactor. Phase 1 (GlobalThreadStore Infrastructure) is a prerequisite.

## Overview

Enable the workspace to swap between different ThreadWorktrees contexts while keeping state in memory. **Key insight:** Project stays singular per window; we swap which ThreadWorktrees group is active.

### Goals

1. Create `ThreadWorktrees` as a scoped lens into `Project` for panels
2. Implement `WorkspaceState` to store per-thread UI state (panes, panels)
3. Enable switching between thread contexts without losing state
4. Migrate existing panels from `Entity<Project>` to `Entity<ThreadWorktrees>`

### Dependencies

- **Phase 1 Complete**: `GlobalThreadStore` with project paths stored per thread
- **Feature Flag**: All changes gated behind `agent-v2` feature flag

---

## 2.1 New Concept: ThreadWorktrees

### ThreadWorktreesId

The identity that ties a thread to workspace-level state:

```rust
// New file: crates/project/src/thread_worktrees.rs

use gpui::{App, Context, Entity, EventEmitter, Task, WeakEntity};
use collections::HashSet;
use std::path::PathBuf;
use anyhow::{Result, anyhow};

/// Identifies a ThreadWorktrees group - glues thread identity to workspace identity
#[derive(Clone, Hash, Eq, PartialEq, Debug)]
pub struct ThreadWorktreesId {
    /// None for "default" (no thread active)
    pub session_id: Option<acp::SessionId>,
    pub workspace_db_id: WorkspaceId,
}

impl ThreadWorktreesId {
    pub fn default_for_workspace(workspace_db_id: WorkspaceId) -> Self {
        Self {
            session_id: None,
            workspace_db_id,
        }
    }

    pub fn for_thread(session_id: acp::SessionId, workspace_db_id: WorkspaceId) -> Self {
        Self {
            session_id: Some(session_id),
            workspace_db_id,
        }
    }
}
```

### ThreadWorktrees Struct

```rust
/// Groups worktrees that belong to a specific thread context.
/// Acts as a filtered lens into Project for panels.
pub struct ThreadWorktrees {
    id: ThreadWorktreesId,
    project: Entity<Project>,
    worktree_ids: HashSet<WorktreeId>,
    folder_paths: Vec<PathBuf>,  // Absolute paths for persistence
    _subscriptions: Vec<gpui::Subscription>,
}
```

### Project Additions

Modify `crates/project/src/project.rs`:

```rust
pub struct Project {
    // EXISTING: all worktrees for all live threads in this window
    worktree_store: Entity<WorktreeStore>,

    // NEW: Groups worktrees by thread
    thread_worktrees: HashMap<ThreadWorktreesId, Entity<ThreadWorktrees>>,

    // NEW: Currently active thread context
    active_thread_worktrees_id: Option<ThreadWorktreesId>,

    // ... existing fields ...
}

impl Project {
    /// Get or create ThreadWorktrees for a given ID
    pub fn thread_worktrees(
        &mut self,
        id: &ThreadWorktreesId,
        cx: &mut Context<Self>,
    ) -> Entity<ThreadWorktrees> {
        if let Some(tw) = self.thread_worktrees.get(id) {
            return tw.clone();
        }
        
        // Create new ThreadWorktrees with worktrees based on thread's folder_paths
        let worktree_ids = self.worktree_ids_for_thread(id, cx);
        let thread_worktrees = cx.new(|cx| {
            ThreadWorktrees::new(id.clone(), cx.entity().clone(), worktree_ids, cx)
        });
        self.thread_worktrees.insert(id.clone(), thread_worktrees.clone());
        thread_worktrees
    }

    /// Set the active thread context
    pub fn set_active_thread_worktrees(
        &mut self,
        id: Option<ThreadWorktreesId>,
        cx: &mut Context<Self>,
    ) {
        self.active_thread_worktrees_id = id;
        cx.notify();
    }

    /// Get the currently active ThreadWorktrees
    pub fn active_thread_worktrees(&self, cx: &App) -> Option<Entity<ThreadWorktrees>> {
        self.active_thread_worktrees_id
            .as_ref()
            .and_then(|id| self.thread_worktrees.get(id).cloned())
    }
}
```

### Worktree Lifecycle

- Worktrees are automatically created/loaded when opening a thread
- Live threads (running in background) keep their worktrees loaded
- Worktrees shared between threads (e.g., /cloud in threads A and B) exist once in Project but are referenced by multiple ThreadWorktrees

---

## 2.2 ThreadWorktrees as Panel API Surface

Panels currently hold `Entity<Project>` and call methods like `project.read(cx).git_store()`. In multi-workspace, panels need **scoped access** - GitPanel for ThreadWorktrees A shouldn't see repos from ThreadWorktrees B.

### Current Panel Pattern (to migrate from)

Based on existing code analysis:

```rust
// crates/git_ui/src/git_panel.rs - Current
pub struct GitPanel {
    pub(crate) project: Entity<Project>,
    pub(crate) active_repository: Option<Entity<Repository>>,
    // ...
}

// crates/project_panel/src/project_panel.rs - Current
pub struct ProjectPanel {
    project: Entity<Project>,
    fs: Arc<dyn Fs>,
    // ...
}
```

### ThreadWorktrees API Implementation

```rust
// crates/project/src/thread_worktrees.rs

impl ThreadWorktrees {
    pub fn new(
        id: ThreadWorktreesId,
        project: Entity<Project>,
        worktree_ids: HashSet<WorktreeId>,
        cx: &mut Context<Self>,
    ) -> Self {
        let git_store = project.read(cx).git_store().clone();
        let worktree_store = project.read(cx).worktree_store.clone();
        let mut subscriptions = Vec::new();

        // Subscribe to git_store and filter events
        subscriptions.push(cx.subscribe(&git_store, {
            let worktree_ids = worktree_ids.clone();
            move |this, _store, event, cx| {
                this.handle_git_store_event(event, &worktree_ids, cx);
            }
        }));

        // Subscribe to worktree_store for worktree changes
        subscriptions.push(cx.subscribe(&worktree_store, {
            let worktree_ids = worktree_ids.clone();
            move |this, _store, event, cx| {
                this.handle_worktree_store_event(event, &worktree_ids, cx);
            }
        }));

        Self {
            id,
            project,
            worktree_ids,
            folder_paths: Vec::new(),
            _subscriptions: subscriptions,
        }
    }

    // === Identity ===
    
    pub fn id(&self) -> &ThreadWorktreesId {
        &self.id
    }

    pub fn project(&self) -> &Entity<Project> {
        &self.project
    }

    // === Scoped Worktree Access ===

    pub fn visible_worktrees<'a>(&self, cx: &'a App) -> impl Iterator<Item = Entity<Worktree>> + 'a {
        let owned_ids = self.worktree_ids.clone();
        self.project
            .read(cx)
            .visible_worktrees(cx)
            .filter(move |wt| owned_ids.contains(&wt.read(cx).id()))
    }

    pub fn worktree_for_id(&self, id: WorktreeId, cx: &App) -> Option<Entity<Worktree>> {
        if self.worktree_ids.contains(&id) {
            self.project.read(cx).worktree_for_id(id, cx)
        } else {
            None
        }
    }

    pub fn worktree_count(&self) -> usize {
        self.worktree_ids.len()
    }

    // === Scoped Git Access ===

    pub fn active_repository(&self, cx: &App) -> Option<Entity<Repository>> {
        let repo = self.project.read(cx).active_repository(cx)?;
        let worktree_id = repo.read(cx).worktree_id();
        self.worktree_ids.contains(&worktree_id).then_some(repo)
    }

    /// Returns the shared git_store - events are filtered via subscriptions
    pub fn git_store(&self, cx: &App) -> Entity<GitStore> {
        self.project.read(cx).git_store().clone()
    }

    fn repo_in_scope(&self, repo_id: RepositoryId, cx: &App) -> bool {
        let git_store = self.project.read(cx).git_store();
        git_store
            .read(cx)
            .repository(repo_id)
            .map(|repo| self.worktree_ids.contains(&repo.read(cx).worktree_id()))
            .unwrap_or(false)
    }

    // === Pass-through for Global State ===

    pub fn is_via_collab(&self, cx: &App) -> bool {
        self.project.read(cx).is_via_collab()
    }

    pub fn is_read_only(&self, cx: &App) -> bool {
        self.project.read(cx).is_read_only(cx)
    }

    pub fn languages(&self, cx: &App) -> Arc<LanguageRegistry> {
        self.project.read(cx).languages().clone()
    }

    pub fn path_style(&self, cx: &App) -> PathStyle {
        self.project.read(cx).path_style(cx)
    }

    pub fn fs(&self, cx: &App) -> Arc<dyn Fs> {
        self.project.read(cx).fs.clone()
    }

    // === Scoped Buffer Operations ===

    /// Shared buffer_store - buffers are deduplicated across threads
    pub fn buffer_store(&self, cx: &App) -> Entity<BufferStore> {
        self.project.read(cx).buffer_store().clone()
    }

    /// Open buffer with scope validation (workflow guard, not isolation)
    pub fn open_buffer(
        &self,
        path: ProjectPath,
        cx: &mut Context<Self>,
    ) -> Task<Result<Entity<Buffer>>> {
        if !self.worktree_ids.contains(&path.worktree_id) {
            return Task::ready(Err(anyhow!("Path not in this ThreadWorktrees scope")));
        }
        self.project
            .update(cx, |project, cx| project.open_buffer(path, cx))
    }

    // === Scoped File Operations ===

    pub fn delete_file(
        &self,
        path: ProjectPath,
        trash: bool,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<()>>> {
        if !self.worktree_ids.contains(&path.worktree_id) {
            return None;
        }
        self.project
            .update(cx, |project, cx| project.delete_file(path, trash, cx))
    }

    pub fn create_file(
        &self,
        path: ProjectPath,
        is_dir: bool,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<CreatedEntry>>> {
        if !self.worktree_ids.contains(&path.worktree_id) {
            return None;
        }
        self.project
            .update(cx, |project, cx| project.create_file(path, is_dir, cx))
    }

    pub fn rename_entry(
        &self,
        entry_id: ProjectEntryId,
        new_path: impl Into<Arc<Path>>,
        cx: &mut Context<Self>,
    ) -> Option<Task<Result<CreatedEntry>>> {
        // Verify entry belongs to our worktrees
        let worktree_id = self.project.read(cx).entry_worktree_id(entry_id)?;
        if !self.worktree_ids.contains(&worktree_id) {
            return None;
        }
        self.project.update(cx, |project, cx| {
            project.rename_entry(entry_id, new_path, cx)
        })
    }
}
```

### ThreadWorktrees Events

```rust
#[derive(Clone, Debug)]
pub enum ThreadWorktreesEvent {
    /// Active repository changed (filtered to this scope)
    ActiveRepositoryChanged(Option<Entity<Repository>>),
    /// A repository in this scope was updated
    RepositoryUpdated(RepositoryId, RepositoryEvent),
    /// A worktree in this scope was added
    WorktreeAdded(WorktreeId),
    /// A worktree in this scope was removed
    WorktreeRemoved(WorktreeId),
    /// Entries updated in a worktree in this scope
    WorktreeEntriesUpdated(WorktreeId, UpdatedEntriesSet),
}

impl EventEmitter<ThreadWorktreesEvent> for ThreadWorktrees {}

impl ThreadWorktrees {
    fn handle_git_store_event(
        &mut self,
        event: &GitStoreEvent,
        worktree_ids: &HashSet<WorktreeId>,
        cx: &mut Context<Self>,
    ) {
        match event {
            GitStoreEvent::ActiveRepositoryChanged(repo_id) => {
                // Only emit if repo is in our worktrees
                let filtered_repo = repo_id.as_ref().and_then(|id| {
                    let git_store = self.project.read(cx).git_store();
                    let repo = git_store.read(cx).repository(*id)?;
                    worktree_ids
                        .contains(&repo.read(cx).worktree_id())
                        .then_some(repo)
                });
                cx.emit(ThreadWorktreesEvent::ActiveRepositoryChanged(filtered_repo));
            }
            GitStoreEvent::RepositoryUpdated(repo_id, event, _) => {
                if self.repo_in_scope(*repo_id, cx) {
                    cx.emit(ThreadWorktreesEvent::RepositoryUpdated(*repo_id, event.clone()));
                }
            }
            _ => {}
        }
    }

    fn handle_worktree_store_event(
        &mut self,
        event: &WorktreeStoreEvent,
        worktree_ids: &HashSet<WorktreeId>,
        cx: &mut Context<Self>,
    ) {
        match event {
            WorktreeStoreEvent::WorktreeAdded(wt) => {
                let id = wt.read(cx).id();
                if worktree_ids.contains(&id) {
                    cx.emit(ThreadWorktreesEvent::WorktreeAdded(id));
                }
            }
            WorktreeStoreEvent::WorktreeRemoved(_, id) => {
                if worktree_ids.contains(id) {
                    cx.emit(ThreadWorktreesEvent::WorktreeRemoved(*id));
                }
            }
            WorktreeStoreEvent::WorktreeEntriesUpdated { worktree_id, entries } => {
                if worktree_ids.contains(worktree_id) {
                    cx.emit(ThreadWorktreesEvent::WorktreeEntriesUpdated(
                        *worktree_id,
                        entries.clone(),
                    ));
                }
            }
            _ => {}
        }
    }
}
```

### API Categories Reference

| Category | Examples | ThreadWorktrees Behavior |
|----------|----------|-------------------------|
| Worktree-scoped | `visible_worktrees()`, `worktree_for_id()` | Filtered to owned worktrees |
| Git-scoped | `active_repository()`, `git_init()` | Filtered by worktree ownership |
| File operations | `open_buffer()`, `delete_file()` | Validated against scope |
| Global/shared | `is_via_collab()`, `languages()`, `path_style()`, `fs()` | Pass-through to Project |
| Buffer store | `buffer_store()`, `open_buffer()` | Shared (deduped by ProjectPath) |

---

## 2.3 WorkspaceState per ThreadWorktrees

Modify `crates/workspace/src/workspace.rs`:

### WorkspaceState Struct

```rust
/// State for a specific ThreadWorktrees context.
/// Each thread gets its own set of panes and panel instances.
pub struct WorkspaceState {
    thread_worktrees_id: ThreadWorktreesId,
    thread_worktrees: Entity<ThreadWorktrees>,
    
    // Pane state (swapped when switching threads)
    center: PaneGroup,
    panes: Vec<Entity<Pane>>,
    panes_by_item: HashMap<EntityId, WeakEntity<Pane>>,
    active_pane: Entity<Pane>,
    last_active_center_pane: Option<WeakEntity<Pane>>,
    
    // Panel instances - each ThreadWorktrees gets its own
    panel_instances: HashMap<TypeId, Box<dyn PanelHandle>>,
    
    // Serialization
    last_serialized: Option<Instant>,
}

impl WorkspaceState {
    pub fn new(
        thread_worktrees_id: ThreadWorktreesId,
        thread_worktrees: Entity<ThreadWorktrees>,
        window: &mut Window,
        cx: &mut Context<Workspace>,
    ) -> Self {
        let pane = cx.new(|cx| {
            Pane::new(
                cx.entity().downgrade(),
                /* project */ thread_worktrees.read(cx).project().clone(),
                Default::default(),
                None,
                NewFile.boxed_clone(),
                window,
                cx,
            )
        });
        
        Self {
            thread_worktrees_id,
            thread_worktrees,
            center: PaneGroup::new(pane.clone()),
            panes: vec![pane.clone()],
            panes_by_item: HashMap::default(),
            active_pane: pane,
            last_active_center_pane: None,
            panel_instances: HashMap::default(),
            last_serialized: None,
        }
    }

    pub fn thread_worktrees(&self) -> &Entity<ThreadWorktrees> {
        &self.thread_worktrees
    }
}
```

### Modified Workspace Struct

```rust
pub struct Workspace {
    project: Entity<Project>,  // Still singular per window

    // NEW: State per ThreadWorktrees
    workspace_states: HashMap<ThreadWorktreesId, WorkspaceState>,
    active_workspace_state_id: Option<ThreadWorktreesId>,

    // Docks stay constant - panel contents come from active WorkspaceState
    left_dock: Entity<Dock>,
    bottom_dock: Entity<Dock>,
    right_dock: Entity<Dock>,

    // These stay constant across thread switches
    status_bar: Entity<StatusBar>,
    modal_layer: Entity<ModalLayer>,
    toast_layer: Entity<ToastLayer>,
    titlebar_item: Option<AnyView>,
    notifications: Notifications,

    // Global panels (not per-thread)
    global_panel_instances: HashMap<TypeId, Box<dyn PanelHandle>>,

    // ... existing fields (weak_self, app_state, etc.) ...
}
```

---

## 2.4 Switching Active ThreadWorktrees

### Implementation

```rust
impl Workspace {
    /// Switch to a different thread context
    pub fn switch_to_thread(
        &mut self,
        thread_worktrees_id: ThreadWorktreesId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // 1. Save current state (implicit - already in workspace_states)
        if let Some(current_id) = &self.active_workspace_state_id {
            if *current_id == thread_worktrees_id {
                return; // Already active
            }
        }

        // 2. Ensure ThreadWorktrees exists in Project
        let thread_worktrees = self.project.update(cx, |project, cx| {
            project.thread_worktrees(&thread_worktrees_id, cx)
        });

        // 3. Update Project's active context
        self.project.update(cx, |project, cx| {
            project.set_active_thread_worktrees(Some(thread_worktrees_id.clone()), cx);
        });

        // 4. Load or create WorkspaceState
        if !self.workspace_states.contains_key(&thread_worktrees_id) {
            let state = WorkspaceState::new(
                thread_worktrees_id.clone(),
                thread_worktrees.clone(),
                window,
                cx,
            );
            self.workspace_states.insert(thread_worktrees_id.clone(), state);
            
            // Initialize scoped panels for new state
            self.initialize_scoped_panels_for_state(&thread_worktrees_id, window, cx);
        }

        // 5. Update active state
        self.active_workspace_state_id = Some(thread_worktrees_id);

        // 6. Update UI
        self.update_docks_for_active_state(window, cx);
        self.update_window_title(window, cx);
        cx.notify();
    }

    /// Get the active WorkspaceState
    pub fn active_workspace_state(&self) -> Option<&WorkspaceState> {
        self.active_workspace_state_id
            .as_ref()
            .and_then(|id| self.workspace_states.get(id))
    }

    pub fn active_workspace_state_mut(&mut self) -> Option<&mut WorkspaceState> {
        self.active_workspace_state_id
            .as_ref()
            .cloned()
            .and_then(move |id| self.workspace_states.get_mut(&id))
    }

    /// Close a thread context and clean up its state
    pub fn close_thread(
        &mut self,
        thread_worktrees_id: &ThreadWorktreesId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        // Remove from workspace_states - GPUI reference counting handles cleanup
        self.workspace_states.remove(thread_worktrees_id);

        // If this was the active state, switch to default or another thread
        if self.active_workspace_state_id.as_ref() == Some(thread_worktrees_id) {
            let fallback = self.workspace_states.keys().next().cloned();
            if let Some(fallback_id) = fallback {
                self.switch_to_thread(fallback_id, window, cx);
            } else {
                self.active_workspace_state_id = None;
                cx.notify();
            }
        }
    }

    fn update_docks_for_active_state(&mut self, window: &mut Window, cx: &mut Context<Self>) {
        let Some(state) = self.active_workspace_state() else {
            return;
        };

        // Update each dock to show panels from the active WorkspaceState
        for (type_id, panel) in &state.panel_instances {
            // Find which dock this panel belongs to and update it
            let position = panel.position(window, cx);
            let dock = match position {
                DockPosition::Left => &self.left_dock,
                DockPosition::Bottom => &self.bottom_dock,
                DockPosition::Right => &self.right_dock,
            };
            // Dock will need API to swap panel instance
            // dock.update(cx, |dock, cx| dock.set_panel_instance(*type_id, panel.clone(), cx));
        }
    }
}
```

---

## 2.5 Panel Instance Management

### Scoped vs Global Panels

**SCOPED (per-ThreadWorktrees)** - stored in `WorkspaceState.panel_instances`:

| Panel | Reason |
|-------|--------|
| `ProjectPanel` | Shows worktree file trees |
| `GitPanel` | Shows repos tied to worktrees |
| `OutlinePanel` | Shows context for files in worktrees |
| `DebugPanel` | Debug sessions are project-specific |
| `TerminalPanel` | Terminals have working directories in worktrees |
| `AgentPanel` | Thread-specific conversation |

**GLOBAL (shared)** - stored on `Workspace.global_panel_instances`:

| Panel | Reason |
|-------|--------|
| `CollabPanel` | Collaborators, channels, contacts - user's social state |
| `NotificationPanel` | User's notifications across all projects |

### Panel Creation

```rust
impl WorkspaceState {
    /// Get or create a scoped panel for this WorkspaceState
    pub fn get_or_create_panel<P: Panel>(
        &mut self,
        thread_worktrees: Entity<ThreadWorktrees>,
        window: &mut Window,
        cx: &mut App,
    ) -> Entity<P>
    where
        P: Panel + PanelFromThreadWorktrees,
    {
        let type_id = TypeId::of::<P>();
        if let Some(panel) = self.panel_instances.get(&type_id) {
            return panel
                .to_any()
                .downcast::<P>()
                .expect("Panel type mismatch");
        }

        let panel = cx.new(|cx| P::new(thread_worktrees, window, cx));
        self.panel_instances.insert(type_id, Box::new(panel.clone()));
        panel
    }
}

/// Trait for panels that can be constructed from ThreadWorktrees
pub trait PanelFromThreadWorktrees: Panel {
    fn new(
        thread_worktrees: Entity<ThreadWorktrees>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self;
}
```

### Initialize Scoped Panels

```rust
impl Workspace {
    fn initialize_scoped_panels_for_state(
        &mut self,
        thread_worktrees_id: &ThreadWorktreesId,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        let Some(state) = self.workspace_states.get_mut(thread_worktrees_id) else {
            return;
        };
        let thread_worktrees = state.thread_worktrees.clone();

        // Initialize all scoped panels
        // Note: This follows the pattern in zed.rs initialize_panels but per-state
        
        // ProjectPanel
        let project_panel = cx.new(|cx| {
            ProjectPanel::new_from_thread_worktrees(thread_worktrees.clone(), window, cx)
        });
        state.panel_instances.insert(
            TypeId::of::<ProjectPanel>(),
            Box::new(project_panel),
        );

        // GitPanel
        let git_panel = cx.new(|cx| {
            GitPanel::new_from_thread_worktrees(thread_worktrees.clone(), window, cx)
        });
        state.panel_instances.insert(
            TypeId::of::<GitPanel>(),
            Box::new(git_panel),
        );

        // OutlinePanel, DebugPanel, TerminalPanel, AgentPanel...
    }
}
```

---

## 2.6 Panel Migration Pattern

### Before/After Comparison

```rust
// BEFORE: Panel holds Project directly
pub struct GitPanel {
    project: Entity<Project>,
    active_repository: Option<Entity<Repository>>,
    // ...
}

impl GitPanel {
    fn new(workspace: &mut Workspace, window: &mut Window, cx: &mut Context<Workspace>) -> Entity<Self> {
        let project = workspace.project().clone();
        let git_store = project.read(cx).git_store().clone();
        let active_repository = project.read(cx).active_repository(cx);
        
        cx.new(|cx| {
            // Subscribe to git_store directly
            cx.subscribe(&git_store, |this, _, event, cx| {
                // Handle ALL events, no filtering
                match event {
                    GitStoreEvent::ActiveRepositoryChanged(repo) => {
                        this.active_repository = repo.clone();
                        cx.notify();
                    }
                    // ...
                }
            }).detach();
            
            Self { project, active_repository, /* ... */ }
        })
    }
}
```

```rust
// AFTER: Panel holds ThreadWorktrees
pub struct GitPanel {
    thread_worktrees: Entity<ThreadWorktrees>,
    active_repository: Option<Entity<Repository>>,
    // ...
}

impl GitPanel {
    fn new_from_thread_worktrees(
        thread_worktrees: Entity<ThreadWorktrees>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Self {
        let active_repository = thread_worktrees.read(cx).active_repository(cx);
        
        // Subscribe to ThreadWorktrees events (pre-filtered)
        cx.subscribe(&thread_worktrees, |this, _, event, cx| {
            match event {
                ThreadWorktreesEvent::ActiveRepositoryChanged(repo) => {
                    this.active_repository = repo.clone();
                    cx.notify();
                }
                ThreadWorktreesEvent::RepositoryUpdated(repo_id, event) => {
                    // Already filtered to our scope
                    this.handle_repo_update(*repo_id, event, cx);
                }
                // ...
            }
        }).detach();
        
        Self { thread_worktrees, active_repository, /* ... */ }
    }
}

// Method migration example
impl GitPanel {
    // BEFORE
    fn old_get_repos(&self, cx: &App) -> Vec<Entity<Repository>> {
        self.project.read(cx).visible_worktrees(cx)
            .filter_map(|wt| /* get repo */)
            .collect()
    }
    
    // AFTER - same API, scoped results
    fn get_repos(&self, cx: &App) -> Vec<Entity<Repository>> {
        self.thread_worktrees.read(cx).visible_worktrees(cx)
            .filter_map(|wt| /* get repo */)
            .collect()
    }
}
```

### Migration Checklist Per Panel

For each panel migrating from `Entity<Project>` to `Entity<ThreadWorktrees>`:

1. [ ] Replace `project: Entity<Project>` field with `thread_worktrees: Entity<ThreadWorktrees>`
2. [ ] Update constructor to take `Entity<ThreadWorktrees>` parameter
3. [ ] Replace `cx.subscribe(&git_store, ...)` with `cx.subscribe(&thread_worktrees, ...)`
4. [ ] Update event handlers to use `ThreadWorktreesEvent` variants
5. [ ] Replace `self.project.read(cx).visible_worktrees(cx)` with `self.thread_worktrees.read(cx).visible_worktrees(cx)`
6. [ ] Replace `self.project.read(cx).active_repository(cx)` with `self.thread_worktrees.read(cx).active_repository(cx)`
7. [ ] Keep pass-through methods: `is_via_collab()`, `languages()`, `path_style()`, `fs()`
8. [ ] Add `impl PanelFromThreadWorktrees for Panel` if needed

---

## 2.7 Feature-Flagged Actions

All Phase 2 functionality is gated behind the `agent-v2` feature flag.

### New Actions

```rust
// crates/workspace/src/workspace.rs

actions!(
    workspace,
    [
        /// Switch to a specific thread's workspace state
        SwitchToThread,
        /// Open a thread (loads worktrees, creates WorkspaceState if needed)
        OpenThread,
        /// Close a thread's workspace state
        CloseThread,
    ]
);

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct SwitchToThread {
    pub session_id: String,
}

impl_actions!(workspace, [SwitchToThread, OpenThread, CloseThread]);
```

### Action Handlers

```rust
impl Workspace {
    pub fn register_thread_actions(cx: &mut App) {
        if !cx.has_flag::<AgentV2FeatureFlag>() {
            return;
        }

        cx.on_action(|workspace: &mut Workspace, action: &SwitchToThread, window, cx| {
            let session_id = acp::SessionId::from(action.session_id.clone());
            let workspace_db_id = workspace.database_id().unwrap_or_default();
            let thread_worktrees_id = ThreadWorktreesId::for_thread(session_id, workspace_db_id);
            workspace.switch_to_thread(thread_worktrees_id, window, cx);
        });
    }
}
```

---

## Cross-Thread File References

**Resolution: Center panes are NOT scoped, only panels are**

When "Go to Definition" (or similar navigation) targets a file outside the current ThreadWorktrees:

- **If the worktree is loaded** (another thread is live) → **Just works**. The file opens in a center tab. ProjectPanel won't show it in its tree (filtered by ThreadWorktrees), but the editor works normally.

- **If the worktree is NOT loaded** → **File not available**. Show message: "Definition is in `/path` which is not currently loaded."

### Architecture

```
┌─────────────────────────────────────────────────────────┐
│                         Window                          │
├──────────┬──────────────────────────────────────────────┤
│  Agents  │  ┌─────────┬──────────────────┬────────┐    │
│  Sidebar │  │ Left    │                  │ Right  │    │
│          │  │ Dock    │   Center Panes   │  Dock  │    │
│          │  │         │                  │        │    │
│          │  │ SCOPED  │   NOT SCOPED     │ SCOPED │    │
│          │  │ (shows  │   (can open any  │        │    │
│          │  │  only   │    buffer from   │        │    │
│          │  │  Thread │    any loaded    │        │    │
│          │  │  A's    │    worktree)     │        │    │
│          │  │  trees) │                  │        │    │
│          │  └─────────┴──────────────────┴────────┘    │
└──────────┴──────────────────────────────────────────────┘
```

This matches current Zed behavior - you can have open files that aren't in the project panel (via drag-drop, external open, etc.).

---

## Key Files to Modify

| File | Changes |
|------|---------|
| `crates/project/src/thread_worktrees.rs` | **NEW**: ThreadWorktrees struct, ThreadWorktreesId, event filtering |
| `crates/project/src/project.rs` | Add `thread_worktrees` HashMap, `active_thread_worktrees_id`, helper methods |
| `crates/workspace/src/workspace.rs` | Add `WorkspaceState`, `workspace_states` HashMap, switch_to_thread logic |
| `crates/git_ui/src/git_panel.rs` | Migrate from `Entity<Project>` to `Entity<ThreadWorktrees>` |
| `crates/project_panel/src/project_panel.rs` | Migrate from `Entity<Project>` to `Entity<ThreadWorktrees>` |
| `crates/outline_panel/src/outline_panel.rs` | Migrate from `Entity<Project>` to `Entity<ThreadWorktrees>` |
| `crates/debugger_ui/src/debugger_panel.rs` | Migrate from `Entity<Project>` to `Entity<ThreadWorktrees>` |
| `crates/terminal_view/src/terminal_panel.rs` | Migrate from `Entity<Project>` to `Entity<ThreadWorktrees>` |
| `crates/agent_ui/src/agent_panel.rs` | Migrate from `Entity<Project>` to `Entity<ThreadWorktrees>` |

---

## Implementation Order

### Step 1: Core Infrastructure
1. Create `crates/project/src/thread_worktrees.rs` with `ThreadWorktreesId` and `ThreadWorktrees` struct
2. Add basic API methods (scoped worktree access, pass-throughs)
3. Add `thread_worktrees` HashMap to `Project`
4. Wire up module in `crates/project/src/project.rs`

### Step 2: Event Filtering
1. Implement `ThreadWorktreesEvent` enum
2. Add subscriptions to git_store and worktree_store in `ThreadWorktrees::new`
3. Implement `handle_git_store_event` and `handle_worktree_store_event`

### Step 3: WorkspaceState
1. Create `WorkspaceState` struct in workspace.rs
2. Add `workspace_states` HashMap to `Workspace`
3. Implement `switch_to_thread` method
4. Add feature-flagged actions

### Step 4: Panel Migration (one at a time)
1. Start with `ProjectPanel` (simplest, most isolated)
2. Then `GitPanel` (tests event filtering)
3. Then remaining panels

### Step 5: Integration Testing
1. Test switching between threads
2. Test panel state preservation
3. Test cross-thread file navigation

---

## Testing Strategy

### Unit Tests

```rust
#[gpui::test]
async fn test_thread_worktrees_scopes_worktrees(cx: &mut TestAppContext) {
    let fs = FakeFs::new(cx.background_executor.clone());
    fs.insert_tree("/project_a", json!({"src": {"main.rs": ""}})).await;
    fs.insert_tree("/project_b", json!({"lib": {"mod.rs": ""}})).await;

    let project = Project::test(fs.clone(), ["/project_a", "/project_b"], cx).await;
    
    // Create ThreadWorktrees with only project_a
    let worktree_a_id = project.read_with(cx, |p, cx| {
        p.worktrees(cx).next().unwrap().read(cx).id()
    });
    
    let thread_worktrees = cx.new(|cx| {
        ThreadWorktrees::new(
            ThreadWorktreesId::default_for_workspace(WorkspaceId(1)),
            project.clone(),
            HashSet::from([worktree_a_id]),
            cx,
        )
    });
    
    // Should only see project_a worktree
    let visible: Vec<_> = thread_worktrees.read_with(cx, |tw, cx| {
        tw.visible_worktrees(cx).collect()
    });
    assert_eq!(visible.len(), 1);
}
```

### Integration Tests

```rust
#[gpui::test]
async fn test_switch_between_threads_preserves_state(cx: &mut TestAppContext) {
    // Setup workspace with two threads
    // Switch to thread A, make changes
    // Switch to thread B
    // Switch back to thread A
    // Verify state is preserved
}

#[gpui::test]
async fn test_panel_receives_filtered_events(cx: &mut TestAppContext) {
    // Setup ThreadWorktrees with subset of worktrees
    // Trigger git event on worktree outside scope
    // Verify panel doesn't receive event
    // Trigger git event on worktree inside scope
    // Verify panel receives event
}
```

---

## Design Decisions Summary

1. **Project remains singular** - One `Entity<Project>` per window holds all worktrees. ThreadWorktrees is a filtered lens, not a separate data store.

2. **ThreadWorktrees as panel API surface** - Provides type safety, centralized event filtering, and clear API boundary.

3. **Buffer store is shared** - Buffers represent files which are singular. Edits in shared worktrees are visible across all threads immediately.

4. **Center panes are not scoped** - Only panels are filtered. Users can open any file from any loaded worktree in editor tabs.

5. **Panel state preserved by design** - Each ThreadWorktrees has its own panel instances stored in WorkspaceState. Switching threads swaps which instances are displayed.

6. **Reference counting handles cleanup** - No explicit cleanup logic needed. When WorkspaceState is removed, GPUI drops panel instances automatically.

---

## Open Questions for Phase 2

1. **Dock panel swapping mechanism** - How exactly do docks swap panel instances? May need new Dock API.

2. **Panel initialization timing** - Should panels be lazily initialized on first access or eagerly on thread switch?

3. **Serialization** - Should WorkspaceState be serialized per-thread for restore on restart? (Likely Phase 3)
