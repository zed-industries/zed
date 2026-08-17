//! Definition of the Lazy combinator, deferring execution of a function until
//! the future is polled.

use core::mem;

use {Future, IntoFuture, Poll};

/// A future which defers creation of the actual future until a callback is
/// scheduled.
///
/// This is created by the `lazy` function.
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct Lazy<F, R: IntoFuture> {
    inner: _Lazy<F, R::Future>,
}

#[derive(Debug)]
enum _Lazy<F, R> {
    First(F),
    Second(R),
    Moved,
}

/// Creates a new future which will eventually be the same as the one created
/// by the closure provided.
///
/// The provided closure is only run once the future has a callback scheduled
/// on it, otherwise the callback never runs. Once run, however, this future is
/// the same as the one the closure creates.
///
/// # Examples
///
/// ```
/// use futures::future::*;
///
/// let a = lazy(|| ok::<u32, u32>(1));
///
/// let b = lazy(|| -> FutureResult<u32, u32> {
///     panic!("oh no!")
/// });
/// drop(b); // closure is never run
/// ```
pub fn lazy<F, R>(f: F) -> Lazy<F, R>
    where F: FnOnce() -> R,
          R: IntoFuture
{
    Lazy {
        inner: _Lazy::First(f),
    }
}

impl<F, R> Lazy<F, R>
    where F: FnOnce() -> R,
          R: IntoFuture,
{
    fn get(&mut self) -> &mut R::Future {
        match self.inner {
            _Lazy::First(_) => {}
            _Lazy::Second(ref mut f) => return f,
            _Lazy::Moved => panic!(), // can only happen if `f()` panics
        }
        match mem::replace(&mut self.inner, _Lazy::Moved) {
            _Lazy::First(f) => self.inner = _Lazy::Second(f().into_future()),
            _ => panic!(), // we already found First
        }
        match self.inner {
            _Lazy::Second(ref mut f) => f,
            _ => panic!(), // we just stored Second
        }
    }
}

impl<F, R> Future for Lazy<F, R>
    where F: FnOnce() -> R,
          R: IntoFuture,
{
    type Item = R::Item;
    type Error = R::Error;

    fn poll(&mut self) -> Poll<R::Item, R::Error> {
        self.get().poll()
    }
}
