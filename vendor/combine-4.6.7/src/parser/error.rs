//! Parsers which cause errors or modifies the returned error on parse failure.

use crate::{
    error::{
        ErrorInfo, ParseError,
        ParseResult::{self, *},
        StreamError, Tracked,
    },
    lib::marker::PhantomData,
    parser::ParseMode,
    Parser, Stream, StreamOnce,
};

#[derive(Clone)]
pub struct Unexpected<I, T, E>(E, PhantomData<fn(I) -> (I, T)>)
where
    I: Stream;
impl<Input, T, E> Parser<Input> for Unexpected<Input, T, E>
where
    Input: Stream,
    E: for<'s> ErrorInfo<'s, Input::Token, Input::Range>,
{
    type Output = T;
    type PartialState = ();
    #[inline]
    fn parse_lazy(&mut self, input: &mut Input) -> ParseResult<T, <Input as StreamOnce>::Error> {
        PeekErr(<Input as StreamOnce>::Error::empty(input.position()).into())
    }
    fn add_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        errors.error.add(StreamError::unexpected(&self.0));
    }
}
/// Always fails with `message` as an unexpected error.
/// Never consumes any input.
///
/// Has `()` the output type
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::error::StreamError;
/// # fn main() {
/// let result = unexpected("token")
///     .easy_parse("a");
/// assert!(result.is_err());
/// assert!(
///     result.err()
///         .unwrap()
///         .errors
///         .iter()
///         .any(|m| *m == StreamError::unexpected("token"))
/// );
/// # }
/// ```
pub fn unexpected<Input, S>(message: S) -> Unexpected<Input, (), S>
where
    Input: Stream,
    S: for<'s> ErrorInfo<'s, Input::Token, Input::Range>,
{
    unexpected_any(message)
}

/// Always fails with `message` as an unexpected error.
/// Never consumes any input.
///
/// May have anything as the output type but must be used such that the output type can inferred.
/// The `unexpected` parser can be used if the output type does not matter
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::error::unexpected_any;
/// # use combine::error::StreamError;
/// # fn main() {
/// let result = token('b').or(unexpected_any("token"))
///     .easy_parse("a");
/// assert!(result.is_err());
/// assert!(
///     result.err()
///         .unwrap()
///         .errors
///         .iter()
///         .any(|m| *m == StreamError::unexpected("token"))
/// );
/// # }
/// ```
pub fn unexpected_any<Input, S, T>(message: S) -> Unexpected<Input, T, S>
where
    Input: Stream,
    S: for<'s> ErrorInfo<'s, Input::Token, Input::Range>,
{
    Unexpected(message, PhantomData)
}

#[derive(Clone)]
pub struct Message<P, S>(P, S);
impl<Input, P, S> Parser<Input> for Message<P, S>
where
    Input: Stream,
    P: Parser<Input>,
    S: for<'s> ErrorInfo<'s, Input::Token, Input::Range>,
{
    type Output = P::Output;
    type PartialState = P::PartialState;

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
        match self.0.parse_mode(mode, input, state) {
            CommitOk(x) => CommitOk(x),
            PeekOk(x) => PeekOk(x),

            // The message should always be added even if some input was committed before failing
            CommitErr(mut err) => {
                err.add_message(&self.1);
                CommitErr(err)
            }

            // The message will be added in `add_error`
            PeekErr(err) => PeekErr(err),
        }
    }

    fn add_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        self.0.add_error(errors);
        errors.error.add_message(&self.1);
    }

    forward_parser!(Input, parser_count add_committed_expected_error, 0);
}

/// Equivalent to [`p1.message(msg)`].
///
/// [`p1.message(msg)`]: ../trait.Parser.html#method.message
pub fn message<Input, P, S>(p: P, msg: S) -> Message<P, S>
where
    P: Parser<Input>,
    Input: Stream,
    S: for<'s> ErrorInfo<'s, Input::Token, Input::Range>,
{
    Message(p, msg)
}

#[derive(Clone)]
pub struct Expected<P, S>(P, S);
impl<Input, P, S> Parser<Input> for Expected<P, S>
where
    P: Parser<Input>,
    Input: Stream,
    S: for<'s> ErrorInfo<'s, Input::Token, Input::Range>,
{
    type Output = P::Output;
    type PartialState = P::PartialState;

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
        self.0.parse_mode(mode, input, state)
    }

    fn add_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        ParseError::set_expected(errors, StreamError::expected(&self.1), |errors| {
            self.0.add_error(errors);
        })
    }

    forward_parser!(Input, parser_count add_committed_expected_error, 0);
}

/// Equivalent to [`p.expected(info)`].
///
/// [`p.expected(info)`]: ../trait.Parser.html#method.expected
pub fn expected<Input, P, S>(p: P, info: S) -> Expected<P, S>
where
    P: Parser<Input>,
    Input: Stream,
    S: for<'s> ErrorInfo<'s, Input::Token, Input::Range>,
{
    Expected(p, info)
}

#[derive(Clone)]
pub struct Silent<P>(P);
impl<Input, P> Parser<Input> for Silent<P>
where
    P: Parser<Input>,
    Input: Stream,
{
    type Output = P::Output;
    type PartialState = P::PartialState;

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
        self.0.parse_mode(mode, input, state).map_err(|mut err| {
            err.clear_expected();
            err
        })
    }

    fn add_error(&mut self, _errors: &mut Tracked<<Input as StreamOnce>::Error>) {}

    fn add_committed_expected_error(
        &mut self,
        _errors: &mut Tracked<<Input as StreamOnce>::Error>,
    ) {
    }

    forward_parser!(Input, parser_count, 0);
}

/// Equivalent to [`p.silent()`].
///
/// [`p.silent()`]: ../trait.Parser.html#method.silent
pub fn silent<Input, P>(p: P) -> Silent<P>
where
    P: Parser<Input>,
    Input: Stream,
{
    Silent(p)
}
