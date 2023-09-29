use crate::{motion::Motion, object::Object, utils::copy_selections_content, Vim};
use collections::HashMap;
use editor::movement::TextLayoutDetails;
use gpui::WindowContext;

pub fn yank_motion(vim: &mut Vim, motion: Motion, times: Option<usize>, cx: &mut WindowContext) {
    vim.update_active_editor(cx, |editor, cx| {
        let text_layout_details = TextLayoutDetails::new(editor, cx);
        editor.transact(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            let mut original_positions: HashMap<_, _> = Default::default();
            editor.change_selections(None, cx, |s| {
                s.move_with(|map, selection| {
                    let original_position = (selection.head(), selection.goal);
                    original_positions.insert(selection.id, original_position);
                    motion.expand_selection(map, selection, times, true, &text_layout_details);
                });
            });
            copy_selections_content(editor, motion.linewise(), cx);
            editor.change_selections(None, cx, |s| {
                s.move_with(|_, selection| {
                    let (head, goal) = original_positions.remove(&selection.id).unwrap();
                    selection.collapse_to(head, goal);
                });
            });
        });
    });
}

pub fn yank_object(vim: &mut Vim, object: Object, around: bool, cx: &mut WindowContext) {
    vim.update_active_editor(cx, |editor, cx| {
        editor.transact(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            let mut original_positions: HashMap<_, _> = Default::default();
            editor.change_selections(None, cx, |s| {
                s.move_with(|map, selection| {
                    let original_position = (selection.head(), selection.goal);
                    object.expand_selection(map, selection, around);
                    original_positions.insert(selection.id, original_position);
                });
            });
            copy_selections_content(editor, false, cx);
            editor.change_selections(None, cx, |s| {
                s.move_with(|_, selection| {
                    let (head, goal) = original_positions.remove(&selection.id).unwrap();
                    selection.collapse_to(head, goal);
                });
            });
        });
    });
}
