// Copyright 2026 The Fancy Regex Authors.
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

#![doc = include_str!("../docs/main.md")]
#![doc = include_str!("../docs/features.md")]
#![doc = include_str!("../docs/syntax.md")]
#![doc = include_str!("../docs/subroutines/1_intro.md")]
#![doc = include_str!("../docs/subroutines/2_flags.md")]
#![doc = include_str!("../docs/subroutines/3_left_recursion.md")]
#![doc = include_str!("../docs/subroutines/4_recursion.md")]
#![doc = include_str!("../docs/absent.md")]
#![deny(missing_docs)]
#![deny(missing_debug_implementations)]
#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

use alloc::borrow::Cow;
use alloc::boxed::Box;
use alloc::string::{String, ToString};
use alloc::sync::Arc;
use alloc::vec;
use alloc::vec::Vec;

use core::convert::TryFrom;
use core::fmt;
use core::fmt::{Debug, Formatter};
use core::ops::{Index, Range};
use core::str::FromStr;
use regex_automata::meta::Regex as RaRegex;
use regex_automata::util::captures::Captures as RaCaptures;
use regex_automata::util::syntax::Config as SyntaxConfig;
use regex_automata::Input as RaInput;

mod analyze;
mod compile;
mod error;
mod expand;
mod optimize;
mod parse;
mod parse_flags;
mod replacer;
mod vm;

use crate::analyze::can_compile_as_anchored;
use crate::analyze::{analyze, AnalyzeContext};
use crate::compile::{compile, CompileOptions};
use crate::optimize::optimize;
use crate::parse::{ExprTree, NamedGroups, Parser};
use crate::parse_flags::*;
use crate::vm::{Prog, OPTION_FIND_NOT_EMPTY, OPTION_SKIPPED_EMPTY_MATCH};

pub use crate::error::{CompileError, Error, ParseError, Result, RuntimeError};
pub use crate::expand::Expander;
pub use crate::replacer::{NoExpand, Replacer, ReplacerRef};

const MAX_RECURSION: usize = 64;

// the public API

/// A builder for a `Regex` to allow configuring options.
#[derive(Debug)]
pub struct RegexBuilder {
    pattern: String,
    options: RegexOptionsBuilder,
}

/// A builder for a `Regex` to allow configuring options.
#[derive(Debug)]
pub struct RegexOptionsBuilder {
    options: RegexOptions,
}

/// A compiled regular expression.
#[derive(Clone)]
pub struct Regex {
    inner: RegexImpl,
    named_groups: Arc<NamedGroups>,
}

// Separate enum because we don't want to expose any of this
#[derive(Clone)]
enum RegexImpl {
    // Do we want to box this? It's pretty big...
    Wrap {
        inner: RaRegex,
        /// The original pattern which the regex was constructed from
        pattern: String,
        /// Some optimizations avoid the VM, but need to use an extra capture group to represent the match boundaries
        explicit_capture_group_0: bool,
        /// The actual pattern passed to regex-automata for delegation
        delegated_pattern: String,
    },
    Fancy {
        prog: Arc<Prog>,
        n_groups: usize,
        /// The original pattern which the regex was constructed from
        pattern: String,
        options: HardRegexRuntimeOptions,
    },
}

/// A single match of a regex or group in an input text
#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct Match<'t> {
    text: &'t str,
    start: usize,
    end: usize,
}

/// An iterator over all non-overlapping matches for a particular string.
///
/// The iterator yields a `Result<Match>`. The iterator stops when no more
/// matches can be found.
///
/// `'r` is the lifetime of the compiled regular expression and `'t` is the
/// lifetime of the matched string.
#[derive(Debug)]
pub struct Matches<'r, 't> {
    re: &'r Regex,
    text: &'t str,
    last_end: usize,
    last_match: Option<usize>,
    last_skipped_empty: bool,
}

impl<'r, 't> Matches<'r, 't> {
    /// Return the text being searched.
    pub fn text(&self) -> &'t str {
        self.text
    }

    /// Return the underlying regex.
    pub fn regex(&self) -> &'r Regex {
        self.re
    }

    /// Adapted from the `regex` crate. Calls `find_from_pos`/`captures_from_pos` repeatedly.
    /// Ignores empty matches immediately after a match.
    /// Also passes a flag when skipping an empty match, so that \G wouldn't match at the new start position.
    fn next_with<F, R>(&mut self, mut search: F) -> Option<Result<R>>
    where
        F: FnMut(&Regex, usize, u32) -> Result<Option<(R, Match<'t>)>>,
    {
        if self.last_end > self.text.len() {
            return None;
        }

        let option_flags = if self.last_skipped_empty {
            OPTION_SKIPPED_EMPTY_MATCH
        } else {
            0
        };

        let pos = self.last_end;
        let (result, mat) = match search(self.re, pos, option_flags) {
            Err(error) => {
                // Stop on first error: If an error is encountered, return it, and set the "last match position"
                // to the string length, so that the next next() call will return None, to prevent an infinite loop.
                self.last_end = self.text.len() + 1;
                return Some(Err(error));
            }
            Ok(None) => return None,
            Ok(Some(pair)) => pair,
        };

        if mat.start == mat.end {
            // This is an empty match. To ensure we make progress, start
            // the next search at the smallest possible starting position
            // of the next match following this one.
            self.last_end = next_utf8(self.text, mat.end);
            // Only set OPTION_SKIPPED_EMPTY_MATCH on the next call if this was a
            // truly zero-length match (the VM consumed no bytes from `pos`).
            // This means that \K won't prevent \G from matching.
            self.last_skipped_empty = mat.end == pos;
            // Don't accept empty matches immediately following a match.
            // Just move on to the next match.
            if Some(mat.end) == self.last_match {
                return self.next_with(search);
            }
        } else {
            self.last_end = mat.end;
            self.last_skipped_empty = false;
        }

        self.last_match = Some(mat.end);

        Some(Ok(result))
    }
}

impl<'r, 't> Iterator for Matches<'r, 't> {
    type Item = Result<Match<'t>>;

    fn next(&mut self) -> Option<Self::Item> {
        let text = self.text;
        self.next_with(move |re, pos, flags| {
            re.find_from_pos_with_option_flags(text, pos, flags)
                .map(|opt| opt.map(|m| (m, m)))
        })
    }
}

/// An iterator that yields all non-overlapping capture groups matching a
/// particular regular expression.
///
/// The iterator stops when no more matches can be found.
///
/// `'r` is the lifetime of the compiled regular expression and `'t` is the
/// lifetime of the matched string.
#[derive(Debug)]
pub struct CaptureMatches<'r, 't>(Matches<'r, 't>);

impl<'r, 't> CaptureMatches<'r, 't> {
    /// Return the text being searched.
    pub fn text(&self) -> &'t str {
        self.0.text
    }

    /// Return the underlying regex.
    pub fn regex(&self) -> &'r Regex {
        self.0.re
    }
}

impl<'r, 't> Iterator for CaptureMatches<'r, 't> {
    type Item = Result<Captures<'t>>;

    fn next(&mut self) -> Option<Self::Item> {
        let text = self.0.text;
        self.0.next_with(move |re, pos, flags| {
            let captures = re.captures_from_pos_with_option_flags(text, pos, flags)?;
            Ok(captures.map(|c| {
                let mat = c
                    .get(0)
                    .expect("`Captures` is expected to have entire match at 0th position");
                (c, mat)
            }))
        })
    }
}

/// A set of capture groups found for a regex.
#[derive(Debug)]
pub struct Captures<'t> {
    inner: CapturesImpl<'t>,
    named_groups: Arc<NamedGroups>,
}

#[derive(Debug)]
enum CapturesImpl<'t> {
    Wrap {
        text: &'t str,
        locations: RaCaptures,
        /// Some optimizations avoid the VM but need an extra capture group to represent the match boundaries.
        /// Therefore what is actually capture group 1 should be treated as capture group 0, and all other
        /// capture groups should have their index reduced by one as well to line up with what the pattern specifies.
        explicit_capture_group_0: bool,
    },
    Fancy {
        text: &'t str,
        saves: Vec<usize>,
    },
}

/// Iterator for captured groups in order in which they appear in the regex.
#[derive(Debug)]
pub struct SubCaptureMatches<'c, 't> {
    caps: &'c Captures<'t>,
    i: usize,
}

/// An iterator over all substrings delimited by a regex.
///
/// This iterator yields `Result<&'h str>`, where each item is a substring of the
/// target string that is delimited by matches of the regular expression. It stops when there
/// are no more substrings to yield.
///
/// `'r` is the lifetime of the compiled regular expression, and `'h` is the
/// lifetime of the target string being split.
///
/// This iterator can be created by the [`Regex::split`] method.
#[derive(Debug)]
pub struct Split<'r, 'h> {
    matches: Matches<'r, 'h>,
    next_start: usize,
    target: &'h str,
}

impl<'r, 'h> Iterator for Split<'r, 'h> {
    type Item = Result<&'h str>;

    /// Returns the next substring that results from splitting the target string by the regex.
    ///
    /// If no more matches are found, returns the remaining part of the string,
    /// or `None` if all substrings have been yielded.
    fn next(&mut self) -> Option<Result<&'h str>> {
        match self.matches.next() {
            None => {
                let len = self.target.len();
                if self.next_start > len {
                    // No more substrings to return
                    None
                } else {
                    // Return the last part of the target string
                    // Next call will return None
                    let part = &self.target[self.next_start..len];
                    self.next_start = len + 1;
                    Some(Ok(part))
                }
            }
            // Return the next substring
            Some(Ok(m)) => {
                let part = &self.target[self.next_start..m.start()];
                self.next_start = m.end();
                Some(Ok(part))
            }
            Some(Err(e)) => Some(Err(e)),
        }
    }
}

impl<'r, 'h> core::iter::FusedIterator for Split<'r, 'h> {}

/// An iterator over at most `N` substrings delimited by a regex.
///
/// This iterator yields `Result<&'h str>`, where each item is a substring of the
/// target that is delimited by matches of the regular expression. It stops either when
/// there are no more substrings to yield, or after `N` substrings have been yielded.
///
/// The `N`th substring is the remaining part of the target.
///
/// `'r` is the lifetime of the compiled regular expression, and `'h` is the
/// lifetime of the target string being split.
///
/// This iterator can be created by the [`Regex::splitn`] method.
#[derive(Debug)]
pub struct SplitN<'r, 'h> {
    splits: Split<'r, 'h>,
    limit: usize,
}

impl<'r, 'h> Iterator for SplitN<'r, 'h> {
    type Item = Result<&'h str>;

    /// Returns the next substring resulting from splitting the target by the regex,
    /// limited to `N` splits.
    ///
    /// Returns `None` if no more matches are found or if the limit is reached after yielding
    /// the remaining part of the target.
    fn next(&mut self) -> Option<Result<&'h str>> {
        if self.limit == 0 {
            // Limit reached. No more substrings available.
            return None;
        }

        // Decrement the limit for each split.
        self.limit -= 1;
        if self.limit > 0 {
            return self.splits.next();
        }

        // Nth split
        let len = self.splits.target.len();
        if self.splits.next_start > len {
            // No more substrings available.
            None
        } else {
            // Return the remaining part of the target
            let start = self.splits.next_start;
            self.splits.next_start = len + 1;
            Some(Ok(&self.splits.target[start..len]))
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, Some(self.limit))
    }
}

impl<'r, 'h> core::iter::FusedIterator for SplitN<'r, 'h> {}

#[derive(Clone, Debug, Default)]
struct RegexOptions {
    syntaxc: SyntaxConfig,
    delegate_size_limit: Option<usize>,
    delegate_dfa_size_limit: Option<usize>,
    oniguruma_mode: bool,
    ignore_numbered_groups_when_named_groups_exist: bool,
    hard_regex_runtime_options: HardRegexRuntimeOptions,
}

#[derive(Copy, Clone, Debug)]
struct HardRegexRuntimeOptions {
    backtrack_limit: usize,
    find_not_empty: bool,
}

impl RegexOptions {
    fn get_flag_value(flag_value: bool, enum_value: u32) -> u32 {
        if flag_value {
            enum_value
        } else {
            0
        }
    }

    fn compute_flags(&self) -> u32 {
        let insensitive = Self::get_flag_value(self.syntaxc.get_case_insensitive(), FLAG_CASEI);
        let multiline = Self::get_flag_value(self.syntaxc.get_multi_line(), FLAG_MULTI);
        let whitespace =
            Self::get_flag_value(self.syntaxc.get_ignore_whitespace(), FLAG_IGNORE_SPACE);
        let dotnl = Self::get_flag_value(self.syntaxc.get_dot_matches_new_line(), FLAG_DOTNL);
        let unicode = Self::get_flag_value(self.syntaxc.get_unicode(), FLAG_UNICODE);
        let oniguruma_mode = Self::get_flag_value(self.oniguruma_mode, FLAG_ONIGURUMA_MODE);
        let crlf = Self::get_flag_value(self.syntaxc.get_crlf(), FLAG_CRLF);
        let named_groups_only = Self::get_flag_value(
            self.ignore_numbered_groups_when_named_groups_exist,
            FLAG_IGNORE_NUMBERED_GROUPS_WHEN_NAMED_GROUPS_EXIST,
        );

        insensitive
            | multiline
            | whitespace
            | dotnl
            | unicode
            | oniguruma_mode
            | crlf
            | named_groups_only
    }
}

impl Default for HardRegexRuntimeOptions {
    fn default() -> Self {
        HardRegexRuntimeOptions {
            backtrack_limit: 1_000_000,
            find_not_empty: false,
        }
    }
}

impl Default for RegexOptionsBuilder {
    fn default() -> Self {
        Self::new()
    }
}

impl RegexOptionsBuilder {
    /// Create a new regex options builder.
    pub fn new() -> Self {
        RegexOptionsBuilder {
            options: RegexOptions::default(),
        }
    }

    /// Build a `Regex` from the given pattern.
    ///
    /// Returns an [`Error`](enum.Error.html) if the pattern could not be parsed.
    pub fn build(&self, pattern: String) -> Result<Regex> {
        Regex::new_options(pattern, &self.options)
    }

    fn set_config(&mut self, func: impl Fn(SyntaxConfig) -> SyntaxConfig) -> &mut Self {
        self.options.syntaxc = func(self.options.syntaxc);
        self
    }

    /// Override default case insensitive
    /// this is to enable/disable casing via builder instead of a flag within
    /// the raw string pattern which will be parsed
    ///
    /// Default is false
    pub fn case_insensitive(&mut self, yes: bool) -> &mut Self {
        self.set_config(|x| x.case_insensitive(yes))
    }

    /// Enable multi-line regex
    pub fn multi_line(&mut self, yes: bool) -> &mut Self {
        self.set_config(|x| x.multi_line(yes))
    }

    /// Allow ignore whitespace
    pub fn ignore_whitespace(&mut self, yes: bool) -> &mut Self {
        self.set_config(|x| x.ignore_whitespace(yes))
    }

    /// Enable or disable the "dot matches any character" flag.
    /// When this is enabled, `.` will match any character. When it's disabled, then `.` will match any character
    /// except for a new line character.
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut Self {
        self.set_config(|x| x.dot_matches_new_line(yes))
    }

    /// Enable or disable the CRLF mode flag (`R`).
    ///
    /// When enabled, `\r\n` is treated as a single line ending for the purposes of
    /// `^` and `$` in multi-line mode, instead of treating `\r` and `\n` as separate
    /// line endings.
    ///
    /// By default, this is disabled. It may be selectively enabled in the regular
    /// expression by using the `R` flag, e.g. `(?mR)` or `(?Rm)`.
    pub fn crlf(&mut self, yes: bool) -> &mut Self {
        self.set_config(|x| x.crlf(yes))
    }

    /// Enable verbose mode in the regular expression.
    ///
    /// The same as ignore_whitespace
    ///
    /// When enabled, verbose mode permits insigificant whitespace in many
    /// places in the regular expression, as well as comments. Comments are
    /// started using `#` and continue until the end of the line.
    ///
    /// By default, this is disabled. It may be selectively enabled in the
    /// regular expression by using the `x` flag regardless of this setting.
    pub fn verbose_mode(&mut self, yes: bool) -> &mut Self {
        self.set_config(|x| x.ignore_whitespace(yes))
    }

    /// Enable or disable the Unicode flag (`u`) by default.
    ///
    /// By default this is **enabled**. It may alternatively be selectively
    /// disabled in the regular expression itself via the `u` flag.
    ///
    /// Note that unless "allow invalid UTF-8" is enabled (it's disabled by
    /// default), a regular expression will fail to parse if Unicode mode is
    /// disabled and a sub-expression could possibly match invalid UTF-8.
    ///
    /// **WARNING**: Unicode mode can greatly increase the size of the compiled
    /// DFA, which can noticeably impact both memory usage and compilation
    /// time. This is especially noticeable if your regex contains character
    /// classes like `\w` that are impacted by whether Unicode is enabled or
    /// not. If Unicode is not necessary, you are encouraged to disable it.
    pub fn unicode_mode(&mut self, yes: bool) -> &mut Self {
        self.set_config(|x| x.unicode(yes))
    }

    /// Limit for how many times backtracking should be attempted for fancy regexes (where
    /// backtracking is used). If this limit is exceeded, execution returns an error with
    /// [`Error::BacktrackLimitExceeded`](enum.Error.html#variant.BacktrackLimitExceeded).
    /// This is for preventing a regex with catastrophic backtracking to run for too long.
    ///
    /// Default is `1_000_000` (1 million).
    pub fn backtrack_limit(&mut self, limit: usize) -> &mut Self {
        self.options.hard_regex_runtime_options.backtrack_limit = limit;
        self
    }

    /// Set the approximate size limit of the compiled regular expression.
    ///
    /// This option is forwarded from the wrapped `regex` crate. Note that depending on the used
    /// regex features there may be multiple delegated sub-regexes fed to the `regex` crate. As
    /// such the actual limit is closer to `<number of delegated regexes> * delegate_size_limit`.
    pub fn delegate_size_limit(&mut self, limit: usize) -> &mut Self {
        self.options.delegate_size_limit = Some(limit);
        self
    }

    /// Set the approximate size of the cache used by the DFA.
    ///
    /// This option is forwarded from the wrapped `regex` crate. Note that depending on the used
    /// regex features there may be multiple delegated sub-regexes fed to the `regex` crate. As
    /// such the actual limit is closer to `<number of delegated regexes> *
    /// delegate_dfa_size_limit`.
    pub fn delegate_dfa_size_limit(&mut self, limit: usize) -> &mut Self {
        self.options.delegate_dfa_size_limit = Some(limit);
        self
    }

    /// Require that matches are non-empty (i.e. match at least one character).
    ///
    /// When this is enabled, any match attempt that would result in a zero-length match is
    /// rejected.
    ///
    /// Default is `false`.
    ///
    /// N.B. When `find_not_empty` is set and analysis determines the pattern will only ever
    /// produce an empty match, compiling the regex will return
    /// `CompileError::PatternCanNeverMatch` instead of silently constructing a regex that can never
    /// return a result. This catches the user error at compile time rather than allowing the
    /// combination to execute pointlessly at runtime.
    pub fn find_not_empty(&mut self, yes: bool) -> &mut Self {
        self.options.hard_regex_runtime_options.find_not_empty = yes;
        self
    }

    /// Treat unnamed capture groups as non-capturing when named groups exist.
    /// Prevents accessing capture groups by number from within the pattern
    /// (backrefs, subroutine calls) when named groups are present.
    pub fn ignore_numbered_groups_when_named_groups_exist(&mut self, yes: bool) -> &mut Self {
        self.options.ignore_numbered_groups_when_named_groups_exist = yes;
        self
    }

    /// Attempts to better match [Oniguruma](https://github.com/kkos/oniguruma)'s default behavior
    ///
    /// Currently this amounts to changing behavior with:
    ///
    /// # Left and right word bounds
    ///
    /// `fancy-regex` follows the default of other regex engines such as the `regex` crate itself
    /// where `\<` and `\>` correspond to a _left_ and _right_ word-bound respectively. This
    /// differs from Oniguruma's defaults which treat them as matching the literals `<` and `>`.
    /// When this option is set using `\<` and `\>` in the pattern will match the literals
    /// `<` and `>` instead of word bounds.
    ///
    /// # Repetition/Quantifiers on empty groups
    ///
    /// `fancy-regex` would normally reject patterns like `(?:)+` because the `+` has nothing
    /// to target. In Oniguruma mode, the empty repeat is silently dropped at parse time.
    ///
    /// ## Example
    ///
    /// ```
    /// use fancy_regex::{Regex, RegexBuilder};
    ///
    /// let haystack = "turbo::<Fish>";
    /// let regex = r"\<\w*\>";
    ///
    /// // By default `\<` and `\>` will match the start and end of a word boundary
    /// let word_bounds_regex = Regex::new(regex).unwrap();
    /// let word_bounds = word_bounds_regex.find(haystack).unwrap().unwrap();
    /// assert_eq!(word_bounds.as_str(), "turbo");
    ///
    /// // With the option set they instead match the literal `<` and `>` characters
    /// let literals_regex = RegexBuilder::new(regex).oniguruma_mode(true).build().unwrap();
    /// let literals = literals_regex.find(haystack).unwrap().unwrap();
    /// assert_eq!(literals.as_str(), "<Fish>");
    /// ```
    pub fn oniguruma_mode(&mut self, yes: bool) -> &mut Self {
        self.options.oniguruma_mode = yes;
        self
    }
}

impl RegexBuilder {
    /// Create a new regex builder.
    pub fn new(pattern: &str) -> Self {
        RegexBuilder {
            pattern: pattern.to_string(),
            options: RegexOptionsBuilder::new(),
        }
    }

    /// Build a `Regex` from the given pattern.
    ///
    /// Returns an [`Error`](enum.Error.html) if the pattern could not be parsed.
    pub fn build(&self) -> Result<Regex> {
        self.options.build(self.pattern.clone())
    }

    /// Change the pattern to build. Useful when building multiple regexes from
    /// many patterns.
    pub fn pattern(&mut self, pattern: String) -> &mut Self {
        self.pattern = pattern;
        self
    }

    /// See [`RegexOptionsBuilder::case_insensitive`]
    pub fn case_insensitive(&mut self, yes: bool) -> &mut Self {
        self.options.case_insensitive(yes);
        self
    }

    /// See [`RegexOptionsBuilder::multi_line`]
    pub fn multi_line(&mut self, yes: bool) -> &mut Self {
        self.options.multi_line(yes);
        self
    }

    /// See [`RegexOptionsBuilder::ignore_whitespace`]
    pub fn ignore_whitespace(&mut self, yes: bool) -> &mut Self {
        self.options.ignore_whitespace(yes);
        self
    }

    /// See [`RegexOptionsBuilder::dot_matches_new_line`]
    pub fn dot_matches_new_line(&mut self, yes: bool) -> &mut Self {
        self.options.dot_matches_new_line(yes);
        self
    }

    /// See [`RegexOptionsBuilder::verbose_mode`]
    pub fn verbose_mode(&mut self, yes: bool) -> &mut Self {
        self.options.ignore_whitespace(yes);
        self
    }

    /// See [`RegexOptionsBuilder::unicode_mode`]
    pub fn unicode_mode(&mut self, yes: bool) -> &mut Self {
        self.options.unicode_mode(yes);
        self
    }

    /// See [`RegexOptionsBuilder::backtrack_limit`]
    pub fn backtrack_limit(&mut self, limit: usize) -> &mut Self {
        self.options.backtrack_limit(limit);
        self
    }

    /// See [`RegexOptionsBuilder::delegate_size_limit`]
    pub fn delegate_size_limit(&mut self, limit: usize) -> &mut Self {
        self.options.delegate_size_limit(limit);
        self
    }

    /// See [`RegexOptionsBuilder::delegate_dfa_size_limit`]
    pub fn delegate_dfa_size_limit(&mut self, limit: usize) -> &mut Self {
        self.options.delegate_dfa_size_limit(limit);
        self
    }

    /// See [`RegexOptionsBuilder::oniguruma_mode`]
    pub fn oniguruma_mode(&mut self, yes: bool) -> &mut Self {
        self.options.oniguruma_mode(yes);
        self
    }

    /// See [`RegexOptionsBuilder::crlf`]
    pub fn crlf(&mut self, yes: bool) -> &mut Self {
        self.options.crlf(yes);
        self
    }

    /// See [`RegexOptionsBuilder::find_not_empty`]
    pub fn find_not_empty(&mut self, yes: bool) -> &mut Self {
        self.options.find_not_empty(yes);
        self
    }

    /// See [`RegexOptionsBuilder::ignore_numbered_groups_when_named_groups_exist`]
    pub fn ignore_numbered_groups_when_named_groups_exist(&mut self, yes: bool) -> &mut Self {
        self.options
            .ignore_numbered_groups_when_named_groups_exist(yes);
        self
    }
}

impl fmt::Debug for Regex {
    /// Shows the original regular expression.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl fmt::Display for Regex {
    /// Shows the original regular expression
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}

impl FromStr for Regex {
    type Err = Error;

    /// Attempts to parse a string into a regular expression
    fn from_str(s: &str) -> Result<Regex> {
        Regex::new(s)
    }
}

impl Regex {
    /// Parse and compile a regex with default options, see `RegexBuilder`.
    ///
    /// Returns an [`Error`](enum.Error.html) if the pattern could not be parsed.
    pub fn new(re: &str) -> Result<Regex> {
        Self::new_options(re.to_string(), &RegexOptions::default())
    }

    fn new_options(pattern: String, options: &RegexOptions) -> Result<Regex> {
        let mut tree = Expr::parse_tree_with_flags(&pattern, options.compute_flags())?;

        let find_not_empty = options.hard_regex_runtime_options.find_not_empty;

        let requires_capture_group_fixup = if find_not_empty {
            // if the find_not_empty flag is set, we skip optimizations
            // partially because we have to go though the VM anyway
            // partially because having the last instruction of the expression not have
            // ix be at the end of capture group 0 ruins our empty match checking logic.
            false
        } else {
            // try to optimize the expression tree so that a hard pattern could become easy
            // with a fixup of the capture groups
            optimize(&mut tree)
        };
        let info = analyze(
            &tree,
            AnalyzeContext {
                explicit_capture_group_0: requires_capture_group_fixup,
                find_not_empty,
            },
        )?;

        if find_not_empty && info.const_size && info.min_size == 0 {
            return Err(CompileError::PatternCanNeverMatch.into());
        }

        if !info.hard {
            // easy case, wrap regex

            // we do our own to_str because escapes are different
            // NOTE: there is a good opportunity here to use Hir to avoid regex-automata re-parsing it
            let mut re_cooked = String::new();
            tree.expr.to_str(&mut re_cooked, 0);
            let inner = compile::compile_inner(&re_cooked, options)?;
            return Ok(Regex {
                inner: RegexImpl::Wrap {
                    inner,
                    pattern,
                    explicit_capture_group_0: requires_capture_group_fixup,
                    delegated_pattern: re_cooked,
                },
                named_groups: Arc::new(tree.named_groups),
            });
        }

        let prog = compile(
            &info,
            CompileOptions {
                anchored: can_compile_as_anchored(&tree.expr),
                contains_subroutines: tree.contains_subroutines,
            },
        )?;
        Ok(Regex {
            inner: RegexImpl::Fancy {
                prog: Arc::new(prog),
                n_groups: info.end_group(),
                options: options.hard_regex_runtime_options,
                pattern,
            },
            named_groups: Arc::new(tree.named_groups),
        })
    }

    /// Returns the original string of this regex.
    pub fn as_str(&self) -> &str {
        match &self.inner {
            RegexImpl::Wrap { pattern, .. } => pattern,
            RegexImpl::Fancy { pattern, .. } => pattern,
        }
    }

    /// Check if the regex matches the input text.
    ///
    /// # Example
    ///
    /// Test if some text contains the same word twice:
    ///
    /// ```rust
    /// # use fancy_regex::Regex;
    ///
    /// let re = Regex::new(r"(\w+) \1").unwrap();
    /// assert!(re.is_match("mirror mirror on the wall").unwrap());
    /// ```
    pub fn is_match(&self, text: &str) -> Result<bool> {
        match &self.inner {
            RegexImpl::Wrap { inner, .. } => Ok(inner.is_match(text)),
            RegexImpl::Fancy { .. } => self.find(text).map(|m| m.is_some()),
        }
    }

    /// Returns an iterator for each successive non-overlapping match in `text`.
    ///
    /// If you have capturing groups in your regex that you want to extract, use the [Regex::captures_iter()]
    /// method.
    ///
    /// # Example
    ///
    /// Find all words followed by an exclamation point:
    ///
    /// ```rust
    /// # use fancy_regex::Regex;
    ///
    /// let re = Regex::new(r"\w+(?=!)").unwrap();
    /// let mut matches = re.find_iter("so fancy! even with! iterators!");
    /// assert_eq!(matches.next().unwrap().unwrap().as_str(), "fancy");
    /// assert_eq!(matches.next().unwrap().unwrap().as_str(), "with");
    /// assert_eq!(matches.next().unwrap().unwrap().as_str(), "iterators");
    /// assert!(matches.next().is_none());
    /// ```
    pub fn find_iter<'r, 't>(&'r self, text: &'t str) -> Matches<'r, 't> {
        Matches {
            re: self,
            text,
            last_end: 0,
            last_match: None,
            last_skipped_empty: false,
        }
    }

    /// Find the first match in the input text.
    ///
    /// If you have capturing groups in your regex that you want to extract, use the [Regex::captures()]
    /// method.
    ///
    /// # Example
    ///
    /// Find a word that is followed by an exclamation point:
    ///
    /// ```rust
    /// # use fancy_regex::Regex;
    ///
    /// let re = Regex::new(r"\w+(?=!)").unwrap();
    /// assert_eq!(re.find("so fancy!").unwrap().unwrap().as_str(), "fancy");
    /// ```
    pub fn find<'t>(&self, text: &'t str) -> Result<Option<Match<'t>>> {
        self.find_from_pos(text, 0)
    }

    /// Returns the first match in `text`, starting from the specified byte position `pos`.
    ///
    /// # Examples
    ///
    /// Finding match starting at a position:
    ///
    /// ```
    /// # use fancy_regex::Regex;
    /// let re = Regex::new(r"(?m:^)(\d+)").unwrap();
    /// let text = "1 test 123\n2 foo";
    /// let mat = re.find_from_pos(text, 7).unwrap().unwrap();
    ///
    /// assert_eq!(mat.start(), 11);
    /// assert_eq!(mat.end(), 12);
    /// ```
    ///
    /// Note that in some cases this is not the same as using the `find`
    /// method and passing a slice of the string, see [Regex::captures_from_pos()] for details.
    pub fn find_from_pos<'t>(&self, text: &'t str, pos: usize) -> Result<Option<Match<'t>>> {
        self.find_from_pos_with_option_flags(text, pos, 0)
    }

    fn find_from_pos_with_option_flags<'t>(
        &self,
        text: &'t str,
        pos: usize,
        option_flags: u32,
    ) -> Result<Option<Match<'t>>> {
        if pos > text.len() {
            return Ok(None);
        }
        match &self.inner {
            RegexImpl::Wrap {
                inner,
                explicit_capture_group_0,
                ..
            } => {
                let result = if !*explicit_capture_group_0 {
                    inner
                        .search(&RaInput::new(text).span(pos..text.len()))
                        .map(|m| Match::new(text, m.start(), m.end()))
                } else {
                    let mut locations = inner.create_captures();
                    inner.captures(RaInput::new(text).span(pos..text.len()), &mut locations);
                    locations
                        .get_group(1)
                        .map(|group1| Match::new(text, group1.start, group1.end))
                };
                Ok(result)
            }
            RegexImpl::Fancy { prog, options, .. } => {
                let option_flags = option_flags
                    | if options.find_not_empty {
                        OPTION_FIND_NOT_EMPTY
                    } else {
                        0
                    };
                let result = vm::run(prog, text, pos, option_flags, options)?;
                Ok(result.map(|saves| Match::new(text, saves[0], saves[1])))
            }
        }
    }

    /// Returns an iterator over all the non-overlapping capture groups matched in `text`.
    ///
    /// # Examples
    ///
    /// Finding all matches and capturing parts of each:
    ///
    /// ```rust
    /// # use fancy_regex::Regex;
    ///
    /// let re = Regex::new(r"(\d{4})-(\d{2})").unwrap();
    /// let text = "It was between 2018-04 and 2020-01";
    /// let mut all_captures = re.captures_iter(text);
    ///
    /// let first = all_captures.next().unwrap().unwrap();
    /// assert_eq!(first.get(1).unwrap().as_str(), "2018");
    /// assert_eq!(first.get(2).unwrap().as_str(), "04");
    /// assert_eq!(first.get(0).unwrap().as_str(), "2018-04");
    ///
    /// let second = all_captures.next().unwrap().unwrap();
    /// assert_eq!(second.get(1).unwrap().as_str(), "2020");
    /// assert_eq!(second.get(2).unwrap().as_str(), "01");
    /// assert_eq!(second.get(0).unwrap().as_str(), "2020-01");
    ///
    /// assert!(all_captures.next().is_none());
    /// ```
    pub fn captures_iter<'r, 't>(&'r self, text: &'t str) -> CaptureMatches<'r, 't> {
        CaptureMatches(self.find_iter(text))
    }

    /// Returns the capture groups for the first match in `text`.
    ///
    /// If no match is found, then `Ok(None)` is returned.
    ///
    /// # Examples
    ///
    /// Finding matches and capturing parts of the match:
    ///
    /// ```rust
    /// # use fancy_regex::Regex;
    ///
    /// let re = Regex::new(r"(\d{4})-(\d{2})-(\d{2})").unwrap();
    /// let text = "The date was 2018-04-07";
    /// let captures = re.captures(text).unwrap().unwrap();
    ///
    /// assert_eq!(captures.get(1).unwrap().as_str(), "2018");
    /// assert_eq!(captures.get(2).unwrap().as_str(), "04");
    /// assert_eq!(captures.get(3).unwrap().as_str(), "07");
    /// assert_eq!(captures.get(0).unwrap().as_str(), "2018-04-07");
    /// ```
    pub fn captures<'t>(&self, text: &'t str) -> Result<Option<Captures<'t>>> {
        self.captures_from_pos(text, 0)
    }

    /// Returns the capture groups for the first match in `text`, starting from
    /// the specified byte position `pos`.
    ///
    /// # Examples
    ///
    /// Finding captures starting at a position:
    ///
    /// ```
    /// # use fancy_regex::Regex;
    /// let re = Regex::new(r"(?m:^)(\d+)").unwrap();
    /// let text = "1 test 123\n2 foo";
    /// let captures = re.captures_from_pos(text, 7).unwrap().unwrap();
    ///
    /// let group = captures.get(1).unwrap();
    /// assert_eq!(group.as_str(), "2");
    /// assert_eq!(group.start(), 11);
    /// assert_eq!(group.end(), 12);
    /// ```
    ///
    /// Note that in some cases this is not the same as using the `captures`
    /// method and passing a slice of the string, see the capture that we get
    /// when we do this:
    ///
    /// ```
    /// # use fancy_regex::Regex;
    /// let re = Regex::new(r"(?m:^)(\d+)").unwrap();
    /// let text = "1 test 123\n2 foo";
    /// let captures = re.captures(&text[7..]).unwrap().unwrap();
    /// assert_eq!(captures.get(1).unwrap().as_str(), "123");
    /// ```
    ///
    /// This matched the number "123" because it's at the beginning of the text
    /// of the string slice.
    ///
    pub fn captures_from_pos<'t>(&self, text: &'t str, pos: usize) -> Result<Option<Captures<'t>>> {
        self.captures_from_pos_with_option_flags(text, pos, 0)
    }

    fn captures_from_pos_with_option_flags<'t>(
        &self,
        text: &'t str,
        pos: usize,
        option_flags: u32,
    ) -> Result<Option<Captures<'t>>> {
        if pos > text.len() {
            return Ok(None);
        }
        let named_groups = self.named_groups.clone();
        match &self.inner {
            RegexImpl::Wrap {
                inner,
                explicit_capture_group_0,
                ..
            } => {
                // find_not_empty patterns are always compiled as Fancy, so find_not_empty is
                // always false here.
                let explicit = *explicit_capture_group_0;
                let mut locations = inner.create_captures();
                inner.captures(RaInput::new(text).span(pos..text.len()), &mut locations);
                Ok(locations.is_match().then_some(Captures {
                    inner: CapturesImpl::Wrap {
                        text,
                        locations,
                        explicit_capture_group_0: explicit,
                    },
                    named_groups,
                }))
            }
            RegexImpl::Fancy {
                prog,
                n_groups,
                options,
                ..
            } => {
                let option_flags = option_flags
                    | if options.find_not_empty {
                        OPTION_FIND_NOT_EMPTY
                    } else {
                        0
                    };
                let result = vm::run(prog, text, pos, option_flags, options)?;
                Ok(result.map(|mut saves| {
                    saves.truncate(n_groups * 2);
                    Captures {
                        inner: CapturesImpl::Fancy { text, saves },
                        named_groups,
                    }
                }))
            }
        }
    }

    /// Returns the number of captures, including the implicit capture of the entire expression.
    pub fn captures_len(&self) -> usize {
        match &self.inner {
            RegexImpl::Wrap {
                inner,
                explicit_capture_group_0,
                ..
            } => inner.captures_len() - if *explicit_capture_group_0 { 1 } else { 0 },
            RegexImpl::Fancy { n_groups, .. } => *n_groups,
        }
    }

    /// Returns an iterator over the capture names.
    pub fn capture_names(&self) -> CaptureNames<'_> {
        let mut names = Vec::new();
        names.resize(self.captures_len(), None);
        for (name, &i) in self.named_groups.iter() {
            names[i] = Some(name.as_str());
        }
        CaptureNames(names.into_iter())
    }

    // for debugging only
    #[doc(hidden)]
    pub fn debug_print(&self, writer: &mut Formatter<'_>) -> fmt::Result {
        match &self.inner {
            RegexImpl::Wrap {
                delegated_pattern,
                explicit_capture_group_0,
                ..
            } => {
                write!(
                    writer,
                    "wrapped Regex {:?}, explicit_capture_group_0: {:}",
                    delegated_pattern, *explicit_capture_group_0
                )
            }
            RegexImpl::Fancy { prog, .. } => prog.debug_print(writer),
        }
    }

    /// Replaces the leftmost-first match with the replacement provided.
    /// The replacement can be a regular string (where `$N` and `$name` are
    /// expanded to match capture groups) or a function that takes the matches'
    /// `Captures` and returns the replaced string.
    ///
    /// If no match is found, then a copy of the string is returned unchanged.
    ///
    /// # Replacement string syntax
    ///
    /// All instances of `$name` in the replacement text is replaced with the
    /// corresponding capture group `name`.
    ///
    /// `name` may be an integer corresponding to the index of the
    /// capture group (counted by order of opening parenthesis where `0` is the
    /// entire match) or it can be a name (consisting of letters, digits or
    /// underscores) corresponding to a named capture group.
    ///
    /// If `name` isn't a valid capture group (whether the name doesn't exist
    /// or isn't a valid index), then it is replaced with the empty string.
    ///
    /// The longest possible name is used. e.g., `$1a` looks up the capture
    /// group named `1a` and not the capture group at index `1`. To exert more
    /// precise control over the name, use braces, e.g., `${1}a`.
    ///
    /// To write a literal `$` use `$$`.
    ///
    /// # Examples
    ///
    /// Note that this function is polymorphic with respect to the replacement.
    /// In typical usage, this can just be a normal string:
    ///
    /// ```rust
    /// # use fancy_regex::Regex;
    /// let re = Regex::new("[^01]+").unwrap();
    /// assert_eq!(re.replace("1078910", ""), "1010");
    /// ```
    ///
    /// But anything satisfying the `Replacer` trait will work. For example,
    /// a closure of type `|&Captures| -> String` provides direct access to the
    /// captures corresponding to a match. This allows one to access
    /// capturing group matches easily:
    ///
    /// ```rust
    /// # use fancy_regex::{Regex, Captures};
    /// let re = Regex::new(r"([^,\s]+),\s+(\S+)").unwrap();
    /// let result = re.replace("Springsteen, Bruce", |caps: &Captures| {
    ///     format!("{} {}", &caps[2], &caps[1])
    /// });
    /// assert_eq!(result, "Bruce Springsteen");
    /// ```
    ///
    /// But this is a bit cumbersome to use all the time. Instead, a simple
    /// syntax is supported that expands `$name` into the corresponding capture
    /// group. Here's the last example, but using this expansion technique
    /// with named capture groups:
    ///
    /// ```rust
    /// # use fancy_regex::Regex;
    /// let re = Regex::new(r"(?P<last>[^,\s]+),\s+(?P<first>\S+)").unwrap();
    /// let result = re.replace("Springsteen, Bruce", "$first $last");
    /// assert_eq!(result, "Bruce Springsteen");
    /// ```
    ///
    /// Note that using `$2` instead of `$first` or `$1` instead of `$last`
    /// would produce the same result. To write a literal `$` use `$$`.
    ///
    /// Sometimes the replacement string requires use of curly braces to
    /// delineate a capture group replacement and surrounding literal text.
    /// For example, if we wanted to join two words together with an
    /// underscore:
    ///
    /// ```rust
    /// # use fancy_regex::Regex;
    /// let re = Regex::new(r"(?P<first>\w+)\s+(?P<second>\w+)").unwrap();
    /// let result = re.replace("deep fried", "${first}_$second");
    /// assert_eq!(result, "deep_fried");
    /// ```
    ///
    /// Without the curly braces, the capture group name `first_` would be
    /// used, and since it doesn't exist, it would be replaced with the empty
    /// string.
    ///
    /// Finally, sometimes you just want to replace a literal string with no
    /// regard for capturing group expansion. This can be done by wrapping a
    /// byte string with `NoExpand`:
    ///
    /// ```rust
    /// # use fancy_regex::Regex;
    /// use fancy_regex::NoExpand;
    ///
    /// let re = Regex::new(r"(?P<last>[^,\s]+),\s+(\S+)").unwrap();
    /// let result = re.replace("Springsteen, Bruce", NoExpand("$2 $last"));
    /// assert_eq!(result, "$2 $last");
    /// ```
    pub fn replace<'t, R: Replacer>(&self, text: &'t str, rep: R) -> Cow<'t, str> {
        self.replacen(text, 1, rep)
    }

    /// Replaces all non-overlapping matches in `text` with the replacement
    /// provided. This is the same as calling `replacen` with `limit` set to
    /// `0`.
    ///
    /// See the documentation for `replace` for details on how to access
    /// capturing group matches in the replacement string.
    pub fn replace_all<'t, R: Replacer>(&self, text: &'t str, rep: R) -> Cow<'t, str> {
        self.replacen(text, 0, rep)
    }

    /// Replaces at most `limit` non-overlapping matches in `text` with the
    /// replacement provided. If `limit` is 0, then all non-overlapping matches
    /// are replaced.
    ///
    /// Will panic if any errors are encountered. Use `try_replacen`, which this
    /// function unwraps, if you want to handle errors.
    ///
    /// See the documentation for `replace` for details on how to access
    /// capturing group matches in the replacement string.
    ///
    pub fn replacen<'t, R: Replacer>(&self, text: &'t str, limit: usize, rep: R) -> Cow<'t, str> {
        self.try_replacen(text, limit, rep).unwrap()
    }

    /// Replaces at most `limit` non-overlapping matches in `text` with the
    /// replacement provided. If `limit` is 0, then all non-overlapping matches
    /// are replaced.
    ///
    /// Propagates any errors encountered, such as `RuntimeError::BacktrackLimitExceeded`.
    ///
    /// See the documentation for `replace` for details on how to access
    /// capturing group matches in the replacement string.
    pub fn try_replacen<'t, R: Replacer>(
        &self,
        text: &'t str,
        limit: usize,
        mut rep: R,
    ) -> Result<Cow<'t, str>> {
        // If we know that the replacement doesn't have any capture expansions,
        // then we can fast path. The fast path can make a tremendous
        // difference:
        //
        //   1) We use `find_iter` instead of `captures_iter`. Not asking for
        //      captures generally makes the regex engines faster.
        //   2) We don't need to look up all of the capture groups and do
        //      replacements inside the replacement string. We just push it
        //      at each match and be done with it.
        if let Some(rep) = rep.no_expansion() {
            let mut it = self.find_iter(text).enumerate().peekable();
            if it.peek().is_none() {
                return Ok(Cow::Borrowed(text));
            }
            let mut new = String::with_capacity(text.len());
            let mut last_match = 0;
            for (i, m) in it {
                let m = m?;

                if limit > 0 && i >= limit {
                    break;
                }
                new.push_str(&text[last_match..m.start()]);
                new.push_str(&rep);
                last_match = m.end();
            }
            new.push_str(&text[last_match..]);
            return Ok(Cow::Owned(new));
        }

        // The slower path, which we use if the replacement needs access to
        // capture groups.
        let mut it = self.captures_iter(text).enumerate().peekable();
        if it.peek().is_none() {
            return Ok(Cow::Borrowed(text));
        }
        let mut new = String::with_capacity(text.len());
        let mut last_match = 0;
        for (i, cap) in it {
            let cap = cap?;

            if limit > 0 && i >= limit {
                break;
            }
            // unwrap on 0 is OK because captures only reports matches
            let m = cap.get(0).unwrap();
            new.push_str(&text[last_match..m.start()]);
            rep.replace_append(&cap, &mut new);
            last_match = m.end();
        }
        new.push_str(&text[last_match..]);
        Ok(Cow::Owned(new))
    }

    /// Splits the string by matches of the regex.
    ///
    /// Returns an iterator over the substrings of the target string
    ///  that *aren't* matched by the regex.
    ///
    /// # Example
    ///
    /// To split a string delimited by arbitrary amounts of spaces or tabs:
    ///
    /// ```rust
    /// # use fancy_regex::Regex;
    /// let re = Regex::new(r"[ \t]+").unwrap();
    /// let target = "a b \t  c\td    e";
    /// let fields: Vec<&str> = re.split(target).map(|x| x.unwrap()).collect();
    /// assert_eq!(fields, vec!["a", "b", "c", "d", "e"]);
    /// ```
    pub fn split<'r, 'h>(&'r self, target: &'h str) -> Split<'r, 'h> {
        Split {
            matches: self.find_iter(target),
            next_start: 0,
            target,
        }
    }

    /// Splits the string by matches of the regex at most `limit` times.
    ///
    /// Returns an iterator over the substrings of the target string
    /// that *aren't* matched by the regex.
    ///
    /// The `N`th substring is the remaining part of the target.
    ///
    /// # Example
    ///
    /// To split a string delimited by arbitrary amounts of spaces or tabs
    /// 3 times:
    ///
    /// ```rust
    /// # use fancy_regex::Regex;
    /// let re = Regex::new(r"[ \t]+").unwrap();
    /// let target = "a b \t  c\td    e";
    /// let fields: Vec<&str> = re.splitn(target, 3).map(|x| x.unwrap()).collect();
    /// assert_eq!(fields, vec!["a", "b", "c\td    e"]);
    /// ```
    pub fn splitn<'r, 'h>(&'r self, target: &'h str, limit: usize) -> SplitN<'r, 'h> {
        SplitN {
            splits: self.split(target),
            limit,
        }
    }
}

impl TryFrom<&str> for Regex {
    type Error = Error;

    /// Attempts to parse a string into a regular expression
    fn try_from(s: &str) -> Result<Self> {
        Self::new(s)
    }
}

impl TryFrom<String> for Regex {
    type Error = Error;

    /// Attempts to parse a string into a regular expression
    fn try_from(s: String) -> Result<Self> {
        Self::new(&s)
    }
}

impl<'t> Match<'t> {
    /// Returns the starting byte offset of the match in the text.
    #[inline]
    pub fn start(&self) -> usize {
        self.start
    }

    /// Returns the ending byte offset of the match in the text.
    #[inline]
    pub fn end(&self) -> usize {
        self.end
    }

    /// Returns the range over the starting and ending byte offsets of the match in text.
    #[inline]
    pub fn range(&self) -> Range<usize> {
        self.start..self.end
    }

    /// Returns the matched text.
    #[inline]
    pub fn as_str(&self) -> &'t str {
        &self.text[self.start..self.end]
    }

    /// Creates a new match from the given text and byte offsets.
    fn new(text: &'t str, start: usize, end: usize) -> Match<'t> {
        Match { text, start, end }
    }
}

impl<'t> From<Match<'t>> for &'t str {
    fn from(m: Match<'t>) -> &'t str {
        m.as_str()
    }
}

impl<'t> From<Match<'t>> for Range<usize> {
    fn from(m: Match<'t>) -> Range<usize> {
        m.range()
    }
}

#[allow(clippy::len_without_is_empty)] // follow regex's API
impl<'t> Captures<'t> {
    /// Get the capture group by its index in the regex.
    ///
    /// If there is no match for that group or the index does not correspond to a group, `None` is
    /// returned. The index 0 returns the whole match.
    pub fn get(&self, i: usize) -> Option<Match<'t>> {
        match &self.inner {
            CapturesImpl::Wrap {
                text,
                locations,
                explicit_capture_group_0,
            } => locations
                .get_group(i + if *explicit_capture_group_0 { 1 } else { 0 })
                .map(|span| Match {
                    text,
                    start: span.start,
                    end: span.end,
                }),
            CapturesImpl::Fancy { text, saves } => {
                let slot = i * 2;
                if slot >= saves.len() {
                    return None;
                }
                let lo = saves[slot];
                if lo == usize::MAX {
                    return None;
                }
                let hi = saves[slot + 1];
                Some(Match {
                    text,
                    start: lo,
                    end: hi,
                })
            }
        }
    }

    /// Returns the match for a named capture group.  Returns `None` the capture
    /// group did not match or if there is no group with the given name.
    pub fn name(&self, name: &str) -> Option<Match<'t>> {
        self.named_groups.get(name).and_then(|i| self.get(*i))
    }

    /// Expands all instances of `$group` in `replacement` to the corresponding
    /// capture group `name`, and writes them to the `dst` buffer given.
    ///
    /// `group` may be an integer corresponding to the index of the
    /// capture group (counted by order of opening parenthesis where `\0` is the
    /// entire match) or it can be a name (consisting of letters, digits or
    /// underscores) corresponding to a named capture group.
    ///
    /// If `group` isn't a valid capture group (whether the name doesn't exist
    /// or isn't a valid index), then it is replaced with the empty string.
    ///
    /// The longest possible name is used. e.g., `$1a` looks up the capture
    /// group named `1a` and not the capture group at index `1`. To exert more
    /// precise control over the name, use braces, e.g., `${1}a`.
    ///
    /// To write a literal `$`, use `$$`.
    ///
    /// For more control over expansion, see [`Expander`].
    ///
    /// [`Expander`]: expand/struct.Expander.html
    pub fn expand(&self, replacement: &str, dst: &mut String) {
        Expander::default().append_expansion(dst, replacement, self);
    }

    /// Iterate over the captured groups in order in which they appeared in the regex. The first
    /// capture corresponds to the whole match.
    pub fn iter<'c>(&'c self) -> SubCaptureMatches<'c, 't> {
        SubCaptureMatches { caps: self, i: 0 }
    }

    /// How many groups were captured. This is always at least 1 because group 0 returns the whole
    /// match.
    pub fn len(&self) -> usize {
        match &self.inner {
            CapturesImpl::Wrap {
                locations,
                explicit_capture_group_0,
                ..
            } => locations.group_len() - if *explicit_capture_group_0 { 1 } else { 0 },
            CapturesImpl::Fancy { saves, .. } => saves.len() / 2,
        }
    }
}

/// Get a group by index.
///
/// `'t` is the lifetime of the matched text.
///
/// The text can't outlive the `Captures` object if this method is
/// used, because of how `Index` is defined (normally `a[i]` is part
/// of `a` and can't outlive it); to do that, use `get()` instead.
///
/// # Panics
///
/// If there is no group at the given index.
impl<'t> Index<usize> for Captures<'t> {
    type Output = str;

    fn index(&self, i: usize) -> &str {
        self.get(i)
            .map(|m| m.as_str())
            .unwrap_or_else(|| panic!("no group at index '{}'", i))
    }
}

/// Get a group by name.
///
/// `'t` is the lifetime of the matched text and `'i` is the lifetime
/// of the group name (the index).
///
/// The text can't outlive the `Captures` object if this method is
/// used, because of how `Index` is defined (normally `a[i]` is part
/// of `a` and can't outlive it); to do that, use `name` instead.
///
/// # Panics
///
/// If there is no group named by the given value.
impl<'t, 'i> Index<&'i str> for Captures<'t> {
    type Output = str;

    fn index<'a>(&'a self, name: &'i str) -> &'a str {
        self.name(name)
            .map(|m| m.as_str())
            .unwrap_or_else(|| panic!("no group named '{}'", name))
    }
}

impl<'c, 't> Iterator for SubCaptureMatches<'c, 't> {
    type Item = Option<Match<'t>>;

    fn next(&mut self) -> Option<Option<Match<'t>>> {
        if self.i < self.caps.len() {
            let result = self.caps.get(self.i);
            self.i += 1;
            Some(result)
        } else {
            None
        }
    }
}

// TODO: might be nice to implement ExactSizeIterator etc for SubCaptures

/// Regular expression AST. This is public for now but may change.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Expr {
    /// An empty expression, e.g. the last branch in `(a|b|)`
    Empty,
    /// Any character, regex `.`
    Any {
        /// Whether it also matches newlines or not
        newline: bool,
        /// Whether CRLF mode is enabled (`\r` also counts as a newline, so dot
        /// excludes both `\r` and `\n`)
        crlf: bool,
    },
    /// An assertion
    Assertion(Assertion),
    /// General newline sequence, `\R`
    /// Matches `\r\n` or any single newline character (\n, \v, \f, \r)
    /// In Unicode mode, also matches U+0085, U+2028, U+2029
    GeneralNewline {
        /// Whether Unicode mode is enabled
        unicode: bool,
    },
    /// The string as a literal, e.g. `a`
    Literal {
        /// The string to match
        val: String,
        /// Whether match is case-insensitive or not
        casei: bool,
    },
    /// Concatenation of multiple expressions, must match in order, e.g. `a.` is a concatenation of
    /// the literal `a` and `.` for any character
    Concat(Vec<Expr>),
    /// Alternative of multiple expressions, one of them must match, e.g. `a|b` is an alternative
    /// where either the literal `a` or `b` must match
    Alt(Vec<Expr>),
    /// Capturing group of expression, e.g. `(a.)` matches `a` and any character and "captures"
    /// (remembers) the match
    Group(Arc<Expr>),
    /// Look-around (e.g. positive/negative look-ahead or look-behind) with an expression, e.g.
    /// `(?=a)` means the next character must be `a` (but the match is not consumed)
    LookAround(Box<Expr>, LookAround),
    /// Repeat of an expression, e.g. `a*` or `a+` or `a{1,3}`
    Repeat {
        /// The expression that is being repeated
        child: Box<Expr>,
        /// The minimum number of repetitions
        lo: usize,
        /// The maximum number of repetitions (or `usize::MAX`)
        hi: usize,
        /// Greedy means as much as possible is matched, e.g. `.*b` would match all of `abab`.
        /// Non-greedy means as little as possible, e.g. `.*?b` would match only `ab` in `abab`.
        greedy: bool,
    },
    /// Delegate a regex to the regex crate. This is used as a simplification so that we don't have
    /// to represent all the expressions in the AST, e.g. character classes.
    ///
    /// **Constraint**: All Delegate expressions must match exactly 1 character. This ensures
    /// consistent analysis and compilation behavior. For zero-width or multi-character patterns,
    /// use the appropriate Expr variants instead (e.g., Assertion, Repeat, Concat).
    Delegate {
        /// The regex
        inner: String,
        /// Whether the matching is case-insensitive or not
        casei: bool,
    },
    /// Back reference to a capture group, e.g. `\1` in `(abc|def)\1` references the captured group
    /// and the whole regex matches either `abcabc` or `defdef`.
    Backref {
        /// The capture group number being referenced
        group: usize,
        /// Whether the matching is case-insensitive or not
        casei: bool,
    },
    /// Back reference to a capture group at the given specified relative recursion level.
    BackrefWithRelativeRecursionLevel {
        /// The capture group number being referenced
        group: usize,
        /// Relative recursion level
        relative_level: isize,
        /// Whether the matching is case-insensitive or not
        casei: bool,
    },
    /// Atomic non-capturing group, e.g. `(?>ab|a)` in text that contains `ab` will match `ab` and
    /// never backtrack and try `a`, even if matching fails after the atomic group.
    AtomicGroup(Box<Expr>),
    /// Keep matched text so far out of overall match
    KeepOut,
    /// Anchor to match at the position where the previous match ended
    ContinueFromPreviousMatchEnd,
    /// Conditional expression based on whether the numbered capture group matched or not.
    /// The optional `relative_recursion_level` qualifies which recursion level's capture is
    /// tested (Oniguruma `(?(name+N)...)` syntax).
    BackrefExistsCondition {
        /// The resolved capture group number
        group: usize,
        /// Optional relative recursion level (e.g. `+0`, `-1`)
        relative_recursion_level: Option<isize>,
    },
    /// If/Then/Else Condition. If there is no Then/Else, these will just be empty expressions.
    Conditional {
        /// The conditional expression to evaluate
        condition: Box<Expr>,
        /// What to execute if the condition is true
        true_branch: Box<Expr>,
        /// What to execute if the condition is false
        false_branch: Box<Expr>,
    },
    /// Subroutine call to the specified group number
    SubroutineCall(usize),
    /// Backtracking control verb
    BacktrackingControlVerb(BacktrackingControlVerb),
    /// Match while the given expression is absent from the haystack
    Absent(Absent),
    /// DEFINE group - defines capture groups for subroutines without matching anything
    /// The expressions inside are parsed and assigned group numbers, but no VM instructions
    /// are generated for the DEFINE block itself.
    DefineGroup {
        /// The expressions/groups being defined
        definitions: Box<Expr>,
    },
    /// Abstract Syntax Tree node - will be resolved into an Expr before analysis.
    /// Contains the position in the pattern where the node was parsed from
    AstNode(AstNode, usize),
}

/// Target of a backreference or subroutine call
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum CaptureGroupTarget {
    /// Direct numbered reference
    ByNumber(usize),

    /// Named reference
    ByName(String),

    /// Relative reference (e.g., -1, -2, etc.)
    Relative(isize),
}

/// Abstract Syntax Tree node - will be resolved into an Expr before analysis
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum AstNode {
    /// Group with optional name - name is only present if explicitly specified in pattern
    AstGroup {
        /// Optional name of the capture group, present only when explicitly named in the pattern
        name: Option<String>,
        /// The inner expression of the group
        inner: Box<Expr>,
    },
    /// Backreference
    Backref {
        /// The target capture group being referenced
        target: CaptureGroupTarget,
        /// Whether the matching is case-insensitive or not
        // TODO: move out of Backref and prefer a Flags AstNode. The resolver can then track the flags and set casei on the resolved Expr accordingly
        casei: bool,
        /// Optional relative recursion level for the backreference
        relative_recursion_level: Option<isize>,
    },
    /// Subroutine Call
    SubroutineCall(CaptureGroupTarget),
    /// Backreference exists condition `(?(name)...)` or `(?(1)...)` - unresolved target.
    /// The optional `relative_recursion_level` corresponds to the Oniguruma `+N`/`-N` suffix
    /// (e.g. `(?(name+0)...)`) which qualifies which recursion level's capture is tested.
    BackrefExistsCondition {
        /// The target capture group being tested for existence
        target: CaptureGroupTarget,
        /// Optional relative recursion level qualifier (e.g. `+0`, `-1`)
        relative_recursion_level: Option<isize>,
    },
}

/// Type of look-around assertion as used for a look-around expression.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum LookAround {
    /// Look-ahead assertion, e.g. `(?=a)`
    LookAhead,
    /// Negative look-ahead assertion, e.g. `(?!a)`
    LookAheadNeg,
    /// Look-behind assertion, e.g. `(?<=a)`
    LookBehind,
    /// Negative look-behind assertion, e.g. `(?<!a)`
    LookBehindNeg,
}

/// Type of absent operator as used for Oniguruma's absent functionality.
#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Absent {
    /// Absent repeater `(?~absent)` - works like `\O*` (match any character including newline, repeated)
    /// but is limited by the range that does not include the string match with `absent`.
    /// This is a written abbreviation of `(?~|absent|\O*)`.
    Repeater(Box<Expr>),
    /// Absent expression `(?~|absent|exp)` - works like `exp`, but is limited by the range
    /// that does not include the string match with `absent`.
    Expression {
        /// The expression to avoid matching
        absent: Box<Expr>,
        /// The expression to match
        exp: Box<Expr>,
    },
    /// Absent stopper `(?~|absent)` - after this operator, haystack range is limited
    /// up to the point where `absent` matches.
    Stopper(Box<Expr>),
    /// Range clear `(?~|)` - clears the effects caused by absent stoppers.
    Clear,
}

/// Type of backtracking control verb which affects how backtracking will behave.
/// See <https://www.regular-expressions.info/verb.html>
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum BacktrackingControlVerb {
    /// Fail this branch immediately
    Fail,
    /// Treat match so far as successful overall match
    Accept,
    /// Abort the entire match on failure
    Commit,
    /// Restart the entire match attempt at the current position
    Skip,
    /// Prune all backtracking states and restart the entire match attempt at the next position
    Prune,
}

/// An iterator over capture names in a [Regex].  The iterator
/// returns the name of each group, or [None] if the group has
/// no name.  Because capture group 0 cannot have a name, the
/// first item returned is always [None].
pub struct CaptureNames<'r>(vec::IntoIter<Option<&'r str>>);

impl Debug for CaptureNames<'_> {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.write_str("<CaptureNames>")
    }
}

impl<'r> Iterator for CaptureNames<'r> {
    type Item = Option<&'r str>;

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next()
    }
}

// silly to write my own, but this is super-fast for the common 1-digit
// case.
fn push_usize(s: &mut String, x: usize) {
    if x >= 10 {
        push_usize(s, x / 10);
        s.push((b'0' + (x % 10) as u8) as char);
    } else {
        s.push((b'0' + (x as u8)) as char);
    }
}

fn is_special(c: char) -> bool {
    matches!(
        c,
        '\\' | '.' | '+' | '*' | '?' | '(' | ')' | '|' | '[' | ']' | '{' | '}' | '^' | '$' | '#'
    )
}

fn push_quoted(buf: &mut String, s: &str) {
    for c in s.chars() {
        if is_special(c) {
            buf.push('\\');
        }
        buf.push(c);
    }
}

/// Escapes special characters in `text` with '\\'.  Returns a string which, when interpreted
/// as a regex, matches exactly `text`.
pub fn escape(text: &str) -> Cow<'_, str> {
    // Using bytes() is OK because all special characters are single bytes.
    match text.bytes().filter(|&b| is_special(b as char)).count() {
        0 => Cow::Borrowed(text),
        n => {
            // The capacity calculation is exact because '\\' is a single byte.
            let mut buf = String::with_capacity(text.len() + n);
            push_quoted(&mut buf, text);
            Cow::Owned(buf)
        }
    }
}

/// Type of assertions
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Assertion {
    /// Start of input text
    StartText,
    /// End of input text
    EndText,
    /// End of input text, or before any trailing newlines at the end (Oniguruma's `\Z`)
    EndTextIgnoreTrailingNewlines {
        /// Whether CRLF mode is enabled.
        /// If `true`, trailing `\r\n` pairs (in addition to bare `\n`) are also ignored.
        crlf: bool,
    },
    /// Start of a line
    StartLine {
        /// CRLF mode.
        /// If true, this assertion matches at the starting position of the input text, or at the position immediately
        /// following either a `\r` or `\n` character, but never after a `\r` when a `\n` follows.
        crlf: bool,
    },
    /// End of a line
    EndLine {
        /// CRLF mode
        /// If true, this assertion matches at the ending position of the input text, or at the position immediately
        /// preceding either a `\r` or `\n` character, but never after a `\r` when a `\n` follows.
        crlf: bool,
    },
    /// Left word boundary
    LeftWordBoundary,
    /// Left word half boundary
    LeftWordHalfBoundary,
    /// Right word boundary
    RightWordBoundary,
    /// Right word half boundary
    RightWordHalfBoundary,
    /// Both word boundaries
    WordBoundary,
    /// Not word boundary
    NotWordBoundary,
}

impl Assertion {
    pub(crate) fn is_hard(&self) -> bool {
        use Assertion::*;
        matches!(
            self,
            // these will make regex-automata use PikeVM
            LeftWordBoundary
                | LeftWordHalfBoundary
                | RightWordBoundary
                | RightWordHalfBoundary
                | WordBoundary
                | NotWordBoundary
                | EndTextIgnoreTrailingNewlines { .. }
        )
    }
}

/// An iterator over the immediate children of an [`Expr`].
///
/// This iterator yields references to child expressions but does not recurse into them.
#[derive(Debug)]
pub enum ExprChildrenIter<'a> {
    /// No children (leaf node)
    Empty,
    /// A single child (Group, LookAround, AtomicGroup, Repeat)
    Single(Option<&'a Expr>),
    /// Multiple children in a Vec (Concat, Alt)
    Vec(alloc::slice::Iter<'a, Expr>),
    /// Three children (Conditional)
    Triple {
        /// First child
        first: Option<&'a Expr>,
        /// Second child
        second: Option<&'a Expr>,
        /// Third child
        third: Option<&'a Expr>,
    },
}

/// An iterator over the immediate children of an [`Expr`] for mutable access.
///
/// This iterator yields mutable references to child expressions but does not recurse into them.
#[derive(Debug)]
pub enum ExprChildrenIterMut<'a> {
    /// No children (leaf node)
    Empty,
    /// A single child (Group, LookAround, AtomicGroup, Repeat)
    Single(Option<&'a mut Expr>),
    /// Multiple children in a Vec (Concat, Alt)
    Vec(alloc::slice::IterMut<'a, Expr>),
    /// Three children (Conditional)
    Triple {
        /// First child
        first: Option<&'a mut Expr>,
        /// Second child
        second: Option<&'a mut Expr>,
        /// Third child
        third: Option<&'a mut Expr>,
    },
}

impl<'a> Iterator for ExprChildrenIter<'a> {
    type Item = &'a Expr;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ExprChildrenIter::Empty => None,
            ExprChildrenIter::Single(ref mut child) => child.take(),
            ExprChildrenIter::Vec(ref mut iter) => iter.next(),
            ExprChildrenIter::Triple {
                ref mut first,
                ref mut second,
                ref mut third,
            } => first
                .take()
                .or_else(|| second.take())
                .or_else(|| third.take()),
        }
    }
}

impl<'a> Iterator for ExprChildrenIterMut<'a> {
    type Item = &'a mut Expr;

    fn next(&mut self) -> Option<Self::Item> {
        match self {
            ExprChildrenIterMut::Empty => None,
            ExprChildrenIterMut::Single(ref mut child) => child.take(),
            ExprChildrenIterMut::Vec(ref mut iter) => iter.next(),
            ExprChildrenIterMut::Triple {
                ref mut first,
                ref mut second,
                ref mut third,
            } => first
                .take()
                .or_else(|| second.take())
                .or_else(|| third.take()),
        }
    }
}

macro_rules! children_iter_match {
    ($self:expr, $iter:ident, $vec_method:ident, $single_method:ident, $group_method:ident) => {
        match $self {
            Expr::Concat(children) | Expr::Alt(children) => $iter::Vec(children.$vec_method()),
            Expr::Group(child) => $iter::Single(Some(Arc::$group_method(child))),
            Expr::Absent(Absent::Repeater(child))
            | Expr::Absent(Absent::Stopper(child))
            | Expr::LookAround(child, _)
            | Expr::AtomicGroup(child)
            | Expr::Repeat { child, .. } => $iter::Single(Some(child.$single_method())),
            Expr::Conditional {
                condition,
                true_branch,
                false_branch,
            } => $iter::Triple {
                first: Some(condition.$single_method()),
                second: Some(true_branch.$single_method()),
                third: Some(false_branch.$single_method()),
            },
            Expr::Absent(Absent::Expression { absent, exp }) => $iter::Triple {
                first: Some(absent.$single_method()),
                second: Some(exp.$single_method()),
                third: None,
            },
            Expr::DefineGroup { definitions } => $iter::Single(Some(definitions.$single_method())),
            _ if $self.is_leaf_node() => $iter::Empty,
            _ => unimplemented!(),
        }
    };
}
impl Expr {
    /// Parse the regex and return an expression (AST) and a bit set with the indexes of groups
    /// that are referenced by backrefs.
    pub fn parse_tree(re: &str) -> Result<ExprTree> {
        Parser::parse(re)
    }

    /// Parse the regex and return an expression (AST)
    /// Flags should be bit based based on flags
    pub fn parse_tree_with_flags(re: &str, flags: u32) -> Result<ExprTree> {
        Parser::parse_with_flags(re, flags)
    }

    /// Returns `true` if this expression is a leaf node (has no children).
    ///
    /// Leaf nodes include literals, assertions, backreferences, and other atomic expressions.
    /// Non-leaf nodes include groups, concatenations, alternations, and repetitions.
    pub fn is_leaf_node(&self) -> bool {
        matches!(
            self,
            Expr::Empty
                | Expr::Any { .. }
                | Expr::Assertion(_)
                | Expr::GeneralNewline { .. }
                | Expr::Literal { .. }
                | Expr::Delegate { .. }
                | Expr::Backref { .. }
                | Expr::BackrefWithRelativeRecursionLevel { .. }
                | Expr::KeepOut
                | Expr::ContinueFromPreviousMatchEnd
                | Expr::BackrefExistsCondition { .. }
                | Expr::BacktrackingControlVerb(_)
                |             Expr::SubroutineCall(_)
                | Expr::Absent(Absent::Clear)
                // An unresolved AstNode has no separate child Expr to iterate; the resolver
                // should have replaced it before analysis, so treat it as a leaf so that
                // collection/iteration doesn't panic, and let the analyzer emit the error.
                | Expr::AstNode(..),
        )
    }

    /// Returns `true` if any descendant of this expression (not including itself)
    /// satisfies the given predicate.
    ///
    /// This performs an iterative depth-first search using [`children_iter`](Self::children_iter).
    pub fn has_descendant(&self, predicate: impl Fn(&Expr) -> bool) -> bool {
        let mut stack: Vec<&Expr> = self.children_iter().collect();
        while let Some(expr) = stack.pop() {
            if predicate(expr) {
                return true;
            }
            stack.extend(expr.children_iter());
        }
        false
    }

    /// Returns an iterator over the immediate children of this expression.
    ///
    /// For leaf nodes, this returns an empty iterator. For non-leaf nodes, it returns
    /// references to their immediate children (non-recursive).
    pub fn children_iter(&self) -> ExprChildrenIter<'_> {
        children_iter_match!(self, ExprChildrenIter, iter, as_ref, as_ref)
    }

    /// Returns an iterator over the immediate children of this expression for mutable access.
    ///
    /// For leaf nodes, this returns an empty iterator. For non-leaf nodes, it returns
    /// mutable references to their immediate children (non-recursive).
    pub fn children_iter_mut(&mut self) -> ExprChildrenIterMut<'_> {
        children_iter_match!(self, ExprChildrenIterMut, iter_mut, as_mut, make_mut)
    }

    /// Convert expression to a regex string in the regex crate's syntax.
    ///
    /// # Panics
    ///
    /// Panics for expressions that are hard, i.e. can not be handled by the regex crate.
    pub fn to_str(&self, buf: &mut String, precedence: u8) {
        match *self {
            Expr::Empty => (),
            Expr::Any { newline, crlf } => buf.push_str(match (newline, crlf) {
                (true, _) => "(?s:.)",
                (false, true) => "(?R-s:.)",
                (false, false) => ".",
            }),
            Expr::Literal { ref val, casei } => {
                if casei {
                    buf.push_str("(?i:");
                }
                push_quoted(buf, val);
                if casei {
                    buf.push(')');
                }
            }
            Expr::Assertion(Assertion::StartText) => buf.push('^'),
            Expr::Assertion(Assertion::EndText) => buf.push('$'),
            Expr::Assertion(Assertion::StartLine { crlf: false }) => buf.push_str("(?m:^)"),
            Expr::Assertion(Assertion::EndLine { crlf: false }) => buf.push_str("(?m:$)"),
            Expr::Assertion(Assertion::StartLine { crlf: true }) => buf.push_str("(?Rm:^)"),
            Expr::Assertion(Assertion::EndLine { crlf: true }) => buf.push_str("(?Rm:$)"),
            Expr::Concat(ref children) => {
                if precedence > 1 {
                    buf.push_str("(?:");
                }
                for child in children {
                    child.to_str(buf, 2);
                }
                if precedence > 1 {
                    buf.push(')')
                }
            }
            Expr::Alt(_) => {
                if precedence > 0 {
                    buf.push_str("(?:");
                }
                let mut children = self.children_iter();
                if let Some(first) = children.next() {
                    first.to_str(buf, 1);
                    for child in children {
                        buf.push('|');
                        child.to_str(buf, 1);
                    }
                }
                if precedence > 0 {
                    buf.push(')');
                }
            }
            Expr::Group(ref child) => {
                buf.push('(');
                child.to_str(buf, 0);
                buf.push(')');
            }
            Expr::Repeat {
                ref child,
                lo,
                hi,
                greedy,
            } => {
                if precedence > 2 {
                    buf.push_str("(?:");
                }
                child.to_str(buf, 3);
                match (lo, hi) {
                    (0, 1) => buf.push('?'),
                    (0, usize::MAX) => buf.push('*'),
                    (1, usize::MAX) => buf.push('+'),
                    (lo, hi) => {
                        buf.push('{');
                        push_usize(buf, lo);
                        if lo != hi {
                            buf.push(',');
                            if hi != usize::MAX {
                                push_usize(buf, hi);
                            }
                        }
                        buf.push('}');
                    }
                }
                if !greedy {
                    buf.push('?');
                }
                if precedence > 2 {
                    buf.push(')');
                }
            }
            Expr::Delegate {
                ref inner, casei, ..
            } => {
                // at the moment, delegate nodes are just atoms
                if casei {
                    buf.push_str("(?i:");
                }
                buf.push_str(inner);
                if casei {
                    buf.push(')');
                }
            }
            Expr::DefineGroup { .. } => {
                // DEFINE groups match nothing - output empty string for delegation
            }
            _ => panic!("attempting to format hard expr {:?}", self),
        }
    }
}

// precondition: ix > 0
fn prev_codepoint_ix(s: &str, mut ix: usize) -> usize {
    let bytes = s.as_bytes();
    loop {
        ix -= 1;
        // fancy bit magic for ranges 0..0x80 + 0xc0..
        if (bytes[ix] as i8) >= -0x40 {
            break;
        }
    }
    ix
}

fn codepoint_len(b: u8) -> usize {
    match b {
        b if b < 0x80 => 1,
        b if b < 0xe0 => 2,
        b if b < 0xf0 => 3,
        _ => 4,
    }
}

/// Returns the smallest possible index of the next valid UTF-8 sequence
/// starting after `i`.
/// Adapted from a function with the same name in the `regex` crate.
pub(crate) fn next_utf8(text: &str, i: usize) -> usize {
    let b = match text.as_bytes().get(i) {
        None => return i + 1,
        Some(&b) => b,
    };
    i + codepoint_len(b)
}

// If this returns false, then there is no possible backref in the re

// Both potential implementations are turned off, because we currently
// always need to do a deeper analysis because of 1-character
// look-behind. If we could call a find_from_pos method of regex::Regex,
// it would make sense to bring this back.
/*
pub fn detect_possible_backref(re: &str) -> bool {
    let mut last = b'\x00';
    for b in re.as_bytes() {
        if b'0' <= *b && *b <= b'9' && last == b'\\' { return true; }
        last = *b;
    }
    false
}

pub fn detect_possible_backref(re: &str) -> bool {
    let mut bytes = re.as_bytes();
    loop {
        match memchr::memchr(b'\\', &bytes[..bytes.len() - 1]) {
            Some(i) => {
                bytes = &bytes[i + 1..];
                let c = bytes[0];
                if b'0' <= c && c <= b'9' { return true; }
            }
            None => return false
        }
    }
}
*/

/// The internal module only exists so that the toy example can access internals for debugging and
/// experimenting.
#[doc(hidden)]
pub mod internal {
    pub use crate::analyze::{analyze, can_compile_as_anchored, AnalyzeContext, Info};
    pub use crate::compile::{compile, CompileOptions};
    pub use crate::optimize::optimize;
    pub use crate::parse_flags::{
        FLAG_CASEI, FLAG_CRLF, FLAG_DOTNL, FLAG_IGNORE_NUMBERED_GROUPS_WHEN_NAMED_GROUPS_EXIST,
        FLAG_IGNORE_SPACE, FLAG_MULTI, FLAG_ONIGURUMA_MODE, FLAG_UNICODE,
    };
    pub use crate::vm::{run_default, run_trace, Insn, Prog};
}

#[cfg(test)]
mod tests {
    use alloc::borrow::Cow;
    use alloc::boxed::Box;
    use alloc::string::{String, ToString};
    use alloc::sync::Arc;
    use alloc::vec::Vec;
    use alloc::{format, vec};

    use crate::parse::{make_group, make_literal};
    use crate::{Absent, Expr, Regex, RegexImpl};

    //use detect_possible_backref;

    // tests for to_str

    fn to_str(e: Expr) -> String {
        let mut s = String::new();
        e.to_str(&mut s, 0);
        s
    }

    #[test]
    fn to_str_concat_alt() {
        let e = Expr::Concat(vec![
            Expr::Alt(vec![make_literal("a"), make_literal("b")]),
            make_literal("c"),
        ]);
        assert_eq!(to_str(e), "(?:a|b)c");
    }

    #[test]
    fn to_str_rep_concat() {
        let e = Expr::Repeat {
            child: Box::new(Expr::Concat(vec![make_literal("a"), make_literal("b")])),
            lo: 2,
            hi: 3,
            greedy: true,
        };
        assert_eq!(to_str(e), "(?:ab){2,3}");
    }

    #[test]
    fn to_str_group_alt() {
        let e = Expr::Group(Arc::new(Expr::Alt(vec![
            make_literal("a"),
            make_literal("b"),
        ])));
        assert_eq!(to_str(e), "(a|b)");
    }

    #[test]
    fn as_str_debug() {
        let s = r"(a+)b\1";
        let regex = Regex::new(s).unwrap();
        assert_eq!(s, regex.as_str());
        assert_eq!(s, format!("{:?}", regex));
    }

    #[test]
    fn display() {
        let s = r"(a+)b\1";
        let regex = Regex::new(s).unwrap();
        assert_eq!(s, format!("{}", regex));
    }

    #[test]
    fn from_str() {
        let s = r"(a+)b\1";
        let regex = s.parse::<Regex>().unwrap();
        assert_eq!(regex.as_str(), s);
    }

    #[test]
    fn to_str_repeat() {
        fn repeat(lo: usize, hi: usize, greedy: bool) -> Expr {
            Expr::Repeat {
                child: Box::new(make_literal("a")),
                lo,
                hi,
                greedy,
            }
        }

        assert_eq!(to_str(repeat(2, 2, true)), "a{2}");
        assert_eq!(to_str(repeat(2, 2, false)), "a{2}?");
        assert_eq!(to_str(repeat(2, 3, true)), "a{2,3}");
        assert_eq!(to_str(repeat(2, 3, false)), "a{2,3}?");
        assert_eq!(to_str(repeat(2, usize::MAX, true)), "a{2,}");
        assert_eq!(to_str(repeat(2, usize::MAX, false)), "a{2,}?");
        assert_eq!(to_str(repeat(0, 1, true)), "a?");
        assert_eq!(to_str(repeat(0, 1, false)), "a??");
        assert_eq!(to_str(repeat(0, usize::MAX, true)), "a*");
        assert_eq!(to_str(repeat(0, usize::MAX, false)), "a*?");
        assert_eq!(to_str(repeat(1, usize::MAX, true)), "a+");
        assert_eq!(to_str(repeat(1, usize::MAX, false)), "a+?");
    }

    #[test]
    fn escape() {
        // Check that strings that need no quoting are borrowed, and that non-special punctuation
        // is not quoted.
        match crate::escape("@foo") {
            Cow::Borrowed(s) => assert_eq!(s, "@foo"),
            _ => panic!("Value should be borrowed."),
        }

        // Check typical usage.
        assert_eq!(crate::escape("fo*o").into_owned(), "fo\\*o");

        // Check that multibyte characters are handled correctly.
        assert_eq!(crate::escape("fø*ø").into_owned(), "fø\\*ø");
    }

    #[test]
    fn trailing_positive_lookahead_wrap_capture_group_fixup() {
        let s = r"a+(?=c)";
        let regex = s.parse::<Regex>().unwrap();
        assert!(matches!(regex.inner,
            RegexImpl::Wrap { explicit_capture_group_0: true, .. }),
            "trailing positive lookahead for an otherwise easy pattern should avoid going through the VM");
        assert_eq!(s, regex.as_str());
        assert_eq!(s, format!("{:?}", regex));
    }

    #[test]
    fn easy_regex() {
        let s = r"(a+)b";
        let regex = s.parse::<Regex>().unwrap();
        assert!(
            matches!(regex.inner, RegexImpl::Wrap { explicit_capture_group_0: false, .. }),
            "easy pattern should avoid going through the VM, and capture group 0 should be implicit"
        );

        assert_eq!(s, regex.as_str());
        assert_eq!(s, format!("{:?}", regex));
    }

    #[test]
    fn hard_regex() {
        let s = r"(a+)(?>c)";
        let regex = s.parse::<Regex>().unwrap();
        assert!(
            matches!(regex.inner, RegexImpl::Fancy { .. }),
            "hard regex should be compiled into a VM"
        );
        assert_eq!(s, regex.as_str());
        assert_eq!(s, format!("{:?}", regex));
    }

    /*
    #[test]
    fn detect_backref() {
        assert_eq!(detect_possible_backref("a0a1a2"), false);
        assert_eq!(detect_possible_backref("a0a1\\a2"), false);
        assert_eq!(detect_possible_backref("a0a\\1a2"), true);
        assert_eq!(detect_possible_backref("a0a1a2\\"), false);
    }
    */

    #[test]
    fn test_is_leaf_node_leaf_nodes() {
        // Test all leaf node variants
        assert!(Expr::Empty.is_leaf_node());
        assert!(Expr::Any {
            newline: false,
            crlf: false
        }
        .is_leaf_node());
        assert!(Expr::Any {
            newline: true,
            crlf: false
        }
        .is_leaf_node());
        assert!(Expr::Assertion(crate::Assertion::StartText).is_leaf_node());
        assert!(Expr::Literal {
            val: "test".to_string(),
            casei: false
        }
        .is_leaf_node());
        assert!(Expr::Delegate {
            inner: "[0-9]".to_string(),
            casei: false
        }
        .is_leaf_node());
        assert!(Expr::Backref {
            group: 1,
            casei: false
        }
        .is_leaf_node());
        assert!(Expr::BackrefWithRelativeRecursionLevel {
            group: 1,
            relative_level: -1,
            casei: false
        }
        .is_leaf_node());
        assert!(Expr::KeepOut.is_leaf_node());
        assert!(Expr::ContinueFromPreviousMatchEnd.is_leaf_node());
        assert!(Expr::BackrefExistsCondition {
            group: 1,
            relative_recursion_level: None
        }
        .is_leaf_node());
        assert!(Expr::BacktrackingControlVerb(crate::BacktrackingControlVerb::Fail).is_leaf_node());
        assert!(Expr::SubroutineCall(1).is_leaf_node());

        assert!(Expr::Absent(Absent::Clear).is_leaf_node());
    }

    #[test]
    fn test_is_leaf_node_non_leaf_nodes() {
        // Test all non-leaf node variants
        assert!(!Expr::Concat(vec![make_literal("a")]).is_leaf_node());
        assert!(!Expr::Alt(vec![make_literal("a"), make_literal("b")]).is_leaf_node());
        assert!(!make_group(make_literal("a")).is_leaf_node());
        assert!(
            !Expr::LookAround(Box::new(make_literal("a")), crate::LookAround::LookAhead)
                .is_leaf_node()
        );
        assert!(!Expr::Repeat {
            child: Box::new(make_literal("a")),
            lo: 0,
            hi: 1,
            greedy: true
        }
        .is_leaf_node());
        assert!(!Expr::AtomicGroup(Box::new(make_literal("a"))).is_leaf_node());
        assert!(!Expr::Conditional {
            condition: Box::new(Expr::BackrefExistsCondition {
                group: 1,
                relative_recursion_level: None
            }),
            true_branch: Box::new(make_literal("a")),
            false_branch: Box::new(Expr::Empty)
        }
        .is_leaf_node());

        assert!(!Expr::Absent(Absent::Repeater(Box::new(make_literal("a")))).is_leaf_node());
        assert!(!Expr::Absent(Absent::Expression {
            absent: Box::new(make_literal("/*")),
            exp: Box::new(Expr::Repeat {
                child: Box::new(Expr::Any {
                    newline: true,
                    crlf: false
                }),
                lo: 0,
                hi: usize::MAX,
                greedy: true
            })
        })
        .is_leaf_node());
        assert!(!Expr::Absent(Absent::Stopper(Box::new(make_literal("/*")))).is_leaf_node());
    }

    #[test]
    fn test_children_iter_empty() {
        // Leaf nodes should return empty iterator
        let expr = Expr::Empty;
        let mut iter = expr.children_iter();
        assert!(iter.next().is_none());

        let expr = make_literal("test");
        let mut iter = expr.children_iter();
        assert!(iter.next().is_none());
    }

    #[test]
    fn test_children_iter_single() {
        // Group, LookAround, AtomicGroup, Repeat should return single child
        let child = make_literal("a");
        let expr = make_group(child.clone());
        let children: Vec<_> = expr.children_iter().collect();
        assert_eq!(children.len(), 1);

        let expr = Expr::Repeat {
            child: Box::new(child.clone()),
            lo: 0,
            hi: 1,
            greedy: true,
        };
        let children: Vec<_> = expr.children_iter().collect();
        assert_eq!(children.len(), 1);
    }

    #[test]
    fn test_children_iter_vec() {
        // Concat and Alt should return all children
        let children_vec = vec![make_literal("a"), make_literal("b"), make_literal("c")];
        let expr = Expr::Concat(children_vec.clone());
        let children: Vec<_> = expr.children_iter().collect();
        assert_eq!(children.len(), 3);

        let expr = Expr::Alt(children_vec);
        let children: Vec<_> = expr.children_iter().collect();
        assert_eq!(children.len(), 3);
    }

    #[test]
    fn test_children_iter_triple() {
        // Conditional should return three children
        let expr = Expr::Conditional {
            condition: Box::new(Expr::BackrefExistsCondition {
                group: 1,
                relative_recursion_level: None,
            }),
            true_branch: Box::new(make_literal("a")),
            false_branch: Box::new(make_literal("b")),
        };
        let children: Vec<_> = expr.children_iter().collect();
        assert_eq!(children.len(), 3);

        // Absent expression should return two children
        let expr = Expr::Absent(Absent::Expression {
            absent: Box::new(make_literal("/*")),
            exp: Box::new(Expr::Repeat {
                child: Box::new(Expr::Any {
                    newline: true,
                    crlf: false,
                }),
                lo: 0,
                hi: usize::MAX,
                greedy: true,
            }),
        });
        let children: Vec<_> = expr.children_iter().collect();
        assert_eq!(children.len(), 2);
    }
}
