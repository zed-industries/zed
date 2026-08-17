//! A fixed capacity double-ended queue.
//!
//! # Examples
//!
//! ```
//! use heapless::Deque;
//!
//! // A deque with a fixed capacity of 8 elements allocated on the stack
//! let mut deque = Deque::<_, 8>::new();
//!
//! // You can use it as a good old FIFO queue.
//! deque.push_back(1);
//! deque.push_back(2);
//! assert_eq!(deque.len(), 2);
//!
//! assert_eq!(deque.pop_front(), Some(1));
//! assert_eq!(deque.pop_front(), Some(2));
//! assert_eq!(deque.len(), 0);
//!
//! // Deque is double-ended, you can push and pop from the front and back.
//! deque.push_back(1);
//! deque.push_front(2);
//! deque.push_back(3);
//! deque.push_front(4);
//! assert_eq!(deque.pop_front(), Some(4));
//! assert_eq!(deque.pop_front(), Some(2));
//! assert_eq!(deque.pop_front(), Some(1));
//! assert_eq!(deque.pop_front(), Some(3));
//!
//! // You can iterate it, yielding all the elements front-to-back.
//! for x in &deque {
//!     println!("{}", x);
//! }
//! ```

use crate::{
    vec::{OwnedVecStorage, VecStorage, VecStorageInner, ViewVecStorage},
    CapacityError,
};
use core::{
    cmp::Ordering,
    fmt,
    iter::FusedIterator,
    marker::PhantomData,
    mem::{ManuallyDrop, MaybeUninit},
    ptr, slice,
};

#[cfg(feature = "zeroize")]
use zeroize::Zeroize;

/// Base struct for [`Deque`] and [`DequeView`], generic over the [`VecStorage`].
///
/// In most cases you should use [`Deque`] or [`DequeView`] directly. Only use this
/// struct if you want to write code that's generic over both.
#[cfg_attr(feature = "zeroize", derive(Zeroize))]
pub struct DequeInner<T, S: VecStorage<T> + ?Sized> {
    // This phantomdata is required because otherwise rustc thinks that `T` is not used
    phantom: PhantomData<T>,
    /// Front index. Always 0..=(N-1)
    front: usize,
    /// Back index. Always 0..=(N-1).
    back: usize,

    /// Used to distinguish "empty" and "full" cases when `front == back`.
    /// May only be `true` if `front == back`, always `false` otherwise.
    full: bool,
    buffer: S,
}

/// A fixed capacity double-ended queue.
///
/// # Examples
///
/// ```
/// use heapless::Deque;
///
/// // A deque with a fixed capacity of 8 elements allocated on the stack
/// let mut deque = Deque::<_, 8>::new();
///
/// // You can use it as a good old FIFO queue.
/// deque.push_back(1);
/// deque.push_back(2);
/// assert_eq!(deque.len(), 2);
///
/// assert_eq!(deque.pop_front(), Some(1));
/// assert_eq!(deque.pop_front(), Some(2));
/// assert_eq!(deque.len(), 0);
///
/// // Deque is double-ended, you can push and pop from the front and back.
/// deque.push_back(1);
/// deque.push_front(2);
/// deque.push_back(3);
/// deque.push_front(4);
/// assert_eq!(deque.pop_front(), Some(4));
/// assert_eq!(deque.pop_front(), Some(2));
/// assert_eq!(deque.pop_front(), Some(1));
/// assert_eq!(deque.pop_front(), Some(3));
///
/// // You can iterate it, yielding all the elements front-to-back.
/// for x in &deque {
///     println!("{}", x);
/// }
/// ```
pub type Deque<T, const N: usize> = DequeInner<T, OwnedVecStorage<T, N>>;

/// A double-ended queue with dynamic capacity.
///
/// # Examples
///
/// ```
/// use heapless::deque::{Deque, DequeView};
///
/// // A deque with a fixed capacity of 8 elements allocated on the stack
/// let mut deque_buf = Deque::<_, 8>::new();
///
/// // A DequeView can be obtained through unsized coercion of a `Deque`
/// let deque: &mut DequeView<_> = &mut deque_buf;
///
/// // You can use it as a good old FIFO queue.
/// deque.push_back(1);
/// deque.push_back(2);
/// assert_eq!(deque.storage_len(), 2);
///
/// assert_eq!(deque.pop_front(), Some(1));
/// assert_eq!(deque.pop_front(), Some(2));
/// assert_eq!(deque.storage_len(), 0);
///
/// // DequeView is double-ended, you can push and pop from the front and back.
/// deque.push_back(1);
/// deque.push_front(2);
/// deque.push_back(3);
/// deque.push_front(4);
/// assert_eq!(deque.pop_front(), Some(4));
/// assert_eq!(deque.pop_front(), Some(2));
/// assert_eq!(deque.pop_front(), Some(1));
/// assert_eq!(deque.pop_front(), Some(3));
///
/// // You can iterate it, yielding all the elements front-to-back.
/// for x in deque {
///     println!("{}", x);
/// }
/// ```
pub type DequeView<T> = DequeInner<T, ViewVecStorage<T>>;

impl<T, const N: usize> Deque<T, N> {
    const INIT: MaybeUninit<T> = MaybeUninit::uninit();

    /// Constructs a new, empty deque with a fixed capacity of `N`
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Deque;
    ///
    /// // allocate the deque on the stack
    /// let mut x: Deque<u8, 16> = Deque::new();
    ///
    /// // allocate the deque in a static variable
    /// static mut X: Deque<u8, 16> = Deque::new();
    /// ```
    pub const fn new() -> Self {
        const {
            assert!(N > 0);
        }

        Self {
            phantom: PhantomData,
            buffer: VecStorageInner {
                buffer: [Self::INIT; N],
            },
            front: 0,
            back: 0,
            full: false,
        }
    }

    /// Returns the maximum number of elements the deque can hold.
    ///
    /// This method is not available on a `DequeView`, use
    /// [`storage_capacity`](DequeInner::storage_capacity) instead.
    pub const fn capacity(&self) -> usize {
        N
    }

    /// Returns the number of elements currently in the deque.
    ///
    /// This method is not available on a `DequeView`, use [`storage_len`](DequeInner::storage_len)
    /// instead.
    pub const fn len(&self) -> usize {
        if self.full {
            N
        } else if self.back < self.front {
            self.back + N - self.front
        } else {
            self.back - self.front
        }
    }
}

impl<T, S: VecStorage<T> + ?Sized> DequeInner<T, S> {
    /// Get a reference to the `Deque`, erasing the `N` const-generic.
    pub fn as_view(&self) -> &DequeView<T> {
        S::as_deque_view(self)
    }

    /// Get a mutable reference to the `Deque`, erasing the `N` const-generic.
    pub fn as_mut_view(&mut self) -> &mut DequeView<T> {
        S::as_deque_view_mut(self)
    }

    /// Returns the maximum number of elements the deque can hold.
    pub fn storage_capacity(&self) -> usize {
        self.buffer.borrow().len()
    }

    fn increment(&self, i: usize) -> usize {
        if i + 1 == self.storage_capacity() {
            0
        } else {
            i + 1
        }
    }

    fn decrement(&self, i: usize) -> usize {
        if i == 0 {
            self.storage_capacity() - 1
        } else {
            i - 1
        }
    }

    /// Returns the number of elements currently in the deque.
    pub fn storage_len(&self) -> usize {
        if self.full {
            self.storage_capacity()
        } else if self.back < self.front {
            self.back + self.storage_capacity() - self.front
        } else {
            self.back - self.front
        }
    }

    /// Clears the deque, removing all values.
    pub fn clear(&mut self) {
        // safety: we're immediately setting a consistent empty state.
        unsafe { self.drop_contents() }
        self.front = 0;
        self.back = 0;
        self.full = false;
    }

    /// Drop all items in the `Deque`, leaving the state `back/front/full` unmodified.
    ///
    /// safety: leaves the `Deque` in an inconsistent state, so can cause duplicate drops.
    unsafe fn drop_contents(&mut self) {
        // We drop each element used in the deque by turning into a &mut[T]
        let (a, b) = self.as_mut_slices();
        ptr::drop_in_place(a);
        ptr::drop_in_place(b);
    }

    /// Returns whether the deque is empty.
    pub fn is_empty(&self) -> bool {
        self.front == self.back && !self.full
    }

    /// Returns whether the deque is full (i.e. if `len() == capacity()`.
    pub fn is_full(&self) -> bool {
        self.full
    }

    /// Returns a pair of slices which contain, in order, the contents of the `Deque`.
    pub fn as_slices(&self) -> (&[T], &[T]) {
        // NOTE(unsafe) avoid bound checks in the slicing operation
        unsafe {
            if self.is_empty() {
                (&[], &[])
            } else if self.back <= self.front {
                (
                    slice::from_raw_parts(
                        self.buffer.borrow().as_ptr().add(self.front).cast::<T>(),
                        self.storage_capacity() - self.front,
                    ),
                    slice::from_raw_parts(self.buffer.borrow().as_ptr().cast::<T>(), self.back),
                )
            } else {
                (
                    slice::from_raw_parts(
                        self.buffer.borrow().as_ptr().add(self.front).cast::<T>(),
                        self.back - self.front,
                    ),
                    &[],
                )
            }
        }
    }

    /// Returns a pair of mutable slices which contain, in order, the contents of the `Deque`.
    pub fn as_mut_slices(&mut self) -> (&mut [T], &mut [T]) {
        let ptr = self.buffer.borrow_mut().as_mut_ptr();

        // NOTE(unsafe) avoid bound checks in the slicing operation
        unsafe {
            if self.is_empty() {
                (&mut [], &mut [])
            } else if self.back <= self.front {
                (
                    slice::from_raw_parts_mut(
                        ptr.add(self.front).cast::<T>(),
                        self.storage_capacity() - self.front,
                    ),
                    slice::from_raw_parts_mut(ptr.cast::<T>(), self.back),
                )
            } else {
                (
                    slice::from_raw_parts_mut(
                        ptr.add(self.front).cast::<T>(),
                        self.back - self.front,
                    ),
                    &mut [],
                )
            }
        }
    }

    #[inline]
    fn is_contiguous(&self) -> bool {
        self.front <= self.storage_capacity() - self.storage_len()
    }

    /// Rearranges the internal storage of the [`Deque`] to make it into a contiguous slice,
    /// which is returned.
    ///
    /// This does **not** change the order of the elements in the deque.
    /// The returned slice can then be used to perform contiguous slice operations on the deque.
    ///
    /// After calling this method, subsequent [`as_slices`] and [`as_mut_slices`] calls will return
    /// a single contiguous slice.
    ///
    /// [`as_slices`]: Deque::as_slices
    /// [`as_mut_slices`]: Deque::as_mut_slices
    ///
    /// # Examples
    /// Sorting a deque:
    /// ```
    /// use heapless::Deque;
    ///
    /// let mut buf = Deque::<_, 4>::new();
    /// buf.push_back(2).unwrap();
    /// buf.push_back(1).unwrap();
    /// buf.push_back(3).unwrap();
    ///
    /// // Sort the deque
    /// buf.make_contiguous().sort();
    /// assert_eq!(buf.as_slices(), (&[1, 2, 3][..], &[][..]));
    ///
    /// // Sort the deque in reverse
    /// buf.make_contiguous().sort_by(|a, b| b.cmp(a));
    /// assert_eq!(buf.as_slices(), (&[3, 2, 1][..], &[][..]));
    /// ```
    pub fn make_contiguous(&mut self) -> &mut [T] {
        if self.is_contiguous() {
            return unsafe {
                slice::from_raw_parts_mut(
                    self.buffer.borrow_mut().as_mut_ptr().add(self.front).cast(),
                    self.storage_len(),
                )
            };
        }

        let buffer_ptr: *mut T = self.buffer.borrow_mut().as_mut_ptr().cast();

        let len = self.storage_len();

        let free = self.storage_capacity() - len;
        let front_len = self.storage_capacity() - self.front;
        let back = len - front_len;
        let back_len = back;

        if free >= front_len {
            // there is enough free space to copy the head in one go,
            // this means that we first shift the tail backwards, and then
            // copy the head to the correct position.
            //
            // from: DEFGH....ABC
            // to:   ABCDEFGH....
            unsafe {
                ptr::copy(buffer_ptr, buffer_ptr.add(front_len), back_len);
                // ...DEFGH.ABC
                ptr::copy_nonoverlapping(buffer_ptr.add(self.front), buffer_ptr, front_len);
                // ABCDEFGH....
            }

            self.front = 0;
            self.back = len;
        } else if free >= back_len {
            // there is enough free space to copy the tail in one go,
            // this means that we first shift the head forwards, and then
            // copy the tail to the correct position.
            //
            // from: FGH....ABCDE
            // to:   ...ABCDEFGH.
            unsafe {
                ptr::copy(
                    buffer_ptr.add(self.front),
                    buffer_ptr.add(self.back),
                    front_len,
                );
                // FGHABCDE....
                ptr::copy_nonoverlapping(
                    buffer_ptr,
                    buffer_ptr.add(self.back + front_len),
                    back_len,
                );
                // ...ABCDEFGH.
            }

            self.front = back;
            self.back = 0;
        } else {
            // `free` is smaller than both `head_len` and `tail_len`.
            // the general algorithm for this first moves the slices
            // right next to each other and then uses `slice::rotate`
            // to rotate them into place:
            //
            // initially:   HIJK..ABCDEFG
            // step 1:      ..HIJKABCDEFG
            // step 2:      ..ABCDEFGHIJK
            //
            // or:
            //
            // initially:   FGHIJK..ABCDE
            // step 1:      FGHIJKABCDE..
            // step 2:      ABCDEFGHIJK..

            // pick the shorter of the 2 slices to reduce the amount
            // of memory that needs to be moved around.
            if front_len > back_len {
                // tail is shorter, so:
                //  1. copy tail forwards
                //  2. rotate used part of the buffer
                //  3. update head to point to the new beginning (which is just `free`)
                unsafe {
                    // if there is no free space in the buffer, then the slices are already
                    // right next to each other and we don't need to move any memory.
                    if free != 0 {
                        // because we only move the tail forward as much as there's free space
                        // behind it, we don't overwrite any elements of the head slice, and
                        // the slices end up right next to each other.
                        ptr::copy(buffer_ptr, buffer_ptr.add(free), back_len);
                    }

                    // We just copied the tail right next to the head slice,
                    // so all of the elements in the range are initialized
                    let slice: &mut [T] = slice::from_raw_parts_mut(
                        buffer_ptr.add(free),
                        self.storage_capacity() - free,
                    );

                    // because the deque wasn't contiguous, we know that `tail_len < self.len ==
                    // slice.len()`, so this will never panic.
                    slice.rotate_left(back_len);

                    // the used part of the buffer now is `free..self.capacity()`, so set
                    // `head` to the beginning of that range.
                    self.front = free;
                    self.back = 0;
                }
            } else {
                // head is shorter so:
                //  1. copy head backwards
                //  2. rotate used part of the buffer
                //  3. update head to point to the new beginning (which is the beginning of the
                //     buffer)

                unsafe {
                    // if there is no free space in the buffer, then the slices are already
                    // right next to each other and we don't need to move any memory.
                    if free != 0 {
                        // copy the head slice to lie right behind the tail slice.
                        ptr::copy(
                            buffer_ptr.add(self.front),
                            buffer_ptr.add(back_len),
                            front_len,
                        );
                    }

                    // because we copied the head slice so that both slices lie right
                    // next to each other, all the elements in the range are initialized.
                    let slice: &mut [T] = slice::from_raw_parts_mut(buffer_ptr, len);

                    // because the deque wasn't contiguous, we know that `head_len < self.len ==
                    // slice.len()` so this will never panic.
                    slice.rotate_right(front_len);

                    // the used part of the buffer now is `0..self.len`, so set
                    // `head` to the beginning of that range.
                    self.front = 0;
                    self.back = len;
                }
            }
        }

        unsafe { slice::from_raw_parts_mut(buffer_ptr.add(self.front), len) }
    }

    /// Provides a reference to the front element, or None if the `Deque` is empty.
    pub fn front(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { &*self.buffer.borrow().get_unchecked(self.front).as_ptr() })
        }
    }

    /// Provides a mutable reference to the front element, or None if the `Deque` is empty.
    pub fn front_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe {
                &mut *self
                    .buffer
                    .borrow_mut()
                    .get_unchecked_mut(self.front)
                    .as_mut_ptr()
            })
        }
    }

    /// Provides a reference to the back element, or None if the `Deque` is empty.
    pub fn back(&self) -> Option<&T> {
        if self.is_empty() {
            None
        } else {
            let index = self.decrement(self.back);
            Some(unsafe { &*self.buffer.borrow().get_unchecked(index).as_ptr() })
        }
    }

    /// Provides a mutable reference to the back element, or None if the `Deque` is empty.
    pub fn back_mut(&mut self) -> Option<&mut T> {
        if self.is_empty() {
            None
        } else {
            let index = self.decrement(self.back);
            Some(unsafe {
                &mut *self
                    .buffer
                    .borrow_mut()
                    .get_unchecked_mut(index)
                    .as_mut_ptr()
            })
        }
    }

    /// Removes the item from the front of the deque and returns it, or `None` if it's empty
    pub fn pop_front(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { self.pop_front_unchecked() })
        }
    }

    /// Removes the item from the back of the deque and returns it, or `None` if it's empty
    pub fn pop_back(&mut self) -> Option<T> {
        if self.is_empty() {
            None
        } else {
            Some(unsafe { self.pop_back_unchecked() })
        }
    }

    /// Appends an `item` to the front of the deque
    ///
    /// Returns back the `item` if the deque is full
    pub fn push_front(&mut self, item: T) -> Result<(), T> {
        if self.is_full() {
            Err(item)
        } else {
            unsafe { self.push_front_unchecked(item) }
            Ok(())
        }
    }

    /// Appends an `item` to the back of the deque
    ///
    /// Returns back the `item` if the deque is full
    pub fn push_back(&mut self, item: T) -> Result<(), T> {
        if self.is_full() {
            Err(item)
        } else {
            unsafe { self.push_back_unchecked(item) }
            Ok(())
        }
    }

    /// Removes an item from the front of the deque and returns it, without checking that the deque
    /// is not empty
    ///
    /// # Safety
    ///
    /// It's undefined behavior to call this on an empty deque
    pub unsafe fn pop_front_unchecked(&mut self) -> T {
        debug_assert!(!self.is_empty());

        let index = self.front;
        self.full = false;
        self.front = self.increment(self.front);
        self.buffer
            .borrow_mut()
            .get_unchecked_mut(index)
            .as_ptr()
            .read()
    }

    /// Removes an item from the back of the deque and returns it, without checking that the deque
    /// is not empty
    ///
    /// # Safety
    ///
    /// It's undefined behavior to call this on an empty deque
    pub unsafe fn pop_back_unchecked(&mut self) -> T {
        debug_assert!(!self.is_empty());

        self.full = false;
        self.back = self.decrement(self.back);
        self.buffer
            .borrow_mut()
            .get_unchecked_mut(self.back)
            .as_ptr()
            .read()
    }

    /// Appends an `item` to the front of the deque
    ///
    /// # Safety
    ///
    /// This assumes the deque is not full.
    pub unsafe fn push_front_unchecked(&mut self, item: T) {
        debug_assert!(!self.is_full());

        let index = self.decrement(self.front);
        // NOTE: the memory slot that we are about to write to is uninitialized. We assign
        // a `MaybeUninit` to avoid running `T`'s destructor on the uninitialized memory
        *self.buffer.borrow_mut().get_unchecked_mut(index) = MaybeUninit::new(item);
        self.front = index;
        if self.front == self.back {
            self.full = true;
        }
    }

    /// Appends an `item` to the back of the deque
    ///
    /// # Safety
    ///
    /// This assumes the deque is not full.
    pub unsafe fn push_back_unchecked(&mut self, item: T) {
        debug_assert!(!self.is_full());

        // NOTE: the memory slot that we are about to write to is uninitialized. We assign
        // a `MaybeUninit` to avoid running `T`'s destructor on the uninitialized memory
        *self.buffer.borrow_mut().get_unchecked_mut(self.back) = MaybeUninit::new(item);
        self.back = self.increment(self.back);
        if self.front == self.back {
            self.full = true;
        }
    }

    /// Returns a reference to the element at the given index.
    ///
    /// Index 0 is the front of the `Deque`.
    pub fn get(&self, index: usize) -> Option<&T> {
        if index < self.storage_len() {
            let idx = self.to_physical_index(index);
            Some(unsafe { self.buffer.borrow().get_unchecked(idx).assume_init_ref() })
        } else {
            None
        }
    }

    /// Returns a mutable reference to the element at the given index.
    ///
    /// Index 0 is the front of the `Deque`.
    pub fn get_mut(&mut self, index: usize) -> Option<&mut T> {
        if index < self.storage_len() {
            let idx = self.to_physical_index(index);
            Some(unsafe {
                self.buffer
                    .borrow_mut()
                    .get_unchecked_mut(idx)
                    .assume_init_mut()
            })
        } else {
            None
        }
    }

    /// Returns a reference to the element at the given index without checking if it exists.
    ///
    /// # Safety
    ///
    /// The element at the given `index` must exist (i.e. `index < self.len()`).
    pub unsafe fn get_unchecked(&self, index: usize) -> &T {
        debug_assert!(index < self.storage_len());

        let idx = self.to_physical_index(index);
        self.buffer.borrow().get_unchecked(idx).assume_init_ref()
    }

    /// Returns a mutable reference to the element at the given index without checking if it exists.
    ///
    /// # Safety
    ///
    /// The element at the given `index` must exist (i.e. `index < self.len()`).
    pub unsafe fn get_unchecked_mut(&mut self, index: usize) -> &mut T {
        debug_assert!(index < self.storage_len());

        let idx = self.to_physical_index(index);
        self.buffer
            .borrow_mut()
            .get_unchecked_mut(idx)
            .assume_init_mut()
    }

    /// Swaps elements at indices `i` and `j`.
    ///
    /// # Panics
    ///
    /// Panics if either `i` or `j` are out of bounds.
    pub fn swap(&mut self, i: usize, j: usize) {
        let len = self.storage_len();
        assert!(i < len);
        assert!(j < len);
        unsafe { self.swap_unchecked(i, j) }
    }

    /// Swaps elements at indices `i` and `j` without checking that they exist.
    ///
    /// # Safety
    ///
    /// Elements at indexes `i` and `j` must exist (i.e. `i < self.len()` and `j < self.len()`).
    pub unsafe fn swap_unchecked(&mut self, i: usize, j: usize) {
        debug_assert!(i < self.storage_len());
        debug_assert!(j < self.storage_len());
        let idx_i = self.to_physical_index(i);
        let idx_j = self.to_physical_index(j);

        let buffer = self.buffer.borrow_mut();
        let buffer_ptr = buffer.as_mut_ptr();
        let ptr_i = buffer_ptr.add(idx_i);
        let ptr_j = buffer_ptr.add(idx_j);
        ptr::swap(ptr_i, ptr_j);
    }

    /// Removes an element from anywhere in the deque and returns it, replacing it with the first
    /// element.
    ///
    /// This does not preserve ordering, but is *O*(1).
    ///
    /// Returns `None` if `index` is out of bounds.
    ///
    /// Element at index 0 is the front of the queue.
    pub fn swap_remove_front(&mut self, index: usize) -> Option<T> {
        let len = self.storage_len();
        if len > 0 && index < len {
            Some(unsafe {
                self.swap_unchecked(index, 0);
                self.pop_front_unchecked()
            })
        } else {
            None
        }
    }

    /// Removes an element from anywhere in the deque and returns it, replacing it with the last
    /// element.
    ///
    /// This does not preserve ordering, but is *O*(1).
    ///
    /// Returns `None` if `index` is out of bounds.
    ///
    /// Element at index 0 is the front of the queue.
    pub fn swap_remove_back(&mut self, index: usize) -> Option<T> {
        let len = self.storage_len();
        if len > 0 && index < len {
            Some(unsafe {
                self.swap_unchecked(index, len - 1);
                self.pop_back_unchecked()
            })
        } else {
            None
        }
    }

    fn to_physical_index(&self, index: usize) -> usize {
        let mut res = self.front + index;
        if res >= self.storage_capacity() {
            res -= self.storage_capacity();
        }
        res
    }

    /// Returns an iterator over the deque.
    pub fn iter(&self) -> Iter<'_, T> {
        let (start, end) = self.as_slices();
        Iter {
            inner: start.iter().chain(end),
        }
    }

    /// Returns an iterator that allows modifying each value.
    pub fn iter_mut(&mut self) -> IterMut<'_, T> {
        let (start, end) = self.as_mut_slices();
        IterMut {
            inner: start.iter_mut().chain(end),
        }
    }

    /// Shortens the deque, keeping the first `len` elements and dropping
    /// the rest.
    ///
    /// If `len` is greater or equal to the deque's current length, this has
    /// no effect.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Deque;
    ///
    /// let mut buf: Deque<_, 5> = Deque::new();
    /// buf.push_back(5);
    /// buf.push_back(10);
    /// buf.push_back(15);
    /// buf.truncate(1);
    /// assert_eq!(buf.make_contiguous(), [5]);
    /// ```
    pub fn truncate(&mut self, len: usize) {
        /// Runs the destructor for all items in the slice when it gets dropped (gracefully or
        /// during unwinding).
        struct Dropper<'a, T>(&'a mut [T]);

        impl<'a, T> Drop for Dropper<'a, T> {
            fn drop(&mut self) {
                unsafe {
                    ptr::drop_in_place(self.0);
                }
            }
        }

        // Safety:
        // * Any slice passed to `drop_in_place` is valid; the second case has `len <= front.len()`
        //   and returning on `len > self.storage_len()` ensures `begin <= back.len()` in the first
        //   case
        // * Deque front/back cursors are moved before calling `drop_in_place`, so no value is
        //   dropped twice if `drop_in_place` panics
        unsafe {
            // If new desired length is greater or equal, we don't need to act.
            if len >= self.storage_len() {
                return;
            }

            let (front, back) = self.as_mut_slices();

            // If `len` desires to keep elements past front's entire length,
            // then only back's contents will need to be dropped
            // as the two slices combined should be more than `len`.
            if len > front.len() {
                let begin = len - front.len();
                let drop_back = back.get_unchecked_mut(begin..) as *mut _;

                // Self::to_physical_index returns the index `len` units _after_ the front cursor,
                // meaning we can use it to find the decremented index for `back` for non-contiguous
                // deques, as well as determine where the new "cap" for front needs
                // to be placed for contiguous deques.
                self.back = self.to_physical_index(len);
                self.full = false;

                ptr::drop_in_place(drop_back);
            } else {
                // Otherwise, we know back's entire contents need to be dropped,
                // since the desired length never reaches into it.
                let drop_back = back as *mut _;
                let drop_front = front.get_unchecked_mut(len..) as *mut _;

                self.back = self.to_physical_index(len);
                self.full = false;

                // If `drop_front` causes a panic, the Dropper will still be called to drop it's
                // slice during unwinding. In either case, front will always be
                // dropped before back.
                let _back_dropper = Dropper(&mut *drop_back);
                ptr::drop_in_place(drop_front);
            }
        }
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` for which `f(&e)` returns false.
    /// This method operates in place, visiting each element exactly once in the
    /// original order, and preserves the order of the retained elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Deque;
    ///
    /// let mut buf: Deque<_, 10> = Deque::new();
    /// buf.extend(1..5);
    /// buf.retain(|&x| x % 2 == 0);
    /// assert_eq!(buf.make_contiguous(), [2, 4]);
    /// ```
    ///
    /// Because the elements are visited exactly once in the original order,
    /// external state may be used to decide which elements to keep.
    ///
    /// ```
    /// use heapless::Deque;
    ///
    /// let mut buf: Deque<_, 10> = Deque::new();
    /// buf.extend(1..6);
    ///
    /// let keep = [false, true, true, false, true];
    /// let mut iter = keep.iter();
    /// buf.retain(|_| *iter.next().unwrap());
    /// assert_eq!(buf.make_contiguous(), [2, 3, 5]);
    /// ```
    pub fn retain<F>(&mut self, mut f: F)
    where
        F: FnMut(&T) -> bool,
    {
        self.retain_mut(|elem| f(elem));
    }

    /// Retains only the elements specified by the predicate.
    ///
    /// In other words, remove all elements `e` for which `f(&mut e)` returns false.
    /// This method operates in place, visiting each element exactly once in the
    /// original order, and preserves the order of the retained elements.
    ///
    /// # Examples
    ///
    /// ```
    /// use heapless::Deque;
    ///
    /// let mut buf: Deque<_, 10> = Deque::new();
    /// buf.extend(1..5);
    /// buf.retain_mut(|x| {
    ///     if *x % 2 == 0 {
    ///         *x += 1;
    ///         true
    ///     } else {
    ///         false
    ///     }
    /// });
    /// assert_eq!(buf.make_contiguous(), [3, 5]);
    /// ```
    pub fn retain_mut<F>(&mut self, mut f: F)
    where
        F: FnMut(&mut T) -> bool,
    {
        let len = self.storage_len();
        let mut idx = 0;
        let mut cur = 0;

        // Stage 1: Check if all values can be retained.
        while cur < len {
            let val = self
                .get_mut(cur)
                .expect("cur was checked to be less than deque's length");
            if !f(val) {
                cur += 1;
                break;
            }

            cur += 1;
            idx += 1;
        }
        // Stage 2: Swap retained values into current idx, building a contiguous chunk from 0 to
        // idx.
        while cur < len {
            let val = self
                .get_mut(cur)
                .expect("cur was checked to be less than deque's length");
            if !f(val) {
                cur += 1;
                continue;
            }

            self.swap(idx, cur);
            cur += 1;
            idx += 1;
        }
        // Stage 3: Truncate any moved values after idx.
        if cur != idx {
            self.truncate(idx);
        }
    }
}

/// Iterator over the contents of a [`Deque`]
pub struct Iter<'a, T> {
    inner: core::iter::Chain<core::slice::Iter<'a, T>, core::slice::Iter<'a, T>>,
}

/// Iterator over the contents of a [`Deque`]
pub struct IterMut<'a, T> {
    inner: core::iter::Chain<core::slice::IterMut<'a, T>, core::slice::IterMut<'a, T>>,
}

impl<'a, T> Iterator for Iter<'a, T> {
    type Item = &'a T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }

    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T> DoubleEndedIterator for Iter<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<T> ExactSizeIterator for Iter<'_, T> {}
impl<T> FusedIterator for Iter<'_, T> {}

impl<'a, T> Iterator for IterMut<'a, T> {
    type Item = &'a mut T;
    #[inline]
    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next()
    }
    #[inline]
    fn size_hint(&self) -> (usize, Option<usize>) {
        self.inner.size_hint()
    }
}

impl<T> DoubleEndedIterator for IterMut<'_, T> {
    #[inline]
    fn next_back(&mut self) -> Option<Self::Item> {
        self.inner.next_back()
    }
}

impl<T> ExactSizeIterator for IterMut<'_, T> {}
impl<T> FusedIterator for IterMut<'_, T> {}

// Trait implementations

impl<T, const N: usize> Default for Deque<T, N> {
    fn default() -> Self {
        Self::new()
    }
}

impl<T, S: VecStorage<T> + ?Sized> Drop for DequeInner<T, S> {
    fn drop(&mut self) {
        // safety: `self` is left in an inconsistent state but it doesn't matter since
        // it's getting dropped. Nothing should be able to observe `self` after drop.
        unsafe { self.drop_contents() }
    }
}

impl<T: fmt::Debug, S: VecStorage<T> + ?Sized> fmt::Debug for DequeInner<T, S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self).finish()
    }
}

/// As with the standard library's `VecDeque`, items are added via `push_back`.
impl<T, S: VecStorage<T> + ?Sized> Extend<T> for DequeInner<T, S> {
    fn extend<I: IntoIterator<Item = T>>(&mut self, iter: I) {
        for item in iter {
            self.push_back(item).ok().unwrap();
        }
    }
}
impl<'a, T: 'a + Copy, S: VecStorage<T> + ?Sized> Extend<&'a T> for DequeInner<T, S> {
    fn extend<I: IntoIterator<Item = &'a T>>(&mut self, iter: I) {
        self.extend(iter.into_iter().copied());
    }
}

/// An iterator that moves out of a [`Deque`].
///
/// This struct is created by calling the `into_iter` method.
#[derive(Clone)]
pub struct IntoIter<T, const N: usize> {
    deque: Deque<T, N>,
}

impl<T, const N: usize> Iterator for IntoIter<T, N> {
    type Item = T;
    fn next(&mut self) -> Option<Self::Item> {
        self.deque.pop_front()
    }
    fn size_hint(&self) -> (usize, Option<usize>) {
        let len = self.len();
        (len, Some(len))
    }
}
impl<T, const N: usize> DoubleEndedIterator for IntoIter<T, N> {
    fn next_back(&mut self) -> Option<Self::Item> {
        self.deque.pop_back()
    }
}
impl<T, const N: usize> FusedIterator for IntoIter<T, N> {}
impl<T, const N: usize> ExactSizeIterator for IntoIter<T, N> {
    fn len(&self) -> usize {
        self.deque.len()
    }
}

impl<T, const N: usize> IntoIterator for Deque<T, N> {
    type Item = T;
    type IntoIter = IntoIter<T, N>;

    fn into_iter(self) -> Self::IntoIter {
        IntoIter { deque: self }
    }
}

impl<'a, T, S: VecStorage<T> + ?Sized> IntoIterator for &'a DequeInner<T, S> {
    type Item = &'a T;
    type IntoIter = Iter<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter()
    }
}

impl<'a, T, S: VecStorage<T> + ?Sized> IntoIterator for &'a mut DequeInner<T, S> {
    type Item = &'a mut T;
    type IntoIter = IterMut<'a, T>;

    fn into_iter(self) -> Self::IntoIter {
        self.iter_mut()
    }
}

impl<T, const N: usize> Clone for Deque<T, N>
where
    T: Clone,
{
    fn clone(&self) -> Self {
        let mut res = Self::new();
        for i in self {
            // safety: the original and new deques have the same capacity, so it can
            // not become full.
            unsafe { res.push_back_unchecked(i.clone()) }
        }
        res
    }
}

impl<T: PartialEq, const N: usize> PartialEq for Deque<T, N> {
    fn eq(&self, other: &Self) -> bool {
        if self.len() != other.len() {
            return false;
        }
        let (sa, sb) = self.as_slices();
        let (oa, ob) = other.as_slices();
        match sa.len().cmp(&oa.len()) {
            Ordering::Equal => sa == oa && sb == ob,
            Ordering::Less => {
                // Always divisible in three sections, for example:
                // self:  [a b c|d e f]
                // other: [0 1 2 3|4 5]
                // front = 3, mid = 1,
                // [a b c] == [0 1 2] && [d] == [3] && [e f] == [4 5]
                let front = sa.len();
                let mid = oa.len() - front;

                let (oa_front, oa_mid) = oa.split_at(front);
                let (sb_mid, sb_back) = sb.split_at(mid);
                debug_assert_eq!(sa.len(), oa_front.len());
                debug_assert_eq!(sb_mid.len(), oa_mid.len());
                debug_assert_eq!(sb_back.len(), ob.len());
                sa == oa_front && sb_mid == oa_mid && sb_back == ob
            }
            Ordering::Greater => {
                let front = oa.len();
                let mid = sa.len() - front;

                let (sa_front, sa_mid) = sa.split_at(front);
                let (ob_mid, ob_back) = ob.split_at(mid);
                debug_assert_eq!(sa_front.len(), oa.len());
                debug_assert_eq!(sa_mid.len(), ob_mid.len());
                debug_assert_eq!(sb.len(), ob_back.len());
                sa_front == oa && sa_mid == ob_mid && sb == ob_back
            }
        }
    }
}

impl<T: Eq, const N: usize> Eq for Deque<T, N> {}

impl<T, const NS: usize, const ND: usize> TryFrom<[T; NS]> for Deque<T, ND> {
    /// Converts a `[T; NS]` into a `Deque<T, ND>`.
    ///
    /// ```
    /// use heapless::Deque;
    ///
    /// let deq1 = Deque::<u8, 5>::try_from([1, 2, 3]).unwrap();
    /// let mut deq2 = Deque::<u8, 5>::new();
    /// deq2.push_back(1).unwrap();
    /// deq2.push_back(2).unwrap();
    /// deq2.push_back(3).unwrap();
    ///
    /// assert_eq!(deq1, deq2);
    /// ```
    type Error = (CapacityError, [T; NS]);

    /// Converts a `[T; NS]` array into a `Deque<T, ND>`.
    ///
    /// Returns back the `value` if NS > ND.
    fn try_from(value: [T; NS]) -> Result<Self, Self::Error> {
        if NS > ND {
            return Err((CapacityError, value));
        }

        let mut deq = Self::default();
        let value = ManuallyDrop::new(value);

        // SAFETY: We already ensured that value fits in deq.
        unsafe {
            ptr::copy_nonoverlapping(
                value.as_ptr(),
                deq.buffer.buffer.as_mut_ptr().cast::<T>(),
                NS,
            );
        }

        deq.front = 0;
        deq.back = NS;
        deq.full = NS == ND;

        Ok(deq)
    }
}

#[cfg(test)]
mod tests {
    use super::Deque;
    use crate::CapacityError;
    use static_assertions::assert_not_impl_any;

    // Ensure a `Deque` containing `!Send` values stays `!Send` itself.
    assert_not_impl_any!(Deque<*const (), 4>: Send);

    #[test]
    fn static_new() {
        static mut _V: Deque<i32, 4> = Deque::new();
    }

    #[test]
    fn stack_new() {
        let mut _v: Deque<i32, 4> = Deque::new();
    }

    #[test]
    fn drop() {
        droppable!();

        {
            let mut v: Deque<Droppable, 2> = Deque::new();
            v.push_back(Droppable::new()).ok().unwrap();
            v.push_back(Droppable::new()).ok().unwrap();
            v.pop_front().unwrap();
        }

        assert_eq!(Droppable::count(), 0);

        {
            let mut v: Deque<Droppable, 2> = Deque::new();
            v.push_back(Droppable::new()).ok().unwrap();
            v.push_back(Droppable::new()).ok().unwrap();
        }

        assert_eq!(Droppable::count(), 0);
        {
            let mut v: Deque<Droppable, 2> = Deque::new();
            v.push_front(Droppable::new()).ok().unwrap();
            v.push_front(Droppable::new()).ok().unwrap();
        }

        assert_eq!(Droppable::count(), 0);
    }

    #[test]
    fn full() {
        let mut v: Deque<i32, 4> = Deque::new();

        v.push_back(0).unwrap();
        v.push_front(1).unwrap();
        v.push_back(2).unwrap();
        v.push_back(3).unwrap();

        assert!(v.push_front(4).is_err());
        assert!(v.push_back(4).is_err());
        assert!(v.is_full());
    }

    #[test]
    fn empty() {
        let mut v: Deque<i32, 4> = Deque::new();
        assert!(v.is_empty());

        v.push_back(0).unwrap();
        assert!(!v.is_empty());

        v.push_front(1).unwrap();
        assert!(!v.is_empty());

        v.pop_front().unwrap();
        v.pop_front().unwrap();

        assert!(v.pop_front().is_none());
        assert!(v.pop_back().is_none());
        assert!(v.is_empty());
    }

    #[test]
    fn front_back() {
        let mut v: Deque<i32, 4> = Deque::new();
        assert_eq!(v.front(), None);
        assert_eq!(v.front_mut(), None);
        assert_eq!(v.back(), None);
        assert_eq!(v.back_mut(), None);

        v.push_back(4).unwrap();
        assert_eq!(v.front(), Some(&4));
        assert_eq!(v.front_mut(), Some(&mut 4));
        assert_eq!(v.back(), Some(&4));
        assert_eq!(v.back_mut(), Some(&mut 4));

        v.push_front(3).unwrap();
        assert_eq!(v.front(), Some(&3));
        assert_eq!(v.front_mut(), Some(&mut 3));
        assert_eq!(v.back(), Some(&4));
        assert_eq!(v.back_mut(), Some(&mut 4));

        v.pop_back().unwrap();
        assert_eq!(v.front(), Some(&3));
        assert_eq!(v.front_mut(), Some(&mut 3));
        assert_eq!(v.back(), Some(&3));
        assert_eq!(v.back_mut(), Some(&mut 3));

        v.pop_front().unwrap();
        assert_eq!(v.front(), None);
        assert_eq!(v.front_mut(), None);
        assert_eq!(v.back(), None);
        assert_eq!(v.back_mut(), None);
    }

    #[test]
    fn extend() {
        let mut v: Deque<i32, 4> = Deque::new();
        v.extend(&[1, 2, 3]);
        assert_eq!(v.pop_front().unwrap(), 1);
        assert_eq!(v.pop_front().unwrap(), 2);
        assert_eq!(*v.front().unwrap(), 3);

        v.push_back(4).unwrap();
        v.extend(&[5, 6]);
        assert_eq!(v.pop_front().unwrap(), 3);
        assert_eq!(v.pop_front().unwrap(), 4);
        assert_eq!(v.pop_front().unwrap(), 5);
        assert_eq!(v.pop_front().unwrap(), 6);
        assert!(v.pop_front().is_none());
    }

    #[test]
    #[should_panic]
    fn extend_panic() {
        let mut v: Deque<i32, 4> = Deque::new();
        // Is too many elements -> should panic
        v.extend(&[1, 2, 3, 4, 5]);
    }

    #[test]
    fn iter() {
        let mut v: Deque<i32, 4> = Deque::new();

        v.push_back(0).unwrap();
        v.push_back(1).unwrap();
        v.push_front(2).unwrap();
        v.push_front(3).unwrap();
        v.pop_back().unwrap();
        v.push_front(4).unwrap();

        let mut items = v.iter();

        assert_eq!(items.next(), Some(&4));
        assert_eq!(items.next(), Some(&3));
        assert_eq!(items.next(), Some(&2));
        assert_eq!(items.next(), Some(&0));
        assert_eq!(items.next(), None);
    }

    #[test]
    fn iter_mut() {
        let mut v: Deque<i32, 4> = Deque::new();

        v.push_back(0).unwrap();
        v.push_back(1).unwrap();
        v.push_front(2).unwrap();
        v.push_front(3).unwrap();
        v.pop_back().unwrap();
        v.push_front(4).unwrap();

        let mut items = v.iter_mut();

        assert_eq!(items.next(), Some(&mut 4));
        assert_eq!(items.next(), Some(&mut 3));
        assert_eq!(items.next(), Some(&mut 2));
        assert_eq!(items.next(), Some(&mut 0));
        assert_eq!(items.next(), None);
    }

    #[test]
    fn iter_move() {
        let mut v: Deque<i32, 4> = Deque::new();
        v.push_back(0).unwrap();
        v.push_back(1).unwrap();
        v.push_back(2).unwrap();
        v.push_back(3).unwrap();

        let mut items = v.into_iter();

        assert_eq!(items.next(), Some(0));
        assert_eq!(items.next(), Some(1));
        assert_eq!(items.next(), Some(2));
        assert_eq!(items.next(), Some(3));
        assert_eq!(items.next(), None);
    }

    #[test]
    fn iter_move_back() {
        let mut v: Deque<i32, 4> = Deque::new();

        v.push_back(0).unwrap();
        v.push_back(1).unwrap();
        v.push_back(2).unwrap();
        v.push_back(3).unwrap();

        let mut items = v.into_iter();
        assert_eq!(items.next_back(), Some(3));
        assert_eq!(items.next_back(), Some(2));
        assert_eq!(items.next_back(), Some(1));
        assert_eq!(items.next_back(), Some(0));
        assert_eq!(items.next_back(), None);
    }

    #[test]
    fn iter_move_len() {
        let mut v: Deque<i32, 3> = Deque::new();

        v.push_back(0).unwrap();
        v.push_back(1).unwrap();
        v.push_back(2).unwrap();

        let mut items = v.into_iter();
        assert_eq!(items.len(), 3);
        let _ = items.next();
        assert_eq!(items.len(), 2);
        let _ = items.next_back();
        assert_eq!(items.len(), 1);
        let _ = items.next();
        assert_eq!(items.len(), 0);
    }

    #[test]
    fn iter_move_drop() {
        droppable!();

        {
            let mut deque: Deque<Droppable, 2> = Deque::new();
            deque.push_back(Droppable::new()).ok().unwrap();
            deque.push_back(Droppable::new()).ok().unwrap();
            let mut items = deque.into_iter();
            // Move all
            let _ = items.next();
            let _ = items.next();
        }

        assert_eq!(Droppable::count(), 0);

        {
            let mut deque: Deque<Droppable, 2> = Deque::new();
            deque.push_back(Droppable::new()).ok().unwrap();
            deque.push_back(Droppable::new()).ok().unwrap();
            let _items = deque.into_iter();
            // Move none
        }

        assert_eq!(Droppable::count(), 0);

        {
            let mut deque: Deque<Droppable, 2> = Deque::new();
            deque.push_back(Droppable::new()).ok().unwrap();
            deque.push_back(Droppable::new()).ok().unwrap();
            let mut items = deque.into_iter();
            let _ = items.next(); // Move partly
        }

        assert_eq!(Droppable::count(), 0);
    }

    #[test]
    fn push_and_pop() {
        let mut q: Deque<i32, 4> = Deque::new();
        assert_eq!(q.len(), 0);

        assert_eq!(q.pop_front(), None);
        assert_eq!(q.pop_back(), None);
        assert_eq!(q.len(), 0);

        q.push_back(0).unwrap();
        assert_eq!(q.len(), 1);

        assert_eq!(q.pop_back(), Some(0));
        assert_eq!(q.len(), 0);

        q.push_back(0).unwrap();
        q.push_back(1).unwrap();
        q.push_front(2).unwrap();
        q.push_front(3).unwrap();
        assert_eq!(q.len(), 4);

        // deque contains: 3 2 0 1
        assert_eq!(q.pop_front(), Some(3));
        assert_eq!(q.len(), 3);
        assert_eq!(q.pop_front(), Some(2));
        assert_eq!(q.len(), 2);
        assert_eq!(q.pop_back(), Some(1));
        assert_eq!(q.len(), 1);
        assert_eq!(q.pop_front(), Some(0));
        assert_eq!(q.len(), 0);

        // deque is now empty
        assert_eq!(q.pop_front(), None);
        assert_eq!(q.pop_back(), None);
        assert_eq!(q.len(), 0);
    }

    #[test]
    fn as_slices() {
        let mut q: Deque<i32, 4> = Deque::new();
        assert_eq!(q.len(), 0);

        q.push_back(0).unwrap();
        q.push_back(1).unwrap();
        q.push_back(2).unwrap();
        q.push_back(3).unwrap();
        assert_eq!(q.as_slices(), (&[0, 1, 2, 3][..], &[][..]));

        q.pop_front().unwrap();
        assert_eq!(q.as_slices(), (&[1, 2, 3][..], &[][..]));

        q.push_back(4).unwrap();
        assert_eq!(q.as_slices(), (&[1, 2, 3][..], &[4][..]));
    }

    #[test]
    fn clear() {
        let mut q: Deque<i32, 4> = Deque::new();
        assert_eq!(q.len(), 0);

        q.push_back(0).unwrap();
        q.push_back(1).unwrap();
        q.push_back(2).unwrap();
        q.push_back(3).unwrap();
        assert_eq!(q.len(), 4);

        q.clear();
        assert_eq!(q.len(), 0);

        q.push_back(0).unwrap();
        assert_eq!(q.len(), 1);
    }

    #[test]
    fn make_contiguous() {
        let mut q: Deque<i32, 4> = Deque::new();
        assert_eq!(q.len(), 0);

        q.push_back(0).unwrap();
        q.push_back(1).unwrap();
        q.push_back(2).unwrap();
        q.push_back(3).unwrap();

        // Deque contains: 0, 1, 2, 3
        assert_eq!(q.pop_front(), Some(0));
        assert_eq!(q.pop_front(), Some(1));

        // Deque contains: ., ., 2, 3
        q.push_back(4).unwrap();

        // Deque contains: 4, ., 2, 3
        assert_eq!(q.as_slices(), ([2, 3].as_slice(), [4].as_slice()));

        assert_eq!(q.make_contiguous(), &[2, 3, 4]);

        // Deque contains: ., 2, 3, 4
        assert_eq!(q.as_slices(), ([2, 3, 4].as_slice(), [].as_slice()));

        assert_eq!(q.pop_front(), Some(2));
        assert_eq!(q.pop_front(), Some(3));
        q.push_back(5).unwrap();
        q.push_back(6).unwrap();

        // Deque contains: 5, 6, ., 4
        assert_eq!(q.as_slices(), ([4].as_slice(), [5, 6].as_slice()));

        assert_eq!(q.make_contiguous(), &[4, 5, 6]);

        // Deque contains: 4, 5, 6, .
        assert_eq!(q.as_slices(), ([4, 5, 6].as_slice(), [].as_slice()));

        assert_eq!(q.pop_front(), Some(4));
        q.push_back(7).unwrap();
        q.push_back(8).unwrap();

        // Deque contains: 8, 5, 6, 7
        assert_eq!(q.as_slices(), ([5, 6, 7].as_slice(), [8].as_slice()));

        assert_eq!(q.make_contiguous(), &[5, 6, 7, 8]);

        // Deque contains: 5, 6, 7, 8
        assert_eq!(q.as_slices(), ([5, 6, 7, 8].as_slice(), [].as_slice()));
    }

    #[test]
    fn get() {
        let mut q: Deque<i32, 4> = Deque::new();
        assert_eq!(q.get(0), None);

        q.push_back(0).unwrap();
        assert_eq!(q.get(0), Some(&0));
        assert_eq!(q.get(1), None);

        q.push_back(1).unwrap();
        assert_eq!(q.get(0), Some(&0));
        assert_eq!(q.get(1), Some(&1));
        assert_eq!(q.get(2), None);

        q.pop_front().unwrap();
        assert_eq!(q.get(0), Some(&1));
        assert_eq!(q.get(1), None);

        q.push_back(2).unwrap();
        q.push_back(3).unwrap();
        q.push_back(4).unwrap();
        assert_eq!(q.get(0), Some(&1));
        assert_eq!(q.get(1), Some(&2));
        assert_eq!(q.get(2), Some(&3));
        assert_eq!(q.get(3), Some(&4));
    }

    #[test]
    fn get_mut() {
        let mut q: Deque<i32, 4> = Deque::new();
        assert_eq!(q.get(0), None);

        q.push_back(0).unwrap();
        assert_eq!(q.get_mut(0), Some(&mut 0));
        assert_eq!(q.get_mut(1), None);

        q.push_back(1).unwrap();
        assert_eq!(q.get_mut(0), Some(&mut 0));
        assert_eq!(q.get_mut(1), Some(&mut 1));
        assert_eq!(q.get_mut(2), None);
        *q.get_mut(0).unwrap() = 42;
        *q.get_mut(1).unwrap() = 43;

        assert_eq!(q.pop_front(), Some(42));
        assert_eq!(q.pop_front(), Some(43));
        assert_eq!(q.pop_front(), None);
    }

    #[test]
    fn swap() {
        let mut q: Deque<i32, 4> = Deque::new();
        q.push_back(40).unwrap();
        q.push_back(41).unwrap();
        q.push_back(42).unwrap();
        q.pop_front().unwrap();
        q.push_back(43).unwrap();
        assert_eq!(*q.get(0).unwrap(), 41);
        assert_eq!(*q.get(1).unwrap(), 42);
        assert_eq!(*q.get(2).unwrap(), 43);

        q.swap(0, 1);
        assert_eq!(*q.get(0).unwrap(), 42);
        assert_eq!(*q.get(1).unwrap(), 41);
        assert_eq!(*q.get(2).unwrap(), 43);

        q.swap(1, 2);
        assert_eq!(*q.get(0).unwrap(), 42);
        assert_eq!(*q.get(1).unwrap(), 43);
        assert_eq!(*q.get(2).unwrap(), 41);

        q.swap(1, 1);
        assert_eq!(*q.get(0).unwrap(), 42);
        assert_eq!(*q.get(1).unwrap(), 43);
        assert_eq!(*q.get(2).unwrap(), 41);
    }

    #[test]
    fn swap_remove_front() {
        let mut q: Deque<i32, 4> = Deque::new();
        q.push_back(40).unwrap();
        q.push_back(41).unwrap();
        q.push_back(42).unwrap();
        q.push_back(43).unwrap();

        assert_eq!(q.swap_remove_front(2), Some(42));
        assert_eq!(q.swap_remove_front(1), Some(40));
        assert_eq!(q.swap_remove_front(0), Some(41));
        assert_eq!(q.swap_remove_front(1), None);
        assert_eq!(q.swap_remove_front(4), None);
        assert_eq!(q.swap_remove_front(6), None);
        assert_eq!(q.swap_remove_front(0), Some(43));
    }

    #[test]
    fn swap_remove_back() {
        let mut q: Deque<i32, 4> = Deque::new();
        q.push_back(40).unwrap();
        q.push_back(41).unwrap();
        q.push_back(42).unwrap();
        q.push_back(43).unwrap();
        q.pop_front().unwrap();
        q.push_back(44).unwrap();

        assert_eq!(q.swap_remove_back(1), Some(42));
        assert_eq!(q.swap_remove_front(1), Some(44));
        assert_eq!(q.swap_remove_front(0), Some(41));
        assert_eq!(q.swap_remove_front(1), None);
        assert_eq!(q.swap_remove_front(4), None);
        assert_eq!(q.swap_remove_front(6), None);
        assert_eq!(q.swap_remove_front(0), Some(43));
    }

    #[test]
    #[should_panic = "i < len"]
    fn swap_i_out_of_bounds() {
        let mut q: Deque<i32, 4> = Deque::new();
        q.push_back(40).unwrap();
        q.push_back(41).unwrap();
        q.push_back(42).unwrap();
        q.pop_front().unwrap();
        q.swap(2, 0);
    }

    #[test]
    #[should_panic = "j < len"]
    fn swap_j_out_of_bounds() {
        let mut q: Deque<i32, 4> = Deque::new();
        q.push_back(40).unwrap();
        q.push_back(41).unwrap();
        q.push_back(42).unwrap();
        q.pop_front().unwrap();
        q.swap(0, 2);
    }

    #[test]
    fn equality() {
        let mut a: Deque<i32, 7> = Deque::new();
        let mut b: Deque<i32, 7> = Deque::new();

        assert_eq!(a, b);

        a.push_back(1).unwrap();
        a.push_back(2).unwrap();
        a.push_back(3).unwrap();

        assert_ne!(a, b);

        b.push_back(1).unwrap();
        b.push_back(2).unwrap();
        b.push_back(3).unwrap();

        assert_eq!(a, b);

        a.push_back(1).unwrap();
        a.push_back(2).unwrap();
        a.push_back(3).unwrap();

        assert_ne!(a, b);

        b.push_front(3).unwrap();
        b.push_front(2).unwrap();
        b.push_front(1).unwrap();

        assert_eq!(a, b);

        a.push_back(4).unwrap();
        b.push_back(4).unwrap();

        assert_eq!(a, b);

        a.clear();
        b.clear();

        a.push_back(1).unwrap();
        a.push_back(2).unwrap();
        a.push_back(3).unwrap();
        a.push_front(3).unwrap();
        a.push_front(2).unwrap();
        a.push_front(1).unwrap();

        b.push_back(2).unwrap();
        b.push_back(3).unwrap();
        b.push_back(1).unwrap();
        b.push_back(2).unwrap();
        b.push_back(3).unwrap();
        b.push_front(1).unwrap();

        assert_eq!(a, b);
    }

    #[test]
    fn try_from_array() {
        // Array is too big error.
        assert!(matches!(
            Deque::<u8, 3>::try_from([1, 2, 3, 4]),
            Err((CapacityError, [1, 2, 3, 4]))
        ));

        // Array is at limit.
        let deq1 = Deque::<u8, 3>::try_from([1, 2, 3]).unwrap();
        let mut deq2 = Deque::<u8, 3>::new();
        deq2.push_back(1).unwrap();
        deq2.push_back(2).unwrap();
        deq2.push_back(3).unwrap();
        assert!(deq1.is_full());
        assert_eq!(deq1, deq2);

        // Array is under limit.
        let deq1 = Deque::<u8, 8>::try_from([1, 2, 3, 4]).unwrap();
        let mut deq2 = Deque::<u8, 8>::new();
        deq2.push_back(1).unwrap();
        deq2.push_back(2).unwrap();
        deq2.push_back(3).unwrap();
        deq2.push_back(4).unwrap();

        assert!(!deq1.is_full());
        assert_eq!(deq1, deq2);
    }

    #[test]
    fn try_from_array_with_zst() {
        #[derive(Debug, PartialEq, Copy, Clone)]
        struct ZeroSizedType;

        // Test with ZST (zero-sized type)
        let deq1 =
            Deque::<ZeroSizedType, 5>::try_from([ZeroSizedType, ZeroSizedType, ZeroSizedType])
                .unwrap();
        let mut deq2 = Deque::<ZeroSizedType, 5>::new();
        deq2.push_back(ZeroSizedType).unwrap();
        deq2.push_back(ZeroSizedType).unwrap();
        deq2.push_back(ZeroSizedType).unwrap();

        assert_eq!(deq1, deq2);
        assert_eq!(deq1.len(), 3);
    }

    #[test]
    fn try_from_array_drop() {
        droppable!();

        // Array is over limit.
        {
            let _result = Deque::<Droppable, 2>::try_from([
                Droppable::new(),
                Droppable::new(),
                Droppable::new(),
            ]);
        }

        assert_eq!(Droppable::count(), 0);

        // Array is at limit.
        {
            let _result = Deque::<Droppable, 3>::try_from([
                Droppable::new(),
                Droppable::new(),
                Droppable::new(),
            ]);
        }

        assert_eq!(Droppable::count(), 0);

        // Array is under limit.
        {
            let _result = Deque::<Droppable, 4>::try_from([
                Droppable::new(),
                Droppable::new(),
                Droppable::new(),
            ]);
        }

        assert_eq!(Droppable::count(), 0);
    }

    #[test]
    #[cfg(feature = "zeroize")]
    fn test_deque_zeroize() {
        use zeroize::Zeroize;

        let mut deque = Deque::<u8, 16>::new();

        for i in 1..=8 {
            deque.push_back(i).unwrap();
        }
        for i in 9..=16 {
            deque.push_front(i).unwrap();
        }

        assert_eq!(deque.len(), 16);
        assert_eq!(deque.front(), Some(&16));
        assert_eq!(deque.back(), Some(&8));

        // zeroized using Vec's implementation
        deque.zeroize();

        assert_eq!(deque.len(), 0);
        assert!(deque.is_empty());
    }

    // Checking that no invalid destructors are called with empty Deques
    #[test]
    fn truncate_empty() {
        droppable!();

        const LEN: usize = 1;
        let mut tester: Deque<_, LEN> = Deque::new();

        // Truncate to 0 from 0
        tester.truncate(0);
        assert_eq!(tester.len(), 0);
        assert_eq!(Droppable::count(), 0);

        // Truncate to 123 from 0 (thus clamping back down to 0)
        tester.truncate(123);
        assert_eq!(tester.len(), 0);
        assert_eq!(Droppable::count(), 0);

        // Ensure state is still valid by pushing one element in and then truncating again
        assert!(tester.push_front(Droppable::new()).is_ok());
        assert_eq!(tester.len(), 1);
        assert_eq!(Droppable::count(), 1);

        // Truncate to 0 from 1
        tester.truncate(0);
        assert_eq!(tester.len(), 0);
        assert_eq!(Droppable::count(), 0);
    }

    // Testing truncation with contiguous Deques
    #[test]
    fn truncate_contiguous() {
        droppable!();

        fn slice_lengths<T>(slices: (&[T], &[T])) -> (usize, usize) {
            let (a, b) = slices;
            (a.len(), b.len())
        }

        const LEN: usize = 20;
        let mut tester: Deque<_, LEN> = Deque::new();

        // Filling from front.
        for _ in 0..5 {
            assert!(tester.push_front(Droppable::new()).is_ok());
        }

        // Truncating past the elements present, no change.
        tester.truncate(10);
        let lens = slice_lengths(tester.as_slices());
        assert_eq!(lens, (5, 0));
        assert_eq!(Droppable::count(), 5);

        // Truncating equal to elements present, no change.
        tester.truncate(5);
        let lens = slice_lengths(tester.as_slices());
        assert_eq!(lens, (5, 0));
        assert_eq!(Droppable::count(), 5);

        // Truncating to empty.
        tester.truncate(0);
        assert_eq!(tester.len(), 0);
        assert_eq!(Droppable::count(), 0);

        // Refill from front.
        for _ in 0..5 {
            assert!(tester.push_front(Droppable::new()).is_ok());
        }

        let lens = slice_lengths(tester.as_slices());
        assert_eq!(lens, (5, 0));
        assert_eq!(Droppable::count(), 5);

        // Truncate into the middle of elements.
        tester.truncate(3);
        let lens = slice_lengths(tester.as_slices());
        assert_eq!(lens, (3, 0));
        assert_eq!(Droppable::count(), 3);

        // Truncating to empty.
        tester.truncate(0);
        assert_eq!(tester.len(), 0);
        assert_eq!(Droppable::count(), 0);

        // Resetting cursors.
        tester.clear();

        // Filling from back...
        for _ in 0..5 {
            assert!(tester.push_back(Droppable::new()).is_ok());
        }

        tester.truncate(10);
        let lens = slice_lengths(tester.as_slices());
        assert_eq!(lens, (5, 0));
        assert_eq!(Droppable::count(), 5);

        tester.truncate(5);
        let lens = slice_lengths(tester.as_slices());
        assert_eq!(lens, (5, 0));
        assert_eq!(Droppable::count(), 5);

        tester.truncate(0);
        assert_eq!(tester.len(), 0);
        assert_eq!(Droppable::count(), 0);

        for _ in 0..5 {
            assert!(tester.push_back(Droppable::new()).is_ok());
        }

        let lens = slice_lengths(tester.as_slices());
        assert_eq!(lens, (5, 0));
        assert_eq!(Droppable::count(), 5);

        tester.truncate(3);
        let lens = slice_lengths(tester.as_slices());
        assert_eq!(lens, (3, 0));
        assert_eq!(Droppable::count(), 3);

        tester.truncate(0);
        assert_eq!(tester.len(), 0);
        assert_eq!(Droppable::count(), 0);
    }

    // Testing truncation with non-contiguous Deques
    #[test]
    fn truncate_non_contiguous() {
        const LEN: usize = 20;
        let mut tester: Deque<u8, LEN> = Deque::new();

        // Filling non-contiguously.
        //
        // Expecting [3, 2, 1, 1, 2, 3]
        for x in 1..=3 {
            assert!(tester.push_front(x).is_ok());
        }
        for y in 1..=3 {
            assert!(tester.push_back(y).is_ok());
        }

        // Truncating past the elements present, no change.
        tester.truncate(10);
        assert_eq!(tester.as_slices(), (&[3, 2, 1][..], &[1, 2, 3][..]));
        println!("{} {}", tester.front, tester.back);
        // Truncating equal to elements present, no change.
        tester.truncate(6);
        assert_eq!(tester.as_slices(), (&[3, 2, 1][..], &[1, 2, 3][..]));

        // Truncating to empty.
        tester.truncate(0);
        assert_eq!(tester.as_slices(), (&[][..], &[][..]));

        // Resetting cursors.
        tester.clear();

        // Refilling.
        for x in 1..=3 {
            assert!(tester.push_front(x).is_ok());
        }
        for y in 1..=3 {
            assert!(tester.push_back(y).is_ok());
        }

        assert_eq!(tester.as_slices(), (&[3, 2, 1][..], &[1, 2, 3][..]));

        // Truncating only part of back, retaining front and part of back.
        tester.truncate(5);
        assert_eq!(tester.as_slices(), (&[3, 2, 1][..], &[1, 2][..]));

        // Replacing the truncated element.
        assert!(tester.push_back(3).is_ok());
        assert_eq!(tester.as_slices(), (&[3, 2, 1][..], &[1, 2, 3][..]));

        // Truncating away all of back's contents, but retaining all of front's contents.
        tester.truncate(3);
        assert_eq!(tester.as_slices(), (&[3, 2, 1][..], &[][..]));

        // Replacing the truncated elements.
        for y in 1..=3 {
            assert!(tester.push_back(y).is_ok());
        }
        assert_eq!(tester.as_slices(), (&[3, 2, 1][..], &[1, 2, 3][..]));

        // Truncating into front, thus also dropping all of back.
        tester.truncate(2);
        assert_eq!(tester.as_slices(), (&[3, 2][..], &[][..]));

        // Truncating to empty.
        tester.truncate(0);
        assert_eq!(tester.as_slices(), (&[][..], &[][..]));

        // Should remain empty.
        tester.truncate(123);
        assert_eq!(tester.as_slices(), (&[][..], &[][..]));
    }

    // Tests that each element's destructor is called when being truncated.
    #[test]
    fn truncate_drop_count() {
        droppable!();

        const LEN: usize = 20;
        const TRUNC: usize = 3;
        for push_front_amt in 0..=LEN {
            let mut tester: Deque<_, LEN> = Deque::new();
            for index in 0..LEN {
                if index < push_front_amt {
                    assert!(
                        tester.push_front(Droppable::new()).is_ok(),
                        "deque must have room for all {LEN} entries"
                    );
                } else {
                    assert!(
                        tester.push_back(Droppable::new()).is_ok(),
                        "deque must have room for all {LEN} entries"
                    );
                }
            }

            assert_eq!(Droppable::count(), LEN as i32);

            tester.truncate(TRUNC);
            assert_eq!(tester.len(), TRUNC);
            assert_eq!(Droppable::count(), TRUNC as i32);

            tester.truncate(0);
            assert_eq!(tester.len(), 0);
            assert_eq!(Droppable::count(), 0);
        }
    }

    #[test]
    fn retain() {
        droppable!();

        const LEN: usize = 20;
        for push_front_amt in 0..=LEN {
            let mut tester: Deque<_, LEN> = Deque::new();
            for index in 0..LEN {
                if index < push_front_amt {
                    assert!(tester.push_front((index, Droppable::new())).is_ok());
                } else {
                    assert!(tester.push_back((index, Droppable::new())).is_ok());
                }
            }
            assert_eq!(Droppable::count(), LEN as i32);

            tester.retain(|(x, _)| *x >= 10);

            assert_eq!(tester.len(), 10);
            assert_eq!(Droppable::count(), 10);
        }
    }
}
