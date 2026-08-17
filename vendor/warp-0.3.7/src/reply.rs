//! Reply to requests.
//!
//! A [`Reply`](./trait.Reply.html) is a type that can be converted into an HTTP
//! response to be sent to the client. These are typically the successful
//! counterpart to a [rejection](../reject).
//!
//! The functions in this module are helpers for quickly creating a reply.
//! Besides them, you can return a type that implements [`Reply`](./trait.Reply.html). This
//! could be any of the following:
//!
//! - [`http::Response<impl Into<hyper::Body>>`](https://docs.rs/http)
//! - `String`
//! - `&'static str`
//! - `http::StatusCode`
//!
//! # Example
//!
//! ```
//! use warp::{Filter, http::Response};
//!
//! // Returns an empty `200 OK` response.
//! let empty_200 = warp::any().map(warp::reply);
//!
//! // Returns a `200 OK` response with custom header and body.
//! let custom = warp::any().map(|| {
//!     Response::builder()
//!         .header("my-custom-header", "some-value")
//!         .body("and a custom body")
//! });
//!
//! // GET requests return the empty 200, POST return the custom.
//! let routes = warp::get().and(empty_200)
//!     .or(warp::post().and(custom));
//! ```

use std::borrow::Cow;
use std::convert::TryFrom;

use crate::generic::{Either, One};
use http::header::{HeaderName, HeaderValue, CONTENT_TYPE};
use http::StatusCode;
use hyper::Body;
use serde::Serialize;

// This re-export just looks weird in docs...
pub(crate) use self::sealed::Reply_;
use self::sealed::{BoxedReply, Internal};
#[doc(hidden)]
pub use crate::filters::reply as with;

/// Response type into which types implementing the `Reply` trait are convertable.
pub type Response = ::http::Response<Body>;

/// Returns an empty `Reply` with status code `200 OK`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// // GET /just-ok returns an empty `200 OK`.
/// let route = warp::path("just-ok")
///     .map(|| {
///         println!("got a /just-ok request!");
///         warp::reply()
///     });
/// ```
#[inline]
pub fn reply() -> impl Reply {
    StatusCode::OK
}

/// Convert the value into a `Reply` with the value encoded as JSON.
///
/// The passed value must implement [`Serialize`][ser]. Many
/// collections do, and custom domain types can have `Serialize` derived.
///
/// [ser]: https://serde.rs
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// // GET /ids returns a `200 OK` with a JSON array of ids:
/// // `[1, 3, 7, 13]`
/// let route = warp::path("ids")
///     .map(|| {
///         let our_ids = vec![1, 3, 7, 13];
///         warp::reply::json(&our_ids)
///     });
/// ```
///
/// # Note
///
/// If a type fails to be serialized into JSON, the error is logged at the
/// `error` level, and the returned `impl Reply` will be an empty
/// `500 Internal Server Error` response.
pub fn json<T>(val: &T) -> Json
where
    T: Serialize,
{
    Json {
        inner: serde_json::to_vec(val).map_err(|err| {
            tracing::error!("reply::json error: {}", err);
        }),
    }
}

/// A JSON formatted reply.
#[allow(missing_debug_implementations)]
pub struct Json {
    inner: Result<Vec<u8>, ()>,
}

impl Reply for Json {
    #[inline]
    fn into_response(self) -> Response {
        match self.inner {
            Ok(body) => {
                let mut res = Response::new(body.into());
                res.headers_mut()
                    .insert(CONTENT_TYPE, HeaderValue::from_static("application/json"));
                res
            }
            Err(()) => StatusCode::INTERNAL_SERVER_ERROR.into_response(),
        }
    }
}

/// Reply with a body and `content-type` set to `text/html; charset=utf-8`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let body = r#"
/// <html>
///     <head>
///         <title>HTML with warp!</title>
///     </head>
///     <body>
///         <h1>warp + HTML = &hearts;</h1>
///     </body>
/// </html>
/// "#;
///
/// let route = warp::any()
///     .map(move || {
///         warp::reply::html(body)
///     });
/// ```
pub fn html<T>(body: T) -> Html<T>
where
    Body: From<T>,
    T: Send,
{
    Html { body }
}

/// An HTML reply.
#[allow(missing_debug_implementations)]
pub struct Html<T> {
    body: T,
}

impl<T> Reply for Html<T>
where
    Body: From<T>,
    T: Send,
{
    #[inline]
    fn into_response(self) -> Response {
        let mut res = Response::new(Body::from(self.body));
        res.headers_mut().insert(
            CONTENT_TYPE,
            HeaderValue::from_static("text/html; charset=utf-8"),
        );
        res
    }
}

/// Types that can be converted into a `Response`.
///
/// This trait is implemented for the following:
///
/// - `http::StatusCode`
/// - `http::Response<impl Into<hyper::Body>>`
/// - `String`
/// - `&'static str`
///
/// # Example
///
/// ```rust
/// use warp::{Filter, http::Response};
///
/// struct Message {
///     msg: String
/// }
///
/// impl warp::Reply for Message {
///     fn into_response(self) -> warp::reply::Response {
///         Response::new(format!("message: {}", self.msg).into())
///     }
/// }
///
/// fn handler() -> Message {
///     Message { msg: "Hello".to_string() }
/// }
///
/// let route = warp::any().map(handler);
/// ```
pub trait Reply: BoxedReply + Send {
    /// Converts the given value into a [`Response`].
    ///
    /// [`Response`]: type.Response.html
    fn into_response(self) -> Response;

    /*
    TODO: Currently unsure about having trait methods here, as it
    requires returning an exact type, which I'd rather not commit to.
    Additionally, it doesn't work great with `Box<Reply>`.

    A possible alternative is to have wrappers, like

    - `WithStatus<R: Reply>(StatusCode, R)`


    /// Change the status code of this `Reply`.
    fn with_status(self, status: StatusCode) -> Reply_
    where
        Self: Sized,
    {
        let mut res = self.into_response();
        *res.status_mut() = status;
        Reply_(res)
    }

    /// Add a header to this `Reply`.
    ///
    /// # Example
    ///
    /// ```rust
    /// use warp::Reply;
    ///
    /// let reply = warp::reply()
    ///     .with_header("x-foo", "bar");
    /// ```
    fn with_header<K, V>(self, name: K, value: V) -> Reply_
    where
        Self: Sized,
        HeaderName: TryFrom<K>,
        HeaderValue: TryFrom<V>,
    {
        match <HeaderName as TryFrom<K>>::try_from(name) {
            Ok(name) => match <HeaderValue as TryFrom<V>>::try_from(value) {
                Ok(value) => {
                    let mut res = self.into_response();
                    res.headers_mut().append(name, value);
                    Reply_(res)
                },
                Err(err) => {
                    tracing::error!("with_header value error: {}", err.into());
                    Reply_(::reject::server_error()
                        .into_response())
                }
            },
            Err(err) => {
                tracing::error!("with_header name error: {}", err.into());
                Reply_(::reject::server_error()
                    .into_response())
            }
        }
    }
    */
}

impl<T: Reply + ?Sized> Reply for Box<T> {
    fn into_response(self) -> Response {
        self.boxed_into_response(Internal)
    }
}

fn _assert_object_safe() {
    fn _assert(_: &dyn Reply) {}
}

/// Wrap an `impl Reply` to change its `StatusCode`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let route = warp::any()
///     .map(warp::reply)
///     .map(|reply| {
///         warp::reply::with_status(reply, warp::http::StatusCode::CREATED)
///     });
/// ```
pub fn with_status<T: Reply>(reply: T, status: StatusCode) -> WithStatus<T> {
    WithStatus { reply, status }
}

/// Wrap an `impl Reply` to change its `StatusCode`.
///
/// Returned by `warp::reply::with_status`.
#[derive(Debug)]
pub struct WithStatus<T> {
    reply: T,
    status: StatusCode,
}

impl<T: Reply> Reply for WithStatus<T> {
    fn into_response(self) -> Response {
        let mut res = self.reply.into_response();
        *res.status_mut() = self.status;
        res
    }
}

/// Wrap an `impl Reply` to add a header when rendering.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let route = warp::any()
///     .map(warp::reply)
///     .map(|reply| {
///         warp::reply::with_header(reply, "server", "warp")
///     });
/// ```
pub fn with_header<T: Reply, K, V>(reply: T, name: K, value: V) -> WithHeader<T>
where
    HeaderName: TryFrom<K>,
    <HeaderName as TryFrom<K>>::Error: Into<http::Error>,
    HeaderValue: TryFrom<V>,
    <HeaderValue as TryFrom<V>>::Error: Into<http::Error>,
{
    let header = match <HeaderName as TryFrom<K>>::try_from(name) {
        Ok(name) => match <HeaderValue as TryFrom<V>>::try_from(value) {
            Ok(value) => Some((name, value)),
            Err(err) => {
                let err = err.into();
                tracing::error!("with_header value error: {}", err);
                None
            }
        },
        Err(err) => {
            let err = err.into();
            tracing::error!("with_header name error: {}", err);
            None
        }
    };

    WithHeader { header, reply }
}

/// Wraps an `impl Reply` and adds a header when rendering.
///
/// Returned by `warp::reply::with_header`.
#[derive(Debug)]
pub struct WithHeader<T> {
    header: Option<(HeaderName, HeaderValue)>,
    reply: T,
}

impl<T: Reply> Reply for WithHeader<T> {
    fn into_response(self) -> Response {
        let mut res = self.reply.into_response();
        if let Some((name, value)) = self.header {
            res.headers_mut().insert(name, value);
        }
        res
    }
}

impl<T: Send> Reply for ::http::Response<T>
where
    Body: From<T>,
{
    #[inline]
    fn into_response(self) -> Response {
        self.map(Body::from)
    }
}

impl Reply for ::http::StatusCode {
    #[inline]
    fn into_response(self) -> Response {
        let mut res = Response::default();
        *res.status_mut() = self;
        res
    }
}

impl Reply for ::http::Error {
    #[inline]
    fn into_response(self) -> Response {
        tracing::error!("reply error: {:?}", self);
        StatusCode::INTERNAL_SERVER_ERROR.into_response()
    }
}

impl<T, E> Reply for Result<T, E>
where
    T: Reply,
    E: Reply,
{
    #[inline]
    fn into_response(self) -> Response {
        match self {
            Ok(t) => t.into_response(),
            Err(e) => e.into_response(),
        }
    }
}

fn text_plain<T: Into<Body>>(body: T) -> Response {
    let mut response = ::http::Response::new(body.into());
    response.headers_mut().insert(
        CONTENT_TYPE,
        HeaderValue::from_static("text/plain; charset=utf-8"),
    );
    response
}

impl Reply for String {
    #[inline]
    fn into_response(self) -> Response {
        text_plain(self)
    }
}

impl Reply for Vec<u8> {
    #[inline]
    fn into_response(self) -> Response {
        ::http::Response::builder()
            .header(
                CONTENT_TYPE,
                HeaderValue::from_static("application/octet-stream"),
            )
            .body(Body::from(self))
            .unwrap()
    }
}

impl Reply for &'static str {
    #[inline]
    fn into_response(self) -> Response {
        text_plain(self)
    }
}

impl Reply for Cow<'static, str> {
    #[inline]
    fn into_response(self) -> Response {
        match self {
            Cow::Borrowed(s) => s.into_response(),
            Cow::Owned(s) => s.into_response(),
        }
    }
}

impl Reply for &'static [u8] {
    #[inline]
    fn into_response(self) -> Response {
        ::http::Response::builder()
            .header(
                CONTENT_TYPE,
                HeaderValue::from_static("application/octet-stream"),
            )
            .body(Body::from(self))
            .unwrap()
    }
}

impl<T, U> Reply for Either<T, U>
where
    T: Reply,
    U: Reply,
{
    #[inline]
    fn into_response(self) -> Response {
        match self {
            Either::A(a) => a.into_response(),
            Either::B(b) => b.into_response(),
        }
    }
}

impl<T> Reply for One<T>
where
    T: Reply,
{
    #[inline]
    fn into_response(self) -> Response {
        self.0.into_response()
    }
}

impl Reply for std::convert::Infallible {
    #[inline(always)]
    fn into_response(self) -> Response {
        match self {}
    }
}

mod sealed {
    use super::{Reply, Response};

    // An opaque type to return `impl Reply` from trait methods.
    #[allow(missing_debug_implementations)]
    pub struct Reply_(pub(crate) Response);

    impl Reply for Reply_ {
        #[inline]
        fn into_response(self) -> Response {
            self.0
        }
    }

    #[allow(missing_debug_implementations)]
    pub struct Internal;

    // Implemented for all types that implement `Reply`.
    //
    // A user doesn't need to worry about this, it's just trait
    // hackery to get `Box<dyn Reply>` working.
    pub trait BoxedReply {
        fn boxed_into_response(self: Box<Self>, internal: Internal) -> Response;
    }

    impl<T: Reply> BoxedReply for T {
        fn boxed_into_response(self: Box<Self>, _: Internal) -> Response {
            (*self).into_response()
        }
    }
}

#[cfg(test)]
mod tests {
    use std::collections::HashMap;

    use super::*;

    #[test]
    fn json_serde_error() {
        // a HashMap<Vec, _> cannot be serialized to JSON
        let mut map = HashMap::new();
        map.insert(vec![1, 2], 45);

        let res = json(&map).into_response();
        assert_eq!(res.status(), 500);
    }

    #[test]
    fn response_builder_error() {
        let res = ::http::Response::builder()
            .status(1337)
            .body("woops")
            .into_response();

        assert_eq!(res.status(), 500);
    }

    #[test]
    fn boxed_reply() {
        let r: Box<dyn Reply> = Box::new(reply());
        let resp = r.into_response();
        assert_eq!(resp.status(), 200);
    }
}
