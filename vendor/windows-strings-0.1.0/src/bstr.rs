use super::*;

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

    /// Returns `true` if the string is empty.
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns the length of the string.
    pub fn len(&self) -> usize {
        if self.0.is_null() {
            0
        } else {
            unsafe { bindings::SysStringLen(self.0) as usize }
        }
    }

    /// Get the string as 16-bit wide characters (wchars).
    pub fn as_wide(&self) -> &[u16] {
        unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len()) }
    }

    /// Returns a raw pointer to the `BSTR` buffer.
    pub fn as_ptr(&self) -> *const u16 {
        if !self.is_empty() {
            self.0
        } else {
            const EMPTY: [u16; 1] = [0];
            EMPTY.as_ptr()
        }
    }

    /// Create a `BSTR` from a slice of 16 bit characters (wchars).
    pub fn from_wide(value: &[u16]) -> Result<Self> {
        if value.is_empty() {
            return Ok(Self::new());
        }

        let result = unsafe {
            Self(bindings::SysAllocStringLen(
                value.as_ptr(),
                value.len().try_into()?,
            ))
        };

        if result.is_empty() {
            Err(Error::from_hresult(HRESULT(bindings::E_OUTOFMEMORY)))
        } else {
            Ok(result)
        }
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

impl Clone for BSTR {
    fn clone(&self) -> Self {
        Self::from_wide(self.as_wide()).unwrap()
    }
}

impl From<&str> for BSTR {
    fn from(value: &str) -> Self {
        let value: alloc::vec::Vec<u16> = value.encode_utf16().collect();
        Self::from_wide(&value).unwrap()
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

impl<'a> TryFrom<&'a BSTR> for String {
    type Error = alloc::string::FromUtf16Error;

    fn try_from(value: &BSTR) -> core::result::Result<Self, Self::Error> {
        String::from_utf16(value.as_wide())
    }
}

impl TryFrom<BSTR> for String {
    type Error = alloc::string::FromUtf16Error;

    fn try_from(value: BSTR) -> core::result::Result<Self, Self::Error> {
        String::try_from(&value)
    }
}

impl Default for BSTR {
    fn default() -> Self {
        Self(core::ptr::null_mut())
    }
}

impl core::fmt::Display for BSTR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(
            f,
            "{}",
            Decode(|| core::char::decode_utf16(self.as_wide().iter().cloned()))
        )
    }
}

impl core::fmt::Debug for BSTR {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::write!(f, "{}", self)
    }
}

impl PartialEq for BSTR {
    fn eq(&self, other: &Self) -> bool {
        self.as_wide() == other.as_wide()
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
        self.as_wide()
            .iter()
            .copied()
            .eq(other.as_ref().encode_utf16())
    }
}

impl Drop for BSTR {
    fn drop(&mut self) {
        if !self.0.is_null() {
            unsafe { bindings::SysFreeString(self.0) }
        }
    }
}
