use std::{cell::RefCell, future::Future, rc::Rc, sync::Arc};

use anyhow::{Result, anyhow};

use crate::{
    AnyView, AnyWindowHandle, App, AppCell, AppContext, BackgroundExecutor, Bounds, Context, Empty,
    Entity, EntityId, Focusable, ForegroundExecutor, Global, IntoElement, Render, Reservation,
    Task, TestDispatcher, TestPlatform, VisualContext, Window, WindowBounds, WindowHandle,
    WindowOptions,
    app::{GpuiBorrow, GpuiMode},
};

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
    dispatcher: TestDispatcher,
    benchmark_name: Option<&'static str>,
    bencher: Rc<RefCell<Option<&'a mut criterion::Bencher<'measurement>>>>,
}

impl<'a, 'measurement> BenchAppContext<'a, 'measurement> {
    /// Creates a new benchmark app context.
    pub fn new(
        benchmark_name: Option<&'static str>,
        bencher: &'a mut criterion::Bencher<'measurement>,
    ) -> Self {
        Self::with_seed(benchmark_name, 0, bencher)
    }

    /// Creates a new benchmark app context with the provided scheduler seed.
    pub fn with_seed(
        benchmark_name: Option<&'static str>,
        seed: u64,
        bencher: &'a mut criterion::Bencher<'measurement>,
    ) -> Self {
        Self::build(TestDispatcher::new(seed), benchmark_name, bencher)
    }

    fn build(
        dispatcher: TestDispatcher,
        benchmark_name: Option<&'static str>,
        bencher: &'a mut criterion::Bencher<'measurement>,
    ) -> Self {
        let dispatcher = Arc::new(dispatcher);
        let background_executor = BackgroundExecutor::new(dispatcher.clone());
        let foreground_executor = ForegroundExecutor::new(dispatcher.clone());
        let platform = TestPlatform::new(background_executor.clone(), foreground_executor.clone());
        let asset_source = Arc::new(());
        let http_client = http_client::FakeHttpClient::with_404_response();
        let app = App::new_app(platform, asset_source, http_client);
        app.borrow_mut().mode = GpuiMode::test();

        Self {
            app,
            background_executor,
            foreground_executor,
            dispatcher: (*dispatcher).clone(),
            benchmark_name,
            bencher: Rc::new(RefCell::new(Some(bencher))),
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

    /// Runs pending scheduled work until the benchmark app is idle.
    pub fn run_until_idle(&self) {
        self.dispatcher.run_until_parked();
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
    /// The closure is invoked once per Criterion iteration and receives this
    /// benchmark app context so it can update GPUI state.
    pub fn bench_iter(&mut self, mut benchmark: impl FnMut(&mut Self)) {
        let bencher = self.take_bencher("bench_iter");
        let mut benchmark = || benchmark(self);
        bencher.iter(&mut benchmark);
        self.replace_bencher(bencher);
    }

    /// Measures the foreground render pipeline for a GPUI entity.
    ///
    /// Each iteration runs `update` against the entity in its current window,
    /// lets GPUI flush synchronous effects from that update, then renders the
    /// entity by running layout, prepaint, and paint.
    pub fn bench_renderer<V>(
        &mut self,
        view: Entity<V>,
        mut update: impl FnMut(&mut V, &mut Window, &mut Context<V>),
    ) where
        V: 'static + Render,
    {
        let bencher = self.take_bencher("bench_renderer");
        let mut benchmark = || {
            self.with_window(view.entity_id(), |window, cx| {
                view.update(cx, |view, cx| update(view, window, cx));
            })
            .expect("cannot benchmark renderer for entity without a current window");

            self.with_window(view.entity_id(), |window, cx| {
                let mut element = view.clone().into_any_element();
                let _ = element.request_layout(window, cx);
                let _ = element.prepaint(window, cx);
                element.paint(window, cx);
            })
            .expect("cannot benchmark renderer for entity without a current window");
        };
        bencher.iter(&mut benchmark);
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
    pub fn teardown(mut self) {
        self.run_until_idle();
        self.update(|cx| {
            cx.background_executor().forbid_parking();
            cx.quit();
        });
        self.run_until_idle();
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

    /// Runs pending scheduled work until the benchmark app is idle.
    pub fn run_until_idle(&self) {
        self.cx.run_until_idle();
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
