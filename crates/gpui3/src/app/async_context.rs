use crate::{
    AnyWindowHandle, AppCell, AppContext, BackgroundExecutor, BorrowAppContext, Context,
    ForegroundExecutor, Global, Model, ModelContext, Render, Reservation, Result, Task, Window,
    WindowHandle,
};
use anyhow::{anyhow, Context as _};
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

    fn update_window<T, F>(&mut self, window: AnyWindowHandle, update: F) -> Result<T>
    where
        F: FnOnce(&mut Window, &mut AppContext) -> T,
    {
        let app = self.app.upgrade().context("app was released")?;
        let mut lock = app.borrow_mut();
        lock.update_window(window, update)
    }

    fn read_window<R>(
        &self,
        window: AnyWindowHandle,
        read: impl FnOnce(&Window, &AppContext) -> R,
    ) -> Result<R> {
        let app = self.app.upgrade().context("app was released")?;
        let lock = app.borrow();
        lock.read_window(window, read)
    }
}

impl AsyncAppContext {
    /// Schedules all windows in the application to be redrawn.
    pub fn refresh(&self) -> Result<()> {
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
    pub fn open_window<T>(
        &self,
        options: crate::WindowOptions,
        builder: impl 'static + Fn(&mut Window, &mut ModelContext<T>) -> T,
    ) -> Result<WindowHandle<T>>
    where
        T: 'static + Render,
    {
        let app = self
            .app
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut lock = app.borrow_mut();
        lock.open_window(options, builder)
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
        &self,
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
