use super::*;

/// A pointer to a constant null-terminated string of 8-bit Windows (ANSI) characters.
#[repr(transparent)]
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct PCSTR(pub *const u8);

impl PCSTR {
    /// Construct a new `PCSTR` from a raw pointer
    pub const fn from_raw(ptr: *const u8) -> Self {
        Self(ptr)
    }

    /// Construct a null `PCSTR`
    pub const fn null() -> Self {
        Self(core::ptr::null())
    }

    /// Returns a raw pointer to the `PCSTR`
    pub const fn as_ptr(&self) -> *const u8 {
        self.0
    }

    /// Checks whether the `PCSTR` is null
    pub fn is_null(&self) -> bool {
        self.0.is_null()
    }

    /// String data without the trailing 0
    ///
    /// # Safety
    ///
    /// The `PCSTR`'s pointer needs to be valid for reads up until and including the next `\0`.
    pub unsafe fn as_bytes(&self) -> &[u8] {
        unsafe {
            let len = strlen(*self);
            core::slice::from_raw_parts(self.0, len)
        }
    }

    /// Copy the `PCSTR` into a Rust `String`.
    ///
    /// # Safety
    ///
    /// See the safety information for `PCSTR::as_bytes`.
    pub unsafe fn to_string(&self) -> core::result::Result<String, alloc::string::FromUtf8Error> {
        unsafe { String::from_utf8(self.as_bytes().into()) }
    }

    /// Allow this string to be displayed.
    ///
    /// # Safety
    ///
    /// See the safety information for `PCSTR::as_bytes`.
    pub unsafe fn display(&self) -> impl core::fmt::Display + '_ {
        unsafe { Decode(move || decode_utf8(self.as_bytes())) }
    }
}

impl Default for PCSTR {
    fn default() -> Self {
        Self::null()
    }
}
