use super::*;
use core::ops::Deref;

/// A BSTR string ([BSTR](https://learn.microsoft.com/en-us/previous-versions/windows/desktop/automat/string-manipulation-functions))
/// is a length-prefixed wide string.
#[repr(transparent)]
pub struct BSTR(*const u16);

impl BSTR {
    /// Create an empty `BSTR`.
    ///
    /// This function does not allocate memory.
    pub const fn new() -> Self {
        Self(core::ptr::null_mut())
    }

    /// Create a `BSTR` from a slice of 16 bit characters (wchars).
    pub fn from_wide(value: &[u16]) -> Self {
        if value.is_empty() {
            return Self::new();
        }

        let result = unsafe {
            Self(bindings::SysAllocStringLen(
                value.as_ptr(),
                value.len().try_into().unwrap(),
            ))
        };

        if result.is_empty() {
            panic!("allocation failed");
        }

        result
    }

    /// # Safety
    #[doc(hidden)]
    pub unsafe fn from_raw(raw: *const u16) -> Self {
        Self(raw)
    }

    /// # Safety
    #[doc(hidden)]
    pub fn into_raw(self) -> *const u16 {
        unsafe { core::mem::transmute(self) }
    }
}

impl Deref for BSTR {
    type Target = [u16];

    fn deref(&self) -> &[u16] {
        let len = if self.0.is_null() {
            0
        } else {
            unsafe { bindings::SysStringLen(self.0) as usize }
        };

        if len > 0 {
            unsafe { core::slice::from_raw_parts(self.0, len) }
        } else {
            // This ensures that if `as_ptr` is called on the slice that the resulting pointer
            // will still refer to a null-terminated string.
            const EMPTY: [u16; 1] = [0];
            &EMPTY[..0]
        }
    }
}

impl Clone for BSTR {
    fn clone(&self) -> Self {
        Self::from_wide(self)
    }
}

impl From<&str> for BSTR {
    fn from(value: &str) -> Self {
        let value: alloc::vec::Vec<u16> = value.encode_utf16().collect();
        Self::from_wide(&value)
    }
}

impl From<String> for BSTR {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&String> for BSTR {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}

impl TryFrom<&BSTR> for String {
    type Error = alloc::string::FromUtf16Error;

    fn try_from(value: &BSTR) -> core::result::Result<Self, Self::Error> {
        Self::from_utf16(value)
    }
}

impl TryFrom<BSTR> for String {
    type Error = alloc::string::FromUtf16Error;

    fn try_from(value: BSTR) -> core::result::Result<Self, Self::Error> {
        Self::try_from(&value)
    }
}

impl Default for BSTR {
    fn default() -> Self {
        Self(core::ptr::null_mut())
    }
}

impl core::fmt::Display for BSTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::write!(
            f,
            "{}",
            Decode(|| core::char::decode_utf16(self.iter().cloned()))
        )
    }
}

impl core::fmt::Debug for BSTR {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        core::write!(f, "{self}")
    }
}

impl PartialEq for BSTR {
    fn eq(&self, other: &Self) -> bool {
        self.deref() == other.deref()
    }
}

impl Eq for BSTR {}

impl PartialEq<BSTR> for &str {
    fn eq(&self, other: &BSTR) -> bool {
        other == self
    }
}

impl PartialEq<BSTR> for String {
    fn eq(&self, other: &BSTR) -> bool {
        other == self
    }
}

impl<T: AsRef<str> + ?Sized> PartialEq<T> for BSTR {
    fn eq(&self, other: &T) -> bool {
        self.iter().copied().eq(other.as_ref().encode_utf16())
    }
}

impl Drop for BSTR {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { bindings::SysFreeString(self.0) }
        }
    }
}
