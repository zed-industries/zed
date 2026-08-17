use super::RaceOk as RaceOkTrait;
use crate::utils::iter_pin_mut;
use crate::utils::MaybeDone;

#[cfg(all(feature = "alloc", not(feature = "std")))]
use alloc::{boxed::Box, vec::Vec};

use core::fmt;
use core::future::{Future, IntoFuture};
use core::mem;
use core::pin::Pin;
use core::task::{Context, Poll};

pub use error::AggregateError;

mod error;

/// A future which waits for the first successful future to complete.
///
/// This `struct` is created by the [`race_ok`] method on the [`RaceOk`] trait. See
/// its documentation for more.
///
/// [`race_ok`]: crate::future::RaceOk::race_ok
/// [`RaceOk`]: crate::future::RaceOk
#[must_use = "futures do nothing unless you `.await` or poll them"]
pub struct RaceOk<Fut, T, E>
where
    Fut: Future<Output = Result<T, E>>,
{
    elems: Pin<Box<[MaybeDone<Fut>]>>,
}

impl<Fut, T, E> fmt::Debug for RaceOk<Fut, T, E>
where
    Fut: Future<Output = Result<T, E>> + fmt::Debug,
    Fut::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.elems.iter()).finish()
    }
}

impl<Fut, T, E> Future for RaceOk<Fut, T, E>
where
    Fut: Future<Output = Result<T, E>>,
{
    type Output = Result<T, AggregateError<E>>;

    fn poll(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut all_done = true;

        for mut elem in iter_pin_mut(self.elems.as_mut()) {
            if elem.as_mut().poll(cx).is_pending() {
                all_done = false
            } else if let Some(output) = elem.take_ok() {
                return Poll::Ready(Ok(output));
            }
        }

        if all_done {
            let mut elems = mem::replace(&mut self.elems, Box::pin([]));
            let result: Vec<E> = iter_pin_mut(elems.as_mut())
                .map(|e| match e.take_err() {
                    Some(err) => err,
                    // Since all futures are done without any one of them returning `Ok`, they're
                    // all `Err`s and so `take_err` cannot fail
                    None => unreachable!(),
                })
                .collect();
            Poll::Ready(Err(AggregateError::new(result)))
        } else {
            Poll::Pending
        }
    }
}

impl<Fut, T, E> RaceOkTrait for Vec<Fut>
where
    Fut: IntoFuture<Output = Result<T, E>>,
{
    type Output = T;
    type Error = AggregateError<E>;
    type Future = RaceOk<Fut::IntoFuture, T, E>;

    fn race_ok(self) -> Self::Future {
        let elems: Box<[_]> = self
            .into_iter()
            .map(|fut| MaybeDone::new(fut.into_future()))
            .collect();
        RaceOk {
            elems: elems.into(),
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
            let res: Result<&str, AggregateError<()>> =
                vec![future::ready(Ok("hello")), future::ready(Ok("world"))]
                    .race_ok()
                    .await;
            assert!(res.is_ok());
        })
    }

    #[test]
    fn one_err() {
        futures_lite::future::block_on(async {
            let res: Result<&str, AggregateError<_>> =
                vec![future::ready(Ok("hello")), future::ready(Err("oh no"))]
                    .race_ok()
                    .await;
            assert_eq!(res.unwrap(), "hello");
        });
    }

    #[test]
    fn all_err() {
        futures_lite::future::block_on(async {
            let res: Result<&str, AggregateError<_>> =
                vec![future::ready(Err("oops")), future::ready(Err("oh no"))]
                    .race_ok()
                    .await;
            let errs = res.unwrap_err();
            assert_eq!(errs[0], "oops");
            assert_eq!(errs[1], "oh no");
        });
    }
}
