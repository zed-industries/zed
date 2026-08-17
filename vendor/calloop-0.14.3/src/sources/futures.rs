//! A futures executor as an event source
//!
//! Only available with the `executor` cargo feature of `calloop`.
//!
//! This executor is intended for light futures, which will be polled as part of your
//! event loop. Such futures may be waiting for IO, or for some external computation on an
//! other thread for example.
//!
//! You can create a new executor using the `executor` function, which creates a pair
//! `(Executor<T>, Scheduler<T>)` to handle futures that all evaluate to type `T`. The
//! executor should be inserted into your event loop, and will yield the return values of
//! the futures as they finish into your callback. The scheduler can be cloned and used
//! to send futures to be executed into the executor. A generic executor can be obtained
//! by choosing `T = ()` and letting futures handle the forwarding of their return values
//! (if any) by their own means.
//!
//! **Note:** The futures must have their own means of being woken up, as this executor is,
//! by itself, not I/O aware. See [`LoopHandle::adapt_io`](crate::LoopHandle#method.adapt_io)
//! for that, or you can use some other mechanism if you prefer.

use async_task::{Builder, Runnable};
use slab::Slab;
use std::{
    cell::RefCell,
    fmt,
    future::Future,
    rc::Rc,
    sync::{
        atomic::{AtomicBool, Ordering},
        mpsc, Arc, Mutex,
    },
    task::Waker,
};

use crate::{
    sources::{
        channel::ChannelError,
        ping::{make_ping, Ping, PingError, PingSource},
        EventSource,
    },
    Poll, PostAction, Readiness, Token, TokenFactory,
};

/// A future executor as an event source
#[derive(Debug)]
pub struct Executor<T> {
    /// Shared state between the executor and the scheduler.
    state: Rc<State<T>>,

    /// Notifies us when the executor is woken up.
    source: PingSource,

    /// Used for when we need to wake ourselves up.
    ping: Ping,
}

/// A scheduler to send futures to an executor
#[derive(Clone, Debug)]
pub struct Scheduler<T> {
    /// Shared state between the executor and the scheduler.
    state: Rc<State<T>>,
}

/// The inner state of the executor.
#[derive(Debug)]
struct State<T> {
    /// The incoming queue of runnables to be executed.
    incoming: mpsc::Receiver<Runnable<usize>>,

    /// The sender corresponding to `incoming`.
    sender: Arc<Sender>,

    /// The list of currently active tasks.
    ///
    /// This is set to `None` when the executor is destroyed.
    active_tasks: RefCell<Option<Slab<Active<T>>>>,
}

/// Send a future to an executor.
///
/// This needs to be thread-safe, as it is called from a `Waker` that may be on a different thread.
#[derive(Debug)]
struct Sender {
    /// The sender used to send runnables to the executor.
    ///
    /// `mpsc::Sender` is `!Sync`, wrapping it in a `Mutex` makes it `Sync`.
    sender: Mutex<mpsc::Sender<Runnable<usize>>>,

    /// The ping source used to wake up the executor.
    wake_up: Ping,

    /// Whether the executor has already been woken.
    notified: AtomicBool,
}

/// An active future or its result.
#[derive(Debug)]
enum Active<T> {
    /// The future is currently being polled.
    ///
    /// Waking this waker will insert the runnable into `incoming`.
    Future(Waker),

    /// The future has finished polling, and its result is stored here.
    Finished(T),
}

impl<T> Active<T> {
    fn is_finished(&self) -> bool {
        matches!(self, Active::Finished(_))
    }
}

impl<T> Scheduler<T> {
    /// Sends the given future to the executor associated to this scheduler
    ///
    /// Returns an error if the the executor not longer exists.
    pub fn schedule<Fut>(&self, future: Fut) -> Result<(), ExecutorDestroyed>
    where
        Fut: Future<Output = T> + 'static,
        T: 'static,
    {
        /// Store this future's result in the executor.
        struct StoreOnDrop<'a, T> {
            index: usize,
            value: Option<T>,
            state: &'a State<T>,
        }

        impl<T> Drop for StoreOnDrop<'_, T> {
            fn drop(&mut self) {
                let mut active_tasks = self.state.active_tasks.borrow_mut();
                if let Some(active_tasks) = active_tasks.as_mut() {
                    if let Some(value) = self.value.take() {
                        active_tasks[self.index] = Active::Finished(value);
                    } else {
                        // The future was dropped before it finished.
                        // Remove it from the active list.
                        active_tasks.remove(self.index);
                    }
                }
            }
        }

        fn assert_send_and_sync<T: Send + Sync>(_: &T) {}

        let mut active_guard = self.state.active_tasks.borrow_mut();
        let active_tasks = active_guard.as_mut().ok_or(ExecutorDestroyed)?;

        // Wrap the future in another future that polls it and stores the result.
        let index = active_tasks.vacant_key();
        let future = {
            let state = self.state.clone();
            async move {
                let mut guard = StoreOnDrop {
                    index,
                    value: None,
                    state: &state,
                };

                // Get the value of the future.
                let value = future.await;

                // Store it in the executor.
                guard.value = Some(value);
            }
        };

        // A schedule function that inserts the runnable into the incoming queue.
        let schedule = {
            let sender = self.state.sender.clone();
            move |runnable| sender.send(runnable)
        };

        assert_send_and_sync(&schedule);

        // Spawn the future.
        let (runnable, task) = Builder::new()
            .metadata(index)
            .spawn_local(move |_| future, schedule);

        // Insert the runnable into the set of active tasks.
        active_tasks.insert(Active::Future(runnable.waker()));
        drop(active_guard);

        // Schedule the runnable and detach the task so it isn't cancellable.
        runnable.schedule();
        task.detach();

        Ok(())
    }
}

impl Sender {
    /// Send a runnable to the executor.
    fn send(&self, runnable: Runnable<usize>) {
        // Send on the channel.
        //
        // All we do with the lock is call `send`, so there's no chance of any state being corrupted on
        // panic. Therefore it's safe to ignore the mutex poison.
        if let Err(e) = self
            .sender
            .lock()
            .unwrap_or_else(|e| e.into_inner())
            .send(runnable)
        {
            // The runnable must be dropped on its origin thread, since the original future might be
            // !Send. This channel immediately sends it back to the Executor, which is pinned to the
            // origin thread. The executor's Drop implementation will force all of the runnables to be
            // dropped, therefore the channel should always be available. If we can't send the runnable,
            // it indicates that the above behavior is broken and that unsoundness has occurred. The
            // only option at this stage is to forget the runnable and leak the future.

            std::mem::forget(e);
            unreachable!("Attempted to send runnable to a stopped executor");
        }

        // If the executor is already awake, don't bother waking it up again.
        if self.notified.swap(true, Ordering::SeqCst) {
            return;
        }

        // Wake the executor.
        self.wake_up.ping();
    }
}

impl<T> Drop for Executor<T> {
    fn drop(&mut self) {
        let active_tasks = self.state.active_tasks.borrow_mut().take().unwrap();

        // Wake all of the active tasks in order to destroy their runnables.
        for (_, task) in active_tasks {
            if let Active::Future(waker) = task {
                // Don't let a panicking waker blow everything up.
                //
                // There is a chance that a future will panic and, during the unwinding process,
                // drop this executor. However, since the future panicked, there is a possibility
                // that the internal state of the waker will be invalid in such a way that the waker
                // panics as well. Since this would be a panic during a panic, Rust will upgrade it
                // into an abort.
                //
                // In the interest of not aborting without a good reason, we just drop the panic here.
                std::panic::catch_unwind(|| waker.wake()).ok();
            }
        }

        // Drain the queue in order to drop all of the runnables.
        while self.state.incoming.try_recv().is_ok() {}
    }
}

/// Error generated when trying to schedule a future after the
/// executor was destroyed.
#[derive(Debug)]
pub struct ExecutorDestroyed;

impl fmt::Display for ExecutorDestroyed {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.write_str("the executor was destroyed")
    }
}

impl std::error::Error for ExecutorDestroyed {}

/// Create a new executor, and its associated scheduler
///
/// May fail due to OS errors preventing calloop to setup its internal pipes (if your
/// process has reatched its file descriptor limit for example).
pub fn executor<T>() -> crate::Result<(Executor<T>, Scheduler<T>)> {
    let (sender, incoming) = mpsc::channel();
    let (wake_up, ping) = make_ping()?;

    let state = Rc::new(State {
        incoming,
        active_tasks: RefCell::new(Some(Slab::new())),
        sender: Arc::new(Sender {
            sender: Mutex::new(sender),
            wake_up: wake_up.clone(),
            notified: AtomicBool::new(false),
        }),
    });

    Ok((
        Executor {
            state: state.clone(),
            source: ping,
            ping: wake_up,
        },
        Scheduler { state },
    ))
}

impl<T> EventSource for Executor<T> {
    type Event = T;
    type Metadata = ();
    type Ret = ();
    type Error = ExecutorError;

    fn process_events<F>(
        &mut self,
        readiness: Readiness,
        token: Token,
        mut callback: F,
    ) -> Result<PostAction, Self::Error>
    where
        F: FnMut(T, &mut ()),
    {
        let state = &self.state;

        let (clear_readiness, action) = {
            let mut clear_readiness = false;

            let action = self
                .source
                .process_events(readiness, token, |(), &mut ()| {
                    // Set to the unnotified state.
                    state.sender.notified.store(false, Ordering::SeqCst);

                    // Process runnables, but not too many at a time; better to move onto the next event quickly!
                    for _ in 0..1024 {
                        let runnable = match state.incoming.try_recv() {
                            Ok(runnable) => runnable,
                            Err(_) => {
                                // Make sure to clear the readiness if there are no more runnables.
                                clear_readiness = true;
                                break;
                            }
                        };

                        // Run the runnable.
                        let index = *runnable.metadata();
                        runnable.run();

                        // If the runnable finished with a result, call the callback.
                        let mut active_guard = state.active_tasks.borrow_mut();
                        let active_tasks = active_guard.as_mut().unwrap();

                        if let Some(state) = active_tasks.get(index) {
                            if state.is_finished() {
                                // Take out the state and provide it to the caller.
                                let result = match active_tasks.remove(index) {
                                    Active::Finished(result) => result,
                                    _ => unreachable!(),
                                };

                                // Drop the guard since the callback may register another future to the scheduler.
                                drop(active_guard);

                                callback(result, &mut ());
                            }
                        }
                    }
                })
                .map_err(ExecutorError::WakeError)?;

            (clear_readiness, action)
        };

        // Re-ready the ping source if we need to re-run this handler.
        if !clear_readiness {
            self.ping.ping();
            Ok(PostAction::Continue)
        } else {
            Ok(action)
        }
    }

    fn register(&mut self, poll: &mut Poll, token_factory: &mut TokenFactory) -> crate::Result<()> {
        self.source.register(poll, token_factory)?;
        Ok(())
    }

    fn reregister(
        &mut self,
        poll: &mut Poll,
        token_factory: &mut TokenFactory,
    ) -> crate::Result<()> {
        self.source.reregister(poll, token_factory)?;
        Ok(())
    }

    fn unregister(&mut self, poll: &mut Poll) -> crate::Result<()> {
        self.source.unregister(poll)?;
        Ok(())
    }
}

/// An error arising from processing events in an async executor event source.
#[derive(Debug)]
pub enum ExecutorError {
    /// Error while reading new futures added via [`Scheduler::schedule()`].
    NewFutureError(ChannelError),

    /// Error while processing wake events from existing futures.
    WakeError(PingError),
}

impl fmt::Display for ExecutorError {
    #[cfg_attr(feature = "nightly_coverage", coverage(off))]
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::NewFutureError(err) => write!(f, "error adding new futures: {}", err),
            Self::WakeError(err) => write!(f, "error processing wake events: {}", err),
        }
    }
}

impl std::error::Error for ExecutorError {}

#[cfg(test)]
mod tests {
    use super::*;

    use std::cell::RefCell;
    use std::rc::Rc;

    #[test]
    fn ready() {
        let mut event_loop = crate::EventLoop::<u32>::try_new().unwrap();

        let handle = event_loop.handle();

        let (exec, sched) = executor::<u32>().unwrap();

        handle
            .insert_source(exec, move |ret, &mut (), got| {
                *got = ret;
            })
            .unwrap();

        let mut got = 0;

        let fut = async { 42 };

        event_loop
            .dispatch(Some(::std::time::Duration::ZERO), &mut got)
            .unwrap();

        // the future is not yet inserted, and thus has not yet run
        assert_eq!(got, 0);

        sched.schedule(fut).unwrap();

        event_loop
            .dispatch(Some(::std::time::Duration::ZERO), &mut got)
            .unwrap();

        // the future has run
        assert_eq!(got, 42);
    }

    #[test]
    fn more_than_1024() {
        let mut event_loop = crate::EventLoop::<()>::try_new().unwrap();
        let handle = event_loop.handle();

        let (exec, sched) = executor::<()>().unwrap();
        handle.insert_source(exec, move |_, _, _| ()).unwrap();

        let counter = Rc::new(RefCell::new(0));
        for _ in 0..1025 {
            let counter = counter.clone();
            sched
                .schedule(async move {
                    *counter.borrow_mut() += 1;
                })
                .unwrap();
        }

        event_loop
            .dispatch(Some(::std::time::Duration::ZERO), &mut ())
            .unwrap();

        assert_eq!(*counter.borrow(), 1024);

        event_loop
            .dispatch(Some(::std::time::Duration::ZERO), &mut ())
            .unwrap();

        assert_eq!(*counter.borrow(), 1025);
    }
}
