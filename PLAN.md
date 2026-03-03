# Multi-ConnectionView Support in AgentPanel

## Goal

Support retaining background agent threads in the `AgentPanel` so that:

1. Navigating away from a **running** thread keeps its `ConnectionView` alive (preserving execution, notifications, and UI state).
2. A completed background thread remains retained until the user clicks back to it and opens it.
3. Navigating away from an **idle** (not running, not recently completed in background) thread drops it as we do today.
4. The `Sidebar` can subscribe to the `AgentPanel` for thread lifecycle events and render them with live status.

## Current Architecture

### Ownership chain

```
AgentPanel
  ├── active_view: ActiveView::AgentThread { server_view: Entity<ConnectionView> }
  └── background_views: HashMap<SessionId, Entity<ConnectionView>>
        └── ConnectionView
              ├── agent: Rc<dyn AgentServer>
              ├── ConnectedServerState
              │     ├── connection: Rc<dyn AgentConnection>
              │     ├── active_id: Option<SessionId>
              │     └── threads: HashMap<SessionId, Entity<ThreadView>>
              ├── notifications: Vec<WindowHandle<AgentNotification>>
              └── on_release → close_all_sessions + remove notification windows
```

### Key facts

- `AgentPanel` holds one `ActiveView` (plus an optional `previous_view` for History/Config go-back) and a `background_views` map for running/unseen threads.
- Each `ConnectionView` gets a single detached observation at creation time (in `_external_thread`) that dynamically emits `ActiveViewChanged` or `BackgroundThreadChanged` depending on whether it's the active view.
- Dropping a `ConnectionView` triggers `on_release`, which calls `close_all_sessions` and removes notification windows. The detached observation is also auto-cleaned.
- Notifications (OS popups + sounds) are driven by `ConnectionView::handle_thread_event`, which subscribes to `AcpThreadEvent` on each `AcpThread`.
- The `Subscription` that routes `AcpThreadEvent` → `ConnectionView::handle_thread_event` is stored on `ThreadView._subscriptions`.
- Thread running state is determined by `AcpThread::status()` → `ThreadStatus::Idle | Generating`, based on whether `running_turn` is `Some`.

### Sidebar's current approach

- `Sidebar::subscribe_to_agent_panels` subscribes to `AgentPanelEvent::ActiveViewChanged` from each workspace's `AgentPanel`.
- `Sidebar::subscribe_to_threads` observes the **single active** `AcpThread` entity in each workspace.
- `Sidebar::active_thread_info_for_workspace` traverses `Workspace → AgentPanel → ConnectionView → ThreadView → AcpThread` to extract live status.
- `Sidebar::update_entries` merges persisted `ThreadStore` data with live `ActiveThreadInfo`, detects `Running → Completed` transitions on background workspaces, and sets notification badges.

## Part 1: Retain background ConnectionViews in AgentPanel ✅

### What was implemented

#### 1a. `background_views` collection

Added `background_views: HashMap<acp::SessionId, Entity<ConnectionView>>` to `AgentPanel`. The key is the primary thread's `SessionId`.

#### 1b. `retain_running_thread` in `set_active_view`

When `set_active_view` replaces an `ActiveView::AgentThread`, the old view is passed to `retain_running_thread`, which:
1. Gets the parent thread (not subagent) from the `ConnectionView`.
2. Checks `AcpThread::status()` — if `Generating`, inserts the `ConnectionView` into `background_views`.
3. If idle, lets it drop naturally.

This also handles `previous_view` — when clearing `previous_view` during a thread-to-thread transition, any running thread there is also retained.

#### 1c. Single detached observation per ConnectionView

Replaced the old `_active_view_observation: Option<Subscription>` field (which was rebuilt on every `set_active_view` call) with a single detached observation created once at `ConnectionView` creation time in `_external_thread`. The callback dynamically checks whether the view is active or background:

```rust
cx.observe(&server_view, |this, server_view, cx| {
    let is_active = this
        .as_active_server_view()
        .is_some_and(|active| active.entity_id() == server_view.entity_id());
    if is_active {
        cx.emit(AgentPanelEvent::ActiveViewChanged);
        this.serialize(cx);
    } else {
        cx.emit(AgentPanelEvent::BackgroundThreadChanged);
    }
    cx.notify();
})
.detach();
```

This avoids accumulation (one observation per entity lifetime) and needs no manual cleanup.

#### 1d. Promote background threads via `load_agent_thread`

`load_agent_thread` checks `background_views` first. If the session is there, it removes it and promotes it to `active_view` via `set_active_view`. Otherwise falls through to the existing DB-load path. All callers (sidebar, history, etc.) just call `load_agent_thread` without knowing if a thread is backgrounded.

#### 1e. Cleanup lifecycle

- Running thread navigated away from → retained in `background_views`.
- Background thread completes → stays retained (unseen results).
- User opens completed background thread → promoted to active.
- User navigates away from it while idle → drops normally.

#### 1f. Public API

- `AgentPanelEvent::BackgroundThreadChanged` — emitted when a background `ConnectionView` notifies.
- `parent_threads(&self, cx) -> Vec<Entity<ThreadView>>` — returns the primary thread view for all retained connections (active + background). Uses `parent_thread()` to walk up from subagents to the root thread.

## Part 2: Update Sidebar to track multiple threads per workspace

#### 2a. Subscribe to AgentPanel events only

The sidebar should **not** reach into the panel to observe individual threads. Instead, `subscribe_to_agent_panels` subscribes to both `AgentPanelEvent` variants, and `subscribe_to_threads` is removed entirely. The `AgentPanel` is responsible for emitting the right events for its threads:

```rust
fn subscribe_to_agent_panels(&mut self, window, cx) -> Vec<Subscription> {
    workspaces.iter().map(|workspace| {
        if let Some(agent_panel) = workspace.read(cx).panel::<AgentPanel>(cx) {
            cx.subscribe_in(&agent_panel, window, |this, _, event: &AgentPanelEvent, window, cx| {
                match event {
                    AgentPanelEvent::ActiveViewChanged
                    | AgentPanelEvent::BackgroundThreadChanged => {
                        this.update_entries(window, cx);
                    }
                }
            })
        } else {
            cx.observe_in(workspace, window, |this, _, window, cx| {
                this.update_entries(window, cx);
            })
        }
    }).collect()
}
```

This is cleaner than the current approach which maintains separate `_thread_subscriptions` — the `AgentPanel` owns the relationship with its threads and exposes changes via events.

#### 2b. Expand `active_thread_info_for_workspace` → `all_thread_infos_for_workspace`

Replace `active_thread_info_for_workspace` (which returns `Option<ActiveThreadInfo>`) with a method that returns info for all retained threads. This delegates to `AgentPanel::parent_threads` and lets the sidebar extract the fields it needs from each `ThreadView`.

#### 2c. Update `update_entries` to merge multiple live threads

Currently `update_entries` patches at most one saved thread per workspace with live info. Update it to patch **all** retained threads with live status, icon, and workspace_index. The merge loop becomes:

```rust
for info in live_thread_infos {
    if let Some(existing) = threads.iter_mut().find(|t| t.session_id == info.session_id) {
        // Patch with live status
    } else {
        // Append as new entry
    }
}
```

#### 2d. Update notification detection for multiple running threads

Currently detects `Running → Completed` on a single thread per workspace. Extend to track transitions for all retained threads by keying `old_statuses` on `(workspace_index, session_id)` instead of just `workspace_index`.

#### 2e. Sidebar click behavior

When a sidebar thread entry is clicked, it calls `load_agent_thread` on the appropriate workspace's `AgentPanel`. The `AgentPanel` handles all the logic internally (Part 1d):
- If the session is in `background_views` → promotes it to active.
- If the session is the already-active thread → no-op / focuses the panel.
- Otherwise → loads from DB as today.

The sidebar doesn't need to know which case applies.

## Implementation Order

1. ~~**Part 1a-1c:** Add `background_views` to `AgentPanel`, modify `set_active_view` to retain running threads, set up detached observations.~~ ✅
2. ~~**Part 1d-1e:** Handle promoting background threads back to active via `load_agent_thread`, and the cleanup lifecycle.~~ ✅
3. ~~**Part 1f:** Add `BackgroundThreadChanged` event and public `parent_threads` API.~~ ✅
4. **Part 2a-2b:** Simplify sidebar to subscribe only to `AgentPanel` events, remove `subscribe_to_threads`.
5. **Part 2c-2d:** Update sidebar entry building and notification detection for multiple threads.
6. **Part 2e:** Wire up sidebar click-to-navigate through `load_agent_thread`.

## Edge Cases to Handle

- **Subagent threads:** A `ConnectionView` can have N threads (primary + subagents). The primary thread's session ID is the key. All subagent threads are implicitly retained with their parent `ConnectionView`. The `parent_threads()` API uses `parent_thread()` to always return the root thread.
- **Multiple background threads from the same server type:** Each `ConnectionView` has its own `AgentConnection`, so multiple background native agent threads each have independent connections. This is fine.
- **Serialization:** Currently `AgentPanel` serializes only `last_active_thread`. Background threads are already persisted to the `ThreadStore` DB. On restart, they'll load from DB rather than being retained in memory — this is acceptable. The detached observation only calls `serialize` when the view is active.
- **Memory pressure:** If many threads accumulate in background, we may want a cap. For now, the "drop on view after idle" policy should keep the count low.
- **`AgentDiff::set_active_thread`:** Currently called in `new_thread_view`. When switching back to a background thread, we need to re-call this so the diff pane shows the correct thread's edits.
- **`previous_view`:** The existing `previous_view` field handles History/Config go-back. Background retention is orthogonal — `previous_view` continues to work as before for overlay views, while `background_views` handles thread-to-thread transitions.

## Testing Strategy

- Unit test: creating a new thread while one is running moves the old `ConnectionView` to `background_views`.
- Unit test: creating a new thread while one is idle drops the old `ConnectionView`.
- Unit test: navigating to a background session ID via `load_agent_thread` promotes it from `background_views` to `active_view`.
- Unit test: a background thread that completes stays in `background_views` until the user opens it.
- Unit test: a completed background thread that is opened and then navigated away from while idle gets dropped.
- Unit test: `BackgroundThreadChanged` is emitted when a background `ConnectionView` notifies.
- Sidebar test: `update_entries` shows live status for both active and background threads.
- Sidebar test: `Running → Completed` transition on a background thread triggers notification badge.
- Sidebar test: `subscribe_to_threads` is removed; sidebar only subscribes to `AgentPanelEvent`.