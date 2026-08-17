use super::RealArray;
use core::marker::PhantomData;
use core::mem::MaybeUninit;

/// A Ring Buffer of items
pub trait RingBuf {
    /// The type of stored items inside the Ring Buffer
    type Item;

    /// Creates a new instance of the Ring Buffer
    fn new() -> Self;
    /// Creates a new instance of the Ring Buffer with the given capacity.
    /// `RingBuf` implementations are allowed to ignore the `capacity` hint and
    /// utilize their default capacity.
    fn with_capacity(cap: usize) -> Self;

    /// The capacity of the buffer
    fn capacity(&self) -> usize;
    /// The amount of stored items in the buffer
    fn len(&self) -> usize;
    /// Returns true if no item is stored inside the buffer.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }

    /// Returns true if there is enough space in the buffer to
    /// store another item.
    fn can_push(&self) -> bool;
    /// Stores the item at the end of the buffer.
    /// Panics if there is not enough free space.
    fn push(&mut self, item: Self::Item);
    /// Returns the oldest item inside the buffer.
    /// Panics if there is no available item.
    fn pop(&mut self) -> Self::Item;
}

/// An array-backed Ring Buffer
///
/// `A` is the type of the backing array. The backing array must be a real
/// array. In order to verify this it must satisfy the [`RealArray`] constraint.
/// In order to create a Ring Buffer backed by an array of 5 integer elements,
/// the following code can be utilized:
///
/// ```
/// use futures_intrusive::buffer::{ArrayBuf, RingBuf};
///
/// type Buffer5 = ArrayBuf<i32, [i32; 5]>;
/// let buffer = Buffer5::new();
/// ```
pub struct ArrayBuf<T, A>
where
    A: core::convert::AsMut<[T]> + core::convert::AsRef<[T]> + RealArray<T>,
{
    buffer: MaybeUninit<A>,
    size: usize,
    recv_idx: usize,
    send_idx: usize,
    _phantom: PhantomData<T>,
}

impl<T, A> core::fmt::Debug for ArrayBuf<T, A>
where
    A: core::convert::AsMut<[T]> + core::convert::AsRef<[T]> + RealArray<T>,
{
    fn fmt(
        &self,
        f: &mut core::fmt::Formatter,
    ) -> Result<(), core::fmt::Error> {
        f.debug_struct("ArrayBuf")
            .field("size", &self.size)
            .field("cap", &self.capacity())
            .finish()
    }
}

impl<T, A> ArrayBuf<T, A>
where
    A: core::convert::AsMut<[T]> + core::convert::AsRef<[T]> + RealArray<T>,
{
    fn next_idx(&mut self, last_idx: usize) -> usize {
        if last_idx + 1 == self.capacity() {
            return 0;
        }
        last_idx + 1
    }
}

impl<T, A> RingBuf for ArrayBuf<T, A>
where
    A: core::convert::AsMut<[T]> + core::convert::AsRef<[T]> + RealArray<T>,
{
    type Item = T;

    fn new() -> Self {
        ArrayBuf {
            buffer: MaybeUninit::uninit(),
            send_idx: 0,
            recv_idx: 0,
            size: 0,
            _phantom: PhantomData,
        }
    }

    fn with_capacity(_cap: usize) -> Self {
        // The fixed size array backed Ring Buffer doesn't support an adjustable
        // capacity. Therefore only the default capacity is utilized.
        Self::new()
    }

    #[inline]
    fn capacity(&self) -> usize {
        A::LEN
    }

    #[inline]
    fn len(&self) -> usize {
        self.size
    }

    #[inline]
    fn can_push(&self) -> bool {
        self.len() != self.capacity()
    }

    #[inline]
    fn push(&mut self, value: Self::Item) {
        assert!(self.can_push());
        // Safety: We asserted that there is available space for an item.
        // Therefore the memory address is valid.
        unsafe {
            let arr_ptr = self.buffer.as_mut_ptr() as *mut T;
            arr_ptr.add(self.send_idx).write(value);
        }
        self.send_idx = self.next_idx(self.send_idx);
        self.size += 1;
    }

    #[inline]
    fn pop(&mut self) -> Self::Item {
        assert!(self.size > 0);
        // Safety: We asserted that there is an element available, so it must
        // have been written before.
        let val = unsafe {
            let arr_ptr = self.buffer.as_mut_ptr() as *mut T;
            arr_ptr.add(self.recv_idx).read()
        };
        self.recv_idx = self.next_idx(self.recv_idx);
        self.size -= 1;
        val
    }
}

impl<T, A> Drop for ArrayBuf<T, A>
where
    A: core::convert::AsMut<[T]> + core::convert::AsRef<[T]> + RealArray<T>,
{
    fn drop(&mut self) {
        // Drop all elements which are still stored inside the buffer
        while self.size > 0 {
            // Safety: This drops only as many elements as have been written via
            // ptr::write and haven't read via ptr::read before
            unsafe {
                let arr_ptr = self.buffer.as_mut_ptr() as *mut T;
                arr_ptr.add(self.recv_idx).drop_in_place();
            }
            self.recv_idx = self.next_idx(self.recv_idx);
            self.size -= 1;
        }
    }
}

#[cfg(feature = "alloc")]
mod if_alloc {
    use super::*;
    use alloc::collections::VecDeque;

    /// A Ring Buffer which stores all items on the heap.
    ///
    /// The `FixedHeapBuf` will allocate its capacity ahead of time. This is good
    /// fit when you have a constant latency between two components.
    pub struct FixedHeapBuf<T> {
        buffer: VecDeque<T>,
        /// The capacity is stored extra, since VecDeque can allocate space for
        /// more elements than specified.
        cap: usize,
    }

    impl<T> core::fmt::Debug for FixedHeapBuf<T> {
        fn fmt(
            &self,
            f: &mut core::fmt::Formatter,
        ) -> Result<(), core::fmt::Error> {
            f.debug_struct("FixedHeapBuf")
                .field("size", &self.buffer.len())
                .field("cap", &self.cap)
                .finish()
        }
    }

    impl<T> RingBuf for FixedHeapBuf<T> {
        type Item = T;

        fn new() -> Self {
            FixedHeapBuf {
                buffer: VecDeque::new(),
                cap: 0,
            }
        }

        fn with_capacity(cap: usize) -> Self {
            FixedHeapBuf {
                buffer: VecDeque::with_capacity(cap),
                cap,
            }
        }

        #[inline]
        fn capacity(&self) -> usize {
            self.cap
        }

        #[inline]
        fn len(&self) -> usize {
            self.buffer.len()
        }

        #[inline]
        fn can_push(&self) -> bool {
            self.buffer.len() != self.cap
        }

        #[inline]
        fn push(&mut self, value: Self::Item) {
            assert!(self.can_push());
            self.buffer.push_back(value);
        }

        #[inline]
        fn pop(&mut self) -> Self::Item {
            assert!(self.buffer.len() > 0);
            self.buffer.pop_front().unwrap()
        }
    }

    /// A Ring Buffer which stores all items on the heap but grows dynamically.
    ///
    /// A `GrowingHeapBuf` does not allocate the capacity ahead of time, as
    /// opposed to the `FixedHeapBuf`. This makes it a good fit when you have
    /// unpredictable latency between two components, when you want to
    /// amortize your allocation costs or when you are using an external
    /// back-pressure mechanism.
    pub struct GrowingHeapBuf<T> {
        buffer: VecDeque<T>,
        /// The maximum number of elements in the buffer.
        limit: usize,
    }

    impl<T> core::fmt::Debug for GrowingHeapBuf<T> {
        fn fmt(
            &self,
            f: &mut core::fmt::Formatter,
        ) -> Result<(), core::fmt::Error> {
            f.debug_struct("GrowingHeapBuf")
                .field("size", &self.buffer.len())
                .field("limit", &self.limit)
                .finish()
        }
    }

    impl<T> RingBuf for GrowingHeapBuf<T> {
        type Item = T;

        fn new() -> Self {
            GrowingHeapBuf {
                buffer: VecDeque::new(),
                limit: 0,
            }
        }

        fn with_capacity(limit: usize) -> Self {
            GrowingHeapBuf {
                buffer: VecDeque::new(),
                limit,
            }
        }

        #[inline]
        fn capacity(&self) -> usize {
            self.limit
        }

        #[inline]
        fn len(&self) -> usize {
            self.buffer.len()
        }

        #[inline]
        fn can_push(&self) -> bool {
            self.buffer.len() != self.limit
        }

        #[inline]
        fn push(&mut self, value: Self::Item) {
            debug_assert!(self.can_push());
            self.buffer.push_back(value);
        }

        #[inline]
        fn pop(&mut self) -> Self::Item {
            debug_assert!(self.buffer.len() > 0);
            self.buffer.pop_front().unwrap()
        }
    }
}

#[cfg(feature = "alloc")]
pub use if_alloc::*;

#[cfg(test)]
#[cfg(feature = "alloc")]
mod tests {
    use super::*;
    use crate::buffer::ring_buffer::if_alloc::FixedHeapBuf;

    fn test_ring_buf<Buf: RingBuf<Item = u32>>(mut buf: Buf) {
        assert_eq!(5, buf.capacity());
        assert_eq!(0, buf.len());
        assert_eq!(true, buf.is_empty());
        assert_eq!(true, buf.can_push());

        buf.push(1);
        buf.push(2);
        buf.push(3);
        assert_eq!(5, buf.capacity());
        assert_eq!(3, buf.len());
        assert_eq!(false, buf.is_empty());
        assert_eq!(true, buf.can_push());

        assert_eq!(1, buf.pop());
        assert_eq!(2, buf.pop());
        assert_eq!(1, buf.len());
        assert_eq!(false, buf.is_empty());
        assert_eq!(3, buf.pop());
        assert_eq!(0, buf.len());
        assert_eq!(true, buf.is_empty());

        for (i, val) in [4, 5, 6, 7, 8].iter().enumerate() {
            buf.push(*val);
            assert_eq!(i + 1, buf.len());
            assert_eq!(i != 4, buf.can_push());
            assert_eq!(false, buf.is_empty());
        }

        for (i, val) in [4, 5, 6, 7, 8].iter().enumerate() {
            assert_eq!(*val, buf.pop());
            assert_eq!(4 - i, buf.len());
            assert_eq!(true, buf.can_push());
            assert_eq!(i == 4, buf.is_empty());
        }
    }

    #[test]
    fn test_array_ring_buf() {
        let buf = ArrayBuf::<u32, [u32; 5]>::new();
        test_ring_buf(buf);
    }

    #[test]
    fn test_heap_ring_buf() {
        let buf = FixedHeapBuf::<u32>::with_capacity(5);
        test_ring_buf(buf);
    }

    #[test]
    fn test_growing_ring_buf() {
        let buf = GrowingHeapBuf::<u32>::with_capacity(5);
        test_ring_buf(buf);
    }
}
