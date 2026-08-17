// Take a look at the license at the top of the repository in the LICENSE file.

use std::{fmt, io, os::raw::c_char, path::PathBuf, ptr};

use crate::{ffi, translate::*, ConvertError, Error, GString, NormalizeMode, Slice};

// rustdoc-stripper-ignore-next
/// A wrapper for [`ConvertError`](crate::ConvertError) that can hold an offset into the input
/// string.
#[derive(Debug)]
pub enum CvtError {
    Convert(Error),
    IllegalSequence { source: Error, offset: usize },
}

impl std::error::Error for CvtError {
    fn source(&self) -> ::core::option::Option<&(dyn std::error::Error + 'static)> {
        match self {
            CvtError::Convert(err) => std::error::Error::source(err),
            CvtError::IllegalSequence { source, .. } => Some(source),
        }
    }
}

impl fmt::Display for CvtError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> ::core::fmt::Result {
        match self {
            CvtError::Convert(err) => fmt::Display::fmt(err, fmt),
            CvtError::IllegalSequence { source, offset } => {
                write!(fmt, "{source} at offset {offset}")
            }
        }
    }
}

impl std::convert::From<Error> for CvtError {
    fn from(err: Error) -> Self {
        CvtError::Convert(err)
    }
}

impl CvtError {
    #[inline]
    fn new(err: Error, bytes_read: usize) -> Self {
        if err.kind::<ConvertError>() == Some(ConvertError::IllegalSequence) {
            Self::IllegalSequence {
                source: err,
                offset: bytes_read,
            }
        } else {
            err.into()
        }
    }
}

#[doc(alias = "g_convert")]
pub fn convert(
    str_: &[u8],
    to_codeset: impl IntoGStr,
    from_codeset: impl IntoGStr,
) -> Result<(Slice<u8>, usize), CvtError> {
    assert!(str_.len() <= isize::MAX as usize);
    let mut bytes_read = 0;
    let mut bytes_written = 0;
    let mut error = ptr::null_mut();
    let result = to_codeset.run_with_gstr(|to_codeset| {
        from_codeset.run_with_gstr(|from_codeset| unsafe {
            ffi::g_convert(
                str_.as_ptr(),
                str_.len() as isize,
                to_codeset.to_glib_none().0,
                from_codeset.to_glib_none().0,
                &mut bytes_read,
                &mut bytes_written,
                &mut error,
            )
        })
    });
    if result.is_null() {
        Err(CvtError::new(unsafe { from_glib_full(error) }, bytes_read))
    } else {
        let slice = unsafe { Slice::from_glib_full_num(result, bytes_written as _) };
        Ok((slice, bytes_read))
    }
}

#[doc(alias = "g_convert_with_fallback")]
pub fn convert_with_fallback(
    str_: &[u8],
    to_codeset: impl IntoGStr,
    from_codeset: impl IntoGStr,
    fallback: Option<impl IntoGStr>,
) -> Result<(Slice<u8>, usize), CvtError> {
    assert!(str_.len() <= isize::MAX as usize);
    let mut bytes_read = 0;
    let mut bytes_written = 0;
    let mut error = ptr::null_mut();
    let result = to_codeset.run_with_gstr(|to_codeset| {
        from_codeset.run_with_gstr(|from_codeset| {
            fallback.run_with_gstr(|fallback| unsafe {
                ffi::g_convert_with_fallback(
                    str_.as_ptr(),
                    str_.len() as isize,
                    to_codeset.to_glib_none().0,
                    from_codeset.to_glib_none().0,
                    fallback.to_glib_none().0,
                    &mut bytes_read,
                    &mut bytes_written,
                    &mut error,
                )
            })
        })
    });
    if result.is_null() {
        Err(CvtError::new(unsafe { from_glib_full(error) }, bytes_read))
    } else {
        let slice = unsafe { Slice::from_glib_full_num(result, bytes_written as _) };
        Ok((slice, bytes_read))
    }
}

// rustdoc-stripper-ignore-next
/// A wrapper for [`std::io::Error`] that can hold an offset into an input string.
#[derive(Debug)]
pub enum IConvError {
    Error(io::Error),
    WithOffset { source: io::Error, offset: usize },
}

impl std::error::Error for IConvError {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            IConvError::Error(err) => std::error::Error::source(err),
            IConvError::WithOffset { source, .. } => Some(source),
        }
    }
}

impl fmt::Display for IConvError {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        match self {
            IConvError::Error(err) => fmt::Display::fmt(err, fmt),
            IConvError::WithOffset { source, offset } => write!(fmt, "{source} at offset {offset}"),
        }
    }
}

impl std::convert::From<io::Error> for IConvError {
    fn from(err: io::Error) -> Self {
        IConvError::Error(err)
    }
}

#[derive(Debug)]
#[repr(transparent)]
#[doc(alias = "GIConv")]
pub struct IConv(ffi::GIConv);

unsafe impl Send for IConv {}

impl IConv {
    #[doc(alias = "g_iconv_open")]
    #[allow(clippy::unnecessary_lazy_evaluations)]
    pub fn new(to_codeset: impl IntoGStr, from_codeset: impl IntoGStr) -> Option<Self> {
        let iconv = to_codeset.run_with_gstr(|to_codeset| {
            from_codeset.run_with_gstr(|from_codeset| unsafe {
                ffi::g_iconv_open(to_codeset.to_glib_none().0, from_codeset.to_glib_none().0)
            })
        });
        (iconv as isize != -1).then(|| Self(iconv))
    }
    #[doc(alias = "g_convert_with_iconv")]
    pub fn convert(&mut self, str_: &[u8]) -> Result<(Slice<u8>, usize), CvtError> {
        assert!(str_.len() <= isize::MAX as usize);
        let mut bytes_read = 0;
        let mut bytes_written = 0;
        let mut error = ptr::null_mut();
        let result = unsafe {
            ffi::g_convert_with_iconv(
                str_.as_ptr(),
                str_.len() as isize,
                self.0,
                &mut bytes_read,
                &mut bytes_written,
                &mut error,
            )
        };
        if result.is_null() {
            Err(CvtError::new(unsafe { from_glib_full(error) }, bytes_read))
        } else {
            let slice = unsafe { Slice::from_glib_full_num(result, bytes_written as _) };
            Ok((slice, bytes_read))
        }
    }
    #[doc(alias = "g_iconv")]
    pub fn iconv(
        &mut self,
        inbuf: Option<&[u8]>,
        outbuf: Option<&mut [std::mem::MaybeUninit<u8>]>,
    ) -> Result<(usize, usize, usize), IConvError> {
        let input_len = inbuf.as_ref().map(|b| b.len()).unwrap_or_default();
        let mut inbytes_left = input_len;
        let mut outbytes_left = outbuf.as_ref().map(|b| b.len()).unwrap_or_default();
        let mut inbuf = inbuf
            .map(|b| mut_override(b.as_ptr()) as *mut c_char)
            .unwrap_or_else(ptr::null_mut);
        let mut outbuf = outbuf
            .map(|b| b.as_mut_ptr() as *mut c_char)
            .unwrap_or_else(ptr::null_mut);
        let conversions = unsafe {
            ffi::g_iconv(
                self.0,
                &mut inbuf,
                &mut inbytes_left,
                &mut outbuf,
                &mut outbytes_left,
            )
        };
        if conversions as isize == -1 {
            let err = io::Error::last_os_error();
            let code = err.raw_os_error().unwrap();
            if code == libc::EILSEQ || code == libc::EINVAL {
                Err(IConvError::WithOffset {
                    source: err,
                    offset: input_len - inbytes_left,
                })
            } else {
                Err(err.into())
            }
        } else {
            Ok((conversions, inbytes_left, outbytes_left))
        }
    }
}

impl Drop for IConv {
    #[inline]
    fn drop(&mut self) {
        unsafe {
            ffi::g_iconv_close(self.0);
        }
    }
}

#[doc(alias = "g_get_filename_charsets")]
#[doc(alias = "get_filename_charsets")]
pub fn filename_charsets() -> (bool, Vec<GString>) {
    let mut filename_charsets = ptr::null_mut();
    unsafe {
        let is_utf8 = ffi::g_get_filename_charsets(&mut filename_charsets);
        (
            from_glib(is_utf8),
            FromGlibPtrContainer::from_glib_none(filename_charsets),
        )
    }
}

#[doc(alias = "g_filename_from_utf8")]
pub fn filename_from_utf8(utf8string: impl IntoGStr) -> Result<(PathBuf, usize), CvtError> {
    let mut bytes_read = 0;
    let mut bytes_written = std::mem::MaybeUninit::uninit();
    let mut error = ptr::null_mut();
    let ret = utf8string.run_with_gstr(|utf8string| {
        assert!(utf8string.len() <= isize::MAX as usize);
        let len = utf8string.len() as isize;
        unsafe {
            ffi::g_filename_from_utf8(
                utf8string.to_glib_none().0,
                len,
                &mut bytes_read,
                bytes_written.as_mut_ptr(),
                &mut error,
            )
        }
    });
    if error.is_null() {
        Ok(unsafe {
            (
                PathBuf::from_glib_full_num(ret, bytes_written.assume_init()),
                bytes_read,
            )
        })
    } else {
        Err(unsafe { CvtError::new(from_glib_full(error), bytes_read) })
    }
}

#[doc(alias = "g_filename_to_utf8")]
pub fn filename_to_utf8(
    opsysstring: impl AsRef<std::path::Path>,
) -> Result<(crate::GString, usize), CvtError> {
    let path = opsysstring.as_ref().to_glib_none();
    let mut bytes_read = 0;
    let mut bytes_written = std::mem::MaybeUninit::uninit();
    let mut error = ptr::null_mut();
    let ret = unsafe {
        ffi::g_filename_to_utf8(
            path.0,
            path.1.as_bytes().len() as isize,
            &mut bytes_read,
            bytes_written.as_mut_ptr(),
            &mut error,
        )
    };
    if error.is_null() {
        Ok(unsafe {
            (
                GString::from_glib_full_num(ret, bytes_written.assume_init()),
                bytes_read,
            )
        })
    } else {
        Err(unsafe { CvtError::new(from_glib_full(error), bytes_read) })
    }
}

#[doc(alias = "g_locale_from_utf8")]
pub fn locale_from_utf8(utf8string: impl IntoGStr) -> Result<(Slice<u8>, usize), CvtError> {
    let mut bytes_read = 0;
    let mut bytes_written = std::mem::MaybeUninit::uninit();
    let mut error = ptr::null_mut();
    let ret = utf8string.run_with_gstr(|utf8string| {
        assert!(utf8string.len() <= isize::MAX as usize);
        unsafe {
            ffi::g_locale_from_utf8(
                utf8string.as_ptr(),
                utf8string.len() as isize,
                &mut bytes_read,
                bytes_written.as_mut_ptr(),
                &mut error,
            )
        }
    });
    if error.is_null() {
        Ok(unsafe {
            (
                Slice::from_glib_full_num(ret, bytes_written.assume_init() + 1),
                bytes_read,
            )
        })
    } else {
        Err(unsafe { CvtError::new(from_glib_full(error), bytes_read) })
    }
}

#[doc(alias = "g_locale_to_utf8")]
pub fn locale_to_utf8(opsysstring: &[u8]) -> Result<(crate::GString, usize), CvtError> {
    let len = opsysstring.len() as isize;
    let mut bytes_read = 0;
    let mut bytes_written = std::mem::MaybeUninit::uninit();
    let mut error = ptr::null_mut();
    let ret = unsafe {
        ffi::g_locale_to_utf8(
            opsysstring.to_glib_none().0,
            len,
            &mut bytes_read,
            bytes_written.as_mut_ptr(),
            &mut error,
        )
    };
    if error.is_null() {
        Ok(unsafe {
            (
                GString::from_glib_full_num(ret, bytes_written.assume_init()),
                bytes_read,
            )
        })
    } else {
        Err(unsafe { CvtError::new(from_glib_full(error), bytes_read) })
    }
}

#[doc(alias = "g_utf8_to_ucs4")]
#[doc(alias = "g_utf8_to_ucs4_fast")]
#[doc(alias = "utf8_to_ucs4")]
pub fn utf8_to_utf32(str: impl AsRef<str>) -> Slice<char> {
    unsafe {
        let mut items_written = 0;

        let str_as_utf32 = ffi::g_utf8_to_ucs4_fast(
            str.as_ref().as_ptr().cast::<c_char>(),
            str.as_ref().len() as _,
            &mut items_written,
        );

        // NOTE: We assume that u32 and char have the same layout and trust that glib won't give us
        //       invalid UTF-32 codepoints
        Slice::from_glib_full_num(str_as_utf32, items_written as usize)
    }
}

#[doc(alias = "g_ucs4_to_utf8")]
#[doc(alias = "ucs4_to_utf8")]
pub fn utf32_to_utf8(str: impl AsRef<[char]>) -> GString {
    let mut items_read = 0;
    let mut items_written = 0;
    let mut error = ptr::null_mut();

    unsafe {
        let str_as_utf8 = ffi::g_ucs4_to_utf8(
            str.as_ref().as_ptr().cast::<u32>(),
            str.as_ref().len() as _,
            &mut items_read,
            &mut items_written,
            &mut error,
        );

        debug_assert!(
            error.is_null(),
            "Rust `char` should always be convertible to UTF-8"
        );

        GString::from_glib_full_num(str_as_utf8, items_written as usize)
    }
}

#[doc(alias = "g_utf8_casefold")]
#[doc(alias = "utf8_casefold")]
pub fn casefold(str: impl AsRef<str>) -> GString {
    unsafe {
        let str = ffi::g_utf8_casefold(str.as_ref().as_ptr().cast(), str.as_ref().len() as isize);

        from_glib_full(str)
    }
}

#[doc(alias = "g_utf8_normalize")]
#[doc(alias = "utf8_normalize")]
pub fn normalize(str: impl AsRef<str>, mode: NormalizeMode) -> GString {
    unsafe {
        let str = ffi::g_utf8_normalize(
            str.as_ref().as_ptr().cast(),
            str.as_ref().len() as isize,
            mode.into_glib(),
        );

        from_glib_full(str)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn convert_ascii() {
        assert!(super::convert(b"Hello", "utf-8", "ascii").is_ok());
        assert!(super::convert(b"He\xaallo", "utf-8", "ascii").is_err());
        assert_eq!(
            super::convert_with_fallback(b"H\xc3\xa9llo", "ascii", "utf-8", crate::NONE_STR)
                .unwrap()
                .0
                .as_slice(),
            b"H\\u00e9llo"
        );
        assert_eq!(
            super::convert_with_fallback(b"H\xc3\xa9llo", "ascii", "utf-8", Some("_"))
                .unwrap()
                .0
                .as_slice(),
            b"H_llo"
        );
    }
    #[test]
    fn iconv() {
        let mut conv = super::IConv::new("utf-8", "ascii").unwrap();
        assert!(conv.convert(b"Hello").is_ok());
        assert!(conv.convert(b"He\xaallo").is_err());
        assert!(super::IConv::new("utf-8", "badcharset123456789").is_none());
    }
    #[test]
    fn filename_charsets() {
        let _ = super::filename_charsets();
    }

    #[test]
    fn utf8_and_utf32() {
        let utf32 = ['A', 'b', '🤔'];
        let utf8 = super::utf32_to_utf8(utf32);
        assert_eq!(utf8, "Ab🤔");

        let utf8 = "🤔 ț";
        let utf32 = super::utf8_to_utf32(utf8);
        assert_eq!(utf32.as_slice(), &['🤔', ' ', 'ț']);
    }
}
