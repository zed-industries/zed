use std::{
    ffi::OsString,
    io::{Error, ErrorKind},
};

use crate::Result;

pub(crate) fn string_from_os(string: OsString) -> Result<String> {
    #[cfg(any(
        all(not(target_os = "windows"), not(target_arch = "wasm32")),
        all(target_arch = "wasm32", target_os = "wasi"),
    ))]
    {
        #[cfg(not(target_os = "wasi"))]
        use std::os::unix::ffi::OsStringExt;
        #[cfg(target_os = "wasi")]
        use std::os::wasi::ffi::OsStringExt;

        String::from_utf8(string.into_vec())
            .map_err(|e| Error::new(ErrorKind::InvalidData, e))
    }

    #[cfg(any(
        target_os = "windows",
        all(target_arch = "wasm32", not(target_os = "wasi")),
    ))]
    {
        string.into_string().map_err(|_| {
            Error::new(ErrorKind::InvalidData, "Not valid unicode")
        })
    }
}
