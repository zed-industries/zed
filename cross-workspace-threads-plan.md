# Cross-Workspace Thread Handling Plan

## Overview

This document outlines the design for handling agent threads across multiple workspaces, turning "zombie threads" from a bug into a feature: **background threads**.

## Problem Statement

Opening the same thread in multiple workspaces currently has significant issues:

1. **Flow disruption**: Opening a thread in two windows causes them to get out of sync
2. **Zombie threads**: Closing a window does NOT stop the thread from running - it continues processing in the background
4. **Split brain**: Each workspace creates its own `AcpThread` entity with no coordination

### Current Architecture

| Aspect | Current Behavior |
|--------|------------------|
| Thread data | Persisted to SQLite database, shared across workspaces |
| Running state | Per-workspace (no coordination) |
| Thread views | Each workspace creates its own `AcpThreadView` and `AcpThread` entity |
| Coordination | None between workspaces for the same thread |

## Research Findings

### AcpThread is Project-Scoped

`AcpThread` takes an `Entity<Project>` and uses it for:
- **Language registry** - syntax highlighting in markdown/diffs
- **Path style** - Windows vs Unix path formatting
- **Git store** - checkpoints and restore functionality
- **Buffer operations** - `read_text_file`, `write_text_file` via `project.open_buffer()`
- **Terminal environment** - directory environment for spawned terminals
- **Path resolution** - `project_path_for_absolute_path()`

**Implication**: Threads cannot be meaningfully shared across projects. A thread from Project A cannot operate on Project B's files.

### No Cancellation on Drop

- `AcpThread` has no `Drop` implementation
- When `AcpThreadView` is dropped, it only cleans up notification windows
- The `send_task` (background LLM work) continues running when entities are dropped
- `cancel()` must be called explicitly to stop a running thread

### Existing Patterns in Zed

- **`BufferStore`**: Per-project store that provides `Entity<Buffer>` instances. Multiple `Editor`s can display the same buffer. Uses `WeakEntity` and deduplicates concurrent loads.
- **`ActiveCall`**: Global singleton for call/room state across workspaces. Uses `impl Global` pattern.
- **`WorkspaceStore`**: Tracks all open workspaces via `HashSet<WindowHandle<Workspace>>`.

## Proposed Solution: AgentThreadStore

Create a new **global** `AgentThreadStore` in a new `agent_2` crate that:
1. **Owns running threads** - holds strong references to keep them alive
2. **Tracks view state** - which window (if any) is viewing each thread
3. **Enables background threads** - threads continue running when windows close

### Design

```rust
// crates/agent_2/src/agent_thread_store.rs

pub struct AgentThreadStore {
    /// Thread metadata from database (the "history")
    threads: Vec<DbThreadMetadata>,

    /// Running threads - store holds strong reference to keep them alive
    /// These continue running even if no window is viewing them
    running_threads: HashMap<acp::SessionId, Entity<AcpThread>>,

    /// Which window (if any) is currently viewing each running thread
    /// None = running in background, Some(window) = open in that window
    thread_views: HashMap<acp::SessionId, WindowHandle<Workspace>>,

    /// Recently opened for quick-access UI
    recently_opened: VecDeque<acp::SessionId>,
}

/// State of a thread for UI display
pub enum ThreadState {
    /// Not running (historical entry only)
    Idle,
    /// Running and visible in a window
    Running { window: WindowHandle<Workspace> },
    /// Running in background (no window viewing it)
    Background,
}

pub enum AgentThreadStoreEvent {
    /// History list changed (thread added/removed/updated)
    HistoryChanged,
    /// A thread started running
    ThreadStarted(acp::SessionId),
    /// A thread stopped running
    ThreadStopped(acp::SessionId),
    /// A thread's view state changed (attached/detached from window)
    ThreadViewChanged(acp::SessionId),
}

/// Result of trying to open a thread
pub enum OpenResult {
    /// Thread opened/attached in this window
    Opened(Entity<AcpThread>),
    /// Thread is open in a different window - that window's handle
    OpenElsewhere(WindowHandle<Workspace>),
}

impl AgentThreadStore {
    // --- Global access ---
    pub fn global(cx: &mut App) -> Entity<Self>;

    // --- History (read from database) ---
    pub fn history(&self) -> &[DbThreadMetadata];
    pub fn recently_opened(&self) -> impl Iterator<Item = &DbThreadMetadata>;
    pub fn reload(&mut self, cx: &mut Context<Self>) -> Task<()>;

    // --- Thread state ---

    /// Get state of a thread for UI display
    pub fn thread_state(&self, session_id: &acp::SessionId) -> ThreadState;

    /// Open/attach to a thread in this window.
    /// - If idle: caller should create thread, then call `register_running`
    /// - If background: attaches to this window
    /// - If open elsewhere: returns that window handle
    pub fn open_thread(
        &mut self,
        session_id: &acp::SessionId,
        window: WindowHandle<Workspace>,
        cx: &mut Context<Self>,
    ) -> OpenResult;

    /// Register a newly created running thread
    pub fn register_running(
        &mut self,
        session_id: acp::SessionId,
        thread: Entity<AcpThread>,
        window: WindowHandle<Workspace>,
        cx: &mut Context<Self>,
    );

    /// Detach from a thread (window closing or user explicitly detaching)
    /// Thread keeps running in background
    pub fn detach_thread(
        &mut self,
        session_id: &acp::SessionId,
        cx: &mut Context<Self>,
    );

    /// Stop a thread completely (user explicitly stopping)
    pub fn stop_thread(
        &mut self,
        session_id: &acp::SessionId,
        cx: &mut Context<Self>,
    );

    // --- Recently opened management ---
    pub fn mark_recently_opened(
        &mut self,
        session_id: acp::SessionId,
        cx: &mut Context<Self>,
    );
}
```

### Usage Flow

**Opening a thread from history:**
```rust
fn open_thread(&mut self, session_id: acp::SessionId, window: &mut Window, cx: &mut Context<Self>) {
    let store = AgentThreadStore::global(cx);
    let this_window = window.window_handle().downcast::<Workspace>().unwrap();

    let state = store.read(cx).thread_state(&session_id);

    match state {
        ThreadState::Running { window } if window == this_window => {
            // Already open here, just focus it
            self.focus_thread_view(cx);
        }
        ThreadState::Running { window } => {
            // Open in another window, activate that
            cx.activate_window(window);
        }
        ThreadState::Background => {
            // Running in background, attach to this window
            store.update(cx, |store, cx| {
                store.open_thread(&session_id, this_window, cx);
            });
            self.show_thread_view(session_id, cx);
        }
        ThreadState::Idle => {
            // Not running, need to create/resume
            let thread = self.create_thread(session_id.clone(), cx);
            store.update(cx, |store, cx| {
                store.register_running(session_id, thread, this_window, cx);
            });
        }
    }
}
```

**Window close handling:**
```rust
fn on_window_close(&mut self, window: WindowHandle<Workspace>, cx: &mut App) {
    let store = AgentThreadStore::global(cx);
    store.update(cx, |store, cx| {
        // Detach all threads viewed in this window - they become background threads
        for (session_id, view_window) in store.thread_views.iter() {
            if *view_window == window {
                store.detach_thread(session_id, cx);
            }
        }
    });
}
```

**History view rendering:**
```rust
fn render_history_entry(&self, entry: &DbThreadMetadata, cx: &App) -> impl IntoElement {
    let store = AgentThreadStore::global(cx);
    let state = store.read(cx).thread_state(&entry.id);

    let status_indicator = match state {
        ThreadState::Running { window } if window == self.this_window => {
            Icon::new(IconName::Play).color(Color::Success) // 🟢 Running here
        }
        ThreadState::Running { .. } => {
            Icon::new(IconName::Play).color(Color::Info) // 🔵 Running elsewhere
        }
        ThreadState::Background => {
            Icon::new(IconName::CloudUpload).color(Color::Warning) // 🟡 Background
        }
        ThreadState::Idle => {
            Icon::new(IconName::Circle).color(Color::Muted) // ⚪ Idle
        }
    };

    div()
        .child(status_indicator)
        .child(Label::new(&entry.title))
}
```

### Key Design Decisions

1. **Global store owns running threads**: Strong `Entity<AcpThread>` references keep threads alive
2. **Background threads are a feature**: Closing a window detaches, doesn't kill
3. **Single view per thread**: Only one window can view a thread at a time
4. **Redirect, don't duplicate**: If a thread is open elsewhere, redirect to that window

## Future Vision

This refactor sets the foundation for a deeper architectural change: **threads as the primary object**.

In the future, threads will become richer objects that the editor UI is downstream of:
- Each thread has its own worktrees
- Each thread has its own workspace state
- The editor becomes a view into a thread's state
- Multiple threads can be running simultaneously, each with their own context

This is similar to how modern AI coding tools treat "sessions" or "conversations" as first-class workspace containers.

The current refactor provides immediate value by:
1. ✅ Fixing the split-brain problem
2. ✅ Enabling background threads as a feature
3. ✅ Establishing the `AgentThreadStore` as the single source of truth
4. ✅ Setting up the ownership model for the future vision

## Implementation Plan

1. [ ] Create new `agent_2` crate
2. [ ] Implement `AgentThreadStore` with global access pattern
3. [ ] Implement thread state tracking (`running_threads`, `thread_views`)
4. [ ] Add `open_thread` / `detach_thread` / `stop_thread` APIs
5. [ ] Wire up `AgentsPanel` (v2) to use `AgentThreadStore`
6. [ ] Add window close handling to detach threads
7. [ ] Update history view to show thread state (running/background/idle)
8. [ ] Add tests for cross-workspace scenarios
9. [ ] Add UI affordances for background threads (attach, stop, etc.)

## Related Files

- `crates/agent_ui_v2/src/agents_panel.rs` - Will consume AgentThreadStore
- `crates/agent_ui_v2/src/agent_thread_pane.rs` - Thread pane management
- `crates/acp_thread/src/acp_thread.rs` - Thread entity and lifecycle
- `crates/agent/src/history_store.rs` - Existing history store (for reference)
- `crates/agent/src/db.rs` - Thread database types (DbThreadMetadata, etc.)
- `crates/workspace/src/workspace.rs` - Workspace lifecycle, WorkspaceStore pattern
