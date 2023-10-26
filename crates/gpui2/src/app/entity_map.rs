use crate::{AnyBox, AppContext, Context};
use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut};
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use slotmap::{SecondaryMap, SlotMap};
use std::{
    any::{type_name, TypeId},
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

impl EntityId {
    pub fn as_u64(self) -> u64 {
        self.0.as_ffi()
    }
}

impl Display for EntityId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self)
    }
}

pub(crate) struct EntityMap {
    entities: SecondaryMap<EntityId, AnyBox>,
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
    pub fn reserve<T: 'static>(&self) -> Slot<T> {
        let id = self.ref_counts.write().counts.insert(1.into());
        Slot(Handle::new(id, Arc::downgrade(&self.ref_counts)))
    }

    /// Insert an entity into a slot obtained by calling `reserve`.
    pub fn insert<T>(&mut self, slot: Slot<T>, entity: T) -> Handle<T>
    where
        T: 'static + Send,
    {
        let handle = slot.0;
        self.entities.insert(handle.entity_id, Box::new(entity));
        handle
    }

    /// Move an entity to the stack.
    pub fn lease<'a, T>(&mut self, handle: &'a Handle<T>) -> Lease<'a, T> {
        self.assert_valid_context(handle);
        let entity = Some(
            self.entities
                .remove(handle.entity_id)
                .expect("Circular entity lease. Is the entity already being updated?"),
        );
        Lease {
            handle,
            entity,
            entity_type: PhantomData,
        }
    }

    /// Return an entity after moving it to the stack.
    pub fn end_lease<T>(&mut self, mut lease: Lease<T>) {
        self.entities
            .insert(lease.handle.entity_id, lease.entity.take().unwrap());
    }

    pub fn read<T: 'static>(&self, handle: &Handle<T>) -> &T {
        self.assert_valid_context(handle);
        self.entities[handle.entity_id].downcast_ref().unwrap()
    }

    fn assert_valid_context(&self, handle: &AnyHandle) {
        debug_assert!(
            Weak::ptr_eq(&handle.entity_map, &Arc::downgrade(&self.ref_counts)),
            "used a handle with the wrong context"
        );
    }

    pub fn take_dropped(&mut self) -> Vec<(EntityId, AnyBox)> {
        let mut ref_counts = self.ref_counts.write();
        let dropped_entity_ids = mem::take(&mut ref_counts.dropped_entity_ids);

        dropped_entity_ids
            .into_iter()
            .map(|entity_id| {
                ref_counts.counts.remove(entity_id);
                (entity_id, self.entities.remove(entity_id).unwrap())
            })
            .collect()
    }
}

pub struct Lease<'a, T> {
    entity: Option<AnyBox>,
    pub handle: &'a Handle<T>,
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
        if self.entity.is_some() {
            // We don't panic here, because other panics can cause us to drop the lease without ending it cleanly.
            log::error!("Leases must be ended with EntityMap::end_lease")
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct Slot<T>(Handle<T>);

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

    pub fn downcast<T: 'static>(&self) -> Option<Handle<T>> {
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
                entity_map.dropped_entity_ids.push(self.entity_id);
            }
        }
    }
}

impl<T> From<Handle<T>> for AnyHandle {
    fn from(handle: Handle<T>) -> Self {
        handle.any_handle
    }
}

impl Hash for AnyHandle {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entity_id.hash(state);
    }
}

impl PartialEq for AnyHandle {
    fn eq(&self, other: &Self) -> bool {
        self.entity_id == other.entity_id
    }
}

impl Eq for AnyHandle {}

#[derive(Deref, DerefMut)]
pub struct Handle<T> {
    #[deref]
    #[deref_mut]
    any_handle: AnyHandle,
    entity_type: PhantomData<T>,
}

unsafe impl<T> Send for Handle<T> {}
unsafe impl<T> Sync for Handle<T> {}

impl<T: 'static> Handle<T> {
    fn new(id: EntityId, entity_map: Weak<RwLock<EntityRefCounts>>) -> Self
    where
        T: 'static,
    {
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
    pub fn update<C, R>(
        &self,
        cx: &mut C,
        update: impl FnOnce(&mut T, &mut C::EntityContext<'_, '_, T>) -> R,
    ) -> C::Result<R>
    where
        C: Context,
    {
        cx.update_entity(self, update)
    }
}

impl<T> Clone for Handle<T> {
    fn clone(&self) -> Self {
        Self {
            any_handle: self.any_handle.clone(),
            entity_type: self.entity_type,
        }
    }
}

impl<T> std::fmt::Debug for Handle<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "Handle {{ entity_id: {:?}, entity_type: {:?} }}",
            self.any_handle.entity_id,
            type_name::<T>()
        )
    }
}

impl<T> Hash for Handle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.any_handle.hash(state);
    }
}

impl<T> PartialEq for Handle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.any_handle == other.any_handle
    }
}

impl<T> Eq for Handle<T> {}

impl<T> PartialEq<WeakHandle<T>> for Handle<T> {
    fn eq(&self, other: &WeakHandle<T>) -> bool {
        self.entity_id() == other.entity_id()
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

impl<T> From<WeakHandle<T>> for AnyWeakHandle {
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

unsafe impl<T> Send for WeakHandle<T> {}
unsafe impl<T> Sync for WeakHandle<T> {}

impl<T> Clone for WeakHandle<T> {
    fn clone(&self) -> Self {
        Self {
            any_handle: self.any_handle.clone(),
            entity_type: self.entity_type,
        }
    }
}

impl<T: 'static> WeakHandle<T> {
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
    pub fn update<C, R>(
        &self,
        cx: &mut C,
        update: impl FnOnce(&mut T, &mut C::EntityContext<'_, '_, T>) -> R,
    ) -> Result<R>
    where
        C: Context,
        Result<C::Result<R>>: crate::Flatten<R>,
    {
        crate::Flatten::flatten(
            self.upgrade()
                .ok_or_else(|| anyhow!("entity release"))
                .map(|this| cx.update_entity(&this, update)),
        )
    }
}

impl<T> Hash for WeakHandle<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.any_handle.hash(state);
    }
}

impl<T> PartialEq for WeakHandle<T> {
    fn eq(&self, other: &Self) -> bool {
        self.any_handle == other.any_handle
    }
}

impl<T> Eq for WeakHandle<T> {}

impl<T> PartialEq<Handle<T>> for WeakHandle<T> {
    fn eq(&self, other: &Handle<T>) -> bool {
        self.entity_id() == other.entity_id()
    }
}
