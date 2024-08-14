use super::Axis;
use crate::{
    Autoscroll, Bias, Editor, EditorMode, NextScreen, NextScrollCursorCenterTopBottom,
    ScrollAnchor, ScrollCursorBottom, ScrollCursorCenter, ScrollCursorCenterTopBottom,
    ScrollCursorTop, SCROLL_CENTER_TOP_BOTTOM_DEBOUNCE_TIMEOUT,
};
use gpui::{Point, ViewContext};

impl Editor {
    pub fn next_screen(&mut self, _: &NextScreen, cx: &mut ViewContext<Editor>) {
        if self.take_rename(true, cx).is_some() {
            return;
        }

        if self.mouse_context_menu.is_some() {
            return;
        }

        if matches!(self.mode, EditorMode::SingleLine { .. }) {
            cx.propagate();
            return;
        }
        self.request_autoscroll(Autoscroll::Next, cx);
    }

    pub fn scroll(
        &mut self,
        scroll_position: Point<f32>,
        axis: Option<Axis>,
        cx: &mut ViewContext<Self>,
    ) {
        self.scroll_manager.update_ongoing_scroll(axis);
        self.set_scroll_position(scroll_position, cx);
    }

    pub fn scroll_cursor_center_top_bottom(
        &mut self,
        _: &ScrollCursorCenterTopBottom,
        cx: &mut ViewContext<Self>,
    ) {
        let snapshot = self.snapshot(cx).display_snapshot;
        let visible_rows = if let Some(visible_rows) = self.visible_line_count() {
            visible_rows as u32
        } else {
            return;
        };

        let scroll_margin_rows = self.vertical_scroll_margin() as u32;
        let mut new_screen_top = self.selections.newest_display(cx).head();
        *new_screen_top.column_mut() = 0;
        match self.next_scroll_position {
            NextScrollCursorCenterTopBottom::Center => {
                *new_screen_top.row_mut() = new_screen_top.row().0.saturating_sub(visible_rows / 2);
            }
            NextScrollCursorCenterTopBottom::Top => {
                *new_screen_top.row_mut() =
                    new_screen_top.row().0.saturating_sub(scroll_margin_rows);
            }
            NextScrollCursorCenterTopBottom::Bottom => {
                *new_screen_top.row_mut() = new_screen_top
                    .row()
                    .0
                    .saturating_sub(visible_rows.saturating_sub(scroll_margin_rows));
            }
        }
        self.set_scroll_anchor(
            ScrollAnchor {
                anchor: snapshot
                    .buffer_snapshot
                    .anchor_before(new_screen_top.to_offset(&snapshot, Bias::Left)),
                offset: Default::default(),
            },
            cx,
        );

        self.next_scroll_position = self.next_scroll_position.next();
        self._scroll_cursor_center_top_bottom_task =
            cx.spawn(|editor, mut cx: gpui::AsyncWindowContext| async move {
                cx.background_executor()
                    .timer(SCROLL_CENTER_TOP_BOTTOM_DEBOUNCE_TIMEOUT)
                    .await;
                editor
                    .update(&mut cx, |editor, _| {
                        editor.next_scroll_position = NextScrollCursorCenterTopBottom::default();
                    })
                    .ok();
            });
    }

    pub fn scroll_cursor_top(&mut self, _: &ScrollCursorTop, cx: &mut ViewContext<Editor>) {
        let snapshot = self.snapshot(cx).display_snapshot;
        let scroll_margin_rows = self.vertical_scroll_margin() as u32;

        let mut new_screen_top = self.selections.newest_display(cx).head();
        *new_screen_top.row_mut() = new_screen_top.row().0.saturating_sub(scroll_margin_rows);
        *new_screen_top.column_mut() = 0;
        let new_screen_top = new_screen_top.to_offset(&snapshot, Bias::Left);
        let new_anchor = snapshot.buffer_snapshot.anchor_before(new_screen_top);

        self.set_scroll_anchor(
            ScrollAnchor {
                anchor: new_anchor,
                offset: Default::default(),
            },
            cx,
        )
    }

    pub fn scroll_cursor_center(&mut self, _: &ScrollCursorCenter, cx: &mut ViewContext<Editor>) {
        let snapshot = self.snapshot(cx).display_snapshot;
        let visible_rows = if let Some(visible_rows) = self.visible_line_count() {
            visible_rows as u32
        } else {
            return;
        };

        let mut new_screen_top = self.selections.newest_display(cx).head();
        *new_screen_top.row_mut() = new_screen_top.row().0.saturating_sub(visible_rows / 2);
        *new_screen_top.column_mut() = 0;
        let new_screen_top = new_screen_top.to_offset(&snapshot, Bias::Left);
        let new_anchor = snapshot.buffer_snapshot.anchor_before(new_screen_top);

        self.set_scroll_anchor(
            ScrollAnchor {
                anchor: new_anchor,
                offset: Default::default(),
            },
            cx,
        )
    }

    pub fn scroll_cursor_bottom(&mut self, _: &ScrollCursorBottom, cx: &mut ViewContext<Editor>) {
        let snapshot = self.snapshot(cx).display_snapshot;
        let scroll_margin_rows = self.vertical_scroll_margin() as u32;
        let visible_rows = if let Some(visible_rows) = self.visible_line_count() {
            visible_rows as u32
        } else {
            return;
        };

        let mut new_screen_top = self.selections.newest_display(cx).head();
        *new_screen_top.row_mut() = new_screen_top
            .row()
            .0
            .saturating_sub(visible_rows.saturating_sub(scroll_margin_rows));
        *new_screen_top.column_mut() = 0;
        let new_screen_top = new_screen_top.to_offset(&snapshot, Bias::Left);
        let new_anchor = snapshot.buffer_snapshot.anchor_before(new_screen_top);

        self.set_scroll_anchor(
            ScrollAnchor {
                anchor: new_anchor,
                offset: Default::default(),
            },
            cx,
        )
    }
}
