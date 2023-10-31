use crate::{
    AnyWindowHandle, AppContext, AsyncAppContext, Context, Executor, MainThread, Model,
    ModelContext, Result, Task, TestDispatcher, TestPlatform, WindowContext,
};
use parking_lot::Mutex;
use std::{future::Future, sync::Arc};

#[derive(Clone)]
pub struct TestAppContext {
    pub app: Arc<Mutex<AppContext>>,
    pub executor: Executor,
}

impl Context for TestAppContext {
    type ModelContext<'a, T> = ModelContext<'a, T>;
    type Result<T> = T;

    fn build_model<T: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut Self::ModelContext<'_, T>) -> T,
    ) -> Self::Result<Model<T>>
    where
        T: 'static + Send,
    {
        let mut lock = self.app.lock();
        lock.build_model(build_model)
    }

    fn update_model<T: 'static, R>(
        &mut self,
        handle: &Model<T>,
        update: impl FnOnce(&mut T, &mut Self::ModelContext<'_, T>) -> R,
    ) -> Self::Result<R> {
        let mut lock = self.app.lock();
        lock.update_model(handle, update)
    }
}

impl TestAppContext {
    pub fn new(dispatcher: TestDispatcher) -> Self {
        let executor = Executor::new(Arc::new(dispatcher));
        let platform = Arc::new(TestPlatform::new(executor.clone()));
        let asset_source = Arc::new(());
        let http_client = util::http::FakeHttpClient::with_404_response();
        Self {
            app: AppContext::new(platform, asset_source, http_client),
            executor,
        }
    }

    pub fn quit(&self) {
        self.app.lock().quit();
    }

    pub fn refresh(&mut self) -> Result<()> {
        let mut lock = self.app.lock();
        lock.refresh();
        Ok(())
    }

    pub fn executor(&self) -> &Executor {
        &self.executor
    }

    pub fn update<R>(&self, f: impl FnOnce(&mut AppContext) -> R) -> R {
        let mut lock = self.app.lock();
        f(&mut *lock)
    }

    pub fn read_window<R>(
        &self,
        handle: AnyWindowHandle,
        read: impl FnOnce(&WindowContext) -> R,
    ) -> R {
        let mut app_context = self.app.lock();
        app_context.read_window(handle, read).unwrap()
    }

    pub fn update_window<R>(
        &self,
        handle: AnyWindowHandle,
        update: impl FnOnce(&mut WindowContext) -> R,
    ) -> R {
        let mut app = self.app.lock();
        app.update_window(handle, update).unwrap()
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
    ) -> Task<R>
    where
        R: Send + 'static,
    {
        let mut app_context = self.app.lock();
        app_context.run_on_main(f)
    }

    pub fn has_global<G: 'static>(&self) -> bool {
        let lock = self.app.lock();
        lock.has_global::<G>()
    }

    pub fn read_global<G: 'static, R>(&self, read: impl FnOnce(&G, &AppContext) -> R) -> R {
        let lock = self.app.lock();
        read(lock.global(), &lock)
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
    ) -> R {
        let mut lock = self.app.lock();
        lock.update_global(update)
    }

    pub fn to_async(&self) -> AsyncAppContext {
        AsyncAppContext {
            app: Arc::downgrade(&self.app),
            executor: self.executor.clone(),
        }
    }
}
