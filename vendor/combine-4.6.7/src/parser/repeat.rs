//! Combinators which take one or more parsers and applies them repeatedly.

use crate::{
    error::{
        Commit, ParseError,
        ParseResult::{self, *},
        ResultExt, StdParseResult, StreamError, Tracked,
    },
    lib::{borrow::BorrowMut, cmp, marker::PhantomData, mem},
    parser::{
        choice::{optional, Optional, Or},
        combinator::{ignore, Ignore},
        function::{parser, FnParser},
        sequence::With,
        token::{value, Value},
        FirstMode, ParseMode,
    },
    stream::{uncons, Stream, StreamOnce},
    ErrorOffset, Parser,
};

parser! {
pub struct Count;

/// Parses `parser` from zero up to `count` times.
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::error::Info;
/// # use combine::stream::easy::Error;
/// # fn main() {
/// let mut parser = count(2, token(b'a'));
///
/// let result = parser.parse(&b"aaab"[..]);
/// assert_eq!(result, Ok((b"aa"[..].to_owned(), &b"ab"[..])));
/// # }
/// ```
pub fn count[F, Input, P](count: usize, parser: P)(Input) -> F
where [
    Input: Stream,
    P: Parser<Input>,
    F: Extend<P::Output> + Default,
]
{
    count_min_max(0, *count, parser)
}
}

parser! {
    pub struct SkipCount;
    type PartialState = <With<Count<Sink, Input, P>, Value<Input, ()>> as Parser<Input>>::PartialState;
    /// Parses `parser` from zero up to `count` times skipping the output of `parser`.
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::stream::easy::{Error, Info};
    /// # fn main() {
    /// let mut parser = skip_count(2, token(b'a'));
    ///
    /// let result = parser.parse(&b"aaab"[..]);
    /// assert_eq!(result, Ok(((), &b"ab"[..])));
    /// # }
    /// ```
    pub fn skip_count[Input, P](count: usize, parser: P)(Input) -> ()
    where [
        P: Parser<Input>
    ]
    {
        self::count::<Sink, _, _>(*count, parser.map(|_| ())).with(value(()))
    }
}

#[derive(Copy, Clone)]
pub struct CountMinMax<F, P> {
    parser: P,
    min: usize,
    max: usize,
    _marker: PhantomData<fn() -> F>,
}

struct SuggestSizeHint<I> {
    iterator: I,
    min: usize,
    max: Option<usize>,
}

impl<I> Iterator for SuggestSizeHint<I>
where
    I: Iterator,
{
    type Item = I::Item;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.iterator.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.min, self.max)
    }
}

fn suggest_size_hint<I>(iterator: I, (min, max): (usize, Option<usize>)) -> SuggestSizeHint<I>
where
    I: Iterator,
{
    SuggestSizeHint {
        iterator,
        // Invalid input may report an extreme size so we guard against that (while still
        // optimizing by preallocating for the expected case of success)
        min: cmp::min(min, 4096),
        max,
    }
}

impl<Input, P, F> Parser<Input> for CountMinMax<F, P>
where
    Input: Stream,
    P: Parser<Input>,
    F: Extend<P::Output> + Default,
{
    type Output = F;
    type PartialState = (usize, F, P::PartialState);

    parse_mode!(Input);
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, Input::Error>
    where
        M: ParseMode,
    {
        let (count, elements, child_state) = state;

        let mut iter = self.parser.by_ref().partial_iter(mode, input, child_state);
        let remaining_min = self.min.saturating_sub(*count);
        let remaining_max = self.max - *count;
        elements.extend(suggest_size_hint(
            iter.by_ref().take(remaining_max).inspect(|_| *count += 1),
            (remaining_min, Some(remaining_max)),
        ));
        if *count < self.min {
            let err = StreamError::message_format(format_args!(
                "expected {} more elements",
                self.min - *count
            ));
            iter.fail(err)
        } else {
            iter.into_result_fast(elements).map(|x| {
                *count = 0;
                x
            })
        }
    }

    fn add_error(&mut self, error: &mut Tracked<<Input as StreamOnce>::Error>) {
        self.parser.add_error(error)
    }
}

/// Parses `parser` from `min` to `max` times (including `min` and `max`).
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::stream::easy::{Error, Info};
/// # fn main() {
/// let mut parser = count_min_max(2, 2, token(b'a'));
///
/// let result = parser.parse(&b"aaab"[..]);
/// assert_eq!(result, Ok((b"aa"[..].to_owned(), &b"ab"[..])));
/// let result = parser.parse(&b"ab"[..]);
/// assert!(result.is_err());
/// # }
/// ```
///
/// # Panics
///
/// If `min` > `max`.
pub fn count_min_max<F, Input, P>(min: usize, max: usize, parser: P) -> CountMinMax<F, P>
where
    Input: Stream,
    P: Parser<Input>,
    F: Extend<P::Output> + Default,
{
    assert!(min <= max);

    CountMinMax {
        parser,
        min,
        max,
        _marker: PhantomData,
    }
}

parser! {
    pub struct SkipCountMinMax;
    type PartialState = <With<CountMinMax<Sink, P>, Value<Input, ()>> as Parser<Input>>::PartialState;
    /// Parses `parser` from `min` to `max` times (including `min` and `max`)
    /// skipping the output of `parser`.
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::*;
    /// # fn main() {
    /// let mut parser = skip_count_min_max(2, 2, token(b'a'));
    ///
    /// let result = parser.parse(&b"aaab"[..]);
    /// assert_eq!(result, Ok(((), &b"ab"[..])));
    /// let result = parser.parse(&b"ab"[..]);
    /// assert!(result.is_err());
    /// # }
    /// ```
    ///
    /// # Panics
    ///
    /// If `min` > `max`.
    pub fn skip_count_min_max[Input, P](min: usize, max: usize, parser: P)(Input) -> ()
    where [
        P: Parser<Input>,
    ]
    {
       count_min_max::<Sink, _, _>(*min, *max, parser.map(|_| ())).with(value(()))
    }
}

pub struct Iter<'a, Input, P, S, M>
where
    Input: Stream,
    P: Parser<Input>,
{
    parser: P,
    input: &'a mut Input,
    committed: bool,
    state: State<<Input as StreamOnce>::Error>,
    partial_state: S,
    mode: M,
}

enum State<E> {
    Ok,
    PeekErr(E),
    CommitErr(E),
}

impl<'a, Input, P, S, M> Iter<'a, Input, P, S, M>
where
    Input: Stream,
    P: Parser<Input>,
    S: BorrowMut<P::PartialState>,
{
    pub fn new(parser: P, mode: M, input: &'a mut Input, partial_state: S) -> Self {
        Iter {
            parser,
            input,
            committed: false,
            state: State::Ok,
            partial_state,
            mode,
        }
    }
    /// Converts the iterator to a `ParseResult`, returning `Ok` if the parsing so far has be done
    /// without any errors which committed data.
    pub fn into_result<O>(self, value: O) -> StdParseResult<O, Input> {
        self.into_result_(value).into()
    }

    fn into_result_<O>(self, value: O) -> ParseResult<O, Input::Error> {
        match self.state {
            State::Ok | State::PeekErr(_) => {
                if self.committed {
                    CommitOk(value)
                } else {
                    PeekOk(value)
                }
            }
            State::CommitErr(e) => CommitErr(e),
        }
    }

    fn into_result_fast<O>(self, value: &mut O) -> ParseResult<O, Input::Error>
    where
        O: Default,
    {
        match self.state {
            State::Ok | State::PeekErr(_) => {
                let value = mem::take(value);
                if self.committed {
                    CommitOk(value)
                } else {
                    PeekOk(value)
                }
            }
            State::CommitErr(e) => CommitErr(e),
        }
    }

    fn fail<T>(
        self,
        err: <<Input as StreamOnce>::Error as ParseError<
            <Input as StreamOnce>::Token,
            <Input as StreamOnce>::Range,
            <Input as StreamOnce>::Position,
        >>::StreamError,
    ) -> ParseResult<T, Input::Error> {
        match self.state {
            State::Ok => {
                let err = <Input as StreamOnce>::Error::from_error(self.input.position(), err);
                if self.committed {
                    CommitErr(err)
                } else {
                    PeekErr(err.into())
                }
            }
            State::PeekErr(mut e) => {
                let err = <Input as StreamOnce>::Error::from_error(self.input.position(), err);
                e = e.merge(err);
                if self.committed {
                    CommitErr(e)
                } else {
                    PeekErr(e.into())
                }
            }
            State::CommitErr(mut e) => {
                e.add(err);
                CommitErr(e)
            }
        }
    }
}

impl<'a, Input, P, S, M> Iterator for Iter<'a, Input, P, S, M>
where
    Input: Stream,
    P: Parser<Input>,
    S: BorrowMut<P::PartialState>,
    M: ParseMode,
{
    type Item = P::Output;

    fn next(&mut self) -> Option<P::Output> {
        let before = self.input.checkpoint();
        match self
            .parser
            .parse_mode(self.mode, self.input, self.partial_state.borrow_mut())
        {
            PeekOk(v) => {
                self.mode.set_first();
                Some(v)
            }
            CommitOk(v) => {
                self.mode.set_first();
                self.committed = true;
                Some(v)
            }
            PeekErr(e) => {
                self.state = match self.input.reset(before) {
                    Err(err) => State::CommitErr(err),
                    Ok(_) => State::PeekErr(e.error),
                };
                None
            }
            CommitErr(e) => {
                self.state = State::CommitErr(e);
                None
            }
        }
    }
}

#[derive(Copy, Clone)]
pub struct Many<F, P>(P, PhantomData<F>);

impl<F, Input, P> Parser<Input> for Many<F, P>
where
    Input: Stream,
    P: Parser<Input>,
    F: Extend<P::Output> + Default,
{
    type Output = F;
    type PartialState = (F, P::PartialState);

    parse_mode!(Input);
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, Input::Error>
    where
        M: ParseMode,
    {
        // TODO
        let (ref mut elements, ref mut child_state) = *state;

        let mut iter = (&mut self.0).partial_iter(mode, input, child_state);
        elements.extend(iter.by_ref());
        iter.into_result_fast(elements)
    }

    fn add_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        self.0.add_error(errors)
    }

    fn add_committed_expected_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        self.add_error(errors);
    }

    fn parser_count(&self) -> ErrorOffset {
        self.0.parser_count()
    }
}

/// Parses `p` zero or more times returning a collection with the values from `p`.
///
/// If the returned collection cannot be inferred type annotations must be supplied, either by
/// annotating the resulting type binding `let collection: Vec<_> = ...` or by specializing when
/// calling many, `many::<Vec<_>, _, _>(...)`.
///
/// NOTE: If `p` can succeed without consuming any input this may hang forever as `many` will
/// repeatedly use `p` to parse the same location in the input every time
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::char::digit;
/// # fn main() {
/// let result = many(digit())
///     .parse("123A")
///     .map(|x| x.0);
/// assert_eq!(result, Ok(vec!['1', '2', '3']));
/// # }
/// ```
pub fn many<F, Input, P>(p: P) -> Many<F, P>
where
    Input: Stream,
    P: Parser<Input>,
    F: Extend<P::Output> + Default,
{
    Many(p, PhantomData)
}

#[derive(Copy, Clone)]
pub struct Many1<F, P>(P, PhantomData<fn() -> F>);
impl<F, Input, P> Parser<Input> for Many1<F, P>
where
    Input: Stream,
    F: Extend<P::Output> + Default,
    P: Parser<Input>,
{
    type Output = F;
    type PartialState = (bool, bool, F, P::PartialState);

    parse_mode!(Input);
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mut mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<F, Input::Error>
    where
        M: ParseMode,
    {
        let (ref mut parsed_one, ref mut committed_state, ref mut elements, ref mut child_state) =
            *state;

        if mode.is_first() || !*parsed_one {
            debug_assert!(!*parsed_one);

            let (first, committed) = ctry!(self.0.parse_mode(mode, input, child_state));
            elements.extend(Some(first));
            // TODO Should PeekOk be an error?
            *committed_state = !committed.is_peek();
            *parsed_one = true;
            mode.set_first();
        }

        let mut iter = Iter {
            parser: &mut self.0,
            committed: *committed_state,
            input,
            state: State::Ok,
            partial_state: child_state,
            mode,
        };
        elements.extend(iter.by_ref());

        iter.into_result_fast(elements).map(|x| {
            *parsed_one = false;
            x
        })
    }

    fn add_committed_expected_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        self.add_error(errors);
    }

    forward_parser!(Input, add_error parser_count, 0);
}

/// Parses `p` one or more times returning a collection with the values from `p`.
///
/// If the returned collection cannot be inferred type annotations must be supplied, either by
/// annotating the resulting type binding `let collection: Vec<_> = ...` or by specializing when
/// calling many1 `many1::<Vec<_>, _>(...)`.
///
/// NOTE: If `p` can succeed without consuming any input this may hang forever as `many1` will
/// repeatedly use `p` to parse the same location in the input every time
///
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::char::digit;
/// # fn main() {
/// let result = many1::<Vec<_>, _, _>(digit())
///     .parse("A123");
/// assert!(result.is_err());
/// # }
/// ```
pub fn many1<F, Input, P>(p: P) -> Many1<F, P>
where
    Input: Stream,
    F: Extend<P::Output> + Default,
    P: Parser<Input>,
{
    Many1(p, PhantomData)
}

#[derive(Clone)]
#[doc(hidden)]
// FIXME Should not be public
pub struct Sink;

impl Default for Sink {
    fn default() -> Self {
        Sink
    }
}

impl<A> Extend<A> for Sink {
    fn extend<T>(&mut self, iter: T)
    where
        T: IntoIterator<Item = A>,
    {
        for _ in iter {}
    }
}

parser! {
    pub struct SkipMany;
    type PartialState = <Ignore<Many<Sink, Ignore<P>>> as Parser<Input>>::PartialState;
/// Parses `p` zero or more times ignoring the result.
///
/// NOTE: If `p` can succeed without consuming any input this may hang forever as `skip_many` will
/// repeatedly use `p` to parse the same location in the input every time
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::char::digit;
/// # fn main() {
/// let result = skip_many(digit())
///     .parse("A");
/// assert_eq!(result, Ok(((), "A")));
/// # }
/// ```
pub fn skip_many[Input, P](p: P)(Input) -> ()
where [
    P: Parser<Input>,
]
{
    ignore(many::<Sink, _, _>(ignore(p)))
}
}

parser! {
    pub struct SkipMany1;
    type PartialState = <Ignore<Many1<Sink, Ignore<P>>> as Parser<Input>>::PartialState;
/// Parses `p` one or more times ignoring the result.
///
/// NOTE: If `p` can succeed without consuming any input this may hang forever as `skip_many1` will
/// repeatedly use `p` to parse the same location in the input every time
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::char::digit;
/// # fn main() {
/// let result = skip_many1(digit())
///     .parse("123A");
/// assert_eq!(result, Ok(((), "A")));
/// # }
/// ```
pub fn skip_many1[Input, P](p: P)(Input) -> ()
where [
    P: Parser<Input>,
]
{
    ignore(many1::<Sink, _, _>(ignore(p)))
}
}

#[derive(Copy, Clone)]
pub struct SepBy<F, P, S> {
    parser: P,
    separator: S,
    _marker: PhantomData<fn() -> F>,
}
impl<F, Input, P, S> Parser<Input> for SepBy<F, P, S>
where
    Input: Stream,
    F: Extend<P::Output> + Default,
    P: Parser<Input>,
    S: Parser<Input>,
{
    type Output = F;
    type PartialState = <Or<
        SepBy1<F, P, S>,
        FnParser<Input, fn(&mut Input) -> StdParseResult<F, Input>>,
    > as Parser<Input>>::PartialState;

    parse_mode!(Input);
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<F, Input::Error>
    where
        M: ParseMode,
    {
        sep_by1(&mut self.parser, &mut self.separator)
            .or(parser(|_| Ok((F::default(), Commit::Peek(())))))
            .parse_mode(mode, input, state)
    }

    fn add_committed_expected_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        self.separator.add_error(errors)
    }

    forward_parser!(Input, add_error parser_count, parser);
}

/// Parses `parser` zero or more time separated by `separator`, returning a collection with the
/// values from `p`.
///
/// If the returned collection cannot be inferred type annotations must be supplied, either by
/// annotating the resulting type binding `let collection: Vec<_> = ...` or by specializing when
/// calling `sep_by`, `sep_by::<Vec<_>, _, _>(...)`.
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::char::digit;
/// # fn main() {
/// let mut parser = sep_by(digit(), token(','));
/// let result_ok = parser.parse("1,2,3");
/// assert_eq!(result_ok, Ok((vec!['1', '2', '3'], "")));
/// let result_ok2 = parser.parse("");
/// assert_eq!(result_ok2, Ok((vec![], "")));
/// # }
/// ```
pub fn sep_by<F, Input, P, S>(parser: P, separator: S) -> SepBy<F, P, S>
where
    Input: Stream,
    F: Extend<P::Output> + Default,
    P: Parser<Input>,
    S: Parser<Input>,
{
    SepBy {
        parser,
        separator,
        _marker: PhantomData,
    }
}

#[derive(Copy, Clone)]
pub struct SepBy1<F, P, S> {
    parser: P,
    separator: S,
    _marker: PhantomData<fn() -> F>,
}
impl<F, Input, P, S> Parser<Input> for SepBy1<F, P, S>
where
    Input: Stream,
    F: Extend<P::Output> + Default,
    P: Parser<Input>,
    S: Parser<Input>,
{
    type Output = F;
    type PartialState = (
        Option<Commit<()>>,
        F,
        <With<S, P> as Parser<Input>>::PartialState,
    );

    parse_mode!(Input);
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, Input::Error>
    where
        M: ParseMode,
    {
        let (ref mut parsed_one, ref mut elements, ref mut child_state) = *state;

        let rest = match *parsed_one {
            Some(rest) => rest,
            None => {
                let (first, rest) =
                    ctry!(self
                        .parser
                        .parse_mode(mode, input, &mut child_state.B.state));
                elements.extend(Some(first));
                rest
            }
        };

        rest.combine_commit(move |_| {
            let rest = (&mut self.separator).with(&mut self.parser);
            let mut iter = Iter::new(rest, mode, input, child_state);

            elements.extend(iter.by_ref());

            iter.into_result_fast(elements).map(|x| {
                *parsed_one = None;
                x
            })
        })
    }

    fn add_committed_expected_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        self.separator.add_error(errors)
    }

    forward_parser!(Input, add_error parser_count, parser);
}

/// Parses `parser` one or more time separated by `separator`, returning a collection with the
/// values from `p`.
///
/// If the returned collection cannot be inferred type annotations must be supplied, either by
/// annotating the resulting type binding `let collection: Vec<_> = ...` or by specializing when
/// calling `sep_by`, `sep_by1::<Vec<_>, _, _>(...)`.
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::char::digit;
/// # use combine::stream::easy;
/// # use combine::stream::position::{self, SourcePosition};
/// # fn main() {
/// let mut parser = sep_by1(digit(), token(','));
/// let result_ok = parser.easy_parse(position::Stream::new("1,2,3"))
///                       .map(|(vec, state)| (vec, state.input));
/// assert_eq!(result_ok, Ok((vec!['1', '2', '3'], "")));
/// let result_err = parser.easy_parse(position::Stream::new(""));
/// assert_eq!(result_err, Err(easy::Errors {
///     position: SourcePosition::default(),
///     errors: vec![
///         easy::Error::end_of_input(),
///         easy::Error::Expected("digit".into())
///     ]
/// }));
/// # }
/// ```
pub fn sep_by1<F, Input, P, S>(parser: P, separator: S) -> SepBy1<F, P, S>
where
    Input: Stream,
    F: Extend<P::Output> + Default,
    P: Parser<Input>,
    S: Parser<Input>,
{
    SepBy1 {
        parser,
        separator,
        _marker: PhantomData,
    }
}

#[derive(Copy, Clone)]
pub struct SepEndBy<F, P, S> {
    parser: P,
    separator: S,
    _marker: PhantomData<fn() -> F>,
}

impl<F, Input, P, S> Parser<Input> for SepEndBy<F, P, S>
where
    Input: Stream,
    F: Extend<P::Output> + Default,
    P: Parser<Input>,
    S: Parser<Input>,
{
    type Output = F;
    type PartialState = <Or<
        SepEndBy1<F, P, S>,
        FnParser<Input, fn(&mut Input) -> StdParseResult<F, Input>>,
    > as Parser<Input>>::PartialState;

    parse_mode!(Input);
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, Input::Error>
    where
        M: ParseMode,
    {
        sep_end_by1(&mut self.parser, &mut self.separator)
            .or(parser(|_| Ok((F::default(), Commit::Peek(())))))
            .parse_mode(mode, input, state)
    }

    fn add_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        self.parser.add_error(errors)
    }
}

/// Parses `parser` zero or more times separated and ended by `separator`, returning a collection
/// with the values from `p`.
///
/// If the returned collection cannot be inferred type annotations must be supplied, either by
/// annotating the resulting type binding `let collection: Vec<_> = ...` or by specializing when
/// calling `sep_by`, `sep_by::<Vec<_>, _, _>(...)`
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::char::digit;
/// # fn main() {
/// let mut parser = sep_end_by(digit(), token(';'));
/// let result_ok = parser.parse("1;2;3;");
/// assert_eq!(result_ok, Ok((vec!['1', '2', '3'], "")));
/// let result_ok2 = parser.parse("1;2;3");
/// assert_eq!(result_ok2, Ok((vec!['1', '2', '3'], "")));
/// # }
/// ```
pub fn sep_end_by<F, Input, P, S>(parser: P, separator: S) -> SepEndBy<F, P, S>
where
    Input: Stream,
    F: Extend<P::Output> + Default,
    P: Parser<Input>,
    S: Parser<Input>,
{
    SepEndBy {
        parser,
        separator,
        _marker: PhantomData,
    }
}

#[derive(Copy, Clone)]
pub struct SepEndBy1<F, P, S> {
    parser: P,
    separator: S,
    _marker: PhantomData<fn() -> F>,
}

impl<F, Input, P, S> Parser<Input> for SepEndBy1<F, P, S>
where
    Input: Stream,
    F: Extend<P::Output> + Default,
    P: Parser<Input>,
    S: Parser<Input>,
{
    type Output = F;
    type PartialState = (
        Option<Commit<()>>,
        F,
        <With<S, Optional<P>> as Parser<Input>>::PartialState,
    );

    parse_mode!(Input);
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, Input::Error>
    where
        M: ParseMode,
    {
        let (ref mut parsed_one, ref mut elements, ref mut child_state) = *state;

        let rest = match *parsed_one {
            Some(rest) => rest,
            None => {
                let (first, rest) =
                    ctry!(self
                        .parser
                        .parse_mode(mode, input, &mut child_state.B.state));
                *parsed_one = Some(rest);
                elements.extend(Some(first));
                rest
            }
        };

        rest.combine_commit(|_| {
            let rest = (&mut self.separator).with(optional(&mut self.parser));
            let mut iter = Iter::new(rest, mode, input, child_state);

            // Parse elements until `self.parser` returns `None`
            elements.extend(iter.by_ref().scan((), |_, x| x));

            if iter.committed {
                *parsed_one = Some(Commit::Commit(()));
            }

            iter.into_result_fast(elements).map(|x| {
                *parsed_one = None;
                x
            })
        })
    }

    fn add_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        self.parser.add_error(errors)
    }
}

/// Parses `parser` one or more times separated and ended by `separator`, returning a collection
/// with the values from `p`.
///
/// If the returned collection cannot be inferred type annotations must be
/// supplied, either by annotating the resulting type binding `let collection: Vec<_> = ...` or by
/// specializing when calling `sep_by`, `sep_by1::<Vec<_>, _, _>(...)`.
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::char::digit;
/// # use combine::stream::easy;
/// # use combine::stream::position::{self, SourcePosition};
/// # fn main() {
/// let mut parser = sep_end_by1(digit(), token(';'));
/// let result_ok = parser.easy_parse(position::Stream::new("1;2;3;"))
///                       .map(|(vec, state)| (vec, state.input));
/// assert_eq!(result_ok, Ok((vec!['1', '2', '3'], "")));
/// let result_err = parser.easy_parse(position::Stream::new(""));
/// assert_eq!(result_err, Err(easy::Errors {
///     position: SourcePosition::default(),
///     errors: vec![
///         easy::Error::end_of_input(),
///         easy::Error::Expected("digit".into())
///     ]
/// }));
/// # }
/// ```
pub fn sep_end_by1<F, Input, P, S>(parser: P, separator: S) -> SepEndBy1<F, P, S>
where
    Input: Stream,
    F: Extend<P::Output> + Default,
    P: Parser<Input>,
    S: Parser<Input>,
{
    SepEndBy1 {
        parser,
        separator,
        _marker: PhantomData,
    }
}

#[derive(Copy, Clone)]
pub struct Chainl1<P, Op>(P, Op);
impl<Input, P, Op> Parser<Input> for Chainl1<P, Op>
where
    Input: Stream,
    P: Parser<Input>,
    Op: Parser<Input>,
    Op::Output: FnOnce(P::Output, P::Output) -> P::Output,
{
    type Output = P::Output;
    type PartialState = (
        Option<(P::Output, Commit<()>)>,
        <(Op, P) as Parser<Input>>::PartialState,
    );

    parse_mode!(Input);
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mut mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, Input::Error>
    where
        M: ParseMode,
    {
        let (ref mut l_state, ref mut child_state) = *state;

        let (mut l, mut committed) = match l_state.take() {
            Some(x) => x,
            None => {
                let x = ctry!(self.0.parse_partial(input, &mut child_state.B.state));
                mode.set_first();
                x
            }
        };

        loop {
            let before = input.checkpoint();
            match (&mut self.1, &mut self.0)
                .parse_mode(mode, input, child_state)
                .into()
            {
                Ok(((op, r), rest)) => {
                    l = op(l, r);
                    committed = committed.merge(rest);
                    mode.set_first();
                }
                Err(Commit::Commit(err)) => {
                    *l_state = Some((l, committed));
                    return CommitErr(err.error);
                }
                Err(Commit::Peek(_)) => {
                    ctry!(input.reset(before).committed());
                    break;
                }
            }
        }
        Ok((l, committed)).into()
    }

    fn add_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        self.0.add_error(errors)
    }
}

/// Parses `p` 1 or more times separated by `op`. The value returned is the one produced by the
/// left associative application of the function returned by the parser `op`.
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::char::digit;
/// # fn main() {
/// let number = digit().map(|c: char| c.to_digit(10).unwrap());
/// let sub = token('-').map(|_| |l: u32, r: u32| l - r);
/// let mut parser = chainl1(number, sub);
/// assert_eq!(parser.parse("9-3-5"), Ok((1, "")));
/// # }
/// ```
pub fn chainl1<Input, P, Op>(parser: P, op: Op) -> Chainl1<P, Op>
where
    Input: Stream,
    P: Parser<Input>,
    Op: Parser<Input>,
    Op::Output: FnOnce(P::Output, P::Output) -> P::Output,
{
    Chainl1(parser, op)
}

#[derive(Copy, Clone)]
pub struct Chainr1<P, Op>(P, Op);
impl<Input, P, Op> Parser<Input> for Chainr1<P, Op>
where
    Input: Stream,
    P: Parser<Input>,
    Op: Parser<Input>,
    Op::Output: FnOnce(P::Output, P::Output) -> P::Output,
{
    type Output = P::Output;
    type PartialState = ();
    #[inline]
    fn parse_lazy(&mut self, input: &mut Input) -> ParseResult<P::Output, Input::Error> {
        // FIXME FastResult
        let (mut l, mut committed) = ctry!(self.0.parse_lazy(input));
        loop {
            let before = input.checkpoint();
            let op = match self.1.parse_lazy(input).into() {
                Ok((x, rest)) => {
                    committed = committed.merge(rest);
                    x
                }
                Err(Commit::Commit(err)) => return CommitErr(err.error),
                Err(Commit::Peek(_)) => {
                    ctry!(input.reset(before).committed());
                    break;
                }
            };
            let before = input.checkpoint();
            match self.parse_lazy(input).into() {
                Ok((r, rest)) => {
                    l = op(l, r);
                    committed = committed.merge(rest);
                }
                Err(Commit::Commit(err)) => return CommitErr(err.error),
                Err(Commit::Peek(_)) => {
                    ctry!(input.reset(before).committed());
                    break;
                }
            }
        }
        Ok((l, committed)).into()
    }
    fn add_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        self.0.add_error(errors)
    }
}

/// Parses `p` one or more times separated by `op`. The value returned is the one produced by the
/// right associative application of the function returned by `op`.
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::char::digit;
/// # fn main() {
/// let number = digit().map(|c: char| c.to_digit(10).unwrap());
/// let pow = token('^').map(|_| |l: u32, r: u32| l.pow(r));
/// let mut parser = chainr1(number, pow);
///     assert_eq!(parser.parse("2^3^2"), Ok((512, "")));
/// }
/// ```
pub fn chainr1<Input, P, Op>(parser: P, op: Op) -> Chainr1<P, Op>
where
    Input: Stream,
    P: Parser<Input>,
    Op: Parser<Input>,
    Op::Output: FnOnce(P::Output, P::Output) -> P::Output,
{
    Chainr1(parser, op)
}

#[derive(Copy, Clone)]
pub struct TakeUntil<F, P> {
    end: P,
    _marker: PhantomData<fn() -> F>,
}
impl<F, Input, P> Parser<Input> for TakeUntil<F, P>
where
    Input: Stream,
    F: Extend<<Input as StreamOnce>::Token> + Default,
    P: Parser<Input>,
{
    type Output = F;
    type PartialState = (F, P::PartialState);

    parse_mode!(Input);
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, Input::Error>
    where
        M: ParseMode,
    {
        let (ref mut output, ref mut end_state) = *state;

        let mut committed = Commit::Peek(());
        loop {
            let before = input.checkpoint();
            match self.end.parse_mode(mode, input, end_state).into() {
                Ok((_, rest)) => {
                    ctry!(input.reset(before).committed());
                    return match committed.merge(rest) {
                        Commit::Commit(()) => CommitOk(mem::take(output)),
                        Commit::Peek(()) => PeekOk(mem::take(output)),
                    };
                }
                Err(Commit::Peek(_)) => {
                    ctry!(input.reset(before).committed());
                    output.extend(Some(ctry!(uncons(input)).0));
                    committed = Commit::Commit(());
                }
                Err(Commit::Commit(e)) => {
                    ctry!(input.reset(before).committed());
                    return CommitErr(e.error);
                }
            };
        }
    }
}

/// Takes input until `end` is encountered or `end` indicates that it has committed input before
/// failing (`attempt` can be used to make it look like it has not committed any input)
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::char;
/// # use combine::parser::byte;
/// # use combine::parser::combinator::attempt;
/// # use combine::parser::repeat::take_until;
/// # fn main() {
/// let mut char_parser = take_until(char::digit());
/// assert_eq!(char_parser.parse("abc123"), Ok(("abc".to_string(), "123")));
///
/// let mut byte_parser = take_until(byte::bytes(&b"TAG"[..]));
/// assert_eq!(byte_parser.parse(&b"123TAG"[..]), Ok((b"123".to_vec(), &b"TAG"[..])));
/// assert!(byte_parser.parse(&b"123TATAG"[..]).is_err());
///
/// // `attempt` must be used if the `end` should be consume input before failing
/// let mut byte_parser = take_until(attempt(byte::bytes(&b"TAG"[..])));
/// assert_eq!(byte_parser.parse(&b"123TATAG"[..]), Ok((b"123TA".to_vec(), &b"TAG"[..])));
/// # }
/// ```
pub fn take_until<F, Input, P>(end: P) -> TakeUntil<F, P>
where
    Input: Stream,
    F: Extend<<Input as StreamOnce>::Token> + Default,
    P: Parser<Input>,
{
    TakeUntil {
        end,
        _marker: PhantomData,
    }
}

parser! {
    pub struct SkipUntil;
    type PartialState = <With<TakeUntil<Sink, P>, Value<Input, ()>> as Parser<Input>>::PartialState;
    /// Skips input until `end` is encountered or `end` indicates that it has committed input before
    /// failing (`attempt` can be used to make it look like it has not committed any input)
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::parser::char;
    /// # use combine::parser::byte;
    /// # use combine::parser::combinator::attempt;
    /// # use combine::parser::repeat::skip_until;
    /// # fn main() {
    /// let mut char_parser = skip_until(char::digit());
    /// assert_eq!(char_parser.parse("abc123"), Ok(((), "123")));
    ///
    /// let mut byte_parser = skip_until(byte::bytes(&b"TAG"[..]));
    /// assert_eq!(byte_parser.parse(&b"123TAG"[..]), Ok(((), &b"TAG"[..])));
    /// assert!(byte_parser.parse(&b"123TATAG"[..]).is_err());
    ///
    /// // `attempt` must be used if the `end` should consume input before failing
    /// let mut byte_parser = skip_until(attempt(byte::bytes(&b"TAG"[..])));
    /// assert_eq!(byte_parser.parse(&b"123TATAG"[..]), Ok(((), &b"TAG"[..])));
    /// # }
    /// ```
    pub fn skip_until[Input, P](end: P)(Input) -> ()
    where [
        P: Parser<Input>,
    ]
    {
        take_until::<Sink, _, _>(end).with(value(()))
    }
}

#[derive(Copy, Clone)]
pub struct RepeatUntil<F, P, E> {
    parser: P,
    end: E,
    _marker: PhantomData<fn() -> F>,
}
impl<F, Input, P, E> Parser<Input> for RepeatUntil<F, P, E>
where
    Input: Stream,
    F: Extend<P::Output> + Default,
    P: Parser<Input>,
    E: Parser<Input>,
{
    type Output = F;
    type PartialState = (F, bool, P::PartialState, E::PartialState);

    parse_mode!(Input);
    #[inline]
    fn parse_mode_impl<M>(
        &mut self,
        mut mode: M,
        input: &mut Input,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, Input::Error>
    where
        M: ParseMode,
    {
        let (output, is_parse, parse_state, end_state) = state;

        let mut committed = Commit::Peek(());
        loop {
            if *is_parse {
                let (token, c) = ctry!(self.parser.parse_mode(mode, input, parse_state));
                output.extend(Some(token));
                committed = committed.merge(c);
                *is_parse = false;
            } else {
                let before = input.checkpoint();
                match self.end.parse_mode(mode, input, end_state).into() {
                    Ok((_, rest)) => {
                        ctry!(input.reset(before).committed());
                        return match committed.merge(rest) {
                            Commit::Commit(()) => CommitOk(mem::take(output)),
                            Commit::Peek(()) => PeekOk(mem::take(output)),
                        };
                    }
                    Err(Commit::Peek(_)) => {
                        ctry!(input.reset(before).committed());
                        mode.set_first();
                        *is_parse = true;
                    }
                    Err(Commit::Commit(e)) => {
                        ctry!(input.reset(before).committed());
                        return CommitErr(e.error);
                    }
                }
            }
        }
    }
}

pub fn repeat_until<F, Input, P, E>(parser: P, end: E) -> RepeatUntil<F, P, E>
where
    Input: Stream,
    F: Extend<P::Output> + Default,
    P: Parser<Input>,
    E: Parser<Input>,
{
    RepeatUntil {
        parser,
        end,
        _marker: PhantomData,
    }
}

parser! {
    pub struct SkipRepeatUntil;
    type PartialState = <With<RepeatUntil<Sink, P, E>, Value<Input, ()>> as Parser<Input>>::PartialState;
    /// Skips input until `end` is encountered or `end` indicates that it has committed input before
    /// failing (`attempt` can be used to continue skipping even if `end` has committed input)
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::*;
    /// # use combine::parser::char;
    /// # use combine::parser::byte;
    /// # use combine::parser::combinator::attempt;
    /// # use combine::parser::repeat::skip_until;
    /// # fn main() {
    ///     let mut char_parser = skip_until(char::digit());
    ///     assert_eq!(char_parser.parse("abc123"), Ok(((), "123")));
    ///
    ///     let mut byte_parser = skip_until(byte::bytes(&b"TAG"[..]));
    ///     assert_eq!(byte_parser.parse(&b"123TAG"[..]), Ok(((), &b"TAG"[..])));
    ///     assert!(byte_parser.parse(&b"123TATAG"[..]).is_err());
    ///
    ///     // `attempt` must be used because the `end` will commit to `TA` before failing,
    ///     // but we want to continue skipping
    ///     let mut byte_parser = skip_until(attempt(byte::bytes(&b"TAG"[..])));
    ///     assert_eq!(byte_parser.parse(&b"123TATAG"[..]), Ok(((), &b"TAG"[..])));
    /// }
    /// ```
    pub fn repeat_skip_until[Input, P, E](parser: P, end: E)(Input) -> ()
    where [
        P: Parser<Input>,
        E: Parser<Input>,
    ]
    {
        repeat_until::<Sink, _, _, _>(parser, end).with(value(()))
    }
}

#[derive(Default)]
pub struct EscapedState<T, U>(PhantomData<(T, U)>);

pub struct Escaped<P, Q, I> {
    parser: P,
    escape: I,
    escape_parser: Q,
}
impl<Input, P, Q> Parser<Input> for Escaped<P, Q, Input::Token>
where
    Input: Stream,
    P: Parser<Input>,
    <Input as StreamOnce>::Token: PartialEq,
    Q: Parser<Input>,
{
    type Output = ();
    type PartialState = EscapedState<P::PartialState, Q::PartialState>;

    fn parse_lazy(&mut self, input: &mut Input) -> ParseResult<Self::Output, Input::Error> {
        let mut committed = Commit::Peek(());
        loop {
            match self.parser.parse_lazy(input) {
                PeekOk(_) => {}
                CommitOk(_) => {
                    committed = Commit::Commit(());
                }
                PeekErr(_) => {
                    let checkpoint = input.checkpoint();
                    match uncons(input) {
                        CommitOk(ref c) | PeekOk(ref c) if *c == self.escape => {
                            match self.escape_parser.parse_committed_mode(
                                FirstMode,
                                input,
                                &mut Default::default(),
                            ) {
                                PeekOk(_) => {}
                                CommitOk(_) => {
                                    committed = Commit::Commit(());
                                }
                                CommitErr(err) => return CommitErr(err),
                                PeekErr(err) => {
                                    return CommitErr(err.error);
                                }
                            }
                        }
                        CommitErr(err) => {
                            return CommitErr(err);
                        }
                        _ => {
                            ctry!(input.reset(checkpoint).committed());
                            return if committed.is_peek() {
                                PeekOk(())
                            } else {
                                CommitOk(())
                            };
                        }
                    }
                }
                CommitErr(err) => return CommitErr(err),
            }
        }
    }

    fn add_error(&mut self, errors: &mut Tracked<<Input as StreamOnce>::Error>) {
        use crate::error;

        self.parser.add_error(errors);

        errors.error.add_expected(error::Token(self.escape.clone()));
    }
}

/// Parses an escaped string by first applying `parser` which accept the normal characters which do
/// not need escaping. Once `parser` can not consume any more input it checks if the next token
/// is `escape`. If it is then `escape_parser` is used to parse the escaped character and then
/// resumes parsing using `parser`. If `escape` was not found then the parser finishes
/// successfully.
///
/// This returns `()` since there isn't a good way to collect the output of the parsers so it is
/// best paired with one of the `recognize` parsers.
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::repeat::escaped;
/// # use combine::parser::char;
/// # use combine::parser::range::{recognize, take_while1};
/// # fn main() {
///     let mut parser = recognize(
///         escaped(take_while1(|c| c != '"' && c != '\\'), '\\', one_of(r#"nr"\"#.chars()))
///     );
///     assert_eq!(parser.parse(r#"ab\"12\n\rc""#), Ok((r#"ab\"12\n\rc"#, r#"""#)));
///     assert!(parser.parse(r#"\"#).is_err());
///     assert!(parser.parse(r#"\a"#).is_err());
/// }
/// ```
pub fn escaped<Input, P, Q>(
    parser: P,
    escape: <Input as StreamOnce>::Token,
    escape_parser: Q,
) -> Escaped<P, Q, Input::Token>
where
    Input: Stream,
    P: Parser<Input>,
    <Input as StreamOnce>::Token: PartialEq,
    Q: Parser<Input>,
{
    Escaped {
        parser,
        escape,
        escape_parser,
    }
}

pub struct Iterate<F, I, P> {
    parser: P,
    iterable: I,
    _marker: PhantomData<fn() -> F>,
}
impl<'s, 'a, P, Q, I, J, F> Parser<I> for Iterate<F, J, P>
where
    P: FnMut(&J::Item, &mut I) -> Q,
    Q: Parser<I>,
    I: Stream,
    J: IntoIterator + Clone,
    F: Extend<Q::Output> + Default,
{
    type Output = F;
    type PartialState = (
        Option<(J::IntoIter, Option<J::Item>)>,
        bool,
        F,
        Q::PartialState,
    );

    parse_mode!(I);

    fn parse_mode_impl<M>(
        &mut self,
        mut mode: M,
        input: &mut I,
        state: &mut Self::PartialState,
    ) -> ParseResult<Self::Output, I::Error>
    where
        M: ParseMode,
    {
        let (opt_iter, committed, buf, next) = state;
        let (iter, next_item) = match opt_iter {
            Some(iter) if !mode.is_first() => iter,
            _ => {
                *opt_iter = Some((self.iterable.clone().into_iter(), None));
                opt_iter.as_mut().unwrap()
            }
        };

        let mut consume = |item: J::Item| {
            let mut parser = (self.parser)(&item, input);
            let before = input.checkpoint();
            match parser.parse_mode(mode, input, next) {
                PeekOk(v) => {
                    mode.set_first();
                    Ok(v)
                }
                CommitOk(v) => {
                    mode.set_first();
                    *committed = true;
                    Ok(v)
                }
                PeekErr(err) => {
                    if let Err(err) = input.reset(before) {
                        return Err((item, CommitErr(err)));
                    }
                    Err((
                        item,
                        if *committed {
                            CommitErr(err.error)
                        } else {
                            PeekErr(err)
                        },
                    ))
                }
                CommitErr(err) => Err((item, CommitErr(err))),
            }
        };

        let result = (|| {
            if let Some(item) = next_item.take() {
                buf.extend(Some(consume(item)?));
            }
            let mut result = Ok(());
            let size_hint = iter.size_hint();
            buf.extend(suggest_size_hint(
                iter.scan((), |_, item| match consume(item) {
                    Ok(item) => Some(item),
                    Err(err) => {
                        result = Err(err);
                        None
                    }
                }),
                size_hint,
            ));
            result
        })();

        if let Err((item, err)) = result {
            *next_item = Some(item);
            return err;
        }

        opt_iter.take();

        let value = mem::take(buf);
        if *committed {
            *committed = false;
            CommitOk(value)
        } else {
            PeekOk(value)
        }
    }
}

///
/// ```
/// # use combine::parser::repeat::{count_min_max, iterate};
/// # use combine::*;
///
/// assert_eq!(
///     iterate(0..3, |&i, _| count_min_max(i, i, any())).parse("abbccc"),
///     Ok((vec!["".to_string(), "a".to_string(), "bb".to_string()], "ccc")),
/// );
/// ```
pub fn iterate<F, J, P, I, Q>(iterable: J, parser: P) -> Iterate<F, J, P>
where
    P: FnMut(&J::Item, &mut I) -> Q,
    Q: Parser<I>,
    I: Stream,
    J: IntoIterator + Clone,
    F: Extend<Q::Output> + Default,
{
    Iterate {
        parser,
        iterable,
        _marker: PhantomData,
    }
}
