//! A dummy implementation of fibers when running with MIRI to use a separate
//! thread as the implementation of a fiber.
//!
//! Note that this technically isn't correct because it means that the code
//! running in the fiber won't share TLS variables with the code managing the
//! fiber, but it's enough for now.
//!
//! The general idea is that a thread is held in a suspended state to hold the
//! state of the stack on that thread. When a fiber is resumed that thread
//! starts executing and the caller stops. When a fiber suspends then that
//! thread stops and the original caller returns. There's still possible minor
//! amounts of parallelism but in general they should be quite scoped and not
//! visible from the caller/callee really.
//!
//! An issue was opened at rust-lang/miri#4392 for a possible extension to miri
//! to support stack-switching in a first-class manner.

use crate::{Result, RunResult, RuntimeFiberStack};
use std::boxed::Box;
use std::cell::Cell;
use std::io;
use std::mem;
use std::ops::Range;
use std::sync::{Arc, Condvar, Mutex};
use std::thread::{self, JoinHandle};

pub type Error = io::Error;

pub struct FiberStack(usize);

impl FiberStack {
    pub fn new(size: usize, _zeroed: bool) -> Result<Self> {
        Ok(FiberStack(size))
    }

    pub unsafe fn from_raw_parts(_base: *mut u8, _guard_size: usize, _len: usize) -> Result<Self> {
        Err(io::ErrorKind::Unsupported.into())
    }

    pub fn is_from_raw_parts(&self) -> bool {
        false
    }

    pub fn from_custom(_custom: Box<dyn RuntimeFiberStack>) -> Result<Self> {
        Err(io::ErrorKind::Unsupported.into())
    }

    pub fn top(&self) -> Option<*mut u8> {
        None
    }

    pub fn range(&self) -> Option<Range<usize>> {
        None
    }

    pub fn guard_range(&self) -> Option<Range<*mut u8>> {
        None
    }
}

pub struct Fiber {
    state: *const u8,
    thread: Option<JoinHandle<()>>,
}

pub struct Suspend {
    state: *const u8,
}

/// Shared state, inside an `Arc`, between `Fiber` and `Suspend`.
struct SharedFiberState<A, B, C> {
    cond: Condvar,
    state: Mutex<State<A, B, C>>,
}

enum State<A, B, C> {
    /// No current state, or otherwise something is waiting for something else
    /// to happen.
    None,

    /// The fiber is being resumed with this result.
    ResumeWith(RunResult<A, B, C>),

    /// The fiber is being suspended with this result
    SuspendWith(RunResult<A, B, C>),

    /// The fiber needs to exit (part of drop).
    Exiting,
}

unsafe impl<A, B, C> Send for State<A, B, C> {}
unsafe impl<A, B, C> Sync for State<A, B, C> {}

struct IgnoreSendSync<T>(T);

unsafe impl<T> Send for IgnoreSendSync<T> {}
unsafe impl<T> Sync for IgnoreSendSync<T> {}

fn run<F, A, B, C>(state: Arc<SharedFiberState<A, B, C>>, func: IgnoreSendSync<F>)
where
    F: FnOnce(A, &mut super::Suspend<A, B, C>) -> C,
{
    // Wait for the initial message of what to initially invoke `func` with.
    let init = {
        let mut lock = state.state.lock().unwrap();
        lock = state
            .cond
            .wait_while(lock, |msg| !matches!(msg, State::ResumeWith(_)))
            .unwrap();
        match mem::replace(&mut *lock, State::None) {
            State::ResumeWith(RunResult::Resuming(init)) => init,
            _ => unreachable!(),
        }
    };

    // Execute this fiber through `Suspend::execute` and once that's done
    // deallocate the `state` that we have.
    let state = Arc::into_raw(state);
    super::Suspend::<A, B, C>::execute(
        Suspend {
            state: state.cast(),
        },
        init,
        func.0,
    );
    unsafe {
        drop(Arc::from_raw(state));
    }
}

impl Fiber {
    pub fn new<F, A, B, C>(stack: &FiberStack, func: F) -> Result<Self>
    where
        F: FnOnce(A, &mut super::Suspend<A, B, C>) -> C,
    {
        // Allocate shared state between the fiber and the suspension argument.
        let state = Arc::new(SharedFiberState::<A, B, C> {
            cond: Condvar::new(),
            state: Mutex::new(State::None),
        });

        // Note the use of `spawn_unchecked` to work around `Send`. Technically
        // a lie as we are sure enough sending values across threads. We don't
        // have many other tools in MIRI though to allocate separate call stacks
        // so we're doing the best we can.
        let thread = unsafe {
            thread::Builder::new()
                .stack_size(stack.0)
                .spawn_unchecked({
                    let state = state.clone();
                    let func = IgnoreSendSync(func);
                    move || run(state, func)
                })
                .unwrap()
        };

        // Cast the fiber back into a raw pointer to lose the type parameters
        // which our storage container does not have access to. Additionally
        // save off the thread so the dtor here can join the thread.
        Ok(Fiber {
            state: Arc::into_raw(state).cast(),
            thread: Some(thread),
        })
    }

    pub(crate) fn resume<A, B, C>(&self, _stack: &FiberStack, result: &Cell<RunResult<A, B, C>>) {
        let my_state = unsafe { self.state() };
        let mut lock = my_state.state.lock().unwrap();

        // Swap `result` into our `lock`, then wake up the actual fiber.
        *lock = State::ResumeWith(result.replace(RunResult::Executing));
        my_state.cond.notify_one();

        // Wait for the fiber to finish
        lock = my_state
            .cond
            .wait_while(lock, |l| !matches!(l, State::SuspendWith(_)))
            .unwrap();

        // Swap the state in our `lock` back into `result`.
        let message = match mem::replace(&mut *lock, State::None) {
            State::SuspendWith(msg) => msg,
            _ => unreachable!(),
        };
        result.set(message);
    }

    unsafe fn state<A, B, C>(&self) -> &SharedFiberState<A, B, C> {
        unsafe { &*(self.state as *const SharedFiberState<A, B, C>) }
    }

    pub(crate) unsafe fn drop<A, B, C>(&mut self) {
        let state = unsafe { self.state::<A, B, C>() };

        // Store an indication that we expect the fiber to exit, then wake it up
        // if it's waiting.
        *state.state.lock().unwrap() = State::Exiting;
        state.cond.notify_one();

        // Wait for the child thread to complete.
        self.thread.take().unwrap().join().unwrap();

        // Clean up our state using the type parameters we know of here.
        unsafe {
            drop(Arc::from_raw(
                self.state.cast::<SharedFiberState<A, B, C>>(),
            ));
        }
    }
}

impl Suspend {
    fn suspend<A, B, C>(&mut self, result: RunResult<A, B, C>) -> State<A, B, C> {
        let state = unsafe { self.state() };
        let mut lock = state.state.lock().unwrap();

        // Our fiber state should be empty, and after verifying that store what
        // we are suspending with.
        assert!(matches!(*lock, State::None));
        *lock = State::SuspendWith(result);
        state.cond.notify_one();

        // Wait for the resumption to come back, which is returned from this
        // method.
        lock = state
            .cond
            .wait_while(lock, |s| {
                !matches!(s, State::ResumeWith(_) | State::Exiting)
            })
            .unwrap();
        mem::replace(&mut *lock, State::None)
    }

    pub(crate) fn switch<A, B, C>(&mut self, result: RunResult<A, B, C>) -> A {
        match self.suspend(result) {
            State::ResumeWith(RunResult::Resuming(a)) => a,
            _ => unreachable!(),
        }
    }

    pub(crate) fn exit<A, B, C>(&mut self, result: RunResult<A, B, C>) {
        match self.suspend(result) {
            State::Exiting => {}
            _ => unreachable!(),
        }
    }

    unsafe fn state<A, B, C>(&self) -> &SharedFiberState<A, B, C> {
        unsafe { &*(self.state as *const SharedFiberState<A, B, C>) }
    }
}
