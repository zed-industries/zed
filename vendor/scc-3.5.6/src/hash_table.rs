pub mod bucket;
pub mod bucket_array;

use std::hash::{BuildHasher, Hash};
use std::mem::forget;
use std::ops::Deref;
use std::ptr::{self, NonNull, from_ref};

#[cfg(not(feature = "loom"))]
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::{AcqRel, Acquire, Relaxed, Release};

use bucket::{BUCKET_LEN, CACHE, DataBlock, EntryPtr, INDEX, LruList, Reader, Writer};
use bucket_array::BucketArray;
#[cfg(feature = "loom")]
use loom::sync::atomic::AtomicUsize;
use sdd::{AtomicShared, Guard, Ptr, Shared, Tag};

use super::Equivalent;
use super::async_helper::AsyncGuard;
use super::exit_guard::ExitGuard;
use super::hash_table::bucket::Bucket;

/// `HashTable` defines common functions for hash table implementations.
pub(super) trait HashTable<K, V, H, L: LruList, const TYPE: char>
where
    K: Eq + Hash,
    H: BuildHasher,
{
    /// Returns the hash value of the key.
    #[inline]
    fn hash<Q>(&self, key: &Q) -> u64
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.hasher().hash_one(key)
    }

    /// Returns its [`BuildHasher`].
    fn hasher(&self) -> &H;

    /// Returns a reference to the [`BucketArray`] pointer.
    fn bucket_array_var(&self) -> &AtomicShared<BucketArray<K, V, L, TYPE>>;

    /// Returns a reference to the current [`BucketArray`].
    #[inline]
    fn bucket_array<'g>(&self, guard: &'g Guard) -> Option<&'g BucketArray<K, V, L, TYPE>> {
        unsafe {
            self.bucket_array_var()
                .load(Acquire, guard)
                .as_ref_unchecked()
        }
    }

    /// Passes the bucket array to the garbage collector associated with the hash table type.
    #[inline]
    fn defer_reclaim(&self, bucket_array: Shared<BucketArray<K, V, L, TYPE>>, _guard: &Guard) {
        drop(bucket_array);
    }

    /// Calculates the bucket index from the supplied key.
    #[inline]
    fn calculate_bucket_index(&self, hash: u64) -> usize {
        unsafe {
            self.bucket_array_var()
                .load(Acquire, &Guard::new())
                .as_ref_unchecked()
                .map_or(0, |a| a.bucket_index(hash))
        }
    }

    /// Returns a reference to a variable containing the minimum allowed capacity.
    fn minimum_capacity_var(&self) -> &AtomicUsize;

    /// Returns the current minimum allowed capacity.
    #[inline]
    fn minimum_capacity(&self) -> usize {
        self.minimum_capacity_var().load(Relaxed) & (!RESIZING)
    }

    /// Returns the maximum capacity.
    ///
    /// The maximum capacity must be a power of `2`.
    #[inline]
    fn maximum_capacity(&self) -> usize {
        MAXIMUM_CAPACITY_LIMIT
    }

    /// Reserves the specified capacity.
    ///
    /// Returns the actually allocated capacity. Return `0` if the sum of the current minimum
    /// capacity and the additional capacity exceeds [`Self::maximum_capacity`].
    fn reserve_capacity(&self, additional_capacity: usize) -> usize {
        let mut current_minimum_capacity = self.minimum_capacity_var().load(Relaxed);
        loop {
            if additional_capacity
                > self.maximum_capacity() - (current_minimum_capacity & (!RESIZING))
            {
                return 0;
            }
            match self.minimum_capacity_var().compare_exchange_weak(
                current_minimum_capacity,
                additional_capacity + current_minimum_capacity,
                Relaxed,
                Relaxed,
            ) {
                Ok(_) => {
                    let guard = Guard::new();
                    if let Some(current_array) = self.bucket_array(&guard) {
                        if !current_array.has_linked_array() {
                            self.try_resize(current_array, &guard);
                        }
                    }
                    return additional_capacity;
                }
                Err(actual) => current_minimum_capacity = actual,
            }
        }
    }

    /// Returns a reference to the bucket array.
    ///
    /// Allocates a new one if no bucket array has been allocated.
    #[inline]
    fn get_or_create_bucket_array<'g>(&self, guard: &'g Guard) -> &'g BucketArray<K, V, L, TYPE> {
        if let Some(current_array) = self.bucket_array(guard) {
            current_array
        } else {
            self.allocate_bucket_array(guard)
        }
    }

    /// Allocates a new bucket array.
    fn allocate_bucket_array<'g>(&self, guard: &'g Guard) -> &'g BucketArray<K, V, L, TYPE> {
        unsafe {
            let capacity = self.minimum_capacity();
            let allocated =
                Shared::new_with_unchecked(|| BucketArray::new(capacity, AtomicShared::null()));
            match self.bucket_array_var().compare_exchange(
                Ptr::null(),
                (Some(allocated), Tag::None),
                AcqRel,
                Acquire,
                guard,
            ) {
                Ok((_, ptr)) | Err((_, ptr)) => ptr.as_ref_unchecked().unwrap_unchecked(),
            }
        }
    }

    /// Returns the number of entry slots.
    #[inline]
    fn num_slots(&self, guard: &Guard) -> usize {
        if let Some(current_array) = self.bucket_array(guard) {
            current_array.num_slots()
        } else {
            0
        }
    }

    /// Returns the number of entries.
    ///
    /// In case there are more than `usize::MAX` entries, it returns `usize::MAX`.
    fn num_entries(&self, guard: &Guard) -> usize {
        let mut num_entries: usize = 0;
        if let Some(current_array) = self.bucket_array(guard) {
            if let Some(old_array) = current_array.linked_array(guard) {
                self.incremental_rehash_sync::<true>(current_array, guard);
                for i in 0..old_array.len() {
                    num_entries = num_entries.saturating_add(old_array.bucket(i).len());
                }
            }
            for i in 0..current_array.len() {
                num_entries = num_entries.saturating_add(current_array.bucket(i).len());
            }
            if num_entries == 0 && self.minimum_capacity() == 0 {
                self.try_resize(current_array, guard);
            }
        }
        num_entries
    }

    /// Returns `true` if a valid entry is found.
    fn has_entry(&self, guard: &Guard) -> bool {
        if let Some(current_array) = self.bucket_array(guard) {
            if let Some(old_array) = current_array.linked_array(guard) {
                self.incremental_rehash_sync::<true>(current_array, guard);
                for i in 0..old_array.len() {
                    if old_array.bucket(i).len() != 0 {
                        return true;
                    }
                }
            }
            for i in 0..current_array.len() {
                if current_array.bucket(i).len() != 0 {
                    return true;
                }
            }
            if self.minimum_capacity() == 0 {
                self.try_resize(current_array, guard);
            }
        }
        false
    }

    /// Estimates the number of entries by sampling buckets at the end of the bucket array.
    #[inline]
    fn sample(
        current_array: &BucketArray<K, V, L, TYPE>,
        start_index: usize,
        sample_size: usize,
    ) -> usize {
        let mut num_entries = 0;
        for i in start_index..start_index + sample_size {
            num_entries += current_array.bucket(i).len();
        }
        num_entries * (current_array.len() / sample_size)
    }

    /// Peeks an entry from the [`HashTable`].
    #[inline]
    fn peek_entry<'g, Q>(&self, key: &Q, guard: &'g Guard) -> Option<&'g (K, V)>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        debug_assert_eq!(TYPE, INDEX);

        let hash = self.hash(key);
        let mut current_array_ptr = self.bucket_array_var().load(Acquire, guard);
        while let Some(current_array) = unsafe { current_array_ptr.as_ref_unchecked() } {
            if let Some(old_array) = current_array.linked_array(guard) {
                self.incremental_rehash_sync::<true>(current_array, guard);
                let index = old_array.bucket_index(hash);
                if let Some(entry) =
                    old_array
                        .bucket(index)
                        .search_entry(old_array.data_block(index), key, hash)
                {
                    return Some(entry);
                }
            }

            let index = current_array.bucket_index(hash);
            if let Some(entry) =
                current_array
                    .bucket(index)
                    .search_entry(current_array.data_block(index), key, hash)
            {
                return Some(entry);
            }

            let new_current_array_ptr = self.bucket_array_var().load(Acquire, guard);
            if current_array_ptr == new_current_array_ptr {
                break;
            }
            current_array_ptr = new_current_array_ptr;
        }
        None
    }

    /// Reads an entry asynchronously from the [`HashTable`] with a shared lock acquired on the
    /// bucket.
    #[inline]
    async fn reader_async<Q, R, F: FnOnce(&K, &V) -> R>(&self, key: &Q, f: F) -> Option<R>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        let hash = self.hash(key);
        let async_guard = AsyncGuard::default();
        while let Some(current_array) = async_guard.load_unchecked(self.bucket_array_var(), Acquire)
        {
            if current_array.has_linked_array() {
                self.incremental_rehash_async(current_array, &async_guard)
                    .await;
                if !self
                    .dedup_bucket_async(
                        current_array,
                        current_array.bucket_index(hash),
                        &async_guard,
                    )
                    .await
                {
                    continue;
                }
            }

            let bucket_index = current_array.bucket_index(hash);
            let bucket = current_array.bucket(bucket_index);
            if let Some(reader) = Reader::lock_async(bucket, &async_guard).await {
                if let Some(entry) =
                    reader.search_entry(current_array.data_block(bucket_index), key, hash)
                {
                    return Some(f(&entry.0, &entry.1));
                }
                break;
            }
        }
        None
    }

    /// Reads an entry synchronously from the [`HashTable`] with a shared lock acquired on the
    /// bucket.
    #[inline]
    fn reader_sync<Q, R, F: FnOnce(&K, &V) -> R>(&self, key: &Q, f: F) -> Option<R>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        let hash = self.hash(key);
        let guard = Guard::new();
        while let Some(current_array) = self.bucket_array(&guard) {
            let index = current_array.bucket_index(hash);
            if let Some(old_array) = current_array.linked_array(&guard) {
                self.incremental_rehash_sync::<false>(current_array, &guard);
                self.dedup_bucket_sync::<false>(current_array, old_array, index);
            }

            let bucket = current_array.bucket(index);
            if let Some(reader) = Reader::lock_sync(bucket) {
                if let Some(entry) = reader.search_entry(current_array.data_block(index), key, hash)
                {
                    return Some(f(&entry.0, &entry.1));
                }
                break;
            }
        }
        None
    }

    /// Returns a [`LockedBucket`] for writing an entry asynchronously.
    ///
    /// If the container is empty, a new bucket array is allocated.
    #[inline]
    async fn writer_async(&self, hash: u64) -> LockedBucket<K, V, L, TYPE> {
        let async_guard = AsyncGuard::default();
        if let Some(locked_bucket) = self.try_optional_writer::<true>(hash, async_guard.guard()) {
            return locked_bucket;
        }
        loop {
            let current_array = self.get_or_create_bucket_array(async_guard.guard());
            if current_array.has_linked_array() {
                self.incremental_rehash_async(current_array, &async_guard)
                    .await;
                if !self
                    .dedup_bucket_async(
                        current_array,
                        current_array.bucket_index(hash),
                        &async_guard,
                    )
                    .await
                {
                    continue;
                }
            }

            let bucket_index = current_array.bucket_index(hash);
            let bucket = current_array.bucket(bucket_index);
            if (TYPE != CACHE || current_array.num_slots() < self.maximum_capacity())
                && bucket.len() >= BUCKET_LEN - 1
                && current_array.initiate_sampling(hash)
            {
                self.try_enlarge(current_array, bucket_index, async_guard.guard());
            }
            if let Some(writer) = Writer::lock_async(bucket, &async_guard).await {
                return LockedBucket {
                    writer,
                    data_block: current_array.data_block(bucket_index),
                    bucket_index,
                    bucket_array: into_non_null(current_array),
                };
            }
        }
    }

    /// Returns a [`LockedBucket`] for writing an entry synchronously.
    ///
    /// If the container is empty, a new bucket array is allocated.
    #[inline]
    fn writer_sync(&self, hash: u64) -> LockedBucket<K, V, L, TYPE> {
        let guard = Guard::new();
        loop {
            let current_array = self.get_or_create_bucket_array(&guard);
            let bucket_index = current_array.bucket_index(hash);
            if let Some(old_array) = current_array.linked_array(&guard) {
                self.incremental_rehash_sync::<false>(current_array, &guard);
                self.dedup_bucket_sync::<false>(current_array, old_array, bucket_index);
            }

            let bucket = current_array.bucket(bucket_index);
            if (TYPE != CACHE || current_array.num_slots() < self.maximum_capacity())
                && bucket.len() >= BUCKET_LEN - 1
                && current_array.initiate_sampling(hash)
            {
                self.try_enlarge(current_array, bucket_index, &guard);
            }

            if let Some(writer) = Writer::lock_sync(bucket) {
                return LockedBucket {
                    writer,
                    data_block: current_array.data_block(bucket_index),
                    bucket_index,
                    bucket_array: into_non_null(current_array),
                };
            }
        }
    }

    /// Returns a [`LockedBucket`] for writing an entry asynchronously.
    ///
    /// If the container is empty, `None` is returned.
    #[inline]
    async fn optional_writer_async(&self, hash: u64) -> Option<LockedBucket<K, V, L, TYPE>> {
        let async_guard = AsyncGuard::default();
        if let Some(locked_bucket) = self.try_optional_writer::<false>(hash, async_guard.guard()) {
            return Some(locked_bucket);
        }
        while let Some(current_array) = async_guard.load_unchecked(self.bucket_array_var(), Acquire)
        {
            if current_array.has_linked_array() {
                self.incremental_rehash_async(current_array, &async_guard)
                    .await;
                if !self
                    .dedup_bucket_async(
                        current_array,
                        current_array.bucket_index(hash),
                        &async_guard,
                    )
                    .await
                {
                    continue;
                }
            }

            let bucket_index = current_array.bucket_index(hash);
            let bucket = current_array.bucket(bucket_index);
            if let Some(writer) = Writer::lock_async(bucket, &async_guard).await {
                return Some(LockedBucket {
                    writer,
                    data_block: current_array.data_block(bucket_index),
                    bucket_index,
                    bucket_array: into_non_null(current_array),
                });
            }
        }
        None
    }

    /// Returns a [`LockedBucket`] for writing an entry synchronously.
    ///
    /// If the container is empty, `None` is returned.
    #[inline]
    fn optional_writer_sync(&self, hash: u64) -> Option<LockedBucket<K, V, L, TYPE>> {
        let guard = Guard::new();
        while let Some(current_array) = self.bucket_array(&guard) {
            let bucket_index = current_array.bucket_index(hash);
            if let Some(old_array) = current_array.linked_array(&guard) {
                self.incremental_rehash_sync::<false>(current_array, &guard);
                self.dedup_bucket_sync::<false>(current_array, old_array, bucket_index);
            }

            let bucket = current_array.bucket(bucket_index);
            if let Some(writer) = Writer::lock_sync(bucket) {
                return Some(LockedBucket {
                    writer,
                    data_block: current_array.data_block(bucket_index),
                    bucket_index,
                    bucket_array: into_non_null(current_array),
                });
            }
        }
        None
    }

    /// Tries to returns a [`LockedBucket`] for writing an entry.
    #[inline]
    fn try_optional_writer<const CHECK_SIZE: bool>(
        &self,
        hash: u64,
        guard: &Guard,
    ) -> Option<LockedBucket<K, V, L, TYPE>> {
        if let Some(current_array) = self.bucket_array(guard) {
            if current_array.has_linked_array() {
                return None;
            }
            let bucket_index = current_array.bucket_index(hash);
            let bucket = current_array.bucket(bucket_index);
            if CHECK_SIZE && bucket.len() >= BUCKET_LEN {
                return None;
            }
            if let Ok(Some(writer)) = Writer::try_lock(bucket) {
                return Some(LockedBucket {
                    writer,
                    data_block: current_array.data_block(bucket_index),
                    bucket_index,
                    bucket_array: into_non_null(current_array),
                });
            }
        }
        None
    }

    /// Iterates over all the buckets in the [`HashTable`] asynchronously.
    ///
    /// This method stops iterating when the closure returns `false`.
    #[inline]
    async fn for_each_reader_async<F>(&self, mut f: F)
    where
        F: FnMut(Reader<K, V, L, TYPE>, NonNull<DataBlock<K, V, BUCKET_LEN>>) -> bool,
    {
        let async_guard = AsyncGuard::default();
        let mut start_index = 0;
        let mut prev_len = 0;
        while let Some(current_array) = async_guard.load_unchecked(self.bucket_array_var(), Acquire)
        {
            // In case the method is repeating the routine, iterate over entries from the middle of
            // the array.
            start_index = if prev_len == 0 || prev_len == current_array.len() {
                start_index
            } else {
                from_index_to_range(prev_len, current_array.len(), start_index).0
            };
            prev_len = current_array.len();

            while start_index < current_array.len() {
                if current_array.has_linked_array() {
                    self.incremental_rehash_async(current_array, &async_guard)
                        .await;
                    if !self
                        .dedup_bucket_async(current_array, start_index, &async_guard)
                        .await
                    {
                        // Retry the operation since there is a possibility that the current bucket
                        // array was replaced by a new one.
                        break;
                    }
                }

                let bucket = current_array.bucket(start_index);
                if let Some(reader) = Reader::lock_async(bucket, &async_guard).await {
                    if !async_guard.check_ref(self.bucket_array_var(), current_array, Acquire) {
                        // `current_array` is no longer the current one.
                        break;
                    }
                    let data_block = current_array.data_block(start_index);
                    if !f(reader, data_block) {
                        return;
                    }
                } else {
                    // `current_array` is no longer the current one.
                    break;
                }

                start_index += 1;
            }

            if start_index == current_array.len() {
                break;
            }
        }
    }

    /// Iterates over all the buckets in the [`HashTable`] synchronously.
    ///
    /// This method stops iterating when the closure returns `false`.
    #[inline]
    fn for_each_reader_sync<F>(&self, guard: &Guard, mut f: F)
    where
        F: FnMut(Reader<K, V, L, TYPE>, NonNull<DataBlock<K, V, BUCKET_LEN>>) -> bool,
    {
        let mut start_index = 0;
        let mut prev_len = 0;
        while let Some(current_array) = self.bucket_array(guard) {
            // In case the method is repeating the routine, iterate over entries from the middle of
            // the array.
            start_index = if prev_len == 0 || prev_len == current_array.len() {
                start_index
            } else {
                from_index_to_range(prev_len, current_array.len(), start_index).0
            };
            prev_len = current_array.len();

            while start_index < current_array.len() {
                let index = start_index;
                if let Some(old_array) = current_array.linked_array(guard) {
                    self.incremental_rehash_sync::<false>(current_array, guard);
                    self.dedup_bucket_sync::<false>(current_array, old_array, index);
                }

                let bucket = current_array.bucket(index);
                if let Some(reader) = Reader::lock_sync(bucket) {
                    let data_block = current_array.data_block(index);
                    if !f(reader, data_block) {
                        return;
                    }
                } else {
                    // `current_array` is no longer the current one.
                    break;
                }
                start_index += 1;
            }

            if start_index == current_array.len() {
                break;
            }
        }
    }

    /// Iterates over all the buckets in the [`HashTable`].
    ///
    /// This method stops iterating when the closure returns `false`.
    #[inline]
    async fn for_each_writer_async<F>(
        &self,
        mut start_index: usize,
        expected_array_len: usize,
        mut f: F,
    ) where
        F: FnMut(LockedBucket<K, V, L, TYPE>, &mut bool) -> bool,
    {
        let async_guard = AsyncGuard::default();
        let mut prev_len = expected_array_len;
        let mut removed = false;
        while let Some(current_array) = async_guard.load_unchecked(self.bucket_array_var(), Acquire)
        {
            // In case the method is repeating the routine, iterate over entries from the middle of
            // the array.
            let current_array_len = current_array.len();
            start_index = if prev_len == 0 || prev_len == current_array_len {
                start_index
            } else {
                from_index_to_range(prev_len, current_array_len, start_index).0
            };
            prev_len = current_array_len;

            while start_index < current_array_len {
                let bucket_index = start_index;
                if current_array.has_linked_array() {
                    self.incremental_rehash_async(current_array, &async_guard)
                        .await;
                    if !self
                        .dedup_bucket_async(current_array, bucket_index, &async_guard)
                        .await
                    {
                        // Retry the operation since there is a possibility that the current bucket
                        // array was replaced by a new one.
                        break;
                    }
                }

                let bucket = current_array.bucket(bucket_index);
                if let Some(writer) = Writer::lock_async(bucket, &async_guard).await {
                    if !async_guard.check_ref(self.bucket_array_var(), current_array, Acquire) {
                        // `current_array` is no longer the current one.
                        break;
                    }
                    let locked_bucket = LockedBucket {
                        writer,
                        data_block: current_array.data_block(bucket_index),
                        bucket_index,
                        bucket_array: into_non_null(current_array),
                    };
                    if !f(locked_bucket, &mut removed) {
                        // Stop iterating over buckets.
                        start_index = current_array_len;
                        break;
                    }
                } else {
                    // `current_array` is no longer the current one.
                    break;
                }
                start_index += 1;
            }

            if start_index == current_array_len {
                break;
            }
        }

        if removed {
            if TYPE == INDEX {
                async_guard.guard().accelerate();
            }
            if let Some(current_array) = self.bucket_array(async_guard.guard()) {
                self.try_shrink(current_array, 0, async_guard.guard());
            }
        }
    }

    /// Iterates over all the buckets in the [`HashTable`].
    ///
    /// This methods stops iterating when the closure returns `false`.
    #[inline]
    fn for_each_writer_sync<F>(
        &self,
        mut start_index: usize,
        expected_array_len: usize,
        guard: &Guard,
        mut f: F,
    ) where
        F: FnMut(LockedBucket<K, V, L, TYPE>, &mut bool) -> bool,
    {
        let mut prev_len = expected_array_len;
        let mut removed = false;
        while let Some(current_array) = self.bucket_array(guard) {
            // In case the method is repeating the routine, iterate over entries from the middle of
            // the array.
            let current_array_len = current_array.len();
            start_index = if prev_len == 0 || prev_len == current_array_len {
                start_index
            } else {
                from_index_to_range(prev_len, current_array_len, start_index).0
            };
            prev_len = current_array_len;

            while start_index < current_array_len {
                let bucket_index = start_index;
                if let Some(old_array) = current_array.linked_array(guard) {
                    self.incremental_rehash_sync::<false>(current_array, guard);
                    self.dedup_bucket_sync::<false>(current_array, old_array, bucket_index);
                }

                let bucket = current_array.bucket(bucket_index);
                if let Some(writer) = Writer::lock_sync(bucket) {
                    let locked_bucket = LockedBucket {
                        writer,
                        data_block: current_array.data_block(bucket_index),
                        bucket_index,
                        bucket_array: into_non_null(current_array),
                    };
                    if !f(locked_bucket, &mut removed) {
                        // Stop iterating over buckets.
                        start_index = current_array_len;
                        break;
                    }
                } else {
                    // `current_array` is no longer the current one.
                    break;
                }
                start_index += 1;
            }

            if start_index == current_array_len {
                break;
            }
        }

        if removed {
            if TYPE == INDEX {
                guard.accelerate();
            }
            if let Some(current_array) = self.bucket_array(guard) {
                self.try_shrink(current_array, 0, guard);
            }
        }
    }

    /// Tries to reserve a [`Bucket`] and returns a [`LockedBucket`].
    #[inline]
    fn try_reserve_bucket(&self, hash: u64, guard: &Guard) -> Option<LockedBucket<K, V, L, TYPE>> {
        loop {
            let current_array = self.get_or_create_bucket_array(guard);
            let bucket_index = current_array.bucket_index(hash);
            if let Some(old_array) = current_array.linked_array(guard) {
                self.incremental_rehash_sync::<true>(current_array, guard);
                if !self.dedup_bucket_sync::<true>(current_array, old_array, bucket_index) {
                    return None;
                }
            }

            let mut bucket = current_array.bucket(bucket_index);
            if (TYPE != CACHE || current_array.num_slots() < self.maximum_capacity())
                && bucket.len() >= BUCKET_LEN - 1
                && current_array.initiate_sampling(hash)
            {
                self.try_enlarge(current_array, bucket_index, guard);
                bucket = current_array.bucket(bucket_index);
            }

            let Ok(writer) = Writer::try_lock(bucket) else {
                return None;
            };
            if let Some(writer) = writer {
                return Some(LockedBucket {
                    writer,
                    data_block: current_array.data_block(bucket_index),
                    bucket_index,
                    bucket_array: into_non_null(current_array),
                });
            }
        }
    }

    /// Deduplicates buckets that may share the same hash values asynchronously.
    ///
    /// Returns `false` if the old buckets may remain in the old bucket array, or the whole
    /// operation has to be retried due to an ABA problem.
    ///
    /// # Note
    ///
    /// There is a possibility of an ABA problem where the bucket array was deallocated and a new
    /// bucket array of a different size has been allocated in the same memory region. To avoid
    /// this problem, the method returns `false` if it finds a killed bucket and the task was
    /// suspended.
    async fn dedup_bucket_async<'g>(
        &self,
        current_array: &'g BucketArray<K, V, L, TYPE>,
        index: usize,
        async_guard: &'g AsyncGuard,
    ) -> bool {
        if !async_guard.check_ref(self.bucket_array_var(), current_array, Acquire) {
            // A new bucket array was created in the meantime.
            return false;
        }

        if let Some(old_array) =
            async_guard.load_unchecked(current_array.linked_array_var(), Acquire)
        {
            let range = from_index_to_range(current_array.len(), old_array.len(), index);
            for old_index in range.0..range.1 {
                let bucket = old_array.bucket(old_index);
                let writer = Writer::lock_async(bucket, async_guard).await;
                if let Some(writer) = writer {
                    self.relocate_bucket_async(
                        current_array,
                        old_array,
                        old_index,
                        writer,
                        async_guard,
                    )
                    .await;
                } else if !async_guard.has_guard() {
                    // The bucket was killed and the guard has been invalidated. Validating the
                    // reference is not sufficient in this case since the current bucket array could
                    // have been replaced with a new one.
                    return false;
                }

                // The old bucket array was removed, no point in trying to move entries from it.
                if !current_array.has_linked_array() {
                    break;
                }
            }
        }

        true
    }

    /// Deduplicates buckets that may share the same hash values synchronously.
    ///
    /// Returns `true` if the corresponding entries were successfully moved.
    fn dedup_bucket_sync<'g, const TRY_LOCK: bool>(
        &self,
        current_array: &'g BucketArray<K, V, L, TYPE>,
        old_array: &'g BucketArray<K, V, L, TYPE>,
        index: usize,
    ) -> bool {
        let range = from_index_to_range(current_array.len(), old_array.len(), index);
        for old_index in range.0..range.1 {
            let bucket = old_array.bucket(old_index);
            let writer = if TRY_LOCK {
                let Ok(writer) = Writer::try_lock(bucket) else {
                    return false;
                };
                writer
            } else {
                Writer::lock_sync(bucket)
            };
            if let Some(writer) = writer {
                if !self.relocate_bucket_sync::<TRY_LOCK>(
                    current_array,
                    old_array,
                    old_index,
                    writer,
                ) {
                    return false;
                }
            }
        }
        true
    }

    /// Relocates the bucket to the current bucket array.
    async fn relocate_bucket_async<'g>(
        &self,
        current_array: &'g BucketArray<K, V, L, TYPE>,
        old_array: &'g BucketArray<K, V, L, TYPE>,
        old_index: usize,
        old_writer: Writer<K, V, L, TYPE>,
        async_guard: &'g AsyncGuard,
    ) {
        if old_writer.len() == 0 {
            // Instantiate a guard while the lock is held to ensure that the bucket arrays are not
            // dropped.
            async_guard.guard();
            old_writer.kill();
            return;
        }

        // Lock the target buckets.
        let (target_index, end_target_index) =
            from_index_to_range(old_array.len(), current_array.len(), old_index);
        for i in target_index..end_target_index {
            let writer = unsafe {
                Writer::lock_async(current_array.bucket(i), async_guard)
                    .await
                    .unwrap_unchecked()
            };
            forget(writer);
        }

        // It may seem inefficient to reevaluate the same values, but it is beneficial for reducing
        // the `Future` size.
        let Some(old_array) = current_array.linked_array(async_guard.guard()) else {
            return;
        };
        let (target_index, end_target_index) =
            from_index_to_range(old_array.len(), current_array.len(), old_index);
        let unlock = ExitGuard::new(
            (current_array, target_index, end_target_index),
            |(current_array, target_index, end_target_index)| {
                for i in target_index..end_target_index {
                    let writer = Writer::from_bucket(current_array.bucket(i));
                    drop(writer);
                }
            },
        );

        self.relocate_bucket(unlock.0, unlock.1, old_array, old_index, &old_writer);
        drop(unlock);

        // Instantiate a guard while the lock is held to ensure that the bucket arrays are not
        // dropped.
        async_guard.guard();
        old_writer.kill();
    }

    /// Relocates the bucket to the current bucket array.
    ///
    /// Returns `false` if locking failed.
    fn relocate_bucket_sync<'g, const TRY_LOCK: bool>(
        &self,
        current_array: &'g BucketArray<K, V, L, TYPE>,
        old_array: &'g BucketArray<K, V, L, TYPE>,
        old_index: usize,
        old_writer: Writer<K, V, L, TYPE>,
    ) -> bool {
        if old_writer.len() == 0 {
            old_writer.kill();
            return true;
        }

        let (target_index, end_target_index) =
            from_index_to_range(old_array.len(), current_array.len(), old_index);

        // Lock the target buckets.
        for i in target_index..end_target_index {
            let writer = if TRY_LOCK {
                let Ok(Some(writer)) = Writer::try_lock(current_array.bucket(i)) else {
                    for j in target_index..i {
                        let writer = Writer::from_bucket(current_array.bucket(j));
                        drop(writer);
                    }
                    return false;
                };
                writer
            } else {
                unsafe { Writer::lock_sync(current_array.bucket(i)).unwrap_unchecked() }
            };
            forget(writer);
        }
        let unlock = ExitGuard::new((), |()| {
            for i in target_index..end_target_index {
                let writer = Writer::from_bucket(current_array.bucket(i));
                drop(writer);
            }
        });

        self.relocate_bucket(
            current_array,
            target_index,
            old_array,
            old_index,
            &old_writer,
        );
        drop(unlock);

        old_writer.kill();
        true
    }

    /// Relocates entries from the old bucket to the corresponding buckets in the current bucket
    /// array.
    ///
    /// This assumes that all the target buckets are locked.
    fn relocate_bucket(
        &self,
        current_array: &BucketArray<K, V, L, TYPE>,
        target_index: usize,
        old_array: &BucketArray<K, V, L, TYPE>,
        old_index: usize,
        old_writer: &Writer<K, V, L, TYPE>,
    ) {
        // Need to pre-allocate slots if the container is shrinking or the old bucket overflows,
        // because incomplete relocation of entries may result in duplicate key problems.
        let pre_allocate_slots =
            old_array.len() > current_array.len() || old_writer.len() > BUCKET_LEN;
        let old_data_block = old_array.data_block(old_index);
        let mut entry_ptr = EntryPtr::null();
        let mut position = 0;
        let mut dist = [0_u32; 8];
        let mut extended_dist: Vec<u32> = Vec::new();
        let mut hash_data = [0_u64; BUCKET_LEN];

        // Collect data for relocation.
        while entry_ptr.find_next(old_writer) {
            let (offset, hash) = if old_array.len() >= current_array.len() {
                (0, u64::from(entry_ptr.partial_hash(&**old_writer)))
            } else {
                let hash = self.hash(&entry_ptr.get_mut(old_data_block, old_writer).0);
                let new_index = current_array.bucket_index(hash);
                debug_assert!(new_index - target_index < (current_array.len() / old_array.len()));
                (new_index - target_index, hash)
            };

            if pre_allocate_slots {
                if position != BUCKET_LEN {
                    hash_data[position] = hash;
                    position += 1;
                }
                if offset < 8 {
                    dist[offset] += 1;
                } else {
                    if extended_dist.len() < offset - 7 {
                        extended_dist.resize(offset - 7, 0);
                    }
                    extended_dist[offset - 8] += 1;
                }
            } else {
                current_array.bucket(target_index + offset).extract_from(
                    current_array.data_block(target_index + offset),
                    hash,
                    old_writer,
                    old_data_block,
                    &mut entry_ptr,
                );
            }
        }

        if !pre_allocate_slots {
            return;
        }

        // Allocate memory.
        for (i, d) in dist.iter().chain(extended_dist.iter()).enumerate() {
            if *d != 0 {
                let bucket = current_array.bucket(target_index + i);
                bucket.reserve_slots((*d) as usize);
            }
        }

        // Relocate entries; it is infallible.
        entry_ptr = EntryPtr::null();
        position = 0;
        while entry_ptr.find_next(old_writer) {
            let hash = if old_array.len() >= current_array.len() {
                u64::from(entry_ptr.partial_hash(&**old_writer))
            } else if position == BUCKET_LEN {
                self.hash(&entry_ptr.get(old_data_block).0)
            } else {
                position += 1;
                hash_data[position - 1]
            };
            let index = if old_array.len() >= current_array.len() {
                target_index
            } else {
                current_array.bucket_index(hash)
            };
            current_array.bucket(index).extract_from(
                current_array.data_block(index),
                hash,
                old_writer,
                old_data_block,
                &mut entry_ptr,
            );
        }
    }

    /// Starts incremental rehashing.
    #[inline]
    fn start_incremental_rehash(old_array: &BucketArray<K, V, L, TYPE>) -> Option<usize> {
        // Assign itself a range of `Bucket` instances to rehash.
        //
        // Aside from the range, it increments the implicit reference counting field in
        // `old_array.rehashing`.
        let rehashing_metadata = old_array.rehashing_metadata();
        let mut current = rehashing_metadata.load(Relaxed);
        loop {
            if current >= old_array.len() || (current & (BUCKET_LEN * 2 - 1)) == BUCKET_LEN * 2 - 1
            {
                // Only `BUCKET_LEN * 2` concurrent threads are allowed to rehash `BUCKET_LEN * 2`
                // buckets.
                return None;
            }
            match rehashing_metadata.compare_exchange_weak(
                current,
                current + BUCKET_LEN * 2 + 1,
                Acquire,
                Relaxed,
            ) {
                Ok(_) => {
                    current &= !(BUCKET_LEN * 2 - 1);
                    return Some(current);
                }
                Err(result) => current = result,
            }
        }
    }

    /// Ends incremental rehashing.
    #[inline]
    fn end_incremental_rehash(
        old_array: &BucketArray<K, V, L, TYPE>,
        prev: usize,
        success: bool,
    ) -> bool {
        let rehashing_metadata = old_array.rehashing_metadata();
        if success {
            // Keep the index as it is.
            let metadata = rehashing_metadata.fetch_sub(1, Release) - 1;
            (metadata & (BUCKET_LEN * 2 - 1) == 0) && metadata >= old_array.len()
        } else {
            // On failure, `rehashing` reverts to its previous state.
            let mut current = rehashing_metadata.load(Relaxed);
            loop {
                let new = if current <= prev {
                    current - 1
                } else {
                    let refs = current & (BUCKET_LEN * 2 - 1);
                    prev | (refs - 1)
                };
                match rehashing_metadata.compare_exchange_weak(current, new, Release, Relaxed) {
                    Ok(_) => break,
                    Err(actual) => current = actual,
                }
            }
            false
        }
    }

    /// Relocates a fixed number of buckets from the old bucket array to the current array
    /// asynchronously.
    ///
    /// Once this methods successfully started rehashing, there is no possibility that the bucket
    /// array is deallocated.
    async fn incremental_rehash_async<'g>(
        &self,
        current_array: &'g BucketArray<K, V, L, TYPE>,
        async_guard: &'g AsyncGuard,
    ) {
        if let Some(old_array) =
            async_guard.load_unchecked(current_array.linked_array_var(), Acquire)
        {
            if let Some(current) = Self::start_incremental_rehash(old_array) {
                let rehashing_guard = ExitGuard::new((old_array, current), |(old_array, prev)| {
                    Self::end_incremental_rehash(old_array, prev, false);
                });

                for bucket_index in
                    rehashing_guard.1..(rehashing_guard.1 + BUCKET_LEN * 2).min(old_array.len())
                {
                    let old_bucket = rehashing_guard.0.bucket(bucket_index);
                    let writer = Writer::lock_async(old_bucket, async_guard).await;
                    if let Some(writer) = writer {
                        self.relocate_bucket_async(
                            current_array,
                            rehashing_guard.0,
                            bucket_index,
                            writer,
                            async_guard,
                        )
                        .await;
                    }
                    debug_assert!(current_array.has_linked_array());
                }

                if Self::end_incremental_rehash(rehashing_guard.0, rehashing_guard.1, true) {
                    if let Some(bucket_array) = current_array
                        .linked_array_var()
                        .swap((None, Tag::None), Release)
                        .0
                    {
                        self.defer_reclaim(bucket_array, async_guard.guard());
                    }
                }
                rehashing_guard.forget();
            }
        }
    }

    /// Relocates a fixed number of buckets from the old array to the current array synchronously.
    ///
    /// Returns `true` if `old_array` is null.
    fn incremental_rehash_sync<'g, const TRY_LOCK: bool>(
        &self,
        current_array: &'g BucketArray<K, V, L, TYPE>,
        guard: &'g Guard,
    ) {
        if let Some(old_array) = current_array.linked_array(guard) {
            if let Some(current) = Self::start_incremental_rehash(old_array) {
                let rehashing_guard = ExitGuard::new((old_array, current), |(old_array, prev)| {
                    Self::end_incremental_rehash(old_array, prev, false);
                });

                for bucket_index in
                    rehashing_guard.1..(rehashing_guard.1 + BUCKET_LEN * 2).min(old_array.len())
                {
                    let old_bucket = rehashing_guard.0.bucket(bucket_index);
                    let writer = if TRY_LOCK {
                        let Ok(writer) = Writer::try_lock(old_bucket) else {
                            return;
                        };
                        writer
                    } else {
                        Writer::lock_sync(old_bucket)
                    };
                    if let Some(writer) = writer {
                        if !self.relocate_bucket_sync::<TRY_LOCK>(
                            current_array,
                            rehashing_guard.0,
                            bucket_index,
                            writer,
                        ) {
                            return;
                        }
                    }
                }

                if Self::end_incremental_rehash(rehashing_guard.0, rehashing_guard.1, true) {
                    if let Some(bucket_array) = current_array
                        .linked_array_var()
                        .swap((None, Tag::None), Release)
                        .0
                    {
                        self.defer_reclaim(bucket_array, guard);
                    }
                }
                rehashing_guard.forget();
            }
        }
    }

    /// Tries to enlarge [`HashTable`].
    fn try_enlarge(&self, current_array: &BucketArray<K, V, L, TYPE>, index: usize, guard: &Guard) {
        if !current_array.has_linked_array() {
            let sample_size = current_array.small_sample_size();
            let sample_capacity = sample_size * BUCKET_LEN;
            let mut num_entries = 0;
            let start_index = index & (sample_size - 1);
            for i in start_index..start_index + sample_size {
                num_entries += current_array.bucket(i).len();
                if BucketArray::<K, V, L, TYPE>::need_enlarge(sample_capacity, num_entries) {
                    self.try_resize(current_array, guard);
                    break;
                }
            }
        }
    }

    /// Tries to shrink the [`HashTable`] to fit.
    fn try_shrink(&self, current_array: &BucketArray<K, V, L, TYPE>, index: usize, guard: &Guard) {
        if !current_array.has_linked_array() && current_array.num_slots() > self.minimum_capacity()
        {
            let sample_size = current_array.small_sample_size();
            let sample_capacity = sample_size * BUCKET_LEN;
            let mut num_entries = 0;
            let start_index = index & (sample_size - 1);
            for i in start_index..start_index + sample_size {
                num_entries += current_array.bucket(i).len();
                if !BucketArray::<K, V, L, TYPE>::need_shrink(sample_capacity, num_entries) {
                    return;
                }
            }
            self.try_resize(current_array, guard);
        }
    }

    /// Tries to resize the array.
    ///
    /// The table is resized after three times of sampling and all three samples indicate that the
    /// array should be resized. First sampling is done with a very small sample size in
    /// `try_enlarge` and `try_shrink`. Second sampling is done with a large sample size in the
    /// first half of this method, before acquiring the resize lock. The last sampling is done after
    /// acquiring the resize lock with a very large sample size.
    fn try_resize(&self, sampled_array: &BucketArray<K, V, L, TYPE>, guard: &Guard) {
        let current_array_ptr = self.bucket_array_var().load(Acquire, guard);
        let Some(current_array) = (unsafe { current_array_ptr.as_ref_unchecked() }) else {
            // The hash table is empty.
            return;
        };
        if !ptr::eq(current_array, sampled_array) {
            // The preliminary sampling result cannot be trusted anymore.
            return;
        } else if current_array.has_linked_array() {
            // Cannot resize with a bucket array linked to the current bucket array.
            return;
        } else if self.minimum_capacity_var().load(Relaxed) >= RESIZING {
            // The table is being resized.
            return;
        }

        let minimum_capacity = self.minimum_capacity();
        let maximum_capacity = self.maximum_capacity();
        let estimation = Self::sample(current_array, 0, current_array.large_sample_size());
        let capacity = current_array.num_slots();

        let try_resize = BucketArray::<K, V, L, TYPE>::need_enlarge(capacity, estimation)
            || BucketArray::<K, V, L, TYPE>::need_shrink(capacity, estimation);
        let try_drop_table = estimation == 0 && minimum_capacity == 0;
        if !try_resize && !try_drop_table {
            // Nothing to do.
            return;
        }

        if self
            .minimum_capacity_var()
            .fetch_update(AcqRel, Acquire, |lock_state| {
                if lock_state >= RESIZING {
                    None
                } else {
                    Some(lock_state + RESIZING)
                }
            })
            .is_err()
        {
            // The bucket array is being replaced with a new one.
            return;
        }
        let _lock_guard = ExitGuard::new((), |()| {
            self.minimum_capacity_var().fetch_sub(RESIZING, Release);
        });

        if self.bucket_array_var().load(Acquire, guard) != current_array_ptr {
            // Resized in the meantime.
            return;
        }

        if try_drop_table {
            // Try to drop the hash table with all the buckets locked.
            let mut writer_guard = ExitGuard::new((0, false), |(len, success): (usize, bool)| {
                for i in 0..len {
                    let writer = Writer::from_bucket(current_array.bucket(i));
                    if success {
                        debug_assert_eq!(writer.len(), 0);
                        writer.kill();
                    }
                }
            });

            if !(0..current_array.len()).any(|i| {
                if let Ok(Some(writer)) = Writer::try_lock(current_array.bucket(i)) {
                    if writer.len() == 0 {
                        // The bucket will be unlocked later.
                        writer_guard.0 = i + 1;
                        forget(writer);
                        return false;
                    }
                }
                true
            }) {
                // All the buckets are empty and locked.
                writer_guard.1 = true;
                if let Some(bucket_array) =
                    self.bucket_array_var().swap((None, Tag::None), Release).0
                {
                    self.defer_reclaim(bucket_array, guard);
                }
            }
        } else if try_resize {
            // Recheck the sampling result before allocating a new bucket array.
            //
            // By sampling buckets at the end of the bucket array, the effect of mutable iterators
            // on sampling is minimized.
            let sample_size = current_array.full_sample_size();
            let estimation = Self::sample(
                current_array,
                current_array.len() - sample_size,
                sample_size,
            );
            let new_capacity = BucketArray::<K, V, L, TYPE>::optimal_capacity(
                capacity,
                estimation,
                minimum_capacity,
                maximum_capacity,
            );
            if new_capacity != capacity {
                let new_bucket_array = unsafe {
                    Shared::new_with_unchecked(|| {
                        BucketArray::<K, V, L, TYPE>::new(
                            new_capacity,
                            (*self.bucket_array_var()).clone(Relaxed, guard),
                        )
                    })
                };
                self.bucket_array_var()
                    .swap((Some(new_bucket_array), Tag::None), Release);
            }
        }
    }

    /// Returns an estimated required size of the container based on the size hint.
    #[inline]
    fn capacity_from_size_hint(size_hint: (usize, Option<usize>)) -> usize {
        // A resize can be triggered when the load factor reaches ~80%.
        (size_hint
            .1
            .unwrap_or(size_hint.0)
            .min(1_usize << (usize::BITS - 2))
            / 4)
            * 5
    }
}

/// Hard limit of the maximum capacity of each container type.
pub(super) const MAXIMUM_CAPACITY_LIMIT: usize = 1_usize << (usize::BITS - 2);

/// Denotes a state where a thread is resizing the container.
pub(super) const RESIZING: usize = 1_usize << (usize::BITS - 1);

/// [`LockedBucket`] has exclusive access to a [`Bucket`].
pub(crate) struct LockedBucket<K, V, L: LruList, const TYPE: char> {
    /// Holds an exclusive lock on the [`Bucket`].
    pub writer: Writer<K, V, L, TYPE>,
    /// Corresponding [`DataBlock`].
    pub data_block: NonNull<DataBlock<K, V, BUCKET_LEN>>,
    /// The index of the [`Bucket`] within the [`BucketArray`].
    pub bucket_index: usize,
    /// Corresponding [`BucketArray`].
    ///
    /// The [`BucketArray`] is not dropped as long as it holds an exclusive lock on the [`Bucket`].
    pub bucket_array: NonNull<BucketArray<K, V, L, TYPE>>,
}

impl<K, V, L: LruList, const TYPE: char> LockedBucket<K, V, L, TYPE> {
    /// Returns a reference to the [`BucketArray`] that contains this [`LockedBucket`].
    #[inline]
    pub(crate) const fn bucket_array(&self) -> &BucketArray<K, V, L, TYPE> {
        unsafe { self.bucket_array.as_ref() }
    }

    /// Gets a mutable reference to the entry.
    #[inline]
    pub(crate) fn entry(&self, entry_ptr: &EntryPtr<K, V, TYPE>) -> &(K, V) {
        entry_ptr.get(self.data_block)
    }

    /// Gets a mutable reference to the entry.
    #[inline]
    pub(crate) fn entry_mut<'b>(
        &'b mut self,
        entry_ptr: &'b mut EntryPtr<K, V, TYPE>,
    ) -> &'b mut (K, V) {
        entry_ptr.get_mut(self.data_block, &self.writer)
    }

    /// Inserts a new entry with the supplied constructor function.
    #[inline]
    pub(crate) fn insert(&self, hash: u64, entry: (K, V)) -> EntryPtr<K, V, TYPE> {
        self.writer.insert(self.data_block, hash, entry)
    }
}

impl<K: Eq + Hash, V, L: LruList, const TYPE: char> LockedBucket<K, V, L, TYPE> {
    /// Searches for an entry with the given key.
    #[inline]
    pub(crate) fn search<Q>(&self, key: &Q, hash: u64) -> EntryPtr<K, V, TYPE>
    where
        Q: Equivalent<K> + ?Sized,
    {
        (*self.writer).get_entry_ptr(self.data_block, key, hash)
    }

    /// Removes the entry and tries to shrink the container.
    #[inline]
    pub(crate) fn remove<H, T: HashTable<K, V, H, L, TYPE>>(
        self,
        hash_table: &T,
        entry_ptr: &mut EntryPtr<K, V, TYPE>,
    ) -> (K, V)
    where
        H: BuildHasher,
    {
        let removed = self.writer.remove(self.data_block, entry_ptr);
        if self.len() == 0 {
            self.try_shrink(hash_table, &Guard::new());
        }
        removed
    }

    /// Removes the entry and tries to shrink the container.
    #[inline]
    pub(crate) fn mark_removed<H, T: HashTable<K, V, H, L, TYPE>>(
        self,
        hash_table: &T,
        entry_ptr: &mut EntryPtr<K, V, TYPE>,
    ) where
        H: BuildHasher,
    {
        debug_assert_eq!(TYPE, INDEX);

        let guard = Guard::new();
        self.writer.mark_removed(entry_ptr, &guard);
        self.set_has_garbage(&guard);
        if self.writer.len() == 0 {
            self.try_shrink(hash_table, &guard);
        }
    }

    /// Sets that there can be a garbage entry in the bucket so the epoch should be advanced.
    #[inline]
    pub(crate) const fn set_has_garbage(&self, guard: &Guard) {
        let sample_size = self.bucket_array().large_sample_size();
        if self.bucket_index % (sample_size * sample_size) == 0 {
            guard.set_has_garbage();
        }
    }

    /// Tries to shrink the container.
    #[inline]
    pub(crate) fn try_shrink<H, T: HashTable<K, V, H, L, TYPE>>(self, hash_table: &T, guard: &Guard)
    where
        H: BuildHasher,
    {
        if let Some(current_array) = hash_table.bucket_array(guard) {
            if ptr::eq(current_array, self.bucket_array()) {
                let bucket_index = self.bucket_index;
                drop(self);

                // Tries to shrink the container after unlocking the bucket.
                hash_table.try_shrink(current_array, bucket_index, guard);
            }
        }
    }

    /// Returns a [`LockedBucket`] owning the next bucket asynchronously.
    #[inline]
    pub(super) async fn next_async<H, T: HashTable<K, V, H, L, TYPE>>(
        self,
        hash_table: &T,
        entry_ptr: &mut EntryPtr<K, V, TYPE>,
    ) -> Option<LockedBucket<K, V, L, TYPE>>
    where
        H: BuildHasher,
    {
        if entry_ptr.find_next(&self.writer) {
            return Some(self);
        }

        let next_index = self.bucket_index + 1;
        let len = self.bucket_array().len();

        if self.writer.len() == 0 {
            self.try_shrink(hash_table, &Guard::new());
        } else {
            drop(self);
        }

        if next_index == len {
            return None;
        }

        let mut next_entry = None;
        hash_table
            .for_each_writer_async(next_index, len, |locked_bucket, _| {
                *entry_ptr = EntryPtr::null();
                if entry_ptr.find_next(&locked_bucket.writer) {
                    next_entry = Some(locked_bucket);
                    return false;
                }
                true
            })
            .await;

        next_entry
    }

    /// Returns a [`LockedBucket`] owning the next bucket synchronously.
    #[inline]
    pub(super) fn next_sync<H, T: HashTable<K, V, H, L, TYPE>>(
        self,
        hash_table: &T,
        entry_ptr: &mut EntryPtr<K, V, TYPE>,
    ) -> Option<Self>
    where
        H: BuildHasher,
    {
        if entry_ptr.find_next(&self.writer) {
            return Some(self);
        }

        let next_index = self.bucket_index + 1;
        let len = self.bucket_array().len();

        if self.writer.len() == 0 {
            self.try_shrink(hash_table, &Guard::new());
        } else {
            drop(self);
        }

        if next_index == len {
            return None;
        }

        let mut next_entry = None;
        hash_table.for_each_writer_sync(next_index, len, &Guard::new(), |locked_bucket, _| {
            *entry_ptr = EntryPtr::null();
            if entry_ptr.find_next(&locked_bucket.writer) {
                next_entry = Some(locked_bucket);
                return false;
            }
            true
        });

        next_entry
    }
}

impl<K, V, L: LruList, const TYPE: char> Deref for LockedBucket<K, V, L, TYPE> {
    type Target = Bucket<K, V, L, TYPE>;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &self.writer
    }
}

unsafe impl<K: Send, V: Send, L: LruList, const TYPE: char> Send for LockedBucket<K, V, L, TYPE> {}
unsafe impl<K: Send + Sync, V: Send + Sync, L: LruList, const TYPE: char> Sync
    for LockedBucket<K, V, L, TYPE>
{
}

/// For the given index in the current array, calculate the respective range in the old array.
#[inline]
const fn from_index_to_range(from_len: usize, to_len: usize, from_index: usize) -> (usize, usize) {
    debug_assert!(from_len.is_power_of_two() && to_len.is_power_of_two());
    if from_len < to_len {
        let ratio = to_len / from_len;
        let start_index = from_index * ratio;
        debug_assert!(start_index + ratio <= to_len,);
        (start_index, start_index + ratio)
    } else {
        let ratio = from_len / to_len;
        let start_index = from_index / ratio;
        debug_assert!(start_index < to_len,);
        (start_index, start_index + 1)
    }
}

/// Turns a reference into a [`NonNull`] pointer.
#[inline]
const fn into_non_null<T: Sized>(t: &T) -> NonNull<T> {
    unsafe { NonNull::new_unchecked(from_ref(t).cast_mut()) }
}
