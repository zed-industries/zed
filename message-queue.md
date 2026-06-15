# Message Queue Refactor Plan

This document describes how to re-apply the message queue improvements that were
previously explored and lost with a discarded branch. It reflects the codebase
state as of the last survey — re-verify line numbers before editing, as this
area changes frequently.

## Motivation

The message queue (follow-up messages typed while the agent is generating) has
two classes of problems:

1. **Stale-index bugs ("train wreck" states).** Queue state is spread across
   parallel arrays on `ThreadView`, coordinated by positional indices. Editor
   event subscriptions capture indices by value, so removing an entry shifts
   the data while closures still point at old positions. Editors are synced
   lazily from `ConversationView::render()`, creating frames where data and UI
   disagree.

2. **The "frozen queue" bug.** Reproduction:
   - Send a message; agent starts generating
   - Send another message; it gets queued
   - Stop the agent → queued message isn't sent (correct)
   - Send something else
   - When the agent stops, the queued message is never picked up — the queue
     is permanently frozen (incorrect)

   Root cause: stopping sets `user_interrupted_generation = true`, which is
   consumed by the *next* Stopped event — but sending a new message generates
   a *new* turn whose Stopped event consumes the flag without re-enabling
   auto-processing... depending on ordering. The loose boolean/counter flags
   (`skip_queue_processing_count`, `user_interrupted_generation`) make the
   state transitions impossible to reason about. Nothing explicitly "resumes"
   the queue when the user re-engages.

## Current State (what you're starting from)

### `crates/agent_ui/src/conversation_view/thread_view.rs`

Queue state is loose `pub` fields on `ThreadView` (~L589–604):

```rust
pub local_queued_messages: Vec<QueuedMessage>,
pub queued_message_editors: Vec<Entity<MessageEditor>>,
pub queued_message_editor_subscriptions: Vec<Subscription>,
pub last_synced_queue_length: usize,
// ...
pub skip_queue_processing_count: usize,
pub user_interrupted_generation: bool,
pub can_fast_track_queue: bool,
```

Queue methods (all index-based):
- `has_queued_messages` (~L1224)
- `queue_message` (~L1951) — entry point when user sends while generating
- `add_to_queue` (~L1986) — pushes a `QueuedMessage` (data only, no editor)
- `remove_from_queue(index)` (~L1999)
- `sync_queue_flag_to_native_thread` (~L2013)
- `send_queued_message_at_index(index, is_send_now)` (~L2022) — uses
  `skip_queue_processing_count += 1` when interrupting a generation
- `move_queued_message_to_main_editor(index, attempt: Option<InputAttempt>, cursor_offset)` (~L2071)
  — NOTE: signature changed since the original refactor; it now takes
  `Option<InputAttempt>` instead of `Option<&str>`
- `clear_queue` (~L3320)
- `render_message_queue_summary` (~L3273), `render_message_queue_entries` (~L4058)
- `cancel_generation` (~L1862) — sets `user_interrupted_generation = true`
- `stop_current_and_send_new_message` (~L1727) — resets the counter, sets the flag
- Action handlers in `impl Render` (~L10396): `SendNextQueuedMessage`,
  `RemoveFirstQueuedMessage`, `EditFirstQueuedMessage`, `ClearMessageQueue`
  (the last one duplicates `clear_queue` inline)

### `crates/agent_ui/src/conversation_view.rs`

- `QueuedMessage` struct (~L112): `{ content, tracked_buffers }`
- `sync_queued_message_editors` (~L2471–2581) — called at the top of
  `ConversationView::render()` (~L3134). Lazily reconciles
  `queued_message_editors` against `local_queued_messages` by count. Creates
  read-only `MessageEditor`s and subscribes with **index captured by value**
  (this is the core stale-index bug).
- Helpers: `queued_messages_len` (~L2404), `update_queued_message` (~L2410),
  `queued_message_contents` (~L2433), `save_queued_message_at_index` (~L2445)
- Delegation wrappers: `send_queued_message_at_index` (~L1472),
  `move_queued_message_to_main_editor` (~L1486)
- `AcpThreadEvent::Stopped` handler (~L1608–1649): the
  `skip_queue_processing_count` / `user_interrupted_generation` decision logic.
  NOTE: `notify_with_sound` is now conditional on `!should_send_queued` —
  preserve this behavior.
- Tests (index-based): `test_move_queued_message_to_empty_main_editor` (~L8542),
  `test_move_queued_message_to_non_empty_main_editor` (~L8579),
  `test_paste_text_into_queued_message_promotes_to_main_editor` (~L8624) and an
  image-paste variant (~L8639),
  `test_no_notification_when_queued_message_will_be_auto_sent` (~L3609)

### Other touch points (no changes needed, but be aware)

- `crates/agent_ui/src/agent_ui.rs` (~L258): action definitions
- `crates/agent/src/thread.rs`: `has_queued_message` flag + turn-boundary check
  in `run_turn_internal` (~L2561). `sync_queue_flag_to_native_thread` feeds this.
- `crates/agent/src/tests/mod.rs`: `test_queued_message_ends_turn_at_boundary`
- Keymaps reference the four queue actions

## Target Design

### New module: `thread_view/message_queue.rs`

This requires converting `thread_view.rs` into a directory module **or**
declaring the module from within `thread_view.rs` if the file layout allows
(`mod message_queue;` + a `thread_view/` directory next to `thread_view.rs`
works in modern Rust without `mod.rs`).

The module encapsulates ALL queue state with private fields, exposing only
intent-based methods so flag bookkeeping can't be forgotten:

```rust
use std::collections::VecDeque;
use super::*;

/// Stable identifier for a queued message entry. Unlike positional indices,
/// these don't shift when entries are removed, so closures can safely capture
/// them without risk of operating on the wrong message.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct QueueEntryId(usize);

pub struct QueueEntry {
    pub id: QueueEntryId,
    pub content: Vec<acp::ContentBlock>,
    pub tracked_buffers: Vec<Entity<Buffer>>,
    pub editor: Entity<MessageEditor>,
    pub _subscription: Subscription,
}

/// Controls whether the queue auto-sends after generation completes.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum ProcessingState {
    /// Normal: auto-send next queued message when generation completes.
    AutoProcess,
    /// Queue is paused because the user stopped generation.
    Paused,
    /// A "Send Now" cancelled the current generation; we must absorb the
    /// Stopped event from that cancellation before resuming auto-processing,
    /// otherwise the queue would double-send.
    AbsorbingCancel,
}

pub struct MessageQueue {
    entries: VecDeque<QueueEntry>,   // private!
    processing_state: ProcessingState,
    can_fast_track: bool,
    next_id: usize,
}
```

API surface:

| Method | Called when | Behavior |
|---|---|---|
| `is_empty()` / `len()` / `first()` / `first_id()` / `iter()` | queries | read-only access |
| `entry_by_id(id)` / `entry_by_id_mut(id)` | save-on-blur | find by stable ID |
| `next_id()` | before constructing a `QueueEntry` | allocates a stable ID (needed because the subscription closure must capture the ID before the entry exists) |
| `enqueue(entry)` | user sends while generating | push back + `AutoProcess` + `can_fast_track = true` |
| `remove(id)` | delete / edit | removes, no state changes |
| `clear()` | Clear All | empties entries + clears fast-track |
| `try_fast_track()` | Enter on empty main editor | if flag set: clear flag, resume to `AutoProcess`, pop front |
| `on_generation_stopped(is_first_editor_focused)` | `Stopped` event | `AbsorbingCancel` → absorb + go `AutoProcess`, return None; `Paused` → None; `AutoProcess` → pop front unless first editor focused |
| `send_now(id, is_generating)` | Send Now button/keybinding | remove entry; if generating, set `AbsorbingCancel` |
| `pause()` | user hits stop/Escape | `Paused` |
| `resume()` | user sends a new message | `Paused` → `AutoProcess` (no-op otherwise). **This is the frozen-queue bug fix.** |

Design notes settled during the original exploration:

- `try_fast_track` works even when `Paused` — pressing Enter is an explicit
  user action, distinct from auto-processing.
- `enqueue` resumes auto-processing — queuing a message is active engagement.
- The enum variant was originally named `AwaitingInterruptedStop`; we agreed
  it was cryptic and settled on `AbsorbingCancel`.
- We discussed folding `add_to_queue` into `queue_message`; outcome was
  ambivalent. Keep them separate (async resolution vs. sync insertion) but
  consider clearer names, e.g. `enqueue_resolved_content` for the inner one.

### `ThreadView` changes

Replace the seven loose fields with:

```rust
pub message_queue: MessageQueue,
```

Method changes:

- `has_queued_messages` → `!self.message_queue.is_empty()`
- `send` fast-track section becomes:
  ```rust
  if is_editor_empty {
      if let Some(entry) = self.message_queue.try_fast_track() {
          self.dispatch_queued_entry(entry, window, cx);
          return;
      }
  }
  ```
- `queue_message`: unchanged flow, but drop `can_fast_track_queue = true`
  (handled inside `enqueue`)
- `add_to_queue`: now takes `window` (it creates the editor + subscription
  inline — this kills the lazy `sync_queued_message_editors` reconciliation).
  Body: allocate `next_id()`, create read-only `MessageEditor`, subscribe with
  the **ID** captured (`handle_queue_editor_event(id, ...)`), `enqueue`,
  `sync_queue_flag_to_native_thread`, `cx.notify()`.
- New `handle_queue_editor_event(id, event, window, cx)`: routes
  `InputAttempted` → `move_queued_message_to_main_editor`, `LostFocus` →
  `save_queued_message`, `Cancel`/`Send` → focus main editor,
  `SendImmediately` → `send_queued_message_now`.
  ⚠️ The current subscription handles `InputAttempted` with an
  `InputAttempt` payload — port the current event-matching code, not the old
  `Option<&str>` version.
- New `save_queued_message(id, cx)`: async-resolves the queue editor's
  contents and writes back via `entry_by_id_mut`. (Replaces
  `ConversationView::save_queued_message_at_index` + `update_queued_message`.)
- `send_queued_message_at_index` → replaced by:
  - `send_queued_message_now(id)`: `message_queue.send_now(id, is_generating)`
    then `dispatch_queued_entry`
  - `dispatch_queued_entry(entry)` (pub): the shared "actually send this
    entry" path — emit `Interacted`, focus main editor, cancel current
    generation, await it, re-follow if needed, `send_content`. Used by
    fast-track, auto-process, and send-now.
- `move_queued_message_to_main_editor`: take `QueueEntryId` instead of
  `usize`; use `message_queue.remove(id)`; keep the current
  `InputAttempt`/cursor handling; call `sync_queue_flag_to_native_thread`.
- `cancel_generation`: replace `user_interrupted_generation = true` with
  `self.message_queue.pause()`
- `stop_current_and_send_new_message`: replace the counter-reset + flag with
  `self.message_queue.pause()`
- `send_impl`: add `self.message_queue.resume()` next to the existing
  `thread_error.take()` / `editing_message.take()` lines. **Frozen-queue fix.**
- `clear_queue`: `message_queue.clear()` + flag sync + notify
- Render methods: iterate `self.message_queue.iter().enumerate()`, use
  `position` for visual concerns (is-next, borders, element IDs) and capture
  `entry.id` in every click closure
- Action handlers: use `first_id()` + ID-based methods; `ClearMessageQueue`
  should just call `clear_queue` (fixes the current inline duplication)

### `ConversationView` changes

- Delete: `QueuedMessage` struct, `sync_queued_message_editors`,
  `queued_messages_len`, `update_queued_message`, `queued_message_contents`,
  `save_queued_message_at_index`, both delegation wrappers, and the
  `sync_queued_message_editors` call in `render`
- `Stopped` handler queue section becomes:
  ```rust
  if let Some(active) = self.root_thread_view() {
      active.update(cx, |active, cx| {
          let is_first_editor_focused = active
              .message_queue
              .first()
              .is_some_and(|e| e.editor.focus_handle(cx).is_focused(window));
          if let Some(entry) = active
              .message_queue
              .on_generation_stopped(is_first_editor_focused)
          {
              active.dispatch_queued_entry(entry, window, cx);
          }
      });
  }
  ```
  ⚠️ Preserve the current `notify_with_sound` conditionality: the sound only
  plays when no queued message is being auto-sent. You may need
  `on_generation_stopped` to be called first and the notify branch driven by
  whether it returned an entry.

### Test updates

All queue tests currently use index-based APIs. Update them to:
- pass `window` to `add_to_queue`
- get IDs via `thread.message_queue.first_id().unwrap()`
- assert via `thread.message_queue.len()`

Affected tests in `conversation_view.rs`:
- `test_move_queued_message_to_empty_main_editor`
- `test_move_queued_message_to_non_empty_main_editor`
- `test_paste_text_into_queued_message_promotes_to_main_editor`
- the image-paste variant next to it
- `test_no_notification_when_queued_message_will_be_auto_sent`

Consider adding a regression test for the frozen-queue bug:
queue a message → cancel → send a new message → complete generation → assert
the queued message gets auto-sent.

## Suggested Implementation Order

1. Create `thread_view/message_queue.rs` with the full `MessageQueue` API
   (compiles standalone; nothing references it yet)
2. Add `mod message_queue; use message_queue::*;` to `thread_view.rs`
3. Swap `ThreadView` fields, fix the constructor
4. Rewire `ThreadView` methods top-to-bottom (the compiler will walk you
   through every stale reference since the old fields are gone)
5. Rewire `ConversationView` (Stopped handler, delete dead helpers, render)
6. Update tests
7. Validate: diagnostics, then
   `cargo test -p agent_ui queued` and
   `cargo test -p agent test_queued_message_ends_turn_at_boundary`

## Known Risks

- `render_message_queue_entries` is large; porting it from index-based to
  ID-based closures is mechanical but easy to fumble — every `on_click` must
  capture `entry_id`, while element IDs/visual logic keep using `position`.
- The `InputAttempt` payload in `MessageEditorEvent::InputAttempted` is newer
  than the original refactor; don't blindly restore old signatures.
- `dispatch_queued_entry` must be `pub` — it's called from
  `ConversationView`'s Stopped handler inside an `active.update` closure.
- The `agent` crate's turn-boundary behavior depends on
  `sync_queue_flag_to_native_thread` being called after every queue size
  change (enqueue, remove, clear, dispatch). Missing a call site means the
  native thread mis-detects whether a queued message exists.
