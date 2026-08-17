//! A filter that matches any route.
use std::convert::Infallible;
use std::future::Future;
use std::pin::Pin;
use std::task::{Context, Poll};

use crate::filter::{Filter, FilterBase, Internal};

/// A [`Filter`](crate::Filter) that matches any route.
///
/// This can be a useful building block to build new filters from,
/// since [`Filter`] is otherwise a sealed trait.
///
/// # Example
///
/// ```
/// use warp::Filter;
///
/// let route = warp::any()
///     .map(|| {
///         "I always return this string!"
///     });
/// ```
///
/// This could allow creating a single `impl Filter` returning a specific
/// reply, that can then be used as the end of several different filter
/// chains.
///
/// Another use case is turning some clone-able resource into a `Filter`,
/// thus allowing to easily `and` it together with others.
///
/// ```
/// use std::sync::Arc;
/// use warp::Filter;
///
/// let state = Arc::new(vec![33, 41]);
/// let with_state = warp::any().map(move || state.clone());
///
/// // Now we could `and` with any other filter:
///
/// let route = warp::path::param()
///     .and(with_state)
///     .map(|param_id: u32, db: Arc<Vec<u32>>| {
///         db.contains(&param_id)
///     });
/// ```
pub fn any() -> impl Filter<Extract = (), Error = Infallible> + Copy {
    Any
}

#[derive(Copy, Clone)]
#[allow(missing_debug_implementations)]
struct Any;

impl FilterBase for Any {
    type Extract = ();
    type Error = Infallible;
    type Future = AnyFut;

    #[inline]
    fn filter(&self, _: Internal) -> Self::Future {
        AnyFut
    }
}

#[allow(missing_debug_implementations)]
struct AnyFut;

impl Future for AnyFut {
    type Output = Result<(), Infallible>;

    #[inline]
    fn poll(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Self::Output> {
        Poll::Ready(Ok(()))
    }
}
