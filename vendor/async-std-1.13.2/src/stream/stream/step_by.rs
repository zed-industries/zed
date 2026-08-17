use core::pin::Pin;

use pin_project_lite::pin_project;

use crate::stream::Stream;
use crate::task::{Context, Poll};

pin_project! {
    /// A stream that steps a given amount of elements of another stream.
    ///
    /// This `struct` is created by the [`step_by`] method on [`Stream`]. See its
    /// documentation for more.
    ///
    /// [`step_by`]: trait.Stream.html#method.step_by
    /// [`Stream`]: trait.Stream.html
    #[derive(Debug)]
    pub struct StepBy<S> {
        #[pin]
        stream: S,
        step: usize,
        i: usize,
    }
}

impl<S> StepBy<S> {
    pub(crate) fn new(stream: S, step: usize) -> Self {
        Self {
            stream,
            step: step.checked_sub(1).unwrap(),
            i: 0,
        }
    }
}

impl<S> Stream for StepBy<S>
where
    S: Stream,
{
    type Item = S::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        loop {
            let next = futures_core::ready!(this.stream.as_mut().poll_next(cx));

            match next {
                Some(v) => match this.i {
                    0 => {
                        *this.i = *this.step;
                        return Poll::Ready(Some(v));
                    }
                    _ => *this.i -= 1,
                },
                None => return Poll::Ready(None),
            }
        }
    }
}
