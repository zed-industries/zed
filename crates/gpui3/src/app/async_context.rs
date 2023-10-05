use crate::{
    AnyWindowHandle, AppContext, Context, Handle, ModelContext, Result, ViewContext, WindowContext,
};
use anyhow::anyhow;
use parking_lot::Mutex;
use std::sync::Weak;

#[derive(Clone)]
pub struct AsyncAppContext(pub(crate) Weak<Mutex<AppContext>>);

impl Context for AsyncAppContext {
    type EntityContext<'a, 'w, T: 'static + Send + Sync> = ModelContext<'a, T>;
    type Result<T> = Result<T>;

    fn entity<T: Send + Sync + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Result<Handle<T>> {
        let app = self
            .0
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut lock = app.lock(); // Does not compile without this variable.
        Ok(lock.entity(build_entity))
    }

    fn update_entity<T: Send + Sync + 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> Result<R> {
        let app = self
            .0
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut lock = app.lock(); // Does not compile without this variable.
        Ok(lock.update_entity(handle, update))
    }
}

impl AsyncAppContext {
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
}

#[derive(Clone)]
pub struct AsyncWindowContext {
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
