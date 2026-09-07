use std::{
    cell::{OnceCell, RefCell},
    future::Future,
    rc::Rc,
    sync::Arc,
    time::Duration,
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
    /// Creates a report whose per-frame budget is one frame at `fps` when
    /// counting frame budget overruns.
    pub fn with_fps(fps: u64) -> Self {
        assert!(fps > 0, "frame rate must be greater than zero");
        Self::with_frame_budget_nanos(NANOS_PER_SECOND / fps as u128)
    }

    /// Creates a report that treats `frame_budget_nanos` as the per-frame budget
    /// when counting frame budget overruns.
    pub fn with_frame_budget_nanos(frame_budget_nanos: u128) -> Self {
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
            // Infallible: the histogram auto-resizes.
            snapshot
                .foreground_work
                .record(duration.as_nanos() as u64)
                .ok();
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

    /// Returns a snapshot of total foreground executor work observed during
    /// the measured interval: every task poll, action handler, and input
    /// dispatch on the foreground thread, whether or not it produced a
    /// window draw. This is captured through GPUI's foreground journal, so
    /// it requires no window and surfaces a slow or stalled task even when
    /// nothing was drawn while it ran. Durations are recorded in
    /// nanoseconds.
    ///
    /// Empty when no foreground work was recorded, e.g. a
    /// [`BenchAppContext::bench_iter`] measurement that does no async work.
    pub fn foreground_work(&self) -> Histogram<u64> {
        self.frame_snapshot.borrow().foreground_work.clone()
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
        self.print_histogram("window present interval", &frame_snapshot.present_interval);
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

    fn print_foreground_work(&self, foreground_work: &Histogram<u64>) {
        if foreground_work.is_empty() {
            return;
        }

        eprintln!("  foreground executor work (task polls, actions, input dispatch):");
        eprintln!("    note: excludes window draw/present, reported separately above");
        self.print_histogram_body(foreground_work);
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
    foreground_work: Histogram<u64>,
}

impl WindowFrameSnapshot {
    fn new() -> Self {
        Self {
            dirty_to_draw: Histogram::new(3).expect("3 significant digits is valid"),
            draw: Histogram::new(3).expect("3 significant digits is valid"),
            present_interval: Histogram::new(3).expect("3 significant digits is valid"),
            invalidations_per_frame: Histogram::new(3).expect("3 significant digits is valid"),
            foreground_work: Histogram::new(3).expect("3 significant digits is valid"),
        }
    }

    fn is_empty(&self) -> bool {
        self.dirty_to_draw.is_empty()
            && self.draw.is_empty()
            && self.present_interval.is_empty()
            && self.foreground_work.is_empty()
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
        self.report.record_frame_timings(events.frame_events.iter());
        self.report
            .record_foreground_events(events.foreground_events());
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
        self.report.record_frame_timings(events.frame_events.iter());
        self.report
            .record_foreground_events(events.foreground_events());
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
    /// bench builds, flushing the update's effects synchronously draws dirty
    /// windows. The entity should be part of the window's render tree, such as the
    /// root view or a child of it.
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
        let window_id = self
            .with_window(view.entity_id(), |window, _| {
                window.window_handle().window_id()
            })
            .expect("cannot benchmark renderer for entity without a current window");

        let dispatcher = self.background_executor.dispatcher().clone();
        let collector = TraceScope::start(self.foreground_journal_collector());

        let mut benchmark = || {
            // Work already queued at frame start delays the frame in
            // production too, so run it inside the measured interval.
            dispatcher
                .as_threaded()
                .expect("validated in BenchAppContext::build")
                .run_ready_main_tasks();
            self.with_window(view.entity_id(), |window, cx| {
                view.update(cx, |view, cx| update(view, window, cx));
            })
            .expect("cannot benchmark renderer for entity without a current window");
            // Submit the frame drawn by the update's effect flush, mirroring
            // production where every drawn frame is presented. With a headless
            // renderer this includes scene submission to the GPU.
            self.with_window(view.entity_id(), |window, _| {
                window.present_if_needed();
            })
            .expect("cannot benchmark renderer for entity without a current window");
        };
        bencher.iter(&mut benchmark);

        let events = collector.finish();
        self.report
            .record_frame_timings(events.frame_events.iter().filter(|event| match event {
                FrameEvent::Draw(timing) => timing.window_id == window_id,
                FrameEvent::Present(timing) => timing.window_id == window_id,
            }));
        // Foreground work isn't attributed to a window, so unlike frame
        // timings above it isn't filtered by `window_id`. A benchmark app
        // hosts one window at a time, so this cannot pick up unrelated
        // windows' work.
        self.report
            .record_foreground_events(events.foreground_events());
        self.replace_bencher(bencher);
    }

    /// Adds a window with an empty root view for benchmark setup.
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

        let histogram = report.foreground_work();
        assert!(
            !histogram.is_empty(),
            "a long task poll should be reported even without a window draw"
        );
        // The spawned task's own poll is one sample; the tiny wrapper poll
        // that observes its completion in `run_task_to_completion` folds
        // into a second, near-zero sample rather than being dropped.
        assert!(histogram.len() >= 1, "expected at least one recorded item");
        let max = Duration::from_nanos(histogram.max());
        assert!(
            max >= Duration::from_millis(55),
            "expected the long poll's duration to be recorded, got {:?}",
            max
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

        let histogram = report.foreground_work();
        assert!(
            !histogram.is_empty(),
            "the measured task's poll should be reported"
        );
        let max = Duration::from_nanos(histogram.max());
        assert!(
            max < Duration::from_millis(40),
            "setup work's 80ms poll must not leak into the measured summary, got {:?}",
            max
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

        let histogram = report.foreground_work();
        assert!(
            !histogram.is_empty(),
            "bench_task should report foreground work with no window involved"
        );
        let max = Duration::from_nanos(histogram.max());
        assert!(
            max >= Duration::from_millis(15),
            "expected a ~20ms task poll to be recorded, got {:?}",
            max
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
