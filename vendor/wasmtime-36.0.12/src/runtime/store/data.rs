use crate::runtime::vm::{self, VMStore};
use crate::store::StoreOpaque;
use crate::{StoreContext, StoreContextMut};
use core::num::NonZeroU64;
use core::ops::{Index, IndexMut};
use core::pin::Pin;

// This is defined here, in a private submodule, so we can explicitly reexport
// it only as `pub(crate)`. This avoids a ton of
// crate-private-type-in-public-interface errors that aren't really too
// interesting to deal with.
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct InstanceId(u32);
wasmtime_environ::entity_impl!(InstanceId);

pub struct StoreData {
    id: StoreId,
    #[cfg(feature = "component-model")]
    pub(crate) components: crate::component::ComponentStoreData,
}

impl StoreData {
    pub fn new() -> StoreData {
        StoreData {
            id: StoreId::allocate(),
            #[cfg(feature = "component-model")]
            components: Default::default(),
        }
    }

    pub fn id(&self) -> StoreId {
        self.id
    }
}

// forward StoreOpaque => StoreData
impl<I> Index<I> for StoreOpaque
where
    StoreData: Index<I>,
{
    type Output = <StoreData as Index<I>>::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.store_data.index(index)
    }
}

impl<I> IndexMut<I> for StoreOpaque
where
    StoreData: IndexMut<I>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.store_data.index_mut(index)
    }
}

// forward StoreContext => StoreOpaque
impl<I, T> Index<I> for StoreContext<'_, T>
where
    StoreOpaque: Index<I>,
{
    type Output = <StoreOpaque as Index<I>>::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.0.index(index)
    }
}

// forward StoreContextMut => StoreOpaque
impl<I, T> Index<I> for StoreContextMut<'_, T>
where
    StoreOpaque: Index<I>,
{
    type Output = <StoreOpaque as Index<I>>::Output;

    #[inline]
    fn index(&self, index: I) -> &Self::Output {
        self.0.index(index)
    }
}

impl<I, T> IndexMut<I> for StoreContextMut<'_, T>
where
    StoreOpaque: IndexMut<I>,
{
    #[inline]
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.0.index_mut(index)
    }
}

// forward dyn VMStore => StoreOpaque
impl<I> Index<I> for dyn VMStore + '_
where
    StoreOpaque: Index<I>,
{
    type Output = <StoreOpaque as Index<I>>::Output;

    fn index(&self, index: I) -> &Self::Output {
        self.store_opaque().index(index)
    }
}

impl<I> IndexMut<I> for dyn VMStore + '_
where
    StoreOpaque: IndexMut<I>,
{
    fn index_mut(&mut self, index: I) -> &mut Self::Output {
        self.store_opaque_mut().index_mut(index)
    }
}

/// A unique identifier to get attached to a store.
///
/// This identifier is embedded into the `Stored<T>` structure and is used to
/// identify the original store that items come from. For example a `Memory` is
/// owned by a `Store` and will embed a `StoreId` internally to say which store
/// it came from. Comparisons with this value are how panics are generated for
/// mismatching the item that a store belongs to.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Hash)]
#[repr(transparent)] // NB: relied on in the C API
pub struct StoreId(NonZeroU64);

impl StoreId {
    /// Allocates a new unique identifier for a store that has never before been
    /// used in this process.
    pub fn allocate() -> StoreId {
        // When 64-bit atomics are allowed then allow 2^63 stores at which point
        // we start panicking to prevent overflow.
        //
        // If a store is created once per microsecond then this will last the
        // current process for 584,540 years before overflowing.
        const OVERFLOW_THRESHOLD: u64 = 1 << 63;

        #[cfg(target_has_atomic = "64")]
        let id = {
            use core::sync::atomic::{AtomicU64, Ordering::Relaxed};

            // Note the usage of `Relaxed` ordering here which should be ok
            // since we're only looking for atomicity on this counter and this
            // otherwise isn't used to synchronize memory stored anywhere else.
            static NEXT_ID: AtomicU64 = AtomicU64::new(0);
            let id = NEXT_ID.fetch_add(1, Relaxed);
            if id > OVERFLOW_THRESHOLD {
                NEXT_ID.store(OVERFLOW_THRESHOLD, Relaxed);
                panic!("store id allocator overflow");
            }
            id
        };

        // When 64-bit atomics are not allowed use a `RwLock<u64>`. This is
        // already used elsewhere in Wasmtime and currently has the
        // implementation of panic-on-contention, but it's at least no worse
        // than what wasmtime had before and is at least correct and UB-free.
        #[cfg(not(target_has_atomic = "64"))]
        let id = {
            use crate::sync::RwLock;
            static NEXT_ID: RwLock<u64> = RwLock::new(0);

            let mut lock = NEXT_ID.write();
            if *lock > OVERFLOW_THRESHOLD {
                panic!("store id allocator overflow");
            }
            let ret = *lock;
            *lock += 1;
            ret
        };

        StoreId(NonZeroU64::new(id + 1).unwrap())
    }

    #[inline]
    pub fn assert_belongs_to(&self, store: StoreId) {
        if *self == store {
            return;
        }
        store_id_mismatch();
    }

    /// Raw accessor for the C API.
    pub fn as_raw(&self) -> NonZeroU64 {
        self.0
    }

    /// Raw constructor for the C API.
    pub fn from_raw(id: NonZeroU64) -> StoreId {
        StoreId(id)
    }
}

#[cold]
fn store_id_mismatch() {
    panic!("object used with the wrong store");
}

/// A type used to represent an allocated `vm::Instance` located within a store.
///
/// This type is held in various locations as a "safe index" into a store. This
/// encapsulates a `StoreId` which owns the instance as well as the index within
/// the store's list of which instance it's pointing to.
///
/// This type can notably be used to index into a `StoreOpaque` to project out
/// the `vm::Instance` that is associated with this id.
#[repr(C)] // used by reference in the C API
#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub struct StoreInstanceId {
    store_id: StoreId,
    instance: InstanceId,
}

impl StoreInstanceId {
    pub(crate) fn new(store_id: StoreId, instance: InstanceId) -> StoreInstanceId {
        StoreInstanceId { store_id, instance }
    }

    #[inline]
    pub fn assert_belongs_to(&self, store: StoreId) {
        self.store_id.assert_belongs_to(store)
    }

    #[inline]
    pub fn store_id(&self) -> StoreId {
        self.store_id
    }

    #[inline]
    pub(crate) fn instance(&self) -> InstanceId {
        self.instance
    }

    /// Looks up the `vm::Instance` within `store` that this id points to.
    ///
    /// # Panics
    ///
    /// Panics if `self` does not belong to `store`.
    #[inline]
    pub(crate) fn get<'a>(&self, store: &'a StoreOpaque) -> &'a vm::Instance {
        self.assert_belongs_to(store.id());
        store.instance(self.instance)
    }

    /// Mutable version of `get` above.
    ///
    /// # Panics
    ///
    /// Panics if `self` does not belong to `store`.
    #[inline]
    pub(crate) fn get_mut<'a>(&self, store: &'a mut StoreOpaque) -> Pin<&'a mut vm::Instance> {
        self.assert_belongs_to(store.id());
        store.instance_mut(self.instance)
    }
}

impl Index<StoreInstanceId> for StoreOpaque {
    type Output = vm::Instance;

    #[inline]
    fn index(&self, id: StoreInstanceId) -> &Self::Output {
        id.get(self)
    }
}
