use core::future::Future;
use core::pin::Pin;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    #[derive(Clone, Debug)]
    #[cfg(feature = "unstable")]
    #[cfg_attr(feature = "docs", doc(cfg(unstable)))]
    pub struct UnzipFuture<S, FromA, FromB> {
        #[pin]
        stream: S,
        res: Option<(FromA, FromB)>,
    }
}

impl<S: Stream, FromA, FromB> UnzipFuture<S, FromA, FromB>
where
    FromA: Default,
    FromB: Default,
{
    pub(super) fn new(stream: S) -> Self {
        UnzipFuture {
            stream,
            res: Some((FromA::default(), FromB::default())),
        }
    }
}

impl<S, A, B, FromA, FromB> Future for UnzipFuture<S, FromA, FromB>
where
    S: Stream<Item = (A, B)>,
    FromA: Default + Extend<A>,
    FromB: Default + Extend<B>,
{
    type Output = (FromA, FromB);

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();

        loop {
            let next = futures_core::ready!(this.stream.as_mut().poll_next(cx));

            match next {
                Some((a, b)) => {
                    let res = this.res.as_mut().unwrap();
                    res.0.extend(Some(a));
                    res.1.extend(Some(b));
                }
                None => return Poll::Ready(this.res.take().unwrap()),
            }
        }
    }
}
