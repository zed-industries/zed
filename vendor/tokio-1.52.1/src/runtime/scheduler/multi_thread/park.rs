//! Parks the runtime.
//!
//! A combination of the various resource driver park handles.

use crate::loom::sync::atomic::AtomicUsize;
use crate::loom::sync::{Arc, Condvar, Mutex};
use crate::runtime::driver::{self, Driver};
use crate::util::TryLock;

use std::sync::atomic::Ordering::SeqCst;
use std::time::{Duration, Instant};

#[cfg(loom)]
use crate::runtime::park::CURRENT_THREAD_PARK_COUNT;

pub(crate) struct Parker {
    inner: Arc<Inner>,
}

pub(crate) struct Unparker {
    inner: Arc<Inner>,
}

/// Represents how a worker thread was parked
#[derive(Copy, Clone, Eq, PartialEq)]
pub(crate) enum HadDriver {
    Yes,
    No,
}

struct Inner {
    /// Avoids entering the park if possible
    state: AtomicUsize,

    /// Used to coordinate access to the driver / `condvar`
    mutex: Mutex<()>,

    /// `Condvar` to block on if the driver is unavailable.
    condvar: Condvar,

    /// Resource (I/O, time, ...) driver
    shared: Arc<Shared>,
}

const EMPTY: usize = 0;
const PARKED_CONDVAR: usize = 1;
const PARKED_DRIVER: usize = 2;
const NOTIFIED: usize = 3;

/// Shared across multiple Parker handles
struct Shared {
    /// Shared driver. Only one thread at a time can use this
    driver: TryLock<Driver>,
}

impl Parker {
    pub(crate) fn new(driver: Driver) -> Parker {
        Parker {
            inner: Arc::new(Inner {
                state: AtomicUsize::new(EMPTY),
                mutex: Mutex::new(()),
                condvar: Condvar::new(),
                shared: Arc::new(Shared {
                    driver: TryLock::new(driver),
                }),
            }),
        }
    }

    pub(crate) fn unpark(&self) -> Unparker {
        Unparker {
            inner: self.inner.clone(),
        }
    }

    pub(crate) fn park(&mut self, handle: &driver::Handle) -> HadDriver {
        self.inner.park(handle)
    }

    /// Parks the current thread for up to `duration`.
    ///
    /// This function tries to acquire the driver lock. If it succeeds, it
    /// parks using the driver. Otherwise, it fails back to using a condvar,
    /// unless the duration is zero, in which case it returns immediately.
    pub(crate) fn park_timeout(
        &mut self,
        handle: &driver::Handle,
        duration: Duration,
    ) -> HadDriver {
        if let Some(mut driver) = self.inner.shared.driver.try_lock() {
            self.inner.park_driver(&mut driver, handle, Some(duration))
        } else if !duration.is_zero() {
            self.inner.park_condvar(Some(duration));
            HadDriver::No
        } else {
            // https://github.com/tokio-rs/tokio/issues/6536
            // Hacky, but it's just for loom tests. The counter gets incremented during
            // `park_timeout`, but we still have to increment the counter if we can't acquire the
            // lock.
            #[cfg(loom)]
            CURRENT_THREAD_PARK_COUNT.with(|count| count.fetch_add(1, SeqCst));
            HadDriver::No
        }
    }

    pub(crate) fn shutdown(&mut self, handle: &driver::Handle) {
        self.inner.shutdown(handle);
    }
}

impl Clone for Parker {
    fn clone(&self) -> Parker {
        Parker {
            inner: Arc::new(Inner {
                state: AtomicUsize::new(EMPTY),
                mutex: Mutex::new(()),
                condvar: Condvar::new(),
                shared: self.inner.shared.clone(),
            }),
        }
    }
}

impl Unparker {
    pub(crate) fn unpark(&self, driver: &driver::Handle) {
        self.inner.unpark(driver);
    }
}

impl Inner {
    /// Parks the current thread for at most `dur`.
    fn park(&self, handle: &driver::Handle) -> HadDriver {
        // If we were previously notified then we consume this notification and
        // return quickly.
        if self
            .state
            .compare_exchange(NOTIFIED, EMPTY, SeqCst, SeqCst)
            .is_ok()
        {
            return HadDriver::No;
        }

        if let Some(mut driver) = self.shared.driver.try_lock() {
            self.park_driver(&mut driver, handle, None)
        } else {
            self.park_condvar(None);
            HadDriver::No
        }
    }

    /// Parks the current thread using a condvar for up to `duration`.
    ///
    /// If `duration` is `None`, parks indefinitely until notified.
    ///
    /// # Panics
    ///
    /// Panics if `duration` is `Some` and the duration is zero.
    fn park_condvar(&self, duration: Option<Duration>) {
        // Otherwise we need to coordinate going to sleep
        let mut m = self.mutex.lock();

        match self
            .state
            .compare_exchange(EMPTY, PARKED_CONDVAR, SeqCst, SeqCst)
        {
            Ok(_) => {}
            Err(NOTIFIED) => {
                // We must read here, even though we know it will be `NOTIFIED`.
                // This is because `unpark` may have been called again since we read
                // `NOTIFIED` in the `compare_exchange` above. We must perform an
                // acquire operation that synchronizes with that `unpark` to observe
                // any writes it made before the call to unpark. To do that we must
                // read from the write it made to `state`.
                let old = self.state.swap(EMPTY, SeqCst);
                debug_assert_eq!(old, NOTIFIED, "park state changed unexpectedly");

                return;
            }
            Err(actual) => panic!("inconsistent park state; actual = {actual}"),
        }

        let timeout_at = duration.map(|d| {
            Instant::now()
                .checked_add(d)
                // best effort to avoid overflow and still provide a usable timeout
                .unwrap_or(Instant::now() + Duration::from_secs(1))
        });

        loop {
            let is_timeout;
            (m, is_timeout) = match timeout_at {
                Some(timeout_at) => {
                    let dur = timeout_at.saturating_duration_since(Instant::now());
                    if !dur.is_zero() {
                        // Ideally, we would use `condvar.wait_timeout_until` here, but it is not available
                        // in `loom`. So we manually compute the timeout.
                        let (m, res) = self.condvar.wait_timeout(m, dur).unwrap();
                        (m, res.timed_out())
                    } else {
                        (m, true)
                    }
                }
                None => (self.condvar.wait(m).unwrap(), false),
            };

            if is_timeout {
                match self.state.swap(EMPTY, SeqCst) {
                    PARKED_CONDVAR => return, // timed out, and no notification received
                    NOTIFIED => return,       // notification and timeout happened concurrently
                    actual @ (PARKED_DRIVER | EMPTY) => {
                        panic!("inconsistent park_timeout state, actual = {actual}")
                    }
                    invalid => panic!("invalid park_timeout state, actual = {invalid}"),
                }
            } else if self
                .state
                .compare_exchange(NOTIFIED, EMPTY, SeqCst, SeqCst)
                .is_ok()
            {
                // got a notification
                return;
            }

            // spurious wakeup, go back to sleep
        }
    }

    fn park_driver(
        &self,
        driver: &mut Driver,
        handle: &driver::Handle,
        duration: Option<Duration>,
    ) -> HadDriver {
        if duration.as_ref().is_some_and(Duration::is_zero) {
            // zero duration doesn't actually park the thread, it just
            // polls the I/O events, timers, etc.
            driver.park_timeout(handle, Duration::ZERO);
            return HadDriver::Yes;
        }

        match self
            .state
            .compare_exchange(EMPTY, PARKED_DRIVER, SeqCst, SeqCst)
        {
            Ok(_) => {}
            Err(NOTIFIED) => {
                // We must read here, even though we know it will be `NOTIFIED`.
                // This is because `unpark` may have been called again since we read
                // `NOTIFIED` in the `compare_exchange` above. We must perform an
                // acquire operation that synchronizes with that `unpark` to observe
                // any writes it made before the call to unpark. To do that we must
                // read from the write it made to `state`.
                let old = self.state.swap(EMPTY, SeqCst);
                debug_assert_eq!(old, NOTIFIED, "park state changed unexpectedly");

                return HadDriver::No;
            }
            Err(actual) => panic!("inconsistent park state; actual = {actual}"),
        }

        if let Some(duration) = duration {
            debug_assert_ne!(duration, Duration::ZERO);
            driver.park_timeout(handle, duration);
        } else {
            driver.park(handle);
        }

        match self.state.swap(EMPTY, SeqCst) {
            NOTIFIED => {}      // got a notification, hurray!
            PARKED_DRIVER => {} // no notification, alas
            n => panic!("inconsistent park_timeout state: {n}"),
        }

        HadDriver::Yes
    }

    fn unpark(&self, driver: &driver::Handle) {
        // To ensure the unparked thread will observe any writes we made before
        // this call, we must perform a release operation that `park` can
        // synchronize with. To do that we must write `NOTIFIED` even if `state`
        // is already `NOTIFIED`. That is why this must be a swap rather than a
        // compare-and-swap that returns if it reads `NOTIFIED` on failure.
        match self.state.swap(NOTIFIED, SeqCst) {
            EMPTY => {}    // no one was waiting
            NOTIFIED => {} // already unparked
            PARKED_CONDVAR => self.unpark_condvar(),
            PARKED_DRIVER => driver.unpark(),
            actual => panic!("inconsistent state in unpark; actual = {actual}"),
        }
    }

    fn unpark_condvar(&self) {
        // There is a period between when the parked thread sets `state` to
        // `PARKED` (or last checked `state` in the case of a spurious wake
        // up) and when it actually waits on `cvar`. If we were to notify
        // during this period it would be ignored and then when the parked
        // thread went to sleep it would never wake up. Fortunately, it has
        // `lock` locked at this stage so we can acquire `lock` to wait until
        // it is ready to receive the notification.
        //
        // Releasing `lock` before the call to `notify_one` means that when the
        // parked thread wakes it doesn't get woken only to have to wait for us
        // to release `lock`.
        drop(self.mutex.lock());

        self.condvar.notify_one();
    }

    fn shutdown(&self, handle: &driver::Handle) {
        if let Some(mut driver) = self.shared.driver.try_lock() {
            driver.shutdown(handle);
        }

        self.condvar.notify_all();
    }
}
