use super::super::plumbing::*;
use crate::SendPtr;
use std::marker::PhantomData;
use std::ptr;
use std::slice;

pub(super) struct CollectConsumer<'c, T: Send> {
    /// See `CollectResult` for explanation of why this is not a slice
    start: SendPtr<T>,
    len: usize,
    marker: PhantomData<&'c mut T>,
}

impl<T: Send> CollectConsumer<'_, T> {
    /// Create a collector for `len` items in the unused capacity of the vector.
    pub(super) fn appender(vec: &mut Vec<T>, len: usize) -> CollectConsumer<'_, T> {
        let start = vec.len();
        assert!(vec.capacity() - start >= len);

        // SAFETY: We already made sure to have the additional space allocated.
        // The pointer is derived from `Vec` directly, not through a `Deref`,
        // so it has provenance over the whole allocation.
        unsafe { CollectConsumer::new(vec.as_mut_ptr().add(start), len) }
    }
}

impl<'c, T: Send + 'c> CollectConsumer<'c, T> {
    /// The target memory is considered uninitialized, and will be
    /// overwritten without reading or dropping existing values.
    unsafe fn new(start: *mut T, len: usize) -> Self {
        CollectConsumer {
            start: SendPtr(start),
            len,
            marker: PhantomData,
        }
    }
}

/// CollectResult represents an initialized part of the target slice.
///
/// This is a proxy owner of the elements in the slice; when it drops,
/// the elements will be dropped, unless its ownership is released before then.
#[must_use]
pub(super) struct CollectResult<'c, T> {
    /// This pointer and length has the same representation as a slice,
    /// but retains the provenance of the entire array so that we can merge
    /// these regions together in `CollectReducer`.
    start: SendPtr<T>,
    total_len: usize,
    /// The current initialized length after `start`
    initialized_len: usize,
    /// Lifetime invariance guarantees that the data flows from consumer to result,
    /// especially for the `scope_fn` callback in `Collect::with_consumer`.
    invariant_lifetime: PhantomData<&'c mut &'c mut [T]>,
}

unsafe impl<'c, T> Send for CollectResult<'c, T> where T: Send {}

impl<'c, T> CollectResult<'c, T> {
    /// The current length of the collect result
    pub(super) fn len(&self) -> usize {
        self.initialized_len
    }

    /// Release ownership of the slice of elements, and return the length
    pub(super) fn release_ownership(mut self) -> usize {
        let ret = self.initialized_len;
        self.initialized_len = 0;
        ret
    }
}

impl<'c, T> Drop for CollectResult<'c, T> {
    fn drop(&mut self) {
        // Drop the first `self.initialized_len` elements, which have been recorded
        // to be initialized by the folder.
        unsafe {
            ptr::drop_in_place(slice::from_raw_parts_mut(
                self.start.0,
                self.initialized_len,
            ));
        }
    }
}

impl<'c, T: Send + 'c> Consumer<T> for CollectConsumer<'c, T> {
    type Folder = CollectResult<'c, T>;
    type Reducer = CollectReducer;
    type Result = CollectResult<'c, T>;

    fn split_at(self, index: usize) -> (Self, Self, CollectReducer) {
        let CollectConsumer { start, len, .. } = self;

        // Produce new consumers.
        // SAFETY: This assert checks that `index` is a valid offset for `start`
        unsafe {
            assert!(index <= len);
            (
                CollectConsumer::new(start.0, index),
                CollectConsumer::new(start.0.add(index), len - index),
                CollectReducer,
            )
        }
    }

    fn into_folder(self) -> Self::Folder {
        // Create a result/folder that consumes values and writes them
        // into the region after start. The initial result has length 0.
        CollectResult {
            start: self.start,
            total_len: self.len,
            initialized_len: 0,
            invariant_lifetime: PhantomData,
        }
    }

    fn full(&self) -> bool {
        false
    }
}

impl<'c, T: Send + 'c> Folder<T> for CollectResult<'c, T> {
    type Result = Self;

    fn consume(mut self, item: T) -> Self {
        assert!(
            self.initialized_len < self.total_len,
            "too many values pushed to consumer"
        );

        // SAFETY: The assert above is a bounds check for this write, and we
        // avoid assignment here so we do not drop an uninitialized T.
        unsafe {
            // Write item and increase the initialized length
            self.start.0.add(self.initialized_len).write(item);
            self.initialized_len += 1;
        }

        self
    }

    fn complete(self) -> Self::Result {
        // NB: We don't explicitly check that the local writes were complete,
        // but Collect will assert the total result length in the end.
        self
    }

    fn full(&self) -> bool {
        false
    }
}

/// Pretend to be unindexed for `special_collect_into_vec`,
/// but we should never actually get used that way...
impl<'c, T: Send + 'c> UnindexedConsumer<T> for CollectConsumer<'c, T> {
    fn split_off_left(&self) -> Self {
        unreachable!("CollectConsumer must be indexed!")
    }
    fn to_reducer(&self) -> Self::Reducer {
        CollectReducer
    }
}

/// CollectReducer combines adjacent chunks; the result must always
/// be contiguous so that it is one combined slice.
pub(super) struct CollectReducer;

impl<'c, T> Reducer<CollectResult<'c, T>> for CollectReducer {
    fn reduce(
        self,
        mut left: CollectResult<'c, T>,
        right: CollectResult<'c, T>,
    ) -> CollectResult<'c, T> {
        // Merge if the CollectResults are adjacent and in left to right order
        // else: drop the right piece now and total length will end up short in the end,
        // when the correctness of the collected result is asserted.
        unsafe {
            let left_end = left.start.0.add(left.initialized_len);
            if left_end == right.start.0 {
                left.total_len += right.total_len;
                left.initialized_len += right.release_ownership();
            }
            left
        }
    }
}
