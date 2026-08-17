use super::*;

/// An ([HSTRING](https://docs.microsoft.com/en-us/windows/win32/winrt/hstring))
/// is a reference-counted and immutable UTF-16 string type.
#[repr(transparent)]
pub struct HSTRING(pub(crate) *mut HStringHeader);

impl HSTRING {
    /// Create an empty `HSTRING`.
    ///
    /// This function does not allocate memory.
    pub const fn new() -> Self {
        Self(core::ptr::null_mut())
    }

    /// Returns `true` if the string is empty.
    pub fn is_empty(&self) -> bool {
        // An empty HSTRING is represented by a null pointer.
        self.0.is_null()
    }

    /// Returns the length of the string. The length is measured in `u16`s (UTF-16 code units), not including the terminating null character.
    pub fn len(&self) -> usize {
        if let Some(header) = self.as_header() {
            header.len as usize
        } else {
            0
        }
    }

    /// Get the string as 16-bit wide characters (wchars).
    pub fn as_wide(&self) -> &[u16] {
        unsafe { core::slice::from_raw_parts(self.as_ptr(), self.len()) }
    }

    /// Returns a raw pointer to the `HSTRING` buffer.
    pub fn as_ptr(&self) -> *const u16 {
        if let Some(header) = self.as_header() {
            header.data
        } else {
            const EMPTY: [u16; 1] = [0];
            EMPTY.as_ptr()
        }
    }

    /// Create a `HSTRING` from a slice of 16 bit characters (wchars).
    pub fn from_wide(value: &[u16]) -> Result<Self> {
        unsafe { Self::from_wide_iter(value.iter().copied(), value.len()) }
    }

    /// Get the contents of this `HSTRING` as a String lossily.
    pub fn to_string_lossy(&self) -> String {
        String::from_utf16_lossy(self.as_wide())
    }

    /// Get the contents of this `HSTRING` as a OsString.
    #[cfg(feature = "std")]
    pub fn to_os_string(&self) -> std::ffi::OsString {
        std::os::windows::ffi::OsStringExt::from_wide(self.as_wide())
    }

    /// # Safety
    /// len must not be less than the number of items in the iterator.
    unsafe fn from_wide_iter<I: Iterator<Item = u16>>(iter: I, len: usize) -> Result<Self> {
        if len == 0 {
            return Ok(Self::new());
        }

        let ptr = HStringHeader::alloc(len.try_into()?)?;

        // Place each utf-16 character into the buffer and
        // increase len as we go along.
        for (index, wide) in iter.enumerate() {
            debug_assert!(index < len);

            (*ptr).data.add(index).write(wide);
            (*ptr).len = index as u32 + 1;
        }

        // Write a 0 byte to the end of the buffer.
        (*ptr).data.offset((*ptr).len as isize).write(0);
        Ok(Self(ptr))
    }

    fn as_header(&self) -> Option<&HStringHeader> {
        unsafe { self.0.as_ref() }
    }
}

impl Default for HSTRING {
    fn default() -> Self {
        Self::new()
    }
}

impl Clone for HSTRING {
    fn clone(&self) -> Self {
        if let Some(header) = self.as_header() {
            Self(header.duplicate().unwrap())
        } else {
            Self::new()
        }
    }
}

impl Drop for HSTRING {
    fn drop(&mut self) {
        if let Some(header) = self.as_header() {
            // HSTRING_REFERENCE_FLAG indicates a string backed by static or stack memory that is
            // thus not reference-counted and does not need to be freed.
            unsafe {
                if header.flags & HSTRING_REFERENCE_FLAG == 0 && header.count.release() == 0 {
                    HStringHeader::free(self.0);
                }
            }
        }
    }
}

unsafe impl Send for HSTRING {}
unsafe impl Sync for HSTRING {}

impl core::fmt::Display for HSTRING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "{}",
            Decode(|| core::char::decode_utf16(self.as_wide().iter().cloned()))
        )
    }
}

impl core::fmt::Debug for HSTRING {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "\"{}\"", self)
    }
}

impl From<&str> for HSTRING {
    fn from(value: &str) -> Self {
        unsafe { Self::from_wide_iter(value.encode_utf16(), value.len()).unwrap() }
    }
}

impl From<String> for HSTRING {
    fn from(value: String) -> Self {
        value.as_str().into()
    }
}

impl From<&String> for HSTRING {
    fn from(value: &String) -> Self {
        value.as_str().into()
    }
}

#[cfg(feature = "std")]
impl From<&std::path::Path> for HSTRING {
    fn from(value: &std::path::Path) -> Self {
        value.as_os_str().into()
    }
}

#[cfg(feature = "std")]
impl From<&std::ffi::OsStr> for HSTRING {
    fn from(value: &std::ffi::OsStr) -> Self {
        unsafe {
            Self::from_wide_iter(
                std::os::windows::ffi::OsStrExt::encode_wide(value),
                value.len(),
            )
            .unwrap()
        }
    }
}

#[cfg(feature = "std")]
impl From<std::ffi::OsString> for HSTRING {
    fn from(value: std::ffi::OsString) -> Self {
        value.as_os_str().into()
    }
}

#[cfg(feature = "std")]
impl From<&std::ffi::OsString> for HSTRING {
    fn from(value: &std::ffi::OsString) -> Self {
        value.as_os_str().into()
    }
}

impl Eq for HSTRING {}

impl Ord for HSTRING {
    fn cmp(&self, other: &Self) -> core::cmp::Ordering {
        self.as_wide().cmp(other.as_wide())
    }
}

impl core::hash::Hash for HSTRING {
    fn hash<H: core::hash::Hasher>(&self, hasher: &mut H) {
        self.as_wide().hash(hasher)
    }
}

impl PartialOrd for HSTRING {
    fn partial_cmp(&self, other: &Self) -> Option<core::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl PartialEq for HSTRING {
    fn eq(&self, other: &Self) -> bool {
        *self.as_wide() == *other.as_wide()
    }
}

impl PartialEq<String> for HSTRING {
    fn eq(&self, other: &String) -> bool {
        *self == **other
    }
}

impl PartialEq<String> for &HSTRING {
    fn eq(&self, other: &String) -> bool {
        **self == **other
    }
}

impl PartialEq<&String> for HSTRING {
    fn eq(&self, other: &&String) -> bool {
        *self == ***other
    }
}

impl PartialEq<str> for HSTRING {
    fn eq(&self, other: &str) -> bool {
        self.as_wide().iter().copied().eq(other.encode_utf16())
    }
}

impl PartialEq<str> for &HSTRING {
    fn eq(&self, other: &str) -> bool {
        **self == *other
    }
}

impl PartialEq<&str> for HSTRING {
    fn eq(&self, other: &&str) -> bool {
        *self == **other
    }
}

impl PartialEq<HSTRING> for str {
    fn eq(&self, other: &HSTRING) -> bool {
        *other == *self
    }
}

impl PartialEq<HSTRING> for &str {
    fn eq(&self, other: &HSTRING) -> bool {
        *other == **self
    }
}

impl PartialEq<&HSTRING> for str {
    fn eq(&self, other: &&HSTRING) -> bool {
        **other == *self
    }
}

impl PartialEq<HSTRING> for String {
    fn eq(&self, other: &HSTRING) -> bool {
        *other == **self
    }
}

impl PartialEq<HSTRING> for &String {
    fn eq(&self, other: &HSTRING) -> bool {
        *other == ***self
    }
}

impl PartialEq<&HSTRING> for String {
    fn eq(&self, other: &&HSTRING) -> bool {
        **other == **self
    }
}

#[cfg(feature = "std")]
impl PartialEq<std::ffi::OsString> for HSTRING {
    fn eq(&self, other: &std::ffi::OsString) -> bool {
        *self == **other
    }
}

#[cfg(feature = "std")]
impl PartialEq<std::ffi::OsString> for &HSTRING {
    fn eq(&self, other: &std::ffi::OsString) -> bool {
        **self == **other
    }
}

#[cfg(feature = "std")]
impl PartialEq<&std::ffi::OsString> for HSTRING {
    fn eq(&self, other: &&std::ffi::OsString) -> bool {
        *self == ***other
    }
}

#[cfg(feature = "std")]
impl PartialEq<std::ffi::OsStr> for HSTRING {
    fn eq(&self, other: &std::ffi::OsStr) -> bool {
        self.as_wide()
            .iter()
            .copied()
            .eq(std::os::windows::ffi::OsStrExt::encode_wide(other))
    }
}

#[cfg(feature = "std")]
impl PartialEq<std::ffi::OsStr> for &HSTRING {
    fn eq(&self, other: &std::ffi::OsStr) -> bool {
        **self == *other
    }
}

#[cfg(feature = "std")]
impl PartialEq<&std::ffi::OsStr> for HSTRING {
    fn eq(&self, other: &&std::ffi::OsStr) -> bool {
        *self == **other
    }
}

#[cfg(feature = "std")]
impl PartialEq<HSTRING> for std::ffi::OsStr {
    fn eq(&self, other: &HSTRING) -> bool {
        *other == *self
    }
}

#[cfg(feature = "std")]
impl PartialEq<HSTRING> for &std::ffi::OsStr {
    fn eq(&self, other: &HSTRING) -> bool {
        *other == **self
    }
}

#[cfg(feature = "std")]
impl PartialEq<&HSTRING> for std::ffi::OsStr {
    fn eq(&self, other: &&HSTRING) -> bool {
        **other == *self
    }
}

#[cfg(feature = "std")]
impl PartialEq<HSTRING> for std::ffi::OsString {
    fn eq(&self, other: &HSTRING) -> bool {
        *other == **self
    }
}

#[cfg(feature = "std")]
impl PartialEq<HSTRING> for &std::ffi::OsString {
    fn eq(&self, other: &HSTRING) -> bool {
        *other == ***self
    }
}

#[cfg(feature = "std")]
impl PartialEq<&HSTRING> for std::ffi::OsString {
    fn eq(&self, other: &&HSTRING) -> bool {
        **other == **self
    }
}

impl<'a> TryFrom<&'a HSTRING> for String {
    type Error = alloc::string::FromUtf16Error;

    fn try_from(hstring: &HSTRING) -> core::result::Result<Self, Self::Error> {
        String::from_utf16(hstring.as_wide())
    }
}

impl TryFrom<HSTRING> for String {
    type Error = alloc::string::FromUtf16Error;

    fn try_from(hstring: HSTRING) -> core::result::Result<Self, Self::Error> {
        String::try_from(&hstring)
    }
}

#[cfg(feature = "std")]
impl<'a> From<&'a HSTRING> for std::ffi::OsString {
    fn from(hstring: &HSTRING) -> Self {
        hstring.to_os_string()
    }
}

#[cfg(feature = "std")]
impl From<HSTRING> for std::ffi::OsString {
    fn from(hstring: HSTRING) -> Self {
        Self::from(&hstring)
    }
}
