use std::fmt;
use std::pin::Pin;
use std::time::Duration;

use super::MutexGuard;
use crate::future::{timeout, Future};
use crate::sync::WakerSet;
use crate::task::{Context, Poll};

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
pub struct WaitTimeoutResult(bool);

/// A type indicating whether a timed wait on a condition variable returned due to a time out or
/// not
impl WaitTimeoutResult {
    /// Returns `true` if the wait was known to have timed out.
    pub fn timed_out(self) -> bool {
        self.0
    }
}

/// A Condition Variable
///
/// This type is an async version of [`std::sync::Condvar`].
///
/// [`std::sync::Condvar`]: https://doc.rust-lang.org/std/sync/struct.Condvar.html
///
/// # Examples
///
/// ```
/// # async_std::task::block_on(async {
/// #
/// use std::sync::Arc;
///
/// use async_std::sync::{Mutex, Condvar};
/// use async_std::task;
///
/// let pair = Arc::new((Mutex::new(false), Condvar::new()));
/// let pair2 = pair.clone();
///
/// // Inside of our lock, spawn a new thread, and then wait for it to start.
/// task::spawn(async move {
///     let (lock, cvar) = &*pair2;
///     let mut started = lock.lock().await;
///     *started = true;
///     // We notify the condvar that the value has changed.
///     cvar.notify_one();
/// });
///
/// // Wait for the thread to start up.
/// let (lock, cvar) = &*pair;
/// let mut started = lock.lock().await;
/// while !*started {
///     started = cvar.wait(started).await;
/// }
///
/// # })
/// ```
pub struct Condvar {
    wakers: WakerSet,
}

unsafe impl Send for Condvar {}
unsafe impl Sync for Condvar {}

impl Default for Condvar {
    fn default() -> Self {
        Condvar::new()
    }
}

impl Condvar {
    /// Creates a new condition variable
    ///
    /// # Examples
    ///
    /// ```
    /// use async_std::sync::Condvar;
    ///
    /// let cvar = Condvar::new();
    /// ```
    pub fn new() -> Self {
        Condvar {
            wakers: WakerSet::new(),
        }
    }

    /// Blocks the current task until this condition variable receives a notification.
    ///
    /// Unlike the std equivalent, this does not check that a single mutex is used at runtime.
    /// However, as a best practice avoid using with multiple mutexes.
    ///
    /// # Examples
    ///
    /// ```
    /// # async_std::task::block_on(async {
    /// use std::sync::Arc;
    ///
    /// use async_std::sync::{Mutex, Condvar};
    /// use async_std::task;
    ///
    /// let pair = Arc::new((Mutex::new(false), Condvar::new()));
    /// let pair2 = pair.clone();
    ///
    /// task::spawn(async move {
    ///     let (lock, cvar) = &*pair2;
    ///     let mut started = lock.lock().await;
    ///     *started = true;
    ///     // We notify the condvar that the value has changed.
    ///     cvar.notify_one();
    /// });
    ///
    /// // Wait for the thread to start up.
    /// let (lock, cvar) = &*pair;
    /// let mut started = lock.lock().await;
    /// while !*started {
    ///     started = cvar.wait(started).await;
    /// }
    /// # })
    /// ```
    #[allow(clippy::needless_lifetimes)]
    pub async fn wait<'a, T>(&self, guard: MutexGuard<'a, T>) -> MutexGuard<'a, T> {
        let mutex = MutexGuard::source(&guard);

        self.await_notify(guard).await;

        mutex.lock().await
    }

    fn await_notify<'a, T>(&self, guard: MutexGuard<'a, T>) -> AwaitNotify<'_, 'a, T> {
        AwaitNotify {
            cond: self,
            guard: Some(guard),
            key: None,
        }
    }

    /// Blocks the current task until this condition variable receives a notification and the
    /// required condition is met. Spurious wakeups are ignored and this function will only
    /// return once the condition has been met.
    ///
    /// # Examples
    ///
    /// ```
    /// # async_std::task::block_on(async {
    /// #
    /// use std::sync::Arc;
    ///
    /// use async_std::sync::{Mutex, Condvar};
    /// use async_std::task;
    ///
    /// let pair = Arc::new((Mutex::new(false), Condvar::new()));
    /// let pair2 = pair.clone();
    ///
    /// task::spawn(async move {
    ///     let (lock, cvar) = &*pair2;
    ///     let mut started = lock.lock().await;
    ///     *started = true;
    ///     // We notify the condvar that the value has changed.
    ///     cvar.notify_one();
    /// });
    ///
    /// // Wait for the thread to start up.
    /// let (lock, cvar) = &*pair;
    /// // As long as the value inside the `Mutex<bool>` is `false`, we wait.
    /// let _guard = cvar.wait_until(lock.lock().await, |started| { *started }).await;
    /// #
    /// # })
    /// ```
    #[allow(clippy::needless_lifetimes)]
    pub async fn wait_until<'a, T, F>(
        &self,
        mut guard: MutexGuard<'a, T>,
        mut condition: F,
    ) -> MutexGuard<'a, T>
    where
        F: FnMut(&mut T) -> bool,
    {
        while !condition(&mut *guard) {
            guard = self.wait(guard).await;
        }
        guard
    }

    /// Waits on this condition variable for a notification, timing out after a specified duration.
    ///
    /// For these reasons `Condvar::wait_timeout_until` is recommended in most cases.
    ///
    /// # Examples
    ///
    /// ```
    /// # async_std::task::block_on(async {
    /// #
    /// use std::sync::Arc;
    /// use std::time::Duration;
    ///
    /// use async_std::sync::{Mutex, Condvar};
    /// use async_std::task;
    ///
    /// let pair = Arc::new((Mutex::new(false), Condvar::new()));
    /// let pair2 = pair.clone();
    ///
    /// task::spawn(async move {
    ///   let (lock, cvar) = &*pair2;
    ///   let mut started = lock.lock().await;
    ///   *started = true;
    ///   // We notify the condvar that the value has changed.
    ///   cvar.notify_one();
    /// });
    ///
    /// // wait for the thread to start up
    /// let (lock, cvar) = &*pair;
    /// let mut started = lock.lock().await;
    /// loop {
    ///   let result = cvar.wait_timeout(started, Duration::from_millis(10)).await;
    ///   started = result.0;
    ///   if *started == true {
    ///       // We received the notification and the value has been updated, we can leave.
    ///       break
    ///   }
    /// }
    /// #
    /// # })
    /// ```
    #[allow(clippy::needless_lifetimes)]
    pub async fn wait_timeout<'a, T>(
        &self,
        guard: MutexGuard<'a, T>,
        dur: Duration,
    ) -> (MutexGuard<'a, T>, WaitTimeoutResult) {
        let mutex = MutexGuard::source(&guard);
        match timeout(dur, self.wait(guard)).await {
            Ok(guard) => (guard, WaitTimeoutResult(false)),
            Err(_) => (mutex.lock().await, WaitTimeoutResult(true)),
        }
    }

    /// Waits on this condition variable for a notification, timing out after a specified duration.
    /// Spurious wakes will not cause this function to return.
    ///
    /// # Examples
    /// ```
    /// # async_std::task::block_on(async {
    /// use std::sync::Arc;
    /// use std::time::Duration;
    ///
    /// use async_std::sync::{Mutex, Condvar};
    /// use async_std::task;
    ///
    /// let pair = Arc::new((Mutex::new(false), Condvar::new()));
    /// let pair2 = pair.clone();
    ///
    /// task::spawn(async move {
    ///     let (lock, cvar) = &*pair2;
    ///     let mut started = lock.lock().await;
    ///     *started = true;
    ///     // We notify the condvar that the value has changed.
    ///     cvar.notify_one();
    /// });
    ///
    /// // wait for the thread to start up
    /// let (lock, cvar) = &*pair;
    /// let result = cvar.wait_timeout_until(
    ///     lock.lock().await,
    ///     Duration::from_millis(100),
    ///     |&mut started| started,
    /// ).await;
    /// if result.1.timed_out() {
    ///     // timed-out without the condition ever evaluating to true.
    /// }
    /// // access the locked mutex via result.0
    /// # });
    /// ```
    #[allow(clippy::needless_lifetimes)]
    pub async fn wait_timeout_until<'a, T, F>(
        &self,
        guard: MutexGuard<'a, T>,
        dur: Duration,
        condition: F,
    ) -> (MutexGuard<'a, T>, WaitTimeoutResult)
    where
        F: FnMut(&mut T) -> bool,
    {
        let mutex = MutexGuard::source(&guard);
        match timeout(dur, self.wait_until(guard, condition)).await {
            Ok(guard) => (guard, WaitTimeoutResult(false)),
            Err(_) => (mutex.lock().await, WaitTimeoutResult(true)),
        }
    }

    /// Wakes up one blocked task on this condvar.
    ///
    /// # Examples
    ///
    /// ```
    /// # fn main() { async_std::task::block_on(async {
    /// use std::sync::Arc;
    ///
    /// use async_std::sync::{Mutex, Condvar};
    /// use async_std::task;
    ///
    /// let pair = Arc::new((Mutex::new(false), Condvar::new()));
    /// let pair2 = pair.clone();
    ///
    /// task::spawn(async move {
    ///     let (lock, cvar) = &*pair2;
    ///     let mut started = lock.lock().await;
    ///     *started = true;
    ///     // We notify the condvar that the value has changed.
    ///     cvar.notify_one();
    /// });
    ///
    /// // Wait for the thread to start up.
    /// let (lock, cvar) = &*pair;
    /// let mut started = lock.lock().await;
    /// while !*started {
    ///     started = cvar.wait(started).await;
    /// }
    /// # }) }
    /// ```
    pub fn notify_one(&self) {
        self.wakers.notify_one();
    }

    /// Wakes up all blocked tasks on this condvar.
    ///
    /// # Examples
    /// ```
    /// # fn main() { async_std::task::block_on(async {
    /// #
    /// use std::sync::Arc;
    ///
    /// use async_std::sync::{Mutex, Condvar};
    /// use async_std::task;
    ///
    /// let pair = Arc::new((Mutex::new(false), Condvar::new()));
    /// let pair2 = pair.clone();
    ///
    /// task::spawn(async move {
    ///     let (lock, cvar) = &*pair2;
    ///     let mut started = lock.lock().await;
    ///     *started = true;
    ///     // We notify the condvar that the value has changed.
    ///     cvar.notify_all();
    /// });
    ///
    /// // Wait for the thread to start up.
    /// let (lock, cvar) = &*pair;
    /// let mut started = lock.lock().await;
    /// // As long as the value inside the `Mutex<bool>` is `false`, we wait.
    /// while !*started {
    ///     started = cvar.wait(started).await;
    /// }
    /// #
    /// # }) }
    /// ```
    pub fn notify_all(&self) {
        self.wakers.notify_all();
    }
}

impl fmt::Debug for Condvar {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.pad("Condvar { .. }")
    }
}

/// A future that waits for another task to notify the condition variable.
///
/// This is an internal future that `wait` and `wait_until` await on.
struct AwaitNotify<'a, 'b, T> {
    /// The condition variable that we are waiting on
    cond: &'a Condvar,
    /// The lock used with `cond`.
    /// This will be released the first time the future is polled,
    /// after registering the context to be notified.
    guard: Option<MutexGuard<'b, T>>,
    /// A key into the conditions variable's `WakerSet`.
    /// This is set to the index of the `Waker` for the context each time
    /// the future is polled and not completed.
    key: Option<usize>,
}

impl<'a, 'b, T> Future for AwaitNotify<'a, 'b, T> {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.guard.take() {
            Some(_) => {
                self.key = Some(self.cond.wakers.insert(cx));
                // the guard is dropped when we return, which frees the lock
                Poll::Pending
            }
            None => {
                if let Some(key) = self.key {
                    if self.cond.wakers.remove_if_notified(key, cx) {
                        self.key = None;
                        Poll::Ready(())
                    } else {
                        Poll::Pending
                    }
                } else {
                    // This should only happen if it is polled twice after receiving a notification
                    Poll::Ready(())
                }
            }
        }
    }
}

impl<'a, 'b, T> Drop for AwaitNotify<'a, 'b, T> {
    fn drop(&mut self) {
        if let Some(key) = self.key {
            self.cond.wakers.cancel(key);
        }
    }
}
