use crate::{AsciiCase, Error, Out, AsOut};

use core::fmt;
use core::mem::MaybeUninit;

use uuid::Uuid;

/// An extension trait for [`uuid::Uuid`]
#[cfg_attr(docsrs, doc(cfg(feature = "uuid")))]
pub trait UuidExt: Sized {
    /// Parses an UUID from arbitrary bytes.
    ///
    /// # Errors
    /// This function returns `Err` if:
    ///
    /// + The length of `src` doesn't match any UUID format variants.
    /// + The content of `src` is invalid.
    ///
    fn parse(src: impl AsRef<[u8]>) -> Result<Self, Error>;

    /// Parses a simple UUID from arbitrary bytes.
    ///
    /// # Errors
    /// This function returns `Err` if:
    ///
    /// + The length of `src` doesn't match the "simple" format.
    /// + The content of `src` is invalid.
    ///
    fn parse_simple(src: impl AsRef<[u8]>) -> Result<Self, Error>;

    /// Parses a hyphenated UUID from arbitrary bytes.
    ///
    /// # Errors
    /// This function returns `Err` if:
    ///
    /// + The length of `src` doesn't match the "hyphenated" format.
    /// + The content of `src` is invalid.
    ///
    fn parse_hyphenated(src: impl AsRef<[u8]>) -> Result<Self, Error>;

    /// Returns a fmt adapter with "simple" format.
    fn format_simple(&self) -> Simple<'_>;

    /// Returns a fmt adapter with "hyphenated" format.
    fn format_hyphenated(&self) -> Hyphenated<'_>;
}

#[allow(clippy::type_complexity)]
#[inline(always)]
unsafe fn parse_uuid(
    src: &[u8],
    f: for<'s, 'd> fn(&'s [u8], Out<'d, [u8; 16]>) -> Result<&'d mut [u8; 16], Error>,
) -> Result<Uuid, Error> {
    let mut uuid = MaybeUninit::<Uuid>::uninit();
    let out = Out::new(uuid.as_mut_ptr().cast());
    f(src, out)?;
    Ok(uuid.assume_init())
}

impl UuidExt for Uuid {
    #[inline]
    fn parse(src: impl AsRef<[u8]>) -> Result<Self, Error> {
        unsafe { parse_uuid(src.as_ref(), crate::parse) }
    }

    #[inline]
    fn parse_simple(src: impl AsRef<[u8]>) -> Result<Self, Error> {
        unsafe { parse_uuid(src.as_ref(), crate::parse_simple) }
    }

    #[inline]
    fn parse_hyphenated(src: impl AsRef<[u8]>) -> Result<Self, Error> {
        unsafe { parse_uuid(src.as_ref(), crate::parse_hyphenated) }
    }

    #[inline]
    fn format_simple(&self) -> Simple<'_> {
        Simple(self)
    }

    #[inline]
    fn format_hyphenated(&self) -> Hyphenated<'_> {
        Hyphenated(self)
    }
}

/// A simple UUID
#[cfg_attr(docsrs, doc(cfg(feature = "uuid")))]
#[derive(Debug)]
pub struct Simple<'a>(&'a Uuid);

/// A hyphenated UUID
#[cfg_attr(docsrs, doc(cfg(feature = "uuid")))]
#[derive(Debug)]
pub struct Hyphenated<'a>(&'a Uuid);

#[allow(clippy::type_complexity)]
#[inline]
unsafe fn format_uuid<R, const N: usize>(
    uuid: &Uuid,
    case: AsciiCase,
    f: for<'s, 'd> fn(&'s [u8; 16], Out<'d, [u8; N]>, case: AsciiCase) -> &'d mut [u8; N],
    g: impl FnOnce(&str) -> R,
) -> R {
    let mut buf = MaybeUninit::<[u8; N]>::uninit();
    let src = uuid.as_bytes();
    let dst = buf.as_out();
    let ans = f(src, dst, case);
    g(core::str::from_utf8_unchecked(ans))
}

impl fmt::LowerHex for Simple<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let case = AsciiCase::Lower;
        unsafe { format_uuid(self.0, case, crate::format_simple, |s| <str as fmt::Display>::fmt(s, f)) }
    }
}

impl fmt::LowerHex for Hyphenated<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let case = AsciiCase::Lower;
        unsafe {
            format_uuid(self.0, case, crate::format_hyphenated, |s| {
                <str as fmt::Display>::fmt(s, f)
            })
        }
    }
}

impl fmt::UpperHex for Simple<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let case = AsciiCase::Upper;
        unsafe { format_uuid(self.0, case, crate::format_simple, |s| <str as fmt::Display>::fmt(s, f)) }
    }
}

impl fmt::UpperHex for Hyphenated<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let case = AsciiCase::Upper;
        unsafe {
            format_uuid(self.0, case, crate::format_hyphenated, |s| {
                <str as fmt::Display>::fmt(s, f)
            })
        }
    }
}

impl fmt::Display for Simple<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::LowerHex>::fmt(self, f)
    }
}

impl fmt::Display for Hyphenated<'_> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <Self as fmt::LowerHex>::fmt(self, f)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[cfg_attr(not(target_arch = "wasm32"), test)]
    #[cfg_attr(target_arch = "wasm32", wasm_bindgen_test::wasm_bindgen_test)]
    fn test_uuid_ext() {
        let s1 = "67e5504410b1426f9247bb680e5fe0c8";
        let s2 = "67e55044-10b1-426f-9247-bb680e5fe0c8";

        let u = Uuid::parse(s1).unwrap();

        let a1 = u.format_simple().to_string();
        let a2 = format!("{:X}", u.format_hyphenated());

        assert_eq!(a1, s1);
        assert_eq!(a2, s2.to_ascii_uppercase());
    }
}
