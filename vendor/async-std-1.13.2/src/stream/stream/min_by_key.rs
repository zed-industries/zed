use core::cmp::Ordering;
use core::future::Future;
use core::pin::Pin;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    #[doc(hidden)]
    #[allow(missing_debug_implementations)]
    pub struct MinByKeyFuture<S, T, K> {
        #[pin]
        stream: S,
        min: Option<(T, T)>,
        key_by: K,
    }
}

impl<S, T, K> MinByKeyFuture<S, T, K> {
    pub(super) fn new(stream: S, key_by: K) -> Self {
        Self {
            stream,
            min: None,
            key_by,
        }
    }
}

impl<S, K> Future for MinByKeyFuture<S, S::Item, K>
where
    S: Stream,
    K: FnMut(&S::Item) -> S::Item,
    S::Item: Ord,
{
    type Output = Option<S::Item>;

    fn poll(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Self::Output> {
        fn key<B, T>(mut f: impl FnMut(&T) -> B) -> impl FnMut(T) -> (B, T) {
            move |x| (f(&x), x)
        }

        let this = self.project();
        let next = futures_core::ready!(this.stream.poll_next(cx));

        match next {
            Some(new) => {
                let (key, value) = key(this.key_by)(new);
                cx.waker().wake_by_ref();

                match this.min.take() {
                    None => *this.min = Some((key, value)),

                    Some(old) => match key.cmp(&old.0) {
                        Ordering::Less => *this.min = Some((key, value)),
                        _ => *this.min = Some(old),
                    },
                }
                Poll::Pending
            }
            None => Poll::Ready(this.min.take().map(|min| min.1)),
        }
    }
}
