use std::{
    cell::{OnceCell, RefCell},
    collections::HashMap,
    future::Future,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use anyhow::{Result, anyhow};
use hdrhistogram::Histogram;

use crate::{
    AnyView, AnyWindowHandle, App, AppCell, AppContext, BackgroundExecutor, Bounds, Context, Empty,
    Entity, EntityId, Focusable, ForegroundExecutor, Global, Platform, PlatformHeadlessRenderer,
    PlatformTextSystem, Render, Reservation, Task, TestPlatform, ThreadedDispatcher, VisualContext,
    Window, WindowBounds, WindowHandle, WindowOptions,
    app::GpuiBorrow,
    profiler::{
        self, FrameEvent, FrameTimingCollector,
        journal::{ForegroundEvent, ForegroundJournalCollector, ForegroundJournalEntry},
    },
};

/// Returns a benchmark platform backed by this thread's shared dispatcher.
///
/// The platform uses this thread's shared multithreaded [`ThreadedDispatcher`], so
/// background work runs with production concurrency in real time. The dispatcher
/// is cached per thread and reused across benchmark invocations so worker and
/// timer threads persist for the whole process instead of being recreated for
/// every Criterion calibration pass.
///
/// Text is shaped with the provided platform text system. Benchmarks generated
/// by `#[gpui::bench]` use the current platform's text system, so text-heavy
/// benchmark measurements include production shaping and glyph rasterization.
///
/// `headless_renderer_factory` supplies a renderer for benchmark windows, e.g.
/// `gpui_platform::current_headless_renderer`. When present, scenes drawn by
/// benchmarks are rasterized through the real sprite atlas and submitted to
/// the GPU on present, so quad/sprite regressions show up in measurements.
/// When `None`, presenting discards the scene. Currently only macOS provides
/// a headless renderer (Metal), so GPU submission is excluded from benchmark
/// measurements on other platforms.
pub fn bench_platform(
    headless_renderer_factory: Option<Box<dyn Fn() -> Option<Box<dyn PlatformHeadlessRenderer>>>>,
    text_system: Arc<dyn PlatformTextSystem>,
) -> Rc<dyn Platform> {
    thread_local! {
        static DISPATCHER: OnceCell<Arc<ThreadedDispatcher>> = const { OnceCell::new() };
    }
    let dispatcher = DISPATCHER.with(|cell| {
        cell.get_or_init(|| Arc::new(ThreadedDispatcher::new()))
            .clone()
    });
    let background_executor = BackgroundExecutor::new(dispatcher.clone());
    let foreground_executor = ForegroundExecutor::new(dispatcher);
    TestPlatform::with_platform(
        background_executor,
        foreground_executor,
        text_system,
        headless_renderer_factory,
    ) as Rc<dyn Platform>
}

/// Default target frame rate when a benchmark doesn't specify `fps = N`.
const DEFAULT_FPS: u64 = 120;

const NANOS_PER_SECOND: u128 = 1_000_000_000;

const RETAINED_SLOW_INTERVALS: usize = 16;

/// Bounded attribution for a slow renderer loop or presentation interval.
#[derive(Clone, Debug)]
struct SlowInterval {
    /// Wall time between the interval's boundaries.
    duration: Duration,
    /// Union of individually timestamped work, including draws and presentation.
    recorded_busy: Duration,
    /// Folded polls have no individual spans; this total is not added to `recorded_busy`.
    folded_poll_work: Duration,
    /// Number of recorded draws ending in this interval.
    draws: u64,
    /// Longest individually timestamped work intersecting the interval.
    longest_work: Duration,
    /// Retains source identity without formatting strings while collecting events.
    longest_event: Option<ForegroundEvent>,
    /// Journal loss prevents treating the attribution as complete.
    incomplete: bool,
}

impl SlowInterval {
    fn longest_work_description(&self) -> String {
        match self.longest_event {
            Some(ForegroundEvent::TaskPoll(timing)) => {
                format!("task poll at {}", timing.location)
            }
            Some(ForegroundEvent::Action(timing)) => format!("action {}", timing.name),
            Some(ForegroundEvent::Input(timing)) => format!("input {}", timing.kind),
            Some(ForegroundEvent::Draw(timing)) => format!("draw for {:?}", timing.window_id),
            Some(ForegroundEvent::Present(timing)) => {
                format!("present for {:?}", timing.window_id)
            }
            Some(ForegroundEvent::SmallPolls(_)) | None => "unattributed".into(),
        }
    }
}

/// Aggregate statistics for total foreground executor work observed during a
/// measured interval, returned by [`BenchReport::foreground_work`].
#[derive(Clone, Copy, Debug)]
pub struct ForegroundWorkSummary {
    /// Number of foreground work items recorded: task polls, action
    /// handlers, input dispatches, and folded sub-floor poll flushes.
    pub count: u64,
    /// Sum of every recorded item's duration.
    pub total: Duration,
    /// The longest single recorded item.
    pub max: Duration,
    /// 50th percentile duration.
    pub p50: Duration,
    /// 90th percentile duration.
    pub p90: Duration,
    /// 95th percentile duration.
    pub p95: Duration,
    /// 99th percentile duration.
    pub p99: Duration,
    /// How many whole frame budgets (at the report's configured FPS) were
    /// exceeded in total, summed across every recorded item.
    pub frame_budget_overruns_total: u64,
    /// How many whole frame budgets the longest recorded item exceeded.
    pub frame_budget_overruns_max: u64,
}

/// A small report produced by GPUI benchmarks.
#[derive(Clone)]
pub struct BenchReport {
    frame_snapshot: Rc<RefCell<WindowFrameSnapshot>>,
    frame_budget_nanos: u128,
}

impl Default for BenchReport {
    fn default() -> Self {
        Self::with_fps(DEFAULT_FPS)
    }
}

impl BenchReport {
    fn record_trace(&self, events: TracedEvents) {
        self.record_frame_timings(events.frame_events.iter());
        self.record_foreground_events(events.foreground_events());
        let mut snapshot = self.frame_snapshot.borrow_mut();
        snapshot
            .whole_loop
            .histogram
            .add(&events.loops.histogram.histogram)
            .expect("compatible histograms");
        snapshot.whole_loop.total_nanos += events.loops.histogram.total_nanos;
        snapshot
            .draws_per_loop
            .add(&events.loops.draws)
            .expect("compatible histograms");
        let lost: u64 = events
            .journal_entries
            .iter()
            .filter_map(|entry| match entry {
                ForegroundJournalEntry::Discontinuity { lost } => Some(*lost),
                _ => None,
            })
            .sum();
        snapshot.lost_journal_entries += lost;
        let work: Vec<_> = events
            .journal_entries
            .iter()
            .filter_map(|entry| match entry {
                ForegroundJournalEntry::Event(event) => Some(*event),
                ForegroundJournalEntry::Boundary(
                    crate::profiler::journal::IntervalBoundary::Presented(frame),
                ) => Some(ForegroundEvent::Present(frame.presentation)),
                _ => None,
            })
            .collect();
        for &(start, end) in &events.loops.slowest {
            if end.duration_since(start).as_nanos() > self.frame_budget_nanos {
                snapshot
                    .slow_loops
                    .push(attribute_interval(&work, start, end, lost > 0));
            }
        }
        snapshot
            .slow_loops
            .sort_by_key(|interval| std::cmp::Reverse(interval.duration));
        snapshot.slow_loops.truncate(RETAINED_SLOW_INTERVALS);
        // Presentation cadence is per window and never crosses setup/teardown.
        // Animation-only intervals omit non-animation frames.
        let mut last_present = HashMap::new();
        let mut candidates = Vec::with_capacity(RETAINED_SLOW_INTERVALS + 1);
        let mut generation = 0;
        for entry in &events.journal_entries {
            let timing = match entry {
                ForegroundJournalEntry::Event(ForegroundEvent::Present(timing)) => timing,
                ForegroundJournalEntry::Boundary(
                    crate::profiler::journal::IntervalBoundary::Presented(frame),
                ) => &frame.presentation,
                ForegroundJournalEntry::Discontinuity { .. } => {
                    generation += 1;
                    continue;
                }
                _ => continue,
            };
            if let Some((start, previous_generation)) =
                last_present.insert(timing.window_id, (timing.present_end, generation))
            {
                let duration = timing.present_end.duration_since(start);
                let incomplete = previous_generation != generation;
                if !incomplete {
                    snapshot
                        .presentation_cadence
                        .record(duration.as_nanos() as u64)
                        .ok();
                }
                if duration.as_nanos() > self.frame_budget_nanos {
                    candidates.push((start, timing.present_end, incomplete));
                    candidates.sort_by_key(|(start, end, _)| {
                        std::cmp::Reverse(end.duration_since(*start))
                    });
                    candidates.truncate(RETAINED_SLOW_INTERVALS);
                }
            }
        }
        for (start, end, incomplete) in candidates {
            // An enclosing span can complete after the presentation. A later
            // journal gap may have lost it even when cadence itself is intact.
            snapshot.slow_intervals.push(attribute_interval(
                &work,
                start,
                end,
                incomplete || lost > 0,
            ));
        }
        snapshot
            .slow_intervals
            .sort_by_key(|interval| std::cmp::Reverse(interval.duration));
        snapshot.slow_intervals.truncate(RETAINED_SLOW_INTERVALS);
    }

    /// Creates a report whose per-frame budget is one frame at `fps` when
    /// counting frame budget overruns.
    ///
    /// # Panics
    ///
    /// Panics if `fps` is zero or yields a frame budget below one nanosecond.
    pub fn with_fps(fps: u64) -> Self {
        assert!(fps > 0, "frame rate must be greater than zero");
        Self::with_frame_budget_nanos(NANOS_PER_SECOND / fps as u128)
    }

    /// Creates a report that treats `frame_budget_nanos` as the per-frame budget
    /// when counting frame budget overruns.
    ///
    /// # Panics
    ///
    /// Panics if `frame_budget_nanos` is zero.
    pub fn with_frame_budget_nanos(frame_budget_nanos: u128) -> Self {
        assert!(
            frame_budget_nanos > 0,
            "frame budget must be at least one nanosecond"
        );
        Self {
            frame_snapshot: Rc::new(RefCell::new(WindowFrameSnapshot::new())),
            frame_budget_nanos,
        }
    }

    fn record_frame_timings<'i>(&self, events: impl IntoIterator<Item = &'i FrameEvent>) {
        let mut snapshot = self.frame_snapshot.borrow_mut();
        // `.ok()` on `record`: this operation is infallible (the histograms auto-resize).
        for event in events {
            match event {
                FrameEvent::Draw(timing) => {
                    snapshot
                        .draw
                        .record(timing.draw_duration().as_nanos() as u64)
                        .ok();
                    if let Some(dirty_to_draw) = timing.dirty_to_draw_duration() {
                        snapshot
                            .dirty_to_draw
                            .record(dirty_to_draw.as_nanos() as u64)
                            .ok();
                    }
                    if timing.invalidations > 0 {
                        snapshot
                            .invalidations_per_frame
                            .record(timing.invalidations)
                            .ok();
                    }
                }
                FrameEvent::Present(timing) => {
                    if let Some(animation_interval) = timing.animation_interval {
                        snapshot
                            .present_interval
                            .record(animation_interval.as_nanos() as u64)
                            .ok();
                    }
                }
            }
        }
    }

    /// Records total foreground executor work observed during a measured
    /// interval: task polls, action handlers, and input dispatches, whether
    /// or not they produced a window draw. Draws and presents are excluded
    /// here since [`Self::record_frame_timings`] already accounts for them.
    fn record_foreground_events<'i>(&self, events: impl IntoIterator<Item = &'i ForegroundEvent>) {
        let mut snapshot = self.frame_snapshot.borrow_mut();
        for event in events {
            let duration = match event {
                ForegroundEvent::Draw(_) | ForegroundEvent::Present(_) => continue,
                // A flush's span (used by `ForegroundEvent::duration`) is not
                // the time spent polling; its summary total is.
                ForegroundEvent::SmallPolls(flush) => flush.summary.total,
                _ => event.duration(),
            };
            snapshot.foreground_work.record(duration);
        }
    }

    fn total_budget_overruns(&self, histogram: &Histogram<u64>) -> u64 {
        histogram
            .iter_recorded()
            .map(|value| {
                self.budget_overruns(Duration::from_nanos(value.value_iterated_to()))
                    * value.count_at_value()
            })
            .sum()
    }

    /// Returns how many whole frame budgets `foreground_time` exceeded the
    /// per frame budget by. This is a synthetic proxy for missed frames: the
    /// benchmark harness has no vsync, so it counts how many frame deadlines
    /// would have elapsed while the foreground thread was busy.
    fn budget_overruns(&self, foreground_time: Duration) -> u64 {
        let foreground_nanos = foreground_time.as_nanos();
        if foreground_nanos <= self.frame_budget_nanos {
            return 0;
        }

        let over_budget_nanos = foreground_nanos - self.frame_budget_nanos;
        over_budget_nanos.div_ceil(self.frame_budget_nanos) as u64
    }

    /// Returns aggregate statistics for total foreground executor work
    /// observed during the measured interval: every task poll, action
    /// handler, and input dispatch on the foreground thread, whether or not
    /// it produced a window draw. This is captured through GPUI's foreground
    /// journal, so it requires no window and surfaces a slow or stalled task
    /// even when nothing was drawn while it ran.
    ///
    /// Returns `None` when no foreground work was recorded, e.g. a
    /// [`BenchAppContext::bench_iter`] measurement that does no async work.
    pub fn foreground_work(&self) -> Option<ForegroundWorkSummary> {
        let frame_snapshot = self.frame_snapshot.borrow();
        let foreground_work = &frame_snapshot.foreground_work;
        if foreground_work.histogram.is_empty() {
            return None;
        }

        let max = Duration::from_nanos(foreground_work.histogram.max());
        Some(ForegroundWorkSummary {
            count: foreground_work.histogram.len(),
            total: Duration::from_nanos(foreground_work.total_nanos),
            max,
            p50: Duration::from_nanos(foreground_work.histogram.value_at_quantile(0.50)),
            p90: Duration::from_nanos(foreground_work.histogram.value_at_quantile(0.90)),
            p95: Duration::from_nanos(foreground_work.histogram.value_at_quantile(0.95)),
            p99: Duration::from_nanos(foreground_work.histogram.value_at_quantile(0.99)),
            frame_budget_overruns_total: self.total_budget_overruns(&foreground_work.histogram),
            frame_budget_overruns_max: self.budget_overruns(max),
        })
    }

    /// Prints this report to stderr.
    pub fn print(&self, benchmark_name: Option<&'static str>) {
        let frame_snapshot = self.frame_snapshot.borrow();
        if frame_snapshot.is_empty() {
            return;
        }

        let benchmark_name = benchmark_name.unwrap_or("unknown benchmark");
        eprintln!("GPUI bench report (all observed iterations): {benchmark_name}");
        eprintln!("  note: includes Criterion warmup/calibration");
        self.print_histogram("window dirty-to-draw", &frame_snapshot.dirty_to_draw);
        self.print_histogram("window draw", &frame_snapshot.draw);
        self.print_histogram(
            "animation presentation interval",
            &frame_snapshot.present_interval,
        );
        self.print_histogram("renderer whole loop", &frame_snapshot.whole_loop.histogram);
        self.print_histogram(
            "presentation cadence (all presentations)",
            &frame_snapshot.presentation_cadence,
        );
        if !frame_snapshot.draws_per_loop.is_empty() {
            eprintln!(
                "  draws per renderer loop: mean {:.2}, max {}",
                frame_snapshot.draws_per_loop.mean(),
                frame_snapshot.draws_per_loop.max()
            );
        }
        if frame_snapshot.lost_journal_entries > 0 {
            eprintln!(
                "  incomplete journal diagnostics: {} lost entries; affected cadence samples excluded",
                frame_snapshot.lost_journal_entries
            );
        }
        for (label, intervals) in [
            ("renderer loop", &frame_snapshot.slow_loops),
            ("presentation", &frame_snapshot.slow_intervals),
        ] {
            for interval in intervals {
                eprintln!(
                    "  slow {label}: {} wall, {} recorded span union, {} folded polls (not added), {} draws; longest {} {}{}",
                    format_duration(interval.duration),
                    format_duration(interval.recorded_busy),
                    format_duration(interval.folded_poll_work),
                    interval.draws,
                    interval.longest_work_description(),
                    format_duration(interval.longest_work),
                    if interval.incomplete {
                        " [incomplete journal]"
                    } else {
                        ""
                    }
                );
            }
        }
        if !frame_snapshot.invalidations_per_frame.is_empty() {
            eprintln!(
                "  invalidations per frame: mean {:.2}, max {}",
                frame_snapshot.invalidations_per_frame.mean(),
                frame_snapshot.invalidations_per_frame.max()
            );
        }
        self.print_foreground_work(&frame_snapshot.foreground_work);
    }

    fn print_histogram(&self, name: &str, histogram: &Histogram<u64>) {
        if histogram.is_empty() {
            return;
        }

        eprintln!("  {name}:");
        self.print_histogram_body(histogram);
    }

    fn print_foreground_work(&self, foreground_work: &DurationHistogram) {
        if foreground_work.histogram.is_empty() {
            return;
        }

        eprintln!("  foreground executor work (task polls, actions, input dispatch):");
        eprintln!("    note: excludes window draw/present, reported separately above");
        eprintln!(
            "    total: {}",
            format_duration(Duration::from_nanos(foreground_work.total_nanos))
        );
        self.print_histogram_body(&foreground_work.histogram);
    }

    fn print_histogram_body(&self, histogram: &Histogram<u64>) {
        let max_foreground_time = Duration::from_nanos(histogram.max());
        eprintln!("    samples: {}", histogram.len());
        eprintln!(
            "    mean: {}",
            format_duration(Duration::from_nanos(histogram.mean() as u64))
        );
        eprintln!(
            "    p50: {}",
            format_duration(Duration::from_nanos(histogram.value_at_quantile(0.50)))
        );
        eprintln!(
            "    p90: {}",
            format_duration(Duration::from_nanos(histogram.value_at_quantile(0.90)))
        );
        eprintln!(
            "    p95: {}",
            format_duration(Duration::from_nanos(histogram.value_at_quantile(0.95)))
        );
        eprintln!(
            "    p99: {}",
            format_duration(Duration::from_nanos(histogram.value_at_quantile(0.99)))
        );
        eprintln!("    max: {}", format_duration(max_foreground_time));
        eprintln!(
            "    frame budget overruns total: {}",
            self.total_budget_overruns(histogram)
        );
        eprintln!(
            "    frame budget overruns max: {}",
            self.budget_overruns(max_foreground_time)
        );
    }
}

struct WindowFrameSnapshot {
    whole_loop: DurationHistogram,
    draws_per_loop: Histogram<u64>,
    presentation_cadence: Histogram<u64>,
    slow_loops: Vec<SlowInterval>,
    slow_intervals: Vec<SlowInterval>,
    lost_journal_entries: u64,
    dirty_to_draw: Histogram<u64>,
    draw: Histogram<u64>,
    present_interval: Histogram<u64>,
    invalidations_per_frame: Histogram<u64>,
    foreground_work: DurationHistogram,
}

impl WindowFrameSnapshot {
    fn new() -> Self {
        Self {
            whole_loop: DurationHistogram::new(),
            draws_per_loop: Histogram::new(3).expect("valid precision"),
            presentation_cadence: Histogram::new(3).expect("valid precision"),
            slow_loops: Vec::new(),
            slow_intervals: Vec::new(),
            lost_journal_entries: 0,
            dirty_to_draw: Histogram::new(3).expect("3 significant digits is valid"),
            draw: Histogram::new(3).expect("3 significant digits is valid"),
            present_interval: Histogram::new(3).expect("3 significant digits is valid"),
            invalidations_per_frame: Histogram::new(3).expect("3 significant digits is valid"),
            foreground_work: DurationHistogram::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.dirty_to_draw.is_empty()
            && self.whole_loop.histogram.is_empty()
            && self.presentation_cadence.is_empty()
            && self.slow_intervals.is_empty()
            && self.lost_journal_entries == 0
            && self.draw.is_empty()
            && self.present_interval.is_empty()
            && self.foreground_work.histogram.is_empty()
    }
}

/// A duration histogram paired with an exact running total, since the
/// histogram's bucketed values (3 significant digits) approximate a sum less
/// precisely than tracking it directly.
struct DurationHistogram {
    histogram: Histogram<u64>,
    total_nanos: u64,
}

impl DurationHistogram {
    fn new() -> Self {
        Self {
            histogram: Histogram::new(3).expect("3 significant digits is valid"),
            total_nanos: 0,
        }
    }

    fn record(&mut self, duration: Duration) {
        let nanos = duration.as_nanos() as u64;
        // Infallible: the histogram auto-resizes.
        self.histogram.record(nanos).ok();
        self.total_nanos += nanos;
    }
}

fn format_duration(duration: Duration) -> String {
    format!("{:.3}ms", duration.as_secs_f64() * 1000.)
}

/// Aggregates every loop, retaining only the slowest spans for later attribution.
struct LoopTimings {
    histogram: DurationHistogram,
    draws: Histogram<u64>,
    slowest: Vec<(scheduler::Instant, scheduler::Instant)>,
}

impl LoopTimings {
    fn new() -> Self {
        Self {
            histogram: DurationHistogram::new(),
            draws: Histogram::new(3).expect("valid precision"),
            slowest: Vec::with_capacity(RETAINED_SLOW_INTERVALS),
        }
    }

    fn record(&mut self, start: scheduler::Instant, end: scheduler::Instant, draws_before: u64) {
        let duration = end.duration_since(start);
        self.histogram.record(duration);
        self.draws
            .record(profiler::journal::benchmark_draw_count() - draws_before)
            .ok();
        let index = self
            .slowest
            .partition_point(|&(start, end)| end.duration_since(start) >= duration);
        if index < RETAINED_SLOW_INTERVALS {
            if self.slowest.len() == RETAINED_SLOW_INTERVALS {
                self.slowest.pop();
            }
            self.slowest.insert(index, (start, end));
        }
    }
}

fn attribute_interval(
    events: &[ForegroundEvent],
    start: scheduler::Instant,
    end: scheduler::Instant,
    incomplete: bool,
) -> SlowInterval {
    use crate::profiler::journal::{FrameSnapshot, IntervalBoundary};
    let mut snapshot = FrameSnapshot {
        interval_start: start,
        boundary: IntervalBoundary::Idle { ended_at: end },
        events: Vec::new(),
        small_polls: Vec::new(),
        dropped_events: 0,
        journal_discontinuous: incomplete,
    };
    let mut result = SlowInterval {
        duration: end.duration_since(start),
        recorded_busy: Duration::ZERO,
        folded_poll_work: Duration::ZERO,
        draws: 0,
        longest_work: Duration::ZERO,
        longest_event: None,
        incomplete,
    };
    for event in events {
        let overlap_start = event.start_time().max(start);
        let overlap_end = event.end_time().min(end);
        if overlap_end <= overlap_start {
            continue;
        }
        match event {
            ForegroundEvent::SmallPolls(flush) => {
                // Folded polls can enclose nested work, so never add this
                // estimate to the individually timestamped span union.
                result.folded_poll_work += flush.summary.total.mul_f64(
                    overlap_end
                        .duration_since(overlap_start)
                        .div_duration_f64(flush.until.duration_since(flush.since)),
                );
                continue;
            }
            ForegroundEvent::Draw(_) => {
                if event.end_time() <= end {
                    result.draws += 1;
                }
            }
            _ => {}
        }
        let duration = overlap_end.duration_since(overlap_start);
        if duration > result.longest_work {
            result.longest_work = duration;
            result.longest_event = Some(*event);
        }
        snapshot.events.push(*event);
    }
    result.recorded_busy = snapshot.occupancy();
    result
}

/// Enables profiler tracing for a measurement and collects its frame events
/// and foreground journal entries.
///
/// The previous tracing state is restored on drop, so a panicking measurement
/// doesn't leave tracing enabled for unrelated code such as a later benchmark
/// in the same process.
///
/// The foreground journal collector is created at the same point, so
/// foreground work recorded before the scope starts (e.g. per-iteration
/// setup) is excluded from what [`Self::finish`] returns: a collector only
/// observes entries recorded after its creation.
struct TraceScope {
    loops: LoopTimings,
    collector: FrameTimingCollector,
    journal_collector: ForegroundJournalCollector,
    _trace_guard: profiler::TraceGuard,
}

impl TraceScope {
    fn start(journal_collector: ForegroundJournalCollector) -> Self {
        let trace_guard = profiler::trace_scope();
        Self {
            loops: LoopTimings::new(),
            collector: FrameTimingCollector::new(),
            journal_collector,
            _trace_guard: trace_guard,
        }
    }

    fn finish(mut self) -> TracedEvents {
        TracedEvents {
            loops: self.loops,
            frame_events: self.collector.collect_unseen(),
            journal_entries: self.journal_collector.collect_unseen().entries,
        }
    }
}

/// Events observed during one [`TraceScope`].
struct TracedEvents {
    loops: LoopTimings,
    frame_events: Vec<FrameEvent>,
    journal_entries: Vec<ForegroundJournalEntry>,
}

impl TracedEvents {
    /// Foreground journal entries that describe completed work (task polls,
    /// action handlers, input dispatches, draws, presents, and folded
    /// sub-floor polls), excluding interval boundaries and metadata.
    fn foreground_events(&self) -> impl Iterator<Item = &ForegroundEvent> {
        self.journal_entries.iter().filter_map(|entry| match entry {
            ForegroundJournalEntry::Event(event) => Some(event),
            _ => None,
        })
    }
}

struct MeasuredTaskInput<Input> {
    input: Input,
    trace_scope: Option<TraceScope>,
}

/// Keeps effect delivery synchronous while leaving drawing to the platform frame callback.
struct RendererScope {
    app: Rc<AppCell>,
    previous: bool,
}

impl RendererScope {
    fn start(app: &Rc<AppCell>) -> Self {
        let previous = std::mem::replace(&mut app.borrow_mut().defer_draw_until_frame, true);
        Self {
            app: app.clone(),
            previous,
        }
    }
}

impl Drop for RendererScope {
    fn drop(&mut self) {
        self.app.borrow_mut().defer_draw_until_frame = self.previous;
    }
}

struct MeasuredTaskOutput<Output> {
    trace_scope: Option<TraceScope>,
    report: BenchReport,
    _output: Output,
}

impl<Output> Drop for MeasuredTaskOutput<Output> {
    fn drop(&mut self) {
        let trace_scope = self
            .trace_scope
            .take()
            .expect("measured task output should retain its trace scope");
        let events = trace_scope.finish();
        self.report.record_trace(events);
    }
}

fn run_task_to_completion<Output>(
    foreground_executor: &ForegroundExecutor,
    task: Task<Output>,
) -> Output
where
    Output: 'static,
{
    let output = Rc::new(RefCell::new(None));
    foreground_executor
        .spawn({
            let output = output.clone();
            async move {
                *output.borrow_mut() = Some(task.await);
            }
        })
        .detach();

    foreground_executor
        .dispatcher()
        .as_threaded()
        .expect("BenchAppContext requires a ThreadedDispatcher")
        .run_until(|| output.borrow_mut().take())
}

/// A GPUI app context for Criterion benchmarks.
///
/// `BenchAppContext` is intentionally separate from `TestAppContext`: it owns a
/// benchmark app instance and exposes only the app/window operations needed by
/// benchmark setup. Criterion remains responsible for the measured loop via its
/// `Bencher` API.
#[derive(Clone)]
pub struct BenchAppContext<'a, 'measurement> {
    app: Rc<AppCell>,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    benchmark_name: Option<&'static str>,
    bencher: Rc<RefCell<Option<&'a mut criterion::Bencher<'measurement>>>>,
    report: BenchReport,
}

impl<'a, 'measurement> BenchAppContext<'a, 'measurement> {
    /// Creates a new benchmark app context backed by the provided platform.
    ///
    /// The platform's executors must be backed by a [`ThreadedDispatcher`]
    /// (see [`bench_platform`]) so the context can drain foreground work via
    /// [`Self::run_until_idle`]; panics otherwise.
    pub fn new(
        platform: Rc<dyn Platform>,
        benchmark_name: Option<&'static str>,
        bencher: &'a mut criterion::Bencher<'measurement>,
    ) -> Self {
        Self::build(platform, benchmark_name, bencher, BenchReport::default())
    }

    /// Creates a new benchmark app context backed by the provided platform.
    ///
    /// The platform's executors must be backed by a [`ThreadedDispatcher`]
    /// (see [`bench_platform`]) so the context can drain foreground work via
    /// [`Self::run_until_idle`]; panics otherwise.
    #[doc(hidden)]
    pub fn new_with_platform_and_report(
        platform: Rc<dyn Platform>,
        benchmark_name: Option<&'static str>,
        bencher: &'a mut criterion::Bencher<'measurement>,
        report: BenchReport,
    ) -> Self {
        Self::build(platform, benchmark_name, bencher, report)
    }

    fn build(
        platform: Rc<dyn Platform>,
        benchmark_name: Option<&'static str>,
        bencher: &'a mut criterion::Bencher<'measurement>,
        report: BenchReport,
    ) -> Self {
        let background_executor = platform.background_executor();
        // Validate up front so misconfiguration fails at construction with a
        // clear message instead of deep inside `run_until_idle`.
        assert!(
            background_executor.dispatcher().as_threaded().is_some(),
            "BenchAppContext requires a platform whose executors are backed by a \
             ThreadedDispatcher; construct one with gpui::bench_platform"
        );
        let foreground_executor = platform.foreground_executor();
        let asset_source = Arc::new(());
        // Benchmark setup must not make accidental network requests. The
        // production `BlockedHttpClient` reports them without enabling a
        // configurable test double through `test-support`.
        let http_client: Arc<dyn http_client::HttpClient> =
            Arc::new(http_client::BlockedHttpClient::new());
        let app = App::new_app(platform, asset_source, http_client);

        Self {
            app,
            background_executor,
            foreground_executor,
            benchmark_name,
            bencher: Rc::new(RefCell::new(Some(bencher))),
            report,
        }
    }

    /// The benchmark function name that created this context.
    pub fn benchmark_name(&self) -> Option<&'static str> {
        self.benchmark_name
    }

    /// Returns the background executor used by this benchmark app.
    pub fn background_executor(&self) -> &BackgroundExecutor {
        &self.background_executor
    }

    /// Returns the foreground executor used by this benchmark app.
    pub fn foreground_executor(&self) -> &ForegroundExecutor {
        &self.foreground_executor
    }

    /// Updates the app and flushes synchronous GPUI effects afterward.
    pub fn update<R>(&mut self, update: impl FnOnce(&mut App) -> R) -> R {
        let mut app = self.app.borrow_mut();
        app.update(update)
    }

    /// Reads app state.
    pub fn read<R>(&self, read: impl FnOnce(&App) -> R) -> R {
        let app = self.app.borrow();
        read(&app)
    }

    /// Runs queued foreground tasks on this thread and waits for in flight
    /// background work to finish. Timers that aren't due yet are not waited
    /// for (see [`ThreadedDispatcher::run_until_idle`]).
    pub fn run_until_idle(&self) {
        self.background_executor
            .dispatcher()
            .as_threaded()
            .expect("validated in BenchAppContext::build")
            .run_until_idle();
    }

    /// Alternates draining queued work with GPUI update cycles until neither
    /// makes progress, so state dropped by benchmark code is fully released.
    ///
    /// Dropped entities are released only inside an update's effect flush, and
    /// releases cascade: one flush drops the entities whose handles are gone,
    /// their drops release further handles and can queue foreground work, and
    /// a later flush collects those. Executor pumping alone never runs a
    /// flush, so without this dropped state would linger in the entity map
    /// until some woken task happened to run an update. Production gets this
    /// cadence for free from frames and input events.
    pub fn settle(&mut self) {
        let dispatcher = self.background_executor.dispatcher().clone();
        let dispatcher = dispatcher
            .as_threaded()
            .expect("validated in BenchAppContext::build");
        loop {
            self.run_until_idle();
            self.update(|_| ());
            if dispatcher.is_idle() {
                return;
            }
        }
    }

    /// Runs main-thread tasks until `ready` returns a value.
    ///
    /// Unlike [`Self::run_until_idle`], this returns as soon as `ready`
    /// reports completion, leaving any remaining queued work pending.
    pub fn run_until<R>(&self, ready: impl FnMut() -> Option<R>) -> R {
        self.background_executor
            .dispatcher()
            .as_threaded()
            .expect("validated in BenchAppContext::build")
            .run_until(ready)
    }

    /// Creates a collector observing foreground journal entries recorded
    /// from this point on, for use by a new [`TraceScope`].
    fn foreground_journal_collector(&self) -> ForegroundJournalCollector {
        self.read(|app| app.foreground_journal().collector())
    }

    /// Measures a generic benchmark workload using Criterion's iteration loop.
    ///
    /// The closure is invoked once per Criterion iteration with this
    /// benchmark app context so it can update GPUI state.
    ///
    /// Any window draws triggered by the workload are recorded into the
    /// benchmark's frame report through the GPUI frame profiler.
    pub fn bench_iter(&mut self, mut benchmark: impl FnMut(&mut Self)) {
        let bencher = self.take_bencher("bench_iter");
        let collector = TraceScope::start(self.foreground_journal_collector());
        let mut benchmark = || benchmark(self);
        bencher.iter(&mut benchmark);
        let events = collector.finish();
        self.report.record_trace(events);
        self.replace_bencher(bencher);
    }

    /// Measures a GPUI task to completion using Criterion's iteration loop.
    ///
    /// The closure is invoked once per Criterion iteration. The returned task
    /// may depend on foreground work, background work, timers, or external
    /// workers that wake GPUI tasks. Its output is dropped after the timed
    /// interval.
    ///
    /// Any window draws triggered by the task are recorded into the benchmark's
    /// frame report through the GPUI frame profiler.
    pub fn bench_task<Output>(&mut self, mut benchmark: impl FnMut(&mut Self) -> Task<Output>)
    where
        Output: 'static,
    {
        self.bench_batched_task_internal("bench_task", |_| (), |_, cx| benchmark(cx));
    }

    /// Measures a GPUI task with per-iteration setup outside the timed interval.
    ///
    /// `setup` runs before timing starts. The returned input is passed by mutable
    /// reference to `benchmark`, which returns the task whose completion is
    /// measured. Both the setup input and task output are dropped after timing
    /// stops.
    ///
    /// Each iteration is kept in its own Criterion batch so profiler tracing and
    /// destruction cannot overlap adjacent measurements.
    pub fn bench_batched_task<Input, Output>(
        &mut self,
        setup: impl FnMut(&mut Self) -> Input,
        benchmark: impl FnMut(&mut Input, &mut Self) -> Task<Output>,
    ) where
        Output: 'static,
    {
        self.bench_batched_task_internal("bench_batched_task", setup, benchmark);
    }

    fn bench_batched_task_internal<Input, Output>(
        &mut self,
        benchmark_kind: &str,
        mut setup: impl FnMut(&mut Self) -> Input,
        mut benchmark: impl FnMut(&mut Input, &mut Self) -> Task<Output>,
    ) where
        Output: 'static,
    {
        let bencher = self.take_bencher(benchmark_kind);
        let mut setup_context = self.clone();
        let mut benchmark_context = self.clone();
        let foreground_executor = self.foreground_executor.clone();
        let report = self.report.clone();

        bencher.iter_batched_ref(
            || {
                // The previous iteration's input and output were just
                // dropped; settling here releases their entities before the
                // next setup, so per-iteration state cannot accumulate
                // across a measurement.
                setup_context.settle();
                MeasuredTaskInput {
                    input: setup(&mut setup_context),
                    trace_scope: Some(TraceScope::start(
                        setup_context.foreground_journal_collector(),
                    )),
                }
            },
            |measured_input| {
                let task = benchmark(&mut measured_input.input, &mut benchmark_context);
                let output = run_task_to_completion(&foreground_executor, task);
                MeasuredTaskOutput {
                    trace_scope: measured_input.trace_scope.take(),
                    report: report.clone(),
                    _output: output,
                }
            },
            criterion::BatchSize::PerIteration,
        );
        self.replace_bencher(bencher);
    }

    /// Measures frame latency after updating a GPUI entity in its current window.
    ///
    /// Each iteration runs `update` against the entity in its current window. In
    /// renderer measurements, effects flush without drawing until the platform
    /// frame callback. The entity should be part of the window's render tree, such as the
    /// root view or a child of it.
    ///
    /// Each iteration first pumps at most the initial ready task count, yielding
    /// to input and a frame when the report's frame budget is exhausted. At least
    /// one ready task runs even if the budget is already exhausted. This is
    /// unpaced: the budget limits ready work, not the whole loop, and cannot
    /// preempt a task poll.
    ///
    /// Frame events are collected through the GPUI frame profiler
    /// ([`crate::profiler::record_frame_event`]), which is enabled for the
    /// duration of the measurement.
    pub fn bench_renderer<V>(
        &mut self,
        view: Entity<V>,
        mut update: impl FnMut(&mut V, &mut Window, &mut Context<V>),
    ) where
        V: 'static + Render,
    {
        let bencher = self.take_bencher("bench_renderer");
        let handle = self
            .with_window(view.entity_id(), |window, _| window.window_handle())
            .expect("cannot benchmark renderer for entity without a current window");

        let dispatcher = self.background_executor.dispatcher().clone();
        let mut collector = TraceScope::start(self.foreground_journal_collector());
        let _renderer_scope = RendererScope::start(&self.app);

        let mut benchmark = || {
            let draws_before = profiler::journal::benchmark_draw_count();
            let loop_start = scheduler::Instant::now();
            let turn_start = Instant::now();
            // Work already queued at frame start delays the frame in
            // production too, so run it inside the measured interval.
            dispatcher
                .as_threaded()
                .expect("validated in BenchAppContext::build")
                .run_ready_main_tasks_while(|ran_any| {
                    !ran_any || turn_start.elapsed().as_nanos() < self.report.frame_budget_nanos
                });
            self.with_window(view.entity_id(), |window, cx| {
                view.update(cx, |view, cx| update(view, window, cx));
            })
            .expect("cannot benchmark renderer for entity without a current window");
            self.request_frame(handle);
            collector
                .loops
                .record(loop_start, scheduler::Instant::now(), draws_before);
        };
        bencher.iter(&mut benchmark);

        let events = collector.finish();
        self.report.record_trace(events);
        self.replace_bencher(bencher);
    }

    /// Measures finite rendering sessions with fresh, untimed setup for each iteration.
    ///
    /// `setup` returns session state, its window, and a shared stop flag.
    /// Loop turns pump at most the initial ready foreground task count, then call
    /// `input` with a zero-based turn number unless stopped. Between polls, the
    /// stop flag and session deadline are checked; after at least one poll, the
    /// report's frame budget yields remaining work to input and a frame. This
    /// snapshots a count, not task identities. It is an unpaced benchmark policy,
    /// not OS event-loop emulation or a bound on input, draw, or whole-loop time.
    /// Once stopped, only scheduled platform frame callbacks run until the window
    /// is clean and its final changes have been submitted for presentation.
    /// Pending animation callbacks alone do not delay completion.
    /// Turns run without pacing or explicit OS-thread yields. A clean turn need not draw.
    ///
    /// Store `true` with release ordering to stop. Stopping does not imply success:
    /// validate the workload in `Input::drop` or retained fixture state afterward.
    /// Setup, session state destruction, and report aggregation are outside both
    /// timing and tracing. Own outstanding tasks in `Input` so dropping it cancels
    /// them; do not detach session work that could leak into subsequent iterations.
    ///
    /// # Panics
    ///
    /// Panics if the window is removed or stopping and final presentation exceed `timeout`.
    /// The deadline is checked between polls and frames and cannot preempt a blocking task
    /// poll, input callback, draw, or present.
    pub fn bench_renderer_session<Input>(
        &mut self,
        timeout: Duration,
        mut setup: impl FnMut(&mut Self) -> (Input, AnyWindowHandle, Arc<AtomicBool>),
        mut input: impl FnMut(&mut Input, u64, &mut Window, &mut App),
    ) {
        let bencher = self.take_bencher("bench_renderer_session");
        let mut setup_context = self.clone();
        let mut benchmark_context = self.clone();
        let dispatcher = self.background_executor.dispatcher().clone();
        let report = self.report.clone();

        bencher.iter_batched_ref(
            || {
                setup_context.settle();
                MeasuredTaskInput {
                    input: setup(&mut setup_context),
                    trace_scope: Some(TraceScope::start(
                        setup_context.foreground_journal_collector(),
                    )),
                }
            },
            |measured_input| {
                let (state, window, stopped) = &mut measured_input.input;
                let _renderer_scope = RendererScope::start(&benchmark_context.app);
                let started = Instant::now();
                let check_deadline = || {
                    assert!(
                        started.elapsed() < timeout,
                        "renderer session did not stop within {timeout:?} with final changes presented"
                    );
                };
                let mut frame = 0;
                loop {
                    check_deadline();
                    let draws_before = profiler::journal::benchmark_draw_count();
                    let loop_start = scheduler::Instant::now();
                    let turn_start = Instant::now();
                    if !stopped.load(Ordering::Acquire) {
                        dispatcher
                            .as_threaded()
                            .expect("validated in BenchAppContext::build")
                            .run_ready_main_tasks_while(|ran_any| {
                                check_deadline();
                                !stopped.load(Ordering::Acquire)
                                    && (!ran_any
                                        || turn_start.elapsed().as_nanos()
                                            < report.frame_budget_nanos)
                            });
                        check_deadline();
                        benchmark_context
                            .update_window(*window, |_, window, cx| {
                                if !stopped.load(Ordering::Acquire) {
                                    input(state, frame, window, cx);
                                }
                            })
                            .expect("renderer session window must remain open");
                    }
                    check_deadline();
                    benchmark_context.request_frame(*window);
                    check_deadline();
                    let finished = stopped.load(Ordering::Acquire) && {
                        // A scheduled callback can return early under throttling.
                        // Read without an update, which would flush unrelated effects.
                        let app = benchmark_context.app.borrow();
                        let window = app
                            .windows
                            .get(window.window_id())
                            .and_then(Option::as_deref)
                            .expect("renderer session window must remain open");
                        !window.invalidator.is_dirty() && !window.needs_present.get()
                    };
                    if !finished {
                        frame += 1;
                    }
                    measured_input
                        .trace_scope
                        .as_mut()
                        .expect("active measurement")
                        .loops
                        .record(loop_start, scheduler::Instant::now(), draws_before);
                    if finished {
                        break;
                    }
                }
                MeasuredTaskOutput {
                    trace_scope: measured_input.trace_scope.take(),
                    report: report.clone(),
                    _output: (),
                }
            },
            criterion::BatchSize::PerIteration,
        );
        self.replace_bencher(bencher);
    }

    fn request_frame(&mut self, handle: AnyWindowHandle) -> bool {
        let platform_window = {
            let mut app = self.app.borrow_mut();
            let window = app
                .windows
                .get_mut(handle.window_id())
                .and_then(Option::as_deref_mut)
                .expect("renderer window must remain open");
            window
                .platform_window
                .as_test()
                .expect("benchmark platform window")
                .clone()
        };
        // The production callback borrows App itself and may rearm its frame request.
        platform_window.simulate_scheduled_frame()
    }

    /// Adds an active window with an empty root view for benchmark setup.
    ///
    /// Activation is settled before returning so renderer measurements exercise
    /// foreground animation rather than the inactive-window frame throttle.
    pub fn add_empty_window(&mut self) -> BenchWindowContext<'a, 'measurement> {
        let bounds = {
            let app = self.app.borrow();
            Bounds::maximized(None, &app)
        };
        let window = {
            let mut app = self.app.borrow_mut();
            let window: AnyWindowHandle = app
                .open_window(
                    WindowOptions {
                        window_bounds: Some(WindowBounds::Windowed(bounds)),
                        ..Default::default()
                    },
                    |_, cx| cx.new(|_| Empty),
                )
                .expect("failed to open benchmark window")
                .into();
            // An active renderer workload must not inherit the platform
            // callback's inactive-window animation throttle.
            app.update_window(window, |_, window, _| window.activate_window())
                .expect("new benchmark window must remain open");
            window
        };

        self.run_until_idle();
        BenchWindowContext {
            cx: self.clone(),
            window,
        }
    }

    fn take_bencher(&self, benchmark_kind: &str) -> &'a mut criterion::Bencher<'measurement> {
        self.bencher.borrow_mut().take().unwrap_or_else(|| {
            panic!("cannot start {benchmark_kind}: benchmark measurement is already running")
        })
    }

    fn replace_bencher(&self, bencher: &'a mut criterion::Bencher<'measurement>) {
        let previous = self.bencher.borrow_mut().replace(bencher);
        assert!(
            previous.is_none(),
            "benchmark bencher was unexpectedly present after measurement"
        );
    }

    /// Runs GPUI benchmark teardown.
    ///
    /// Cancels any timers still armed on the shared dispatcher and drains the
    /// work that cancellation unblocks so they can't fire during a later
    /// benchmark; assumes no other `BenchAppContext` is live on this thread.
    pub fn teardown(mut self) {
        self.run_until_idle();
        self.update(|cx| {
            cx.quit();
        });
        self.run_until_idle();

        let dispatcher = self.background_executor.dispatcher();
        let dispatcher = dispatcher
            .as_threaded()
            .expect("validated in BenchAppContext::build");

        drop(self.app);
        drop(self.foreground_executor);

        for _ in 0..100 {
            if dispatcher.cancel_pending_timers() == 0 {
                return;
            }
            dispatcher.run_until_idle();
        }
        panic!(
            "benchmark teardown kept scheduling timers: {}",
            dispatcher.debug_state()
        );
    }
}

impl AppContext for BenchAppContext<'_, '_> {
    fn new<T: 'static>(&mut self, build_entity: impl FnOnce(&mut Context<T>) -> T) -> Entity<T> {
        let mut app = self.app.borrow_mut();
        app.new(build_entity)
    }

    fn reserve_entity<T: 'static>(&mut self) -> Reservation<T> {
        let mut app = self.app.borrow_mut();
        app.reserve_entity()
    }

    fn insert_entity<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_entity: impl FnOnce(&mut Context<T>) -> T,
    ) -> Entity<T> {
        let mut app = self.app.borrow_mut();
        app.insert_entity(reservation, build_entity)
    }

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Entity<T>,
        update: impl FnOnce(&mut T, &mut Context<T>) -> R,
    ) -> R {
        let mut app = self.app.borrow_mut();
        app.update_entity(handle, update)
    }

    fn as_mut<'b, T>(&'b mut self, _: &Entity<T>) -> GpuiBorrow<'b, T>
    where
        T: 'static,
    {
        panic!("Cannot use as_mut with BenchAppContext. Call update() instead.")
    }

    fn read_entity<T, R>(&self, handle: &Entity<T>, read: impl FnOnce(&T, &App) -> R) -> R
    where
        T: 'static,
    {
        let app = self.app.borrow();
        app.read_entity(handle, read)
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, update: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut Window, &mut App) -> T,
    {
        let mut app = self.app.borrow_mut();
        app.update_window(window, update)
    }

    fn with_window<R>(
        &mut self,
        entity_id: EntityId,
        update: impl FnOnce(&mut Window, &mut App) -> R,
    ) -> Option<R> {
        let mut app = self.app.borrow_mut();
        app.with_window(entity_id, update)
    }

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(Entity<T>, &App) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        let app = self.app.borrow();
        app.read_window(window, read)
    }

    fn background_spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        self.background_executor.spawn(future)
    }

    fn read_global<G, R>(&self, callback: impl FnOnce(&G, &App) -> R) -> R
    where
        G: Global,
    {
        let app = self.app.borrow();
        app.read_global(callback)
    }
}

/// A window-specific context for GPUI benchmarks.
///
/// This is separate from `VisualTestContext`; it provides access to a benchmark
/// window without exposing test-only helpers such as input simulation.
#[derive(Clone)]
pub struct BenchWindowContext<'a, 'measurement> {
    cx: BenchAppContext<'a, 'measurement>,
    window: AnyWindowHandle,
}

impl<'a, 'measurement> BenchWindowContext<'a, 'measurement> {
    /// Returns the underlying benchmark app context.
    pub fn app_context(&mut self) -> &mut BenchAppContext<'a, 'measurement> {
        &mut self.cx
    }

    /// Returns the window associated with this context.
    pub fn window_handle(&self) -> AnyWindowHandle {
        self.window
    }

    /// Runs queued foreground tasks on this thread and waits for in-flight
    /// background work to finish. Pending timers are not waited for.
    pub fn run_until_idle(&self) {
        self.cx.run_until_idle();
    }

    /// Updates the benchmark window.
    pub fn update<R>(&mut self, update: impl FnOnce(&mut Window, &mut App) -> R) -> R {
        self.cx
            .update_window(self.window, |_, window, cx| update(window, cx))
            .expect("benchmark window was unexpectedly closed")
    }
}

impl AppContext for BenchWindowContext<'_, '_> {
    fn new<T: 'static>(&mut self, build_entity: impl FnOnce(&mut Context<T>) -> T) -> Entity<T> {
        self.window
            .update(&mut self.cx, |_, _, cx| cx.new(build_entity))
            .expect("benchmark window was unexpectedly closed")
    }

    fn reserve_entity<T: 'static>(&mut self) -> Reservation<T> {
        self.cx.reserve_entity()
    }

    fn insert_entity<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_entity: impl FnOnce(&mut Context<T>) -> T,
    ) -> Entity<T> {
        self.window
            .update(&mut self.cx, |_, _, cx| {
                cx.insert_entity(reservation, build_entity)
            })
            .expect("benchmark window was unexpectedly closed")
    }

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Entity<T>,
        update: impl FnOnce(&mut T, &mut Context<T>) -> R,
    ) -> R {
        self.cx.update_entity(handle, update)
    }

    fn as_mut<'b, T>(&'b mut self, handle: &Entity<T>) -> GpuiBorrow<'b, T>
    where
        T: 'static,
    {
        self.cx.as_mut(handle)
    }

    fn read_entity<T, R>(&self, handle: &Entity<T>, read: impl FnOnce(&T, &App) -> R) -> R
    where
        T: 'static,
    {
        self.cx.read_entity(handle, read)
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, update: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut Window, &mut App) -> T,
    {
        self.cx.update_window(window, update)
    }

    fn with_window<R>(
        &mut self,
        entity_id: EntityId,
        update: impl FnOnce(&mut Window, &mut App) -> R,
    ) -> Option<R> {
        self.cx.with_window(entity_id, update)
    }

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(Entity<T>, &App) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        self.cx.read_window(window, read)
    }

    fn background_spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        self.cx.background_spawn(future)
    }

    fn read_global<G, R>(&self, callback: impl FnOnce(&G, &App) -> R) -> R
    where
        G: Global,
    {
        self.cx.read_global(callback)
    }
}

impl VisualContext for BenchWindowContext<'_, '_> {
    type Result<T> = Result<T>;

    fn window_handle(&self) -> AnyWindowHandle {
        self.window
    }

    fn update_window_entity<T: 'static, R>(
        &mut self,
        entity: &Entity<T>,
        update: impl FnOnce(&mut T, &mut Window, &mut Context<T>) -> R,
    ) -> Result<R> {
        let entity = entity.clone();
        self.cx
            .app
            .borrow_mut()
            .with_window(entity.entity_id(), |window, app| {
                entity.update(app, |entity, cx| update(entity, window, cx))
            })
            .ok_or_else(|| {
                anyhow!("entity has no current window; use `update` instead of `update_in`")
            })
    }

    fn new_window_entity<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Window, &mut Context<T>) -> T,
    ) -> Result<Entity<T>> {
        self.window.update(&mut self.cx, |_, window, cx| {
            cx.new(|cx| build_entity(window, cx))
        })
    }

    fn replace_root_view<V>(
        &mut self,
        build_view: impl FnOnce(&mut Window, &mut Context<V>) -> V,
    ) -> Result<Entity<V>>
    where
        V: 'static + Render,
    {
        self.window.update(&mut self.cx, |_, window, cx| {
            window.replace_root(cx, build_view)
        })
    }

    fn focus<V>(&mut self, entity: &Entity<V>) -> Result<()>
    where
        V: Focusable,
    {
        self.window.update(&mut self.cx, |_, window, cx| {
            entity.read(cx).focus_handle(cx).focus(window, cx)
        })
    }
}

#[cfg(test)]
mod tests {
    use std::{rc::Rc, sync::Arc};

    use super::*;
    use crate::profiler::journal::install_test_foreground_journal;

    #[test]
    #[should_panic(expected = "frame budget must be at least one nanosecond")]
    fn report_rejects_zero_frame_budget() {
        BenchReport::with_frame_budget_nanos(0);
    }

    #[test]
    #[should_panic(expected = "frame budget must be at least one nanosecond")]
    fn report_rejects_subnanosecond_frame_rate() {
        BenchReport::with_fps(1_000_000_001);
    }

    #[test]
    fn loop_counts_survive_journal_wrap_and_tracing_sessions() {
        let (journal, _guard) = install_test_foreground_journal(8, 2);
        let start = scheduler::Instant::now();
        let draw = profiler::FrameTiming {
            window_id: crate::WindowId::from(1),
            dirty_at: Some(start),
            invalidations: 1,
            draw_start: start,
            draw_end: start + Duration::from_millis(1),
        };
        for _ in 0..2 {
            let mut trace = TraceScope::start(journal.collector());
            let before = profiler::journal::benchmark_draw_count();
            for _ in 0..256 {
                profiler::journal::record_draw(draw);
            }
            trace
                .loops
                .record(start, start + Duration::from_millis(20), before);
            let report = BenchReport::default();
            report.record_trace(trace.finish());
            let snapshot = report.frame_snapshot.borrow();
            assert_eq!(snapshot.whole_loop.histogram.len(), 1);
            assert_eq!(snapshot.whole_loop.total_nanos, 20_000_000);
            assert_eq!(snapshot.draws_per_loop.min(), 256);
            assert_eq!(snapshot.draws_per_loop.max(), 256);
            assert!(snapshot.lost_journal_entries > 0);
            assert_eq!(snapshot.slow_loops.len(), 1);
            assert!(snapshot.slow_loops[0].incomplete);
        }
    }

    #[test]
    fn first_slow_loop_has_attribution_without_presentation_cadence() {
        let start = scheduler::Instant::now();
        let location = std::panic::Location::caller();
        for lost in [false, true] {
            let mut loops = LoopTimings::new();
            loops.record(
                start,
                start + Duration::from_millis(12),
                profiler::journal::benchmark_draw_count(),
            );
            let mut journal_entries = vec![
                ForegroundJournalEntry::Event(ForegroundEvent::Draw(profiler::FrameTiming {
                    window_id: crate::WindowId::from(1),
                    dirty_at: Some(start),
                    invalidations: 1,
                    draw_start: start + Duration::from_millis(8),
                    draw_end: start + Duration::from_millis(10),
                })),
                ForegroundJournalEntry::Event(ForegroundEvent::TaskPoll(profiler::TaskTiming {
                    location,
                    spawned: scheduler::SpawnTime(start),
                    start,
                    end: profiler::YieldTime(start + Duration::from_millis(11)),
                })),
                ForegroundJournalEntry::Event(ForegroundEvent::Present(profiler::PresentTiming {
                    window_id: crate::WindowId::from(1),
                    present_start: start + Duration::from_millis(11),
                    present_end: start + Duration::from_millis(12),
                    animation_interval: None,
                })),
            ];
            if lost {
                journal_entries.push(ForegroundJournalEntry::Discontinuity { lost: 1 });
            }
            let report = BenchReport::default();
            report.record_trace(TracedEvents {
                loops,
                frame_events: Vec::new(),
                journal_entries,
            });
            let snapshot = report.frame_snapshot.borrow();
            assert!(snapshot.presentation_cadence.is_empty());
            assert!(snapshot.slow_intervals.is_empty());
            assert_eq!(snapshot.slow_loops.len(), 1);
            let interval = &snapshot.slow_loops[0];
            assert_eq!(interval.duration, Duration::from_millis(12));
            assert_eq!(interval.recorded_busy, Duration::from_millis(12));
            assert_eq!(interval.draws, 1);
            assert_eq!(interval.longest_work, Duration::from_millis(11));
            assert_eq!(
                interval.longest_work_description(),
                format!("task poll at {location}")
            );
            assert_eq!(interval.incomplete, lost);
        }
    }

    #[test]
    fn slow_loop_retention_is_bounded_across_traces_without_losing_histogram_samples() {
        let start = scheduler::Instant::now();
        let report = BenchReport::default();
        for batch in 0..3 {
            let mut loops = LoopTimings::new();
            let capacity = loops.slowest.capacity();
            for milliseconds in (1..=100).rev() {
                let milliseconds = if batch == 1 {
                    101 - milliseconds
                } else {
                    milliseconds
                };
                loops.record(
                    start,
                    start + Duration::from_millis(batch * 100 + milliseconds),
                    profiler::journal::benchmark_draw_count(),
                );
                assert!(loops.slowest.len() <= RETAINED_SLOW_INTERVALS);
                assert_eq!(loops.slowest.capacity(), capacity);
            }
            assert_eq!(loops.histogram.histogram.len(), 100);
            assert_eq!(loops.draws.len(), 100);
            report.record_trace(TracedEvents {
                loops,
                frame_events: Vec::new(),
                journal_entries: Vec::new(),
            });
        }
        let snapshot = report.frame_snapshot.borrow();
        assert_eq!(snapshot.whole_loop.histogram.len(), 300);
        assert_eq!(snapshot.draws_per_loop.len(), 300);
        assert_eq!(snapshot.whole_loop.total_nanos, 45_150_000_000);
        assert_eq!(
            snapshot
                .slow_loops
                .iter()
                .map(|interval| interval.duration)
                .collect::<Vec<_>>(),
            (285..=300)
                .rev()
                .map(Duration::from_millis)
                .collect::<Vec<_>>()
        );
    }

    #[test]
    fn interval_attribution_unions_nested_and_late_completed_work() {
        use profiler::journal::{InputTiming, PollSummary, SmallPollFlush};
        let start = scheduler::Instant::now();
        let input = |from, until| {
            ForegroundEvent::Input(InputTiming {
                kind: "test",
                start: start + Duration::from_millis(from),
                end: start + Duration::from_millis(until),
                caused_invalidation: false,
            })
        };
        // The enclosing event completes after this presentation boundary and is
        // therefore recorded later. Attribution must still clip it to this interval.
        let location = std::panic::Location::caller();
        let events = [
            input(3, 5),
            ForegroundEvent::TaskPoll(profiler::TaskTiming {
                location,
                spawned: scheduler::SpawnTime(start),
                start: start + Duration::from_millis(1),
                end: profiler::YieldTime(start + Duration::from_millis(25)),
            }),
            ForegroundEvent::SmallPolls(SmallPollFlush {
                summary: PollSummary {
                    count: 2,
                    total: Duration::from_millis(1),
                },
                since: start + Duration::from_millis(3),
                until: start + Duration::from_millis(5),
            }),
        ];
        let attribution =
            attribute_interval(&events, start, start + Duration::from_millis(20), false);
        assert_eq!(attribution.recorded_busy, Duration::from_millis(19));
        assert_eq!(attribution.folded_poll_work, Duration::from_millis(1));
        assert_eq!(attribution.longest_work, Duration::from_millis(19));
        assert_eq!(
            attribution.longest_work_description(),
            format!("task poll at {location}")
        );
        assert!(!attribution.incomplete);
        assert!(
            attribute_interval(&events, start, start + Duration::from_millis(20), true).incomplete
        );
    }

    #[test]
    fn presentation_reporting_uses_non_animation_frames_and_marks_loss() {
        let start = scheduler::Instant::now();
        let presentation = |milliseconds| {
            ForegroundJournalEntry::Event(ForegroundEvent::Present(profiler::PresentTiming {
                window_id: crate::WindowId::from(1),
                present_start: start + Duration::from_millis(milliseconds),
                present_end: start + Duration::from_millis(milliseconds),
                animation_interval: None,
            }))
        };
        for lost in [false, true] {
            let mut entries = vec![presentation(0), presentation(20)];
            if lost {
                entries.insert(1, ForegroundJournalEntry::Discontinuity { lost: 3 });
            }
            let report = BenchReport::default();
            report.record_trace(TracedEvents {
                loops: LoopTimings::new(),
                frame_events: Vec::new(),
                journal_entries: entries,
            });
            let snapshot = report.frame_snapshot.borrow();
            assert_eq!(snapshot.presentation_cadence.len(), u64::from(!lost));
            assert!(
                snapshot.whole_loop.histogram.is_empty(),
                "frames alone do not turn compute work into renderer loops"
            );
            assert!(snapshot.slow_loops.is_empty());
            assert_eq!(snapshot.slow_intervals.len(), 1);
            assert_eq!(
                snapshot.slow_intervals[0].duration,
                Duration::from_millis(20)
            );
            assert_eq!(snapshot.slow_intervals[0].incomplete, lost);
        }
    }

    #[test]
    fn renderer_scope_coalesces_effects_and_preserves_synchronous_callers() {
        use std::cell::Cell;
        let platform = bench_platform(None, Arc::new(crate::NoopTextSystem::new()));
        let mut criterion = criterion::Criterion::default().without_plots();
        criterion = criterion
            .sample_size(10)
            .warm_up_time(Duration::from_millis(1))
            .measurement_time(Duration::from_millis(1));
        criterion.bench_function("renderer_effects_contract", |bencher| {
            let mut cx = BenchAppContext::new(platform.clone(), None, bencher);
            let mut window = cx.add_empty_window();
            let handle = window.window_handle();
            let state = cx.new(|_| ());
            let notifications = Rc::new(Cell::new(0));
            let subscription = cx.update(|cx| {
                cx.observe(&state, {
                    let notifications = notifications.clone();
                    move |_, _| notifications.set(notifications.get() + 1)
                })
            });
            let events = Rc::new(Cell::new(0));
            let emitter = cx.new(|_| BenchEmitter);
            let event_subscription = cx.update(|cx| {
                cx.subscribe(&emitter, {
                    let events = events.clone();
                    move |_, _, _| events.set(events.get() + 1)
                })
            });
            let trace = TraceScope::start(cx.foreground_journal_collector());
            let before = profiler::journal::benchmark_draw_count();
            {
                let _scope = RendererScope::start(&cx.app);
                for _ in 0..3 {
                    window.update(|window, cx| {
                        state.update(cx, |_, cx| cx.notify());
                        emitter.update(cx, |_, cx| cx.emit(()));
                        window.refresh();
                    });
                }
                assert_eq!(notifications.get(), 3);
                assert_eq!(events.get(), 3);
                assert_eq!(profiler::journal::benchmark_draw_count(), before);
                assert!(cx.request_frame(handle));
                assert_eq!(profiler::journal::benchmark_draw_count(), before + 1);
                window.update(|window, _| assert!(!window.needs_present.get()));
                let callbacks = Rc::new(Cell::new(0));
                window.update(|window, _| {
                    window.on_next_frame({
                        let callbacks = callbacks.clone();
                        move |window, _| {
                            callbacks.set(callbacks.get() + 1);
                            window.refresh();
                            window.on_next_frame(move |window, _| {
                                callbacks.set(callbacks.get() + 1);
                                window.refresh();
                            });
                        }
                    })
                });
                assert!(cx.request_frame(handle));
                assert_eq!(callbacks.get(), 1);
                assert!(cx.request_frame(handle));
                assert_eq!(callbacks.get(), 2);
                assert_eq!(profiler::journal::benchmark_draw_count(), before + 3);
                let special_draws_before = profiler::journal::benchmark_draw_count();
                let special_start = scheduler::Instant::now();
                window.update(|window, cx| {
                    window.refresh();
                    window.dispatch_event(
                        crate::PlatformInput::ModifiersChanged(Default::default()),
                        cx,
                    );
                    assert_eq!(
                        profiler::journal::benchmark_draw_count(),
                        special_draws_before + 1,
                        "key dispatch still needs a current dispatch tree"
                    );
                    window.refresh();
                });
                cx.request_frame(handle);
                let mut timings = LoopTimings::new();
                timings.record(
                    special_start,
                    scheduler::Instant::now(),
                    special_draws_before,
                );
                assert_eq!(
                    timings.draws.max(),
                    2,
                    "count special input draws as well as the scheduled draw"
                );
            }
            window.update(|window, _| window.refresh());
            assert_eq!(profiler::journal::benchmark_draw_count(), before + 6);
            let unwind = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let _scope = RendererScope::start(&cx.app);
                panic!("scope restoration");
            }));
            assert!(unwind.is_err());
            assert!(!cx.read(|cx| cx.defer_draw_until_frame));
            drop(trace);
            drop(subscription);
            drop(event_subscription);
            let view = window.update(|window, cx| window.replace_root(cx, |_, _| Empty));
            cx.bench_renderer(view, |_, window, _| {
                window.refresh();
                window.refresh();
            });
            assert_eq!(cx.report.frame_snapshot.borrow().draws_per_loop.max(), 1);
            assert!(cx.report.frame_snapshot.borrow().whole_loop.histogram.len() > 0);
            window.update(|window, _| window.remove_window());
            cx.teardown();
        });
    }

    #[test]
    fn renderer_session_keeps_up_with_input_generated_work() {
        use std::cell::Cell;

        let platform = bench_platform(None, Arc::new(crate::NoopTextSystem::new()));
        // Isolate the count bound from machine speed; the tiny-budget test below
        // separately exercises yielding to frames.
        let report = BenchReport::with_frame_budget_nanos(Duration::from_secs(5).as_nanos());
        let mut criterion = criterion::Criterion::default()
            .without_plots()
            .sample_size(10)
            .warm_up_time(Duration::from_millis(1))
            .measurement_time(Duration::from_millis(1));
        criterion.bench_function("renderer_input_work", |bencher| {
            let mut cx = BenchAppContext::new_with_platform_and_report(
                platform.clone(),
                None,
                bencher,
                report.clone(),
            );
            cx.bench_renderer_session(
                Duration::from_secs(5),
                |cx| {
                    let mut window = cx.add_empty_window();
                    let handle = window.window_handle();
                    let stopped = Arc::new(AtomicBool::new(false));
                    let completed = Rc::new(Cell::new(0));
                    let teardown = OnDrop({
                        let completed = completed.clone();
                        move || {
                            assert_eq!(
                                completed.get(),
                                65,
                                "stop must leave the other ready tasks unpolled"
                            );
                            window.update(|window, _| {
                                assert!(!window.invalidator.is_dirty());
                                assert!(!window.needs_present.get());
                                window.remove_window();
                            });
                        }
                    });
                    (
                        (Vec::new(), completed, stopped.clone(), teardown),
                        handle,
                        stopped,
                    )
                },
                |(tasks, completed, stopped, _), turn, window, cx| {
                    assert!(turn <= 16, "stop must skip the final input");
                    assert_eq!(
                        completed.get(),
                        turn * 4,
                        "each input's ready work must finish before the next input"
                    );
                    window.refresh();
                    tasks.clear();
                    for _ in 0..4 {
                        tasks.push(cx.spawn({
                            let completed = completed.clone();
                            let stopped = stopped.clone();
                            let handle = window.window_handle();
                            async move |cx| {
                                completed.set(completed.get() + 1);
                                if turn == 16 {
                                    cx.update_window(handle, |_, window, _| window.refresh())
                                        .expect("open window");
                                    stopped.store(true, Ordering::Release);
                                }
                            }
                        }));
                    }
                },
            );
            assert_eq!(report.frame_snapshot.borrow().draws_per_loop.max(), 1);
            cx.teardown();
        });
    }

    #[test]
    fn renderer_session_gives_frames_progress_with_a_task_backlog() {
        use std::cell::Cell;
        let platform = bench_platform(None, Arc::new(crate::NoopTextSystem::new()));
        let report = BenchReport::with_frame_budget_nanos(1);
        let mut criterion = criterion::Criterion::default()
            .without_plots()
            .sample_size(10)
            .warm_up_time(Duration::from_millis(1))
            .measurement_time(Duration::from_millis(1));
        criterion.bench_function("renderer_task_backlog", |bencher| {
            let mut cx = BenchAppContext::new_with_platform_and_report(
                platform.clone(),
                None,
                bencher,
                report.clone(),
            );
            cx.bench_renderer_session(
                Duration::from_secs(5),
                |cx| {
                    let mut window = cx.add_empty_window();
                    let handle = window.window_handle();
                    let stopped = Arc::new(AtomicBool::new(false));
                    let polls = Rc::new(Cell::new(0));
                    let tasks: Vec<_> = (0..8)
                        .map(|_| {
                            cx.update(|cx| {
                                cx.spawn({
                                    let polls = polls.clone();
                                    async move |cx| {
                                        polls.set(polls.get() + 1);
                                        for _ in 0..3 {
                                            cx.update_window(handle, |_, window, _| {
                                                window.refresh()
                                            })
                                            .expect("open window");
                                        }
                                        let start = Instant::now();
                                        while start.elapsed() < Duration::from_millis(2) {
                                            std::hint::spin_loop();
                                        }
                                    }
                                })
                            })
                        })
                        .collect();
                    let loops_before = report.frame_snapshot.borrow().whole_loop.histogram.len();
                    let teardown = OnDrop({
                        let report = report.clone();
                        move || {
                            assert_eq!(
                                polls.get(),
                                3,
                                "do not drain the backlog before input and frames"
                            );
                            let snapshot = report.frame_snapshot.borrow();
                            assert_eq!(snapshot.whole_loop.histogram.len() - loops_before, 3);
                            assert_eq!(
                                snapshot.draws_per_loop.max(),
                                1,
                                "coalesce each task's updates with input"
                            );
                            assert!(
                                snapshot.whole_loop.total_nanos
                                    >= snapshot.whole_loop.histogram.len() * 2_000_000
                            );
                            drop(snapshot);
                            window.update(|window, _| {
                                assert!(!window.needs_present.get());
                                window.remove_window();
                            });
                        }
                    });
                    ((tasks, teardown, stopped.clone()), handle, stopped)
                },
                |(_, _, stopped), turn, window, _| {
                    window.refresh();
                    if turn == 2 {
                        stopped.store(true, Ordering::Release);
                    }
                },
            );
            assert!(report.foreground_work().expect("task polls").max >= Duration::from_millis(2));
            cx.teardown();
        });
    }

    #[test]
    fn renderer_session_presents_task_completion_and_restarts_outside_tracing() {
        use futures::StreamExt;
        use std::cell::Cell;

        let platform = bench_platform(None, Arc::new(crate::NoopTextSystem::new()));
        let report = BenchReport::default();
        let sessions = Rc::new(Cell::new(0));
        let mut criterion = criterion::Criterion::default()
            .without_plots()
            .sample_size(10)
            .warm_up_time(Duration::from_millis(1))
            .measurement_time(Duration::from_millis(1));
        criterion.bench_function("renderer_session_restarts", |bencher| {
            let mut cx = BenchAppContext::new_with_platform_and_report(
                platform.clone(),
                None,
                bencher,
                report.clone(),
            );
            cx.bench_renderer_session(
                Duration::from_secs(5),
                |cx| {
                    let draws_before = report.frame_snapshot.borrow().draw.len();
                    let mut window = cx.add_empty_window();
                    let handle = window.window_handle();
                    window.update(|window, _| window.present_if_needed());
                    let stopped = Arc::new(AtomicBool::new(false));
                    let (sender, mut receiver) = futures::channel::mpsc::unbounded();
                    let task = cx.update(|cx| {
                        cx.spawn({
                            let stopped = stopped.clone();
                            async move |cx| {
                                for _ in 0..3 {
                                    receiver
                                        .next()
                                        .await
                                        .expect("input must advance each frame");
                                    cx.update_window(handle, |_, window, _| window.refresh())
                                        .expect("session window must remain open");
                                }
                                stopped.store(true, Ordering::Release);
                            }
                        })
                    });
                    let callbacks = Rc::new(Cell::new(0));
                    let teardown = OnDrop({
                        let callbacks = callbacks.clone();
                        let sessions = sessions.clone();
                        let report = report.clone();
                        move || {
                            assert_eq!(callbacks.get(), 3, "stop must skip the final input");
                            window.update(|window, _| {
                                assert!(
                                    !window.needs_present.get(),
                                    "the task's final dirty frame must be presented"
                                );
                            });
                            assert_eq!(
                                report.frame_snapshot.borrow().draw.len() - draws_before,
                                4,
                                "coalesce task and input invalidations into frame callbacks, excluding setup"
                            );
                            // A teardown draw must not enter the next session's report.
                            window.update(|window, _| window.refresh());
                            window.update(|window, _| window.remove_window());
                            sessions.set(sessions.get() + 1);
                        }
                    });
                    ((sender, callbacks, task, teardown), handle, stopped)
                },
                |(sender, callbacks, _, _), frame, window, _| {
                    assert_eq!(frame, callbacks.get());
                    callbacks.set(frame + 1);
                    window.refresh();
                    sender
                        .unbounded_send(())
                        .expect("session task must be alive");
                },
            );
            cx.teardown();
        });
        assert!(sessions.get() > 1, "Criterion must create fresh sessions");
    }

    #[test]
    fn renderer_session_presents_stopped_inactive_window_without_draining_work() {
        use std::cell::Cell;

        let platform = bench_platform(None, Arc::new(crate::NoopTextSystem::new()));
        let mut criterion = criterion::Criterion::default()
            .without_plots()
            .sample_size(10)
            .warm_up_time(Duration::from_millis(1))
            .measurement_time(Duration::from_millis(1));
        criterion.bench_function("renderer_session_inactive_stop", |bencher| {
            let mut cx = BenchAppContext::new(platform.clone(), None, bencher);
            cx.bench_renderer_session(
                Duration::from_secs(5),
                |cx| {
                    let mut window = cx.add_empty_window();
                    let handle = window.window_handle();
                    let _scope = RendererScope::start(&cx.app);
                    window.update(|window, _| window.refresh());
                    assert!(cx.request_frame(handle));
                    let presented = Instant::now();
                    let platform_window = window.update(|window, _| {
                        assert!(!window.invalidator.is_dirty());
                        assert!(!window.needs_present.get());
                        window.platform_window.as_test().unwrap().clone()
                    });
                    platform_window.simulate_active_status_change(false);
                    let stopped = Arc::new(AtomicBool::new(false));
                    let callbacks = Rc::new(Cell::new(0));
                    let task_ran = Rc::new(Cell::new(false));
                    let draws_before = profiler::journal::benchmark_draw_count();
                    let teardown = OnDrop({
                        let callbacks = callbacks.clone();
                        let task_ran = task_ran.clone();
                        move || {
                            assert!(!task_ran.get(), "stopped sessions must not pump tasks");
                            assert_eq!(callbacks.get(), 1);
                            assert_eq!(
                                profiler::journal::benchmark_draw_count(),
                                draws_before + 1,
                                "the final refresh must draw despite throttling"
                            );
                            window.update(|window, _| {
                                assert!(!window.is_window_active());
                                assert!(!window.invalidator.is_dirty());
                                assert!(!window.needs_present.get());
                                assert_eq!(window.next_frame_callbacks.borrow().len(), 1);
                                window.remove_window();
                            });
                        }
                    });
                    (
                        (
                            stopped.clone(),
                            callbacks,
                            task_ran,
                            None,
                            presented,
                            teardown,
                        ),
                        handle,
                        stopped,
                    )
                },
                |(stopped, callbacks, task_ran, task, presented, _), turn, window, cx| {
                    assert_eq!(turn, 0, "no input may run after stop");
                    assert!(!window.is_window_active());
                    window.refresh();
                    window.on_next_frame({
                        let callbacks = callbacks.clone();
                        move |window, _| {
                            callbacks.set(callbacks.get() + 1);
                            window.on_next_frame(|_, _| {
                                panic!("future animation must not delay completion");
                            });
                        }
                    });
                    *task = Some(cx.foreground_executor().spawn({
                        let task_ran = task_ran.clone();
                        async move { task_ran.set(true) }
                    }));
                    assert!(
                        presented.elapsed() < Duration::from_micros(33_333),
                        "stop must occur before the inactive throttle interval"
                    );
                    stopped.store(true, Ordering::Release);
                },
            );
            cx.teardown();
        });
    }

    #[test]
    #[should_panic(expected = "renderer session did not stop within")]
    fn renderer_session_bounds_an_unset_stop_flag() {
        let platform = bench_platform(None, Arc::new(crate::NoopTextSystem::new()));
        let mut criterion = criterion::Criterion::default().without_plots();
        criterion.bench_function("renderer_session_timeout", |bencher| {
            let mut cx = BenchAppContext::new(platform.clone(), None, bencher);
            cx.bench_renderer_session(
                Duration::ZERO,
                |cx| {
                    let window = cx.add_empty_window();
                    ((), window.window_handle(), Arc::new(AtomicBool::new(false)))
                },
                |_, _, window, _| window.refresh(),
            );
        });
    }

    #[test]
    fn renderer_session_rejects_overlong_stopping_callbacks() {
        use std::cell::Cell;

        for stop_in_frame in [false, true] {
            let callback_ran = Rc::new(Cell::new(false));
            let session_dropped = Rc::new(Cell::new(false));
            let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                let platform = bench_platform(None, Arc::new(crate::NoopTextSystem::new()));
                let mut criterion = criterion::Criterion::default().without_plots();
                criterion.bench_function("renderer_session_overlong_stop", |bencher| {
                    let mut cx = BenchAppContext::new(platform.clone(), None, bencher);
                    cx.bench_renderer_session(
                        Duration::from_secs(1),
                        |cx| {
                            assert!(
                                !callback_ran.get(),
                                "overlong stopping callback must not complete successfully"
                            );
                            let mut window = cx.add_empty_window();
                            let handle = window.window_handle();
                            let stopped = Arc::new(AtomicBool::new(false));
                            let teardown = OnDrop({
                                let session_dropped = session_dropped.clone();
                                move || {
                                    window.update(|window, _| window.remove_window());
                                    session_dropped.set(true);
                                }
                            });
                            ((stopped.clone(), teardown), handle, stopped)
                        },
                        |(stopped, _), turn, window, _| {
                            assert_eq!(turn, 0);
                            let stop = {
                                let callback_ran = callback_ran.clone();
                                let stopped = stopped.clone();
                                move || {
                                    callback_ran.set(true);
                                    // Setup is untimed; only this final callback exceeds the deadline.
                                    std::thread::sleep(Duration::from_millis(1100));
                                    stopped.store(true, Ordering::Release);
                                }
                            };
                            if stop_in_frame {
                                window.on_next_frame(move |_, _| stop());
                            } else {
                                stop();
                            }
                        },
                    );
                    panic!("overlong stopping callback must not complete successfully");
                });
            }));
            assert!(callback_ran.get(), "must reach the final stopping callback");
            assert!(session_dropped.get(), "timeout must drop session state");
            let panic = result.expect_err("overlong session must panic");
            let message = panic
                .downcast_ref::<String>()
                .map(String::as_str)
                .or_else(|| panic.downcast_ref::<&str>().copied())
                .expect("timeout panic must have a message");
            assert!(
                message.contains("renderer session did not stop within"),
                "unexpected panic: {message}"
            );
        }
    }

    #[test]
    fn renderer_session_presents_input_stop_and_cancels_pending_owned_work() {
        use std::cell::Cell;

        let platform = bench_platform(None, Arc::new(crate::NoopTextSystem::new()));
        let report = BenchReport::default();
        let sessions = Rc::new(Cell::new(0));
        let cancelled = Rc::new(Cell::new(0));
        let resumed = Rc::new(Cell::new(0));
        let mut criterion = criterion::Criterion::default()
            .without_plots()
            .sample_size(10)
            .warm_up_time(Duration::from_millis(1))
            .measurement_time(Duration::from_millis(1));
        criterion.bench_function("renderer_session_input_stop", |bencher| {
            let mut cx = BenchAppContext::new_with_platform_and_report(
                platform.clone(),
                None,
                bencher,
                report.clone(),
            );
            cx.bench_renderer_session(
                Duration::from_secs(5),
                |cx| {
                    assert_eq!(cancelled.get(), sessions.get());
                    assert_eq!(resumed.get(), 0, "previous session work must not resume");
                    let draws_before = report.frame_snapshot.borrow().draw.len();
                    let mut window = cx.add_empty_window();
                    let handle = window.window_handle();
                    window.update(|window, _| window.present_if_needed());
                    let stopped = Arc::new(AtomicBool::new(false));
                    let started = Rc::new(Cell::new(false));
                    let (sender, receiver) = futures::channel::oneshot::channel();
                    let task = cx.foreground_executor().spawn({
                        let started = started.clone();
                        let cancelled = cancelled.clone();
                        let resumed = resumed.clone();
                        async move {
                            let _on_drop = OnDrop(|| cancelled.set(cancelled.get() + 1));
                            started.set(true);
                            receiver.await.expect("input must wake the pending task");
                            resumed.set(resumed.get() + 1);
                        }
                    });
                    let teardown = OnDrop({
                        let report = report.clone();
                        let sessions = sessions.clone();
                        move || {
                            window.update(|window, _| {
                                assert!(
                                    !window.needs_present.get(),
                                    "input's final dirty frame must be presented"
                                );
                            });
                            assert_eq!(report.frame_snapshot.borrow().draw.len() - draws_before, 1);
                            window.update(|window, _| window.remove_window());
                            sessions.set(sessions.get() + 1);
                        }
                    });
                    (
                        (Some(sender), started, stopped.clone(), task, teardown),
                        handle,
                        stopped,
                    )
                },
                |(sender, started, stopped, _, _), frame, window, _| {
                    assert_eq!(frame, 0, "stopping input must not be called again");
                    assert!(started.get(), "the owned task must already be pending");
                    // Make the unfinished task runnable: without cancellation it
                    // would resume when the next session's setup settles work.
                    sender
                        .take()
                        .expect("only one input")
                        .send(())
                        .expect("pending receiver");
                    window.refresh();
                    window.on_next_frame({
                        let stopped = stopped.clone();
                        move |window, _| {
                            window.refresh();
                            stopped.store(true, Ordering::Release);
                        }
                    });
                },
            );
            cx.teardown();
        });
        assert!(sessions.get() > 1);
        assert_eq!(cancelled.get(), sessions.get());
        assert_eq!(
            resumed.get(),
            0,
            "no cancelled task may resume, including the last"
        );
    }

    #[test]
    fn foreground_work_reports_long_task_without_window_draw() {
        let (journal, _journal_guard) = install_test_foreground_journal(1024, 64);
        let dispatcher = Arc::new(ThreadedDispatcher::new());
        let foreground_executor = ForegroundExecutor::new(dispatcher);

        let trace_scope = TraceScope::start(journal.collector());

        // A single foreground task poll that never touches a window, akin
        // to the stall a debounced background computation can cause.
        let task = foreground_executor.spawn(async move {
            std::thread::sleep(Duration::from_millis(60));
        });
        run_task_to_completion(&foreground_executor, task);

        let events = trace_scope.finish();
        assert!(
            events.frame_events.is_empty(),
            "no window was involved, so no frame events should be recorded"
        );

        let report = BenchReport::default();
        report.record_foreground_events(events.foreground_events());

        let summary = report
            .foreground_work()
            .expect("a long task poll should be reported even without a window draw");
        // The spawned task's own poll is one sample; the tiny wrapper poll
        // that observes its completion in `run_task_to_completion` folds
        // into a second, near-zero sample rather than being dropped.
        assert!(summary.count >= 1, "expected at least one recorded item");
        assert!(
            summary.max >= Duration::from_millis(55),
            "expected the long poll's duration to be recorded, got {:?}",
            summary.max
        );
        // `total` is an exact sum, while `max` may be rounded up to its
        // histogram bucket's boundary, so compare each against the expected
        // floor directly instead of against each other.
        assert!(
            summary.total >= Duration::from_millis(55),
            "expected the long poll's duration to be included in the total, got {:?}",
            summary.total
        );
    }

    #[test]
    fn foreground_work_excludes_setup_before_trace_scope_starts() {
        let (journal, _journal_guard) = install_test_foreground_journal(1024, 64);
        let dispatcher = Arc::new(ThreadedDispatcher::new());
        let foreground_executor = ForegroundExecutor::new(dispatcher);

        // Fixture/setup work that must not be attributed to the measurement:
        // a long poll recorded before the trace scope (and its journal
        // collector) is created.
        let setup_task = foreground_executor.spawn(async move {
            std::thread::sleep(Duration::from_millis(80));
        });
        run_task_to_completion(&foreground_executor, setup_task);

        let trace_scope = TraceScope::start(journal.collector());

        let measured_task = foreground_executor.spawn(async move {
            std::thread::sleep(Duration::from_millis(10));
        });
        run_task_to_completion(&foreground_executor, measured_task);

        let events = trace_scope.finish();
        let report = BenchReport::default();
        report.record_foreground_events(events.foreground_events());

        let summary = report
            .foreground_work()
            .expect("the measured task's poll should be reported");
        assert!(
            summary.max < Duration::from_millis(40),
            "setup work's 80ms poll must not leak into the measured summary, got {:?}",
            summary.max
        );
        assert!(
            summary.total < Duration::from_millis(40),
            "setup work's 80ms poll must not leak into the measured total, got {:?}",
            summary.total
        );
    }

    #[test]
    fn bench_task_reports_long_task_without_window() {
        let platform = bench_platform(None, Arc::new(crate::NoopTextSystem::new()));
        let report = BenchReport::default();
        let name = "bench_task_reports_long_task_without_window";

        let mut criterion = criterion::Criterion::default()
            .without_plots()
            .sample_size(10)
            .warm_up_time(Duration::from_millis(1))
            .measurement_time(Duration::from_millis(1));

        criterion.bench_function(name, |bencher| {
            let mut cx = BenchAppContext::new_with_platform_and_report(
                platform.clone(),
                Some(name),
                bencher,
                report.clone(),
            );
            cx.bench_task(|cx| {
                cx.foreground_executor().spawn(async move {
                    std::thread::sleep(Duration::from_millis(20));
                })
            });
            cx.teardown();
        });

        let summary = report
            .foreground_work()
            .expect("bench_task should report foreground work with no window involved");
        assert!(
            summary.max >= Duration::from_millis(15),
            "expected a ~20ms task poll to be recorded, got {:?}",
            summary.max
        );
    }

    #[test]
    fn task_completion_supports_non_send_foreground_output() {
        let dispatcher = Arc::new(ThreadedDispatcher::new());
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher);
        let (sender, receiver) = futures::channel::oneshot::channel();

        background_executor
            .spawn(async move {
                sender
                    .send(())
                    .expect("foreground receiver should remain alive");
            })
            .detach();
        let expected_output = Rc::new(42);
        let task_output = expected_output.clone();
        let task = foreground_executor.spawn(async move {
            receiver.await.expect("background task should send a value");
            task_output
        });

        let output = run_task_to_completion(&foreground_executor, task);
        assert!(
            Rc::ptr_eq(&output, &expected_output),
            "task runner should preserve non-Send foreground output"
        );
    }

    struct BenchEmitter;

    impl crate::EventEmitter<()> for BenchEmitter {}

    struct OnDrop<F: FnMut()>(F);

    impl<F: FnMut()> Drop for OnDrop<F> {
        fn drop(&mut self) {
            (self.0)();
        }
    }
}
