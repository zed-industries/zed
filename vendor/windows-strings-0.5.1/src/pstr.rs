use super::*;

/// A pointer to a null-terminated string of 8-bit Windows (ANSI) characters.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PSTR(pub *mut u8);

impl PSTR {
    /// Construct a new `PSTR` from a raw pointer
    pub const fn from_raw(ptr: *mut u8) -> Self {
        Self(ptr)
    }

    /// Construct a null `PSTR`
    pub const fn null() -> Self {
        Self(core::ptr::null_mut())
    }

    /// Returns a raw pointer to the `PSTR`
    pub const fn as_ptr(&self) -> *mut u8 {
        self.0
    }

    /// Checks whether the `PSTR` is null
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// String data without the trailing 0
    ///
    /// # Safety
    ///
    /// The `PSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn as_bytes(&self) -> &[u8] {
        unsafe {
            let len = strlen(PCSTR::from_raw(self.0));
            core::slice::from_raw_parts(self.0, len)
        }
    }

    /// Copy the `PSTR` into a Rust `String`.
    ///
    /// # Safety
    ///
    /// See the safety information for `PSTR::as_bytes`.
    pub unsafe fn to_string(&self) -> core::result::Result<String, alloc::string::FromUtf8Error> {
        unsafe { String::from_utf8(self.as_bytes().into()) }
    }

    /// Allow this string to be displayed.
    ///
    /// # Safety
    ///
    /// See the safety information for `PSTR::as_bytes`.
    pub unsafe fn display(&self) -> impl core::fmt::Display + '_ {
        unsafe { Decode(move || decode_utf8(self.as_bytes())) }
    }
}

impl Default for PSTR {
    fn default() -> Self {
        Self::null()
    }
}
