use editor::{
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement::{self, FindRange},
    scroll::Autoscroll,
    DisplayPoint, ToPoint,
};
use gpui::actions;
use language::{char_kind, CharKind, Selection};
use log::error;
use settings::Settings;
use ui::{ViewContext, WindowContext};
use workspace::Workspace;

use crate::{
    motion::{
        next_subword_end, next_subword_start, next_word_end, next_word_start, previous_subword_end,
        previous_subword_start, previous_word_end, previous_word_start, Motion,
    },
    normal::normal_motion,
    utils::coerce_punctuation,
    visual::visual_motion,
    HelixModeSetting, Vim,
};

actions!(helix, [SelectNextLine,]);

pub fn register(workspace: &mut Workspace, cx: &mut ViewContext<Workspace>) {
    workspace.register_action(|_: &mut Workspace, _: &SelectNextLine, cx: _| select_next_line(cx));
}

fn select_next_line(cx: &mut WindowContext) {}

pub fn helix_normal_motion(motion: Motion, times: Option<usize>, cx: &mut WindowContext) {
    let times = times.unwrap_or(1);
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
            simple_motion(motion, times, cx);
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
        _ => {
            clear_selection(cx);
            visual_motion(motion.to_owned(), None, cx);
        }
    };
}

fn simple_motion(motion: Motion, times: usize, cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            let text_layout_details = editor.text_layout_details(cx);
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    let mut cursor = selection.cursor(map);
                    (cursor, selection.goal) = motion
                        .move_point(
                            map,
                            cursor,
                            selection.goal,
                            Some(times),
                            &text_layout_details,
                        )
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
        if !self.reversed {
            self.end
                .prev_char(map)
                .map_or(self.start, |(_, offset)| offset)
        } else {
            self.start
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

fn select_current(cx: &mut WindowContext) {
    Vim::update(cx, |vim, cx| {
        vim.update_active_editor(cx, |_, editor, cx| {
            editor.change_selections(Some(Autoscroll::fit()), cx, |s| {
                s.move_with(|map, selection| {
                    let cursor = selection.cursor(map);
                    selection.start = cursor;
                    selection.end = cursor.next_char(map).map_or(cursor, |(_, next)| next);
                    selection.reversed = false;
                });
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
