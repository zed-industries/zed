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
        Slot(Model::new(id, Arc::downgrade(&self.ref_counts)))
    }

    /// Insert an entity into a slot obtained by calling `reserve`.
    pub fn insert<T>(&mut self, slot: Slot<T>, entity: T) -> Model<T>
    where
        T: 'static + Send,
    {
        let model = slot.0;
        self.entities.insert(model.entity_id, Box::new(entity));
        model
    }

    /// Move an entity to the stack.
    pub fn lease<'a, T>(&mut self, model: &'a Model<T>) -> Lease<'a, T> {
        self.assert_valid_context(model);
        let entity = Some(
            self.entities
                .remove(model.entity_id)
                .expect("Circular entity lease. Is the entity already being updated?"),
        );
        Lease {
            model,
            entity,
            entity_type: PhantomData,
        }
    }

    /// Return an entity after moving it to the stack.
    pub fn end_lease<T>(&mut self, mut lease: Lease<T>) {
        self.entities
            .insert(lease.model.entity_id, lease.entity.take().unwrap());
    }

    pub fn read<T: 'static>(&self, model: &Model<T>) -> &T {
        self.assert_valid_context(model);
        self.entities[model.entity_id].downcast_ref().unwrap()
    }

    fn assert_valid_context(&self, model: &AnyModel) {
        debug_assert!(
            Weak::ptr_eq(&model.entity_map, &Arc::downgrade(&self.ref_counts)),
            "used a model with the wrong context"
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
        if self.entity.is_some() {
            // We don't panic here, because other panics can cause us to drop the lease without ending it cleanly.
            log::error!("Leases must be ended with EntityMap::end_lease")
        }
    }
}

#[derive(Deref, DerefMut)]
pub struct Slot<T>(Model<T>);

pub struct AnyModel {
    pub(crate) entity_id: EntityId,
    pub(crate) entity_type: TypeId,
    entity_map: Weak<RwLock<EntityRefCounts>>,
}

impl AnyModel {
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

    pub fn downgrade(&self) -> AnyWeakModel {
        AnyWeakModel {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
            entity_ref_counts: self.entity_map.clone(),
        }
    }

    pub fn downcast<T: 'static>(&self) -> Option<Model<T>> {
        if TypeId::of::<T>() == self.entity_type {
            Some(Model {
                any_model: self.clone(),
                entity_type: PhantomData,
            })
        } else {
            None
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
                .expect("Detected over-release of a model.");
            let prev_count = count.fetch_sub(1, SeqCst);
            assert_ne!(prev_count, 0, "Detected over-release of a model.");
            if prev_count == 1 {
                // We were the last reference to this entity, so we can remove it.
                let mut entity_map = RwLockUpgradableReadGuard::upgrade(entity_map);
                entity_map.dropped_entity_ids.push(self.entity_id);
            }
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

#[derive(Deref, DerefMut)]
pub struct Model<T> {
    #[deref]
    #[deref_mut]
    pub(crate) any_model: AnyModel,
    pub(crate) entity_type: PhantomData<T>,
}

unsafe impl<T> Send for Model<T> {}
unsafe impl<T> Sync for Model<T> {}

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

    pub fn downgrade(&self) -> WeakModel<T> {
        WeakModel {
            any_model: self.any_model.downgrade(),
            entity_type: self.entity_type,
        }
    }

    /// Convert this into a dynamically typed model.
    pub fn into_any(self) -> AnyModel {
        self.any_model
    }

    pub fn read<'a>(&self, cx: &'a AppContext) -> &'a T {
        cx.entities.read(self)
    }

    /// Update the entity referenced by this model with the given function.
    ///
    /// The update function receives a context appropriate for its environment.
    /// When updating in an `AppContext`, it receives a `ModelContext`.
    /// When updating an a `WindowContext`, it receives a `ViewContext`.
    pub fn update<C, R>(
        &self,
        cx: &mut C,
        update: impl FnOnce(&mut T, &mut C::ModelContext<'_, T>) -> R,
    ) -> C::Result<R>
    where
        C: Context,
    {
        cx.update_entity(self, update)
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
        self.entity_id() == other.entity_id()
    }
}

#[derive(Clone)]
pub struct AnyWeakModel {
    pub(crate) entity_id: EntityId,
    entity_type: TypeId,
    entity_ref_counts: Weak<RwLock<EntityRefCounts>>,
}

impl AnyWeakModel {
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

    pub fn upgrade(&self) -> Option<AnyModel> {
        let entity_map = self.entity_ref_counts.upgrade()?;
        entity_map
            .read()
            .counts
            .get(self.entity_id)?
            .fetch_add(1, SeqCst);
        Some(AnyModel {
            entity_id: self.entity_id,
            entity_type: self.entity_type,
            entity_map: self.entity_ref_counts.clone(),
        })
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
    pub fn upgrade(&self) -> Option<Model<T>> {
        Some(Model {
            any_model: self.any_model.upgrade()?,
            entity_type: self.entity_type,
        })
    }

    /// Update the entity referenced by this model with the given function if
    /// the referenced entity still exists. Returns an error if the entity has
    /// been released.
    ///
    /// The update function receives a context appropriate for its environment.
    /// When updating in an `AppContext`, it receives a `ModelContext`.
    /// When updating an a `WindowContext`, it receives a `ViewContext`.
    pub fn update<C, R>(
        &self,
        cx: &mut C,
        update: impl FnOnce(&mut T, &mut C::ModelContext<'_, T>) -> R,
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
        self.entity_id() == other.entity_id()
    }
}
