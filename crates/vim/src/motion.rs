use editor::{
    char_kind,
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement, Bias, CharKind, DisplayPoint,
};
use gpui::{actions, impl_actions, MutableAppContext};
use language::{Point, Selection, SelectionGoal};
use serde::Deserialize;
use workspace::Workspace;

use crate::{
    normal::normal_motion,
    state::{Mode, Operator},
    visual::visual_motion,
    Vim,
};

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum Motion {
    Left,
    Backspace,
    Down,
    Up,
    Right,
    NextWordStart { ignore_punctuation: bool },
    NextWordEnd { ignore_punctuation: bool },
    PreviousWordStart { ignore_punctuation: bool },
    FirstNonWhitespace,
    CurrentLine,
    StartOfLine,
    EndOfLine,
    StartOfDocument,
    EndOfDocument,
    Matching,
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

actions!(
    vim,
    [
        Left,
        Backspace,
        Down,
        Up,
        Right,
        FirstNonWhitespace,
        StartOfLine,
        EndOfLine,
        CurrentLine,
        StartOfDocument,
        EndOfDocument,
        Matching,
    ]
);
impl_actions!(vim, [NextWordStart, NextWordEnd, PreviousWordStart]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(|_: &mut Workspace, _: &Left, cx: _| motion(Motion::Left, cx));
    cx.add_action(|_: &mut Workspace, _: &Backspace, cx: _| motion(Motion::Backspace, cx));
    cx.add_action(|_: &mut Workspace, _: &Down, cx: _| motion(Motion::Down, cx));
    cx.add_action(|_: &mut Workspace, _: &Up, cx: _| motion(Motion::Up, cx));
    cx.add_action(|_: &mut Workspace, _: &Right, cx: _| motion(Motion::Right, cx));
    cx.add_action(|_: &mut Workspace, _: &FirstNonWhitespace, cx: _| {
        motion(Motion::FirstNonWhitespace, cx)
    });
    cx.add_action(|_: &mut Workspace, _: &StartOfLine, cx: _| motion(Motion::StartOfLine, cx));
    cx.add_action(|_: &mut Workspace, _: &EndOfLine, cx: _| motion(Motion::EndOfLine, cx));
    cx.add_action(|_: &mut Workspace, _: &CurrentLine, cx: _| motion(Motion::CurrentLine, cx));
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
}

pub(crate) fn motion(motion: Motion, cx: &mut MutableAppContext) {
    if let Some(Operator::Namespace(_)) = Vim::read(cx).active_operator() {
        Vim::update(cx, |vim, cx| vim.pop_operator(cx));
    }

    let times = Vim::update(cx, |vim, cx| vim.pop_number_operator(cx));
    let operator = Vim::read(cx).active_operator();
    match Vim::read(cx).state.mode {
        Mode::Normal => normal_motion(motion, operator, times, cx),
        Mode::Visual { .. } => visual_motion(motion, times, cx),
        Mode::Insert => {
            // Shouldn't execute a motion in insert mode. Ignoring
        }
    }
    Vim::update(cx, |vim, cx| vim.clear_operator(cx));
}

// Motion handling is specified here:
// https://github.com/vim/vim/blob/master/runtime/doc/motion.txt
impl Motion {
    pub fn linewise(self) -> bool {
        use Motion::*;
        matches!(
            self,
            Down | Up | StartOfDocument | EndOfDocument | CurrentLine
        )
    }

    pub fn infallible(self) -> bool {
        use Motion::*;
        matches!(self, StartOfDocument | CurrentLine | EndOfDocument)
    }

    pub fn inclusive(self) -> bool {
        use Motion::*;
        match self {
            Down
            | Up
            | StartOfDocument
            | EndOfDocument
            | CurrentLine
            | EndOfLine
            | NextWordEnd { .. }
            | Matching => true,
            Left
            | Backspace
            | Right
            | StartOfLine
            | NextWordStart { .. }
            | PreviousWordStart { .. }
            | FirstNonWhitespace => false,
        }
    }

    pub fn move_point(
        self,
        map: &DisplaySnapshot,
        point: DisplayPoint,
        goal: SelectionGoal,
        times: usize,
    ) -> Option<(DisplayPoint, SelectionGoal)> {
        use Motion::*;
        let (new_point, goal) = match self {
            Left => (left(map, point, times), SelectionGoal::None),
            Backspace => (backspace(map, point, times), SelectionGoal::None),
            Down => down(map, point, goal, times),
            Up => up(map, point, goal, times),
            Right => (right(map, point, times), SelectionGoal::None),
            NextWordStart { ignore_punctuation } => (
                next_word_start(map, point, ignore_punctuation, times),
                SelectionGoal::None,
            ),
            NextWordEnd { ignore_punctuation } => (
                next_word_end(map, point, ignore_punctuation, times),
                SelectionGoal::None,
            ),
            PreviousWordStart { ignore_punctuation } => (
                previous_word_start(map, point, ignore_punctuation, times),
                SelectionGoal::None,
            ),
            FirstNonWhitespace => (first_non_whitespace(map, point), SelectionGoal::None),
            StartOfLine => (start_of_line(map, point), SelectionGoal::None),
            EndOfLine => (end_of_line(map, point), SelectionGoal::None),
            CurrentLine => (end_of_line(map, point), SelectionGoal::None),
            StartOfDocument => (start_of_document(map, point, times), SelectionGoal::None),
            EndOfDocument => (end_of_document(map, point, times), SelectionGoal::None),
            Matching => (matching(map, point), SelectionGoal::None),
        };

        (new_point != point || self.infallible()).then_some((new_point, goal))
    }

    // Expands a selection using self motion for an operator
    pub fn expand_selection(
        self,
        map: &DisplaySnapshot,
        selection: &mut Selection<DisplayPoint>,
        times: usize,
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
                    && self != Motion::Backspace
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
        *point.column_mut() = point.column().saturating_sub(1);
        point = map.clip_point(point, Bias::Left);
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
    mut point: DisplayPoint,
    mut goal: SelectionGoal,
    times: usize,
) -> (DisplayPoint, SelectionGoal) {
    for _ in 0..times {
        (point, goal) = movement::down(map, point, goal, true);
    }
    (point, goal)
}

fn up(
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
        let mut new_point = point;
        *new_point.column_mut() += 1;
        let new_point = map.clip_point(new_point, Bias::Right);
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
    for _ in 0..times {
        let mut crossed_newline = false;
        point = movement::find_boundary(map, point, |left, right| {
            let left_kind = char_kind(left).coerce_punctuation(ignore_punctuation);
            let right_kind = char_kind(right).coerce_punctuation(ignore_punctuation);
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
    for _ in 0..times {
        *point.column_mut() += 1;
        point = movement::find_boundary(map, point, |left, right| {
            let left_kind = char_kind(left).coerce_punctuation(ignore_punctuation);
            let right_kind = char_kind(right).coerce_punctuation(ignore_punctuation);

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
    for _ in 0..times {
        // This works even though find_preceding_boundary is called for every character in the line containing
        // cursor because the newline is checked only once.
        point = movement::find_preceding_boundary(map, point, |left, right| {
            let left_kind = char_kind(left).coerce_punctuation(ignore_punctuation);
            let right_kind = char_kind(right).coerce_punctuation(ignore_punctuation);

            (left_kind != right_kind && !right.is_whitespace()) || left == '\n'
        });
    }
    point
}

fn first_non_whitespace(map: &DisplaySnapshot, from: DisplayPoint) -> DisplayPoint {
    let mut last_point = DisplayPoint::new(from.row(), 0);
    for (ch, point) in map.chars_at(last_point) {
        if ch == '\n' {
            return from;
        }

        last_point = point;

        if char_kind(ch) != CharKind::Whitespace {
            break;
        }
    }

    map.clip_point(last_point, Bias::Left)
}

fn start_of_line(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    map.prev_line_boundary(point.to_point(map)).1
}

fn end_of_line(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    map.clip_point(map.next_line_boundary(point.to_point(map)).1, Bias::Left)
}

fn start_of_document(map: &DisplaySnapshot, point: DisplayPoint, line: usize) -> DisplayPoint {
    let mut new_point = Point::new((line - 1) as u32, 0).to_display_point(map);
    *new_point.column_mut() = point.column();
    map.clip_point(new_point, Bias::Left)
}

fn end_of_document(map: &DisplaySnapshot, point: DisplayPoint, line: usize) -> DisplayPoint {
    let mut new_point = if line == 1 {
        map.max_point()
    } else {
        Point::new((line - 1) as u32, 0).to_display_point(map)
    };
    *new_point.column_mut() = point.column();
    map.clip_point(new_point, Bias::Left)
}

fn matching(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    let offset = point.to_offset(map, Bias::Left);
    if let Some((open_range, close_range)) =
        map.buffer_snapshot.enclosing_bracket_ranges(offset..offset)
    {
        if open_range.contains(&offset) {
            close_range.start.to_display_point(map)
        } else {
            open_range.start.to_display_point(map)
        }
    } else {
        point
    }
}
