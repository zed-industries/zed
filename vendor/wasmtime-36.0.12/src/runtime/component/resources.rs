use crate::component::func::{LiftContext, LowerContext, bad_type_info, desc};
use crate::component::matching::InstanceType;
use crate::component::{ComponentType, Lift, Lower};
use crate::prelude::*;
use crate::runtime::vm::component::{
    ComponentInstance, InstanceFlags, ResourceTables, TypedResource, TypedResourceIndex,
};
use crate::runtime::vm::{SendSyncPtr, VMFuncRef, ValRaw};
use crate::store::{StoreId, StoreOpaque};
use crate::{AsContextMut, StoreContextMut, Trap};
use core::any::TypeId;
use core::fmt;
use core::marker;
use core::mem::MaybeUninit;
use core::ptr::NonNull;
use core::sync::atomic::{AtomicU32, Ordering::Relaxed};
use wasmtime_environ::component::{
    CanonicalAbiInfo, ComponentTypes, DefinedResourceIndex, InterfaceType, ResourceIndex,
    TypeResourceTableIndex,
};

/// Representation of a resource type in the component model.
///
/// Resources are currently always represented as 32-bit integers but they have
/// unique types across instantiations and the host. For example instantiating
/// the same component twice means that defined resource types in the component
/// will all be different. Values of this type can be compared to see if
/// resources have the same type.
///
/// Resource types can also be defined on the host in addition to guests. On the
/// host resource types are tied to a `T`, an arbitrary Rust type. Two host
/// resource types are the same if they point to the same `T`.
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub struct ResourceType {
    kind: ResourceTypeKind,
}

impl ResourceType {
    /// Creates a new host resource type corresponding to `T`.
    ///
    /// Note that `T` is a mostly a phantom type parameter here. It does not
    /// need to reflect the actual storage of the resource `T`. For example this
    /// is valid:
    ///
    /// ```rust
    /// use wasmtime::component::ResourceType;
    ///
    /// struct Foo;
    ///
    /// let ty = ResourceType::host::<Foo>();
    /// ```
    ///
    /// A resource type of type `ResourceType::host::<T>()` will match the type
    /// of the value produced by `Resource::<T>::new_{own,borrow}`.
    pub fn host<T: 'static>() -> ResourceType {
        ResourceType {
            kind: ResourceTypeKind::Host(TypeId::of::<T>()),
        }
    }

    pub(crate) fn guest(
        store: StoreId,
        instance: &ComponentInstance,
        id: DefinedResourceIndex,
    ) -> ResourceType {
        ResourceType {
            kind: ResourceTypeKind::Guest {
                store,
                instance: instance as *const _ as usize,
                id,
            },
        }
    }

    pub(crate) fn uninstantiated(types: &ComponentTypes, index: ResourceIndex) -> ResourceType {
        ResourceType {
            kind: ResourceTypeKind::Uninstantiated {
                component: types as *const _ as usize,
                index,
            },
        }
    }
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
enum ResourceTypeKind {
    Host(TypeId),
    Guest {
        store: StoreId,
        // For now this is the `*mut ComponentInstance` pointer within the store
        // that this guest corresponds to. It's used to distinguish different
        // instantiations of the same component within the store.
        instance: usize,
        id: DefinedResourceIndex,
    },
    Uninstantiated {
        // Like `instance` in `Guest` above this is a pointer and is used to
        // distinguish between two components. Technically susceptible to ABA
        // issues but the consequence is a nonexistent resource would be equal
        // to a new resource so there's not really any issue with that.
        component: usize,
        index: ResourceIndex,
    },
}

/// A host-defined resource in the component model.
///
/// This type can be thought of as roughly a newtype wrapper around `u32` for
/// use as a resource with the component model. The main guarantee that the
/// component model provides is that the `u32` is not forgeable by guests and
/// there are guaranteed semantics about when a `u32` may be in use by the guest
/// and when it's guaranteed no longer needed. This means that it is safe for
/// embedders to consider the internal `u32` representation "trusted" and use it
/// for things like table indices with infallible accessors that panic on
/// out-of-bounds. This should only panic for embedder bugs, not because of any
/// possible behavior in the guest.
///
/// A `Resource<T>` value dynamically represents both an `(own $t)` in the
/// component model as well as a `(borrow $t)`. It can be inspected via
/// [`Resource::owned`] to test whether it is an owned handle. An owned handle
/// which is not actively borrowed can be destroyed at any time as it's
/// guaranteed that the guest does not have access to it. A borrowed handle, on
/// the other hand, may be accessed by the guest so it's not necessarily
/// guaranteed to be able to be destroyed.
///
/// Note that the "own" and "borrow" here refer to the component model, not
/// Rust. The semantics of Rust ownership and borrowing are slightly different
/// than the component model's (but spiritually the same) in that more dynamic
/// tracking is employed as part of the component model. This means that it's
/// possible to get runtime errors when using a `Resource<T>`. For example it is
/// an error to call [`Resource::new_borrow`] and pass that to a component
/// function expecting `(own $t)` and this is not statically disallowed.
///
/// The [`Resource`] type implements both the [`Lift`] and [`Lower`] trait to be
/// used with typed functions in the component model or as part of aggregate
/// structures and datatypes.
///
/// # Destruction of a resource
///
/// Resources in the component model are optionally defined with a destructor,
/// but this host resource type does not specify a destructor. It is left up to
/// the embedder to be able to determine how best to a destroy a resource when
/// it is owned.
///
/// Note, though, that while [`Resource`] itself does not specify destructors
/// it's still possible to do so via the [`Linker::resource`] definition. When a
/// resource type is defined for a guest component a destructor can be specified
/// which can be used to hook into resource destruction triggered by the guest.
///
/// This means that there are two ways that resource destruction is handled:
///
/// * Host resources destroyed by the guest can hook into the
///   [`Linker::resource`] destructor closure to handle resource destruction.
///   This could, for example, remove table entries.
///
/// * Host resources owned by the host itself have no automatic means of
///   destruction. The host can make its own determination that its own resource
///   is not lent out to the guest and at that time choose to destroy or
///   deallocate it.
///
/// # Dynamic behavior of a resource
///
/// A host-defined [`Resource`] does not necessarily represent a static value.
/// Its internals may change throughout its usage to track the state associated
/// with the resource. The internal 32-bit host-defined representation never
/// changes, however.
///
/// For example if there's a component model function of the form:
///
/// ```wasm
/// (func (param "a" (borrow $t)) (param "b" (own $t)))
/// ```
///
/// Then that can be extracted in Rust with:
///
/// ```rust,ignore
/// let func = instance.get_typed_func::<(&Resource<T>, &Resource<T>), ()>(&mut store, "name")?;
/// ```
///
/// Here the exact same resource can be provided as both arguments but that is
/// not valid to do so because the same resource cannot be actively borrowed and
/// passed by-value as the second parameter at the same time. The internal state
/// in `Resource<T>` will track this information and provide a dynamic runtime
/// error in this situation.
///
/// Mostly it's important to be aware that there is dynamic state associated
/// with a [`Resource<T>`] to provide errors in situations that cannot be
/// statically ruled out.
///
/// # Borrows and host responsibilities
///
/// Borrows to resources in the component model are guaranteed to be transient
/// such that if a borrow is passed as part of a function call then when the
/// function returns it's guaranteed that the guest no longer has access to the
/// resource. This guarantee, however, must be manually upheld by the host when
/// it receives its own borrow.
///
/// As mentioned above the [`Resource<T>`] type can represent a borrowed value
/// in addition to an owned value. This means a guest can provide the host with
/// a borrow, such as an argument to an imported function:
///
/// ```rust,ignore
/// linker.root().func_wrap("name", |_cx, (r,): (Resource<MyType>,)| {
///     assert!(!r.owned());
///     // .. here `r` is a borrowed value provided by the guest and the host
///     // shouldn't continue to access it beyond the scope of this call
/// })?;
/// ```
///
/// In this situation the host should take care to not attempt to persist the
/// resource beyond the scope of the call. It's the host's resource so it
/// technically can do what it wants with it but nothing is statically
/// preventing `r` to stay pinned to the lifetime of the closure invocation.
/// It's considered a mistake that the host performed if `r` is persisted too
/// long and accessed at the wrong time.
///
/// [`Linker::resource`]: crate::component::LinkerInstance::resource
pub struct Resource<T> {
    /// The host-defined 32-bit representation of this resource.
    rep: u32,

    /// Dear rust please consider `T` used even though it's not actually used.
    _marker: marker::PhantomData<fn() -> T>,

    state: AtomicResourceState,
}

/// Internal dynamic state tracking for this resource. This can be one of
/// four different states:
///
/// * `BORROW` / `u64::MAX` - this indicates that this is a borrowed
///   resource. The `rep` doesn't live in the host table and this `Resource`
///   instance is transiently available. It's the host's responsibility to
///   discard this resource when the borrow duration has finished.
///
/// * `NOT_IN_TABLE` / `u64::MAX - 1` - this indicates that this is an owned
///   resource not present in any store's table. This resource is not lent
///   out. It can be passed as an `(own $t)` directly into a guest's table
///   or it can be passed as a borrow to a guest which will insert it into
///   a host store's table for dynamic borrow tracking.
///
/// * `TAKEN` / `u64::MAX - 2` - while the `rep` is available the resource
///   has been dynamically moved into a guest and cannot be moved in again.
///   This is used for example to prevent the same resource from being
///   passed twice to a guest.
///
/// * All other values - any other value indicates that the value is an
///   index into a store's table of host resources. It's guaranteed that the
///   table entry represents a host resource and the resource may have
///   borrow tracking associated with it. The low 32-bits of the value are
///   the table index and the upper 32-bits are the generation.
///
/// Note that this is two `AtomicU32` fields but it's not intended to actually
/// be used in conjunction with threads as generally a `Store<T>` lives on one
/// thread at a time. The pair of `AtomicU32` here is used to ensure that this
/// type is `Send + Sync` when captured as a reference to make async
/// programming more ergonomic.
///
/// Also note that two `AtomicU32` here are used instead of `AtomicU64` to be
/// more portable to platforms without 64-bit atomics.
struct AtomicResourceState {
    index: AtomicU32,
    generation: AtomicU32,
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum ResourceState {
    Borrow,
    NotInTable,
    Taken,
    Index(HostResourceIndex),
}

impl AtomicResourceState {
    const BORROW: Self = AtomicResourceState::new(ResourceState::Borrow);
    const NOT_IN_TABLE: Self = AtomicResourceState::new(ResourceState::NotInTable);

    const fn new(state: ResourceState) -> AtomicResourceState {
        let (index, generation) = state.encode();
        Self {
            index: AtomicU32::new(index),
            generation: AtomicU32::new(generation),
        }
    }

    fn get(&self) -> ResourceState {
        ResourceState::decode(self.index.load(Relaxed), self.generation.load(Relaxed))
    }

    fn swap(&self, state: ResourceState) -> ResourceState {
        let (index, generation) = state.encode();
        let index_prev = self.index.load(Relaxed);
        self.index.store(index, Relaxed);
        let generation_prev = self.generation.load(Relaxed);
        self.generation.store(generation, Relaxed);
        ResourceState::decode(index_prev, generation_prev)
    }
}

impl ResourceState {
    // See comments on `state` above for info about these values.
    const BORROW: u32 = u32::MAX;
    const NOT_IN_TABLE: u32 = u32::MAX - 1;
    const TAKEN: u32 = u32::MAX - 2;

    fn decode(idx: u32, generation: u32) -> ResourceState {
        match generation {
            Self::BORROW => Self::Borrow,
            Self::NOT_IN_TABLE => Self::NotInTable,
            Self::TAKEN => Self::Taken,
            _ => Self::Index(HostResourceIndex::new(idx, generation)),
        }
    }

    const fn encode(&self) -> (u32, u32) {
        match self {
            Self::Borrow => (0, Self::BORROW),
            Self::NotInTable => (0, Self::NOT_IN_TABLE),
            Self::Taken => (0, Self::TAKEN),
            Self::Index(index) => (index.index(), index.generation()),
        }
    }
}

/// Metadata tracking the state of resources within a `Store`.
///
/// This is a borrowed structure created from a `Store` piecemeal from below.
/// The `ResourceTables` type holds most of the raw information and this
/// structure tacks on a reference to `HostResourceData` to track generation
/// numbers of host indices.
pub struct HostResourceTables<'a> {
    tables: ResourceTables<'a>,
    host_resource_data: &'a mut HostResourceData,
}

/// Metadata for host-owned resources owned within a `Store`.
///
/// This metadata is used to prevent the ABA problem with indices handed out as
/// part of `Resource` and `ResourceAny`. Those structures are `Copy` meaning
/// that it's easy to reuse them, possibly accidentally. To prevent issues in
/// the host Wasmtime attaches both an index (within `ResourceTables`) as well
/// as a 32-bit generation counter onto each `HostResourceIndex` which the host
/// actually holds in `Resource` and `ResourceAny`.
///
/// This structure holds a list which is a parallel list to the "list of reps"
/// that's stored within `ResourceTables` elsewhere in the `Store`. This
/// parallel list holds the last known generation of each element in the table.
/// The generation is then compared on access to make sure it's the same.
///
/// Whenever a slot in the table is allocated the `cur_generation` field is
/// pushed at the corresponding index of `generation_of_table_slot`. Whenever
/// a field is accessed the current value of `generation_of_table_slot` is
/// checked against the generation of the index. Whenever a slot is deallocated
/// the generation is incremented. Put together this means that any access of a
/// deallocated slot should deterministically provide an error.
#[derive(Default)]
pub struct HostResourceData {
    cur_generation: u32,
    table_slot_metadata: Vec<TableSlot>,
}

#[derive(Copy, Clone)]
struct TableSlot {
    generation: u32,
    flags: Option<InstanceFlags>,
    dtor: Option<SendSyncPtr<VMFuncRef>>,
}

/// Host representation of an index into a table slot.
///
/// This is morally (u32, u32) but is encoded as a 64-bit integer. The low
/// 32-bits are the table index and the upper 32-bits are the generation
/// counter.
#[derive(PartialEq, Eq, Debug, Copy, Clone)]
#[repr(transparent)]
pub struct HostResourceIndex(u64);

impl HostResourceIndex {
    fn new(idx: u32, generation: u32) -> HostResourceIndex {
        HostResourceIndex(u64::from(idx) | (u64::from(generation) << 32))
    }

    const fn index(&self) -> u32 {
        (self.0 & 0xffffffff) as u32
    }

    const fn generation(&self) -> u32 {
        (self.0 >> 32) as u32
    }
}

impl<'a> HostResourceTables<'a> {
    pub fn new_host(store: &'a mut StoreOpaque) -> HostResourceTables<'a> {
        let (calls, host_table, host_resource_data) = store.component_resource_state();
        HostResourceTables::from_parts(
            ResourceTables {
                host_table: Some(host_table),
                calls,
                guest: None,
            },
            host_resource_data,
        )
    }

    pub fn from_parts(
        tables: ResourceTables<'a>,
        host_resource_data: &'a mut HostResourceData,
    ) -> Self {
        HostResourceTables {
            tables,
            host_resource_data,
        }
    }

    /// Lifts an `own` resource that resides in the host's tables at the `idx`
    /// specified into its `rep`.
    ///
    /// # Errors
    ///
    /// Returns an error if `idx` doesn't point to a valid owned resource, or
    /// if `idx` can't be lifted as an `own` (e.g. it has active borrows).
    pub fn host_resource_lift_own(&mut self, idx: HostResourceIndex) -> Result<u32> {
        let (idx, _) = self.validate_host_index(idx, true)?;
        self.tables.resource_lift_own(TypedResourceIndex::Host(idx))
    }

    /// See [`HostResourceTables::host_resource_lift_own`].
    pub fn host_resource_lift_borrow(&mut self, idx: HostResourceIndex) -> Result<u32> {
        let (idx, _) = self.validate_host_index(idx, false)?;
        self.tables
            .resource_lift_borrow(TypedResourceIndex::Host(idx))
    }

    /// Lowers an `own` resource to be owned by the host.
    ///
    /// This returns a new index into the host's set of resource tables which
    /// will point to the `rep` specified. The returned index is suitable for
    /// conversion into either [`Resource`] or [`ResourceAny`].
    ///
    /// The `dtor` and instance `flags` are specified as well to know what
    /// destructor to run when this resource is destroyed.
    pub fn host_resource_lower_own(
        &mut self,
        rep: u32,
        dtor: Option<NonNull<VMFuncRef>>,
        flags: Option<InstanceFlags>,
    ) -> Result<HostResourceIndex> {
        let idx = self.tables.resource_lower_own(TypedResource::Host(rep))?;
        Ok(self.new_host_index(idx, dtor, flags))
    }

    /// See [`HostResourceTables::host_resource_lower_own`].
    pub fn host_resource_lower_borrow(&mut self, rep: u32) -> Result<HostResourceIndex> {
        let idx = self
            .tables
            .resource_lower_borrow(TypedResource::Host(rep))?;
        Ok(self.new_host_index(idx, None, None))
    }

    /// Validates that `idx` is still valid for the host tables, notably
    /// ensuring that the generation listed in `idx` is the same as the
    /// last recorded generation of the slot itself.
    ///
    /// The `is_removal` option indicates whether or not this table access will
    /// end up removing the element from the host table. In such a situation the
    /// current generation number is incremented.
    fn validate_host_index(
        &mut self,
        idx: HostResourceIndex,
        is_removal: bool,
    ) -> Result<(u32, Option<TableSlot>)> {
        let actual = usize::try_from(idx.index())
            .ok()
            .and_then(|i| self.host_resource_data.table_slot_metadata.get(i).copied());

        // If `idx` is out-of-bounds then skip returning an error. In such a
        // situation the operation that this is guarding will return a more
        // precise error, such as a lift operation.
        if let Some(actual) = actual {
            if actual.generation != idx.generation() {
                bail!("host-owned resource is being used with the wrong type");
            }
        }

        // Bump the current generation of this is a removal to ensure any
        // future item placed in this slot can't be pointed to by the `idx`
        // provided above.
        if is_removal {
            self.host_resource_data.cur_generation += 1;
        }

        Ok((idx.index(), actual))
    }

    /// Creates a new `HostResourceIndex` which will point to the raw table
    /// slot provided by `idx`.
    ///
    /// This will register metadata necessary to track the current generation
    /// in the returned `HostResourceIndex` as well.
    fn new_host_index(
        &mut self,
        idx: u32,
        dtor: Option<NonNull<VMFuncRef>>,
        flags: Option<InstanceFlags>,
    ) -> HostResourceIndex {
        let list = &mut self.host_resource_data.table_slot_metadata;
        let info = TableSlot {
            generation: self.host_resource_data.cur_generation,
            flags,
            dtor: dtor.map(SendSyncPtr::new),
        };
        match list.get_mut(idx as usize) {
            Some(slot) => *slot = info,
            None => {
                // Resource handles start at 1, not zero, so push two elements
                // for the first resource handle.
                if list.is_empty() {
                    assert_eq!(idx, 1);
                    list.push(TableSlot {
                        generation: 0,
                        flags: None,
                        dtor: None,
                    });
                }
                assert_eq!(idx as usize, list.len());
                list.push(info);
            }
        }

        HostResourceIndex::new(idx, info.generation)
    }

    /// Drops a host-owned resource from host tables.
    ///
    /// This method will attempt to interpret `idx` as pointing to either a
    /// `borrow` or `own` resource with the `expected` type specified. This
    /// method will then return the underlying `rep` if it points to an `own`
    /// resource which can then be further processed for destruction.
    ///
    /// # Errors
    ///
    /// Returns an error if `idx` doesn't point to a valid resource, points to
    /// an `own` with active borrows, or if it doesn't have the type `expected`
    /// in the host tables.
    fn host_resource_drop(&mut self, idx: HostResourceIndex) -> Result<Option<(u32, TableSlot)>> {
        let (idx, slot) = self.validate_host_index(idx, true)?;
        match self.tables.resource_drop(TypedResourceIndex::Host(idx))? {
            Some(rep) => Ok(Some((rep, slot.unwrap()))),
            None => Ok(None),
        }
    }

    /// Lowers an `own` resource into the guest, converting the `rep` specified
    /// into a guest-local index.
    ///
    /// The `ty` provided is which table to put this into.
    pub fn guest_resource_lower_own(
        &mut self,
        rep: u32,
        ty: TypeResourceTableIndex,
    ) -> Result<u32> {
        self.tables
            .resource_lower_own(TypedResource::Component { ty, rep })
    }

    /// Lowers a `borrow` resource into the guest, converting the `rep`
    /// specified into a guest-local index.
    ///
    /// The `ty` provided is which table to put this into.
    ///
    /// Note that this cannot be used in isolation because lowering a borrow
    /// into a guest has a special case where `rep` is returned directly if `ty`
    /// belongs to the component being lowered into. That property must be
    /// handled by the caller of this function.
    pub fn guest_resource_lower_borrow(
        &mut self,
        rep: u32,
        ty: TypeResourceTableIndex,
    ) -> Result<u32> {
        self.tables
            .resource_lower_borrow(TypedResource::Component { ty, rep })
    }

    /// Lifts an `own` resource from the `idx` specified from the table `ty`.
    ///
    /// This will lookup the appropriate table in the guest and return the `rep`
    /// corresponding to `idx` if it's valid.
    pub fn guest_resource_lift_own(
        &mut self,
        index: u32,
        ty: TypeResourceTableIndex,
    ) -> Result<u32> {
        self.tables
            .resource_lift_own(TypedResourceIndex::Component { ty, index })
    }

    /// Lifts a `borrow` resource from the `idx` specified from the table `ty`.
    ///
    /// This will lookup the appropriate table in the guest and return the `rep`
    /// corresponding to `idx` if it's valid.
    pub fn guest_resource_lift_borrow(
        &mut self,
        index: u32,
        ty: TypeResourceTableIndex,
    ) -> Result<u32> {
        self.tables
            .resource_lift_borrow(TypedResourceIndex::Component { ty, index })
    }

    /// Begins a call into the component instance, starting recording of
    /// metadata related to resource borrowing.
    #[inline]
    pub fn enter_call(&mut self) {
        self.tables.enter_call()
    }

    /// Completes a call into the component instance, validating that it's ok to
    /// complete by ensuring the are no remaining active borrows.
    #[inline]
    pub fn exit_call(&mut self) -> Result<()> {
        self.tables.exit_call()
    }
}

impl<T> Resource<T>
where
    T: 'static,
{
    /// Creates a new owned resource with the `rep` specified.
    ///
    /// The returned value is suitable for passing to a guest as either a
    /// `(borrow $t)` or `(own $t)`.
    pub fn new_own(rep: u32) -> Resource<T> {
        Resource {
            state: AtomicResourceState::NOT_IN_TABLE,
            rep,
            _marker: marker::PhantomData,
        }
    }

    /// Creates a new borrowed resource which isn't actually rooted in any
    /// ownership.
    ///
    /// This can be used to pass to a guest as a borrowed resource and the
    /// embedder will know that the `rep` won't be in use by the guest
    /// afterwards. Exactly how the lifetime of `rep` works is up to the
    /// embedder.
    pub fn new_borrow(rep: u32) -> Resource<T> {
        Resource {
            state: AtomicResourceState::BORROW,
            rep,
            _marker: marker::PhantomData,
        }
    }

    /// Returns the underlying 32-bit representation used to originally create
    /// this resource.
    pub fn rep(&self) -> u32 {
        self.rep
    }

    /// Returns whether this is an owned resource or not.
    ///
    /// Owned resources can be safely destroyed by the embedder at any time, and
    /// borrowed resources have an owner somewhere else on the stack so can only
    /// be accessed, not destroyed.
    pub fn owned(&self) -> bool {
        match self.state.get() {
            ResourceState::Borrow => false,
            ResourceState::Taken | ResourceState::NotInTable | ResourceState::Index(_) => true,
        }
    }

    fn lower_to_index<U>(&self, cx: &mut LowerContext<'_, U>, ty: InterfaceType) -> Result<u32> {
        match ty {
            InterfaceType::Own(t) => {
                let rep = match self.state.get() {
                    // If this is a borrow resource then this is a dynamic
                    // error on behalf of the embedder.
                    ResourceState::Borrow => {
                        bail!("cannot lower a `borrow` resource into an `own`")
                    }

                    // If this resource does not yet live in a table then we're
                    // dynamically transferring ownership to wasm. Record that
                    // it's no longer present and then pass through the
                    // representation.
                    ResourceState::NotInTable => {
                        let prev = self.state.swap(ResourceState::Taken);
                        assert_eq!(prev, ResourceState::NotInTable);
                        self.rep
                    }

                    // This resource has already been moved into wasm so this is
                    // a dynamic error on behalf of the embedder.
                    ResourceState::Taken => bail!("host resource already consumed"),

                    // If this resource lives in a host table then try to take
                    // it out of the table, which may fail, and on success we
                    // can move the rep into the guest table.
                    ResourceState::Index(idx) => cx.host_resource_lift_own(idx)?,
                };
                cx.guest_resource_lower_own(t, rep)
            }
            InterfaceType::Borrow(t) => {
                let rep = match self.state.get() {
                    // If this is already a borrowed resource, nothing else to
                    // do and the rep is passed through.
                    ResourceState::Borrow => self.rep,

                    // If this resource is already gone, that's a dynamic error
                    // for the embedder.
                    ResourceState::Taken => bail!("host resource already consumed"),

                    // If this resource is not currently in a table then it
                    // needs to move into a table to participate in state
                    // related to borrow tracking. Execute the
                    // `host_resource_lower_own` operation here and update our
                    // state.
                    //
                    // Afterwards this is the same as the `idx` case below.
                    //
                    // Note that flags/dtor are passed as `None` here since
                    // `Resource<T>` doesn't offer destruction support.
                    ResourceState::NotInTable => {
                        let idx = cx.host_resource_lower_own(self.rep, None, None)?;
                        let prev = self.state.swap(ResourceState::Index(idx));
                        assert_eq!(prev, ResourceState::NotInTable);
                        cx.host_resource_lift_borrow(idx)?
                    }

                    // If this resource lives in a table then it needs to come
                    // out of the table with borrow-tracking employed.
                    ResourceState::Index(idx) => cx.host_resource_lift_borrow(idx)?,
                };
                cx.guest_resource_lower_borrow(t, rep)
            }
            _ => bad_type_info(),
        }
    }

    fn lift_from_index(cx: &mut LiftContext<'_>, ty: InterfaceType, index: u32) -> Result<Self> {
        let (state, rep) = match ty {
            // Ownership is being transferred from a guest to the host, so move
            // it from the guest table into a new `Resource`. Note that this
            // isn't immediately inserted into the host table and that's left
            // for the future if it's necessary to take a borrow from this owned
            // resource.
            InterfaceType::Own(t) => {
                debug_assert!(cx.resource_type(t) == ResourceType::host::<T>());
                let (rep, dtor, flags) = cx.guest_resource_lift_own(t, index)?;
                assert!(dtor.is_some());
                assert!(flags.is_none());
                (AtomicResourceState::NOT_IN_TABLE, rep)
            }

            // The borrow here is lifted from the guest, but note the lack of
            // `host_resource_lower_borrow` as it's intentional. Lowering
            // a borrow has a special case in the canonical ABI where if the
            // receiving module is the owner of the resource then it directly
            // receives the `rep` and no other dynamic tracking is employed.
            // This effectively mirrors that even though the canonical ABI
            // isn't really all that applicable in host context here.
            InterfaceType::Borrow(t) => {
                debug_assert!(cx.resource_type(t) == ResourceType::host::<T>());
                let rep = cx.guest_resource_lift_borrow(t, index)?;
                (AtomicResourceState::BORROW, rep)
            }
            _ => bad_type_info(),
        };
        Ok(Resource {
            state,
            rep,
            _marker: marker::PhantomData,
        })
    }

    /// Attempts to convert a [`ResourceAny`] into [`Resource`].
    ///
    /// This method will check that `resource` has type
    /// `ResourceType::host::<T>()` and then convert it into a typed version of
    /// the resource.
    ///
    /// # Errors
    ///
    /// This function will return an error if `resource` does not have type
    /// `ResourceType::host::<T>()`. This function may also return an error if
    /// `resource` is no longer valid, for example it was previously converted.
    ///
    /// # Panics
    ///
    /// This function will panic if `resource` does not belong to the `store`
    /// specified.
    pub fn try_from_resource_any(
        resource: ResourceAny,
        mut store: impl AsContextMut,
    ) -> Result<Self> {
        let store = store.as_context_mut();
        let mut tables = HostResourceTables::new_host(store.0);
        let ResourceAny { idx, ty, owned } = resource;
        ensure!(ty == ResourceType::host::<T>(), "resource type mismatch");
        let (state, rep) = if owned {
            let rep = tables.host_resource_lift_own(idx)?;
            (AtomicResourceState::NOT_IN_TABLE, rep)
        } else {
            // For borrowed handles, first acquire the `rep` via lifting the
            // borrow. Afterwards though remove any dynamic state associated
            // with this borrow. `Resource<T>` doesn't participate in dynamic
            // state tracking and it's assumed embedders know what they're
            // doing, so the drop call will clear out that a borrow is active
            //
            // Note that the result of `drop` should always be `None` as it's a
            // borrowed handle, so assert so.
            let rep = tables.host_resource_lift_borrow(idx)?;
            let res = tables.host_resource_drop(idx)?;
            assert!(res.is_none());
            (AtomicResourceState::BORROW, rep)
        };
        Ok(Resource {
            state,
            rep,
            _marker: marker::PhantomData,
        })
    }

    /// See [`ResourceAny::try_from_resource`]
    pub fn try_into_resource_any(self, store: impl AsContextMut) -> Result<ResourceAny> {
        ResourceAny::try_from_resource(self, store)
    }
}

unsafe impl<T: 'static> ComponentType for Resource<T> {
    const ABI: CanonicalAbiInfo = CanonicalAbiInfo::SCALAR4;

    type Lower = <u32 as ComponentType>::Lower;

    fn typecheck(ty: &InterfaceType, types: &InstanceType<'_>) -> Result<()> {
        let resource = match ty {
            InterfaceType::Own(t) | InterfaceType::Borrow(t) => *t,
            other => bail!("expected `own` or `borrow`, found `{}`", desc(other)),
        };
        match types.resource_type(resource).kind {
            ResourceTypeKind::Host(id) if TypeId::of::<T>() == id => {}
            _ => bail!("resource type mismatch"),
        }

        Ok(())
    }
}

unsafe impl<T: 'static> Lower for Resource<T> {
    fn linear_lower_to_flat<U>(
        &self,
        cx: &mut LowerContext<'_, U>,
        ty: InterfaceType,
        dst: &mut MaybeUninit<Self::Lower>,
    ) -> Result<()> {
        self.lower_to_index(cx, ty)?
            .linear_lower_to_flat(cx, InterfaceType::U32, dst)
    }

    fn linear_lower_to_memory<U>(
        &self,
        cx: &mut LowerContext<'_, U>,
        ty: InterfaceType,
        offset: usize,
    ) -> Result<()> {
        self.lower_to_index(cx, ty)?
            .linear_lower_to_memory(cx, InterfaceType::U32, offset)
    }
}

unsafe impl<T: 'static> Lift for Resource<T> {
    fn linear_lift_from_flat(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        src: &Self::Lower,
    ) -> Result<Self> {
        let index = u32::linear_lift_from_flat(cx, InterfaceType::U32, src)?;
        Resource::lift_from_index(cx, ty, index)
    }

    fn linear_lift_from_memory(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        bytes: &[u8],
    ) -> Result<Self> {
        let index = u32::linear_lift_from_memory(cx, InterfaceType::U32, bytes)?;
        Resource::lift_from_index(cx, ty, index)
    }
}

impl<T> fmt::Debug for Resource<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let state = match self.state.get() {
            ResourceState::Borrow => "borrow",
            ResourceState::NotInTable => "own (not in table)",
            ResourceState::Taken => "taken",
            ResourceState::Index(_) => "own",
        };
        f.debug_struct("Resource")
            .field("rep", &self.rep)
            .field("state", &state)
            .finish()
    }
}

/// Representation of a resource in the component model, either a guest-defined
/// or a host-defined resource.
///
/// This type is similar to [`Resource`] except that it can be used to represent
/// any resource, either host or guest. This type cannot be directly constructed
/// and is only available if the guest returns it to the host (e.g. a function
/// returning a guest-defined resource) or by a conversion from [`Resource`] via
/// [`ResourceAny::try_from_resource`].
/// This type also does not carry a static type parameter `T` for example and
/// does not have as much information about its type.
/// This means that it's possible to get runtime type-errors when
/// using this type because it cannot statically prevent mismatching resource
/// types.
///
/// Like [`Resource`] this type represents either an `own` or a `borrow`
/// resource internally. Unlike [`Resource`], however, a [`ResourceAny`] must
/// always be explicitly destroyed with the [`ResourceAny::resource_drop`]
/// method. This will update internal dynamic state tracking and invoke the
/// WebAssembly-defined destructor for a resource, if any.
///
/// Note that it is required to call `resource_drop` for all instances of
/// [`ResourceAny`]: even borrows. Both borrows and own handles have state
/// associated with them that must be discarded by the time they're done being
/// used.
#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct ResourceAny {
    idx: HostResourceIndex,
    ty: ResourceType,
    owned: bool,
}

impl ResourceAny {
    /// Attempts to convert an imported [`Resource`] into [`ResourceAny`].
    ///
    /// * `resource` is the resource to convert.
    /// * `store` is the store to place the returned resource into.
    ///
    /// The returned `ResourceAny` will not have a destructor attached to it
    /// meaning that if `resource_drop` is called then it will not invoked a
    /// host-defined destructor. This is similar to how `Resource<T>` does not
    /// have a destructor associated with it.
    ///
    /// # Errors
    ///
    /// This method will return an error if `resource` has already been "taken"
    /// and has ownership transferred elsewhere which can happen in situations
    /// such as when it's already lowered into a component.
    pub fn try_from_resource<T: 'static>(
        resource: Resource<T>,
        mut store: impl AsContextMut,
    ) -> Result<Self> {
        let Resource { rep, state, .. } = resource;
        let store = store.as_context_mut();

        let mut tables = HostResourceTables::new_host(store.0);
        let (idx, owned) = match state.get() {
            ResourceState::Borrow => (tables.host_resource_lower_borrow(rep)?, false),
            ResourceState::NotInTable => {
                let idx = tables.host_resource_lower_own(rep, None, None)?;
                (idx, true)
            }
            ResourceState::Taken => bail!("host resource already consumed"),
            ResourceState::Index(idx) => (idx, true),
        };
        Ok(Self {
            idx,
            ty: ResourceType::host::<T>(),
            owned,
        })
    }

    /// See [`Resource::try_from_resource_any`]
    pub fn try_into_resource<T: 'static>(self, store: impl AsContextMut) -> Result<Resource<T>> {
        Resource::try_from_resource_any(self, store)
    }

    /// Returns the corresponding type associated with this resource, either a
    /// host-defined type or a guest-defined type.
    ///
    /// This can be compared against [`ResourceType::host`] for example to see
    /// if it's a host-resource or against a type extracted with
    /// [`Instance::get_resource`] to see if it's a guest-defined resource.
    ///
    /// [`Instance::get_resource`]: crate::component::Instance::get_resource
    pub fn ty(&self) -> ResourceType {
        self.ty
    }

    /// Returns whether this is an owned resource, and if not it's a borrowed
    /// resource.
    pub fn owned(&self) -> bool {
        self.owned
    }

    /// Destroy this resource and release any state associated with it.
    ///
    /// This is required to be called (or the async version) for all instances
    /// of [`ResourceAny`] to ensure that state associated with this resource is
    /// properly cleaned up. For owned resources this may execute the
    /// guest-defined destructor if applicable (or the host-defined destructor
    /// if one was specified).
    pub fn resource_drop(self, mut store: impl AsContextMut) -> Result<()> {
        let mut store = store.as_context_mut();
        assert!(
            !store.0.async_support(),
            "must use `resource_drop_async` when async support is enabled on the config"
        );
        self.resource_drop_impl(&mut store.as_context_mut())
    }

    /// Same as [`ResourceAny::resource_drop`] except for use with async stores
    /// to execute the destructor asynchronously.
    #[cfg(feature = "async")]
    pub async fn resource_drop_async<T>(
        self,
        mut store: impl AsContextMut<Data: Send>,
    ) -> Result<()> {
        let mut store = store.as_context_mut();
        assert!(
            store.0.async_support(),
            "cannot use `resource_drop_async` without enabling async support in the config"
        );
        store
            .on_fiber(|store| self.resource_drop_impl(store))
            .await?
    }

    fn resource_drop_impl<T: 'static>(self, store: &mut StoreContextMut<'_, T>) -> Result<()> {
        // Attempt to remove `self.idx` from the host table in `store`.
        //
        // This could fail if the index is invalid or if this is removing an
        // `Own` entry which is currently being borrowed.
        let pair = HostResourceTables::new_host(store.0).host_resource_drop(self.idx)?;

        let (rep, slot) = match (pair, self.owned) {
            (Some(pair), true) => pair,

            // A `borrow` was removed from the table and no further
            // destruction, e.g. the destructor, is required so we're done.
            (None, false) => return Ok(()),

            _ => unreachable!(),
        };

        // Implement the reentrance check required by the canonical ABI. Note
        // that this happens whether or not a destructor is present.
        //
        // Note that this should be safe because the raw pointer access in
        // `flags` is valid due to `store` being the owner of the flags and
        // flags are never destroyed within the store.
        if let Some(flags) = slot.flags {
            unsafe {
                if !flags.may_enter() {
                    bail!(Trap::CannotEnterComponent);
                }
            }
        }

        let dtor = match slot.dtor {
            Some(dtor) => dtor.as_non_null(),
            None => return Ok(()),
        };
        let mut args = [ValRaw::u32(rep)];

        // This should be safe because `dtor` has been checked to belong to the
        // `store` provided which means it's valid and still alive. Additionally
        // destructors have al been previously type-checked and are guaranteed
        // to take one i32 argument and return no results, so the parameters
        // here should be configured correctly.
        unsafe { crate::Func::call_unchecked_raw(store, dtor, NonNull::from(&mut args)) }
    }

    fn lower_to_index<U>(&self, cx: &mut LowerContext<'_, U>, ty: InterfaceType) -> Result<u32> {
        match ty {
            InterfaceType::Own(t) => {
                if cx.resource_type(t) != self.ty {
                    bail!("mismatched resource types");
                }
                let rep = cx.host_resource_lift_own(self.idx)?;
                cx.guest_resource_lower_own(t, rep)
            }
            InterfaceType::Borrow(t) => {
                if cx.resource_type(t) != self.ty {
                    bail!("mismatched resource types");
                }
                let rep = cx.host_resource_lift_borrow(self.idx)?;
                cx.guest_resource_lower_borrow(t, rep)
            }
            _ => bad_type_info(),
        }
    }

    fn lift_from_index(cx: &mut LiftContext<'_>, ty: InterfaceType, index: u32) -> Result<Self> {
        match ty {
            InterfaceType::Own(t) => {
                let ty = cx.resource_type(t);
                let (rep, dtor, flags) = cx.guest_resource_lift_own(t, index)?;
                let idx = cx.host_resource_lower_own(rep, dtor, flags)?;
                Ok(ResourceAny {
                    idx,
                    ty,
                    owned: true,
                })
            }
            InterfaceType::Borrow(t) => {
                let ty = cx.resource_type(t);
                let rep = cx.guest_resource_lift_borrow(t, index)?;
                let idx = cx.host_resource_lower_borrow(rep)?;
                Ok(ResourceAny {
                    idx,
                    ty,
                    owned: false,
                })
            }
            _ => bad_type_info(),
        }
    }
}

unsafe impl ComponentType for ResourceAny {
    const ABI: CanonicalAbiInfo = CanonicalAbiInfo::SCALAR4;

    type Lower = <u32 as ComponentType>::Lower;

    fn typecheck(ty: &InterfaceType, _types: &InstanceType<'_>) -> Result<()> {
        match ty {
            InterfaceType::Own(_) | InterfaceType::Borrow(_) => Ok(()),
            other => bail!("expected `own` or `borrow`, found `{}`", desc(other)),
        }
    }
}

unsafe impl Lower for ResourceAny {
    fn linear_lower_to_flat<T>(
        &self,
        cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        dst: &mut MaybeUninit<Self::Lower>,
    ) -> Result<()> {
        self.lower_to_index(cx, ty)?
            .linear_lower_to_flat(cx, InterfaceType::U32, dst)
    }

    fn linear_lower_to_memory<T>(
        &self,
        cx: &mut LowerContext<'_, T>,
        ty: InterfaceType,
        offset: usize,
    ) -> Result<()> {
        self.lower_to_index(cx, ty)?
            .linear_lower_to_memory(cx, InterfaceType::U32, offset)
    }
}

unsafe impl Lift for ResourceAny {
    fn linear_lift_from_flat(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        src: &Self::Lower,
    ) -> Result<Self> {
        let index = u32::linear_lift_from_flat(cx, InterfaceType::U32, src)?;
        ResourceAny::lift_from_index(cx, ty, index)
    }

    fn linear_lift_from_memory(
        cx: &mut LiftContext<'_>,
        ty: InterfaceType,
        bytes: &[u8],
    ) -> Result<Self> {
        let index = u32::linear_lift_from_memory(cx, InterfaceType::U32, bytes)?;
        ResourceAny::lift_from_index(cx, ty, index)
    }
}
