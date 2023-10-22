use crate::{
    AnyWindowHandle, AppContext, Context, Executor, Handle, MainThread, ModelContext, Result, Task,
    ViewContext, WindowContext,
};
use anyhow::anyhow;
use derive_more::{Deref, DerefMut};
use parking_lot::Mutex;
use std::{future::Future, sync::Weak};

#[derive(Clone)]
pub struct AsyncAppContext(pub(crate) Weak<Mutex<AppContext>>);

impl Context for AsyncAppContext {
    type EntityContext<'a, 'w, T: 'static + Send + Sync> = ModelContext<'a, T>;
    type Result<T> = Result<T>;

    fn entity<T: Send + Sync + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Self::Result<Handle<T>> {
        let app = self
            .0
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut lock = app.lock(); // Need this to compile
        Ok(lock.entity(build_entity))
    }

    fn update_entity<T: Send + Sync + 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> Self::Result<R> {
        let app = self
            .0
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut lock = app.lock(); // Need this to compile
        Ok(lock.update_entity(handle, update))
    }
}

impl AsyncAppContext {
    pub fn refresh(&mut self) -> Result<()> {
        let app = self
            .0
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut lock = app.lock(); // Need this to compile
        lock.refresh();
        Ok(())
    }

    pub fn executor(&self) -> Result<Executor> {
        let app = self
            .0
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let lock = app.lock(); // Need this to compile
        Ok(lock.executor().clone())
    }

    pub fn read_window<R>(
        &self,
        handle: AnyWindowHandle,
        update: impl FnOnce(&WindowContext) -> R,
    ) -> Result<R> {
        let app = self
            .0
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut app_context = app.lock();
        app_context.read_window(handle.id, update)
    }

    pub fn update_window<R>(
        &self,
        handle: AnyWindowHandle,
        update: impl FnOnce(&mut WindowContext) -> R,
    ) -> Result<R> {
        let app = self
            .0
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut app_context = app.lock();
        app_context.update_window(handle.id, update)
    }

    pub fn spawn<Fut, R>(
        &self,
        f: impl FnOnce(AsyncAppContext) -> Fut + Send + 'static,
    ) -> Result<Task<R>>
    where
        Fut: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let app = self
            .0
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let app_context = app.lock();
        Ok(app_context.spawn(f))
    }

    pub fn run_on_main<R>(
        &self,
        f: impl FnOnce(&mut MainThread<AppContext>) -> R + Send + 'static,
    ) -> Result<Task<R>>
    where
        R: Send + 'static,
    {
        let app = self
            .0
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut app_context = app.lock();
        Ok(app_context.run_on_main(f))
    }

    pub fn has_global<G: 'static + Send + Sync>(&self) -> Result<bool> {
        let app = self
            .0
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let lock = app.lock(); // Need this to compile
        Ok(lock.has_global::<G>())
    }

    pub fn read_global<G: 'static + Send + Sync, R>(
        &self,
        read: impl FnOnce(&G, &AppContext) -> R,
    ) -> Result<R> {
        let app = self
            .0
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let lock = app.lock(); // Need this to compile
        Ok(read(lock.global(), &lock))
    }

    pub fn try_read_global<G: 'static + Send + Sync, R>(
        &self,
        read: impl FnOnce(&G, &AppContext) -> R,
    ) -> Option<R> {
        let app = self.0.upgrade()?;
        let lock = app.lock(); // Need this to compile
        Some(read(lock.try_global()?, &lock))
    }

    pub fn update_global<G: 'static + Send + Sync, R>(
        &mut self,
        update: impl FnOnce(&mut G, &mut AppContext) -> R,
    ) -> Result<R> {
        let app = self
            .0
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
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

    pub fn read_global<G: 'static + Send + Sync, R>(
        &self,
        read: impl FnOnce(&G, &WindowContext) -> R,
    ) -> Result<R> {
        self.app
            .read_window(self.window, |cx| read(cx.global(), cx))
    }

    pub fn update_global<G: 'static + Send + Sync, R>(
        &mut self,
        update: impl FnOnce(&mut G, &mut WindowContext) -> R,
    ) -> Result<R> {
        self.app
            .update_window(self.window, |cx| cx.update_global(update))
    }
}

impl Context for AsyncWindowContext {
    type EntityContext<'a, 'w, T: 'static + Send + Sync> = ViewContext<'a, 'w, T>;
    type Result<T> = Result<T>;

    fn entity<R: Send + Sync + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, R>) -> R,
    ) -> Result<Handle<R>> {
        self.app
            .update_window(self.window, |cx| cx.entity(build_entity))
    }

    fn update_entity<T: Send + Sync + 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> Result<R> {
        self.app
            .update_window(self.window, |cx| cx.update_entity(handle, update))
    }
}
