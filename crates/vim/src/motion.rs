use editor::{
    Anchor, Bias, DisplayPoint, Editor, RowExt, ToOffset, ToPoint,
    display_map::{DisplayRow, DisplaySnapshot, FoldPoint, ToDisplayPoint},
    movement::{
        self, FindRange, TextLayoutDetails, find_boundary, find_preceding_boundary_display_point,
    },
};
use gpui::{Action, Context, Window, actions, px};
use language::{CharKind, Point, Selection, SelectionGoal};
use multi_buffer::MultiBufferRow;
use schemars::JsonSchema;
use serde::Deserialize;
use std::ops::Range;
use workspace::searchable::Direction;

use crate::{
    Vim,
    normal::mark,
    state::{Mode, Operator},
    surrounds::SurroundsType,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum MotionKind {
    Linewise,
    Exclusive,
    Inclusive,
}

impl MotionKind {
    pub(crate) fn for_mode(mode: Mode) -> Self {
        match mode {
            Mode::VisualLine => MotionKind::Linewise,
            _ => MotionKind::Exclusive,
        }
    }

    pub(crate) fn linewise(&self) -> bool {
        matches!(self, MotionKind::Linewise)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Motion {
    Left,
    WrappingLeft,
    Down {
        display_lines: bool,
    },
    Up {
        display_lines: bool,
    },
    Right,
    WrappingRight,
    NextWordStart {
        ignore_punctuation: bool,
    },
    NextWordEnd {
        ignore_punctuation: bool,
    },
    PreviousWordStart {
        ignore_punctuation: bool,
    },
    PreviousWordEnd {
        ignore_punctuation: bool,
    },
    NextSubwordStart {
        ignore_punctuation: bool,
    },
    NextSubwordEnd {
        ignore_punctuation: bool,
    },
    PreviousSubwordStart {
        ignore_punctuation: bool,
    },
    PreviousSubwordEnd {
        ignore_punctuation: bool,
    },
    FirstNonWhitespace {
        display_lines: bool,
    },
    CurrentLine,
    StartOfLine {
        display_lines: bool,
    },
    MiddleOfLine {
        display_lines: bool,
    },
    EndOfLine {
        display_lines: bool,
    },
    SentenceBackward,
    SentenceForward,
    StartOfParagraph,
    EndOfParagraph,
    StartOfDocument,
    EndOfDocument,
    Matching,
    GoToPercentage,
    UnmatchedForward {
        char: char,
    },
    UnmatchedBackward {
        char: char,
    },
    FindForward {
        before: bool,
        char: char,
        mode: FindRange,
        smartcase: bool,
    },
    FindBackward {
        after: bool,
        char: char,
        mode: FindRange,
        smartcase: bool,
    },
    Sneak {
        first_char: char,
        second_char: char,
        smartcase: bool,
    },
    SneakBackward {
        first_char: char,
        second_char: char,
        smartcase: bool,
    },
    RepeatFind {
        last_find: Box<Motion>,
    },
    RepeatFindReversed {
        last_find: Box<Motion>,
    },
    NextLineStart,
    PreviousLineStart,
    StartOfLineDownward,
    EndOfLineDownward,
    GoToColumn,
    WindowTop,
    WindowMiddle,
    WindowBottom,
    NextSectionStart,
    NextSectionEnd,
    PreviousSectionStart,
    PreviousSectionEnd,
    NextMethodStart,
    NextMethodEnd,
    PreviousMethodStart,
    PreviousMethodEnd,
    NextComment,
    PreviousComment,
    PreviousLesserIndent,
    PreviousGreaterIndent,
    PreviousSameIndent,
    NextLesserIndent,
    NextGreaterIndent,
    NextSameIndent,

    // we don't have a good way to run a search synchronously, so
    // we handle search motions by running the search async and then
    // calling back into motion with this
    ZedSearchResult {
        prior_selections: Vec<Range<Anchor>>,
        new_selections: Vec<Range<Anchor>>,
    },
    Jump {
        anchor: Anchor,
        line: bool,
    },
}

#[derive(Clone, Copy)]
enum IndentType {
    Lesser,
    Greater,
    Same,
}

/// Moves to the start of the next word.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct NextWordStart {
    #[serde(default)]
    ignore_punctuation: bool,
}

/// Moves to the end of the next word.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct NextWordEnd {
    #[serde(default)]
    ignore_punctuation: bool,
}

/// Moves to the start of the previous word.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct PreviousWordStart {
    #[serde(default)]
    ignore_punctuation: bool,
}

/// Moves to the end of the previous word.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct PreviousWordEnd {
    #[serde(default)]
    ignore_punctuation: bool,
}

/// Moves to the start of the next subword.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
pub(crate) struct NextSubwordStart {
    #[serde(default)]
    pub(crate) ignore_punctuation: bool,
}

/// Moves to the end of the next subword.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
pub(crate) struct NextSubwordEnd {
    #[serde(default)]
    pub(crate) ignore_punctuation: bool,
}

/// Moves to the start of the previous subword.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
pub(crate) struct PreviousSubwordStart {
    #[serde(default)]
    pub(crate) ignore_punctuation: bool,
}

/// Moves to the end of the previous subword.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
pub(crate) struct PreviousSubwordEnd {
    #[serde(default)]
    pub(crate) ignore_punctuation: bool,
}

/// Moves cursor up by the specified number of lines.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
pub(crate) struct Up {
    #[serde(default)]
    pub(crate) display_lines: bool,
}

/// Moves cursor down by the specified number of lines.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
pub(crate) struct Down {
    #[serde(default)]
    pub(crate) display_lines: bool,
}

/// Moves to the first non-whitespace character on the current line.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct FirstNonWhitespace {
    #[serde(default)]
    display_lines: bool,
}

/// Moves to the end of the current line.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct EndOfLine {
    #[serde(default)]
    display_lines: bool,
}

/// Moves to the start of the current line.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
pub struct StartOfLine {
    #[serde(default)]
    pub(crate) display_lines: bool,
}

/// Moves to the middle of the current line.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct MiddleOfLine {
    #[serde(default)]
    display_lines: bool,
}

/// Finds the next unmatched bracket or delimiter.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct UnmatchedForward {
    #[serde(default)]
    char: char,
}

/// Finds the previous unmatched bracket or delimiter.
#[derive(Clone, Deserialize, JsonSchema, PartialEq, Action)]
#[action(namespace = vim)]
#[serde(deny_unknown_fields)]
struct UnmatchedBackward {
    #[serde(default)]
    char: char,
}

actions!(
    vim,
    [
        /// Moves cursor left one character.
        Left,
        /// Moves cursor left one character, wrapping to previous line.
        #[action(deprecated_aliases = ["vim::Backspace"])]
        WrappingLeft,
        /// Moves cursor right one character.
        Right,
        /// Moves cursor right one character, wrapping to next line.
        #[action(deprecated_aliases = ["vim::Space"])]
        WrappingRight,
        /// Selects the current line.
        CurrentLine,
        /// Moves to the start of the next sentence.
        SentenceForward,
        /// Moves to the start of the previous sentence.
        SentenceBackward,
        /// Moves to the start of the paragraph.
        StartOfParagraph,
        /// Moves to the end of the paragraph.
        EndOfParagraph,
        /// Moves to the start of the document.
        StartOfDocument,
        /// Moves to the end of the document.
        EndOfDocument,
        /// Moves to the matching bracket or delimiter.
        Matching,
        /// Goes to a percentage position in the file.
        GoToPercentage,
        /// Moves to the start of the next line.
        NextLineStart,
        /// Moves to the start of the previous line.
        PreviousLineStart,
        /// Moves to the start of a line downward.
        StartOfLineDownward,
        /// Moves to the end of a line downward.
        EndOfLineDownward,
        /// Goes to a specific column number.
        GoToColumn,
        /// Repeats the last character find.
        RepeatFind,
        /// Repeats the last character find in reverse.
        RepeatFindReversed,
        /// Moves to the top of the window.
        WindowTop,
        /// Moves to the middle of the window.
        WindowMiddle,
        /// Moves to the bottom of the window.
        WindowBottom,
        /// Moves to the start of the next section.
        NextSectionStart,
        /// Moves to the end of the next section.
        NextSectionEnd,
        /// Moves to the start of the previous section.
        PreviousSectionStart,
        /// Moves to the end of the previous section.
        PreviousSectionEnd,
        /// Moves to the start of the next method.
        NextMethodStart,
        /// Moves to the end of the next method.
        NextMethodEnd,
        /// Moves to the start of the previous method.
        PreviousMethodStart,
        /// Moves to the end of the previous method.
        PreviousMethodEnd,
        /// Moves to the next comment.
        NextComment,
        /// Moves to the previous comment.
        PreviousComment,
        /// Moves to the previous line with lesser indentation.
        PreviousLesserIndent,
        /// Moves to the previous line with greater indentation.
        PreviousGreaterIndent,
        /// Moves to the previous line with the same indentation.
        PreviousSameIndent,
        /// Moves to the next line with lesser indentation.
        NextLesserIndent,
        /// Moves to the next line with greater indentation.
        NextGreaterIndent,
        /// Moves to the next line with the same indentation.
        NextSameIndent,
    ]
);

pub fn register(editor: &mut Editor, cx: &mut Context<Vim>) {
    Vim::action(editor, cx, |vim, _: &Left, window, cx| {
        vim.motion(Motion::Left, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &WrappingLeft, window, cx| {
        vim.motion(Motion::WrappingLeft, window, cx)
    });
    Vim::action(editor, cx, |vim, action: &Down, window, cx| {
        vim.motion(
            Motion::Down {
                display_lines: action.display_lines,
            },
            window,
            cx,
        )
    });
    Vim::action(editor, cx, |vim, action: &Up, window, cx| {
        vim.motion(
            Motion::Up {
                display_lines: action.display_lines,
            },
            window,
            cx,
        )
    });
    Vim::action(editor, cx, |vim, _: &Right, window, cx| {
        vim.motion(Motion::Right, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &WrappingRight, window, cx| {
        vim.motion(Motion::WrappingRight, window, cx)
    });
    Vim::action(
        editor,
        cx,
        |vim, action: &FirstNonWhitespace, window, cx| {
            vim.motion(
                Motion::FirstNonWhitespace {
                    display_lines: action.display_lines,
                },
                window,
                cx,
            )
        },
    );
    Vim::action(editor, cx, |vim, action: &StartOfLine, window, cx| {
        vim.motion(
            Motion::StartOfLine {
                display_lines: action.display_lines,
            },
            window,
            cx,
        )
    });
    Vim::action(editor, cx, |vim, action: &MiddleOfLine, window, cx| {
        vim.motion(
            Motion::MiddleOfLine {
                display_lines: action.display_lines,
            },
            window,
            cx,
        )
    });
    Vim::action(editor, cx, |vim, action: &EndOfLine, window, cx| {
        vim.motion(
            Motion::EndOfLine {
                display_lines: action.display_lines,
            },
            window,
            cx,
        )
    });
    Vim::action(editor, cx, |vim, _: &CurrentLine, window, cx| {
        vim.motion(Motion::CurrentLine, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &StartOfParagraph, window, cx| {
        vim.motion(Motion::StartOfParagraph, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &EndOfParagraph, window, cx| {
        vim.motion(Motion::EndOfParagraph, window, cx)
    });

    Vim::action(editor, cx, |vim, _: &SentenceForward, window, cx| {
        vim.motion(Motion::SentenceForward, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &SentenceBackward, window, cx| {
        vim.motion(Motion::SentenceBackward, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &StartOfDocument, window, cx| {
        vim.motion(Motion::StartOfDocument, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &EndOfDocument, window, cx| {
        vim.motion(Motion::EndOfDocument, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &Matching, window, cx| {
        vim.motion(Motion::Matching, window, cx)
    });
    Vim::action(editor, cx, |vim, _: &GoToPercentage, window, cx| {
        vim.motion(Motion::GoToPercentage, window, cx)
    });
    Vim::action(
        editor,
        cx,
        |vim, &UnmatchedForward { char }: &UnmatchedForward, window, cx| {
            vim.motion(Motion::UnmatchedForward { char }, window, cx)
        },
    );
    Vim::action(
        editor,
        cx,
        |vim, &UnmatchedBackward { char }: &UnmatchedBackward, window, cx| {
            vim.motion(Motion::UnmatchedBackward { char }, window, cx)
        },
    );
    Vim::action(
        editor,
        cx,
        |vim, &NextWordStart { ignore_punctuation }: &NextWordStart, window, cx| {
            vim.motion(Motion::NextWordStart { ignore_punctuation }, window, cx)
        },
    );
    Vim::action(
        editor,
        cx,
        |vim, &NextWordEnd { ignore_punctuation }: &NextWordEnd, window, cx| {
            vim.motion(Motion::NextWordEnd { ignore_punctuation }, window, cx)
        },
    );
    Vim::action(
        editor,
        cx,
        |vim, &PreviousWordStart { ignore_punctuation }: &PreviousWordStart, window, cx| {
            vim.motion(Motion::PreviousWordStart { ignore_punctuation }, window, cx)
        },
    );
    Vim::action(
        editor,
        cx,
        |vim, &PreviousWordEnd { ignore_punctuation }, window, cx| {
            vim.motion(Motion::PreviousWordEnd { ignore_punctuation }, window, cx)
        },
    );
    Vim::action(
        editor,
        cx,
        |vim, &NextSubwordStart { ignore_punctuation }: &NextSubwordStart, window, cx| {
            vim.motion(Motion::NextSubwordStart { ignore_punctuation }, window, cx)
        },
    );
    Vim::action(
        editor,
        cx,
        |vim, &NextSubwordEnd { ignore_punctuation }: &NextSubwordEnd, window, cx| {
            vim.motion(Motion::NextSubwordEnd { ignore_punctuation }, window, cx)
        },
    );
    Vim::action(
        editor,
        cx,
        |vim, &PreviousSubwordStart { ignore_punctuation }: &PreviousSubwordStart, window, cx| {
            vim.motion(
                Motion::PreviousSubwordStart { ignore_punctuation },
                window,
                cx,
            )
        },
    );
    Vim::action(
        editor,
        cx,
        |vim, &PreviousSubwordEnd { ignore_punctuation }, window, cx| {
            vim.motion(
                Motion::PreviousSubwordEnd { ignore_punctuation },
                window,
                cx,
            )
        },
    );
    Vim::action(editor, cx, |vim, &NextLineStart, window, cx| {
        vim.motion(Motion::NextLineStart, window, cx)
    });
    Vim::action(editor, cx, |vim, &PreviousLineStart, window, cx| {
        vim.motion(Motion::PreviousLineStart, window, cx)
    });
    Vim::action(editor, cx, |vim, &StartOfLineDownward, window, cx| {
        vim.motion(Motion::StartOfLineDownward, window, cx)
    });
    Vim::action(editor, cx, |vim, &EndOfLineDownward, window, cx| {
        vim.motion(Motion::EndOfLineDownward, window, cx)
    });
    Vim::action(editor, cx, |vim, &GoToColumn, window, cx| {
        vim.motion(Motion::GoToColumn, window, cx)
    });

    Vim::action(editor, cx, |vim, _: &RepeatFind, window, cx| {
        if let Some(last_find) = Vim::globals(cx).last_find.clone().map(Box::new) {
            vim.motion(Motion::RepeatFind { last_find }, window, cx);
        }
    });

    Vim::action(editor, cx, |vim, _: &RepeatFindReversed, window, cx| {
        if let Some(last_find) = Vim::globals(cx).last_find.clone().map(Box::new) {
            vim.motion(Motion::RepeatFindReversed { last_find }, window, cx);
        }
    });
    Vim::action(editor, cx, |vim, &WindowTop, window, cx| {
        vim.motion(Motion::WindowTop, window, cx)
    });
    Vim::action(editor, cx, |vim, &WindowMiddle, window, cx| {
        vim.motion(Motion::WindowMiddle, window, cx)
    });
    Vim::action(editor, cx, |vim, &WindowBottom, window, cx| {
        vim.motion(Motion::WindowBottom, window, cx)
    });

    Vim::action(editor, cx, |vim, &PreviousSectionStart, window, cx| {
        vim.motion(Motion::PreviousSectionStart, window, cx)
    });
    Vim::action(editor, cx, |vim, &NextSectionStart, window, cx| {
        vim.motion(Motion::NextSectionStart, window, cx)
    });
    Vim::action(editor, cx, |vim, &PreviousSectionEnd, window, cx| {
        vim.motion(Motion::PreviousSectionEnd, window, cx)
    });
    Vim::action(editor, cx, |vim, &NextSectionEnd, window, cx| {
        vim.motion(Motion::NextSectionEnd, window, cx)
    });
    Vim::action(editor, cx, |vim, &PreviousMethodStart, window, cx| {
        vim.motion(Motion::PreviousMethodStart, window, cx)
    });
    Vim::action(editor, cx, |vim, &NextMethodStart, window, cx| {
        vim.motion(Motion::NextMethodStart, window, cx)
    });
    Vim::action(editor, cx, |vim, &PreviousMethodEnd, window, cx| {
        vim.motion(Motion::PreviousMethodEnd, window, cx)
    });
    Vim::action(editor, cx, |vim, &NextMethodEnd, window, cx| {
        vim.motion(Motion::NextMethodEnd, window, cx)
    });
    Vim::action(editor, cx, |vim, &NextComment, window, cx| {
        vim.motion(Motion::NextComment, window, cx)
    });
    Vim::action(editor, cx, |vim, &PreviousComment, window, cx| {
        vim.motion(Motion::PreviousComment, window, cx)
    });
    Vim::action(editor, cx, |vim, &PreviousLesserIndent, window, cx| {
        vim.motion(Motion::PreviousLesserIndent, window, cx)
    });
    Vim::action(editor, cx, |vim, &PreviousGreaterIndent, window, cx| {
        vim.motion(Motion::PreviousGreaterIndent, window, cx)
    });
    Vim::action(editor, cx, |vim, &PreviousSameIndent, window, cx| {
        vim.motion(Motion::PreviousSameIndent, window, cx)
    });
    Vim::action(editor, cx, |vim, &NextLesserIndent, window, cx| {
        vim.motion(Motion::NextLesserIndent, window, cx)
    });
    Vim::action(editor, cx, |vim, &NextGreaterIndent, window, cx| {
        vim.motion(Motion::NextGreaterIndent, window, cx)
    });
    Vim::action(editor, cx, |vim, &NextSameIndent, window, cx| {
        vim.motion(Motion::NextSameIndent, window, cx)
    });
}

impl Vim {
    pub(crate) fn search_motion(&mut self, m: Motion, window: &mut Window, cx: &mut Context<Self>) {
        if let Motion::ZedSearchResult {
            prior_selections, ..
        } = &m
        {
            match self.mode {
                Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                    if !prior_selections.is_empty() {
                        self.update_editor(cx, |_, editor, cx| {
                            editor.change_selections(Default::default(), window, cx, |s| {
                                s.select_ranges(prior_selections.iter().cloned())
                            })
                        });
                    }
                }
                Mode::Normal | Mode::Replace | Mode::Insert => {
                    if self.active_operator().is_none() {
                        return;
                    }
                }

                Mode::HelixNormal => {}
            }
        }

        self.motion(m, window, cx)
    }

    pub(crate) fn motion(&mut self, motion: Motion, window: &mut Window, cx: &mut Context<Self>) {
        if let Some(Operator::FindForward { .. })
        | Some(Operator::Sneak { .. })
        | Some(Operator::SneakBackward { .. })
        | Some(Operator::FindBackward { .. }) = self.active_operator()
        {
            self.pop_operator(window, cx);
        }

        let count = Vim::take_count(cx);
        let forced_motion = Vim::take_forced_motion(cx);
        let active_operator = self.active_operator();
        let mut waiting_operator: Option<Operator> = None;
        match self.mode {
            Mode::Normal | Mode::Replace | Mode::Insert => {
                if active_operator == Some(Operator::AddSurrounds { target: None }) {
                    waiting_operator = Some(Operator::AddSurrounds {
                        target: Some(SurroundsType::Motion(motion)),
                    });
                } else {
                    self.normal_motion(motion, active_operator, count, forced_motion, window, cx)
                }
            }
            Mode::Visual | Mode::VisualLine | Mode::VisualBlock => {
                self.visual_motion(motion, count, window, cx)
            }

            Mode::HelixNormal => self.helix_normal_motion(motion, count, window, cx),
        }
        self.clear_operator(window, cx);
        if let Some(operator) = waiting_operator {
            self.push_operator(operator, window, cx);
            Vim::globals(cx).pre_count = count
        }
    }
}

// Motion handling is specified here:
// https://github.com/vim/vim/blob/master/runtime/doc/motion.txt
impl Motion {
    fn default_kind(&self) -> MotionKind {
        use Motion::*;
        match self {
            Down { .. }
            | Up { .. }
            | StartOfDocument
            | EndOfDocument
            | CurrentLine
            | NextLineStart
            | PreviousLineStart
            | StartOfLineDownward
            | WindowTop
            | WindowMiddle
            | WindowBottom
            | NextSectionStart
            | NextSectionEnd
            | PreviousSectionStart
            | PreviousSectionEnd
            | NextMethodStart
            | NextMethodEnd
            | PreviousMethodStart
            | PreviousMethodEnd
            | NextComment
            | PreviousComment
            | PreviousLesserIndent
            | PreviousGreaterIndent
            | PreviousSameIndent
            | NextLesserIndent
            | NextGreaterIndent
            | NextSameIndent
            | GoToPercentage
            | Jump { line: true, .. } => MotionKind::Linewise,
            EndOfLine { .. }
            | EndOfLineDownward
            | Matching
            | FindForward { .. }
            | NextWordEnd { .. }
            | PreviousWordEnd { .. }
            | NextSubwordEnd { .. }
            | PreviousSubwordEnd { .. } => MotionKind::Inclusive,
            Left
            | WrappingLeft
            | Right
            | WrappingRight
            | StartOfLine { .. }
            | StartOfParagraph
            | EndOfParagraph
            | SentenceBackward
            | SentenceForward
            | GoToColumn
            | MiddleOfLine { .. }
            | UnmatchedForward { .. }
            | UnmatchedBackward { .. }
            | NextWordStart { .. }
            | PreviousWordStart { .. }
            | NextSubwordStart { .. }
            | PreviousSubwordStart { .. }
            | FirstNonWhitespace { .. }
            | FindBackward { .. }
            | Sneak { .. }
            | SneakBackward { .. }
            | Jump { .. }
            | ZedSearchResult { .. } => MotionKind::Exclusive,
            RepeatFind { last_find: motion } | RepeatFindReversed { last_find: motion } => {
                motion.default_kind()
            }
        }
    }

    fn skip_exclusive_special_case(&self) -> bool {
        matches!(self, Motion::WrappingLeft | Motion::WrappingRight)
    }

    pub(crate) fn push_to_jump_list(&self) -> bool {
        use Motion::*;
        match self {
            CurrentLine
            | Down { .. }
            | EndOfLine { .. }
            | EndOfLineDownward
            | FindBackward { .. }
            | FindForward { .. }
            | FirstNonWhitespace { .. }
            | GoToColumn
            | Left
            | MiddleOfLine { .. }
            | NextLineStart
            | NextSubwordEnd { .. }
            | NextSubwordStart { .. }
            | NextWordEnd { .. }
            | NextWordStart { .. }
            | PreviousLineStart
            | PreviousSubwordEnd { .. }
            | PreviousSubwordStart { .. }
            | PreviousWordEnd { .. }
            | PreviousWordStart { .. }
            | RepeatFind { .. }
            | RepeatFindReversed { .. }
            | Right
            | StartOfLine { .. }
            | StartOfLineDownward
            | Up { .. }
            | WrappingLeft
            | WrappingRight => false,
            EndOfDocument
            | EndOfParagraph
            | GoToPercentage
            | Jump { .. }
            | Matching
            | NextComment
            | NextGreaterIndent
            | NextLesserIndent
            | NextMethodEnd
            | NextMethodStart
            | NextSameIndent
            | NextSectionEnd
            | NextSectionStart
            | PreviousComment
            | PreviousGreaterIndent
            | PreviousLesserIndent
            | PreviousMethodEnd
            | PreviousMethodStart
            | PreviousSameIndent
            | PreviousSectionEnd
            | PreviousSectionStart
            | SentenceBackward
            | SentenceForward
            | Sneak { .. }
            | SneakBackward { .. }
            | StartOfDocument
            | StartOfParagraph
            | UnmatchedBackward { .. }
            | UnmatchedForward { .. }
            | WindowBottom
            | WindowMiddle
            | WindowTop
            | ZedSearchResult { .. } => true,
        }
    }

    pub fn infallible(&self) -> bool {
        use Motion::*;
        match self {
            StartOfDocument | EndOfDocument | CurrentLine => true,
            Down { .. }
            | Up { .. }
            | EndOfLine { .. }
            | MiddleOfLine { .. }
            | Matching
            | UnmatchedForward { .. }
            | UnmatchedBackward { .. }
            | FindForward { .. }
            | RepeatFind { .. }
            | Left
            | WrappingLeft
            | Right
            | WrappingRight
            | StartOfLine { .. }
            | StartOfParagraph
            | EndOfParagraph
            | SentenceBackward
            | SentenceForward
            | StartOfLineDownward
            | EndOfLineDownward
            | GoToColumn
            | GoToPercentage
            | NextWordStart { .. }
            | NextWordEnd { .. }
            | PreviousWordStart { .. }
            | PreviousWordEnd { .. }
            | NextSubwordStart { .. }
            | NextSubwordEnd { .. }
            | PreviousSubwordStart { .. }
            | PreviousSubwordEnd { .. }
            | FirstNonWhitespace { .. }
            | FindBackward { .. }
            | Sneak { .. }
            | SneakBackward { .. }
            | RepeatFindReversed { .. }
            | WindowTop
            | WindowMiddle
            | WindowBottom
            | NextLineStart
            | PreviousLineStart
            | ZedSearchResult { .. }
            | NextSectionStart
            | NextSectionEnd
            | PreviousSectionStart
            | PreviousSectionEnd
            | NextMethodStart
            | NextMethodEnd
            | PreviousMethodStart
            | PreviousMethodEnd
            | NextComment
            | PreviousComment
            | PreviousLesserIndent
            | PreviousGreaterIndent
            | PreviousSameIndent
            | NextLesserIndent
            | NextGreaterIndent
            | NextSameIndent
            | Jump { .. } => false,
        }
    }

    pub fn move_point(
        &self,
        map: &DisplaySnapshot,
        point: DisplayPoint,
        goal: SelectionGoal,
        maybe_times: Option<usize>,
        text_layout_details: &TextLayoutDetails,
    ) -> Option<(DisplayPoint, SelectionGoal)> {
        let times = maybe_times.unwrap_or(1);
        use Motion::*;
        let infallible = self.infallible();
        let (new_point, goal) = match self {
            Left => (left(map, point, times), SelectionGoal::None),
            WrappingLeft => (wrapping_left(map, point, times), SelectionGoal::None),
            Down {
                display_lines: false,
            } => up_down_buffer_rows(map, point, goal, times as isize, text_layout_details),
            Down {
                display_lines: true,
            } => down_display(map, point, goal, times, text_layout_details),
            Up {
                display_lines: false,
            } => up_down_buffer_rows(map, point, goal, 0 - times as isize, text_layout_details),
            Up {
                display_lines: true,
            } => up_display(map, point, goal, times, text_layout_details),
            Right => (right(map, point, times), SelectionGoal::None),
            WrappingRight => (wrapping_right(map, point, times), SelectionGoal::None),
            NextWordStart { ignore_punctuation } => (
                next_word_start(map, point, *ignore_punctuation, times),
                SelectionGoal::None,
            ),
            NextWordEnd { ignore_punctuation } => (
                next_word_end(map, point, *ignore_punctuation, times, true, true),
                SelectionGoal::None,
            ),
            PreviousWordStart { ignore_punctuation } => (
                previous_word_start(map, point, *ignore_punctuation, times),
                SelectionGoal::None,
            ),
            PreviousWordEnd { ignore_punctuation } => (
                previous_word_end(map, point, *ignore_punctuation, times),
                SelectionGoal::None,
            ),
            NextSubwordStart { ignore_punctuation } => (
                next_subword_start(map, point, *ignore_punctuation, times),
                SelectionGoal::None,
            ),
            NextSubwordEnd { ignore_punctuation } => (
                next_subword_end(map, point, *ignore_punctuation, times, true),
                SelectionGoal::None,
            ),
            PreviousSubwordStart { ignore_punctuation } => (
                previous_subword_start(map, point, *ignore_punctuation, times),
                SelectionGoal::None,
            ),
            PreviousSubwordEnd { ignore_punctuation } => (
                previous_subword_end(map, point, *ignore_punctuation, times),
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
            MiddleOfLine { display_lines } => (
                middle_of_line(map, *display_lines, point, maybe_times),
                SelectionGoal::None,
            ),
            EndOfLine { display_lines } => (
                end_of_line(map, *display_lines, point, times),
                SelectionGoal::None,
            ),
            SentenceBackward => (sentence_backwards(map, point, times), SelectionGoal::None),
            SentenceForward => (sentence_forwards(map, point, times), SelectionGoal::None),
            StartOfParagraph => (
                movement::start_of_paragraph(map, point, times),
                SelectionGoal::None,
            ),
            EndOfParagraph => (
                map.clip_at_line_end(movement::end_of_paragraph(map, point, times)),
                SelectionGoal::None,
            ),
            CurrentLine => (next_line_end(map, point, times), SelectionGoal::None),
            StartOfDocument => (
                start_of_document(map, point, maybe_times),
                SelectionGoal::None,
            ),
            EndOfDocument => (
                end_of_document(map, point, maybe_times),
                SelectionGoal::None,
            ),
            Matching => (matching(map, point), SelectionGoal::None),
            GoToPercentage => (go_to_percentage(map, point, times), SelectionGoal::None),
            UnmatchedForward { char } => (
                unmatched_forward(map, point, *char, times),
                SelectionGoal::None,
            ),
            UnmatchedBackward { char } => (
                unmatched_backward(map, point, *char, times),
                SelectionGoal::None,
            ),
            // t f
            FindForward {
                before,
                char,
                mode,
                smartcase,
            } => {
                return find_forward(map, point, *before, *char, times, *mode, *smartcase)
                    .map(|new_point| (new_point, SelectionGoal::None));
            }
            // T F
            FindBackward {
                after,
                char,
                mode,
                smartcase,
            } => (
                find_backward(map, point, *after, *char, times, *mode, *smartcase),
                SelectionGoal::None,
            ),
            Sneak {
                first_char,
                second_char,
                smartcase,
            } => {
                return sneak(map, point, *first_char, *second_char, times, *smartcase)
                    .map(|new_point| (new_point, SelectionGoal::None));
            }
            SneakBackward {
                first_char,
                second_char,
                smartcase,
            } => {
                return sneak_backward(map, point, *first_char, *second_char, times, *smartcase)
                    .map(|new_point| (new_point, SelectionGoal::None));
            }
            // ; -- repeat the last find done with t, f, T, F
            RepeatFind { last_find } => match **last_find {
                Motion::FindForward {
                    before,
                    char,
                    mode,
                    smartcase,
                } => {
                    let mut new_point =
                        find_forward(map, point, before, char, times, mode, smartcase);
                    if new_point == Some(point) {
                        new_point =
                            find_forward(map, point, before, char, times + 1, mode, smartcase);
                    }

                    return new_point.map(|new_point| (new_point, SelectionGoal::None));
                }

                Motion::FindBackward {
                    after,
                    char,
                    mode,
                    smartcase,
                } => {
                    let mut new_point =
                        find_backward(map, point, after, char, times, mode, smartcase);
                    if new_point == point {
                        new_point =
                            find_backward(map, point, after, char, times + 1, mode, smartcase);
                    }

                    (new_point, SelectionGoal::None)
                }
                Motion::Sneak {
                    first_char,
                    second_char,
                    smartcase,
                } => {
                    let mut new_point =
                        sneak(map, point, first_char, second_char, times, smartcase);
                    if new_point == Some(point) {
                        new_point =
                            sneak(map, point, first_char, second_char, times + 1, smartcase);
                    }

                    return new_point.map(|new_point| (new_point, SelectionGoal::None));
                }

                Motion::SneakBackward {
                    first_char,
                    second_char,
                    smartcase,
                } => {
                    let mut new_point =
                        sneak_backward(map, point, first_char, second_char, times, smartcase);
                    if new_point == Some(point) {
                        new_point = sneak_backward(
                            map,
                            point,
                            first_char,
                            second_char,
                            times + 1,
                            smartcase,
                        );
                    }

                    return new_point.map(|new_point| (new_point, SelectionGoal::None));
                }
                _ => return None,
            },
            // , -- repeat the last find done with t, f, T, F, s, S, in opposite direction
            RepeatFindReversed { last_find } => match **last_find {
                Motion::FindForward {
                    before,
                    char,
                    mode,
                    smartcase,
                } => {
                    let mut new_point =
                        find_backward(map, point, before, char, times, mode, smartcase);
                    if new_point == point {
                        new_point =
                            find_backward(map, point, before, char, times + 1, mode, smartcase);
                    }

                    (new_point, SelectionGoal::None)
                }

                Motion::FindBackward {
                    after,
                    char,
                    mode,
                    smartcase,
                } => {
                    let mut new_point =
                        find_forward(map, point, after, char, times, mode, smartcase);
                    if new_point == Some(point) {
                        new_point =
                            find_forward(map, point, after, char, times + 1, mode, smartcase);
                    }

                    return new_point.map(|new_point| (new_point, SelectionGoal::None));
                }

                Motion::Sneak {
                    first_char,
                    second_char,
                    smartcase,
                } => {
                    let mut new_point =
                        sneak_backward(map, point, first_char, second_char, times, smartcase);
                    if new_point == Some(point) {
                        new_point = sneak_backward(
                            map,
                            point,
                            first_char,
                            second_char,
                            times + 1,
                            smartcase,
                        );
                    }

                    return new_point.map(|new_point| (new_point, SelectionGoal::None));
                }

                Motion::SneakBackward {
                    first_char,
                    second_char,
                    smartcase,
                } => {
                    let mut new_point =
                        sneak(map, point, first_char, second_char, times, smartcase);
                    if new_point == Some(point) {
                        new_point =
                            sneak(map, point, first_char, second_char, times + 1, smartcase);
                    }

                    return new_point.map(|new_point| (new_point, SelectionGoal::None));
                }
                _ => return None,
            },
            NextLineStart => (next_line_start(map, point, times), SelectionGoal::None),
            PreviousLineStart => (previous_line_start(map, point, times), SelectionGoal::None),
            StartOfLineDownward => (next_line_start(map, point, times - 1), SelectionGoal::None),
            EndOfLineDownward => (last_non_whitespace(map, point, times), SelectionGoal::None),
            GoToColumn => (go_to_column(map, point, times), SelectionGoal::None),
            WindowTop => window_top(map, point, text_layout_details, times - 1),
            WindowMiddle => window_middle(map, point, text_layout_details),
            WindowBottom => window_bottom(map, point, text_layout_details, times - 1),
            Jump { line, anchor } => mark::jump_motion(map, *anchor, *line),
            ZedSearchResult { new_selections, .. } => {
                // There will be only one selection, as
                // Search::SelectNextMatch selects a single match.
                if let Some(new_selection) = new_selections.first() {
                    (
                        new_selection.start.to_display_point(map),
                        SelectionGoal::None,
                    )
                } else {
                    return None;
                }
            }
            NextSectionStart => (
                section_motion(map, point, times, Direction::Next, true),
                SelectionGoal::None,
            ),
            NextSectionEnd => (
                section_motion(map, point, times, Direction::Next, false),
                SelectionGoal::None,
            ),
            PreviousSectionStart => (
                section_motion(map, point, times, Direction::Prev, true),
                SelectionGoal::None,
            ),
            PreviousSectionEnd => (
                section_motion(map, point, times, Direction::Prev, false),
                SelectionGoal::None,
            ),

            NextMethodStart => (
                method_motion(map, point, times, Direction::Next, true),
                SelectionGoal::None,
            ),
            NextMethodEnd => (
                method_motion(map, point, times, Direction::Next, false),
                SelectionGoal::None,
            ),
            PreviousMethodStart => (
                method_motion(map, point, times, Direction::Prev, true),
                SelectionGoal::None,
            ),
            PreviousMethodEnd => (
                method_motion(map, point, times, Direction::Prev, false),
                SelectionGoal::None,
            ),
            NextComment => (
                comment_motion(map, point, times, Direction::Next),
                SelectionGoal::None,
            ),
            PreviousComment => (
                comment_motion(map, point, times, Direction::Prev),
                SelectionGoal::None,
            ),
            PreviousLesserIndent => (
                indent_motion(map, point, times, Direction::Prev, IndentType::Lesser),
                SelectionGoal::None,
            ),
            PreviousGreaterIndent => (
                indent_motion(map, point, times, Direction::Prev, IndentType::Greater),
                SelectionGoal::None,
            ),
            PreviousSameIndent => (
                indent_motion(map, point, times, Direction::Prev, IndentType::Same),
                SelectionGoal::None,
            ),
            NextLesserIndent => (
                indent_motion(map, point, times, Direction::Next, IndentType::Lesser),
                SelectionGoal::None,
            ),
            NextGreaterIndent => (
                indent_motion(map, point, times, Direction::Next, IndentType::Greater),
                SelectionGoal::None,
            ),
            NextSameIndent => (
                indent_motion(map, point, times, Direction::Next, IndentType::Same),
                SelectionGoal::None,
            ),
        };
        (new_point != point || infallible).then_some((new_point, goal))
    }

    // Get the range value after self is applied to the specified selection.
    pub fn range(
        &self,
        map: &DisplaySnapshot,
        mut selection: Selection<DisplayPoint>,
        times: Option<usize>,
        text_layout_details: &TextLayoutDetails,
        forced_motion: bool,
    ) -> Option<(Range<DisplayPoint>, MotionKind)> {
        if let Motion::ZedSearchResult {
            prior_selections,
            new_selections,
        } = self
        {
            if let Some((prior_selection, new_selection)) =
                prior_selections.first().zip(new_selections.first())
            {
                let start = prior_selection
                    .start
                    .to_display_point(map)
                    .min(new_selection.start.to_display_point(map));
                let end = new_selection
                    .end
                    .to_display_point(map)
                    .max(prior_selection.end.to_display_point(map));

                if start < end {
                    return Some((start..end, MotionKind::Exclusive));
                } else {
                    return Some((end..start, MotionKind::Exclusive));
                }
            } else {
                return None;
            }
        }
        let maybe_new_point = self.move_point(
            map,
            selection.head(),
            selection.goal,
            times,
            text_layout_details,
        );

        let (new_head, goal) = match (maybe_new_point, forced_motion) {
            (Some((p, g)), _) => Some((p, g)),
            (None, false) => None,
            (None, true) => Some((selection.head(), selection.goal)),
        }?;

        selection.set_head(new_head, goal);

        let mut kind = match (self.default_kind(), forced_motion) {
            (MotionKind::Linewise, true) => MotionKind::Exclusive,
            (MotionKind::Exclusive, true) => MotionKind::Inclusive,
            (MotionKind::Inclusive, true) => MotionKind::Exclusive,
            (kind, false) => kind,
        };

        if let Motion::NextWordStart {
            ignore_punctuation: _,
        } = self
        {
            // Another special case: When using the "w" motion in combination with an
            // operator and the last word moved over is at the end of a line, the end of
            // that word becomes the end of the operated text, not the first word in the
            // next line.
            let start = selection.start.to_point(map);
            let end = selection.end.to_point(map);
            let start_row = MultiBufferRow(selection.start.to_point(map).row);
            if end.row > start.row {
                selection.end = Point::new(start_row.0, map.buffer_snapshot.line_len(start_row))
                    .to_display_point(map);

                // a bit of a hack, we need `cw` on a blank line to not delete the newline,
                // but dw on a blank line should. The `Linewise` returned from this method
                // causes the `d` operator to include the trailing newline.
                if selection.start == selection.end {
                    return Some((selection.start..selection.end, MotionKind::Linewise));
                }
            }
        } else if kind == MotionKind::Exclusive && !self.skip_exclusive_special_case() {
            let start_point = selection.start.to_point(map);
            let mut end_point = selection.end.to_point(map);
            let mut next_point = selection.end;
            *next_point.column_mut() += 1;
            next_point = map.clip_point(next_point, Bias::Right);
            if next_point.to_point(map) == end_point && forced_motion {
                selection.end = movement::saturating_left(map, selection.end);
            }

            if end_point.row > start_point.row {
                let first_non_blank_of_start_row = map
                    .line_indent_for_buffer_row(MultiBufferRow(start_point.row))
                    .raw_len();
                // https://github.com/neovim/neovim/blob/ee143aaf65a0e662c42c636aa4a959682858b3e7/src/nvim/ops.c#L6178-L6203
                if end_point.column == 0 {
                    // If the motion is exclusive and the end of the motion is in column 1, the
                    // end of the motion is moved to the end of the previous line and the motion
                    // becomes inclusive. Example: "}" moves to the first line after a paragraph,
                    // but "d}" will not include that line.
                    //
                    // If the motion is exclusive, the end of the motion is in column 1 and the
                    // start of the motion was at or before the first non-blank in the line, the
                    // motion becomes linewise.  Example: If a paragraph begins with some blanks
                    // and you do "d}" while standing on the first non-blank, all the lines of
                    // the paragraph are deleted, including the blanks.
                    if start_point.column <= first_non_blank_of_start_row {
                        kind = MotionKind::Linewise;
                    } else {
                        kind = MotionKind::Inclusive;
                    }
                    end_point.row -= 1;
                    end_point.column = 0;
                    selection.end = map.clip_point(map.next_line_boundary(end_point).1, Bias::Left);
                } else if let Motion::EndOfParagraph = self {
                    // Special case: When using the "}" motion, it's possible
                    // that there's no blank lines after the paragraph the
                    // cursor is currently on.
                    // In this situation the `end_point.column` value will be
                    // greater than 0, so the selection doesn't actually end on
                    // the first character of a blank line. In that case, we'll
                    // want to move one column to the right, to actually include
                    // all characters of the last non-blank line.
                    selection.end = movement::saturating_right(map, selection.end)
                }
            }
        } else if kind == MotionKind::Inclusive {
            selection.end = movement::saturating_right(map, selection.end)
        }

        if kind == MotionKind::Linewise {
            selection.start = map.prev_line_boundary(selection.start.to_point(map)).1;
            selection.end = map.next_line_boundary(selection.end.to_point(map)).1;
        }
        Some((selection.start..selection.end, kind))
    }

    // Expands a selection using self for an operator
    pub fn expand_selection(
        &self,
        map: &DisplaySnapshot,
        selection: &mut Selection<DisplayPoint>,
        times: Option<usize>,
        text_layout_details: &TextLayoutDetails,
        forced_motion: bool,
    ) -> Option<MotionKind> {
        let (range, kind) = self.range(
            map,
            selection.clone(),
            times,
            text_layout_details,
            forced_motion,
        )?;
        selection.start = range.start;
        selection.end = range.end;
        Some(kind)
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

pub(crate) fn wrapping_left(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    times: usize,
) -> DisplayPoint {
    for _ in 0..times {
        point = movement::left(map, point);
        if point.is_zero() {
            break;
        }
    }
    point
}

fn wrapping_right(map: &DisplaySnapshot, mut point: DisplayPoint, times: usize) -> DisplayPoint {
    for _ in 0..times {
        point = wrapping_right_single(map, point);
        if point == map.max_point() {
            break;
        }
    }
    point
}

fn wrapping_right_single(map: &DisplaySnapshot, point: DisplayPoint) -> DisplayPoint {
    let mut next_point = point;
    *next_point.column_mut() += 1;
    next_point = map.clip_point(next_point, Bias::Right);
    if next_point == point {
        if next_point.row() == map.max_point().row() {
            next_point
        } else {
            DisplayPoint::new(next_point.row().next_row(), 0)
        }
    } else {
        next_point
    }
}

pub(crate) fn start_of_relative_buffer_row(
    map: &DisplaySnapshot,
    point: DisplayPoint,
    times: isize,
) -> DisplayPoint {
    let start = map.display_point_to_fold_point(point, Bias::Left);
    let target = start.row() as isize + times;
    let new_row = (target.max(0) as u32).min(map.fold_snapshot.max_point().row());

    map.clip_point(
        map.fold_point_to_display_point(
            map.fold_snapshot
                .clip_point(FoldPoint::new(new_row, 0), Bias::Right),
        ),
        Bias::Right,
    )
}

fn up_down_buffer_rows(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    mut goal: SelectionGoal,
    mut times: isize,
    text_layout_details: &TextLayoutDetails,
) -> (DisplayPoint, SelectionGoal) {
    let bias = if times < 0 { Bias::Left } else { Bias::Right };

    while map.is_folded_buffer_header(point.row()) {
        if times < 0 {
            (point, _) = movement::up(map, point, goal, true, text_layout_details);
            times += 1;
        } else if times > 0 {
            (point, _) = movement::down(map, point, goal, true, text_layout_details);
            times -= 1;
        } else {
            break;
        }
    }

    let start = map.display_point_to_fold_point(point, Bias::Left);
    let begin_folded_line = map.fold_point_to_display_point(
        map.fold_snapshot
            .clip_point(FoldPoint::new(start.row(), 0), Bias::Left),
    );
    let select_nth_wrapped_row = point.row().0 - begin_folded_line.row().0;

    let (goal_wrap, goal_x) = match goal {
        SelectionGoal::WrappedHorizontalPosition((row, x)) => (row, x),
        SelectionGoal::HorizontalRange { end, .. } => (select_nth_wrapped_row, end),
        SelectionGoal::HorizontalPosition(x) => (select_nth_wrapped_row, x),
        _ => {
            let x = map.x_for_display_point(point, text_layout_details);
            goal = SelectionGoal::WrappedHorizontalPosition((select_nth_wrapped_row, x.0));
            (select_nth_wrapped_row, x.0)
        }
    };

    let target = start.row() as isize + times;
    let new_row = (target.max(0) as u32).min(map.fold_snapshot.max_point().row());

    let mut begin_folded_line = map.fold_point_to_display_point(
        map.fold_snapshot
            .clip_point(FoldPoint::new(new_row, 0), bias),
    );

    let mut i = 0;
    while i < goal_wrap && begin_folded_line.row() < map.max_point().row() {
        let next_folded_line = DisplayPoint::new(begin_folded_line.row().next_row(), 0);
        if map
            .display_point_to_fold_point(next_folded_line, bias)
            .row()
            == new_row
        {
            i += 1;
            begin_folded_line = next_folded_line;
        } else {
            break;
        }
    }

    let new_col = if i == goal_wrap {
        map.display_column_for_x(begin_folded_line.row(), px(goal_x), text_layout_details)
    } else {
        map.line_len(begin_folded_line.row())
    };

    (
        map.clip_point(DisplayPoint::new(begin_folded_line.row(), new_col), bias),
        goal,
    )
}

fn down_display(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    mut goal: SelectionGoal,
    times: usize,
    text_layout_details: &TextLayoutDetails,
) -> (DisplayPoint, SelectionGoal) {
    for _ in 0..times {
        (point, goal) = movement::down(map, point, goal, true, text_layout_details);
    }

    (point, goal)
}

fn up_display(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    mut goal: SelectionGoal,
    times: usize,
    text_layout_details: &TextLayoutDetails,
) -> (DisplayPoint, SelectionGoal) {
    for _ in 0..times {
        (point, goal) = movement::up(map, point, goal, true, text_layout_details);
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

pub(crate) fn next_char(
    map: &DisplaySnapshot,
    point: DisplayPoint,
    allow_cross_newline: bool,
) -> DisplayPoint {
    let mut new_point = point;
    let mut max_column = map.line_len(new_point.row());
    if !allow_cross_newline {
        max_column -= 1;
    }
    if new_point.column() < max_column {
        *new_point.column_mut() += 1;
    } else if new_point < map.max_point() && allow_cross_newline {
        *new_point.row_mut() += 1;
        *new_point.column_mut() = 0;
    }
    map.clip_ignoring_line_ends(new_point, Bias::Right)
}

pub(crate) fn next_word_start(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    ignore_punctuation: bool,
    times: usize,
) -> DisplayPoint {
    let classifier = map
        .buffer_snapshot
        .char_classifier_at(point.to_point(map))
        .ignore_punctuation(ignore_punctuation);
    for _ in 0..times {
        let mut crossed_newline = false;
        let new_point = movement::find_boundary(map, point, FindRange::MultiLine, |left, right| {
            let left_kind = classifier.kind(left);
            let right_kind = classifier.kind(right);
            let at_newline = right == '\n';

            let found = (left_kind != right_kind && right_kind != CharKind::Whitespace)
                || at_newline && crossed_newline
                || at_newline && left == '\n'; // Prevents skipping repeated empty lines

            crossed_newline |= at_newline;
            found
        });
        if point == new_point {
            break;
        }
        point = new_point;
    }
    point
}

pub(crate) fn next_word_end(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    ignore_punctuation: bool,
    times: usize,
    allow_cross_newline: bool,
    always_advance: bool,
) -> DisplayPoint {
    let classifier = map
        .buffer_snapshot
        .char_classifier_at(point.to_point(map))
        .ignore_punctuation(ignore_punctuation);
    for _ in 0..times {
        let mut need_next_char = false;
        let new_point = if always_advance {
            next_char(map, point, allow_cross_newline)
        } else {
            point
        };
        let new_point = movement::find_boundary_exclusive(
            map,
            new_point,
            FindRange::MultiLine,
            |left, right| {
                let left_kind = classifier.kind(left);
                let right_kind = classifier.kind(right);
                let at_newline = right == '\n';

                if !allow_cross_newline && at_newline {
                    need_next_char = true;
                    return true;
                }

                left_kind != right_kind && left_kind != CharKind::Whitespace
            },
        );
        let new_point = if need_next_char {
            next_char(map, new_point, true)
        } else {
            new_point
        };
        let new_point = map.clip_point(new_point, Bias::Left);
        if point == new_point {
            break;
        }
        point = new_point;
    }
    point
}

fn previous_word_start(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    ignore_punctuation: bool,
    times: usize,
) -> DisplayPoint {
    let classifier = map
        .buffer_snapshot
        .char_classifier_at(point.to_point(map))
        .ignore_punctuation(ignore_punctuation);
    for _ in 0..times {
        // This works even though find_preceding_boundary is called for every character in the line containing
        // cursor because the newline is checked only once.
        let new_point = movement::find_preceding_boundary_display_point(
            map,
            point,
            FindRange::MultiLine,
            |left, right| {
                let left_kind = classifier.kind(left);
                let right_kind = classifier.kind(right);

                (left_kind != right_kind && !right.is_whitespace()) || left == '\n'
            },
        );
        if point == new_point {
            break;
        }
        point = new_point;
    }
    point
}

fn previous_word_end(
    map: &DisplaySnapshot,
    point: DisplayPoint,
    ignore_punctuation: bool,
    times: usize,
) -> DisplayPoint {
    let classifier = map
        .buffer_snapshot
        .char_classifier_at(point.to_point(map))
        .ignore_punctuation(ignore_punctuation);
    let mut point = point.to_point(map);

    if point.column < map.buffer_snapshot.line_len(MultiBufferRow(point.row))
        && let Some(ch) = map.buffer_snapshot.chars_at(point).next()
    {
        point.column += ch.len_utf8() as u32;
    }
    for _ in 0..times {
        let new_point = movement::find_preceding_boundary_point(
            &map.buffer_snapshot,
            point,
            FindRange::MultiLine,
            |left, right| {
                let left_kind = classifier.kind(left);
                let right_kind = classifier.kind(right);
                match (left_kind, right_kind) {
                    (CharKind::Punctuation, CharKind::Whitespace)
                    | (CharKind::Punctuation, CharKind::Word)
                    | (CharKind::Word, CharKind::Whitespace)
                    | (CharKind::Word, CharKind::Punctuation) => true,
                    (CharKind::Whitespace, CharKind::Whitespace) => left == '\n' && right == '\n',
                    _ => false,
                }
            },
        );
        if new_point == point {
            break;
        }
        point = new_point;
    }
    movement::saturating_left(map, point.to_display_point(map))
}

fn next_subword_start(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    ignore_punctuation: bool,
    times: usize,
) -> DisplayPoint {
    let classifier = map
        .buffer_snapshot
        .char_classifier_at(point.to_point(map))
        .ignore_punctuation(ignore_punctuation);
    for _ in 0..times {
        let mut crossed_newline = false;
        let new_point = movement::find_boundary(map, point, FindRange::MultiLine, |left, right| {
            let left_kind = classifier.kind(left);
            let right_kind = classifier.kind(right);
            let at_newline = right == '\n';

            let is_word_start = (left_kind != right_kind) && !left.is_alphanumeric();
            let is_subword_start =
                left == '_' && right != '_' || left.is_lowercase() && right.is_uppercase();

            let found = (!right.is_whitespace() && (is_word_start || is_subword_start))
                || at_newline && crossed_newline
                || at_newline && left == '\n'; // Prevents skipping repeated empty lines

            crossed_newline |= at_newline;
            found
        });
        if point == new_point {
            break;
        }
        point = new_point;
    }
    point
}

pub(crate) fn next_subword_end(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    ignore_punctuation: bool,
    times: usize,
    allow_cross_newline: bool,
) -> DisplayPoint {
    let classifier = map
        .buffer_snapshot
        .char_classifier_at(point.to_point(map))
        .ignore_punctuation(ignore_punctuation);
    for _ in 0..times {
        let new_point = next_char(map, point, allow_cross_newline);

        let mut crossed_newline = false;
        let mut need_backtrack = false;
        let new_point =
            movement::find_boundary(map, new_point, FindRange::MultiLine, |left, right| {
                let left_kind = classifier.kind(left);
                let right_kind = classifier.kind(right);
                let at_newline = right == '\n';

                if !allow_cross_newline && at_newline {
                    return true;
                }

                let is_word_end = (left_kind != right_kind) && !right.is_alphanumeric();
                let is_subword_end =
                    left != '_' && right == '_' || left.is_lowercase() && right.is_uppercase();

                let found = !left.is_whitespace() && !at_newline && (is_word_end || is_subword_end);

                if found && (is_word_end || is_subword_end) {
                    need_backtrack = true;
                }

                crossed_newline |= at_newline;
                found
            });
        let mut new_point = map.clip_point(new_point, Bias::Left);
        if need_backtrack {
            *new_point.column_mut() -= 1;
        }
        let new_point = map.clip_point(new_point, Bias::Left);
        if point == new_point {
            break;
        }
        point = new_point;
    }
    point
}

fn previous_subword_start(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    ignore_punctuation: bool,
    times: usize,
) -> DisplayPoint {
    let classifier = map
        .buffer_snapshot
        .char_classifier_at(point.to_point(map))
        .ignore_punctuation(ignore_punctuation);
    for _ in 0..times {
        let mut crossed_newline = false;
        // This works even though find_preceding_boundary is called for every character in the line containing
        // cursor because the newline is checked only once.
        let new_point = movement::find_preceding_boundary_display_point(
            map,
            point,
            FindRange::MultiLine,
            |left, right| {
                let left_kind = classifier.kind(left);
                let right_kind = classifier.kind(right);
                let at_newline = right == '\n';

                let is_word_start = (left_kind != right_kind) && !left.is_alphanumeric();
                let is_subword_start =
                    left == '_' && right != '_' || left.is_lowercase() && right.is_uppercase();

                let found = (!right.is_whitespace() && (is_word_start || is_subword_start))
                    || at_newline && crossed_newline
                    || at_newline && left == '\n'; // Prevents skipping repeated empty lines

                crossed_newline |= at_newline;

                found
            },
        );
        if point == new_point {
            break;
        }
        point = new_point;
    }
    point
}

fn previous_subword_end(
    map: &DisplaySnapshot,
    point: DisplayPoint,
    ignore_punctuation: bool,
    times: usize,
) -> DisplayPoint {
    let classifier = map
        .buffer_snapshot
        .char_classifier_at(point.to_point(map))
        .ignore_punctuation(ignore_punctuation);
    let mut point = point.to_point(map);

    if point.column < map.buffer_snapshot.line_len(MultiBufferRow(point.row))
        && let Some(ch) = map.buffer_snapshot.chars_at(point).next()
    {
        point.column += ch.len_utf8() as u32;
    }
    for _ in 0..times {
        let new_point = movement::find_preceding_boundary_point(
            &map.buffer_snapshot,
            point,
            FindRange::MultiLine,
            |left, right| {
                let left_kind = classifier.kind(left);
                let right_kind = classifier.kind(right);

                let is_subword_end =
                    left != '_' && right == '_' || left.is_lowercase() && right.is_uppercase();

                if is_subword_end {
                    return true;
                }

                match (left_kind, right_kind) {
                    (CharKind::Word, CharKind::Whitespace)
                    | (CharKind::Word, CharKind::Punctuation) => true,
                    (CharKind::Whitespace, CharKind::Whitespace) => left == '\n' && right == '\n',
                    _ => false,
                }
            },
        );
        if new_point == point {
            break;
        }
        point = new_point;
    }
    movement::saturating_left(map, point.to_display_point(map))
}

pub(crate) fn first_non_whitespace(
    map: &DisplaySnapshot,
    display_lines: bool,
    from: DisplayPoint,
) -> DisplayPoint {
    let mut start_offset = start_of_line(map, display_lines, from).to_offset(map, Bias::Left);
    let classifier = map.buffer_snapshot.char_classifier_at(from.to_point(map));
    for (ch, offset) in map.buffer_chars_at(start_offset) {
        if ch == '\n' {
            return from;
        }

        start_offset = offset;

        if classifier.kind(ch) != CharKind::Whitespace {
            break;
        }
    }

    start_offset.to_display_point(map)
}

pub(crate) fn last_non_whitespace(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    count: usize,
) -> DisplayPoint {
    let mut end_of_line = end_of_line(map, false, from, count).to_offset(map, Bias::Left);
    let classifier = map.buffer_snapshot.char_classifier_at(from.to_point(map));

    // NOTE: depending on clip_at_line_end we may already be one char back from the end.
    if let Some((ch, _)) = map.buffer_chars_at(end_of_line).next()
        && classifier.kind(ch) != CharKind::Whitespace
    {
        return end_of_line.to_display_point(map);
    }

    for (ch, offset) in map.reverse_buffer_chars_at(end_of_line) {
        if ch == '\n' {
            break;
        }
        end_of_line = offset;
        if classifier.kind(ch) != CharKind::Whitespace || ch == '\n' {
            break;
        }
    }

    end_of_line.to_display_point(map)
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

pub(crate) fn middle_of_line(
    map: &DisplaySnapshot,
    display_lines: bool,
    point: DisplayPoint,
    times: Option<usize>,
) -> DisplayPoint {
    let percent = if let Some(times) = times.filter(|&t| t <= 100) {
        times as f64 / 100.
    } else {
        0.5
    };
    if display_lines {
        map.clip_point(
            DisplayPoint::new(
                point.row(),
                (map.line_len(point.row()) as f64 * percent) as u32,
            ),
            Bias::Left,
        )
    } else {
        let mut buffer_point = point.to_point(map);
        buffer_point.column = (map
            .buffer_snapshot
            .line_len(MultiBufferRow(buffer_point.row)) as f64
            * percent) as u32;

        map.clip_point(buffer_point.to_display_point(map), Bias::Left)
    }
}

pub(crate) fn end_of_line(
    map: &DisplaySnapshot,
    display_lines: bool,
    mut point: DisplayPoint,
    times: usize,
) -> DisplayPoint {
    if times > 1 {
        point = start_of_relative_buffer_row(map, point, times as isize - 1);
    }
    if display_lines {
        map.clip_point(
            DisplayPoint::new(point.row(), map.line_len(point.row())),
            Bias::Left,
        )
    } else {
        map.clip_point(map.next_line_boundary(point.to_point(map)).1, Bias::Left)
    }
}

pub(crate) fn sentence_backwards(
    map: &DisplaySnapshot,
    point: DisplayPoint,
    mut times: usize,
) -> DisplayPoint {
    let mut start = point.to_point(map).to_offset(&map.buffer_snapshot);
    let mut chars = map.reverse_buffer_chars_at(start).peekable();

    let mut was_newline = map
        .buffer_chars_at(start)
        .next()
        .is_some_and(|(c, _)| c == '\n');

    while let Some((ch, offset)) = chars.next() {
        let start_of_next_sentence = if was_newline && ch == '\n' {
            Some(offset + ch.len_utf8())
        } else if ch == '\n' && chars.peek().is_some_and(|(c, _)| *c == '\n') {
            Some(next_non_blank(map, offset + ch.len_utf8()))
        } else if ch == '.' || ch == '?' || ch == '!' {
            start_of_next_sentence(map, offset + ch.len_utf8())
        } else {
            None
        };

        if let Some(start_of_next_sentence) = start_of_next_sentence {
            if start_of_next_sentence < start {
                times = times.saturating_sub(1);
            }
            if times == 0 || offset == 0 {
                return map.clip_point(
                    start_of_next_sentence
                        .to_offset(&map.buffer_snapshot)
                        .to_display_point(map),
                    Bias::Left,
                );
            }
        }
        if was_newline {
            start = offset;
        }
        was_newline = ch == '\n';
    }

    DisplayPoint::zero()
}

pub(crate) fn sentence_forwards(
    map: &DisplaySnapshot,
    point: DisplayPoint,
    mut times: usize,
) -> DisplayPoint {
    let start = point.to_point(map).to_offset(&map.buffer_snapshot);
    let mut chars = map.buffer_chars_at(start).peekable();

    let mut was_newline = map
        .reverse_buffer_chars_at(start)
        .next()
        .is_some_and(|(c, _)| c == '\n')
        && chars.peek().is_some_and(|(c, _)| *c == '\n');

    while let Some((ch, offset)) = chars.next() {
        if was_newline && ch == '\n' {
            continue;
        }
        let start_of_next_sentence = if was_newline {
            Some(next_non_blank(map, offset))
        } else if ch == '\n' && chars.peek().is_some_and(|(c, _)| *c == '\n') {
            Some(next_non_blank(map, offset + ch.len_utf8()))
        } else if ch == '.' || ch == '?' || ch == '!' {
            start_of_next_sentence(map, offset + ch.len_utf8())
        } else {
            None
        };

        if let Some(start_of_next_sentence) = start_of_next_sentence {
            times = times.saturating_sub(1);
            if times == 0 {
                return map.clip_point(
                    start_of_next_sentence
                        .to_offset(&map.buffer_snapshot)
                        .to_display_point(map),
                    Bias::Right,
                );
            }
        }

        was_newline = ch == '\n' && chars.peek().is_some_and(|(c, _)| *c == '\n');
    }

    map.max_point()
}

fn next_non_blank(map: &DisplaySnapshot, start: usize) -> usize {
    for (c, o) in map.buffer_chars_at(start) {
        if c == '\n' || !c.is_whitespace() {
            return o;
        }
    }

    map.buffer_snapshot.len()
}

// given the offset after a ., !, or ? find the start of the next sentence.
// if this is not a sentence boundary, returns None.
fn start_of_next_sentence(map: &DisplaySnapshot, end_of_sentence: usize) -> Option<usize> {
    let chars = map.buffer_chars_at(end_of_sentence);
    let mut seen_space = false;

    for (char, offset) in chars {
        if !seen_space && (char == ')' || char == ']' || char == '"' || char == '\'') {
            continue;
        }

        if char == '\n' && seen_space {
            return Some(offset);
        } else if char.is_whitespace() {
            seen_space = true;
        } else if seen_space {
            return Some(offset);
        } else {
            return None;
        }
    }

    Some(map.buffer_snapshot.len())
}

fn go_to_line(map: &DisplaySnapshot, display_point: DisplayPoint, line: usize) -> DisplayPoint {
    let point = map.display_point_to_point(display_point, Bias::Left);
    let Some(mut excerpt) = map.buffer_snapshot.excerpt_containing(point..point) else {
        return display_point;
    };
    let offset = excerpt.buffer().point_to_offset(
        excerpt
            .buffer()
            .clip_point(Point::new((line - 1) as u32, point.column), Bias::Left),
    );
    let buffer_range = excerpt.buffer_range();
    if offset >= buffer_range.start && offset <= buffer_range.end {
        let point = map
            .buffer_snapshot
            .offset_to_point(excerpt.map_offset_from_buffer(offset));
        return map.clip_point(map.point_to_display_point(point, Bias::Left), Bias::Left);
    }
    let mut last_position = None;
    for (excerpt, buffer, range) in map.buffer_snapshot.excerpts() {
        let excerpt_range = language::ToOffset::to_offset(&range.context.start, buffer)
            ..language::ToOffset::to_offset(&range.context.end, buffer);
        if offset >= excerpt_range.start && offset <= excerpt_range.end {
            let text_anchor = buffer.anchor_after(offset);
            let anchor = Anchor::in_buffer(excerpt, buffer.remote_id(), text_anchor);
            return anchor.to_display_point(map);
        } else if offset <= excerpt_range.start {
            let anchor = Anchor::in_buffer(excerpt, buffer.remote_id(), range.context.start);
            return anchor.to_display_point(map);
        } else {
            last_position = Some(Anchor::in_buffer(
                excerpt,
                buffer.remote_id(),
                range.context.end,
            ));
        }
    }

    let mut last_point = last_position.unwrap().to_point(&map.buffer_snapshot);
    last_point.column = point.column;

    map.clip_point(
        map.point_to_display_point(
            map.buffer_snapshot.clip_point(point, Bias::Left),
            Bias::Left,
        ),
        Bias::Left,
    )
}

fn start_of_document(
    map: &DisplaySnapshot,
    display_point: DisplayPoint,
    maybe_times: Option<usize>,
) -> DisplayPoint {
    if let Some(times) = maybe_times {
        return go_to_line(map, display_point, times);
    }

    let point = map.display_point_to_point(display_point, Bias::Left);
    let mut first_point = Point::zero();
    first_point.column = point.column;

    map.clip_point(
        map.point_to_display_point(
            map.buffer_snapshot.clip_point(first_point, Bias::Left),
            Bias::Left,
        ),
        Bias::Left,
    )
}

fn end_of_document(
    map: &DisplaySnapshot,
    display_point: DisplayPoint,
    maybe_times: Option<usize>,
) -> DisplayPoint {
    if let Some(times) = maybe_times {
        return go_to_line(map, display_point, times);
    };
    let point = map.display_point_to_point(display_point, Bias::Left);
    let mut last_point = map.buffer_snapshot.max_point();
    last_point.column = point.column;

    map.clip_point(
        map.point_to_display_point(
            map.buffer_snapshot.clip_point(last_point, Bias::Left),
            Bias::Left,
        ),
        Bias::Left,
    )
}

fn matching_tag(map: &DisplaySnapshot, head: DisplayPoint) -> Option<DisplayPoint> {
    let inner = crate::object::surrounding_html_tag(map, head, head..head, false)?;
    let outer = crate::object::surrounding_html_tag(map, head, head..head, true)?;

    if head > outer.start && head < inner.start {
        let mut offset = inner.end.to_offset(map, Bias::Left);
        for c in map.buffer_snapshot.chars_at(offset) {
            if c == '/' || c == '\n' || c == '>' {
                return Some(offset.to_display_point(map));
            }
            offset += c.len_utf8();
        }
    } else {
        let mut offset = outer.start.to_offset(map, Bias::Left);
        for c in map.buffer_snapshot.chars_at(offset) {
            offset += c.len_utf8();
            if c == '<' || c == '\n' {
                return Some(offset.to_display_point(map));
            }
        }
    }

    None
}

fn matching(map: &DisplaySnapshot, display_point: DisplayPoint) -> DisplayPoint {
    // https://github.com/vim/vim/blob/1d87e11a1ef201b26ed87585fba70182ad0c468a/runtime/doc/motion.txt#L1200
    let display_point = map.clip_at_line_end(display_point);
    let point = display_point.to_point(map);
    let offset = point.to_offset(&map.buffer_snapshot);

    // Ensure the range is contained by the current line.
    let mut line_end = map.next_line_boundary(point).0;
    if line_end == point {
        line_end = map.max_point().to_point(map);
    }

    if let Some((opening_range, closing_range)) = map
        .buffer_snapshot
        .innermost_enclosing_bracket_ranges(offset..offset, None)
    {
        if opening_range.contains(&offset) {
            return closing_range.start.to_display_point(map);
        } else if closing_range.contains(&offset) {
            return opening_range.start.to_display_point(map);
        }
    }

    let line_range = map.prev_line_boundary(point).0..line_end;
    let visible_line_range =
        line_range.start..Point::new(line_range.end.row, line_range.end.column.saturating_sub(1));
    let ranges = map.buffer_snapshot.bracket_ranges(visible_line_range);
    if let Some(ranges) = ranges {
        let line_range = line_range.start.to_offset(&map.buffer_snapshot)
            ..line_range.end.to_offset(&map.buffer_snapshot);
        let mut closest_pair_destination = None;
        let mut closest_distance = usize::MAX;

        for (open_range, close_range) in ranges {
            if map.buffer_snapshot.chars_at(open_range.start).next() == Some('<') {
                if offset > open_range.start && offset < close_range.start {
                    let mut chars = map.buffer_snapshot.chars_at(close_range.start);
                    if (Some('/'), Some('>')) == (chars.next(), chars.next()) {
                        return display_point;
                    }
                    if let Some(tag) = matching_tag(map, display_point) {
                        return tag;
                    }
                } else if close_range.contains(&offset) {
                    return open_range.start.to_display_point(map);
                } else if open_range.contains(&offset) {
                    return (close_range.end - 1).to_display_point(map);
                }
            }

            if (open_range.contains(&offset) || open_range.start >= offset)
                && line_range.contains(&open_range.start)
            {
                let distance = open_range.start.saturating_sub(offset);
                if distance < closest_distance {
                    closest_pair_destination = Some(close_range.start);
                    closest_distance = distance;
                    continue;
                }
            }

            if (close_range.contains(&offset) || close_range.start >= offset)
                && line_range.contains(&close_range.start)
            {
                let distance = close_range.start.saturating_sub(offset);
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

// Go to {count} percentage in the file, on the first
// non-blank in the line linewise.  To compute the new
// line number this formula is used:
// ({count} * number-of-lines + 99) / 100
//
// https://neovim.io/doc/user/motion.html#N%25
fn go_to_percentage(map: &DisplaySnapshot, point: DisplayPoint, count: usize) -> DisplayPoint {
    let total_lines = map.buffer_snapshot.max_point().row + 1;
    let target_line = (count * total_lines as usize).div_ceil(100);
    let target_point = DisplayPoint::new(
        DisplayRow(target_line.saturating_sub(1) as u32),
        point.column(),
    );
    map.clip_point(target_point, Bias::Left)
}

fn unmatched_forward(
    map: &DisplaySnapshot,
    mut display_point: DisplayPoint,
    char: char,
    times: usize,
) -> DisplayPoint {
    for _ in 0..times {
        // https://github.com/vim/vim/blob/1d87e11a1ef201b26ed87585fba70182ad0c468a/runtime/doc/motion.txt#L1245
        let point = display_point.to_point(map);
        let offset = point.to_offset(&map.buffer_snapshot);

        let ranges = map.buffer_snapshot.enclosing_bracket_ranges(point..point);
        let Some(ranges) = ranges else { break };
        let mut closest_closing_destination = None;
        let mut closest_distance = usize::MAX;

        for (_, close_range) in ranges {
            if close_range.start > offset {
                let mut chars = map.buffer_snapshot.chars_at(close_range.start);
                if Some(char) == chars.next() {
                    let distance = close_range.start - offset;
                    if distance < closest_distance {
                        closest_closing_destination = Some(close_range.start);
                        closest_distance = distance;
                        continue;
                    }
                }
            }
        }

        let new_point = closest_closing_destination
            .map(|destination| destination.to_display_point(map))
            .unwrap_or(display_point);
        if new_point == display_point {
            break;
        }
        display_point = new_point;
    }
    display_point
}

fn unmatched_backward(
    map: &DisplaySnapshot,
    mut display_point: DisplayPoint,
    char: char,
    times: usize,
) -> DisplayPoint {
    for _ in 0..times {
        // https://github.com/vim/vim/blob/1d87e11a1ef201b26ed87585fba70182ad0c468a/runtime/doc/motion.txt#L1239
        let point = display_point.to_point(map);
        let offset = point.to_offset(&map.buffer_snapshot);

        let ranges = map.buffer_snapshot.enclosing_bracket_ranges(point..point);
        let Some(ranges) = ranges else {
            break;
        };

        let mut closest_starting_destination = None;
        let mut closest_distance = usize::MAX;

        for (start_range, _) in ranges {
            if start_range.start < offset {
                let mut chars = map.buffer_snapshot.chars_at(start_range.start);
                if Some(char) == chars.next() {
                    let distance = offset - start_range.start;
                    if distance < closest_distance {
                        closest_starting_destination = Some(start_range.start);
                        closest_distance = distance;
                        continue;
                    }
                }
            }
        }

        let new_point = closest_starting_destination
            .map(|destination| destination.to_display_point(map))
            .unwrap_or(display_point);
        if new_point == display_point {
            break;
        } else {
            display_point = new_point;
        }
    }
    display_point
}

fn find_forward(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    before: bool,
    target: char,
    times: usize,
    mode: FindRange,
    smartcase: bool,
) -> Option<DisplayPoint> {
    let mut to = from;
    let mut found = false;

    for _ in 0..times {
        found = false;
        let new_to = find_boundary(map, to, mode, |_, right| {
            found = is_character_match(target, right, smartcase);
            found
        });
        if to == new_to {
            break;
        }
        to = new_to;
    }

    if found {
        if before && to.column() > 0 {
            *to.column_mut() -= 1;
            Some(map.clip_point(to, Bias::Left))
        } else if before && to.row().0 > 0 {
            *to.row_mut() -= 1;
            *to.column_mut() = map.line(to.row()).len() as u32;
            Some(map.clip_point(to, Bias::Left))
        } else {
            Some(to)
        }
    } else {
        None
    }
}

fn find_backward(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    after: bool,
    target: char,
    times: usize,
    mode: FindRange,
    smartcase: bool,
) -> DisplayPoint {
    let mut to = from;

    for _ in 0..times {
        let new_to = find_preceding_boundary_display_point(map, to, mode, |_, right| {
            is_character_match(target, right, smartcase)
        });
        if to == new_to {
            break;
        }
        to = new_to;
    }

    let next = map.buffer_snapshot.chars_at(to.to_point(map)).next();
    if next.is_some() && is_character_match(target, next.unwrap(), smartcase) {
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

/// Returns true if one char is equal to the other or its uppercase variant (if smartcase is true).
pub fn is_character_match(target: char, other: char, smartcase: bool) -> bool {
    if smartcase {
        if target.is_uppercase() {
            target == other
        } else {
            target == other.to_ascii_lowercase()
        }
    } else {
        target == other
    }
}

fn sneak(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    first_target: char,
    second_target: char,
    times: usize,
    smartcase: bool,
) -> Option<DisplayPoint> {
    let mut to = from;
    let mut found = false;

    for _ in 0..times {
        found = false;
        let new_to = find_boundary(
            map,
            movement::right(map, to),
            FindRange::MultiLine,
            |left, right| {
                found = is_character_match(first_target, left, smartcase)
                    && is_character_match(second_target, right, smartcase);
                found
            },
        );
        if to == new_to {
            break;
        }
        to = new_to;
    }

    if found {
        Some(movement::left(map, to))
    } else {
        None
    }
}

fn sneak_backward(
    map: &DisplaySnapshot,
    from: DisplayPoint,
    first_target: char,
    second_target: char,
    times: usize,
    smartcase: bool,
) -> Option<DisplayPoint> {
    let mut to = from;
    let mut found = false;

    for _ in 0..times {
        found = false;
        let new_to =
            find_preceding_boundary_display_point(map, to, FindRange::MultiLine, |left, right| {
                found = is_character_match(first_target, left, smartcase)
                    && is_character_match(second_target, right, smartcase);
                found
            });
        if to == new_to {
            break;
        }
        to = new_to;
    }

    if found {
        Some(movement::left(map, to))
    } else {
        None
    }
}

fn next_line_start(map: &DisplaySnapshot, point: DisplayPoint, times: usize) -> DisplayPoint {
    let correct_line = start_of_relative_buffer_row(map, point, times as isize);
    first_non_whitespace(map, false, correct_line)
}

fn previous_line_start(map: &DisplaySnapshot, point: DisplayPoint, times: usize) -> DisplayPoint {
    let correct_line = start_of_relative_buffer_row(map, point, -(times as isize));
    first_non_whitespace(map, false, correct_line)
}

fn go_to_column(map: &DisplaySnapshot, point: DisplayPoint, times: usize) -> DisplayPoint {
    let correct_line = start_of_relative_buffer_row(map, point, 0);
    right(map, correct_line, times.saturating_sub(1))
}

pub(crate) fn next_line_end(
    map: &DisplaySnapshot,
    mut point: DisplayPoint,
    times: usize,
) -> DisplayPoint {
    if times > 1 {
        point = start_of_relative_buffer_row(map, point, times as isize - 1);
    }
    end_of_line(map, false, point, 1)
}

fn window_top(
    map: &DisplaySnapshot,
    point: DisplayPoint,
    text_layout_details: &TextLayoutDetails,
    mut times: usize,
) -> (DisplayPoint, SelectionGoal) {
    let first_visible_line = text_layout_details
        .scroll_anchor
        .anchor
        .to_display_point(map);

    if first_visible_line.row() != DisplayRow(0)
        && text_layout_details.vertical_scroll_margin as usize > times
    {
        times = text_layout_details.vertical_scroll_margin.ceil() as usize;
    }

    if let Some(visible_rows) = text_layout_details.visible_rows {
        let bottom_row = first_visible_line.row().0 + visible_rows as u32;
        let new_row = (first_visible_line.row().0 + (times as u32))
            .min(bottom_row)
            .min(map.max_point().row().0);
        let new_col = point.column().min(map.line_len(first_visible_line.row()));

        let new_point = DisplayPoint::new(DisplayRow(new_row), new_col);
        (map.clip_point(new_point, Bias::Left), SelectionGoal::None)
    } else {
        let new_row =
            DisplayRow((first_visible_line.row().0 + (times as u32)).min(map.max_point().row().0));
        let new_col = point.column().min(map.line_len(first_visible_line.row()));

        let new_point = DisplayPoint::new(new_row, new_col);
        (map.clip_point(new_point, Bias::Left), SelectionGoal::None)
    }
}

fn window_middle(
    map: &DisplaySnapshot,
    point: DisplayPoint,
    text_layout_details: &TextLayoutDetails,
) -> (DisplayPoint, SelectionGoal) {
    if let Some(visible_rows) = text_layout_details.visible_rows {
        let first_visible_line = text_layout_details
            .scroll_anchor
            .anchor
            .to_display_point(map);

        let max_visible_rows =
            (visible_rows as u32).min(map.max_point().row().0 - first_visible_line.row().0);

        let new_row =
            (first_visible_line.row().0 + (max_visible_rows / 2)).min(map.max_point().row().0);
        let new_row = DisplayRow(new_row);
        let new_col = point.column().min(map.line_len(new_row));
        let new_point = DisplayPoint::new(new_row, new_col);
        (map.clip_point(new_point, Bias::Left), SelectionGoal::None)
    } else {
        (point, SelectionGoal::None)
    }
}

fn window_bottom(
    map: &DisplaySnapshot,
    point: DisplayPoint,
    text_layout_details: &TextLayoutDetails,
    mut times: usize,
) -> (DisplayPoint, SelectionGoal) {
    if let Some(visible_rows) = text_layout_details.visible_rows {
        let first_visible_line = text_layout_details
            .scroll_anchor
            .anchor
            .to_display_point(map);
        let bottom_row = first_visible_line.row().0
            + (visible_rows + text_layout_details.scroll_anchor.offset.y - 1.).floor() as u32;
        if bottom_row < map.max_point().row().0
            && text_layout_details.vertical_scroll_margin as usize > times
        {
            times = text_layout_details.vertical_scroll_margin.ceil() as usize;
        }
        let bottom_row_capped = bottom_row.min(map.max_point().row().0);
        let new_row = if bottom_row_capped.saturating_sub(times as u32) < first_visible_line.row().0
        {
            first_visible_line.row()
        } else {
            DisplayRow(bottom_row_capped.saturating_sub(times as u32))
        };
        let new_col = point.column().min(map.line_len(new_row));
        let new_point = DisplayPoint::new(new_row, new_col);
        (map.clip_point(new_point, Bias::Left), SelectionGoal::None)
    } else {
        (point, SelectionGoal::None)
    }
}

fn method_motion(
    map: &DisplaySnapshot,
    mut display_point: DisplayPoint,
    times: usize,
    direction: Direction,
    is_start: bool,
) -> DisplayPoint {
    let Some((_, _, buffer)) = map.buffer_snapshot.as_singleton() else {
        return display_point;
    };

    for _ in 0..times {
        let point = map.display_point_to_point(display_point, Bias::Left);
        let offset = point.to_offset(&map.buffer_snapshot);
        let range = if direction == Direction::Prev {
            0..offset
        } else {
            offset..buffer.len()
        };

        let possibilities = buffer
            .text_object_ranges(range, language::TreeSitterOptions::max_start_depth(4))
            .filter_map(|(range, object)| {
                if !matches!(object, language::TextObject::AroundFunction) {
                    return None;
                }

                let relevant = if is_start { range.start } else { range.end };
                if direction == Direction::Prev && relevant < offset {
                    Some(relevant)
                } else if direction == Direction::Next && relevant > offset + 1 {
                    Some(relevant)
                } else {
                    None
                }
            });

        let dest = if direction == Direction::Prev {
            possibilities.max().unwrap_or(offset)
        } else {
            possibilities.min().unwrap_or(offset)
        };
        let new_point = map.clip_point(dest.to_display_point(map), Bias::Left);
        if new_point == display_point {
            break;
        }
        display_point = new_point;
    }
    display_point
}

fn comment_motion(
    map: &DisplaySnapshot,
    mut display_point: DisplayPoint,
    times: usize,
    direction: Direction,
) -> DisplayPoint {
    let Some((_, _, buffer)) = map.buffer_snapshot.as_singleton() else {
        return display_point;
    };

    for _ in 0..times {
        let point = map.display_point_to_point(display_point, Bias::Left);
        let offset = point.to_offset(&map.buffer_snapshot);
        let range = if direction == Direction::Prev {
            0..offset
        } else {
            offset..buffer.len()
        };

        let possibilities = buffer
            .text_object_ranges(range, language::TreeSitterOptions::max_start_depth(6))
            .filter_map(|(range, object)| {
                if !matches!(object, language::TextObject::AroundComment) {
                    return None;
                }

                let relevant = if direction == Direction::Prev {
                    range.start
                } else {
                    range.end
                };
                if direction == Direction::Prev && relevant < offset {
                    Some(relevant)
                } else if direction == Direction::Next && relevant > offset + 1 {
                    Some(relevant)
                } else {
                    None
                }
            });

        let dest = if direction == Direction::Prev {
            possibilities.max().unwrap_or(offset)
        } else {
            possibilities.min().unwrap_or(offset)
        };
        let new_point = map.clip_point(dest.to_display_point(map), Bias::Left);
        if new_point == display_point {
            break;
        }
        display_point = new_point;
    }

    display_point
}

fn section_motion(
    map: &DisplaySnapshot,
    mut display_point: DisplayPoint,
    times: usize,
    direction: Direction,
    is_start: bool,
) -> DisplayPoint {
    if map.buffer_snapshot.as_singleton().is_some() {
        for _ in 0..times {
            let offset = map
                .display_point_to_point(display_point, Bias::Left)
                .to_offset(&map.buffer_snapshot);
            let range = if direction == Direction::Prev {
                0..offset
            } else {
                offset..map.buffer_snapshot.len()
            };

            // we set a max start depth here because we want a section to only be "top level"
            // similar to vim's default of '{' in the first column.
            // (and without it, ]] at the start of editor.rs is -very- slow)
            let mut possibilities = map
                .buffer_snapshot
                .text_object_ranges(range, language::TreeSitterOptions::max_start_depth(3))
                .filter(|(_, object)| {
                    matches!(
                        object,
                        language::TextObject::AroundClass | language::TextObject::AroundFunction
                    )
                })
                .collect::<Vec<_>>();
            possibilities.sort_by_key(|(range_a, _)| range_a.start);
            let mut prev_end = None;
            let possibilities = possibilities.into_iter().filter_map(|(range, t)| {
                if t == language::TextObject::AroundFunction
                    && prev_end.is_some_and(|prev_end| prev_end > range.start)
                {
                    return None;
                }
                prev_end = Some(range.end);

                let relevant = if is_start { range.start } else { range.end };
                if direction == Direction::Prev && relevant < offset {
                    Some(relevant)
                } else if direction == Direction::Next && relevant > offset + 1 {
                    Some(relevant)
                } else {
                    None
                }
            });

            let offset = if direction == Direction::Prev {
                possibilities.max().unwrap_or(0)
            } else {
                possibilities.min().unwrap_or(map.buffer_snapshot.len())
            };

            let new_point = map.clip_point(offset.to_display_point(map), Bias::Left);
            if new_point == display_point {
                break;
            }
            display_point = new_point;
        }
        return display_point;
    };

    for _ in 0..times {
        let next_point = if is_start {
            movement::start_of_excerpt(map, display_point, direction)
        } else {
            movement::end_of_excerpt(map, display_point, direction)
        };
        if next_point == display_point {
            break;
        }
        display_point = next_point;
    }

    display_point
}

fn matches_indent_type(
    target_indent: &text::LineIndent,
    current_indent: &text::LineIndent,
    indent_type: IndentType,
) -> bool {
    match indent_type {
        IndentType::Lesser => {
            target_indent.spaces < current_indent.spaces || target_indent.tabs < current_indent.tabs
        }
        IndentType::Greater => {
            target_indent.spaces > current_indent.spaces || target_indent.tabs > current_indent.tabs
        }
        IndentType::Same => {
            target_indent.spaces == current_indent.spaces
                && target_indent.tabs == current_indent.tabs
        }
    }
}

fn indent_motion(
    map: &DisplaySnapshot,
    mut display_point: DisplayPoint,
    times: usize,
    direction: Direction,
    indent_type: IndentType,
) -> DisplayPoint {
    let buffer_point = map.display_point_to_point(display_point, Bias::Left);
    let current_row = MultiBufferRow(buffer_point.row);
    let current_indent = map.line_indent_for_buffer_row(current_row);
    if current_indent.is_line_empty() {
        return display_point;
    }
    let max_row = map.max_point().to_point(map).row;

    for _ in 0..times {
        let current_buffer_row = map.display_point_to_point(display_point, Bias::Left).row;

        let target_row = match direction {
            Direction::Next => (current_buffer_row + 1..=max_row).find(|&row| {
                let indent = map.line_indent_for_buffer_row(MultiBufferRow(row));
                !indent.is_line_empty()
                    && matches_indent_type(&indent, &current_indent, indent_type)
            }),
            Direction::Prev => (0..current_buffer_row).rev().find(|&row| {
                let indent = map.line_indent_for_buffer_row(MultiBufferRow(row));
                !indent.is_line_empty()
                    && matches_indent_type(&indent, &current_indent, indent_type)
            }),
        }
        .unwrap_or(current_buffer_row);

        let new_point = map.point_to_display_point(Point::new(target_row, 0), Bias::Right);
        let new_point = first_non_whitespace(map, false, new_point);
        if new_point == display_point {
            break;
        }
        display_point = new_point;
    }
    display_point
}

#[cfg(test)]
mod test {

    use crate::{
        state::Mode,
        test::{NeovimBackedTestContext, VimTestContext},
    };
    use editor::display_map::Inlay;
    use indoc::indoc;
    use language::Point;
    use multi_buffer::MultiBufferRow;

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
        cx.simulate_shared_keystrokes("}").await;
        cx.shared_state().await.assert_eq(indoc! {r"abc
            def
            ˇ
            paragraph
            the second



            third and
            final"});

        // goes up once
        cx.simulate_shared_keystrokes("{").await;
        cx.shared_state().await.assert_eq(initial_state);

        // goes down twice
        cx.simulate_shared_keystrokes("2 }").await;
        cx.shared_state().await.assert_eq(indoc! {r"abc
            def

            paragraph
            the second
            ˇ


            third and
            final"});

        // goes down over multiple blanks
        cx.simulate_shared_keystrokes("}").await;
        cx.shared_state().await.assert_eq(indoc! {r"abc
                def

                paragraph
                the second



                third and
                finaˇl"});

        // goes up twice
        cx.simulate_shared_keystrokes("2 {").await;
        cx.shared_state().await.assert_eq(indoc! {r"abc
                def
                ˇ
                paragraph
                the second



                third and
                final"});
    }

    #[gpui::test]
    async fn test_matching(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {r"func ˇ(a string) {
                do(something(with<Types>.and_arrays[0, 2]))
            }"})
            .await;
        cx.simulate_shared_keystrokes("%").await;
        cx.shared_state()
            .await
            .assert_eq(indoc! {r"func (a stringˇ) {
                do(something(with<Types>.and_arrays[0, 2]))
            }"});

        // test it works on the last character of the line
        cx.set_shared_state(indoc! {r"func (a string) ˇ{
            do(something(with<Types>.and_arrays[0, 2]))
            }"})
            .await;
        cx.simulate_shared_keystrokes("%").await;
        cx.shared_state()
            .await
            .assert_eq(indoc! {r"func (a string) {
            do(something(with<Types>.and_arrays[0, 2]))
            ˇ}"});

        // test it works on immediate nesting
        cx.set_shared_state("ˇ{()}").await;
        cx.simulate_shared_keystrokes("%").await;
        cx.shared_state().await.assert_eq("{()ˇ}");
        cx.simulate_shared_keystrokes("%").await;
        cx.shared_state().await.assert_eq("ˇ{()}");

        // test it works on immediate nesting inside braces
        cx.set_shared_state("{\n    ˇ{()}\n}").await;
        cx.simulate_shared_keystrokes("%").await;
        cx.shared_state().await.assert_eq("{\n    {()ˇ}\n}");

        // test it jumps to the next paren on a line
        cx.set_shared_state("func ˇboop() {\n}").await;
        cx.simulate_shared_keystrokes("%").await;
        cx.shared_state().await.assert_eq("func boop(ˇ) {\n}");
    }

    #[gpui::test]
    async fn test_unmatched_forward(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        // test it works with curly braces
        cx.set_shared_state(indoc! {r"func (a string) {
                do(something(with<Types>.anˇd_arrays[0, 2]))
            }"})
            .await;
        cx.simulate_shared_keystrokes("] }").await;
        cx.shared_state()
            .await
            .assert_eq(indoc! {r"func (a string) {
                do(something(with<Types>.and_arrays[0, 2]))
            ˇ}"});

        // test it works with brackets
        cx.set_shared_state(indoc! {r"func (a string) {
                do(somethiˇng(with<Types>.and_arrays[0, 2]))
            }"})
            .await;
        cx.simulate_shared_keystrokes("] )").await;
        cx.shared_state()
            .await
            .assert_eq(indoc! {r"func (a string) {
                do(something(with<Types>.and_arrays[0, 2])ˇ)
            }"});

        cx.set_shared_state(indoc! {r"func (a string) { a((b, cˇ))}"})
            .await;
        cx.simulate_shared_keystrokes("] )").await;
        cx.shared_state()
            .await
            .assert_eq(indoc! {r"func (a string) { a((b, c)ˇ)}"});

        // test it works on immediate nesting
        cx.set_shared_state("{ˇ {}{}}").await;
        cx.simulate_shared_keystrokes("] }").await;
        cx.shared_state().await.assert_eq("{ {}{}ˇ}");
        cx.set_shared_state("(ˇ ()())").await;
        cx.simulate_shared_keystrokes("] )").await;
        cx.shared_state().await.assert_eq("( ()()ˇ)");

        // test it works on immediate nesting inside braces
        cx.set_shared_state("{\n    ˇ {()}\n}").await;
        cx.simulate_shared_keystrokes("] }").await;
        cx.shared_state().await.assert_eq("{\n     {()}\nˇ}");
        cx.set_shared_state("(\n    ˇ {()}\n)").await;
        cx.simulate_shared_keystrokes("] )").await;
        cx.shared_state().await.assert_eq("(\n     {()}\nˇ)");
    }

    #[gpui::test]
    async fn test_unmatched_backward(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        // test it works with curly braces
        cx.set_shared_state(indoc! {r"func (a string) {
                do(something(with<Types>.anˇd_arrays[0, 2]))
            }"})
            .await;
        cx.simulate_shared_keystrokes("[ {").await;
        cx.shared_state()
            .await
            .assert_eq(indoc! {r"func (a string) ˇ{
                do(something(with<Types>.and_arrays[0, 2]))
            }"});

        // test it works with brackets
        cx.set_shared_state(indoc! {r"func (a string) {
                do(somethiˇng(with<Types>.and_arrays[0, 2]))
            }"})
            .await;
        cx.simulate_shared_keystrokes("[ (").await;
        cx.shared_state()
            .await
            .assert_eq(indoc! {r"func (a string) {
                doˇ(something(with<Types>.and_arrays[0, 2]))
            }"});

        // test it works on immediate nesting
        cx.set_shared_state("{{}{} ˇ }").await;
        cx.simulate_shared_keystrokes("[ {").await;
        cx.shared_state().await.assert_eq("ˇ{{}{}  }");
        cx.set_shared_state("(()() ˇ )").await;
        cx.simulate_shared_keystrokes("[ (").await;
        cx.shared_state().await.assert_eq("ˇ(()()  )");

        // test it works on immediate nesting inside braces
        cx.set_shared_state("{\n    {()} ˇ\n}").await;
        cx.simulate_shared_keystrokes("[ {").await;
        cx.shared_state().await.assert_eq("ˇ{\n    {()} \n}");
        cx.set_shared_state("(\n    {()} ˇ\n)").await;
        cx.simulate_shared_keystrokes("[ (").await;
        cx.shared_state().await.assert_eq("ˇ(\n    {()} \n)");
    }

    #[gpui::test]
    async fn test_matching_tags(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new_html(cx).await;

        cx.neovim.exec("set filetype=html").await;

        cx.set_shared_state(indoc! {r"<bˇody></body>"}).await;
        cx.simulate_shared_keystrokes("%").await;
        cx.shared_state()
            .await
            .assert_eq(indoc! {r"<body><ˇ/body>"});
        cx.simulate_shared_keystrokes("%").await;

        // test jumping backwards
        cx.shared_state()
            .await
            .assert_eq(indoc! {r"<ˇbody></body>"});

        // test self-closing tags
        cx.set_shared_state(indoc! {r"<a><bˇr/></a>"}).await;
        cx.simulate_shared_keystrokes("%").await;
        cx.shared_state().await.assert_eq(indoc! {r"<a><bˇr/></a>"});

        // test tag with attributes
        cx.set_shared_state(indoc! {r"<div class='test' ˇid='main'>
            </div>
            "})
            .await;
        cx.simulate_shared_keystrokes("%").await;
        cx.shared_state()
            .await
            .assert_eq(indoc! {r"<div class='test' id='main'>
            <ˇ/div>
            "});

        // test multi-line self-closing tag
        cx.set_shared_state(indoc! {r#"<a>
            <br
                test = "test"
            /ˇ>
        </a>"#})
            .await;
        cx.simulate_shared_keystrokes("%").await;
        cx.shared_state().await.assert_eq(indoc! {r#"<a>
            ˇ<br
                test = "test"
            />
        </a>"#});
    }

    #[gpui::test]
    async fn test_matching_braces_in_tag(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new_typescript(cx).await;

        // test brackets within tags
        cx.set_shared_state(indoc! {r"function f() {
            return (
                <div rules={ˇ[{ a: 1 }]}>
                    <h1>test</h1>
                </div>
            );
        }"})
            .await;
        cx.simulate_shared_keystrokes("%").await;
        cx.shared_state().await.assert_eq(indoc! {r"function f() {
            return (
                <div rules={[{ a: 1 }ˇ]}>
                    <h1>test</h1>
                </div>
            );
        }"});
    }

    #[gpui::test]
    async fn test_comma_semicolon(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        // f and F
        cx.set_shared_state("ˇone two three four").await;
        cx.simulate_shared_keystrokes("f o").await;
        cx.shared_state().await.assert_eq("one twˇo three four");
        cx.simulate_shared_keystrokes(",").await;
        cx.shared_state().await.assert_eq("ˇone two three four");
        cx.simulate_shared_keystrokes("2 ;").await;
        cx.shared_state().await.assert_eq("one two three fˇour");
        cx.simulate_shared_keystrokes("shift-f e").await;
        cx.shared_state().await.assert_eq("one two threˇe four");
        cx.simulate_shared_keystrokes("2 ;").await;
        cx.shared_state().await.assert_eq("onˇe two three four");
        cx.simulate_shared_keystrokes(",").await;
        cx.shared_state().await.assert_eq("one two thrˇee four");

        // t and T
        cx.set_shared_state("ˇone two three four").await;
        cx.simulate_shared_keystrokes("t o").await;
        cx.shared_state().await.assert_eq("one tˇwo three four");
        cx.simulate_shared_keystrokes(",").await;
        cx.shared_state().await.assert_eq("oˇne two three four");
        cx.simulate_shared_keystrokes("2 ;").await;
        cx.shared_state().await.assert_eq("one two three ˇfour");
        cx.simulate_shared_keystrokes("shift-t e").await;
        cx.shared_state().await.assert_eq("one two threeˇ four");
        cx.simulate_shared_keystrokes("3 ;").await;
        cx.shared_state().await.assert_eq("oneˇ two three four");
        cx.simulate_shared_keystrokes(",").await;
        cx.shared_state().await.assert_eq("one two thˇree four");
    }

    #[gpui::test]
    async fn test_next_word_end_newline_last_char(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        let initial_state = indoc! {r"something(ˇfoo)"};
        cx.set_shared_state(initial_state).await;
        cx.simulate_shared_keystrokes("}").await;
        cx.shared_state().await.assert_eq("something(fooˇ)");
    }

    #[gpui::test]
    async fn test_next_line_start(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state("ˇone\n  two\nthree").await;
        cx.simulate_shared_keystrokes("enter").await;
        cx.shared_state().await.assert_eq("one\n  ˇtwo\nthree");
    }

    #[gpui::test]
    async fn test_end_of_line_downward(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state("ˇ one\n two \nthree").await;
        cx.simulate_shared_keystrokes("g _").await;
        cx.shared_state().await.assert_eq(" onˇe\n two \nthree");

        cx.set_shared_state("ˇ one \n two \nthree").await;
        cx.simulate_shared_keystrokes("g _").await;
        cx.shared_state().await.assert_eq(" onˇe \n two \nthree");
        cx.simulate_shared_keystrokes("2 g _").await;
        cx.shared_state().await.assert_eq(" one \n twˇo \nthree");
    }

    #[gpui::test]
    async fn test_window_top(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        let initial_state = indoc! {r"abc
          def
          paragraph
          the second
          third ˇand
          final"};

        cx.set_shared_state(initial_state).await;
        cx.simulate_shared_keystrokes("shift-h").await;
        cx.shared_state().await.assert_eq(indoc! {r"abˇc
          def
          paragraph
          the second
          third and
          final"});

        // clip point
        cx.set_shared_state(indoc! {r"
          1 2 3
          4 5 6
          7 8 ˇ9
          "})
            .await;
        cx.simulate_shared_keystrokes("shift-h").await;
        cx.shared_state().await.assert_eq(indoc! {"
          1 2 ˇ3
          4 5 6
          7 8 9
          "});

        cx.set_shared_state(indoc! {r"
          1 2 3
          4 5 6
          ˇ7 8 9
          "})
            .await;
        cx.simulate_shared_keystrokes("shift-h").await;
        cx.shared_state().await.assert_eq(indoc! {"
          ˇ1 2 3
          4 5 6
          7 8 9
          "});

        cx.set_shared_state(indoc! {r"
          1 2 3
          4 5 ˇ6
          7 8 9"})
            .await;
        cx.simulate_shared_keystrokes("9 shift-h").await;
        cx.shared_state().await.assert_eq(indoc! {"
          1 2 3
          4 5 6
          7 8 ˇ9"});
    }

    #[gpui::test]
    async fn test_window_middle(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        let initial_state = indoc! {r"abˇc
          def
          paragraph
          the second
          third and
          final"};

        cx.set_shared_state(initial_state).await;
        cx.simulate_shared_keystrokes("shift-m").await;
        cx.shared_state().await.assert_eq(indoc! {r"abc
          def
          paˇragraph
          the second
          third and
          final"});

        cx.set_shared_state(indoc! {r"
          1 2 3
          4 5 6
          7 8 ˇ9
          "})
            .await;
        cx.simulate_shared_keystrokes("shift-m").await;
        cx.shared_state().await.assert_eq(indoc! {"
          1 2 3
          4 5 ˇ6
          7 8 9
          "});
        cx.set_shared_state(indoc! {r"
          1 2 3
          4 5 6
          ˇ7 8 9
          "})
            .await;
        cx.simulate_shared_keystrokes("shift-m").await;
        cx.shared_state().await.assert_eq(indoc! {"
          1 2 3
          ˇ4 5 6
          7 8 9
          "});
        cx.set_shared_state(indoc! {r"
          ˇ1 2 3
          4 5 6
          7 8 9
          "})
            .await;
        cx.simulate_shared_keystrokes("shift-m").await;
        cx.shared_state().await.assert_eq(indoc! {"
          1 2 3
          ˇ4 5 6
          7 8 9
          "});
        cx.set_shared_state(indoc! {r"
          1 2 3
          ˇ4 5 6
          7 8 9
          "})
            .await;
        cx.simulate_shared_keystrokes("shift-m").await;
        cx.shared_state().await.assert_eq(indoc! {"
          1 2 3
          ˇ4 5 6
          7 8 9
          "});
        cx.set_shared_state(indoc! {r"
          1 2 3
          4 5 ˇ6
          7 8 9
          "})
            .await;
        cx.simulate_shared_keystrokes("shift-m").await;
        cx.shared_state().await.assert_eq(indoc! {"
          1 2 3
          4 5 ˇ6
          7 8 9
          "});
    }

    #[gpui::test]
    async fn test_window_bottom(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        let initial_state = indoc! {r"abc
          deˇf
          paragraph
          the second
          third and
          final"};

        cx.set_shared_state(initial_state).await;
        cx.simulate_shared_keystrokes("shift-l").await;
        cx.shared_state().await.assert_eq(indoc! {r"abc
          def
          paragraph
          the second
          third and
          fiˇnal"});

        cx.set_shared_state(indoc! {r"
          1 2 3
          4 5 ˇ6
          7 8 9
          "})
            .await;
        cx.simulate_shared_keystrokes("shift-l").await;
        cx.shared_state().await.assert_eq(indoc! {"
          1 2 3
          4 5 6
          7 8 9
          ˇ"});

        cx.set_shared_state(indoc! {r"
          1 2 3
          ˇ4 5 6
          7 8 9
          "})
            .await;
        cx.simulate_shared_keystrokes("shift-l").await;
        cx.shared_state().await.assert_eq(indoc! {"
          1 2 3
          4 5 6
          7 8 9
          ˇ"});

        cx.set_shared_state(indoc! {r"
          1 2 ˇ3
          4 5 6
          7 8 9
          "})
            .await;
        cx.simulate_shared_keystrokes("shift-l").await;
        cx.shared_state().await.assert_eq(indoc! {"
          1 2 3
          4 5 6
          7 8 9
          ˇ"});

        cx.set_shared_state(indoc! {r"
          ˇ1 2 3
          4 5 6
          7 8 9
          "})
            .await;
        cx.simulate_shared_keystrokes("shift-l").await;
        cx.shared_state().await.assert_eq(indoc! {"
          1 2 3
          4 5 6
          7 8 9
          ˇ"});

        cx.set_shared_state(indoc! {r"
          1 2 3
          4 5 ˇ6
          7 8 9
          "})
            .await;
        cx.simulate_shared_keystrokes("9 shift-l").await;
        cx.shared_state().await.assert_eq(indoc! {"
          1 2 ˇ3
          4 5 6
          7 8 9
          "});
    }

    #[gpui::test]
    async fn test_previous_word_end(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state(indoc! {r"
        456 5ˇ67 678
        "})
            .await;
        cx.simulate_shared_keystrokes("g e").await;
        cx.shared_state().await.assert_eq(indoc! {"
        45ˇ6 567 678
        "});

        // Test times
        cx.set_shared_state(indoc! {r"
        123 234 345
        456 5ˇ67 678
        "})
            .await;
        cx.simulate_shared_keystrokes("4 g e").await;
        cx.shared_state().await.assert_eq(indoc! {"
        12ˇ3 234 345
        456 567 678
        "});

        // With punctuation
        cx.set_shared_state(indoc! {r"
        123 234 345
        4;5.6 5ˇ67 678
        789 890 901
        "})
            .await;
        cx.simulate_shared_keystrokes("g e").await;
        cx.shared_state().await.assert_eq(indoc! {"
          123 234 345
          4;5.ˇ6 567 678
          789 890 901
        "});

        // With punctuation and count
        cx.set_shared_state(indoc! {r"
        123 234 345
        4;5.6 5ˇ67 678
        789 890 901
        "})
            .await;
        cx.simulate_shared_keystrokes("5 g e").await;
        cx.shared_state().await.assert_eq(indoc! {"
          123 234 345
          ˇ4;5.6 567 678
          789 890 901
        "});

        // newlines
        cx.set_shared_state(indoc! {r"
        123 234 345

        78ˇ9 890 901
        "})
            .await;
        cx.simulate_shared_keystrokes("g e").await;
        cx.shared_state().await.assert_eq(indoc! {"
          123 234 345
          ˇ
          789 890 901
        "});
        cx.simulate_shared_keystrokes("g e").await;
        cx.shared_state().await.assert_eq(indoc! {"
          123 234 34ˇ5

          789 890 901
        "});

        // With punctuation
        cx.set_shared_state(indoc! {r"
        123 234 345
        4;5.ˇ6 567 678
        789 890 901
        "})
            .await;
        cx.simulate_shared_keystrokes("g shift-e").await;
        cx.shared_state().await.assert_eq(indoc! {"
          123 234 34ˇ5
          4;5.6 567 678
          789 890 901
        "});

        // With multi byte char
        cx.set_shared_state(indoc! {r"
        bar ˇó
        "})
            .await;
        cx.simulate_shared_keystrokes("g e").await;
        cx.shared_state().await.assert_eq(indoc! {"
        baˇr ó
        "});
    }

    #[gpui::test]
    async fn test_visual_match_eol(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            fn aˇ() {
              return
            }
        "})
            .await;
        cx.simulate_shared_keystrokes("v $ %").await;
        cx.shared_state().await.assert_eq(indoc! {"
            fn a«() {
              return
            }ˇ»
        "});
    }

    #[gpui::test]
    async fn test_clipping_with_inlay_hints(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state(
            indoc! {"
                struct Foo {
                ˇ
                }
            "},
            Mode::Normal,
        );

        cx.update_editor(|editor, _window, cx| {
            let range = editor.selections.newest_anchor().range();
            let inlay_text = "  field: int,\n  field2: string\n  field3: float";
            let inlay = Inlay::edit_prediction(1, range.start, inlay_text);
            editor.splice_inlays(&[], vec![inlay], cx);
        });

        cx.simulate_keystrokes("j");
        cx.assert_state(
            indoc! {"
                struct Foo {

                ˇ}
            "},
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_clipping_with_inlay_hints_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;

        cx.set_state(
            indoc! {"
            ˇstruct Foo {

            }
        "},
            Mode::Normal,
        );
        cx.update_editor(|editor, _window, cx| {
            let snapshot = editor.buffer().read(cx).snapshot(cx);
            let end_of_line =
                snapshot.anchor_after(Point::new(0, snapshot.line_len(MultiBufferRow(0))));
            let inlay_text = " hint";
            let inlay = Inlay::edit_prediction(1, end_of_line, inlay_text);
            editor.splice_inlays(&[], vec![inlay], cx);
        });
        cx.simulate_keystrokes("$");
        cx.assert_state(
            indoc! {"
            struct Foo ˇ{

            }
        "},
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_go_to_percentage(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        // Normal mode
        cx.set_shared_state(indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog
            The quick brown
            fox jumps over
            the lazy dog
            The quick brown
            fox jumps over
            the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("2 0 %").await;
        cx.shared_state().await.assert_eq(indoc! {"
            The quick brown
            fox ˇjumps over
            the lazy dog
            The quick brown
            fox jumps over
            the lazy dog
            The quick brown
            fox jumps over
            the lazy dog"});

        cx.simulate_shared_keystrokes("2 5 %").await;
        cx.shared_state().await.assert_eq(indoc! {"
            The quick brown
            fox jumps over
            the ˇlazy dog
            The quick brown
            fox jumps over
            the lazy dog
            The quick brown
            fox jumps over
            the lazy dog"});

        cx.simulate_shared_keystrokes("7 5 %").await;
        cx.shared_state().await.assert_eq(indoc! {"
            The quick brown
            fox jumps over
            the lazy dog
            The quick brown
            fox jumps over
            the lazy dog
            The ˇquick brown
            fox jumps over
            the lazy dog"});

        // Visual mode
        cx.set_shared_state(indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog
            The quick brown
            fox jumps over
            the lazy dog
            The quick brown
            fox jumps over
            the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("v 5 0 %").await;
        cx.shared_state().await.assert_eq(indoc! {"
            The «quick brown
            fox jumps over
            the lazy dog
            The quick brown
            fox jˇ»umps over
            the lazy dog
            The quick brown
            fox jumps over
            the lazy dog"});

        cx.set_shared_state(indoc! {"
            The ˇquick brown
            fox jumps over
            the lazy dog
            The quick brown
            fox jumps over
            the lazy dog
            The quick brown
            fox jumps over
            the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("v 1 0 0 %").await;
        cx.shared_state().await.assert_eq(indoc! {"
            The «quick brown
            fox jumps over
            the lazy dog
            The quick brown
            fox jumps over
            the lazy dog
            The quick brown
            fox jumps over
            the lˇ»azy dog"});
    }

    #[gpui::test]
    async fn test_space_non_ascii(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state("ˇπππππ").await;
        cx.simulate_shared_keystrokes("3 space").await;
        cx.shared_state().await.assert_eq("πππˇππ");
    }

    #[gpui::test]
    async fn test_space_non_ascii_eol(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
            ππππˇπ
            πanotherline"})
            .await;
        cx.simulate_shared_keystrokes("4 space").await;
        cx.shared_state().await.assert_eq(indoc! {"
            πππππ
            πanˇotherline"});
    }

    #[gpui::test]
    async fn test_backspace_non_ascii_bol(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
                        ππππ
                        πanˇotherline"})
            .await;
        cx.simulate_shared_keystrokes("4 backspace").await;
        cx.shared_state().await.assert_eq(indoc! {"
                        πππˇπ
                        πanotherline"});
    }

    #[gpui::test]
    async fn test_go_to_indent(cx: &mut gpui::TestAppContext) {
        let mut cx = VimTestContext::new(cx, true).await;
        cx.set_state(
            indoc! {
                "func empty(a string) bool {
                     ˇif a == \"\" {
                         return true
                     }
                     return false
                }"
            },
            Mode::Normal,
        );
        cx.simulate_keystrokes("[ -");
        cx.assert_state(
            indoc! {
                "ˇfunc empty(a string) bool {
                     if a == \"\" {
                         return true
                     }
                     return false
                }"
            },
            Mode::Normal,
        );
        cx.simulate_keystrokes("] =");
        cx.assert_state(
            indoc! {
                "func empty(a string) bool {
                     if a == \"\" {
                         return true
                     }
                     return false
                ˇ}"
            },
            Mode::Normal,
        );
        cx.simulate_keystrokes("[ +");
        cx.assert_state(
            indoc! {
                "func empty(a string) bool {
                     if a == \"\" {
                         return true
                     }
                     ˇreturn false
                }"
            },
            Mode::Normal,
        );
        cx.simulate_keystrokes("2 [ =");
        cx.assert_state(
            indoc! {
                "func empty(a string) bool {
                     ˇif a == \"\" {
                         return true
                     }
                     return false
                }"
            },
            Mode::Normal,
        );
        cx.simulate_keystrokes("] +");
        cx.assert_state(
            indoc! {
                "func empty(a string) bool {
                     if a == \"\" {
                         ˇreturn true
                     }
                     return false
                }"
            },
            Mode::Normal,
        );
        cx.simulate_keystrokes("] -");
        cx.assert_state(
            indoc! {
                "func empty(a string) bool {
                     if a == \"\" {
                         return true
                     ˇ}
                     return false
                }"
            },
            Mode::Normal,
        );
    }

    #[gpui::test]
    async fn test_delete_key_can_remove_last_character(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;
        cx.set_shared_state("abˇc").await;
        cx.simulate_shared_keystrokes("delete").await;
        cx.shared_state().await.assert_eq("aˇb");
    }

    #[gpui::test]
    async fn test_forced_motion_delete_to_start_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
             ˇthe quick brown fox
             jumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("d v 0").await;
        cx.shared_state().await.assert_eq(indoc! {"
             ˇhe quick brown fox
             jumped over the lazy dog"});
        assert!(!cx.cx.forced_motion());

        cx.set_shared_state(indoc! {"
            the quick bˇrown fox
            jumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("d v 0").await;
        cx.shared_state().await.assert_eq(indoc! {"
            ˇown fox
            jumped over the lazy dog"});
        assert!(!cx.cx.forced_motion());

        cx.set_shared_state(indoc! {"
            the quick brown foˇx
            jumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("d v 0").await;
        cx.shared_state().await.assert_eq(indoc! {"
            ˇ
            jumped over the lazy dog"});
        assert!(!cx.cx.forced_motion());
    }

    #[gpui::test]
    async fn test_forced_motion_delete_to_middle_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
             ˇthe quick brown fox
             jumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("d v g shift-m").await;
        cx.shared_state().await.assert_eq(indoc! {"
             ˇbrown fox
             jumped over the lazy dog"});
        assert!(!cx.cx.forced_motion());

        cx.set_shared_state(indoc! {"
            the quick bˇrown fox
            jumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("d v g shift-m").await;
        cx.shared_state().await.assert_eq(indoc! {"
            the quickˇown fox
            jumped over the lazy dog"});
        assert!(!cx.cx.forced_motion());

        cx.set_shared_state(indoc! {"
            the quick brown foˇx
            jumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("d v g shift-m").await;
        cx.shared_state().await.assert_eq(indoc! {"
            the quicˇk
            jumped over the lazy dog"});
        assert!(!cx.cx.forced_motion());

        cx.set_shared_state(indoc! {"
            ˇthe quick brown fox
            jumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("d v 7 5 g shift-m").await;
        cx.shared_state().await.assert_eq(indoc! {"
            ˇ fox
            jumped over the lazy dog"});
        assert!(!cx.cx.forced_motion());

        cx.set_shared_state(indoc! {"
            ˇthe quick brown fox
            jumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("d v 2 3 g shift-m").await;
        cx.shared_state().await.assert_eq(indoc! {"
            ˇuick brown fox
            jumped over the lazy dog"});
        assert!(!cx.cx.forced_motion());
    }

    #[gpui::test]
    async fn test_forced_motion_delete_to_end_of_line(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
             the quick brown foˇx
             jumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("d v $").await;
        cx.shared_state().await.assert_eq(indoc! {"
             the quick brown foˇx
             jumped over the lazy dog"});
        assert!(!cx.cx.forced_motion());

        cx.set_shared_state(indoc! {"
             ˇthe quick brown fox
             jumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("d v $").await;
        cx.shared_state().await.assert_eq(indoc! {"
             ˇx
             jumped over the lazy dog"});
        assert!(!cx.cx.forced_motion());
    }

    #[gpui::test]
    async fn test_forced_motion_yank(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
               ˇthe quick brown fox
               jumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("y v j p").await;
        cx.shared_state().await.assert_eq(indoc! {"
               the quick brown fox
               ˇthe quick brown fox
               jumped over the lazy dog"});
        assert!(!cx.cx.forced_motion());

        cx.set_shared_state(indoc! {"
              the quick bˇrown fox
              jumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("y v j p").await;
        cx.shared_state().await.assert_eq(indoc! {"
              the quick brˇrown fox
              jumped overown fox
              jumped over the lazy dog"});
        assert!(!cx.cx.forced_motion());

        cx.set_shared_state(indoc! {"
             the quick brown foˇx
             jumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("y v j p").await;
        cx.shared_state().await.assert_eq(indoc! {"
             the quick brown foxˇx
             jumped over the la
             jumped over the lazy dog"});
        assert!(!cx.cx.forced_motion());

        cx.set_shared_state(indoc! {"
             the quick brown fox
             jˇumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("y v k p").await;
        cx.shared_state().await.assert_eq(indoc! {"
            thˇhe quick brown fox
            je quick brown fox
            jumped over the lazy dog"});
        assert!(!cx.cx.forced_motion());
    }

    #[gpui::test]
    async fn test_inclusive_to_exclusive_delete(cx: &mut gpui::TestAppContext) {
        let mut cx = NeovimBackedTestContext::new(cx).await;

        cx.set_shared_state(indoc! {"
              ˇthe quick brown fox
              jumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("d v e").await;
        cx.shared_state().await.assert_eq(indoc! {"
              ˇe quick brown fox
              jumped over the lazy dog"});
        assert!(!cx.cx.forced_motion());

        cx.set_shared_state(indoc! {"
              the quick bˇrown fox
              jumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("d v e").await;
        cx.shared_state().await.assert_eq(indoc! {"
              the quick bˇn fox
              jumped over the lazy dog"});
        assert!(!cx.cx.forced_motion());

        cx.set_shared_state(indoc! {"
             the quick brown foˇx
             jumped over the lazy dog"})
            .await;
        cx.simulate_shared_keystrokes("d v e").await;
        cx.shared_state().await.assert_eq(indoc! {"
        the quick brown foˇd over the lazy dog"});
        assert!(!cx.cx.forced_motion());
    }
}
