use std::time::Duration;

use crate::{
    motion::Motion,
    object::Object,
    state::{Mode, Register},
    Vim,
};
use collections::HashMap;
use editor::{ClipboardSelection, Editor};
use gpui::WindowContext;
use language::Point;
use multi_buffer::MultiBufferRow;
use ui::ViewContext;

pub fn yank_motion(vim: &mut Vim, motion: Motion, times: Option<usize>, cx: &mut WindowContext) {
    vim.update_active_editor(cx, |vim, editor, cx| {
        let text_layout_details = editor.text_layout_details(cx);
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
            yank_selections_content(vim, editor, motion.linewise(), cx);
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
    vim.update_active_editor(cx, |vim, editor, cx| {
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
            yank_selections_content(vim, editor, false, cx);
            editor.change_selections(None, cx, |s| {
                s.move_with(|_, selection| {
                    let (head, goal) = original_positions.remove(&selection.id).unwrap();
                    selection.collapse_to(head, goal);
                });
            });
        });
    });
}

pub fn yank_selections_content(
    vim: &mut Vim,
    editor: &mut Editor,
    linewise: bool,
    cx: &mut ViewContext<Editor>,
) {
    copy_selections_content_internal(vim, editor, linewise, true, cx);
}

pub fn copy_selections_content(
    vim: &mut Vim,
    editor: &mut Editor,
    linewise: bool,
    cx: &mut ViewContext<Editor>,
) {
    copy_selections_content_internal(vim, editor, linewise, false, cx);
}

struct HighlightOnYank;

fn copy_selections_content_internal(
    vim: &mut Vim,
    editor: &mut Editor,
    linewise: bool,
    is_yank: bool,
    cx: &mut ViewContext<Editor>,
) {
    let selections = editor.selections.all_adjusted(cx);
    let buffer = editor.buffer().read(cx).snapshot(cx);
    let mut text = String::new();
    let mut clipboard_selections = Vec::with_capacity(selections.len());
    let mut ranges_to_highlight = Vec::new();

    vim.update_state(|state| {
        state.marks.insert(
            "[".to_string(),
            selections
                .iter()
                .map(|s| buffer.anchor_before(s.start))
                .collect(),
        );
        state.marks.insert(
            "]".to_string(),
            selections
                .iter()
                .map(|s| buffer.anchor_after(s.end))
                .collect(),
        )
    });

    {
        let mut is_first = true;
        for selection in selections.iter() {
            let mut start = selection.start;
            let end = selection.end;
            if is_first {
                is_first = false;
            } else {
                text.push_str("\n");
            }
            let initial_len = text.len();

            // if the file does not end with \n, and our line-mode selection ends on
            // that line, we will have expanded the start of the selection to ensure it
            // contains a newline (so that delete works as expected). We undo that change
            // here.
            let is_last_line = linewise
                && end.row == buffer.max_buffer_row().0
                && buffer.max_point().column > 0
                && start.row < buffer.max_buffer_row().0
                && start == Point::new(start.row, buffer.line_len(MultiBufferRow(start.row)));

            if is_last_line {
                start = Point::new(start.row + 1, 0);
            }

            let start_anchor = buffer.anchor_after(start);
            let end_anchor = buffer.anchor_before(end);
            ranges_to_highlight.push(start_anchor..end_anchor);

            for chunk in buffer.text_for_range(start..end) {
                text.push_str(chunk);
            }
            if is_last_line {
                text.push_str("\n");
            }
            clipboard_selections.push(ClipboardSelection {
                len: text.len() - initial_len,
                is_entire_line: linewise,
                first_line_indent: buffer.indent_size_for_line(MultiBufferRow(start.row)).len,
            });
        }
    }

    let selected_register = vim.update_state(|state| state.selected_register.take());
    vim.write_registers(
        Register {
            text: text.into(),
            clipboard_selections: Some(clipboard_selections),
        },
        selected_register,
        is_yank,
        linewise,
        cx,
    );

    if !is_yank || vim.state().mode == Mode::Visual {
        return;
    }

    editor.highlight_background::<HighlightOnYank>(
        &ranges_to_highlight,
        |colors| colors.editor_document_highlight_read_background,
        cx,
    );
    cx.spawn(|this, mut cx| async move {
        cx.background_executor()
            .timer(Duration::from_millis(200))
            .await;
        this.update(&mut cx, |editor, cx| {
            editor.clear_background_highlights::<HighlightOnYank>(cx)
        })
        .ok();
    })
    .detach();
}
