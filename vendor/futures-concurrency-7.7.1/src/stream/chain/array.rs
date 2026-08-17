use core::fmt;
use core::pin::Pin;
use core::task::{Context, Poll};

use futures_core::Stream;
use pin_project::pin_project;

use crate::utils;

use super::Chain as ChainTrait;

/// A stream that chains multiple streams one after another.
///
/// This `struct` is created by the [`chain`] method on the [`Chain`] trait. See its
/// documentation for more.
///
/// [`chain`]: trait.Chain.html#method.merge
/// [`Chain`]: trait.Chain.html
#[pin_project]
pub struct Chain<S, const N: usize> {
    #[pin]
    streams: [S; N],
    index: usize,
    len: usize,
    done: bool,
}

impl<S: Stream, const N: usize> Stream for Chain<S, N> {
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        assert!(!*this.done, "Stream should not be polled after completion");

        loop {
            if this.index == this.len {
                *this.done = true;
                return Poll::Ready(None);
            }
            let stream = utils::iter_pin_mut(this.streams.as_mut())
                .nth(*this.index)
                .unwrap();
            match stream.poll_next(cx) {
                Poll::Ready(Some(item)) => return Poll::Ready(Some(item)),
                Poll::Ready(None) => {
                    *this.index += 1;
                    continue;
                }
                Poll::Pending => return Poll::Pending,
            }
        }
    }
}

impl<S, const N: usize> fmt::Debug for Chain<S, N>
where
    S: Stream + fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_list().entries(self.streams.iter()).finish()
    }
}

impl<S: Stream, const N: usize> ChainTrait for [S; N] {
    type Item = S::Item;

    type Stream = Chain<S, N>;

    fn chain(self) -> Self::Stream {
        Chain {
            len: self.len(),
            streams: self,
            index: 0,
            done: false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures_lite::future::block_on;
    use futures_lite::prelude::*;
    use futures_lite::stream;

    #[test]
    fn chain_3() {
        block_on(async {
            let a = stream::once(1);
            let b = stream::once(2);
            let c = stream::once(3);
            let mut s = [a, b, c].chain();

            assert_eq!(s.next().await, Some(1));
            assert_eq!(s.next().await, Some(2));
            assert_eq!(s.next().await, Some(3));
            assert_eq!(s.next().await, None);
        })
    }
}
