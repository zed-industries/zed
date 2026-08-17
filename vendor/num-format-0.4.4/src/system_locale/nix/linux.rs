#![cfg(all(
    feature = "with-system-locale",
    unix,
    not(any(
        target_os = "dragonfly",
        target_os = "freebsd",
        target_os = "ios",
        target_os = "macos",
        target_os = "openbsd",
        target_os = "netbsd"
    ))
))]

use std::env;

use libc::{c_char, c_void};

use crate::error::Error;
use crate::system_locale::nix::{Encoding, Lconv, StaticCString, UTF_8};

extern "C" {
    fn localeconv() -> *const libc::lconv;
    fn nl_langinfo(item: libc::nl_item) -> *const c_char;
}

pub(crate) fn get_encoding(_locale: *const c_void) -> Result<Encoding, Error> {
    let encoding_ptr = unsafe { nl_langinfo(libc::CODESET) };
    let encoding_static_c_string = StaticCString::new(encoding_ptr, *UTF_8, "nl_langinfo")?;
    let encoding_string = encoding_static_c_string.to_string()?;
    let encoding = Encoding::from_bytes(encoding_string.as_bytes())?;
    Ok(encoding)
}

pub(crate) fn get_lconv(_locale: *const c_void, encoding: Encoding) -> Result<Lconv, Error> {
    let lconv_ptr = unsafe { localeconv() };
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

pub(crate) fn get_name(_locale: *const c_void, _encoding: Encoding) -> Result<String, Error> {
    if let Ok(name) = env::var("LC_ALL") {
        return Ok(name);
    }
    if let Ok(name) = env::var("LC_NUMERIC") {
        return Ok(name);
    }
    if let Ok(name) = env::var("LC_MONETARY") {
        return Ok(name);
    }
    if let Ok(name) = env::var("LANG") {
        return Ok(name);
    }
    Ok("C".to_string())
}
