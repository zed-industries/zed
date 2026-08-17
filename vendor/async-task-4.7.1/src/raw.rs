use alloc::alloc::Layout as StdLayout;
use core::future::Future;
use core::marker::PhantomData;
use core::mem::{self, ManuallyDrop};
use core::pin::Pin;
use core::ptr::NonNull;
use core::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};

use core::sync::atomic::Ordering;

use crate::header::{DropWakerAction, Header, HeaderWithMetadata};
use crate::runnable::{Schedule, ScheduleInfo};
use crate::state::*;
use crate::utils::{abort, abort_on_panic, max, Layout};
use crate::Runnable;

#[cfg(feature = "std")]
pub(crate) type Panic = alloc::boxed::Box<dyn core::any::Any + Send + 'static>;

#[cfg(not(feature = "std"))]
pub(crate) type Panic = core::convert::Infallible;

/// The vtable for a task.
pub(crate) struct TaskVTable {
    pub(crate) raw_waker_vtable: &'static RawWakerVTable,

    /// Schedules the task.
    pub(crate) schedule: unsafe fn(*const (), ScheduleInfo),

    /// Drops the future inside the task.
    pub(crate) drop_future: unsafe fn(*const (), &TaskLayout),

    /// Destroys the task.
    pub(crate) destroy: unsafe fn(*const ()),

    /// Runs the task.
    pub(crate) run: unsafe fn(*const ()) -> bool,

    /// The memory layout of the task. This information enables
    /// debuggers to decode raw task memory blobs. Do not remove
    /// the field, even if it appears to be unused.
    pub(crate) layout_info: &'static TaskLayout,
}

impl TaskVTable {
    /// Returns a pointer to the output inside a task.
    pub(crate) unsafe fn get_output(&self, ptr: *const ()) -> *const () {
        ptr.add_byte(self.layout_info.offset_r)
    }
}

/// Memory layout of a task.
///
/// This struct contains the following information:
///
/// 1. How to allocate and deallocate the task.
/// 2. How to access the fields inside the task.
#[derive(Clone, Copy)]
pub(crate) struct TaskLayout {
    /// Memory layout of the whole task.
    pub(crate) layout: StdLayout,

    /// Offset into the task at which the schedule function is stored.
    pub(crate) offset_s: usize,

    /// Offset into the task at which the future is stored.
    pub(crate) offset_f: usize,

    /// Offset into the task at which the output is stored.
    pub(crate) offset_r: usize,
}

/// Raw pointers to the fields inside a task.
pub(crate) struct RawTask<F, T, S, M> {
    /// The task header.
    pub(crate) header: *const HeaderWithMetadata<M>,

    /// The schedule function.
    pub(crate) schedule: *const S,

    /// The future.
    pub(crate) future: *mut F,

    /// The output of the future.
    pub(crate) output: *mut Result<T, Panic>,
}

impl<F, T, S, M> Copy for RawTask<F, T, S, M> {}

impl<F, T, S, M> Clone for RawTask<F, T, S, M> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<F, T, S, M> RawTask<F, T, S, M> {
    pub(crate) const TASK_LAYOUT: TaskLayout = Self::eval_task_layout();

    /// Computes the memory layout for a task.
    #[inline]
    const fn eval_task_layout() -> TaskLayout {
        // Compute the layouts for `Header`, `S`, `F`, and `T`.
        let layout_header = Layout::new::<HeaderWithMetadata<M>>();
        let layout_s = Layout::new::<S>();
        let layout_f = Layout::new::<F>();
        let layout_r = Layout::new::<Result<T, Panic>>();

        // Compute the layout for `union { F, T }`.
        let size_union = max(layout_f.size(), layout_r.size());
        let align_union = max(layout_f.align(), layout_r.align());
        let layout_union = Layout::from_size_align(size_union, align_union);

        // Compute the layout for `Header` followed by `S` and `union { F, T }`.
        let layout = layout_header;
        let (layout, offset_s) = leap_unwrap!(layout.extend(layout_s));
        let (layout, offset_union) = leap_unwrap!(layout.extend(layout_union));
        let offset_f = offset_union;
        let offset_r = offset_union;

        TaskLayout {
            layout: unsafe { layout.into_std() },
            offset_s,
            offset_f,
            offset_r,
        }
    }
}

/// Allocates a task with the given `future` and `schedule` function.
///
/// It is assumed that initially only the `Runnable` and the `Task` exist.
///
/// Use a macro to brute force inlining to minimize stack copies of potentially
/// large futures.
macro_rules! allocate_task {
    ($f:tt, $s:tt, $m:tt, $builder:ident, $schedule:ident, $raw:ident => $future:block) => {{
        let allocation =
            alloc::alloc::alloc(RawTask::<$f, <$f as Future>::Output, $s, $m>::TASK_LAYOUT.layout);
        // Allocate enough space for the entire task.
        let ptr = NonNull::new(allocation as *mut ()).unwrap_or_else(|| crate::utils::abort());

        let $raw = RawTask::<$f, <$f as Future>::Output, $s, $m>::from_ptr(ptr.as_ptr());

        let crate::Builder {
            metadata,
            #[cfg(feature = "std")]
            propagate_panic,
        } = $builder;

        // Write the header as the first field of the task.
        ($raw.header as *mut HeaderWithMetadata<$m>).write(HeaderWithMetadata {
            header: Header {
                #[cfg(not(feature = "portable-atomic"))]
                state: core::sync::atomic::AtomicUsize::new(SCHEDULED | TASK | REFERENCE),
                #[cfg(feature = "portable-atomic")]
                state: portable_atomic::AtomicUsize::new(SCHEDULED | TASK | REFERENCE),
                awaiter: core::cell::UnsafeCell::new(None),
                vtable: &RawTask::<$f, <$f as Future>::Output, $s, $m>::TASK_VTABLE,
                #[cfg(feature = "std")]
                propagate_panic,
            },
            metadata,
        });

        // Write the schedule function as the third field of the task.
        ($raw.schedule as *mut S).write($schedule);

        // Explicitly avoid using abort_on_panic here to avoid extra stack
        // copies of the future on lower optimization levels.
        let bomb = crate::utils::Bomb;

        // Generate the future, now that the metadata has been pinned in place.
        // Write the future as the fourth field of the task.
        $raw.future.write($future);
        // (&(*raw.header).metadata)

        mem::forget(bomb);
        ptr
    }};
}

pub(crate) use allocate_task;

impl<F, T, S, M> RawTask<F, T, S, M>
where
    F: Future<Output = T>,
    S: Schedule<M>,
{
    pub(crate) const RAW_WAKER_VTABLE: RawWakerVTable = RawWakerVTable::new(
        Header::clone_waker,
        wake::<S, M>,
        wake_by_ref::<S, M>,
        drop_waker,
    );

    pub(crate) const TASK_VTABLE: TaskVTable = TaskVTable {
        raw_waker_vtable: &Self::RAW_WAKER_VTABLE,
        schedule: schedule::<S, M>,
        drop_future: drop_future::<F>,
        destroy: destroy::<S, M>,
        run: Self::run,
        layout_info: &Self::TASK_LAYOUT,
    };

    /// Creates a `RawTask` from a raw task pointer.
    #[inline]
    pub(crate) fn from_ptr(ptr: *const ()) -> Self {
        unsafe {
            Self {
                header: ptr as *const HeaderWithMetadata<M>,
                schedule: ptr.add_byte(Self::TASK_LAYOUT.offset_s) as *const S,
                future: ptr.add_byte(Self::TASK_LAYOUT.offset_f) as *mut F,
                output: ptr.add_byte(Self::TASK_LAYOUT.offset_r) as *mut Result<T, Panic>,
            }
        }
    }

    /// Runs a task.
    ///
    /// If polling its future panics, the task will be closed and the panic will be propagated into
    /// the caller.
    unsafe fn run(ptr: *const ()) -> bool {
        let raw = Self::from_ptr(ptr);
        let header = ptr as *const Header;

        // Create a context from the raw task pointer and the vtable inside the its header.
        let waker = ManuallyDrop::new(Waker::from_raw(RawWaker::new(ptr, &Self::RAW_WAKER_VTABLE)));
        let cx = &mut Context::from_waker(&waker);

        let mut state = (*header).state.load(Ordering::Acquire);

        // Update the task's state before polling its future.
        loop {
            // If the task has already been closed, drop the task reference and return.
            if state & CLOSED != 0 {
                // Drop the future.
                drop_future::<F>(ptr, &Self::TASK_LAYOUT);

                // Mark the task as unscheduled.
                let state = (*header).state.fetch_and(!SCHEDULED, Ordering::AcqRel);

                // Take the awaiter out.
                let mut awaiter = None;
                if state & AWAITER != 0 {
                    awaiter = (*header).take(None);
                }

                // Drop the task reference.
                drop_ref(ptr);

                // Notify the awaiter that the future has been dropped.
                if let Some(w) = awaiter {
                    abort_on_panic(|| w.wake());
                }
                return false;
            }

            // Mark the task as unscheduled and running.
            match (*header).state.compare_exchange_weak(
                state,
                (state & !SCHEDULED) | RUNNING,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    // Update the state because we're continuing with polling the future.
                    state = (state & !SCHEDULED) | RUNNING;
                    break;
                }
                Err(s) => state = s,
            }
        }

        // Poll the inner future, but surround it with a guard that closes the task in case polling
        // panics.
        // If available, we should also try to catch the panic so that it is propagated correctly.
        let guard = Guard::<F, T>(ptr, &Self::TASK_LAYOUT, PhantomData);

        // Panic propagation is not available for no_std.
        #[cfg(not(feature = "std"))]
        let poll = <F as Future>::poll(Pin::new_unchecked(&mut *raw.future), cx).map(Ok);

        #[cfg(feature = "std")]
        let poll = {
            // Check if we should propagate panics.
            if (*header).propagate_panic {
                // Use catch_unwind to catch the panic.
                match std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                    <F as Future>::poll(Pin::new_unchecked(&mut *raw.future), cx)
                })) {
                    Ok(Poll::Ready(v)) => Poll::Ready(Ok(v)),
                    Ok(Poll::Pending) => Poll::Pending,
                    Err(e) => Poll::Ready(Err(e)),
                }
            } else {
                <F as Future>::poll(Pin::new_unchecked(&mut *raw.future), cx).map(Ok)
            }
        };

        mem::forget(guard);

        match poll {
            Poll::Ready(out) => {
                // Replace the future with its output.
                drop_future::<F>(ptr, &Self::TASK_LAYOUT);
                raw.output.write(out);

                // The task is now completed.
                loop {
                    // If the `Task` is dropped, we'll need to close it and drop the output.
                    let new = if state & TASK == 0 {
                        (state & !RUNNING & !SCHEDULED) | COMPLETED | CLOSED
                    } else {
                        (state & !RUNNING & !SCHEDULED) | COMPLETED
                    };

                    // Mark the task as not running and completed.
                    match (*header).state.compare_exchange_weak(
                        state,
                        new,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    ) {
                        Ok(_) => {
                            // If the `Task` is dropped or if the task was closed while running,
                            // now it's time to drop the output.
                            if state & TASK == 0 || state & CLOSED != 0 {
                                // Drop the output.
                                abort_on_panic(|| raw.output.drop_in_place());
                            }

                            // Take the awaiter out.
                            let mut awaiter = None;
                            if state & AWAITER != 0 {
                                awaiter = (*header).take(None);
                            }

                            // Drop the task reference.
                            drop_ref(ptr);

                            // Notify the awaiter that the future has been dropped.
                            if let Some(w) = awaiter {
                                abort_on_panic(|| w.wake());
                            }
                            break;
                        }
                        Err(s) => state = s,
                    }
                }
            }
            Poll::Pending => {
                let mut future_dropped = false;

                // The task is still not completed.
                loop {
                    // If the task was closed while running, we'll need to unschedule in case it
                    // was woken up and then destroy it.
                    let new = if state & CLOSED != 0 {
                        state & !RUNNING & !SCHEDULED
                    } else {
                        state & !RUNNING
                    };

                    if state & CLOSED != 0 && !future_dropped {
                        // The thread that closed the task didn't drop the future because it was
                        // running so now it's our responsibility to do so.
                        drop_future::<F>(ptr, &Self::TASK_LAYOUT);
                        future_dropped = true;
                    }

                    // Mark the task as not running.
                    match (*header).state.compare_exchange_weak(
                        state,
                        new,
                        Ordering::AcqRel,
                        Ordering::Acquire,
                    ) {
                        Ok(state) => {
                            // If the task was closed while running, we need to notify the awaiter.
                            // If the task was woken up while running, we need to schedule it.
                            // Otherwise, we just drop the task reference.
                            if state & CLOSED != 0 {
                                // Take the awaiter out.
                                let mut awaiter = None;
                                if state & AWAITER != 0 {
                                    awaiter = (*header).take(None);
                                }

                                // Drop the task reference.
                                drop_ref(ptr);

                                // Notify the awaiter that the future has been dropped.
                                if let Some(w) = awaiter {
                                    abort_on_panic(|| w.wake());
                                }
                            } else if state & SCHEDULED != 0 {
                                // The thread that woke the task up didn't reschedule it because
                                // it was running so now it's our responsibility to do so.
                                schedule::<S, M>(ptr, ScheduleInfo::new(true));
                                return true;
                            } else {
                                // Drop the task reference.
                                drop_ref(ptr);
                            }
                            break;
                        }
                        Err(s) => state = s,
                    }
                }
            }
        }

        false
    }
}

/// A guard that closes the task if polling its future panics.
struct Guard<F, T>(*const (), &'static TaskLayout, PhantomData<fn() -> F>)
where
    F: Future<Output = T>;

impl<F, T> Drop for Guard<F, T>
where
    F: Future<Output = T>,
{
    fn drop(&mut self) {
        let ptr = self.0;
        let task_layout = self.1;
        let header = ptr as *const Header;

        unsafe {
            let header = &*header;
            let mut state = header.state.load(Ordering::Acquire);

            loop {
                // If the task was closed while running, then unschedule it, drop its
                // future, and drop the task reference.
                if state & CLOSED != 0 {
                    // The thread that closed the task didn't drop the future because it
                    // was running so now it's our responsibility to do so.
                    drop_future::<F>(ptr, task_layout);

                    // Mark the task as not running and not scheduled.
                    header
                        .state
                        .fetch_and(!RUNNING & !SCHEDULED, Ordering::AcqRel);

                    // Take the awaiter out.
                    let mut awaiter = None;
                    if state & AWAITER != 0 {
                        awaiter = header.take(None);
                    }

                    // Drop the task reference.
                    drop_ref(ptr);

                    // Notify the awaiter that the future has been dropped.
                    if let Some(w) = awaiter {
                        abort_on_panic(|| w.wake());
                    }
                    break;
                }

                // Mark the task as not running, not scheduled, and closed.
                match header.state.compare_exchange_weak(
                    state,
                    (state & !RUNNING & !SCHEDULED) | CLOSED,
                    Ordering::AcqRel,
                    Ordering::Acquire,
                ) {
                    Ok(state) => {
                        // Drop the future because the task is now closed.
                        drop_future::<F>(ptr, task_layout);

                        // Take the awaiter out.
                        let mut awaiter = None;
                        if state & AWAITER != 0 {
                            awaiter = header.take(None);
                        }

                        // Drop the task reference.
                        drop_ref(ptr);

                        // Notify the awaiter that the future has been dropped.
                        if let Some(w) = awaiter {
                            abort_on_panic(|| w.wake());
                        }
                        break;
                    }
                    Err(s) => state = s,
                }
            }
        }
    }
}

/// Schedules a task for running.
///
/// This function doesn't modify the state of the task. It only passes the task reference to
/// its schedule function.
unsafe fn schedule<S: Schedule<M>, M>(ptr: *const (), info: ScheduleInfo) {
    let header = ptr as *const Header;
    let task_layout = (*header).vtable.layout_info;
    let schedule = ptr.add_byte(task_layout.offset_s) as *mut S;

    // If the schedule function has captured variables, create a temporary waker that prevents
    // the task from getting deallocated while the function is being invoked.
    let _waker;
    if mem::size_of::<S>() > 0 {
        _waker = Waker::from_raw(Header::clone_waker(ptr));
    }

    let task = Runnable::from_raw(NonNull::new_unchecked(ptr as *mut ()));
    (*schedule).schedule(task, info);
}

/// Drops a waker.
///
/// This function will decrement the reference count. If it drops down to zero, the associated
/// `Task` has been dropped too, and the task has not been completed, then it will get
/// scheduled one more time so that its future gets dropped by the executor.
#[inline]
unsafe fn drop_waker(ptr: *const ()) {
    let header = ptr as *const Header;
    match Header::drop_waker(ptr) {
        DropWakerAction::Schedule => ((*header).vtable.schedule)(ptr, ScheduleInfo::new(false)),
        DropWakerAction::Destroy => ((*header).vtable.destroy)(ptr),
        DropWakerAction::None => {}
    }
}

/// Drops the future inside a task.
#[inline]
unsafe fn drop_future<F>(ptr: *const (), task_layout: &TaskLayout) {
    let future_ptr = ptr.add_byte(task_layout.offset_f) as *mut F;

    // We need a safeguard against panics because the destructor can panic.
    abort_on_panic(|| {
        future_ptr.drop_in_place();
    })
}

/// Wakes a waker.
unsafe fn wake<S: Schedule<M>, M>(ptr: *const ()) {
    // This is just an optimization. If the schedule function has captured variables, then
    // we'll do less reference counting if we wake the waker by reference and then drop it.
    if mem::size_of::<S>() > 0 {
        wake_by_ref::<S, M>(ptr);
        drop_waker(ptr);
        return;
    }

    let header = ptr as *const Header;

    let mut state = (*header).state.load(Ordering::Acquire);

    loop {
        // If the task is completed or closed, it can't be woken up.
        if state & (COMPLETED | CLOSED) != 0 {
            // Drop the waker.
            drop_waker(ptr);
            break;
        }

        // If the task is already scheduled, we just need to synchronize with the thread that
        // will run the task by "publishing" our current view of the memory.
        if state & SCHEDULED != 0 {
            // Update the state without actually modifying it.
            match (*header).state.compare_exchange_weak(
                state,
                state,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    // Drop the waker.
                    drop_waker(ptr);
                    break;
                }
                Err(s) => state = s,
            }
        } else {
            // Mark the task as scheduled.
            match (*header).state.compare_exchange_weak(
                state,
                state | SCHEDULED,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    // If the task is not yet scheduled and isn't currently running, now is the
                    // time to schedule it.
                    if state & RUNNING == 0 {
                        // Schedule the task.
                        schedule::<S, M>(ptr, ScheduleInfo::new(false));
                    } else {
                        // Drop the waker.
                        drop_waker(ptr);
                    }

                    break;
                }
                Err(s) => state = s,
            }
        }
    }
}

/// Wakes a waker by reference.
unsafe fn wake_by_ref<S: Schedule<M>, M>(ptr: *const ()) {
    let header = ptr as *const Header;
    let header = &*header;
    let task_layout = header.vtable.layout_info;

    let mut state = header.state.load(Ordering::Acquire);

    loop {
        // If the task is completed or closed, it can't be woken up.
        if state & (COMPLETED | CLOSED) != 0 {
            break;
        }

        // If the task is already scheduled, we just need to synchronize with the thread that
        // will run the task by "publishing" our current view of the memory.
        if state & SCHEDULED != 0 {
            // Update the state without actually modifying it.
            match header.state.compare_exchange_weak(
                state,
                state,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => break,
                Err(s) => state = s,
            }
        } else {
            // If the task is not running, we can schedule right away.
            let new = if state & RUNNING == 0 {
                (state | SCHEDULED) + REFERENCE
            } else {
                state | SCHEDULED
            };

            // Mark the task as scheduled.
            match header.state.compare_exchange_weak(
                state,
                new,
                Ordering::AcqRel,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    // If the task is not running, now is the time to schedule.
                    if state & RUNNING == 0 {
                        // If the reference count overflowed, abort.
                        if state > isize::MAX as usize {
                            abort();
                        }

                        let schedule = ptr.add_byte(task_layout.offset_s) as *mut S;

                        // Schedule the task. There is no need to call `Self::schedule(ptr)`
                        // because the schedule function cannot be destroyed while the waker is
                        // still alive.
                        let task = Runnable::from_raw(NonNull::new_unchecked(ptr as *mut ()));
                        (*schedule).schedule(task, ScheduleInfo::new(false));
                    }

                    break;
                }
                Err(s) => state = s,
            }
        }
    }
}

/// Cleans up task's resources and deallocates it.
///
/// The schedule function will be dropped, and the task will then get deallocated.
/// The task must be closed before this function is called.
#[inline]
unsafe fn destroy<S, M>(ptr: *const ()) {
    let header = ptr as *const Header;
    let task_layout = (*header).vtable.layout_info;
    let schedule = ptr.add_byte(task_layout.offset_s);

    // We need a safeguard against panics because destructors can panic.
    abort_on_panic(|| {
        // Drop the header along with the metadata.
        (ptr as *mut HeaderWithMetadata<M>).drop_in_place();

        // Drop the schedule function.
        (schedule as *mut S).drop_in_place();
    });

    // Finally, deallocate the memory reserved by the task.
    alloc::alloc::dealloc(ptr as *mut u8, task_layout.layout);
}

/// Drops a task reference (`Runnable` or `Waker`).
///
/// This function will decrement the reference count. If it drops down to zero and the
/// associated `Task` handle has been dropped too, then the task gets destroyed.
#[inline]
pub(crate) unsafe fn drop_ref(ptr: *const ()) {
    let header = ptr as *const Header;
    let header = &*header;

    // Decrement the reference count.
    let new = header.state.fetch_sub(REFERENCE, Ordering::AcqRel) - REFERENCE;

    // If this was the last reference to the task and the `Task` has been dropped too,
    // then destroy the task.
    if new & !(REFERENCE - 1) == 0 && new & TASK == 0 {
        (header.vtable.destroy)(ptr);
    }
}

trait PointerPolyfill {
    // Polyfill for `byte_add`.
    // TODO: Replace this with `byte_add` once the MSRV should be bumped past 1.75
    /// Adds an unsigned offset in bytes to a pointer.
    ///
    /// `count` is in units of bytes.
    ///
    /// This is purely a convenience for casting to a `u8` pointer and
    /// using [add][pointer::add] on it. See that method for documentation
    /// and safety requirements.
    ///
    /// # Safety
    /// If any of the following conditions are violated, the result is Undefined Behavior:
    ///
    ///  - The offset in bytes, count * size_of::<T>(), computed on mathematical integers
    ///    (without “wrapping around”), must fit in an isize.
    ///  - If the computed offset is non-zero, then self must be derived from a pointer to
    ///    some allocation, and the entire memory range between self and the result must be
    ///    in bounds of that allocation. In particular, this range must not “wrap around”
    ///    the edge of the address space.
    unsafe fn add_byte(self, size: usize) -> Self;
}

impl<T> PointerPolyfill for *const T {
    #[inline]
    unsafe fn add_byte(self, size: usize) -> Self {
        (self.cast::<u8>().add(size)).cast::<T>()
    }
}
