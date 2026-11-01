use std::{
    cell::{OnceCell, RefCell},
    future::Future,
    rc::Rc,
    sync::{
        Arc,
        atomic::{AtomicBool, Ordering},
    },
    time::{Duration, Instant},
};

use anyhow::{Result, anyhow};
pub use bench_metrics::{BenchMeasurement, CountingAllocator, MetricReport};
use gpui_util::ResultExt as _;
use hdrhistogram::Histogram;

use crate::{
    AnyView, AnyWindowHandle, App, AppCell, AppContext, BackgroundExecutor, Bounds, Context, Empty,
    Entity, EntityId, Focusable, ForegroundExecutor, Global, GpuMemoryStats, GpuStats, Pixels,
    Platform, PlatformHeadlessRenderer, PlatformTextSystem, Render, Reservation, Size, Task,
    TestPlatform, TestWindow, ThreadedDispatcher, VisualContext, Window, WindowBounds,
    WindowHandle, WindowOptions,
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
    headless_renderer_factory: Option<
        Box<dyn Fn() -> anyhow::Result<Option<Box<dyn PlatformHeadlessRenderer>>>>,
    >,
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

fn gpu_metrics_enabled_from_env() -> Result<bool> {
    let setting = std::env::var("GPUI_BENCH_GPU_METRICS");
    let setting = match &setting {
        Ok(value) => Some(value.as_str()),
        Err(std::env::VarError::NotPresent) => None,
        Err(error) => {
            return Err(anyhow!(
                "GPUI_BENCH_GPU_METRICS is not valid Unicode: {error}"
            ));
        }
    };
    let wall_time_only = std::env::var(bench_metrics::BENCH_MEASUREMENT_ENV_VAR)
        .is_ok_and(|value| value == "wall-time");
    gpu_metrics_enabled(setting, wall_time_only)
}

fn gpu_metrics_enabled(setting: Option<&str>, wall_time_only: bool) -> Result<bool> {
    let enabled = match setting {
        None | Some("on" | "1" | "true") => true,
        Some("off" | "0" | "false") => false,
        Some(value) => anyhow::bail!(
            "unsupported GPUI_BENCH_GPU_METRICS value {value:?}; expected \
             \"on\", \"1\", \"true\", \"off\", \"0\", or \"false\""
        ),
    };
    Ok(enabled && !wall_time_only)
}

/// Distribution of available GPU duration samples (histogram precision: three digits).
#[derive(Clone, Copy, Debug)]
pub struct GpuDurationSummary {
    /// Number of frames with this duration available.
    pub count: u64,
    /// Median duration.
    pub p50: Duration,
    /// 90th percentile duration.
    pub p90: Duration,
    /// 95th percentile duration.
    pub p95: Duration,
    /// 99th percentile duration.
    pub p99: Duration,
    /// Maximum duration.
    pub max: Duration,
}

impl GpuDurationSummary {
    fn from_histogram(histogram: &Histogram<u64>) -> Option<Self> {
        (!histogram.is_empty()).then(|| Self {
            count: histogram.len(),
            p50: Duration::from_nanos(histogram.value_at_quantile(0.50)),
            p90: Duration::from_nanos(histogram.value_at_quantile(0.90)),
            p95: Duration::from_nanos(histogram.value_at_quantile(0.95)),
            p99: Duration::from_nanos(histogram.value_at_quantile(0.99)),
            max: Duration::from_nanos(histogram.max()),
        })
    }
}

/// A per-frame counter, including zero-valued frames in its mean.
#[derive(Clone, Copy, Debug, Default)]
pub struct GpuCounterSummary {
    /// Exact sum across recorded frames.
    pub total: u128,
    /// Mean per frame.
    pub mean: f64,
    /// Maximum in any one frame.
    pub max: u64,
}

impl GpuCounterSummary {
    fn record(&mut self, value: u64, frames: u64) {
        self.total += u128::from(value);
        self.max = self.max.max(value);
        self.mean = self.total as f64 / frames as f64;
    }
}

/// GPU results across all measured iterations, including Criterion warmup/calibration.
#[derive(Clone, Debug, Default)]
pub struct GpuSummary {
    /// Number of submitted frames recorded.
    pub frames: u64,
    /// Execution-time distribution, or `None` when no frame could be timed.
    pub gpu_duration: Option<GpuDurationSummary>,
    /// CPU-observed submission-to-completion distribution.
    pub submit_to_complete: Option<GpuDurationSummary>,
    /// Number of timed frames strictly exceeding the configured frame budget.
    pub frames_over_budget: u64,
    /// Maximum outstanding submissions in any renderer, including the submitted frame.
    pub max_frames_in_flight: u32,
    /// Instance upload bytes per frame.
    pub instance_bytes: GpuCounterSummary,
    /// Atlas upload bytes per frame.
    pub atlas_upload_bytes: GpuCounterSummary,
    /// Uniform upload bytes per frame.
    pub uniform_bytes: GpuCounterSummary,
    /// Atlas uploads per frame.
    pub atlas_uploads: GpuCounterSummary,
    /// Draw calls per frame.
    pub draw_calls: GpuCounterSummary,
    /// Render passes per frame.
    pub render_passes: GpuCounterSummary,
    /// GPU resource creations per frame.
    pub resource_creations: GpuCounterSummary,
    /// Newly created GPU resource bytes per frame.
    pub resource_created_bytes: GpuCounterSummary,
    /// Combined owned memory before the first measured interval. Driver totals are
    /// preserved only for a single renderer, since drivers may report device-wide totals.
    pub memory_start: Option<GpuMemoryStats>,
    /// Combined owned memory after the last measured interval.
    pub memory_end: Option<GpuMemoryStats>,
    /// Maximum of the sums of renderer high-water marks per measured interval.
    /// With multiple renderers this is an upper bound, not a simultaneous peak.
    pub peak_owned_bytes: Option<u64>,
    /// Signed change in owned bytes from `memory_start` to `memory_end`.
    /// `None` if either endpoint is unavailable.
    pub owned_bytes_growth: Option<i128>,
}

struct GpuSnapshot {
    summary: GpuSummary,
    gpu_duration: DurationHistogram,
    submit_to_complete: DurationHistogram,
    recorded: bool,
}

impl GpuSnapshot {
    fn new() -> Self {
        Self {
            summary: GpuSummary::default(),
            gpu_duration: DurationHistogram::new(),
            submit_to_complete: DurationHistogram::new(),
            recorded: false,
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
    gpu_snapshot: Rc<RefCell<GpuSnapshot>>,
    metrics: MetricReport,
    frame_budget_nanos: u128,
}

impl Default for BenchReport {
    fn default() -> Self {
        Self::with_fps(DEFAULT_FPS)
    }
}

impl BenchReport {
    /// Creates a report whose per-frame budget is one frame at `fps` when
    /// counting frame budget overruns.
    pub fn with_fps(fps: u64) -> Self {
        assert!(fps > 0, "frame rate must be greater than zero");
        Self::with_frame_budget_nanos(NANOS_PER_SECOND / fps as u128)
    }

    /// Creates a report that treats `frame_budget_nanos` as the per-frame budget
    /// when counting frame budget overruns.
    pub fn with_frame_budget_nanos(frame_budget_nanos: u128) -> Self {
        assert!(
            frame_budget_nanos > 0,
            "frame budget must be at least one nanosecond"
        );
        Self {
            frame_snapshot: Rc::new(RefCell::new(WindowFrameSnapshot::new())),
            gpu_snapshot: Rc::new(RefCell::new(GpuSnapshot::new())),
            metrics: MetricReport::new(),
            frame_budget_nanos,
        }
    }

    /// Returns the per-iteration secondary metrics (retired instructions,
    /// cycles, context switches, ...) recorded alongside Criterion's primary
    /// measurement.
    pub fn metrics(&self) -> &MetricReport {
        &self.metrics
    }

    /// Returns GPU results, or `None` when no renderer supplied statistics.
    pub fn gpu(&self) -> Option<GpuSummary> {
        let snapshot = self.gpu_snapshot.borrow();
        if !snapshot.recorded {
            return None;
        }
        let mut summary = snapshot.summary.clone();
        summary.gpu_duration = GpuDurationSummary::from_histogram(&snapshot.gpu_duration.histogram);
        summary.submit_to_complete =
            GpuDurationSummary::from_histogram(&snapshot.submit_to_complete.histogram);
        summary.owned_bytes_growth = summary
            .memory_start
            .zip(summary.memory_end)
            .map(|(start, end)| i128::from(end.owned_bytes()) - i128::from(start.owned_bytes()));
        Some(summary)
    }

    fn record_gpu_stats(&self, start: Option<GpuMemoryStats>, stats: GpuStats) {
        let mut snapshot = self.gpu_snapshot.borrow_mut();
        if !snapshot.recorded {
            snapshot.summary.memory_start = start;
        }
        snapshot.recorded = true;
        snapshot.summary.memory_end = stats.memory;
        if let Some(peak) = stats.peak_owned_bytes {
            snapshot.summary.peak_owned_bytes =
                Some(snapshot.summary.peak_owned_bytes.unwrap_or(0).max(peak));
        }
        for frame in stats.frames {
            if let Some(duration) = frame.gpu_duration {
                snapshot.gpu_duration.record(duration);
                if duration.as_nanos() > self.frame_budget_nanos {
                    snapshot.summary.frames_over_budget += 1;
                }
            }
            if let Some(duration) = frame.submit_to_complete {
                snapshot.submit_to_complete.record(duration);
            }
            let summary = &mut snapshot.summary;
            summary.frames += 1;
            summary.max_frames_in_flight = summary.max_frames_in_flight.max(frame.frames_in_flight);
            summary
                .instance_bytes
                .record(frame.instance_bytes, summary.frames);
            summary
                .atlas_upload_bytes
                .record(frame.atlas_upload_bytes, summary.frames);
            summary
                .uniform_bytes
                .record(frame.uniform_bytes, summary.frames);
            summary
                .atlas_uploads
                .record(frame.atlas_uploads.into(), summary.frames);
            summary
                .draw_calls
                .record(frame.draw_calls.into(), summary.frames);
            summary
                .render_passes
                .record(frame.render_passes.into(), summary.frames);
            summary
                .resource_creations
                .record(frame.resource_creations.into(), summary.frames);
            summary
                .resource_created_bytes
                .record(frame.resource_created_bytes, summary.frames);
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
    pub fn print(&self, benchmark_name: &str) {
        let frame_snapshot = self.frame_snapshot.borrow();
        if frame_snapshot.is_empty() && self.metrics.is_empty() && self.gpu().is_none() {
            return;
        }

        eprintln!("GPUI bench report (all observed iterations): {benchmark_name}");
        eprintln!("  note: includes Criterion warmup/calibration");
        self.metrics.print("  ");
        self.print_histogram("window dirty-to-draw", &frame_snapshot.dirty_to_draw);
        self.print_histogram("window draw", &frame_snapshot.draw);
        self.print_histogram("window present interval", &frame_snapshot.present_interval);
        if !frame_snapshot.invalidations_per_frame.is_empty() {
            eprintln!(
                "  invalidations per frame: mean {:.2}, max {}",
                frame_snapshot.invalidations_per_frame.mean(),
                frame_snapshot.invalidations_per_frame.max()
            );
        }
        self.print_foreground_work(&frame_snapshot.foreground_work);
        self.print_gpu();
    }

    fn print_gpu(&self) {
        let Some(summary) = self.gpu() else { return };
        let snapshot = self.gpu_snapshot.borrow();
        eprintln!("  GPU:");
        eprintln!("    frames: {}", summary.frames);
        if summary.frames > 0 && summary.gpu_duration.is_none() {
            eprintln!("    GPU timing unavailable");
        }
        self.print_histogram("GPU execution time", &snapshot.gpu_duration.histogram);
        self.print_histogram(
            "GPU submit-to-complete",
            &snapshot.submit_to_complete.histogram,
        );
        eprintln!(
            "    GPU-timed frames over budget: {}",
            summary.frames_over_budget
        );
        eprintln!("    max frames in flight: {}", summary.max_frames_in_flight);
        for (name, counter) in [
            ("instance bytes", summary.instance_bytes),
            ("atlas upload bytes", summary.atlas_upload_bytes),
            ("uniform bytes", summary.uniform_bytes),
            ("atlas uploads", summary.atlas_uploads),
            ("draw calls", summary.draw_calls),
            ("render passes", summary.render_passes),
            ("resource creations", summary.resource_creations),
            ("resource created bytes", summary.resource_created_bytes),
        ] {
            eprintln!(
                "    {name} per frame: mean {:.2}, max {}",
                counter.mean, counter.max
            );
        }
        if let Some(memory) = summary.memory_end {
            eprintln!(
                "    memory at end: {} owned bytes (atlas {}, instances {}, intermediate {}, target {})",
                memory.owned_bytes(),
                memory.atlas_bytes,
                memory.instance_buffer_bytes,
                memory.intermediate_texture_bytes,
                memory.render_target_bytes
            );
            if let Some(driver) = memory.driver_allocated_bytes {
                eprintln!("    driver allocated bytes: {driver}");
            }
        }
        if let Some(peak) = summary.peak_owned_bytes {
            eprintln!(
                "    peak owned bytes: {peak} (sum of renderer peaks; upper bound for multiple windows)"
            );
        }
        if let Some(growth) = summary.owned_bytes_growth {
            eprintln!("    owned bytes growth (first to last snapshot): {growth:+}");
        }
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
    dirty_to_draw: Histogram<u64>,
    draw: Histogram<u64>,
    present_interval: Histogram<u64>,
    invalidations_per_frame: Histogram<u64>,
    foreground_work: DurationHistogram,
}

impl WindowFrameSnapshot {
    fn new() -> Self {
        Self {
            dirty_to_draw: Histogram::new(3).expect("3 significant digits is valid"),
            draw: Histogram::new(3).expect("3 significant digits is valid"),
            present_interval: Histogram::new(3).expect("3 significant digits is valid"),
            invalidations_per_frame: Histogram::new(3).expect("3 significant digits is valid"),
            foreground_work: DurationHistogram::new(),
        }
    }

    fn is_empty(&self) -> bool {
        self.dirty_to_draw.is_empty()
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
    collector: FrameTimingCollector,
    journal_collector: ForegroundJournalCollector,
    _trace_guard: profiler::TraceGuard,
}

impl TraceScope {
    fn start(journal_collector: ForegroundJournalCollector) -> Self {
        let trace_guard = profiler::trace_scope();
        Self {
            collector: FrameTimingCollector::new(),
            journal_collector,
            _trace_guard: trace_guard,
        }
    }

    fn finish(mut self) -> TracedEvents {
        TracedEvents {
            frame_events: self.collector.collect_unseen(),
            journal_entries: self.journal_collector.collect_unseen().entries,
        }
    }
}

/// Events observed during one [`TraceScope`].
struct TracedEvents {
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

struct GpuMeasurement {
    windows: Vec<TestWindow>,
    memory_start: Option<GpuMemoryStats>,
}

#[derive(Clone)]
pub(super) struct BenchGpuMetrics {
    enabled: bool,
    measurement: Rc<RefCell<Option<GpuMeasurement>>>,
}

impl BenchGpuMetrics {
    pub(super) fn window_opened(&self, window: &mut Window) {
        if let Some(window) = window.platform_window.as_test() {
            window.set_gpu_metrics_enabled(self.enabled);
            if let Some(measurement) = self.measurement.borrow_mut().as_mut() {
                measurement.include(window);
            }
        }
    }
}

fn drain_gpu_stats(window: &TestWindow) -> Result<Option<GpuStats>> {
    window.wait_for_gpu_idle()?;
    Ok(window.take_gpu_stats())
}

fn combine_gpu_stats(stats: impl IntoIterator<Item = GpuStats>) -> Option<GpuStats> {
    let mut stats = stats.into_iter();
    let mut combined = stats.next()?;
    for stats in stats {
        combined.frames.extend(stats.frames);
        combined.memory = combined
            .memory
            .zip(stats.memory)
            .map(|(left, right)| GpuMemoryStats {
                atlas_bytes: left.atlas_bytes + right.atlas_bytes,
                instance_buffer_bytes: left.instance_buffer_bytes + right.instance_buffer_bytes,
                intermediate_texture_bytes: left.intermediate_texture_bytes
                    + right.intermediate_texture_bytes,
                render_target_bytes: left.render_target_bytes + right.render_target_bytes,
                // Device-wide driver totals cannot be summed without double counting.
                driver_allocated_bytes: None,
            });
        combined.peak_owned_bytes = combined
            .peak_owned_bytes
            .zip(stats.peak_owned_bytes)
            .map(|(left, right)| left + right);
    }
    Some(combined)
}

impl GpuMeasurement {
    fn include(&mut self, window: &TestWindow) {
        if !self
            .windows
            .iter()
            .any(|existing| Rc::ptr_eq(&existing.0, &window.0))
        {
            // Retain the renderer until the untimed drain even if its window closes.
            self.windows.push(window.clone());
        }
    }

    fn finish(self, report: &BenchReport) {
        if let Some(stats) = combine_gpu_stats(
            self.windows
                .iter()
                .filter_map(|window| drain_gpu_stats(window).log_err().flatten()),
        ) {
            report.record_gpu_stats(self.memory_start, stats);
        }
    }
}

struct MeasuredTaskOutput<Output> {
    trace_scope: Option<TraceScope>,
    report: BenchReport,
    gpu_measurement: Rc<RefCell<Option<GpuMeasurement>>>,
    _output: Output,
}

impl<Output> Drop for MeasuredTaskOutput<Output> {
    fn drop(&mut self) {
        let trace_scope = self
            .trace_scope
            .take()
            .expect("measured task output should retain its trace scope");
        let events = trace_scope.finish();
        self.report.record_frame_timings(events.frame_events.iter());
        self.report
            .record_foreground_events(events.foreground_events());
        if let Some(measurement) = self.gpu_measurement.borrow_mut().take() {
            measurement.finish(&self.report);
        }
    }
}

#[cfg(test)]
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
///
/// GPU metrics default to enabled. Set `GPUI_BENCH_GPU_METRICS` to `off`, `0`,
/// or `false` to disable them (`on`, `1`, and `true` explicitly enable them).
/// `BENCH_MEASUREMENT=wall-time` always disables GPU metrics as well as CPU
/// counters. Unsupported GPU renderers contribute no GPU report section.
/// GPU waits and report aggregation run outside measured intervals.
#[derive(Clone)]
pub struct BenchAppContext<'a, 'measurement> {
    app: Rc<AppCell>,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    benchmark_name: Option<&'static str>,
    bencher: Rc<RefCell<Option<&'a mut criterion::Bencher<'measurement, BenchMeasurement>>>>,
    report: BenchReport,
    gpu_metrics: BenchGpuMetrics,
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
        bencher: &'a mut criterion::Bencher<'measurement, BenchMeasurement>,
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
        bencher: &'a mut criterion::Bencher<'measurement, BenchMeasurement>,
        report: BenchReport,
    ) -> Self {
        Self::build(platform, benchmark_name, bencher, report)
    }

    fn build(
        platform: Rc<dyn Platform>,
        benchmark_name: Option<&'static str>,
        bencher: &'a mut criterion::Bencher<'measurement, BenchMeasurement>,
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
        let gpu_metrics_enabled = gpu_metrics_enabled_from_env().unwrap_or_else(|error| {
            eprintln!("failed to select benchmark GPU measurements: {error:#}");
            std::process::exit(2);
        });
        let gpu_metrics = BenchGpuMetrics {
            enabled: gpu_metrics_enabled,
            measurement: Rc::new(RefCell::new(None)),
        };
        app.borrow_mut().benchmark_gpu_metrics = Some(gpu_metrics.clone());

        Self {
            app,
            background_executor,
            foreground_executor,
            benchmark_name,
            bencher: Rc::new(RefCell::new(Some(bencher))),
            report,
            gpu_metrics,
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
    /// for (see [`ThreadedDispatcher::run_until_idle`]). Scheduled frames are
    /// delivered after each task poll and when there are no ready tasks, but
    /// animations alone do not keep this method running.
    pub fn run_until_idle(&self) {
        let dispatcher = self
            .background_executor
            .dispatcher()
            .as_threaded()
            .expect("validated in BenchAppContext::build");
        dispatcher.run_until_idle_with(|| {
            let ran_tasks = dispatcher.run_ready_main_tasks_with(
                || true,
                || {
                    self.dispatch_pending_frames(|| true);
                },
            );
            if !ran_tasks {
                self.dispatch_pending_frames(|| true);
            }
            ran_tasks
        });
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
            self.dispatch_pending_frames(|| true);
            if dispatcher.is_idle() {
                return;
            }
        }
    }

    /// Runs main-thread tasks until `ready` returns a value.
    ///
    /// While incomplete, scheduled frames run after each task poll and while
    /// waiting for work. Readiness is checked before frame delivery, so completion
    /// does not wait for a final presentation; renderer sessions do that explicitly.
    pub fn run_until<R>(&self, ready: impl FnMut() -> Option<R>) -> R {
        self.background_executor
            .dispatcher()
            .as_threaded()
            .expect("validated in BenchAppContext::build")
            .run_until_with_frames(ready, || self.dispatch_pending_frames(|| true))
    }

    /// Creates a collector observing foreground journal entries recorded
    /// from this point on, for use by a new [`TraceScope`].
    fn foreground_journal_collector(&self) -> ForegroundJournalCollector {
        self.read(|app| app.foreground_journal().collector())
    }

    /// Measures a generic benchmark workload using Criterion's iteration loop.
    ///
    /// The closure is invoked once per Criterion iteration with this
    /// benchmark app context so it can update GPUI state. Each iteration then
    /// delivers the scheduled frame batch, without draining foreground tasks.
    ///
    /// Any window draws triggered by the workload are recorded into the
    /// benchmark's frame report through the GPUI frame profiler.
    pub fn bench_iter(&mut self, mut benchmark: impl FnMut(&mut Self)) {
        let bencher = self.take_bencher("bench_iter");
        self.dispatch_pending_frames(|| true);
        self.begin_gpu_measurement();
        let collector = TraceScope::start(self.foreground_journal_collector());
        self.report.metrics.discard_pending();
        let mut iterations = 0;
        let mut benchmark = || {
            iterations += 1;
            benchmark(self);
            self.dispatch_pending_frames(|| true);
        };
        bencher.iter(&mut benchmark);
        let events = collector.finish();
        self.report.metrics.record_sample(iterations);
        self.report.record_frame_timings(events.frame_events.iter());
        self.report
            .record_foreground_events(events.foreground_events());
        self.finish_gpu_measurement();
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
        report.metrics.discard_pending();
        let iterations = std::cell::Cell::new(0);

        bencher.iter_batched_ref(
            || {
                // The previous iteration's input and output were just
                // dropped; settling here releases their entities before the
                // next setup, so per-iteration state cannot accumulate
                // across a measurement.
                setup_context.settle();
                let input = setup(&mut setup_context);
                setup_context.dispatch_pending_frames(|| true);
                setup_context.begin_gpu_measurement();
                MeasuredTaskInput {
                    input,
                    trace_scope: Some(TraceScope::start(
                        setup_context.foreground_journal_collector(),
                    )),
                }
            },
            |measured_input| {
                iterations.set(iterations.get() + 1);
                let task = benchmark(&mut measured_input.input, &mut benchmark_context);
                benchmark_context.dispatch_pending_frames(|| true);
                let output = Rc::new(RefCell::new(None));
                let completion = foreground_executor.spawn({
                    let output = output.clone();
                    async move {
                        *output.borrow_mut() = Some(task.await);
                    }
                });
                let output = benchmark_context.run_until(|| output.borrow_mut().take());
                drop(completion);
                MeasuredTaskOutput {
                    trace_scope: measured_input.trace_scope.take(),
                    report: report.clone(),
                    gpu_measurement: benchmark_context.gpu_metrics.measurement.clone(),
                    _output: output,
                }
            },
            criterion::BatchSize::PerIteration,
        );
        report.metrics.record_sample(iterations.get());
        self.replace_bencher(bencher);
    }

    /// Measures unpaced update and rendering work for an entity in its current window.
    ///
    /// Each iteration runs `update` against the entity in its current window. In
    /// renderer measurements, effects flush without drawing until the platform
    /// frame callback. The entity should be part of the window's render tree, such as the
    /// root view or a child of it.
    ///
    /// Each iteration first pumps at most the initial ready task count, delivering
    /// pending frames after every poll. It then runs `update` and delivers pending
    /// frames again. Nested actions and effects finish before frame delivery.
    /// This measures work and rendering cost, not display latency: there is no
    /// pacing, and the report's frame budget does not control execution.
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
        let dispatcher = self.background_executor.dispatcher().clone();
        self.dispatch_pending_frames(|| true);
        self.begin_gpu_measurement();
        let collector = TraceScope::start(self.foreground_journal_collector());
        self.report.metrics.discard_pending();
        let mut iterations = 0;
        let mut benchmark = || {
            iterations += 1;
            dispatcher
                .as_threaded()
                .expect("validated in BenchAppContext::build")
                .run_ready_main_tasks_with(
                    || true,
                    || {
                        self.dispatch_pending_frames(|| true);
                    },
                );
            self.with_window(view.entity_id(), |window, cx| {
                view.update(cx, |view, cx| update(view, window, cx));
            })
            .expect("cannot benchmark renderer for entity without a current window");
            self.dispatch_pending_frames(|| true);
        };
        bencher.iter(&mut benchmark);

        let events = collector.finish();
        self.report.metrics.record_sample(iterations);
        self.report.record_frame_timings(events.frame_events.iter());
        self.report
            .record_foreground_events(events.foreground_events());
        self.finish_gpu_measurement();
        self.replace_bencher(bencher);
    }

    /// Measures finite rendering sessions with fresh, untimed setup for each iteration.
    ///
    /// `setup` returns session state, its window, and a shared stop flag.
    /// Loop turns pump at most the initial ready foreground task count, then call
    /// `input` with a zero-based turn number unless stopped. Pending frames are
    /// delivered after each task poll and input callback. Between polls, the stop
    /// flag and session deadline are checked. The ready batch snapshots a count,
    /// not task identities; rendering after a poll does not inject another input.
    /// This is an unpaced work-and-render policy, not OS event-loop emulation.
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
        let dispatcher = dispatcher
            .as_threaded()
            .expect("validated in BenchAppContext::build");
        let report = self.report.clone();
        report.metrics.discard_pending();
        let iterations = std::cell::Cell::new(0);

        bencher.iter_batched_ref(
            || {
                setup_context.settle();
                let input = setup(&mut setup_context);
                setup_context.dispatch_pending_frames(|| true);
                setup_context.begin_gpu_measurement();
                MeasuredTaskInput {
                    input,
                    trace_scope: Some(TraceScope::start(
                        setup_context.foreground_journal_collector(),
                    )),
                }
            },
            |measured_input| {
                iterations.set(iterations.get() + 1);
                let (state, window, stopped) = &mut measured_input.input;
                let started = Instant::now();
                let check_deadline = || {
                    assert!(
                        started.elapsed() < timeout,
                        "renderer session did not stop within {timeout:?} with final changes presented"
                    );
                };
                let can_poll = || {
                    check_deadline();
                    !stopped.load(Ordering::Acquire)
                };
                let is_finished = |cx: &Self| {
                    if !stopped.load(Ordering::Acquire) {
                        return false;
                    }
                    // Animation callbacks alone must not keep a stopped session alive.
                    let app = cx.app.borrow();
                    let window = app
                        .windows
                        .get(window.window_id())
                        .and_then(Option::as_deref)
                        .expect("renderer session window must remain open");
                    !window.invalidator.is_dirty() && !window.needs_present.get()
                };
                let dispatch_frames = |cx: &Self| {
                    check_deadline();
                    cx.dispatch_pending_frames(|| {
                        check_deadline();
                        !is_finished(cx)
                    });
                    check_deadline();
                };
                let mut frame = 0;
                loop {
                    check_deadline();
                    if is_finished(&benchmark_context) {
                        break;
                    }
                    if can_poll() {
                        dispatcher.run_ready_main_tasks_with(&can_poll, || {
                            dispatch_frames(&benchmark_context);
                        });
                        if !can_poll() {
                            continue;
                        }
                        benchmark_context
                            .update_window(*window, |_, window, cx| {
                                if can_poll() {
                                    input(state, frame, window, cx);
                                }
                            })
                            .expect("renderer session window must remain open");
                    }
                    dispatch_frames(&benchmark_context);
                    frame += 1;
                }
                MeasuredTaskOutput {
                    trace_scope: measured_input.trace_scope.take(),
                    report: report.clone(),
                    gpu_measurement: benchmark_context.gpu_metrics.measurement.clone(),
                    _output: (),
                }
            },
            criterion::BatchSize::PerIteration,
        );
        report.metrics.record_sample(iterations.get());
        self.replace_bencher(bencher);
    }

    fn begin_gpu_measurement(&self) {
        if !self.gpu_metrics.enabled {
            return;
        }
        let mut windows: Vec<_> = self
            .app
            .borrow_mut()
            .windows
            .values_mut()
            .filter_map(|window| window.as_deref_mut()?.platform_window.as_test().cloned())
            .collect();
        for window in &windows {
            window.set_gpu_metrics_enabled(true);
        }
        let mut initial_stats = Vec::new();
        windows.retain(|window| {
            let Some(stats) = drain_gpu_stats(window).log_err() else {
                // A failed baseline wait cannot separate setup from measured frames.
                return false;
            };
            initial_stats.extend(stats);
            true
        });
        let memory_start = combine_gpu_stats(initial_stats).and_then(|stats| stats.memory);
        *self.gpu_metrics.measurement.borrow_mut() = Some(GpuMeasurement {
            windows,
            memory_start,
        });
    }

    fn finish_gpu_measurement(&self) {
        if let Some(measurement) = self.gpu_metrics.measurement.borrow_mut().take() {
            measurement.finish(&self.report);
        }
    }

    fn dispatch_pending_frames(&self, mut should_continue: impl FnMut() -> bool) -> bool {
        let pending: Vec<_> = {
            let mut app = self.app.borrow_mut();
            app.windows
                .values_mut()
                .filter_map(|window| {
                    let window = window.as_deref_mut()?;
                    let handle = window.window_handle();
                    let platform_window = window
                        .platform_window
                        .as_test()
                        .expect("benchmark platform window");
                    platform_window
                        .frame_scheduled()
                        .then(|| (handle, platform_window.clone()))
                })
                .collect()
        };

        let mut dispatched = false;
        for (handle, window) in pending {
            if !should_continue() {
                break;
            }
            // An earlier callback may have closed another window in the batch.
            let is_open = self
                .app
                .borrow()
                .windows
                .get(handle.window_id())
                .and_then(Option::as_deref)
                .is_some();
            if is_open {
                dispatched |= window.simulate_scheduled_frame();
            }
        }
        dispatched
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
                .expect("failed to activate newly created benchmark window");
            window
        };

        self.run_until_idle();
        BenchWindowContext {
            cx: self.clone(),
            window,
        }
    }

    fn take_bencher(
        &self,
        benchmark_kind: &str,
    ) -> &'a mut criterion::Bencher<'measurement, BenchMeasurement> {
        self.bencher.borrow_mut().take().unwrap_or_else(|| {
            panic!("cannot start {benchmark_kind}: benchmark measurement is already running")
        })
    }

    fn replace_bencher(&self, bencher: &'a mut criterion::Bencher<'measurement, BenchMeasurement>) {
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

    /// Resizes the window's content area as the platform does when the user drags its
    /// edge, through the window's resize callback. The next frame redraws at the new size.
    ///
    /// Must not be called from inside [`Self::update`]: the callback updates the window.
    pub fn simulate_resize(&mut self, size: Size<Pixels>) {
        let mut platform_window = {
            let mut app = self.cx.app.borrow_mut();
            app.windows
                .get_mut(self.window.window_id())
                .and_then(|window| window.as_deref_mut())
                .expect("benchmark window was unexpectedly closed")
                .platform_window
                .as_test()
                .expect("benchmark platform window")
                .clone()
        };
        platform_window.simulate_resize(size);
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
    #[cfg(feature = "test-support")]
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::{rc::Rc, sync::Arc};

    use super::*;
    use crate::profiler::journal::install_test_foreground_journal;

    #[derive(Default)]
    struct FakeGpuState {
        enabled: bool,
        measured: bool,
        measured_frames: u64,
        pending: Vec<crate::GpuFrameStats>,
        completed: Vec<crate::GpuFrameStats>,
        waits: usize,
        drains: usize,
        fail_next_wait: bool,
        failed_waits: usize,
    }

    struct FakeGpuRenderer {
        state: Rc<RefCell<FakeGpuState>>,
        timing: Rc<std::cell::Cell<bool>>,
    }

    struct GpuTestMeasurement(Rc<std::cell::Cell<bool>>);

    impl criterion::measurement::Measurement for GpuTestMeasurement {
        type Intermediate = Instant;
        type Value = Duration;

        fn start(&self) -> Instant {
            self.0.set(true);
            Instant::now()
        }

        fn end(&self, started: Instant) -> Duration {
            self.0.set(false);
            started.elapsed()
        }

        fn add(&self, left: &Duration, right: &Duration) -> Duration {
            *left + *right
        }

        fn zero(&self) -> Duration {
            Duration::ZERO
        }

        fn to_f64(&self, value: &Duration) -> f64 {
            value.as_nanos() as f64
        }

        fn formatter(&self) -> &dyn criterion::measurement::ValueFormatter {
            criterion::measurement::WallTime.formatter()
        }
    }

    impl PlatformHeadlessRenderer for FakeGpuRenderer {
        fn render_scene_to_image(
            &mut self,
            _: &crate::Scene,
            _: crate::Size<crate::DevicePixels>,
        ) -> Result<image::RgbaImage> {
            anyhow::bail!("fake GPU renderer has no pixels")
        }

        fn render_scene(
            &mut self,
            _: &crate::Scene,
            _: crate::Size<crate::DevicePixels>,
        ) -> Result<()> {
            let mut state = self.state.borrow_mut();
            if state.enabled {
                let measured = state.measured;
                state.measured_frames += u64::from(measured);
                state.pending.push(crate::GpuFrameStats {
                    gpu_duration: Some(Duration::from_millis(if measured { 10 } else { 1000 })),
                    submit_to_complete: Some(Duration::from_millis(12)),
                    frames_in_flight: 2,
                    instance_bytes: if measured { 16 } else { 1_000_000 },
                    atlas_upload_bytes: 32,
                    uniform_bytes: 4,
                    atlas_uploads: 1,
                    draw_calls: 3,
                    render_passes: 1,
                    resource_creations: 2,
                    resource_created_bytes: 64,
                });
            }
            Ok(())
        }

        fn sprite_atlas(&self) -> Arc<dyn crate::PlatformAtlas> {
            Arc::new(crate::HeadlessAtlas::default())
        }

        fn set_gpu_metrics_enabled(&mut self, enabled: bool) {
            let mut state = self.state.borrow_mut();
            state.enabled = enabled;
            if !enabled {
                state.pending.clear();
                state.completed.clear();
            }
        }

        fn wait_for_gpu_idle(&mut self) -> Result<()> {
            assert!(
                !self.timing.get(),
                "GPU waits must be outside Criterion timing"
            );
            let mut state = self.state.borrow_mut();
            state.waits += 1;
            if std::mem::take(&mut state.fail_next_wait) {
                state.failed_waits += 1;
                anyhow::bail!("simulated GPU wait failure");
            }
            let pending = std::mem::take(&mut state.pending);
            state.completed.extend(pending);
            Ok(())
        }

        fn take_gpu_stats(&mut self) -> Option<GpuStats> {
            assert!(
                !self.timing.get(),
                "GPU drains must be outside Criterion timing"
            );
            let mut state = self.state.borrow_mut();
            assert!(state.pending.is_empty(), "must wait before draining");
            state.drains += 1;
            state.enabled.then(|| GpuStats {
                frames: std::mem::take(&mut state.completed),
                memory: Some(GpuMemoryStats {
                    atlas_bytes: if state.measured { 200 } else { 100 },
                    ..Default::default()
                }),
                peak_owned_bytes: Some(if state.measured { 300 } else { 100 }),
            })
        }
    }

    fn gpu_benchmark(
        name: &'static str,
        enabled: bool,
        benchmark: impl Fn(&mut BenchAppContext, Rc<RefCell<FakeGpuState>>),
    ) -> BenchReport {
        let report = BenchReport::default();
        let timing = Rc::new(std::cell::Cell::new(false));
        let mut criterion = criterion::Criterion::default()
            .with_measurement(BenchMeasurement::new(GpuTestMeasurement(timing.clone())))
            .without_plots()
            .sample_size(10)
            .warm_up_time(Duration::from_millis(1))
            .measurement_time(Duration::from_millis(1));
        criterion.bench_function(name, |bencher| {
            let state = Rc::new(RefCell::new(FakeGpuState::default()));
            let platform = bench_platform(
                Some(Box::new({
                    let state = state.clone();
                    let timing = timing.clone();
                    move || {
                        Ok(Some(Box::new(FakeGpuRenderer {
                            state: state.clone(),
                            timing: timing.clone(),
                        })))
                    }
                })),
                Arc::new(crate::NoopTextSystem::new()),
            );
            let mut cx = BenchAppContext::new_with_platform_and_report(
                platform,
                Some(name),
                bencher,
                report.clone(),
            );
            // Inject parsed configuration without mutating process-wide environment.
            cx.gpu_metrics.enabled = enabled;
            cx.app.borrow_mut().benchmark_gpu_metrics = Some(cx.gpu_metrics.clone());
            let frames_before = report.gpu().map_or(0, |summary| summary.frames);
            benchmark(&mut cx, state.clone());
            if enabled {
                let state = state.borrow();
                let summary = report.gpu().expect("GPU frames should be recorded");
                assert!(state.measured_frames > 0);
                assert_eq!(summary.frames - frames_before, state.measured_frames);
                assert_eq!(summary.instance_bytes.mean, 16.);
                assert_eq!(
                    summary.instance_bytes.max, 16,
                    "setup frames must be discarded"
                );
                assert_eq!(state.waits, state.drains + state.failed_waits);
                assert!(state.pending.is_empty());
                assert!(state.completed.is_empty());
            } else {
                assert!(report.gpu().is_none());
                assert!(!state.borrow().enabled);
                assert_eq!(state.borrow().waits, 0);
                assert_eq!(state.borrow().drains, 0);
            }
            cx.teardown();
        });
        report
    }

    #[test]
    fn gpu_renderer_records_completed_frames_and_discards_setup() {
        let report = gpu_benchmark("gpu_renderer", true, |cx, state| {
            let mut window = cx.add_empty_window();
            let view = window.update(|window, cx| window.replace_root(cx, |_, _| Empty));
            cx.bench_renderer(view, |_, _, cx| {
                state.borrow_mut().measured = true;
                cx.notify();
            });
        });
        let summary = report.gpu().expect("GPU summary");
        assert_eq!(summary.frames_over_budget, summary.frames);
        assert_eq!(summary.max_frames_in_flight, 2);
        assert_eq!(
            summary.gpu_duration.expect("GPU timestamps").count,
            summary.frames
        );
        assert_eq!(
            summary.memory_start.expect("initial memory").owned_bytes(),
            100
        );
        assert_eq!(summary.memory_end.expect("final memory").owned_bytes(), 200);
        assert_eq!(summary.owned_bytes_growth, Some(100));
        assert_eq!(summary.peak_owned_bytes, Some(300));
    }

    #[test]
    fn gpu_failed_baseline_wait_does_not_record_setup_frames() {
        gpu_benchmark("gpu_failed_wait", true, |cx, state| {
            let mut window = cx.add_empty_window();
            let view = window.update(|window, cx| window.replace_root(cx, |_, _| Empty));
            let frames_before = cx.report.gpu().map_or(0, |summary| summary.frames);
            state.borrow_mut().fail_next_wait = true;
            cx.begin_gpu_measurement();
            cx.finish_gpu_measurement();
            assert_eq!(
                cx.report.gpu().map_or(0, |summary| summary.frames),
                frames_before
            );
            assert_eq!(state.borrow().drains, 0);
            assert!(!state.borrow().pending.is_empty());
            cx.bench_renderer(view, |_, _, cx| {
                state.borrow_mut().measured = true;
                cx.notify();
            });
        });
    }

    #[test]
    fn gpu_renderer_opt_out_disables_collection() {
        for setting in ["off", "0", "false"] {
            gpu_benchmark(
                "gpu_renderer_disabled",
                gpu_metrics_enabled(Some(setting), false).unwrap(),
                |cx, _| {
                    let mut window = cx.add_empty_window();
                    let view = window.update(|window, cx| window.replace_root(cx, |_, _| Empty));
                    cx.bench_renderer(view, |_, _, cx| cx.notify());
                },
            );
        }
        assert!(gpu_metrics_enabled(None, false).unwrap());
        for setting in [None, Some("on"), Some("true"), Some("1")] {
            assert!(!gpu_metrics_enabled(setting, true).unwrap());
        }
        assert!(gpu_metrics_enabled(Some("invalid"), false).is_err());
        assert!(gpu_metrics_enabled(Some("invalid"), true).is_err());
    }

    #[test]
    fn gpu_renderer_session_drains_after_timing() {
        gpu_benchmark("gpu_renderer_session", true, |cx, state| {
            let mut window = cx.add_empty_window();
            let view = window.update(|window, cx| window.replace_root(cx, |_, _| Empty));
            let handle = window.window_handle();
            cx.bench_renderer_session(
                Duration::from_secs(1),
                |_| {
                    state.borrow_mut().measured = false;
                    let stopped = Arc::new(AtomicBool::new(false));
                    ((view.clone(), stopped.clone()), handle, stopped)
                },
                |(view, stopped), _, _, cx| {
                    state.borrow_mut().measured = true;
                    view.update(cx, |_, cx| cx.notify());
                    stopped.store(true, Ordering::Release);
                },
            );
            drop(view);
        });
    }

    #[test]
    fn gpu_task_and_iteration_paths_record_frames() {
        for kind in ["iter", "task", "batched_task"] {
            gpu_benchmark(kind, true, |cx, state| {
                let mut window = cx.add_empty_window();
                let view = window.update(|window, cx| window.replace_root(cx, |_, _| Empty));
                let render = |cx: &mut BenchAppContext| {
                    state.borrow_mut().measured = true;
                    view.update(cx, |_, cx| cx.notify());
                };
                match kind {
                    "iter" => cx.bench_iter(render),
                    "task" => cx.bench_task(|cx| {
                        render(cx);
                        Task::ready(())
                    }),
                    "batched_task" => cx.bench_batched_task(
                        |cx| {
                            state.borrow_mut().measured = false;
                            view.update(cx, |_, cx| cx.notify());
                        },
                        |(), cx| {
                            render(cx);
                            Task::ready(())
                        },
                    ),
                    _ => unreachable!(),
                }
            });
        }
    }

    #[test]
    fn gpu_tracks_windows_opened_and_closed_inside_measurement() {
        gpu_benchmark("gpu_window_lifetime", true, |cx, state| {
            cx.bench_iter(|cx| {
                state.borrow_mut().measured = true;
                let handle = cx.update(|cx| {
                    cx.open_window(WindowOptions::default(), |_, cx| cx.new(|_| Empty))
                        .expect("open benchmark window")
                });
                cx.dispatch_pending_frames(|| true);
                cx.update(|cx| {
                    handle
                        .update(cx, |_, window, _| window.remove_window())
                        .expect("close benchmark window");
                });
            });
        });
    }

    #[test]
    fn gpu_combines_owned_memory_without_double_counting_driver_totals() {
        let snapshot = || GpuStats {
            memory: Some(GpuMemoryStats {
                atlas_bytes: 10,
                driver_allocated_bytes: Some(1000),
                ..Default::default()
            }),
            peak_owned_bytes: Some(20),
            ..Default::default()
        };
        let combined = combine_gpu_stats([snapshot(), snapshot()]).unwrap();
        assert_eq!(combined.memory.unwrap().owned_bytes(), 20);
        assert_eq!(combined.memory.unwrap().driver_allocated_bytes, None);
        assert_eq!(combined.peak_owned_bytes, Some(40));
        let combined = combine_gpu_stats([snapshot(), GpuStats::default()]).unwrap();
        assert_eq!(combined.memory, None);
        assert_eq!(combined.peak_owned_bytes, None);
        assert!(combine_gpu_stats([]).is_none());
    }

    #[test]
    fn gpu_summary_aggregates_counters_and_optional_durations() {
        let report = BenchReport::with_frame_budget_nanos(100);
        assert!(report.gpu().is_none());
        let memory = GpuMemoryStats {
            atlas_bytes: 10,
            instance_buffer_bytes: 20,
            intermediate_texture_bytes: 30,
            render_target_bytes: 40,
            driver_allocated_bytes: Some(1000),
        };
        assert_eq!(memory.owned_bytes(), 100);
        report.record_gpu_stats(
            Some(memory),
            GpuStats {
                frames: vec![
                    crate::GpuFrameStats {
                        gpu_duration: Some(Duration::from_nanos(100)),
                        submit_to_complete: Some(Duration::from_nanos(150)),
                        instance_bytes: 10,
                        atlas_upload_bytes: 20,
                        uniform_bytes: 30,
                        atlas_uploads: 2,
                        draw_calls: 4,
                        render_passes: 6,
                        resource_creations: 8,
                        resource_created_bytes: 40,
                        frames_in_flight: 3,
                    },
                    crate::GpuFrameStats::default(),
                    crate::GpuFrameStats {
                        gpu_duration: Some(Duration::from_nanos(200)),
                        ..Default::default()
                    },
                ],
                memory: Some(GpuMemoryStats::default()),
                peak_owned_bytes: Some(200),
            },
        );
        let summary = report.gpu().unwrap();
        assert_eq!(summary.frames, 3);
        assert_eq!(summary.frames_over_budget, 1);
        let durations = summary.gpu_duration.unwrap();
        assert_eq!(durations.count, 2);
        assert_eq!(durations.p50, Duration::from_nanos(100));
        assert_eq!(durations.p90, Duration::from_nanos(200));
        assert_eq!(durations.p95, durations.p90);
        assert_eq!(durations.p99, durations.p90);
        assert_eq!(durations.max, durations.p90);
        assert_eq!(summary.submit_to_complete.unwrap().count, 1);
        assert_eq!(summary.owned_bytes_growth, Some(-100));
        for (counter, total) in [
            (summary.instance_bytes, 10),
            (summary.atlas_upload_bytes, 20),
            (summary.uniform_bytes, 30),
            (summary.atlas_uploads, 2),
            (summary.draw_calls, 4),
            (summary.render_passes, 6),
            (summary.resource_creations, 8),
            (summary.resource_created_bytes, 40),
        ] {
            assert_eq!(counter.total, total);
            assert_eq!(counter.mean, total as f64 / 3.);
            assert_eq!(counter.max, total as u64);
        }
        let report = BenchReport::default();
        report.record_gpu_stats(
            None,
            GpuStats {
                frames: vec![crate::GpuFrameStats::default()],
                ..Default::default()
            },
        );
        let summary = report.gpu().unwrap();
        assert!(summary.gpu_duration.is_none());
        assert!(summary.submit_to_complete.is_none());
        assert!(summary.owned_bytes_growth.is_none());
    }

    /// Runs `benchmark` under a wall-time primary with a fake counter as the
    /// secondary metric and returns the per-iteration values the report
    /// recorded for it.
    #[cfg(feature = "test-support")]
    fn secondary_values_per_iteration(
        counter: Arc<AtomicU64>,
        benchmark: impl Fn(&mut BenchAppContext, &Arc<AtomicU64>),
    ) -> Vec<u64> {
        let platform = bench_platform(None, Arc::new(crate::NoopTextSystem::new()));
        let report = BenchReport::default();
        let mut criterion = criterion::Criterion::default()
            .with_measurement(
                BenchMeasurement::new(criterion::measurement::WallTime).with_secondary(
                    "fake counter",
                    bench_metrics::FakeCounter::new(counter.clone()),
                ),
            )
            .without_plots()
            .sample_size(10)
            .warm_up_time(Duration::from_millis(1))
            .measurement_time(Duration::from_millis(1));

        criterion.bench_function("secondary_values_per_iteration", |bencher| {
            let mut cx = BenchAppContext::new_with_platform_and_report(
                platform.clone(),
                Some("secondary_values_per_iteration"),
                bencher,
                report.clone(),
            );
            // Fixture work outside the measured loop must not be attributed
            // to any iteration.
            counter.fetch_add(1_000_000, Ordering::SeqCst);
            benchmark(&mut cx, &counter);
            cx.teardown();
        });

        report
            .metrics()
            .values("fake counter")
            .expect("the fake counter was recorded")
            .into_iter()
            .map(|value| value as u64)
            .collect()
    }

    #[cfg(feature = "test-support")]
    #[test]
    fn secondary_metric_is_normalized_per_iteration_for_bench_iter() {
        let values = secondary_values_per_iteration(Arc::new(AtomicU64::new(0)), |cx, counter| {
            cx.bench_iter(|_| {
                counter.fetch_add(11, Ordering::SeqCst);
            });
        });
        assert!(!values.is_empty());
        assert!(
            values.iter().all(|value| *value == 11),
            "each sample's total should be divided by its iteration count: {values:?}"
        );
    }

    #[cfg(feature = "test-support")]
    #[test]
    fn secondary_metric_is_accumulated_across_per_iteration_batches() {
        let values = secondary_values_per_iteration(Arc::new(AtomicU64::new(0)), |cx, counter| {
            cx.bench_batched_task(
                |_| {
                    // Setup runs outside every measured interval.
                    counter.fetch_add(1_000, Ordering::SeqCst);
                },
                |(), cx| {
                    counter.fetch_add(11, Ordering::SeqCst);
                    cx.background_spawn(async {})
                },
            );
        });
        assert!(!values.is_empty());
        assert!(
            values.iter().all(|value| *value == 11),
            "PerIteration batches should sum to one per-iteration value: {values:?}"
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
            .with_measurement(BenchMeasurement::new(criterion::measurement::WallTime))
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
}
