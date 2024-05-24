use crate::{seal::Sealed, AppContext, Context, Entity, ModelContext};
use anyhow::{anyhow, Result};
use derive_more::{Deref, DerefMut};
use parking_lot::{RwLock, RwLockUpgradableReadGuard};
use slotmap::{KeyData, SecondaryMap, SlotMap};
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
        f: impl FnOnce(&T, &AppContext) -> R,
    ) -> C::Result<R> {
        cx.read_model(self, f)
    }

    /// Updates the entity referenced by this model with the given function.
    ///
    /// The update function receives a context appropriate for its environment.
    /// When updating in an `AppContext`, it receives a `ModelContext`.
    /// When updating in a `WindowContext`, it receives a `ViewContext`.
    pub fn update<C, R>(
        &self,
        cx: &mut C,
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
    ) -> C::Result<R>
    where
        C: Context,
    {
        cx.update_model(self, update)
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
        write!(
            f,
            "Model {{ entity_id: {:?}, entity_type: {:?} }}",
            self.any_model.entity_id,
            type_name::<T>()
        )
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
        update: impl FnOnce(&mut T, &mut ModelContext<'_, T>) -> R,
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

    /// Reads the entity referenced by this model with the given function if
    /// the referenced entity still exists. Returns an error if the entity has
    /// been released.
    pub fn read_with<C, R>(&self, cx: &C, read: impl FnOnce(&T, &AppContext) -> R) -> Result<R>
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
lazy_static::lazy_static! {
    static ref LEAK_BACKTRACE: bool =
        std::env::var("LEAK_BACKTRACE").map_or(false, |b| !b.is_empty());
}

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
            LEAK_BACKTRACE.then(|| backtrace::Backtrace::new_unresolved()),
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
            for (_, backtrace) in handles {
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
