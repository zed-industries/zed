use std::{cell::RefCell, future::Future, rc::Rc, sync::Arc, time::Duration};

use anyhow::{Result, anyhow};
use gpui_util::ResultExt;
use hdrhistogram::Histogram;
use scheduler::Instant;

use crate::{
    AnyView, AnyWindowHandle, App, AppCell, AppContext, BackgroundExecutor, Bounds, Context, Empty,
    Entity, EntityId, Focusable, ForegroundExecutor, FrameLatencySnapshot, Global, Platform,
    Render, Reservation, Task, VisualContext, Window, WindowBounds, WindowHandle, WindowOptions,
    app::GpuiBorrow,
};

/// Frame budget used when a benchmark doesn't specify one, in nanoseconds (120fps).
const DEFAULT_FRAME_BUDGET_NANOS: u128 = 1_000_000_000 / 120;

/// A small report produced by GPUI benchmarks.
#[derive(Clone)]
pub struct BenchReport {
    measurements: Rc<RefCell<Vec<BenchMeasurementReport>>>,
    frame_budget_nanos: u128,
}

impl Default for BenchReport {
    fn default() -> Self {
        Self::with_frame_budget_nanos(DEFAULT_FRAME_BUDGET_NANOS)
    }
}

impl BenchReport {
    /// Creates a report that treats `frame_budget_nanos` as the per-frame budget
    /// when counting missed frames.
    pub fn with_frame_budget_nanos(frame_budget_nanos: u128) -> Self {
        Self {
            measurements: Rc::default(),
            frame_budget_nanos,
        }
    }

    fn record_sample(&self, name: &'static str, foreground_time: Duration) {
        self.record_value(name, duration_to_nanos(foreground_time));
    }

    fn record_frame_latency_delta(
        &self,
        before: &FrameLatencySnapshot,
        after: &FrameLatencySnapshot,
    ) {
        let mut dirty_to_draw = after.dirty_to_draw_histogram.clone();
        if dirty_to_draw
            .subtract(&before.dirty_to_draw_histogram)
            .log_err()
            .is_some()
        {
            self.record_histogram("window dirty-to-draw", &dirty_to_draw);
        }

        let mut draw = after.draw_histogram.clone();
        if draw.subtract(&before.draw_histogram).log_err().is_some() {
            self.record_histogram("window draw", &draw);
        }
    }

    fn record_value(&self, name: &'static str, value: u64) {
        let mut measurements = self.measurements.borrow_mut();
        match measurements
            .iter_mut()
            .find(|measurement| measurement.name == name)
        {
            Some(measurement) => measurement.record_value(value),
            None => {
                if let Some(measurement) = BenchMeasurementReport::with_value(name, value) {
                    measurements.push(measurement);
                }
            }
        }
    }

    fn record_histogram(&self, name: &'static str, histogram: &Histogram<u64>) {
        if histogram.is_empty() {
            return;
        }

        let mut measurements = self.measurements.borrow_mut();
        match measurements
            .iter_mut()
            .find(|measurement| measurement.name == name)
        {
            Some(measurement) => measurement.record_histogram(histogram),
            None => measurements.push(BenchMeasurementReport::new(name, histogram.clone())),
        }
    }

    fn total_missed_frames(&self, histogram: &hdrhistogram::Histogram<u64>) -> u64 {
        histogram
            .iter_recorded()
            .map(|value| {
                self.missed_frames(Duration::from_nanos(value.value_iterated_to()))
                    * value.count_at_value()
            })
            .sum()
    }

    fn missed_frames(&self, foreground_time: Duration) -> u64 {
        let foreground_nanos = foreground_time.as_nanos();
        if foreground_nanos <= self.frame_budget_nanos {
            return 0;
        }

        let over_budget_nanos = foreground_nanos - self.frame_budget_nanos;
        over_budget_nanos.div_ceil(self.frame_budget_nanos) as u64
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
            let max_foreground_time = measurement.max_foreground_time();
            eprintln!("  {}:", measurement.name);
            eprintln!("    samples: {}", measurement.samples());
            eprintln!(
                "    mean: {}",
                format_duration(measurement.mean_foreground_time())
            );
            eprintln!(
                "    p50: {}",
                format_duration(measurement.percentile_foreground_time(0.50))
            );
            eprintln!(
                "    p90: {}",
                format_duration(measurement.percentile_foreground_time(0.90))
            );
            eprintln!(
                "    p95: {}",
                format_duration(measurement.percentile_foreground_time(0.95))
            );
            eprintln!(
                "    p99: {}",
                format_duration(measurement.percentile_foreground_time(0.99))
            );
            eprintln!("    max: {}", format_duration(max_foreground_time));
            eprintln!(
                "    missed frames total: {}",
                self.total_missed_frames(&measurement.histogram)
            );
            eprintln!(
                "    missed frames max: {}",
                self.missed_frames(max_foreground_time)
            );
        }
    }
}

struct BenchMeasurementReport {
    name: &'static str,
    histogram: Histogram<u64>,
}

impl BenchMeasurementReport {
    fn new(name: &'static str, histogram: Histogram<u64>) -> Self {
        Self { name, histogram }
    }

    fn with_value(name: &'static str, value: u64) -> Option<Self> {
        let mut histogram = Histogram::new(3).log_err()?;
        histogram.record(value).log_err()?;
        Some(Self { name, histogram })
    }

    fn record_value(&mut self, value: u64) {
        self.histogram.record(value).log_err();
    }

    fn record_histogram(&mut self, histogram: &Histogram<u64>) {
        self.histogram.add(histogram).log_err();
    }

    fn samples(&self) -> u64 {
        self.histogram.len()
    }

    fn mean_foreground_time(&self) -> Duration {
        Duration::from_nanos(self.histogram.mean() as u64)
    }

    fn percentile_foreground_time(&self, quantile: f64) -> Duration {
        Duration::from_nanos(self.histogram.value_at_quantile(quantile))
    }

    fn max_foreground_time(&self) -> Duration {
        Duration::from_nanos(self.histogram.max())
    }
}

fn duration_to_nanos(duration: Duration) -> u64 {
    duration.as_nanos().min(u128::from(u64::MAX)) as u64
}

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
    pub fn new(
        platform: Rc<dyn Platform>,
        benchmark_name: Option<&'static str>,
        bencher: &'a mut criterion::Bencher<'measurement>,
    ) -> Self {
        Self::build(platform, benchmark_name, bencher, BenchReport::default())
    }

    /// Creates a new benchmark app context backed by the provided platform.
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
        let foreground_executor = platform.foreground_executor();
        let asset_source = Arc::new(());
        let http_client = http_client::FakeHttpClient::with_404_response();
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

    /// Measures a generic benchmark workload using Criterion's iteration loop.
    ///
    /// The closure is invoked once per Criterion iteration an
    /// benchmark app context so it can update GPUI state.
    pub fn bench_iter(&mut self, mut benchmark: impl FnMut(&mut Self)) {
        let bencher = self.take_bencher("bench_iter");
        let mut benchmark = || {
            let started_at = Instant::now();
            benchmark(self);
            self.report
                .record_sample("bench_iter", started_at.elapsed());
        };
        bencher.iter(&mut benchmark);
        self.replace_bencher(bencher);
    }

    /// Measures frame latency after updating a GPUI entity in its current window.
    ///
    /// Each iteration runs `update` against the entity in its current window. In
    /// bench builds, flushing the update's effects synchronously draws dirty
    /// windows. The entity should be part of the window's render tree, such as the
    /// root view or a child of it.
    pub fn bench_renderer<V>(
        &mut self,
        view: Entity<V>,
        mut update: impl FnMut(&mut V, &mut Window, &mut Context<V>),
    ) where
        V: 'static + Render,
    {
        let bencher = self.take_bencher("bench_renderer");
        let report = self.report.clone();
        let before = self
            .with_window(view.entity_id(), |window, _| {
                window.frame_latency_snapshot()
            })
            .expect("cannot benchmark renderer for entity without a current window");

        let mut benchmark = || {
            self.with_window(view.entity_id(), |window, cx| {
                view.update(cx, |view, cx| update(view, window, cx));
            })
            .expect("cannot benchmark renderer for entity without a current window");
        };
        bencher.iter(&mut benchmark);

        let after = self
            .with_window(view.entity_id(), |window, _| {
                window.frame_latency_snapshot()
            })
            .expect("cannot benchmark renderer for entity without a current window");
        report.record_frame_latency_delta(&before, &after);
        self.replace_bencher(bencher);
    }

    /// Adds a window with an empty root view for benchmark setup.
    pub fn add_empty_window(&mut self) -> BenchWindowContext<'a, 'measurement> {
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
    pub fn teardown(mut self) {
        self.update(|cx| {
            cx.quit();
        });
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
