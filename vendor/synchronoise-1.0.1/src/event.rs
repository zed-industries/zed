//! "Event" primitives, allowing one thread to wait on a signal or countdown from other threads.
//!
//! The primary types in this module are the [`CountdownEvent`] and the [`SignalEvent`] structs. See
//! the documentation on those types for further information.
//!
//! [`CountdownEvent`]: struct.CountdownEvent.html
//! [`SignalEvent`]: struct.SignalEvent.html

use std::convert::identity;
use std::sync::atomic::{AtomicBool, AtomicUsize, Ordering};
use std::thread;
use std::time::Duration;

use crossbeam_queue::SegQueue;

/// A synchronization primitive that signals when its count reaches zero.
///
/// With a `CountdownEvent`, it's possible to cause one thread to wait on a set of computations
/// occurring in other threads by making the other threads interact with the counter as they
/// perform their work.
///
/// The main limitation of a CountdownEvent is that once its counter reaches zero (even by starting
/// there), any attempts to update the counter will return `CountdownError::AlreadySet` until the
/// counter is reset by calling `reset` or `reset_to_count`.
///
/// `CountdownEvent` is a port of [System.Threading.CountdownEvent][src-link] from .NET (also
/// called [`CountDownLatch`][java-src] in Java).
///
/// [src-link]: https://msdn.microsoft.com/en-us/library/system.threading.countdownevent(v=vs.110).aspx
/// [java-src]: https://docs.oracle.com/javase/7/docs/api/java/util/concurrent/CountDownLatch.html
///
/// # Example
///
/// This example uses a `CountdownEvent` to make the "coordinator" thread sleep until all of its
/// "worker" threads have finished. Each thread calls `signal.decrement()` to signal to the Event
/// that its work has completed. When the last thread does this (and brings the counter to zero),
/// the "coordinator" thread wakes up and prints `all done!`.
///
/// ```
/// use synchronoise::CountdownEvent;
/// use std::sync::Arc;
/// use std::thread;
/// use std::time::Duration;
///
/// let thread_count = 5;
/// let counter = Arc::new(CountdownEvent::new(thread_count));
///
/// for i in 0..thread_count {
///     let signal = counter.clone();
///     thread::spawn(move || {
///         thread::sleep(Duration::from_secs(i as u64));
///         println!("thread {} activated!", i);
///         signal.decrement().unwrap();
///     });
/// }
///
/// counter.wait();
///
/// println!("all done!");
/// ```
pub struct CountdownEvent {
    initial: usize,
    counter: AtomicUsize,
    waiting: SegQueue<thread::Thread>,
}

/// The collection of errors that can be returned by [`CountdownEvent`] methods.
///
/// See [`CountdownEvent`] for more details.
///
/// [`CountdownEvent`]: struct.CountdownEvent.html
#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum CountdownError {
    /// Returned when adding to a counter would have caused it to overflow.
    SaturatedCounter,
    /// Returned when attempting to signal would have caused the counter to go below zero.
    TooManySignals,
    /// Returned when attempting to modify the counter after it has reached zero.
    AlreadySet,
}

impl CountdownEvent {
    /// Creates a new `CountdownEvent`, initialized to the given count.
    ///
    /// Remember that once the counter reaches zero, calls to `add` or `signal` will fail, so
    /// passing zero to this function will create a `CountdownEvent` that is permanently signaled.
    pub fn new(count: usize) -> CountdownEvent {
        CountdownEvent {
            initial: count,
            counter: AtomicUsize::new(count),
            waiting: SegQueue::new(),
        }
    }

    /// Resets the counter to the count given to `new`.
    ///
    /// This function is safe because the `&mut self` enforces that no other references or locks
    /// exist.
    pub fn reset(&mut self) {
        self.counter = AtomicUsize::new(self.initial);
        // there shouldn't be any remaining thread handles in here, but let's clear it out anyway
        while let Some(thread) = self.waiting.pop() {
            thread.unpark();
        }
    }

    /// Resets the counter to the given count.
    ///
    /// This function is safe because the `&mut self` enforces that no other references or locks
    /// exist.
    pub fn reset_to_count(&mut self, count: usize) {
        self.initial = count;
        self.reset();
    }

    /// Returns the current counter value.
    pub fn count(&self) -> usize {
        self.counter.load(Ordering::SeqCst)
    }

    /// Adds the given count to the counter.
    ///
    /// # Errors
    ///
    /// If the counter is already at zero, this function will return `CountdownError::AlreadySet`.
    ///
    /// If the given count would cause the counter to overflow `usize`, this function will return
    /// `CountdownError::SaturatedCounter`.
    pub fn add(&self, count: usize) -> Result<(), CountdownError> {
        let mut current = self.count();

        loop {
            if current == 0 {
                return Err(CountdownError::AlreadySet);
            }

            if let Some(new_count) = current.checked_add(count) {
                let exchange_result = self.counter.compare_exchange_weak(
                    current,
                    new_count,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                );
                match exchange_result {
                    Ok(_) => return Ok(()),
                    Err(last_count) => current = last_count,
                }
            } else {
                return Err(CountdownError::SaturatedCounter);
            }
        }
    }

    /// Subtracts the given count to the counter, and returns whether this caused any waiting
    /// threads to wake up.
    ///
    /// # Errors
    ///
    /// If the counter is already at zero, this function will return `CountdownError::AlreadySet`.
    ///
    /// If the given count would cause the counter to go *below* zero (instead of reaching zero),
    /// this function will return `CountdownError::TooManySignals`.
    pub fn signal(&self, count: usize) -> Result<bool, CountdownError> {
        let mut current = self.count();

        loop {
            if current == 0 {
                return Err(CountdownError::AlreadySet);
            }

            if let Some(new_count) = current.checked_sub(count) {
                let exchange_result = self.counter.compare_exchange_weak(
                    current,
                    new_count,
                    Ordering::SeqCst,
                    Ordering::SeqCst,
                );
                match exchange_result {
                    Ok(_) => {
                        current = new_count;
                        break;
                    }
                    Err(last_count) => current = last_count,
                }
            } else {
                return Err(CountdownError::TooManySignals);
            }
        }

        if current == 0 {
            while let Some(thread) = self.waiting.pop() {
                thread.unpark();
            }
            Ok(true)
        } else {
            Ok(false)
        }
    }

    /// Adds one to the count.
    ///
    /// # Errors
    ///
    /// See [`add`] for the situations where this function will return an error.
    ///
    /// [`add`]: #method.add
    pub fn increment(&self) -> Result<(), CountdownError> {
        self.add(1)
    }

    /// Subtracts one from the counter, and returns whether this caused any waiting threads to wake
    /// up.
    ///
    /// # Errors
    ///
    /// See [`signal`] for the situations where this function will return an error.
    ///
    /// [`signal`]: #method.signal
    pub fn decrement(&self) -> Result<bool, CountdownError> {
        self.signal(1)
    }

    /// Increments the counter, then returns a guard object that will decrement the counter upon
    /// drop.
    ///
    /// # Errors
    ///
    /// This function will return the same errors as `add`. If the event has already signaled by
    /// the time the guard is dropped (and would cause its `decrement` call to return an error),
    /// then the error will be silently ignored.
    ///
    /// # Example
    ///
    /// Here's the sample from the main docs, using `CountdownGuard`s instead of manually
    /// decrementing:
    ///
    /// ```
    /// use synchronoise::CountdownEvent;
    /// use std::sync::Arc;
    /// use std::thread;
    /// use std::time::Duration;
    ///
    /// let thread_count = 5;
    /// // counter can't start from zero, but the guard increments on its own, so start at one and
    /// // just decrement once when we're ready to wait
    /// let counter = Arc::new(CountdownEvent::new(1));
    ///
    /// for i in 0..thread_count {
    ///     let signal = counter.clone();
    ///     thread::spawn(move || {
    ///         let _guard = signal.guard().unwrap();
    ///         thread::sleep(Duration::from_secs(i));
    ///         println!("thread {} activated!", i);
    ///     });
    /// }
    ///
    /// // give all the threads time to increment the counter before continuing
    /// thread::sleep(Duration::from_millis(100));
    /// counter.decrement().unwrap();
    /// counter.wait();
    ///
    /// println!("all done!");
    /// ```
    pub fn guard(&self) -> Result<CountdownGuard, CountdownError> {
        CountdownGuard::new(self)
    }

    /// Blocks the current thread until the counter reaches zero.
    ///
    /// This function will block indefinitely until the counter reaches zero. It will return
    /// immediately if it is already at zero.
    pub fn wait(&self) {
        // see SignalEvent::wait for why we push first even if the count is already set
        self.waiting.push(thread::current());

        let mut first = true;
        while self.count() > 0 {
            if first {
                first = false;
            } else {
                self.waiting.push(thread::current());
            }

            thread::park();
        }
    }

    /// Blocks the current thread until the timer reaches zero, or until the given timeout elapses,
    /// returning the count at the time of wakeup.
    ///
    /// This function will return immediately if the counter was already at zero. Otherwise, it
    /// will block for roughly no longer than `timeout`, or when the counter reaches zero,
    /// whichever comes first.
    pub fn wait_timeout(&self, timeout: Duration) -> usize {
        use std::time::Instant;

        // see SignalEvent::wait for why we push first even if the count is already set
        self.waiting.push(thread::current());

        let begin = Instant::now();
        let mut first = true;
        let mut remaining = timeout;
        loop {
            let current = self.count();

            if current == 0 {
                return 0;
            }

            if first {
                first = false;
            } else {
                let elapsed = begin.elapsed();
                if elapsed >= timeout {
                    return current;
                } else {
                    remaining = timeout - elapsed;
                }

                self.waiting.push(thread::current());
            }

            thread::park_timeout(remaining);
        }
    }
}

/// An opaque guard struct that decrements the count of a borrowed `CountdownEvent` on drop.
///
/// See [`CountdownEvent::guard`] for more information about this struct.
///
/// [`CountdownEvent::guard`]: struct.CountdownEvent.html#method.guard
pub struct CountdownGuard<'a> {
    event: &'a CountdownEvent,
}

impl<'a> CountdownGuard<'a> {
    fn new(event: &'a CountdownEvent) -> Result<CountdownGuard<'a>, CountdownError> {
        event.increment()?;
        Ok(CountdownGuard { event })
    }
}

/// Upon drop, this guard will decrement the counter of its parent `CountdownEvent`. If this would
/// cause an error (see [`CountdownEvent::signal`] for details), the error is silently ignored.
///
/// [`CountdownEvent::signal`]: struct.CountdownEvent.html#method.signal
impl<'a> Drop for CountdownGuard<'a> {
    fn drop(&mut self) {
        // if decrement() returns an error, then the event has already been signaled somehow. i'm
        // not gonna care about it tho
        self.event.decrement().ok();
    }
}

/// Determines the reset behavior of a [`SignalEvent`].
///
/// See [`SignalEvent`] for more information.
///
/// [`SignalEvent`]: struct.SignalEvent.html
#[derive(Debug, PartialEq, Copy, Clone)]
pub enum SignalKind {
    /// An activated `SignalEvent` automatically resets when a thread is resumed.
    ///
    /// `SignalEvent`s with this kind will only resume one thread at a time.
    Auto,
    /// An activated `SignalEvent` must be manually reset to block threads again.
    ///
    /// `SignalEvent`s with this kind will signal every waiting thread to continue at once.
    Manual,
}

/// A synchronization primitive that allows one or more threads to wait on a signal from another
/// thread.
///
/// With a `SignalEvent`, it's possible to have one or more threads gate on a signal from another
/// thread. The behavior for what happens when an event is signaled depends on the value of the
/// `signal_kind` parameter given to `new`, or whether `auto` or `manual` is used to construct the
/// `SignalEvent`:
///
/// * A value of `SignalKind::Auto` (or a `SignalEvent` created via `SignalEvent::auto()`) will
///   automatically reset the signal when a thread is resumed by this event. If more than one
///   thread is waiting on the event when it is signaled, only one will be resumed.
/// * A value of `SignalKind::Manual` (or a `SignalEvent` created via `SignalEvent::manual()`) will
///   remain signaled until it is manually reset. If more than one thread is waiting on the event
///   when it is signaled, all of them will be resumed. Any other thread that tries to wait on the
///   signal before it is reset will not be blocked at all.
///
/// `SignalEvent` is a port of [System.Threading.EventWaitHandle][src-link] from .NET.
///
/// [src-link]: https://msdn.microsoft.com/en-us/library/system.threading.eventwaithandle(v=vs.110).aspx
///
/// # Example
///
/// The following example uses two `SignalEvent`s:
///
/// * `start_signal` is used as a kind of `std::sync::Barrier`, that keeps all the threads inside
///   the loop from starting until they all have been spawned. All the `start.wait()` calls resume
///   when `start_signal.signal()` is called after the initial loop.
///   * Note that because the "coordinator" doesn't wait for each thread to be scheduled before
///     signaling, it's possible that some later threads may not have had a chance to enter
///     `start.wait()` before the signal is set. In this case they won't block in the first place,
///     and immediately return.
/// * `stop_signal` is used to wake up the "coordinator" thread when each "worker" thread is
///   finished with its work. This allows it to keep a count of the number of threads yet to
///   finish, so it can exit its final loop when all the threads have stopped.
///
/// ```
/// use synchronoise::SignalEvent;
/// use std::sync::Arc;
/// use std::thread;
/// use std::time::Duration;
///
/// let start_signal = Arc::new(SignalEvent::manual(false));
/// let stop_signal = Arc::new(SignalEvent::auto(false));
/// let mut thread_count = 5;
///
/// for i in 0..thread_count {
///     let start = start_signal.clone();
///     let stop = stop_signal.clone();
///     thread::spawn(move || {
///         // as a Manual-reset signal, all the threads will start at the same time
///         start.wait();
///         thread::sleep(Duration::from_secs(i));
///         println!("thread {} activated!", i);
///         stop.signal();
///     });
/// }
///
/// start_signal.signal();
///
/// while thread_count > 0 {
///     // as an Auto-reset signal, this will automatically reset when resuming
///     // so when the loop comes back, we don't have to reset before blocking again
///     stop_signal.wait();
///     thread_count -= 1;
/// }
///
/// println!("all done!");
/// ```
pub struct SignalEvent {
    reset: SignalKind,
    signal: AtomicBool,
    waiting: SegQueue<thread::Thread>,
}

impl SignalEvent {
    /// Creates a new `SignalEvent` with the given starting state and reset behavior.
    ///
    /// If `init_state` is `true`, then this `SignalEvent` will start with the signal already set,
    /// so that threads that wait will immediately unblock.
    pub fn new(init_state: bool, signal_kind: SignalKind) -> SignalEvent {
        SignalEvent {
            reset: signal_kind,
            signal: AtomicBool::new(init_state),
            waiting: SegQueue::new(),
        }
    }

    /// Creates a new automatically-resetting `SignalEvent` with the given starting state.
    ///
    /// If `init_state` is `true`, then this `SignalEvent` will start with the signal already set,
    /// so that the first thread that tries to wait will immediately unblock.
    pub fn auto(init_state: bool) -> SignalEvent {
        SignalEvent::new(init_state, SignalKind::Auto)
    }

    /// Creates a new manually-resetting `SignalEvent` with the given starting state.
    ///
    /// If `init_state` is `true`, then this `SignalEvent` will start with the signal alraedy set,
    /// so that threads that wait will immediately unblock until `reset` is called.
    pub fn manual(init_state: bool) -> SignalEvent {
        SignalEvent::new(init_state, SignalKind::Manual)
    }

    /// Returns the current signal status of the `SignalEvent`.
    pub fn status(&self) -> bool {
        self.signal.load(Ordering::SeqCst)
    }

    /// Sets the signal on this `SignalEvent`, potentially waking up one or all threads waiting on
    /// it.
    ///
    /// If more than one thread is waiting on the event, the behavior is different depending on the
    /// `SignalKind` passed to the event when it was created. For a value of `Auto`, one thread
    /// will be resumed. For a value of `Manual`, all waiting threads will be resumed.
    ///
    /// If no thread is currently waiting on the event, its state will be set regardless. Any
    /// future attempts to wait on the event will unblock immediately, except for a `SignalKind` of
    /// Auto, which will immediately unblock the first thread only.
    pub fn signal(&self) {
        self.signal.store(true, Ordering::SeqCst);

        match self.reset {
            // there may be duplicate handles in the queue due to spurious wakeups, so just loop
            // until we know the signal got reset - any that got woken up wrongly will also observe
            // the reset signal and push their handle back in
            SignalKind::Auto => {
                while self.signal.load(Ordering::SeqCst) {
                    if let Some(thread) = self.waiting.pop() {
                        thread.unpark();
                    } else {
                        break;
                    }
                }
            }
            // for manual resets, just unilaterally drain the queue
            SignalKind::Manual => {
                while let Some(thread) = self.waiting.pop() {
                    thread.unpark();
                }
            }
        }
    }

    /// Resets the signal on this `SignalEvent`, allowing threads that wait on it to block.
    pub fn reset(&self) {
        self.signal.store(false, Ordering::SeqCst);
    }

    /// Blocks this thread until another thread calls `signal`.
    ///
    /// If this event is already set, then this function will immediately return without blocking.
    /// For events with a `SignalKind` of `Auto`, this will reset the signal so that the next
    /// thread to wait will block.
    pub fn wait(&self) {
        // Push first, regardless, because in SignalEvent's doctest there's a thorny race condition
        // where (1) the waiting thread will see an unset signal, (2) the signalling thread will
        // set the signal and drain the queue, and only then (3) the waiting thread will push its
        // handle. Having erroneous handles is ultimately harmless from a correctness standpoint
        // because signal loops properly anyway, and if the park handle is already set when a
        // thread tries to wait it will just immediately unpark, see that the signal is still
        // unset, and park again. Shame about those spent cycles dealing with it though.
        self.waiting.push(thread::current());

        // loop on the park in case we spuriously wake up
        let mut first = true;
        while !self.check_signal() {
            // push every time in case there's a race between `signal` and this, since on
            // `SignalKind::Auto` it will loop until someone turns it off - but only one will
            // actually exit this loop, because `check_signal` does a CAS
            if first {
                first = false;
            } else {
                self.waiting.push(thread::current());
            }

            thread::park();
        }
    }

    /// Blocks this thread until either another thread calls `signal`, or until the timeout
    /// elapses.
    ///
    /// This function returns the status of the signal when it woke up. If this function exits
    /// because the signal was set, and this event has a `SignalKind` of `Auto`, the signal will be
    /// reset so that the next thread to wait will block.
    pub fn wait_timeout(&self, timeout: Duration) -> bool {
        use std::time::Instant;

        // see SignalEvent::wait for why we push first even if the signal is already set
        self.waiting.push(thread::current());

        let begin = Instant::now();
        let mut first = true;
        let mut remaining = timeout;
        loop {
            if self.check_signal() {
                return true;
            }

            if first {
                first = false;
            } else {
                let elapsed = begin.elapsed();
                if elapsed >= timeout {
                    return self.status();
                } else {
                    remaining = timeout - elapsed;
                }

                self.waiting.push(thread::current());
            }

            thread::park_timeout(remaining);
        }
    }

    /// Perfoms an atomic compare-exchange on the signal, resetting it if (1) it was set, and (2)
    /// this `SignalEvent` was configured with `SignalKind::Auto`. Returns whether the signal was
    /// previously set.
    fn check_signal(&self) -> bool {
        self.signal
            .compare_exchange_weak(
                true,
                self.reset == SignalKind::Manual,
                Ordering::SeqCst,
                Ordering::SeqCst,
            )
            .unwrap_or_else(identity)
    }
}
