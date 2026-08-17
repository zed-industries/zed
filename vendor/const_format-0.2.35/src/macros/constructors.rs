/// Constructs an [`AsciiStr`] constant from an ascii string,
///
/// # Compile-time errors
///
/// This macro produces a compile-time error by indexing an empty array with
/// the index of the first non-ascii byte.
///
/// # Example
///
/// ```rust
/// use const_format::ascii_str;
///
/// let fooo = ascii_str!("hello");
///
/// assert_eq!(fooo.as_str(), "hello");
///
/// // You can pass constants as arguments!
/// const BAR_S: &str = "world";
/// let bar = ascii_str!(BAR_S);
///
/// assert_eq!(bar.as_str(), "world");
///
/// ```
///
/// ```compile_fail
/// use const_format::ascii_str;
///
/// let fooo = ascii_str!("Γειά σου Κόσμε!");
///
/// ```
///
/// [`AsciiStr`]: ./struct.AsciiStr.html
///
#[cfg_attr(feature = "__docsrs", doc(cfg(feature = "fmt")))]
#[cfg(feature = "fmt")]
#[macro_export]
macro_rules! ascii_str {
    ($str:expr $(,)*) => {{
        const __CF_ASCII_STR_CONSTANT: $crate::AsciiStr<'static> = {
            match $crate::AsciiStr::new($str.as_bytes()) {
                Ok(x) => x,
                $crate::pmr::Err(e) => [][e.invalid_from],
            }
        };
        __CF_ASCII_STR_CONSTANT
    }};
}
