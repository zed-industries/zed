use crate::{
    AnyWindowHandle, AppContext, AsyncAppContext, Context, Executor, Handle, MainThread,
    ModelContext, Result, Task, WindowContext,
};
use parking_lot::Mutex;
use std::{any::Any, future::Future, sync::Arc};

#[derive(Clone)]
pub struct TestAppContext {
    pub(crate) app: Arc<Mutex<AppContext>>,
    pub(crate) executor: Executor,
}

impl Context for TestAppContext {
    type EntityContext<'a, 'w, T> = ModelContext<'a, T>;
    type Result<T> = T;

    fn entity<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Self::Result<Handle<T>>
    where
        T: Any + Send + Sync,
    {
        let mut lock = self.app.lock();
        lock.entity(build_entity)
    }

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> Self::Result<R> {
        let mut lock = self.app.lock();
        lock.update_entity(handle, update)
    }
}

impl TestAppContext {
    pub fn refresh(&mut self) -> Result<()> {
        let mut lock = self.app.lock();
        lock.refresh();
        Ok(())
    }

    pub fn executor(&self) -> &Executor {
        &self.executor
    }

    pub fn update<R>(&self, f: impl FnOnce(&mut AppContext) -> R) -> Result<R> {
        let mut lock = self.app.lock();
        Ok(f(&mut *lock))
    }

    pub fn read_window<R>(
        &self,
        handle: AnyWindowHandle,
        update: impl FnOnce(&WindowContext) -> R,
    ) -> Result<R> {
        let mut app_context = self.app.lock();
        app_context.read_window(handle.id, update)
    }

    pub fn update_window<R>(
        &self,
        handle: AnyWindowHandle,
        update: impl FnOnce(&mut WindowContext) -> R,
    ) -> Result<R> {
        let mut app = self.app.lock();
        app.update_window(handle.id, update)
    }

    pub fn spawn<Fut, R>(&self, f: impl FnOnce(AsyncAppContext) -> Fut + Send + 'static) -> Task<R>
    where
        Fut: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let cx = self.to_async();
        self.executor.spawn(async move { f(cx).await })
    }

    pub fn spawn_on_main<Fut, R>(
        &self,
        f: impl FnOnce(AsyncAppContext) -> Fut + Send + 'static,
    ) -> Task<R>
    where
        Fut: Future<Output = R> + 'static,
        R: Send + 'static,
    {
        let cx = self.to_async();
        self.executor.spawn_on_main(|| f(cx))
    }

    pub fn run_on_main<R>(
        &self,
        f: impl FnOnce(&mut MainThread<AppContext>) -> R + Send + 'static,
    ) -> Result<Task<R>>
    where
        R: Send + 'static,
    {
        let mut app_context = self.app.lock();
        Ok(app_context.run_on_main(f))
    }

    pub fn has_global<G: 'static>(&self) -> Result<bool> {
        let lock = self.app.lock();
        Ok(lock.has_global::<G>())
    }

    pub fn read_global<G: 'static, R>(&self, read: impl FnOnce(&G, &AppContext) -> R) -> Result<R> {
        let lock = self.app.lock();
        Ok(read(lock.global(), &lock))
    }

    pub fn try_read_global<G: 'static, R>(
        &self,
        read: impl FnOnce(&G, &AppContext) -> R,
    ) -> Option<R> {
        let lock = self.app.lock();
        Some(read(lock.try_global()?, &lock))
    }

    pub fn update_global<G: 'static, R>(
        &mut self,
        update: impl FnOnce(&mut G, &mut AppContext) -> R,
    ) -> Result<R> {
        let mut lock = self.app.lock();
        Ok(lock.update_global(update))
    }

    fn to_async(&self) -> AsyncAppContext {
        AsyncAppContext {
            app: Arc::downgrade(&self.app),
            executor: self.executor.clone(),
        }
    }
}
