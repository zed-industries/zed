//! A scrollable list of elements with uniform height, optimized for large lists.
//! Rather than use the full taffy layout system, uniform_list simply measures
//! the first element and then lays out all remaining elements in a line based on that
//! measurement. This is much faster than the full layout system, but only works for
//! elements with uniform height.

use crate::{
    AnyElement, App, AvailableSpace, Bounds, ContentMask, Context, Element, ElementId, Entity,
    GlobalElementId, Hitbox, InteractiveElement, Interactivity, IntoElement, IsZero, LayoutId,
    ListSizingBehavior, Pixels, Render, ScrollHandle, Size, StyleRefinement, Styled, Window, point,
    size,
};
use smallvec::SmallVec;
use std::{cell::RefCell, cmp, ops::Range, rc::Rc};
use taffy::style::Overflow;

use super::ListHorizontalSizingBehavior;

/// uniform_list provides lazy rendering for a set of items that are of uniform height.
/// When rendered into a container with overflow-y: hidden and a fixed (or max) height,
/// uniform_list will only render the visible subset of items.
#[track_caller]
pub fn uniform_list<I, R, V>(
    view: Entity<V>,
    id: I,
    item_count: usize,
    f: impl 'static + Fn(&mut V, Range<usize>, &mut Window, &mut Context<V>) -> Vec<R>,
) -> UniformList
where
    I: Into<ElementId>,
    R: IntoElement,
    V: Render,
{
    let id = id.into();
    let mut base_style = StyleRefinement::default();
    base_style.overflow.y = Some(Overflow::Scroll);

    let render_range = move |range, window: &mut Window, cx: &mut App| {
        view.update(cx, |this, cx| {
            f(this, range, window, cx)
                .into_iter()
                .map(|component| component.into_any_element())
                .collect()
        })
    };

    UniformList {
        item_count,
        item_to_measure_index: 0,
        render_items: Box::new(render_range),
        decorations: Vec::new(),
        interactivity: Interactivity {
            element_id: Some(id),
            base_style: Box::new(base_style),

            #[cfg(debug_assertions)]
            location: Some(*core::panic::Location::caller()),

            ..Default::default()
        },
        scroll_handle: None,
        sizing_behavior: ListSizingBehavior::default(),
        horizontal_sizing_behavior: ListHorizontalSizingBehavior::default(),
    }
}

/// A list element for efficiently laying out and displaying a list of uniform-height elements.
pub struct UniformList {
    item_count: usize,
    item_to_measure_index: usize,
    render_items: Box<
        dyn for<'a> Fn(Range<usize>, &'a mut Window, &'a mut App) -> SmallVec<[AnyElement; 64]>,
    >,
    decorations: Vec<Box<dyn UniformListDecoration>>,
    interactivity: Interactivity,
    scroll_handle: Option<UniformListScrollHandle>,
    sizing_behavior: ListSizingBehavior,
    horizontal_sizing_behavior: ListHorizontalSizingBehavior,
}

/// Frame state used by the [UniformList].
pub struct UniformListFrameState {
    items: SmallVec<[AnyElement; 32]>,
    decorations: SmallVec<[AnyElement; 1]>,
}

/// A handle for controlling the scroll position of a uniform list.
/// This should be stored in your view and passed to the uniform_list on each frame.
#[derive(Clone, Debug, Default)]
pub struct UniformListScrollHandle(pub Rc<RefCell<UniformListScrollState>>);

/// Where to place the element scrolled to.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ScrollStrategy {
    /// Place the element at the top of the list's viewport.
    Top,
    /// Attempt to place the element in the middle of the list's viewport.
    /// May not be possible if there's not enough list items above the item scrolled to:
    /// in this case, the element will be placed at the closest possible position.
    Center,
}

#[derive(Clone, Debug, Default)]
#[allow(missing_docs)]
pub struct UniformListScrollState {
    pub base_handle: ScrollHandle,
    pub deferred_scroll_to_item: Option<(usize, ScrollStrategy)>,
    /// Size of the item, captured during last layout.
    pub last_item_size: Option<ItemSize>,
    /// Whether the list was vertically flipped during last layout.
    pub y_flipped: bool,
}

#[derive(Copy, Clone, Debug, Default)]
/// The size of the item and its contents.
pub struct ItemSize {
    /// The size of the item.
    pub item: Size<Pixels>,
    /// The size of the item's contents, which may be larger than the item itself,
    /// if the item was bounded by a parent element.
    pub contents: Size<Pixels>,
}

impl UniformListScrollHandle {
    /// Create a new scroll handle to bind to a uniform list.
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(UniformListScrollState {
            base_handle: ScrollHandle::new(),
            deferred_scroll_to_item: None,
            last_item_size: None,
            y_flipped: false,
        })))
    }

    /// Scroll the list to the given item index.
    pub fn scroll_to_item(&self, ix: usize, strategy: ScrollStrategy) {
        self.0.borrow_mut().deferred_scroll_to_item = Some((ix, strategy));
    }

    /// Check if the list is flipped vertically.
    pub fn y_flipped(&self) -> bool {
        self.0.borrow().y_flipped
    }

    /// Get the index of the topmost visible child.
    #[cfg(any(test, feature = "test-support"))]
    pub fn logical_scroll_top_index(&self) -> usize {
        let this = self.0.borrow();
        this.deferred_scroll_to_item
            .map(|(ix, _)| ix)
            .unwrap_or_else(|| this.base_handle.logical_scroll_top().0)
    }
}

impl Styled for UniformList {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl Element for UniformList {
    type RequestLayoutState = UniformListFrameState;
    type PrepaintState = Option<Hitbox>;

    fn id(&self) -> Option<ElementId> {
        self.interactivity.element_id.clone()
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let max_items = self.item_count;
        let item_size = self.measure_item(None, window, cx);
        let layout_id = self.interactivity.request_layout(
            global_id,
            window,
            cx,
            |style, window, cx| match self.sizing_behavior {
                ListSizingBehavior::Infer => {
                    window.with_text_style(style.text_style().cloned(), |window| {
                        window.request_measured_layout(
                            style,
                            move |known_dimensions, available_space, _window, _cx| {
                                let desired_height = item_size.height * max_items;
                                let width = known_dimensions.width.unwrap_or(match available_space
                                    .width
                                {
                                    AvailableSpace::Definite(x) => x,
                                    AvailableSpace::MinContent | AvailableSpace::MaxContent => {
                                        item_size.width
                                    }
                                });
                                let height = match available_space.height {
                                    AvailableSpace::Definite(height) => desired_height.min(height),
                                    AvailableSpace::MinContent | AvailableSpace::MaxContent => {
                                        desired_height
                                    }
                                };
                                size(width, height)
                            },
                        )
                    })
                }
                ListSizingBehavior::Auto => window
                    .with_text_style(style.text_style().cloned(), |window| {
                        window.request_layout(style, None, cx)
                    }),
            },
        );

        (
            layout_id,
            UniformListFrameState {
                items: SmallVec::new(),
                decorations: SmallVec::new(),
            },
        )
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        frame_state: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Option<Hitbox> {
        let style = self
            .interactivity
            .compute_style(global_id, None, window, cx);
        let border = style.border_widths.to_pixels(window.rem_size());
        let padding = style
            .padding
            .to_pixels(bounds.size.into(), window.rem_size());

        let padded_bounds = Bounds::from_corners(
            bounds.origin + point(border.left + padding.left, border.top + padding.top),
            bounds.bottom_right()
                - point(border.right + padding.right, border.bottom + padding.bottom),
        );

        let can_scroll_horizontally = matches!(
            self.horizontal_sizing_behavior,
            ListHorizontalSizingBehavior::Unconstrained
        );

        let longest_item_size = self.measure_item(None, window, cx);
        let content_width = if can_scroll_horizontally {
            padded_bounds.size.width.max(longest_item_size.width)
        } else {
            padded_bounds.size.width
        };
        let content_size = Size {
            width: content_width,
            height: longest_item_size.height * self.item_count + padding.top + padding.bottom,
        };

        let shared_scroll_offset = self.interactivity.scroll_offset.clone().unwrap();
        let item_height = longest_item_size.height;
        let shared_scroll_to_item = self.scroll_handle.as_mut().and_then(|handle| {
            let mut handle = handle.0.borrow_mut();
            handle.last_item_size = Some(ItemSize {
                item: padded_bounds.size,
                contents: content_size,
            });
            handle.deferred_scroll_to_item.take()
        });

        self.interactivity.prepaint(
            global_id,
            bounds,
            content_size,
            window,
            cx,
            |style, mut scroll_offset, hitbox, window, cx| {
                let border = style.border_widths.to_pixels(window.rem_size());
                let padding = style
                    .padding
                    .to_pixels(bounds.size.into(), window.rem_size());

                let padded_bounds = Bounds::from_corners(
                    bounds.origin + point(border.left + padding.left, border.top),
                    bounds.bottom_right() - point(border.right + padding.right, border.bottom),
                );

                let y_flipped = if let Some(scroll_handle) = self.scroll_handle.as_mut() {
                    let mut scroll_state = scroll_handle.0.borrow_mut();
                    scroll_state.base_handle.set_bounds(bounds);
                    scroll_state.y_flipped
                } else {
                    false
                };

                if self.item_count > 0 {
                    let content_height =
                        item_height * self.item_count + padding.top + padding.bottom;
                    let is_scrolled_vertically = !scroll_offset.y.is_zero();
                    let min_vertical_scroll_offset = padded_bounds.size.height - content_height;
                    if is_scrolled_vertically && scroll_offset.y < min_vertical_scroll_offset {
                        shared_scroll_offset.borrow_mut().y = min_vertical_scroll_offset;
                        scroll_offset.y = min_vertical_scroll_offset;
                    }

                    let content_width = content_size.width + padding.left + padding.right;
                    let is_scrolled_horizontally =
                        can_scroll_horizontally && !scroll_offset.x.is_zero();
                    if is_scrolled_horizontally && content_width <= padded_bounds.size.width {
                        shared_scroll_offset.borrow_mut().x = Pixels::ZERO;
                        scroll_offset.x = Pixels::ZERO;
                    }

                    if let Some((mut ix, scroll_strategy)) = shared_scroll_to_item {
                        if y_flipped {
                            ix = self.item_count.saturating_sub(ix + 1);
                        }
                        let list_height = padded_bounds.size.height;
                        let mut updated_scroll_offset = shared_scroll_offset.borrow_mut();
                        let item_top = item_height * ix + padding.top;
                        let item_bottom = item_top + item_height;
                        let scroll_top = -updated_scroll_offset.y;
                        let mut scrolled_to_top = false;
                        if item_top < scroll_top + padding.top {
                            scrolled_to_top = true;
                            updated_scroll_offset.y = -(item_top) + padding.top;
                        } else if item_bottom > scroll_top + list_height - padding.bottom {
                            scrolled_to_top = true;
                            updated_scroll_offset.y = -(item_bottom - list_height) - padding.bottom;
                        }

                        match scroll_strategy {
                            ScrollStrategy::Top => {}
                            ScrollStrategy::Center => {
                                if scrolled_to_top {
                                    let item_center = item_top + item_height / 2.0;
                                    let target_scroll_top = item_center - list_height / 2.0;

                                    if item_top < scroll_top
                                        || item_bottom > scroll_top + list_height
                                    {
                                        updated_scroll_offset.y = -target_scroll_top
                                            .max(Pixels::ZERO)
                                            .min(content_height - list_height)
                                            .max(Pixels::ZERO);
                                    }
                                }
                            }
                        }
                        scroll_offset = *updated_scroll_offset
                    }

                    let first_visible_element_ix =
                        (-(scroll_offset.y + padding.top) / item_height).floor() as usize;
                    let last_visible_element_ix = ((-scroll_offset.y + padded_bounds.size.height)
                        / item_height)
                        .ceil() as usize;
                    let visible_range = first_visible_element_ix
                        ..cmp::min(last_visible_element_ix, self.item_count);

                    let items = if y_flipped {
                        let flipped_range = self.item_count.saturating_sub(visible_range.end)
                            ..self.item_count.saturating_sub(visible_range.start);
                        let mut items = (self.render_items)(flipped_range, window, cx);
                        items.reverse();
                        items
                    } else {
                        (self.render_items)(visible_range.clone(), window, cx)
                    };

                    let content_mask = ContentMask { bounds };
                    window.with_content_mask(Some(content_mask), |window| {
                        for (mut item, ix) in items.into_iter().zip(visible_range.clone()) {
                            let item_origin = padded_bounds.origin
                                + point(
                                    if can_scroll_horizontally {
                                        scroll_offset.x + padding.left
                                    } else {
                                        scroll_offset.x
                                    },
                                    item_height * ix + scroll_offset.y + padding.top,
                                );
                            let available_width = if can_scroll_horizontally {
                                padded_bounds.size.width + scroll_offset.x.abs()
                            } else {
                                padded_bounds.size.width
                            };
                            let available_space = size(
                                AvailableSpace::Definite(available_width),
                                AvailableSpace::Definite(item_height),
                            );
                            item.layout_as_root(available_space, window, cx);
                            item.prepaint_at(item_origin, window, cx);
                            frame_state.items.push(item);
                        }

                        let bounds = Bounds::new(
                            padded_bounds.origin
                                + point(
                                    if can_scroll_horizontally {
                                        scroll_offset.x + padding.left
                                    } else {
                                        scroll_offset.x
                                    },
                                    scroll_offset.y + padding.top,
                                ),
                            padded_bounds.size,
                        );
                        for decoration in &self.decorations {
                            let mut decoration = decoration.as_ref().compute(
                                visible_range.clone(),
                                bounds,
                                item_height,
                                self.item_count,
                                window,
                                cx,
                            );
                            let available_space = size(
                                AvailableSpace::Definite(bounds.size.width),
                                AvailableSpace::Definite(bounds.size.height),
                            );
                            decoration.layout_as_root(available_space, window, cx);
                            decoration.prepaint_at(bounds.origin, window, cx);
                            frame_state.decorations.push(decoration);
                        }
                    });
                }

                hitbox
            },
        )
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        bounds: Bounds<crate::Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        hitbox: &mut Option<Hitbox>,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.interactivity.paint(
            global_id,
            bounds,
            hitbox.as_ref(),
            window,
            cx,
            |_, window, cx| {
                for item in &mut request_layout.items {
                    item.paint(window, cx);
                }
                for decoration in &mut request_layout.decorations {
                    decoration.paint(window, cx);
                }
            },
        )
    }
}

impl IntoElement for UniformList {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

/// A decoration for a [`UniformList`]. This can be used for various things,
/// such as rendering indent guides, or other visual effects.
pub trait UniformListDecoration {
    /// Compute the decoration element, given the visible range of list items,
    /// the bounds of the list, and the height of each item.
    fn compute(
        &self,
        visible_range: Range<usize>,
        bounds: Bounds<Pixels>,
        item_height: Pixels,
        item_count: usize,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement;
}

impl UniformList {
    /// Selects a specific list item for measurement.
    pub fn with_width_from_item(mut self, item_index: Option<usize>) -> Self {
        self.item_to_measure_index = item_index.unwrap_or(0);
        self
    }

    /// Sets the sizing behavior, similar to the `List` element.
    pub fn with_sizing_behavior(mut self, behavior: ListSizingBehavior) -> Self {
        self.sizing_behavior = behavior;
        self
    }

    /// Sets the horizontal sizing behavior, controlling the way list items laid out horizontally.
    /// With [`ListHorizontalSizingBehavior::Unconstrained`] behavior, every item and the list itself will
    /// have the size of the widest item and lay out pushing the `end_slot` to the right end.
    pub fn with_horizontal_sizing_behavior(
        mut self,
        behavior: ListHorizontalSizingBehavior,
    ) -> Self {
        self.horizontal_sizing_behavior = behavior;
        match behavior {
            ListHorizontalSizingBehavior::FitList => {
                self.interactivity.base_style.overflow.x = None;
            }
            ListHorizontalSizingBehavior::Unconstrained => {
                self.interactivity.base_style.overflow.x = Some(Overflow::Scroll);
            }
        }
        self
    }

    /// Adds a decoration element to the list.
    pub fn with_decoration(mut self, decoration: impl UniformListDecoration + 'static) -> Self {
        self.decorations.push(Box::new(decoration));
        self
    }

    fn measure_item(
        &self,
        list_width: Option<Pixels>,
        window: &mut Window,
        cx: &mut App,
    ) -> Size<Pixels> {
        if self.item_count == 0 {
            return Size::default();
        }

        let item_ix = cmp::min(self.item_to_measure_index, self.item_count - 1);
        let mut items = (self.render_items)(item_ix..item_ix + 1, window, cx);
        let Some(mut item_to_measure) = items.pop() else {
            return Size::default();
        };
        let available_space = size(
            list_width.map_or(AvailableSpace::MinContent, |width| {
                AvailableSpace::Definite(width)
            }),
            AvailableSpace::MinContent,
        );
        item_to_measure.layout_as_root(available_space, window, cx)
    }

    /// Track and render scroll state of this list with reference to the given scroll handle.
    pub fn track_scroll(mut self, handle: UniformListScrollHandle) -> Self {
        self.interactivity.tracked_scroll_handle = Some(handle.0.borrow().base_handle.clone());
        self.scroll_handle = Some(handle);
        self
    }

    /// Sets whether the list is flipped vertically, such that item 0 appears at the bottom.
    pub fn y_flipped(mut self, y_flipped: bool) -> Self {
        if let Some(ref scroll_handle) = self.scroll_handle {
            let mut scroll_state = scroll_handle.0.borrow_mut();
            let mut base_handle = &scroll_state.base_handle;
            let offset = base_handle.offset();
            match scroll_state.last_item_size {
                Some(last_size) if scroll_state.y_flipped != y_flipped => {
                    let new_y_offset =
                        -(offset.y + last_size.contents.height - last_size.item.height);
                    base_handle.set_offset(point(offset.x, new_y_offset));
                    scroll_state.y_flipped = y_flipped;
                }
                // Handle case where list is initially flipped.
                None if y_flipped => {
                    base_handle.set_offset(point(offset.x, Pixels::MIN));
                    scroll_state.y_flipped = y_flipped;
                }
                _ => {}
            }
        }
        self
    }
}

impl InteractiveElement for UniformList {
    fn interactivity(&mut self) -> &mut crate::Interactivity {
        &mut self.interactivity
    }
}
