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
use alloc::string::String;
use alloc::vec::Vec;
use core::fmt;
use core::hash::{Hash, Hasher};
use core::iter::Filter;
use core::ptr;
use core::str;

#[cfg(feature = "pretty-print")]
use serde::ser::SerializeStruct;

use super::flat_pairs::{self, FlatPairs};
use super::line_index::LineIndex;
use super::pair::{self, Pair};
use super::queueable_token::QueueableToken;
use super::tokens::{self, Tokens};
use crate::RuleType;

/// An iterator over [`Pair`]s. It is created by [`pest::state`] and [`Pair::into_inner`].
///
/// [`Pair`]: struct.Pair.html
/// [`pest::state`]: ../fn.state.html
/// [`Pair::into_inner`]: struct.Pair.html#method.into_inner
#[derive(Clone)]
pub struct Pairs<'i, R> {
    queue: Rc<Vec<QueueableToken<'i, R>>>,
    input: &'i str,
    start: usize,
    end: usize,
    pairs_count: usize,
    line_index: Rc<LineIndex>,
}

pub fn new<'i, R: RuleType>(
    queue: Rc<Vec<QueueableToken<'i, R>>>,
    input: &'i str,
    line_index: Option<Rc<LineIndex>>,
    start: usize,
    end: usize,
) -> Pairs<'i, R> {
    let line_index = match line_index {
        Some(line_index) => line_index,
        None => {
            let last_input_pos = queue
                .last()
                .map(|token| match *token {
                    QueueableToken::Start { input_pos, .. }
                    | QueueableToken::End { input_pos, .. } => input_pos,
                })
                .unwrap_or(0);

            Rc::new(LineIndex::new(&input[..last_input_pos]))
        }
    };

    let mut pairs_count = 0;
    let mut cursor = start;
    while cursor < end {
        cursor = match queue[cursor] {
            QueueableToken::Start {
                end_token_index, ..
            } => end_token_index,
            _ => unreachable!(),
        } + 1;
        pairs_count += 1;
    }

    Pairs {
        queue,
        input,
        start,
        end,
        pairs_count,
        line_index,
    }
}

impl<'i, R: RuleType> Pairs<'i, R> {
    /// Captures a slice from the `&str` defined by the starting position of the first token `Pair`
    /// and the ending position of the last token `Pair` of the `Pairs`. This also captures
    /// the input between those two token `Pair`s.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a,
    ///     b
    /// }
    ///
    /// let input = "a b";
    /// let pairs = pest::state(input, |state| {
    ///     // generating Token pairs with Rule::a and Rule::b ...
    /// #     state.rule(Rule::a, |s| s.match_string("a")).and_then(|s| s.skip(1))
    /// #         .and_then(|s| s.rule(Rule::b, |s| s.match_string("b")))
    /// }).unwrap();
    ///
    /// assert_eq!(pairs.as_str(), "a b");
    /// ```
    #[inline]
    pub fn as_str(&self) -> &'i str {
        if self.start < self.end {
            let start = self.pos(self.start);
            let end = self.pos(self.end - 1);
            // Generated positions always come from Positions and are UTF-8 borders.
            &self.input[start..end]
        } else {
            ""
        }
    }

    /// Returns the input string of `Pairs`.
    ///
    /// This function returns the input string of `Pairs` as a `&str`. This is the source string
    /// from which `Pairs` was created. The returned `&str` can be used to examine the contents of
    /// `Pairs` or to perform further processing on the string.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a,
    ///     b
    /// }
    ///
    /// // Example: Get input string from Pairs
    ///
    /// let input = "a b";
    /// let pairs = pest::state(input, |state| {
    ///     // generating Token pairs with Rule::a and Rule::b ...
    /// #     state.rule(Rule::a, |s| s.match_string("a")).and_then(|s| s.skip(1))
    /// #         .and_then(|s| s.rule(Rule::b, |s| s.match_string("b")))
    /// }).unwrap();
    ///
    /// assert_eq!(pairs.as_str(), "a b");
    /// assert_eq!(input, pairs.get_input());
    /// ```
    pub fn get_input(&self) -> &'i str {
        self.input
    }

    /// Captures inner token `Pair`s and concatenates resulting `&str`s. This does not capture
    /// the input between token `Pair`s.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a,
    ///     b
    /// }
    ///
    /// let input = "a b";
    /// let pairs = pest::state(input, |state| {
    ///     // generating Token pairs with Rule::a and Rule::b ...
    /// #     state.rule(Rule::a, |s| s.match_string("a")).and_then(|s| s.skip(1))
    /// #         .and_then(|s| s.rule(Rule::b, |s| s.match_string("b")))
    /// }).unwrap();
    ///
    /// assert_eq!(pairs.concat(), "ab");
    /// ```
    #[inline]
    pub fn concat(&self) -> String {
        self.clone()
            .fold(String::new(), |string, pair| string + pair.as_str())
    }

    /// Flattens the `Pairs`.
    ///
    /// # Examples
    ///
    /// ```
    /// # use std::rc::Rc;
    /// # use pest;
    /// # #[allow(non_camel_case_types)]
    /// # #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     a,
    ///     b
    /// }
    ///
    /// let input = "";
    /// let pairs = pest::state(input, |state| {
    ///     // generating nested Token pair with Rule::b inside Rule::a
    /// #     state.rule(Rule::a, |state| {
    /// #         state.rule(Rule::b, |s| Ok(s))
    /// #     })
    /// }).unwrap();
    /// let tokens: Vec<_> = pairs.flatten().tokens().collect();
    ///
    /// assert_eq!(tokens.len(), 4);
    /// ```
    #[inline]
    pub fn flatten(self) -> FlatPairs<'i, R> {
        flat_pairs::new(
            self.queue,
            self.input,
            self.line_index,
            self.start,
            self.end,
        )
    }

    /// Finds the first pair that has its node or branch tagged with the provided
    /// label. Searches in the flattened [`Pairs`] iterator.
    ///
    /// # Examples
    ///
    /// Try to recognize the branch between add and mul
    /// ```
    /// use pest::{state, ParseResult, ParserState};
    /// #[allow(non_camel_case_types)]
    /// #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     number, // 0..9
    ///     add,    // num + num
    ///     mul,    // num * num
    /// }
    /// fn mark_branch(
    ///     state: Box<ParserState<'_, Rule>>,
    /// ) -> ParseResult<Box<ParserState<'_, Rule>>> {
    ///     expr(state, Rule::mul, "*")
    ///         .and_then(|state| state.tag_node("mul"))
    ///         .or_else(|state| expr(state, Rule::add, "+"))
    ///         .and_then(|state| state.tag_node("add"))
    /// }
    /// fn expr<'a>(
    ///     state: Box<ParserState<'a, Rule>>,
    ///     r: Rule,
    ///     o: &'static str,
    /// ) -> ParseResult<Box<ParserState<'a, Rule>>> {
    ///     state.rule(r, |state| {
    ///         state.sequence(|state| {
    ///             number(state)
    ///                 .and_then(|state| state.tag_node("lhs"))
    ///                 .and_then(|state| state.match_string(o))
    ///                 .and_then(number)
    ///                 .and_then(|state| state.tag_node("rhs"))
    ///         })
    ///     })
    /// }
    /// fn number(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
    ///     state.rule(Rule::number, |state| state.match_range('0'..'9'))
    /// }
    /// let input = "1+2";
    /// let pairs = state(input, mark_branch).unwrap();
    /// assert_eq!(pairs.find_first_tagged("add").unwrap().as_rule(), Rule::add);
    /// assert_eq!(pairs.find_first_tagged("mul"), None);
    /// ```
    #[inline]
    pub fn find_first_tagged(&self, tag: &'i str) -> Option<Pair<'i, R>> {
        self.clone().find_tagged(tag).next()
    }

    /// Returns the iterator over pairs that have their node or branch tagged
    /// with the provided label. The iterator is built from a flattened [`Pairs`] iterator.
    ///
    /// # Examples
    ///
    /// Try to recognize the node between left and right hand side
    /// ```
    /// use pest::{state, ParseResult, ParserState};
    /// #[allow(non_camel_case_types)]
    /// #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
    /// enum Rule {
    ///     number, // 0..9
    ///     add,    // num + num
    ///     mul,    // num * num
    /// }
    /// fn mark_branch(
    ///     state: Box<ParserState<'_, Rule>>,
    /// ) -> ParseResult<Box<ParserState<'_, Rule>>> {
    ///     expr(state, Rule::mul, "*")
    ///         .and_then(|state| state.tag_node("mul"))
    ///         .or_else(|state| expr(state, Rule::add, "+"))
    ///         .and_then(|state| state.tag_node("add"))
    /// }
    /// fn expr<'a>(
    ///     state: Box<ParserState<'a, Rule>>,
    ///     r: Rule,
    ///     o: &'static str,
    /// ) -> ParseResult<Box<ParserState<'a, Rule>>> {
    ///     state.rule(r, |state| {
    ///         state.sequence(|state| {
    ///             number(state)
    ///                 .and_then(|state| state.tag_node("lhs"))
    ///                 .and_then(|state| state.match_string(o))
    ///                 .and_then(number)
    ///                 .and_then(|state| state.tag_node("rhs"))
    ///         })
    ///     })
    /// }
    /// fn number(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
    ///     state.rule(Rule::number, |state| state.match_range('0'..'9'))
    /// }
    ///
    /// let input = "1+2";
    /// let pairs = state(input, mark_branch).unwrap();
    /// let mut left_numbers = pairs.find_tagged("lhs");
    /// assert_eq!(left_numbers.next().unwrap().as_str(), "1");
    /// assert_eq!(left_numbers.next(), None);
    /// ```
    #[inline]
    pub fn find_tagged(
        self,
        tag: &'i str,
    ) -> Filter<FlatPairs<'i, R>, impl FnMut(&Pair<'i, R>) -> bool + 'i> {
        self.flatten()
            .filter(move |pair: &Pair<'i, R>| matches!(pair.as_node_tag(), Some(nt) if nt == tag))
    }

    /// Returns the `Tokens` for the `Pairs`.
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
    /// let pairs = pest::state(input, |state| {
    ///     // generating Token pair with Rule::a ...
    /// #     state.rule(Rule::a, |s| Ok(s))
    /// }).unwrap();
    /// let tokens: Vec<_> = pairs.tokens().collect();
    ///
    /// assert_eq!(tokens.len(), 2);
    /// ```
    #[inline]
    pub fn tokens(self) -> Tokens<'i, R> {
        tokens::new(self.queue, self.input, self.start, self.end)
    }

    /// Peek at the first inner `Pair` without changing the position of this iterator.
    #[inline]
    pub fn peek(&self) -> Option<Pair<'i, R>> {
        if self.start < self.end {
            Some(pair::new(
                Rc::clone(&self.queue),
                self.input,
                Rc::clone(&self.line_index),
                self.start,
            ))
        } else {
            None
        }
    }

    /// Generates a string that stores the lexical information of `self` in
    /// a pretty-printed JSON format.
    #[cfg(feature = "pretty-print")]
    pub fn to_json(&self) -> String {
        ::serde_json::to_string_pretty(self).expect("Failed to pretty-print Pairs to json.")
    }

    fn pair(&self) -> usize {
        match self.queue[self.start] {
            QueueableToken::Start {
                end_token_index, ..
            } => end_token_index,
            _ => unreachable!(),
        }
    }

    fn pair_from_end(&self) -> usize {
        match self.queue[self.end - 1] {
            QueueableToken::End {
                start_token_index, ..
            } => start_token_index,
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

impl<'i, R: RuleType> ExactSizeIterator for Pairs<'i, R> {
    #[inline]
    fn len(&self) -> usize {
        self.pairs_count
    }
}

impl<'i, R: RuleType> Iterator for Pairs<'i, R> {
    type Item = Pair<'i, R>;

    fn next(&mut self) -> Option<Self::Item> {
        let pair = self.peek()?;

        self.start = self.pair() + 1;
        self.pairs_count -= 1;
        Some(pair)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = <Self as ExactSizeIterator>::len(self);
        (len, Some(len))
    }
}

impl<'i, R: RuleType> DoubleEndedIterator for Pairs<'i, R> {
    fn next_back(&mut self) -> Option<Self::Item> {
        if self.end <= self.start {
            return None;
        }

        self.end = self.pair_from_end();
        self.pairs_count -= 1;

        let pair = pair::new(
            Rc::clone(&self.queue),
            self.input,
            Rc::clone(&self.line_index),
            self.end,
        );

        Some(pair)
    }
}

impl<'i, R: RuleType> fmt::Debug for Pairs<'i, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.clone()).finish()
    }
}

impl<'i, R: RuleType> fmt::Display for Pairs<'i, R> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "[{}]",
            self.clone()
                .map(|pair| format!("{}", pair))
                .collect::<Vec<_>>()
                .join(", ")
        )
    }
}

impl<'i, R: PartialEq> PartialEq for Pairs<'i, R> {
    fn eq(&self, other: &Pairs<'i, R>) -> bool {
        Rc::ptr_eq(&self.queue, &other.queue)
            && ptr::eq(self.input, other.input)
            && self.start == other.start
            && self.end == other.end
    }
}

impl<'i, R: Eq> Eq for Pairs<'i, R> {}

impl<'i, R: Hash> Hash for Pairs<'i, R> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        (&*self.queue as *const Vec<QueueableToken<'i, R>>).hash(state);
        (self.input as *const str).hash(state);
        self.start.hash(state);
        self.end.hash(state);
    }
}

#[cfg(feature = "pretty-print")]
impl<'i, R: RuleType> ::serde::Serialize for Pairs<'i, R> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: ::serde::Serializer,
    {
        let start = self.pos(self.start);
        let end = self.pos(self.end - 1);
        let pairs = self.clone().collect::<Vec<_>>();

        let mut ser = serializer.serialize_struct("Pairs", 2)?;
        ser.serialize_field("pos", &(start, end))?;
        ser.serialize_field("pairs", &pairs)?;
        ser.end()
    }
}

#[cfg(test)]
mod tests {
    use super::super::super::macros::tests::*;
    use super::super::super::Parser;
    use alloc::borrow::ToOwned;
    use alloc::boxed::Box;
    use alloc::format;
    use alloc::vec;
    use alloc::vec::Vec;

    #[test]
    #[cfg(feature = "pretty-print")]
    fn test_pretty_print() {
        let pairs = AbcParser::parse(Rule::a, "abcde").unwrap();

        let expected = r#"{
  "pos": [
    0,
    5
  ],
  "pairs": [
    {
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
    },
    {
      "pos": [
        4,
        5
      ],
      "rule": "c",
      "inner": "e"
    }
  ]
}"#;

        assert_eq!(expected, pairs.to_json());
    }

    #[test]
    fn as_str() {
        let pairs = AbcParser::parse(Rule::a, "abcde").unwrap();

        assert_eq!(pairs.as_str(), "abcde");
    }

    #[test]
    fn get_input_of_pairs() {
        let input = "abcde";
        let pairs = AbcParser::parse(Rule::a, input).unwrap();

        assert_eq!(pairs.get_input(), input);
    }

    #[test]
    fn as_str_empty() {
        let mut pairs = AbcParser::parse(Rule::a, "abcde").unwrap();

        assert_eq!(pairs.nth(1).unwrap().into_inner().as_str(), "");
    }

    #[test]
    fn concat() {
        let pairs = AbcParser::parse(Rule::a, "abcde").unwrap();

        assert_eq!(pairs.concat(), "abce");
    }

    #[test]
    fn pairs_debug() {
        let pairs = AbcParser::parse(Rule::a, "abcde").unwrap();

        #[rustfmt::skip]
        assert_eq!(
            format!("{:?}", pairs),
            "[\
                Pair { rule: a, span: Span { str: \"abc\", start: 0, end: 3 }, inner: [\
                    Pair { rule: b, span: Span { str: \"b\", start: 1, end: 2 }, inner: [] }\
                ] }, \
                Pair { rule: c, span: Span { str: \"e\", start: 4, end: 5 }, inner: [] }\
            ]"
            .to_owned()
        );
    }

    #[test]
    fn pairs_display() {
        let pairs = AbcParser::parse(Rule::a, "abcde").unwrap();

        assert_eq!(
            format!("{}", pairs),
            "[a(0, 3, [b(1, 2)]), c(4, 5)]".to_owned()
        );
    }

    #[test]
    fn iter_for_pairs() {
        let pairs = AbcParser::parse(Rule::a, "abcde").unwrap();
        assert_eq!(
            pairs.map(|p| p.as_rule()).collect::<Vec<Rule>>(),
            vec![Rule::a, Rule::c]
        );
    }

    #[test]
    fn double_ended_iter_for_pairs() {
        let pairs = AbcParser::parse(Rule::a, "abcde").unwrap();
        assert_eq!(
            pairs.rev().map(|p| p.as_rule()).collect::<Vec<Rule>>(),
            vec![Rule::c, Rule::a]
        );
    }

    #[test]
    fn test_line_col() {
        let mut pairs = AbcParser::parse(Rule::a, "abc\nefgh").unwrap();
        let pair = pairs.next().unwrap();
        assert_eq!(pair.as_str(), "abc");
        assert_eq!(pair.line_col(), (1, 1));

        let pair = pairs.next().unwrap();
        assert_eq!(pair.as_str(), "e");
        assert_eq!(pair.line_col(), (2, 1));

        let pair = pairs.next().unwrap();
        assert_eq!(pair.as_str(), "fgh");
        assert_eq!(pair.line_col(), (2, 2));
    }

    #[test]
    fn test_rev_iter_line_col() {
        let mut pairs = AbcParser::parse(Rule::a, "abc\nefgh").unwrap().rev();
        let pair = pairs.next().unwrap();
        assert_eq!(pair.as_str(), "fgh");
        assert_eq!(pair.line_col(), (2, 2));

        let pair = pairs.next().unwrap();
        assert_eq!(pair.as_str(), "e");
        assert_eq!(pair.line_col(), (2, 1));

        let pair = pairs.next().unwrap();
        assert_eq!(pair.as_str(), "abc");
        assert_eq!(pair.line_col(), (1, 1));
    }

    #[test]
    // false positive: pest uses `..` as a complete range (historically)
    #[allow(clippy::almost_complete_range)]
    fn test_tag_node_branch() {
        use crate::{state, ParseResult, ParserState};
        #[allow(non_camel_case_types)]
        #[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
        enum Rule {
            number, // 0..9
            add,    // num + num
            mul,    // num * num
        }
        fn mark_branch(
            state: Box<ParserState<'_, Rule>>,
        ) -> ParseResult<Box<ParserState<'_, Rule>>> {
            expr(state, Rule::mul, "*")
                .and_then(|state| state.tag_node("mul"))
                .or_else(|state| expr(state, Rule::add, "+"))
                .and_then(|state| state.tag_node("add"))
        }
        fn expr<'a>(
            state: Box<ParserState<'a, Rule>>,
            r: Rule,
            o: &'static str,
        ) -> ParseResult<Box<ParserState<'a, Rule>>> {
            state.rule(r, |state| {
                state.sequence(|state| {
                    number(state)
                        .and_then(|state| state.tag_node("lhs"))
                        .and_then(|state| state.match_string(o))
                        .and_then(number)
                        .and_then(|state| state.tag_node("rhs"))
                })
            })
        }
        fn number(state: Box<ParserState<'_, Rule>>) -> ParseResult<Box<ParserState<'_, Rule>>> {
            state.rule(Rule::number, |state| state.match_range('0'..'9'))
        }
        let input = "1+2";
        let pairs = state(input, mark_branch).unwrap();
        assert_eq!(pairs.find_first_tagged("add").unwrap().as_rule(), Rule::add);
        assert_eq!(pairs.find_first_tagged("mul"), None);

        let mut left_numbers = pairs.clone().find_tagged("lhs");

        assert_eq!(left_numbers.next().unwrap().as_str(), "1");
        assert_eq!(left_numbers.next(), None);
        let mut right_numbers = pairs.find_tagged("rhs");

        assert_eq!(right_numbers.next().unwrap().as_str(), "2");
        assert_eq!(right_numbers.next(), None);
    }

    #[test]
    fn exact_size_iter_for_pairs() {
        let pairs = AbcParser::parse(Rule::a, "abc\nefgh").unwrap();
        assert_eq!(pairs.len(), pairs.count());

        let pairs = AbcParser::parse(Rule::a, "abc\nefgh").unwrap().rev();
        assert_eq!(pairs.len(), pairs.count());

        let mut pairs = AbcParser::parse(Rule::a, "abc\nefgh").unwrap();
        let pairs_len = pairs.len();
        let _ = pairs.next().unwrap();
        assert_eq!(pairs.count() + 1, pairs_len);
    }
}
