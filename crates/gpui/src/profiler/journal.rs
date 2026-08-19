//! A journal of foreground work and the semantic boundaries between activity intervals.
//!
//! The foreground thread records task polls, action handlers, input dispatches,
//! draws, and presentations as [`ForegroundEvent`]s in a bounded journal ring.
//! Task polls shorter than [`TASK_POLL_FLOOR`] are folded into summaries, which
//! bound the stream by the number of slow polls while preserving their exact
//! count and total duration.
//!
//! Presentation and the foreground going idle are explicit [`IntervalBoundary`]
//! entries in the same stream. Independent [`ForegroundJournalCollector`]s
//! feed entries to [`IntervalSealer`], a pure state machine that groups all work
//! preceding each boundary into a [`FrameSnapshot`]. The sealer does not infer
//! boundaries from elapsed time or from incidental event kinds such as draws.

use std::cell::{RefCell, UnsafeCell};
use std::collections::{HashMap, VecDeque};
use std::mem::MaybeUninit;
use std::sync::{
    Arc,
    atomic::{AtomicU64, AtomicUsize, Ordering},
};
use std::time::Duration;

use scheduler::Instant;

use super::{ActionTiming, FrameTiming, PresentTiming, TaskTiming};
use crate::WindowId;

/// Task polls shorter than this are folded into a [`PollSummary`] instead of
/// being recorded individually. This keeps the stream bounded by the number
/// of *slow* polls while preserving their exact count and total duration.
pub const TASK_POLL_FLOOR: Duration = Duration::from_micros(100);

/// A dirty frame that has not been presented by this deadline stops blocking
/// foreground-idle boundaries, so a window that never presents (hidden or
/// occluded windows receive no frame callbacks) cannot suppress boundaries
/// indefinitely. Expiry only unblocks: the open interval still seals at a
/// real presentation or idle boundary, keeping a starving hang and the frame
/// it starved in one interval.
pub const FRAME_DEADLINE: Duration = Duration::from_secs(1);

// Backstop against pathological event storms within a single interval. At the
// 100us floor, a fully hung second can produce at most ~10k recordable polls,
// so this bound is only reachable when something is already deeply wrong.
const MAX_INTERVAL_EVENTS: usize = 16 * 1024;

// Allow 4MiB for the fixed ring allocation, including each slot's atomic
// metadata. The poll floor and frame cadence bound the event rate to roughly
// 10k per second in the worst case, so this holds several seconds of
// worst-case traffic between consumer drains.
const MAX_JOURNAL_ENTRIES: usize = (4 * 1024 * 1024) / core::mem::size_of::<JournalSlot>();

// Absorbs brief collisions with a collector reading the exact slot being
// wrapped. The foreground never waits for a reader; queued entries are retried
// in order on the next publication.
const MAX_PENDING_JOURNAL_ENTRIES: usize = 64;

/// One entry in the foreground stream.
///
/// Events are recorded in completion order (ordered by their end time). An
/// event that was in progress across a frame boundary (e.g. the task poll
/// enclosing a draw) is recorded when it *ends*, with timestamps that may
/// precede events recorded before it.
#[derive(Debug, Copy, Clone)]
pub enum ForegroundEvent {
    /// A foreground task poll at least [`TASK_POLL_FLOOR`] long.
    TaskPoll(TaskTiming),
    /// A completed action handler.
    Action(ActionTiming),
    /// A dispatched platform input event.
    Input(InputTiming),
    /// A completed window draw.
    Draw(FrameTiming),
    /// Work spent submitting a frame to the platform.
    Present(PresentTiming),
    /// Aggregate of task polls below [`TASK_POLL_FLOOR`], flushed before the
    /// next individually retained event or interval boundary.
    SmallPolls(SmallPollFlush),
}

impl ForegroundEvent {
    /// When the work described by this event began. For draws this is the
    /// start of the draw itself, not the frame's first invalidation.
    pub fn start_time(&self) -> Instant {
        match self {
            Self::TaskPoll(timing) => timing.start,
            Self::Action(timing) => timing.start,
            Self::Input(timing) => timing.start,
            Self::Draw(timing) => timing.draw_start,
            Self::Present(timing) => timing.present_start,
            Self::SmallPolls(flush) => flush.since,
        }
    }

    /// When the work described by this event ended.
    pub fn end_time(&self) -> Instant {
        match self {
            Self::TaskPoll(timing) => timing.end.0,
            Self::Action(timing) => timing.end,
            Self::Input(timing) => timing.end,
            Self::Draw(timing) => timing.draw_end,
            Self::Present(timing) => timing.present_end,
            Self::SmallPolls(flush) => flush.until,
        }
    }

    /// How long the work described by this event took. For
    /// [`Self::SmallPolls`] this is the span the summary covers, not time
    /// spent polling; use the summary's `total` for occupancy.
    pub fn duration(&self) -> Duration {
        self.end_time().duration_since(self.start_time())
    }
}

/// Timing of one platform input dispatch on a window.
#[derive(Debug, Copy, Clone)]
pub struct InputTiming {
    /// The platform input variant dispatched (see
    /// [`crate::PlatformInput::kind_name`]).
    pub kind: &'static str,
    /// When the input dispatch started.
    pub start: Instant,
    /// When the input dispatch finished.
    pub end: Instant,
    /// Whether handling the input invalidated a window.
    pub caused_invalidation: bool,
}

/// Exact count and total duration of task polls below [`TASK_POLL_FLOOR`].
#[derive(Debug, Copy, Clone, Default, PartialEq, Eq)]
pub struct PollSummary {
    /// Number of polls below the floor.
    pub count: u64,
    /// Total duration of polls below the floor.
    pub total: Duration,
}

impl PollSummary {
    fn add(&mut self, other: Self) {
        self.count += other.count;
        self.total += other.total;
    }
}

/// A flushed [`PollSummary`] and the tightest span containing its folded polls.
#[derive(Debug, Copy, Clone)]
pub struct SmallPollFlush {
    /// The folded polls.
    pub summary: PollSummary,
    /// When the first folded poll began.
    pub since: Instant,
    /// When the last folded poll ended.
    pub until: Instant,
}

/// A newly drawn frame and the platform submission that completed its interval.
#[derive(Debug, Copy, Clone)]
pub struct PresentedFrame {
    /// The draw whose rendered scene was submitted.
    pub frame: FrameTiming,
    /// Work spent submitting that scene to the platform.
    pub presentation: PresentTiming,
}

impl PresentedFrame {
    /// Time from the frame's first invalidation through platform submission.
    pub fn dirty_to_present_duration(&self) -> Option<Duration> {
        self.frame
            .dirty_at
            .map(|dirty_at| self.presentation.present_end.duration_since(dirty_at))
    }
}

/// The semantic event that completed a foreground activity interval.
#[derive(Debug, Copy, Clone)]
pub enum IntervalBoundary {
    /// A newly drawn frame was submitted to the platform.
    Presented(PresentedFrame),
    /// The foreground returned to an idle platform loop with no unexpired
    /// frame pending.
    Idle {
        /// When the foreground went idle.
        ended_at: Instant,
    },
}

impl IntervalBoundary {
    /// When the interval ended.
    pub fn end_time(&self) -> Instant {
        match self {
            Self::Presented(presented) => presented.presentation.present_end,
            Self::Idle { ended_at } => *ended_at,
        }
    }

    /// The first invalidation of the frame satisfied by this boundary, if any.
    pub fn dirty_at(&self) -> Option<Instant> {
        match self {
            Self::Presented(presented) => presented.frame.dirty_at,
            Self::Idle { .. } => None,
        }
    }
}

/// A control-plane change to one window's pending-frame state.
#[derive(Debug, Copy, Clone)]
pub enum FrameStateChange {
    /// The window has a frame waiting to be presented.
    Pending {
        /// The window waiting for presentation.
        window_id: WindowId,
        /// When this frame generation first became dirty.
        dirty_at: Instant,
    },
    /// The window closed, so any pending frame no longer blocks idle
    /// boundaries.
    Closed {
        /// The window that closed.
        window_id: WindowId,
        /// When the window closed.
        at: Instant,
    },
}

/// One item retained in the foreground journal.
#[derive(Debug, Copy, Clone)]
pub enum ForegroundJournalEntry {
    /// A completed piece of foreground work.
    Event(ForegroundEvent),
    /// A semantic interval boundary.
    Boundary(IntervalBoundary),
    /// A change to pending-frame state. This is metadata, not foreground work.
    FrameState(FrameStateChange),
    /// One or more logical entries were unavailable at this point in the
    /// stream. Consumers must not infer interval boundaries across this gap.
    Discontinuity {
        /// Number of unavailable logical entries.
        lost: u64,
    },
}

/// An immutable view of one sealed foreground interval, produced by
/// [`IntervalSealer`].
#[derive(Debug, Clone)]
pub struct FrameSnapshot {
    /// When the interval started (the previous seal, or the end of the idle
    /// stretch preceding the interval's first event).
    pub interval_start: Instant,
    /// The semantic event that completed the interval, including its metadata.
    pub boundary: IntervalBoundary,
    /// Foreground work recorded during the interval, in completion order.
    /// [`ForegroundEvent::SmallPolls`] entries are collected into
    /// `small_polls` instead of appearing here.
    pub events: Vec<ForegroundEvent>,
    /// Span-tagged aggregates of task polls below [`TASK_POLL_FLOOR`]. Spans
    /// are retained so occupancy can apportion folded poll time to reporting
    /// windows narrower than the interval.
    pub small_polls: Vec<SmallPollFlush>,
    /// Events lost to the interval's event cap, plus ring losses reported
    /// via [`IntervalSealer::note_lost`].
    pub dropped_events: u64,
    /// Whether the journal had an unobserved gap during this interval.
    pub journal_discontinuous: bool,
}

impl FrameSnapshot {
    /// When the interval ended.
    pub fn interval_end(&self) -> Instant {
        self.boundary.end_time()
    }

    /// The combined count and total of all folded sub-floor polls in the
    /// interval.
    pub fn small_poll_summary(&self) -> PollSummary {
        let mut summary = PollSummary::default();
        for flush in &self.small_polls {
            summary.add(flush.summary);
        }
        summary
    }

    /// Total foreground time occupied within the interval: the union of the
    /// recorded events' spans (clamped to the interval, so nested work like an
    /// action inside an input dispatch is not double counted) plus the folded
    /// small polls. Folded polls cannot be unioned with nested individually
    /// timed work, so this remains a close approximation.
    pub fn occupancy(&self) -> Duration {
        self.occupancy_within(self.interval_start, self.interval_end())
    }

    /// Like [`Self::occupancy`], but measured against an arbitrary window
    /// (e.g. a reporting window anchored at a frame's first invalidation).
    /// Event spans are clamped to the window. Folded polls carry no
    /// individual timestamps, so each flush's total is apportioned by how
    /// much of its span overlaps the window — assuming the folded time is
    /// spread uniformly across the span.
    pub fn occupancy_within(&self, window_start: Instant, window_end: Instant) -> Duration {
        let mut spans: Vec<(Instant, Instant)> = self
            .events
            .iter()
            .map(|event| {
                let start = event.start_time().max(window_start);
                let end = event.end_time().min(window_end).max(start);
                (start, end)
            })
            .collect();
        spans.sort_by_key(|(start, _)| *start);

        let mut occupied = Duration::ZERO;
        let mut merged_until: Option<Instant> = None;
        for (start, end) in spans {
            let start = match merged_until {
                Some(merged_until) => start.max(merged_until),
                None => start,
            };
            occupied += end.duration_since(start);
            merged_until = Some(match merged_until {
                Some(merged_until) => merged_until.max(end),
                None => end,
            });
        }

        let apportioned_small_polls: Duration = self
            .small_polls
            .iter()
            .map(|flush| {
                let span = flush.until.duration_since(flush.since);
                if span.is_zero() {
                    // A point-like flush lies either inside or outside the
                    // window.
                    if flush.since >= window_start && flush.since <= window_end {
                        flush.summary.total
                    } else {
                        Duration::ZERO
                    }
                } else {
                    let overlap_start = flush.since.max(window_start);
                    let overlap_end = flush.until.min(window_end).max(overlap_start);
                    let overlap = overlap_end.duration_since(overlap_start);
                    flush.summary.total.mul_f64(overlap.div_duration_f64(span))
                }
            })
            .sum();
        occupied + apportioned_small_polls
    }

    /// The fraction of the interval the foreground spent working, in `0.0..=1.0`.
    pub fn busy_fraction(&self) -> f64 {
        let interval = self.interval_end().duration_since(self.interval_start);
        if interval.is_zero() {
            return 1.0;
        }
        (self.occupancy().div_duration_f64(interval)).min(1.0)
    }
}

#[derive(Clone)]
pub(crate) struct ForegroundRunnableCounter(Arc<AtomicUsize>);

impl ForegroundRunnableCounter {
    fn new() -> Self {
        Self(Arc::new(AtomicUsize::new(0)))
    }

    pub(crate) fn queued(&self) {
        self.0.fetch_add(1, Ordering::Release);
    }

    fn finished(&self) {
        let _ = self
            .0
            .fetch_update(Ordering::AcqRel, Ordering::Acquire, |count| {
                count.checked_sub(1)
            });
    }

    fn has_runnables(&self) -> bool {
        self.0.load(Ordering::Acquire) > 0
    }
}

struct ForegroundJournalWriter {
    foreground_runnables: ForegroundRunnableCounter,
    publisher: JournalPublisher,
    turn_depth: usize,
    pending_frames: HashMap<WindowId, Instant>,
    retained_since_boundary: bool,
    small_polls: Option<SmallPollFlush>,
}

impl ForegroundJournalWriter {
    fn new(foreground_runnables: ForegroundRunnableCounter, publisher: JournalPublisher) -> Self {
        Self {
            foreground_runnables,
            publisher,
            turn_depth: 0,
            pending_frames: HashMap::new(),
            retained_since_boundary: false,
            small_polls: None,
        }
    }

    fn begin_turn(&mut self) {
        self.turn_depth += 1;
    }

    // Idle boundaries require a retained event since the last boundary, not
    // merely folded sub-floor polls. Sporadic wake-ups (timers, file
    // watchers) leave the foreground idle after every tiny poll; a boundary
    // for each would re-admit to the ring the very polls the fold keeps out
    // of it, wrapping the ring within seconds. Folded-only work is discarded
    // at true idle so unrelated wake-ups cannot accumulate toward a later
    // frame budget. Polls remain folded while a frame or runnable is pending.
    fn end_turn(&mut self, ended_at: Instant) {
        let Some(turn_depth) = self.turn_depth.checked_sub(1) else {
            debug_assert!(false, "foreground turn must be begun before it ends");
            return;
        };
        self.turn_depth = turn_depth;
        if self.turn_depth > 0
            || self.foreground_runnables.has_runnables()
            || self.has_unexpired_pending_frame(ended_at)
        {
            return;
        }
        if !self.retained_since_boundary {
            self.small_polls = None;
            return;
        }

        self.record_entry(ForegroundJournalEntry::Boundary(IntervalBoundary::Idle {
            ended_at,
        }));
    }

    fn has_unexpired_pending_frame(&mut self, now: Instant) -> bool {
        if self.pending_frames.is_empty() {
            return false;
        }
        self.pending_frames
            .retain(|_, dirty_at| now.saturating_duration_since(*dirty_at) < FRAME_DEADLINE);
        !self.pending_frames.is_empty()
    }

    fn fold_small_poll(&mut self, timing: TaskTiming) {
        let flush = self.small_polls.get_or_insert(SmallPollFlush {
            summary: PollSummary::default(),
            since: timing.start,
            until: timing.end.0,
        });
        flush.summary.count += 1;
        flush.summary.total += timing.poll_duration();
        flush.since = flush.since.min(timing.start);
        flush.until = flush.until.max(timing.end.0);
    }

    fn take_small_polls(&mut self) -> Option<SmallPollFlush> {
        self.small_polls.take()
    }

    fn record_event(&mut self, event: ForegroundEvent) {
        self.retained_since_boundary = true;
        self.record_entry(ForegroundJournalEntry::Event(event));
    }

    fn record_entry(&mut self, entry: ForegroundJournalEntry) {
        let small_polls = self
            .take_small_polls()
            .map(ForegroundEvent::SmallPolls)
            .map(ForegroundJournalEntry::Event);
        self.publisher
            .publish([small_polls, Some(entry)].into_iter().flatten());
        if matches!(entry, ForegroundJournalEntry::Boundary(_)) {
            self.retained_since_boundary = false;
        }
    }

    fn record_frame_state(&mut self, change: FrameStateChange) {
        self.record_entry(ForegroundJournalEntry::FrameState(change));
    }

    fn record_frame_pending(&mut self, window_id: WindowId, dirty_at: Instant) {
        let should_record = match self.pending_frames.get(&window_id) {
            Some(previous_dirty_at) => {
                dirty_at.saturating_duration_since(*previous_dirty_at) >= FRAME_DEADLINE
            }
            None => true,
        };
        if !should_record {
            return;
        }

        self.pending_frames.insert(window_id, dirty_at);
        self.record_frame_state(FrameStateChange::Pending {
            window_id,
            dirty_at,
        });
    }

    fn record_window_closed(&mut self, window_id: WindowId, at: Instant) {
        self.pending_frames.remove(&window_id);
        self.record_frame_state(FrameStateChange::Closed { window_id, at });
    }

    fn record_present(&mut self, timing: PresentTiming, frame: Option<FrameTiming>) {
        match frame {
            Some(frame) => {
                self.pending_frames.remove(&frame.window_id);
                self.record_entry(ForegroundJournalEntry::Boundary(
                    IntervalBoundary::Presented(PresentedFrame {
                        frame,
                        presentation: timing,
                    }),
                ));
            }
            None => {
                self.pending_frames.remove(&timing.window_id);
                self.record_event(ForegroundEvent::Present(timing));
            }
        }
    }
}

thread_local! {
    static FOREGROUND_RUNNABLES: ForegroundRunnableCounter = ForegroundRunnableCounter::new();
    static FOREGROUND_JOURNAL: RefCell<Option<ForegroundJournalWriter>> = const { RefCell::new(None) };
}

pub(crate) fn foreground_runnable_counter() -> ForegroundRunnableCounter {
    FOREGROUND_RUNNABLES.with(Clone::clone)
}

/// Starts journaling on the calling thread. Called once by `App` construction
/// on the main thread; every other thread's recording calls are no-ops.
/// Idempotent so that multiple `App`s on one thread (tests) share one journal.
pub(crate) fn install_foreground_journal() -> ForegroundJournal {
    let foreground_runnables = foreground_runnable_counter();
    FOREGROUND_JOURNAL.with(|journal| {
        let mut journal = journal.borrow_mut();
        if let Some(journal) = journal.as_ref() {
            return ForegroundJournal {
                ring: Arc::clone(&journal.publisher.ring),
            };
        }

        let (handle, publisher) =
            ForegroundJournal::new(MAX_JOURNAL_ENTRIES, MAX_PENDING_JOURNAL_ENTRIES);
        *journal = Some(ForegroundJournalWriter::new(
            foreground_runnables,
            publisher,
        ));
        handle
    })
}

#[cfg(test)]
pub(super) struct TestForegroundJournalGuard {
    previous: Option<ForegroundJournalWriter>,
    _not_send: std::marker::PhantomData<std::rc::Rc<()>>,
}

#[cfg(test)]
impl Drop for TestForegroundJournalGuard {
    fn drop(&mut self) {
        FOREGROUND_JOURNAL.with(|journal| {
            *journal.borrow_mut() = self.previous.take();
        });
    }
}

#[cfg(test)]
pub(super) fn install_test_foreground_journal(
    capacity: usize,
    pending_capacity: usize,
) -> (ForegroundJournal, TestForegroundJournalGuard) {
    let foreground_runnables = foreground_runnable_counter();
    let (handle, publisher) = ForegroundJournal::new(capacity, pending_capacity);
    let previous = FOREGROUND_JOURNAL.with(|journal| {
        journal.borrow_mut().replace(ForegroundJournalWriter::new(
            foreground_runnables,
            publisher,
        ))
    });
    (
        handle,
        TestForegroundJournalGuard {
            previous,
            _not_send: std::marker::PhantomData,
        },
    )
}

fn with_journal(f: impl FnOnce(&mut ForegroundJournalWriter)) {
    FOREGROUND_JOURNAL.with(|journal| {
        if let Some(journal) = journal.borrow_mut().as_mut() {
            f(journal);
        }
    });
}

// TODO(gpui-profiler): the turn brackets in the dispatchers and
// WindowProfiler are bare begin/end call pairs rather than uses of this
// guard. A caught unwind between a pair leaves `turn_depth` (and potentially
// the runnable counter) permanently unbalanced, which silently disables
// idle boundaries for the rest of the process. Zed aborts on panics so
// this is latent today, but gpui-as-a-library callers may catch unwinds;
// route all bracketing through this guard.
pub(crate) struct ForegroundTurnGuard;

impl Drop for ForegroundTurnGuard {
    fn drop(&mut self) {
        end_foreground_turn();
    }
}

pub(crate) fn foreground_turn() -> ForegroundTurnGuard {
    begin_foreground_turn();
    ForegroundTurnGuard
}

pub(crate) fn begin_foreground_turn() {
    with_journal(ForegroundJournalWriter::begin_turn);
}

pub(crate) fn end_foreground_turn() {
    with_journal(|journal| journal.end_turn(Instant::now()));
}

pub(crate) fn record_task_poll(timing: TaskTiming) {
    FOREGROUND_RUNNABLES.with(ForegroundRunnableCounter::finished);
    with_journal(|journal| {
        if timing.poll_duration() >= TASK_POLL_FLOOR {
            journal.record_event(ForegroundEvent::TaskPoll(timing));
        } else {
            journal.fold_small_poll(timing);
        }
        journal.end_turn(timing.end.0);
    });
}

pub(crate) fn record_action(timing: ActionTiming) {
    with_journal(|journal| journal.record_event(ForegroundEvent::Action(timing)));
}

pub(crate) fn record_input(timing: InputTiming) {
    with_journal(|journal| journal.record_event(ForegroundEvent::Input(timing)));
}

pub(crate) fn record_draw(timing: FrameTiming) {
    with_journal(|journal| journal.record_event(ForegroundEvent::Draw(timing)));
}

pub(crate) fn record_present(timing: PresentTiming, frame: Option<FrameTiming>) {
    with_journal(|journal| journal.record_present(timing, frame));
}

pub(crate) fn record_frame_pending(window_id: WindowId, dirty_at: Instant) {
    with_journal(|journal| journal.record_frame_pending(window_id, dirty_at));
}

pub(crate) fn record_window_closed(window_id: WindowId) {
    let at = Instant::now();
    with_journal(|journal| journal.record_window_closed(window_id, at));
}

const SLOT_WRITER: usize = 1 << (usize::BITS - 1);
const SLOT_READER_MASK: usize = !SLOT_WRITER;
const EMPTY_SEQUENCE: u64 = u64::MAX;

struct JournalSlot {
    users: AtomicUsize,
    sequence: AtomicU64,
    entry: UnsafeCell<MaybeUninit<ForegroundJournalEntry>>,
}

// `users` ensures the entry is only read while no writer owns the slot and is
// only written while no readers own it.
unsafe impl Sync for JournalSlot {}

impl JournalSlot {
    fn new() -> Self {
        Self {
            users: AtomicUsize::new(0),
            sequence: AtomicU64::new(EMPTY_SEQUENCE),
            entry: UnsafeCell::new(MaybeUninit::uninit()),
        }
    }

    fn try_publish(&self, sequence: u64, entry: ForegroundJournalEntry) -> bool {
        if self
            .users
            .compare_exchange(0, SLOT_WRITER, Ordering::Acquire, Ordering::Relaxed)
            .is_err()
        {
            return false;
        }

        // SAFETY: setting SLOT_WRITER from zero gives this writer exclusive
        // access to the slot until the Release store below.
        unsafe {
            (*self.entry.get()).write(entry);
        }
        self.sequence.store(sequence, Ordering::Relaxed);
        self.users.store(0, Ordering::Release);
        true
    }

    fn try_read(&self, expected_sequence: u64) -> Option<ForegroundJournalEntry> {
        let _guard = JournalSlotReadGuard::try_new(self)?;
        if self.sequence.load(Ordering::Relaxed) != expected_sequence {
            return None;
        }

        // SAFETY: a matching sequence means the slot was initialized for this
        // logical entry, and `guard` prevents the writer from overwriting it
        // until after the Copy.
        Some(unsafe { *(*self.entry.get()).assume_init_ref() })
    }

    fn try_add_reader(&self) -> bool {
        let mut users = self.users.load(Ordering::Relaxed);
        loop {
            if users & SLOT_WRITER != 0 || users == SLOT_READER_MASK {
                return false;
            }
            match self.users.compare_exchange_weak(
                users,
                users + 1,
                Ordering::Acquire,
                Ordering::Relaxed,
            ) {
                Ok(_) => return true,
                Err(updated_users) => users = updated_users,
            }
        }
    }

    fn remove_reader(&self) {
        let previous = self.users.fetch_sub(1, Ordering::Release);
        debug_assert!(
            previous > 0 && previous & SLOT_WRITER == 0,
            "invalid slot reader state: {previous:#x}"
        );
    }
}

struct JournalSlotReadGuard<'a> {
    slot: &'a JournalSlot,
}

impl<'a> JournalSlotReadGuard<'a> {
    fn try_new(slot: &'a JournalSlot) -> Option<Self> {
        if slot.try_add_reader() {
            Some(Self { slot })
        } else {
            None
        }
    }
}

impl Drop for JournalSlotReadGuard<'_> {
    fn drop(&mut self) {
        self.slot.remove_reader();
    }
}

struct JournalRing {
    slots: Box<[JournalSlot]>,
    finalized: AtomicU64,
    offered: AtomicU64,
}

impl JournalRing {
    fn new(capacity: usize) -> Self {
        let capacity = capacity.max(1);
        Self {
            slots: (0..capacity).map(|_| JournalSlot::new()).collect(),
            finalized: AtomicU64::new(0),
            offered: AtomicU64::new(0),
        }
    }

    fn capacity(&self) -> usize {
        self.slots.len()
    }

    fn try_publish(&self, sequence: u64, entry: ForegroundJournalEntry) -> bool {
        let index = (sequence % self.slots.len() as u64) as usize;
        self.slots[index].try_publish(sequence, entry)
    }

    fn read(&self, sequence: u64) -> Option<ForegroundJournalEntry> {
        let index = (sequence % self.slots.len() as u64) as usize;
        self.slots[index].try_read(sequence)
    }
}

struct PendingJournalEntry {
    sequence: u64,
    entry: ForegroundJournalEntry,
}

struct JournalPublisher {
    ring: Arc<JournalRing>,
    next_sequence: u64,
    pending: VecDeque<PendingJournalEntry>,
    dropped_after_pending: u64,
    pending_capacity: usize,
}

impl JournalPublisher {
    fn new(ring: Arc<JournalRing>, pending_capacity: usize) -> Self {
        Self {
            ring,
            next_sequence: 0,
            pending: VecDeque::with_capacity(pending_capacity),
            dropped_after_pending: 0,
            pending_capacity,
        }
    }

    fn publish(&mut self, entries: impl IntoIterator<Item = ForegroundJournalEntry>) {
        for entry in entries {
            self.flush_pending();
            self.publish_one(entry);
        }
        self.flush_pending();
    }

    fn publish_one(&mut self, entry: ForegroundJournalEntry) {
        let sequence = self.next_sequence;
        self.next_sequence += 1;
        self.ring
            .offered
            .store(self.next_sequence, Ordering::Release);

        if !self.pending.is_empty() || self.dropped_after_pending > 0 {
            if self.dropped_after_pending == 0 && self.pending.len() < self.pending_capacity {
                self.pending
                    .push_back(PendingJournalEntry { sequence, entry });
            } else {
                self.dropped_after_pending += 1;
            }
            return;
        }

        if self.ring.try_publish(sequence, entry) {
            self.ring.finalized.store(sequence + 1, Ordering::Release);
        } else if self.pending_capacity > 0 {
            self.pending
                .push_back(PendingJournalEntry { sequence, entry });
        } else {
            self.dropped_after_pending = 1;
        }
    }

    fn flush_pending(&mut self) {
        while let Some(pending) = self.pending.front() {
            if !self.ring.try_publish(pending.sequence, pending.entry) {
                return;
            }
            let sequence = pending.sequence;
            self.pending.pop_front();
            self.ring.finalized.store(sequence + 1, Ordering::Release);
        }

        if self.dropped_after_pending > 0 {
            self.ring
                .finalized
                .store(self.next_sequence, Ordering::Release);
            self.dropped_after_pending = 0;
        }
    }
}

/// A cloneable handle to one foreground journal stream.
///
/// Each collector has an independent cursor. Collectors briefly pin one slot
/// at a time and never block recording; entries they could not observe are
/// reported as discontinuities. Apps on the same foreground thread share the
/// stream.
#[derive(Clone)]
pub struct ForegroundJournal {
    ring: Arc<JournalRing>,
}

impl ForegroundJournal {
    fn new(capacity: usize, pending_capacity: usize) -> (Self, JournalPublisher) {
        let ring = Arc::new(JournalRing::new(capacity));
        (
            Self {
                ring: Arc::clone(&ring),
            },
            JournalPublisher::new(ring, pending_capacity),
        )
    }

    /// Creates an independent collector that observes entries offered after
    /// this call.
    pub fn collector(&self) -> ForegroundJournalCollector {
        ForegroundJournalCollector {
            cursor: self.ring.offered.load(Ordering::Acquire),
            ring: Arc::clone(&self.ring),
        }
    }
}

/// Entries returned by one [`ForegroundJournalCollector::collect_unseen`] call.
#[derive(Debug, Default)]
pub struct DrainedEntries {
    /// Journal entries recorded since the previous drain, in recording order,
    /// including synthetic [`ForegroundJournalEntry::Discontinuity`] markers at
    /// unavailable logical positions.
    pub entries: Vec<ForegroundJournalEntry>,
    /// Entries unavailable to this collector because they were overwritten or
    /// could not be published after the fixed pending queue filled. This is the
    /// aggregate of the discontinuity markers in `entries`.
    pub lost: u64,
}

/// Reads the foreground stream, tracking a cursor so each call to
/// [`Self::collect_unseen`] returns only entries recorded since the previous
/// call. Independent collectors do not affect each other.
pub struct ForegroundJournalCollector {
    cursor: u64,
    ring: Arc<JournalRing>,
}

impl ForegroundJournalCollector {
    /// Returns entries recorded since the previous call (or since the
    /// collector was created), reporting how many were unavailable before this
    /// drain observed them.
    pub fn collect_unseen(&mut self) -> DrainedEntries {
        let end = self.ring.finalized.load(Ordering::Acquire);
        if self.cursor >= end {
            return DrainedEntries::default();
        }
        let retained_start = end.saturating_sub(self.ring.capacity() as u64);
        let mut lost = retained_start.saturating_sub(self.cursor);
        self.cursor = self.cursor.max(retained_start);
        let mut entries = Vec::with_capacity((end - self.cursor) as usize + usize::from(lost > 0));
        if lost > 0 {
            entries.push(ForegroundJournalEntry::Discontinuity { lost });
        }

        while self.cursor < end {
            if let Some(entry) = self.ring.read(self.cursor) {
                entries.push(entry);
            } else {
                lost += 1;
                match entries.last_mut() {
                    Some(ForegroundJournalEntry::Discontinuity { lost }) => *lost += 1,
                    _ => entries.push(ForegroundJournalEntry::Discontinuity { lost: 1 }),
                }
            }
            self.cursor += 1;
        }

        DrainedEntries { entries, lost }
    }
}

/// A pure state machine that groups foreground journal entries into
/// [`FrameSnapshot`]s.
///
/// Feed it drained entries in recording order. Work is carried across calls
/// until an explicit [`IntervalBoundary`] arrives. Idle time before an
/// interval's first event is excluded without relying on an elapsed-time
/// heuristic.
#[derive(Debug)]
pub struct IntervalSealer {
    interval_start: Instant,
    events: Vec<ForegroundEvent>,
    small_polls: Vec<SmallPollFlush>,
    dropped_events: u64,
    journal_discontinuous: bool,
}

impl IntervalSealer {
    /// Creates a sealer whose first interval starts at `start` (typically
    /// the moment the consumer's collector was created).
    pub fn new(start: Instant) -> Self {
        Self {
            interval_start: start,
            events: Vec::new(),
            small_polls: Vec::new(),
            dropped_events: 0,
            journal_discontinuous: false,
        }
    }

    /// Accounts for a loss supplied outside a collector drain. Collector losses
    /// are already represented by ordered discontinuity entries.
    pub fn note_lost(&mut self, lost: u64) {
        self.dropped_events += lost;
        self.journal_discontinuous |= lost > 0;
    }

    /// Processes a batch of drained entries, returning the snapshots completed
    /// by explicit boundaries. Work without a following boundary is carried
    /// over to subsequent calls.
    pub fn push_entries(
        &mut self,
        entries: impl IntoIterator<Item = ForegroundJournalEntry>,
    ) -> Vec<FrameSnapshot> {
        let mut snapshots = Vec::new();
        for entry in entries {
            match entry {
                ForegroundJournalEntry::Event(event) => {
                    if self.is_empty() {
                        self.interval_start = self.interval_start.max(event.start_time());
                    }
                    match event {
                        ForegroundEvent::SmallPolls(flush) => self.push_small_polls(flush),
                        event => self.push_event(event),
                    }
                }
                ForegroundJournalEntry::Boundary(boundary) => {
                    if let IntervalBoundary::Presented(presented) = boundary {
                        if self.is_empty() {
                            self.interval_start = self
                                .interval_start
                                .max(presented.presentation.present_start);
                        }
                        self.push_event(ForegroundEvent::Present(presented.presentation));
                    }
                    if self.is_empty() {
                        self.interval_start = self.interval_start.max(boundary.end_time());
                    } else {
                        snapshots.push(self.seal(boundary));
                    }
                }
                // Pending-frame state gates boundaries on the writer side;
                // the entries remain in the stream for consumers that want
                // dirty timing, but the sealer has no use for them.
                ForegroundJournalEntry::FrameState(_) => {}
                ForegroundJournalEntry::Discontinuity { lost } => self.note_lost(lost),
            }
        }
        snapshots
    }

    fn is_empty(&self) -> bool {
        self.events.is_empty()
            && self.small_polls.is_empty()
            && self.dropped_events == 0
            && !self.journal_discontinuous
    }

    fn push_event(&mut self, event: ForegroundEvent) {
        if self.events.len() >= MAX_INTERVAL_EVENTS {
            self.dropped_events += 1;
        } else {
            self.events.push(event);
        }
    }

    fn push_small_polls(&mut self, flush: SmallPollFlush) {
        if self.small_polls.len() >= MAX_INTERVAL_EVENTS
            && let Some(last) = self.small_polls.last_mut()
        {
            // Degrade gracefully at the cap: widen the last flush's span
            // instead of dropping poll time, at the cost of coarser
            // apportioning.
            last.summary.add(flush.summary);
            last.since = last.since.min(flush.since);
            last.until = last.until.max(flush.until);
        } else {
            self.small_polls.push(flush);
        }
    }

    fn seal(&mut self, boundary: IntervalBoundary) -> FrameSnapshot {
        let ended = boundary.end_time();
        let snapshot = FrameSnapshot {
            interval_start: self.interval_start,
            boundary,
            events: std::mem::take(&mut self.events),
            small_polls: std::mem::take(&mut self.small_polls),
            dropped_events: std::mem::take(&mut self.dropped_events),
            journal_discontinuous: std::mem::take(&mut self.journal_discontinuous),
        };
        self.interval_start = ended;
        snapshot
    }
}

#[cfg(test)]
mod tests {
    use proptest::prelude::*;
    use scheduler::SpawnTime;

    use super::*;
    use crate::{WindowId, profiler::YieldTime};

    #[test]
    fn draw_waits_for_its_presentation_boundary() {
        let start = Instant::now();
        let input = InputTiming {
            kind: "test",
            start: start + Duration::from_millis(1),
            end: start + Duration::from_millis(2),
            caused_invalidation: true,
        };
        let frame = FrameTiming {
            window_id: WindowId::from(1),
            dirty_at: Some(input.start),
            invalidations: 1,
            draw_start: start + Duration::from_millis(3),
            draw_end: start + Duration::from_millis(4),
        };
        let presentation = PresentTiming {
            window_id: frame.window_id,
            present_start: start + Duration::from_millis(5),
            present_end: start + Duration::from_millis(6),
            animation_interval: None,
        };
        let mut sealer = IntervalSealer::new(start);

        let snapshots = sealer.push_entries([
            ForegroundJournalEntry::Event(ForegroundEvent::Input(input)),
            ForegroundJournalEntry::Event(ForegroundEvent::Draw(frame)),
        ]);
        assert!(snapshots.is_empty());

        let snapshots = sealer.push_entries([ForegroundJournalEntry::Boundary(
            IntervalBoundary::Presented(PresentedFrame {
                frame,
                presentation,
            }),
        )]);
        let [snapshot] = snapshots.as_slice() else {
            panic!("expected one presentation-sealed snapshot, got {snapshots:?}");
        };
        assert_eq!(snapshot.interval_start, input.start);
        assert_eq!(snapshot.interval_end(), presentation.present_end);
        assert_eq!(snapshot.events.len(), 3);
        assert!(matches!(
            snapshot.events.last(),
            Some(ForegroundEvent::Present(timing))
                if timing.present_duration() == Duration::from_millis(1)
        ));
        assert_eq!(
            match snapshot.boundary {
                IntervalBoundary::Presented(presented) => {
                    presented.dirty_to_present_duration()
                }
                IntervalBoundary::Idle { .. } => None,
            },
            Some(Duration::from_millis(5))
        );
    }

    #[test]
    fn outermost_turn_seals_no_frame_work_when_idle() {
        let start = Instant::now();
        let counter = ForegroundRunnableCounter::new();
        let (mut journal, mut collector) = test_journal(counter);
        let events = [
            ForegroundEvent::Input(InputTiming {
                kind: "test",
                start,
                end: start + Duration::from_millis(1),
                caused_invalidation: false,
            }),
            ForegroundEvent::Action(ActionTiming {
                name: "test.action",
                start: start + Duration::from_millis(2),
                end: start + Duration::from_millis(3),
            }),
            ForegroundEvent::TaskPoll(task_timing(
                start + Duration::from_millis(4),
                start + Duration::from_millis(5),
            )),
        ];

        for event in events {
            journal.begin_turn();
            journal.record_event(event);
            journal.end_turn(event.end_time());
        }

        let entries = collector.collect_unseen().entries;
        for event in events {
            assert!(entries.iter().any(|entry| {
                matches!(
                    entry,
                    ForegroundJournalEntry::Boundary(IntervalBoundary::Idle { ended_at })
                        if *ended_at == event.end_time()
                )
            }));
        }
    }

    #[test]
    fn nested_turns_only_seal_after_the_outermost_turn() {
        let start = Instant::now();
        let (mut journal, mut collector) = test_journal(ForegroundRunnableCounter::new());
        let action = ForegroundEvent::Action(ActionTiming {
            name: "test.action",
            start,
            end: start + Duration::from_millis(1),
        });
        let input = ForegroundEvent::Input(InputTiming {
            kind: "test",
            start,
            end: start + Duration::from_millis(2),
            caused_invalidation: false,
        });

        journal.begin_turn();
        journal.begin_turn();
        journal.record_event(action);
        journal.end_turn(action.end_time());
        assert!(!has_boundary_at(
            &collector.collect_unseen().entries,
            action.end_time()
        ));

        journal.record_event(input);
        journal.end_turn(input.end_time());
        assert!(collector.collect_unseen().entries.iter().any(|entry| {
            matches!(
                entry,
                ForegroundJournalEntry::Boundary(IntervalBoundary::Idle { ended_at })
                    if *ended_at == input.end_time()
            )
        }));
    }

    #[test]
    fn an_immediately_ready_runnable_prevents_an_idle_boundary_between_polls() {
        let start = Instant::now();
        let counter = ForegroundRunnableCounter::new();
        let (mut journal, mut collector) = test_journal(counter.clone());
        counter.queued();
        counter.queued();

        let first = ForegroundEvent::TaskPoll(task_timing(start, start + Duration::from_millis(1)));
        journal.begin_turn();
        journal.record_event(first);
        counter.finished();
        journal.end_turn(first.end_time());
        assert!(!has_boundary_at(
            &collector.collect_unseen().entries,
            first.end_time()
        ));

        let second = ForegroundEvent::TaskPoll(task_timing(
            start + Duration::from_millis(2),
            start + Duration::from_millis(3),
        ));
        journal.begin_turn();
        journal.record_event(second);
        counter.finished();
        journal.end_turn(second.end_time());
        assert!(collector.collect_unseen().entries.iter().any(|entry| {
            matches!(
                entry,
                ForegroundJournalEntry::Boundary(IntervalBoundary::Idle { ended_at })
                    if *ended_at == second.end_time()
            )
        }));
    }

    #[test]
    fn a_pending_frame_prevents_idle_until_presentation() {
        let start = Instant::now();
        let window_id = WindowId::from(0xD17A);
        let (mut journal, mut collector) = test_journal(ForegroundRunnableCounter::new());
        journal.record_frame_pending(window_id, start);
        journal.begin_turn();
        journal.record_event(ForegroundEvent::Input(InputTiming {
            kind: "test",
            start,
            end: start + Duration::from_millis(1),
            caused_invalidation: true,
        }));
        let input_end = start + Duration::from_millis(1);
        journal.end_turn(input_end);
        assert!(!has_boundary_at(
            &collector.collect_unseen().entries,
            input_end
        ));

        let frame = frame_timing(window_id, start, start + Duration::from_millis(2));
        let presentation = presentation_timing(window_id, start + Duration::from_millis(3));
        journal.record_present(presentation, Some(frame));
        assert!(collector.collect_unseen().entries.iter().any(|entry| {
            matches!(
                entry,
                ForegroundJournalEntry::Boundary(IntervalBoundary::Presented(presented))
                    if presented.frame.window_id == window_id
            )
        }));
    }

    #[test]
    fn a_present_without_a_draw_clears_the_window_pending_state() {
        let start = Instant::now();
        let presented_at = start + Duration::from_millis(1);
        let window_id = WindowId::from(0xD17B);
        let (mut journal, mut collector) = test_journal(ForegroundRunnableCounter::new());
        journal.record_frame_pending(window_id, start);
        journal.begin_turn();
        journal.record_present(presentation_timing(window_id, presented_at), None);
        journal.end_turn(presented_at);

        let entries = collector.collect_unseen().entries;
        assert!(entries.iter().any(|entry| {
            matches!(
                entry,
                ForegroundJournalEntry::Event(ForegroundEvent::Present(timing))
                    if timing.window_id == window_id
            )
        }));
        assert!(has_boundary_at(&entries, presented_at));
    }

    /// A frame that outlives [`FRAME_DEADLINE`] no longer seals an interval
    /// of its own: the work that starved it and its eventual presentation
    /// stay in one interval, preserving the dirty-to-present association.
    #[test]
    fn a_presentation_after_the_deadline_seals_the_whole_interval() {
        let start = Instant::now();
        let window_id = WindowId::from(0xDEA1);
        let presented_at = start + FRAME_DEADLINE + Duration::from_millis(250);
        let mut sealer = IntervalSealer::new(start);
        let snapshots = sealer.push_entries([
            pending_frame(window_id, start),
            input_entry(start, start + Duration::from_millis(20)),
            ForegroundJournalEntry::Boundary(presented_boundary(window_id, start, presented_at)),
        ]);
        let [snapshot] = snapshots.as_slice() else {
            panic!("expected one presented snapshot, got {snapshots:?}");
        };
        assert!(matches!(
            snapshot.boundary,
            IntervalBoundary::Presented(presented)
                if presented.frame.window_id == window_id
                    && presented.dirty_to_present_duration()
                        == Some(presented_at.duration_since(start))
        ));
        assert_eq!(snapshot.interval_end(), presented_at);
        assert_eq!(snapshot.events.len(), 2);
    }

    /// Frame expiry unblocks idle boundaries on the writer side: retained
    /// work after the deadline seals at its turn's end even though the window
    /// never presented.
    #[test]
    fn an_expired_frame_no_longer_blocks_idle_boundaries() {
        let start = Instant::now();
        let window_id = WindowId::from(0xDEA2);
        let (mut journal, mut collector) = test_journal(ForegroundRunnableCounter::new());
        journal.record_frame_pending(window_id, start);

        let blocked_end = start + Duration::from_millis(20);
        journal.begin_turn();
        journal.record_event(ForegroundEvent::Input(InputTiming {
            kind: "test",
            start,
            end: blocked_end,
            caused_invalidation: true,
        }));
        journal.end_turn(blocked_end);
        assert!(!has_boundary_at(
            &collector.collect_unseen().entries,
            blocked_end
        ));

        let unblocked_end = start + FRAME_DEADLINE + Duration::from_millis(1);
        journal.begin_turn();
        journal.record_event(ForegroundEvent::Input(InputTiming {
            kind: "test",
            start: start + FRAME_DEADLINE,
            end: unblocked_end,
            caused_invalidation: false,
        }));
        journal.end_turn(unblocked_end);
        assert!(has_boundary_at(
            &collector.collect_unseen().entries,
            unblocked_end
        ));
    }

    /// Closing a window clears its pending frame, so idle boundaries resume
    /// without waiting for the deadline.
    #[test]
    fn closing_a_window_unblocks_idle_boundaries() {
        let start = Instant::now();
        let window_id = WindowId::from(0xDEA7);
        let (mut journal, mut collector) = test_journal(ForegroundRunnableCounter::new());
        journal.record_frame_pending(window_id, start);
        journal.record_window_closed(window_id, start + Duration::from_millis(1));

        let event_end = start + Duration::from_millis(2);
        journal.begin_turn();
        journal.record_event(ForegroundEvent::Input(InputTiming {
            kind: "test",
            start: start + Duration::from_millis(1),
            end: event_end,
            caused_invalidation: false,
        }));
        journal.end_turn(event_end);
        assert!(has_boundary_at(
            &collector.collect_unseen().entries,
            event_end
        ));
    }

    /// One window presenting must not unblock idle boundaries while another
    /// window's unexpired frame is still pending.
    #[test]
    fn presenting_one_window_does_not_clear_another_pending_window() {
        let start = Instant::now();
        let first_window = WindowId::from(0xDEA5);
        let second_window = WindowId::from(0xDEA6);
        let (mut journal, mut collector) = test_journal(ForegroundRunnableCounter::new());
        journal.record_frame_pending(first_window, start);
        journal.record_frame_pending(second_window, start + Duration::from_millis(100));
        journal.record_present(
            presentation_timing(first_window, start + Duration::from_millis(500)),
            Some(frame_timing(
                first_window,
                start,
                start + Duration::from_millis(500),
            )),
        );

        let event_end = start + Duration::from_millis(620);
        journal.begin_turn();
        journal.record_event(ForegroundEvent::Input(InputTiming {
            kind: "test",
            start: start + Duration::from_millis(600),
            end: event_end,
            caused_invalidation: false,
        }));
        journal.end_turn(event_end);

        let entries = collector.collect_unseen().entries;
        assert!(entries.iter().any(|entry| {
            matches!(
                entry,
                ForegroundJournalEntry::Boundary(IntervalBoundary::Presented(presented))
                    if presented.frame.window_id == first_window
            )
        }));
        assert!(!has_boundary_at(&entries, event_end));
    }

    #[test]
    fn small_polls_are_flushed_immediately_before_a_retained_event() {
        let start = Instant::now();
        let first = task_timing(
            start + Duration::from_millis(1),
            start + Duration::from_millis(1) + Duration::from_micros(20),
        );
        let second = task_timing(
            start + Duration::from_millis(3),
            start + Duration::from_millis(3) + Duration::from_micros(30),
        );
        let input = InputTiming {
            kind: "test",
            start: start + Duration::from_millis(5),
            end: start + Duration::from_millis(6),
            caused_invalidation: false,
        };
        let (mut journal, mut collector) = test_journal(ForegroundRunnableCounter::new());

        journal.fold_small_poll(first);
        journal.fold_small_poll(second);
        journal.record_event(ForegroundEvent::Input(input));

        let drained = collector.collect_unseen();
        let input_index = drained
            .entries
            .iter()
            .position(|entry| {
                matches!(
                    entry,
                    ForegroundJournalEntry::Event(ForegroundEvent::Input(timing))
                        if timing.start == input.start
                )
            })
            .expect("input event should be retained");
        let Some(ForegroundJournalEntry::Event(ForegroundEvent::SmallPolls(flush))) = input_index
            .checked_sub(1)
            .and_then(|index| drained.entries.get(index))
        else {
            panic!("small-poll summary should immediately precede the input event");
        };
        assert_eq!(flush.summary.count, 2);
        assert_eq!(flush.summary.total, Duration::from_micros(50));
        assert_eq!(flush.since, first.start);
        assert_eq!(flush.until, second.end.0);
    }

    #[test]
    fn small_polls_are_flushed_before_frame_state_changes() {
        let start = Instant::now();
        let window_id = WindowId::from(0xDEA9);
        let poll = task_timing(start, start + Duration::from_micros(50));
        let (mut journal, mut collector) = test_journal(ForegroundRunnableCounter::new());
        journal.fold_small_poll(poll);
        journal.record_frame_pending(window_id, poll.end.0);

        let entries = collector.collect_unseen().entries;
        let pending_index = entries
            .iter()
            .position(|entry| {
                matches!(
                    entry,
                    ForegroundJournalEntry::FrameState(FrameStateChange::Pending {
                        window_id: pending_window,
                        ..
                    }) if *pending_window == window_id
                )
            })
            .expect("pending frame should be retained");
        assert!(matches!(
            pending_index
                .checked_sub(1)
                .and_then(|index| entries.get(index)),
            Some(ForegroundJournalEntry::Event(ForegroundEvent::SmallPolls(flush)))
                if flush.since == poll.start && flush.until == poll.end.0
        ));
    }

    /// Sporadic wake-ups (a tiny folded poll after which the foreground goes
    /// idle, repeatedly) must write nothing to the ring and must not accumulate
    /// toward a later retained interval.
    #[test]
    fn sparse_small_polls_are_discarded_when_the_foreground_returns_to_idle() {
        let start = Instant::now();
        let (mut journal, mut collector) = test_journal(ForegroundRunnableCounter::new());
        let mut tiny_wake = |ended_at: Instant| {
            journal.begin_turn();
            journal.fold_small_poll(task_timing(ended_at - Duration::from_micros(50), ended_at));
            journal.end_turn(ended_at);
        };

        for second in 1..=160 {
            tiny_wake(start + Duration::from_secs(second));
        }
        let quiet = collector.collect_unseen().entries;
        assert!(quiet.is_empty());

        // A later retained event seals normally without inheriting the earlier
        // folded-only wake-ups.
        let retained = ForegroundEvent::TaskPoll(task_timing(
            start + Duration::from_secs(161),
            start + Duration::from_secs(161) + Duration::from_millis(1),
        ));
        journal.begin_turn();
        journal.record_event(retained);
        journal.end_turn(retained.end_time());
        let entries = collector.collect_unseen().entries;
        assert!(has_boundary_at(&entries, retained.end_time()));
        assert!(!entries.iter().any(|entry| matches!(
            entry,
            ForegroundJournalEntry::Event(ForegroundEvent::SmallPolls(_))
        )));
    }

    #[test]
    fn note_lost_is_reported_on_the_next_snapshot_only() {
        let start = Instant::now();
        let mut sealer = IntervalSealer::new(start);
        sealer.note_lost(7);

        let snapshots = sealer.push_entries([
            input_entry(start, start + Duration::from_millis(1)),
            ForegroundJournalEntry::Boundary(IntervalBoundary::Idle {
                ended_at: start + Duration::from_millis(1),
            }),
            input_entry(
                start + Duration::from_millis(2),
                start + Duration::from_millis(3),
            ),
            ForegroundJournalEntry::Boundary(IntervalBoundary::Idle {
                ended_at: start + Duration::from_millis(3),
            }),
        ]);

        assert_eq!(snapshots.len(), 2);
        assert_eq!(snapshots[0].dropped_events, 7);
        assert_eq!(snapshots[1].dropped_events, 0);
    }

    /// Ring losses must be able to surface even when every retained entry was
    /// among the losses: a loss-only interval still seals at the next
    /// boundary rather than sliding away as empty.
    #[test]
    fn losses_alone_seal_a_snapshot_at_the_next_boundary() {
        let start = Instant::now();
        let mut sealer = IntervalSealer::new(start);
        sealer.note_lost(3);

        let snapshots =
            sealer.push_entries([ForegroundJournalEntry::Boundary(IntervalBoundary::Idle {
                ended_at: start + Duration::from_millis(1),
            })]);

        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0].dropped_events, 3);
        assert!(snapshots[0].events.is_empty());
    }

    #[test]
    fn interval_event_cap_counts_overflowing_events() {
        let start = Instant::now();
        let mut sealer = IntervalSealer::new(start);
        let overflow = 5;
        let mut entries: Vec<ForegroundJournalEntry> = (0..(MAX_INTERVAL_EVENTS + overflow) as u64)
            .map(|i| {
                input_entry(
                    start + Duration::from_micros(i),
                    start + Duration::from_micros(i + 1),
                )
            })
            .collect();
        entries.push(ForegroundJournalEntry::Boundary(IntervalBoundary::Idle {
            ended_at: start + Duration::from_micros((MAX_INTERVAL_EVENTS + overflow) as u64 + 1),
        }));

        let snapshots = sealer.push_entries(entries);

        assert_eq!(snapshots.len(), 1);
        assert_eq!(snapshots[0].events.len(), MAX_INTERVAL_EVENTS);
        assert_eq!(snapshots[0].dropped_events, overflow as u64);
    }

    #[test]
    fn collect_unseen_returns_each_entry_once_in_order() {
        let start = Instant::now();
        let (journal, mut publisher) = ForegroundJournal::new(32, 4);
        let mut collector = journal.collector();
        let timestamps: Vec<Instant> = (0..19)
            .map(|i| start + Duration::from_micros(i as u64 + 1))
            .collect();
        publisher.publish(timestamps.iter().map(|&at| input_entry(at, at)));

        let ours = |entries: &[ForegroundJournalEntry]| -> Vec<Instant> {
            entries
                .iter()
                .filter_map(|entry| match entry {
                    ForegroundJournalEntry::Event(ForegroundEvent::Input(timing))
                        if timestamps.contains(&timing.end) =>
                    {
                        Some(timing.end)
                    }
                    _ => None,
                })
                .collect()
        };

        let drained = collector.collect_unseen();
        assert_eq!(ours(&drained.entries), timestamps);

        // The cursor advanced past everything: a second drain sees none of
        // our entries again.
        let drained = collector.collect_unseen();
        assert!(ours(&drained.entries).is_empty());
    }

    #[test]
    fn concurrent_collection_preserves_the_complete_logical_sequence() {
        const ENTRY_COUNT: u64 = 20_000;

        let origin = Instant::now();
        let (journal, mut publisher) = ForegroundJournal::new(8, 4);
        let mut collector = journal.collector();
        let done = Arc::new(std::sync::atomic::AtomicBool::new(false));
        let collector_done = Arc::clone(&done);
        let collector_task = std::thread::spawn(move || {
            let mut observed = Vec::new();
            loop {
                let drained = collector.collect_unseen();
                observed.extend(normalize_ring_drain(origin, drained.entries));
                if collector_done.load(Ordering::Acquire) {
                    let drained = collector.collect_unseen();
                    observed.extend(normalize_ring_drain(origin, drained.entries));
                    return (collector, observed);
                }
                std::thread::yield_now();
            }
        });

        for sequence in 0..ENTRY_COUNT {
            publisher.publish([ring_entry(origin, sequence)]);
        }
        done.store(true, Ordering::Release);

        let (mut collector, mut observed) = collector_task
            .join()
            .expect("collector thread should not panic");
        publisher.flush_pending();
        observed.extend(normalize_ring_drain(
            origin,
            collector.collect_unseen().entries,
        ));

        let mut expected_sequence = 0;
        for entry in observed {
            match entry {
                ModelDrainEntry::Entry(sequence) => {
                    assert_eq!(sequence, expected_sequence);
                    expected_sequence += 1;
                }
                ModelDrainEntry::Discontinuity(lost) => expected_sequence += lost,
            }
        }
        assert_eq!(expected_sequence, ENTRY_COUNT);
        assert_eq!(
            publisher.ring.finalized.load(Ordering::Acquire),
            ENTRY_COUNT
        );
    }

    #[derive(Clone, Debug)]
    enum RingOperation {
        Publish,
        Collect(u8),
        NewCollector,
        Pin(u8),
        Unpin(u8),
    }

    #[derive(Clone, Debug, PartialEq, Eq)]
    enum ModelDrainEntry {
        Entry(u64),
        Discontinuity(u64),
    }

    struct ModelRing {
        capacity: usize,
        pending_capacity: usize,
        next_sequence: u64,
        finalized: u64,
        offered: u64,
        slots: Vec<Option<(u64, u64)>>,
        pinned: Vec<bool>,
        pending: VecDeque<(u64, u64)>,
        dropped_after_pending: u64,
    }

    impl ModelRing {
        fn new(capacity: usize, pending_capacity: usize) -> Self {
            Self {
                capacity,
                pending_capacity,
                next_sequence: 0,
                finalized: 0,
                offered: 0,
                slots: vec![None; capacity],
                pinned: vec![false; capacity],
                pending: VecDeque::new(),
                dropped_after_pending: 0,
            }
        }

        fn publish(&mut self) {
            self.flush_pending();
            let sequence = self.next_sequence;
            self.next_sequence += 1;
            self.offered = self.next_sequence;

            if !self.pending.is_empty() || self.dropped_after_pending > 0 {
                if self.dropped_after_pending == 0 && self.pending.len() < self.pending_capacity {
                    self.pending.push_back((sequence, sequence));
                } else {
                    self.dropped_after_pending += 1;
                }
            } else {
                let index = sequence as usize % self.capacity;
                if self.pinned[index] {
                    if self.pending_capacity > 0 {
                        self.pending.push_back((sequence, sequence));
                    } else {
                        self.dropped_after_pending = 1;
                    }
                } else {
                    self.slots[index] = Some((sequence, sequence));
                    self.finalized = sequence + 1;
                }
            }
            self.flush_pending();
        }

        fn flush_pending(&mut self) {
            while let Some(&(sequence, value)) = self.pending.front() {
                let index = sequence as usize % self.capacity;
                if self.pinned[index] {
                    return;
                }
                self.slots[index] = Some((sequence, value));
                self.pending.pop_front();
                self.finalized = sequence + 1;
            }

            if self.dropped_after_pending > 0 {
                self.finalized = self.next_sequence;
                self.dropped_after_pending = 0;
            }
        }

        fn collect(&self, cursor: &mut u64) -> (Vec<ModelDrainEntry>, u64) {
            let end = self.finalized;
            if *cursor >= end {
                return (Vec::new(), 0);
            }

            let retained_start = end.saturating_sub(self.capacity as u64);
            let mut lost = retained_start.saturating_sub(*cursor);
            *cursor = (*cursor).max(retained_start);
            let mut entries = Vec::new();
            if lost > 0 {
                entries.push(ModelDrainEntry::Discontinuity(lost));
            }
            while *cursor < end {
                let index = *cursor as usize % self.capacity;
                match self.slots[index] {
                    Some((sequence, value)) if sequence == *cursor => {
                        entries.push(ModelDrainEntry::Entry(value));
                    }
                    _ => {
                        lost += 1;
                        match entries.last_mut() {
                            Some(ModelDrainEntry::Discontinuity(lost)) => *lost += 1,
                            _ => entries.push(ModelDrainEntry::Discontinuity(1)),
                        }
                    }
                }
                *cursor += 1;
            }
            (entries, lost)
        }
    }

    fn ring_entry(origin: Instant, sequence: u64) -> ForegroundJournalEntry {
        let at = origin + Duration::from_micros(sequence);
        input_entry(at, at)
    }

    fn normalize_ring_drain(
        origin: Instant,
        entries: Vec<ForegroundJournalEntry>,
    ) -> Vec<ModelDrainEntry> {
        entries
            .into_iter()
            .map(|entry| match entry {
                ForegroundJournalEntry::Event(ForegroundEvent::Input(timing)) => {
                    ModelDrainEntry::Entry(timing.start.duration_since(origin).as_micros() as u64)
                }
                ForegroundJournalEntry::Discontinuity { lost } => {
                    ModelDrainEntry::Discontinuity(lost)
                }
                other => panic!("ring property observed an unexpected entry: {other:?}"),
            })
            .collect()
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct NormalizedEvent {
        kind: u8,
        start: u64,
        end: u64,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct NormalizedSmallPolls {
        count: u64,
        total_micros: u64,
        since: u64,
        until: u64,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct NormalizedSnapshot {
        interval_start: u64,
        boundary_kind: u8,
        interval_end: u64,
        events: Vec<NormalizedEvent>,
        small_polls: Vec<NormalizedSmallPolls>,
        dropped_events: u64,
        journal_discontinuous: bool,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    struct NormalizedSealerTail {
        interval_start: u64,
        events: Vec<NormalizedEvent>,
        small_polls: Vec<NormalizedSmallPolls>,
        dropped_events: u64,
        journal_discontinuous: bool,
    }

    fn completion_order_entries(
        origin: Instant,
        specifications: &[(u8, u8, u8)],
    ) -> Vec<ForegroundJournalEntry> {
        let mut completed_at_micros = 1u64;
        specifications
            .iter()
            .map(|(advance, span, kind)| {
                completed_at_micros += u64::from(*advance) + 1;
                let started_at_micros = completed_at_micros.saturating_sub(u64::from(*span));
                let start = origin + Duration::from_micros(started_at_micros);
                let end = origin + Duration::from_micros(completed_at_micros);
                match kind % 7 {
                    0 => input_entry(start, end),
                    1 => ForegroundJournalEntry::Event(ForegroundEvent::Action(ActionTiming {
                        name: "test.action",
                        start,
                        end,
                    })),
                    2 => ForegroundJournalEntry::Event(ForegroundEvent::TaskPoll(task_timing(
                        start, end,
                    ))),
                    3 => {
                        ForegroundJournalEntry::Event(ForegroundEvent::SmallPolls(SmallPollFlush {
                            summary: PollSummary {
                                count: u64::from(*span % 5) + 1,
                                total: Duration::from_micros(u64::from(*span)),
                            },
                            since: start,
                            until: end,
                        }))
                    }
                    4 => pending_frame(WindowId::from(completed_at_micros), start),
                    5 => ForegroundJournalEntry::Boundary(IntervalBoundary::Idle { ended_at: end }),
                    _ => ForegroundJournalEntry::Boundary(presented_boundary(
                        WindowId::from(completed_at_micros),
                        start,
                        end,
                    )),
                }
            })
            .collect()
    }

    fn normalize_event(origin: Instant, event: ForegroundEvent) -> NormalizedEvent {
        let kind = match event {
            ForegroundEvent::TaskPoll(_) => 0,
            ForegroundEvent::Action(_) => 1,
            ForegroundEvent::Input(_) => 2,
            ForegroundEvent::Draw(_) => 3,
            ForegroundEvent::Present(_) => 4,
            ForegroundEvent::SmallPolls(_) => 5,
        };
        NormalizedEvent {
            kind,
            start: event.start_time().duration_since(origin).as_micros() as u64,
            end: event.end_time().duration_since(origin).as_micros() as u64,
        }
    }

    fn normalize_small_polls(origin: Instant, flush: SmallPollFlush) -> NormalizedSmallPolls {
        NormalizedSmallPolls {
            count: flush.summary.count,
            total_micros: flush.summary.total.as_micros() as u64,
            since: flush.since.duration_since(origin).as_micros() as u64,
            until: flush.until.duration_since(origin).as_micros() as u64,
        }
    }

    fn normalize_snapshot(origin: Instant, snapshot: &FrameSnapshot) -> NormalizedSnapshot {
        NormalizedSnapshot {
            interval_start: snapshot.interval_start.duration_since(origin).as_micros() as u64,
            boundary_kind: match snapshot.boundary {
                IntervalBoundary::Idle { .. } => 0,
                IntervalBoundary::Presented(_) => 1,
            },
            interval_end: snapshot.interval_end().duration_since(origin).as_micros() as u64,
            events: snapshot
                .events
                .iter()
                .copied()
                .map(|event| normalize_event(origin, event))
                .collect(),
            small_polls: snapshot
                .small_polls
                .iter()
                .copied()
                .map(|flush| normalize_small_polls(origin, flush))
                .collect(),
            dropped_events: snapshot.dropped_events,
            journal_discontinuous: snapshot.journal_discontinuous,
        }
    }

    fn normalize_tail(origin: Instant, sealer: &IntervalSealer) -> NormalizedSealerTail {
        NormalizedSealerTail {
            interval_start: sealer.interval_start.duration_since(origin).as_micros() as u64,
            events: sealer
                .events
                .iter()
                .copied()
                .map(|event| normalize_event(origin, event))
                .collect(),
            small_polls: sealer
                .small_polls
                .iter()
                .copied()
                .map(|flush| normalize_small_polls(origin, flush))
                .collect(),
            dropped_events: sealer.dropped_events,
            journal_discontinuous: sealer.journal_discontinuous,
        }
    }

    fn reference_seal(
        origin: Instant,
        entries: &[ForegroundJournalEntry],
    ) -> (Vec<NormalizedSnapshot>, NormalizedSealerTail) {
        let mut interval_start = 0;
        let mut events = Vec::new();
        let mut small_polls = Vec::new();
        let mut snapshots = Vec::new();
        let mut dropped_events = 0;
        let mut journal_discontinuous = false;

        for entry in entries {
            match *entry {
                ForegroundJournalEntry::Event(event) => {
                    let event_start = event.start_time().duration_since(origin).as_micros() as u64;
                    if events.is_empty()
                        && small_polls.is_empty()
                        && dropped_events == 0
                        && !journal_discontinuous
                    {
                        interval_start = interval_start.max(event_start);
                    }
                    match event {
                        ForegroundEvent::SmallPolls(flush) => {
                            small_polls.push(normalize_small_polls(origin, flush));
                        }
                        event => events.push(normalize_event(origin, event)),
                    }
                }
                ForegroundJournalEntry::Boundary(boundary) => {
                    if let IntervalBoundary::Presented(presented) = boundary {
                        let present = ForegroundEvent::Present(presented.presentation);
                        if events.is_empty()
                            && small_polls.is_empty()
                            && dropped_events == 0
                            && !journal_discontinuous
                        {
                            interval_start =
                                interval_start
                                    .max(present.start_time().duration_since(origin).as_micros()
                                        as u64);
                        }
                        events.push(normalize_event(origin, present));
                    }

                    let interval_end =
                        boundary.end_time().duration_since(origin).as_micros() as u64;
                    if events.is_empty()
                        && small_polls.is_empty()
                        && dropped_events == 0
                        && !journal_discontinuous
                    {
                        interval_start = interval_start.max(interval_end);
                    } else {
                        snapshots.push(NormalizedSnapshot {
                            interval_start,
                            boundary_kind: match boundary {
                                IntervalBoundary::Idle { .. } => 0,
                                IntervalBoundary::Presented(_) => 1,
                            },
                            interval_end,
                            events: std::mem::take(&mut events),
                            small_polls: std::mem::take(&mut small_polls),
                            dropped_events: std::mem::take(&mut dropped_events),
                            journal_discontinuous: std::mem::take(&mut journal_discontinuous),
                        });
                        interval_start = interval_end;
                    }
                }
                ForegroundJournalEntry::FrameState(_) => {}
                ForegroundJournalEntry::Discontinuity { lost } => {
                    dropped_events += lost;
                    journal_discontinuous = true;
                }
            }
        }

        (
            snapshots,
            NormalizedSealerTail {
                interval_start,
                events,
                small_polls,
                dropped_events,
                journal_discontinuous,
            },
        )
    }

    fn reference_occupancy_micros(
        event_spans: &[(u8, u8)],
        small_poll_spans: &[(u8, u8, u8)],
        window_start: u8,
        window_end: u8,
    ) -> u64 {
        let mut clamped_spans = event_spans
            .iter()
            .map(|&(first, second)| {
                let start = first.min(second).max(window_start);
                let end = first.max(second).min(window_end).max(start);
                (start, end)
            })
            .collect::<Vec<_>>();
        clamped_spans.sort_unstable();

        let mut occupied = 0u64;
        let mut merged_until = None;
        for (start, end) in clamped_spans {
            let start = merged_until.map_or(start, |until| start.max(until));
            occupied += u64::from(end.saturating_sub(start));
            merged_until = Some(merged_until.map_or(end, |until| until.max(end)));
        }

        occupied
            + small_poll_spans
                .iter()
                .map(|&(first, second, factor)| {
                    let since = first.min(second);
                    let until = first.max(second);
                    if since == until {
                        if since >= window_start && since <= window_end {
                            u64::from(factor)
                        } else {
                            0
                        }
                    } else {
                        let overlap_start = since.max(window_start);
                        let overlap_end = until.min(window_end).max(overlap_start);
                        u64::from(overlap_end - overlap_start) * u64::from(factor)
                    }
                })
                .sum::<u64>()
    }

    #[derive(Debug, Clone)]
    struct WriterOperation {
        kind: u8,
        advance: u16,
        argument: u8,
    }

    #[derive(Debug, Clone, PartialEq, Eq)]
    enum WriterEntry {
        Input(u64),
        SmallPolls {
            count: u64,
            total_micros: u64,
            since: u64,
            until: u64,
        },
        Pending {
            window: u64,
            at: u64,
        },
        Closed {
            window: u64,
            at: u64,
        },
        Present {
            window: u64,
            at: u64,
        },
        Presented {
            window: u64,
            at: u64,
        },
        Idle(u64),
    }

    #[derive(Default)]
    struct ModelSmallPolls {
        count: u64,
        total_micros: u64,
        since: u64,
        until: u64,
    }

    #[derive(Default)]
    struct ModelWriter {
        turn_depth: usize,
        runnables: usize,
        pending_frames: HashMap<u64, u64>,
        retained_since_boundary: bool,
        small_polls: Option<ModelSmallPolls>,
        entries: Vec<WriterEntry>,
    }

    impl ModelWriter {
        fn record_entry(&mut self, entry: WriterEntry) {
            if let Some(small_polls) = self.small_polls.take() {
                self.entries.push(WriterEntry::SmallPolls {
                    count: small_polls.count,
                    total_micros: small_polls.total_micros,
                    since: small_polls.since,
                    until: small_polls.until,
                });
            }
            let boundary = matches!(entry, WriterEntry::Presented { .. } | WriterEntry::Idle(_));
            self.entries.push(entry);
            if boundary {
                self.retained_since_boundary = false;
            }
        }

        fn record_input(&mut self, at: u64) {
            self.retained_since_boundary = true;
            self.record_entry(WriterEntry::Input(at));
        }

        fn fold_small_poll(&mut self, since: u64, until: u64) {
            let small_polls = self.small_polls.get_or_insert(ModelSmallPolls {
                since,
                until,
                ..ModelSmallPolls::default()
            });
            small_polls.count += 1;
            small_polls.total_micros += until - since;
            small_polls.since = small_polls.since.min(since);
            small_polls.until = small_polls.until.max(until);
        }

        fn record_pending(&mut self, window: u64, at: u64) {
            let should_record = self
                .pending_frames
                .get(&window)
                .is_none_or(|previous| at.saturating_sub(*previous) >= 1_000_000);
            if should_record {
                self.pending_frames.insert(window, at);
                self.record_entry(WriterEntry::Pending { window, at });
            }
        }

        fn record_closed(&mut self, window: u64, at: u64) {
            self.pending_frames.remove(&window);
            self.record_entry(WriterEntry::Closed { window, at });
        }

        fn record_present(&mut self, window: u64, at: u64, has_frame: bool) {
            if has_frame {
                self.pending_frames.remove(&window);
                self.record_entry(WriterEntry::Presented { window, at });
            } else {
                self.pending_frames.remove(&window);
                self.retained_since_boundary = true;
                self.record_entry(WriterEntry::Present { window, at });
            }
        }

        fn end_turn(&mut self, at: u64) {
            if self.turn_depth == 0 {
                return;
            }
            self.turn_depth -= 1;
            if self.turn_depth > 0 || self.runnables > 0 {
                return;
            }
            self.pending_frames
                .retain(|_, dirty_at| at.saturating_sub(*dirty_at) < 1_000_000);
            if !self.pending_frames.is_empty() {
                return;
            }
            if !self.retained_since_boundary {
                self.small_polls = None;
                return;
            }
            self.record_entry(WriterEntry::Idle(at));
        }
    }

    fn normalize_writer_entries(
        origin: Instant,
        entries: Vec<ForegroundJournalEntry>,
    ) -> Vec<WriterEntry> {
        let at = |instant: Instant| instant.duration_since(origin).as_micros() as u64;
        entries
            .into_iter()
            .map(|entry| match entry {
                ForegroundJournalEntry::Event(ForegroundEvent::Input(timing)) => {
                    WriterEntry::Input(at(timing.end))
                }
                ForegroundJournalEntry::Event(ForegroundEvent::SmallPolls(flush)) => {
                    WriterEntry::SmallPolls {
                        count: flush.summary.count,
                        total_micros: flush.summary.total.as_micros() as u64,
                        since: at(flush.since),
                        until: at(flush.until),
                    }
                }
                ForegroundJournalEntry::Event(ForegroundEvent::Present(timing)) => {
                    WriterEntry::Present {
                        window: timing.window_id.as_u64(),
                        at: at(timing.present_end),
                    }
                }
                ForegroundJournalEntry::FrameState(FrameStateChange::Pending {
                    window_id,
                    dirty_at,
                }) => WriterEntry::Pending {
                    window: window_id.as_u64(),
                    at: at(dirty_at),
                },
                ForegroundJournalEntry::FrameState(FrameStateChange::Closed {
                    window_id,
                    at: closed_at,
                }) => WriterEntry::Closed {
                    window: window_id.as_u64(),
                    at: at(closed_at),
                },
                ForegroundJournalEntry::Boundary(IntervalBoundary::Presented(presented)) => {
                    WriterEntry::Presented {
                        window: presented.frame.window_id.as_u64(),
                        at: at(presented.presentation.present_end),
                    }
                }
                ForegroundJournalEntry::Boundary(IntervalBoundary::Idle { ended_at }) => {
                    WriterEntry::Idle(at(ended_at))
                }
                other => panic!("writer property emitted an unexpected entry: {other:?}"),
            })
            .collect()
    }

    proptest! {
        #![proptest_config(ProptestConfig {
            failure_persistence: None,
            ..ProptestConfig::default()
        })]

        #[test]
        fn ring_matches_reference_model_under_wrap_collisions_and_independent_cursors(
            capacity in 1usize..=8,
            pending_capacity in 0usize..=4,
            operations in prop::collection::vec(
                prop_oneof![
                    5 => Just(RingOperation::Publish),
                    3 => any::<u8>().prop_map(RingOperation::Collect),
                    1 => Just(RingOperation::NewCollector),
                    2 => any::<u8>().prop_map(RingOperation::Pin),
                    2 => any::<u8>().prop_map(RingOperation::Unpin),
                ],
                1..=128,
            ),
        ) {
            let origin = Instant::now();
            let (journal, mut publisher) = ForegroundJournal::new(capacity, pending_capacity);
            let mut collectors = vec![journal.collector()];
            let mut model_collectors = vec![0];
            let mut model = ModelRing::new(capacity, pending_capacity);
            let mut pins: Vec<Option<JournalSlotReadGuard<'_>>> =
                (0..capacity).map(|_| None).collect();

            for operation in operations {
                match operation {
                    RingOperation::Publish => {
                        let sequence = model.next_sequence;
                        publisher.publish([ring_entry(origin, sequence)]);
                        model.publish();
                    }
                    RingOperation::Collect(collector) => {
                        let index = usize::from(collector) % collectors.len();
                        let drained = collectors[index].collect_unseen();
                        let (expected_entries, expected_lost) =
                            model.collect(&mut model_collectors[index]);
                        let observed_entries = normalize_ring_drain(origin, drained.entries);
                        prop_assert_eq!(observed_entries, expected_entries);
                        prop_assert_eq!(drained.lost, expected_lost);
                    }
                    RingOperation::NewCollector if collectors.len() < 4 => {
                        collectors.push(journal.collector());
                        model_collectors.push(model.offered);
                    }
                    RingOperation::NewCollector => {}
                    RingOperation::Pin(slot) => {
                        let index = usize::from(slot) % capacity;
                        if pins[index].is_none() {
                            pins[index] = JournalSlotReadGuard::try_new(&journal.ring.slots[index]);
                            prop_assert!(pins[index].is_some());
                            model.pinned[index] = true;
                        }
                    }
                    RingOperation::Unpin(slot) => {
                        let index = usize::from(slot) % capacity;
                        pins[index].take();
                        model.pinned[index] = false;
                    }
                }

                prop_assert_eq!(publisher.next_sequence, model.next_sequence);
                prop_assert_eq!(
                    publisher.ring.offered.load(Ordering::Acquire),
                    model.offered
                );
                prop_assert_eq!(
                    publisher.ring.finalized.load(Ordering::Acquire),
                    model.finalized
                );
                prop_assert_eq!(
                    publisher
                        .pending
                        .iter()
                        .map(|entry| entry.sequence)
                        .collect::<Vec<_>>(),
                    model
                        .pending
                        .iter()
                        .map(|(sequence, _)| *sequence)
                        .collect::<Vec<_>>()
                );
                prop_assert_eq!(
                    publisher.dropped_after_pending,
                    model.dropped_after_pending
                );
            }

            for pin in &mut pins {
                pin.take();
            }
            model.pinned.fill(false);
            publisher.flush_pending();
            model.flush_pending();

            for (collector, model_cursor) in collectors.iter_mut().zip(&mut model_collectors) {
                let drained = collector.collect_unseen();
                let (expected_entries, expected_lost) = model.collect(model_cursor);
                let observed_entries = normalize_ring_drain(origin, drained.entries);
                prop_assert_eq!(observed_entries, expected_entries);
                prop_assert_eq!(drained.lost, expected_lost);
            }
        }

        #[test]
        fn a_presentation_seals_exactly_once_regardless_of_delay(
            present_delay_micros in 1u64..=2_000_000
        ) {
            let start = Instant::now();
            let window_id = WindowId::from(0xDEA8);
            let presented_at = start + Duration::from_micros(present_delay_micros);
            let mut sealer = IntervalSealer::new(start);
            let snapshots = sealer.push_entries([
                pending_frame(window_id, start),
                input_entry(start, start + Duration::from_micros(1)),
                ForegroundJournalEntry::Boundary(presented_boundary(
                    window_id,
                    start,
                    presented_at,
                )),
            ]);

            prop_assert_eq!(snapshots.len(), 1);
            prop_assert!(matches!(
                snapshots[0].boundary,
                IntervalBoundary::Presented(_)
            ));
            prop_assert_eq!(snapshots[0].interval_end(), presented_at);
        }

        #[test]
        fn completion_order_sealer_matches_reference_under_arbitrary_batching(
            specifications in prop::collection::vec((0u8..16, 0u8..128, any::<u8>()), 1..=96),
            batch_sizes in prop::collection::vec(1usize..=12, 1..=24),
        ) {
            let origin = Instant::now();
            let entries = completion_order_entries(origin, &specifications);
            let (expected_snapshots, expected_tail) = reference_seal(origin, &entries);

            let mut one_batch_sealer = IntervalSealer::new(origin);
            let one_batch_snapshots = one_batch_sealer
                .push_entries(entries.iter().copied())
                .iter()
                .map(|snapshot| normalize_snapshot(origin, snapshot))
                .collect::<Vec<_>>();
            prop_assert_eq!(&one_batch_snapshots, &expected_snapshots);
            prop_assert_eq!(&normalize_tail(origin, &one_batch_sealer), &expected_tail);

            let mut batched_sealer = IntervalSealer::new(origin);
            let mut batched_snapshots = Vec::new();
            let mut offset = 0;
            let mut batch_index = 0;
            while offset < entries.len() {
                let batch_size = batch_sizes[batch_index % batch_sizes.len()];
                let batch_end = (offset + batch_size).min(entries.len());
                batched_snapshots.extend(
                    batched_sealer
                        .push_entries(entries[offset..batch_end].iter().copied())
                        .iter()
                        .map(|snapshot| normalize_snapshot(origin, snapshot)),
                );
                offset = batch_end;
                batch_index += 1;
            }
            prop_assert_eq!(batched_snapshots, expected_snapshots);
            prop_assert_eq!(normalize_tail(origin, &batched_sealer), expected_tail);
        }

        #[test]
        fn occupancy_matches_interval_union_and_fold_apportionment(
            event_spans in prop::collection::vec((0u8..=128, 0u8..=128), 0..=16),
            small_poll_spans in prop::collection::vec(
                (0u8..=128, 0u8..=128, 0u8..=4),
                0..=12,
            ),
            window in (0u8..=128, 0u8..=128),
        ) {
            let origin = Instant::now();
            let window_start_micros = window.0.min(window.1);
            let window_end_micros = window.0.max(window.1);
            let events = event_spans
                .iter()
                .map(|&(first, second)| {
                    let start = origin + Duration::from_micros(u64::from(first.min(second)));
                    let end = origin + Duration::from_micros(u64::from(first.max(second)));
                    ForegroundEvent::Input(InputTiming {
                        kind: "test",
                        start,
                        end,
                        caused_invalidation: false,
                    })
                })
                .collect();
            let small_polls = small_poll_spans
                .iter()
                .map(|&(first, second, factor)| {
                    let since_micros = first.min(second);
                    let until_micros = first.max(second);
                    let span_micros = u64::from(until_micros - since_micros);
                    let total_micros = if span_micros == 0 {
                        u64::from(factor)
                    } else {
                        span_micros * u64::from(factor)
                    };
                    SmallPollFlush {
                        summary: PollSummary {
                            count: 1,
                            total: Duration::from_micros(total_micros),
                        },
                        since: origin + Duration::from_micros(u64::from(since_micros)),
                        until: origin + Duration::from_micros(u64::from(until_micros)),
                    }
                })
                .collect();
            let snapshot = FrameSnapshot {
                interval_start: origin,
                boundary: IntervalBoundary::Idle {
                    ended_at: origin + Duration::from_micros(128),
                },
                events,
                small_polls,
                dropped_events: 0,
                journal_discontinuous: false,
            };
            let expected_micros = reference_occupancy_micros(
                &event_spans,
                &small_poll_spans,
                window_start_micros,
                window_end_micros,
            );
            let observed = snapshot.occupancy_within(
                origin + Duration::from_micros(u64::from(window_start_micros)),
                origin + Duration::from_micros(u64::from(window_end_micros)),
            );

            prop_assert_eq!(observed, Duration::from_micros(expected_micros));

            let full_expected =
                reference_occupancy_micros(&event_spans, &small_poll_spans, 0, 128);
            prop_assert_eq!(snapshot.occupancy(), Duration::from_micros(full_expected));
            prop_assert!((0.0..=1.0).contains(&snapshot.busy_fraction()));
            prop_assert_eq!(
                snapshot.busy_fraction(),
                (full_expected as f64 / 128.0).min(1.0)
            );
        }

        #[test]
        fn writer_matches_state_model_across_turn_frame_and_runnable_transitions(
            operations in prop::collection::vec(
                (any::<u8>(), any::<u16>(), any::<u8>()).prop_map(
                    |(kind, advance, argument)| WriterOperation {
                        kind,
                        advance,
                        argument,
                    },
                ),
                1..=128,
            ),
        ) {
            let origin = Instant::now();
            let counter = ForegroundRunnableCounter::new();
            let (mut writer, mut collector) = test_journal(counter.clone());
            let mut model = ModelWriter::default();
            let mut at_micros = 100u64;

            for operation in operations {
                if operation.advance % 16 == 0 {
                    at_micros += FRAME_DEADLINE.as_micros() as u64;
                } else {
                    at_micros += u64::from(operation.advance) + 1;
                }
                let at = origin + Duration::from_micros(at_micros);
                let window_id = WindowId::from(u64::from(operation.argument % 4));
                let window = window_id.as_u64();

                match operation.kind % 10 {
                    0 => {
                        writer.begin_turn();
                        model.turn_depth += 1;
                    }
                    1 if model.turn_depth > 0 => {
                        writer.end_turn(at);
                        model.end_turn(at_micros);
                    }
                    1 => {}
                    2 => {
                        writer.record_event(ForegroundEvent::Input(InputTiming {
                            kind: "test",
                            start: at,
                            end: at,
                            caused_invalidation: false,
                        }));
                        model.record_input(at_micros);
                    }
                    3 => {
                        let duration_micros = u64::from(operation.argument % 100);
                        writer.fold_small_poll(task_timing(
                            at - Duration::from_micros(duration_micros),
                            at,
                        ));
                        model.fold_small_poll(
                            at_micros - duration_micros,
                            at_micros,
                        );
                    }
                    4 => {
                        counter.queued();
                        model.runnables += 1;
                    }
                    5 if model.runnables > 0 => {
                        counter.finished();
                        model.runnables -= 1;
                    }
                    5 => {}
                    6 => {
                        writer.record_frame_pending(window_id, at);
                        model.record_pending(window, at_micros);
                    }
                    7 => {
                        writer.record_window_closed(window_id, at);
                        model.record_closed(window, at_micros);
                    }
                    8 => {
                        writer.record_present(
                            presentation_timing(window_id, at),
                            Some(frame_timing(window_id, at, at)),
                        );
                        model.record_present(window, at_micros, true);
                    }
                    _ => {
                        writer.record_present(presentation_timing(window_id, at), None);
                        model.record_present(window, at_micros, false);
                    }
                }

                let drained = collector.collect_unseen();
                prop_assert_eq!(drained.lost, 0);
                prop_assert_eq!(
                    normalize_writer_entries(origin, drained.entries),
                    std::mem::take(&mut model.entries)
                );
                prop_assert_eq!(writer.turn_depth, model.turn_depth);
                prop_assert_eq!(writer.retained_since_boundary, model.retained_since_boundary);
                prop_assert_eq!(writer.pending_frames.len(), model.pending_frames.len());
            }
        }
    }

    fn test_journal(
        foreground_runnables: ForegroundRunnableCounter,
    ) -> (ForegroundJournalWriter, ForegroundJournalCollector) {
        let (journal, publisher) = ForegroundJournal::new(256, 8);
        let collector = journal.collector();
        (
            ForegroundJournalWriter::new(foreground_runnables, publisher),
            collector,
        )
    }

    fn has_boundary_at(entries: &[ForegroundJournalEntry], at: Instant) -> bool {
        entries.iter().any(|entry| {
            matches!(entry, ForegroundJournalEntry::Boundary(boundary) if boundary.end_time() == at)
        })
    }

    fn input_entry(start: Instant, end: Instant) -> ForegroundJournalEntry {
        ForegroundJournalEntry::Event(ForegroundEvent::Input(InputTiming {
            kind: "test",
            start,
            end,
            caused_invalidation: false,
        }))
    }

    fn pending_frame(window_id: WindowId, dirty_at: Instant) -> ForegroundJournalEntry {
        ForegroundJournalEntry::FrameState(FrameStateChange::Pending {
            window_id,
            dirty_at,
        })
    }

    fn frame_timing(window_id: WindowId, dirty_at: Instant, draw_end: Instant) -> FrameTiming {
        FrameTiming {
            window_id,
            dirty_at: Some(dirty_at),
            invalidations: 1,
            draw_start: draw_end,
            draw_end,
        }
    }

    fn presentation_timing(window_id: WindowId, present_end: Instant) -> PresentTiming {
        PresentTiming {
            window_id,
            present_start: present_end,
            present_end,
            animation_interval: None,
        }
    }

    fn presented_boundary(
        window_id: WindowId,
        dirty_at: Instant,
        present_end: Instant,
    ) -> IntervalBoundary {
        IntervalBoundary::Presented(PresentedFrame {
            frame: frame_timing(window_id, dirty_at, present_end),
            presentation: presentation_timing(window_id, present_end),
        })
    }

    fn task_timing(start: Instant, end: Instant) -> TaskTiming {
        TaskTiming {
            location: std::panic::Location::caller(),
            spawned: SpawnTime(start),
            start,
            end: YieldTime(end),
        }
    }
}
