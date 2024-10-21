use crate::{
    AnyView, AnyWindowHandle, AppContext, AsyncAppContext, Context, Effect, Entity, EntityId,
    EventEmitter, Model, Reservation, Subscription, Task, View, WeakModel, WindowContext,
    WindowHandle,
};
use anyhow::Result;
use derive_more::{Deref, DerefMut};
use futures::FutureExt;
use std::{
    any::{Any, TypeId},
    borrow::{Borrow, BorrowMut},
    future::Future,
};

/// The app context, with specialized behavior for the given model.
#[derive(Deref, DerefMut)]
pub struct ModelContext<'a, T> {
    #[deref]
    #[deref_mut]
    app: &'a mut AppContext,
    model_state: WeakModel<T>,
}

impl<'a, T: 'static> ModelContext<'a, T> {
    pub(crate) fn new(app: &'a mut AppContext, model_state: WeakModel<T>) -> Self {
        Self { app, model_state }
    }

    /// The entity id of the model backing this context.
    pub fn entity_id(&self) -> EntityId {
        self.model_state.entity_id
    }

    /// Returns a handle to the model belonging to this context.
    pub fn handle(&self) -> Model<T> {
        self.weak_model()
            .upgrade()
            .expect("The entity must be alive if we have a model context")
    }

    /// Returns a weak handle to the model belonging to this context.
    pub fn weak_model(&self) -> WeakModel<T> {
        self.model_state.clone()
    }

    /// Arranges for the given function to be called whenever [`ModelContext::notify`] or
    /// [`ViewContext::notify`](crate::ViewContext::notify) is called with the given model or view.
    pub fn observe<W, E>(
        &mut self,
        entity: &E,
        mut on_notify: impl FnMut(&mut T, E, &mut ModelContext<'_, T>) + 'static,
    ) -> Subscription
    where
        T: 'static,
        W: 'static,
        E: Entity<W>,
    {
        let this = self.weak_model();
        self.app.observe_internal(entity, move |e, cx| {
            if let Some(this) = this.upgrade() {
                this.update(cx, |this, cx| on_notify(this, e, cx));
                true
            } else {
                false
            }
        })
    }

    /// Subscribe to an event type from another model or view
    pub fn subscribe<T2, E, Evt>(
        &mut self,
        entity: &E,
        mut on_event: impl FnMut(&mut T, E, &Evt, &mut ModelContext<'_, T>) + 'static,
    ) -> Subscription
    where
        T: 'static,
        T2: 'static + EventEmitter<Evt>,
        E: Entity<T2>,
        Evt: 'static,
    {
        let this = self.weak_model();
        self.app.subscribe_internal(entity, move |e, event, cx| {
            if let Some(this) = this.upgrade() {
                this.update(cx, |this, cx| on_event(this, e, event, cx));
                true
            } else {
                false
            }
        })
    }

    /// Register a callback to be invoked when GPUI releases this model.
    pub fn on_release(
        &self,
        on_release: impl FnOnce(&mut T, &mut AppContext) + 'static,
    ) -> Subscription
    where
        T: 'static,
    {
        let (subscription, activate) = self.app.release_listeners.insert(
            self.model_state.entity_id,
            Box::new(move |this, cx| {
                let this = this.downcast_mut().expect("invalid entity type");
                on_release(this, cx);
            }),
        );
        activate();
        subscription
    }

    /// Register a callback to be run on the release of another model or view
    pub fn observe_release<T2, E>(
        &self,
        entity: &E,
        on_release: impl FnOnce(&mut T, &mut T2, &mut ModelContext<'_, T>) + 'static,
    ) -> Subscription
    where
        T: Any,
        T2: 'static,
        E: Entity<T2>,
    {
        let entity_id = entity.entity_id();
        let this = self.weak_model();
        let (subscription, activate) = self.app.release_listeners.insert(
            entity_id,
            Box::new(move |entity, cx| {
                let entity = entity.downcast_mut().expect("invalid entity type");
                if let Some(this) = this.upgrade() {
                    this.update(cx, |this, cx| on_release(this, entity, cx));
                }
            }),
        );
        activate();
        subscription
    }

    /// Register a callback to for updates to the given global
    pub fn observe_global<G: 'static>(
        &mut self,
        mut f: impl FnMut(&mut T, &mut ModelContext<'_, T>) + 'static,
    ) -> Subscription
    where
        T: 'static,
    {
        let handle = self.weak_model();
        let (subscription, activate) = self.global_observers.insert(
            TypeId::of::<G>(),
            Box::new(move |cx| handle.update(cx, |view, cx| f(view, cx)).is_ok()),
        );
        self.defer(move |_| activate());
        subscription
    }

    /// Arrange for the given function to be invoked whenever the application is quit.
    /// The future returned from this callback will be polled for up to [crate::SHUTDOWN_TIMEOUT] until the app fully quits.
    pub fn on_app_quit<Fut>(
        &self,
        mut on_quit: impl FnMut(&mut T, &mut ModelContext<T>) -> Fut + 'static,
    ) -> Subscription
    where
        Fut: 'static + Future<Output = ()>,
        T: 'static,
    {
        let handle = self.weak_model();
        let (subscription, activate) = self.app.quit_observers.insert(
            (),
            Box::new(move |cx| {
                let future = handle.update(cx, |entity, cx| on_quit(entity, cx)).ok();
                async move {
                    if let Some(future) = future {
                        future.await;
                    }
                }
                .boxed_local()
            }),
        );
        activate();
        subscription
    }

    /// Tell GPUI that this model has changed and observers of it should be notified.
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

    /// Spawn the future returned by the given function.
    /// The function is provided a weak handle to the model owned by this context and a context that can be held across await points.
    /// The returned task must be held or detached.
    pub fn spawn<Fut, R>(&self, f: impl FnOnce(WeakModel<T>, AsyncAppContext) -> Fut) -> Task<R>
    where
        T: 'static,
        Fut: Future<Output = R> + 'static,
        R: 'static,
    {
        let this = self.weak_model();
        self.app.spawn(|cx| f(this, cx))
    }
}

impl<'a, T> ModelContext<'a, T> {
    /// Emit an event of the specified type, which can be handled by other entities that have subscribed via `subscribe` methods on their respective contexts.
    pub fn emit<Evt>(&mut self, event: Evt)
    where
        T: EventEmitter<Evt>,
        Evt: 'static,
    {
        self.app.pending_effects.push_back(Effect::Emit {
            emitter: self.model_state.entity_id,
            event_type: TypeId::of::<Evt>(),
            event: Box::new(event),
        });
    }
}

impl<'a, T> Context for ModelContext<'a, T> {
    type Result<U> = U;

    fn new_model<U: 'static>(
        &mut self,
        build_model: impl FnOnce(&mut ModelContext<'_, U>) -> U,
    ) -> Model<U> {
        self.app.new_model(build_model)
    }

    fn reserve_model<U: 'static>(&mut self) -> Reservation<U> {
        self.app.reserve_model()
    }

    fn insert_model<U: 'static>(
        &mut self,
        reservation: Reservation<U>,
        build_model: impl FnOnce(&mut ModelContext<'_, U>) -> U,
    ) -> Self::Result<Model<U>> {
        self.app.insert_model(reservation, build_model)
    }

    fn update_model<U: 'static, R>(
        &mut self,
        handle: &Model<U>,
        update: impl FnOnce(&mut U, &mut ModelContext<'_, U>) -> R,
    ) -> R {
        self.app.update_model(handle, update)
    }

    fn read_model<U, R>(
        &self,
        handle: &Model<U>,
        read: impl FnOnce(&U, &AppContext) -> R,
    ) -> Self::Result<R>
    where
        U: 'static,
    {
        self.app.read_model(handle, read)
    }

    fn update_window<R, F>(&mut self, window: AnyWindowHandle, update: F) -> Result<R>
    where
        F: FnOnce(AnyView, &mut WindowContext<'_>) -> R,
    {
        self.app.update_window(window, update)
    }

    fn read_window<U, R>(
        &self,
        window: &WindowHandle<U>,
        read: impl FnOnce(View<U>, &AppContext) -> R,
    ) -> Result<R>
    where
        U: 'static,
    {
        self.app.read_window(window, read)
    }
}

impl<T> Borrow<AppContext> for ModelContext<'_, T> {
    fn borrow(&self) -> &AppContext {
        self.app
    }
}

impl<T> BorrowMut<AppContext> for ModelContext<'_, T> {
    fn borrow_mut(&mut self) -> &mut AppContext {
        self.app
    }
}
