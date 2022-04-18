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

#[derive(Copy, Clone)]
pub enum Motion {
    Left,
    Down,
    Up,
    Right,
    NextWordStart {
        ignore_punctuation: bool,
        stop_at_newline: bool,
    },
    NextWordEnd {
        ignore_punctuation: bool,
    },
    PreviousWordStart {
        ignore_punctuation: bool,
    },
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
    #[serde(default)]
    stop_at_newline: bool,
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
        |_: &mut Workspace,
         &NextWordStart {
             ignore_punctuation,
             stop_at_newline,
         }: &NextWordStart,
         cx: _| {
            motion(
                Motion::NextWordStart {
                    ignore_punctuation,
                    stop_at_newline,
                },
                cx,
            )
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
        Mode::Insert => panic!("motion bindings in insert mode interfere with normal typing"),
    }
}

impl Motion {
    pub fn move_point(
        self,
        map: &DisplaySnapshot,
        point: DisplayPoint,
        goal: SelectionGoal,
    ) -> (DisplayPoint, SelectionGoal) {
        use Motion::*;
        match self {
            Left => (left(map, point), SelectionGoal::None),
            Down => movement::down(map, point, goal),
            Up => movement::up(map, point, goal),
            Right => (right(map, point), SelectionGoal::None),
            NextWordStart {
                ignore_punctuation,
                stop_at_newline,
            } => (
                next_word_start(map, point, ignore_punctuation, stop_at_newline),
                SelectionGoal::None,
            ),
            NextWordEnd { ignore_punctuation } => (
                next_word_end(map, point, ignore_punctuation, true),
                SelectionGoal::None,
            ),
            PreviousWordStart { ignore_punctuation } => (
                previous_word_start(map, point, ignore_punctuation),
                SelectionGoal::None,
            ),
            StartOfLine => (
                movement::line_beginning(map, point, false),
                SelectionGoal::None,
            ),
            EndOfLine => (
                map.clip_point(movement::line_end(map, point, false), Bias::Left),
                SelectionGoal::None,
            ),
            StartOfDocument => (start_of_document(map), SelectionGoal::None),
            EndOfDocument => (end_of_document(map), SelectionGoal::None),
        }
    }

    pub fn expand_selection(self, map: &DisplaySnapshot, selection: &mut Selection<DisplayPoint>) {
        use Motion::*;
        match self {
            Up => {
                let (start, _) = Up.move_point(map, selection.start, SelectionGoal::None);
                // Cursor at top of file. Return early rather
                if start == selection.start {
                    return;
                }
                let (start, _) = StartOfLine.move_point(map, start, SelectionGoal::None);
                let (end, _) = EndOfLine.move_point(map, selection.end, SelectionGoal::None);
                selection.start = start;
                selection.end = end;
                // TODO: Make sure selection goal is correct here
                selection.goal = SelectionGoal::None;
            }
            Down => {
                let (end, _) = Down.move_point(map, selection.end, SelectionGoal::None);
                // Cursor at top of file. Return early rather
                if end == selection.start {
                    return;
                }
                let (start, _) = StartOfLine.move_point(map, selection.start, SelectionGoal::None);
                let (end, _) = EndOfLine.move_point(map, end, SelectionGoal::None);
                selection.start = start;
                selection.end = end;
                // TODO: Make sure selection goal is correct here
                selection.goal = SelectionGoal::None;
            }
            NextWordEnd { ignore_punctuation } => {
                selection.set_head(
                    next_word_end(map, selection.head(), ignore_punctuation, false),
                    SelectionGoal::None,
                );
            }
            _ => {
                let (head, goal) = self.move_point(map, selection.head(), selection.goal);
                selection.set_head(head, goal);
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
    stop_at_newline: bool,
) -> DisplayPoint {
    let mut crossed_newline = false;
    movement::find_boundary(map, point, |left, right| {
        let left_kind = char_kind(left).coerce_punctuation(ignore_punctuation);
        let right_kind = char_kind(right).coerce_punctuation(ignore_punctuation);
        let at_newline = right == '\n';

        let found = (left_kind != right_kind && !right.is_whitespace())
            || (at_newline && (crossed_newline || stop_at_newline))
            || (at_newline && left == '\n'); // Prevents skipping repeated empty lines

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
    before_end_character: bool,
) -> DisplayPoint {
    *point.column_mut() += 1;
    point = movement::find_boundary(map, point, |left, right| {
        let left_kind = char_kind(left).coerce_punctuation(ignore_punctuation);
        let right_kind = char_kind(right).coerce_punctuation(ignore_punctuation);

        left_kind != right_kind && !left.is_whitespace()
    });
    // find_boundary clips, so if the character after the next character is a newline or at the end of the document, we know
    // we have backtraced already
    if before_end_character
        && !map
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

fn start_of_document(map: &DisplaySnapshot) -> DisplayPoint {
    0usize.to_display_point(map)
}

fn end_of_document(map: &DisplaySnapshot) -> DisplayPoint {
    map.clip_point(map.max_point(), Bias::Left)
}
