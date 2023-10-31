use crate::{
    AnyWindowHandle, AppContext, Context, Executor, MainThread, Model, ModelContext, Result, Task,
    View, ViewContext, VisualContext, WindowContext, WindowHandle,
};
use anyhow::Context as _;
use derive_more::{Deref, DerefMut};
use parking_lot::Mutex;
use std::{future::Future, sync::Weak};

#[derive(Clone)]
pub struct AsyncAppContext {
    pub(crate) app: Weak<Mutex<AppContext>>,
    pub(crate) executor: Executor,
}

impl Context for AsyncAppContext {
    type ModelContext<'a, T> = ModelContext<'a, T>;
    type Result<T> = Result<T>;

    fn build_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut Self::ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>>
    where
        T: 'static + Send,
    {
        let app = self.app.upgrade().context("app was released")?;
        let mut lock = app.lock(); // Need this to compile
        Ok(lock.build_model(build_model))
    }

    fn update_model<T: 'static, R>(
        &mut self,
        handle: &Model<T>,
        update: impl FnOnce(&mut T, &mut Self::ModelContext<'_, T>) -> R,
    ) -> Self::Result<R> {
        let app = self.app.upgrade().context("app was released")?;
        let mut lock = app.lock(); // Need this to compile
        Ok(lock.update_model(handle, update))
    }
}

impl AsyncAppContext {
    pub fn refresh(&mut self) -> Result<()> {
        let app = self.app.upgrade().context("app was released")?;
        let mut lock = app.lock(); // Need this to compile
        lock.refresh();
        Ok(())
    }

    pub fn executor(&self) -> &Executor {
        &self.executor
    }

    pub fn update<R>(&self, f: impl FnOnce(&mut AppContext) -> R) -> Result<R> {
        let app = self.app.upgrade().context("app was released")?;
        let mut lock = app.lock();
        Ok(f(&mut *lock))
    }

    pub fn read_window<R>(
        &self,
        handle: AnyWindowHandle,
        update: impl FnOnce(&WindowContext) -> R,
    ) -> Result<R> {
        let app = self.app.upgrade().context("app was released")?;
        let mut app_context = app.lock();
        app_context.read_window(handle, update)
    }

    pub fn update_window<R>(
        &self,
        handle: AnyWindowHandle,
        update: impl FnOnce(&mut WindowContext) -> R,
    ) -> Result<R> {
        let app = self.app.upgrade().context("app was released")?;
        let mut app_context = app.lock();
        app_context.update_window(handle, update)
    }

    pub fn update_window_root<V, R>(
        &mut self,
        handle: &WindowHandle<V>,
        update: impl FnOnce(&mut V, &mut ViewContext<'_, V>) -> R,
    ) -> Result<R>
    where
        V: 'static + Send,
    {
        let app = self.app.upgrade().context("app was released")?;
        let mut app_context = app.lock();
        app_context.update_window_root(handle, update)
    }

    pub fn spawn<Fut, R>(&self, f: impl FnOnce(AsyncAppContext) -> Fut + Send + 'static) -> Task<R>
    where
        Fut: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let this = self.clone();
        self.executor.spawn(async move { f(this).await })
    }

    pub fn spawn_on_main<Fut, R>(
        &self,
        f: impl FnOnce(AsyncAppContext) -> Fut + Send + 'static,
    ) -> Task<R>
    where
        Fut: Future<Output = R> + 'static,
        R: Send + 'static,
    {
        let this = self.clone();
        self.executor.spawn_on_main(|| f(this))
    }

    pub fn run_on_main<R>(
        &self,
        f: impl FnOnce(&mut MainThread<AppContext>) -> R + Send + 'static,
    ) -> Result<Task<R>>
    where
        R: Send + 'static,
    {
        let app = self.app.upgrade().context("app was released")?;
        let mut app_context = app.lock();
        Ok(app_context.run_on_main(f))
    }

    pub fn has_global<G: 'static>(&self) -> Result<bool> {
        let app = self.app.upgrade().context("app was released")?;
        let lock = app.lock(); // Need this to compile
        Ok(lock.has_global::<G>())
    }

    pub fn read_global<G: 'static, R>(&self, read: impl FnOnce(&G, &AppContext) -> R) -> Result<R> {
        let app = self.app.upgrade().context("app was released")?;
        let lock = app.lock(); // Need this to compile
        Ok(read(lock.global(), &lock))
    }

    pub fn try_read_global<G: 'static, R>(
        &self,
        read: impl FnOnce(&G, &AppContext) -> R,
    ) -> Option<R> {
        let app = self.app.upgrade()?;
        let lock = app.lock(); // Need this to compile
        Some(read(lock.try_global()?, &lock))
    }

    pub fn update_global<G: 'static, R>(
        &mut self,
        update: impl FnOnce(&mut G, &mut AppContext) -> R,
    ) -> Result<R> {
        let app = self.app.upgrade().context("app was released")?;
        let mut lock = app.lock(); // Need this to compile
        Ok(lock.update_global(update))
    }
}

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

    pub fn update<R>(&self, update: impl FnOnce(&mut WindowContext) -> R) -> Result<R> {
        self.app.update_window(self.window, update)
    }

    pub fn on_next_frame(&mut self, f: impl FnOnce(&mut WindowContext) + Send + 'static) {
        self.app
            .update_window(self.window, |cx| cx.on_next_frame(f))
            .ok();
    }

    pub fn read_global<G: 'static, R>(
        &self,
        read: impl FnOnce(&G, &WindowContext) -> R,
    ) -> Result<R> {
        self.app
            .read_window(self.window, |cx| read(cx.global(), cx))
    }

    pub fn update_global<G, R>(
        &mut self,
        update: impl FnOnce(&mut G, &mut WindowContext) -> R,
    ) -> Result<R>
    where
        G: 'static,
    {
        self.app
            .update_window(self.window, |cx| cx.update_global(update))
    }

    pub fn spawn<Fut, R>(
        &self,
        f: impl FnOnce(AsyncWindowContext) -> Fut + Send + 'static,
    ) -> Task<R>
    where
        Fut: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let this = self.clone();
        self.executor.spawn(async move { f(this).await })
    }

    pub fn spawn_on_main<Fut, R>(
        &self,
        f: impl FnOnce(AsyncWindowContext) -> Fut + Send + 'static,
    ) -> Task<R>
    where
        Fut: Future<Output = R> + 'static,
        R: Send + 'static,
    {
        let this = self.clone();
        self.executor.spawn_on_main(|| f(this))
    }

    pub fn run_on_main<R>(
        &self,
        f: impl FnOnce(&mut MainThread<WindowContext>) -> R + Send + 'static,
    ) -> Task<Result<R>>
    where
        R: Send + 'static,
    {
        self.update(|cx| cx.run_on_main(f))
            .unwrap_or_else(|error| Task::ready(Err(error)))
    }
}

impl Context for AsyncWindowContext {
    type ModelContext<'a, T> = ModelContext<'a, T>;
    type Result<T> = Result<T>;

    fn build_model<T>(
        &mut self,
        build_model: impl FnOnce(&mut Self::ModelContext<'_, T>) -> T,
    ) -> Result<Model<T>>
    where
        T: 'static + Send,
    {
        self.app
            .update_window(self.window, |cx| cx.build_model(build_model))
    }

    fn update_model<T: 'static, R>(
        &mut self,
        handle: &Model<T>,
        update: impl FnOnce(&mut T, &mut Self::ModelContext<'_, T>) -> R,
    ) -> Result<R> {
        self.app
            .update_window(self.window, |cx| cx.update_model(handle, update))
    }
}

impl VisualContext for AsyncWindowContext {
    type ViewContext<'a, V: 'static> = ViewContext<'a, V>;

    fn build_view<V>(
        &mut self,
        build_view_state: impl FnOnce(&mut Self::ViewContext<'_, V>) -> V,
    ) -> Self::Result<View<V>>
    where
        V: 'static + Send,
    {
        self.app
            .update_window(self.window, |cx| cx.build_view(build_view_state))
    }

    fn update_view<V: 'static, R>(
        &mut self,
        view: &View<V>,
        update: impl FnOnce(&mut V, &mut Self::ViewContext<'_, V>) -> R,
    ) -> Self::Result<R> {
        self.app
            .update_window(self.window, |cx| cx.update_view(view, update))
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_async_app_context_send_sync() {
        fn assert_send_sync<T: Send + Sync>() {}
        assert_send_sync::<AsyncAppContext>();
    }
}
