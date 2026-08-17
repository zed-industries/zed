use std::{fmt, marker::PhantomData};

use crate::ParseError;

use regex_automata::hybrid::dfa::{Cache, DFA};
use regex_automata::hybrid::{BuildError, LazyStateID};
use regex_automata::nfa::thompson::Config as NfaConfig;
use regex_automata::util::syntax::Config as SyntaxConfig;
use regex_automata::{Anchored, Input, MatchKind};

#[derive(Clone, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct Token<'input>(pub usize, pub &'input str);
impl<'a> fmt::Display for Token<'a> {
    fn fmt(&self, formatter: &mut fmt::Formatter) -> Result<(), fmt::Error> {
        fmt::Display::fmt(self.1, formatter)
    }
}

pub struct MatcherBuilder {
    dfa: DFA,
    skip_vec: Vec<bool>,
}

impl MatcherBuilder {
    #[allow(clippy::result_large_err)]
    pub fn new<S>(exprs: impl IntoIterator<Item = (S, bool)>) -> Result<MatcherBuilder, BuildError>
    where
        S: AsRef<str>,
    {
        let exprs = exprs.into_iter();
        let mut regex_vec = Vec::with_capacity(exprs.size_hint().0);
        let mut skip_vec = Vec::with_capacity(exprs.size_hint().0);
        for (regex, skip) in exprs {
            regex_vec.push(regex);
            skip_vec.push(skip);
        }

        let enable_unicode = cfg!(feature = "unicode");
        let dfa = DFA::builder()
            .configure(DFA::config().match_kind(MatchKind::All))
            .syntax(
                SyntaxConfig::new()
                    .unicode(enable_unicode)
                    .utf8(enable_unicode),
            )
            .thompson(NfaConfig::new().utf8(enable_unicode).shrink(true))
            .build_many(&regex_vec)?;

        Ok(MatcherBuilder { dfa, skip_vec })
    }

    pub fn matcher<'input, 'builder, E>(
        &'builder self,
        text: &'input str,
    ) -> Matcher<'input, 'builder, E> {
        let input = Input::new(text).anchored(Anchored::Yes);
        let mut cache = self.dfa.create_cache();
        let start = self.dfa.start_state_forward(&mut cache, &input).unwrap();
        Matcher {
            text,
            consumed: 0,
            cache,
            start,
            dfa: &self.dfa,
            skip_vec: &self.skip_vec,
            _marker: PhantomData,
        }
    }
}

pub struct Matcher<'input, 'builder, E> {
    text: &'input str,
    consumed: usize,
    cache: Cache,
    start: LazyStateID,
    dfa: &'builder DFA,
    skip_vec: &'builder [bool],
    _marker: PhantomData<fn() -> E>,
}

impl<'input, 'builder, E> Iterator for Matcher<'input, 'builder, E> {
    type Item = Result<(usize, Token<'input>, usize), ParseError<usize, Token<'input>, E>>;

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            let text = self.text;
            let start_offset = self.consumed;
            if text.is_empty() {
                self.consumed = start_offset;
                return None;
            }

            let mut match_ = None;
            'search: {
                let mut state = self.start;
                for (i, byte) in text.bytes().enumerate() {
                    state = self.dfa.next_state(&mut self.cache, state, byte).unwrap();
                    if state.is_match() {
                        match_ = Some((state, i));
                    } else if state.is_dead() {
                        break 'search;
                    }
                }
                state = self.dfa.next_eoi_state(&mut self.cache, state).unwrap();
                if state.is_match() {
                    match_ = Some((state, text.len()));
                }
            }

            let (match_state, longest_match) = match match_ {
                Some(match_) => match_,
                None => {
                    return Some(Err(ParseError::InvalidToken {
                        location: start_offset,
                    }))
                }
            };
            let index = (0..self.dfa.match_len(&self.cache, match_state))
                .map(|n| {
                    self.dfa
                        .match_pattern(&self.cache, match_state, n)
                        .as_usize()
                })
                .max()
                .unwrap();

            let result = &text[..longest_match];
            let remaining = &text[longest_match..];
            let end_offset = start_offset + longest_match;
            self.text = remaining;
            self.consumed = end_offset;

            if self.skip_vec[index] {
                if longest_match == 0 {
                    return Some(Err(ParseError::InvalidToken {
                        location: start_offset,
                    }));
                }
                continue;
            }

            return Some(Ok((start_offset, Token(index, result), end_offset)));
        }
    }
}
