use super::bucket::{self, Bucket, BucketArray, InsertOrModifyState, RehashOp};

use std::{
    hash::{BuildHasher, Hash},
    sync::atomic::{AtomicUsize, Ordering},
};

use crossbeam_epoch::{Atomic, CompareExchangeError, Guard, Owned, Shared};

pub(crate) struct BucketArrayRef<'a, K, V, S> {
    pub(crate) bucket_array: &'a Atomic<BucketArray<K, V>>,
    pub(crate) build_hasher: &'a S,
    pub(crate) len: &'a AtomicUsize,
}

impl<K, V, S> BucketArrayRef<'_, K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    pub(crate) fn get_key_value_and_then<T>(
        &self,
        hash: u64,
        mut eq: impl FnMut(&K) -> bool,
        with_entry: impl FnOnce(&K, &V) -> Option<T>,
    ) -> Option<T> {
        let guard = &crossbeam_epoch::pin();
        let current_ref = self.get(guard);
        let mut bucket_array_ref = current_ref;

        let result;

        loop {
            match bucket_array_ref
                .get(guard, hash, &mut eq)
                .map(|p| unsafe { p.as_ref() })
            {
                Ok(Some(Bucket {
                    key,
                    maybe_value: value,
                })) => {
                    result = with_entry(key, unsafe { &*value.as_ptr() });
                    break;
                }
                Ok(None) => {
                    result = None;
                    break;
                }
                Err(_) => {
                    if let Some(r) =
                        bucket_array_ref.rehash(guard, self.build_hasher, RehashOp::Expand)
                    {
                        bucket_array_ref = r;
                    }
                }
            }
        }

        self.swing(guard, current_ref, bucket_array_ref);

        result
    }

    pub(crate) fn remove_entry_if_and<T>(
        &self,
        hash: u64,
        mut eq: impl FnMut(&K) -> bool,
        mut condition: impl FnMut(&K, &V) -> bool,
        with_previous_entry: impl FnOnce(&K, &V) -> T,
    ) -> Option<T> {
        let guard = &crossbeam_epoch::pin();
        let current_ref = self.get(guard);
        let mut bucket_array_ref = current_ref;

        let result;

        loop {
            loop {
                let rehash_op = RehashOp::new(
                    bucket_array_ref.capacity(),
                    &bucket_array_ref.tombstone_count,
                    self.len,
                );
                if rehash_op.is_skip() {
                    break;
                }
                if let Some(r) = bucket_array_ref.rehash(guard, self.build_hasher, rehash_op) {
                    bucket_array_ref = r;
                }
            }

            match bucket_array_ref.remove_if(guard, hash, &mut eq, condition) {
                Ok(previous_bucket_ptr) => {
                    if let Some(previous_bucket_ref) = unsafe { previous_bucket_ptr.as_ref() } {
                        let Bucket {
                            key,
                            maybe_value: value,
                        } = previous_bucket_ref;
                        self.len.fetch_sub(1, Ordering::Relaxed);
                        bucket_array_ref
                            .tombstone_count
                            .fetch_add(1, Ordering::Relaxed);
                        result = Some(with_previous_entry(key, unsafe { &*value.as_ptr() }));

                        unsafe { bucket::defer_destroy_tombstone(guard, previous_bucket_ptr) };
                    } else {
                        result = None;
                    }

                    break;
                }
                Err(c) => {
                    condition = c;
                    if let Some(r) =
                        bucket_array_ref.rehash(guard, self.build_hasher, RehashOp::Expand)
                    {
                        bucket_array_ref = r;
                    }
                }
            }
        }

        self.swing(guard, current_ref, bucket_array_ref);

        result
    }

    pub(crate) fn insert_if_not_present_and<T>(
        &self,
        key: K,
        hash: u64,
        on_insert: impl FnOnce() -> V,
        with_existing_entry: impl FnOnce(&K, &V) -> T,
    ) -> Option<T> {
        use bucket::InsertionResult;

        let guard = &crossbeam_epoch::pin();
        let current_ref = self.get(guard);
        let mut bucket_array_ref = current_ref;
        let mut state = InsertOrModifyState::New(key, on_insert);

        let result;

        loop {
            loop {
                let rehash_op = RehashOp::new(
                    bucket_array_ref.capacity(),
                    &bucket_array_ref.tombstone_count,
                    self.len,
                );
                if rehash_op.is_skip() {
                    break;
                }
                if let Some(r) = bucket_array_ref.rehash(guard, self.build_hasher, rehash_op) {
                    bucket_array_ref = r;
                }
            }

            match bucket_array_ref.insert_if_not_present(guard, hash, state) {
                Ok(InsertionResult::AlreadyPresent(current_bucket_ptr)) => {
                    let current_bucket_ref = unsafe { current_bucket_ptr.as_ref() }.unwrap();
                    assert!(!bucket::is_tombstone(current_bucket_ptr));
                    let Bucket {
                        key,
                        maybe_value: value,
                    } = current_bucket_ref;
                    result = Some(with_existing_entry(key, unsafe { &*value.as_ptr() }));
                    break;
                }
                Ok(InsertionResult::Inserted) => {
                    self.len.fetch_add(1, Ordering::Relaxed);
                    result = None;
                    break;
                }
                Ok(InsertionResult::ReplacedTombstone(previous_bucket_ptr)) => {
                    assert!(bucket::is_tombstone(previous_bucket_ptr));
                    self.len.fetch_add(1, Ordering::Relaxed);
                    unsafe { bucket::defer_destroy_bucket(guard, previous_bucket_ptr) };
                    result = None;
                    break;
                }
                Err(s) => {
                    state = s;
                    if let Some(r) =
                        bucket_array_ref.rehash(guard, self.build_hasher, RehashOp::Expand)
                    {
                        bucket_array_ref = r;
                    }
                }
            }
        }

        self.swing(guard, current_ref, bucket_array_ref);

        result
    }

    pub(crate) fn insert_with_or_modify_entry_and<T>(
        &self,
        key: K,
        hash: u64,
        on_insert: impl FnOnce() -> V,
        mut on_modify: impl FnMut(&K, &V) -> V,
        with_old_entry: impl FnOnce(&K, &V) -> T,
    ) -> Option<T> {
        let guard = &crossbeam_epoch::pin();
        let current_ref = self.get(guard);
        let mut bucket_array_ref = current_ref;
        let mut state = InsertOrModifyState::New(key, on_insert);

        let result;

        loop {
            loop {
                let rehash_op = RehashOp::new(
                    bucket_array_ref.capacity(),
                    &bucket_array_ref.tombstone_count,
                    self.len,
                );
                if rehash_op.is_skip() {
                    break;
                }
                if let Some(r) = bucket_array_ref.rehash(guard, self.build_hasher, rehash_op) {
                    bucket_array_ref = r;
                }
            }

            match bucket_array_ref.insert_or_modify(guard, hash, state, on_modify) {
                Ok(previous_bucket_ptr) => {
                    if let Some(previous_bucket_ref) = unsafe { previous_bucket_ptr.as_ref() } {
                        if bucket::is_tombstone(previous_bucket_ptr) {
                            self.len.fetch_add(1, Ordering::Relaxed);
                            result = None;
                        } else {
                            let Bucket {
                                key,
                                maybe_value: value,
                            } = previous_bucket_ref;
                            result = Some(with_old_entry(key, unsafe { &*value.as_ptr() }));
                        }

                        unsafe { bucket::defer_destroy_bucket(guard, previous_bucket_ptr) };
                    } else {
                        self.len.fetch_add(1, Ordering::Relaxed);
                        result = None;
                    }

                    break;
                }
                Err((s, f)) => {
                    state = s;
                    on_modify = f;
                    if let Some(r) =
                        bucket_array_ref.rehash(guard, self.build_hasher, RehashOp::Expand)
                    {
                        bucket_array_ref = r;
                    }
                }
            }
        }

        self.swing(guard, current_ref, bucket_array_ref);

        result
    }

    pub(crate) fn keys<T>(&self, mut with_key: impl FnMut(&K) -> T) -> Vec<T> {
        let guard = &crossbeam_epoch::pin();
        let current_ref = self.get(guard);
        let mut bucket_array_ref = current_ref;

        let result;

        loop {
            match bucket_array_ref.keys(guard, &mut with_key) {
                Ok(keys) => {
                    result = keys;
                    break;
                }
                Err(_) => {
                    if let Some(r) =
                        bucket_array_ref.rehash(guard, self.build_hasher, RehashOp::Expand)
                    {
                        bucket_array_ref = r;
                    }
                }
            }
        }

        self.swing(guard, current_ref, bucket_array_ref);

        result
    }
}

impl<'g, K, V, S> BucketArrayRef<'_, K, V, S> {
    fn get(&self, guard: &'g Guard) -> &'g BucketArray<K, V> {
        let mut maybe_new_bucket_array = None;

        loop {
            let bucket_array_ptr = self.bucket_array.load_consume(guard);

            if let Some(bucket_array_ref) = unsafe { bucket_array_ptr.as_ref() } {
                return bucket_array_ref;
            }

            let new_bucket_array =
                maybe_new_bucket_array.unwrap_or_else(|| Owned::new(BucketArray::default()));

            match self.bucket_array.compare_exchange_weak(
                Shared::null(),
                new_bucket_array,
                Ordering::AcqRel,
                Ordering::Relaxed,
                guard,
            ) {
                Ok(b) => return unsafe { b.as_ref() }.unwrap(),
                Err(CompareExchangeError { new, .. }) => maybe_new_bucket_array = Some(new),
            }
        }
    }

    fn swing(
        &self,
        guard: &'g Guard,
        mut current_ref: &'g BucketArray<K, V>,
        min_ref: &'g BucketArray<K, V>,
    ) {
        let min_epoch = min_ref.epoch;

        let mut current_ptr = (current_ref as *const BucketArray<K, V>).into();
        let min_ptr: Shared<'g, _> = (min_ref as *const BucketArray<K, V>).into();

        loop {
            if current_ref.epoch >= min_epoch {
                return;
            }

            match self.bucket_array.compare_exchange_weak(
                current_ptr,
                min_ptr,
                Ordering::AcqRel,
                Ordering::Relaxed,
                guard,
            ) {
                Ok(_) => unsafe { bucket::defer_acquire_destroy(guard, current_ptr) },
                Err(_) => {
                    let new_ptr = self.bucket_array.load_consume(guard);
                    assert!(!new_ptr.is_null());

                    current_ptr = new_ptr;
                    current_ref = unsafe { new_ptr.as_ref() }.unwrap();
                }
            }
        }
    }
}
