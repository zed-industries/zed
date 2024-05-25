use crate::{motion::Motion, state::Mode, Vim};
use collections::HashMap;
use editor::{scroll::Autoscroll, Bias};
use gpui::WindowContext;

pub fn indent_motion(
    vim: &mut Vim,
    motion: Motion,
    times: Option<usize>,
    outdent: bool,
    cx: &mut WindowContext,
) {
    vim.stop_recording();
    vim.update_active_editor(cx, |_, editor, cx| {
        let text_layout_details = editor.text_layout_details(cx);
        editor.transact(cx, |editor, cx| {
            // What does this do? It fixes the issue when doing >G
            editor.set_clip_at_line_ends(false, cx);

            let mut original_columns: HashMap<_, _> = Default::default();

            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    let original_head = selection.head();
                    original_columns.insert(
                        selection.id,
                        (original_head.row().0, original_head.column()),
                    );
                    // set expand_to_surrounding_newline to false so that
                    // we don't accidentally indent the line above when doing >G
                    motion.expand_selection(map, selection, times, false, &text_layout_details);
                });
            });

            if outdent {
                editor.outdent(&Default::default(), cx);
            } else {
                editor.indent(&Default::default(), cx);
            }

            // Fixup cursor position after the indentation
            editor.set_clip_at_line_ends(true, cx);

            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    let mut cursor = selection.head();
                    if let Some(pos) = original_columns.get(&selection.id) {
                        *cursor.row_mut() = pos.0;
                        *cursor.column_mut() = pos.1;
                    }
                    cursor = map.clip_point(cursor, Bias::Left);
                    selection.collapse_to(cursor, selection.goal)
                });
            });
        });
    });

    // move_cursor(vim, Motion::FirstNonWhitespace { display_lines: false }, None, cx);

    if vim.state().mode.is_visual() {
        vim.switch_mode(Mode::Normal, false, cx)
    }
}
