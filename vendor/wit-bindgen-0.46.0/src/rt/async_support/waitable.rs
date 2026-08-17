//! Generic support for "any waitable" and performing asynchronous operations on
//! that waitable.

use super::cabi;
use std::ffi::c_void;
use std::future::Future;
use std::marker;
use std::mem;
use std::pin::Pin;
use std::ptr;
use std::task::{Context, Poll, Waker};

/// Generic future-based operation on any "waitable" in the component model.
///
/// This is used right now to power futures and streams for both read/write
/// halves. This structure is driven by `S`, an implementation of
/// [`WaitableOp`], which codifies the various state transitions and what to do
/// on each state transition.
pub struct WaitableOperation<S: WaitableOp> {
    state: WaitableOperationState<S>,
    /// Storage for the final result of this asynchronous operation, if it's
    /// completed asynchronously.
    completion_status: CompletionStatus,
}

/// Structure used to store the `u32` return code from the canonical ABI about
/// an asynchronous operation.
///
/// When an asynchronous operation is started and it does not immediately
/// complete then this structure is used to asynchronously fill in the return
/// code. A `Pin<&mut CompletionStatus>` is used to register a pointer with
/// `FutureState` to get filled in.
///
/// Note that this means that this type is participating in unsafe lifetime
/// management and has properties it needs to uphold as a result. Specifically
/// the `PhantomPinned` field here means that `Pin` actually has meaning for
/// this structure, notably that once `Pin<&mut CompletionStatus>` is created
/// then it's guaranteed the destructor will be run before the backing memory
/// is deallocated. That's used in `WaitableOperation` above to share an
/// internal pointer of this data structure with `FuturesState` safely. The
/// destructor of `WaitableOperation` will deregister from `FutureState` meaning
/// that if `FuturesState` has a pointer here then it should be valid .
struct CompletionStatus {
    /// Where the async operation's code is filled in, and `None` until that
    /// happens.
    code: Option<u32>,

    waker: Option<Waker>,

    /// This is necessary to ensure that `Pin<&mut CompletionStatus>` carries
    /// the "pin guarantee", basically to mean that it's not safe to construct
    /// `Pin<&mut CompletionStatus>` and it must somehow require `unsafe` code.
    _pinned: marker::PhantomPinned,
}

/// Helper trait to be used with `WaitableOperation` to assist with machinery
/// necessary to track in-flight reads/writes on futures.
///
/// # Unsafety
///
/// This trait is `unsafe` as it has various guarantees that must be upheld by
/// implementors such as:
///
/// * `S::in_progress_waitable` must always return the same value for the state
///   given.
pub unsafe trait WaitableOp {
    /// Initial state of this operation, used to kick off the actual component
    /// model operation and transition to `InProgress`.
    type Start;

    /// Intermediate state of this operation when the component model is
    /// involved but it hasn't resolved just yet.
    type InProgress;

    /// Result type of this operation.
    type Result;

    /// Result of when this operation is cancelled.
    type Cancel;

    /// Starts the async operation.
    ///
    /// This method will actually call `{future,stream}.{read,write}` with
    /// `state` provided. The return code of the intrinsic is returned here
    /// along with the `InProgress` state.
    fn start(state: Self::Start) -> (u32, Self::InProgress);

    /// Optionally complete the async operation.
    ///
    /// This method will transition from the `InProgress` state, with some
    /// status code that was received, to either a completed result or a new
    /// `InProgress` state. This is invoked when:
    ///
    /// * a new status code has been received by an async export's `callback`
    /// * cancellation returned a code to be processed here
    fn in_progress_update(
        state: Self::InProgress,
        code: u32,
    ) -> Result<Self::Result, Self::InProgress>;

    /// Conversion from the "start" state to the "cancel" result, needed when an
    /// operation is cancelled before it's started.
    fn start_cancelled(state: Self::Start) -> Self::Cancel;

    /// Acquires the component-model `waitable` index that the `InProgress`
    /// state is waiting on.
    fn in_progress_waitable(state: &Self::InProgress) -> u32;

    /// Initiates a request for cancellation of this operation. Returns the
    /// status code returned by the `{future,stream}.cancel-{read,write}`
    /// intrinsic.
    ///
    /// Note that this must synchronously complete the operation somehow. This
    /// cannot return a status code indicating that an operation is pending,
    /// instead the operation must be complete with the returned code. That may
    /// mean that this intrinsic can block while figuring things out in the
    /// component model ABI, for example.
    fn in_progress_cancel(state: &Self::InProgress) -> u32;

    /// Converts a "completion result" into a "cancel result". This is necessary
    /// when an in-progress operation is cancelled so the in-progress result is
    /// first acquired and then transitioned to a cancel request.
    fn result_into_cancel(result: Self::Result) -> Self::Cancel;
}

enum WaitableOperationState<S: WaitableOp> {
    Start(S::Start),
    InProgress(S::InProgress),
    Done,
}

impl<S> WaitableOperation<S>
where
    S: WaitableOp,
{
    /// Creates a new operation in the initial state.
    pub fn new(state: S::Start) -> WaitableOperation<S> {
        WaitableOperation {
            state: WaitableOperationState::Start(state),
            completion_status: CompletionStatus {
                code: None,
                waker: None,
                _pinned: marker::PhantomPinned,
            },
        }
    }

    fn pin_project(
        self: Pin<&mut Self>,
    ) -> (&mut WaitableOperationState<S>, Pin<&mut CompletionStatus>) {
        // SAFETY: this is the one method used to project from `Pin<&mut Self>`
        // to the fields, and the contract we're deciding on is that
        // `state` is never pinned but the `CompletionStatus` is. That's used
        // to share a raw pointer with the completion callback with
        // respect to `Option<u32>` internally.
        unsafe {
            let me = self.get_unchecked_mut();
            (&mut me.state, Pin::new_unchecked(&mut me.completion_status))
        }
    }

    /// Registers a completion of `waitable` within the current task's future to:
    ///
    /// * Fill in `completion_status` with the result of a completion event.
    /// * Call `cx.waker().wake()`.
    pub fn register_waker(self: Pin<&mut Self>, waitable: u32, cx: &mut Context) {
        let (_, mut completion_status) = self.pin_project();
        debug_assert!(completion_status.as_mut().code_mut().is_none());
        *completion_status.as_mut().waker_mut() = Some(cx.waker().clone());

        // SAFETY: There's quite a lot going on here. First is the usage of
        // `task` below, and for that see `unregister_waker` below for why this
        // pattern should be safe.
        //
        // Otherwise we're handing off a pointer to `completion_status` to the
        // `task` itself. That should be safe as we're guaranteed, via
        // `Pin<&mut Self>`, that before `&mut Self` is deallocated the
        // destructor will be run which will perform de-registration via
        // cancellation.
        unsafe {
            let task = cabi::wasip3_task_set(ptr::null_mut());
            assert!(!task.is_null());
            assert!((*task).version >= cabi::WASIP3_TASK_V1);
            let ptr: *mut CompletionStatus = completion_status.get_unchecked_mut();
            let prev = ((*task).waitable_register)((*task).ptr, waitable, cabi_wake, ptr.cast());
            // We might be inserting a waker for the first time or overwriting
            // the previous waker. Only assert the expected value here if the
            // previous value was non-null.
            if !prev.is_null() {
                assert_eq!(ptr, prev.cast());
            }
            cabi::wasip3_task_set(task);
        }

        unsafe extern "C" fn cabi_wake(ptr: *mut c_void, code: u32) {
            let ptr: &mut CompletionStatus = &mut *ptr.cast::<CompletionStatus>();
            ptr.code = Some(code);
            ptr.waker.take().unwrap().wake()
        }
    }

    /// Deregisters the corresponding `register_waker` within the current task
    /// for the `waitable` passed here.
    ///
    /// This relinquishes control of the original `completion_status` pointer
    /// passed to `register_waker` after this call has completed.
    pub fn unregister_waker(self: Pin<&mut Self>, waitable: u32) {
        // SAFETY: the contract of `wasip3_task_set` is that the returned
        // pointer is valid for the lifetime of our entire task, so it's valid
        // for this stack frame. Additionally we assert it's non-null to
        // double-check it's initialized and additionally check the version for
        // the fields that we access.
        //
        // Otherwise the `waitable_unregister` callback should be safe because:
        //
        // * We're fulfilling the contract where the first argument must be
        //   `(*task).ptr`
        // * We own the `waitable` that we're passing in, so we're fulfilling
        //   the contract that arbitrary waitables for other units of work
        //   aren't being manipulated.
        unsafe {
            let task = cabi::wasip3_task_set(ptr::null_mut());
            assert!(!task.is_null());
            assert!((*task).version >= cabi::WASIP3_TASK_V1);
            let prev = ((*task).waitable_unregister)((*task).ptr, waitable);

            // Note that `_prev` here is not guaranteed to be either `NULL` or
            // not. A racy completion notification may have come in and
            // removed our waitable from the map even though we're in the
            // `InProgress` state, meaning it may not be present.
            //
            // The main thing is that after this method is called the
            // internal `completion_status` is guaranteed to no longer be in
            // `task`.
            //
            // Note, though, that if present this must be our `CompletionStatus`
            // pointer.
            if !prev.is_null() {
                let ptr: *mut CompletionStatus = self.pin_project().1.get_unchecked_mut();
                assert_eq!(ptr, prev.cast());
            }

            cabi::wasip3_task_set(task);
        }
    }

    /// Polls this operation to see if it has completed yet.
    ///
    /// This is intended to be used within `Future::poll`.
    pub fn poll_complete(mut self: Pin<&mut Self>, cx: &mut Context) -> Poll<S::Result> {
        use WaitableOperationState::*;

        let (state, completion_status) = self.as_mut().pin_project();

        // First up, determine the completion status, if any, that's available.
        let optional_code = match state {
            // If this operation hasn't actually started yet then now's the
            // time to start it.
            Start(_) => {
                let Start(s) = mem::replace(state, Done) else {
                    unreachable!()
                };
                let (code, s) = S::start(s);
                *state = InProgress(s);
                Some(code)
            }

            // This operation was previously queued so we're just waiting on
            // the completion to come in. Read the completion status and
            // interpret it down below.
            //
            // Note that it's the responsibility of the completion callback at
            // the ABI level that we install to fill in this pointer, e.g. it's
            // part of the `register_waker` contract.
            InProgress(_) => completion_status.code_mut().take(),

            // This write has already completed, it's a Rust-level API violation
            // to call this function again.
            Done => panic!("cannot re-poll after operation completes"),
        };

        self.poll_complete_with_code(Some(cx), optional_code)
    }

    /// After acquiring the current return of this operation in `optional_code`,
    /// figures out what to do with it.
    ///
    /// The `cx` argument is optional to do nothing in the case that
    /// `optional_code` is not present.
    fn poll_complete_with_code(
        mut self: Pin<&mut Self>,
        cx: Option<&mut Context>,
        optional_code: Option<u32>,
    ) -> Poll<S::Result> {
        use WaitableOperationState::*;

        let (state, _completion_status) = self.as_mut().pin_project();

        // If a status code is provided, then extract the in-progress state and
        // see what it thinks about this code. If we're done, yay! If not then
        // record the new in-progress state and fall through to registering a
        // waker.
        //
        // If no status code is available then that means we were polled before
        // the status came back, so just re-register the waker.
        if let Some(code) = optional_code {
            let InProgress(in_progress) = mem::replace(state, Done) else {
                unreachable!()
            };
            match S::in_progress_update(in_progress, code) {
                Ok(result) => return Poll::Ready(result),
                Err(in_progress) => *state = InProgress(in_progress),
            }
        }

        let in_progress = match state {
            InProgress(s) => s,
            _ => unreachable!(),
        };

        // The operation is still in progress.
        //
        // Register the `cx.waker()` to get notified when `writer.handle`
        // receives its completion.
        if let Some(cx) = cx {
            let handle = S::in_progress_waitable(in_progress);
            self.register_waker(handle, cx);
        }
        Poll::Pending
    }

    /// Cancels the in-flight operation, if it's still in-flight, and sees what
    /// happened.
    ///
    /// Defers to `S` how to communicate the current status through the
    /// cancellation type.
    ///
    /// # Panics
    ///
    /// Panics if the operation has already been completed via `poll_complete`
    /// above.
    /// Panics if this method is called twice.
    pub fn cancel(mut self: Pin<&mut Self>) -> S::Cancel {
        use WaitableOperationState::*;

        let (state, mut completion_status) = self.as_mut().pin_project();
        let in_progress = match state {
            // This operation was never actually started, so there's no need to
            // cancel anything, just pull out the value and return it.
            Start(_) => {
                let Start(s) = mem::replace(state, Done) else {
                    unreachable!()
                };
                return S::start_cancelled(s);
            }

            // This operation is actively in progress, fall through to below.
            InProgress(s) => s,

            // This operation was already completed after a `poll_complete`
            // above advanced to the `Done` state, or this was cancelled twice.
            // In such situations this is a programmer error to call this
            // method, so panic.
            Done => panic!("cannot cancel operation after completing it"),
        };

        // Our operation is in-progress, let's take a look at the pending
        // completion code, if any.
        match completion_status.as_mut().code_mut().take() {
            // A completion code, or status update, is available. This can
            // happen for example if an export received a status update for
            // this operation but then during the subsequent poll we decided
            // that the future should be dropped instead, aka a race between
            // two events. In this situation though to fully process the
            // cancellation we need to see what's up, so check to see if the
            // operation is done with this code.
            //
            // Note that in this branch it's known that this operation's waker
            // is not registered with the exported task because the exported
            // task already delivered us the completion code, which
            // automatically deregisters it at this time.
            Some(code) => {
                match self.as_mut().poll_complete_with_code(None, Some(code)) {
                    // The operation completed without us needing to cancel it,
                    // so just convert that to the `Cancel` type. In this
                    // situation no cancellation is necessary, the async
                    // operation is now inert, and we can immediately return.
                    Poll::Ready(result) => return S::result_into_cancel(result),

                    // The operation, despite receiving an update via a code,
                    // has not yet completed. In this case we do indeed need to
                    // perform cancellation, so fall through to below.
                    Poll::Pending => {}
                }
            }

            // A completion code is not yet available. In this situation we
            // deregister our waker from the exported task's waitable set and
            // callback handling since we'll be no longer waiting for events.
            // Cancellation below happens synchronously.
            //
            // After we've unregistered fall through to below.
            None => {
                let waitable = S::in_progress_waitable(in_progress);
                self.as_mut().unregister_waker(waitable);
            }
        }

        // This operation is guaranteed actively in progress at this point.
        // That means we really do in fact need to cancel it. Here the
        // appropriate cancellation intrinsic for the component model is
        // invoked which returns the final completion status for this
        // operation.
        //
        // The completion code is forwarded to `poll_complete_with_code` which
        // determines what happened as a result. Note that at this time
        // cancellation is required to be a synchronous operation in Rust, even
        // if it's async in the component model, since that's the only way for
        // this to be sound. Rust doesn't currently have linear types or async
        // destructors for example to ensure otherwise that if this were to
        // proceed asynchronously that we could rely on it being invoked.
        let (InProgress(in_progress), _) = self.as_mut().pin_project() else {
            unreachable!()
        };
        let code = S::in_progress_cancel(in_progress);
        match self.poll_complete_with_code(None, Some(code)) {
            Poll::Ready(result) => S::result_into_cancel(result),
            Poll::Pending => unreachable!(),
        }
    }
}

impl<S: WaitableOp> Future for WaitableOperation<S> {
    type Output = S::Result;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<S::Result> {
        self.poll_complete(cx)
    }
}

impl<S: WaitableOp> Drop for WaitableOperation<S> {
    fn drop(&mut self) {
        // SAFETY: we're in the destructor here so the value `self` is about
        // to go away and we can guarantee we're not moving out of it.
        let mut pin = unsafe { Pin::new_unchecked(self) };

        let (state, _) = pin.as_mut().pin_project();

        // If this operation has already completed then skip cancellation,
        // otherwise it's our job to cancel anything in-flight.
        if let WaitableOperationState::Done = state {
            return;
        }
        pin.cancel();
    }
}

impl CompletionStatus {
    fn code_mut(self: Pin<&mut Self>) -> &mut Option<u32> {
        unsafe { &mut self.get_unchecked_mut().code }
    }

    fn waker_mut(self: Pin<&mut Self>) -> &mut Option<Waker> {
        unsafe { &mut self.get_unchecked_mut().waker }
    }
}
