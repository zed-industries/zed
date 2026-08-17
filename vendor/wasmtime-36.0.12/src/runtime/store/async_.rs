#[cfg(feature = "call-hook")]
use crate::CallHook;
use crate::fiber::{self};
use crate::prelude::*;
use crate::store::{ResourceLimiterInner, StoreInner, StoreOpaque, StoreToken};
use crate::{AsContextMut, Store, StoreContextMut, UpdateDeadline};

/// An object that can take callbacks when the runtime enters or exits hostcalls.
#[cfg(feature = "call-hook")]
#[async_trait::async_trait]
pub trait CallHookHandler<T>: Send {
    /// A callback to run when wasmtime is about to enter a host call, or when about to
    /// exit the hostcall.
    async fn handle_call_event(&self, t: StoreContextMut<'_, T>, ch: CallHook) -> Result<()>;
}

impl<T> Store<T> {
    /// Configures the [`ResourceLimiterAsync`](crate::ResourceLimiterAsync)
    /// used to limit resource creation within this [`Store`].
    ///
    /// This method is an asynchronous variant of the [`Store::limiter`] method
    /// where the embedder can block the wasm request for more resources with
    /// host `async` execution of futures.
    ///
    /// By using a [`ResourceLimiterAsync`](`crate::ResourceLimiterAsync`)
    /// with a [`Store`], you can no longer use
    /// [`Memory::new`](`crate::Memory::new`),
    /// [`Memory::grow`](`crate::Memory::grow`),
    /// [`Table::new`](`crate::Table::new`), and
    /// [`Table::grow`](`crate::Table::grow`). Instead, you must use their
    /// `async` variants: [`Memory::new_async`](`crate::Memory::new_async`),
    /// [`Memory::grow_async`](`crate::Memory::grow_async`),
    /// [`Table::new_async`](`crate::Table::new_async`), and
    /// [`Table::grow_async`](`crate::Table::grow_async`).
    ///
    /// Note that this limiter is only used to limit the creation/growth of
    /// resources in the future, this does not retroactively attempt to apply
    /// limits to the [`Store`]. Additionally this must be used with an async
    /// [`Store`] configured via
    /// [`Config::async_support`](crate::Config::async_support).
    pub fn limiter_async(
        &mut self,
        mut limiter: impl (FnMut(&mut T) -> &mut dyn crate::ResourceLimiterAsync)
        + Send
        + Sync
        + 'static,
    ) {
        debug_assert!(self.inner.async_support());
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
        inner.limiter = Some(ResourceLimiterInner::Async(Box::new(limiter)));
    }

    /// Configures an async function that runs on calls and returns between
    /// WebAssembly and host code. For the non-async equivalent of this method,
    /// see [`Store::call_hook`].
    ///
    /// The function is passed a [`CallHook`] argument, which indicates which
    /// state transition the VM is making.
    ///
    /// This function's future may return a [`Trap`]. If a trap is returned
    /// when an import was called, it is immediately raised as-if the host
    /// import had returned the trap. If a trap is returned after wasm returns
    /// to the host then the wasm function's result is ignored and this trap is
    /// returned instead.
    ///
    /// After this function returns a trap, it may be called for subsequent
    /// returns to host or wasm code as the trap propagates to the root call.
    #[cfg(feature = "call-hook")]
    pub fn call_hook_async(&mut self, hook: impl CallHookHandler<T> + Send + Sync + 'static) {
        self.inner.call_hook = Some(crate::store::CallHookInner::Async(Box::new(hook)));
    }

    /// Perform garbage collection asynchronously.
    ///
    /// Note that it is not required to actively call this function. GC will
    /// automatically happen according to various internal heuristics. This is
    /// provided if fine-grained control over the GC is desired.
    ///
    /// This method is only available when the `gc` Cargo feature is enabled.
    #[cfg(feature = "gc")]
    pub async fn gc_async(&mut self, why: Option<&crate::GcHeapOutOfMemory<()>>) -> Result<()>
    where
        T: Send,
    {
        self.inner.gc_async(why).await
    }

    /// Configures epoch-deadline expiration to yield to the async
    /// caller and the update the deadline.
    ///
    /// When epoch-interruption-instrumented code is executed on this
    /// store and the epoch deadline is reached before completion,
    /// with the store configured in this way, execution will yield
    /// (the future will return `Pending` but re-awake itself for
    /// later execution) and, upon resuming, the store will be
    /// configured with an epoch deadline equal to the current epoch
    /// plus `delta` ticks.
    ///
    /// This setting is intended to allow for cooperative timeslicing
    /// of multiple CPU-bound Wasm guests in different stores, all
    /// executing under the control of an async executor. To drive
    /// this, stores should be configured to "yield and update"
    /// automatically with this function, and some external driver (a
    /// thread that wakes up periodically, or a timer
    /// signal/interrupt) should call
    /// [`Engine::increment_epoch()`](crate::Engine::increment_epoch).
    ///
    /// See documentation on
    /// [`Config::epoch_interruption()`](crate::Config::epoch_interruption)
    /// for an introduction to epoch-based interruption.
    #[cfg(target_has_atomic = "64")]
    pub fn epoch_deadline_async_yield_and_update(&mut self, delta: u64) {
        self.inner.epoch_deadline_async_yield_and_update(delta);
    }
}

impl<'a, T> StoreContextMut<'a, T> {
    /// Perform garbage collection of `ExternRef`s.
    ///
    /// Same as [`Store::gc`].
    ///
    /// This method is only available when the `gc` Cargo feature is enabled.
    #[cfg(feature = "gc")]
    pub async fn gc_async(&mut self, why: Option<&crate::GcHeapOutOfMemory<()>>) -> Result<()>
    where
        T: Send + 'static,
    {
        self.0.gc_async(why).await
    }

    /// Configures epoch-deadline expiration to yield to the async
    /// caller and the update the deadline.
    ///
    /// For more information see
    /// [`Store::epoch_deadline_async_yield_and_update`].
    #[cfg(target_has_atomic = "64")]
    pub fn epoch_deadline_async_yield_and_update(&mut self, delta: u64) {
        self.0.epoch_deadline_async_yield_and_update(delta);
    }
}

impl<T> StoreInner<T> {
    #[cfg(target_has_atomic = "64")]
    fn epoch_deadline_async_yield_and_update(&mut self, delta: u64) {
        assert!(
            self.async_support(),
            "cannot use `epoch_deadline_async_yield_and_update` without enabling async support in the config"
        );
        self.epoch_deadline_behavior =
            Some(Box::new(move |_store| Ok(UpdateDeadline::Yield(delta))));
    }
}

#[doc(hidden)]
impl StoreOpaque {
    /// Executes a synchronous computation `func` asynchronously on a new fiber.
    ///
    /// This function will convert the synchronous `func` into an asynchronous
    /// future. This is done by running `func` in a fiber on a separate native
    /// stack which can be suspended and resumed from.
    pub(crate) async fn on_fiber<R: Send + Sync>(
        &mut self,
        func: impl FnOnce(&mut Self) -> R + Send + Sync,
    ) -> Result<R> {
        fiber::on_fiber(self, func).await
    }

    #[cfg(feature = "gc")]
    pub(super) async fn do_gc_async(&mut self) {
        assert!(
            self.async_support(),
            "cannot use `gc_async` without enabling async support in the config",
        );

        // If the GC heap hasn't been initialized, there is nothing to collect.
        if self.gc_store.is_none() {
            return;
        }

        log::trace!("============ Begin Async GC ===========");

        // Take the GC roots out of `self` so we can borrow it mutably but still
        // call mutable methods on `self`.
        let mut roots = core::mem::take(&mut self.gc_roots_list);

        self.trace_roots_async(&mut roots).await;
        self.unwrap_gc_store_mut()
            .gc_async(unsafe { roots.iter() })
            .await;

        // Restore the GC roots for the next GC.
        roots.clear();
        self.gc_roots_list = roots;

        log::trace!("============ End Async GC ===========");
    }

    #[inline]
    #[cfg(not(feature = "gc"))]
    pub async fn gc_async(&mut self) {
        // Nothing to collect.
        //
        // Note that this is *not* a public method, this is just defined for the
        // crate-internal `StoreOpaque` type. This is a convenience so that we
        // don't have to `cfg` every call site.
    }

    #[cfg(feature = "gc")]
    async fn trace_roots_async(&mut self, gc_roots_list: &mut crate::runtime::vm::GcRootsList) {
        use crate::runtime::vm::Yield;

        log::trace!("Begin trace GC roots");

        // We shouldn't have any leftover, stale GC roots.
        assert!(gc_roots_list.is_empty());

        self.trace_wasm_stack_roots(gc_roots_list);
        Yield::new().await;
        self.trace_vmctx_roots(gc_roots_list);
        Yield::new().await;
        self.trace_user_roots(gc_roots_list);

        log::trace!("End trace GC roots")
    }

    /// Yields execution to the caller on out-of-gas or epoch interruption.
    ///
    /// This only works on async futures and stores, and assumes that we're
    /// executing on a fiber. This will yield execution back to the caller once.
    pub fn async_yield_impl(&mut self) -> Result<()> {
        // When control returns, we have a `Result<()>` passed
        // in from the host fiber. If this finished successfully then
        // we were resumed normally via a `poll`, so keep going.  If
        // the future was dropped while we were yielded, then we need
        // to clean up this fiber. Do so by raising a trap which will
        // abort all wasm and get caught on the other side to clean
        // things up.
        self.block_on(|_| Box::pin(crate::runtime::vm::Yield::new()))
    }

    pub(crate) fn allocate_fiber_stack(&mut self) -> Result<wasmtime_fiber::FiberStack> {
        if let Some(stack) = self.async_state.last_fiber_stack().take() {
            return Ok(stack);
        }
        self.engine().allocator().allocate_fiber_stack()
    }

    pub(crate) fn deallocate_fiber_stack(&mut self, stack: wasmtime_fiber::FiberStack) {
        self.flush_fiber_stack();
        *self.async_state.last_fiber_stack() = Some(stack);
    }

    /// Releases the last fiber stack to the underlying instance allocator, if
    /// present.
    pub fn flush_fiber_stack(&mut self) {
        if let Some(stack) = self.async_state.last_fiber_stack().take() {
            unsafe {
                self.engine.allocator().deallocate_fiber_stack(stack);
            }
        }
    }
}

impl<T> StoreContextMut<'_, T> {
    /// Executes a synchronous computation `func` asynchronously on a new fiber.
    pub(crate) async fn on_fiber<R: Send + Sync>(
        &mut self,
        func: impl FnOnce(&mut StoreContextMut<'_, T>) -> R + Send + Sync,
    ) -> Result<R>
    where
        T: Send + 'static,
    {
        let token = StoreToken::new(self.as_context_mut());
        self.0
            .on_fiber(|opaque| func(&mut token.as_context_mut(opaque.traitobj_mut())))
            .await
    }
}
