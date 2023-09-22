use crate::{
    current_platform, Context, LayoutId, Platform, Reference, RootView, TextSystem, Window,
    WindowContext, WindowHandle, WindowId,
};
use anyhow::{anyhow, Result};
use async_task::Runnable;
use futures::Future;
use parking_lot::RwLock;
use slotmap::SlotMap;
use std::{
    any::Any,
    marker::PhantomData,
    sync::{Arc, Weak},
};

#[derive(Clone)]
pub struct App(Arc<RwLock<AppContext<()>>>);

pub struct MainThread;

impl App {
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(AppContext::new(current_platform()))))
    }

    pub fn run<F>(self, on_finish_launching: F)
    where
        F: 'static + FnOnce(&mut AppContext<MainThread>),
    {
        platform.run(Box::new(move || {
            let mut cx = self.0.write();
            on_finish_launching(&mut *cx);
        }));
    }
}

pub struct AppContext<Thread = ()> {
    this: Weak<RwLock<AppContext>>,
    thread: PhantomData<Thread>,
    platform: Box<dyn Platform>,
    text_system: Arc<TextSystem>,
    pub(crate) unit_entity_id: EntityId,
    pub(crate) entities: SlotMap<EntityId, Option<Box<dyn Any>>>,
    pub(crate) windows: SlotMap<WindowId, Option<Window>>,
    // We recycle this memory across layout requests.
    pub(crate) layout_id_buffer: Vec<LayoutId>,
}

impl AppContext<()> {
    pub fn run_on_main<F: 'static, T: 'static>(
        &self,
        to_call: F,
    ) -> Result<T, impl Future<Output = T>>
    where
        F: Fn(&mut AppContext<MainThread>) -> T + Send + Sync,
    {
        let dispatcher = self.platform().dispatcher();
        if dispatcher.is_main_thread() {
        } else {
            let future = async move {
                // let cx = unsafe {  };
            };
            let schedule = move |runnable: Runnable| dispatcher.run_on_main_thread(runnable);

            let (runnable, task) = async_task::spawn_local();
            runnable.schedule();
        }

        let (runnable, task) = async_task::spawn_local(future, schedule);
        runnable.schedule();
        task
    }
}

impl<Thread> AppContext<Thread> {
    pub fn new(platform: Arc<dyn Platform>) -> Self {
        let text_system = Arc::new(TextSystem::new(platform.text_system()));
        let mut entities = SlotMap::with_key();
        let unit_entity_id = entities.insert(Some(Box::new(()) as Box<dyn Any>));

        AppContext {
            thread: PhantomData,
            platform,
            text_system,
            unit_entity_id,
            entities,
            windows: SlotMap::with_key(),
            layout_id_buffer: Default::default(),
        }
    }

    #[cfg(any(test, feature = "test"))]
    pub fn test() -> Self {
        Self::new(Arc::new(super::TestPlatform::new()))
    }

    pub fn text_system(&self) -> &Arc<TextSystem> {
        &self.text_system
    }

    pub fn open_window<S: 'static>(
        &mut self,
        options: crate::WindowOptions,
        build_root_view: impl FnOnce(&mut WindowContext) -> RootView<S>,
    ) -> WindowHandle<S> {
        let id = self.windows.insert(None);
        let handle = WindowHandle::new(id);
        let platform_window = self.platform.open_window(handle.into(), options);

        let mut window = Window::new(id, platform_window);
        let root_view = build_root_view(&mut WindowContext::mutable(self, &mut window));
        window.root_view.replace(Box::new(root_view));

        self.windows.get_mut(id).unwrap().replace(window);
        handle
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

impl AppContext<MainThread> {
    pub fn platform(&self) -> &dyn Platform {
        self.platform.as_ref()
    }
}

impl<Thread: 'static> Context for AppContext<Thread> {
    type EntityContext<'a, 'w, T: 'static> = ModelContext<'a, Thread, T>;

    fn entity<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Handle<T> {
        let id = self.entities.insert(None);
        let entity = Box::new(build_entity(&mut ModelContext::mutable(self, id)));
        self.entities.get_mut(id).unwrap().replace(entity);

        Handle::new(id)
    }

    fn update_entity<T: 'static, R>(
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

pub struct ModelContext<'a, Thread: 'static, T> {
    app: Reference<'a, AppContext<Thread>>,
    entity_type: PhantomData<T>,
    entity_id: EntityId,
}

impl<'a, Thread, T: 'static> ModelContext<'a, Thread, T> {
    pub(crate) fn mutable(app: &'a mut AppContext<Thread>, entity_id: EntityId) -> Self {
        Self {
            app: Reference::Mutable(app),
            entity_type: PhantomData,
            entity_id,
        }
    }

    fn immutable(app: &'a AppContext<Thread>, entity_id: EntityId) -> Self {
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

impl<'a, Thread, T: 'static> Context for ModelContext<'a, Thread, T> {
    type EntityContext<'b, 'c, U: 'static> = ModelContext<'b, Thread, U>;
    fn entity<U: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, U>) -> U,
    ) -> Handle<U> {
        self.app.entity(build_entity)
    }

    fn update_entity<U: 'static, R>(
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

impl<T: 'static> Handle<T> {
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
