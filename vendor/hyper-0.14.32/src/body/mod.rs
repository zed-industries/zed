//! Streaming bodies for Requests and Responses
//!
//! For both [Clients](crate::client) and [Servers](crate::server), requests and
//! responses use streaming bodies, instead of complete buffering. This
//! allows applications to not use memory they don't need, and allows exerting
//! back-pressure on connections by only reading when asked.
//!
//! There are two pieces to this in hyper:
//!
//! - **The [`HttpBody`](HttpBody) trait** describes all possible bodies.
//!   hyper allows any body type that implements `HttpBody`, allowing
//!   applications to have fine-grained control over their streaming.
//! - **The [`Body`](Body) concrete type**, which is an implementation of
//!   `HttpBody`, and returned by hyper as a "receive stream" (so, for server
//!   requests and client responses). It is also a decent default implementation
//!   if you don't have very custom needs of your send streams.

pub use bytes::{Buf, Bytes};
pub use http_body::Body as HttpBody;
pub use http_body::SizeHint;

#[cfg_attr(feature = "deprecated", allow(deprecated))]
pub use self::aggregate::aggregate;
pub use self::body::{Body, Sender};
pub(crate) use self::length::DecodedLength;
#[cfg_attr(feature = "deprecated", allow(deprecated))]
pub use self::to_bytes::to_bytes;

mod aggregate;
mod body;
mod length;
mod to_bytes;

/// An optimization to try to take a full body if immediately available.
///
/// This is currently limited to *only* `hyper::Body`s.
#[cfg(feature = "http1")]
pub(crate) fn take_full_data<T: HttpBody + 'static>(body: &mut T) -> Option<T::Data> {
    use std::any::{Any, TypeId};

    // This static type check can be optimized at compile-time.
    if TypeId::of::<T>() == TypeId::of::<Body>() {
        let mut full = (body as &mut dyn Any)
            .downcast_mut::<Body>()
            .expect("must be Body")
            .take_full_data();
        // This second cast is required to make the type system happy.
        // Without it, the compiler cannot reason that the type is actually
        // `T::Data`. Oh wells.
        //
        // It's still a measurable win!
        (&mut full as &mut dyn Any)
            .downcast_mut::<Option<T::Data>>()
            .expect("must be T::Data")
            .take()
    } else {
        None
    }
}

fn _assert_send_sync() {
    fn _assert_send<T: Send>() {}
    fn _assert_sync<T: Sync>() {}

    _assert_send::<Body>();
    _assert_sync::<Body>();
}
