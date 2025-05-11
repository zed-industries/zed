use crate::{
    AnyView, AnyWindowHandle, App, AppCell, AppContext, BackgroundExecutor, BorrowAppContext,
    Entity, EventEmitter, Focusable, ForegroundExecutor, Global, PromptLevel, Render, Reservation,
    Result, Subscription, Task, VisualContext, Window, WindowHandle,
};
use anyhow::{Context as _, anyhow};
use derive_more::{Deref, DerefMut};
use futures::channel::oneshot;
use std::{future::Future, rc::Weak};

use super::{Context, WeakEntity};

/// An async-friendly version of [App] with a static lifetime so it can be held across `await` points in async code.
/// You're provided with an instance when calling [App::spawn], and you can also create one with [App::to_async].
/// Internally, this holds a weak reference to an `App`, so its methods are fallible to protect against cases where the [App] is dropped.
#[derive(Clone)]
pub struct AsyncApp {
    pub(crate) app: Weak<AppCell>,
    pub(crate) background_executor: BackgroundExecutor,
    pub(crate) foreground_executor: ForegroundExecutor,
}

impl AppContext for AsyncApp {
    type Result<T> = Result<T>;

    fn new<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Context<T>) -> T,
    ) -> Self::Result<Entity<T>> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut app = app.borrow_mut();
        Ok(app.new(build_entity))
    }

    fn reserve_entity<T: 'static>(&mut self) -> Result<Reservation<T>> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut app = app.borrow_mut();
        Ok(app.reserve_entity())
    }

    fn insert_entity<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_entity: impl FnOnce(&mut Context<T>) -> T,
    ) -> Result<Entity<T>> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut app = app.borrow_mut();
        Ok(app.insert_entity(reservation, build_entity))
    }

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Entity<T>,
        update: impl FnOnce(&mut T, &mut Context<T>) -> R,
    ) -> Self::Result<R> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut app = app.borrow_mut();
        Ok(app.update_entity(handle, update))
    }

    fn read_entity<T, R>(
        &self,
        handle: &Entity<T>,
        callback: impl FnOnce(&T, &App) -> R,
    ) -> Self::Result<R>
    where
        T: 'static,
    {
        let app = self.app.upgrade().context("app was released")?;
        let lock = app.borrow();
        Ok(lock.read_entity(handle, callback))
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut Window, &mut App) -> T,
    {
        let app = self.app.upgrade().context("app was released")?;
        let mut lock = app.borrow_mut();
        lock.update_window(window, f)
    }

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(Entity<T>, &App) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        let app = self.app.upgrade().context("app was released")?;
        let lock = app.borrow();
        lock.read_window(window, read)
    }

    fn background_spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        self.background_executor.spawn(future)
    }

    fn read_global<G, R>(&self, callback: impl FnOnce(&G, &App) -> R) -> Self::Result<R>
    where
        G: Global,
    {
        let app = self.app.upgrade().context("app was released")?;
        let mut lock = app.borrow_mut();
        Ok(lock.update(|this| this.read_global(callback)))
    }
}

impl AsyncApp {
    /// Schedules all windows in the application to be redrawn.
    pub fn refresh(&self) -> Result<()> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut lock = app.borrow_mut();
        lock.refresh_windows();
        Ok(())
    }

    /// Get an executor which can be used to spawn futures in the background.
    pub fn background_executor(&self) -> &BackgroundExecutor {
        &self.background_executor
    }

    /// Get an executor which can be used to spawn futures in the foreground.
    pub fn foreground_executor(&self) -> &ForegroundExecutor {
        &self.foreground_executor
    }

    /// Invoke the given function in the context of the app, then flush any effects produced during its invocation.
    pub fn update<R>(&self, f: impl FnOnce(&mut App) -> R) -> Result<R> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut lock = app.borrow_mut();
        Ok(lock.update(f))
    }

    /// Arrange for the given callback to be invoked whenever the given entity emits an event of a given type.
    /// The callback is provided a handle to the emitting entity and a reference to the emitted event.
    pub fn subscribe<T, Event>(
        &mut self,
        entity: &Entity<T>,
        mut on_event: impl FnMut(Entity<T>, &Event, &mut App) + 'static,
    ) -> Result<Subscription>
    where
        T: 'static + EventEmitter<Event>,
        Event: 'static,
    {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut lock = app.borrow_mut();
        let subscription = lock.subscribe(entity, on_event);
        Ok(subscription)
    }

    /// Open a window with the given options based on the root view returned by the given function.
    pub fn open_window<V>(
        &self,
        options: crate::WindowOptions,
        build_root_view: impl FnOnce(&mut Window, &mut App) -> Entity<V>,
    ) -> Result<WindowHandle<V>>
    where
        V: 'static + Render,
    {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut lock = app.borrow_mut();
        lock.open_window(options, build_root_view)
    }

    /// Schedule a future to be polled in the background.
    #[track_caller]
    pub fn spawn<AsyncFn, R>(&self, f: AsyncFn) -> Task<R>
    where
        AsyncFn: AsyncFnOnce(&mut AsyncApp) -> R + 'static,
        R: 'static,
    {
        let mut cx = self.clone();
        self.foreground_executor
            .spawn(async move { f(&mut cx).await })
    }

    /// Determine whether global state of the specified type has been assigned.
    /// Returns an error if the `App` has been dropped.
    pub fn has_global<G: Global>(&self) -> Result<bool> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let app = app.borrow_mut();
        Ok(app.has_global::<G>())
    }

    /// Reads the global state of the specified type, passing it to the given callback.
    ///
    /// Panics if no global state of the specified type has been assigned.
    /// Returns an error if the `App` has been dropped.
    pub fn read_global<G: Global, R>(&self, read: impl FnOnce(&G, &App) -> R) -> Result<R> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let app = app.borrow_mut();
        Ok(read(app.global(), &app))
    }

    /// Reads the global state of the specified type, passing it to the given callback.
    ///
    /// Similar to [`AsyncApp::read_global`], but returns an error instead of panicking
    /// if no state of the specified type has been assigned.
    ///
    /// Returns an error if no state of the specified type has been assigned the `App` has been dropped.
    pub fn try_read_global<G: Global, R>(&self, read: impl FnOnce(&G, &App) -> R) -> Option<R> {
        let app = self.app.upgrade()?;
        let app = app.borrow_mut();
        Some(read(app.try_global()?, &app))
    }

    /// A convenience method for [App::update_global]
    /// for updating the global state of the specified type.
    pub fn update_global<G: Global, R>(
        &self,
        update: impl FnOnce(&mut G, &mut App) -> R,
    ) -> Result<R> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut app = app.borrow_mut();
        Ok(app.update(|cx| cx.update_global(update)))
    }

    /// Run something using this entity and cx, when the returned struct is dropped
    pub fn on_drop<T: 'static, Callback: FnOnce(&mut T, &mut Context<T>) + 'static>(
        &self,
        entity: &WeakEntity<T>,
        f: Callback,
    ) -> util::Deferred<impl FnOnce() + use<T, Callback>> {
        let entity = entity.clone();
        let mut cx = self.clone();
        util::defer(move || {
            entity.update(&mut cx, f).ok();
        })
    }
}

/// A cloneable, owned handle to the application context,
/// composed with the window associated with the current task.
#[derive(Clone, Deref, DerefMut)]
pub struct AsyncWindowContext {
    #[deref]
    #[deref_mut]
    app: AsyncApp,
    window: AnyWindowHandle,
}

impl AsyncWindowContext {
    pub(crate) fn new_context(app: AsyncApp, window: AnyWindowHandle) -> Self {
        Self { app, window }
    }

    /// Get the handle of the window this context is associated with.
    pub fn window_handle(&self) -> AnyWindowHandle {
        self.window
    }

    /// A convenience method for [`App::update_window`].
    pub fn update<R>(&mut self, update: impl FnOnce(&mut Window, &mut App) -> R) -> Result<R> {
        self.app
            .update_window(self.window, |_, window, cx| update(window, cx))
    }

    /// A convenience method for [`App::update_window`].
    pub fn update_root<R>(
        &mut self,
        update: impl FnOnce(AnyView, &mut Window, &mut App) -> R,
    ) -> Result<R> {
        self.app.update_window(self.window, update)
    }

    /// A convenience method for [`Window::on_next_frame`].
    pub fn on_next_frame(&mut self, f: impl FnOnce(&mut Window, &mut App) + 'static) {
        self.window
            .update(self, |_, window, _| window.on_next_frame(f))
            .ok();
    }

    /// A convenience method for [`App::global`].
    pub fn read_global<G: Global, R>(
        &mut self,
        read: impl FnOnce(&G, &Window, &App) -> R,
    ) -> Result<R> {
        self.window
            .update(self, |_, window, cx| read(cx.global(), window, cx))
    }

    /// A convenience method for [`App::update_global`].
    /// for updating the global state of the specified type.
    pub fn update_global<G, R>(
        &mut self,
        update: impl FnOnce(&mut G, &mut Window, &mut App) -> R,
    ) -> Result<R>
    where
        G: Global,
    {
        self.window.update(self, |_, window, cx| {
            cx.update_global(|global, cx| update(global, window, cx))
        })
    }

    /// Schedule a future to be executed on the main thread. This is used for collecting
    /// the results of background tasks and updating the UI.
    #[track_caller]
    pub fn spawn<AsyncFn, R>(&self, f: AsyncFn) -> Task<R>
    where
        AsyncFn: AsyncFnOnce(&mut AsyncWindowContext) -> R + 'static,
        R: 'static,
    {
        let mut cx = self.clone();
        self.foreground_executor
            .spawn(async move { f(&mut cx).await })
    }

    /// Present a platform dialog.
    /// The provided message will be presented, along with buttons for each answer.
    /// When a button is clicked, the returned Receiver will receive the index of the clicked button.
    pub fn prompt(
        &mut self,
        level: PromptLevel,
        message: &str,
        detail: Option<&str>,
        answers: &[&str],
    ) -> oneshot::Receiver<usize> {
        self.window
            .update(self, |_, window, cx| {
                window.prompt(level, message, detail, answers, cx)
            })
            .unwrap_or_else(|_| oneshot::channel().1)
    }
}

impl AppContext for AsyncWindowContext {
    type Result<T> = Result<T>;

    fn new<T>(&mut self, build_entity: impl FnOnce(&mut Context<T>) -> T) -> Result<Entity<T>>
    where
        T: 'static,
    {
        self.window.update(self, |_, _, cx| cx.new(build_entity))
    }

    fn reserve_entity<T: 'static>(&mut self) -> Result<Reservation<T>> {
        self.window.update(self, |_, _, cx| cx.reserve_entity())
    }

    fn insert_entity<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_entity: impl FnOnce(&mut Context<T>) -> T,
    ) -> Self::Result<Entity<T>> {
        self.window
            .update(self, |_, _, cx| cx.insert_entity(reservation, build_entity))
    }

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Entity<T>,
        update: impl FnOnce(&mut T, &mut Context<T>) -> R,
    ) -> Result<R> {
        self.window
            .update(self, |_, _, cx| cx.update_entity(handle, update))
    }

    fn read_entity<T, R>(
        &self,
        handle: &Entity<T>,
        read: impl FnOnce(&T, &App) -> R,
    ) -> Self::Result<R>
    where
        T: 'static,
    {
        self.app.read_entity(handle, read)
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, update: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut Window, &mut App) -> T,
    {
        self.app.update_window(window, update)
    }

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(Entity<T>, &App) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        self.app.read_window(window, read)
    }

    fn background_spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        self.app.background_executor.spawn(future)
    }

    fn read_global<G, R>(&self, callback: impl FnOnce(&G, &App) -> R) -> Result<R>
    where
        G: Global,
    {
        self.app.read_global(callback)
    }
}

impl VisualContext for AsyncWindowContext {
    fn window_handle(&self) -> AnyWindowHandle {
        self.window
    }

    fn new_window_entity<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Window, &mut Context<T>) -> T,
    ) -> Self::Result<Entity<T>> {
        self.window
            .update(self, |_, window, cx| cx.new(|cx| build_entity(window, cx)))
    }

    fn update_window_entity<T: 'static, R>(
        &mut self,
        view: &Entity<T>,
        update: impl FnOnce(&mut T, &mut Window, &mut Context<T>) -> R,
    ) -> Self::Result<R> {
        self.window.update(self, |_, window, cx| {
            view.update(cx, |entity, cx| update(entity, window, cx))
        })
    }

    fn replace_root_view<V>(
        &mut self,
        build_view: impl FnOnce(&mut Window, &mut Context<V>) -> V,
    ) -> Self::Result<Entity<V>>
    where
        V: 'static + Render,
    {
        self.window
            .update(self, |_, window, cx| window.replace_root(cx, build_view))
    }

    fn focus<V>(&mut self, view: &Entity<V>) -> Self::Result<()>
    where
        V: Focusable,
    {
        self.window.update(self, |_, window, cx| {
            view.read(cx).focus_handle(cx).clone().focus(window);
        })
    }
}
