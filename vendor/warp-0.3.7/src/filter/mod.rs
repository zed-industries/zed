mod and;
mod and_then;
mod boxed;
mod map;
mod map_err;
mod or;
mod or_else;
mod recover;
pub(crate) mod service;
mod then;
mod unify;
mod untuple_one;
mod wrap;

use std::future::Future;

use futures_util::{future, TryFuture, TryFutureExt};

pub(crate) use crate::generic::{one, Combine, Either, Func, One, Tuple};
use crate::reject::{CombineRejection, IsReject, Rejection};
use crate::route::{self, Route};

pub(crate) use self::and::And;
use self::and_then::AndThen;
pub use self::boxed::BoxedFilter;
pub(crate) use self::map::Map;
pub(crate) use self::map_err::MapErr;
pub(crate) use self::or::Or;
use self::or_else::OrElse;
use self::recover::Recover;
use self::then::Then;
use self::unify::Unify;
use self::untuple_one::UntupleOne;
pub use self::wrap::wrap_fn;
pub(crate) use self::wrap::{Wrap, WrapSealed};

// A crate-private base trait, allowing the actual `filter` method to change
// signatures without it being a breaking change.
pub trait FilterBase {
    type Extract: Tuple; // + Send;
    type Error: IsReject;
    type Future: Future<Output = Result<Self::Extract, Self::Error>> + Send;

    fn filter(&self, internal: Internal) -> Self::Future;

    fn map_err<F, E>(self, _internal: Internal, fun: F) -> MapErr<Self, F>
    where
        Self: Sized,
        F: Fn(Self::Error) -> E + Clone,
        E: ::std::fmt::Debug + Send,
    {
        MapErr {
            filter: self,
            callback: fun,
        }
    }
}

// A crate-private argument to prevent users from calling methods on
// the `FilterBase` trait.
//
// For instance, this innocent user code could otherwise call `filter`:
//
// ```
// async fn with_filter<F: Filter>(f: F) -> Result<F::Extract, F::Error> {
//     f.filter().await
// }
// ```
#[allow(missing_debug_implementations)]
pub struct Internal;

/// Composable request filters.
///
/// A `Filter` can optionally extract some data from a request, combine
/// it with others, mutate it, and return back some value as a reply. The
/// power of `Filter`s come from being able to isolate small subsets, and then
/// chain and reuse them in various parts of your app.
///
/// # Extracting Tuples
///
/// You may notice that several of these filters extract some tuple, often
/// times a tuple of just 1 item! Why?
///
/// If a filter extracts a `(String,)`, that simply means that it
/// extracts a `String`. If you were to `map` the filter, the argument type
/// would be exactly that, just a `String`.
///
/// What is it? It's just some type magic that allows for automatic combining
/// and flattening of tuples. Without it, combining two filters together with
/// `and`, where one extracted `()`, and another `String`, would mean the
/// `map` would be given a single argument of `((), String,)`, which is just
/// no fun.
pub trait Filter: FilterBase {
    /// Composes a new `Filter` that requires both this and the other to filter a request.
    ///
    /// Additionally, this will join together the extracted values of both
    /// filters, so that `map` and `and_then` receive them as separate arguments.
    ///
    /// If a `Filter` extracts nothing (so, `()`), combining with any other
    /// filter will simply discard the `()`. If a `Filter` extracts one or
    /// more items, combining will mean it extracts the values of itself
    /// combined with the other.
    ///
    /// # Example
    ///
    /// ```
    /// use warp::Filter;
    ///
    /// // Match `/hello/:name`...
    /// warp::path("hello")
    ///     .and(warp::path::param::<String>());
    /// ```
    fn and<F>(self, other: F) -> And<Self, F>
    where
        Self: Sized,
        <Self::Extract as Tuple>::HList: Combine<<F::Extract as Tuple>::HList>,
        F: Filter + Clone,
        F::Error: CombineRejection<Self::Error>,
    {
        And {
            first: self,
            second: other,
        }
    }

    /// Composes a new `Filter` of either this or the other filter.
    ///
    /// # Example
    ///
    /// ```
    /// use std::net::SocketAddr;
    /// use warp::Filter;
    ///
    /// // Match either `/:u32` or `/:socketaddr`
    /// warp::path::param::<u32>()
    ///     .or(warp::path::param::<SocketAddr>());
    /// ```
    fn or<F>(self, other: F) -> Or<Self, F>
    where
        Self: Filter<Error = Rejection> + Sized,
        F: Filter,
        F::Error: CombineRejection<Self::Error>,
    {
        Or {
            first: self,
            second: other,
        }
    }

    /// Composes this `Filter` with a function receiving the extracted value.
    ///
    ///
    /// # Example
    ///
    /// ```
    /// use warp::Filter;
    ///
    /// // Map `/:id`
    /// warp::path::param().map(|id: u64| {
    ///   format!("Hello #{}", id)
    /// });
    /// ```
    ///
    /// # `Func`
    ///
    /// The generic `Func` trait is implemented for any function that receives
    /// the same arguments as this `Filter` extracts. In practice, this
    /// shouldn't ever bother you, and simply makes things feel more natural.
    ///
    /// For example, if three `Filter`s were combined together, suppose one
    /// extracts nothing (so `()`), and the other two extract two integers,
    /// a function that accepts exactly two integer arguments is allowed.
    /// Specifically, any `Fn(u32, u32)`.
    ///
    /// Without `Product` and `Func`, this would be a lot messier. First of
    /// all, the `()`s couldn't be discarded, and the tuples would be nested.
    /// So, instead, you'd need to pass an `Fn(((), (u32, u32)))`. That's just
    /// a single argument. Bleck!
    ///
    /// Even worse, the tuples would shuffle the types around depending on
    /// the exact invocation of `and`s. So, `unit.and(int).and(int)` would
    /// result in a different extracted type from `unit.and(int.and(int))`,
    /// or from `int.and(unit).and(int)`. If you changed around the order
    /// of filters, while still having them be semantically equivalent, you'd
    /// need to update all your `map`s as well.
    ///
    /// `Product`, `HList`, and `Func` do all the heavy work so that none of
    /// this is a bother to you. What's more, the types are enforced at
    /// compile-time, and tuple flattening is optimized away to nothing by
    /// LLVM.
    fn map<F>(self, fun: F) -> Map<Self, F>
    where
        Self: Sized,
        F: Func<Self::Extract> + Clone,
    {
        Map {
            filter: self,
            callback: fun,
        }
    }

    /// Composes this `Filter` with an async function receiving
    /// the extracted value.
    ///
    /// The function should return some `Future` type.
    ///
    /// # Example
    ///
    /// ```
    /// use warp::Filter;
    ///
    /// // Map `/:id`
    /// warp::path::param().then(|id: u64| async move {
    ///   format!("Hello #{}", id)
    /// });
    /// ```
    fn then<F>(self, fun: F) -> Then<Self, F>
    where
        Self: Sized,
        F: Func<Self::Extract> + Clone,
        F::Output: Future + Send,
    {
        Then {
            filter: self,
            callback: fun,
        }
    }

    /// Composes this `Filter` with a fallible async function receiving
    /// the extracted value.
    ///
    /// The function should return some `TryFuture` type.
    ///
    /// The `Error` type of the return `Future` needs be a `Rejection`, which
    /// means most futures will need to have their error mapped into one.
    ///
    /// Rejections are meant to say "this filter didn't accept the request,
    /// maybe another can". So for application-level errors, consider using
    /// [`Filter::then`] instead.
    ///
    /// # Example
    ///
    /// ```
    /// use warp::Filter;
    ///
    /// // Validate after `/:id`
    /// warp::path::param().and_then(|id: u64| async move {
    ///     if id != 0 {
    ///         Ok(format!("Hello #{}", id))
    ///     } else {
    ///         Err(warp::reject::not_found())
    ///     }
    /// });
    /// ```
    fn and_then<F>(self, fun: F) -> AndThen<Self, F>
    where
        Self: Sized,
        F: Func<Self::Extract> + Clone,
        F::Output: TryFuture + Send,
        <F::Output as TryFuture>::Error: CombineRejection<Self::Error>,
    {
        AndThen {
            filter: self,
            callback: fun,
        }
    }

    /// Compose this `Filter` with a function receiving an error.
    ///
    /// The function should return some `TryFuture` type yielding the
    /// same item and error types.
    fn or_else<F>(self, fun: F) -> OrElse<Self, F>
    where
        Self: Filter<Error = Rejection> + Sized,
        F: Func<Rejection>,
        F::Output: TryFuture<Ok = Self::Extract> + Send,
        <F::Output as TryFuture>::Error: IsReject,
    {
        OrElse {
            filter: self,
            callback: fun,
        }
    }

    /// Compose this `Filter` with a function receiving an error and
    /// returning a *new* type, instead of the *same* type.
    ///
    /// This is useful for "customizing" rejections into new response types.
    /// See also the [rejections example][ex].
    ///
    /// [ex]: https://github.com/seanmonstar/warp/blob/master/examples/rejections.rs
    fn recover<F>(self, fun: F) -> Recover<Self, F>
    where
        Self: Filter<Error = Rejection> + Sized,
        F: Func<Rejection>,
        F::Output: TryFuture + Send,
        <F::Output as TryFuture>::Error: IsReject,
    {
        Recover {
            filter: self,
            callback: fun,
        }
    }

    /// Unifies the extracted value of `Filter`s composed with `or`.
    ///
    /// When a `Filter` extracts some `Either<T, T>`, where both sides
    /// are the same type, this combinator can be used to grab the
    /// inner value, regardless of which side of `Either` it was. This
    /// is useful for values that could be extracted from multiple parts
    /// of a request, and the exact place isn't important.
    ///
    /// # Example
    ///
    /// ```rust
    /// use std::net::SocketAddr;
    /// use warp::Filter;
    ///
    /// let client_ip = warp::header("x-real-ip")
    ///     .or(warp::header("x-forwarded-for"))
    ///     .unify()
    ///     .map(|ip: SocketAddr| {
    ///         // Get the IP from either header,
    ///         // and unify into the inner type.
    ///     });
    /// ```
    fn unify<T>(self) -> Unify<Self>
    where
        Self: Filter<Extract = (Either<T, T>,)> + Sized,
        T: Tuple,
    {
        Unify { filter: self }
    }

    /// Convenience method to remove one layer of tupling.
    ///
    /// This is useful for when things like `map` don't return a new value,
    /// but just `()`, since warp will wrap it up into a `((),)`.
    ///
    /// # Example
    ///
    /// ```
    /// use warp::Filter;
    ///
    /// let route = warp::path::param()
    ///     .map(|num: u64| {
    ///         println!("just logging: {}", num);
    ///         // returning "nothing"
    ///     })
    ///     .untuple_one()
    ///     .map(|| {
    ///         println!("the ((),) was removed");
    ///         warp::reply()
    ///     });
    /// ```
    ///
    /// ```
    /// use warp::Filter;
    ///
    /// let route = warp::any()
    ///     .map(|| {
    ///         // wanting to return a tuple
    ///         (true, 33)
    ///     })
    ///     .untuple_one()
    ///     .map(|is_enabled: bool, count: i32| {
    ///         println!("untupled: ({}, {})", is_enabled, count);
    ///     });
    /// ```
    fn untuple_one<T>(self) -> UntupleOne<Self>
    where
        Self: Filter<Extract = (T,)> + Sized,
        T: Tuple,
    {
        UntupleOne { filter: self }
    }

    /// Wraps the current filter with some wrapper.
    ///
    /// The wrapper may do some preparation work before starting this filter,
    /// and may do post-processing after the filter completes.
    ///
    /// # Example
    ///
    /// ```
    /// use warp::Filter;
    ///
    /// let route = warp::any()
    ///     .map(warp::reply);
    ///
    /// // Wrap the route with a log wrapper.
    /// let route = route.with(warp::log("example"));
    /// ```
    fn with<W>(self, wrapper: W) -> W::Wrapped
    where
        Self: Sized,
        W: Wrap<Self>,
    {
        wrapper.wrap(self)
    }

    /// Boxes this filter into a trait object, making it easier to name the type.
    ///
    /// # Example
    ///
    /// ```
    /// use warp::Filter;
    ///
    /// fn impl_reply() -> warp::filters::BoxedFilter<(impl warp::Reply,)> {
    ///     warp::any()
    ///         .map(warp::reply)
    ///         .boxed()
    /// }
    ///
    /// fn named_i32() -> warp::filters::BoxedFilter<(i32,)> {
    ///     warp::path::param::<i32>()
    ///         .boxed()
    /// }
    ///
    /// fn named_and() -> warp::filters::BoxedFilter<(i32, String)> {
    ///     warp::path::param::<i32>()
    ///         .and(warp::header::<String>("host"))
    ///         .boxed()
    /// }
    /// ```
    fn boxed(self) -> BoxedFilter<Self::Extract>
    where
        Self: Sized + Send + Sync + 'static,
        Self::Extract: Send,
        Self::Error: Into<Rejection>,
    {
        BoxedFilter::new(self)
    }
}

impl<T: FilterBase> Filter for T {}

pub trait FilterClone: Filter + Clone {}

impl<T: Filter + Clone> FilterClone for T {}

fn _assert_object_safe() {
    fn _assert(_f: &dyn Filter<Extract = (), Error = (), Future = future::Ready<()>>) {}
}

// ===== FilterFn =====

pub(crate) fn filter_fn<F, U>(func: F) -> FilterFn<F>
where
    F: Fn(&mut Route) -> U,
    U: TryFuture,
    U::Ok: Tuple,
    U::Error: IsReject,
{
    FilterFn { func }
}

pub(crate) fn filter_fn_one<F, U>(
    func: F,
) -> impl Filter<Extract = (U::Ok,), Error = U::Error> + Copy
where
    F: Fn(&mut Route) -> U + Copy,
    U: TryFuture + Send + 'static,
    U::Ok: Send,
    U::Error: IsReject,
{
    filter_fn(move |route| func(route).map_ok(|item| (item,)))
}

#[derive(Copy, Clone)]
#[allow(missing_debug_implementations)]
pub(crate) struct FilterFn<F> {
    // TODO: could include a `debug_str: &'static str` to be used in Debug impl
    func: F,
}

impl<F, U> FilterBase for FilterFn<F>
where
    F: Fn(&mut Route) -> U,
    U: TryFuture + Send + 'static,
    U::Ok: Tuple + Send,
    U::Error: IsReject,
{
    type Extract = U::Ok;
    type Error = U::Error;
    type Future = future::IntoFuture<U>;

    #[inline]
    fn filter(&self, _: Internal) -> Self::Future {
        route::with(|route| (self.func)(route)).into_future()
    }
}
