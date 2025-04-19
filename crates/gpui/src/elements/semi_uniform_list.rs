//! A scrollable list of elements with known but varying heights, optimized for large lists.
//! Similar to `uniform_list`, but allows elements with different heights.
//! This sits between the full Taffy layout system and the strict uniform height requirement
//! of `uniform_list`.

use crate::{
    AnyElement, App, AvailableSpace, Bounds, ContentMask, Context, Element, ElementId, Entity,
    GlobalElementId, Hitbox, InteractiveElement, Interactivity, IntoElement, IsZero, LayoutId,
    ListSizingBehavior, Pixels, Render, ScrollHandle, Size, StyleRefinement, Styled, Window, point,
    size,
};
use smallvec::SmallVec;
use std::{cell::RefCell, cmp, ops::Range, rc::Rc};
use taffy::style::Overflow;

use super::{uniform_list::ScrollStrategy, ListHorizontalSizingBehavior};

/// semi_uniform_list provides lazy rendering for a set of items that have known heights.
/// Similar to uniform_list, but supports items with varying heights.
/// 
/// The heights must be provided through a function that returns the height for a given item index.
/// This allows the list to calculate positions and visible items efficiently without doing full layout.
#[track_caller]
pub fn semi_uniform_list<I, R, V>(
    view: Entity<V>,
    id: I,
    item_count: usize,
    item_heights: impl Fn(usize) -> Pixels + 'static,
    f: impl 'static + Fn(&mut V, Range<usize>, &mut Window, &mut Context<V>) -> Vec<R>,
) -> SemiUniformList
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

    SemiUniformList {
        item_count,
        item_heights: Box::new(item_heights),
        render_items: Box::new(render_range),
        height_cache: RefCell::new(None),
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

/// A list element for efficiently laying out and displaying a list of elements with known heights.
pub struct SemiUniformList {
    item_count: usize,
    item_heights: Box<dyn Fn(usize) -> Pixels>,
    render_items: Box<
        dyn for<'a> Fn(Range<usize>, &'a mut Window, &'a mut App) -> SmallVec<[AnyElement; 64]>,
    >,
    height_cache: RefCell<Option<HeightCache>>,
    decorations: Vec<Box<dyn SemiUniformListDecoration>>,
    interactivity: Interactivity,
    scroll_handle: Option<SemiUniformListScrollHandle>,
    sizing_behavior: ListSizingBehavior,
    horizontal_sizing_behavior: ListHorizontalSizingBehavior,
}

/// Frame state used by the [SemiUniformList].
pub struct SemiUniformListFrameState {
    items: SmallVec<[AnyElement; 32]>,
    decorations: SmallVec<[AnyElement; 1]>,
}

/// A handle for controlling the scroll position of a semi-uniform list.
/// This should be stored in your view and passed to the semi_uniform_list on each frame.
#[derive(Clone, Debug, Default)]
pub struct SemiUniformListScrollHandle(pub Rc<RefCell<SemiUniformListScrollState>>);

#[derive(Clone, Debug, Default)]
#[allow(missing_docs)]
pub struct SemiUniformListScrollState {
    pub base_handle: ScrollHandle,
    pub deferred_scroll_to_item: Option<(usize, ScrollStrategy)>,
    /// Size of the list, captured during last layout.
    pub last_list_size: Option<Size<Pixels>>,
    /// Whether the list was vertically flipped during last layout.
    pub y_flipped: bool,
}

/// Cache for height calculations to avoid recomputing on every frame
struct HeightCache {
    /// Running sum of heights to index i
    cumulative_heights: Vec<Pixels>,
    /// Total content height
    total_height: Pixels,
    /// Maximum item width
    max_width: Pixels,
}

impl SemiUniformListScrollHandle {
    /// Create a new scroll handle to bind to a semi-uniform list.
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(SemiUniformListScrollState {
            base_handle: ScrollHandle::new(),
            deferred_scroll_to_item: None,
            last_list_size: None,
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
}

impl Styled for SemiUniformList {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl SemiUniformList {
    /// Sets the sizing behavior, similar to the `List` element.
    pub fn with_sizing_behavior(mut self, behavior: ListSizingBehavior) -> Self {
        self.sizing_behavior = behavior;
        self
    }

    /// Sets the horizontal sizing behavior, controlling the way list items laid out horizontally.
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
    pub fn with_decoration(mut self, decoration: impl SemiUniformListDecoration + 'static) -> Self {
        self.decorations.push(Box::new(decoration));
        self
    }

    /// Track and render scroll state of this list with reference to the given scroll handle.
    pub fn track_scroll(mut self, handle: SemiUniformListScrollHandle) -> Self {
        self.interactivity.tracked_scroll_handle = Some(handle.0.borrow().base_handle.clone());
        self.scroll_handle = Some(handle);
        self
    }

    /// Sets whether the list is flipped vertically, such that item 0 appears at the bottom.
    pub fn y_flipped(mut self, y_flipped: bool) -> Self {
        if let Some(ref scroll_handle) = self.scroll_handle {
            let mut scroll_state = scroll_handle.0.borrow_mut();
            let base_handle = &scroll_state.base_handle;
            let offset = base_handle.offset();
            
            if scroll_state.y_flipped != y_flipped {
                // Reset or adjust offset as needed when flipping
                if y_flipped {
                    base_handle.set_offset(point(offset.x, Pixels::MIN));
                } else {
                    base_handle.set_offset(point(offset.x, Pixels::ZERO));
                }
                scroll_state.y_flipped = y_flipped;
            }
        }
        self
    }

    /// Initialize or update the height cache for efficient calculations
    fn ensure_cache(&mut self, window: &mut Window, cx: &mut App) {
        let mut cache = self.height_cache.borrow_mut();
        
        if cache.is_none() {
            let mut cumulative_heights = Vec::with_capacity(self.item_count + 1);
            let mut current_height = Pixels::ZERO;
            let mut max_width = Pixels::ZERO;
            
            // Initialize with zero at position 0
            cumulative_heights.push(Pixels::ZERO);
            
            // Measure one element to get a baseline width
            if self.item_count > 0 {
                let mut items = (self.render_items)(0..1, window, cx);
                if let Some(mut item) = items.pop() {
                    // Measure just to get width (height will be provided by item_heights)
                    let available_space = size(
                        AvailableSpace::MinContent,
                        AvailableSpace::MinContent,
                    );
                    let measured = item.layout_as_root(available_space, window, cx);
                    max_width = measured.width;
                }
            }
            
            // Calculate cumulative heights
            for i in 0..self.item_count {
                current_height += (self.item_heights)(i);
                cumulative_heights.push(current_height);
            }
            
            *cache = Some(HeightCache {
                cumulative_heights,
                total_height: current_height,
                max_width,
            });
        }
    }
    

}

impl Element for SemiUniformList {
    type RequestLayoutState = SemiUniformListFrameState;
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
        // Initialize the height cache
        self.ensure_cache(window, cx);
        let cache_ref = self.height_cache.borrow();
        let (total_height, max_width) = if let Some(cache) = cache_ref.as_ref() {
            (cache.total_height, cache.max_width)
        } else {
            (Pixels::ZERO, Pixels::ZERO)
        };
        
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
                                let width = known_dimensions.width.unwrap_or(match available_space.width {
                                    AvailableSpace::Definite(x) => x,
                                    AvailableSpace::MinContent | AvailableSpace::MaxContent => max_width,
                                });
                                let height = match available_space.height {
                                    AvailableSpace::Definite(height) => total_height.min(height),
                                    AvailableSpace::MinContent | AvailableSpace::MaxContent => total_height,
                                };
                                size(width, height)
                            },
                        )
                    })
                }
                ListSizingBehavior::Auto => window.with_text_style(style.text_style().cloned(), |window| {
                    window.request_layout(style, None, cx)
                }),
            },
        );

        (
            layout_id,
            SemiUniformListFrameState {
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
        // Ensure the height cache is initialized
        self.ensure_cache(window, cx);
        
        // Get dimensions from cache
        let cache_ref = self.height_cache.borrow();
        let (total_height, max_width) = if let Some(cache) = cache_ref.as_ref() {
            (cache.total_height, cache.max_width)
        } else {
            (Pixels::ZERO, Pixels::ZERO)
        };
        drop(cache_ref);
        
        let style = self.interactivity.compute_style(global_id, None, window, cx);
        let border = style.border_widths.to_pixels(window.rem_size());
        let padding = style.padding.to_pixels(bounds.size.into(), window.rem_size());

        let padded_bounds = Bounds::from_corners(
            bounds.origin + point(border.left + padding.left, border.top + padding.top),
            bounds.bottom_right() - point(border.right + padding.right, border.bottom + padding.bottom),
        );

        let can_scroll_horizontally = matches!(
            self.horizontal_sizing_behavior,
            ListHorizontalSizingBehavior::Unconstrained
        );

        let content_width = if can_scroll_horizontally {
            padded_bounds.size.width.max(max_width)
        } else {
            padded_bounds.size.width
        };
        
        let content_size = Size {
            width: content_width,
            height: total_height + padding.top + padding.bottom,
        };

        // Update scroll handle
        let y_flipped;
        let scroll_to_item = if let Some(handle) = &mut self.scroll_handle {
            {
                let mut state = handle.0.borrow_mut();
                state.last_list_size = Some(Size {
                    width: padded_bounds.size.width,
                    height: padded_bounds.size.height,
                });
                
                y_flipped = state.y_flipped;
                state.deferred_scroll_to_item.take()
            }
        } else {
            y_flipped = false;
            None
        };

        // Get the shared scroll offset
        let shared_scroll_offset = self.interactivity.scroll_offset.clone().unwrap();
        
        // Store important references we need for the closure
        let item_count = self.item_count;
        let item_heights = &self.item_heights;
        let render_items = &self.render_items;
        let height_cache = &self.height_cache;
        let decorations = &self.decorations;
        
        self.interactivity.prepaint(
            global_id,
            bounds,
            content_size,
            window,
            cx,
            |style, mut scroll_offset, hitbox, window, cx| {
                let border = style.border_widths.to_pixels(window.rem_size());
                let padding = style.padding.to_pixels(bounds.size.into(), window.rem_size());

                let padded_bounds = Bounds::from_corners(
                    bounds.origin + point(border.left + padding.left, border.top),
                    bounds.bottom_right() - point(border.right + padding.right, border.bottom),
                );

                // Set the bounds for the scroll handle
                if let Some(scroll_handle) = &self.scroll_handle {
                    scroll_handle.0.borrow_mut().base_handle.set_bounds(bounds);
                }

                if item_count > 0 {
                    let content_height = content_size.height;
                    let is_scrolled_vertically = !scroll_offset.y.is_zero();
                    let min_vertical_scroll_offset = padded_bounds.size.height - content_height;
                    
                    if is_scrolled_vertically && scroll_offset.y < min_vertical_scroll_offset {
                        shared_scroll_offset.borrow_mut().y = min_vertical_scroll_offset;
                        scroll_offset.y = min_vertical_scroll_offset;
                    }

                    let content_width = content_size.width + padding.left + padding.right;
                    let is_scrolled_horizontally = can_scroll_horizontally && !scroll_offset.x.is_zero();
                    
                    if is_scrolled_horizontally && content_width <= padded_bounds.size.width {
                        shared_scroll_offset.borrow_mut().x = Pixels::ZERO;
                        scroll_offset.x = Pixels::ZERO;
                    }

                    // Handle scroll_to_item
                    if let Some((mut ix, scroll_strategy)) = scroll_to_item {
                        if y_flipped {
                            ix = item_count.saturating_sub(ix + 1);
                        }
                        
                        let list_height = padded_bounds.size.height;
                        let mut updated_scroll_offset = shared_scroll_offset.borrow_mut();
                        
                        // Helper function to get item y position safely
                        let get_y_position = |index: usize| -> Pixels {
                            let cache = height_cache.borrow();
                            if let Some(cache) = cache.as_ref() {
                                if index < cache.cumulative_heights.len() {
                                    cache.cumulative_heights[index]
                                } else {
                                    cache.total_height
                                }
                            } else {
                                Pixels::ZERO
                            }
                        };
                        
                        // Get position of item to scroll to
                        let item_top = get_y_position(ix) + padding.top;
                        let item_height = (item_heights)(ix);
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

                                    if item_top < scroll_top || item_bottom > scroll_top + list_height {
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

                    // Helper functions to find visible range
                    let find_first_visible = |scroll_y: Pixels, padding_top: Pixels| -> usize {
                        let cache = height_cache.borrow();
                        if let Some(cache) = cache.as_ref() {
                            let target_height = -(scroll_y + padding_top);
                            
                            match cache.cumulative_heights.binary_search_by(|height| {
                                height.partial_cmp(&target_height).unwrap_or(cmp::Ordering::Equal)
                            }) {
                                Ok(index) => index,
                                Err(index) => index.saturating_sub(1),
                            }
                        } else {
                            0
                        }
                    };
                    
                    let find_last_visible = |scroll_y: Pixels, viewport_height: Pixels| -> usize {
                        let cache = height_cache.borrow();
                        if let Some(cache) = cache.as_ref() {
                            let target_height = -(scroll_y) + viewport_height;
                            
                            match cache.cumulative_heights.binary_search_by(|height| {
                                if *height <= target_height {
                                    cmp::Ordering::Less
                                } else {
                                    cmp::Ordering::Greater
                                }
                            }) {
                                Ok(index) => index,
                                Err(index) => cmp::min(index, item_count),
                            }
                        } else {
                            item_count.min(10)
                        }
                    };

                    // Find visible range of items
                    let first_visible_element_ix = find_first_visible(scroll_offset.y, padding.top);
                    let last_visible_element_ix = find_last_visible(scroll_offset.y, padded_bounds.size.height);
                    let visible_range = first_visible_element_ix..cmp::min(last_visible_element_ix, item_count);

                    let items = if y_flipped {
                        let flipped_range = item_count.saturating_sub(visible_range.end)
                            ..item_count.saturating_sub(visible_range.start);
                        let mut items = (render_items)(flipped_range, window, cx);
                        items.reverse();
                        items
                    } else {
                        (render_items)(visible_range.clone(), window, cx)
                    };

                    let content_mask = ContentMask { bounds };
                    window.with_content_mask(Some(content_mask), |window| {
                        // Helper function to get item y position safely
                        let get_y_position = |index: usize| -> Pixels {
                            let cache = height_cache.borrow();
                            if let Some(cache) = cache.as_ref() {
                                if index < cache.cumulative_heights.len() {
                                    cache.cumulative_heights[index]
                                } else {
                                    cache.total_height
                                }
                            } else {
                                Pixels::ZERO
                            }
                        };
                        
                        let mut current_y = if y_flipped {
                            get_y_position(item_count - visible_range.start)
                        } else {
                            get_y_position(visible_range.start)
                        };
                        
                        for (mut item, ix) in items.into_iter().zip(visible_range.clone()) {
                            let item_origin = padded_bounds.origin + point(
                                if can_scroll_horizontally {
                                    scroll_offset.x + padding.left
                                } else {
                                    scroll_offset.x
                                },
                                current_y + scroll_offset.y + padding.top,
                            );
                            
                            let item_height = (item_heights)(ix);
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
                            
                            // Move to next item position
                            if y_flipped {
                                current_y -= item_height;
                            } else {
                                current_y += item_height;
                            }
                        }

                        // Render decorations
                        let decoration_bounds = Bounds::new(
                            padded_bounds.origin + point(
                                if can_scroll_horizontally {
                                    scroll_offset.x + padding.left
                                } else {
                                    scroll_offset.x
                                },
                                scroll_offset.y + padding.top,
                            ),
                            padded_bounds.size,
                        );
                        
                        for decoration in decorations {
                            let mut decoration = decoration.as_ref().compute(
                                visible_range.clone(),
                                decoration_bounds,
                                item_count,
                                window,
                                cx,
                            );
                            
                            let available_space = size(
                                AvailableSpace::Definite(decoration_bounds.size.width),
                                AvailableSpace::Definite(decoration_bounds.size.height),
                            );
                            
                            decoration.layout_as_root(available_space, window, cx);
                            decoration.prepaint_at(decoration_bounds.origin, window, cx);
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
        bounds: Bounds<Pixels>,
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

impl IntoElement for SemiUniformList {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

/// A decoration for a [`SemiUniformList`]. This can be used for various things,
/// such as rendering indent guides, or other visual effects.
pub trait SemiUniformListDecoration {
    /// Compute the decoration element, given the visible range of list items,
    /// the bounds of the list, and the total number of items.
    fn compute(
        &self,
        visible_range: Range<usize>,
        bounds: Bounds<Pixels>,
        item_count: usize,
        window: &mut Window,
        cx: &mut App,
    ) -> AnyElement;
}

impl InteractiveElement for SemiUniformList {
    fn interactivity(&mut self) -> &mut Interactivity {
        &mut self.interactivity
    }
}