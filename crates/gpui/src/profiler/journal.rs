//! A journal of everything the foreground thread does between window frames.
//!
//! The foreground thread records task polls, action handlers, input
//! dispatches, draws, and presents as a flat stream of raw
//! [`ForegroundEvent`]s in a bounded global ring. Recording applies no
//! interval policy beyond folding task polls shorter than
//! [`TASK_POLL_FLOOR`] into periodically flushed
//! [`ForegroundEvent::SmallPolls`] summaries, which keep
//! foreground-occupancy accounting exact while bounding the stream by the
//! number of slow polls.
//!
//! Consumers read the stream through independent [`ForegroundEventCollector`]
//! cursors and interpret it with [`IntervalSealer`], a pure state machine
//! that groups events into [`FrameSnapshot`]s: one per drawn frame, or one
//! per [`SEAL_TIMEOUT`] of undrawn foreground work. Sealing is a
//! deterministic function of event content, so snapshots are a view over the
//! stream rather than a storage format: consumers with different needs can
//! apply different interpretations to the same events.

use std::cell::RefCell;
use std::collections::VecDeque;
use std::time::Duration;

use scheduler::Instant;

use super::{ActionTiming, FrameTiming, PresentTiming, TaskTiming};

/// Task polls shorter than this are folded into a [`PollSummary`] instead of
/// being recorded individually. This keeps the stream bounded by the number
/// of *slow* polls while preserving exact foreground-occupancy accounting.
pub const TASK_POLL_FLOOR: Duration = Duration::from_micros(100);

/// [`IntervalSealer`] seals an interval after this long when no window draws,
/// so foreground work that never invalidates a window (or happens while all
/// windows are occluded and receive no frame callbacks) still becomes
/// observable. The writer also flushes its small-poll summary at this
/// cadence, so the stream never lags reality by more than one timeout.
pub const SEAL_TIMEOUT: Duration = Duration::from_secs(1);

// Backstop against pathological event storms within a single interval. At the
// 100us floor, a fully hung second can produce at most ~10k recordable polls,
// so this bound is only reachable when something is already deeply wrong.
const MAX_INTERVAL_EVENTS: usize = 16 * 1024;

// Allow 4MiB of journal entries. The poll floor and frame cadence bound the
// event rate to roughly 10k per second in the worst case, so this holds
// several seconds of worst-case traffic between consumer drains.
const MAX_JOURNAL_EVENTS: usize = (4 * 1024 * 1024) / core::mem::size_of::<ForegroundEvent>();

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
    /// A newly drawn frame reaching the screen. Presents follow the draw
    /// they belong to in the stream, so [`IntervalSealer`] attributes them to
    /// the interval after that draw's.
    Present(PresentTiming),
    /// Aggregate of task polls below [`TASK_POLL_FLOOR`], flushed ahead of
    /// every draw and at least once per [`SEAL_TIMEOUT`].
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
            Self::Present(timing) => timing.presented_at,
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
            Self::Present(timing) => timing.presented_at,
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

/// Exact accounting of task polls below [`TASK_POLL_FLOOR`]. Together with
/// the individually recorded events this preserves total foreground
/// occupancy.
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

/// A flushed [`PollSummary`] covering a span of the stream.
#[derive(Debug, Copy, Clone)]
pub struct SmallPollFlush {
    /// The folded polls.
    pub summary: PollSummary,
    /// Start of the span the summary covers (the previous flush).
    pub since: Instant,
    /// End of the span the summary covers.
    pub until: Instant,
}

/// Why [`IntervalSealer`] sealed an interval.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum SealReason {
    /// A window finished drawing.
    Draw,
    /// No window drew for [`SEAL_TIMEOUT`]. Idle stretches produce no
    /// snapshots, so a timeout snapshot always contains foreground work.
    Timeout,
}

/// An immutable view of one sealed foreground interval, produced by
/// [`IntervalSealer`].
#[derive(Debug, Clone)]
pub struct FrameSnapshot {
    /// When the interval started (the previous seal, or the end of the idle
    /// stretch preceding the interval's first event).
    pub interval_start: Instant,
    /// When the interval was sealed.
    pub interval_end: Instant,
    /// Why the interval was sealed.
    pub reason: SealReason,
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
    /// Total foreground time occupied within the interval: the union of the
    /// recorded events' spans (clamped to the interval, so nested work like an
    /// action inside an input dispatch is not double counted) plus the folded
    /// small polls. Small-poll flush spans may straddle a seal, so this is a
    /// close approximation rather than exact at interval edges.
    pub fn occupancy(&self) -> Duration {
        let mut spans: Vec<(Instant, Instant)> = self
            .events
            .iter()
            .map(|event| {
                let start = event.start_time().max(self.interval_start);
                let end = event.end_time().min(self.interval_end).max(start);
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
        let interval = self.interval_end.duration_since(self.interval_start);
        if interval.is_zero() {
            return 1.0;
        }
        (self.occupancy().div_duration_f64(interval)).min(1.0)
    }
}

struct ForegroundJournal {
    small_polls: PollSummary,
    flushed_at: Instant,
}

impl ForegroundJournal {
    fn new(now: Instant) -> Self {
        Self {
            small_polls: PollSummary::default(),
            flushed_at: now,
        }
    }

    fn fold_small_poll(&mut self, duration: Duration) {
        self.small_polls.count += 1;
        self.small_polls.total += duration;
    }

    /// Flushes the small-poll summary into the stream. `force` flushes ahead
    /// of the flush cadence (used before draws so the summary lands in the
    /// interval it belongs to). Empty flushes emit no event but still slide
    /// the covered span forward.
    fn flush_small_polls(&mut self, now: Instant, force: bool) {
        if !force && now.duration_since(self.flushed_at) < SEAL_TIMEOUT {
            return;
        }
        if !self.small_polls.is_empty() {
            push_to_ring(ForegroundEvent::SmallPolls(SmallPollFlush {
                summary: std::mem::take(&mut self.small_polls),
                since: self.flushed_at,
                until: now,
            }));
        }
        self.flushed_at = now;
    }
}

thread_local! {
    static FOREGROUND_JOURNAL: RefCell<Option<ForegroundJournal>> = const { RefCell::new(None) };
}

/// Starts journaling on the calling thread. Called once by `App` construction
/// on the main thread; every other thread's recording calls are no-ops.
/// Idempotent so that multiple `App`s on one thread (tests) share one journal.
pub(crate) fn install_foreground_journal() {
    FOREGROUND_JOURNAL.with(|journal| {
        let mut journal = journal.borrow_mut();
        if journal.is_none() {
            *journal = Some(ForegroundJournal::new(Instant::now()));
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

pub(crate) fn record_task_poll(timing: TaskTiming) {
    with_journal(|journal| {
        if timing.poll_duration() >= TASK_POLL_FLOOR {
            push_to_ring(ForegroundEvent::TaskPoll(timing));
        } else {
            journal.fold_small_poll(timing.poll_duration());
        }
        journal.flush_small_polls(timing.end.0, false);
    });
}

pub(crate) fn record_action(timing: ActionTiming) {
    with_journal(|journal| {
        push_to_ring(ForegroundEvent::Action(timing));
        journal.flush_small_polls(timing.end, false);
    });
}

pub(crate) fn record_input(timing: InputTiming) {
    with_journal(|journal| {
        push_to_ring(ForegroundEvent::Input(timing));
        journal.flush_small_polls(timing.end, false);
    });
}

pub(crate) fn record_draw(timing: FrameTiming) {
    with_journal(|journal| {
        journal.flush_small_polls(timing.draw_end, true);
        push_to_ring(ForegroundEvent::Draw(timing));
    });
}

pub(crate) fn record_present(timing: PresentTiming) {
    with_journal(|journal| {
        push_to_ring(ForegroundEvent::Present(timing));
        journal.flush_small_polls(timing.presented_at, false);
    });
}

struct EventRing {
    events: VecDeque<ForegroundEvent>,
    total_pushed: u64,
}

// The poll floor and frame cadence bound the push rate, drains happen about
// once per second, and the lock is never held across blocking work, so a
// spinlock is appropriate here as elsewhere in the profiler.
static FOREGROUND_EVENTS: spin::Mutex<EventRing> = spin::Mutex::new(EventRing {
    events: VecDeque::new(),
    total_pushed: 0,
});

fn push_to_ring(event: ForegroundEvent) {
    let mut ring = FOREGROUND_EVENTS.lock();
    if ring.events.len() >= MAX_JOURNAL_EVENTS {
        ring.events.pop_front();
    }
    ring.events.push_back(event);
    ring.total_pushed += 1;
}

/// Events returned by one [`ForegroundEventCollector::collect_unseen`] call.
#[derive(Debug, Default)]
pub struct DrainedEvents {
    /// The events recorded since the previous drain, in recording order.
    pub events: Vec<ForegroundEvent>,
    /// Events overwritten in the ring before this drain observed them.
    pub lost: u64,
}

/// Reads the foreground stream, tracking a cursor so each call to
/// [`Self::collect_unseen`] returns only events recorded since the previous
/// call. Independent collectors do not affect each other.
pub struct ForegroundEventCollector {
    cursor: u64,
}

impl Default for ForegroundEventCollector {
    fn default() -> Self {
        Self::new()
    }
}

impl ForegroundEventCollector {
    /// Creates a collector that only sees events recorded from this point on.
    pub fn new() -> Self {
        Self {
            cursor: FOREGROUND_EVENTS.lock().total_pushed,
        }
    }

    /// Returns events recorded since the previous call (or since the
    /// collector was created), reporting how many were overwritten in the
    /// ring before this drain observed them.
    pub fn collect_unseen(&mut self) -> DrainedEvents {
        let ring = FOREGROUND_EVENTS.lock();
        let buffer_len = ring.events.len() as u64;
        let buffer_start = ring.total_pushed.saturating_sub(buffer_len);
        let lost = buffer_start.saturating_sub(self.cursor);
        let skip = self.cursor.saturating_sub(buffer_start) as usize;
        let events = ring
            .events
            .iter()
            .skip(skip.min(ring.events.len()))
            .copied()
            .collect();
        self.cursor = ring.total_pushed;
        DrainedEvents { events, lost }
    }
}

/// A pure state machine that groups a foreground event stream into
/// [`FrameSnapshot`]s.
///
/// Feed it drained events in recording order; it seals one snapshot per
/// [`ForegroundEvent::Draw`] and one per [`SEAL_TIMEOUT`] of undrawn
/// foreground work. Idle stretches produce no snapshots: when the stream is
/// silent for longer than [`SEAL_TIMEOUT`], the next interval starts at its
/// first event rather than spanning the idle period.
#[derive(Debug)]
pub struct IntervalSealer {
    interval_start: Instant,
    events: Vec<ForegroundEvent>,
    small_polls: PollSummary,
    dropped_events: u64,
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
        }
    }

    /// Accounts for events lost before observation (see
    /// [`DrainedEvents::lost`]); they are reported on the next snapshot.
    pub fn note_lost(&mut self, lost: u64) {
        self.dropped_events += lost;
    }

    /// Processes a batch of drained events, returning the snapshots sealed
    /// by it. Events that do not yet complete an interval are carried over
    /// to subsequent calls.
    pub fn push_events(
        &mut self,
        events: impl IntoIterator<Item = ForegroundEvent>,
    ) -> Vec<FrameSnapshot> {
        let mut snapshots = Vec::new();
        for event in events {
            if self.is_empty() {
                let start = event.start_time();
                if start.duration_since(self.interval_start) >= SEAL_TIMEOUT {
                    self.interval_start = start;
                }
            }

            let end = event.end_time();
            let is_draw = matches!(event, ForegroundEvent::Draw(_));
            match event {
                ForegroundEvent::SmallPolls(flush) => self.small_polls.add(flush.summary),
                event => self.push_event(event),
            }

            if is_draw {
                snapshots.push(self.seal(SealReason::Draw, end));
            } else if end.duration_since(self.interval_start) >= SEAL_TIMEOUT {
                snapshots.push(self.seal(SealReason::Timeout, end));
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

    fn seal(&mut self, reason: SealReason, ended: Instant) -> FrameSnapshot {
        let snapshot = FrameSnapshot {
            interval_start: self.interval_start,
            interval_end: ended,
            reason,
            events: std::mem::take(&mut self.events),
            small_polls: std::mem::take(&mut self.small_polls),
            dropped_events: std::mem::take(&mut self.dropped_events),
        };
        self.interval_start = ended;
        snapshot
    }
}
