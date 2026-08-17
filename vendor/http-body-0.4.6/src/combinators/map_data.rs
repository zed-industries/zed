use crate::Body;
use bytes::Buf;
use pin_project_lite::pin_project;
use std::{
    any::type_name,
    fmt,
    pin::Pin,
    task::{Context, Poll},
};

pin_project! {
    /// Body returned by the [`map_data`] combinator.
    ///
    /// [`map_data`]: crate::util::BodyExt::map_data
    #[derive(Clone, Copy)]
    pub struct MapData<B, F> {
        #[pin]
        inner: B,
        f: F
    }
}

impl<B, F> MapData<B, F> {
    #[inline]
    pub(crate) fn new(body: B, f: F) -> Self {
        Self { inner: body, f }
    }

    /// Get a reference to the inner body
    pub fn get_ref(&self) -> &B {
        &self.inner
    }

    /// Get a mutable reference to the inner body
    pub fn get_mut(&mut self) -> &mut B {
        &mut self.inner
    }

    /// Get a pinned mutable reference to the inner body
    pub fn get_pin_mut(self: Pin<&mut Self>) -> Pin<&mut B> {
        self.project().inner
    }

    /// Consume `self`, returning the inner body
    pub fn into_inner(self) -> B {
        self.inner
    }
}

impl<B, F, B2> Body for MapData<B, F>
where
    B: Body,
    F: FnMut(B::Data) -> B2,
    B2: Buf,
{
    type Data = B2;
    type Error = B::Error;

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        let this = self.project();
        match this.inner.poll_data(cx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Ok(data))) => Poll::Ready(Some(Ok((this.f)(data)))),
            Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err(err))),
        }
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<http::HeaderMap>, Self::Error>> {
        self.project().inner.poll_trailers(cx)
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }
}

impl<B, F> fmt::Debug for MapData<B, F>
where
    B: fmt::Debug,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("MapData")
            .field("inner", &self.inner)
            .field("f", &type_name::<F>())
            .finish()
    }
}
