use crate::utils::{self, Indexer};

use super::Race as RaceTrait;

use core::fmt;
use core::future::{Future, IntoFuture};
use core::pin::Pin;
use core::task::{Context, Poll};

use pin_project::pin_project;

/// A future which waits for the first future to complete.
///
/// This `struct` is created by the [`race`] method on the [`Race`] trait. See
/// its documentation for more.
///
/// [`race`]: crate::future::Race::race
/// [`Race`]: crate::future::Race
#[must_use = "futures do nothing unless you `.await` or poll them"]
#[pin_project]
pub struct Race<Fut, const N: usize>
where
    Fut: Future,
{
    #[pin]
    futures: [Fut; N],
    indexer: Indexer<N>,
    done: bool,
}

impl<Fut, const N: usize> fmt::Debug for Race<Fut, N>
where
    Fut: Future + fmt::Debug,
    Fut::Output: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.futures.iter()).finish()
    }
}

impl<Fut, const N: usize> Future for Race<Fut, N>
where
    Fut: Future,
{
    type Output = Fut::Output;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        assert!(!*this.done, "Futures must not be polled after completing");

        for index in this.indexer.iter() {
            let fut = utils::get_pin_mut(this.futures.as_mut(), index).unwrap();
            match fut.poll(cx) {
                Poll::Ready(item) => {
                    *this.done = true;
                    return Poll::Ready(item);
                }
                Poll::Pending => continue,
            }
        }
        Poll::Pending
    }
}

impl<Fut, const N: usize> RaceTrait for [Fut; N]
where
    Fut: IntoFuture,
{
    type Output = Fut::Output;
    type Future = Race<Fut::IntoFuture, N>;

    fn race(self) -> Self::Future {
        assert!(N > 0, "race requires at least one future");
        Race {
            futures: self.map(|fut| fut.into_future()),
            indexer: Indexer::new(),
            done: false,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use core::future;

    #[test]
    #[should_panic(expected = "race requires at least one future")]
    fn empty_array() {
        let futs: [future::Ready<()>; 0] = [];
        let _ = futs.race();
    }

    // NOTE: we should probably poll in random order.
    #[test]
    fn no_fairness() {
        futures_lite::future::block_on(async {
            let res = [future::ready("hello"), future::ready("world")]
                .race()
                .await;
            assert!(matches!(res, "hello" | "world"));
        });
    }
}
