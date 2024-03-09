use crate::{
    motion::Motion,
    state::{Mode, Operator},
    Vim,
};
use collections::HashMap;
use editor::{display_map::ToDisplayPoint, movement, Bias};
use gpui::{actions, ViewContext, WindowContext};
use language::AutoindentMode;
use log::error;
use std::ops::Range;
use std::sync::Arc;
use workspace::Workspace;

actions!(vim, [ToggleReplace]);

pub fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(|_, _: &ToggleReplace, cx: &mut ViewContext<Workspace>| {
        Vim::update(cx, |vim, cx| {
            if vim.state().mode == Mode::Replace {
                vim.stop_recording();
                vim.switch_mode(Mode::Normal, false, cx);
            } else {
                vim.switch_mode(Mode::Replace, false, cx);
                vim.update_active_editor(cx, |_, editor, cx| {
                    editor.set_last_snapshot(Some(editor.buffer().clone().read(cx).snapshot(cx)));
                    editor.vim_replace_map = Default::default();
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
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.transact(cx, |editor, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let (map, display_selections) = editor.selections.all_display(cx);
                // Handles all string that require manipulation, including inserts and replaces
                let edits = display_selections
                    .into_iter()
                    .map(|selection| {
                        let is_new_line = text.as_ref() == "\n";
                        let mut range = selection.range();
                        // "\n" need to be handled separately, because when a "\n" is typing,
                        // we don't do a replace, we need insert a "\n"
                        if !is_new_line {
                            *range.end.column_mut() += 1;
                            range.end = map.clip_point(range.end, Bias::Right);
                        }
                        let replace_range = range.start.to_offset(&map, Bias::Left)
                            ..range.end.to_offset(&map, Bias::Right);
                        let snapshot = editor.buffer().read(cx).snapshot(cx);
                        let current_text = if is_new_line
                            || snapshot.line_len(range.start.row()) <= range.start.column()
                        {
                            // Handle insertion of newlines and end-of-line insertions
                            "".to_string()
                        } else {
                            editor
                                .buffer()
                                .read(cx)
                                .snapshot(cx)
                                .chars_at(range.start.to_offset(&map, Bias::Left))
                                .next()
                                .map(|item| item.to_string())
                                .unwrap_or("".to_string())
                        };
                        if !editor.vim_replace_map.contains_key(&replace_range) {
                            editor
                                .vim_replace_map
                                .insert(replace_range.clone(), current_text);
                        }
                        (replace_range, text.clone())
                    })
                    .collect::<Vec<_>>();

                let stable_anchors = editor
                    .selections
                    .disjoint_anchors()
                    .into_iter()
                    // .rev()
                    .map(|selection| {
                        let start = selection.start.bias_right(&map.buffer_snapshot);
                        start..start
                    })
                    .collect::<Vec<_>>();

                // If the operation is insert, the hierarchy of the file will be changed,
                // and the dictionary of the save location will need to be updated synchronously
                for edit in edits.iter().rev() {
                    let (replace_range, _) = edit;
                    if replace_range.start == replace_range.end {
                        let vim_replace_map: HashMap<Range<usize>, String> = editor
                            .vim_replace_map
                            .iter()
                            .map(|(range, content)| {
                                if range == replace_range {
                                    (range.start..range.end + 1, content.clone())
                                } else if range.start >= replace_range.start {
                                    (range.start + 1..range.end + 1, content.clone())
                                } else {
                                    (range.clone(), content.clone())
                                }
                            })
                            .collect();
                        editor.vim_replace_map = vim_replace_map;
                    }
                }

                // There is currently an issue with the automatic indent, pending resolution
                editor.buffer().update(cx, |buffer, cx| {
                    buffer.edit(
                        edits,
                        Some(AutoindentMode::Block {
                            original_indent_columns: Vec::new(),
                        }),
                        cx,
                    );
                });

                editor.change_selections(None, cx, |s| {
                    s.select_anchor_ranges(stable_anchors);
                });
                editor.set_clip_at_line_ends(true, cx);
            });
        });
    });
}

fn undo_replace_motion(vim: &mut Vim, _: Motion, _: Option<usize>, cx: &mut WindowContext) {
    vim.update_active_editor(cx, |_, editor, cx| {
        editor.transact(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let (map, display_selections) = editor.selections.all_display(cx);
            let edits = display_selections
                .into_iter()
                .map(|selection| {
                    // Look for the previous cursor position that is qualified
                    let mut range = selection.range();
                    if range.start.column() > 0 {
                        *range.start.column_mut() -= 1;
                    } else if range.start.row() > 0 {
                        *range.start.row_mut() -= 1;
                        *range.start.column_mut() = snapshot.line_len(range.start.row()) + 1;
                    } else {
                        *range.end.column_mut() += 1;
                    }
                    range.start = map.clip_point(range.start, Bias::Left);
                    range.end = map.clip_point(range.end, Bias::Left);
                    let cur_range = range.start.to_offset(&map, Bias::Left)
                        ..range.end.to_offset(&map, Bias::Left);

                    let mut replace_text = editor
                        .buffer()
                        .read(cx)
                        .snapshot(cx)
                        .chars_at(range.start.to_offset(&map, Bias::Left))
                        .next()
                        .map(|item| item.to_string())
                        .unwrap_or("".to_string());

                    if let Some(last) = editor.vim_replace_map.get(&cur_range) {
                        replace_text = last.to_string();
                        editor.vim_replace_map.remove(&cur_range);
                    }
                    (cur_range, replace_text)
                })
                .collect::<Vec<_>>();

            // If cureent operation is a delete, the editor structure will be changed and it will need to be synchronously
            let reverse_edits: Vec<(Range<usize>, String)> = edits.iter().rev().cloned().collect();
            for edit in reverse_edits.iter() {
                let (replace_range, content) = &edit;
                if content == "" {
                    let vim_replace_map: HashMap<Range<usize>, String> = editor
                        .vim_replace_map
                        .iter()
                        .map(|(range, content)| {
                            if range.start > replace_range.start {
                                (range.start - 1..range.end - 1, content.clone())
                            } else {
                                (range.clone(), content.clone())
                            }
                        })
                        .collect();
                    editor.vim_replace_map = vim_replace_map;
                }
            }

            // This needs to be backwards and forwards,
            // as there is a possibility that a deletion operation will change the current editor structure
            editor.buffer().update(cx, |buffer, cx| {
                for edit in reverse_edits {
                    buffer.edit([edit.clone()], None, cx);
                }
            });

            // If cureent operation is a delete, the editor structure will be changed and it will need to be synchronously
            let display_map = editor.display_map.update(cx, |map, cx| map.snapshot(cx));
            let mut delete_count = 0;
            let stable_anchors = edits
                .iter()
                .map(|(range, content)| {
                    let target_point = (range.start - delete_count).to_display_point(&display_map);
                    if content == "" {
                        delete_count += 1;
                    }
                    target_point..target_point
                })
                .collect::<Vec<_>>();

            editor.change_selections(None, cx, |s| {
                s.select_display_ranges(stable_anchors);
            });
            editor.set_clip_at_line_ends(true, cx);
        });
    });
}
