use crate::{
    AnyView, AnyWindowHandle, App, AppContext, AsyncAppContext, DispatchPhase, Effect, EntityId,
    EventEmitter, FocusHandle, FocusOutEvent, Focusable, Global, KeystrokeObserver, Reservation,
    SubscriberSet, Subscription, Task, WeakEntity, WeakFocusHandle, Window, WindowHandle,
};
use anyhow::Result;
use derive_more::{Deref, DerefMut};
use futures::FutureExt;
use std::{
    any::{Any, TypeId},
    borrow::{Borrow, BorrowMut},
    future::Future,
    sync::Arc,
};

use super::{AsyncWindowContext, Entity, KeystrokeEvent};

/// The app context, with specialized behavior for the given model.
#[derive(Deref, DerefMut)]
pub struct Context<'a, T> {
    #[deref]
    #[deref_mut]
    app: &'a mut App,
    model_state: WeakEntity<T>,
}

impl<'a, T: 'static> Context<'a, T> {
    pub(crate) fn new_context(app: &'a mut App, model_state: WeakEntity<T>) -> Self {
        Self { app, model_state }
    }

    /// The entity id of the model backing this context.
    pub fn entity_id(&self) -> EntityId {
        self.model_state.entity_id
    }

    /// Returns a handle to the model belonging to this context.
    pub fn model(&self) -> Entity<T> {
        self.weak_model()
            .upgrade()
            .expect("The entity must be alive if we have a model context")
    }

    /// Returns a weak handle to the model belonging to this context.
    pub fn weak_model(&self) -> WeakEntity<T> {
        self.model_state.clone()
    }

    /// Arranges for the given function to be called whenever [`ModelContext::notify`] or
    /// [`ViewContext::notify`](crate::ViewContext::notify) is called with the given model or view.
    pub fn observe<W>(
        &mut self,
        entity: &Entity<W>,
        mut on_notify: impl FnMut(&mut T, Entity<W>, &mut Context<'_, T>) + 'static,
    ) -> Subscription
    where
        T: 'static,
        W: 'static,
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
    pub fn subscribe<T2, Evt>(
        &mut self,
        entity: &Entity<T2>,
        mut on_event: impl FnMut(&mut T, Entity<T2>, &Evt, &mut Context<'_, T>) + 'static,
    ) -> Subscription
    where
        T: 'static,
        T2: 'static + EventEmitter<Evt>,
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
    pub fn on_release(&self, on_release: impl FnOnce(&mut T, &mut App) + 'static) -> Subscription
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
    pub fn observe_release<T2>(
        &self,
        entity: &Entity<T2>,
        on_release: impl FnOnce(&mut T, &mut T2, &mut Context<'_, T>) + 'static,
    ) -> Subscription
    where
        T: Any,
        T2: 'static,
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
        mut f: impl FnMut(&mut T, &mut Context<'_, T>) + 'static,
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
        mut on_quit: impl FnMut(&mut T, &mut Context<T>) -> Fut + 'static,
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
        self.app.notify(self.model_state.entity_id);
    }

    /// Spawn the future returned by the given function.
    /// The function is provided a weak handle to the model owned by this context and a context that can be held across await points.
    /// The returned task must be held or detached.
    pub fn spawn<Fut, R>(&self, f: impl FnOnce(WeakEntity<T>, AsyncAppContext) -> Fut) -> Task<R>
    where
        T: 'static,
        Fut: Future<Output = R> + 'static,
        R: 'static,
    {
        let this = self.weak_model();
        self.app.spawn(|cx| f(this, cx))
    }

    /// Convenience method for accessing view state in an event callback.
    ///
    /// Many GPUI callbacks take the form of `Fn(&E, &mut Window, &mut AppContext)`,
    /// but it's often useful to be able to access view state in these
    /// callbacks. This method provides a convenient way to do so.
    pub fn listener<E: ?Sized>(
        &self,
        f: impl Fn(&mut T, &E, &mut Window, &mut Context<T>) + 'static,
    ) -> impl Fn(&E, &mut Window, &mut App) + 'static {
        let view = self.model().downgrade();
        move |e: &E, window: &mut Window, cx: &mut App| {
            view.update(cx, |view, cx| f(view, e, window, cx)).ok();
        }
    }

    /// Focus the given view in the given window. View type is required to implement Focusable.
    pub fn focus_view<W: Focusable>(&mut self, view: &Entity<W>, window: &mut Window) {
        window.focus(&view.focus_handle(self));
    }

    /// Sets a given callback to be run on the next frame.
    pub fn on_next_frame(
        &self,
        window: &mut Window,
        f: impl FnOnce(&mut T, &mut Window, &mut Context<T>) + 'static,
    ) where
        T: 'static,
    {
        let view = self.model();
        window.on_next_frame(move |window, cx| view.update(cx, |view, cx| f(view, window, cx)));
    }

    /// Schedules the given function to be run at the end of the current effect cycle, allowing entities
    /// that are currently on the stack to be returned to the app.
    pub fn defer_in(
        &mut self,
        window: &Window,
        f: impl FnOnce(&mut T, &mut Window, &mut Context<T>) + 'static,
    ) {
        let view = self.model();
        window.defer(self, move |window, cx| {
            view.update(cx, |view, cx| f(view, window, cx))
        });
    }

    /// Observe another model or view for changes to its state, as tracked by [`ModelContext::notify`].
    pub fn observe_in<V2>(
        &mut self,
        observed: &Entity<V2>,
        window: &mut Window,
        mut on_notify: impl FnMut(&mut T, Entity<V2>, &mut Window, &mut Context<'_, T>) + 'static,
    ) -> Subscription
    where
        V2: 'static,
        T: 'static,
    {
        let observed_id = observed.entity_id();
        let observed = observed.downgrade();
        let window_handle = window.handle;
        let observer = self.weak_model();
        self.new_observer(
            observed_id,
            Box::new(move |cx| {
                window_handle
                    .update(cx, |_, window, cx| {
                        if let Some((observer, observed)) =
                            observer.upgrade().zip(observed.upgrade())
                        {
                            observer.update(cx, |observer, cx| {
                                on_notify(observer, observed, window, cx);
                            });
                            true
                        } else {
                            false
                        }
                    })
                    .unwrap_or(false)
            }),
        )
    }

    /// Subscribe to events emitted by another model or view.
    /// The entity to which you're subscribing must implement the [`EventEmitter`] trait.
    /// The callback will be invoked with a reference to the current view, a handle to the emitting entity (either a [`View`] or [`Model`]), the event, and a view context for the current view.
    pub fn subscribe_in<Emitter, Evt>(
        &mut self,
        emitter: &Entity<Emitter>,
        window: &Window,
        mut on_event: impl FnMut(&mut T, &Entity<Emitter>, &Evt, &mut Window, &mut Context<'_, T>)
            + 'static,
    ) -> Subscription
    where
        Emitter: EventEmitter<Evt>,
        Evt: 'static,
    {
        let emitter = emitter.downgrade();
        let window_handle = window.handle;
        let subscriber = self.weak_model();
        self.new_subscription(
            emitter.entity_id(),
            (
                TypeId::of::<Evt>(),
                Box::new(move |event, cx| {
                    window_handle
                        .update(cx, |_, window, cx| {
                            if let Some((subscriber, emitter)) =
                                subscriber.upgrade().zip(emitter.upgrade())
                            {
                                let event = event.downcast_ref().expect("invalid event type");
                                subscriber.update(cx, |subscriber, cx| {
                                    on_event(subscriber, &emitter, event, window, cx);
                                });
                                true
                            } else {
                                false
                            }
                        })
                        .unwrap_or(false)
                }),
            ),
        )
    }

    /// Register a callback to be invoked when the view is released.
    ///
    /// The callback receives a handle to the view's window. This handle may be
    /// invalid, if the window was closed before the view was released.
    pub fn on_release_in(
        &mut self,
        window: &Window,
        on_release: impl FnOnce(&mut T, AnyWindowHandle, &mut App) + 'static,
    ) -> Subscription {
        let window_handle = window.handle;
        let (subscription, activate) = self.release_listeners.insert(
            self.entity_id(),
            Box::new(move |this, cx| {
                let this = this.downcast_mut().expect("invalid entity type");
                on_release(this, window_handle, cx)
            }),
        );
        activate();
        subscription
    }

    /// Register a callback to be invoked when the given Model or View is released.
    pub fn observe_release_in<V2>(
        &self,
        observed: &Entity<V2>,
        window: &Window,
        mut on_release: impl FnMut(&mut T, &mut V2, &mut Window, &mut Context<'_, T>) + 'static,
    ) -> Subscription
    where
        T: 'static,
        V2: 'static,
    {
        let observer = self.weak_model();
        let window_handle = window.handle;
        let (subscription, activate) = self.release_listeners.insert(
            observed.entity_id(),
            Box::new(move |observed, cx| {
                let observed = observed
                    .downcast_mut()
                    .expect("invalid observed entity type");
                let _ = window_handle.update(cx, |_, window, cx| {
                    observer.update(cx, |this, cx| on_release(this, observed, window, cx))
                });
            }),
        );
        activate();
        subscription
    }

    /// Register a callback to be invoked when the window is resized.
    pub fn observe_window_bounds(
        &self,
        window: &mut Window,
        mut callback: impl FnMut(&mut T, &mut Window, &mut Context<T>) + 'static,
    ) -> Subscription {
        let view = self.weak_model();
        let (subscription, activate) = window.bounds_observers.insert(
            (),
            Box::new(move |window, cx| {
                view.update(cx, |view, cx| callback(view, window, cx))
                    .is_ok()
            }),
        );
        activate();
        subscription
    }

    /// Register a callback to be invoked when the window is activated or deactivated.
    pub fn observe_window_activation(
        &self,
        window: &mut Window,
        mut callback: impl FnMut(&mut T, &mut Window, &mut Context<T>) + 'static,
    ) -> Subscription {
        let view = self.weak_model();
        let (subscription, activate) = window.activation_observers.insert(
            (),
            Box::new(move |window, cx| {
                view.update(cx, |view, cx| callback(view, window, cx))
                    .is_ok()
            }),
        );
        activate();
        subscription
    }

    /// Registers a callback to be invoked when the window appearance changes.
    pub fn observe_window_appearance(
        &self,
        window: &mut Window,
        mut callback: impl FnMut(&mut T, &mut Window, &mut Context<T>) + 'static,
    ) -> Subscription {
        let view = self.weak_model();
        let (subscription, activate) = window.appearance_observers.insert(
            (),
            Box::new(move |window, cx| {
                view.update(cx, |view, cx| callback(view, window, cx))
                    .is_ok()
            }),
        );
        activate();
        subscription
    }

    /// Register a callback to be invoked when a keystroke is received by the application
    /// in any window. Note that this fires after all other action and event mechanisms have resolved
    /// and that this API will not be invoked if the event's propagation is stopped.
    pub fn observe_keystrokes(
        &mut self,
        mut f: impl FnMut(&mut T, &KeystrokeEvent, &mut Window, &mut Context<T>) + 'static,
    ) -> Subscription {
        fn inner(
            keystroke_observers: &SubscriberSet<(), KeystrokeObserver>,
            handler: KeystrokeObserver,
        ) -> Subscription {
            let (subscription, activate) = keystroke_observers.insert((), handler);
            activate();
            subscription
        }

        let view = self.weak_model();
        inner(
            &mut self.keystroke_observers,
            Box::new(move |event, window, cx| {
                if let Some(view) = view.upgrade() {
                    view.update(cx, |view, cx| f(view, event, window, cx));
                    true
                } else {
                    false
                }
            }),
        )
    }

    /// Register a callback to be invoked when the window's pending input changes.
    pub fn observe_pending_input(
        &self,
        window: &mut Window,
        mut callback: impl FnMut(&mut T, &mut Window, &mut Context<T>) + 'static,
    ) -> Subscription {
        let view = self.weak_model();
        let (subscription, activate) = window.pending_input_observers.insert(
            (),
            Box::new(move |window, cx| {
                view.update(cx, |view, cx| callback(view, window, cx))
                    .is_ok()
            }),
        );
        activate();
        subscription
    }

    /// Register a listener to be called when the given focus handle receives focus.
    /// Returns a subscription and persists until the subscription is dropped.
    pub fn on_focus(
        &mut self,
        handle: &FocusHandle,
        window: &mut Window,
        mut listener: impl FnMut(&mut T, &mut Window, &mut Context<T>) + 'static,
    ) -> Subscription {
        let view = self.weak_model();
        let focus_id = handle.id;
        let (subscription, activate) =
            window.new_focus_listener(Box::new(move |event, window, cx| {
                view.update(cx, |view, cx| {
                    if event.previous_focus_path.last() != Some(&focus_id)
                        && event.current_focus_path.last() == Some(&focus_id)
                    {
                        listener(view, window, cx)
                    }
                })
                .is_ok()
            }));
        self.defer(|_| activate());
        subscription
    }

    /// Register a listener to be called when the given focus handle or one of its descendants receives focus.
    /// This does not fire if the given focus handle - or one of its descendants - was previously focused.
    /// Returns a subscription and persists until the subscription is dropped.
    pub fn on_focus_in(
        &mut self,
        handle: &FocusHandle,
        window: &mut Window,
        mut listener: impl FnMut(&mut T, &mut Window, &mut Context<T>) + 'static,
    ) -> Subscription {
        let view = self.weak_model();
        let focus_id = handle.id;
        let (subscription, activate) =
            window.new_focus_listener(Box::new(move |event, window, cx| {
                view.update(cx, |view, cx| {
                    if event.is_focus_in(focus_id) {
                        listener(view, window, cx)
                    }
                })
                .is_ok()
            }));
        self.defer(|_| activate());
        subscription
    }

    /// Register a listener to be called when the given focus handle loses focus.
    /// Returns a subscription and persists until the subscription is dropped.
    pub fn on_blur(
        &mut self,
        handle: &FocusHandle,
        window: &mut Window,
        mut listener: impl FnMut(&mut T, &mut Window, &mut Context<T>) + 'static,
    ) -> Subscription {
        let view = self.weak_model();
        let focus_id = handle.id;
        let (subscription, activate) =
            window.new_focus_listener(Box::new(move |event, window, cx| {
                view.update(cx, |view, cx| {
                    if event.previous_focus_path.last() == Some(&focus_id)
                        && event.current_focus_path.last() != Some(&focus_id)
                    {
                        listener(view, window, cx)
                    }
                })
                .is_ok()
            }));
        self.defer(|_| activate());
        subscription
    }

    /// Register a listener to be called when nothing in the window has focus.
    /// This typically happens when the node that was focused is removed from the tree,
    /// and this callback lets you chose a default place to restore the users focus.
    /// Returns a subscription and persists until the subscription is dropped.
    pub fn on_focus_lost(
        &mut self,
        window: &mut Window,
        mut listener: impl FnMut(&mut T, &mut Window, &mut Context<T>) + 'static,
    ) -> Subscription {
        let view = self.weak_model();
        let (subscription, activate) = window.focus_lost_listeners.insert(
            (),
            Box::new(move |window, cx| {
                view.update(cx, |view, cx| listener(view, window, cx))
                    .is_ok()
            }),
        );
        self.defer(|_| activate());
        subscription
    }

    /// Register a listener to be called when the given focus handle or one of its descendants loses focus.
    /// Returns a subscription and persists until the subscription is dropped.
    pub fn on_focus_out(
        &mut self,
        handle: &FocusHandle,
        window: &mut Window,
        mut listener: impl FnMut(&mut T, FocusOutEvent, &mut Window, &mut Context<T>) + 'static,
    ) -> Subscription {
        let view = self.weak_model();
        let focus_id = handle.id;
        let (subscription, activate) =
            window.new_focus_listener(Box::new(move |event, window, cx| {
                view.update(cx, |view, cx| {
                    if let Some(blurred_id) = event.previous_focus_path.last().copied() {
                        if event.is_focus_out(focus_id) {
                            let event = FocusOutEvent {
                                blurred: WeakFocusHandle {
                                    id: blurred_id,
                                    handles: Arc::downgrade(&cx.focus_handles),
                                },
                            };
                            listener(view, event, window, cx)
                        }
                    }
                })
                .is_ok()
            }));
        self.defer(|_| activate());
        subscription
    }

    /// Schedule a future to be run asynchronously.
    /// The given callback is invoked with a [`WeakModel<V>`] to avoid leaking the view for a long-running process.
    /// It's also given an [`AsyncWindowContext`], which can be used to access the state of the view across await points.
    /// The returned future will be polled on the main thread.
    pub fn spawn_in<Fut, R>(
        &self,
        window: &Window,
        f: impl FnOnce(WeakEntity<T>, AsyncWindowContext) -> Fut,
    ) -> Task<R>
    where
        R: 'static,
        Fut: Future<Output = R> + 'static,
    {
        let view = self.weak_model();
        window.spawn(self, |mut cx| f(view, cx))
    }

    /// Register a callback to be invoked when the given global state changes.
    pub fn observe_global_in<G: Global>(
        &mut self,
        window: &Window,
        mut f: impl FnMut(&mut T, &mut Window, &mut Context<'_, T>) + 'static,
    ) -> Subscription {
        let window_handle = window.handle;
        let view = self.weak_model();
        let (subscription, activate) = self.global_observers.insert(
            TypeId::of::<G>(),
            Box::new(move |cx| {
                window_handle
                    .update(cx, |_, window, cx| {
                        view.update(cx, |view, cx| f(view, window, cx)).is_ok()
                    })
                    .unwrap_or(false)
            }),
        );
        self.defer(move |_| activate());
        subscription
    }

    /// Register a callback to be invoked when the given Action type is dispatched to the window.
    pub fn on_action(
        &mut self,
        action_type: TypeId,
        window: &mut Window,
        listener: impl Fn(&mut T, &dyn Any, DispatchPhase, &mut Window, &mut Context<T>) + 'static,
    ) {
        let handle = self.weak_model();
        window.on_action(action_type, move |action, phase, window, cx| {
            handle
                .update(cx, |view, cx| {
                    listener(view, action, phase, window, cx);
                })
                .ok();
        });
    }

    /// Move focus to the current view, assuming it implements [`Focusable`].
    pub fn focus_self(&mut self, window: &mut Window)
    where
        T: Focusable,
    {
        let view = self.model();
        window.defer(self, move |window, cx| {
            view.read(cx).focus_handle(cx).focus(window)
        })
    }
}

impl<'a, T> Context<'a, T> {
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

impl<'a, T> AppContext for Context<'a, T> {
    type Result<U> = U;

    fn new<U: 'static>(&mut self, build_model: impl FnOnce(&mut Context<'_, U>) -> U) -> Entity<U> {
        self.app.new(build_model)
    }

    fn reserve_model<U: 'static>(&mut self) -> Reservation<U> {
        self.app.reserve_model()
    }

    fn insert_model<U: 'static>(
        &mut self,
        reservation: Reservation<U>,
        build_model: impl FnOnce(&mut Context<'_, U>) -> U,
    ) -> Self::Result<Entity<U>> {
        self.app.insert_model(reservation, build_model)
    }

    fn update_model<U: 'static, R>(
        &mut self,
        handle: &Entity<U>,
        update: impl FnOnce(&mut U, &mut Context<'_, U>) -> R,
    ) -> R {
        self.app.update_model(handle, update)
    }

    fn read_model<U, R>(
        &self,
        handle: &Entity<U>,
        read: impl FnOnce(&U, &App) -> R,
    ) -> Self::Result<R>
    where
        U: 'static,
    {
        self.app.read_model(handle, read)
    }

    fn update_window<R, F>(&mut self, window: AnyWindowHandle, update: F) -> Result<R>
    where
        F: FnOnce(AnyView, &mut Window, &mut App) -> R,
    {
        self.app.update_window(window, update)
    }

    fn read_window<U, R>(
        &self,
        window: &WindowHandle<U>,
        read: impl FnOnce(Entity<U>, &App) -> R,
    ) -> Result<R>
    where
        U: 'static,
    {
        self.app.read_window(window, read)
    }
}

impl<T> Borrow<App> for Context<'_, T> {
    fn borrow(&self) -> &App {
        self.app
    }
}

impl<T> BorrowMut<App> for Context<'_, T> {
    fn borrow_mut(&mut self) -> &mut App {
        self.app
    }
}
