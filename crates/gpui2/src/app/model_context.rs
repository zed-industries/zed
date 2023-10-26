use crate::{
    AppContext, AsyncAppContext, Context, Effect, EntityId, EventEmitter, Handle, MainThread,
    Reference, Subscription, Task, WeakHandle,
};
use derive_more::{Deref, DerefMut};
use futures::FutureExt;
use std::{
    any::{Any, TypeId},
    borrow::{Borrow, BorrowMut},
    future::Future,
};

#[derive(Deref, DerefMut)]
pub struct ModelContext<'a, T> {
    #[deref]
    #[deref_mut]
    app: Reference<'a, AppContext>,
    model_state: WeakHandle<T>,
}

impl<'a, T: 'static> ModelContext<'a, T> {
    pub(crate) fn mutable(app: &'a mut AppContext, model_state: WeakHandle<T>) -> Self {
        Self {
            app: Reference::Mutable(app),
            model_state,
        }
    }

    pub fn entity_id(&self) -> EntityId {
        self.model_state.entity_id
    }

    pub fn handle(&self) -> Handle<T> {
        self.weak_handle()
            .upgrade()
            .expect("The entity must be alive if we have a model context")
    }

    pub fn weak_handle(&self) -> WeakHandle<T> {
        self.model_state.clone()
    }

    pub fn observe<T2: 'static>(
        &mut self,
        handle: &Handle<T2>,
        mut on_notify: impl FnMut(&mut T, Handle<T2>, &mut ModelContext<'_, T>) + Send + 'static,
    ) -> Subscription
    where
        T: 'static + Send,
    {
        let this = self.weak_handle();
        let handle = handle.downgrade();
        self.app.observers.insert(
            handle.entity_id,
            Box::new(move |cx| {
                if let Some((this, handle)) = this.upgrade().zip(handle.upgrade()) {
                    this.update(cx, |this, cx| on_notify(this, handle, cx));
                    true
                } else {
                    false
                }
            }),
        )
    }

    pub fn subscribe<E: 'static + EventEmitter>(
        &mut self,
        handle: &Handle<E>,
        mut on_event: impl FnMut(&mut T, Handle<E>, &E::Event, &mut ModelContext<'_, T>)
            + Send
            + 'static,
    ) -> Subscription
    where
        T: 'static + Send,
    {
        let this = self.weak_handle();
        let handle = handle.downgrade();
        self.app.event_listeners.insert(
            handle.entity_id,
            Box::new(move |event, cx| {
                let event: &E::Event = event.downcast_ref().expect("invalid event type");
                if let Some((this, handle)) = this.upgrade().zip(handle.upgrade()) {
                    this.update(cx, |this, cx| on_event(this, handle, event, cx));
                    true
                } else {
                    false
                }
            }),
        )
    }

    pub fn on_release(
        &mut self,
        mut on_release: impl FnMut(&mut T, &mut AppContext) + Send + 'static,
    ) -> Subscription
    where
        T: 'static,
    {
        self.app.release_listeners.insert(
            self.model_state.entity_id,
            Box::new(move |this, cx| {
                let this = this.downcast_mut().expect("invalid entity type");
                on_release(this, cx);
            }),
        )
    }

    pub fn observe_release<E: 'static>(
        &mut self,
        handle: &Handle<E>,
        mut on_release: impl FnMut(&mut T, &mut E, &mut ModelContext<'_, T>) + Send + 'static,
    ) -> Subscription
    where
        T: Any + Send,
    {
        let this = self.weak_handle();
        self.app.release_listeners.insert(
            handle.entity_id,
            Box::new(move |entity, cx| {
                let entity = entity.downcast_mut().expect("invalid entity type");
                if let Some(this) = this.upgrade() {
                    this.update(cx, |this, cx| on_release(this, entity, cx));
                }
            }),
        )
    }

    pub fn observe_global<G: 'static>(
        &mut self,
        mut f: impl FnMut(&mut T, &mut ModelContext<'_, T>) + Send + 'static,
    ) -> Subscription
    where
        T: 'static + Send,
    {
        let handle = self.weak_handle();
        self.global_observers.insert(
            TypeId::of::<G>(),
            Box::new(move |cx| handle.update(cx, |view, cx| f(view, cx)).is_ok()),
        )
    }

    pub fn on_app_quit<Fut>(
        &mut self,
        mut on_quit: impl FnMut(&mut T, &mut ModelContext<T>) -> Fut + Send + 'static,
    ) -> Subscription
    where
        Fut: 'static + Future<Output = ()> + Send,
        T: 'static + Send,
    {
        let handle = self.weak_handle();
        self.app.quit_observers.insert(
            (),
            Box::new(move |cx| {
                let future = handle.update(cx, |entity, cx| on_quit(entity, cx)).ok();
                async move {
                    if let Some(future) = future {
                        future.await;
                    }
                }
                .boxed()
            }),
        )
    }

    pub fn notify(&mut self) {
        if self
            .app
            .pending_notifications
            .insert(self.model_state.entity_id)
        {
            self.app.pending_effects.push_back(Effect::Notify {
                emitter: self.model_state.entity_id,
            });
        }
    }

    pub fn update_global<G, R>(&mut self, f: impl FnOnce(&mut G, &mut Self) -> R) -> R
    where
        G: 'static + Send,
    {
        let mut global = self.app.lease_global::<G>();
        let result = f(&mut global, self);
        self.app.end_global_lease(global);
        result
    }

    pub fn spawn<Fut, R>(
        &self,
        f: impl FnOnce(WeakHandle<T>, AsyncAppContext) -> Fut + Send + 'static,
    ) -> Task<R>
    where
        T: 'static,
        Fut: Future<Output = R> + Send + 'static,
        R: Send + 'static,
    {
        let this = self.weak_handle();
        self.app.spawn(|cx| f(this, cx))
    }

    pub fn spawn_on_main<Fut, R>(
        &self,
        f: impl FnOnce(WeakHandle<T>, MainThread<AsyncAppContext>) -> Fut + Send + 'static,
    ) -> Task<R>
    where
        Fut: Future<Output = R> + 'static,
        R: Send + 'static,
    {
        let this = self.weak_handle();
        self.app.spawn_on_main(|cx| f(this, cx))
    }
}

impl<'a, T> ModelContext<'a, T>
where
    T: EventEmitter,
    T::Event: Send,
{
    pub fn emit(&mut self, event: T::Event) {
        self.app.pending_effects.push_back(Effect::Emit {
            emitter: self.model_state.entity_id,
            event: Box::new(event),
        });
    }
}

impl<'a, T> Context for ModelContext<'a, T> {
    type EntityContext<'b, 'c, U> = ModelContext<'b, U>;
    type Result<U> = U;

    fn entity<U>(
        &mut self,
        build_entity: impl FnOnce(&mut Self::EntityContext<'_, '_, U>) -> U,
    ) -> Handle<U>
    where
        U: 'static + Send,
    {
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

impl<T> Borrow<AppContext> for ModelContext<'_, T> {
    fn borrow(&self) -> &AppContext {
        &self.app
    }
}

impl<T> BorrowMut<AppContext> for ModelContext<'_, T> {
    fn borrow_mut(&mut self) -> &mut AppContext {
        &mut self.app
    }
}
