//! Runtime support for the Component Model Async ABI.
//!
//! This module and its submodules provide host runtime support for Component
//! Model Async features such as async-lifted exports, async-lowered imports,
//! streams, futures, and related intrinsics.  See [the Async
//! Explainer](https://github.com/WebAssembly/component-model/blob/main/design/mvp/Async.md)
//! for a high-level overview.
//!
//! At the core of this support is an event loop which schedules and switches
//! between guest tasks and any host tasks they create.  Each
//! `ComponentInstance` will have at most one event loop running at any given
//! time, and that loop may be suspended and resumed by the host embedder using
//! e.g. `Instance::run_concurrent`.  The `ComponentInstance::poll_until`
//! function contains the loop itself, while the
//! `ComponentInstance::concurrent_state` field holds its state.
//!
//! # Public API Overview
//!
//! ## Top-level API (e.g. kicking off host->guest calls and driving the event loop)
//!
//! - `[Typed]Func::call_concurrent`: Start a host->guest call to an
//! async-lifted or sync-lifted import, creating a guest task.
//!
//! - `Instance::run_concurrent`: Run the event loop for the specified instance,
//! allowing any and all tasks belonging to that instance to make progress.
//!
//! - `Instance::spawn`: Run a background task as part of the event loop for the
//! specified instance.
//!
//! - `Instance::{future,stream}`: Create a new Component Model `future` or
//! `stream`; the read end may be passed to the guest.
//!
//! - `{Future,Stream}Reader::read` and `{Future,Stream}Writer::write`: read
//! from or write to a future or stream, respectively.
//!
//! ## Host Task API (e.g. implementing concurrent host functions and background tasks)
//!
//! - `LinkerInstance::func_wrap_concurrent`: Register a concurrent host
//! function with the linker.  That function will take an `Accessor` as its
//! first parameter, which provides access to the store and instance between
//! (but not across) await points.
//!
//! - `Accessor::with`: Access the store, its associated data, and the current
//! instance.
//!
//! - `Accessor::spawn`: Run a background task as part of the event loop for the
//! specified instance.  This is equivalent to `Instance::spawn` but more
//! convenient to use in host functions.

use crate::component::func::{self, Func, Options};
use crate::component::{Component, ComponentInstanceId, HasData, HasSelf, Instance};
use crate::fiber::{self, StoreFiber, StoreFiberYield};
use crate::store::{StoreInner, StoreOpaque, StoreToken};
use crate::vm::component::{CallContext, InstanceFlags, ResourceTables};
use crate::vm::{SendSyncPtr, VMFuncRef, VMMemoryDefinition, VMStore};
use crate::{AsContext, AsContextMut, StoreContext, StoreContextMut, ValRaw};
use anyhow::{Context as _, Result, anyhow, bail};
use error_contexts::{GlobalErrorContextRefCount, LocalErrorContextRefCount};
use futures::channel::oneshot;
use futures::future::{self, Either, FutureExt};
use futures::stream::{FuturesUnordered, StreamExt};
use futures_and_streams::{FlatAbi, ReturnCode, StreamFutureState, TableIndex, TransmitHandle};
use states::StateTable;
use std::any::Any;
use std::borrow::ToOwned;
use std::boxed::Box;
use std::cell::UnsafeCell;
use std::collections::{BTreeMap, BTreeSet, HashMap, HashSet};
use std::fmt;
use std::future::Future;
use std::marker::PhantomData;
use std::mem::{self, ManuallyDrop, MaybeUninit};
use std::ops::DerefMut;
use std::pin::{Pin, pin};
use std::ptr::{self, NonNull};
use std::slice;
use std::sync::Mutex;
use std::task::{Context, Poll, Waker};
use std::vec::Vec;
use table::{Table, TableDebug, TableError, TableId};
use wasmtime_environ::PrimaryMap;
use wasmtime_environ::component::{
    CanonicalOptions, CanonicalOptionsDataModel, ExportIndex, MAX_FLAT_PARAMS, MAX_FLAT_RESULTS,
    OptionsIndex, PREPARE_ASYNC_NO_RESULT, PREPARE_ASYNC_WITH_RESULT,
    RuntimeComponentInstanceIndex, StringEncoding, TypeComponentGlobalErrorContextTableIndex,
    TypeComponentLocalErrorContextTableIndex, TypeFutureTableIndex, TypeStreamTableIndex,
    TypeTupleIndex,
};

pub use abort::AbortHandle;
pub use futures_and_streams::{
    ErrorContext, FutureReader, FutureWriter, GuardedFutureReader, GuardedFutureWriter,
    GuardedStreamReader, GuardedStreamWriter, ReadBuffer, StreamReader, StreamWriter, VecBuffer,
    WriteBuffer,
};
pub(crate) use futures_and_streams::{
    ResourcePair, lower_error_context_to_index, lower_future_to_index, lower_stream_to_index,
};

mod abort;
mod error_contexts;
mod futures_and_streams;
mod states;
mod table;
pub(crate) mod tls;

/// Constant defined in the Component Model spec to indicate that the async
/// intrinsic (e.g. `future.write`) has not yet completed.
const BLOCKED: u32 = 0xffff_ffff;

/// Corresponds to `CallState` in the upstream spec.
#[derive(Clone, Copy, Eq, PartialEq, Debug)]
pub enum Status {
    Starting = 0,
    Started = 1,
    Returned = 2,
    StartCancelled = 3,
    ReturnCancelled = 4,
}

impl Status {
    /// Packs this status and the optional `waitable` provided into a 32-bit
    /// result that the canonical ABI requires.
    ///
    /// The low 4 bits are reserved for the status while the upper 28 bits are
    /// the waitable, if present.
    pub fn pack(self, waitable: Option<u32>) -> u32 {
        assert!(matches!(self, Status::Returned) == waitable.is_none());
        let waitable = waitable.unwrap_or(0);
        assert!(waitable < (1 << 28));
        (waitable << 4) | (self as u32)
    }
}

/// Corresponds to `EventCode` in the Component Model spec, plus related payload
/// data.
#[derive(Clone, Copy, Debug)]
enum Event {
    None,
    Cancelled,
    Subtask {
        status: Status,
    },
    StreamRead {
        code: ReturnCode,
        pending: Option<(TypeStreamTableIndex, u32)>,
    },
    StreamWrite {
        code: ReturnCode,
        pending: Option<(TypeStreamTableIndex, u32)>,
    },
    FutureRead {
        code: ReturnCode,
        pending: Option<(TypeFutureTableIndex, u32)>,
    },
    FutureWrite {
        code: ReturnCode,
        pending: Option<(TypeFutureTableIndex, u32)>,
    },
}

impl Event {
    /// Lower this event to core Wasm integers for delivery to the guest.
    ///
    /// Note that the waitable handle, if any, is assumed to be lowered
    /// separately.
    fn parts(self) -> (u32, u32) {
        const EVENT_NONE: u32 = 0;
        const EVENT_SUBTASK: u32 = 1;
        const EVENT_STREAM_READ: u32 = 2;
        const EVENT_STREAM_WRITE: u32 = 3;
        const EVENT_FUTURE_READ: u32 = 4;
        const EVENT_FUTURE_WRITE: u32 = 5;
        const EVENT_CANCELLED: u32 = 6;
        match self {
            Event::None => (EVENT_NONE, 0),
            Event::Cancelled => (EVENT_CANCELLED, 0),
            Event::Subtask { status } => (EVENT_SUBTASK, status as u32),
            Event::StreamRead { code, .. } => (EVENT_STREAM_READ, code.encode()),
            Event::StreamWrite { code, .. } => (EVENT_STREAM_WRITE, code.encode()),
            Event::FutureRead { code, .. } => (EVENT_FUTURE_READ, code.encode()),
            Event::FutureWrite { code, .. } => (EVENT_FUTURE_WRITE, code.encode()),
        }
    }
}

/// Corresponds to `CallbackCode` in the spec.
mod callback_code {
    pub const EXIT: u32 = 0;
    pub const YIELD: u32 = 1;
    pub const WAIT: u32 = 2;
    pub const POLL: u32 = 3;
}

/// A flag indicating that the callee is an async-lowered export.
///
/// This may be passed to the `async-start` intrinsic from a fused adapter.
const START_FLAG_ASYNC_CALLEE: u32 = wasmtime_environ::component::START_FLAG_ASYNC_CALLEE as u32;

/// Provides access to either store data (via the `get` method) or the store
/// itself (via [`AsContext`]/[`AsContextMut`]), as well as the component
/// instance to which the current host task belongs.
///
/// See [`Accessor::with`] for details.
pub struct Access<'a, T: 'static, D: HasData + ?Sized = HasSelf<T>> {
    accessor: &'a Accessor<T, D>,
    store: StoreContextMut<'a, T>,
}

impl<'a, T, D> Access<'a, T, D>
where
    D: HasData + ?Sized,
    T: 'static,
{
    /// Get mutable access to the store data.
    pub fn data_mut(&mut self) -> &mut T {
        self.store.data_mut()
    }

    /// Get mutable access to the store data.
    pub fn get(&mut self) -> D::Data<'_> {
        let get_data = self.accessor.get_data;
        get_data(self.data_mut())
    }

    /// Spawn a background task.
    ///
    /// See [`Accessor::spawn`] for details.
    pub fn spawn(&mut self, task: impl AccessorTask<T, D, Result<()>>) -> AbortHandle
    where
        T: 'static,
    {
        self.accessor.instance.unwrap().spawn_with_accessor(
            self.store.as_context_mut(),
            self.accessor.clone_for_spawn(),
            task,
        )
    }

    /// Retrieve the component instance of the caller.
    pub fn instance(&self) -> Instance {
        self.accessor.instance()
    }
}

impl<'a, T, D> AsContext for Access<'a, T, D>
where
    D: HasData + ?Sized,
    T: 'static,
{
    type Data = T;

    fn as_context(&self) -> StoreContext<'_, T> {
        self.store.as_context()
    }
}

impl<'a, T, D> AsContextMut for Access<'a, T, D>
where
    D: HasData + ?Sized,
    T: 'static,
{
    fn as_context_mut(&mut self) -> StoreContextMut<'_, T> {
        self.store.as_context_mut()
    }
}

/// Provides scoped mutable access to store data in the context of a concurrent
/// host task future.
///
/// This allows multiple host task futures to execute concurrently and access
/// the store between (but not across) `await` points.
///
/// # Rationale
///
/// This structure is sort of like `&mut T` plus a projection from `&mut T` to
/// `D::Data<'_>`. The problem this is solving, however, is that it does not
/// literally store these values. The basic problem is that when a concurrent
/// host future is being polled it has access to `&mut T` (and the whole
/// `Store`) but when it's not being polled it does not have access to these
/// values. This reflects how the store is only ever polling one future at a
/// time so the store is effectively being passed between futures.
///
/// Rust's `Future` trait, however, has no means of passing a `Store`
/// temporarily between futures. The [`Context`](std::task::Context) type does
/// not have the ability to attach arbitrary information to it at this time.
/// This type, [`Accessor`], is used to bridge this expressivity gap.
///
/// The [`Accessor`] type here represents the ability to acquire, temporarily in
/// a synchronous manner, the current store. The [`Accessor::with`] function
/// yields an [`Access`] which can be used to access [`StoreContextMut`], `&mut
/// T`, or `D::Data<'_>`. Note though that [`Accessor::with`] intentionally does
/// not take an `async` closure as its argument, instead it's a synchronous
/// closure which must complete during on run of `Future::poll`. This reflects
/// how the store is temporarily made available while a host future is being
/// polled.
///
/// # Implementation
///
/// This type does not actually store `&mut T` nor `StoreContextMut<T>`, and
/// this type additionally doesn't even have a lifetime parameter. This is
/// instead a representation of proof of the ability to acquire these while a
/// future is being polled. Wasmtime will, when it polls a host future,
/// configure ambient state such that the `Accessor` that a future closes over
/// will work and be able to access the store.
///
/// This has a number of implications for users such as:
///
/// * It's intentional that `Accessor` cannot be cloned, it needs to stay within
///   the lifetime of a single future.
/// * A futures is expected to, however, close over an `Accessor` and keep it
///   alive probably for the duration of the entire future.
/// * Different host futures will be given different `Accessor`s, and that's
///   intentional.
/// * The `Accessor` type is `Send` and `Sync` irrespective of `T` which
///   alleviates some otherwise required bounds to be written down.
///
/// # Using `Accessor` in `Drop`
///
/// The methods on `Accessor` are only expected to work in the context of
/// `Future::poll` and are not guaranteed to work in `Drop`. This is because a
/// host future can be dropped at any time throughout the system and Wasmtime
/// store context is not necessarily available at that time. It's recommended to
/// not use `Accessor` methods in anything connected to a `Drop` implementation
/// as they will panic and have unintended results. If you run into this though
/// feel free to file an issue on the Wasmtime repository.
pub struct Accessor<T: 'static, D = HasSelf<T>>
where
    D: HasData + ?Sized,
{
    token: StoreToken<T>,
    get_data: fn(&mut T) -> D::Data<'_>,
    instance: Option<Instance>,
}

/// A helper trait to take any type of accessor-with-data in functions.
///
/// This trait is similar to [`AsContextMut`] except that it's used when
/// working with an [`Accessor`] instead of a [`StoreContextMut`]. The
/// [`Accessor`] is the main type used in concurrent settings and is passed to
/// functions such as [`Func::call_concurrent`] or [`FutureWriter::write`].
///
/// This trait is implemented for [`Accessor`] and `&T` where `T` implements
/// this trait. This effectively means that regardless of the `D` in
/// `Accessor<T, D>` it can still be passed to a function which just needs a
/// store accessor.
///
/// Acquiring an [`Accessor`] can be done through [`Instance::run_concurrent`]
/// for example or in a host function through
/// [`Linker::func_wrap_concurrent`](crate::component::Linker::func_wrap_concurrent).
pub trait AsAccessor {
    /// The `T` in `Store<T>` that this accessor refers to.
    type Data: 'static;

    /// The `D` in `Accessor<T, D>`, or the projection out of
    /// `Self::Data`.
    type AccessorData: HasData + ?Sized;

    /// Returns the accessor that this is referring to.
    fn as_accessor(&self) -> &Accessor<Self::Data, Self::AccessorData>;
}

impl<T: AsAccessor + ?Sized> AsAccessor for &T {
    type Data = T::Data;
    type AccessorData = T::AccessorData;

    fn as_accessor(&self) -> &Accessor<Self::Data, Self::AccessorData> {
        T::as_accessor(self)
    }
}

impl<T, D: HasData + ?Sized> AsAccessor for Accessor<T, D> {
    type Data = T;
    type AccessorData = D;

    fn as_accessor(&self) -> &Accessor<T, D> {
        self
    }
}

// Note that it is intentional at this time that `Accessor` does not actually
// store `&mut T` or anything similar. This distinctly enables the `Accessor`
// structure to be both `Send` and `Sync` regardless of what `T` is (or `D` for
// that matter). This is used to ergonomically simplify bindings where the
// majority of the time `Accessor` is closed over in a future which then needs
// to be `Send` and `Sync`. To avoid needing to write `T: Send` everywhere (as
// you already have to write `T: 'static`...) it helps to avoid this.
//
// Note as well that `Accessor` doesn't actually store its data at all. Instead
// it's more of a "proof" of what can be accessed from TLS. API design around
// `Accessor` and functions like `Linker::func_wrap_concurrent` are
// intentionally made to ensure that `Accessor` is ideally only used in the
// context that TLS variables are actually set. For example host functions are
// given `&Accessor`, not `Accessor`, and this prevents them from persisting
// the value outside of a future. Within the future the TLS variables are all
// guaranteed to be set while the future is being polled.
//
// Finally though this is not an ironclad guarantee, but nor does it need to be.
// The TLS APIs are designed to panic or otherwise model usage where they're
// called recursively or similar. It's hoped that code cannot be constructed to
// actually hit this at runtime but this is not a safety requirement at this
// time.
const _: () = {
    const fn assert<T: Send + Sync>() {}
    assert::<Accessor<UnsafeCell<u32>>>();
};

impl<T> Accessor<T> {
    /// Creates a new `Accessor` backed by the specified functions.
    ///
    /// - `get`: used to retrieve the store
    ///
    /// - `get_data`: used to "project" from the store's associated data to
    /// another type (e.g. a field of that data or a wrapper around it).
    ///
    /// - `spawn`: used to queue spawned background tasks to be run later
    ///
    /// - `instance`: used to access the `Instance` to which this `Accessor`
    /// (and the future which closes over it) belongs
    pub(crate) fn new(token: StoreToken<T>, instance: Option<Instance>) -> Self {
        Self {
            token,
            get_data: |x| x,
            instance,
        }
    }
}

impl<T, D> Accessor<T, D>
where
    D: HasData + ?Sized,
{
    /// Run the specified closure, passing it mutable access to the store.
    ///
    /// This function is one of the main building blocks of the [`Accessor`]
    /// type. This yields synchronous, blocking, access to store via an
    /// [`Access`]. The [`Access`] implements [`AsContextMut`] in addition to
    /// providing the ability to access `D` via [`Access::get`]. Note that the
    /// `fun` here is given only temporary access to the store and `T`/`D`
    /// meaning that the return value `R` here is not allowed to capture borrows
    /// into the two. If access is needed to data within `T` or `D` outside of
    /// this closure then it must be `clone`d out, for example.
    ///
    /// # Panics
    ///
    /// This function will panic if it is call recursively with any other
    /// accessor already in scope. For example if `with` is called within `fun`,
    /// then this function will panic. It is up to the embedder to ensure that
    /// this does not happen.
    pub fn with<R>(&self, fun: impl FnOnce(Access<'_, T, D>) -> R) -> R {
        tls::get(|vmstore| {
            fun(Access {
                store: self.token.as_context_mut(vmstore),
                accessor: self,
            })
        })
    }

    /// Changes this accessor to access `D2` instead of the current type
    /// parameter `D`.
    ///
    /// This changes the underlying data access from `T` to `D2::Data<'_>`.
    ///
    /// Note that this is not a public or recommended API because it's easy to
    /// cause panics with this by having two `Accessor` values live at the same
    /// time. The returned `Accessor` does not refer to this `Accessor` meaning
    /// that both can be used. You could, for example, call `Accessor::with`
    /// simultaneously on both. That would cause a panic though.
    ///
    /// In short while there's nothing unsafe about this it's a footgun. It's
    /// here for bindings generation where the provided accessor is transformed
    /// into a new accessor and then this returned accessor is passed to
    /// implementations.
    ///
    /// Note that one possible fix for this would be a lifetime parameter on
    /// `Accessor` itself so the returned value could borrow from the original
    /// value (or this could be `self`-by-value instead of `&mut self`) but in
    /// attempting that it was found to be a bit too onerous in terms of
    /// plumbing things around without a whole lot of benefit.
    ///
    /// In short, this works, but must be treated with care. The current main
    /// user, bindings generation, treats this with care.
    #[doc(hidden)]
    pub fn with_data<D2: HasData>(&self, get_data: fn(&mut T) -> D2::Data<'_>) -> Accessor<T, D2> {
        Accessor {
            token: self.token,
            get_data,
            instance: self.instance,
        }
    }

    /// Spawn a background task which will receive an `&Accessor<T, D>` and
    /// run concurrently with any other tasks in progress for the current
    /// instance.
    ///
    /// This is particularly useful for host functions which return a `stream`
    /// or `future` such that the code to write to the write end of that
    /// `stream` or `future` must run after the function returns.
    ///
    /// The returned [`AbortHandle`] may be used to cancel the task.
    ///
    /// # Panics
    ///
    /// Panics if called within a closure provided to the [`Accessor::with`]
    /// function. This can only be called outside an active invocation of
    /// [`Accessor::with`].
    pub fn spawn(&self, task: impl AccessorTask<T, D, Result<()>>) -> AbortHandle
    where
        T: 'static,
    {
        let instance = self.instance.unwrap();
        let accessor = self.clone_for_spawn();
        self.with(|mut access| {
            instance.spawn_with_accessor(access.as_context_mut(), accessor, task)
        })
    }

    /// Retrieve the component instance of the caller.
    pub fn instance(&self) -> Instance {
        self.instance.unwrap()
    }

    fn clone_for_spawn(&self) -> Self {
        Self {
            token: self.token,
            get_data: self.get_data,
            instance: self.instance,
        }
    }
}

/// Represents a task which may be provided to `Accessor::spawn`,
/// `Accessor::forward`, or `Instance::spawn`.
// TODO: Replace this with `std::ops::AsyncFnOnce` when that becomes a viable
// option.
//
// `AsyncFnOnce` is still nightly-only in latest stable Rust version as of this
// writing (1.84.1), and even with 1.85.0-beta it's not possible to specify
// e.g. `Send` and `Sync` bounds on the `Future` type returned by an
// `AsyncFnOnce`.  Also, using `F: Future<Output = Result<()>> + Send + Sync,
// FN: FnOnce(&Accessor<T>) -> F + Send + Sync + 'static` fails with a type
// mismatch error when we try to pass it an async closure (e.g. `async move |_|
// { ... }`).  So this seems to be the best we can do for the time being.
pub trait AccessorTask<T, D, R>: Send + 'static
where
    D: HasData + ?Sized,
{
    /// Run the task.
    fn run(self, accessor: &Accessor<T, D>) -> impl Future<Output = R> + Send;
}

/// Represents the state of a waitable handle.
#[derive(Debug)]
enum WaitableState {
    /// Represents a host task handle.
    HostTask,
    /// Represents a guest task handle.
    GuestTask,
    /// Represents a stream handle.
    Stream(TypeStreamTableIndex, StreamFutureState),
    /// Represents a future handle.
    Future(TypeFutureTableIndex, StreamFutureState),
    /// Represents a waitable-set handle.
    Set,
}

/// Represents parameter and result metadata for the caller side of a
/// guest->guest call orchestrated by a fused adapter.
enum CallerInfo {
    /// Metadata for a call to an async-lowered import
    Async {
        params: Vec<ValRaw>,
        has_result: bool,
    },
    /// Metadata for a call to an sync-lowered import
    Sync {
        params: Vec<ValRaw>,
        result_count: u32,
    },
}

/// Indicates how a guest task is waiting on a waitable set.
enum WaitMode {
    /// The guest task is waiting using `task.wait`
    Fiber(StoreFiber<'static>),
    /// The guest task is waiting via a callback declared as part of an
    /// async-lifted export.
    Callback(RuntimeComponentInstanceIndex),
}

/// Represents the reason a fiber is suspending itself.
#[derive(Debug)]
enum SuspendReason {
    /// The fiber is waiting for an event to be delivered to the specified
    /// waitable set or task.
    Waiting {
        set: TableId<WaitableSet>,
        task: TableId<GuestTask>,
    },
    /// The fiber has finished handling its most recent work item and is waiting
    /// for another (or to be dropped if it is no longer needed).
    NeedWork,
    /// The fiber is yielding and should be resumed once other tasks have had a
    /// chance to run.
    Yielding { task: TableId<GuestTask> },
}

/// Represents a pending call into guest code for a given guest task.
enum GuestCallKind {
    /// Indicates there's an event to deliver to the task, possibly related to a
    /// waitable set the task has been waiting on or polling.
    DeliverEvent {
        /// The (sub-)component instance in which the task has most recently
        /// been executing.
        ///
        /// Note that this might not be the same as the instance the guest task
        /// started executing in given that one or more synchronous guest->guest
        /// calls may have occurred involving multiple instances.
        instance: RuntimeComponentInstanceIndex,
        /// The waitable set the event belongs to, if any.
        ///
        /// If this is `None` the event will be waiting in the
        /// `GuestTask::event` field for the task.
        set: Option<TableId<WaitableSet>>,
    },
    /// Indicates that a new guest task call is pending and may be executed
    /// using the specified closure.
    Start(Box<dyn FnOnce(&mut dyn VMStore, Instance) -> Result<()> + Send + Sync>),
}

impl fmt::Debug for GuestCallKind {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::DeliverEvent { instance, set } => f
                .debug_struct("DeliverEvent")
                .field("instance", instance)
                .field("set", set)
                .finish(),
            Self::Start(_) => f.debug_tuple("Start").finish(),
        }
    }
}

/// Represents a pending call into guest code for a given guest task.
#[derive(Debug)]
struct GuestCall {
    task: TableId<GuestTask>,
    kind: GuestCallKind,
}

impl GuestCall {
    /// Returns whether or not the call is ready to run.
    ///
    /// A call will not be ready to run if either:
    ///
    /// - the (sub-)component instance to be called has already been entered and
    /// cannot be reentered until an in-progress call completes
    ///
    /// - the call is for a not-yet started task and the (sub-)component
    /// instance to be called has backpressure enabled
    fn is_ready(&self, state: &mut ConcurrentState) -> Result<bool> {
        let task_instance = state.get(self.task)?.instance;
        let state = state.instance_state(task_instance);
        let ready = match &self.kind {
            GuestCallKind::DeliverEvent { .. } => !state.do_not_enter,
            GuestCallKind::Start(_) => !(state.do_not_enter || state.backpressure),
        };
        log::trace!(
            "call {self:?} ready? {ready} (do_not_enter: {}; backpressure: {})",
            state.do_not_enter,
            state.backpressure
        );
        Ok(ready)
    }
}

/// Job to be run on a worker fiber.
enum WorkerItem {
    GuestCall(GuestCall),
    Function(Mutex<Box<dyn FnOnce(&mut dyn VMStore, Instance) -> Result<()> + Send>>),
}

/// Represents state related to an in-progress poll operation (e.g. `task.poll`
/// or `CallbackCode.POLL`).
#[derive(Debug)]
struct PollParams {
    /// Identifies the polling task.
    task: TableId<GuestTask>,
    /// The waitable set being polled.
    set: TableId<WaitableSet>,
    /// The (sub-)component instance in which the task has most recently been
    /// executing.
    ///
    /// Note that this might not be the same as the instance the guest task
    /// started executing in given that one or more synchronous guest->guest
    /// calls may have occurred involving multiple instances.
    instance: RuntimeComponentInstanceIndex,
}

/// Represents a pending work item to be handled by the event loop for a given
/// component instance.
enum WorkItem {
    /// A host task to be pushed to `ConcurrentState::futures`.
    PushFuture(Mutex<HostTaskFuture>),
    /// A fiber to resume.
    ResumeFiber(StoreFiber<'static>),
    /// A pending call into guest code for a given guest task.
    GuestCall(GuestCall),
    /// A pending `task.poll` or `CallbackCode.POLL` operation.
    Poll(PollParams),
    /// A job to run on a worker fiber.
    WorkerFunction(Mutex<Box<dyn FnOnce(&mut dyn VMStore, Instance) -> Result<()> + Send>>),
}

impl fmt::Debug for WorkItem {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::PushFuture(_) => f.debug_tuple("PushFuture").finish(),
            Self::ResumeFiber(_) => f.debug_tuple("ResumeFiber").finish(),
            Self::GuestCall(call) => f.debug_tuple("GuestCall").field(call).finish(),
            Self::Poll(params) => f.debug_tuple("Poll").field(params).finish(),
            Self::WorkerFunction(_) => f.debug_tuple("WorkerFunction").finish(),
        }
    }
}

impl ConcurrentState {
    fn instance_state(&mut self, instance: RuntimeComponentInstanceIndex) -> &mut InstanceState {
        self.instance_states.entry(instance).or_default()
    }

    fn push<V: Send + Sync + 'static>(&mut self, value: V) -> Result<TableId<V>, TableError> {
        self.table.push(value)
    }

    fn get<V: 'static>(&self, id: TableId<V>) -> Result<&V, TableError> {
        self.table.get(id)
    }

    fn get_mut<V: 'static>(&mut self, id: TableId<V>) -> Result<&mut V, TableError> {
        self.table.get_mut(id)
    }

    pub fn add_child<T, U>(
        &mut self,
        child: TableId<T>,
        parent: TableId<U>,
    ) -> Result<(), TableError> {
        self.table.add_child(child, parent)
    }

    pub fn remove_child<T, U>(
        &mut self,
        child: TableId<T>,
        parent: TableId<U>,
    ) -> Result<(), TableError> {
        self.table.remove_child(child, parent)
    }

    fn delete<V: 'static>(&mut self, id: TableId<V>) -> Result<V, TableError> {
        self.table.delete(id)
    }

    fn push_future(&mut self, future: HostTaskFuture) {
        // Note that we can't directly push to `ConcurrentState::futures` here
        // since this may be called from a future that's being polled inside
        // `Self::poll_until`, which temporarily removes the `FuturesUnordered`
        // so it has exclusive access while polling it.  Therefore, we push a
        // work item to the "high priority" queue, which will actually push to
        // `ConcurrentState::futures` later.
        self.push_high_priority(WorkItem::PushFuture(Mutex::new(future)));
    }

    fn push_high_priority(&mut self, item: WorkItem) {
        log::trace!("push high priority: {item:?}");
        self.high_priority.push(item);
    }

    fn push_low_priority(&mut self, item: WorkItem) {
        log::trace!("push low priority: {item:?}");
        self.low_priority.push(item);
    }

    /// Determine whether the instance associated with the specified guest task
    /// may be entered (i.e. is not already on the async call stack).
    ///
    /// This is an additional check on top of the "may_enter" instance flag;
    /// it's needed because async-lifted exports with callback functions must
    /// not call their own instances directly or indirectly, and due to the
    /// "stackless" nature of callback-enabled guest tasks this may happen even
    /// if there are no activation records on the stack (i.e. the "may_enter"
    /// field is `true`) for that instance.
    fn may_enter(&mut self, mut guest_task: TableId<GuestTask>) -> bool {
        let guest_instance = self.get(guest_task).unwrap().instance;

        // Walk the task tree back to the root, looking for potential
        // reentrance.
        //
        // TODO: This could be optimized by maintaining a per-`GuestTask` bitset
        // such that each bit represents and instance which has been entered by
        // that task or an ancestor of that task, in which case this would be a
        // constant time check.
        loop {
            match &self.get_mut(guest_task).unwrap().caller {
                Caller::Host { .. } => break true,
                Caller::Guest { task, instance } => {
                    if *instance == guest_instance {
                        break false;
                    } else {
                        guest_task = *task;
                    }
                }
            }
        }
    }

    /// Handle the `CallbackCode` returned from an async-lifted export or its
    /// callback.
    ///
    /// If `initial_call` is `true`, then the code was received from the
    /// async-lifted export; otherwise, it was received from its callback.
    fn handle_callback_code(
        &mut self,
        guest_task: TableId<GuestTask>,
        runtime_instance: RuntimeComponentInstanceIndex,
        code: u32,
        initial_call: bool,
    ) -> Result<()> {
        let (code, set) = unpack_callback_code(code);

        log::trace!("received callback code from {guest_task:?}: {code} (set: {set})");

        let task = self.get_mut(guest_task)?;

        if task.lift_result.is_some() {
            if code == callback_code::EXIT {
                return Err(anyhow!(crate::Trap::NoAsyncResult));
            }
            if initial_call {
                // Notify any current or future waiters that this subtask has
                // started.
                Waitable::Guest(guest_task).set_event(
                    self,
                    Some(Event::Subtask {
                        status: Status::Started,
                    }),
                )?;
            }
        }

        let get_set = |instance: &mut Self, handle| {
            if handle == 0 {
                bail!("invalid waitable-set handle");
            }

            let (set, WaitableState::Set) =
                instance.waitable_tables[runtime_instance].get_mut_by_index(handle)?
            else {
                bail!("invalid waitable-set handle");
            };

            Ok(TableId::<WaitableSet>::new(set))
        };

        match code {
            callback_code::EXIT => {
                let task = self.get_mut(guest_task)?;
                match &task.caller {
                    Caller::Host {
                        remove_task_automatically,
                        ..
                    } => {
                        if *remove_task_automatically {
                            log::trace!("handle_callback_code will delete task {guest_task:?}");
                            Waitable::Guest(guest_task).delete_from(self)?;
                        }
                    }
                    Caller::Guest { .. } => {
                        task.exited = true;
                        task.callback = None;
                    }
                }
            }
            callback_code::YIELD => {
                // Push this task onto the "low priority" queue so it runs after
                // any other tasks have had a chance to run.
                let task = self.get_mut(guest_task)?;
                assert!(task.event.is_none());
                task.event = Some(Event::None);
                self.push_low_priority(WorkItem::GuestCall(GuestCall {
                    task: guest_task,
                    kind: GuestCallKind::DeliverEvent {
                        instance: runtime_instance,
                        set: None,
                    },
                }));
            }
            callback_code::WAIT | callback_code::POLL => {
                let set = get_set(self, set)?;

                if self.get_mut(guest_task)?.event.is_some() || !self.get_mut(set)?.ready.is_empty()
                {
                    // An event is immediately available; deliver it ASAP.
                    self.push_high_priority(WorkItem::GuestCall(GuestCall {
                        task: guest_task,
                        kind: GuestCallKind::DeliverEvent {
                            instance: runtime_instance,
                            set: Some(set),
                        },
                    }));
                } else {
                    // No event is immediately available.
                    match code {
                        callback_code::POLL => {
                            // We're polling, so just yield and check whether an
                            // event has arrived after that.
                            self.push_low_priority(WorkItem::Poll(PollParams {
                                task: guest_task,
                                instance: runtime_instance,
                                set,
                            }));
                        }
                        callback_code::WAIT => {
                            // We're waiting, so register to be woken up when an
                            // event is published for this waitable set.
                            //
                            // Here we also set `GuestTask::wake_on_cancel`
                            // which allows `subtask.cancel` to interrupt the
                            // wait.
                            let old = self.get_mut(guest_task)?.wake_on_cancel.replace(set);
                            assert!(old.is_none());
                            let old = self
                                .get_mut(set)?
                                .waiting
                                .insert(guest_task, WaitMode::Callback(runtime_instance));
                            assert!(old.is_none());
                        }
                        _ => unreachable!(),
                    }
                }
            }
            _ => bail!("unsupported callback code: {code}"),
        }

        Ok(())
    }

    /// Record that we're about to enter a (sub-)component instance which does
    /// not support more than one concurrent, stackful activation, meaning it
    /// cannot be entered again until the next call returns.
    fn enter_instance(&mut self, instance: RuntimeComponentInstanceIndex) {
        self.instance_state(instance).do_not_enter = true;
    }

    /// Record that we've exited a (sub-)component instance previously entered
    /// with `Self::enter_instance` and then calls `Self::partition_pending`.
    /// See the documentation for the latter for details.
    fn exit_instance(&mut self, instance: RuntimeComponentInstanceIndex) -> Result<()> {
        self.instance_state(instance).do_not_enter = false;
        self.partition_pending(instance)
    }

    /// Iterate over `InstanceState::pending`, moving any ready items into the
    /// "high priority" work item queue.
    ///
    /// See `GuestCall::is_ready` for details.
    fn partition_pending(&mut self, instance: RuntimeComponentInstanceIndex) -> Result<()> {
        for (task, kind) in mem::take(&mut self.instance_state(instance).pending).into_iter() {
            let call = GuestCall { task, kind };
            if call.is_ready(self)? {
                self.push_high_priority(WorkItem::GuestCall(call));
            } else {
                self.instance_state(instance)
                    .pending
                    .insert(call.task, call.kind);
            }
        }

        Ok(())
    }

    /// Get the next pending event for the specified task and (optional)
    /// waitable set, along with the waitable handle if applicable.
    fn get_event(
        &mut self,
        guest_task: TableId<GuestTask>,
        instance: RuntimeComponentInstanceIndex,
        set: Option<TableId<WaitableSet>>,
    ) -> Result<Option<(Event, Option<(Waitable, u32)>)>> {
        Ok(
            if let Some(event) = self.get_mut(guest_task)?.event.take() {
                log::trace!("deliver event {event:?} to {guest_task:?}");

                Some((event, None))
            } else if let Some((set, waitable)) = set
                .and_then(|set| {
                    self.get_mut(set)
                        .map(|v| v.ready.pop_first().map(|v| (set, v)))
                        .transpose()
                })
                .transpose()?
            {
                let event = waitable.common(self)?.event.take().unwrap();

                log::trace!(
                    "deliver event {event:?} to {guest_task:?} for {waitable:?}; set {set:?}"
                );

                let entry = self.waitable_tables[instance].get_mut_by_rep(waitable.rep());
                let Some((
                    handle,
                    WaitableState::HostTask
                    | WaitableState::GuestTask
                    | WaitableState::Stream(..)
                    | WaitableState::Future(..),
                )) = entry
                else {
                    bail!("handle not found for waitable rep {waitable:?} instance {instance:?}");
                };

                waitable.on_delivery(self, event);

                Some((event, Some((waitable, handle))))
            } else {
                None
            },
        )
    }

    /// Implements the `backpressure.set` intrinsic.
    pub(crate) fn backpressure_set(
        &mut self,
        caller_instance: RuntimeComponentInstanceIndex,
        enabled: u32,
    ) -> Result<()> {
        let state = self.instance_state(caller_instance);
        let old = state.backpressure;
        let new = enabled != 0;
        state.backpressure = new;

        if old && !new {
            // Backpressure was previously enabled and is now disabled; move any
            // newly-eligible guest calls to the "high priority" queue.
            self.partition_pending(caller_instance)?;
        }

        Ok(())
    }

    /// Implements the `waitable-set.new` intrinsic.
    pub(crate) fn waitable_set_new(
        &mut self,
        caller_instance: RuntimeComponentInstanceIndex,
    ) -> Result<u32> {
        let set = self.push(WaitableSet::default())?;
        let handle = self.waitable_tables[caller_instance].insert(set.rep(), WaitableState::Set)?;
        log::trace!("new waitable set {set:?} (handle {handle})");
        Ok(handle)
    }

    /// Implements the `waitable-set.drop` intrinsic.
    pub(crate) fn waitable_set_drop(
        &mut self,
        caller_instance: RuntimeComponentInstanceIndex,
        set: u32,
    ) -> Result<()> {
        let (rep, WaitableState::Set) =
            self.waitable_tables[caller_instance].remove_by_index(set)?
        else {
            bail!("invalid waitable-set handle");
        };

        log::trace!("drop waitable set {rep} (handle {set})");

        let set = self.delete(TableId::<WaitableSet>::new(rep))?;

        if !set.waiting.is_empty() {
            bail!("cannot drop waitable set with waiters");
        }

        Ok(())
    }

    /// Implements the `waitable.join` intrinsic.
    pub(crate) fn waitable_join(
        &mut self,
        caller_instance: RuntimeComponentInstanceIndex,
        waitable_handle: u32,
        set_handle: u32,
    ) -> Result<()> {
        let waitable = Waitable::from_instance(self, caller_instance, waitable_handle)?;

        let set = if set_handle == 0 {
            None
        } else {
            let (set, WaitableState::Set) =
                self.waitable_tables[caller_instance].get_mut_by_index(set_handle)?
            else {
                bail!("invalid waitable-set handle");
            };

            Some(TableId::<WaitableSet>::new(set))
        };

        log::trace!(
            "waitable {waitable:?} (handle {waitable_handle}) join set {set:?} (handle {set_handle})",
        );

        waitable.join(self, set)
    }

    /// Implements the `subtask.drop` intrinsic.
    pub(crate) fn subtask_drop(
        &mut self,
        caller_instance: RuntimeComponentInstanceIndex,
        task_id: u32,
    ) -> Result<()> {
        self.waitable_join(caller_instance, task_id, 0)?;

        let (rep, state) = self.waitable_tables[caller_instance].remove_by_index(task_id)?;

        let (waitable, expected_caller_instance, delete) = match state {
            WaitableState::HostTask => {
                let id = TableId::<HostTask>::new(rep);
                let task = self.get(id)?;
                if task.abort_handle.is_some() {
                    bail!("cannot drop a subtask which has not yet resolved");
                }
                (Waitable::Host(id), task.caller_instance, true)
            }
            WaitableState::GuestTask => {
                let id = TableId::<GuestTask>::new(rep);
                let task = self.get(id)?;
                if task.lift_result.is_some() {
                    bail!("cannot drop a subtask which has not yet resolved");
                }
                if let Caller::Guest { instance, .. } = &task.caller {
                    (Waitable::Guest(id), *instance, task.exited)
                } else {
                    unreachable!()
                }
            }
            _ => bail!("invalid task handle: {task_id}"),
        };

        if waitable.take_event(self)?.is_some() {
            bail!("cannot drop a subtask with an undelivered event");
        }

        if delete {
            waitable.delete_from(self)?;
        }

        // Since waitables can neither be passed between instances nor forged,
        // this should never fail unless there's a bug in Wasmtime, but we check
        // here to be sure:
        assert_eq!(expected_caller_instance, caller_instance);
        log::trace!("subtask_drop {waitable:?} (handle {task_id})");
        Ok(())
    }

    /// Implements the `context.get` intrinsic.
    pub(crate) fn context_get(&mut self, slot: u32) -> Result<u32> {
        let task = self.guest_task.unwrap();
        let val = self.get(task)?.context[usize::try_from(slot).unwrap()];
        log::trace!("context_get {task:?} slot {slot} val {val:#x}");
        Ok(val)
    }

    /// Implements the `context.set` intrinsic.
    pub(crate) fn context_set(&mut self, slot: u32, val: u32) -> Result<()> {
        let task = self.guest_task.unwrap();
        log::trace!("context_set {task:?} slot {slot} val {val:#x}");
        self.get_mut(task)?.context[usize::try_from(slot).unwrap()] = val;
        Ok(())
    }

    fn options(&self, options: OptionsIndex) -> &CanonicalOptions {
        &self.component.env_component().options[options]
    }
}

impl Instance {
    /// Enable or disable concurrent state debugging mode for e.g. integration
    /// tests.
    ///
    /// This will avoid re-using deleted handles, making it easier to catch
    /// e.g. "use-after-delete" and "double-delete" errors.  It can also make
    /// reading trace output easier since it ensures handles are never
    /// repurposed.
    #[doc(hidden)]
    pub fn enable_concurrent_state_debug(&self, mut store: impl AsContextMut, enable: bool) {
        self.id()
            .get_mut(store.as_context_mut().0)
            .concurrent_state_mut()
            .table
            .enable_debug(enable);
        // TODO: do the same for the tables holding guest-facing handles
    }

    /// Assert that all the relevant tables and queues in the concurrent state
    /// for this instance are empty.
    ///
    /// This is for sanity checking in integration tests
    /// (e.g. `component-async-tests`) that the relevant state has been cleared
    /// after each test concludes.  This should help us catch leaks, e.g. guest
    /// tasks which haven't been deleted despite having completed and having
    /// been dropped by their supertasks.
    #[doc(hidden)]
    pub fn assert_concurrent_state_empty(&self, mut store: impl AsContextMut) {
        let state = self
            .id()
            .get_mut(store.as_context_mut().0)
            .concurrent_state_mut();
        assert!(state.table.is_empty(), "non-empty table: {:?}", state.table);
        assert!(state.high_priority.is_empty());
        assert!(state.low_priority.is_empty());
        assert!(state.guest_task.is_none());
        assert!(
            state
                .futures
                .get_mut()
                .unwrap()
                .as_ref()
                .unwrap()
                .is_empty()
        );
        assert!(
            state
                .waitable_tables
                .iter()
                .all(|(_, table)| table.is_empty())
        );
        assert!(
            state
                .instance_states
                .iter()
                .all(|(_, state)| state.pending.is_empty())
        );
        assert!(
            state
                .error_context_tables
                .iter()
                .all(|(_, table)| table.is_empty())
        );
        assert!(state.global_error_context_ref_counts.is_empty());
    }

    /// Run the specified closure `fun` to completion as part of this instance's
    /// event loop.
    ///
    /// Like [`Self::run`], this will run `fun` as part of this instance's event
    /// loop until it yields a result _or_ there are no more tasks to run.
    /// Unlike [`Self::run`], `fun` is provided an [`Accessor`], which provides
    /// controlled access to the `Store` and its data.
    ///
    /// This function can be used to invoke [`Func::call_concurrent`] for
    /// example within the async closure provided here.
    ///
    /// # Example
    ///
    /// ```
    /// # use {
    /// #   anyhow::{Result},
    /// #   wasmtime::{
    /// #     component::{ Component, Linker, Resource, ResourceTable},
    /// #     Config, Engine, Store
    /// #   },
    /// # };
    /// #
    /// # struct MyResource(u32);
    /// # struct Ctx { table: ResourceTable }
    /// #
    /// # async fn foo() -> Result<()> {
    /// # let mut config = Config::new();
    /// # let engine = Engine::new(&config)?;
    /// # let mut store = Store::new(&engine, Ctx { table: ResourceTable::new() });
    /// # let mut linker = Linker::new(&engine);
    /// # let component = Component::new(&engine, "")?;
    /// # let instance = linker.instantiate_async(&mut store, &component).await?;
    /// # let foo = instance.get_typed_func::<(Resource<MyResource>,), (Resource<MyResource>,)>(&mut store, "foo")?;
    /// # let bar = instance.get_typed_func::<(u32,), ()>(&mut store, "bar")?;
    /// instance.run_concurrent(&mut store, async |accessor| -> wasmtime::Result<_> {
    ///    let resource = accessor.with(|mut access| access.get().table.push(MyResource(42)))?;
    ///    let (another_resource,) = foo.call_concurrent(accessor, (resource,)).await?;
    ///    let value = accessor.with(|mut access| access.get().table.delete(another_resource))?;
    ///    bar.call_concurrent(accessor, (value.0,)).await?;
    ///    Ok(())
    /// }).await??;
    /// # Ok(())
    /// # }
    /// ```
    pub async fn run_concurrent<T, R>(
        self,
        mut store: impl AsContextMut<Data = T>,
        fun: impl AsyncFnOnce(&Accessor<T>) -> R,
    ) -> Result<R>
    where
        T: 'static,
    {
        check_recursive_run();
        let mut store = store.as_context_mut();
        let token = StoreToken::new(store.as_context_mut());

        struct Dropper<'a, T: 'static, V> {
            store: StoreContextMut<'a, T>,
            value: ManuallyDrop<V>,
        }

        impl<'a, T, V> Drop for Dropper<'a, T, V> {
            fn drop(&mut self) {
                tls::set(self.store.0.traitobj_mut(), || {
                    // SAFETY: Here we drop the value without moving it for the
                    // first and only time -- per the contract for `Drop::drop`,
                    // this code won't run again, and the `value` field will no
                    // longer be accessible.
                    unsafe { ManuallyDrop::drop(&mut self.value) }
                });
            }
        }

        let accessor = &Accessor::new(token, Some(self));
        let dropper = &mut Dropper {
            store,
            value: ManuallyDrop::new(fun(accessor)),
        };
        // SAFETY: We never move `dropper` nor its `value` field.
        let future = unsafe { Pin::new_unchecked(dropper.value.deref_mut()) };

        self.poll_until(dropper.store.as_context_mut(), future)
            .await
    }

    /// Spawn a background task to run as part of this instance's event loop.
    ///
    /// The task will receive an `&Accessor<U>` and run concurrently with
    /// any other tasks in progress for the instance.
    ///
    /// Note that the task will only make progress if and when the event loop
    /// for this instance is run.
    ///
    /// The returned [`SpawnHandle`] may be used to cancel the task.
    pub fn spawn<U: 'static>(
        self,
        mut store: impl AsContextMut<Data = U>,
        task: impl AccessorTask<U, HasSelf<U>, Result<()>>,
    ) -> AbortHandle {
        let mut store = store.as_context_mut();
        let accessor = Accessor::new(StoreToken::new(store.as_context_mut()), Some(self));
        self.spawn_with_accessor(store, accessor, task)
    }

    /// Internal implementation of `spawn` functions where a `store` is
    /// available along with an `Accessor`.
    fn spawn_with_accessor<T, D>(
        self,
        mut store: StoreContextMut<T>,
        accessor: Accessor<T, D>,
        task: impl AccessorTask<T, D, Result<()>>,
    ) -> AbortHandle
    where
        T: 'static,
        D: HasData + ?Sized,
    {
        let store = store.as_context_mut();

        // Create an "abortable future" here where internally the future will
        // hook calls to poll and possibly spawn more background tasks on each
        // iteration.
        let (handle, future) =
            AbortHandle::run(async move { HostTaskOutput::Result(task.run(&accessor).await) });
        self.concurrent_state_mut(store.0)
            .push_future(Box::pin(async move {
                future.await.unwrap_or(HostTaskOutput::Result(Ok(())))
            }));

        handle
    }

    /// Run this instance's event loop.
    ///
    /// The returned future will resolve when either the specified future
    /// completes (in which case we return its result) or no further progress
    /// can be made (in which case we trap with `Trap::AsyncDeadlock`).
    async fn poll_until<T, R>(
        self,
        store: StoreContextMut<'_, T>,
        mut future: Pin<&mut impl Future<Output = R>>,
    ) -> Result<R> {
        loop {
            // Take `ConcurrentState::futures` out of the instance so we can
            // poll it while also safely giving any of the futures inside access
            // to `self`.
            let mut futures = self
                .concurrent_state_mut(store.0)
                .futures
                .get_mut()
                .unwrap()
                .take()
                .unwrap();
            let mut next = pin!(futures.next());

            let result = future::poll_fn(|cx| {
                // First, poll the future we were passed as an argument and
                // return immediately if it's ready.
                if let Poll::Ready(value) = self.set_tls(store.0, || future.as_mut().poll(cx)) {
                    return Poll::Ready(Ok(Either::Left(value)));
                }

                // Next, poll `ConcurrentState::futures` (which includes any
                // pending host tasks and/or background tasks), returning
                // immediately if one of them fails.
                let next = match self.set_tls(store.0, || next.as_mut().poll(cx)) {
                    Poll::Ready(Some(output)) => {
                        match output {
                            HostTaskOutput::Result(Err(e)) => return Poll::Ready(Err(e)),
                            HostTaskOutput::Result(Ok(())) => {}
                            HostTaskOutput::Function(fun) => {
                                // Defer calling this function to a worker fiber
                                // in case it involves calling a guest realloc
                                // function as part of a lowering operation.
                                //
                                // TODO: This isn't necessary for _all_
                                // `HostOutput::Function`s, so we could optimize
                                // by adding another variant to `HostOutput` to
                                // distinguish which ones need it and which
                                // don't.
                                self.concurrent_state_mut(store.0)
                                    .push_high_priority(WorkItem::WorkerFunction(Mutex::new(fun)))
                            }
                        }
                        Poll::Ready(true)
                    }
                    Poll::Ready(None) => Poll::Ready(false),
                    Poll::Pending => Poll::Pending,
                };

                let mut instance = self.id().get_mut(store.0);

                // Next, check the "high priority" work queue and return
                // immediately if it has at least one item.
                let state = instance.as_mut().concurrent_state_mut();
                let ready = mem::take(&mut state.high_priority);
                let ready = if ready.is_empty() {
                    // Next, check the "low priority" work queue and return
                    // immediately if it has at least one item.
                    let ready = mem::take(&mut state.low_priority);
                    if ready.is_empty() {
                        return match next {
                            Poll::Ready(true) => {
                                // In this case, one of the futures in
                                // `ConcurrentState::futures` completed
                                // successfully, so we return now and continue
                                // the outer loop in case there is another one
                                // ready to complete.
                                Poll::Ready(Ok(Either::Right(Vec::new())))
                            }
                            Poll::Ready(false) => {
                                // Poll the future we were passed one last time
                                // in case one of `ConcurrentState::futures` had
                                // the side effect of unblocking it.
                                if let Poll::Ready(value) =
                                    self.set_tls(store.0, || future.as_mut().poll(cx))
                                {
                                    Poll::Ready(Ok(Either::Left(value)))
                                } else {
                                    // In this case, there are no more pending
                                    // futures in `ConcurrentState::futures`,
                                    // there are no remaining work items, _and_
                                    // the future we were passed as an argument
                                    // still hasn't completed, meaning we're
                                    // stuck, so we return an error.  The
                                    // underlying assumption is that `future`
                                    // depends on this component instance making
                                    // such progress, and thus there's no point
                                    // in continuing to poll it given we've run
                                    // out of work to do.
                                    //
                                    // Note that we'd also reach this point if
                                    // the host embedder passed e.g. a
                                    // `std::future::Pending` to
                                    // `Instance::run_concurrent`, in which case
                                    // we'd return a "deadlock" error even when
                                    // any and all tasks have completed
                                    // normally.  However, that's not how
                                    // `Instance::run_concurrent` is intended
                                    // (and documented) to be used, so it seems
                                    // reasonable to lump that case in with
                                    // "real" deadlocks.
                                    //
                                    // TODO: Once we've added host APIs for
                                    // cancelling in-progress tasks, we can
                                    // return some other, non-error value here,
                                    // treating it as "normal" and giving the
                                    // host embedder a chance to intervene by
                                    // cancelling one or more tasks and/or
                                    // starting new tasks capable of waking the
                                    // existing ones.
                                    Poll::Ready(Err(anyhow!(crate::Trap::AsyncDeadlock)))
                                }
                            }
                            // There is at least one pending future in
                            // `ConcurrentState::futures` and we have nothing
                            // else to do but wait for now, so we return
                            // `Pending`.
                            Poll::Pending => Poll::Pending,
                        };
                    } else {
                        ready
                    }
                } else {
                    ready
                };

                Poll::Ready(Ok(Either::Right(ready)))
            })
            .await;

            // Put the `ConcurrentState::futures` back into the instance before
            // we return or handle any work items since one or more of those
            // items might append more futures.
            *self
                .concurrent_state_mut(store.0)
                .futures
                .get_mut()
                .unwrap() = Some(futures);

            match result? {
                // The future we were passed as an argument completed, so we
                // return the result.
                Either::Left(value) => break Ok(value),
                // The future we were passed has not yet completed, so handle
                // any work items and then loop again.
                Either::Right(ready) => {
                    for item in ready {
                        self.handle_work_item(store.0.traitobj_mut(), item).await?;
                    }
                }
            }
        }
    }

    /// Handle the specified work item, possibly resuming a fiber if applicable.
    async fn handle_work_item(self, store: &mut StoreOpaque, item: WorkItem) -> Result<()> {
        log::trace!("handle work item {item:?}");
        match item {
            WorkItem::PushFuture(future) => {
                self.concurrent_state_mut(store)
                    .futures
                    .get_mut()
                    .unwrap()
                    .as_mut()
                    .unwrap()
                    .push(future.into_inner().unwrap());
            }
            WorkItem::ResumeFiber(fiber) => {
                self.resume_fiber(store, fiber).await?;
            }
            WorkItem::GuestCall(call) => {
                let state = self.concurrent_state_mut(store);
                if call.is_ready(state)? {
                    self.run_on_worker(store, WorkerItem::GuestCall(call))
                        .await?;
                } else {
                    let task = state.get_mut(call.task)?;
                    if !task.starting_sent {
                        task.starting_sent = true;
                        if let GuestCallKind::Start(_) = &call.kind {
                            Waitable::Guest(call.task).set_event(
                                state,
                                Some(Event::Subtask {
                                    status: Status::Starting,
                                }),
                            )?;
                        }
                    }

                    let runtime_instance = state.get(call.task)?.instance;
                    state
                        .instance_state(runtime_instance)
                        .pending
                        .insert(call.task, call.kind);
                }
            }
            WorkItem::Poll(params) => {
                let state = self.concurrent_state_mut(store);
                if state.get_mut(params.task)?.event.is_some()
                    || !state.get_mut(params.set)?.ready.is_empty()
                {
                    // There's at least one event immediately available; deliver
                    // it to the guest ASAP.
                    state.push_high_priority(WorkItem::GuestCall(GuestCall {
                        task: params.task,
                        kind: GuestCallKind::DeliverEvent {
                            instance: params.instance,
                            set: Some(params.set),
                        },
                    }));
                } else {
                    // There are no events immediately available; deliver
                    // `Event::None` to the guest.
                    state.get_mut(params.task)?.event = Some(Event::None);
                    state.push_high_priority(WorkItem::GuestCall(GuestCall {
                        task: params.task,
                        kind: GuestCallKind::DeliverEvent {
                            instance: params.instance,
                            set: Some(params.set),
                        },
                    }));
                }
            }
            WorkItem::WorkerFunction(fun) => {
                self.run_on_worker(store, WorkerItem::Function(fun)).await?;
            }
        }

        Ok(())
    }

    /// Resume the specified fiber, giving it exclusive access to the specified
    /// store.
    async fn resume_fiber(self, store: &mut StoreOpaque, fiber: StoreFiber<'static>) -> Result<()> {
        let old_task = self.concurrent_state_mut(store).guest_task;
        log::trace!("resume_fiber: save current task {old_task:?}");

        let fiber = fiber::resolve_or_release(store, fiber).await?;

        let state = self.concurrent_state_mut(store);

        state.guest_task = old_task;
        log::trace!("resume_fiber: restore current task {old_task:?}");

        if let Some(mut fiber) = fiber {
            // See the `SuspendReason` documentation for what each case means.
            match state.suspend_reason.take().unwrap() {
                SuspendReason::NeedWork => {
                    if state.worker.is_none() {
                        state.worker = Some(fiber);
                    } else {
                        fiber.dispose(store);
                    }
                }
                SuspendReason::Yielding { .. } => {
                    state.push_low_priority(WorkItem::ResumeFiber(fiber));
                }
                SuspendReason::Waiting { set, task } => {
                    let old = state
                        .get_mut(set)?
                        .waiting
                        .insert(task, WaitMode::Fiber(fiber));
                    assert!(old.is_none());
                }
            }
        }

        Ok(())
    }

    /// Execute the specified guest call on a worker fiber.
    async fn run_on_worker(self, store: &mut StoreOpaque, item: WorkerItem) -> Result<()> {
        let worker = if let Some(fiber) = self.concurrent_state_mut(store).worker.take() {
            fiber
        } else {
            fiber::make_fiber(store.traitobj_mut(), move |store| {
                loop {
                    match self.concurrent_state_mut(store).worker_item.take().unwrap() {
                        WorkerItem::GuestCall(call) => self.handle_guest_call(store, call)?,
                        WorkerItem::Function(fun) => fun.into_inner().unwrap()(store, self)?,
                    }

                    self.suspend(store, SuspendReason::NeedWork)?;
                }
            })?
        };

        let worker_item = &mut self.concurrent_state_mut(store).worker_item;
        assert!(worker_item.is_none());
        *worker_item = Some(item);

        self.resume_fiber(store, worker).await
    }

    /// Execute the specified guest call.
    fn handle_guest_call(self, store: &mut dyn VMStore, call: GuestCall) -> Result<()> {
        match call.kind {
            GuestCallKind::DeliverEvent {
                instance: runtime_instance,
                set,
            } => {
                let state = self.concurrent_state_mut(store);
                let (event, waitable) = state.get_event(call.task, runtime_instance, set)?.unwrap();
                let task = state.get_mut(call.task)?;
                let runtime_instance = task.instance;
                let handle = waitable.map(|(_, v)| v).unwrap_or(0);

                log::trace!(
                    "use callback to deliver event {event:?} to {:?} for {waitable:?}",
                    call.task,
                );

                let old_task = state.guest_task.replace(call.task);
                log::trace!(
                    "GuestCallKind::DeliverEvent: replaced {old_task:?} with {:?} as current task",
                    call.task
                );

                self.maybe_push_call_context(store.store_opaque_mut(), call.task)?;

                let state = self.concurrent_state_mut(store);
                state.enter_instance(runtime_instance);

                let callback = state.get_mut(call.task)?.callback.take().unwrap();

                let code = callback(store, self, runtime_instance, event, handle)?;

                let state = self.concurrent_state_mut(store);

                state.get_mut(call.task)?.callback = Some(callback);

                state.exit_instance(runtime_instance)?;

                self.maybe_pop_call_context(store.store_opaque_mut(), call.task)?;

                let state = self.concurrent_state_mut(store);
                state.handle_callback_code(call.task, runtime_instance, code, false)?;

                state.guest_task = old_task;
                log::trace!("GuestCallKind::DeliverEvent: restored {old_task:?} as current task");
            }
            GuestCallKind::Start(fun) => {
                fun(store, self)?;
            }
        }

        Ok(())
    }

    /// Suspend the current fiber, storing the reason in
    /// `ConcurrentState::suspend_reason` to indicate the conditions under which
    /// it should be resumed.
    ///
    /// See the `SuspendReason` documentation for details.
    fn suspend(self, store: &mut dyn VMStore, reason: SuspendReason) -> Result<()> {
        log::trace!("suspend fiber: {reason:?}");

        // If we're yielding or waiting on behalf of a guest task, we'll need to
        // pop the call context which manages resource borrows before suspending
        // and then push it again once we've resumed.
        let task = match &reason {
            SuspendReason::Yielding { task } | SuspendReason::Waiting { task, .. } => Some(*task),
            SuspendReason::NeedWork => None,
        };

        let old_guest_task = if let Some(task) = task {
            self.maybe_pop_call_context(store.store_opaque_mut(), task)?;
            self.concurrent_state_mut(store).guest_task
        } else {
            None
        };

        let suspend_reason = &mut self.concurrent_state_mut(store).suspend_reason;
        assert!(suspend_reason.is_none());
        *suspend_reason = Some(reason);

        store.with_blocking(|_, cx| cx.suspend(StoreFiberYield::ReleaseStore))?;

        if let Some(task) = task {
            self.concurrent_state_mut(store).guest_task = old_guest_task;
            self.maybe_push_call_context(store.store_opaque_mut(), task)?;
        }

        Ok(())
    }

    /// Push the call context for managing resource borrows for the specified
    /// guest task if it has not yet either returned a result or cancelled
    /// itself.
    fn maybe_push_call_context(
        self,
        store: &mut StoreOpaque,
        guest_task: TableId<GuestTask>,
    ) -> Result<()> {
        let task = self.concurrent_state_mut(store).get_mut(guest_task)?;
        if task.lift_result.is_some() {
            log::trace!("push call context for {guest_task:?}");
            let call_context = task.call_context.take().unwrap();
            store.component_resource_state().0.push(call_context);
        }
        Ok(())
    }

    /// Pop the call context for managing resource borrows for the specified
    /// guest task if it has not yet either returned a result or cancelled
    /// itself.
    fn maybe_pop_call_context(
        self,
        store: &mut StoreOpaque,
        guest_task: TableId<GuestTask>,
    ) -> Result<()> {
        if self
            .concurrent_state_mut(store)
            .get(guest_task)?
            .lift_result
            .is_some()
        {
            log::trace!("pop call context for {guest_task:?}");
            let call_context = Some(store.component_resource_state().0.pop().unwrap());
            self.concurrent_state_mut(store)
                .get_mut(guest_task)?
                .call_context = call_context;
        }
        Ok(())
    }

    /// Add the specified guest call to the "high priority" work item queue, to
    /// be started as soon as backpressure and/or reentrance rules allow.
    ///
    /// SAFETY: The raw pointer arguments must be valid references to guest
    /// functions (with the appropriate signatures) when the closures queued by
    /// this function are called.
    unsafe fn queue_call<T: 'static>(
        self,
        mut store: StoreContextMut<T>,
        guest_task: TableId<GuestTask>,
        callee: SendSyncPtr<VMFuncRef>,
        param_count: usize,
        result_count: usize,
        flags: Option<InstanceFlags>,
        async_: bool,
        callback: Option<SendSyncPtr<VMFuncRef>>,
        post_return: Option<SendSyncPtr<VMFuncRef>>,
    ) -> Result<()> {
        /// Return a closure which will call the specified function in the scope
        /// of the specified task.
        ///
        /// This will use `GuestTask::lower_params` to lower the parameters, but
        /// will not lift the result; instead, it returns a
        /// `[MaybeUninit<ValRaw>; MAX_FLAT_PARAMS]` from which the result, if
        /// any, may be lifted.  Note that an async-lifted export will have
        /// returned its result using the `task.return` intrinsic (or not
        /// returned a result at all, in the case of `task.cancel`), in which
        /// case the "result" of this call will either be a callback code or
        /// nothing.
        ///
        /// SAFETY: `callee` must be a valid `*mut VMFuncRef` at the time when
        /// the returned closure is called.
        unsafe fn make_call<T: 'static>(
            store: StoreContextMut<T>,
            guest_task: TableId<GuestTask>,
            callee: SendSyncPtr<VMFuncRef>,
            param_count: usize,
            result_count: usize,
            flags: Option<InstanceFlags>,
        ) -> impl FnOnce(
            &mut dyn VMStore,
            Instance,
        ) -> Result<[MaybeUninit<ValRaw>; MAX_FLAT_PARAMS]>
        + Send
        + Sync
        + 'static
        + use<T> {
            let token = StoreToken::new(store);
            move |store: &mut dyn VMStore, instance: Instance| {
                let mut storage = [MaybeUninit::uninit(); MAX_FLAT_PARAMS];
                let task = instance.concurrent_state_mut(store).get_mut(guest_task)?;
                let may_enter_after_call = task.call_post_return_automatically();
                let lower = task.lower_params.take().unwrap();

                lower(store, instance, &mut storage[..param_count])?;

                let mut store = token.as_context_mut(store);

                // SAFETY: Per the contract documented in `make_call's`
                // documentation, `callee` must be a valid pointer.
                unsafe {
                    if let Some(mut flags) = flags {
                        flags.set_may_enter(false);
                    }
                    crate::Func::call_unchecked_raw(
                        &mut store,
                        callee.as_non_null(),
                        NonNull::new(
                            &mut storage[..param_count.max(result_count)]
                                as *mut [MaybeUninit<ValRaw>] as _,
                        )
                        .unwrap(),
                    )?;
                    if let Some(mut flags) = flags {
                        flags.set_may_enter(may_enter_after_call);
                    }
                }

                Ok(storage)
            }
        }

        // SAFETY: Per the contract described in this function documentation,
        // the `callee` pointer which `call` closes over must be valid when
        // called by the closure we queue below.
        let call = unsafe {
            make_call(
                store.as_context_mut(),
                guest_task,
                callee,
                param_count,
                result_count,
                flags,
            )
        };

        let callee_instance = self.concurrent_state_mut(store.0).get(guest_task)?.instance;
        let fun = if callback.is_some() {
            assert!(async_);

            Box::new(move |store: &mut dyn VMStore, instance: Instance| {
                let old_task = instance
                    .concurrent_state_mut(store)
                    .guest_task
                    .replace(guest_task);
                log::trace!(
                    "stackless call: replaced {old_task:?} with {guest_task:?} as current task"
                );

                instance.maybe_push_call_context(store.store_opaque_mut(), guest_task)?;

                instance
                    .concurrent_state_mut(store)
                    .enter_instance(callee_instance);

                // SAFETY: See the documentation for `make_call` to review the
                // contract we must uphold for `call` here.
                //
                // Per the contract described in the `queue_call`
                // documentation, the `callee` pointer which `call` closes
                // over must be valid.
                let storage = call(store, instance)?;

                instance
                    .concurrent_state_mut(store)
                    .exit_instance(callee_instance)?;

                instance.maybe_pop_call_context(store.store_opaque_mut(), guest_task)?;

                let state = instance.concurrent_state_mut(store);
                state.guest_task = old_task;
                log::trace!("stackless call: restored {old_task:?} as current task");

                // SAFETY: `wasmparser` will have validated that the callback
                // function returns a `i32` result.
                let code = unsafe { storage[0].assume_init() }.get_i32() as u32;

                state.handle_callback_code(guest_task, callee_instance, code, true)?;

                Ok(())
            })
                as Box<dyn FnOnce(&mut dyn VMStore, Instance) -> Result<()> + Send + Sync>
        } else {
            let token = StoreToken::new(store.as_context_mut());
            Box::new(move |store: &mut dyn VMStore, instance: Instance| {
                let old_task = instance
                    .concurrent_state_mut(store)
                    .guest_task
                    .replace(guest_task);
                log::trace!(
                    "stackful call: replaced {old_task:?} with {guest_task:?} as current task",
                );

                let mut flags = instance.id().get(store).instance_flags(callee_instance);

                instance.maybe_push_call_context(store.store_opaque_mut(), guest_task)?;

                // Unless this is a callback-less (i.e. stackful)
                // async-lifted export, we need to record that the instance
                // cannot be entered until the call returns.
                if !async_ {
                    instance
                        .concurrent_state_mut(store)
                        .enter_instance(callee_instance);
                }

                // SAFETY: See the documentation for `make_call` to review the
                // contract we must uphold for `call` here.
                //
                // Per the contract described in the `queue_call`
                // documentation, the `callee` pointer which `call` closes
                // over must be valid.
                let storage = call(store, instance)?;

                if async_ {
                    // This is a callback-less (i.e. stackful) async-lifted
                    // export, so there is no post-return function, and
                    // either `task.return` or `task.cancel` should have
                    // been called.
                    if instance
                        .concurrent_state_mut(store)
                        .get(guest_task)?
                        .lift_result
                        .is_some()
                    {
                        return Err(anyhow!(crate::Trap::NoAsyncResult));
                    }
                } else {
                    // This is a sync-lifted export, so now is when we lift the
                    // result, optionally call the post-return function, if any,
                    // and finally notify any current or future waiters that the
                    // subtask has returned.

                    let lift = {
                        let state = instance.concurrent_state_mut(store);
                        state.exit_instance(callee_instance)?;

                        assert!(state.get(guest_task)?.result.is_none());

                        state.get_mut(guest_task)?.lift_result.take().unwrap()
                    };

                    // SAFETY: `result_count` represents the number of core Wasm
                    // results returned, per `wasmparser`.
                    let result = (lift.lift)(store, instance, unsafe {
                        mem::transmute::<&[MaybeUninit<ValRaw>], &[ValRaw]>(
                            &storage[..result_count],
                        )
                    })?;

                    let post_return_arg = match result_count {
                        0 => ValRaw::i32(0),
                        // SAFETY: `result_count` represents the number of
                        // core Wasm results returned, per `wasmparser`.
                        1 => unsafe { storage[0].assume_init() },
                        _ => unreachable!(),
                    };

                    if instance
                        .concurrent_state_mut(store)
                        .get(guest_task)?
                        .call_post_return_automatically()
                    {
                        unsafe { flags.set_needs_post_return(false) }

                        if let Some(func) = post_return {
                            let mut store = token.as_context_mut(store);

                            // SAFETY: `func` is a valid `*mut VMFuncRef` from
                            // either `wasmtime-cranelift`-generated fused adapter
                            // code or `component::Options`.  Per `wasmparser`
                            // post-return signature validation, we know it takes a
                            // single parameter.
                            unsafe {
                                crate::Func::call_unchecked_raw(
                                    &mut store,
                                    func.as_non_null(),
                                    slice::from_ref(&post_return_arg).into(),
                                )?;
                            }
                        }

                        unsafe { flags.set_may_enter(true) }
                    }

                    instance.task_complete(
                        store,
                        guest_task,
                        result,
                        Status::Returned,
                        post_return_arg,
                    )?;
                }

                instance.maybe_pop_call_context(store.store_opaque_mut(), guest_task)?;

                let task = instance.concurrent_state_mut(store).get_mut(guest_task)?;

                match &task.caller {
                    Caller::Host {
                        remove_task_automatically,
                        ..
                    } => {
                        if *remove_task_automatically {
                            Waitable::Guest(guest_task)
                                .delete_from(instance.concurrent_state_mut(store))?;
                        }
                    }
                    Caller::Guest { .. } => {
                        task.exited = true;
                    }
                }

                Ok(())
            })
        };

        self.concurrent_state_mut(store.0)
            .push_high_priority(WorkItem::GuestCall(GuestCall {
                task: guest_task,
                kind: GuestCallKind::Start(fun),
            }));

        Ok(())
    }

    /// Prepare (but do not start) a guest->guest call.
    ///
    /// This is called from fused adapter code generated in
    /// `wasmtime_environ::fact::trampoline::Compiler`.  `start` and `return_`
    /// are synthesized Wasm functions which move the parameters from the caller
    /// to the callee and the result from the callee to the caller,
    /// respectively.  The adapter will call `Self::start_call` immediately
    /// after calling this function.
    ///
    /// SAFETY: All the pointer arguments must be valid pointers to guest
    /// entities (and with the expected signatures for the function references
    /// -- see `wasmtime_environ::fact::trampoline::Compiler` for details).
    unsafe fn prepare_call<T: 'static>(
        self,
        mut store: StoreContextMut<T>,
        start: *mut VMFuncRef,
        return_: *mut VMFuncRef,
        caller_instance: RuntimeComponentInstanceIndex,
        callee_instance: RuntimeComponentInstanceIndex,
        task_return_type: TypeTupleIndex,
        memory: *mut VMMemoryDefinition,
        string_encoding: u8,
        caller_info: CallerInfo,
    ) -> Result<()> {
        enum ResultInfo {
            Heap { results: u32 },
            Stack { result_count: u32 },
        }

        let result_info = match &caller_info {
            CallerInfo::Async {
                has_result: true,
                params,
            } => ResultInfo::Heap {
                results: params.last().unwrap().get_u32(),
            },
            CallerInfo::Async {
                has_result: false, ..
            } => ResultInfo::Stack { result_count: 0 },
            CallerInfo::Sync {
                result_count,
                params,
            } if *result_count > u32::try_from(MAX_FLAT_RESULTS).unwrap() => ResultInfo::Heap {
                results: params.last().unwrap().get_u32(),
            },
            CallerInfo::Sync { result_count, .. } => ResultInfo::Stack {
                result_count: *result_count,
            },
        };

        let sync_caller = matches!(caller_info, CallerInfo::Sync { .. });

        // Create a new guest task for the call, closing over the `start` and
        // `return_` functions to lift the parameters and lower the result,
        // respectively.
        let start = SendSyncPtr::new(NonNull::new(start).unwrap());
        let return_ = SendSyncPtr::new(NonNull::new(return_).unwrap());
        let token = StoreToken::new(store.as_context_mut());
        let state = self.concurrent_state_mut(store.0);
        let old_task = state.guest_task.take();
        let new_task = GuestTask::new(
            state,
            Box::new(move |store, instance, dst| {
                let mut store = token.as_context_mut(store);
                assert!(dst.len() <= MAX_FLAT_PARAMS);
                let mut src = [MaybeUninit::uninit(); MAX_FLAT_PARAMS];
                let count = match caller_info {
                    // Async callers, if they have a result, use the last
                    // parameter as a return pointer so chop that off if
                    // relevant here.
                    CallerInfo::Async { params, has_result } => {
                        let params = &params[..params.len() - usize::from(has_result)];
                        for (param, src) in params.iter().zip(&mut src) {
                            src.write(*param);
                        }
                        params.len()
                    }

                    // Sync callers forward everything directly.
                    CallerInfo::Sync { params, .. } => {
                        for (param, src) in params.iter().zip(&mut src) {
                            src.write(*param);
                        }
                        params.len()
                    }
                };
                // SAFETY: `start` is a valid `*mut VMFuncRef` from
                // `wasmtime-cranelift`-generated fused adapter code.  Based on
                // how it was constructed (see
                // `wasmtime_environ::fact::trampoline::Compiler::compile_async_start_adapter`
                // for details) we know it takes count parameters and returns
                // `dst.len()` results.
                unsafe {
                    crate::Func::call_unchecked_raw(
                        &mut store,
                        start.as_non_null(),
                        NonNull::new(
                            &mut src[..count.max(dst.len())] as *mut [MaybeUninit<ValRaw>] as _,
                        )
                        .unwrap(),
                    )?;
                }
                dst.copy_from_slice(&src[..dst.len()]);
                let state = instance.concurrent_state_mut(store.0);
                let task = state.guest_task.unwrap();
                Waitable::Guest(task).set_event(
                    state,
                    Some(Event::Subtask {
                        status: Status::Started,
                    }),
                )?;
                Ok(())
            }),
            LiftResult {
                lift: Box::new(move |store, instance, src| {
                    // SAFETY: See comment in closure passed as `lower_params`
                    // parameter above.
                    let mut store = token.as_context_mut(store);
                    let mut my_src = src.to_owned(); // TODO: use stack to avoid allocation?
                    if let ResultInfo::Heap { results } = &result_info {
                        my_src.push(ValRaw::u32(*results));
                    }
                    // SAFETY: `return_` is a valid `*mut VMFuncRef` from
                    // `wasmtime-cranelift`-generated fused adapter code.  Based
                    // on how it was constructed (see
                    // `wasmtime_environ::fact::trampoline::Compiler::compile_async_return_adapter`
                    // for details) we know it takes `src.len()` parameters and
                    // returns up to 1 result.
                    unsafe {
                        crate::Func::call_unchecked_raw(
                            &mut store,
                            return_.as_non_null(),
                            my_src.as_mut_slice().into(),
                        )?;
                    }
                    let state = instance.concurrent_state_mut(store.0);
                    let task = state.guest_task.unwrap();
                    if sync_caller {
                        state.get_mut(task)?.sync_result =
                            Some(if let ResultInfo::Stack { result_count } = &result_info {
                                match result_count {
                                    0 => None,
                                    1 => Some(my_src[0]),
                                    _ => unreachable!(),
                                }
                            } else {
                                None
                            });
                    }
                    Ok(Box::new(DummyResult) as Box<dyn Any + Send + Sync>)
                }),
                ty: task_return_type,
                memory: NonNull::new(memory).map(SendSyncPtr::new),
                string_encoding: StringEncoding::from_u8(string_encoding).unwrap(),
            },
            Caller::Guest {
                task: old_task.unwrap(),
                instance: caller_instance,
            },
            None,
            callee_instance,
        )?;

        let guest_task = state.push(new_task)?;

        if let Some(old_task) = old_task {
            if !state.may_enter(guest_task) {
                bail!(crate::Trap::CannotEnterComponent);
            }

            state.get_mut(old_task)?.subtasks.insert(guest_task);
        };

        // Make the new task the current one so that `Self::start_call` knows
        // which one to start.
        state.guest_task = Some(guest_task);
        log::trace!("pushed {guest_task:?} as current task; old task was {old_task:?}");

        Ok(())
    }

    /// Call the specified callback function for an async-lifted export.
    ///
    /// SAFETY: `function` must be a valid reference to a guest function of the
    /// correct signature for a callback.
    unsafe fn call_callback<T>(
        self,
        mut store: StoreContextMut<T>,
        callee_instance: RuntimeComponentInstanceIndex,
        function: SendSyncPtr<VMFuncRef>,
        event: Event,
        handle: u32,
        may_enter_after_call: bool,
    ) -> Result<u32> {
        let mut flags = self.id().get(store.0).instance_flags(callee_instance);

        let (ordinal, result) = event.parts();
        let params = &mut [
            ValRaw::u32(ordinal),
            ValRaw::u32(handle),
            ValRaw::u32(result),
        ];
        // SAFETY: `func` is a valid `*mut VMFuncRef` from either
        // `wasmtime-cranelift`-generated fused adapter code or
        // `component::Options`.  Per `wasmparser` callback signature
        // validation, we know it takes three parameters and returns one.
        unsafe {
            flags.set_may_enter(false);
            crate::Func::call_unchecked_raw(
                &mut store,
                function.as_non_null(),
                params.as_mut_slice().into(),
            )?;
            flags.set_may_enter(may_enter_after_call);
        }
        Ok(params[0].get_u32())
    }

    /// Start a guest->guest call previously prepared using
    /// `Self::prepare_call`.
    ///
    /// This is called from fused adapter code generated in
    /// `wasmtime_environ::fact::trampoline::Compiler`.  The adapter will call
    /// this function immediately after calling `Self::prepare_call`.
    ///
    /// SAFETY: The `*mut VMFuncRef` arguments must be valid pointers to guest
    /// functions with the appropriate signatures for the current guest task.
    /// If this is a call to an async-lowered import, the actual call may be
    /// deferred and run after this function returns, in which case the pointer
    /// arguments must also be valid when the call happens.
    unsafe fn start_call<T: 'static>(
        self,
        mut store: StoreContextMut<T>,
        callback: *mut VMFuncRef,
        post_return: *mut VMFuncRef,
        callee: *mut VMFuncRef,
        param_count: u32,
        result_count: u32,
        flags: u32,
        storage: Option<&mut [MaybeUninit<ValRaw>]>,
    ) -> Result<u32> {
        let token = StoreToken::new(store.as_context_mut());
        let async_caller = storage.is_none();
        let state = self.concurrent_state_mut(store.0);
        let guest_task = state.guest_task.unwrap();
        let may_enter_after_call = state.get(guest_task)?.call_post_return_automatically();
        let callee = SendSyncPtr::new(NonNull::new(callee).unwrap());
        let param_count = usize::try_from(param_count).unwrap();
        assert!(param_count <= MAX_FLAT_PARAMS);
        let result_count = usize::try_from(result_count).unwrap();
        assert!(result_count <= MAX_FLAT_RESULTS);

        let task = state.get_mut(guest_task)?;
        if !callback.is_null() {
            // We're calling an async-lifted export with a callback, so store
            // the callback and related context as part of the task so we can
            // call it later when needed.
            let callback = SendSyncPtr::new(NonNull::new(callback).unwrap());
            task.callback = Some(Box::new(
                move |store, instance, runtime_instance, event, handle| {
                    let store = token.as_context_mut(store);
                    unsafe {
                        instance.call_callback::<T>(
                            store,
                            runtime_instance,
                            callback,
                            event,
                            handle,
                            may_enter_after_call,
                        )
                    }
                },
            ));
        }

        let Caller::Guest {
            task: caller,
            instance: runtime_instance,
        } = &task.caller
        else {
            // As of this writing, `start_call` is only used for guest->guest
            // calls.
            unreachable!()
        };
        let caller = *caller;
        let caller_instance = *runtime_instance;

        let callee_instance = task.instance;

        let instance_flags = if callback.is_null() {
            None
        } else {
            Some(self.id().get(store.0).instance_flags(callee_instance))
        };

        // Queue the call as a "high priority" work item.
        unsafe {
            self.queue_call(
                store.as_context_mut(),
                guest_task,
                callee,
                param_count,
                result_count,
                instance_flags,
                (flags & START_FLAG_ASYNC_CALLEE) != 0,
                NonNull::new(callback).map(SendSyncPtr::new),
                NonNull::new(post_return).map(SendSyncPtr::new),
            )?;
        }

        let state = self.concurrent_state_mut(store.0);

        // Use the caller's `GuestTask::sync_call_set` to register interest in
        // the subtask...
        let guest_waitable = Waitable::Guest(guest_task);
        let old_set = guest_waitable.common(state)?.set;
        let set = state.get_mut(caller)?.sync_call_set;
        guest_waitable.join(state, Some(set))?;

        // ... and suspend this fiber temporarily while we wait for it to start.
        //
        // Note that we _could_ call the callee directly using the current fiber
        // rather than suspend this one, but that would make reasoning about the
        // event loop more complicated and is probably only worth doing if
        // there's a measurable performance benefit.  In addition, it would mean
        // blocking the caller if the callee calls a blocking sync-lowered
        // import, and as of this writing the spec says we must not do that.
        //
        // Alternatively, the fused adapter code could be modified to call the
        // callee directly without calling a host-provided intrinsic at all (in
        // which case it would need to do its own, inline backpressure checks,
        // etc.).  Again, we'd want to see a measurable performance benefit
        // before committing to such an optimization.  And again, we'd need to
        // update the spec to allow that.
        let (status, waitable) = loop {
            self.suspend(
                store.0.traitobj_mut(),
                SuspendReason::Waiting { set, task: caller },
            )?;

            let state = self.concurrent_state_mut(store.0);

            let event = Waitable::Guest(guest_task).take_event(state)?;
            let Some(Event::Subtask { status }) = event else {
                unreachable!();
            };

            log::trace!("status {status:?} for {guest_task:?}");

            if status == Status::Returned {
                // It returned, so we can stop waiting.
                break (status, None);
            } else if async_caller {
                // It hasn't returned yet, but the caller is calling via an
                // async-lowered import, so we generate a handle for the task
                // waitable and return the status.
                break (
                    status,
                    Some(
                        state.waitable_tables[caller_instance]
                            .insert(guest_task.rep(), WaitableState::GuestTask)?,
                    ),
                );
            } else {
                // The callee hasn't returned yet, and the caller is calling via
                // a sync-lowered import, so we loop and keep waiting until the
                // callee returns.
            }
        };

        let state = self.concurrent_state_mut(store.0);

        guest_waitable.join(state, old_set)?;

        if let Some(storage) = storage {
            // The caller used a sync-lowered import to call an async-lifted
            // export, in which case the result, if any, has been stashed in
            // `GuestTask::sync_result`.
            if let Some(result) = state.get_mut(guest_task)?.sync_result.take() {
                if let Some(result) = result {
                    storage[0] = MaybeUninit::new(result);
                }

                Waitable::Guest(guest_task).delete_from(state)?;
            } else {
                // This means the callee failed to call either `task.return` or
                // `task.cancel` before exiting.
                return Err(anyhow!(crate::Trap::NoAsyncResult));
            }
        }

        // Reset the current task to point to the caller as it resumes control.
        state.guest_task = Some(caller);
        log::trace!("popped current task {guest_task:?}; new task is {caller:?}");

        Ok(status.pack(waitable))
    }

    /// Wrap the specified host function in a future which will call it, passing
    /// it an `&Accessor<T>`.
    ///
    /// See the `Accessor` documentation for details.
    pub(crate) fn wrap_call<T: 'static, F, R>(
        self,
        store: StoreContextMut<T>,
        closure: F,
    ) -> impl Future<Output = Result<R>> + 'static
    where
        T: 'static,
        F: FnOnce(&Accessor<T>) -> Pin<Box<dyn Future<Output = Result<R>> + Send + '_>>
            + Send
            + Sync
            + 'static,
        R: Send + Sync + 'static,
    {
        let token = StoreToken::new(store);
        async move {
            let mut accessor = Accessor::new(token, Some(self));
            closure(&mut accessor).await
        }
    }

    /// Poll the specified future once on behalf of a guest->host call using an
    /// async-lowered import.
    ///
    /// If it returns `Ready`, return `Ok(None)`.  Otherwise, if it returns
    /// `Pending`, add it to the set of futures to be polled as part of this
    /// instance's event loop until it completes, and then return
    /// `Ok(Some(handle))` where `handle` is the waitable handle to return.
    ///
    /// Whether the future returns `Ready` immediately or later, the `lower`
    /// function will be used to lower the result, if any, into the guest caller's
    /// stack and linear memory unless the task has been cancelled.
    pub(crate) fn first_poll<T: 'static, R: Send + 'static>(
        self,
        mut store: StoreContextMut<T>,
        future: impl Future<Output = Result<R>> + Send + 'static,
        caller_instance: RuntimeComponentInstanceIndex,
        lower: impl FnOnce(StoreContextMut<T>, Instance, R) -> Result<()> + Send + 'static,
    ) -> Result<Option<u32>> {
        let token = StoreToken::new(store.as_context_mut());
        let state = self.concurrent_state_mut(store.0);
        let caller = state.guest_task.unwrap();

        // Create an abortable future which hooks calls to poll and manages call
        // context state for the future.
        let (abort_handle, future) = AbortHandle::run(async move {
            let mut future = pin!(future);
            let mut call_context = None;
            future::poll_fn(move |cx| {
                // Push the call context for managing any resource borrows
                // for the task.
                tls::get(|store| {
                    if let Some(call_context) = call_context.take() {
                        token
                            .as_context_mut(store)
                            .0
                            .component_resource_state()
                            .0
                            .push(call_context);
                    }
                });

                let result = future.as_mut().poll(cx);

                if result.is_pending() {
                    // Pop the call context for managing any resource
                    // borrows for the task.
                    tls::get(|store| {
                        call_context = Some(
                            token
                                .as_context_mut(store)
                                .0
                                .component_resource_state()
                                .0
                                .pop()
                                .unwrap(),
                        );
                    });
                }
                result
            })
            .await
        });

        // We create a new host task even though it might complete immediately
        // (in which case we won't need to pass a waitable back to the guest).
        // If it does complete immediately, we'll remove it before we return.
        let task = state.push(HostTask::new(caller_instance, Some(abort_handle)))?;

        log::trace!("new host task child of {caller:?}: {task:?}");
        let token = StoreToken::new(store.as_context_mut());

        // Map the output of the future to a `HostTaskOutput` responsible for
        // lowering the result into the guest's stack and memory, as well as
        // notifying any waiters that the task returned.
        let mut future = Box::pin(async move {
            let result = match future.await {
                Some(result) => result,
                // Task was cancelled; nothing left to do.
                None => return HostTaskOutput::Result(Ok(())),
            };
            HostTaskOutput::Function(Box::new(move |store, instance| {
                let mut store = token.as_context_mut(store);
                lower(store.as_context_mut(), instance, result?)?;
                let state = instance.concurrent_state_mut(store.0);
                state.get_mut(task)?.abort_handle.take();
                Waitable::Host(task).set_event(
                    state,
                    Some(Event::Subtask {
                        status: Status::Returned,
                    }),
                )?;

                Ok(())
            }))
        });

        // Finally, poll the future.  We can use a dummy `Waker` here because
        // we'll add the future to `ConcurrentState::futures` and poll it
        // automatically from the event loop if it doesn't complete immediately
        // here.
        let poll = self.set_tls(store.0, || {
            future
                .as_mut()
                .poll(&mut Context::from_waker(&Waker::noop()))
        });

        Ok(match poll {
            Poll::Ready(output) => {
                // It finished immediately; lower the result and delete the
                // task.
                output.consume(store.0.traitobj_mut(), self)?;
                log::trace!("delete host task {task:?} (already ready)");
                self.concurrent_state_mut(store.0).delete(task)?;
                None
            }
            Poll::Pending => {
                // It hasn't finished yet; add the future to
                // `ConcurrentState::futures` so it will be polled by the event
                // loop and allocate a waitable handle to return to the guest.
                let state = self.concurrent_state_mut(store.0);
                state.push_future(future);
                let handle = state.waitable_tables[caller_instance]
                    .insert(task.rep(), WaitableState::HostTask)?;
                log::trace!(
                    "assign {task:?} handle {handle} for {caller:?} instance {caller_instance:?}"
                );
                Some(handle)
            }
        })
    }

    /// Poll the specified future until it completes on behalf of a guest->host
    /// call using a sync-lowered import.
    ///
    /// This is similar to `Self::first_poll` except it's for sync-lowered
    /// imports, meaning we don't need to handle cancellation and we can block
    /// the caller until the task completes, at which point the caller can
    /// handle lowering the result to the guest's stack and linear memory.
    pub(crate) fn poll_and_block<R: Send + Sync + 'static>(
        self,
        store: &mut dyn VMStore,
        future: impl Future<Output = Result<R>> + Send + 'static,
        caller_instance: RuntimeComponentInstanceIndex,
    ) -> Result<R> {
        let state = self.concurrent_state_mut(store);

        // If there is no current guest task set, that means the host function
        // was registered using e.g. `LinkerInstance::func_wrap`, in which case
        // it should complete immediately.
        let Some(caller) = state.guest_task else {
            return match pin!(future).poll(&mut Context::from_waker(&Waker::noop())) {
                Poll::Ready(result) => result,
                Poll::Pending => {
                    unreachable!()
                }
            };
        };

        // Save any existing result stashed in `GuestTask::result` so we can
        // replace it with the new result.
        let old_result = state
            .get_mut(caller)
            .with_context(|| format!("bad handle: {caller:?}"))?
            .result
            .take();

        // Add a temporary host task into the table so we can track its
        // progress.  Note that we'll never allocate a waitable handle for the
        // guest since we're being called synchronously.
        let task = state.push(HostTask::new(caller_instance, None))?;

        log::trace!("new host task child of {caller:?}: {task:?}");

        // Map the output of the future to a `HostTaskOutput` which will take
        // care of stashing the result in `GuestTask::result` and resuming this
        // fiber when the host task completes.
        let mut future = Box::pin(future.map(move |result| {
            HostTaskOutput::Function(Box::new(move |store, instance| {
                let state = instance.concurrent_state_mut(store);
                state.get_mut(caller)?.result = Some(Box::new(result?) as _);

                Waitable::Host(task).set_event(
                    state,
                    Some(Event::Subtask {
                        status: Status::Returned,
                    }),
                )?;

                Ok(())
            }))
        })) as HostTaskFuture;

        // Finally, poll the future.  We can use a dummy `Waker` here because
        // we'll add the future to `ConcurrentState::futures` and poll it
        // automatically from the event loop if it doesn't complete immediately
        // here.
        let poll = self.set_tls(store, || {
            future
                .as_mut()
                .poll(&mut Context::from_waker(&Waker::noop()))
        });

        match poll {
            Poll::Ready(output) => {
                // It completed immediately; run the `HostTaskOutput` function
                // to stash the result and delete the task.
                output.consume(store, self)?;
                log::trace!("delete host task {task:?} (already ready)");
                self.concurrent_state_mut(store).delete(task)?;
            }
            Poll::Pending => {
                // It did not complete immediately; add it to
                // `ConcurrentState::futures` so it will be polled via the event
                // loop, then use `GuestTask::sync_call_set` to wait for the
                // task to complete, suspending the current fiber until it does
                // so.
                let state = self.concurrent_state_mut(store);
                state.push_future(future);

                let set = state.get_mut(caller)?.sync_call_set;
                Waitable::Host(task).join(state, Some(set))?;

                self.suspend(store, SuspendReason::Waiting { set, task: caller })?;
            }
        }

        // Retrieve and return the result.
        Ok(*mem::replace(
            &mut self.concurrent_state_mut(store).get_mut(caller)?.result,
            old_result,
        )
        .unwrap()
        .downcast()
        .unwrap())
    }

    /// Implements the `task.return` intrinsic, lifting the result for the
    /// current guest task.
    pub(crate) fn task_return(
        self,
        store: &mut dyn VMStore,
        ty: TypeTupleIndex,
        options: OptionsIndex,
        storage: &[ValRaw],
    ) -> Result<()> {
        let state = self.concurrent_state_mut(store);
        let CanonicalOptions {
            string_encoding,
            data_model,
            ..
        } = *state.options(options);
        let guest_task = state.guest_task.unwrap();
        let lift = state
            .get_mut(guest_task)?
            .lift_result
            .take()
            .ok_or_else(|| {
                anyhow!("`task.return` or `task.cancel` called more than once for current task")
            })?;
        assert!(state.get(guest_task)?.result.is_none());

        let invalid = ty != lift.ty
            || string_encoding != lift.string_encoding
            || match data_model {
                CanonicalOptionsDataModel::LinearMemory(opts) => match opts.memory {
                    Some(memory) => {
                        let expected = lift.memory.map(|v| v.as_ptr()).unwrap_or(ptr::null_mut());
                        let actual = self.id().get(store).runtime_memory(memory);
                        expected != actual
                    }
                    // Memory not specified, meaning it didn't need to be
                    // specified per validation, so not invalid.
                    None => false,
                },
                // Always invalid as this isn't supported.
                CanonicalOptionsDataModel::Gc { .. } => true,
            };

        if invalid {
            bail!("invalid `task.return` signature and/or options for current task");
        }

        log::trace!("task.return for {guest_task:?}");

        let result = (lift.lift)(store, self, storage)?;

        self.task_complete(store, guest_task, result, Status::Returned, ValRaw::i32(0))
    }

    /// Implements the `task.cancel` intrinsic.
    pub(crate) fn task_cancel(
        self,
        store: &mut dyn VMStore,
        _caller_instance: RuntimeComponentInstanceIndex,
    ) -> Result<()> {
        let state = self.concurrent_state_mut(store);
        let guest_task = state.guest_task.unwrap();
        let task = state.get_mut(guest_task)?;
        if !task.cancel_sent {
            bail!("`task.cancel` called by task which has not been cancelled")
        }
        _ = task.lift_result.take().ok_or_else(|| {
            anyhow!("`task.return` or `task.cancel` called more than once for current task")
        })?;

        assert!(task.result.is_none());

        log::trace!("task.cancel for {guest_task:?}");

        self.task_complete(
            store,
            guest_task,
            Box::new(DummyResult),
            Status::ReturnCancelled,
            ValRaw::i32(0),
        )
    }

    /// Complete the specified guest task (i.e. indicate that it has either
    /// returned a (possibly empty) result or cancelled itself).
    ///
    /// This will return any resource borrows and notify any current or future
    /// waiters that the task has completed.
    fn task_complete(
        self,
        store: &mut dyn VMStore,
        guest_task: TableId<GuestTask>,
        result: Box<dyn Any + Send + Sync>,
        status: Status,
        post_return_arg: ValRaw,
    ) -> Result<()> {
        if self
            .concurrent_state_mut(store)
            .get(guest_task)?
            .call_post_return_automatically()
        {
            let (calls, host_table, _, instance) = store
                .store_opaque_mut()
                .component_resource_state_with_instance(self);
            ResourceTables {
                calls,
                host_table: Some(host_table),
                guest: Some(instance.guest_tables()),
            }
            .exit_call()?;
        } else {
            // As of this writing, the only scenario where `call_post_return_automatically`
            // would be false for a `GuestTask` is for host-to-guest calls using
            // `[Typed]Func::call_async`, in which case the `function_index`
            // should be a non-`None` value.
            let function_index = self
                .concurrent_state_mut(store)
                .get(guest_task)?
                .function_index
                .unwrap();

            self.id()
                .get_mut(store)
                .post_return_arg_set(function_index, post_return_arg);
        }

        let state = self.concurrent_state_mut(store);
        let task = state.get_mut(guest_task)?;

        if let Caller::Host { tx, .. } = &mut task.caller {
            if let Some(tx) = tx.take() {
                _ = tx.send(result);
            }
        } else {
            task.result = Some(result);
            Waitable::Guest(guest_task).set_event(state, Some(Event::Subtask { status }))?;
        }

        Ok(())
    }

    /// Implements the `waitable-set.wait` intrinsic.
    pub(crate) fn waitable_set_wait(
        self,
        store: &mut dyn VMStore,
        options: OptionsIndex,
        set: u32,
        payload: u32,
    ) -> Result<u32> {
        let state = self.concurrent_state_mut(store);
        let opts = state.options(options);
        let async_ = opts.async_;
        let caller_instance = opts.instance;
        let (rep, WaitableState::Set) =
            state.waitable_tables[caller_instance].get_mut_by_index(set)?
        else {
            bail!("invalid waitable-set handle");
        };

        self.waitable_check(
            store,
            async_,
            WaitableCheck::Wait(WaitableCheckParams {
                set: TableId::new(rep),
                caller_instance,
                options,
                payload,
            }),
        )
    }

    /// Implements the `waitable-set.poll` intrinsic.
    pub(crate) fn waitable_set_poll(
        self,
        store: &mut dyn VMStore,
        options: OptionsIndex,
        set: u32,
        payload: u32,
    ) -> Result<u32> {
        let state = self.concurrent_state_mut(store);
        let opts = state.options(options);
        let async_ = opts.async_;
        let caller_instance = opts.instance;
        let (rep, WaitableState::Set) =
            state.waitable_tables[caller_instance].get_mut_by_index(set)?
        else {
            bail!("invalid waitable-set handle");
        };

        self.waitable_check(
            store,
            async_,
            WaitableCheck::Poll(WaitableCheckParams {
                set: TableId::new(rep),
                caller_instance,
                options,
                payload,
            }),
        )
    }

    /// Implements the `yield` intrinsic.
    pub(crate) fn yield_(self, store: &mut dyn VMStore, async_: bool) -> Result<bool> {
        self.waitable_check(store, async_, WaitableCheck::Yield)
            .map(|_code| {
                // TODO: plumb cancellation to here:
                // https://github.com/bytecodealliance/wasmtime/issues/11191
                false
            })
    }

    /// Helper function for the `waitable-set.wait`, `waitable-set.poll`, and
    /// `yield` intrinsics.
    fn waitable_check(
        self,
        store: &mut dyn VMStore,
        async_: bool,
        check: WaitableCheck,
    ) -> Result<u32> {
        if async_ {
            bail!(
                "todo: async `waitable-set.wait`, `waitable-set.poll`, and `yield` not yet implemented"
            );
        }

        let guest_task = self.concurrent_state_mut(store).guest_task.unwrap();

        let (wait, set) = match &check {
            WaitableCheck::Wait(params) => (true, Some(params.set)),
            WaitableCheck::Poll(params) => (false, Some(params.set)),
            WaitableCheck::Yield => (false, None),
        };

        // First, suspend this fiber, allowing any other tasks to run.
        self.suspend(store, SuspendReason::Yielding { task: guest_task })?;

        log::trace!("waitable check for {guest_task:?}; set {set:?}");

        let state = self.concurrent_state_mut(store);
        let task = state.get(guest_task)?;

        if wait && task.callback.is_some() {
            bail!("cannot call `task.wait` from async-lifted export with callback");
        }

        // If we're waiting, and there are no events immediately available,
        // suspend the fiber until that changes.
        if wait {
            let set = set.unwrap();

            if task.event.is_none() && state.get(set)?.ready.is_empty() {
                let old = state.get_mut(guest_task)?.wake_on_cancel.replace(set);
                assert!(old.is_none());

                self.suspend(
                    store,
                    SuspendReason::Waiting {
                        set,
                        task: guest_task,
                    },
                )?;
            }
        }

        log::trace!("waitable check for {guest_task:?}; set {set:?}, part two");

        let result = match check {
            // Deliver any pending events to the guest and return.
            WaitableCheck::Wait(params) | WaitableCheck::Poll(params) => {
                let event = self.concurrent_state_mut(store).get_event(
                    guest_task,
                    params.caller_instance,
                    Some(params.set),
                )?;

                let (ordinal, handle, result) = if wait {
                    let (event, waitable) = event.unwrap();
                    let handle = waitable.map(|(_, v)| v).unwrap_or(0);
                    let (ordinal, result) = event.parts();
                    (ordinal, handle, result)
                } else {
                    if let Some((event, waitable)) = event {
                        let handle = waitable.map(|(_, v)| v).unwrap_or(0);
                        let (ordinal, result) = event.parts();
                        (ordinal, handle, result)
                    } else {
                        log::trace!(
                            "no events ready to deliver via waitable-set.poll to {guest_task:?}; set {:?}",
                            params.set
                        );
                        let (ordinal, result) = Event::None.parts();
                        (ordinal, 0, result)
                    }
                };
                let store = store.store_opaque_mut();
                let options = Options::new_index(store, self, params.options);
                let ptr = func::validate_inbounds::<(u32, u32)>(
                    options.memory_mut(store),
                    &ValRaw::u32(params.payload),
                )?;
                options.memory_mut(store)[ptr + 0..][..4].copy_from_slice(&handle.to_le_bytes());
                options.memory_mut(store)[ptr + 4..][..4].copy_from_slice(&result.to_le_bytes());
                Ok(ordinal)
            }
            // TODO: Check `GuestTask::event` in case it contains
            // `Event::Cancelled`, in which case we'll need to return that to
            // the guest:
            // https://github.com/bytecodealliance/wasmtime/issues/11191
            WaitableCheck::Yield => Ok(0),
        };

        result
    }

    /// Implements the `subtask.cancel` intrinsic.
    pub(crate) fn subtask_cancel(
        self,
        store: &mut dyn VMStore,
        caller_instance: RuntimeComponentInstanceIndex,
        async_: bool,
        task_id: u32,
    ) -> Result<u32> {
        let concurrent_state = self.concurrent_state_mut(store);
        let (rep, state) =
            concurrent_state.waitable_tables[caller_instance].get_mut_by_index(task_id)?;
        let (waitable, expected_caller_instance) = match state {
            WaitableState::HostTask => {
                let id = TableId::<HostTask>::new(rep);
                (
                    Waitable::Host(id),
                    concurrent_state.get(id)?.caller_instance,
                )
            }
            WaitableState::GuestTask => {
                let id = TableId::<GuestTask>::new(rep);
                if let Caller::Guest { instance, .. } = &concurrent_state.get(id)?.caller {
                    (Waitable::Guest(id), *instance)
                } else {
                    unreachable!()
                }
            }
            _ => bail!("invalid task handle: {task_id}"),
        };
        // Since waitables can neither be passed between instances nor forged,
        // this should never fail unless there's a bug in Wasmtime, but we check
        // here to be sure:
        assert_eq!(expected_caller_instance, caller_instance);

        log::trace!("subtask_cancel {waitable:?} (handle {task_id})");

        if let Waitable::Host(host_task) = waitable {
            if let Some(handle) = concurrent_state.get_mut(host_task)?.abort_handle.take() {
                handle.abort();
                return Ok(Status::ReturnCancelled as u32);
            }
        } else {
            let caller = concurrent_state.guest_task.unwrap();
            let guest_task = TableId::<GuestTask>::new(rep);
            let task = concurrent_state.get_mut(guest_task)?;
            if task.lower_params.is_some() {
                task.lower_params = None;
                task.lift_result = None;

                // Not yet started; cancel and remove from pending
                let callee_instance = task.instance;

                let kind = concurrent_state
                    .instance_state(callee_instance)
                    .pending
                    .remove(&guest_task);

                if kind.is_none() {
                    bail!("`subtask.cancel` called after terminal status delivered");
                }

                return Ok(Status::StartCancelled as u32);
            } else if task.lift_result.is_some() {
                // Started, but not yet returned or cancelled; send the
                // `CANCELLED` event
                task.cancel_sent = true;
                // Note that this might overwrite an event that was set earlier
                // (e.g. `Event::None` if the task is yielding, or
                // `Event::Cancelled` if it was already cancelled), but that's
                // okay -- this should supersede the previous state.
                task.event = Some(Event::Cancelled);
                if let Some(set) = task.wake_on_cancel.take() {
                    let item = match concurrent_state
                        .get_mut(set)?
                        .waiting
                        .remove(&guest_task)
                        .unwrap()
                    {
                        WaitMode::Fiber(fiber) => WorkItem::ResumeFiber(fiber),
                        WaitMode::Callback(instance) => WorkItem::GuestCall(GuestCall {
                            task: guest_task,
                            kind: GuestCallKind::DeliverEvent {
                                instance,
                                set: None,
                            },
                        }),
                    };
                    concurrent_state.push_high_priority(item);

                    self.suspend(store, SuspendReason::Yielding { task: caller })?;
                }

                let concurrent_state = self.concurrent_state_mut(store);
                let task = concurrent_state.get_mut(guest_task)?;
                if task.lift_result.is_some() {
                    // Still not yet returned or cancelled; if `async_`, return
                    // `BLOCKED`; otherwise wait
                    if async_ {
                        return Ok(BLOCKED);
                    } else {
                        let waitable = Waitable::Guest(guest_task);
                        let old_set = waitable.common(concurrent_state)?.set;
                        let set = concurrent_state.get_mut(caller)?.sync_call_set;
                        waitable.join(concurrent_state, Some(set))?;

                        self.suspend(store, SuspendReason::Waiting { set, task: caller })?;

                        waitable.join(self.concurrent_state_mut(store), old_set)?;
                    }
                }
            }
        }

        let event = waitable.take_event(self.concurrent_state_mut(store))?;
        if let Some(Event::Subtask {
            status: status @ (Status::Returned | Status::ReturnCancelled),
        }) = event
        {
            Ok(status as u32)
        } else {
            bail!("`subtask.cancel` called after terminal status delivered");
        }
    }

    /// Configures TLS state so `store` will be available via `tls::get` within
    /// the closure `f` provided.
    ///
    /// This is used to ensure that `Future::poll`, which doesn't take a `store`
    /// parameter, is able to get access to the `store` during future poll
    /// methods.
    fn set_tls<R>(self, store: &mut dyn VMStore, f: impl FnOnce() -> R) -> R {
        struct Reset<'a>(&'a mut dyn VMStore, Option<ComponentInstanceId>);

        impl Drop for Reset<'_> {
            fn drop(&mut self) {
                self.0.concurrent_async_state_mut().current_instance = self.1;
            }
        }
        let prev = mem::replace(
            &mut store.concurrent_async_state_mut().current_instance,
            Some(self.id().instance()),
        );
        let reset = Reset(store, prev);

        tls::set(reset.0, f)
    }

    /// Convenience function to reduce boilerplate.
    pub(crate) fn concurrent_state_mut<'a>(
        &self,
        store: &'a mut StoreOpaque,
    ) -> &'a mut ConcurrentState {
        self.id().get_mut(store).concurrent_state_mut()
    }
}

/// Trait representing component model ABI async intrinsics and fused adapter
/// helper functions.
///
/// SAFETY (callers): Most of the methods in this trait accept raw pointers,
/// which must be valid for at least the duration of the call (and possibly for
/// as long as the relevant guest task exists, in the case of `*mut VMFuncRef`
/// pointers used for async calls).
pub trait VMComponentAsyncStore {
    /// A helper function for fused adapter modules involving calls where the
    /// one of the caller or callee is async.
    ///
    /// This helper is not used when the caller and callee both use the sync
    /// ABI, only when at least one is async is this used.
    unsafe fn prepare_call(
        &mut self,
        instance: Instance,
        memory: *mut VMMemoryDefinition,
        start: *mut VMFuncRef,
        return_: *mut VMFuncRef,
        caller_instance: RuntimeComponentInstanceIndex,
        callee_instance: RuntimeComponentInstanceIndex,
        task_return_type: TypeTupleIndex,
        string_encoding: u8,
        result_count: u32,
        storage: *mut ValRaw,
        storage_len: usize,
    ) -> Result<()>;

    /// A helper function for fused adapter modules involving calls where the
    /// caller is sync-lowered but the callee is async-lifted.
    unsafe fn sync_start(
        &mut self,
        instance: Instance,
        callback: *mut VMFuncRef,
        callee: *mut VMFuncRef,
        param_count: u32,
        storage: *mut MaybeUninit<ValRaw>,
        storage_len: usize,
    ) -> Result<()>;

    /// A helper function for fused adapter modules involving calls where the
    /// caller is async-lowered.
    unsafe fn async_start(
        &mut self,
        instance: Instance,
        callback: *mut VMFuncRef,
        post_return: *mut VMFuncRef,
        callee: *mut VMFuncRef,
        param_count: u32,
        result_count: u32,
        flags: u32,
    ) -> Result<u32>;

    /// The `future.write` intrinsic.
    fn future_write(
        &mut self,
        instance: Instance,
        ty: TypeFutureTableIndex,
        options: OptionsIndex,
        future: u32,
        address: u32,
    ) -> Result<u32>;

    /// The `future.read` intrinsic.
    fn future_read(
        &mut self,
        instance: Instance,
        ty: TypeFutureTableIndex,
        options: OptionsIndex,
        future: u32,
        address: u32,
    ) -> Result<u32>;

    /// The `future.drop-writable` intrinsic.
    fn future_drop_writable(
        &mut self,
        instance: Instance,
        ty: TypeFutureTableIndex,
        writer: u32,
    ) -> Result<()>;

    /// The `stream.write` intrinsic.
    fn stream_write(
        &mut self,
        instance: Instance,
        ty: TypeStreamTableIndex,
        options: OptionsIndex,
        stream: u32,
        address: u32,
        count: u32,
    ) -> Result<u32>;

    /// The `stream.read` intrinsic.
    fn stream_read(
        &mut self,
        instance: Instance,
        ty: TypeStreamTableIndex,
        options: OptionsIndex,
        stream: u32,
        address: u32,
        count: u32,
    ) -> Result<u32>;

    /// The "fast-path" implementation of the `stream.write` intrinsic for
    /// "flat" (i.e. memcpy-able) payloads.
    fn flat_stream_write(
        &mut self,
        instance: Instance,
        ty: TypeStreamTableIndex,
        options: OptionsIndex,
        payload_size: u32,
        payload_align: u32,
        stream: u32,
        address: u32,
        count: u32,
    ) -> Result<u32>;

    /// The "fast-path" implementation of the `stream.read` intrinsic for "flat"
    /// (i.e. memcpy-able) payloads.
    fn flat_stream_read(
        &mut self,
        instance: Instance,
        ty: TypeStreamTableIndex,
        options: OptionsIndex,
        payload_size: u32,
        payload_align: u32,
        stream: u32,
        address: u32,
        count: u32,
    ) -> Result<u32>;

    /// The `stream.drop-writable` intrinsic.
    fn stream_drop_writable(
        &mut self,
        instance: Instance,
        ty: TypeStreamTableIndex,
        writer: u32,
    ) -> Result<()>;

    /// The `error-context.debug-message` intrinsic.
    fn error_context_debug_message(
        &mut self,
        instance: Instance,
        ty: TypeComponentLocalErrorContextTableIndex,
        options: OptionsIndex,
        err_ctx_handle: u32,
        debug_msg_address: u32,
    ) -> Result<()>;
}

/// SAFETY: See trait docs.
impl<T: 'static> VMComponentAsyncStore for StoreInner<T> {
    unsafe fn prepare_call(
        &mut self,
        instance: Instance,
        memory: *mut VMMemoryDefinition,
        start: *mut VMFuncRef,
        return_: *mut VMFuncRef,
        caller_instance: RuntimeComponentInstanceIndex,
        callee_instance: RuntimeComponentInstanceIndex,
        task_return_type: TypeTupleIndex,
        string_encoding: u8,
        result_count_or_max_if_async: u32,
        storage: *mut ValRaw,
        storage_len: usize,
    ) -> Result<()> {
        // SAFETY: The `wasmtime_cranelift`-generated code that calls
        // this method will have ensured that `storage` is a valid
        // pointer containing at least `storage_len` items.
        let params = unsafe { std::slice::from_raw_parts(storage, storage_len) }.to_vec();

        unsafe {
            instance.prepare_call(
                StoreContextMut(self),
                start,
                return_,
                caller_instance,
                callee_instance,
                task_return_type,
                memory,
                string_encoding,
                match result_count_or_max_if_async {
                    PREPARE_ASYNC_NO_RESULT => CallerInfo::Async {
                        params,
                        has_result: false,
                    },
                    PREPARE_ASYNC_WITH_RESULT => CallerInfo::Async {
                        params,
                        has_result: true,
                    },
                    result_count => CallerInfo::Sync {
                        params,
                        result_count,
                    },
                },
            )
        }
    }

    unsafe fn sync_start(
        &mut self,
        instance: Instance,
        callback: *mut VMFuncRef,
        callee: *mut VMFuncRef,
        param_count: u32,
        storage: *mut MaybeUninit<ValRaw>,
        storage_len: usize,
    ) -> Result<()> {
        unsafe {
            instance
                .start_call(
                    StoreContextMut(self),
                    callback,
                    ptr::null_mut(),
                    callee,
                    param_count,
                    1,
                    START_FLAG_ASYNC_CALLEE,
                    // SAFETY: The `wasmtime_cranelift`-generated code that calls
                    // this method will have ensured that `storage` is a valid
                    // pointer containing at least `storage_len` items.
                    Some(std::slice::from_raw_parts_mut(storage, storage_len)),
                )
                .map(drop)
        }
    }

    unsafe fn async_start(
        &mut self,
        instance: Instance,
        callback: *mut VMFuncRef,
        post_return: *mut VMFuncRef,
        callee: *mut VMFuncRef,
        param_count: u32,
        result_count: u32,
        flags: u32,
    ) -> Result<u32> {
        unsafe {
            instance.start_call(
                StoreContextMut(self),
                callback,
                post_return,
                callee,
                param_count,
                result_count,
                flags,
                None,
            )
        }
    }

    fn future_write(
        &mut self,
        instance: Instance,
        ty: TypeFutureTableIndex,
        options: OptionsIndex,
        future: u32,
        address: u32,
    ) -> Result<u32> {
        instance
            .guest_write(
                StoreContextMut(self),
                TableIndex::Future(ty),
                options,
                None,
                future,
                address,
                1,
            )
            .map(|result| result.encode())
    }

    fn future_read(
        &mut self,
        instance: Instance,
        ty: TypeFutureTableIndex,
        options: OptionsIndex,
        future: u32,
        address: u32,
    ) -> Result<u32> {
        instance
            .guest_read(
                StoreContextMut(self),
                TableIndex::Future(ty),
                options,
                None,
                future,
                address,
                1,
            )
            .map(|result| result.encode())
    }

    fn stream_write(
        &mut self,
        instance: Instance,
        ty: TypeStreamTableIndex,
        options: OptionsIndex,
        stream: u32,
        address: u32,
        count: u32,
    ) -> Result<u32> {
        instance
            .guest_write(
                StoreContextMut(self),
                TableIndex::Stream(ty),
                options,
                None,
                stream,
                address,
                count,
            )
            .map(|result| result.encode())
    }

    fn stream_read(
        &mut self,
        instance: Instance,
        ty: TypeStreamTableIndex,
        options: OptionsIndex,
        stream: u32,
        address: u32,
        count: u32,
    ) -> Result<u32> {
        instance
            .guest_read(
                StoreContextMut(self),
                TableIndex::Stream(ty),
                options,
                None,
                stream,
                address,
                count,
            )
            .map(|result| result.encode())
    }

    fn future_drop_writable(
        &mut self,
        instance: Instance,
        ty: TypeFutureTableIndex,
        writer: u32,
    ) -> Result<()> {
        instance.guest_drop_writable(StoreContextMut(self), TableIndex::Future(ty), writer)
    }

    fn flat_stream_write(
        &mut self,
        instance: Instance,
        ty: TypeStreamTableIndex,
        options: OptionsIndex,
        payload_size: u32,
        payload_align: u32,
        stream: u32,
        address: u32,
        count: u32,
    ) -> Result<u32> {
        instance
            .guest_write(
                StoreContextMut(self),
                TableIndex::Stream(ty),
                options,
                Some(FlatAbi {
                    size: payload_size,
                    align: payload_align,
                }),
                stream,
                address,
                count,
            )
            .map(|result| result.encode())
    }

    fn flat_stream_read(
        &mut self,
        instance: Instance,
        ty: TypeStreamTableIndex,
        options: OptionsIndex,
        payload_size: u32,
        payload_align: u32,
        stream: u32,
        address: u32,
        count: u32,
    ) -> Result<u32> {
        instance
            .guest_read(
                StoreContextMut(self),
                TableIndex::Stream(ty),
                options,
                Some(FlatAbi {
                    size: payload_size,
                    align: payload_align,
                }),
                stream,
                address,
                count,
            )
            .map(|result| result.encode())
    }

    fn stream_drop_writable(
        &mut self,
        instance: Instance,
        ty: TypeStreamTableIndex,
        writer: u32,
    ) -> Result<()> {
        instance.guest_drop_writable(StoreContextMut(self), TableIndex::Stream(ty), writer)
    }

    fn error_context_debug_message(
        &mut self,
        instance: Instance,
        ty: TypeComponentLocalErrorContextTableIndex,
        options: OptionsIndex,
        err_ctx_handle: u32,
        debug_msg_address: u32,
    ) -> Result<()> {
        instance.error_context_debug_message(
            StoreContextMut(self),
            ty,
            options,
            err_ctx_handle,
            debug_msg_address,
        )
    }
}

/// Represents the output of a host task or background task.
pub(crate) enum HostTaskOutput {
    /// A plain result
    Result(Result<()>),
    /// A function to be run after the future completes (e.g. post-processing
    /// which requires access to the store and instance).
    Function(Box<dyn FnOnce(&mut dyn VMStore, Instance) -> Result<()> + Send>),
}

impl HostTaskOutput {
    /// Retrieve the result of the host or background task, running the
    /// post-processing function if present.
    fn consume(self, store: &mut dyn VMStore, instance: Instance) -> Result<()> {
        match self {
            Self::Function(fun) => fun(store, instance),
            Self::Result(result) => result,
        }
    }
}

type HostTaskFuture = Pin<Box<dyn Future<Output = HostTaskOutput> + Send + 'static>>;

/// Represents the state of a pending host task.
struct HostTask {
    common: WaitableCommon,
    caller_instance: RuntimeComponentInstanceIndex,
    abort_handle: Option<AbortHandle>,
}

impl HostTask {
    fn new(
        caller_instance: RuntimeComponentInstanceIndex,
        abort_handle: Option<AbortHandle>,
    ) -> Self {
        Self {
            common: WaitableCommon::default(),
            caller_instance,
            abort_handle,
        }
    }
}

impl TableDebug for HostTask {
    fn type_name() -> &'static str {
        "HostTask"
    }
}

type CallbackFn = Box<
    dyn Fn(&mut dyn VMStore, Instance, RuntimeComponentInstanceIndex, Event, u32) -> Result<u32>
        + Send
        + Sync
        + 'static,
>;

/// Represents the caller of a given guest task.
enum Caller {
    /// The host called the guest task.
    Host {
        /// If present, may be used to deliver the result.
        tx: Option<oneshot::Sender<LiftedResult>>,
        /// If true, remove the task from the concurrent state that owns it
        /// automatically after it completes.
        remove_task_automatically: bool,
        /// If true, call `post-return` function (if any) automatically.
        call_post_return_automatically: bool,
    },
    /// Another guest task called the guest task
    Guest {
        /// The id of the caller
        task: TableId<GuestTask>,
        /// The instance to use to enforce reentrance rules.
        ///
        /// Note that this might not be the same as the instance the caller task
        /// started executing in given that one or more synchronous guest->guest
        /// calls may have occurred involving multiple instances.
        instance: RuntimeComponentInstanceIndex,
    },
}

/// Represents a closure and related canonical ABI parameters required to
/// validate a `task.return` call at runtime and lift the result.
struct LiftResult {
    lift: RawLift,
    ty: TypeTupleIndex,
    memory: Option<SendSyncPtr<VMMemoryDefinition>>,
    string_encoding: StringEncoding,
}

/// Represents a pending guest task.
struct GuestTask {
    /// See `WaitableCommon`
    common: WaitableCommon,
    /// Closure to lower the parameters passed to this task.
    lower_params: Option<RawLower>,
    /// See `LiftResult`
    lift_result: Option<LiftResult>,
    /// A place to stash the type-erased lifted result if it can't be delivered
    /// immediately.
    result: Option<LiftedResult>,
    /// Closure to call the callback function for an async-lifted export, if
    /// provided.
    callback: Option<CallbackFn>,
    /// See `Caller`
    caller: Caller,
    /// A place to stash the call context for managing resource borrows while
    /// switching between guest tasks.
    call_context: Option<CallContext>,
    /// A place to stash the lowered result for a sync-to-async call until it
    /// can be returned to the caller.
    sync_result: Option<Option<ValRaw>>,
    /// Whether or not the task has been cancelled (i.e. whether the task is
    /// permitted to call `task.cancel`).
    cancel_sent: bool,
    /// Whether or not we've sent a `Status::Starting` event to any current or
    /// future waiters for this waitable.
    starting_sent: bool,
    /// Context-local state used to implement the `context.{get,set}`
    /// intrinsics.
    context: [u32; 2],
    /// Pending guest subtasks created by this task (directly or indirectly).
    ///
    /// This is used to re-parent subtasks which are still running when their
    /// parent task is disposed.
    subtasks: HashSet<TableId<GuestTask>>,
    /// Scratch waitable set used to watch subtasks during synchronous calls.
    sync_call_set: TableId<WaitableSet>,
    /// The instance to which the exported function for this guest task belongs.
    ///
    /// Note that the task may do a sync->sync call via a fused adapter which
    /// results in that task executing code in a different instance, and it may
    /// call host functions and intrinsics from that other instance.
    instance: RuntimeComponentInstanceIndex,
    /// If present, a pending `Event::None` or `Event::Cancelled` to be
    /// delivered to this task.
    event: Option<Event>,
    /// If present, indicates that the task is currently waiting on the
    /// specified set but may be cancelled and woken immediately.
    wake_on_cancel: Option<TableId<WaitableSet>>,
    /// The `ExportIndex` of the guest function being called, if known.
    function_index: Option<ExportIndex>,
    /// Whether or not the task has exited.
    exited: bool,
}

impl GuestTask {
    fn new(
        state: &mut ConcurrentState,
        lower_params: RawLower,
        lift_result: LiftResult,
        caller: Caller,
        callback: Option<CallbackFn>,
        component_instance: RuntimeComponentInstanceIndex,
    ) -> Result<Self> {
        let sync_call_set = state.push(WaitableSet::default())?;

        Ok(Self {
            common: WaitableCommon::default(),
            lower_params: Some(lower_params),
            lift_result: Some(lift_result),
            result: None,
            callback,
            caller,
            call_context: Some(CallContext::default()),
            sync_result: None,
            cancel_sent: false,
            starting_sent: false,
            context: [0u32; 2],
            subtasks: HashSet::new(),
            sync_call_set,
            instance: component_instance,
            event: None,
            wake_on_cancel: None,
            function_index: None,
            exited: false,
        })
    }

    /// Dispose of this guest task, reparenting any pending subtasks to the
    /// caller.
    fn dispose(self, state: &mut ConcurrentState, me: TableId<GuestTask>) -> Result<()> {
        // If there are not-yet-delivered completion events for subtasks in
        // `self.sync_call_set`, recursively dispose of those subtasks as well.
        for waitable in mem::take(&mut state.get_mut(self.sync_call_set)?.ready) {
            if let Some(Event::Subtask {
                status: Status::Returned | Status::ReturnCancelled,
            }) = waitable.common(state)?.event
            {
                waitable.delete_from(state)?;
            }
        }

        state.delete(self.sync_call_set)?;

        // Reparent any pending subtasks to the caller.
        if let Caller::Guest {
            task,
            instance: runtime_instance,
        } = &self.caller
        {
            let task_mut = state.get_mut(*task)?;
            let present = task_mut.subtasks.remove(&me);
            assert!(present);

            for subtask in &self.subtasks {
                task_mut.subtasks.insert(*subtask);
            }

            for subtask in &self.subtasks {
                state.get_mut(*subtask)?.caller = Caller::Guest {
                    task: *task,
                    instance: *runtime_instance,
                };
            }
        } else {
            for subtask in &self.subtasks {
                state.get_mut(*subtask)?.caller = Caller::Host {
                    tx: None,
                    remove_task_automatically: true,
                    call_post_return_automatically: true,
                };
            }
        }

        Ok(())
    }

    fn call_post_return_automatically(&self) -> bool {
        matches!(
            self.caller,
            Caller::Guest { .. }
                | Caller::Host {
                    call_post_return_automatically: true,
                    ..
                }
        )
    }
}

impl TableDebug for GuestTask {
    fn type_name() -> &'static str {
        "GuestTask"
    }
}

/// Represents state common to all kinds of waitables.
#[derive(Default)]
struct WaitableCommon {
    /// The currently pending event for this waitable, if any.
    event: Option<Event>,
    /// The set to which this waitable belongs, if any.
    set: Option<TableId<WaitableSet>>,
}

/// Represents a Component Model Async `waitable`.
#[derive(Copy, Clone, Ord, PartialOrd, Eq, PartialEq)]
enum Waitable {
    /// A host task
    Host(TableId<HostTask>),
    /// A guest task
    Guest(TableId<GuestTask>),
    /// The read or write end of a stream or future
    Transmit(TableId<TransmitHandle>),
}

impl Waitable {
    /// Retrieve the `Waitable` corresponding to the specified guest-visible
    /// handle.
    fn from_instance(
        state: &mut ConcurrentState,
        caller_instance: RuntimeComponentInstanceIndex,
        waitable: u32,
    ) -> Result<Self> {
        let (waitable, state) =
            state.waitable_tables[caller_instance].get_mut_by_index(waitable)?;

        Ok(match state {
            WaitableState::HostTask => Waitable::Host(TableId::new(waitable)),
            WaitableState::GuestTask => Waitable::Guest(TableId::new(waitable)),
            WaitableState::Stream(..) | WaitableState::Future(..) => {
                Waitable::Transmit(TableId::new(waitable))
            }
            _ => bail!("invalid waitable handle"),
        })
    }

    /// Retrieve the host-visible identifier for this `Waitable`.
    fn rep(&self) -> u32 {
        match self {
            Self::Host(id) => id.rep(),
            Self::Guest(id) => id.rep(),
            Self::Transmit(id) => id.rep(),
        }
    }

    /// Move this `Waitable` to the specified set (when `set` is `Some(_)`) or
    /// remove it from any set it may currently belong to (when `set` is
    /// `None`).
    fn join(&self, state: &mut ConcurrentState, set: Option<TableId<WaitableSet>>) -> Result<()> {
        log::trace!("waitable {self:?} join set {set:?}",);

        let old = mem::replace(&mut self.common(state)?.set, set);

        if let Some(old) = old {
            match *self {
                Waitable::Host(id) => state.remove_child(id, old),
                Waitable::Guest(id) => state.remove_child(id, old),
                Waitable::Transmit(id) => state.remove_child(id, old),
            }?;

            state.get_mut(old)?.ready.remove(self);
        }

        if let Some(set) = set {
            match *self {
                Waitable::Host(id) => state.add_child(id, set),
                Waitable::Guest(id) => state.add_child(id, set),
                Waitable::Transmit(id) => state.add_child(id, set),
            }?;

            if self.common(state)?.event.is_some() {
                self.mark_ready(state)?;
            }
        }

        Ok(())
    }

    /// Retrieve mutable access to the `WaitableCommon` for this `Waitable`.
    fn common<'a>(&self, state: &'a mut ConcurrentState) -> Result<&'a mut WaitableCommon> {
        Ok(match self {
            Self::Host(id) => &mut state.get_mut(*id)?.common,
            Self::Guest(id) => &mut state.get_mut(*id)?.common,
            Self::Transmit(id) => &mut state.get_mut(*id)?.common,
        })
    }

    /// Set or clear the pending event for this waitable and either deliver it
    /// to the first waiter, if any, or mark it as ready to be delivered to the
    /// next waiter that arrives.
    fn set_event(&self, state: &mut ConcurrentState, event: Option<Event>) -> Result<()> {
        log::trace!("set event for {self:?}: {event:?}");
        self.common(state)?.event = event;
        self.mark_ready(state)
    }

    /// Take the pending event from this waitable, leaving `None` in its place.
    fn take_event(&self, state: &mut ConcurrentState) -> Result<Option<Event>> {
        let common = self.common(state)?;
        let event = common.event.take();
        if let Some(set) = self.common(state)?.set {
            state.get_mut(set)?.ready.remove(self);
        }
        Ok(event)
    }

    /// Deliver the current event for this waitable to the first waiter, if any,
    /// or else mark it as ready to be delivered to the next waiter that
    /// arrives.
    fn mark_ready(&self, state: &mut ConcurrentState) -> Result<()> {
        if let Some(set) = self.common(state)?.set {
            state.get_mut(set)?.ready.insert(*self);
            if let Some((task, mode)) = state.get_mut(set)?.waiting.pop_first() {
                let wake_on_cancel = state.get_mut(task)?.wake_on_cancel.take();
                assert!(wake_on_cancel.is_none() || wake_on_cancel == Some(set));

                let item = match mode {
                    WaitMode::Fiber(fiber) => WorkItem::ResumeFiber(fiber),
                    WaitMode::Callback(instance) => WorkItem::GuestCall(GuestCall {
                        task,
                        kind: GuestCallKind::DeliverEvent {
                            instance,
                            set: Some(set),
                        },
                    }),
                };
                state.push_high_priority(item);
            }
        }
        Ok(())
    }

    /// Handle the imminent delivery of the specified event, e.g. by updating
    /// the state of the stream or future.
    fn on_delivery(&self, state: &mut ConcurrentState, event: Event) {
        match event {
            Event::FutureRead {
                pending: Some((ty, handle)),
                ..
            }
            | Event::FutureWrite {
                pending: Some((ty, handle)),
                ..
            } => {
                let runtime_instance = state.component.types()[ty].instance;
                let (rep, WaitableState::Future(actual_ty, state)) = state.waitable_tables
                    [runtime_instance]
                    .get_mut_by_index(handle)
                    .unwrap()
                else {
                    unreachable!()
                };
                assert_eq!(*actual_ty, ty);
                assert_eq!(rep, self.rep());
                assert_eq!(*state, StreamFutureState::Busy);
                *state = match event {
                    Event::FutureRead { .. } => StreamFutureState::Read { done: false },
                    Event::FutureWrite { .. } => StreamFutureState::Write { done: false },
                    _ => unreachable!(),
                };
            }
            Event::StreamRead {
                pending: Some((ty, handle)),
                code,
            }
            | Event::StreamWrite {
                pending: Some((ty, handle)),
                code,
            } => {
                let runtime_instance = state.component.types()[ty].instance;
                let (rep, WaitableState::Stream(actual_ty, state)) = state.waitable_tables
                    [runtime_instance]
                    .get_mut_by_index(handle)
                    .unwrap()
                else {
                    unreachable!()
                };
                assert_eq!(*actual_ty, ty);
                assert_eq!(rep, self.rep());
                assert_eq!(*state, StreamFutureState::Busy);
                let done = matches!(code, ReturnCode::Dropped(_));
                *state = match event {
                    Event::StreamRead { .. } => StreamFutureState::Read { done },
                    Event::StreamWrite { .. } => StreamFutureState::Write { done },
                    _ => unreachable!(),
                };
            }
            _ => {}
        }
    }

    /// Remove this waitable from the instance's rep table.
    fn delete_from(&self, state: &mut ConcurrentState) -> Result<()> {
        match self {
            Self::Host(task) => {
                log::trace!("delete host task {task:?}");
                state.delete(*task)?;
            }
            Self::Guest(task) => {
                log::trace!("delete guest task {task:?}");
                state.delete(*task)?.dispose(state, *task)?;
            }
            Self::Transmit(task) => {
                state.delete(*task)?;
            }
        }

        Ok(())
    }
}

impl fmt::Debug for Waitable {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Self::Host(id) => write!(f, "{id:?}"),
            Self::Guest(id) => write!(f, "{id:?}"),
            Self::Transmit(id) => write!(f, "{id:?}"),
        }
    }
}

/// Represents a Component Model Async `waitable-set`.
#[derive(Default)]
struct WaitableSet {
    /// Which waitables in this set have pending events, if any.
    ready: BTreeSet<Waitable>,
    /// Which guest tasks are currently waiting on this set, if any.
    waiting: BTreeMap<TableId<GuestTask>, WaitMode>,
}

impl TableDebug for WaitableSet {
    fn type_name() -> &'static str {
        "WaitableSet"
    }
}

/// Type-erased closure to lower the parameters for a guest task.
type RawLower = Box<
    dyn FnOnce(&mut dyn VMStore, Instance, &mut [MaybeUninit<ValRaw>]) -> Result<()> + Send + Sync,
>;

/// Type-erased closure to lift the result for a guest task.
type RawLift = Box<
    dyn FnOnce(&mut dyn VMStore, Instance, &[ValRaw]) -> Result<Box<dyn Any + Send + Sync>>
        + Send
        + Sync,
>;

/// Type erased result of a guest task which may be downcast to the expected
/// type by a host caller (or simply ignored in the case of a guest caller; see
/// `DummyResult`).
type LiftedResult = Box<dyn Any + Send + Sync>;

/// Used to return a result from a `LiftFn` when the actual result has already
/// been lowered to a guest task's stack and linear memory.
struct DummyResult;

/// Represents the state of a currently executing fiber which has been resumed
/// via `self::poll_fn`.
pub(crate) struct AsyncState {
    /// The current instance being polled, if any, which is used to perform
    /// checks to ensure that futures are always polled within the correct
    /// instance.
    current_instance: Option<ComponentInstanceId>,
}

impl Default for AsyncState {
    fn default() -> Self {
        Self {
            current_instance: None,
        }
    }
}

/// Represents the Component Model Async state of a (sub-)component instance.
#[derive(Default)]
struct InstanceState {
    /// Whether backpressure is set for this instance
    backpressure: bool,
    /// Whether this instance can be entered
    do_not_enter: bool,
    /// Pending calls for this instance which require `Self::backpressure` to be
    /// `true` and/or `Self::do_not_enter` to be false before they can proceed.
    pending: BTreeMap<TableId<GuestTask>, GuestCallKind>,
}

/// Represents the Component Model Async state of a top-level component instance
/// (i.e. a `super::ComponentInstance`).
pub struct ConcurrentState {
    /// The currently running guest task, if any.
    guest_task: Option<TableId<GuestTask>>,
    /// The set of pending host and background tasks, if any.
    ///
    /// We must wrap this in a `Mutex` to ensure that `ComponentInstance` and
    /// `Store` satisfy a `Sync` bound, but it can't actually be accessed from
    /// more than one thread at a time.
    ///
    /// See `ComponentInstance::poll_until` for where we temporarily take this
    /// out, poll it, then put it back to avoid any mutable aliasing hazards.
    futures: Mutex<Option<FuturesUnordered<HostTaskFuture>>>,
    /// The table of waitables, waitable sets, etc.
    table: Table,
    /// Per (sub-)component instance states.
    ///
    /// See `InstanceState` for details and note that this map is lazily
    /// populated as needed.
    // TODO: this can and should be a `PrimaryMap`
    instance_states: HashMap<RuntimeComponentInstanceIndex, InstanceState>,
    /// Tables for tracking per-(sub-)component waitable handles and their
    /// states.
    waitable_tables: PrimaryMap<RuntimeComponentInstanceIndex, StateTable<WaitableState>>,
    /// The "high priority" work queue for this instance's event loop.
    high_priority: Vec<WorkItem>,
    /// The "high priority" work queue for this instance's event loop.
    low_priority: Vec<WorkItem>,
    /// A place to stash the reason a fiber is suspending so that the code which
    /// resumed it will know under what conditions the fiber should be resumed
    /// again.
    suspend_reason: Option<SuspendReason>,
    /// A cached fiber which is waiting for work to do.
    ///
    /// This helps us avoid creating a new fiber for each `GuestCall` work item.
    worker: Option<StoreFiber<'static>>,
    /// A place to stash the work item for which we're resuming a worker fiber.
    worker_item: Option<WorkerItem>,

    /// (Sub)Component specific error context tracking
    ///
    /// At the component level, only the number of references (`usize`) to a given error context is tracked,
    /// with state related to the error context being held at the component model level, in concurrent
    /// state.
    ///
    /// The state tables in the (sub)component local tracking must contain a pointer into the global
    /// error context lookups in order to ensure that in contexts where only the local reference is present
    /// the global state can still be maintained/updated.
    error_context_tables:
        PrimaryMap<TypeComponentLocalErrorContextTableIndex, StateTable<LocalErrorContextRefCount>>,

    /// Reference counts for all component error contexts
    ///
    /// NOTE: it is possible the global ref count to be *greater* than the sum of
    /// (sub)component ref counts as tracked by `error_context_tables`, for
    /// example when the host holds one or more references to error contexts.
    ///
    /// The key of this primary map is often referred to as the "rep" (i.e. host-side
    /// component-wide representation) of the index into concurrent state for a given
    /// stored `ErrorContext`.
    ///
    /// Stated another way, `TypeComponentGlobalErrorContextTableIndex` is essentially the same
    /// as a `TableId<ErrorContextState>`.
    global_error_context_ref_counts:
        BTreeMap<TypeComponentGlobalErrorContextTableIndex, GlobalErrorContextRefCount>,

    /// Mirror of type information in `ComponentInstance`, placed here for
    /// convenience at the cost of an extra `Arc` clone.
    component: Component,
}

impl ConcurrentState {
    pub(crate) fn new(component: &Component) -> Self {
        let num_waitable_tables = component.env_component().num_runtime_component_instances;
        let num_error_context_tables = component.env_component().num_error_context_tables;
        let mut waitable_tables =
            PrimaryMap::with_capacity(usize::try_from(num_waitable_tables).unwrap());
        for _ in 0..num_waitable_tables {
            waitable_tables.push(StateTable::default());
        }

        let mut error_context_tables = PrimaryMap::<
            TypeComponentLocalErrorContextTableIndex,
            StateTable<LocalErrorContextRefCount>,
        >::with_capacity(num_error_context_tables);
        for _ in 0..num_error_context_tables {
            error_context_tables.push(StateTable::default());
        }

        Self {
            guest_task: None,
            table: Table::new(),
            futures: Mutex::new(Some(FuturesUnordered::new())),
            instance_states: HashMap::new(),
            waitable_tables,
            high_priority: Vec::new(),
            low_priority: Vec::new(),
            suspend_reason: None,
            worker: None,
            worker_item: None,
            error_context_tables,
            global_error_context_ref_counts: BTreeMap::new(),
            component: component.clone(),
        }
    }

    /// Take ownership of any fibers and futures owned by this object.
    ///
    /// This should be used when disposing of the `Store` containing this object
    /// in order to gracefully resolve any and all fibers using
    /// `StoreFiber::dispose`.  This is necessary to avoid possible
    /// use-after-free bugs due to fibers which may still have access to the
    /// `Store`.
    ///
    /// Additionally, the futures collected with this function should be dropped
    /// within a `tls::set` call, which will ensure than any futures closing
    /// over an `&Accessor` will have access to the store when dropped, allowing
    /// e.g. `WithAccessor[AndValue]` instances to be disposed of without
    /// panicking.
    ///
    /// Note that this will leave the object in an inconsistent and unusable
    /// state, so it should only be used just prior to dropping it.
    pub(crate) fn take_fibers_and_futures(
        &mut self,
        fibers: &mut Vec<StoreFiber<'static>>,
        futures: &mut Vec<FuturesUnordered<HostTaskFuture>>,
    ) {
        for entry in self.table.iter_mut() {
            if let Some(set) = entry.downcast_mut::<WaitableSet>() {
                for mode in mem::take(&mut set.waiting).into_values() {
                    if let WaitMode::Fiber(fiber) = mode {
                        fibers.push(fiber);
                    }
                }
            }
        }

        if let Some(fiber) = self.worker.take() {
            fibers.push(fiber);
        }

        let mut take_items = |list| {
            for item in mem::take(list) {
                match item {
                    WorkItem::ResumeFiber(fiber) => {
                        fibers.push(fiber);
                    }
                    WorkItem::PushFuture(future) => {
                        self.futures
                            .get_mut()
                            .unwrap()
                            .as_mut()
                            .unwrap()
                            .push(future.into_inner().unwrap());
                    }
                    _ => {}
                }
            }
        };

        take_items(&mut self.high_priority);
        take_items(&mut self.low_priority);

        if let Some(them) = self.futures.get_mut().unwrap().take() {
            futures.push(them);
        }
    }
}

/// Provide a type hint to compiler about the shape of a parameter lower
/// closure.
fn for_any_lower<
    F: FnOnce(&mut dyn VMStore, Instance, &mut [MaybeUninit<ValRaw>]) -> Result<()> + Send + Sync,
>(
    fun: F,
) -> F {
    fun
}

/// Provide a type hint to compiler about the shape of a result lift closure.
fn for_any_lift<
    F: FnOnce(&mut dyn VMStore, Instance, &[ValRaw]) -> Result<Box<dyn Any + Send + Sync>>
        + Send
        + Sync,
>(
    fun: F,
) -> F {
    fun
}

/// Wrap the specified future in a `poll_fn` which asserts that the future is
/// only polled from the event loop of the specified `Instance`.
///
/// See `Instance::run_concurrent` for details.
fn checked<F: Future + Send + 'static>(
    instance: Instance,
    fut: F,
) -> impl Future<Output = F::Output> + Send + 'static {
    async move {
        let mut fut = pin!(fut);
        future::poll_fn(move |cx| {
            let message = "\
                `Future`s which depend on asynchronous component tasks, streams, or \
                futures to complete may only be polled from the event loop of the \
                instance from which they originated.  Please use \
                `Instance::{run_concurrent,spawn}` to poll or await them.\
            ";
            tls::try_get(|store| {
                let matched = match store {
                    tls::TryGet::Some(store) => {
                        let a = store.concurrent_async_state_mut().current_instance;
                        a == Some(instance.id().instance())
                    }
                    tls::TryGet::Taken | tls::TryGet::None => false,
                };

                if !matched {
                    panic!("{message}")
                }
            });
            fut.as_mut().poll(cx)
        })
        .await
    }
}

/// Assert that `Instance::run_concurrent` has not been called from within an
/// instance's event loop.
fn check_recursive_run() {
    tls::try_get(|store| {
        if !matches!(store, tls::TryGet::None) {
            panic!("Recursive `Instance::run_concurrent` calls not supported")
        }
    });
}

fn unpack_callback_code(code: u32) -> (u32, u32) {
    (code & 0xF, code >> 4)
}

/// Helper struct for packaging parameters to be passed to
/// `ComponentInstance::waitable_check` for calls to `waitable-set.wait` or
/// `waitable-set.poll`.
struct WaitableCheckParams {
    set: TableId<WaitableSet>,
    caller_instance: RuntimeComponentInstanceIndex,
    options: OptionsIndex,
    payload: u32,
}

/// Helper enum for passing parameters to `ComponentInstance::waitable_check`.
enum WaitableCheck {
    Wait(WaitableCheckParams),
    Poll(WaitableCheckParams),
    Yield,
}

/// Represents a guest task called from the host, prepared using `prepare_call`.
pub(crate) struct PreparedCall<R> {
    /// The guest export to be called
    handle: Func,
    /// The guest task created by `prepare_call`
    task: TableId<GuestTask>,
    /// The number of lowered core Wasm parameters to pass to the call.
    param_count: usize,
    /// The `oneshot::Receiver` to which the result of the call will be
    /// delivered when it is available.
    rx: oneshot::Receiver<LiftedResult>,
    _phantom: PhantomData<R>,
}

impl<R> PreparedCall<R> {
    /// Get a copy of the `TaskId` for this `PreparedCall`.
    pub(crate) fn task_id(&self) -> TaskId {
        TaskId {
            handle: self.handle,
            task: self.task,
        }
    }
}

/// Represents a task created by `prepare_call`.
pub(crate) struct TaskId {
    handle: Func,
    task: TableId<GuestTask>,
}

impl TaskId {
    /// Remove the specified task from the concurrent state to which it belongs.
    ///
    /// This must be used with care to avoid use-after-delete or double-delete
    /// bugs.  Specifically, it should only be called on tasks created with the
    /// `remove_task_automatically` parameter to `prepare_call` set to `false`,
    /// which tells the runtime that the caller is responsible for removing the
    /// task from the state; otherwise, it will be removed automatically.  Also,
    /// it should only be called once for a given task, and only after either
    /// the task has completed or the instance has trapped.
    pub(crate) fn remove<T>(&self, store: StoreContextMut<T>) -> Result<()> {
        Waitable::Guest(self.task).delete_from(self.handle.instance().concurrent_state_mut(store.0))
    }
}

/// Prepare a call to the specified exported Wasm function, providing functions
/// for lowering the parameters and lifting the result.
///
/// To enqueue the returned `PreparedCall` in the `ComponentInstance`'s event
/// loop, use `queue_call`.
pub(crate) fn prepare_call<T, R>(
    mut store: StoreContextMut<T>,
    handle: Func,
    param_count: usize,
    remove_task_automatically: bool,
    call_post_return_automatically: bool,
    lower_params: impl FnOnce(Func, StoreContextMut<T>, &mut [MaybeUninit<ValRaw>]) -> Result<()>
    + Send
    + Sync
    + 'static,
    lift_result: impl FnOnce(Func, &mut StoreOpaque, &[ValRaw]) -> Result<Box<dyn Any + Send + Sync>>
    + Send
    + Sync
    + 'static,
) -> Result<PreparedCall<R>> {
    let (options, _flags, ty, raw_options) = handle.abi_info(store.0);

    let instance = handle.instance().id().get(store.0);
    let task_return_type = instance.component().types()[ty].results;
    let component_instance = raw_options.instance;
    let callback = options.callback();
    let memory = options.memory_raw().map(SendSyncPtr::new);
    let string_encoding = options.string_encoding();
    let token = StoreToken::new(store.as_context_mut());
    let state = handle.instance().concurrent_state_mut(store.0);

    assert!(state.guest_task.is_none());

    let (tx, rx) = oneshot::channel();

    let mut task = GuestTask::new(
        state,
        Box::new(for_any_lower(move |store, instance, params| {
            debug_assert!(instance.id() == handle.instance().id());
            lower_params(handle, token.as_context_mut(store), params)
        })),
        LiftResult {
            lift: Box::new(for_any_lift(move |store, instance, result| {
                debug_assert!(instance.id() == handle.instance().id());
                lift_result(handle, store, result)
            })),
            ty: task_return_type,
            memory,
            string_encoding,
        },
        Caller::Host {
            tx: Some(tx),
            remove_task_automatically,
            call_post_return_automatically,
        },
        callback.map(|callback| {
            let callback = SendSyncPtr::new(callback);
            Box::new(
                move |store: &mut dyn VMStore,
                      instance: Instance,
                      runtime_instance,
                      event,
                      handle| {
                    let store = token.as_context_mut(store);
                    // SAFETY: Per the contract of `prepare_call`, the callback
                    // will remain valid at least as long is this task exists.
                    unsafe {
                        instance.call_callback(
                            store,
                            runtime_instance,
                            callback,
                            event,
                            handle,
                            call_post_return_automatically,
                        )
                    }
                },
            ) as CallbackFn
        }),
        component_instance,
    )?;
    task.function_index = Some(handle.index());

    let task = state.push(task)?;

    Ok(PreparedCall {
        handle,
        task,
        param_count,
        rx,
        _phantom: PhantomData,
    })
}

/// Queue a call previously prepared using `prepare_call` to be run as part of
/// the associated `ComponentInstance`'s event loop.
///
/// The returned future will resolve to the result once it is available, but
/// must only be polled via the instance's event loop. See
/// `Instance::run_concurrent` for details.
pub(crate) fn queue_call<T: 'static, R: Send + 'static>(
    mut store: StoreContextMut<T>,
    prepared: PreparedCall<R>,
) -> Result<impl Future<Output = Result<R>> + Send + 'static + use<T, R>> {
    let PreparedCall {
        handle,
        task,
        param_count,
        rx,
        ..
    } = prepared;

    queue_call0(store.as_context_mut(), handle, task, param_count)?;

    Ok(checked(
        handle.instance(),
        rx.map(|result| {
            result
                .map(|v| *v.downcast().unwrap())
                .map_err(anyhow::Error::from)
        }),
    ))
}

/// Queue a call previously prepared using `prepare_call` to be run as part of
/// the associated `ComponentInstance`'s event loop.
fn queue_call0<T: 'static>(
    store: StoreContextMut<T>,
    handle: Func,
    guest_task: TableId<GuestTask>,
    param_count: usize,
) -> Result<()> {
    let (options, flags, _ty, raw_options) = handle.abi_info(store.0);
    let is_concurrent = raw_options.async_;
    let instance = handle.instance();
    let callee = handle.lifted_core_func(store.0);
    let callback = options.callback();
    let post_return = handle.post_return_core_func(store.0);

    log::trace!("queueing call {guest_task:?}");

    let instance_flags = if callback.is_none() {
        None
    } else {
        Some(flags)
    };

    // SAFETY: `callee`, `callback`, and `post_return` are valid pointers
    // (with signatures appropriate for this call) and will remain valid as
    // long as this instance is valid.
    unsafe {
        instance.queue_call(
            store,
            guest_task,
            SendSyncPtr::new(callee),
            param_count,
            1,
            instance_flags,
            is_concurrent,
            callback.map(SendSyncPtr::new),
            post_return.map(SendSyncPtr::new),
        )
    }
}
