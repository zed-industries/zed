# Sidebar Selection Refactor Plan

## Problem

`active_item` was derived from multi-workspace/agent-panel state via `ActiveItem::from_multi_workspace()`
inside `update_entries()`. When you click a project header, it calls `activate_workspace()`, which
triggers `update_entries()`, which recomputes `active_item` — and if another workspace has a thread
open, the selection snaps to *that* thread instead of staying on what you clicked.

## Goal

Selection should reflect **user intent**: what you clicked is what stays selected.

## Implementation: focus-based model

Replaced the `active_item: ActiveItem` enum with a single field:

```rust
focused_thread: Option<acp::SessionId>
```

### Design principle

- `Some(id)` → that thread is highlighted in the sidebar
- `None` → the active workspace header is highlighted (derived from `multi_workspace.workspace()` at render time)
- **Set** on thread click (`activate_thread`) or external `AgentPanelEvent::ActiveViewChanged`
- **Cleared** on workspace header click (`activate_workspace`)
- **Sticky** across defocus, external workspace switches, and re-renders

### Why this works

Each code path explicitly manages its own state:

1. **`activate_workspace`** (workspace header click) → `focused_thread = None`, then activates the workspace
2. **`activate_thread`** (thread click/confirm) → `focused_thread = Some(session_id)`, then activates workspace + loads thread
3. **`AgentPanelEvent::ActiveViewChanged`** (external thread open) → `focused_thread = Some(session_id)` from agent panel's active connection
4. **`MultiWorkspaceEvent::ActiveWorkspaceChanged`** → just `cx.notify()` to trigger re-render; does NOT touch `focused_thread`

### Avoiding the deferred-effect ordering bug

The previous approach had `ActiveWorkspaceChanged` clear `focused_thread`. This caused a bug:
when `activate_thread` switched workspaces (cross-workspace thread click), the event fired as
a deferred effect and clobbered the `focused_thread` that was just set.

The fix: `ActiveWorkspaceChanged` never touches `focused_thread`. The event only triggers a
re-render via `cx.notify()`. This makes `focused_thread` "sticky" — it stays wherever the user
put it until an explicit action changes it.

### Render logic

- **`render_project_header`**: `toggle_state` is true when `focused_thread.is_none()` AND
  the workspace is the active one in `multi_workspace`
- **`render_thread`**: `.selected()` is true when `focused_thread == Some(session_info.session_id)`

### `update_entries` is purely structural

`update_entries` rebuilds the list entries, re-subscribes to projects/agent panels, and
resets the list state. It never touches `focused_thread`.

## Changes to workspace crate

Added `MultiWorkspaceEvent::ActiveWorkspaceChanged` enum and `impl EventEmitter<MultiWorkspaceEvent> for MultiWorkspace`.

Emitted from:
- `set_active_workspace` — guarded by `changed` check (only when index actually changes)
- `activate` (disabled multi-workspace path) — always emits
- `activate_index` — guarded by `changed` check
- `remove_workspace` — always emits (active workspace may have shifted)

## Test coverage

`test_focused_thread_tracks_user_intent` covers 8 scenarios:
1. Initial state — `focused_thread` tracks thread opened during setup
2. Click workspace header — clears `focused_thread`
3. Click thread — sets `focused_thread`
4. Cross-workspace thread click — sets `focused_thread` (not clobbered by workspace change event)
5. External workspace switch — `focused_thread` stays (sticky)
6. External thread open — updates `focused_thread`
7. Thread selection is sticky across defocus
8. Clicking workspace header clears sticky thread
