use core::pin::Pin;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    /// A stream to maintain state while polling another stream.
    ///
    /// This `struct` is created by the [`scan`] method on [`Stream`]. See its
    /// documentation for more.
    ///
    /// [`scan`]: trait.Stream.html#method.scan
    /// [`Stream`]: trait.Stream.html
    #[derive(Debug)]
    pub struct Scan<S, St, F> {
        #[pin]
        stream: S,
        state_f: (St, F),
    }
}

impl<S, St, F> Scan<S, St, F> {
    pub(crate) fn new(stream: S, initial_state: St, f: F) -> Self {
        Self {
            stream,
            state_f: (initial_state, f),
        }
    }
}

impl<S, St, F, B> Stream for Scan<S, St, F>
where
    S: Stream,
    F: FnMut(&mut St, S::Item) -> Option<B>,
{
    type Item = B;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<B>> {
        let mut this = self.project();
        let poll_result = this.stream.as_mut().poll_next(cx);
        poll_result.map(|item| {
            item.and_then(|item| {
                let (state, f) = this.state_f;
                f(state, item)
            })
        })
    }
}
