// Copyright © 2023-2025 Andrea Corbellini and contributors
// SPDX-License-Identifier: BSD-3-Clause

//! This crate implements a [circular buffer], also known as cyclic buffer, circular queue or ring.
//!
//! The main struct is [`CircularBuffer`]. It can live on the stack and does not require any heap
//! memory allocation. A `CircularBuffer` is sequence of elements with a maximum capacity: elements
//! can be added to the buffer, and once the maximum capacity is reached, the elements at the start
//! of the buffer are discarded and overwritten.
//!
//! [circular buffer]: https://en.wikipedia.org/wiki/Circular_buffer
//!
//! # Examples
//!
//! ```
//! use circular_buffer::CircularBuffer;
//!
//! // Initialize a new, empty circular buffer with a capacity of 5 elements
//! let mut buf = CircularBuffer::<5, u32>::new();
//!
//! // Add a few elements
//! buf.push_back(1);
//! buf.push_back(2);
//! buf.push_back(3);
//! assert_eq!(buf, [1, 2, 3]);
//!
//! // Add more elements to fill the buffer capacity completely
//! buf.push_back(4);
//! buf.push_back(5);
//! assert_eq!(buf, [1, 2, 3, 4, 5]);
//!
//! // Adding more elements than the buffer can contain causes the front elements to be
//! // automatically dropped
//! buf.push_back(6);
//! assert_eq!(buf, [2, 3, 4, 5, 6]); // `1` got dropped to make room for `6`
//! ```
//!
//! # Interface
//!
//! [`CircularBuffer`] provides methods akin the ones for the standard
//! [`VecDeque`](std::collections::VecDeque) and [`LinkedList`](std::collections::LinkedList). The
//! list below includes the most common methods, but see the [`CircularBuffer` struct
//! documentation](CircularBuffer) to see more.
//!
//! ## Adding/removing elements
//!
//! * [`push_back()`](CircularBuffer::push_back), [`push_front()`](CircularBuffer::push_front)
//! * [`pop_back()`](CircularBuffer::pop_back), [`pop_front()`](CircularBuffer::pop_front)
//! * [`swap_remove_back()`](CircularBuffer::swap_remove_back),
//!   [`swap_remove_front()`](CircularBuffer::swap_remove_front)
//!
//! ## Getting/mutating elements
//!
//! * [`get()`](CircularBuffer::get), [`get_mut()`](CircularBuffer::get_mut)
//! * [`front()`](CircularBuffer::front), [`front_mut()`](CircularBuffer::front_mut)
//! * [`back()`](CircularBuffer::back), [`back_mut()`](CircularBuffer::back_mut)
//! * [`nth_front()`](CircularBuffer::nth_front), [`nth_front_mut()`](CircularBuffer::nth_front_mut)
//! * [`nth_back()`](CircularBuffer::nth_back), [`nth_back_mut()`](CircularBuffer::nth_back_mut)
//!
//! ## Adding multiple elements at once
//!
//! * [`extend()`](CircularBuffer::extend),
//!   [`extend_from_slice()`](CircularBuffer::extend_from_slice)
//! * [`fill()`](CircularBuffer::fill), [`fill_with()`](CircularBuffer::fill_with)
//! * [`fill_spare()`](CircularBuffer::fill), [`fill_spare_with()`](CircularBuffer::fill_with)
//!
//! ## Iterators
//!
//! * [`into_iter()`](CircularBuffer::into_iter)
//! * [`iter()`](CircularBuffer::iter), [`iter_mut()`](CircularBuffer::iter_mut)
//! * [`range()`](CircularBuffer::range), [`range_mut()`](CircularBuffer::range_mut)
//! * [`drain()`](CircularBuffer::drain)
//!
//! ## Writing/reading bytes
//!
//! For the special case of a `CircularBuffer` containing `u8` elements, bytes can be written and
//! read using the standard [`Write`](std::io::Write) and [`Read`](std::io::Read) traits. Writing
//! past the buffer capacity will overwrite the bytes at the start of the buffer, and reading
//! elements will consume elements from the buffer.
//!
//! ```
//! # #[allow(unused_must_use)]
//! # #[cfg(feature = "std")]
//! # {
//! use circular_buffer::CircularBuffer;
//! use std::io::Read;
//! use std::io::Write;
//!
//! let mut buf = CircularBuffer::<5, u8>::new();
//! assert_eq!(buf, b"");
//!
//! write!(buf, "hello");
//! assert_eq!(buf, b"hello");
//!
//! write!(buf, "this string will overflow the buffer and wrap around");
//! assert_eq!(buf, b"round");
//!
//! let mut s = String::new();
//! buf.read_to_string(&mut s)
//!     .expect("failed to read from buffer");
//! assert_eq!(s, "round");
//! assert_eq!(buf, b"");
//! # }
//! ```
//!
//! # Time complexity
//!
//! Most of the methods implemented by [`CircularBuffer`] run in constant time. Some of the methods
//! may run in linear time if the type of the elements implements [`Drop`], as each element needs
//! to be deallocated one-by-one.
//!
//! | Method                                                                                                                                                                                     | Complexity                                                           |
//! |--------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------------|----------------------------------------------------------------------|
//! | [`push_back()`](CircularBuffer::push_back), [`push_front()`](CircularBuffer::push_front)                                                                                                   | *O*(1)                                                               |
//! | [`pop_back()`](CircularBuffer::pop_back), [`pop_front()`](CircularBuffer::pop_front)                                                                                                       | *O*(1)                                                               |
//! | [`remove(i)`](CircularBuffer::remove)                                                                                                                                                      | *O*(*n* − *i*)                                                       |
//! | [`truncate_back(i)`](CircularBuffer::truncate_back), [`truncate_front(i)`](CircularBuffer::truncate_front)                                                                                 | *O*(*n* − *i*) for types that implement [`Drop`], *O*(1) otherwise   |
//! | [`clear()`](CircularBuffer::clear)                                                                                                                                                         | *O*(*n*) for types that implement [`Drop`], *O*(1) otherwise         |
//! | [`drain(i..j)`](CircularBuffer::drain)                                                                                                                                                     | *O*(*n* − *j*)                                                       |
//! | [`fill()`](CircularBuffer::fill), [`fill_with()`](CircularBuffer::fill_with)                                                                                                               | *O*(*c* + *n*) for types that implement [`Drop`], *O*(*c*) otherwise |
//! | [`fill_spare()`](CircularBuffer::fill_spare), [`fill_spare_with()`](CircularBuffer::fill_spare_with)                                                                                       | *O*(*c* − *n*)                                                       |
//! | [`get()`](CircularBuffer::get), [`front()`](CircularBuffer::front), [`back()`](CircularBuffer::back), [`nth_front()`](CircularBuffer::nth_front), [`nth_back()`](CircularBuffer::nth_back) | *O*(1)                                                               |
//! | [`swap()`](CircularBuffer::swap), [`swap_remove_front()`](CircularBuffer::swap_remove_front), [`swap_remove_back()`](CircularBuffer::swap_remove_back)                                     | *O*(1)                                                               |
//! | [`as_slices()`](CircularBuffer::as_slices), [`as_mut_slices()`](CircularBuffer::as_mut_slices)                                                                                             | *O*(1)                                                               |
//! | [`len()`](CircularBuffer::len), [`capacity()`](CircularBuffer::capacity)                                                                                                                   | *O*(1)                                                               |
//!
//! Notation: *n* is the [length](CircularBuffer::len) of the buffer, *c* is the
//! [capacity](CircularBuffer::capacity) of the buffer, *i* and *j* are variables.
//!
//! # Stack vs heap
//!
//! The [`CircularBuffer`] struct is compact and has a fixed size, so it may live on the stack.
//! This can provide optimal performance for small buffers as memory allocation can be avoided.
//!
//! For large buffers, or for buffers that need to be passed around often, it can be useful to
//! allocate the buffer on the heap. Use a [`Box`](std::boxed) for that:
//!
//! ```
//! # #[cfg(feature = "std")]
//! # {
//! use circular_buffer::CircularBuffer;
//!
//! let mut buf = CircularBuffer::<4096, u32>::boxed();
//! assert_eq!(buf.len(), 0);
//!
//! for i in 0..1024 {
//!     buf.push_back(i);
//! }
//! assert_eq!(buf.len(), 1024);
//!
//! buf.truncate_back(128);
//! assert_eq!(buf.len(), 128);
//! # }
//! ```
//!
//! # `no_std`
//!
//! This crate can be used in a [`no_std` environment], although the I/O features and
//! heap-allocation features won't be available by default in `no_std` mode. By default, this crate
//! uses `std`; to use this crate in `no_std` mode, disable the default features for this crate in
//! your `Cargo.toml`:
//!
//! ```text
//! [dependencies]
//! circular-buffer = { version = "0.1", default-features = false }
//! ```
//!
//! When using `no_std` mode, this crate supports heap-allocation features through the [`alloc`
//! crate](alloc). To enable the use of the `alloc` crate, enable the `alloc` feature:
//!
//! ```text
//! [dependencies]
//! circular-buffer = { version = "0.1", default-features = false, features = ["alloc"] }
//! ```
//!
//! [`no_std` environment]: https://docs.rust-embedded.org/book/intro/no-std.html
//!
//! # Cargo feature flags
//!
//! * `std`: enables support for the [`std` library](https://doc.rust-lang.org/std/) (enabled by
//!   default).
//! * `alloc`: enables support for the [`alloc` crate](https://doc.rust-lang.org/alloc/) (enabled
//!   by default).
//! * `embedded-io`: enables implementation for the
//!   [`embedded_io`](https://docs.rs/embedded-io/) traits.
//! * `embedded-io-async`: enables implementation for the
//!   [`embedded_io_async`](https://docs.rs/embedded-io-async) traits.

#![cfg_attr(not(feature = "std"), no_std)]
#![cfg_attr(feature = "unstable", feature(maybe_uninit_slice))]
#![cfg_attr(feature = "unstable", feature(maybe_uninit_write_slice))]
#![warn(missing_debug_implementations)]
#![warn(missing_docs)]
#![warn(unreachable_pub)]
#![warn(unused_qualifications)]
#![doc(test(attr(deny(warnings))))]

mod drain;
mod iter;

#[cfg(feature = "std")]
mod io;

#[cfg(any(feature = "embedded-io", feature = "embedded-io-async"))]
mod embedded_io;

#[cfg(test)]
mod tests;

use core::cmp::Ordering;
use core::fmt;
use core::hash::Hash;
use core::hash::Hasher;
use core::mem;
use core::mem::MaybeUninit;
use core::ops::Index;
use core::ops::IndexMut;
use core::ops::Range;
use core::ops::RangeBounds;
use core::ptr;

pub use crate::drain::Drain;
pub use crate::iter::IntoIter;
pub use crate::iter::Iter;
pub use crate::iter::IterMut;

#[cfg(feature = "alloc")]
extern crate alloc;

#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::boxed::Box;
#[cfg(all(not(feature = "std"), feature = "alloc"))]
use alloc::vec::Vec;

/// Returns `(x + y) % m` without risk of overflows if `x + y` cannot fit in `usize`.
///
/// `x` and `y` are expected to be less than, or equal to `m`.
#[inline]
const fn add_mod(x: usize, y: usize, m: usize) -> usize {
    debug_assert!(m > 0);
    debug_assert!(x <= m);
    debug_assert!(y <= m);
    let (z, overflow) = x.overflowing_add(y);
    (z + (overflow as usize) * (usize::MAX % m + 1)) % m
}

/// Returns `(x - y) % m` without risk of underflows if `x - y` is negative.
///
/// `x` and `y` are expected to be less than, or equal to `m`.
#[inline]
const fn sub_mod(x: usize, y: usize, m: usize) -> usize {
    debug_assert!(m > 0);
    debug_assert!(x <= m);
    debug_assert!(y <= m);
    add_mod(x, m - y, m)
}

#[inline]
const unsafe fn slice_assume_init_ref<T>(slice: &[MaybeUninit<T>]) -> &[T] {
    #[cfg(feature = "unstable")]
    unsafe {
        slice.assume_init_ref()
    }
    #[cfg(not(feature = "unstable"))]
    unsafe {
        &*(slice as *const [MaybeUninit<T>] as *const [T])
    }
}

#[inline]
unsafe fn slice_assume_init_mut<T>(slice: &mut [MaybeUninit<T>]) -> &mut [T] {
    #[cfg(feature = "unstable")]
    unsafe {
        slice.assume_init_mut()
    }
    #[cfg(not(feature = "unstable"))]
    unsafe {
        &mut *(slice as *mut [MaybeUninit<T>] as *mut [T])
    }
}

/// A fixed-size circular buffer.
///
/// A `CircularBuffer` may live on the stack. Wrap the `CircularBuffer` in a [`Box`](std::boxed)
/// using [`CircularBuffer::boxed()`] if you need the struct to be heap-allocated.
///
/// See the [module-level documentation](self) for more details and examples.
pub struct CircularBuffer<const N: usize, T> {
    size: usize,
    start: usize,
    items: [MaybeUninit<T>; N],
}

impl<const N: usize, T> CircularBuffer<N, T> {
    /// Returns an empty `CircularBuffer`.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    /// let buf = CircularBuffer::<16, u32>::new();
    /// assert_eq!(buf, []);
    /// ```
    #[inline]
    #[must_use]
    pub const fn new() -> Self {
        #[cfg(feature = "unstable")]
        {
            Self {
                size: 0,
                start: 0,
                items: [const { MaybeUninit::uninit() }; N],
            }
        }
        #[cfg(not(feature = "unstable"))]
        {
            Self {
                size: 0,
                start: 0,
                items: unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() },
            }
        }
    }

    /// Returns an empty heap-allocated `CircularBuffer`.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    /// let buf = CircularBuffer::<1024, f64>::boxed();
    /// assert_eq!(buf.len(), 0);
    /// ```
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn boxed() -> Box<Self> {
        let mut uninit: Box<MaybeUninit<Self>> = Box::new_uninit();
        let ptr = uninit.as_mut_ptr();

        unsafe {
            // SAFETY: the pointer contains enough memory to contain `Self` and `addr_of_mut`
            // ensures that the address written to is properly aligned.
            core::ptr::addr_of_mut!((*ptr).size).write(0);
            core::ptr::addr_of_mut!((*ptr).start).write(0);

            // SAFETY: `size` and `start` have been properly initialized to 0; `items` does not
            // need to be initialized if `size` is 0
            uninit.assume_init()
        }
    }

    /// Returns the number of elements in the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<16, u32>::new();
    /// assert_eq!(buf.len(), 0);
    ///
    /// buf.push_back(1);
    /// buf.push_back(2);
    /// buf.push_back(3);
    /// assert_eq!(buf.len(), 3);
    /// ```
    #[inline]
    pub const fn len(&self) -> usize {
        self.size
    }

    /// Returns the capacity of the buffer.
    ///
    /// This is the maximum number of elements that the buffer can hold.
    ///
    /// This method always returns the generic const parameter `N`.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    /// let buf = CircularBuffer::<16, u32>::new();
    /// assert_eq!(buf.capacity(), 16);
    /// ```
    #[inline]
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns `true` if the buffer contains 0 elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<16, u32>::new();
    /// assert!(buf.is_empty());
    ///
    /// buf.push_back(1);
    /// assert!(!buf.is_empty());
    /// ```
    #[inline]
    pub const fn is_empty(&self) -> bool {
        self.size == 0
    }

    /// Returns `true` if the number of elements in the buffer matches the buffer capacity.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<5, u32>::new();
    /// assert!(!buf.is_full());
    ///
    /// buf.push_back(1);
    /// assert!(!buf.is_full());
    ///
    /// buf.push_back(2);
    /// buf.push_back(3);
    /// buf.push_back(4);
    /// buf.push_back(5);
    /// assert!(buf.is_full());
    /// ```
    #[inline]
    pub const fn is_full(&self) -> bool {
        self.size == N
    }

    /// Returns an iterator over the elements of the buffer.
    ///
    /// The iterator advances from front to back. Use [`.rev()`](Iter::rev) to advance from
    /// back to front.
    ///
    /// # Examples
    ///
    /// Iterate from front to back:
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let buf = CircularBuffer::<5, char>::from_iter("abc".chars());
    /// let mut it = buf.iter();
    ///
    /// assert_eq!(it.next(), Some(&'a'));
    /// assert_eq!(it.next(), Some(&'b'));
    /// assert_eq!(it.next(), Some(&'c'));
    /// assert_eq!(it.next(), None);
    /// ```
    ///
    /// Iterate from back to front:
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let buf = CircularBuffer::<5, char>::from_iter("abc".chars());
    /// let mut it = buf.iter().rev();
    ///
    /// assert_eq!(it.next(), Some(&'c'));
    /// assert_eq!(it.next(), Some(&'b'));
    /// assert_eq!(it.next(), Some(&'a'));
    /// assert_eq!(it.next(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn iter(&self) -> Iter<'_, T> {
        Iter::new(self)
    }

    /// Returns an iterator over the elements of the buffer that allows modifying each value.
    ///
    /// The iterator advances from front to back. Use [`.rev()`](Iter::rev) to advance from back to
    /// front.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<5, u32>::from([1, 2, 3]);
    /// for elem in buf.iter_mut() {
    ///     *elem += 5;
    /// }
    /// assert_eq!(buf, [6, 7, 8]);
    /// ```
    #[inline]
    #[must_use]
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        IterMut::new(self)
    }

    /// Returns an iterator over the specified range of elements of the buffer.
    ///
    /// The iterator advances from front to back. Use [`.rev()`](Iter::rev) to advance from back to
    /// front.
    ///
    /// # Panics
    ///
    /// If the start of the range is greater than the end, or if the end is greater than the length
    /// of the buffer.
    ///
    /// # Examples
    ///
    /// Iterate from front to back:
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let buf = CircularBuffer::<16, char>::from_iter("abcdefghi".chars());
    /// let mut it = buf.range(3..6);
    ///
    /// assert_eq!(it.next(), Some(&'d'));
    /// assert_eq!(it.next(), Some(&'e'));
    /// assert_eq!(it.next(), Some(&'f'));
    /// assert_eq!(it.next(), None);
    /// ```
    ///
    /// Iterate from back to front:
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let buf = CircularBuffer::<16, char>::from_iter("abcdefghi".chars());
    /// let mut it = buf.range(3..6).rev();
    ///
    /// assert_eq!(it.next(), Some(&'f'));
    /// assert_eq!(it.next(), Some(&'e'));
    /// assert_eq!(it.next(), Some(&'d'));
    /// assert_eq!(it.next(), None);
    /// ```
    #[inline]
    #[must_use]
    pub fn range<R>(&self, range: R) -> Iter<'_, T>
    where
        R: RangeBounds<usize>,
    {
        Iter::over_range(self, range)
    }

    /// Returns an iterator over the specified range of elements of the buffer that allows
    /// modifying each value.
    ///
    /// The iterator advances from front to back. Use [`.rev()`](Iter::rev) to advance from back to
    /// front.
    ///
    /// # Panics
    ///
    /// If the start of the range is greater than the end, or if the end is greater than the length
    /// of the buffer.
    ///
    /// # Examples
    ///
    /// Iterate from front to back:
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<16, i32>::from_iter([1, 2, 3, 4, 5, 6]);
    /// for elem in buf.range_mut(..3) {
    ///     *elem *= -1;
    /// }
    /// assert_eq!(buf, [-1, -2, -3, 4, 5, 6]);
    /// ```
    #[inline]
    #[must_use]
    pub fn range_mut<R>(&mut self, range: R) -> IterMut<'_, T>
    where
        R: RangeBounds<usize>,
    {
        IterMut::over_range(self, range)
    }

    /// Removes the specified range from the buffer in bulk, returning the removed elements as an
    /// iterator. If the iterator is dropped before being fully consumed, it drops the remaining
    /// removed elements.
    ///
    /// # Panics
    ///
    /// If the start of the range is greater than the end, or if the end is greater than the length
    /// of the buffer.
    ///
    /// # Leaking
    ///
    /// If the returned iterator goes out of scope without being dropped (for example, due to
    /// calling [`mem::forget()`] on it), the buffer may have lost and leaked arbitrary elements,
    /// including elements outside of the range.
    ///
    /// The current implementation leaks all the elements of the buffer if the iterator is leaked,
    /// but this behavior may change in the future.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<6, char>::from_iter("abcdef".chars());
    /// let drained = buf.drain(3..).collect::<Vec<char>>();
    ///
    /// assert_eq!(drained, ['d', 'e', 'f']);
    /// assert_eq!(buf, ['a', 'b', 'c']);
    /// ```
    ///
    /// Not consuming the draining iterator still removes the range of elements:
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<6, char>::from_iter("abcdef".chars());
    /// buf.drain(3..);
    ///
    /// assert_eq!(buf, ['a', 'b', 'c']);
    /// ```
    #[inline]
    pub fn drain<R>(&mut self, range: R) -> Drain<'_, N, T>
    where
        R: RangeBounds<usize>,
    {
        Drain::over_range(self, range)
    }

    /// Rearranges the internal memory of the buffer so that all elements are in a contiguous
    /// slice, which is then returned.
    ///
    /// This method does not allocate and does not change the order of the inserted elements.
    /// Because it returns a mutable slice, any [slice methods](slice) may be called on the
    /// elements of the buffer, such as sorting methods.
    ///
    /// Once the internal storage is contiguous, the [`as_slices()`](CircularBuffer::as_slices) and
    /// [`as_mut_slices()`](CircularBuffer::as_mut_slices) methods will return the entire contents
    /// of the deque in a single slice. Adding new elements to the buffer may make the buffer
    /// disjoint (not contiguous).
    ///
    /// # Complexity
    ///
    /// If the buffer is disjoint (not contiguous), this method takes *O*(*N*) time, where *N* is
    /// the capacity of the buffer.
    ///
    /// If the buffer is already contiguous, this method takes *O*(1) time.
    ///
    /// This means that this method may be called multiple times on the same buffer without a
    /// performance penalty (provided that no new elements are added to the buffer in between
    /// calls).
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// // Create a new buffer, adding more elements than its capacity
    /// let mut buf = CircularBuffer::<4, u32>::from_iter([1, 4, 3, 0, 2, 5]);
    /// assert_eq!(buf, [3, 0, 2, 5]);
    ///
    /// // The buffer is disjoint: as_slices() returns two non-empty slices
    /// assert_eq!(buf.as_slices(), (&[3, 0][..], &[2, 5][..]));
    ///
    /// // Make the buffer contiguous
    /// assert_eq!(buf.make_contiguous(), &mut [3, 0, 2, 5]);
    /// // as_slices() now returns a single non-empty slice
    /// assert_eq!(buf.as_slices(), (&[3, 0, 2, 5][..], &[][..]));
    /// // The order of the elements in the buffer did not get modified
    /// assert_eq!(buf, [3, 0, 2, 5]);
    ///
    /// // Make the buffer contiguous and sort its elements
    /// buf.make_contiguous().sort();
    /// assert_eq!(buf, [0, 2, 3, 5]);
    /// ```
    pub fn make_contiguous(&mut self) -> &mut [T] {
        if N == 0 || self.size == 0 {
            return &mut [];
        }

        debug_assert!(self.start < N, "start out-of-bounds");
        debug_assert!(self.size <= N, "size out-of-bounds");

        let start = self.start;
        let end = add_mod(self.start, self.size, N);

        let slice = if start < end {
            // Already contiguous; nothing to do
            &mut self.items[start..end]
        } else {
            // Not contiguous; need to rotate
            self.start = 0;
            self.items.rotate_left(start);
            &mut self.items[..self.size]
        };

        // SAFETY: The elements in the slice are guaranteed to be initialized
        unsafe { slice_assume_init_mut(slice) }
    }

    /// Returns a pair of slices which contain the elements of this buffer.
    ///
    /// The second slice may be empty if the internal buffer is contiguous.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<4, char>::new();
    /// buf.push_back('a');
    /// buf.push_back('b');
    /// buf.push_back('c');
    /// buf.push_back('d');
    ///
    /// // Buffer is contiguous; second slice is empty
    /// assert_eq!(buf.as_slices(), (&['a', 'b', 'c', 'd'][..], &[][..]));
    ///
    /// buf.push_back('e');
    /// buf.push_back('f');
    ///
    /// // Buffer is disjoint; both slices are non-empty
    /// assert_eq!(buf.as_slices(), (&['c', 'd'][..], &['e', 'f'][..]));
    /// ```
    #[inline]
    pub fn as_slices(&self) -> (&[T], &[T]) {
        if N == 0 || self.size == 0 {
            return (&[], &[]);
        }

        debug_assert!(self.start < N, "start out-of-bounds");
        debug_assert!(self.size <= N, "size out-of-bounds");

        let start = self.start;
        let end = add_mod(self.start, self.size, N);

        let (front, back) = if start < end {
            (&self.items[start..end], &[][..])
        } else {
            let (back, front) = self.items.split_at(start);
            (front, &back[..end])
        };

        // SAFETY: The elements in these slices are guaranteed to be initialized
        unsafe { (slice_assume_init_ref(front), slice_assume_init_ref(back)) }
    }

    /// Returns a pair of mutable slices which contain the elements of this buffer.
    ///
    /// These slices can be used to modify or replace the elements in the buffer.
    ///
    /// The second slice may be empty if the internal buffer is contiguous.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<4, char>::new();
    /// buf.push_back('a');
    /// buf.push_back('b');
    /// buf.push_back('c');
    /// buf.push_back('d');
    /// buf.push_back('e');
    /// buf.push_back('f');
    ///
    /// assert_eq!(buf, ['c', 'd', 'e', 'f']);
    ///
    /// let (left, right) = buf.as_mut_slices();
    /// assert_eq!(left, &mut ['c', 'd'][..]);
    /// assert_eq!(right, &mut ['e', 'f'][..]);
    ///
    /// left[0] = 'z';
    ///
    /// assert_eq!(buf, ['z', 'd', 'e', 'f']);
    /// ```
    #[inline]
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        if N == 0 || self.size == 0 {
            return (&mut [][..], &mut [][..]);
        }

        debug_assert!(self.start < N, "start out-of-bounds");
        debug_assert!(self.size <= N, "size out-of-bounds");

        let start = self.start;
        let end = add_mod(self.start, self.size, N);

        let (front, back) = if start < end {
            (&mut self.items[start..end], &mut [][..])
        } else {
            let (back, front) = self.items.split_at_mut(start);
            (front, &mut back[..end])
        };

        // SAFETY: The elements in these slices are guaranteed to be initialized
        unsafe { (slice_assume_init_mut(front), slice_assume_init_mut(back)) }
    }

    #[inline]
    fn front_maybe_uninit_mut(&mut self) -> &mut MaybeUninit<T> {
        debug_assert!(self.size > 0, "empty buffer");
        debug_assert!(self.start < N, "start out-of-bounds");
        &mut self.items[self.start]
    }

    #[inline]
    const fn front_maybe_uninit(&self) -> &MaybeUninit<T> {
        debug_assert!(self.size > 0, "empty buffer");
        debug_assert!(self.size <= N, "size out-of-bounds");
        debug_assert!(self.start < N, "start out-of-bounds");
        &self.items[self.start]
    }

    #[inline]
    const fn back_maybe_uninit(&self) -> &MaybeUninit<T> {
        debug_assert!(self.size > 0, "empty buffer");
        debug_assert!(self.size <= N, "size out-of-bounds");
        debug_assert!(self.start < N, "start out-of-bounds");
        let back = add_mod(self.start, self.size - 1, N);
        &self.items[back]
    }

    #[inline]
    fn back_maybe_uninit_mut(&mut self) -> &mut MaybeUninit<T> {
        debug_assert!(self.size > 0, "empty buffer");
        debug_assert!(self.size <= N, "size out-of-bounds");
        debug_assert!(self.start < N, "start out-of-bounds");
        let back = add_mod(self.start, self.size - 1, N);
        &mut self.items[back]
    }

    #[inline]
    const fn get_maybe_uninit(&self, index: usize) -> &MaybeUninit<T> {
        debug_assert!(self.size > 0, "empty buffer");
        debug_assert!(index < N, "index out-of-bounds");
        debug_assert!(self.start < N, "start out-of-bounds");
        let index = add_mod(self.start, index, N);
        &self.items[index]
    }

    #[inline]
    fn get_maybe_uninit_mut(&mut self, index: usize) -> &mut MaybeUninit<T> {
        debug_assert!(self.size > 0, "empty buffer");
        debug_assert!(index < N, "index out-of-bounds");
        debug_assert!(self.start < N, "start out-of-bounds");
        let index = add_mod(self.start, index, N);
        &mut self.items[index]
    }

    #[inline]
    fn slices_uninit_mut(&mut self) -> (&mut [MaybeUninit<T>], &mut [MaybeUninit<T>]) {
        if N == 0 {
            return (&mut [][..], &mut [][..]);
        }

        debug_assert!(self.start < N, "start out-of-bounds");
        debug_assert!(self.size <= N, "size out-of-bounds");

        let start = self.start;
        let end = add_mod(start, self.size, N);
        if end < start {
            (&mut self.items[end..start], &mut [][..])
        } else {
            let (left, right) = self.items.split_at_mut(end);
            let left = &mut left[..start];
            (right, left)
        }
    }

    #[inline]
    fn inc_start(&mut self) {
        debug_assert!(self.start < N, "start out-of-bounds");
        self.start = add_mod(self.start, 1, N);
    }

    #[inline]
    fn dec_start(&mut self) {
        debug_assert!(self.start < N, "start out-of-bounds");
        self.start = sub_mod(self.start, 1, N);
    }

    #[inline]
    fn inc_size(&mut self) {
        debug_assert!(self.size <= N, "size out-of-bounds");
        debug_assert!(self.size < N, "size at capacity limit");
        self.size += 1;
    }

    #[inline]
    fn dec_size(&mut self) {
        debug_assert!(self.size > 0, "size is 0");
        self.size -= 1;
    }

    #[inline]
    unsafe fn drop_range(&mut self, range: Range<usize>) {
        if range.is_empty() {
            return;
        }

        debug_assert!(self.start < N, "start out-of-bounds");
        debug_assert!(self.size <= N, "size out-of-bounds");
        debug_assert!(range.start < self.size, "start of range out-of-bounds");
        debug_assert!(range.end <= self.size, "end of range out-of-bounds");
        debug_assert!(range.start < range.end, "start of range is past its end");
        debug_assert!(
            range.start == 0 || range.end == self.size,
            "range does not include boundary of the buffer"
        );

        // Drops all the items in the slice when dropped. This is needed to ensure that all
        // elements are dropped in case a panic occurs during the drop of a single element.
        struct Dropper<'a, T>(&'a mut [MaybeUninit<T>]);

        impl<T> Drop for Dropper<'_, T> {
            #[inline]
            fn drop(&mut self) {
                // SAFETY: the caller of `drop_range` is responsible to check that this slice was
                // initialized.
                unsafe {
                    ptr::drop_in_place(slice_assume_init_mut(self.0));
                }
            }
        }

        let drop_from = add_mod(self.start, range.start, N);
        let drop_to = add_mod(self.start, range.end, N);

        let (right, left) = if drop_from < drop_to {
            (&mut self.items[drop_from..drop_to], &mut [][..])
        } else {
            let (left, right) = self.items.split_at_mut(drop_from);
            let left = &mut left[..drop_to];
            (right, left)
        };

        let _left = Dropper(left);
        let _right = Dropper(right);
    }

    /// Returns a reference to the back element, or `None` if the buffer is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<4, char>::new();
    /// assert_eq!(buf.back(), None);
    ///
    /// buf.push_back('a');
    /// buf.push_back('b');
    /// buf.push_back('c');
    /// assert_eq!(buf.back(), Some(&'c'));
    /// ```
    #[inline]
    pub fn back(&self) -> Option<&T> {
        if N == 0 || self.size == 0 {
            // Nothing to do
            return None;
        }
        // SAFETY: `size` is non-zero; back element is guaranteed to be initialized
        Some(unsafe { self.back_maybe_uninit().assume_init_ref() })
    }

    /// Returns a mutable reference to the back element, or `None` if the buffer is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<4, char>::new();
    /// assert_eq!(buf.back_mut(), None);
    ///
    /// buf.push_back('a');
    /// buf.push_back('b');
    /// buf.push_back('c');
    /// match buf.back_mut() {
    ///     None => (),
    ///     Some(x) => *x = 'z',
    /// }
    /// assert_eq!(buf, ['a', 'b', 'z']);
    /// ```
    #[inline]
    pub fn back_mut(&mut self) -> Option<&mut T> {
        if N == 0 || self.size == 0 {
            // Nothing to do
            return None;
        }
        // SAFETY: `size` is non-zero; back element is guaranteed to be initialized
        Some(unsafe { self.back_maybe_uninit_mut().assume_init_mut() })
    }

    /// Returns a reference to the front element, or `None` if the buffer is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<4, char>::new();
    /// assert_eq!(buf.front(), None);
    ///
    /// buf.push_back('a');
    /// buf.push_back('b');
    /// buf.push_back('c');
    /// assert_eq!(buf.front(), Some(&'a'));
    /// ```
    #[inline]
    pub fn front(&self) -> Option<&T> {
        if N == 0 || self.size == 0 {
            // Nothing to do
            return None;
        }
        // SAFETY: `size` is non-zero; front element is guaranteed to be initialized
        Some(unsafe { self.front_maybe_uninit().assume_init_ref() })
    }

    /// Returns a mutable reference to the front element, or `None` if the buffer is empty.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<4, char>::new();
    /// assert_eq!(buf.front_mut(), None);
    ///
    /// buf.push_back('a');
    /// buf.push_back('b');
    /// buf.push_back('c');
    /// match buf.front_mut() {
    ///     None => (),
    ///     Some(x) => *x = 'z',
    /// }
    /// assert_eq!(buf, ['z', 'b', 'c']);
    /// ```
    #[inline]
    pub fn front_mut(&mut self) -> Option<&mut T> {
        if N == 0 || self.size == 0 {
            // Nothing to do
            return None;
        }
        // SAFETY: `size` is non-zero; front element is guaranteed to be initialized
        Some(unsafe { self.front_maybe_uninit_mut().assume_init_mut() })
    }

    /// Returns a reference to the element at the given index from the front of the buffer, or
    /// `None` if the element does not exist.
    ///
    /// Element at index 0 is the front of the queue.
    ///
    /// This is the same as [`nth_front()`](CircularBuffer::nth_front).
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<5, char>::new();
    /// assert_eq!(buf.get(1), None);
    ///
    /// buf.push_back('a');
    /// buf.push_back('b');
    /// buf.push_back('c');
    /// buf.push_back('d');
    /// assert_eq!(buf.get(1), Some(&'b'));
    /// ```
    #[inline]
    pub fn get(&self, index: usize) -> Option<&T> {
        if N == 0 || index >= self.size {
            // Nothing to do
            return None;
        }
        // SAFETY: `index` is in a valid range; it is guaranteed to point to an initialized element
        Some(unsafe { self.get_maybe_uninit(index).assume_init_ref() })
    }

    /// Returns a mutable reference to the element at the given index, or `None` if the element
    /// does not exist.
    ///
    /// Element at index 0 is the front of the queue.
    ///
    /// This is the same as [`nth_front_mut()`](CircularBuffer::nth_front_mut).
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<5, char>::new();
    /// assert_eq!(buf.get_mut(1), None);
    ///
    /// buf.push_back('a');
    /// buf.push_back('b');
    /// buf.push_back('c');
    /// buf.push_back('d');
    /// match buf.get_mut(1) {
    ///     None => (),
    ///     Some(x) => *x = 'z',
    /// }
    /// assert_eq!(buf, ['a', 'z', 'c', 'd']);
    /// ```
    #[inline]
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if N == 0 || index >= self.size {
            // Nothing to do
            return None;
        }
        // SAFETY: `index` is in a valid range; it is guaranteed to point to an initialized element
        Some(unsafe { self.get_maybe_uninit_mut(index).assume_init_mut() })
    }

    /// Returns a reference to the element at the given index from the front of the buffer, or
    /// `None` if the element does not exist.
    ///
    /// Like most indexing operations, the count starts from zero, so `nth_front(0)` returns the
    /// first value, `nth_front(1)` the second, and so on. Element at index 0 is the front of the
    /// queue.
    ///
    /// This is the same as [`get()`](CircularBuffer::get).
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<5, char>::new();
    /// assert_eq!(buf.nth_front(1), None);
    ///
    /// buf.push_back('a');
    /// buf.push_back('b');
    /// buf.push_back('c');
    /// buf.push_back('d');
    /// assert_eq!(buf.nth_front(1), Some(&'b'));
    /// ```
    #[inline]
    pub fn nth_front(&self, index: usize) -> Option<&T> {
        self.get(index)
    }

    /// Returns a mutable reference to the element at the given index from the front of the buffer,
    /// or `None` if the element does not exist.
    ///
    /// Like most indexing operations, the count starts from zero, so `nth_front_mut(0)` returns
    /// the first value, `nth_front_mut(1)` the second, and so on. Element at index 0 is the front
    /// of the queue.
    ///
    /// This is the same as [`get_mut()`](CircularBuffer::get_mut).
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<5, char>::new();
    /// assert_eq!(buf.nth_front_mut(1), None);
    ///
    /// buf.push_back('a');
    /// buf.push_back('b');
    /// buf.push_back('c');
    /// buf.push_back('d');
    /// match buf.nth_front_mut(1) {
    ///     None => (),
    ///     Some(x) => *x = 'z',
    /// }
    /// assert_eq!(buf, ['a', 'z', 'c', 'd']);
    /// ```
    #[inline]
    pub fn nth_front_mut(&mut self, index: usize) -> Option<&mut T> {
        self.get_mut(index)
    }

    /// Returns a reference to the element at the given index from the back of the buffer, or
    /// `None` if the element does not exist.
    ///
    /// Like most indexing operations, the count starts from zero, so `nth_back(0)` returns the
    /// first value, `nth_back(1)` the second, and so on. Element at index 0 is the back of the
    /// queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<5, char>::new();
    /// assert_eq!(buf.nth_back(1), None);
    ///
    /// buf.push_back('a');
    /// buf.push_back('b');
    /// buf.push_back('c');
    /// buf.push_back('d');
    /// assert_eq!(buf.nth_back(1), Some(&'c'));
    /// ```
    #[inline]
    pub fn nth_back(&self, index: usize) -> Option<&T> {
        let index = self.size.checked_sub(index)?.checked_sub(1)?;
        self.get(index)
    }

    /// Returns a mutable reference to the element at the given index from the back of the buffer,
    /// or `None` if the element does not exist.
    ///
    /// Like most indexing operations, the count starts from zero, so `nth_back_mut(0)` returns the
    /// first value, `nth_back_mut(1)` the second, and so on. Element at index 0 is the back of the
    /// queue.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<5, char>::new();
    /// assert_eq!(buf.nth_back_mut(1), None);
    ///
    /// buf.push_back('a');
    /// buf.push_back('b');
    /// buf.push_back('c');
    /// buf.push_back('d');
    /// match buf.nth_back_mut(1) {
    ///     None => (),
    ///     Some(x) => *x = 'z',
    /// }
    /// assert_eq!(buf, ['a', 'b', 'z', 'd']);
    /// ```
    #[inline]
    pub fn nth_back_mut(&mut self, index: usize) -> Option<&mut T> {
        let index = self.size.checked_sub(index)?.checked_sub(1)?;
        self.get_mut(index)
    }

    /// Appends an element to the back of the buffer.
    ///
    /// If the buffer is full, the element at the front of the buffer is overwritten and returned.
    ///
    /// See also [`try_push_back()`](CircularBuffer::try_push_back) for a non-overwriting version
    /// of this method.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<3, char>::new();
    ///
    /// assert_eq!(buf.push_back('a'), None);
    /// assert_eq!(buf, ['a']);
    ///
    /// assert_eq!(buf.push_back('b'), None);
    /// assert_eq!(buf, ['a', 'b']);
    ///
    /// assert_eq!(buf.push_back('c'), None);
    /// assert_eq!(buf, ['a', 'b', 'c']);
    ///
    /// // The buffer is now full; adding more values causes the front elements to be removed and
    /// // returned
    /// assert_eq!(buf.push_back('d'), Some('a'));
    /// assert_eq!(buf, ['b', 'c', 'd']);
    ///
    /// assert_eq!(buf.push_back('e'), Some('b'));
    /// assert_eq!(buf, ['c', 'd', 'e']);
    ///
    /// assert_eq!(buf.push_back('f'), Some('c'));
    /// assert_eq!(buf, ['d', 'e', 'f']);
    /// ```
    pub fn push_back(&mut self, item: T) -> Option<T> {
        if N == 0 {
            // Nothing to do
            return Some(item);
        }

        if self.size >= N {
            // At capacity; need to replace the front item
            //
            // SAFETY: if size is greater than 0, the front item is guaranteed to be initialized.
            let replaced_item = mem::replace(
                unsafe { self.front_maybe_uninit_mut().assume_init_mut() },
                item,
            );
            self.inc_start();
            Some(replaced_item)
        } else {
            // Some uninitialized slots left; append at the end
            self.inc_size();
            self.back_maybe_uninit_mut().write(item);
            None
        }
    }

    /// Appends an element to the back of the buffer.
    ///
    /// If the buffer is full, the buffer is not modified and the given element is returned as an
    /// error.
    ///
    /// See also [`push_back()`](CircularBuffer::push_back) for a version of this method that
    /// overwrites the front of the buffer when full.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<3, char>::new();
    ///
    /// assert_eq!(buf.try_push_back('a'), Ok(()));
    /// assert_eq!(buf, ['a']);
    ///
    /// assert_eq!(buf.try_push_back('b'), Ok(()));
    /// assert_eq!(buf, ['a', 'b']);
    ///
    /// assert_eq!(buf.try_push_back('c'), Ok(()));
    /// assert_eq!(buf, ['a', 'b', 'c']);
    ///
    /// // The buffer is now full; adding more values results in an error
    /// assert_eq!(buf.try_push_back('d'), Err('d'))
    /// ```
    pub fn try_push_back(&mut self, item: T) -> Result<(), T> {
        if N == 0 {
            // Nothing to do
            return Ok(());
        }
        if self.size >= N {
            // At capacity; return the pushed item as error
            Err(item)
        } else {
            // Some uninitialized slots left; append at the end
            self.inc_size();
            self.back_maybe_uninit_mut().write(item);
            Ok(())
        }
    }

    /// Appends an element to the front of the buffer.
    ///
    /// If the buffer is full, the element at the back of the buffer is overwritten and returned.
    ///
    /// See also [`try_push_front()`](CircularBuffer::try_push_front) for a non-overwriting version
    /// of this method.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<3, char>::new();
    ///
    /// assert_eq!(buf.push_front('a'), None);
    /// assert_eq!(buf, ['a']);
    ///
    /// assert_eq!(buf.push_front('b'), None);
    /// assert_eq!(buf, ['b', 'a']);
    ///
    /// assert_eq!(buf.push_front('c'), None);
    /// assert_eq!(buf, ['c', 'b', 'a']);
    ///
    /// // The buffer is now full; adding more values causes the back elements to be dropped
    /// assert_eq!(buf.push_front('d'), Some('a'));
    /// assert_eq!(buf, ['d', 'c', 'b']);
    ///
    /// assert_eq!(buf.push_front('e'), Some('b'));
    /// assert_eq!(buf, ['e', 'd', 'c']);
    ///
    /// assert_eq!(buf.push_front('f'), Some('c'));
    /// assert_eq!(buf, ['f', 'e', 'd']);
    /// ```
    pub fn push_front(&mut self, item: T) -> Option<T> {
        if N == 0 {
            // Nothing to do
            return Some(item);
        }

        if self.size >= N {
            // At capacity; need to replace the back item
            //
            // SAFETY: if size is greater than 0, the back item is guaranteed to be initialized.
            let replaced_item = mem::replace(
                unsafe { self.back_maybe_uninit_mut().assume_init_mut() },
                item,
            );
            self.dec_start();
            Some(replaced_item)
        } else {
            // Some uninitialized slots left; insert at the start
            self.inc_size();
            self.dec_start();
            self.front_maybe_uninit_mut().write(item);
            None
        }
    }

    /// Appends an element to the front of the buffer.
    ///
    /// If the buffer is full, the buffer is not modified and the given element is returned as an
    /// error.
    ///
    /// See also [`push_front()`](CircularBuffer::push_front) for a version of this method that
    /// overwrites the back of the buffer when full.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<3, char>::new();
    ///
    /// assert_eq!(buf.try_push_front('a'), Ok(()));
    /// assert_eq!(buf, ['a']);
    ///
    /// assert_eq!(buf.try_push_front('b'), Ok(()));
    /// assert_eq!(buf, ['b', 'a']);
    ///
    /// assert_eq!(buf.try_push_front('c'), Ok(()));
    /// assert_eq!(buf, ['c', 'b', 'a']);
    ///
    /// // The buffer is now full; adding more values results in an error
    /// assert_eq!(buf.try_push_front('d'), Err('d'));
    /// ```
    pub fn try_push_front(&mut self, item: T) -> Result<(), T> {
        if N == 0 {
            // Nothing to do
            return Ok(());
        }
        if self.size >= N {
            // At capacity; return the pushed item as error
            Err(item)
        } else {
            // Some uninitialized slots left; insert at the start
            self.inc_size();
            self.dec_start();
            self.front_maybe_uninit_mut().write(item);
            Ok(())
        }
    }

    /// Removes and returns an element from the back of the buffer.
    ///
    /// If the buffer is empty, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<3, char>::from(['a', 'b', 'c']);
    ///
    /// assert_eq!(buf.pop_back(), Some('c'));
    /// assert_eq!(buf.pop_back(), Some('b'));
    /// assert_eq!(buf.pop_back(), Some('a'));
    /// assert_eq!(buf.pop_back(), None);
    /// ```
    pub fn pop_back(&mut self) -> Option<T> {
        if N == 0 || self.size == 0 {
            // Nothing to do
            return None;
        }

        // SAFETY: if size is greater than 0, the back item is guaranteed to be initialized.
        let back = unsafe { self.back_maybe_uninit().assume_init_read() };
        self.dec_size();
        Some(back)
    }

    /// Removes and returns an element from the front of the buffer.
    ///
    /// If the buffer is empty, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<3, char>::from(['a', 'b', 'c']);
    ///
    /// assert_eq!(buf.pop_front(), Some('a'));
    /// assert_eq!(buf.pop_front(), Some('b'));
    /// assert_eq!(buf.pop_front(), Some('c'));
    /// assert_eq!(buf.pop_front(), None);
    /// ```
    pub fn pop_front(&mut self) -> Option<T> {
        if N == 0 || self.size == 0 {
            // Nothing to do
            return None;
        }

        // SAFETY: if size is greater than 0, the front item is guaranteed to be initialized.
        let front = unsafe { self.front_maybe_uninit().assume_init_read() };
        self.dec_size();
        self.inc_start();
        Some(front)
    }

    /// Removes and returns an element at the specified index.
    ///
    /// If the index is out of bounds, `None` is returned.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<3, char>::from(['a', 'b', 'c']);
    ///
    /// assert_eq!(buf.remove(1), Some('b'));
    /// assert_eq!(buf, ['a', 'c']);
    ///
    /// assert_eq!(buf.remove(5), None);
    /// ```
    pub fn remove(&mut self, index: usize) -> Option<T> {
        if N == 0 || index >= self.size {
            return None;
        }

        let index = add_mod(self.start, index, N);
        let back_index = add_mod(self.start, self.size - 1, N);

        // SAFETY: `index` is in a valid range; the element is guaranteed to be initialized
        let item = unsafe { self.items[index].assume_init_read() };

        // SAFETY: the pointers being moved are in a valid range; the elements behind those
        // pointers are guaranteed to be initialized
        unsafe {
            // TODO: optimize for the case where `index < len - index` (i.e. when copying items to
            // the right is cheaper than moving items to the left)
            let ptr = self.items.as_mut_ptr();
            if back_index >= index {
                // Move the values at the right of `index` by 1 position to the left
                ptr::copy(ptr.add(index).add(1), ptr.add(index), back_index - index);
            } else {
                // Move the values at the right of `index` by 1 position to the left
                ptr::copy(ptr.add(index).add(1), ptr.add(index), N - index - 1);
                // Move the leftmost value to the end of the array
                ptr::copy(ptr, ptr.add(N - 1), 1);
                // Move the values at the left of `back_index` by 1 position to the left
                ptr::copy(ptr.add(1), ptr, back_index);
            }
        }

        self.dec_size();
        Some(item)
    }

    /// Swap the element at index `i` with the element at index `j`.
    ///
    /// # Panics
    ///
    /// If either `i` or `j` is out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<5, char>::from(['a', 'b', 'c', 'd']);
    /// assert_eq!(buf, ['a', 'b', 'c', 'd']);
    ///
    /// buf.swap(0, 3);
    /// assert_eq!(buf, ['d', 'b', 'c', 'a']);
    /// ```
    ///
    /// Trying to swap an invalid index panics:
    ///
    /// ```should_panic
    /// use circular_buffer::CircularBuffer;
    /// let mut buf = CircularBuffer::<5, char>::from(['a', 'b', 'c', 'd']);
    /// buf.swap(0, 7);
    /// ```
    pub fn swap(&mut self, i: usize, j: usize) {
        assert!(i < self.size, "i index out-of-bounds");
        assert!(j < self.size, "j index out-of-bounds");
        if i != j {
            let i = add_mod(self.start, i, N);
            let j = add_mod(self.start, j, N);
            // SAFETY: these are valid pointers
            unsafe { ptr::swap_nonoverlapping(&mut self.items[i], &mut self.items[j], 1) };
        }
    }

    /// Removes the element at `index` and returns it, replacing it with the back of the buffer.
    ///
    /// Returns `None` if `index` is out-of-bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<5, char>::from(['a', 'b', 'c', 'd']);
    /// assert_eq!(buf, ['a', 'b', 'c', 'd']);
    ///
    /// assert_eq!(buf.swap_remove_back(2), Some('c'));
    /// assert_eq!(buf, ['a', 'b', 'd']);
    ///
    /// assert_eq!(buf.swap_remove_back(7), None);
    /// ```
    pub fn swap_remove_back(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }
        self.swap(index, self.size - 1);
        self.pop_back()
    }

    /// Removes the element at `index` and returns it, replacing it with the front of the buffer.
    ///
    /// Returns `None` if `index` is out-of-bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<5, char>::from(['a', 'b', 'c', 'd']);
    /// assert_eq!(buf, ['a', 'b', 'c', 'd']);
    ///
    /// assert_eq!(buf.swap_remove_front(2), Some('c'));
    /// assert_eq!(buf, ['b', 'a', 'd']);
    ///
    /// assert_eq!(buf.swap_remove_front(7), None);
    /// ```
    pub fn swap_remove_front(&mut self, index: usize) -> Option<T> {
        if index >= self.size {
            return None;
        }
        self.swap(index, 0);
        self.pop_front()
    }

    /// Fills the entire capacity of `self` with elements by cloning `value`.
    ///
    /// The elements already present in the buffer (if any) are all replaced by clones of `value`,
    /// and the spare capacity of the buffer is also filled with clones of `value`.
    ///
    /// This is equivalent to clearing the buffer and adding clones of `value` until reaching the
    /// maximum capacity.
    ///
    /// If you want to replace only the existing elements of the buffer, without affecting the
    /// spare capacity, use [`as_mut_slices()`](CircularBuffer::as_mut_slices) and call
    /// [`slice::fill()`]([]::fill) on the resulting slices.
    ///
    /// See also: [`fill_with()`](CircularBuffer::fill_with),
    /// [`fill_spare()`](CircularBuffer::fill_spare),
    /// [`fill_spare_with()`](CircularBuffer::fill_spare_with).
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<10, u32>::from([1, 2, 3]);
    /// assert_eq!(buf, [1, 2, 3]);
    ///
    /// buf.fill(9);
    /// assert_eq!(buf, [9, 9, 9, 9, 9, 9, 9, 9, 9, 9]);
    /// ```
    ///
    /// If you want to replace existing elements only:
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<10, u32>::from([1, 2, 3]);
    /// assert_eq!(buf, [1, 2, 3]);
    ///
    /// let (front, back) = buf.as_mut_slices();
    /// front.fill(9);
    /// back.fill(9);
    /// assert_eq!(buf, [9, 9, 9]);
    /// ```
    pub fn fill(&mut self, value: T)
    where
        T: Clone,
    {
        self.clear();
        self.fill_spare(value);
    }

    /// Fills the entire capacity of `self` with elements by calling a closure.
    ///
    /// The elements already present in the buffer (if any) are all replaced by the result of the
    /// closure, and the spare capacity of the buffer is also filled with the result of the
    /// closure.
    ///
    /// This is equivalent to clearing the buffer and adding the result of the closure until
    /// reaching the maximum capacity.
    ///
    /// If you want to replace only the existing elements of the buffer, without affecting the
    /// spare capacity, use [`as_mut_slices()`](CircularBuffer::as_mut_slices) and call
    /// [`slice::fill_with()`]([]::fill_with) on the resulting slices.
    ///
    /// See also: [`fill()`](CircularBuffer::fill), [`fill_spare()`](CircularBuffer::fill_spare),
    /// [`fill_spare_with()`](CircularBuffer::fill_spare_with).
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<10, u32>::from([1, 2, 3]);
    /// assert_eq!(buf, [1, 2, 3]);
    ///
    /// let mut x = 2;
    /// buf.fill_with(|| {
    ///     x *= 2;
    ///     x
    /// });
    /// assert_eq!(buf, [4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048]);
    /// ```
    ///
    /// If you want to replace existing elements only:
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<10, u32>::from([1, 2, 3]);
    /// assert_eq!(buf, [1, 2, 3]);
    ///
    /// let mut x = 2;
    /// let (front, back) = buf.as_mut_slices();
    /// front.fill_with(|| {
    ///     x *= 2;
    ///     x
    /// });
    /// back.fill_with(|| {
    ///     x *= 2;
    ///     x
    /// });
    /// assert_eq!(buf, [4, 8, 16]);
    /// ```
    pub fn fill_with<F>(&mut self, f: F)
    where
        F: FnMut() -> T,
    {
        self.clear();
        self.fill_spare_with(f);
    }

    /// Fills the spare capacity of `self` with elements by cloning `value`.
    ///
    /// The elements already present in the buffer (if any) are unaffected.
    ///
    /// This is equivalent to adding clones of `value` to the buffer until reaching the maximum
    /// capacity.
    ///
    /// See also: [`fill()`](CircularBuffer::fill), [`fill_with()`](CircularBuffer::fill_with),
    /// [`fill_spare_with()`](CircularBuffer::fill_spare_with).
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<10, u32>::from([1, 2, 3]);
    /// assert_eq!(buf, [1, 2, 3]);
    ///
    /// buf.fill_spare(9);
    /// assert_eq!(buf, [1, 2, 3, 9, 9, 9, 9, 9, 9, 9]);
    /// ```
    pub fn fill_spare(&mut self, value: T)
    where
        T: Clone,
    {
        if N == 0 || self.size == N {
            return;
        }
        // TODO Optimize
        while self.size < N - 1 {
            self.push_back(value.clone());
        }
        self.push_back(value);
    }

    /// Fills the spare capacity of `self` with elements by calling a closure.
    ///
    /// The elements already present in the buffer (if any) are unaffected.
    ///
    /// This is equivalent adding the result of the closure to the buffer until reaching the
    /// maximum capacity.
    ///
    /// See also: [`fill()`](CircularBuffer::fill), [`fill_with()`](CircularBuffer::fill_with),
    /// [`fill_spare()`](CircularBuffer::fill_spare).
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<10, u32>::from([1, 2, 3]);
    /// assert_eq!(buf, [1, 2, 3]);
    ///
    /// let mut x = 2;
    /// buf.fill_spare_with(|| {
    ///     x *= 2;
    ///     x
    /// });
    /// assert_eq!(buf, [1, 2, 3, 4, 8, 16, 32, 64, 128, 256]);
    /// ```
    pub fn fill_spare_with<F>(&mut self, mut f: F)
    where
        F: FnMut() -> T,
    {
        if N == 0 {
            return;
        }
        // TODO Optimize
        while self.size < N {
            self.push_back(f());
        }
    }

    /// Shortens the buffer, keeping only the front `len` elements and dropping the rest.
    ///
    /// If `len` is equal or greater to the buffer's current length, this has no effect.
    ///
    /// Calling `truncate_back(0)` is equivalent to [`clear()`](Self::clear).
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<4, u32>::from([10, 20, 30]);
    ///
    /// buf.truncate_back(1);
    /// assert_eq!(buf, [10]);
    ///
    /// // Truncating to a length that is greater than the buffer's length has no effect
    /// buf.truncate_back(8);
    /// assert_eq!(buf, [10]);
    /// ```
    pub fn truncate_back(&mut self, len: usize) {
        if N == 0 || len >= self.size {
            // Nothing to do
            return;
        }

        let drop_range = len..self.size;
        // SAFETY: `drop_range` is a valid range, so elements within are guaranteed to be
        // initialized. The `size` of the buffer is shrunk before dropping, so no value will be
        // dropped twice in case of panics.
        unsafe { self.drop_range(drop_range) };
        self.size = len;
    }

    /// Shortens the buffer, keeping only the back `len` elements and dropping the rest.
    ///
    /// If `len` is equal or greater to the buffer's current length, this has no effect.
    ///
    /// Calling `truncate_front(0)` is equivalent to [`clear()`](Self::clear).
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<4, u32>::from([10, 20, 30]);
    ///
    /// buf.truncate_front(1);
    /// assert_eq!(buf, [30]);
    ///
    /// // Truncating to a length that is greater than the buffer's length has no effect
    /// buf.truncate_front(8);
    /// assert_eq!(buf, [30]);
    /// ```
    pub fn truncate_front(&mut self, len: usize) {
        if N == 0 || len >= self.size {
            // Nothing to do
            return;
        }

        let drop_len = self.size - len;
        let drop_range = 0..drop_len;
        // SAFETY: `drop_range` is a valid range, so elements within are guaranteed to be
        // initialized. The `start` of the buffer is shrunk before dropping, so no value will be
        // dropped twice in case of panics.
        unsafe { self.drop_range(drop_range) };
        self.start = add_mod(self.start, drop_len, N);
        self.size = len;
    }

    /// Drops all the elements in the buffer.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf = CircularBuffer::<4, u32>::from([10, 20, 30]);
    /// assert_eq!(buf, [10, 20, 30]);
    /// buf.clear();
    /// assert_eq!(buf, []);
    /// ```
    #[inline]
    pub fn clear(&mut self) {
        self.truncate_back(0)
    }
}

impl<const N: usize, T> CircularBuffer<N, T>
where
    T: Clone,
{
    /// Clones and appends all the elements from the slice to the back of the buffer.
    ///
    /// This is an optimized version of [`extend()`](CircularBuffer::extend) for slices.
    ///
    /// If slice contains more values than the available capacity, the elements at the front of the
    /// buffer are dropped.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let mut buf: CircularBuffer<5, u32> = CircularBuffer::from([1, 2, 3]);
    /// buf.extend_from_slice(&[4, 5, 6, 7]);
    /// assert_eq!(buf, [3, 4, 5, 6, 7]);
    /// ```
    pub fn extend_from_slice(&mut self, other: &[T]) {
        if N == 0 {
            return;
        }

        debug_assert!(self.start < N, "start out-of-bounds");
        debug_assert!(self.size <= N, "size out-of-bounds");

        #[cfg(not(feature = "unstable"))]
        fn write_uninit_slice_cloned<T: Clone>(dst: &mut [MaybeUninit<T>], src: &[T]) {
            // Each call to `clone()` may panic, therefore we need to track how many elements we
            // successfully cloned so that we can drop them in case of panic. This `Guard` struct
            // does exactly that: it keeps track of how many items have been successfully cloned
            // and drops them if the guard is dropped.
            //
            // This implementation was highly inspired by the implementation of
            // `MaybeUninit::clone_from_slice`
            struct Guard<'a, T> {
                dst: &'a mut [MaybeUninit<T>],
                initialized: usize,
            }

            impl<T> Drop for Guard<'_, T> {
                fn drop(&mut self) {
                    let initialized = &mut self.dst[..self.initialized];
                    // SAFETY: this slice contain only initialized objects; `MaybeUninit<T>` has
                    // the same alignment and size as `T`
                    unsafe {
                        let initialized =
                            &mut *(initialized as *mut [MaybeUninit<T>] as *mut [T]);
                        ptr::drop_in_place(initialized);
                    }
                }
            }

            debug_assert_eq!(dst.len(), src.len());
            let len = dst.len();
            let mut guard = Guard {
                dst,
                initialized: 0,
            };
            #[allow(clippy::needless_range_loop)]
            for i in 0..len {
                guard.dst[i].write(src[i].clone());
                guard.initialized += 1;
            }

            // All the `clone()` calls succeded; get rid of the guard without running its `drop()`
            // implementation
            mem::forget(guard);
        }

        if other.len() < N {
            // All the elements of `other` fit into the buffer
            let free_size = N - self.size;
            let final_size = if other.len() < free_size {
                // All the elements of `other` fit at the back of the buffer
                self.size + other.len()
            } else {
                // Some of the elements of `other` need to overwrite the front of the buffer
                self.truncate_front(N - other.len());
                N
            };

            let (right, left) = self.slices_uninit_mut();

            let write_len = core::cmp::min(right.len(), other.len());
            #[cfg(feature = "unstable")]
            right[..write_len].write_clone_of_slice(&other[..write_len]);
            #[cfg(not(feature = "unstable"))]
            write_uninit_slice_cloned(&mut right[..write_len], &other[..write_len]);

            let other = &other[write_len..];
            debug_assert!(left.len() >= other.len());
            let write_len = other.len();
            #[cfg(feature = "unstable")]
            left[..write_len].write_clone_of_slice(other);
            #[cfg(not(feature = "unstable"))]
            write_uninit_slice_cloned(&mut left[..write_len], other);

            self.size = final_size;
        } else {
            // `other` overwrites the whole buffer; get only the last `N` elements from `other` and
            // overwrite
            self.clear();
            self.start = 0;

            let other = &other[other.len() - N..];
            debug_assert_eq!(self.items.len(), other.len());
            #[cfg(feature = "unstable")]
            self.items.write_clone_of_slice(other);
            #[cfg(not(feature = "unstable"))]
            write_uninit_slice_cloned(&mut self.items, other);

            self.size = N;
        }
    }

    /// Clones the elements of the buffer into a new [`Vec`], leaving the buffer unchanged.
    ///
    /// # Examples
    ///
    /// ```
    /// use circular_buffer::CircularBuffer;
    ///
    /// let buf: CircularBuffer<5, u32> = CircularBuffer::from([1, 2, 3]);
    /// let vec: Vec<u32> = buf.to_vec();
    ///
    /// assert_eq!(buf, [1, 2, 3]);
    /// assert_eq!(vec, [1, 2, 3]);
    /// ```
    #[must_use]
    #[cfg(feature = "alloc")]
    pub fn to_vec(&self) -> Vec<T> {
        let mut vec = Vec::with_capacity(self.size);
        vec.extend(self.iter().cloned());
        debug_assert_eq!(vec.len(), self.size);
        vec
    }
}

impl<const N: usize, T> Default for CircularBuffer<N, T> {
    #[inline]
    fn default() -> Self {
        Self::new()
    }
}

impl<const N: usize, const M: usize, T> From<[T; M]> for CircularBuffer<N, T> {
    fn from(mut arr: [T; M]) -> Self {
        #[cfg(feature = "unstable")]
        let mut elems = [const { MaybeUninit::uninit() }; N];
        #[cfg(not(feature = "unstable"))]
        let mut elems = unsafe { MaybeUninit::<[MaybeUninit<T>; N]>::uninit().assume_init() };
        let arr_ptr = &arr as *const T as *const MaybeUninit<T>;
        let elems_ptr = &mut elems as *mut MaybeUninit<T>;
        let size = if N >= M { M } else { N };

        // SAFETY:
        // - `M - size` is non-negative, and `arr_ptr.add(M - size)` points to a memory location
        //   that contains exactly `size` elements
        // - `elems_ptr` points to a memory location that contains exactly `N` elements, and `N` is
        //   greater than or equal to `size`
        unsafe {
            ptr::copy_nonoverlapping(arr_ptr.add(M - size), elems_ptr, size);
        }

        // Prevent destructors from running on those elements that we've taken ownership of; only
        // destroy the elements that were discareded
        //
        // SAFETY: All elements in `arr` are initialized; `forget` will make sure that destructors
        // are not run twice
        unsafe {
            ptr::drop_in_place(&mut arr[..M - size]);
        }
        mem::forget(arr);

        Self {
            size,
            start: 0,
            items: elems,
        }
    }
}

impl<const N: usize, T> FromIterator<T> for CircularBuffer<N, T> {
    fn from_iter<I>(iter: I) -> Self
    where
        I: IntoIterator<Item = T>,
    {
        // TODO Optimize
        let mut buf = Self::new();
        iter.into_iter().for_each(|item| {
            buf.push_back(item);
        });
        buf
    }
}

impl<const N: usize, T> Extend<T> for CircularBuffer<N, T> {
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = T>,
    {
        // TODO Optimize
        iter.into_iter().for_each(|item| {
            self.push_back(item);
        });
    }
}

impl<'a, const N: usize, T> Extend<&'a T> for CircularBuffer<N, T>
where
    T: Copy,
{
    fn extend<I>(&mut self, iter: I)
    where
        I: IntoIterator<Item = &'a T>,
    {
        // TODO Optimize
        iter.into_iter().for_each(|item| {
            self.push_back(*item);
        });
    }
}

impl<const N: usize, T> Index<usize> for CircularBuffer<N, T> {
    type Output = T;

    #[inline]
    fn index(&self, index: usize) -> &Self::Output {
        self.get(index).expect("index out-of-bounds")
    }
}

impl<const N: usize, T> IndexMut<usize> for CircularBuffer<N, T> {
    #[inline]
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        self.get_mut(index).expect("index out-of-bounds")
    }
}

impl<const N: usize, T> IntoIterator for CircularBuffer<N, T> {
    type Item = T;
    type IntoIter = IntoIter<N, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        IntoIter::new(self)
    }
}

impl<'a, const N: usize, T> IntoIterator for &'a CircularBuffer<N, T> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    #[inline]
    fn into_iter(self) -> Self::IntoIter {
        Iter::new(self)
    }
}

impl<const N: usize, const M: usize, T, U> PartialEq<CircularBuffer<M, U>> for CircularBuffer<N, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &CircularBuffer<M, U>) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let (a_left, a_right) = self.as_slices();
        let (b_left, b_right) = other.as_slices();

        match a_left.len().cmp(&b_left.len()) {
            Ordering::Less => {
                let x = a_left.len();
                let y = b_left.len() - x;
                a_left[..] == b_left[..x]
                    && a_right[..y] == b_left[x..]
                    && a_right[y..] == b_right[..]
            }
            Ordering::Greater => {
                let x = b_left.len();
                let y = a_left.len() - x;
                a_left[..x] == b_left[..]
                    && a_left[x..] == b_right[..y]
                    && a_right[..] == b_right[y..]
            }
            Ordering::Equal => {
                debug_assert_eq!(a_left.len(), b_left.len());
                debug_assert_eq!(a_right.len(), b_right.len());
                a_left == b_left && a_right == b_right
            }
        }
    }
}

impl<const N: usize, T> Eq for CircularBuffer<N, T> where T: Eq {}

impl<const N: usize, T, U> PartialEq<[U]> for CircularBuffer<N, T>
where
    T: PartialEq<U>,
{
    fn eq(&self, other: &[U]) -> bool {
        if self.len() != other.len() {
            return false;
        }

        let (a_left, a_right) = self.as_slices();
        let (b_left, b_right) = other.split_at(a_left.len());

        debug_assert_eq!(a_left.len(), b_left.len());
        debug_assert_eq!(a_right.len(), b_right.len());
        a_left == b_left && a_right == b_right
    }
}

impl<const N: usize, const M: usize, T, U> PartialEq<[U; M]> for CircularBuffer<N, T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &[U; M]) -> bool {
        self == &other[..]
    }
}

impl<'a, const N: usize, T, U> PartialEq<&'a [U]> for CircularBuffer<N, T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &&'a [U]) -> bool {
        self == *other
    }
}

impl<'a, const N: usize, T, U> PartialEq<&'a mut [U]> for CircularBuffer<N, T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &&'a mut [U]) -> bool {
        self == *other
    }
}

impl<'a, const N: usize, const M: usize, T, U> PartialEq<&'a [U; M]> for CircularBuffer<N, T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &&'a [U; M]) -> bool {
        self == *other
    }
}

impl<'a, const N: usize, const M: usize, T, U> PartialEq<&'a mut [U; M]> for CircularBuffer<N, T>
where
    T: PartialEq<U>,
{
    #[inline]
    fn eq(&self, other: &&'a mut [U; M]) -> bool {
        self == *other
    }
}

impl<const N: usize, const M: usize, T, U> PartialOrd<CircularBuffer<M, U>> for CircularBuffer<N, T>
where
    T: PartialOrd<U>,
{
    fn partial_cmp(&self, other: &CircularBuffer<M, U>) -> Option<Ordering> {
        self.iter().partial_cmp(other.iter())
    }
}

impl<const N: usize, T> Ord for CircularBuffer<N, T>
where
    T: Ord,
{
    fn cmp(&self, other: &Self) -> Ordering {
        self.iter().cmp(other.iter())
    }
}

impl<const N: usize, T> Hash for CircularBuffer<N, T>
where
    T: Hash,
{
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.size.hash(state);
        self.iter().for_each(|item| item.hash(state));
    }
}

impl<const N: usize, T> Clone for CircularBuffer<N, T>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        // TODO Optimize
        Self::from_iter(self.iter().cloned())
    }

    fn clone_from(&mut self, other: &Self) {
        // TODO Optimize
        self.clear();
        self.extend(other.iter().cloned());
    }
}

impl<const N: usize, T> Drop for CircularBuffer<N, T> {
    #[inline]
    fn drop(&mut self) {
        // `clear()` will make sure that every element is dropped in a safe way
        self.clear();
    }
}

impl<const N: usize, T> fmt::Debug for CircularBuffer<N, T>
where
    T: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}
