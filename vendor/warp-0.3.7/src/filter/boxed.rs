use std::fmt;
use std::future::Future;
use std::pin::Pin;
use std::sync::Arc;

use futures_util::TryFutureExt;

use super::{Filter, FilterBase, Internal, Tuple};
use crate::reject::Rejection;

/// A type representing a boxed [`Filter`](crate::Filter) trait object.
///
/// The filter inside is a dynamic trait object. The purpose of this type is
/// to ease returning `Filter`s from other functions.
///
/// To create one, call `Filter::boxed` on any filter.
///
/// # Examples
///
/// ```
/// use warp::{Filter, filters::BoxedFilter, Reply};
///
/// pub fn assets_filter() -> BoxedFilter<(impl Reply,)> {
///     warp::path("assets")
///         .and(warp::fs::dir("./assets"))
///         .boxed()
/// }
/// ```
///
pub struct BoxedFilter<T: Tuple> {
    filter: Arc<
        dyn Filter<
                Extract = T,
                Error = Rejection,
                Future = Pin<Box<dyn Future<Output = Result<T, Rejection>> + Send>>,
            > + Send
            + Sync,
    >,
}

impl<T: Tuple + Send> BoxedFilter<T> {
    pub(super) fn new<F>(filter: F) -> BoxedFilter<T>
    where
        F: Filter<Extract = T> + Send + Sync + 'static,
        F::Error: Into<Rejection>,
    {
        BoxedFilter {
            filter: Arc::new(BoxingFilter {
                filter: filter.map_err(super::Internal, Into::into),
            }),
        }
    }
}

impl<T: Tuple> Clone for BoxedFilter<T> {
    fn clone(&self) -> BoxedFilter<T> {
        BoxedFilter {
            filter: self.filter.clone(),
        }
    }
}

impl<T: Tuple> fmt::Debug for BoxedFilter<T> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_struct("BoxedFilter").finish()
    }
}

fn _assert_send() {
    fn _assert<T: Send>() {}
    _assert::<BoxedFilter<()>>();
}

impl<T: Tuple + Send> FilterBase for BoxedFilter<T> {
    type Extract = T;
    type Error = Rejection;
    type Future = Pin<Box<dyn Future<Output = Result<T, Rejection>> + Send>>;

    fn filter(&self, _: Internal) -> Self::Future {
        self.filter.filter(Internal)
    }
}

struct BoxingFilter<F> {
    filter: F,
}

impl<F> FilterBase for BoxingFilter<F>
where
    F: Filter,
    F::Future: Send + 'static,
{
    type Extract = F::Extract;
    type Error = F::Error;
    type Future = Pin<Box<dyn Future<Output = Result<Self::Extract, Self::Error>> + Send>>;

    fn filter(&self, _: Internal) -> Self::Future {
        Box::pin(self.filter.filter(Internal).into_future())
    }
}
