use std::ops::Range;

use gpui::{
    point, quad, Bounds, ContentMask, Corners, Edges, Element, ElementId, GlobalElementId, Hitbox,
    Hsla, LayoutId, Pixels, Style, WindowContext,
};
use std::cell::Cell;
use std::rc::Rc;
use ui::ActiveTheme;

use crate::{px, relative, IntoElement};

pub struct TerminalScrollbar {
    thumb: Range<f32>,
    state: TerminalScrollbarState,
}

pub struct TerminalScrollbarState {
    drag: Rc<Cell<Option<f32>>>,
    total_lines: usize,
    viewport_lines: usize,
    display_offset: usize,
}

impl TerminalScrollbar {
    pub fn new(state: TerminalScrollbarState) -> Option<Self> {
        let thumb = state.thumb_range()?;
        Some(Self { thumb, state })
    }
}

impl TerminalScrollbarState {
    pub fn new(total_lines: usize, viewport_lines: usize, display_offset: usize) -> Self {
        TerminalScrollbarState {
            drag: Rc::new(Cell::new(None)),
            total_lines,
            viewport_lines,
            display_offset,
        }
    }

    fn is_dragging(&self) -> bool {
        self.drag.get().is_some()
    }

    fn thumb_range(&self) -> Option<Range<f32>> {
        const MINIMUM_THUMB_SIZE: f32 = 0.05; // 5%
        if self.total_lines <= self.viewport_lines {
            return None;
        }
        let thumb_size =
            (self.viewport_lines as f32 / self.total_lines as f32).max(MINIMUM_THUMB_SIZE);
        let max_scroll = self.total_lines.saturating_sub(self.viewport_lines);
        let scroll_progress = self.display_offset as f32 / max_scroll as f32;
        let thumb_position = (scroll_progress * (thumb_size - 1.0)) + 1.0;
        Some((thumb_position - thumb_size)..thumb_position)
    }
}

impl Element for TerminalScrollbar {
    type RequestLayoutState = ();
    type PrepaintState = Hitbox;

    fn id(&self) -> Option<ElementId> {
        None
    }

    fn request_layout(
        &mut self,
        _id: Option<&GlobalElementId>,
        cx: &mut WindowContext,
    ) -> (LayoutId, Self::RequestLayoutState) {
        let mut style = Style::default();
        style.flex_grow = 1.;
        style.flex_shrink = 1.;

        style.size.width = px(12.).into();
        style.size.height = relative(1.).into();

        (cx.request_layout(style, None), ())
    }

    fn prepaint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        cx: &mut WindowContext,
    ) -> Self::PrepaintState {
        cx.with_content_mask(Some(ContentMask { bounds }), |cx| {
            cx.insert_hitbox(bounds, false)
        })
    }

    fn paint(
        &mut self,
        _id: Option<&GlobalElementId>,
        bounds: Bounds<Pixels>,
        _request_layout: &mut Self::RequestLayoutState,
        _prepaint: &mut Self::PrepaintState,
        cx: &mut WindowContext,
    ) {
        cx.with_content_mask(Some(ContentMask { bounds }), |cx| {
            let colors = cx.theme().colors();
            let thumb_background = colors
                .surface_background
                .blend(colors.scrollbar_thumb_background);
            let extra_padding = px(5.0);
            let padded_bounds = Bounds::from_corners(
                bounds.origin + point(Pixels::ZERO, extra_padding),
                bounds.bottom_right() - point(Pixels::ZERO, extra_padding * 3),
            );

            let mut thumb_bounds = {
                let thumb_offset = self.thumb.start * padded_bounds.size.height;
                let thumb_end = self.thumb.end * padded_bounds.size.height;
                Bounds::from_corners(
                    point(
                        padded_bounds.origin.x,
                        padded_bounds.origin.y + thumb_offset,
                    ),
                    point(
                        padded_bounds.origin.x + padded_bounds.size.width,
                        padded_bounds.origin.y + thumb_end,
                    ),
                )
            };
            let corners = {
                thumb_bounds.size.width /= 1.5;
                Corners::all(thumb_bounds.size.width / 2.0)
            };
            cx.paint_quad(quad(
                thumb_bounds,
                corners,
                thumb_background,
                Edges::default(),
                Hsla::transparent_black(),
            ));
        });
    }
}

impl IntoElement for TerminalScrollbar {
    type Element = Self;

    fn into_element(self) -> Self::Element {
        self
    }
}
