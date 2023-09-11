use std::cmp;

use editor::{
    char_kind,
    display_map::{DisplaySnapshot, FoldPoint, ToDisplayPoint},
    movement::{self, find_boundary, find_preceding_boundary, FindRange},
    Bias, CharKind, DisplayPoint, ToOffset,
};
use gpui::{actions, impl_actions, AppContext, WindowContext};
use language::{Point, Selection, SelectionGoal};
use serde::Deserialize;
use workspace::Workspace;

use crate::{
    normal::normal_motion,
    state::{Mode, Operator},
    visual::visual_motion,
    Vim,
};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Motion {
    Left,
    Backspace,
    Down { display_lines: bool },
    Up { display_lines: bool },
    Right,
    NextWordStart { ignore_punctuation: bool },
    NextWordEnd { ignore_punctuation: bool },
    PreviousWordStart { ignore_punctuation: bool },
    FirstNonWhitespace { display_lines: bool },
    CurrentLine,
    StartOfLine { display_lines: bool },
    EndOfLine { display_lines: bool },
    StartOfParagraph,
    EndOfParagraph,
    StartOfDocument,
    EndOfDocument,
    Matching,
    FindForward { before: bool, char: char },
    FindBackward { after: bool, char: char },
    NextLineStart,
    StartOfLineDownward,
    EndOfLineDownward,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct NextWordStart {
    #[serde(default)]
    ignore_punctuation: bool,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct NextWordEnd {
    #[serde(default)]
    ignore_punctuation: bool,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct PreviousWordStart {
    #[serde(default)]
    ignore_punctuation: bool,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub(crate) struct Up {
    #[serde(default)]
    pub(crate) display_lines: bool,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct Down {
    #[serde(default)]
    display_lines: bool,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct FirstNonWhitespace {
    #[serde(default)]
    display_lines: bool,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
struct EndOfLine {
    #[serde(default)]
    display_lines: bool,
}

#[derive(Clone, Deserialize, PartialEq)]
#[serde(rename_all = "camelCase")]
pub struct StartOfLine {
    #[serde(default)]
    pub(crate) display_lines: bool,
}

#[derive(Clone, Deserialize, PartialEq)]
struct RepeatFind {
    #[serde(default)]
    backwards: bool,
}

actions!(
    vim,
    [
        Left,
        Backspace,
        Right,
        CurrentLine,
        StartOfParagraph,
        EndOfParagraph,
        StartOfDocument,
        EndOfDocument,
        Matching,
        NextLineStart,
        StartOfLineDownward,
        EndOfLineDownward,
    ]
);
impl_actions!(
    vim,
    [
        NextWordStart,
        NextWordEnd,
        PreviousWordStart,
        RepeatFind,
        Up,
        Down,
        FirstNonWhitespace,
        EndOfLine,
        StartOfLine,
    ]
);

pub fn init(cx: &mut AppContext) {
    cx.add_action(|_: &mut Workspace, _: &Left, cx: _| motion(Motion::Left, cx));
    cx.add_action(|_: &mut Workspace, _: &Backspace, cx: _| motion(Motion::Backspace, cx));
    cx.add_action(|_: &mut Workspace, action: &Down, cx: _| {
        motion(
            Motion::Down {
                display_lines: action.display_lines,
            },
            cx,
        )
    });
    cx.add_action(|_: &mut Workspace, action: &Up, cx: _| {
        motion(
            Motion::Up {
                display_lines: action.display_lines,
            },
            cx,
        )
    });
    cx.add_action(|_: &mut Workspace, _: &Right, cx: _| motion(Motion::Right, cx));
    cx.add_action(|_: &mut Workspace, action: &FirstNonWhitespace, cx: _| {
        motion(
            Motion::FirstNonWhitespace {
                display_lines: action.display_lines,
            },
            cx,
        )
    });
    cx.add_action(|_: &mut Workspace, action: &StartOfLine, cx: _| {
        motion(
            Motion::StartOfLine {
                display_lines: action.display_lines,
            },
            cx,
        )
    });
    cx.add_action(|_: &mut Workspace, action: &EndOfLine, cx: _| {
        motion(
            Motion::EndOfLine {
                display_lines: action.display_lines,
            },
            cx,
        )
    });
    cx.add_action(|_: &mut Workspace, _: &CurrentLine, cx: _| motion(Motion::CurrentLine, cx));
    cx.add_action(|_: &mut Workspace, _: &StartOfParagraph, cx: _| {
        motion(Motion::StartOfParagraph, cx)
    });
    cx.add_action(|_: &mut Workspace, _: &EndOfParagraph, cx: _| {
        motion(Motion::EndOfParagraph, cx)
    });
    cx.add_action(|_: &mut Workspace, _: &StartOfDocument, cx: _| {
        motion(Motion::StartOfDocument, cx)
    });
    cx.add_action(|_: &mut Workspace, _: &EndOfDocument, cx: _| motion(Motion::EndOfDocument, cx));
    cx.add_action(|_: &mut Workspace, _: &Matching, cx: _| motion(Motion::Matching, cx));

    cx.add_action(
        |_: &mut Workspace, &NextWordStart { ignore_punctuation }: &NextWordStart, cx: _| {
            motion(Motion::NextWordStart { ignore_punctuation }, cx)
        },
    );
    cx.add_action(
        |_: &mut Workspace, &NextWordEnd { ignore_punctuation }: &NextWordEnd, cx: _| {
            motion(Motion::NextWordEnd { ignore_punctuation }, cx)
        },
    );
    cx.add_action(
        |_: &mut Workspace,
         &PreviousWordStart { ignore_punctuation }: &PreviousWordStart,
         cx: _| { motion(Motion::PreviousWordStart { ignore_punctuation }, cx) },
    );
    cx.add_action(|_: &mut Workspace, &NextLineStart, cx: _| motion(Motion::NextLineStart, cx));
    cx.add_action(|_: &mut Workspace, &StartOfLineDownward, cx: _| {
        motion(Motion::StartOfLineDownward, cx)
    });
    cx.add_action(|_: &mut Workspace, &EndOfLineDownward, cx: _| {
        motion(Motion::EndOfLineDownward, cx)
    });
    cx.add_action(|_: &mut Workspace, action: &RepeatFind, cx: _| {
        repeat_motion(action.backwards, cx)
    })
}

pub(crate) fn motion(motion: Motion, cx: &mut WindowContext) {
    if let Some(Operator::FindForward { .. }) | Some(Operator::FindBackward { .. }) =
        Vim::read(cx).active_operator()
    {
        Vim::update(cx, |vim, cx| vim.pop_operator(cx));
    }

    let count = Vim::update(cx, |vim, _| vim.take_count());
    let operator = Vim::read(cx).active_operator();
    match Vim::read(cx).state().mode {
        Mode::Normal => normal_motion(motion, operator, count, cx),
        Mode::Visual | Mode::VisualLine | Mode::VisualBlock => visual_motion(motion, count, cx),
        Mode::Insert => {
            // Shouldn't execute a motion in insert mode. Ignoring
        }
    }
    Vim::update(cx, |vim, cx| vim.clear_operator(cx));
}

fn repeat_motion(backwards: bool, cx: &mut WindowContext) {
    let find = match Vim::read(cx).workspace_state.last_find.clone() {
        Some(Motion::FindForward { before, char }) => {
            if backwards {
                Motion::FindBackward {
                    after: before,
                    char,
                }
            } else {
                Motion::FindForward { before, char }
            }
        }

        Some(Motion::FindBackward { after, char }) => {
            if backwards {
                Motion::FindForward {
                    before: after,
                    char,
                }
            } else {
                Motion::FindBackward { after, char }
            }
        }
        _ => return,
    };

    motion(find, cx)
}

// Motion handling is specified here:
// https://github.com/vim/vim/blob/master/runtime/doc/motion.txt
impl Motion {
    pub fn linewise(&self) -> bool {
        use Motion::*;
        match self {
            Down { .. }
            | Up { .. }
            | StartOfDocument
            | EndOfDocument
            | CurrentLine
            | NextLineStart
            | StartOfLineDownward
            | StartOfParagraph
            | EndOfParagraph => true,
            EndOfLine { .. }
            | NextWordEnd { .. }
            | Matching
            | FindForward { .. }
            | Left
            | Backspace
            | Right
            | StartOfLine { .. }
            | EndOfLineDownward
            | NextWordStart { .. }
            | PreviousWordStart { .. }
            | FirstNonWhitespace { .. }
            | FindBackward { .. } => false,
        }
    }

    pub fn infallible(&self) -> bool {
        use Motion::*;
        match self {
            StartOfDocument | EndOfDocument | CurrentLine => true,
            Down { .. }
            | Up { .. }
            | EndOfLine { .. }
            | NextWordEnd { .. }
            | Matching
            | FindForward { .. }
            | Left
            | Backspace
            | Right
            | StartOfLine { .. }
            | StartOfParagraph
            | EndOfParagraph
            | StartOfLineDownward
            | EndOfLineDownward
            | NextWordStart { .. }
            | PreviousWordStart { .. }
            | FirstNonWhitespace { .. }
            | FindBackward { .. }
            | NextLineStart => false,
        }
    }

    pub fn inclusive(&self) -> bool {
        use Motion::*;
        match self {
            Down { .. }
            | Up { .. }
            | StartOfDocument
            | EndOfDocument
            | CurrentLine
            | EndOfLine { .. }
            | EndOfLineDownward
            | NextWordEnd { .. }
            | Matching
            | FindForward { .. }
            | NextLineStart => true,
            Left
            | Backspace
            | Right
            | StartOfLine { .. }
            | StartOfLineDownward
            | StartOfParagraph
            | EndOfParagraph
            | NextWordStart { .. }
            | PreviousWordStart { .. }
            | FirstNonWhitespace { .. }
            | FindBackward { .. } => false,
        }
    }

    pub fn move_point(
        &self,
        map: &DisplaySnapshot,
        point: DisplayPoint,
        goal: SelectionGoal,
        maybe_times: Option<usize>,
    ) -> Option<(DisplayPoint, SelectionGoal)> {
        let times = maybe_times.unwrap_or(1);
        use Motion::*;
        let infallible = self.infallible();
        let (new_point, goal) = match self {
            Left => (left(map, point, times), SelectionGoal::None),
            Backspace => (backspace(map, point, times), SelectionGoal::None),
            Down {
                display_lines: false,
            } => down(map, point, goal, times),
            Down {
                display_lines: true,
            } => down_display(map, point, goal, times),
            Up {
                display_lines: false,
            } => up(map, point, goal, times),
            Up {
                display_lines: true,
            } => up_display(map, point, goal, times),
            Right => (right(map, point, times), SelectionGoal::None),
            NextWordStart { ignore_punctuation } => (
                next_word_start(map, point, *ignore_punctuation, times),
                SelectionGoal::None,
            ),
            NextWordEnd { ignore_punctuation } => (
                next_word_end(map, point, *ignore_punctuation, times),
                SelectionGoal::None,
            ),
            PreviousWordStart { ignore_punctuation } => (
                previous_word_start(map, point, *ignore_punctuation, times),
                SelectionGoal::None,
            ),
            FirstNonWhitespace { display_lines } => (
                first_non_whitespace(map, *display_lines, point),
                SelectionGoal::None,
            ),
            StartOfLine { display_lines } => (
                start_of_line(map, *display_lines, point),
                SelectionGoal::None,
            ),
            EndOfLine { display_lines } => {
                (end_of_line(map, *display_lines, point), SelectionGoal::None)
            }
            StartOfParagraph => (
                movement::start_of_paragraph(map, point, times),
                SelectionGoal::None,
            ),
            EndOfParagraph => (
                map.clip_at_line_end(movement::end_of_paragraph(map, point, times)),
                SelectionGoal::None,
            ),
            CurrentLine => (next_line_end(map, point, times), SelectionGoal::None),
            StartOfDocument => (start_of_document(map, point, times), SelectionGoal::None),
            EndOfDocument => (
                end_of_document(map, point, maybe_times),
                SelectionGoal::None,
            ),
            Matching => (matching(map, point), SelectionGoal::None),
            FindForward { before, char } => (
                find_forward(map, point, *before, *char, times),
                SelectionGoal::None,
            ),
            FindBackward { after, char } => (
                find_backward(map, point, *after, *char, times),
                SelectionGoal::None,
            ),
            NextLineStart => (next_line_start(map, point, times), SelectionGoal::None),
            StartOfLineDownward => (next_line_start(map, point, times - 1), SelectionGoal::None),
            EndOfLineDownward => (next_line_end(map, point, times), SelectionGoal::None),
        };

        (new_point != point || infallible).then_some((new_point, goal))
    }

    // Expands a selection using self motion for an operator
    pub fn expand_selection(
        &self,
        map: &DisplaySnapshot,
        selection: &mut Selection<DisplayPoint>,
        times: Option<usize>,
        expand_to_surrounding_newline: bool,
    ) -> bool {
        if let Some((new_head, goal)) =
            self.move_point(map, selection.head(), selection.goal, times)
        {
            selection.set_head(new_head, goal);

            if self.linewise() {
                selection.start = map.prev_line_boundary(selection.start.to_point(map)).1;

                if expand_to_surrounding_newline {
                    if selection.end.row() < map.max_point().row() {
                        *selection.end.row_mut() += 1;
                        *selection.end.column_mut() = 0;
                        selection.end = map.clip_point(selection.end, Bias::Right);
                        // Don't reset the end here
                        return true;
                    } else if selection.start.row() > 0 {
                        *selection.start.row_mut() -= 1;
                        *selection.start.column_mut() = map.line_len(selection.start.row());
                        selection.start = map.clip_point(selection.start, Bias::Left);
                    }
                }

                (_, selection.end) = map.next_line_boundary(selection.end.to_point(map));
            } else {
                // If the motion is exclusive and the end of the motion is in column 1, the
                // end of the motion is moved to the end of the previous line and the motion
                // becomes inclusive. Example: "}" moves to the first line after a paragraph,
                // but "d}" will not include that line.
                let mut inclusive = self.inclusive();
                if !inclusive
                    && self != &Motion::Backspace
                    && selection.end.row() > selection.start.row()
                    && selection.end.column() == 0
                {
                    inclusive = true;
                    *selection.end.row_mut() -= 1;
                    *selection.end.column_mut() = 0;
                    selection.end = map.clip_point(
                        map.next_line_boundary(selection.end.to_point(map)).1,
                        Bias::Left,
                    );
                }

                if inclusive && selection.end.column() < map.line_len(selection.end.row()) {
                    *selection.end.column_mut() += 1;
                }
            }
            true
        } else {
            false
        }
    }
}

fn left(map: &DisplaySnapshot, mut point: DisplayPoint, times: usize) -> DisplayPoint {
    for _ in 0..times {
        point = movement::saturating_left(map, point);
        if point.column() == 0 {
            break;
        }
    }
    point
}

fn backspace(map: &DisplaySnapshot, mut point: DisplayPoint, times: usize) -> DisplayPoint {
    for _ in 0..times {
        point = movement::left(map, point);
    }
    point
}

fn down(
    map: &DisplaySnapshot,
    point: DisplayPoint,
    mut goal: SelectionGoal,
    times: usize,
) -> (DisplayPoint, SelectionGoal) {
    let start = map.display_point_to_fold_point(point, Bias::Left);

    let goal_column = match goal {
        SelectionGoal::Column(column) => column,
        SelectionGoal::ColumnRange { end, .. } => end,
        _ => {
            goal = SelectionGoal::Column(start.column());
            start.column()
        }
    };

    let new_row = cmp::min(
        start.row() + times as u32,
        map.buffer_snapshot.max_point().row,
    );
    let new_col = cmp::min(goal_column, map.fold_snapshot.line_len(new_row));
    let point = map.fold_point_to_display_point(FoldPoint::new(new_row, new_col));

    (map.clip_point(point, Bias::Left), goal)
}

fn down_display(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    mut goal: SelectionGoal,
    times: usize,
) -> (DisplayPoint, SelectionGoal) {
    for _ in 0..times {
        (point, goal) = movement::down(map, point, goal, true);
    }

    (point, goal)
}

pub(crate) fn up(
    map: &DisplaySnapshot,
    point: DisplayPoint,
    mut goal: SelectionGoal,
    times: usize,
) -> (DisplayPoint, SelectionGoal) {
    let start = map.display_point_to_fold_point(point, Bias::Left);

    let goal_column = match goal {
        SelectionGoal::Column(column) => column,
        SelectionGoal::ColumnRange { end, .. } => end,
        _ => {
            goal = SelectionGoal::Column(start.column());
            start.column()
        }
    };

    let new_row = start.row().saturating_sub(times as u32);
    let new_col = cmp::min(goal_column, map.fold_snapshot.line_len(new_row));
    let point = map.fold_point_to_display_point(FoldPoint::new(new_row, new_col));

    (map.clip_point(point, Bias::Left), goal)
}

fn up_display(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    mut goal: SelectionGoal,
    times: usize,
) -> (DisplayPoint, SelectionGoal) {
    for _ in 0..times {
        (point, goal) = movement::up(map, point, goal, true);
    }

    (point, goal)
}

pub(crate) fn right(map: &DisplaySnapshot, mut point: DisplayPoint, times: usize) -> DisplayPoint {
    for _ in 0..times {
        let new_point = movement::saturating_right(map, point);
        if point == new_point {
            break;
        }
        point = new_point;
    }
    point
}

pub(crate) fn next_word_start(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    ignore_punctuation: bool,
    times: usize,
) -> DisplayPoint {
    let scope = map.buffer_snapshot.language_scope_at(point.to_point(map));
    for _ in 0..times {
        let mut crossed_newline = false;
        point = movement::find_boundary(map, point, FindRange::MultiLine, |left, right| {
            let left_kind = char_kind(&scope, left).coerce_punctuation(ignore_punctuation);
            let right_kind = char_kind(&scope, right).coerce_punctuation(ignore_punctuation);
            let at_newline = right == '\n';

            let found = (left_kind != right_kind && right_kind != CharKind::Whitespace)
                || at_newline && crossed_newline
                || at_newline && left == '\n'; // Prevents skipping repeated empty lines

            crossed_newline |= at_newline;
            found
        })
    }
    point
}

fn next_word_end(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    ignore_punctuation: bool,
    times: usize,
) -> DisplayPoint {
    let scope = map.buffer_snapshot.language_scope_at(point.to_point(map));
    for _ in 0..times {
        if point.column() < map.line_len(point.row()) {
            *point.column_mut() += 1;
        } else if point.row() < map.max_buffer_row() {
            *point.row_mut() += 1;
            *point.column_mut() = 0;
        }
        point = movement::find_boundary(map, point, FindRange::MultiLine, |left, right| {
            let left_kind = char_kind(&scope, left).coerce_punctuation(ignore_punctuation);
            let right_kind = char_kind(&scope, right).coerce_punctuation(ignore_punctuation);

            left_kind != right_kind && left_kind != CharKind::Whitespace
        });

        // find_boundary clips, so if the character after the next character is a newline or at the end of the document, we know
        // we have backtracked already
        if !map
            .chars_at(point)
            .nth(1)
            .map(|(c, _)| c == '\n')
            .unwrap_or(true)
        {
            *point.column_mut() = point.column().saturating_sub(1);
        }
        point = map.clip_point(point, Bias::Left);
    }
    point
}

fn previous_word_start(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    ignore_punctuation: bool,
    times: usize,
) -> DisplayPoint {
    let scope = map.buffer_snapshot.language_scope_at(point.to_point(map));
    for _ in 0..times {
        // This works even though find_preceding_boundary is called for every character in the line containing
        // cursor because the newline is checked only once.
        point =
            movement::find_preceding_boundary(map, point, FindRange::MultiLine, |left, right| {
                let left_kind = char_kind(&scope, left).coerce_punctuation(ignore_punctuation);
                let right_kind = char_kind(&scope, right).coerce_punctuation(ignore_punctuation);

                (left_kind != right_kind && !right.is_whitespace()) || left == '\n'
            });
    }
    point
}

fn first_non_whitespace(
    map: &DisplaySnapshot,
    display_lines: bool,
    from: DisplayPoint,
) -> DisplayPoint {
    let mut last_point = start_of_line(map, display_lines, from);
    let scope = map.buffer_snapshot.language_scope_at(from.to_point(map));
    for (ch, point) in map.chars_at(last_point) {
        if ch == '\n' {
            return from;
        }

        last_point = point;

        if char_kind(&scope, ch) != CharKind::Whitespace {
            break;
        }
    }

    map.clip_point(last_point, Bias::Left)
}

pub(crate) fn start_of_line(
    map: &DisplaySnapshot,
    display_lines: bool,
    point: DisplayPoint,
) -> DisplayPoint {
    if display_lines {
        map.clip_point(DisplayPoint::new(point.row(), 0), Bias::Right)
    } else {
        map.prev_line_boundary(point.to_point(map)).1
    }
}

pub(crate) fn end_of_line(
    map: &DisplaySnapshot,
    display_lines: bool,
    point: DisplayPoint,
) -> DisplayPoint {
    if display_lines {
        map.clip_point(
            DisplayPoint::new(point.row(), map.line_len(point.row())),
            Bias::Left,
        )
    } else {
        map.clip_point(map.next_line_boundary(point.to_point(map)).1, Bias::Left)
    }
}

fn start_of_document(map: &DisplaySnapshot, point: DisplayPoint, line: usize) -> DisplayPoint {
    let mut new_point = Point::new((line - 1) as u32, 0).to_display_point(map);
    *new_point.column_mut() = point.column();
    map.clip_point(new_point, Bias::Left)
}

fn end_of_document(
    map: &DisplaySnapshot,
    point: DisplayPoint,
    line: Option<usize>,
) -> DisplayPoint {
    let new_row = if let Some(line) = line {
        (line - 1) as u32
    } else {
        map.max_buffer_row()
    };

    let new_point = Point::new(new_row, point.column());
    map.clip_point(new_point.to_display_point(map), Bias::Left)
}

fn matching(map: &DisplaySnapshot, display_point: DisplayPoint) -> DisplayPoint {
    // https://github.com/vim/vim/blob/1d87e11a1ef201b26ed87585fba70182ad0c468a/runtime/doc/motion.txt#L1200
    let point = display_point.to_point(map);
    let offset = point.to_offset(&map.buffer_snapshot);

    // Ensure the range is contained by the current line.
    let mut line_end = map.next_line_boundary(point).0;
    if line_end == point {
        line_end = map.max_point().to_point(map);
    }

    let line_range = map.prev_line_boundary(point).0..line_end;
    let visible_line_range =
        line_range.start..Point::new(line_range.end.row, line_range.end.column.saturating_sub(1));
    let ranges = map
        .buffer_snapshot
        .bracket_ranges(visible_line_range.clone());
    if let Some(ranges) = ranges {
        let line_range = line_range.start.to_offset(&map.buffer_snapshot)
            ..line_range.end.to_offset(&map.buffer_snapshot);
        let mut closest_pair_destination = None;
        let mut closest_distance = usize::MAX;

        for (open_range, close_range) in ranges {
            if open_range.start >= offset && line_range.contains(&open_range.start) {
                let distance = open_range.start - offset;
                if distance < closest_distance {
                    closest_pair_destination = Some(close_range.start);
                    closest_distance = distance;
                    continue;
                }
            }

            if close_range.start >= offset && line_range.contains(&close_range.start) {
                let distance = close_range.start - offset;
                if distance < closest_distance {
                    closest_pair_destination = Some(open_range.start);
                    closest_distance = distance;
                    continue;
                }
            }

            continue;
        }

        closest_pair_destination
            .map(|destination| destination.to_display_point(map))
            .unwrap_or(display_point)
    } else {
        display_point
    }
}

fn find_forward(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    before: bool,
    target: char,
    times: usize,
) -> DisplayPoint {
    let mut to = from;
    let mut found = false;

    for _ in 0..times {
        found = false;
        to = find_boundary(map, to, FindRange::SingleLine, |_, right| {
            found = right == target;
            found
        });
    }

    if found {
        if before && to.column() > 0 {
            *to.column_mut() -= 1;
            map.clip_point(to, Bias::Left)
        } else {
            to
        }
    } else {
        from
    }
}

fn find_backward(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    after: bool,
    target: char,
    times: usize,
) -> DisplayPoint {
    let mut to = from;

    for _ in 0..times {
        to = find_preceding_boundary(map, to, FindRange::SingleLine, |_, right| right == target);
    }

    if map.buffer_snapshot.chars_at(to.to_point(map)).next() == Some(target) {
        if after {
            *to.column_mut() += 1;
            map.clip_point(to, Bias::Right)
        } else {
            to
        }
    } else {
        from
    }
}

fn next_line_start(map: &DisplaySnapshot, point: DisplayPoint, times: usize) -> DisplayPoint {
    let correct_line = down(map, point, SelectionGoal::None, times).0;
    first_non_whitespace(map, false, correct_line)
}

fn next_line_end(map: &DisplaySnapshot, mut point: DisplayPoint, times: usize) -> DisplayPoint {
    if times > 1 {
        point = down(map, point, SelectionGoal::None, times - 1).0;
    }
    end_of_line(map, false, point)
}

#[cfg(test)]

mod test {

    use crate::test::NeovimBackedTestContext;
    use indoc::indoc;

    #[gpui::test]
    async fn test_start_end_of_paragraph(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        let initial_state = indoc! {r"ˇabc
            def

            paragraph
            the second



            third and
            final"};

        // goes down once
        cx.set_shared_state(initial_state).await;
        cx.simulate_shared_keystrokes(["}"]).await;
        cx.assert_shared_state(indoc! {r"abc
            def
            ˇ
            paragraph
            the second



            third and
            final"})
            .await;

        // goes up once
        cx.simulate_shared_keystrokes(["{"]).await;
        cx.assert_shared_state(initial_state).await;

        // goes down twice
        cx.simulate_shared_keystrokes(["2", "}"]).await;
        cx.assert_shared_state(indoc! {r"abc
            def

            paragraph
            the second
            ˇ


            third and
            final"})
            .await;

        // goes down over multiple blanks
        cx.simulate_shared_keystrokes(["}"]).await;
        cx.assert_shared_state(indoc! {r"abc
                def

                paragraph
                the second



                third and
                finaˇl"})
            .await;

        // goes up twice
        cx.simulate_shared_keystrokes(["2", "{"]).await;
        cx.assert_shared_state(indoc! {r"abc
                def
                ˇ
                paragraph
                the second



                third and
                final"})
            .await
    }

    #[gpui::test]
    async fn test_matching(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {r"func ˇ(a string) {
                do(something(with<Types>.and_arrays[0, 2]))
            }"})
            .await;
        cx.simulate_shared_keystrokes(["%"]).await;
        cx.assert_shared_state(indoc! {r"func (a stringˇ) {
                do(something(with<Types>.and_arrays[0, 2]))
            }"})
            .await;

        // test it works on the last character of the line
        cx.set_shared_state(indoc! {r"func (a string) ˇ{
            do(something(with<Types>.and_arrays[0, 2]))
            }"})
            .await;
        cx.simulate_shared_keystrokes(["%"]).await;
        cx.assert_shared_state(indoc! {r"func (a string) {
            do(something(with<Types>.and_arrays[0, 2]))
            ˇ}"})
            .await;

        // test it works on immediate nesting
        cx.set_shared_state("ˇ{()}").await;
        cx.simulate_shared_keystrokes(["%"]).await;
        cx.assert_shared_state("{()ˇ}").await;
        cx.simulate_shared_keystrokes(["%"]).await;
        cx.assert_shared_state("ˇ{()}").await;

        // test it works on immediate nesting inside braces
        cx.set_shared_state("{\n    ˇ{()}\n}").await;
        cx.simulate_shared_keystrokes(["%"]).await;
        cx.assert_shared_state("{\n    {()ˇ}\n}").await;

        // test it jumps to the next paren on a line
        cx.set_shared_state("func ˇboop() {\n}").await;
        cx.simulate_shared_keystrokes(["%"]).await;
        cx.assert_shared_state("func boop(ˇ) {\n}").await;
    }

    #[gpui::test]
    async fn test_comma_semicolon(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇone two three four").await;
        cx.simulate_shared_keystrokes(["f", "o"]).await;
        cx.assert_shared_state("one twˇo three four").await;
        cx.simulate_shared_keystrokes([","]).await;
        cx.assert_shared_state("ˇone two three four").await;
        cx.simulate_shared_keystrokes(["2", ";"]).await;
        cx.assert_shared_state("one two three fˇour").await;
        cx.simulate_shared_keystrokes(["shift-t", "e"]).await;
        cx.assert_shared_state("one two threeˇ four").await;
        cx.simulate_shared_keystrokes(["3", ";"]).await;
        cx.assert_shared_state("oneˇ two three four").await;
        cx.simulate_shared_keystrokes([","]).await;
        cx.assert_shared_state("one two thˇree four").await;
    }

    #[gpui::test]
    async fn test_next_line_start(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state("ˇone\n  two\nthree").await;
        cx.simulate_shared_keystrokes(["enter"]).await;
        cx.assert_shared_state("one\n  ˇtwo\nthree").await;
    }
}
