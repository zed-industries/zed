use crate::prelude::*;
use crate::store::{Executor, StoreId, StoreInner, StoreOpaque};
use crate::vm::mpk::{self, ProtectionMask};
use crate::vm::{AsyncWasmCallState, VMStore};
use crate::{Engine, StoreContextMut};
use anyhow::{Result, anyhow};
use core::mem;
use core::ops::Range;
use core::pin::Pin;
use core::ptr::{self, NonNull};
use core::task::{Context, Poll};
use wasmtime_fiber::{Fiber, Suspend};

type WasmtimeResume = Result<NonNull<Context<'static>>>;
type WasmtimeYield = StoreFiberYield;
type WasmtimeComplete = Result<()>;
type WasmtimeSuspend = Suspend<WasmtimeResume, WasmtimeYield, WasmtimeComplete>;

/// State related to asynchronous computations stored within a `Store<T>`.
///
/// This structure resides inside of a `Store<T>` and is used to manage the
/// various pieces of state associated with asynchronous computations. Chiefly
/// this manages the `WasmtimeSuspend` pointer as well as `&mut Context<'_>`
/// when polling futures. This serves as storage to use these pointers across a
/// WebAssembly function boundary, for example, where the values cannot
/// otherwise be explicitly threaded through.
pub(crate) struct AsyncState {
    /// The `Suspend` for the current fiber (or null if no such fiber is
    /// running).
    ///
    /// This pointer is provided by the `wasmtime_fiber` crate when a fiber
    /// first starts, but this pointer is unable to be carried through
    /// WebAssembly frames for example. This serves as an alternative storage
    /// location for the pointer provided by `wasmtime_fiber` within a fiber's
    /// execution.
    ///
    /// This pointer is null when a fiber is not executing, but it is also null
    /// when a `BlockingContext` is created. Note that when a fiber is suspended
    /// it's always through a `BlockingContext` so this field is null whenever a
    /// fiber is suspended as well. Fiber resumption will save the prior value
    /// in a store and then set it to null, where suspension will then restore
    /// what was previously in the store.
    current_suspend: Option<NonNull<WasmtimeSuspend>>,

    /// The `Context` pointer last provided in `Future for FiberFuture`.
    ///
    /// Like `current_suspend` above this is an example of a piece of context
    /// which needs to be carried over a WebAssembly function frame which
    /// otherwise doesn't take this as a parameter. This differs from
    /// `current_suspend` though in that it is provided as part of a `Future`
    /// poll operation but is "gone" after that poll operation completes. That
    /// means that while `current_suspend` is the same for the lifetime of a
    /// future this field is always changing.
    ///
    /// Like `current_suspend` though this is null either when a fiber isn't
    /// running or when a `BlockingContext` is created (in which case this is
    /// "take"en). That means that this is null on suspension/resumption of a
    /// fiber.
    ///
    /// The value for this pointer is threaded directly through the
    /// `WasmtimeResume` type which is how a pointer flows into this field from
    /// a future-related poll call. This means that the `BlockingContext`
    /// creation may take one value of a pointer here but restore another. That
    /// would represent suspending in one call to `Future::poll` and then
    /// resuming some time later in a different call to `Future::poll`.
    ///
    /// # Safety
    ///
    /// Note that this is a pretty unsafe field for two reasons. One is that
    /// it's a raw pointer to a `Context` provided ephemerally to some call to
    /// `Future::poll` on the stack. Another reason is that the lifetime
    /// parameter of `Context` is unsafely changed to `'static` here which is
    /// not correct. The ephemeral nature of this pointer is managed through the
    /// take-style operations in `BlockingContext` and the `'static` lifetime is
    /// handled by ensuring the signatures that work with `BlockingContext` all
    /// use constrained anonymous lifetimes that are guaranteed to be shorter
    /// than the original `Context` lifetime.
    current_future_cx: Option<NonNull<Context<'static>>>,

    /// The last fiber stack that was in use by the store.
    ///
    /// We use this to cache and reuse stacks as a performance optimization.
    // TODO: With stack switching and the Component Model Async ABI, there may
    // be multiple concurrent fibers in play; consider caching more than one
    // stack at a time and making the number tunable via `Config`.
    last_fiber_stack: Option<wasmtime_fiber::FiberStack>,
}

// SAFETY: it's known that `std::task::Context` is neither `Send` nor `Sync`,
// but despite this the storage here is purely temporary in getting these
// pointers across function frames. The actual types are not sent across threads
// as when a store isn't polling anything the pointer values are all set to
// `None`. Thus if a store is being sent across threads that's done because no
// fibers are active, and once fibers are active everything will stick within
// the same thread.
unsafe impl Send for AsyncState {}
unsafe impl Sync for AsyncState {}

impl Default for AsyncState {
    fn default() -> Self {
        Self {
            current_suspend: None,
            current_future_cx: None,
            last_fiber_stack: None,
        }
    }
}

impl AsyncState {
    pub(crate) fn last_fiber_stack(&mut self) -> &mut Option<wasmtime_fiber::FiberStack> {
        &mut self.last_fiber_stack
    }
}

trait AsStoreOpaque {
    fn as_store_opaque(&mut self) -> &mut StoreOpaque;
}

impl AsStoreOpaque for StoreOpaque {
    fn as_store_opaque(&mut self) -> &mut StoreOpaque {
        self
    }
}

impl<T: 'static> AsStoreOpaque for StoreInner<T> {
    fn as_store_opaque(&mut self) -> &mut StoreOpaque {
        self
    }
}

/// A helper structure used to block a fiber.
///
/// This is acquired via either `StoreContextMut::with_blocking` or
/// `StoreOpaque::with_blocking`. This structure represents the "taken" state of
/// pointers from a store's `AsyncState`, then modeling them as safe pointers.
///
/// Note that the lifetimes here are carefully controlled in instances of this
/// structure through the construction of the `with` function.
pub(crate) struct BlockingContext<'a, 'b> {
    /// Pointer to `wasmtime_fiber::Suspend` which was supplied when a fiber
    /// first started.
    ///
    /// When a `BlockingContext` is first created this pointer is "taken" from
    /// the store (the store is null'd out) and then the raw pointer previously
    /// in the store is unsafely transformed to this safe pointer. This
    /// represents how a `BlockingContext` temporarily has access to this
    /// suspend but when the `BlockingContext` goes away this'll make its way
    /// back into the store.
    suspend: &'a mut WasmtimeSuspend,

    /// Pointer to the future `Context` that this fiber is being polled with.
    ///
    /// Similar to `suspend` above this is taken from a store when a
    /// `BlockingContext` is created and it's restored when the
    /// `BlockingContext` goes away. Note though that unlike `suspend`, as
    /// alluded to in the documentation on `AsyncState`, this value changes over
    /// time as calls to poll are made. This field becomes `None` during a
    /// suspension because that means that the context is released and no longer
    /// available. Upon resumption the context here is *optionally* provided.
    /// Cancellation is a case where it isn't passed back and a re-poll is a
    /// case where it's passed back.
    future_cx: Option<&'a mut Context<'b>>,
}

impl<'a, 'b> BlockingContext<'a, 'b> {
    /// Method to go from a `store` provided (which internally contains a
    /// `StoreOpaque`) to a `BlockingContext`.
    ///
    /// This function will "take" context from `store`'s `AsyncState` field. It
    /// will then construct a `BlockingContext` and yield it to the closure `f`
    /// provided. The closure can then block on futures, suspend, etc.
    ///
    /// Upon return of the closure `f` the state from `BlockingContext` is
    /// restored within the store. The return value of `f` is the return value
    /// of this function.
    ///
    /// Note that the `store` must be provided to this function as an argument
    /// to originally acquire state from `AsyncState`. This store is then
    /// supplied back to the closure `f` provided here so the store can be used
    /// to construct an asynchronous or blocking computation which the
    /// `BlockingContext` tries to block on.
    ///
    /// # Safety
    ///
    /// This method is safe to call at any time, but it's worth noting that the
    /// safety of this function relies on the signature of this function.
    /// Notably the lifetime parameters of `BlockingContext` in the `f` closure
    /// here must be anonymous. That ensures that the `BlockingContext` that
    /// callers get access to cannot be persisted outside of that closure call
    /// and everything is scoped to just the closure `f` provided with nothing
    /// escaping.
    fn with<S, R>(store: &mut S, f: impl FnOnce(&mut S, &mut BlockingContext<'_, '_>) -> R) -> R
    where
        S: AsStoreOpaque,
    {
        let opaque = store.as_store_opaque();

        let state = opaque.fiber_async_state_mut();

        // SAFETY: this is taking pointers from `AsyncState` and then unsafely
        // turning them into safe references. Lifetime-wise this should be safe
        // because the inferred lifetimes for all these pointers is constrained
        // by the signature of `f` provided here. That ensures that everything
        // is scoped purely to the closure `f` and nothing should be persisted
        // outside of this function call. This, for example, ensures that the
        // `Context<'static>` doesn't leak out, it's only with an anonymous
        // lifetime that's forcibly shorter.
        //
        // Provenance-wise this should be safe as if these fields in the store
        // are non-null then the pointers are provided up-the-stack on this
        // fiber and for this fiber. The "take" pattern here ensures that if
        // this `BlockingContext` context acquires the pointers then there are
        // no other instances of these pointers in use anywhere else.
        let future_cx = unsafe { Some(state.current_future_cx.take().unwrap().as_mut()) };
        let suspend = unsafe { state.current_suspend.take().unwrap().as_mut() };

        let mut reset = ResetBlockingContext {
            store,
            cx: BlockingContext { future_cx, suspend },
        };
        return f(&mut reset.store, &mut reset.cx);

        struct ResetBlockingContext<'a, 'b, S: AsStoreOpaque> {
            store: &'a mut S,
            cx: BlockingContext<'a, 'b>,
        }

        impl<S: AsStoreOpaque> Drop for ResetBlockingContext<'_, '_, S> {
            fn drop(&mut self) {
                let store = self.store.as_store_opaque();
                let state = store.fiber_async_state_mut();

                debug_assert!(state.current_future_cx.is_none());
                debug_assert!(state.current_suspend.is_none());
                state.current_suspend = Some(NonNull::from(&mut *self.cx.suspend));

                if let Some(cx) = &mut self.cx.future_cx {
                    // SAFETY: while this is changing the lifetime to `'static`
                    // it should never be used while it's `'static` given this
                    // `BlockingContext` abstraction.
                    state.current_future_cx =
                        Some(NonNull::from(unsafe { change_context_lifetime(cx) }));
                }
            }
        }
    }

    /// Blocks on the asynchronous computation represented by `future` and
    /// produces the result here, in-line.
    ///
    /// This function is designed to only work when it's currently executing on
    /// a native fiber. This fiber provides the ability for us to handle the
    /// future's `Pending` state as "jump back to whomever called the fiber in
    /// an asynchronous fashion and propagate `Pending`". This tight coupling
    /// with `on_fiber` below is what powers the asynchronicity of calling wasm.
    ///
    /// This function takes a `future` and will (appear to) synchronously wait
    /// on the result. While this function is executing it will fiber switch
    /// to-and-from the original frame calling `on_fiber` which should be a
    /// guarantee due to how async stores are configured.
    ///
    /// The return value here is either the output of the future `T`, or a trap
    /// which represents that the asynchronous computation was cancelled. It is
    /// not recommended to catch the trap and try to keep executing wasm, so
    /// we've tried to liberally document this.
    ///
    /// Note that this function suspends (if needed) with
    /// `StoreFiberYield::KeepStore`, indicating that the store must not be used
    /// (and that no other fibers may be resumed) until this fiber resumes.
    /// Therefore, it is not appropriate for use in e.g. guest calls to
    /// async-lowered imports implemented as host functions, since it will
    /// prevent any other tasks from being run.  Use `Instance::suspend` to
    /// suspend and release the store to allow other tasks to run before this
    /// fiber is resumed.
    ///
    /// # Return Value
    ///
    /// A return value of `Ok(value)` means that the future completed with
    /// `value`. A return value of `Err(e)` means that the fiber and its future
    /// have been cancelled and the fiber needs to exit and complete ASAP.
    ///
    /// # Safety
    ///
    /// This function is safe to call at any time but relies on a trait bound
    /// that is manually placed here the compiler does not otherwise require.
    /// Notably the `Send` bound on the future provided here is not required
    /// insofar as things compile without that. The purpose of this, however, is
    /// to make the `unsafe impl Send for StoreFiber` more safe. The `future`
    /// here is state that is stored on the stack during the suspension of this
    /// fiber and is otherwise not visible to the compiler. By having a `Send`
    /// bound here it ensures that the future doesn't have things like `Rc` or
    /// similar pointing into thread locals which would not be sound if this
    /// fiber crosses threads.
    pub(crate) fn block_on<F>(&mut self, future: F) -> Result<F::Output>
    where
        F: Future + Send,
    {
        let mut future = core::pin::pin!(future);
        loop {
            match future.as_mut().poll(self.future_cx.as_mut().unwrap()) {
                Poll::Ready(v) => break Ok(v),
                Poll::Pending => self.suspend(StoreFiberYield::KeepStore)?,
            }
        }
    }

    /// Suspend this fiber with `yield_` as the reason.
    ///
    /// This function will suspend the current fiber and only return after the
    /// fiber has resumed. This function return `Ok(())` if the fiber was
    /// resumed to be completed, and `Err(e)` indicates that the fiber has been
    /// cancelled and needs to exit/complete ASAP.
    pub(crate) fn suspend(&mut self, yield_: StoreFiberYield) -> Result<()> {
        // Over a suspension point we're guaranteed that the `Context` provided
        // here is no longer valid, so discard it. If we're supposed to be able
        // to poll afterwards this will be given back as part of the resume
        // value given back.
        self.future_cx.take();

        let mut new_future_cx: NonNull<Context<'static>> = self.suspend.suspend(yield_)?;

        // SAFETY: this function is unsafe as we're doing "funky" things to the
        // `new_future_cx` we have been given. The safety here relies on the
        // fact that the lifetimes of `BlockingContext` are all "smaller" than
        // the original `Context` itself, and that should be guaranteed through
        // the exclusive constructor of this type `BlockingContext::with`.
        unsafe {
            self.future_cx = Some(change_context_lifetime(new_future_cx.as_mut()));
        }
        Ok(())
    }
}

impl<T> StoreContextMut<'_, T> {
    /// Blocks on the future computed by `f`.
    ///
    /// # Panics
    ///
    /// Panics if this is invoked outside the context of a fiber.
    pub(crate) fn block_on<R>(
        self,
        f: impl FnOnce(StoreContextMut<'_, T>) -> Pin<Box<dyn Future<Output = R> + Send + '_>>,
    ) -> Result<R> {
        self.with_blocking(|store, cx| cx.block_on(f(store).as_mut()))
    }

    /// Creates a `BlockingContext` suitable for blocking on futures or
    /// suspending the current fiber.
    ///
    /// # Panics
    ///
    /// Panics if this is invoked outside the context of a fiber.
    pub(crate) fn with_blocking<R>(
        self,
        f: impl FnOnce(StoreContextMut<'_, T>, &mut BlockingContext<'_, '_>) -> R,
    ) -> R {
        BlockingContext::with(self.0, |store, cx| f(StoreContextMut(store), cx))
    }
}

impl<T> crate::store::StoreInner<T> {
    /// Blocks on the future computed by `f`.
    ///
    /// # Panics
    ///
    /// Panics if this is invoked outside the context of a fiber.
    pub(crate) fn block_on<R>(
        &mut self,
        f: impl FnOnce(StoreContextMut<'_, T>) -> Pin<Box<dyn Future<Output = R> + Send + '_>>,
    ) -> Result<R> {
        BlockingContext::with(self, |store, cx| {
            cx.block_on(f(StoreContextMut(store)).as_mut())
        })
    }
}

impl StoreOpaque {
    /// Blocks on the future computed by `f`.
    ///
    /// # Panics
    ///
    /// Panics if this is invoked outside the context of a fiber.
    pub(crate) fn block_on<R>(
        &mut self,
        f: impl FnOnce(&mut Self) -> Pin<Box<dyn Future<Output = R> + Send + '_>>,
    ) -> Result<R> {
        BlockingContext::with(self, |store, cx| cx.block_on(f(store).as_mut()))
    }

    /// Creates a `BlockingContext` suitable for blocking on futures or
    /// suspending the current fiber.
    ///
    /// # Panics
    ///
    /// Panics if this is invoked outside the context of a fiber.
    #[cfg(feature = "component-model-async")]
    pub(crate) fn with_blocking<R>(
        &mut self,
        f: impl FnOnce(&mut Self, &mut BlockingContext<'_, '_>) -> R,
    ) -> R {
        BlockingContext::with(self, |store, cx| f(store, cx))
    }

    /// Returns whether `block_on` will succeed or panic.
    #[cfg(feature = "call-hook")]
    pub(crate) fn can_block(&mut self) -> bool {
        self.fiber_async_state_mut().current_future_cx.is_some()
    }
}

/// Indicates whether or not a fiber needs to retain exclusive access to its
/// store across a suspend/resume interval.
pub(crate) enum StoreFiberYield {
    /// Indicates the fiber needs to retain exclusive access, meaning the store
    /// should not be used outside of the fiber until after the fiber either
    /// suspends with `ReleaseStore` or resolves.
    KeepStore,
    /// Indicates the fiber does _not_ need exclusive access across the
    /// suspend/resume interval, meaning the store may be used as needed until
    /// the fiber is resumed.
    #[cfg(feature = "component-model-async")]
    ReleaseStore,
}

pub(crate) struct StoreFiber<'a> {
    /// The raw `wasmtime_fiber::Fiber`.
    ///
    /// Note that using `StoreFiberYield` as the `Yield` type parameter allows
    /// the fiber to indicate whether it needs exclusive access to the store
    /// across suspend points (in which case it will pass `KeepStore` when
    /// suspending , meaning the store must not be used at all until the fiber
    /// is resumed again) or whether it is giving up exclusive access (in which
    /// case it will pass `ReleaseStore` when yielding, meaning exclusive access
    /// may be given to another fiber that runs concurrently.
    ///
    /// Note also that every `StoreFiber` is implicitly granted exclusive access
    /// to the store when it is resumed.
    fiber: Option<Fiber<'a, WasmtimeResume, WasmtimeYield, WasmtimeComplete>>,
    /// See `FiberResumeState`
    state: Option<FiberResumeState>,
    /// The Wasmtime `Engine` to which this fiber belongs.
    engine: Engine,
    /// The id of the store with which this fiber was created.
    ///
    /// Any attempt to resume a fiber with a different store than the one with
    /// which it was created will panic.
    id: StoreId,
}

impl StoreFiber<'_> {
    pub(crate) fn dispose(&mut self, store: &mut StoreOpaque) {
        if let Some(fiber) = &mut self.fiber {
            if !fiber.done() {
                let result = resume_fiber(store, self, Err(anyhow!("future dropped")));
                debug_assert!(result.is_ok());
            }
        }
    }
}

// Note that this implementation will panic if the fiber is in-progress, which
// will abort the process if there is already a panic being unwound.  That
// should only happen if we failed to call `StoreFiber::dispose` on the
// in-progress fiber prior to dropping it, which indicates a bug in this crate
// which must be fixed.
impl Drop for StoreFiber<'_> {
    fn drop(&mut self) {
        if self.fiber.is_none() {
            return;
        }

        assert!(
            self.fiber.as_ref().unwrap().done(),
            "attempted to drop in-progress fiber without first calling `StoreFiber::dispose`"
        );

        self.state.take().unwrap().dispose();

        unsafe {
            self.engine
                .allocator()
                .deallocate_fiber_stack(self.fiber.take().unwrap().into_stack());
        }
    }
}

// This is surely the most dangerous `unsafe impl Send` in the entire
// crate. There are two members in `StoreFiber` which cause it to not be
// `Send`. One is `suspend` and is entirely uninteresting.  This is just used to
// manage `Suspend` when resuming, and requires raw pointers to get it to happen
// easily.  Nothing too weird about the `Send`-ness, values aren't actually
// crossing threads.
//
// The really interesting piece is `fiber`. Now the "fiber" here is actual
// honest-to-god Rust code which we're moving around. What we're doing is the
// equivalent of moving our thread's stack to another OS thread. Turns out we,
// in general, have no idea what's on the stack and would generally have no way
// to verify that this is actually safe to do!
//
// Thankfully, though, Wasmtime has the power. Without being glib it's actually
// worth examining what's on the stack. It's unfortunately not super-local to
// this function itself. Our closure to `Fiber::new` runs `func`, which is given
// to us from the outside. Thankfully, though, we have tight control over
// this. Usage of `on_fiber` or `Instance::resume_fiber` is typically done
// *just* before entering WebAssembly itself, so we'll have a few stack frames
// of Rust code (all in Wasmtime itself) before we enter wasm.
//
// Once we've entered wasm, well then we have a whole bunch of wasm frames on
// the stack. We've got this nifty thing called Cranelift, though, which allows
// us to also have complete control over everything on the stack!
//
// Finally, when wasm switches back to the fiber's starting pointer (this future
// we're returning) then it means wasm has reentered Rust.  Suspension can only
// happen via either `block_on` or `Instance::suspend`. This, conveniently, also
// happens entirely in Wasmtime controlled code!
//
// There's an extremely important point that should be called out here.
// User-provided futures **are not on the stack** during suspension points. This
// is extremely crucial because we in general cannot reason about Send/Sync for
// stack-local variables since rustc doesn't analyze them at all. With our
// construction, though, we are guaranteed that Wasmtime owns all stack frames
// between the stack of a fiber and when the fiber suspends (and it could move
// across threads). At this time the only user-provided piece of data on the
// stack is the future itself given to us. Lo-and-behold as you might notice the
// future is required to be `Send`!
//
// What this all boils down to is that we, as the authors of Wasmtime, need to
// be extremely careful that on the async fiber stack we only store Send
// things. For example we can't start using `Rc` willy nilly by accident and
// leave a copy in TLS somewhere. (similarly we have to be ready for TLS to
// change while we're executing wasm code between suspension points).
//
// While somewhat onerous it shouldn't be too too hard (the TLS bit is the
// hardest bit so far). This does mean, though, that no user should ever have to
// worry about the `Send`-ness of Wasmtime. If rustc says it's ok, then it's ok.
//
// With all that in mind we unsafely assert here that Wasmtime is correct. We
// declare the fiber as only containing Send data on its stack, despite not
// knowing for sure at compile time that this is correct. That's what `unsafe`
// in Rust is all about, though, right?
unsafe impl Send for StoreFiber<'_> {}
// See the docs about the `Send` impl above, which also apply to this `Sync`
// impl.  `Sync` is needed since we store `StoreFiber`s and switch between them
// when executing components that export async-lifted functions.
unsafe impl Sync for StoreFiber<'_> {}

/// State of the world when a fiber last suspended.
///
/// This structure represents global state that a fiber clobbers during its
/// execution. For example TLS variables are updated, system resources like MPK
/// masks are updated, etc. The purpose of this structure is to track all of
/// this state and appropriately save/restore it around fiber suspension points.
struct FiberResumeState {
    /// Saved list of `CallThreadState` activations that are stored on a fiber
    /// stack.
    ///
    /// This is a linked list that references stack-stored nodes on the fiber
    /// stack that is currently suspended. The `AsyncWasmCallState` type
    /// documents this more thoroughly but the general gist is that when we this
    /// fiber is resumed this linked list needs to be pushed on to the current
    /// thread's linked list of activations.
    tls: crate::runtime::vm::AsyncWasmCallState,

    /// Saved MPK protection mask, if enabled.
    ///
    /// When MPK is enabled then executing WebAssembly will modify the
    /// processor's current mask of addressable protection keys. This means that
    /// our current state may get clobbered when a fiber suspends. To ensure
    /// that this function preserves context it will, when MPK is enabled, save
    /// the current mask when this function is called and then restore the mask
    /// when the function returns (aka the fiber suspends).
    mpk: Option<ProtectionMask>,

    /// The current wasm stack limit, if in use.
    ///
    /// This field stores the old of `VMStoreContext::stack_limit` that this
    /// fiber should be using during its execution. This is saved/restored when
    /// a fiber is suspended/resumed to ensure that when there are multiple
    /// fibers within the store they all maintain an appropriate fiber-relative
    /// stack limit.
    stack_limit: usize,

    /// The executor (e.g. the Pulley interpreter state) belonging to this
    /// fiber.
    ///
    /// This is swapped with `StoreOpaque::executor` whenever this fiber is
    /// resumed, suspended, or resolved.
    executor: Executor,
}

impl FiberResumeState {
    unsafe fn replace(
        self,
        store: &mut StoreOpaque,
        fiber: &mut StoreFiber<'_>,
    ) -> PriorFiberResumeState {
        let tls = unsafe { self.tls.push() };
        let mpk = swap_mpk_states(self.mpk);
        let async_guard_range = fiber
            .fiber
            .as_ref()
            .unwrap()
            .stack()
            .guard_range()
            .unwrap_or(ptr::null_mut()..ptr::null_mut());
        let mut executor = self.executor;
        store.swap_executor(&mut executor);
        PriorFiberResumeState {
            tls,
            mpk,
            executor,
            stack_limit: store.replace_stack_limit(self.stack_limit),
            async_guard_range: store.replace_async_guard_range(async_guard_range),

            // The current suspend/future_cx are always null upon resumption, so
            // insert null. Save the old values through to get preserved across
            // this resume/suspend.
            current_suspend: store.replace_current_suspend(None),
            current_future_cx: store.replace_current_future_cx(None),
        }
    }

    fn dispose(self) {
        self.tls.assert_null();
    }
}

impl StoreOpaque {
    /// Helper function to swap the `stack_limit` field in the `VMStoreContext`
    /// within this store.
    fn replace_stack_limit(&mut self, stack_limit: usize) -> usize {
        mem::replace(
            &mut self.vm_store_context_mut().stack_limit.get_mut(),
            stack_limit,
        )
    }

    /// Helper function to swap the `async_guard_range` field in the `VMStoreContext`
    /// within this store.
    fn replace_async_guard_range(&mut self, range: Range<*mut u8>) -> Range<*mut u8> {
        mem::replace(&mut self.vm_store_context_mut().async_guard_range, range)
    }

    fn replace_current_suspend(
        &mut self,
        ptr: Option<NonNull<WasmtimeSuspend>>,
    ) -> Option<NonNull<WasmtimeSuspend>> {
        mem::replace(&mut self.fiber_async_state_mut().current_suspend, ptr)
    }

    fn replace_current_future_cx(
        &mut self,
        ptr: Option<NonNull<Context<'static>>>,
    ) -> Option<NonNull<Context<'static>>> {
        mem::replace(&mut self.fiber_async_state_mut().current_future_cx, ptr)
    }
}

struct PriorFiberResumeState {
    tls: crate::runtime::vm::PreviousAsyncWasmCallState,
    mpk: Option<ProtectionMask>,
    stack_limit: usize,
    async_guard_range: Range<*mut u8>,
    current_suspend: Option<NonNull<WasmtimeSuspend>>,
    current_future_cx: Option<NonNull<Context<'static>>>,
    executor: Executor,
}

impl PriorFiberResumeState {
    unsafe fn replace(self, store: &mut StoreOpaque) -> FiberResumeState {
        let tls = unsafe { self.tls.restore() };
        let mpk = swap_mpk_states(self.mpk);
        // No need to save `_my_guard` since we can re-infer it from the fiber
        // that this state is attached to.
        let _my_guard = store.replace_async_guard_range(self.async_guard_range);

        // Restore the previous values of current_{suspend,future_cx} but we
        // should be guaranteed that the prior values are null, so double-check
        // that here.
        let prev = store.replace_current_suspend(self.current_suspend);
        assert!(prev.is_none());
        let prev = store.replace_current_future_cx(self.current_future_cx);
        assert!(prev.is_none());

        let mut executor = self.executor;
        store.swap_executor(&mut executor);

        FiberResumeState {
            tls,
            mpk,
            executor,
            stack_limit: store.replace_stack_limit(self.stack_limit),
        }
    }
}

fn swap_mpk_states(mask: Option<ProtectionMask>) -> Option<ProtectionMask> {
    mask.map(|mask| {
        let current = mpk::current_mask();
        mpk::allow(mask);
        current
    })
}

/// Resume the specified fiber, granting it exclusive access to the store with
/// which it was created.
///
/// This will return `Ok(result)` if the fiber resolved, where `result` is the
/// returned value; it will return `Err(yield_)` if the fiber suspended, where
/// `yield_` indicates whether it released access to the store or not.  See
/// `StoreFiber::fiber` for details.
fn resume_fiber<'a>(
    store: &mut StoreOpaque,
    fiber: &mut StoreFiber<'a>,
    result: WasmtimeResume,
) -> Result<WasmtimeComplete, StoreFiberYield> {
    assert_eq!(store.id(), fiber.id);

    struct Restore<'a, 'b> {
        store: &'b mut StoreOpaque,
        fiber: &'b mut StoreFiber<'a>,
        state: Option<PriorFiberResumeState>,
    }

    impl Drop for Restore<'_, '_> {
        fn drop(&mut self) {
            self.fiber.state = Some(unsafe { self.state.take().unwrap().replace(self.store) });
        }
    }
    let result = unsafe {
        let prev = fiber.state.take().unwrap().replace(store, fiber);
        let restore = Restore {
            store,
            fiber,
            state: Some(prev),
        };
        restore.fiber.fiber.as_ref().unwrap().resume(result)
    };

    match &result {
        // The fiber has finished, so recycle its stack by disposing of the
        // underlying fiber itself.
        Ok(_) => {
            let stack = fiber.fiber.take().map(|f| f.into_stack());
            if let Some(stack) = stack {
                store.deallocate_fiber_stack(stack);
            }
        }

        // The fiber has not yet finished, so it stays as-is.
        Err(_) => {
            // If `Err` is returned that means the fiber suspended, so we
            // propagate that here.
            //
            // An additional safety check is performed when leaving this
            // function to help bolster the guarantees of `unsafe impl Send`
            // above. Notably this future may get re-polled on a different
            // thread. Wasmtime's thread-local state points to the stack,
            // however, meaning that it would be incorrect to leave a pointer in
            // TLS when this function returns. This function performs a runtime
            // assert to verify that this is the case, notably that the one TLS
            // pointer Wasmtime uses is not pointing anywhere within the
            // stack. If it is then that's a bug indicating that TLS management
            // in Wasmtime is incorrect.
            if let Some(range) = fiber.fiber.as_ref().unwrap().stack().range() {
                AsyncWasmCallState::assert_current_state_not_in_range(range);
            }
        }
    }

    result
}

/// Create a new `StoreFiber` which runs the specified closure.
pub(crate) fn make_fiber<'a>(
    store: &mut dyn VMStore,
    fun: impl FnOnce(&mut dyn VMStore) -> Result<()> + Send + Sync + 'a,
) -> Result<StoreFiber<'a>> {
    let engine = store.engine().clone();
    let executor = Executor::new(&engine);
    let id = store.store_opaque().id();
    let stack = store.store_opaque_mut().allocate_fiber_stack()?;
    let track_pkey_context_switch = store.has_pkey();
    let store = &raw mut *store;
    let fiber = Fiber::new(stack, move |result: WasmtimeResume, suspend| {
        let future_cx = match result {
            Ok(cx) => cx,
            // Cancelled before we started? Just return.
            Err(_) => return Ok(()),
        };

        // SAFETY: This fiber will only be resumed using `resume_fiber`, which
        // takes a `&mut StoreOpaque` parameter and has given us exclusive
        // access to the store until we exit or yield it back to the resumer.
        let store_ref = unsafe { &mut *store };

        // It should be a guarantee that the store has null pointers here upon
        // starting a fiber, so now's the time to fill in the pointers now that
        // the fiber is running and `future_cx` and `suspend` are both in scope.
        // Note that these pointers are removed when this function returns as
        // that's when they fall out of scope.
        let async_state = store_ref.store_opaque_mut().fiber_async_state_mut();
        assert!(async_state.current_suspend.is_none());
        assert!(async_state.current_future_cx.is_none());
        async_state.current_suspend = Some(NonNull::from(suspend));
        async_state.current_future_cx = Some(future_cx);

        struct ResetCurrentPointersToNull<'a>(&'a mut dyn VMStore);

        impl Drop for ResetCurrentPointersToNull<'_> {
            fn drop(&mut self) {
                let state = self.0.fiber_async_state_mut();

                // Double-check that the current suspension isn't null (it
                // should be what's in this closure). Note though that we
                // can't check `current_future_cx` because it may either be
                // here or not be here depending on whether this was
                // cancelled or not.
                debug_assert!(state.current_suspend.is_some());

                state.current_suspend = None;
                state.current_future_cx = None;
            }
        }
        let reset = ResetCurrentPointersToNull(store_ref);

        fun(reset.0)
    })?;
    Ok(StoreFiber {
        state: Some(FiberResumeState {
            tls: crate::runtime::vm::AsyncWasmCallState::new(),
            mpk: if track_pkey_context_switch {
                Some(ProtectionMask::all())
            } else {
                None
            },
            stack_limit: usize::MAX,
            executor,
        }),
        engine,
        id,
        fiber: Some(fiber),
    })
}

/// Run the specified function on a newly-created fiber and `.await` its
/// completion.
pub(crate) async fn on_fiber<R: Send + Sync>(
    store: &mut StoreOpaque,
    func: impl FnOnce(&mut StoreOpaque) -> R + Send + Sync,
) -> Result<R> {
    let config = store.engine().config();
    debug_assert!(store.async_support());
    debug_assert!(config.async_stack_size > 0);

    let mut result = None;
    let fiber = make_fiber(store.traitobj_mut(), |store| {
        result = Some(func(store));
        Ok(())
    })?;

    {
        let fiber = FiberFuture {
            store,
            fiber: Some(fiber),
            #[cfg(feature = "component-model-async")]
            on_release: OnRelease::ReturnPending,
        }
        .await
        .unwrap();

        debug_assert!(fiber.is_none());
    }

    Ok(result.unwrap())
}

/// Run the specified fiber until it either suspends with
/// `StoreFiberYield::ReleaseStore` or resolves.
///
/// This will return `Some` if the fiber suspends with
/// `StoreFiberYield::ReleaseStore` or else `None` if it resolves.
#[cfg(feature = "component-model-async")]
pub(crate) async fn resolve_or_release<'a>(
    store: &mut StoreOpaque,
    fiber: StoreFiber<'a>,
) -> Result<Option<StoreFiber<'a>>> {
    FiberFuture {
        store,
        fiber: Some(fiber),
        on_release: OnRelease::ReturnReady,
    }
    .await
}

/// Tells a `FiberFuture` what to do if `poll_fiber` returns
/// `Err(StoreFiberYield::ReleaseStore)`.
#[cfg(feature = "component-model-async")]
enum OnRelease {
    /// Return `Poll::Pending` from `FiberFuture::poll`
    ReturnPending,
    /// Return `Poll::Ready` from `FiberFuture::poll`, handing ownership of the
    /// `StoreFiber` to the caller.
    ReturnReady,
}

/// A `Future` implementation for running a `StoreFiber` to completion, giving
/// it exclusive access to its store until it resolves.
struct FiberFuture<'a, 'b> {
    store: &'a mut StoreOpaque,
    fiber: Option<StoreFiber<'b>>,
    #[cfg(feature = "component-model-async")]
    on_release: OnRelease,
}

impl<'b> Future for FiberFuture<'_, 'b> {
    type Output = Result<Option<StoreFiber<'b>>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let me = self.get_mut();

        // SAFETY: We need to carry over this `cx` into our fiber's runtime for
        // when it tries to poll sub-futures that are created. Doing this must
        // be done unsafely, however, since `cx` is only alive for this one
        // singular function call. Here we do a `transmute` to extend the
        // lifetime of `Context` so it can be stored in our `Store`, and then we
        // replace the current polling context with this one.
        //
        // The safety of this extension relies on never actually using
        // `Context<'static>` with `'static` actually there, which should be
        // satisfied by the users of this in the `BlockingContext` structure
        // where the lifetime parameters there are always more constrained than
        // they are here.
        let cx: &mut Context<'static> = unsafe { change_context_lifetime(cx) };
        let cx = NonNull::from(cx);

        match resume_fiber(me.store, me.fiber.as_mut().unwrap(), Ok(cx)) {
            Ok(Ok(())) => Poll::Ready(Ok(None)),
            Ok(Err(e)) => Poll::Ready(Err(e)),
            Err(StoreFiberYield::KeepStore) => Poll::Pending,
            #[cfg(feature = "component-model-async")]
            Err(StoreFiberYield::ReleaseStore) => match &me.on_release {
                OnRelease::ReturnPending => Poll::Pending,
                OnRelease::ReturnReady => Poll::Ready(Ok(me.fiber.take())),
            },
        }
    }
}

impl Drop for FiberFuture<'_, '_> {
    fn drop(&mut self) {
        if let Some(fiber) = &mut self.fiber {
            fiber.dispose(self.store);
        }
    }
}

/// Changes the lifetime `'l` in `Context<'l>` to something else.
///
/// # Safety
///
/// Not a safe operation. Requires external knowledge about how the pointer is
/// being used to determine whether it's actually safe or not. See docs on
/// callers of this function. The purpose of this is to scope the `transmute` to
/// as small an operation as possible.
unsafe fn change_context_lifetime<'a, 'b>(cx: &'a mut Context<'_>) -> &'a mut Context<'b> {
    // SAFETY: See the function documentation, this is not safe in general.
    unsafe { mem::transmute::<&mut Context<'_>, &mut Context<'b>>(cx) }
}
