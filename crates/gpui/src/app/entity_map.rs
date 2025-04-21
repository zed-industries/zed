use crate::{App, AppContext, VisualContext, Window, seal::Sealed};
use anyhow::{Result, anyhow};
use collections::FxHashSet;
use derive_more::{Deref, DerefMut};
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use slotmap::{KeyData, SecondaryMap, SlotMap};
use std::{
    any::{Any, TypeId, type_name},
    cell::RefCell,
    cmp::Ordering,
    fmt::{self, Display},
    hash::{Hash, Hasher},
    marker::PhantomData,
    mem,
    num::NonZeroU64,
    sync::{
        Arc, Weak,
        atomic::{AtomicUsize, Ordering::SeqCst},
    },
    thread::panicking,
};

#[cfg(any(test, feature = "leak-detection"))]
use collections::HashMap;

use super::Context;

slotmap::new_key_type! {
    /// A unique identifier for a entity across the application.
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
    pub accessed_entities: RefCell<FxHashSet<EntityId>>,
    ref_counts: Arc<RwLock<EntityRefCounts>>,
}

struct EntityRefCounts {
    counts: SlotMap<EntityId, AtomicUsize>,
    dropped_entity_ids: Vec<EntityId>,
    #[cfg(any(test, feature = "leak-detection"))]
    leak_detector: LeakDetector,
}

impl EntityMap {
    pub fn new() -> Self {
        Self {
            entities: SecondaryMap::new(),
            accessed_entities: RefCell::new(FxHashSet::default()),
            ref_counts: Arc::new(RwLock::new(EntityRefCounts {
                counts: SlotMap::with_key(),
                dropped_entity_ids: Vec::new(),
                #[cfg(any(test, feature = "leak-detection"))]
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
        Slot(Entity::new(id, Arc::downgrade(&self.ref_counts)))
    }

    /// Insert an entity into a slot obtained by calling `reserve`.
    pub fn insert<T>(&mut self, slot: Slot<T>, entity: T) -> Entity<T>
    where
        T: 'static,
    {
        let mut accessed_entities = self.accessed_entities.borrow_mut();
        accessed_entities.insert(slot.entity_id);

        let handle = slot.0;
        self.entities.insert(handle.entity_id, Box::new(entity));
        handle
    }

    /// Move an entity to the stack.
    #[track_caller]
    pub fn lease<'a, T>(&mut self, pointer: &'a Entity<T>) -> Lease<'a, T> {
        self.assert_valid_context(pointer);
        let mut accessed_entities = self.accessed_entities.borrow_mut();
        accessed_entities.insert(pointer.entity_id);

        let entity = Some(
            self.entities
                .remove(pointer.entity_id)
                .unwrap_or_else(|| double_lease_panic::<T>("update")),
        );
        Lease {
            entity,
            pointer,
            entity_type: PhantomData,
        }
    }

    /// Returns an entity after moving it to the stack.
    pub fn end_lease<T>(&mut self, mut lease: Lease<T>) {
        self.entities
            .insert(lease.pointer.entity_id, lease.entity.take().unwrap());
    }

    pub fn read<T: 'static>(&self, entity: &Entity<T>) -> &T {
        self.assert_valid_context(entity);
        let mut accessed_entities = self.accessed_entities.borrow_mut();
        accessed_entities.insert(entity.entity_id);

        self.entities
            .get(entity.entity_id)
            .and_then(|entity| entity.downcast_ref())
            .unwrap_or_else(|| double_lease_panic::<T>("read"))
    }

    fn assert_valid_context(&self, entity: &AnyEntity) {
        debug_assert!(
            Weak::ptr_eq(&entity.entity_map, &Arc::downgrade(&self.ref_counts)),
            "used a entity with the wrong context"
        );
    }

    pub fn extend_accessed(&mut self, entities: &FxHashSet<EntityId>) {
        self.accessed_entities
            .borrow_mut()
            .extend(entities.iter().copied());
    }

    pub fn clear_accessed(&mut self) {
        self.accessed_entities.borrow_mut().clear();
    }

    pub fn take_dropped(&mut self) -> Vec<(EntityId, Box<dyn Any>)> {
        let mut ref_counts = self.ref_counts.write();
        let dropped_entity_ids = mem::take(&mut ref_counts.dropped_entity_ids);
        let mut accessed_entities = self.accessed_entities.borrow_mut();

        dropped_entity_ids
            .into_iter()
            .filter_map(|entity_id| {
                let count = ref_counts.counts.remove(entity_id).unwrap();
                debug_assert_eq!(
                    count.load(SeqCst),
                    0,
                    "dropped an entity that was referenced"
                );
                accessed_entities.remove(&entity_id);
                // If the EntityId was allocated with `Context::reserve`,
                // the entity may not have been inserted.
                Some((entity_id, self.entities.remove(entity_id)?))
            })
            .collect()
    }
}

#[track_caller]
fn double_lease_panic<T>(operation: &str) -> ! {
    panic!(
        "cannot {operation} {} while it is already being updated",
        std::any::type_name::<T>()
    )
}

pub(crate) struct Lease<'a, T> {
    entity: Option<Box<dyn Any>>,
    pub pointer: &'a Entity<T>,
    entity_type: PhantomData<T>,
}

impl<T: 'static> core::ops::Deref for Lease<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.entity.as_ref().unwrap().downcast_ref().unwrap()
    }
}

impl<T: 'static> core::ops::DerefMut for Lease<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.entity.as_mut().unwrap().downcast_mut().unwrap()
    }
}

impl<T> Drop for Lease<'_, T> {
    fn drop(&mut self) {
        if self.entity.is_some() && !panicking() {
            panic!("Leases must be ended with EntityMap::end_lease")
        }
    }
}

#[derive(Deref, DerefMut)]
pub(crate) struct Slot<T>(Entity<T>);

/// A dynamically typed reference to a entity, which can be downcast into a `Entity<T>`.
pub struct AnyEntity {
    pub(crate) entity_id: EntityId,
    pub(crate) entity_type: TypeId,
    entity_map: Weak<RwLock<EntityRefCounts>>,
    #[cfg(any(test, feature = "leak-detection"))]
    handle_id: HandleId,
}

impl AnyEntity {
    fn new(id: EntityId, entity_type: TypeId, entity_map: Weak<RwLock<EntityRefCounts>>) -> Self {
        Self {
            entity_id: id,
            entity_type,
            entity_map: entity_map.clone(),
            #[cfg(any(test, feature = "leak-detection"))]
            handle_id: entity_map
                .upgrade()
                .unwrap()
                .write()
                .leak_detector
                .handle_created(id),
        }
    }

    /// Returns the id associated with this entity.
    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    /// Returns the [TypeId] associated with this entity.
    pub fn entity_type(&self) -> TypeId {
        self.entity_type
    }

    /// Converts this entity handle into a weak variant, which does not prevent it from being released.
    pub fn downgrade(&self) -> AnyWeakEntity {
        AnyWeakEntity {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
            entity_ref_counts: self.entity_map.clone(),
        }
    }

    /// Converts this entity handle into a strongly-typed entity handle of the given type.
    /// If this entity handle is not of the specified type, returns itself as an error variant.
    pub fn downcast<T: 'static>(self) -> Result<Entity<T>, AnyEntity> {
        if TypeId::of::<T>() == self.entity_type {
            Ok(Entity {
                any_entity: self,
                entity_type: PhantomData,
            })
        } else {
            Err(self)
        }
    }
}

impl Clone for AnyEntity {
    fn clone(&self) -> Self {
        if let Some(entity_map) = self.entity_map.upgrade() {
            let entity_map = entity_map.read();
            let count = entity_map
                .counts
                .get(self.entity_id)
                .expect("detected over-release of a entity");
            let prev_count = count.fetch_add(1, SeqCst);
            assert_ne!(prev_count, 0, "Detected over-release of a entity.");
        }

        Self {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
            entity_map: self.entity_map.clone(),
            #[cfg(any(test, feature = "leak-detection"))]
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

impl Drop for AnyEntity {
    fn drop(&mut self) {
        if let Some(entity_map) = self.entity_map.upgrade() {
            let entity_map = entity_map.upgradable_read();
            let count = entity_map
                .counts
                .get(self.entity_id)
                .expect("detected over-release of a handle.");
            let prev_count = count.fetch_sub(1, SeqCst);
            assert_ne!(prev_count, 0, "Detected over-release of a entity.");
            if prev_count == 1 {
                // We were the last reference to this entity, so we can remove it.
                let mut entity_map = RwLockUpgradableReadGuard::upgrade(entity_map);
                entity_map.dropped_entity_ids.push(self.entity_id);
            }
        }

        #[cfg(any(test, feature = "leak-detection"))]
        if let Some(entity_map) = self.entity_map.upgrade() {
            entity_map
                .write()
                .leak_detector
                .handle_released(self.entity_id, self.handle_id)
        }
    }
}

impl<T> From<Entity<T>> for AnyEntity {
    fn from(entity: Entity<T>) -> Self {
        entity.any_entity
    }
}

impl Hash for AnyEntity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entity_id.hash(state);
    }
}

impl PartialEq for AnyEntity {
    fn eq(&self, other: &Self) -> bool {
        self.entity_id == other.entity_id
    }
}

impl Eq for AnyEntity {}

impl Ord for AnyEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.entity_id.cmp(&other.entity_id)
    }
}

impl PartialOrd for AnyEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl std::fmt::Debug for AnyEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("AnyEntity")
            .field("entity_id", &self.entity_id.as_u64())
            .finish()
    }
}

/// A strong, well typed reference to a struct which is managed
/// by GPUI
#[derive(Deref, DerefMut)]
pub struct Entity<T> {
    #[deref]
    #[deref_mut]
    pub(crate) any_entity: AnyEntity,
    pub(crate) entity_type: PhantomData<T>,
}

unsafe impl<T> Send for Entity<T> {}
unsafe impl<T> Sync for Entity<T> {}
impl<T> Sealed for Entity<T> {}

impl<T: 'static> Entity<T> {
    fn new(id: EntityId, entity_map: Weak<RwLock<EntityRefCounts>>) -> Self
    where
        T: 'static,
    {
        Self {
            any_entity: AnyEntity::new(id, TypeId::of::<T>(), entity_map),
            entity_type: PhantomData,
        }
    }

    /// Get the entity ID associated with this entity
    pub fn entity_id(&self) -> EntityId {
        self.any_entity.entity_id
    }

    /// Downgrade this entity pointer to a non-retaining weak pointer
    pub fn downgrade(&self) -> WeakEntity<T> {
        WeakEntity {
            any_entity: self.any_entity.downgrade(),
            entity_type: self.entity_type,
        }
    }

    /// Convert this into a dynamically typed entity.
    pub fn into_any(self) -> AnyEntity {
        self.any_entity
    }

    /// Grab a reference to this entity from the context.
    pub fn read<'a>(&self, cx: &'a App) -> &'a T {
        cx.entities.read(self)
    }

    /// Read the entity referenced by this handle with the given function.
    pub fn read_with<R, C: AppContext>(
        &self,
        cx: &C,
        f: impl FnOnce(&T, &App) -> R,
    ) -> C::Result<R> {
        cx.read_entity(self, f)
    }

    /// Updates the entity referenced by this handle with the given function.
    pub fn update<R, C: AppContext>(
        &self,
        cx: &mut C,
        update: impl FnOnce(&mut T, &mut Context<T>) -> R,
    ) -> C::Result<R> {
        cx.update_entity(self, update)
    }

    /// Updates the entity referenced by this handle with the given function if
    /// the referenced entity still exists, within a visual context that has a window.
    /// Returns an error if the entity has been released.
    pub fn update_in<R, C: VisualContext>(
        &self,
        cx: &mut C,
        update: impl FnOnce(&mut T, &mut Window, &mut Context<T>) -> R,
    ) -> C::Result<R> {
        cx.update_window_entity(self, update)
    }
}

impl<T> Clone for Entity<T> {
    fn clone(&self) -> Self {
        Self {
            any_entity: self.any_entity.clone(),
            entity_type: self.entity_type,
        }
    }
}

impl<T> std::fmt::Debug for Entity<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Entity")
            .field("entity_id", &self.any_entity.entity_id)
            .field("entity_type", &type_name::<T>())
            .finish()
    }
}

impl<T> Hash for Entity<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.any_entity.hash(state);
    }
}

impl<T> PartialEq for Entity<T> {
    fn eq(&self, other: &Self) -> bool {
        self.any_entity == other.any_entity
    }
}

impl<T> Eq for Entity<T> {}

impl<T> PartialEq<WeakEntity<T>> for Entity<T> {
    fn eq(&self, other: &WeakEntity<T>) -> bool {
        self.any_entity.entity_id() == other.entity_id()
    }
}

impl<T: 'static> Ord for Entity<T> {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.entity_id().cmp(&other.entity_id())
    }
}

impl<T: 'static> PartialOrd for Entity<T> {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

/// A type erased, weak reference to a entity.
#[derive(Clone)]
pub struct AnyWeakEntity {
    pub(crate) entity_id: EntityId,
    entity_type: TypeId,
    entity_ref_counts: Weak<RwLock<EntityRefCounts>>,
}

impl AnyWeakEntity {
    /// Get the entity ID associated with this weak reference.
    pub fn entity_id(&self) -> EntityId {
        self.entity_id
    }

    /// Check if this weak handle can be upgraded, or if the entity has already been dropped
    pub fn is_upgradable(&self) -> bool {
        let ref_count = self
            .entity_ref_counts
            .upgrade()
            .and_then(|ref_counts| Some(ref_counts.read().counts.get(self.entity_id)?.load(SeqCst)))
            .unwrap_or(0);
        ref_count > 0
    }

    /// Upgrade this weak entity reference to a strong reference.
    pub fn upgrade(&self) -> Option<AnyEntity> {
        let ref_counts = &self.entity_ref_counts.upgrade()?;
        let ref_counts = ref_counts.read();
        let ref_count = ref_counts.counts.get(self.entity_id)?;

        // entity_id is in dropped_entity_ids
        if ref_count.load(SeqCst) == 0 {
            return None;
        }
        ref_count.fetch_add(1, SeqCst);
        drop(ref_counts);

        Some(AnyEntity {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
            entity_map: self.entity_ref_counts.clone(),
            #[cfg(any(test, feature = "leak-detection"))]
            handle_id: self
                .entity_ref_counts
                .upgrade()
                .unwrap()
                .write()
                .leak_detector
                .handle_created(self.entity_id),
        })
    }

    /// Assert that entity referenced by this weak handle has been released.
    #[cfg(any(test, feature = "leak-detection"))]
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

impl std::fmt::Debug for AnyWeakEntity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(type_name::<Self>())
            .field("entity_id", &self.entity_id)
            .field("entity_type", &self.entity_type)
            .finish()
    }
}

impl<T> From<WeakEntity<T>> for AnyWeakEntity {
    fn from(entity: WeakEntity<T>) -> Self {
        entity.any_entity
    }
}

impl Hash for AnyWeakEntity {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.entity_id.hash(state);
    }
}

impl PartialEq for AnyWeakEntity {
    fn eq(&self, other: &Self) -> bool {
        self.entity_id == other.entity_id
    }
}

impl Eq for AnyWeakEntity {}

impl Ord for AnyWeakEntity {
    fn cmp(&self, other: &Self) -> Ordering {
        self.entity_id.cmp(&other.entity_id)
    }
}

impl PartialOrd for AnyWeakEntity {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

/// A weak reference to a entity of the given type.
#[derive(Deref, DerefMut)]
pub struct WeakEntity<T> {
    #[deref]
    #[deref_mut]
    any_entity: AnyWeakEntity,
    entity_type: PhantomData<T>,
}

impl<T> std::fmt::Debug for WeakEntity<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct(&type_name::<Self>())
            .field("entity_id", &self.any_entity.entity_id)
            .field("entity_type", &type_name::<T>())
            .finish()
    }
}

unsafe impl<T> Send for WeakEntity<T> {}
unsafe impl<T> Sync for WeakEntity<T> {}

impl<T> Clone for WeakEntity<T> {
    fn clone(&self) -> Self {
        Self {
            any_entity: self.any_entity.clone(),
            entity_type: self.entity_type,
        }
    }
}

impl<T: 'static> WeakEntity<T> {
    /// Upgrade this weak entity reference into a strong entity reference
    pub fn upgrade(&self) -> Option<Entity<T>> {
        Some(Entity {
            any_entity: self.any_entity.upgrade()?,
            entity_type: self.entity_type,
        })
    }

    /// Updates the entity referenced by this handle with the given function if
    /// the referenced entity still exists. Returns an error if the entity has
    /// been released.
    pub fn update<C, R>(
        &self,
        cx: &mut C,
        update: impl FnOnce(&mut T, &mut Context<T>) -> R,
    ) -> Result<R>
    where
        C: AppContext,
        Result<C::Result<R>>: crate::Flatten<R>,
    {
        crate::Flatten::flatten(
            self.upgrade()
                .ok_or_else(|| anyhow!("entity released"))
                .map(|this| cx.update_entity(&this, update)),
        )
    }

    /// Updates the entity referenced by this handle with the given function if
    /// the referenced entity still exists, within a visual context that has a window.
    /// Returns an error if the entity has been released.
    pub fn update_in<C, R>(
        &self,
        cx: &mut C,
        update: impl FnOnce(&mut T, &mut Window, &mut Context<T>) -> R,
    ) -> Result<R>
    where
        C: VisualContext,
        Result<C::Result<R>>: crate::Flatten<R>,
    {
        let window = cx.window_handle();
        let this = self.upgrade().ok_or_else(|| anyhow!("entity released"))?;

        crate::Flatten::flatten(window.update(cx, |_, window, cx| {
            this.update(cx, |entity, cx| update(entity, window, cx))
        }))
    }

    /// Reads the entity referenced by this handle with the given function if
    /// the referenced entity still exists. Returns an error if the entity has
    /// been released.
    pub fn read_with<C, R>(&self, cx: &C, read: impl FnOnce(&T, &App) -> R) -> Result<R>
    where
        C: AppContext,
        Result<C::Result<R>>: crate::Flatten<R>,
    {
        crate::Flatten::flatten(
            self.upgrade()
                .ok_or_else(|| anyhow!("entity release"))
                .map(|this| cx.read_entity(&this, read)),
        )
    }
}

impl<T> Hash for WeakEntity<T> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.any_entity.hash(state);
    }
}

impl<T> PartialEq for WeakEntity<T> {
    fn eq(&self, other: &Self) -> bool {
        self.any_entity == other.any_entity
    }
}

impl<T> Eq for WeakEntity<T> {}

impl<T> PartialEq<Entity<T>> for WeakEntity<T> {
    fn eq(&self, other: &Entity<T>) -> bool {
        self.entity_id() == other.any_entity.entity_id()
    }
}

impl<T: 'static> Ord for WeakEntity<T> {
    fn cmp(&self, other: &Self) -> Ordering {
        self.entity_id().cmp(&other.entity_id())
    }
}

impl<T: 'static> PartialOrd for WeakEntity<T> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[cfg(any(test, feature = "leak-detection"))]
static LEAK_BACKTRACE: std::sync::LazyLock<bool> =
    std::sync::LazyLock::new(|| std::env::var("LEAK_BACKTRACE").map_or(false, |b| !b.is_empty()));

#[cfg(any(test, feature = "leak-detection"))]
#[derive(Clone, Copy, Debug, Default, Hash, PartialEq, Eq)]
pub(crate) struct HandleId {
    id: u64, // id of the handle itself, not the pointed at object
}

#[cfg(any(test, feature = "leak-detection"))]
pub(crate) struct LeakDetector {
    next_handle_id: u64,
    entity_handles: HashMap<EntityId, HashMap<HandleId, Option<backtrace::Backtrace>>>,
}

#[cfg(any(test, feature = "leak-detection"))]
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
