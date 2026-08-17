//! Definition of the Shared combinator, a future that is cloneable,
//! and can be polled in multiple threads.
//!
//! # Examples
//!
//! ```
//! use futures::future::*;
//!
//! let future = ok::<_, bool>(6);
//! let shared1 = future.shared();
//! let shared2 = shared1.clone();
//! assert_eq!(6, *shared1.wait().unwrap());
//! assert_eq!(6, *shared2.wait().unwrap());
//! ```

use {Future, Poll, Async};
use task::{self, Task};
use executor::{self, Notify, Spawn};

use std::{error, fmt, mem, ops};
use std::cell::UnsafeCell;
use std::sync::{Arc, Mutex};
use std::sync::atomic::AtomicUsize;
use std::sync::atomic::Ordering::SeqCst;
use std::collections::HashMap;

/// A future that is cloneable and can be polled in multiple threads.
/// Use `Future::shared()` method to convert any future into a `Shared` future.
#[must_use = "futures do nothing unless polled"]
pub struct Shared<F: Future> {
    inner: Arc<Inner<F>>,
    waiter: usize,
}

impl<F> fmt::Debug for Shared<F>
    where F: Future + fmt::Debug,
          F::Item: fmt::Debug,
          F::Error: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Shared")
            .field("inner", &self.inner)
            .field("waiter", &self.waiter)
            .finish()
    }
}

struct Inner<F: Future> {
    next_clone_id: AtomicUsize,
    future: UnsafeCell<Option<Spawn<F>>>,
    result: UnsafeCell<Option<Result<SharedItem<F::Item>, SharedError<F::Error>>>>,
    notifier: Arc<Notifier>,
}

struct Notifier {
    state: AtomicUsize,
    waiters: Mutex<HashMap<usize, Task>>,
}

const IDLE: usize = 0;
const POLLING: usize = 1;
const COMPLETE: usize = 2;
const POISONED: usize = 3;

pub fn new<F: Future>(future: F) -> Shared<F> {
    Shared {
        inner: Arc::new(Inner {
            next_clone_id: AtomicUsize::new(1),
            notifier: Arc::new(Notifier {
                state: AtomicUsize::new(IDLE),
                waiters: Mutex::new(HashMap::new()),
            }),
            future: UnsafeCell::new(Some(executor::spawn(future))),
            result: UnsafeCell::new(None),
        }),
        waiter: 0,
    }
}

impl<F> Shared<F> where F: Future {
    // TODO: make this private
    #[deprecated(since = "0.1.12", note = "use `Future::shared` instead")]
    #[cfg(feature = "with-deprecated")]
    #[doc(hidden)]
    pub fn new(future: F) -> Self {
        new(future)
    }

    /// If any clone of this `Shared` has completed execution, returns its result immediately
    /// without blocking. Otherwise, returns None without triggering the work represented by
    /// this `Shared`.
    pub fn peek(&self) -> Option<Result<SharedItem<F::Item>, SharedError<F::Error>>> {
        match self.inner.notifier.state.load(SeqCst) {
            COMPLETE => {
                Some(unsafe { self.clone_result() })
            }
            POISONED => panic!("inner future panicked during poll"),
            _ => None,
        }
    }

    fn set_waiter(&mut self) {
        let mut waiters = self.inner.notifier.waiters.lock().unwrap();
        waiters.insert(self.waiter, task::current());
    }

    unsafe fn clone_result(&self) -> Result<SharedItem<F::Item>, SharedError<F::Error>> {
        match *self.inner.result.get() {
            Some(Ok(ref item)) => Ok(SharedItem { item: item.item.clone() }),
            Some(Err(ref e)) => Err(SharedError { error: e.error.clone() }),
            _ => unreachable!(),
        }
    }

    fn complete(&self) {
        unsafe { *self.inner.future.get() = None };
        self.inner.notifier.state.store(COMPLETE, SeqCst);
        self.inner.notifier.notify(0);
    }
}

impl<F> Future for Shared<F>
    where F: Future
{
    type Item = SharedItem<F::Item>;
    type Error = SharedError<F::Error>;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        self.set_waiter();

        match self.inner.notifier.state.compare_and_swap(IDLE, POLLING, SeqCst) {
            IDLE => {
                // Lock acquired, fall through
            }
            POLLING => {
                // Another task is currently polling, at this point we just want
                // to ensure that our task handle is currently registered

                return Ok(Async::NotReady);
            }
            COMPLETE => {
                return unsafe { self.clone_result().map(Async::Ready) };
            }
            POISONED => panic!("inner future panicked during poll"),
            _ => unreachable!(),
        }

        struct Reset<'a>(&'a AtomicUsize);

        impl<'a> Drop for Reset<'a> {
            fn drop(&mut self) {
                use std::thread;

                if thread::panicking() {
                    self.0.store(POISONED, SeqCst);
                }
            }
        }

        let _reset = Reset(&self.inner.notifier.state);

        // Poll the future
        let res = unsafe {
            (*self.inner.future.get()).as_mut().unwrap()
                .poll_future_notify(&self.inner.notifier, 0)
        };
        match res {
            Ok(Async::NotReady) => {
                // Not ready, try to release the handle
                match self.inner.notifier.state.compare_and_swap(POLLING, IDLE, SeqCst) {
                    POLLING => {
                        // Success
                        return Ok(Async::NotReady);
                    }
                    _ => unreachable!(),
                }

            }
            Ok(Async::Ready(i)) => {
                unsafe {
                    (*self.inner.result.get()) = Some(Ok(SharedItem { item: Arc::new(i) }));
                }
            }
            Err(e) => {
                unsafe {
                    (*self.inner.result.get()) = Some(Err(SharedError { error: Arc::new(e) }));
                }
            }
        }

        self.complete();
        unsafe { self.clone_result().map(Async::Ready) }
    }
}

impl<F> Clone for Shared<F> where F: Future {
    fn clone(&self) -> Self {
        let next_clone_id = self.inner.next_clone_id.fetch_add(1, SeqCst);

        Shared {
            inner: self.inner.clone(),
            waiter: next_clone_id,
        }
    }
}

impl<F> Drop for Shared<F> where F: Future {
    fn drop(&mut self) {
        let mut waiters = self.inner.notifier.waiters.lock().unwrap();
        waiters.remove(&self.waiter);
    }
}

impl Notify for Notifier {
    fn notify(&self, _id: usize) {
        let waiters = mem::replace(&mut *self.waiters.lock().unwrap(), HashMap::new());

        for (_, waiter) in waiters {
            waiter.notify();
        }
    }
}

// The `F` is synchronized by a lock, so `F` doesn't need
// to be `Sync`. However, its `Item` or `Error` are exposed
// through an `Arc` but not lock, so they must be `Send + Sync`.
unsafe impl<F> Send for Inner<F>
    where F: Future + Send,
          F::Item: Send + Sync,
          F::Error: Send + Sync,
{}

unsafe impl<F> Sync for Inner<F>
    where F: Future + Send,
          F::Item: Send + Sync,
          F::Error: Send + Sync,
{}

impl<F> fmt::Debug for Inner<F>
    where F: Future + fmt::Debug,
          F::Item: fmt::Debug,
          F::Error: fmt::Debug,
{
    fn fmt(&self, fmt: &mut fmt::Formatter) -> fmt::Result {
        fmt.debug_struct("Inner")
            .finish()
    }
}

/// A wrapped item of the original future that is cloneable and implements Deref
/// for ease of use.
#[derive(Clone, Debug)]
pub struct SharedItem<T> {
    item: Arc<T>,
}

impl<T> ops::Deref for SharedItem<T> {
    type Target = T;

    fn deref(&self) -> &T {
        &self.item.as_ref()
    }
}

/// A wrapped error of the original future that is cloneable and implements Deref
/// for ease of use.
#[derive(Clone, Debug)]
pub struct SharedError<E> {
    error: Arc<E>,
}

impl<E> ops::Deref for SharedError<E> {
    type Target = E;

    fn deref(&self) -> &E {
        &self.error.as_ref()
    }
}

impl<E> fmt::Display for SharedError<E>
    where E: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        self.error.fmt(f)
    }
}

impl<E> error::Error for SharedError<E>
    where E: error::Error,
{
    #[allow(deprecated)]
    fn description(&self) -> &str {
        self.error.description()
    }

    #[allow(deprecated)]
    fn cause(&self) -> Option<&error::Error> {
        self.error.cause()
    }
}
