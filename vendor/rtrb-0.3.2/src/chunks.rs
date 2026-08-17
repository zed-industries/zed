//! Writing and reading multiple items at once into and from a [`RingBuffer`].
//!
//! Multiple items at once can be moved from an iterator into the ring buffer by using
//! [`Producer::write_chunk_uninit()`] followed by [`WriteChunkUninit::fill_from_iter()`].
//! Alternatively, mutable access to the (uninitialized) slots of the chunk can be obtained with
//! [`WriteChunkUninit::as_mut_slices()`], which requires writing some `unsafe` code.
//! To avoid that, [`Producer::write_chunk()`] can be used,
//! which initializes all slots with their [`Default`] value
//! and provides mutable access by means of [`WriteChunk::as_mut_slices()`].
//!
//! Multiple items at once can be moved out of the ring buffer by using
//! [`Consumer::read_chunk()`] and iterating over the returned [`ReadChunk`]
//! (or by explicitly calling [`ReadChunk::into_iter()`]).
//! Immutable access to the slots of the chunk can be obtained with [`ReadChunk::as_slices()`].
//!
//! # Examples
//!
//! This example uses a single thread for simplicity, but in a real application,
//! `producer` and `consumer` would of course live on different threads:
//!
//! ```
//! use rtrb::RingBuffer;
//!
//! let (mut producer, mut consumer) = RingBuffer::new(5);
//!
//! if let Ok(chunk) = producer.write_chunk_uninit(4) {
//!     chunk.fill_from_iter([10, 11, 12]);
//!     // Note that we requested 4 slots but we've only written to 3 of them!
//! } else {
//!     unreachable!();
//! }
//!
//! assert_eq!(producer.slots(), 2);
//! assert_eq!(consumer.slots(), 3);
//!
//! if let Ok(chunk) = consumer.read_chunk(2) {
//!     assert_eq!(chunk.into_iter().collect::<Vec<_>>(), [10, 11]);
//! } else {
//!     unreachable!();
//! }
//!
//! // One element is still in the queue:
//! assert_eq!(consumer.peek(), Ok(&12));
//!
//! let data = vec![20, 21, 22, 23];
//! // NB: write_chunk_uninit() could be used for possibly better performance:
//! if let Ok(mut chunk) = producer.write_chunk(4) {
//!     let (first, second) = chunk.as_mut_slices();
//!     let mid = first.len();
//!     first.copy_from_slice(&data[..mid]);
//!     second.copy_from_slice(&data[mid..]);
//!     chunk.commit_all();
//! } else {
//!     unreachable!();
//! }
//!
//! assert!(producer.is_full());
//! assert_eq!(consumer.slots(), 5);
//!
//! let mut v = Vec::<i32>::with_capacity(5);
//! if let Ok(chunk) = consumer.read_chunk(5) {
//!     let (first, second) = chunk.as_slices();
//!     v.extend(first);
//!     v.extend(second);
//!     chunk.commit_all();
//! } else {
//!     unreachable!();
//! }
//! assert_eq!(v, [12, 20, 21, 22, 23]);
//! assert!(consumer.is_empty());
//! ```
//!
//! The iterator API can be used to move items from one ring buffer to another:
//!
//! ```
//! use rtrb::{Consumer, Producer};
//!
//! fn move_items<T>(src: &mut Consumer<T>, dst: &mut Producer<T>) -> usize {
//!     let n = src.slots().min(dst.slots());
//!     dst.write_chunk_uninit(n).unwrap().fill_from_iter(src.read_chunk(n).unwrap())
//! }
//! ```
//!
//! ## Common Access Patterns
//!
//! The following examples show the [`Producer`] side;
//! similar patterns can of course be used with [`Consumer::read_chunk()`] as well.
//! Furthermore, the examples use [`Producer::write_chunk_uninit()`],
//! along with a bit of `unsafe` code.
//! To avoid this, you can use [`Producer::write_chunk()`] instead,
//! which requires the trait bound `T: Default` and will lead to a small runtime overhead.
//!
//! Copy a whole slice of items into the ring buffer, but only if space permits
//! (if not, the entire input slice is returned as an error):
//!
//! ```
//! use rtrb::{Producer, CopyToUninit};
//!
//! fn push_entire_slice<'a, T>(queue: &mut Producer<T>, slice: &'a [T]) -> Result<(), &'a [T]>
//! where
//!     T: Copy,
//! {
//!     if let Ok(mut chunk) = queue.write_chunk_uninit(slice.len()) {
//!         let (first, second) = chunk.as_mut_slices();
//!         let mid = first.len();
//!         slice[..mid].copy_to_uninit(first);
//!         slice[mid..].copy_to_uninit(second);
//!         // SAFETY: All slots have been initialized
//!         unsafe { chunk.commit_all() };
//!         Ok(())
//!     } else {
//!         Err(slice)
//!     }
//! }
//! ```
//!
//! Copy as many items as possible from a given slice, returning the number of copied items:
//!
//! ```
//! use rtrb::{Producer, CopyToUninit, chunks::ChunkError::TooFewSlots};
//!
//! fn push_partial_slice<T>(queue: &mut Producer<T>, slice: &[T]) -> usize
//! where
//!     T: Copy,
//! {
//!     let mut chunk = match queue.write_chunk_uninit(slice.len()) {
//!         Ok(chunk) => chunk,
//!         // Remaining slots are returned, this will always succeed:
//!         Err(TooFewSlots(n)) => queue.write_chunk_uninit(n).unwrap(),
//!     };
//!     let end = chunk.len();
//!     let (first, second) = chunk.as_mut_slices();
//!     let mid = first.len();
//!     slice[..mid].copy_to_uninit(first);
//!     slice[mid..end].copy_to_uninit(second);
//!     // SAFETY: All slots have been initialized
//!     unsafe { chunk.commit_all() };
//!     end
//! }
//! ```
//!
//! Write as many slots as possible, given an iterator
//! (and return the number of written slots):
//!
//! ```
//! use rtrb::{Producer, chunks::ChunkError::TooFewSlots};
//!
//! fn push_from_iter<T, I>(queue: &mut Producer<T>, iter: I) -> usize
//! where
//!     T: Default,
//!     I: IntoIterator<Item = T>,
//! {
//!     let iter = iter.into_iter();
//!     let n = match iter.size_hint() {
//!         (_, None) => queue.slots(),
//!         (_, Some(n)) => n,
//!     };
//!     let chunk = match queue.write_chunk_uninit(n) {
//!         Ok(chunk) => chunk,
//!         // Remaining slots are returned, this will always succeed:
//!         Err(TooFewSlots(n)) => queue.write_chunk_uninit(n).unwrap(),
//!     };
//!     chunk.fill_from_iter(iter)
//! }
//! ```

use core::fmt;
use core::marker::PhantomData;
use core::mem::MaybeUninit;
use core::sync::atomic::Ordering;

use crate::{Consumer, CopyToUninit, Producer};

// This is used in the documentation.
#[allow(unused_imports)]
use crate::RingBuffer;

impl<T> Producer<T> {
    /// Returns `n` slots (initially containing their [`Default`] value) for writing.
    ///
    /// [`WriteChunk::as_mut_slices()`] provides mutable access to the slots.
    /// After writing to those slots, they explicitly have to be made available
    /// to be read by the [`Consumer`] by calling [`WriteChunk::commit()`]
    /// or [`WriteChunk::commit_all()`].
    ///
    /// For an alternative that does not require the trait bound [`Default`],
    /// see [`Producer::write_chunk_uninit()`].
    ///
    /// If items are supposed to be moved from an iterator into the ring buffer,
    /// [`Producer::write_chunk_uninit()`] followed by [`WriteChunkUninit::fill_from_iter()`]
    /// can be used.
    ///
    /// # Errors
    ///
    /// If not enough slots are available, an error
    /// (containing the number of available slots) is returned.
    /// Use [`Producer::slots()`] to obtain the number of available slots beforehand.
    ///
    /// # Examples
    ///
    /// See the documentation of the [`chunks`](crate::chunks#examples) module.
    pub fn write_chunk(&mut self, n: usize) -> Result<WriteChunk<'_, T>, ChunkError>
    where
        T: Default,
    {
        self.write_chunk_uninit(n).map(WriteChunk::from)
    }

    /// Returns `n` (uninitialized) slots for writing.
    ///
    /// [`WriteChunkUninit::as_mut_slices()`] provides mutable access
    /// to the uninitialized slots.
    /// After writing to those slots, they explicitly have to be made available
    /// to be read by the [`Consumer`] by calling [`WriteChunkUninit::commit()`]
    /// or [`WriteChunkUninit::commit_all()`].
    ///
    /// Alternatively, [`WriteChunkUninit::fill_from_iter()`] can be used
    /// to move items from an iterator into the available slots.
    /// All moved items are automatically made available to be read by the [`Consumer`].
    ///
    /// # Errors
    ///
    /// If not enough slots are available, an error
    /// (containing the number of available slots) is returned.
    /// Use [`Producer::slots()`] to obtain the number of available slots beforehand.
    ///
    /// # Safety
    ///
    /// This function itself is safe, as is [`WriteChunkUninit::fill_from_iter()`].
    /// However, when using [`WriteChunkUninit::as_mut_slices()`],
    /// the user has to make sure that the relevant slots have been initialized
    /// before calling [`WriteChunkUninit::commit()`] or [`WriteChunkUninit::commit_all()`].
    ///
    /// For a safe alternative that provides mutable slices of [`Default`]-initialized slots,
    /// see [`Producer::write_chunk()`].
    pub fn write_chunk_uninit(&mut self, n: usize) -> Result<WriteChunkUninit<'_, T>, ChunkError> {
        let tail = self.cached_tail.get();

        // Check if the queue has *possibly* not enough slots.
        if self.buffer.capacity - self.buffer.distance(self.cached_head.get(), tail) < n {
            // Refresh the head ...
            let head = self.buffer.head.load(Ordering::Acquire);
            self.cached_head.set(head);

            // ... and check if there *really* are not enough slots.
            let slots = self.buffer.capacity - self.buffer.distance(head, tail);
            if slots < n {
                return Err(ChunkError::TooFewSlots(slots));
            }
        }
        let tail = self.buffer.collapse_position(tail);
        let first_len = n.min(self.buffer.capacity - tail);
        Ok(WriteChunkUninit {
            // SAFETY: tail has been updated to a valid position.
            first_ptr: unsafe { self.buffer.data_ptr.add(tail) },
            first_len,
            second_ptr: self.buffer.data_ptr,
            second_len: n - first_len,
            producer: self,
        })
    }
}

impl<T> Consumer<T> {
    /// Returns `n` slots for reading.
    ///
    /// [`ReadChunk::as_slices()`] provides immutable access to the slots.
    /// After reading from those slots, they explicitly have to be made available
    /// to be written again by the [`Producer`] by calling [`ReadChunk::commit()`]
    /// or [`ReadChunk::commit_all()`].
    ///
    /// Alternatively, items can be moved out of the [`ReadChunk`] using iteration
    /// because it implements [`IntoIterator`]
    /// ([`ReadChunk::into_iter()`] can be used to explicitly turn it into an [`Iterator`]).
    /// All moved items are automatically made available to be written again by the [`Producer`].
    ///
    /// # Errors
    ///
    /// If not enough slots are available, an error
    /// (containing the number of available slots) is returned.
    /// Use [`Consumer::slots()`] to obtain the number of available slots beforehand.
    ///
    /// # Examples
    ///
    /// See the documentation of the [`chunks`](crate::chunks#examples) module.
    pub fn read_chunk(&mut self, n: usize) -> Result<ReadChunk<'_, T>, ChunkError> {
        let head = self.cached_head.get();

        // Check if the queue has *possibly* not enough slots.
        if self.buffer.distance(head, self.cached_tail.get()) < n {
            // Refresh the tail ...
            let tail = self.buffer.tail.load(Ordering::Acquire);
            self.cached_tail.set(tail);

            // ... and check if there *really* are not enough slots.
            let slots = self.buffer.distance(head, tail);
            if slots < n {
                return Err(ChunkError::TooFewSlots(slots));
            }
        }

        let head = self.buffer.collapse_position(head);
        let first_len = n.min(self.buffer.capacity - head);
        Ok(ReadChunk {
            // SAFETY: head has been updated to a valid position.
            first_ptr: unsafe { self.buffer.data_ptr.add(head) },
            first_len,
            second_ptr: self.buffer.data_ptr,
            second_len: n - first_len,
            consumer: self,
        })
    }
}

/// Structure for writing into multiple ([`Default`]-initialized) slots in one go.
///
/// This is returned from [`Producer::write_chunk()`].
///
/// To obtain uninitialized slots, use [`Producer::write_chunk_uninit()`] instead,
/// which also allows moving items from an iterator into the ring buffer
/// by means of [`WriteChunkUninit::fill_from_iter()`].
#[derive(Debug, PartialEq, Eq)]
pub struct WriteChunk<'a, T>(Option<WriteChunkUninit<'a, T>>, PhantomData<T>);

impl<T> Drop for WriteChunk<'_, T> {
    fn drop(&mut self) {
        // NB: If `commit()` or `commit_all()` has been called, `self.0` is `None`.
        if let Some(mut chunk) = self.0.take() {
            // No part of the chunk has been committed, all slots are dropped.
            // SAFETY: All slots have been initialized in From::from().
            unsafe { chunk.drop_suffix(0) };
        }
    }
}

impl<'a, T> From<WriteChunkUninit<'a, T>> for WriteChunk<'a, T>
where
    T: Default,
{
    /// Fills all slots with the [`Default`] value.
    fn from(chunk: WriteChunkUninit<'a, T>) -> Self {
        for i in 0..chunk.first_len {
            // SAFETY: i is in a valid range.
            unsafe { chunk.first_ptr.add(i).write(Default::default()) };
        }
        for i in 0..chunk.second_len {
            // SAFETY: i is in a valid range.
            unsafe { chunk.second_ptr.add(i).write(Default::default()) };
        }
        WriteChunk(Some(chunk), PhantomData)
    }
}

impl<T> WriteChunk<'_, T>
where
    T: Default,
{
    /// Returns two slices for writing to the requested slots.
    ///
    /// All slots are initially filled with their [`Default`] value.
    ///
    /// The first slice can only be empty if `0` slots have been requested.
    /// If the first slice contains all requested slots, the second one is empty.
    ///
    /// After writing to the slots, they are *not* automatically made available
    /// to be read by the [`Consumer`].
    /// This has to be explicitly done by calling [`commit()`](WriteChunk::commit)
    /// or [`commit_all()`](WriteChunk::commit_all).
    /// If items are written but *not* committed afterwards,
    /// they will *not* become available for reading and
    /// they will be leaked (which is only relevant if `T` implements [`Drop`]).
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        // self.0 is always Some(chunk).
        let chunk = self.0.as_ref().unwrap();
        // SAFETY: The pointers and lengths have been computed correctly in write_chunk_uninit()
        // and all slots have been initialized in From::from().
        unsafe {
            (
                core::slice::from_raw_parts_mut(chunk.first_ptr, chunk.first_len),
                core::slice::from_raw_parts_mut(chunk.second_ptr, chunk.second_len),
            )
        }
    }

    /// Makes the first `n` slots of the chunk available for reading.
    ///
    /// The rest of the chunk is dropped.
    ///
    /// # Panics
    ///
    /// Panics if `n` is greater than the number of slots in the chunk.
    pub fn commit(mut self, n: usize) {
        // self.0 is always Some(chunk).
        let mut chunk = self.0.take().unwrap();
        // SAFETY: All slots have been initialized in From::from().
        unsafe {
            // Slots at index `n` and higher are dropped ...
            chunk.drop_suffix(n);
            // ... everything below `n` is committed.
            chunk.commit(n);
        }
        // `self` is dropped here, with `self.0` being set to `None`.
    }

    /// Makes the whole chunk available for reading.
    pub fn commit_all(mut self) {
        // self.0 is always Some(chunk).
        let chunk = self.0.take().unwrap();
        // SAFETY: All slots have been initialized in From::from().
        unsafe { chunk.commit_all() };
        // `self` is dropped here, with `self.0` being set to `None`.
    }

    /// Returns the number of slots in the chunk.
    #[must_use]
    pub fn len(&self) -> usize {
        // self.0 is always Some(chunk).
        self.0.as_ref().unwrap().len()
    }

    /// Returns `true` if the chunk contains no slots.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        // self.0 is always Some(chunk).
        self.0.as_ref().unwrap().is_empty()
    }
}

/// Structure for writing into multiple (uninitialized) slots in one go.
///
/// This is returned from [`Producer::write_chunk_uninit()`].
#[derive(Debug, PartialEq, Eq)]
pub struct WriteChunkUninit<'a, T> {
    first_ptr: *mut T,
    first_len: usize,
    second_ptr: *mut T,
    second_len: usize,
    producer: &'a Producer<T>,
}

// SAFETY: WriteChunkUninit only exists while a unique reference to the Producer is held.
// It is therefore safe to move it to another thread.
unsafe impl<T: Send> Send for WriteChunkUninit<'_, T> {}

impl<T> WriteChunkUninit<'_, T> {
    /// Returns two slices for writing to the requested slots.
    ///
    /// The first slice can only be empty if `0` slots have been requested.
    /// If the first slice contains all requested slots, the second one is empty.
    ///
    /// The extension trait [`CopyToUninit`] can be used to safely copy data into those slices.
    ///
    /// After writing to the slots, they are *not* automatically made available
    /// to be read by the [`Consumer`].
    /// This has to be explicitly done by calling [`commit()`](WriteChunkUninit::commit)
    /// or [`commit_all()`](WriteChunkUninit::commit_all).
    /// If items are written but *not* committed afterwards,
    /// they will *not* become available for reading and
    /// they will be leaked (which is only relevant if `T` implements [`Drop`]).
    pub fn as_mut_slices(&mut self) -> (&mut [MaybeUninit<T>], &mut [MaybeUninit<T>]) {
        // SAFETY: The pointers and lengths have been computed correctly in write_chunk_uninit().
        unsafe {
            (
                core::slice::from_raw_parts_mut(self.first_ptr.cast(), self.first_len),
                core::slice::from_raw_parts_mut(self.second_ptr.cast(), self.second_len),
            )
        }
    }

    /// Makes the first `n` slots of the chunk available for reading.
    ///
    /// # Panics
    ///
    /// Panics if `n` is greater than the number of slots in the chunk.
    ///
    /// # Safety
    ///
    /// The caller must make sure that the first `n` elements have been initialized.
    pub unsafe fn commit(self, n: usize) {
        assert!(n <= self.len(), "cannot commit more than chunk size");
        // SAFETY: Delegated to the caller.
        unsafe { self.commit_unchecked(n) };
    }

    /// Makes the whole chunk available for reading.
    ///
    /// # Safety
    ///
    /// The caller must make sure that all elements have been initialized.
    pub unsafe fn commit_all(self) {
        let slots = self.len();
        // SAFETY: Delegated to the caller.
        unsafe { self.commit_unchecked(slots) };
    }

    unsafe fn commit_unchecked(self, n: usize) -> usize {
        let p = self.producer;
        let tail = p.buffer.increment(p.cached_tail.get(), n);
        p.buffer.tail.store(tail, Ordering::Release);
        p.cached_tail.set(tail);
        n
    }

    /// Moves items from an iterator into the (uninitialized) slots of the chunk.
    ///
    /// The number of moved items is returned.
    ///
    /// All moved items are automatically made availabe to be read by the [`Consumer`].
    ///
    /// # Examples
    ///
    /// If the iterator contains too few items, only a part of the chunk
    /// is made available for reading:
    ///
    /// ```
    /// use rtrb::{RingBuffer, PopError};
    ///
    /// let (mut p, mut c) = RingBuffer::new(4);
    ///
    /// if let Ok(chunk) = p.write_chunk_uninit(3) {
    ///     assert_eq!(chunk.fill_from_iter([10, 20]), 2);
    /// } else {
    ///     unreachable!();
    /// }
    /// assert_eq!(p.slots(), 2);
    /// assert_eq!(c.pop(), Ok(10));
    /// assert_eq!(c.pop(), Ok(20));
    /// assert_eq!(c.pop(), Err(PopError::Empty));
    /// ```
    ///
    /// If the chunk size is too small, some items may remain in the iterator.
    /// To be able to keep using the iterator after the call,
    /// `&mut` (or [`Iterator::by_ref()`]) can be used.
    ///
    /// ```
    /// use rtrb::{RingBuffer, PopError};
    ///
    /// let (mut p, mut c) = RingBuffer::new(4);
    ///
    /// let mut it = vec![10, 20, 30].into_iter();
    /// if let Ok(chunk) = p.write_chunk_uninit(2) {
    ///     assert_eq!(chunk.fill_from_iter(&mut it), 2);
    /// } else {
    ///     unreachable!();
    /// }
    /// assert_eq!(c.pop(), Ok(10));
    /// assert_eq!(c.pop(), Ok(20));
    /// assert_eq!(c.pop(), Err(PopError::Empty));
    /// assert_eq!(it.next(), Some(30));
    /// ```
    pub fn fill_from_iter<I>(self, iter: I) -> usize
    where
        I: IntoIterator<Item = T>,
    {
        let mut iter = iter.into_iter();
        let mut iterated = 0;
        'outer: for &(ptr, len) in &[
            (self.first_ptr, self.first_len),
            (self.second_ptr, self.second_len),
        ] {
            for i in 0..len {
                match iter.next() {
                    Some(item) => {
                        // SAFETY: It is allowed to write to this memory slot
                        unsafe { ptr.add(i).write(item) };
                        iterated += 1;
                    }
                    None => break 'outer,
                }
            }
        }
        // SAFETY: iterated slots have been initialized above
        unsafe { self.commit_unchecked(iterated) }
    }

    /// Returns the number of slots in the chunk.
    #[must_use]
    pub fn len(&self) -> usize {
        self.first_len + self.second_len
    }

    /// Returns `true` if the chunk contains no slots.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.first_len == 0
    }

    /// Drops all elements starting from index `n`.
    ///
    /// All of those slots must be initialized.
    unsafe fn drop_suffix(&mut self, n: usize) {
        // NB: If n >= self.len(), the loops are not entered.
        for i in n..self.first_len {
            // SAFETY: The caller must make sure that all slots are initialized.
            unsafe { self.first_ptr.add(i).drop_in_place() };
        }
        for i in n.saturating_sub(self.first_len)..self.second_len {
            // SAFETY: The caller must make sure that all slots are initialized.
            unsafe { self.second_ptr.add(i).drop_in_place() };
        }
    }
}

/// Structure for reading from multiple slots in one go.
///
/// This is returned from [`Consumer::read_chunk()`].
#[derive(Debug, PartialEq, Eq)]
pub struct ReadChunk<'a, T> {
    // Must be "mut" for drop_in_place()
    first_ptr: *mut T,
    first_len: usize,
    // Must be "mut" for drop_in_place()
    second_ptr: *mut T,
    second_len: usize,
    consumer: &'a Consumer<T>,
}

// SAFETY: ReadChunk only exists while a unique reference to the Consumer is held.
// It is therefore safe to move it to another thread.
unsafe impl<T: Send> Send for ReadChunk<'_, T> {}

impl<T> ReadChunk<'_, T> {
    /// Returns two slices for reading from the requested slots.
    ///
    /// The first slice can only be empty if `0` slots have been requested.
    /// If the first slice contains all requested slots, the second one is empty.
    ///
    /// The provided slots are *not* automatically made available
    /// to be written again by the [`Producer`].
    /// This has to be explicitly done by calling [`commit()`](ReadChunk::commit)
    /// or [`commit_all()`](ReadChunk::commit_all).
    /// Note that this runs the destructor of the committed items (if `T` implements [`Drop`]).
    /// You can "peek" at the contained values by simply not calling any of the "commit" methods.
    #[must_use]
    pub fn as_slices(&self) -> (&[T], &[T]) {
        // SAFETY: The pointers and lengths have been computed correctly in read_chunk().
        unsafe {
            (
                core::slice::from_raw_parts(self.first_ptr, self.first_len),
                core::slice::from_raw_parts(self.second_ptr, self.second_len),
            )
        }
    }

    /// Returns two mutable slices for reading from the requested slots.
    ///
    /// This has the same semantics as [`as_slices()`](ReadChunk::as_slices),
    /// except that it returns mutable slices and requires a mutable reference
    /// to the chunk.
    ///
    /// In the vast majority of cases, mutable access is not required when
    /// reading data and the immutable version should be preferred. However,
    /// there are some scenarios where it might be desirable to perform
    /// operations on the data in-place without copying it to a separate buffer
    /// (e.g. streaming decryption), in which case this version can be used.
    #[must_use]
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        // SAFETY: The pointers and lengths have been computed correctly in read_chunk().
        unsafe {
            (
                core::slice::from_raw_parts_mut(self.first_ptr, self.first_len),
                core::slice::from_raw_parts_mut(self.second_ptr, self.second_len),
            )
        }
    }

    /// Drops the first `n` slots of the chunk, making the space available for writing again.
    ///
    /// # Panics
    ///
    /// Panics if `n` is greater than the number of slots in the chunk.
    ///
    /// # Examples
    ///
    /// The following example shows that items are dropped when "committed"
    /// (which is only relevant if `T` implements [`Drop`]).
    ///
    /// ```
    /// use rtrb::RingBuffer;
    ///
    /// // Static variable to count all drop() invocations
    /// static mut DROP_COUNT: i32 = 0;
    /// #[derive(Debug)]
    /// struct Thing;
    /// impl Drop for Thing {
    ///     fn drop(&mut self) { unsafe { DROP_COUNT += 1; } }
    /// }
    ///
    /// // Scope to limit lifetime of ring buffer
    /// {
    ///     let (mut p, mut c) = RingBuffer::new(2);
    ///
    ///     assert!(p.push(Thing).is_ok()); // 1
    ///     assert!(p.push(Thing).is_ok()); // 2
    ///     if let Ok(thing) = c.pop() {
    ///         // "thing" has been *moved* out of the queue but not yet dropped
    ///         assert_eq!(unsafe { DROP_COUNT }, 0);
    ///     } else {
    ///         unreachable!();
    ///     }
    ///     // First Thing has been dropped when "thing" went out of scope:
    ///     assert_eq!(unsafe { DROP_COUNT }, 1);
    ///     assert!(p.push(Thing).is_ok()); // 3
    ///
    ///     if let Ok(chunk) = c.read_chunk(2) {
    ///         assert_eq!(chunk.len(), 2);
    ///         assert_eq!(unsafe { DROP_COUNT }, 1);
    ///         chunk.commit(1); // Drops only one of the two Things
    ///         assert_eq!(unsafe { DROP_COUNT }, 2);
    ///     } else {
    ///         unreachable!();
    ///     }
    ///     // The last Thing is still in the queue ...
    ///     assert_eq!(unsafe { DROP_COUNT }, 2);
    /// }
    /// // ... and it is dropped when the ring buffer goes out of scope:
    /// assert_eq!(unsafe { DROP_COUNT }, 3);
    /// ```
    pub fn commit(self, n: usize) {
        assert!(n <= self.len(), "cannot commit more than chunk size");
        // SAFETY: self.len() initialized elements have been obtained in read_chunk().
        unsafe { self.commit_unchecked(n) };
    }

    /// Drops all slots of the chunk, making the space available for writing again.
    pub fn commit_all(self) {
        let slots = self.len();
        // SAFETY: self.len() initialized elements have been obtained in read_chunk().
        unsafe { self.commit_unchecked(slots) };
    }

    unsafe fn commit_unchecked(self, n: usize) -> usize {
        let first_len = self.first_len.min(n);
        for i in 0..first_len {
            // SAFETY: The caller must make sure that there are n initialized elements.
            unsafe { self.first_ptr.add(i).drop_in_place() };
        }
        let second_len = self.second_len.min(n - first_len);
        for i in 0..second_len {
            // SAFETY: The caller must make sure that there are n initialized elements.
            unsafe { self.second_ptr.add(i).drop_in_place() };
        }
        let c = self.consumer;
        let head = c.buffer.increment(c.cached_head.get(), n);
        c.buffer.head.store(head, Ordering::Release);
        c.cached_head.set(head);
        n
    }

    /// Returns the number of slots in the chunk.
    #[must_use]
    pub fn len(&self) -> usize {
        self.first_len + self.second_len
    }

    /// Returns `true` if the chunk contains no slots.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.first_len == 0
    }
}

impl<'a, T> IntoIterator for ReadChunk<'a, T> {
    type Item = T;
    type IntoIter = ReadChunkIntoIter<'a, T>;

    /// Turns a [`ReadChunk`] into an iterator.
    ///
    /// When the iterator is dropped, all iterated slots are made available for writing again.
    /// Non-iterated items remain in the ring buffer.
    fn into_iter(self) -> Self::IntoIter {
        Self::IntoIter {
            chunk: self,
            iterated: 0,
        }
    }
}

/// An iterator that moves out of a [`ReadChunk`].
///
/// This `struct` is created by the [`into_iter()`](ReadChunk::into_iter) method
/// on [`ReadChunk`] (provided by the [`IntoIterator`] trait).
///
/// When this `struct` is dropped, the iterated slots are made available for writing again.
/// Non-iterated items remain in the ring buffer.
#[derive(Debug)]
pub struct ReadChunkIntoIter<'a, T> {
    chunk: ReadChunk<'a, T>,
    iterated: usize,
}

impl<T> Drop for ReadChunkIntoIter<'_, T> {
    /// Makes all iterated slots available for writing again.
    ///
    /// Non-iterated items remain in the ring buffer and are *not* dropped.
    fn drop(&mut self) {
        let c = &self.chunk.consumer;
        let head = c.buffer.increment(c.cached_head.get(), self.iterated);
        c.buffer.head.store(head, Ordering::Release);
        c.cached_head.set(head);
    }
}

impl<T> Iterator for ReadChunkIntoIter<'_, T> {
    type Item = T;

    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        let ptr = if self.iterated < self.chunk.first_len {
            // SAFETY: first_len is valid.
            unsafe { self.chunk.first_ptr.add(self.iterated) }
        } else if self.iterated < self.chunk.first_len + self.chunk.second_len {
            // SAFETY: first_len and second_len are valid.
            unsafe {
                self.chunk
                    .second_ptr
                    .add(self.iterated - self.chunk.first_len)
            }
        } else {
            return None;
        };
        self.iterated += 1;
        // SAFETY: ptr points to an initialized slot.
        Some(unsafe { ptr.read() })
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.chunk.first_len + self.chunk.second_len - self.iterated;
        (remaining, Some(remaining))
    }
}

impl<T> ExactSizeIterator for ReadChunkIntoIter<'_, T> {}

impl<T> core::iter::FusedIterator for ReadChunkIntoIter<'_, T> {}

#[cfg(feature = "std")]
impl std::io::Write for Producer<u8> {
    #[inline]
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        use ChunkError::TooFewSlots;
        let mut chunk = match self.write_chunk_uninit(buf.len()) {
            Ok(chunk) => chunk,
            Err(TooFewSlots(0)) => return Err(std::io::ErrorKind::WouldBlock.into()),
            Err(TooFewSlots(n)) => self.write_chunk_uninit(n).unwrap(),
        };
        let end = chunk.len();
        let (first, second) = chunk.as_mut_slices();
        let mid = first.len();
        // NB: If buf.is_empty(), chunk will be empty as well and the following are no-ops:
        buf[..mid].copy_to_uninit(first);
        buf[mid..end].copy_to_uninit(second);
        // SAFETY: All slots have been initialized
        unsafe { chunk.commit_all() };
        Ok(end)
    }

    #[inline]
    fn flush(&mut self) -> std::io::Result<()> {
        // Nothing to do here.
        Ok(())
    }
}

#[cfg(feature = "std")]
impl std::io::Read for Consumer<u8> {
    #[inline]
    fn read(&mut self, buf: &mut [u8]) -> std::io::Result<usize> {
        use ChunkError::TooFewSlots;
        let chunk = match self.read_chunk(buf.len()) {
            Ok(chunk) => chunk,
            Err(TooFewSlots(0)) => return Err(std::io::ErrorKind::WouldBlock.into()),
            Err(TooFewSlots(n)) => self.read_chunk(n).unwrap(),
        };
        let (first, second) = chunk.as_slices();
        let mid = first.len();
        let end = chunk.len();
        // NB: If buf.is_empty(), chunk will be empty as well and the following are no-ops:
        buf[..mid].copy_from_slice(first);
        buf[mid..end].copy_from_slice(second);
        chunk.commit_all();
        Ok(end)
    }
}

/// Error type for [`Consumer::read_chunk()`], [`Producer::write_chunk()`]
/// and [`Producer::write_chunk_uninit()`].
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ChunkError {
    /// Fewer than the requested number of slots were available.
    ///
    /// Contains the number of slots that were available.
    TooFewSlots(usize),
}

#[cfg(feature = "std")]
impl std::error::Error for ChunkError {}

impl fmt::Display for ChunkError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            ChunkError::TooFewSlots(n) => {
                alloc::format!("only {} slots available in ring buffer", n).fmt(f)
            }
        }
    }
}
