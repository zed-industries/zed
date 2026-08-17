use super::{OnBodyChunk, OnEos, OnFailure};
use crate::classify::ClassifyEos;
use futures_core::ready;
use http::HeaderMap;
use http_body::Body;
use pin_project_lite::pin_project;
use std::{
    fmt,
    pin::Pin,
    task::{Context, Poll},
    time::Instant,
};
use tracing::Span;

pin_project! {
    /// Response body for [`Trace`].
    ///
    /// [`Trace`]: super::Trace
    pub struct ResponseBody<B, C, OnBodyChunk, OnEos, OnFailure> {
        #[pin]
        pub(crate) inner: B,
        pub(crate) classify_eos: Option<C>,
        pub(crate) on_eos: Option<(OnEos, Instant)>,
        pub(crate) on_body_chunk: OnBodyChunk,
        pub(crate) on_failure: Option<OnFailure>,
        pub(crate) start: Instant,
        pub(crate) span: Span,
    }
}

impl<B, C, OnBodyChunkT, OnEosT, OnFailureT> Body
    for ResponseBody<B, C, OnBodyChunkT, OnEosT, OnFailureT>
where
    B: Body,
    B::Error: fmt::Display + 'static,
    C: ClassifyEos,
    OnEosT: OnEos,
    OnBodyChunkT: OnBodyChunk<B::Data>,
    OnFailureT: OnFailure<C::FailureClass>,
{
    type Data = B::Data;
    type Error = B::Error;

    fn poll_data(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Option<Result<Self::Data, Self::Error>>> {
        let this = self.project();
        let _guard = this.span.enter();

        let result = if let Some(result) = ready!(this.inner.poll_data(cx)) {
            result
        } else {
            return Poll::Ready(None);
        };

        let latency = this.start.elapsed();
        *this.start = Instant::now();

        match &result {
            Ok(chunk) => {
                this.on_body_chunk.on_body_chunk(chunk, latency, this.span);
            }
            Err(err) => {
                if let Some((classify_eos, mut on_failure)) =
                    this.classify_eos.take().zip(this.on_failure.take())
                {
                    let failure_class = classify_eos.classify_error(err);
                    on_failure.on_failure(failure_class, latency, this.span);
                }
            }
        }

        Poll::Ready(Some(result))
    }

    fn poll_trailers(
        self: Pin<&mut Self>,
        cx: &mut Context<'_>,
    ) -> Poll<Result<Option<HeaderMap>, Self::Error>> {
        let this = self.project();
        let _guard = this.span.enter();
        let result = ready!(this.inner.poll_trailers(cx));

        let latency = this.start.elapsed();

        if let Some((classify_eos, mut on_failure)) =
            this.classify_eos.take().zip(this.on_failure.take())
        {
            match &result {
                Ok(trailers) => {
                    if let Err(failure_class) = classify_eos.classify_eos(trailers.as_ref()) {
                        on_failure.on_failure(failure_class, latency, this.span);
                    }

                    if let Some((on_eos, stream_start)) = this.on_eos.take() {
                        on_eos.on_eos(trailers.as_ref(), stream_start.elapsed(), this.span);
                    }
                }
                Err(err) => {
                    let failure_class = classify_eos.classify_error(err);
                    on_failure.on_failure(failure_class, latency, this.span);
                }
            }
        }

        Poll::Ready(result)
    }

    fn is_end_stream(&self) -> bool {
        self.inner.is_end_stream()
    }

    fn size_hint(&self) -> http_body::SizeHint {
        self.inner.size_hint()
    }
}

impl<B: Default, C: Default, OnBodyChunk: Default, OnEos: Default, OnFailure: Default> Default
    for ResponseBody<B, C, OnBodyChunk, OnEos, OnFailure>
{
    fn default() -> Self {
        Self {
            inner: Default::default(),
            classify_eos: Default::default(),
            on_eos: Default::default(),
            on_body_chunk: Default::default(),
            on_failure: Default::default(),
            start: Instant::now(),
            span: Span::current(),
        }
    }
}
