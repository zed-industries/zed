use super::Task;

use core::fmt;
use core::cell::UnsafeCell;
use core::sync::atomic::AtomicUsize;
use core::sync::atomic::Ordering::{Acquire, Release, AcqRel};

/// A synchronization primitive for task notification.
///
/// `AtomicTask` will coordinate concurrent notifications with the consumer
/// potentially "updating" the underlying task to notify. This is useful in
/// scenarios where a computation completes in another thread and wants to
/// notify the consumer, but the consumer is in the process of being migrated to
/// a new logical task.
///
/// Consumers should call `register` before checking the result of a computation
/// and producers should call `notify` after producing the computation (this
/// differs from the usual `thread::park` pattern). It is also permitted for
/// `notify` to be called **before** `register`. This results in a no-op.
///
/// A single `AtomicTask` may be reused for any number of calls to `register` or
/// `notify`.
///
/// `AtomicTask` does not provide any memory ordering guarantees, as such the
/// user should use caution and use other synchronization primitives to guard
/// the result of the underlying computation.
pub struct AtomicTask {
    state: AtomicUsize,
    task: UnsafeCell<Option<Task>>,
}

// `AtomicTask` is a multi-consumer, single-producer transfer cell. The cell
// stores a `Task` value produced by calls to `register` and many threads can
// race to take the task (to notify it) by calling `notify.
//
// If a new `Task` instance is produced by calling `register` before an existing
// one is consumed, then the existing one is overwritten.
//
// While `AtomicTask` is single-producer, the implementation ensures memory
// safety. In the event of concurrent calls to `register`, there will be a
// single winner whose task will get stored in the cell. The losers will not
// have their tasks notified. As such, callers should ensure to add
// synchronization to calls to `register`.
//
// The implementation uses a single `AtomicUsize` value to coordinate access to
// the `Task` cell. There are two bits that are operated on independently. These
// are represented by `REGISTERING` and `NOTIFYING`.
//
// The `REGISTERING` bit is set when a producer enters the critical section. The
// `NOTIFYING` bit is set when a consumer enters the critical section. Neither
// bit being set is represented by `WAITING`.
//
// A thread obtains an exclusive lock on the task cell by transitioning the
// state from `WAITING` to `REGISTERING` or `NOTIFYING`, depending on the
// operation the thread wishes to perform. When this transition is made, it is
// guaranteed that no other thread will access the task cell.
//
// # Registering
//
// On a call to `register`, an attempt to transition the state from WAITING to
// REGISTERING is made. On success, the caller obtains a lock on the task cell.
//
// If the lock is obtained, then the thread sets the task cell to the task
// provided as an argument. Then it attempts to transition the state back from
// `REGISTERING` -> `WAITING`.
//
// If this transition is successful, then the registering process is complete
// and the next call to `notify` will observe the task.
//
// If the transition fails, then there was a concurrent call to `notify` that
// was unable to access the task cell (due to the registering thread holding the
// lock). To handle this, the registering thread removes the task it just set
// from the cell and calls `notify` on it. This call to notify represents the
// attempt to notify by the other thread (that set the `NOTIFYING` bit). The
// state is then transitioned from `REGISTERING | NOTIFYING` back to `WAITING`.
// This transition must succeed because, at this point, the state cannot be
// transitioned by another thread.
//
// # Notifying
//
// On a call to `notify`, an attempt to transition the state from `WAITING` to
// `NOTIFYING` is made. On success, the caller obtains a lock on the task cell.
//
// If the lock is obtained, then the thread takes ownership of the current value
// in teh task cell, and calls `notify` on it. The state is then transitioned
// back to `WAITING`. This transition must succeed as, at this point, the state
// cannot be transitioned by another thread.
//
// If the thread is unable to obtain the lock, the `NOTIFYING` bit is still.
// This is because it has either been set by the current thread but the previous
// value included the `REGISTERING` bit **or** a concurrent thread is in the
// `NOTIFYING` critical section. Either way, no action must be taken.
//
// If the current thread is the only concurrent call to `notify` and another
// thread is in the `register` critical section, when the other thread **exits**
// the `register` critical section, it will observe the `NOTIFYING` bit and
// handle the notify itself.
//
// If another thread is in the `notify` critical section, then it will handle
// notifying the task.
//
// # A potential race (is safely handled).
//
// Imagine the following situation:
//
// * Thread A obtains the `notify` lock and notifies a task.
//
// * Before thread A releases the `notify` lock, the notified task is scheduled.
//
// * Thread B attempts to notify the task. In theory this should result in the
//   task being notified, but it cannot because thread A still holds the notify
//   lock.
//
// This case is handled by requiring users of `AtomicTask` to call `register`
// **before** attempting to observe the application state change that resulted
// in the task being notified. The notifiers also change the application state
// before calling notify.
//
// Because of this, the task will do one of two things.
//
// 1) Observe the application state change that Thread B is notifying on. In
//    this case, it is OK for Thread B's notification to be lost.
//
// 2) Call register before attempting to observe the application state. Since
//    Thread A still holds the `notify` lock, the call to `register` will result
//    in the task notifying itself and get scheduled again.

/// Idle state
const WAITING: usize = 0;

/// A new task value is being registered with the `AtomicTask` cell.
const REGISTERING: usize = 0b01;

/// The task currently registered with the `AtomicTask` cell is being notified.
const NOTIFYING: usize = 0b10;

impl AtomicTask {
    /// Create an `AtomicTask` initialized with the given `Task`
    pub fn new() -> AtomicTask {
        // Make sure that task is Sync
        trait AssertSync: Sync {}
        impl AssertSync for Task {}

        AtomicTask {
            state: AtomicUsize::new(WAITING),
            task: UnsafeCell::new(None),
        }
    }

    /// Registers the current task to be notified on calls to `notify`.
    ///
    /// This is the same as calling `register_task` with `task::current()`.
    pub fn register(&self) {
        self.register_task(super::current());
    }

    /// Registers the provided task to be notified on calls to `notify`.
    ///
    /// The new task will take place of any previous tasks that were registered
    /// by previous calls to `register`. Any calls to `notify` that happen after
    /// a call to `register` (as defined by the memory ordering rules), will
    /// notify the `register` caller's task.
    ///
    /// It is safe to call `register` with multiple other threads concurrently
    /// calling `notify`. This will result in the `register` caller's current
    /// task being notified once.
    ///
    /// This function is safe to call concurrently, but this is generally a bad
    /// idea. Concurrent calls to `register` will attempt to register different
    /// tasks to be notified. One of the callers will win and have its task set,
    /// but there is no guarantee as to which caller will succeed.
    pub fn register_task(&self, task: Task) {
        match self.state.compare_and_swap(WAITING, REGISTERING, Acquire) {
            WAITING => {
                unsafe {
                    // Locked acquired, update the waker cell
                    *self.task.get() = Some(task.clone());

                    // Release the lock. If the state transitioned to include
                    // the `NOTIFYING` bit, this means that a notify has been
                    // called concurrently, so we have to remove the task and
                    // notify it.`
                    //
                    // Start by assuming that the state is `REGISTERING` as this
                    // is what we jut set it to.
                    let res = self.state.compare_exchange(
                        REGISTERING, WAITING, AcqRel, Acquire);

                    match res {
                        Ok(_) => {}
                        Err(actual) => {
                            // This branch can only be reached if a
                            // concurrent thread called `notify`. In this
                            // case, `actual` **must** be `REGISTERING |
                            // `NOTIFYING`.
                            debug_assert_eq!(actual, REGISTERING | NOTIFYING);

                            // Take the task to notify once the atomic operation has
                            // completed.
                            let notify = (*self.task.get()).take().unwrap();

                            // Just swap, because no one could change state
                            // while state == `Registering | `Waking`
                            self.state.swap(WAITING, AcqRel);

                            // The atomic swap was complete, now
                            // notify the task and return.
                            notify.notify();
                        }
                    }
                }
            }
            NOTIFYING => {
                // Currently in the process of notifying the task, i.e.,
                // `notify` is currently being called on the old task handle.
                // So, we call notify on the new task handle
                task.notify();
            }
            state => {
                // In this case, a concurrent thread is holding the
                // "registering" lock. This probably indicates a bug in the
                // caller's code as racing to call `register` doesn't make much
                // sense.
                //
                // We just want to maintain memory safety. It is ok to drop the
                // call to `register`.
                debug_assert!(
                    state == REGISTERING ||
                    state == REGISTERING | NOTIFYING);
            }
        }
    }

    /// Notifies the task that last called `register`.
    ///
    /// If `register` has not been called yet, then this does nothing.
    pub fn notify(&self) {
        // AcqRel ordering is used in order to acquire the value of the `task`
        // cell as well as to establish a `release` ordering with whatever
        // memory the `AtomicTask` is associated with.
        match self.state.fetch_or(NOTIFYING, AcqRel) {
            WAITING => {
                // The notifying lock has been acquired.
                let task = unsafe { (*self.task.get()).take() };

                // Release the lock
                self.state.fetch_and(!NOTIFYING, Release);

                if let Some(task) = task {
                    task.notify();
                }
            }
            state => {
                // There is a concurrent thread currently updating the
                // associated task.
                //
                // Nothing more to do as the `NOTIFYING` bit has been set. It
                // doesn't matter if there are concurrent registering threads or
                // not.
                //
                debug_assert!(
                    state == REGISTERING ||
                    state == REGISTERING | NOTIFYING ||
                    state == NOTIFYING);
            }
        }
    }
}

impl Default for AtomicTask {
    fn default() -> Self {
        AtomicTask::new()
    }
}

impl fmt::Debug for AtomicTask {
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        write!(fmt, "AtomicTask")
    }
}

unsafe impl Send for AtomicTask {}
unsafe impl Sync for AtomicTask {}
