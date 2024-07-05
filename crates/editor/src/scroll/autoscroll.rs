use crate::{
    display_map::ToDisplayPoint, DisplayRow, Editor, EditorMode, LineWithInvisibles, RowExt,
};
use gpui::{px, Bounds, Pixels, ViewContext};
use language::Point;
use std::{cmp, f32};

#[derive(PartialEq, Eq, Clone, Copy)]
pub enum Autoscroll {
    Next,
    Strategy(AutoscrollStrategy),
}

impl Autoscroll {
    /// scrolls the minimal amount to (try) and fit all cursors onscreen
    pub fn fit() -> Self {
        Self::Strategy(AutoscrollStrategy::Fit)
    }

    /// scrolls the minimal amount to fit the newest cursor
    pub fn newest() -> Self {
        Self::Strategy(AutoscrollStrategy::Newest)
    }

    /// scrolls so the newest cursor is vertically centered
    pub fn center() -> Self {
        Self::Strategy(AutoscrollStrategy::Center)
    }

    /// scrolls so the neweset cursor is near the top
    /// (offset by vertical_scroll_margin)
    pub fn focused() -> Self {
        Self::Strategy(AutoscrollStrategy::Focused)
    }
    /// Scrolls so that the newest cursor is roughly an n-th line from the top.
    pub fn top_relative(n: usize) -> Self {
        Self::Strategy(AutoscrollStrategy::TopRelative(n))
    }
}

#[derive(PartialEq, Eq, Default, Clone, Copy)]
pub enum AutoscrollStrategy {
    Fit,
    Newest,
    #[default]
    Center,
    Focused,
    Top,
    Bottom,
    TopRelative(usize),
}

impl AutoscrollStrategy {
    fn next(&self) -> Self {
        match self {
            AutoscrollStrategy::Center => AutoscrollStrategy::Top,
            AutoscrollStrategy::Top => AutoscrollStrategy::Bottom,
            _ => AutoscrollStrategy::Center,
        }
    }
}

impl Editor {
    pub fn autoscroll_requested(&self) -> bool {
        self.scroll_manager.autoscroll_requested()
    }

    pub fn autoscroll_vertically(
        &mut self,
        bounds: Bounds<Pixels>,
        line_height: Pixels,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        let viewport_height = bounds.size.height;
        let visible_lines = viewport_height / line_height;
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut scroll_position = self.scroll_manager.scroll_position(&display_map);
        let original_y = scroll_position.y;
        if let Some(last_bounds) = self.expect_bounds_change.take() {
            if scroll_position.y != 0. {
                scroll_position.y += (bounds.top() - last_bounds.top()) / line_height;
                if scroll_position.y < 0. {
                    scroll_position.y = 0.;
                }
            }
        }
        let max_scroll_top = if matches!(self.mode, EditorMode::AutoHeight { .. }) {
            (display_map.max_point().row().as_f32() - visible_lines + 1.).max(0.)
        } else {
            display_map.max_point().row().as_f32()
        };
        if scroll_position.y > max_scroll_top {
            scroll_position.y = max_scroll_top;
        }

        if original_y != scroll_position.y {
            self.set_scroll_position(scroll_position, cx);
        }

        let Some((autoscroll, local)) = self.scroll_manager.autoscroll_request.take() else {
            return false;
        };

        let mut target_top;
        let mut target_bottom;
        if let Some(first_highlighted_row) =
            self.highlighted_display_row_for_autoscroll(&display_map)
        {
            target_top = first_highlighted_row.as_f32();
            target_bottom = target_top + 1.;
        } else {
            let selections = self.selections.all::<Point>(cx);
            target_top = selections
                .first()
                .unwrap()
                .head()
                .to_display_point(&display_map)
                .row()
                .as_f32();
            target_bottom = selections
                .last()
                .unwrap()
                .head()
                .to_display_point(&display_map)
                .row()
                .next_row()
                .as_f32();

            // If the selections can't all fit on screen, scroll to the newest.
            if autoscroll == Autoscroll::newest()
                || autoscroll == Autoscroll::fit() && target_bottom - target_top > visible_lines
            {
                let newest_selection_top = selections
                    .iter()
                    .max_by_key(|s| s.id)
                    .unwrap()
                    .head()
                    .to_display_point(&display_map)
                    .row()
                    .as_f32();
                target_top = newest_selection_top;
                target_bottom = newest_selection_top + 1.;
            }
        }

        let margin = if matches!(self.mode, EditorMode::AutoHeight { .. }) {
            0.
        } else {
            ((visible_lines - (target_bottom - target_top)) / 2.0).floor()
        };

        let strategy = match autoscroll {
            Autoscroll::Strategy(strategy) => strategy,
            Autoscroll::Next => {
                let last_autoscroll = &self.scroll_manager.last_autoscroll;
                if let Some(last_autoscroll) = last_autoscroll {
                    if self.scroll_manager.anchor.offset == last_autoscroll.0
                        && target_top == last_autoscroll.1
                        && target_bottom == last_autoscroll.2
                    {
                        last_autoscroll.3.next()
                    } else {
                        AutoscrollStrategy::default()
                    }
                } else {
                    AutoscrollStrategy::default()
                }
            }
        };

        match strategy {
            AutoscrollStrategy::Fit | AutoscrollStrategy::Newest => {
                let margin = margin.min(self.scroll_manager.vertical_scroll_margin);
                let target_top = (target_top - margin).max(0.0);
                let target_bottom = target_bottom + margin;
                let start_row = scroll_position.y;
                let end_row = start_row + visible_lines;

                let needs_scroll_up = target_top < start_row;
                let needs_scroll_down = target_bottom >= end_row;

                if needs_scroll_up && !needs_scroll_down {
                    scroll_position.y = target_top;
                    self.set_scroll_position_internal(scroll_position, local, true, cx);
                }
                if !needs_scroll_up && needs_scroll_down {
                    scroll_position.y = target_bottom - visible_lines;
                    self.set_scroll_position_internal(scroll_position, local, true, cx);
                }
            }
            AutoscrollStrategy::Center => {
                scroll_position.y = (target_top - margin).max(0.0);
                self.set_scroll_position_internal(scroll_position, local, true, cx);
            }
            AutoscrollStrategy::Focused => {
                scroll_position.y =
                    (target_top - self.scroll_manager.vertical_scroll_margin).max(0.0);
                self.set_scroll_position_internal(scroll_position, local, true, cx);
            }
            AutoscrollStrategy::Top => {
                scroll_position.y = (target_top).max(0.0);
                self.set_scroll_position_internal(scroll_position, local, true, cx);
            }
            AutoscrollStrategy::Bottom => {
                scroll_position.y = (target_bottom - visible_lines).max(0.0);
                self.set_scroll_position_internal(scroll_position, local, true, cx);
            }
            AutoscrollStrategy::TopRelative(lines) => {
                scroll_position.y = target_top - lines as f32;
                self.set_scroll_position_internal(scroll_position, local, true, cx);
            }
        }

        self.scroll_manager.last_autoscroll = Some((
            self.scroll_manager.anchor.offset,
            target_top,
            target_bottom,
            strategy,
        ));

        true
    }

    pub(crate) fn autoscroll_horizontally(
        &mut self,
        start_row: DisplayRow,
        viewport_width: Pixels,
        scroll_width: Pixels,
        max_glyph_width: Pixels,
        layouts: &[LineWithInvisibles],
        cx: &mut ViewContext<Self>,
    ) -> bool {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let selections = self.selections.all::<Point>(cx);

        let mut target_left;
        let mut target_right;

        if self
            .highlighted_display_row_for_autoscroll(&display_map)
            .is_none()
        {
            target_left = px(f32::INFINITY);
            target_right = px(0.);
            for selection in selections {
                let head = selection.head().to_display_point(&display_map);
                if head.row() >= start_row
                    && head.row() < DisplayRow(start_row.0 + layouts.len() as u32)
                {
                    let start_column = head.column().saturating_sub(3);
                    let end_column = cmp::min(display_map.line_len(head.row()), head.column() + 3);
                    target_left = target_left.min(
                        layouts[head.row().minus(start_row) as usize]
                            .x_for_index(start_column as usize),
                    );
                    target_right = target_right.max(
                        layouts[head.row().minus(start_row) as usize]
                            .x_for_index(end_column as usize)
                            + max_glyph_width,
                    );
                }
            }
        } else {
            target_left = px(0.);
            target_right = px(0.);
        }

        target_right = target_right.min(scroll_width);

        if target_right - target_left > viewport_width {
            return false;
        }

        let scroll_left = self.scroll_manager.anchor.offset.x * max_glyph_width;
        let scroll_right = scroll_left + viewport_width;

        if target_left < scroll_left {
            self.scroll_manager.anchor.offset.x = target_left / max_glyph_width;
            true
        } else if target_right > scroll_right {
            self.scroll_manager.anchor.offset.x = (target_right - viewport_width) / max_glyph_width;
            true
        } else {
            false
        }
    }

    pub fn request_autoscroll(&mut self, autoscroll: Autoscroll, cx: &mut ViewContext<Self>) {
        self.scroll_manager.autoscroll_request = Some((autoscroll, true));
        cx.notify();
    }

    pub(crate) fn request_autoscroll_remotely(
        &mut self,
        autoscroll: Autoscroll,
        cx: &mut ViewContext<Self>,
    ) {
        self.scroll_manager.autoscroll_request = Some((autoscroll, false));
        cx.notify();
    }
}
