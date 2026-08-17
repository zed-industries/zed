// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

//! Types for different kinds of parsing failures.

use crate::parser_state::{ParseAttempts, ParsingToken, RulesCallStack};
use alloc::borrow::Cow;
use alloc::borrow::ToOwned;
use alloc::boxed::Box;
use alloc::collections::{BTreeMap, BTreeSet};
use alloc::format;
use alloc::string::String;
use alloc::string::ToString;
use alloc::vec;
use alloc::vec::Vec;
use core::cmp;
use core::fmt;
use core::mem;

use crate::position::Position;
use crate::span::Span;
use crate::RuleType;

/// Parse-related error type.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub struct Error<R> {
    /// Variant of the error
    pub variant: ErrorVariant<R>,
    /// Location within the input string
    pub location: InputLocation,
    /// Line/column within the input string
    pub line_col: LineColLocation,
    path: Option<String>,
    line: String,
    continued_line: Option<String>,
    parse_attempts: Option<ParseAttempts<R>>,
}

impl<R: RuleType> core::error::Error for Error<R> {}

/// Different kinds of parsing errors.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum ErrorVariant<R> {
    /// Generated parsing error with expected and unexpected `Rule`s
    ParsingError {
        /// Positive attempts
        positives: Vec<R>,
        /// Negative attempts
        negatives: Vec<R>,
    },
    /// Custom error with a message
    CustomError {
        /// Short explanation
        message: String,
    },
}

impl<R: RuleType> core::error::Error for ErrorVariant<R> {}

/// Where an `Error` has occurred.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum InputLocation {
    /// `Error` was created by `Error::new_from_pos`
    Pos(usize),
    /// `Error` was created by `Error::new_from_span`
    Span((usize, usize)),
}

/// Line/column where an `Error` has occurred.
#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum LineColLocation {
    /// Line/column pair if `Error` was created by `Error::new_from_pos`
    Pos((usize, usize)),
    /// Line/column pairs if `Error` was created by `Error::new_from_span`
    Span((usize, usize), (usize, usize)),
}

impl From<Position<'_>> for LineColLocation {
    fn from(value: Position<'_>) -> Self {
        Self::Pos(value.line_col())
    }
}

impl From<Span<'_>> for LineColLocation {
    fn from(value: Span<'_>) -> Self {
        let (start, end) = value.split();
        Self::Span(start.line_col(), end.line_col())
    }
}

/// Function mapping rule to its helper message defined by user.
pub type RuleToMessageFn<R> = Box<dyn Fn(&R) -> Option<String>>;
/// Function mapping string element to bool denoting whether it's a whitespace defined by user.
pub type IsWhitespaceFn = Box<dyn Fn(String) -> bool>;

impl ParsingToken {
    pub fn is_whitespace(&self, is_whitespace: &IsWhitespaceFn) -> bool {
        match self {
            ParsingToken::Sensitive { token } => is_whitespace(token.clone()),
            ParsingToken::Insensitive { token } => is_whitespace(token.clone()),
            ParsingToken::Range { .. } => false,
            ParsingToken::BuiltInRule => false,
        }
    }
}

impl<R: RuleType> ParseAttempts<R> {
    /// Helper formatting function to get message informing about tokens we've
    /// (un)expected to see.
    /// Used as a part of `parse_attempts_error`.
    fn tokens_helper_messages(
        &self,
        is_whitespace_fn: &IsWhitespaceFn,
        spacing: &str,
    ) -> Vec<String> {
        let mut helper_messages = Vec::new();
        let tokens_header_pairs = vec![
            (self.expected_tokens(), "expected"),
            (self.unexpected_tokens(), "unexpected"),
        ];

        for (tokens, header) in &tokens_header_pairs {
            if tokens.is_empty() {
                continue;
            }

            let mut helper_tokens_message = format!("{spacing}note: {header} ");
            helper_tokens_message.push_str(if tokens.len() == 1 {
                "token: "
            } else {
                "one of tokens: "
            });

            let expected_tokens_set: BTreeSet<String> = tokens
                .iter()
                .map(|token| {
                    if token.is_whitespace(is_whitespace_fn) {
                        String::from("WHITESPACE")
                    } else {
                        format!("`{}`", token)
                    }
                })
                .collect();

            helper_tokens_message.push_str(
                &expected_tokens_set
                    .iter()
                    .cloned()
                    .collect::<Vec<String>>()
                    .join(", "),
            );
            helper_messages.push(helper_tokens_message);
        }

        helper_messages
    }
}

impl<R: RuleType> Error<R> {
    /// Creates `Error` from `ErrorVariant` and `Position`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::error::{Error, ErrorVariant};
    /// # use pest::Position;
    /// # #[allow(non_camel_case_types)]
    /// # #[allow(dead_code)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// # enum Rule {
    /// #     open_paren,
    /// #     closed_paren
    /// # }
    /// # let input = "";
    /// # let pos = Position::from_start(input);
    /// let error = Error::new_from_pos(
    ///     ErrorVariant::ParsingError {
    ///         positives: vec![Rule::open_paren],
    ///         negatives: vec![Rule::closed_paren],
    ///     },
    ///     pos
    /// );
    ///
    /// println!("{}", error);
    /// ```
    pub fn new_from_pos(variant: ErrorVariant<R>, pos: Position<'_>) -> Error<R> {
        let visualize_ws = pos.match_char('\n') || pos.match_char('\r');
        let line_of = pos.line_of();
        let line = if visualize_ws {
            visualize_whitespace(line_of)
        } else {
            line_of.replace(&['\r', '\n'][..], "")
        };
        Error {
            variant,
            location: InputLocation::Pos(pos.pos()),
            path: None,
            line,
            continued_line: None,
            line_col: LineColLocation::Pos(pos.line_col()),
            parse_attempts: None,
        }
    }

    /// Wrapper function to track `parse_attempts` as a result
    /// of `state` function call in `parser_state.rs`.
    pub(crate) fn new_from_pos_with_parsing_attempts(
        variant: ErrorVariant<R>,
        pos: Position<'_>,
        parse_attempts: ParseAttempts<R>,
    ) -> Error<R> {
        let mut error = Self::new_from_pos(variant, pos);
        error.parse_attempts = Some(parse_attempts);
        error
    }

    /// Creates `Error` from `ErrorVariant` and `Span`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::error::{Error, ErrorVariant};
    /// # use pest::{Position, Span};
    /// # #[allow(non_camel_case_types)]
    /// # #[allow(dead_code)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// # enum Rule {
    /// #     open_paren,
    /// #     closed_paren
    /// # }
    /// # let input = "";
    /// # let start = Position::from_start(input);
    /// # let end = start.clone();
    /// # let span = start.span(&end);
    /// let error = Error::new_from_span(
    ///     ErrorVariant::ParsingError {
    ///         positives: vec![Rule::open_paren],
    ///         negatives: vec![Rule::closed_paren],
    ///     },
    ///     span
    /// );
    ///
    /// println!("{}", error);
    /// ```
    pub fn new_from_span(variant: ErrorVariant<R>, span: Span<'_>) -> Error<R> {
        let end = span.end_pos();
        let mut end_line_col = end.line_col();
        // end position is after a \n, so we want to point to the visual lf symbol
        if end_line_col.1 == 1 {
            let mut visual_end = end;
            visual_end.skip_back(1);
            let lc = visual_end.line_col();
            end_line_col = (lc.0, lc.1 + 1);
        };

        let mut line_iter = span.lines();
        let sl = line_iter.next().unwrap_or("");
        let mut chars = span.as_str().chars();
        let visualize_ws = matches!(chars.next(), Some('\n') | Some('\r'))
            || matches!(chars.last(), Some('\n') | Some('\r'));
        let start_line = if visualize_ws {
            visualize_whitespace(sl)
        } else {
            sl.to_owned().replace(&['\r', '\n'][..], "")
        };
        let ll = line_iter.last();
        let continued_line = if visualize_ws {
            ll.map(str::to_owned)
        } else {
            ll.map(visualize_whitespace)
        };

        Error {
            variant,
            location: InputLocation::Span((span.start(), end.pos())),
            path: None,
            line: start_line,
            continued_line,
            line_col: LineColLocation::Span(span.start_pos().line_col(), end_line_col),
            parse_attempts: None,
        }
    }

    /// Returns `Error` variant with `path` which is shown when formatted with `Display`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::error::{Error, ErrorVariant};
    /// # use pest::Position;
    /// # #[allow(non_camel_case_types)]
    /// # #[allow(dead_code)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// # enum Rule {
    /// #     open_paren,
    /// #     closed_paren
    /// # }
    /// # let input = "";
    /// # let pos = Position::from_start(input);
    /// Error::new_from_pos(
    ///     ErrorVariant::ParsingError {
    ///         positives: vec![Rule::open_paren],
    ///         negatives: vec![Rule::closed_paren],
    ///     },
    ///     pos
    /// ).with_path("file.rs");
    /// ```
    pub fn with_path(mut self, path: &str) -> Error<R> {
        self.path = Some(path.to_owned());

        self
    }

    /// Returns the path set using [`Error::with_path()`].
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::error::{Error, ErrorVariant};
    /// # use pest::Position;
    /// # #[allow(non_camel_case_types)]
    /// # #[allow(dead_code)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// # enum Rule {
    /// #     open_paren,
    /// #     closed_paren
    /// # }
    /// # let input = "";
    /// # let pos = Position::from_start(input);
    /// # let error = Error::new_from_pos(
    /// #     ErrorVariant::ParsingError {
    /// #         positives: vec![Rule::open_paren],
    /// #         negatives: vec![Rule::closed_paren],
    /// #     },
    /// #     pos);
    /// let error = error.with_path("file.rs");
    /// assert_eq!(Some("file.rs"), error.path());
    /// ```
    pub fn path(&self) -> Option<&str> {
        self.path.as_deref()
    }

    /// Returns the line that the error is on.
    pub fn line(&self) -> &str {
        self.line.as_str()
    }

    /// Renames all `Rule`s if this is a [`ParsingError`]. It does nothing when called on a
    /// [`CustomError`].
    ///
    /// Useful in order to rename verbose rules or have detailed per-`Rule` formatting.
    ///
    /// [`ParsingError`]: enum.ErrorVariant.html#variant.ParsingError
    /// [`CustomError`]: enum.ErrorVariant.html#variant.CustomError
    ///
    /// # Examples
    ///
    /// ```
    /// # use pest::error::{Error, ErrorVariant};
    /// # use pest::Position;
    /// # #[allow(non_camel_case_types)]
    /// # #[allow(dead_code)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// # enum Rule {
    /// #     open_paren,
    /// #     closed_paren
    /// # }
    /// # let input = "";
    /// # let pos = Position::from_start(input);
    /// Error::new_from_pos(
    ///     ErrorVariant::ParsingError {
    ///         positives: vec![Rule::open_paren],
    ///         negatives: vec![Rule::closed_paren],
    ///     },
    ///     pos
    /// ).renamed_rules(|rule| {
    ///     match *rule {
    ///         Rule::open_paren => "(".to_owned(),
    ///         Rule::closed_paren => "closed paren".to_owned()
    ///     }
    /// });
    /// ```
    pub fn renamed_rules<F>(mut self, f: F) -> Error<R>
    where
        F: FnMut(&R) -> String,
    {
        let variant = match self.variant {
            ErrorVariant::ParsingError {
                positives,
                negatives,
            } => {
                let message = Error::parsing_error_message(&positives, &negatives, f);
                ErrorVariant::CustomError { message }
            }
            variant => variant,
        };

        self.variant = variant;

        self
    }

    /// Get detailed information about errored rules sequence.
    /// Returns `Some(results)` only for `ParsingError`.
    pub fn parse_attempts(&self) -> Option<ParseAttempts<R>> {
        self.parse_attempts.clone()
    }

    /// Get error message based on parsing attempts.
    /// Returns `None` in case self `parse_attempts` is `None`.
    pub fn parse_attempts_error(
        &self,
        input: &str,
        rule_to_message: &RuleToMessageFn<R>,
        is_whitespace: &IsWhitespaceFn,
    ) -> Option<Error<R>> {
        let attempts = if let Some(ref parse_attempts) = self.parse_attempts {
            parse_attempts.clone()
        } else {
            return None;
        };

        let spacing = self.spacing() + "   ";
        let error_position = attempts.max_position;
        let message = {
            let mut help_lines: Vec<String> = Vec::new();
            help_lines.push(String::from("error: parsing error occurred."));

            // Note: at least one of `(un)expected_tokens` must not be empty.
            for tokens_helper_message in attempts.tokens_helper_messages(is_whitespace, &spacing) {
                help_lines.push(tokens_helper_message);
            }

            let call_stacks = attempts.call_stacks();
            // Group call stacks by their parents so that we can print common header and
            // several sub helper messages.
            let mut call_stacks_parents_groups: BTreeMap<Option<R>, Vec<RulesCallStack<R>>> =
                BTreeMap::new();
            for call_stack in call_stacks {
                call_stacks_parents_groups
                    .entry(call_stack.parent)
                    .or_default()
                    .push(call_stack);
            }

            for (group_parent, group) in call_stacks_parents_groups {
                if let Some(parent_rule) = group_parent {
                    let mut contains_meaningful_info = false;
                    help_lines.push(format!(
                        "{spacing}help: {}",
                        if let Some(message) = rule_to_message(&parent_rule) {
                            contains_meaningful_info = true;
                            message
                        } else {
                            String::from("[Unknown parent rule]")
                        }
                    ));
                    for call_stack in group {
                        if let Some(r) = call_stack.deepest.get_rule() {
                            if let Some(message) = rule_to_message(r) {
                                contains_meaningful_info = true;
                                help_lines.push(format!("{spacing}      - {message}"));
                            }
                        }
                    }
                    if !contains_meaningful_info {
                        // Have to remove useless line for unknown parent rule.
                        help_lines.pop();
                    }
                } else {
                    for call_stack in group {
                        // Note that `deepest` rule may be `None`. E.g. in case it corresponds
                        // to WHITESPACE expected token which has no parent rule (on the top level
                        // parsing).
                        if let Some(r) = call_stack.deepest.get_rule() {
                            let helper_message = rule_to_message(r);
                            if let Some(helper_message) = helper_message {
                                help_lines.push(format!("{spacing}help: {helper_message}"));
                            }
                        }
                    }
                }
            }

            help_lines.join("\n")
        };
        let error = Error::new_from_pos(
            ErrorVariant::CustomError { message },
            Position::new_internal(input, error_position),
        );
        Some(error)
    }

    fn start(&self) -> (usize, usize) {
        match self.line_col {
            LineColLocation::Pos(line_col) => line_col,
            LineColLocation::Span(start_line_col, _) => start_line_col,
        }
    }

    fn spacing(&self) -> String {
        let line = match self.line_col {
            LineColLocation::Pos((line, _)) => line,
            LineColLocation::Span((start_line, _), (end_line, _)) => cmp::max(start_line, end_line),
        };

        let line_str_len = format!("{}", line).len();

        let mut spacing = String::new();
        for _ in 0..line_str_len {
            spacing.push(' ');
        }

        spacing
    }

    fn underline(&self) -> String {
        let mut underline = String::new();

        let mut start = self.start().1;
        let end = match self.line_col {
            LineColLocation::Span(_, (_, mut end)) => {
                let inverted_cols = start > end;
                if inverted_cols {
                    mem::swap(&mut start, &mut end);
                    start -= 1;
                    end += 1;
                }

                Some(end)
            }
            _ => None,
        };
        let offset = start - 1;
        let line_chars = self.line.chars();

        for c in line_chars.take(offset) {
            match c {
                '\t' => underline.push('\t'),
                _ => underline.push(' '),
            }
        }

        if let Some(end) = end {
            underline.push('^');
            if end - start > 1 {
                for _ in 2..(end - start) {
                    underline.push('-');
                }
                underline.push('^');
            }
        } else {
            underline.push_str("^---")
        }

        underline
    }

    fn message(&self) -> String {
        self.variant.message().to_string()
    }

    fn parsing_error_message<F>(positives: &[R], negatives: &[R], mut f: F) -> String
    where
        F: FnMut(&R) -> String,
    {
        match (negatives.is_empty(), positives.is_empty()) {
            (false, false) => format!(
                "unexpected {}; expected {}",
                Error::enumerate(negatives, &mut f),
                Error::enumerate(positives, &mut f)
            ),
            (false, true) => format!("unexpected {}", Error::enumerate(negatives, &mut f)),
            (true, false) => format!("expected {}", Error::enumerate(positives, &mut f)),
            (true, true) => "unknown parsing error".to_owned(),
        }
    }

    fn enumerate<F>(rules: &[R], f: &mut F) -> String
    where
        F: FnMut(&R) -> String,
    {
        match rules.len() {
            1 => f(&rules[0]),
            2 => format!("{} or {}", f(&rules[0]), f(&rules[1])),
            l => {
                let non_separated = f(&rules[l - 1]);
                let separated = rules
                    .iter()
                    .take(l - 1)
                    .map(f)
                    .collect::<Vec<_>>()
                    .join(", ");
                format!("{}, or {}", separated, non_separated)
            }
        }
    }

    pub(crate) fn format(&self) -> String {
        let spacing = self.spacing();
        let path = self
            .path
            .as_ref()
            .map(|path| format!("{}:", path))
            .unwrap_or_default();

        let pair = (self.line_col.clone(), &self.continued_line);
        if let (LineColLocation::Span(_, end), Some(ref continued_line)) = pair {
            let has_line_gap = end.0 - self.start().0 > 1;
            if has_line_gap {
                format!(
                    "{s    }--> {p}{ls}:{c}\n\
                     {s    } |\n\
                     {ls:w$} | {line}\n\
                     {s    } | ...\n\
                     {le:w$} | {continued_line}\n\
                     {s    } | {underline}\n\
                     {s    } |\n\
                     {s    } = {message}",
                    s = spacing,
                    w = spacing.len(),
                    p = path,
                    ls = self.start().0,
                    le = end.0,
                    c = self.start().1,
                    line = self.line,
                    continued_line = continued_line,
                    underline = self.underline(),
                    message = self.message()
                )
            } else {
                format!(
                    "{s    }--> {p}{ls}:{c}\n\
                     {s    } |\n\
                     {ls:w$} | {line}\n\
                     {le:w$} | {continued_line}\n\
                     {s    } | {underline}\n\
                     {s    } |\n\
                     {s    } = {message}",
                    s = spacing,
                    w = spacing.len(),
                    p = path,
                    ls = self.start().0,
                    le = end.0,
                    c = self.start().1,
                    line = self.line,
                    continued_line = continued_line,
                    underline = self.underline(),
                    message = self.message()
                )
            }
        } else {
            format!(
                "{s}--> {p}{l}:{c}\n\
                 {s} |\n\
                 {l} | {line}\n\
                 {s} | {underline}\n\
                 {s} |\n\
                 {s} = {message}",
                s = spacing,
                p = path,
                l = self.start().0,
                c = self.start().1,
                line = self.line,
                underline = self.underline(),
                message = self.message()
            )
        }
    }

    #[cfg(feature = "miette-error")]
    /// Turns an error into a [miette](crates.io/miette) Diagnostic.
    pub fn into_miette(self) -> impl ::miette::Diagnostic {
        miette_adapter::MietteAdapter(self)
    }
}

impl<R: RuleType> ErrorVariant<R> {
    ///
    /// Returns the error message for [`ErrorVariant`]
    ///
    /// If [`ErrorVariant`] is [`CustomError`], it returns a
    /// [`Cow::Borrowed`] reference to [`message`]. If [`ErrorVariant`] is [`ParsingError`], a
    /// [`Cow::Owned`] containing "expected [ErrorVariant::ParsingError::positives] [ErrorVariant::ParsingError::negatives]" is returned.
    ///
    /// [`ErrorVariant`]: enum.ErrorVariant.html
    /// [`CustomError`]: enum.ErrorVariant.html#variant.CustomError
    /// [`ParsingError`]: enum.ErrorVariant.html#variant.ParsingError
    /// [`Cow::Owned`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html#variant.Owned
    /// [`Cow::Borrowed`]: https://doc.rust-lang.org/std/borrow/enum.Cow.html#variant.Borrowed
    /// [`message`]: enum.ErrorVariant.html#variant.CustomError.field.message
    /// # Examples
    ///
    /// ```
    /// # use pest::error::ErrorVariant;
    /// let variant = ErrorVariant::<()>::CustomError {
    ///     message: String::from("unexpected error")
    /// };
    ///
    /// println!("{}", variant.message());
    pub fn message(&self) -> Cow<'_, str> {
        match self {
            ErrorVariant::ParsingError {
                ref positives,
                ref negatives,
            } => Cow::Owned(Error::parsing_error_message(positives, negatives, |r| {
                format!("{:?}", r)
            })),
            ErrorVariant::CustomError { ref message } => Cow::Borrowed(message),
        }
    }
}

impl<R: RuleType> fmt::Display for Error<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.format())
    }
}

impl<R: RuleType> fmt::Display for ErrorVariant<R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ErrorVariant::ParsingError { .. } => write!(f, "parsing error: {}", self.message()),
            ErrorVariant::CustomError { .. } => write!(f, "{}", self.message()),
        }
    }
}

fn visualize_whitespace(input: &str) -> String {
    input.to_owned().replace('\r', "␍").replace('\n', "␊")
}

#[cfg(feature = "miette-error")]
mod miette_adapter {
    use alloc::string::ToString;
    use core::fmt;
    use std::boxed::Box;

    use crate::error::LineColLocation;

    use super::{Error, RuleType};

    use miette::{Diagnostic, LabeledSpan, SourceCode};

    #[derive(Debug)]
    pub(crate) struct MietteAdapter<R: RuleType>(pub(crate) Error<R>);

    impl<R: RuleType> Diagnostic for MietteAdapter<R> {
        fn source_code(&self) -> Option<&dyn SourceCode> {
            Some(&self.0.line)
        }

        fn labels(&self) -> Option<Box<dyn Iterator<Item = LabeledSpan>>> {
            let message = self.0.variant.message().to_string();

            let (offset, length) = match self.0.line_col {
                LineColLocation::Pos((_, c)) => (c - 1, 1),
                LineColLocation::Span((_, start_c), (_, end_c)) => {
                    (start_c - 1, end_c - start_c + 1)
                }
            };

            let span = LabeledSpan::new(Some(message), offset, length);

            Some(Box::new(std::iter::once(span)))
        }

        fn help<'a>(&'a self) -> Option<Box<dyn fmt::Display + 'a>> {
            Some(Box::new(self.0.message()))
        }
    }

    impl<R: RuleType> fmt::Display for MietteAdapter<R> {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "Failure to parse at {:?}", self.0.line_col)
        }
    }

    impl<R> core::error::Error for MietteAdapter<R>
    where
        R: RuleType,
        Self: fmt::Debug + fmt::Display,
    {
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use alloc::vec;

    #[test]
    fn display_parsing_error_mixed() {
        let input = "ab\ncd\nef";
        let pos = Position::new(input, 4).unwrap();
        let error: Error<u32> = Error::new_from_pos(
            ErrorVariant::ParsingError {
                positives: vec![1, 2, 3],
                negatives: vec![4, 5, 6],
            },
            pos,
        );

        assert_eq!(
            format!("{}", error),
            [
                " --> 2:2",
                "  |",
                "2 | cd",
                "  |  ^---",
                "  |",
                "  = unexpected 4, 5, or 6; expected 1, 2, or 3"
            ]
            .join("\n")
        );
    }

    #[test]
    fn display_parsing_error_positives() {
        let input = "ab\ncd\nef";
        let pos = Position::new(input, 4).unwrap();
        let error: Error<u32> = Error::new_from_pos(
            ErrorVariant::ParsingError {
                positives: vec![1, 2],
                negatives: vec![],
            },
            pos,
        );

        assert_eq!(
            format!("{}", error),
            [
                " --> 2:2",
                "  |",
                "2 | cd",
                "  |  ^---",
                "  |",
                "  = expected 1 or 2"
            ]
            .join("\n")
        );
    }

    #[test]
    fn display_parsing_error_negatives() {
        let input = "ab\ncd\nef";
        let pos = Position::new(input, 4).unwrap();
        let error: Error<u32> = Error::new_from_pos(
            ErrorVariant::ParsingError {
                positives: vec![],
                negatives: vec![4, 5, 6],
            },
            pos,
        );

        assert_eq!(
            format!("{}", error),
            [
                " --> 2:2",
                "  |",
                "2 | cd",
                "  |  ^---",
                "  |",
                "  = unexpected 4, 5, or 6"
            ]
            .join("\n")
        );
    }

    #[test]
    fn display_parsing_error_unknown() {
        let input = "ab\ncd\nef";
        let pos = Position::new(input, 4).unwrap();
        let error: Error<u32> = Error::new_from_pos(
            ErrorVariant::ParsingError {
                positives: vec![],
                negatives: vec![],
            },
            pos,
        );

        assert_eq!(
            format!("{}", error),
            [
                " --> 2:2",
                "  |",
                "2 | cd",
                "  |  ^---",
                "  |",
                "  = unknown parsing error"
            ]
            .join("\n")
        );
    }

    #[test]
    fn display_custom_pos() {
        let input = "ab\ncd\nef";
        let pos = Position::new(input, 4).unwrap();
        let error: Error<u32> = Error::new_from_pos(
            ErrorVariant::CustomError {
                message: "error: big one".to_owned(),
            },
            pos,
        );

        assert_eq!(
            format!("{}", error),
            [
                " --> 2:2",
                "  |",
                "2 | cd",
                "  |  ^---",
                "  |",
                "  = error: big one"
            ]
            .join("\n")
        );
    }

    #[test]
    fn display_custom_span_two_lines() {
        let input = "ab\ncd\nefgh";
        let start = Position::new(input, 4).unwrap();
        let end = Position::new(input, 9).unwrap();
        let error: Error<u32> = Error::new_from_span(
            ErrorVariant::CustomError {
                message: "error: big one".to_owned(),
            },
            start.span(&end),
        );

        assert_eq!(
            format!("{}", error),
            [
                " --> 2:2",
                "  |",
                "2 | cd",
                "3 | efgh",
                "  |  ^^",
                "  |",
                "  = error: big one"
            ]
            .join("\n")
        );
    }

    #[test]
    fn display_custom_span_three_lines() {
        let input = "ab\ncd\nefgh";
        let start = Position::new(input, 1).unwrap();
        let end = Position::new(input, 9).unwrap();
        let error: Error<u32> = Error::new_from_span(
            ErrorVariant::CustomError {
                message: "error: big one".to_owned(),
            },
            start.span(&end),
        );

        assert_eq!(
            format!("{}", error),
            [
                " --> 1:2",
                "  |",
                "1 | ab",
                "  | ...",
                "3 | efgh",
                "  |  ^^",
                "  |",
                "  = error: big one"
            ]
            .join("\n")
        );
    }

    #[test]
    fn display_custom_span_two_lines_inverted_cols() {
        let input = "abcdef\ngh";
        let start = Position::new(input, 5).unwrap();
        let end = Position::new(input, 8).unwrap();
        let error: Error<u32> = Error::new_from_span(
            ErrorVariant::CustomError {
                message: "error: big one".to_owned(),
            },
            start.span(&end),
        );

        assert_eq!(
            format!("{}", error),
            [
                " --> 1:6",
                "  |",
                "1 | abcdef",
                "2 | gh",
                "  | ^----^",
                "  |",
                "  = error: big one"
            ]
            .join("\n")
        );
    }

    #[test]
    fn display_custom_span_end_after_newline() {
        let input = "abcdef\n";
        let start = Position::new(input, 0).unwrap();
        let end = Position::new(input, 7).unwrap();
        assert!(start.at_start());
        assert!(end.at_end());

        let error: Error<u32> = Error::new_from_span(
            ErrorVariant::CustomError {
                message: "error: big one".to_owned(),
            },
            start.span(&end),
        );

        assert_eq!(
            format!("{}", error),
            [
                " --> 1:1",
                "  |",
                "1 | abcdef␊",
                "  | ^-----^",
                "  |",
                "  = error: big one"
            ]
            .join("\n")
        );
    }

    #[test]
    fn display_custom_span_empty() {
        let input = "";
        let start = Position::new(input, 0).unwrap();
        let end = Position::new(input, 0).unwrap();
        assert!(start.at_start());
        assert!(end.at_end());

        let error: Error<u32> = Error::new_from_span(
            ErrorVariant::CustomError {
                message: "error: empty".to_owned(),
            },
            start.span(&end),
        );

        assert_eq!(
            format!("{}", error),
            [
                " --> 1:1",
                "  |",
                "1 | ",
                "  | ^",
                "  |",
                "  = error: empty"
            ]
            .join("\n")
        );
    }

    #[test]
    fn mapped_parsing_error() {
        let input = "ab\ncd\nef";
        let pos = Position::new(input, 4).unwrap();
        let error: Error<u32> = Error::new_from_pos(
            ErrorVariant::ParsingError {
                positives: vec![1, 2, 3],
                negatives: vec![4, 5, 6],
            },
            pos,
        )
        .renamed_rules(|n| format!("{}", n + 1));

        assert_eq!(
            format!("{}", error),
            [
                " --> 2:2",
                "  |",
                "2 | cd",
                "  |  ^---",
                "  |",
                "  = unexpected 5, 6, or 7; expected 2, 3, or 4"
            ]
            .join("\n")
        );
    }

    #[test]
    fn error_with_path() {
        let input = "ab\ncd\nef";
        let pos = Position::new(input, 4).unwrap();
        let error: Error<u32> = Error::new_from_pos(
            ErrorVariant::ParsingError {
                positives: vec![1, 2, 3],
                negatives: vec![4, 5, 6],
            },
            pos,
        )
        .with_path("file.rs");

        assert_eq!(
            format!("{}", error),
            [
                " --> file.rs:2:2",
                "  |",
                "2 | cd",
                "  |  ^---",
                "  |",
                "  = unexpected 4, 5, or 6; expected 1, 2, or 3"
            ]
            .join("\n")
        );
    }

    #[test]
    fn underline_with_tabs() {
        let input = "a\txbc";
        let pos = Position::new(input, 2).unwrap();
        let error: Error<u32> = Error::new_from_pos(
            ErrorVariant::ParsingError {
                positives: vec![1, 2, 3],
                negatives: vec![4, 5, 6],
            },
            pos,
        )
        .with_path("file.rs");

        assert_eq!(
            format!("{}", error),
            [
                " --> file.rs:1:3",
                "  |",
                "1 | a	xbc",
                "  |  	^---",
                "  |",
                "  = unexpected 4, 5, or 6; expected 1, 2, or 3"
            ]
            .join("\n")
        );
    }

    #[test]
    fn pos_to_lcl_conversion() {
        let input = "input";

        let pos = Position::new(input, 2).unwrap();

        assert_eq!(LineColLocation::Pos(pos.line_col()), pos.into());
    }

    #[test]
    fn span_to_lcl_conversion() {
        let input = "input";

        let span = Span::new(input, 2, 4).unwrap();
        let (start, end) = span.split();

        assert_eq!(
            LineColLocation::Span(start.line_col(), end.line_col()),
            span.into()
        );
    }

    #[cfg(feature = "miette-error")]
    #[test]
    fn miette_error() {
        let input = "abc\ndef";
        let pos = Position::new(input, 4).unwrap();
        let error: Error<u32> = Error::new_from_pos(
            ErrorVariant::ParsingError {
                positives: vec![1, 2, 3],
                negatives: vec![4, 5, 6],
            },
            pos,
        );

        let miette_error = miette::Error::new(error.into_miette());

        assert_eq!(
            format!("{:?}", miette_error),
            [
                "  \u{1b}[31m×\u{1b}[0m Failure to parse at Pos((2, 1))",
                "   ╭────",
                " \u{1b}[2m1\u{1b}[0m │ def",
                "   · \u{1b}[35;1m┬\u{1b}[0m",
                "   · \u{1b}[35;1m╰── \u{1b}[35;1munexpected 4, 5, or 6; expected 1, 2, or 3\u{1b}[0m\u{1b}[0m",
                "   ╰────",
                "\u{1b}[36m  help: \u{1b}[0munexpected 4, 5, or 6; expected 1, 2, or 3\n"
            ]
            .join("\n")
        );
    }
}
