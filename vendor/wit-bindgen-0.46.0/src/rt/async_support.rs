#![deny(missing_docs)]
// TODO: Switch to interior mutability (e.g. use Mutexes or thread-local
// RefCells) and remove this, since even in single-threaded mode `static mut`
// references can be a hazard due to recursive access.
#![allow(static_mut_refs)]

extern crate std;
use core::sync::atomic::{AtomicBool, Ordering};
use std::boxed::Box;
use std::collections::BTreeMap;
use std::ffi::c_void;
use std::future::Future;
use std::mem;
use std::pin::Pin;
use std::ptr;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};
use std::vec::Vec;

use futures::channel::oneshot;
use futures::future::FutureExt;
use futures::stream::{FuturesUnordered, StreamExt};

macro_rules! rtdebug {
    ($($f:tt)*) => {
        // Change this flag to enable debugging, right now we're not using a
        // crate like `log` or such to reduce runtime deps. Intended to be used
        // during development for now.
        if false {
            std::eprintln!($($f)*);
        }
    }

}

mod abi_buffer;
mod cabi;
mod error_context;
mod future_support;
mod stream_support;
mod subtask;
mod waitable;
mod waitable_set;

use self::waitable_set::WaitableSet;
pub use abi_buffer::*;
pub use error_context::*;
pub use future_support::*;
pub use stream_support::*;
#[doc(hidden)]
pub use subtask::Subtask;

pub use futures;

type BoxFuture = Pin<Box<dyn Future<Output = ()> + 'static>>;

/// Represents a task created by either a call to an async-lifted export or a
/// future run using `block_on` or `start_task`.
struct FutureState {
    /// Remaining work to do (if any) before this task can be considered "done".
    ///
    /// Note that we won't tell the host the task is done until this is drained
    /// and `waitables` is empty.
    tasks: FuturesUnordered<BoxFuture>,

    /// The waitable set containing waitables created by this task, if any.
    waitable_set: Option<WaitableSet>,

    /// State of all waitables in `waitable_set`, and the ptr/callback they're
    /// associated with.
    //
    // Note that this is a `BTreeMap` rather than a `HashMap` only because, as
    // of this writing, initializing the default hasher for `HashMap` requires
    // calling `wasi_snapshot_preview1:random_get`, which requires initializing
    // the `wasi_snapshot_preview1` adapter when targeting `wasm32-wasip2` and
    // later, and that's expensive enough that we'd prefer to avoid it for apps
    // which otherwise make no use of the adapter.
    waitables: BTreeMap<u32, (*mut c_void, unsafe extern "C" fn(*mut c_void, u32))>,

    /// Raw structure used to pass to `cabi::wasip3_task_set`
    wasip3_task: cabi::wasip3_task,

    /// Rust-level state for the waker, notably a bool as to whether this has
    /// been woken.
    waker: Arc<FutureWaker>,

    /// Clone of `waker` field, but represented as `std::task::Waker`.
    waker_clone: Waker,
}

impl FutureState {
    fn new(future: BoxFuture) -> FutureState {
        let waker = Arc::new(FutureWaker::default());
        FutureState {
            waker_clone: waker.clone().into(),
            waker,
            tasks: [future].into_iter().collect(),
            waitable_set: None,
            waitables: BTreeMap::new(),
            wasip3_task: cabi::wasip3_task {
                // This pointer is filled in before calling `wasip3_task_set`.
                ptr: ptr::null_mut(),
                version: cabi::WASIP3_TASK_V1,
                waitable_register,
                waitable_unregister,
            },
        }
    }

    fn get_or_create_waitable_set(&mut self) -> &WaitableSet {
        self.waitable_set.get_or_insert_with(WaitableSet::new)
    }

    fn add_waitable(&mut self, waitable: u32) {
        self.get_or_create_waitable_set().join(waitable)
    }

    fn remove_waitable(&mut self, waitable: u32) {
        WaitableSet::remove_waitable_from_all_sets(waitable)
    }

    fn remaining_work(&self) -> bool {
        !self.waitables.is_empty()
    }

    /// Handles the `event{0,1,2}` event codes and returns a corresponding
    /// return code along with a flag whether this future is "done" or not.
    fn callback(&mut self, event0: u32, event1: u32, event2: u32) -> (u32, bool) {
        match event0 {
            EVENT_NONE => rtdebug!("EVENT_NONE"),
            EVENT_SUBTASK => rtdebug!("EVENT_SUBTASK({event1:#x}, {event2:#x})"),
            EVENT_STREAM_READ => rtdebug!("EVENT_STREAM_READ({event1:#x}, {event2:#x})"),
            EVENT_STREAM_WRITE => rtdebug!("EVENT_STREAM_WRITE({event1:#x}, {event2:#x})"),
            EVENT_FUTURE_READ => rtdebug!("EVENT_FUTURE_READ({event1:#x}, {event2:#x})"),
            EVENT_FUTURE_WRITE => rtdebug!("EVENT_FUTURE_WRITE({event1:#x}, {event2:#x})"),
            EVENT_CANCEL => {
                rtdebug!("EVENT_CANCEL");

                // Cancellation is mapped to destruction in Rust, so return a
                // code/bool indicating we're done. The caller will then
                // appropriately deallocate this `FutureState` which will
                // transitively run all destructors.
                return (CALLBACK_CODE_EXIT, true);
            }
            _ => unreachable!(),
        }
        if event0 != EVENT_NONE {
            self.deliver_waitable_event(event1, event2)
        }

        self.poll()
    }

    /// Deliver the `code` event to the `waitable` store within our map. This
    /// waitable should be present because it's part of the waitable set which
    /// is kept in-sync with our map.
    fn deliver_waitable_event(&mut self, waitable: u32, code: u32) {
        self.remove_waitable(waitable);
        let (ptr, callback) = self.waitables.remove(&waitable).unwrap();
        unsafe {
            callback(ptr, code);
        }
    }

    /// Poll this task until it either completes or can't make immediate
    /// progress.
    ///
    /// Returns the code representing what happened along with a boolean as to
    /// whether this execution is done.
    fn poll(&mut self) -> (u32, bool) {
        self.with_p3_task_set(|me| {
            let mut context = Context::from_waker(&me.waker_clone);

            loop {
                // Reset the waker before polling to clear out any pending
                // notification, if any.
                me.waker.0.store(false, Ordering::Relaxed);

                // Poll our future, handling `SPAWNED` around this.
                let poll;
                unsafe {
                    poll = me.tasks.poll_next_unpin(&mut context);
                    if !SPAWNED.is_empty() {
                        me.tasks.extend(SPAWNED.drain(..));
                    }
                }

                match poll {
                    // A future completed, yay! Keep going to see if more have
                    // completed.
                    Poll::Ready(Some(())) => (),

                    // The `FuturesUnordered` list is empty meaning that there's no
                    // more work left to do, so we're done.
                    Poll::Ready(None) => {
                        assert!(!me.remaining_work());
                        assert!(me.tasks.is_empty());
                        break (CALLBACK_CODE_EXIT, true);
                    }

                    // Some future within `FuturesUnordered` is not ready yet. If
                    // our `waker` was signaled then that means this is a yield
                    // operation, otherwise it means we're blocking on something.
                    Poll::Pending => {
                        assert!(!me.tasks.is_empty());
                        if me.waker.0.load(Ordering::Relaxed) {
                            break (CALLBACK_CODE_YIELD, false);
                        }

                        assert!(me.remaining_work());
                        let waitable = me.waitable_set.as_ref().unwrap().as_raw();
                        break (CALLBACK_CODE_WAIT | (waitable << 4), false);
                    }
                }
            }
        })
    }

    fn with_p3_task_set<R>(&mut self, f: impl FnOnce(&mut Self) -> R) -> R {
        // Finish our `wasip3_task` by initializing its self-referential pointer,
        // and then register it for the duration of this function with
        // `wasip3_task_set`. The previous value of `wasip3_task_set` will get
        // restored when this function returns.
        struct ResetTask(*mut cabi::wasip3_task);
        impl Drop for ResetTask {
            fn drop(&mut self) {
                unsafe {
                    cabi::wasip3_task_set(self.0);
                }
            }
        }
        let self_raw = self as *mut FutureState;
        self.wasip3_task.ptr = self_raw.cast();
        let prev = unsafe { cabi::wasip3_task_set(&mut self.wasip3_task) };
        let _reset = ResetTask(prev);

        f(self)
    }
}

impl Drop for FutureState {
    fn drop(&mut self) {
        // If this state has active tasks then they need to be dropped which may
        // execute arbitrary code. This arbitrary code might require the p3 APIs
        // for managing waitables, notably around removing them. In this
        // situation we ensure that the p3 task is set while futures are being
        // destroyed.
        if !self.tasks.is_empty() {
            self.with_p3_task_set(|me| {
                me.tasks = Default::default();
            })
        }
    }
}

unsafe extern "C" fn waitable_register(
    ptr: *mut c_void,
    waitable: u32,
    callback: unsafe extern "C" fn(*mut c_void, u32),
    callback_ptr: *mut c_void,
) -> *mut c_void {
    let ptr = ptr.cast::<FutureState>();
    assert!(!ptr.is_null());
    (*ptr).add_waitable(waitable);
    match (*ptr).waitables.insert(waitable, (callback_ptr, callback)) {
        Some((prev, _)) => prev,
        None => ptr::null_mut(),
    }
}

unsafe extern "C" fn waitable_unregister(ptr: *mut c_void, waitable: u32) -> *mut c_void {
    let ptr = ptr.cast::<FutureState>();
    assert!(!ptr.is_null());
    (*ptr).remove_waitable(waitable);
    match (*ptr).waitables.remove(&waitable) {
        Some((prev, _)) => prev,
        None => ptr::null_mut(),
    }
}

#[derive(Default)]
struct FutureWaker(AtomicBool);

impl Wake for FutureWaker {
    fn wake(self: Arc<Self>) {
        Self::wake_by_ref(&self)
    }

    fn wake_by_ref(self: &Arc<Self>) {
        self.0.store(true, Ordering::Relaxed)
    }
}

/// Any newly-deferred work queued by calls to the `spawn` function while
/// polling the current task.
static mut SPAWNED: Vec<BoxFuture> = Vec::new();

const EVENT_NONE: u32 = 0;
const EVENT_SUBTASK: u32 = 1;
const EVENT_STREAM_READ: u32 = 2;
const EVENT_STREAM_WRITE: u32 = 3;
const EVENT_FUTURE_READ: u32 = 4;
const EVENT_FUTURE_WRITE: u32 = 5;
const EVENT_CANCEL: u32 = 6;

const CALLBACK_CODE_EXIT: u32 = 0;
const CALLBACK_CODE_YIELD: u32 = 1;
const CALLBACK_CODE_WAIT: u32 = 2;
const _CALLBACK_CODE_POLL: u32 = 3;

const STATUS_STARTING: u32 = 0;
const STATUS_STARTED: u32 = 1;
const STATUS_RETURNED: u32 = 2;
const STATUS_STARTED_CANCELLED: u32 = 3;
const STATUS_RETURNED_CANCELLED: u32 = 4;

const BLOCKED: u32 = 0xffff_ffff;
const COMPLETED: u32 = 0x0;
const DROPPED: u32 = 0x1;
const CANCELLED: u32 = 0x2;

/// Return code of stream/future operations.
#[derive(PartialEq, Debug, Copy, Clone)]
enum ReturnCode {
    /// The operation is blocked and has not completed.
    Blocked,
    /// The operation completed with the specified number of items.
    Completed(u32),
    /// The other end is dropped, but before that the specified number of items
    /// were transferred.
    Dropped(u32),
    /// The operation was cancelled, but before that the specified number of
    /// items were transferred.
    Cancelled(u32),
}

impl ReturnCode {
    fn decode(val: u32) -> ReturnCode {
        if val == BLOCKED {
            return ReturnCode::Blocked;
        }
        let amt = val >> 4;
        match val & 0xf {
            COMPLETED => ReturnCode::Completed(amt),
            DROPPED => ReturnCode::Dropped(amt),
            CANCELLED => ReturnCode::Cancelled(amt),
            _ => panic!("unknown return code {val:#x}"),
        }
    }
}

/// Starts execution of the `task` provided, an asynchronous computation.
///
/// This is used for async-lifted exports at their definition site. The
/// representation of the export is `task` and this function is called from the
/// entrypoint. The code returned here is the same as the callback associated
/// with this export, and the callback will be used if this task doesn't exit
/// immediately with its result.
#[doc(hidden)]
pub fn start_task(task: impl Future<Output = ()> + 'static) -> i32 {
    // Allocate a new `FutureState` which will track all state necessary for
    // our exported task.
    let state = Box::into_raw(Box::new(FutureState::new(Box::pin(task))));

    // Store our `FutureState` into our context-local-storage slot and then
    // pretend we got EVENT_NONE to kick off everything.
    //
    // SAFETY: we should own `context.set` as we're the root level exported
    // task, and then `callback` is only invoked when context-local storage is
    // valid.
    unsafe {
        assert!(context_get().is_null());
        context_set(state.cast());
        callback(EVENT_NONE, 0, 0) as i32
    }
}

/// Handle a progress notification from the host regarding either a call to an
/// async-lowered import or a stream/future read/write operation.
///
/// # Unsafety
///
/// This function assumes that `context_get()` returns a `FutureState`.
#[doc(hidden)]
pub unsafe fn callback(event0: u32, event1: u32, event2: u32) -> u32 {
    // Acquire our context-local state, assert it's not-null, and then reset
    // the state to null while we're running to help prevent any unintended
    // usage.
    let state = context_get().cast::<FutureState>();
    assert!(!state.is_null());
    unsafe {
        context_set(ptr::null_mut());
    }

    // Use `state` to run the `callback` function in the context of our event
    // codes we received. If the callback decides to exit then we're done with
    // our future so deallocate it. Otherwise put our future back in
    // context-local storage and forward the code.
    unsafe {
        let (rc, done) = (*state).callback(event0, event1, event2);
        if done {
            drop(Box::from_raw(state));
        } else {
            context_set(state.cast());
        }
        rtdebug!(" => (cb) {rc:#x}");
        rc
    }
}

/// Defer the specified future to be run after the current async-lifted export
/// task has returned a value.
///
/// The task will remain in a running state until all spawned futures have
/// completed.
pub fn spawn(future: impl Future<Output = ()> + 'static) {
    unsafe { SPAWNED.push(Box::pin(future)) }
}

/// Run the specified future to completion, returning the result.
///
/// This uses `waitable-set.wait` to poll for progress on any in-progress calls
/// to async-lowered imports as necessary.
// TODO: refactor so `'static` bounds aren't necessary
pub fn block_on<T: 'static>(future: impl Future<Output = T> + 'static) -> T {
    let (tx, mut rx) = oneshot::channel();
    let state = &mut FutureState::new(Box::pin(future.map(move |v| drop(tx.send(v)))) as BoxFuture);
    let mut event = (EVENT_NONE, 0, 0);
    loop {
        match state.callback(event.0, event.1, event.2) {
            (_, true) => break rx.try_recv().unwrap().unwrap(),
            (CALLBACK_CODE_YIELD, false) => event = state.waitable_set.as_ref().unwrap().poll(),
            _ => event = state.waitable_set.as_ref().unwrap().wait(),
        }
    }
}

/// Call the `yield` canonical built-in function.
///
/// This yields control to the host temporarily, allowing other tasks to make
/// progress. It's a good idea to call this inside a busy loop which does not
/// otherwise ever yield control the host.
///
/// Note that this function is a blocking function, not an `async` function.
/// That means that this is not an async yield which allows other tasks in this
/// component to progress, but instead this will block the current function
/// until the host gets back around to returning from this yield. Asynchronous
/// functions should probably use [`yield_async`] instead.
///
/// # Return Value
///
/// This function returns a `bool` which indicates whether execution should
/// continue after this yield point. A return value of `true` means that the
/// task was not cancelled and execution should continue. A return value of
/// `false`, however, means that the task was cancelled while it was suspended
/// at this yield point. The caller should return back and exit from the task
/// ASAP in this situation.
pub fn yield_blocking() -> bool {
    #[cfg(not(target_arch = "wasm32"))]
    unsafe fn yield_() -> bool {
        unreachable!();
    }

    #[cfg(target_arch = "wasm32")]
    #[link(wasm_import_module = "$root")]
    extern "C" {
        #[link_name = "[thread-yield]"]
        fn yield_() -> bool;
    }
    // Note that the return value from the raw intrinsic is inverted, the
    // canonical ABI returns "did this task get cancelled" while this function
    // works as "should work continue going".
    unsafe { !yield_() }
}

/// The asynchronous counterpart to [`yield_blocking`].
///
/// This function does not block the current task but instead gives the
/// Rust-level executor a chance to yield control back to the host temporarily.
/// This means that other Rust-level tasks may also be able to progress during
/// this yield operation.
///
/// # Return Value
///
/// Unlike [`yield_blocking`] this function does not return anything. If this
/// component task is cancelled while paused at this yield point then the future
/// will be dropped and a Rust-level destructor will take over and clean up the
/// task. It's not necessary to do anything with the return value of this
/// function other than ensuring that you `.await` the function call.
pub async fn yield_async() {
    #[derive(Default)]
    struct Yield {
        yielded: bool,
    }

    impl Future for Yield {
        type Output = ();

        fn poll(mut self: Pin<&mut Self>, context: &mut Context<'_>) -> Poll<()> {
            if self.yielded {
                Poll::Ready(())
            } else {
                self.yielded = true;
                context.waker().wake_by_ref();
                Poll::Pending
            }
        }
    }

    Yield::default().await;
}

/// Call the `backpressure.set` canonical built-in function.
///
/// When `enabled` is `true`, this tells the host to defer any new calls to this
/// component instance until further notice (i.e. until `backpressure.set` is
/// called again with `enabled` set to `false`).
#[deprecated = "use backpressure_{inc,dec} instead"]
pub fn backpressure_set(enabled: bool) {
    #[cfg(not(target_arch = "wasm32"))]
    unsafe fn backpressure_set(_: i32) {
        unreachable!();
    }

    #[cfg(target_arch = "wasm32")]
    #[link(wasm_import_module = "$root")]
    extern "C" {
        #[link_name = "[backpressure-set]"]
        fn backpressure_set(_: i32);
    }

    unsafe { backpressure_set(if enabled { 1 } else { 0 }) }
}

/// Call the `backpressure.inc` canonical built-in function.
pub fn backpressure_inc() {
    #[cfg(not(target_arch = "wasm32"))]
    unsafe fn backpressure_inc() {
        unreachable!();
    }

    #[cfg(target_arch = "wasm32")]
    #[link(wasm_import_module = "$root")]
    extern "C" {
        #[link_name = "[backpressure-inc]"]
        fn backpressure_inc();
    }

    unsafe { backpressure_inc() }
}

/// Call the `backpressure.dec` canonical built-in function.
pub fn backpressure_dec() {
    #[cfg(not(target_arch = "wasm32"))]
    unsafe fn backpressure_dec() {
        unreachable!();
    }

    #[cfg(target_arch = "wasm32")]
    #[link(wasm_import_module = "$root")]
    extern "C" {
        #[link_name = "[backpressure-dec]"]
        fn backpressure_dec();
    }

    unsafe { backpressure_dec() }
}

fn context_get() -> *mut u8 {
    #[cfg(not(target_arch = "wasm32"))]
    unsafe fn get() -> *mut u8 {
        unreachable!()
    }

    #[cfg(target_arch = "wasm32")]
    #[link(wasm_import_module = "$root")]
    extern "C" {
        #[link_name = "[context-get-0]"]
        fn get() -> *mut u8;
    }

    unsafe { get() }
}

unsafe fn context_set(value: *mut u8) {
    #[cfg(not(target_arch = "wasm32"))]
    unsafe fn set(_: *mut u8) {
        unreachable!()
    }

    #[cfg(target_arch = "wasm32")]
    #[link(wasm_import_module = "$root")]
    extern "C" {
        #[link_name = "[context-set-0]"]
        fn set(value: *mut u8);
    }

    unsafe { set(value) }
}

#[doc(hidden)]
pub struct TaskCancelOnDrop {
    _priv: (),
}

impl TaskCancelOnDrop {
    #[doc(hidden)]
    pub fn new() -> TaskCancelOnDrop {
        TaskCancelOnDrop { _priv: () }
    }

    #[doc(hidden)]
    pub fn forget(self) {
        mem::forget(self);
    }
}

impl Drop for TaskCancelOnDrop {
    fn drop(&mut self) {
        #[cfg(not(target_arch = "wasm32"))]
        unsafe fn cancel() {
            unreachable!()
        }

        #[cfg(target_arch = "wasm32")]
        #[link(wasm_import_module = "[export]$root")]
        extern "C" {
            #[link_name = "[task-cancel]"]
            fn cancel();
        }

        unsafe { cancel() }
    }
}
