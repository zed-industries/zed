//! Additions to the [`TryStream`] trait.
//!
//! [`TryStream`]: futures_core_crate::TryStream

use crate::{Error, ErrorCompat, IntoError};
use core::{
    marker::PhantomData,
    pin::Pin,
    task::{Context as TaskContext, Poll},
};
use futures_core_crate::stream::{Stream, TryStream};
use pin_project::pin_project;

#[cfg(any(feature = "alloc", test))]
use alloc::string::String;

#[cfg(any(feature = "alloc", test))]
use crate::FromString;

/// Additions to [`TryStream`].
pub trait TryStreamExt: TryStream + Sized {
    /// Extend a [`TryStream`]'s error with additional context-sensitive
    /// information.
    ///
    /// ```rust
    /// # use futures_crate as futures;
    /// use futures::TryStream;
    /// # use futures::stream;
    /// use snafu::prelude::*;
    ///
    /// #[derive(Debug, Snafu)]
    /// enum Error {
    ///     Authenticating {
    ///         user_name: String,
    ///         user_id: i32,
    ///         source: ApiError,
    ///     },
    /// }
    ///
    /// fn example() -> impl TryStream<Ok = i32, Error = Error> {
    ///     stock_prices().context(AuthenticatingSnafu {
    ///         user_name: "admin",
    ///         user_id: 42,
    ///     })
    /// }
    ///
    /// # type ApiError = Box<dyn std::error::Error>;
    /// fn stock_prices() -> impl TryStream<Ok = i32, Error = ApiError> {
    ///     /* ... */
    /// # stream::empty()
    /// }
    /// ```
    ///
    /// Note that the context selector will call [`Into::into`] on
    /// each field, so the types are not required to exactly match.
    fn context<C, E>(self, context: C) -> Context<Self, C, E>
    where
        C: IntoError<E, Source = Self::Error> + Clone,
        E: Error + ErrorCompat;

    /// Extend a [`TryStream`]'s error with lazily-generated
    /// context-sensitive information.
    ///
    /// ```rust
    /// # use futures_crate as futures;
    /// use futures::TryStream;
    /// # use futures::stream;
    /// use snafu::prelude::*;
    ///
    /// #[derive(Debug, Snafu)]
    /// enum Error {
    ///     Authenticating {
    ///         user_name: String,
    ///         user_id: i32,
    ///         source: ApiError,
    ///     },
    /// }
    ///
    /// fn example() -> impl TryStream<Ok = i32, Error = Error> {
    ///     stock_prices().with_context(|_| AuthenticatingSnafu {
    ///         user_name: "admin",
    ///         user_id: 42,
    ///     })
    /// }
    ///
    /// # type ApiError = Box<dyn std::error::Error>;
    /// fn stock_prices() -> impl TryStream<Ok = i32, Error = ApiError> {
    ///     /* ... */
    /// # stream::empty()
    /// }
    /// ```
    ///
    /// Note that this *may not* be needed in many cases because the
    /// context selector will call [`Into::into`] on each field.
    fn with_context<F, C, E>(self, context: F) -> WithContext<Self, F, E>
    where
        F: FnMut(&mut Self::Error) -> C,
        C: IntoError<E, Source = Self::Error>,
        E: Error + ErrorCompat;

    /// Extend a [`TryStream`]'s error with information from a string.
    ///
    /// The target error type must implement [`FromString`] by using
    /// the
    /// [`#[snafu(whatever)]`][crate::Snafu#controlling-stringly-typed-errors]
    /// attribute. The premade [`Whatever`](crate::Whatever) type is also available.
    ///
    /// In many cases, you will want to use
    /// [`with_whatever_context`][Self::with_whatever_context] instead
    /// as it is only called in case of error. This method is best
    /// suited for when you have a string literal.
    ///
    /// ```rust
    /// # use futures_crate as futures;
    /// use futures::TryStream;
    /// # use futures::stream;
    /// use snafu::{prelude::*, Whatever};
    ///
    /// fn example() -> impl TryStream<Ok = i32, Error = Whatever> {
    ///     stock_prices().whatever_context("Couldn't get stock prices")
    /// }
    ///
    /// # type ApiError = Box<dyn std::error::Error>;
    /// fn stock_prices() -> impl TryStream<Ok = i32, Error = ApiError> {
    ///     /* ... */
    /// # stream::empty()
    /// }
    /// ```
    #[cfg(any(feature = "alloc", test))]
    fn whatever_context<S, E>(self, context: S) -> WhateverContext<Self, S, E>
    where
        S: Into<String>,
        E: FromString;

    /// Extend a [`TryStream`]'s error with information from a
    /// lazily-generated string.
    ///
    /// The target error type must implement [`FromString`] by using
    /// the
    /// [`#[snafu(whatever)]`][crate::Snafu#controlling-stringly-typed-errors]
    /// attribute. The premade [`Whatever`](crate::Whatever) type is also available.
    ///
    /// ```rust
    /// # use futures_crate as futures;
    /// use futures::TryStream;
    /// # use futures::stream;
    /// use snafu::{prelude::*, Whatever};
    ///
    /// fn example(symbol: &'static str) -> impl TryStream<Ok = i32, Error = Whatever> {
    ///     stock_prices(symbol)
    ///         .with_whatever_context(move |_| format!("Couldn't get stock prices for {symbol}"))
    /// }
    ///
    /// # type ApiError = Box<dyn std::error::Error>;
    /// fn stock_prices(symbol: &'static str) -> impl TryStream<Ok = i32, Error = ApiError> {
    ///     /* ... */
    /// # stream::empty()
    /// }
    /// ```
    #[cfg(any(feature = "alloc", test))]
    fn with_whatever_context<F, S, E>(self, context: F) -> WithWhateverContext<Self, F, E>
    where
        F: FnMut(&mut Self::Error) -> S,
        S: Into<String>,
        E: FromString;
}

impl<St> TryStreamExt for St
where
    St: TryStream,
{
    fn context<C, E>(self, context: C) -> Context<Self, C, E>
    where
        C: IntoError<E, Source = Self::Error> + Clone,
        E: Error + ErrorCompat,
    {
        Context {
            inner: self,
            context,
            _e: PhantomData,
        }
    }

    fn with_context<F, C, E>(self, context: F) -> WithContext<Self, F, E>
    where
        F: FnMut(&mut Self::Error) -> C,
        C: IntoError<E, Source = Self::Error>,
        E: Error + ErrorCompat,
    {
        WithContext {
            inner: self,
            context,
            _e: PhantomData,
        }
    }

    #[cfg(any(feature = "alloc", test))]
    fn whatever_context<S, E>(self, context: S) -> WhateverContext<Self, S, E>
    where
        S: Into<String>,
        E: FromString,
    {
        WhateverContext {
            inner: self,
            context,
            _e: PhantomData,
        }
    }

    #[cfg(any(feature = "alloc", test))]
    fn with_whatever_context<F, S, E>(self, context: F) -> WithWhateverContext<Self, F, E>
    where
        F: FnMut(&mut Self::Error) -> S,
        S: Into<String>,
        E: FromString,
    {
        WithWhateverContext {
            inner: self,
            context,
            _e: PhantomData,
        }
    }
}

/// Stream for the [`context`](TryStreamExt::context) combinator.
///
/// See the [`TryStreamExt::context`] method for more details.
#[pin_project]
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct Context<St, C, E> {
    #[pin]
    inner: St,
    context: C,
    _e: PhantomData<E>,
}

impl<St, C, E> Stream for Context<St, C, E>
where
    St: TryStream,
    C: IntoError<E, Source = St::Error> + Clone,
    E: Error + ErrorCompat,
{
    type Item = Result<St::Ok, E>;

    #[track_caller]
    fn poll_next(self: Pin<&mut Self>, ctx: &mut TaskContext) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let inner = this.inner;
        let context = this.context;

        match inner.try_poll_next(ctx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Ok(v))) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(Some(Err(error))) => {
                let error = context.clone().into_error(error);
                Poll::Ready(Some(Err(error)))
            }
        }
    }
}

/// Stream for the [`with_context`](TryStreamExt::with_context) combinator.
///
/// See the [`TryStreamExt::with_context`] method for more details.
#[pin_project]
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
pub struct WithContext<St, F, E> {
    #[pin]
    inner: St,
    context: F,
    _e: PhantomData<E>,
}

impl<St, F, C, E> Stream for WithContext<St, F, E>
where
    St: TryStream,
    F: FnMut(&mut St::Error) -> C,
    C: IntoError<E, Source = St::Error>,
    E: Error + ErrorCompat,
{
    type Item = Result<St::Ok, E>;

    #[track_caller]
    fn poll_next(self: Pin<&mut Self>, ctx: &mut TaskContext) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let inner = this.inner;
        let context = this.context;

        match inner.try_poll_next(ctx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Ok(v))) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(Some(Err(mut error))) => {
                let error = context(&mut error).into_error(error);
                Poll::Ready(Some(Err(error)))
            }
        }
    }
}

/// Stream for the
/// [`whatever_context`](TryStreamExt::whatever_context) combinator.
///
/// See the [`TryStreamExt::whatever_context`] method for more
/// details.
#[pin_project]
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
#[cfg(any(feature = "alloc", test))]
pub struct WhateverContext<St, S, E> {
    #[pin]
    inner: St,
    context: S,
    _e: PhantomData<E>,
}

#[cfg(any(feature = "alloc", test))]
impl<St, S, E> Stream for WhateverContext<St, S, E>
where
    St: TryStream,
    S: Into<String> + Clone,
    E: FromString,
    St::Error: Into<E::Source>,
{
    type Item = Result<St::Ok, E>;

    #[track_caller]
    fn poll_next(self: Pin<&mut Self>, ctx: &mut TaskContext) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let inner = this.inner;
        let context = this.context;

        match inner.try_poll_next(ctx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Ok(v))) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(Some(Err(error))) => {
                let error = E::with_source(error.into(), context.clone().into());
                Poll::Ready(Some(Err(error)))
            }
        }
    }
}

/// Stream for the
/// [`with_whatever_context`](TryStreamExt::with_whatever_context)
/// combinator.
///
/// See the [`TryStreamExt::with_whatever_context`] method for more
/// details.
#[pin_project]
#[derive(Debug)]
#[must_use = "streams do nothing unless polled"]
#[cfg(any(feature = "alloc", test))]
pub struct WithWhateverContext<St, F, E> {
    #[pin]
    inner: St,
    context: F,
    _e: PhantomData<E>,
}

#[cfg(any(feature = "alloc", test))]
impl<St, F, S, E> Stream for WithWhateverContext<St, F, E>
where
    St: TryStream,
    F: FnMut(&mut St::Error) -> S,
    S: Into<String>,
    E: FromString,
    St::Error: Into<E::Source>,
{
    type Item = Result<St::Ok, E>;

    #[track_caller]
    fn poll_next(self: Pin<&mut Self>, ctx: &mut TaskContext) -> Poll<Option<Self::Item>> {
        let this = self.project();
        let inner = this.inner;
        let context = this.context;

        match inner.try_poll_next(ctx) {
            Poll::Pending => Poll::Pending,
            Poll::Ready(None) => Poll::Ready(None),
            Poll::Ready(Some(Ok(v))) => Poll::Ready(Some(Ok(v))),
            Poll::Ready(Some(Err(mut error))) => {
                let context = context(&mut error);
                let error = E::with_source(error.into(), context.into());
                Poll::Ready(Some(Err(error)))
            }
        }
    }
}
