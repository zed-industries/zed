use std::cmp::Ordering;
use std::hash::Hash;
use std::ops::{Add, Range};

use crate::intern::Token;

pub trait SliderHeuristic {
    fn best_slider_end(&mut self, tokens: &[Token], hunk: Range<u32>, earliest_end: u32) -> u32;
}

impl<F> SliderHeuristic for F
where
    F: FnMut(&[Token], Range<u32>, u32) -> u32,
{
    fn best_slider_end(&mut self, tokens: &[Token], hunk: Range<u32>, earliest_end: u32) -> u32 {
        self(tokens, hunk, earliest_end)
    }
}

pub struct NoSliderHeuristic;

impl SliderHeuristic for NoSliderHeuristic {
    fn best_slider_end(&mut self, _tokens: &[Token], hunk: Range<u32>, _earliest_end: u32) -> u32 {
        hunk.end
    }
}

pub struct IndentHeuristic<IndentOfToken> {
    indent_of_token: IndentOfToken,
}

impl<IndentOfToken> IndentHeuristic<IndentOfToken> {
    pub fn new(indent_of_token: IndentOfToken) -> Self {
        Self { indent_of_token }
    }
}

impl<IndentOfToken: Fn(Token) -> IndentLevel> SliderHeuristic for IndentHeuristic<IndentOfToken> {
    fn best_slider_end(&mut self, tokens: &[Token], hunk: Range<u32>, earliest_end: u32) -> u32 {
        const MAX_SLIDING: u32 = 100;
        // this is a pure insertation that can be moved freely up and down
        // to get more intutive results apply a heuristic
        let mut top_slider_end = earliest_end;
        // TODO: why is this needed
        if top_slider_end < hunk.start - 1 {
            top_slider_end = hunk.start - 1;
        }
        if hunk.end > top_slider_end + MAX_SLIDING {
            top_slider_end = hunk.end - MAX_SLIDING;
        }
        let group_size = hunk.end - hunk.start;
        let mut best_score = Score::for_range(
            top_slider_end - group_size..top_slider_end,
            tokens,
            &self.indent_of_token,
        );
        let mut best_slider_end = top_slider_end;
        for slider_end in (top_slider_end + 1)..=hunk.end {
            let score = Score::for_range(
                slider_end - group_size..slider_end,
                tokens,
                &self.indent_of_token,
            );
            if score.is_improvement_over(best_score) {
                best_score = score;
                best_slider_end = slider_end;
            }
        }
        best_slider_end
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy, Hash, PartialOrd)]
pub struct IndentLevel(u8);

impl IndentLevel {
    /// line is empty or only contains whitespaces (or EOF)
    const BLANK: IndentLevel = IndentLevel(u8::MAX);
    const MAX: IndentLevel = IndentLevel(200);

    pub fn for_ascii_line(src: impl IntoIterator<Item = u8>, tab_width: u8) -> IndentLevel {
        let mut indent_level = IndentLevel(0);
        for c in src {
            match c {
                b' ' => indent_level.0 += 1,
                b'\t' => indent_level.0 += tab_width - indent_level.0 % tab_width,
                b'\r' | b'\n' | b'\x0C' => (),
                _ => return indent_level,
            }
            if indent_level >= Self::MAX {
                return indent_level;
            }
        }
        IndentLevel::BLANK
    }

    pub fn for_line(src: impl IntoIterator<Item = char>, tab_width: u8) -> IndentLevel {
        let mut indent_level = IndentLevel(0);
        for c in src {
            match c {
                ' ' => indent_level.0 += 1,
                '\t' => indent_level.0 += tab_width - indent_level.0 % tab_width,
                '\r' | '\n' | '\x0C' => (),
                _ => return indent_level,
            }
            if indent_level >= Self::MAX {
                return indent_level;
            }
        }
        IndentLevel::BLANK
    }

    fn map_or<T>(self, default: T, f: impl FnOnce(u8) -> T) -> T {
        if self == Self::BLANK {
            default
        } else {
            f(self.0)
        }
    }

    fn or(self, default: Self) -> Self {
        if self == Self::BLANK {
            default
        } else {
            self
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Indents {
    /// indent level of the line/token
    indent: IndentLevel,
    /// indent level at the previous (non-blank) line
    prev_indent: IndentLevel,
    /// indent level at the next (non-blank) line
    next_indent: IndentLevel,
    /// How many consecutive lines above the split are blank?
    leading_blanks: u8,
    /// How many lines after the line following the split are blank?
    trailing_blanks: u8,
}

const MAX_BLANKS: usize = 20;

impl Indents {
    fn at_token(
        tokens: &[Token],
        token_idx: usize,
        indent_of_token: impl Fn(Token) -> IndentLevel,
    ) -> Indents {
        let (leading_blank_lines, indent_previous_line) = tokens[..token_idx]
            .iter()
            .rev()
            .enumerate()
            .find_map(|(i, &token)| {
                if i == MAX_BLANKS {
                    Some((i, IndentLevel(0)))
                } else {
                    let level = indent_of_token(token);
                    if level == IndentLevel::BLANK {
                        None
                    } else {
                        Some((i, level))
                    }
                }
            })
            .unwrap_or((token_idx, IndentLevel::BLANK));
        let at_eof = token_idx == tokens.len();
        let (trailing_blank_lines, indent_next_line) = if at_eof {
            (0, IndentLevel::BLANK)
        } else {
            tokens[token_idx + 1..]
                .iter()
                .enumerate()
                .find_map(|(i, &token)| {
                    if i == MAX_BLANKS {
                        Some((i, IndentLevel(0)))
                    } else {
                        let level = indent_of_token(token);
                        if level == IndentLevel::BLANK {
                            None
                        } else {
                            Some((i, level))
                        }
                    }
                })
                .unwrap_or((token_idx, IndentLevel::BLANK))
        };
        let indent = tokens
            .get(token_idx)
            .map_or(IndentLevel::BLANK, |&token| indent_of_token(token));
        Indents {
            indent,
            prev_indent: indent_previous_line,
            next_indent: indent_next_line,
            leading_blanks: leading_blank_lines as u8,
            trailing_blanks: trailing_blank_lines as u8,
        }
    }

    fn score(&self) -> Score {
        let mut penalty = 0;
        if self.prev_indent == IndentLevel::BLANK && self.leading_blanks == 0 {
            penalty += START_OF_FILE_PENALTY;
        }
        if self.next_indent == IndentLevel::BLANK && self.trailing_blanks == 0 {
            penalty += END_OF_FILE_PENALTY;
        }

        let trailing_blank_lines = if self.indent == IndentLevel::BLANK {
            self.trailing_blanks as i32 + 1
        } else {
            0
        };
        let total_blank_lines = trailing_blank_lines + self.leading_blanks as i32;
        penalty += TOTAL_BLANK_LINE_WEIGHT * total_blank_lines
            + trailing_blank_lines * TRAILING_BLANK_LINES_WEIGHT;
        let indent = self.indent.or(self.next_indent);
        if indent != IndentLevel::BLANK && self.prev_indent != IndentLevel::BLANK {
            match indent.0.cmp(&self.prev_indent.0) {
                Ordering::Equal => {}
                // self.next_indent != IndentLevel::BLANK follows for free here
                // since indent != BLANK and therefore self.next_indent <= indent < BLANK
                Ordering::Less if self.next_indent.0 <= indent.0 => {
                    penalty += if total_blank_lines != 0 {
                        RELATIVE_DEDENT_WITH_BLANK_PENALTY
                    } else {
                        RELATIVE_DEDENT_PENALTY
                    }
                }
                Ordering::Less => {
                    penalty += if total_blank_lines != 0 {
                        RELATIVE_OUTDENT_WITH_BLANK_PENALTY
                    } else {
                        RELATIVE_OUTDENT_PENALTY
                    }
                }
                Ordering::Greater => {
                    penalty += if total_blank_lines != 0 {
                        RELATIVE_INDENT_WITH_BLANK_PENALTY
                    } else {
                        RELATIVE_INDENT_PENALTY
                    }
                }
            }
        }
        Score {
            indent: indent.map_or(-1, i32::from),
            penalty,
        }
    }
}

const START_OF_FILE_PENALTY: i32 = 1;
const END_OF_FILE_PENALTY: i32 = 21;
const TOTAL_BLANK_LINE_WEIGHT: i32 = -30;
const TRAILING_BLANK_LINES_WEIGHT: i32 = 6;

const RELATIVE_INDENT_PENALTY: i32 = -4;
const RELATIVE_INDENT_WITH_BLANK_PENALTY: i32 = 10;

const RELATIVE_OUTDENT_PENALTY: i32 = 24;
const RELATIVE_OUTDENT_WITH_BLANK_PENALTY: i32 = 17;

const RELATIVE_DEDENT_PENALTY: i32 = 23;
const RELATIVE_DEDENT_WITH_BLANK_PENALTY: i32 = 17;

const INDENT_WEIGHT: i32 = 60;

#[derive(PartialEq, Eq, Clone, Copy)]
struct Score {
    indent: i32,
    penalty: i32,
}

impl Score {
    fn for_range(
        range: Range<u32>,
        tokens: &[Token],
        indent_of_token: impl Fn(Token) -> IndentLevel,
    ) -> Score {
        Indents::at_token(tokens, range.start as usize, &indent_of_token).score()
            + Indents::at_token(tokens, range.end as usize, &indent_of_token).score()
    }
}

impl Add for Score {
    type Output = Score;

    fn add(self, rhs: Self) -> Self::Output {
        Score {
            indent: self.indent + rhs.indent,
            penalty: self.penalty + rhs.penalty,
        }
    }
}

impl Score {
    fn is_improvement_over(self, prev_score: Self) -> bool {
        // smaller indentation level is preferred (with a weight)
        let indent_score = match prev_score.indent.cmp(&self.indent) {
            Ordering::Less => INDENT_WEIGHT,
            Ordering::Greater => -INDENT_WEIGHT,
            Ordering::Equal => 0,
        };
        (indent_score + self.penalty - prev_score.penalty) <= 0
    }
}
