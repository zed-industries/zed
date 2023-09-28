use crate::Context;
use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut};
use parking_lot::Mutex;
use slotmap::{SecondaryMap, SlotMap};
use std::{any::Any, marker::PhantomData, sync::Arc};

slotmap::new_key_type! { pub struct EntityId; }

#[derive(Deref, DerefMut)]
pub struct Lease<T> {
    #[deref]
    #[deref_mut]
    entity: Box<T>,
    pub id: EntityId,
}

pub(crate) struct EntityMap {
    ref_counts: Arc<Mutex<SlotMap<EntityId, usize>>>,
    entities: Arc<Mutex<SecondaryMap<EntityId, Box<dyn Any + Send + Sync>>>>,
}

impl EntityMap {
    pub fn new() -> Self {
        Self {
            ref_counts: Arc::new(Mutex::new(SlotMap::with_key())),
            entities: Arc::new(Mutex::new(SecondaryMap::new())),
        }
    }

    pub fn reserve<T>(&self) -> Slot<T> {
        let id = self.ref_counts.lock().insert(1);
        Slot(Handle {
            id,
            entity_type: PhantomData,
        })
    }

    pub fn redeem<T: 'static + Any + Send + Sync>(&self, slot: Slot<T>, entity: T) -> Handle<T> {
        let handle = slot.0;
        self.entities.lock().insert(handle.id, Box::new(entity));
        handle
    }

    pub fn lease<T: 'static>(&self, handle: &Handle<T>) -> Lease<T> {
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
}

#[derive(Deref, DerefMut)]
pub struct Slot<T>(Handle<T>);

pub struct Handle<T> {
    pub(crate) id: EntityId,
    pub(crate) entity_type: PhantomData<T>,
}

impl<T: Send + Sync + 'static> Handle<T> {
    pub fn new(id: EntityId) -> Self {
        Self {
            id,
            entity_type: PhantomData,
        }
    }

    pub fn downgrade(&self) -> WeakHandle<T> {
        WeakHandle {
            id: self.id,
            entity_type: self.entity_type,
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

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            id: self.id,
            entity_type: PhantomData,
        }
    }
}

pub struct WeakHandle<T> {
    pub(crate) id: EntityId,
    pub(crate) entity_type: PhantomData<T>,
}

impl<T: Send + Sync + 'static> WeakHandle<T> {
    pub fn upgrade(&self, _: &impl Context) -> Option<Handle<T>> {
        // todo!("Actually upgrade")
        Some(Handle {
            id: self.id,
            entity_type: self.entity_type,
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
