use crate::{motion::Motion, object::Object, Vim};
use collections::HashMap;
use gpui::WindowContext;

#[derive(PartialEq, Eq)]
pub(super) enum IndentDirection {
    In,
    Out
}

pub fn indent_motion(
    vim: &mut Vim,
    motion: Motion,
    times: Option<usize>,
    dir: IndentDirection,
    cx: &mut WindowContext
) {
    vim.stop_recording();
    vim.update_active_editor(cx, |_, editor, cx| {
        let text_layout_details = editor.text_layout_details(cx);
        editor.transact(cx, |editor, cx| {
            let mut original_positions: HashMap<_, _> = Default::default();
            editor.change_selections(None, cx, |s| {
                s.move_with(|map, selection| {
                    let cursor = selection.head();
                    original_positions.insert(selection.id, (cursor, map.line_len(cursor.row())));
                    motion.expand_selection(map, selection, times, false, &text_layout_details);
                });
            });
            if dir == IndentDirection::In {
                editor.indent(&Default::default(), cx);
            } else {
                editor.outdent(&Default::default(), cx);
            }
            editor.change_selections(None, cx, |s| {
                s.move_with(|map, selection| {
                    let (mut cursor, line_len) = original_positions.remove(
                        &selection.id
                    ).unwrap();
                    if dir == IndentDirection::In {
                        *cursor.column_mut() += map.line_len(cursor.row()) - line_len;
                    } else {
                        *cursor.column_mut() -= line_len - map.line_len(cursor.row());
                    }
                    selection.collapse_to(cursor, selection.goal);
                });
            });
        });
    });
}

pub fn indent_object(
    vim: &mut Vim,
    object: Object,
    around: bool,
    dir: IndentDirection,
    cx: &mut WindowContext
) {
    vim.stop_recording();
    vim.update_active_editor(cx, |_, editor, cx| {
        editor.transact(cx, |editor, cx| {
            let mut original_positions: HashMap<_, _> = Default::default();
            editor.change_selections(None, cx, |s| {
                s.move_with(|map, selection| {
                    let cursor = selection.head();
                    original_positions.insert(selection.id, (cursor, map.line_len(cursor.row())));
                    object.expand_selection(map, selection, around);
                });
            });
            if dir == IndentDirection::In {
                editor.indent(&Default::default(), cx);
            } else {
                editor.outdent(&Default::default(), cx);
            }
            editor.change_selections(None, cx, |s| {
                s.move_with(|map, selection| {
                    let (mut cursor, line_len) = original_positions.remove(
                        &selection.id
                    ).unwrap();
                    if dir == IndentDirection::In {
                        *cursor.column_mut() += map.line_len(cursor.row()) - line_len;
                    } else {
                        *cursor.column_mut() -= line_len - map.line_len(cursor.row());
                    }
                    selection.collapse_to(cursor, selection.goal);
                });
            });
        });
    });
}
