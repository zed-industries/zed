use crate::Context;
use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut};
use parking_lot::{Mutex, RwLock};
use slotmap::{SecondaryMap, SlotMap};
use std::{
    any::Any,
    marker::PhantomData,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc, Weak,
    },
};

slotmap::new_key_type! { pub struct EntityId; }

#[derive(Deref, DerefMut)]
pub struct Lease<T> {
    #[deref]
    #[deref_mut]
    entity: Box<T>,
    pub id: EntityId,
}

pub(crate) struct EntityMap {
    ref_counts: Arc<RwLock<RefCounts>>,
    entities: Arc<Mutex<SecondaryMap<EntityId, Box<dyn Any + Send + Sync>>>>,
}

impl EntityMap {
    pub fn new() -> Self {
        Self {
            ref_counts: Arc::new(RwLock::new(SlotMap::with_key())),
            entities: Arc::new(Mutex::new(SecondaryMap::new())),
        }
    }

    pub fn reserve<T: 'static + Send + Sync>(&self) -> Slot<T> {
        let id = self.ref_counts.write().insert(1.into());
        Slot(Handle::new(id, Arc::downgrade(&self.ref_counts)))
    }

    pub fn redeem<T: 'static + Any + Send + Sync>(&self, slot: Slot<T>, entity: T) -> Handle<T> {
        let handle = slot.0;
        self.entities.lock().insert(handle.id, Box::new(entity));
        handle
    }

    pub fn lease<T: 'static + Send + Sync>(&self, handle: &Handle<T>) -> Lease<T> {
        let id = handle.id;
        let entity = self
            .entities
            .lock()
            .remove(id)
            .expect("Circular entity lease. Is the entity already being updated?")
            .downcast::<T>()
            .unwrap();
        Lease { id, entity }
    }

    pub fn end_lease<T: 'static + Send + Sync>(&mut self, lease: Lease<T>) {
        self.entities.lock().insert(lease.id, lease.entity);
    }

    pub fn weak_handle<T: 'static + Send + Sync>(&self, id: EntityId) -> WeakHandle<T> {
        WeakHandle {
            id,
            entity_type: PhantomData,
            ref_counts: Arc::downgrade(&self.ref_counts),
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct Slot<T: Send + Sync + 'static>(Handle<T>);

pub struct Handle<T: Send + Sync> {
    pub(crate) id: EntityId,
    entity_type: PhantomData<T>,
    ref_counts: Weak<RwLock<RefCounts>>,
}

type RefCounts = SlotMap<EntityId, AtomicUsize>;

impl<T: 'static + Send + Sync> Handle<T> {
    pub fn new(id: EntityId, ref_counts: Weak<RwLock<RefCounts>>) -> Self {
        Self {
            id,
            entity_type: PhantomData,
            ref_counts,
        }
    }

    pub fn downgrade(&self) -> WeakHandle<T> {
        WeakHandle {
            id: self.id,
            entity_type: self.entity_type,
            ref_counts: self.ref_counts.clone(),
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
    ) -> C::Result<R> {
        cx.update_entity(self, update)
    }
}

impl<T: Send + Sync> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            entity_type: PhantomData,
            ref_counts: self.ref_counts.clone(),
        }
    }
}

impl<T: Send + Sync> Drop for Handle<T> {
    fn drop(&mut self) {
        if let Some(_ref_counts) = self.ref_counts.upgrade() {
            // todo!()
            // if let Some(count) = ref_counts.read().get(self.id) {
            //     let prev_count = count.fetch_sub(1, SeqCst);
            //     assert_ne!(prev_count, 0, "Detected over-release of a handle.");
            // }
        }
    }
}

pub struct WeakHandle<T> {
    pub(crate) id: EntityId,
    pub(crate) entity_type: PhantomData<T>,
    pub(crate) ref_counts: Weak<RwLock<RefCounts>>,
}

impl<T: Send + Sync + 'static> WeakHandle<T> {
    pub fn upgrade(&self, _: &impl Context) -> Option<Handle<T>> {
        let ref_counts = self.ref_counts.upgrade()?;
        ref_counts.read().get(self.id).unwrap().fetch_add(1, SeqCst);
        Some(Handle {
            id: self.id,
            entity_type: self.entity_type,
            ref_counts: self.ref_counts.clone(),
        })
    }

    /// Update the entity referenced by this handle with the given function if
    /// the referenced entity still exists. Returns an error if the entity has
    /// been released.
    ///
    /// The update function receives a context appropriate for its environment.
    /// When updating in an `AppContext`, it receives a `ModelContext`.
    /// When updating an a `WindowContext`, it receives a `ViewContext`.
    pub fn update<C: Context, R>(
        &self,
        cx: &mut C,
        update: impl FnOnce(&mut T, &mut C::EntityContext<'_, '_, T>) -> R,
    ) -> Result<R>
    where
        Result<C::Result<R>>: crate::Flatten<R>,
    {
        crate::Flatten::flatten(
            self.upgrade(cx)
                .ok_or_else(|| anyhow!("entity release"))
                .map(|this| cx.update_entity(&this, update)),
        )
    }
}
