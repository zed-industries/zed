use super::*;

/// An [HSTRING] builder that supports preallocating the `HSTRING` to avoid extra allocations and copies.
///
/// This is similar to the `WindowsPreallocateStringBuffer` function but implemented directly in Rust for efficiency.
/// It is implemented as a separate type since [HSTRING] values are immutable.
pub struct HStringBuilder(*mut HStringHeader);

impl HStringBuilder {
    /// Creates a preallocated `HSTRING` value.
    pub fn new(len: usize) -> Self {
        let header = HStringHeader::alloc(len.try_into().unwrap());

        if len > 0 {
            unsafe { core::ptr::write_bytes((*header).data, 0, len) };
        }

        Self(header)
    }

    /// Shortens the string by removing any trailing 0 characters.
    pub fn trim_end(&mut self) {
        if let Some(header) = self.as_header_mut() {
            while header.len > 0
                && unsafe { header.data.offset(header.len as isize - 1).read() == 0 }
            {
                header.len -= 1;
            }

            if header.len == 0 {
                unsafe {
                    HStringHeader::free(self.0);
                }
                self.0 = core::ptr::null_mut();
            }
        }
    }

    /// Allows the `HSTRING` to be constructed from bytes.
    pub fn as_bytes_mut(&mut self) -> &mut [u8] {
        if let Some(header) = self.as_header() {
            unsafe {
                core::slice::from_raw_parts_mut(header.data as *mut _, header.len as usize * 2)
            }
        } else {
            &mut []
        }
    }

    fn as_header(&self) -> Option<&HStringHeader> {
        unsafe { self.0.as_ref() }
    }

    fn as_header_mut(&mut self) -> Option<&mut HStringHeader> {
        unsafe { self.0.as_mut() }
    }
}

impl From<HStringBuilder> for HSTRING {
    fn from(value: HStringBuilder) -> Self {
        if let Some(header) = value.as_header() {
            unsafe { header.data.offset(header.len as isize).write(0) };
            let result = Self(value.0);
            core::mem::forget(value);
            result
        } else {
            Self::new()
        }
    }
}

impl core::ops::Deref for HStringBuilder {
    type Target = [u16];

    fn deref(&self) -> &[u16] {
        if let Some(header) = self.as_header() {
            unsafe { core::slice::from_raw_parts(header.data, header.len as usize) }
        } else {
            &[]
        }
    }
}

impl core::ops::DerefMut for HStringBuilder {
    fn deref_mut(&mut self) -> &mut [u16] {
        if let Some(header) = self.as_header() {
            unsafe { core::slice::from_raw_parts_mut(header.data, header.len as usize) }
        } else {
            &mut []
        }
    }
}

impl Drop for HStringBuilder {
    fn drop(&mut self) {
        unsafe {
            HStringHeader::free(self.0);
        }
    }
}

impl core::fmt::Debug for HStringBuilder {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        write!(
            f,
            "\"{}\"",
            Decode(|| core::char::decode_utf16(self.iter().cloned()))
        )
    }
}
