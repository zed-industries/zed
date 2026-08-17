use core::future::Future;
use core::pin::Pin;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    #[derive(Debug)]
    pub struct FoldFuture<S, F, B> {
        #[pin]
        stream: S,
        f: F,
        acc: Option<B>,
    }
}

impl<S, F, B> FoldFuture<S, F, B> {
    pub(super) fn new(stream: S, init: B, f: F) -> Self {
        Self {
            stream,
            f,
            acc: Some(init),
        }
    }
}

impl<S, F, B> Future for FoldFuture<S, F, B>
where
    S: Stream + Sized,
    F: FnMut(B, S::Item) -> B,
{
    type Output = B;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        loop {
            let next = futures_core::ready!(this.stream.as_mut().poll_next(cx));

            match next {
                Some(v) => {
                    let old = this.acc.take().unwrap();
                    let new = (this.f)(old, v);
                    *this.acc = Some(new);
                }
                None => return Poll::Ready(this.acc.take().unwrap()),
            }
        }
    }
}
