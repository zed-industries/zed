use crate::{AnyWindowHandle, AppContext, Context, Handle, ModelContext, Result, WindowContext};
use anyhow::anyhow;
use parking_lot::Mutex;
use std::sync::Weak;

#[derive(Clone)]
pub struct AsyncContext(pub(crate) Weak<Mutex<AppContext>>);

impl Context for AsyncContext {
    type EntityContext<'a, 'b, T: Send + Sync + 'static> = ModelContext<'a, T>;
    type Result<T> = Result<T>;

    fn entity<T: Send + Sync + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Result<Handle<T>> {
        let app = self
            .0
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut lock = app.lock();
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
        let mut lock = app.lock();
        Ok(lock.update_entity(handle, update))
    }
}

impl AsyncContext {
    pub fn update_window<T>(
        &self,
        handle: AnyWindowHandle,
        update: impl FnOnce(&mut WindowContext) -> T + Send + Sync,
    ) -> Result<T> {
        let app = self
            .0
            .upgrade()
            .ok_or_else(|| anyhow!("app was released"))?;
        let mut app_context = app.lock();
        app_context.update_window(handle.id, update)
    }
}
