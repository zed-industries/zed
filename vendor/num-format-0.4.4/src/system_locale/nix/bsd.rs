#![cfg(all(
    feature = "with-system-locale",
    unix,
    any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "openbsd",
        target_os = "netbsd"
    )
))]

use libc::{c_char, c_int, c_void};

use crate::error::Error;
use crate::system_locale::nix::{Encoding, Lconv, StaticCString, UTF_8};

extern "C" {
    fn localeconv_l(locale: *const c_void) -> *const libc::lconv;
    fn nl_langinfo_l(item: libc::nl_item, locale: *const c_void) -> *const c_char;
    fn querylocale(mask: c_int, locale: *const c_void) -> *const c_char;
}

pub(crate) fn get_encoding(locale: *const c_void) -> Result<Encoding, Error> {
    let encoding_ptr = unsafe { nl_langinfo_l(libc::CODESET, locale) };
    let encoding_static_c_string = StaticCString::new(encoding_ptr, *UTF_8, "nl_langinfo_l")?;
    let encoding_string = encoding_static_c_string.to_string()?;
    let encoding = Encoding::from_bytes(encoding_string.as_bytes())?;
    Ok(encoding)
}

pub(crate) fn get_lconv(locale: *const c_void, encoding: Encoding) -> Result<Lconv, Error> {
    let lconv_ptr = unsafe { localeconv_l(locale) };
    if lconv_ptr.is_null() {
        return Err(Error::system_invalid_return(
            "localeconv_l",
            "localeconv_l unexpectedly returned a null pointer.",
        ));
    }
    let lconv: &libc::lconv = unsafe { lconv_ptr.as_ref() }.unwrap();
    let lconv = Lconv::new(lconv, encoding)?;
    Ok(lconv)
}

pub(crate) fn get_name(locale: *const c_void, encoding: Encoding) -> Result<String, Error> {
    let mask = libc::LC_CTYPE_MASK | libc::LC_MONETARY_MASK | libc::LC_NUMERIC_MASK;
    let name_ptr = unsafe { querylocale(mask, locale) };
    let name_static_c_string = StaticCString::new(name_ptr, encoding, "querylocale")?;
    let name = name_static_c_string.to_string()?;
    Ok(name)
}
