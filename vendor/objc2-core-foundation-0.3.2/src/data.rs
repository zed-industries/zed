use core::slice;

#[cfg(feature = "alloc")]
use alloc::vec::Vec;

use crate::{CFData, CFIndex};

fn get_len(bytes: &[u8]) -> CFIndex {
    // An allocation in Rust cannot be larger than isize::MAX, so this will
    // never fail.
    let len = bytes.len();
    debug_assert!(len < CFIndex::MAX as usize);
    len as CFIndex
}

impl CFData {
    /// Creates a new `CFData` from a byte slice.
    #[inline]
    #[doc(alias = "CFDataCreate")]
    pub fn from_bytes(bytes: &[u8]) -> crate::CFRetained<Self> {
        let len = get_len(bytes);
        unsafe { crate::CFData::new(None, bytes.as_ptr(), len) }.expect("failed creating CFData")
    }

    /// Alias for easier transition from the `core-foundation` crate.
    #[inline]
    #[deprecated = "renamed to CFData::from_bytes"]
    pub fn from_buffer(bytes: &[u8]) -> crate::CFRetained<Self> {
        Self::from_bytes(bytes)
    }

    /// Creates a new `CFData` from a `'static` byte slice.
    ///
    /// This may be slightly more efficient than [`CFData::from_bytes`], as it
    /// may be able to re-use the existing buffer (since we know it won't be
    /// deallocated).
    #[inline]
    #[doc(alias = "CFDataCreateWithBytesNoCopy")]
    pub fn from_static_bytes(bytes: &'static [u8]) -> crate::CFRetained<Self> {
        let len = get_len(bytes);
        // SAFETY: Same as `CFString::from_static_str`.
        unsafe { CFData::with_bytes_no_copy(None, bytes.as_ptr(), len, crate::kCFAllocatorNull) }
            .expect("failed creating CFData")
    }

    /// The number of bytes contained by the `CFData`.
    #[inline]
    #[doc(alias = "CFDataGetLength")]
    pub fn len(&self) -> usize {
        self.length() as usize
    }

    /// Whether the `CFData` is empty.
    #[inline]
    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The underlying bytes in the `CFData`.
    ///
    ///
    /// # Safety
    ///
    /// The `CFData` must not be mutated for the lifetime of the returned
    /// string. Consider using [`to_vec`] instead if this requirement is a bit
    /// difficult to uphold.
    ///
    /// [`to_vec`]: Self::to_vec
    #[inline]
    #[doc(alias = "CFDataGetBytePtr")]
    pub unsafe fn as_bytes_unchecked(&self) -> &[u8] {
        let ptr = self.byte_ptr();
        if !ptr.is_null() {
            // SAFETY: The pointer is valid, and caller ensures that the
            // `CFData` is not mutated for the lifetime of it.
            //
            // Same as
            unsafe { slice::from_raw_parts(ptr, self.len()) }
        } else {
            // The bytes pointer may be null for length zero
            &[]
        }
    }

    /// Copy the contents of the `CFData` into a new [`Vec`].
    #[cfg(feature = "alloc")]
    #[doc(alias = "CFDataGetBytePtr")]
    pub fn to_vec(&self) -> Vec<u8> {
        // NOTE: We don't do `Vec::from`, as that will call the allocator
        // while the buffer is active, and we don't know if that allocator
        // uses a CFMutableData under the hood (though very theoretical).

        let mut vec = Vec::with_capacity(self.len());
        // SAFETY: We've pre-allocated the Vec, so it won't call the allocator
        // while the byte slice is alive (and hence it won't ).
        vec.extend_from_slice(unsafe { self.as_bytes_unchecked() });
        vec
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn roundtrip() {
        let data = CFData::from_bytes(&[1, 2, 3]);
        assert_eq!(data.to_vec(), [1, 2, 3]);
    }

    #[test]
    fn empty() {
        let data = CFData::from_bytes(&[]);
        assert!(data.is_empty());
        assert_eq!(data.to_vec(), []);
    }
}
