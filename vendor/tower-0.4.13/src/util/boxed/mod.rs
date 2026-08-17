//! Trait object [`Service`] instances
//!
//! Dynamically dispatched [`Service`] objects allow for erasing the underlying
//! [`Service`] type and using the `Service` instances as opaque handles. This can
//! be useful when the service instance cannot be explicitly named for whatever
//! reason.
//!
//! There are two variants of service objects. [`BoxService`] requires both the
//! service and the response future to be [`Send`]. These values can move freely
//! across threads. [`UnsyncBoxService`] requires both the service and the
//! response future to remain on the current thread. This is useful for
//! representing services that are backed by [`Rc`] or other non-[`Send`] types.
//!
//! # Examples
//!
//! ```
//! use futures_util::future::ready;
//! # use tower_service::Service;
//! # use tower::util::{BoxService, service_fn};
//! // Respond to requests using a closure, but closures cannot be named...
//! # pub fn main() {
//! let svc = service_fn(|mut request: String| {
//!     request.push_str(" response");
//!     ready(Ok(request))
//! });
//!
//! let service: BoxService<String, String, ()> = BoxService::new(svc);
//! # drop(service);
//! }
//! ```
//!
//! [`Service`]: crate::Service
//! [`Rc`]: std::rc::Rc

mod layer;
mod sync;
mod unsync;

#[allow(unreachable_pub)] // https://github.com/rust-lang/rust/issues/57411
pub use self::{layer::BoxLayer, sync::BoxService, unsync::UnsyncBoxService};
