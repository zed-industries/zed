use crate::{
    current_platform, Context, LayoutId, MainThreadOnly, Platform, Reference, RootView, TextSystem,
    Window, WindowContext, WindowHandle, WindowId,
};
use anyhow::{anyhow, Result};
use futures::Future;
use parking_lot::Mutex;
use slotmap::SlotMap;
use std::{
    any::Any,
    marker::PhantomData,
    sync::{Arc, Weak},
};

#[derive(Clone)]
pub struct App(Arc<Mutex<AppContext>>);

impl App {
    pub fn production() -> Self {
        Self::new(current_platform())
    }

    #[cfg(any(test, feature = "test"))]
    pub fn test() -> Self {
        Self::new(Arc::new(super::TestPlatform::new()))
    }

    fn new(platform: Arc<dyn Platform>) -> Self {
        let dispatcher = platform.dispatcher();
        let text_system = Arc::new(TextSystem::new(platform.text_system()));
        let mut entities = SlotMap::with_key();
        let unit_entity_id = entities.insert(Some(Box::new(()) as Box<dyn Any + Send>));
        Self(Arc::new_cyclic(|this| {
            Mutex::new(AppContext {
                this: this.clone(),
                platform: MainThreadOnly::new(platform, dispatcher),
                text_system,
                unit_entity_id,
                entities,
                windows: SlotMap::with_key(),
                layout_id_buffer: Default::default(),
            })
        }))
    }

    pub fn run<F>(self, on_finish_launching: F)
    where
        F: 'static + FnOnce(&mut AppContext),
    {
        let this = self.clone();
        let platform = self.0.lock().platform.clone();
        platform.borrow_on_main_thread().run(Box::new(move || {
            let cx = &mut *this.0.lock();
            on_finish_launching(cx);
        }));
    }
}

pub struct AppContext {
    this: Weak<Mutex<AppContext>>,
    platform: MainThreadOnly<dyn Platform>,
    text_system: Arc<TextSystem>,
    pub(crate) unit_entity_id: EntityId,
    pub(crate) entities: SlotMap<EntityId, Option<Box<dyn Any + Send>>>,
    pub(crate) windows: SlotMap<WindowId, Option<Window>>,
    // We recycle this memory across layout requests.
    pub(crate) layout_id_buffer: Vec<LayoutId>,
}

impl AppContext {
    pub fn text_system(&self) -> &Arc<TextSystem> {
        &self.text_system
    }

    pub fn with_platform<R: Send + 'static>(
        &mut self,
        f: impl FnOnce(&dyn Platform, &mut Self) -> R + Send + 'static,
    ) -> impl Future<Output = R> {
        let this = self.this.upgrade().unwrap();
        self.platform.read(move |platform| {
            let cx = &mut *this.lock();
            f(platform, cx)
        })
    }

    pub fn open_window<S: 'static + Send>(
        &mut self,
        options: crate::WindowOptions,
        build_root_view: impl FnOnce(&mut WindowContext) -> RootView<S> + Send + 'static,
    ) -> impl Future<Output = WindowHandle<S>> {
        self.with_platform(move |platform, cx| {
            let id = cx.windows.insert(None);
            let handle = WindowHandle::new(id);

            let mut window = Window::new(handle.into(), options, platform);
            let root_view = build_root_view(&mut WindowContext::mutable(cx, &mut window));
            window.root_view.replace(Box::new(root_view));

            cx.windows.get_mut(id).unwrap().replace(window);
            handle
        })
    }

    pub(crate) fn update_window<R>(
        &mut self,
        window_id: WindowId,
        update: impl FnOnce(&mut WindowContext) -> R,
    ) -> Result<R> {
        let mut window = self
            .windows
            .get_mut(window_id)
            .ok_or_else(|| anyhow!("window not found"))?
            .take()
            .unwrap();

        let result = update(&mut WindowContext::mutable(self, &mut window));

        self.windows
            .get_mut(window_id)
            .ok_or_else(|| anyhow!("window not found"))?
            .replace(window);

        Ok(result)
    }
}

impl Context for AppContext {
    type EntityContext<'a, 'w, T: Send + 'static> = ModelContext<'a, T>;

    fn entity<T: Send + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Handle<T> {
        let id = self.entities.insert(None);
        let entity = Box::new(build_entity(&mut ModelContext::mutable(self, id)));
        self.entities.get_mut(id).unwrap().replace(entity);

        Handle::new(id)
    }

    fn update_entity<T: Send + 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> R {
        let mut entity = self
            .entities
            .get_mut(handle.id)
            .unwrap()
            .take()
            .unwrap()
            .downcast::<T>()
            .unwrap();

        let result = update(&mut *entity, &mut ModelContext::mutable(self, handle.id));
        self.entities.get_mut(handle.id).unwrap().replace(entity);
        result
    }
}

pub struct ModelContext<'a, T> {
    app: Reference<'a, AppContext>,
    entity_type: PhantomData<T>,
    entity_id: EntityId,
}

impl<'a, T: 'static> ModelContext<'a, T> {
    pub(crate) fn mutable(app: &'a mut AppContext, entity_id: EntityId) -> Self {
        Self {
            app: Reference::Mutable(app),
            entity_type: PhantomData,
            entity_id,
        }
    }

    fn immutable(app: &'a AppContext, entity_id: EntityId) -> Self {
        Self {
            app: Reference::Immutable(app),
            entity_type: PhantomData,
            entity_id,
        }
    }

    fn update<R>(&mut self, update: impl FnOnce(&mut T, &mut Self) -> R) -> R {
        let mut entity = self
            .app
            .entities
            .get_mut(self.entity_id)
            .unwrap()
            .take()
            .unwrap();
        let result = update(entity.downcast_mut::<T>().unwrap(), self);
        self.app
            .entities
            .get_mut(self.entity_id)
            .unwrap()
            .replace(entity);
        result
    }
}

impl<'a, T: 'static> Context for ModelContext<'a, T> {
    type EntityContext<'b, 'c, U: Send + 'static> = ModelContext<'b, U>;

    fn entity<U: Send + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, U>) -> U,
    ) -> Handle<U> {
        self.app.entity(build_entity)
    }

    fn update_entity<U: Send + 'static, R>(
        &mut self,
        handle: &Handle<U>,
        update: impl FnOnce(&mut U, &mut Self::EntityContext<'_, '_, U>) -> R,
    ) -> R {
        self.app.update_entity(handle, update)
    }
}

pub struct Handle<T> {
    pub(crate) id: EntityId,
    pub(crate) entity_type: PhantomData<T>,
}

slotmap::new_key_type! { pub struct EntityId; }

impl<T: Send + 'static> Handle<T> {
    fn new(id: EntityId) -> Self {
        Self {
            id,
            entity_type: PhantomData,
        }
    }

    /// Update the entity referenced by this handle with the given function.
    ///
    /// The update function receives a context appropriate for its environment.
    /// When updating in an `AppContext`, it receives a `ModelContext`.
    /// When updating an a `WindowContext`, it receives a `ViewContext`.
    pub fn update<C: Context, R>(
        &self,
        cx: &mut C,
        update: impl FnOnce(&mut T, &mut C::EntityContext<'_, '_, T>) -> R,
    ) -> R {
        cx.update_entity(self, update)
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            entity_type: PhantomData,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::AppContext;

    #[test]
    fn test_app_context_send_sync() {
        // This will not compile if `AppContext` does not implement `Send`
        fn assert_send<T: Send>() {}
        assert_send::<AppContext>();
    }
}
