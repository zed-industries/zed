use crate::error::ParserError;
use crate::stream::{Compare, CompareResult};
use crate::stream::{SliceLen, Stream, StreamIsPartial};
use crate::Parser;
use crate::Result;

/// Mark a value as case-insensitive for ASCII characters
///
/// # Example
/// ```rust
/// # use winnow::prelude::*;
/// # use winnow::ascii::Caseless;
///
/// fn parser<'s>(s: &mut &'s str) -> ModalResult<&'s str> {
///   Caseless("hello").parse_next(s)
/// }
///
/// assert_eq!(parser.parse_peek("Hello, World!"), Ok((", World!", "Hello")));
/// assert_eq!(parser.parse_peek("hello, World!"), Ok((", World!", "hello")));
/// assert_eq!(parser.parse_peek("HeLlo, World!"), Ok((", World!", "HeLlo")));
/// assert!(parser.parse_peek("Some").is_err());
/// assert!(parser.parse_peek("").is_err());
/// ```
#[derive(Copy, Clone, Debug)]
pub struct Caseless<T>(pub T);

impl Caseless<&str> {
    /// Get the byte-representation of this case-insensitive value
    #[inline(always)]
    pub fn as_bytes(&self) -> Caseless<&[u8]> {
        Caseless(self.0.as_bytes())
    }
}

impl<S: SliceLen> SliceLen for Caseless<S> {
    #[inline(always)]
    fn slice_len(&self) -> usize {
        self.0.slice_len()
    }
}

impl<'b> Compare<Caseless<&'b [u8]>> for &[u8] {
    #[inline]
    fn compare(&self, t: Caseless<&'b [u8]>) -> CompareResult {
        if t.0
            .iter()
            .zip(*self)
            .any(|(a, b)| !a.eq_ignore_ascii_case(b))
        {
            CompareResult::Error
        } else if self.len() < t.slice_len() {
            CompareResult::Incomplete
        } else {
            CompareResult::Ok(t.slice_len())
        }
    }
}

impl<const LEN: usize> Compare<Caseless<[u8; LEN]>> for &[u8] {
    #[inline(always)]
    fn compare(&self, t: Caseless<[u8; LEN]>) -> CompareResult {
        self.compare(Caseless(&t.0[..]))
    }
}

impl<'b, const LEN: usize> Compare<Caseless<&'b [u8; LEN]>> for &[u8] {
    #[inline(always)]
    fn compare(&self, t: Caseless<&'b [u8; LEN]>) -> CompareResult {
        self.compare(Caseless(&t.0[..]))
    }
}

impl<'b> Compare<Caseless<&'b str>> for &[u8] {
    #[inline(always)]
    fn compare(&self, t: Caseless<&'b str>) -> CompareResult {
        self.compare(Caseless(t.0.as_bytes()))
    }
}

impl Compare<Caseless<u8>> for &[u8] {
    #[inline]
    fn compare(&self, t: Caseless<u8>) -> CompareResult {
        match self.first() {
            Some(c) if t.0.eq_ignore_ascii_case(c) => CompareResult::Ok(t.slice_len()),
            Some(_) => CompareResult::Error,
            None => CompareResult::Incomplete,
        }
    }
}

impl Compare<Caseless<char>> for &[u8] {
    #[inline(always)]
    fn compare(&self, t: Caseless<char>) -> CompareResult {
        self.compare(Caseless(t.0.encode_utf8(&mut [0; 4]).as_bytes()))
    }
}

impl<'b> Compare<Caseless<&'b str>> for &str {
    #[inline(always)]
    fn compare(&self, t: Caseless<&'b str>) -> CompareResult {
        self.as_bytes().compare(t.as_bytes())
    }
}

impl Compare<Caseless<char>> for &str {
    #[inline(always)]
    fn compare(&self, t: Caseless<char>) -> CompareResult {
        self.as_bytes().compare(t)
    }
}

/// This is a shortcut for [`literal`][crate::token::literal].
///
/// # Example
/// ```rust
/// # use winnow::prelude::*;
/// # use winnow::{error::ErrMode, error::ContextError, error::Needed};
/// # use winnow::combinator::alt;
/// # use winnow::token::take;
/// use winnow::ascii::Caseless;
///
/// fn parser<'s>(s: &mut &'s [u8]) -> ModalResult<&'s [u8]> {
///   alt((Caseless(&"hello"[..]), take(5usize))).parse_next(s)
/// }
///
/// assert_eq!(parser.parse_peek(&b"Hello, World!"[..]), Ok((&b", World!"[..], &b"Hello"[..])));
/// assert_eq!(parser.parse_peek(&b"hello, World!"[..]), Ok((&b", World!"[..], &b"hello"[..])));
/// assert_eq!(parser.parse_peek(&b"HeLlo, World!"[..]), Ok((&b", World!"[..], &b"HeLlo"[..])));
/// assert_eq!(parser.parse_peek(&b"Something"[..]), Ok((&b"hing"[..], &b"Somet"[..])));
/// assert!(parser.parse_peek(&b"Some"[..]).is_err());
/// assert!(parser.parse_peek(&b""[..]).is_err());
/// ```
impl<'s, I, E: ParserError<I>> Parser<I, <I as Stream>::Slice, E> for Caseless<&'s [u8]>
where
    I: Compare<Caseless<&'s [u8]>> + StreamIsPartial,
    I: Stream,
{
    #[inline(always)]
    fn parse_next(&mut self, i: &mut I) -> Result<<I as Stream>::Slice, E> {
        crate::token::literal(*self).parse_next(i)
    }
}

/// This is a shortcut for [`literal`][crate::token::literal].
///
/// # Example
/// ```rust
/// # use winnow::prelude::*;
/// # use winnow::{error::ErrMode, error::ContextError, error::Needed};
/// # use winnow::combinator::alt;
/// # use winnow::token::take;
/// use winnow::ascii::Caseless;
///
/// fn parser<'s>(s: &mut &'s [u8]) -> ModalResult<&'s [u8]> {
///   alt((Caseless(b"hello"), take(5usize))).parse_next(s)
/// }
///
/// assert_eq!(parser.parse_peek(&b"Hello, World!"[..]), Ok((&b", World!"[..], &b"Hello"[..])));
/// assert_eq!(parser.parse_peek(&b"hello, World!"[..]), Ok((&b", World!"[..], &b"hello"[..])));
/// assert_eq!(parser.parse_peek(&b"HeLlo, World!"[..]), Ok((&b", World!"[..], &b"HeLlo"[..])));
/// assert_eq!(parser.parse_peek(&b"Something"[..]), Ok((&b"hing"[..], &b"Somet"[..])));
/// assert!(parser.parse_peek(&b"Some"[..]).is_err());
/// assert!(parser.parse_peek(&b""[..]).is_err());
/// ```
impl<'s, I, E: ParserError<I>, const N: usize> Parser<I, <I as Stream>::Slice, E>
    for Caseless<&'s [u8; N]>
where
    I: Compare<Caseless<&'s [u8; N]>> + StreamIsPartial,
    I: Stream,
{
    #[inline(always)]
    fn parse_next(&mut self, i: &mut I) -> Result<<I as Stream>::Slice, E> {
        crate::token::literal(*self).parse_next(i)
    }
}

/// This is a shortcut for [`literal`][crate::token::literal].
///
/// # Example
/// ```rust
/// # use winnow::prelude::*;
/// # use winnow::{error::ErrMode, error::ContextError};
/// # use winnow::combinator::alt;
/// # use winnow::token::take;
/// # use winnow::ascii::Caseless;
///
/// fn parser<'s>(s: &mut &'s str) -> ModalResult<&'s str> {
///   alt((Caseless("hello"), take(5usize))).parse_next(s)
/// }
///
/// assert_eq!(parser.parse_peek("Hello, World!"), Ok((", World!", "Hello")));
/// assert_eq!(parser.parse_peek("hello, World!"), Ok((", World!", "hello")));
/// assert_eq!(parser.parse_peek("HeLlo, World!"), Ok((", World!", "HeLlo")));
/// assert_eq!(parser.parse_peek("Something"), Ok(("hing", "Somet")));
/// assert!(parser.parse_peek("Some").is_err());
/// assert!(parser.parse_peek("").is_err());
/// ```
impl<'s, I, E: ParserError<I>> Parser<I, <I as Stream>::Slice, E> for Caseless<&'s str>
where
    I: Compare<Caseless<&'s str>> + StreamIsPartial,
    I: Stream,
{
    #[inline(always)]
    fn parse_next(&mut self, i: &mut I) -> Result<<I as Stream>::Slice, E> {
        crate::token::literal(*self).parse_next(i)
    }
}
