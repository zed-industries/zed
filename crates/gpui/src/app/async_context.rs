use crate::{
    AnyView, AnyWindowHandle, App, AppCell, AppContext, BackgroundExecutor, BorrowAppContext,
    Entity, Focusable, ForegroundContext, ForegroundExecutor, Global, PromptLevel, Render,
    Reservation, Result, Task, VisualContext, Window, WindowHandle,
};

use anyhow::Context as _;
use derive_more::{Deref, DerefMut};
use futures::channel::oneshot;
use std::{future::Future, rc::Weak};

use super::{Context, NotClone};

/// An async-friendly version of [App] with a static lifetime so it can be held across `await` points in async code.
/// You're provided with an instance when calling [App::spawn], and you can also create one with [App::to_async].
pub struct AsyncApp {
    pub(crate) app: WeakAsyncApp,
    _not_clone: NotClone,
}

impl AppContext for AsyncApp {
    fn new<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Context<'_, T>) -> T,
    ) -> Entity<T> {
        self.app.new(build_entity).unwrap()
    }

    fn reserve_entity<T: 'static>(&mut self) -> Reservation<T> {
        self.app.reserve_entity().unwrap()
    }

    fn insert_entity<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_entity: impl FnOnce(&mut Context<'_, T>) -> T,
    ) -> Entity<T> {
        self.app.insert_entity(reservation, build_entity).unwrap()
    }

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Entity<T>,
        update: impl FnOnce(&mut T, &mut Context<'_, T>) -> R,
    ) -> R {
        self.app.update_entity(handle, update).unwrap()
    }

    fn read_entity<T, R>(&self, handle: &Entity<T>, callback: impl FnOnce(&T, &App) -> R) -> R
    where
        T: 'static,
    {
        self.app.read_entity(handle, callback).unwrap()
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut Window, &mut App) -> T,
    {
        self.app.update_window(window, f).unwrap()
    }

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(Entity<T>, &App) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        self.app.read_window(window, read).unwrap()
    }

    fn background_spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        self.app.background_spawn(future)
    }

    fn read_global<G, R>(&self, callback: impl FnOnce(&G, &App) -> R) -> R
    where
        G: Global,
    {
        self.app.read_global(callback).unwrap()
    }
}

impl AsyncApp {
    /// Schedules all windows in the application to be redrawn.
    pub fn refresh(&self) {
        self.app.refresh().unwrap()
    }

    /// Get an executor which can be used to spawn futures in the background.
    pub fn background_executor(&self) -> &BackgroundExecutor {
        self.app.background_executor()
    }

    /// Get an executor which can be used to spawn futures in the foreground.
    pub fn foreground_executor(&self) -> &ForegroundExecutor {
        self.app.foreground_executor()
    }

    /// Invoke the given function in the context of the app, then flush any effects produced during its invocation.
    /// Panics if the app has been dropped since this was created
    pub fn update<R>(&self, f: impl FnOnce(&mut App) -> R) -> R {
        self.app.update(f).unwrap()
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
        self.app.open_window(options, build_root_view).unwrap()
    }

    /// Schedule a future to be polled in the background.
    #[track_caller]
    pub fn spawn<Fut, R>(&self, f: impl FnOnce(AsyncApp) -> Fut) -> Task<R>
    where
        Fut: Future<Output = R> + 'static,
        R: 'static,
    {
        self.app.spawn(f)
    }

    /// Determine whether global state of the specified type has been assigned.
    /// Returns an error if the `App` has been dropped.
    pub fn has_global<G: Global>(&self) -> bool {
        self.app.has_global::<G>().unwrap()
    }

    /// Reads the global state of the specified type, passing it to the given callback.
    ///
    /// Panics if no global state of the specified type has been assigned.
    /// Returns an error if the `App` has been dropped.
    pub fn read_global<G: Global, R>(&self, read: impl FnOnce(&G, &App) -> R) -> R {
        self.app.read_global(read).unwrap()
    }

    /// Reads the global state of the specified type, passing it to the given callback.
    ///
    /// Similar to [`AsyncApp::read_global`], but returns an error instead of panicking
    /// if no state of the specified type has been assigned.
    ///
    /// Returns an error if no state of the specified type has been assigned the `App` has been dropped.
    pub fn try_read_global<G: Global, R>(&self, read: impl FnOnce(&G, &App) -> R) -> Option<R> {
        self.app.try_read_global(read).unwrap()
    }

    /// A convenience method for [App::update_global]
    /// for updating the global state of the specified type.
    pub fn update_global<G: Global, R>(&self, update: impl FnOnce(&mut G, &mut App) -> R) -> R {
        self.app.update_global(update).unwrap()
    }
}

/// A cloneable, owned handle to the application context,
/// composed with the window associated with the current task.
#[derive(Deref, DerefMut)]
pub struct AsyncWindowContext {
    #[deref]
    #[deref_mut]
    cx: WeakAsyncWindowContext,
    _not_clone: NotClone,
}

impl AsyncWindowContext {
    pub(crate) fn new_context(app: AsyncApp, window: AnyWindowHandle) -> Self {
        Self {
            cx: WeakAsyncWindowContext::new_context(app.app, window),
            _not_clone: NotClone,
        }
    }

    /// Get the handle of the window this context is associated with.
    pub fn window_handle(&self) -> AnyWindowHandle {
        self.window
    }

    /// A convenience method for [`App::update_window`].
    pub fn update<R>(&mut self, update: impl FnOnce(&mut Window, &mut App) -> R) -> R {
        self.cx.update(update).unwrap()
    }

    /// A convenience method for [`App::update_window`].
    pub fn update_root<R>(
        &mut self,
        update: impl FnOnce(AnyView, &mut Window, &mut App) -> R,
    ) -> R {
        self.cx.update_root(update).unwrap()
    }

    /// A convenience method for [`Window::on_next_frame`].
    pub fn on_next_frame(&mut self, f: impl FnOnce(&mut Window, &mut App) + 'static) {
        self.cx.on_next_frame(f).unwrap()
    }

    /// A convenience method for [`App::global`].
    pub fn read_global<G: Global, R>(&mut self, read: impl FnOnce(&G, &Window, &App) -> R) -> R {
        self.cx.read_global(read).unwrap()
    }

    /// A convenience method for [`App::update_global`].
    /// for updating the global state of the specified type.
    pub fn update_global<G, R>(
        &mut self,
        update: impl FnOnce(&mut G, &mut Window, &mut App) -> R,
    ) -> R
    where
        G: Global,
    {
        self.cx.update_global(update).unwrap()
    }

    /// Schedule a future to be executed on the main thread. This is used for collecting
    /// the results of background tasks and updating the UI.
    #[track_caller]
    pub fn spawn<Fut, R>(&self, f: impl FnOnce(AsyncWindowContext) -> Fut) -> Task<R>
    where
        Fut: Future<Output = R> + 'static,
        R: 'static,
    {
        self.cx.spawn(f)
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
            .unwrap()
    }
}

impl AppContext for AsyncWindowContext {
    fn new<T>(&mut self, build_entity: impl FnOnce(&mut Context<'_, T>) -> T) -> Entity<T>
    where
        T: 'static,
    {
        self.cx.new(build_entity).unwrap()
    }

    fn reserve_entity<T: 'static>(&mut self) -> Reservation<T> {
        self.cx.reserve_entity().unwrap()
    }

    fn insert_entity<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_entity: impl FnOnce(&mut Context<'_, T>) -> T,
    ) -> Entity<T> {
        self.cx.insert_entity(reservation, build_entity).unwrap()
    }

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Entity<T>,
        update: impl FnOnce(&mut T, &mut Context<'_, T>) -> R,
    ) -> R {
        self.cx.update_entity(handle, update).unwrap()
    }

    fn read_entity<T, R>(&self, handle: &Entity<T>, read: impl FnOnce(&T, &App) -> R) -> R
    where
        T: 'static,
    {
        self.cx.read_entity(handle, read).unwrap()
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, update: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut Window, &mut App) -> T,
    {
        self.cx.update_window(window, update)
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
        self.app.read_global(callback).unwrap()
    }
}

impl VisualContext for AsyncWindowContext {
    fn window_handle(&self) -> AnyWindowHandle {
        self.window
    }

    fn new_window_entity<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Window, &mut Context<T>) -> T,
    ) -> Entity<T> {
        self.cx.new_window_entity(build_entity).unwrap()
    }

    fn update_window_entity<T: 'static, R>(
        &mut self,
        view: &Entity<T>,
        update: impl FnOnce(&mut T, &mut Window, &mut Context<T>) -> R,
    ) -> R {
        self.cx.update_window_entity(view, update).unwrap()
    }

    fn replace_root_view<V>(
        &mut self,
        build_view: impl FnOnce(&mut Window, &mut Context<V>) -> V,
    ) -> Entity<V>
    where
        V: 'static + Render,
    {
        self.cx.replace_root_view(build_view).unwrap()
    }

    fn focus<V>(&mut self, view: &Entity<V>)
    where
        V: Focusable,
    {
        self.cx.focus(view).unwrap()
    }
}

/// An async-friendly version of [App] with a static lifetime so it can be held across `await` points in async code.
/// You're provided with an instance when calling [App::spawn], and you can also create one with [App::to_async].
/// Internally, this holds a weak reference to an `App`, so its methods are fallible to protect against cases where the [App] is dropped.
pub struct WeakAsyncApp {
    pub(crate) app: Weak<AppCell>,
    pub(crate) background_executor: BackgroundExecutor,
    pub(crate) foreground_executor: ForegroundExecutor,
}

impl Clone for WeakAsyncApp {
    fn clone(&self) -> Self {
        Self {
            app: self.app.clone(),
            background_executor: self.background_executor.clone(),
            foreground_executor: self.foreground_executor.clone(),
        }
    }
}

impl WeakAsyncApp {
    pub(crate) fn upgrade(self) -> AsyncApp {
        AsyncApp {
            app: self,
            _not_clone: NotClone,
        }
    }

    fn new<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Context<'_, T>) -> T,
    ) -> Result<Entity<T>> {
        let app = self.app.upgrade().context("App dropped")?;
        let mut app = app.borrow_mut();
        Ok(app.new(build_entity))
    }

    fn reserve_entity<T: 'static>(&mut self) -> Result<Reservation<T>> {
        let app = self.app.upgrade().context("App dropped")?;
        let mut app = app.borrow_mut();
        Ok(app.reserve_entity())
    }

    fn insert_entity<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_entity: impl FnOnce(&mut Context<'_, T>) -> T,
    ) -> Result<Entity<T>> {
        let app = self.app.upgrade().context("App dropped")?;
        let mut app = app.borrow_mut();
        Ok(app.insert_entity(reservation, build_entity))
    }

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Entity<T>,
        update: impl FnOnce(&mut T, &mut Context<'_, T>) -> R,
    ) -> Result<R> {
        let app = self.app.upgrade().context("App dropped")?;
        let mut app = app.borrow_mut();
        Ok(app.update_entity(handle, update))
    }

    fn read_entity<T, R>(
        &self,
        handle: &Entity<T>,
        callback: impl FnOnce(&T, &App) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        let app = self.app.upgrade().context("App dropped")?;
        let lock = app.borrow();
        Ok(lock.read_entity(handle, callback))
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Result<Result<T>>
    where
        F: FnOnce(AnyView, &mut Window, &mut App) -> T,
    {
        let app = self.app.upgrade().context("App dropped")?;
        let mut lock = app.borrow_mut();
        Ok(lock.update_window(window, f))
    }

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(Entity<T>, &App) -> R,
    ) -> Result<Result<R>>
    where
        T: 'static,
    {
        let app = self.app.upgrade().context("App dropped")?;
        let lock = app.borrow();
        Ok(lock.read_window(window, read))
    }

    fn background_spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        self.background_executor.spawn(future)
    }

    fn read_global<G, R>(&self, callback: impl FnOnce(&G, &App) -> R) -> Result<R>
    where
        G: Global,
    {
        let app = self.app.upgrade().context("App dropped")?;
        let mut lock = app.borrow_mut();
        Ok(lock.update(|this| this.read_global(callback)))
    }

    /// Schedules all windows in the application to be redrawn.
    pub fn refresh(&self) -> Result<()> {
        let app = self.app.upgrade().context("App dropped")?;
        let mut lock = app.borrow_mut();
        Ok(lock.refresh_windows())
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
    /// Panics if the app has been dropped since this was created
    pub fn update<R>(&self, f: impl FnOnce(&mut App) -> R) -> Result<R> {
        let app = self.app.upgrade().context("App dropped")?;
        let mut lock = app.borrow_mut();
        Ok(f(&mut lock))
    }

    /// Open a window with the given options based on the root view returned by the given function.
    pub fn open_window<V>(
        &self,
        options: crate::WindowOptions,
        build_root_view: impl FnOnce(&mut Window, &mut App) -> Entity<V>,
    ) -> Result<Result<WindowHandle<V>>>
    where
        V: 'static + Render,
    {
        let app = self.app.upgrade().context("App dropped")?;
        let mut lock = app.borrow_mut();
        Ok(lock.open_window(options, build_root_view))
    }

    /// Schedule a future to be polled in the background.
    #[track_caller]
    pub fn spawn<Fut, R>(&self, f: impl FnOnce(AsyncApp) -> Fut) -> Task<R>
    where
        Fut: Future<Output = R> + 'static,
        R: 'static,
    {
        self.foreground_executor
            .spawn_with_context(ForegroundContext::app(&self.app), f(self.clone().upgrade()))
    }

    /// Determine whether global state of the specified type has been assigned.
    /// Returns an error if the `App` has been dropped.
    pub fn has_global<G: Global>(&self) -> Result<bool> {
        let app = self.app.upgrade().context("App dropped")?;
        let app = app.borrow_mut();
        Ok(app.has_global::<G>())
    }

    /// Reads the global state of the specified type, passing it to the given callback.
    ///
    /// Similar to [`AsyncApp::read_global`], but returns an error instead of panicking
    /// if no state of the specified type has been assigned.
    ///
    /// Returns an error if no state of the specified type has been assigned the `App` has been dropped.
    pub fn try_read_global<G: Global, R>(
        &self,
        read: impl FnOnce(&G, &App) -> R,
    ) -> Result<Option<R>> {
        let app = self.app.upgrade().context("App dropped")?;
        let app = app.borrow_mut();
        let Some(global) = app.try_global::<G>() else {
            return Ok(None);
        };
        Ok(Some(read(global, &app)))
    }

    /// A convenience method for [App::update_global]
    /// for updating the global state of the specified type.
    pub fn update_global<G: Global, R>(
        &self,
        update: impl FnOnce(&mut G, &mut App) -> R,
    ) -> Result<R> {
        let app = self.app.upgrade().context("App dropped")?;
        let mut app = app.borrow_mut();
        Ok(app.update(|cx| cx.update_global(update)))
    }
}

/// A cloneable, owned handle to the application context,
/// composed with the window associated with the current task.
#[derive(Clone, Deref, DerefMut)]
pub struct WeakAsyncWindowContext {
    #[deref]
    #[deref_mut]
    app: WeakAsyncApp,
    window: AnyWindowHandle,
}

impl WeakAsyncWindowContext {
    pub(crate) fn new_context(app: WeakAsyncApp, window: AnyWindowHandle) -> Self {
        Self { app, window }
    }

    pub(crate) fn upgrade(self) -> AsyncWindowContext {
        AsyncWindowContext {
            cx: self,
            _not_clone: NotClone,
        }
    }

    /// Get the handle of the window this context is associated with.
    pub fn window_handle(&self) -> AnyWindowHandle {
        self.window
    }

    /// A convenience method for [`App::update_window`].
    pub fn update<R>(&mut self, update: impl FnOnce(&mut Window, &mut App) -> R) -> Result<R> {
        crate::Flatten::flatten(
            self.app
                .update_window(self.window, |_, window, cx| update(window, cx)),
        )
    }

    /// A convenience method for [`App::update_window`].
    pub fn update_root<R>(
        &mut self,
        update: impl FnOnce(AnyView, &mut Window, &mut App) -> R,
    ) -> Result<R> {
        crate::Flatten::flatten(self.app.update_window(self.window, update))
    }

    /// A convenience method for [`Window::on_next_frame`].
    pub fn on_next_frame(&mut self, f: impl FnOnce(&mut Window, &mut App) + 'static) -> Result<()> {
        self.update_window(self.window, |_, window, _| window.on_next_frame(f))
    }

    /// A convenience method for [`App::global`].
    pub fn read_global<G: Global, R>(
        &mut self,
        read: impl FnOnce(&G, &Window, &App) -> R,
    ) -> Result<R> {
        self.update_window(self.window, |_, window, cx| read(cx.global(), window, cx))
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
        self.update_window(self.window, |_, window, cx| {
            cx.update_global(|global, cx| update(global, window, cx))
        })
    }

    /// Schedule a future to be executed on the main thread. This is used for collecting
    /// the results of background tasks and updating the UI.
    #[track_caller]
    pub fn spawn<Fut, R>(&self, f: impl FnOnce(AsyncWindowContext) -> Fut) -> Task<R>
    where
        Fut: Future<Output = R> + 'static,
        R: 'static,
    {
        self.foreground_executor.spawn_with_context(
            ForegroundContext::window(&self.app.app, self.window.id),
            f(self.clone().upgrade()),
        )
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
    ) -> Result<oneshot::Receiver<usize>> {
        self.update_window(self.window, |_, window, cx| {
            window.prompt(level, message, detail, answers, cx)
        })
    }

    fn new<T>(&mut self, build_entity: impl FnOnce(&mut Context<'_, T>) -> T) -> Result<Entity<T>>
    where
        T: 'static,
    {
        self.update_window(self.window, |_, _, cx| cx.new(build_entity))
    }

    fn reserve_entity<T: 'static>(&mut self) -> Result<Reservation<T>> {
        self.update_window(self.window, |_, _, cx| cx.reserve_entity())
    }

    fn insert_entity<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_entity: impl FnOnce(&mut Context<'_, T>) -> T,
    ) -> Result<Entity<T>> {
        self.update_window(self.window, |_, _, cx| {
            cx.insert_entity(reservation, build_entity)
        })
    }

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Entity<T>,
        update: impl FnOnce(&mut T, &mut Context<'_, T>) -> R,
    ) -> Result<R> {
        self.update_window(self.window, |_, _, cx| cx.update_entity(handle, update))
    }

    fn read_entity<T, R>(&self, handle: &Entity<T>, read: impl FnOnce(&T, &App) -> R) -> Result<R>
    where
        T: 'static,
    {
        self.app.read_entity(handle, read)
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, update: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut Window, &mut App) -> T,
    {
        crate::Flatten::flatten(self.app.update_window(window, update))
    }

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(Entity<T>, &App) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        crate::Flatten::flatten(self.app.read_window(window, read))
    }

    fn background_spawn<R>(&self, future: impl Future<Output = R> + Send + 'static) -> Task<R>
    where
        R: Send + 'static,
    {
        self.app.background_executor.spawn(future)
    }

    fn new_window_entity<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Window, &mut Context<T>) -> T,
    ) -> Result<Entity<T>> {
        self.update_window(self.window, |_, window, cx| {
            cx.new(|cx| build_entity(window, cx))
        })
    }

    fn update_window_entity<T: 'static, R>(
        &mut self,
        view: &Entity<T>,
        update: impl FnOnce(&mut T, &mut Window, &mut Context<T>) -> R,
    ) -> Result<R> {
        self.update(|window, cx| view.update(cx, |entity, cx| update(entity, window, cx)))
    }

    fn replace_root_view<V>(
        &mut self,
        build_view: impl FnOnce(&mut Window, &mut Context<V>) -> V,
    ) -> Result<Entity<V>>
    where
        V: 'static + Render,
    {
        self.update_window(self.window, |_, window, cx| {
            window.replace_root(cx, build_view)
        })
    }

    fn focus<V>(&mut self, view: &Entity<V>) -> Result<()>
    where
        V: Focusable,
    {
        self.update_window(self.window, |_, window, cx| {
            view.read(cx).focus_handle(cx).clone().focus(window)
        })
    }
}
