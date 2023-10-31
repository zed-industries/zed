use crate::{
    AnyWindowHandle, AppContext, AsyncAppContext, Context, EventEmitter, Executor, Handle,
    MainThread, ModelContext, Result, Task, TestDispatcher, TestPlatform, WindowContext,
};
use futures::SinkExt;
use parking_lot::Mutex;
use std::{any::Any, future::Future, sync::Arc};

#[derive(Clone)]
pub struct TestAppContext {
    pub app: Arc<Mutex<AppContext>>,
    pub executor: Executor,
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
        let mut cx = self.app.lock();
        cx.update(f)
    }

    pub fn read_window<R>(
        &self,
        handle: AnyWindowHandle,
        read: impl FnOnce(&WindowContext) -> R,
    ) -> R {
        let mut app_context = self.app.lock();
        app_context.read_window(handle.id, read).unwrap()
    }

    pub fn update_window<R>(
        &self,
        handle: AnyWindowHandle,
        update: impl FnOnce(&mut WindowContext) -> R,
    ) -> R {
        let mut app = self.app.lock();
        app.update_window(handle.id, update).unwrap()
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

    pub fn subscribe<T: 'static + EventEmitter + Send + Sync>(
        &mut self,
        entity: &Handle<T>,
    ) -> futures::channel::mpsc::UnboundedReceiver<T::Event>
    where
        T::Event: 'static + Send + Clone,
    {
        let (mut tx, rx) = futures::channel::mpsc::unbounded();
        entity
            .update(self, |_, cx: &mut ModelContext<T>| {
                cx.subscribe(&entity, move |_, _, event, _| {
                    let _ = tx.send(event.clone());
                })
            })
            .detach();
        rx
    }
}

// pub fn subscribe<T: Entity>(
//     entity: &impl Handle<T>,
//     cx: &mut TestAppContext,
// ) -> Observation<T::Event>
// where
//     T::Event: Clone,
// {
//     let (tx, rx) = smol::channel::unbounded();
//     let _subscription = cx.update(|cx| {
//         cx.subscribe(entity, move |_, event, _| {
//             let _ = smol::block_on(tx.send(event.clone()));
//         })
//     });

//     Observation { rx, _subscription }
// }
