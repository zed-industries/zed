use std::{
    hash::{BuildHasher, Hash, Hasher},
    mem::{self, MaybeUninit},
    ptr,
    sync::{
        atomic::{self, AtomicUsize, Ordering},
        Arc, Mutex, TryLockError,
    },
};

#[cfg(feature = "unstable-debug-counters")]
use crate::common::concurrent::debug_counters;

use crossbeam_epoch::{Atomic, CompareExchangeError, Guard, Owned, Shared};

pub(crate) const BUCKET_ARRAY_DEFAULT_LENGTH: usize = 128;

pub(crate) struct BucketArray<K, V> {
    pub(crate) buckets: Box<[Atomic<Bucket<K, V>>]>,
    pub(crate) next: Atomic<BucketArray<K, V>>,
    pub(crate) epoch: usize,
    pub(crate) rehash_lock: Arc<Mutex<()>>,
    pub(crate) tombstone_count: AtomicUsize,
}

impl<K, V> Default for BucketArray<K, V> {
    fn default() -> Self {
        Self::with_length(0, BUCKET_ARRAY_DEFAULT_LENGTH)
    }
}

impl<K, V> BucketArray<K, V> {
    pub(crate) fn with_length(epoch: usize, length: usize) -> Self {
        assert!(length.is_power_of_two());
        let mut buckets = Vec::with_capacity(length);

        unsafe {
            ptr::write_bytes(buckets.as_mut_ptr(), 0, length);
            buckets.set_len(length);
        }

        let buckets = buckets.into_boxed_slice();

        #[cfg(feature = "unstable-debug-counters")]
        {
            use debug_counters::InternalGlobalDebugCounters as Counters;

            let size = (buckets.len() * std::mem::size_of::<Atomic<Bucket<K, V>>>()) as u64;
            Counters::bucket_array_created(size);
        }

        Self {
            buckets,
            next: Atomic::null(),
            epoch,
            rehash_lock: Arc::new(Mutex::new(())),
            tombstone_count: AtomicUsize::default(),
        }
    }

    pub(crate) fn capacity(&self) -> usize {
        assert!(self.buckets.len().is_power_of_two());

        self.buckets.len() / 2
    }
}

#[cfg(feature = "unstable-debug-counters")]
impl<K, V> Drop for BucketArray<K, V> {
    fn drop(&mut self) {
        use debug_counters::InternalGlobalDebugCounters as Counters;

        let size = (self.buckets.len() * std::mem::size_of::<Atomic<Bucket<K, V>>>()) as u64;
        Counters::bucket_array_dropped(size);
    }
}

impl<'g, K: 'g + Eq, V: 'g> BucketArray<K, V> {
    pub(crate) fn get(
        &self,
        guard: &'g Guard,
        hash: u64,
        mut eq: impl FnMut(&K) -> bool,
    ) -> Result<Shared<'g, Bucket<K, V>>, RelocatedError> {
        for bucket in self.probe(guard, hash) {
            let Ok((_, _, this_bucket_ptr)) = bucket else {
                return Err(RelocatedError);
            };

            let Some(this_bucket_ref) = (unsafe { this_bucket_ptr.as_ref() }) else {
                // Not found.
                return Ok(Shared::null());
            };

            if !eq(&this_bucket_ref.key) {
                // Different key. Try next bucket
                continue;
            }

            if is_tombstone(this_bucket_ptr) {
                // Not found. (It has been removed)
                return Ok(Shared::null());
            } else {
                // Found.
                return Ok(this_bucket_ptr);
            }
        }

        Ok(Shared::null())
    }

    pub(crate) fn remove_if<F>(
        &self,
        guard: &'g Guard,
        hash: u64,
        mut eq: impl FnMut(&K) -> bool,
        mut condition: F,
    ) -> Result<Shared<'g, Bucket<K, V>>, F>
    where
        F: FnMut(&K, &V) -> bool,
    {
        let mut probe = self.probe(guard, hash);
        while let Some(bucket) = probe.next() {
            let Ok((_, this_bucket, this_bucket_ptr)) = bucket else {
                return Err(condition);
            };

            let Some(this_bucket_ref) = (unsafe { this_bucket_ptr.as_ref() }) else {
                // Nothing to remove.
                return Ok(Shared::null());
            };

            let this_key = &this_bucket_ref.key;

            if !eq(this_key) {
                // Different key. Try next bucket.
                continue;
            }

            if is_tombstone(this_bucket_ptr) {
                // Already removed.
                return Ok(Shared::null());
            }

            let this_value = unsafe { &*this_bucket_ref.maybe_value.as_ptr() };

            if !condition(this_key, this_value) {
                // Found but the condition is false. Do not remove.
                return Ok(Shared::null());
            }

            // Found and the condition is true. Remove it. (Make it a tombstone)

            let new_bucket_ptr = this_bucket_ptr.with_tag(TOMBSTONE_TAG);

            match this_bucket.compare_exchange_weak(
                this_bucket_ptr,
                new_bucket_ptr,
                Ordering::AcqRel,
                Ordering::Relaxed,
                guard,
            ) {
                // Succeeded. Return the removed value. (can be null)
                Ok(_) => return Ok(new_bucket_ptr),
                // Failed. Reload to retry.
                Err(_) => probe.reload(),
            }
        }

        Ok(Shared::null())
    }

    pub(crate) fn insert_if_not_present<F>(
        &self,
        guard: &'g Guard,
        hash: u64,
        mut state: InsertOrModifyState<K, V, F>,
    ) -> Result<InsertionResult<'g, K, V>, InsertOrModifyState<K, V, F>>
    where
        F: FnOnce() -> V,
    {
        let mut probe = self.probe(guard, hash);
        while let Some(Ok((_, this_bucket, this_bucket_ptr))) = probe.next() {
            if let Some(this_bucket_ref) = unsafe { this_bucket_ptr.as_ref() } {
                if &this_bucket_ref.key != state.key() {
                    // Different key. Try next bucket.
                    continue;
                }

                if !is_tombstone(this_bucket_ptr) {
                    // Found. Return it.
                    return Ok(InsertionResult::AlreadyPresent(this_bucket_ptr));
                }
            }

            // Not found or found a tombstone. Insert it.

            let new_bucket = state.into_insert_bucket();

            if let Err(CompareExchangeError { new, .. }) = this_bucket.compare_exchange_weak(
                this_bucket_ptr,
                new_bucket,
                Ordering::AcqRel,
                Ordering::Relaxed,
                guard,
            ) {
                state = InsertOrModifyState::from_bucket_value(new, None);
                probe.reload();
            } else if unsafe { this_bucket_ptr.as_ref() }.is_some() {
                // Inserted by replacing a tombstone.
                return Ok(InsertionResult::ReplacedTombstone(this_bucket_ptr));
            } else {
                // Inserted.
                return Ok(InsertionResult::Inserted);
            }
        }

        Err(state)
    }

    // https://rust-lang.github.io/rust-clippy/master/index.html#type_complexity
    #[allow(clippy::type_complexity)]
    pub(crate) fn insert_or_modify<F, G>(
        &self,
        guard: &'g Guard,
        hash: u64,
        mut state: InsertOrModifyState<K, V, F>,
        mut modifier: G,
    ) -> Result<Shared<'g, Bucket<K, V>>, (InsertOrModifyState<K, V, F>, G)>
    where
        F: FnOnce() -> V,
        G: FnMut(&K, &V) -> V,
    {
        let mut probe = self.probe(guard, hash);
        while let Some(bucket) = probe.next() {
            let Ok((_, this_bucket, this_bucket_ptr)) = bucket else {
                return Err((state, modifier));
            };

            let (new_bucket, maybe_insert_value) =
                if let Some(this_bucket_ref) = unsafe { this_bucket_ptr.as_ref() } {
                    let this_key = &this_bucket_ref.key;

                    if this_key != state.key() {
                        // Different key. Try next bucket.
                        continue;
                    }

                    if is_tombstone(this_bucket_ptr) {
                        // Found a tombstone for this key. Replace it.
                        (state.into_insert_bucket(), None)
                    } else {
                        // Found. Modify it.
                        let this_value = unsafe { &*this_bucket_ref.maybe_value.as_ptr() };
                        let new_value = modifier(this_key, this_value);

                        let (new_bucket, insert_value) = state.into_modify_bucket(new_value);

                        (new_bucket, Some(insert_value))
                    }
                } else {
                    // Not found. Insert it.
                    (state.into_insert_bucket(), None)
                };

            if let Err(CompareExchangeError { new, .. }) = this_bucket.compare_exchange_weak(
                this_bucket_ptr,
                new_bucket,
                Ordering::AcqRel,
                Ordering::Relaxed,
                guard,
            ) {
                // Failed. Reload to retry.
                state = InsertOrModifyState::from_bucket_value(new, maybe_insert_value);
                probe.reload();
            } else {
                // Succeeded. Return the previous value. (can be null)
                return Ok(this_bucket_ptr);
            }
        }

        Err((state, modifier))
    }

    fn insert_for_grow(
        &self,
        guard: &'g Guard,
        hash: u64,
        bucket_ptr: Shared<'g, Bucket<K, V>>,
    ) -> Option<usize> {
        assert!(!bucket_ptr.is_null());
        assert!(!is_sentinel(bucket_ptr));
        assert!(is_borrowed(bucket_ptr));

        let key = &unsafe { bucket_ptr.deref() }.key;

        let mut probe = self.probe(guard, hash);
        while let Some(bucket) = probe.next() {
            let Ok((i, this_bucket, this_bucket_ptr)) = bucket else {
                return None;
            };

            if let Some(Bucket { key: this_key, .. }) = unsafe { this_bucket_ptr.as_ref() } {
                if this_bucket_ptr == bucket_ptr {
                    return None;
                } else if this_key != key {
                    continue;
                } else if !is_borrowed(this_bucket_ptr) {
                    return None;
                }
            }

            if this_bucket_ptr.is_null() && is_tombstone(bucket_ptr) {
                return None;
            } else if this_bucket
                .compare_exchange_weak(
                    this_bucket_ptr,
                    bucket_ptr,
                    Ordering::AcqRel,
                    Ordering::Relaxed,
                    guard,
                )
                .is_ok()
            {
                return Some(i);
            } else {
                probe.reload();
            }
        }

        None
    }

    pub(crate) fn keys<F, T>(
        &self,
        guard: &'g Guard,
        with_key: &mut F,
    ) -> Result<Vec<T>, RelocatedError>
    where
        F: FnMut(&K) -> T,
    {
        let mut keys = Vec::new();

        for bucket in self.buckets.iter() {
            let bucket_ptr = bucket.load_consume(guard);

            if is_sentinel(bucket_ptr) {
                return Err(RelocatedError);
            }

            if let Some(bucket_ref) = unsafe { bucket_ptr.as_ref() } {
                if !is_tombstone(bucket_ptr) {
                    keys.push(with_key(&bucket_ref.key));
                }
            }
        }

        Ok(keys)
    }
}

struct Probe<'b, 'g, K: 'g, V: 'g> {
    buckets: &'b [Atomic<Bucket<K, V>>],
    guard: &'g Guard,
    this_bucket: (usize, &'b Atomic<Bucket<K, V>>),
    offset: usize,

    i: usize,
    reload: bool,
}

impl<'g, K: 'g, V: 'g> Probe<'_, 'g, K, V> {
    fn reload(&mut self) {
        self.reload = true;
    }
}

impl<'b, 'g, K: 'g, V: 'g> Iterator for Probe<'b, 'g, K, V> {
    type Item = Result<(usize, &'b Atomic<Bucket<K, V>>, Shared<'g, Bucket<K, V>>), ()>;

    fn next(&mut self) -> Option<Self::Item> {
        if !self.reload {
            let max = self.buckets.len() - 1;
            if self.i >= max {
                return None;
            }
            self.i += 1;
            let i = self.i.wrapping_add(self.offset) & max;
            self.this_bucket = (i, &self.buckets[i]);
        }
        self.reload = false;

        let this_bucket_ptr = self.this_bucket.1.load_consume(self.guard);

        if is_sentinel(this_bucket_ptr) {
            return Some(Err(()));
        }

        let val = (self.this_bucket.0, self.this_bucket.1, this_bucket_ptr);
        Some(Ok(val))
    }
}

impl<'g, K: 'g, V: 'g> BucketArray<K, V> {
    fn probe(&self, guard: &'g Guard, hash: u64) -> Probe<'_, 'g, K, V> {
        let buckets = &self.buckets;
        let offset = hash as usize & (buckets.len() - 1);
        // SAFETY: `len()` is never be 0 so this index access will never panic.
        // This invariant is ensured by the `assert!()` at the beginning of
        // `with_length()` because 0 is not a power of two.
        let this_bucket = (offset, &buckets[offset]);
        Probe {
            buckets,
            guard,
            this_bucket,
            offset,

            i: 0,
            reload: true,
        }
    }

    pub(crate) fn rehash<H>(
        &self,
        guard: &'g Guard,
        build_hasher: &H,
        rehash_op: RehashOp,
    ) -> Option<&'g BucketArray<K, V>>
    where
        K: Hash + Eq,
        H: BuildHasher,
    {
        // Ensure that the rehashing is not performed concurrently.
        let lock = match self.rehash_lock.try_lock() {
            Ok(lk) => lk,
            Err(TryLockError::WouldBlock) => {
                // Wait until the lock become available.
                std::mem::drop(self.rehash_lock.lock());
                // We need to return here to see if rehashing is still needed.
                return None;
            }
            Err(e @ TryLockError::Poisoned(_)) => panic!("{e:?}"),
        };

        let next_array = self.next_array(guard, rehash_op);

        for this_bucket in self.buckets.iter() {
            let mut maybe_state: Option<(usize, Shared<'g, Bucket<K, V>>)> = None;

            loop {
                let this_bucket_ptr = this_bucket.load_consume(guard);

                if is_sentinel(this_bucket_ptr) {
                    break;
                }

                let to_put_ptr = this_bucket_ptr.with_tag(this_bucket_ptr.tag() | BORROWED_TAG);

                if let Some((index, mut next_bucket_ptr)) = maybe_state {
                    assert!(!this_bucket_ptr.is_null());

                    let next_bucket = &next_array.buckets[index];

                    while is_borrowed(next_bucket_ptr)
                        && next_bucket
                            .compare_exchange_weak(
                                next_bucket_ptr,
                                to_put_ptr,
                                Ordering::AcqRel,
                                Ordering::Relaxed,
                                guard,
                            )
                            .is_err()
                    {
                        next_bucket_ptr = next_bucket.load_consume(guard);
                    }
                } else if let Some(this_bucket_ref) = unsafe { this_bucket_ptr.as_ref() } {
                    let key = &this_bucket_ref.key;
                    let hash = hash(build_hasher, key);

                    if let Some(index) = next_array.insert_for_grow(guard, hash, to_put_ptr) {
                        maybe_state = Some((index, to_put_ptr));
                    }
                }

                if this_bucket
                    .compare_exchange_weak(
                        this_bucket_ptr,
                        Shared::null().with_tag(SENTINEL_TAG),
                        Ordering::AcqRel,
                        Ordering::Relaxed,
                        guard,
                    )
                    .is_ok()
                {
                    // TODO: If else, we may need to count tombstone.
                    if !this_bucket_ptr.is_null()
                        && is_tombstone(this_bucket_ptr)
                        && maybe_state.is_none()
                    {
                        unsafe { defer_destroy_bucket(guard, this_bucket_ptr) };
                    }

                    break;
                }
            }
        }

        guard.flush();
        std::mem::drop(lock);

        Some(next_array)
    }

    fn next_array(&self, guard: &'g Guard, rehash_op: RehashOp) -> &'g BucketArray<K, V> {
        let mut maybe_new_next = None;

        loop {
            let next_ptr = self.next.load_consume(guard);

            if let Some(next_ref) = unsafe { next_ptr.as_ref() } {
                return next_ref;
            }

            let new_length = rehash_op.new_len(self.buckets.len());
            let new_next = maybe_new_next.unwrap_or_else(|| {
                Owned::new(BucketArray::with_length(self.epoch + 1, new_length))
            });

            match self.next.compare_exchange_weak(
                Shared::null(),
                new_next,
                Ordering::AcqRel,
                Ordering::Relaxed,
                guard,
            ) {
                Ok(p) => return unsafe { p.deref() },
                Err(CompareExchangeError { new, .. }) => {
                    maybe_new_next = Some(new);
                }
            }
        }
    }
}

#[repr(align(8))]
#[derive(Debug)]
pub(crate) struct Bucket<K, V> {
    pub(crate) key: K,
    pub(crate) maybe_value: MaybeUninit<V>,
}

impl<K, V> Bucket<K, V> {
    pub(crate) fn new(key: K, value: V) -> Bucket<K, V> {
        #[cfg(feature = "unstable-debug-counters")]
        debug_counters::InternalGlobalDebugCounters::bucket_created();

        Self {
            key,
            maybe_value: MaybeUninit::new(value),
        }
    }
}

#[cfg(feature = "unstable-debug-counters")]
impl<K, V> Drop for Bucket<K, V> {
    fn drop(&mut self) {
        debug_counters::InternalGlobalDebugCounters::bucket_dropped();
    }
}

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct RelocatedError;

pub(crate) enum InsertOrModifyState<K, V, F: FnOnce() -> V> {
    New(K, F),
    AttemptedInsertion(Owned<Bucket<K, V>>),
    AttemptedModification(Owned<Bucket<K, V>>, ValueOrFunction<V, F>),
}

impl<K, V, F: FnOnce() -> V> InsertOrModifyState<K, V, F> {
    fn from_bucket_value(
        bucket: Owned<Bucket<K, V>>,
        value_or_function: Option<ValueOrFunction<V, F>>,
    ) -> Self {
        if let Some(value_or_function) = value_or_function {
            Self::AttemptedModification(bucket, value_or_function)
        } else {
            Self::AttemptedInsertion(bucket)
        }
    }

    fn key(&self) -> &K {
        match self {
            InsertOrModifyState::New(k, _) => k,
            InsertOrModifyState::AttemptedInsertion(b)
            | InsertOrModifyState::AttemptedModification(b, _) => &b.key,
        }
    }

    fn into_insert_bucket(self) -> Owned<Bucket<K, V>> {
        match self {
            InsertOrModifyState::New(k, f) => Owned::new(Bucket::new(k, f())),
            InsertOrModifyState::AttemptedInsertion(b) => b,
            InsertOrModifyState::AttemptedModification(mut b, v_or_f) => {
                unsafe {
                    mem::drop(
                        mem::replace(&mut b.maybe_value, MaybeUninit::new(v_or_f.into_value()))
                            .assume_init(),
                    );
                };

                b
            }
        }
    }

    fn into_modify_bucket(self, value: V) -> (Owned<Bucket<K, V>>, ValueOrFunction<V, F>) {
        match self {
            InsertOrModifyState::New(k, f) => (
                Owned::new(Bucket::new(k, value)),
                ValueOrFunction::Function(f),
            ),
            InsertOrModifyState::AttemptedInsertion(mut b) => {
                let insert_value = unsafe {
                    mem::replace(&mut b.maybe_value, MaybeUninit::new(value)).assume_init()
                };

                (b, ValueOrFunction::Value(insert_value))
            }
            InsertOrModifyState::AttemptedModification(mut b, v_or_f) => {
                unsafe {
                    mem::drop(
                        mem::replace(&mut b.maybe_value, MaybeUninit::new(value)).assume_init(),
                    );
                }

                (b, v_or_f)
            }
        }
    }
}

pub(crate) enum ValueOrFunction<V, F: FnOnce() -> V> {
    Value(V),
    Function(F),
}

impl<V, F: FnOnce() -> V> ValueOrFunction<V, F> {
    fn into_value(self) -> V {
        match self {
            ValueOrFunction::Value(v) => v,
            ValueOrFunction::Function(f) => f(),
        }
    }
}

pub(crate) fn hash<K, H>(build_hasher: &H, key: &K) -> u64
where
    K: ?Sized + Hash,
    H: BuildHasher,
{
    let mut hasher = build_hasher.build_hasher();
    key.hash(&mut hasher);

    hasher.finish()
}

pub(crate) enum InsertionResult<'g, K, V> {
    AlreadyPresent(Shared<'g, Bucket<K, V>>),
    Inserted,
    ReplacedTombstone(Shared<'g, Bucket<K, V>>),
}

pub(crate) unsafe fn defer_destroy_bucket<'g, K, V>(
    guard: &'g Guard,
    mut ptr: Shared<'g, Bucket<K, V>>,
) {
    assert!(!ptr.is_null());

    guard.defer_unchecked(move || {
        atomic::fence(Ordering::Acquire);

        if !is_tombstone(ptr) {
            ptr::drop_in_place(ptr.deref_mut().maybe_value.as_mut_ptr());
        }

        mem::drop(ptr.into_owned());
    });
}

pub(crate) unsafe fn defer_destroy_tombstone<'g, K, V>(
    guard: &'g Guard,
    mut ptr: Shared<'g, Bucket<K, V>>,
) {
    assert!(!ptr.is_null());
    assert!(is_tombstone(ptr));

    atomic::fence(Ordering::Acquire);
    // read the value now, but defer its destruction for later
    let value = ptr::read(ptr.deref_mut().maybe_value.as_ptr());

    // to be entirely honest, i don't know what order deferred functions are
    // called in crossbeam-epoch. in the case that the deferred functions are
    // called out of order, this prevents that from being an issue.
    guard.defer_unchecked(move || mem::drop(value));
}

pub(crate) unsafe fn defer_acquire_destroy<'g, T>(guard: &'g Guard, ptr: Shared<'g, T>) {
    assert!(!ptr.is_null());

    guard.defer_unchecked(move || {
        atomic::fence(Ordering::Acquire);
        mem::drop(ptr.into_owned());
    });
}

#[derive(Clone, Copy)]
pub(crate) enum RehashOp {
    Expand,
    Shrink,
    GcOnly,
    Skip,
}

impl RehashOp {
    pub(crate) fn new(cap: usize, tombstone_count: &AtomicUsize, len: &AtomicUsize) -> Self {
        let real_cap = cap as f64 * 2.0;
        let quarter_cap = real_cap / 4.0;
        let tbc = tombstone_count.load(Ordering::Relaxed) as f64;
        let len = len.load(Ordering::Relaxed) as f64;

        if tbc >= 25_000.0 || tbc / real_cap >= 0.1 {
            if len - tbc < quarter_cap && quarter_cap as usize >= BUCKET_ARRAY_DEFAULT_LENGTH {
                return Self::Shrink;
            } else {
                return Self::GcOnly;
            }
        }

        if len > real_cap * 0.7 {
            return Self::Expand;
        }

        Self::Skip
    }

    pub(crate) fn is_skip(self) -> bool {
        matches!(self, Self::Skip)
    }

    fn new_len(self, current_len: usize) -> usize {
        match self {
            Self::Expand => current_len * 2,
            Self::Shrink => current_len / 2,
            Self::GcOnly => current_len,
            Self::Skip => unreachable!(),
        }
    }
}

pub(crate) const SENTINEL_TAG: usize = 0b001; // set on old table buckets when copied into a new table
pub(crate) const TOMBSTONE_TAG: usize = 0b010; // set when the value has been destroyed
pub(crate) const BORROWED_TAG: usize = 0b100; // set on new table buckets when copied from an old table

#[inline]
pub(crate) fn is_sentinel<K, V>(bucket_ptr: Shared<'_, Bucket<K, V>>) -> bool {
    bucket_ptr.tag() & SENTINEL_TAG != 0
}

#[inline]
pub(crate) fn is_tombstone<K, V>(bucket_ptr: Shared<'_, Bucket<K, V>>) -> bool {
    bucket_ptr.tag() & TOMBSTONE_TAG != 0
}

#[inline]
pub(crate) fn is_borrowed<K, V>(bucket_ptr: Shared<'_, Bucket<K, V>>) -> bool {
    bucket_ptr.tag() & BORROWED_TAG != 0
}

#[cfg(test)]
mod tests {
    use super::{
        defer_destroy_bucket, defer_destroy_tombstone, hash, is_tombstone, Bucket, BucketArray,
        InsertOrModifyState, InsertionResult, RelocatedError,
    };
    use crossbeam_epoch::{Guard, Shared};
    use std::{collections::hash_map::RandomState, sync::atomic::Ordering};

    #[test]
    fn get_insert_remove() {
        let build_hasher = RandomState::new();
        let buckets = BucketArray::with_length(0, 16);
        let guard = unsafe { crossbeam_epoch::unprotected() };

        let k1 = "foo";
        let h1 = hash(&build_hasher, k1);
        let v1 = 5;

        let k2 = "bar";
        let h2 = hash(&build_hasher, k2);
        let v2 = 10;

        let k3 = "baz";
        let h3 = hash(&build_hasher, k3);
        let v3 = 15;

        assert_eq!(buckets.get(guard, h1, |&k| k == k1), Ok(Shared::null()));
        assert_eq!(buckets.get(guard, h2, |&k| k == k2), Ok(Shared::null()));
        assert_eq!(buckets.get(guard, h3, |&k| k == k3), Ok(Shared::null()));

        assert!(matches!(
            insert(&buckets, guard, k1, h1, || v1),
            Ok(InsertionResult::Inserted)
        ));

        assert_eq!(
            into_value(buckets.get(guard, h1, |&k| k == k1)),
            Ok(Some(v1))
        );
        assert_eq!(buckets.get(guard, h2, |&k| k == k2), Ok(Shared::null()));
        assert_eq!(buckets.get(guard, h3, |&k| k == k3), Ok(Shared::null()));

        assert!(matches!(
            insert(&buckets, guard, k2, h2, || v2),
            Ok(InsertionResult::Inserted)
        ));

        assert_eq!(
            into_value(buckets.get(guard, h1, |&k| k == k1)),
            Ok(Some(v1))
        );
        assert_eq!(
            into_value(buckets.get(guard, h2, |&k| k == k2)),
            Ok(Some(v2))
        );
        assert_eq!(buckets.get(guard, h3, |&k| k == k3), Ok(Shared::null()));

        assert!(matches!(
            insert(&buckets, guard, k3, h3, || v3),
            Ok(InsertionResult::Inserted)
        ));

        assert_eq!(
            into_value(buckets.get(guard, h1, |&k| k == k1)),
            Ok(Some(v1))
        );
        assert_eq!(
            into_value(buckets.get(guard, h2, |&k| k == k2)),
            Ok(Some(v2))
        );
        assert_eq!(
            into_value(buckets.get(guard, h3, |&k| k == k3)),
            Ok(Some(v3))
        );

        let b1 = buckets
            .remove_if(guard, h1, |&k| k == k1, |_, _| true)
            .ok()
            .unwrap();
        assert!(is_tombstone(b1));
        unsafe { defer_destroy_tombstone(guard, b1) };

        let b2 = buckets
            .remove_if(guard, h2, |&k| k == k2, |_, _| true)
            .ok()
            .unwrap();
        assert!(is_tombstone(b2));
        unsafe { defer_destroy_tombstone(guard, b2) };

        let b3 = buckets
            .remove_if(guard, h3, |&k| k == k3, |_, _| true)
            .ok()
            .unwrap();
        assert!(is_tombstone(b3));
        unsafe { defer_destroy_tombstone(guard, b3) };

        assert_eq!(buckets.get(guard, h1, |&k| k == k1), Ok(Shared::null()));
        assert_eq!(buckets.get(guard, h2, |&k| k == k2), Ok(Shared::null()));
        assert_eq!(buckets.get(guard, h3, |&k| k == k3), Ok(Shared::null()));

        for this_bucket in buckets.buckets.iter() {
            let this_bucket_ptr = this_bucket.swap(Shared::null(), Ordering::Relaxed, guard);

            if this_bucket_ptr.is_null() {
                continue;
            }

            unsafe {
                defer_destroy_bucket(guard, this_bucket_ptr);
            }
        }
    }

    fn insert<'g, K, V, F>(
        buckets: &BucketArray<K, V>,
        guard: &'g Guard,
        key: K,
        hash: u64,
        value_init: F,
    ) -> Result<InsertionResult<'g, K, V>, InsertOrModifyState<K, V, F>>
    where
        K: Eq,
        F: FnOnce() -> V,
    {
        let state = InsertOrModifyState::New(key, value_init);
        buckets.insert_if_not_present(guard, hash, state)
    }

    fn into_value<K, V>(
        maybe_bucket_ptr: Result<Shared<'_, Bucket<K, V>>, RelocatedError>,
    ) -> Result<Option<V>, RelocatedError>
    where
        V: Clone,
    {
        maybe_bucket_ptr
            .map(|p| unsafe { p.as_ref() }.map(|b| unsafe { &*b.maybe_value.as_ptr() }.clone()))
    }
}
