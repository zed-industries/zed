//! # allocator infrastructure
//!
//! The public interface allows setting a custom allocator, but we need to configure a default
//! allocator if the user did not configure one. We have two choices, configured by feature flags:
//!
//! - `"rust-allocator"` uses the rust global allocator
//! - `"c-allocator"` uses an allocator based on `malloc` and `free`
//!
//! When both configured, `"rust-allocator"` is preferred.
//!
//! The interface for the allocator is not a great fit for rust. In particular, rust always needs
//! the layout of an allocation to deallocate it, and C interfaces don't usually provide this
//! information. Luckily in the library we know in all cases how big the allocation was at the
//! point where we deallocate it.

#[cfg(feature = "rust-allocator")]
extern crate alloc;

use core::ffi::{c_int, c_void};

use crate::bzlib::{BzStream, StreamState};

type AllocFunc = unsafe extern "C" fn(*mut c_void, c_int, c_int) -> *mut c_void;
type FreeFunc = unsafe extern "C" fn(*mut c_void, *mut c_void) -> ();

pub(crate) enum Allocator {
    #[cfg(feature = "rust-allocator")]
    Rust,
    #[cfg(feature = "c-allocator")]
    C,
    Custom {
        allocate: AllocFunc,
        deallocate: FreeFunc,
        opaque: *mut c_void,
    },
}

impl Allocator {
    #[allow(unreachable_code)]
    pub(crate) const DEFAULT: Option<Self> = 'blk: {
        #[cfg(feature = "rust-allocator")]
        break 'blk Some(Self::Rust);

        #[cfg(feature = "c-allocator")]
        break 'blk Some(Self::C);

        None
    };

    #[allow(unreachable_code)]
    pub(crate) fn default_function_pointers() -> Option<(AllocFunc, FreeFunc)> {
        #[cfg(feature = "rust-allocator")]
        return Some(rust_allocator::ALLOCATOR);

        #[cfg(feature = "c-allocator")]
        return Some(c_allocator::ALLOCATOR);

        None
    }

    /// # Safety
    ///
    /// - `strm.bzalloc` and `strm.opaque` must form a valid allocator, meaning `strm.bzalloc` returns either
    ///     * a `NULL` pointer
    ///     * a valid pointer to an allocation of `len * size_of::<T>()` bytes aligned to at least `align_of::<usize>()`
    /// - `strm.bzfree` frees memory allocated by `strm.bzalloc`
    pub(crate) unsafe fn from_bz_stream<S: StreamState>(strm: &BzStream<S>) -> Option<Self> {
        let bzalloc = strm.bzalloc?;
        let bzfree = strm.bzfree?;

        #[cfg(feature = "rust-allocator")]
        if (bzalloc, bzfree) == rust_allocator::ALLOCATOR {
            return Some(Self::Rust);
        }

        #[cfg(feature = "c-allocator")]
        if (bzalloc, bzfree) == c_allocator::ALLOCATOR {
            return Some(Self::C);
        }

        Some(Self::custom(bzalloc, bzfree, strm.opaque))
    }

    /// # Safety
    ///
    /// - `allocate` and `opaque` must form a valid allocator, meaning `allocate` returns either
    ///     * a `NULL` pointer
    ///     * a valid pointer to an allocation of `len * size_of::<T>()` bytes aligned to at least `align_of::<usize>()`
    /// - `deallocate` frees memory allocated by `allocate`
    pub(crate) fn custom(allocate: AllocFunc, deallocate: FreeFunc, opaque: *mut c_void) -> Self {
        Self::Custom {
            allocate,
            deallocate,
            opaque,
        }
    }
}

#[cfg(feature = "c-allocator")]
pub(crate) mod c_allocator {
    use super::*;

    // make sure that the only way these function pointers leave this module is via this constant
    // that way the function pointer address is a reliable way to know that the default C allocator
    // is used.
    pub(crate) static ALLOCATOR: (AllocFunc, FreeFunc) = (self::allocate, self::deallocate);

    unsafe extern "C" fn allocate(_opaque: *mut c_void, count: c_int, size: c_int) -> *mut c_void {
        unsafe { libc::malloc((count * size) as usize) }
    }

    unsafe extern "C" fn deallocate(_opaque: *mut c_void, ptr: *mut c_void) {
        if !ptr.is_null() {
            unsafe {
                libc::free(ptr);
            }
        }
    }
}

#[cfg(feature = "rust-allocator")]
mod rust_allocator {
    use super::*;

    // make sure that the only way these function pointers leave this module is via this constant
    // that way the function pointer address is a reliable way to know that the default C allocator
    // is used.
    pub(crate) static ALLOCATOR: (AllocFunc, FreeFunc) = (self::allocate, self::deallocate);

    unsafe extern "C" fn allocate(
        _opaque: *mut c_void,
        _count: c_int,
        _size: c_int,
    ) -> *mut c_void {
        unreachable!("the default rust allocation function should never be called directly");
    }

    unsafe extern "C" fn deallocate(_opaque: *mut c_void, _ptr: *mut c_void) {
        unreachable!("the default rust deallocation function should never be called directly");
    }
}

impl Allocator {
    /// Allocates `count` contiguous values of type `T`, and zeros out all elements.
    pub(crate) fn allocate_zeroed<T>(&self, count: usize) -> Option<*mut T> {
        const {
            assert!(size_of::<T>() != 0);
        }
        assert_ne!(count, 0);

        match self {
            #[cfg(feature = "rust-allocator")]
            Allocator::Rust => {
                let layout = core::alloc::Layout::array::<T>(count).unwrap();
                let ptr = unsafe { alloc::alloc::alloc_zeroed(layout) };
                (!ptr.is_null()).then_some(ptr.cast())
            }
            #[cfg(feature = "c-allocator")]
            Allocator::C => {
                let ptr = unsafe { libc::calloc(count, core::mem::size_of::<T>()) };
                (!ptr.is_null()).then_some(ptr.cast())
            }
            Allocator::Custom {
                allocate, opaque, ..
            } => unsafe {
                let ptr = (allocate)(*opaque, count as i32, core::mem::size_of::<T>() as i32);
                let ptr = ptr.cast::<T>();

                if ptr.is_null() {
                    return None;
                }

                core::ptr::write_bytes(ptr, 0, count);

                Some(ptr)
            },
        }
    }

    pub(crate) unsafe fn deallocate<T>(&self, ptr: *mut T, count: usize) {
        if ptr.is_null() || count == 0 {
            return;
        }

        match self {
            #[cfg(feature = "rust-allocator")]
            Allocator::Rust => {
                let layout = core::alloc::Layout::array::<T>(count).unwrap();
                unsafe { alloc::alloc::dealloc(ptr.cast(), layout) }
            }
            #[cfg(feature = "c-allocator")]
            Allocator::C => {
                unsafe { libc::free(ptr.cast()) };
            }
            Allocator::Custom {
                deallocate, opaque, ..
            } => {
                unsafe { deallocate(*opaque, ptr.cast()) };
            }
        }
    }
}
