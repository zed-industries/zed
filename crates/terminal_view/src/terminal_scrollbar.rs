use std::ops::Range;

use gpui::{
    point, quad, Bounds, ContentMask, Corners, Edges, Element, ElementId, GlobalElementId, Hitbox,
    Hsla, LayoutId, Model, Pixels, Style, WeakView, WindowContext,
};
use std::cell::Cell;
use std::rc::Rc;
use terminal::Terminal;
use ui::ActiveTheme;

use crate::{px, relative, IntoElement, TerminalView};

const MINIMUM_THUMB_SIZE: f32 = 0.05; // 5%

#[derive(Clone)]
pub struct TerminalScrollbar {
    terminal: Model<Terminal>,
    terminal_view: WeakView<TerminalView>,
    drag: Rc<Cell<Option<f32>>>,
}

impl TerminalScrollbar {
    pub fn new(terminal: Model<Terminal>, terminal_view: WeakView<TerminalView>) -> Self {
        Self {
            terminal,
            terminal_view,
            drag: Default::default(),
        }
    }

    fn thumb_range(&self, cx: &WindowContext) -> Range<f32> {
        let terminal = self.terminal.read(cx);
        let viewport_lines = terminal.viewport_lines();
        let total_lines = terminal.total_lines();

        if total_lines <= viewport_lines {
            return 0.0..0.0;
        }

        let thumb_size = (viewport_lines as f32 / total_lines as f32).max(MINIMUM_THUMB_SIZE);
        let max_scroll = total_lines.saturating_sub(viewport_lines);
        let scroll_progress = terminal.last_content.display_offset as f32 / max_scroll as f32;
        let thumb_position = (scroll_progress * (thumb_size - 1.0)) + 1.0;
        (thumb_position - thumb_size)..thumb_position
    }

    pub fn is_dragging(&self) -> bool {
        self.drag.get().is_some()
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
                let thumb = self.thumb_range(cx);
                let thumb_offset = thumb.start * padded_bounds.size.height;
                let thumb_end = thumb.end * padded_bounds.size.height;
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
