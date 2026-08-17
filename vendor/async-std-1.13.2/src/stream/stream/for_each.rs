use core::pin::Pin;
use core::future::Future;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    #[doc(hidden)]
    #[allow(missing_debug_implementations)]
    pub struct ForEachFuture<S, F> {
        #[pin]
        stream: S,
        f: F,
    }
}

impl<S, F> ForEachFuture<S, F> {
    pub(super) fn new(stream: S, f: F) -> Self {
        Self {
            stream,
            f,
        }
    }
}

impl<S, F> Future for ForEachFuture<S, F>
where
    S: Stream + Sized,
    F: FnMut(S::Item),
{
    type Output = ();

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        let mut this = self.project();
        loop {
            let next = futures_core::ready!(this.stream.as_mut().poll_next(cx));

            match next {
                Some(v) => (this.f)(v),
                None => return Poll::Ready(()),
            }
        }
    }
}
