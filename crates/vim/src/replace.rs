use std::sync::Arc;

use crate::{
    motion::Motion,
    state::{Mode, Operator},
    Vim,
};
use editor::{movement, Bias};
use gpui::{actions, ViewContext, WindowContext};
use log::error;
use workspace::Workspace;

actions!(vim, [ToggleReplace]);

pub fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(|_, _: &ToggleReplace, cx: &mut ViewContext<Workspace>| {
        Vim::update(cx, |vim, cx| {
            if vim.state().mode == Mode::Replace {
                vim.switch_mode(Mode::Normal, false, cx);
            } else {
                vim.switch_mode(Mode::Replace, false, cx);
                vim.update_active_editor(cx, |_, editor, cx| {
                    editor.set_last_snapshot(Some(editor.buffer().clone().read(cx).snapshot(cx)));
                });
            }
        });
    });
}

pub fn replace_motion(
    motion: Motion,
    operator: Option<Operator>,
    times: Option<usize>,
    cx: &mut WindowContext,
) {
    Vim::update(cx, |vim, cx| {
        match operator {
            None => undo_replace_motion(vim, motion, times, cx),
            Some(operator) => {
                // Can't do anything for text objects, Ignoring
                error!("Unexpected replace mode motion operator: {:?}", operator)
            }
        }
    });
}

pub(crate) fn multi_replace(text: Arc<str>, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.stop_recording();
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.transact(cx, |editor, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let (map, display_selections) = editor.selections.all_display(cx);
                let stable_anchors = editor
                    .selections
                    .disjoint_anchors()
                    .into_iter()
                    .map(|selection| {
                        let start = selection.start.bias_right(&map.buffer_snapshot);
                        start..start
                    })
                    .collect::<Vec<_>>();

                let edits = display_selections
                    .into_iter()
                    .map(|selection| {
                        let mut range = selection.range();
                        // "\n" need to be handled separately, because when a "\n" is typing,
                        // we don't do a replace, we need insert a "\n"
                        if text.as_ref() != "\n" {
                            *range.end.column_mut() += 1;
                            range.end = map.clip_point(range.end, Bias::Right);
                        }
                        (
                            range.start.to_offset(&map, Bias::Left)
                                ..range.end.to_offset(&map, Bias::Left),
                            text.clone(),
                        )
                    })
                    .collect::<Vec<_>>();

                editor.buffer().update(cx, |buffer, cx| {
                    buffer.edit(edits, None, cx);
                });
                editor.set_clip_at_line_ends(true, cx);
                editor.change_selections(None, cx, |s| {
                    s.select_anchor_ranges(stable_anchors);
                });
            });
        });
    });
}

fn undo_replace_motion(vim: &mut Vim, _: Motion, _: Option<usize>, cx: &mut WindowContext) {
    vim.stop_recording();
    vim.update_active_editor(cx, |_, editor, cx| {
        if let Some(original_snapshot) = editor.last_snapshot.clone() {
            editor.transact(cx, |editor, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let (map, display_selections) = editor.selections.all_display(cx);
                let stable_anchors = display_selections
                    .into_iter()
                    .map(|selection| {
                        let is_multi_cursor = editor.selections.count() > 1;
                        let range = if selection.start.column() == 0
                            && (is_multi_cursor || selection.start.row() == 0)
                        {
                            selection.range()
                        } else {
                            movement::left(&map, selection.start)
                                ..movement::left(&map, selection.start)
                        };
                        range
                    })
                    .collect::<Vec<_>>();

                let (map, display_selections) = editor.selections.all_display(cx);

                let edits = display_selections
                    .into_iter()
                    .map(|selection| {
                        let is_multi_cursor = editor.selections.count() > 1;
                        let range = if selection.start.column() == 0
                            && (is_multi_cursor || selection.start.row() == 0)
                        {
                            selection.start..movement::right(&map, selection.start)
                        } else {
                            movement::left(&map, selection.start)..selection.start
                        };
                        let recover_text = if range.start.row()
                            >= original_snapshot.max_buffer_row()
                            || range.start.column() >= original_snapshot.line_len(range.start.row())
                        {
                            "".to_string()
                        } else {
                            original_snapshot
                                .chars_at(range.start.to_offset(&map, Bias::Left))
                                .next()
                                .map(|item| item.to_string())
                                .unwrap_or("".to_string())
                        };
                        let current_text = editor
                            .buffer()
                            .read(cx)
                            .snapshot(cx)
                            .chars_at(range.start.to_offset(&map, Bias::Left))
                            .next()
                            .map(|item| item.to_string())
                            .unwrap_or("".to_string());
                        let mut replace_text = recover_text.to_string();
                        let target_range = range.start.to_offset(&map, Bias::Left)
                            ..range.end.to_offset(&map, Bias::Left);
                        // "\n" need to be handled se
                        //
                        // parately, because when a backspace is triggered
                        // and the previous character is "\n", we don't do a replace, we need delete a "\n"
                        if current_text == "\n" && current_text != recover_text {
                            replace_text = "".to_string();
                        }
                        (target_range, replace_text)
                    })
                    .collect::<Vec<_>>();
                editor.buffer().update(cx, |buffer, cx| {
                    buffer.edit(edits, None, cx);
                });

                editor.set_clip_at_line_ends(true, cx);
                editor.change_selections(None, cx, |s| {
                    s.select_display_ranges(stable_anchors);
                });
            });
        };
    });
}
