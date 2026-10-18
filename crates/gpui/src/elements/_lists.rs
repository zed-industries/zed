//! # Lists in GPUI
//!
//! The [`list`] element renders large, variable-height collections efficiently:
//! it lays out and paints only the rows in the viewport plus the overdraw, a
//! configurable amount of extra space measured beyond each edge. Rows outside
//! that range stay unmeasured until they scroll closer.
//!
//! [`ListState`] is the handle that coordinates this with your view. It is
//! stored on the view, and cloning it yields a shared handle to the same state.
//! It caches row heights, tracks the scroll anchor, and keeps the scrollbar in
//! sync as content changes.
//!
//! A minimal example can be found in the `examples/list_example.rs` file.
//!
//! ## Creating a list
//!
//! Construct a [`ListState`] with the item count, alignment, and overdraw. Rows
//! are rendered lazily by the closure passed to [`list`]:
//!
//! ```rust
//! use gpui::*;
//!
//! struct MyView {
//!     state: ListState,
//! }
//!
//! impl MyView {
//!     fn new() -> Self {
//!         // 100 rows, top-to-bottom, with about 50px of overdraw beyond each
//!         // edge. Rows are measured a whole row at a time, so the actual
//!         // buffer is the first row boundary past 50px.
//!         let state = ListState::new(100, ListAlignment::Top, px(50.0));
//!         Self { state }
//!     }
//! }
//!
//! impl Render for MyView {
//!     fn render(&mut self, _window: &mut Window, _cx: &mut Context<Self>) -> impl IntoElement {
//!         list(self.state.clone(), |ix, _window, _cx| {
//!             div()
//!                 .h(px(32.0))
//!                 .w_full()
//!                 .child(format!("Item #{ix}"))
//!                 .into_any_element()
//!         })
//!         .size_full()
//!     }
//! }
//! ```
//!
//! The list scrolls itself, but it only scrolls if its height is bounded by the
//! parent. Give it a `flex_1` row, a `size_full` container, or an explicit
//! height; in unbounded space it lays out at its content height and there is
//! nothing to scroll.
//!
//! Use [`ListAlignment::Bottom`] for chat-like logs that grow at the end.
//! [`ListState::measure_all`] measures every row on the first frame, which
//! gives the scrollbar an exact size up front; [`ListState::with_uniform_item_height`]
//! is a cheaper alternative when rows have roughly uniform heights.
//!
//! ## Changing content
//!
//! [`ListState`] caches row heights, so tell it when the data changes.
//!
//! | Method | Use when | Effect on scroll position |
//! | --- | --- | --- |
//! | [`ListState::splice`] | Rows are added, removed, or replaced | Adjusts indices; the anchor is clamped if it was removed |
//! | [`ListState::splice_focusable`] | Same, for rows with focus handles | Same as `splice` |
//! | [`ListState::remeasure_items`] | A row's content changed but its index did not | Preserves the in-item pixel offset |
//! | [`ListState::remeasure`] | A global size change, such as a font size or zoom change | Preserves the proportional position |
//! | [`ListState::reset`] | The whole data set was replaced | Drops cached heights and the scroll position |
//!
//! In short: if most rows carry over, [`ListState::splice`]; if one row's
//! content or height changed at a stable index, [`ListState::remeasure_items`];
//! if every row may have changed height but the rows themselves are the same,
//! [`ListState::remeasure`]; if the data set is entirely new,
//! [`ListState::reset`].
//!
//! ```rust
//! # use gpui::*;
//! # let state = ListState::new(10, ListAlignment::Top, px(0.));
//! // Replace rows 5..10 with two new rows.
//! state.splice(5..10, 2);
//!
//! // A row's content changed but it still exists at the same index.
//! state.remeasure_items(4..5);
//!
//! // The whole data set was replaced by 42 different rows.
//! state.reset(42);
//! ```
//!
//! A splice reindexes the rows that follow `old_range`, so an index captured
//! before the splice can name a different row afterwards:
//!
//! ```rust
//! # use gpui::*;
//! # let state = ListState::new(10, ListAlignment::Top, px(0.));
//! // Before: rows 0..10. Replace rows 5..8 with one new row.
//! state.splice(5..8, 1);
//! // Old row 8 is now row 6, and old row 9 is now row 7.
//! ```
//!
//! Hold the row's own identity (its domain id) instead of an index, and resolve
//! the index again after a splice.
//!
//! [`ListState::reset`] also drops scroll events until the next paint, and it
//! bumps [`ListState::generation`] (see below).
//!
//! ## Following the tail
//!
//! For logs and chat transcripts, [`ListState::set_follow_mode`] with
//! [`FollowMode::Tail`] keeps the list at the end as content arrives:
//!
//! ```rust
//! # use gpui::*;
//! # let state = ListState::new(10, ListAlignment::Top, px(0.));
//! state.set_follow_mode(FollowMode::Tail);
//! ```
//!
//! Scrolling up suspends following. Scrolling back to the bottom resumes it.
//! [`ListState::pause_following_tail`] freezes the position without leaving
//! tail mode, which is useful while zooming or resizing content.
//!
//! ## Provisional heights
//!
//! Some rows do not know their height on the first layout: deferred markdown,
//! rasterized diagrams, remote images. If those rows measure as zero height,
//! the list's normal underfill and follow corrections would move the scroll
//! anchor and lose a restored position.
//!
//! Provisional mode pins the anchor while those rows resolve. While it is
//! active:
//!
//! - the requested [`ListOffset`] and its in-item offset are preserved;
//! - rows above the anchor are not laid out;
//! - tail-following is not re-enabled;
//! - the viewport may remain underfilled.
//!
//! The lifecycle is a small state machine:
//!
//! ```text
//! scroll_to(anchor)
//!       │
//!       ▼
//! begin_provisional_layout() ──▶ Provisional { anchor, pending: None }
//!                                        │
//!                     remeasure_items ───┤ pending = offset relative to anchor
//!                                        │
//!                     rows settle ───────┤
//!                                        ▼
//!                           end_provisional_layout()
//!                                        │
//!                                        ▼
//!                     Free; next layout applies the pending offset
//!
//! navigation, reset, or a splice at or before the anchor ends Provisional
//! and drops the pending offset with it.
//! ```
//!
//! Enter it after positioning the list, and leave it once the rows around the
//! anchor have their final heights:
//!
//! ```rust
//! # use gpui::*;
//! # let state = ListState::new(100, ListAlignment::Top, px(0.));
//! state.scroll_to(ListOffset {
//!     item_ix: 42,
//!     offset_in_item: px(15.0),
//! });
//!
//! // Returns false if there was no anchor to pin; call `scroll_to` first.
//! if !state.begin_provisional_layout() {
//!     // Nothing was pinned.
//! }
//!
//! // Once the rows around the anchor have been laid out:
//! state.end_provisional_layout();
//! ```
//!
//! [`ListState::end_provisional_layout`] is idempotent: navigation may already
//! have ended provisional mode.
//!
//! [`ListState::pin_provisional_layout`] combines the two and returns a
//! [`ProvisionalGuard`] that ends provisional mode when dropped. The guard must
//! stay alive for as long as the heights are unsettled; dropping it at the end
//! of the setup function releases the pin immediately. Store it on the view
//! when the settling spans frames:
//!
//! ```rust
//! use gpui::*;
//!
//! struct MyView {
//!     state: ListState,
//!     provisional: Option<ProvisionalGuard>,
//! }
//!
//! impl MyView {
//!     fn restore_position(&mut self, saved_offset: ListOffset) {
//!         self.state.scroll_to(saved_offset);
//!         self.provisional = Some(self.state.pin_provisional_layout());
//!     }
//!
//!     fn rows_have_final_heights(&mut self) {
//!         // Navigation may have released the pin already; this is a no-op then.
//!         self.provisional = None;
//!     }
//! }
//! ```
//!
//! [`ListState::set_item_heights_provisional`] is the low-level boolean form of
//! the same lifecycle.
//!
//! Navigation that moves the anchor ends provisional mode and drops the
//! pending offset with it: [`ListState::scroll_to`], [`ListState::scroll_to_end`],
//! [`ListState::scroll_to_reveal_item`], non-zero [`ListState::scroll_by`], wheel
//! scrolling, and scrollbar navigation. [`ListState::reset`] and a
//! [`ListState::splice`] at or before the anchor do the same.
//!
//! ## Generations
//!
//! An index captured before an `await` can become stale if the list is reset
//! while waiting. [`ListState::generation`] increments on every
//! [`ListState::reset`]; compare it before applying a measurement:
//!
//! ```rust
//! # use gpui::*;
//! # struct MyView { state: ListState }
//! # async fn fetch_metadata(_ix: usize) {}
//! # impl MyView {
//! # fn apply(&mut self, _ix: usize, _data: ()) {}
//! fn load(&mut self, item_ix: usize, cx: &mut Context<Self>) {
//!     let generation = self.state.generation();
//!     cx.spawn(async move |this, cx| {
//!         let data = fetch_metadata(item_ix).await;
//!         this.update(cx, |view, cx| {
//!             if view.state.generation() == generation {
//!                 view.apply(item_ix, data);
//!                 view.state.remeasure_items(item_ix..item_ix + 1);
//!                 cx.notify();
//!             }
//!         })
//!         .ok();
//!     })
//!     .detach();
//! }
//! # }
//! ```
//!
//! [`ListState::splice`] does not bump the generation: it reindexes items in
//! place, so an index captured across a splice can refer to a different item
//! without the generation changing. Callers that hold an index across frames
//! should keep the item's own identity (for example, its domain id) and resolve
//! the current index after a splice.
//!
//! ## Viewport queries
//!
//! [`ListState::item_is_above_viewport`] and
//! [`ListState::item_is_below_viewport`] report whether a row has left the
//! viewport. They return `None` until enough layout is known:
//!
//! ```rust
//! # use gpui::*;
//! # let state = ListState::new(10, ListAlignment::Top, px(0.));
//! # let item_index = 3;
//! if let Some(is_above) = state.item_is_above_viewport(item_index) {
//!     if is_above {
//!         // The row is scrolled off past the top edge.
//!     }
//! }
//! ```
//!
//! ## Further reading
//!
//! - [`ListState`] for the full scroll and measurement API.
//! - [`list`] for the element itself, including the item closure signature.
//! - `examples/list_example.rs` for a runnable bottom-aligned list.

#[cfg(doc)]
use crate::*; // so I don't have to qualify every type :)
