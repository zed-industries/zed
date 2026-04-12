# Nested Scroll Issue in GPUI — Deep Analysis

## Issue Summary

**GitHub Issue**: [#40623](https://github.com/zed-industries/zed/issues/40623) — Horizontal scroll with trackpad does not prevent vertical scroll on macOS.

When scrolling horizontally over a code block inside the AI Chat panel, the parent vertical list also scrolls. One physical trackpad gesture results in **two** independent scroll mutations.

---

## How Nested Scrolling Works in GPUI

### 1. Mouse Event Dispatch Architecture

All mouse events, including `ScrollWheelEvent`, are dispatched via a single flat list of listeners.

**File**: [window.rs:4015-4050](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/window.rs#L4015-L4050)

```rust
fn dispatch_mouse_event(&mut self, event: &dyn Any, cx: &mut App) {
    let mut mouse_listeners = mem::take(&mut self.rendered_frame.mouse_listeners);

    // Capture phase — front to back
    for listener in &mut mouse_listeners {
        listener(event, DispatchPhase::Capture, self, cx);
        if !cx.propagate_event { break; }
    }

    // Bubble phase — back to front (innermost first)
    if cx.propagate_event {
        for listener in mouse_listeners.iter_mut().rev() {
            listener(event, DispatchPhase::Bubble, self, cx);
            if !cx.propagate_event { break; }
        }
    }
}
```

> [!IMPORTANT]
> **Key architectural fact**: Mouse listeners are stored in a **flat `Vec`**, not a tree. During the bubble phase, they are iterated in **reverse order** (last-registered = innermost element = processed first). Propagation only stops if a listener calls `cx.stop_propagation()`.

### 2. The `should_handle_scroll` Guard

All scroll listeners use `hitbox.should_handle_scroll(window)` to decide whether they should handle the event.

**File**: [window.rs:582-584](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/window.rs#L582-L584)

```rust
pub fn should_handle_scroll(self, window: &Window) -> bool {
    window.mouse_hit_test.ids.contains(&self)
}
```

The hit test collects **all** hitbox IDs under the mouse position, walking from front to back:

**File**: [window.rs:845-867](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/window.rs#L845-L867)

```rust
pub(crate) fn hit_test(&self, position: Point<Pixels>) -> HitTest {
    let mut hit_test = HitTest::default();
    for hitbox in self.hitboxes.iter().rev() {
        let bounds = hitbox.bounds.intersect(&hitbox.content_mask.bounds);
        if bounds.contains(&position) {
            hit_test.ids.push(hitbox.id);
            // Only stops early for BlockMouse behavior
            if hitbox.behavior == HitboxBehavior::BlockMouse { break; }
        }
    }
    hit_test
}
```

> [!WARNING]
> `should_handle_scroll` returns `true` for **every** scrollable element under the mouse — it does **not** mean "topmost scrollable only". If a code block div and its parent List both contain the mouse, **both** return `true` from `should_handle_scroll`.

---

## The Three Scroll Listener Implementations

### A. `Interactivity::paint_scroll_listener` (used by `div` and `UniformList`)

**File**: [div.rs:2703-2753](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/div.rs#L2703-L2753)

```rust
fn paint_scroll_listener(&self, hitbox: &Hitbox, style: &Style, window: &mut Window, _cx: &mut App) {
    if let Some(scroll_offset) = self.scroll_offset.clone() {
        // ...
        window.on_mouse_event(move |event: &ScrollWheelEvent, phase, window, cx| {
            if phase == DispatchPhase::Bubble && hitbox.should_handle_scroll(window) {
                let mut scroll_offset = scroll_offset.borrow_mut();
                let old_scroll_offset = *scroll_offset;
                let delta = event.delta.pixel_delta(line_height);

                // Compute delta_x and delta_y based on overflow settings...
                // Apply restrict_scroll_to_axis and allow_concurrent_scroll...

                scroll_offset.y += delta_y;
                scroll_offset.x += delta_x;
                if *scroll_offset != old_scroll_offset {
                    cx.notify(current_view);
                    // ⚠️ DOES NOT call cx.stop_propagation()
                }
            }
        });
    }
}
```

> [!CAUTION]
> **No `cx.stop_propagation()` call.** Even when it successfully applies a scroll delta, propagation continues to the next listener.

This is the listener used by:
- **Div scroll containers** (`.overflow_x_scroll()`, `.overflow_y_scroll()`)
- **UniformList** — which delegates to `Interactivity` for scroll handling

### B. `List::paint` scroll listener

**File**: [list.rs:1148-1169](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/list.rs#L1148-L1169)

```rust
window.on_mouse_event(move |event: &ScrollWheelEvent, phase, window, cx| {
    if phase == DispatchPhase::Bubble && hitbox_id.should_handle_scroll(window) {
        accumulated_scroll_delta = accumulated_scroll_delta.coalesce(event.delta);
        let mut pixel_delta = accumulated_scroll_delta.pixel_delta(px(20.));
        // Your previous fix was here ↓
        if pixel_delta.x.abs() > pixel_delta.y.abs() {
            // zero out y...
        }
        list_state.0.borrow_mut().scroll(
            &scroll_top, height, pixel_delta, current_view, window, cx,
        )
        // ⚠️ DOES NOT call cx.stop_propagation()
    }
});
```

`List` has its **own completely separate** scroll handler that does **not** go through `Interactivity::paint_scroll_listener`. It handles scrolling independently and also does **not** stop propagation.

### C. `UniformList` — delegates to `Interactivity`

Unlike `List`, `UniformList` does **not** have a custom scroll listener. It uses `Interactivity::paint` which calls `paint_scroll_listener` (variant A above). This means `UniformList` would have the **exact same issue** if nested — confirming the maintainer's point.

---

## The Exact Nesting in the Agent Panel

The element hierarchy when the mouse is over a code block in the AI chat:

```
┌──────────────────────────────────────────────────┐
│ ThreadView                                        │
│  ├── List (self.list_state)                      │  ← Outer vertical scroller
│  │    ├── [entry 0] UserMessage                  │     (List has custom scroll listener)
│  │    ├── [entry 1] AssistantMessage              │
│  │    │    ├── MarkdownElement                    │
│  │    │    │    ├── code_block parent_container   │  ← Has custom_scrollbars
│  │    │    │    │    ├── code_block div           │  ← Inner horizontal scroller
│  │    │    │    │    │    overflow_x: Scroll       │     (div paint_scroll_listener)
│  │    │    │    │    │    restrict_scroll_to_axis  │     = true
│  │    │    │    │    │    overflow_y: Visible      │
│  │    │    │    │    └── (code text content)       │
│  │    │    │    └──                                │
│  │    │    └──                                    │
│  │    └──                                        │
│  └──                                              │
└──────────────────────────────────────────────────┘
```

**Source references**:
- Outer List: [thread_view.rs:3702-3715](file:///Users/prayanshchhablani/work/zed/crates/agent_ui/src/connection_view/thread_view.rs#L3702-L3715)
- Inner code block div: [markdown.rs:1176-1190](file:///Users/prayanshchhablani/work/zed/crates/markdown/src/markdown.rs#L1176-L1190)

The markdown code block sets:
```rust
code_block.style().restrict_scroll_to_axis = Some(true);
code_block.flex().overflow_x_scroll().track_scroll(scroll_handle)
```

This `restrict_scroll_to_axis` only affects **how the div interprets the delta for itself** — it does not stop the event from reaching the parent.

---

## What Happens on a Horizontal Trackpad Gesture

1. macOS generates a `ScrollWheelEvent` with delta like `{x: -15.0, y: 2.0}` (mostly horizontal, with slight vertical component — trackpads always produce mixed deltas).

2. The event enters `dispatch_mouse_event`. `propagate_event = true`.

3. **Bubble phase** starts iterating listeners in reverse (innermost first).

4. **Inner code block div listener fires** (via `paint_scroll_listener`):
   - `should_handle_scroll` → true (mouse is over the code block)
   - `overflow.x == Scroll`, so `delta_x = delta.x` (horizontal delta applied)
   - `overflow.y != Scroll`, so `delta_y = 0` (vertical component correctly ignored)
   - `restrict_scroll_to_axis = true` — prevents vertical-to-horizontal rerouting
   - Scroll offset updated → `cx.notify()` called
   - **`cx.stop_propagation()` NOT called** → propagation continues ✅❌

5. **Outer List listener fires** (its own custom `on_mouse_event` in `List::paint`):
   - `should_handle_scroll` → true (mouse is also over the List)
   - `pixel_delta.y` derived from `event.delta.y` → non-zero (the 2.0px vertical component)
   - `list_state.scroll(...)` called → vertical scroll position changes
   - `cx.notify()` called
   - **`cx.stop_propagation()` NOT called**

6. **Result**: Code block scrolls horizontally ✓, List scrolls vertically ✗ (unwanted)

---

## 5 Reasons Why This Issue Occurs

### Reason 1: No Stop-Propagation After Scroll Consumption

**The fundamental architectural gap.** Neither `paint_scroll_listener` nor `List::paint`'s scroll handler calls `cx.stop_propagation()` after consuming a scroll event. In web browsers, when a scrollable element consumes a `wheel` event, it does not propagate to parent scrollables. GPUI has no equivalent behavior.

**Why this was likely deliberate**: Stopping propagation naively would break cases where an inner element has `overflow: visible` but a parent has `overflow: scroll` — the parent needs to see the event. The problem is that there's no mechanism to distinguish "I consumed this delta" from "I didn't care about this delta."

### Reason 2: Flat Listener List, No Parent-Child Awareness

Unlike the DOM's tree-based event bubbling, GPUI's mouse listeners are stored in a **flat Vec**. During dispatch, there is no concept of "this listener belongs to a child of that listener." The only relationship is registration order. This makes it impossible to implement "if a child scrollable consumed the event, skip ancestor scrollables" without an explicit side-channel.

### Reason 3: `should_handle_scroll` Is Inclusion-Based, Not Exclusion-Based

`should_handle_scroll` checks `window.mouse_hit_test.ids.contains(&self)`. This means **every** scrollable element whose hitbox contains the mouse position will independently decide to handle the event. There's no concept of a "topmost scrollable" or "most specific scrollable" that gets priority.

In contrast, web browsers have a specific algorithm: scroll events are delivered to the **nearest scrollable ancestor** of the target element, and if it can't scroll further in the requested direction, it propagates upward (a behavior called "scroll chaining").

### Reason 4: Mixed Delta Components from Trackpads

macOS trackpads generate scroll events with **both** x and y components simultaneously (e.g., a mostly horizontal gesture might produce `{x: -15, y: 2}`). The inner code block correctly handles this by only applying the x component (due to `overflow_x: Scroll` + `restrict_scroll_to_axis`), but the residual y component still reaches the outer List, which applies it as vertical scroll.

The `restrict_scroll_to_axis` style only governs **how that specific element interprets the delta** — it doesn't filter the event's delta for downstream listeners.

### Reason 5: `List` Does Not Use `Interactivity` for Scroll — No Shared Infrastructure

`List` implements its own separate scroll listener in its `paint` method, completely bypassing `Interactivity::paint_scroll_listener`. This means any fix applied to `paint_scroll_listener` (like adding propagation control) would **not** automatically fix `List`. Similarly, `UniformList` delegates to `Interactivity`, so changes there wouldn't propagate to `List` either.

This is exactly what the maintainer meant: the fix needs to be **in the scroll listener infrastructure within GPUI**, not in individual list implementations. Both `List` and `UniformList` (and plain div scrollables) need a consistent approach.

---

## What the Maintainer Means by "Fix Within GPUI"

The maintainer is saying that the correct fix is one of:

1. **Scroll event consumption tracking** — When a scroll listener actually applies a delta (moves its scroll offset), it should signal that the event has been consumed, preventing ancestor scrollables from also processing it. This could be done via:
   - `cx.stop_propagation()` (but needs care to handle the cases where an element can't scroll further)
   - A new mechanism like `event.consumed = true` on the scroll event itself
   - A scroll-specific side-channel on `Window` or `App`

2. **Scroll chaining** (web-like behavior) — Only propagate the scroll event to a parent scrollable if the child has reached its scroll boundary (can't scroll further in the requested direction). This is the sophisticated approach browsers use.

3. **Delta filtering** — When an inner element consumes part of the delta (e.g., the x component), the remaining delta seen by parent listeners should have that component zeroed out. This prevents "double-dipping."

All of these approaches need to work across `div` scroll listeners, `List`, and `UniformList` — hence the maintainer's emphasis on fixing it "within GPUI" rather than per-component.

---

## The Exact Code Causing the Problem

### Primary: No propagation stop after scroll consumption

**Code block div** — [div.rs:2748-2750](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/div.rs#L2748-L2750):
```rust
if *scroll_offset != old_scroll_offset {
    cx.notify(current_view);
    // ← Missing: cx.stop_propagation() or equivalent
}
```

**List** — [list.rs:1160-1167](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/list.rs#L1160-L1167):
```rust
list_state.0.borrow_mut().scroll(
    &scroll_top, height, pixel_delta, current_view, window, cx,
)
// ← Missing: cx.stop_propagation() or equivalent
```

### Secondary: Both listeners pass `should_handle_scroll` independently

**Code block div** — [div.rs:2718](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/div.rs#L2718):
```rust
if phase == DispatchPhase::Bubble && hitbox.should_handle_scroll(window) {
```

**List** — [list.rs:1150](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/list.rs#L1150):
```rust
if phase == DispatchPhase::Bubble && hitbox_id.should_handle_scroll(window) {
```

Both conditions are true simultaneously for nested scrollables.

---

## Debug Logging to Verify

Add these `eprintln!` statements to confirm the theory. When you horizontally scroll over a code block in the agent panel, you should see both inner and outer listeners processing the same event.

### Log 1: Dispatch Order

**File**: [window.rs](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/window.rs), around line 4043, inside the bubble loop:

```rust
// Before the listener call:
if event.is::<crate::ScrollWheelEvent>() {
    eprintln!(
        "[scroll-dispatch] about to call bubble listener #{}, propagate={}",
        mouse_listeners.len() - 1 - i, // approximate index for identification
        cx.propagate_event
    );
}
```

> You'd need to change the `for` to enumerate. Alternatively, just log inside each handler below.

### Log 2: Div Scroll Listener

**File**: [div.rs](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/div.rs), line ~2718, inside the `if phase == Bubble && should_handle_scroll` block:

```rust
eprintln!(
    "[div-scroll] hitbox={:?} overflow=({:?},{:?}) restrict={} delta=({:?},{:?}) applied=({:?},{:?})",
    hitbox.id,
    overflow.x, overflow.y,
    restrict_scroll_to_axis,
    delta.x, delta.y,
    delta_x, delta_y,
);
```

Place this **after** the delta_x/delta_y computation but **before** the offset mutation.

**Expected**: When mouse is over a code block:
- `overflow=(Scroll, Visible)`, `restrict=true`
- `applied=(non-zero, 0.0)` — only horizontal scroll applied

### Log 3: List Scroll Listener  

**File**: [list.rs](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/list.rs), line ~1150, inside the `if phase == Bubble && should_handle_scroll` block:

```rust
eprintln!(
    "[list-scroll] hitbox={:?} event_delta={:?} pixel_delta=({:?},{:?})",
    hitbox_id,
    event.delta,
    pixel_delta.x, pixel_delta.y,
);
```

**Expected**: On the **same** gesture event, this also fires with `pixel_delta.y` being non-zero, confirming the double-consumption.

### Log 4: List Scroll State Change

**File**: [list.rs](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/list.rs), line ~549, at the top of `fn scroll(...)`:

```rust
eprintln!(
    "[list-scroll-state] delta=({:?},{:?}) old_top={:?}",
    delta.x, delta.y,
    self.scroll_top(scroll_top),
);
```

**Expected**: `delta.y` is non-zero, showing the list is being scrolled vertically by the same gesture.

### What Confirms the Theory

For **one** horizontal trackpad gesture over a code block in the agent thread panel:

1. `[div-scroll]` logs with `applied=(some_x, 0.0)` — inner code block scrolls only horizontally ✓
2. `[list-scroll]` logs for the **same gesture** — outer list is also processing it
3. `[list-scroll-state]` shows `delta.y` is non-zero — outer list scrolls vertically ✗
4. No `[scroll-dispatch]` log shows `propagate=false` between the two — nothing stopped propagation

---

## How Other Scroll Listeners Compare

| Element | Scroll Handler | Uses `Interactivity`? | Stops Propagation? |
|---------|---------------|----------------------|-------------------|
| `div` (overflow scroll) | `paint_scroll_listener` | Yes | ❌ No |
| `List` | Custom in `paint()` | No | ❌ No |
| `UniformList` | `paint_scroll_listener` via `Interactivity` | Yes | ❌ No |
| Drop listeners | Custom | N/A | ✅ Yes (`cx.stop_propagation()`) |
| Click listeners | Custom | N/A | Sometimes |

Notice that **drop handlers** do call `cx.stop_propagation()` at [div.rs:2390](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/div.rs#L2390), showing the mechanism exists and is used for other event types. Scroll is the outlier.

---

## Scope of Impact

This bug affects **any** nesting of scrollable elements in Zed, not just the agent panel. Examples:

- Code blocks in markdown preview panels
- Any `UniformList` containing horizontally scrollable items
- Editor views with horizontal scroll inside scrollable panels
- Settings UI with scrollable sections containing code examples

The maintainer correctly identified that fixing this in `List` alone misses `UniformList` and div-based scrollables.
