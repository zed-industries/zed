//! A collection of both concrete parsers as well as parser combinators.
//!
//! Implements the [`Parser`] trait which is the core of `combine` and contains the submodules
//! implementing all combine parsers.

use crate::{
    error::{
        ErrorInfo, ParseError,
        ParseResult::{self, *},
        ResultExt, StreamError, Token, Tracked,
    },
    parser::{
        combinator::{
            and_then, flat_map, map, map_input, spanned, AndThen, Either, FlatMap, Map, MapInput,
            Spanned,
        },
        error::{expected, message, silent, Expected, Message, Silent},
        repeat::Iter,
        sequence::{then, then_partial, then_ref, Then, ThenPartial, ThenRef},
    },
    stream::{Stream, StreamErrorFor, StreamOnce},
    ErrorOffset,
};

use self::{
    choice::{or, Or},
    sequence::{skip, with, Skip, With},
};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;

/// Internal API. May break without a semver bump
#[macro_export]
#[doc(hidden)]
macro_rules! parse_mode {
    ($input_type: ty) => {
        #[inline]
        fn parse_partial(
            &mut self,
            input: &mut $input_type,
            state: &mut Self::PartialState,
        ) -> $crate::error::ParseResult<Self::Output, <$input_type as $crate::StreamOnce>::Error> {
            self.parse_mode($crate::parser::PartialMode::default(), input, state)
        }

        #[inline]
        fn parse_first(
            &mut self,
            input: &mut $input_type,
            state: &mut Self::PartialState,
        ) -> $crate::error::ParseResult<Self::Output, <$input_type as $crate::StreamOnce>::Error> {
            self.parse_mode($crate::parser::FirstMode, input, state)
        }
    };
}

pub mod byte;
pub mod char;
pub mod choice;
pub mod combinator;
pub mod error;
pub mod function;
pub mod range;
#[cfg(feature = "regex")]
#[cfg_attr(docsrs, doc(cfg(feature = "regex")))]
pub mod regex;
pub mod repeat;
pub mod sequence;
pub mod token;

/// By implementing the `Parser` trait a type says that it can be used to parse an input stream
/// into the type `Output`.
///
/// All methods have a default implementation but there needs to be at least an implementation of
/// [`parse_stream`], [`parse_stream`], or [`parse_lazy`]. If the last is implemented, an
/// implementation of [`add_error`] may also be required. See the documentation for
/// [`parse_lazy`] for details.
///
/// [`parse_stream`]: trait.Parser.html#method.parse_stream
/// [`parse_stream`]: trait.Parser.html#method.parse_stream
/// [`parse_lazy`]: trait.Parser.html#method.parse_lazy
/// [`add_error`]: trait.Parser.html#method.add_error
pub trait Parser<Input: Stream> {
    /// The type which is returned if the parser is successful.
    type Output;

    /// Determines the state necessary to resume parsing after more input is supplied.
    ///
    /// If partial parsing is not supported this can be set to `()`.
    type PartialState: Default;

    /// Entry point of the parser. Takes some input and tries to parse it.
    ///
    /// Returns the parsed result and the remaining input if the parser succeeds, or a
    /// error otherwise.
    ///
    /// This is the most straightforward entry point to a parser. Since it does not decorate the
    /// input in any way you may find the error messages a hard to read. If that is the case you
    /// may want to try wrapping your input with an [`easy::Stream`] or call [`easy_parse`]
    /// instead.
    ///
    /// [`easy::Stream`]: super::easy::Stream
    /// [`easy_parse`]: super::parser::EasyParser::easy_parse
    fn parse(
        &mut self,
        mut input: Input,
    ) -> Result<(Self::Output, Input), <Input as StreamOnce>::Error> {
        match self.parse_stream(&mut input).into() {
            Ok((v, _)) => Ok((v, input)),
            Err(error) => Err(error.into_inner().error),
        }
    }

    /// Entry point of the parser when using partial parsing.
    /// Takes some input and tries to parse it.
    ///
    /// Returns the parsed result and the remaining input if the parser succeeds, or a
    /// error otherwise.
    fn parse_with_state(
        &mut self,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> Result<Self::Output, <Input as StreamOnce>::Error> {
        match self.parse_stream_partial(input, state).into() {
            Ok((v, _)) => Ok(v),
            Err(error) => Err(error.into_inner().error),
        }
    }

    /// Parses using the stream `input` by calling [`Stream::uncons`] one or more times.
    ///
    /// Semantically equivalent to [`parse_stream`], except this method returns a flattened result
    /// type, combining `Result` and [`Commit`] into a single [`ParseResult`].
    ///
    /// [`Stream::uncons`]: super::stream::StreamOnce::uncons
    /// [`parse_stream`]: Parser::parse_stream
    /// [`Commit`]: super::error::Commit
    /// [`ParseResult`]: super::error::ParseResult
    #[inline]
    fn parse_stream(
        &mut self,
        input: &mut Input,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error> {
        let before = input.checkpoint();
        let mut state = Default::default();
        let mut result = self.parse_first(input, &mut state);
        if let ParseResult::PeekErr(ref mut error) = result {
            ctry!(input.reset(before.clone()).committed());
            if let Ok(t) = input.uncons() {
                ctry!(input.reset(before).committed());
                error.error.add_unexpected(Token(t));
            } else {
                error.error.add(StreamErrorFor::<Input>::end_of_input());
            }
            self.add_error(error);
        }
        result
    }

    /// Parses using the stream `input` by calling [`Stream::uncons`] one or more times.
    ///
    /// Specialized version of [`parse_stream`] which permits error value creation to be
    /// skipped in the common case.
    ///
    /// When this parser returns `PeekErr`, this method is allowed to return an empty
    /// [`Error`]. The error value that would have been returned can instead be obtained by
    /// calling [`add_error`]. This allows a parent parser such as `choice` to skip the creation of
    /// an unnecessary error value, if an alternative parser succeeds.
    ///
    /// Parsers should seek to implement this function instead of the above two if errors can be
    /// encountered before consuming input. The default implementation always returns all errors,
    /// with [`add_error`] being a no-op.
    ///
    /// [`Stream::uncons`]: super::stream::StreamOnce::uncons
    /// [`parse_stream`]: Parser::parse_stream
    /// [`Error`]: super::stream::StreamOnce::Error
    /// [`add_error`]: trait.Parser.html#method.add_error
    #[inline]
    fn parse_lazy(
        &mut self,
        input: &mut Input,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error> {
        if input.is_partial() {
            // If a partial parser were called from a non-partial parser (as it is here) we must
            // reset the input to before the partial parser were called on errors that committed
            // data as that parser's partial state was just temporary and it will not be able to
            // resume itself
            let before = input.checkpoint();
            let result = self.parse_first(input, &mut Default::default());
            if let CommitErr(_) = result {
                ctry!(input.reset(before).committed());
            }
            result
        } else {
            self.parse_first(input, &mut Default::default())
        }
    }

    /// Adds the first error that would normally be returned by this parser if it failed with an
    /// `PeekErr` result.
    ///
    /// See [`parse_lazy`] for details.
    ///
    /// [`parse_lazy`]: trait.Parser.html#method.parse_lazy
    fn add_error(&mut self, _error: &mut Tracked<<Input as StreamOnce>::Error>) {}

    /// Like `parse_stream` but supports partial parsing.
    #[inline]
    fn parse_stream_partial(
        &mut self,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error> {
        let before = input.checkpoint();
        let mut result = self.parse_partial(input, state);
        if let ParseResult::PeekErr(ref mut error) = result {
            ctry!(input.reset(before.clone()).committed());
            if let Ok(t) = input.uncons() {
                ctry!(input.reset(before).committed());
                error.error.add_unexpected(Token(t));
            } else {
                error.error.add(StreamErrorFor::<Input>::end_of_input());
            }
            self.add_error(error);
        }
        result
    }

    /// Parses using the stream `input` and allows itself to be resumed at a later point using
    /// `parse_partial` by storing the necessary intermediate state in `state`.
    ///
    /// Unlike `parse_partial` function this is allowed to assume that there is no partial state to
    /// resume.
    ///
    /// Internal API. May break without a semver bump
    /// Always overridden by the `parse_mode!` macro
    #[inline]
    #[doc(hidden)]
    fn parse_first(
        &mut self,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error> {
        self.parse_partial(input, state)
    }

    /// Parses using the stream `input` and allows itself to be resumed at a later point using
    /// `parse_partial` by storing the necessary intermediate state in `state`
    ///
    /// Internal API. May break without a semver bump
    /// Always overridden by the `parse_mode!` macro
    #[inline]
    #[doc(hidden)]
    fn parse_partial(
        &mut self,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error> {
        let _ = state;
        self.parse_lazy(input)
    }

    /// Internal API. May break without a semver bump
    #[doc(hidden)]
    #[inline]
    fn parse_mode<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>
    where
        M: ParseMode,
        Self: Sized,
    {
        mode.parse(self, input, state)
    }

    /// Internal API. May break without a semver bump
    #[doc(hidden)]
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>
    where
        M: ParseMode,
        Self: Sized,
    {
        if mode.is_first() {
            self.parse_first(input, state)
        } else {
            self.parse_partial(input, state)
        }
    }

    /// Internal API. May break without a semver bump
    #[doc(hidden)]
    #[inline]
    fn parse_committed_mode<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error>
    where
        M: ParseMode,
        Self: Sized,
    {
        if mode.is_first() {
            FirstMode.parse_committed(self, input, state)
        } else {
            PartialMode::default().parse_committed(self, input, state)
        }
    }

    /// Returns how many parsers this parser contains
    ///
    /// Internal API: This should not be implemented explicitly outside of combine.
    #[doc(hidden)]
    fn parser_count(&self) -> ErrorOffset {
        ErrorOffset(1)
    }

    /// Internal API: This should not be implemented explicitly outside of combine.
    #[doc(hidden)]
    fn add_committed_expected_error(&mut self, _error: &mut Tracked<<Input as StreamOnce>::Error>) {
    }

    /// Borrows a parser instead of consuming it.
    ///
    /// Used to apply parser combinators on `self` without losing ownership.
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::error::Commit;
    /// # use combine::parser::char::{digit, letter};
    /// fn test(input: &mut &'static str) -> StdParseResult<(char, char), &'static str> {
    ///     let mut p = digit();
    ///     let ((d, _), committed) = (p.by_ref(), letter()).parse_stream(input).into_result()?;
    ///     let (d2, committed) = committed.combine(|_| p.parse_stream(input).into_result())?;
    ///     Ok(((d, d2), committed))
    /// }
    ///
    /// fn main() {
    ///     let mut input = "1a23";
    ///     assert_eq!(
    ///         test(&mut input).map(|(t, c)| (t, c.map(|_| input))),
    ///         Ok((('1', '2'), Commit::Commit("3")))
    ///     );
    /// }
    /// ```
    fn by_ref(&mut self) -> &mut Self
    where
        Self: Sized,
    {
        self
    }

    /// Discards the value of the `self` parser and returns the value of `p`.
    /// Fails if any of the parsers fails.
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::parser::char::digit;
    /// # fn main() {
    /// let result = digit()
    ///     .with(token('i'))
    ///     .parse("9i")
    ///     .map(|x| x.0);
    /// assert_eq!(result, Ok('i'));
    /// # }
    /// ```
    fn with<P2>(self, p: P2) -> With<Self, P2>
    where
        Self: Sized,
        P2: Parser<Input>,
    {
        with(self, p)
    }

    /// Discards the value of the `p` parser and returns the value of `self`.
    /// Fails if any of the parsers fails.
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::parser::char::digit;
    /// # fn main() {
    /// let result = digit()
    ///     .skip(token('i'))
    ///     .parse("9i")
    ///     .map(|x| x.0);
    /// assert_eq!(result, Ok('9'));
    /// # }
    /// ```
    fn skip<P2>(self, p: P2) -> Skip<Self, P2>
    where
        Self: Sized,
        P2: Parser<Input>,
    {
        skip(self, p)
    }

    /// Parses with `self` followed by `p`.
    /// Succeeds if both parsers succeed, otherwise fails.
    /// Returns a tuple with both values on success.
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::parser::char::digit;
    /// # fn main() {
    /// let result = digit()
    ///     .and(token('i'))
    ///     .parse("9i")
    ///     .map(|x| x.0);
    /// assert_eq!(result, Ok(('9', 'i')));
    /// # }
    /// ```
    fn and<P2>(self, p: P2) -> (Self, P2)
    where
        Self: Sized,
        P2: Parser<Input>,
    {
        (self, p)
    }

    /// Returns a parser which attempts to parse using `self`. If `self` fails without committing
    /// it tries to consume the same input using `p`.
    ///
    /// If you are looking to chain 3 or more parsers using `or` you may consider using the
    /// [`choice!`] macro instead, which can be clearer and may result in a faster parser.
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::parser::char::{digit, string};
    /// # fn main() {
    /// let mut parser = string("let")
    ///     .or(digit().map(|_| "digit"))
    ///     .or(string("led"));
    /// assert_eq!(parser.parse("let"), Ok(("let", "")));
    /// assert_eq!(parser.parse("1"), Ok(("digit", "")));
    /// assert!(parser.parse("led").is_err());
    ///
    /// let mut parser2 = string("two").or(string("three"));
    /// // Fails as the parser for "two" consumes the first 't' before failing
    /// assert!(parser2.parse("three").is_err());
    ///
    /// // Use 'attempt' to make failing parsers always act as if they have not committed any input
    /// let mut parser3 = attempt(string("two")).or(attempt(string("three")));
    /// assert_eq!(parser3.parse("three"), Ok(("three", "")));
    /// # }
    /// ```
    ///
    /// [`choice!`]: super::choice!
    fn or<P2>(self, p: P2) -> Or<Self, P2>
    where
        Self: Sized,
        P2: Parser<Input, Output = Self::Output>,
    {
        or(self, p)
    }

    /// Parses using `self` and then passes the value to `f` which returns a parser used to parse
    /// the rest of the input.
    ///
    /// Since the parser returned from `f` must have a single type it can be useful to use the
    /// [`left`](Parser::left) and [`right`](Parser::right) methods to merge parsers of differing types into one.
    ///
    /// If you are using partial parsing you may want to use [`then_partial`](Parser::then_partial) instead.
    ///
    /// ```
    /// # #![cfg(feature = "std")]
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::parser::char::digit;
    /// # use combine::error::Commit;
    /// # use combine::stream::easy;
    /// # fn main() {
    /// let result = digit()
    ///     .then(|d| {
    ///         if d == '9' {
    ///             value(9).left()
    ///         }
    ///         else {
    ///             unexpected_any(d).message("Not a nine").right()
    ///         }
    ///     })
    ///     .easy_parse("9");
    /// assert_eq!(result, Ok((9, "")));
    /// # }
    /// ```
    fn then<N, F>(self, f: F) -> Then<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> N,
        N: Parser<Input>,
    {
        then(self, f)
    }

    /// Variant of [`then`](Parser::then) which parses using `self` and then passes the value to `f` as a `&mut` reference.
    ///
    /// Useful when doing partial parsing since it does not need to store the parser returned by
    /// `f` in the partial state. Instead it will call `f` each to request a new parser each time
    /// parsing resumes and that parser is needed.
    ///
    /// Since the parser returned from `f` must have a single type it can be useful to use the
    /// [`left`](Parser::left) and [`right`](Parser::right) methods to merge parsers of differing types into one.
    ///
    /// ```
    /// # #![cfg(feature = "std")]
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::parser::char::digit;
    /// # use combine::error::Commit;
    /// # use combine::stream::easy;
    /// # fn main() {
    /// let result = digit()
    ///     .then_partial(|d| {
    ///         if *d == '9' {
    ///             value(9).left()
    ///         }
    ///         else {
    ///             unexpected_any(*d).message("Not a nine").right()
    ///         }
    ///     })
    ///     .easy_parse("9");
    /// assert_eq!(result, Ok((9, "")));
    /// # }
    /// ```
    fn then_partial<N, F>(self, f: F) -> ThenPartial<Self, F>
    where
        Self: Sized,
        F: FnMut(&mut Self::Output) -> N,
        N: Parser<Input>,
    {
        then_partial(self, f)
    }

    /// Parses using `self` and then passes a reference to the value to `f` which returns a parser
    /// used to parse the rest of the input. The value is then combined with the output of `f`.
    ///
    /// Since the parser returned from `f` must have a single type it can be useful to use the
    /// `left` and `right` methods to merge parsers of differing types into one.
    ///
    /// ```
    /// # #![cfg(feature = "std")]
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::parser::char::digit;
    /// # use combine::error::Commit;
    /// # use combine::stream::easy;
    /// # fn main() {
    /// let result = digit()
    ///     .then_ref(|d| {
    ///         if *d == '9' {
    ///             digit().left()
    ///         }
    ///         else {
    ///             unexpected_any(*d).message("Not a nine").right()
    ///         }
    ///     })
    ///     .easy_parse("98");
    /// assert_eq!(result, Ok((('9', '8'), "")));
    /// # }
    /// ```
    fn then_ref<N, F>(self, f: F) -> ThenRef<Self, F>
    where
        Self: Sized,
        F: FnMut(&Self::Output) -> N,
        N: Parser<Input>,
    {
        then_ref(self, f)
    }

    /// Uses `f` to map over the parsed value.
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::parser::char::digit;
    /// # fn main() {
    /// let result = digit()
    ///     .map(|c| c == '9')
    ///     .parse("9")
    ///     .map(|x| x.0);
    /// assert_eq!(result, Ok(true));
    /// # }
    /// ```
    fn map<F, B>(self, f: F) -> Map<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> B,
    {
        map(self, f)
    }

    fn map_input<F, B>(self, f: F) -> MapInput<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output, &mut Input) -> B,
    {
        map_input(self, f)
    }

    /// Uses `f` to map over the output of `self`. If `f` returns an error the parser fails.
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::parser::char::digit;
    /// # use combine::parser::range::take;
    /// # fn main() {
    /// let result = take(4)
    ///     .flat_map(|bs| many(digit()).parse(bs).map(|t| t.0))
    ///     .parse("12abcd");
    /// assert_eq!(result, Ok((String::from("12"), "cd")));
    /// # }
    /// ```
    fn flat_map<F, B>(self, f: F) -> FlatMap<Self, F>
    where
        Self: Sized,
        F: FnMut(Self::Output) -> Result<B, <Input as StreamOnce>::Error>,
    {
        flat_map(self, f)
    }

    /// Parses with `self` and if it fails, adds the message `msg` to the error.
    ///
    /// ```
    /// # #![cfg(feature = "std")]
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::stream::easy;
    /// # use combine::stream::position::{self, SourcePosition};
    /// # fn main() {
    /// let result = token('9')
    ///     .message("Not a nine")
    ///     .easy_parse(position::Stream::new("8"));
    /// assert_eq!(result, Err(easy::Errors {
    ///     position: SourcePosition::default(),
    ///     errors: vec![
    ///         easy::Error::Unexpected('8'.into()),
    ///         easy::Error::Expected('9'.into()),
    ///         easy::Error::Message("Not a nine".into())
    ///     ]
    /// }));
    /// # }
    /// ```
    fn message<S>(self, msg: S) -> Message<Self, S>
    where
        Self: Sized,
        S: for<'s> ErrorInfo<'s, Input::Token, Input::Range>,
    {
        message(self, msg)
    }

    /// Parses with `self` and if it fails without consuming any input any expected errors are
    /// replaced by `msg`. `msg` is then used in error messages as "Expected `msg`".
    ///
    /// ```
    /// # #![cfg(feature = "std")]
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::error;
    /// # use combine::stream::easy;
    /// # use combine::stream::position::{self, SourcePosition};
    /// # fn main() {
    /// let result = token('9')
    ///     .expected("nine")
    ///     .easy_parse(position::Stream::new("8"));
    /// assert_eq!(result, Err(easy::Errors {
    ///     position: SourcePosition::default(),
    ///     errors: vec![
    ///         easy::Error::Unexpected('8'.into()),
    ///         easy::Error::Expected("nine".into())
    ///     ]
    /// }));
    ///
    /// let result = token('9')
    ///     .expected(error::Format(format_args!("That is not a nine!")))
    ///     .easy_parse(position::Stream::new("8"));
    /// assert_eq!(result, Err(easy::Errors {
    ///     position: SourcePosition::default(),
    ///     errors: vec![
    ///         easy::Error::Unexpected('8'.into()),
    ///         easy::Error::Expected("That is not a nine!".to_string().into())
    ///     ]
    /// }));
    /// # }
    /// ```
    fn expected<S>(self, msg: S) -> Expected<Self, S>
    where
        Self: Sized,
        S: for<'s> ErrorInfo<'s, Input::Token, Input::Range>,
    {
        expected(self, msg)
    }

    /// Parses with `self`, if it fails without consuming any input any expected errors that would
    /// otherwise be emitted by `self` are suppressed.
    ///
    /// ```
    /// # #![cfg(feature = "std")]
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::stream::easy;
    /// # use combine::stream::position::{self, SourcePosition};
    /// # fn main() {
    /// let result = token('9')
    ///     .expected("nine")
    ///     .silent()
    ///     .easy_parse(position::Stream::new("8"));
    /// assert_eq!(result, Err(easy::Errors {
    ///     position: SourcePosition::default(),
    ///     errors: vec![
    ///         easy::Error::Unexpected('8'.into()),
    ///     ]
    /// }));
    /// # }
    /// ```
    fn silent(self) -> Silent<Self>
    where
        Self: Sized,
    {
        silent(self)
    }

    /// Parses with `self` and applies `f` on the result if `self` parses successfully.
    /// `f` may optionally fail with an error which is automatically converted to a `ParseError`.
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::stream::position::{self, SourcePosition};
    /// # use combine::parser::char::digit;
    /// # fn main() {
    /// let mut parser = many1(digit())
    ///     .and_then(|s: String| s.parse::<i32>());
    /// let result = parser.easy_parse(position::Stream::new("1234")).map(|(x, state)| (x, state.input));
    /// assert_eq!(result, Ok((1234, "")));
    /// let result = parser.easy_parse(position::Stream::new("999999999999999999999999"));
    /// assert!(result.is_err());
    /// // Errors are report as if they occurred at the start of the parse
    /// assert_eq!(result.unwrap_err().position, SourcePosition { line: 1, column: 1 });
    /// # }
    /// ```
    fn and_then<F, O, E>(self, f: F) -> AndThen<Self, F>
    where
        Self: Parser<Input> + Sized,
        F: FnMut(Self::Output) -> Result<O, E>,
        E: Into<
            <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError,
        >,
    {
        and_then(self, f)
    }

    /// Creates an iterator from a parser and a state. Can be used as an alternative to [`many`]
    /// when collecting directly into a `Extend` type is not desirable.
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::parser::char::{char, digit};
    /// # fn main() {
    /// let mut buffer = String::new();
    /// let number = parser(|input| {
    ///     buffer.clear();
    ///     let mut iter = digit().iter(input);
    ///     buffer.extend(&mut iter);
    ///     let i = buffer.parse::<i32>().unwrap();
    ///     iter.into_result(i)
    /// });
    /// let result = sep_by(number, char(','))
    ///     .parse("123,45,6");
    /// assert_eq!(result, Ok((vec![123, 45, 6], "")));
    /// # }
    /// ```
    ///
    /// [`many`]: repeat::many
    fn iter(self, input: &mut Input) -> Iter<'_, Input, Self, Self::PartialState, FirstMode>
    where
        Self: Parser<Input> + Sized,
    {
        Iter::new(self, FirstMode, input, Default::default())
    }

    /// Creates an iterator from a parser and a state. Can be used as an alternative to [`many`]
    /// when collecting directly into a `Extend` type is not desirable.
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::parser::char::{char, digit};
    /// # fn main() {
    /// let mut buffer = String::new();
    /// let number = parser(|input| {
    ///     buffer.clear();
    ///     let mut iter = digit().iter(input);
    ///     buffer.extend(&mut iter);
    ///     let i = buffer.parse::<i32>().unwrap();
    ///     iter.into_result(i)
    /// });
    /// let result = sep_by(number, char(','))
    ///     .parse("123,45,6");
    /// assert_eq!(result, Ok((vec![123, 45, 6], "")));
    /// # }
    /// ```
    ///
    /// [`many`]: repeat::many
    fn partial_iter<'a, 's, M>(
        self,
        mode: M,
        input: &'a mut Input,
        partial_state: &'s mut Self::PartialState,
    ) -> Iter<'a, Input, Self, &'s mut Self::PartialState, M>
    where
        Self: Parser<Input> + Sized,
        M: ParseMode,
    {
        Iter::new(self, mode, input, partial_state)
    }

    /// Turns the parser into a trait object by putting it in a `Box`. Can be used to easily
    /// return parsers from functions without naming the type.
    ///
    /// ```
    /// # use combine::*;
    /// # fn main() {
    /// fn test<'input, F>(
    ///     c: char,
    ///     f: F)
    ///     -> Box<dyn Parser<&'input str, Output = (char, char), PartialState = ()> + 'input>
    ///     where F: FnMut(char) -> bool + 'static
    /// {
    ///     combine::parser::combinator::no_partial((token(c), satisfy(f))).boxed()
    /// }
    /// let result = test('a', |c| c >= 'a' && c <= 'f')
    ///     .parse("ac");
    /// assert_eq!(result, Ok((('a', 'c'), "")));
    /// # }
    /// ```
    #[cfg(feature = "alloc")]
    #[cfg_attr(docsrs, doc(cfg(feature = "std")))]
    fn boxed<'a>(
        self,
    ) -> Box<dyn Parser<Input, Output = Self::Output, PartialState = Self::PartialState> + 'a>
    where
        Self: Sized + 'a,
    {
        Box::new(self)
    }

    /// Wraps the parser into the [`Either`](combinator::Either) enum which allows combinators such as [`then`](Parser::then) to return
    /// multiple different parser types (merging them to one)
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::parser::char::{digit, letter};
    /// # fn main() {
    /// let mut parser = any().then(|c|
    ///     if c == '#' {
    ///         skip_many(satisfy(|c| c != '\n'))
    ///             .with(value("".to_string()))
    ///             .left()
    ///     } else {
    ///         many1(letter())
    ///             .map(move |mut s: String| { s.insert(0, c); s })
    ///             .right()
    ///     });
    ///
    /// let result = parser.parse("ac2");
    /// assert_eq!(result, Ok(("ac".to_string(), "2")));
    ///
    /// let result = parser.parse("# ac2");
    /// assert_eq!(result, Ok(("".to_string(), "")));
    /// # }
    /// ```
    fn left<R>(self) -> Either<Self, R>
    where
        Self: Sized,
        R: Parser<Input, Output = Self::Output>,
    {
        Either::Left(self)
    }

    /// Wraps the parser into the [`Either`](combinator::Either) enum which allows combinators such as [`then`](Parser::then) to return
    /// multiple different parser types (merging them to one)
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::parser::char::{digit, letter};
    /// # fn main() {
    /// let mut parser = any().then(|c|
    ///     if c == '#' {
    ///         skip_many(satisfy(|c| c != '\n'))
    ///             .with(value("".to_string()))
    ///             .left()
    ///     } else {
    ///         many1(letter())
    ///             .map(move |mut s: String| { s.insert(0, c); s })
    ///             .right()
    ///     });
    ///
    /// let result = parser.parse("ac2");
    /// assert_eq!(result, Ok(("ac".to_string(), "2")));
    ///
    /// let result = parser.parse("# ac2");
    /// assert_eq!(result, Ok(("".to_string(), "")));
    /// # }
    /// ```
    fn right<L>(self) -> Either<L, Self>
    where
        Self: Sized,
        L: Parser<Input, Output = Self::Output>,
    {
        Either::Right(self)
    }

    /// Marks errors produced inside the `self` parser with the span from the start of the parse to
    /// the end of it.
    ///
    /// [`p.spanned()`]: ../trait.Parser.html#method.spanned
    ///
    /// ```
    /// use combine::{*, parser::{char::string, combinator::spanned}};
    /// use combine::stream::{easy, span};
    ///
    /// let input = "hel";
    /// let result = spanned(string("hello")).parse(
    ///     span::Stream::<_, easy::Errors<_, _, span::Span<_>>>::from(easy::Stream::from(input)),
    /// );
    /// assert!(result.is_err());
    /// assert_eq!(
    ///     result.unwrap_err().position.map(|p| p.translate_position(input)),
    ///     span::Span { start: 0, end: 3 },
    /// );
    /// ```
    fn spanned(self) -> Spanned<Self>
    where
        Self: Sized,
    {
        spanned(self)
    }
}

/// Provides the `easy_parse` method which provides good error messages by default
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub trait EasyParser<Input: Stream>: Parser<crate::easy::Stream<Input>>
where
    Input::Token: PartialEq,
    Input::Range: PartialEq,
{
    /// Entry point of the parser. Takes some input and tries to parse it, returning an easy to use
    /// and format error if parsing did not succeed.
    ///
    /// Returns the parsed result and the remaining input if the parser succeeds, or a
    /// This function wraps requires `Input == easy::Stream<Input>` which makes it return
    /// return `easy::Errors` if an error occurs. Due to this wrapping it is recommended that the
    /// parser `Self` is written with a generic input type.
    ///
    /// ```
    /// # #[macro_use]
    /// # extern crate combine;
    ///
    /// use combine::*;
    /// use combine::parser::repeat::many1;
    /// use combine::parser::char::letter;
    ///
    /// // Good!
    /// parser!{
    /// fn my_parser[Input]()(Input) -> String
    ///     where [Input: Stream<Token = char>]
    /// {
    ///     many1::<String, _, _>(letter())
    /// }
    /// }
    ///
    /// // Won't compile with `easy_parse` since it is specialized on `&str`
    /// parser!{
    /// fn my_parser2['a]()(&'a str) -> String
    ///     where [&'a str: Stream<Token = char, Range = &'a str>]
    /// {
    ///     many1(letter())
    /// }
    /// }
    ///
    /// fn main() {
    ///     assert_eq!(my_parser().parse("abc"), Ok(("abc".to_string(), "")));
    ///     // Would fail to compile if uncommented
    ///     // my_parser2().parse("abc")
    /// }
    /// ```
    ///
    /// [`ParseError`]: struct.ParseError.html
    fn easy_parse(
        &mut self,
        input: Input,
    ) -> Result<
        (<Self as Parser<crate::easy::Stream<Input>>>::Output, Input),
        crate::easy::ParseError<Input>,
    >
    where
        Input: Stream,
        crate::easy::Stream<Input>: StreamOnce<
            Token = Input::Token,
            Range = Input::Range,
            Error = crate::easy::ParseError<crate::easy::Stream<Input>>,
            Position = Input::Position,
        >,
        Input::Position: Default,
        Self: Sized + Parser<crate::easy::Stream<Input>>,
    {
        let input = crate::easy::Stream(input);
        self.parse(input).map(|(v, input)| (v, input.0))
    }
}

#[cfg(feature = "std")]
impl<Input, P> EasyParser<Input> for P
where
    P: ?Sized + Parser<crate::easy::Stream<Input>>,
    Input: Stream,
    Input::Token: PartialEq,
    Input::Range: PartialEq,
{
}

macro_rules! forward_deref {
    (Input) => {
        type Output = P::Output;
        type PartialState = P::PartialState;

        #[inline]
        fn parse_first(
            &mut self,
            input: &mut Input,
            state: &mut Self::PartialState,
        ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error> {
            (**self).parse_first(input, state)
        }

        #[inline]
        fn parse_partial(
            &mut self,
            input: &mut Input,
            state: &mut Self::PartialState,
        ) -> ParseResult<Self::Output, <Input as StreamOnce>::Error> {
            (**self).parse_partial(input, state)
        }

        #[inline]
        fn add_error(&mut self, error: &mut Tracked<<Input as StreamOnce>::Error>) {
            (**self).add_error(error)
        }

        #[inline]
        fn add_committed_expected_error(
            &mut self,
            error: &mut Tracked<<Input as StreamOnce>::Error>,
        ) {
            (**self).add_committed_expected_error(error)
        }

        #[inline]
        fn parser_count(&self) -> ErrorOffset {
            (**self).parser_count()
        }
    };
}

impl<'a, P, Input> Parser<Input> for &'a mut P
where
    P: ?Sized + Parser<Input>,
    Input: Stream,
{
    forward_deref!(Input);
}

#[cfg(feature = "alloc")]
impl<P, Input> Parser<Input> for Box<P>
where
    P: ?Sized + Parser<Input>,
    Input: Stream,
{
    forward_deref!(Input);
}

/// Internal API. May break without a semver bump
#[doc(hidden)]
/// Specifies whether the parser must check for partial state that must be resumed
pub trait ParseMode: Copy {
    /// If `true` then the parser has no previous state to resume otherwise the parser *might* have
    /// state to resume which it must check.
    fn is_first(self) -> bool;
    /// Puts the mode into `first` parsing.
    fn set_first(&mut self);

    fn parse<P, Input>(
        self,
        parser: &mut P,
        input: &mut Input,
        state: &mut P::PartialState,
    ) -> ParseResult<P::Output, Input::Error>
    where
        P: Parser<Input>,
        Input: Stream;

    #[inline]
    fn parse_committed<P, Input>(
        self,
        parser: &mut P,
        input: &mut Input,
        state: &mut P::PartialState,
    ) -> ParseResult<P::Output, <Input as StreamOnce>::Error>
    where
        P: Parser<Input>,
        Input: Stream,
    {
        let before = input.checkpoint();
        let mut result = parser.parse_mode_impl(self, input, state);
        if let ParseResult::PeekErr(ref mut error) = result {
            ctry!(input.reset(before.clone()).committed());
            if let Ok(t) = input.uncons() {
                ctry!(input.reset(before).committed());
                error.error.add_unexpected(Token(t));
            } else {
                error.error.add(StreamErrorFor::<Input>::end_of_input());
            }
            parser.add_error(error);
        }
        result
    }
}

/// Internal API. May break without a semver bump
#[doc(hidden)]
#[derive(Copy, Clone)]
pub struct FirstMode;
impl ParseMode for FirstMode {
    #[inline]
    fn is_first(self) -> bool {
        true
    }
    #[inline]
    fn set_first(&mut self) {}

    fn parse<P, Input>(
        self,
        parser: &mut P,
        input: &mut Input,
        state: &mut P::PartialState,
    ) -> ParseResult<P::Output, Input::Error>
    where
        P: Parser<Input>,
        Input: Stream,
    {
        parser.parse_mode_impl(FirstMode, input, state)
    }
}

/// Internal API. May break without a semver bump
#[doc(hidden)]
#[derive(Copy, Clone, Default)]
pub struct PartialMode {
    pub first: bool,
}
impl ParseMode for PartialMode {
    #[inline]
    fn is_first(self) -> bool {
        self.first
    }

    #[inline]
    fn set_first(&mut self) {
        self.first = true;
    }

    fn parse<P, Input>(
        self,
        parser: &mut P,
        input: &mut Input,
        state: &mut P::PartialState,
    ) -> ParseResult<P::Output, Input::Error>
    where
        P: Parser<Input>,
        Input: Stream,
    {
        if self.is_first() {
            parser.parse_mode_impl(FirstMode, input, state)
        } else {
            parser.parse_mode_impl(self, input, state)
        }
    }
}
