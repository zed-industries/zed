// use crate::fmt::Error;

#[cfg(feature = "fmt")]
use crate::fmt::{Error, Formatter};

use core::fmt::{self, Display};

////////////////////////////////////////////////////////////////////////////////

/// An ascii string slice.
///
/// You can also construct an `AsciiStr` at compile-time with the [`ascii_str`] macro,
/// erroring at compile if the constant isn't ascii.
///
/// # Example
///
/// ```rust
/// use const_format::wrapper_types::{AsciiStr, NotAsciiError};
/// use const_format::ascii_str;
///
/// const HELLO: AsciiStr = unwrap_ascii(AsciiStr::new(b"hello"));
/// const EURO: AsciiStr = unwrap_ascii(AsciiStr::new("foo €".as_bytes()));
///
/// assert_eq!(HELLO.as_str(), "hello");
/// assert_eq!(EURO.as_str(), "<error>");
/// assert_eq!(AsciiStr::new("foo €".as_bytes()), Err(NotAsciiError{invalid_from: 4}));
///
/// const fn unwrap_ascii(res: Result<AsciiStr<'_>, NotAsciiError>) -> AsciiStr<'_> {
///     match res {
///         Ok(x) => x,
///         Err(_) => ascii_str!("<error>"),
///     }
/// }
///
/// ```
///
/// [`ascii_str`]: ./macro.ascii_str.html
///
#[cfg_attr(feature = "__docsrs", doc(cfg(feature = "fmt")))]
#[derive(Debug, Copy, Clone, PartialEq, Eq, Ord, PartialOrd, Hash)]
pub struct AsciiStr<'a>(&'a [u8]);

impl<'a> AsciiStr<'a> {
    /// Constructs this  AsciiStr from a possibly non-ascii str slice.
    ///
    /// Returns a `NonAsciiError` error on the first non-ascii byte.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::wrapper_types::{AsciiStr, NotAsciiError};
    ///
    /// let ok = AsciiStr::from_str("foo bar").unwrap();
    ///
    /// assert_eq!(ok.as_str(), "foo bar");
    /// assert_eq!(AsciiStr::from_str("foo bar ½"), Err(NotAsciiError{invalid_from: 8}));
    ///
    /// ```
    #[inline(always)]
    pub const fn from_str(s: &'a str) -> Result<Self, NotAsciiError> {
        Self::new(s.as_bytes())
    }

    /// Constructs this  AsciiStr from a possibly non-ascii byte slice.
    ///
    /// Returns a `NonAsciiError` error on the first non-ascii byte.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::wrapper_types::{AsciiStr, NotAsciiError};
    ///
    /// let ok = AsciiStr::new(b"foo bar").unwrap();
    ///
    /// assert_eq!(ok.as_str(), "foo bar");
    /// assert_eq!(AsciiStr::new(b"foo bar \x80"), Err(NotAsciiError{invalid_from: 8}));
    ///
    /// ```
    pub const fn new(s: &'a [u8]) -> Result<Self, NotAsciiError> {
        __for_range! {i in 0..s.len()=>
            if s[i] > 127 {
                return Err(NotAsciiError{invalid_from: i});
            }
        }
        Ok(AsciiStr(s))
    }

    /// Constructs an empty `AsciiStr`
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::AsciiStr;
    ///
    /// assert_eq!(AsciiStr::empty().as_str(), "");
    /// ```
    pub const fn empty() -> Self {
        Self(&[])
    }

    /// Queries the length of the `AsciiStr`
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::{AsciiStr, ascii_str};
    ///
    /// assert_eq!(AsciiStr::empty().len(), 0);
    /// assert_eq!(ascii_str!("hello").len(), 5);
    /// ```
    #[inline(always)]
    pub const fn len(self) -> usize {
        self.0.len()
    }

    /// Queries whether this `AsciiStr` is empty.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::{AsciiStr, ascii_str};
    ///
    /// assert_eq!(AsciiStr::empty().is_empty(), true);
    /// assert_eq!(ascii_str!("hello").is_empty(), false);
    /// ```
    #[inline(always)]
    pub const fn is_empty(self) -> bool {
        self.0.is_empty()
    }

    /// Accessor for the wrapped ascii string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::{AsciiStr, ascii_str};
    ///
    /// assert_eq!(AsciiStr::empty().as_bytes(), b"");
    /// assert_eq!(ascii_str!("hello").as_bytes(), b"hello");
    /// ```
    #[inline(always)]
    pub const fn as_bytes(self) -> &'a [u8] {
        self.0
    }

    /// Accessor for the wrapped ascii string.
    ///
    /// # Example
    ///
    /// ```rust
    /// use const_format::{AsciiStr, ascii_str};
    ///
    /// assert_eq!(AsciiStr::empty().as_str(), "");
    /// assert_eq!(ascii_str!("hello").as_str(), "hello");
    /// ```
    #[inline]
    pub fn as_str(self) -> &'a str {
        unsafe { core::str::from_utf8_unchecked(self.0) }
    }
}

////////////////////////////////////////////////////////////////////////////////

/// Error from [`AsciiStr`] constructor, caused by a byte not being valid ascii.
///
/// [`AsciiStr`]: ../struct.AsciiStr.html
#[derive(Debug, Copy, Clone, PartialEq)]
pub struct NotAsciiError {
    /// The first non-ascii byte in the byte slice.
    pub invalid_from: usize,
}

impl Display for NotAsciiError {
    fn fmt(&self, fmt: &mut fmt::Formatter<'_>) -> Result<(), fmt::Error> {
        fmt.write_str("error: the input bytes were not valid ascii")
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(feature = "fmt")]
impl_fmt! {
    impl AsciiStr<'_>;

    ///
    pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_ascii(*self)
    }

    ///
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_ascii_debug(*self)
    }
}

////////////////////////////////////////////////////////////////////////////////

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg(feature = "fmt")]
    use crate::fmt::ComputeStrLength;

    use arrayvec::ArrayString;

    #[test]
    fn basic() {
        {
            let ok = AsciiStr::new("hello!".as_bytes()).unwrap();
            assert_eq!(ok.as_bytes(), "hello!".as_bytes());
            assert_eq!(ok.as_str(), "hello!");
        }
        {
            let err = AsciiStr::from_str("Φοο!").unwrap_err();
            assert_eq!(err.invalid_from, 0)
        }
        {
            let err = AsciiStr::from_str("hello Φοο!").unwrap_err();
            assert_eq!(err.invalid_from, 6)
        }
    }

    // This doesn't use unsafe code
    #[cfg(not(miri))]
    #[test]
    fn only_ascii_constructible() {
        let mut string = ArrayString::<1024>::new();
        let min = '\u{20}';
        let max = '\u{80}';
        assert!(!max.is_ascii());
        for end in min..=max {
            for start in min..=end {
                string.clear();
                for n in start..=end {
                    string.push(n);
                }
                let res = AsciiStr::new(string.as_bytes());
                assert_eq!(res.is_ok(), string.as_bytes().is_ascii());

                if let Ok(ascii) = res {
                    assert_eq!(ascii.as_bytes(), string.as_bytes());
                }
            }
        }
    }

    #[cfg(feature = "fmt")]
    #[test]
    fn formatting() {
        use crate::fmt::{FormattingFlags, NumberFormatting, StrWriter};

        const fn inner_debug(
            ascii: AsciiStr<'_>,
            writer: &mut StrWriter,
            flags: FormattingFlags,
        ) -> Result<usize, Error> {
            try_!(ascii.const_debug_fmt(&mut writer.make_formatter(flags)));

            let mut str_len = ComputeStrLength::new();
            try_!(ascii.const_debug_fmt(&mut str_len.make_formatter(flags)));

            Ok(str_len.len())
        }

        const fn inner_display(
            ascii: AsciiStr<'_>,
            writer: &mut StrWriter,
            flags: FormattingFlags,
        ) -> Result<usize, Error> {
            try_!(ascii.const_display_fmt(&mut writer.make_formatter(flags)));

            let mut str_len = ComputeStrLength::new();
            try_!(ascii.const_display_fmt(&mut str_len.make_formatter(flags)));

            Ok(str_len.len())
        }

        fn test_case(
            ascii: AsciiStr<'_>,
            writer: &mut StrWriter,
            flags: FormattingFlags,
            expected_debug: &str,
            expected_display: &str,
        ) {
            writer.clear();
            let len = inner_debug(ascii, writer, flags).unwrap();

            assert_eq!(writer.as_str(), expected_debug);
            assert_eq!(writer.len(), len, "{}", writer.as_str());

            writer.clear();
            let len = inner_display(ascii, writer, flags).unwrap();

            assert_eq!(writer.as_str(), expected_display);
            assert_eq!(writer.len(), len, "{}", writer.as_str());
        }

        let writer: &mut StrWriter = &mut StrWriter::new([0; 128]);

        let foo = AsciiStr::new("\0\x10hello\tworld\n".as_bytes()).unwrap();

        for num_fmt in NumberFormatting::ALL.iter().copied() {
            for is_alt in [false, true].iter().copied() {
                let flag = FormattingFlags::NEW
                    .set_num_fmt(num_fmt)
                    .set_alternate(is_alt);
                test_case(
                    foo,
                    writer,
                    flag,
                    "\"\\x00\\x10hello\\tworld\\n\"",
                    foo.as_str(),
                );
            }
        }
    }
}
