use std::ops::Range;

use crate::{
    AnyElement, App, Bounds, Element, ElementId, GlobalElementId, InspectorElementId,
    Interactivity, IntoElement, LayoutId, Overflow, Pixels, Size, StyleRefinement, Window, point,
};

/// todo!
pub struct UniformTable<const COLS: usize> {
    id: ElementId,
    row_count: usize,
    striped: bool,
    headers: Option<[AnyElement; COLS]>,
    render_rows:
        Box<dyn Fn(Range<usize>, &mut Window, &mut App) -> Vec<[AnyElement; COLS]> + 'static>,
    interactivity: Interactivity,
    // flexes: Arc<Mutex<[f32; COL]>>,
}

/// todo!
pub fn uniform_table<const COLS: usize>(
    id: impl Into<ElementId>,
    row_count: usize,
) -> UniformTable<COLS> {
    let mut base_style = StyleRefinement::default();
    base_style.overflow.y = Some(Overflow::Scroll);
    let id = id.into();

    let mut interactivity = Interactivity::new();
    interactivity.element_id = Some(id.clone());

    UniformTable {
        id: id.clone(),
        row_count,
        striped: false,
        headers: None,
        render_rows: Box::new(UniformTable::default_render_rows), // flexes: Arc::new(Mutex::new([0.0; COLS])),
        interactivity: Interactivity {
            element_id: Some(id),
            base_style: Box::new(base_style),
            ..Interactivity::new()
        },
    }
}

impl<const COLS: usize> UniformTable<COLS> {
    /// todo!
    pub fn striped(mut self, striped: bool) -> Self {
        self.striped = striped;
        self
    }

    /// todo!
    pub fn header(mut self, headers: [impl IntoElement; COLS]) -> Self {
        self.headers = Some(headers.map(IntoElement::into_any_element));
        self
    }

    /// todo!
    pub fn rows<F>(mut self, render_rows: F) -> Self
    where
        F: 'static + Fn(Range<usize>, &mut Window, &mut App) -> Vec<[AnyElement; COLS]>,
    {
        // let render_rows = move |range: Range<usize>, window: &mut Window, cx: &mut App| {
        //     // FIXME: avoid the double copy from vec and collect
        //     render_rows(range, window, cx)
        //         .into_iter()
        //         .map(|component| component.into_any_element())
        //         .collect()
        // };
        self.render_rows = Box::new(render_rows);
        self
    }

    fn default_render_rows(
        range: Range<usize>,
        window: &mut Window,
        cx: &mut App,
    ) -> Vec<[AnyElement; COLS]> {
        vec![]
    }
}

impl<const COLS: usize> IntoElement for UniformTable<COLS> {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}

impl<const COLS: usize> Element for UniformTable<COLS> {
    type RequestLayoutState = ();

    type PrepaintState = ();

    fn id(&self) -> Option<ElementId> {
        Some(self.id.clone())
    }

    fn source_location(&self) -> Option<&'static std::panic::Location<'static>> {
        None
    }

    fn request_layout(
        &mut self,
        global_id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        window: &mut Window,
        cx: &mut App,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let max_items = self.row_count;
        let layout_id = self.interactivity.request_layout(
            global_id,
            inspector_id,
            window,
            cx,
            |style, window, cx| {
                window.with_text_style(style.text_style().cloned(), |window| {
                    window.request_layout(style, None, cx)
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
        request_layout: &mut Self::RequestLayoutState,
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

        // let can_scroll_horizontally = matches!(
        //     self.horizontal_sizing_behavior,
        //     ListHorizontalSizingBehavior::Unconstrained
        // );

        // let longest_item_size = self.measure_item(None, window, cx);
        // let content_width = if can_scroll_horizontally {
        //     padded_bounds.size.width.max(longest_item_size.width)
        // } else {
        //     padded_bounds.size.width
        // };

        // let content_size = Size {
        //     width: content_width,
        //     height: longest_item_size.height * self.row_count + padding.top + padding.bottom,
        // };

        // let shared_scroll_offset = self.interactivity.scroll_offset.clone().unwrap();
        // let item_height = longest_item_size.height;
        // let shared_scroll_to_item = self.scroll_handle.as_mut().and_then(|handle| {
        //     let mut handle = handle.0.borrow_mut();
        //     handle.last_item_size = Some(ItemSize {
        //         item: padded_bounds.size,
        //         contents: content_size,
        //     });
        //     handle.deferred_scroll_to_item.take()
        // });

        // self.interactivity.prepaint(
        //     global_id,
        //     inspector_id,
        //     bounds,
        //     content_size,
        //     window,
        //     cx,
        //     |style, mut scroll_offset, hitbox, window, cx| {
        //         let border = style.border_widths.to_pixels(window.rem_size());
        //         let padding = style
        //             .padding
        //             .to_pixels(bounds.size.into(), window.rem_size());

        //         let padded_bounds = Bounds::from_corners(
        //             bounds.origin + point(border.left + padding.left, border.top),
        //             bounds.bottom_right() - point(border.right + padding.right, border.bottom),
        //         );

        //         let y_flipped = if let Some(scroll_handle) = self.scroll_handle.as_mut() {
        //             let mut scroll_state = scroll_handle.0.borrow_mut();
        //             scroll_state.base_handle.set_bounds(bounds);
        //             scroll_state.y_flipped
        //         } else {
        //             false
        //         };

        //         if self.item_count > 0 {
        //             let content_height =
        //                 item_height * self.item_count + padding.top + padding.bottom;
        //             let is_scrolled_vertically = !scroll_offset.y.is_zero();
        //             let min_vertical_scroll_offset = padded_bounds.size.height - content_height;
        //             if is_scrolled_vertically && scroll_offset.y < min_vertical_scroll_offset {
        //                 shared_scroll_offset.borrow_mut().y = min_vertical_scroll_offset;
        //                 scroll_offset.y = min_vertical_scroll_offset;
        //             }

        //             let content_width = content_size.width + padding.left + padding.right;
        //             let is_scrolled_horizontally =
        //                 can_scroll_horizontally && !scroll_offset.x.is_zero();
        //             if is_scrolled_horizontally && content_width <= padded_bounds.size.width {
        //                 shared_scroll_offset.borrow_mut().x = Pixels::ZERO;
        //                 scroll_offset.x = Pixels::ZERO;
        //             }

        //             if let Some((mut ix, scroll_strategy)) = shared_scroll_to_item {
        //                 if y_flipped {
        //                     ix = self.item_count.saturating_sub(ix + 1);
        //                 }
        //                 let list_height = padded_bounds.size.height;
        //                 let mut updated_scroll_offset = shared_scroll_offset.borrow_mut();
        //                 let item_top = item_height * ix + padding.top;
        //                 let item_bottom = item_top + item_height;
        //                 let scroll_top = -updated_scroll_offset.y;
        //                 let mut scrolled_to_top = false;
        //                 if item_top < scroll_top + padding.top {
        //                     scrolled_to_top = true;
        //                     updated_scroll_offset.y = -(item_top) + padding.top;
        //                 } else if item_bottom > scroll_top + list_height - padding.bottom {
        //                     scrolled_to_top = true;
        //                     updated_scroll_offset.y = -(item_bottom - list_height) - padding.bottom;
        //                 }

        //                 match scroll_strategy {
        //                     ScrollStrategy::Top => {}
        //                     ScrollStrategy::Center => {
        //                         if scrolled_to_top {
        //                             let item_center = item_top + item_height / 2.0;
        //                             let target_scroll_top = item_center - list_height / 2.0;

        //                             if item_top < scroll_top
        //                                 || item_bottom > scroll_top + list_height
        //                             {
        //                                 updated_scroll_offset.y = -target_scroll_top
        //                                     .max(Pixels::ZERO)
        //                                     .min(content_height - list_height)
        //                                     .max(Pixels::ZERO);
        //                             }
        //                         }
        //                     }
        //                 }
        //                 scroll_offset = *updated_scroll_offset
        //             }

        //             let first_visible_element_ix =
        //                 (-(scroll_offset.y + padding.top) / item_height).floor() as usize;
        //             let last_visible_element_ix = ((-scroll_offset.y + padded_bounds.size.height)
        //                 / item_height)
        //                 .ceil() as usize;
        //             let visible_range = first_visible_element_ix
        //                 ..cmp::min(last_visible_element_ix, self.item_count);

        //             let items = if y_flipped {
        //                 let flipped_range = self.item_count.saturating_sub(visible_range.end)
        //                     ..self.item_count.saturating_sub(visible_range.start);
        //                 let mut items = (self.render_items)(flipped_range, window, cx);
        //                 items.reverse();
        //                 items
        //             } else {
        //                 (self.render_items)(visible_range.clone(), window, cx)
        //             };

        //             let content_mask = ContentMask { bounds };
        //             window.with_content_mask(Some(content_mask), |window| {
        //                 for (mut item, ix) in items.into_iter().zip(visible_range.clone()) {
        //                     let item_origin = padded_bounds.origin
        //                         + point(
        //                             if can_scroll_horizontally {
        //                                 scroll_offset.x + padding.left
        //                             } else {
        //                                 scroll_offset.x
        //                             },
        //                             item_height * ix + scroll_offset.y + padding.top,
        //                         );
        //                     let available_width = if can_scroll_horizontally {
        //                         padded_bounds.size.width + scroll_offset.x.abs()
        //                     } else {
        //                         padded_bounds.size.width
        //                     };
        //                     let available_space = size(
        //                         AvailableSpace::Definite(available_width),
        //                         AvailableSpace::Definite(item_height),
        //                     );
        //                     item.layout_as_root(available_space, window, cx);
        //                     item.prepaint_at(item_origin, window, cx);
        //                     frame_state.items.push(item);
        //                 }

        //                 let bounds = Bounds::new(
        //                     padded_bounds.origin
        //                         + point(
        //                             if can_scroll_horizontally {
        //                                 scroll_offset.x + padding.left
        //                             } else {
        //                                 scroll_offset.x
        //                             },
        //                             scroll_offset.y + padding.top,
        //                         ),
        //                     padded_bounds.size,
        //                 );
        //                 for decoration in &self.decorations {
        //                     let mut decoration = decoration.as_ref().compute(
        //                         visible_range.clone(),
        //                         bounds,
        //                         item_height,
        //                         self.item_count,
        //                         window,
        //                         cx,
        //                     );
        //                     let available_space = size(
        //                         AvailableSpace::Definite(bounds.size.width),
        //                         AvailableSpace::Definite(bounds.size.height),
        //                     );
        //                     decoration.layout_as_root(available_space, window, cx);
        //                     decoration.prepaint_at(bounds.origin, window, cx);
        //                     frame_state.decorations.push(decoration);
        //                 }
        //             });
        //         }

        //         hitbox
        //     },
        // )
        todo!()
    }

    fn paint(
        &mut self,
        id: Option<&GlobalElementId>,
        inspector_id: Option<&InspectorElementId>,
        bounds: Bounds<Pixels>,
        request_layout: &mut Self::RequestLayoutState,
        prepaint: &mut Self::PrepaintState,
        window: &mut Window,
        cx: &mut App,
    ) {
        todo!()
    }
}
