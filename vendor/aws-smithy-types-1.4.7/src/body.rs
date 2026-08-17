/*
 * Copyright Amazon.com, Inc. or its affiliates. All Rights Reserved.
 * SPDX-License-Identifier: Apache-2.0
 */

//! Types for representing the body of an HTTP request or response

use bytes::Bytes;
use pin_project_lite::pin_project;
use std::collections::VecDeque;
use std::error::Error as StdError;
use std::fmt::{self, Debug, Formatter};
use std::future::poll_fn;
use std::pin::Pin;
use std::sync::Arc;
use std::task::{Context, Poll};

/// This module is named after the `http-body` version number since we anticipate
/// needing to provide equivalent functionality for 1.x of that crate in the future.
/// The name has a suffix `_x` to avoid name collision with a third-party `http-body-0-4`.
#[cfg(feature = "http-body-0-4-x")]
pub mod http_body_0_4_x;
#[cfg(feature = "http-body-1-x")]
pub mod http_body_1_x;

/// A generic, boxed error that's `Send` and `Sync`
pub type Error = Box<dyn StdError + Send + Sync>;

pin_project! {
    /// SdkBody type
    ///
    /// This is the Body used for dispatching all HTTP Requests.
    /// For handling responses, the type of the body will be controlled
    /// by the HTTP stack.
    ///
    pub struct SdkBody {
        #[pin]
        inner: Inner,
        // An optional function to recreate the inner body
        //
        // In the event of retry, this function will be called to generate a new body. See
        // [`try_clone()`](SdkBody::try_clone)
        rebuild: Option<Arc<dyn (Fn() -> Inner) + Send + Sync>>,
        bytes_contents: Option<Bytes>,
        // Here the optionality indicates whether we have started streaming trailers, and the
        // VecDeque serves as a buffer for trailer frames that are polled by poll_next instead
        // of poll_next_trailers
        trailers: Option<VecDeque<http_1x::HeaderMap>>,
    }
}

impl Debug for SdkBody {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        f.debug_struct("SdkBody")
            .field("inner", &self.inner)
            .field("retryable", &self.rebuild.is_some())
            .finish()
    }
}

/// A boxed generic HTTP body that, when consumed, will result in [`Bytes`] or an [`Error`].
#[allow(dead_code)]
enum BoxBody {
    // This is enabled by the **dependency**, not the feature. This allows us to construct it
    // whenever we have the dependency and keep the APIs private
    #[cfg(any(
        feature = "http-body-0-4-x",
        feature = "http-body-1-x",
        feature = "rt-tokio"
    ))]
    // will be dead code with `--no-default-features --features rt-tokio`
    HttpBody04(#[allow(dead_code)] http_body_0_4::combinators::BoxBody<Bytes, Error>),

    #[cfg(feature = "http-body-1-x")]
    HttpBody1(#[allow(dead_code)] http_body_util::combinators::BoxBody<Bytes, Error>),
}

pin_project! {
    #[project = InnerProj]
    enum Inner {
        // An in-memory body
        Once {
            inner: Option<Bytes>
        },
        // A streaming body
        Dyn {
            #[pin]
            inner: BoxBody,
        },

        /// When a streaming body is transferred out to a stream parser, the body is replaced with
        /// `Taken`. This will return an Error when polled. Attempting to read data out of a `Taken`
        /// Body is a bug.
        Taken,
    }
}

impl Debug for Inner {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        match &self {
            Inner::Once { inner: once } => f.debug_tuple("Once").field(once).finish(),
            Inner::Dyn { .. } => write!(f, "BoxBody"),
            Inner::Taken => f.debug_tuple("Taken").finish(),
        }
    }
}

impl SdkBody {
    /// Construct an explicitly retryable SDK body
    ///
    /// _Note: This is probably not what you want_
    ///
    /// All bodies constructed from in-memory data (`String`, `Vec<u8>`, `Bytes`, etc.) will be
    /// retryable out of the box. If you want to read data from a file, you should use
    /// [`ByteStream::from_path`](crate::byte_stream::ByteStream::from_path). This function
    /// is only necessary when you need to enable retries for your own streaming container.
    pub fn retryable(f: impl Fn() -> SdkBody + Send + Sync + 'static) -> Self {
        let initial = f();
        SdkBody {
            inner: initial.inner,
            rebuild: Some(Arc::new(move || f().inner)),
            bytes_contents: initial.bytes_contents,
            trailers: None,
        }
    }

    /// When an SdkBody is read, the inner data must be consumed. In order to do this, the SdkBody
    /// is swapped with a "taken" body. This "taken" body cannot be read but aids in debugging.
    pub fn taken() -> Self {
        Self {
            inner: Inner::Taken,
            rebuild: None,
            bytes_contents: None,
            trailers: None,
        }
    }

    /// Create an empty SdkBody for requests and responses that don't transfer any data in the body.
    pub fn empty() -> Self {
        Self {
            inner: Inner::Once { inner: None },
            rebuild: Some(Arc::new(|| Inner::Once { inner: None })),
            bytes_contents: Some(Bytes::new()),
            trailers: None,
        }
    }

    pub(crate) async fn next(&mut self) -> Option<Result<Bytes, Error>> {
        let mut me = Pin::new(self);
        poll_fn(|cx| me.as_mut().poll_next(cx)).await
    }

    pub(crate) fn poll_next(
        self: Pin<&mut Self>,
        #[allow(unused)] cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Bytes, Error>>> {
        let this = self.project();
        match this.inner.project() {
            InnerProj::Once { ref mut inner } => {
                let data = inner.take();
                match data {
                    Some(bytes) if bytes.is_empty() => Poll::Ready(None),
                    Some(bytes) => Poll::Ready(Some(Ok(bytes))),
                    None => Poll::Ready(None),
                }
            }
            InnerProj::Dyn { inner: body } => match body.get_mut() {
                #[cfg(feature = "http-body-0-4-x")]
                BoxBody::HttpBody04(box_body) => {
                    use http_body_0_4::Body;
                    Pin::new(box_body).poll_data(cx)
                }
                #[cfg(feature = "http-body-1-x")]
                BoxBody::HttpBody1(box_body) => {
                    // If this is polled after the trailers have been cached end early
                    if this.trailers.is_some() {
                        return Poll::Ready(None);
                    }
                    use http_body_1_0::Body;
                    let maybe_data = Pin::new(box_body).poll_frame(cx);
                    match maybe_data {
                        Poll::Ready(Some(Ok(frame))) => {
                            if frame.is_data() {
                                Poll::Ready(Some(Ok(frame
                                    .into_data()
                                    .expect("Confirmed data frame"))))
                            } else if frame.is_trailers() {
                                let trailers =
                                    frame.into_trailers().expect("Confirmed trailer frame");
                                // Buffer the trailers for the trailer poll
                                this.trailers.get_or_insert_with(VecDeque::new).push_back(trailers);

                                Poll::Ready(None)
                            } else {
                                unreachable!("Frame must be either data or trailers");
                            }
                        }
                        Poll::Ready(Some(Err(err))) => Poll::Ready(Some(Err(err))),
                        Poll::Ready(None) => Poll::Ready(None),
                        Poll::Pending => Poll::Pending,
                    }
                }
                #[allow(unreachable_patterns)]
                _ => unreachable!(
                    "enabling `http-body-0-4-x` or `http-body-1-x` is the only way to create the `Dyn` variant"
                ),
            },
            InnerProj::Taken => {
                Poll::Ready(Some(Err("A `Taken` body should never be polled".into())))
            }
        }
    }

    #[allow(dead_code)]
    #[cfg(any(
        feature = "http-body-0-4-x",
        feature = "http-body-1-x",
        feature = "rt-tokio"
    ))]
    pub(crate) fn from_body_0_4_internal<T, E>(body: T) -> Self
    where
        T: http_body_0_4::Body<Data = Bytes, Error = E> + Send + Sync + 'static,
        E: Into<Error> + 'static,
    {
        Self {
            inner: Inner::Dyn {
                inner: BoxBody::HttpBody04(http_body_0_4::combinators::BoxBody::new(
                    body.map_err(Into::into),
                )),
            },
            rebuild: None,
            bytes_contents: None,
            trailers: None,
        }
    }

    #[cfg(feature = "http-body-1-x")]
    pub(crate) fn from_body_1_x_internal<T, E>(body: T) -> Self
    where
        T: http_body_1_0::Body<Data = Bytes, Error = E> + Send + Sync + 'static,
        E: Into<Error> + 'static,
    {
        use http_body_util::BodyExt;
        Self {
            inner: Inner::Dyn {
                inner: BoxBody::HttpBody1(http_body_util::combinators::BoxBody::new(
                    body.map_err(Into::into),
                )),
            },
            rebuild: None,
            bytes_contents: None,
            trailers: None,
        }
    }

    #[cfg(any(feature = "http-body-0-4-x", feature = "http-body-1-x",))]
    pub(crate) fn poll_next_trailers(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<http_1x::HeaderMap<http_1x::HeaderValue>>, Error>> {
        // Three cases that matter here:
        // 1) Both http-body features disabled, doesn't matter because this func won't compile
        // 2) http-body-0-4-x enabled but 1-x disabled, we use the http_body_0_4_x conversion
        // 3) http-body-1-x enabled (and 0-4-x is enabled or disabled), we use the 1-x conversion
        // as our default whenever it is available
        #[cfg(all(feature = "http-body-0-4-x", not(feature = "http-body-1-x")))]
        use crate::body::http_body_0_4_x::convert_headers_0x_1x;
        #[cfg(feature = "http-body-1-x")]
        use crate::body::http_body_1_x::convert_headers_0x_1x;

        let this = self.project();
        match this.inner.project() {
            InnerProj::Once { .. } => Poll::Ready(Ok(None)),
            InnerProj::Dyn { inner } => match inner.get_mut() {
                BoxBody::HttpBody04(box_body) => {
                    use http_body_0_4::Body;
                    let polled = Pin::new(box_body).poll_trailers(cx);

                    match polled {
                        Poll::Ready(Ok(maybe_trailers)) => {
                            let http_1x_trailers = maybe_trailers.map(convert_headers_0x_1x);
                            Poll::Ready(Ok(http_1x_trailers))
                        }
                        Poll::Ready(Err(err)) => Poll::Ready(Err(err)),
                        Poll::Pending => Poll::Pending,
                    }
                }
                #[cfg(feature = "http-body-1-x")]
                BoxBody::HttpBody1(box_body) => {
                    use http_body_1_0::Body;
                    // Return the cached trailers without polling
                    if let Some(trailer_buf) = this.trailers {
                        if let Some(next_trailer) = trailer_buf.pop_front() {
                            return Poll::Ready(Ok(Some(next_trailer)));
                        }
                    }

                    let polled = Pin::new(box_body).poll_frame(cx);
                    match polled {
                        Poll::Ready(Some(Ok(maybe_trailers))) => {
                            if maybe_trailers.is_data() {
                                Poll::Ready(Err("Trailers polled while body still has data".into()))
                            } else {
                                let trailers = maybe_trailers
                                    .into_trailers()
                                    .expect("Frame must be trailers because it is not data");
                                Poll::Ready(Ok(Some(trailers)))
                            }
                        }
                        Poll::Ready(None) => Poll::Ready(Ok(None)),
                        Poll::Ready(Some(Err(err))) => Poll::Ready(Err(err)),
                        Poll::Pending => Poll::Pending,
                    }
                }
            },
            InnerProj::Taken => Poll::Ready(Err(
                "A `Taken` body should never be polled for trailers".into(),
            )),
        }
    }

    /// If possible, return a reference to this body as `&[u8]`
    ///
    /// If this SdkBody is NOT streaming, this will return the byte slab
    /// If this SdkBody is streaming, this will return `None`
    pub fn bytes(&self) -> Option<&[u8]> {
        match &self.bytes_contents {
            Some(b) => Some(b),
            None => None,
        }
    }

    /// Attempt to clone this SdkBody. This will fail if the inner data is not cloneable, such as when
    /// it is a single-use stream that can't be recreated.
    pub fn try_clone(&self) -> Option<Self> {
        self.rebuild.as_ref().map(|rebuild| {
            let next = rebuild();
            Self {
                inner: next,
                rebuild: self.rebuild.clone(),
                bytes_contents: self.bytes_contents.clone(),
                trailers: self.trailers.clone(),
            }
        })
    }

    /// Return `true` if this SdkBody is streaming, `false` if it is in-memory.
    pub fn is_streaming(&self) -> bool {
        matches!(self.inner, Inner::Dyn { .. })
    }

    /// Return the length, in bytes, of this SdkBody. If this returns `None`, then the body does not
    /// have a known length.
    pub fn content_length(&self) -> Option<u64> {
        match self.bounds_on_remaining_length() {
            (lo, Some(hi)) if lo == hi => Some(lo),
            _ => None,
        }
    }

    #[allow(dead_code)] // used by a feature-gated `http-body`'s trait method
    pub(crate) fn is_end_stream(&self) -> bool {
        match &self.inner {
            Inner::Once { inner: None } => true,
            Inner::Once { inner: Some(bytes) } => bytes.is_empty(),
            Inner::Dyn { inner: box_body } => match box_body {
                #[cfg(feature = "http-body-0-4-x")]
                BoxBody::HttpBody04(box_body) => {
                    use http_body_0_4::Body;
                    box_body.is_end_stream()
                }
                #[cfg(feature = "http-body-1-x")]
                BoxBody::HttpBody1(box_body) => {
                    use http_body_1_0::Body;
                    box_body.is_end_stream()
                }
                #[allow(unreachable_patterns)]
                _ => unreachable!(
                    "enabling `http-body-0-4-x` or `http-body-1-x` is the only way to create the `Dyn` variant"
                ),
            },
            Inner::Taken => true,
        }
    }

    pub(crate) fn bounds_on_remaining_length(&self) -> (u64, Option<u64>) {
        match &self.inner {
            Inner::Once { inner: None } => (0, Some(0)),
            Inner::Once { inner: Some(bytes) } => {
                let len = bytes.len() as u64;
                (len, Some(len))
            }
            Inner::Dyn { inner: box_body } => match box_body {
                #[cfg(feature = "http-body-0-4-x")]
                BoxBody::HttpBody04(box_body) => {
                    use http_body_0_4::Body;
                    let hint = box_body.size_hint();
                    (hint.lower(), hint.upper())
                }
                #[cfg(feature = "http-body-1-x")]
                BoxBody::HttpBody1(box_body) => {
                    use http_body_1_0::Body;
                    let hint = box_body.size_hint();
                    (hint.lower(), hint.upper())
                }
                #[allow(unreachable_patterns)]
                _ => unreachable!(
                    "enabling `http-body-0-4-x` or `http-body-1-x` is the only way to create the `Dyn` variant"
                ),
            },
            Inner::Taken => (0, Some(0)),
        }
    }

    /// Given a function to modify an `SdkBody`, run that function against this `SdkBody` before
    /// returning the result.
    pub fn map(self, f: impl Fn(SdkBody) -> SdkBody + Sync + Send + 'static) -> SdkBody {
        if self.rebuild.is_some() {
            SdkBody::retryable(move || f(self.try_clone().unwrap()))
        } else {
            f(self)
        }
    }

    /// Update this `SdkBody` with `map`. **This function MUST NOT alter the data of the body.**
    ///
    /// This function is useful for adding metadata like progress tracking to an [`SdkBody`] that
    /// does not alter the actual byte data. If your mapper alters the contents of the body, use [`SdkBody::map`]
    /// instead.
    pub fn map_preserve_contents(
        self,
        f: impl Fn(SdkBody) -> SdkBody + Sync + Send + 'static,
    ) -> SdkBody {
        let contents = self.bytes_contents.clone();
        let mut out = if self.rebuild.is_some() {
            SdkBody::retryable(move || f(self.try_clone().unwrap()))
        } else {
            f(self)
        };
        out.bytes_contents = contents;
        out
    }
}

impl From<&str> for SdkBody {
    fn from(s: &str) -> Self {
        Self::from(s.as_bytes())
    }
}

impl From<Bytes> for SdkBody {
    fn from(bytes: Bytes) -> Self {
        let b = bytes.clone();
        SdkBody {
            inner: Inner::Once {
                inner: Some(bytes.clone()),
            },
            rebuild: Some(Arc::new(move || Inner::Once {
                inner: Some(bytes.clone()),
            })),
            bytes_contents: Some(b),
            trailers: None,
        }
    }
}

impl From<Vec<u8>> for SdkBody {
    fn from(data: Vec<u8>) -> Self {
        Self::from(Bytes::from(data))
    }
}

impl From<String> for SdkBody {
    fn from(s: String) -> Self {
        Self::from(s.into_bytes())
    }
}

impl From<&[u8]> for SdkBody {
    fn from(data: &[u8]) -> Self {
        Self::from(Bytes::copy_from_slice(data))
    }
}

#[cfg(test)]
mod test {
    use crate::body::SdkBody;
    use std::pin::Pin;

    #[test]
    fn valid_size_hint() {
        assert_eq!(SdkBody::from("hello").content_length(), Some(5));
        assert_eq!(SdkBody::from("").content_length(), Some(0));
    }

    #[allow(clippy::bool_assert_comparison)]
    #[test]
    fn valid_eos() {
        assert_eq!(SdkBody::from("hello").is_end_stream(), false);
        assert_eq!(SdkBody::from("").is_end_stream(), true);
    }

    #[tokio::test]
    async fn http_body_consumes_data() {
        let mut body = SdkBody::from("hello!");
        let mut body = Pin::new(&mut body);
        assert!(!body.is_end_stream());
        let data = body.next().await;
        assert!(data.is_some());
        let data = body.next().await;
        assert!(data.is_none());
        assert!(body.is_end_stream());
    }

    #[tokio::test]
    async fn empty_body_returns_none() {
        // Its important to avoid sending empty chunks of data to avoid H2 data frame problems
        let mut body = SdkBody::from("");
        let mut body = Pin::new(&mut body);
        let data = body.next().await;
        assert!(data.is_none());
    }

    #[test]
    fn sdkbody_debug_once() {
        let body = SdkBody::from("123");
        assert!(format!("{body:?}").contains("Once"));
    }

    #[test]
    fn sdk_body_is_send() {
        fn is_send<T: Send>() {}
        is_send::<SdkBody>()
    }
}
