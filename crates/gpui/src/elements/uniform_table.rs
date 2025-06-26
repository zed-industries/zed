use std::{cell::RefCell, cmp, ops::Range, rc::Rc};

use smallvec::SmallVec;

use crate::{
    AnyElement, App, AvailableSpace, Bounds, ContentMask, Div, Element, ElementId, GlobalElementId,
    Hitbox, InspectorElementId, Interactivity, IntoElement, IsZero as _, LayoutId, Length,
    Overflow, Pixels, ScrollHandle, Size, StyleRefinement, Styled, Window, point, px, size,
};

/// todo!
pub struct UniformTable<const COLS: usize> {
    id: ElementId,
    row_count: usize,
    render_rows:
        Rc<dyn Fn(Range<usize>, &mut Window, &mut App) -> Vec<[AnyElement; COLS]> + 'static>,
    interactivity: Interactivity,
    source_location: &'static std::panic::Location<'static>,
    item_to_measure_index: usize,
    scroll_handle: Option<UniformTableScrollHandle>, // todo! we either want to make our own or make a shared scroll handle between list and table
    sizings: [Length; COLS],
}

/// TODO
#[track_caller]
pub fn uniform_table<const COLS: usize, F>(
    id: impl Into<ElementId>,
    row_count: usize,
    render_rows: F,
) -> UniformTable<COLS>
where
    F: 'static + Fn(Range<usize>, &mut Window, &mut App) -> Vec<[AnyElement; COLS]>,
{
    let mut base_style = StyleRefinement::default();
    base_style.overflow.y = Some(Overflow::Scroll);
    let id = id.into();

    let mut interactivity = Interactivity::new();
    interactivity.element_id = Some(id.clone());

    UniformTable {
        id: id.clone(),
        row_count,
        render_rows: Rc::new(render_rows),
        interactivity: Interactivity {
            element_id: Some(id),
            base_style: Box::new(base_style),
            ..Interactivity::new()
        },
        source_location: core::panic::Location::caller(),
        item_to_measure_index: 0,
        scroll_handle: None,
        sizings: [Length::Auto; COLS],
    }
}

impl<const COLS: usize> UniformTable<COLS> {
    /// todo!
    pub fn with_width_from_item(mut self, item_index: Option<usize>) -> Self {
        self.item_to_measure_index = item_index.unwrap_or(0);
        self
    }
}

impl<const COLS: usize> IntoElement for UniformTable<COLS> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl<const COLS: usize> Styled for UniformTable<COLS> {
    fn style(&mut self) -> &mut StyleRefinement {
        &mut self.interactivity.base_style
    }
}

impl<const COLS: usize> Element for UniformTable<COLS> {
    type RequestLayoutState = ();

    type PrepaintState = (Option<Hitbox>, SmallVec<[AnyElement; 32]>);

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        Some(self.source_location)
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let measure_cx = MeasureContext::new(self);
        let item_size = measure_cx.measure_item(AvailableSpace::MinContent, None, window, cx);
        let layout_id =
            self.interactivity.request_layout(
                global_id,
                inspector_id,
                window,
                cx,
                |style, window, _cx| {
                    window.with_text_style(style.text_style().cloned(), |window| {
                        window.request_measured_layout(
                            style,
                            move |known_dimensions, available_space, window, cx| {
                                let desired_height = item_size.height * measure_cx.row_count;
                                let width = known_dimensions.width.unwrap_or(match available_space
                                    .width
                                {
                                    AvailableSpace::Definite(x) => x,
                                    AvailableSpace::MinContent | AvailableSpace::MaxContent => {
                                        item_size.width
                                    }
                                });
                                let height =
                                    known_dimensions.height.unwrap_or(
                                        match available_space.height {
                                            AvailableSpace::Definite(height) => desired_height
                                                .min(dbg!(window.bounds()).size.height),
                                            AvailableSpace::MinContent
                                            | AvailableSpace::MaxContent => desired_height,
                                        },
                                    );
                                size(width, height)
                            },
                        )
                    })
                },
            );

        (layout_id, ())
    }

    fn prepaint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        window: &mut Window,
        cx: &mut App,
    ) -> Self::PrepaintState {
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

        let can_scroll_horizontally = true;

        let mut column_widths = [Pixels::default(); COLS];
        let longest_row_size = MeasureContext::new(self).measure_item(
            AvailableSpace::Definite(bounds.size.width),
            Some(&mut column_widths),
            window,
            cx,
        );

        // We need to run this for each column:
        let content_width = padded_bounds.size.width.max(longest_row_size.width);

        let content_size = Size {
            width: content_width,
            height: longest_row_size.height * self.row_count + padding.top + padding.bottom,
        };

        let shared_scroll_offset = self.interactivity.scroll_offset.clone().unwrap();
        let row_height = longest_row_size.height;
        let shared_scroll_to_item = self.scroll_handle.as_mut().and_then(|handle| {
            let mut handle = handle.0.borrow_mut();
            handle.last_row_size = Some(RowSize {
                row: padded_bounds.size,
                contents: content_size,
            });
            handle.deferred_scroll_to_item.take()
        });

        let mut rendered_rows = SmallVec::default();

        let hitbox = self.interactivity.prepaint(
            global_id,
            inspector_id,
            bounds,
            content_size,
            window,
            cx,
            |style, mut scroll_offset, hitbox, window, cx| {
                dbg!(bounds, window.bounds());
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

                if self.row_count > 0 {
                    let content_height = row_height * self.row_count + padding.top + padding.bottom;
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
                            ix = self.row_count.saturating_sub(ix + 1);
                        }
                        let list_height = dbg!(padded_bounds.size.height);
                        let mut updated_scroll_offset = shared_scroll_offset.borrow_mut();
                        let item_top = row_height * ix + padding.top;
                        let item_bottom = item_top + row_height;
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
                                    let item_center = item_top + row_height / 2.0;
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
                        (-(scroll_offset.y + padding.top) / row_height).floor() as usize;
                    let last_visible_element_ix = ((-scroll_offset.y + padded_bounds.size.height)
                        / row_height)
                        .ceil() as usize;
                    let visible_range =
                        first_visible_element_ix..cmp::min(last_visible_element_ix, self.row_count);
                    let rows = if y_flipped {
                        let flipped_range = self.row_count.saturating_sub(visible_range.end)
                            ..self.row_count.saturating_sub(visible_range.start);
                        let mut items = (self.render_rows)(flipped_range, window, cx);
                        items.reverse();
                        items
                    } else {
                        (self.render_rows)(visible_range.clone(), window, cx)
                    };

                    let content_mask = ContentMask { bounds };
                    window.with_content_mask(Some(content_mask), |window| {
                        let available_width = if can_scroll_horizontally {
                            padded_bounds.size.width + scroll_offset.x.abs()
                        } else {
                            padded_bounds.size.width
                        };
                        let available_space = size(
                            AvailableSpace::Definite(available_width),
                            AvailableSpace::Definite(row_height),
                        );
                        for (mut row, ix) in rows.into_iter().zip(visible_range.clone()) {
                            let row_origin = padded_bounds.origin
                                + point(
                                    if can_scroll_horizontally {
                                        scroll_offset.x + padding.left
                                    } else {
                                        scroll_offset.x
                                    },
                                    row_height * ix + scroll_offset.y + padding.top,
                                );

                            let mut item = render_row(row, column_widths, row_height).into_any();

                            item.layout_as_root(available_space, window, cx);
                            item.prepaint_at(row_origin, window, cx);
                            rendered_rows.push(item);
                        }
                    });
                }

                hitbox
            },
        );
        return (hitbox, rendered_rows);
    }

    fn paint(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        _: &mut Self::RequestLayoutState,
        (hitbox, rendered_rows): &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        self.interactivity.paint(
            global_id,
            inspector_id,
            bounds,
            hitbox.as_ref(),
            window,
            cx,
            |_, window, cx| {
                for item in rendered_rows {
                    item.paint(window, cx);
                }
            },
        )
    }
}

const DIVIDER_PADDING_PX: Pixels = px(2.0);

fn render_row<const COLS: usize>(
    row: [AnyElement; COLS],
    column_widths: [Pixels; COLS],
    row_height: Pixels,
) -> Div {
    use crate::ParentElement;
    let mut div = crate::div().flex().flex_row().gap(DIVIDER_PADDING_PX);

    for (ix, cell) in row.into_iter().enumerate() {
        div = div.child(
            crate::div()
                .w(column_widths[ix])
                .h(row_height)
                .overflow_hidden()
                .child(cell),
        )
    }

    div
}

struct MeasureContext<const COLS: usize> {
    row_count: usize,
    item_to_measure_index: usize,
    render_rows:
        Rc<dyn Fn(Range<usize>, &mut Window, &mut App) -> Vec<[AnyElement; COLS]> + 'static>,
    sizings: [Length; COLS],
}

impl<const COLS: usize> MeasureContext<COLS> {
    fn new(table: &UniformTable<COLS>) -> Self {
        Self {
            row_count: table.row_count,
            item_to_measure_index: table.item_to_measure_index,
            render_rows: table.render_rows.clone(),
            sizings: table.sizings,
        }
    }

    fn measure_item(
        &self,
        table_width: AvailableSpace,
        column_sizes: Option<&mut [Pixels; COLS]>,
        window: &mut Window,
        cx: &mut App,
    ) -> Size<Pixels> {
        if self.row_count == 0 {
            return Size::default();
        }

        let item_ix = cmp::min(self.item_to_measure_index, self.row_count - 1);
        let mut items = (self.render_rows)(item_ix..item_ix + 1, window, cx);
        let Some(mut item_to_measure) = items.pop() else {
            return Size::default();
        };
        let mut default_column_sizes = [Pixels::default(); COLS];
        let column_sizes = column_sizes.unwrap_or(&mut default_column_sizes);

        let mut row_height = px(0.0);
        for i in 0..COLS {
            let column_available_width = match self.sizings[i] {
                Length::Definite(definite_length) => match table_width {
                    AvailableSpace::Definite(pixels) => AvailableSpace::Definite(
                        definite_length.to_pixels(pixels.into(), window.rem_size()),
                    ),
                    AvailableSpace::MinContent => AvailableSpace::MinContent,
                    AvailableSpace::MaxContent => AvailableSpace::MaxContent,
                },
                Length::Auto => AvailableSpace::MaxContent,
            };

            let column_available_space = size(column_available_width, AvailableSpace::MinContent);

            // todo!: Adjust row sizing to account for inter-column spacing
            let cell_size = item_to_measure[i].layout_as_root(column_available_space, window, cx);
            column_sizes[i] = cell_size.width;
            row_height = row_height.max(cell_size.height);
        }

        let mut width = Pixels::ZERO;

        for size in *column_sizes {
            width += size;
        }

        Size::new(width + (COLS - 1) * DIVIDER_PADDING_PX, row_height)
    }
}

impl<const COLS: usize> UniformTable<COLS> {}

/// A handle for controlling the scroll position of a uniform list.
/// This should be stored in your view and passed to the uniform_list on each frame.
#[derive(Clone, Debug, Default)]
pub struct UniformTableScrollHandle(pub Rc<RefCell<UniformTableScrollState>>);

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

#[derive(Copy, Clone, Debug, Default)]
/// The size of the item and its contents.
pub struct RowSize {
    /// The size of the item.
    pub row: Size<Pixels>,
    /// The size of the item's contents, which may be larger than the item itself,
    /// if the item was bounded by a parent element.
    pub contents: Size<Pixels>,
}

#[derive(Clone, Debug, Default)]
#[allow(missing_docs)]
pub struct UniformTableScrollState {
    pub base_handle: ScrollHandle,
    pub deferred_scroll_to_item: Option<(usize, ScrollStrategy)>,
    /// Size of the item, captured during last layout.
    pub last_row_size: Option<RowSize>,
    /// Whether the list was vertically flipped during last layout.
    pub y_flipped: bool,
}

impl UniformTableScrollHandle {
    /// Create a new scroll handle to bind to a uniform list.
    pub fn new() -> Self {
        Self(Rc::new(RefCell::new(UniformTableScrollState {
            base_handle: ScrollHandle::new(),
            deferred_scroll_to_item: None,
            last_row_size: None,
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
