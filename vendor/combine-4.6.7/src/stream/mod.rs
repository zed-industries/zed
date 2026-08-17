//
//Traits and implementations of arbitrary data streams.
//!
//! Streams are similar to the `Iterator` trait in that they represent some sequential set of items
//! which can be retrieved one by one. Where `Stream`s differ is that they are allowed to return
//! errors instead of just `None` and if they implement the `RangeStreamOnce` trait they are also
//! capable of returning multiple items at the same time, usually in the form of a slice.
//!
//! In addition to he functionality above, a proper `Stream` usable by a `Parser` must also have a
//! position (marked by the `Positioned` trait) and must also be resetable (marked by the
//! `ResetStream` trait). The former is used to ensure that errors at different points in the stream
//! aren't combined and the latter is used in parsers such as `or` to try multiple alternative
//! parses.

use crate::lib::{cmp::Ordering, fmt, marker::PhantomData, str::Chars};

use crate::{
    error::{
        ParseError,
        ParseResult::{self, *},
        StreamError, StringStreamError, Tracked, UnexpectedParse,
    },
    Parser,
};

#[cfg(feature = "std")]
pub use self::decoder::Decoder;

#[doc(hidden)]
#[macro_export]
macro_rules! clone_resetable {
    (( $($params: tt)* ) $ty: ty) => {
        impl<$($params)*> ResetStream for $ty
            where Self: StreamOnce
        {
            type Checkpoint = Self;

            fn checkpoint(&self) -> Self {
                self.clone()
            }
            #[inline]
            fn reset(&mut self, checkpoint: Self) -> Result<(), Self::Error> {
                *self = checkpoint;
                Ok(())
            }
        }
    }
}

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod buf_reader;
/// Stream wrapper which provides a `ResetStream` impl for `StreamOnce` impls which do not have
/// one.
#[cfg(feature = "alloc")]
#[cfg_attr(docsrs, doc(cfg(feature = "alloc")))]
pub mod buffered;
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod easy;
/// Stream wrapper which provides more detailed position information.
pub mod position;
/// Stream wrapper allowing `std::io::Read` to be used
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod read;
pub mod span;
/// Stream wrapper allowing custom state to be used.
pub mod state;

#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
pub mod decoder;

/// A type which has a position.
pub trait Positioned: StreamOnce {
    /// Returns the current position of the stream.
    fn position(&self) -> Self::Position;
}

/// Convenience alias over the `StreamError` for the input stream `Input`
///
/// ```
/// #[macro_use]
/// extern crate combine;
/// use combine::{easy, Parser, Stream, many1};
/// use combine::parser::char::letter;
/// use combine::stream::StreamErrorFor;
/// use combine::error::{ParseError, StreamError};
///
/// parser!{
///    fn parser[Input]()(Input) -> String
///     where [ Input: Stream<Token = char>, ]
///     {
///         many1(letter()).and_then(|word: String| {
///             if word == "combine" {
///                 Ok(word)
///             } else {
///                 // The alias makes it easy to refer to the `StreamError` type of `Input`
///                 Err(StreamErrorFor::<Input>::expected_static_message("combine"))
///             }
///         })
///     }
/// }
///
/// fn main() {
/// }
/// ```
pub type StreamErrorFor<Input> = <<Input as StreamOnce>::Error as ParseError<
    <Input as StreamOnce>::Token,
    <Input as StreamOnce>::Range,
    <Input as StreamOnce>::Position,
>>::StreamError;

/// `StreamOnce` represents a sequence of items that can be extracted one by one.
pub trait StreamOnce {
    /// The type of items which is yielded from this stream.
    type Token: Clone;

    /// The type of a range of items yielded from this stream.
    /// Types which do not a have a way of yielding ranges of items should just use the
    /// `Self::Token` for this type.
    type Range: Clone;

    /// Type which represents the position in a stream.
    /// `Ord` is required to allow parsers to determine which of two positions are further ahead.
    type Position: Clone + Ord;

    type Error: ParseError<Self::Token, Self::Range, Self::Position>;
    /// Takes a stream and removes its first token, yielding the token and the rest of the elements.
    /// Returns `Err` if no element could be retrieved.
    fn uncons(&mut self) -> Result<Self::Token, StreamErrorFor<Self>>;

    /// Returns `true` if this stream only contains partial input.
    ///
    /// See `PartialStream`.
    fn is_partial(&self) -> bool {
        false
    }
}

/// A `StreamOnce` which can create checkpoints which the stream can be reset to
pub trait ResetStream: StreamOnce {
    type Checkpoint: Clone;

    /// Creates a `Checkpoint` at the current position which can be used to reset the stream
    /// later to the current position
    fn checkpoint(&self) -> Self::Checkpoint;
    /// Attempts to reset the stream to an earlier position.
    fn reset(&mut self, checkpoint: Self::Checkpoint) -> Result<(), Self::Error>;
}

clone_resetable! {('a) &'a str}
clone_resetable! {('a, T) &'a [T]}
clone_resetable! {('a, T) SliceStream<'a, T> }
clone_resetable! {(T: Clone) IteratorStream<T>}

/// A stream of tokens which can be duplicated
///
/// This is a trait over types which implement the `StreamOnce`, `ResetStream` and `Positioned`
/// traits. If you need a custom `Stream` object then implement those traits and `Stream` is
/// implemented automatically.
pub trait Stream: StreamOnce + ResetStream + Positioned {}

impl<Input> Stream for Input
where
    Input: StreamOnce + Positioned + ResetStream,
{
}

#[inline]
pub fn uncons<Input>(input: &mut Input) -> ParseResult<Input::Token, Input::Error>
where
    Input: ?Sized + Stream,
{
    match input.uncons() {
        Ok(x) => CommitOk(x),
        Err(err) => wrap_stream_error(input, err),
    }
}

/// A `RangeStream` is an extension of `StreamOnce` which allows for zero copy parsing.
pub trait RangeStreamOnce: StreamOnce + ResetStream {
    /// Takes `size` elements from the stream.
    /// Fails if the length of the stream is less than `size`.
    fn uncons_range(&mut self, size: usize) -> Result<Self::Range, StreamErrorFor<Self>>;

    /// Takes items from stream, testing each one with `predicate`.
    /// returns the range of items which passed `predicate`.
    fn uncons_while<F>(&mut self, f: F) -> Result<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool;

    #[inline]
    /// Takes items from stream, testing each one with `predicate`
    /// returns a range of at least one items which passed `predicate`.
    ///
    /// # Note
    ///
    /// This may not return `PeekOk` as it should uncons at least one token.
    fn uncons_while1<F>(&mut self, mut f: F) -> ParseResult<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        let mut committed = false;
        let mut started_at_eoi = true;
        let result = self.uncons_while(|c| {
            let ok = f(c);
            committed |= ok;
            started_at_eoi = false;
            ok
        });
        if committed {
            match result {
                Ok(x) => CommitOk(x),
                Err(x) => CommitErr(x),
            }
        } else if started_at_eoi {
            PeekErr(Tracked::from(StreamErrorFor::<Self>::end_of_input()))
        } else {
            PeekErr(Tracked::from(
                StreamErrorFor::<Self>::unexpected_static_message(""),
            ))
        }
    }

    /// Returns the distance between `self` and `end`. The returned `usize` must be so that
    ///
    /// ```ignore
    /// let start = stream.checkpoint();
    /// stream.uncons_range(distance);
    /// stream.distance(&start) == distance
    /// ```
    fn distance(&self, end: &Self::Checkpoint) -> usize;

    /// Returns the entire range of `self`
    fn range(&self) -> Self::Range;
}

/// A `RangeStream` is an extension of `Stream` which allows for zero copy parsing.
pub trait RangeStream: Stream + RangeStreamOnce {}

impl<Input> RangeStream for Input where Input: RangeStreamOnce + Stream {}

#[doc(hidden)]
pub fn wrap_stream_error<T, Input>(
    input: &Input,
    err: <Input::Error as ParseError<Input::Token, Input::Range, Input::Position>>::StreamError,
) -> ParseResult<T, <Input as StreamOnce>::Error>
where
    Input: ?Sized + StreamOnce + Positioned,
{
    let err = Input::Error::from_error(input.position(), err);
    if input.is_partial() {
        CommitErr(err)
    } else {
        PeekErr(err.into())
    }
}

#[inline]
pub fn uncons_range<Input>(
    input: &mut Input,
    size: usize,
) -> ParseResult<Input::Range, <Input as StreamOnce>::Error>
where
    Input: ?Sized + RangeStream,
{
    match input.uncons_range(size) {
        Err(err) => wrap_stream_error(input, err),
        Ok(x) => {
            if size == 0 {
                PeekOk(x)
            } else {
                CommitOk(x)
            }
        }
    }
}

#[doc(hidden)]
pub fn input_at_eof<Input>(input: &mut Input) -> bool
where
    Input: ?Sized + Stream,
{
    let before = input.checkpoint();
    let x = input
        .uncons()
        .err()
        .map_or(false, |err| err.is_unexpected_end_of_input());
    input.reset(before).is_ok() && x
}

/// Removes items from the input while `predicate` returns `true`.
#[inline]
pub fn uncons_while<Input, F>(
    input: &mut Input,
    predicate: F,
) -> ParseResult<Input::Range, Input::Error>
where
    F: FnMut(Input::Token) -> bool,
    Input: ?Sized + RangeStream,
    Input::Range: Range,
{
    match input.uncons_while(predicate) {
        Err(err) => wrap_stream_error(input, err),
        Ok(x) => {
            if input.is_partial() && input_at_eof(input) {
                // Partial inputs which encounter end of file must fail to let more input be
                // retrieved
                CommitErr(Input::Error::from_error(
                    input.position(),
                    StreamError::end_of_input(),
                ))
            } else if x.len() == 0 {
                PeekOk(x)
            } else {
                CommitOk(x)
            }
        }
    }
}

#[inline]
/// Takes items from stream, testing each one with `predicate`
/// returns a range of at least one items which passed `predicate`.
///
/// # Note
///
/// This may not return `PeekOk` as it should uncons at least one token.
pub fn uncons_while1<Input, F>(
    input: &mut Input,
    predicate: F,
) -> ParseResult<Input::Range, Input::Error>
where
    F: FnMut(Input::Token) -> bool,
    Input: ?Sized + RangeStream,
{
    match input.uncons_while1(predicate) {
        CommitOk(x) => {
            if input.is_partial() && input_at_eof(input) {
                // Partial inputs which encounter end of file must fail to let more input be
                // retrieved
                CommitErr(Input::Error::from_error(
                    input.position(),
                    StreamError::end_of_input(),
                ))
            } else {
                CommitOk(x)
            }
        }
        PeekErr(_) => {
            if input.is_partial() && input_at_eof(input) {
                // Partial inputs which encounter end of file must fail to let more input be
                // retrieved
                CommitErr(Input::Error::from_error(
                    input.position(),
                    StreamError::end_of_input(),
                ))
            } else {
                PeekErr(Input::Error::empty(input.position()).into())
            }
        }
        CommitErr(err) => {
            if input.is_partial() && input_at_eof(input) {
                // Partial inputs which encounter end of file must fail to let more input be
                // retrieved
                CommitErr(Input::Error::from_error(
                    input.position(),
                    StreamError::end_of_input(),
                ))
            } else {
                wrap_stream_error(input, err)
            }
        }
        PeekOk(_) => unreachable!(),
    }
}

/// Trait representing a range of elements.
pub trait Range {
    /// Returns the remaining length of `self`.
    /// The returned length need not be the same as the number of items left in the stream.
    fn len(&self) -> usize;

    /// Returns `true` if the range does not contain any elements (`Range::len() == 0`)
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
}

impl<'a, I> StreamOnce for &'a mut I
where
    I: StreamOnce + ?Sized,
{
    type Token = I::Token;

    type Range = I::Range;

    type Position = I::Position;

    type Error = I::Error;
    fn uncons(&mut self) -> Result<Self::Token, StreamErrorFor<Self>> {
        (**self).uncons()
    }

    fn is_partial(&self) -> bool {
        (**self).is_partial()
    }
}

impl<'a, I> Positioned for &'a mut I
where
    I: Positioned + ?Sized,
{
    #[inline]
    fn position(&self) -> Self::Position {
        (**self).position()
    }
}

impl<'a, I> ResetStream for &'a mut I
where
    I: ResetStream + ?Sized,
{
    type Checkpoint = I::Checkpoint;

    fn checkpoint(&self) -> Self::Checkpoint {
        (**self).checkpoint()
    }

    fn reset(&mut self, checkpoint: Self::Checkpoint) -> Result<(), Self::Error> {
        (**self).reset(checkpoint)
    }
}

impl<'a, I> RangeStreamOnce for &'a mut I
where
    I: RangeStreamOnce + ?Sized,
{
    #[inline]
    fn uncons_while<F>(&mut self, f: F) -> Result<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        (**self).uncons_while(f)
    }

    #[inline]
    fn uncons_while1<F>(&mut self, f: F) -> ParseResult<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        (**self).uncons_while1(f)
    }

    #[inline]
    fn uncons_range(&mut self, size: usize) -> Result<Self::Range, StreamErrorFor<Self>> {
        (**self).uncons_range(size)
    }

    #[inline]
    fn distance(&self, end: &Self::Checkpoint) -> usize {
        (**self).distance(end)
    }

    fn range(&self) -> Self::Range {
        (**self).range()
    }
}

impl<'a, I> Range for &'a mut I
where
    I: Range + ?Sized,
{
    fn len(&self) -> usize {
        (**self).len()
    }
}

impl<'a> StreamOnce for &'a str {
    type Token = char;
    type Range = &'a str;
    type Position = PointerOffset<str>;
    type Error = StringStreamError;

    #[inline]
    fn uncons(&mut self) -> Result<char, StreamErrorFor<Self>> {
        let mut chars = self.chars();
        match chars.next() {
            Some(c) => {
                *self = chars.as_str();
                Ok(c)
            }
            None => Err(StringStreamError::Eoi),
        }
    }
}

impl<'a> Positioned for &'a str {
    #[inline]
    fn position(&self) -> Self::Position {
        PointerOffset::new(self.as_bytes().position().0)
    }
}

#[allow(clippy::while_let_loop)]
fn str_uncons_while<'a, F>(slice: &mut &'a str, mut chars: Chars<'a>, mut f: F) -> &'a str
where
    F: FnMut(char) -> bool,
{
    let mut last_char_size = 0;

    macro_rules! test_next {
        () => {
            match chars.next() {
                Some(c) => {
                    if !f(c) {
                        last_char_size = c.len_utf8();
                        break;
                    }
                }
                None => break,
            }
        };
    }
    loop {
        test_next!();
        test_next!();
        test_next!();
        test_next!();
        test_next!();
        test_next!();
        test_next!();
        test_next!();
    }

    let len = slice.len() - chars.as_str().len() - last_char_size;
    let (result, rest) = slice.split_at(len);
    *slice = rest;
    result
}

impl<'a> RangeStreamOnce for &'a str {
    fn uncons_while<F>(&mut self, f: F) -> Result<&'a str, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        Ok(str_uncons_while(self, self.chars(), f))
    }

    #[inline]
    fn uncons_while1<F>(&mut self, mut f: F) -> ParseResult<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        let mut chars = self.chars();
        match chars.next() {
            Some(c) => {
                if !f(c) {
                    return PeekErr(Tracked::from(StringStreamError::UnexpectedParse));
                }
            }
            None => return PeekErr(Tracked::from(StringStreamError::Eoi)),
        }

        CommitOk(str_uncons_while(self, chars, f))
    }

    #[inline]
    fn uncons_range(&mut self, size: usize) -> Result<&'a str, StreamErrorFor<Self>> {
        fn is_char_boundary(s: &str, index: usize) -> bool {
            if index == s.len() {
                return true;
            }
            match s.as_bytes().get(index) {
                None => false,
                Some(b) => !(128..=192).contains(b),
            }
        }
        if size <= self.len() {
            if is_char_boundary(self, size) {
                let (result, remaining) = self.split_at(size);
                *self = remaining;
                Ok(result)
            } else {
                Err(StringStreamError::CharacterBoundary)
            }
        } else {
            Err(StringStreamError::Eoi)
        }
    }

    #[inline]
    fn distance(&self, end: &Self) -> usize {
        self.position().0 - end.position().0
    }

    fn range(&self) -> Self::Range {
        self
    }
}

impl<'a> Range for &'a str {
    #[inline]
    fn len(&self) -> usize {
        str::len(self)
    }
}

impl<'a, T> Range for &'a [T] {
    #[inline]
    fn len(&self) -> usize {
        <[T]>::len(self)
    }
}

#[repr(usize)]
enum UnconsStart {
    Zero = 0,
    One = 1,
}

fn slice_uncons_while<'a, T, F>(slice: &mut &'a [T], start: UnconsStart, mut f: F) -> &'a [T]
where
    F: FnMut(T) -> bool,
    T: Clone,
{
    let mut i = start as usize;
    let len = slice.len();
    // SAFETY: We only call this function with `One` if the slice has length >= 1
    debug_assert!(len >= i, "");
    let mut found = false;

    macro_rules! check {
        () => {
            if !f(unsafe { slice.get_unchecked(i).clone() }) {
                found = true;
                break;
            }
            i += 1;
        };
    }

    // SAFETY: ensures we can access at least 8 elements starting at i, making get_unchecked sound.
    while len - i >= 8 {
        check!();
        check!();
        check!();
        check!();
        check!();
        check!();
        check!();
        check!();
    }

    if !found {
        while let Some(c) = slice.get(i) {
            if !f(c.clone()) {
                break;
            }
            i += 1;
        }
    }

    let (result, remaining) = slice.split_at(i);
    *slice = remaining;
    result
}

impl<'a, T> RangeStreamOnce for &'a [T]
where
    T: Clone + PartialEq,
{
    #[inline]
    fn uncons_range(&mut self, size: usize) -> Result<&'a [T], StreamErrorFor<Self>> {
        if size <= self.len() {
            let (result, remaining) = self.split_at(size);
            *self = remaining;
            Ok(result)
        } else {
            Err(UnexpectedParse::Eoi)
        }
    }

    #[inline]
    fn uncons_while<F>(&mut self, f: F) -> Result<&'a [T], StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        Ok(slice_uncons_while(self, UnconsStart::Zero, f))
    }

    #[inline]
    fn uncons_while1<F>(&mut self, mut f: F) -> ParseResult<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        match self.first() {
            Some(c) => {
                if !f(c.clone()) {
                    return PeekErr(Tracked::from(UnexpectedParse::Unexpected));
                }
            }
            None => {
                return PeekErr(Tracked::from(UnexpectedParse::Eoi));
            }
        }

        CommitOk(slice_uncons_while(self, UnconsStart::One, f))
    }

    #[inline]
    fn distance(&self, end: &Self) -> usize {
        end.len() - self.len()
    }

    fn range(&self) -> Self::Range {
        self
    }
}

impl<'a, T> Positioned for &'a [T]
where
    T: Clone + PartialEq,
{
    #[inline]
    fn position(&self) -> Self::Position {
        PointerOffset::new(self.as_ptr() as usize)
    }
}

impl<'a, T> StreamOnce for &'a [T]
where
    T: Clone + PartialEq,
{
    type Token = T;
    type Range = &'a [T];
    type Position = PointerOffset<[T]>;
    type Error = UnexpectedParse;

    #[inline]
    fn uncons(&mut self) -> Result<T, StreamErrorFor<Self>> {
        match self.split_first() {
            Some((first, rest)) => {
                *self = rest;
                Ok(first.clone())
            }
            None => Err(UnexpectedParse::Eoi),
        }
    }
}

/// Stream type which indicates that the stream is partial if end of input is reached
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct PartialStream<S>(pub S);

impl<S> From<S> for PartialStream<S> {
    fn from(t: S) -> Self {
        PartialStream(t)
    }
}

impl<S> Positioned for PartialStream<S>
where
    S: Positioned,
{
    #[inline]
    fn position(&self) -> Self::Position {
        self.0.position()
    }
}

impl<S> ResetStream for PartialStream<S>
where
    S: ResetStream,
{
    type Checkpoint = S::Checkpoint;

    #[inline]
    fn checkpoint(&self) -> Self::Checkpoint {
        self.0.checkpoint()
    }

    #[inline]
    fn reset(&mut self, checkpoint: Self::Checkpoint) -> Result<(), S::Error> {
        self.0.reset(checkpoint)
    }
}

impl<S> StreamOnce for PartialStream<S>
where
    S: StreamOnce,
{
    type Token = S::Token;
    type Range = S::Range;
    type Position = S::Position;
    type Error = S::Error;

    #[inline]
    fn uncons(&mut self) -> Result<S::Token, StreamErrorFor<Self>> {
        self.0.uncons()
    }

    fn is_partial(&self) -> bool {
        true
    }
}

impl<S> RangeStreamOnce for PartialStream<S>
where
    S: RangeStreamOnce,
{
    #[inline]
    fn uncons_range(&mut self, size: usize) -> Result<Self::Range, StreamErrorFor<Self>> {
        self.0.uncons_range(size)
    }

    #[inline]
    fn uncons_while<F>(&mut self, f: F) -> Result<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        self.0.uncons_while(f)
    }

    fn uncons_while1<F>(&mut self, f: F) -> ParseResult<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        self.0.uncons_while1(f)
    }

    #[inline]
    fn distance(&self, end: &Self::Checkpoint) -> usize {
        self.0.distance(end)
    }

    #[inline]
    fn range(&self) -> Self::Range {
        self.0.range()
    }
}

/// Stream type which indicates that the stream is complete if end of input is reached
///
/// For most streams this is already the default but this wrapper can be used to override a nested
/// `PartialStream`
#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
#[repr(transparent)]
pub struct CompleteStream<S>(pub S);

impl<S> From<S> for CompleteStream<S> {
    fn from(t: S) -> Self {
        CompleteStream(t)
    }
}

impl<'s, S> From<&'s mut S> for &'s mut CompleteStream<S> {
    fn from(t: &'s mut S) -> Self {
        // SAFETY repr(transparent) is specified on CompleteStream
        unsafe { &mut *(t as *mut S as *mut CompleteStream<S>) }
    }
}

impl<S> Positioned for CompleteStream<S>
where
    S: Positioned,
{
    #[inline]
    fn position(&self) -> Self::Position {
        self.0.position()
    }
}

impl<S> ResetStream for CompleteStream<S>
where
    S: ResetStream,
{
    type Checkpoint = S::Checkpoint;

    #[inline]
    fn checkpoint(&self) -> Self::Checkpoint {
        self.0.checkpoint()
    }

    #[inline]
    fn reset(&mut self, checkpoint: Self::Checkpoint) -> Result<(), S::Error> {
        self.0.reset(checkpoint)
    }
}

impl<S> StreamOnce for CompleteStream<S>
where
    S: StreamOnce,
{
    type Token = S::Token;
    type Range = S::Range;
    type Position = S::Position;
    type Error = S::Error;

    #[inline]
    fn uncons(&mut self) -> Result<S::Token, StreamErrorFor<Self>> {
        self.0.uncons()
    }

    fn is_partial(&self) -> bool {
        false
    }
}

impl<S> RangeStreamOnce for CompleteStream<S>
where
    S: RangeStreamOnce,
{
    #[inline]
    fn uncons_range(&mut self, size: usize) -> Result<Self::Range, StreamErrorFor<Self>> {
        self.0.uncons_range(size)
    }

    #[inline]
    fn uncons_while<F>(&mut self, f: F) -> Result<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        self.0.uncons_while(f)
    }

    fn uncons_while1<F>(&mut self, f: F) -> ParseResult<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        self.0.uncons_while1(f)
    }

    #[inline]
    fn distance(&self, end: &Self::Checkpoint) -> usize {
        self.0.distance(end)
    }

    #[inline]
    fn range(&self) -> Self::Range {
        self.0.range()
    }
}

#[derive(Copy, Clone, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct MaybePartialStream<S>(pub S, pub bool);

impl<S> Positioned for MaybePartialStream<S>
where
    S: Positioned,
{
    #[inline]
    fn position(&self) -> Self::Position {
        self.0.position()
    }
}

impl<S> ResetStream for MaybePartialStream<S>
where
    S: ResetStream,
{
    type Checkpoint = S::Checkpoint;

    #[inline]
    fn checkpoint(&self) -> Self::Checkpoint {
        self.0.checkpoint()
    }

    #[inline]
    fn reset(&mut self, checkpoint: Self::Checkpoint) -> Result<(), S::Error> {
        self.0.reset(checkpoint)
    }
}

impl<S> StreamOnce for MaybePartialStream<S>
where
    S: StreamOnce,
{
    type Token = S::Token;
    type Range = S::Range;
    type Position = S::Position;
    type Error = S::Error;

    #[inline]
    fn uncons(&mut self) -> Result<S::Token, StreamErrorFor<Self>> {
        self.0.uncons()
    }

    fn is_partial(&self) -> bool {
        self.1
    }
}

impl<S> RangeStreamOnce for MaybePartialStream<S>
where
    S: RangeStreamOnce,
{
    #[inline]
    fn uncons_range(&mut self, size: usize) -> Result<Self::Range, StreamErrorFor<Self>> {
        self.0.uncons_range(size)
    }

    #[inline]
    fn uncons_while<F>(&mut self, f: F) -> Result<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        self.0.uncons_while(f)
    }

    fn uncons_while1<F>(&mut self, f: F) -> ParseResult<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        self.0.uncons_while1(f)
    }

    #[inline]
    fn distance(&self, end: &Self::Checkpoint) -> usize {
        self.0.distance(end)
    }

    #[inline]
    fn range(&self) -> Self::Range {
        self.0.range()
    }
}

/// Newtype for constructing a stream from a slice where the items in the slice are not copyable.
#[derive(Copy, Eq, PartialEq, Ord, PartialOrd, Debug)]
pub struct SliceStream<'a, T>(pub &'a [T]);

impl<'a, T> Clone for SliceStream<'a, T> {
    fn clone(&self) -> SliceStream<'a, T> {
        SliceStream(self.0)
    }
}

impl<'a, T> Positioned for SliceStream<'a, T>
where
    T: PartialEq + 'a,
{
    #[inline]
    fn position(&self) -> Self::Position {
        PointerOffset::new(self.0.as_ptr() as usize)
    }
}

impl<'a, T> StreamOnce for SliceStream<'a, T>
where
    T: PartialEq + 'a,
{
    type Token = &'a T;
    type Range = &'a [T];
    type Position = PointerOffset<[T]>;
    type Error = UnexpectedParse;

    #[inline]
    fn uncons(&mut self) -> Result<&'a T, StreamErrorFor<Self>> {
        match self.0.split_first() {
            Some((first, rest)) => {
                self.0 = rest;
                Ok(first)
            }
            None => Err(UnexpectedParse::Eoi),
        }
    }
}

fn slice_uncons_while_ref<'a, T, F>(slice: &mut &'a [T], start: UnconsStart, mut f: F) -> &'a [T]
where
    F: FnMut(&'a T) -> bool,
{
    let mut i = start as usize;
    let len = slice.len();
    // SAFETY: We only call this function with `One` if the slice has length >= 1
    debug_assert!(len >= i, "");
    let mut found = false;

    macro_rules! check {
        () => {
            if !f(unsafe { slice.get_unchecked(i) }) {
                found = true;
                break;
            }
            i += 1;
        };
    }

    // SAFETY: ensures we can access at least 8 elements starting at i, making get_unchecked sound.
    while len - i >= 8 {
        check!();
        check!();
        check!();
        check!();
        check!();
        check!();
        check!();
        check!();
    }

    if !found {
        while let Some(c) = slice.get(i) {
            if !f(c) {
                break;
            }
            i += 1;
        }
    }

    let (result, remaining) = slice.split_at(i);
    *slice = remaining;
    result
}

impl<'a, T> RangeStreamOnce for SliceStream<'a, T>
where
    T: PartialEq + 'a,
{
    #[inline]
    fn uncons_range(&mut self, size: usize) -> Result<&'a [T], StreamErrorFor<Self>> {
        if size <= self.0.len() {
            let (range, rest) = self.0.split_at(size);
            self.0 = rest;
            Ok(range)
        } else {
            Err(UnexpectedParse::Eoi)
        }
    }

    #[inline]
    fn uncons_while<F>(&mut self, f: F) -> Result<&'a [T], StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        Ok(slice_uncons_while_ref(&mut self.0, UnconsStart::Zero, f))
    }

    #[inline]
    fn uncons_while1<F>(&mut self, mut f: F) -> ParseResult<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        match self.0.first() {
            Some(c) => {
                if !f(c) {
                    return PeekErr(Tracked::from(UnexpectedParse::Unexpected));
                }
            }
            None => return PeekErr(Tracked::from(UnexpectedParse::Eoi)),
        }

        CommitOk(slice_uncons_while_ref(&mut self.0, UnconsStart::One, f))
    }

    #[inline]
    fn distance(&self, end: &Self) -> usize {
        end.0.len() - self.0.len()
    }

    fn range(&self) -> Self::Range {
        self.0
    }
}

/// Wrapper around iterators which allows them to be treated as a stream.
/// Returned by [`IteratorStream::new`].
#[derive(Copy, Clone, Debug)]
pub struct IteratorStream<Input>(Input);

impl<Input> IteratorStream<Input>
where
    Input: Iterator,
{
    /// Converts an `Iterator` into a stream.
    ///
    /// NOTE: This type do not implement `Positioned` and `Clone` and must be wrapped with types
    ///     such as `BufferedStreamRef` and `State` to become a `Stream` which can be parsed
    pub fn new<T>(iter: T) -> IteratorStream<Input>
    where
        T: IntoIterator<IntoIter = Input, Item = Input::Item>,
    {
        IteratorStream(iter.into_iter())
    }
}

impl<Input> Iterator for IteratorStream<Input>
where
    Input: Iterator,
{
    type Item = Input::Item;
    fn next(&mut self) -> Option<Input::Item> {
        self.0.next()
    }
}

impl<Input: Iterator> StreamOnce for IteratorStream<Input>
where
    Input::Item: Clone + PartialEq,
{
    type Token = Input::Item;
    type Range = Input::Item;
    type Position = ();
    type Error = UnexpectedParse;

    #[inline]
    fn uncons(&mut self) -> Result<Self::Token, StreamErrorFor<Self>> {
        match self.next() {
            Some(x) => Ok(x),
            None => Err(UnexpectedParse::Eoi),
        }
    }
}

/// Newtype around a pointer offset into a slice stream (`&[T]`/`&str`).
pub struct PointerOffset<T: ?Sized>(pub usize, PhantomData<T>);

impl<T: ?Sized> Clone for PointerOffset<T> {
    fn clone(&self) -> Self {
        PointerOffset::new(self.0)
    }
}

impl<T: ?Sized> Copy for PointerOffset<T> {}

impl<T: ?Sized> Default for PointerOffset<T> {
    fn default() -> Self {
        PointerOffset::new(0)
    }
}

impl<T: ?Sized> PartialEq for PointerOffset<T> {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}

impl<T: ?Sized> Eq for PointerOffset<T> {}

impl<T: ?Sized> PartialOrd for PointerOffset<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T: ?Sized> Ord for PointerOffset<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.0.cmp(&other.0)
    }
}

impl<T> fmt::Debug for PointerOffset<T>
where
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

impl<T> fmt::Display for PointerOffset<T>
where
    T: ?Sized,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "PointerOffset({:?})", self.0 as *const ())
    }
}

impl<T> PointerOffset<T>
where
    T: ?Sized,
{
    pub fn new(offset: usize) -> Self {
        PointerOffset(offset, PhantomData)
    }

    /// Converts the pointer-based position into an indexed position.
    ///
    /// ```rust
    /// # extern crate combine;
    /// # use combine::*;
    /// # fn main() {
    /// let text = "b";
    /// let err = token('a').easy_parse(text).unwrap_err();
    /// assert_eq!(err.position.0, text.as_ptr() as usize);
    /// assert_eq!(err.map_position(|p| p.translate_position(text)).position, 0);
    /// # }
    /// ```
    pub fn translate_position(mut self, initial_slice: &T) -> usize {
        self.0 -= initial_slice as *const T as *const () as usize;
        self.0
    }
}

/// Decodes `input` using `parser`.
///
/// Return `Ok(Some(token), committed_data)` if there was enough data to finish parsing using
/// `parser`.
/// Returns `Ok(None, committed_data)` if `input` did not contain enough data to finish parsing
/// using `parser`.
///
/// See `examples/async.rs` for example usage in a `tokio_io::codec::Decoder`
pub fn decode<Input, P>(
    mut parser: P,
    input: &mut Input,
    partial_state: &mut P::PartialState,
) -> Result<(Option<P::Output>, usize), <Input as StreamOnce>::Error>
where
    P: Parser<Input>,
    Input: RangeStream,
{
    let start = input.checkpoint();
    match parser.parse_with_state(input, partial_state) {
        Ok(message) => Ok((Some(message), input.distance(&start))),
        Err(err) => {
            if err.is_unexpected_end_of_input() {
                if input.is_partial() {
                    // The parser expected more input to parse and input is partial, return `None`
                    // as we did not finish and also return how much may be removed from the stream
                    Ok((None, input.distance(&start)))
                } else {
                    Err(err)
                }
            } else {
                Err(err)
            }
        }
    }
}

/// Decodes `input` using `parser`. Like `decode` but works directly in both
/// `tokio_util::Decoder::decode` and `tokio_util::Decoder::decode_eof`
///
/// Return `Ok(Some(token), committed_data)` if there was enough data to finish parsing using
/// `parser`.
/// Returns `Ok(None, committed_data)` if `input` did not contain enough data to finish parsing
/// using `parser`.
/// Returns `Ok(None, 0)` if `input` did not contain enough data to finish parsing
/// using `parser`.
///
/// See `examples/async.rs` for example usage in a `tokio_io::codec::Decoder`
pub fn decode_tokio<Input, P>(
    mut parser: P,
    input: &mut Input,
    partial_state: &mut P::PartialState,
) -> Result<(Option<P::Output>, usize), <Input as StreamOnce>::Error>
where
    P: Parser<Input>,
    Input: RangeStream,
{
    let start = input.checkpoint();
    match parser.parse_with_state(input, partial_state) {
        Ok(message) => Ok((Some(message), input.distance(&start))),
        Err(err) => {
            if err.is_unexpected_end_of_input() {
                if input.is_partial() {
                    // The parser expected more input to parse and input is partial, return `None`
                    // as we did not finish and also return how much may be removed from the stream
                    Ok((None, input.distance(&start)))
                } else if input_at_eof(input) && input.distance(&start) == 0 {
                    // We are at eof and the input is empty, return None to indicate that we are
                    // done
                    Ok((None, 0))
                } else {
                    Err(err)
                }
            } else {
                Err(err)
            }
        }
    }
}

/// Parses an instance of `std::io::Read` as a `&[u8]` without reading the entire file into
/// memory.
///
/// This is defined as a macro to work around the lack of Higher Ranked Types. See the
/// example for how to pass a parser to the macro (constructing parts of the parser outside of
/// the `decode!` call is unlikely to work.
///
/// ```
/// use std::{
///     fs::File,
/// };
/// use combine::{decode, satisfy, skip_many1, many1, sep_end_by, Parser, stream::Decoder};
///
/// let mut read = File::open("README.md").unwrap();
/// let mut decoder = Decoder::new();
/// let is_whitespace = |b: u8| b == b' ' || b == b'\r' || b == b'\n';
/// assert_eq!(
///     decode!(
///         decoder,
///         read,
///         {
///             let word = many1(satisfy(|b| !is_whitespace(b)));
///             sep_end_by(word, skip_many1(satisfy(is_whitespace))).map(|words: Vec<Vec<u8>>| words.len())
///         },
///         |input, _position| combine::easy::Stream::from(input),
///     ).map_err(combine::easy::Errors::<u8, &[u8], _>::from),
///     Ok(773),
/// );
/// ```
#[cfg(feature = "std")]
#[cfg_attr(docsrs, doc(cfg(feature = "std")))]
#[macro_export]
macro_rules! decode {
    ($decoder: expr, $read: expr, $parser: expr $(,)?) => {
        $crate::decode!($decoder, $read, $parser, |input, _position| input, |x| x)
    };

    ($decoder: expr, $read: expr, $parser: expr, $input_stream: expr $(,)?) => {
        $crate::decode!($decoder, $read, $parser, $input_stream, |x| x)
    };

    ($decoder: expr, $read: expr, $parser: expr, $input_stream: expr, $post_decode: expr $(,)?) => {
        match $decoder {
            ref mut decoder => match $read {
                ref mut read => 'outer: loop {
                    let (opt, removed) = {
                        let (state, position, buffer, end_of_input) = decoder.__inner();
                        let buffer =
                            $crate::stream::buf_reader::CombineBuffer::buffer(buffer, read);

                        let mut stream = $crate::stream::call_with2(
                            $crate::stream::MaybePartialStream(buffer, !end_of_input),
                            *position,
                            $input_stream,
                        );
                        let result = $crate::stream::decode($parser, &mut stream, state);
                        *position = $crate::stream::Positioned::position(&stream);
                        $crate::stream::call_with(stream, $post_decode);
                        match result {
                            Ok(x) => x,
                            Err(err) => {
                                break 'outer Err($crate::stream::decoder::Error::Parse(err))
                            }
                        }
                    };

                    decoder.advance(&mut *read, removed);

                    if let Some(v) = opt {
                        break 'outer Ok(v);
                    }

                    match decoder.__before_parse(&mut *read) {
                        Ok(x) => x,
                        Err(error) => {
                            break 'outer Err($crate::stream::decoder::Error::Io {
                                error,
                                position: Clone::clone(decoder.position()),
                            })
                        }
                    };
                },
            },
        }
    };
}

/// Parses an instance of `futures::io::AsyncRead` as a `&[u8]` without reading the entire file into
/// memory.
///
/// This is defined as a macro to work around the lack of Higher Ranked Types. See the
/// example for how to pass a parser to the macro (constructing parts of the parser outside of
/// the `decode!` call is unlikely to work.
///
/// ```
/// # use futures_03_dep as futures;
/// use futures::pin_mut;
/// use async_std::{
///     fs::File,
///     task,
/// };
///
/// use combine::{decode_futures_03, satisfy, skip_many1, many1, sep_end_by, Parser, stream::Decoder};
///
/// fn main() {
///     task::block_on(main_());
/// }
///
/// async fn main_() {
///     let mut read = File::open("README.md").await.unwrap();
///     let mut decoder = Decoder::new();
///     let is_whitespace = |b: u8| b == b' ' || b == b'\r' || b == b'\n';
///     assert_eq!(
///         decode_futures_03!(
///             decoder,
///             read,
///             {
///                 let word = many1(satisfy(|b| !is_whitespace(b)));
///                 sep_end_by(word, skip_many1(satisfy(is_whitespace))).map(|words: Vec<Vec<u8>>| words.len())
///             },
///             |input, _position| combine::easy::Stream::from(input),
///         ).map_err(combine::easy::Errors::<u8, &[u8], _>::from),
///         Ok(773),
///     );
/// }
/// ```
#[cfg(feature = "futures-io-03")]
#[cfg_attr(docsrs, doc(cfg(feature = "futures-io-03")))]
#[macro_export]
macro_rules! decode_futures_03 {
    ($decoder: expr, $read: expr, $parser: expr) => {
        $crate::decode_futures_03!($decoder, $read, $parser, |x| x $(,)?)
    };


    ($decoder: expr, $read: expr, $parser: expr, $input_stream: expr $(,)?) => {
        $crate::decode_futures_03!($decoder, $read, $parser, $input_stream, |x| x)
    };

    ($decoder: expr, $read: expr, $parser: expr, $input_stream: expr, $post_decode: expr $(,)?) => {
        match $decoder {
            ref mut decoder => match $read {
                ref mut read => 'outer: loop {
                    let (opt, removed) = {
                        let (state, position, buffer, end_of_input) = decoder.__inner();
                        let buffer =
                            $crate::stream::buf_reader::CombineBuffer::buffer(buffer, &*read);

                        let mut stream = $crate::stream::call_with2(
                            $crate::stream::MaybePartialStream(buffer, !end_of_input),
                            *position,
                            $input_stream,
                        );
                        let result = $crate::stream::decode($parser, &mut stream, state);
                        *position = $crate::stream::Positioned::position(&stream);
                        $crate::stream::call_with(stream, $post_decode);
                        match result {
                            Ok(x) => x,
                            Err(err) => break 'outer Err($crate::stream::decoder::Error::Parse(err)),
                        }
                    };

                    decoder.advance_pin(std::pin::Pin::new(&mut *read), removed);

                    if let Some(v) = opt {
                        break 'outer Ok(v);
                    }


                    match decoder.__before_parse_async(std::pin::Pin::new(&mut *read)).await {
                        Ok(_) => (),
                        Err(error) => {
                            break 'outer Err($crate::stream::decoder::Error::Io {
                                error,
                                position: Clone::clone(decoder.position()),
                            })
                        }
                    };
                }
            }
        }
    };
}

/// Parses an instance of `tokio::io::AsyncRead` as a `&[u8]` without reading the entire file into
/// memory.
///
/// This is defined as a macro to work around the lack of Higher Ranked Types. See the
/// example for how to pass a parser to the macro (constructing parts of the parser outside of
/// the `decode!` call is unlikely to work.
///
/// ```
/// # use tokio_02_dep as tokio;
/// # use futures_03_dep as futures;
/// use futures::pin_mut;
/// use tokio::{
///     fs::File,
/// };
///
/// use combine::{decode_tokio_02, satisfy, skip_many1, many1, sep_end_by, Parser, stream::{Decoder, buf_reader::BufReader}};
///
/// #[tokio::main]
/// async fn main() {
///     let mut read = BufReader::new(File::open("README.md").await.unwrap());
///     let mut decoder = Decoder::new_bufferless();
///     let is_whitespace = |b: u8| b == b' ' || b == b'\r' || b == b'\n';
///     assert_eq!(
///         decode_tokio_02!(
///             decoder,
///             read,
///             {
///                 let word = many1(satisfy(|b| !is_whitespace(b)));
///                 sep_end_by(word, skip_many1(satisfy(is_whitespace))).map(|words: Vec<Vec<u8>>| words.len())
///             },
///             |input, _position| combine::easy::Stream::from(input),
///         ).map_err(combine::easy::Errors::<u8, &[u8], _>::from),
///         Ok(773),
///     );
/// }
/// ```
#[cfg(feature = "tokio-02")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-02")))]
#[macro_export]
macro_rules! decode_tokio_02 {
    ($decoder: expr, $read: expr, $parser: expr $(,)?) => {
        $crate::decode_tokio_02!($decoder, $read, $parser, |input, _position| input)
    };

    ($decoder: expr, $read: expr, $parser: expr, $input_stream: expr $(,)?) => {
        $crate::decode_tokio_02!($decoder, $read, $parser, $input_stream, |x| x)
    };

    ($decoder: expr, $read: expr, $parser: expr, $input_stream: expr, $post_decode: expr $(,)?) => {
        match $decoder {
            ref mut decoder => match $read {
                ref mut read => 'outer: loop {
                    let (opt, removed) = {
                        let (state, position, buffer, end_of_input) = decoder.__inner();
                        let buffer =
                            $crate::stream::buf_reader::CombineBuffer::buffer(buffer, &*read);
                        let mut stream = $crate::stream::call_with2(
                            $crate::stream::MaybePartialStream(buffer, !end_of_input),
                            *position,
                            $input_stream,
                        );
                        let result = $crate::stream::decode($parser, &mut stream, state);
                        *position = $crate::stream::Positioned::position(&stream);
                        $crate::stream::call_with(stream, $post_decode);
                        match result {
                            Ok(x) => x,
                            Err(err) => {
                                break 'outer Err($crate::stream::decoder::Error::Parse(err))
                            }
                        }
                    };

                    decoder.advance_pin(std::pin::Pin::new(read), removed);

                    if let Some(v) = opt {
                        break 'outer Ok(v);
                    }

                    match decoder
                        .__before_parse_tokio_02(std::pin::Pin::new(&mut *read))
                        .await
                    {
                        Ok(x) => x,
                        Err(error) => {
                            break 'outer Err($crate::stream::decoder::Error::Io {
                                error,
                                position: Clone::clone(decoder.position()),
                            })
                        }
                    };
                },
            },
        }
    };
}

/// Parses an instance of `tokio::io::AsyncRead` as a `&[u8]` without reading the entire file into
/// memory.
///
/// This is defined as a macro to work around the lack of Higher Ranked Types. See the
/// example for how to pass a parser to the macro (constructing parts of the parser outside of
/// the `decode!` call is unlikely to work.
///
/// ```
/// # use tokio_03_dep as tokio;
/// # use futures_03_dep as futures;
/// use futures::pin_mut;
/// use tokio::{
///     fs::File,
/// };
///
/// use combine::{decode_tokio_03, satisfy, skip_many1, many1, sep_end_by, Parser, stream::{Decoder, buf_reader::BufReader}};
///
/// #[tokio::main]
/// async fn main() {
///     let mut read = BufReader::new(File::open("README.md").await.unwrap());
///     let mut decoder = Decoder::new_bufferless();
///     let is_whitespace = |b: u8| b == b' ' || b == b'\r' || b == b'\n';
///     assert_eq!(
///         decode_tokio_03!(
///             decoder,
///             read,
///             {
///                 let word = many1(satisfy(|b| !is_whitespace(b)));
///                 sep_end_by(word, skip_many1(satisfy(is_whitespace))).map(|words: Vec<Vec<u8>>| words.len())
///             },
///             |input, _position| combine::easy::Stream::from(input),
///         ).map_err(combine::easy::Errors::<u8, &[u8], _>::from),
///         Ok(773),
///     );
/// }
/// ```
#[cfg(feature = "tokio-03")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio-03")))]
#[macro_export]
macro_rules! decode_tokio_03 {
    ($decoder: expr, $read: expr, $parser: expr $(,)?) => {
        $crate::decode_tokio_03!($decoder, $read, $parser, |input, _position| input)
    };

    ($decoder: expr, $read: expr, $parser: expr, $input_stream: expr $(,)?) => {
        $crate::decode_tokio_03!($decoder, $read, $parser, $input_stream, |x| x)
    };

    ($decoder: expr, $read: expr, $parser: expr, $input_stream: expr, $post_decode: expr $(,)?) => {
        match $decoder {
            ref mut decoder => match $read {
                ref mut read => 'outer: loop {
                    let (opt, removed) = {
                        let (state, position, buffer, end_of_input) = decoder.__inner();
                        let buffer =
                            $crate::stream::buf_reader::CombineBuffer::buffer(buffer, &*read);
                        let mut stream = $crate::stream::call_with2(
                            $crate::stream::MaybePartialStream(buffer, !end_of_input),
                            *position,
                            $input_stream,
                        );
                        let result = $crate::stream::decode($parser, &mut stream, state);
                        *position = $crate::stream::Positioned::position(&stream);
                        $crate::stream::call_with(stream, $post_decode);
                        match result {
                            Ok(x) => x,
                            Err(err) => {
                                break 'outer Err($crate::stream::decoder::Error::Parse(err))
                            }
                        }
                    };

                    decoder.advance_pin(std::pin::Pin::new(read), removed);

                    if let Some(v) = opt {
                        break 'outer Ok(v);
                    }

                    match decoder
                        .__before_parse_tokio_03(std::pin::Pin::new(&mut *read))
                        .await
                    {
                        Ok(x) => x,
                        Err(error) => {
                            break 'outer Err($crate::stream::decoder::Error::Io {
                                error,
                                position: Clone::clone(decoder.position()),
                            })
                        }
                    };
                },
            },
        }
    };
}

/// Parses an instance of `tokio::io::AsyncRead` as a `&[u8]` without reading the entire file into
/// memory.
///
/// This is defined as a macro to work around the lack of Higher Ranked Types. See the
/// example for how to pass a parser to the macro (constructing parts of the parser outside of
/// the `decode!` call is unlikely to work.
///
/// ```
/// # use tokio_dep as tokio;
/// # use futures_03_dep as futures;
/// use futures::pin_mut;
/// use tokio::{
///     fs::File,
/// };
///
/// use combine::{decode_tokio, satisfy, skip_many1, many1, sep_end_by, Parser, stream::{Decoder, buf_reader::BufReader}};
///
/// #[tokio::main]
/// async fn main() {
///     let mut read = BufReader::new(File::open("README.md").await.unwrap());
///     let mut decoder = Decoder::new_bufferless();
///     let is_whitespace = |b: u8| b == b' ' || b == b'\r' || b == b'\n';
///     assert_eq!(
///         decode_tokio!(
///             decoder,
///             read,
///             {
///                 let word = many1(satisfy(|b| !is_whitespace(b)));
///                 sep_end_by(word, skip_many1(satisfy(is_whitespace))).map(|words: Vec<Vec<u8>>| words.len())
///             },
///             |input, _position| combine::easy::Stream::from(input),
///         ).map_err(combine::easy::Errors::<u8, &[u8], _>::from),
///         Ok(773),
///     );
/// }
/// ```
#[cfg(feature = "tokio")]
#[cfg_attr(docsrs, doc(cfg(feature = "tokio")))]
#[macro_export]
macro_rules! decode_tokio {
    ($decoder: expr, $read: expr, $parser: expr $(,)?) => {
        $crate::decode_tokio!($decoder, $read, $parser, |input, _position| input)
    };

    ($decoder: expr, $read: expr, $parser: expr, $input_stream: expr $(,)?) => {
        $crate::decode_tokio!($decoder, $read, $parser, $input_stream, |x| x)
    };

    ($decoder: expr, $read: expr, $parser: expr, $input_stream: expr, $post_decode: expr $(,)?) => {
        match $decoder {
            ref mut decoder => match $read {
                ref mut read => 'outer: loop {
                    let (opt, removed) = {
                        let (state, position, buffer, end_of_input) = decoder.__inner();
                        let buffer =
                            $crate::stream::buf_reader::CombineBuffer::buffer(buffer, &*read);
                        let mut stream = $crate::stream::call_with2(
                            $crate::stream::MaybePartialStream(buffer, !end_of_input),
                            *position,
                            $input_stream,
                        );
                        let result = $crate::stream::decode($parser, &mut stream, state);
                        *position = $crate::stream::Positioned::position(&stream);
                        $crate::stream::call_with(stream, $post_decode);
                        match result {
                            Ok(x) => x,
                            Err(err) => {
                                break 'outer Err($crate::stream::decoder::Error::Parse(err))
                            }
                        }
                    };

                    decoder.advance_pin(std::pin::Pin::new(read), removed);

                    if let Some(v) = opt {
                        break 'outer Ok(v);
                    }

                    match decoder
                        .__before_parse_tokio(std::pin::Pin::new(&mut *read))
                        .await
                    {
                        Ok(x) => x,
                        Err(error) => {
                            break 'outer Err($crate::stream::decoder::Error::Io {
                                error,
                                position: Clone::clone(decoder.position()),
                            })
                        }
                    };
                },
            },
        }
    };
}

#[doc(hidden)]
pub fn call_with2<F, A, B, R>(a: A, b: B, f: F) -> R
where
    F: FnOnce(A, B) -> R,
{
    f(a, b)
}

#[doc(hidden)]
pub fn call_with<F, A, R>(a: A, f: F) -> R
where
    F: FnOnce(A) -> R,
{
    f(a)
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    #[inline]
    fn uncons_range_at_end() {
        assert_eq!("".uncons_range(0), Ok(""));
        assert_eq!("123".uncons_range(3), Ok("123"));
        assert_eq!((&[1][..]).uncons_range(1), Ok(&[1][..]));
        let s: &[u8] = &[];
        assert_eq!(SliceStream(s).uncons_range(0), Ok(&[][..]));
    }

    #[test]
    fn larger_than_1_byte_items_return_correct_distance() {
        let mut input = &[123i32, 0i32][..];

        let before = input.checkpoint();
        assert_eq!(input.distance(&before), 0);

        input.uncons().unwrap();
        assert_eq!(input.distance(&before), 1);

        input.uncons().unwrap();
        assert_eq!(input.distance(&before), 2);

        input.reset(before).unwrap();
        assert_eq!(input.distance(&before), 0);
    }
}
