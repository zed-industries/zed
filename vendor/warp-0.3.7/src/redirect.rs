//! Redirect requests to a new location.
//!
//! The types in this module are helpers that implement [`Reply`], and easy
//! to use in order to setup redirects.

use http::{header, StatusCode};

pub use self::sealed::AsLocation;
use crate::reply::{self, Reply};

/// HTTP 301 Moved Permanently
/// Description: The requested resource has been permanently moved to a new URL.
/// Usage: It is used when a URL has permanently moved to a new location. Search engines will update their index to the new URL. Browsers and clients will automatically cache this redirect, so subsequent requests for the old URL will automatically go to the new URL without making a request to the old URL.
/// Common Use Case: Changing domain names, restructuring website URLs.
///
/// # Example
///
/// ```
/// use warp::{http::Uri, Filter};
///
/// let route = warp::path("v1")
///     .map(|| {
///         warp::redirect(Uri::from_static("/v2"))
///     });
/// ```
pub fn redirect(uri: impl AsLocation) -> impl Reply {
    reply::with_header(
        StatusCode::MOVED_PERMANENTLY,
        header::LOCATION,
        uri.header_value(),
    )
}

/// HTTP 302 Found (or Temporary Redirect)
/// Description: The requested resource can be found at a different URL temporarily.
/// Usage: Historically, this status code was used for temporary redirects. However, its meaning was often misunderstood, and different clients treated it differently. As a result, it is recommended to use 307 (or 303) for temporary redirects instead.
/// Common Use Case: Rarely used directly due to ambiguity; replaced by 307 or 303.
///
/// # Example
///
/// ```
/// use warp::{http::Uri, Filter};
///
/// let route = warp::path("v1")
///     .map(|| {
///         warp::redirect::found(Uri::from_static("/v2"))
///     });
/// ```
pub fn found(uri: impl AsLocation) -> impl Reply {
    reply::with_header(StatusCode::FOUND, header::LOCATION, uri.header_value())
}

/// HTTP 303 See Other
/// Description: The response to the request can be found at a different URL, and the client should retrieve it using the GET method.
/// Usage: It is typically used to redirect the client to another URL using a GET request after processing a POST request. It ensures that the client doesn't repeat the POST request if they refresh the page.
/// Common Use Case: After form submissions or any non-idempotent request.
///
/// The HTTP method of the request to the new location will always be `GET`.
///
/// # Example
///
/// ```
/// use warp::{http::Uri, Filter};
///
/// let route = warp::path("v1")
///     .map(|| {
///         warp::redirect::see_other(Uri::from_static("/v2"))
///     });
/// ```
pub fn see_other(uri: impl AsLocation) -> impl Reply {
    reply::with_header(StatusCode::SEE_OTHER, header::LOCATION, uri.header_value())
}

/// HTTP 307 Temporary Redirect:
/// Description: The requested resource can be found at a different URL temporarily.
/// Usage: Similar to 302, but explicitly defined as a temporary redirect. The main difference between 307 and 302 is that 307 preserves the method of the original request when redirecting. If the original request was a POST, the subsequent request to the new URL will also be a POST.
/// Common Use Case: Temporary redirects that should preserve the original request method.
///
/// This is similar to [`see_other`](fn@see_other) but the HTTP method and the body of the request
/// to the new location will be the same as the method and body of the current request.
///
/// # Example
///
/// ```
/// use warp::{http::Uri, Filter};
///
/// let route = warp::path("v1")
///     .map(|| {
///         warp::redirect::temporary(Uri::from_static("/v2"))
///     });
/// ```
pub fn temporary(uri: impl AsLocation) -> impl Reply {
    reply::with_header(
        StatusCode::TEMPORARY_REDIRECT,
        header::LOCATION,
        uri.header_value(),
    )
}

/// HTTP 308 Permanent Redirect
/// Description: The requested resource has been permanently moved to a new URL, and future requests should use the new URL.
/// Usage: Similar to 301, but like 307, it preserves the original request method when redirecting. It indicates that the redirection is permanent, and browsers and clients will cache this redirect like they do for 301.
// Common Use Case: Permanently moving resources to a new URL while maintaining the original request method.
///
/// This is similar to [`redirect`](fn@redirect) but the HTTP method of the request to the new
/// location will be the same as the method of the current request.
///
/// # Example
///
/// ```
/// use warp::{http::Uri, Filter};
///
/// let route = warp::path("v1")
///     .map(|| {
///         warp::redirect::permanent(Uri::from_static("/v2"))
///     });
/// ```
pub fn permanent(uri: impl AsLocation) -> impl Reply {
    reply::with_header(
        StatusCode::PERMANENT_REDIRECT,
        header::LOCATION,
        uri.header_value(),
    )
}

mod sealed {
    use bytes::Bytes;
    use http::{header::HeaderValue, Uri};

    /// Trait for redirect locations. Currently only a `Uri` can be used in
    /// redirect.
    /// This sealed trait exists to allow adding possibly new impls so other
    /// arguments could be accepted, like maybe just `warp::redirect("/v2")`.
    pub trait AsLocation: Sealed {}
    pub trait Sealed {
        fn header_value(self) -> HeaderValue;
    }

    impl AsLocation for Uri {}

    impl Sealed for Uri {
        fn header_value(self) -> HeaderValue {
            let bytes = Bytes::from(self.to_string());
            HeaderValue::from_maybe_shared(bytes).expect("Uri is a valid HeaderValue")
        }
    }
}
