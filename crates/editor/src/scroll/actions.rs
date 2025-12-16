use super::Axis;
use crate::{
    Autoscroll, Editor, EditorMode, EditorSettings, NextScreen, NextScrollCursorCenterTopBottom,
    SCROLL_CENTER_TOP_BOTTOM_DEBOUNCE_TIMEOUT, ScrollCursorBottom, ScrollCursorCenter,
    ScrollCursorCenterTopBottom, ScrollCursorTop, display_map::DisplayRow, scroll::ScrollOffset,
};
use gpui::{Context, Point, Window};
use settings::Settings;

impl Editor {
    pub fn next_screen(&mut self, _: &NextScreen, window: &mut Window, cx: &mut Context<Editor>) {
        if self.take_rename(true, window, cx).is_some() {
            return;
        }

        if self.mouse_context_menu.is_some() {
            return;
        }

        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate();
            return;
        }
        self.request_autoscroll(Autoscroll::Next, cx);
    }

    pub fn scroll(
        &mut self,
        scroll_position: Point<ScrollOffset>,
        axis: Option<Axis>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.scroll_manager.update_ongoing_scroll(axis);
        self.set_scroll_position(scroll_position, window, cx);
    }

    pub fn scroll_animated(
        &mut self,
        scroll_position: Point<ScrollOffset>,
        axis: Option<Axis>,
        cx: &mut Context<Self>,
    ) {
        let current_position = self.scroll_position(cx);
        self.scroll_manager.update_ongoing_scroll(axis);
        self.scroll_manager
            .start_animation(current_position, scroll_position);

        cx.notify();
    }

    pub fn scroll_cursor_center_top_bottom(
        &mut self,
        _: &ScrollCursorCenterTopBottom,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        match self.next_scroll_position {
            NextScrollCursorCenterTopBottom::Center => {
                self.scroll_cursor_center(&Default::default(), window, cx);
            }
            NextScrollCursorCenterTopBottom::Top => {
                self.scroll_cursor_top(&Default::default(), window, cx);
            }
            NextScrollCursorCenterTopBottom::Bottom => {
                self.scroll_cursor_bottom(&Default::default(), window, cx);
            }
        }

        self.next_scroll_position = self.next_scroll_position.next();
        self._scroll_cursor_center_top_bottom_task = cx.spawn(async move |editor, cx| {
            cx.background_executor()
                .timer(SCROLL_CENTER_TOP_BOTTOM_DEBOUNCE_TIMEOUT)
                .await;
            editor
                .update(cx, |editor, _| {
                    editor.next_scroll_position = NextScrollCursorCenterTopBottom::default();
                })
                .ok();
        });
    }

    pub fn scroll_cursor_top(
        &mut self,
        _: &ScrollCursorTop,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let display_snapshot = self.display_snapshot(cx);
        let scroll_margin_rows = self.vertical_scroll_margin() as u32;
        let new_screen_top = self
            .selections
            .newest_display(&display_snapshot)
            .head()
            .row()
            .0;
        let header_offset = display_snapshot
            .buffer_snapshot()
            .show_headers()
            .then(|| display_snapshot.buffer_header_height())
            .unwrap_or(0);
        let new_screen_top = new_screen_top.saturating_sub(scroll_margin_rows + header_offset);
        self.set_scroll_top_row(DisplayRow(new_screen_top), window, cx);
    }

    pub fn scroll_cursor_center(
        &mut self,
        _: &ScrollCursorCenter,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let Some(visible_rows) = self.visible_line_count().map(|count| count as u32) else {
            return;
        };
        let new_screen_top = self
            .selections
            .newest_display(&self.display_snapshot(cx))
            .head()
            .row()
            .0;
        let new_screen_top = new_screen_top.saturating_sub(visible_rows / 2);
        self.set_scroll_top_row(DisplayRow(new_screen_top), window, cx);
    }

    pub fn scroll_cursor_bottom(
        &mut self,
        _: &ScrollCursorBottom,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let scroll_margin_rows = self.vertical_scroll_margin() as u32;
        let Some(visible_rows) = self.visible_line_count().map(|count| count as u32) else {
            return;
        };
        let new_screen_top = self
            .selections
            .newest_display(&self.display_snapshot(cx))
            .head()
            .row()
            .0;
        let new_screen_top =
            new_screen_top.saturating_sub(visible_rows.saturating_sub(scroll_margin_rows));
        self.set_scroll_top_row(DisplayRow(new_screen_top), window, cx);
    }
}
