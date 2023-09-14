use anyhow::{anyhow, Result};
use std::{any::Any, collections::HashMap, marker::PhantomData};

use super::{
    window::{Window, WindowHandle, WindowId},
    Context, LayoutId, Reference, View, WindowContext,
};

pub struct AppContext {
    pub(crate) entity_count: usize,
    pub(crate) entities: HashMap<EntityId, Box<dyn Any>>,
    pub(crate) window_count: usize,
    pub(crate) windows: HashMap<WindowId, Window>,
    // We recycle this memory across layout requests.
    pub(crate) child_layout_buffer: Vec<LayoutId>,
}

impl AppContext {
    pub fn new() -> Self {
        AppContext {
            entity_count: 0,
            entities: HashMap::new(),
            window_count: 0,
            windows: HashMap::new(),
            child_layout_buffer: Default::default(),
        }
    }

    pub fn open_window<S>(
        &mut self,
        build_root_view: impl FnOnce(&mut WindowContext) -> View<S>,
    ) -> WindowHandle<S> {
        let window = Window::new(&mut self.window_count);

        unimplemented!()
    }

    pub(crate) fn update_window<R>(
        &mut self,
        window_id: WindowId,
        update: impl FnOnce(&mut WindowContext) -> R,
    ) -> Result<R> {
        let mut window = self
            .windows
            .remove(&window_id)
            .ok_or_else(|| anyhow!("window not found"))?;
        let result = update(&mut WindowContext::mutable(self, &mut window));
        self.windows.insert(window_id, window);
        Ok(result)
    }
}

impl Context for AppContext {
    type EntityContext<'a, 'w, T: 'static> = ModelContext<'a, T>;

    fn entity<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Handle<T> {
        let entity_id = EntityId::new(&mut self.entity_count);
        let entity = build_entity(&mut ModelContext::mutable(self, entity_id));
        self.entities.insert(entity_id, Box::new(entity));
        Handle::new(entity_id)
    }

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> R {
        let mut entity = self
            .entities
            .remove(&handle.id)
            .unwrap()
            .downcast::<T>()
            .unwrap();
        let result = update(&mut *entity, &mut ModelContext::mutable(self, handle.id));
        self.entities.insert(handle.id, Box::new(entity));
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
        let mut entity = self.app.entities.remove(&self.entity_id).unwrap();
        let result = update(entity.downcast_mut::<T>().unwrap(), self);
        self.app.entities.insert(self.entity_id, Box::new(entity));
        result
    }
}

impl<'a, T: 'static> Context for ModelContext<'a, T> {
    type EntityContext<'b, 'c, U: 'static> = ModelContext<'b, U>;

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

#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub struct EntityId(usize);

impl EntityId {
    pub fn new(entity_count: &mut usize) -> EntityId {
        let id = *entity_count;
        *entity_count += 1;
        Self(id)
    }
}

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
