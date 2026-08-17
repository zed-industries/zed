use std::{
    fmt,
    io::{self, Bytes, Read},
};

use crate::{
    error::{ParseError, StreamError, Tracked},
    stream::{StreamErrorFor, StreamOnce},
};

#[derive(Debug)]
pub enum Error {
    Unexpected,
    EndOfInput,
    Io(io::Error),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::Unexpected => write!(f, "unexpected parse"),
            Error::EndOfInput => write!(f, "unexpected end of input"),
            Error::Io(err) => write!(f, "{}", err),
        }
    }
}

impl PartialEq for Error {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Error::Unexpected, Error::Unexpected) => true,
            (Error::EndOfInput, Error::EndOfInput) => true,
            _ => false,
        }
    }
}

impl<Item, Range> StreamError<Item, Range> for Error {
    #[inline]
    fn unexpected_token(_: Item) -> Self {
        Error::Unexpected
    }
    #[inline]
    fn unexpected_range(_: Range) -> Self {
        Error::Unexpected
    }
    #[inline]
    fn unexpected_format<T>(_: T) -> Self
    where
        T: fmt::Display,
    {
        Error::Unexpected
    }

    #[inline]
    fn expected_token(_: Item) -> Self {
        Error::Unexpected
    }
    #[inline]
    fn expected_range(_: Range) -> Self {
        Error::Unexpected
    }
    #[inline]
    fn expected_format<T>(_: T) -> Self
    where
        T: fmt::Display,
    {
        Error::Unexpected
    }
    #[inline]
    fn message_format<T>(_: T) -> Self
    where
        T: fmt::Display,
    {
        Error::Unexpected
    }
    #[inline]
    fn message_token(_: Item) -> Self {
        Error::Unexpected
    }
    #[inline]
    fn message_range(_: Range) -> Self {
        Error::Unexpected
    }

    #[inline]
    fn end_of_input() -> Self {
        Error::EndOfInput
    }

    #[inline]
    fn is_unexpected_end_of_input(&self) -> bool {
        *self == Error::EndOfInput
    }

    #[inline]
    fn into_other<T>(self) -> T
    where
        T: StreamError<Item, Range>,
    {
        match self {
            Error::Unexpected => T::unexpected_static_message("parse"),
            Error::EndOfInput => T::end_of_input(),
            Error::Io(err) => T::other(err),
        }
    }
}

impl<Item, Range, Position> ParseError<Item, Range, Position> for Error
where
    Position: Default,
{
    type StreamError = Self;
    #[inline]
    fn empty(_position: Position) -> Self {
        Error::Unexpected
    }

    #[inline]
    fn from_error(_: Position, err: Self::StreamError) -> Self {
        err
    }

    #[inline]
    fn set_position(&mut self, _position: Position) {}

    #[inline]
    fn add(&mut self, err: Self::StreamError) {
        *self = match (&*self, err) {
            (Error::EndOfInput, _) => Error::EndOfInput,
            (_, err) => err,
        };
    }

    #[inline]
    fn set_expected<F>(self_: &mut Tracked<Self>, info: Self::StreamError, f: F)
    where
        F: FnOnce(&mut Tracked<Self>),
    {
        f(self_);
        self_.error = info;
    }

    fn is_unexpected_end_of_input(&self) -> bool {
        *self == Error::EndOfInput
    }

    #[inline]
    fn into_other<T>(self) -> T
    where
        T: ParseError<Item, Range, Position>,
    {
        T::from_error(Position::default(), StreamError::into_other(self))
    }
}

pub struct Stream<R> {
    bytes: Bytes<R>,
}

impl<R: Read> StreamOnce for Stream<R> {
    type Token = u8;
    type Range = &'static [u8];
    type Position = usize;
    type Error = Error;

    #[inline]
    fn uncons(&mut self) -> Result<u8, StreamErrorFor<Self>> {
        match self.bytes.next() {
            Some(Ok(b)) => Ok(b),
            Some(Err(err)) => Err(Error::Io(err)),
            None => Err(Error::EndOfInput),
        }
    }
}

impl<R> Stream<R>
where
    R: Read,
{
    /// Creates a `StreamOnce` instance from a value implementing `std::io::Read`.
    ///
    /// NOTE: This type do not implement `Positioned` and `Clone` and must be wrapped with types
    ///     such as `BufferedStreamRef` and `State` to become a `Stream` which can be parsed
    ///
    /// ```rust
    /// # #![cfg(feature = "std")]
    /// # extern crate combine;
    /// use combine::*;
    /// use combine::parser::byte::*;
    /// use combine::stream::read;
    /// use combine::stream::buffered;
    /// use combine::stream::position;
    /// use std::io::Read;
    ///
    /// # fn main() {
    /// let input: &[u8] = b"123,";
    /// let stream = buffered::Stream::new(position::Stream::new(read::Stream::new(input)), 1);
    /// let result = (many(digit()), byte(b','))
    ///     .parse(stream)
    ///     .map(|t| t.0);
    /// assert_eq!(result, Ok((vec![b'1', b'2', b'3'], b',')));
    /// # }
    /// ```
    pub fn new(read: R) -> Stream<R> {
        Stream {
            bytes: read.bytes(),
        }
    }
}
