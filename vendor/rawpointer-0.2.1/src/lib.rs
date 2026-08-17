// Copyright 2016-2019 bluss and rawpointer developers.
//
// Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// option. This file may not be copied, modified, or distributed
// except according to those terms.
#![no_std]


//! Rawpointer adds extra utility methods to raw pointers `*const T`, `*mut T`
//! and `NonNull<T>`.
//!
//! Features include:
//!
//! - Strided offsets - [`.stride_offset(stride,
//!   index)`](PointerExt::stride_offset) make it easy to compute
//!   pointer offsets where the index is unsigned and the stride is signed.
//!
//! - Offsetting methods in general for `NonNull`, since it does not have these
//!   from libcore
//!
//! - Post- and preincrement and post- and predecrement methods
//!
//!   - For `p++` use [`p.post_inc()`](PointerExt::post_inc).
//!   - For `++p` use [`p.pre_inc()`](PointerExt::pre_inc).
//!   - For `p--` use [`p.post_dec()`](PointerExt::post_dec).
//!   - For `--p` use [`p.pre_dec()`](PointerExt::pre_dec).
//!
//! ```rust
//! use rawpointer::PointerExt;
//!
//! unsafe {
//!     // In this example:
//!     // Use .post_inc() to iterate and overwrite the first four
//!     // elements of the array.
//!
//!     let mut xs = [0; 16];
//!     let mut ptr = xs.as_mut_ptr();
//!     let end = ptr.offset(4);
//!     let mut i = 0;
//!     while ptr != end {
//!         *ptr.post_inc() = i;
//!         i += 1;
//!     }
//!     assert_eq!(&xs[..8], &[0, 1, 2, 3, 0, 0, 0, 0]);
//! }
//! ```
//!
//! ## Safety
//!
//! See the Rust [core::ptr] documentation for more information.
//!
//! ## Rust Version
//!
//! This version of the crate requires Rust 1.26 or later

use core::mem::size_of;
use core::ptr::NonNull;

/// Return the number of elements of `T` from `start` to `end`.<br>
/// Return the arithmetic difference if `T` is zero size.
#[inline(always)]
pub fn ptrdistance<T>(start: *const T, end: *const T) -> usize {
    let size = size_of::<T>();
    if size == 0 {
        (end as usize).wrapping_sub(start as usize)
    } else {
        (end as usize - start as usize) / size
    }
}

/// Extension methods for raw pointers
pub trait PointerExt : Copy {
    unsafe fn offset(self, i: isize) -> Self;

    unsafe fn add(self, i: usize) -> Self {
        self.offset(i as isize)
    }

    unsafe fn sub(self, i: usize) -> Self {
        self.offset((i as isize).wrapping_neg())
    }

    /// Increment the pointer by 1, and return its new value.
    ///
    /// Equivalent to the C idiom `++ptr`.
    #[inline(always)]
    unsafe fn pre_inc(&mut self) -> Self {
        *self = self.offset(1);
        *self
    }

    /// Increment the pointer by 1, but return its old value.
    ///
    /// Equivalent to the C idiom `ptr++`.
    #[inline(always)]
    unsafe fn post_inc(&mut self) -> Self {
        let current = *self;
        *self = self.offset(1);
        current
    }

    /// Decrement the pointer by 1, and return its new value.
    ///
    /// Equivalent to the C idiom `--ptr`.
    #[inline(always)]
    unsafe fn pre_dec(&mut self) -> Self {
        *self = self.offset(-1);
        *self
    }

    /// Decrement the pointer by 1, but return its old value.
    ///
    /// Equivalent to the C idiom `ptr--`.
    #[inline(always)]
    unsafe fn post_dec(&mut self) -> Self {
        let current = *self;
        *self = self.offset(-1);
        current
    }

    /// Increment by 1
    #[inline(always)]
    unsafe fn inc(&mut self) {
        *self = self.offset(1);
    }

    /// Decrement by 1
    #[inline(always)]
    unsafe fn dec(&mut self) {
        *self = self.offset(-1);
    }

    /// Offset the pointer by `s` multiplied by `index`.
    #[inline(always)]
    unsafe fn stride_offset(self, s: isize, index: usize) -> Self {
        self.offset(s * index as isize)
    }
}

impl<T> PointerExt for *const T {
    #[inline(always)]
    unsafe fn offset(self, i: isize) -> Self {
        self.offset(i)
    }

    // Call inherent version of add/sub
    #[inline]
    unsafe fn add(self, i: usize) -> Self {
        self.add(i)
    }

    #[inline]
    unsafe fn sub(self, i: usize) -> Self {
        self.sub(i)
    }
}

impl<T> PointerExt for *mut T {
    #[inline(always)]
    unsafe fn offset(self, i: isize) -> Self {
        self.offset(i)
    }

    #[inline]
    unsafe fn add(self, i: usize) -> Self {
        self.add(i)
    }

    #[inline]
    unsafe fn sub(self, i: usize) -> Self {
        self.sub(i)
    }
}

/// `NonNull<T>` supports the same offsetting methods under the same
/// safety constraints as the other raw pointer implementations.
///
/// There is no difference - both when offsetting `*mut T` and `NonNull<T>`,
/// the offset is only well defined if we remain inside the same object or
/// one-past the end, and we can never land in a null pointer while obeying
/// those rules.
impl<T> PointerExt for NonNull<T> {
    #[inline(always)]
    unsafe fn offset(self, i: isize) -> Self {
        NonNull::new_unchecked(self.as_ptr().offset(i))
    }
}


#[cfg(test)]
mod tests {
    use super::PointerExt;
    use core::ptr::NonNull;

    #[test]
    fn it_works() {
        unsafe {
            let mut xs = [0; 16];
            let mut ptr = xs.as_mut_ptr();
            let end = ptr.offset(4);
            let mut i = 0;
            while ptr != end {
                *ptr.post_inc() = i;
                i += 1;
            }
            assert_eq!(&xs[..8], &[0, 1, 2, 3, 0, 0, 0, 0]);
        }
    }

    #[test]
    fn nonnull() {
        unsafe {
            let mut xs = [0; 16];
            let mut ptr = NonNull::new(xs.as_mut_ptr()).unwrap();
            let end = ptr.offset(4);
            let mut i = 0;
            while ptr != end {
                *ptr.post_inc().as_ptr() = i;
                i += 1;
            }
            assert_eq!(&xs[..8], &[0, 1, 2, 3, 0, 0, 0, 0]);
        }
    }

    #[test]
    fn nonnull_sub() {
        unsafe {
            // Test NonNull<T> .sub(1) iteration and equivalence to *mut T
            let mut xs = [0; 16];
            let mut ptr = xs.as_mut_ptr().add(xs.len());
            let nptr = NonNull::new(xs.as_mut_ptr()).unwrap();
            let mut nend = nptr.add(xs.len());
            let mut i = 0;
            while nptr != nend {
                nend = nend.sub(1);
                ptr = ptr.sub(1);
                assert_eq!(nend.as_ptr(), ptr);
                *nend.as_ptr() = i;
                i += 1;
            }
            assert_eq!(&xs[..8], &[15, 14, 13, 12, 11, 10, 9, 8]);
        }
    }
}
