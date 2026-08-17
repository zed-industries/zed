use crate::BoxError;
use futures_core::{ready, Future};
use http_body::Body;
use pin_project_lite::pin_project;
use std::{
    pin::Pin,
    task::{Context, Poll},
    time::Duration,
};
use tokio::time::{sleep, Sleep};

pin_project! {
    /// Middleware that applies a timeout to request and response bodies.
    ///
    /// Wrapper around a [`http_body::Body`] to time out if data is not ready within the specified duration.
    ///
    /// Bodies must produce data at most within the specified timeout.
    /// If the body does not produce a requested data frame within the timeout period, it will return an error.
    ///
    /// # Differences from [`Timeout`][crate::timeout::Timeout]
    ///
    /// [`Timeout`][crate::timeout::Timeout] applies a timeout to the request future, not body.
    /// That timeout is not reset when bytes are handled, whether the request is active or not.
    /// Bodies are handled asynchronously outside of the tower stack's future and thus needs an additional timeout.
    ///
    /// This middleware will return a [`TimeoutError`].
    ///
    /// # Example
    ///
    /// ```
    /// use http::{Request, Response};
    /// use hyper::Body;
    /// use std::time::Duration;
    /// use tower::ServiceBuilder;
    /// use tower_http::timeout::RequestBodyTimeoutLayer;
    ///
    /// async fn handle(_: Request<Body>) -> Result<Response<Body>, std::convert::Infallible> {
    ///     // ...
    ///     # todo!()
    /// }
    ///
    /// # #[tokio::main]
    /// # async fn main() -> Result<(), Box<dyn std::error::Error>> {
    /// let svc = ServiceBuilder::new()
    ///     // Timeout bodies after 30 seconds of inactivity
    ///     .layer(RequestBodyTimeoutLayer::new(Duration::from_secs(30)))
    ///     .service_fn(handle);
    /// # Ok(())
    /// # }
    /// ```
    pub struct TimeoutBody<B> {
        timeout: Duration,
        // In http-body 1.0, `poll_*` will be merged into `poll_frame`.
        // Merge the two `sleep_data` and `sleep_trailers` into one `sleep`.
        // See: https://github.com/tower-rs/tower-http/pull/303#discussion_r1004834958
        #[pin]
        sleep_data: Option<Sleep>,
        #[pin]
        sleep_trailers: Option<Sleep>,
        #[pin]
        body: B,
    }
}

impl<B> TimeoutBody<B> {
    /// Creates a new [`TimeoutBody`].
    pub fn new(timeout: Duration, body: B) -> Self {
        TimeoutBody {
            timeout,
            sleep_data: None,
            sleep_trailers: None,
            body,
        }
    }
}

impl<B> Body for TimeoutBody<B>
where
    B: Body,
    B::Error: Into<BoxError>,
{
    type Data = B::Data;
    type Error = Box<dyn std::error::Error + Send + Sync>;

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        let mut this = self.project();

        // Start the `Sleep` if not active.
        let sleep_pinned = if let Some(some) = this.sleep_data.as_mut().as_pin_mut() {
            some
        } else {
            this.sleep_data.set(Some(sleep(*this.timeout)));
            this.sleep_data.as_mut().as_pin_mut().unwrap()
        };

        // Error if the timeout has expired.
        if let Poll::Ready(()) = sleep_pinned.poll(cx) {
            return Poll::Ready(Some(Err(Box::new(TimeoutError(())))));
        }

        // Check for body data.
        let data = ready!(this.body.poll_data(cx));
        // Some data is ready. Reset the `Sleep`...
        this.sleep_data.set(None);

        Poll::Ready(data.transpose().map_err(Into::into).transpose())
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<http::HeaderMap>, Self::Error>> {
        let mut this = self.project();

        // In http-body 1.0, `poll_*` will be merged into `poll_frame`.
        // Merge the two `sleep_data` and `sleep_trailers` into one `sleep`.
        // See: https://github.com/tower-rs/tower-http/pull/303#discussion_r1004834958

        let sleep_pinned = if let Some(some) = this.sleep_trailers.as_mut().as_pin_mut() {
            some
        } else {
            this.sleep_trailers.set(Some(sleep(*this.timeout)));
            this.sleep_trailers.as_mut().as_pin_mut().unwrap()
        };

        // Error if the timeout has expired.
        if let Poll::Ready(()) = sleep_pinned.poll(cx) {
            return Poll::Ready(Err(Box::new(TimeoutError(()))));
        }

        this.body.poll_trailers(cx).map_err(Into::into)
    }
}

/// Error for [`TimeoutBody`].
#[derive(Debug)]
pub struct TimeoutError(());

impl std::error::Error for TimeoutError {}

impl std::fmt::Display for TimeoutError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "data was not received within the designated timeout")
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    use bytes::Bytes;
    use pin_project_lite::pin_project;
    use std::{error::Error, fmt::Display};

    #[derive(Debug)]
    struct MockError;

    impl Error for MockError {}
    impl Display for MockError {
        fn fmt(&self, _f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            todo!()
        }
    }

    pin_project! {
        struct MockBody {
            #[pin]
            sleep: Sleep
        }
    }

    impl Body for MockBody {
        type Data = Bytes;
        type Error = MockError;

        fn poll_data(
            self: Pin<&mut Self>,
            cx: &mut Context<'_>,
        ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
            let this = self.project();
            this.sleep.poll(cx).map(|_| Some(Ok(vec![].into())))
        }

        fn poll_trailers(
            self: Pin<&mut Self>,
            _cx: &mut Context<'_>,
        ) -> Poll<Result<Option<http::HeaderMap>, Self::Error>> {
            todo!()
        }
    }

    #[tokio::test]
    async fn test_body_available_within_timeout() {
        let mock_sleep = Duration::from_secs(1);
        let timeout_sleep = Duration::from_secs(2);

        let mock_body = MockBody {
            sleep: sleep(mock_sleep),
        };
        let timeout_body = TimeoutBody::new(timeout_sleep, mock_body);

        assert!(timeout_body.boxed().data().await.unwrap().is_ok());
    }

    #[tokio::test]
    async fn test_body_unavailable_within_timeout_error() {
        let mock_sleep = Duration::from_secs(2);
        let timeout_sleep = Duration::from_secs(1);

        let mock_body = MockBody {
            sleep: sleep(mock_sleep),
        };
        let timeout_body = TimeoutBody::new(timeout_sleep, mock_body);

        assert!(timeout_body.boxed().data().await.unwrap().is_err());
    }
}
