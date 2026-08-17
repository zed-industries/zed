use super::*;

/// A pointer to a null-terminated string of 16-bit Unicode characters.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PWSTR(pub *mut u16);

impl PWSTR {
    /// Construct a new `PWSTR` from a raw pointer.
    pub const fn from_raw(ptr: *mut u16) -> Self {
        Self(ptr)
    }

    /// Construct a null `PWSTR`.
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }

    /// Returns a raw pointer to the `PWSTR`.
    pub const fn as_ptr(&self) -> *mut u16 {
        self.0
    }

    /// Checks whether the `PWSTR` is null.
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// String length without the trailing 0
    ///
    /// # Safety
    ///
    /// The `PWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn len(&self) -> usize {
        unsafe { PCWSTR(self.0).len() }
    }

    /// Returns `true` if the string length is zero, and `false` otherwise.
    ///
    /// # Safety
    ///
    /// The `PWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn is_empty(&self) -> bool {
        unsafe { self.len() == 0 }
    }

    /// String data without the trailing 0.
    ///
    /// # Safety
    ///
    /// The `PWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn as_wide(&self) -> &[u16] {
        unsafe { core::slice::from_raw_parts(self.0, self.len()) }
    }

    /// Copy the `PWSTR` into a Rust `String`.
    ///
    /// # Safety
    ///
    /// See the safety information for `PWSTR::as_wide`.
    pub unsafe fn to_string(&self) -> core::result::Result<String, alloc::string::FromUtf16Error> {
        unsafe { String::from_utf16(self.as_wide()) }
    }

    /// Copy the `PWSTR` into an `HSTRING`.
    ///
    /// # Safety
    ///
    /// See the safety information for `PWSTR::as_wide`.
    pub unsafe fn to_hstring(&self) -> HSTRING {
        unsafe { HSTRING::from_wide(self.as_wide()) }
    }

    /// Allow this string to be displayed.
    ///
    /// # Safety
    ///
    /// See the safety information for `PWSTR::as_wide`.
    pub unsafe fn display(&self) -> impl core::fmt::Display + '_ {
        unsafe { Decode(move || core::char::decode_utf16(self.as_wide().iter().cloned())) }
    }
}

impl Default for PWSTR {
    fn default() -> Self {
        Self::null()
    }
}
