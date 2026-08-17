//! Path Filters
//!
//! The [`Filter`](crate::Filter)s here work on the "path" of requests.
//!
//! - [`path`](./fn.path.html) matches a specific segment, like `/foo`.
//! - [`param`](./fn.param.html) tries to parse a segment into a type, like `/:u16`.
//! - [`end`](./fn.end.html) matches when the path end is found.
//! - [`path!`](../../macro.path.html) eases combining multiple `path` and `param` filters.
//!
//! # Routing
//!
//! Routing in warp is simple yet powerful.
//!
//! First up, matching a single segment:
//!
//! ```
//! use warp::Filter;
//!
//! // GET /hi
//! let hi = warp::path("hi").map(|| {
//!     "Hello, World!"
//! });
//! ```
//!
//! How about multiple segments? It's easiest with the `path!` macro:
//!
//! ```
//! # use warp::Filter;
//! // GET /hello/from/warp
//! let hello_from_warp = warp::path!("hello" / "from" / "warp").map(|| {
//!     "Hello from warp!"
//! });
//! ```
//!
//! Neat! But how do I handle **parameters** in paths?
//!
//! ```
//! # use warp::Filter;
//! // GET /sum/:u32/:u32
//! let sum = warp::path!("sum" / u32 / u32).map(|a, b| {
//!     format!("{} + {} = {}", a, b, a + b)
//! });
//! ```
//!
//! In fact, any type that implements `FromStr` can be used, in any order:
//!
//! ```
//! # use warp::Filter;
//! // GET /:u16/times/:u16
//! let times = warp::path!(u16 / "times" / u16).map(|a, b| {
//!     format!("{} times {} = {}", a, b, a * b)
//! });
//! ```
//!
//! Oh shoot, those math routes should be **mounted** at a different path,
//! is that possible? Yep!
//!
//! ```
//! # use warp::Filter;
//! # let sum = warp::any().map(warp::reply);
//! # let times = sum.clone();
//! // GET /math/sum/:u32/:u32
//! // GET /math/:u16/times/:u16
//! let math = warp::path("math");
//! let math_sum = math.and(sum);
//! let math_times = math.and(times);
//! ```
//!
//! What! `and`? What's that do?
//!
//! It combines the filters in a sort of "this and then that" order. In fact,
//! it's exactly what the `path!` macro has been doing internally.
//!
//! ```
//! # use warp::Filter;
//! // GET /bye/:string
//! let bye = warp::path("bye")
//!     .and(warp::path::param())
//!     .map(|name: String| {
//!         format!("Good bye, {}!", name)
//!     });
//! ```
//!
//! Ah, so, can filters do things besides `and`?
//!
//! Why, yes they can! They can also `or`! As you might expect, `or` creates a
//! "this or else that" chain of filters. If the first doesn't succeed, then
//! it tries the other.
//!
//! So, those `math` routes could have been **mounted** all as one, with `or`.
//!
//!
//! ```
//! # use warp::Filter;
//! # let sum = warp::path("sum");
//! # let times = warp::path("times");
//! // GET /math/sum/:u32/:u32
//! // GET /math/:u16/times/:u16
//! let math = warp::path("math")
//!     .and(sum.or(times));
//! ```
//!
//! It turns out, using `or` is how you combine everything together into a
//! single API.
//!
//! ```
//! # use warp::Filter;
//! # let hi = warp::path("hi");
//! # let hello_from_warp = hi.clone();
//! # let bye = hi.clone();
//! # let math = hi.clone();
//! // GET /hi
//! // GET /hello/from/warp
//! // GET /bye/:string
//! // GET /math/sum/:u32/:u32
//! // GET /math/:u16/times/:u16
//! let routes = hi
//!     .or(hello_from_warp)
//!     .or(bye)
//!     .or(math);
//! ```
//!
//! Note that you will generally want path filters to come **before** other filters
//! like `body` or `headers`. If a different type of filter comes first, a request
//! with an invalid body for route `/right-path-wrong-body` may try matching against `/wrong-path`
//! and return the error from `/wrong-path` instead of the correct body-related error.

use std::convert::Infallible;
use std::fmt;
use std::str::FromStr;

use futures_util::future;
use http::uri::PathAndQuery;

use self::internal::Opaque;
use crate::filter::{filter_fn, one, Filter, FilterBase, Internal, One, Tuple};
use crate::reject::{self, Rejection};
use crate::route::{self, Route};

/// Create an exact match path segment [`Filter`](crate::Filter).
///
/// This will try to match exactly to the current request path segment.
///
/// # Note
///
/// - [`end()`](./fn.end.html) should be used to match the end of a path to avoid having
///   filters for shorter paths like `/math` unintentionally match a longer
///   path such as `/math/sum`
/// - Path-related filters should generally come **before** other types of filters, such
///   as those checking headers or body types. Including those other filters before
///   the path checks may result in strange errors being returned because a given request
///   does not match the parameters for a completely separate route.
///
/// # Panics
///
/// Exact path filters cannot be empty, or contain slashes.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// // Matches '/hello'
/// let hello = warp::path("hello")
///     .map(|| "Hello, World!");
/// ```
pub fn path<P>(p: P) -> Exact<Opaque<P>>
where
    P: AsRef<str>,
{
    let s = p.as_ref();
    assert!(!s.is_empty(), "exact path segments should not be empty");
    assert!(
        !s.contains('/'),
        "exact path segments should not contain a slash: {:?}",
        s
    );

    Exact(Opaque(p))
    /*
    segment(move |seg| {
        tracing::trace!("{:?}?: {:?}", p, seg);
        if seg == p {
            Ok(())
        } else {
            Err(reject::not_found())
        }
    })
    */
}

/// A [`Filter`](crate::Filter) matching an exact path segment.
///
/// Constructed from `path()` or `path!()`.
#[allow(missing_debug_implementations)]
#[derive(Clone, Copy)]
pub struct Exact<P>(P);

impl<P> FilterBase for Exact<P>
where
    P: AsRef<str>,
{
    type Extract = ();
    type Error = Rejection;
    type Future = future::Ready<Result<Self::Extract, Self::Error>>;

    #[inline]
    fn filter(&self, _: Internal) -> Self::Future {
        route::with(|route| {
            let p = self.0.as_ref();
            future::ready(with_segment(route, |seg| {
                tracing::trace!("{:?}?: {:?}", p, seg);

                if seg == p {
                    Ok(())
                } else {
                    Err(reject::not_found())
                }
            }))
        })
    }
}

/// Matches the end of a route.
///
/// Note that _not_ including `end()` may result in shorter paths like
/// `/math` unintentionally matching `/math/sum`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// // Matches '/'
/// let hello = warp::path::end()
///     .map(|| "Hello, World!");
/// ```
pub fn end() -> impl Filter<Extract = (), Error = Rejection> + Copy {
    filter_fn(move |route| {
        if route.path().is_empty() {
            future::ok(())
        } else {
            future::err(reject::not_found())
        }
    })
}

/// Extract a parameter from a path segment.
///
/// This will try to parse a value from the current request path
/// segment, and if successful, the value is returned as the `Filter`'s
/// "extracted" value.
///
/// If the value could not be parsed, rejects with a `404 Not Found`.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let route = warp::path::param()
///     .map(|id: u32| {
///         format!("You asked for /{}", id)
///     });
/// ```
pub fn param<T: FromStr + Send + 'static>(
) -> impl Filter<Extract = One<T>, Error = Rejection> + Copy {
    filter_segment(|seg| {
        tracing::trace!("param?: {:?}", seg);
        if seg.is_empty() {
            return Err(reject::not_found());
        }
        T::from_str(seg).map(one).map_err(|_| reject::not_found())
    })
}

/// Extract the unmatched tail of the path.
///
/// This will return a `Tail`, which allows access to the rest of the path
/// that previous filters have not already matched.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let route = warp::path("foo")
///     .and(warp::path::tail())
///     .map(|tail| {
///         // GET /foo/bar/baz would return "bar/baz".
///         format!("The tail after foo is {:?}", tail)
///     });
/// ```
pub fn tail() -> impl Filter<Extract = One<Tail>, Error = Infallible> + Copy {
    filter_fn(move |route| {
        let path = path_and_query(route);
        let idx = route.matched_path_index();

        // Giving the user the full tail means we assume the full path
        // has been matched now.
        let end = path.path().len() - idx;
        route.set_unmatched_path(end);

        future::ok(one(Tail {
            path,
            start_index: idx,
        }))
    })
}

/// Represents the tail part of a request path, returned by the [`tail()`] filter.
pub struct Tail {
    path: PathAndQuery,
    start_index: usize,
}

impl Tail {
    /// Get the `&str` representation of the remaining path.
    pub fn as_str(&self) -> &str {
        &self.path.path()[self.start_index..]
    }
}

impl fmt::Debug for Tail {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

/// Peek at the unmatched tail of the path, without affecting the matched path.
///
/// This will return a `Peek`, which allows access to the rest of the path
/// that previous filters have not already matched. This differs from `tail`
/// in that `peek` will **not** set the entire path as matched.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let route = warp::path("foo")
///     .and(warp::path::peek())
///     .map(|peek| {
///         // GET /foo/bar/baz would return "bar/baz".
///         format!("The path after foo is {:?}", peek)
///     });
/// ```
pub fn peek() -> impl Filter<Extract = One<Peek>, Error = Infallible> + Copy {
    filter_fn(move |route| {
        let path = path_and_query(route);
        let idx = route.matched_path_index();

        future::ok(one(Peek {
            path,
            start_index: idx,
        }))
    })
}

/// Represents the tail part of a request path, returned by the [`peek()`] filter.
pub struct Peek {
    path: PathAndQuery,
    start_index: usize,
}

impl Peek {
    /// Get the `&str` representation of the remaining path.
    pub fn as_str(&self) -> &str {
        &self.path.path()[self.start_index..]
    }

    /// Get an iterator over the segments of the peeked path.
    pub fn segments(&self) -> impl Iterator<Item = &str> {
        self.as_str().split('/').filter(|seg| !seg.is_empty())
    }
}

impl fmt::Debug for Peek {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

/// Returns the full request path, irrespective of other filters.
///
/// This will return a `FullPath`, which can be stringified to return the
/// full path of the request.
///
/// This is more useful in generic pre/post-processing filters, and should
/// probably not be used for request matching/routing.
///
/// # Example
///
/// ```
/// use warp::{Filter, path::FullPath};
/// use std::{collections::HashMap, sync::{Arc, Mutex}};
///
/// let counts = Arc::new(Mutex::new(HashMap::new()));
/// let access_counter = warp::path::full()
///     .map(move |path: FullPath| {
///         let mut counts = counts.lock().unwrap();
///
///         *counts.entry(path.as_str().to_string())
///             .and_modify(|c| *c += 1)
///             .or_insert(0)
///     });
///
/// let route = warp::path("foo")
///     .and(warp::path("bar"))
///     .and(access_counter)
///     .map(|count| {
///         format!("This is the {}th visit to this URL!", count)
///     });
/// ```
pub fn full() -> impl Filter<Extract = One<FullPath>, Error = Infallible> + Copy {
    filter_fn(move |route| future::ok(one(FullPath(path_and_query(route)))))
}

/// Represents the full request path, returned by the [`full()`] filter.
pub struct FullPath(PathAndQuery);

impl FullPath {
    /// Get the `&str` representation of the request path.
    pub fn as_str(&self) -> &str {
        self.0.path()
    }
}

impl fmt::Debug for FullPath {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        fmt::Debug::fmt(self.as_str(), f)
    }
}

fn filter_segment<F, U>(func: F) -> impl Filter<Extract = U, Error = Rejection> + Copy
where
    F: Fn(&str) -> Result<U, Rejection> + Copy,
    U: Tuple + Send + 'static,
{
    filter_fn(move |route| future::ready(with_segment(route, func)))
}

fn with_segment<F, U>(route: &mut Route, func: F) -> Result<U, Rejection>
where
    F: Fn(&str) -> Result<U, Rejection>,
{
    let seg = segment(route);
    let ret = func(seg);
    if ret.is_ok() {
        let idx = seg.len();
        route.set_unmatched_path(idx);
    }
    ret
}

fn segment(route: &Route) -> &str {
    route
        .path()
        .splitn(2, '/')
        .next()
        .expect("split always has at least 1")
}

fn path_and_query(route: &Route) -> PathAndQuery {
    route
        .uri()
        .path_and_query()
        .cloned()
        .unwrap_or_else(|| PathAndQuery::from_static(""))
}

/// Convenient way to chain multiple path filters together.
///
/// Any number of either type identifiers or string expressions can be passed,
/// each separated by a forward slash (`/`). Strings will be used to match
/// path segments exactly, and type identifiers are used just like
/// [`param`](crate::path::param) filters.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// // Match `/sum/:a/:b`
/// let route = warp::path!("sum" / u32 / u32)
///     .map(|a, b| {
///         format!("{} + {} = {}", a, b, a + b)
///     });
/// ```
///
/// The equivalent filter chain without using the `path!` macro looks this:
///
/// ```
/// use warp::Filter;
///
/// let route = warp::path("sum")
///     .and(warp::path::param::<u32>())
///     .and(warp::path::param::<u32>())
///     .and(warp::path::end())
///     .map(|a, b| {
///         format!("{} + {} = {}", a, b, a + b)
///     });
/// ```
///
/// # Path Prefixes
///
/// The `path!` macro automatically assumes the path should include an `end()`
/// filter. To build up a path filter *prefix*, such that the `end()` isn't
/// included, use the `/ ..` syntax.
///
///
/// ```
/// use warp::Filter;
///
/// let prefix = warp::path!("math" / "sum" / ..);
///
/// let sum = warp::path!(u32 / u32)
///     .map(|a, b| {
///         format!("{} + {} = {}", a, b, a + b)
///     });
///
/// let help = warp::path::end()
///     .map(|| "This API returns the sum of two u32's");
///
/// let api = prefix.and(sum.or(help));
/// ```
#[macro_export]
macro_rules! path {
    ($($pieces:tt)*) => ({
        $crate::__internal_path!(@start $($pieces)*)
    });
}

#[doc(hidden)]
#[macro_export]
// not public API
macro_rules! __internal_path {
    (@start) => (
        $crate::path::end()
    );
    (@start ..) => ({
        compile_error!("'..' cannot be the only segment")
    });
    (@start $first:tt $(/ $tail:tt)*) => ({
        $crate::__internal_path!(@munch $crate::any(); [$first] [$(/ $tail)*])
    });

    (@munch $sum:expr; [$cur:tt] [/ $next:tt $(/ $tail:tt)*]) => ({
        $crate::__internal_path!(@munch $crate::Filter::and($sum, $crate::__internal_path!(@segment $cur)); [$next] [$(/ $tail)*])
    });
    (@munch $sum:expr; [$cur:tt] []) => ({
        $crate::__internal_path!(@last $sum; $cur)
    });

    (@last $sum:expr; ..) => (
        $sum
    );
    (@last $sum:expr; $end:tt) => (
        $crate::Filter::and(
            $crate::Filter::and($sum, $crate::__internal_path!(@segment $end)),
            $crate::path::end()
        )
    );

    (@segment ..) => (
        compile_error!("'..' must be the last segment")
    );
    (@segment $param:ty) => (
        $crate::path::param::<$param>()
    );
    // Constructs a unique ZST so the &'static str pointer doesn't need to
    // be carried around.
    (@segment $s:literal) => ({
        #[derive(Clone, Copy)]
        struct __StaticPath;
        impl ::std::convert::AsRef<str> for __StaticPath {
            fn as_ref(&self) -> &str {
                static S: &str = $s;
                S
            }
        }
        $crate::path(__StaticPath)
    });
}

// path! compile fail tests

/// ```compile_fail
/// warp::path!("foo" / .. / "bar");
/// ```
///
/// ```compile_fail
/// warp::path!(.. / "bar");
/// ```
///
/// ```compile_fail
/// warp::path!("foo" ..);
/// ```
///
/// ```compile_fail
/// warp::path!("foo" / .. /);
/// ```
///
/// ```compile_fail
/// warp::path!(..);
/// ```
fn _path_macro_compile_fail() {}

mod internal {
    // Used to prevent users from naming this type.
    //
    // For instance, `Exact<Opaque<String>>` means a user cannot depend
    // on it being `Exact<String>`.
    #[allow(missing_debug_implementations)]
    #[derive(Clone, Copy)]
    pub struct Opaque<T>(pub(super) T);

    impl<T: AsRef<str>> AsRef<str> for Opaque<T> {
        #[inline]
        fn as_ref(&self) -> &str {
            self.0.as_ref()
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_exact_size() {
        use std::mem::{size_of, size_of_val};

        assert_eq!(
            size_of_val(&path("hello")),
            size_of::<&str>(),
            "exact(&str) is size of &str"
        );

        assert_eq!(
            size_of_val(&path(String::from("world"))),
            size_of::<String>(),
            "exact(String) is size of String"
        );

        assert_eq!(
            size_of_val(&path!("zst")),
            size_of::<()>(),
            "path!(&str) is ZST"
        );
    }
}
