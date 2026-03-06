# Sidebar Selection Refactor Plan

## Problem

`active_item` was derived from multi-workspace/agent-panel state via `ActiveItem::from_multi_workspace()`
inside `update_entries()`. When you click a project header, it calls `activate_workspace()`, which
triggers `update_entries()`, which recomputes `active_item` — and if another workspace has a thread
open, the selection snaps to *that* thread instead of staying on what you clicked.

## Goal

Selection should reflect **user intent**: what you clicked is what stays selected.

## Implementation

### 1. `update_entries` never touches `active_item` ✅ DONE

`update_entries` rebuilds the list entries — that's all it does.

### 2. Set `active_item` in `activate_workspace` / `activate_thread` via `cx.defer` ✅ DONE

`activate_workspace` and `activate_thread` (the sidebar's internal methods) use `cx.defer`
to set `active_item` after all queued effects (events) have been processed. This ensures the
sidebar's intent always wins over the `ActiveWorkspaceChanged` / `ActiveViewChanged` events
that fire as side effects of the workspace/thread activation.

- **Project header click / confirm** → calls `activate_workspace` → deferred `Workspace(X)`
- **Thread click / confirm** → calls `activate_thread` → deferred `Thread(T)`

### 3. External sync via dedicated events ✅ DONE

Instead of deriving state or diffing, we listen directly to specific events and set intent:

**Workspace switches** — Added `MultiWorkspaceEvent::ActiveWorkspaceChanged` to the workspace
crate. `MultiWorkspace` emits this from `set_active_workspace`, `activate` (disabled path),
`activate_index`, and `remove_workspace`. The sidebar subscribes via `cx.subscribe_in` and
sets `active_item = ActiveItem::Workspace(workspace)` directly.

The existing `cx.observe_in` is kept for `update_entries` (rebuilding the list on any
multi-workspace change), but it no longer touches `active_item`.

**Thread changes** — In the `subscribe_to_agent_panels` callback, when
`AgentPanelEvent::ActiveViewChanged` fires, read the agent panel's active thread and set
`active_item = ActiveItem::Thread(session_id)` if there is one.

**Why there's no conflict with sidebar clicks**: The sidebar's `activate_workspace` and
`activate_thread` methods use `cx.defer` to set `active_item` after events are delivered.
Events fire first (potentially setting `active_item` to something wrong), then the deferred
callback runs last and sets the correct value — the user's actual intent.

### 4. Thread selection is sticky

Once you click a thread, it stays selected until you explicitly click something else.
Defocusing the sidebar does NOT clear thread selection.

### 5. Cleanup ✅ DONE

- Removed `dbg!()` calls in `render_project_header` and `render`

## Known issues with this approach

This event + defer approach almost works, but has problems:

1. **Background threads pull selection over.** When a background thread emits
   `AgentPanelEvent::ActiveViewChanged`, the sidebar's subscription sets
   `active_item = Thread(session_id)`, yanking the selection to that thread
   even though the user didn't interact with it. Any agent panel activity
   (thread status changes, title updates, etc.) can cause unwanted selection
   jumps.

2. **Redundant `update_entries` calls.** A single thread click triggers at least
   2 full `update_entries` rebuilds (one from the multi-workspace observe, one
   from the agent panel event subscription). Each rebuild re-subscribes to all
   projects and agent panels. The defers are scheduled via the same effect
   queue as events, so there's no clean way to coalesce them without a
   shielding boolean.

3. **`cx.defer` ordering is fragile.** The correctness relies on deferred
   callbacks running after event effects in the GPUI effect queue. This is
   an implementation detail of effect ordering, not a documented guarantee.

## Next: explore focus-based approach

Instead of explicit state tracking, derive `active_item` from what actually has
focus. The sidebar would check focus state in the render path:
- If the agent panel is focused and has an active thread → show that thread
- Otherwise → show the active workspace

Thread stickiness via a lightweight `focused_thread: Option<SessionId>` —
filled when you click/focus a thread, cleared when you switch workspaces.

This avoids the event/defer dance entirely because the side effects (focusing
the workspace, focusing the agent panel) naturally produce the correct focus
state.

## External change inventory

### Workspace switches (12 external paths)

All handled by `cx.observe_in(&multi_workspace, ...)` + `last_active_workspace` tracking:
- `NextWorkspaceInWindow` / `PreviousWorkspaceInWindow` keybindings
- `NewWorkspaceInWindow` action
- `WorkspaceEvent::Activate` (save prompts, close-in-call prompts)
- `Workspace::new_local` (opening a folder/project)
- `open_paths` (CLI, Finder, drag-and-drop)
- `restore_multiworkspace` (session restore)
- `open_remote_project_inner` (SSH project)
- `join_in_room_project` (collab room)
- Recent/remote project picker selection
- Agent panel `setup_new_workspace` (agent creates workspace)
- Agent notification "Accept" click
- App quit flow (iterates workspaces for save prompts)

### Thread changes (19 external paths)

All handled by `AgentPanelEvent::ActiveViewChanged` subscription:
- `NewThread` workspace action (keybinding)
- `NewNativeAgentThreadFromSummary` action
- `NewExternalAgentThread` action
- `NewTextThread` action
- `ReviewBranchDiff` action (git UI)
- `LoadThreadFromClipboard` action
- Panel deserialization (workspace restore)
- Panel history `ThreadHistoryEvent::Open`
- History entry element click
- Navigation menu "Recently Updated" pick
- Thread mention crease click (inline link)
- Configuration view "New Thread" button
- CLI `--agent` open request
- Shared thread URL (`zed://agent-thread/...`)
- Notification click (focuses panel with already-active thread)
- ACP onboarding "Open Panel" button
- Claude Agent onboarding "Open Panel" button
- New worktree workspace setup
