use std::cmp;

use gpui::ViewContext;
use language::Point;

use crate::{display_map::ToDisplayPoint, Editor, EditorMode, LineWithInvisibles};

#[derive(PartialEq, Eq)]
pub enum Autoscroll {
    Next,
    Strategy(AutoscrollStrategy),
}

impl Autoscroll {
    pub fn fit() -> Self {
        Self::Strategy(AutoscrollStrategy::Fit)
    }

    pub fn newest() -> Self {
        Self::Strategy(AutoscrollStrategy::Newest)
    }

    pub fn center() -> Self {
        Self::Strategy(AutoscrollStrategy::Center)
    }
}

#[derive(PartialEq, Eq, Default)]
pub enum AutoscrollStrategy {
    Fit,
    Newest,
    #[default]
    Center,
    Top,
    Bottom,
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
    pub fn autoscroll_vertically(
        &mut self,
        viewport_height: f32,
        line_height: f32,
        cx: &mut ViewContext<Editor>,
    ) -> bool {
        let visible_lines = viewport_height / line_height;
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let mut scroll_position = self.scroll_manager.scroll_position(&display_map);
        let max_scroll_top = if matches!(self.mode, EditorMode::AutoHeight { .. }) {
            (display_map.max_point().row() as f32 - visible_lines + 1.).max(0.)
        } else {
            display_map.max_point().row() as f32
        };
        if scroll_position.y() > max_scroll_top {
            scroll_position.set_y(max_scroll_top);
            self.set_scroll_position(scroll_position, cx);
        }

        let (autoscroll, local) =
            if let Some(autoscroll) = self.scroll_manager.autoscroll_request.take() {
                autoscroll
            } else {
                return false;
            };

        let first_cursor_top;
        let last_cursor_bottom;
        if let Some(highlighted_rows) = &self.highlighted_rows {
            first_cursor_top = highlighted_rows.start as f32;
            last_cursor_bottom = first_cursor_top + 1.;
        } else if autoscroll == Autoscroll::newest() {
            let newest_selection = self.selections.newest::<Point>(cx);
            first_cursor_top = newest_selection.head().to_display_point(&display_map).row() as f32;
            last_cursor_bottom = first_cursor_top + 1.;
        } else {
            let selections = self.selections.all::<Point>(cx);
            first_cursor_top = selections
                .first()
                .unwrap()
                .head()
                .to_display_point(&display_map)
                .row() as f32;
            last_cursor_bottom = selections
                .last()
                .unwrap()
                .head()
                .to_display_point(&display_map)
                .row() as f32
                + 1.0;
        }

        let margin = if matches!(self.mode, EditorMode::AutoHeight { .. }) {
            0.
        } else {
            ((visible_lines - (last_cursor_bottom - first_cursor_top)) / 2.0).floor()
        };
        if margin < 0.0 {
            return false;
        }

        let strategy = match autoscroll {
            Autoscroll::Strategy(strategy) => strategy,
            Autoscroll::Next => {
                let last_autoscroll = &self.scroll_manager.last_autoscroll;
                if let Some(last_autoscroll) = last_autoscroll {
                    if self.scroll_manager.anchor.offset == last_autoscroll.0
                        && first_cursor_top == last_autoscroll.1
                        && last_cursor_bottom == last_autoscroll.2
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
                let target_top = (first_cursor_top - margin).max(0.0);
                let target_bottom = last_cursor_bottom + margin;
                let start_row = scroll_position.y();
                let end_row = start_row + visible_lines;

                if target_top < start_row {
                    scroll_position.set_y(target_top);
                    self.set_scroll_position_internal(scroll_position, local, true, cx);
                } else if target_bottom >= end_row {
                    scroll_position.set_y(target_bottom - visible_lines);
                    self.set_scroll_position_internal(scroll_position, local, true, cx);
                }
            }
            AutoscrollStrategy::Center => {
                scroll_position.set_y((first_cursor_top - margin).max(0.0));
                self.set_scroll_position_internal(scroll_position, local, true, cx);
            }
            AutoscrollStrategy::Top => {
                scroll_position.set_y((first_cursor_top).max(0.0));
                self.set_scroll_position_internal(scroll_position, local, true, cx);
            }
            AutoscrollStrategy::Bottom => {
                scroll_position.set_y((last_cursor_bottom - visible_lines).max(0.0));
                self.set_scroll_position_internal(scroll_position, local, true, cx);
            }
        }

        self.scroll_manager.last_autoscroll = Some((
            self.scroll_manager.anchor.offset,
            first_cursor_top,
            last_cursor_bottom,
            strategy,
        ));

        true
    }

    pub fn autoscroll_horizontally(
        &mut self,
        start_row: u32,
        viewport_width: f32,
        scroll_width: f32,
        max_glyph_width: f32,
        layouts: &[LineWithInvisibles],
        cx: &mut ViewContext<Self>,
    ) -> bool {
        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let selections = self.selections.all::<Point>(cx);

        let mut target_left;
        let mut target_right;

        if self.highlighted_rows.is_some() {
            target_left = 0.0_f32;
            target_right = 0.0_f32;
        } else {
            target_left = std::f32::INFINITY;
            target_right = 0.0_f32;
            for selection in selections {
                let head = selection.head().to_display_point(&display_map);
                if head.row() >= start_row && head.row() < start_row + layouts.len() as u32 {
                    let start_column = head.column().saturating_sub(3);
                    let end_column = cmp::min(display_map.line_len(head.row()), head.column() + 3);
                    target_left = target_left.min(
                        layouts[(head.row() - start_row) as usize]
                            .line
                            .x_for_index(start_column as usize),
                    );
                    target_right = target_right.max(
                        layouts[(head.row() - start_row) as usize]
                            .line
                            .x_for_index(end_column as usize)
                            + max_glyph_width,
                    );
                }
            }
        }

        target_right = target_right.min(scroll_width);

        if target_right - target_left > viewport_width {
            return false;
        }

        let scroll_left = self.scroll_manager.anchor.offset.x() * max_glyph_width;
        let scroll_right = scroll_left + viewport_width;

        if target_left < scroll_left {
            self.scroll_manager
                .anchor
                .offset
                .set_x(target_left / max_glyph_width);
            true
        } else if target_right > scroll_right {
            self.scroll_manager
                .anchor
                .offset
                .set_x((target_right - viewport_width) / max_glyph_width);
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
