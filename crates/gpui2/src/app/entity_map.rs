use crate::{AppContext, Context};
use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut};
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use slotmap::{SecondaryMap, SlotMap};
use std::{
    any::{Any, TypeId},
    fmt::{self, Display},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem,
    sync::{
        atomic::{AtomicUsize, Ordering::SeqCst},
        Arc, Weak,
    },
};

slotmap::new_key_type! { pub struct EntityId; }

impl Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub(crate) struct EntityMap {
    entities: SecondaryMap<EntityId, Box<dyn Any + Send + Sync>>,
    ref_counts: Arc<RwLock<EntityRefCounts>>,
}

struct EntityRefCounts {
    counts: SlotMap<EntityId, AtomicUsize>,
    dropped_entity_ids: Vec<EntityId>,
}

impl EntityMap {
    pub fn new() -> Self {
        Self {
            entities: SecondaryMap::new(),
            ref_counts: Arc::new(RwLock::new(EntityRefCounts {
                counts: SlotMap::with_key(),
                dropped_entity_ids: Vec::new(),
            })),
        }
    }

    /// Reserve a slot for an entity, which you can subsequently use with `insert`.
    pub fn reserve<T: 'static + Send + Sync>(&self) -> Slot<T> {
        let id = self.ref_counts.write().counts.insert(1.into());
        Slot(Handle::new(id, Arc::downgrade(&self.ref_counts)))
    }

    /// Insert an entity into a slot obtained by calling `reserve`.
    pub fn insert<T: 'static + Any + Send + Sync>(
        &mut self,
        slot: Slot<T>,
        entity: T,
    ) -> Handle<T> {
        let handle = slot.0;
        self.entities.insert(handle.entity_id, Box::new(entity));
        handle
    }

    /// Move an entity to the stack.
    pub fn lease<'a, T: 'static + Send + Sync>(&mut self, handle: &'a Handle<T>) -> Lease<'a, T> {
        let entity = Some(
            self.entities
                .remove(handle.entity_id)
                .expect("Circular entity lease. Is the entity already being updated?")
                .downcast::<T>()
                .unwrap(),
        );
        Lease { handle, entity }
    }

    /// Return an entity after moving it to the stack.
    pub fn end_lease<T: 'static + Send + Sync>(&mut self, mut lease: Lease<T>) {
        self.entities
            .insert(lease.handle.entity_id, lease.entity.take().unwrap());
    }

    pub fn read<T: 'static + Send + Sync>(&self, handle: &Handle<T>) -> &T {
        self.entities[handle.entity_id].downcast_ref().unwrap()
    }

    pub fn weak_handle<T: 'static + Send + Sync>(&self, id: EntityId) -> WeakHandle<T> {
        WeakHandle {
            any_handle: AnyWeakHandle {
                entity_id: id,
                entity_type: TypeId::of::<T>(),
                entity_ref_counts: Arc::downgrade(&self.ref_counts),
            },
            entity_type: PhantomData,
        }
    }

    pub fn take_dropped(&mut self) -> Vec<(EntityId, Box<dyn Any + Send + Sync>)> {
        let dropped_entity_ids = mem::take(&mut self.ref_counts.write().dropped_entity_ids);
        dropped_entity_ids
            .into_iter()
            .map(|entity_id| (entity_id, self.entities.remove(entity_id).unwrap()))
            .collect()
    }
}

pub struct Lease<'a, T: Send + Sync> {
    entity: Option<Box<T>>,
    pub handle: &'a Handle<T>,
}

impl<'a, T> core::ops::Deref for Lease<'a, T>
where
    T: Send + Sync,
{
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.entity.as_ref().unwrap()
    }
}

impl<'a, T> core::ops::DerefMut for Lease<'a, T>
where
    T: Send + Sync,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.entity.as_mut().unwrap()
    }
}

impl<'a, T> Drop for Lease<'a, T>
where
    T: Send + Sync,
{
    fn drop(&mut self) {
        if self.entity.is_some() {
            // We don't panic here, because other panics can cause us to drop the lease without ending it cleanly.
            log::error!("Leases must be ended with EntityMap::end_lease")
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct Slot<T: Send + Sync + 'static>(Handle<T>);

pub struct AnyHandle {
    pub(crate) entity_id: EntityId,
    entity_type: TypeId,
    entity_map: Weak<RwLock<EntityRefCounts>>,
}

impl AnyHandle {
    fn new(id: EntityId, entity_type: TypeId, entity_map: Weak<RwLock<EntityRefCounts>>) -> Self {
        Self {
            entity_id: id,
            entity_type,
            entity_map,
        }
    }

    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub fn downgrade(&self) -> AnyWeakHandle {
        AnyWeakHandle {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
            entity_ref_counts: self.entity_map.clone(),
        }
    }

    pub fn downcast<T>(&self) -> Option<Handle<T>>
    where
        T: 'static + Send + Sync,
    {
        if TypeId::of::<T>() == self.entity_type {
            Some(Handle {
                any_handle: self.clone(),
                entity_type: PhantomData,
            })
        } else {
            None
        }
    }
}

impl Clone for AnyHandle {
    fn clone(&self) -> Self {
        if let Some(entity_map) = self.entity_map.upgrade() {
            let entity_map = entity_map.read();
            let count = entity_map
                .counts
                .get(self.entity_id)
                .expect("detected over-release of a handle");
            let prev_count = count.fetch_add(1, SeqCst);
            assert_ne!(prev_count, 0, "Detected over-release of a handle.");
        }

        Self {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
            entity_map: self.entity_map.clone(),
        }
    }
}

impl Drop for AnyHandle {
    fn drop(&mut self) {
        if let Some(entity_map) = self.entity_map.upgrade() {
            let entity_map = entity_map.upgradable_read();
            let count = entity_map
                .counts
                .get(self.entity_id)
                .expect("Detected over-release of a handle.");
            let prev_count = count.fetch_sub(1, SeqCst);
            assert_ne!(prev_count, 0, "Detected over-release of a handle.");
            if prev_count == 1 {
                // We were the last reference to this entity, so we can remove it.
                let mut entity_map = RwLockUpgradableReadGuard::upgrade(entity_map);
                entity_map.counts.remove(self.entity_id);
                entity_map.dropped_entity_ids.push(self.entity_id);
            }
        }
    }
}

impl<T> From<Handle<T>> for AnyHandle
where
    T: 'static + Send + Sync,
{
    fn from(handle: Handle<T>) -> Self {
        handle.any_handle
    }
}

#[derive(Deref, DerefMut)]
pub struct Handle<T: Send + Sync> {
    #[deref]
    #[deref_mut]
    any_handle: AnyHandle,
    entity_type: PhantomData<T>,
}

impl<T: 'static + Send + Sync> Handle<T> {
    fn new(id: EntityId, entity_map: Weak<RwLock<EntityRefCounts>>) -> Self {
        Self {
            any_handle: AnyHandle::new(id, TypeId::of::<T>(), entity_map),
            entity_type: PhantomData,
        }
    }

    pub fn downgrade(&self) -> WeakHandle<T> {
        WeakHandle {
            any_handle: self.any_handle.downgrade(),
            entity_type: self.entity_type,
        }
    }

    pub fn read<'a>(&self, cx: &'a AppContext) -> &'a T {
        cx.entities.read(self)
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
            any_handle: self.any_handle.clone(),
            entity_type: self.entity_type,
        }
    }
}

#[derive(Clone)]
pub struct AnyWeakHandle {
    pub(crate) entity_id: EntityId,
    entity_type: TypeId,
    entity_ref_counts: Weak<RwLock<EntityRefCounts>>,
}

impl AnyWeakHandle {
    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    pub fn is_upgradable(&self) -> bool {
        let ref_count = self
            .entity_ref_counts
            .upgrade()
            .and_then(|ref_counts| Some(ref_counts.read().counts.get(self.entity_id)?.load(SeqCst)))
            .unwrap_or(0);
        ref_count > 0
    }

    pub fn upgrade(&self) -> Option<AnyHandle> {
        let entity_map = self.entity_ref_counts.upgrade()?;
        entity_map
            .read()
            .counts
            .get(self.entity_id)?
            .fetch_add(1, SeqCst);
        Some(AnyHandle {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
            entity_map: self.entity_ref_counts.clone(),
        })
    }
}

impl<T> From<WeakHandle<T>> for AnyWeakHandle
where
    T: 'static + Send + Sync,
{
    fn from(handle: WeakHandle<T>) -> Self {
        handle.any_handle
    }
}

impl Hash for AnyWeakHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entity_id.hash(state);
    }
}

impl PartialEq for AnyWeakHandle {
    fn eq(&self, other: &Self) -> bool {
        self.entity_id == other.entity_id
    }
}

impl Eq for AnyWeakHandle {}

#[derive(Deref, DerefMut)]
pub struct WeakHandle<T> {
    #[deref]
    #[deref_mut]
    any_handle: AnyWeakHandle,
    entity_type: PhantomData<T>,
}

impl<T: 'static + Send + Sync> Clone for WeakHandle<T> {
    fn clone(&self) -> Self {
        Self {
            any_handle: self.any_handle.clone(),
            entity_type: self.entity_type,
        }
    }
}

impl<T: Send + Sync + 'static> WeakHandle<T> {
    pub fn upgrade(&self) -> Option<Handle<T>> {
        Some(Handle {
            any_handle: self.any_handle.upgrade()?,
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
            self.upgrade()
                .ok_or_else(|| anyhow!("entity release"))
                .map(|this| cx.update_entity(&this, update)),
        )
    }
}

impl<T: Send + Sync + 'static> Hash for WeakHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.any_handle.hash(state);
    }
}

impl<T: Send + Sync + 'static> PartialEq for WeakHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.any_handle == other.any_handle
    }
}

impl<T: Send + Sync + 'static> Eq for WeakHandle<T> {}
