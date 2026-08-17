//! Header Filters
//!
//! These filters are used to interact with the Request HTTP headers. Some
//! of them, like `exact` and `exact_ignore_case`, are just predicates,
//! they don't extract any values. The `header` filter allows parsing
//! a type from any header.
use std::convert::Infallible;
use std::str::FromStr;

use futures_util::future;
use headers::{Header, HeaderMapExt};
use http::header::HeaderValue;
use http::HeaderMap;

use crate::filter::{filter_fn, filter_fn_one, Filter, One};
use crate::reject::{self, Rejection};

/// Create a `Filter` that tries to parse the specified header.
///
/// This `Filter` will look for a header with supplied name, and try to
/// parse to a `T`, otherwise rejects the request.
///
/// # Example
///
/// ```
/// use std::net::SocketAddr;
///
/// // Parse `content-length: 100` as a `u64`
/// let content_length = warp::header::<u64>("content-length");
///
/// // Parse `host: 127.0.0.1:8080` as a `SocketAddr
/// let local_host = warp::header::<SocketAddr>("host");
///
/// // Parse `foo: bar` into a `String`
/// let foo = warp::header::<String>("foo");
/// ```
pub fn header<T: FromStr + Send + 'static>(
    name: &'static str,
) -> impl Filter<Extract = One<T>, Error = Rejection> + Copy {
    filter_fn_one(move |route| {
        tracing::trace!("header({:?})", name);
        let route = route
            .headers()
            .get(name)
            .ok_or_else(|| reject::missing_header(name))
            .and_then(|value| value.to_str().map_err(|_| reject::invalid_header(name)))
            .and_then(|s| T::from_str(s).map_err(|_| reject::invalid_header(name)));
        future::ready(route)
    })
}

pub(crate) fn header2<T: Header + Send + 'static>(
) -> impl Filter<Extract = One<T>, Error = Rejection> + Copy {
    filter_fn_one(move |route| {
        tracing::trace!("header2({:?})", T::name());
        let route = route
            .headers()
            .typed_get()
            .ok_or_else(|| reject::invalid_header(T::name().as_str()));
        future::ready(route)
    })
}

/// Create a `Filter` that tries to parse the specified header, if it exists.
///
/// If the header does not exist, it yields `None`. Otherwise, it will try to
/// parse as a `T`, and if it fails, a invalid header rejection is return. If
/// successful, the filter yields `Some(T)`.
///
/// # Example
///
/// ```
/// // Grab the `authorization` header if it exists.
/// let opt_auth = warp::header::optional::<String>("authorization");
/// ```
pub fn optional<T>(
    name: &'static str,
) -> impl Filter<Extract = One<Option<T>>, Error = Rejection> + Copy
where
    T: FromStr + Send + 'static,
{
    filter_fn_one(move |route| {
        tracing::trace!("optional({:?})", name);
        let result = route.headers().get(name).map(|value| {
            value
                .to_str()
                .map_err(|_| reject::invalid_header(name))?
                .parse::<T>()
                .map_err(|_| reject::invalid_header(name))
        });

        match result {
            Some(Ok(t)) => future::ok(Some(t)),
            Some(Err(e)) => future::err(e),
            None => future::ok(None),
        }
    })
}

pub(crate) fn optional2<T>() -> impl Filter<Extract = One<Option<T>>, Error = Infallible> + Copy
where
    T: Header + Send + 'static,
{
    filter_fn_one(move |route| future::ready(Ok(route.headers().typed_get())))
}

/* TODO
pub fn exact2<T>(header: T) -> impl FilterClone<Extract=(), Error=Rejection>
where
    T: Header + PartialEq + Clone + Send,
{
    filter_fn(move |route| {
        tracing::trace!("exact2({:?})", T::NAME);
        route.headers()
            .typed_get::<T>()
            .and_then(|val| if val == header {
                Some(())
            } else {
                None
            })
            .ok_or_else(|| reject::bad_request())
    })
}
*/

/// Create a `Filter` that requires a header to match the value exactly.
///
/// This `Filter` will look for a header with supplied name and the exact
/// value, otherwise rejects the request.
///
/// # Example
///
/// ```
/// // Require `dnt: 1` header to be set.
/// let must_dnt = warp::header::exact("dnt", "1");
/// ```
pub fn exact(
    name: &'static str,
    value: &'static str,
) -> impl Filter<Extract = (), Error = Rejection> + Copy {
    filter_fn(move |route| {
        tracing::trace!("exact?({:?}, {:?})", name, value);
        let route = route
            .headers()
            .get(name)
            .ok_or_else(|| reject::missing_header(name))
            .and_then(|val| {
                if val == value {
                    Ok(())
                } else {
                    Err(reject::invalid_header(name))
                }
            });
        future::ready(route)
    })
}

/// Create a `Filter` that requires a header to match the value exactly.
///
/// This `Filter` will look for a header with supplied name and the exact
/// value, ignoring ASCII case, otherwise rejects the request.
///
/// # Example
///
/// ```
/// // Require `connection: keep-alive` header to be set.
/// let keep_alive = warp::header::exact_ignore_case("connection", "keep-alive");
/// ```
pub fn exact_ignore_case(
    name: &'static str,
    value: &'static str,
) -> impl Filter<Extract = (), Error = Rejection> + Copy {
    filter_fn(move |route| {
        tracing::trace!("exact_ignore_case({:?}, {:?})", name, value);
        let route = route
            .headers()
            .get(name)
            .ok_or_else(|| reject::missing_header(name))
            .and_then(|val| {
                if val.as_bytes().eq_ignore_ascii_case(value.as_bytes()) {
                    Ok(())
                } else {
                    Err(reject::invalid_header(name))
                }
            });
        future::ready(route)
    })
}

/// Create a `Filter` that gets a `HeaderValue` for the name.
///
/// # Example
///
/// ```
/// use warp::{Filter, http::header::HeaderValue};
///
/// let filter = warp::header::value("x-token")
///     .map(|value: HeaderValue| {
///         format!("header value bytes: {:?}", value)
///     });
/// ```
pub fn value(
    name: &'static str,
) -> impl Filter<Extract = One<HeaderValue>, Error = Rejection> + Copy {
    filter_fn_one(move |route| {
        tracing::trace!("value({:?})", name);
        let route = route
            .headers()
            .get(name)
            .cloned()
            .ok_or_else(|| reject::missing_header(name));
        future::ready(route)
    })
}

/// Create a `Filter` that returns a clone of the request's `HeaderMap`.
///
/// # Example
///
/// ```
/// use warp::{Filter, http::HeaderMap};
///
/// let headers = warp::header::headers_cloned()
///     .map(|headers: HeaderMap| {
///         format!("header count: {}", headers.len())
///     });
/// ```
pub fn headers_cloned() -> impl Filter<Extract = One<HeaderMap>, Error = Infallible> + Copy {
    filter_fn_one(|route| future::ok(route.headers().clone()))
}
