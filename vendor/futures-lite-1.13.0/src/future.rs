//! Combinators for the [`Future`] trait.
//!
//! # Examples
//!
//! ```
//! use futures_lite::future;
//!
//! # spin_on::spin_on(async {
//! for step in 0..3 {
//!     println!("step {}", step);
//!
//!     // Give other tasks a chance to run.
//!     future::yield_now().await;
//! }
//! # });
//! ```

#[cfg(feature = "alloc")]
extern crate alloc;

#[doc(no_inline)]
pub use core::future::Future;

use core::fmt;
use core::marker::PhantomData;
use core::pin::Pin;

use pin_project_lite::pin_project;

#[cfg(feature = "std")]
use std::{
    any::Any,
    panic::{catch_unwind, AssertUnwindSafe, UnwindSafe},
};

#[cfg(feature = "alloc")]
use alloc::boxed::Box;
use core::task::{Context, Poll};

/// Blocks the current thread on a future.
///
/// # Examples
///
/// ```
/// use futures_lite::future;
///
/// let val = future::block_on(async {
///     1 + 2
/// });
///
/// assert_eq!(val, 3);
/// ```
#[cfg(feature = "std")]
pub fn block_on<T>(future: impl Future<Output = T>) -> T {
    use std::cell::RefCell;
    use std::task::Waker;

    use parking::Parker;
    use waker_fn::waker_fn;

    // Pin the future on the stack.
    crate::pin!(future);

    // Creates a parker and an associated waker that unparks it.
    fn parker_and_waker() -> (Parker, Waker) {
        let parker = Parker::new();
        let unparker = parker.unparker();
        let waker = waker_fn(move || {
            unparker.unpark();
        });
        (parker, waker)
    }

    thread_local! {
        // Cached parker and waker for efficiency.
        static CACHE: RefCell<(Parker, Waker)> = RefCell::new(parker_and_waker());
    }

    CACHE.with(|cache| {
        // Try grabbing the cached parker and waker.
        match cache.try_borrow_mut() {
            Ok(cache) => {
                // Use the cached parker and waker.
                let (parker, waker) = &*cache;
                let cx = &mut Context::from_waker(waker);

                // Keep polling until the future is ready.
                loop {
                    match future.as_mut().poll(cx) {
                        Poll::Ready(output) => return output,
                        Poll::Pending => parker.park(),
                    }
                }
            }
            Err(_) => {
                // Looks like this is a recursive `block_on()` call.
                // Create a fresh parker and waker.
                let (parker, waker) = parker_and_waker();
                let cx = &mut Context::from_waker(&waker);

                // Keep polling until the future is ready.
                loop {
                    match future.as_mut().poll(cx) {
                        Poll::Ready(output) => return output,
                        Poll::Pending => parker.park(),
                    }
                }
            }
        }
    })
}

/// Creates a future that is always pending.
///
/// # Examples
///
/// ```no_run
/// use futures_lite::future;
///
/// # spin_on::spin_on(async {
/// future::pending::<()>().await;
/// unreachable!();
/// # })
/// ```
pub fn pending<T>() -> Pending<T> {
    Pending {
        _marker: PhantomData,
    }
}

/// Future for the [`pending()`] function.
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Pending<T> {
    _marker: PhantomData<T>,
}

impl<T> Unpin for Pending<T> {}

impl<T> fmt::Debug for Pending<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Pending").finish()
    }
}

impl<T> Future for Pending<T> {
    type Output = T;

    fn poll(self: Pin<&mut Self>, _: &mut Context<'_>) -> Poll<T> {
        Poll::Pending
    }
}

/// Polls a future just once and returns an [`Option`] with the result.
///
/// # Examples
///
/// ```
/// use futures_lite::future;
///
/// # spin_on::spin_on(async {
/// assert_eq!(future::poll_once(future::pending::<()>()).await, None);
/// assert_eq!(future::poll_once(future::ready(42)).await, Some(42));
/// # })
/// ```
pub fn poll_once<T, F>(f: F) -> PollOnce<F>
where
    F: Future<Output = T>,
{
    PollOnce { f }
}

pin_project! {
    /// Future for the [`poll_once()`] function.
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct PollOnce<F> {
        #[pin]
        f: F,
    }
}

impl<F> fmt::Debug for PollOnce<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PollOnce").finish()
    }
}

impl<T, F> Future for PollOnce<F>
where
    F: Future<Output = T>,
{
    type Output = Option<T>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        match self.project().f.poll(cx) {
            Poll::Ready(t) => Poll::Ready(Some(t)),
            Poll::Pending => Poll::Ready(None),
        }
    }
}

/// Creates a future from a function returning [`Poll`].
///
/// # Examples
///
/// ```
/// use futures_lite::future;
/// use std::task::{Context, Poll};
///
/// # spin_on::spin_on(async {
/// fn f(_: &mut Context<'_>) -> Poll<i32> {
///     Poll::Ready(7)
/// }
///
/// assert_eq!(future::poll_fn(f).await, 7);
/// # })
/// ```
pub fn poll_fn<T, F>(f: F) -> PollFn<F>
where
    F: FnMut(&mut Context<'_>) -> Poll<T>,
{
    PollFn { f }
}

pin_project! {
    /// Future for the [`poll_fn()`] function.
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct PollFn<F> {
        f: F,
    }
}

impl<F> fmt::Debug for PollFn<F> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("PollFn").finish()
    }
}

impl<T, F> Future for PollFn<F>
where
    F: FnMut(&mut Context<'_>) -> Poll<T>,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<T> {
        let this = self.project();
        (this.f)(cx)
    }
}

/// Creates a future that resolves to the provided value.
///
/// # Examples
///
/// ```
/// use futures_lite::future;
///
/// # spin_on::spin_on(async {
/// assert_eq!(future::ready(7).await, 7);
/// # })
/// ```
pub fn ready<T>(val: T) -> Ready<T> {
    Ready(Some(val))
}

/// Future for the [`ready()`] function.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct Ready<T>(Option<T>);

impl<T> Unpin for Ready<T> {}

impl<T> Future for Ready<T> {
    type Output = T;

    fn poll(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<T> {
        Poll::Ready(self.0.take().expect("`Ready` polled after completion"))
    }
}

/// Wakes the current task and returns [`Poll::Pending`] once.
///
/// This function is useful when we want to cooperatively give time to the task scheduler. It is
/// generally a good idea to yield inside loops because that way we make sure long-running tasks
/// don't prevent other tasks from running.
///
/// # Examples
///
/// ```
/// use futures_lite::future;
///
/// # spin_on::spin_on(async {
/// future::yield_now().await;
/// # })
/// ```
pub fn yield_now() -> YieldNow {
    YieldNow(false)
}

/// Future for the [`yield_now()`] function.
#[derive(Debug)]
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct YieldNow(bool);

impl Future for YieldNow {
    type Output = ();

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        if !self.0 {
            self.0 = true;
            cx.waker().wake_by_ref();
            Poll::Pending
        } else {
            Poll::Ready(())
        }
    }
}

/// Joins two futures, waiting for both to complete.
///
/// # Examples
///
/// ```
/// use futures_lite::future;
///
/// # spin_on::spin_on(async {
/// let a = async { 1 };
/// let b = async { 2 };
///
/// assert_eq!(future::zip(a, b).await, (1, 2));
/// # })
/// ```
pub fn zip<F1, F2>(future1: F1, future2: F2) -> Zip<F1, F2>
where
    F1: Future,
    F2: Future,
{
    Zip {
        future1,
        future2,
        output1: None,
        output2: None,
    }
}

pin_project! {
    /// Future for the [`zip()`] function.
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct Zip<F1, F2>
    where
        F1: Future,
        F2: Future,
    {
        #[pin]
        future1: F1,
        output1: Option<F1::Output>,
        #[pin]
        future2: F2,
        output2: Option<F2::Output>,
    }
}

impl<F1, F2> Future for Zip<F1, F2>
where
    F1: Future,
    F2: Future,
{
    type Output = (F1::Output, F2::Output);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if this.output1.is_none() {
            if let Poll::Ready(out) = this.future1.poll(cx) {
                *this.output1 = Some(out);
            }
        }

        if this.output2.is_none() {
            if let Poll::Ready(out) = this.future2.poll(cx) {
                *this.output2 = Some(out);
            }
        }

        if this.output1.is_some() && this.output2.is_some() {
            Poll::Ready((this.output1.take().unwrap(), this.output2.take().unwrap()))
        } else {
            Poll::Pending
        }
    }
}

/// Joins two fallible futures, waiting for both to complete or one of them to error.
///
/// # Examples
///
/// ```
/// use futures_lite::future;
///
/// # spin_on::spin_on(async {
/// let a = async { Ok::<i32, i32>(1) };
/// let b = async { Err::<i32, i32>(2) };
///
/// assert_eq!(future::try_zip(a, b).await, Err(2));
/// # })
/// ```
pub fn try_zip<T1, T2, E, F1, F2>(future1: F1, future2: F2) -> TryZip<F1, F2>
where
    F1: Future<Output = Result<T1, E>>,
    F2: Future<Output = Result<T2, E>>,
{
    TryZip {
        future1,
        future2,
        output1: None,
        output2: None,
    }
}

pin_project! {
    /// Future for the [`try_zip()`] function.
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct TryZip<F1, F2>
    where
        F1: Future,
        F2: Future,
    {
        #[pin]
        future1: F1,
        output1: Option<F1::Output>,
        #[pin]
        future2: F2,
        output2: Option<F2::Output>,
    }
}

impl<T1, T2, E, F1, F2> Future for TryZip<F1, F2>
where
    F1: Future<Output = Result<T1, E>>,
    F2: Future<Output = Result<T2, E>>,
{
    type Output = Result<(T1, T2), E>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if this.output1.is_none() {
            if let Poll::Ready(out) = this.future1.poll(cx) {
                match out {
                    Ok(t) => *this.output1 = Some(Ok(t)),
                    Err(err) => return Poll::Ready(Err(err)),
                }
            }
        }

        if this.output2.is_none() {
            if let Poll::Ready(out) = this.future2.poll(cx) {
                match out {
                    Ok(t) => *this.output2 = Some(Ok(t)),
                    Err(err) => return Poll::Ready(Err(err)),
                }
            }
        }

        if this.output1.is_some() && this.output2.is_some() {
            let res1 = this.output1.take().unwrap();
            let res2 = this.output2.take().unwrap();
            let t1 = res1.map_err(|_| unreachable!()).unwrap();
            let t2 = res2.map_err(|_| unreachable!()).unwrap();
            Poll::Ready(Ok((t1, t2)))
        } else {
            Poll::Pending
        }
    }
}

/// Returns the result of the future that completes first, preferring `future1` if both are ready.
///
/// If you need to treat the two futures fairly without a preference for either, use the [`race()`]
/// function or the [`FutureExt::race()`] method.
///
/// # Examples
///
/// ```
/// use futures_lite::future::{self, pending, ready};
///
/// # spin_on::spin_on(async {
/// assert_eq!(future::or(ready(1), pending()).await, 1);
/// assert_eq!(future::or(pending(), ready(2)).await, 2);
///
/// // The first future wins.
/// assert_eq!(future::or(ready(1), ready(2)).await, 1);
/// # })
/// ```
pub fn or<T, F1, F2>(future1: F1, future2: F2) -> Or<F1, F2>
where
    F1: Future<Output = T>,
    F2: Future<Output = T>,
{
    Or { future1, future2 }
}

pin_project! {
    /// Future for the [`or()`] function and the [`FutureExt::or()`] method.
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct Or<F1, F2> {
        #[pin]
        future1: F1,
        #[pin]
        future2: F2,
    }
}

impl<T, F1, F2> Future for Or<F1, F2>
where
    F1: Future<Output = T>,
    F2: Future<Output = T>,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if let Poll::Ready(t) = this.future1.poll(cx) {
            return Poll::Ready(t);
        }
        if let Poll::Ready(t) = this.future2.poll(cx) {
            return Poll::Ready(t);
        }
        Poll::Pending
    }
}

/// Returns the result of the future that completes first, with no preference if both are ready.
///
/// Each time [`Race`] is polled, the two inner futures are polled in random order. Therefore, no
/// future takes precedence over the other if both can complete at the same time.
///
/// If you have preference for one of the futures, use the [`or()`] function or the
/// [`FutureExt::or()`] method.
///
/// # Examples
///
/// ```
/// use futures_lite::future::{self, pending, ready};
///
/// # spin_on::spin_on(async {
/// assert_eq!(future::race(ready(1), pending()).await, 1);
/// assert_eq!(future::race(pending(), ready(2)).await, 2);
///
/// // One of the two futures is randomly chosen as the winner.
/// let res = future::race(ready(1), ready(2)).await;
/// # })
/// ```
#[cfg(feature = "std")]
pub fn race<T, F1, F2>(future1: F1, future2: F2) -> Race<F1, F2>
where
    F1: Future<Output = T>,
    F2: Future<Output = T>,
{
    Race { future1, future2 }
}

#[cfg(feature = "std")]
pin_project! {
    /// Future for the [`race()`] function and the [`FutureExt::race()`] method.
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct Race<F1, F2> {
        #[pin]
        future1: F1,
        #[pin]
        future2: F2,
    }
}

#[cfg(feature = "std")]
impl<T, F1, F2> Future for Race<F1, F2>
where
    F1: Future<Output = T>,
    F2: Future<Output = T>,
{
    type Output = T;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        if fastrand::bool() {
            if let Poll::Ready(t) = this.future1.poll(cx) {
                return Poll::Ready(t);
            }
            if let Poll::Ready(t) = this.future2.poll(cx) {
                return Poll::Ready(t);
            }
        } else {
            if let Poll::Ready(t) = this.future2.poll(cx) {
                return Poll::Ready(t);
            }
            if let Poll::Ready(t) = this.future1.poll(cx) {
                return Poll::Ready(t);
            }
        }
        Poll::Pending
    }
}

#[cfg(feature = "std")]
pin_project! {
    /// Future for the [`FutureExt::catch_unwind()`] method.
    #[derive(Debug)]
    #[must_use = "futures do nothing unless you `.await` or poll them"]
    pub struct CatchUnwind<F> {
        #[pin]
        inner: F,
    }
}

#[cfg(feature = "std")]
impl<F: Future + UnwindSafe> Future for CatchUnwind<F> {
    type Output = Result<F::Output, Box<dyn Any + Send>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();
        catch_unwind(AssertUnwindSafe(|| this.inner.poll(cx)))?.map(Ok)
    }
}

/// Type alias for `Pin<Box<dyn Future<Output = T> + Send + 'static>>`.
///
/// # Examples
///
/// ```
/// use futures_lite::future::{self, FutureExt};
///
/// // These two lines are equivalent:
/// let f1: future::Boxed<i32> = async { 1 + 2 }.boxed();
/// let f2: future::Boxed<i32> = Box::pin(async { 1 + 2 });
/// ```
#[cfg(feature = "alloc")]
pub type Boxed<T> = Pin<Box<dyn Future<Output = T> + Send + 'static>>;

/// Type alias for `Pin<Box<dyn Future<Output = T> + 'static>>`.
///
/// # Examples
///
/// ```
/// use futures_lite::future::{self, FutureExt};
///
/// // These two lines are equivalent:
/// let f1: future::BoxedLocal<i32> = async { 1 + 2 }.boxed_local();
/// let f2: future::BoxedLocal<i32> = Box::pin(async { 1 + 2 });
/// ```
#[cfg(feature = "alloc")]
pub type BoxedLocal<T> = Pin<Box<dyn Future<Output = T> + 'static>>;

/// Extension trait for [`Future`].
pub trait FutureExt: Future {
    /// A convenience for calling [`Future::poll()`] on `!`[`Unpin`] types.
    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Self::Output>
    where
        Self: Unpin,
    {
        Future::poll(Pin::new(self), cx)
    }

    /// Returns the result of `self` or `other` future, preferring `self` if both are ready.
    ///
    /// If you need to treat the two futures fairly without a preference for either, use the
    /// [`race()`] function or the [`FutureExt::race()`] method.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures_lite::future::{pending, ready, FutureExt};
    ///
    /// # spin_on::spin_on(async {
    /// assert_eq!(ready(1).or(pending()).await, 1);
    /// assert_eq!(pending().or(ready(2)).await, 2);
    ///
    /// // The first future wins.
    /// assert_eq!(ready(1).or(ready(2)).await, 1);
    /// # })
    /// ```
    fn or<F>(self, other: F) -> Or<Self, F>
    where
        Self: Sized,
        F: Future<Output = Self::Output>,
    {
        Or {
            future1: self,
            future2: other,
        }
    }

    /// Returns the result of `self` or `other` future, with no preference if both are ready.
    ///
    /// Each time [`Race`] is polled, the two inner futures are polled in random order. Therefore,
    /// no future takes precedence over the other if both can complete at the same time.
    ///
    /// If you have preference for one of the futures, use the [`or()`] function or the
    /// [`FutureExt::or()`] method.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures_lite::future::{pending, ready, FutureExt};
    ///
    /// # spin_on::spin_on(async {
    /// assert_eq!(ready(1).race(pending()).await, 1);
    /// assert_eq!(pending().race(ready(2)).await, 2);
    ///
    /// // One of the two futures is randomly chosen as the winner.
    /// let res = ready(1).race(ready(2)).await;
    /// # })
    /// ```
    #[cfg(feature = "std")]
    fn race<F>(self, other: F) -> Race<Self, F>
    where
        Self: Sized,
        F: Future<Output = Self::Output>,
    {
        Race {
            future1: self,
            future2: other,
        }
    }

    /// Catches panics while polling the future.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures_lite::future::FutureExt;
    ///
    /// # spin_on::spin_on(async {
    /// let fut1 = async {}.catch_unwind();
    /// let fut2 = async { panic!() }.catch_unwind();
    ///
    /// assert!(fut1.await.is_ok());
    /// assert!(fut2.await.is_err());
    /// # })
    /// ```
    #[cfg(feature = "std")]
    fn catch_unwind(self) -> CatchUnwind<Self>
    where
        Self: Sized + UnwindSafe,
    {
        CatchUnwind { inner: self }
    }

    /// Boxes the future and changes its type to `dyn Future + Send + 'a`.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures_lite::future::{self, FutureExt};
    ///
    /// # spin_on::spin_on(async {
    /// let a = future::ready('a');
    /// let b = future::pending();
    ///
    /// // Futures of different types can be stored in
    /// // the same collection when they are boxed:
    /// let futures = vec![a.boxed(), b.boxed()];
    /// # })
    /// ```
    #[cfg(feature = "alloc")]
    fn boxed<'a>(self) -> Pin<Box<dyn Future<Output = Self::Output> + Send + 'a>>
    where
        Self: Sized + Send + 'a,
    {
        Box::pin(self)
    }

    /// Boxes the future and changes its type to `dyn Future + 'a`.
    ///
    /// # Examples
    ///
    /// ```
    /// use futures_lite::future::{self, FutureExt};
    ///
    /// # spin_on::spin_on(async {
    /// let a = future::ready('a');
    /// let b = future::pending();
    ///
    /// // Futures of different types can be stored in
    /// // the same collection when they are boxed:
    /// let futures = vec![a.boxed_local(), b.boxed_local()];
    /// # })
    /// ```
    #[cfg(feature = "alloc")]
    fn boxed_local<'a>(self) -> Pin<Box<dyn Future<Output = Self::Output> + 'a>>
    where
        Self: Sized + 'a,
    {
        Box::pin(self)
    }
}

impl<F: Future + ?Sized> FutureExt for F {}
