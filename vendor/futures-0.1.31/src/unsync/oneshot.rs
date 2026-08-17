//! A one-shot, futures-aware channel
//!
//! This channel is similar to that in `sync::oneshot` but cannot be sent across
//! threads.

use std::cell::{Cell, RefCell};
use std::fmt;
use std::rc::{Rc, Weak};

use {Future, Poll, Async};
use future::{Executor, IntoFuture, Lazy, lazy};
use task::{self, Task};

/// Creates a new futures-aware, one-shot channel.
///
/// This function is the same as `sync::oneshot::channel` except that the
/// returned values cannot be sent across threads.
pub fn channel<T>() -> (Sender<T>, Receiver<T>) {
    let inner = Rc::new(RefCell::new(Inner {
        value: None,
        tx_task: None,
        rx_task: None,
    }));
    let tx = Sender {
        inner: Rc::downgrade(&inner),
    };
    let rx = Receiver {
        state: State::Open(inner),
    };
    (tx, rx)
}

/// Represents the completion half of a oneshot through which the result of a
/// computation is signaled.
///
/// This is created by the `unsync::oneshot::channel` function and is equivalent
/// in functionality to `sync::oneshot::Sender` except that it cannot be sent
/// across threads.
#[derive(Debug)]
pub struct Sender<T> {
    inner: Weak<RefCell<Inner<T>>>,
}

/// A future representing the completion of a computation happening elsewhere in
/// memory.
///
/// This is created by the `unsync::oneshot::channel` function and is equivalent
/// in functionality to `sync::oneshot::Receiver` except that it cannot be sent
/// across threads.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct Receiver<T> {
    state: State<T>,
}

#[derive(Debug)]
enum State<T> {
    Open(Rc<RefCell<Inner<T>>>),
    Closed(Option<T>),
}

pub use sync::oneshot::Canceled;

#[derive(Debug)]
struct Inner<T> {
    value: Option<T>,
    tx_task: Option<Task>,
    rx_task: Option<Task>,
}

impl<T> Sender<T> {
    /// Completes this oneshot with a successful result.
    ///
    /// This function will consume `self` and indicate to the other end, the
    /// `Receiver`, that the error provided is the result of the computation this
    /// represents.
    ///
    /// If the value is successfully enqueued for the remote end to receive,
    /// then `Ok(())` is returned. If the receiving end was deallocated before
    /// this function was called, however, then `Err` is returned with the value
    /// provided.
    pub fn send(self, val: T) -> Result<(), T> {
        if let Some(inner) = self.inner.upgrade() {
            inner.borrow_mut().value = Some(val);
            Ok(())
        } else {
            Err(val)
        }
    }

    /// Polls this `Sender` half to detect whether the `Receiver` this has
    /// paired with has gone away.
    ///
    /// This function can be used to learn about when the `Receiver` (consumer)
    /// half has gone away and nothing will be able to receive a message sent
    /// from `complete`.
    ///
    /// Like `Future::poll`, this function will panic if it's not called from
    /// within the context of a task. In other words, this should only ever be
    /// called from inside another future.
    ///
    /// If `Ready` is returned then it means that the `Receiver` has disappeared
    /// and the result this `Sender` would otherwise produce should no longer
    /// be produced.
    ///
    /// If `NotReady` is returned then the `Receiver` is still alive and may be
    /// able to receive a message if sent. The current task, however, is
    /// scheduled to receive a notification if the corresponding `Receiver` goes
    /// away.
    pub fn poll_cancel(&mut self) -> Poll<(), ()> {
        match self.inner.upgrade() {
            Some(inner) => {
                inner.borrow_mut().tx_task = Some(task::current());
                Ok(Async::NotReady)
            }
            None => Ok(().into()),
        }
    }

    /// Tests to see whether this `Sender`'s corresponding `Receiver`
    /// has gone away.
    ///
    /// This function can be used to learn about when the `Receiver` (consumer)
    /// half has gone away and nothing will be able to receive a message sent
    /// from `send`.
    ///
    /// Note that this function is intended to *not* be used in the context of a
    /// future. If you're implementing a future you probably want to call the
    /// `poll_cancel` function which will block the current task if the
    /// cancellation hasn't happened yet. This can be useful when working on a
    /// non-futures related thread, though, which would otherwise panic if
    /// `poll_cancel` were called.
    pub fn is_canceled(&self) -> bool {
        !self.inner.upgrade().is_some()
    }
}

impl<T> Drop for Sender<T> {
    fn drop(&mut self) {
        let inner = match self.inner.upgrade() {
            Some(inner) => inner,
            None => return,
        };
        let rx_task = {
            let mut borrow = inner.borrow_mut();
            borrow.tx_task.take();
            borrow.rx_task.take()
        };
        if let Some(task) = rx_task {
            task.notify();
        }
    }
}

impl<T> Receiver<T> {
    /// Gracefully close this receiver, preventing sending any future messages.
    ///
    /// Any `send` operation which happens after this method returns is
    /// guaranteed to fail. Once this method is called the normal `poll` method
    /// can be used to determine whether a message was actually sent or not. If
    /// `Canceled` is returned from `poll` then no message was sent.
    pub fn close(&mut self) {
        let (item, task) = match self.state {
            State::Open(ref inner) => {
                let mut inner = inner.borrow_mut();
                drop(inner.rx_task.take());
                (inner.value.take(), inner.tx_task.take())
            }
            State::Closed(_) => return,
        };
        self.state = State::Closed(item);
        if let Some(task) = task {
            task.notify();
        }
    }
}

impl<T> Future for Receiver<T> {
    type Item = T;
    type Error = Canceled;

    fn poll(&mut self) -> Poll<Self::Item, Self::Error> {
        let inner = match self.state {
            State::Open(ref mut inner) => inner,
            State::Closed(ref mut item) => {
                match item.take() {
                    Some(item) => return Ok(item.into()),
                    None => return Err(Canceled),
                }
            }
        };

        // If we've got a value, then skip the logic below as we're done.
        if let Some(val) = inner.borrow_mut().value.take() {
            return Ok(Async::Ready(val))
        }

        // If we can get mutable access, then the sender has gone away. We
        // didn't see a value above, so we're canceled. Otherwise we park
        // our task and wait for a value to come in.
        if Rc::get_mut(inner).is_some() {
            Err(Canceled)
        } else {
            inner.borrow_mut().rx_task = Some(task::current());
            Ok(Async::NotReady)
        }
    }
}

impl<T> Drop for Receiver<T> {
    fn drop(&mut self) {
        self.close();
    }
}

/// Handle returned from the `spawn` function.
///
/// This handle is a future representing the completion of a different future on
/// a separate executor. Created through the `oneshot::spawn` function this
/// handle will resolve when the future provided to `spawn` resolves on the
/// `Executor` instance provided to that function.
///
/// If this handle is dropped then the future will automatically no longer be
/// polled and is scheduled to be dropped. This can be canceled with the
/// `forget` function, however.
pub struct SpawnHandle<T, E> {
    rx: Receiver<Result<T, E>>,
    keep_running: Rc<Cell<bool>>,
}

/// Type of future which `Spawn` instances below must be able to spawn.
pub struct Execute<F: Future> {
    future: F,
    tx: Option<Sender<Result<F::Item, F::Error>>>,
    keep_running: Rc<Cell<bool>>,
}

/// Spawns a `future` onto the instance of `Executor` provided, `executor`,
/// returning a handle representing the completion of the future.
///
/// The `SpawnHandle` returned is a future that is a proxy for `future` itself.
/// When `future` completes on `executor` then the `SpawnHandle` will itself be
/// resolved.  Internally `SpawnHandle` contains a `oneshot` channel and is
/// thus not safe to send across threads.
///
/// The `future` will be canceled if the `SpawnHandle` is dropped. If this is
/// not desired then the `SpawnHandle::forget` function can be used to continue
/// running the future to completion.
///
/// # Panics
///
/// This function will panic if the instance of `Spawn` provided is unable to
/// spawn the `future` provided.
///
/// If the provided instance of `Spawn` does not actually run `future` to
/// completion, then the returned handle may panic when polled. Typically this
/// is not a problem, though, as most instances of `Spawn` will run futures to
/// completion.
pub fn spawn<F, E>(future: F, executor: &E) -> SpawnHandle<F::Item, F::Error>
    where F: Future,
          E: Executor<Execute<F>>,
{
    let flag = Rc::new(Cell::new(false));
    let (tx, rx) = channel();
    executor.execute(Execute {
        future: future,
        tx: Some(tx),
        keep_running: flag.clone(),
    }).expect("failed to spawn future");
    SpawnHandle {
        rx: rx,
        keep_running: flag,
    }
}

/// Spawns a function `f` onto the `Spawn` instance provided `s`.
///
/// For more information see the `spawn` function in this module. This function
/// is just a thin wrapper around `spawn` which will execute the closure on the
/// executor provided and then complete the future that the closure returns.
pub fn spawn_fn<F, R, E>(f: F, executor: &E) -> SpawnHandle<R::Item, R::Error>
    where F: FnOnce() -> R,
          R: IntoFuture,
          E: Executor<Execute<Lazy<F, R>>>,
{
    spawn(lazy(f), executor)
}

impl<T, E> SpawnHandle<T, E> {
    /// Drop this future without canceling the underlying future.
    ///
    /// When `SpawnHandle` is dropped, the spawned future will be canceled as
    /// well if the future hasn't already resolved. This function can be used
    /// when to drop this future but keep executing the underlying future.
    pub fn forget(self) {
        self.keep_running.set(true);
    }
}

impl<T, E> Future for SpawnHandle<T, E> {
    type Item = T;
    type Error = E;

    fn poll(&mut self) -> Poll<T, E> {
        match self.rx.poll() {
            Ok(Async::Ready(Ok(t))) => Ok(t.into()),
            Ok(Async::Ready(Err(e))) => Err(e),
            Ok(Async::NotReady) => Ok(Async::NotReady),
            Err(_) => panic!("future was canceled before completion"),
        }
    }
}

impl<T: fmt::Debug, E: fmt::Debug> fmt::Debug for SpawnHandle<T, E> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SpawnHandle")
         .finish()
    }
}

impl<F: Future> Future for Execute<F> {
    type Item = ();
    type Error = ();

    fn poll(&mut self) -> Poll<(), ()> {
        // If we're canceled then we may want to bail out early.
        //
        // If the `forget` function was called, though, then we keep going.
        if self.tx.as_mut().unwrap().poll_cancel().unwrap().is_ready() {
            if !self.keep_running.get() {
                return Ok(().into())
            }
        }

        let result = match self.future.poll() {
            Ok(Async::NotReady) => return Ok(Async::NotReady),
            Ok(Async::Ready(t)) => Ok(t),
            Err(e) => Err(e),
        };
        drop(self.tx.take().unwrap().send(result));
        Ok(().into())
    }
}

impl<F: Future + fmt::Debug> fmt::Debug for Execute<F> {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("Execute")
         .field("future", &self.future)
         .finish()
    }
}
