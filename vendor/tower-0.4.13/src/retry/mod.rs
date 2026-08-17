//! Middleware for retrying "failed" requests.

pub mod budget;
pub mod future;
mod layer;
mod policy;

pub use self::layer::RetryLayer;
pub use self::policy::Policy;

use self::future::ResponseFuture;
use pin_project_lite::pin_project;
use std::task::{Context, Poll};
use tower_service::Service;

pin_project! {
    /// Configure retrying requests of "failed" responses.
    ///
    /// A [`Policy`] classifies what is a "failed" response.
    #[derive(Clone, Debug)]
    pub struct Retry<P, S> {
        #[pin]
        policy: P,
        service: S,
    }
}

// ===== impl Retry =====

impl<P, S> Retry<P, S> {
    /// Retry the inner service depending on this [`Policy`].
    pub fn new(policy: P, service: S) -> Self {
        Retry { policy, service }
    }

    /// Get a reference to the inner service
    pub fn get_ref(&self) -> &S {
        &self.service
    }

    /// Get a mutable reference to the inner service
    pub fn get_mut(&mut self) -> &mut S {
        &mut self.service
    }

    /// Consume `self`, returning the inner service
    pub fn into_inner(self) -> S {
        self.service
    }
}

impl<P, S, Request> Service<Request> for Retry<P, S>
where
    P: Policy<Request, S::Response, S::Error> + Clone,
    S: Service<Request> + Clone,
{
    type Response = S::Response;
    type Error = S::Error;
    type Future = ResponseFuture<P, S, Request>;

    fn poll_ready(&mut self, cx: &mut Context<'_>) -> Poll<Result<(), Self::Error>> {
        // NOTE: the Future::poll impl for ResponseFuture assumes that Retry::poll_ready is
        // equivalent to Ready.service.poll_ready. If this ever changes, that code must be updated
        // as well.
        self.service.poll_ready(cx)
    }

    fn call(&mut self, request: Request) -> Self::Future {
        let cloned = self.policy.clone_request(&request);
        let future = self.service.call(request);

        ResponseFuture::new(cloned, self.clone(), future)
    }
}
