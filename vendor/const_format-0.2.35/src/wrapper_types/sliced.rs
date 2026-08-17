use crate::{
    fmt::{Error, Formatter},
    wrapper_types::AsciiStr,
};

use core::ops::{Range, RangeFrom, RangeFull, RangeInclusive, RangeTo, RangeToInclusive};

/// Wrapper for writing a range of a string slice.
///
/// This is a workaround for not being able to do `&string[start..end]` at compile-time.
///
/// # Example
///
/// ```rust
///
/// use const_format::Sliced;
/// use const_format::{concatc, formatc};
///
/// const NUMS: &str = "0123456789";
/// const SRC: &str = "foo bar baz";
///
/// assert_eq!(concatc!(Sliced(NUMS, 1..=4)), "1234");
/// assert_eq!(concatc!(Sliced(SRC, 0..5), "ros."), "foo bros.");
///
/// assert_eq!(formatc!("{}", Sliced(NUMS, 4..)), "456789");
/// assert_eq!(formatc!("{}t", Sliced(SRC, 4..7)), "bart");
///
/// ```
#[cfg_attr(feature = "__docsrs", doc(cfg(feature = "fmt")))]
pub struct Sliced<T, R>(pub T, pub R);

impl_fmt! {
    impl['a,] Sliced<&'a str, Range<usize>>;
    impl['a,] Sliced<&'a str, RangeFrom<usize>>;
    impl['a,] Sliced<&'a str, RangeFull>;
    impl['a,] Sliced<&'a str, RangeInclusive<usize>>;
    impl['a,] Sliced<&'a str, RangeTo<usize>>;
    impl['a,] Sliced<&'a str, RangeToInclusive<usize>>;

    ///
    #[inline]
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str_range_debug(self.0, self.range())
    }

    ///
    #[inline]
    pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_str_range(self.0, self.range())
    }
}

impl_fmt! {
    impl['a,] Sliced<AsciiStr<'a>, Range<usize>>;
    impl['a,] Sliced<AsciiStr<'a>, RangeFrom<usize>>;
    impl['a,] Sliced<AsciiStr<'a>, RangeFull>;
    impl['a,] Sliced<AsciiStr<'a>, RangeInclusive<usize>>;
    impl['a,] Sliced<AsciiStr<'a>, RangeTo<usize>>;
    impl['a,] Sliced<AsciiStr<'a>, RangeToInclusive<usize>>;

    ///
    #[inline]
    pub const fn const_debug_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_ascii_range_debug(self.0, self.range())
    }

    ///
    #[inline]
    pub const fn const_display_fmt(&self, f: &mut Formatter<'_>) -> Result<(), Error> {
        f.write_ascii_range(self.0, self.range())
    }
}

impl<T> Sliced<T, Range<usize>> {
    #[inline(always)]
    const fn range(&self) -> Range<usize> {
        self.1.start..self.1.end
    }
}

impl<T> Sliced<T, RangeFrom<usize>> {
    #[inline(always)]
    const fn range(&self) -> Range<usize> {
        self.1.start..usize::MAX
    }
}

impl<T> Sliced<T, RangeFull> {
    #[inline(always)]
    const fn range(&self) -> Range<usize> {
        0..usize::MAX
    }
}

const UM: usize = usize::MAX >> 1;

impl<T> Sliced<T, RangeInclusive<usize>> {
    // If people are somehow indexing over a thing that's larger than isize::MAX,
    // then we have problems.
    #[inline(always)]
    const fn range(&self) -> Range<usize> {
        *self.1.start()..(*self.1.end() & UM) + 1
    }
}

impl<T> Sliced<T, RangeTo<usize>> {
    #[inline(always)]
    const fn range(&self) -> Range<usize> {
        0..self.1.end
    }
}

impl<T> Sliced<T, RangeToInclusive<usize>> {
    // If people are somehow indexing over a thing that's larger than isize::MAX,
    // then we have problems.
    #[inline(always)]
    const fn range(&self) -> Range<usize> {
        0..(self.1.end & UM) + 1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use crate::fmt::{FormattingFlags, StrWriter};

    macro_rules! test_case {
        (
            $writer:ident,
            $str_val:expr,
            $range:expr,
            $expected:expr
        ) => {{
            const fn inner(fmt: &mut Formatter<'_>) -> Result<(), Error> {
                let string = Sliced($str_val, $range);

                try_!(string.const_display_fmt(fmt));
                try_!(fmt.write_str(";"));
                try_!(string.const_debug_fmt(fmt));

                Ok(())
            }
            $writer.clear();
            inner(&mut $writer.make_formatter(FormattingFlags::NEW)).unwrap();

            assert_eq!($writer.as_str(), $expected, "range = {:?}", $range);
        }};
    }

    const S: &str = "\x00\n\t3456789\x06\x07";

    macro_rules! generate_test {
        ($str_val:expr) => {
            let writer: &mut StrWriter = &mut StrWriter::new([0u8; 512]);

            test_case!(writer, $str_val, 2..9, "\t345678;\"\\t345678\"");

            test_case!(
                writer,
                $str_val,
                2..,
                "\t3456789\x06\x07;\"\\t3456789\\x06\\x07\""
            );

            test_case!(
                writer,
                $str_val,
                ..,
                "\x00\n\t3456789\x06\x07;\"\\x00\\n\\t3456789\\x06\\x07\""
            );

            test_case!(
                writer,
                $str_val,
                ..9,
                "\x00\n\t345678;\"\\x00\\n\\t345678\""
            );

            test_case!(writer, $str_val, 2..=9, "\t3456789;\"\\t3456789\"");
            test_case!(
                writer,
                $str_val,
                2..=!0,
                "\t3456789\x06\x07;\"\\t3456789\\x06\\x07\""
            );

            test_case!(
                writer,
                $str_val,
                ..=9,
                "\x00\n\t3456789;\"\\x00\\n\\t3456789\""
            );
            test_case!(
                writer,
                $str_val,
                ..=!0,
                "\x00\n\t3456789\x06\x07;\"\\x00\\n\\t3456789\\x06\\x07\""
            );
        };
    }

    #[test]
    fn str_tests() {
        generate_test!(S);
    }
    #[test]
    fn asciistr_tests() {
        const ASCII: AsciiStr<'_> = ascii_str!(S);
        generate_test!(ASCII);
    }
}
