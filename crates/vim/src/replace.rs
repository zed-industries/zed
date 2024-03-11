use crate::{
    motion::{self},
    state::Mode,
    Vim,
};
use editor::{display_map::ToDisplayPoint, Bias, ToPoint};
use gpui::{actions, ViewContext, WindowContext};
use language::{AutoindentMode, Point};
use std::ops::Range;
use std::sync::Arc;
use workspace::Workspace;

actions!(vim, [ToggleReplace, UndoReplace]);

pub fn register(workspace: &mut Workspace, _: &mut ViewContext<Workspace>) {
    workspace.register_action(|_, _: &ToggleReplace, cx: &mut ViewContext<Workspace>| {
        Vim::update(cx, |vim, cx| {
            vim.update_state(|state| state.replacements = vec![]);
            vim.switch_mode(Mode::Replace, false, cx);
        });
    });

    workspace.register_action(|_, _: &UndoReplace, cx: &mut ViewContext<Workspace>| {
        Vim::update(cx, |vim, cx| {
            if vim.state().mode != Mode::Replace {
                return;
            }
            let count = vim.take_count(cx);
            undo_replace(vim, count, cx)
        });
    });
}

pub(crate) fn multi_replace(text: Arc<str>, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |vim, editor, cx| {
            editor.transact(cx, |editor, cx| {
                editor.set_clip_at_line_ends(false, cx);
                let map = editor.snapshot(cx);
                let display_selections = editor.selections.all::<Point>(cx);

                // Handles all string that require manipulation, including inserts and replaces
                let edits = display_selections
                    .into_iter()
                    .map(|selection| {
                        let is_new_line = text.as_ref() == "\n";
                        let mut range = selection.range();
                        // "\n" need to be handled separately, because when a "\n" is typing,
                        // we don't do a replace, we need insert a "\n"
                        if !is_new_line {
                            range.end.column += 1;
                            range.end = map.buffer_snapshot.clip_point(range.end, Bias::Right);
                        }
                        let replace_range = map.buffer_snapshot.anchor_before(range.start)
                            ..map.buffer_snapshot.anchor_after(range.end);
                        let current_text = map
                            .buffer_snapshot
                            .text_for_range(replace_range.clone())
                            .collect();
                        vim.update_state(|state| {
                            state
                                .replacements
                                .push((replace_range.clone(), current_text))
                        });
                        (replace_range, text.clone())
                    })
                    .collect::<Vec<_>>();

                // There is currently an issue with the automatic indent, pending resolution
                editor.buffer().update(cx, |buffer, cx| {
                    buffer.edit(
                        edits.clone(),
                        Some(AutoindentMode::Block {
                            original_indent_columns: Vec::new(),
                        }),
                        cx,
                    );
                });

                editor.change_selections(None, cx, |s| {
                    s.select_anchor_ranges(edits.iter().map(|(range, _)| range.end..range.end));
                });
                editor.set_clip_at_line_ends(true, cx);
            });
        });
    });
}

fn undo_replace(vim: &mut Vim, maybe_times: Option<usize>, cx: &mut WindowContext) {
    vim.update_active_editor(cx, |vim, editor, cx| {
        editor.transact(cx, |editor, cx| {
            editor.set_clip_at_line_ends(false, cx);
            let map = editor.snapshot(cx);
            let selections = editor.selections.all::<Point>(cx);
            let mut new_selections = vec![];
            let edits: Vec<(Range<Point>, String)> = selections
                .into_iter()
                .filter_map(|selection| {
                    let end = selection.head();
                    let start = motion::backspace(
                        &map,
                        end.to_display_point(&map),
                        maybe_times.unwrap_or(1),
                    )
                    .to_point(&map);
                    new_selections.push(
                        map.buffer_snapshot.anchor_before(start)
                            ..map.buffer_snapshot.anchor_before(start),
                    );

                    let mut undo = None;
                    let edit_range = start..end;
                    for (range, inverse) in vim.state().replacements.iter().rev() {
                        if range.start.to_point(&map.buffer_snapshot) <= edit_range.start
                            && range.end.to_point(&map.buffer_snapshot) >= edit_range.end
                        {
                            undo = Some(inverse);
                            break;
                        }
                    }
                    Some((edit_range, undo?.clone()))
                })
                .collect::<Vec<_>>();

            editor.buffer().update(cx, |buffer, cx| {
                buffer.edit(edits, None, cx);
            });

            editor.change_selections(None, cx, |s| {
                s.select_ranges(new_selections);
            });
            editor.set_clip_at_line_ends(true, cx);
        });
    });
}
