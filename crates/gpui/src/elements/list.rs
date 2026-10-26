//! A list element that can be used to render a large number of differently sized elements
//! efficiently. Clients of this API need to ensure that elements outside of the scrolled
//! area do not change their height for this element to function correctly. If your elements
//! do change height, notify the list element via [`ListState::splice`] or [`ListState::reset`].
//! In order to minimize re-renders, this element's state is stored intrusively
//! on your own views, so that your code can coordinate directly with the list element's cached state.
//!
//! If all of your elements are the same height, see [`crate::UniformList`] for a simpler API

use crate::{
    AnyElement, App, AvailableSpace, Bounds, ContentMask, DispatchPhase, Edges, Element, EntityId,
    FocusHandle, GlobalElementId, Hitbox, HitboxBehavior, InspectorElementId, IntoElement,
    Overflow, Pixels, Point, ScrollDelta, ScrollEvent, Size, Style, StyleRefinement, Styled,
    Window, point, px, size,
};
use collections::VecDeque;
use refineable::Refineable as _;
use std::{cell::RefCell, ops::Range, rc::Rc};
use sum_tree::{Bias, Dimensions, SumTree};

type RenderItemFn = dyn FnMut(usize, &mut Window, &mut App) -> AnyElement + 'static;

/// Construct a new list element
pub fn list(
    state: ListState,
    render_item: impl FnMut(usize, &mut Window, &mut App) -> AnyElement + 'static,
) -> List {
    List {
        state,
        render_item: Box::new(render_item),
        style: StyleRefinement::default(),
        sizing_behavior: ListSizingBehavior::default(),
    }
}

/// A list element
pub struct List {
    state: ListState,
    render_item: Box<RenderItemFn>,
    style: StyleRefinement,
    sizing_behavior: ListSizingBehavior,
}

impl List {
    /// Set the sizing behavior for the list.
    pub fn with_sizing_behavior(mut self, behavior: ListSizingBehavior) -> Self {
        self.sizing_behavior = behavior;
        self
    }
}

/// The list state that views must hold on behalf of the list element.
#[derive(Clone)]
pub struct ListState(Rc<RefCell<StateInner>>);

impl std::fmt::Debug for ListState {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ListState")
    }
}

struct StateInner {
    last_layout_bounds: Option<Bounds<Pixels>>,
    last_padding: Option<Edges<Pixels>>,
    items: SumTree<ListItem>,
    scroll_anchor: Option<ScrollAnchor>,
    alignment: ListAlignment,
    overdraw: Pixels,
    reset: bool,
    #[allow(clippy::type_complexity)]
    scroll_handler: Option<Box<dyn FnMut(&ListScrollEvent, &mut Window, &mut App)>>,
    scrollbar_drag_start_height: Option<Pixels>,
    measuring_behavior: ListMeasuringBehavior,
    item_measurement_ready: Option<Box<dyn Fn(usize, &App) -> bool>>,
    follow_state: FollowState,
}

#[derive(Clone, Copy)]
struct ScrollAnchor {
    position: ListOffset,
    pending: Option<PendingScroll>,
    provisional: bool,
    pending_end: bool,
}

#[derive(Clone, Copy)]
enum PendingScroll {
    Absolute(Pixels),
    Proportional(f32),
}

enum ScrollAnchorMode {
    Absolute,
    Proportional,
}

/// Controls whether the list automatically follows new content at the end.
#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub enum FollowMode {
    /// Normal scrolling — no automatic following.
    #[default]
    Normal,
    /// The list should auto-scroll along with the tail, when scrolled to bottom.
    Tail,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
enum FollowState {
    #[default]
    Normal,
    Tail {
        is_following: bool,
    },
}

impl FollowState {
    fn is_following(&self) -> bool {
        matches!(self, FollowState::Tail { is_following: true })
    }

    fn has_stopped_following(&self) -> bool {
        matches!(
            self,
            FollowState::Tail {
                is_following: false
            }
        )
    }

    fn start_following(&mut self) {
        if let FollowState::Tail {
            is_following: false,
        } = self
        {
            *self = FollowState::Tail { is_following: true };
        }
    }

    fn stop_following(&mut self) {
        if let FollowState::Tail { is_following: true } = self {
            *self = FollowState::Tail {
                is_following: false,
            };
        }
    }
}

/// Whether the list is scrolling from top to bottom or bottom to top.
#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ListAlignment {
    /// The list is scrolling from top to bottom, like most lists.
    Top,
    /// The list is scrolling from bottom to top, like a chat log.
    Bottom,
}

/// A scroll event that has been converted to be in terms of the list's items.
pub struct ListScrollEvent {
    /// The range of items currently visible in the list, after applying the scroll event.
    pub visible_range: Range<usize>,

    /// The number of items that are currently visible in the list, after applying the scroll event.
    pub count: usize,

    /// Whether the list has been scrolled.
    pub is_scrolled: bool,

    /// Whether the list is currently in follow-tail mode (auto-scrolling to end).
    pub is_following_tail: bool,
}

/// The sizing behavior to apply during layout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ListSizingBehavior {
    /// The list should calculate its size based on the size of its items.
    Infer,
    /// The list should not calculate a fixed size.
    #[default]
    Auto,
}

/// The measuring behavior to apply during layout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ListMeasuringBehavior {
    /// Measure all items in the list.
    /// Note: This can be expensive for the first frame in a large list.
    Measure(bool),
    /// Only measure visible items
    #[default]
    Visible,
}

impl ListMeasuringBehavior {
    fn reset(&mut self) {
        match self {
            ListMeasuringBehavior::Measure(has_measured) => *has_measured = false,
            ListMeasuringBehavior::Visible => {}
        }
    }
}

/// The horizontal sizing behavior to apply during layout.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum ListHorizontalSizingBehavior {
    /// List items' width can never exceed the width of the list.
    #[default]
    FitList,
    /// List items' width may go over the width of the list, if any item is wider.
    Unconstrained,
}

struct LayoutItemsResponse {
    max_item_width: Pixels,
    scroll_top: ListOffset,
    item_layouts: VecDeque<ItemLayout>,
}

struct ItemLayout {
    index: usize,
    element: AnyElement,
    size: Size<Pixels>,
}

/// Frame state used by the [List] element after layout.
pub struct ListPrepaintState {
    hitbox: Hitbox,
    layout: LayoutItemsResponse,
}

#[derive(Clone)]
enum ListItem {
    Unmeasured {
        size_hint: Option<Size<Pixels>>,
        focus_handle: Option<FocusHandle>,
    },
    Measured {
        size: Size<Pixels>,
        focus_handle: Option<FocusHandle>,
    },
}

impl ListItem {
    fn with_measurement(&self, size: Size<Pixels>, ready: bool) -> Self {
        if ready {
            Self::Measured {
                size,
                focus_handle: self.focus_handle(),
            }
        } else {
            Self::Unmeasured {
                size_hint: Some(size),
                focus_handle: self.focus_handle(),
            }
        }
    }

    fn size(&self) -> Option<Size<Pixels>> {
        if let ListItem::Measured { size, .. } = self {
            Some(*size)
        } else {
            None
        }
    }

    fn size_hint(&self) -> Option<Size<Pixels>> {
        match self {
            ListItem::Measured { size, .. } => Some(*size),
            ListItem::Unmeasured { size_hint, .. } => *size_hint,
        }
    }

    fn focus_handle(&self) -> Option<FocusHandle> {
        match self {
            ListItem::Unmeasured { focus_handle, .. } | ListItem::Measured { focus_handle, .. } => {
                focus_handle.clone()
            }
        }
    }

    fn contains_focused(&self, window: &Window, cx: &App) -> bool {
        match self {
            ListItem::Unmeasured { focus_handle, .. } | ListItem::Measured { focus_handle, .. } => {
                focus_handle
                    .as_ref()
                    .is_some_and(|handle| handle.contains_focused(window, cx))
            }
        }
    }
}

#[derive(Clone, Debug, Default, PartialEq)]
struct ListItemSummary {
    count: usize,
    rendered_count: usize,
    unrendered_count: usize,
    height: Pixels,
    has_focus_handles: bool,
    has_unknown_height: bool,
}

#[derive(Clone, Debug, Default, PartialEq, Eq, PartialOrd, Ord)]
struct Count(usize);

#[derive(Clone, Debug, Default)]
struct Height(Pixels);

impl ListState {
    /// Construct a new list state, for storage on a view.
    ///
    /// The overdraw parameter controls how much extra space is rendered
    /// above and below the visible area. Elements within this area will
    /// be measured even though they are not visible. This can help ensure
    /// that the list doesn't flicker or pop in when scrolling.
    pub fn new(item_count: usize, alignment: ListAlignment, overdraw: Pixels) -> Self {
        let state = Self(Rc::new(RefCell::new(StateInner {
            last_layout_bounds: None,
            last_padding: None,
            items: SumTree::default(),
            scroll_anchor: None,
            alignment,
            overdraw,
            scroll_handler: None,
            reset: false,
            scrollbar_drag_start_height: None,
            measuring_behavior: ListMeasuringBehavior::default(),
            item_measurement_ready: None,
            follow_state: FollowState::default(),
        })));
        state.splice(0..0, item_count);
        state
    }

    /// Set the list to measure all items in the list in the first layout phase.
    ///
    /// This is useful for ensuring that the scrollbar size is correct instead of based on only rendered elements.
    pub fn measure_all(self) -> Self {
        self.0.borrow_mut().measuring_behavior = ListMeasuringBehavior::Measure(false);
        self
    }

    /// Pre-populate every unmeasured item with a uniform height hint so the scrollbar thumb
    /// is correctly sized from the first frame, without measuring all items up front.
    ///
    /// As items are actually rendered their real heights replace the hint, so the scrollbar
    /// converges to the exact size over time. This is a cheaper alternative to [`Self::measure_all`]
    /// for lists where items have roughly uniform heights (e.g. table rows).
    pub fn with_uniform_item_height(self, height: Pixels) -> Self {
        self.apply_uniform_item_height(height);
        self
    }

    /// Reset this instantiation of the list state.
    ///
    /// Note that this will cause scroll events to be dropped until the next paint.
    pub fn reset(&self, element_count: usize) {
        let old_count = {
            let state = &mut *self.0.borrow_mut();
            state.reset = true;
            state.measuring_behavior.reset();
            state.set_scroll_anchor(None);
            state.scrollbar_drag_start_height = None;
            state.items.summary().count
        };

        self.splice(0..old_count, element_count);
    }

    /// Reset the list to `element_count` items, pre-populating every item with a
    /// uniform height hint so the scrollbar thumb is correctly sized from the first
    /// frame even for off-screen items.
    pub fn reset_with_uniform_height(&self, element_count: usize, height: Pixels) {
        self.reset(element_count);
        self.apply_uniform_item_height(height);
    }

    fn apply_uniform_item_height(&self, height: Pixels) {
        let size_hint = Size {
            width: px(0.),
            height,
        };
        let mut state = self.0.borrow_mut();
        let new_items = state
            .items
            .iter()
            .map(|item| ListItem::Unmeasured {
                size_hint: Some(item.size_hint().unwrap_or(size_hint)),
                focus_handle: item.focus_handle(),
            })
            .collect::<Vec<_>>();
        let mut tree = SumTree::default();
        tree.extend(new_items, ());
        state.items = tree;
    }

    /// Remeasure all items while preserving proportional scroll position.
    ///
    /// Use this when item heights may have changed (e.g., font size changes)
    /// but the number and identity of items remains the same.
    pub fn remeasure(&self) {
        let count = self.item_count();
        self.remeasure_items_with_scroll_anchor(0..count, ScrollAnchorMode::Proportional);
    }

    /// Mark items in `range` as needing remeasurement while preserving
    /// the current scroll position. Unlike [`Self::splice`], this does
    /// not change the number of items or blow away `logical_scroll_top`.
    ///
    /// Use this when an item's content has changed and its rendered
    /// height may be different (e.g., streaming text, tool results
    /// loading), but the item itself still exists at the same index.
    pub fn remeasure_items(&self, range: Range<usize>) {
        self.remeasure_items_with_scroll_anchor(range, ScrollAnchorMode::Absolute);
    }

    fn remeasure_items_with_scroll_anchor(&self, range: Range<usize>, mode: ScrollAnchorMode) {
        let state = &mut *self.0.borrow_mut();

        if let Some(anchor) = state.scroll_anchor.as_mut()
            && range.contains(&anchor.position.item_ix)
        {
            anchor.pending = match mode {
                ScrollAnchorMode::Absolute => {
                    Some(PendingScroll::Absolute(anchor.position.offset_in_item))
                }
                ScrollAnchorMode::Proportional => {
                    let mut cursor = state.items.cursor::<Count>(());
                    cursor.seek(&Count(anchor.position.item_ix), Bias::Right);
                    cursor
                        .item()
                        .and_then(|item| item.size())
                        .map(|size| {
                            let fraction = if size.height > px(0.) {
                                (anchor.position.offset_in_item / size.height).clamp(0., 1.)
                            } else {
                                0.
                            };
                            PendingScroll::Proportional(fraction)
                        })
                        .or(anchor.pending)
                }
            };
        }

        // Rebuild the tree, replacing items in the range with
        // Unmeasured copies that keep their focus handles.
        let new_items = {
            let mut cursor = state.items.cursor::<Count>(());
            let mut new_items = cursor.slice(&Count(range.start), Bias::Right);
            let invalidated = cursor.slice(&Count(range.end), Bias::Right);
            new_items.extend(
                invalidated.iter().map(|item| ListItem::Unmeasured {
                    size_hint: item.size_hint(),
                    focus_handle: item.focus_handle(),
                }),
                (),
            );
            new_items.append(cursor.suffix(), ());
            new_items
        };
        state.items = new_items;
        state.measuring_behavior.reset();
    }

    /// The number of items in this list.
    pub fn item_count(&self) -> usize {
        self.0.borrow().items.summary().count
    }

    /// Whether the list is scrolled to the end, or `None` if the list is
    /// not scrollable or the total content height is not yet known.
    pub fn is_scrolled_to_end(&self) -> Option<bool> {
        let state = self.0.borrow();
        let bounds = state.last_layout_bounds?;
        let summary = state.items.summary();
        if summary.has_unknown_height {
            return None;
        }
        let padding = state.last_padding.unwrap_or_default();
        let content_height = summary.height + padding.top + padding.bottom;
        let scroll_max = (content_height - bounds.size.height).max(px(0.));
        if scroll_max <= px(0.) {
            return None;
        }
        let scroll_top = state.scroll_top(&state.logical_scroll_top());
        Some(scroll_top >= scroll_max)
    }

    /// Inform the list state that the items in `old_range` have been replaced
    /// by `count` new items that must be recalculated.
    pub fn splice(&self, old_range: Range<usize>, count: usize) {
        self.splice_focusable(old_range, (0..count).map(|_| None))
    }

    /// Register with the list state that the items in `old_range` have been replaced
    /// by new items. As opposed to [`Self::splice`], this method allows an iterator of optional focus handles
    /// to be supplied to properly integrate with items in the list that can be focused. If a focused item
    /// is scrolled out of view, the list will continue to render it to allow keyboard interaction.
    pub fn splice_focusable(
        &self,
        old_range: Range<usize>,
        focus_handles: impl IntoIterator<Item = Option<FocusHandle>>,
    ) {
        let state = &mut *self.0.borrow_mut();

        let mut old_items = state.items.cursor::<Count>(());
        let mut new_items = old_items.slice(&Count(old_range.start), Bias::Right);
        old_items.seek_forward(&Count(old_range.end), Bias::Right);

        let mut spliced_count = 0;
        new_items.extend(
            focus_handles.into_iter().map(|focus_handle| {
                spliced_count += 1;
                ListItem::Unmeasured {
                    size_hint: None,
                    focus_handle,
                }
            }),
            (),
        );
        new_items.append(old_items.suffix(), ());
        drop(old_items);
        state.items = new_items;

        if let Some(anchor) = state.scroll_anchor.as_mut() {
            if anchor.provisional && old_range.start <= anchor.position.item_ix {
                anchor.provisional = false;
                anchor.pending = None;
            }
            if old_range.contains(&anchor.position.item_ix) {
                anchor.position = ListOffset {
                    item_ix: old_range.start,
                    offset_in_item: px(0.),
                };
                anchor.pending = None;
            } else if old_range.end <= anchor.position.item_ix {
                anchor.position.item_ix = anchor.position.item_ix - old_range.len() + spliced_count;
            }
        }
    }

    /// Set a handler that will be called when the list is scrolled.
    pub fn set_scroll_handler(
        &self,
        handler: impl FnMut(&ListScrollEvent, &mut Window, &mut App) + 'static,
    ) {
        self.0.borrow_mut().scroll_handler = Some(Box::new(handler))
    }

    /// Get the current scroll offset, in terms of the list's items.
    pub fn logical_scroll_top(&self) -> ListOffset {
        self.0.borrow().logical_scroll_top()
    }

    /// Scroll the list by the given offset
    pub fn scroll_by(&self, distance: Pixels) {
        if distance == px(0.) {
            return;
        }

        let current_offset = self.logical_scroll_top();
        let state = &mut *self.0.borrow_mut();

        if distance < px(0.) {
            state.follow_state.stop_following();
        }

        let mut cursor = state.items.cursor::<ListItemSummary>(());
        cursor.seek(&Count(current_offset.item_ix), Bias::Right);

        let start_pixel_offset = cursor.start().height + current_offset.offset_in_item;
        let new_pixel_offset = (start_pixel_offset + distance).max(px(0.));
        if new_pixel_offset > start_pixel_offset {
            cursor.seek_forward(&Height(new_pixel_offset), Bias::Right);
        } else {
            cursor.seek(&Height(new_pixel_offset), Bias::Right);
        }

        let scroll_top = ListOffset {
            item_ix: cursor.start().count,
            offset_in_item: new_pixel_offset - cursor.start().height,
        };
        drop(cursor);
        state.set_scroll_anchor(Some(scroll_top));
    }

    /// Scroll the list to the very end (past the last item).
    ///
    /// Unlike [`scroll_to_reveal_item`], this uses the total item count as the
    /// anchor, so the list's layout pass will walk backwards from the end and
    /// always show the bottom of the last item — even when that item is still
    /// growing (e.g. during streaming).
    pub fn scroll_to_end(&self) {
        let state = &mut *self.0.borrow_mut();
        let item_count = state.items.summary().count;
        state.set_scroll_anchor(Some(ListOffset {
            item_ix: item_count,
            offset_in_item: px(0.),
        }));
    }

    /// Set the follow mode for the list. In `Tail` mode, the list
    /// will auto-scroll to the end and re-engage after the user
    /// scrolls back to the bottom. In `Normal` mode, no automatic
    /// following occurs.
    pub fn set_follow_mode(&self, mode: FollowMode) {
        if mode == FollowMode::Normal {
            self.pause_following_tail();
        }
        let state = &mut *self.0.borrow_mut();

        match mode {
            FollowMode::Normal => {
                state.follow_state = FollowState::Normal;
            }
            FollowMode::Tail => {
                state.follow_state = FollowState::Tail { is_following: true };
                if matches!(mode, FollowMode::Tail) {
                    let item_count = state.items.summary().count;
                    state.set_scroll_anchor(Some(ListOffset {
                        item_ix: item_count,
                        offset_in_item: px(0.),
                    }));
                }
            }
        }
    }

    /// Pause tail-following, freezing the list at its current scroll
    /// position. Unlike [`Self::set_follow_mode`] with [`FollowMode::Normal`],
    /// this keeps the list in `Tail` mode, so it will resume following
    /// automatically once the view returns to the bottom. No-op when the list
    /// isn't currently following.
    ///
    /// Useful when something other than the user grows an item (e.g. zooming a
    /// diagram) and the current position should stay put rather than snapping
    /// to the end.
    pub fn pause_following_tail(&self) {
        let state = &mut *self.0.borrow_mut();
        if state.follow_state.is_following()
            && let Some(anchor) = state.scroll_anchor.as_mut()
        {
            anchor.pending_end = false;
        }
        state.follow_state.stop_following();
    }

    /// Sets a predicate queried after item layout to determine whether the measured height is stable.
    /// Measurements reported as `false` remain provisional and are retried on later layouts.
    /// The consumer must request a new frame when measurement becomes ready.
    pub fn set_item_measurement_readiness(&self, ready: impl Fn(usize, &App) -> bool + 'static) {
        self.0.borrow_mut().item_measurement_ready = Some(Box::new(ready));
    }

    /// Returns whether the list is currently actively following the
    /// tail (snapping to the end on each layout).
    pub fn is_following_tail(&self) -> bool {
        matches!(
            self.0.borrow().follow_state,
            FollowState::Tail { is_following: true }
        )
    }

    /// Scroll the list to the given offset
    pub fn scroll_to(&self, mut scroll_top: ListOffset) {
        let state = &mut *self.0.borrow_mut();
        let item_count = state.items.summary().count;
        if scroll_top.item_ix >= item_count {
            scroll_top.item_ix = item_count;
            scroll_top.offset_in_item = px(0.);
        }

        if scroll_top.item_ix < item_count {
            state.follow_state.stop_following();
        }

        state.set_scroll_anchor(Some(scroll_top));
    }

    /// Scroll the list to the given item, such that the item is fully visible.
    pub fn scroll_to_reveal_item(&self, ix: usize) {
        let state = &mut *self.0.borrow_mut();

        let mut scroll_top = state.logical_scroll_top();
        let height = state
            .last_layout_bounds
            .map_or(px(0.), |bounds| bounds.size.height);
        let padding = state.last_padding.unwrap_or_default();

        if ix <= scroll_top.item_ix {
            scroll_top.item_ix = ix;
            scroll_top.offset_in_item = px(0.);
        } else {
            let mut cursor = state.items.cursor::<ListItemSummary>(());
            cursor.seek(&Count(ix + 1), Bias::Right);
            let bottom = cursor.start().height + padding.top;
            let goal_top = px(0.).max(bottom - height + padding.bottom);

            cursor.seek(&Height(goal_top), Bias::Left);
            let start_ix = cursor.start().count;
            let start_item_top = cursor.start().height;

            if start_ix >= scroll_top.item_ix {
                scroll_top.item_ix = start_ix;
                scroll_top.offset_in_item = goal_top - start_item_top;
            }
        }

        state.set_scroll_anchor(Some(scroll_top));
    }

    /// Get the bounds for the given item in window coordinates, if it's
    /// been rendered.
    pub fn bounds_for_item(&self, ix: usize) -> Option<Bounds<Pixels>> {
        let state = &*self.0.borrow();

        let bounds = state.last_layout_bounds.unwrap_or_default();
        let scroll_top = state.logical_scroll_top();
        if ix < scroll_top.item_ix {
            return None;
        }

        let mut cursor = state.items.cursor::<Dimensions<Count, Height>>(());
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right);

        let scroll_top = cursor.start().1.0 + scroll_top.offset_in_item;

        cursor.seek_forward(&Count(ix), Bias::Right);
        if let Some(&ListItem::Measured { size, .. }) = cursor.item() {
            let &Dimensions(Count(count), Height(top), _) = cursor.start();
            if count == ix {
                let top = bounds.top() + top - scroll_top;
                return Some(Bounds::from_corners(
                    point(bounds.left(), top),
                    point(bounds.right(), top + size.height),
                ));
            }
        }
        None
    }

    /// Call this method when the user starts dragging the scrollbar.
    ///
    /// This will prevent the height reported to the scrollbar from changing during the drag
    /// as items in the overdraw get measured, and help offset scroll position changes accordingly.
    pub fn scrollbar_drag_started(&self) {
        let mut state = self.0.borrow_mut();
        state.scrollbar_drag_start_height = Some(state.items.summary().height);
    }

    /// Called when the user stops dragging the scrollbar.
    ///
    /// See `scrollbar_drag_started`.
    pub fn scrollbar_drag_ended(&self) {
        self.0.borrow_mut().scrollbar_drag_start_height.take();
    }

    /// Returns `true` if the scrollbar is currently being dragged.
    ///
    /// This is set between [`scrollbar_drag_started`](Self::scrollbar_drag_started)
    /// and [`scrollbar_drag_ended`](Self::scrollbar_drag_ended) calls. Useful for
    /// consumers that need to distinguish scrollbar drags from wheel/trackpad scrolls,
    /// e.g. to suppress auto-scroll behavior during manual positioning.
    pub fn is_scrollbar_dragging(&self) -> bool {
        self.0.borrow().scrollbar_drag_start_height.is_some()
    }

    /// Set the offset from the scrollbar
    pub fn set_offset_from_scrollbar(&self, point: Point<Pixels>) {
        self.0.borrow_mut().set_offset_from_scrollbar(point);
    }

    /// Returns the maximum scroll offset according to the items we have measured.
    /// This value remains constant while dragging to prevent the scrollbar from moving away unexpectedly.
    pub fn max_offset_for_scrollbar(&self) -> Point<Pixels> {
        let state = self.0.borrow();
        point(Pixels::ZERO, state.max_scroll_offset())
    }

    /// Returns the current scroll offset adjusted for the scrollbar.
    ///
    /// The returned offset has a negative `y` component representing
    /// how far the content has scrolled.
    pub fn scroll_px_offset_for_scrollbar(&self) -> Point<Pixels> {
        let state = &self.0.borrow();

        if state.scroll_anchor.is_none() && state.alignment == ListAlignment::Bottom {
            return Point::new(px(0.), -state.max_scroll_offset());
        }

        let logical_scroll_top = state.logical_scroll_top();

        let mut cursor = state.items.cursor::<ListItemSummary>(());
        let summary: ListItemSummary =
            cursor.summary(&Count(logical_scroll_top.item_ix), Bias::Right);
        let offset = summary.height + logical_scroll_top.offset_in_item;

        Point::new(px(0.), -offset)
    }

    /// Return the bounds of the viewport in pixels.
    pub fn viewport_bounds(&self) -> Bounds<Pixels> {
        self.0.borrow().last_layout_bounds.unwrap_or_default()
    }

    /// Returns whether the item is entirely above the viewport, or `None` if
    /// the list has not measured enough layout to know.
    ///
    /// A zero-height viewport still yields a definitive answer: callers may
    /// size sibling UI based on this query (potentially squeezing the list
    /// itself to zero height), so returning `None` in that case would make
    /// the answer oscillate from frame to frame.
    pub fn item_is_above_viewport(&self, ix: usize) -> Option<bool> {
        let viewport_bounds = self.0.borrow().last_layout_bounds?;

        let scroll_top = self.logical_scroll_top();
        if ix < scroll_top.item_ix {
            // Rows before the logical scroll top have no item bounds, but
            // their position relative to the viewport is known from scroll state.
            return Some(true);
        }

        let item_bounds = self.bounds_for_item(ix)?;
        Some(item_bounds.bottom() <= viewport_bounds.top())
    }

    /// Returns whether the item is entirely below the viewport, or `None` if
    /// the list has not measured enough layout to know.
    ///
    /// See [`Self::item_is_above_viewport`] for why a zero-height viewport
    /// still yields a definitive answer.
    pub fn item_is_below_viewport(&self, ix: usize) -> Option<bool> {
        let viewport_bounds = self.0.borrow().last_layout_bounds?;

        let scroll_top = self.logical_scroll_top();
        if ix < scroll_top.item_ix {
            // Rows before the logical scroll top have no item bounds, but
            // their position relative to the viewport is known from scroll state.
            return Some(false);
        }

        let item_bounds = self.bounds_for_item(ix)?;
        Some(item_bounds.top() >= viewport_bounds.bottom())
    }
}

impl StateInner {
    fn max_scroll_offset(&self) -> Pixels {
        let bounds = self.last_layout_bounds.unwrap_or_default();
        let height = self
            .scrollbar_drag_start_height
            .unwrap_or_else(|| self.items.summary().height);
        (height - bounds.size.height).max(px(0.))
    }

    fn visible_range(
        items: &SumTree<ListItem>,
        height: Pixels,
        scroll_top: &ListOffset,
    ) -> Range<usize> {
        let mut cursor = items.cursor::<ListItemSummary>(());
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right);
        let start_y = cursor.start().height + scroll_top.offset_in_item;
        cursor.seek_forward(&Height(start_y + height), Bias::Left);
        scroll_top.item_ix..cursor.start().count + 1
    }

    fn scroll(
        &mut self,
        scroll_top: &ListOffset,
        height: Pixels,
        delta: Point<Pixels>,
        current_view: EntityId,
        window: &mut Window,
        cx: &mut App,
    ) {
        // Drop scroll events after a reset, since we can't calculate
        // the new logical scroll top without the item heights
        if self.reset || delta.y == px(0.) {
            return;
        }

        let padding = self.last_padding.unwrap_or_default();
        let scroll_max =
            (self.items.summary().height + padding.top + padding.bottom - height).max(px(0.));
        let new_scroll_top = (self.scroll_top(scroll_top) - delta.y)
            .max(px(0.))
            .min(scroll_max);

        if self.alignment == ListAlignment::Bottom && new_scroll_top == scroll_max {
            self.set_scroll_anchor(None);
        } else {
            let (start, ..) =
                self.items
                    .find::<ListItemSummary, _>((), &Height(new_scroll_top), Bias::Right);
            let scroll_top = ListOffset {
                item_ix: start.count,
                offset_in_item: new_scroll_top - start.height,
            };
            self.set_scroll_anchor(Some(scroll_top));
        }

        if delta.y > px(0.) {
            self.follow_state.stop_following();
        }

        if let Some(handler) = self.scroll_handler.as_mut() {
            let visible_range = Self::visible_range(&self.items, height, scroll_top);
            handler(
                &ListScrollEvent {
                    visible_range,
                    count: self.items.summary().count,
                    is_scrolled: self.scroll_anchor.is_some(),
                    is_following_tail: matches!(
                        self.follow_state,
                        FollowState::Tail { is_following: true }
                    ),
                },
                window,
                cx,
            );
        }

        cx.notify(current_view);
    }

    fn logical_scroll_top(&self) -> ListOffset {
        self.scroll_anchor
            .map(|anchor| anchor.position)
            .unwrap_or_else(|| match self.alignment {
                ListAlignment::Top => ListOffset {
                    item_ix: 0,
                    offset_in_item: px(0.),
                },
                ListAlignment::Bottom => ListOffset {
                    item_ix: self.items.summary().count,
                    offset_in_item: px(0.),
                },
            })
    }

    fn scroll_top(&self, logical_scroll_top: &ListOffset) -> Pixels {
        let (start, ..) = self.items.find::<ListItemSummary, _>(
            (),
            &Count(logical_scroll_top.item_ix),
            Bias::Right,
        );
        start.height + logical_scroll_top.offset_in_item
    }

    fn is_item_ready(&self, index: usize, cx: &App) -> bool {
        self.item_measurement_ready
            .as_ref()
            .is_none_or(|ready| ready(index, cx))
    }

    fn is_provisional(&self) -> bool {
        self.scroll_anchor.is_some_and(|anchor| anchor.provisional)
    }

    fn set_scroll_anchor(&mut self, position: Option<ListOffset>) {
        let Some(position) = position else {
            self.scroll_anchor = None;
            return;
        };
        let pending = self
            .scroll_anchor
            .and_then(|anchor| anchor.pending)
            .filter(|_| position.item_ix < self.items.summary().count)
            .and_then(|pending| match (pending, self.is_provisional()) {
                (PendingScroll::Absolute(_), _) | (_, true) => {
                    Some(PendingScroll::Absolute(position.offset_in_item))
                }
                (PendingScroll::Proportional(_), false) => {
                    let mut cursor = self.items.cursor::<Count>(());
                    cursor.seek(&Count(position.item_ix), Bias::Right);
                    cursor
                        .item()
                        .and_then(|item| item.size_hint())
                        .filter(|size| size.height > px(0.))
                        .map(|size| {
                            PendingScroll::Proportional(
                                (position.offset_in_item / size.height).clamp(0., 1.),
                            )
                        })
                }
            });
        self.scroll_anchor = Some(ScrollAnchor {
            position,
            pending,
            provisional: false,
            pending_end: false,
        });
    }

    fn layout_all_items(
        &mut self,
        available_width: Pixels,
        render_item: &mut RenderItemFn,
        window: &mut Window,
        cx: &mut App,
    ) {
        match &mut self.measuring_behavior {
            ListMeasuringBehavior::Visible => {
                return;
            }
            ListMeasuringBehavior::Measure(has_measured) => {
                if *has_measured {
                    return;
                }
                *has_measured = true;
            }
        }

        let mut cursor = self.items.cursor::<Count>(());
        let available_item_space = size(
            AvailableSpace::Definite(available_width),
            AvailableSpace::MinContent,
        );

        let mut measured_items = Vec::default();

        for (ix, item) in cursor.enumerate() {
            let size = item.size().unwrap_or_else(|| {
                let mut element = render_item(ix, window, cx);
                element.layout_as_root(available_item_space, window, cx)
            });

            measured_items.push(item.with_measurement(size, self.is_item_ready(ix, cx)));
        }

        self.items = SumTree::from_iter(measured_items, ());
        self.measuring_behavior =
            ListMeasuringBehavior::Measure(self.items.summary().unrendered_count == 0);
    }

    fn layout_items(
        &mut self,
        available_width: Option<Pixels>,
        available_height: Pixels,
        padding: &Edges<Pixels>,
        render_item: &mut RenderItemFn,
        window: &mut Window,
        cx: &mut App,
    ) -> LayoutItemsResponse {
        let old_items = self.items.clone();
        let mut measured_items = VecDeque::new();
        let mut item_layouts = VecDeque::new();
        let mut rendered_height = padding.top;
        let mut max_item_width = px(0.);
        let mut scroll_top = self.logical_scroll_top();

        if self.follow_state.is_following()
            || self.scroll_anchor.is_some_and(|anchor| anchor.pending_end)
        {
            scroll_top = ListOffset {
                item_ix: self.items.summary().count,
                offset_in_item: px(0.),
            };
            self.set_scroll_anchor(Some(scroll_top));
        }

        let preserve_anchor = scroll_top.item_ix < self.items.summary().count;
        if !preserve_anchor {
            self.follow_state.start_following();
        }
        let mut provisional = false;
        let mut rendered_focused_item = false;

        let available_item_space = size(
            available_width.map_or(AvailableSpace::MaxContent, |width| {
                AvailableSpace::Definite(width)
            }),
            AvailableSpace::MinContent,
        );

        let mut cursor = old_items.cursor::<Count>(());

        // Render items after the scroll top, including those in the trailing overdraw
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right);
        for (ix, item) in cursor.by_ref().enumerate() {
            let visible_height = rendered_height - scroll_top.offset_in_item;
            if visible_height >= available_height + self.overdraw {
                break;
            }

            let item_index = scroll_top.item_ix + ix;
            let (size, ready) = if visible_height >= available_height
                && let Some(size) = item.size()
            {
                (size, self.is_item_ready(item_index, cx))
            } else {
                let mut element = render_item(item_index, window, cx);
                let element_size = element.layout_as_root(available_item_space, window, cx);
                let ready = self.is_item_ready(item_index, cx);
                if ix == 0
                    && ready
                    && let Some(anchor) = self.scroll_anchor.as_mut()
                    && let Some(pending) = anchor.pending.take()
                {
                    scroll_top.offset_in_item = match pending {
                        PendingScroll::Absolute(offset) => offset.min(element_size.height),
                        PendingScroll::Proportional(fraction) => element_size.height * fraction,
                    };
                    anchor.position = scroll_top;
                }

                if visible_height < available_height {
                    item_layouts.push_back(ItemLayout {
                        index: item_index,
                        element,
                        size: element_size,
                    });
                    if item.contains_focused(window, cx) {
                        rendered_focused_item = true;
                    }
                }
                (element_size, ready)
            };

            rendered_height += size.height;
            max_item_width = max_item_width.max(size.width);
            provisional |= !ready;
            measured_items.push_back(item.with_measurement(size, ready));
            if !ready && visible_height >= available_height {
                break;
            }
        }
        rendered_height += padding.bottom;

        // Prepare to start walking upward from the item at the scroll top.
        cursor.seek(&Count(scroll_top.item_ix), Bias::Right);

        // If the rendered items do not fill the visible region, then adjust
        // the scroll top upward.
        if !provisional && rendered_height - scroll_top.offset_in_item < available_height {
            let forward_layout_count = item_layouts.len();
            let forward_focused = rendered_focused_item;
            while rendered_height < available_height {
                cursor.prev();
                if let Some(item) = cursor.item() {
                    let item_index = cursor.start().0;
                    let mut element = render_item(item_index, window, cx);
                    let element_size = element.layout_as_root(available_item_space, window, cx);
                    let ready = self.is_item_ready(item_index, cx);
                    provisional |= !ready;
                    rendered_height += element_size.height;
                    measured_items.push_front(item.with_measurement(element_size, ready));
                    item_layouts.push_front(ItemLayout {
                        index: item_index,
                        element,
                        size: element_size,
                    });
                    if item.contains_focused(window, cx) {
                        rendered_focused_item = true;
                    }
                    if preserve_anchor && provisional {
                        break;
                    }
                } else {
                    break;
                }
            }

            if preserve_anchor && provisional {
                item_layouts.drain(..item_layouts.len() - forward_layout_count);
                rendered_focused_item = forward_focused;
            } else {
                scroll_top = ListOffset {
                    item_ix: cursor.start().0,
                    offset_in_item: rendered_height - available_height,
                };

                match self.alignment {
                    ListAlignment::Top => {
                        scroll_top.offset_in_item = scroll_top.offset_in_item.max(px(0.));
                        self.set_scroll_anchor(Some(scroll_top));
                    }
                    ListAlignment::Bottom => {
                        scroll_top = ListOffset {
                            item_ix: cursor.start().0,
                            offset_in_item: rendered_height - available_height,
                        };
                        self.set_scroll_anchor(None);
                    }
                };
            }
        }

        if let Some(anchor) = self.scroll_anchor.as_mut() {
            anchor.provisional = preserve_anchor && provisional;
            anchor.pending_end = !preserve_anchor && provisional;
        }

        // Measure items in the leading overdraw
        if !provisional {
            let mut leading_overdraw = scroll_top.offset_in_item;
            while leading_overdraw < self.overdraw {
                cursor.prev();
                if let Some(item) = cursor.item() {
                    let size = if let ListItem::Measured { size, .. } = item {
                        *size
                    } else {
                        let mut element = render_item(cursor.start().0, window, cx);
                        element.layout_as_root(available_item_space, window, cx)
                    };

                    leading_overdraw += size.height;
                    let ready = self.is_item_ready(cursor.start().0, cx);
                    measured_items.push_front(item.with_measurement(size, ready));
                    if !ready {
                        break;
                    }
                } else {
                    break;
                }
            }
        }

        let measured_range = cursor.start().0..(cursor.start().0 + measured_items.len());
        let mut cursor = old_items.cursor::<Count>(());
        let mut new_items = cursor.slice(&Count(measured_range.start), Bias::Right);
        new_items.extend(measured_items, ());
        cursor.seek(&Count(measured_range.end), Bias::Right);
        new_items.append(cursor.suffix(), ());
        self.items = new_items;

        // If follow_tail mode is on but the user scrolled away
        // (is_following is false), check whether the current scroll
        // position has returned to the bottom.
        if !provisional && self.follow_state.has_stopped_following() {
            let padding = self.last_padding.unwrap_or_default();
            let total_height = self.items.summary().height + padding.top + padding.bottom;
            let scroll_offset = self.scroll_top(&scroll_top);
            if scroll_offset + available_height >= total_height - px(1.0) {
                self.follow_state.start_following();
            }
        }

        // If none of the visible items are focused, check if an off-screen item is focused
        // and include it to be rendered after the visible items so keyboard interaction continues
        // to work for it.
        if !rendered_focused_item {
            let mut cursor = self
                .items
                .filter::<_, Count>((), |summary| summary.has_focus_handles);
            cursor.next();
            while let Some(item) = cursor.item() {
                if item.contains_focused(window, cx) {
                    let item_index = cursor.start().0;
                    let mut element = render_item(cursor.start().0, window, cx);
                    let size = element.layout_as_root(available_item_space, window, cx);
                    item_layouts.push_back(ItemLayout {
                        index: item_index,
                        element,
                        size,
                    });
                    break;
                }
                cursor.next();
            }
        }

        LayoutItemsResponse {
            max_item_width,
            scroll_top,
            item_layouts,
        }
    }

    fn prepaint_items(
        &mut self,
        bounds: Bounds<Pixels>,
        padding: Edges<Pixels>,
        autoscroll: bool,
        render_item: &mut RenderItemFn,
        window: &mut Window,
        cx: &mut App,
    ) -> Result<LayoutItemsResponse, ListOffset> {
        window.transact(|window| {
            match self.measuring_behavior {
                ListMeasuringBehavior::Measure(has_measured) if !has_measured => {
                    self.layout_all_items(bounds.size.width, render_item, window, cx);
                }
                _ => {}
            }

            let mut layout_response = self.layout_items(
                Some(bounds.size.width),
                bounds.size.height,
                &padding,
                render_item,
                window,
                cx,
            );

            // Avoid honoring autoscroll requests from elements other than our children.
            window.take_autoscroll();

            // Only paint the visible items, if there is actually any space for them (taking padding into account)
            if bounds.size.height > padding.top + padding.bottom {
                let mut item_origin = bounds.origin + Point::new(px(0.), padding.top);
                item_origin.y -= layout_response.scroll_top.offset_in_item;
                for item in &mut layout_response.item_layouts {
                    window.with_content_mask(Some(ContentMask { bounds }), |window| {
                        item.element.prepaint_at(item_origin, window, cx);
                    });

                    if let Some(autoscroll_bounds) = window.take_autoscroll()
                        && autoscroll
                    {
                        if autoscroll_bounds.top() < bounds.top() {
                            let mut item_ix = item.index;
                            let mut offset_in_item = autoscroll_bounds.top() - item_origin.y;

                            // The requested top can sit above this item's own
                            // top. Walk into earlier items so the offset stays
                            // non-negative and no blank space appears above the
                            // list.
                            if offset_in_item < Pixels::ZERO {
                                let mut cursor = self.items.cursor::<Count>(());
                                cursor.seek(&Count(item_ix), Bias::Right);
                                while offset_in_item < Pixels::ZERO {
                                    cursor.prev();
                                    let Some(prev_item) = cursor.item() else {
                                        offset_in_item = Pixels::ZERO;
                                        break;
                                    };
                                    let size = prev_item.size().unwrap_or_else(|| {
                                        let mut element = render_item(cursor.start().0, window, cx);
                                        let item_available_size = size(
                                            bounds.size.width.into(),
                                            AvailableSpace::MinContent,
                                        );
                                        element.layout_as_root(item_available_size, window, cx)
                                    });
                                    item_ix = cursor.start().0;
                                    offset_in_item += size.height;
                                }
                            }

                            return Err(ListOffset {
                                item_ix,
                                offset_in_item,
                            });
                        } else if autoscroll_bounds.bottom() > bounds.bottom() {
                            let mut cursor = self.items.cursor::<Count>(());
                            cursor.seek(&Count(item.index), Bias::Right);
                            let mut height = bounds.size.height - padding.top - padding.bottom;

                            // Account for the height of the element down until the autoscroll bottom.
                            height -= autoscroll_bounds.bottom() - item_origin.y;

                            // Keep decreasing the scroll top until we fill all the available space.
                            while height > Pixels::ZERO {
                                cursor.prev();
                                let Some(item) = cursor.item() else { break };

                                let size = item.size().unwrap_or_else(|| {
                                    let mut item = render_item(cursor.start().0, window, cx);
                                    let item_available_size =
                                        size(bounds.size.width.into(), AvailableSpace::MinContent);
                                    item.layout_as_root(item_available_size, window, cx)
                                });
                                height -= size.height;
                            }

                            return Err(ListOffset {
                                item_ix: cursor.start().0,
                                offset_in_item: if height < Pixels::ZERO {
                                    -height
                                } else {
                                    Pixels::ZERO
                                },
                            });
                        }
                    }

                    item_origin.y += item.size.height;
                }
            } else {
                layout_response.item_layouts.clear();
            }

            Ok(layout_response)
        })
    }

    // Scrollbar support

    fn set_offset_from_scrollbar(&mut self, point: Point<Pixels>) {
        let Some(bounds) = self.last_layout_bounds else {
            return;
        };
        let height = bounds.size.height;

        let padding = self.last_padding.unwrap_or_default();
        // Scrollbar drag positions are computed from the content height
        // captured at drag start, so map them back using the same height.
        let content_height = self
            .scrollbar_drag_start_height
            .unwrap_or_else(|| self.items.summary().height);
        let scroll_max = (content_height + padding.top + padding.bottom - height).max(px(0.));
        let new_scroll_top = (-point.y).max(px(0.)).min(scroll_max);

        // If content grew during the drag, the frozen bottom is below the
        // live bottom. Treat dragging to the frozen end as resuming tail follow.
        let dragged_to_end =
            scroll_max > px(0.) && new_scroll_top >= (scroll_max - px(1.0)).max(px(0.));
        if dragged_to_end && matches!(self.follow_state, FollowState::Tail { .. }) {
            self.follow_state = FollowState::Tail { is_following: true };
            let item_count = self.items.summary().count;
            self.set_scroll_anchor(Some(ListOffset {
                item_ix: item_count,
                offset_in_item: px(0.),
            }));
            return;
        }

        self.follow_state.stop_following();

        if self.alignment == ListAlignment::Bottom && new_scroll_top == scroll_max {
            self.set_scroll_anchor(None);
        } else {
            let (start, _, _) =
                self.items
                    .find::<ListItemSummary, _>((), &Height(new_scroll_top), Bias::Right);

            let scroll_top = ListOffset {
                item_ix: start.count,
                offset_in_item: new_scroll_top - start.height,
            };
            self.set_scroll_anchor(Some(scroll_top));
        }
    }
}

impl std::fmt::Debug for ListItem {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Unmeasured { .. } => write!(f, "Unrendered"),
            Self::Measured { size, .. } => f.debug_struct("Rendered").field("size", size).finish(),
        }
    }
}

/// An offset into the list's items, in terms of the item index and the number
/// of pixels off the top left of the item.
#[derive(Debug, Clone, Copy, Default, PartialEq)]
pub struct ListOffset {
    /// The index of an item in the list
    pub item_ix: usize,
    /// The number of pixels to offset from the item index.
    pub offset_in_item: Pixels,
}

impl Element for List {
    type RequestLayoutState = ();
    type PrepaintState = ListPrepaintState;

    fn id(&self) -> Option<crate::ElementId> {
        None
    }

    fn source_location(&self) -> Option<&'static core::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (crate::LayoutId, Self::RequestLayoutState) {
        let layout_id = match self.sizing_behavior {
            ListSizingBehavior::Infer => {
                let mut style = Style::default();
                style.overflow.y = Overflow::Scroll;
                style.refine(&self.style);
                window.with_text_style(style.text_style().cloned(), |window| {
                    let state = &mut *self.state.0.borrow_mut();

                    let available_height = if let Some(last_bounds) = state.last_layout_bounds {
                        last_bounds.size.height
                    } else {
                        // If we don't have the last layout bounds (first render),
                        // we might just use the overdraw value as the available height to layout enough items.
                        state.overdraw
                    };
                    let padding = style.padding.to_pixels(
                        state.last_layout_bounds.unwrap_or_default().size.into(),
                        window.rem_size(),
                    );

                    let layout_response = state.layout_items(
                        None,
                        available_height,
                        &padding,
                        &mut self.render_item,
                        window,
                        cx,
                    );
                    let max_element_width = layout_response.max_item_width;

                    let summary = state.items.summary();
                    let total_height = summary.height;

                    window.request_measured_layout(
                        style,
                        move |known_dimensions, available_space, _window, _cx| {
                            let width =
                                known_dimensions
                                    .width
                                    .unwrap_or(match available_space.width {
                                        AvailableSpace::Definite(x) => x,
                                        AvailableSpace::MinContent | AvailableSpace::MaxContent => {
                                            max_element_width
                                        }
                                    });
                            let height = match available_space.height {
                                AvailableSpace::Definite(height) => total_height.min(height),
                                AvailableSpace::MinContent | AvailableSpace::MaxContent => {
                                    total_height
                                }
                            };
                            size(width, height)
                        },
                    )
                })
            }
            ListSizingBehavior::Auto => {
                let mut style = Style::default();
                style.refine(&self.style);
                window.with_text_style(style.text_style().cloned(), |window| {
                    window.request_layout(style, None, cx)
                })
            }
        };
        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> ListPrepaintState {
        let state = &mut *self.state.0.borrow_mut();
        state.reset = false;

        let mut style = Style::default();
        style.refine(&self.style);

        let hitbox = window.insert_hitbox(bounds, HitboxBehavior::Normal);

        // If the width of the list has changed, invalidate all cached item heights
        if state
            .last_layout_bounds
            .is_none_or(|last_bounds| last_bounds.size.width != bounds.size.width)
        {
            let new_items = SumTree::from_iter(
                state.items.iter().map(|item| ListItem::Unmeasured {
                    size_hint: None,
                    focus_handle: item.focus_handle(),
                }),
                (),
            );

            state.items = new_items;
            state.measuring_behavior.reset();
        }

        let padding = style
            .padding
            .to_pixels(bounds.size.into(), window.rem_size());
        let mut autoscroll = true;
        let layout = loop {
            match state.prepaint_items(
                bounds,
                padding,
                autoscroll,
                &mut self.render_item,
                window,
                cx,
            ) {
                Ok(layout) => break layout,
                Err(autoscroll_request) => {
                    state.set_scroll_anchor(Some(autoscroll_request));
                    autoscroll = false;
                }
            }
        };

        state.last_layout_bounds = Some(bounds);
        state.last_padding = Some(padding);
        ListPrepaintState { hitbox, layout }
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        _inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<crate::Pixels>,
        _: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        let current_view = window.current_view();

        // Register the scroll listener before painting children so that, in
        // the bubble phase (which runs in reverse registration order),
        // children's scroll-wheel handlers run first and can stop propagation
        // to prevent the list from scrolling. This matches the ordering of
        // div-based scroll containers.
        let list_state = self.state.clone();
        let height = bounds.size.height;
        let scroll_top = prepaint.layout.scroll_top;
        let hitbox_id = prepaint.hitbox.id;
        let mut accumulated_scroll_delta = ScrollDelta::default();
        window.on_mouse_event(move |event: &ScrollEvent, phase, window, cx| {
            if phase == DispatchPhase::Bubble && hitbox_id.should_handle_scroll(window) {
                accumulated_scroll_delta = accumulated_scroll_delta.coalesce(event.delta);
                let pixel_delta = accumulated_scroll_delta.pixel_delta(px(20.));
                list_state.0.borrow_mut().scroll(
                    &scroll_top,
                    height,
                    pixel_delta,
                    current_view,
                    window,
                    cx,
                )
            }
        });

        window.with_content_mask(Some(ContentMask { bounds }), |window| {
            for item in &mut prepaint.layout.item_layouts {
                item.element.paint(window, cx);
            }
        });
    }
}

impl IntoElement for List {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl Styled for List {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.style
    }
}

impl sum_tree::Item for ListItem {
    type Summary = ListItemSummary;

    fn summary(&self, _: ()) -> Self::Summary {
        match self {
            ListItem::Unmeasured {
                size_hint,
                focus_handle,
            } => ListItemSummary {
                count: 1,
                rendered_count: 0,
                unrendered_count: 1,
                height: if let Some(size) = size_hint {
                    size.height
                } else {
                    px(0.)
                },
                has_focus_handles: focus_handle.is_some(),
                has_unknown_height: size_hint.is_none(),
            },
            ListItem::Measured {
                size, focus_handle, ..
            } => ListItemSummary {
                count: 1,
                rendered_count: 1,
                unrendered_count: 0,
                height: size.height,
                has_focus_handles: focus_handle.is_some(),
                has_unknown_height: false,
            },
        }
    }
}

impl sum_tree::ContextLessSummary for ListItemSummary {
    fn zero() -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &Self) {
        self.count += summary.count;
        self.rendered_count += summary.rendered_count;
        self.unrendered_count += summary.unrendered_count;
        self.height += summary.height;
        self.has_focus_handles |= summary.has_focus_handles;
        self.has_unknown_height |= summary.has_unknown_height;
    }
}

impl<'a> sum_tree::Dimension<'a, ListItemSummary> for Count {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a ListItemSummary, _: ()) {
        self.0 += summary.count;
    }
}

impl<'a> sum_tree::Dimension<'a, ListItemSummary> for Height {
    fn zero(_cx: ()) -> Self {
        Default::default()
    }

    fn add_summary(&mut self, summary: &'a ListItemSummary, _: ()) {
        self.0 += summary.height;
    }
}

impl sum_tree::SeekTarget<'_, ListItemSummary, ListItemSummary> for Count {
    fn cmp(&self, other: &ListItemSummary, _: ()) -> std::cmp::Ordering {
        Ord::cmp(&self.0, &other.count)
    }
}

impl sum_tree::SeekTarget<'_, ListItemSummary, ListItemSummary> for Height {
    fn cmp(&self, other: &ListItemSummary, _: ()) -> std::cmp::Ordering {
        Ord::cmp(&self.0, &other.height)
    }
}

#[cfg(test)]
mod test {

    use gpui::{ScrollDelta, ScrollEvent};
    use std::cell::{Cell, RefCell};
    use std::rc::Rc;

    use crate::{
        self as gpui, AnyElement, AppContext, Bounds, Context, Element, FollowMode,
        InteractiveElement, IntoElement, ListOffset, ListState, Pixels, Render, Styled,
        TestAppContext, VisualTestContext, Window, canvas, div, list, point, px, size,
    };

    #[gpui::test]
    fn test_autoscroll_above_item_top_renders_items_above(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        let state = ListState::new(5, crate::ListAlignment::Top, px(10.));
        state.scroll_to(gpui::ListOffset {
            item_ix: 2,
            offset_in_item: px(0.),
        });

        struct TestView(ListState);
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                list(self.0.clone(), |ix, _, _| {
                    if ix == 2 {
                        // Request an autoscroll whose top sits 30px above item 2's
                        // own top, mimicking a scroll-margin overshoot.
                        canvas(
                            |bounds, window, _| {
                                window.request_autoscroll(Bounds::from_corners(
                                    point(bounds.left(), bounds.top() - px(30.)),
                                    point(bounds.right(), bounds.top() + px(5.)),
                                ));
                            },
                            |_, _, _, _| {},
                        )
                        .h(px(20.))
                        .w_full()
                        .into_any()
                    } else {
                        div().h(px(20.)).w_full().into_any()
                    }
                })
                .w_full()
                .h_full()
            }
        }

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(60.)), |_, cx| {
            cx.new(|_| TestView(state.clone())).into_any_element()
        });

        // 30px above item 2's top, with 20px items, lands 10px into item 0.
        let scroll_top = state.logical_scroll_top();
        assert!(
            scroll_top.offset_in_item >= px(0.),
            "offset_in_item must never be negative (would leave blank space above), got {:?}",
            scroll_top.offset_in_item,
        );
        assert_eq!(scroll_top.item_ix, 0);
        assert_eq!(scroll_top.offset_in_item, px(10.));
    }

    #[gpui::test]
    fn test_reset_after_paint_before_scroll(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        let state = ListState::new(5, crate::ListAlignment::Top, px(10.));

        // Ensure that the list is scrolled to the top
        state.scroll_to(gpui::ListOffset {
            item_ix: 0,
            offset_in_item: px(0.0),
        });

        struct TestView(ListState);
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                list(self.0.clone(), |_, _, _| {
                    div().h(px(10.)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        // Paint
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(20.)), |_, cx| {
            cx.new(|_| TestView(state.clone())).into_any_element()
        });

        // Reset
        state.reset(5);

        // And then receive a scroll event _before_ the next paint
        cx.simulate_event(ScrollEvent {
            position: point(px(1.), px(1.)),
            delta: ScrollDelta::Pixels(point(px(0.), px(-500.))),
            ..Default::default()
        });

        // Scroll position should stay at the top of the list
        assert_eq!(state.logical_scroll_top().item_ix, 0);
        assert_eq!(state.logical_scroll_top().offset_in_item, px(0.));
    }

    #[gpui::test]
    fn test_scroll_by_positive_and_negative_distance(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        let state = ListState::new(5, crate::ListAlignment::Top, px(10.));

        struct TestView(ListState);
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                list(self.0.clone(), |_, _, _| {
                    div().h(px(20.)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        // Paint
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(100.)), |_, cx| {
            cx.new(|_| TestView(state.clone())).into_any_element()
        });

        // Test positive distance: start at item 1, move down 30px
        state.scroll_by(px(30.));

        // Should move to item 2
        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 1);
        assert_eq!(offset.offset_in_item, px(10.));

        // Test negative distance: start at item 2, move up 30px
        state.scroll_by(px(-30.));

        // Should move back to item 1
        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 0);
        assert_eq!(offset.offset_in_item, px(0.));

        // Test zero distance
        state.scroll_by(px(0.));
        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 0);
        assert_eq!(offset.offset_in_item, px(0.));
    }

    #[gpui::test]
    fn test_child_scroll_handler_can_stop_list_scroll(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        let state = ListState::new(5, crate::ListAlignment::Top, px(10.));
        let child_saw_event = Rc::new(Cell::new(false));

        struct TestView {
            state: ListState,
            child_saw_event: Rc<Cell<bool>>,
        }
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                let child_saw_event = self.child_saw_event.clone();
                list(self.state.clone(), move |_, _, _| {
                    let child_saw_event = child_saw_event.clone();
                    div()
                        .h(px(20.))
                        .w_full()
                        .on_scroll(move |_, _, cx| {
                            child_saw_event.set(true);
                            cx.stop_propagation();
                        })
                        .into_any()
                })
                .w_full()
                .h_full()
            }
        }

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(100.)), |_, cx| {
            cx.new(|_| TestView {
                state: state.clone(),
                child_saw_event: child_saw_event.clone(),
            })
            .into_any_element()
        });

        cx.simulate_event(ScrollEvent {
            position: point(px(50.), px(10.)),
            delta: ScrollDelta::Pixels(point(px(0.), px(-30.))),
            ..Default::default()
        });

        assert!(
            child_saw_event.get(),
            "the child's scroll-wheel handler should run"
        );
        // The child stopped propagation, so the list must not have scrolled.
        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 0);
        assert_eq!(offset.offset_in_item, px(0.));
    }

    #[gpui::test]
    fn test_item_viewport_queries_return_none_before_layout(_cx: &mut TestAppContext) {
        let state = ListState::new(5, crate::ListAlignment::Top, px(10.)).measure_all();

        assert_eq!(state.item_is_above_viewport(0), None);
        assert_eq!(state.item_is_below_viewport(0), None);
    }

    #[gpui::test]
    fn test_item_viewport_queries_before_logical_scroll_top(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        let state = ListState::new(5, crate::ListAlignment::Top, px(10.)).measure_all();

        state.scroll_to(gpui::ListOffset {
            item_ix: 2,
            offset_in_item: px(0.),
        });
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(20.)), |_, cx| {
            cx.new(|_| TestListView(state.clone())).into_any_element()
        });

        assert_eq!(state.item_is_above_viewport(1), Some(true));
        assert_eq!(state.item_is_below_viewport(1), Some(false));
    }

    #[gpui::test]
    fn test_item_viewport_queries_measured_item_inside_viewport(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        let state = ListState::new(5, crate::ListAlignment::Top, px(10.)).measure_all();

        state.scroll_to(gpui::ListOffset {
            item_ix: 2,
            offset_in_item: px(0.),
        });
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(20.)), |_, cx| {
            cx.new(|_| TestListView(state.clone())).into_any_element()
        });

        assert_eq!(state.item_is_above_viewport(2), Some(false));
        assert_eq!(state.item_is_below_viewport(2), Some(false));
    }

    #[gpui::test]
    fn test_item_viewport_queries_measured_item_above_viewport(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        let state = ListState::new(5, crate::ListAlignment::Top, px(10.)).measure_all();

        state.scroll_to(gpui::ListOffset {
            item_ix: 2,
            offset_in_item: px(20.),
        });
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(20.)), |_, cx| {
            cx.new(|_| TestListView(state.clone())).into_any_element()
        });

        assert_eq!(state.item_is_above_viewport(2), Some(true));
        assert_eq!(state.item_is_below_viewport(2), Some(false));
    }

    #[gpui::test]
    fn test_item_viewport_queries_measured_item_below_viewport(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        let state = ListState::new(5, crate::ListAlignment::Top, px(10.)).measure_all();

        state.scroll_to(gpui::ListOffset {
            item_ix: 2,
            offset_in_item: px(0.),
        });
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(20.)), |_, cx| {
            cx.new(|_| TestListView(state.clone())).into_any_element()
        });

        assert_eq!(state.item_is_above_viewport(3), Some(false));
        assert_eq!(state.item_is_below_viewport(3), Some(true));
    }

    #[gpui::test]
    fn test_item_viewport_queries_remain_stable_with_zero_height_viewport(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        let state = ListState::new(5, crate::ListAlignment::Top, px(10.)).measure_all();

        state.scroll_to(gpui::ListOffset {
            item_ix: 2,
            offset_in_item: px(0.),
        });
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(20.)), |_, cx| {
            cx.new(|_| TestListView(state.clone())).into_any_element()
        });

        assert_eq!(state.item_is_above_viewport(3), Some(false));
        assert_eq!(state.item_is_below_viewport(3), Some(true));

        // Squeeze the list to zero height, e.g. because a sibling element
        // (sized based on the queries above) consumed all the space. The
        // answers must remain definitive rather than becoming `None`,
        // otherwise the sibling's size can oscillate between frames.
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(0.)), |_, cx| {
            cx.new(|_| TestListView(state.clone())).into_any_element()
        });

        assert_eq!(state.item_is_above_viewport(1), Some(true));
        assert_eq!(state.item_is_below_viewport(1), Some(false));
        assert_eq!(state.item_is_above_viewport(3), Some(false));
        assert_eq!(state.item_is_below_viewport(3), Some(true));
    }

    #[gpui::test]
    fn test_item_viewport_queries_after_scroll_to_end_before_layout(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        let state = ListState::new(5, crate::ListAlignment::Top, px(10.)).measure_all();

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(20.)), |_, cx| {
            cx.new(|_| TestListView(state.clone())).into_any_element()
        });

        state.scroll_to_end();

        assert_eq!(state.logical_scroll_top().item_ix, state.item_count());
        assert_eq!(state.item_is_above_viewport(0), Some(true));
        assert_eq!(state.item_is_below_viewport(0), Some(false));
    }

    #[gpui::test]
    fn test_measure_all_after_width_change(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        let state = ListState::new(10, crate::ListAlignment::Top, px(0.)).measure_all();

        struct TestView(ListState);
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                list(self.0.clone(), |_, _, _| {
                    div().h(px(50.)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        let view = cx.update(|_, cx| cx.new(|_| TestView(state.clone())));

        // First draw at width 100: all 10 items measured (total 500px).
        // Viewport is 200px, so max scroll offset should be 300px.
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });
        assert_eq!(state.max_offset_for_scrollbar().y, px(300.));

        // Second draw at a different width: items get invalidated.
        // Without the fix, max_offset would drop because unmeasured items
        // contribute 0 height.
        cx.draw(point(px(0.), px(0.)), size(px(200.), px(200.)), |_, _| {
            view.into_any_element()
        });
        assert_eq!(state.max_offset_for_scrollbar().y, px(300.));
    }

    #[gpui::test]
    fn test_remeasure(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        // Create a list with 10 items, each 100px tall. We'll keep a reference
        // to the item height so we can later change the height and assert how
        // `ListState` handles it.
        let item_height = Rc::new(Cell::new(100usize));
        let state = ListState::new(10, crate::ListAlignment::Top, px(10.));

        struct TestView {
            state: ListState,
            item_height: Rc<Cell<usize>>,
        }

        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                let height = self.item_height.get();
                list(self.state.clone(), move |_, _, _| {
                    div().h(px(height as f32)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        let state_clone = state.clone();
        let item_height_clone = item_height.clone();
        let view = cx.update(|_, cx| {
            cx.new(|_| TestView {
                state: state_clone,
                item_height: item_height_clone,
            })
        });

        // Simulate scrolling 40px inside the element with index 2. Since the
        // original item height is 100px, this equates to 40% inside the item.
        state.scroll_to(gpui::ListOffset {
            item_ix: 2,
            offset_in_item: px(40.),
        });

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });

        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 2);
        assert_eq!(offset.offset_in_item, px(40.));

        // Update the `item_height` to be 50px instead of 100px so we can assert
        // that the scroll position is proportionally preserved, that is,
        // instead of 40px from the top of item 2, it should be 20px, since the
        // item's height has been halved.
        item_height.set(50);
        state.remeasure();

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.into_any_element()
        });

        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 2);
        assert_eq!(offset.offset_in_item, px(20.));
    }

    #[gpui::test]
    fn test_remeasure_item_preserves_scroll_offset(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        let item_height = Rc::new(Cell::new(100usize));
        let state = ListState::new(20, crate::ListAlignment::Top, px(10.));

        struct TestView {
            state: ListState,
            item_height: Rc<Cell<usize>>,
        }

        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                let height = self.item_height.get();
                list(self.state.clone(), move |index, _, _| {
                    let height = if index == 5 { height } else { 100 };
                    div().h(px(height as f32)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        let state_clone = state.clone();
        let item_height_clone = item_height.clone();
        let view = cx.update(|_, cx| {
            cx.new(|_| TestView {
                state: state_clone,
                item_height: item_height_clone,
            })
        });

        state.scroll_to(gpui::ListOffset {
            item_ix: 5,
            offset_in_item: px(40.),
        });

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });

        item_height.set(200);
        state.remeasure_items(5..6);

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.into_any_element()
        });

        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 5);
        assert_eq!(offset.offset_in_item, px(40.));
    }

    #[gpui::test]
    fn test_provisional_item_heights_keep_scroll_anchor(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();
        let state = ListState::new(20, crate::ListAlignment::Top, px(100.));
        let painted = Rc::new(RefCell::new(Vec::new()));
        let first_rendered = Rc::new(Cell::new(None::<usize>));
        let ready = item_readiness(&state, false);
        state.set_follow_mode(FollowMode::Tail);
        state.scroll_to(list_offset(10, 40.));

        for (height, measurement_ready) in [(0., false), (100., false), (100., true)] {
            ready.set(measurement_ready);
            painted.borrow_mut().clear();
            first_rendered.set(None);
            draw_list(
                &state,
                200.,
                {
                    let painted = painted.clone();
                    let first_rendered = first_rendered.clone();
                    move |index| {
                        first_rendered.set(Some(first_rendered.get().unwrap_or(index).min(index)));
                        painted_item(index, height, &painted)
                    }
                },
                cx,
            );
            assert_eq!(state.logical_scroll_top(), list_offset(10, 40.));
            assert!(!state.is_following_tail());
            assert_eq!(
                first_rendered.get(),
                Some(if measurement_ready { 9 } else { 10 })
            );
            if height > 0. {
                assert_eq!(
                    painted.borrow().as_slice(),
                    &[
                        (10, px(-40.), px(60.)),
                        (11, px(60.), px(160.)),
                        (12, px(160.), px(260.)),
                    ]
                );
            }
        }
    }

    #[gpui::test]
    fn test_unready_measurements_preserve_pending_fraction(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();
        for (provisional_height, destination, final_height, expected_offset) in [
            (Some(200.), None, 200., 100.),
            (None, Some(40.), 200., 80.),
            (Some(200.), Some(50.), 200., 50.),
            (Some(0.), Some(40.), 20., 20.),
            (Some(10.), Some(40.), 100., 40.),
            (Some(200.), Some(50.), 300., 50.),
        ] {
            let state = ListState::new(10, crate::ListAlignment::Top, px(0.));
            let painted = Rc::new(RefCell::new(Vec::new()));
            let draw = |height, cx: &mut VisualTestContext| {
                painted.borrow_mut().clear();
                let painted = painted.clone();
                draw_list(
                    &state,
                    200.,
                    move |index| {
                        painted_item(index, if index == 5 { height } else { 100. }, &painted)
                    },
                    cx,
                );
            };
            let ready = item_readiness(&state, true);
            state.scroll_to(list_offset(5, 50.));
            draw(100., cx);
            ready.set(false);
            state.remeasure();
            if let Some(height) = provisional_height {
                for height in [0., height, height] {
                    draw(height, cx);
                    assert_eq!(state.logical_scroll_top(), list_offset(5, 50.));
                }
            }
            if let Some(offset) = destination {
                state.scroll_to(list_offset(5, offset));
            }
            ready.set(true);
            for _ in 0..2 {
                draw(final_height, cx);
                assert_eq!(state.logical_scroll_top(), list_offset(5, expected_offset));
                assert_eq!(
                    painted.borrow().first().copied(),
                    Some((5, px(-expected_offset), px(final_height - expected_offset)))
                );
            }
        }
    }

    #[gpui::test]
    fn test_navigation_cancels_provisional_and_paints_destination(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();
        for measured in [false, true] {
            let navigations: [(&str, fn(&ListState), usize, f32); 7] = [
                (
                    "scroll_to",
                    |state| state.scroll_to(list_offset(12, 0.)),
                    12,
                    0.,
                ),
                ("end", ListState::scroll_to_end, 19, 150.),
                ("down", |state| state.scroll_by(px(5000.)), 19, 150.),
                (
                    "up",
                    |state| state.scroll_by(px(-5000.)),
                    if measured { 0 } else { 6 },
                    0.,
                ),
                (
                    "reveal_below",
                    |state| state.scroll_to_reveal_item(10),
                    10,
                    150.,
                ),
                (
                    "reveal_above",
                    |state| state.scroll_to_reveal_item(0),
                    0,
                    0.,
                ),
                (
                    "follow_tail",
                    |state| state.set_follow_mode(FollowMode::Tail),
                    19,
                    150.,
                ),
            ];
            for (name, navigate, destination, top) in navigations {
                let state = ListState::new(20, crate::ListAlignment::Top, px(0.));
                let state = if measured { state.measure_all() } else { state };
                let painted = Rc::new(RefCell::new(Vec::new()));
                let draw = |cx: &mut VisualTestContext| {
                    painted.borrow_mut().clear();
                    let painted = painted.clone();
                    draw_list(
                        &state,
                        200.,
                        move |index| {
                            painted_item(index, if index == 5 { 0. } else { 50. }, &painted)
                        },
                        cx,
                    );
                };
                state.set_item_measurement_readiness(|index, _| index != 5);
                state.scroll_to(list_offset(5, 20.));
                state.remeasure_items(5..6);
                draw(cx);
                assert!(state.0.borrow().is_provisional(), "{name}");
                navigate(&state);
                assert!(!state.0.borrow().is_provisional(), "{name}");
                draw(cx);
                assert!(!state.0.borrow().is_provisional(), "{name}");
                assert_eq!(
                    painted
                        .borrow()
                        .iter()
                        .find(|(index, _, _)| *index == destination)
                        .copied(),
                    Some((destination, px(top), px(top + 50.))),
                    "{name}, measured={measured}",
                );
            }
        }

        let navigations: [(&str, fn(&ListState)); 6] = [
            ("end_sentinel", |state| state.scroll_to(list_offset(3, 0.))),
            ("scroll_to_end", ListState::scroll_to_end),
            ("follow_tail", |state| {
                state.set_follow_mode(FollowMode::Tail)
            }),
            ("horizontal", ListState::scroll_to_end),
            ("pause_tail", |state| {
                state.set_follow_mode(FollowMode::Tail)
            }),
            ("disable_tail", |state| {
                state.set_follow_mode(FollowMode::Tail)
            }),
        ];
        for (name, navigate) in navigations {
            let state = ListState::new(3, crate::ListAlignment::Top, px(0.)).measure_all();
            let ready = item_readiness(&state, false);
            let painted = Rc::new(RefCell::new(Vec::new()));
            let draw = |height, cx: &mut VisualTestContext| {
                painted.borrow_mut().clear();
                let painted = painted.clone();
                draw_list(
                    &state,
                    200.,
                    move |index| painted_item(index, height, &painted),
                    cx,
                );
            };
            navigate(&state);
            for (height, measurement_ready, expected) in [
                (
                    100.,
                    false,
                    [(1, px(0.), px(100.)), (2, px(100.), px(200.))],
                ),
                (
                    150.,
                    true,
                    [(1, px(-100.), px(50.)), (2, px(50.), px(200.))],
                ),
            ] {
                if measurement_ready {
                    match name {
                        "horizontal" => cx.simulate_event(ScrollEvent {
                            position: point(px(50.), px(100.)),
                            delta: ScrollDelta::Pixels(point(px(20.), px(0.))),
                            ..ScrollEvent::default()
                        }),
                        "pause_tail" => state.pause_following_tail(),
                        "disable_tail" => state.set_follow_mode(FollowMode::Normal),
                        _ => {}
                    }
                }
                ready.set(measurement_ready);
                draw(height, cx);
                if !measurement_ready {
                    assert_eq!(
                        state.scroll_px_offset_for_scrollbar().y,
                        px(-100.),
                        "{name}"
                    );
                    assert_eq!(state.logical_scroll_top(), list_offset(1, 0.), "{name}");
                    assert_eq!(state.item_is_above_viewport(2), None, "{name}");
                }
                let paused = name == "pause_tail" || name == "disable_tail";
                let expected = if measurement_ready && paused {
                    [(1, px(0.), px(150.)), (2, px(150.), px(300.))]
                } else {
                    expected
                };
                assert_eq!(
                    state.is_following_tail(),
                    name == "follow_tail" || (paused && !measurement_ready),
                    "{name}"
                );
                assert_eq!(painted.borrow().as_slice(), &expected, "{name}");
            }
            ready.set(false);
            navigate(&state);
            draw(150., cx);
            state.scroll_by(px(-20.));
            draw(150., cx);
            assert_eq!(state.logical_scroll_top(), list_offset(1, 80.), "{name}");
            assert!(!state.is_following_tail(), "{name}");
            assert_eq!(
                painted.borrow().as_slice(),
                &[(1, px(-80.), px(70.)), (2, px(70.), px(220.)),],
                "{name}"
            );
        }
    }

    #[gpui::test]
    fn test_unready_overdraw_and_backfill_preserve_scroll_anchor(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();
        for (overdraw, anchor, unready_item) in
            [(100., list_offset(0, 0.), 2), (0., list_offset(2, 40.), 1)]
        {
            let state = ListState::new(3, crate::ListAlignment::Top, px(overdraw));
            let ready = Rc::new(Cell::new(false));
            state.set_item_measurement_readiness({
                let ready = ready.clone();
                move |index, _| index != unready_item || ready.get()
            });
            let painted = Rc::new(RefCell::new(Vec::new()));
            let draw = |height, cx: &mut VisualTestContext| {
                painted.borrow_mut().clear();
                let painted = painted.clone();
                draw_list(
                    &state,
                    200.,
                    move |index| {
                        let height = if index == unready_item { height } else { 100. };
                        painted_item(index, height, &painted)
                    },
                    cx,
                );
            };
            state.set_follow_mode(FollowMode::Tail);
            state.scroll_to(anchor);
            draw(0., cx);
            assert_eq!(state.logical_scroll_top(), anchor);
            assert!(!state.is_following_tail());
            assert_eq!(
                *painted.borrow(),
                if unready_item == 2 {
                    vec![(0, px(0.), px(100.)), (1, px(100.), px(200.))]
                } else {
                    vec![(2, px(-40.), px(60.))]
                }
            );
            ready.set(true);
            draw(500., cx);
            if unready_item == 2 {
                assert_eq!(state.logical_scroll_top(), anchor);
                assert!(!state.is_following_tail());
                assert_eq!(state.max_offset_for_scrollbar().y, px(500.));
                assert_eq!(
                    painted.borrow().as_slice(),
                    &[(0, px(0.), px(100.)), (1, px(100.), px(200.))]
                );
            } else {
                assert_eq!(state.logical_scroll_top(), list_offset(1, 400.));
                assert!(state.is_following_tail());
                assert_eq!(
                    painted.borrow().as_slice(),
                    &[(1, px(-400.), px(100.)), (2, px(100.), px(200.))]
                );
            }
        }
    }

    #[gpui::test]
    fn test_unready_overdraw_stops_at_first_pending_item(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();
        for anchor in [0, 10] {
            let state = ListState::new(13, crate::ListAlignment::Top, px(100.));
            let ready = Rc::new(Cell::new(false));
            let initially_ready = move |index| {
                if anchor == 0 { index < 2 } else { index >= 10 }
            };
            state.set_item_measurement_readiness({
                let ready = ready.clone();
                move |index, _| initially_ready(index) || ready.get()
            });
            state.scroll_to(list_offset(anchor, 0.));
            let rendered = Rc::new(RefCell::new(Vec::new()));
            let painted = Rc::new(RefCell::new(Vec::new()));
            let expected_frames = if anchor == 0 {
                [vec![0, 1, 2], vec![0, 1, 2], vec![0, 1, 2], vec![0, 1]]
            } else {
                [
                    vec![10, 11, 12, 9],
                    vec![10, 11, 9],
                    vec![10, 11, 9],
                    vec![10, 11],
                ]
            };
            for (frame, expected_rendered) in expected_frames.into_iter().enumerate() {
                let measurement_ready = frame >= 2;
                ready.set(measurement_ready);
                rendered.borrow_mut().clear();
                painted.borrow_mut().clear();
                draw_list(
                    &state,
                    200.,
                    {
                        let rendered = rendered.clone();
                        let painted = painted.clone();
                        move |index| {
                            rendered.borrow_mut().push(index);
                            let height = if initially_ready(index) || measurement_ready {
                                100.
                            } else {
                                0.
                            };
                            painted_item(index, height, &painted)
                        }
                    },
                    cx,
                );
                assert_eq!(*rendered.borrow(), expected_rendered);
                assert_eq!(state.item_count(), 13);
                assert_eq!(state.logical_scroll_top(), list_offset(anchor, 0.));
                assert_eq!(
                    painted.borrow().as_slice(),
                    &[(anchor, px(0.), px(100.)), (anchor + 1, px(100.), px(200.))]
                );
            }
        }
    }

    #[gpui::test]
    fn test_scrollbar_and_reset_do_not_resurrect_provisional(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();
        let state = ListState::new(20, crate::ListAlignment::Top, px(0.));
        let painted = Rc::new(RefCell::new(Vec::new()));
        let draw = |cx: &mut VisualTestContext| {
            painted.borrow_mut().clear();
            let painted = painted.clone();
            draw_list(
                &state,
                200.,
                move |index| painted_item(index, if index == 5 { 0. } else { 50. }, &painted),
                cx,
            );
        };
        state.set_item_measurement_readiness(|index, _| index != 5);
        assert!(!state.0.borrow().is_provisional());
        state.set_follow_mode(FollowMode::Tail);
        for intervening_frame in [false, true] {
            state.scroll_to(list_offset(5, 20.));
            state.remeasure_items(5..6);
            draw(cx);
            state.scrollbar_drag_started();
            if intervening_frame {
                draw(cx);
                assert!(state.0.borrow().is_provisional());
            }
            state.set_offset_from_scrollbar(point(px(0.), px(-10_000.)));
            assert!(!state.0.borrow().is_provisional());
            state.scrollbar_drag_ended();
            assert!(!state.is_scrollbar_dragging());
            draw(cx);
            assert_eq!(
                painted.borrow().last().copied(),
                Some((19, px(150.), px(200.)))
            );
            assert!(!state.0.borrow().is_provisional());
            state.scroll_to(list_offset(5, 20.));
            assert!(!state.0.borrow().is_provisional());
            draw(cx);
            assert!(state.0.borrow().is_provisional());
            assert_eq!(state.logical_scroll_top(), list_offset(5, 20.));
        }
        state.remeasure_items(5..6);
        state.reset(20);
        assert!(!state.0.borrow().is_provisional());
        draw(cx);
        assert!(!state.0.borrow().is_provisional());
        assert_eq!(state.logical_scroll_top(), ListOffset::default());
        assert_eq!(
            painted.borrow().first().copied(),
            Some((0, px(0.), px(50.)))
        );
    }

    #[gpui::test]
    fn test_splice_preserves_only_surviving_scroll_offsets(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();
        let painted = Rc::new(RefCell::new(Vec::new()));
        let draw = |state: &ListState, height, cx: &mut VisualTestContext| {
            painted.borrow_mut().clear();
            let painted = painted.clone();
            draw_list(
                state,
                200.,
                move |index| painted_item(index, height, &painted),
                cx,
            );
        };
        for (provisional, replacement_count) in [(true, 0), (false, 1)] {
            let state = ListState::new(20, crate::ListAlignment::Top, px(0.));
            let ready = item_readiness(&state, !provisional);
            state.scroll_to(list_offset(8, 50.));
            state.remeasure_items(8..9);
            if provisional {
                draw(&state, 100., cx);
            }
            state.splice(8..9, replacement_count);
            assert!(!state.0.borrow().is_provisional());
            ready.set(true);
            draw(&state, 100., cx);
            assert_eq!(state.logical_scroll_top(), list_offset(8, 0.));
            assert_eq!(
                painted.borrow().first().copied(),
                Some((8, px(0.), px(100.)))
            );
        }
        let state = ListState::new(20, crate::ListAlignment::Top, px(0.)).measure_all();
        state.scroll_to(list_offset(8, 50.));
        draw(&state, 100., cx);
        state.remeasure();
        state.splice(0..2, 0);
        draw(&state, 80., cx);
        assert_eq!(state.logical_scroll_top(), list_offset(6, 40.));
        assert_eq!(
            painted.borrow().first().copied(),
            Some((6, px(-40.), px(40.)))
        );
        let ready = item_readiness(&state, false);
        state.remeasure_items(6..7);
        state.splice(8..10, 3);
        draw(&state, 100., cx);
        assert!(state.0.borrow().is_provisional());
        assert_eq!(state.logical_scroll_top(), list_offset(6, 40.));
        state.splice(0..0, 2);
        assert!(!state.0.borrow().is_provisional());
        draw(&state, 100., cx);
        assert!(state.0.borrow().is_provisional());
        assert_eq!(state.logical_scroll_top(), list_offset(8, 40.));
        ready.set(true);
        draw(&state, 100., cx);
        assert_eq!(state.logical_scroll_top(), list_offset(8, 40.));
        assert_eq!(
            painted.borrow().first().copied(),
            Some((8, px(-40.), px(60.)))
        );
    }

    #[gpui::test]
    fn test_child_autoscroll_overrides_pending_and_release_fills_viewport(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();
        for (count, child, request, expected, released) in [
            (10, 2, 40.0..45.0, list_offset(2, 40.), list_offset(2, 40.)),
            (3, 2, 40.0..45.0, list_offset(2, 40.), list_offset(1, 0.)),
            (10, 4, 80.0..90.0, list_offset(2, 90.), list_offset(2, 90.)),
            (10, 2, -130.0..5.0, list_offset(0, 70.), list_offset(0, 70.)),
            (10, 2, -250.0..5.0, list_offset(0, 0.), list_offset(0, 0.)),
        ] {
            let state = ListState::new(count, crate::ListAlignment::Top, px(0.));
            let requested = Rc::new(Cell::new(false));
            let painted = Rc::new(RefCell::new(Vec::new()));
            let draw = |cx: &mut VisualTestContext| {
                painted.borrow_mut().clear();
                let painted = painted.clone();
                let requested = requested.clone();
                let request = request.clone();
                draw_list(
                    &state,
                    200.,
                    move |index| {
                        let painted = painted.clone();
                        let requested = requested.clone();
                        let request = request.clone();
                        canvas(
                            move |bounds, window, _| {
                                if index == child && !requested.replace(true) {
                                    window.request_autoscroll(Bounds::from_corners(
                                        point(bounds.left(), bounds.top() + px(request.start)),
                                        point(bounds.right(), bounds.top() + px(request.end)),
                                    ));
                                }
                            },
                            move |bounds, _, _, _| {
                                painted
                                    .borrow_mut()
                                    .push((index, bounds.top(), bounds.bottom()));
                            },
                        )
                        .h(px(100.))
                        .w_full()
                        .into_any()
                    },
                    cx,
                );
            };
            let ready = item_readiness(&state, false);
            state.scroll_to(list_offset(2, 50.));
            state.remeasure_items(2..3);
            for _ in 0..2 {
                draw(cx);
                assert!(requested.get());
                assert!(state.0.borrow().is_provisional());
                assert_eq!(state.logical_scroll_top(), expected);
                assert_eq!(
                    painted.borrow().first().copied(),
                    Some((
                        expected.item_ix,
                        -expected.offset_in_item,
                        px(100.) - expected.offset_in_item,
                    ))
                );
            }
            ready.set(true);
            draw(cx);
            assert!(!state.0.borrow().is_provisional());
            assert_eq!(state.logical_scroll_top(), released);
            let visible_count = if released.offset_in_item == px(0.) {
                2
            } else {
                3
            };
            assert_eq!(
                painted.borrow().as_slice(),
                (0..visible_count)
                    .map(|index| {
                        let top = px(index as f32 * 100.) - released.offset_in_item;
                        (released.item_ix + index, top, top + px(100.))
                    })
                    .collect::<Vec<_>>()
                    .as_slice()
            );
        }
    }

    #[gpui::test]
    fn test_remeasure_then_scroll_does_not_revert_scroll_position(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        let state = ListState::new(20, crate::ListAlignment::Top, px(10.));

        struct TestView(ListState);
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                list(self.0.clone(), |_, _, _| {
                    div().h(px(100.)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        let view = {
            let state = state.clone();
            cx.update(|_, cx| cx.new(|_| TestView(state)))
        };

        state.scroll_to(gpui::ListOffset {
            item_ix: 5,
            offset_in_item: px(40.),
        });

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });

        state.remeasure_items(5..6);

        cx.simulate_event(ScrollEvent {
            position: point(px(50.), px(100.)),
            delta: ScrollDelta::Pixels(point(px(0.), px(-30.))),
            ..Default::default()
        });

        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 5);
        assert_eq!(offset.offset_in_item, px(70.));

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.into_any_element()
        });

        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 5);
        assert_eq!(
            offset.offset_in_item,
            px(70.),
            "scrolling after a remeasure should not be reverted by the stale pending scroll"
        );
    }

    #[gpui::test]
    fn test_scroll_after_remeasure_clamps_to_shrunk_item_height(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();
        for provisional in [false, true] {
            let state = ListState::new(20, crate::ListAlignment::Top, px(10.));
            let painted = Rc::new(RefCell::new(Vec::new()));
            let draw = |height, cx: &mut VisualTestContext| {
                painted.borrow_mut().clear();
                let painted = painted.clone();
                draw_list(
                    &state,
                    200.,
                    move |index| {
                        painted_item(index, if index == 5 { height } else { 100. }, &painted)
                    },
                    cx,
                );
            };
            let ready = item_readiness(&state, true);
            state.scroll_to(list_offset(5, 40.));
            draw(100., cx);
            ready.set(!provisional);
            state.remeasure_items(5..6);
            if provisional {
                draw(100., cx);
            }
            ready.set(true);
            cx.simulate_event(ScrollEvent {
                position: point(px(50.), px(100.)),
                delta: ScrollDelta::Pixels(point(px(0.), px(-30.))),
                ..ScrollEvent::default()
            });
            assert_eq!(state.logical_scroll_top(), list_offset(5, 70.));
            for _ in 0..2 {
                draw(50., cx);
                assert_eq!(state.logical_scroll_top(), list_offset(5, 50.));
                assert_eq!(
                    painted
                        .borrow()
                        .iter()
                        .find(|(index, _, _)| *index == 6)
                        .copied(),
                    Some((6, px(0.), px(100.)))
                );
            }
        }
    }

    #[gpui::test]
    fn test_follow_tail_stays_at_bottom_as_items_grow(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        // 10 items, each 50px tall → 500px total content, 200px viewport.
        // With follow-tail on, the list should always show the bottom.
        let item_height = Rc::new(Cell::new(50usize));
        let state = ListState::new(10, crate::ListAlignment::Top, px(0.));

        struct TestView {
            state: ListState,
            item_height: Rc<Cell<usize>>,
        }
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                let height = self.item_height.get();
                list(self.state.clone(), move |_, _, _| {
                    div().h(px(height as f32)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        let state_clone = state.clone();
        let item_height_clone = item_height.clone();
        let view = cx.update(|_, cx| {
            cx.new(|_| TestView {
                state: state_clone,
                item_height: item_height_clone,
            })
        });

        state.set_follow_mode(FollowMode::Tail);

        // First paint — items are 50px, total 500px, viewport 200px.
        // Follow-tail should anchor to the end.
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });

        // The scroll should be at the bottom: the last visible items fill the
        // 200px viewport from the end of 500px of content (offset 300px).
        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 6);
        assert_eq!(offset.offset_in_item, px(0.));
        assert!(state.is_following_tail());

        // Simulate items growing (e.g. streaming content makes each item taller).
        // 10 items × 80px = 800px total.
        item_height.set(80);
        state.remeasure();

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.into_any_element()
        });

        // After growth, follow-tail should have re-anchored to the new end.
        // 800px total − 200px viewport = 600px offset → item 7 at offset 40px,
        // but follow-tail anchors to item_count (10), and layout walks back to
        // fill 200px, landing at item 7 (7 × 80 = 560, 800 − 560 = 240 > 200,
        // so item 8: 8 × 80 = 640, 800 − 640 = 160 < 200 → keeps walking →
        // item 7: offset = 800 − 200 = 600, item_ix = 600/80 = 7, remainder 40).
        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 7);
        assert_eq!(offset.offset_in_item, px(40.));
        assert!(state.is_following_tail());
    }

    #[gpui::test]
    fn test_pause_following_tail_reengages_when_still_at_bottom(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        // 10 items × 50px = 500px total, 200px viewport.
        let state = ListState::new(10, crate::ListAlignment::Top, px(0.));

        struct TestView(ListState);
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                list(self.0.clone(), |_, _, _| {
                    div().h(px(50.)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        let view = cx.update(|_, cx| cx.new(|_| TestView(state.clone())));
        state.set_follow_mode(FollowMode::Tail);

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });
        assert!(state.is_following_tail());

        // Pausing while the view is still at the bottom (e.g. a no-op zoom)
        // must not strand follow-tail: the next layout has to re-engage so the
        // invariant "at the bottom + new content => visible" is preserved.
        state.pause_following_tail();
        assert!(!state.is_following_tail());

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.into_any_element()
        });
        assert!(
            state.is_following_tail(),
            "pausing while at the bottom must re-engage follow-tail on the next layout"
        );
    }

    #[gpui::test]
    fn test_pause_following_tail_freezes_off_bottom(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        // 10 items, 200px viewport. Item height is adjustable to simulate a
        // diagram block growing/shrinking on zoom.
        let item_height = Rc::new(Cell::new(50usize));
        let state = ListState::new(10, crate::ListAlignment::Top, px(0.));

        struct TestView {
            state: ListState,
            item_height: Rc<Cell<usize>>,
        }
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                let height = self.item_height.get();
                list(self.state.clone(), move |_, _, _| {
                    div().h(px(height as f32)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        let view = cx.update(|_, cx| {
            cx.new(|_| TestView {
                state: state.clone(),
                item_height: item_height.clone(),
            })
        });
        state.set_follow_mode(FollowMode::Tail);

        // At the bottom: 500px content, 200px viewport → top at item 6.
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });
        assert_eq!(state.logical_scroll_top().item_ix, 6);
        assert!(state.is_following_tail());

        // Pause, then grow items (a zoom-in that pushes content below the fold).
        // The frozen top must stay put rather than snapping to the new end, and
        // following stays paused since we're no longer at the bottom.
        state.pause_following_tail();
        item_height.set(80);
        state.remeasure();
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });
        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 6);
        assert_eq!(offset.offset_in_item, px(0.));
        assert!(
            !state.is_following_tail(),
            "a paused list must not re-engage while the frozen top is off the bottom"
        );

        // Shrink back (zoom-out) so the frozen top once again reaches the
        // bottom: follow-tail must re-engage on its own.
        item_height.set(50);
        state.remeasure();
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.into_any_element()
        });
        assert!(
            state.is_following_tail(),
            "returning to the bottom must restore follow-tail"
        );
    }

    #[gpui::test]
    fn test_follow_tail_disengages_on_user_scroll(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        // 10 items × 50px = 500px total, 200px viewport.
        let state = ListState::new(10, crate::ListAlignment::Top, px(0.));

        struct TestView(ListState);
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                list(self.0.clone(), |_, _, _| {
                    div().h(px(50.)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        state.set_follow_mode(FollowMode::Tail);

        // Paint with follow-tail — scroll anchored to the bottom.
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, cx| {
            cx.new(|_| TestView(state.clone())).into_any_element()
        });
        assert!(state.is_following_tail());

        // Simulate the user scrolling up.
        // This should disengage follow-tail.
        cx.simulate_event(ScrollEvent {
            position: point(px(50.), px(100.)),
            delta: ScrollDelta::Pixels(point(px(0.), px(100.))),
            ..Default::default()
        });

        assert!(
            !state.is_following_tail(),
            "follow-tail should disengage when the user scrolls toward the start"
        );
    }

    #[gpui::test]
    fn test_follow_tail_disengages_on_scrollbar_reposition(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        // 10 items × 50px = 500px total, 200px viewport.
        let state = ListState::new(10, crate::ListAlignment::Top, px(0.)).measure_all();

        struct TestView(ListState);
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                list(self.0.clone(), |_, _, _| {
                    div().h(px(50.)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        let view = cx.update(|_, cx| cx.new(|_| TestView(state.clone())));

        state.set_follow_mode(FollowMode::Tail);

        // Paint with follow-tail — scroll anchored to the bottom.
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });
        assert!(state.is_following_tail());

        // Simulate the scrollbar moving the viewport to the middle.
        state.set_offset_from_scrollbar(point(px(0.), px(-150.)));

        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 3);
        assert_eq!(offset.offset_in_item, px(0.));
        assert!(
            !state.is_following_tail(),
            "follow-tail should disengage when the scrollbar manually repositions the list"
        );

        // A subsequent draw should preserve the user's manual position instead
        // of snapping back to the end.
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.into_any_element()
        });

        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 3);
        assert_eq!(offset.offset_in_item, px(0.));
    }

    #[gpui::test]
    fn test_scrollbar_drag_with_growing_content(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        let last_item_height = Rc::new(Cell::new(50usize));
        let state = ListState::new(10, crate::ListAlignment::Top, px(0.)).measure_all();

        struct TestView {
            state: ListState,
            last_item_height: Rc<Cell<usize>>,
        }
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                let last_item_height = self.last_item_height.clone();
                list(self.state.clone(), move |index, _, _| {
                    let height = if index == 9 {
                        last_item_height.get()
                    } else {
                        50
                    };
                    div().h(px(height as f32)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        let view = cx.update(|_, cx| {
            cx.new(|_| TestView {
                state: state.clone(),
                last_item_height: last_item_height.clone(),
            })
        });

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });

        state.scrollbar_drag_started();

        state.set_offset_from_scrollbar(point(px(0.), px(-150.)));
        let scrollbar_offset_before_growth = state.scroll_px_offset_for_scrollbar();

        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 3);
        assert_eq!(offset.offset_in_item, px(0.));

        last_item_height.set(550);
        state.remeasure_items(9..10);
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });

        assert_eq!(state.max_offset_for_scrollbar().y, px(300.));
        assert_eq!(
            state.scroll_px_offset_for_scrollbar(),
            scrollbar_offset_before_growth
        );

        state.set_offset_from_scrollbar(point(px(0.), px(-150.)));
        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 3);
        assert_eq!(offset.offset_in_item, px(0.));
    }

    #[gpui::test]
    fn test_set_follow_tail_snaps_to_bottom(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        // 10 items × 50px = 500px total, 200px viewport.
        let state = ListState::new(10, crate::ListAlignment::Top, px(0.));

        struct TestView(ListState);
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                list(self.0.clone(), |_, _, _| {
                    div().h(px(50.)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        let view = cx.update(|_, cx| cx.new(|_| TestView(state.clone())));

        // Scroll to the middle of the list (item 3).
        state.scroll_to(gpui::ListOffset {
            item_ix: 3,
            offset_in_item: px(0.),
        });

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });

        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 3);
        assert_eq!(offset.offset_in_item, px(0.));
        assert!(!state.is_following_tail());

        // Enable follow-tail — this should immediately snap the scroll anchor
        // to the end, like the user just sent a prompt.
        state.set_follow_mode(FollowMode::Tail);

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.into_any_element()
        });

        // After paint, scroll should be at the bottom.
        // 500px total − 200px viewport = 300px offset → item 6, offset 0.
        let offset = state.logical_scroll_top();
        assert_eq!(offset.item_ix, 6);
        assert_eq!(offset.offset_in_item, px(0.));
        assert!(state.is_following_tail());
    }

    #[gpui::test]
    fn test_bottom_aligned_scrollbar_offset_at_end(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        const ITEMS: usize = 10;
        const ITEM_SIZE: f32 = 50.0;

        let state = ListState::new(
            ITEMS,
            crate::ListAlignment::Bottom,
            px(ITEMS as f32 * ITEM_SIZE),
        );

        struct TestView(ListState);
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                list(self.0.clone(), |_, _, _| {
                    div().h(px(ITEM_SIZE)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(100.)), |_, cx| {
            cx.new(|_| TestView(state.clone())).into_any_element()
        });

        // Bottom-aligned lists start pinned to the end: logical_scroll_top returns
        // item_ix == item_count, meaning no explicit scroll position has been set.
        assert_eq!(state.logical_scroll_top().item_ix, ITEMS);

        let max_offset = state.max_offset_for_scrollbar();
        let scroll_offset = state.scroll_px_offset_for_scrollbar();

        assert_eq!(
            -scroll_offset.y, max_offset.y,
            "scrollbar offset ({}) should equal max offset ({}) when list is pinned to bottom",
            -scroll_offset.y, max_offset.y,
        );
    }

    /// When the user scrolls away from the bottom during follow_tail,
    /// follow_tail suspends. If they scroll back to the bottom, the
    /// next paint should re-engage follow_tail using fresh measurements.
    #[gpui::test]
    fn test_follow_tail_reengages_when_scrolled_back_to_bottom(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        // 10 items × 50px = 500px total, 200px viewport.
        let state = ListState::new(10, crate::ListAlignment::Top, px(0.));

        struct TestView(ListState);
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                list(self.0.clone(), |_, _, _| {
                    div().h(px(50.)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        let view = cx.update(|_, cx| cx.new(|_| TestView(state.clone())));

        state.set_follow_mode(FollowMode::Tail);

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });
        assert!(state.is_following_tail());

        // Scroll up — follow_tail should suspend (not fully disengage).
        cx.simulate_event(ScrollEvent {
            position: point(px(50.), px(100.)),
            delta: ScrollDelta::Pixels(point(px(0.), px(50.))),
            ..Default::default()
        });
        assert!(!state.is_following_tail());

        // Scroll back down to the bottom.
        cx.simulate_event(ScrollEvent {
            position: point(px(50.), px(100.)),
            delta: ScrollDelta::Pixels(point(px(0.), px(-10000.))),
            ..Default::default()
        });

        // After a paint, follow_tail should re-engage because the
        // layout confirmed we're at the true bottom.
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });
        assert!(
            state.is_following_tail(),
            "follow_tail should re-engage after scrolling back to the bottom"
        );
    }

    /// When an item is spliced to unmeasured (0px) while follow_tail
    /// is suspended, the re-engagement check should still work correctly
    #[gpui::test]
    fn test_follow_tail_reengagement_not_fooled_by_unmeasured_items(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        // 20 items × 50px = 1000px total, 200px viewport, 1000px
        // overdraw so all items get measured during the follow_tail
        // paint (matching realistic production settings).
        let state = ListState::new(20, crate::ListAlignment::Top, px(1000.));

        struct TestView(ListState);
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                list(self.0.clone(), |_, _, _| {
                    div().h(px(50.)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        let view = cx.update(|_, cx| cx.new(|_| TestView(state.clone())));

        state.set_follow_mode(FollowMode::Tail);

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });
        assert!(state.is_following_tail());

        // Scroll up a meaningful amount — suspends follow_tail.
        // 20 items × 50px = 1000px. viewport 200px. scroll_max = 800px.
        // Scrolling up 200px puts us at 600px, clearly not at bottom.
        cx.simulate_event(ScrollEvent {
            position: point(px(50.), px(100.)),
            delta: ScrollDelta::Pixels(point(px(0.), px(200.))),
            ..Default::default()
        });
        assert!(!state.is_following_tail());

        // Invalidate the last item (simulates EntryUpdated calling
        // remeasure_items). This makes items.summary().height
        // temporarily wrong (0px for the invalidated item).
        state.remeasure_items(19..20);

        // Paint — layout re-measures the invalidated item with its true
        // height. The re-engagement check uses these fresh measurements.
        // Since we scrolled 200px up from the 800px max, we're at
        // ~600px — NOT at the bottom, so follow_tail should NOT
        // re-engage.
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });
        assert!(
            !state.is_following_tail(),
            "follow_tail should not falsely re-engage due to an unmeasured item \
             reducing items.summary().height"
        );
    }

    #[gpui::test]
    fn test_follow_tail_reengages_after_scrollbar_disengagement(cx: &mut TestAppContext) {
        let cx = cx.add_empty_window();

        // 10 items × 50px = 500px total, 200px viewport, scroll_max = 300px.
        let state = ListState::new(10, crate::ListAlignment::Top, px(0.)).measure_all();

        struct TestView(ListState);
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                list(self.0.clone(), |_, _, _| {
                    div().h(px(50.)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        let view = cx.update(|_, cx| cx.new(|_| TestView(state.clone())));

        state.set_follow_mode(FollowMode::Tail);
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });
        assert!(state.is_following_tail());

        // Drag the scrollbar up to the middle — follow_tail should suspend.
        state.set_offset_from_scrollbar(point(px(0.), px(-150.)));
        assert!(!state.is_following_tail());

        // Drag the scrollbar back to the bottom — follow_tail should re-engage
        // on the next paint.
        state.set_offset_from_scrollbar(point(px(0.), px(-300.)));
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.into_any_element()
        });
        assert!(
            state.is_following_tail(),
            "follow_tail should re-engage after scrolling back to the bottom via the scrollbar"
        );
    }

    #[gpui::test]
    fn test_follow_tail_reengages_after_scrollbar_drag_to_bottom_while_growing(
        cx: &mut TestAppContext,
    ) {
        let cx = cx.add_empty_window();

        let state = ListState::new(10, crate::ListAlignment::Top, px(0.)).measure_all();

        struct TestView(ListState);
        impl Render for TestView {
            fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
                list(self.0.clone(), |_, _, _| {
                    div().h(px(50.)).w_full().into_any()
                })
                .w_full()
                .h_full()
            }
        }

        let view = cx.update(|_, cx| cx.new(|_| TestView(state.clone())));

        state.set_follow_mode(FollowMode::Tail);
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });
        assert!(state.is_following_tail());

        state.scrollbar_drag_started();

        state.splice(10..10, 10);
        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.clone().into_any_element()
        });

        state.set_offset_from_scrollbar(point(px(0.), px(-300.)));
        state.scrollbar_drag_ended();

        cx.draw(point(px(0.), px(0.)), size(px(100.), px(200.)), |_, _| {
            view.into_any_element()
        });

        assert!(
            state.is_following_tail(),
            "follow_tail should re-engage when the user drags the scrollbar to \
             the bottom of its track, even when content has grown during the drag \
             (so frozen_bottom < live_bottom)"
        );
    }

    fn draw_list(
        state: &ListState,
        viewport_height: f32,
        render_item: impl Fn(usize) -> AnyElement + 'static,
        cx: &mut VisualTestContext,
    ) {
        cx.draw(
            point(px(0.), px(0.)),
            size(px(100.), px(viewport_height)),
            |_, cx| {
                cx.new(|_| TestView {
                    state: state.clone(),
                    render_item: Rc::new(render_item),
                })
                .into_any_element()
            },
        );
    }

    fn painted_item(
        index: usize,
        height: f32,
        painted: &Rc<RefCell<Vec<(usize, Pixels, Pixels)>>>,
    ) -> AnyElement {
        let painted = painted.clone();
        canvas(
            |_, _, _| {},
            move |bounds, _, _, _| {
                painted
                    .borrow_mut()
                    .push((index, bounds.top(), bounds.bottom()));
            },
        )
        .h(px(height))
        .w_full()
        .into_any()
    }

    fn item_readiness(state: &ListState, ready: bool) -> Rc<Cell<bool>> {
        let ready = Rc::new(Cell::new(ready));
        state.set_item_measurement_readiness({
            let ready = ready.clone();
            move |_, _| ready.get()
        });
        ready
    }

    fn list_offset(item_ix: usize, offset: f32) -> ListOffset {
        ListOffset {
            item_ix,
            offset_in_item: px(offset),
        }
    }

    struct TestView {
        state: ListState,
        render_item: Rc<dyn Fn(usize) -> AnyElement>,
    }

    impl Render for TestView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            let render_item = self.render_item.clone();
            list(self.state.clone(), move |index, _, _| render_item(index))
                .w_full()
                .h_full()
        }
    }

    struct TestListView(ListState);
    impl Render for TestListView {
        fn render(&mut self, _: &mut Window, _: &mut Context<Self>) -> impl IntoElement {
            list(self.0.clone(), |_, _, _| {
                div().h(px(20.)).w_full().into_any()
            })
            .w_full()
            .h_full()
        }
    }
}
