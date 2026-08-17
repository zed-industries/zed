use crate::{
    error::{ParseError, ParseResult, StreamError},
    lib::fmt,
    stream::{
        IteratorStream, Positioned, RangeStreamOnce, ResetStream, SliceStream, StreamErrorFor,
        StreamOnce,
    },
};

#[cfg(feature = "std")]
use crate::stream::read;

/// Trait for tracking the current position of a `Stream`.
pub trait Positioner<Item> {
    /// The type which keeps track of the position
    type Position: Clone + Ord;

    type Checkpoint: Clone;

    /// Returns the current position
    fn position(&self) -> Self::Position;
    /// Updates the position given that `token` has been taken from the stream
    fn update(&mut self, token: &Item);

    fn checkpoint(&self) -> Self::Checkpoint;
    fn reset(&mut self, checkpoint: Self::Checkpoint);
}

/// Trait for tracking the current position of a `RangeStream`.
pub trait RangePositioner<Item, Range>: Positioner<Item> {
    /// Updates the position given that `range` has been taken from the stream
    fn update_range(&mut self, range: &Range);
}

/// Defines a default `Positioner` type for a particular `Stream` type.
pub trait DefaultPositioned {
    type Positioner: Default;
}

impl<'a> DefaultPositioned for &'a str {
    type Positioner = SourcePosition;
}

impl<'a, T> DefaultPositioned for &'a [T] {
    type Positioner = IndexPositioner;
}

impl<'a, T> DefaultPositioned for SliceStream<'a, T> {
    type Positioner = IndexPositioner;
}

impl<T> DefaultPositioned for IteratorStream<T> {
    type Positioner = IndexPositioner;
}

#[cfg(feature = "std")]
impl<R> DefaultPositioned for read::Stream<R> {
    type Positioner = IndexPositioner;
}

/// The `Stream<Input>` struct maintains the current position in the stream `Input` using
/// the `Positioner` trait to track the position.
///
/// ```
/// # #![cfg(feature = "std")]
/// # extern crate combine;
/// # use combine::*;
/// # use combine::stream::easy;
/// # use combine::stream::position;
/// # fn main() {
///     let result = token(b'9')
///         .message("Not a nine")
///         .easy_parse(position::Stream::new(&b"8"[..]));
///     assert_eq!(result, Err(easy::Errors {
///         position: 0,
///         errors: vec![
///             easy::Error::Unexpected(b'8'.into()),
///             easy::Error::Expected(b'9'.into()),
///             easy::Error::Message("Not a nine".into())
///         ]
///     }));
/// # }
/// ```
#[derive(Clone, Debug, PartialEq)]
pub struct Stream<Input, X> {
    /// The input stream used when items are requested
    pub input: Input,
    /// The positioner used to update the current position
    pub positioner: X,
}

impl<Input, X> Stream<Input, X>
where
    Input: StreamOnce,
    X: Positioner<Input::Token>,
{
    /// Creates a new `Stream<Input, X>` from an input stream and a positioner.
    pub fn with_positioner(input: Input, positioner: X) -> Stream<Input, X> {
        Stream { input, positioner }
    }
}

impl<Input> Stream<Input, Input::Positioner>
where
    Input: StreamOnce + DefaultPositioned,
    Input::Positioner: Positioner<Input::Token>,
{
    /// Creates a new `Stream<Input, X>` from an input stream and its default positioner.
    pub fn new(input: Input) -> Stream<Input, Input::Positioner> {
        Stream::with_positioner(input, Input::Positioner::default())
    }
}

impl<Input, X, S> Positioned for Stream<Input, X>
where
    Input: StreamOnce,
    X: Positioner<Input::Token>,
    S: StreamError<Input::Token, Input::Range>,
    Input::Error: ParseError<Input::Token, Input::Range, X::Position, StreamError = S>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position, StreamError = S>,
{
    #[inline]
    fn position(&self) -> Self::Position {
        self.positioner.position()
    }
}

impl<Input, X, S> StreamOnce for Stream<Input, X>
where
    Input: StreamOnce,
    X: Positioner<Input::Token>,
    S: StreamError<Input::Token, Input::Range>,
    Input::Error: ParseError<Input::Token, Input::Range, X::Position, StreamError = S>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position, StreamError = S>,
{
    type Token = Input::Token;
    type Range = Input::Range;
    type Position = X::Position;
    type Error = Input::Error;

    #[inline]
    fn uncons(&mut self) -> Result<Input::Token, StreamErrorFor<Self>> {
        self.input.uncons().map(|c| {
            self.positioner.update(&c);
            c
        })
    }

    fn is_partial(&self) -> bool {
        self.input.is_partial()
    }
}

impl<Item, T> Positioner<Item> for &'_ mut T
where
    Item: Clone,
    T: ?Sized + Positioner<Item>,
{
    type Position = T::Position;
    type Checkpoint = T::Checkpoint;

    #[inline]
    fn position(&self) -> T::Position {
        (**self).position()
    }

    #[inline]
    fn update(&mut self, item: &Item) {
        (**self).update(item)
    }

    #[inline]
    fn checkpoint(&self) -> Self::Checkpoint {
        (**self).checkpoint()
    }

    #[inline]
    fn reset(&mut self, checkpoint: Self::Checkpoint) {
        (**self).reset(checkpoint)
    }
}

impl<Item, Range, T> RangePositioner<Item, Range> for &'_ mut T
where
    Item: Clone,
    Range: Clone + crate::stream::Range,
    T: ?Sized + RangePositioner<Item, Range>,
{
    fn update_range(&mut self, range: &Range) {
        (**self).update_range(range);
    }
}

/// The `IndexPositioner<Item, Range>` struct maintains the current index into the stream `Input`.  The
/// initial index is index 0.  Each `Item` committed increments the index by 1; each `range` committed
/// increments the position by `range.len()`.
#[derive(Clone, Debug, Default, PartialEq)]
pub struct IndexPositioner(usize);

impl<Item> Positioner<Item> for IndexPositioner
where
    Item: Clone,
{
    type Position = usize;
    type Checkpoint = Self;

    #[inline]
    fn position(&self) -> usize {
        self.0
    }

    #[inline]
    fn update(&mut self, _item: &Item) {
        self.0 += 1
    }

    #[inline]
    fn checkpoint(&self) -> Self::Checkpoint {
        self.clone()
    }

    #[inline]
    fn reset(&mut self, checkpoint: Self::Checkpoint) {
        *self = checkpoint;
    }
}

impl IndexPositioner {
    pub fn new() -> IndexPositioner {
        IndexPositioner::new_with_position(0)
    }

    pub fn new_with_position(position: usize) -> IndexPositioner {
        IndexPositioner(position)
    }
}

impl<Item, Range> RangePositioner<Item, Range> for IndexPositioner
where
    Item: Clone,
    Range: Clone + crate::stream::Range,
{
    fn update_range(&mut self, range: &Range) {
        self.0 += range.len()
    }
}

/// Struct which represents a position in a source file.
#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub struct SourcePosition {
    /// Current line of the input
    pub line: i32,
    /// Current column of the input
    pub column: i32,
}

impl Default for SourcePosition {
    fn default() -> Self {
        SourcePosition { line: 1, column: 1 }
    }
}

impl fmt::Display for SourcePosition {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "line: {}, column: {}", self.line, self.column)
    }
}

impl SourcePosition {
    pub fn new() -> Self {
        SourcePosition::default()
    }
}

impl Positioner<char> for SourcePosition {
    type Position = SourcePosition;
    type Checkpoint = Self;

    #[inline]
    fn position(&self) -> SourcePosition {
        *self
    }

    #[inline]
    fn update(&mut self, token: &char) {
        self.column += 1;
        if *token == '\n' {
            self.column = 1;
            self.line += 1;
        }
    }

    #[inline]
    fn checkpoint(&self) -> Self::Checkpoint {
        *self
    }

    #[inline]
    fn reset(&mut self, checkpoint: Self::Checkpoint) {
        *self = checkpoint;
    }
}

impl Positioner<u8> for SourcePosition {
    type Position = SourcePosition;
    type Checkpoint = Self;

    #[inline]
    fn position(&self) -> SourcePosition {
        *self
    }

    #[inline]
    fn update(&mut self, token: &u8) {
        self.column += 1;
        if *token == b'\n' {
            self.column = 1;
            self.line += 1;
        }
    }

    #[inline]
    fn checkpoint(&self) -> Self::Checkpoint {
        *self
    }

    #[inline]
    fn reset(&mut self, checkpoint: Self::Checkpoint) {
        *self = checkpoint;
    }
}

impl<'a> RangePositioner<char, &'a str> for SourcePosition {
    fn update_range(&mut self, range: &&'a str) {
        for c in range.chars() {
            self.update(&c);
        }
    }
}

impl<Input, X, S> RangeStreamOnce for Stream<Input, X>
where
    Input: RangeStreamOnce,
    X: RangePositioner<Input::Token, Input::Range>,
    S: StreamError<Input::Token, Input::Range>,
    Input::Error: ParseError<Input::Token, Input::Range, X::Position, StreamError = S>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position, StreamError = S>,
    Input::Position: Clone + Ord,
{
    #[inline]
    fn uncons_range(&mut self, size: usize) -> Result<Input::Range, StreamErrorFor<Self>> {
        self.input.uncons_range(size).map(|range| {
            self.positioner.update_range(&range);
            range
        })
    }

    #[inline]
    fn uncons_while<F>(&mut self, mut predicate: F) -> Result<Input::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Input::Token) -> bool,
    {
        let positioner = &mut self.positioner;
        self.input.uncons_while(|t| {
            if predicate(t.clone()) {
                positioner.update(&t);
                true
            } else {
                false
            }
        })
    }

    #[inline]
    fn uncons_while1<F>(
        &mut self,
        mut predicate: F,
    ) -> ParseResult<Self::Range, StreamErrorFor<Self>>
    where
        F: FnMut(Self::Token) -> bool,
    {
        let positioner = &mut self.positioner;
        self.input.uncons_while1(|t| {
            if predicate(t.clone()) {
                positioner.update(&t);
                true
            } else {
                false
            }
        })
    }

    #[inline]
    fn distance(&self, end: &Self::Checkpoint) -> usize {
        self.input.distance(&end.input)
    }

    fn range(&self) -> Self::Range {
        self.input.range()
    }
}

impl<Input, X, S> ResetStream for Stream<Input, X>
where
    Input: ResetStream,
    X: Positioner<Input::Token>,
    S: StreamError<Input::Token, Input::Range>,
    Input::Error: ParseError<Input::Token, Input::Range, X::Position, StreamError = S>,
    Input::Error: ParseError<Input::Token, Input::Range, Input::Position, StreamError = S>,
{
    type Checkpoint = Stream<Input::Checkpoint, X::Checkpoint>;
    fn checkpoint(&self) -> Self::Checkpoint {
        Stream {
            input: self.input.checkpoint(),
            positioner: self.positioner.checkpoint(),
        }
    }
    fn reset(&mut self, checkpoint: Self::Checkpoint) -> Result<(), Self::Error> {
        self.input.reset(checkpoint.input)?;
        self.positioner.reset(checkpoint.positioner);
        Ok(())
    }
}

#[cfg(all(feature = "std", test))]
mod tests {

    use crate::Parser;

    use super::*;

    #[test]
    fn test_positioner() {
        let input = ["a".to_string(), "b".to_string()];
        let mut parser = crate::any();
        let result = parser.parse(Stream::new(&input[..]));
        assert_eq!(
            result,
            Ok((
                "a".to_string(),
                Stream::with_positioner(
                    &["b".to_string()][..],
                    IndexPositioner::new_with_position(1)
                )
            ))
        );
    }

    #[test]
    fn test_range_positioner() {
        let input = ["a".to_string(), "b".to_string(), "c".to_string()];
        let mut parser = crate::parser::range::take(2);
        let result = parser.parse(Stream::new(&input[..]));
        assert_eq!(
            result,
            Ok((
                &["a".to_string(), "b".to_string()][..],
                Stream::with_positioner(
                    &["c".to_string()][..],
                    IndexPositioner::new_with_position(2)
                )
            ))
        );
    }
}
