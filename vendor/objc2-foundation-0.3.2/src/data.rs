use alloc::vec::Vec;
use core::ffi::c_void;
use core::fmt;
use core::marker::PhantomData;
#[cfg(feature = "NSRange")]
use core::ops::Range;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::NonNull;
use core::slice::{self};

use objc2::rc::Retained;
#[cfg(feature = "block2")]
use objc2::rc::RetainedFromIterator;
use objc2::{extern_methods, AnyThread};

use crate::{NSData, NSMutableData};

impl UnwindSafe for NSData {}
impl RefUnwindSafe for NSData {}

// GNUStep returns NULL from these methods, and Apple's documentation says
// that's valid (even though the headers say otherwise).
impl NSData {
    extern_methods!(
        #[unsafe(method(bytes))]
        pub(crate) fn bytes_raw(&self) -> *const c_void;
    );
}

impl NSMutableData {
    extern_methods!(
        #[unsafe(method(mutableBytes))]
        pub(crate) fn mutable_bytes_raw(&self) -> *mut c_void;
    );
}

impl NSData {
    // TODO: Rename to `from_bytes` to match `CFData::from_bytes`.
    pub fn with_bytes(bytes: &[u8]) -> Retained<Self> {
        let bytes_ptr = bytes.as_ptr() as *mut c_void;
        unsafe { Self::initWithBytes_length(Self::alloc(), bytes_ptr, bytes.len()) }
    }

    #[cfg(feature = "block2")]
    #[cfg(feature = "alloc")]
    pub fn from_vec(bytes: Vec<u8>) -> Retained<Self> {
        unsafe { with_vec(Self::alloc(), bytes) }
    }
}

impl NSMutableData {
    pub fn with_bytes(bytes: &[u8]) -> Retained<Self> {
        let bytes_ptr = bytes.as_ptr() as *mut c_void;
        // SAFETY: Same as `NSData::with_bytes`
        unsafe { Self::initWithBytes_length(Self::alloc(), bytes_ptr, bytes.len()) }
    }

    #[cfg(feature = "block2")]
    pub fn from_vec(bytes: Vec<u8>) -> Retained<Self> {
        // SAFETY: Same as `NSData::from_vec`
        unsafe { with_vec(Self::alloc(), bytes) }
    }
}

impl NSData {
    pub fn len(&self) -> usize {
        self.length()
    }

    pub fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// The bytes in the data.
    ///
    /// # Safety
    ///
    /// The data must not be mutated while the returned slice is alive.
    /// Consider using [`to_vec`] instead if this requirement is a bit
    /// difficult to uphold.
    ///
    /// [`to_vec`]: Self::to_vec
    pub unsafe fn as_bytes_unchecked(&self) -> &[u8] {
        let ptr = self.bytes_raw();
        if !ptr.is_null() {
            let ptr: *const u8 = ptr.cast();
            // SAFETY: The pointer is checked to not be NULL, and since we're
            // working with raw bytes (`u8`), the alignment is also correct.
            //
            // The pointer is guaranteed to be valid for as long as the data
            // is alive, or until it is mutated, see the documentation:
            // <https://developer.apple.com/documentation/foundation/nsdata/1410616-bytes?language=objc>
            //
            // By bounding the lifetime of the returned slice to `self`, we
            // ensure that the data is not deallocated while the slice is
            // alive.
            //
            // Caller ensures that the data is not mutated for the lifetime of
            // the slice.
            unsafe { slice::from_raw_parts(ptr, self.len()) }
        } else {
            // The bytes pointer may be null for length zero
            &[]
        }
    }

    /// Copy the contents of the data into a new [`Vec`].
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

    /// Iterate over the bytes of the data.
    pub fn iter(&self) -> Iter<'_> {
        Iter::new(self)
    }
}

impl NSMutableData {
    /// A mutable view of the bytes in the data.
    ///
    /// # Safety
    ///
    /// No methods on the `NSMutableData` may be called while the returned
    /// slice is alive.
    #[doc(alias = "mutableBytes")]
    #[allow(clippy::mut_from_ref)]
    pub unsafe fn as_mut_bytes_unchecked(&self) -> &mut [u8] {
        let ptr = self.mutable_bytes_raw();
        // &Cell<[u8]> is safe because the slice length is not actually in the
        // cell
        if !ptr.is_null() {
            let ptr: *mut u8 = ptr.cast();
            // SAFETY: Same as `NSData::bytes`, with the addition that a
            // mutable slice is safe, as the caller upholds that the slice is
            // not .
            unsafe { slice::from_raw_parts_mut(ptr, self.len()) }
        } else {
            &mut []
        }
    }

    #[doc(alias = "appendBytes:length:")]
    pub fn extend_from_slice(&self, bytes: &[u8]) {
        let bytes_ptr: NonNull<c_void> = NonNull::new(bytes.as_ptr() as *mut u8).unwrap().cast();
        unsafe { self.appendBytes_length(bytes_ptr, bytes.len()) }
    }

    pub fn push(&self, byte: u8) {
        self.extend_from_slice(&[byte]);
    }

    #[doc(alias = "replaceBytesInRange:withBytes:length:")]
    #[cfg(feature = "NSRange")]
    pub fn replace_range(&self, range: Range<usize>, bytes: &[u8]) {
        // No need to verify the length of the range here,
        // `replaceBytesInRange:` zero-fills if out of bounds.
        let ptr = bytes.as_ptr().cast();
        unsafe { self.replaceBytesInRange_withBytes_length(range.into(), ptr, bytes.len()) }
    }

    #[cfg(feature = "NSRange")]
    pub fn set_bytes(&self, bytes: &[u8]) {
        let len = self.len();
        self.replace_range(0..len, bytes);
    }
}

impl fmt::Debug for NSData {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // -[NSData description] is quite unreadable
        f.debug_list().entries(self.iter()).finish()
    }
}

/// An iterator over the bytes in an `NSData`.
///
///
/// # Panics
///
/// The iteration may panic if the data is mutated while being iterated upon.
//
// TODO: The implementation of this currently copies the `NSData` and iterates
// using `Vec`. We should consider optimizing this by using a stack buffer
// that we continuously copy into using `CFDataGetBytes`, or perhaps by getting
// the pointer from `bytes` (or `CFDataGetBytePtr`) and copying directly from
// that.
#[derive(Debug, Clone)]
pub struct Iter<'a> {
    p: PhantomData<&'a NSData>,
    #[cfg(debug_assertions)]
    data: &'a NSData,
    #[cfg(debug_assertions)]
    length: usize,
    bytes: alloc::vec::IntoIter<u8>,
}

impl<'a> Iter<'a> {
    fn new(data: &'a NSData) -> Self {
        Self {
            p: PhantomData,
            #[cfg(debug_assertions)]
            data,
            #[cfg(debug_assertions)]
            length: data.length(),
            bytes: data.to_vec().into_iter(),
        }
    }
}

impl ExactSizeIterator for Iter<'_> {
    fn len(&self) -> usize {
        self.bytes.len()
    }
}

impl Iterator for Iter<'_> {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        #[cfg(debug_assertions)]
        {
            if self.length != self.data.length() {
                panic!("NSData was mutated while iterating");
            }
        }

        self.bytes.next()
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}

// TODO: Consider this after the final implementation
// impl DoubleEndedIterator for Iter<'_> {
//     fn next_back(&mut self) -> Option<Self::Item> {
//         self.bytes.next_back()
//     }
// }

impl core::iter::FusedIterator for Iter<'_> {}

impl<'a> IntoIterator for &'a NSData {
    type Item = u8;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

impl<'a> IntoIterator for &'a NSMutableData {
    type Item = u8;
    type IntoIter = Iter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

impl Extend<u8> for &NSMutableData {
    /// You should use [`extend_from_slice`] whenever possible, it is more
    /// performant.
    ///
    /// [`extend_from_slice`]: NSMutableData::extend_from_slice
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        let iterator = iter.into_iter();
        iterator.for_each(move |item| self.push(item));
    }
}

// Vec also has this impl
impl<'a> Extend<&'a u8> for &NSMutableData {
    fn extend<T: IntoIterator<Item = &'a u8>>(&mut self, iter: T) {
        let iterator = iter.into_iter();
        iterator.for_each(move |item| self.push(*item));
    }
}

#[cfg(feature = "std")]
impl std::io::Write for &NSMutableData {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        self.extend_from_slice(buf);
        Ok(buf.len())
    }

    fn write_all(&mut self, buf: &[u8]) -> std::io::Result<()> {
        self.extend_from_slice(buf);
        Ok(())
    }

    fn flush(&mut self) -> std::io::Result<()> {
        Ok(())
    }
}

#[cfg(feature = "block2")]
impl RetainedFromIterator<u8> for NSData {
    fn retained_from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_vec(vec)
    }
}

#[cfg(feature = "block2")]
impl RetainedFromIterator<u8> for NSMutableData {
    fn retained_from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_vec(vec)
    }
}

#[cfg(feature = "block2")]
#[cfg(feature = "alloc")]
unsafe fn with_vec<T: objc2::Message>(obj: objc2::rc::Allocated<T>, bytes: Vec<u8>) -> Retained<T> {
    use core::mem::ManuallyDrop;

    use block2::{DynBlock, RcBlock};

    let capacity = bytes.capacity();

    let dealloc = RcBlock::new(move |bytes: *mut c_void, len: usize| {
        // Recreate the Vec and let it drop
        let _ = unsafe { <Vec<u8>>::from_raw_parts(bytes.cast(), len, capacity) };
    });
    let dealloc: &DynBlock<dyn Fn(*mut c_void, usize) + 'static> = &dealloc;

    let mut bytes = ManuallyDrop::new(bytes);
    // We intentionally extract the length before we access the
    // pointer as mutable, to not invalidate that mutable pointer.
    let len = bytes.len();
    let bytes_ptr: *mut c_void = bytes.as_mut_ptr().cast();

    unsafe {
        objc2::msg_send![
            obj,
            initWithBytesNoCopy: bytes_ptr,
            length: len,
            deallocator: dealloc,
        ]
    }
}
