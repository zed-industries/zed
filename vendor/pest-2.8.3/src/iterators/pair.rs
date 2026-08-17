// pest. The Elegant Parser
// Copyright (c) 2018 Dragoș Tiselice
//
// Licensed under the Apache License, Version 2.0
// <LICENSE-APACHE or http://www.apache.org/licenses/LICENSE-2.0> or the MIT
// license <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. All files in the project carrying such notice may not be copied,
// modified, or distributed except according to those terms.

use alloc::format;
use alloc::rc::Rc;
#[cfg(feature = "pretty-print")]
use alloc::string::String;
use alloc::vec::Vec;
use core::borrow::Borrow;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::ptr;
use core::str;

#[cfg(feature = "pretty-print")]
use serde::ser::SerializeStruct;

use super::line_index::LineIndex;
use super::pairs::{self, Pairs};
use super::queueable_token::QueueableToken;
use super::tokens::{self, Tokens};
use crate::span::Span;
use crate::RuleType;

/// A matching pair of [`Token`]s and everything between them.
///
/// A matching `Token` pair is formed by a `Token::Start` and a subsequent `Token::End` with the
/// same `Rule`, with the condition that all `Token`s between them can form such pairs as well.
/// This is similar to the [brace matching problem](https://en.wikipedia.org/wiki/Brace_matching) in
/// editors.
///
/// [`Token`]: ../enum.Token.html
#[derive(Clone)]
pub struct Pair<'i, R> {
    queue: Rc<Vec<QueueableToken<'i, R>>>,
    input: &'i str,
    /// Token index into `queue`.
    start: usize,
    line_index: Rc<LineIndex>,
}

pub fn new<'i, R: RuleType>(
    queue: Rc<Vec<QueueableToken<'i, R>>>,
    input: &'i str,
    line_index: Rc<LineIndex>,
    start: usize,
) -> Pair<'i, R> {
    Pair {
        queue,
        input,
        start,
        line_index,
    }
}

impl<'i, R: RuleType> Pair<'i, R> {
    /// Returns the `Rule` of the `Pair`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a
    /// }
    ///
    /// let input = "";
    /// let pair = pest::state(input, |state| {
    ///     // generating Token pair with Rule::a ...
    /// #     state.rule(Rule::a, |s| Ok(s))
    /// }).unwrap().next().unwrap();
    ///
    /// assert_eq!(pair.as_rule(), Rule::a);
    /// ```
    #[inline]
    pub fn as_rule(&self) -> R {
        match self.queue[self.pair()] {
            QueueableToken::End { rule, .. } => rule,
            _ => unreachable!(),
        }
    }

    /// Captures a slice from the `&str` defined by the token `Pair`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     ab
    /// }
    ///
    /// let input = "ab";
    /// let pair = pest::state(input, |state| {
    ///     // generating Token pair with Rule::ab ...
    /// #     state.rule(Rule::ab, |s| s.match_string("ab"))
    /// }).unwrap().next().unwrap();
    ///
    /// assert_eq!(pair.as_str(), "ab");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &'i str {
        let start = self.pos(self.start);
        let end = self.pos(self.pair());

        // Generated positions always come from Positions and are UTF-8 borders.
        &self.input[start..end]
    }

    /// Returns the input string of the `Pair`.
    ///
    /// This function returns the input string of the `Pair` as a `&str`. This is the source string
    /// from which the `Pair` was created. The returned `&str` can be used to examine the contents of
    /// the `Pair` or to perform further processing on the string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     ab
    /// }
    ///
    /// // Example: Get input string from a Pair
    ///
    /// let input = "ab";
    /// let pair = pest::state(input, |state| {
    ///     // generating Token pair with Rule::ab ...
    /// #     state.rule(Rule::ab, |s| s.match_string("ab"))
    /// }).unwrap().next().unwrap();
    ///
    /// assert_eq!(pair.as_str(), "ab");
    /// assert_eq!(input, pair.get_input());
    /// ```
    pub fn get_input(&self) -> &'i str {
        self.input
    }

    /// Returns the `Span` defined by the `Pair`, consuming it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     ab
    /// }
    ///
    /// let input = "ab";
    /// let pair = pest::state(input, |state| {
    ///     // generating Token pair with Rule::ab ...
    /// #     state.rule(Rule::ab, |s| s.match_string("ab"))
    /// }).unwrap().next().unwrap();
    ///
    /// assert_eq!(pair.into_span().as_str(), "ab");
    /// ```
    #[inline]
    #[deprecated(since = "2.0.0", note = "Please use `as_span` instead")]
    pub fn into_span(self) -> Span<'i> {
        self.as_span()
    }

    /// Returns the `Span` defined by the `Pair`, **without** consuming it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     ab
    /// }
    ///
    /// let input = "ab";
    /// let pair = pest::state(input, |state| {
    ///     // generating Token pair with Rule::ab ...
    /// #     state.rule(Rule::ab, |s| s.match_string("ab"))
    /// }).unwrap().next().unwrap();
    ///
    /// assert_eq!(pair.as_span().as_str(), "ab");
    /// ```
    #[inline]
    pub fn as_span(&self) -> Span<'i> {
        let start = self.pos(self.start);
        let end = self.pos(self.pair());

        Span::new_internal(self.input, start, end)
    }

    /// Get current node tag
    #[inline]
    pub fn as_node_tag(&self) -> Option<&str> {
        match &self.queue[self.pair()] {
            QueueableToken::End { tag, .. } => tag.as_ref().map(|x| x.borrow()),
            _ => None,
        }
    }

    /// Returns the inner `Pairs` between the `Pair`, consuming it.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a
    /// }
    ///
    /// let input = "";
    /// let pair = pest::state(input, |state| {
    ///     // generating Token pair with Rule::a ...
    /// #     state.rule(Rule::a, |s| Ok(s))
    /// }).unwrap().next().unwrap();
    ///
    /// assert!(pair.into_inner().next().is_none());
    /// ```
    #[inline]
    pub fn into_inner(self) -> Pairs<'i, R> {
        let pair = self.pair();

        pairs::new(
            self.queue,
            self.input,
            Some(self.line_index),
            self.start + 1,
            pair,
        )
    }

    /// Returns the `Tokens` for the `Pair`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a
    /// }
    ///
    /// let input = "";
    /// let pair = pest::state(input, |state| {
    ///     // generating Token pair with Rule::a ...
    /// #     state.rule(Rule::a, |s| Ok(s))
    /// }).unwrap().next().unwrap();
    /// let tokens: Vec<_> = pair.tokens().collect();
    ///
    /// assert_eq!(tokens.len(), 2);
    /// ```
    #[inline]
    pub fn tokens(self) -> Tokens<'i, R> {
        let end = self.pair();

        tokens::new(self.queue, self.input, self.start, end + 1)
    }

    /// Generates a string that stores the lexical information of `self` in
    /// a pretty-printed JSON format.
    #[cfg(feature = "pretty-print")]
    pub fn to_json(&self) -> String {
        ::serde_json::to_string_pretty(self).expect("Failed to pretty-print Pair to json.")
    }

    /// Returns the `line`, `col` of this pair start.
    pub fn line_col(&self) -> (usize, usize) {
        let pos = self.pos(self.start);
        self.line_index.line_col(self.input, pos)
    }

    fn pair(&self) -> usize {
        match self.queue[self.start] {
            QueueableToken::Start {
                end_token_index, ..
            } => end_token_index,
            _ => unreachable!(),
        }
    }

    fn pos(&self, index: usize) -> usize {
        match self.queue[index] {
            QueueableToken::Start { input_pos, .. } | QueueableToken::End { input_pos, .. } => {
                input_pos
            }
        }
    }
}

impl<'i, R: RuleType> Pairs<'i, R> {
    /// Create a new `Pairs` iterator containing just the single `Pair`.
    pub fn single(pair: Pair<'i, R>) -> Self {
        let end = pair.pair();
        pairs::new(
            pair.queue,
            pair.input,
            Some(pair.line_index),
            pair.start,
            end,
        )
    }
}

impl<'i, R: RuleType> fmt::Debug for Pair<'i, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let pair = &mut f.debug_struct("Pair");
        pair.field("rule", &self.as_rule());
        // In order not to break compatibility
        if let Some(s) = self.as_node_tag() {
            pair.field("node_tag", &s);
        }
        pair.field("span", &self.as_span())
            .field("inner", &self.clone().into_inner().collect::<Vec<_>>())
            .finish()
    }
}

impl<'i, R: RuleType> fmt::Display for Pair<'i, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let rule = self.as_rule();
        let start = self.pos(self.start);
        let end = self.pos(self.pair());
        let mut pairs = self.clone().into_inner().peekable();

        if pairs.peek().is_none() {
            write!(f, "{:?}({}, {})", rule, start, end)
        } else {
            write!(
                f,
                "{:?}({}, {}, [{}])",
                rule,
                start,
                end,
                pairs
                    .map(|pair| format!("{}", pair))
                    .collect::<Vec<_>>()
                    .join(", ")
            )
        }
    }
}

impl<'i, R: PartialEq> PartialEq for Pair<'i, R> {
    fn eq(&self, other: &Pair<'i, R>) -> bool {
        Rc::ptr_eq(&self.queue, &other.queue)
            && ptr::eq(self.input, other.input)
            && self.start == other.start
    }
}

impl<'i, R: Eq> Eq for Pair<'i, R> {}

impl<'i, R: Hash> Hash for Pair<'i, R> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&*self.queue as *const Vec<QueueableToken<'i, R>>).hash(state);
        (self.input as *const str).hash(state);
        self.start.hash(state);
    }
}

#[cfg(feature = "pretty-print")]
impl<'i, R: RuleType> ::serde::Serialize for Pair<'i, R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        let start = self.pos(self.start);
        let end = self.pos(self.pair());
        let rule = format!("{:?}", self.as_rule());
        let inner = self.clone().into_inner();

        let mut ser = serializer.serialize_struct("Pairs", 3)?;
        ser.serialize_field("pos", &(start, end))?;
        ser.serialize_field("rule", &rule)?;

        if inner.peek().is_none() {
            ser.serialize_field("inner", &self.as_str())?;
        } else {
            ser.serialize_field("inner", &inner)?;
        }

        ser.end()
    }
}

#[cfg(test)]
mod tests {
    use crate::macros::tests::*;
    use crate::parser::Parser;

    #[test]
    #[cfg(feature = "pretty-print")]
    fn test_pretty_print() {
        let pair = AbcParser::parse(Rule::a, "abcde").unwrap().next().unwrap();

        let expected = r#"{
  "pos": [
    0,
    3
  ],
  "rule": "a",
  "inner": {
    "pos": [
      1,
      2
    ],
    "pairs": [
      {
        "pos": [
          1,
          2
        ],
        "rule": "b",
        "inner": "b"
      }
    ]
  }
}"#;

        assert_eq!(expected, pair.to_json());
    }

    #[test]
    fn pair_into_inner() {
        let pair = AbcParser::parse(Rule::a, "abcde").unwrap().next().unwrap(); // the tokens a(b())

        let pairs = pair.into_inner(); // the tokens b()

        assert_eq!(2, pairs.tokens().count());
    }

    #[test]
    fn get_input_of_pair() {
        let input = "abcde";
        let pair = AbcParser::parse(Rule::a, input).unwrap().next().unwrap();

        assert_eq!(input, pair.get_input());
    }
}
