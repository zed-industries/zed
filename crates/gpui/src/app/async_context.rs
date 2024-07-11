use crate::{
    AnyView, AnyWindowHandle, AppCell, AppContext, BackgroundExecutor, BorrowAppContext, Context,
    DismissEvent, FocusableView, ForegroundExecutor, Global, Model, ModelContext, PromptLevel,
    Render, Reservation, Result, Task, View, ViewContext, VisualContext, WindowContext,
    WindowHandle,
};
use anyhow::{anyhow, Context as _};
use derive_more::{Deref, DerefMut};
use futures::channel::oneshot;
use std::{future::Future, rc::Weak};

/// An async-friendly version of [AppContext] with a static lifetime so it can be held across `await` points in async code.
/// You're provided with an instance when calling [AppContext::spawn], and you can also create one with [AppContext::to_async].
/// Internally, this holds a weak reference to an `AppContext`, so its methods are fallible to protect against cases where the [AppContext] is dropped.
#[derive(Clone)]
pub struct AsyncAppContext {
    pub(crate) app: Weak<AppCell>,
    pub(crate) background_executor: BackgroundExecutor,
    pub(crate) foreground_executor: ForegroundExecutor,
}

impl Context for AsyncAppContext {
    type Result<T> = Result<T>;

    fn new_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut app = app.borrow_mut();
        Ok(app.new_model(build_model))
    }

    fn reserve_model<T: 'static>(&mut self) -> Result<Reservation<T>> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut app = app.borrow_mut();
        Ok(app.reserve_model())
    }

    fn insert_model<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Result<Model<T>> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut app = app.borrow_mut();
        Ok(app.insert_model(reservation, build_model))
    }

    fn update_model<T: 'static, R>(
        &mut self,
        handle: &Model<T>,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> Self::Result<R> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut app = app.borrow_mut();
        Ok(app.update_model(handle, update))
    }

    fn read_model<T, R>(
        &self,
        handle: &Model<T>,
        callback: impl FnOnce(&T, &AppContext) -> R,
    ) -> Self::Result<R>
    where
        T: 'static,
    {
        let app = self.app.upgrade().context("app was released")?;
        let lock = app.borrow();
        Ok(lock.read_model(handle, callback))
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, f: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut WindowContext<'_>) -> T,
    {
        let app = self.app.upgrade().context("app was released")?;
        let mut lock = app.borrow_mut();
        lock.update_window(window, f)
    }

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(View<T>, &AppContext) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        let app = self.app.upgrade().context("app was released")?;
        let lock = app.borrow();
        lock.read_window(window, read)
    }
}

impl AsyncAppContext {
    /// Schedules all windows in the application to be redrawn.
    pub fn refresh(&mut self) -> Result<()> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut lock = app.borrow_mut();
        lock.refresh();
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
    pub fn update<R>(&self, f: impl FnOnce(&mut AppContext) -> R) -> Result<R> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut lock = app.borrow_mut();
        Ok(f(&mut lock))
    }

    /// Open a window with the given options based on the root view returned by the given function.
    pub fn open_window<V>(
        &self,
        options: crate::WindowOptions,
        build_root_view: impl FnOnce(&mut WindowContext) -> View<V>,
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
    pub fn spawn<Fut, R>(&self, f: impl FnOnce(AsyncAppContext) -> Fut) -> Task<R>
    where
        Fut: Future<Output = R> + 'static,
        R: 'static,
    {
        self.foreground_executor.spawn(f(self.clone()))
    }

    /// Determine whether global state of the specified type has been assigned.
    /// Returns an error if the `AppContext` has been dropped.
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
    /// Returns an error if the `AppContext` has been dropped.
    pub fn read_global<G: Global, R>(&self, read: impl FnOnce(&G, &AppContext) -> R) -> Result<R> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let app = app.borrow_mut();
        Ok(read(app.global(), &app))
    }

    /// Reads the global state of the specified type, passing it to the given callback.
    ///
    /// Similar to [`AsyncAppContext::read_global`], but returns an error instead of panicking
    /// if no state of the specified type has been assigned.
    ///
    /// Returns an error if no state of the specified type has been assigned the `AppContext` has been dropped.
    pub fn try_read_global<G: Global, R>(
        &self,
        read: impl FnOnce(&G, &AppContext) -> R,
    ) -> Option<R> {
        let app = self.app.upgrade()?;
        let app = app.borrow_mut();
        Some(read(app.try_global()?, &app))
    }

    /// A convenience method for [AppContext::update_global]
    /// for updating the global state of the specified type.
    pub fn update_global<G: Global, R>(
        &mut self,
        update: impl FnOnce(&mut G, &mut AppContext) -> R,
    ) -> Result<R> {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut app = app.borrow_mut();
        Ok(app.update(|cx| cx.update_global(update)))
    }
}

/// A cloneable, owned handle to the application context,
/// composed with the window associated with the current task.
#[derive(Clone, Deref, DerefMut)]
pub struct AsyncWindowContext {
    #[deref]
    #[deref_mut]
    app: AsyncAppContext,
    window: AnyWindowHandle,
}

impl AsyncWindowContext {
    pub(crate) fn new(app: AsyncAppContext, window: AnyWindowHandle) -> Self {
        Self { app, window }
    }

    /// Get the handle of the window this context is associated with.
    pub fn window_handle(&self) -> AnyWindowHandle {
        self.window
    }

    /// A convenience method for [`AppContext::update_window`].
    pub fn update<R>(&mut self, update: impl FnOnce(&mut WindowContext) -> R) -> Result<R> {
        self.app.update_window(self.window, |_, cx| update(cx))
    }

    /// A convenience method for [`AppContext::update_window`].
    pub fn update_root<R>(
        &mut self,
        update: impl FnOnce(AnyView, &mut WindowContext) -> R,
    ) -> Result<R> {
        self.app.update_window(self.window, update)
    }

    /// A convenience method for [`WindowContext::on_next_frame`].
    pub fn on_next_frame(&mut self, f: impl FnOnce(&mut WindowContext) + 'static) {
        self.window.update(self, |_, cx| cx.on_next_frame(f)).ok();
    }

    /// A convenience method for [`AppContext::global`].
    pub fn read_global<G: Global, R>(
        &mut self,
        read: impl FnOnce(&G, &WindowContext) -> R,
    ) -> Result<R> {
        self.window.update(self, |_, cx| read(cx.global(), cx))
    }

    /// A convenience method for [`AppContext::update_global`].
    /// for updating the global state of the specified type.
    pub fn update_global<G, R>(
        &mut self,
        update: impl FnOnce(&mut G, &mut WindowContext) -> R,
    ) -> Result<R>
    where
        G: Global,
    {
        self.window.update(self, |_, cx| cx.update_global(update))
    }

    /// Schedule a future to be executed on the main thread. This is used for collecting
    /// the results of background tasks and updating the UI.
    pub fn spawn<Fut, R>(&self, f: impl FnOnce(AsyncWindowContext) -> Fut) -> Task<R>
    where
        Fut: Future<Output = R> + 'static,
        R: 'static,
    {
        self.foreground_executor.spawn(f(self.clone()))
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
            .update(self, |_, cx| cx.prompt(level, message, detail, answers))
            .unwrap_or_else(|_| oneshot::channel().1)
    }
}

impl Context for AsyncWindowContext {
    type Result<T> = Result<T>;

    fn new_model<T>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Result<Model<T>>
    where
        T: 'static,
    {
        self.window.update(self, |_, cx| cx.new_model(build_model))
    }

    fn reserve_model<T: 'static>(&mut self) -> Result<Reservation<T>> {
        self.window.update(self, |_, cx| cx.reserve_model())
    }

    fn insert_model<T: 'static>(
        &mut self,
        reservation: Reservation<T>,
        build_model: impl FnOnce(&mut ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>> {
        self.window
            .update(self, |_, cx| cx.insert_model(reservation, build_model))
    }

    fn update_model<T: 'static, R>(
        &mut self,
        handle: &Model<T>,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> Result<R> {
        self.window
            .update(self, |_, cx| cx.update_model(handle, update))
    }

    fn read_model<T, R>(
        &self,
        handle: &Model<T>,
        read: impl FnOnce(&T, &AppContext) -> R,
    ) -> Self::Result<R>
    where
        T: 'static,
    {
        self.app.read_model(handle, read)
    }

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, update: F) -> Result<T>
    where
        F: FnOnce(AnyView, &mut WindowContext<'_>) -> T,
    {
        self.app.update_window(window, update)
    }

    fn read_window<T, R>(
        &self,
        window: &WindowHandle<T>,
        read: impl FnOnce(View<T>, &AppContext) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        self.app.read_window(window, read)
    }
}

impl VisualContext for AsyncWindowContext {
    fn new_view<V>(
        &mut self,
        build_view_state: impl FnOnce(&mut ViewContext<'_, V>) -> V,
    ) -> Self::Result<View<V>>
    where
        V: 'static + Render,
    {
        self.window
            .update(self, |_, cx| cx.new_view(build_view_state))
    }

    fn update_view<V: 'static, R>(
        &mut self,
        view: &View<V>,
        update: impl FnOnce(&mut V, &mut ViewContext<'_, V>) -> R,
    ) -> Self::Result<R> {
        self.window
            .update(self, |_, cx| cx.update_view(view, update))
    }

    fn replace_root_view<V>(
        &mut self,
        build_view: impl FnOnce(&mut ViewContext<'_, V>) -> V,
    ) -> Self::Result<View<V>>
    where
        V: 'static + Render,
    {
        self.window
            .update(self, |_, cx| cx.replace_root_view(build_view))
    }

    fn focus_view<V>(&mut self, view: &View<V>) -> Self::Result<()>
    where
        V: FocusableView,
    {
        self.window.update(self, |_, cx| {
            view.read(cx).focus_handle(cx).clone().focus(cx);
        })
    }

    fn dismiss_view<V>(&mut self, view: &View<V>) -> Self::Result<()>
    where
        V: crate::ManagedView,
    {
        self.window
            .update(self, |_, cx| view.update(cx, |_, cx| cx.emit(DismissEvent)))
    }
}
