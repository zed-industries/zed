use crate::{
    body::{self, Bytes, HttpBody},
    response::{IntoResponse, Response},
    BoxError, Error,
};
use futures_util::{
    ready,
    stream::{self, TryStream},
};
use http::HeaderMap;
use pin_project_lite::pin_project;
use std::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
};
use sync_wrapper::SyncWrapper;

pin_project! {
    /// An [`http_body::Body`] created from a [`Stream`].
    ///
    /// The purpose of this type is to be used in responses. If you want to
    /// extract the request body as a stream consider using
    /// [`BodyStream`](crate::extract::BodyStream).
    ///
    /// # Example
    ///
    /// ```
    /// use axum::{
    ///     Router,
    ///     routing::get,
    ///     body::StreamBody,
    ///     response::IntoResponse,
    /// };
    /// use futures_util::stream::{self, Stream};
    /// use std::io;
    ///
    /// async fn handler() -> StreamBody<impl Stream<Item = io::Result<&'static str>>> {
    ///     let chunks: Vec<io::Result<_>> = vec![
    ///         Ok("Hello,"),
    ///         Ok(" "),
    ///         Ok("world!"),
    ///     ];
    ///     let stream = stream::iter(chunks);
    ///     StreamBody::new(stream)
    /// }
    ///
    /// let app = Router::new().route("/", get(handler));
    /// # async {
    /// # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
    /// # };
    /// ```
    ///
    /// [`Stream`]: futures_util::stream::Stream
    #[must_use]
    pub struct StreamBody<S> {
        #[pin]
        stream: SyncWrapper<S>,
    }
}

impl<S> From<S> for StreamBody<S>
where
    S: TryStream + Send + 'static,
    S::Ok: Into<Bytes>,
    S::Error: Into<BoxError>,
{
    fn from(stream: S) -> Self {
        Self::new(stream)
    }
}

impl<S> StreamBody<S> {
    /// Create a new `StreamBody` from a [`Stream`].
    ///
    /// [`Stream`]: futures_util::stream::Stream
    pub fn new(stream: S) -> Self
    where
        S: TryStream + Send + 'static,
        S::Ok: Into<Bytes>,
        S::Error: Into<BoxError>,
    {
        Self {
            stream: SyncWrapper::new(stream),
        }
    }
}

impl<S> IntoResponse for StreamBody<S>
where
    S: TryStream + Send + 'static,
    S::Ok: Into<Bytes>,
    S::Error: Into<BoxError>,
{
    fn into_response(self) -> Response {
        Response::new(body::boxed(self))
    }
}

impl Default for StreamBody<futures_util::stream::Empty<Result<Bytes, Error>>> {
    fn default() -> Self {
        Self::new(stream::empty())
    }
}

impl<S> fmt::Debug for StreamBody<S> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("StreamBody").finish()
    }
}

impl<S> HttpBody for StreamBody<S>
where
    S: TryStream,
    S::Ok: Into<Bytes>,
    S::Error: Into<BoxError>,
{
    type Data = Bytes;
    type Error = Error;

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        let stream = self.project().stream.get_pin_mut();
        match ready!(stream.try_poll_next(cx)) {
            Some(Ok(chunk)) => Poll::Ready(Some(Ok(chunk.into()))),
            Some(Err(err)) => Poll::Ready(Some(Err(Error::new(err)))),
            None => Poll::Ready(None),
        }
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        _cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        Poll::Ready(Ok(None))
    }
}

#[test]
fn stream_body_traits() {
    use futures_util::stream::Empty;

    type EmptyStream = StreamBody<Empty<Result<Bytes, BoxError>>>;

    crate::test_helpers::assert_send::<EmptyStream>();
    crate::test_helpers::assert_sync::<EmptyStream>();
    crate::test_helpers::assert_unpin::<EmptyStream>();
}
