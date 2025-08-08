use crate::{
    DisplayRow, Editor, EditorMode, LineWithInvisibles, RowExt, SelectionEffects,
    display_map::ToDisplayPoint, scroll::WasScrolled,
};
use gpui::{Bounds, Context, Pixels, Window, px};
use language::Point;
use multi_buffer::Anchor;
use std::{cmp, f32};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Autoscroll {
    Next,
    Strategy(AutoscrollStrategy, Option<Anchor>),
}

impl Autoscroll {
    /// scrolls the minimal amount to (try) and fit all cursors onscreen
    pub fn fit() -> Self {
        Self::Strategy(AutoscrollStrategy::Fit, None)
    }

    /// scrolls the minimal amount to fit the newest cursor
    pub fn newest() -> Self {
        Self::Strategy(AutoscrollStrategy::Newest, None)
    }

    /// scrolls so the newest cursor is vertically centered
    pub fn center() -> Self {
        Self::Strategy(AutoscrollStrategy::Center, None)
    }

    /// scrolls so the newest cursor is near the top
    /// (offset by vertical_scroll_margin)
    pub fn focused() -> Self {
        Self::Strategy(AutoscrollStrategy::Focused, None)
    }

    /// Scrolls so that the newest cursor is roughly an n-th line from the top.
    pub fn top_relative(n: usize) -> Self {
        Self::Strategy(AutoscrollStrategy::TopRelative(n), None)
    }

    /// Scrolls so that the newest cursor is at the top.
    pub fn top() -> Self {
        Self::Strategy(AutoscrollStrategy::Top, None)
    }

    /// Scrolls so that the newest cursor is roughly an n-th line from the bottom.
    pub fn bottom_relative(n: usize) -> Self {
        Self::Strategy(AutoscrollStrategy::BottomRelative(n), None)
    }

    /// Scrolls so that the newest cursor is at the bottom.
    pub fn bottom() -> Self {
        Self::Strategy(AutoscrollStrategy::Bottom, None)
    }

    /// Applies a given auto-scroll strategy to a given anchor instead of a cursor.
    /// E.G: Autoscroll::center().for_anchor(...) results in the anchor being at the center of the screen.
    pub fn for_anchor(self, anchor: Anchor) -> Self {
        match self {
            Autoscroll::Next => self,
            Autoscroll::Strategy(autoscroll_strategy, _) => {
                Self::Strategy(autoscroll_strategy, Some(anchor))
            }
        }
    }
}

impl Into<SelectionEffects> for Option<Autoscroll> {
    fn into(self) -> SelectionEffects {
        match self {
            Some(autoscroll) => SelectionEffects::scroll(autoscroll),
            None => SelectionEffects::no_scroll(),
        }
    }
}

#[derive(Debug, PartialEq, Eq, Default, Clone, Copy)]
pub enum AutoscrollStrategy {
    Fit,
    Newest,
    #[default]
    Center,
    Focused,
    Top,
    Bottom,
    TopRelative(usize),
    BottomRelative(usize),
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

pub(crate) struct NeedsHorizontalAutoscroll(pub(crate) bool);

impl Editor {
    pub(crate) fn autoscroll_vertically(
        &mut self,
        bounds: Bounds<Pixels>,
        line_height: Pixels,
        max_scroll_top: f32,
        autoscroll_request: Option<(Autoscroll, bool)>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) -> (NeedsHorizontalAutoscroll, WasScrolled) {
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
        if scroll_position.y > max_scroll_top {
            scroll_position.y = max_scroll_top;
        }

        let editor_was_scrolled = if original_y != scroll_position.y {
            self.set_scroll_position(scroll_position, window, cx)
        } else {
            WasScrolled(false)
        };

        let Some((autoscroll, local)) = autoscroll_request else {
            return (NeedsHorizontalAutoscroll(false), editor_was_scrolled);
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

            let selections_fit = target_bottom - target_top <= visible_lines;
            if matches!(
                autoscroll,
                Autoscroll::Strategy(AutoscrollStrategy::Newest, _)
            ) || (matches!(autoscroll, Autoscroll::Strategy(AutoscrollStrategy::Fit, _))
                && !selections_fit)
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
            Autoscroll::Strategy(strategy, _) => strategy,
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
        if let Autoscroll::Strategy(_, Some(anchor)) = autoscroll {
            target_top = anchor.to_display_point(&display_map).row().as_f32();
            target_bottom = target_top + 1.;
        }

        let was_autoscrolled = match strategy {
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
                } else if !needs_scroll_up && needs_scroll_down {
                    scroll_position.y = target_bottom - visible_lines;
                }

                if needs_scroll_up ^ needs_scroll_down {
                    self.set_scroll_position_internal(scroll_position, local, true, window, cx)
                } else {
                    WasScrolled(false)
                }
            }
            AutoscrollStrategy::Center => {
                scroll_position.y = (target_top - margin).max(0.0);
                self.set_scroll_position_internal(scroll_position, local, true, window, cx)
            }
            AutoscrollStrategy::Focused => {
                let margin = margin.min(self.scroll_manager.vertical_scroll_margin);
                scroll_position.y = (target_top - margin).max(0.0);
                self.set_scroll_position_internal(scroll_position, local, true, window, cx)
            }
            AutoscrollStrategy::Top => {
                scroll_position.y = (target_top).max(0.0);
                self.set_scroll_position_internal(scroll_position, local, true, window, cx)
            }
            AutoscrollStrategy::Bottom => {
                scroll_position.y = (target_bottom - visible_lines).max(0.0);
                self.set_scroll_position_internal(scroll_position, local, true, window, cx)
            }
            AutoscrollStrategy::TopRelative(lines) => {
                scroll_position.y = target_top - lines as f32;
                self.set_scroll_position_internal(scroll_position, local, true, window, cx)
            }
            AutoscrollStrategy::BottomRelative(lines) => {
                scroll_position.y = target_bottom + lines as f32;
                self.set_scroll_position_internal(scroll_position, local, true, window, cx)
            }
        };

        self.scroll_manager.last_autoscroll = Some((
            self.scroll_manager.anchor.offset,
            target_top,
            target_bottom,
            strategy,
        ));

        let was_scrolled = WasScrolled(editor_was_scrolled.0 || was_autoscrolled.0);
        (NeedsHorizontalAutoscroll(true), was_scrolled)
    }

    pub(crate) fn autoscroll_horizontally(
        &mut self,
        start_row: DisplayRow,
        viewport_width: Pixels,
        scroll_width: Pixels,
        em_advance: Pixels,
        layouts: &[LineWithInvisibles],
        autoscroll_request: Option<(Autoscroll, bool)>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) -> Option<gpui::Point<f32>> {
        let (_, local) = autoscroll_request?;

        let display_map = self.display_map.update(cx, |map, cx| map.snapshot(cx));
        let selections = self.selections.all::<Point>(cx);
        let mut scroll_position = self.scroll_manager.scroll_position(&display_map);

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
                    let start_column = head.column();
                    let end_column = cmp::min(display_map.line_len(head.row()), head.column());
                    target_left = target_left.min(
                        layouts[head.row().minus(start_row) as usize]
                            .x_for_index(start_column as usize)
                            + self.gutter_dimensions.margin,
                    );
                    target_right = target_right.max(
                        layouts[head.row().minus(start_row) as usize]
                            .x_for_index(end_column as usize)
                            + em_advance,
                    );
                }
            }
        } else {
            target_left = px(0.);
            target_right = px(0.);
        }

        target_right = target_right.min(scroll_width);

        if target_right - target_left > viewport_width {
            return None;
        }

        let scroll_left = self.scroll_manager.anchor.offset.x * em_advance;
        let scroll_right = scroll_left + viewport_width;

        let was_scrolled = if target_left < scroll_left {
            scroll_position.x = target_left / em_advance;
            self.set_scroll_position_internal(scroll_position, local, true, window, cx)
        } else if target_right > scroll_right {
            scroll_position.x = (target_right - viewport_width) / em_advance;
            self.set_scroll_position_internal(scroll_position, local, true, window, cx)
        } else {
            WasScrolled(false)
        };

        if was_scrolled.0 {
            Some(scroll_position)
        } else {
            None
        }
    }

    pub fn request_autoscroll(&mut self, autoscroll: Autoscroll, cx: &mut Context<Self>) {
        self.scroll_manager.autoscroll_request = Some((autoscroll, true));
        cx.notify();
    }

    pub(crate) fn request_autoscroll_remotely(
        &mut self,
        autoscroll: Autoscroll,
        cx: &mut Context<Self>,
    ) {
        self.scroll_manager.autoscroll_request = Some((autoscroll, false));
        cx.notify();
    }
}
