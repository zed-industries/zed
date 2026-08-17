//! Wasmtime's "store" type
//!
//! This module, and its submodules, contain the `Store` type and various types
//! used to interact with it. At first glance this is a pretty confusing module
//! where you need to know the difference between:
//!
//! * `Store<T>`
//! * `StoreContext<T>`
//! * `StoreContextMut<T>`
//! * `AsContext`
//! * `AsContextMut`
//! * `StoreInner<T>`
//! * `StoreOpaque`
//! * `StoreData`
//!
//! There's... quite a lot going on here, and it's easy to be confused. This
//! comment is ideally going to serve the purpose of clarifying what all these
//! types are for and why they're motivated.
//!
//! First it's important to know what's "internal" and what's "external". Almost
//! everything above is defined as `pub`, but only some of the items are
//! reexported to the outside world to be usable from this crate. Otherwise all
//! items are `pub` within this `store` module, and the `store` module is
//! private to the `wasmtime` crate. Notably `Store<T>`, `StoreContext<T>`,
//! `StoreContextMut<T>`, `AsContext`, and `AsContextMut` are all public
//! interfaces to the `wasmtime` crate. You can think of these as:
//!
//! * `Store<T>` - an owned reference to a store, the "root of everything"
//! * `StoreContext<T>` - basically `&StoreInner<T>`
//! * `StoreContextMut<T>` - more-or-less `&mut StoreInner<T>` with caveats.
//!   Explained later.
//! * `AsContext` - similar to `AsRef`, but produces `StoreContext<T>`
//! * `AsContextMut` - similar to `AsMut`, but produces `StoreContextMut<T>`
//!
//! Next comes the internal structure of the `Store<T>` itself. This looks like:
//!
//! * `Store<T>` - this type is just a pointer large. It's primarily just
//!   intended to be consumed by the outside world. Note that the "just a
//!   pointer large" is a load-bearing implementation detail in Wasmtime. This
//!   enables it to store a pointer to its own trait object which doesn't need
//!   to change over time.
//!
//! * `StoreInner<T>` - the first layer of the contents of a `Store<T>`, what's
//!   stored inside the `Box`. This is the general Rust pattern when one struct
//!   is a layer over another. The surprising part, though, is that this is
//!   further subdivided. This structure only contains things which actually
//!   need `T` itself. The downside of this structure is that it's always
//!   generic and means that code is monomorphized into consumer crates. We
//!   strive to have things be as monomorphic as possible in `wasmtime` so this
//!   type is not heavily used.
//!
//! * `StoreOpaque` - this is the primary contents of the `StoreInner<T>` type.
//!   Stored inline in the outer type the "opaque" here means that it's a
//!   "store" but it doesn't have access to the `T`. This is the primary
//!   "internal" reference that Wasmtime uses since `T` is rarely needed by the
//!   internals of Wasmtime.
//!
//! * `StoreData` - this is a final helper struct stored within `StoreOpaque`.
//!   All references of Wasm items into a `Store` are actually indices into a
//!   table in this structure, and the `StoreData` being separate makes it a bit
//!   easier to manage/define/work with. There's no real fundamental reason this
//!   is split out, although sometimes it's useful to have separate borrows into
//!   these tables than the `StoreOpaque`.
//!
//! A major caveat with these representations is that the internal `&mut
//! StoreInner<T>` is never handed out publicly to consumers of this crate, only
//! through a wrapper of `StoreContextMut<'_, T>`. The reason for this is that
//! we want to provide mutable, but not destructive, access to the contents of a
//! `Store`. For example if a `StoreInner<T>` were replaced with some other
//! `StoreInner<T>` then that would drop live instances, possibly those
//! currently executing beneath the current stack frame. This would not be a
//! safe operation.
//!
//! This means, though, that the `wasmtime` crate, which liberally uses `&mut
//! StoreOpaque` internally, has to be careful to never actually destroy the
//! contents of `StoreOpaque`. This is an invariant that we, as the authors of
//! `wasmtime`, must uphold for the public interface to be safe.

use crate::RootSet;
#[cfg(feature = "component-model-async")]
use crate::component::ComponentStoreData;
#[cfg(feature = "component-model-async")]
use crate::component::concurrent;
#[cfg(feature = "async")]
use crate::fiber;
use crate::module::RegisteredModuleId;
use crate::prelude::*;
#[cfg(feature = "gc")]
use crate::runtime::vm::GcRootsList;
#[cfg(feature = "stack-switching")]
use crate::runtime::vm::VMContRef;
use crate::runtime::vm::mpk::ProtectionKey;
use crate::runtime::vm::{
    self, GcStore, Imports, InstanceAllocationRequest, InstanceAllocator, InstanceHandle,
    Interpreter, InterpreterRef, ModuleRuntimeInfo, OnDemandInstanceAllocator, SendSyncPtr,
    SignalHandler, StoreBox, StorePtr, Unwind, VMContext, VMFuncRef, VMGcRef, VMStoreContext,
};
use crate::trampoline::VMHostGlobalContext;
use crate::{Engine, Module, Trap, Val, ValRaw, module::ModuleRegistry};
use crate::{Global, Instance, Memory, Table, Uninhabited};
use alloc::sync::Arc;
use core::fmt;
use core::marker;
use core::mem::{self, ManuallyDrop};
use core::num::NonZeroU64;
use core::ops::{Deref, DerefMut};
use core::pin::Pin;
use core::ptr::NonNull;
use wasmtime_environ::{DefinedGlobalIndex, DefinedTableIndex, EntityRef, PrimaryMap, TripleExt};

mod context;
pub use self::context::*;
mod data;
pub use self::data::*;
mod func_refs;
use func_refs::FuncRefs;
#[cfg(feature = "async")]
mod token;
#[cfg(feature = "async")]
pub(crate) use token::StoreToken;
#[cfg(feature = "async")]
mod async_;
#[cfg(all(feature = "async", feature = "call-hook"))]
pub use self::async_::CallHookHandler;
#[cfg(feature = "gc")]
mod gc;

/// A [`Store`] is a collection of WebAssembly instances and host-defined state.
///
/// All WebAssembly instances and items will be attached to and refer to a
/// [`Store`]. For example instances, functions, globals, and tables are all
/// attached to a [`Store`]. Instances are created by instantiating a
/// [`Module`](crate::Module) within a [`Store`].
///
/// A [`Store`] is intended to be a short-lived object in a program. No form
/// of GC is implemented at this time so once an instance is created within a
/// [`Store`] it will not be deallocated until the [`Store`] itself is dropped.
/// This makes [`Store`] unsuitable for creating an unbounded number of
/// instances in it because [`Store`] will never release this memory. It's
/// recommended to have a [`Store`] correspond roughly to the lifetime of a
/// "main instance" that an embedding is interested in executing.
///
/// ## Type parameter `T`
///
/// Each [`Store`] has a type parameter `T` associated with it. This `T`
/// represents state defined by the host. This state will be accessible through
/// the [`Caller`](crate::Caller) type that host-defined functions get access
/// to. This `T` is suitable for storing `Store`-specific information which
/// imported functions may want access to.
///
/// The data `T` can be accessed through methods like [`Store::data`] and
/// [`Store::data_mut`].
///
/// ## Stores, contexts, oh my
///
/// Most methods in Wasmtime take something of the form
/// [`AsContext`](crate::AsContext) or [`AsContextMut`](crate::AsContextMut) as
/// the first argument. These two traits allow ergonomically passing in the
/// context you currently have to any method. The primary two sources of
/// contexts are:
///
/// * `Store<T>`
/// * `Caller<'_, T>`
///
/// corresponding to what you create and what you have access to in a host
/// function. You can also explicitly acquire a [`StoreContext`] or
/// [`StoreContextMut`] and pass that around as well.
///
/// Note that all methods on [`Store`] are mirrored onto [`StoreContext`],
/// [`StoreContextMut`], and [`Caller`](crate::Caller). This way no matter what
/// form of context you have you can call various methods, create objects, etc.
///
/// ## Stores and `Default`
///
/// You can create a store with default configuration settings using
/// `Store::default()`. This will create a brand new [`Engine`] with default
/// configuration (see [`Config`](crate::Config) for more information).
///
/// ## Cross-store usage of items
///
/// In `wasmtime` wasm items such as [`Global`] and [`Memory`] "belong" to a
/// [`Store`]. The store they belong to is the one they were created with
/// (passed in as a parameter) or instantiated with. This store is the only
/// store that can be used to interact with wasm items after they're created.
///
/// The `wasmtime` crate will panic if the [`Store`] argument passed in to these
/// operations is incorrect. In other words it's considered a programmer error
/// rather than a recoverable error for the wrong [`Store`] to be used when
/// calling APIs.
pub struct Store<T: 'static> {
    // for comments about `ManuallyDrop`, see `Store::into_data`
    inner: ManuallyDrop<Box<StoreInner<T>>>,
}

#[derive(Copy, Clone, Debug)]
/// Passed to the argument of [`Store::call_hook`] to indicate a state transition in
/// the WebAssembly VM.
pub enum CallHook {
    /// Indicates the VM is calling a WebAssembly function, from the host.
    CallingWasm,
    /// Indicates the VM is returning from a WebAssembly function, to the host.
    ReturningFromWasm,
    /// Indicates the VM is calling a host function, from WebAssembly.
    CallingHost,
    /// Indicates the VM is returning from a host function, to WebAssembly.
    ReturningFromHost,
}

impl CallHook {
    /// Indicates the VM is entering host code (exiting WebAssembly code)
    pub fn entering_host(&self) -> bool {
        match self {
            CallHook::ReturningFromWasm | CallHook::CallingHost => true,
            _ => false,
        }
    }
    /// Indicates the VM is exiting host code (entering WebAssembly code)
    pub fn exiting_host(&self) -> bool {
        match self {
            CallHook::ReturningFromHost | CallHook::CallingWasm => true,
            _ => false,
        }
    }
}

/// Internal contents of a `Store<T>` that live on the heap.
///
/// The members of this struct are those that need to be generic over `T`, the
/// store's internal type storage. Otherwise all things that don't rely on `T`
/// should go into `StoreOpaque`.
pub struct StoreInner<T: 'static> {
    /// Generic metadata about the store that doesn't need access to `T`.
    inner: StoreOpaque,

    limiter: Option<ResourceLimiterInner<T>>,
    call_hook: Option<CallHookInner<T>>,
    #[cfg(target_has_atomic = "64")]
    epoch_deadline_behavior:
        Option<Box<dyn FnMut(StoreContextMut<T>) -> Result<UpdateDeadline> + Send + Sync>>,
    // for comments about `ManuallyDrop`, see `Store::into_data`
    data: ManuallyDrop<T>,
}

enum ResourceLimiterInner<T> {
    Sync(Box<dyn (FnMut(&mut T) -> &mut dyn crate::ResourceLimiter) + Send + Sync>),
    #[cfg(feature = "async")]
    Async(Box<dyn (FnMut(&mut T) -> &mut dyn crate::ResourceLimiterAsync) + Send + Sync>),
}

enum CallHookInner<T: 'static> {
    #[cfg(feature = "call-hook")]
    Sync(Box<dyn FnMut(StoreContextMut<'_, T>, CallHook) -> Result<()> + Send + Sync>),
    #[cfg(all(feature = "async", feature = "call-hook"))]
    Async(Box<dyn CallHookHandler<T> + Send + Sync>),
    #[expect(
        dead_code,
        reason = "forcing, regardless of cfg, the type param to be used"
    )]
    ForceTypeParameterToBeUsed {
        uninhabited: Uninhabited,
        _marker: marker::PhantomData<T>,
    },
}

/// What to do after returning from a callback when the engine epoch reaches
/// the deadline for a Store during execution of a function using that store.
#[non_exhaustive]
pub enum UpdateDeadline {
    /// Extend the deadline by the specified number of ticks.
    Continue(u64),
    /// Extend the deadline by the specified number of ticks after yielding to
    /// the async executor loop. This can only be used with an async [`Store`]
    /// configured via [`Config::async_support`](crate::Config::async_support).
    #[cfg(feature = "async")]
    Yield(u64),
    /// Extend the deadline by the specified number of ticks after yielding to
    /// the async executor loop. This can only be used with an async [`Store`]
    /// configured via [`Config::async_support`](crate::Config::async_support).
    ///
    /// The yield will be performed by the future provided; when using `tokio`
    /// it is recommended to provide [`tokio::task::yield_now`](https://docs.rs/tokio/latest/tokio/task/fn.yield_now.html)
    /// here.
    #[cfg(feature = "async")]
    YieldCustom(
        u64,
        ::core::pin::Pin<Box<dyn ::core::future::Future<Output = ()> + Send>>,
    ),
}

// Forward methods on `StoreOpaque` to also being on `StoreInner<T>`
impl<T> Deref for StoreInner<T> {
    type Target = StoreOpaque;
    fn deref(&self) -> &Self::Target {
        &self.inner
    }
}

impl<T> DerefMut for StoreInner<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.inner
    }
}

/// Monomorphic storage for a `Store<T>`.
///
/// This structure contains the bulk of the metadata about a `Store`. This is
/// used internally in Wasmtime when dependence on the `T` of `Store<T>` isn't
/// necessary, allowing code to be monomorphic and compiled into the `wasmtime`
/// crate itself.
pub struct StoreOpaque {
    // This `StoreOpaque` structure has references to itself. These aren't
    // immediately evident, however, so we need to tell the compiler that it
    // contains self-references. This notably suppresses `noalias` annotations
    // when this shows up in compiled code because types of this structure do
    // indeed alias itself. An example of this is `default_callee` holds a
    // `*mut dyn Store` to the address of this `StoreOpaque` itself, indeed
    // aliasing!
    //
    // It's somewhat unclear to me at this time if this is 100% sufficient to
    // get all the right codegen in all the right places. For example does
    // `Store` need to internally contain a `Pin<Box<StoreInner<T>>>`? Do the
    // contexts need to contain `Pin<&mut StoreInner<T>>`? I'm not familiar
    // enough with `Pin` to understand if it's appropriate here (we do, for
    // example want to allow movement in and out of `data: T`, just not movement
    // of most of the other members). It's also not clear if using `Pin` in a
    // few places buys us much other than a bunch of `unsafe` that we already
    // sort of hand-wave away.
    //
    // In any case this seems like a good mid-ground for now where we're at
    // least telling the compiler something about all the aliasing happening
    // within a `Store`.
    _marker: marker::PhantomPinned,

    engine: Engine,
    vm_store_context: VMStoreContext,

    // Contains all continuations ever allocated throughout the lifetime of this
    // store.
    #[cfg(feature = "stack-switching")]
    continuations: Vec<Box<VMContRef>>,

    instances: PrimaryMap<InstanceId, StoreInstance>,

    #[cfg(feature = "component-model")]
    num_component_instances: usize,
    signal_handler: Option<SignalHandler>,
    modules: ModuleRegistry,
    func_refs: FuncRefs,
    host_globals: PrimaryMap<DefinedGlobalIndex, StoreBox<VMHostGlobalContext>>,
    // GC-related fields.
    gc_store: Option<GcStore>,
    gc_roots: RootSet,
    #[cfg(feature = "gc")]
    gc_roots_list: GcRootsList,
    // Types for which the embedder has created an allocator for.
    #[cfg(feature = "gc")]
    gc_host_alloc_types: crate::hash_set::HashSet<crate::type_registry::RegisteredType>,

    // Numbers of resources instantiated in this store, and their limits
    instance_count: usize,
    instance_limit: usize,
    memory_count: usize,
    memory_limit: usize,
    table_count: usize,
    table_limit: usize,
    #[cfg(feature = "async")]
    async_state: fiber::AsyncState,

    // If fuel_yield_interval is enabled, then we store the remaining fuel (that isn't in
    // runtime_limits) here. The total amount of fuel is the runtime limits and reserve added
    // together. Then when we run out of gas, we inject the yield amount from the reserve
    // until the reserve is empty.
    fuel_reserve: u64,
    fuel_yield_interval: Option<NonZeroU64>,
    /// Indexed data within this `Store`, used to store information about
    /// globals, functions, memories, etc.
    store_data: StoreData,
    traitobj: StorePtr,
    default_caller_vmctx: SendSyncPtr<VMContext>,

    /// Used to optimized wasm->host calls when the host function is defined with
    /// `Func::new` to avoid allocating a new vector each time a function is
    /// called.
    hostcall_val_storage: Vec<Val>,
    /// Same as `hostcall_val_storage`, but for the direction of the host
    /// calling wasm.
    wasm_val_raw_storage: Vec<ValRaw>,

    /// Keep track of what protection key is being used during allocation so
    /// that the right memory pages can be enabled when entering WebAssembly
    /// guest code.
    pkey: Option<ProtectionKey>,

    /// Runtime state for components used in the handling of resources, borrow,
    /// and calls. These also interact with the `ResourceAny` type and its
    /// internal representation.
    #[cfg(feature = "component-model")]
    component_host_table: vm::component::ResourceTable,
    #[cfg(feature = "component-model")]
    component_calls: vm::component::CallContexts,
    #[cfg(feature = "component-model")]
    host_resource_data: crate::component::HostResourceData,

    #[cfg(feature = "component-model-async")]
    concurrent_async_state: concurrent::AsyncState,

    /// State related to the executor of wasm code.
    ///
    /// For example if Pulley is enabled and configured then this will store a
    /// Pulley interpreter.
    executor: Executor,
}

/// Executor state within `StoreOpaque`.
///
/// Effectively stores Pulley interpreter state and handles conditional support
/// for Cranelift at compile time.
pub(crate) enum Executor {
    Interpreter(Interpreter),
    #[cfg(has_host_compiler_backend)]
    Native,
}

impl Executor {
    pub(crate) fn new(engine: &Engine) -> Self {
        #[cfg(has_host_compiler_backend)]
        if cfg!(feature = "pulley") && engine.target().is_pulley() {
            Executor::Interpreter(Interpreter::new(engine))
        } else {
            Executor::Native
        }
        #[cfg(not(has_host_compiler_backend))]
        {
            debug_assert!(engine.target().is_pulley());
            Executor::Interpreter(Interpreter::new(engine))
        }
    }
}

/// A borrowed reference to `Executor` above.
pub(crate) enum ExecutorRef<'a> {
    Interpreter(InterpreterRef<'a>),
    #[cfg(has_host_compiler_backend)]
    Native,
}

/// An RAII type to automatically mark a region of code as unsafe for GC.
#[doc(hidden)]
pub struct AutoAssertNoGc<'a> {
    store: &'a mut StoreOpaque,
    entered: bool,
}

impl<'a> AutoAssertNoGc<'a> {
    #[inline]
    pub fn new(store: &'a mut StoreOpaque) -> Self {
        let entered = if !cfg!(feature = "gc") {
            false
        } else if let Some(gc_store) = store.gc_store.as_mut() {
            gc_store.gc_heap.enter_no_gc_scope();
            true
        } else {
            false
        };

        AutoAssertNoGc { store, entered }
    }

    /// Creates an `AutoAssertNoGc` value which is forcibly "not entered" and
    /// disables checks for no GC happening for the duration of this value.
    ///
    /// This is used when it is statically otherwise known that a GC doesn't
    /// happen for the various types involved.
    ///
    /// # Unsafety
    ///
    /// This method is `unsafe` as it does not provide the same safety
    /// guarantees as `AutoAssertNoGc::new`. It must be guaranteed by the
    /// caller that a GC doesn't happen.
    #[inline]
    pub unsafe fn disabled(store: &'a mut StoreOpaque) -> Self {
        if cfg!(debug_assertions) {
            AutoAssertNoGc::new(store)
        } else {
            AutoAssertNoGc {
                store,
                entered: false,
            }
        }
    }
}

impl core::ops::Deref for AutoAssertNoGc<'_> {
    type Target = StoreOpaque;

    #[inline]
    fn deref(&self) -> &Self::Target {
        &*self.store
    }
}

impl core::ops::DerefMut for AutoAssertNoGc<'_> {
    #[inline]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut *self.store
    }
}

impl Drop for AutoAssertNoGc<'_> {
    #[inline]
    fn drop(&mut self) {
        if self.entered {
            self.store.unwrap_gc_store_mut().gc_heap.exit_no_gc_scope();
        }
    }
}

/// Used to associate instances with the store.
///
/// This is needed to track if the instance was allocated explicitly with the on-demand
/// instance allocator.
struct StoreInstance {
    handle: InstanceHandle,
    kind: StoreInstanceKind,
}

enum StoreInstanceKind {
    /// An actual, non-dummy instance.
    Real {
        /// The id of this instance's module inside our owning store's
        /// `ModuleRegistry`.
        module_id: RegisteredModuleId,
    },

    /// This is a dummy instance that is just an implementation detail for
    /// something else. For example, host-created memories internally create a
    /// dummy instance.
    ///
    /// Regardless of the configured instance allocator for the engine, dummy
    /// instances always use the on-demand allocator to deallocate the instance.
    Dummy,
}

impl<T> Store<T> {
    /// Creates a new [`Store`] to be associated with the given [`Engine`] and
    /// `data` provided.
    ///
    /// The created [`Store`] will place no additional limits on the size of
    /// linear memories or tables at runtime. Linear memories and tables will
    /// be allowed to grow to any upper limit specified in their definitions.
    /// The store will limit the number of instances, linear memories, and
    /// tables created to 10,000. This can be overridden with the
    /// [`Store::limiter`] configuration method.
    pub fn new(engine: &Engine, data: T) -> Self {
        let store_data = StoreData::new();
        log::trace!("creating new store {:?}", store_data.id());

        let pkey = engine.allocator().next_available_pkey();

        let inner = StoreOpaque {
            _marker: marker::PhantomPinned,
            engine: engine.clone(),
            vm_store_context: Default::default(),
            #[cfg(feature = "stack-switching")]
            continuations: Vec::new(),
            instances: PrimaryMap::new(),
            #[cfg(feature = "component-model")]
            num_component_instances: 0,
            signal_handler: None,
            gc_store: None,
            gc_roots: RootSet::default(),
            #[cfg(feature = "gc")]
            gc_roots_list: GcRootsList::default(),
            #[cfg(feature = "gc")]
            gc_host_alloc_types: Default::default(),
            modules: ModuleRegistry::default(),
            func_refs: FuncRefs::default(),
            host_globals: PrimaryMap::new(),
            instance_count: 0,
            instance_limit: crate::DEFAULT_INSTANCE_LIMIT,
            memory_count: 0,
            memory_limit: crate::DEFAULT_MEMORY_LIMIT,
            table_count: 0,
            table_limit: crate::DEFAULT_TABLE_LIMIT,
            #[cfg(feature = "async")]
            async_state: Default::default(),
            fuel_reserve: 0,
            fuel_yield_interval: None,
            store_data,
            traitobj: StorePtr::empty(),
            default_caller_vmctx: SendSyncPtr::new(NonNull::dangling()),
            hostcall_val_storage: Vec::new(),
            wasm_val_raw_storage: Vec::new(),
            pkey,
            #[cfg(feature = "component-model")]
            component_host_table: Default::default(),
            #[cfg(feature = "component-model")]
            component_calls: Default::default(),
            #[cfg(feature = "component-model")]
            host_resource_data: Default::default(),
            executor: Executor::new(engine),
            #[cfg(feature = "component-model-async")]
            concurrent_async_state: Default::default(),
        };
        let mut inner = Box::new(StoreInner {
            inner,
            limiter: None,
            call_hook: None,
            #[cfg(target_has_atomic = "64")]
            epoch_deadline_behavior: None,
            data: ManuallyDrop::new(data),
        });

        inner.traitobj = StorePtr::new(NonNull::from(&mut *inner));

        // Wasmtime uses the callee argument to host functions to learn about
        // the original pointer to the `Store` itself, allowing it to
        // reconstruct a `StoreContextMut<T>`. When we initially call a `Func`,
        // however, there's no "callee" to provide. To fix this we allocate a
        // single "default callee" for the entire `Store`. This is then used as
        // part of `Func::call` to guarantee that the `callee: *mut VMContext`
        // is never null.
        let module = Arc::new(wasmtime_environ::Module::default());
        let shim = ModuleRuntimeInfo::bare(module);
        let allocator = OnDemandInstanceAllocator::default();

        allocator
            .validate_module(shim.env_module(), shim.offsets())
            .unwrap();

        unsafe {
            let id = inner
                .allocate_instance(
                    AllocateInstanceKind::Dummy {
                        allocator: &allocator,
                    },
                    &shim,
                    Default::default(),
                )
                .expect("failed to allocate default callee");
            let default_caller_vmctx = inner.instance(id).vmctx();
            inner.default_caller_vmctx = default_caller_vmctx.into();
        }

        Self {
            inner: ManuallyDrop::new(inner),
        }
    }

    /// Access the underlying data owned by this `Store`.
    #[inline]
    pub fn data(&self) -> &T {
        self.inner.data()
    }

    /// Access the underlying data owned by this `Store`.
    #[inline]
    pub fn data_mut(&mut self) -> &mut T {
        self.inner.data_mut()
    }

    fn run_manual_drop_routines(&mut self) {
        // We need to drop the fibers of each component instance before
        // attempting to drop the instances themselves since the fibers may need
        // to be resumed and allowed to exit cleanly before we yank the state
        // out from under them.
        //
        // This will also drop any futures which might use a `&Accessor` fields
        // in their `Drop::drop` implementations, in which case they'll need to
        // be called from with in the context of a `tls::set` closure.
        #[cfg(feature = "component-model-async")]
        ComponentStoreData::drop_fibers_and_futures(&mut self.inner);

        // Ensure all fiber stacks, even cached ones, are all flushed out to the
        // instance allocator.
        self.inner.flush_fiber_stack();
    }

    /// Consumes this [`Store`], destroying it, and returns the underlying data.
    pub fn into_data(mut self) -> T {
        self.run_manual_drop_routines();

        // This is an unsafe operation because we want to avoid having a runtime
        // check or boolean for whether the data is actually contained within a
        // `Store`. The data itself is stored as `ManuallyDrop` since we're
        // manually managing the memory here, and there's also a `ManuallyDrop`
        // around the `Box<StoreInner<T>>`. The way this works though is a bit
        // tricky, so here's how things get dropped appropriately:
        //
        // * When a `Store<T>` is normally dropped, the custom destructor for
        //   `Store<T>` will drop `T`, then the `self.inner` field. The
        //   rustc-glue destructor runs for `Box<StoreInner<T>>` which drops
        //   `StoreInner<T>`. This cleans up all internal fields and doesn't
        //   touch `T` because it's wrapped in `ManuallyDrop`.
        //
        // * When calling this method we skip the top-level destructor for
        //   `Store<T>` with `mem::forget`. This skips both the destructor for
        //   `T` and the destructor for `StoreInner<T>`. We do, however, run the
        //   destructor for `Box<StoreInner<T>>` which, like above, will skip
        //   the destructor for `T` since it's `ManuallyDrop`.
        //
        // In both cases all the other fields of `StoreInner<T>` should all get
        // dropped, and the manual management of destructors is basically
        // between this method and `Drop for Store<T>`. Note that this also
        // means that `Drop for StoreInner<T>` cannot access `self.data`, so
        // there is a comment indicating this as well.
        unsafe {
            let mut inner = ManuallyDrop::take(&mut self.inner);
            core::mem::forget(self);
            ManuallyDrop::take(&mut inner.data)
        }
    }

    /// Configures the [`ResourceLimiter`] used to limit resource creation
    /// within this [`Store`].
    ///
    /// Whenever resources such as linear memory, tables, or instances are
    /// allocated the `limiter` specified here is invoked with the store's data
    /// `T` and the returned [`ResourceLimiter`] is used to limit the operation
    /// being allocated. The returned [`ResourceLimiter`] is intended to live
    /// within the `T` itself, for example by storing a
    /// [`StoreLimits`](crate::StoreLimits).
    ///
    /// Note that this limiter is only used to limit the creation/growth of
    /// resources in the future, this does not retroactively attempt to apply
    /// limits to the [`Store`].
    ///
    /// # Examples
    ///
    /// ```
    /// use wasmtime::*;
    ///
    /// struct MyApplicationState {
    ///     my_state: u32,
    ///     limits: StoreLimits,
    /// }
    ///
    /// let engine = Engine::default();
    /// let my_state = MyApplicationState {
    ///     my_state: 42,
    ///     limits: StoreLimitsBuilder::new()
    ///         .memory_size(1 << 20 /* 1 MB */)
    ///         .instances(2)
    ///         .build(),
    /// };
    /// let mut store = Store::new(&engine, my_state);
    /// store.limiter(|state| &mut state.limits);
    ///
    /// // Creation of smaller memories is allowed
    /// Memory::new(&mut store, MemoryType::new(1, None)).unwrap();
    ///
    /// // Creation of a larger memory, however, will exceed the 1MB limit we've
    /// // configured
    /// assert!(Memory::new(&mut store, MemoryType::new(1000, None)).is_err());
    ///
    /// // The number of instances in this store is limited to 2, so the third
    /// // instance here should fail.
    /// let module = Module::new(&engine, "(module)").unwrap();
    /// assert!(Instance::new(&mut store, &module, &[]).is_ok());
    /// assert!(Instance::new(&mut store, &module, &[]).is_ok());
    /// assert!(Instance::new(&mut store, &module, &[]).is_err());
    /// ```
    ///
    /// [`ResourceLimiter`]: crate::ResourceLimiter
    pub fn limiter(
        &mut self,
        mut limiter: impl (FnMut(&mut T) -> &mut dyn crate::ResourceLimiter) + Send + Sync + 'static,
    ) {
        // Apply the limits on instances, tables, and memory given by the limiter:
        let inner = &mut self.inner;
        let (instance_limit, table_limit, memory_limit) = {
            let l = limiter(&mut inner.data);
            (l.instances(), l.tables(), l.memories())
        };
        let innermost = &mut inner.inner;
        innermost.instance_limit = instance_limit;
        innermost.table_limit = table_limit;
        innermost.memory_limit = memory_limit;

        // Save the limiter accessor function:
        inner.limiter = Some(ResourceLimiterInner::Sync(Box::new(limiter)));
    }

    /// Configure a function that runs on calls and returns between WebAssembly
    /// and host code.
    ///
    /// The function is passed a [`CallHook`] argument, which indicates which
    /// state transition the VM is making.
    ///
    /// This function may return a [`Trap`]. If a trap is returned when an
    /// import was called, it is immediately raised as-if the host import had
    /// returned the trap. If a trap is returned after wasm returns to the host
    /// then the wasm function's result is ignored and this trap is returned
    /// instead.
    ///
    /// After this function returns a trap, it may be called for subsequent returns
    /// to host or wasm code as the trap propagates to the root call.
    #[cfg(feature = "call-hook")]
    pub fn call_hook(
        &mut self,
        hook: impl FnMut(StoreContextMut<'_, T>, CallHook) -> Result<()> + Send + Sync + 'static,
    ) {
        self.inner.call_hook = Some(CallHookInner::Sync(Box::new(hook)));
    }

    /// Returns the [`Engine`] that this store is associated with.
    pub fn engine(&self) -> &Engine {
        self.inner.engine()
    }

    /// Perform garbage collection.
    ///
    /// Note that it is not required to actively call this function. GC will
    /// automatically happen according to various internal heuristics. This is
    /// provided if fine-grained control over the GC is desired.
    ///
    /// If you are calling this method after an attempted allocation failed, you
    /// may pass in the [`GcHeapOutOfMemory`][crate::GcHeapOutOfMemory] error.
    /// When you do so, this method will attempt to create enough space in the
    /// GC heap for that allocation, so that it will succeed on the next
    /// attempt.
    ///
    /// This method is only available when the `gc` Cargo feature is enabled.
    #[cfg(feature = "gc")]
    pub fn gc(&mut self, why: Option<&crate::GcHeapOutOfMemory<()>>) {
        assert!(!self.inner.async_support());
        self.inner.gc(why);
    }

    /// Returns the amount fuel in this [`Store`]. When fuel is enabled, it must
    /// be configured via [`Store::set_fuel`].
    ///
    /// # Errors
    ///
    /// This function will return an error if fuel consumption is not enabled
    /// via [`Config::consume_fuel`](crate::Config::consume_fuel).
    pub fn get_fuel(&self) -> Result<u64> {
        self.inner.get_fuel()
    }

    /// Set the fuel to this [`Store`] for wasm to consume while executing.
    ///
    /// For this method to work fuel consumption must be enabled via
    /// [`Config::consume_fuel`](crate::Config::consume_fuel). By default a
    /// [`Store`] starts with 0 fuel for wasm to execute with (meaning it will
    /// immediately trap). This function must be called for the store to have
    /// some fuel to allow WebAssembly to execute.
    ///
    /// Most WebAssembly instructions consume 1 unit of fuel. Some
    /// instructions, such as `nop`, `drop`, `block`, and `loop`, consume 0
    /// units, as any execution cost associated with them involves other
    /// instructions which do consume fuel.
    ///
    /// Note that when fuel is entirely consumed it will cause wasm to trap.
    ///
    /// # Errors
    ///
    /// This function will return an error if fuel consumption is not enabled via
    /// [`Config::consume_fuel`](crate::Config::consume_fuel).
    pub fn set_fuel(&mut self, fuel: u64) -> Result<()> {
        self.inner.set_fuel(fuel)
    }

    /// Configures a [`Store`] to yield execution of async WebAssembly code
    /// periodically.
    ///
    /// When a [`Store`] is configured to consume fuel with
    /// [`Config::consume_fuel`](crate::Config::consume_fuel) this method will
    /// configure WebAssembly to be suspended and control will be yielded back to the
    /// caller every `interval` units of fuel consumed. This is only suitable with use of
    /// a store associated with an [async config](crate::Config::async_support) because
    /// only then are futures used and yields are possible.
    ///
    /// The purpose of this behavior is to ensure that futures which represent
    /// execution of WebAssembly do not execute too long inside their
    /// `Future::poll` method. This allows for some form of cooperative
    /// multitasking where WebAssembly will voluntarily yield control
    /// periodically (based on fuel consumption) back to the running thread.
    ///
    /// Note that futures returned by this crate will automatically flag
    /// themselves to get re-polled if a yield happens. This means that
    /// WebAssembly will continue to execute, just after giving the host an
    /// opportunity to do something else.
    ///
    /// The `interval` parameter indicates how much fuel should be
    /// consumed between yields of an async future. When fuel runs out wasm will trap.
    ///
    /// # Error
    ///
    /// This method will error if it is not called on a store associated with an [async
    /// config](crate::Config::async_support).
    pub fn fuel_async_yield_interval(&mut self, interval: Option<u64>) -> Result<()> {
        self.inner.fuel_async_yield_interval(interval)
    }

    /// Sets the epoch deadline to a certain number of ticks in the future.
    ///
    /// When the Wasm guest code is compiled with epoch-interruption
    /// instrumentation
    /// ([`Config::epoch_interruption()`](crate::Config::epoch_interruption)),
    /// and when the `Engine`'s epoch is incremented
    /// ([`Engine::increment_epoch()`](crate::Engine::increment_epoch))
    /// past a deadline, execution can be configured to either trap or
    /// yield and then continue.
    ///
    /// This deadline is always set relative to the current epoch:
    /// `ticks_beyond_current` ticks in the future. The deadline can
    /// be set explicitly via this method, or refilled automatically
    /// on a yield if configured via
    /// [`epoch_deadline_async_yield_and_update()`](Store::epoch_deadline_async_yield_and_update). After
    /// this method is invoked, the deadline is reached when
    /// [`Engine::increment_epoch()`] has been invoked at least
    /// `ticks_beyond_current` times.
    ///
    /// By default a store will trap immediately with an epoch deadline of 0
    /// (which has always "elapsed"). This method is required to be configured
    /// for stores with epochs enabled to some future epoch deadline.
    ///
    /// See documentation on
    /// [`Config::epoch_interruption()`](crate::Config::epoch_interruption)
    /// for an introduction to epoch-based interruption.
    #[cfg(target_has_atomic = "64")]
    pub fn set_epoch_deadline(&mut self, ticks_beyond_current: u64) {
        self.inner.set_epoch_deadline(ticks_beyond_current);
    }

    /// Configures epoch-deadline expiration to trap.
    ///
    /// When epoch-interruption-instrumented code is executed on this
    /// store and the epoch deadline is reached before completion,
    /// with the store configured in this way, execution will
    /// terminate with a trap as soon as an epoch check in the
    /// instrumented code is reached.
    ///
    /// This behavior is the default if the store is not otherwise
    /// configured via
    /// [`epoch_deadline_trap()`](Store::epoch_deadline_trap),
    /// [`epoch_deadline_callback()`](Store::epoch_deadline_callback) or
    /// [`epoch_deadline_async_yield_and_update()`](Store::epoch_deadline_async_yield_and_update).
    ///
    /// This setting is intended to allow for coarse-grained
    /// interruption, but not a deterministic deadline of a fixed,
    /// finite interval. For deterministic interruption, see the
    /// "fuel" mechanism instead.
    ///
    /// Note that when this is used it's required to call
    /// [`Store::set_epoch_deadline`] or otherwise wasm will always immediately
    /// trap.
    ///
    /// See documentation on
    /// [`Config::epoch_interruption()`](crate::Config::epoch_interruption)
    /// for an introduction to epoch-based interruption.
    #[cfg(target_has_atomic = "64")]
    pub fn epoch_deadline_trap(&mut self) {
        self.inner.epoch_deadline_trap();
    }

    /// Configures epoch-deadline expiration to invoke a custom callback
    /// function.
    ///
    /// When epoch-interruption-instrumented code is executed on this
    /// store and the epoch deadline is reached before completion, the
    /// provided callback function is invoked.
    ///
    /// This callback should either return an [`UpdateDeadline`], or
    /// return an error, which will terminate execution with a trap.
    ///
    /// The [`UpdateDeadline`] is a positive number of ticks to
    /// add to the epoch deadline, as well as indicating what
    /// to do after the callback returns. If the [`Store`] is
    /// configured with async support, then the callback may return
    /// [`UpdateDeadline::Yield`] or [`UpdateDeadline::YieldCustom`]
    /// to yield to the async executor before updating the epoch deadline.
    /// Alternatively, the callback may return [`UpdateDeadline::Continue`] to
    /// update the epoch deadline immediately.
    ///
    /// This setting is intended to allow for coarse-grained
    /// interruption, but not a deterministic deadline of a fixed,
    /// finite interval. For deterministic interruption, see the
    /// "fuel" mechanism instead.
    ///
    /// See documentation on
    /// [`Config::epoch_interruption()`](crate::Config::epoch_interruption)
    /// for an introduction to epoch-based interruption.
    #[cfg(target_has_atomic = "64")]
    pub fn epoch_deadline_callback(
        &mut self,
        callback: impl FnMut(StoreContextMut<T>) -> Result<UpdateDeadline> + Send + Sync + 'static,
    ) {
        self.inner.epoch_deadline_callback(Box::new(callback));
    }
}

impl<'a, T> StoreContext<'a, T> {
    pub(crate) fn async_support(&self) -> bool {
        self.0.async_support()
    }

    /// Returns the underlying [`Engine`] this store is connected to.
    pub fn engine(&self) -> &Engine {
        self.0.engine()
    }

    /// Access the underlying data owned by this `Store`.
    ///
    /// Same as [`Store::data`].
    pub fn data(&self) -> &'a T {
        self.0.data()
    }

    /// Returns the remaining fuel in this store.
    ///
    /// For more information see [`Store::get_fuel`].
    pub fn get_fuel(&self) -> Result<u64> {
        self.0.get_fuel()
    }
}

impl<'a, T> StoreContextMut<'a, T> {
    /// Access the underlying data owned by this `Store`.
    ///
    /// Same as [`Store::data`].
    pub fn data(&self) -> &T {
        self.0.data()
    }

    /// Access the underlying data owned by this `Store`.
    ///
    /// Same as [`Store::data_mut`].
    pub fn data_mut(&mut self) -> &mut T {
        self.0.data_mut()
    }

    /// Returns the underlying [`Engine`] this store is connected to.
    pub fn engine(&self) -> &Engine {
        self.0.engine()
    }

    /// Perform garbage collection of `ExternRef`s.
    ///
    /// Same as [`Store::gc`].
    ///
    /// This method is only available when the `gc` Cargo feature is enabled.
    #[cfg(feature = "gc")]
    pub fn gc(&mut self, why: Option<&crate::GcHeapOutOfMemory<()>>) {
        self.0.gc(why);
    }

    /// Returns remaining fuel in this store.
    ///
    /// For more information see [`Store::get_fuel`]
    pub fn get_fuel(&self) -> Result<u64> {
        self.0.get_fuel()
    }

    /// Set the amount of fuel in this store.
    ///
    /// For more information see [`Store::set_fuel`]
    pub fn set_fuel(&mut self, fuel: u64) -> Result<()> {
        self.0.set_fuel(fuel)
    }

    /// Configures this `Store` to periodically yield while executing futures.
    ///
    /// For more information see [`Store::fuel_async_yield_interval`]
    pub fn fuel_async_yield_interval(&mut self, interval: Option<u64>) -> Result<()> {
        self.0.fuel_async_yield_interval(interval)
    }

    /// Sets the epoch deadline to a certain number of ticks in the future.
    ///
    /// For more information see [`Store::set_epoch_deadline`].
    #[cfg(target_has_atomic = "64")]
    pub fn set_epoch_deadline(&mut self, ticks_beyond_current: u64) {
        self.0.set_epoch_deadline(ticks_beyond_current);
    }

    /// Configures epoch-deadline expiration to trap.
    ///
    /// For more information see [`Store::epoch_deadline_trap`].
    #[cfg(target_has_atomic = "64")]
    pub fn epoch_deadline_trap(&mut self) {
        self.0.epoch_deadline_trap();
    }
}

impl<T> StoreInner<T> {
    #[inline]
    fn data(&self) -> &T {
        &self.data
    }

    #[inline]
    fn data_mut(&mut self) -> &mut T {
        &mut self.data
    }

    #[inline]
    pub fn call_hook(&mut self, s: CallHook) -> Result<()> {
        if self.inner.pkey.is_none() && self.call_hook.is_none() {
            Ok(())
        } else {
            self.call_hook_slow_path(s)
        }
    }

    fn call_hook_slow_path(&mut self, s: CallHook) -> Result<()> {
        if let Some(pkey) = &self.inner.pkey {
            let allocator = self.engine().allocator();
            match s {
                CallHook::CallingWasm | CallHook::ReturningFromHost => {
                    allocator.restrict_to_pkey(*pkey)
                }
                CallHook::ReturningFromWasm | CallHook::CallingHost => allocator.allow_all_pkeys(),
            }
        }

        // Temporarily take the configured behavior to avoid mutably borrowing
        // multiple times.
        if let Some(mut call_hook) = self.call_hook.take() {
            let result = self.invoke_call_hook(&mut call_hook, s);
            self.call_hook = Some(call_hook);
            return result;
        }

        Ok(())
    }

    fn invoke_call_hook(&mut self, call_hook: &mut CallHookInner<T>, s: CallHook) -> Result<()> {
        match call_hook {
            #[cfg(feature = "call-hook")]
            CallHookInner::Sync(hook) => hook((&mut *self).as_context_mut(), s),

            #[cfg(all(feature = "async", feature = "call-hook"))]
            CallHookInner::Async(handler) => {
                if !self.can_block() {
                    bail!("couldn't grab async_cx for call hook")
                }
                return (&mut *self)
                    .as_context_mut()
                    .with_blocking(|store, cx| cx.block_on(handler.handle_call_event(store, s)))?;
            }

            CallHookInner::ForceTypeParameterToBeUsed { uninhabited, .. } => {
                let _ = s;
                match *uninhabited {}
            }
        }
    }

    #[cfg(not(feature = "async"))]
    fn flush_fiber_stack(&mut self) {
        // noop shim so code can assume this always exists.
    }
}

fn get_fuel(injected_fuel: i64, fuel_reserve: u64) -> u64 {
    fuel_reserve.saturating_add_signed(-injected_fuel)
}

// Add remaining fuel from the reserve into the active fuel if there is any left.
fn refuel(
    injected_fuel: &mut i64,
    fuel_reserve: &mut u64,
    yield_interval: Option<NonZeroU64>,
) -> bool {
    let fuel = get_fuel(*injected_fuel, *fuel_reserve);
    if fuel > 0 {
        set_fuel(injected_fuel, fuel_reserve, yield_interval, fuel);
        true
    } else {
        false
    }
}

fn set_fuel(
    injected_fuel: &mut i64,
    fuel_reserve: &mut u64,
    yield_interval: Option<NonZeroU64>,
    new_fuel_amount: u64,
) {
    let interval = yield_interval.unwrap_or(NonZeroU64::MAX).get();
    // If we're yielding periodically we only store the "active" amount of fuel into consumed_ptr
    // for the VM to use.
    let injected = core::cmp::min(interval, new_fuel_amount);
    // Fuel in the VM is stored as an i64, so we have to cap the amount of fuel we inject into the
    // VM at once to be i64 range.
    let injected = core::cmp::min(injected, i64::MAX as u64);
    // Add whatever is left over after injection to the reserve for later use.
    *fuel_reserve = new_fuel_amount - injected;
    // Within the VM we increment to count fuel, so inject a negative amount. The VM will halt when
    // this counter is positive.
    *injected_fuel = -(injected as i64);
}

#[doc(hidden)]
impl StoreOpaque {
    pub fn id(&self) -> StoreId {
        self.store_data.id()
    }

    pub fn bump_resource_counts(&mut self, module: &Module) -> Result<()> {
        fn bump(slot: &mut usize, max: usize, amt: usize, desc: &str) -> Result<()> {
            let new = slot.saturating_add(amt);
            if new > max {
                bail!(
                    "resource limit exceeded: {} count too high at {}",
                    desc,
                    new
                );
            }
            *slot = new;
            Ok(())
        }

        let module = module.env_module();
        let memories = module.num_defined_memories();
        let tables = module.num_defined_tables();

        bump(&mut self.instance_count, self.instance_limit, 1, "instance")?;
        bump(
            &mut self.memory_count,
            self.memory_limit,
            memories,
            "memory",
        )?;
        bump(&mut self.table_count, self.table_limit, tables, "table")?;

        Ok(())
    }

    #[inline]
    pub fn async_support(&self) -> bool {
        cfg!(feature = "async") && self.engine().config().async_support
    }

    #[inline]
    pub fn engine(&self) -> &Engine {
        &self.engine
    }

    #[inline]
    pub fn store_data(&self) -> &StoreData {
        &self.store_data
    }

    #[inline]
    pub fn store_data_mut(&mut self) -> &mut StoreData {
        &mut self.store_data
    }

    #[inline]
    pub(crate) fn modules(&self) -> &ModuleRegistry {
        &self.modules
    }

    #[inline]
    pub(crate) fn modules_mut(&mut self) -> &mut ModuleRegistry {
        &mut self.modules
    }

    pub(crate) fn func_refs_and_modules(&mut self) -> (&mut FuncRefs, &ModuleRegistry) {
        (&mut self.func_refs, &self.modules)
    }

    pub(crate) fn host_globals(
        &self,
    ) -> &PrimaryMap<DefinedGlobalIndex, StoreBox<VMHostGlobalContext>> {
        &self.host_globals
    }

    pub(crate) fn host_globals_mut(
        &mut self,
    ) -> &mut PrimaryMap<DefinedGlobalIndex, StoreBox<VMHostGlobalContext>> {
        &mut self.host_globals
    }

    pub fn module_for_instance(&self, instance: StoreInstanceId) -> Option<&'_ Module> {
        instance.store_id().assert_belongs_to(self.id());
        match self.instances[instance.instance()].kind {
            StoreInstanceKind::Dummy => None,
            StoreInstanceKind::Real { module_id } => {
                let module = self
                    .modules()
                    .lookup_module_by_id(module_id)
                    .expect("should always have a registered module for real instances");
                Some(module)
            }
        }
    }

    /// Accessor from `InstanceId` to `&vm::Instance`.
    ///
    /// Note that if you have a `StoreInstanceId` you should use
    /// `StoreInstanceId::get` instead. This assumes that `id` has been
    /// validated to already belong to this store.
    #[inline]
    pub fn instance(&self, id: InstanceId) -> &vm::Instance {
        self.instances[id].handle.get()
    }

    /// Accessor from `InstanceId` to `Pin<&mut vm::Instance>`.
    ///
    /// Note that if you have a `StoreInstanceId` you should use
    /// `StoreInstanceId::get_mut` instead. This assumes that `id` has been
    /// validated to already belong to this store.
    #[inline]
    pub fn instance_mut(&mut self, id: InstanceId) -> Pin<&mut vm::Instance> {
        self.instances[id].handle.get_mut()
    }

    /// Access multiple instances specified via `ids`.
    ///
    /// # Panics
    ///
    /// This method will panic if any indices in `ids` overlap.
    ///
    /// # Safety
    ///
    /// This method is not safe if the returned instances are used to traverse
    /// "laterally" between other instances. For example accessing imported
    /// items in an instance may traverse laterally to a sibling instance thus
    /// aliasing a returned value here. The caller must ensure that only defined
    /// items within the instances themselves are accessed.
    #[inline]
    pub unsafe fn optional_gc_store_and_instances_mut<const N: usize>(
        &mut self,
        ids: [InstanceId; N],
    ) -> (Option<&mut GcStore>, [Pin<&mut vm::Instance>; N]) {
        let instances = self
            .instances
            .get_disjoint_mut(ids)
            .unwrap()
            .map(|h| h.handle.get_mut());
        (self.gc_store.as_mut(), instances)
    }

    /// Pair of `Self::optional_gc_store_mut` and `Self::instance_mut`
    pub fn optional_gc_store_and_instance_mut(
        &mut self,
        id: InstanceId,
    ) -> (Option<&mut GcStore>, Pin<&mut vm::Instance>) {
        (self.gc_store.as_mut(), self.instances[id].handle.get_mut())
    }

    /// Get all instances (ignoring dummy instances) within this store.
    pub fn all_instances<'a>(&'a mut self) -> impl ExactSizeIterator<Item = Instance> + 'a {
        let instances = self
            .instances
            .iter()
            .filter_map(|(id, inst)| {
                if let StoreInstanceKind::Dummy = inst.kind {
                    None
                } else {
                    Some(id)
                }
            })
            .collect::<Vec<_>>();
        instances
            .into_iter()
            .map(|i| Instance::from_wasmtime(i, self))
    }

    /// Get all memories (host- or Wasm-defined) within this store.
    pub fn all_memories<'a>(&'a self) -> impl Iterator<Item = Memory> + 'a {
        // NB: Host-created memories have dummy instances. Therefore, we can get
        // all memories in the store by iterating over all instances (including
        // dummy instances) and getting each of their defined memories.
        let id = self.id();
        self.instances
            .iter()
            .flat_map(move |(_, instance)| instance.handle.get().defined_memories(id))
    }

    /// Iterate over all tables (host- or Wasm-defined) within this store.
    pub fn for_each_table(&mut self, mut f: impl FnMut(&mut Self, Table)) {
        // NB: Host-created tables have dummy instances. Therefore, we can get
        // all tables in the store by iterating over all instances (including
        // dummy instances) and getting each of their defined memories.
        for id in self.instances.keys() {
            let instance = StoreInstanceId::new(self.id(), id);
            for table in 0..self.instance(id).env_module().num_defined_tables() {
                let table = DefinedTableIndex::new(table);
                f(self, Table::from_raw(instance, table));
            }
        }
    }

    /// Iterate over all globals (host- or Wasm-defined) within this store.
    pub fn for_each_global(&mut self, mut f: impl FnMut(&mut Self, Global)) {
        // First enumerate all the host-created globals.
        for global in self.host_globals.keys() {
            let global = Global::new_host(self, global);
            f(self, global);
        }

        // Then enumerate all instances' defined globals.
        for id in self.instances.keys() {
            for index in 0..self.instance(id).env_module().num_defined_globals() {
                let index = DefinedGlobalIndex::new(index);
                let global = Global::new_instance(self, id, index);
                f(self, global);
            }
        }
    }

    #[cfg(all(feature = "std", any(unix, windows)))]
    pub fn set_signal_handler(&mut self, handler: Option<SignalHandler>) {
        self.signal_handler = handler;
    }

    #[inline]
    pub fn vm_store_context(&self) -> &VMStoreContext {
        &self.vm_store_context
    }

    #[inline]
    pub fn vm_store_context_mut(&mut self) -> &mut VMStoreContext {
        &mut self.vm_store_context
    }

    #[inline(never)]
    pub(crate) fn allocate_gc_heap(&mut self) -> Result<()> {
        log::trace!("allocating GC heap for store {:?}", self.id());

        assert!(self.gc_store.is_none());
        assert_eq!(
            self.vm_store_context.gc_heap.base.as_non_null(),
            NonNull::dangling(),
        );
        assert_eq!(self.vm_store_context.gc_heap.current_length(), 0);

        let vmstore = self.traitobj();
        let gc_store = allocate_gc_store(self.engine(), vmstore, self.get_pkey())?;
        self.vm_store_context.gc_heap = gc_store.vmmemory_definition();
        self.gc_store = Some(gc_store);
        return Ok(());

        #[cfg(feature = "gc")]
        fn allocate_gc_store(
            engine: &Engine,
            vmstore: NonNull<dyn vm::VMStore>,
            pkey: Option<ProtectionKey>,
        ) -> Result<GcStore> {
            use wasmtime_environ::packed_option::ReservedValue;

            ensure!(
                engine.features().gc_types(),
                "cannot allocate a GC store when GC is disabled at configuration time"
            );

            // First, allocate the memory that will be our GC heap's storage.
            let mut request = InstanceAllocationRequest {
                id: InstanceId::reserved_value(),
                runtime_info: &ModuleRuntimeInfo::bare(Arc::new(
                    wasmtime_environ::Module::default(),
                )),
                imports: vm::Imports::default(),
                store: StorePtr::new(vmstore),
                #[cfg(feature = "wmemcheck")]
                wmemcheck: false,
                pkey,
                tunables: engine.tunables(),
            };
            let mem_ty = engine.tunables().gc_heap_memory_type();
            let tunables = engine.tunables();

            let (mem_alloc_index, mem) =
                engine
                    .allocator()
                    .allocate_memory(&mut request, &mem_ty, tunables, None)?;

            // Then, allocate the actual GC heap, passing in that memory
            // storage.
            let gc_runtime = engine
                .gc_runtime()
                .context("no GC runtime: GC disabled at compile time or configuration time")?;
            let (index, heap) =
                engine
                    .allocator()
                    .allocate_gc_heap(engine, &**gc_runtime, mem_alloc_index, mem)?;

            Ok(GcStore::new(index, heap))
        }

        #[cfg(not(feature = "gc"))]
        fn allocate_gc_store(
            _engine: &Engine,
            _vmstore: NonNull<dyn vm::VMStore>,
            _pkey: Option<ProtectionKey>,
        ) -> Result<GcStore> {
            bail!("cannot allocate a GC store: the `gc` feature was disabled at compile time")
        }
    }

    #[inline]
    pub(crate) fn gc_store(&self) -> Result<&GcStore> {
        match &self.gc_store {
            Some(gc_store) => Ok(gc_store),
            None => bail!("GC heap not initialized yet"),
        }
    }

    #[inline]
    pub(crate) fn gc_store_mut(&mut self) -> Result<&mut GcStore> {
        if self.gc_store.is_none() {
            self.allocate_gc_heap()?;
        }
        Ok(self.unwrap_gc_store_mut())
    }

    /// If this store is configured with a GC heap, return a mutable reference
    /// to it. Otherwise, return `None`.
    #[inline]
    pub(crate) fn optional_gc_store_mut(&mut self) -> Option<&mut GcStore> {
        if cfg!(not(feature = "gc")) || !self.engine.features().gc_types() {
            debug_assert!(self.gc_store.is_none());
            None
        } else {
            self.gc_store.as_mut()
        }
    }

    #[inline]
    #[track_caller]
    #[cfg(feature = "gc")]
    pub(crate) fn unwrap_gc_store(&self) -> &GcStore {
        self.gc_store
            .as_ref()
            .expect("attempted to access the store's GC heap before it has been allocated")
    }

    #[inline]
    #[track_caller]
    pub(crate) fn unwrap_gc_store_mut(&mut self) -> &mut GcStore {
        self.gc_store
            .as_mut()
            .expect("attempted to access the store's GC heap before it has been allocated")
    }

    #[inline]
    pub(crate) fn gc_roots(&self) -> &RootSet {
        &self.gc_roots
    }

    #[inline]
    #[cfg(feature = "gc")]
    pub(crate) fn gc_roots_mut(&mut self) -> &mut RootSet {
        &mut self.gc_roots
    }

    #[inline]
    pub(crate) fn exit_gc_lifo_scope(&mut self, scope: usize) {
        self.gc_roots.exit_lifo_scope(self.gc_store.as_mut(), scope);
    }

    #[cfg(feature = "gc")]
    fn do_gc(&mut self) {
        assert!(
            !self.async_support(),
            "must use `store.gc_async()` instead of `store.gc()` for async stores"
        );

        // If the GC heap hasn't been initialized, there is nothing to collect.
        if self.gc_store.is_none() {
            return;
        }

        log::trace!("============ Begin GC ===========");

        // Take the GC roots out of `self` so we can borrow it mutably but still
        // call mutable methods on `self`.
        let mut roots = core::mem::take(&mut self.gc_roots_list);

        self.trace_roots(&mut roots);
        self.unwrap_gc_store_mut().gc(unsafe { roots.iter() });

        // Restore the GC roots for the next GC.
        roots.clear();
        self.gc_roots_list = roots;

        log::trace!("============ End GC ===========");
    }

    #[cfg(feature = "gc")]
    fn trace_roots(&mut self, gc_roots_list: &mut GcRootsList) {
        log::trace!("Begin trace GC roots");

        // We shouldn't have any leftover, stale GC roots.
        assert!(gc_roots_list.is_empty());

        self.trace_wasm_stack_roots(gc_roots_list);
        #[cfg(feature = "stack-switching")]
        self.trace_wasm_continuation_roots(gc_roots_list);
        self.trace_vmctx_roots(gc_roots_list);
        self.trace_user_roots(gc_roots_list);

        log::trace!("End trace GC roots")
    }

    #[cfg(feature = "gc")]
    fn trace_wasm_stack_frame(
        &self,
        gc_roots_list: &mut GcRootsList,
        frame: crate::runtime::vm::Frame,
    ) {
        use crate::runtime::vm::SendSyncPtr;
        use core::ptr::NonNull;

        let pc = frame.pc();
        debug_assert!(pc != 0, "we should always get a valid PC for Wasm frames");

        let fp = frame.fp() as *mut usize;
        debug_assert!(
            !fp.is_null(),
            "we should always get a valid frame pointer for Wasm frames"
        );

        let module_info = self
            .modules()
            .lookup_module_by_pc(pc)
            .expect("should have module info for Wasm frame");

        let stack_map = match module_info.lookup_stack_map(pc) {
            Some(sm) => sm,
            None => {
                log::trace!("No stack map for this Wasm frame");
                return;
            }
        };
        log::trace!(
            "We have a stack map that maps {} bytes in this Wasm frame",
            stack_map.frame_size()
        );

        let sp = unsafe { stack_map.sp(fp) };
        for stack_slot in unsafe { stack_map.live_gc_refs(sp) } {
            let raw: u32 = unsafe { core::ptr::read(stack_slot) };
            log::trace!("Stack slot @ {stack_slot:p} = {raw:#x}");

            let gc_ref = VMGcRef::from_raw_u32(raw);
            if gc_ref.is_some() {
                unsafe {
                    gc_roots_list
                        .add_wasm_stack_root(SendSyncPtr::new(NonNull::new(stack_slot).unwrap()));
                }
            }
        }
    }

    #[cfg(feature = "gc")]
    fn trace_wasm_stack_roots(&mut self, gc_roots_list: &mut GcRootsList) {
        use crate::runtime::vm::Backtrace;
        log::trace!("Begin trace GC roots :: Wasm stack");

        Backtrace::trace(self, |frame| {
            self.trace_wasm_stack_frame(gc_roots_list, frame);
            core::ops::ControlFlow::Continue(())
        });

        log::trace!("End trace GC roots :: Wasm stack");
    }

    #[cfg(all(feature = "gc", feature = "stack-switching"))]
    fn trace_wasm_continuation_roots(&mut self, gc_roots_list: &mut GcRootsList) {
        use crate::{runtime::vm::Backtrace, vm::VMStackState};
        log::trace!("Begin trace GC roots :: continuations");

        for continuation in &self.continuations {
            let state = continuation.common_stack_information.state;

            // FIXME(frank-emrich) In general, it is not enough to just trace
            // through the stacks of continuations; we also need to look through
            // their `cont.bind` arguments. However, we don't currently have
            // enough RTTI information to check if any of the values in the
            // buffers used by `cont.bind` are GC values. As a workaround, note
            // that we currently disallow cont.bind-ing GC values altogether.
            // This way, it is okay not to check them here.
            match state {
                VMStackState::Suspended => {
                    Backtrace::trace_suspended_continuation(self, continuation.deref(), |frame| {
                        self.trace_wasm_stack_frame(gc_roots_list, frame);
                        core::ops::ControlFlow::Continue(())
                    });
                }
                VMStackState::Running => {
                    // Handled by `trace_wasm_stack_roots`.
                }
                VMStackState::Parent => {
                    // We don't know whether our child is suspended or running, but in
                    // either case things should be hanlded correctly when traversing
                    // further along in the chain, nothing required at this point.
                }
                VMStackState::Fresh | VMStackState::Returned => {
                    // Fresh/Returned continuations have no gc values on their stack.
                }
            }
        }

        log::trace!("End trace GC roots :: continuations");
    }

    #[cfg(feature = "gc")]
    fn trace_vmctx_roots(&mut self, gc_roots_list: &mut GcRootsList) {
        log::trace!("Begin trace GC roots :: vmctx");
        self.for_each_global(|store, global| global.trace_root(store, gc_roots_list));
        self.for_each_table(|store, table| table.trace_roots(store, gc_roots_list));
        log::trace!("End trace GC roots :: vmctx");
    }

    #[cfg(feature = "gc")]
    fn trace_user_roots(&mut self, gc_roots_list: &mut GcRootsList) {
        log::trace!("Begin trace GC roots :: user");
        self.gc_roots.trace_roots(gc_roots_list);
        log::trace!("End trace GC roots :: user");
    }

    /// Insert a host-allocated GC type into this store.
    ///
    /// This makes it suitable for the embedder to allocate instances of this
    /// type in this store, and we don't have to worry about the type being
    /// reclaimed (since it is possible that none of the Wasm modules in this
    /// store are holding it alive).
    #[cfg(feature = "gc")]
    pub(crate) fn insert_gc_host_alloc_type(&mut self, ty: crate::type_registry::RegisteredType) {
        self.gc_host_alloc_types.insert(ty);
    }

    pub fn get_fuel(&self) -> Result<u64> {
        anyhow::ensure!(
            self.engine().tunables().consume_fuel,
            "fuel is not configured in this store"
        );
        let injected_fuel = unsafe { *self.vm_store_context.fuel_consumed.get() };
        Ok(get_fuel(injected_fuel, self.fuel_reserve))
    }

    fn refuel(&mut self) -> bool {
        let injected_fuel = unsafe { &mut *self.vm_store_context.fuel_consumed.get() };
        refuel(
            injected_fuel,
            &mut self.fuel_reserve,
            self.fuel_yield_interval,
        )
    }

    pub fn set_fuel(&mut self, fuel: u64) -> Result<()> {
        anyhow::ensure!(
            self.engine().tunables().consume_fuel,
            "fuel is not configured in this store"
        );
        let injected_fuel = unsafe { &mut *self.vm_store_context.fuel_consumed.get() };
        set_fuel(
            injected_fuel,
            &mut self.fuel_reserve,
            self.fuel_yield_interval,
            fuel,
        );
        Ok(())
    }

    pub fn fuel_async_yield_interval(&mut self, interval: Option<u64>) -> Result<()> {
        anyhow::ensure!(
            self.engine().tunables().consume_fuel,
            "fuel is not configured in this store"
        );
        anyhow::ensure!(
            self.engine().config().async_support,
            "async support is not configured in this store"
        );
        anyhow::ensure!(
            interval != Some(0),
            "fuel_async_yield_interval must not be 0"
        );
        self.fuel_yield_interval = interval.and_then(|i| NonZeroU64::new(i));
        // Reset the fuel active + reserve states by resetting the amount.
        self.set_fuel(self.get_fuel()?)
    }

    #[inline]
    pub fn signal_handler(&self) -> Option<*const SignalHandler> {
        let handler = self.signal_handler.as_ref()?;
        Some(handler)
    }

    #[inline]
    pub fn vm_store_context_ptr(&self) -> NonNull<VMStoreContext> {
        NonNull::from(&self.vm_store_context)
    }

    #[inline]
    pub fn default_caller(&self) -> NonNull<VMContext> {
        self.default_caller_vmctx.as_non_null()
    }

    #[inline]
    pub fn traitobj(&self) -> NonNull<dyn vm::VMStore> {
        self.traitobj.as_raw().unwrap()
    }

    #[inline]
    pub fn traitobj_mut(&mut self) -> &mut dyn vm::VMStore {
        unsafe { self.traitobj().as_mut() }
    }

    /// Takes the cached `Vec<Val>` stored internally across hostcalls to get
    /// used as part of calling the host in a `Func::new` method invocation.
    #[inline]
    pub fn take_hostcall_val_storage(&mut self) -> Vec<Val> {
        mem::take(&mut self.hostcall_val_storage)
    }

    /// Restores the vector previously taken by `take_hostcall_val_storage`
    /// above back into the store, allowing it to be used in the future for the
    /// next wasm->host call.
    #[inline]
    pub fn save_hostcall_val_storage(&mut self, storage: Vec<Val>) {
        if storage.capacity() > self.hostcall_val_storage.capacity() {
            self.hostcall_val_storage = storage;
        }
    }

    /// Same as `take_hostcall_val_storage`, but for the direction of the host
    /// calling wasm.
    #[inline]
    pub fn take_wasm_val_raw_storage(&mut self) -> Vec<ValRaw> {
        mem::take(&mut self.wasm_val_raw_storage)
    }

    /// Same as `save_hostcall_val_storage`, but for the direction of the host
    /// calling wasm.
    #[inline]
    pub fn save_wasm_val_raw_storage(&mut self, storage: Vec<ValRaw>) {
        if storage.capacity() > self.wasm_val_raw_storage.capacity() {
            self.wasm_val_raw_storage = storage;
        }
    }

    /// Translates a WebAssembly fault at the native `pc` and native `addr` to a
    /// WebAssembly-relative fault.
    ///
    /// This function may abort the process if `addr` is not found to actually
    /// reside in any linear memory. In such a situation it means that the
    /// segfault was erroneously caught by Wasmtime and is possibly indicative
    /// of a code generator bug.
    ///
    /// This function returns `None` for dynamically-bounds-checked-memories
    /// with spectre mitigations enabled since the hardware fault address is
    /// always zero in these situations which means that the trapping context
    /// doesn't have enough information to report the fault address.
    pub(crate) fn wasm_fault(&self, pc: usize, addr: usize) -> Option<vm::WasmFault> {
        // There are a few instances where a "close to zero" pointer is loaded
        // and we expect that to happen:
        //
        // * Explicitly bounds-checked memories with spectre-guards enabled will
        //   cause out-of-bounds accesses to get routed to address 0, so allow
        //   wasm instructions to fault on the null address.
        // * `call_indirect` when invoking a null function pointer may load data
        //   from the a `VMFuncRef` whose address is null, meaning any field of
        //   `VMFuncRef` could be the address of the fault.
        //
        // In these situations where the address is so small it won't be in any
        // instance, so skip the checks below.
        if addr <= mem::size_of::<VMFuncRef>() {
            const _: () = {
                // static-assert that `VMFuncRef` isn't too big to ensure that
                // it lives solely within the first page as we currently only
                // have the guarantee that the first page of memory is unmapped,
                // no more.
                assert!(mem::size_of::<VMFuncRef>() <= 512);
            };
            return None;
        }

        // Search all known instances in this store for this address. Note that
        // this is probably not the speediest way to do this. Traps, however,
        // are generally not expected to be super fast and additionally stores
        // probably don't have all that many instances or memories.
        //
        // If this loop becomes hot in the future, however, it should be
        // possible to precompute maps about linear memories in a store and have
        // a quicker lookup.
        let mut fault = None;
        for (_, instance) in self.instances.iter() {
            if let Some(f) = instance.handle.get().wasm_fault(addr) {
                assert!(fault.is_none());
                fault = Some(f);
            }
        }
        if fault.is_some() {
            return fault;
        }

        cfg_if::cfg_if! {
            if #[cfg(feature = "std")] {
                // With the standard library a rich error can be printed here
                // to stderr and the native abort path is used.
                eprintln!(
                    "\
Wasmtime caught a segfault for a wasm program because the faulting instruction
is allowed to segfault due to how linear memories are implemented. The address
that was accessed, however, is not known to any linear memory in use within this
Store. This may be indicative of a critical bug in Wasmtime's code generation
because all addresses which are known to be reachable from wasm won't reach this
message.

    pc:      0x{pc:x}
    address: 0x{addr:x}

This is a possible security issue because WebAssembly has accessed something it
shouldn't have been able to. Other accesses may have succeeded and this one just
happened to be caught. The process will now be aborted to prevent this damage
from going any further and to alert what's going on. If this is a security
issue please reach out to the Wasmtime team via its security policy
at https://bytecodealliance.org/security.
"
                );
                std::process::abort();
            } else if #[cfg(panic = "abort")] {
                // Without the standard library but with `panic=abort` then
                // it's safe to panic as that's known to halt execution. For
                // now avoid the above error message as well since without
                // `std` it's probably best to be a bit more size-conscious.
                let _ = pc;
                panic!("invalid fault");
            } else {
                // Without `std` and with `panic = "unwind"` there's no
                // dedicated API to abort the process portably, so manufacture
                // this with a double-panic.
                let _ = pc;

                struct PanicAgainOnDrop;

                impl Drop for PanicAgainOnDrop {
                    fn drop(&mut self) {
                        panic!("panicking again to trigger a process abort");
                    }

                }

                let _bomb = PanicAgainOnDrop;

                panic!("invalid fault");
            }
        }
    }

    /// Retrieve the store's protection key.
    #[inline]
    pub(crate) fn get_pkey(&self) -> Option<ProtectionKey> {
        self.pkey
    }

    #[inline]
    #[cfg(feature = "component-model")]
    pub(crate) fn component_resource_state(
        &mut self,
    ) -> (
        &mut vm::component::CallContexts,
        &mut vm::component::ResourceTable,
        &mut crate::component::HostResourceData,
    ) {
        (
            &mut self.component_calls,
            &mut self.component_host_table,
            &mut self.host_resource_data,
        )
    }

    #[cfg(feature = "component-model")]
    pub(crate) fn push_component_instance(&mut self, instance: crate::component::Instance) {
        // We don't actually need the instance itself right now, but it seems
        // like something we will almost certainly eventually want to keep
        // around, so force callers to provide it.
        let _ = instance;

        self.num_component_instances += 1;
    }

    #[inline]
    #[cfg(feature = "component-model")]
    pub(crate) fn component_resource_state_with_instance(
        &mut self,
        instance: crate::component::Instance,
    ) -> (
        &mut vm::component::CallContexts,
        &mut vm::component::ResourceTable,
        &mut crate::component::HostResourceData,
        Pin<&mut vm::component::ComponentInstance>,
    ) {
        (
            &mut self.component_calls,
            &mut self.component_host_table,
            &mut self.host_resource_data,
            instance.id().from_data_get_mut(&mut self.store_data),
        )
    }

    #[cfg(feature = "async")]
    pub(crate) fn fiber_async_state_mut(&mut self) -> &mut fiber::AsyncState {
        &mut self.async_state
    }

    #[cfg(feature = "component-model-async")]
    pub(crate) fn concurrent_async_state_mut(&mut self) -> &mut concurrent::AsyncState {
        &mut self.concurrent_async_state
    }

    #[cfg(feature = "async")]
    pub(crate) fn has_pkey(&self) -> bool {
        self.pkey.is_some()
    }

    pub(crate) fn executor(&mut self) -> ExecutorRef<'_> {
        match &mut self.executor {
            Executor::Interpreter(i) => ExecutorRef::Interpreter(i.as_interpreter_ref()),
            #[cfg(has_host_compiler_backend)]
            Executor::Native => ExecutorRef::Native,
        }
    }

    #[cfg(feature = "async")]
    pub(crate) fn swap_executor(&mut self, executor: &mut Executor) {
        mem::swap(&mut self.executor, executor);
    }

    pub(crate) fn unwinder(&self) -> &'static dyn Unwind {
        match &self.executor {
            Executor::Interpreter(i) => i.unwinder(),
            #[cfg(has_host_compiler_backend)]
            Executor::Native => &vm::UnwindHost,
        }
    }

    /// Allocates a new continuation. Note that we currently don't support
    /// deallocating them. Instead, all continuations remain allocated
    /// throughout the store's lifetime.
    #[cfg(feature = "stack-switching")]
    pub fn allocate_continuation(&mut self) -> Result<*mut VMContRef> {
        // FIXME(frank-emrich) Do we need to pin this?
        let mut continuation = Box::new(VMContRef::empty());
        let stack_size = self.engine.config().async_stack_size;
        let stack = crate::vm::VMContinuationStack::new(stack_size)?;
        continuation.stack = stack;
        let ptr = continuation.deref_mut() as *mut VMContRef;
        self.continuations.push(continuation);
        Ok(ptr)
    }

    /// Constructs and executes an `InstanceAllocationRequest` and pushes the
    /// returned instance into the store.
    ///
    /// This is a helper method for invoking
    /// `InstanceAllocator::allocate_module` with the appropriate parameters
    /// from this store's own configuration. The `kind` provided is used to
    /// distinguish between "real" modules and dummy ones that are synthesized
    /// for embedder-created memories, globals, tables, etc. The `kind` will
    /// also use a different instance allocator by default, the one passed in,
    /// rather than the engine's default allocator.
    ///
    /// This method will push the instance within `StoreOpaque` onto the
    /// `instances` array and return the `InstanceId` which can be use to look
    /// it up within the store.
    ///
    /// # Safety
    ///
    /// The `imports` provided must be correctly sized/typed for the module
    /// being allocated.
    pub(crate) unsafe fn allocate_instance(
        &mut self,
        kind: AllocateInstanceKind<'_>,
        runtime_info: &ModuleRuntimeInfo,
        imports: Imports<'_>,
    ) -> Result<InstanceId> {
        let id = self.instances.next_key();

        let allocator = match kind {
            AllocateInstanceKind::Module(_) => self.engine().allocator(),
            AllocateInstanceKind::Dummy { allocator } => allocator,
        };
        // SAFETY: this function's own contract is the same as
        // `allocate_module`, namely the imports provided are valid.
        let handle = unsafe {
            allocator.allocate_module(InstanceAllocationRequest {
                id,
                runtime_info,
                imports,
                store: StorePtr::new(self.traitobj()),
                #[cfg(feature = "wmemcheck")]
                wmemcheck: self.engine().config().wmemcheck,
                pkey: self.get_pkey(),
                tunables: self.engine().tunables(),
            })?
        };

        let actual = match kind {
            AllocateInstanceKind::Module(module_id) => {
                log::trace!(
                    "Adding instance to store: store={:?}, module={module_id:?}, instance={id:?}",
                    self.id()
                );
                self.instances.push(StoreInstance {
                    handle,
                    kind: StoreInstanceKind::Real { module_id },
                })
            }
            AllocateInstanceKind::Dummy { .. } => {
                log::trace!(
                    "Adding dummy instance to store: store={:?}, instance={id:?}",
                    self.id()
                );
                self.instances.push(StoreInstance {
                    handle,
                    kind: StoreInstanceKind::Dummy,
                })
            }
        };

        // double-check we didn't accidentally allocate two instances and our
        // prediction of what the id would be is indeed the id it should be.
        assert_eq!(id, actual);

        Ok(id)
    }
}

/// Helper parameter to [`StoreOpaque::allocate_instance`].
pub(crate) enum AllocateInstanceKind<'a> {
    /// An embedder-provided module is being allocated meaning that the default
    /// engine's allocator will be used.
    Module(RegisteredModuleId),

    /// Add a dummy instance that to the store.
    ///
    /// These are instances that are just implementation details of something
    /// else (e.g. host-created memories that are not actually defined in any
    /// Wasm module) and therefore shouldn't show up in things like core dumps.
    ///
    /// A custom, typically OnDemand-flavored, allocator is provided to execute
    /// the allocation.
    Dummy {
        allocator: &'a dyn InstanceAllocator,
    },
}

unsafe impl<T> vm::VMStore for StoreInner<T> {
    #[cfg(feature = "component-model-async")]
    fn component_async_store(
        &mut self,
    ) -> &mut dyn crate::runtime::component::VMComponentAsyncStore {
        self
    }

    fn store_opaque(&self) -> &StoreOpaque {
        &self.inner
    }

    fn store_opaque_mut(&mut self) -> &mut StoreOpaque {
        &mut self.inner
    }

    fn memory_growing(
        &mut self,
        current: usize,
        desired: usize,
        maximum: Option<usize>,
    ) -> Result<bool, anyhow::Error> {
        match self.limiter {
            Some(ResourceLimiterInner::Sync(ref mut limiter)) => {
                limiter(&mut self.data).memory_growing(current, desired, maximum)
            }
            #[cfg(feature = "async")]
            Some(ResourceLimiterInner::Async(_)) => self.block_on(|store| {
                let limiter = match &mut store.0.limiter {
                    Some(ResourceLimiterInner::Async(limiter)) => limiter,
                    _ => unreachable!(),
                };
                limiter(&mut store.0.data).memory_growing(current, desired, maximum)
            })?,
            None => Ok(true),
        }
    }

    fn memory_grow_failed(&mut self, error: anyhow::Error) -> Result<()> {
        match self.limiter {
            Some(ResourceLimiterInner::Sync(ref mut limiter)) => {
                limiter(&mut self.data).memory_grow_failed(error)
            }
            #[cfg(feature = "async")]
            Some(ResourceLimiterInner::Async(ref mut limiter)) => {
                limiter(&mut self.data).memory_grow_failed(error)
            }
            None => {
                log::debug!("ignoring memory growth failure error: {error:?}");
                Ok(())
            }
        }
    }

    fn table_growing(
        &mut self,
        current: usize,
        desired: usize,
        maximum: Option<usize>,
    ) -> Result<bool, anyhow::Error> {
        match self.limiter {
            Some(ResourceLimiterInner::Sync(ref mut limiter)) => {
                limiter(&mut self.data).table_growing(current, desired, maximum)
            }
            #[cfg(feature = "async")]
            Some(ResourceLimiterInner::Async(_)) => self.block_on(|store| {
                let limiter = match &mut store.0.limiter {
                    Some(ResourceLimiterInner::Async(limiter)) => limiter,
                    _ => unreachable!(),
                };
                limiter(&mut store.0.data).table_growing(current, desired, maximum)
            })?,
            None => Ok(true),
        }
    }

    fn table_grow_failed(&mut self, error: anyhow::Error) -> Result<()> {
        match self.limiter {
            Some(ResourceLimiterInner::Sync(ref mut limiter)) => {
                limiter(&mut self.data).table_grow_failed(error)
            }
            #[cfg(feature = "async")]
            Some(ResourceLimiterInner::Async(ref mut limiter)) => {
                limiter(&mut self.data).table_grow_failed(error)
            }
            None => {
                log::debug!("ignoring table growth failure: {error:?}");
                Ok(())
            }
        }
    }

    fn out_of_gas(&mut self) -> Result<()> {
        if !self.refuel() {
            return Err(Trap::OutOfFuel.into());
        }
        #[cfg(feature = "async")]
        if self.fuel_yield_interval.is_some() {
            self.async_yield_impl()?;
        }
        Ok(())
    }

    #[cfg(target_has_atomic = "64")]
    fn new_epoch(&mut self) -> Result<u64, anyhow::Error> {
        // Temporarily take the configured behavior to avoid mutably borrowing
        // multiple times.
        let mut behavior = self.epoch_deadline_behavior.take();
        let delta_result = match &mut behavior {
            None => Err(Trap::Interrupt.into()),
            Some(callback) => callback((&mut *self).as_context_mut()).and_then(|update| {
                let delta = match update {
                    UpdateDeadline::Continue(delta) => delta,
                    #[cfg(feature = "async")]
                    UpdateDeadline::Yield(delta) => {
                        assert!(
                            self.async_support(),
                            "cannot use `UpdateDeadline::Yield` without enabling async support in the config"
                        );
                        // Do the async yield. May return a trap if future was
                        // canceled while we're yielded.
                        self.async_yield_impl()?;
                        delta
                    }
                    #[cfg(feature = "async")]
                    UpdateDeadline::YieldCustom(delta, future) => {
                        assert!(
                            self.async_support(),
                            "cannot use `UpdateDeadline::YieldCustom` without enabling async support in the config"
                        );

                        // When control returns, we have a `Result<()>` passed
                        // in from the host fiber. If this finished successfully then
                        // we were resumed normally via a `poll`, so keep going.  If
                        // the future was dropped while we were yielded, then we need
                        // to clean up this fiber. Do so by raising a trap which will
                        // abort all wasm and get caught on the other side to clean
                        // things up.
                        self.block_on(|_| future)?;
                        delta
                    }
                };

                // Set a new deadline and return the new epoch deadline so
                // the Wasm code doesn't have to reload it.
                self.set_epoch_deadline(delta);
                Ok(self.get_epoch_deadline())
            })
        };

        // Put back the original behavior which was replaced by `take`.
        self.epoch_deadline_behavior = behavior;
        delta_result
    }

    #[cfg(feature = "gc")]
    unsafe fn maybe_async_grow_or_collect_gc_heap(
        &mut self,
        root: Option<VMGcRef>,
        bytes_needed: Option<u64>,
    ) -> Result<Option<VMGcRef>> {
        unsafe { self.inner.maybe_async_gc(root, bytes_needed) }
    }

    #[cfg(not(feature = "gc"))]
    unsafe fn maybe_async_grow_or_collect_gc_heap(
        &mut self,
        root: Option<VMGcRef>,
        _bytes_needed: Option<u64>,
    ) -> Result<Option<VMGcRef>> {
        Ok(root)
    }

    #[cfg(feature = "component-model")]
    fn component_calls(&mut self) -> &mut vm::component::CallContexts {
        &mut self.component_calls
    }
}

impl<T> StoreInner<T> {
    #[cfg(target_has_atomic = "64")]
    pub(crate) fn set_epoch_deadline(&mut self, delta: u64) {
        // Set a new deadline based on the "epoch deadline delta".
        //
        // Also, note that when this update is performed while Wasm is
        // on the stack, the Wasm will reload the new value once we
        // return into it.
        let current_epoch = self.engine().current_epoch();
        let epoch_deadline = self.vm_store_context.epoch_deadline.get_mut();
        *epoch_deadline = current_epoch + delta;
    }

    #[cfg(target_has_atomic = "64")]
    fn epoch_deadline_trap(&mut self) {
        self.epoch_deadline_behavior = None;
    }

    #[cfg(target_has_atomic = "64")]
    fn epoch_deadline_callback(
        &mut self,
        callback: Box<dyn FnMut(StoreContextMut<T>) -> Result<UpdateDeadline> + Send + Sync>,
    ) {
        self.epoch_deadline_behavior = Some(callback);
    }

    fn get_epoch_deadline(&mut self) -> u64 {
        *self.vm_store_context.epoch_deadline.get_mut()
    }
}

impl<T: Default> Default for Store<T> {
    fn default() -> Store<T> {
        Store::new(&Engine::default(), T::default())
    }
}

impl<T: fmt::Debug> fmt::Debug for Store<T> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let inner = &**self.inner as *const StoreInner<T>;
        f.debug_struct("Store")
            .field("inner", &inner)
            .field("data", &self.inner.data)
            .finish()
    }
}

impl<T> Drop for Store<T> {
    fn drop(&mut self) {
        self.run_manual_drop_routines();

        // for documentation on this `unsafe`, see `into_data`.
        unsafe {
            ManuallyDrop::drop(&mut self.inner.data);
            ManuallyDrop::drop(&mut self.inner);
        }
    }
}

impl Drop for StoreOpaque {
    fn drop(&mut self) {
        // NB it's important that this destructor does not access `self.data`.
        // That is deallocated by `Drop for Store<T>` above.

        unsafe {
            let allocator = self.engine.allocator();
            let ondemand = OnDemandInstanceAllocator::default();
            let store_id = self.id();

            #[cfg(feature = "gc")]
            if let Some(gc_store) = self.gc_store.take() {
                let gc_alloc_index = gc_store.allocation_index;
                log::trace!("store {store_id:?} is deallocating GC heap {gc_alloc_index:?}");
                debug_assert!(self.engine.features().gc_types());
                let (mem_alloc_index, mem) =
                    allocator.deallocate_gc_heap(gc_alloc_index, gc_store.gc_heap);
                allocator.deallocate_memory(None, mem_alloc_index, mem);
            }

            for (id, instance) in self.instances.iter_mut() {
                log::trace!("store {store_id:?} is deallocating {id:?}");
                if let StoreInstanceKind::Dummy = instance.kind {
                    ondemand.deallocate_module(&mut instance.handle);
                } else {
                    allocator.deallocate_module(&mut instance.handle);
                }
            }

            #[cfg(feature = "component-model")]
            {
                for _ in 0..self.num_component_instances {
                    allocator.decrement_component_instance_count();
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::{get_fuel, refuel, set_fuel};
    use std::num::NonZeroU64;

    struct FuelTank {
        pub consumed_fuel: i64,
        pub reserve_fuel: u64,
        pub yield_interval: Option<NonZeroU64>,
    }

    impl FuelTank {
        fn new() -> Self {
            FuelTank {
                consumed_fuel: 0,
                reserve_fuel: 0,
                yield_interval: None,
            }
        }
        fn get_fuel(&self) -> u64 {
            get_fuel(self.consumed_fuel, self.reserve_fuel)
        }
        fn refuel(&mut self) -> bool {
            refuel(
                &mut self.consumed_fuel,
                &mut self.reserve_fuel,
                self.yield_interval,
            )
        }
        fn set_fuel(&mut self, fuel: u64) {
            set_fuel(
                &mut self.consumed_fuel,
                &mut self.reserve_fuel,
                self.yield_interval,
                fuel,
            );
        }
    }

    #[test]
    fn smoke() {
        let mut tank = FuelTank::new();
        tank.set_fuel(10);
        assert_eq!(tank.consumed_fuel, -10);
        assert_eq!(tank.reserve_fuel, 0);

        tank.yield_interval = NonZeroU64::new(10);
        tank.set_fuel(25);
        assert_eq!(tank.consumed_fuel, -10);
        assert_eq!(tank.reserve_fuel, 15);
    }

    #[test]
    fn does_not_lose_precision() {
        let mut tank = FuelTank::new();
        tank.set_fuel(u64::MAX);
        assert_eq!(tank.get_fuel(), u64::MAX);

        tank.set_fuel(i64::MAX as u64);
        assert_eq!(tank.get_fuel(), i64::MAX as u64);

        tank.set_fuel(i64::MAX as u64 + 1);
        assert_eq!(tank.get_fuel(), i64::MAX as u64 + 1);
    }

    #[test]
    fn yielding_does_not_lose_precision() {
        let mut tank = FuelTank::new();

        tank.yield_interval = NonZeroU64::new(10);
        tank.set_fuel(u64::MAX);
        assert_eq!(tank.get_fuel(), u64::MAX);
        assert_eq!(tank.consumed_fuel, -10);
        assert_eq!(tank.reserve_fuel, u64::MAX - 10);

        tank.yield_interval = NonZeroU64::new(u64::MAX);
        tank.set_fuel(u64::MAX);
        assert_eq!(tank.get_fuel(), u64::MAX);
        assert_eq!(tank.consumed_fuel, -i64::MAX);
        assert_eq!(tank.reserve_fuel, u64::MAX - (i64::MAX as u64));

        tank.yield_interval = NonZeroU64::new((i64::MAX as u64) + 1);
        tank.set_fuel(u64::MAX);
        assert_eq!(tank.get_fuel(), u64::MAX);
        assert_eq!(tank.consumed_fuel, -i64::MAX);
        assert_eq!(tank.reserve_fuel, u64::MAX - (i64::MAX as u64));
    }

    #[test]
    fn refueling() {
        // It's possible to fuel to have consumed over the limit as some instructions can consume
        // multiple units of fuel at once. Refueling should be strict in it's consumption and not
        // add more fuel than there is.
        let mut tank = FuelTank::new();

        tank.yield_interval = NonZeroU64::new(10);
        tank.reserve_fuel = 42;
        tank.consumed_fuel = 4;
        assert!(tank.refuel());
        assert_eq!(tank.reserve_fuel, 28);
        assert_eq!(tank.consumed_fuel, -10);

        tank.yield_interval = NonZeroU64::new(1);
        tank.reserve_fuel = 8;
        tank.consumed_fuel = 4;
        assert_eq!(tank.get_fuel(), 4);
        assert!(tank.refuel());
        assert_eq!(tank.reserve_fuel, 3);
        assert_eq!(tank.consumed_fuel, -1);
        assert_eq!(tank.get_fuel(), 4);

        tank.yield_interval = NonZeroU64::new(10);
        tank.reserve_fuel = 3;
        tank.consumed_fuel = 4;
        assert_eq!(tank.get_fuel(), 0);
        assert!(!tank.refuel());
        assert_eq!(tank.reserve_fuel, 3);
        assert_eq!(tank.consumed_fuel, 4);
        assert_eq!(tank.get_fuel(), 0);
    }
}
