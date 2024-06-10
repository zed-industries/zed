use std::{ops::Range, sync::Arc};

use editor::{
    actions::Paste,
    display_map::{DisplayRow, DisplaySnapshot, ToDisplayPoint},
    movement::{self, FindRange},
    scroll::Autoscroll,
    Anchor, Bias, DisplayPoint, ToPoint,
};
use gpui::{actions, impl_actions};
use language::{char_kind, AutoindentMode, CharKind, Selection, SelectionGoal};
use log::error;
use settings::Settings;
use ui::{ViewContext, WindowContext};
use workspace::Workspace;

use crate::{
    motion::{
        next_line_end, next_subword_end, next_subword_start, next_word_end, next_word_start,
        previous_subword_end, previous_subword_start, previous_word_end, previous_word_start,
        Motion,
    },
    normal::{normal_motion, yank::copy_selections_content},
    state::{Mode, Operator, Register},
    visual::visual_motion,
    HelixModeSetting, Vim,
};

actions!(
    helix,
    [
        Delete,
        Change,
        InsertBefore,
        InsertAfter,
        ExtendLineBelow,
        Yank,
        Paste,
    ]
);

pub fn register(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
    workspace.register_action(|_: &mut Workspace, _: &Delete, cx: _| delete(cx));
    workspace.register_action(|_: &mut Workspace, _: &Change, cx: _| change(cx));
    workspace.register_action(|_: &mut Workspace, _: &InsertBefore, cx: _| insert(cx, false));
    workspace.register_action(|_: &mut Workspace, _: &InsertAfter, cx: _| insert(cx, true));
    workspace.register_action(|_: &mut Workspace, _: &ExtendLineBelow, cx: _| extend_line(cx));
    workspace.register_action(|_: &mut Workspace, _: &Yank, cx: _| yank(cx));
    workspace.register_action(|_: &mut Workspace, _: &Paste, cx: _| paste(cx));
}

pub fn helix_normal_motion(motion: Motion, maybe_times: Option<usize>, cx: &mut WindowContext) {
    let times = maybe_times.unwrap_or(1);
    match motion {
        Motion::Up { .. }
        | Motion::Down { .. }
        | Motion::Right
        | Motion::Left
        | Motion::GoToColumn
        | Motion::FirstNonWhitespace { .. }
        | Motion::StartOfDocument
        | Motion::EndOfDocument
        | Motion::Jump { .. }
        | Motion::WindowTop
        | Motion::WindowBottom
        | Motion::WindowMiddle
        | Motion::EndOfLine { .. }
        | Motion::StartOfLine { .. }
        | Motion::FirstNonWhitespace { .. } => {
            simple_motion(motion, maybe_times, cx);
        }
        Motion::NextWordStart { .. }
        | Motion::NextWordEnd { .. }
        | Motion::NextSubwordStart { .. }
        | Motion::NextSubwordEnd { .. } => {
            next_word(motion, times, cx);
        }
        Motion::PreviousWordStart { .. }
        | Motion::PreviousWordEnd { .. }
        | Motion::PreviousSubwordStart { .. }
        | Motion::PreviousSubwordEnd { .. } => {
            prev_word(motion, times, cx);
        }
        Motion::FindForward { .. }
        | Motion::FindBackward { .. }
        | Motion::RepeatFind { .. }
        | Motion::RepeatFindReversed { .. } => find(motion, times, cx),
        Motion::ZedSearchResult {
            prior_selections,
            new_selections,
        } => {
            select_search_result(cx, prior_selections, new_selections);
        }
        _ => {
            clear_selection(cx);
            visual_motion(motion.to_owned(), None, cx);
        }
    };
}

fn simple_motion(motion: Motion, times: Option<usize>, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            let text_layout_details = editor.text_layout_details(cx);
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    let mut cursor = selection.cursor(map);
                    (cursor, selection.goal) = motion
                        .move_point(map, cursor, selection.goal, times, &text_layout_details)
                        .unwrap_or((cursor, selection.goal));
                    selection.start = cursor;
                    selection.end = cursor.next_char(map).map_or(cursor, |(_, offset)| offset);
                    selection.reversed = false;
                })
            })
        });
    })
}

fn prev_word(motion: Motion, times: usize, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    for _ in 0..times {
                        prev_word_selection_update(map, selection, 1, motion.clone());
                    }
                })
            })
        });
    })
}

fn next_word(motion: Motion, times: usize, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    for _ in 0..times {
                        next_word_selection_update(map, selection, times, motion.clone());
                    }
                })
            })
        });
    })
}

trait NextPrevCharExt {
    fn prev_char(&self, map: &DisplaySnapshot) -> Option<(char, DisplayPoint)>;
    fn next_char(&self, map: &DisplaySnapshot) -> Option<(char, DisplayPoint)>;
}

impl NextPrevCharExt for DisplayPoint {
    fn prev_char(&self, map: &DisplaySnapshot) -> Option<(char, DisplayPoint)> {
        let mut prev = *self;
        map.reverse_buffer_chars_at(self.to_offset(map, editor::Bias::Right))
            .next()
            .map(|(c, offset)| (c, offset.to_display_point(map)))
    }
    fn next_char(&self, map: &DisplaySnapshot) -> Option<(char, DisplayPoint)> {
        map.display_chars_at(*self)
            .nth(1)
            .map(|(c, offset)| (c, offset))
    }
}

trait CursorSelectionExt {
    fn cursor(&self, map: &DisplaySnapshot) -> DisplayPoint;
}

impl CursorSelectionExt for Selection<DisplayPoint> {
    fn cursor(&self, map: &DisplaySnapshot) -> DisplayPoint {
        if self.end == self.start || self.reversed {
            self.start
        } else {
            self.end
                .prev_char(map)
                .map_or(self.start, |(_, offset)| offset)
        }
    }
}

fn next_word_selection_update(
    map: &DisplaySnapshot,
    selection: &mut Selection<DisplayPoint>,
    times: usize,
    motion: Motion,
) {
    let (a, b, c, d);
    let next_word_fn: &dyn Fn(DisplayPoint) -> DisplayPoint = match &motion {
        Motion::NextWordStart { ignore_punctuation } => {
            a = |point| next_word_start(map, point, *ignore_punctuation, 1);
            &a
        }
        Motion::NextWordEnd { ignore_punctuation } => {
            b = |point| next_word_end(map, point, *ignore_punctuation, 1, true);
            &b
        }
        Motion::NextSubwordStart { ignore_punctuation } => {
            c = |point| next_subword_start(map, point, *ignore_punctuation, 1);
            &c
        }
        Motion::NextSubwordEnd { ignore_punctuation } => {
            d = |point| next_subword_end(map, point, *ignore_punctuation, 1, true);
            &d
        }
        _ => unreachable!(),
    };
    let cursor = selection.cursor(map);
    let mut anchor = selection.cursor(map);
    let mut head = selection.head();
    let mut skipped = false;
    // skip new lines directly after cursor
    {
        let mut curr = cursor;
        loop {
            match curr.next_char(map) {
                Some((val, next)) if val == '\n' => {
                    skipped = true;
                    curr = next;
                }
                Some((_, next)) => {
                    head = next;
                    break;
                }
                None => {
                    break;
                }
            }
        }
    }
    if skipped {
        anchor = head;
    }

    let mut next = next_word_fn(anchor);
    if next == head {
        anchor = next;
        next = next_word_fn(next);
    }
    selection.start = anchor;
    selection.reversed = false;
    selection.end = next;
}

fn prev_word_selection_update(
    map: &DisplaySnapshot,
    selection: &mut Selection<DisplayPoint>,
    times: usize,
    motion: Motion,
) {
    let (a, b, c, d);
    let prev_word_fn: &dyn Fn(DisplayPoint) -> DisplayPoint = match &motion {
        Motion::PreviousWordStart { ignore_punctuation } => {
            a = |point| previous_word_start(map, point, *ignore_punctuation, 1);
            &a
        }
        Motion::PreviousWordEnd { ignore_punctuation } => {
            b = |point| previous_word_end(map, point, *ignore_punctuation, 1);
            &b
        }
        Motion::PreviousSubwordStart { ignore_punctuation } => {
            c = |point| previous_subword_start(map, point, *ignore_punctuation, 1);
            &c
        }
        Motion::PreviousSubwordEnd { ignore_punctuation } => {
            d = |point| previous_subword_end(map, point, *ignore_punctuation, 1);
            &d
        }
        _ => unreachable!(),
    };
    let cursor = selection.cursor(map);
    let mut anchor = cursor.next_char(map).map_or(cursor, |(_, next)| next);
    let mut head = cursor;
    let mut skipped = false;
    // skip new lines directly before cursor
    {
        let mut curr = cursor;
        loop {
            match curr.prev_char(map) {
                Some((val, prev)) if val == '\n' => {
                    skipped = true;
                    head = prev;
                    curr = prev;
                }
                Some((val, prev)) => {
                    break;
                }
                None => {
                    break;
                }
            }
        }
    }
    if skipped {
        anchor = head;
    }

    let mut prev = prev_word_fn(anchor);
    if prev == head {
        anchor = prev;
        prev = prev_word_fn(prev);
    }
    selection.reversed = true;
    selection.end = anchor;
    selection.start = prev;
}

fn clear_selection(cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|_, selection| {
                    let point = selection.head();
                    selection.collapse_to(point, selection.goal);
                });
            });
        });
    });
}

fn select_search_result(
    cx: &mut WindowContext,
    prior_selections: Vec<Range<Anchor>>,
    new_selections: Vec<Range<Anchor>>,
) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.select_anchors(
                    new_selections
                        .into_iter()
                        .enumerate()
                        .map(|(id, r)| Selection {
                            id,
                            start: r.start,
                            end: r.end,
                            reversed: false,
                            goal: SelectionGoal::None,
                        })
                        .collect(),
                );
            });
        });
    });
}

fn find(motion: Motion, times: usize, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    let cursor = selection.cursor(map);
                    let pos = match motion {
                        Motion::FindForward {
                            before,
                            char,
                            mode,
                            smartcase,
                        } => {
                            let pos = crate::motion::find_forward(
                                map,
                                cursor,
                                before,
                                char,
                                times,
                                FindRange::SingleLine,
                                smartcase,
                            );
                            if let Some(pos) = pos {
                                selection.start = cursor;
                                selection.end =
                                    pos.next_char(map).map_or(pos, |(_, offset)| offset);
                                selection.reversed = false;
                            }
                        }
                        Motion::FindBackward {
                            after,
                            char,
                            mode,
                            smartcase,
                        } => {
                            let pos = crate::motion::find_backward(
                                map,
                                cursor,
                                after,
                                char,
                                times,
                                FindRange::SingleLine,
                                smartcase,
                            );
                            if pos != cursor {
                                selection.start = pos;
                                selection.end =
                                    cursor.next_char(map).map_or(cursor, |(_, offset)| offset);
                                selection.reversed = true;
                            }
                        }
                        _ => unreachable!(),
                    };
                })
            });
        })
    });
}

pub(crate) fn hx_replace(text: Arc<str>, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.stop_recording();
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.transact(cx, |editor, cx| {
                let (display_map, selections) = editor.selections.all_adjusted_display(cx);

                // Selections are biased right at the start. So we need to store
                // anchors that are biased left so that we can restore the selections
                // after the change
                let stable_anchors = editor
                    .selections
                    .disjoint_anchors()
                    .into_iter()
                    .map(|selection| {
                        let start = selection.start.bias_left(&display_map.buffer_snapshot);
                        let end = selection.end.bias_left(&display_map.buffer_snapshot);
                        start..end
                    })
                    .collect::<Vec<_>>();

                let mut edits = Vec::new();
                for selection in selections.iter() {
                    let selection = selection.clone();
                    for row_range in
                        movement::split_display_range_by_lines(&display_map, selection.range())
                    {
                        let range = row_range.start.to_offset(&display_map, Bias::Right)
                            ..row_range.end.to_offset(&display_map, Bias::Right);
                        let text = text.repeat(range.len());
                        edits.push((range, text));
                    }
                }

                editor.buffer().update(cx, |buffer, cx| {
                    buffer.edit(edits, None, cx);
                });
                editor.change_selections(None, cx, |s| s.select_ranges(stable_anchors));
            });
        });
        vim.switch_mode(Mode::HelixNormal, false, cx);
    });
}

pub(crate) fn delete(cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.transact(cx, |editor, cx| {
                let (display_map, selections) = editor.selections.all_adjusted_display(cx);

                // Selections are biased right at the start. So we need to store
                // anchors that are biased left so that we can restore the selections
                // after the change
                let stable_anchors = editor
                    .selections
                    .disjoint_anchors()
                    .into_iter()
                    .map(|selection| {
                        let start = selection.start.bias_left(&display_map.buffer_snapshot);
                        let end = selection.end.bias_left(&display_map.buffer_snapshot);
                        start..start
                    })
                    .collect::<Vec<_>>();

                let mut edits = Vec::new();
                for selection in selections.iter() {
                    let selection = selection.clone();
                    let range = selection.start.to_offset(&display_map, Bias::Right)
                        ..selection.end.to_offset(&display_map, Bias::Right);
                    edits.push((range, String::new()));
                }

                editor.buffer().update(cx, |buffer, cx| {
                    buffer.edit(edits, None, cx);
                });
                editor.change_selections(None, cx, |s| s.select_ranges(stable_anchors));
            });
        });
        vim.switch_mode(Mode::HelixNormal, false, cx);
    });
}
pub(crate) fn change(cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.transact(cx, |editor, cx| {
                let (display_map, selections) = editor.selections.all_adjusted_display(cx);

                // Selections are biased right at the start. So we need to store
                // anchors that are biased left so that we can restore the selections
                // after the change
                let stable_anchors = editor
                    .selections
                    .disjoint_anchors()
                    .into_iter()
                    .map(|selection| {
                        let start = selection.start.bias_left(&display_map.buffer_snapshot);
                        let end = selection.end.bias_left(&display_map.buffer_snapshot);
                        start..start
                    })
                    .collect::<Vec<_>>();

                let mut edits = Vec::new();
                for selection in selections.iter() {
                    let selection = selection.clone();
                    let mut range = selection.start.to_offset(&display_map, Bias::Right)
                        ..selection.tail().to_offset(&display_map, Bias::Right);
                    // probably incorrect
                    let line_wise = selection.start.column() == 0 && selection.start.column() == 0;
                    if line_wise {
                        range.end = movement::left(&display_map, selection.end)
                            .to_offset(&display_map, Bias::Right);
                    }

                    edits.push((range, String::new()));
                }

                editor.buffer().update(cx, |buffer, cx| {
                    buffer.edit(edits, Some(AutoindentMode::EachLine), cx);
                });
                editor.change_selections(None, cx, |s| s.select_ranges(stable_anchors));
            });
        });
        vim.switch_mode(Mode::Insert, false, cx);
    });
}

pub(crate) fn insert(cx: &mut WindowContext, after: bool) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.move_with(|_, selection| {
                    selection.reversed = !after;
                });
            })
        });
        let selections = vim.editor_selections(cx);
        vim.update_state(|state| state.hx_return_selection = Some(selections));
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.move_with(|map, selection| {
                    selection.collapse_to(
                        if after {
                            selection.end
                        } else {
                            selection.start
                        },
                        SelectionGoal::None,
                    );
                })
            });
        });
        vim.switch_mode(Mode::Insert, false, cx);
    });
}

pub(crate) fn extend_line(cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(None, cx, |s| {
                s.move_with(|map, selection| {
                    // TODO: times
                    let start_row = selection.start.row();
                    let end_row = if selection.end.column() == 0 {
                        selection.end.row()
                    } else {
                        selection.end.row() + DisplayRow(1)
                    };
                    selection.start = DisplayPoint::new(start_row, 0);
                    selection.end = DisplayPoint::new(end_row, 0);
                });
            });
        });
    });
}

pub fn yank(cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |vim, editor, cx| {
            copy_selections_content(vim, editor, false, cx);
        });
    })
}

pub fn paste(cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |vim, editor, cx| {
            let selected_register = vim.update_state(|state| state.selected_register.take());
            let Some(Register {
                text,
                clipboard_selections,
            }) = vim
                .read_register(selected_register, Some(editor), cx)
                .filter(|reg| !reg.text.is_empty())
            else {
                return;
            };
            dbg!
        });
    })
}
