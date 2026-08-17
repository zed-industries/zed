#![allow(missing_docs)]

use core::mem::{self, MaybeUninit};
use core::ptr::{self, NonNull};
use core::slice;

// ABI compatible with C++ rust::Slice<T> (not necessarily &[T]).
#[repr(C)]
pub struct RustSlice {
    repr: [MaybeUninit<usize>; mem::size_of::<NonNull<[()]>>() / mem::size_of::<usize>()],
}

impl RustSlice {
    pub fn from_ref<T>(slice: &[T]) -> Self {
        let ptr = NonNull::from(slice).cast::<T>();
        let len = slice.len();
        Self::from_raw_parts(ptr, len)
    }

    pub fn from_mut<T>(slice: &mut [T]) -> Self {
        let ptr = NonNull::from(&mut *slice).cast::<T>();
        let len = slice.len();
        Self::from_raw_parts(ptr, len)
    }

    pub unsafe fn as_slice<'a, T>(self) -> &'a [T] {
        let ptr = self.as_non_null_ptr().as_ptr();
        let len = self.len();
        unsafe { slice::from_raw_parts(ptr, len) }
    }

    pub unsafe fn as_mut_slice<'a, T>(self) -> &'a mut [T] {
        let ptr = self.as_non_null_ptr().as_ptr();
        let len = self.len();
        unsafe { slice::from_raw_parts_mut(ptr, len) }
    }

    pub(crate) fn from_raw_parts<T>(ptr: NonNull<T>, len: usize) -> Self {
        // TODO: use NonNull::from_raw_parts(ptr.cast(), len) when stable.
        // https://doc.rust-lang.org/nightly/std/ptr/struct.NonNull.html#method.from_raw_parts
        // https://github.com/rust-lang/rust/issues/81513
        let ptr = ptr::slice_from_raw_parts_mut(ptr.as_ptr().cast(), len);
        unsafe { mem::transmute::<NonNull<[()]>, RustSlice>(NonNull::new_unchecked(ptr)) }
    }

    pub(crate) fn as_non_null_ptr<T>(&self) -> NonNull<T> {
        let rust_slice = RustSlice { repr: self.repr };
        let repr = unsafe { mem::transmute::<RustSlice, NonNull<[()]>>(rust_slice) };
        repr.cast()
    }

    pub(crate) fn len(&self) -> usize {
        let rust_slice = RustSlice { repr: self.repr };
        let repr = unsafe { mem::transmute::<RustSlice, NonNull<[()]>>(rust_slice) };
        // TODO: use repr.len() when stable.
        // https://doc.rust-lang.org/nightly/std/ptr/struct.NonNull.html#method.len
        // https://github.com/rust-lang/rust/issues/71146
        unsafe { repr.as_ref() }.len()
    }
}

const_assert_eq!(mem::size_of::<NonNull<[()]>>(), mem::size_of::<RustSlice>());
const_assert_eq!(
    mem::align_of::<NonNull<[()]>>(),
    mem::align_of::<RustSlice>(),
);
