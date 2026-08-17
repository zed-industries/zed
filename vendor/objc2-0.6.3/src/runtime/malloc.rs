//! A minimal alternative to crates like `malloc_buf`, `mbox` and `malloced`.
use core::ffi::c_char;
use core::ffi::CStr;
use core::fmt;
use core::marker::PhantomData;
use core::ops::Deref;
use core::ptr::{self, NonNull};

use crate::ffi;

#[repr(transparent)]
pub(crate) struct MallocSlice<T> {
    ptr: NonNull<[T]>,
    // Necessary for dropck
    _p: PhantomData<[T]>,
}

impl<T> MallocSlice<T> {
    pub(crate) unsafe fn from_array(mut ptr: *mut T, len: usize) -> Self {
        // If the length is 0, the pointer is usually NULL, and as such we
        // need to conjure some other pointer (slices are always non-null).
        if len == 0 {
            ptr = NonNull::dangling().as_ptr();
        }

        let ptr = ptr::slice_from_raw_parts_mut(ptr, len);
        let ptr = NonNull::new(ptr).expect("tried to construct MallocSlice from a NULL pointer");
        Self {
            ptr,
            _p: PhantomData,
        }
    }

    fn len(&self) -> usize {
        // TODO: Use `self.ptr.len()` once in MSRV
        (**self).len()
    }
}

impl<T> Drop for MallocSlice<T> {
    #[allow(clippy::len_zero)]
    fn drop(&mut self) {
        // If the length is 0, then the pointer is dangling from `from_array`
        // (since the length is immutable), and we can skip calling `free`.
        if self.len() != 0 {
            // SAFETY: We take ownership over the slice elements in
            // `from_array`.
            unsafe { ptr::drop_in_place(self.ptr.as_ptr()) };
            // SAFETY: We take ownership over the pointer in `from_array`,
            // and the pointer is valid if the length is non-zero.
            unsafe { ffi::free(self.ptr.cast().as_ptr()) };
        }
    }
}

impl<T> Deref for MallocSlice<T> {
    type Target = [T];

    #[inline]
    fn deref(&self) -> &[T] {
        // SAFETY:
        // - That the pointer is aligned, dereferenceable and initialized is
        //   ensured by the caller of `from_array` (which usually get it from
        //   some external API that will do this for you).
        // - The lifetime is bound to the `MallocSlice`, which in turn ensures
        //   the pointer is valid until it is dropped.
        unsafe { self.ptr.as_ref() }
    }
}

impl<T: fmt::Debug> fmt::Debug for MallocSlice<T> {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl<T> AsRef<[T]> for MallocSlice<T> {
    #[inline]
    fn as_ref(&self) -> &[T] {
        self
    }
}

#[repr(transparent)]
pub(crate) struct MallocCStr {
    ptr: NonNull<CStr>,
}

impl MallocCStr {
    pub(crate) unsafe fn from_c_str(ptr: *mut c_char) -> Self {
        if ptr.is_null() {
            panic!("tried to construct MallocStr from a NULL pointer");
        }
        // SAFETY: We just checked that the pointer is not NULL.
        //
        // Further validity of the pointer is ensured by the caller.
        let cstr = unsafe { CStr::from_ptr(ptr) };
        // Note that we construct this `NonNull` from an immutable reference
        // (there is not yet a `CStr::from_mut_ptr`).
        //
        // This means that we're (probably) no longer allowed to mutate the
        // value, if that is desired for `MallocStr` in the future, then we'll
        // have to implement this method a bit differently.
        let ptr = NonNull::from(cstr);
        Self { ptr }
    }
}

impl Drop for MallocCStr {
    #[inline]
    fn drop(&mut self) {
        // SAFETY: We take ownership in `from_c_str`.
        unsafe { ffi::free(self.ptr.cast().as_ptr()) };
    }
}

impl Deref for MallocCStr {
    type Target = CStr;

    #[inline]
    fn deref(&self) -> &CStr {
        // SAFETY: Same as `MallocSlice::deref`
        unsafe { self.ptr.as_ref() }
    }
}

impl fmt::Debug for MallocCStr {
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(&**self, f)
    }
}

impl AsRef<CStr> for MallocCStr {
    #[inline]
    fn as_ref(&self) -> &CStr {
        self
    }
}
