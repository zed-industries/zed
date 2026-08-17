// SPDX-License-Identifier: Apache-2.0 OR MIT

//! Types and Traits for working with asynchronous tasks.

// This module is based on alloc::task::Wake.
//
// The code has been adjusted to work with stable Rust.
//
// Source: https://github.com/rust-lang/rust/blob/1.80.0/library/alloc/src/task.rs.
//
// Copyright & License of the original code:
// - https://github.com/rust-lang/rust/blob/1.80.0/COPYRIGHT
// - https://github.com/rust-lang/rust/blob/1.80.0/LICENSE-APACHE
// - https://github.com/rust-lang/rust/blob/1.80.0/LICENSE-MIT

use core::{
    mem::ManuallyDrop,
    task::{RawWaker, RawWakerVTable, Waker},
};

use crate::Arc;

/// The implementation of waking a task on an executor.
///
/// This is an equivalent to [`std::task::Wake`], but using [`portable_atomic_util::Arc`](crate::Arc)
/// as a reference-counted pointer. See the documentation for [`std::task::Wake`] for more details.
///
/// **Note:** Unlike `std::task::Wake`, all methods take `this:` instead of `self:`.
/// This is because using `portable_atomic_util::Arc` as a receiver requires the
/// [unstable `arbitrary_self_types` feature](https://github.com/rust-lang/rust/issues/44874).
///
/// # Examples
///
/// A basic `block_on` function that takes a future and runs it to completion on
/// the current thread.
///
/// **Note:** This example trades correctness for simplicity. In order to prevent
/// deadlocks, production-grade implementations will also need to handle
/// intermediate calls to `thread::unpark` as well as nested invocations.
///
/// ```
/// use portable_atomic_util::{task::Wake, Arc};
/// use std::{
///     future::Future,
///     task::{Context, Poll},
///     thread::{self, Thread},
/// };
///
/// /// A waker that wakes up the current thread when called.
/// struct ThreadWaker(Thread);
///
/// impl Wake for ThreadWaker {
///     fn wake(this: Arc<Self>) {
///         this.0.unpark();
///     }
/// }
///
/// /// Run a future to completion on the current thread.
/// fn block_on<T>(fut: impl Future<Output = T>) -> T {
///     // Pin the future so it can be polled.
///     let mut fut = Box::pin(fut);
///
///     // Create a new context to be passed to the future.
///     let t = thread::current();
///     let waker = Arc::new(ThreadWaker(t)).into();
///     let mut cx = Context::from_waker(&waker);
///
///     // Run the future to completion.
///     loop {
///         match fut.as_mut().poll(&mut cx) {
///             Poll::Ready(res) => return res,
///             Poll::Pending => thread::park(),
///         }
///     }
/// }
///
/// block_on(async {
///     println!("Hi from inside a future!");
/// });
/// ```
pub trait Wake {
    /// Wake this task.
    fn wake(this: Arc<Self>);

    /// Wake this task without consuming the waker.
    ///
    /// If an executor supports a cheaper way to wake without consuming the
    /// waker, it should override this method. By default, it clones the
    /// [`Arc`] and calls [`wake`] on the clone.
    ///
    /// [`wake`]: Wake::wake
    fn wake_by_ref(this: &Arc<Self>) {
        Self::wake(this.clone());
    }
}

impl<W: Wake + Send + Sync + 'static> From<Arc<W>> for Waker {
    /// Use a `Wake`-able type as a `Waker`.
    ///
    /// No heap allocations or atomic operations are used for this conversion.
    fn from(waker: Arc<W>) -> Self {
        // SAFETY: This is safe because raw_waker safely constructs
        // a RawWaker from Arc<W>.
        unsafe { Self::from_raw(raw_waker(waker)) }
    }
}

impl<W: Wake + Send + Sync + 'static> From<Arc<W>> for RawWaker {
    /// Use a `Wake`-able type as a `RawWaker`.
    ///
    /// No heap allocations or atomic operations are used for this conversion.
    fn from(waker: Arc<W>) -> Self {
        raw_waker(waker)
    }
}

// NB: This private function for constructing a RawWaker is used, rather than
// inlining this into the `From<Arc<W>> for RawWaker` impl, to ensure that
// the safety of `From<Arc<W>> for Waker` does not depend on the correct
// trait dispatch - instead both impls call this function directly and
// explicitly.
#[inline(always)]
fn raw_waker<W: Wake + Send + Sync + 'static>(waker: Arc<W>) -> RawWaker {
    // Increment the reference count of the arc to clone it.
    //
    // The #[inline(always)] is to ensure that raw_waker and clone_waker are
    // always generated in the same code generation unit as one another, and
    // therefore that the structurally identical const-promoted RawWakerVTable
    // within both functions is deduplicated at LLVM IR code generation time.
    // This allows optimizing Waker::will_wake to a single pointer comparison of
    // the vtable pointers, rather than comparing all four function pointers
    // within the vtables.
    #[inline(always)]
    unsafe fn clone_waker<W: Wake + Send + Sync + 'static>(waker: *const ()) -> RawWaker {
        // SAFETY: the caller must uphold the safety contract.
        unsafe { Arc::increment_strong_count(waker as *const W) };
        RawWaker::new(
            waker,
            &RawWakerVTable::new(clone_waker::<W>, wake::<W>, wake_by_ref::<W>, drop_waker::<W>),
        )
    }

    // Wake by value, moving the Arc into the Wake::wake function
    unsafe fn wake<W: Wake + Send + Sync + 'static>(waker: *const ()) {
        // SAFETY: the caller must uphold the safety contract.
        let waker = unsafe { Arc::from_raw(waker as *const W) };
        <W as Wake>::wake(waker);
    }

    // Wake by reference, wrap the waker in ManuallyDrop to avoid dropping it
    unsafe fn wake_by_ref<W: Wake + Send + Sync + 'static>(waker: *const ()) {
        // SAFETY: the caller must uphold the safety contract.
        let waker = unsafe { ManuallyDrop::new(Arc::from_raw(waker as *const W)) };
        <W as Wake>::wake_by_ref(&waker);
    }

    // Decrement the reference count of the Arc on drop
    unsafe fn drop_waker<W: Wake + Send + Sync + 'static>(waker: *const ()) {
        // SAFETY: the caller must uphold the safety contract.
        unsafe { Arc::decrement_strong_count(waker as *const W) };
    }

    RawWaker::new(
        Arc::into_raw(waker) as *const (),
        &RawWakerVTable::new(clone_waker::<W>, wake::<W>, wake_by_ref::<W>, drop_waker::<W>),
    )
}
