use std::{ops::Range, time::Duration};

use crate::{
    motion::Motion,
    object::Object,
    state::{Mode, Register},
    Vim, VimSettings,
};
use collections::HashMap;
use editor::{ClipboardSelection, Editor};
use gpui::Context;
use gpui::Window;
use language::Point;
use multi_buffer::MultiBufferRow;
use settings::Settings;

struct HighlightOnYank;

impl Vim {
    pub fn yank_motion(
        &mut self,
        motion: Motion,
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_editor(window, cx, |vim, editor, window, cx| {
            let text_layout_details = editor.text_layout_details(window);
            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let mut original_positions: HashMap<_, _> = Default::default();
                editor.change_selections(None, window, cx, |s| {
                    s.move_with(|map, selection| {
                        let original_position = (selection.head(), selection.goal);
                        original_positions.insert(selection.id, original_position);
                        motion.expand_selection(map, selection, times, true, &text_layout_details);
                    });
                });
                vim.yank_selections_content(editor, motion.linewise(), cx);
                editor.change_selections(None, window, cx, |s| {
                    s.move_with(|_, selection| {
                        let (head, goal) = original_positions.remove(&selection.id).unwrap();
                        selection.collapse_to(head, goal);
                    });
                });
            });
        });
        self.exit_temporary_normal(window, cx);
    }

    pub fn yank_object(
        &mut self,
        object: Object,
        around: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_editor(window, cx, |vim, editor, window, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let mut original_positions: HashMap<_, _> = Default::default();
                editor.change_selections(None, window, cx, |s| {
                    s.move_with(|map, selection| {
                        let original_position = (selection.head(), selection.goal);
                        object.expand_selection(map, selection, around);
                        original_positions.insert(selection.id, original_position);
                    });
                });
                vim.yank_selections_content(editor, false, cx);
                editor.change_selections(None, window, cx, |s| {
                    s.move_with(|_, selection| {
                        let (head, goal) = original_positions.remove(&selection.id).unwrap();
                        selection.collapse_to(head, goal);
                    });
                });
            });
        });
        self.exit_temporary_normal(window, cx);
    }

    pub fn yank_selections_content(
        &mut self,
        editor: &mut Editor,
        linewise: bool,
        cx: &mut Context<Editor>,
    ) {
        self.copy_ranges(
            editor,
            linewise,
            true,
            editor
                .selections
                .all_adjusted(cx)
                .iter()
                .map(|s| s.range())
                .collect(),
            cx,
        )
    }

    pub fn copy_selections_content(
        &mut self,
        editor: &mut Editor,
        linewise: bool,
        cx: &mut Context<Editor>,
    ) {
        self.copy_ranges(
            editor,
            linewise,
            false,
            editor
                .selections
                .all_adjusted(cx)
                .iter()
                .map(|s| s.range())
                .collect(),
            cx,
        )
    }

    pub(crate) fn copy_ranges(
        &mut self,
        editor: &mut Editor,
        linewise: bool,
        is_yank: bool,
        selections: Vec<Range<Point>>,
        cx: &mut Context<Editor>,
    ) {
        let buffer = editor.buffer().read(cx).snapshot(cx);
        let mut text = String::new();
        let mut clipboard_selections = Vec::with_capacity(selections.len());
        let mut ranges_to_highlight = Vec::new();

        self.marks.insert(
            "[".to_string(),
            selections
                .iter()
                .map(|s| buffer.anchor_before(s.start))
                .collect(),
        );
        self.marks.insert(
            "]".to_string(),
            selections
                .iter()
                .map(|s| buffer.anchor_after(s.end))
                .collect(),
        );

        {
            let mut is_first = true;
            for selection in selections.iter() {
                let mut start = selection.start;
                let end = selection.end;
                if is_first {
                    is_first = false;
                } else {
                    text.push('\n');
                }
                let initial_len = text.len();

                // if the file does not end with \n, and our line-mode selection ends on
                // that line, we will have expanded the start of the selection to ensure it
                // contains a newline (so that delete works as expected). We undo that change
                // here.
                let max_point = buffer.max_point();
                let should_adjust_start = linewise
                    && end.row == max_point.row
                    && max_point.column > 0
                    && start.row < max_point.row
                    && start == Point::new(start.row, buffer.line_len(MultiBufferRow(start.row)));
                let should_add_newline =
                    should_adjust_start || (end == max_point && max_point.column > 0 && linewise);

                if should_adjust_start {
                    start = Point::new(start.row + 1, 0);
                }

                let start_anchor = buffer.anchor_after(start);
                let end_anchor = buffer.anchor_before(end);
                ranges_to_highlight.push(start_anchor..end_anchor);

                for chunk in buffer.text_for_range(start..end) {
                    text.push_str(chunk);
                }
                if should_add_newline {
                    text.push('\n');
                }
                clipboard_selections.push(ClipboardSelection {
                    len: text.len() - initial_len,
                    is_entire_line: linewise,
                    start_column: start.column,
                });
            }
        }

        let selected_register = self.selected_register.take();
        Vim::update_globals(cx, |globals, cx| {
            globals.write_registers(
                Register {
                    text: text.into(),
                    clipboard_selections: Some(clipboard_selections),
                },
                selected_register,
                is_yank,
                linewise,
                cx,
            )
        });

        let highlight_duration = VimSettings::get_global(cx).highlight_on_yank_duration;
        if !is_yank || self.mode == Mode::Visual || highlight_duration == 0 {
            return;
        }

        editor.highlight_background::<HighlightOnYank>(
            &ranges_to_highlight,
            |colors| colors.editor_document_highlight_read_background,
            cx,
        );
        cx.spawn(|this, mut cx| async move {
            cx.background_executor()
                .timer(Duration::from_millis(highlight_duration))
                .await;
            this.update(&mut cx, |editor, cx| {
                editor.clear_background_highlights::<HighlightOnYank>(cx)
            })
            .ok();
        })
        .detach();
    }
}
