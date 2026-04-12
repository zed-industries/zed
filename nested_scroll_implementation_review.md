# Review of `implementation_plan.md` and Replacement Plan

## Bottom line

I do **not** agree with the full implementation plan as written.

My verdict on the three options is:

- **Option 1**: reject
- **Option 2**: directionally right, but incomplete and too blunt in `List`
- **Option 3**: reject for this bug

My recommended fix is:

> add a GPUI-internal "built-in scroll claimed" state for the current `ScrollWheelEvent`, make `div`/`UniformList` and `List` both respect it, and only claim the event when the element actually changed its scroll position.

That fixes the issue across GPUI without suppressing user `.on_scroll_wheel(...)` callbacks.

---

## Why I reject Option 1

Option 1 says: call `cx.stop_propagation()` after a scrollable moves.

I do not think that is the right GPUI fix.

### What is good about it

- very small diff
- proves the theory quickly
- would likely fix the agent-panel code-block case

### Why I still reject it

It is too broad. `cx.stop_propagation()` stops every later mouse listener, not only later built-in scroll handlers.

That means it risks breaking:

- parent `.on_scroll_wheel(...)` listeners that want to observe scroll events
- custom wheel behavior layered on top of built-in scrolling
- future nested interactions that are not themselves built-in scroll containers

This is especially important in this repo because GPUI exposes `.on_scroll_wheel(...)` directly, and several crates use it.

So Option 1 is a good debug experiment, but not a good architectural fix.

---

## Why Option 2 is close, but needs to be changed

Option 2 is the right overall direction because it separates:

- built-in scroll consumption
- user event propagation

That is the correct split.

### What Option 2 gets right

1. The state should live at the `Window` dispatch level, because the problem exists across multiple built-in scroll implementations.
2. `div`/`UniformList` and `List` both need to participate.
3. User `.on_scroll_wheel(...)` listeners should still receive the event.

### What Option 2 gets wrong

The problem is in the `List` part of the plan.

The plan says `List` should set `window.scroll_event_handled = true` after calling:

```rust
list_state.0.borrow_mut().scroll(...)
window.scroll_event_handled = true;
```

That is too blunt because `StateInner::scroll(...)` currently returns `()`, not "did scroll."

So with the plan as written, `List` would mark the event as handled even when:

- the list was already at its boundary
- `delta.y` was effectively zero
- clamping prevented any actual position change

That would break the fallback behavior where a parent scrollable should still get the event if the child could not move.

### What must change

`List::scroll` needs to report whether its scroll position actually changed.

Without that, Option 2 will create false positives.

---

## Why I reject Option 3

Option 3 proposes axis-by-axis consumption.

I do not think that matches the semantics needed for this bug.

### Why it looks attractive

It sounds more precise:

- child consumes `x`
- parent still gets `y`

### Why that is wrong here

That is basically the opposite of what the user is complaining about.

In this issue, the horizontal code block should "win" the gesture once it successfully scrolls. If the child consumes only `x` and leaves `y` free, the outer vertical list will still move a little on the same gesture, which is the bug we are trying to remove.

So axis-by-axis consumption would preserve the exact macOS trackpad failure mode:

- child code block scrolls horizontally
- parent list still consumes the small vertical component

That makes Option 3 a poor fit for this issue.

---

## Replacement plan

## Goal

Ensure that for one `ScrollWheelEvent`, once an inner **built-in** scroll container actually changes its scroll position, later built-in scroll containers skip built-in scrolling for that same event.

At the same time:

- user `.on_scroll_wheel(...)` callbacks should still run
- if the inner scrollable cannot actually move, the parent should still be allowed to scroll

---

## Proposed behavior

For each `ScrollWheelEvent`:

1. Reset a `Window`-local built-in scroll claim state.
2. Let listeners bubble as they do today.
3. The first built-in scroll handler that actually changes its offset claims the event.
4. Later built-in scroll handlers skip their own built-in scrolling if the event is already claimed.
5. User wheel listeners still run because propagation is not stopped.

This is intentionally **event-level** claiming, not axis-level claiming.

That matches the desired UX for horizontal code blocks on a trackpad.

---

## Files to change

- `crates/gpui/src/window.rs`
- `crates/gpui/src/elements/div.rs`
- `crates/gpui/src/elements/list.rs`

`UniformList` should not need its own custom patch because it already goes through `Interactivity` and therefore inherits the `div.rs` fix.

---

## Detailed implementation, line by line

### 1. `crates/gpui/src/window.rs`

Add a small private state field to `Window`.

Suggested shape:

```rust
#[derive(Default, Clone, Copy)]
struct BuiltInScrollDispatchState {
    claimed: bool,
}
```

Add it as a private field on `Window`:

```rust
built_in_scroll_dispatch: BuiltInScrollDispatchState,
```

I prefer this over a raw `pub(crate) bool` because:

- it keeps the state semantically named
- it stays private to `window.rs` unless we intentionally expose helpers
- it leaves room to expand later if GPUI eventually wants richer chaining semantics

Initialize it in the `Window` constructor:

```rust
built_in_scroll_dispatch: BuiltInScrollDispatchState::default(),
```

Reset it at the beginning of `dispatch_mouse_event` for wheel events:

```rust
if event.is::<crate::ScrollWheelEvent>() {
    self.built_in_scroll_dispatch = BuiltInScrollDispatchState::default();
}
```

Then add two tiny `pub(crate)` helper methods on `Window`:

```rust
pub(crate) fn built_in_scroll_already_claimed(&self) -> bool {
    self.built_in_scroll_dispatch.claimed
}

pub(crate) fn claim_built_in_scroll(&mut self) {
    self.built_in_scroll_dispatch.claimed = true;
}
```

### Why this helps

- the state is per-event
- the state is shared across all built-in scroll handlers
- user callbacks do not need to know about it

---

### 2. `crates/gpui/src/elements/div.rs`

Change the built-in scroll listener guard from:

```rust
if phase == DispatchPhase::Bubble && hitbox.should_handle_scroll(window) {
```

to:

```rust
if phase == DispatchPhase::Bubble
    && !window.built_in_scroll_already_claimed()
    && hitbox.should_handle_scroll(window)
{
```

Then keep the existing delta computation exactly as-is.

After this block:

```rust
scroll_offset.y += delta_y;
scroll_offset.x += delta_x;
if *scroll_offset != old_scroll_offset {
    cx.notify(current_view);
}
```

change it to:

```rust
scroll_offset.y += delta_y;
scroll_offset.x += delta_x;
if *scroll_offset != old_scroll_offset {
    window.claim_built_in_scroll();
    cx.notify(current_view);
}
```

### Why this helps

Line by line:

- `!window.built_in_scroll_already_claimed()`
  prevents later built-in scroll containers from re-handling the same event

- `if *scroll_offset != old_scroll_offset`
  means the element only claims the event if it genuinely moved

- `window.claim_built_in_scroll()`
  makes the claim visible to later built-in scroll handlers in the same bubble pass

Because `UniformList` uses this path through `Interactivity`, it gets the fix automatically.

---

### 3. `crates/gpui/src/elements/list.rs`

This file needs two changes.

#### 3a. Remove the local branch-specific axis hack

The current branch contains:

```rust
if pixel_delta.x.abs() > pixel_delta.y.abs() {
    accumulated_scroll_delta = match accumulated_scroll_delta {
        ScrollDelta::Pixels(p) => ScrollDelta::Pixels(point(p.x, px(0.))),
        ScrollDelta::Lines(p) => ScrollDelta::Lines(point(p.x, 0.)),
    };
    pixel_delta.y = px(0.);
}
```

I would remove that when implementing the GPUI fix, because the shared claim mechanism is the real fix.

#### 3b. Make `List` participate in built-in scroll claiming

First, change the bubble guard from:

```rust
if phase == DispatchPhase::Bubble && hitbox_id.should_handle_scroll(window) {
```

to:

```rust
if phase == DispatchPhase::Bubble
    && !window.built_in_scroll_already_claimed()
    && hitbox_id.should_handle_scroll(window)
{
```

Then change `StateInner::scroll(...)` so it returns `bool`:

```rust
fn scroll(...) -> bool
```

Inside that function, compute whether the logical scroll position changed.

The safest way is to compare before and after:

```rust
let old_logical_scroll_top = self.logical_scroll_top();
...
let new_logical_scroll_top = ...;
...
let did_scroll = new_logical_scroll_top != old_logical_scroll_top;
```

and return `did_scroll`.

Then in `List::paint`, use that boolean:

```rust
let did_scroll = list_state.0.borrow_mut().scroll(
    &scroll_top,
    height,
    pixel_delta,
    current_view,
    window,
    cx,
);

if did_scroll {
    window.claim_built_in_scroll();
}
```

### Why this helps

Line by line:

- the new guard stops `List` from double-handling an event already claimed by an inner built-in scroll container
- returning `bool` from `scroll(...)` avoids falsely claiming the event when the list did not actually move
- claiming only after real motion preserves parent fallback when the child is at a boundary

This is the change Option 2 was missing.

---

## Why this fixes the agent-panel bug

When the pointer is over a horizontally scrollable code block inside the thread list:

1. The code block's built-in `div` scroll listener runs first in bubble phase.
2. It changes `scroll_offset.x`.
3. It claims the built-in scroll event.
4. The outer thread `List` listener runs later.
5. The `List` listener sees that built-in scrolling was already claimed and skips.
6. Result: code block scrolls horizontally, thread list does not drift vertically.

That is the exact behavior the issue wants.

---

## Why this is better than the existing plan

Compared to Option 1:

- does not suppress later user mouse listeners

Compared to Option 2 as written:

- does not falsely mark `List` scroll as handled when no actual motion occurred

Compared to Option 3:

- consumes the whole event once the inner scrollable actually wins, which matches the desired UX for this bug

---

## Tests I would add

### 1. Nested horizontal `div` inside vertical `List`

Build a small GPUI test view with:

- outer `List`
- inner `div().overflow_x_scroll().track_scroll(...)`

Simulate a `ScrollWheelEvent` with mixed `x` and `y`.

Assert:

- inner horizontal offset changed
- outer list vertical position did not change

### 2. Boundary fallback

Same setup, but put the inner horizontal scroller at its horizontal boundary so it cannot move.

Assert:

- inner offset did not change
- outer vertical list did change

This proves the "claim only on real scroll" behavior.

### 3. `UniformList` coverage

Create a nested case where the parent is `UniformList` or the child is `UniformList`.

Assert the same claim semantics hold.

This ensures the fix is really GPUI-wide and not just `List`.

### 4. User `on_scroll_wheel` callback still fires

Attach a custom `.on_scroll_wheel(...)` callback to a parent or sibling scrollable.

Assert:

- the callback still runs
- only built-in scrolling is suppressed after the child claims the event

This protects the main regression risk that Option 1 would introduce.

---

## Final recommendation

I would keep the diagnosis from `nested_scroll_analysis.md`, but I would replace the implementation plan with this version:

> introduce a `Window`-level built-in scroll claim for each wheel event, make `div`/`UniformList` and `List` both respect it, and only claim after real scroll movement.

That is the smallest GPUI-wide fix that actually addresses the maintainer's objection:

- it is not specific to the agent panel
- it covers both `List` and `UniformList`
- it is testable
- it fixes the nested-scroll ownership problem at the GPUI layer instead of papering over one component
