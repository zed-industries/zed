//! Memory allocation APIs

use crate::rt;

#[doc(no_inline)]
pub use std::alloc::Layout;

/// Allocate memory with the global allocator.
///
/// This is equivalent to the standard library's [`std::alloc::alloc`], but with
/// the addition of leak tracking for allocated objects. Loom's leak tracking
/// will not function for allocations not performed via this method.
///
/// This function forwards calls to the [`GlobalAlloc::alloc`] method
/// of the allocator registered with the `#[global_allocator]` attribute
/// if there is one, or the `std` crate’s default.
///
/// # Safety
///
/// See [`GlobalAlloc::alloc`].
///
/// [`GlobalAlloc::alloc`]: std::alloc::GlobalAlloc::alloc
#[track_caller]
pub unsafe fn alloc(layout: Layout) -> *mut u8 {
    let ptr = std::alloc::alloc(layout);
    rt::alloc(ptr, location!());
    ptr
}

/// Allocate zero-initialized memory with the global allocator.
///
/// This is equivalent to the standard library's [`std::alloc::alloc_zeroed`],
/// but with the addition of leak tracking for allocated objects. Loom's leak
/// tracking will not function for allocations not performed via this method.
///
/// This function forwards calls to the [`GlobalAlloc::alloc_zeroed`] method
/// of the allocator registered with the `#[global_allocator]` attribute
/// if there is one, or the `std` crate’s default.
///
/// # Safety
///
/// See [`GlobalAlloc::alloc_zeroed`].
///
/// [`GlobalAlloc::alloc_zeroed`]: std::alloc::GlobalAlloc::alloc_zeroed
#[track_caller]
pub unsafe fn alloc_zeroed(layout: Layout) -> *mut u8 {
    let ptr = std::alloc::alloc_zeroed(layout);
    rt::alloc(ptr, location!());
    ptr
}

/// Deallocate memory with the global allocator.
///
/// This is equivalent to the standard library's [`std::alloc::dealloc`],
/// but with the addition of leak tracking for allocated objects. Loom's leak
/// tracking may report false positives if allocations allocated with
/// [`loom::alloc::alloc`] or [`loom::alloc::alloc_zeroed`] are deallocated via
/// [`std::alloc::dealloc`] rather than by this function.
///
/// This function forwards calls to the [`GlobalAlloc::dealloc`] method
/// of the allocator registered with the `#[global_allocator]` attribute
/// if there is one, or the `std` crate’s default.
///
/// # Safety
///
/// See [`GlobalAlloc::dealloc`].
///
/// [`GlobalAlloc::dealloc`]: std::alloc::GlobalAlloc::dealloc
/// [`loom::alloc::alloc`]: crate::alloc::alloc
/// [`loom::alloc::alloc_zeroed`]: crate::alloc::alloc_zeroed
#[track_caller]
pub unsafe fn dealloc(ptr: *mut u8, layout: Layout) {
    rt::dealloc(ptr, location!());
    std::alloc::dealloc(ptr, layout)
}

/// Track allocations, detecting leaks
#[derive(Debug)]
pub struct Track<T> {
    value: T,
    /// Drop guard tracking the allocation's lifetime.
    _obj: rt::Allocation,
}

impl<T> Track<T> {
    /// Track a value for leaks
    #[track_caller]
    pub fn new(value: T) -> Track<T> {
        Track {
            value,
            _obj: rt::Allocation::new(location!()),
        }
    }

    /// Get a reference to the value
    pub fn get_ref(&self) -> &T {
        &self.value
    }

    /// Get a mutable reference to the value
    pub fn get_mut(&mut self) -> &mut T {
        &mut self.value
    }

    /// Stop tracking the value for leaks
    pub fn into_inner(self) -> T {
        self.value
    }
}
