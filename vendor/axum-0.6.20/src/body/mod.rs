//! HTTP body utilities.

mod stream_body;

pub use self::stream_body::StreamBody;

#[doc(no_inline)]
pub use http_body::{Body as HttpBody, Empty, Full};

#[doc(no_inline)]
pub use hyper::body::Body;

#[doc(no_inline)]
pub use bytes::Bytes;

#[doc(inline)]
pub use axum_core::body::{boxed, BoxBody};
