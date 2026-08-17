// Take a look at the license at the top of the repository in the LICENSE file.

use std::num::TryFromIntError;

use libc::{c_char, c_uchar};

use crate::translate::*;

// rustdoc-stripper-ignore-next
/// Wrapper for values where C functions expect a plain C `char`
///
/// Consider the following C function prototype from glib:
///
/// ```C
/// void g_key_file_set_list_separator (GKeyFile *key_file, gchar separator);
/// ```
///
/// This function plainly expects a byte as the `separator` argument.  However,
/// having this function exposed to Rust as the following would be inconvenient:
///
/// ```ignore
/// impl KeyFile {
///     pub fn set_list_separator(&self, separator: libc:c_char) { }
/// }
/// ```
///
/// This would be inconvenient because users would have to do the conversion from a Rust `char` to an `libc::c_char` by hand, which is just a type alias
/// for `i8` on most system.
///
/// This `Char` type is a wrapper over an `libc::c_char`, so that we can pass it to Glib or C functions.
/// The check for whether a Rust `char` (a Unicode scalar value) actually fits in a `libc::c_char` is
/// done in the `new` function; see its documentation for details.
///
/// The inner `libc::c_char` (which is equivalent to `i8`) can be extracted with `.0`, or
/// by calling `my_char.into_glib()`.
///
/// # Examples
/// ```
/// use glib::Char;
/// use std::convert::TryFrom;
///
/// Char::from(b'a');
/// Char::try_from('a').unwrap();
/// assert!(Char::try_from('☔').is_err());
/// ```
///
/// ```ignore
/// extern "C" fn have_a_byte(b: libc::c_char);
///
/// have_a_byte(Char::from(b'a').into_glib());
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct Char(pub c_char);

impl TryFrom<char> for Char {
    type Error = TryFromIntError;

    #[allow(clippy::unnecessary_cast)]
    fn try_from(c: char) -> Result<Char, Self::Error> {
        Ok(Self(u8::try_from(u32::from(c))? as c_char))
    }
}

impl From<Char> for char {
    fn from(c: Char) -> char {
        c.0 as u8 as char
    }
}

impl From<u8> for Char {
    #[allow(clippy::unnecessary_cast)]
    fn from(c: u8) -> Char {
        Char(c as c_char)
    }
}

impl From<Char> for u8 {
    #[allow(clippy::unnecessary_cast)]
    fn from(c: Char) -> u8 {
        c.0 as u8
    }
}

#[doc(hidden)]
impl FromGlib<c_char> for Char {
    #[inline]
    unsafe fn from_glib(value: c_char) -> Self {
        Self(value)
    }
}

#[doc(hidden)]
impl IntoGlib for Char {
    type GlibType = c_char;

    #[inline]
    fn into_glib(self) -> c_char {
        self.0
    }
}

// rustdoc-stripper-ignore-next
/// Wrapper for values where C functions expect a plain C `unsigned char`
///
/// This `UChar` type is a wrapper over an `libc::c_uchar`, so that we can pass it to Glib or C functions.
/// The check for whether a Rust `char` (a Unicode scalar value) actually fits in a `libc::c_uchar` is
/// done in the `new` function; see its documentation for details.
///
/// The inner `libc::c_uchar` (which is equivalent to `u8`) can be extracted with `.0`, or
/// by calling `my_char.into_glib()`.
///
/// # Examples
/// ```
/// use glib::UChar;
/// use std::convert::TryFrom;
///
/// UChar::from(b'a');
/// UChar::try_from('a').unwrap();
/// assert!(UChar::try_from('☔').is_err());
/// ```
///
/// ```ignore
/// extern "C" fn have_a_byte(b: libc::c_uchar);
///
/// have_a_byte(UChar::from(b'a').into_glib());
/// ```
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
pub struct UChar(pub c_uchar);

impl TryFrom<char> for UChar {
    type Error = TryFromIntError;

    #[allow(clippy::unnecessary_cast)]
    fn try_from(c: char) -> Result<UChar, Self::Error> {
        Ok(Self(u8::try_from(u32::from(c))? as c_uchar))
    }
}

impl From<UChar> for char {
    fn from(c: UChar) -> char {
        c.0 as _
    }
}

impl From<u8> for UChar {
    #[allow(clippy::unnecessary_cast)]
    fn from(c: u8) -> UChar {
        UChar(c as _)
    }
}

impl From<UChar> for u8 {
    fn from(c: UChar) -> u8 {
        c.0 as _
    }
}

#[doc(hidden)]
impl FromGlib<c_uchar> for UChar {
    #[inline]
    unsafe fn from_glib(value: c_uchar) -> Self {
        Self(value)
    }
}

#[doc(hidden)]
impl IntoGlib for UChar {
    type GlibType = c_uchar;

    #[inline]
    fn into_glib(self) -> c_uchar {
        self.0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[allow(clippy::unnecessary_cast)]
    fn converts_single_byte_chars() {
        assert_eq!(Char::try_from(0 as char), Ok(Char(0 as c_char)));
        assert_eq!(UChar::try_from(0 as char), Ok(UChar(0 as c_uchar)));
        assert_eq!(UChar::try_from(255 as char), Ok(UChar(255 as c_uchar)));
        assert_eq!(UChar::try_from('ñ'), Ok(UChar(241 as c_uchar)));
    }

    #[test]
    fn refuses_multibyte_chars() {
        assert!(Char::try_from('☔').is_err()); // no umbrella for you
        assert!(UChar::try_from('☔').is_err());
    }

    #[test]
    #[allow(clippy::unnecessary_cast)]
    fn into_i8() {
        assert_eq!(Char::from(b'A').into_glib(), 65 as c_char);
    }

    #[test]
    #[allow(clippy::unnecessary_cast)]
    fn into_u8() {
        assert_eq!(UChar::from(b'A').into_glib(), 65 as c_uchar);
    }

    #[test]
    #[allow(clippy::unnecessary_cast)]
    fn into_char() {
        assert_eq!(char::from(Char(65 as c_char)), 'A');
        assert_eq!('ñ', UChar(241 as c_uchar).into());
    }

    #[test]
    #[allow(clippy::unnecessary_cast)]
    fn convert_from_glib() {
        assert_eq!(Char(65 as c_char), unsafe { from_glib(65 as c_char) });
        assert_eq!(UChar(241 as c_uchar), unsafe { from_glib(241 as c_uchar) });
    }
}
