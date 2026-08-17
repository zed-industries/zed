#[cfg(feature = "block2")]
use alloc::vec::Vec;
use core::ffi::c_void;
use core::fmt;
use core::ops::Index;
use core::ops::IndexMut;
#[cfg(feature = "NSRange")]
use core::ops::Range;
use core::panic::{RefUnwindSafe, UnwindSafe};
use core::ptr::NonNull;
use core::slice::{self, SliceIndex};

use objc2::rc::Retained;
#[cfg(feature = "block2")]
use objc2::rc::RetainedFromIterator;
use objc2::{extern_methods, ClassType};

use crate::Foundation::{NSData, NSMutableData};

// SAFETY: `NSData` is immutable and `NSMutableData` can only be mutated from
// `&mut` methods.
unsafe impl Sync for NSData {}
unsafe impl Send for NSData {}

impl UnwindSafe for NSData {}
impl RefUnwindSafe for NSData {}

// GNUStep returns NULL from these methods
extern_methods!(
    unsafe impl NSData {
        #[method(bytes)]
        pub(crate) fn bytes_raw(&self) -> Option<NonNull<c_void>>;
    }

    unsafe impl NSMutableData {
        #[method(mutableBytes)]
        pub(crate) fn mutable_bytes_raw(&mut self) -> Option<NonNull<c_void>>;
    }
);

impl NSData {
    pub fn with_bytes(bytes: &[u8]) -> Retained<Self> {
        let bytes_ptr = bytes.as_ptr() as *mut c_void;
        unsafe { Self::initWithBytes_length(Self::alloc(), bytes_ptr, bytes.len()) }
    }

    #[cfg(feature = "block2")]
    pub fn from_vec(bytes: Vec<u8>) -> Retained<Self> {
        // GNUStep's NSData `initWithBytesNoCopy:length:deallocator:` has a
        // bug; it forgets to assign the input buffer and length to the
        // instance before it swizzles to NSDataWithDeallocatorBlock.
        // See https://github.com/gnustep/libs-base/pull/213
        // So instead we use NSDataWithDeallocatorBlock directly.
        //
        // NSMutableData does not have this problem.
        #[cfg(feature = "gnustep-1-7")]
        let obj = unsafe { objc2::msg_send_id![objc2::class!(NSDataWithDeallocatorBlock), alloc] };
        #[cfg(not(feature = "gnustep-1-7"))]
        let obj = Self::alloc();

        unsafe { with_vec(obj, bytes) }
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

    pub fn bytes(&self) -> &[u8] {
        if let Some(ptr) = self.bytes_raw() {
            let ptr: *const u8 = ptr.as_ptr().cast();
            // SAFETY: The pointer is checked to not be NULL, and since we're
            // working with raw bytes (`u8`), the alignment is also correct.
            unsafe { slice::from_raw_parts(ptr, self.len()) }
        } else {
            // The bytes pointer may be null for length zero on GNUStep
            &[]
        }
    }
}

impl NSMutableData {
    #[doc(alias = "mutableBytes")]
    pub fn bytes_mut(&mut self) -> &mut [u8] {
        if let Some(ptr) = self.mutable_bytes_raw() {
            let ptr: *mut u8 = ptr.as_ptr().cast();
            // SAFETY: Same as `NSData::bytes`, with the addition that a
            // mutable slice is safe, since we take `&mut NSMutableData`.
            unsafe { slice::from_raw_parts_mut(ptr, self.len()) }
        } else {
            &mut []
        }
    }

    #[doc(alias = "appendBytes:length:")]
    pub fn extend_from_slice(&mut self, bytes: &[u8]) {
        let bytes_ptr: NonNull<c_void> = NonNull::new(bytes.as_ptr() as *mut u8).unwrap().cast();
        unsafe { self.appendBytes_length(bytes_ptr, bytes.len()) }
    }

    pub fn push(&mut self, byte: u8) {
        self.extend_from_slice(&[byte]);
    }

    #[doc(alias = "replaceBytesInRange:withBytes:length:")]
    #[cfg(feature = "NSRange")]
    pub fn replace_range(&mut self, range: Range<usize>, bytes: &[u8]) {
        // No need to verify the length of the range here,
        // `replaceBytesInRange:` zero-fills if out of bounds.
        let ptr = bytes.as_ptr() as *mut c_void;
        unsafe { self.replaceBytesInRange_withBytes_length(range.into(), ptr, bytes.len()) }
    }

    #[cfg(feature = "NSRange")]
    pub fn set_bytes(&mut self, bytes: &[u8]) {
        let len = self.len();
        self.replace_range(0..len, bytes);
    }
}

impl AsRef<[u8]> for NSData {
    fn as_ref(&self) -> &[u8] {
        self.bytes()
    }
}

impl AsRef<[u8]> for NSMutableData {
    fn as_ref(&self) -> &[u8] {
        self.bytes()
    }
}

impl AsMut<[u8]> for NSMutableData {
    fn as_mut(&mut self) -> &mut [u8] {
        self.bytes_mut()
    }
}

// Note: We don't implement `Borrow<[u8]>` since we can't guarantee that `Eq`,
// `Ord` and `Hash` are equal for `NSData` vs. `[u8]`!

impl<I: SliceIndex<[u8]>> Index<I> for NSData {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        // Replaces the need for getBytes:range:
        Index::index(self.bytes(), index)
    }
}

impl<I: SliceIndex<[u8]>> Index<I> for NSMutableData {
    type Output = I::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        Index::index(self.bytes(), index)
    }
}

impl<I: SliceIndex<[u8]>> IndexMut<I> for NSMutableData {
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        IndexMut::index_mut(self.bytes_mut(), index)
    }
}

impl fmt::Debug for NSData {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        // -[NSData description] is quite unreadable
        fmt::Debug::fmt(self.bytes(), f)
    }
}

impl fmt::Debug for NSMutableData {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<'a> IntoIterator for &'a NSData {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes().iter()
    }
}

impl<'a> IntoIterator for &'a NSMutableData {
    type Item = &'a u8;
    type IntoIter = core::slice::Iter<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes().iter()
    }
}

impl<'a> IntoIterator for &'a mut NSMutableData {
    type Item = &'a mut u8;
    type IntoIter = core::slice::IterMut<'a, u8>;

    fn into_iter(self) -> Self::IntoIter {
        self.bytes_mut().iter_mut()
    }
}

impl Extend<u8> for NSMutableData {
    /// You should use [`extend_from_slice`] whenever possible, it is more
    /// performant.
    ///
    /// [`extend_from_slice`]: Self::extend_from_slice
    fn extend<T: IntoIterator<Item = u8>>(&mut self, iter: T) {
        let iterator = iter.into_iter();
        iterator.for_each(move |item| self.push(item));
    }
}

// Vec also has this impl
impl<'a> Extend<&'a u8> for NSMutableData {
    fn extend<T: IntoIterator<Item = &'a u8>>(&mut self, iter: T) {
        let iterator = iter.into_iter();
        iterator.for_each(move |item| self.push(*item));
    }
}

impl std::io::Write for NSMutableData {
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
    fn id_from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_vec(vec)
    }
}

#[cfg(feature = "block2")]
impl RetainedFromIterator<u8> for NSMutableData {
    fn id_from_iter<I: IntoIterator<Item = u8>>(iter: I) -> Retained<Self> {
        let vec = Vec::from_iter(iter);
        Self::from_vec(vec)
    }
}

#[cfg(feature = "block2")]
unsafe fn with_vec<T: objc2::Message>(obj: objc2::rc::Allocated<T>, bytes: Vec<u8>) -> Retained<T> {
    use core::mem::ManuallyDrop;

    use block2::{Block, RcBlock};

    let capacity = bytes.capacity();

    let dealloc = RcBlock::new(move |bytes: *mut c_void, len: usize| {
        // Recreate the Vec and let it drop
        let _ = unsafe { <Vec<u8>>::from_raw_parts(bytes.cast(), len, capacity) };
    });
    let dealloc: &Block<dyn Fn(*mut c_void, usize) + 'static> = &dealloc;

    let mut bytes = ManuallyDrop::new(bytes);
    let bytes_ptr: *mut c_void = bytes.as_mut_ptr().cast();

    unsafe {
        objc2::msg_send_id![
            obj,
            initWithBytesNoCopy: bytes_ptr,
            length: bytes.len(),
            deallocator: dealloc,
        ]
    }
}
