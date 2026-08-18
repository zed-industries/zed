//! A journal of foreground work and the semantic boundaries between activity intervals.
//!
//! The foreground thread records task polls, action handlers, input dispatches,
//! draws, and presentations as [`ForegroundEvent`]s in a bounded global ring.
//! Task polls shorter than [`TASK_POLL_FLOOR`] are folded into summaries, which
//! bound the stream by the number of slow polls while preserving their exact
//! count and total duration.
//!
//! Presentation and foreground quiescence are explicit [`IntervalBoundary`]
//! entries in the same stream. Independent [`ForegroundJournalCollector`]s
//! feed entries to [`IntervalSealer`], a pure state machine that groups all work
//! preceding each boundary into a [`FrameSnapshot`]. The sealer does not infer
//! boundaries from elapsed time or from incidental event kinds such as draws.

use std::cell::RefCell;
use std::collections::{HashMap, VecDeque};
use std::sync::{
    Arc,
    atomic::{AtomicUsize, Ordering},
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
/// foreground-quiescence boundaries and completes its interval explicitly.
pub const FRAME_DEADLINE: Duration = Duration::from_secs(1);

// Backstop against pathological event storms within a single interval. At the
// 100us floor, a fully hung second can produce at most ~10k recordable polls,
// so this bound is only reachable when something is already deeply wrong.
const MAX_INTERVAL_EVENTS: usize = 16 * 1024;

// Allow 4MiB of journal entries. The poll floor and frame cadence bound the
// event rate to roughly 10k per second in the worst case, so this holds
// several seconds of worst-case traffic between consumer drains.
const MAX_JOURNAL_ENTRIES: usize =
    (4 * 1024 * 1024) / core::mem::size_of::<ForegroundJournalEntry>();

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
    fn is_empty(&self) -> bool {
        self.count == 0
    }

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

/// A pending frame that did not reach the platform before its deadline.
#[derive(Debug, Copy, Clone)]
pub struct FrameDeadline {
    /// The window whose frame missed its deadline.
    pub window_id: WindowId,
    /// When the frame first became dirty.
    pub dirty_at: Instant,
    /// The exact deadline that completed the interval.
    pub ended_at: Instant,
}

/// The semantic event that completed a foreground activity interval.
#[derive(Debug, Copy, Clone)]
pub enum IntervalBoundary {
    /// A newly drawn frame was submitted to the platform.
    Presented(PresentedFrame),
    /// A dirty frame did not reach the platform before its deadline.
    FrameDeadline(FrameDeadline),
    /// The foreground returned to an idle platform loop with no frame pending.
    Quiescent {
        /// When the foreground became quiescent.
        ended_at: Instant,
    },
}

impl IntervalBoundary {
    /// When the interval ended.
    pub fn end_time(&self) -> Instant {
        match self {
            Self::Presented(presented) => presented.presentation.present_end,
            Self::FrameDeadline(deadline) => deadline.ended_at,
            Self::Quiescent { ended_at } => *ended_at,
        }
    }

    /// The first invalidation of the frame satisfied by this boundary, if any.
    pub fn dirty_at(&self) -> Option<Instant> {
        match self {
            Self::Presented(presented) => presented.frame.dirty_at,
            Self::FrameDeadline(deadline) => Some(deadline.dirty_at),
            Self::Quiescent { .. } => None,
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
    /// The window closed, so any pending frame no longer blocks quiescence.
    Closed {
        /// The window that closed.
        window_id: WindowId,
        /// When the window closed.
        at: Instant,
    },
}

impl FrameStateChange {
    fn time(&self) -> Instant {
        match self {
            Self::Pending { dirty_at, .. } => *dirty_at,
            Self::Closed { at, .. } => *at,
        }
    }
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
}

impl ForegroundJournalEntry {
    fn time(&self) -> Instant {
        match self {
            Self::Event(event) => event.end_time(),
            Self::Boundary(boundary) => boundary.end_time(),
            Self::FrameState(change) => change.time(),
        }
    }
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
    /// [`ForegroundEvent::SmallPolls`] entries are folded into `small_polls`
    /// instead of appearing here.
    pub events: Vec<ForegroundEvent>,
    /// Aggregate of task polls below [`TASK_POLL_FLOOR`].
    pub small_polls: PollSummary,
    /// Events lost to the interval's event cap, plus ring losses reported
    /// via [`IntervalSealer::note_lost`].
    pub dropped_events: u64,
}

impl FrameSnapshot {
    /// When the interval ended.
    pub fn interval_end(&self) -> Instant {
        self.boundary.end_time()
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
    /// Event spans are clamped to the window; the small-poll summary is added
    /// wholesale since folded polls carry no individual timestamps.
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
        occupied + self.small_polls.total
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

struct ForegroundJournal {
    foreground_runnables: ForegroundRunnableCounter,
    turn_depth: usize,
    pending_frames: HashMap<WindowId, Instant>,
    work_since_boundary: bool,
    small_polls: Option<SmallPollFlush>,
}

impl ForegroundJournal {
    fn new(foreground_runnables: ForegroundRunnableCounter) -> Self {
        Self {
            foreground_runnables,
            turn_depth: 0,
            pending_frames: HashMap::new(),
            work_since_boundary: false,
            small_polls: None,
        }
    }

    fn begin_turn(&mut self) {
        self.turn_depth += 1;
    }

    fn end_turn(&mut self, ended_at: Instant) {
        let Some(turn_depth) = self.turn_depth.checked_sub(1) else {
            debug_assert!(false, "foreground turn must be begun before it ends");
            return;
        };
        self.turn_depth = turn_depth;
        if self.turn_depth > 0
            || self.foreground_runnables.has_runnables()
            || self.has_unexpired_pending_frame(ended_at)
            || !self.work_since_boundary
        {
            return;
        }

        self.record_entry(ForegroundJournalEntry::Boundary(
            IntervalBoundary::Quiescent { ended_at },
        ));
    }

    fn has_unexpired_pending_frame(&mut self, now: Instant) -> bool {
        self.pending_frames
            .retain(|_, dirty_at| now.saturating_duration_since(*dirty_at) < FRAME_DEADLINE);
        !self.pending_frames.is_empty()
    }

    fn fold_small_poll(&mut self, timing: TaskTiming) {
        self.work_since_boundary = true;
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
        self.work_since_boundary = true;
        self.record_entry(ForegroundJournalEntry::Event(event));
    }

    fn record_entry(&mut self, entry: ForegroundJournalEntry) {
        let small_polls = self
            .take_small_polls()
            .map(ForegroundEvent::SmallPolls)
            .map(ForegroundJournalEntry::Event);
        push_to_ring([small_polls, Some(entry)].into_iter().flatten());
        if matches!(entry, ForegroundJournalEntry::Boundary(_)) {
            self.work_since_boundary = false;
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
            None => self.record_event(ForegroundEvent::Present(timing)),
        }
    }
}

thread_local! {
    static FOREGROUND_RUNNABLES: ForegroundRunnableCounter = ForegroundRunnableCounter::new();
    static FOREGROUND_JOURNAL: RefCell<Option<ForegroundJournal>> = const { RefCell::new(None) };
}

pub(crate) fn foreground_runnable_counter() -> ForegroundRunnableCounter {
    FOREGROUND_RUNNABLES.with(Clone::clone)
}

/// Starts journaling on the calling thread. Called once by `App` construction
/// on the main thread; every other thread's recording calls are no-ops.
/// Idempotent so that multiple `App`s on one thread (tests) share one journal.
pub(crate) fn install_foreground_journal() {
    let foreground_runnables = foreground_runnable_counter();
    FOREGROUND_JOURNAL.with(|journal| {
        let mut journal = journal.borrow_mut();
        if journal.is_none() {
            *journal = Some(ForegroundJournal::new(foreground_runnables));
        }
    });
}

fn with_journal(f: impl FnOnce(&mut ForegroundJournal)) {
    FOREGROUND_JOURNAL.with(|journal| {
        if let Some(journal) = journal.borrow_mut().as_mut() {
            f(journal);
        }
    });
}

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
    with_journal(ForegroundJournal::begin_turn);
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

struct JournalRing {
    entries: VecDeque<ForegroundJournalEntry>,
    total_pushed: u64,
}

// The poll floor and frame cadence bound the push rate, drains happen about
// once per second, and the lock is never held across blocking work, so a
// spinlock is appropriate here as elsewhere in the profiler.
static FOREGROUND_ENTRIES: spin::Mutex<JournalRing> = spin::Mutex::new(JournalRing {
    entries: VecDeque::new(),
    total_pushed: 0,
});

fn push_to_ring(entries: impl IntoIterator<Item = ForegroundJournalEntry>) {
    let mut ring = FOREGROUND_ENTRIES.lock();
    for entry in entries {
        if ring.entries.len() >= MAX_JOURNAL_ENTRIES {
            ring.entries.pop_front();
        }
        ring.entries.push_back(entry);
        ring.total_pushed += 1;
    }
}

/// Entries returned by one [`ForegroundJournalCollector::collect_unseen`] call.
#[derive(Debug, Default)]
pub struct DrainedEntries {
    /// Journal entries recorded since the previous drain, in recording order.
    pub entries: Vec<ForegroundJournalEntry>,
    /// Entries overwritten in the ring before this drain observed them.
    pub lost: u64,
}

/// Reads the foreground stream, tracking a cursor so each call to
/// [`Self::collect_unseen`] returns only entries recorded since the previous
/// call. Independent collectors do not affect each other.
pub struct ForegroundJournalCollector {
    cursor: u64,
}

impl Default for ForegroundJournalCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl ForegroundJournalCollector {
    /// Creates a collector that only sees entries recorded from this point on.
    pub fn new() -> Self {
        Self {
            cursor: FOREGROUND_ENTRIES.lock().total_pushed,
        }
    }

    /// Returns entries recorded since the previous call (or since the
    /// collector was created), reporting how many were overwritten in the
    /// ring before this drain observed them.
    pub fn collect_unseen(&mut self) -> DrainedEntries {
        let ring = FOREGROUND_ENTRIES.lock();
        let buffer_len = ring.entries.len() as u64;
        let buffer_start = ring.total_pushed.saturating_sub(buffer_len);
        let lost = buffer_start.saturating_sub(self.cursor);
        let skip = self.cursor.saturating_sub(buffer_start) as usize;
        let entries = ring
            .entries
            .iter()
            .skip(skip.min(ring.entries.len()))
            .copied()
            .collect();
        self.cursor = ring.total_pushed;
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
    small_polls: PollSummary,
    dropped_events: u64,
    pending_frames: HashMap<WindowId, Instant>,
}

impl IntervalSealer {
    /// Creates a sealer whose first interval starts at `start` (typically
    /// the moment the consumer's collector was created).
    pub fn new(start: Instant) -> Self {
        Self {
            interval_start: start,
            events: Vec::new(),
            small_polls: PollSummary::default(),
            dropped_events: 0,
            pending_frames: HashMap::new(),
        }
    }

    /// Accounts for entries lost before observation (see
    /// [`DrainedEntries::lost`]); they are reported on the next snapshot.
    pub fn note_lost(&mut self, lost: u64) {
        self.dropped_events += lost;
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
            let boundary_cancels_exact_deadline = matches!(
                entry,
                ForegroundJournalEntry::Boundary(IntervalBoundary::Presented(_))
                    | ForegroundJournalEntry::FrameState(FrameStateChange::Closed { .. })
            );
            snapshots.extend(self.advance_to(entry.time(), !boundary_cancels_exact_deadline));
            match entry {
                ForegroundJournalEntry::Event(event) => {
                    if self.is_empty() {
                        self.interval_start = self.interval_start.max(event.start_time());
                    }
                    match event {
                        ForegroundEvent::SmallPolls(flush) => self.small_polls.add(flush.summary),
                        event => self.push_event(event),
                    }
                }
                ForegroundJournalEntry::Boundary(boundary) => {
                    if let IntervalBoundary::Presented(presented) = boundary {
                        self.pending_frames.remove(&presented.frame.window_id);
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
                ForegroundJournalEntry::FrameState(change) => match change {
                    FrameStateChange::Pending {
                        window_id,
                        dirty_at,
                    } => {
                        self.pending_frames
                            .entry(window_id)
                            .and_modify(|pending_at| *pending_at = (*pending_at).min(dirty_at))
                            .or_insert(dirty_at);
                    }
                    FrameStateChange::Closed { window_id, .. } => {
                        self.pending_frames.remove(&window_id);
                    }
                },
            }
        }
        snapshots
    }

    /// Completes pending frames whose exact deadline is at or before `now`.
    /// Silent polling can therefore report a frame that never presents without
    /// manufacturing timeout boundaries for unrelated foreground work.
    pub fn advance(&mut self, now: Instant) -> Vec<FrameSnapshot> {
        self.advance_to(now, true)
    }

    fn advance_to(&mut self, now: Instant, include_now: bool) -> Vec<FrameSnapshot> {
        let mut snapshots = Vec::new();
        loop {
            let next_expired = self
                .pending_frames
                .iter()
                .filter_map(|(window_id, dirty_at)| {
                    let ended_at = *dirty_at + FRAME_DEADLINE;
                    let expired = ended_at < now || (include_now && ended_at == now);
                    expired.then_some((*window_id, *dirty_at, ended_at))
                })
                .min_by_key(|(_, _, ended_at)| *ended_at);
            let Some((window_id, dirty_at, ended_at)) = next_expired else {
                break;
            };

            self.pending_frames.remove(&window_id);
            let boundary = IntervalBoundary::FrameDeadline(FrameDeadline {
                window_id,
                dirty_at,
                ended_at,
            });
            if self.is_empty() {
                self.interval_start = self.interval_start.max(ended_at);
            } else {
                snapshots.push(self.seal(boundary));
            }
        }
        snapshots
    }

    fn is_empty(&self) -> bool {
        self.events.is_empty() && self.small_polls.is_empty() && self.dropped_events == 0
    }

    fn push_event(&mut self, event: ForegroundEvent) {
        if self.events.len() >= MAX_INTERVAL_EVENTS {
            self.dropped_events += 1;
        } else {
            self.events.push(event);
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
                IntervalBoundary::FrameDeadline(_) | IntervalBoundary::Quiescent { .. } => None,
            },
            Some(Duration::from_millis(5))
        );
    }

    #[test]
    fn outermost_turn_seals_no_frame_work_at_quiescence() {
        let start = Instant::now();
        let counter = ForegroundRunnableCounter::new();
        let mut collector = ForegroundJournalCollector::new();
        let mut journal = ForegroundJournal::new(counter);
        let events = [
            ForegroundEvent::Input(InputTiming {
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
                    ForegroundJournalEntry::Boundary(IntervalBoundary::Quiescent { ended_at })
                        if *ended_at == event.end_time()
                )
            }));
        }
    }

    #[test]
    fn nested_turns_only_seal_after_the_outermost_turn() {
        let start = Instant::now();
        let mut collector = ForegroundJournalCollector::new();
        let mut journal = ForegroundJournal::new(ForegroundRunnableCounter::new());
        let action = ForegroundEvent::Action(ActionTiming {
            name: "test.action",
            start,
            end: start + Duration::from_millis(1),
        });
        let input = ForegroundEvent::Input(InputTiming {
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
                ForegroundJournalEntry::Boundary(IntervalBoundary::Quiescent { ended_at })
                    if *ended_at == input.end_time()
            )
        }));
    }

    #[test]
    fn an_immediately_ready_runnable_prevents_quiescence_between_polls() {
        let start = Instant::now();
        let counter = ForegroundRunnableCounter::new();
        let mut collector = ForegroundJournalCollector::new();
        let mut journal = ForegroundJournal::new(counter.clone());
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
                ForegroundJournalEntry::Boundary(IntervalBoundary::Quiescent { ended_at })
                    if *ended_at == second.end_time()
            )
        }));
    }

    #[test]
    fn a_pending_frame_prevents_quiescence_until_presentation() {
        let start = Instant::now();
        let window_id = WindowId::from(0xD17A);
        let mut collector = ForegroundJournalCollector::new();
        let mut journal = ForegroundJournal::new(ForegroundRunnableCounter::new());
        journal.record_frame_pending(window_id, start);
        journal.begin_turn();
        journal.record_event(ForegroundEvent::Input(InputTiming {
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
    fn pending_frame_seals_at_its_exact_deadline() {
        let start = Instant::now();
        let window_id = WindowId::from(0xDEA1);
        let mut sealer = IntervalSealer::new(start);
        let snapshots = sealer.push_entries([
            pending_frame(window_id, start),
            input_entry(
                start + Duration::from_millis(1),
                start + Duration::from_millis(20),
            ),
        ]);
        assert!(snapshots.is_empty());
        assert!(
            sealer
                .advance(start + FRAME_DEADLINE - Duration::from_nanos(1))
                .is_empty()
        );

        let snapshots = sealer.advance(start + FRAME_DEADLINE);
        let [snapshot] = snapshots.as_slice() else {
            panic!("expected one deadline snapshot, got {snapshots:?}");
        };
        assert_eq!(snapshot.interval_end(), start + FRAME_DEADLINE);
        assert!(matches!(
            snapshot.boundary,
            IntervalBoundary::FrameDeadline(FrameDeadline {
                window_id: deadline_window,
                dirty_at,
                ended_at,
            }) if deadline_window == window_id
                && dirty_at == start
                && ended_at == start + FRAME_DEADLINE
        ));
    }

    #[test]
    fn expired_frame_no_longer_blocks_later_quiescence() {
        let start = Instant::now();
        let window_id = WindowId::from(0xDEA2);
        let mut sealer = IntervalSealer::new(start);
        sealer.push_entries([
            pending_frame(window_id, start),
            input_entry(start, start + Duration::from_millis(20)),
        ]);
        assert_eq!(sealer.advance(start + FRAME_DEADLINE).len(), 1);

        let event_start = start + FRAME_DEADLINE + Duration::from_millis(1);
        let event_end = event_start + Duration::from_millis(20);
        let snapshots = sealer.push_entries([
            input_entry(event_start, event_end),
            ForegroundJournalEntry::Boundary(IntervalBoundary::Quiescent {
                ended_at: event_end,
            }),
        ]);
        let [snapshot] = snapshots.as_slice() else {
            panic!("expected one quiescent snapshot, got {snapshots:?}");
        };
        assert!(matches!(
            snapshot.boundary,
            IntervalBoundary::Quiescent { ended_at } if ended_at == event_end
        ));
    }

    #[test]
    fn presentation_before_deadline_cancels_it() {
        let start = Instant::now();
        let window_id = WindowId::from(0xDEA3);
        let presented_at = start + FRAME_DEADLINE / 2;
        let mut sealer = IntervalSealer::new(start);
        let snapshots = sealer.push_entries([
            pending_frame(window_id, start),
            input_entry(start, start + Duration::from_millis(20)),
            ForegroundJournalEntry::Boundary(presented_boundary(window_id, start, presented_at)),
        ]);
        assert_eq!(snapshots.len(), 1);
        assert!(matches!(
            snapshots[0].boundary,
            IntervalBoundary::Presented(_)
        ));
        assert!(sealer.advance(start + FRAME_DEADLINE * 2).is_empty());
    }

    #[test]
    fn closing_a_window_at_its_deadline_cancels_the_pending_frame() {
        let start = Instant::now();
        let window_id = WindowId::from(0xDEA7);
        let mut sealer = IntervalSealer::new(start);
        let snapshots = sealer.push_entries([
            pending_frame(window_id, start),
            input_entry(start, start + Duration::from_millis(20)),
            ForegroundJournalEntry::FrameState(FrameStateChange::Closed {
                window_id,
                at: start + FRAME_DEADLINE,
            }),
            ForegroundJournalEntry::Boundary(IntervalBoundary::Quiescent {
                ended_at: start + FRAME_DEADLINE,
            }),
        ]);
        let [snapshot] = snapshots.as_slice() else {
            panic!("expected one quiescent snapshot, got {snapshots:?}");
        };
        assert!(matches!(
            snapshot.boundary,
            IntervalBoundary::Quiescent { .. }
        ));
        assert!(sealer.advance(start + FRAME_DEADLINE * 2).is_empty());
    }

    #[test]
    fn late_presentation_starts_a_new_interval_after_the_deadline() {
        let start = Instant::now();
        let window_id = WindowId::from(0xDEA4);
        let presented_at = start + FRAME_DEADLINE + Duration::from_millis(250);
        let mut sealer = IntervalSealer::new(start);
        let snapshots = sealer.push_entries([
            pending_frame(window_id, start),
            input_entry(start, start + Duration::from_millis(20)),
            ForegroundJournalEntry::Boundary(presented_boundary(window_id, start, presented_at)),
        ]);
        assert_eq!(snapshots.len(), 2);
        assert!(matches!(
            snapshots[0].boundary,
            IntervalBoundary::FrameDeadline(_)
        ));
        assert!(matches!(
            snapshots[1].boundary,
            IntervalBoundary::Presented(_)
        ));
        assert_eq!(snapshots[1].events.len(), 1);
        assert!(matches!(
            snapshots[1].events[0],
            ForegroundEvent::Present(_)
        ));
    }

    #[test]
    fn presenting_one_window_does_not_clear_another_pending_window() {
        let start = Instant::now();
        let first_window = WindowId::from(0xDEA5);
        let second_window = WindowId::from(0xDEA6);
        let second_dirty_at = start + Duration::from_millis(100);
        let mut sealer = IntervalSealer::new(start);
        let snapshots = sealer.push_entries([
            pending_frame(first_window, start),
            pending_frame(second_window, second_dirty_at),
            input_entry(start, start + Duration::from_millis(20)),
            ForegroundJournalEntry::Boundary(presented_boundary(
                first_window,
                start,
                start + Duration::from_millis(500),
            )),
            input_entry(
                start + Duration::from_millis(600),
                start + Duration::from_millis(620),
            ),
        ]);
        assert_eq!(snapshots.len(), 1);
        assert!(matches!(
            snapshots[0].boundary,
            IntervalBoundary::Presented(_)
        ));

        let snapshots = sealer.advance(second_dirty_at + FRAME_DEADLINE);
        let [snapshot] = snapshots.as_slice() else {
            panic!("expected the second window's deadline, got {snapshots:?}");
        };
        assert!(matches!(
            snapshot.boundary,
            IntervalBoundary::FrameDeadline(FrameDeadline { window_id, .. })
                if window_id == second_window
        ));
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
            start: start + Duration::from_millis(5),
            end: start + Duration::from_millis(6),
            caused_invalidation: false,
        };
        let mut collector = ForegroundJournalCollector::new();
        let mut journal = ForegroundJournal::new(ForegroundRunnableCounter::new());

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
        let mut collector = ForegroundJournalCollector::new();
        let mut journal = ForegroundJournal::new(ForegroundRunnableCounter::new());
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

    proptest! {
        #![proptest_config(ProptestConfig {
            failure_persistence: None,
            ..ProptestConfig::default()
        })]

        #[test]
        fn presentation_deadline_ordering_is_stable(
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

            if presented_at <= start + FRAME_DEADLINE {
                prop_assert_eq!(snapshots.len(), 1);
                prop_assert!(matches!(
                    snapshots[0].boundary,
                    IntervalBoundary::Presented(_)
                ));
            } else {
                prop_assert_eq!(snapshots.len(), 2);
                prop_assert!(matches!(
                    snapshots[0].boundary,
                    IntervalBoundary::FrameDeadline(_)
                ));
                prop_assert!(matches!(
                    snapshots[1].boundary,
                    IntervalBoundary::Presented(_)
                ));
            }
        }

        #[test]
        fn explicit_boundaries_partition_events_exactly_once(
            steps in prop::collection::vec((0u16..1000, 1u16..1000, 0u8..3), 1..128)
        ) {
            let origin = Instant::now();
            let mut cursor = origin;
            let mut entries = Vec::with_capacity(steps.len() * 2);
            let mut expected_boundary_ends = Vec::new();

            for (idle_micros, duration_micros, boundary_kind) in &steps {
                let event_start = cursor + Duration::from_micros(u64::from(*idle_micros));
                let event_end = event_start + Duration::from_micros(u64::from(*duration_micros));
                entries.push(ForegroundJournalEntry::Event(ForegroundEvent::Input(
                    InputTiming {
                        start: event_start,
                        end: event_end,
                        caused_invalidation: false,
                    },
                )));
                cursor = event_end;

                let boundary = match boundary_kind {
                    1 => Some(IntervalBoundary::Quiescent { ended_at: cursor }),
                    2 => Some(presented_boundary_at(cursor)),
                    _ => None,
                };
                if let Some(boundary) = boundary {
                    expected_boundary_ends.push(boundary.end_time());
                    entries.push(ForegroundJournalEntry::Boundary(boundary));
                }
            }

            let mut sealer = IntervalSealer::new(origin);
            let snapshots = sealer.push_entries(entries);
            let observed_boundary_ends = snapshots
                .iter()
                .map(|snapshot| snapshot.interval_end())
                .collect::<Vec<_>>();
            prop_assert_eq!(observed_boundary_ends, expected_boundary_ends);

            let sealed_event_count: usize = snapshots
                .iter()
                .map(|snapshot| snapshot.events.len())
                .sum();
            let presentation_count = steps
                .iter()
                .filter(|(_, _, boundary_kind)| *boundary_kind == 2)
                .count();
            prop_assert_eq!(
                sealed_event_count + sealer.events.len(),
                steps.len() + presentation_count
            );
            for snapshot in &snapshots {
                prop_assert!(!snapshot.events.is_empty());
                prop_assert_eq!(snapshot.interval_start, snapshot.events[0].start_time());
                prop_assert!(snapshot.interval_start <= snapshot.interval_end());
            }
        }
    }

    fn has_boundary_at(entries: &[ForegroundJournalEntry], at: Instant) -> bool {
        entries.iter().any(|entry| {
            matches!(entry, ForegroundJournalEntry::Boundary(boundary) if boundary.end_time() == at)
        })
    }

    fn input_entry(start: Instant, end: Instant) -> ForegroundJournalEntry {
        ForegroundJournalEntry::Event(ForegroundEvent::Input(InputTiming {
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

    fn presented_boundary_at(at: Instant) -> IntervalBoundary {
        let frame = FrameTiming {
            window_id: WindowId::from(2),
            dirty_at: Some(at),
            invalidations: 1,
            draw_start: at,
            draw_end: at,
        };
        IntervalBoundary::Presented(PresentedFrame {
            frame,
            presentation: PresentTiming {
                window_id: frame.window_id,
                present_start: at,
                present_end: at,
                animation_interval: None,
            },
        })
    }
}
