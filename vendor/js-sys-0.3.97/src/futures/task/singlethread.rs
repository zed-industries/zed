use alloc::boxed::Box;
use alloc::rc::Rc;
use core::cell::{Cell, RefCell};
use core::future::Future;
use core::mem::ManuallyDrop;
use core::pin::Pin;
use core::task::{Context, RawWaker, RawWakerVTable, Waker};

struct Inner {
    future: Pin<Box<dyn Future<Output = ()> + 'static>>,
    waker: Waker,
}

impl Inner {
    fn is_ready(&mut self) -> bool {
        let mut cx = Context::from_waker(&self.waker);
        self.future.as_mut().poll(&mut cx).is_ready()
    }
}

#[cfg(debug_assertions)]
#[wasm_bindgen::prelude::wasm_bindgen]
extern "C" {
    type ConsoleTask;

    #[wasm_bindgen(thread_local_v2, js_namespace = console, js_name = createTask)]
    static CREATE_TASK: Option<crate::Function<fn(crate::JsString) -> ConsoleTask>>;

    #[wasm_bindgen(method)]
    fn run(this: &ConsoleTask, poll: &mut dyn FnMut() -> bool) -> bool;
}

#[cfg(debug_assertions)]
fn try_create_task(name: &str) -> Option<ConsoleTask> {
    CREATE_TASK.with(|create_task| {
        create_task.as_ref().and_then(|f| {
            f.call(&wasm_bindgen::JsValue::UNDEFINED, (&name.into(),))
                .ok()
        })
    })
}

pub(crate) struct Task {
    // Console tracking for this task to avoid deeply nested stacks from individual `poll()` calls.
    // See [Linked Stack Traces](https://developer.chrome.com/blog/devtools-modern-web-debugging#linked_stack_traces).
    #[cfg(debug_assertions)]
    console: Option<ConsoleTask>,

    // The actual Future that we're executing as part of this task.
    //
    // This is an Option so that the Future can be immediately dropped when it's
    // finished
    inner: RefCell<Option<Inner>>,

    // This is used to ensure that the Task will only be queued once
    is_queued: Cell<bool>,
}

impl Task {
    pub(crate) fn spawn<F: Future<Output = ()> + 'static>(future: F) {
        let this = Rc::new(Self {
            #[cfg(debug_assertions)]
            console: try_create_task(core::any::type_name::<F>()),
            inner: RefCell::new(None),
            is_queued: Cell::new(true),
        });

        let waker = unsafe { Waker::from_raw(Task::into_raw_waker(Rc::clone(&this))) };

        *this.inner.borrow_mut() = Some(Inner {
            future: Box::pin(future),
            waker,
        });

        crate::futures::queue::Queue::with(|queue| queue.schedule_task(this));
    }

    fn force_wake(this: Rc<Self>) {
        crate::futures::queue::Queue::with(|queue| {
            queue.push_task(this);
        });
    }

    fn wake(this: Rc<Self>) {
        // If we've already been placed on the run queue then there's no need to
        // requeue ourselves since we're going to run at some point in the
        // future anyway.
        if this.is_queued.replace(true) {
            return;
        }

        Self::force_wake(this);
    }

    fn wake_by_ref(this: &Rc<Self>) {
        // If we've already been placed on the run queue then there's no need to
        // requeue ourselves since we're going to run at some point in the
        // future anyway.
        if this.is_queued.replace(true) {
            return;
        }

        Self::force_wake(Rc::clone(this));
    }

    /// Creates a standard library `RawWaker` from an `Rc` of ourselves.
    ///
    /// Note that in general this is wildly unsafe because everything with
    /// Futures requires `Sync` + `Send` with regard to Wakers. For wasm,
    /// however, everything is guaranteed to be singlethreaded (since we're
    /// compiled without the `atomics` feature) so we "safely lie" and say our
    /// `Rc` pointer is good enough.
    ///
    /// The implementation is based off of futures::task::ArcWake
    unsafe fn into_raw_waker(this: Rc<Self>) -> RawWaker {
        unsafe fn raw_clone(ptr: *const ()) -> RawWaker {
            let ptr = ManuallyDrop::new(Rc::from_raw(ptr as *const Task));
            Task::into_raw_waker(Rc::clone(&ptr))
        }

        unsafe fn raw_wake(ptr: *const ()) {
            let ptr = Rc::from_raw(ptr as *const Task);
            Task::wake(ptr);
        }

        unsafe fn raw_wake_by_ref(ptr: *const ()) {
            let ptr = ManuallyDrop::new(Rc::from_raw(ptr as *const Task));
            Task::wake_by_ref(&ptr);
        }

        unsafe fn raw_drop(ptr: *const ()) {
            drop(Rc::from_raw(ptr as *const Task));
        }

        static VTABLE: RawWakerVTable =
            RawWakerVTable::new(raw_clone, raw_wake, raw_wake_by_ref, raw_drop);

        RawWaker::new(Rc::into_raw(this) as *const (), &VTABLE)
    }

    pub(crate) fn run(&self) {
        let mut borrow = self.inner.borrow_mut();

        // Wakeups can come in after a Future has finished and been destroyed,
        // so handle this gracefully by just ignoring the request to run.
        let inner = match borrow.as_mut() {
            Some(inner) => inner,
            None => return,
        };

        // Ensure that if poll calls `waker.wake()` we can get enqueued back on
        // the run queue.
        self.is_queued.set(false);

        // In debug mode we want to avoid deeply nested stacks from individual
        // `poll()` calls, so we use `task.run` on a task created per future.
        #[cfg(debug_assertions)]
        let is_ready = match self.console.as_ref() {
            // Wrap `inner` in AssertUnwindSafe before capturing it, so the closure
            // satisfies MaybeUnwindSafe (required when panic=unwind). This is safe:
            // console.run's poll callback is not invoked inside a panic-catching context.
            Some(console) => {
                let mut inner = core::panic::AssertUnwindSafe(inner);
                console.run(&mut move || inner.is_ready())
            }
            None => inner.is_ready(),
        };

        // In release mode we prefer to avoid the overhead of the JS wrapper
        // and just poll directly.
        #[cfg(not(debug_assertions))]
        let is_ready = inner.is_ready();

        // If a future has finished (`Ready`) then clean up resources associated
        // with the future ASAP. This ensures that we don't keep anything extra
        // alive in-memory by accident. Our own struct, `Rc<Task>` won't
        // actually go away until all wakers referencing us go away, which may
        // take quite some time, so ensure that the heaviest of resources are
        // released early.
        if is_ready {
            *borrow = None;
        }
    }
}
