//! Module containing zero-copy parsers.
//!
//! These parsers require the [`RangeStream`][] bound instead of a plain [`Stream`][].
//!
//! [`RangeStream`]: ../../stream/trait.RangeStream.html
//! [`Stream`]: ../../stream/trait.Stream.html

use crate::{
    error::{
        self, ParseError,
        ParseResult::{self, *},
        ResultExt, StreamError, Tracked,
    },
    lib::{convert::TryFrom, marker::PhantomData},
    parser::ParseMode,
};

#[cfg(feature = "std")]
use crate::lib::error::Error as StdError;

#[cfg(not(feature = "std"))]
use crate::lib::fmt;

use crate::stream::{
    uncons_range, uncons_while, uncons_while1, wrap_stream_error, Range as StreamRange,
    RangeStream, StreamErrorFor, StreamOnce,
};

use crate::Parser;

pub struct Range<Input>(Input::Range)
where
    Input: RangeStream;

impl<Input> Parser<Input> for Range<Input>
where
    Input: RangeStream,
    Input::Range: PartialEq + crate::stream::Range,
{
    type Output = Input::Range;
    type PartialState = ();

    #[inline]
    fn parse_lazy(
        &mut self,
        input: &mut Input,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error> {
        use crate::stream::Range;

        let position = input.position();
        match input.uncons_range(self.0.len()) {
            Ok(other) => {
                if other == self.0 {
                    CommitOk(other)
                } else {
                    PeekErr(Input::Error::empty(position).into())
                }
            }
            Err(err) => wrap_stream_error(input, err),
        }
    }
    fn add_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        // TODO Add unexpected message?
        errors.error.add_expected(error::Range(self.0.clone()));
    }
}

parser! {
    #[derive(Clone)]
    pub struct Recognize;
    type PartialState = <RecognizeWithValue<P> as Parser<Input>>::PartialState;
    /// Zero-copy parser which returns committed input range.
    ///
    /// [`combinator::recognize`][] is a non-`RangeStream` alternative.
    ///
    /// [`combinator::recognize`]: ../../parser/combinator/fn.recognize.html
    /// ```
    /// # extern crate combine;
    /// # use combine::parser::range::recognize;
    /// # use combine::parser::char::letter;
    /// # use combine::*;
    /// # fn main() {
    /// let mut parser = recognize(skip_many1(letter()));
    /// assert_eq!(parser.parse("hello world"), Ok(("hello", " world")));
    /// assert!(parser.parse("!").is_err());
    /// # }
    /// ```
    pub fn recognize[Input, P](parser: P)(Input) -> <Input as StreamOnce>::Range
    where [
        P: Parser<Input>,
        Input: RangeStream,
        <Input as StreamOnce>::Range: crate::stream::Range,
    ]
    {
        recognize_with_value(parser).map(|(range, _)| range)
    }
}

#[inline]
fn parse_partial_range<M, F, G, S, Input>(
    mode: M,
    input: &mut Input,
    distance_state: &mut usize,
    state: S,
    first: F,
    resume: G,
) -> ParseResult<Input::Range, Input::Error>
where
    M: ParseMode,
    F: FnOnce(&mut Input, S) -> ParseResult<Input::Range, <Input as StreamOnce>::Error>,
    G: FnOnce(&mut Input, S) -> ParseResult<Input::Range, <Input as StreamOnce>::Error>,
    Input: RangeStream,
{
    let before = input.checkpoint();

    if !input.is_partial() {
        first(input, state)
    } else if mode.is_first() || *distance_state == 0 {
        let result = first(input, state);
        if let CommitErr(_) = result {
            *distance_state = input.distance(&before);
            ctry!(input.reset(before).committed());
        }
        result
    } else {
        if input.uncons_range(*distance_state).is_err() {
            panic!("recognize errored when restoring the input stream to its expected state");
        }

        match resume(input, state) {
            CommitOk(_) | PeekOk(_) => (),
            PeekErr(err) => return PeekErr(err),
            CommitErr(err) => {
                *distance_state = input.distance(&before);
                ctry!(input.reset(before).committed());
                return CommitErr(err);
            }
        }

        let distance = input.distance(&before);
        ctry!(input.reset(before).committed());
        take(distance).parse_lazy(input).map(|range| {
            *distance_state = 0;
            range
        })
    }
}

#[derive(Clone)]
pub struct RecognizeWithValue<P>(P);

impl<Input, P> Parser<Input> for RecognizeWithValue<P>
where
    P: Parser<Input>,
    Input: RangeStream,
    <Input as StreamOnce>::Range: crate::stream::Range,
{
    type Output = (<Input as StreamOnce>::Range, P::Output);
    type PartialState = (usize, P::PartialState);

    parse_mode!(Input);
    #[inline]
    fn parse_mode<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>
    where
        M: ParseMode,
    {
        let (ref mut distance_state, ref mut child_state) = *state;

        let before = input.checkpoint();
        if !mode.is_first() && input.uncons_range(*distance_state).is_err() {
            panic!("recognize errored when restoring the input stream to its expected state");
        }

        let value = match self.0.parse_mode(mode, input, child_state) {
            CommitOk(x) | PeekOk(x) => x,
            PeekErr(err) => return PeekErr(err),
            CommitErr(err) => {
                *distance_state = input.distance(&before);
                ctry!(input.reset(before).committed());
                return CommitErr(err);
            }
        };

        let distance = input.distance(&before);
        ctry!(input.reset(before).committed());
        take(distance).parse_lazy(input).map(|range| {
            *distance_state = 0;
            (range, value)
        })
    }
    fn add_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        self.0.add_error(errors)
    }
}

/// Zero-copy parser which returns a pair: (committed input range, parsed value).
///
///
/// [`combinator::recognize_with_value`] is a non-`RangeStream` alternative.
///
/// [`combinator::recognize_with_value`]: recognize_with_value
/// ```
/// # extern crate combine;
/// # use combine::parser::range::recognize_with_value;
/// # use combine::parser::char::{digit, char};
/// # use combine::*;
/// # fn main() {
/// let mut parser = recognize_with_value((
///     skip_many1(digit()),
///     optional((attempt(char('.')), skip_many1(digit()))),
/// ).map(|(_, opt)| opt.is_some()));
///
/// assert_eq!(parser.parse("1234!"), Ok((("1234", false), "!")));
/// assert_eq!(parser.parse("1234.0001!"), Ok((("1234.0001", true), "!")));
/// assert!(parser.parse("!").is_err());
/// assert!(parser.parse("1234.").is_err());
/// # }
/// ```
pub fn recognize_with_value<Input, P>(parser: P) -> RecognizeWithValue<P>
where
    P: Parser<Input>,
    Input: RangeStream,
    <Input as StreamOnce>::Range: crate::stream::Range,
{
    RecognizeWithValue(parser)
}

/// Zero-copy parser which reads a range of length `i.len()` and succeeds if `i` is equal to that
/// range.
///
/// [`tokens`] is a non-`RangeStream` alternative.
///
/// [`tokens`]: super::token::tokens
/// ```
/// # extern crate combine;
/// # use combine::parser::range::range;
/// # use combine::*;
/// # fn main() {
/// let mut parser = range("hello");
/// let result = parser.parse("hello world");
/// assert_eq!(result, Ok(("hello", " world")));
/// let result = parser.parse("hel world");
/// assert!(result.is_err());
/// # }
/// ```
pub fn range<Input>(i: Input::Range) -> Range<Input>
where
    Input: RangeStream,
    Input::Range: PartialEq,
{
    Range(i)
}

pub struct Take<Input>(usize, PhantomData<fn(Input)>);
impl<Input> Parser<Input> for Take<Input>
where
    Input: RangeStream,
{
    type Output = Input::Range;
    type PartialState = ();

    #[inline]
    fn parse_lazy(
        &mut self,
        input: &mut Input,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error> {
        uncons_range(input, self.0)
    }
}

/// Zero-copy parser which reads a range of length `n`.
///
/// [`count_min_max`][] is a non-`RangeStream` alternative.
///
/// [`count_min_max`]: ../../parser/repeat/fn.count_min_max.html
/// ```
/// # extern crate combine;
/// # use combine::parser::range::take;
/// # use combine::*;
/// # fn main() {
/// let mut parser = take(1);
/// let result = parser.parse("1");
/// assert_eq!(result, Ok(("1", "")));
/// let mut parser = take(4);
/// let result = parser.parse("123abc");
/// assert_eq!(result, Ok(("123a", "bc")));
/// let result = parser.parse("abc");
/// assert!(result.is_err());
/// # }
/// ```
pub fn take<Input>(n: usize) -> Take<Input>
where
    Input: RangeStream,
{
    Take(n, PhantomData)
}

pub struct TakeWhile<Input, F>(F, PhantomData<fn(Input) -> Input>);
impl<Input, F> Parser<Input> for TakeWhile<Input, F>
where
    Input: RangeStream,
    Input::Range: crate::stream::Range,
    F: FnMut(Input::Token) -> bool,
{
    type Output = Input::Range;
    type PartialState = usize;

    parse_mode!(Input);
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>
    where
        M: ParseMode,
    {
        parse_partial_range(
            mode,
            input,
            state,
            &mut self.0,
            |input, predicate| uncons_while(input, predicate),
            |input, predicate| uncons_while(input, predicate),
        )
    }
}

/// Zero-copy parser which reads a range of 0 or more tokens which satisfy `f`.
///
/// [`many`][] is a non-`RangeStream` alternative.
///
/// [`many`]: ../../parser/repeat/fn.many.html
/// ```
/// # extern crate combine;
/// # use combine::parser::range::take_while;
/// # use combine::*;
/// # fn main() {
/// let mut parser = take_while(|c: char| c.is_digit(10));
/// let result = parser.parse("123abc");
/// assert_eq!(result, Ok(("123", "abc")));
/// let result = parser.parse("abc");
/// assert_eq!(result, Ok(("", "abc")));
/// # }
/// ```
pub fn take_while<Input, F>(f: F) -> TakeWhile<Input, F>
where
    Input: RangeStream,
    Input::Range: crate::stream::Range,
    F: FnMut(Input::Token) -> bool,
{
    TakeWhile(f, PhantomData)
}

pub struct TakeWhile1<Input, F>(F, PhantomData<fn(Input) -> Input>);
impl<Input, F> Parser<Input> for TakeWhile1<Input, F>
where
    Input: RangeStream,
    Input::Range: crate::stream::Range,
    F: FnMut(Input::Token) -> bool,
{
    type Output = Input::Range;
    type PartialState = usize;

    parse_mode!(Input);
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>
    where
        M: ParseMode,
    {
        parse_partial_range(
            mode,
            input,
            state,
            &mut self.0,
            |input, predicate| uncons_while1(input, predicate),
            |input, predicate| uncons_while(input, predicate),
        )
    }
}

/// Zero-copy parser which reads a range of 1 or more tokens which satisfy `f`.
///
/// [`many1`][] is a non-`RangeStream` alternative.
///
/// [`many1`]: ../../parser/repeat/fn.many1.html
/// ```
/// # extern crate combine;
/// # use combine::parser::range::take_while1;
/// # use combine::*;
/// # fn main() {
/// let mut parser = take_while1(|c: char| c.is_digit(10));
/// let result = parser.parse("123abc");
/// assert_eq!(result, Ok(("123", "abc")));
/// let result = parser.parse("abc");
/// assert!(result.is_err());
/// # }
/// ```
pub fn take_while1<Input, F>(f: F) -> TakeWhile1<Input, F>
where
    Input: RangeStream,
    Input::Range: crate::stream::Range,
    F: FnMut(Input::Token) -> bool,
{
    TakeWhile1(f, PhantomData)
}

pub struct TakeUntilRange<Input>(Input::Range)
where
    Input: RangeStream;
impl<Input> Parser<Input> for TakeUntilRange<Input>
where
    Input: RangeStream,
    Input::Range: PartialEq + crate::stream::Range,
{
    type Output = Input::Range;
    type PartialState = usize;

    #[inline]
    fn parse_partial(
        &mut self,
        input: &mut Input,
        to_consume: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error> {
        use crate::stream::Range;

        let len = self.0.len();
        let before = input.checkpoint();
        let mut first_stream_error = None;

        // Skip until the end of the last parse attempt
        ctry!(uncons_range(input, *to_consume));

        loop {
            let look_ahead_input = input.checkpoint();

            match input.uncons_range(len) {
                Ok(xs) => {
                    if xs == self.0 {
                        let distance = input.distance(&before) - len;
                        ctry!(input.reset(before).committed());

                        if let Ok(committed) = input.uncons_range(distance) {
                            if distance == 0 {
                                return PeekOk(committed);
                            } else {
                                *to_consume = 0;
                                return CommitOk(committed);
                            }
                        }

                        // We are guaranteed able to uncons to_consume characters here
                        // because we've already done it on look_ahead_input.
                        unreachable!();
                    } else {
                        // Reset the stream back to where it was when we entered the top of the loop
                        ctry!(input.reset(look_ahead_input).committed());

                        // Advance the stream by one token
                        if input.uncons().is_err() {
                            unreachable!();
                        }
                    }
                }
                Err(first_error) => {
                    // If we are unable to find a successful parse even after advancing with `uncons`
                    // below we must reset the stream to its state before the first error.
                    // If we don't we may try and match the range `::` against `:<EOF>` which would
                    // fail as only one `:` is present at this parse attempt. But when we later resume
                    // with more input we must start parsing again at the first time we errored so we
                    // can see the entire `::`
                    if first_stream_error.is_none() {
                        first_stream_error = Some((first_error, input.distance(&before)));
                    }

                    // Reset the stream back to where it was when we entered the top of the loop
                    ctry!(input.reset(look_ahead_input).committed());

                    // See if we can advance anyway
                    if input.uncons().is_err() {
                        let (first_error, first_error_distance) = first_stream_error.unwrap();

                        // Reset the stream
                        ctry!(input.reset(before).committed());
                        *to_consume = first_error_distance;

                        // Return the original error if uncons failed
                        return wrap_stream_error(input, first_error);
                    }
                }
            };
        }
    }
}

/// Zero-copy parser which reads a range of 0 or more tokens until `r` is found.
///
/// The range `r` will not be committed. If `r` is not found, the parser will
/// return an error.
///
/// [`repeat::take_until`][] is a non-`RangeStream` alternative.
///
/// [`repeat::take_until`]: ../../parser/repeat/fn.take_until.html
/// ```
/// # extern crate combine;
/// # use combine::parser::range::{range, take_until_range};
/// # use combine::*;
/// # fn main() {
/// let mut parser = take_until_range("\r\n");
/// let result = parser.parse("To: user@example.com\r\n");
/// assert_eq!(result, Ok(("To: user@example.com", "\r\n")));
/// let result = parser.parse("Hello, world\n");
/// assert!(result.is_err());
/// # }
/// ```
pub fn take_until_range<Input>(r: Input::Range) -> TakeUntilRange<Input>
where
    Input: RangeStream,
{
    TakeUntilRange(r)
}

#[derive(Debug, PartialEq)]
pub enum TakeRange {
    /// Found the pattern at this offset
    Found(usize),
    /// Did not find the pattern but the parser can skip ahead to this offset.
    NotFound(usize),
}

impl From<Option<usize>> for TakeRange {
    fn from(opt: Option<usize>) -> TakeRange {
        match opt {
            Some(i) => TakeRange::Found(i),
            None => TakeRange::NotFound(0),
        }
    }
}

pub struct TakeFn<F, Input> {
    searcher: F,
    _marker: PhantomData<fn(Input)>,
}

impl<Input, F, R> Parser<Input> for TakeFn<F, Input>
where
    F: FnMut(Input::Range) -> R,
    R: Into<TakeRange>,
    Input: RangeStream,
    Input::Range: crate::stream::Range,
{
    type Output = Input::Range;
    type PartialState = usize;

    parse_mode!(Input);
    #[inline]
    fn parse_mode<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        offset: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>
    where
        M: ParseMode,
    {
        let checkpoint = input.checkpoint();

        if mode.is_first() {
            *offset = 0;
        } else {
            let _ = input.uncons_range(*offset);
        }

        match (self.searcher)(input.range()).into() {
            TakeRange::Found(i) => {
                ctry!(input.reset(checkpoint).committed());
                let result = uncons_range(input, *offset + i);
                if result.is_ok() {
                    *offset = 0;
                }
                result
            }
            TakeRange::NotFound(next_offset) => {
                *offset = next_offset;

                let range = input.range();
                let _ = input.uncons_range(range.len());
                let position = input.position();
                ctry!(input.reset(checkpoint).committed());

                let err = Input::Error::from_error(position, StreamError::end_of_input());
                if !input.is_partial() && range.is_empty() {
                    PeekErr(err.into())
                } else {
                    CommitErr(err)
                }
            }
        }
    }
}

/// Searches the entire range using `searcher` and then consumes a range of `Some(n)`.
/// If `f` can not find anything in the range it must return `None/NotFound` which indicates an end of input error.
///
/// If partial parsing is used the `TakeRange` enum can be returned instead of `Option`. By
/// returning `TakeRange::NotFound(n)` it indicates that the input can skip ahead until `n`
/// when parsing is next resumed.
///
/// See [`take_until_bytes`](../byte/fn.take_until_bytes.html) for a usecase.
pub fn take_fn<F, R, Input>(searcher: F) -> TakeFn<F, Input>
where
    F: FnMut(Input::Range) -> R,
    R: Into<TakeRange>,
    Input: RangeStream,
    Input::Range: crate::stream::Range,
{
    TakeFn {
        searcher,
        _marker: PhantomData,
    }
}

#[cfg(feature = "std")]
parser! {
/// Takes a parser which parses a `length` then extracts a range of that length and returns it.
/// Commonly used in binary formats
///
/// ```
/// # use combine::parser::{byte::num::be_u16, range::length_prefix};
/// # use combine::*;
/// # fn main() {
/// let mut input = Vec::new();
/// input.extend_from_slice(&3u16.to_be_bytes());
/// input.extend_from_slice(b"1234");
///
/// let mut parser = length_prefix(be_u16());
/// let result = parser.parse(&input[..]);
/// assert_eq!(result, Ok((&b"123"[..], &b"4"[..])));
/// # }
/// ```
pub fn length_prefix[Input, P](len: P)(Input) -> Input::Range
where [
    Input: RangeStream,
    P: Parser<Input>,
    usize: TryFrom<P::Output>,
    <usize as TryFrom<P::Output>>::Error: StdError + Send + Sync + 'static,
]
{
    len
        .and_then(|u| {
            usize::try_from(u)
                .map_err(StreamErrorFor::<Input>::other)
        })
        .then_partial(|&mut len| take(len))
}
}

#[cfg(not(feature = "std"))]
parser! {
/// Takes a parser which parses a `length` then extracts a range of that length and returns it.
/// Commonly used in binary formats
///
/// ```
/// # use combine::parser::{byte::num::be_u16, range::length_prefix};
/// # use combine::*;
/// # fn main() {
/// let mut input = Vec::new();
/// input.extend_from_slice(&3u16.to_be_bytes());
/// input.extend_from_slice(b"1234");
///
/// let mut parser = length_prefix(be_u16());
/// let result = parser.parse(&input[..]);
/// assert_eq!(result, Ok((&b"123"[..], &b"4"[..])));
/// # }
/// ```
pub fn length_prefix[Input, P](len: P)(Input) -> Input::Range
where [
    Input: RangeStream,
    P: Parser<Input>,
    usize: TryFrom<P::Output>,
    <usize as TryFrom<P::Output>>::Error: fmt::Display + Send + Sync + 'static,
]
{
    len
        .and_then(|u| {
            usize::try_from(u)
                .map_err(StreamErrorFor::<Input>::message_format)
        })
        .then_partial(|&mut len| take(len))
}
}

#[cfg(test)]
mod tests {

    use crate::Parser;

    use super::*;

    #[test]
    fn take_while_test() {
        let result = take_while(|c: char| c.is_digit(10)).parse("123abc");
        assert_eq!(result, Ok(("123", "abc")));
        let result = take_while(|c: char| c.is_digit(10)).parse("abc");
        assert_eq!(result, Ok(("", "abc")));
    }

    #[test]
    fn take_while1_test() {
        let result = take_while1(|c: char| c.is_digit(10)).parse("123abc");
        assert_eq!(result, Ok(("123", "abc")));
        let result = take_while1(|c: char| c.is_digit(10)).parse("abc");
        assert!(result.is_err());
    }

    #[test]
    fn range_string_no_char_boundary_error() {
        let mut parser = range("hello");
        let result = parser.parse("hell\u{00EE} world");
        assert!(result.is_err());
    }

    #[test]
    fn take_until_range_1() {
        let result = take_until_range("\"").parse("Foo baz bar quux\"");
        assert_eq!(result, Ok(("Foo baz bar quux", "\"")));
    }

    #[test]
    fn take_until_range_2() {
        let result = take_until_range("===").parse("if ((pointless_comparison == 3) === true) {");
        assert_eq!(
            result,
            Ok(("if ((pointless_comparison == 3) ", "=== true) {"))
        );
    }

    #[test]
    fn take_until_range_unicode_1() {
        let result = take_until_range("🦀")
            .parse("😃 Ferris the friendly rustacean 🦀 and his snake friend 🐍");
        assert_eq!(
            result,
            Ok((
                "😃 Ferris the friendly rustacean ",
                "🦀 and his snake friend 🐍"
            ))
        );
    }

    #[test]
    fn take_until_range_unicode_2() {
        let result = take_until_range("⁘⁙/⁘").parse("⚙️🛠️🦀=🏎️⁘⁙⁘⁘⁙/⁘⁘⁙/⁘");
        assert_eq!(result, Ok(("⚙️🛠️🦀=🏎️⁘⁙⁘", "⁘⁙/⁘⁘⁙/⁘")));
    }
}
