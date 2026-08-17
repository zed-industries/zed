//! Query Filters

use futures_util::future;
use serde::de::DeserializeOwned;

use crate::filter::{filter_fn_one, Filter, One};
use crate::reject::{self, Rejection};

/// Creates a `Filter` that decodes query parameters to the type `T`.
///
/// If cannot decode into a `T`, the request is rejected with a `400 Bad Request`.
///
/// # Example
///
/// ```
/// use std::collections::HashMap;
/// use warp::{
///     http::Response,
///     Filter,
/// };
///
/// let route = warp::any()
///     .and(warp::query::<HashMap<String, String>>())
///     .map(|map: HashMap<String, String>| {
///         let mut response: Vec<String> = Vec::new();
///         for (key, value) in map.into_iter() {
///             response.push(format!("{}={}", key, value))
///         }
///         Response::builder().body(response.join(";"))
///     });
/// ```
///
/// You can define your custom query object and deserialize with [Serde][Serde]. Ensure to include
/// the crate in your dependencies before usage.
///
/// ```
/// use serde_derive::{Deserialize, Serialize};
/// use std::collections::HashMap;
/// use warp::{
///     http::Response,
///     Filter,
/// };
///
/// #[derive(Serialize, Deserialize)]
/// struct FooQuery {
///     foo: Option<String>,
///     bar: u8,
/// }
///
/// let route = warp::any()
///     .and(warp::query::<FooQuery>())
///     .map(|q: FooQuery| {
///         if let Some(foo) = q.foo {
///             Response::builder().body(format!("foo={}", foo))
///         } else {
///             Response::builder().body(format!("bar={}", q.bar))
///         }
///     });
/// ```
///
/// For more examples, please take a look at [examples/query_string.rs](https://github.com/seanmonstar/warp/blob/master/examples/query_string.rs).
///
/// [Serde]: https://docs.rs/serde
pub fn query<T: DeserializeOwned + Send + 'static>(
) -> impl Filter<Extract = One<T>, Error = Rejection> + Copy {
    filter_fn_one(|route| {
        let query_string = route.query().unwrap_or_else(|| {
            tracing::debug!("route was called without a query string, defaulting to empty");
            ""
        });

        let query_encoded = serde_urlencoded::from_str(query_string).map_err(|e| {
            tracing::debug!("failed to decode query string '{}': {:?}", query_string, e);
            reject::invalid_query()
        });
        future::ready(query_encoded)
    })
}

/// Creates a `Filter` that returns the raw query string as type String.
pub fn raw() -> impl Filter<Extract = One<String>, Error = Rejection> + Copy {
    filter_fn_one(|route| {
        let route = route
            .query()
            .map(|q| q.to_owned())
            .map(Ok)
            .unwrap_or_else(|| Err(reject::invalid_query()));
        future::ready(route)
    })
}
