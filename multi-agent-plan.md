# Multi-Agent Support Plan for agent_ui_v2

## Overview

This document outlines the implementation plan for supporting multiple concurrent agent threads in the `agent_ui_v2` crate. The goal is to allow users to run multiple agents simultaneously on the same codebase, with proper lifecycle management and visual indicators.

## Current Architecture Summary

### Key Components

| Component | File | Purpose |
|-----------|------|---------|
| `AgentsPanel` | `agents_panel.rs` | Top-level panel, manages single `AgentThreadPane` |
| `AgentThreadPane` | `agent_thread_pane.rs` | Utility pane wrapper for thread view |
| `AcpThreadHistory` | `thread_history.rs` | Thread list and search UI |
| `AcpThread` | `acp_thread/acp_thread.rs` | Core thread with LLM processing |
| `AcpThreadView` | `agent_ui/acp/thread_view.rs` | Renders thread content |

### Current Limitations

1. **Single Active Thread**: `AgentsPanel` holds `agent_thread_pane: Option<Entity<AgentThreadPane>>` - only one thread can be viewed at a time
2. **Thread Lifecycle Tied to UI**: When switching threads, the previous thread's entities may be dropped, canceling in-flight requests
3. **No Running Threads Tracking**: No mechanism to track which threads are currently processing LLM requests
4. **No Visual Indicators**: Thread history doesn't show which threads are actively generating

## Proposed Changes

### 1. Running Threads Registry

The key insight from the current implementation:
- `AgentsPanel::open_thread()` creates a new `AgentThreadPane` each time
- `AgentThreadPane::open_thread()` creates a new `AcpThreadView` which manages the `AcpThread`
- When switching threads, the old pane/view gets replaced and dropped

We need to retain `AcpThreadView` entities (which contain the `AcpThread`) for running threads.

Add a registry in `AgentsPanel`:

```rust
// In agents_panel.rs

struct RunningThreadView {
    view: Entity<AcpThreadView>,
    thread_id: HistoryEntryId,
    _subscription: Subscription,  // To receive thread events
}

pub struct AgentsPanel {
    // ... existing fields ...

    /// Thread views that are currently generating responses.
    /// These are retained even when not visible in the UI, allowing
    /// background processing to continue.
    running_thread_views: HashMap<HistoryEntryId, RunningThreadView>,

    /// Thread IDs that completed in the background but haven't been viewed yet.
    /// These stay in the "Running" section with a checkmark icon until opened.
    completed_thread_ids: HashSet<HistoryEntryId>,
}
```

**Why `AcpThreadView` instead of `AcpThread`?**

The `AcpThreadView` manages the `AcpThread` internally and handles:
- Notifications (sound, popup)
- Tool authorization UI callbacks
- Error handling and display

Retaining the view ensures all these behaviors continue working for background threads.

### 2. Thread Lifecycle Changes

#### Starting a Thread (Sending a Message)

When a message is sent and the thread starts generating:

1. `AcpThread::run_turn()` sets `send_task = Some(Task)` and emits `AcpThreadEvent::Started`
2. `AcpThreadView` receives this event and notifies observers
3. `AgentsPanel` (subscribed to the thread view) adds the view to `running_thread_views`

#### Switching Threads

When the user clicks on a different thread in history:

```rust
// In AgentsPanel::open_thread()

fn open_thread(&mut self, entry: HistoryEntry, ...) {
    let entry_id = entry.id();

    // Check if new thread is already running - reuse its view
    if let Some(running) = self.running_thread_views.get(&entry_id) {
        // Create pane with existing view instead of creating a new one
        self.create_pane_with_existing_view(running.view.clone(), entry_id, window, cx);
        return;
    }

    // Otherwise, create new pane/view as before
    // ... existing open_thread logic ...
}
```

#### Stopping a Thread

A thread should only be stopped/cancelled when:

1. User clicks the **stop button** in the thread view
2. User clicks the **"X" button** to close the utility pane **while viewing that thread**
3. User explicitly **deletes the thread** from history

When stopped:

```rust
// In AgentsPanel - when close pane event received while viewing a running thread

fn handle_close_pane_event(&mut self, pane: Entity<AgentThreadPane>, _event: &ClosePane, cx: &mut Context<Self>) {
    // Get the thread being viewed
    if let Some(thread_id) = pane.read(cx).thread_id() {
        // If it's running, cancel it
        if let Some(running) = self.running_thread_views.remove(&thread_id) {
            running.view.update(cx, |view, cx| {
                if let Some(thread) = view.thread() {
                    thread.update(cx, |t, cx| t.cancel(cx));
                }
            });
        }
    }

    self.agent_thread_pane = None;
    self.update_history_running_threads(cx);
    self.serialize(cx);
    cx.notify();
}
```

**Important**: Clicking away (switching to another thread) does NOT cancel - only explicit close/stop.

#### Thread Completion

When a thread finishes generating naturally:

1. `AcpThread::run_turn()` async completes, `send_task` becomes `None`
2. `AcpThread` emits `AcpThreadEvent::Stopped` or handles error with `AcpThreadEvent::Error`
3. `AcpThreadView` receives this and shows notification (already implemented)
4. `AgentsPanel` (subscribed) removes thread from `running_thread_views`

### 3. New Events

Add to `AcpThreadEvent`:

```rust
pub enum AcpThreadEvent {
    // ... existing events ...

    /// Emitted when the thread starts processing a request
    Started,
}
```

### 4. UI Changes for Thread History

The current `AcpThreadHistory` (`thread_history.rs`) uses a `ListItemType` enum and `TimeBucket` for grouping. We'll extend this pattern.

#### Extend ListItemType and TimeBucket

```rust
// In thread_history.rs

enum ListItemType {
    BucketSeparator(TimeBucket),
    Entry {
        entry: HistoryEntry,
        format: EntryTimeFormat,
        is_running: bool,
        is_completed: bool,  // NEW: completed in background but not yet viewed
    },
    SearchResult {
        entry: HistoryEntry,
        positions: Vec<usize>,
        is_running: bool,
        is_completed: bool,  // NEW: completed in background but not yet viewed
    },
}

#[derive(PartialEq, Eq, Clone, Copy, Debug)]
enum TimeBucket {
    Running,  // NEW: for active threads
    Today,
    Yesterday,
    ThisWeek,
    PastWeek,
    All,
}
```

#### Modify add_list_separators

```rust
fn add_list_separators(
    &self,
    entries: Vec<HistoryEntry>,
    running_thread_ids: &HashSet<HistoryEntryId>,
    completed_thread_ids: &HashSet<HistoryEntryId>,  // NEW: completed but not yet viewed
    cx: &App,
) -> Task<Vec<ListItemType>> {
    cx.background_spawn(async move {
        let mut items = Vec::with_capacity(entries.len() + 2);
        let mut bucket = None;
        let today = Local::now().naive_local().date();

        // Separate running/completed and other entries
        // Running and completed threads both stay in the "Running" section
        let (active, other): (Vec<_>, Vec<_>) = entries
            .into_iter()
            .partition(|e| running_thread_ids.contains(&e.id()) || completed_thread_ids.contains(&e.id()));

        // Add running/completed threads first
        if !active.is_empty() {
            items.push(ListItemType::BucketSeparator(TimeBucket::Running));
            for entry in active {
                let is_running = running_thread_ids.contains(&entry.id());
                let is_completed = completed_thread_ids.contains(&entry.id());
                items.push(ListItemType::Entry {
                    entry,
                    format: EntryTimeFormat::TimeOnly,
                    is_running,
                    is_completed,
                });
            }
        }

        // Then add other threads with time buckets
        for entry in other {
            let entry_date = entry.updated_at().with_timezone(&Local).naive_local().date();
            let entry_bucket = TimeBucket::from_dates(today, entry_date);

            if Some(entry_bucket) != bucket {
                bucket = Some(entry_bucket);
                items.push(ListItemType::BucketSeparator(entry_bucket));
            }

            items.push(ListItemType::Entry {
                entry,
                format: entry_bucket.into(),
                is_running: false,
            });
        }
        items
    })
}
```

#### Loading Spinner in render_history_entry

```rust
fn render_history_entry(
    &self,
    entry: &HistoryEntry,
    format: EntryTimeFormat,
    ix: usize,
    highlight_positions: Vec<usize>,
    is_running: bool,
    is_completed: bool,
    cx: &Context<Self>,
) -> AnyElement {
    // Add to the existing entry rendering:
    // - is_running: show animated spinner icon (IconName::ArrowCircle with rotation)
    // - is_completed: show checkmark icon (IconName::Check, Color::Success)
    // - otherwise: show timestamp as before
}
```

#### TimeBucket Display for "Running"

```rust
impl Display for TimeBucket {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            TimeBucket::Running => write!(f, "Running"),  // NEW
            TimeBucket::Today => write!(f, "Today"),
            // ... rest unchanged ...
        }
    }
}
```

#### Communicate Running Status to History

`AcpThreadHistory` needs access to running thread IDs at render time. Since `AgentsPanel` already maintains `running_thread_views: HashMap<HistoryEntryId, RunningThreadView>`, we derive the running status from that rather than duplicating state.

**Approach: Separate State Entity from Render Component**

1. Keep `AcpThreadHistory` as an `Entity` that manages state:
   - `selected_index`, `hovered_index`, `search_query`, `visible_items`
   - `search_editor: Entity<Editor>`
   - Subscriptions to `history_store` and `search_editor`
   - Async `_update_task` for background processing

2. Remove the `impl Render for AcpThreadHistory`

3. Create a new `AcpThreadHistoryComponent` that implements `RenderOnce`:

```rust
// NOTE: The lifetime approach below is experimental - if GPUI's RenderOnce
// doesn't play well with lifetimes, fall back to passing HashSet<HistoryEntryId> by value.
#[derive(IntoElement)]
pub struct AcpThreadHistoryComponent<'a> {
    history: Entity<AcpThreadHistory>,
    running_thread_ids: &'a HashSet<HistoryEntryId>,
    completed_thread_ids: &'a HashSet<HistoryEntryId>,
}

impl<'a> AcpThreadHistoryComponent<'a> {
    pub fn new(
        history: Entity<AcpThreadHistory>,
        running_thread_ids: &'a HashSet<HistoryEntryId>,
        completed_thread_ids: &'a HashSet<HistoryEntryId>,
    ) -> Self {
        Self { history, running_thread_ids, completed_thread_ids }
    }
}

impl<'a> RenderOnce for AcpThreadHistoryComponent<'a> {
    fn render(self, window: &mut Window, cx: &mut App) -> impl IntoElement {
        // Read from self.history and use running/completed thread IDs
        // to determine which entries show spinners vs checkmarks
        self.history.update(cx, |history, cx| {
            history.render_with_running_threads(
                self.running_thread_ids,
                self.completed_thread_ids,
                window,
                cx,
            )
        })
    }
}
```

4. `AgentsPanel` renders history by creating the component:

```rust
// In AgentsPanel::render()
let running_ids: HashSet<_> = self.running_thread_views.keys().cloned().collect();
// ...
.child(AcpThreadHistoryComponent::new(
    self.history.clone(),
    &running_ids,
    &self.completed_thread_ids,
))
```

This keeps the source of truth for running threads in `AgentsPanel` while allowing `AcpThreadHistory` to use that information during rendering without storing duplicate state.

### 5. Pane Close Behavior

Modify `AgentThreadPane` close handling:

- `AgentThreadPane::on_close()` emits `AgentsPaneEvent::CancelThread(thread_id)` before `ClosePane`
- `AgentsPanel::handle_pane_event()` handles `CancelThread` by removing from `running_thread_views` and calling `thread.cancel(cx)`

### 6. Reconnecting to Running Threads

When opening a thread, check `running_thread_views` first - if present, reuse the existing view instead of loading from storage.

## Implementation Steps

### Phase 1: Running Threads Registry

1. Add `running_thread_views: HashMap<HistoryEntryId, RunningThreadView>` to `AgentsPanel`
2. Add `Started` event to `AcpThreadEvent` (emitted when `run_turn` starts)
3. Subscribe to `AcpThreadView` events in `AgentsPanel` when opening threads
4. On `Started`, add view to registry; on `Stopped`/`Error`, remove from registry


### Phase 2: Thread Lifecycle Management

1. Modify `AgentsPanel::open_thread()` to check `running_thread_views` first and reuse existing view
2. Add `AgentThreadPane::set_thread_view()` method to display an existing `AcpThreadView`
3. When switching threads, the old view stays alive in `running_thread_views` if running
4. Modify close handling to cancel the viewed thread if running, then remove from registry

### Phase 3: UI Indicators

1. Add method to check if a thread is running: `AgentsPanel::is_thread_running(id)`
2. Expose running status to `AcpThreadHistory` (via callback or shared state)
3. Group running threads at the top of the history list with a "Running" section header
4. Render loading spinner next to each running thread in history list
5. Add running threads count badge to panel header

### Phase 4: Edge Cases & Polish

1. Handle thread errors - remove from `running_thread_views` on `AcpThreadEvent::Error`
2. Handle thread deletion - if user deletes a running thread from history, cancel and remove it
3. Add keyboard shortcut to quickly switch to next/previous running thread
4. Cross-workspace thread handling (see Follow-Up section)

## Data Flow Diagram

```
User sends message in thread view
        ↓
AcpThreadView::send_message()
        ↓
AcpThread::send() → run_turn()
  ├─→ self.send_task = Some(Task)
  └─→ cx.emit(AcpThreadEvent::Started)
        ↓
AcpThreadView receives event, notifies observers
        ↓
AgentsPanel receives notification (via cx.observe)
  └─→ self.running_thread_views.insert(thread_id, RunningThreadView { view, ... })
        ↓
AcpThreadHistory re-renders (running status derived from running_thread_views)
        ↓
[User may switch to different thread]
  └─→ AgentsPanel::open_thread()
      ├─→ View stays alive in running_thread_views
      └─→ New pane created for different thread
        ↓
[Eventually, generation completes]
        ↓
AcpThread::run_turn() async completes
  ├─→ self.send_task = None
  └─→ cx.emit(AcpThreadEvent::Stopped)
        ↓
AcpThreadView receives Stopped, shows notification (already implemented)
        ↓
AgentsPanel receives notification
  ├─→ self.running_thread_views.remove(thread_id)
  └─→ self.completed_thread_ids.insert(thread_id)
        ↓
AcpThreadHistory re-renders
  └─→ Thread stays in "Running" section but shows checkmark icon instead of spinner
        ↓
[User opens the completed thread]
  └─→ AgentsPanel::open_thread()
      └─→ self.completed_thread_ids.remove(thread_id)
            ↓
AcpThreadHistory re-renders
  └─→ Thread moves to time-based bucket
```

## Cancellation Flow

```
User clicks X on pane (while viewing Thread A)
        ↓
AgentThreadPane close button clicked
  └─→ cx.emit(ClosePane)
        ↓
AgentsPanel::handle_close_pane_event()
  ├─→ Gets thread_id from pane
  ├─→ If running: running_thread_views.remove(thread_id)
  └─→ view.update(cx, |v, cx| v.thread().cancel(cx))
        ↓
AcpThread::cancel()
  ├─→ self.send_task.take()
  ├─→ Mark pending tool calls as Canceled
  └─→ self.connection.cancel()
        ↓
UI updates, pane closes, thread no longer in "Running" section
```

## Design Decisions

1. **No maximum concurrent threads limit**: Users are unlikely to create many concurrent threads, so no artificial limit is needed.

2. **Notifications already implemented**: The `AcpThreadView` (in `agent_ui/src/acp/thread_view.rs`) already handles notifications via the `notify_with_sound()` method, triggered by:
   - `AcpThreadEvent::ToolAuthorizationRequired` - "Waiting for tool confirmation"
   - `AcpThreadEvent::Stopped` - Shows summary of tools used when generation completes
   - `AcpThreadEvent::Refusal` - Model refusal notification
   - `AcpThreadEvent::Error` - Error notification

   Since `agent_ui_v2` wraps `AcpThreadView`, these notifications will work automatically for background threads.

3. **Running threads grouped at top**: In the thread history UI, running threads should be grouped at the top of the list for easy access.

## Files to Modify

| File | Changes |
|------|---------|
| `agent_ui_v2/src/agents_panel.rs` | Add `running_thread_views` registry, modify `open_thread()`, render history via `AcpThreadHistoryComponent` |
| `agent_ui_v2/src/agent_thread_pane.rs` | Add `set_thread_view()` method to display existing view |
| `agent_ui_v2/src/thread_history.rs` | Remove `impl Render`, add `AcpThreadHistoryComponent` (RenderOnce), add `TimeBucket::Running`, spinner rendering |
| `acp_thread/src/acp_thread.rs` | Add `AcpThreadEvent::Started` event, emit in `run_turn()` |
| `agent_ui/src/acp/thread_view.rs` | Expose `thread()` method if not already public, forward `Started` event |

## Testing Considerations

### Core Functionality
1. Start thread A, switch to thread B, verify A continues running (check network requests)
2. Start thread A, close pane with X button, verify A is cancelled
3. Start thread A, click stop button, verify A is cancelled
4. Start thread A, switch to B, switch back to A, verify A's state is preserved and view reconnects

### UI Indicators
5. Start multiple threads, verify all show spinners in history under "Running" section
6. Thread completes while viewing different thread, verify spinner removed and thread moves to time bucket
7. Error during generation, verify thread removed from "Running" section

### Edge Cases
8. Delete a running thread from history - verify it gets cancelled
9. Start thread, minimize pane (not close), switch to another thread - verify original continues
10. Rapid switching between running threads - verify no race conditions or dropped views
11. Thread completes exactly as user switches to it - verify clean state transition

### Notifications
12. Running thread in background completes - verify notification appears (already implemented via `AcpThreadView`)
13. Running thread in background needs tool authorization - verify notification appears
