use futures_core::{ready, Stream};
use pin_project_lite::pin_project;
use std::{
    future::Future,
    pin::Pin,
    task::{Context, Poll},
};
use tower_service::Service;

pin_project! {
    /// The [`Future`] returned by the [`ServiceExt::call_all`] combinator.
    #[derive(Debug)]
    pub(crate) struct CallAll<Svc, S, Q> {
        service: Option<Svc>,
        #[pin]
        stream: S,
        queue: Q,
        eof: bool,
    }
}

pub(crate) trait Drive<F: Future> {
    fn is_empty(&self) -> bool;

    fn push(&mut self, future: F);

    fn poll(&mut self, cx: &mut Context<'_>) -> Poll<Option<F::Output>>;
}

impl<Svc, S, Q> CallAll<Svc, S, Q>
where
    Svc: Service<S::Item>,
    Svc::Error: Into<crate::BoxError>,
    S: Stream,
    Q: Drive<Svc::Future>,
{
    pub(crate) fn new(service: Svc, stream: S, queue: Q) -> CallAll<Svc, S, Q> {
        CallAll {
            service: Some(service),
            stream,
            queue,
            eof: false,
        }
    }

    /// Extract the wrapped [`Service`].
    pub(crate) fn into_inner(mut self) -> Svc {
        self.service.take().expect("Service already taken")
    }

    /// Extract the wrapped [`Service`].
    pub(crate) fn take_service(self: Pin<&mut Self>) -> Svc {
        self.project()
            .service
            .take()
            .expect("Service already taken")
    }

    pub(crate) fn unordered(mut self) -> super::CallAllUnordered<Svc, S> {
        assert!(self.queue.is_empty() && !self.eof);

        super::CallAllUnordered::new(self.service.take().unwrap(), self.stream)
    }
}

impl<Svc, S, Q> Stream for CallAll<Svc, S, Q>
where
    Svc: Service<S::Item>,
    Svc::Error: Into<crate::BoxError>,
    S: Stream,
    Q: Drive<Svc::Future>,
{
    type Item = Result<Svc::Response, crate::BoxError>;

    fn poll_next(self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let mut this = self.project();

        loop {
            // First, see if we have any responses to yield
            if let Poll::Ready(r) = this.queue.poll(cx) {
                if let Some(rsp) = r.transpose().map_err(Into::into)? {
                    return Poll::Ready(Some(Ok(rsp)));
                }
            }

            // If there are no more requests coming, check if we're done
            if *this.eof {
                if this.queue.is_empty() {
                    return Poll::Ready(None);
                } else {
                    return Poll::Pending;
                }
            }

            // Then, see that the service is ready for another request
            let svc = this
                .service
                .as_mut()
                .expect("Using CallAll after extracing inner Service");
            ready!(svc.poll_ready(cx)).map_err(Into::into)?;

            // If it is, gather the next request (if there is one), or return `Pending` if the
            // stream is not ready.
            // TODO: We probably want to "release" the slot we reserved in Svc if the
            // stream returns `Pending`. It may be a while until we get around to actually
            // using it.
            match ready!(this.stream.as_mut().poll_next(cx)) {
                Some(req) => {
                    this.queue.push(svc.call(req));
                }
                None => {
                    // We're all done once any outstanding requests have completed
                    *this.eof = true;
                }
            }
        }
    }
}
