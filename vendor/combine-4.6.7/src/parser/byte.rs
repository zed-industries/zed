//! Module containing parsers specialized on byte streams.

use crate::{
    error::{self, ParseResult::*},
    parser::{
        combinator::no_partial,
        range::{take_fn, TakeRange},
        repeat::skip_many,
        token::{satisfy, token, tokens_cmp, Token},
    },
    stream::{RangeStream, Stream},
    Parser,
};

/// Parses a byte and succeeds if the byte is equal to `c`.
///
/// ```
/// use combine::Parser;
/// use combine::parser::byte::byte;
/// assert_eq!(byte(b'!').parse(&b"!"[..]), Ok((b'!', &b""[..])));
/// assert!(byte(b'A').parse(&b""[..]).is_err());
/// assert!(byte(b'A').parse(&b"!"[..]).is_err());
/// ```
pub fn byte<Input>(c: u8) -> Token<Input>
where
    Input: Stream<Token = u8>,
{
    token(c)
}

macro_rules! byte_parser {
    ($name:ident, $ty:ident, $f: ident) => {{
        satisfy(|c: u8| c.$f())
            .expected(stringify!($name))
    }};
    ($name:ident, $ty:ident, $f: ident $($args:tt)+) => {{
        satisfy(|c: u8| c.$f $($args)+)
            .expected(stringify!($name))
    }};
}

/// Parses a base-10 digit (0–9).
///
/// ```
/// use combine::Parser;
/// use combine::parser::byte::digit;
/// assert_eq!(digit().parse(&b"9"[..]), Ok((b'9', &b""[..])));
/// assert!(digit().parse(&b"A"[..]).is_err());
/// ```
pub fn digit<Input>() -> impl Parser<Input, Output = u8, PartialState = ()>
where
    Input: Stream<Token = u8>,
{
    byte_parser!(digit, Digit, is_ascii_digit())
}

/// Parses a `b' '`, `b'\t'`, `b'\n'` or `'b\'r'`.
///
/// ```
/// use combine::Parser;
/// use combine::parser::byte::space;
/// assert_eq!(space().parse(&b" "[..]), Ok((b' ', &b""[..])));
/// assert_eq!(space().parse(&b"  "[..]), Ok((b' ', &b" "[..])));
/// assert!(space().parse(&b"!"[..]).is_err());
/// assert!(space().parse(&b""[..]).is_err());
/// ```
pub fn space<Input>() -> impl Parser<Input, Output = u8, PartialState = ()>
where
    Input: Stream<Token = u8>,
{
    byte_parser!(space, Space, is_ascii_whitespace)
}

/// Skips over [`space`] zero or more times
///
/// [`space`]: fn.space.html
///
/// ```
/// use combine::Parser;
/// use combine::parser::byte::spaces;
/// assert_eq!(spaces().parse(&b""[..]), Ok(((), &b""[..])));
/// assert_eq!(spaces().parse(&b"   "[..]), Ok(((), &b""[..])));
/// ```
pub fn spaces<Input>() -> impl Parser<Input, Output = ()>
where
    Input: Stream<Token = u8>,
{
    skip_many(space()).expected("whitespaces")
}

/// Parses a newline byte (`b'\n'`).
///
/// ```
/// use combine::Parser;
/// use combine::parser::byte::newline;
/// assert_eq!(newline().parse(&b"\n"[..]), Ok((b'\n', &b""[..])));
/// assert!(newline().parse(&b"\r"[..]).is_err());
/// ```
pub fn newline<Input>() -> impl Parser<Input, Output = u8, PartialState = ()>
where
    Input: Stream<Token = u8>,
{
    satisfy(|ch: u8| ch == b'\n').expected("lf newline")
}

/// Parses carriage return and newline (`&b"\r\n"`), returning the newline byte.
///
/// ```
/// use combine::Parser;
/// use combine::parser::byte::crlf;
/// assert_eq!(crlf().parse(&b"\r\n"[..]), Ok((b'\n', &b""[..])));
/// assert!(crlf().parse(&b"\r"[..]).is_err());
/// assert!(crlf().parse(&b"\n"[..]).is_err());
/// ```
pub fn crlf<Input>() -> impl Parser<Input, Output = u8, PartialState = ()>
where
    Input: Stream<Token = u8>,
{
    no_partial(satisfy(|ch: u8| ch == b'\r').with(newline())).expected("crlf newline")
}

/// Parses a tab byte (`b'\t'`).
///
/// ```
/// use combine::Parser;
/// use combine::parser::byte::tab;
/// assert_eq!(tab().parse(&b"\t"[..]), Ok((b'\t', &b""[..])));
/// assert!(tab().parse(&b" "[..]).is_err());
/// ```
pub fn tab<Input>() -> impl Parser<Input, Output = u8, PartialState = ()>
where
    Input: Stream<Token = u8>,
{
    satisfy(|ch| ch == b'\t').expected("tab")
}

/// Parses an uppercase ASCII letter (A–Z).
///
/// ```
/// use combine::Parser;
/// use combine::parser::byte::upper;
/// assert_eq!(upper().parse(&b"A"[..]), Ok((b'A', &b""[..])));
/// assert!(upper().parse(&b"a"[..]).is_err());
/// ```
pub fn upper<Input>() -> impl Parser<Input, Output = u8, PartialState = ()>
where
    Input: Stream<Token = u8>,
{
    byte_parser!(upper, Upper, is_ascii_uppercase)
}

/// Parses an lowercase ASCII letter (a–z).
///
/// ```
/// use combine::Parser;
/// use combine::parser::byte::lower;
/// assert_eq!(lower().parse(&b"a"[..]), Ok((b'a', &b""[..])));
/// assert!(lower().parse(&b"A"[..]).is_err());
/// ```
pub fn lower<Input>() -> impl Parser<Input, Output = u8, PartialState = ()>
where
    Input: Stream<Token = u8>,
{
    byte_parser!(lower, Lower, is_ascii_lowercase)
}

/// Parses either an ASCII alphabet letter or digit (a–z, A–Z, 0–9).
///
/// ```
/// use combine::Parser;
/// use combine::parser::byte::alpha_num;
/// assert_eq!(alpha_num().parse(&b"A"[..]), Ok((b'A', &b""[..])));
/// assert_eq!(alpha_num().parse(&b"1"[..]), Ok((b'1', &b""[..])));
/// assert!(alpha_num().parse(&b"!"[..]).is_err());
/// ```
pub fn alpha_num<Input>() -> impl Parser<Input, Output = u8, PartialState = ()>
where
    Input: Stream<Token = u8>,
{
    byte_parser!(alpha_num, AlphaNum, is_ascii_alphanumeric)
}

/// Parses an ASCII alphabet letter (a–z, A–Z).
///
/// ```
/// use combine::Parser;
/// use combine::parser::byte::letter;
/// assert_eq!(letter().parse(&b"a"[..]), Ok((b'a', &b""[..])));
/// assert_eq!(letter().parse(&b"A"[..]), Ok((b'A', &b""[..])));
/// assert!(letter().parse(&b"9"[..]).is_err());
/// ```
pub fn letter<Input>() -> impl Parser<Input, Output = u8, PartialState = ()>
where
    Input: Stream<Token = u8>,
{
    byte_parser!(letter, Letter, is_ascii_alphabetic)
}

/// Parses an octal digit.
///
/// ```
/// use combine::Parser;
/// use combine::parser::byte::oct_digit;
/// assert_eq!(oct_digit().parse(&b"7"[..]), Ok((b'7', &b""[..])));
/// assert!(oct_digit().parse(&b"8"[..]).is_err());
/// ```
pub fn oct_digit<Input>() -> impl Parser<Input, Output = u8, PartialState = ()>
where
    Input: Stream<Token = u8>,
{
    satisfy(|ch| (b'0'..=b'7').contains(&ch)).expected("octal digit")
}

/// Parses an ASCII hexdecimal digit (accepts both uppercase and lowercase).
///
/// ```
/// use combine::Parser;
/// use combine::parser::byte::hex_digit;
/// assert_eq!(hex_digit().parse(&b"F"[..]), Ok((b'F', &b""[..])));
/// assert!(hex_digit().parse(&b"H"[..]).is_err());
/// ```
pub fn hex_digit<Input>() -> impl Parser<Input, Output = u8, PartialState = ()>
where
    Input: Stream<Token = u8>,
{
    byte_parser!(hex_digit, HexDigit, is_ascii_hexdigit())
}

parser! {
/// Parses the bytes `s`.
///
/// If you have a stream implementing [`RangeStream`] such as `&[u8]` you can also use the
/// [`range`] parser which may be more efficient.
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::byte::bytes;
/// # fn main() {
/// let result = bytes(&b"rust"[..])
///     .parse(&b"rust"[..])
///     .map(|x| x.0);
/// assert_eq!(result, Ok(&b"rust"[..]));
/// # }
/// ```
///
/// [`RangeStream`]: super::super::stream::RangeStream
/// [`range`]: super::range::range
pub fn bytes['a, 'b, Input](s: &'static [u8])(Input) -> &'a [u8]
where [
    Input: Stream<Token = u8, Range = &'b [u8]>,
]
{
    bytes_cmp(s, |l: u8, r: u8| l == r)
}
}

parser! {
/// Parses the bytes `s` using `cmp` to compare each token.
///
/// If you have a stream implementing [`RangeStream`] such as `&[u8]` you can also use the
/// [`range`] parser which may be more efficient.
///
/// ```
/// # extern crate combine;
/// # use combine::*;
/// # use combine::parser::byte::bytes_cmp;
/// # use combine::stream::easy::Info;
/// # fn main() {
/// let result = bytes_cmp(&b"abc"[..], |l, r| l.eq_ignore_ascii_case(&r))
///     .parse(&b"AbC"[..]);
/// assert_eq!(result, Ok((&b"abc"[..], &b""[..])));
/// # }
/// ```
///
/// [`RangeStream`]: super::super::stream::RangeStream
/// [`range`]: super::range::range
pub fn bytes_cmp['a, 'b, C, Input](s: &'static [u8], cmp: C)(Input) -> &'a [u8]
where [
    C: FnMut(u8, u8) -> bool,
    Input: Stream<Token = u8, Range = &'b [u8]>,
]
{
    let s = *s;
    tokens_cmp(s.iter().cloned(), cmp)
        .map(move |_| s)
        .expected(error::Range(s))
}
}

macro_rules! take_until {
    (
        $(#[$attr:meta])*
        $type_name: ident, $func_name: ident, $memchr: ident, $($param: ident),+
    ) => {
        parser!{
            #[derive(Clone)]
            pub struct $type_name;
            type PartialState = usize;
            $(#[$attr])*
            pub fn $func_name[Input]($($param : u8),*)(Input) -> Input::Range
                where [
                    Input: RangeStream,
                    Input::Range: AsRef<[u8]> + crate::stream::Range,
                ]
            {
                take_fn(move |haystack: Input::Range| {
                    let haystack = haystack.as_ref();
                    match ::memchr::$memchr( $(*$param),+ , haystack) {
                        Some(i) => TakeRange::Found(i),
                        None => TakeRange::NotFound(haystack.len()),
                    }
                })
            }
        }
    }
}

take_until! {
    /// Zero-copy parser which reads a range of 0 or more tokens until `a` is found.
    ///
    /// If `a` is not found, the parser will return an error.
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::parser::byte::take_until_byte;
    /// # use combine::*;
    /// # fn main() {
    /// let mut parser = take_until_byte(b'\r');
    /// let result = parser.parse("To: user@example.com\r\n");
    /// assert_eq!(result, Ok(("To: user@example.com", "\r\n")));
    /// let result = parser.parse("Hello, world\n");
    /// assert!(result.is_err());
    /// # }
    /// ```
    TakeUntilByte, take_until_byte, memchr, a
}
take_until! {
    /// Zero-copy parser which reads a range of 0 or more tokens until `a` or `b` is found.
    ///
    /// If `a` or `b` is not found, the parser will return an error.
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::parser::byte::take_until_byte2;
    /// # use combine::*;
    /// # fn main() {
    /// let mut parser = take_until_byte2(b'\r', b'\n');
    /// let result = parser.parse("To: user@example.com\r\n");
    /// assert_eq!(result, Ok(("To: user@example.com", "\r\n")));
    /// let result = parser.parse("Hello, world\n");
    /// assert_eq!(result, Ok(("Hello, world", "\n")));
    /// # }
    /// ```
    TakeUntilByte2, take_until_byte2, memchr2, a, b
}
take_until! {
    /// Zero-copy parser which reads a range of 0 or more tokens until `a`, 'b' or `c` is found.
    ///
    /// If `a`, 'b' or `c` is not found, the parser will return an error.
    ///
    /// ```
    /// # extern crate combine;
    /// # use combine::parser::byte::take_until_byte3;
    /// # use combine::*;
    /// # fn main() {
    /// let mut parser = take_until_byte3(b'\r', b'\n', b' ');
    /// let result = parser.parse("To: user@example.com\r\n");
    /// assert_eq!(result, Ok(("To:", " user@example.com\r\n")));
    /// let result = parser.parse("Helloworld");
    /// assert!(result.is_err());
    /// # }
    /// ```
    TakeUntilByte3, take_until_byte3, memchr3, a, b, c
}

parser! {
type PartialState = usize;
/// Zero-copy parser which reads a range of 0 or more tokens until `needle` is found.
///
/// If `a`, 'b' or `c` is not found, the parser will return an error.
///
/// Optimized variant of [`take_until_range`](../range/fn.take_until_range.html)
///
/// ```
/// use combine::*;
/// use combine::parser::byte::take_until_bytes;
/// assert_eq!(
///     take_until_bytes(&b"\r\n"[..]).easy_parse(&b"abc\r\n"[..]).map(|(x, _)| x),
///     Ok((&b"abc"[..]))
/// );
/// // Also works on strings as long as `needle` is UTF-8
/// assert_eq!(
///     take_until_bytes("\r\n".as_bytes()).easy_parse("abc\r\n").map(|(x, _)| x),
///     Ok(("abc"))
/// );
/// ```
pub fn take_until_bytes['a, Input](needle: &'a [u8])(Input) -> Input::Range
where [
    Input: RangeStream,
    Input::Range: AsRef<[u8]> + crate::stream::Range,
]
{
    take_fn(move |haystack: Input::Range| {
        let haystack = haystack.as_ref();
        match memslice(needle, haystack) {
            Some(i) => TakeRange::Found(i),
            None => TakeRange::NotFound(haystack.len().saturating_sub(needle.len() - 1)),
        }
    })
}

}

fn memslice(needle: &[u8], haystack: &[u8]) -> Option<usize> {
    let (&prefix, suffix) = match needle.split_first() {
        Some(x) => x,
        None => return Some(0),
    };
    for i in memchr::memchr_iter(prefix, haystack) {
        if haystack[i + 1..].starts_with(suffix) {
            return Some(i);
        }
    }
    None
}

/// Parsers for decoding numbers in big-endian or little-endian order.
pub mod num {

    use crate::{error::ResultExt, lib::mem::size_of, parser::function::parser, stream::uncons};

    use super::*;

    macro_rules! integer_parser {
        (
            $(#[$attr:meta])*
            pub $type_name: ident,
            $output_type: ident, $be_name: ident, $le_name: ident, $read_name: ident
        ) => {
            $(#[$attr])*
            pub fn $be_name<'a, Input>() -> impl Parser<Input, Output = $output_type, PartialState = ()>
            where
                Input: Stream<Token = u8>,
            {
                parser(|input: &mut Input| {
                    let checkpoint = input.checkpoint();
                    let result = (|input: &mut Input| {
                        let mut buffer = [0u8; size_of::<$output_type>()];
                        for elem in &mut buffer[..] {
                            *elem = ctry!(uncons(input)).0;
                        }
                        CommitOk($output_type::from_be_bytes(buffer))
                    })(input);
                    if result.is_err() {
                        input.reset(checkpoint).committed().into_result()?;
                    }
                    result.into_result()
                })
            }

            $(#[$attr])*
            pub fn $le_name<'a, Input>() -> impl Parser<Input, Output = $output_type, PartialState = ()>
            where
                Input: Stream<Token = u8>,
            {
                parser(|input: &mut Input| {
                    let checkpoint = input.checkpoint();
                    let result = (|input: &mut Input| {
                        let mut buffer = [0u8; size_of::<$output_type>()];
                        for elem in &mut buffer[..] {
                            *elem = ctry!(uncons(input)).0;
                        }
                        CommitOk($output_type::from_le_bytes(buffer))
                    })(input);
                    if result.is_err() {
                        input.reset(checkpoint).committed().into_result()?;
                    }
                    result.into_result()
                })
            }
        }
    }

    integer_parser!(
        /// Reads a u16 out of the byte stream with the specified endianess
        ///
        /// ```
        /// use combine::Parser;
        /// use combine::parser::byte::num::le_u16;
        ///
        /// assert_eq!(le_u16().parse(&b"\x01\0"[..]), Ok((1, &b""[..])));
        /// assert!(le_u16().parse(&b"\0"[..]).is_err());
        /// ```
        pub U16, u16, be_u16, le_u16, read_u16
    );
    integer_parser!(
        /// Reads a u32 out of the byte stream with the specified endianess
        ///
        /// ```
        /// use combine::Parser;
        /// use combine::parser::byte::num::le_u32;
        ///
        /// assert_eq!(le_u32().parse(&b"\x01\0\0\0"[..]), Ok((1, &b""[..])));
        /// assert!(le_u32().parse(&b"\x01\0\0"[..]).is_err());
        /// ```
        pub U32, u32, be_u32, le_u32, read_u32
    );
    integer_parser!(
        /// Reads a u64 out of the byte stream with the specified endianess
        ///
        /// ```
        /// use combine::Parser;
        /// use combine::parser::byte::num::le_u64;
        ///
        /// assert_eq!(le_u64().parse(&b"\x01\0\0\0\0\0\0\0"[..]), Ok((1, &b""[..])));
        /// assert!(le_u64().parse(&b"\x01\0\0\0\0\0\0"[..]).is_err());
        /// ```
        pub U64, u64, be_u64, le_u64, read_u64
    );

    integer_parser!(
        /// Reads a i16 out of the byte stream with the specified endianess
        ///
        /// ```
        /// use combine::Parser;
        /// use combine::parser::byte::num::le_i16;
        ///
        /// assert_eq!(le_i16().parse(&b"\x01\0"[..]), Ok((1, &b""[..])));
        /// assert!(le_i16().parse(&b"\x01"[..]).is_err());
        /// ```
        pub I16, i16, be_i16, le_i16, read_i16
    );

    integer_parser!(
        /// Reads a i32 out of the byte stream with the specified endianess
        ///
        /// ```
        /// use combine::Parser;
        /// use combine::parser::byte::num::le_i32;
        ///
        /// assert_eq!(le_i32().parse(&b"\x01\0\0\0"[..]), Ok((1, &b""[..])));
        /// assert!(le_i32().parse(&b"\x01\0\0"[..]).is_err());
        /// ```
        pub I32, i32, be_i32, le_i32, read_i32
    );
    integer_parser!(
        /// Reads a i64 out of the byte stream with the specified endianess
        ///
        /// ```
        /// use combine::Parser;
        /// use combine::parser::byte::num::le_i64;
        ///
        /// assert_eq!(le_i64().parse(&b"\x01\0\0\0\0\0\0\0"[..]), Ok((1, &b""[..])));
        /// assert!(le_i64().parse(&b"\x01\0\0\0\0\0\0"[..]).is_err());
        /// ```
        pub I64, i64, be_i64, le_i64, read_i64
    );

    integer_parser!(
        /// Reads a i32 out of the byte stream with the specified endianess
        ///
        /// ```
        /// use combine::Parser;
        /// use combine::parser::byte::num::le_f32;
        ///
        /// let buf = 123.45f32.to_le_bytes();
        /// assert_eq!(le_f32().parse(&buf[..]), Ok((123.45, &b""[..])));
        /// assert!(le_f32().parse(&b"\x01\0\0"[..]).is_err());
        /// ```
        pub F32, f32, be_f32, le_f32, read_f32
    );
    integer_parser!(
        /// Reads a i64 out of the byte stream with the specified endianess
        ///
        /// ```
        /// use combine::Parser;
        /// use combine::parser::byte::num::le_f64;
        ///
        /// let buf = 123.45f64.to_le_bytes();
        /// assert_eq!(le_f64().parse(&buf[..]), Ok((123.45, &b""[..])));
        /// assert!(le_f64().parse(&b"\x01\0\0\0\0\0\0"[..]).is_err());
        /// ```
        pub F64, f64, be_f64, le_f64, read_f64
    );

    #[cfg(all(feature = "std", test))]
    mod tests {

        use crate::stream::{buffered, position, IteratorStream};

        use super::*;

        #[test]
        fn no_rangestream() {
            let buf = 123.45f64.to_le_bytes();
            assert_eq!(
                le_f64()
                    .parse(buffered::Stream::new(
                        position::Stream::new(IteratorStream::new(buf.iter().cloned())),
                        1
                    ))
                    .map(|(t, _)| t),
                Ok(123.45)
            );
            assert_eq!(
                le_f64()
                    .parse(buffered::Stream::new(
                        position::Stream::new(IteratorStream::new(buf.iter().cloned())),
                        1
                    ))
                    .map(|(t, _)| t),
                Ok(123.45)
            );
            let buf = 123.45f64.to_be_bytes();
            assert_eq!(
                be_f64()
                    .parse(buffered::Stream::new(
                        position::Stream::new(IteratorStream::new(buf.iter().cloned())),
                        1
                    ))
                    .map(|(t, _)| t),
                Ok(123.45)
            );
        }
    }
}

#[cfg(all(feature = "std", test))]
mod tests {

    use crate::stream::{buffered, position, read};

    use super::*;

    #[test]
    fn memslice_basic() {
        let haystack = b"abc123";
        assert_eq!(memslice(b"", haystack), Some(0));
        assert_eq!(memslice(b"a", haystack), Some(0));
        assert_eq!(memslice(b"ab", haystack), Some(0));
        assert_eq!(memslice(b"c12", haystack), Some(2));

        let haystack2 = b"abcab2";
        assert_eq!(memslice(b"abc", haystack2), Some(0));
        assert_eq!(memslice(b"ab2", haystack2), Some(3));

        let haystack3 = b"aaabaaaa";
        assert_eq!(memslice(b"aaaa", haystack3), Some(4));
    }

    #[test]
    fn bytes_read_stream() {
        assert!(bytes(b"abc")
            .parse(buffered::Stream::new(
                position::Stream::new(read::Stream::new("abc".as_bytes())),
                1
            ))
            .is_ok());
    }
}
