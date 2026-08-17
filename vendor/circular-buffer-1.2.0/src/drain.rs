// Copyright © 2023-2025 Andrea Corbellini and contributors
// SPDX-License-Identifier: BSD-3-Clause

use crate::add_mod;
use crate::iter::translate_range_bounds;
use crate::iter::Iter;
use crate::CircularBuffer;
use core::fmt;
use core::iter::FusedIterator;
use core::marker::PhantomData;
#[cfg(not(feature = "unstable"))]
use core::mem::MaybeUninit;
use core::ops::Range;
use core::ops::RangeBounds;
use core::ptr;
use core::ptr::NonNull;

/// A draining [iterator](core::iter::Iterator) that removes and returns elements from a
/// `CircularBuffer`.
///
/// This struct is created by [`CircularBuffer::drain()`]. See its documentation for more details.
pub struct Drain<'a, const N: usize, T> {
    /// This is a pointer and not a reference (`&'a mut CircularBuffer`) because using a reference
    /// would make `Drain` an invariant over `CircularBuffer`, but instead we want `Drain` to be
    /// covariant over `CircularBuffer`.
    ///
    /// The reason why `Drain` needs to be covariant is that, semantically,
    /// `CircularBuffer::drain()` should be equivalent to popping all the drained elements from the
    /// buffer, storing them into a vector, and returning an iterable over the vector.
    /// Equivalently, `Drain` owns the drained elements, so it would be unnecessarily restrictive
    /// to make this type invariant over `CircularBuffer`.
    buf: NonNull<CircularBuffer<N, T>>,
    /// A backup of the size of the buffer. Necessary because `buf.size` is set to 0 during the
    /// lifetime of the `Drain` and is restored only during drop.
    buf_size: usize,
    /// The range that was requested to drain. Necessary to properly rearrange the buffer memory
    /// during drop.
    range: Range<usize>,
    /// An iterator over the indexes of the elements to return from the draining iterator.
    /// Initially, `range` and `iter` are set to the same `Range`, but as the draining iterator is
    /// used (via `Iterator::next()`, or similar), `iter` is mutated, while `range` is preserved.
    iter: Range<usize>,
    /// Necessary to bind the lifetime of `CircularBuffer` to `Drain`. Note that this is an `&`
    /// reference, and not a mutable `&mut` reference: this is to make `Drain` covariant over
    /// `CircularBuffer`.
    phantom: PhantomData<&'a T>,
}

impl<'a, const N: usize, T> Drain<'a, N, T> {
    pub(crate) fn over_range<R>(buf: &'a mut CircularBuffer<N, T>, range: R) -> Self
    where
        R: RangeBounds<usize>,
    {
        let (start, end) = translate_range_bounds(buf, range);

        // Iterating over a `Drain` returns items from the buffer, but does actually remove the
        // item from the buffer right away. Because of that, forgetting a `Drain` (via
        // `mem::forget`) can potentially leave the `CircularBuffer` in an unsafe state: the same
        // item may have been returned from the `Drain` iterator, and be part of the
        // `CircularBuffer` at the same time, which would be unsafe for non-`Copy` types.
        //
        // To avoid getting into this unsafe state, the size of the buffer is set to 0 while the
        // `Drain` is alive, and it's restored when the `Drain` is dropped. Forgetting a `Drain`
        // will therefore forget all the items in the buffer (even the ones that were not drained).
        // This ensures maximum safety while keeping the implementation simple and performant
        // enough.
        let buf_size = buf.size;
        buf.size = 0;

        let buf = NonNull::from(buf);

        Self {
            buf,
            buf_size,
            range: start..end,
            iter: start..end,
            phantom: PhantomData,
        }
    }

    /// Reads an element from the `CircularBuffer`.
    ///
    /// # Safety
    ///
    /// The `index` must point to an initialized element of the buffer. After this method is used,
    /// the element at `index` must be considered as uninitialized memory and therefore the `index`
    /// must not be reused.
    unsafe fn read(&self, index: usize) -> T {
        debug_assert!(
            index < N && index < self.buf_size,
            "index out-of-bounds for buffer"
        );
        debug_assert!(
            index >= self.range.start && index < self.range.end,
            "index out-of-bounds for drain range"
        );
        debug_assert!(
            index < self.iter.start || index >= self.iter.end,
            "attempt to read an item that may be returned by the iterator"
        );
        let buf = unsafe { self.buf.as_ref() };
        let index = add_mod(buf.start, index, N);
        unsafe { ptr::read(buf.items[index].assume_init_ref()) }
    }

    fn as_slices(&self) -> (&[T], &[T]) {
        if N == 0 || self.buf_size == 0 || self.iter.is_empty() {
            return (&[][..], &[][..]);
        }

        let buf = unsafe { self.buf.as_ref() };

        debug_assert!(buf.start < N, "start out-of-bounds");
        debug_assert!(self.buf_size <= N, "size out-of-bounds");

        let start = add_mod(buf.start, self.iter.start, N);
        let end = add_mod(buf.start, self.iter.end, N);

        let (right, left) = if start < end {
            (&buf.items[start..end], &[][..])
        } else {
            let (left, right) = buf.items.split_at(end);
            let right = &right[start - end..];
            (right, left)
        };

        // SAFETY: The elements in these slices are guaranteed to be initialized
        #[cfg(feature = "unstable")]
        unsafe {
            (right.assume_init_ref(), left.assume_init_ref())
        }
        #[cfg(not(feature = "unstable"))]
        unsafe {
            (
                &*(right as *const [MaybeUninit<T>] as *const [T]),
                &*(left as *const [MaybeUninit<T>] as *const [T]),
            )
        }
    }

    fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        if N == 0 || self.buf_size == 0 || self.iter.is_empty() {
            return (&mut [][..], &mut [][..]);
        }

        let buf = unsafe { self.buf.as_mut() };

        debug_assert!(buf.start < N, "start out-of-bounds");
        debug_assert!(self.buf_size <= N, "size out-of-bounds");

        let start = add_mod(buf.start, self.iter.start, N);
        let end = add_mod(buf.start, self.iter.end, N);

        let (right, left) = if start < end {
            (&mut buf.items[start..end], &mut [][..])
        } else {
            let (left, right) = buf.items.split_at_mut(end);
            let right = &mut right[start - end..];
            (right, left)
        };

        // SAFETY: The elements in these slices are guaranteed to be initialized
        #[cfg(feature = "unstable")]
        unsafe {
            (right.assume_init_mut(), left.assume_init_mut())
        }
        #[cfg(not(feature = "unstable"))]
        unsafe {
            (
                &mut *(right as *mut [MaybeUninit<T>] as *mut [T]),
                &mut *(left as *mut [MaybeUninit<T>] as *mut [T]),
            )
        }
    }
}

impl<const N: usize, T> Iterator for Drain<'_, N, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        // SAFETY: the element at the index is guaranteed to be initialized
        self.iter.next().map(|index| unsafe { self.read(index) })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.iter.size_hint()
    }
}

impl<const N: usize, T> ExactSizeIterator for Drain<'_, N, T> {
    #[inline]
    fn len(&self) -> usize {
        self.iter.len()
    }
}

impl<const N: usize, T> FusedIterator for Drain<'_, N, T> {}

impl<const N: usize, T> DoubleEndedIterator for Drain<'_, N, T> {
    fn next_back(&mut self) -> Option<Self::Item> {
        // SAFETY: the element at the index is guaranteed to be initialized
        self.iter
            .next_back()
            .map(|index| unsafe { self.read(index) })
    }
}

impl<const N: usize, T> Drop for Drain<'_, N, T> {
    fn drop(&mut self) {
        // Drop the items that were not consumed
        struct Dropper<'a, T>(&'a mut [T]);

        impl<T> Drop for Dropper<'_, T> {
            fn drop(&mut self) {
                // SAFETY: the slice is guaranteed to be valid for read and writes as the `Drain`
                // holds a mutable reference to the `CircularBuffer` that contains the data
                // referenced by the slices.
                unsafe {
                    ptr::drop_in_place(self.0);
                }
            }
        }

        let (right, left) = self.as_mut_slices();

        let right = Dropper(right);
        let left = Dropper(left);

        drop(right);
        drop(left);

        // The drain has left a "hole" of items in the `CircularBuffer` that either got moved out
        // during iteration, or got dropped earlier. There are 3 possible scenarios for the state
        // of the `CircularBuffer` at this point:
        //
        // 1. The "hole" is at the front of the buffer:
        //    | hole | remaining items |
        //
        // 2. The "hole" is at the back of the buffer:
        //    | remaining items | hole |
        //
        // 3. The "hole" is in the middle of the buffer:
        //    | remaining items | hole | remaining items |
        //
        // Scenario #1 and #2 can be handled by adjusting the start offset and length of the
        // buffer. Scenario #3 requires moving the remaining items into the "hole" to fill the gap.
        //
        // Filling the hole for scenario #3 requires at most a 3-steps. The worst case looks like
        // this:
        //
        //     | back items [part 2/2] | front items | hole | back items [part 1/2] |
        //                             ^
        //                             ` start
        //
        // The first step to do is to move `back items [part 1/2]` into `hole`, so that the
        // `CircularBuffer` looks like this:
        //
        //     | back items [part 2/2] | front items | back items [part 1/2] | hole |
        //                             ^
        //                             ` start
        //
        // Then a portion of `back items [part 2/2]` can be copied into the new `hole`. Note that
        // `back items [part 2/2]` may not fit into `hole`, and so it may be necessary to split it
        // in two chunks:
        //
        //     | hole | back items [part 3/3] | front items | back items [part 1/3] | back items [part 2/3] |
        //                                    ^
        //                                    ` start
        //
        // Finally the last chunk `back items [part 3/3]` can be moved into that `hole`:
        //
        //     | back items [part 3/3] | hole | front items | back items [part 1/3] | back items [part 2/3] |
        //                                    ^
        //                                    ` start
        //
        // A similar strategy could be applied to move the front items into the hole instead of the
        // back items. Ideally the implementation should decide whether to move the front items or
        // the back items depending on which one results in fewer data to be moved; however for now
        // only the implementation always moves the back items.

        // TODO: optimize for the case where the hole is in the front or the back
        // TODO: optimize for the case where there are fewer items to move from the front

        // SAFETY: `buf` is a valid pointer because `Drain` holds a mutable reference to it.
        let buf = unsafe { self.buf.as_mut() };
        let mut remaining = self.buf_size - self.range.end;

        let items = CircularSlicePtr::new(&mut buf.items).add(buf.start);
        let mut hole = items.add(self.range.start);
        let mut backfill = items.add(self.range.end);

        // This loop should run at most 3 times as explained above
        while remaining > 0 {
            let copy_len = hole
                .available_len()
                .min(backfill.available_len())
                .min(remaining);
            // SAFETY: both pointers are properly aligned, and are valid for read and writes.
            unsafe { ptr::copy(backfill.as_ptr(), hole.as_mut_ptr(), copy_len) };

            hole = hole.add(copy_len);
            backfill = backfill.add(copy_len);
            remaining -= copy_len;
        }

        // Now that the buffer memory contains valid items, the size can be restored
        buf.size = self.buf_size - self.range.len();
    }
}

impl<const N: usize, T> fmt::Debug for Drain<'_, N, T>
where
    T: fmt::Debug,
{
    #[inline]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (right, left) = self.as_slices();
        let it = Iter { right, left };
        it.fmt(f)
    }
}

#[derive(Debug)]
struct CircularSlicePtr<'a, T> {
    slice_start: *mut T,
    slice_len: usize,
    offset: usize,
    phantom: PhantomData<&'a T>,
}

impl<'a, T> CircularSlicePtr<'a, T> {
    fn new(slice: &'a mut [T]) -> Self {
        Self {
            slice_start: slice as *mut [T] as *mut T,
            slice_len: slice.len(),
            offset: 0,
            phantom: PhantomData,
        }
    }

    fn as_ptr(&self) -> *const T {
        debug_assert!(self.offset < self.slice_len);
        // SAFETY: `slice_start` is a valid pointer because it was obtained from a reference that
        // is still alive; `offset` is within the bounds of the slice, so the resulting pointer is
        // also valid.
        unsafe { self.slice_start.add(self.offset) }
    }

    fn as_mut_ptr(&self) -> *mut T {
        debug_assert!(self.offset < self.slice_len);
        // SAFETY: `slice_start` is a valid pointer because it was obtained from a reference that
        // is still alive; `offset` is within the bounds of the slice, so the resulting pointer is
        // also valid.
        unsafe { self.slice_start.add(self.offset) }
    }

    fn available_len(&self) -> usize {
        debug_assert!(self.offset < self.slice_len);
        self.slice_len - self.offset
    }

    fn add(mut self, increment: usize) -> Self {
        debug_assert!(self.offset < self.slice_len);
        debug_assert!(increment <= self.slice_len);
        self.offset = add_mod(self.offset, increment, self.slice_len);
        self
    }
}

// Need to manually implement `Copy` because `#[derive(Copy)]` requires `T` to implement `Copy`.
// Also need to manually implement `Clone` because `Copy` requires `Clone`.

impl<T> Copy for CircularSlicePtr<'_, T> {}

impl<T> Clone for CircularSlicePtr<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}
