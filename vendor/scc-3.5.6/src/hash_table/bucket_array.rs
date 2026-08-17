use std::alloc::{Layout, alloc, alloc_zeroed, dealloc};
use std::mem::{align_of, needs_drop, size_of};
use std::panic::UnwindSafe;
use std::ptr::NonNull;
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{Acquire, Relaxed};

use sdd::{AtomicShared, Guard, Tag};

use super::bucket::{BUCKET_LEN, Bucket, DataBlock, INDEX, LruList};
use crate::exit_guard::ExitGuard;

/// [`BucketArray`] is a special purpose array to manage [`Bucket`] and [`DataBlock`].
pub struct BucketArray<K, V, L: LruList, const TYPE: char> {
    buckets: NonNull<Bucket<K, V, L, TYPE>>,
    data_blocks: NonNull<DataBlock<K, V, BUCKET_LEN>>,
    array_len: usize,
    hash_offset: u8,
    sample_size: u8,
    bucket_ptr_offset: u16,
    linked_array: AtomicShared<BucketArray<K, V, L, TYPE>>,
    num_cleared_buckets: AtomicUsize,
}

impl<K, V, L: LruList, const TYPE: char> BucketArray<K, V, L, TYPE> {
    /// Creates a new [`BucketArray`] of the given capacity.
    ///
    /// `capacity` is the desired number of entries, not the length of the bucket array.
    pub(crate) fn new(
        capacity: usize,
        linked_array: AtomicShared<BucketArray<K, V, L, TYPE>>,
    ) -> Self {
        let adjusted_capacity = capacity
            .min(1_usize << (usize::BITS - 2))
            .next_power_of_two()
            .max(Self::minimum_capacity());
        let array_len = adjusted_capacity / BUCKET_LEN;
        let log2_array_len = u8::try_from(array_len.trailing_zeros()).unwrap_or(0);
        assert_eq!(1_usize << log2_array_len, array_len);

        let sample_size = log2_array_len.next_power_of_two();
        let alignment = align_of::<Bucket<K, V, L, TYPE>>();
        let layout = Self::bucket_array_layout(array_len);

        unsafe {
            let Some(unaligned_bucket_array_ptr) = NonNull::new(alloc_zeroed(layout)) else {
                panic!("memory allocation failure: {layout:?}");
            };
            let bucket_array_ptr_offset = unaligned_bucket_array_ptr.align_offset(alignment);
            assert_eq!(
                (unaligned_bucket_array_ptr.addr().get() + bucket_array_ptr_offset) % alignment,
                0
            );

            #[allow(clippy::cast_ptr_alignment)] // The alignment was just asserted.
            let buckets = unaligned_bucket_array_ptr
                .add(bucket_array_ptr_offset)
                .cast::<Bucket<K, V, L, TYPE>>();
            let bucket_array_ptr_offset = u16::try_from(bucket_array_ptr_offset).unwrap_or(0);
            let data_block_array_layout = Layout::from_size_align(
                size_of::<DataBlock<K, V, BUCKET_LEN>>() * array_len,
                align_of::<[DataBlock<K, V, BUCKET_LEN>; 0]>(),
            )
            .unwrap();

            // In case the below data block allocation fails, deallocate the bucket array.
            let alloc_guard = ExitGuard::new((), |()| {
                dealloc(unaligned_bucket_array_ptr.cast::<u8>().as_ptr(), layout);
            });

            let Some(data_blocks) =
                NonNull::new(alloc(data_block_array_layout).cast::<DataBlock<K, V, BUCKET_LEN>>())
            else {
                panic!("memory allocation failure: {data_block_array_layout:?}");
            };
            alloc_guard.forget();

            #[cfg(feature = "loom")]
            for i in 0..array_len {
                // `loom` types need proper initialization.
                buckets.add(i).write(Bucket::new());
            }

            Self {
                buckets,
                data_blocks,
                array_len,
                hash_offset: u8::try_from(u64::BITS).unwrap_or(64) - log2_array_len,
                sample_size,
                bucket_ptr_offset: bucket_array_ptr_offset,
                linked_array,
                num_cleared_buckets: AtomicUsize::new(0),
            }
        }
    }

    /// Returns the number of [`Bucket`] instances in the [`BucketArray`].
    #[inline]
    pub(crate) const fn len(&self) -> usize {
        self.array_len
    }

    /// Returns the number of entry slots in the bucket array.
    #[inline]
    pub(crate) const fn num_slots(&self) -> usize {
        self.array_len * BUCKET_LEN
    }

    /// Calculates the [`Bucket`] index for the hash value.
    #[allow(clippy::cast_possible_truncation)] // Intended truncation.
    #[inline]
    pub(crate) const fn bucket_index(&self, hash: u64) -> usize {
        // Take the upper n-bits to make sure that a single bucket is spread across a few adjacent
        // buckets when the hash table is resized.
        (hash >> self.hash_offset) as usize
    }

    /// Returns the minimum capacity.
    #[inline]
    pub(crate) const fn minimum_capacity() -> usize {
        BUCKET_LEN << 1
    }

    /// Returns `true` if the bucket array needs to be enlarged.
    pub(crate) const fn need_enlarge(capacity: usize, num_entries: usize) -> bool {
        // When the load factor is greater than `13/16`; `~10%` of buckets are expected to have
        // overflow buckets.
        num_entries > (capacity / 16) * 13
    }

    /// Returns `true` if the bucket array needs to be shrunk.
    pub(crate) const fn need_shrink(capacity: usize, num_entries: usize) -> bool {
        // When the load factor is less than `1/8`.
        num_entries < capacity / 8
    }

    /// Returns the optimal capacity.
    #[inline]
    pub(crate) fn optimal_capacity(
        capacity: usize,
        num_entries: usize,
        minimum_capacity: usize,
        maximum_capacity: usize,
    ) -> usize {
        if capacity < minimum_capacity || Self::need_enlarge(capacity, num_entries) {
            if capacity == maximum_capacity {
                capacity
            } else {
                let mut new_capacity = minimum_capacity
                    .next_power_of_two()
                    .max(capacity)
                    .min(maximum_capacity);
                while new_capacity / 2 < num_entries {
                    if new_capacity >= maximum_capacity {
                        break;
                    }
                    new_capacity *= 2;
                }
                new_capacity
            }
        } else if Self::need_shrink(capacity, num_entries) {
            (num_entries * 2)
                .max(minimum_capacity)
                .max(Self::minimum_capacity())
                .next_power_of_two()
        } else {
            capacity
        }
    }

    /// Returns a reference to its rehashing metadata.
    #[inline]
    pub(crate) const fn rehashing_metadata(&self) -> &AtomicUsize {
        &self.num_cleared_buckets
    }

    /// Checks if the key is eligible to initiate sampling.
    #[allow(clippy::cast_possible_truncation)] // Intended truncation.
    #[inline]
    pub(crate) const fn initiate_sampling(&self, hash: u64) -> bool {
        (hash as u8 & (self.sample_size - 1)) == 0
    }

    /// Returns the recommended sampling size for preliminary estimation.
    #[inline]
    pub(crate) const fn small_sample_size(&self) -> usize {
        // `Log2(Log2(len))` Expected error of size estimation is `~5%`.
        //
        // `2 -> 1`, `4-> 2`, `8 -> 4`, `1024 -> 8`, `1048576 -> 8`, and `2^58 -> 8`.
        (1 + self.sample_size.trailing_zeros()).next_power_of_two() as usize
    }

    /// Returns the recommended sampling size for more accurate estimation.
    #[inline]
    pub(crate) const fn large_sample_size(&self) -> usize {
        // `Log2(len)`: if `len` is sufficiently large, expected error of size estimation is `~3%`.
        //
        // `2 -> 1`, `4 -> 2`, `8 -> 4`, `1024 -> 16`, `1048576 -> 32`, and `2^58 -> 64`.
        self.sample_size as usize
    }

    /// Returns the recommended sampling size for full estimation.
    #[inline]
    pub(crate) fn full_sample_size(&self) -> usize {
        // `Max(len, 128)`: expected error of size estimation is `~1%`.
        self.len().min(128)
    }

    /// Returns a reference to a [`Bucket`] at the given position.
    #[inline]
    pub(crate) const fn bucket(&self, index: usize) -> &Bucket<K, V, L, TYPE> {
        debug_assert!(index < self.len());
        unsafe { self.buckets.add(index).as_ref() }
    }

    /// Returns a pointer to a [`DataBlock`] at the given position.
    #[inline]
    pub(crate) const fn data_block(&self, index: usize) -> NonNull<DataBlock<K, V, BUCKET_LEN>> {
        debug_assert!(index < self.len());
        unsafe { self.data_blocks.add(index) }
    }

    /// Returns `true` if an linked bucket array exists.
    #[inline]
    pub(crate) fn has_linked_array(&self) -> bool {
        !self.linked_array.is_null(Acquire)
    }

    /// Returns a reference to the linked bucket array pointer.
    #[inline]
    pub(crate) const fn linked_array_var(&self) -> &AtomicShared<BucketArray<K, V, L, TYPE>> {
        &self.linked_array
    }

    /// Returns a reference to the linked bucket array.
    #[inline]
    pub(crate) fn linked_array<'g>(
        &self,
        guard: &'g Guard,
    ) -> Option<&'g BucketArray<K, V, L, TYPE>> {
        unsafe { self.linked_array.load(Acquire, guard).as_ref_unchecked() }
    }

    /// Calculates the layout of the memory block for an array of `T`.
    #[inline]
    const fn bucket_array_layout(array_len: usize) -> Layout {
        let size_of_t = size_of::<Bucket<K, V, L, TYPE>>();
        let allocation_size = (array_len + 1) * size_of_t;
        // Intentionally misaligned in order to take full advantage of demand paging.
        unsafe { Layout::from_size_align_unchecked(allocation_size, 1) }
    }
}

impl<K, V, L: LruList, const TYPE: char> Drop for BucketArray<K, V, L, TYPE> {
    fn drop(&mut self) {
        if !self.linked_array.is_null(Relaxed) {
            unsafe {
                self.linked_array
                    .swap((None, Tag::None), Relaxed)
                    .0
                    .map(|a| a.drop_in_place());
            }
        }

        let num_cleared_buckets = if TYPE == INDEX && needs_drop::<(K, V)>() {
            // Removed entries in non-overflow buckets are neither relocated nor dropped.
            0
        } else {
            self.num_cleared_buckets.load(Relaxed)
        };
        (num_cleared_buckets..self.array_len).for_each(|i| {
            self.bucket(i).drop_entries(self.data_block(i));
        });

        #[cfg(feature = "loom")]
        for i in 0..self.array_len {
            // `loom` types need proper cleanup.
            drop(unsafe { self.buckets.add(i).read() });
        }

        unsafe {
            dealloc(
                self.buckets
                    .cast::<u8>()
                    .sub(self.bucket_ptr_offset as usize)
                    .as_ptr(),
                Self::bucket_array_layout(self.array_len),
            );
            dealloc(
                self.data_blocks.cast::<u8>().as_ptr(),
                Layout::from_size_align(
                    size_of::<DataBlock<K, V, BUCKET_LEN>>() * self.array_len,
                    align_of::<[DataBlock<K, V, BUCKET_LEN>; 0]>(),
                )
                .unwrap(),
            );
        }
    }
}

unsafe impl<K: Send, V: Send, L: LruList, const TYPE: char> Send for BucketArray<K, V, L, TYPE> {}

unsafe impl<K: Send + Sync, V: Send + Sync, L: LruList, const TYPE: char> Sync
    for BucketArray<K, V, L, TYPE>
{
}

impl<K: UnwindSafe, V: UnwindSafe, L: LruList, const TYPE: char> UnwindSafe
    for BucketArray<K, V, L, TYPE>
{
}
