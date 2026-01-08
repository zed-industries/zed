# Telemetry Log View Revamp Plan

## Overview

This document outlines the plan to revamp the telemetry log view to make it more useful for project managers and developers who need to understand what telemetry data Zed is collecting and sending.

## Current State

### How Telemetry Works Today

1. **Event Creation**: Events are created throughout the codebase via `telemetry::event!()` macro (~51 unique event types)
2. **Event Flow**: Events are sent through an unbounded channel to `Telemetry::report_event()`
3. **Queuing**: Events are queued until:
   - Queue reaches max size (5 in debug, 50 in release)
   - Flush interval elapses (1 second in debug, 5 minutes in release)
4. **Logging**: On flush, events are written to `<logs_dir>/telemetry.log` as JSON (one `EventWrapper` per line)
5. **Sending**: Events are sent to the server in an `EventRequestBody`

### Current Telemetry Log View (`open_telemetry_log_file`)

Location: `crates/zed/src/zed.rs:1985-2050`

**Problems:**

- **No live updates**: Reads file once when opened, creates a local buffer
- **Raw JSON**: Hard to read, especially for non-developers
- **No filtering**: Shows all events, no way to filter by type
- **Missing context**: Log file only contains `EventWrapper` (no session/system metadata)
- **Truncation**: Only shows last 5MB of log file

## Goals

1. **Live updates**: Show new telemetry events as they happen
2. **Better formatting**: Human-readable display with collapsible JSON
3. **Filtering**: Filter by event type to focus on specific categories (e.g., Agent events)

## Proposed Solution

### Architecture

Create a new `TelemetryLogView` workspace item (similar to `AcpTools`) that subscribes to telemetry events in real-time.

```
┌────────────────────────────────────────────────────────────────────┐
│                     Telemetry Flow                                 │
├────────────────────────────────────────────────────────────────────┤
│                                                                    │
│  telemetry::event!() ──► mpsc channel ──► Telemetry::report_event()│
│                              │                    │                │
│                              │                    ▼                │
│                              │            events_queue             │
│                              │                    │                │
│                              │                    ▼                │
│                              │              flush_events()         │
│                              │                    │                │
│                              │          ┌────────┴────────┐        │
│                              │          ▼                 ▼        │
│                              │    telemetry.log      HTTP POST     │
│                              │                                     │
│                              ▼                                     │
│                    TelemetryLogView (NEW)                          │
│                    - Real-time display                             │
│                    - Filtering                                     │
│                    - Pretty formatting                             │
└────────────────────────────────────────────────────────────────────┘
```

### Chosen Approach: Broadcast Channel with Synchronized Initialization

Modify the telemetry system to support subscribers, with a careful synchronization strategy to ensure no events are lost when opening the view.

#### Synchronization Strategy

When the user opens the telemetry log view, we need to capture:

1. Historical events (already flushed to disk)
2. Queued events (in memory, not yet flushed)
3. Future events (arriving after we subscribe)

**The synchronization sequence:**

```
User opens telemetry log view
         │
         ▼
    ┌─────────────────────────────────────┐
    │       TAKE STATE LOCK               │
    ├─────────────────────────────────────┤
    │ 1. Read historical data from        │
    │    telemetry.log file               │
    │                                     │
    │ 2. Read out the unflushed           │
    │    events_queue (clone events)      │
    │                                     │
    │ 3. Hook up broadcast channel        │
    │    to receive live events           │
    ├─────────────────────────────────────┤
    │       DROP STATE LOCK               │
    └─────────────────────────────────────┘
         │
         ▼
    View is now synchronized
    - Has all historical events
    - Has all queued events
    - Will receive all future events
```

This lock-based approach ensures atomicity: no events can be added to the queue or flushed while we're setting up, guaranteeing we don't miss or duplicate any events.

#### Implementation

```rust
// In crates/client/src/telemetry.rs

struct TelemetryState {
    // ... existing fields ...
    settings: TelemetrySettings,
    events_queue: Vec<EventWrapper>,
    log_file: Option<File>,
    // ... etc ...

    /// Subscribers receiving live event updates (new field)
    subscribers: Vec<mpsc::UnboundedSender<EventWrapper>>,
}

impl Telemetry {
    /// Subscribe to telemetry events with full history.
    /// Returns historical events and a channel for live events.
    ///
    /// The state lock is held during this operation to ensure no events
    /// are lost between reading history and subscribing to live events.
    pub fn subscribe_with_history(
        self: &Arc<Self>,
    ) -> (Vec<EventWrapper>, mpsc::UnboundedReceiver<EventWrapper>) {
        let mut state = self.state.lock();

        // 1. Read historical events from log file
        let historical = Self::read_log_file();

        // 2. Clone the unflushed queue
        let queued: Vec<EventWrapper> = state.events_queue.clone();

        // 3. Set up broadcast channel (stored on state, not static)
        let (tx, rx) = mpsc::unbounded();
        state.subscribers.push(tx);

        // Combine historical + queued
        let mut all_events = historical;
        all_events.extend(queued);

        (all_events, rx)
    }

    fn read_log_file() -> Vec<EventWrapper> {
        let path = Self::log_file_path();
        // Read last 5MB of file (same limit as current implementation)
        // Parse each line as EventWrapper JSON
        // ... implementation details ...
    }

    // Modified report_event to broadcast to subscribers
    fn report_event(self: &Arc<Self>, event: Event) {
        let mut state = self.state.lock();

        // ... existing queue logic ...

        // Broadcast to subscribers (accessing field on state)
        state.subscribers.retain(|tx| {
            tx.unbounded_send(event_wrapper.clone()).is_ok()
        });
    }
}
```

## Implementation Plan

### Phase 1: Core Infrastructure

**Files to create:**

- `crates/zed/src/telemetry_log.rs` - View implementation in the zed crate

**Files to modify:**

- `crates/client/src/telemetry.rs` - Add subscriber support and broadcast mechanism
- `crates/zed/src/zed.rs` - Register new action and view, replace existing `open_telemetry_log_file`

**Tasks:**

1. Add `subscribers` field to `TelemetryState` and broadcast mechanism
2. Implement `subscribe_with_history` on `Telemetry`
3. Create `TelemetryLogView` in zed crate

### Phase 2: View Implementation

**Reference:** `crates/acp_tools/src/acp_tools.rs`

**Components:**

1. `TelemetryLogView` - Main view struct implementing `Item`, `Render`, `Focusable`
2. `TelemetryLogToolbarItemView` - Toolbar with filter controls
3. `TelemetryLogEntry` - Individual event display

**Key features to implement:**

```rust
const MAX_EVENTS: usize = 10_000;

struct TelemetryLogView {
    focus_handle: FocusHandle,
    events: VecDeque<TelemetryLogEntry>,  // Bounded to MAX_EVENTS
    list_state: ListState,
    expanded: HashSet<usize>,
    search_query: String,                  // Text search filter
    _subscription: Task<()>,
}

struct TelemetryLogEntry {
    received_at: Instant,                  // For "4s ago" display
    event_type: SharedString,
    event_properties: HashMap<String, serde_json::Value>,
    signed_in: bool,
    collapsed_md: Option<Entity<Markdown>>,
    expanded_md: Option<Entity<Markdown>>,
}
```

### Phase 3: Filtering UI

**Toolbar components:**

1. **Search input** - Text search within event type and properties
2. **Clear button** - Clear displayed events
3. **Open log file button** - Open the raw `telemetry.log` file

### Phase 4: Polish & Integration

1. **Keyboard shortcuts**: Add keybinding for opening telemetry log

## Data Model

### TelemetryLogEntry (displayed in view)

```rust
pub struct TelemetryLogEntry {
    /// When the event was received (local time)
    pub received_at: DateTime<Utc>,

    /// The event type name (e.g., "Agent Message Sent")
    pub event_type: String,

    /// Event properties as key-value pairs
    pub properties: HashMap<String, serde_json::Value>,

    /// Whether user was signed in when event fired
    pub signed_in: bool,
}
```

### Display Format

**Timestamps:** Show relative time (e.g., "4s ago") with exact timestamp in tooltip on hover.

**Collapsed view (one line per event):**

```
▼ 4s ago    Agent Message Sent
  { agent: "claude-code", session: "abc123", message_count: 5 }

▶ 6s ago    Editor Edited
```

**Expanded view (click to expand):**

```
▼ 4s ago    Agent Message Sent
  {
    "agent": "claude-code",
    "session": "abc123",
    "message_count": 5,
    "thread_id": "thread_xyz..."
  }
```

## File Structure

```
crates/
├── client/
│   └── src/
│       └── telemetry.rs          # Add subscribers field and subscribe_with_history()
│
└── zed/
    └── src/
        ├── zed.rs                # Register OpenTelemetryLog action (replace existing)
        └── telemetry_log.rs      # NEW: View implementation
```

## Testing Strategy

1. **Unit tests**: Test search filtering logic, event parsing
2. **Integration tests**: Test subscription mechanism
3. **Manual testing**:
   - Open view, trigger various actions, verify events appear
   - Test search filtering
   - Test with high event volume (rapid actions)
