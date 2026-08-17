use core::future::Future;
use core::pin::Pin;
use core::task::{Context, Poll};

use pin_project_lite::pin_project;

use crate::stream::DoubleEndedStream;

pin_project! {
    #[doc(hidden)]
    #[allow(missing_debug_implementations)]
    pub struct RFoldFuture<S, F, B> {
        #[pin]
        stream: S,
        f: F,
        acc: Option<B>,
    }
}

impl<S, F, B> RFoldFuture<S, F, B> {
    pub(super) fn new(stream: S, init: B, f: F) -> Self {
        RFoldFuture {
            stream,
            f,
            acc: Some(init),
        }
    }
}

impl<S, F, B> Future for RFoldFuture<S, F, B>
where
    S: DoubleEndedStream + Sized,
    F: FnMut(B, S::Item) -> B,
{
    type Output = B;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        loop {
            let next = futures_core::ready!(this.stream.as_mut().poll_next_back(cx));

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
