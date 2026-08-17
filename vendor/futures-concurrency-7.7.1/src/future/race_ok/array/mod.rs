use super::RaceOk as RaceOkTrait;
use crate::utils::array_assume_init;
use crate::utils::iter_pin_mut;
use crate::utils::PollArray;

use core::array;
use core::fmt;
use core::future::{Future, IntoFuture};
use core::mem::{self, MaybeUninit};
use core::pin::Pin;
use core::task::{Context, Poll};

use pin_project::{pin_project, pinned_drop};

mod error;

pub use error::AggregateError;

/// A future which waits for the first successful future to complete.
///
/// This `struct` is created by the [`race_ok`] method on the [`RaceOk`] trait. See
/// its documentation for more.
///
/// [`race_ok`]: crate::future::RaceOk::race_ok
/// [`RaceOk`]: crate::future::RaceOk
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[pin_project(PinnedDrop)]
pub struct RaceOk<Fut, T, E, const N: usize>
where
    Fut: Future<Output = Result<T, E>>,
{
    #[pin]
    futures: [Fut; N],
    errors: [MaybeUninit<E>; N],
    error_states: PollArray<N>,
    completed: usize,
}

#[pinned_drop]
impl<Fut, T, E, const N: usize> PinnedDrop for RaceOk<Fut, T, E, N>
where
    Fut: Future<Output = Result<T, E>>,
{
    fn drop(self: Pin<&mut Self>) {
        let this = self.project();
        for (st, err) in this
            .error_states
            .iter_mut()
            .zip(this.errors.iter_mut())
            .filter(|(st, _err)| st.is_ready())
        {
            // SAFETY: we've filtered down to only the `ready`/initialized data
            unsafe { err.assume_init_drop() };
            st.set_none();
        }
    }
}

impl<Fut, T, E, const N: usize> fmt::Debug for RaceOk<Fut, T, E, N>
where
    Fut: Future<Output = Result<T, E>> + fmt::Debug,
    Fut::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.futures.iter()).finish()
    }
}

impl<Fut, T, E, const N: usize> Future for RaceOk<Fut, T, E, N>
where
    Fut: Future<Output = Result<T, E>>,
{
    type Output = Result<T, AggregateError<E, N>>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let this = self.project();

        let futures = iter_pin_mut(this.futures);

        for ((fut, out), st) in futures
            .zip(this.errors.iter_mut())
            .zip(this.error_states.iter_mut())
        {
            if st.is_ready() {
                continue;
            }
            if let Poll::Ready(output) = fut.poll(cx) {
                match output {
                    Ok(ok) => return Poll::Ready(Ok(ok)),
                    Err(err) => {
                        *out = MaybeUninit::new(err);
                        *this.completed += 1;
                        st.set_ready();
                    }
                }
            }
        }

        let all_completed = *this.completed == N;
        if all_completed {
            let mut errors = array::from_fn(|_| MaybeUninit::uninit());
            mem::swap(&mut errors, this.errors);
            this.error_states.set_all_none();

            // SAFETY: we know that all futures are properly initialized because they're all completed
            let result = unsafe { array_assume_init(errors) };

            Poll::Ready(Err(AggregateError::new(result)))
        } else {
            Poll::Pending
        }
    }
}

impl<Fut, T, E, const N: usize> RaceOkTrait for [Fut; N]
where
    Fut: IntoFuture<Output = Result<T, E>>,
{
    type Output = T;
    type Error = AggregateError<E, N>;
    type Future = RaceOk<Fut::IntoFuture, T, E, N>;

    fn race_ok(self) -> Self::Future {
        RaceOk {
            futures: self.map(|fut| fut.into_future()),
            errors: array::from_fn(|_| MaybeUninit::uninit()),
            error_states: PollArray::new_pending(),
            completed: 0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::future;

    #[test]
    fn all_ok() {
        futures_lite::future::block_on(async {
            let res: Result<&str, AggregateError<(), 2>> =
                [future::ready(Ok("hello")), future::ready(Ok("world"))]
                    .race_ok()
                    .await;
            assert!(res.is_ok());
        })
    }

    #[test]
    fn one_err() {
        futures_lite::future::block_on(async {
            let res: Result<&str, AggregateError<_, 2>> =
                [future::ready(Ok("hello")), future::ready(Err("oh no"))]
                    .race_ok()
                    .await;
            assert_eq!(res.unwrap(), "hello");
        });
    }

    #[test]
    fn all_err() {
        futures_lite::future::block_on(async {
            let res: Result<&str, AggregateError<_, 2>> =
                [future::ready(Err("oops")), future::ready(Err("oh no"))]
                    .race_ok()
                    .await;
            let errs = res.unwrap_err();
            assert_eq!(errs[0], "oops");
            assert_eq!(errs[1], "oh no");
        });
    }

    #[test]
    fn resume_after_completion() {
        use futures_lite::future::yield_now;
        futures_lite::future::block_on(async {
            let fut = |ok| async move {
                if ok {
                    yield_now().await;
                    yield_now().await;
                    Ok(())
                } else {
                    Err(())
                }
            };

            let res = [fut(true), fut(false)].race_ok().await;
            assert_eq!(res.ok().unwrap(), ());
        });
    }

    #[test]
    fn drop_errors() {
        use futures_lite::future::yield_now;

        struct Droper<'a>(&'a core::cell::Cell<usize>);
        impl Drop for Droper<'_> {
            fn drop(&mut self) {
                self.0.set(self.0.get() + 1);
            }
        }

        futures_lite::future::block_on(async {
            let drop_count = Default::default();
            let fut = |ok| {
                let drop_count = &drop_count;
                async move {
                    if ok {
                        yield_now().await;
                        yield_now().await;
                        Ok(())
                    } else {
                        Err(Droper(drop_count))
                    }
                }
            };
            let res = [fut(true), fut(false)].race_ok().await;
            assert_eq!(drop_count.get(), 1);
            assert_eq!(res.ok().unwrap(), ());

            drop_count.set(0);
            let res = [fut(false), fut(false)].race_ok().await;
            assert!(res.is_err());
            assert_eq!(drop_count.get(), 0);
            drop(res);
            assert_eq!(drop_count.get(), 2);
        })
    }
}
