use alloc::sync::Arc;

use crate::{
    id::Id,
    identity::IdentityManager,
    lock::{rank, RwLock, RwLockReadGuard},
    storage::{Element, Storage, StorageItem},
};

#[derive(Copy, Clone, Debug, Default, PartialEq, Eq)]
pub struct RegistryReport {
    pub num_allocated: usize,
    pub num_kept_from_user: usize,
    pub num_released_from_user: usize,
    pub element_size: usize,
}

impl RegistryReport {
    pub fn is_empty(&self) -> bool {
        self.num_allocated + self.num_kept_from_user == 0
    }
}

/// Registry is the primary holder of each resource type
/// Every resource is now arcanized so the last arc released
/// will in the end free the memory and release the inner raw resource
///
/// Registry act as the main entry point to keep resource alive
/// when created and released from user land code
///
/// A resource may still be alive when released from user land code
/// if it's used in active submission or anyway kept alive from
/// any other dependent resource
///
#[derive(Debug)]
pub(crate) struct Registry<T: StorageItem> {
    // Must only contain an id which has either never been used or has been released from `storage`
    identity: Arc<IdentityManager<T::Marker>>,
    storage: RwLock<Storage<T>>,
}

impl<T: StorageItem> Registry<T> {
    pub(crate) fn new() -> Self {
        Self {
            identity: Arc::new(IdentityManager::new()),
            storage: RwLock::new(rank::REGISTRY_STORAGE, Storage::new()),
        }
    }
}

#[must_use]
pub(crate) struct FutureId<'a, T: StorageItem> {
    id: Id<T::Marker>,
    data: &'a RwLock<Storage<T>>,
}

impl<T: StorageItem> FutureId<'_, T> {
    /// Assign a new resource to this ID.
    ///
    /// Registers it with the registry.
    pub fn assign(self, value: T) -> Id<T::Marker> {
        let mut data = self.data.write();
        data.insert(self.id, value);
        self.id
    }
}

impl<T: StorageItem> Registry<T> {
    pub(crate) fn prepare(&self, id_in: Option<Id<T::Marker>>) -> FutureId<'_, T> {
        FutureId {
            id: match id_in {
                Some(id_in) => {
                    self.identity.mark_as_used(id_in);
                    id_in
                }
                None => self.identity.process(),
            },
            data: &self.storage,
        }
    }

    #[track_caller]
    pub(crate) fn read<'a>(&'a self) -> RwLockReadGuard<'a, Storage<T>> {
        self.storage.read()
    }
    pub(crate) fn remove(&self, id: Id<T::Marker>) -> T {
        let value = self.storage.write().remove(id);
        // This needs to happen *after* removing it from the storage, to maintain the
        // invariant that `self.identity` only contains ids which are actually available
        // See https://github.com/gfx-rs/wgpu/issues/5372
        self.identity.free(id);
        //Returning None is legal if it's an error ID
        value
    }

    pub(crate) fn generate_report(&self) -> RegistryReport {
        let storage = self.storage.read();
        let mut report = RegistryReport {
            element_size: size_of::<T>(),
            ..Default::default()
        };
        report.num_allocated = self.identity.values.lock().count();
        for element in storage.map.iter() {
            match *element {
                Element::Occupied(..) => report.num_kept_from_user += 1,
                Element::Vacant => report.num_released_from_user += 1,
            }
        }
        report
    }
}

impl<T: StorageItem + Clone> Registry<T> {
    pub(crate) fn get(&self, id: Id<T::Marker>) -> T {
        self.read().get(id)
    }
}

#[cfg(test)]
mod tests {
    use super::Registry;
    use crate::{id::Marker, resource::ResourceType, storage::StorageItem};
    use alloc::sync::Arc;

    struct TestData;
    struct TestDataId;
    impl Marker for TestDataId {}

    impl ResourceType for TestData {
        const TYPE: &'static str = "TestData";
    }
    impl StorageItem for TestData {
        type Marker = TestDataId;
    }

    #[test]
    fn simultaneous_registration() {
        let registry = Registry::new();
        std::thread::scope(|s| {
            for _ in 0..5 {
                s.spawn(|| {
                    for _ in 0..1000 {
                        let value = Arc::new(TestData);
                        let new_id = registry.prepare(None);
                        let id = new_id.assign(value);
                        registry.remove(id);
                    }
                });
            }
        })
    }
}
