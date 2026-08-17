//! Additions to the [`TryFuture`] trait.
//!
//! [`TryFuture`]: futures_core_crate::future::TryFuture

use crate::{Error, ErrorCompat, IntoError};
use core::{
    future::Future,
    marker::PhantomData,
    pin::Pin,
    task::{Context as TaskContext, Poll},
};
use futures_core_crate::future::TryFuture;
use pin_project::pin_project;

#[cfg(feature = "alloc")]
use alloc::string::String;

#[cfg(feature = "alloc")]
use crate::FromString;

/// Additions to [`TryFuture`].
pub trait TryFutureExt: TryFuture + Sized {
    /// Extend a [`TryFuture`]'s error with additional context-sensitive
    /// information.
    ///
    /// ```rust
    /// # use futures_crate as futures;
    /// use futures::future::TryFuture;
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
    /// fn example() -> impl TryFuture<Ok = i32, Error = Error> {
    ///     another_function().context(AuthenticatingSnafu {
    ///         user_name: "admin",
    ///         user_id: 42,
    ///     })
    /// }
    ///
    /// # type ApiError = Box<dyn std::error::Error>;
    /// fn another_function() -> impl TryFuture<Ok = i32, Error = ApiError> {
    ///     /* ... */
    /// # futures::future::ok(42)
    /// }
    /// ```
    ///
    /// Note that the context selector will call [`Into::into`] on
    /// each field, so the types are not required to exactly match.
    fn context<C, E>(self, context: C) -> Context<Self, C, E>
    where
        C: IntoError<E, Source = Self::Error>,
        E: Error + ErrorCompat;

    /// Extend a [`TryFuture`]'s error with lazily-generated context-sensitive
    /// information.
    ///
    /// ```rust
    /// # use futures_crate as futures;
    /// use futures::future::TryFuture;
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
    /// fn example() -> impl TryFuture<Ok = i32, Error = Error> {
    ///     another_function().with_context(|_| AuthenticatingSnafu {
    ///         user_name: "admin".to_string(),
    ///         user_id: 42,
    ///     })
    /// }
    ///
    /// # type ApiError = Box<dyn std::error::Error>;
    /// fn another_function() -> impl TryFuture<Ok = i32, Error = ApiError> {
    ///     /* ... */
    /// # futures::future::ok(42)
    /// }
    /// ```
    ///
    /// Note that this *may not* be needed in many cases because the
    /// context selector will call [`Into::into`] on each field.
    fn with_context<F, C, E>(self, context: F) -> WithContext<Self, F, E>
    where
        F: FnOnce(&mut Self::Error) -> C,
        C: IntoError<E, Source = Self::Error>,
        E: Error + ErrorCompat;

    /// Extend a [`TryFuture`]'s error with information from a string.
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
    /// use futures::future::TryFuture;
    /// use snafu::{prelude::*, Whatever};
    ///
    /// fn example() -> impl TryFuture<Ok = i32, Error = Whatever> {
    ///     api_function().whatever_context("The API failed")
    /// }
    ///
    /// # type ApiError = Box<dyn std::error::Error>;
    /// fn api_function() -> impl TryFuture<Ok = i32, Error = ApiError> {
    ///     /* ... */
    /// # futures::future::ok(42)
    /// }
    /// ```
    #[cfg(any(feature = "alloc", test))]
    fn whatever_context<S, E>(self, context: S) -> WhateverContext<Self, S, E>
    where
        S: Into<String>,
        E: FromString;

    /// Extend a [`TryFuture`]'s error with information from a
    /// lazily-generated string.
    ///
    /// The target error type must implement [`FromString`] by using
    /// the
    /// [`#[snafu(whatever)]`][crate::Snafu#controlling-stringly-typed-errors]
    /// attribute. The premade [`Whatever`](crate::Whatever) type is also available.
    ///
    /// ```rust
    /// # use futures_crate as futures;
    /// use futures::future::TryFuture;
    /// use snafu::{prelude::*, Whatever};
    ///
    /// fn example(arg: &'static str) -> impl TryFuture<Ok = i32, Error = Whatever> {
    ///     api_function(arg)
    ///         .with_whatever_context(move |_| format!("The API failed for argument {arg}"))
    /// }
    ///
    /// # type ApiError = Box<dyn std::error::Error>;
    /// fn api_function(arg: &'static str) -> impl TryFuture<Ok = i32, Error = ApiError> {
    ///     /* ... */
    /// # futures::future::ok(42)
    /// }
    /// ```
    #[cfg(any(feature = "alloc", test))]
    fn with_whatever_context<F, S, E>(self, context: F) -> WithWhateverContext<Self, F, E>
    where
        F: FnOnce(&mut Self::Error) -> S,
        S: Into<String>,
        E: FromString;
}

impl<Fut> TryFutureExt for Fut
where
    Fut: TryFuture,
{
    fn context<C, E>(self, context: C) -> Context<Self, C, E>
    where
        C: IntoError<E, Source = Self::Error>,
        E: Error + ErrorCompat,
    {
        Context {
            inner: self,
            context: Some(context),
            _e: PhantomData,
        }
    }

    fn with_context<F, C, E>(self, context: F) -> WithContext<Self, F, E>
    where
        F: FnOnce(&mut Self::Error) -> C,
        C: IntoError<E, Source = Self::Error>,
        E: Error + ErrorCompat,
    {
        WithContext {
            inner: self,
            context: Some(context),
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
            context: Some(context),
            _e: PhantomData,
        }
    }

    #[cfg(any(feature = "alloc", test))]
    fn with_whatever_context<F, S, E>(self, context: F) -> WithWhateverContext<Self, F, E>
    where
        F: FnOnce(&mut Self::Error) -> S,
        S: Into<String>,
        E: FromString,
    {
        WithWhateverContext {
            inner: self,
            context: Some(context),
            _e: PhantomData,
        }
    }
}

/// Future for the [`context`](TryFutureExt::context) combinator.
///
/// See the [`TryFutureExt::context`] method for more details.
#[pin_project]
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct Context<Fut, C, E> {
    #[pin]
    inner: Fut,
    context: Option<C>,
    _e: PhantomData<E>,
}

impl<Fut, C, E> Future for Context<Fut, C, E>
where
    Fut: TryFuture,
    C: IntoError<E, Source = Fut::Error>,
    E: Error + ErrorCompat,
{
    type Output = Result<Fut::Ok, E>;

    #[track_caller]
    fn poll(self: Pin<&mut Self>, ctx: &mut TaskContext) -> Poll<Self::Output> {
        let this = self.project();
        let inner = this.inner;
        let context = this.context;

        // https://github.com/rust-lang/rust/issues/74042
        match inner.try_poll(ctx) {
            Poll::Ready(Ok(v)) => Poll::Ready(Ok(v)),
            Poll::Ready(Err(error)) => {
                let error = context
                    .take()
                    .expect("Cannot poll Context after it resolves")
                    .into_error(error);
                Poll::Ready(Err(error))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Future for the [`with_context`](TryFutureExt::with_context) combinator.
///
/// See the [`TryFutureExt::with_context`] method for more details.
#[pin_project]
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
pub struct WithContext<Fut, F, E> {
    #[pin]
    inner: Fut,
    context: Option<F>,
    _e: PhantomData<E>,
}

impl<Fut, F, C, E> Future for WithContext<Fut, F, E>
where
    Fut: TryFuture,
    F: FnOnce(&mut Fut::Error) -> C,
    C: IntoError<E, Source = Fut::Error>,
    E: Error + ErrorCompat,
{
    type Output = Result<Fut::Ok, E>;

    #[track_caller]
    fn poll(self: Pin<&mut Self>, ctx: &mut TaskContext) -> Poll<Self::Output> {
        let this = self.project();
        let inner = this.inner;
        let context = this.context;

        // https://github.com/rust-lang/rust/issues/74042
        match inner.try_poll(ctx) {
            Poll::Ready(Ok(v)) => Poll::Ready(Ok(v)),
            Poll::Ready(Err(mut error)) => {
                let context = context
                    .take()
                    .expect("Cannot poll WithContext after it resolves");

                let error = context(&mut error).into_error(error);

                Poll::Ready(Err(error))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Future for the
/// [`whatever_context`](TryFutureExt::whatever_context) combinator.
///
/// See the [`TryFutureExt::whatever_context`] method for more
/// details.
#[pin_project]
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
#[cfg(any(feature = "alloc", test))]
pub struct WhateverContext<Fut, S, E> {
    #[pin]
    inner: Fut,
    context: Option<S>,
    _e: PhantomData<E>,
}

#[cfg(any(feature = "alloc", test))]
impl<Fut, S, E> Future for WhateverContext<Fut, S, E>
where
    Fut: TryFuture,
    S: Into<String>,
    E: FromString,
    Fut::Error: Into<E::Source>,
{
    type Output = Result<Fut::Ok, E>;

    #[track_caller]
    fn poll(self: Pin<&mut Self>, ctx: &mut TaskContext) -> Poll<Self::Output> {
        let this = self.project();
        let inner = this.inner;
        let context = this.context;

        // https://github.com/rust-lang/rust/issues/74042
        match inner.try_poll(ctx) {
            Poll::Ready(Ok(v)) => Poll::Ready(Ok(v)),
            Poll::Ready(Err(error)) => {
                let context = context
                    .take()
                    .expect("Cannot poll WhateverContext after it resolves");
                let error = FromString::with_source(error.into(), context.into());

                Poll::Ready(Err(error))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}

/// Future for the
/// [`with_whatever_context`](TryFutureExt::with_whatever_context)
/// combinator.
///
/// See the [`TryFutureExt::with_whatever_context`] method for more
/// details.
#[pin_project]
#[derive(Debug)]
#[must_use = "futures do nothing unless polled"]
#[cfg(any(feature = "alloc", test))]
pub struct WithWhateverContext<Fut, F, E> {
    #[pin]
    inner: Fut,
    context: Option<F>,
    _e: PhantomData<E>,
}

#[cfg(any(feature = "alloc", test))]
impl<Fut, F, S, E> Future for WithWhateverContext<Fut, F, E>
where
    Fut: TryFuture,
    F: FnOnce(&mut Fut::Error) -> S,
    S: Into<String>,
    E: FromString,
    Fut::Error: Into<E::Source>,
{
    type Output = Result<Fut::Ok, E>;

    #[track_caller]
    fn poll(self: Pin<&mut Self>, ctx: &mut TaskContext) -> Poll<Self::Output> {
        let this = self.project();
        let inner = this.inner;
        let context = this.context;

        // https://github.com/rust-lang/rust/issues/74042
        match inner.try_poll(ctx) {
            Poll::Ready(Ok(v)) => Poll::Ready(Ok(v)),
            Poll::Ready(Err(mut error)) => {
                let context = context
                    .take()
                    .expect("Cannot poll WhateverContext after it resolves");
                let context = context(&mut error);
                let error = FromString::with_source(error.into(), context.into());

                Poll::Ready(Err(error))
            }
            Poll::Pending => Poll::Pending,
        }
    }
}
