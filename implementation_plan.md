# Fix Nested Scroll Propagation in GPUI

## Problem

When a child scrollable element (e.g., a horizontal code block) handles a `ScrollWheelEvent`, the parent scrollable (e.g., the agent chat `List`) also handles the same event. Neither calls `cx.stop_propagation()`, causing double-scrolling.

## User Review Required

> [!IMPORTANT]
> **Pick one option.** Each has different complexity and risk profiles. My recommendation is **Option 2** for its balance of correctness and safety.

---

## Option 1: Simple `stop_propagation` After Scroll

**Idea**: When a scroll listener actually changes its scroll offset, call `cx.stop_propagation()` to prevent parent scrollables from also handling the event.

### Diffs

#### [MODIFY] [div.rs](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/div.rs)

```diff
 // paint_scroll_listener, around line 2746-2750
                     scroll_offset.y += delta_y;
                     scroll_offset.x += delta_x;
                     if *scroll_offset != old_scroll_offset {
                         cx.notify(current_view);
+                        cx.stop_propagation();
                     }
```

#### [MODIFY] [list.rs](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/list.rs)

```diff
 // List::paint scroll listener, around line 1149-1168
         window.on_mouse_event(move |event: &ScrollWheelEvent, phase, window, cx| {
             if phase == DispatchPhase::Bubble && hitbox_id.should_handle_scroll(window) {
                 accumulated_scroll_delta = accumulated_scroll_delta.coalesce(event.delta);
                 let mut pixel_delta = accumulated_scroll_delta.pixel_delta(px(20.));
-                if pixel_delta.x.abs() > pixel_delta.y.abs() {
-                     accumulated_scroll_delta = match accumulated_scroll_delta {
-                         ScrollDelta::Pixels(p) => ScrollDelta::Pixels(point(p.x, px(0.))),
-                         ScrollDelta::Lines(p) => ScrollDelta::Lines(point(p.x, 0.)),
-                     };
-                    pixel_delta.y = px(0.);
-                }
                 list_state.0.borrow_mut().scroll(
                     &scroll_top,
                     height,
                     pixel_delta,
                     current_view,
                     window,
                     cx,
-                )
+                );
+                cx.stop_propagation();
             }
         });
```

### Tradeoffs

| Pros | Cons |
|------|------|
| ✅ Simplest change — 3 lines added | ❌ **Breaks scroll chaining**: child at boundary can't forward scroll to parent |
| ✅ Easy to understand | ❌ **Breaks user scroll_wheel_listeners**: parent `on_scroll_wheel` callbacks stop receiving events when a child scrollable exists |
| ✅ Consistent behavior | ❌ List always stops propagation, even on zero-delta scroll |
| | ❌ Elements like editors with both custom scroll listeners AND overflow scroll break |

> [!CAUTION]
> **Not recommended.** This breaks legitimate use cases where parent elements need to observe scroll events (e.g., scroll-to-bottom detection in the agent panel, auto-hide scrollbars in parent).

---

## Option 2: Scroll-Handled Flag on Window ⭐ Recommended

**Idea**: Add a `scroll_event_handled: bool` flag to `Window`. Built-in scroll listeners set it after consuming scroll delta. Other built-in scroll listeners check it before processing. User-registered `scroll_wheel_listeners` still receive the event (no `stop_propagation`).

This is essentially a scroll-specific side-channel that only affects built-in scroll handling, not user-registered event handlers.

### Diffs

#### [MODIFY] [window.rs](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/window.rs)

Add the flag to `Window` struct:

```diff
 pub struct Window {
     // ... existing fields around line 930
     default_prevented: bool,
     mouse_position: Point<Pixels>,
     mouse_hit_test: HitTest,
+    pub(crate) scroll_event_handled: bool,
     modifiers: Modifiers,
```

Reset it at the beginning of `dispatch_mouse_event`:

```diff
 fn dispatch_mouse_event(&mut self, event: &dyn Any, cx: &mut App) {
     let hit_test = self.rendered_frame.hit_test(self.mouse_position());
     if hit_test != self.mouse_hit_test {
         self.mouse_hit_test = hit_test;
         self.reset_cursor_style(cx);
     }
+
+    if event.is::<crate::ScrollWheelEvent>() {
+        self.scroll_event_handled = false;
+    }
```

Initialize in the `Window` constructor (search for where `default_prevented` is initialized):

```diff
             default_prevented: true,
+            scroll_event_handled: false,
             mouse_position: Default::default(),
```

#### [MODIFY] [div.rs](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/div.rs)

In `paint_scroll_listener`:

```diff
             window.on_mouse_event(move |event: &ScrollWheelEvent, phase, window, cx| {
-                if phase == DispatchPhase::Bubble && hitbox.should_handle_scroll(window) {
+                if phase == DispatchPhase::Bubble
+                    && !window.scroll_event_handled
+                    && hitbox.should_handle_scroll(window)
+                {
                     let mut scroll_offset = scroll_offset.borrow_mut();
                     let old_scroll_offset = *scroll_offset;
                     let delta = event.delta.pixel_delta(line_height);
                     // ... delta computation unchanged ...
                     scroll_offset.y += delta_y;
                     scroll_offset.x += delta_x;
                     if *scroll_offset != old_scroll_offset {
                         cx.notify(current_view);
+                        window.scroll_event_handled = true;
                     }
                 }
             });
```

#### [MODIFY] [list.rs](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/list.rs)

In `List::paint`:

```diff
         window.on_mouse_event(move |event: &ScrollWheelEvent, phase, window, cx| {
-            if phase == DispatchPhase::Bubble && hitbox_id.should_handle_scroll(window) {
+            if phase == DispatchPhase::Bubble
+                && !window.scroll_event_handled
+                && hitbox_id.should_handle_scroll(window)
+            {
                 accumulated_scroll_delta = accumulated_scroll_delta.coalesce(event.delta);
-                let mut pixel_delta = accumulated_scroll_delta.pixel_delta(px(20.));
-                if pixel_delta.x.abs() > pixel_delta.y.abs() {
-                     accumulated_scroll_delta = match accumulated_scroll_delta {
-                         ScrollDelta::Pixels(p) => ScrollDelta::Pixels(point(p.x, px(0.))),
-                         ScrollDelta::Lines(p) => ScrollDelta::Lines(point(p.x, 0.)),
-                     };
-                    pixel_delta.y = px(0.);
-                }
+                let pixel_delta = accumulated_scroll_delta.pixel_delta(px(20.));
                 list_state.0.borrow_mut().scroll(
                     &scroll_top,
                     height,
                     pixel_delta,
                     current_view,
                     window,
                     cx,
                 );
+                window.scroll_event_handled = true;
             }
         });
```

### Why This Works

1. **Bubble phase processes innermost first** — the code block's div listener fires before the parent List listener.
2. When the code block applies scroll delta, it sets `window.scroll_event_handled = true`.
3. When the parent List's listener runs, it sees `scroll_event_handled == true` and skips.
4. User-registered `scroll_wheel_listeners` (registered via `.on_scroll_wheel()`) are **not affected** — they don't check this flag, so they still receive all events.
5. The flag resets per-event in `dispatch_mouse_event`.

### Tradeoffs

| Pros | Cons |
|------|------|
| ✅ Doesn't break user `scroll_wheel_listeners` | ❌ Doesn't implement scroll chaining (child at boundary blocks parent) |
| ✅ Works uniformly across `div`, `List`, `UniformList` | ❌ Adds a `pub(crate)` field to `Window` (minor API surface) |
| ✅ Removes the need for per-component hacks (your `list.rs` change) | ❌ `List` always marks handled even when delta is zero — but this is edge case |
| ✅ Simple to test | |
| ✅ Low risk of regressions | |

> [!TIP]
> For the "child at boundary" issue: if you scroll horizontally in a code block that has no horizontal overflow (nothing to scroll), the offset won't change, so `scroll_event_handled` stays `false` and the parent List still gets the event. This naturally handles the common case.

---

## Option 3: Consumed-Delta Tracking

**Idea**: Instead of a boolean, track which **axes** of the delta have been consumed. A parent scrollable only processes the unconsumed axis. This allows a horizontal child to consume `delta.x` while letting `delta.y` pass through to the vertical parent.

### Diffs

#### [MODIFY] [window.rs](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/window.rs)

```diff
+/// Tracks which axes of a scroll event's delta have been consumed
+/// by an inner scrollable element during the current dispatch.
+#[derive(Default, Clone, Copy)]
+pub(crate) struct ScrollConsumption {
+    pub x: bool,
+    pub y: bool,
+}

 pub struct Window {
     // ...
     default_prevented: bool,
+    pub(crate) scroll_consumed: ScrollConsumption,
     mouse_position: Point<Pixels>,
```

Reset in `dispatch_mouse_event`:

```diff
+    if event.is::<crate::ScrollWheelEvent>() {
+        self.scroll_consumed = ScrollConsumption::default();
+    }
```

#### [MODIFY] [div.rs](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/div.rs)

```diff
             window.on_mouse_event(move |event: &ScrollWheelEvent, phase, window, cx| {
                 if phase == DispatchPhase::Bubble && hitbox.should_handle_scroll(window) {
                     let mut scroll_offset = scroll_offset.borrow_mut();
                     let old_scroll_offset = *scroll_offset;
                     let delta = event.delta.pixel_delta(line_height);

-                    let mut delta_x = Pixels::ZERO;
+                    let mut delta_x = if window.scroll_consumed.x { Pixels::ZERO } else { Pixels::ZERO };
                     if overflow.x == Overflow::Scroll {
-                        if !delta.x.is_zero() {
+                        if !delta.x.is_zero() && !window.scroll_consumed.x {
                             delta_x = delta.x;
                         } else if !restrict_scroll_to_axis && overflow.y != Overflow::Scroll {
-                            delta_x = delta.y;
+                            if !window.scroll_consumed.y {
+                                delta_x = delta.y;
+                            }
                         }
                     }
-                    let mut delta_y = Pixels::ZERO;
+                    let mut delta_y = Pixels::ZERO;
                     if overflow.y == Overflow::Scroll {
-                        if !delta.y.is_zero() {
+                        if !delta.y.is_zero() && !window.scroll_consumed.y {
                             delta_y = delta.y;
                         } else if !restrict_scroll_to_axis && overflow.x != Overflow::Scroll {
-                            delta_y = delta.x;
+                            if !window.scroll_consumed.x {
+                                delta_y = delta.x;
+                            }
                         }
                     }
                     // ... allow_concurrent_scroll logic unchanged ...
                     scroll_offset.y += delta_y;
                     scroll_offset.x += delta_x;
                     if *scroll_offset != old_scroll_offset {
                         cx.notify(current_view);
+                        if !delta_x.is_zero() {
+                            window.scroll_consumed.x = true;
+                        }
+                        if !delta_y.is_zero() {
+                            window.scroll_consumed.y = true;
+                        }
                     }
                 }
             });
```

#### [MODIFY] [list.rs](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/list.rs)

```diff
         window.on_mouse_event(move |event: &ScrollWheelEvent, phase, window, cx| {
-            if phase == DispatchPhase::Bubble && hitbox_id.should_handle_scroll(window) {
+            if phase == DispatchPhase::Bubble
+                && !window.scroll_consumed.y
+                && hitbox_id.should_handle_scroll(window)
+            {
                 accumulated_scroll_delta = accumulated_scroll_delta.coalesce(event.delta);
-                let mut pixel_delta = accumulated_scroll_delta.pixel_delta(px(20.));
-                if pixel_delta.x.abs() > pixel_delta.y.abs() {
-                     accumulated_scroll_delta = match accumulated_scroll_delta {
-                         ScrollDelta::Pixels(p) => ScrollDelta::Pixels(point(p.x, px(0.))),
-                         ScrollDelta::Lines(p) => ScrollDelta::Lines(point(p.x, 0.)),
-                     };
-                    pixel_delta.y = px(0.);
-                }
+                let pixel_delta = accumulated_scroll_delta.pixel_delta(px(20.));
                 list_state.0.borrow_mut().scroll(
                     &scroll_top, height, pixel_delta, current_view, window, cx,
                 );
+                window.scroll_consumed.y = true;
             }
         });
```

### Tradeoffs

| Pros | Cons |
|------|------|
| ✅ Most precise — axes are handled independently | ❌ More complex diff |
| ✅ Horizontal child doesn't block parent's vertical scroll if child has `overflow_x: Scroll` only | ❌ Still no scroll chaining at boundaries |
| ✅ User listeners unaffected | ❌ Requires careful logic in `paint_scroll_listener` to avoid consuming axes that weren't actually used |

> [!IMPORTANT]
> This option has a subtle behavior difference from Option 2: if a code block has `overflow_x: Scroll`, it only consumes the X axis. A trackpad gesture with `{x: -15, y: 2}` would let `y: 2` still reach the parent List. Whether this is desired depends on UX expectations.
> 
> macOS trackpads generate a small y-component on "pure horizontal" gestures. With this option, that small y-component would still scroll the parent. **Option 2 avoids this** because the inner code block's offset changes (from x), which sets the boolean flag to block the parent entirely.

---

## Option 4: Scroll Chaining with Boundary Detection

**Idea**: Like browsers, only propagate the scroll event to a parent if the child can't scroll further in the requested direction. This is the most sophisticated approach.

### Diffs

#### [MODIFY] [div.rs](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/div.rs)

Requires knowing the content_size to detect boundaries. `paint_scroll_listener` needs access to the content bounds.

```diff
     fn paint_scroll_listener(
         &self,
         hitbox: &Hitbox,
         style: &Style,
+        content_size: Size<Pixels>,
+        scroll_bounds: Size<Pixels>,
         window: &mut Window,
         _cx: &mut App,
     ) {
         if let Some(scroll_offset) = self.scroll_offset.clone() {
             // ... existing setup ...
+            let content_size = content_size;
+            let scroll_bounds = scroll_bounds;
             window.on_mouse_event(move |event: &ScrollWheelEvent, phase, window, cx| {
                 if phase == DispatchPhase::Bubble && hitbox.should_handle_scroll(window) {
                     let mut scroll_offset = scroll_offset.borrow_mut();
                     let old_scroll_offset = *scroll_offset;
                     // ... existing delta computation ...
                     scroll_offset.y += delta_y;
                     scroll_offset.x += delta_x;
+
+                    // Clamp to bounds
+                    let max_x = (content_size.width - scroll_bounds.width).max(Pixels::ZERO);
+                    let max_y = (content_size.height - scroll_bounds.height).max(Pixels::ZERO);
+                    scroll_offset.x = scroll_offset.x.clamp(-max_x, Pixels::ZERO);
+                    scroll_offset.y = scroll_offset.y.clamp(-max_y, Pixels::ZERO);
+
                     if *scroll_offset != old_scroll_offset {
                         cx.notify(current_view);
+                        window.scroll_event_handled = true;
+                    } else if !delta_x.is_zero() || !delta_y.is_zero() {
+                        // Delta was non-zero but offset didn't change = at boundary.
+                        // Don't mark as handled, let parent scroll.
                     }
                 }
             });
         }
     }
```

This also requires threading `content_size` and `scroll_bounds` into `paint_scroll_listener`, which means modifying `Interactivity::paint` to pass these values.

### Tradeoffs

| Pros | Cons |
|------|------|
| ✅ **Best UX** — matches browser behavior exactly | ❌ **Significantly more complex** — requires tracking content_size in all scroll containers |
| ✅ Scroll chaining works: reaching end of child lets parent take over | ❌ `Interactivity::paint_scroll_listener` needs new parameters |
| ✅ Handles all edge cases | ❌ `List` doesn't easily know its content_size at paint time (it uses lazy measurement) |
| | ❌ Highest risk of regressions |

> [!WARNING]
> The `content_size` is already available in `Interactivity::prepaint` (it's the value passed to `prepaint`), but threading it to `paint_scroll_listener` (which runs during `paint`) requires storing it in frame state. This is doable but touches more code.

---

## Option 5: New HitboxBehavior: `BlockScroll`

**Idea**: Introduce a new `HitboxBehavior::BlockScroll` that causes `should_handle_scroll` to return `false` for all hitboxes behind it. Scrollable elements can opt into this behavior to prevent parent scrollables from also processing scroll events.

### Diffs

#### [MODIFY] [window.rs](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/window.rs)

```diff
 pub enum HitboxBehavior {
     Normal,
     BlockMouse,
     BlockMouseExceptScroll,
+    BlockScrollPropagation,
 }
```

Modify `hit_test`:

```diff
     pub(crate) fn hit_test(&self, position: Point<Pixels>) -> HitTest {
         let mut set_hover_hitbox_count = false;
+        let mut set_scroll_hitbox_count = false;
         let mut hit_test = HitTest::default();
         for hitbox in self.hitboxes.iter().rev() {
             let bounds = hitbox.bounds.intersect(&hitbox.content_mask.bounds);
             if bounds.contains(&position) {
                 hit_test.ids.push(hitbox.id);
                 if !set_hover_hitbox_count
                     && hitbox.behavior == HitboxBehavior::BlockMouseExceptScroll
                 {
                     hit_test.hover_hitbox_count = hit_test.ids.len();
                     set_hover_hitbox_count = true;
                 }
+                if !set_scroll_hitbox_count
+                    && hitbox.behavior == HitboxBehavior::BlockScrollPropagation
+                {
+                    hit_test.scroll_hitbox_count = hit_test.ids.len();
+                    set_scroll_hitbox_count = true;
+                }
                 if hitbox.behavior == HitboxBehavior::BlockMouse {
                     break;
                 }
             }
         }
+        if !set_scroll_hitbox_count {
+            hit_test.scroll_hitbox_count = hit_test.ids.len();
+        }
         // ...
     }
```

Add to `HitTest`:

```diff
 pub(crate) struct HitTest {
     pub(crate) ids: SmallVec<[HitboxId; 8]>,
     pub(crate) hover_hitbox_count: usize,
+    pub(crate) scroll_hitbox_count: usize,
 }
```

Modify `should_handle_scroll`:

```diff
     pub fn should_handle_scroll(self, window: &Window) -> bool {
         let hit_test = &window.mouse_hit_test;
-        hit_test.ids.contains(&self)
+        hit_test.ids.iter().take(hit_test.scroll_hitbox_count).any(|id| *id == self)
     }
```

Then scrollable divs use `BlockScrollPropagation` instead of `Normal`:

#### [MODIFY] [div.rs](file:///Users/prayanshchhablani/work/zed/crates/gpui/src/elements/div.rs)

Where hitbox is created in `Interactivity::prepaint`:

```diff
-    let hitbox_behavior = if self.occlude_mouse {
-        HitboxBehavior::BlockMouse
-    } else if ... {
-        HitboxBehavior::BlockMouseExceptScroll
-    } else {
-        HitboxBehavior::Normal
-    };
+    let hitbox_behavior = if self.occlude_mouse {
+        HitboxBehavior::BlockMouse
+    } else if ... {
+        HitboxBehavior::BlockMouseExceptScroll
+    } else if has_scroll_overflow {
+        HitboxBehavior::BlockScrollPropagation
+    } else {
+        HitboxBehavior::Normal
+    };
```

### Tradeoffs

| Pros | Cons |
|------|------|
| ✅ Uses existing hitbox infrastructure | ❌ **Most invasive** — changes `HitTest`, `HitboxBehavior`, and `should_handle_scroll` |
| ✅ Opt-in per element | ❌ `HitboxBehavior` is the wrong abstraction — it gates **which** elements handle events, not **how** |
| ✅ No runtime flag needed | ❌ `List` creates its own hitbox and would need updating separately |
| | ❌ Doesn't support scroll chaining at boundaries |
| | ❌ Changes public API surface (`HitboxBehavior` enum) |

---

## Comparison Matrix

| Criteria | Option 1 | Option 2 ⭐ | Option 3 | Option 4 | Option 5 |
|----------|----------|------------|----------|----------|----------|
| **Complexity** | Trivial | Low | Medium | High | High |
| **Lines changed** | ~6 | ~15 | ~30 | ~50+ | ~40 |
| **Breaks user listeners** | ❌ Yes | ✅ No | ✅ No | ✅ No | ✅ No |
| **Scroll chaining** | ❌ No | ⚠️ Natural (boundary = no offset change) | ❌ No | ✅ Yes | ❌ No |
| **Per-axis control** | ❌ No | ❌ No | ✅ Yes | ✅ Yes | ❌ No |
| **Fixes List** | ✅ | ✅ | ✅ | ✅ | ⚠️ Needs extra |
| **Fixes UniformList** | ✅ | ✅ | ✅ | ✅ | ⚠️ Needs extra |
| **Fixes div scroll** | ✅ | ✅ | ✅ | ✅ | ✅ |
| **Test difficulty** | Easy | Easy | Medium | Hard | Medium |
| **Regression risk** | High | Low | Low | Medium | Medium |

---

## Recommendation: Option 2

**Option 2** is the best balance of:
- **Safety**: Doesn't use `stop_propagation`, so user-registered listeners still work
- **Coverage**: One flag works for `div`, `List`, and `UniformList` uniformly
- **Natural scroll chaining**: If the inner element can't scroll (offset doesn't change), the flag stays `false` and the parent handles it
- **Simplicity**: ~15 lines, easy to review
- **Testability**: Straightforward nested-scroll test

The code block in the agent panel has `restrict_scroll_to_axis = true` and `overflow_x: Scroll`. When you swipe horizontally, the code block's offset changes → `scroll_event_handled = true` → parent List skips. When the code block has no horizontal overflow, offset stays the same → flag stays `false` → parent List scrolls normally.

---

## Test Plan

For any option, add this test to `crates/gpui/src/elements/div.rs` (or a new test file):

```rust
#[cfg(test)]
mod scroll_propagation_tests {
    use crate::{self as gpui, *};

    #[gpui::test]
    fn test_nested_scroll_does_not_propagate_to_parent(cx: &mut TestAppContext) {
        let outer_scroll_offset = Rc::new(Cell::new(Point::<Pixels>::default()));
        let inner_scroll_offset = Rc::new(Cell::new(Point::<Pixels>::default()));

        struct TestView {
            outer_scroll: ScrollHandle,
            inner_scroll: ScrollHandle,
        }

        impl Render for TestView {
            fn render(
                &mut self,
                _window: &mut Window,
                _cx: &mut Context<Self>,
            ) -> impl IntoElement {
                // Outer: vertical scroll container (200px viewport, 400px content)
                div()
                    .id("outer")
                    .overflow_y_scroll()
                    .track_scroll(&self.outer_scroll)
                    .h(px(200.))
                    .w(px(300.))
                    .child(
                        div().h(px(400.)).w(px(300.)).child(
                            // Inner: horizontal scroll container (200px viewport, 500px content)
                            div()
                                .id("inner")
                                .overflow_x_scroll()
                                .track_scroll(&self.inner_scroll)
                                .h(px(50.))
                                .w(px(200.))
                                .child(div().h(px(50.)).w(px(500.))),
                        ),
                    )
            }
        }

        let outer_handle = ScrollHandle::new();
        let inner_handle = ScrollHandle::new();
        let outer_for_assert = outer_handle.clone();
        let inner_for_assert = inner_handle.clone();

        let (_, cx) = cx.add_window_view(|_, _| TestView {
            outer_scroll: outer_handle,
            inner_scroll: inner_handle,
        });

        // Simulate a horizontal scroll event over the inner element
        cx.simulate_event(ScrollWheelEvent {
            position: point(px(100.), px(25.)),  // inside the inner scroll container
            delta: ScrollDelta::Pixels(point(px(-30.), px(-5.))),  // mostly horizontal
            ..Default::default()
        });

        // Inner should have scrolled horizontally
        let inner_offset = inner_for_assert.offset();
        assert!(inner_offset.x < px(0.), "inner should scroll horizontally");

        // Outer should NOT have scrolled vertically
        let outer_offset = outer_for_assert.offset();
        assert_eq!(
            outer_offset.y,
            px(0.),
            "outer should not scroll when inner handled the event"
        );
    }

    #[gpui::test]
    fn test_scroll_propagates_when_child_cant_scroll(cx: &mut TestAppContext) {
        struct TestView {
            outer_scroll: ScrollHandle,
            inner_scroll: ScrollHandle,
        }

        impl Render for TestView {
            fn render(
                &mut self,
                _window: &mut Window,
                _cx: &mut Context<Self>,
            ) -> impl IntoElement {
                div()
                    .id("outer")
                    .overflow_y_scroll()
                    .track_scroll(&self.outer_scroll)
                    .h(px(200.))
                    .w(px(300.))
                    .child(
                        div().h(px(400.)).w(px(300.)).child(
                            // Inner: horizontal scroll but content fits (no overflow)
                            div()
                                .id("inner")
                                .overflow_x_scroll()
                                .track_scroll(&self.inner_scroll)
                                .h(px(50.))
                                .w(px(200.))
                                .child(div().h(px(50.)).w(px(100.))),  // fits inside!
                        ),
                    )
            }
        }

        let outer_handle = ScrollHandle::new();
        let inner_handle = ScrollHandle::new();
        let outer_for_assert = outer_handle.clone();

        let (_, cx) = cx.add_window_view(|_, _| TestView {
            outer_scroll: outer_handle,
            inner_scroll: inner_handle,
        });

        // Simulate a vertical scroll event over the inner element
        cx.simulate_event(ScrollWheelEvent {
            position: point(px(100.), px(25.)),
            delta: ScrollDelta::Pixels(point(px(0.), px(-30.))),
            ..Default::default()
        });

        // Outer SHOULD scroll because inner had no scrollable content
        let outer_offset = outer_for_assert.offset();
        assert!(
            outer_offset.y < px(0.),
            "outer should scroll when inner can't"
        );
    }
}
```

---

## Open Questions

1. **Should `List` mark `scroll_event_handled` unconditionally, or only when the scroll position actually changed?** For Option 2, I opted for unconditional since `List::scroll` doesn't return whether it changed. We could modify `scroll` to return a `bool` indicating whether position changed.

2. **Should we revert your existing `list.rs` change** (the `pixel_delta.x.abs() > pixel_delta.y.abs()` check)? With Option 2, this local fix is unnecessary since the parent won't process the event anyway.

3. **Do we need per-axis tracking (Option 3)?** With Option 2, a horizontal-only child that applies any delta blocks the parent entirely for that event. Given trackpads always produce mixed deltas, this seems correct — but worth confirming.

---

## Verification Plan

### Automated Tests
- Run the two tests above: `test_nested_scroll_does_not_propagate_to_parent` and `test_scroll_propagates_when_child_cant_scroll`
- Run existing scroll tests: `cargo test -p gpui scroll` to verify no regressions

### Manual Verification
- Open AI Chat panel, get a response with a code block that has horizontal overflow
- Swipe horizontally on the code block — parent list should NOT scroll vertically
- Scroll vertically outside any code block — should work normally
- Scroll in a code block that fits (no overflow) — parent should scroll normally
