//! Bindings used to manage subtasks, or invocations of imported functions.
//!
//! See `future_support` for some more discussion but the basic idea is the same
//! where we require that everything is passed by ownership to primarily deal
//! with the possibility of leaking futures. By always requiring ownership we
//! can guarantee that even when a future is leaked all its parameters passed to
//! the canonical ABI are additionally leaked with it which should be memory
//! safe.

use crate::rt::async_support::waitable::{WaitableOp, WaitableOperation};
use crate::rt::async_support::{
    STATUS_RETURNED, STATUS_RETURNED_CANCELLED, STATUS_STARTED, STATUS_STARTED_CANCELLED,
    STATUS_STARTING,
};
use crate::rt::Cleanup;
use std::alloc::Layout;
use std::future::Future;
use std::marker;
use std::num::NonZeroU32;
use std::ptr;

/// Raw operations used to invoke an imported asynchronous function.
///
/// This trait is implemented by generated bindings and is used to implement
/// asynchronous imports.
///
/// # Unsafety
///
/// All operations/constants must be self-consistent for how this module expects
/// them all to be used.
pub unsafe trait Subtask {
    /// The in-memory layout of both parameters and results allocated with
    /// parameters coming first.
    const ABI_LAYOUT: Layout;

    /// The offset, in bytes, from the start of `ABI_LAYOUT` to where the
    /// results will be stored.
    const RESULTS_OFFSET: usize;

    /// The parameters to this task.
    type Params;
    /// The representation of lowered parameters for this task.
    ///
    /// This is used to account for how lowered imports may have up to 4 flat
    /// arguments or may also be indirect as well in memory. Either way this
    /// represents the actual ABI values passed to the import.
    type ParamsLower: Copy;
    /// The results of this task.
    type Results;

    /// The raw function import using `[async-lower]` and the canonical ABI.
    unsafe fn call_import(params: Self::ParamsLower, results: *mut u8) -> u32;

    /// Bindings-generated version of lowering `params`.
    ///
    /// This may use the heap-allocated `dst`, which is an uninitialized
    /// allocation of `Self::ABI_LAYOUT`. This returns any ABI parameters
    /// necessary to actually invoke the imported function.
    ///
    /// Note that `ParamsLower` may return `dst` if there are more ABI
    /// parameters than are allowed flat params (as specified by the canonical
    /// ABI).
    unsafe fn params_lower(params: Self::Params, dst: *mut u8) -> Self::ParamsLower;

    /// Bindings-generated version of deallocating any lists stored within
    /// `lower`.
    unsafe fn params_dealloc_lists(lower: Self::ParamsLower);

    /// Bindings-generated version of deallocating not only owned lists within
    /// `lower` but also deallocating any owned resources.
    unsafe fn params_dealloc_lists_and_own(lower: Self::ParamsLower);

    /// Bindings-generated version of lifting the results stored at `src`.
    unsafe fn results_lift(src: *mut u8) -> Self::Results;

    /// Helper function to actually perform this asynchronous call with
    /// `params`.
    fn call(params: Self::Params) -> impl Future<Output = Self::Results>
    where
        Self: Sized,
    {
        async {
            match WaitableOperation::<SubtaskOps<Self>>::new(Start { params }).await {
                Ok(results) => results,
                Err(_) => unreachable!(
                    "cancellation is not exposed API-wise, \
                    should not be possible"
                ),
            }
        }
    }
}

struct SubtaskOps<T>(marker::PhantomData<T>);

struct Start<T: Subtask> {
    params: T::Params,
}

unsafe impl<T: Subtask> WaitableOp for SubtaskOps<T> {
    type Start = Start<T>;
    type InProgress = InProgress<T>;
    type Result = Result<T::Results, ()>;
    type Cancel = Result<T::Results, ()>;

    fn start(state: Self::Start) -> (u32, Self::InProgress) {
        unsafe {
            let (ptr_params, cleanup) = Cleanup::new(T::ABI_LAYOUT);
            let ptr_results = ptr_params.add(T::RESULTS_OFFSET);
            let params_lower = T::params_lower(state.params, ptr_params);
            let packed = T::call_import(params_lower, ptr_results);
            let code = packed & 0xf;
            let subtask = NonZeroU32::new(packed >> 4).map(|handle| SubtaskHandle { handle });
            rtdebug!("<import>({ptr_params:?}, {ptr_results:?}) = ({code:#x}, {subtask:#x?})");

            (
                code,
                InProgress {
                    params_lower,
                    params_and_results: cleanup,
                    subtask,
                    started: false,
                    _marker: marker::PhantomData,
                },
            )
        }
    }

    fn start_cancelled(_state: Self::Start) -> Self::Cancel {
        Err(())
    }

    fn in_progress_update(
        mut state: Self::InProgress,
        code: u32,
    ) -> Result<Self::Result, Self::InProgress> {
        match code {
            // Nothing new to do in this state, we're still waiting for the task
            // to start.
            STATUS_STARTING => {
                assert!(!state.started);
                Err(state)
            }

            // Still not done yet, but we can record that this is started and
            // otherwise deallocate lists in the parameters.
            STATUS_STARTED => {
                state.flag_started();
                Err(state)
            }

            STATUS_RETURNED => {
                // Conditionally flag as started if we haven't otherwise
                // explicitly transitioned through `STATUS_STARTED`.
                if !state.started {
                    state.flag_started();
                }

                // Now that our results have been written we can read them.
                //
                // Note that by dropping `state` here we'll both deallocate the
                // params/results storage area as well as the subtask handle
                // itself.
                unsafe { Ok(Ok(T::results_lift(state.ptr_results()))) }
            }

            // This subtask was dropped which forced cancellation. Said
            // cancellation stopped the subtask before it reached the "started"
            // state, meaning that we still own all of the parameters in their
            // lowered form.
            //
            // In this situation we lift the parameters, even after we
            // previously lowered them, back into `T::Params`. That notably
            // re-acquires ownership and is suitable for disposing of all of
            // the parameters via normal Rust-based destructors.
            STATUS_STARTED_CANCELLED => {
                assert!(!state.started);
                unsafe {
                    T::params_dealloc_lists_and_own(state.params_lower);
                }
                Ok(Err(()))
            }

            // This subtask was dropped which forced cancellation. Said
            // cancellation stopped the subtask before it reached the "returned"
            // state, meaning that it started, received the arguments, but then
            // did not complete.
            //
            // In this situation we may have already received `STATUS_STARTED`,
            // but we also might not have. This means we conditionally need
            // to flag this task as started which will deallocate all lists
            // owned by the parameters.
            //
            // After that though we do not have ownership of the parameters any
            // more (e.g. own resources are all gone) so there's nothing to
            // return. Here we yield a result and dispose of the in-progress
            // state.
            STATUS_RETURNED_CANCELLED => {
                if !state.started {
                    state.flag_started();
                }
                Ok(Err(()))
            }

            other => panic!("unknown code {other:#x}"),
        }
    }

    fn in_progress_waitable(state: &Self::InProgress) -> u32 {
        // This shouldn't get called in the one case this isn't present: when
        // `STATUS_RETURNED` is returned and no waitable is created. That's the
        // `unwrap()` condition here.
        state.subtask.as_ref().unwrap().handle.get()
    }

    fn in_progress_cancel(state: &Self::InProgress) -> u32 {
        unsafe { cancel(Self::in_progress_waitable(state)) }
    }

    fn result_into_cancel(result: Self::Result) -> Self::Cancel {
        result
    }
}

#[derive(Debug)]
struct SubtaskHandle {
    handle: NonZeroU32,
}

impl Drop for SubtaskHandle {
    fn drop(&mut self) {
        unsafe {
            drop(self.handle.get());
        }
    }
}

struct InProgress<T: Subtask> {
    params_and_results: Option<Cleanup>,
    params_lower: T::ParamsLower,
    started: bool,
    subtask: Option<SubtaskHandle>,
    _marker: marker::PhantomData<T>,
}

impl<T: Subtask> InProgress<T> {
    fn flag_started(&mut self) {
        assert!(!self.started);
        self.started = true;

        // SAFETY: the initial entrypoint of `call` requires that the vtable is
        // setup correctly and we're obeying the invariants of the vtable,
        // deallocating lists in an allocation that we exclusively own.
        unsafe {
            T::params_dealloc_lists(self.params_lower);
        }
    }

    fn ptr_results(&self) -> *mut u8 {
        // SAFETY: the `T` trait has unsafely promised us that the offset is
        // in-bounds of the allocation layout.
        unsafe {
            self.params_and_results
                .as_ref()
                .map(|c| c.ptr.as_ptr())
                .unwrap_or(ptr::null_mut())
                .add(T::RESULTS_OFFSET)
        }
    }
}

#[cfg(not(target_arch = "wasm32"))]
unsafe fn drop(_: u32) {
    unreachable!()
}

#[cfg(not(target_arch = "wasm32"))]
unsafe fn cancel(_: u32) -> u32 {
    unreachable!()
}

#[cfg(target_arch = "wasm32")]
#[link(wasm_import_module = "$root")]
extern "C" {
    #[link_name = "[subtask-cancel]"]
    fn cancel(handle: u32) -> u32;
    #[link_name = "[subtask-drop]"]
    fn drop(handle: u32);
}
