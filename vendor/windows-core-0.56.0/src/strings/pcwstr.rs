use super::*;

/// A pointer to a constant null-terminated string of 16-bit Unicode characters.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PCWSTR(pub *const u16);

impl PCWSTR {
    /// Construct a new `PCWSTR` from a raw pointer
    pub const fn from_raw(ptr: *const u16) -> Self {
        Self(ptr)
    }

    /// Construct a null `PCWSTR`
    pub const fn null() -> Self {
        Self(std::ptr::null())
    }

    /// Returns a raw pointer to the `PCWSTR`
    pub const fn as_ptr(&self) -> *const u16 {
        self.0
    }

    /// Checks whether the `PCWSTR` is null
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// String data without the trailing 0
    ///
    /// # Safety
    ///
    /// The `PCWSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn as_wide(&self) -> &[u16] {
        let len = wcslen(*self);
        std::slice::from_raw_parts(self.0, len)
    }

    /// Copy the `PCWSTR` into a Rust `String`.
    ///
    /// # Safety
    ///
    /// See the safety information for `PCWSTR::as_wide`.
    pub unsafe fn to_string(&self) -> std::result::Result<String, std::string::FromUtf16Error> {
        String::from_utf16(self.as_wide())
    }

    /// Copy the `PCWSTR` into an `HSTRING`.
    ///
    /// # Safety
    ///
    /// See the safety information for `PCWSTR::as_wide`.
    pub unsafe fn to_hstring(&self) -> Result<HSTRING> {
        HSTRING::from_wide(self.as_wide())
    }

    /// Allow this string to be displayed.
    ///
    /// # Safety
    ///
    /// See the safety information for `PCWSTR::as_wide`.
    pub unsafe fn display(&self) -> impl std::fmt::Display + '_ {
        Decode(move || std::char::decode_utf16(self.as_wide().iter().cloned()))
    }
}
