use super::TryJoin as TryJoinTrait;
use crate::utils::{FutureVec, OutputVec, PollVec, WakerVec};

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::vec::Vec;

use core::fmt;
use core::future::{Future, IntoFuture};
use core::mem::ManuallyDrop;
use core::ops::DerefMut;
use core::pin::Pin;
use core::task::{Context, Poll};

use pin_project::{pin_project, pinned_drop};

/// A future which waits for all futures to complete successfully, or abort early on error.
///
/// This `struct` is created by the [`try_join`] method on the [`TryJoin`] trait. See
/// its documentation for more.
///
/// [`try_join`]: crate::future::TryJoin::try_join
/// [`TryJoin`]: crate::future::TryJoin
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[pin_project(PinnedDrop)]
pub struct TryJoin<Fut, T, E>
where
    Fut: Future<Output = Result<T, E>>,
{
    /// A boolean which holds whether the future has completed
    consumed: bool,
    /// The number of futures which are currently still in-flight
    pending: usize,
    /// The output data, to be returned after the future completes
    items: OutputVec<T>,
    /// A structure holding the waker passed to the future, and the various
    /// sub-wakers passed to the contained futures.
    wakers: WakerVec,
    /// The individual poll state of each future.
    state: PollVec,
    #[pin]
    /// The array of futures passed to the structure.
    futures: FutureVec<Fut>,
}

impl<Fut, T, E> TryJoin<Fut, T, E>
where
    Fut: Future<Output = Result<T, E>>,
{
    #[inline]
    pub(crate) fn new(futures: Vec<Fut>) -> Self {
        let len = futures.len();
        Self {
            consumed: false,
            pending: len,
            items: OutputVec::uninit(len),
            wakers: WakerVec::new(len),
            state: PollVec::new_pending(len),
            futures: FutureVec::new(futures),
        }
    }
}

impl<Fut, T, E> TryJoinTrait for Vec<Fut>
where
    Fut: IntoFuture<Output = Result<T, E>>,
{
    type Output = Vec<T>;
    type Error = E;
    type Future = TryJoin<Fut::IntoFuture, T, E>;

    fn try_join(self) -> Self::Future {
        TryJoin::new(self.into_iter().map(IntoFuture::into_future).collect())
    }
}

impl<Fut, T, E> fmt::Debug for TryJoin<Fut, T, E>
where
    Fut: Future<Output = Result<T, E>> + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.state.iter()).finish()
    }
}

impl<Fut, T, E> Future for TryJoin<Fut, T, E>
where
    Fut: Future<Output = Result<T, E>>,
{
    type Output = Result<Vec<T>, E>;

    #[inline]
    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        assert!(
            !*this.consumed,
            "Futures must not be polled after completing"
        );

        let mut readiness = this.wakers.readiness();
        readiness.set_waker(cx.waker());
        if *this.pending != 0 && !readiness.any_ready() {
            // Nothing is ready yet
            return Poll::Pending;
        }

        // Poll all ready futures
        for (i, mut fut) in this.futures.iter().enumerate() {
            if this.state[i].is_pending() && readiness.clear_ready(i) {
                // unlock readiness so we don't deadlock when polling
                #[allow(clippy::drop_non_drop)]
                drop(readiness);

                // Obtain the intermediate waker.
                let mut cx = Context::from_waker(this.wakers.get(i).unwrap());

                // Poll the future
                // SAFETY: the future's state was "pending", so it's safe to poll
                if let Poll::Ready(value) = unsafe {
                    fut.as_mut()
                        .map_unchecked_mut(|t| t.deref_mut())
                        .poll(&mut cx)
                } {
                    *this.pending -= 1;

                    // Check the value, short-circuit on error.
                    match value {
                        Ok(value) => {
                            this.items.write(i, value);

                            // SAFETY: We're marking the state as "ready", which
                            // means the future has been consumed, and data is
                            // now available to be consumed. The future will no
                            // longer be used after this point so it's safe to drop.
                            this.state[i].set_ready();
                            unsafe { ManuallyDrop::drop(fut.get_unchecked_mut()) };
                        }
                        Err(err) => {
                            // The future should no longer be polled after we're done here
                            *this.consumed = true;

                            // SAFETY: We're about to return the error value
                            // from the future, and drop the entire future.
                            // We're marking the future as consumed, and then
                            // proceeding to drop all other futures and
                            // initiatlized values in the destructor.
                            this.state[i].set_none();
                            unsafe { ManuallyDrop::drop(fut.get_unchecked_mut()) };

                            return Poll::Ready(Err(err));
                        }
                    }
                }

                // Lock readiness so we can use it again
                readiness = this.wakers.readiness();
            }
        }

        // Check whether we're all done now or need to keep going.
        if *this.pending == 0 {
            // Mark all data as "consumed" before we take it
            *this.consumed = true;
            for state in this.state.iter_mut() {
                debug_assert!(
                    state.is_ready(),
                    "Future should have reached a `Ready` state"
                );
                state.set_none();
            }

            // SAFETY: we've checked with the state that all of our outputs have been
            // filled, which means we're ready to take the data and assume it's initialized.
            Poll::Ready(Ok(unsafe { this.items.take() }))
        } else {
            Poll::Pending
        }
    }
}

/// Drop the already initialized values on cancellation.
#[pinned_drop]
impl<Fut, T, E> PinnedDrop for TryJoin<Fut, T, E>
where
    Fut: Future<Output = Result<T, E>>,
{
    fn drop(self: Pin<&mut Self>) {
        let mut this = self.project();

        // Drop all initialized values.
        for i in this.state.ready_indexes() {
            // SAFETY: we've just filtered down to *only* the initialized values.
            // We can assume they're initialized, and this is where we drop them.
            unsafe { this.items.drop(i) };
        }

        // Drop all pending futures.
        for i in this.state.pending_indexes() {
            // SAFETY: we've just filtered down to *only* the pending futures,
            // which have not yet been dropped.
            unsafe { this.futures.as_mut().drop(i) };
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use alloc::vec;
    use core::future;

    #[test]
    fn all_ok() {
        futures_lite::future::block_on(async {
            let res: Result<_, ()> = vec![future::ready(Ok("hello")), future::ready(Ok("world"))]
                .try_join()
                .await;
            assert_eq!(res.unwrap(), ["hello", "world"]);
        })
    }

    #[test]
    fn empty() {
        futures_lite::future::block_on(async {
            let data: Vec<future::Ready<Result<(), ()>>> = vec![];
            let res = data.try_join().await;
            assert_eq!(res.unwrap(), vec![]);
        });
    }

    #[test]
    fn one_err() {
        futures_lite::future::block_on(async {
            let res: Result<_, _> = vec![future::ready(Ok("hello")), future::ready(Err("oh no"))]
                .try_join()
                .await;
            assert_eq!(res.unwrap_err(), "oh no");
        });
    }
}
