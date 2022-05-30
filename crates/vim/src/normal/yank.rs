use crate::{motion::Motion, utils::copy_selections_content, Vim};
use collections::HashMap;
use gpui::MutableAppContext;

pub fn yank_over(vim: &mut Vim, motion: Motion, cx: &mut MutableAppContext) {
    vim.update_active_editor(cx, |editor, cx| {
        editor.transact(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            let mut original_positions: HashMap<_, _> = Default::default();
            editor.change_selections(None, cx, |s| {
                s.move_with(|map, selection| {
                    let original_position = (selection.head(), selection.goal);
                    motion.expand_selection(map, selection, true);
                    original_positions.insert(selection.id, original_position);
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
