use crate::runtime::vm::component::{ComponentInstance, OwnedComponentInstance};
use crate::store::{StoreData, StoreId, StoreOpaque};
use crate::{AsContext, AsContextMut, Store, StoreContextMut};
#[cfg(feature = "component-model-async")]
use alloc::vec::Vec;
use core::pin::Pin;
use wasmtime_environ::PrimaryMap;

/// Default amount of fuel allowed for all guest-to-host calls in the component
/// model.
///
/// This is the maximal amount of data which will be copied from the guest to
/// the host by default. This is set to a very large value to avoid breaking
/// existing embeddings for when this feature was backported.
const DEFAULT_HOSTCALL_FUEL: usize = 2 << 30;

pub struct ComponentStoreData {
    instances: PrimaryMap<ComponentInstanceId, Option<OwnedComponentInstance>>,

    /// Fuel to be used for each time the guest calls the host or transfers data
    /// to the host.
    ///
    /// Caps the size of the allocations made on the host to this amount
    /// effectively.
    hostcall_fuel: usize,
}

impl Default for ComponentStoreData {
    fn default() -> Self {
        Self {
            instances: PrimaryMap::default(),
            hostcall_fuel: DEFAULT_HOSTCALL_FUEL,
        }
    }
}

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct ComponentInstanceId(u32);
wasmtime_environ::entity_impl!(ComponentInstanceId);

impl StoreData {
    pub(crate) fn push_component_instance(
        &mut self,
        data: OwnedComponentInstance,
    ) -> ComponentInstanceId {
        let expected = data.get().id();
        let ret = self.components.instances.push(Some(data));
        assert_eq!(expected, ret);
        ret
    }
}

impl ComponentStoreData {
    pub fn next_component_instance_id(&self) -> ComponentInstanceId {
        self.instances.next_key()
    }

    #[cfg(feature = "component-model-async")]
    pub(crate) fn drop_fibers_and_futures(store: &mut StoreOpaque) {
        let mut fibers = Vec::new();
        let mut futures = Vec::new();
        for (_, instance) in store.store_data_mut().components.instances.iter_mut() {
            let Some(instance) = instance.as_mut() else {
                continue;
            };

            instance
                .get_mut()
                .concurrent_state_mut()
                .take_fibers_and_futures(&mut fibers, &mut futures);
        }

        for mut fiber in fibers {
            fiber.dispose(store);
        }

        crate::component::concurrent::tls::set(store.traitobj_mut(), move || drop(futures));
    }
}

impl StoreData {
    pub(crate) fn component_instance(&self, id: ComponentInstanceId) -> &ComponentInstance {
        self.components.instances[id].as_ref().unwrap().get()
    }

    pub(crate) fn component_instance_mut(
        &mut self,
        id: ComponentInstanceId,
    ) -> Pin<&mut ComponentInstance> {
        self.components.instances[id].as_mut().unwrap().get_mut()
    }
}

impl StoreOpaque {
    pub(crate) fn component_instance(&self, id: ComponentInstanceId) -> &ComponentInstance {
        self.store_data().component_instance(id)
    }

    pub(crate) fn hostcall_fuel(&self) -> usize {
        self.store_data().components.hostcall_fuel
    }

    pub(crate) fn set_hostcall_fuel(&mut self, fuel: usize) {
        self.store_data_mut().components.hostcall_fuel = fuel;
    }
}

/// A type used to represent an allocated `ComponentInstance` located within a
/// store.
///
/// This type is held in various locations as a "safe index" into a store. This
/// encapsulates a `StoreId` which owns the instance as well as the index within
/// the store's list of which instance it's pointing to.
///
/// This type can notably be used to index into a `StoreOpaque` to project out
/// the `ComponentInstance` that is associated with this id.
#[repr(C)] // used by reference in the C API
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StoreComponentInstanceId {
    store_id: StoreId,
    instance: ComponentInstanceId,
}

impl StoreComponentInstanceId {
    pub(crate) fn new(
        store_id: StoreId,
        instance: ComponentInstanceId,
    ) -> StoreComponentInstanceId {
        StoreComponentInstanceId { store_id, instance }
    }

    #[inline]
    pub fn assert_belongs_to(&self, store: StoreId) {
        self.store_id.assert_belongs_to(store)
    }

    #[inline]
    pub(crate) fn store_id(&self) -> StoreId {
        self.store_id
    }

    #[inline]
    pub(crate) fn instance(&self) -> ComponentInstanceId {
        self.instance
    }

    /// Looks up the `vm::ComponentInstance` within `store` that this id points
    /// to.
    ///
    /// # Panics
    ///
    /// Panics if `self` does not belong to `store`.
    pub(crate) fn get<'a>(&self, store: &'a StoreOpaque) -> &'a ComponentInstance {
        self.assert_belongs_to(store.id());
        store.component_instance(self.instance)
    }

    /// Mutable version of `get` above.
    ///
    /// # Panics
    ///
    /// Panics if `self` does not belong to `store`.
    pub(crate) fn get_mut<'a>(&self, store: &'a mut StoreOpaque) -> Pin<&'a mut ComponentInstance> {
        self.from_data_get_mut(store.store_data_mut())
    }

    /// Same as `get_mut`, but borrows less of a store.
    pub(crate) fn from_data_get_mut<'a>(
        &self,
        store: &'a mut StoreData,
    ) -> Pin<&'a mut ComponentInstance> {
        self.assert_belongs_to(store.id());
        store.component_instance_mut(self.instance)
    }
}

impl<T> Store<T> {
    /// Returns the amount of "hostcall fuel" used for guest-to-host component
    /// calls.
    ///
    /// This is either the default amount if it hasn't been configured or
    /// returns the last value passed to [`Store::set_hostcall_fuel`].
    ///
    /// See [`Store::set_hostcall_fuel`] `for more details.
    pub fn hostcall_fuel(&self) -> usize {
        self.as_context().0.hostcall_fuel()
    }

    /// Sets the amount of "hostcall fuel" used for guest-to-host component
    /// calls.
    ///
    /// Whenever the guest calls the host it often wants to transfer some data
    /// as well, such as strings or lists. This configured fuel value can be
    /// used to limit the amount of data that the host allocates on behalf of
    /// the guest. This is a DoS mitigation mechanism to prevent a malicious
    /// guest from causing the host to allocate an unbounded amount of memory
    /// for example.
    ///
    /// Fuel is considered distinct for each host call. The host is responsible
    /// for ensuring it retains a proper amount of data between host calls if
    /// applicable. The `fuel` provided here will be the initial value for each
    /// time the guest calls the host.
    ///
    /// The `fuel` value here should roughly corresponds to the maximal number
    /// of bytes that the guest may transfer to the host in one call.
    ///
    /// Note that data transferred from the host to the guest is not limited
    /// because it's already resident on the host itself. Only data from the
    /// guest to the host is limited.
    ///
    /// The default value for this is 128 MiB.
    pub fn set_hostcall_fuel(&mut self, fuel: usize) {
        self.as_context_mut().set_hostcall_fuel(fuel)
    }
}

impl<T> StoreContextMut<'_, T> {
    /// See [`Store::hostcall_fuel`].
    pub fn hostcall_fuel(&self) -> usize {
        self.0.hostcall_fuel()
    }

    /// See [`Store::set_hostcall_fuel`].
    pub fn set_hostcall_fuel(&mut self, fuel: usize) {
        self.0.set_hostcall_fuel(fuel)
    }
}
