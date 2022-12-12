use gpui::{
    actions, geometry::vector::Vector2F, impl_internal_actions, Axis, MutableAppContext,
    ViewContext,
};
use language::Bias;

use crate::{Editor, EditorMode};

use super::{autoscroll::Autoscroll, scroll_amount::ScrollAmount, ScrollAnchor};

actions!(
    editor,
    [
        LineDown,
        LineUp,
        HalfPageDown,
        HalfPageUp,
        PageDown,
        PageUp,
        NextScreen,
        ScrollCursorTop,
        ScrollCursorCenter,
        ScrollCursorBottom,
    ]
);

#[derive(Clone, PartialEq)]
pub struct Scroll {
    pub scroll_position: Vector2F,
    pub axis: Option<Axis>,
}

impl_internal_actions!(editor, [Scroll]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(Editor::next_screen);
    cx.add_action(Editor::scroll);
    cx.add_action(Editor::scroll_cursor_top);
    cx.add_action(Editor::scroll_cursor_center);
    cx.add_action(Editor::scroll_cursor_bottom);
    cx.add_action(|this: &mut Editor, _: &LineDown, cx| {
        this.scroll_screen(&ScrollAmount::LineDown, cx)
    });
    cx.add_action(|this: &mut Editor, _: &LineUp, cx| {
        this.scroll_screen(&ScrollAmount::LineUp, cx)
    });
    cx.add_action(|this: &mut Editor, _: &HalfPageDown, cx| {
        this.scroll_screen(&ScrollAmount::HalfPageDown, cx)
    });
    cx.add_action(|this: &mut Editor, _: &HalfPageUp, cx| {
        this.scroll_screen(&ScrollAmount::HalfPageUp, cx)
    });
    cx.add_action(|this: &mut Editor, _: &PageDown, cx| {
        this.scroll_screen(&ScrollAmount::PageDown, cx)
    });
    cx.add_action(|this: &mut Editor, _: &PageUp, cx| {
        this.scroll_screen(&ScrollAmount::PageUp, cx)
    });
}

impl Editor {
    pub fn next_screen(&mut self, _: &NextScreen, cx: &mut ViewContext<Editor>) -> Option<()> {
        if self.take_rename(true, cx).is_some() {
            return None;
        }

        if self.mouse_context_menu.read(cx).visible() {
            return None;
        }

        if matches!(self.mode, EditorMode::SingleLine) {
            cx.propagate_action();
            return None;
        }
        self.request_autoscroll(Autoscroll::Next, cx);
        Some(())
    }

    fn scroll(&mut self, action: &Scroll, cx: &mut ViewContext<Self>) {
        self.scroll_manager.update_ongoing_scroll(action.axis);
        self.set_scroll_position(action.scroll_position, cx);
    }

    fn scroll_cursor_top(editor: &mut Editor, _: &ScrollCursorTop, cx: &mut ViewContext<Editor>) {
        let snapshot = editor.snapshot(cx).display_snapshot;
        let scroll_margin_rows = editor.vertical_scroll_margin() as u32;

        let mut new_screen_top = editor.selections.newest_display(cx).head();
        *new_screen_top.row_mut() = new_screen_top.row().saturating_sub(scroll_margin_rows);
        *new_screen_top.column_mut() = 0;
        let new_screen_top = new_screen_top.to_offset(&snapshot, Bias::Left);
        let new_anchor = snapshot.buffer_snapshot.anchor_before(new_screen_top);

        editor.set_scroll_anchor(
            ScrollAnchor {
                top_anchor: new_anchor,
                offset: Default::default(),
            },
            cx,
        )
    }

    fn scroll_cursor_center(
        editor: &mut Editor,
        _: &ScrollCursorCenter,
        cx: &mut ViewContext<Editor>,
    ) {
        let snapshot = editor.snapshot(cx).display_snapshot;
        let visible_rows = if let Some(visible_rows) = editor.visible_line_count() {
            visible_rows as u32
        } else {
            return;
        };

        let mut new_screen_top = editor.selections.newest_display(cx).head();
        *new_screen_top.row_mut() = new_screen_top.row().saturating_sub(visible_rows / 2);
        *new_screen_top.column_mut() = 0;
        let new_screen_top = new_screen_top.to_offset(&snapshot, Bias::Left);
        let new_anchor = snapshot.buffer_snapshot.anchor_before(new_screen_top);

        editor.set_scroll_anchor(
            ScrollAnchor {
                top_anchor: new_anchor,
                offset: Default::default(),
            },
            cx,
        )
    }

    fn scroll_cursor_bottom(
        editor: &mut Editor,
        _: &ScrollCursorBottom,
        cx: &mut ViewContext<Editor>,
    ) {
        let snapshot = editor.snapshot(cx).display_snapshot;
        let scroll_margin_rows = editor.vertical_scroll_margin() as u32;
        let visible_rows = if let Some(visible_rows) = editor.visible_line_count() {
            visible_rows as u32
        } else {
            return;
        };

        let mut new_screen_top = editor.selections.newest_display(cx).head();
        *new_screen_top.row_mut() = new_screen_top
            .row()
            .saturating_sub(visible_rows.saturating_sub(scroll_margin_rows));
        *new_screen_top.column_mut() = 0;
        let new_screen_top = new_screen_top.to_offset(&snapshot, Bias::Left);
        let new_anchor = snapshot.buffer_snapshot.anchor_before(new_screen_top);

        editor.set_scroll_anchor(
            ScrollAnchor {
                top_anchor: new_anchor,
                offset: Default::default(),
            },
            cx,
        )
    }
}
