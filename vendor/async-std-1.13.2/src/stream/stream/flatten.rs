use core::fmt;
use core::pin::Pin;

use pin_project_lite::pin_project;

use crate::stream::{IntoStream, Stream};
use crate::task::{Context, Poll};

pin_project! {
    /// A stream that flattens one level of nesting in an stream of things that can be turned into
    /// streams.
    ///
    /// This `struct` is created by the [`flatten`] method on [`Stream`]. See its
    /// documentation for more.
    ///
    /// [`flatten`]: trait.Stream.html#method.flatten
    /// [`Stream`]: trait.Stream.html
    pub struct Flatten<S>
    where
        S: Stream,
        S::Item: IntoStream,
    {
        #[pin]
        stream: S,
        #[pin]
        inner_stream: Option<<S::Item as IntoStream>::IntoStream>,
    }
}

impl<S> Flatten<S>
where
    S: Stream,
    S::Item: IntoStream,
{
    pub(super) fn new(stream: S) -> Self {
        Self {
            stream,
            inner_stream: None,
        }
    }
}

impl<S, U> Stream for Flatten<S>
where
    S: Stream,
    S::Item: IntoStream<IntoStream = U, Item = U::Item>,
    U: Stream,
{
    type Item = U::Item;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();
        loop {
            if let Some(inner) = this.inner_stream.as_mut().as_pin_mut() {
                match futures_core::ready!(inner.poll_next(cx)) {
                    item @ Some(_) => return Poll::Ready(item),
                    None => this.inner_stream.set(None),
                }
            }

            match futures_core::ready!(this.stream.as_mut().poll_next(cx)) {
                inner @ Some(_) => this.inner_stream.set(inner.map(IntoStream::into_stream)),
                None => return Poll::Ready(None),
            }
        }
    }
}

impl<S, U> fmt::Debug for Flatten<S>
where
    S: fmt::Debug + Stream,
    S::Item: IntoStream<IntoStream = U, Item = U::Item>,
    U: fmt::Debug + Stream,
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("Flatten")
            .field("inner", &self.stream)
            .finish()
    }
}
