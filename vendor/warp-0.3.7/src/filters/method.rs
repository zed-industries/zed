//! HTTP Method filters.
//!
//! The filters deal with the HTTP Method part of a request. Several here will
//! match the request `Method`, and if not matched, will reject the request
//! with a `405 Method Not Allowed`.
//!
//! There is also [`warp::method()`](method), which never rejects
//! a request, and just extracts the method to be used in your filter chains.
use futures_util::future;
use http::Method;

use crate::filter::{filter_fn, filter_fn_one, Filter, One};
use crate::reject::Rejection;
use std::convert::Infallible;

/// Create a `Filter` that requires the request method to be `GET`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let get_only = warp::get().map(warp::reply);
/// ```
pub fn get() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    method_is(|| &Method::GET)
}

/// Create a `Filter` that requires the request method to be `POST`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let post_only = warp::post().map(warp::reply);
/// ```
pub fn post() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    method_is(|| &Method::POST)
}

/// Create a `Filter` that requires the request method to be `PUT`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let put_only = warp::put().map(warp::reply);
/// ```
pub fn put() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    method_is(|| &Method::PUT)
}

/// Create a `Filter` that requires the request method to be `DELETE`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let delete_only = warp::delete().map(warp::reply);
/// ```
pub fn delete() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    method_is(|| &Method::DELETE)
}

/// Create a `Filter` that requires the request method to be `HEAD`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let head_only = warp::head().map(warp::reply);
/// ```
pub fn head() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    method_is(|| &Method::HEAD)
}

/// Create a `Filter` that requires the request method to be `OPTIONS`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let options_only = warp::options().map(warp::reply);
/// ```
pub fn options() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    method_is(|| &Method::OPTIONS)
}

/// Create a `Filter` that requires the request method to be `PATCH`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let patch_only = warp::patch().map(warp::reply);
/// ```
pub fn patch() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    method_is(|| &Method::PATCH)
}

/// Extract the `Method` from the request.
///
/// This never rejects a request.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let route = warp::method()
///     .map(|method| {
///         format!("You sent a {} request!", method)
///     });
/// ```
pub fn method() -> impl Filter<Extract = One<Method>, Error = Infallible> + Copy {
    filter_fn_one(|route| future::ok::<_, Infallible>(route.method().clone()))
}

// NOTE: This takes a static function instead of `&'static Method` directly
// so that the `impl Filter` can be zero-sized. Moving it around should be
// cheaper than holding a single static pointer (which would make it 1 word).
fn method_is<F>(func: F) -> impl Filter<Extract = (), Error = Rejection> + Copy
where
    F: Fn() -> &'static Method + Copy,
{
    filter_fn(move |route| {
        let method = func();
        tracing::trace!("method::{:?}?: {:?}", method, route.method());
        if route.method() == method {
            future::ok(())
        } else {
            future::err(crate::reject::method_not_allowed())
        }
    })
}

#[cfg(test)]
mod tests {
    #[test]
    fn method_size_of() {
        // See comment on `method_is` function.
        assert_eq!(std::mem::size_of_val(&super::get()), 0,);
    }
}
