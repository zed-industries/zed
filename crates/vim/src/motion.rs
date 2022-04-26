use editor::{
    char_kind,
    display_map::{DisplaySnapshot, ToDisplayPoint},
    movement, Bias, DisplayPoint,
};
use gpui::{actions, impl_actions, MutableAppContext};
use language::{Selection, SelectionGoal};
use serde::Deserialize;
use workspace::Workspace;

use crate::{
    normal::normal_motion,
    state::{Mode, Operator},
    Vim,
};

#[derive(Copy, Clone, Debug)]
pub enum Motion {
    Left,
    Down,
    Up,
    Right,
    NextWordStart { ignore_punctuation: bool },
    NextWordEnd { ignore_punctuation: bool },
    PreviousWordStart { ignore_punctuation: bool },
    StartOfLine,
    EndOfLine,
    StartOfDocument,
    EndOfDocument,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NextWordStart {
    #[serde(default)]
    ignore_punctuation: bool,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct NextWordEnd {
    #[serde(default)]
    ignore_punctuation: bool,
}

#[derive(Clone, Deserialize)]
#[serde(rename_all = "camelCase")]
struct PreviousWordStart {
    #[serde(default)]
    ignore_punctuation: bool,
}

actions!(
    vim,
    [
        Left,
        Down,
        Up,
        Right,
        StartOfLine,
        EndOfLine,
        StartOfDocument,
        EndOfDocument
    ]
);
impl_actions!(vim, [NextWordStart, NextWordEnd, PreviousWordStart]);

pub fn init(cx: &mut MutableAppContext) {
    cx.add_action(|_: &mut Workspace, _: &Left, cx: _| motion(Motion::Left, cx));
    cx.add_action(|_: &mut Workspace, _: &Down, cx: _| motion(Motion::Down, cx));
    cx.add_action(|_: &mut Workspace, _: &Up, cx: _| motion(Motion::Up, cx));
    cx.add_action(|_: &mut Workspace, _: &Right, cx: _| motion(Motion::Right, cx));
    cx.add_action(|_: &mut Workspace, _: &StartOfLine, cx: _| motion(Motion::StartOfLine, cx));
    cx.add_action(|_: &mut Workspace, _: &EndOfLine, cx: _| motion(Motion::EndOfLine, cx));
    cx.add_action(|_: &mut Workspace, _: &StartOfDocument, cx: _| {
        motion(Motion::StartOfDocument, cx)
    });
    cx.add_action(|_: &mut Workspace, _: &EndOfDocument, cx: _| motion(Motion::EndOfDocument, cx));

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

fn motion(motion: Motion, cx: &mut MutableAppContext) {
    Vim::update(cx, |vim, cx| {
        if let Some(Operator::Namespace(_)) = vim.active_operator() {
            vim.pop_operator(cx);
        }
    });
    match Vim::read(cx).state.mode {
        Mode::Normal => normal_motion(motion, cx),
        Mode::Insert => {
            // Shouldn't execute a motion in insert mode. Ignoring
        }
    }
}

// Motion handling is specified here:
// https://github.com/vim/vim/blob/master/runtime/doc/motion.txt
impl Motion {
    pub fn linewise(self) -> bool {
        use Motion::*;
        match self {
            Down | Up | StartOfDocument | EndOfDocument => true,
            _ => false,
        }
    }

    pub fn inclusive(self) -> bool {
        use Motion::*;
        if self.linewise() {
            return true;
        }

        match self {
            EndOfLine | NextWordEnd { .. } => true,
            Left | Right | StartOfLine | NextWordStart { .. } | PreviousWordStart { .. } => false,
            _ => panic!("Exclusivity not defined for {self:?}"),
        }
    }

    pub fn move_point(
        self,
        map: &DisplaySnapshot,
        point: DisplayPoint,
        goal: SelectionGoal,
    ) -> (DisplayPoint, SelectionGoal) {
        use Motion::*;
        match self {
            Left => (left(map, point), SelectionGoal::None),
            Down => movement::down(map, point, goal, true),
            Up => movement::up(map, point, goal, true),
            Right => (right(map, point), SelectionGoal::None),
            NextWordStart { ignore_punctuation } => (
                next_word_start(map, point, ignore_punctuation),
                SelectionGoal::None,
            ),
            NextWordEnd { ignore_punctuation } => (
                next_word_end(map, point, ignore_punctuation),
                SelectionGoal::None,
            ),
            PreviousWordStart { ignore_punctuation } => (
                previous_word_start(map, point, ignore_punctuation),
                SelectionGoal::None,
            ),
            StartOfLine => (start_of_line(map, point), SelectionGoal::None),
            EndOfLine => (end_of_line(map, point), SelectionGoal::None),
            StartOfDocument => (start_of_document(map, point), SelectionGoal::None),
            EndOfDocument => (end_of_document(map, point), SelectionGoal::None),
        }
    }

    // Expands a selection using self motion for an operator
    pub fn expand_selection(
        self,
        map: &DisplaySnapshot,
        selection: &mut Selection<DisplayPoint>,
        expand_to_surrounding_newline: bool,
    ) {
        let (head, goal) = self.move_point(map, selection.head(), selection.goal);
        selection.set_head(head, goal);

        if self.linewise() {
            selection.start = map.prev_line_boundary(selection.start.to_point(map)).1;

            if expand_to_surrounding_newline {
                if selection.end.row() < map.max_point().row() {
                    *selection.end.row_mut() += 1;
                    *selection.end.column_mut() = 0;
                    // Don't reset the end here
                    return;
                } else if selection.start.row() > 0 {
                    *selection.start.row_mut() -= 1;
                    *selection.start.column_mut() = map.line_len(selection.start.row());
                }
            }

            selection.end = map.next_line_boundary(selection.end.to_point(map)).1;
        } else {
            // If the motion is exclusive and the end of the motion is in column 1, the
            // end of the motion is moved to the end of the previous line and the motion
            // becomes inclusive. Example: "}" moves to the first line after a paragraph,
            // but "d}" will not include that line.
            let mut inclusive = self.inclusive();
            if !inclusive
                && selection.end.row() > selection.start.row()
                && selection.end.column() == 0
                && selection.end.row() > 0
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
    }
}

fn left(map: &DisplaySnapshot, mut point: DisplayPoint) -> DisplayPoint {
    *point.column_mut() = point.column().saturating_sub(1);
    map.clip_point(point, Bias::Left)
}

fn right(map: &DisplaySnapshot, mut point: DisplayPoint) -> DisplayPoint {
    *point.column_mut() += 1;
    map.clip_point(point, Bias::Right)
}

fn next_word_start(
    map: &DisplaySnapshot,
    point: DisplayPoint,
    ignore_punctuation: bool,
) -> DisplayPoint {
    let mut crossed_newline = false;
    movement::find_boundary(map, point, |left, right| {
        let left_kind = char_kind(left).coerce_punctuation(ignore_punctuation);
        let right_kind = char_kind(right).coerce_punctuation(ignore_punctuation);
        let at_newline = right == '\n';

        let found = (left_kind != right_kind && !right.is_whitespace())
            || at_newline && crossed_newline
            || at_newline && left == '\n'; // Prevents skipping repeated empty lines

        if at_newline {
            crossed_newline = true;
        }
        found
    })
}

fn next_word_end(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    ignore_punctuation: bool,
) -> DisplayPoint {
    *point.column_mut() += 1;
    point = movement::find_boundary(map, point, |left, right| {
        let left_kind = char_kind(left).coerce_punctuation(ignore_punctuation);
        let right_kind = char_kind(right).coerce_punctuation(ignore_punctuation);

        left_kind != right_kind && !left.is_whitespace()
    });
    // find_boundary clips, so if the character after the next character is a newline or at the end of the document, we know
    // we have backtraced already
    if !map
        .chars_at(point)
        .skip(1)
        .next()
        .map(|c| c == '\n')
        .unwrap_or(true)
    {
        *point.column_mut() = point.column().saturating_sub(1);
    }
    map.clip_point(point, Bias::Left)
}

fn previous_word_start(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    ignore_punctuation: bool,
) -> DisplayPoint {
    // This works even though find_preceding_boundary is called for every character in the line containing
    // cursor because the newline is checked only once.
    point = movement::find_preceding_boundary(map, point, |left, right| {
        let left_kind = char_kind(left).coerce_punctuation(ignore_punctuation);
        let right_kind = char_kind(right).coerce_punctuation(ignore_punctuation);

        (left_kind != right_kind && !right.is_whitespace()) || left == '\n'
    });
    point
}

fn start_of_line(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    map.prev_line_boundary(point.to_point(map)).1
}

fn end_of_line(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    map.clip_point(map.next_line_boundary(point.to_point(map)).1, Bias::Left)
}

fn start_of_document(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    let mut new_point = 0usize.to_display_point(map);
    *new_point.column_mut() = point.column();
    map.clip_point(new_point, Bias::Left)
}

fn end_of_document(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    let mut new_point = map.max_point();
    *new_point.column_mut() = point.column();
    map.clip_point(new_point, Bias::Left)
}
