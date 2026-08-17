use super::{
    invalidator::{Invalidator, KeyDateLite, PredicateFun},
    key_lock::{KeyLock, KeyLockMap},
    PredicateId,
};

use crate::{
    common::{
        self,
        concurrent::{
            arc::MiniArc,
            constants::{
                READ_LOG_CH_SIZE, READ_LOG_FLUSH_POINT, WRITE_LOG_CH_SIZE, WRITE_LOG_FLUSH_POINT,
            },
            deques::Deques,
            entry_info::EntryInfo,
            housekeeper::{Housekeeper, InnerSync},
            AccessTime, KeyHash, KeyHashDate, KvEntry, OldEntryInfo, ReadOp, ValueEntry, Weigher,
            WriteOp,
        },
        deque::{DeqNode, Deque},
        frequency_sketch::FrequencySketch,
        iter::ScanningGet,
        time::{AtomicInstant, Clock, Instant},
        timer_wheel::{ReschedulingResult, TimerWheel},
        CacheRegion, HousekeeperConfig,
    },
    notification::{notifier::RemovalNotifier, EvictionListener, RemovalCause},
    policy::{EvictionPolicy, EvictionPolicyConfig, ExpirationPolicy},
    Entry, Expiry, Policy, PredicateError,
};

use crossbeam_channel::{Receiver, Sender, TrySendError};
use crossbeam_utils::atomic::AtomicCell;
use equivalent::Equivalent;
use parking_lot::{Mutex, RwLock};
use smallvec::SmallVec;
use std::{
    borrow::Borrow,
    collections::hash_map::RandomState,
    hash::{BuildHasher, Hash, Hasher},
    rc::Rc,
    sync::{
        atomic::{AtomicBool, AtomicU8, Ordering},
        Arc,
    },
    time::{Duration, Instant as StdInstant},
};

pub(crate) type HouseKeeperArc = Arc<Housekeeper>;

pub(crate) struct BaseCache<K, V, S = RandomState> {
    pub(crate) inner: Arc<Inner<K, V, S>>,
    read_op_ch: Sender<ReadOp<K, V>>,
    pub(crate) write_op_ch: Sender<WriteOp<K, V>>,
    pub(crate) housekeeper: Option<HouseKeeperArc>,
}

impl<K, V, S> Clone for BaseCache<K, V, S> {
    /// Makes a clone of this shared cache.
    ///
    /// This operation is cheap as it only creates thread-safe reference counted
    /// pointers to the shared internal data structures.
    fn clone(&self) -> Self {
        Self {
            inner: Arc::clone(&self.inner),
            read_op_ch: self.read_op_ch.clone(),
            write_op_ch: self.write_op_ch.clone(),
            housekeeper: self.housekeeper.clone(),
        }
    }
}

impl<K, V, S> Drop for BaseCache<K, V, S> {
    fn drop(&mut self) {
        // The housekeeper needs to be dropped before the inner is dropped.
        std::mem::drop(self.housekeeper.take());
    }
}

impl<K, V, S> BaseCache<K, V, S> {
    pub(crate) fn name(&self) -> Option<&str> {
        self.inner.name()
    }

    pub(crate) fn policy(&self) -> Policy {
        self.inner.policy()
    }

    pub(crate) fn entry_count(&self) -> u64 {
        self.inner.entry_count()
    }

    pub(crate) fn weighted_size(&self) -> u64 {
        self.inner.weighted_size()
    }

    pub(crate) fn is_map_disabled(&self) -> bool {
        self.inner.max_capacity == Some(0)
    }

    #[inline]
    pub(crate) fn is_removal_notifier_enabled(&self) -> bool {
        self.inner.is_removal_notifier_enabled()
    }

    #[inline]
    pub(crate) fn current_time(&self) -> Instant {
        self.inner.current_time()
    }

    pub(crate) fn notify_invalidate(&self, key: &Arc<K>, entry: &MiniArc<ValueEntry<K, V>>)
    where
        K: Send + Sync + 'static,
        V: Clone + Send + Sync + 'static,
    {
        self.inner.notify_invalidate(key, entry);
    }
}

impl<K, V, S> BaseCache<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher,
{
    pub(crate) fn maybe_key_lock(&self, key: &Arc<K>) -> Option<KeyLock<'_, K, S>> {
        self.inner.maybe_key_lock(key)
    }
}

impl<K, V, S> BaseCache<K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    // https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn new(
        name: Option<String>,
        max_capacity: Option<u64>,
        initial_capacity: Option<usize>,
        build_hasher: S,
        weigher: Option<Weigher<K, V>>,
        eviction_policy: EvictionPolicy,
        eviction_listener: Option<EvictionListener<K, V>>,
        expiration_policy: ExpirationPolicy<K, V>,
        housekeeper_config: HousekeeperConfig,
        invalidator_enabled: bool,
        clock: Clock,
    ) -> Self {
        let (r_size, w_size) = if max_capacity == Some(0) {
            (0, 0)
        } else {
            (READ_LOG_CH_SIZE, WRITE_LOG_CH_SIZE)
        };
        let is_eviction_listener_enabled = eviction_listener.is_some();
        let fast_now = clock.fast_now();

        let (r_snd, r_rcv) = crossbeam_channel::bounded(r_size);
        let (w_snd, w_rcv) = crossbeam_channel::bounded(w_size);

        let inner = Arc::new(Inner::new(
            name,
            max_capacity,
            initial_capacity,
            build_hasher,
            weigher,
            eviction_policy,
            eviction_listener,
            r_rcv,
            w_rcv,
            expiration_policy,
            invalidator_enabled,
            clock,
        ));

        Self {
            inner,
            read_op_ch: r_snd,
            write_op_ch: w_snd,
            housekeeper: Some(Arc::new(Housekeeper::new(
                is_eviction_listener_enabled,
                housekeeper_config,
                fast_now,
            ))),
        }
    }

    #[inline]
    pub(crate) fn hash<Q>(&self, key: &Q) -> u64
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.inner.hash(key)
    }

    pub(crate) fn contains_key_with_hash<Q>(&self, key: &Q, hash: u64) -> bool
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        // TODO: Maybe we can just call ScanningGet::scanning_get.
        self.inner
            .get_key_value_and(key, hash, |k, entry| {
                let i = &self.inner;
                let (ttl, tti, va) = (&i.time_to_live(), &i.time_to_idle(), &i.valid_after());
                let now = self.current_time();

                !is_expired_by_per_entry_ttl(entry.entry_info(), now)
                    && !is_expired_entry_wo(ttl, va, entry, now)
                    && !is_expired_entry_ao(tti, va, entry, now)
                    && !i.is_invalidated_entry(k, entry)
            })
            .unwrap_or_default() // `false` is the default for `bool` type.
    }

    pub(crate) fn get_with_hash<Q>(&self, key: &Q, hash: u64, need_key: bool) -> Option<Entry<K, V>>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        // Define a closure to record a read op.
        let record = |op, now| {
            self.record_read_op(op, now)
                .expect("Failed to record a get op");
        };
        let ignore_if = None as Option<&mut fn(&V) -> bool>;
        self.do_get_with_hash(key, hash, record, ignore_if, need_key)
    }

    pub(crate) fn get_with_hash_and_ignore_if<Q, I>(
        &self,
        key: &Q,
        hash: u64,
        ignore_if: Option<&mut I>,
        need_key: bool,
    ) -> Option<Entry<K, V>>
    where
        Q: Equivalent<K> + Hash + ?Sized,
        I: FnMut(&V) -> bool,
    {
        // Define a closure to record a read op.
        let record = |op, now| {
            self.record_read_op(op, now)
                .expect("Failed to record a get op");
        };
        self.do_get_with_hash(key, hash, record, ignore_if, need_key)
    }

    pub(crate) fn get_with_hash_without_recording<Q, I>(
        &self,
        key: &Q,
        hash: u64,
        ignore_if: Option<&mut I>,
    ) -> Option<V>
    where
        Q: Equivalent<K> + Hash + ?Sized,
        I: FnMut(&V) -> bool,
    {
        // Define a closure that skips to record a read op.
        let record = |_op, _now| {};
        self.do_get_with_hash(key, hash, record, ignore_if, false)
            .map(Entry::into_value)
    }

    fn do_get_with_hash<Q, R, I>(
        &self,
        key: &Q,
        hash: u64,
        read_recorder: R,
        mut ignore_if: Option<&mut I>,
        need_key: bool,
    ) -> Option<Entry<K, V>>
    where
        Q: Equivalent<K> + Hash + ?Sized,
        R: Fn(ReadOp<K, V>, Instant),
        I: FnMut(&V) -> bool,
    {
        if self.is_map_disabled() {
            return None;
        }

        let mut now = self.current_time();

        let maybe_entry = self
            .inner
            .get_key_value_and_then(key, hash, move |k, entry| {
                if let Some(ignore_if) = &mut ignore_if {
                    if ignore_if(&entry.value) {
                        // Ignore the entry.
                        return None;
                    }
                }

                let i = &self.inner;
                let (ttl, tti, va) = (&i.time_to_live(), &i.time_to_idle(), &i.valid_after());

                if is_expired_by_per_entry_ttl(entry.entry_info(), now)
                    || is_expired_entry_wo(ttl, va, entry, now)
                    || is_expired_entry_ao(tti, va, entry, now)
                    || i.is_invalidated_entry(k, entry)
                {
                    // Expired or invalidated entry.
                    None
                } else {
                    // Valid entry.
                    let maybe_key = if need_key { Some(Arc::clone(k)) } else { None };
                    Some((maybe_key, MiniArc::clone(entry)))
                }
            });

        if let Some((maybe_key, entry)) = maybe_entry {
            let mut is_expiry_modified = false;

            // Call the user supplied `expire_after_read` method if any.
            if let Some(expiry) = &self.inner.expiration_policy.expiry() {
                let lm = entry.last_modified().expect("Last modified is not set");
                // Check if the `last_modified` of entry is earlier than or equals to
                // `now`. If not, update the `now` to `last_modified`. This is needed
                // because there is a small chance that other threads have inserted
                // the entry _after_ we obtained `now`.
                now = now.max(lm);

                // Convert `last_modified` from `moka::common::time::Instant` to
                // `std::time::Instant`.
                let lm = self.inner.clock().to_std_instant(lm);

                // Call the user supplied `expire_after_read` method.
                //
                // We will put the return value (`is_expiry_modified: bool`) to a
                // `ReadOp` so that `apply_reads` method can determine whether or not
                // to reschedule the timer for the entry.
                //
                // NOTE: It is not guaranteed that the `ReadOp` is passed to
                // `apply_reads`. Here are the corner cases that the `ReadOp` will
                // not be passed to `apply_reads`:
                //
                // - If the bounded `read_op_ch` channel is full, the `ReadOp` will
                //   be discarded.
                // - If we were called by `get_with_hash_without_recording` method,
                //   the `ReadOp` will not be recorded at all.
                //
                // These cases are okay because when the timer wheel tries to expire
                // the entry, it will check if the entry is actually expired. If not,
                // the timer wheel will reschedule the expiration timer for the
                // entry.
                is_expiry_modified = Self::expire_after_read_or_update(
                    |k, v, t, d| expiry.expire_after_read(k, v, t, d, lm),
                    &entry.entry_info().key_hash().key,
                    &entry,
                    self.inner.expiration_policy.time_to_live(),
                    self.inner.expiration_policy.time_to_idle(),
                    now,
                    self.inner.clock(),
                );
            }

            entry.set_last_accessed(now);

            let v = entry.value.clone();
            let op = ReadOp::Hit {
                value_entry: entry,
                is_expiry_modified,
            };
            read_recorder(op, now);
            Some(Entry::new(maybe_key, v, false, false))
        } else {
            read_recorder(ReadOp::Miss(hash), now);
            None
        }
    }

    pub(crate) fn get_key_with_hash<Q>(&self, key: &Q, hash: u64) -> Option<Arc<K>>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.inner
            .get_key_value_and(key, hash, |k, _entry| Arc::clone(k))
    }

    #[inline]
    pub(crate) fn remove_entry<Q>(&self, key: &Q, hash: u64) -> Option<KvEntry<K, V>>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.inner.remove_entry(key, hash)
    }

    #[inline]
    pub(crate) fn apply_reads_writes_if_needed(
        inner: &impl InnerSync,
        ch: &Sender<WriteOp<K, V>>,
        now: Instant,
        housekeeper: Option<&HouseKeeperArc>,
    ) {
        let w_len = ch.len();

        if let Some(hk) = housekeeper {
            if Self::should_apply_writes(hk, w_len, now) {
                hk.try_run_pending_tasks(inner);
            }
        }
    }

    pub(crate) fn invalidate_all(&self) {
        let now = self.current_time();
        self.inner.set_valid_after(now);
    }

    pub(crate) fn invalidate_entries_if(
        &self,
        predicate: PredicateFun<K, V>,
    ) -> Result<PredicateId, PredicateError> {
        let now = self.current_time();
        self.inner.register_invalidation_predicate(predicate, now)
    }
}

//
// Iterator support
//
impl<K, V, S> ScanningGet<K, V> for BaseCache<K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    fn num_cht_segments(&self) -> usize {
        self.inner.num_cht_segments()
    }

    fn scanning_get(&self, key: &Arc<K>) -> Option<V> {
        let hash = self.hash(&**key);
        self.inner.get_key_value_and_then(&**key, hash, |k, entry| {
            let i = &self.inner;
            let (ttl, tti, va) = (&i.time_to_live(), &i.time_to_idle(), &i.valid_after());
            let now = self.current_time();

            if is_expired_by_per_entry_ttl(entry.entry_info(), now)
                || is_expired_entry_wo(ttl, va, entry, now)
                || is_expired_entry_ao(tti, va, entry, now)
                || i.is_invalidated_entry(k, entry)
            {
                // Expired or invalidated entry.
                None
            } else {
                // Valid entry.
                Some(entry.value.clone())
            }
        })
    }

    fn keys(&self, cht_segment: usize) -> Option<Vec<Arc<K>>> {
        self.inner.keys(cht_segment)
    }
}

//
// private methods
//
impl<K, V, S> BaseCache<K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    #[inline]
    fn record_read_op(
        &self,
        op: ReadOp<K, V>,
        now: Instant,
    ) -> Result<(), TrySendError<ReadOp<K, V>>> {
        self.apply_reads_if_needed(&self.inner, now);
        let ch = &self.read_op_ch;
        match ch.try_send(op) {
            // Discard the ReadOp when the channel is full.
            Ok(()) | Err(TrySendError::Full(_)) => Ok(()),
            Err(e @ TrySendError::Disconnected(_)) => Err(e),
        }
    }

    #[inline]
    pub(crate) fn do_insert_with_hash(
        &self,
        key: Arc<K>,
        hash: u64,
        value: V,
    ) -> (WriteOp<K, V>, Instant) {
        let weight = self.inner.weigh(&key, &value);
        let op_cnt1 = Rc::new(AtomicU8::new(0));
        let op_cnt2 = Rc::clone(&op_cnt1);
        let mut op1 = None;
        let mut op2 = None;

        // Lock the key for update if blocking removal notification is enabled.
        let kl = self.maybe_key_lock(&key);
        let _klg = &kl.as_ref().map(|kl| kl.lock());

        let ts = self.current_time();

        // TODO: Instead using Arc<AtomicU8> to check if the actual operation was
        // insert or update, check the return value of insert_with_or_modify. If it
        // is_some, the value was updated, otherwise the value was inserted.

        // Since the cache (cht::SegmentedHashMap) employs optimistic locking
        // strategy, insert_with_or_modify() may get an insert/modify operation
        // conflicted with other concurrent hash table operations. In that case, it
        // has to retry the insertion or modification, so on_insert and/or on_modify
        // closures can be executed more than once. In order to identify the last
        // call of these closures, we use a shared counter (op_cnt{1,2}) here to
        // record a serial number on a WriteOp, and consider the WriteOp with the
        // largest serial number is the one made by the last call of the closures.
        self.inner.cache.insert_with_or_modify(
            Arc::clone(&key),
            hash,
            // on_insert
            || {
                let (entry, gen) = self.new_value_entry(&key, hash, value.clone(), ts, weight);
                let ins_op = WriteOp::new_upsert(&key, hash, &entry, gen, 0, weight);
                let cnt = op_cnt1.fetch_add(1, Ordering::Relaxed);
                op1 = Some((cnt, ins_op));
                entry
            },
            // on_modify
            |_k, old_entry| {
                let old_weight = old_entry.policy_weight();

                // Create this OldEntryInfo _before_ creating a new ValueEntry, so
                // that the OldEntryInfo can preserve the old EntryInfo's
                // last_accessed and last_modified timestamps.
                let old_info = OldEntryInfo::new(old_entry);
                let (entry, gen) = self.new_value_entry_from(value.clone(), ts, weight, old_entry);
                let upd_op = WriteOp::new_upsert(&key, hash, &entry, gen, old_weight, weight);
                let cnt = op_cnt2.fetch_add(1, Ordering::Relaxed);
                op2 = Some((cnt, old_info, upd_op));
                entry
            },
        );

        match (op1, op2) {
            (Some((_cnt, ins_op)), None) => self.do_post_insert_steps(ts, &key, ins_op),
            (Some((cnt1, ins_op)), Some((cnt2, ..))) if cnt1 > cnt2 => {
                self.do_post_insert_steps(ts, &key, ins_op)
            }
            (_, Some((_cnt, old_info, upd_op))) => {
                self.do_post_update_steps(ts, key, old_info, upd_op)
            }
            (None, None) => unreachable!(),
        }
    }

    fn do_post_insert_steps(
        &self,
        ts: Instant,
        key: &Arc<K>,
        ins_op: WriteOp<K, V>,
    ) -> (WriteOp<K, V>, Instant) {
        if let (Some(expiry), WriteOp::Upsert { value_entry, .. }) =
            (&self.inner.expiration_policy.expiry(), &ins_op)
        {
            Self::expire_after_create(expiry, key, value_entry, ts, self.inner.clock());
        }
        (ins_op, ts)
    }

    fn do_post_update_steps(
        &self,
        ts: Instant,
        key: Arc<K>,
        old_info: OldEntryInfo<K, V>,
        upd_op: WriteOp<K, V>,
    ) -> (WriteOp<K, V>, Instant) {
        if let (Some(expiry), WriteOp::Upsert { value_entry, .. }) =
            (&self.inner.expiration_policy.expiry(), &upd_op)
        {
            Self::expire_after_read_or_update(
                |k, v, t, d| expiry.expire_after_update(k, v, t, d),
                &key,
                value_entry,
                self.inner.expiration_policy.time_to_live(),
                self.inner.expiration_policy.time_to_idle(),
                ts,
                self.inner.clock(),
            );
        }

        if self.is_removal_notifier_enabled() {
            self.inner.notify_upsert(
                key,
                &old_info.entry,
                old_info.last_accessed,
                old_info.last_modified,
            );
        }
        crossbeam_epoch::pin().flush();
        (upd_op, ts)
    }

    #[inline]
    fn apply_reads_if_needed(&self, inner: &Inner<K, V, S>, now: Instant) {
        let len = self.read_op_ch.len();

        if let Some(hk) = &self.housekeeper {
            if Self::should_apply_reads(hk, len, now) {
                hk.try_run_pending_tasks(inner);
            }
        }
    }

    #[inline]
    fn should_apply_reads(hk: &HouseKeeperArc, ch_len: usize, now: Instant) -> bool {
        hk.should_apply_reads(ch_len, now)
    }

    #[inline]
    fn should_apply_writes(hk: &HouseKeeperArc, ch_len: usize, now: Instant) -> bool {
        hk.should_apply_writes(ch_len, now)
    }
}

impl<K, V, S> BaseCache<K, V, S> {
    #[inline]
    fn new_value_entry(
        &self,
        key: &Arc<K>,
        hash: u64,
        value: V,
        timestamp: Instant,
        policy_weight: u32,
    ) -> (MiniArc<ValueEntry<K, V>>, u16) {
        let key_hash = KeyHash::new(Arc::clone(key), hash);
        let info = MiniArc::new(EntryInfo::new(key_hash, timestamp, policy_weight));
        let gen: u16 = info.entry_gen();
        (MiniArc::new(ValueEntry::new(value, info)), gen)
    }

    #[inline]
    fn new_value_entry_from(
        &self,
        value: V,
        timestamp: Instant,
        policy_weight: u32,
        other: &ValueEntry<K, V>,
    ) -> (MiniArc<ValueEntry<K, V>>, u16) {
        let info = MiniArc::clone(other.entry_info());
        // To prevent this updated ValueEntry from being evicted by an expiration
        // policy, increment the entry generation.
        let gen = info.incr_entry_gen();
        info.set_last_accessed(timestamp);
        info.set_last_modified(timestamp);
        info.set_policy_weight(policy_weight);
        (MiniArc::new(ValueEntry::new_from(value, info, other)), gen)
    }

    fn expire_after_create(
        expiry: &Arc<dyn Expiry<K, V> + Send + Sync + 'static>,
        key: &K,
        value_entry: &ValueEntry<K, V>,
        ts: Instant,
        clock: &Clock,
    ) {
        let duration =
            expiry.expire_after_create(key, &value_entry.value, clock.to_std_instant(ts));
        let expiration_time = duration.map(|duration| ts.saturating_add(duration));
        value_entry
            .entry_info()
            .set_expiration_time(expiration_time);
    }

    fn expire_after_read_or_update(
        expiry: impl FnOnce(&K, &V, StdInstant, Option<Duration>) -> Option<Duration>,
        key: &K,
        value_entry: &ValueEntry<K, V>,
        ttl: Option<Duration>,
        tti: Option<Duration>,
        ts: Instant,
        clock: &Clock,
    ) -> bool {
        let current_time = clock.to_std_instant(ts);
        let ei = &value_entry.entry_info();

        let exp_time = IntoIterator::into_iter([
            ei.expiration_time(),
            ttl.and_then(|dur| ei.last_modified().map(|ts| ts.saturating_add(dur))),
            tti.and_then(|dur| ei.last_accessed().map(|ts| ts.saturating_add(dur))),
        ])
        .flatten()
        .min();

        let current_duration = exp_time.and_then(|time| {
            let std_time = clock.to_std_instant(time);
            std_time.checked_duration_since(current_time)
        });

        let duration = expiry(key, &value_entry.value, current_time, current_duration);

        if duration != current_duration {
            let expiration_time = duration.map(|duration| ts.saturating_add(duration));
            value_entry
                .entry_info()
                .set_expiration_time(expiration_time);
            // The `expiration_time` has changed from `None` to `Some` or vice versa.
            true
        } else {
            false
        }
    }
}

//
// for testing
//
#[cfg(test)]
impl<K, V, S> BaseCache<K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    pub(crate) fn invalidation_predicate_count(&self) -> usize {
        self.inner.invalidation_predicate_count()
    }

    pub(crate) fn reconfigure_for_testing(&mut self) {
        // Enable the frequency sketch.
        self.inner.enable_frequency_sketch_for_testing();
        // Disable auto clean up of pending tasks.
        if let Some(hk) = &self.housekeeper {
            hk.disable_auto_run();
        }
    }

    pub(crate) fn key_locks_map_is_empty(&self) -> bool {
        self.inner.key_locks_map_is_empty()
    }
}

struct EvictionState<'a, K, V> {
    counters: EvictionCounters,
    notifier: Option<&'a RemovalNotifier<K, V>>,
    more_entries_to_evict: bool,
}

impl<'a, K, V> EvictionState<'a, K, V> {
    fn new(
        entry_count: u64,
        weighted_size: u64,
        notifier: Option<&'a RemovalNotifier<K, V>>,
    ) -> Self {
        Self {
            counters: EvictionCounters::new(entry_count, weighted_size),
            notifier,
            more_entries_to_evict: false,
        }
    }

    fn is_notifier_enabled(&self) -> bool {
        self.notifier.is_some()
    }

    fn notify_entry_removal(
        &mut self,
        key: Arc<K>,
        entry: &MiniArc<ValueEntry<K, V>>,
        cause: RemovalCause,
    ) where
        K: Send + Sync + 'static,
        V: Clone + Send + Sync + 'static,
    {
        if let Some(notifier) = self.notifier {
            notifier.notify(key, entry.value.clone(), cause);
        } else {
            panic!("notify_entry_removal is called when the notification is disabled");
        }
    }
}

struct EvictionCounters {
    entry_count: u64,
    weighted_size: u64,
    eviction_count: u64,
}

impl EvictionCounters {
    #[inline]
    fn new(entry_count: u64, weighted_size: u64) -> Self {
        Self {
            entry_count,
            weighted_size,
            eviction_count: 0,
        }
    }

    #[inline]
    fn saturating_add(&mut self, entry_count: u64, weight: u32) {
        self.entry_count += entry_count;
        let total = &mut self.weighted_size;
        *total = total.saturating_add(weight as u64);
    }

    #[inline]
    fn saturating_sub(&mut self, entry_count: u64, weight: u32) {
        self.entry_count -= entry_count;
        let total = &mut self.weighted_size;
        *total = total.saturating_sub(weight as u64);
    }

    #[inline]
    fn incr_eviction_count(&mut self) {
        let count = &mut self.eviction_count;
        *count = count.saturating_add(1);
    }
}

#[derive(Default)]
struct EntrySizeAndFrequency {
    policy_weight: u64,
    freq: u32,
}

impl EntrySizeAndFrequency {
    fn new(policy_weight: u32) -> Self {
        Self {
            policy_weight: policy_weight as u64,
            ..Default::default()
        }
    }

    fn add_policy_weight(&mut self, weight: u32) {
        self.policy_weight += weight as u64;
    }

    fn add_frequency(&mut self, freq: &FrequencySketch, hash: u64) {
        self.freq += freq.frequency(hash) as u32;
    }
}

// NOTE: Clippy found that the `Admitted` variant contains at least a few hundred
// bytes of data and the `Rejected` variant contains no data at all. It suggested to
// box the `SmallVec`.
//
// We ignore the suggestion because (1) the `SmallVec` is used to avoid heap
// allocation as it will be used in a performance hot spot, and (2) this enum has a
// very short lifetime and there will only one instance at a time.
#[allow(clippy::large_enum_variant)]
enum AdmissionResult<K> {
    Admitted {
        /// A vec of pairs of `KeyHash` and `last_accessed`.
        victim_keys: SmallVec<[(KeyHash<K>, Option<Instant>); 8]>,
    },
    Rejected,
}

type CacheStore<K, V, S> = crate::cht::SegmentedHashMap<Arc<K>, MiniArc<ValueEntry<K, V>>, S>;

pub(crate) struct Inner<K, V, S> {
    name: Option<String>,
    max_capacity: Option<u64>,
    entry_count: AtomicCell<u64>,
    weighted_size: AtomicCell<u64>,
    pub(crate) cache: CacheStore<K, V, S>,
    build_hasher: S,
    deques: Mutex<Deques<K>>,
    timer_wheel: Mutex<TimerWheel<K>>,
    frequency_sketch: RwLock<FrequencySketch>,
    frequency_sketch_enabled: AtomicBool,
    read_op_ch: Receiver<ReadOp<K, V>>,
    write_op_ch: Receiver<WriteOp<K, V>>,
    eviction_policy: EvictionPolicyConfig,
    expiration_policy: ExpirationPolicy<K, V>,
    valid_after: AtomicInstant,
    weigher: Option<Weigher<K, V>>,
    removal_notifier: Option<RemovalNotifier<K, V>>,
    key_locks: Option<KeyLockMap<K, S>>,
    invalidator: Option<Invalidator<K, V, S>>,
    clock: Clock,
}

impl<K, V, S> Drop for Inner<K, V, S> {
    fn drop(&mut self) {
        // Ensure crossbeam-epoch to collect garbages (`deferred_fn`s) in the
        // global bag so that previously cached values will be dropped.
        for _ in 0..128 {
            crossbeam_epoch::pin().flush();
        }

        // NOTE: The `CacheStore` (`cht`) will be dropped after returning from this
        // `drop` method. It uses crossbeam-epoch internally, but we do not have to
        // call `flush` for it because its `drop` methods do not create
        // `deferred_fn`s, and drop its values in place.
    }
}

//
// functions/methods used by BaseCache
//

impl<K, V, S> Inner<K, V, S> {
    fn name(&self) -> Option<&str> {
        self.name.as_deref()
    }

    fn policy(&self) -> Policy {
        let exp = &self.expiration_policy;
        Policy::new(self.max_capacity, 1, exp.time_to_live(), exp.time_to_idle())
    }

    #[inline]
    fn entry_count(&self) -> u64 {
        self.entry_count.load()
    }

    #[inline]
    fn weighted_size(&self) -> u64 {
        self.weighted_size.load()
    }

    #[inline]
    pub(crate) fn is_removal_notifier_enabled(&self) -> bool {
        self.removal_notifier.is_some()
    }

    pub(crate) fn maybe_key_lock(&self, key: &Arc<K>) -> Option<KeyLock<'_, K, S>>
    where
        K: Hash + Eq,
        S: BuildHasher,
    {
        self.key_locks.as_ref().map(|kls| kls.key_lock(key))
    }

    #[inline]
    fn current_time(&self) -> Instant {
        self.clock.now()
    }

    fn clock(&self) -> &Clock {
        &self.clock
    }

    fn num_cht_segments(&self) -> usize {
        self.cache.actual_num_segments()
    }

    #[inline]
    fn time_to_live(&self) -> Option<Duration> {
        self.expiration_policy.time_to_live()
    }

    #[inline]
    fn time_to_idle(&self) -> Option<Duration> {
        self.expiration_policy.time_to_idle()
    }

    #[inline]
    fn has_expiry(&self) -> bool {
        let exp = &self.expiration_policy;
        exp.time_to_live().is_some() || exp.time_to_idle().is_some()
    }

    #[inline]
    fn is_write_order_queue_enabled(&self) -> bool {
        self.expiration_policy.time_to_live().is_some() || self.invalidator.is_some()
    }

    #[inline]
    fn valid_after(&self) -> Option<Instant> {
        self.valid_after.instant()
    }

    #[inline]
    fn set_valid_after(&self, timestamp: Instant) {
        self.valid_after.set_instant(timestamp);
    }

    #[inline]
    fn has_valid_after(&self) -> bool {
        self.valid_after.is_set()
    }
}

impl<K, V, S> Inner<K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Send + Sync + 'static,
    S: BuildHasher + Clone,
{
    // Disable a Clippy warning for having more than seven arguments.
    // https://rust-lang.github.io/rust-clippy/master/index.html#too_many_arguments
    #[allow(clippy::too_many_arguments)]
    fn new(
        name: Option<String>,
        max_capacity: Option<u64>,
        initial_capacity: Option<usize>,
        build_hasher: S,
        weigher: Option<Weigher<K, V>>,
        eviction_policy: EvictionPolicy,
        eviction_listener: Option<EvictionListener<K, V>>,
        read_op_ch: Receiver<ReadOp<K, V>>,
        write_op_ch: Receiver<WriteOp<K, V>>,
        expiration_policy: ExpirationPolicy<K, V>,
        invalidator_enabled: bool,
        clock: Clock,
    ) -> Self {
        // TODO: Calculate the number of segments based on the max capacity and the
        // number of CPUs.
        let (num_segments, initial_capacity) = if max_capacity == Some(0) {
            (1, 0)
        } else {
            let ic = initial_capacity
                .map(|cap| cap + WRITE_LOG_CH_SIZE)
                .unwrap_or_default();
            (64, ic)
        };
        let cache = crate::cht::SegmentedHashMap::with_num_segments_capacity_and_hasher(
            num_segments,
            initial_capacity,
            build_hasher.clone(),
        );

        let now = clock.now();
        let timer_wheel = Mutex::new(TimerWheel::new(now));

        let (removal_notifier, key_locks) = if let Some(listener) = eviction_listener {
            let rn = RemovalNotifier::new(listener, name.clone());
            let kl = KeyLockMap::with_hasher(build_hasher.clone());
            (Some(rn), Some(kl))
        } else {
            (None, None)
        };

        let invalidator = if invalidator_enabled {
            Some(Invalidator::new(build_hasher.clone()))
        } else {
            None
        };

        Self {
            name,
            max_capacity,
            entry_count: AtomicCell::default(),
            weighted_size: AtomicCell::default(),
            cache,
            build_hasher,
            deques: Mutex::default(),
            timer_wheel,
            frequency_sketch: RwLock::new(FrequencySketch::default()),
            frequency_sketch_enabled: AtomicBool::default(),
            read_op_ch,
            write_op_ch,
            eviction_policy: eviction_policy.config,
            expiration_policy,
            valid_after: AtomicInstant::default(),
            weigher,
            removal_notifier,
            key_locks,
            invalidator,
            clock,
        }
    }

    #[inline]
    fn hash<Q>(&self, key: &Q) -> u64
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        let mut hasher = self.build_hasher.build_hasher();
        key.hash(&mut hasher);
        hasher.finish()
    }

    #[inline]
    fn get_key_value_and<Q, F, T>(&self, key: &Q, hash: u64, with_entry: F) -> Option<T>
    where
        Q: Equivalent<K> + Hash + ?Sized,
        F: FnOnce(&Arc<K>, &MiniArc<ValueEntry<K, V>>) -> T,
    {
        self.cache
            .get_key_value_and(hash, |k| key.equivalent(k as &K), with_entry)
    }

    #[inline]
    fn get_key_value_and_then<Q, F, T>(&self, key: &Q, hash: u64, with_entry: F) -> Option<T>
    where
        Q: Equivalent<K> + Hash + ?Sized,
        F: FnOnce(&Arc<K>, &MiniArc<ValueEntry<K, V>>) -> Option<T>,
    {
        self.cache
            .get_key_value_and_then(hash, |k| key.equivalent(k as &K), with_entry)
    }

    #[inline]
    fn remove_entry<Q>(&self, key: &Q, hash: u64) -> Option<KvEntry<K, V>>
    where
        Q: Equivalent<K> + Hash + ?Sized,
    {
        self.cache
            .remove_entry(hash, |k| key.equivalent(k as &K))
            .map(|(key, entry)| KvEntry::new(key, entry))
    }

    fn keys(&self, cht_segment: usize) -> Option<Vec<Arc<K>>> {
        // Do `Arc::clone` instead of `Arc::downgrade`. Updating existing entry
        // in the cht with a new value replaces the key in the cht even though the
        // old and new keys are equal. If we return `Weak<K>`, it will not be
        // upgraded later to `Arc<K> as the key may have been replaced with a new
        // key that equals to the old key.
        self.cache.keys(cht_segment, Arc::clone)
    }

    #[inline]
    fn register_invalidation_predicate(
        &self,
        predicate: PredicateFun<K, V>,
        registered_at: Instant,
    ) -> Result<PredicateId, PredicateError> {
        if let Some(inv) = &self.invalidator {
            inv.register_predicate(predicate, registered_at)
        } else {
            Err(PredicateError::InvalidationClosuresDisabled)
        }
    }

    /// Returns `true` if the entry is invalidated by `invalidate_entries_if` method.
    #[inline]
    fn is_invalidated_entry(&self, key: &Arc<K>, entry: &MiniArc<ValueEntry<K, V>>) -> bool
    where
        V: Clone,
    {
        if let Some(inv) = &self.invalidator {
            return inv.apply_predicates(key, entry);
        }
        false
    }

    #[inline]
    fn weigh(&self, key: &K, value: &V) -> u32 {
        self.weigher.as_ref().map_or(1, |w| w(key, value))
    }
}

impl<K, V, S> InnerSync for Inner<K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    fn run_pending_tasks(
        &self,
        timeout: Option<Duration>,
        max_log_sync_repeats: u32,
        eviction_batch_size: u32,
    ) -> bool {
        self.do_run_pending_tasks(timeout, max_log_sync_repeats, eviction_batch_size)
    }

    fn now(&self) -> Instant {
        self.current_time()
    }
}

impl<K, V, S> Inner<K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    fn do_run_pending_tasks(
        &self,
        timeout: Option<Duration>,
        max_log_sync_repeats: u32,
        eviction_batch_size: u32,
    ) -> bool {
        if self.max_capacity == Some(0) {
            return false;
        }

        // Acquire some locks.
        let mut deqs = self.deques.lock();
        let mut timer_wheel = self.timer_wheel.lock();

        let started_at = if timeout.is_some() {
            Some(self.current_time())
        } else {
            None
        };
        let mut should_process_logs = true;
        let mut calls = 0u32;
        let current_ec = self.entry_count.load();
        let current_ws = self.weighted_size.load();
        let mut eviction_state =
            EvictionState::new(current_ec, current_ws, self.removal_notifier.as_ref());

        loop {
            if should_process_logs {
                let r_len = self.read_op_ch.len();
                if r_len > 0 {
                    self.apply_reads(&mut deqs, &mut timer_wheel, r_len);
                }

                let w_len = self.write_op_ch.len();
                if w_len > 0 {
                    self.apply_writes(&mut deqs, &mut timer_wheel, w_len, &mut eviction_state);
                }

                if self.eviction_policy == EvictionPolicyConfig::TinyLfu
                    && self.should_enable_frequency_sketch(&eviction_state.counters)
                {
                    self.enable_frequency_sketch(&eviction_state.counters);
                }

                calls += 1;
            }

            // Set this flag to `false`. The `evict_*` and `invalidate_*` methods
            // below may set it to `true` if there are more entries to evict in next
            // loop.
            eviction_state.more_entries_to_evict = false;
            let last_eviction_count = eviction_state.counters.eviction_count;

            // Evict entries if there are any expired entries in the hierarchical
            // timer wheels.
            if timer_wheel.is_enabled() {
                self.evict_expired_entries_using_timers(
                    &mut timer_wheel,
                    &mut deqs,
                    &mut eviction_state,
                );
            }

            // Evict entries if there are any expired entries in the write order or
            // access order deques.
            if self.has_expiry() || self.has_valid_after() {
                self.evict_expired_entries_using_deqs(
                    &mut deqs,
                    &mut timer_wheel,
                    eviction_batch_size,
                    &mut eviction_state,
                );
            }

            // Evict entries if there are any invalidation predicates set by the
            // `invalidate_entries_if` method.
            if let Some(invalidator) = &self.invalidator {
                if !invalidator.is_empty() {
                    self.invalidate_entries(
                        invalidator,
                        &mut deqs,
                        &mut timer_wheel,
                        eviction_batch_size,
                        &mut eviction_state,
                    );
                }
            }

            // Evict if this cache has more entries than its capacity.
            let weights_to_evict = self.weights_to_evict(&eviction_state.counters);
            if weights_to_evict > 0 {
                self.evict_lru_entries(
                    &mut deqs,
                    &mut timer_wheel,
                    eviction_batch_size,
                    weights_to_evict,
                    &mut eviction_state,
                );
            }

            // Check whether to continue this loop or not.

            should_process_logs = calls <= max_log_sync_repeats
                && (self.read_op_ch.len() >= READ_LOG_FLUSH_POINT
                    || self.write_op_ch.len() >= WRITE_LOG_FLUSH_POINT);

            let should_evict_more_entries = eviction_state.more_entries_to_evict
                // Check if there were any entries evicted in this loop.
                && (eviction_state.counters.eviction_count - last_eviction_count) > 0;

            // Break the loop if there will be nothing to do in next loop.
            if !should_process_logs && !should_evict_more_entries {
                break;
            }

            // Break the loop if the eviction listener is set and timeout has been
            // reached.
            if let (Some(to), Some(started)) = (timeout, started_at) {
                let elapsed = self.current_time().saturating_duration_since(started);
                if elapsed >= to {
                    break;
                }
            }
        }

        debug_assert_eq!(self.entry_count.load(), current_ec);
        debug_assert_eq!(self.weighted_size.load(), current_ws);
        self.entry_count.store(eviction_state.counters.entry_count);
        self.weighted_size
            .store(eviction_state.counters.weighted_size);

        crossbeam_epoch::pin().flush();

        // Ensure the deqs lock is held until here.
        drop(deqs);

        eviction_state.more_entries_to_evict
    }
}

//
// private methods
//
impl<K, V, S> Inner<K, V, S>
where
    K: Hash + Eq + Send + Sync + 'static,
    V: Send + Sync + 'static,
    S: BuildHasher + Clone + Send + Sync + 'static,
{
    fn has_enough_capacity(&self, candidate_weight: u32, counters: &EvictionCounters) -> bool {
        self.max_capacity.map_or(true, |limit| {
            counters.weighted_size + candidate_weight as u64 <= limit
        })
    }

    fn weights_to_evict(&self, counters: &EvictionCounters) -> u64 {
        self.max_capacity
            .map(|limit| counters.weighted_size.saturating_sub(limit))
            .unwrap_or_default()
    }

    #[inline]
    fn should_enable_frequency_sketch(&self, counters: &EvictionCounters) -> bool {
        match self.max_capacity {
            None | Some(0) => false,
            Some(max_cap) => {
                if self.frequency_sketch_enabled.load(Ordering::Acquire) {
                    false // The frequency sketch is already enabled.
                } else {
                    counters.weighted_size >= max_cap / 2
                }
            }
        }
    }

    #[inline]
    fn enable_frequency_sketch(&self, counters: &EvictionCounters) {
        if let Some(max_cap) = self.max_capacity {
            let c = counters;
            let cap = if self.weigher.is_none() {
                max_cap
            } else {
                (c.entry_count as f64 * (c.weighted_size as f64 / max_cap as f64)) as u64
            };
            self.do_enable_frequency_sketch(cap);
        }
    }

    #[cfg(test)]
    fn enable_frequency_sketch_for_testing(&self) {
        if let Some(max_cap) = self.max_capacity {
            self.do_enable_frequency_sketch(max_cap);
        }
    }

    #[inline]
    fn do_enable_frequency_sketch(&self, cache_capacity: u64) {
        let skt_capacity = common::sketch_capacity(cache_capacity);
        self.frequency_sketch.write().ensure_capacity(skt_capacity);
        self.frequency_sketch_enabled.store(true, Ordering::Release);
    }

    fn apply_reads(&self, deqs: &mut Deques<K>, timer_wheel: &mut TimerWheel<K>, count: usize) {
        use ReadOp::{Hit, Miss};
        let mut freq = self.frequency_sketch.write();
        let ch = &self.read_op_ch;
        for _ in 0..count {
            match ch.try_recv() {
                Ok(Hit {
                    value_entry,
                    is_expiry_modified,
                }) => {
                    let kh = value_entry.entry_info().key_hash();
                    freq.increment(kh.hash);
                    if is_expiry_modified {
                        self.update_timer_wheel(&value_entry, timer_wheel);
                    }
                    deqs.move_to_back_ao(&value_entry);
                }
                Ok(Miss(hash)) => freq.increment(hash),
                Err(_) => break,
            }
        }
    }

    fn apply_writes(
        &self,
        deqs: &mut Deques<K>,
        timer_wheel: &mut TimerWheel<K>,
        count: usize,
        eviction_state: &mut EvictionState<'_, K, V>,
    ) where
        V: Clone,
    {
        use WriteOp::{Remove, Upsert};
        let freq = self.frequency_sketch.read();
        let ch = &self.write_op_ch;

        for _ in 0..count {
            match ch.try_recv() {
                Ok(Upsert {
                    key_hash: kh,
                    value_entry: entry,
                    entry_gen: gen,
                    old_weight,
                    new_weight,
                }) => self.handle_upsert(
                    kh,
                    entry,
                    gen,
                    old_weight,
                    new_weight,
                    deqs,
                    timer_wheel,
                    &freq,
                    eviction_state,
                ),
                Ok(Remove {
                    kv_entry: KvEntry { key: _key, entry },
                    entry_gen: gen,
                }) => {
                    Self::handle_remove(
                        deqs,
                        timer_wheel,
                        entry,
                        Some(gen),
                        &mut eviction_state.counters,
                    );
                }
                Err(_) => break,
            };
        }
    }

    #[allow(clippy::too_many_arguments)]
    fn handle_upsert(
        &self,
        kh: KeyHash<K>,
        entry: MiniArc<ValueEntry<K, V>>,
        gen: u16,
        old_weight: u32,
        new_weight: u32,
        deqs: &mut Deques<K>,
        timer_wheel: &mut TimerWheel<K>,
        freq: &FrequencySketch,
        eviction_state: &mut EvictionState<'_, K, V>,
    ) where
        V: Clone,
    {
        {
            let counters = &mut eviction_state.counters;

            if entry.is_admitted() {
                // The entry has been already admitted, so treat this as an update.
                counters.saturating_sub(0, old_weight);
                counters.saturating_add(0, new_weight);
                self.update_timer_wheel(&entry, timer_wheel);
                deqs.move_to_back_ao(&entry);
                deqs.move_to_back_wo(&entry);
                entry.entry_info().set_policy_gen(gen);
                return;
            }

            if self.has_enough_capacity(new_weight, counters) {
                // There are enough room in the cache (or the cache is unbounded).
                // Add the candidate to the deques.
                self.handle_admit(&entry, new_weight, deqs, timer_wheel, counters);
                entry.entry_info().set_policy_gen(gen);
                return;
            }
        }

        if let Some(max) = self.max_capacity {
            if new_weight as u64 > max {
                // The candidate is too big to fit in the cache. Reject it.

                // Lock the key for removal if blocking removal notification is enabled.
                let kl = self.maybe_key_lock(&kh.key);
                let _klg = &kl.as_ref().map(|kl| kl.lock());

                let removed = self.cache.remove_if(
                    kh.hash,
                    |k| k == &kh.key,
                    |_, current_entry| {
                        MiniArc::ptr_eq(entry.entry_info(), current_entry.entry_info())
                            && current_entry.entry_info().entry_gen() == gen
                    },
                );
                if let Some(entry) = removed {
                    if eviction_state.is_notifier_enabled() {
                        let key = Arc::clone(&kh.key);
                        eviction_state.notify_entry_removal(key, &entry, RemovalCause::Size);
                    }
                    eviction_state.counters.incr_eviction_count();
                }
                entry.entry_info().set_policy_gen(gen);
                return;
            }
        }

        // TODO: Refactoring the policy implementations.
        // https://github.com/moka-rs/moka/issues/389

        // Try to admit the candidate.
        let admission_result = match &self.eviction_policy {
            EvictionPolicyConfig::TinyLfu => {
                let mut candidate = EntrySizeAndFrequency::new(new_weight);
                candidate.add_frequency(freq, kh.hash);
                Self::admit(&candidate, &self.cache, deqs, freq)
            }
            EvictionPolicyConfig::Lru => AdmissionResult::Admitted {
                victim_keys: SmallVec::default(),
            },
        };

        match admission_result {
            AdmissionResult::Admitted { victim_keys } => {
                // Try to remove the victims from the hash map.
                for (vic_kh, vic_la) in victim_keys {
                    let vic_key = vic_kh.key;
                    let vic_hash = vic_kh.hash;

                    // Lock the key for removal if blocking removal notification is enabled.
                    let kl = self.maybe_key_lock(&vic_key);
                    let _klg = &kl.as_ref().map(|kl| kl.lock());

                    if let Some((vic_key, vic_entry)) = self.cache.remove_entry_if_and(
                        vic_hash,
                        |k| k == &vic_key,
                        |_, entry| entry.entry_info().last_accessed() == vic_la,
                        |k, v| (k.clone(), v.clone()),
                    ) {
                        if eviction_state.is_notifier_enabled() {
                            eviction_state.notify_entry_removal(
                                vic_key,
                                &vic_entry,
                                RemovalCause::Size,
                            );
                        }
                        eviction_state.counters.incr_eviction_count();
                        // And then remove the victim from the deques.
                        Self::handle_remove(
                            deqs,
                            timer_wheel,
                            vic_entry,
                            None,
                            &mut eviction_state.counters,
                        );
                    } else {
                        // Could not remove the victim from the cache. Skip it as its
                        // ValueEntry might have been invalidated.
                        if let Some(node) = deqs.probation.peek_front() {
                            if node.element.key() == &vic_key && node.element.hash() == vic_hash {
                                deqs.probation.move_front_to_back();
                            }
                        }
                    }
                }

                // Add the candidate to the deques.
                self.handle_admit(
                    &entry,
                    new_weight,
                    deqs,
                    timer_wheel,
                    &mut eviction_state.counters,
                );
                entry.entry_info().set_policy_gen(gen);
            }
            AdmissionResult::Rejected => {
                // Lock the key for removal if blocking removal notification is enabled.
                let kl = self.maybe_key_lock(&kh.key);
                let _klg = &kl.as_ref().map(|kl| kl.lock());

                // Remove the candidate from the cache (hash map) if the entry
                // generation matches.
                let key = Arc::clone(&kh.key);
                let removed = self.cache.remove_if(
                    kh.hash,
                    |k| k == &key,
                    |_, current_entry| {
                        MiniArc::ptr_eq(entry.entry_info(), current_entry.entry_info())
                            && current_entry.entry_info().entry_gen() == gen
                    },
                );

                if let Some(entry) = removed {
                    entry.entry_info().set_policy_gen(gen);
                    if eviction_state.is_notifier_enabled() {
                        eviction_state.notify_entry_removal(key, &entry, RemovalCause::Size);
                    }
                    eviction_state.counters.incr_eviction_count();
                }
            }
        };
    }

    /// Performs size-aware admission explained in the paper:
    /// [Lightweight Robust Size Aware Cache Management][size-aware-cache-paper]
    /// by Gil Einziger, Ohad Eytan, Roy Friedman, Ben Manes.
    ///
    /// [size-aware-cache-paper]: https://arxiv.org/abs/2105.08770
    ///
    /// There are some modifications in this implementation:
    /// - To admit to the main space, candidate's frequency must be higher than
    ///   the aggregated frequencies of the potential victims. (In the paper,
    ///   `>=` operator is used rather than `>`)  The `>` operator will do a better
    ///   job to prevent the main space from polluting.
    /// - When a candidate is rejected, the potential victims will stay at the LRU
    ///   position of the probation access-order queue. (In the paper, they will be
    ///   promoted (to the MRU position?) to force the eviction policy to select a
    ///   different set of victims for the next candidate). We may implement the
    ///   paper's behavior later?
    ///
    #[inline]
    fn admit(
        candidate: &EntrySizeAndFrequency,
        cache: &CacheStore<K, V, S>,
        deqs: &mut Deques<K>,
        freq: &FrequencySketch,
    ) -> AdmissionResult<K> {
        const MAX_CONSECUTIVE_RETRIES: usize = 5;
        let mut retries = 0;

        let mut victims = EntrySizeAndFrequency::default();
        let mut victim_keys = SmallVec::default();

        let deq = &mut deqs.probation;

        // Get first potential victim at the LRU position.
        let mut next_victim = deq.peek_front_ptr();

        // Aggregate potential victims.
        while victims.policy_weight < candidate.policy_weight
            && victims.freq <= candidate.freq
            && retries <= MAX_CONSECUTIVE_RETRIES
        {
            let Some(victim) = next_victim.take() else {
                // No more potential victims.
                break;
            };
            next_victim = DeqNode::next_node_ptr(victim);

            let vic_elem = &unsafe { victim.as_ref() }.element;
            if vic_elem.is_dirty() {
                // Skip this node as its ValueEntry have been updated or invalidated.
                unsafe { deq.move_to_back(victim) };
                retries += 1;
                continue;
            }

            let key = vic_elem.key();
            let hash = vic_elem.hash();
            let last_accessed = vic_elem.entry_info().last_accessed();

            if let Some(vic_entry) = cache.get(hash, |k| k == key) {
                victims.add_policy_weight(vic_entry.policy_weight());
                victims.add_frequency(freq, hash);
                victim_keys.push((KeyHash::new(Arc::clone(key), hash), last_accessed));
                retries = 0;
            } else {
                // Could not get the victim from the cache (hash map). Skip this node
                // as its ValueEntry might have been invalidated (after we checked
                // `is_dirty` above`).
                unsafe { deq.move_to_back(victim) };
                retries += 1;
            }
        }

        // Admit or reject the candidate.

        // TODO: Implement some randomness to mitigate hash DoS attack.
        // See Caffeine's implementation.

        if victims.policy_weight >= candidate.policy_weight && candidate.freq > victims.freq {
            AdmissionResult::Admitted { victim_keys }
        } else {
            AdmissionResult::Rejected
        }
    }

    fn handle_admit(
        &self,
        entry: &MiniArc<ValueEntry<K, V>>,
        policy_weight: u32,
        deqs: &mut Deques<K>,
        timer_wheel: &mut TimerWheel<K>,
        counters: &mut EvictionCounters,
    ) {
        counters.saturating_add(1, policy_weight);

        self.update_timer_wheel(entry, timer_wheel);

        // Update the deques.
        deqs.push_back_ao(
            CacheRegion::MainProbation,
            KeyHashDate::new(entry.entry_info()),
            entry,
        );
        if self.is_write_order_queue_enabled() {
            deqs.push_back_wo(KeyHashDate::new(entry.entry_info()), entry);
        }
        entry.set_admitted(true);
    }

    /// NOTE: This method may enable the timer wheel.
    fn update_timer_wheel(
        &self,
        entry: &MiniArc<ValueEntry<K, V>>,
        timer_wheel: &mut TimerWheel<K>,
    ) {
        // Enable the timer wheel if needed.
        if entry.entry_info().expiration_time().is_some() && !timer_wheel.is_enabled() {
            timer_wheel.enable();
        }

        // Update the timer wheel.
        match (
            entry.entry_info().expiration_time().is_some(),
            entry.timer_node(),
        ) {
            // Do nothing; the cache entry has no expiration time and not registered
            // to the timer wheel.
            (false, None) => (),
            // Register the cache entry to the timer wheel; the cache entry has an
            // expiration time and not registered to the timer wheel.
            (true, None) => {
                let timer = timer_wheel.schedule(
                    MiniArc::clone(entry.entry_info()),
                    MiniArc::clone(entry.deq_nodes()),
                );
                entry.set_timer_node(timer);
            }
            // Reschedule the cache entry in the timer wheel; the cache entry has an
            // expiration time and already registered to the timer wheel.
            (true, Some(tn)) => {
                let result = timer_wheel.reschedule(tn);
                if let ReschedulingResult::Removed(removed_tn) = result {
                    // The timer node was removed from the timer wheel because the
                    // expiration time has been unset by other thread after we
                    // checked.
                    entry.set_timer_node(None);
                    drop(removed_tn);
                }
            }
            // Unregister the cache entry from the timer wheel; the cache entry has
            // no expiration time but registered to the timer wheel.
            (false, Some(tn)) => {
                entry.set_timer_node(None);
                timer_wheel.deschedule(tn);
            }
        }
    }

    fn handle_remove(
        deqs: &mut Deques<K>,
        timer_wheel: &mut TimerWheel<K>,
        entry: MiniArc<ValueEntry<K, V>>,
        gen: Option<u16>,
        counters: &mut EvictionCounters,
    ) {
        if let Some(timer_node) = entry.take_timer_node() {
            timer_wheel.deschedule(timer_node);
        }
        Self::handle_remove_without_timer_wheel(deqs, entry, gen, counters);
    }

    fn handle_remove_without_timer_wheel(
        deqs: &mut Deques<K>,
        entry: MiniArc<ValueEntry<K, V>>,
        gen: Option<u16>,
        counters: &mut EvictionCounters,
    ) {
        if entry.is_admitted() {
            entry.set_admitted(false);
            counters.saturating_sub(1, entry.policy_weight());
            // The following two unlink_* functions will unset the deq nodes.
            deqs.unlink_ao(&entry);
            Deques::unlink_wo(&mut deqs.write_order, &entry);
        } else {
            entry.unset_q_nodes();
        }
        if let Some(g) = gen {
            entry.entry_info().set_policy_gen(g);
        }
    }

    fn handle_remove_with_deques(
        ao_deq_name: &str,
        ao_deq: &mut Deque<KeyHashDate<K>>,
        wo_deq: &mut Deque<KeyHashDate<K>>,
        timer_wheel: &mut TimerWheel<K>,
        entry: MiniArc<ValueEntry<K, V>>,
        counters: &mut EvictionCounters,
    ) {
        if let Some(timer) = entry.take_timer_node() {
            timer_wheel.deschedule(timer);
        }
        if entry.is_admitted() {
            entry.set_admitted(false);
            counters.saturating_sub(1, entry.policy_weight());
            // The following two unlink_* functions will unset the deq nodes.
            Deques::unlink_ao_from_deque(ao_deq_name, ao_deq, &entry);
            Deques::unlink_wo(wo_deq, &entry);
        } else {
            entry.unset_q_nodes();
        }
    }

    fn evict_expired_entries_using_timers(
        &self,
        timer_wheel: &mut TimerWheel<K>,
        deqs: &mut Deques<K>,
        eviction_state: &mut EvictionState<'_, K, V>,
    ) where
        V: Clone,
    {
        use crate::common::timer_wheel::TimerEvent;

        let now = self.current_time();

        // NOTES:
        //
        // 1. When necessary, the iterator returned from advance() will unset the
        //    timer node pointer in the `ValueEntry`, so we do not have to do it
        //    here.
        // 2. If an entry is dirty or `cache.remove_if` returns `None`, we will skip
        //    it as it may have been read, updated or invalidated by other thread.
        //    - The timer node should have been unset in the current `ValueEntry` as
        //      described above.
        //    - When necessary, a new timer node will be recreated for the current or
        //      new `ValueEntry` when its `WriteOp` or `ReadOp` is processed.
        for event in timer_wheel.advance(now) {
            // We do not have to do anything if event is `TimerEvent::Descheduled(_)`
            // or `TimerEvent::Rescheduled(_)`.
            if let TimerEvent::Expired(node) = event {
                let entry_info = node.element.entry_info();

                if entry_info.is_dirty() {
                    // Skip this entry as it has been updated or invalidated by other
                    // thread.
                    continue;
                }

                let kh = entry_info.key_hash();
                let key = &kh.key;
                let hash = kh.hash;

                // Lock the key for removal if blocking removal notification is
                // enabled.
                let kl = self.maybe_key_lock(key);
                let _klg = &kl.as_ref().map(|kl| kl.lock());

                // Remove the key from the map only when the entry is really expired.
                let maybe_entry = self.cache.remove_if(
                    hash,
                    |k| k == key,
                    |_, v| is_expired_by_per_entry_ttl(v.entry_info(), now),
                );

                if let Some(entry) = maybe_entry {
                    if eviction_state.is_notifier_enabled() {
                        let key = Arc::clone(key);
                        eviction_state.notify_entry_removal(key, &entry, RemovalCause::Expired);
                    }
                    eviction_state.counters.incr_eviction_count();
                    Self::handle_remove_without_timer_wheel(
                        deqs,
                        entry,
                        None,
                        &mut eviction_state.counters,
                    );
                } else {
                    // Skip this entry as the key may have been read, updated or
                    // invalidated by other thread.
                }
            }
        }
    }

    fn evict_expired_entries_using_deqs(
        &self,
        deqs: &mut Deques<K>,
        timer_wheel: &mut TimerWheel<K>,
        batch_size: u32,
        state: &mut EvictionState<'_, K, V>,
    ) where
        V: Clone,
    {
        use CacheRegion::{MainProbation as Probation, MainProtected as Protected, Window};

        let now = self.current_time();

        if self.is_write_order_queue_enabled() {
            self.remove_expired_wo(deqs, timer_wheel, batch_size, now, state);
        }

        if self.expiration_policy.time_to_idle().is_some() || self.has_valid_after() {
            self.remove_expired_ao(Window, deqs, timer_wheel, batch_size, now, state);
            self.remove_expired_ao(Probation, deqs, timer_wheel, batch_size, now, state);
            self.remove_expired_ao(Protected, deqs, timer_wheel, batch_size, now, state);
        }
    }

    #[allow(clippy::too_many_arguments)]
    #[inline]
    fn remove_expired_ao(
        &self,
        cache_region: CacheRegion,
        deqs: &mut Deques<K>,
        timer_wheel: &mut TimerWheel<K>,
        batch_size: u32,
        now: Instant,
        eviction_state: &mut EvictionState<'_, K, V>,
    ) where
        V: Clone,
    {
        let tti = &self.expiration_policy.time_to_idle();
        let va = &self.valid_after();
        let deq_name = cache_region.name();
        let (ao_deq, wo_deq) = deqs.select_mut(cache_region);
        let mut more_to_evict = true;

        for _ in 0..batch_size {
            let maybe_key_hash_ts = ao_deq.peek_front().map(|node| {
                let elem = &node.element;
                (
                    Arc::clone(elem.key()),
                    elem.hash(),
                    elem.is_dirty(),
                    elem.last_accessed(),
                )
            });

            let (key, hash, cause) = match maybe_key_hash_ts {
                Some((key, hash, false, Some(ts))) => {
                    let cause = match is_entry_expired_ao_or_invalid(tti, va, ts, now) {
                        (true, _) => RemovalCause::Expired,
                        (false, true) => RemovalCause::Explicit,
                        (false, false) => {
                            more_to_evict = false;
                            break;
                        }
                    };
                    (key, hash, cause)
                }
                // TODO: Remove the second pattern `Some((_key, false, None))` once
                // we change `last_modified` and `last_accessed` in `EntryInfo` from
                // `Option<Instant>` to `Instant`.
                Some((key, hash, true, _) | (key, hash, false, None)) => {
                    // `is_dirty` is true or `last_modified` is None. Skip this entry
                    // as it may have been updated by this or other async task but
                    // its `WriteOp` is not processed yet.
                    self.skip_updated_entry_ao(&key, hash, deq_name, ao_deq, wo_deq);
                    // Set `more_to_evict` to `false` to make `run_pending_tasks` to
                    // return early. This will help that `schedule_write_op` to send
                    // the `WriteOp` to the write op channel.
                    more_to_evict = false;
                    continue;
                }
                None => {
                    more_to_evict = false;
                    break;
                }
            };

            // Lock the key for removal if blocking removal notification is enabled.
            let kl = self.maybe_key_lock(&key);
            let _klg = &kl.as_ref().map(|kl| kl.lock());

            // Remove the key from the map only when the entry is really
            // expired. This check is needed because it is possible that the entry in
            // the map has been updated or deleted but its deque node we checked
            // above has not been updated yet.
            let maybe_entry = self.cache.remove_if(
                hash,
                |k| k == &key,
                |_, v| is_expired_entry_ao(tti, va, v, now),
            );

            if let Some(entry) = maybe_entry {
                if eviction_state.is_notifier_enabled() {
                    eviction_state.notify_entry_removal(key, &entry, cause);
                }
                eviction_state.counters.incr_eviction_count();
                Self::handle_remove_with_deques(
                    deq_name,
                    ao_deq,
                    wo_deq,
                    timer_wheel,
                    entry,
                    &mut eviction_state.counters,
                );
            } else {
                self.skip_updated_entry_ao(&key, hash, deq_name, ao_deq, wo_deq);
                more_to_evict = false;
            }
        }

        if more_to_evict {
            eviction_state.more_entries_to_evict = true;
        }
    }

    #[inline]
    fn skip_updated_entry_ao(
        &self,
        key: &K,
        hash: u64,
        deq_name: &str,
        deq: &mut Deque<KeyHashDate<K>>,
        write_order_deq: &mut Deque<KeyHashDate<K>>,
    ) {
        if let Some(entry) = self.cache.get(hash, |k| (k.borrow() as &K) == key) {
            // The key exists and the entry may have been read or updated by other
            // thread.
            Deques::move_to_back_ao_in_deque(deq_name, deq, &entry);
            if entry.is_dirty() {
                Deques::move_to_back_wo_in_deque(write_order_deq, &entry);
            }
        } else {
            // Skip this entry as the key may have been invalidated by other thread.
            // Since the invalidated ValueEntry (which should be still in the write
            // op queue) has a pointer to this node, move the node to the back of the
            // deque instead of popping (dropping) it.
            deq.move_front_to_back();
        }
    }

    #[inline]
    fn skip_updated_entry_wo(&self, key: &K, hash: u64, deqs: &mut Deques<K>) {
        if let Some(entry) = self.cache.get(hash, |k| (k.borrow() as &K) == key) {
            // The key exists and the entry may have been read or updated by other
            // thread.
            deqs.move_to_back_ao(&entry);
            deqs.move_to_back_wo(&entry);
        } else {
            // Skip this entry as the key may have been invalidated by other thread.
            // Since the invalidated `ValueEntry` (which should be still in the write
            // op queue) has a pointer to this node, move the node to the back of the
            // deque instead of popping (dropping) it.
            deqs.write_order.move_front_to_back();
        }
    }

    #[inline]
    fn remove_expired_wo(
        &self,
        deqs: &mut Deques<K>,
        timer_wheel: &mut TimerWheel<K>,
        batch_size: u32,
        now: Instant,
        eviction_state: &mut EvictionState<'_, K, V>,
    ) where
        V: Clone,
    {
        let ttl = &self.expiration_policy.time_to_live();
        let va = &self.valid_after();
        let mut more_to_evict = true;

        for _ in 0..batch_size {
            let maybe_key_hash_ts = deqs.write_order.peek_front().map(|node| {
                let elem = &node.element;
                (
                    Arc::clone(elem.key()),
                    elem.hash(),
                    elem.is_dirty(),
                    elem.last_modified(),
                )
            });

            let (key, hash, cause) = match maybe_key_hash_ts {
                Some((key, hash, false, Some(ts))) => {
                    let cause = match is_entry_expired_wo_or_invalid(ttl, va, ts, now) {
                        (true, _) => RemovalCause::Expired,
                        (false, true) => RemovalCause::Explicit,
                        (false, false) => {
                            more_to_evict = false;
                            break;
                        }
                    };
                    (key, hash, cause)
                }
                // TODO: Remove the second pattern `Some((_key, false, None))` once
                // we change `last_modified` and `last_accessed` in `EntryInfo` from
                // `Option<Instant>` to `Instant`.
                Some((key, hash, true, _) | (key, hash, false, None)) => {
                    self.skip_updated_entry_wo(&key, hash, deqs);
                    more_to_evict = false;
                    continue;
                }
                None => {
                    more_to_evict = false;
                    break;
                }
            };

            // Lock the key for removal if blocking removal notification is enabled.
            let kl = self.maybe_key_lock(&key);
            let _klg = &kl.as_ref().map(|kl| kl.lock());

            let maybe_entry = self.cache.remove_if(
                hash,
                |k| k == &key,
                |_, v| is_expired_entry_wo(ttl, va, v, now),
            );

            if let Some(entry) = maybe_entry {
                if eviction_state.is_notifier_enabled() {
                    eviction_state.notify_entry_removal(key, &entry, cause);
                }
                eviction_state.counters.incr_eviction_count();
                Self::handle_remove(deqs, timer_wheel, entry, None, &mut eviction_state.counters);
            } else {
                self.skip_updated_entry_wo(&key, hash, deqs);
                more_to_evict = false;
            }
        }

        if more_to_evict {
            eviction_state.more_entries_to_evict = true;
        }
    }

    fn invalidate_entries(
        &self,
        invalidator: &Invalidator<K, V, S>,
        deqs: &mut Deques<K>,
        timer_wheel: &mut TimerWheel<K>,
        batch_size: u32,
        eviction_state: &mut EvictionState<'_, K, V>,
    ) where
        V: Clone,
    {
        let now = self.current_time();

        // If the write order queue is empty, we are done and can remove the predicates
        // that have been registered by now.
        if deqs.write_order.len() == 0 {
            invalidator.remove_predicates_registered_before(now);
            return;
        }

        let mut candidates = Vec::new();
        let mut len = 0;
        let has_next;
        {
            let iter = &mut deqs.write_order.peekable();

            while len < batch_size {
                if let Some(kd) = iter.next() {
                    if !kd.is_dirty() {
                        if let Some(ts) = kd.last_modified() {
                            let key = kd.key();
                            let hash = self.hash(&**key);
                            candidates.push(KeyDateLite::new(key, hash, ts));
                            len += 1;
                        }
                    }
                } else {
                    break;
                }
            }

            has_next = iter.peek().is_some();
        }

        if len == 0 {
            return;
        }

        let is_truncated = len == batch_size && has_next;
        let (invalidated, is_done) =
            invalidator.scan_and_invalidate(self, candidates, is_truncated);

        for KvEntry { key: _key, entry } in invalidated {
            Self::handle_remove(deqs, timer_wheel, entry, None, &mut eviction_state.counters);
        }
        if is_done {
            deqs.write_order.reset_cursor();
        }
        if !invalidator.is_empty() {
            eviction_state.more_entries_to_evict = true;
        }
    }

    fn evict_lru_entries(
        &self,
        deqs: &mut Deques<K>,
        timer_wheel: &mut TimerWheel<K>,
        batch_size: u32,
        weights_to_evict: u64,
        eviction_state: &mut EvictionState<'_, K, V>,
    ) where
        V: Clone,
    {
        const CACHE_REGION: CacheRegion = CacheRegion::MainProbation;
        let deq_name = CACHE_REGION.name();
        let (ao_deq, wo_deq) = deqs.select_mut(CACHE_REGION);
        let mut evicted = 0u64;
        let mut more_to_evict = true;

        for _ in 0..batch_size {
            if evicted >= weights_to_evict {
                more_to_evict = false;
                break;
            }

            let maybe_key_hash_ts = ao_deq.peek_front().map(|node| {
                let entry_info = node.element.entry_info();
                (
                    Arc::clone(node.element.key()),
                    node.element.hash(),
                    entry_info.is_dirty(),
                    entry_info.last_accessed(),
                )
            });

            let (key, hash, ts) = match maybe_key_hash_ts {
                Some((key, hash, false, Some(ts))) => (key, hash, ts),
                // TODO: Remove the second pattern `Some((_key, false, None))` once we change
                // `last_modified` and `last_accessed` in `EntryInfo` from `Option<Instant>` to
                // `Instant`.
                Some((key, hash, true, _) | (key, hash, false, None)) => {
                    // `is_dirty` is true or `last_modified` is None. Skip this entry
                    // as it may have been updated by this or other async task but
                    // its `WriteOp` is not processed yet.
                    self.skip_updated_entry_ao(&key, hash, deq_name, ao_deq, wo_deq);
                    // Set `more_to_evict` to `false` to make `run_pending_tasks` to
                    // return early. This will help that `schedule_write_op` to send
                    // the `WriteOp` to the write op channel.
                    more_to_evict = false;
                    continue;
                }
                None => {
                    more_to_evict = false;
                    break;
                }
            };

            // Lock the key for removal if blocking removal notification is enabled.
            let kl = self.maybe_key_lock(&key);
            let _klg = &kl.as_ref().map(|kl| kl.lock());

            let maybe_entry = self.cache.remove_if(
                hash,
                |k| k == &key,
                |_, v| {
                    if let Some(la) = v.last_accessed() {
                        la == ts
                    } else {
                        false
                    }
                },
            );

            if let Some(entry) = maybe_entry {
                if eviction_state.is_notifier_enabled() {
                    eviction_state.notify_entry_removal(key, &entry, RemovalCause::Size);
                }
                eviction_state.counters.incr_eviction_count();
                let weight = entry.policy_weight();
                Self::handle_remove_with_deques(
                    deq_name,
                    ao_deq,
                    wo_deq,
                    timer_wheel,
                    entry,
                    &mut eviction_state.counters,
                );
                evicted = evicted.saturating_add(weight as u64);
            } else {
                self.skip_updated_entry_ao(&key, hash, deq_name, ao_deq, wo_deq);
                more_to_evict = false;
            }
        }

        if more_to_evict {
            eviction_state.more_entries_to_evict = true;
        }
    }
}

impl<K, V, S> Inner<K, V, S>
where
    K: Send + Sync + 'static,
    V: Clone + Send + Sync + 'static,
{
    pub(crate) fn notify_single_removal(
        &self,
        key: Arc<K>,
        entry: &MiniArc<ValueEntry<K, V>>,
        cause: RemovalCause,
    ) {
        if let Some(notifier) = &self.removal_notifier {
            notifier.notify(key, entry.value.clone(), cause);
        }
    }

    #[inline]
    fn notify_upsert(
        &self,
        key: Arc<K>,
        entry: &MiniArc<ValueEntry<K, V>>,
        last_accessed: Option<Instant>,
        last_modified: Option<Instant>,
    ) {
        let now = self.current_time();
        let exp = &self.expiration_policy;

        let mut cause = RemovalCause::Replaced;

        if let Some(last_accessed) = last_accessed {
            if is_expired_by_tti(&exp.time_to_idle(), last_accessed, now) {
                cause = RemovalCause::Expired;
            }
        }

        if let Some(last_modified) = last_modified {
            if is_expired_by_ttl(&exp.time_to_live(), last_modified, now) {
                cause = RemovalCause::Expired;
            } else if is_invalid_entry(&self.valid_after(), last_modified) {
                cause = RemovalCause::Explicit;
            }
        }

        self.notify_single_removal(key, entry, cause);
    }

    #[inline]
    fn notify_invalidate(&self, key: &Arc<K>, entry: &MiniArc<ValueEntry<K, V>>) {
        let now = self.current_time();
        let exp = &self.expiration_policy;

        let mut cause = RemovalCause::Explicit;

        if let Some(last_accessed) = entry.last_accessed() {
            if is_expired_by_tti(&exp.time_to_idle(), last_accessed, now) {
                cause = RemovalCause::Expired;
            }
        }

        if let Some(last_modified) = entry.last_modified() {
            if is_expired_by_ttl(&exp.time_to_live(), last_modified, now) {
                cause = RemovalCause::Expired;
            }
        }

        self.notify_single_removal(Arc::clone(key), entry, cause);
    }
}

//
// for testing
//
#[cfg(test)]
impl<K, V, S> Inner<K, V, S>
where
    K: Hash + Eq,
    S: BuildHasher + Clone,
{
    fn invalidation_predicate_count(&self) -> usize {
        if let Some(inv) = &self.invalidator {
            inv.predicate_count()
        } else {
            0
        }
    }

    fn key_locks_map_is_empty(&self) -> bool {
        self.key_locks
            .as_ref()
            .map(|m| m.is_empty())
            // If key_locks is None, consider it is empty.
            .unwrap_or(true)
    }
}

//
// private free-standing functions
//

/// Returns `true` if this entry is expired by its per-entry TTL.
#[inline]
fn is_expired_by_per_entry_ttl<K>(entry_info: &MiniArc<EntryInfo<K>>, now: Instant) -> bool {
    if let Some(ts) = entry_info.expiration_time() {
        ts <= now
    } else {
        false
    }
}

/// Returns `true` when one of the followings conditions is met:
///
/// - This entry is expired by the time-to-idle config of this cache instance.
/// - Or, it is invalidated by the `invalidate_all` method.
#[inline]
fn is_expired_entry_ao(
    time_to_idle: &Option<Duration>,
    valid_after: &Option<Instant>,
    entry: &impl AccessTime,
    now: Instant,
) -> bool {
    if let Some(ts) = entry.last_accessed() {
        is_invalid_entry(valid_after, ts) || is_expired_by_tti(time_to_idle, ts, now)
    } else {
        false
    }
}

/// Returns `true` when one of the following conditions is met:
///
/// - This entry is expired by the time-to-live (TTL) config of this cache instance.
/// - Or, it is invalidated by the `invalidate_all` method.
#[inline]
fn is_expired_entry_wo(
    time_to_live: &Option<Duration>,
    valid_after: &Option<Instant>,
    entry: &impl AccessTime,
    now: Instant,
) -> bool {
    if let Some(ts) = entry.last_modified() {
        is_invalid_entry(valid_after, ts) || is_expired_by_ttl(time_to_live, ts, now)
    } else {
        false
    }
}

#[inline]
fn is_entry_expired_ao_or_invalid(
    time_to_idle: &Option<Duration>,
    valid_after: &Option<Instant>,
    entry_last_accessed: Instant,
    now: Instant,
) -> (bool, bool) {
    let ts = entry_last_accessed;
    let expired = is_expired_by_tti(time_to_idle, ts, now);
    let invalid = is_invalid_entry(valid_after, ts);
    (expired, invalid)
}

#[inline]
fn is_entry_expired_wo_or_invalid(
    time_to_live: &Option<Duration>,
    valid_after: &Option<Instant>,
    entry_last_modified: Instant,
    now: Instant,
) -> (bool, bool) {
    let ts = entry_last_modified;
    let expired = is_expired_by_ttl(time_to_live, ts, now);
    let invalid = is_invalid_entry(valid_after, ts);
    (expired, invalid)
}

#[inline]
fn is_invalid_entry(valid_after: &Option<Instant>, entry_ts: Instant) -> bool {
    if let Some(va) = valid_after {
        entry_ts < *va
    } else {
        false
    }
}

#[inline]
fn is_expired_by_tti(
    time_to_idle: &Option<Duration>,
    entry_last_accessed: Instant,
    now: Instant,
) -> bool {
    if let Some(tti) = time_to_idle {
        let expiration = entry_last_accessed.saturating_add(*tti);
        expiration <= now
    } else {
        false
    }
}

#[inline]
fn is_expired_by_ttl(
    time_to_live: &Option<Duration>,
    entry_last_modified: Instant,
    now: Instant,
) -> bool {
    if let Some(ttl) = time_to_live {
        let expiration = entry_last_modified.saturating_add(*ttl);
        expiration <= now
    } else {
        false
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        common::{time::Clock, HousekeeperConfig},
        policy::{EvictionPolicy, ExpirationPolicy},
    };

    use super::BaseCache;

    #[cfg_attr(target_pointer_width = "16", ignore)]
    #[test]
    fn test_skt_capacity_will_not_overflow() {
        use std::collections::hash_map::RandomState;

        // power of two
        let pot = |exp| 2u64.pow(exp);

        let ensure_sketch_len = |max_capacity, len, name| {
            let cache = BaseCache::<u8, u8>::new(
                None,
                Some(max_capacity),
                None,
                RandomState::default(),
                None,
                EvictionPolicy::default(),
                None,
                ExpirationPolicy::default(),
                HousekeeperConfig::default(),
                false,
                Clock::default(),
            );
            cache.inner.enable_frequency_sketch_for_testing();
            assert_eq!(
                cache.inner.frequency_sketch.read().table_len(),
                len as usize,
                "{name}"
            );
        };

        if cfg!(target_pointer_width = "32") {
            let pot24 = pot(24);
            let pot16 = pot(16);
            ensure_sketch_len(0, 128, "0");
            ensure_sketch_len(128, 128, "128");
            ensure_sketch_len(pot16, pot16, "pot16");
            // due to ceiling to next_power_of_two
            ensure_sketch_len(pot16 + 1, pot(17), "pot16 + 1");
            // due to ceiling to next_power_of_two
            ensure_sketch_len(pot24 - 1, pot24, "pot24 - 1");
            ensure_sketch_len(pot24, pot24, "pot24");
            ensure_sketch_len(pot(27), pot24, "pot(27)");
            ensure_sketch_len(u32::MAX as u64, pot24, "u32::MAX");
        } else {
            // target_pointer_width: 64 or larger.
            let pot30 = pot(30);
            let pot16 = pot(16);
            ensure_sketch_len(0, 128, "0");
            ensure_sketch_len(128, 128, "128");
            ensure_sketch_len(pot16, pot16, "pot16");
            // due to ceiling to next_power_of_two
            ensure_sketch_len(pot16 + 1, pot(17), "pot16 + 1");

            // The following tests will allocate large memory (~8GiB).
            if !cfg!(skip_large_mem_tests) {
                // due to ceiling to next_power_of_two
                ensure_sketch_len(pot30 - 1, pot30, "pot30- 1");
                ensure_sketch_len(pot30, pot30, "pot30");
                ensure_sketch_len(u64::MAX, pot30, "u64::MAX");
            }
        };
    }

    #[test]
    fn test_per_entry_expiration() {
        use super::InnerSync;
        use crate::{common::time::Clock, Entry, Expiry};

        use std::{
            collections::hash_map::RandomState,
            sync::{Arc, Mutex},
            time::{Duration, Instant as StdInstant},
        };

        type Key = u32;
        type Value = char;

        fn current_time(cache: &BaseCache<Key, Value>) -> StdInstant {
            cache.inner.clock().to_std_instant(cache.current_time())
        }

        fn insert(cache: &BaseCache<Key, Value>, key: Key, hash: u64, value: Value) {
            let (op, _now) = cache.do_insert_with_hash(Arc::new(key), hash, value);
            cache.write_op_ch.send(op).expect("Failed to send");
        }

        macro_rules! assert_params_eq {
            ($left:expr, $right:expr, $param_name:expr, $line:expr) => {
                assert_eq!(
                    $left, $right,
                    "Mismatched `{}`s. line: {}",
                    $param_name, $line
                );
            };
        }

        macro_rules! assert_expiry {
            ($cache:ident, $key:ident, $hash:ident, $mock:ident, $duration_secs:expr) => {
                // Increment the time.
                $mock.increment(Duration::from_millis($duration_secs * 1000 - 1));
                $cache.inner.run_pending_tasks(None, 1, 10);
                assert!($cache.contains_key_with_hash(&$key, $hash));
                assert_eq!($cache.entry_count(), 1);

                // Increment the time by 1ms (3). The entry should be expired.
                $mock.increment(Duration::from_millis(1));
                $cache.inner.run_pending_tasks(None, 1, 10);
                assert!(!$cache.contains_key_with_hash(&$key, $hash));

                // Increment the time again to ensure the entry has been evicted from the
                // cache.
                $mock.increment(Duration::from_secs(1));
                $cache.inner.run_pending_tasks(None, 1, 10);
                assert_eq!($cache.entry_count(), 0);
            };
        }

        /// Contains expected call parameters and also a return value.
        #[derive(Debug)]
        enum ExpiryExpectation {
            NoCall,
            AfterCreate {
                caller_line: u32,
                key: Key,
                value: Value,
                current_time: StdInstant,
                new_duration_secs: Option<u64>,
            },
            AfterRead {
                caller_line: u32,
                key: Key,
                value: Value,
                current_time: StdInstant,
                current_duration_secs: Option<u64>,
                last_modified_at: StdInstant,
                new_duration_secs: Option<u64>,
            },
            AfterUpdate {
                caller_line: u32,
                key: Key,
                value: Value,
                current_time: StdInstant,
                current_duration_secs: Option<u64>,
                new_duration_secs: Option<u64>,
            },
        }

        impl ExpiryExpectation {
            fn after_create(
                caller_line: u32,
                key: Key,
                value: Value,
                current_time: StdInstant,
                new_duration_secs: Option<u64>,
            ) -> Self {
                Self::AfterCreate {
                    caller_line,
                    key,
                    value,
                    current_time,
                    new_duration_secs,
                }
            }

            fn after_read(
                caller_line: u32,
                key: Key,
                value: Value,
                current_time: StdInstant,
                current_duration_secs: Option<u64>,
                last_modified_at: StdInstant,
                new_duration_secs: Option<u64>,
            ) -> Self {
                Self::AfterRead {
                    caller_line,
                    key,
                    value,
                    current_time,
                    current_duration_secs,
                    last_modified_at,
                    new_duration_secs,
                }
            }

            fn after_update(
                caller_line: u32,
                key: Key,
                value: Value,
                current_time: StdInstant,
                current_duration_secs: Option<u64>,
                new_duration_secs: Option<u64>,
            ) -> Self {
                Self::AfterUpdate {
                    caller_line,
                    key,
                    value,
                    current_time,
                    current_duration_secs,
                    new_duration_secs,
                }
            }
        }

        let expectation = Arc::new(Mutex::new(ExpiryExpectation::NoCall));

        struct MyExpiry {
            expectation: Arc<Mutex<ExpiryExpectation>>,
        }

        impl Expiry<u32, char> for MyExpiry {
            fn expire_after_create(
                &self,
                actual_key: &u32,
                actual_value: &char,
                actual_current_time: StdInstant,
            ) -> Option<Duration> {
                use ExpiryExpectation::*;

                let lock = &mut *self.expectation.lock().unwrap();
                let expected = std::mem::replace(lock, NoCall);
                match expected {
                    AfterCreate {
                        caller_line,
                        key,
                        value,
                        current_time,
                        new_duration_secs: new_duration,
                    } => {
                        assert_params_eq!(*actual_key, key, "key", caller_line);
                        assert_params_eq!(*actual_value, value, "value", caller_line);
                        assert_params_eq!(
                            actual_current_time,
                            current_time,
                            "current_time",
                            caller_line
                        );
                        new_duration.map(Duration::from_secs)
                    }
                    expected => {
                        panic!(
                            "Unexpected call to expire_after_create: caller_line {}, expected: {expected:?}",
                            line!()
                        );
                    }
                }
            }

            fn expire_after_read(
                &self,
                actual_key: &u32,
                actual_value: &char,
                actual_current_time: StdInstant,
                actual_current_duration: Option<Duration>,
                actual_last_modified_at: StdInstant,
            ) -> Option<Duration> {
                use ExpiryExpectation::*;

                let lock = &mut *self.expectation.lock().unwrap();
                let expected = std::mem::replace(lock, NoCall);
                match expected {
                    AfterRead {
                        caller_line,
                        key,
                        value,
                        current_time,
                        current_duration_secs,
                        last_modified_at,
                        new_duration_secs,
                    } => {
                        assert_params_eq!(*actual_key, key, "key", caller_line);
                        assert_params_eq!(*actual_value, value, "value", caller_line);
                        assert_params_eq!(
                            actual_current_time,
                            current_time,
                            "current_time",
                            caller_line
                        );
                        assert_params_eq!(
                            actual_current_duration,
                            current_duration_secs.map(Duration::from_secs),
                            "current_duration",
                            caller_line
                        );
                        assert_params_eq!(
                            actual_last_modified_at,
                            last_modified_at,
                            "last_modified_at",
                            caller_line
                        );
                        new_duration_secs.map(Duration::from_secs)
                    }
                    expected => {
                        panic!(
                            "Unexpected call to expire_after_read: caller_line {}, expected: {expected:?}",
                            line!()
                        );
                    }
                }
            }

            fn expire_after_update(
                &self,
                actual_key: &u32,
                actual_value: &char,
                actual_current_time: StdInstant,
                actual_current_duration: Option<Duration>,
            ) -> Option<Duration> {
                use ExpiryExpectation::*;

                let lock = &mut *self.expectation.lock().unwrap();
                let expected = std::mem::replace(lock, NoCall);
                match expected {
                    AfterUpdate {
                        caller_line,
                        key,
                        value,
                        current_time,
                        current_duration_secs,
                        new_duration_secs,
                    } => {
                        assert_params_eq!(*actual_key, key, "key", caller_line);
                        assert_params_eq!(*actual_value, value, "value", caller_line);
                        assert_params_eq!(
                            actual_current_time,
                            current_time,
                            "current_time",
                            caller_line
                        );
                        assert_params_eq!(
                            actual_current_duration,
                            current_duration_secs.map(Duration::from_secs),
                            "current_duration",
                            caller_line
                        );
                        new_duration_secs.map(Duration::from_secs)
                    }
                    expected => {
                        panic!(
                            "Unexpected call to expire_after_update: caller_line {}, expected: {expected:?}",
                            line!()
                        );
                    }
                }
            }
        }

        const TTL: u64 = 16;
        const TTI: u64 = 7;
        let expiry: Option<Arc<dyn Expiry<_, _> + Send + Sync + 'static>> =
            Some(Arc::new(MyExpiry {
                expectation: Arc::clone(&expectation),
            }));

        let (clock, mock) = Clock::mock();

        let mut cache = BaseCache::<Key, Value>::new(
            None,
            None,
            None,
            RandomState::default(),
            None,
            EvictionPolicy::default(),
            None,
            ExpirationPolicy::new(
                Some(Duration::from_secs(TTL)),
                Some(Duration::from_secs(TTI)),
                expiry,
            ),
            HousekeeperConfig::default(),
            false,
            clock,
        );
        cache.reconfigure_for_testing();

        // Make the cache exterior immutable.
        let cache = cache;

        mock.increment(Duration::from_millis(10));

        // ----------------------------------------------------
        // Case 1
        //
        // 1.  0s: Insert with per-entry TTL 1s.
        // 2. +1s: Expires.
        // ----------------------------------------------------

        // Insert an entry (1). It will have a per-entry TTL of 1 second.
        let key = 1;
        let hash = cache.hash(&key);
        let value = 'a';

        *expectation.lock().unwrap() =
            ExpiryExpectation::after_create(line!(), key, value, current_time(&cache), Some(1));

        insert(&cache, key, hash, value);
        // Run a sync to register the entry to the internal data structures including
        // the timer wheel.
        cache.inner.run_pending_tasks(None, 1, 10);
        assert_eq!(cache.entry_count(), 1);

        assert_expiry!(cache, key, hash, mock, 1);

        // ----------------------------------------------------
        // Case 2
        //
        // 1.  0s: Insert with no per-entry TTL.
        // 2. +1s: Get with per-entry TTL 3s.
        // 3. +3s: Expires.
        // ----------------------------------------------------

        // Insert an entry (1).
        let key = 2;
        let hash = cache.hash(&key);
        let value = 'b';

        *expectation.lock().unwrap() =
            ExpiryExpectation::after_create(line!(), key, value, current_time(&cache), None);
        let inserted_at = current_time(&cache);
        insert(&cache, key, hash, value);
        cache.inner.run_pending_tasks(None, 1, 10);
        assert_eq!(cache.entry_count(), 1);

        // Increment the time.
        mock.increment(Duration::from_secs(1));
        cache.inner.run_pending_tasks(None, 1, 10);
        assert!(cache.contains_key_with_hash(&key, hash));

        // Read the entry (2).
        *expectation.lock().unwrap() = ExpiryExpectation::after_read(
            line!(),
            key,
            value,
            current_time(&cache),
            Some(TTI - 1),
            inserted_at,
            Some(3),
        );
        assert_eq!(
            cache
                .get_with_hash(&key, hash, false)
                .map(Entry::into_value),
            Some(value)
        );
        cache.inner.run_pending_tasks(None, 1, 10);

        assert_expiry!(cache, key, hash, mock, 3);

        // ----------------------------------------------------
        // Case 3
        //
        // 1.  0s: Insert with no per-entry TTL.
        // 2. +1s: Get with no per-entry TTL.
        // 3. +2s: Update with per-entry TTL 3s.
        // 4. +3s: Expires.
        // ----------------------------------------------------

        // Insert an entry (1).
        let key = 3;
        let hash = cache.hash(&key);
        let value = 'c';

        *expectation.lock().unwrap() =
            ExpiryExpectation::after_create(line!(), key, value, current_time(&cache), None);
        let inserted_at = current_time(&cache);
        insert(&cache, key, hash, value);
        cache.inner.run_pending_tasks(None, 1, 10);
        assert_eq!(cache.entry_count(), 1);

        // Increment the time.
        mock.increment(Duration::from_secs(1));
        cache.inner.run_pending_tasks(None, 1, 10);
        assert!(cache.contains_key_with_hash(&key, hash));

        // Read the entry (2).
        *expectation.lock().unwrap() = ExpiryExpectation::after_read(
            line!(),
            key,
            value,
            current_time(&cache),
            Some(TTI - 1),
            inserted_at,
            None,
        );
        assert_eq!(
            cache
                .get_with_hash(&key, hash, false)
                .map(Entry::into_value),
            Some(value)
        );
        cache.inner.run_pending_tasks(None, 1, 10);

        // Increment the time.
        mock.increment(Duration::from_secs(2));
        cache.inner.run_pending_tasks(None, 1, 10);
        assert!(cache.contains_key_with_hash(&key, hash));
        assert_eq!(cache.entry_count(), 1);

        // Update the entry (3).
        *expectation.lock().unwrap() = ExpiryExpectation::after_update(
            line!(),
            key,
            value,
            current_time(&cache),
            // TTI should be reset by this update.
            Some(TTI),
            Some(3),
        );
        insert(&cache, key, hash, value);
        cache.inner.run_pending_tasks(None, 1, 10);
        assert_eq!(cache.entry_count(), 1);

        assert_expiry!(cache, key, hash, mock, 3);

        // ----------------------------------------------------
        // Case 4
        //
        // 1.  0s: Insert with no per-entry TTL.
        // 2. +1s: Get with no per-entry TTL.
        // 3. +2s: Update with no per-entry TTL.
        // 4. +7s: Expires by TTI (7s from step 3).
        // ----------------------------------------------------

        // Insert an entry (1).
        let key = 4;
        let hash = cache.hash(&key);
        let value = 'd';

        *expectation.lock().unwrap() =
            ExpiryExpectation::after_create(line!(), key, value, current_time(&cache), None);
        let inserted_at = current_time(&cache);
        insert(&cache, key, hash, value);
        cache.inner.run_pending_tasks(None, 1, 10);
        assert_eq!(cache.entry_count(), 1);

        // Increment the time.
        mock.increment(Duration::from_secs(1));
        cache.inner.run_pending_tasks(None, 1, 10);
        assert!(cache.contains_key_with_hash(&key, hash));
        assert_eq!(cache.entry_count(), 1);

        // Read the entry (2).
        *expectation.lock().unwrap() = ExpiryExpectation::after_read(
            line!(),
            key,
            value,
            current_time(&cache),
            Some(TTI - 1),
            inserted_at,
            None,
        );
        assert_eq!(
            cache
                .get_with_hash(&key, hash, false)
                .map(Entry::into_value),
            Some(value)
        );
        cache.inner.run_pending_tasks(None, 1, 10);

        // Increment the time.
        mock.increment(Duration::from_secs(2));
        cache.inner.run_pending_tasks(None, 1, 10);
        assert!(cache.contains_key_with_hash(&key, hash));
        assert_eq!(cache.entry_count(), 1);

        // Update the entry (3).
        *expectation.lock().unwrap() = ExpiryExpectation::after_update(
            line!(),
            key,
            value,
            current_time(&cache),
            // TTI should be reset by this update.
            Some(TTI),
            None,
        );
        insert(&cache, key, hash, value);
        cache.inner.run_pending_tasks(None, 1, 10);
        assert_eq!(cache.entry_count(), 1);

        assert_expiry!(cache, key, hash, mock, 7);

        // ----------------------------------------------------
        // Case 5
        //
        // 1.  0s: Insert with per-entry TTL 8s.
        // 2. +5s: Get with per-entry TTL 8s.
        // 3. +7s: Expires by TTI (7s).
        // ----------------------------------------------------

        // Insert an entry.
        let key = 5;
        let hash = cache.hash(&key);
        let value = 'e';

        *expectation.lock().unwrap() =
            ExpiryExpectation::after_create(line!(), key, value, current_time(&cache), Some(8));
        let inserted_at = current_time(&cache);
        insert(&cache, key, hash, value);
        cache.inner.run_pending_tasks(None, 1, 10);
        assert_eq!(cache.entry_count(), 1);

        // Increment the time.
        mock.increment(Duration::from_secs(5));
        cache.inner.run_pending_tasks(None, 1, 10);
        assert!(cache.contains_key_with_hash(&key, hash));
        assert_eq!(cache.entry_count(), 1);

        // Read the entry.
        *expectation.lock().unwrap() = ExpiryExpectation::after_read(
            line!(),
            key,
            value,
            current_time(&cache),
            Some(TTI - 5),
            inserted_at,
            Some(8),
        );
        assert_eq!(
            cache
                .get_with_hash(&key, hash, false)
                .map(Entry::into_value),
            Some(value)
        );
        cache.inner.run_pending_tasks(None, 1, 10);

        assert_expiry!(cache, key, hash, mock, 7);

        // ----------------------------------------------------
        // Case 6
        //
        // 1.  0s: Insert with per-entry TTL 8s.
        // 2. +5s: Get with per-entry TTL 9s.
        // 3. +6s: Get with per-entry TTL 10s.
        // 4. +5s: Expires by TTL (16s).
        // ----------------------------------------------------

        // Insert an entry.
        let key = 6;
        let hash = cache.hash(&key);
        let value = 'f';

        *expectation.lock().unwrap() =
            ExpiryExpectation::after_create(line!(), key, value, current_time(&cache), Some(8));
        let inserted_at = current_time(&cache);
        insert(&cache, key, hash, value);
        cache.inner.run_pending_tasks(None, 1, 10);
        assert_eq!(cache.entry_count(), 1);

        // Increment the time.
        mock.increment(Duration::from_secs(5));
        cache.inner.run_pending_tasks(None, 1, 10);
        assert!(cache.contains_key_with_hash(&key, hash));
        assert_eq!(cache.entry_count(), 1);

        // Read the entry.
        *expectation.lock().unwrap() = ExpiryExpectation::after_read(
            line!(),
            key,
            value,
            current_time(&cache),
            Some(TTI - 5),
            inserted_at,
            Some(9),
        );
        assert_eq!(
            cache
                .get_with_hash(&key, hash, false)
                .map(Entry::into_value),
            Some(value)
        );
        cache.inner.run_pending_tasks(None, 1, 10);

        // Increment the time.
        mock.increment(Duration::from_secs(6));
        cache.inner.run_pending_tasks(None, 1, 10);
        assert!(cache.contains_key_with_hash(&key, hash));
        assert_eq!(cache.entry_count(), 1);

        // Read the entry.
        *expectation.lock().unwrap() = ExpiryExpectation::after_read(
            line!(),
            key,
            value,
            current_time(&cache),
            Some(TTI - 6),
            inserted_at,
            Some(10),
        );
        assert_eq!(
            cache
                .get_with_hash(&key, hash, false)
                .map(Entry::into_value),
            Some(value)
        );
        cache.inner.run_pending_tasks(None, 1, 10);

        assert_expiry!(cache, key, hash, mock, 5);

        // ----------------------------------------------------
        // Case 7
        //
        // 1.   0s: Insert with per-entry TTL 9s.
        // 2.  +6s: Update with per-entry TTL 8s.
        // 3.  +6s: Get with per-entry TTL 9s
        // 4.  +6s: Get with per-entry TTL 5s.
        // 5.  +4s: Expires by TTL (16s from step 2).
        // ----------------------------------------------------
        // Insert an entry.
        let key = 7;
        let hash = cache.hash(&key);
        let value = 'g';

        *expectation.lock().unwrap() =
            ExpiryExpectation::after_create(line!(), key, value, current_time(&cache), Some(9));
        insert(&cache, key, hash, value);
        cache.inner.run_pending_tasks(None, 1, 10);
        assert_eq!(cache.entry_count(), 1);

        // Increment the time.
        mock.increment(Duration::from_secs(6));
        cache.inner.run_pending_tasks(None, 1, 10);
        assert!(cache.contains_key_with_hash(&key, hash));
        assert_eq!(cache.entry_count(), 1);

        // Update the entry (3).
        *expectation.lock().unwrap() = ExpiryExpectation::after_update(
            line!(),
            key,
            value,
            current_time(&cache),
            // From the per-entry TTL.
            Some(9 - 6),
            Some(8),
        );
        let updated_at = current_time(&cache);
        insert(&cache, key, hash, value);
        cache.inner.run_pending_tasks(None, 1, 10);
        assert_eq!(cache.entry_count(), 1);

        // Increment the time.
        mock.increment(Duration::from_secs(6));
        cache.inner.run_pending_tasks(None, 1, 10);
        assert!(cache.contains_key_with_hash(&key, hash));
        assert_eq!(cache.entry_count(), 1);

        // Read the entry.
        *expectation.lock().unwrap() = ExpiryExpectation::after_read(
            line!(),
            key,
            value,
            current_time(&cache),
            Some(TTI - 6),
            updated_at,
            Some(9),
        );
        assert_eq!(
            cache
                .get_with_hash(&key, hash, false)
                .map(Entry::into_value),
            Some(value)
        );
        cache.inner.run_pending_tasks(None, 1, 10);

        // Increment the time.
        mock.increment(Duration::from_secs(6));
        cache.inner.run_pending_tasks(None, 1, 10);
        assert!(cache.contains_key_with_hash(&key, hash));
        assert_eq!(cache.entry_count(), 1);

        // Read the entry.
        *expectation.lock().unwrap() = ExpiryExpectation::after_read(
            line!(),
            key,
            value,
            current_time(&cache),
            Some(TTI - 6),
            updated_at,
            Some(5),
        );
        assert_eq!(
            cache
                .get_with_hash(&key, hash, false)
                .map(Entry::into_value),
            Some(value)
        );
        cache.inner.run_pending_tasks(None, 1, 10);

        assert_expiry!(cache, key, hash, mock, 4);
    }
}
