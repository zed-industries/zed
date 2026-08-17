// License and Copyright Notice:
//
// Some of the code and doc comments in this module were ported or copied from
// a Java class `com.github.benmanes.caffeine.cache.TimerWheel` of Caffeine.
// https://github.com/ben-manes/caffeine/blob/master/caffeine/src/main/java/com/github/benmanes/caffeine/cache/TimerWheel.java
//
// The original code/comments from Caffeine are licensed under the Apache License,
// Version 2.0 <https://github.com/ben-manes/caffeine/blob/master/LICENSE>
//
// Copyrights of the original code/comments are retained by their contributors.
// For full authorship information, see the version control history of
// https://github.com/ben-manes/caffeine/

use std::{ptr::NonNull, time::Duration};

use super::{
    concurrent::{arc::MiniArc, entry_info::EntryInfo, DeqNodes},
    deque::{DeqNode, Deque},
    time::Instant,
};

use parking_lot::Mutex;

const BUCKET_COUNTS: &[u64] = &[
    64, // roughly seconds
    64, // roughly minutes
    32, // roughly hours
    4,  // roughly days
    1,  // overflow (> ~6.5 days)
];

const OVERFLOW_QUEUE_INDEX: usize = BUCKET_COUNTS.len() - 1;
const NUM_LEVELS: usize = OVERFLOW_QUEUE_INDEX - 1;

const DAY: Duration = Duration::from_secs(60 * 60 * 24);

const SPANS: &[u64] = &[
    aligned_duration(Duration::from_secs(1)),       // 1.07s
    aligned_duration(Duration::from_secs(60)),      // 1.14m
    aligned_duration(Duration::from_secs(60 * 60)), // 1.22h
    aligned_duration(DAY),                          // 1.63d
    BUCKET_COUNTS[3] * aligned_duration(DAY),       // 6.5d
    BUCKET_COUNTS[3] * aligned_duration(DAY),       // 6.5d
];

const SHIFT: &[u64] = &[
    SPANS[0].trailing_zeros() as u64,
    SPANS[1].trailing_zeros() as u64,
    SPANS[2].trailing_zeros() as u64,
    SPANS[3].trailing_zeros() as u64,
    SPANS[4].trailing_zeros() as u64,
];

/// Returns the next power of two of the duration in nanoseconds.
const fn aligned_duration(duration: Duration) -> u64 {
    // NOTE: as_nanos() returns u128, so convert it to u64 by using `as`.
    // We cannot call TryInto::try_into() here because it is not a const fn.
    (duration.as_nanos() as u64).next_power_of_two()
}

/// A timer node stored in a bucket of a timer wheel.
pub(crate) enum TimerNode<K> {
    /// A sentinel node that is used to mark the end of a timer wheel bucket.
    Sentinel,
    /// A timer entry that is holding Arc pointers to the data structures in a cache
    /// entry.
    Entry {
        /// The position (level and index) of the timer wheel bucket.
        pos: Option<(u8, u8)>,
        /// An Arc pointer to the `EntryInfo` of the cache entry (`ValueEntry`).
        entry_info: MiniArc<EntryInfo<K>>,
        /// An Arc pointer to the `DeqNodes` of the cache entry (`ValueEntry`).
        deq_nodes: MiniArc<Mutex<DeqNodes<K>>>,
    },
}

impl<K> TimerNode<K> {
    fn new(
        entry_info: MiniArc<EntryInfo<K>>,
        deq_nodes: MiniArc<Mutex<DeqNodes<K>>>,
        level: usize,
        index: usize,
    ) -> Self {
        Self::Entry {
            pos: Some((level as u8, index as u8)),
            entry_info,
            deq_nodes,
        }
    }

    /// Returns the position (level and index) of the timer wheel bucket.
    fn position(&self) -> Option<(usize, usize)> {
        if let Self::Entry { pos, .. } = &self {
            pos.map(|(level, index)| (level as usize, index as usize))
        } else {
            unreachable!()
        }
    }

    fn set_position(&mut self, level: usize, index: usize) {
        if let Self::Entry { pos, .. } = self {
            *pos = Some((level as u8, index as u8));
        } else {
            unreachable!()
        }
    }

    fn unset_position(&mut self) {
        if let Self::Entry { pos, .. } = self {
            *pos = None;
        } else {
            unreachable!()
        }
    }

    fn is_sentinel(&self) -> bool {
        matches!(self, Self::Sentinel)
    }

    pub(crate) fn entry_info(&self) -> &MiniArc<EntryInfo<K>> {
        if let Self::Entry { entry_info, .. } = &self {
            entry_info
        } else {
            unreachable!()
        }
    }

    fn unset_timer_node_in_deq_nodes(&self) {
        if let Self::Entry { deq_nodes, .. } = &self {
            deq_nodes.lock().set_timer_node(None);
        } else {
            unreachable!();
        }
    }
}

type Bucket<K> = Deque<TimerNode<K>>;

#[must_use = "this `ReschedulingResult` may be an `Removed` variant, which should be handled"]
pub(crate) enum ReschedulingResult<K> {
    /// The timer event was rescheduled.
    Rescheduled,
    /// The timer event was not rescheduled because the entry has no expiration time.
    Removed(Box<DeqNode<TimerNode<K>>>),
}

/// A hierarchical timer wheel to add, remove, and fire expiration events in
/// amortized O(1) time.
///
/// The expiration events are deferred until the timer is advanced, which is
/// performed as part of the cache's housekeeping cycle.
pub(crate) struct TimerWheel<K> {
    /// The hierarchical timer wheels.
    wheels: Box<[Box<[Bucket<K>]>]>,
    /// The time when this `TimerWheel` was created.
    origin: Instant,
    /// The time when this `TimerWheel` was last advanced.
    current: Instant,
}

#[cfg(feature = "future")]
// TODO: https://github.com/moka-rs/moka/issues/54
#[allow(clippy::non_send_fields_in_send_ty)]
// Multi-threaded async runtimes require base_cache::Inner to be Send, but it will
// not be without this `unsafe impl`. This is because DeqNodes have NonNull
// pointers.
unsafe impl<K> Send for TimerWheel<K> {}

impl<K> TimerWheel<K> {
    pub(crate) fn new(now: Instant) -> Self {
        Self {
            wheels: Box::default(), // Empty.
            origin: now,
            current: now,
        }
    }

    pub(crate) fn is_enabled(&self) -> bool {
        !self.wheels.is_empty()
    }

    pub(crate) fn enable(&mut self) {
        assert!(!self.is_enabled());

        // Populate each bucket with a queue having a sentinel node.
        self.wheels = BUCKET_COUNTS
            .iter()
            .map(|b| {
                (0..*b)
                    .map(|_| {
                        let mut deq = Deque::new(super::CacheRegion::Other);
                        deq.push_back(Box::new(DeqNode::new(TimerNode::Sentinel)));
                        deq
                    })
                    .collect::<Vec<_>>()
                    .into_boxed_slice()
            })
            .collect::<Vec<_>>()
            .into_boxed_slice();
    }

    /// Schedules a timer event for the node.
    pub(crate) fn schedule(
        &mut self,
        entry_info: MiniArc<EntryInfo<K>>,
        deq_nodes: MiniArc<Mutex<DeqNodes<K>>>,
    ) -> Option<NonNull<DeqNode<TimerNode<K>>>> {
        debug_assert!(self.is_enabled());

        if let Some(t) = entry_info.expiration_time() {
            let (level, index) = self.bucket_indices(t);
            let node = Box::new(DeqNode::new(TimerNode::new(
                entry_info, deq_nodes, level, index,
            )));
            let node = self.wheels[level][index].push_back(node);
            Some(node)
        } else {
            None
        }
    }

    fn schedule_existing_node(
        &mut self,
        mut node: NonNull<DeqNode<TimerNode<K>>>,
    ) -> ReschedulingResult<K> {
        debug_assert!(self.is_enabled());

        // Since cache entry's ValueEntry has a pointer to this node, we must reuse
        // the node.
        //
        // SAFETY on `node.as_mut()`: The self (`TimerWheel`) is the only owner of
        // the node, and we have `&mut self` here. We are the only one who can mutate
        // the node.
        if let entry @ TimerNode::Entry { .. } = &mut unsafe { node.as_mut() }.element {
            if let Some(t) = entry.entry_info().expiration_time() {
                let (level, index) = self.bucket_indices(t);
                entry.set_position(level, index);
                let node = unsafe { Box::from_raw(node.as_ptr()) };
                self.wheels[level][index].push_back(node);
                ReschedulingResult::Rescheduled
            } else {
                entry.unset_position();
                entry.unset_timer_node_in_deq_nodes();
                ReschedulingResult::Removed(unsafe { Box::from_raw(node.as_ptr()) })
            }
        } else {
            unreachable!()
        }
    }

    /// Reschedules an active timer event for the node.
    pub(crate) fn reschedule(
        &mut self,
        node: NonNull<DeqNode<TimerNode<K>>>,
    ) -> ReschedulingResult<K> {
        debug_assert!(self.is_enabled());
        unsafe { self.unlink_timer(node) };
        self.schedule_existing_node(node)
    }

    /// Removes a timer event for this node if present.
    pub(crate) fn deschedule(&mut self, node: NonNull<DeqNode<TimerNode<K>>>) {
        debug_assert!(self.is_enabled());
        unsafe {
            self.unlink_timer(node);
            Self::drop_node(node);
        }
    }

    /// Removes a timer event for this node if present.
    ///
    /// IMPORTANT: This method does not drop the node.
    unsafe fn unlink_timer(&mut self, mut node: NonNull<DeqNode<TimerNode<K>>>) {
        // SAFETY: The self (`TimerWheel`) is the only owner of the node, and we have
        // `&mut self` here. We are the only one who can mutate the node.
        let p = node.as_mut();
        if let entry @ TimerNode::Entry { .. } = &mut p.element {
            if let Some((level, index)) = entry.position() {
                self.wheels[level][index].unlink(node);
                entry.unset_position();
            }
        } else {
            unreachable!();
        }
    }

    unsafe fn drop_node(node: NonNull<DeqNode<TimerNode<K>>>) {
        std::mem::drop(Box::from_raw(node.as_ptr()));
    }

    /// Advances the timer wheel to the current time, and returns an iterator over
    /// timer events.
    pub(crate) fn advance(
        &mut self,
        current_time: Instant,
    ) -> impl Iterator<Item = TimerEvent<K>> + '_ {
        debug_assert!(self.is_enabled());

        let previous_time = self.current;
        self.current = current_time;
        TimerEventsIter::new(self, previous_time, current_time)
    }

    /// Returns a pointer to the timer event (cache entry) at the front of the queue.
    /// Returns `None` if the front node is a sentinel.
    fn pop_timer_node(&mut self, level: usize, index: usize) -> Option<Box<DeqNode<TimerNode<K>>>> {
        let deque = &mut self.wheels[level][index];
        if let Some(node) = deque.peek_front() {
            if node.element.is_sentinel() {
                return None;
            }
        }

        deque.pop_front()
    }

    /// Reset the positions of the nodes in the queue at the given level and index.
    /// When done, the sentinel is at the back of the queue.
    fn reset_timer_node_positions(&mut self, level: usize, index: usize) {
        let deque = &mut self.wheels[level][index];
        debug_assert!(
            deque.len() > 0,
            "BUG: The queue is empty. level: {level}, index: {index}"
        );

        // Rotate the nodes in the queue until we see the sentinel at the back of the
        // queue.
        while !deque.peek_back().unwrap().element.is_sentinel() {
            deque.move_front_to_back();
        }
    }

    /// Returns the bucket indices to locate the bucket that the timer event
    /// should be added to.
    fn bucket_indices(&self, time: Instant) -> (usize, usize) {
        let duration_nanos = self.duration_nanos_since_last_advanced(time);
        let time_nanos = self.time_nanos(time);
        for level in 0..=NUM_LEVELS {
            if duration_nanos < SPANS[level + 1] {
                let ticks = time_nanos >> SHIFT[level];
                let index = ticks & (BUCKET_COUNTS[level] - 1);
                return (level, index as usize);
            }
        }
        (OVERFLOW_QUEUE_INDEX, 0)
    }

    // Returns nano-seconds between the given `time` and the time when this timer
    // wheel was advanced. If the `time` is earlier than other, returns zero.
    fn duration_nanos_since_last_advanced(&self, time: Instant) -> u64 {
        // If `time` is earlier than `self.current`, use zero. This could happen
        // when a user provided `Expiry` method returned zero or a very short
        // duration.
        time.saturating_duration_since(self.current).as_nanos() as u64
    }

    // Returns nano-seconds between the given `time` and `self.origin`, the time when
    // this timer wheel was created.
    //
    // - If the `time` is earlier than other, returns zero.
    // - If the `time` is later than `self.origin + u64::MAX`, returns `u64::MAX`,
    //   which is ~584 years in nanoseconds.
    //
    fn time_nanos(&self, time: Instant) -> u64 {
        let nanos_u128 = time
            // If `time` is earlier than `self.origin`, use zero. This would never
            // happen in practice as there should be some delay between the timer
            // wheel was created and the first timer event is scheduled. But we will
            // do this just in case.
            .saturating_duration_since(self.origin)
            .as_nanos();

        // Convert an `u128` into an `u64`. If the value is too large, use `u64::MAX`
        // (~584 years)
        nanos_u128.try_into().unwrap_or(u64::MAX)
    }
}

/// A timer event, which is either an expired/rescheduled cache entry, or a
/// descheduled timer. `TimerWheel::advance` method returns an iterator over timer
/// events.
#[derive(Debug)]
pub(crate) enum TimerEvent<K> {
    /// This cache entry has expired.
    Expired(Box<DeqNode<TimerNode<K>>>),
    // This cache entry has been rescheduled. Rescheduling includes moving a timer
    // from one wheel to another in a lower level of the hierarchy. (This variant
    // is mainly used for testing)
    #[cfg(test)]
    Rescheduled(MiniArc<EntryInfo<K>>),
    #[cfg(not(test))]
    Rescheduled(()),
    /// This timer node (containing a cache entry) has been removed from the timer.
    /// (This variant is mainly used for testing)
    Descheduled,
}

/// An iterator over expired cache entries.
pub(crate) struct TimerEventsIter<'iter, K> {
    timer_wheel: &'iter mut TimerWheel<K>,
    previous_time: Instant,
    current_time: Instant,
    is_done: bool,
    level: usize,
    index: u8,
    end_index: u8,
    index_mask: u64,
    is_new_level: bool,
    is_new_index: bool,
}

impl<'iter, K> TimerEventsIter<'iter, K> {
    fn new(
        timer_wheel: &'iter mut TimerWheel<K>,
        previous_time: Instant,
        current_time: Instant,
    ) -> Self {
        Self {
            timer_wheel,
            previous_time,
            current_time,
            is_done: false,
            level: 0,
            index: 0,
            end_index: 0,
            index_mask: 0,
            is_new_level: true,
            is_new_index: true,
        }
    }
}

impl<K> Drop for TimerEventsIter<'_, K> {
    fn drop(&mut self) {
        if !self.is_done {
            // This iterator was dropped before consuming all events. Reset the
            // `current` to the time when the timer wheel was last successfully
            // advanced.
            self.timer_wheel.current = self.previous_time;
        }
    }
}

impl<K> Iterator for TimerEventsIter<'_, K> {
    type Item = TimerEvent<K>;

    /// NOTE: When necessary, this iterator will unset the timer node pointer in the
    /// `ValueEntry`.
    fn next(&mut self) -> Option<Self::Item> {
        if self.is_done {
            return None;
        }

        loop {
            if self.is_new_level {
                let previous_time_nanos = self.timer_wheel.time_nanos(self.previous_time);
                let current_time_nanos = self.timer_wheel.time_nanos(self.current_time);
                let previous_ticks = previous_time_nanos >> SHIFT[self.level];
                let current_ticks = current_time_nanos >> SHIFT[self.level];

                if current_ticks <= previous_ticks {
                    self.is_done = true;
                    return None;
                }

                self.index_mask = BUCKET_COUNTS[self.level] - 1;
                self.index = (previous_ticks & self.index_mask) as u8;
                let steps =
                    (current_ticks - previous_ticks + 1).min(BUCKET_COUNTS[self.level]) as u8;
                self.end_index = self.index + steps;

                self.is_new_level = false;
                self.is_new_index = true;

                // dbg!(self.level, self.index, self.end_index);
            }

            let i = self.index & self.index_mask as u8;

            if self.is_new_index {
                // Move the sentinel to the back of the queue.
                self.timer_wheel
                    .reset_timer_node_positions(self.level, i as usize);

                self.is_new_index = false;
            }

            // Pop the next timer event (cache entry) from the queue at the current
            // level and index.
            //
            // We will repeat processing this level until we see the sentinel.
            // (`pop_timer_node` will return `None` when it sees the sentinel)
            if let Some(node) = self.timer_wheel.pop_timer_node(self.level, i as usize) {
                let expiration_time = node.as_ref().element.entry_info().expiration_time();
                if let Some(t) = expiration_time {
                    if t <= self.current_time {
                        // The cache entry has expired. Unset the timer node from
                        // the ValueEntry and return the node.
                        node.as_ref().element.unset_timer_node_in_deq_nodes();
                        return Some(TimerEvent::Expired(node));
                    }

                    // The cache entry has not expired. Reschedule it.
                    let node_p = NonNull::new(Box::into_raw(node)).expect("Got a null ptr");

                    #[cfg(test)]
                    // Get the entry info before rescheduling (mutating) the node to
                    // avoid Stacked Borrows/Tree Borrows violations on `node_p`.
                    let entry_info =
                        MiniArc::clone(unsafe { node_p.as_ref() }.element.entry_info());

                    match self.timer_wheel.schedule_existing_node(node_p) {
                        ReschedulingResult::Rescheduled => {
                            #[cfg(test)]
                            return Some(TimerEvent::Rescheduled(entry_info));
                            #[cfg(not(test))]
                            return Some(TimerEvent::Rescheduled(()));
                        }
                        ReschedulingResult::Removed(node) => {
                            // The timer event has been removed from the timer
                            // wheel. Unset the timer node from the ValueEntry.
                            node.as_ref().element.unset_timer_node_in_deq_nodes();
                            return Some(TimerEvent::Descheduled);
                        }
                    }
                }
            } else {
                // Done with the current queue. Move to the next index
                // and/or next level.
                self.index += 1;
                self.is_new_index = true;

                if self.index >= self.end_index {
                    self.level += 1;
                    // No more levels to process. We are done.
                    if self.level >= BUCKET_COUNTS.len() {
                        self.is_done = true;
                        return None;
                    }
                    self.is_new_level = true;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use std::{sync::Arc, time::Duration};

    use super::{TimerEvent, TimerWheel, SPANS};
    use crate::common::{
        concurrent::{arc::MiniArc, entry_info::EntryInfo, KeyHash},
        time::{Clock, Instant, Mock},
    };

    #[test]
    fn test_bucket_indices() {
        fn bi(timer: &TimerWheel<()>, now: Instant, dur: Duration) -> (usize, usize) {
            let t = now.saturating_add(dur);
            timer.bucket_indices(t)
        }

        let (clock, mock) = Clock::mock();
        let now = clock.now();

        let mut timer = TimerWheel::<()>::new(now);
        timer.enable();

        assert_eq!(timer.bucket_indices(now), (0, 0));

        // Level 0: 1.07s
        assert_eq!(bi(&timer, now, n2d(SPANS[0] - 1)), (0, 0));
        assert_eq!(bi(&timer, now, n2d(SPANS[0])), (0, 1));
        assert_eq!(bi(&timer, now, n2d(SPANS[0] * 63)), (0, 63));

        // Level 1: 1.14m
        assert_eq!(bi(&timer, now, n2d(SPANS[0] * 64)), (1, 1));
        assert_eq!(bi(&timer, now, n2d(SPANS[1])), (1, 1));
        assert_eq!(bi(&timer, now, n2d(SPANS[1] * 63 + SPANS[0] * 63)), (1, 63));

        // Level 2: 1.22h
        assert_eq!(bi(&timer, now, n2d(SPANS[1] * 64)), (2, 1));
        assert_eq!(bi(&timer, now, n2d(SPANS[2])), (2, 1));
        assert_eq!(
            bi(
                &timer,
                now,
                n2d(SPANS[2] * 31 + SPANS[1] * 63 + SPANS[0] * 63)
            ),
            (2, 31)
        );

        // Level 3: 1.63dh
        assert_eq!(bi(&timer, now, n2d(SPANS[2] * 32)), (3, 1));
        assert_eq!(bi(&timer, now, n2d(SPANS[3])), (3, 1));
        assert_eq!(bi(&timer, now, n2d(SPANS[3] * 3)), (3, 3));

        // Overflow
        assert_eq!(bi(&timer, now, n2d(SPANS[3] * 4)), (4, 0));
        assert_eq!(bi(&timer, now, n2d(SPANS[4])), (4, 0));
        assert_eq!(bi(&timer, now, n2d(SPANS[4] * 100)), (4, 0));

        // Increment the clock by 5 ticks. (1 tick ~= 1.07s)
        let now = advance_clock(&clock, &mock, n2d(SPANS[0] * 5));
        timer.current = now;

        // Level 0: 1.07s
        assert_eq!(bi(&timer, now, n2d(SPANS[0] - 1)), (0, 5));
        assert_eq!(bi(&timer, now, n2d(SPANS[0])), (0, 6));
        assert_eq!(bi(&timer, now, n2d(SPANS[0] * 63)), (0, 4));

        // Level 1: 1.14m
        assert_eq!(bi(&timer, now, n2d(SPANS[0] * 64)), (1, 1));
        assert_eq!(bi(&timer, now, n2d(SPANS[1])), (1, 1));
        assert_eq!(
            bi(&timer, now, n2d(SPANS[1] * 63 + SPANS[0] * (63 - 5))),
            (1, 63)
        );

        // Increment the clock by 61 ticks. (total 66 ticks)
        let now = advance_clock(&clock, &mock, n2d(SPANS[0] * 61));
        timer.current = now;

        // Level 0: 1.07s
        assert_eq!(bi(&timer, now, n2d(SPANS[0] - 1)), (0, 2));
        assert_eq!(bi(&timer, now, n2d(SPANS[0])), (0, 3));
        assert_eq!(bi(&timer, now, n2d(SPANS[0] * 63)), (0, 1));

        // Level 1: 1.14m
        assert_eq!(bi(&timer, now, n2d(SPANS[0] * 64)), (1, 2));
        assert_eq!(bi(&timer, now, n2d(SPANS[1])), (1, 2));
        assert_eq!(
            bi(&timer, now, n2d(SPANS[1] * 63 + SPANS[0] * (63 - 2))),
            (1, 0)
        );
    }

    #[test]
    fn test_advance() {
        fn schedule_timer(timer: &mut TimerWheel<u32>, key: u32, now: Instant, ttl: Duration) {
            let hash = key as u64;
            let key_hash = KeyHash::new(Arc::new(key), hash);
            let policy_weight = 0;
            let entry_info = MiniArc::new(EntryInfo::new(key_hash, now, policy_weight));
            entry_info.set_expiration_time(Some(now.saturating_add(ttl)));
            let deq_nodes = Default::default();
            let timer_node = timer.schedule(entry_info, MiniArc::clone(&deq_nodes));
            deq_nodes.lock().set_timer_node(timer_node);
        }

        fn expired_key(maybe_entry: Option<TimerEvent<u32>>) -> u32 {
            let entry = maybe_entry.expect("entry is none");
            match entry {
                TimerEvent::Expired(node) => *node.element.entry_info().key_hash().key,
                _ => panic!("Expected an expired entry. Got {entry:?}"),
            }
        }

        fn rescheduled_key(maybe_entry: Option<TimerEvent<u32>>) -> u32 {
            let entry = maybe_entry.expect("entry is none");
            match entry {
                TimerEvent::Rescheduled(entry) => *entry.key_hash().key,
                _ => panic!("Expected a rescheduled entry. Got {entry:?}"),
            }
        }

        let (clock, mock) = Clock::mock();
        let now = advance_clock(&clock, &mock, s2d(10));

        let mut timer = TimerWheel::<u32>::new(now);
        timer.enable();

        // Add timers that will expire in some seconds.
        schedule_timer(&mut timer, 1, now, s2d(5));
        schedule_timer(&mut timer, 2, now, s2d(1));
        schedule_timer(&mut timer, 3, now, s2d(63));
        schedule_timer(&mut timer, 4, now, s2d(3));

        let now = advance_clock(&clock, &mock, s2d(4));
        let mut expired_entries = timer.advance(now);
        assert_eq!(expired_key(expired_entries.next()), 2);
        assert_eq!(expired_key(expired_entries.next()), 4);
        assert!(expired_entries.next().is_none());
        drop(expired_entries);

        let now = advance_clock(&clock, &mock, s2d(4));
        let mut expired_entries = timer.advance(now);
        assert_eq!(expired_key(expired_entries.next()), 1);
        assert!(expired_entries.next().is_none());
        drop(expired_entries);

        let now = advance_clock(&clock, &mock, s2d(64 - 8));
        let mut expired_entries = timer.advance(now);
        assert_eq!(expired_key(expired_entries.next()), 3);
        assert!(expired_entries.next().is_none());
        drop(expired_entries);

        // Add timers that will expire in some minutes.
        const MINUTES: u64 = 60;
        schedule_timer(&mut timer, 1, now, s2d(5 * MINUTES));
        #[allow(clippy::identity_op)]
        schedule_timer(&mut timer, 2, now, s2d(1 * MINUTES));
        schedule_timer(&mut timer, 3, now, s2d(63 * MINUTES));
        schedule_timer(&mut timer, 4, now, s2d(3 * MINUTES));

        let now = advance_clock(&clock, &mock, s2d(4 * MINUTES));
        let mut expired_entries = timer.advance(now);
        assert_eq!(expired_key(expired_entries.next()), 2);
        assert_eq!(expired_key(expired_entries.next()), 4);
        assert!(expired_entries.next().is_none());
        drop(expired_entries);

        let now = advance_clock(&clock, &mock, s2d(4 * MINUTES));
        let mut expired_entries = timer.advance(now);
        assert_eq!(expired_key(expired_entries.next()), 1);
        assert!(expired_entries.next().is_none());
        drop(expired_entries);

        let now = advance_clock(&clock, &mock, s2d((64 - 8) * MINUTES));
        let mut expired_entries = timer.advance(now);
        assert_eq!(expired_key(expired_entries.next()), 3);
        assert!(expired_entries.next().is_none());
        drop(expired_entries);

        // Add timers that will expire in some hours.
        const HOURS: u64 = 60 * 60;
        schedule_timer(&mut timer, 1, now, s2d(5 * HOURS));
        #[allow(clippy::identity_op)]
        schedule_timer(&mut timer, 2, now, s2d(1 * HOURS));
        schedule_timer(&mut timer, 3, now, s2d(31 * HOURS));
        schedule_timer(&mut timer, 4, now, s2d(3 * HOURS));

        let now = advance_clock(&clock, &mock, s2d(4 * HOURS));
        let mut expired_entries = timer.advance(now);
        assert_eq!(expired_key(expired_entries.next()), 2);
        assert_eq!(expired_key(expired_entries.next()), 4);
        assert_eq!(rescheduled_key(expired_entries.next()), 1);
        assert!(expired_entries.next().is_none());
        drop(expired_entries);

        let now = advance_clock(&clock, &mock, s2d(4 * HOURS));
        let mut expired_entries = timer.advance(now);
        assert_eq!(expired_key(expired_entries.next()), 1);
        assert!(expired_entries.next().is_none());
        drop(expired_entries);

        let now = advance_clock(&clock, &mock, s2d((32 - 8) * HOURS));
        let mut expired_entries = timer.advance(now);
        assert_eq!(expired_key(expired_entries.next()), 3);
        assert!(expired_entries.next().is_none());
        drop(expired_entries);

        // Add timers that will expire in a few days.
        const DAYS: u64 = 24 * 60 * 60;
        schedule_timer(&mut timer, 1, now, s2d(5 * DAYS));
        #[allow(clippy::identity_op)]
        schedule_timer(&mut timer, 2, now, s2d(1 * DAYS));
        schedule_timer(&mut timer, 3, now, s2d(2 * DAYS));
        // Longer than ~6.5 days, so this should be stored in the overflow area.
        schedule_timer(&mut timer, 4, now, s2d(8 * DAYS));

        let now = advance_clock(&clock, &mock, s2d(3 * DAYS));
        let mut expired_entries = timer.advance(now);
        assert_eq!(expired_key(expired_entries.next()), 2);
        assert_eq!(expired_key(expired_entries.next()), 3);
        assert!(expired_entries.next().is_none());
        drop(expired_entries);

        let now = advance_clock(&clock, &mock, s2d(3 * DAYS));
        let mut expired_entries = timer.advance(now);
        assert_eq!(expired_key(expired_entries.next()), 1);
        assert_eq!(rescheduled_key(expired_entries.next()), 4);
        assert!(expired_entries.next().is_none());
        drop(expired_entries);

        let now = advance_clock(&clock, &mock, s2d(3 * DAYS));
        let mut expired_entries = timer.advance(now);
        assert_eq!(expired_key(expired_entries.next()), 4);
        assert!(expired_entries.next().is_none());
        drop(expired_entries);
    }

    //
    // Utility functions
    //

    fn advance_clock(clock: &Clock, mock: &Arc<Mock>, duration: Duration) -> Instant {
        mock.increment(duration);
        clock.now()
    }

    /// Convert nano-seconds to duration.
    fn n2d(nanos: u64) -> Duration {
        Duration::from_nanos(nanos)
    }

    /// Convert seconds to duration.
    fn s2d(secs: u64) -> Duration {
        Duration::from_secs(secs)
    }
}
