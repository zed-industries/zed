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
        Self(std::ptr::null_mut())
    }

    /// Returns a raw pointer to the `PWSTR`.
    pub const fn as_ptr(&self) -> *mut u16 {
        self.0
    }

    /// Checks whether the `PWSTR` is null.
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// String data without the trailing 0.
    ///
    /// # Safety
    ///
    /// The `PWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn as_wide(&self) -> &[u16] {
        let len = wcslen(PCWSTR::from_raw(self.0));
        std::slice::from_raw_parts(self.0, len)
    }

    /// Copy the `PWSTR` into a Rust `String`.
    ///
    /// # Safety
    ///
    /// See the safety information for `PWSTR::as_wide`.
    pub unsafe fn to_string(&self) -> std::result::Result<String, std::string::FromUtf16Error> {
        String::from_utf16(self.as_wide())
    }

    /// Copy the `PWSTR` into an `HSTRING`.
    ///
    /// # Safety
    ///
    /// See the safety information for `PWSTR::as_wide`.
    pub unsafe fn to_hstring(&self) -> Result<HSTRING> {
        HSTRING::from_wide(self.as_wide())
    }

    /// Allow this string to be displayed.
    ///
    /// # Safety
    ///
    /// See the safety information for `PWSTR::as_wide`.
    pub unsafe fn display(&self) -> impl std::fmt::Display + '_ {
        Decode(move || std::char::decode_utf16(self.as_wide().iter().cloned()))
    }
}
