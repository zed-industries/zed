#![deny(missing_docs)]

extern crate std;
use core::sync::atomic::{AtomicU32, Ordering};
use std::boxed::Box;
use std::collections::BTreeMap;
use std::ffi::c_void;
use std::future::Future;
use std::mem;
use std::pin::Pin;
use std::ptr;
use std::sync::Arc;
use std::task::{Context, Poll, Wake, Waker};

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

/// Helper macro to deduplicate foreign definitions of wasm functions.
///
/// This automatically imports when on wasm targets and then defines a dummy
/// panicking shim for native targets to support native compilation but fail at
/// runtime.
macro_rules! extern_wasm {
    (
        $(#[$extern_attr:meta])*
        unsafe extern "C" {
            $(
                $(#[$func_attr:meta])*
                $vis:vis fn $func_name:ident ( $($args:tt)* ) $(-> $ret:ty)?;
            )*
        }
    ) => {
        $(
            #[cfg(not(target_family = "wasm"))]
            #[allow(unused, reason = "dummy shim for non-wasm compilation, never invoked")]
            $vis unsafe fn $func_name($($args)*) $(-> $ret)? {
                unreachable!();
            }
        )*

        #[cfg(target_family = "wasm")]
        $(#[$extern_attr])*
        unsafe extern "C" {
            $(
                $(#[$func_attr])*
                $vis fn $func_name($($args)*) $(-> $ret)?;
            )*
        }
    };
}

mod abi_buffer;
mod cabi;
mod error_context;
mod future_support;
#[cfg(feature = "inter-task-wakeup")]
mod inter_task_wakeup;
mod stream_support;
mod subtask;
#[cfg(feature = "inter-task-wakeup")]
mod unit_stream;
mod waitable;
mod waitable_set;

#[cfg(not(feature = "inter-task-wakeup"))]
use inter_task_wakeup_disabled as inter_task_wakeup;
#[cfg(not(feature = "inter-task-wakeup"))]
mod inter_task_wakeup_disabled;

use self::waitable_set::WaitableSet;
pub use abi_buffer::*;
pub use error_context::*;
pub use future_support::*;
pub use stream_support::*;
#[doc(hidden)]
pub use subtask::Subtask;
#[cfg(feature = "inter-task-wakeup")]
pub use unit_stream::*;

type BoxFuture<'a> = Pin<Box<dyn Future<Output = ()> + 'a>>;

#[cfg(feature = "async-spawn")]
mod spawn;
#[cfg(feature = "async-spawn")]
pub use spawn::spawn;
#[cfg(not(feature = "async-spawn"))]
mod spawn_disabled;
#[cfg(not(feature = "async-spawn"))]
use spawn_disabled as spawn;

/// Represents a task created by either a call to an async-lifted export or a
/// future run using `block_on` or `start_task`.
struct FutureState<'a> {
    /// Remaining work to do (if any) before this task can be considered "done".
    ///
    /// Note that we won't tell the host the task is done until this is drained
    /// and `waitables` is empty.
    tasks: spawn::Tasks<'a>,

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

    /// State related to supporting inter-task wakeup scenarios.
    inter_task_wakeup: inter_task_wakeup::State,
}

impl FutureState<'_> {
    fn new(future: BoxFuture<'_>) -> FutureState<'_> {
        let waker = Arc::new(FutureWaker::default());
        FutureState {
            waker_clone: waker.clone().into(),
            waker,
            tasks: spawn::Tasks::new(future),
            waitable_set: None,
            waitables: BTreeMap::new(),
            wasip3_task: cabi::wasip3_task {
                // This pointer is filled in before calling `wasip3_task_set`.
                ptr: ptr::null_mut(),
                version: cabi::WASIP3_TASK_V1,
                waitable_register,
                waitable_unregister,
            },
            inter_task_wakeup: Default::default(),
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
    fn callback(&mut self, event0: u32, event1: u32, event2: u32) -> CallbackCode {
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
                return CallbackCode::Exit;
            }
            _ => unreachable!(),
        }

        self.with_p3_task_set(|me| {
            // Transition our sleep state to ensure that the inter-task stream
            // isn't used since there's no need to use that here.
            me.waker
                .sleep_state
                .store(SLEEP_STATE_WOKEN, Ordering::Relaxed);

            // With all of our context now configured, deliver the event
            // notification this callback corresponds to.
            //
            // Note that this should happen under the reset of
            // `waker.sleep_state` above to ensure that if a waker is woken it
            // won't actually signal our inter-task stream since we're already
            // in the process of handling the future.
            if event0 != EVENT_NONE {
                me.deliver_waitable_event(event1, event2)
            }

            // If there's still an in-progress read (e.g. `event{1,2}`) wasn't
            // ourselves getting woken up, then cancel the read since we're
            // processing the future here anyway.
            me.cancel_inter_task_stream_read();

            loop {
                let mut context = Context::from_waker(&me.waker_clone);

                // On each turn of this loop reset the state to "polling"
                // which clears out any pending wakeup if one was sent. This
                // in theory helps minimize wakeups from previous iterations
                // happening in this iteration.
                me.waker
                    .sleep_state
                    .store(SLEEP_STATE_POLLING, Ordering::Relaxed);

                // Poll our future, seeing if it was able to make progress.
                let poll = me.tasks.poll_next(&mut context);

                match poll {
                    // A future completed, yay! Keep going to see if more have
                    // completed.
                    Poll::Ready(Some(())) => (),

                    // The task list is empty, but there might be remaining work
                    // in terms of waitables through the cabi interface. In this
                    // situation wait for all waitables to be resolved before
                    // signaling that our own task is done.
                    Poll::Ready(None) => {
                        assert!(me.tasks.is_empty());
                        if me.remaining_work() {
                            let waitable = me.waitable_set.as_ref().unwrap().as_raw();
                            break CallbackCode::Wait(waitable);
                        } else {
                            break CallbackCode::Exit;
                        }
                    }

                    // Some future within `self.tasks` is not ready yet. If our
                    // `waker` was signaled then that means this is a yield
                    // operation, otherwise it means we're blocking on
                    // something.
                    Poll::Pending => {
                        assert!(!me.tasks.is_empty());
                        if me.waker.sleep_state.load(Ordering::Relaxed) == SLEEP_STATE_WOKEN {
                            if me.remaining_work() {
                                let (event0, event1, event2) =
                                    me.waitable_set.as_ref().unwrap().poll();
                                if event0 != EVENT_NONE {
                                    me.deliver_waitable_event(event1, event2);
                                    continue;
                                }
                            }
                            break CallbackCode::Yield;
                        }

                        // Transition our state to "sleeping" so wakeup
                        // notifications know that they need to signal the
                        // inter-task stream.
                        me.waker
                            .sleep_state
                            .store(SLEEP_STATE_SLEEPING, Ordering::Relaxed);
                        me.read_inter_task_stream();
                        let waitable = me.waitable_set.as_ref().unwrap().as_raw();
                        break CallbackCode::Wait(waitable);
                    }
                }
            }
        })
    }

    /// Deliver the `code` event to the `waitable` store within our map. This
    /// waitable should be present because it's part of the waitable set which
    /// is kept in-sync with our map.
    fn deliver_waitable_event(&mut self, waitable: u32, code: u32) {
        self.remove_waitable(waitable);

        if self
            .inter_task_wakeup
            .consume_waitable_event(waitable, code)
        {
            return;
        }

        let (ptr, callback) = self.waitables.remove(&waitable).unwrap();
        unsafe {
            callback(ptr, code);
        }
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
        let self_raw = self as *mut FutureState<'_>;
        self.wasip3_task.ptr = self_raw.cast();
        let prev = unsafe { cabi::wasip3_task_set(&mut self.wasip3_task) };
        let _reset = ResetTask(prev);

        f(self)
    }
}

impl Drop for FutureState<'_> {
    fn drop(&mut self) {
        // If there's an active read of the inter-task stream, go ahead and
        // cancel it, since we're about to drop the stream anyway.
        self.cancel_inter_task_stream_read();

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
    let ptr = ptr.cast::<FutureState<'static>>();
    assert!(!ptr.is_null());
    unsafe {
        (*ptr).add_waitable(waitable);
        match (*ptr).waitables.insert(waitable, (callback_ptr, callback)) {
            Some((prev, _)) => prev,
            None => ptr::null_mut(),
        }
    }
}

unsafe extern "C" fn waitable_unregister(ptr: *mut c_void, waitable: u32) -> *mut c_void {
    let ptr = ptr.cast::<FutureState<'static>>();
    assert!(!ptr.is_null());
    unsafe {
        (*ptr).remove_waitable(waitable);
        match (*ptr).waitables.remove(&waitable) {
            Some((prev, _)) => prev,
            None => ptr::null_mut(),
        }
    }
}

/// Status for "this task is actively being polled"
const SLEEP_STATE_POLLING: u32 = 0;
/// Status for "this task has a wakeup scheduled, no more action need be taken".
const SLEEP_STATE_WOKEN: u32 = 1;
/// Status for "this task is not being polled and has not been woken"
///
/// Wakeups on this status signal the inter-task stream.
const SLEEP_STATE_SLEEPING: u32 = 2;

#[derive(Default)]
struct FutureWaker {
    /// One of `SLEEP_STATE_*` indicating the current status.
    sleep_state: AtomicU32,
    inter_task_stream: inter_task_wakeup::WakerState,
}

impl Wake for FutureWaker {
    fn wake(self: Arc<Self>) {
        Self::wake_by_ref(&self)
    }

    fn wake_by_ref(self: &Arc<Self>) {
        match self.sleep_state.swap(SLEEP_STATE_WOKEN, Ordering::Relaxed) {
            // If this future was currently being polled, or if someone else
            // already woke it up, then there's nothing to do.
            SLEEP_STATE_POLLING | SLEEP_STATE_WOKEN => {}

            // If this future is sleeping, however, then this is a cross-task
            // wakeup meaning that we need to write to its wakeup stream.
            other => {
                assert_eq!(other, SLEEP_STATE_SLEEPING);
                self.inter_task_stream.wake();
            }
        }
    }
}

const EVENT_NONE: u32 = 0;
const EVENT_SUBTASK: u32 = 1;
const EVENT_STREAM_READ: u32 = 2;
const EVENT_STREAM_WRITE: u32 = 3;
const EVENT_FUTURE_READ: u32 = 4;
const EVENT_FUTURE_WRITE: u32 = 5;
const EVENT_CANCEL: u32 = 6;

#[derive(PartialEq, Debug)]
enum CallbackCode {
    Exit,
    Yield,
    Wait(u32),
}

impl CallbackCode {
    fn encode(self) -> u32 {
        match self {
            CallbackCode::Exit => 0,
            CallbackCode::Yield => 1,
            CallbackCode::Wait(waitable) => 2 | (waitable << 4),
        }
    }
}

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
    let state = context_get().cast::<FutureState<'static>>();
    assert!(!state.is_null());
    unsafe {
        context_set(ptr::null_mut());
    }

    // Use `state` to run the `callback` function in the context of our event
    // codes we received. If the callback decides to exit then we're done with
    // our future so deallocate it. Otherwise put our future back in
    // context-local storage and forward the code.
    unsafe {
        let rc = (*state).callback(event0, event1, event2);
        if rc == CallbackCode::Exit {
            drop(Box::from_raw(state));
        } else {
            context_set(state.cast());
        }
        rtdebug!(" => (cb) {rc:?}");
        rc.encode()
    }
}

/// Run the specified future to completion, returning the result.
///
/// This uses `waitable-set.wait` to poll for progress on any in-progress calls
/// to async-lowered imports as necessary.
// TODO: refactor so `'static` bounds aren't necessary
pub fn block_on<T: 'static>(future: impl Future<Output = T>) -> T {
    let mut result = None;
    let mut state = FutureState::new(Box::pin(async {
        result = Some(future.await);
    }));
    let mut event = (EVENT_NONE, 0, 0);
    loop {
        match state.callback(event.0, event.1, event.2) {
            CallbackCode::Exit => {
                drop(state);
                break result.unwrap();
            }
            CallbackCode::Yield => event = state.waitable_set.as_ref().unwrap().poll(),
            CallbackCode::Wait(_) => event = state.waitable_set.as_ref().unwrap().wait(),
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
    extern_wasm! {
        #[link(wasm_import_module = "$root")]
        unsafe extern "C" {
            #[link_name = "[thread-yield]"]
            fn yield_() -> bool;
        }
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

/// Call the `backpressure.inc` canonical built-in function.
pub fn backpressure_inc() {
    extern_wasm! {
        #[link(wasm_import_module = "$root")]
        unsafe extern "C" {
            #[link_name = "[backpressure-inc]"]
            fn backpressure_inc();
        }
    }

    unsafe { backpressure_inc() }
}

/// Call the `backpressure.dec` canonical built-in function.
pub fn backpressure_dec() {
    extern_wasm! {
        #[link(wasm_import_module = "$root")]
        unsafe extern "C" {
            #[link_name = "[backpressure-dec]"]
            fn backpressure_dec();
        }
    }

    unsafe { backpressure_dec() }
}

fn context_get() -> *mut u8 {
    extern_wasm! {
        #[link(wasm_import_module = "$root")]
        unsafe extern "C" {
            #[link_name = "[context-get-0]"]
            fn get() -> *mut u8;
        }
    }

    unsafe { get() }
}

unsafe fn context_set(value: *mut u8) {
    extern_wasm! {
        #[link(wasm_import_module = "$root")]
        unsafe extern "C" {
            #[link_name = "[context-set-0]"]
            fn set(value: *mut u8);
        }
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
        extern_wasm! {
            #[link(wasm_import_module = "[export]$root")]
            unsafe extern "C" {
                #[link_name = "[task-cancel]"]
                fn cancel();
            }
        }

        unsafe { cancel() }
    }
}
