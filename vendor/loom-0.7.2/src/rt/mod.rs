#[macro_use]
mod location;
pub(crate) use self::location::Location;

mod access;
use self::access::Access;

mod alloc;
pub(crate) use self::alloc::{alloc, dealloc, Allocation};

mod arc;
pub(crate) use self::arc::Arc;

mod atomic;
pub(crate) use self::atomic::{fence, Atomic};

pub(crate) mod cell;
pub(crate) use self::cell::Cell;

mod condvar;
pub(crate) use self::condvar::Condvar;

mod execution;
pub(crate) use self::execution::Execution;

mod notify;
pub(crate) use self::notify::Notify;

mod num;
pub(crate) use self::num::Numeric;

#[macro_use]
pub(crate) mod object;

mod mpsc;
pub(crate) use self::mpsc::Channel;

mod mutex;
pub(crate) use self::mutex::Mutex;

mod path;
pub(crate) use self::path::Path;

mod rwlock;
pub(crate) use self::rwlock::RwLock;

mod scheduler;
pub(crate) use self::scheduler::Scheduler;

mod synchronize;
pub(crate) use self::synchronize::Synchronize;

pub(crate) mod lazy_static;
pub(crate) mod thread;

mod vv;
pub(crate) use self::vv::VersionVec;

use tracing::trace;

/// Maximum number of threads that can be included in a model.
pub const MAX_THREADS: usize = 5;

/// Maximum number of atomic store history to track per-cell.
pub(crate) const MAX_ATOMIC_HISTORY: usize = 7;

pub(crate) fn spawn<F>(stack_size: Option<usize>, f: F) -> crate::rt::thread::Id
where
    F: FnOnce() + 'static,
{
    let id = execution(|execution| execution.new_thread());

    trace!(thread = ?id, "spawn");

    Scheduler::spawn(
        stack_size,
        Box::new(move || {
            f();
            thread_done();
        }),
    );

    id
}

/// Marks the current thread as blocked
pub(crate) fn park(location: Location) {
    let switch = execution(|execution| {
        use thread::State;
        let thread = execution.threads.active_id();
        let active = execution.threads.active_mut();

        trace!(?thread, ?active.state, "park");

        match active.state {
            // The thread was previously unparked while it was active. Instead
            // of parking, consume the unpark.
            State::Runnable { unparked: true } => {
                active.set_runnable();
                return false;
            }
            // The thread doesn't have a saved unpark; set its state to blocked.
            _ => active.set_blocked(location),
        };

        execution.threads.active_mut().set_blocked(location);
        execution.threads.active_mut().operation = None;
        execution.schedule()
    });

    if switch {
        Scheduler::switch();
    }
}

/// Add an execution branch point.
fn branch<F, R>(f: F) -> R
where
    F: FnOnce(&mut Execution) -> R,
{
    let (ret, switch) = execution(|execution| {
        let ret = f(execution);
        let switch = execution.schedule();

        trace!(?switch, "branch");

        (ret, switch)
    });

    if switch {
        Scheduler::switch();
    }

    ret
}

fn synchronize<F, R>(f: F) -> R
where
    F: FnOnce(&mut Execution) -> R,
{
    execution(|execution| {
        execution.threads.active_causality_inc();
        trace!("synchronize");
        f(execution)
    })
}

/// Yield the thread.
///
/// This enables concurrent algorithms that require other threads to make
/// progress.
pub fn yield_now() {
    let switch = execution(|execution| {
        let thread = execution.threads.active_id();

        execution.threads.active_mut().set_yield();
        execution.threads.active_mut().operation = None;
        let switch = execution.schedule();

        trace!(?thread, ?switch, "yield_now");

        switch
    });

    if switch {
        Scheduler::switch();
    }
}

pub(crate) fn execution<F, R>(f: F) -> R
where
    F: FnOnce(&mut Execution) -> R,
{
    Scheduler::with_execution(f)
}

pub fn thread_done() {
    let locals = execution(|execution| {
        let thread = execution.threads.active_id();

        trace!(?thread, "thread_done: drop locals");

        execution.threads.active_mut().drop_locals()
    });

    // Drop outside of the execution context
    drop(locals);

    execution(|execution| {
        let thread = execution.threads.active_id();

        execution.threads.active_mut().operation = None;
        execution.threads.active_mut().set_terminated();
        let switch = execution.schedule();
        trace!(?thread, ?switch, "thread_done: terminate");
    });
}

/// Tells loom to explore possible concurrent executions starting at this point.
pub fn explore() {
    execution(|execution| {
        execution.path.explore_state();
    })
}

/// Tells loom to stop exploring possible concurrent executions starting at this
/// point.
///
/// Exploration can be enabled again with `explore`.
pub fn stop_exploring() {
    execution(|execution| {
        execution.path.critical();
    })
}

/// Tells loom to stop exploring possible concurrent execution starting at this
/// point.
///
/// Unlike `stop_exploring`, exploration cannot be restarted by `explore`.
pub fn skip_branch() {
    execution(|execution| execution.path.skip_branch())
}
