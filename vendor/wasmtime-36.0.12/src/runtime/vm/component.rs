//! Runtime support for the component model in Wasmtime
//!
//! Currently this runtime support includes a `VMComponentContext` which is
//! similar in purpose to `VMContext`. The context is read from
//! cranelift-generated trampolines when entering the host from a wasm module.
//! Eventually it's intended that module-to-module calls, which would be
//! cranelift-compiled adapters, will use this `VMComponentContext` as well.

use crate::component::{Component, Instance, InstancePre, ResourceType, RuntimeImport};
use crate::runtime::component::ComponentInstanceId;
use crate::runtime::vm::instance::{InstanceLayout, OwnedInstance, OwnedVMContext};
use crate::runtime::vm::vmcontext::VMFunctionBody;
use crate::runtime::vm::{
    SendSyncPtr, VMArrayCallFunction, VMFuncRef, VMGlobalDefinition, VMMemoryDefinition,
    VMOpaqueContext, VMStore, VMStoreRawPtr, VMTableImport, VMWasmCallFunction, ValRaw, VmPtr,
    VmSafe,
};
use crate::store::InstanceId;
use alloc::alloc::Layout;
use alloc::sync::Arc;
use core::mem;
use core::mem::offset_of;
use core::pin::Pin;
use core::ptr::NonNull;
use wasmtime_environ::component::*;
use wasmtime_environ::{HostPtr, PrimaryMap, VMSharedTypeIndex};

#[allow(
    clippy::cast_possible_truncation,
    reason = "it's intended this is truncated on 32-bit platforms"
)]
const INVALID_PTR: usize = 0xdead_dead_beef_beef_u64 as usize;

mod libcalls;
mod resources;

#[cfg(feature = "component-model-async")]
pub use self::resources::CallContext;
pub use self::resources::{
    CallContexts, ResourceTable, ResourceTables, TypedResource, TypedResourceIndex,
};

#[cfg(feature = "component-model-async")]
use crate::component::concurrent;

/// Runtime representation of a component instance and all state necessary for
/// the instance itself.
///
/// This type never exists by-value, but rather it's always behind a pointer.
/// The size of the allocation for `ComponentInstance` includes the trailing
/// `VMComponentContext` which is variably sized based on the `offsets`
/// contained within.
///
/// # Pin
///
/// Note that this type is mutated through `Pin<&mut ComponentInstance>` in the
/// same manner as `vm::Instance` for core modules, and see more information
/// over there for documentation and rationale.
#[repr(C)]
pub struct ComponentInstance {
    /// The index within the store of where to find this component instance.
    id: ComponentInstanceId,

    /// Size and offset information for the trailing `VMComponentContext`.
    offsets: VMComponentOffsets<HostPtr>,

    /// The component that this instance was created from.
    //
    // NB: in the future if necessary it would be possible to avoid storing an
    // entire `Component` here and instead storing only information such as:
    //
    // * Some reference to `Arc<ComponentTypes>`
    // * Necessary references to closed-over modules which are exported from the
    //   component itself.
    //
    // Otherwise the full guts of this component should only ever be used during
    // the instantiation of this instance, meaning that after instantiation much
    // of the component can be thrown away (theoretically).
    component: Component,

    /// State of resources for this component.
    ///
    /// This is paired with other information to create a `ResourceTables` which
    /// is how this field is manipulated.
    instance_resource_tables: PrimaryMap<RuntimeComponentInstanceIndex, ResourceTable>,

    /// State related to async for this component, e.g. futures, streams, tasks,
    /// etc.
    #[cfg(feature = "component-model-async")]
    concurrent_state: concurrent::ConcurrentState,

    /// What all compile-time-identified core instances are mapped to within the
    /// `Store` that this component belongs to.
    instances: PrimaryMap<RuntimeInstanceIndex, InstanceId>,

    /// Storage for the type information about resources within this component
    /// instance.
    resource_types: Arc<PrimaryMap<ResourceIndex, ResourceType>>,

    /// Arguments that this instance used to be instantiated.
    ///
    /// Strong references are stored to these arguments since pointers are saved
    /// into the structures such as functions within the
    /// `OwnedComponentInstance` but it's our job to keep them alive.
    ///
    /// One purpose of this storage is to enable embedders to drop a `Linker`,
    /// for example, after a component is instantiated. In that situation if the
    /// arguments weren't held here then they might be dropped, and structures
    /// such as `.lowering()` which point back into the original function would
    /// become stale and use-after-free conditions when used. By preserving the
    /// entire list here though we're guaranteed that nothing is lost for the
    /// duration of the lifetime of this instance.
    imports: Arc<PrimaryMap<RuntimeImportIndex, RuntimeImport>>,

    /// Self-pointer back to `Store<T>` and its functions.
    store: VMStoreRawPtr,

    /// Cached ABI return value from the last-invoked function call along with
    /// the function index that was invoked.
    ///
    /// Used in `post_return_arg_set` and `post_return_arg_take` below.
    post_return_arg: Option<(ExportIndex, ValRaw)>,

    /// Required by `InstanceLayout`, also required to be the last field (with
    /// repr(C))
    vmctx: OwnedVMContext<VMComponentContext>,
}

/// Type signature for host-defined trampolines that are called from
/// WebAssembly.
///
/// This function signature is invoked from a cranelift-compiled trampoline that
/// adapts from the core wasm System-V ABI into the ABI provided here:
///
/// * `vmctx` - this is the first argument to the wasm import, and should always
///   end up being a `VMComponentContext`.
/// * `data` - this is the data pointer associated with the `VMLowering` for
///   which this function pointer was registered.
/// * `ty` - the type index, relative to the tables in `vmctx`, that is the
///   type of the function being called.
/// * `options` - the `OptionsIndex` which indicates the canonical ABI options
///   in use for this call.
/// * `args_and_results` - pointer to stack-allocated space in the caller where
///   all the arguments are stored as well as where the results will be written
///   to. The size and initialized bytes of this depends on the core wasm type
///   signature that this callee corresponds to.
/// * `nargs_and_results` - the size, in units of `ValRaw`, of
///   `args_and_results`.
///
/// This function returns a `bool` which indicates whether the call succeeded
/// or not. On failure this function records trap information in TLS which
/// should be suitable for reading later.
pub type VMLoweringCallee = extern "C" fn(
    vmctx: NonNull<VMOpaqueContext>,
    data: NonNull<u8>,
    ty: u32,
    options: u32,
    args_and_results: NonNull<mem::MaybeUninit<ValRaw>>,
    nargs_and_results: usize,
) -> bool;

/// An opaque function pointer which is a `VMLoweringFunction` under the hood
/// but this is stored as `VMPtr<VMLoweringFunction>` within `VMLowering` below
/// to handle provenance correctly when using Pulley.
#[repr(transparent)]
pub struct VMLoweringFunction(VMFunctionBody);

/// Structure describing a lowered host function stored within a
/// `VMComponentContext` per-lowering.
#[derive(Copy, Clone)]
#[repr(C)]
pub struct VMLowering {
    /// The host function pointer that is invoked when this lowering is
    /// invoked.
    pub callee: VmPtr<VMLoweringFunction>,
    /// The host data pointer (think void* pointer) to get passed to `callee`.
    pub data: VmPtr<u8>,
}

// SAFETY: the above structure is repr(C) and only contains `VmSafe` fields.
unsafe impl VmSafe for VMLowering {}

/// This is a marker type to represent the underlying allocation of a
/// `VMComponentContext`.
///
/// This type is similar to `VMContext` for core wasm and is allocated once per
/// component instance in Wasmtime. While the static size of this type is 0 the
/// actual runtime size is variable depending on the shape of the component that
/// this corresponds to. This structure always trails a `ComponentInstance`
/// allocation and the allocation/lifetime of this allocation is managed by
/// `ComponentInstance`.
#[repr(C)]
// Set an appropriate alignment for this structure where the most-aligned value
// internally right now `VMGlobalDefinition` which has an alignment of 16 bytes.
#[repr(align(16))]
pub struct VMComponentContext;

impl ComponentInstance {
    /// Converts the `vmctx` provided into a `ComponentInstance` and runs the
    /// provided closure with that instance.
    ///
    /// # Unsafety
    ///
    /// This is `unsafe` because `vmctx` cannot be guaranteed to be a valid
    /// pointer and it cannot be proven statically that it's safe to get a
    /// mutable reference at this time to the instance from `vmctx`. Note that
    /// it must be also safe to borrow the store mutably, meaning it can't
    /// already be in use elsewhere.
    pub unsafe fn from_vmctx<R>(
        vmctx: NonNull<VMComponentContext>,
        f: impl FnOnce(&mut dyn VMStore, Instance) -> R,
    ) -> R {
        // SAFETY: it's a contract of this function that `vmctx` is a valid
        // allocation which can go backwards to a `ComponentInstance`.
        let mut ptr = unsafe {
            vmctx
                .byte_sub(mem::size_of::<ComponentInstance>())
                .cast::<ComponentInstance>()
        };
        // SAFETY: it's a contract of this function that it's safe to use `ptr`
        // as a mutable reference.
        let reference = unsafe { ptr.as_mut() };

        // SAFETY: it's a contract of this function that it's safe to use the
        // store mutably at this time.
        let store = unsafe { &mut *reference.store.0.as_ptr() };

        let instance = Instance::from_wasmtime(store, reference.id);
        f(store, instance)
    }

    /// Returns the `InstanceId` associated with the `vmctx` provided.
    ///
    /// # Safety
    ///
    /// The `vmctx` pointer must be a valid pointer to read the
    /// `ComponentInstanceId` from.
    pub(crate) unsafe fn vmctx_instance_id(
        vmctx: NonNull<VMComponentContext>,
    ) -> ComponentInstanceId {
        // SAFETY: it's a contract of this function that `vmctx` is a valid
        // pointer with a `ComponentInstance` in front which can be read.
        unsafe {
            vmctx
                .byte_sub(mem::size_of::<ComponentInstance>())
                .cast::<ComponentInstance>()
                .as_ref()
                .id
        }
    }

    /// Returns the layout corresponding to what would be an allocation of a
    /// `ComponentInstance` for the `offsets` provided.
    ///
    /// The returned layout has space for both the `ComponentInstance` and the
    /// trailing `VMComponentContext`.
    fn alloc_layout(offsets: &VMComponentOffsets<HostPtr>) -> Layout {
        let size = mem::size_of::<Self>()
            .checked_add(usize::try_from(offsets.size_of_vmctx()).unwrap())
            .unwrap();
        let align = mem::align_of::<Self>();
        Layout::from_size_align(size, align).unwrap()
    }

    /// Allocates a new `ComponentInstance + VMComponentContext` pair on the
    /// heap with `malloc` and configures it for the `component` specified.
    pub(crate) fn new(
        id: ComponentInstanceId,
        component: &Component,
        resource_types: Arc<PrimaryMap<ResourceIndex, ResourceType>>,
        imports: &Arc<PrimaryMap<RuntimeImportIndex, RuntimeImport>>,
        store: NonNull<dyn VMStore>,
    ) -> OwnedComponentInstance {
        let offsets = VMComponentOffsets::new(HostPtr, component.env_component());
        let num_instances = component.env_component().num_runtime_component_instances;
        let mut instance_resource_tables =
            PrimaryMap::with_capacity(num_instances.try_into().unwrap());
        for _ in 0..num_instances {
            instance_resource_tables.push(ResourceTable::default());
        }

        let mut ret = OwnedInstance::new(ComponentInstance {
            id,
            offsets,
            instance_resource_tables,
            instances: PrimaryMap::with_capacity(
                component
                    .env_component()
                    .num_runtime_instances
                    .try_into()
                    .unwrap(),
            ),
            component: component.clone(),
            resource_types,
            imports: imports.clone(),
            store: VMStoreRawPtr(store),
            post_return_arg: None,
            #[cfg(feature = "component-model-async")]
            concurrent_state: concurrent::ConcurrentState::new(component),
            vmctx: OwnedVMContext::new(),
        });
        unsafe {
            ret.get_mut().initialize_vmctx();
        }
        ret
    }

    #[inline]
    pub fn vmctx(&self) -> NonNull<VMComponentContext> {
        InstanceLayout::vmctx(self)
    }

    /// Returns a pointer to the "may leave" flag for this instance specified
    /// for canonical lowering and lifting operations.
    #[inline]
    pub fn instance_flags(&self, instance: RuntimeComponentInstanceIndex) -> InstanceFlags {
        unsafe {
            let ptr = self
                .vmctx_plus_offset_raw::<VMGlobalDefinition>(self.offsets.instance_flags(instance));
            InstanceFlags(SendSyncPtr::new(ptr))
        }
    }

    /// Returns the runtime memory definition corresponding to the index of the
    /// memory provided.
    ///
    /// This can only be called after `idx` has been initialized at runtime
    /// during the instantiation process of a component.
    pub fn runtime_memory(&self, idx: RuntimeMemoryIndex) -> *mut VMMemoryDefinition {
        unsafe {
            let ret = *self.vmctx_plus_offset::<VmPtr<_>>(self.offsets.runtime_memory(idx));
            debug_assert!(ret.as_ptr() as usize != INVALID_PTR);
            ret.as_ptr()
        }
    }

    /// Returns the runtime table definition and associated instance `VMContext`
    /// corresponding to the index of the table provided.
    ///
    /// This can only be called after `idx` has been initialized at runtime
    /// during the instantiation process of a component.
    pub fn runtime_table(&self, idx: RuntimeTableIndex) -> VMTableImport {
        unsafe {
            let ret = *self.vmctx_plus_offset::<VMTableImport>(self.offsets.runtime_table(idx));
            debug_assert!(ret.from.as_ptr() as usize != INVALID_PTR);
            debug_assert!(ret.vmctx.as_ptr() as usize != INVALID_PTR);
            ret
        }
    }

    /// Returns the realloc pointer corresponding to the index provided.
    ///
    /// This can only be called after `idx` has been initialized at runtime
    /// during the instantiation process of a component.
    pub fn runtime_realloc(&self, idx: RuntimeReallocIndex) -> NonNull<VMFuncRef> {
        unsafe {
            let ret = *self.vmctx_plus_offset::<VmPtr<_>>(self.offsets.runtime_realloc(idx));
            debug_assert!(ret.as_ptr() as usize != INVALID_PTR);
            ret.as_non_null()
        }
    }

    /// Returns the async callback pointer corresponding to the index provided.
    ///
    /// This can only be called after `idx` has been initialized at runtime
    /// during the instantiation process of a component.
    pub fn runtime_callback(&self, idx: RuntimeCallbackIndex) -> NonNull<VMFuncRef> {
        unsafe {
            let ret = *self.vmctx_plus_offset::<VmPtr<_>>(self.offsets.runtime_callback(idx));
            debug_assert!(ret.as_ptr() as usize != INVALID_PTR);
            ret.as_non_null()
        }
    }

    /// Returns the post-return pointer corresponding to the index provided.
    ///
    /// This can only be called after `idx` has been initialized at runtime
    /// during the instantiation process of a component.
    pub fn runtime_post_return(&self, idx: RuntimePostReturnIndex) -> NonNull<VMFuncRef> {
        unsafe {
            let ret = *self.vmctx_plus_offset::<VmPtr<_>>(self.offsets.runtime_post_return(idx));
            debug_assert!(ret.as_ptr() as usize != INVALID_PTR);
            ret.as_non_null()
        }
    }

    /// Returns the host information for the lowered function at the index
    /// specified.
    ///
    /// This can only be called after `idx` has been initialized at runtime
    /// during the instantiation process of a component.
    pub fn lowering(&self, idx: LoweredIndex) -> VMLowering {
        unsafe {
            let ret = *self.vmctx_plus_offset::<VMLowering>(self.offsets.lowering(idx));
            debug_assert!(ret.callee.as_ptr() as usize != INVALID_PTR);
            debug_assert!(ret.data.as_ptr() as usize != INVALID_PTR);
            ret
        }
    }

    /// Returns the core wasm `funcref` corresponding to the trampoline
    /// specified.
    ///
    /// The returned function is suitable to pass directly to a wasm module
    /// instantiation and the function contains cranelift-compiled trampolines.
    ///
    /// This can only be called after `idx` has been initialized at runtime
    /// during the instantiation process of a component.
    pub fn trampoline_func_ref(&self, idx: TrampolineIndex) -> NonNull<VMFuncRef> {
        unsafe {
            let offset = self.offsets.trampoline_func_ref(idx);
            let ret = self.vmctx_plus_offset_raw::<VMFuncRef>(offset);
            debug_assert!(
                mem::transmute::<Option<VmPtr<VMWasmCallFunction>>, usize>(ret.as_ref().wasm_call)
                    != INVALID_PTR
            );
            debug_assert!(ret.as_ref().vmctx.as_ptr() as usize != INVALID_PTR);
            ret
        }
    }

    /// Stores the runtime memory pointer at the index specified.
    ///
    /// This is intended to be called during the instantiation process of a
    /// component once a memory is available, which may not be until part-way
    /// through component instantiation.
    ///
    /// Note that it should be a property of the component model that the `ptr`
    /// here is never needed prior to it being configured here in the instance.
    pub fn set_runtime_memory(
        self: Pin<&mut Self>,
        idx: RuntimeMemoryIndex,
        ptr: NonNull<VMMemoryDefinition>,
    ) {
        unsafe {
            let offset = self.offsets.runtime_memory(idx);
            let storage = self.vmctx_plus_offset_mut::<VmPtr<VMMemoryDefinition>>(offset);
            debug_assert!((*storage).as_ptr() as usize == INVALID_PTR);
            *storage = ptr.into();
        }
    }

    /// Same as `set_runtime_memory` but for realloc function pointers.
    pub fn set_runtime_realloc(
        self: Pin<&mut Self>,
        idx: RuntimeReallocIndex,
        ptr: NonNull<VMFuncRef>,
    ) {
        unsafe {
            let offset = self.offsets.runtime_realloc(idx);
            let storage = self.vmctx_plus_offset_mut::<VmPtr<VMFuncRef>>(offset);
            debug_assert!((*storage).as_ptr() as usize == INVALID_PTR);
            *storage = ptr.into();
        }
    }

    /// Same as `set_runtime_memory` but for async callback function pointers.
    pub fn set_runtime_callback(
        self: Pin<&mut Self>,
        idx: RuntimeCallbackIndex,
        ptr: NonNull<VMFuncRef>,
    ) {
        unsafe {
            let offset = self.offsets.runtime_callback(idx);
            let storage = self.vmctx_plus_offset_mut::<VmPtr<VMFuncRef>>(offset);
            debug_assert!((*storage).as_ptr() as usize == INVALID_PTR);
            *storage = ptr.into();
        }
    }

    /// Same as `set_runtime_memory` but for post-return function pointers.
    pub fn set_runtime_post_return(
        self: Pin<&mut Self>,
        idx: RuntimePostReturnIndex,
        ptr: NonNull<VMFuncRef>,
    ) {
        unsafe {
            let offset = self.offsets.runtime_post_return(idx);
            let storage = self.vmctx_plus_offset_mut::<VmPtr<VMFuncRef>>(offset);
            debug_assert!((*storage).as_ptr() as usize == INVALID_PTR);
            *storage = ptr.into();
        }
    }

    /// Stores the runtime table pointer at the index specified.
    ///
    /// This is intended to be called during the instantiation process of a
    /// component once a table is available, which may not be until part-way
    /// through component instantiation.
    ///
    /// Note that it should be a property of the component model that the `ptr`
    /// here is never needed prior to it being configured here in the instance.
    pub fn set_runtime_table(self: Pin<&mut Self>, idx: RuntimeTableIndex, import: VMTableImport) {
        unsafe {
            let offset = self.offsets.runtime_table(idx);
            let storage = self.vmctx_plus_offset_mut::<VMTableImport>(offset);
            debug_assert!((*storage).vmctx.as_ptr() as usize == INVALID_PTR);
            debug_assert!((*storage).from.as_ptr() as usize == INVALID_PTR);
            *storage = import;
        }
    }

    /// Configures host runtime lowering information associated with imported f
    /// functions for the `idx` specified.
    pub fn set_lowering(self: Pin<&mut Self>, idx: LoweredIndex, lowering: VMLowering) {
        unsafe {
            let callee = self.offsets.lowering_callee(idx);
            debug_assert!(*self.vmctx_plus_offset::<usize>(callee) == INVALID_PTR);
            let data = self.offsets.lowering_data(idx);
            debug_assert!(*self.vmctx_plus_offset::<usize>(data) == INVALID_PTR);
            let offset = self.offsets.lowering(idx);
            *self.vmctx_plus_offset_mut(offset) = lowering;
        }
    }

    /// Same as `set_lowering` but for the resource.drop functions.
    pub fn set_trampoline(
        self: Pin<&mut Self>,
        idx: TrampolineIndex,
        wasm_call: NonNull<VMWasmCallFunction>,
        array_call: NonNull<VMArrayCallFunction>,
        type_index: VMSharedTypeIndex,
    ) {
        unsafe {
            let offset = self.offsets.trampoline_func_ref(idx);
            debug_assert!(*self.vmctx_plus_offset::<usize>(offset) == INVALID_PTR);
            let vmctx = VMOpaqueContext::from_vmcomponent(self.vmctx());
            *self.vmctx_plus_offset_mut(offset) = VMFuncRef {
                wasm_call: Some(wasm_call.into()),
                array_call: array_call.into(),
                type_index,
                vmctx: vmctx.into(),
            };
        }
    }

    /// Configures the destructor for a resource at the `idx` specified.
    ///
    /// This is required to be called for each resource as it's defined within a
    /// component during the instantiation process.
    pub fn set_resource_destructor(
        self: Pin<&mut Self>,
        idx: ResourceIndex,
        dtor: Option<NonNull<VMFuncRef>>,
    ) {
        unsafe {
            let offset = self.offsets.resource_destructor(idx);
            debug_assert!(*self.vmctx_plus_offset::<usize>(offset) == INVALID_PTR);
            *self.vmctx_plus_offset_mut(offset) = dtor.map(VmPtr::from);
        }
    }

    /// Returns the destructor, if any, for `idx`.
    ///
    /// This is only valid to call after `set_resource_destructor`, or typically
    /// after instantiation.
    pub fn resource_destructor(&self, idx: ResourceIndex) -> Option<NonNull<VMFuncRef>> {
        unsafe {
            let offset = self.offsets.resource_destructor(idx);
            debug_assert!(*self.vmctx_plus_offset::<usize>(offset) != INVALID_PTR);
            (*self.vmctx_plus_offset::<Option<VmPtr<VMFuncRef>>>(offset)).map(|p| p.as_non_null())
        }
    }

    unsafe fn initialize_vmctx(mut self: Pin<&mut Self>) {
        let offset = self.offsets.magic();
        // SAFETY: it's safe to write the magic value during initialization and
        // this is also the right type of value to write.
        unsafe {
            *self.as_mut().vmctx_plus_offset_mut(offset) = VMCOMPONENT_MAGIC;
        }

        // Initialize the built-in functions
        //
        // SAFETY: it's safe to initialize the vmctx in this function and this
        // is also the right type of value to store in the vmctx.
        static BUILTINS: libcalls::VMComponentBuiltins = libcalls::VMComponentBuiltins::INIT;
        let ptr = BUILTINS.expose_provenance();
        let offset = self.offsets.builtins();
        unsafe {
            *self.as_mut().vmctx_plus_offset_mut(offset) = VmPtr::from(ptr);
        }

        // SAFETY: it's safe to initialize the vmctx in this function and this
        // is also the right type of value to store in the vmctx.
        let offset = self.offsets.vm_store_context();
        unsafe {
            *self.as_mut().vmctx_plus_offset_mut(offset) =
                VmPtr::from(self.store.0.as_ref().vm_store_context_ptr());
        }

        for i in 0..self.offsets.num_runtime_component_instances {
            let i = RuntimeComponentInstanceIndex::from_u32(i);
            let mut def = VMGlobalDefinition::new();
            // SAFETY: this is a valid initialization of all globals which are
            // 32-bit values.
            unsafe {
                *def.as_i32_mut() = FLAG_MAY_ENTER | FLAG_MAY_LEAVE;
                self.instance_flags(i).as_raw().write(def);
            }
        }

        // In debug mode set non-null bad values to all "pointer looking" bits
        // and pieces related to lowering and such. This'll help detect any
        // erroneous usage and enable debug assertions above as well to prevent
        // loading these before they're configured or setting them twice.
        //
        // SAFETY: it's valid to write a garbage pointer during initialization
        // when this is otherwise uninitialized memory
        if cfg!(debug_assertions) {
            for i in 0..self.offsets.num_lowerings {
                let i = LoweredIndex::from_u32(i);
                let offset = self.offsets.lowering_callee(i);
                // SAFETY: see above
                unsafe {
                    *self.as_mut().vmctx_plus_offset_mut(offset) = INVALID_PTR;
                }
                let offset = self.offsets.lowering_data(i);
                // SAFETY: see above
                unsafe {
                    *self.as_mut().vmctx_plus_offset_mut(offset) = INVALID_PTR;
                }
            }
            for i in 0..self.offsets.num_trampolines {
                let i = TrampolineIndex::from_u32(i);
                let offset = self.offsets.trampoline_func_ref(i);
                // SAFETY: see above
                unsafe {
                    *self.as_mut().vmctx_plus_offset_mut(offset) = INVALID_PTR;
                }
            }
            for i in 0..self.offsets.num_runtime_memories {
                let i = RuntimeMemoryIndex::from_u32(i);
                let offset = self.offsets.runtime_memory(i);
                // SAFETY: see above
                unsafe {
                    *self.as_mut().vmctx_plus_offset_mut(offset) = INVALID_PTR;
                }
            }
            for i in 0..self.offsets.num_runtime_reallocs {
                let i = RuntimeReallocIndex::from_u32(i);
                let offset = self.offsets.runtime_realloc(i);
                // SAFETY: see above
                unsafe {
                    *self.as_mut().vmctx_plus_offset_mut(offset) = INVALID_PTR;
                }
            }
            for i in 0..self.offsets.num_runtime_callbacks {
                let i = RuntimeCallbackIndex::from_u32(i);
                let offset = self.offsets.runtime_callback(i);
                // SAFETY: see above
                unsafe {
                    *self.as_mut().vmctx_plus_offset_mut(offset) = INVALID_PTR;
                }
            }
            for i in 0..self.offsets.num_runtime_post_returns {
                let i = RuntimePostReturnIndex::from_u32(i);
                let offset = self.offsets.runtime_post_return(i);
                // SAFETY: see above
                unsafe {
                    *self.as_mut().vmctx_plus_offset_mut(offset) = INVALID_PTR;
                }
            }
            for i in 0..self.offsets.num_resources {
                let i = ResourceIndex::from_u32(i);
                let offset = self.offsets.resource_destructor(i);
                // SAFETY: see above
                unsafe {
                    *self.as_mut().vmctx_plus_offset_mut(offset) = INVALID_PTR;
                }
            }
            for i in 0..self.offsets.num_runtime_tables {
                let i = RuntimeTableIndex::from_u32(i);
                let offset = self.offsets.runtime_table(i);
                // SAFETY: see above
                unsafe {
                    *self.as_mut().vmctx_plus_offset_mut(offset) = INVALID_PTR;
                }
            }
        }
    }

    /// Returns a reference to the component type information for this
    /// instance.
    pub fn component(&self) -> &Component {
        &self.component
    }

    /// Same as [`Self::component`] but additionally returns the
    /// `Pin<&mut Self>` with the same original lifetime.
    pub fn component_and_self(self: Pin<&mut Self>) -> (&Component, Pin<&mut Self>) {
        // SAFETY: this function is projecting both `&Component` and the same
        // pointer both connected to the same lifetime. This is safe because
        // it's a contract of `Pin<&mut Self>` that the `Component` field is
        // never written, meaning it's effectively unsafe to have `&mut
        // Component` projected from `Pin<&mut Self>`. Consequently it's safe to
        // have a read-only view of the field while still retaining mutable
        // access to all other fields.
        let component = unsafe { &*(&raw const self.component) };
        (component, self)
    }

    /// Returns a reference to the resource type information.
    pub fn resource_types(&self) -> &Arc<PrimaryMap<ResourceIndex, ResourceType>> {
        &self.resource_types
    }

    /// Returns a mutable reference to the resource type information.
    pub fn resource_types_mut(
        self: Pin<&mut Self>,
    ) -> &mut Arc<PrimaryMap<ResourceIndex, ResourceType>> {
        // SAFETY: we've chosen the `Pin` guarantee of `Self` to not apply to
        // the map returned.
        unsafe { &mut self.get_unchecked_mut().resource_types }
    }

    /// Returns whether the resource that `ty` points to is owned by the
    /// instance that `ty` correspond to.
    ///
    /// This is used when lowering borrows to skip table management and instead
    /// thread through the underlying representation directly.
    pub fn resource_owned_by_own_instance(&self, ty: TypeResourceTableIndex) -> bool {
        let resource = &self.component.types()[ty];
        let component = self.component.env_component();
        let idx = match component.defined_resource_index(resource.ty) {
            Some(idx) => idx,
            None => return false,
        };
        resource.instance == component.defined_resource_instances[idx]
    }

    /// Returns the runtime state of resources associated with this component.
    #[inline]
    pub fn guest_tables(
        self: Pin<&mut Self>,
    ) -> (
        &mut PrimaryMap<RuntimeComponentInstanceIndex, ResourceTable>,
        &ComponentTypes,
    ) {
        // safety: we've chosen the `pin` guarantee of `self` to not apply to
        // the map returned.
        unsafe {
            let me = self.get_unchecked_mut();
            (&mut me.instance_resource_tables, me.component.types())
        }
    }

    /// Returns the destructor and instance flags for the specified resource
    /// table type.
    ///
    /// This will lookup the origin definition of the `ty` table and return the
    /// destructor/flags for that.
    pub fn dtor_and_flags(
        &self,
        ty: TypeResourceTableIndex,
    ) -> (Option<NonNull<VMFuncRef>>, Option<InstanceFlags>) {
        let resource = self.component.types()[ty].ty;
        let dtor = self.resource_destructor(resource);
        let component = self.component.env_component();
        let flags = component.defined_resource_index(resource).map(|i| {
            let instance = component.defined_resource_instances[i];
            self.instance_flags(instance)
        });
        (dtor, flags)
    }

    /// Returns the store-local id that points to this component.
    pub fn id(&self) -> ComponentInstanceId {
        self.id
    }

    /// Pushes a new runtime instance that's been created into
    /// `self.instances`.
    pub fn push_instance_id(self: Pin<&mut Self>, id: InstanceId) -> RuntimeInstanceIndex {
        self.instances_mut().push(id)
    }

    /// Returns the [`InstanceId`] previously pushed by `push_instance_id`
    /// above.
    ///
    /// # Panics
    ///
    /// Panics if `idx` hasn't been initialized yet.
    pub fn instance(&self, idx: RuntimeInstanceIndex) -> InstanceId {
        self.instances[idx]
    }

    fn instances_mut(self: Pin<&mut Self>) -> &mut PrimaryMap<RuntimeInstanceIndex, InstanceId> {
        // SAFETY: we've chosen the `Pin` guarantee of `Self` to not apply to
        // the map returned.
        unsafe { &mut self.get_unchecked_mut().instances }
    }

    /// Looks up the value used for `import` at runtime.
    ///
    /// # Panics
    ///
    /// Panics of `import` is out of bounds for this component.
    pub(crate) fn runtime_import(&self, import: RuntimeImportIndex) -> &RuntimeImport {
        &self.imports[import]
    }

    /// Returns an `InstancePre<T>` which can be used to re-instantiated this
    /// component if desired.
    ///
    /// # Safety
    ///
    /// This function places no bounds on `T` so it's up to the caller to match
    /// that up appropriately with the store that this instance resides within.
    pub unsafe fn instance_pre<T>(&self) -> InstancePre<T> {
        // SAFETY: The `T` part of `new_unchecked` is forwarded as a contract of
        // this function, and otherwise the validity of the components of the
        // InstancePre should be guaranteed as it's what we were built with
        // ourselves.
        unsafe {
            InstancePre::new_unchecked(
                self.component.clone(),
                self.imports.clone(),
                self.resource_types.clone(),
            )
        }
    }

    /// Sets the cached argument for the canonical ABI option `post-return` to
    /// the `arg` specified.
    ///
    /// This function is used in conjunction with function calls to record,
    /// after a function call completes, the optional ABI return value. This
    /// return value is cached within this instance for future use when the
    /// `post_return` Rust-API-level function is invoked.
    ///
    /// Note that `index` here is the index of the export that was just
    /// invoked, and this is used to ensure that `post_return` is called on the
    /// same function afterwards. This restriction technically isn't necessary
    /// though and may be one we want to lift in the future.
    ///
    /// # Panics
    ///
    /// This function will panic if `post_return_arg` is already set to `Some`.
    pub fn post_return_arg_set(self: Pin<&mut Self>, index: ExportIndex, arg: ValRaw) {
        assert!(self.post_return_arg.is_none());
        *self.post_return_arg_mut() = Some((index, arg));
    }

    /// Re-acquires the value originally saved via `post_return_arg_set`.
    ///
    /// This function will take a function `index` that's having its
    /// `post_return` function called. If an argument was previously stored and
    /// `index` matches the index that was stored then `Some(arg)` is returned.
    /// Otherwise `None` is returned.
    pub fn post_return_arg_take(self: Pin<&mut Self>, index: ExportIndex) -> Option<ValRaw> {
        let post_return_arg = self.post_return_arg_mut();
        let (expected_index, arg) = post_return_arg.take()?;
        if index != expected_index {
            *post_return_arg = Some((expected_index, arg));
            None
        } else {
            Some(arg)
        }
    }

    fn post_return_arg_mut(self: Pin<&mut Self>) -> &mut Option<(ExportIndex, ValRaw)> {
        // SAFETY: we've chosen the `Pin` guarantee of `Self` to not apply to
        // the map returned.
        unsafe { &mut self.get_unchecked_mut().post_return_arg }
    }

    #[cfg(feature = "component-model-async")]
    pub(crate) fn concurrent_state_mut(self: Pin<&mut Self>) -> &mut concurrent::ConcurrentState {
        // SAFETY: we've chosen the `Pin` guarantee of `Self` to not apply to
        // the map returned.
        unsafe { &mut self.get_unchecked_mut().concurrent_state }
    }
}

// SAFETY: `layout` should describe this accurately and `OwnedVMContext` is the
// last field of `ComponentInstance`.
unsafe impl InstanceLayout for ComponentInstance {
    /// Technically it is not required to `alloc_zeroed` here. The primary
    /// reason for doing this is because a component context start is a "partly
    /// initialized" state where pointers and such are configured as the
    /// instantiation process continues. The component model should guarantee
    /// that we never access uninitialized memory in the context, but to help
    /// protect against possible bugs a zeroed allocation is done here to try to
    /// contain use-before-initialized issues.
    const INIT_ZEROED: bool = true;

    type VMContext = VMComponentContext;

    fn layout(&self) -> Layout {
        ComponentInstance::alloc_layout(&self.offsets)
    }

    fn owned_vmctx(&self) -> &OwnedVMContext<VMComponentContext> {
        &self.vmctx
    }

    fn owned_vmctx_mut(&mut self) -> &mut OwnedVMContext<VMComponentContext> {
        &mut self.vmctx
    }
}

pub type OwnedComponentInstance = OwnedInstance<ComponentInstance>;

impl VMComponentContext {
    /// Moves the `self` pointer backwards to the `ComponentInstance` pointer
    /// that this `VMComponentContext` trails.
    pub fn instance(&self) -> *mut ComponentInstance {
        unsafe {
            (self as *const Self as *mut u8)
                .offset(-(offset_of!(ComponentInstance, vmctx) as isize))
                as *mut ComponentInstance
        }
    }

    /// Helper function to cast between context types using a debug assertion to
    /// protect against some mistakes.
    ///
    /// # Safety
    ///
    /// The `opaque` value must be a valid pointer where it's safe to read its
    /// "magic" value.
    #[inline]
    pub unsafe fn from_opaque(opaque: NonNull<VMOpaqueContext>) -> NonNull<VMComponentContext> {
        // See comments in `VMContext::from_opaque` for this debug assert
        //
        // SAFETY: it's a contract of this function that it's safe to read
        // `opaque`.
        unsafe {
            debug_assert_eq!(opaque.as_ref().magic, VMCOMPONENT_MAGIC);
        }
        opaque.cast()
    }
}

impl VMOpaqueContext {
    /// Helper function to clearly indicate the cast desired
    #[inline]
    pub fn from_vmcomponent(ptr: NonNull<VMComponentContext>) -> NonNull<VMOpaqueContext> {
        ptr.cast()
    }
}

#[repr(transparent)]
#[derive(Copy, Clone)]
pub struct InstanceFlags(SendSyncPtr<VMGlobalDefinition>);

impl InstanceFlags {
    /// Wraps the given pointer as an `InstanceFlags`
    ///
    /// # Unsafety
    ///
    /// This is a raw pointer argument which needs to be valid for the lifetime
    /// that `InstanceFlags` is used.
    pub unsafe fn from_raw(ptr: NonNull<VMGlobalDefinition>) -> InstanceFlags {
        InstanceFlags(SendSyncPtr::from(ptr))
    }

    #[inline]
    pub unsafe fn may_leave(&self) -> bool {
        unsafe { *self.as_raw().as_ref().as_i32() & FLAG_MAY_LEAVE != 0 }
    }

    #[inline]
    pub unsafe fn set_may_leave(&mut self, val: bool) {
        unsafe {
            if val {
                *self.as_raw().as_mut().as_i32_mut() |= FLAG_MAY_LEAVE;
            } else {
                *self.as_raw().as_mut().as_i32_mut() &= !FLAG_MAY_LEAVE;
            }
        }
    }

    #[inline]
    pub unsafe fn may_enter(&self) -> bool {
        unsafe { *self.as_raw().as_ref().as_i32() & FLAG_MAY_ENTER != 0 }
    }

    #[inline]
    pub unsafe fn set_may_enter(&mut self, val: bool) {
        unsafe {
            if val {
                *self.as_raw().as_mut().as_i32_mut() |= FLAG_MAY_ENTER;
            } else {
                *self.as_raw().as_mut().as_i32_mut() &= !FLAG_MAY_ENTER;
            }
        }
    }

    #[inline]
    pub unsafe fn needs_post_return(&self) -> bool {
        unsafe { *self.as_raw().as_ref().as_i32() & FLAG_NEEDS_POST_RETURN != 0 }
    }

    #[inline]
    pub unsafe fn set_needs_post_return(&mut self, val: bool) {
        unsafe {
            if val {
                *self.as_raw().as_mut().as_i32_mut() |= FLAG_NEEDS_POST_RETURN;
            } else {
                *self.as_raw().as_mut().as_i32_mut() &= !FLAG_NEEDS_POST_RETURN;
            }
        }
    }

    #[inline]
    pub fn as_raw(&self) -> NonNull<VMGlobalDefinition> {
        self.0.as_non_null()
    }
}
