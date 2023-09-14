use super::{px, AppContext, Bounds, Context, EntityId, Handle, Pixels, Style, TaffyLayoutEngine};
use anyhow::Result;
use derive_more::{Deref, DerefMut};
use gpui2::Reference;
use std::{any::Any, marker::PhantomData};

pub struct AnyWindow {}

pub struct Window {
    id: WindowId,
    rem_size: Pixels,
    layout_engine: Box<dyn LayoutEngine>,
    pub(crate) root_view: Option<Box<dyn Any>>,
}

impl Window {
    pub fn new(id: WindowId) -> Window {
        Window {
            id,
            layout_engine: Box::new(TaffyLayoutEngine::new()),
            rem_size: px(16.),
            root_view: None,
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct WindowContext<'a, 'b> {
    #[deref]
    #[deref_mut]
    app: Reference<'a, AppContext>,
    window: Reference<'b, Window>,
}

impl<'a, 'w> WindowContext<'a, 'w> {
    pub(crate) fn mutable(app: &'a mut AppContext, window: &'w mut Window) -> Self {
        Self {
            app: Reference::Mutable(app),
            window: Reference::Mutable(window),
        }
    }

    pub(crate) fn immutable(app: &'a AppContext, window: &'w Window) -> Self {
        Self {
            app: Reference::Immutable(app),
            window: Reference::Immutable(window),
        }
    }

    pub fn request_layout(
        &mut self,
        style: Style,
        children: impl IntoIterator<Item = LayoutId>,
    ) -> Result<LayoutId> {
        self.app.child_layout_buffer.clear();
        self.app.child_layout_buffer.extend(children.into_iter());
        self.window
            .layout_engine
            .request_layout(style, &self.app.child_layout_buffer)
    }

    pub fn layout(&mut self, layout_id: LayoutId) -> Result<Layout> {
        Ok(self
            .window
            .layout_engine
            .layout(layout_id)
            .map(Into::into)?)
    }

    pub fn rem_size(&self) -> Pixels {
        self.window.rem_size
    }

    fn update_window<R>(
        &mut self,
        window_id: WindowId,
        update: impl FnOnce(&mut WindowContext) -> R,
    ) -> Result<R> {
        if window_id == self.window.id {
            Ok(update(self))
        } else {
            self.app.update_window(window_id, update)
        }
    }
}

impl Context for WindowContext<'_, '_> {
    type EntityContext<'a, 'w, T: 'static> = ViewContext<'a, 'w, T>;

    fn entity<T: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T>) -> T,
    ) -> Handle<T> {
        let id = self.entities.insert(None);
        let entity = Box::new(build_entity(&mut ViewContext::mutable(
            &mut *self.app,
            &mut self.window,
            id,
        )));
        self.entities.get_mut(id).unwrap().replace(entity);

        Handle {
            id,
            entity_type: PhantomData,
        }
    }

    fn update_entity<T: 'static, R>(
        &mut self,
        handle: &Handle<T>,
        update: impl FnOnce(&mut T, &mut Self::EntityContext<'_, '_, T>) -> R,
    ) -> R {
        let mut entity = self
            .app
            .entities
            .get_mut(handle.id)
            .unwrap()
            .take()
            .unwrap()
            .downcast::<T>()
            .unwrap();

        let result = update(
            &mut *entity,
            &mut ViewContext::mutable(&mut *self.app, &mut *self.window, handle.id),
        );

        self.app
            .entities
            .get_mut(handle.id)
            .unwrap()
            .replace(entity);

        result
    }
}

#[derive(Deref, DerefMut)]
pub struct ViewContext<'a, 'w, T> {
    #[deref]
    #[deref_mut]
    window_cx: WindowContext<'a, 'w>,
    entity_type: PhantomData<T>,
    entity_id: EntityId,
}

impl<'a, 'w, T: 'static> ViewContext<'a, 'w, T> {
    // fn update<R>(&mut self, update: impl FnOnce(&mut T, &mut Self) -> R) -> R {

    //     self.window_cx.update_entity(handle, update)

    //     let mut entity = self.window_cx.app.entities.remove(&self.entity_id).unwrap();
    //     let result = update(entity.downcast_mut::<T>().unwrap(), self);
    //     self.window_cx
    //         .app
    //         .entities
    //         .insert(self.entity_id, Box::new(entity));
    //     result
    // }

    fn mutable(app: &'a mut AppContext, window: &'w mut Window, entity_id: EntityId) -> Self {
        Self {
            window_cx: WindowContext::mutable(app, window),
            entity_id,
            entity_type: PhantomData,
        }
    }

    fn immutable(app: &'a AppContext, window: &'w Window, entity_id: EntityId) -> Self {
        Self {
            window_cx: WindowContext::immutable(app, window),
            entity_id,
            entity_type: PhantomData,
        }
    }
}

impl<'a, 'w, T: 'static> Context for ViewContext<'a, 'w, T> {
    type EntityContext<'b, 'c, U: 'static> = ViewContext<'b, 'c, U>;

    fn entity<T2: 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, T2>) -> T2,
    ) -> Handle<T2> {
        self.window_cx.entity(build_entity)
    }

    fn update_entity<U: 'static, R>(
        &mut self,
        handle: &Handle<U>,
        update: impl FnOnce(&mut U, &mut Self::EntityContext<'_, '_, U>) -> R,
    ) -> R {
        self.window_cx.update_entity(handle, update)
    }
}

// #[derive(Clone, Copy, Eq, PartialEq, Hash)]
slotmap::new_key_type! { pub struct WindowId; }

pub struct WindowHandle<S> {
    id: WindowId,
    state_type: PhantomData<S>,
}

impl<S> WindowHandle<S> {
    pub fn new(id: WindowId) -> Self {
        WindowHandle {
            id,
            state_type: PhantomData,
        }
    }
}

#[derive(Clone)]
pub struct Layout {
    pub order: u32,
    pub bounds: Bounds<Pixels>,
}

#[derive(Copy, Clone)]
pub struct LayoutId(slotmap::DefaultKey);

pub trait LayoutEngine {
    /// Register a new node on which to perform layout.
    fn request_layout(&mut self, style: Style, children: &[LayoutId]) -> Result<LayoutId>;

    /// Get the layout for the given id.
    fn layout(&mut self, id: LayoutId) -> Result<Layout>;
}
