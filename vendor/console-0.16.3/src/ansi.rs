#[cfg(feature = "alloc")]
use alloc::{borrow::Cow, string::String};
use core::{
    fmt::Display,
    iter::{FusedIterator, Peekable},
    str::CharIndices,
};

#[derive(Debug, Clone, Copy, Default)]
enum State {
    #[default]
    Start,
    S1,
    S2,
    S3,
    S4,
    S5,
    S6,
    S7,
    S8,
    S9,
    S10,
    S11,
    Trap,
}

impl State {
    fn is_final(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            Self::S3 | Self::S5 | Self::S6 | Self::S7 | Self::S8 | Self::S9 | Self::S11 => true,
            _ => false,
        }
    }

    fn is_trapped(&self) -> bool {
        #[allow(clippy::match_like_matches_macro)]
        match self {
            Self::Trap => true,
            _ => false,
        }
    }

    fn transition(&mut self, c: char) {
        *self = match c {
            '\u{1b}' | '\u{9b}' => match self {
                Self::Start => Self::S1,
                _ => Self::Trap,
            },
            '(' | ')' => match self {
                Self::S1 => Self::S2,
                Self::S2 | Self::S4 => Self::S4,
                _ => Self::Trap,
            },
            ';' => match self {
                Self::S1 | Self::S2 | Self::S4 => Self::S4,
                Self::S5 | Self::S6 | Self::S7 | Self::S8 | Self::S10 => Self::S10,
                _ => Self::Trap,
            },

            '[' | '#' | '?' => match self {
                Self::S1 | Self::S2 | Self::S4 => Self::S4,
                _ => Self::Trap,
            },
            '0'..='2' => match self {
                Self::S1 | Self::S4 => Self::S5,
                Self::S2 => Self::S3,
                Self::S5 => Self::S6,
                Self::S6 => Self::S7,
                Self::S7 => Self::S8,
                Self::S8 => Self::S9,
                Self::S10 => Self::S5,
                _ => Self::Trap,
            },
            '3'..='9' => match self {
                Self::S1 | Self::S4 => Self::S5,
                Self::S2 => Self::S5,
                Self::S5 => Self::S6,
                Self::S6 => Self::S7,
                Self::S7 => Self::S8,
                Self::S8 => Self::S9,
                Self::S10 => Self::S5,
                _ => Self::Trap,
            },
            'A'..='P' | 'R' | 'Z' | 'c' | 'f'..='n' | 'q' | 'r' | 'y' | '=' | '>' | '<' => {
                match self {
                    Self::S1
                    | Self::S2
                    | Self::S4
                    | Self::S5
                    | Self::S6
                    | Self::S7
                    | Self::S8
                    | Self::S10 => Self::S11,
                    _ => Self::Trap,
                }
            }
            _ => Self::Trap,
        };
    }
}

#[derive(Debug)]
struct Matches<'a> {
    s: &'a str,
    it: Peekable<CharIndices<'a>>,
}

impl<'a> Matches<'a> {
    fn new(s: &'a str) -> Self {
        let it = s.char_indices().peekable();
        Self { s, it }
    }
}

#[derive(Debug)]
struct Match<'a> {
    text: &'a str,
    start: usize,
    end: usize,
}

impl<'a> Match<'a> {
    #[inline]
    pub(crate) fn as_str(&self) -> &'a str {
        &self.text[self.start..self.end]
    }
}

impl<'a> Iterator for Matches<'a> {
    type Item = Match<'a>;

    fn next(&mut self) -> Option<Self::Item> {
        find_ansi_code_exclusive(&mut self.it).map(|(start, end)| Match {
            text: self.s,
            start,
            end,
        })
    }
}

impl FusedIterator for Matches<'_> {}

fn find_ansi_code_exclusive(it: &mut Peekable<CharIndices>) -> Option<(usize, usize)> {
    'outer: loop {
        if let (start, '\u{1b}') | (start, '\u{9b}') = it.peek()? {
            let start = *start;
            let mut state = State::default();
            let mut maybe_end = None;

            loop {
                let item = it.peek();

                if let Some((idx, c)) = item {
                    state.transition(*c);

                    if state.is_final() {
                        maybe_end = Some(*idx);
                    }
                }

                // The match is greedy so run till we hit the trap state no matter what. A valid
                // match is just one that was final at some point
                if state.is_trapped() || item.is_none() {
                    match maybe_end {
                        Some(end) => {
                            // All possible final characters are a single byte so it's safe to make
                            // the end exclusive by just adding one
                            return Some((start, end + 1));
                        }
                        // The character we are peeking right now might be the start of a match so
                        // we want to continue the loop without popping off that char
                        None => continue 'outer,
                    }
                }

                it.next();
            }
        }

        it.next();
    }
}

/// Helper function to strip ansi codes.
#[cfg(feature = "alloc")]
pub fn strip_ansi_codes(s: &str) -> Cow<'_, str> {
    let mut char_it = s.char_indices().peekable();
    match find_ansi_code_exclusive(&mut char_it) {
        Some(_) => {
            let stripped: String = AnsiCodeIterator::new(s)
                .filter_map(|(text, is_ansi)| if is_ansi { None } else { Some(text) })
                .collect();
            Cow::Owned(stripped)
        }
        None => Cow::Borrowed(s),
    }
}

/// A wrapper struct that implements [`core::fmt::Display`], only displaying non-ansi parts.
pub struct WithoutAnsi<'a> {
    str: &'a str,
}

impl<'a> WithoutAnsi<'a> {
    pub fn new(str: &'a str) -> Self {
        Self { str }
    }
}

impl Display for WithoutAnsi<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        for (str, is_ansi) in AnsiCodeIterator::new(self.str) {
            if !is_ansi {
                f.write_str(str)?;
            }
        }
        Ok(())
    }
}

/// An iterator over ansi codes in a string.
///
/// This type can be used to scan over ansi codes in a string.
/// It yields tuples in the form `(s, is_ansi)` where `s` is a slice of
/// the original string and `is_ansi` indicates if the slice contains
/// ansi codes or string values.
pub struct AnsiCodeIterator<'a> {
    s: &'a str,
    pending_item: Option<(&'a str, bool)>,
    last_idx: usize,
    cur_idx: usize,
    iter: Matches<'a>,
}

impl<'a> AnsiCodeIterator<'a> {
    /// Creates a new ansi code iterator.
    pub fn new(s: &'a str) -> AnsiCodeIterator<'a> {
        AnsiCodeIterator {
            s,
            pending_item: None,
            last_idx: 0,
            cur_idx: 0,
            iter: Matches::new(s),
        }
    }

    /// Returns the string slice up to the current match.
    pub fn current_slice(&self) -> &str {
        &self.s[..self.cur_idx]
    }

    /// Returns the string slice from the current match to the end.
    pub fn rest_slice(&self) -> &str {
        &self.s[self.cur_idx..]
    }
}

impl<'a> Iterator for AnsiCodeIterator<'a> {
    type Item = (&'a str, bool);

    fn next(&mut self) -> Option<(&'a str, bool)> {
        if let Some(pending_item) = self.pending_item.take() {
            self.cur_idx += pending_item.0.len();
            Some(pending_item)
        } else if let Some(m) = self.iter.next() {
            let s = &self.s[self.last_idx..m.start];
            self.last_idx = m.end;
            if s.is_empty() {
                self.cur_idx = m.end;
                Some((m.as_str(), true))
            } else {
                self.cur_idx = m.start;
                self.pending_item = Some((m.as_str(), true));
                Some((s, false))
            }
        } else if self.last_idx < self.s.len() {
            let rv = &self.s[self.last_idx..];
            self.cur_idx = self.s.len();
            self.last_idx = self.s.len();
            Some((rv, false))
        } else {
            None
        }
    }
}

impl FusedIterator for AnsiCodeIterator<'_> {}

#[cfg(test)]
mod tests {
    use super::*;

    use core::fmt::Write;
    use proptest::prelude::*;
    use regex::Regex;
    use std::sync::OnceLock;

    // The manual dfa `State` is a handwritten translation from the previously used regex. That
    // regex is kept here and used to ensure that the new matches are the same as the old
    fn strip_ansi_re() -> &'static Regex {
        static RE: OnceLock<Regex> = OnceLock::new();

        RE.get_or_init(|| Regex::new(
            r"[\x1b\x9b]([()][012AB]|[\[()#;?]*(?:[0-9]{1,4}(?:;[0-9]{0,4})*)?[0-9A-PRZcf-nqry=><])",
        ).unwrap())
    }

    impl<'a> PartialEq<Match<'a>> for regex::Match<'_> {
        fn eq(&self, other: &Match<'a>) -> bool {
            self.start() == other.start && self.end() == other.end
        }
    }

    proptest! {
        #[test]
        fn dfa_matches_old_regex(s in r"([\x1b\x9b]?.*){0,5}") {
            let old_matches: Vec<_> = strip_ansi_re().find_iter(&s).collect();
            let new_matches: Vec<_> = Matches::new(&s).collect();
            assert_eq!(old_matches, new_matches);
        }
    }

    #[test]
    fn dfa_matches_regex_on_small_strings() {
        // To make sure the test runs in a reasonable time this is a slimmed down list of
        // characters to reduce the groups that are only used with each other along with one
        // arbitrarily chosen character not used in the regex (' ')
        const POSSIBLE_BYTES: &[u8] = &[b' ', 0x1b, 0x9b, b'(', b'0', b'[', b';', b'3', b'C'];

        fn check_all_strings_of_len(len: usize) {
            _check_all_strings_of_len(len, &mut Vec::with_capacity(len));
        }

        fn _check_all_strings_of_len(len: usize, chunk: &mut Vec<u8>) {
            if len == 0 {
                if let Ok(s) = core::str::from_utf8(chunk) {
                    let old_matches: Vec<_> = strip_ansi_re().find_iter(s).collect();
                    let new_matches: Vec<_> = Matches::new(s).collect();
                    assert_eq!(old_matches, new_matches);
                }

                return;
            }

            for b in POSSIBLE_BYTES {
                chunk.push(*b);
                _check_all_strings_of_len(len - 1, chunk);
                chunk.pop();
            }
        }

        for str_len in 0..=6 {
            check_all_strings_of_len(str_len);
        }
    }

    #[test]
    fn complex_data() {
        let s = std::fs::read_to_string(
            std::path::Path::new("tests")
                .join("data")
                .join("sample_zellij_session.log"),
        )
        .unwrap();

        let old_matches: Vec<_> = strip_ansi_re().find_iter(&s).collect();
        let new_matches: Vec<_> = Matches::new(&s).collect();
        assert_eq!(old_matches, new_matches);
    }

    #[test]
    fn state_machine() {
        let ansi_code = "\x1b)B";
        let mut state = State::default();
        assert!(!state.is_final());

        for c in ansi_code.chars() {
            state.transition(c);
        }
        assert!(state.is_final());

        state.transition('A');
        assert!(state.is_trapped());
    }

    #[test]
    fn back_to_back_entry_char() {
        let s = "\x1b\x1bf";
        let matches: Vec<_> = Matches::new(s).map(|m| m.as_str()).collect();
        assert_eq!(&["\x1bf"], matches.as_slice());
    }

    #[test]
    fn early_paren_can_use_many_chars() {
        let s = "\x1b(C";
        let matches: Vec<_> = Matches::new(s).map(|m| m.as_str()).collect();
        assert_eq!(&[s], matches.as_slice());
    }

    #[test]
    fn long_run_of_digits() {
        let s = "\u{1b}00000";
        let matches: Vec<_> = Matches::new(s).map(|m| m.as_str()).collect();
        assert_eq!(&[s], matches.as_slice());
    }

    #[test]
    fn test_without_ansi() {
        let str_with_ansi = "\x1b[1;97;41mError\x1b[0m";
        let without_ansi = WithoutAnsi::new(str_with_ansi);
        for _ in 0..2 {
            let mut output = String::default();
            write!(output, "{without_ansi}").unwrap();
            assert_eq!(output, "Error");
        }
    }

    #[test]
    fn test_ansi_iter_re_vt100() {
        let s = "\x1b(0lpq\x1b)Benglish";
        let mut iter = AnsiCodeIterator::new(s);
        assert_eq!(iter.next(), Some(("\x1b(0", true)));
        assert_eq!(iter.next(), Some(("lpq", false)));
        assert_eq!(iter.next(), Some(("\x1b)B", true)));
        assert_eq!(iter.next(), Some(("english", false)));
    }

    #[test]
    fn test_ansi_iter_re() {
        use crate::style;
        let s = format!("Hello {}!", style("World").red().force_styling(true));
        let mut iter = AnsiCodeIterator::new(&s);
        assert_eq!(iter.next(), Some(("Hello ", false)));
        assert_eq!(iter.current_slice(), "Hello ");
        assert_eq!(iter.rest_slice(), "\x1b[31mWorld\x1b[0m!");
        assert_eq!(iter.next(), Some(("\x1b[31m", true)));
        assert_eq!(iter.current_slice(), "Hello \x1b[31m");
        assert_eq!(iter.rest_slice(), "World\x1b[0m!");
        assert_eq!(iter.next(), Some(("World", false)));
        assert_eq!(iter.current_slice(), "Hello \x1b[31mWorld");
        assert_eq!(iter.rest_slice(), "\x1b[0m!");
        assert_eq!(iter.next(), Some(("\x1b[0m", true)));
        assert_eq!(iter.current_slice(), "Hello \x1b[31mWorld\x1b[0m");
        assert_eq!(iter.rest_slice(), "!");
        assert_eq!(iter.next(), Some(("!", false)));
        assert_eq!(iter.current_slice(), "Hello \x1b[31mWorld\x1b[0m!");
        assert_eq!(iter.rest_slice(), "");
        assert_eq!(iter.next(), None);
    }

    #[test]
    fn test_ansi_iter_re_on_multi() {
        use crate::style;
        let s = format!("{}", style("a").red().bold().force_styling(true));
        let mut iter = AnsiCodeIterator::new(&s);
        assert_eq!(iter.next(), Some(("\x1b[31m", true)));
        assert_eq!(iter.current_slice(), "\x1b[31m");
        assert_eq!(iter.rest_slice(), "\x1b[1ma\x1b[0m");
        assert_eq!(iter.next(), Some(("\x1b[1m", true)));
        assert_eq!(iter.current_slice(), "\x1b[31m\x1b[1m");
        assert_eq!(iter.rest_slice(), "a\x1b[0m");
        assert_eq!(iter.next(), Some(("a", false)));
        assert_eq!(iter.current_slice(), "\x1b[31m\x1b[1ma");
        assert_eq!(iter.rest_slice(), "\x1b[0m");
        assert_eq!(iter.next(), Some(("\x1b[0m", true)));
        assert_eq!(iter.current_slice(), "\x1b[31m\x1b[1ma\x1b[0m");
        assert_eq!(iter.rest_slice(), "");
        assert_eq!(iter.next(), None);
    }
}
