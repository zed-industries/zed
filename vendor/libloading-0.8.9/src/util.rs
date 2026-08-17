use std::borrow::Cow;
use std::ffi::{CStr, CString};
use std::os::raw;

use crate::Error;

/// Checks for the last byte and avoids allocating if it is zero.
///
/// Non-last null bytes still result in an error.
pub(crate) fn cstr_cow_from_bytes(slice: &[u8]) -> Result<Cow<'_, CStr>, Error> {
    static ZERO: raw::c_char = 0;
    Ok(match slice.last() {
        // Slice out of 0 elements
        None => unsafe { Cow::Borrowed(CStr::from_ptr(&ZERO)) },
        // Slice with trailing 0
        Some(&0) => Cow::Borrowed(
            CStr::from_bytes_with_nul(slice)
                .map_err(|source| Error::CreateCStringWithTrailing { source })?,
        ),
        // Slice with no trailing 0
        Some(_) => {
            Cow::Owned(CString::new(slice).map_err(|source| Error::CreateCString { source })?)
        }
    })
}

#[inline]
pub(crate) fn ensure_compatible_types<T, E>() -> Result<(), Error> {
    if ::std::mem::size_of::<T>() != ::std::mem::size_of::<E>() {
        Err(Error::IncompatibleSize)
    } else {
        Ok(())
    }
}
