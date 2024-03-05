//! A scrollable list of elements with uniform height, optimized for large lists.
//! Rather than use the full taffy layout system, uniform_list simply measures
//! the first element and then lays out all remaining elements in a line based on that
//! measurement. This is much faster than the full layout system, but only works for
//! elements with uniform height.

use crate::{
    point, px, size, AnyElement, AvailableSpace, Bounds, ContentMask, Element, ElementContext,
    ElementId, InteractiveElement, InteractiveElementState, Interactivity, IntoElement, LayoutId,
    Pixels, Render, Size, StyleRefinement, Styled, View, ViewContext, WindowContext,
};
use smallvec::SmallVec;
use std::{cell::RefCell, cmp, ops::Range, rc::Rc};
use taffy::style::Overflow;

/// uniform_list provides lazy rendering for a set of items that are of uniform height.
/// When rendered into a container with overflow-y: hidden and a fixed (or max) height,
/// uniform_list will only render the visible subset of items.
#[track_caller]
pub fn uniform_list<I, R, V>(
    view: View<V>,
    id: I,
    item_count: usize,
    f: impl 'static + Fn(&mut V, Range<usize>, &mut ViewContext<V>) -> Vec<R>,
) -> UniformList
where
    I: Into<ElementId>,
    R: IntoElement,
    V: Render,
{
    let id = id.into();
    let mut base_style = StyleRefinement::default();
    base_style.overflow.y = Some(Overflow::Scroll);

    let render_range = move |range, cx: &mut WindowContext| {
        view.update(cx, |this, cx| {
            f(this, range, cx)
                .into_iter()
                .map(|component| component.into_any_element())
                .collect()
        })
    };

    UniformList {
        id: id.clone(),
        item_count,
        item_to_measure_index: 0,
        render_items: Box::new(render_range),
        interactivity: Interactivity {
            element_id: Some(id),
            base_style: Box::new(base_style),

            #[cfg(debug_assertions)]
            location: Some(*core::panic::Location::caller()),

            ..Default::default()
        },
        scroll_handle: None,
    }
}

/// A list element for efficiently laying out and displaying a list of uniform-height elements.
pub struct UniformList {
    id: ElementId,
    item_count: usize,
    item_to_measure_index: usize,
    render_items:
        Box<dyn for<'a> Fn(Range<usize>, &'a mut WindowContext) -> SmallVec<[AnyElement; 64]>>,
    interactivity: Interactivity,
    scroll_handle: Option<UniformListScrollHandle>,
}

/// A handle for controlling the scroll position of a uniform list.
/// This should be stored in your view and passed to the uniform_list on each frame.
#[derive(Clone, Default)]
pub struct UniformListScrollHandle {
    deferred_scroll_to_item: Rc<RefCell<Option<usize>>>,
}

impl UniformListScrollHandle {
    /// Create a new scroll handle to bind to a uniform list.
    pub fn new() -> Self {
        Self {
            deferred_scroll_to_item: Rc::new(RefCell::new(None)),
        }
    }

    /// Scroll the list to the given item index.
    pub fn scroll_to_item(&mut self, ix: usize) {
        self.deferred_scroll_to_item.replace(Some(ix));
    }
}

impl Styled for UniformList {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

#[doc(hidden)]
#[derive(Default)]
pub struct UniformListState {
    interactive: InteractiveElementState,
    item_size: Size<Pixels>,
}

impl Element for UniformList {
    type State = UniformListState;

    fn request_layout(
        &mut self,
        state: Option<Self::State>,
        cx: &mut ElementContext,
    ) -> (LayoutId, Self::State) {
        let max_items = self.item_count;
        let item_size = state
            .as_ref()
            .map(|s| s.item_size)
            .unwrap_or_else(|| self.measure_item(None, cx));

        let (layout_id, interactive) =
            self.interactivity
                .layout(state.map(|s| s.interactive), cx, |style, cx| {
                    cx.request_measured_layout(
                        style,
                        move |known_dimensions, available_space, _cx| {
                            let desired_height = item_size.height * max_items;
                            let width =
                                known_dimensions
                                    .width
                                    .unwrap_or(match available_space.width {
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
                });

        let element_state = UniformListState {
            interactive,
            item_size,
        };

        (layout_id, element_state)
    }

    fn paint(
        &mut self,
        bounds: Bounds<crate::Pixels>,
        element_state: &mut Self::State,
        cx: &mut ElementContext,
    ) {
        let style =
            self.interactivity
                .compute_style(Some(bounds), &mut element_state.interactive, cx);
        let border = style.border_widths.to_pixels(cx.rem_size());
        let padding = style.padding.to_pixels(bounds.size.into(), cx.rem_size());

        let padded_bounds = Bounds::from_corners(
            bounds.origin + point(border.left + padding.left, border.top + padding.top),
            bounds.lower_right()
                - point(border.right + padding.right, border.bottom + padding.bottom),
        );

        let item_size = element_state.item_size;
        let content_size = Size {
            width: padded_bounds.size.width,
            height: item_size.height * self.item_count + padding.top + padding.bottom,
        };

        let shared_scroll_offset = element_state
            .interactive
            .scroll_offset
            .get_or_insert_with(Rc::default)
            .clone();

        let item_height = self.measure_item(Some(padded_bounds.size.width), cx).height;
        let shared_scroll_to_item = self
            .scroll_handle
            .as_mut()
            .and_then(|handle| handle.deferred_scroll_to_item.take());

        self.interactivity.paint(
            bounds,
            content_size,
            &mut element_state.interactive,
            cx,
            |style, mut scroll_offset, cx| {
                let border = style.border_widths.to_pixels(cx.rem_size());
                let padding = style.padding.to_pixels(bounds.size.into(), cx.rem_size());

                let padded_bounds = Bounds::from_corners(
                    bounds.origin + point(border.left + padding.left, border.top),
                    bounds.lower_right() - point(border.right + padding.right, border.bottom),
                );

                if self.item_count > 0 {
                    let content_height =
                        item_height * self.item_count + padding.top + padding.bottom;
                    let min_scroll_offset = padded_bounds.size.height - content_height;
                    let is_scrolled = scroll_offset.y != px(0.);

                    if is_scrolled && scroll_offset.y < min_scroll_offset {
                        shared_scroll_offset.borrow_mut().y = min_scroll_offset;
                        scroll_offset.y = min_scroll_offset;
                    }

                    if let Some(ix) = shared_scroll_to_item {
                        let list_height = padded_bounds.size.height;
                        let mut updated_scroll_offset = shared_scroll_offset.borrow_mut();
                        let item_top = item_height * ix + padding.top;
                        let item_bottom = item_top + item_height;
                        let scroll_top = -updated_scroll_offset.y;
                        if item_top < scroll_top + padding.top {
                            updated_scroll_offset.y = -(item_top) + padding.top;
                        } else if item_bottom > scroll_top + list_height - padding.bottom {
                            updated_scroll_offset.y = -(item_bottom - list_height) - padding.bottom;
                        }
                        scroll_offset = *updated_scroll_offset;
                    }

                    let first_visible_element_ix =
                        (-(scroll_offset.y + padding.top) / item_height).floor() as usize;
                    let last_visible_element_ix = ((-scroll_offset.y + padded_bounds.size.height)
                        / item_height)
                        .ceil() as usize;
                    let visible_range = first_visible_element_ix
                        ..cmp::min(last_visible_element_ix, self.item_count);

                    let mut items = (self.render_items)(visible_range.clone(), cx);
                    cx.with_z_index(1, |cx| {
                        let content_mask = ContentMask { bounds };
                        cx.with_content_mask(Some(content_mask), |cx| {
                            for (item, ix) in items.iter_mut().zip(visible_range) {
                                let item_origin = padded_bounds.origin
                                    + point(
                                        px(0.),
                                        item_height * ix + scroll_offset.y + padding.top,
                                    );
                                let available_space = size(
                                    AvailableSpace::Definite(padded_bounds.size.width),
                                    AvailableSpace::Definite(item_height),
                                );
                                item.draw(item_origin, available_space, cx);
                            }
                        });
                    });
                }
            },
        )
    }
}

impl IntoElement for UniformList {
    type Element = Self;

    fn element_id(&self) -> Option<crate::ElementId> {
        Some(self.id.clone())
    }

    fn into_element(self) -> Self::Element {
        self
    }
}

impl UniformList {
    /// Selects a specific list item for measurement.
    pub fn with_width_from_item(mut self, item_index: Option<usize>) -> Self {
        self.item_to_measure_index = item_index.unwrap_or(0);
        self
    }

    fn measure_item(&self, list_width: Option<Pixels>, cx: &mut ElementContext) -> Size<Pixels> {
        if self.item_count == 0 {
            return Size::default();
        }

        let item_ix = cmp::min(self.item_to_measure_index, self.item_count - 1);
        let mut items = (self.render_items)(item_ix..item_ix + 1, cx);
        let mut item_to_measure = items.pop().unwrap();
        let available_space = size(
            list_width.map_or(AvailableSpace::MinContent, |width| {
                AvailableSpace::Definite(width)
            }),
            AvailableSpace::MinContent,
        );
        item_to_measure.measure(available_space, cx)
    }

    /// Track and render scroll state of this list with reference to the given scroll handle.
    pub fn track_scroll(mut self, handle: UniformListScrollHandle) -> Self {
        self.scroll_handle = Some(handle);
        self
    }
}

impl InteractiveElement for UniformList {
    fn interactivity(&mut self) -> &mut crate::Interactivity {
        &mut self.interactivity
    }
}
