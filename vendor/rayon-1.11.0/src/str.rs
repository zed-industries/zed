//! Parallel iterator types for [strings]
//!
//! You will rarely need to interact with this module directly unless you need
//! to name one of the iterator types.
//!
//! Note: [`ParallelString::par_split()`] and [`par_split_terminator()`]
//! reference a `Pattern` trait which is not visible outside this crate.
//! This trait is intentionally kept private, for use only by Rayon itself.
//! It is implemented for `char`, `&[char]`, `[char; N]`, `&[char; N]`,
//! and any function or closure `F: Fn(char) -> bool + Sync + Send`.
//!
//! [`par_split_terminator()`]: ParallelString::par_split_terminator()
//! [strings]: std::str

use crate::iter::plumbing::*;
use crate::iter::*;
use crate::split_producer::*;

/// Test if a byte is the start of a UTF-8 character.
/// (extracted from `str::is_char_boundary`)
#[inline]
fn is_char_boundary(b: u8) -> bool {
    // This is bit magic equivalent to: b < 128 || b >= 192
    (b as i8) >= -0x40
}

/// Find the index of a character boundary near the midpoint.
#[inline]
fn find_char_midpoint(chars: &str) -> usize {
    let mid = chars.len() / 2;

    // We want to split near the midpoint, but we need to find an actual
    // character boundary.  So we look at the raw bytes, first scanning
    // forward from the midpoint for a boundary, then trying backward.
    let (left, right) = chars.as_bytes().split_at(mid);
    match right.iter().copied().position(is_char_boundary) {
        Some(i) => mid + i,
        None => left
            .iter()
            .copied()
            .rposition(is_char_boundary)
            .unwrap_or(0),
    }
}

/// Try to split a string near the midpoint.
#[inline]
fn split(chars: &str) -> Option<(&str, &str)> {
    let index = find_char_midpoint(chars);
    if index > 0 {
        Some(chars.split_at(index))
    } else {
        None
    }
}

/// Parallel extensions for strings.
pub trait ParallelString {
    /// Returns a plain string slice, which is used to implement the rest of
    /// the parallel methods.
    fn as_parallel_string(&self) -> &str;

    /// Returns a parallel iterator over the characters of a string.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let max = "hello".par_chars().max_by_key(|c| *c as i32);
    /// assert_eq!(Some('o'), max);
    /// ```
    fn par_chars(&self) -> Chars<'_> {
        Chars {
            chars: self.as_parallel_string(),
        }
    }

    /// Returns a parallel iterator over the characters of a string, with their positions.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let min = "hello".par_char_indices().min_by_key(|&(_i, c)| c as i32);
    /// assert_eq!(Some((1, 'e')), min);
    /// ```
    fn par_char_indices(&self) -> CharIndices<'_> {
        CharIndices {
            chars: self.as_parallel_string(),
        }
    }

    /// Returns a parallel iterator over the bytes of a string.
    ///
    /// Note that multi-byte sequences (for code points greater than `U+007F`)
    /// are produced as separate items, but will not be split across threads.
    /// If you would prefer an indexed iterator without that guarantee, consider
    /// `string.as_bytes().par_iter().copied()` instead.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let max = "hello".par_bytes().max();
    /// assert_eq!(Some(b'o'), max);
    /// ```
    fn par_bytes(&self) -> Bytes<'_> {
        Bytes {
            chars: self.as_parallel_string(),
        }
    }

    /// Returns a parallel iterator over a string encoded as UTF-16.
    ///
    /// Note that surrogate pairs (for code points greater than `U+FFFF`) are
    /// produced as separate items, but will not be split across threads.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    ///
    /// let max = "hello".par_encode_utf16().max();
    /// assert_eq!(Some(b'o' as u16), max);
    ///
    /// let text = "Zażółć gęślą jaźń";
    /// let utf8_len = text.len();
    /// let utf16_len = text.par_encode_utf16().count();
    /// assert!(utf16_len <= utf8_len);
    /// ```
    fn par_encode_utf16(&self) -> EncodeUtf16<'_> {
        EncodeUtf16 {
            chars: self.as_parallel_string(),
        }
    }

    /// Returns a parallel iterator over substrings separated by a
    /// given character or predicate, similar to `str::split`.
    ///
    /// Note: the `Pattern` trait is private, for use only by Rayon itself.
    /// It is implemented for `char`, `&[char]`, `[char; N]`, `&[char; N]`,
    /// and any function or closure `F: Fn(char) -> bool + Sync + Send`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let total = "1, 2, buckle, 3, 4, door"
    ///    .par_split(',')
    ///    .filter_map(|s| s.trim().parse::<i32>().ok())
    ///    .sum();
    /// assert_eq!(10, total);
    /// ```
    fn par_split<P: Pattern>(&self, separator: P) -> Split<'_, P> {
        Split::new(self.as_parallel_string(), separator)
    }

    /// Returns a parallel iterator over substrings separated by a
    /// given character or predicate, keeping the matched part as a terminator
    /// of the substring similar to `str::split_inclusive`.
    ///
    /// Note: the `Pattern` trait is private, for use only by Rayon itself.
    /// It is implemented for `char`, `&[char]`, `[char; N]`, `&[char; N]`,
    /// and any function or closure `F: Fn(char) -> bool + Sync + Send`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let lines: Vec<_> = "Mary had a little lamb\nlittle lamb\nlittle lamb."
    ///    .par_split_inclusive('\n')
    ///    .collect();
    /// assert_eq!(lines, ["Mary had a little lamb\n", "little lamb\n", "little lamb."]);
    /// ```
    fn par_split_inclusive<P: Pattern>(&self, separator: P) -> SplitInclusive<'_, P> {
        SplitInclusive::new(self.as_parallel_string(), separator)
    }

    /// Returns a parallel iterator over substrings terminated by a
    /// given character or predicate, similar to `str::split_terminator`.
    /// It's equivalent to `par_split`, except it doesn't produce an empty
    /// substring after a trailing terminator.
    ///
    /// Note: the `Pattern` trait is private, for use only by Rayon itself.
    /// It is implemented for `char`, `&[char]`, `[char; N]`, `&[char; N]`,
    /// and any function or closure `F: Fn(char) -> bool + Sync + Send`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let parts: Vec<_> = "((1 + 3) * 2)"
    ///     .par_split_terminator(|c| c == '(' || c == ')')
    ///     .collect();
    /// assert_eq!(vec!["", "", "1 + 3", " * 2"], parts);
    /// ```
    fn par_split_terminator<P: Pattern>(&self, terminator: P) -> SplitTerminator<'_, P> {
        SplitTerminator::new(self.as_parallel_string(), terminator)
    }

    /// Returns a parallel iterator over the lines of a string, ending with an
    /// optional carriage return and with a newline (`\r\n` or just `\n`).
    /// The final line ending is optional, and line endings are not included in
    /// the output strings.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let lengths: Vec<_> = "hello world\nfizbuzz"
    ///     .par_lines()
    ///     .map(|l| l.len())
    ///     .collect();
    /// assert_eq!(vec![11, 7], lengths);
    /// ```
    fn par_lines(&self) -> Lines<'_> {
        Lines(self.as_parallel_string())
    }

    /// Returns a parallel iterator over the sub-slices of a string that are
    /// separated by any amount of whitespace.
    ///
    /// As with `str::split_whitespace`, 'whitespace' is defined according to
    /// the terms of the Unicode Derived Core Property `White_Space`.
    /// If you only want to split on ASCII whitespace instead, use
    /// [`par_split_ascii_whitespace`][`ParallelString::par_split_ascii_whitespace`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let longest = "which is the longest word?"
    ///     .par_split_whitespace()
    ///     .max_by_key(|word| word.len());
    /// assert_eq!(Some("longest"), longest);
    /// ```
    ///
    /// All kinds of whitespace are considered:
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let words: Vec<&str> = " Mary   had\ta\u{2009}little  \n\t lamb"
    ///     .par_split_whitespace()
    ///     .collect();
    /// assert_eq!(words, ["Mary", "had", "a", "little", "lamb"]);
    /// ```
    ///
    /// If the string is empty or all whitespace, the iterator yields no string slices:
    ///
    /// ```
    /// use rayon::prelude::*;
    /// assert_eq!("".par_split_whitespace().count(), 0);
    /// assert_eq!("   ".par_split_whitespace().count(), 0);
    /// ```
    fn par_split_whitespace(&self) -> SplitWhitespace<'_> {
        SplitWhitespace(self.as_parallel_string())
    }

    /// Returns a parallel iterator over the sub-slices of a string that are
    /// separated by any amount of ASCII whitespace.
    ///
    /// To split by Unicode `White_Space` instead, use
    /// [`par_split_whitespace`][`ParallelString::par_split_whitespace`].
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let longest = "which is the longest word?"
    ///     .par_split_ascii_whitespace()
    ///     .max_by_key(|word| word.len());
    /// assert_eq!(Some("longest"), longest);
    /// ```
    ///
    /// All kinds of ASCII whitespace are considered, but not Unicode `White_Space`:
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let words: Vec<&str> = " Mary   had\ta\u{2009}little  \n\t lamb"
    ///     .par_split_ascii_whitespace()
    ///     .collect();
    /// assert_eq!(words, ["Mary", "had", "a\u{2009}little", "lamb"]);
    /// ```
    ///
    /// If the string is empty or all ASCII whitespace, the iterator yields no string slices:
    ///
    /// ```
    /// use rayon::prelude::*;
    /// assert_eq!("".par_split_whitespace().count(), 0);
    /// assert_eq!("   ".par_split_whitespace().count(), 0);
    /// ```
    fn par_split_ascii_whitespace(&self) -> SplitAsciiWhitespace<'_> {
        SplitAsciiWhitespace(self.as_parallel_string())
    }

    /// Returns a parallel iterator over substrings that match a
    /// given character or predicate, similar to `str::matches`.
    ///
    /// Note: the `Pattern` trait is private, for use only by Rayon itself.
    /// It is implemented for `char`, `&[char]`, `[char; N]`, `&[char; N]`,
    /// and any function or closure `F: Fn(char) -> bool + Sync + Send`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let total = "1, 2, buckle, 3, 4, door"
    ///    .par_matches(char::is_numeric)
    ///    .map(|s| s.parse::<i32>().expect("digit"))
    ///    .sum();
    /// assert_eq!(10, total);
    /// ```
    fn par_matches<P: Pattern>(&self, pattern: P) -> Matches<'_, P> {
        Matches {
            chars: self.as_parallel_string(),
            pattern,
        }
    }

    /// Returns a parallel iterator over substrings that match a given character
    /// or predicate, with their positions, similar to `str::match_indices`.
    ///
    /// Note: the `Pattern` trait is private, for use only by Rayon itself.
    /// It is implemented for `char`, `&[char]`, `[char; N]`, `&[char; N]`,
    /// and any function or closure `F: Fn(char) -> bool + Sync + Send`.
    ///
    /// # Examples
    ///
    /// ```
    /// use rayon::prelude::*;
    /// let digits: Vec<_> = "1, 2, buckle, 3, 4, door"
    ///    .par_match_indices(char::is_numeric)
    ///    .collect();
    /// assert_eq!(digits, vec![(0, "1"), (3, "2"), (14, "3"), (17, "4")]);
    /// ```
    fn par_match_indices<P: Pattern>(&self, pattern: P) -> MatchIndices<'_, P> {
        MatchIndices {
            chars: self.as_parallel_string(),
            pattern,
        }
    }
}

impl ParallelString for str {
    #[inline]
    fn as_parallel_string(&self) -> &str {
        self
    }
}

// /////////////////////////////////////////////////////////////////////////

/// We hide the `Pattern` trait in a private module, as its API is not meant
/// for general consumption.  If we could have privacy on trait items, then it
/// would be nicer to have its basic existence and implementors public while
/// keeping all of the methods private.
mod private {
    use crate::iter::plumbing::Folder;

    /// Pattern-matching trait for `ParallelString`, somewhat like a mix of
    /// `std::str::pattern::{Pattern, Searcher}`.
    ///
    /// Implementing this trait is not permitted outside of `rayon`.
    pub trait Pattern: Sized + Sync + Send {
        private_decl! {}
        fn find_in(&self, haystack: &str) -> Option<usize>;
        fn rfind_in(&self, haystack: &str) -> Option<usize>;
        fn is_suffix_of(&self, haystack: &str) -> bool;
        fn fold_splits<'ch, F>(&self, haystack: &'ch str, folder: F, skip_last: bool) -> F
        where
            F: Folder<&'ch str>;
        fn fold_inclusive_splits<'ch, F>(&self, haystack: &'ch str, folder: F) -> F
        where
            F: Folder<&'ch str>;
        fn fold_matches<'ch, F>(&self, haystack: &'ch str, folder: F) -> F
        where
            F: Folder<&'ch str>;
        fn fold_match_indices<'ch, F>(&self, haystack: &'ch str, folder: F, base: usize) -> F
        where
            F: Folder<(usize, &'ch str)>;
    }
}
use self::private::Pattern;

#[inline]
fn offset<T>(base: usize) -> impl Fn((usize, T)) -> (usize, T) {
    move |(i, x)| (base + i, x)
}

macro_rules! impl_pattern {
    (&$self:ident => $pattern:expr) => {
        private_impl! {}

        #[inline]
        fn find_in(&$self, chars: &str) -> Option<usize> {
            chars.find($pattern)
        }

        #[inline]
        fn rfind_in(&$self, chars: &str) -> Option<usize> {
            chars.rfind($pattern)
        }

        #[inline]
        fn is_suffix_of(&$self, chars: &str) -> bool {
            chars.ends_with($pattern)
        }

        fn fold_splits<'ch, F>(&$self, chars: &'ch str, folder: F, skip_last: bool) -> F
        where
            F: Folder<&'ch str>,
        {
            let mut split = chars.split($pattern);
            if skip_last {
                split.next_back();
            }
            folder.consume_iter(split)
        }

        fn fold_inclusive_splits<'ch, F>(&$self, chars: &'ch str, folder: F) -> F
        where
            F: Folder<&'ch str>,
        {
            folder.consume_iter(chars.split_inclusive($pattern))
        }

        fn fold_matches<'ch, F>(&$self, chars: &'ch str, folder: F) -> F
        where
            F: Folder<&'ch str>,
        {
            folder.consume_iter(chars.matches($pattern))
        }

        fn fold_match_indices<'ch, F>(&$self, chars: &'ch str, folder: F, base: usize) -> F
        where
            F: Folder<(usize, &'ch str)>,
        {
            folder.consume_iter(chars.match_indices($pattern).map(offset(base)))
        }
    }
}

impl Pattern for char {
    impl_pattern!(&self => *self);
}

impl Pattern for &[char] {
    impl_pattern!(&self => *self);
}

impl<const N: usize> Pattern for [char; N] {
    impl_pattern!(&self => *self);
}

impl<const N: usize> Pattern for &[char; N] {
    impl_pattern!(&self => *self);
}

impl<FN: Sync + Send + Fn(char) -> bool> Pattern for FN {
    impl_pattern!(&self => self);
}

// /////////////////////////////////////////////////////////////////////////

/// Parallel iterator over the characters of a string
#[derive(Debug, Clone)]
pub struct Chars<'ch> {
    chars: &'ch str,
}

struct CharsProducer<'ch> {
    chars: &'ch str,
}

impl<'ch> ParallelIterator for Chars<'ch> {
    type Item = char;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge_unindexed(CharsProducer { chars: self.chars }, consumer)
    }
}

impl<'ch> UnindexedProducer for CharsProducer<'ch> {
    type Item = char;

    fn split(self) -> (Self, Option<Self>) {
        match split(self.chars) {
            Some((left, right)) => (
                CharsProducer { chars: left },
                Some(CharsProducer { chars: right }),
            ),
            None => (self, None),
        }
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        folder.consume_iter(self.chars.chars())
    }
}

// /////////////////////////////////////////////////////////////////////////

/// Parallel iterator over the characters of a string, with their positions
#[derive(Debug, Clone)]
pub struct CharIndices<'ch> {
    chars: &'ch str,
}

struct CharIndicesProducer<'ch> {
    index: usize,
    chars: &'ch str,
}

impl<'ch> ParallelIterator for CharIndices<'ch> {
    type Item = (usize, char);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let producer = CharIndicesProducer {
            index: 0,
            chars: self.chars,
        };
        bridge_unindexed(producer, consumer)
    }
}

impl<'ch> UnindexedProducer for CharIndicesProducer<'ch> {
    type Item = (usize, char);

    fn split(self) -> (Self, Option<Self>) {
        match split(self.chars) {
            Some((left, right)) => (
                CharIndicesProducer {
                    chars: left,
                    ..self
                },
                Some(CharIndicesProducer {
                    chars: right,
                    index: self.index + left.len(),
                }),
            ),
            None => (self, None),
        }
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        let base = self.index;
        folder.consume_iter(self.chars.char_indices().map(offset(base)))
    }
}

// /////////////////////////////////////////////////////////////////////////

/// Parallel iterator over the bytes of a string
#[derive(Debug, Clone)]
pub struct Bytes<'ch> {
    chars: &'ch str,
}

struct BytesProducer<'ch> {
    chars: &'ch str,
}

impl<'ch> ParallelIterator for Bytes<'ch> {
    type Item = u8;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge_unindexed(BytesProducer { chars: self.chars }, consumer)
    }
}

impl<'ch> UnindexedProducer for BytesProducer<'ch> {
    type Item = u8;

    fn split(self) -> (Self, Option<Self>) {
        match split(self.chars) {
            Some((left, right)) => (
                BytesProducer { chars: left },
                Some(BytesProducer { chars: right }),
            ),
            None => (self, None),
        }
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        folder.consume_iter(self.chars.bytes())
    }
}

// /////////////////////////////////////////////////////////////////////////

/// Parallel iterator over a string encoded as UTF-16
#[derive(Debug, Clone)]
pub struct EncodeUtf16<'ch> {
    chars: &'ch str,
}

struct EncodeUtf16Producer<'ch> {
    chars: &'ch str,
}

impl<'ch> ParallelIterator for EncodeUtf16<'ch> {
    type Item = u16;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        bridge_unindexed(EncodeUtf16Producer { chars: self.chars }, consumer)
    }
}

impl<'ch> UnindexedProducer for EncodeUtf16Producer<'ch> {
    type Item = u16;

    fn split(self) -> (Self, Option<Self>) {
        match split(self.chars) {
            Some((left, right)) => (
                EncodeUtf16Producer { chars: left },
                Some(EncodeUtf16Producer { chars: right }),
            ),
            None => (self, None),
        }
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        folder.consume_iter(self.chars.encode_utf16())
    }
}

// /////////////////////////////////////////////////////////////////////////

/// Parallel iterator over substrings separated by a pattern
#[derive(Debug, Clone)]
pub struct Split<'ch, P: Pattern> {
    chars: &'ch str,
    separator: P,
}

impl<'ch, P: Pattern> Split<'ch, P> {
    fn new(chars: &'ch str, separator: P) -> Self {
        Split { chars, separator }
    }
}

impl<'ch, P: Pattern> ParallelIterator for Split<'ch, P> {
    type Item = &'ch str;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let producer = SplitProducer::new(self.chars, &self.separator);
        bridge_unindexed(producer, consumer)
    }
}

/// Implement support for `SplitProducer`.
impl<P: Pattern> Fissile<P> for &str {
    fn length(&self) -> usize {
        self.len()
    }

    fn midpoint(&self, end: usize) -> usize {
        // First find a suitable UTF-8 boundary.
        find_char_midpoint(&self[..end])
    }

    fn find(&self, separator: &P, start: usize, end: usize) -> Option<usize> {
        separator.find_in(&self[start..end])
    }

    fn rfind(&self, separator: &P, end: usize) -> Option<usize> {
        separator.rfind_in(&self[..end])
    }

    fn split_once<const INCL: bool>(self, index: usize) -> (Self, Self) {
        if INCL {
            // include the separator in the left side
            let separator = self[index..].chars().next().unwrap();
            self.split_at(index + separator.len_utf8())
        } else {
            let (left, right) = self.split_at(index);
            let mut right_iter = right.chars();
            right_iter.next(); // skip the separator
            (left, right_iter.as_str())
        }
    }

    fn fold_splits<F, const INCL: bool>(self, separator: &P, folder: F, skip_last: bool) -> F
    where
        F: Folder<Self>,
    {
        if INCL {
            debug_assert!(!skip_last);
            separator.fold_inclusive_splits(self, folder)
        } else {
            separator.fold_splits(self, folder, skip_last)
        }
    }
}

// /////////////////////////////////////////////////////////////////////////

/// Parallel iterator over substrings separated by a pattern
#[derive(Debug, Clone)]
pub struct SplitInclusive<'ch, P: Pattern> {
    chars: &'ch str,
    separator: P,
}

impl<'ch, P: Pattern> SplitInclusive<'ch, P> {
    fn new(chars: &'ch str, separator: P) -> Self {
        SplitInclusive { chars, separator }
    }
}

impl<'ch, P: Pattern> ParallelIterator for SplitInclusive<'ch, P> {
    type Item = &'ch str;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let producer = SplitInclusiveProducer::new_incl(self.chars, &self.separator);
        bridge_unindexed(producer, consumer)
    }
}

// /////////////////////////////////////////////////////////////////////////

/// Parallel iterator over substrings separated by a terminator pattern
#[derive(Debug, Clone)]
pub struct SplitTerminator<'ch, P: Pattern> {
    chars: &'ch str,
    terminator: P,
}

struct SplitTerminatorProducer<'ch, 'sep, P: Pattern> {
    splitter: SplitProducer<'sep, P, &'ch str>,
    skip_last: bool,
}

impl<'ch, P: Pattern> SplitTerminator<'ch, P> {
    fn new(chars: &'ch str, terminator: P) -> Self {
        SplitTerminator { chars, terminator }
    }
}

impl<'ch, 'sep, P: Pattern + 'sep> SplitTerminatorProducer<'ch, 'sep, P> {
    fn new(chars: &'ch str, terminator: &'sep P) -> Self {
        SplitTerminatorProducer {
            splitter: SplitProducer::new(chars, terminator),
            skip_last: chars.is_empty() || terminator.is_suffix_of(chars),
        }
    }
}

impl<'ch, P: Pattern> ParallelIterator for SplitTerminator<'ch, P> {
    type Item = &'ch str;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let producer = SplitTerminatorProducer::new(self.chars, &self.terminator);
        bridge_unindexed(producer, consumer)
    }
}

impl<'ch, 'sep, P: Pattern + 'sep> UnindexedProducer for SplitTerminatorProducer<'ch, 'sep, P> {
    type Item = &'ch str;

    fn split(mut self) -> (Self, Option<Self>) {
        let (left, right) = self.splitter.split();
        self.splitter = left;
        let right = right.map(|right| {
            let skip_last = self.skip_last;
            self.skip_last = false;
            SplitTerminatorProducer {
                splitter: right,
                skip_last,
            }
        });
        (self, right)
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        self.splitter.fold_with(folder, self.skip_last)
    }
}

// /////////////////////////////////////////////////////////////////////////

/// Parallel iterator over lines in a string
#[derive(Debug, Clone)]
pub struct Lines<'ch>(&'ch str);

#[inline]
fn no_carriage_return(line: &str) -> &str {
    line.strip_suffix('\r').unwrap_or(line)
}

impl<'ch> ParallelIterator for Lines<'ch> {
    type Item = &'ch str;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.0
            .par_split_terminator('\n')
            .map(no_carriage_return)
            .drive_unindexed(consumer)
    }
}

// /////////////////////////////////////////////////////////////////////////

/// Parallel iterator over substrings separated by whitespace
#[derive(Debug, Clone)]
pub struct SplitWhitespace<'ch>(&'ch str);

#[inline]
fn not_empty(s: &&str) -> bool {
    !s.is_empty()
}

impl<'ch> ParallelIterator for SplitWhitespace<'ch> {
    type Item = &'ch str;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.0
            .par_split(char::is_whitespace)
            .filter(not_empty)
            .drive_unindexed(consumer)
    }
}

// /////////////////////////////////////////////////////////////////////////

/// Parallel iterator over substrings separated by ASCII whitespace
#[derive(Debug, Clone)]
pub struct SplitAsciiWhitespace<'ch>(&'ch str);

#[inline]
fn is_ascii_whitespace(c: char) -> bool {
    c.is_ascii_whitespace()
}

impl<'ch> ParallelIterator for SplitAsciiWhitespace<'ch> {
    type Item = &'ch str;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        self.0
            .par_split(is_ascii_whitespace)
            .filter(not_empty)
            .drive_unindexed(consumer)
    }
}

// /////////////////////////////////////////////////////////////////////////

/// Parallel iterator over substrings that match a pattern
#[derive(Debug, Clone)]
pub struct Matches<'ch, P: Pattern> {
    chars: &'ch str,
    pattern: P,
}

struct MatchesProducer<'ch, 'pat, P: Pattern> {
    chars: &'ch str,
    pattern: &'pat P,
}

impl<'ch, P: Pattern> ParallelIterator for Matches<'ch, P> {
    type Item = &'ch str;

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let producer = MatchesProducer {
            chars: self.chars,
            pattern: &self.pattern,
        };
        bridge_unindexed(producer, consumer)
    }
}

impl<'ch, 'pat, P: Pattern> UnindexedProducer for MatchesProducer<'ch, 'pat, P> {
    type Item = &'ch str;

    fn split(self) -> (Self, Option<Self>) {
        match split(self.chars) {
            Some((left, right)) => (
                MatchesProducer {
                    chars: left,
                    ..self
                },
                Some(MatchesProducer {
                    chars: right,
                    ..self
                }),
            ),
            None => (self, None),
        }
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        self.pattern.fold_matches(self.chars, folder)
    }
}

// /////////////////////////////////////////////////////////////////////////

/// Parallel iterator over substrings that match a pattern, with their positions
#[derive(Debug, Clone)]
pub struct MatchIndices<'ch, P: Pattern> {
    chars: &'ch str,
    pattern: P,
}

struct MatchIndicesProducer<'ch, 'pat, P: Pattern> {
    index: usize,
    chars: &'ch str,
    pattern: &'pat P,
}

impl<'ch, P: Pattern> ParallelIterator for MatchIndices<'ch, P> {
    type Item = (usize, &'ch str);

    fn drive_unindexed<C>(self, consumer: C) -> C::Result
    where
        C: UnindexedConsumer<Self::Item>,
    {
        let producer = MatchIndicesProducer {
            index: 0,
            chars: self.chars,
            pattern: &self.pattern,
        };
        bridge_unindexed(producer, consumer)
    }
}

impl<'ch, 'pat, P: Pattern> UnindexedProducer for MatchIndicesProducer<'ch, 'pat, P> {
    type Item = (usize, &'ch str);

    fn split(self) -> (Self, Option<Self>) {
        match split(self.chars) {
            Some((left, right)) => (
                MatchIndicesProducer {
                    chars: left,
                    ..self
                },
                Some(MatchIndicesProducer {
                    chars: right,
                    index: self.index + left.len(),
                    ..self
                }),
            ),
            None => (self, None),
        }
    }

    fn fold_with<F>(self, folder: F) -> F
    where
        F: Folder<Self::Item>,
    {
        self.pattern
            .fold_match_indices(self.chars, folder, self.index)
    }
}
