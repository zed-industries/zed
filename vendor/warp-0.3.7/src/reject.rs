//! Rejections
//!
//! Part of the power of the [`Filter`](../trait.Filter.html) system is being able to
//! reject a request from a filter chain. This allows for filters to be
//! combined with `or`, so that if one side of the chain finds that a request
//! doesn't fulfill its requirements, the other side can try to process
//! the request.
//!
//! Many of the built-in [`filters`](../filters) will automatically reject
//! the request with an appropriate rejection. However, you can also build
//! new custom [`Filter`](../trait.Filter.html)s and still want other routes to be
//! matchable in the case a predicate doesn't hold.
//!
//! As a request is processed by a Filter chain, the rejections are accumulated into
//! a list contained by the [`Rejection`](struct.Rejection.html) type. Rejections from
//! filters can be handled using [`Filter::recover`](../trait.Filter.html#method.recover).
//! This is a convenient way to map rejections into a [`Reply`](../reply/trait.Reply.html).
//!
//! For a more complete example see the
//! [Rejection Example](https://github.com/seanmonstar/warp/blob/master/examples/rejections.rs)
//! from the repository.
//!
//! # Example
//!
//! ```
//! use warp::{reply, Reply, Filter, reject, Rejection, http::StatusCode};
//!
//! #[derive(Debug)]
//! struct InvalidParameter;
//!
//! impl reject::Reject for InvalidParameter {}
//!
//! // Custom rejection handler that maps rejections into responses.
//! async fn handle_rejection(err: Rejection) -> Result<impl Reply, std::convert::Infallible> {
//!     if err.is_not_found() {
//!         Ok(reply::with_status("NOT_FOUND", StatusCode::NOT_FOUND))
//!     } else if let Some(e) = err.find::<InvalidParameter>() {
//!         Ok(reply::with_status("BAD_REQUEST", StatusCode::BAD_REQUEST))
//!     } else {
//!         eprintln!("unhandled rejection: {:?}", err);
//!         Ok(reply::with_status("INTERNAL_SERVER_ERROR", StatusCode::INTERNAL_SERVER_ERROR))
//!     }
//! }
//!
//!
//! // Filter on `/:id`, but reject with InvalidParameter if the `id` is `0`.
//! // Recover from this rejection using a custom rejection handler.
//! let route = warp::path::param()
//!     .and_then(|id: u32| async move {
//!         if id == 0 {
//!             Err(warp::reject::custom(InvalidParameter))
//!         } else {
//!             Ok("id is valid")
//!         }
//!     })
//!     .recover(handle_rejection);
//! ```

use std::any::Any;
use std::convert::Infallible;
use std::error::Error as StdError;
use std::fmt;

use http::{
    header::{HeaderValue, CONTENT_TYPE},
    StatusCode,
};
use hyper::Body;

pub(crate) use self::sealed::{CombineRejection, IsReject};

/// Rejects a request with `404 Not Found`.
#[inline]
pub fn reject() -> Rejection {
    not_found()
}

/// Rejects a request with `404 Not Found`.
#[inline]
pub fn not_found() -> Rejection {
    Rejection {
        reason: Reason::NotFound,
    }
}

// 400 Bad Request
#[inline]
pub(crate) fn invalid_query() -> Rejection {
    known(InvalidQuery { _p: () })
}

// 400 Bad Request
#[inline]
pub(crate) fn missing_header(name: &'static str) -> Rejection {
    known(MissingHeader { name })
}

// 400 Bad Request
#[inline]
pub(crate) fn invalid_header(name: &'static str) -> Rejection {
    known(InvalidHeader { name })
}

// 400 Bad Request
#[inline]
pub(crate) fn missing_cookie(name: &'static str) -> Rejection {
    known(MissingCookie { name })
}

// 405 Method Not Allowed
#[inline]
pub(crate) fn method_not_allowed() -> Rejection {
    known(MethodNotAllowed { _p: () })
}

// 411 Length Required
#[inline]
pub(crate) fn length_required() -> Rejection {
    known(LengthRequired { _p: () })
}

// 413 Payload Too Large
#[inline]
pub(crate) fn payload_too_large() -> Rejection {
    known(PayloadTooLarge { _p: () })
}

// 415 Unsupported Media Type
//
// Used by the body filters if the request payload content-type doesn't match
// what can be deserialized.
#[inline]
pub(crate) fn unsupported_media_type() -> Rejection {
    known(UnsupportedMediaType { _p: () })
}

/// Rejects a request with a custom cause.
///
/// A [`recover`][] filter should convert this `Rejection` into a `Reply`,
/// or else this will be returned as a `500 Internal Server Error`.
///
/// [`recover`]: ../trait.Filter.html#method.recover
pub fn custom<T: Reject>(err: T) -> Rejection {
    Rejection::custom(Box::new(err))
}

/// Protect against re-rejecting a rejection.
///
/// ```compile_fail
/// fn with(r: warp::Rejection) {
///     let _wat = warp::reject::custom(r);
/// }
/// ```
fn __reject_custom_compilefail() {}

/// A marker trait to ensure proper types are used for custom rejections.
///
/// Can be converted into Rejection.
///
/// # Example
///
/// ```
/// use warp::{Filter, reject::Reject};
///
/// #[derive(Debug)]
/// struct RateLimited;
///
/// impl Reject for RateLimited {}
///
/// let route = warp::any().and_then(|| async {
///     Err::<(), _>(warp::reject::custom(RateLimited))
/// });
/// ```
// Require `Sized` for now to prevent passing a `Box<dyn Reject>`, since we
// would be double-boxing it, and the downcasting wouldn't work as expected.
pub trait Reject: fmt::Debug + Sized + Send + Sync + 'static {}

trait Cause: fmt::Debug + Send + Sync + 'static {
    fn as_any(&self) -> &dyn Any;
}

impl<T> Cause for T
where
    T: fmt::Debug + Send + Sync + 'static,
{
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl dyn Cause {
    fn downcast_ref<T: Any>(&self) -> Option<&T> {
        self.as_any().downcast_ref::<T>()
    }
}

pub(crate) fn known<T: Into<Known>>(err: T) -> Rejection {
    Rejection::known(err.into())
}

/// Rejection of a request by a [`Filter`](crate::Filter).
///
/// See the [`reject`](module@crate::reject) documentation for more.
pub struct Rejection {
    reason: Reason,
}

enum Reason {
    NotFound,
    Other(Box<Rejections>),
}

enum Rejections {
    Known(Known),
    Custom(Box<dyn Cause>),
    Combined(Box<Rejections>, Box<Rejections>),
}

macro_rules! enum_known {
     ($($(#[$attr:meta])* $var:ident($ty:path),)+) => (
        pub(crate) enum Known {
            $(
            $(#[$attr])*
            $var($ty),
            )+
        }

        impl Known {
            fn inner_as_any(&self) -> &dyn Any {
                match *self {
                    $(
                    $(#[$attr])*
                    Known::$var(ref t) => t,
                    )+
                }
            }
        }

        impl fmt::Debug for Known {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match *self {
                    $(
                    $(#[$attr])*
                    Known::$var(ref t) => t.fmt(f),
                    )+
                }
            }
        }

        impl fmt::Display for Known {
            fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
                match *self {
                    $(
                    $(#[$attr])*
                    Known::$var(ref t) => t.fmt(f),
                    )+
                }
            }
        }

        $(
        #[doc(hidden)]
        $(#[$attr])*
        impl From<$ty> for Known {
            fn from(ty: $ty) -> Known {
                Known::$var(ty)
            }
        }
        )+
    );
}

enum_known! {
    MethodNotAllowed(MethodNotAllowed),
    InvalidHeader(InvalidHeader),
    MissingHeader(MissingHeader),
    MissingCookie(MissingCookie),
    InvalidQuery(InvalidQuery),
    LengthRequired(LengthRequired),
    PayloadTooLarge(PayloadTooLarge),
    UnsupportedMediaType(UnsupportedMediaType),
    FileOpenError(crate::fs::FileOpenError),
    FilePermissionError(crate::fs::FilePermissionError),
    BodyReadError(crate::body::BodyReadError),
    BodyDeserializeError(crate::body::BodyDeserializeError),
    CorsForbidden(crate::cors::CorsForbidden),
    #[cfg(feature = "websocket")]
    MissingConnectionUpgrade(crate::ws::MissingConnectionUpgrade),
    MissingExtension(crate::ext::MissingExtension),
    BodyConsumedMultipleTimes(crate::body::BodyConsumedMultipleTimes),
}

impl Rejection {
    fn known(known: Known) -> Self {
        Rejection {
            reason: Reason::Other(Box::new(Rejections::Known(known))),
        }
    }

    fn custom(other: Box<dyn Cause>) -> Self {
        Rejection {
            reason: Reason::Other(Box::new(Rejections::Custom(other))),
        }
    }

    /// Searches this `Rejection` for a specific cause.
    ///
    /// A `Rejection` will accumulate causes over a `Filter` chain. This method
    /// can search through them and return the first cause of this type.
    ///
    /// # Example
    ///
    /// ```
    /// #[derive(Debug)]
    /// struct Nope;
    ///
    /// impl warp::reject::Reject for Nope {}
    ///
    /// let reject = warp::reject::custom(Nope);
    ///
    /// if let Some(nope) = reject.find::<Nope>() {
    ///    println!("found it: {:?}", nope);
    /// }
    /// ```
    pub fn find<T: 'static>(&self) -> Option<&T> {
        if let Reason::Other(ref rejections) = self.reason {
            return rejections.find();
        }
        None
    }

    /// Returns true if this Rejection was made via `warp::reject::not_found`.
    ///
    /// # Example
    ///
    /// ```
    /// let rejection = warp::reject();
    ///
    /// assert!(rejection.is_not_found());
    /// ```
    pub fn is_not_found(&self) -> bool {
        matches!(self.reason, Reason::NotFound)
    }
}

impl<T: Reject> From<T> for Rejection {
    #[inline]
    fn from(err: T) -> Rejection {
        custom(err)
    }
}

impl From<Infallible> for Rejection {
    #[inline]
    fn from(infallible: Infallible) -> Rejection {
        match infallible {}
    }
}

impl IsReject for Infallible {
    fn status(&self) -> StatusCode {
        match *self {}
    }

    fn into_response(&self) -> crate::reply::Response {
        match *self {}
    }
}

impl IsReject for Rejection {
    fn status(&self) -> StatusCode {
        match self.reason {
            Reason::NotFound => StatusCode::NOT_FOUND,
            Reason::Other(ref other) => other.status(),
        }
    }

    fn into_response(&self) -> crate::reply::Response {
        match self.reason {
            Reason::NotFound => {
                let mut res = http::Response::default();
                *res.status_mut() = StatusCode::NOT_FOUND;
                res
            }
            Reason::Other(ref other) => other.into_response(),
        }
    }
}

impl fmt::Debug for Rejection {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Rejection").field(&self.reason).finish()
    }
}

impl fmt::Debug for Reason {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match *self {
            Reason::NotFound => f.write_str("NotFound"),
            Reason::Other(ref other) => match **other {
                Rejections::Known(ref e) => fmt::Debug::fmt(e, f),
                Rejections::Custom(ref e) => fmt::Debug::fmt(e, f),
                Rejections::Combined(ref a, ref b) => {
                    let mut list = f.debug_list();
                    a.debug_list(&mut list);
                    b.debug_list(&mut list);
                    list.finish()
                }
            },
        }
    }
}

// ===== Rejections =====

impl Rejections {
    fn status(&self) -> StatusCode {
        match *self {
            Rejections::Known(ref k) => match *k {
                Known::MethodNotAllowed(_) => StatusCode::METHOD_NOT_ALLOWED,
                Known::InvalidHeader(_)
                | Known::MissingHeader(_)
                | Known::MissingCookie(_)
                | Known::InvalidQuery(_)
                | Known::BodyReadError(_)
                | Known::BodyDeserializeError(_) => StatusCode::BAD_REQUEST,
                #[cfg(feature = "websocket")]
                Known::MissingConnectionUpgrade(_) => StatusCode::BAD_REQUEST,
                Known::LengthRequired(_) => StatusCode::LENGTH_REQUIRED,
                Known::PayloadTooLarge(_) => StatusCode::PAYLOAD_TOO_LARGE,
                Known::UnsupportedMediaType(_) => StatusCode::UNSUPPORTED_MEDIA_TYPE,
                Known::FilePermissionError(_) | Known::CorsForbidden(_) => StatusCode::FORBIDDEN,
                Known::FileOpenError(_)
                | Known::MissingExtension(_)
                | Known::BodyConsumedMultipleTimes(_) => StatusCode::INTERNAL_SERVER_ERROR,
            },
            Rejections::Custom(..) => StatusCode::INTERNAL_SERVER_ERROR,
            Rejections::Combined(..) => self.preferred().status(),
        }
    }

    fn into_response(&self) -> crate::reply::Response {
        match *self {
            Rejections::Known(ref e) => {
                let mut res = http::Response::new(Body::from(e.to_string()));
                *res.status_mut() = self.status();
                res.headers_mut().insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static("text/plain; charset=utf-8"),
                );
                res
            }
            Rejections::Custom(ref e) => {
                tracing::error!(
                    "unhandled custom rejection, returning 500 response: {:?}",
                    e
                );
                let body = format!("Unhandled rejection: {:?}", e);
                let mut res = http::Response::new(Body::from(body));
                *res.status_mut() = self.status();
                res.headers_mut().insert(
                    CONTENT_TYPE,
                    HeaderValue::from_static("text/plain; charset=utf-8"),
                );
                res
            }
            Rejections::Combined(..) => self.preferred().into_response(),
        }
    }

    fn find<T: 'static>(&self) -> Option<&T> {
        match *self {
            Rejections::Known(ref e) => e.inner_as_any().downcast_ref(),
            Rejections::Custom(ref e) => e.downcast_ref(),
            Rejections::Combined(ref a, ref b) => a.find().or_else(|| b.find()),
        }
    }

    fn debug_list(&self, f: &mut fmt::DebugList<'_, '_>) {
        match *self {
            Rejections::Known(ref e) => {
                f.entry(e);
            }
            Rejections::Custom(ref e) => {
                f.entry(e);
            }
            Rejections::Combined(ref a, ref b) => {
                a.debug_list(f);
                b.debug_list(f);
            }
        }
    }

    fn preferred(&self) -> &Rejections {
        match self {
            Rejections::Known(_) | Rejections::Custom(_) => self,
            Rejections::Combined(a, b) => {
                let a = a.preferred();
                let b = b.preferred();
                // Now both a and b are known or custom, so it is safe
                // to get status
                // Compare status codes, with this priority:
                // - NOT_FOUND is lowest
                // - METHOD_NOT_ALLOWED is second
                // - if one status code is greater than the other
                // - otherwise, prefer A...
                match (a.status(), b.status()) {
                    (_, StatusCode::NOT_FOUND) => a,
                    (StatusCode::NOT_FOUND, _) => b,
                    (_, StatusCode::METHOD_NOT_ALLOWED) => a,
                    (StatusCode::METHOD_NOT_ALLOWED, _) => b,
                    (sa, sb) if sa < sb => b,
                    _ => a,
                }
            }
        }
    }
}

unit_error! {
    /// Invalid query
    pub InvalidQuery: "Invalid query string"
}

unit_error! {
    /// HTTP method not allowed
    pub MethodNotAllowed: "HTTP method not allowed"
}

unit_error! {
    /// A content-length header is required
    pub LengthRequired: "A content-length header is required"
}

unit_error! {
    /// The request payload is too large
    pub PayloadTooLarge: "The request payload is too large"
}

unit_error! {
    /// The request's content-type is not supported
    pub UnsupportedMediaType: "The request's content-type is not supported"
}

/// Missing request header
#[derive(Debug)]
pub struct MissingHeader {
    name: &'static str,
}

impl MissingHeader {
    /// Retrieve the name of the header that was missing
    pub fn name(&self) -> &str {
        self.name
    }
}

impl fmt::Display for MissingHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Missing request header {:?}", self.name)
    }
}

impl StdError for MissingHeader {}

/// Invalid request header
#[derive(Debug)]
pub struct InvalidHeader {
    name: &'static str,
}

impl InvalidHeader {
    /// Retrieve the name of the header that was invalid
    pub fn name(&self) -> &str {
        self.name
    }
}

impl fmt::Display for InvalidHeader {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Invalid request header {:?}", self.name)
    }
}

impl StdError for InvalidHeader {}

/// Missing cookie
#[derive(Debug)]
pub struct MissingCookie {
    name: &'static str,
}

impl MissingCookie {
    /// Retrieve the name of the cookie that was missing
    pub fn name(&self) -> &str {
        self.name
    }
}

impl fmt::Display for MissingCookie {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "Missing request cookie {:?}", self.name)
    }
}

impl StdError for MissingCookie {}

mod sealed {
    use super::{Reason, Rejection, Rejections};
    use http::StatusCode;
    use std::convert::Infallible;
    use std::fmt;

    // This sealed trait exists to allow Filters to return either `Rejection`
    // or `!`. There are no other types that make sense, and so it is sealed.
    pub trait IsReject: fmt::Debug + Send + Sync {
        fn status(&self) -> StatusCode;
        fn into_response(&self) -> crate::reply::Response;
    }

    fn _assert_object_safe() {
        fn _assert(_: &dyn IsReject) {}
    }

    // This weird trait is to allow optimizations of propagating when a
    // rejection can *never* happen (currently with the `Never` type,
    // eventually to be replaced with `!`).
    //
    // Using this trait means the `Never` gets propagated to chained filters,
    // allowing LLVM to eliminate more code paths. Without it, such as just
    // requiring that `Rejection::from(Never)` were used in those filters,
    // would mean that links later in the chain may assume a rejection *could*
    // happen, and no longer eliminate those branches.
    pub trait CombineRejection<E>: Send + Sized {
        /// The type that should be returned when only 1 of the two
        /// "rejections" occurs.
        ///
        /// # For example:
        ///
        /// `warp::any().and(warp::path("foo"))` has the following steps:
        ///
        /// 1. Since this is `and`, only **one** of the rejections will occur,
        ///    and as soon as it does, it will be returned.
        /// 2. `warp::any()` rejects with `Never`. So, it will never return `Never`.
        /// 3. `warp::path()` rejects with `Rejection`. It may return `Rejection`.
        ///
        /// Thus, if the above filter rejects, it will definitely be `Rejection`.
        type One: IsReject + From<Self> + From<E> + Into<Rejection>;

        /// The type that should be returned when both rejections occur,
        /// and need to be combined.
        type Combined: IsReject;

        fn combine(self, other: E) -> Self::Combined;
    }

    impl CombineRejection<Rejection> for Rejection {
        type One = Rejection;
        type Combined = Rejection;

        fn combine(self, other: Rejection) -> Self::Combined {
            let reason = match (self.reason, other.reason) {
                (Reason::Other(left), Reason::Other(right)) => {
                    Reason::Other(Box::new(Rejections::Combined(left, right)))
                }
                (Reason::Other(other), Reason::NotFound)
                | (Reason::NotFound, Reason::Other(other)) => {
                    // ignore the NotFound
                    Reason::Other(other)
                }
                (Reason::NotFound, Reason::NotFound) => Reason::NotFound,
            };

            Rejection { reason }
        }
    }

    impl CombineRejection<Infallible> for Rejection {
        type One = Rejection;
        type Combined = Infallible;

        fn combine(self, other: Infallible) -> Self::Combined {
            match other {}
        }
    }

    impl CombineRejection<Rejection> for Infallible {
        type One = Rejection;
        type Combined = Infallible;

        fn combine(self, _: Rejection) -> Self::Combined {
            match self {}
        }
    }

    impl CombineRejection<Infallible> for Infallible {
        type One = Infallible;
        type Combined = Infallible;

        fn combine(self, _: Infallible) -> Self::Combined {
            match self {}
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[derive(Debug, PartialEq)]
    struct Left;

    #[derive(Debug, PartialEq)]
    struct Right;

    impl Reject for Left {}
    impl Reject for Right {}

    #[test]
    fn rejection_status() {
        assert_eq!(not_found().status(), StatusCode::NOT_FOUND);
        assert_eq!(
            method_not_allowed().status(),
            StatusCode::METHOD_NOT_ALLOWED
        );
        assert_eq!(length_required().status(), StatusCode::LENGTH_REQUIRED);
        assert_eq!(payload_too_large().status(), StatusCode::PAYLOAD_TOO_LARGE);
        assert_eq!(
            unsupported_media_type().status(),
            StatusCode::UNSUPPORTED_MEDIA_TYPE
        );
        assert_eq!(custom(Left).status(), StatusCode::INTERNAL_SERVER_ERROR);
    }

    #[tokio::test]
    async fn combine_rejection_causes_with_some_left_and_none_right() {
        let left = custom(Left);
        let right = not_found();
        let reject = left.combine(right);
        let resp = reject.into_response();

        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            response_body_string(resp).await,
            "Unhandled rejection: Left"
        )
    }

    #[tokio::test]
    async fn combine_rejection_causes_with_none_left_and_some_right() {
        let left = not_found();
        let right = custom(Right);
        let reject = left.combine(right);
        let resp = reject.into_response();

        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            response_body_string(resp).await,
            "Unhandled rejection: Right"
        )
    }

    #[tokio::test]
    async fn unhandled_customs() {
        let reject = not_found().combine(custom(Right));

        let resp = reject.into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            response_body_string(resp).await,
            "Unhandled rejection: Right"
        );

        // There's no real way to determine which is worse, since both are a 500,
        // so pick the first one.
        let reject = custom(Left).combine(custom(Right));

        let resp = reject.into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            response_body_string(resp).await,
            "Unhandled rejection: Left"
        );

        // With many rejections, custom still is top priority.
        let reject = not_found()
            .combine(not_found())
            .combine(not_found())
            .combine(custom(Right))
            .combine(not_found());

        let resp = reject.into_response();
        assert_eq!(resp.status(), StatusCode::INTERNAL_SERVER_ERROR);
        assert_eq!(
            response_body_string(resp).await,
            "Unhandled rejection: Right"
        );
    }

    async fn response_body_string(resp: crate::reply::Response) -> String {
        let (_, body) = resp.into_parts();
        let body_bytes = hyper::body::to_bytes(body).await.expect("failed concat");
        String::from_utf8_lossy(&body_bytes).to_string()
    }

    #[test]
    fn find_cause() {
        let rej = custom(Left);

        assert_eq!(rej.find::<Left>(), Some(&Left));

        let rej = rej.combine(method_not_allowed());

        assert_eq!(rej.find::<Left>(), Some(&Left));
        assert!(rej.find::<MethodNotAllowed>().is_some(), "MethodNotAllowed");
    }

    #[test]
    fn size_of_rejection() {
        assert_eq!(
            ::std::mem::size_of::<Rejection>(),
            ::std::mem::size_of::<usize>(),
        );
    }

    #[derive(Debug)]
    struct X(#[allow(unused)] u32);
    impl Reject for X {}

    fn combine_n<F, R>(n: u32, new_reject: F) -> Rejection
    where
        F: Fn(u32) -> R,
        R: Reject,
    {
        let mut rej = not_found();

        for i in 0..n {
            rej = rej.combine(custom(new_reject(i)));
        }

        rej
    }

    #[test]
    fn test_debug() {
        let rej = combine_n(3, X);

        let s = format!("{:?}", rej);
        assert_eq!(s, "Rejection([X(0), X(1), X(2)])");
    }

    #[test]
    fn convert_big_rejections_into_response() {
        let mut rejections = Rejections::Custom(Box::new(std::io::Error::from_raw_os_error(100)));
        for _ in 0..50 {
            rejections = Rejections::Combined(
                Box::new(Rejections::Known(Known::MethodNotAllowed(
                    MethodNotAllowed { _p: () },
                ))),
                Box::new(rejections),
            );
        }
        let reason = Reason::Other(Box::new(rejections));
        let rejection = Rejection { reason };
        assert_eq!(
            StatusCode::INTERNAL_SERVER_ERROR,
            rejection.into_response().status()
        );
    }
}
