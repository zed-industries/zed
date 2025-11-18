use std::{ops::Range, time::Duration};

use crate::{
    Vim, VimSettings,
    motion::{Motion, MotionKind},
    object::Object,
    state::{Mode, Register},
};
use collections::HashMap;
use editor::{ClipboardSelection, Editor, SelectionEffects};
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
        forced_motion: bool,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_editor(cx, |vim, editor, cx| {
            let text_layout_details = editor.text_layout_details(window);
            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let mut original_positions: HashMap<_, _> = Default::default();
                let mut kind = None;
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.move_with(|map, selection| {
                        let original_position = (selection.head(), selection.goal);
                        kind = motion.expand_selection(
                            map,
                            selection,
                            times,
                            &text_layout_details,
                            forced_motion,
                        );
                        if kind == Some(MotionKind::Exclusive) {
                            original_positions
                                .insert(selection.id, (selection.start, selection.goal));
                        } else {
                            original_positions.insert(selection.id, original_position);
                        }
                    })
                });
                let Some(kind) = kind else { return };
                vim.yank_selections_content(editor, kind, window, cx);
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
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
        times: Option<usize>,
        window: &mut Window,
        cx: &mut Context<Self>,
    ) {
        self.update_editor(cx, |vim, editor, cx| {
            editor.transact(window, cx, |editor, window, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let mut start_positions: HashMap<_, _> = Default::default();
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.move_with(|map, selection| {
                        object.expand_selection(map, selection, around, times);
                        let start_position = (selection.start, selection.goal);
                        start_positions.insert(selection.id, start_position);
                    });
                });
                vim.yank_selections_content(editor, MotionKind::Exclusive, window, cx);
                editor.change_selections(SelectionEffects::no_scroll(), window, cx, |s| {
                    s.move_with(|_, selection| {
                        let (head, goal) = start_positions.remove(&selection.id).unwrap();
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
        kind: MotionKind,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        self.copy_ranges(
            editor,
            kind,
            true,
            editor
                .selections
                .all_adjusted(&editor.display_snapshot(cx))
                .iter()
                .map(|s| s.range())
                .collect(),
            window,
            cx,
        )
    }

    pub fn copy_selections_content(
        &mut self,
        editor: &mut Editor,
        kind: MotionKind,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        self.copy_ranges(
            editor,
            kind,
            false,
            editor
                .selections
                .all_adjusted(&editor.display_snapshot(cx))
                .iter()
                .map(|s| s.range())
                .collect(),
            window,
            cx,
        )
    }

    pub(crate) fn copy_ranges(
        &mut self,
        editor: &mut Editor,
        kind: MotionKind,
        is_yank: bool,
        selections: Vec<Range<Point>>,
        window: &mut Window,
        cx: &mut Context<Editor>,
    ) {
        let buffer = editor.buffer().read(cx).snapshot(cx);
        self.set_mark(
            "[".to_string(),
            selections
                .iter()
                .map(|s| buffer.anchor_before(s.start))
                .collect(),
            editor.buffer(),
            window,
            cx,
        );
        self.set_mark(
            "]".to_string(),
            selections
                .iter()
                .map(|s| buffer.anchor_after(s.end))
                .collect(),
            editor.buffer(),
            window,
            cx,
        );

        let mut text = String::new();
        let mut clipboard_selections = Vec::with_capacity(selections.len());
        let mut ranges_to_highlight = Vec::new();

        {
            let mut is_first = true;
            for selection in selections.iter() {
                let start = selection.start;
                let end = selection.end;
                if is_first {
                    is_first = false;
                } else {
                    text.push('\n');
                }
                let initial_len = text.len();

                let start_anchor = buffer.anchor_after(start);
                let end_anchor = buffer.anchor_before(end);
                ranges_to_highlight.push(start_anchor..end_anchor);

                for chunk in buffer.text_for_range(start..end) {
                    text.push_str(chunk);
                }
                if kind.linewise() {
                    text.push('\n');
                }
                clipboard_selections.push(ClipboardSelection {
                    len: text.len() - initial_len,
                    is_entire_line: false,
                    first_line_indent: buffer.indent_size_for_line(MultiBufferRow(start.row)).len,
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
                kind,
                cx,
            )
        });

        let highlight_duration = VimSettings::get_global(cx).highlight_on_yank_duration;
        if !is_yank || self.mode == Mode::Visual || highlight_duration == 0 {
            return;
        }

        editor.highlight_background::<HighlightOnYank>(
            &ranges_to_highlight,
            |colors| colors.colors().editor_document_highlight_read_background,
            cx,
        );
        cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(highlight_duration))
                .await;
            this.update(cx, |editor, cx| {
                editor.clear_background_highlights::<HighlightOnYank>(cx)
            })
            .ok();
        })
        .detach();
    }
}
