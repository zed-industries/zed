use crate::{AppContext, Context, Effect, EntityId, Handle, Reference, WeakHandle};
use std::{marker::PhantomData, sync::Arc};

pub struct ModelContext<'a, T> {
    app: Reference<'a, AppContext>,
    entity_type: PhantomData<T>,
    entity_id: EntityId,
}

impl<'a, T: Send + Sync + 'static> ModelContext<'a, T> {
    pub(crate) fn mutable(app: &'a mut AppContext, entity_id: EntityId) -> Self {
        Self {
            app: Reference::Mutable(app),
            entity_type: PhantomData,
            entity_id,
        }
    }

    // todo!
    // fn update<R>(&mut self, update: impl FnOnce(&mut T, &mut Self) -> R) -> R {
    //     let mut entity = self
    //         .app
    //         .entities
    //         .get_mut(self.entity_id)
    //         .unwrap()
    //         .take()
    //         .unwrap();
    //     let result = update(entity.downcast_mut::<T>().unwrap(), self);
    //     self.app
    //         .entities
    //         .get_mut(self.entity_id)
    //         .unwrap()
    //         .replace(entity);
    //     result
    // }

    pub fn handle(&self) -> WeakHandle<T> {
        self.app.entities.weak_handle(self.entity_id)
    }

    pub fn observe<E: Send + Sync + 'static>(
        &mut self,
        handle: &Handle<E>,
        on_notify: impl Fn(&mut T, Handle<E>, &mut ModelContext<'_, T>) + Send + Sync + 'static,
    ) {
        let this = self.handle();
        let handle = handle.downgrade();
        self.app
            .observers
            .entry(handle.id)
            .or_default()
            .push(Arc::new(move |cx| {
                if let Some((this, handle)) = this.upgrade(cx).zip(handle.upgrade(cx)) {
                    this.update(cx, |this, cx| on_notify(this, handle, cx));
                    true
                } else {
                    false
                }
            }));
    }

    pub fn notify(&mut self) {
        self.app
            .pending_effects
            .push_back(Effect::Notify(self.entity_id));
    }
}

impl<'a, T: 'static> Context for ModelContext<'a, T> {
    type EntityContext<'b, 'c, U: Send + Sync + 'static> = ModelContext<'b, U>;
    type Result<U> = U;

    fn entity<U: Send + Sync + 'static>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, U>) -> U,
    ) -> Handle<U> {
        self.app.entity(build_entity)
    }

    fn update_entity<U: Send + Sync + 'static, R>(
        &mut self,
        handle: &Handle<U>,
        update: impl FnOnce(&mut U, &mut Self::EntityContext<'_, '_, U>) -> R,
    ) -> R {
        self.app.update_entity(handle, update)
    }
}
