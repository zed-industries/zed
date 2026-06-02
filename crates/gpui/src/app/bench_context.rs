#[cfg(feature = "bench")]
use std::{cell::RefCell, time::Duration};
use std::{future::Future, rc::Rc, sync::Arc};

use anyhow::{Result, anyhow};
#[cfg(feature = "bench")]
use scheduler::Instant;

#[cfg(feature = "bench")]
use crate::FrameLatencySnapshot;
use crate::{
    AnyView, AnyWindowHandle, App, AppCell, AppContext, BackgroundExecutor, Bounds, Context, Empty,
    Entity, EntityId, Focusable, ForegroundExecutor, Global, Render, Reservation, Task,
    TestDispatcher, TestPlatform, VisualContext, Window, WindowBounds, WindowHandle, WindowOptions,
    app::{GpuiBorrow, GpuiMode},
};

#[cfg(feature = "bench")]
const FRAME_BUDGET_NANOS: u128 = 16_666_667;

/// A small report produced by GPUI benchmarks.
#[cfg(feature = "bench")]
#[derive(Clone, Default)]
pub struct BenchReport {
    measurements: Rc<RefCell<Vec<BenchMeasurementReport>>>,
}

#[cfg(feature = "bench")]
impl BenchReport {
    fn record_sample(&self, name: &'static str, foreground_time: Duration) {
        let missed_frames = missed_frames(foreground_time);
        self.record_summary(
            name,
            1,
            foreground_time.as_nanos(),
            foreground_time,
            missed_frames,
            missed_frames,
        );
    }

    fn record_frame_latency_delta(
        &self,
        before: &FrameLatencySnapshot,
        after: &FrameLatencySnapshot,
    ) {
        let mut dirty_to_draw = after.dirty_to_draw_histogram.clone();
        match dirty_to_draw.subtract(&before.dirty_to_draw_histogram) {
            Ok(()) => self.record_histogram_summary("bench_renderer dirty-to-draw", &dirty_to_draw),
            Err(error) => eprintln!("failed to compute dirty-to-draw benchmark delta: {error}"),
        }

        let mut draw = after.draw_histogram.clone();
        match draw.subtract(&before.draw_histogram) {
            Ok(()) => self.record_histogram_summary("bench_renderer draw", &draw),
            Err(error) => eprintln!("failed to compute draw benchmark delta: {error}"),
        }
    }

    fn record_histogram_summary(
        &self,
        name: &'static str,
        histogram: &hdrhistogram::Histogram<u64>,
    ) {
        let samples = histogram.len();
        if samples == 0 {
            return;
        }

        let total_nanos = (histogram.mean() * samples as f64) as u128;
        let max = Duration::from_nanos(histogram.max());
        self.record_summary(
            name,
            samples,
            total_nanos,
            max,
            total_missed_frames(histogram),
            missed_frames(max),
        );
    }

    fn record_summary(
        &self,
        name: &'static str,
        samples: u64,
        total_foreground_nanos: u128,
        max_foreground_time: Duration,
        total_missed_frames: u64,
        max_missed_frames: u64,
    ) {
        let mut measurements = self.measurements.borrow_mut();
        match measurements
            .iter_mut()
            .find(|measurement| measurement.name == name)
        {
            Some(measurement) => measurement.record_summary(
                samples,
                total_foreground_nanos,
                max_foreground_time,
                total_missed_frames,
                max_missed_frames,
            ),
            None => {
                let mut measurement = BenchMeasurementReport::new(name);
                measurement.record_summary(
                    samples,
                    total_foreground_nanos,
                    max_foreground_time,
                    total_missed_frames,
                    max_missed_frames,
                );
                measurements.push(measurement);
            }
        }
    }

    /// Prints this report to stderr.
    pub fn print(&self, benchmark_name: Option<&'static str>) {
        let measurements = self.measurements.borrow();
        if measurements.is_empty() {
            return;
        }

        let benchmark_name = benchmark_name.unwrap_or("unknown benchmark");
        eprintln!("GPUI bench report (all observed iterations): {benchmark_name}");
        eprintln!("  note: includes Criterion warmup/calibration");
        for measurement in measurements.iter() {
            eprintln!(
                "  {}: foreground mean {}, max {}, missed frames total {}, max {}",
                measurement.name,
                format_duration(measurement.mean_foreground_time()),
                format_duration(measurement.max_foreground_time),
                measurement.total_missed_frames,
                measurement.max_missed_frames,
            );
        }
    }
}

#[cfg(feature = "bench")]
struct BenchMeasurementReport {
    name: &'static str,
    samples: u64,
    total_foreground_nanos: u128,
    max_foreground_time: Duration,
    total_missed_frames: u64,
    max_missed_frames: u64,
}

#[cfg(feature = "bench")]
impl BenchMeasurementReport {
    fn new(name: &'static str) -> Self {
        Self {
            name,
            samples: 0,
            total_foreground_nanos: 0,
            max_foreground_time: Duration::ZERO,
            total_missed_frames: 0,
            max_missed_frames: 0,
        }
    }

    fn record_summary(
        &mut self,
        samples: u64,
        total_foreground_nanos: u128,
        max_foreground_time: Duration,
        total_missed_frames: u64,
        max_missed_frames: u64,
    ) {
        self.samples += samples;
        self.total_foreground_nanos += total_foreground_nanos;
        self.max_foreground_time = self.max_foreground_time.max(max_foreground_time);
        self.total_missed_frames += total_missed_frames;
        self.max_missed_frames = self.max_missed_frames.max(max_missed_frames);
    }

    fn mean_foreground_time(&self) -> Duration {
        Duration::from_nanos((self.total_foreground_nanos / self.samples as u128) as u64)
    }
}

#[cfg(feature = "bench")]
fn total_missed_frames(histogram: &hdrhistogram::Histogram<u64>) -> u64 {
    histogram
        .iter_recorded()
        .map(|value| {
            missed_frames(Duration::from_nanos(value.value_iterated_to())) * value.count_at_value()
        })
        .sum()
}

#[cfg(feature = "bench")]
fn missed_frames(foreground_time: Duration) -> u64 {
    let foreground_nanos = foreground_time.as_nanos();
    if foreground_nanos <= FRAME_BUDGET_NANOS {
        return 0;
    }

    let over_budget_nanos = foreground_nanos - FRAME_BUDGET_NANOS;
    ((over_budget_nanos + FRAME_BUDGET_NANOS - 1) / FRAME_BUDGET_NANOS) as u64
}

#[cfg(feature = "bench")]
fn format_duration(duration: Duration) -> String {
    format!("{:.3}ms", duration.as_secs_f64() * 1000.)
}

/// A GPUI app context for Criterion benchmarks.
///
/// `BenchAppContext` is intentionally separate from `TestAppContext`: it owns a
/// benchmark app instance and exposes only the app/window operations needed by
/// benchmark setup. Criterion remains responsible for the measured loop via its
/// `Bencher` API.
#[derive(Clone)]
pub struct BenchAppContext {
    app: Rc<AppCell>,
    background_executor: BackgroundExecutor,
    foreground_executor: ForegroundExecutor,
    dispatcher: TestDispatcher,
    benchmark_name: Option<&'static str>,
    #[cfg(feature = "bench")]
    report: BenchReport,
}

impl BenchAppContext {
    /// Creates a new benchmark app context.
    pub fn new(benchmark_name: Option<&'static str>) -> Self {
        Self::with_seed(benchmark_name, 0)
    }

    /// Creates a new benchmark app context with the provided scheduler seed.
    pub fn with_seed(benchmark_name: Option<&'static str>, seed: u64) -> Self {
        #[cfg(feature = "bench")]
        {
            Self::build(
                TestDispatcher::new(seed),
                benchmark_name,
                BenchReport::default(),
            )
        }

        #[cfg(not(feature = "bench"))]
        {
            Self::build(TestDispatcher::new(seed), benchmark_name)
        }
    }

    /// Creates a new benchmark app context with a shared report.
    #[cfg(feature = "bench")]
    #[doc(hidden)]
    pub fn new_with_report(benchmark_name: Option<&'static str>, report: BenchReport) -> Self {
        Self::build(TestDispatcher::new(0), benchmark_name, report)
    }

    #[cfg(feature = "bench")]
    fn build(
        dispatcher: TestDispatcher,
        benchmark_name: Option<&'static str>,
        report: BenchReport,
    ) -> Self {
        let (app, background_executor, foreground_executor, dispatcher) =
            Self::build_parts(dispatcher);

        Self {
            app,
            background_executor,
            foreground_executor,
            dispatcher,
            benchmark_name,
            report,
        }
    }

    #[cfg(not(feature = "bench"))]
    fn build(dispatcher: TestDispatcher, benchmark_name: Option<&'static str>) -> Self {
        let (app, background_executor, foreground_executor, dispatcher) =
            Self::build_parts(dispatcher);

        Self {
            app,
            background_executor,
            foreground_executor,
            dispatcher,
            benchmark_name,
        }
    }

    fn build_parts(
        dispatcher: TestDispatcher,
    ) -> (
        Rc<AppCell>,
        BackgroundExecutor,
        ForegroundExecutor,
        TestDispatcher,
    ) {
        let dispatcher = Arc::new(dispatcher);
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher.clone());
        let platform = TestPlatform::new(background_executor.clone(), foreground_executor.clone());
        let asset_source = Arc::new(());
        let http_client = http_client::FakeHttpClient::with_404_response();
        let app = App::new_app(platform, asset_source, http_client);
        app.borrow_mut().mode = GpuiMode::test();

        (
            app,
            background_executor,
            foreground_executor,
            (*dispatcher).clone(),
        )
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

    /// Runs pending scheduled work until the benchmark app is idle.
    pub fn run_until_idle(&self) {
        self.dispatcher.run_until_parked();
    }

    /// Updates the app.
    pub fn update<R>(&mut self, update: impl FnOnce(&mut App) -> R) -> R {
        let mut app = self.app.borrow_mut();
        app.update(update)
    }

    /// Reads app state.
    pub fn read<R>(&self, read: impl FnOnce(&App) -> R) -> R {
        let app = self.app.borrow();
        read(&app)
    }

    /// Measures a generic benchmark workload using Criterion's iteration loop.
    ///
    /// The closure is invoked once per Criterion iteration and receives this
    /// benchmark app context so it can update GPUI state.
    #[cfg(feature = "bench")]
    pub fn bench_iter(
        &mut self,
        bencher: &mut criterion::Bencher<'_>,
        mut benchmark: impl FnMut(&mut Self),
    ) {
        let report = self.report.clone();
        bencher.iter(|| {
            let started_at = Instant::now();
            benchmark(self);
            report.record_sample("bench_iter", started_at.elapsed());
        });
    }

    /// Measures the foreground render pipeline for a GPUI entity's current window.
    ///
    /// Each iteration runs `update` against the entity in its current window, then
    /// measures a normal `Window::draw` for that window. The entity should be part
    /// of the window's render tree, such as the root view or a child of it.
    #[cfg(feature = "bench")]
    pub fn bench_renderer<V>(
        &mut self,
        bencher: &mut criterion::Bencher<'_>,
        view: Entity<V>,
        mut update: impl FnMut(&mut V, &mut Window, &mut Context<V>),
    ) where
        V: 'static + Render,
    {
        let report = self.report.clone();
        bencher.iter(|| {
            let before = self
                .with_window(view.entity_id(), |window, _| {
                    window.frame_latency_snapshot()
                })
                .expect("cannot benchmark renderer for entity without a current window");
            self.with_window(view.entity_id(), |window, cx| {
                view.update(cx, |view, cx| update(view, window, cx));
                let arena_clear_needed = window.draw(cx);
                window.present();
                arena_clear_needed.clear();
                let after = window.frame_latency_snapshot();
                report.record_frame_latency_delta(&before, &after);
            })
            .expect("cannot benchmark renderer for entity without a current window");
        });
    }

    /// Adds a window with an empty root view for benchmark setup.
    pub fn add_empty_window(&mut self) -> BenchWindowContext {
        let window = {
            let mut app = self.app.borrow_mut();
            let bounds = Bounds::maximized(None, &app);
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

    /// Runs GPUI benchmark teardown.
    pub fn teardown(mut self) {
        self.run_until_idle();
        self.update(|cx| {
            cx.background_executor().forbid_parking();
            cx.quit();
        });
        self.run_until_idle();
    }
}

impl AppContext for BenchAppContext {
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

    fn as_mut<'a, T>(&'a mut self, _: &Entity<T>) -> GpuiBorrow<'a, T>
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
pub struct BenchWindowContext {
    cx: BenchAppContext,
    window: AnyWindowHandle,
}

impl BenchWindowContext {
    /// Returns the underlying benchmark app context.
    pub fn app_context(&mut self) -> &mut BenchAppContext {
        &mut self.cx
    }

    /// Returns the window associated with this context.
    pub fn window_handle(&self) -> AnyWindowHandle {
        self.window
    }

    /// Updates the benchmark window.
    pub fn update<R>(&mut self, update: impl FnOnce(&mut Window, &mut App) -> R) -> R {
        self.cx
            .update_window(self.window, |_, window, cx| update(window, cx))
            .expect("benchmark window was unexpectedly closed")
    }

    /// Runs pending scheduled work until the benchmark app is idle.
    pub fn run_until_idle(&self) {
        self.cx.run_until_idle();
    }
}

impl AppContext for BenchWindowContext {
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

    fn as_mut<'a, T>(&'a mut self, handle: &Entity<T>) -> GpuiBorrow<'a, T>
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

impl VisualContext for BenchWindowContext {
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
