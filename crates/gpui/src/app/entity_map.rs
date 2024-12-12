use crate::{
    seal::Sealed, Action, AnyWindowHandle, AppContext, AsyncAppContext, Context, Effect, Entity,
    EventEmitter, Subscription, Task, Window,
};
use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut};
use futures::Future;
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use slotmap::{KeyData, SecondaryMap, SlotMap};
use smol::future::FutureExt;
use std::{
    any::{type_name, Any, TypeId},
    fmt::{self, Display},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem,
    num::NonZeroU64,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc, Weak,
    },
    thread::panicking,
};

#[cfg(any(test, feature = "test-support"))]
use collections::HashMap;

slotmap::new_key_type! {
    /// A unique identifier for a model or view across the application.
    pub struct EntityId;
}

impl From<u64> for EntityId {
    fn from(value: u64) -> Self {
        Self(KeyData::from_ffi(value))
    }
}

impl EntityId {
    /// Converts this entity id to a [NonZeroU64]
    pub fn as_non_zero_u64(self) -> NonZeroU64 {
        NonZeroU64::new(self.0.as_ffi()).unwrap()
    }

    /// Converts this entity id to a [u64]
    pub fn as_u64(self) -> u64 {
        self.0.as_ffi()
    }
}

impl Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_u64())
    }
}

pub(crate) struct EntityMap {
    entities: SecondaryMap<EntityId, Box<dyn Any>>,
    ref_counts: Arc<RwLock<EntityRefCounts>>,
}

struct EntityRefCounts {
    counts: SlotMap<EntityId, AtomicUsize>,
    dropped_entity_ids: Vec<EntityId>,
    #[cfg(any(test, feature = "test-support"))]
    leak_detector: LeakDetector,
}

impl EntityMap {
    pub fn new() -> Self {
        Self {
            entities: SecondaryMap::new(),
            ref_counts: Arc::new(RwLock::new(EntityRefCounts {
                counts: SlotMap::with_key(),
                dropped_entity_ids: Vec::new(),
                #[cfg(any(test, feature = "test-support"))]
                leak_detector: LeakDetector {
                    next_handle_id: 0,
                    entity_handles: HashMap::default(),
                },
            })),
        }
    }

    /// Reserve a slot for an entity, which you can subsequently use with `insert`.
    pub fn reserve<T: 'static>(&self) -> Slot<T> {
        let id = self.ref_counts.write().counts.insert(1.into());
        Slot(Model::new(id, Arc::downgrade(&self.ref_counts)))
    }

    /// Insert an entity into a slot obtained by calling `reserve`.
    pub fn insert<T>(&mut self, slot: Slot<T>, entity: T) -> Model<T>
    where
        T: 'static,
    {
        let model = slot.0;
        self.entities.insert(model.entity_id, Box::new(entity));
        model
    }

    /// Move an entity to the stack.
    #[track_caller]
    pub fn lease<'a, T>(&mut self, model: &'a Model<T>) -> Lease<'a, T> {
        self.assert_valid_context(model);
        let entity = Some(
            self.entities
                .remove(model.entity_id)
                .unwrap_or_else(|| double_lease_panic::<T>("update")),
        );
        Lease {
            model,
            entity,
            entity_type: PhantomData,
        }
    }

    /// Returns an entity after moving it to the stack.
    pub fn end_lease<T>(&mut self, mut lease: Lease<T>) {
        self.entities
            .insert(lease.model.entity_id, lease.entity.take().unwrap());
    }

    pub fn read<T: 'static>(&self, model: &Model<T>) -> &T {
        self.assert_valid_context(model);
        self.entities[model.entity_id]
            .downcast_ref()
            .unwrap_or_else(|| double_lease_panic::<T>("read"))
    }

    fn assert_valid_context(&self, model: &AnyModel) {
        debug_assert!(
            Weak::ptr_eq(&model.entity_map, &Arc::downgrade(&self.ref_counts)),
            "used a model with the wrong context"
        );
    }

    pub fn take_dropped(&mut self) -> Vec<(EntityId, Box<dyn Any>)> {
        let mut ref_counts = self.ref_counts.write();
        let dropped_entity_ids = mem::take(&mut ref_counts.dropped_entity_ids);

        dropped_entity_ids
            .into_iter()
            .filter_map(|entity_id| {
                let count = ref_counts.counts.remove(entity_id).unwrap();
                debug_assert_eq!(
                    count.load(SeqCst),
                    0,
                    "dropped an entity that was referenced"
                );
                // If the EntityId was allocated with `Context::reserve`,
                // the entity may not have been inserted.
                Some((entity_id, self.entities.remove(entity_id)?))
            })
            .collect()
    }
}

fn double_lease_panic<T>(operation: &str) -> ! {
    panic!(
        "cannot {operation} {} while it is already being updated",
        std::any::type_name::<T>()
    )
}

pub(crate) struct Lease<'a, T> {
    entity: Option<Box<dyn Any>>,
    pub model: &'a Model<T>,
    entity_type: PhantomData<T>,
}

impl<'a, T: 'static> core::ops::Deref for Lease<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.entity.as_ref().unwrap().downcast_ref().unwrap()
    }
}

impl<'a, T: 'static> core::ops::DerefMut for Lease<'a, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.entity.as_mut().unwrap().downcast_mut().unwrap()
    }
}

impl<'a, T> Drop for Lease<'a, T> {
    fn drop(&mut self) {
        if self.entity.is_some() && !panicking() {
            panic!("Leases must be ended with EntityMap::end_lease")
        }
    }
}

#[derive(Deref, DerefMut)]
pub(crate) struct Slot<T>(Model<T>);

/// A dynamically typed reference to a model, which can be downcast into a `Model<T>`.
pub struct AnyModel {
    pub(crate) entity_id: EntityId,
    pub(crate) entity_type: TypeId,
    entity_map: Weak<RwLock<EntityRefCounts>>,
    #[cfg(any(test, feature = "test-support"))]
    handle_id: HandleId,
}

impl AnyModel {
    fn new(id: EntityId, entity_type: TypeId, entity_map: Weak<RwLock<EntityRefCounts>>) -> Self {
        Self {
            entity_id: id,
            entity_type,
            entity_map: entity_map.clone(),
            #[cfg(any(test, feature = "test-support"))]
            handle_id: entity_map
                .upgrade()
                .unwrap()
                .write()
                .leak_detector
                .handle_created(id),
        }
    }

    /// Returns the id associated with this model.
    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    /// Returns the [TypeId] associated with this model.
    pub fn entity_type(&self) -> TypeId {
        self.entity_type
    }

    /// Converts this model handle into a weak variant, which does not prevent it from being released.
    pub fn downgrade(&self) -> AnyWeakModel {
        AnyWeakModel {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
            entity_ref_counts: self.entity_map.clone(),
        }
    }

    /// Converts this model handle into a strongly-typed model handle of the given type.
    /// If this model handle is not of the specified type, returns itself as an error variant.
    pub fn downcast<T: 'static>(self) -> Result<Model<T>, AnyModel> {
        if TypeId::of::<T>() == self.entity_type {
            Ok(Model {
                any_model: self,
                entity_type: PhantomData,
            })
        } else {
            Err(self)
        }
    }
}

impl Clone for AnyModel {
    fn clone(&self) -> Self {
        if let Some(entity_map) = self.entity_map.upgrade() {
            let entity_map = entity_map.read();
            let count = entity_map
                .counts
                .get(self.entity_id)
                .expect("detected over-release of a model");
            let prev_count = count.fetch_add(1, SeqCst);
            assert_ne!(prev_count, 0, "Detected over-release of a model.");
        }

        Self {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
            entity_map: self.entity_map.clone(),
            #[cfg(any(test, feature = "test-support"))]
            handle_id: self
                .entity_map
                .upgrade()
                .unwrap()
                .write()
                .leak_detector
                .handle_created(self.entity_id),
        }
    }
}

impl Drop for AnyModel {
    fn drop(&mut self) {
        if let Some(entity_map) = self.entity_map.upgrade() {
            let entity_map = entity_map.upgradable_read();
            let count = entity_map
                .counts
                .get(self.entity_id)
                .expect("detected over-release of a handle.");
            let prev_count = count.fetch_sub(1, SeqCst);
            assert_ne!(prev_count, 0, "Detected over-release of a model.");
            if prev_count == 1 {
                // We were the last reference to this entity, so we can remove it.
                let mut entity_map = RwLockUpgradableReadGuard::upgrade(entity_map);
                entity_map.dropped_entity_ids.push(self.entity_id);
            }
        }

        #[cfg(any(test, feature = "test-support"))]
        if let Some(entity_map) = self.entity_map.upgrade() {
            entity_map
                .write()
                .leak_detector
                .handle_released(self.entity_id, self.handle_id)
        }
    }
}

impl<T> From<Model<T>> for AnyModel {
    fn from(model: Model<T>) -> Self {
        model.any_model
    }
}

impl Hash for AnyModel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entity_id.hash(state);
    }
}

impl PartialEq for AnyModel {
    fn eq(&self, other: &Self) -> bool {
        self.entity_id == other.entity_id
    }
}

impl Eq for AnyModel {}

impl std::fmt::Debug for AnyModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnyModel")
            .field("entity_id", &self.entity_id.as_u64())
            .finish()
    }
}

/// A strong, well typed reference to a struct which is managed
/// by GPUI
#[derive(Deref, DerefMut)]
pub struct Model<T> {
    #[deref]
    #[deref_mut]
    pub(crate) any_model: AnyModel,
    pub(crate) entity_type: PhantomData<T>,
}

unsafe impl<T> Send for Model<T> {}
unsafe impl<T> Sync for Model<T> {}
impl<T> Sealed for Model<T> {}

impl<T: 'static> Entity<T> for Model<T> {
    type Weak = WeakModel<T>;

    fn entity_id(&self) -> EntityId {
        self.any_model.entity_id
    }

    fn downgrade(&self) -> Self::Weak {
        WeakModel {
            any_model: self.any_model.downgrade(),
            entity_type: self.entity_type,
        }
    }

    fn upgrade_from(weak: &Self::Weak) -> Option<Self>
    where
        Self: Sized,
    {
        Some(Model {
            any_model: weak.any_model.upgrade()?,
            entity_type: weak.entity_type,
        })
    }
}

impl<T: 'static> Model<T> {
    fn new(id: EntityId, entity_map: Weak<RwLock<EntityRefCounts>>) -> Self
    where
        T: 'static,
    {
        Self {
            any_model: AnyModel::new(id, TypeId::of::<T>(), entity_map),
            entity_type: PhantomData,
        }
    }

    /// Downgrade the this to a weak model reference
    pub fn downgrade(&self) -> WeakModel<T> {
        // Delegate to the trait implementation to keep behavior in one place.
        // This method was included to improve method resolution in the presence of
        // the Model's deref
        Entity::downgrade(self)
    }

    /// Convert this into a dynamically typed model.
    pub fn into_any(self) -> AnyModel {
        self.any_model
    }

    /// Grab a reference to this entity from the context.
    pub fn read<'a>(&self, cx: &'a AppContext) -> &'a T {
        cx.entities.read(self)
    }

    /// Read the entity referenced by this model with the given function.
    pub fn read_with<R, C: Context>(
        &self,
        cx: &C,
        f: impl FnOnce(&T, &Model<T>, &AppContext) -> R,
    ) -> C::Result<R> {
        cx.read_model(self, f)
    }

    /// Updates the entity referenced by this model with the given function.
    pub fn update<C, R>(
        &self,
        cx: &mut C,
        update: impl FnOnce(&mut T, &Model<T>, &mut AppContext) -> R,
    ) -> C::Result<R>
    where
        C: Context,
    {
        cx.update_model(self, update)
    }

    /// Updates the entity referenced by this model with the given function in the specified window.
    pub fn update_in_window<C: Context, R>(
        &self,
        window: impl Into<AnyWindowHandle>,
        cx: &mut C,
        update: impl FnOnce(&mut T, &Model<T>, &mut Window, &mut AppContext) -> R,
    ) -> Result<R> {
        window.into().update(cx, |window, cx: &mut AppContext| {
            self.update(cx, |this, model, cx| update(this, model, window, cx))
        })
    }

    /// Schedules an update to be performed on this model at the end of the current effect cycle.
    pub fn defer<R>(
        &self,
        cx: &mut AppContext,
        f: impl FnOnce(&mut T, &Model<T>, &mut AppContext) -> R + 'static,
    ) {
        let model = self.clone();
        cx.defer(move |cx| {
            model.update(cx, |this, model, cx| f(this, model, cx));
        });
    }

    /// Creates a listener function that updates this model when an event is received.
    ///
    /// This method takes a callback function that will be called with the model, the event, and a mutable
    /// reference to the `AppContext`. It returns a new function that can be used as an event listener.
    ///
    /// # Arguments
    ///
    /// * `callback` - A closure that takes a mutable reference to the model (`&mut T`), a reference to the event (`&E`),
    ///   and a mutable reference to the `AppContext`.
    ///
    /// # Returns
    ///
    /// A new function that can be used as an event listener. This function takes a reference to the event (`&E`)
    /// and a mutable reference to the `AppContext`.
    pub fn listener<E: ?Sized>(
        &self,
        callback: impl Fn(&mut T, &E, &Model<T>, &mut Window, &mut AppContext) + 'static,
    ) -> impl Fn(&E, &mut Window, &mut AppContext) {
        let model = self.clone();
        move |event: &E, window: &mut Window, cx: &mut AppContext| {
            model.update(cx, |this, model, cx| {
                callback(this, event, model, window, cx);
            });
        }
    }

    /// Notify observers of this model that it may have changed.
    pub fn notify(&self, app: &mut AppContext) {
        app.notify(Some(self.entity_id))
    }

    /// Creates a closure that binds to this model and executes the given function.
    ///
    /// This method takes a function that operates on the model and returns a new function
    /// that can be called with just an `AppContext`. This is useful for creating callbacks
    /// that need to interact with the model but don't have direct access to it.
    ///
    /// # Arguments
    ///
    /// * `f` - A closure that takes a mutable reference to the model (`&mut T`), a reference to the model handle (`&Model<T>`),
    ///   and a reference to the `AppContext`.
    ///
    /// # Returns
    ///
    /// A new closure that takes a mutable reference to the `AppContext` and returns the result of `f`.
    pub fn bind<R>(
        &self,
        f: impl Fn(&mut T, &Model<T>, &AppContext) -> R + 'static,
    ) -> impl Fn(&mut AppContext) -> R {
        let model = self.clone();
        move |cx: &mut AppContext| model.update(cx, |this, model, cx| f(this, model, cx))
    }

    /// Creates a closure that binds to this model and executes the given function within a specific window.
    ///
    /// This method takes a reference to a Window and a function that operates on the model, the Window, and the AppContext.
    /// It returns a new function that can be called with just a Window and an AppContext. This is useful for creating
    /// callbacks that need to interact with both the model and a specific window.
    ///
    /// # Arguments
    ///
    /// * `window` - A reference to the Window in which the function will be executed.
    /// * `f` - A closure that takes a mutable reference to the model (`&mut T`), a reference to the model handle (`&Model<T>`),
    ///   a mutable reference to the Window, and a mutable reference to the AppContext.
    ///
    /// # Returns
    ///
    /// A new closure that takes a mutable reference to the Window and a mutable reference to the AppContext,
    /// and returns the result of `f`.
    pub fn bind_in_window<R>(
        &self,
        window: &Window,
        f: impl Fn(&mut T, &Model<T>, &mut Window, &mut AppContext) -> R + 'static,
    ) -> impl Fn(&mut Window, &mut AppContext) -> R {
        let model = self.clone();
        let window_handle = window.handle();
        move |window: &mut Window, cx: &mut AppContext| {
            if window.handle() == window_handle {
                model.update(cx, |this, model, cx| f(this, model, window, cx))
            } else {
                panic!("Window mismatch in bind_in_window")
            }
        }
    }

    /// Spawn the future returned by the given function.
    /// The function is provided a weak handle to this model and a context that can be held across await points.
    /// The returned task must be held or detached.
    pub fn spawn<Fut, R>(
        &self,
        cx: &AppContext,
        f: impl FnOnce(WeakModel<T>, AsyncAppContext) -> Fut,
    ) -> Task<R>
    where
        T: 'static,
        Fut: Future<Output = R> + 'static,
        R: 'static,
    {
        let model = self.downgrade();
        cx.spawn(|cx| f(model, cx))
    }

    /// Arranges for the given function to be called whenever [`Model::notify`] is called with the given model or view.
    pub fn observe<U>(
        &self,
        entity: &Model<U>,
        cx: &mut AppContext,
        mut on_notify: impl FnMut(&mut T, Model<U>, &Model<T>, &mut AppContext) + 'static,
    ) -> Subscription
    where
        T: 'static,
        U: 'static,
    {
        let this = self.downgrade();
        cx.observe_internal(entity, move |e, cx| {
            if let Some(this) = this.upgrade() {
                this.update(cx, |this, model, cx| on_notify(this, e, model, cx));
                true
            } else {
                false
            }
        })
    }

    /// Subscribe to an event type from another model or view
    pub fn subscribe<U, E>(
        &self,
        entity: &Model<U>,
        cx: &mut AppContext,
        mut on_event: impl FnMut(&mut T, Model<U>, &E, &Model<T>, &mut AppContext) + 'static,
    ) -> Subscription
    where
        T: 'static,
        U: 'static + EventEmitter<E>,
        E: 'static,
    {
        let this = self.downgrade();
        cx.subscribe_internal(entity, move |emitter, event, cx| {
            if let Some(this) = this.upgrade() {
                this.update(cx, |this, model, cx| {
                    on_event(this, emitter, event, model, cx)
                });
                true
            } else {
                false
            }
        })
    }

    /// Emit an event of the specified type, which can be handled by other entities that have subscribed via `subscribe` methods on their respective contexts.
    pub fn emit<E>(&self, event: E, cx: &mut AppContext)
    where
        T: EventEmitter<E>,
        E: 'static,
    {
        cx.pending_effects.push_back(Effect::Emit {
            emitter: self.entity_id(),
            event_type: TypeId::of::<E>(),
            event: Box::new(event),
        });
    }

    /// Register a callback to be invoked when GPUI releases this model.
    pub fn on_release(
        &self,
        cx: &mut AppContext,
        on_release: impl FnOnce(&mut T, &mut AppContext) + 'static,
    ) -> Subscription
    where
        T: 'static,
    {
        let (subscription, activate) = cx.release_listeners.insert(
            self.entity_id,
            Box::new(move |this, cx| {
                let this = this.downcast_mut().expect("invalid entity type");
                on_release(this, cx);
            }),
        );
        activate();
        subscription
    }

    /// Register a callback to be run on the release of another model.
    pub fn observe_release<T2, E>(
        &self,
        observed: &E,
        cx: &mut AppContext,
        on_release: impl FnOnce(&mut T, &mut T2, &Model<T>, &mut AppContext) + 'static,
    ) -> Subscription
    where
        T: Any,
        T2: 'static,
        E: Entity<T2>,
    {
        let entity_id = observed.entity_id();
        let this = self.downgrade();
        let (subscription, activate) = cx.release_listeners.insert(
            entity_id,
            Box::new(move |released, cx| {
                let released = released.downcast_mut().expect("invalid entity type");
                if let Some(this) = this.upgrade() {
                    this.update(cx, |this, model, cx| on_release(this, released, model, cx));
                }
            }),
        );
        activate();
        subscription
    }

    /// Register a callback to for updates to the given global
    pub fn observe_global<G: 'static>(
        &self,
        cx: &mut AppContext,
        mut f: impl FnMut(&mut T, &Model<T>, &mut AppContext) + 'static,
    ) -> Subscription
    where
        T: 'static,
    {
        let model = self.downgrade();
        let (subscription, activate) = cx.global_observers.insert(
            TypeId::of::<G>(),
            Box::new(move |cx| {
                model
                    .update(cx, |this, model, cx| f(this, model, cx))
                    .is_ok()
            }),
        );
        cx.defer(move |_| activate());
        subscription
    }

    /// Arrange for the given function to be invoked whenever the application is quit.
    /// The future returned from this callback will be polled for up to [crate::SHUTDOWN_TIMEOUT] until the app fully quits.
    pub fn on_app_quit<Fut>(
        &self,
        cx: &mut AppContext,
        mut on_quit: impl FnMut(&mut T, &Model<T>, &mut AppContext) -> Fut + 'static,
    ) -> Subscription
    where
        Fut: 'static + Future<Output = ()>,
        T: 'static,
    {
        let handle = self.downgrade();
        let (subscription, activate) = cx.quit_observers.insert(
            (),
            Box::new(move |cx| {
                let future = handle
                    .update(cx, |this, model, cx| on_quit(this, model, cx))
                    .ok();
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

    /// Register a global listener for actions invoked via the keyboard.
    pub fn on_action<A: Action>(
        &mut self,
        cx: &mut AppContext,
        listener: impl Fn(&mut T, &A, &Self, &mut AppContext) + 'static,
    ) {
        let model = self.downgrade();
        cx.on_action(move |action: &A, cx| {
            model
                .update(cx, |this, model, cx| listener(this, action, model, cx))
                .ok();
        });
    }
}

impl<T> Clone for Model<T> {
    fn clone(&self) -> Self {
        Self {
            any_model: self.any_model.clone(),
            entity_type: self.entity_type,
        }
    }
}

impl<T> std::fmt::Debug for Model<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Model")
            .field("entity_id", &self.any_model.entity_id)
            .field("entity_type", &type_name::<T>())
            .finish()
    }
}

impl<T> Hash for Model<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.any_model.hash(state);
    }
}

impl<T> PartialEq for Model<T> {
    fn eq(&self, other: &Self) -> bool {
        self.any_model == other.any_model
    }
}

impl<T> Eq for Model<T> {}

impl<T> PartialEq<WeakModel<T>> for Model<T> {
    fn eq(&self, other: &WeakModel<T>) -> bool {
        self.any_model.entity_id() == other.entity_id()
    }
}

/// A type erased, weak reference to a model.
#[derive(Clone)]
pub struct AnyWeakModel {
    pub(crate) entity_id: EntityId,
    entity_type: TypeId,
    entity_ref_counts: Weak<RwLock<EntityRefCounts>>,
}

impl AnyWeakModel {
    /// Get the entity ID associated with this weak reference.
    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    /// Check if this weak handle can be upgraded, or if the model has already been dropped
    pub fn is_upgradable(&self) -> bool {
        let ref_count = self
            .entity_ref_counts
            .upgrade()
            .and_then(|ref_counts| Some(ref_counts.read().counts.get(self.entity_id)?.load(SeqCst)))
            .unwrap_or(0);
        ref_count > 0
    }

    /// Upgrade this weak model reference to a strong reference.
    pub fn upgrade(&self) -> Option<AnyModel> {
        let ref_counts = &self.entity_ref_counts.upgrade()?;
        let ref_counts = ref_counts.read();
        let ref_count = ref_counts.counts.get(self.entity_id)?;

        // entity_id is in dropped_entity_ids
        if ref_count.load(SeqCst) == 0 {
            return None;
        }
        ref_count.fetch_add(1, SeqCst);
        drop(ref_counts);

        Some(AnyModel {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
            entity_map: self.entity_ref_counts.clone(),
            #[cfg(any(test, feature = "test-support"))]
            handle_id: self
                .entity_ref_counts
                .upgrade()
                .unwrap()
                .write()
                .leak_detector
                .handle_created(self.entity_id),
        })
    }

    /// Assert that model referenced by this weak handle has been released.
    #[cfg(any(test, feature = "test-support"))]
    pub fn assert_released(&self) {
        self.entity_ref_counts
            .upgrade()
            .unwrap()
            .write()
            .leak_detector
            .assert_released(self.entity_id);

        if self
            .entity_ref_counts
            .upgrade()
            .and_then(|ref_counts| Some(ref_counts.read().counts.get(self.entity_id)?.load(SeqCst)))
            .is_some()
        {
            panic!(
                "entity was recently dropped but resources are retained until the end of the effect cycle."
            )
        }
    }
}

impl std::fmt::Debug for AnyWeakModel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("entity_id", &self.entity_id)
            .field("entity_type", &self.entity_type)
            .finish()
    }
}

impl<T> From<WeakModel<T>> for AnyWeakModel {
    fn from(model: WeakModel<T>) -> Self {
        model.any_model
    }
}

impl Hash for AnyWeakModel {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entity_id.hash(state);
    }
}

impl PartialEq for AnyWeakModel {
    fn eq(&self, other: &Self) -> bool {
        self.entity_id == other.entity_id
    }
}

impl Eq for AnyWeakModel {}

/// A weak reference to a model of the given type.
#[derive(Deref, DerefMut)]
pub struct WeakModel<T> {
    #[deref]
    #[deref_mut]
    any_model: AnyWeakModel,
    entity_type: PhantomData<T>,
}

impl<T> std::fmt::Debug for WeakModel<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&type_name::<Self>())
            .field("entity_id", &self.any_model.entity_id)
            .field("entity_type", &type_name::<T>())
            .finish()
    }
}

unsafe impl<T> Send for WeakModel<T> {}
unsafe impl<T> Sync for WeakModel<T> {}

impl<T> Clone for WeakModel<T> {
    fn clone(&self) -> Self {
        Self {
            any_model: self.any_model.clone(),
            entity_type: self.entity_type,
        }
    }
}

impl<T: 'static> WeakModel<T> {
    /// Upgrade this weak model reference into a strong model reference
    pub fn upgrade(&self) -> Option<Model<T>> {
        // Delegate to the trait implementation to keep behavior in one place.
        Model::upgrade_from(self)
    }

    /// Updates the entity referenced by this model with the given function if
    /// the referenced entity still exists. Returns an error if the entity has
    /// been released.
    pub fn update<C, R>(
        &self,
        cx: &mut C,
        update: impl FnOnce(&mut T, &Model<T>, &mut AppContext) -> R,
    ) -> Result<R>
    where
        C: Context,
        Result<C::Result<R>>: crate::Flatten<R>,
    {
        crate::Flatten::flatten(
            self.upgrade()
                .ok_or_else(|| anyhow!("entity release"))
                .map(|this| cx.update_model(&this, update)),
        )
    }

    /// Schedules an update to be performed on this model at the end of the current effect cycle.
    pub fn defer(
        &self,
        cx: &mut AppContext,
        deferred: impl FnOnce(&mut T, &Model<T>, &mut AppContext) + 'static,
    ) -> Result<()> {
        let model = self.clone();
        cx.defer(move |cx| {
            if let Some(model) = model.upgrade() {
                model.update(cx, deferred);
            }
        });
        Ok(())
    }

    /// Reads the entity referenced by this model with the given function if
    /// the referenced entity still exists. Returns an error if the entity has
    /// been released.
    pub fn read_with<C, R>(
        &self,
        cx: &C,
        read: impl FnOnce(&T, &Model<T>, &AppContext) -> R,
    ) -> Result<R>
    where
        C: Context,
        Result<C::Result<R>>: crate::Flatten<R>,
    {
        crate::Flatten::flatten(
            self.upgrade()
                .ok_or_else(|| anyhow!("entity release"))
                .map(|this| cx.read_model(&this, read)),
        )
    }
}

impl<T> Hash for WeakModel<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.any_model.hash(state);
    }
}

impl<T> PartialEq for WeakModel<T> {
    fn eq(&self, other: &Self) -> bool {
        self.any_model == other.any_model
    }
}

impl<T> Eq for WeakModel<T> {}

impl<T> PartialEq<Model<T>> for WeakModel<T> {
    fn eq(&self, other: &Model<T>) -> bool {
        self.entity_id() == other.any_model.entity_id()
    }
}

#[cfg(any(test, feature = "test-support"))]
static LEAK_BACKTRACE: std::sync::LazyLock<bool> =
    std::sync::LazyLock::new(|| std::env::var("LEAK_BACKTRACE").map_or(false, |b| !b.is_empty()));

#[cfg(any(test, feature = "test-support"))]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub(crate) struct HandleId {
    id: u64, // id of the handle itself, not the pointed at object
}

#[cfg(any(test, feature = "test-support"))]
pub(crate) struct LeakDetector {
    next_handle_id: u64,
    entity_handles: HashMap<EntityId, HashMap<HandleId, Option<backtrace::Backtrace>>>,
}

#[cfg(any(test, feature = "test-support"))]
impl LeakDetector {
    #[track_caller]
    pub fn handle_created(&mut self, entity_id: EntityId) -> HandleId {
        let id = util::post_inc(&mut self.next_handle_id);
        let handle_id = HandleId { id };
        let handles = self.entity_handles.entry(entity_id).or_default();
        handles.insert(
            handle_id,
            LEAK_BACKTRACE.then(backtrace::Backtrace::new_unresolved),
        );
        handle_id
    }

    pub fn handle_released(&mut self, entity_id: EntityId, handle_id: HandleId) {
        let handles = self.entity_handles.entry(entity_id).or_default();
        handles.remove(&handle_id);
    }

    pub fn assert_released(&mut self, entity_id: EntityId) {
        let handles = self.entity_handles.entry(entity_id).or_default();
        if !handles.is_empty() {
            for backtrace in handles.values_mut() {
                if let Some(mut backtrace) = backtrace.take() {
                    backtrace.resolve();
                    eprintln!("Leaked handle: {:#?}", backtrace);
                } else {
                    eprintln!("Leaked handle: export LEAK_BACKTRACE to find allocation site");
                }
            }
            panic!();
        }
    }
}

#[cfg(test)]
mod test {
    use crate::EntityMap;

    struct TestEntity {
        pub i: i32,
    }

    #[test]
    fn test_entity_map_slot_assignment_before_cleanup() {
        // Tests that slots are not re-used before take_dropped.
        let mut entity_map = EntityMap::new();

        let slot = entity_map.reserve::<TestEntity>();
        entity_map.insert(slot, TestEntity { i: 1 });

        let slot = entity_map.reserve::<TestEntity>();
        entity_map.insert(slot, TestEntity { i: 2 });

        let dropped = entity_map.take_dropped();
        assert_eq!(dropped.len(), 2);

        assert_eq!(
            dropped
                .into_iter()
                .map(|(_, entity)| entity.downcast::<TestEntity>().unwrap().i)
                .collect::<Vec<i32>>(),
            vec![1, 2],
        );
    }

    #[test]
    fn test_entity_map_weak_upgrade_before_cleanup() {
        // Tests that weak handles are not upgraded before take_dropped
        let mut entity_map = EntityMap::new();

        let slot = entity_map.reserve::<TestEntity>();
        let handle = entity_map.insert(slot, TestEntity { i: 1 });
        let weak = handle.downgrade();
        drop(handle);

        let strong = weak.upgrade();
        assert_eq!(strong, None);

        let dropped = entity_map.take_dropped();
        assert_eq!(dropped.len(), 1);

        assert_eq!(
            dropped
                .into_iter()
                .map(|(_, entity)| entity.downcast::<TestEntity>().unwrap().i)
                .collect::<Vec<i32>>(),
            vec![1],
        );
    }
}
