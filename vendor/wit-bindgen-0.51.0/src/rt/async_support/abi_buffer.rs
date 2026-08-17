use crate::rt::Cleanup;
use crate::rt::async_support::StreamOps;
use std::alloc::Layout;
use std::mem::{self, MaybeUninit};
use std::ptr;
use std::vec::Vec;

/// A helper structure used with a stream to handle the canonical ABI
/// representation of lists and track partial writes.
///
/// This structure is returned whenever a write to a stream completes. This
/// keeps track of the original buffer used to perform a write (`Vec<T>`) and
/// additionally tracks any partial writes. Writes can then be resumed with
/// this buffer again or the partial write can be converted back to `Vec<T>` to
/// get access to the remaining values.
///
/// This value is created through the [`StreamWrite`](super::StreamWrite)
/// future's return value.
pub struct AbiBuffer<O: StreamOps> {
    rust_storage: Vec<MaybeUninit<O::Payload>>,
    ops: O,
    alloc: Option<Cleanup>,
    cursor: usize,
}

impl<O: StreamOps> AbiBuffer<O> {
    pub(crate) fn new(mut vec: Vec<O::Payload>, mut ops: O) -> AbiBuffer<O> {
        // SAFETY: We're converting `Vec<T>` to `Vec<MaybeUninit<T>>`, which
        // should be safe.
        let rust_storage = unsafe {
            let ptr = vec.as_mut_ptr();
            let len = vec.len();
            let cap = vec.capacity();
            mem::forget(vec);
            Vec::<MaybeUninit<O::Payload>>::from_raw_parts(ptr.cast(), len, cap)
        };

        // If `lower` is provided then the canonical ABI format is different
        // from the native format, so all items are converted at this time.
        //
        // Note that this is probably pretty inefficient for "big" use cases
        // but it's hoped that "big" use cases are using `u8` and therefore
        // skip this entirely.
        let alloc = if ops.native_abi_matches_canonical_abi() {
            None
        } else {
            let elem_layout = ops.elem_layout();
            let layout = Layout::from_size_align(
                elem_layout.size() * rust_storage.len(),
                elem_layout.align(),
            )
            .unwrap();
            let (mut ptr, cleanup) = Cleanup::new(layout);
            // SAFETY: All items in `rust_storage` are already initialized so
            // it should be safe to read them and move ownership into the
            // canonical ABI format.
            unsafe {
                for item in rust_storage.iter() {
                    let item = item.assume_init_read();
                    ops.lower(item, ptr);
                    ptr = ptr.add(elem_layout.size());
                }
            }
            cleanup
        };
        AbiBuffer {
            rust_storage,
            alloc,
            ops,
            cursor: 0,
        }
    }

    /// Returns the canonical ABI pointer/length to pass off to a write
    /// operation.
    pub(crate) fn abi_ptr_and_len(&self) -> (*const u8, usize) {
        // If there's no `lower` operation then it means that `T`'s layout is
        // the same in the canonical ABI so it can be used as-is. In this
        // situation the list would have been un-tampered with above.
        if self.ops.native_abi_matches_canonical_abi() {
            // SAFETY: this should be in-bounds, so it should be safe.
            let ptr = unsafe { self.rust_storage.as_ptr().add(self.cursor).cast() };
            let len = self.rust_storage.len() - self.cursor;
            return (ptr, len.try_into().unwrap());
        }

        // Othereise when `lower` is present that means that `self.alloc` has
        // the ABI pointer we should pass along.
        let ptr = self
            .alloc
            .as_ref()
            .map(|c| c.ptr.as_ptr())
            .unwrap_or(ptr::null_mut());
        (
            // SAFETY: this should be in-bounds, so it should be safe.
            unsafe { ptr.add(self.cursor * self.ops.elem_layout().size()) },
            self.rust_storage.len() - self.cursor,
        )
    }

    /// Converts this `AbiBuffer<T>` back into a `Vec<T>`
    ///
    /// This commit consumes this buffer and yields back unwritten values as a
    /// `Vec<T>`. The remaining items in `Vec<T>` have not yet been written and
    /// all written items have been removed from the front of the list.
    ///
    /// Note that the backing storage of the returned `Vec<T>` has not changed
    /// from whe this buffer was created.
    ///
    /// Also note that this can be an expensive operation if a partial write
    /// occurred as this will involve shifting items from the end of the vector
    /// to the start of the vector.
    pub fn into_vec(mut self) -> Vec<O::Payload> {
        self.take_vec()
    }

    /// Returns the number of items remaining in this buffer.
    pub fn remaining(&self) -> usize {
        self.rust_storage.len() - self.cursor
    }

    /// Advances this buffer by `amt` items.
    ///
    /// This signals that `amt` items are no longer going to be yielded from
    /// `abi_ptr_and_len`. Additionally this will perform any deallocation
    /// necessary for the starting `amt` items in this list.
    pub(crate) fn advance(&mut self, amt: usize) {
        assert!(amt + self.cursor <= self.rust_storage.len());
        if !self.ops.contains_lists() {
            self.cursor += amt;
            return;
        }
        let (mut ptr, len) = self.abi_ptr_and_len();
        assert!(amt <= len);
        for _ in 0..amt {
            // SAFETY: we're managing the pointer passed to `dealloc_lists` and
            // it was initialized with a `lower`, and then the pointer
            // arithmetic should all be in-bounds.
            unsafe {
                self.ops.dealloc_lists(ptr.cast_mut());
                ptr = ptr.add(self.ops.elem_layout().size());
            }
        }
        self.cursor += amt;
    }

    fn take_vec(&mut self) -> Vec<O::Payload> {
        // First, if necessary, convert remaining values within `self.alloc`
        // back into `self.rust_storage`. This is necessary when a lift
        // operation is available meaning that the representation of `T` is
        // different in the canonical ABI.
        //
        // Note that when `lift` is provided then when this original
        // `AbiBuffer` was created it moved ownership of all values from the
        // original vector into the `alloc` value. This is the reverse
        // operation, moving all the values back into the vector.
        if !self.ops.native_abi_matches_canonical_abi() {
            let (mut ptr, mut len) = self.abi_ptr_and_len();
            // SAFETY: this should be safe as `lift` is operating on values that
            // were initialized with a previous `lower`, and the pointer
            // arithmetic here should all be in-bounds.
            unsafe {
                for dst in self.rust_storage[self.cursor..].iter_mut() {
                    dst.write(self.ops.lift(ptr.cast_mut()));
                    ptr = ptr.add(self.ops.elem_layout().size());
                    len -= 1;
                }
                assert_eq!(len, 0);
            }
        }

        // Next extract the rust storage and zero out this struct's fields.
        // This is also the location where a "shift" happens to remove items
        // from the beginning of the returned vector as those have already been
        // transferred somewhere else.
        let mut storage = mem::take(&mut self.rust_storage);
        storage.drain(..self.cursor);
        self.cursor = 0;
        self.alloc = None;

        // SAFETY: we're casting `Vec<MaybeUninit<T>>` here to `Vec<T>`. The
        // elements were either always initialized (`lift` is `None`) or we just
        // re-initialized them above from `self.alloc`.
        unsafe {
            let ptr = storage.as_mut_ptr();
            let len = storage.len();
            let cap = storage.capacity();
            mem::forget(storage);
            Vec::<O::Payload>::from_raw_parts(ptr.cast(), len, cap)
        }
    }
}

impl<O> Drop for AbiBuffer<O>
where
    O: StreamOps,
{
    fn drop(&mut self) {
        let _ = self.take_vec();
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::rt::async_support::StreamVtable;
    use std::sync::atomic::{AtomicUsize, Ordering::Relaxed};
    use std::vec;

    extern "C" fn cancel(_: u32) -> u32 {
        todo!()
    }
    extern "C" fn drop(_: u32) {
        todo!()
    }
    extern "C" fn new() -> u64 {
        todo!()
    }
    extern "C" fn start_read(_: u32, _: *mut u8, _: usize) -> u32 {
        todo!()
    }
    extern "C" fn start_write(_: u32, _: *const u8, _: usize) -> u32 {
        todo!()
    }

    static BLANK: StreamVtable<u8> = StreamVtable {
        cancel_read: cancel,
        cancel_write: cancel,
        drop_readable: drop,
        drop_writable: drop,
        dealloc_lists: None,
        lift: None,
        lower: None,
        layout: unsafe { Layout::from_size_align_unchecked(1, 1) },
        new,
        start_read,
        start_write,
    };

    #[test]
    fn blank_advance_to_end() {
        let mut buffer = AbiBuffer::new(vec![1, 2, 3, 4], &BLANK);
        assert_eq!(buffer.remaining(), 4);
        buffer.advance(1);
        assert_eq!(buffer.remaining(), 3);
        buffer.advance(2);
        assert_eq!(buffer.remaining(), 1);
        buffer.advance(1);
        assert_eq!(buffer.remaining(), 0);
        assert_eq!(buffer.into_vec(), []);
    }

    #[test]
    fn blank_advance_partial() {
        let buffer = AbiBuffer::new(vec![1, 2, 3, 4], &BLANK);
        assert_eq!(buffer.into_vec(), [1, 2, 3, 4]);
        let mut buffer = AbiBuffer::new(vec![1, 2, 3, 4], &BLANK);
        buffer.advance(1);
        assert_eq!(buffer.into_vec(), [2, 3, 4]);
        let mut buffer = AbiBuffer::new(vec![1, 2, 3, 4], &BLANK);
        buffer.advance(1);
        buffer.advance(2);
        assert_eq!(buffer.into_vec(), [4]);
    }

    #[test]
    fn blank_ptr_eq() {
        let mut buf = vec![1, 2, 3, 4];
        let ptr = buf.as_mut_ptr();
        let mut buffer = AbiBuffer::new(buf, &BLANK);
        let (a, b) = buffer.abi_ptr_and_len();
        assert_eq!(a, ptr);
        assert_eq!(b, 4);
        unsafe {
            assert_eq!(std::slice::from_raw_parts(a, b), [1, 2, 3, 4]);
        }

        buffer.advance(1);
        let (a, b) = buffer.abi_ptr_and_len();
        assert_eq!(a, ptr.wrapping_add(1));
        assert_eq!(b, 3);
        unsafe {
            assert_eq!(std::slice::from_raw_parts(a, b), [2, 3, 4]);
        }

        buffer.advance(2);
        let (a, b) = buffer.abi_ptr_and_len();
        assert_eq!(a, ptr.wrapping_add(3));
        assert_eq!(b, 1);
        unsafe {
            assert_eq!(std::slice::from_raw_parts(a, b), [4]);
        }

        let ret = buffer.into_vec();
        assert_eq!(ret, [4]);
        assert_eq!(ret.as_ptr(), ptr);
    }

    #[derive(PartialEq, Eq, Debug)]
    struct B(u8);

    static OP: StreamVtable<B> = StreamVtable {
        cancel_read: cancel,
        cancel_write: cancel,
        drop_readable: drop,
        drop_writable: drop,
        dealloc_lists: Some(|_ptr| {}),
        lift: Some(|ptr| unsafe { B(*ptr - 1) }),
        lower: Some(|b, ptr| unsafe {
            *ptr = b.0 + 1;
        }),
        layout: unsafe { Layout::from_size_align_unchecked(1, 1) },
        new,
        start_read,
        start_write,
    };

    #[test]
    fn op_advance_to_end() {
        let mut buffer = AbiBuffer::new(vec![B(1), B(2), B(3), B(4)], &OP);
        assert_eq!(buffer.remaining(), 4);
        buffer.advance(1);
        assert_eq!(buffer.remaining(), 3);
        buffer.advance(2);
        assert_eq!(buffer.remaining(), 1);
        buffer.advance(1);
        assert_eq!(buffer.remaining(), 0);
        assert_eq!(buffer.into_vec(), []);
    }

    #[test]
    fn op_advance_partial() {
        let buffer = AbiBuffer::new(vec![B(1), B(2), B(3), B(4)], &OP);
        assert_eq!(buffer.into_vec(), [B(1), B(2), B(3), B(4)]);
        let mut buffer = AbiBuffer::new(vec![B(1), B(2), B(3), B(4)], &OP);
        buffer.advance(1);
        assert_eq!(buffer.into_vec(), [B(2), B(3), B(4)]);
        let mut buffer = AbiBuffer::new(vec![B(1), B(2), B(3), B(4)], &OP);
        buffer.advance(1);
        buffer.advance(2);
        assert_eq!(buffer.into_vec(), [B(4)]);
    }

    #[test]
    fn op_ptrs() {
        let mut buf = vec![B(1), B(2), B(3), B(4)];
        let ptr = buf.as_mut_ptr().cast::<u8>();
        let mut buffer = AbiBuffer::new(buf, &OP);
        let (a, b) = buffer.abi_ptr_and_len();
        let base = a;
        assert_ne!(a, ptr);
        assert_eq!(b, 4);
        unsafe {
            assert_eq!(std::slice::from_raw_parts(a, b), [2, 3, 4, 5]);
        }

        buffer.advance(1);
        let (a, b) = buffer.abi_ptr_and_len();
        assert_ne!(a, ptr.wrapping_add(1));
        assert_eq!(a, base.wrapping_add(1));
        assert_eq!(b, 3);
        unsafe {
            assert_eq!(std::slice::from_raw_parts(a, b), [3, 4, 5]);
        }

        buffer.advance(2);
        let (a, b) = buffer.abi_ptr_and_len();
        assert_ne!(a, ptr.wrapping_add(3));
        assert_eq!(a, base.wrapping_add(3));
        assert_eq!(b, 1);
        unsafe {
            assert_eq!(std::slice::from_raw_parts(a, b), [5]);
        }

        let ret = buffer.into_vec();
        assert_eq!(ret, [B(4)]);
        assert_eq!(ret.as_ptr(), ptr.cast());
    }

    #[test]
    fn dealloc_lists() {
        static DEALLOCS: AtomicUsize = AtomicUsize::new(0);
        static OP: StreamVtable<B> = StreamVtable {
            cancel_read: cancel,
            cancel_write: cancel,
            drop_readable: drop,
            drop_writable: drop,
            dealloc_lists: Some(|ptr| {
                let prev = DEALLOCS.fetch_add(1, Relaxed);
                assert_eq!(unsafe { usize::from(*ptr) }, prev + 1);
            }),
            lift: Some(|ptr| unsafe { B(*ptr) }),
            lower: Some(|b, ptr| unsafe {
                *ptr = b.0;
            }),
            layout: unsafe { Layout::from_size_align_unchecked(1, 1) },
            new,
            start_read,
            start_write,
        };

        assert_eq!(DEALLOCS.load(Relaxed), 0);
        let buf = vec![B(1), B(2), B(3), B(4)];
        let mut buffer = AbiBuffer::new(buf, &OP);
        assert_eq!(DEALLOCS.load(Relaxed), 0);
        buffer.abi_ptr_and_len();
        assert_eq!(DEALLOCS.load(Relaxed), 0);

        buffer.advance(1);
        assert_eq!(DEALLOCS.load(Relaxed), 1);
        buffer.abi_ptr_and_len();
        assert_eq!(DEALLOCS.load(Relaxed), 1);
        buffer.advance(2);
        assert_eq!(DEALLOCS.load(Relaxed), 3);
        buffer.abi_ptr_and_len();
        assert_eq!(DEALLOCS.load(Relaxed), 3);
        buffer.into_vec();
        assert_eq!(DEALLOCS.load(Relaxed), 3);
    }
}
