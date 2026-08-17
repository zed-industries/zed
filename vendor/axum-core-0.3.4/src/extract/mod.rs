//! Types and traits for extracting data from requests.
//!
//! See [`axum::extract`] for more details.
//!
//! [`axum::extract`]: https://docs.rs/axum/latest/axum/extract/index.html

use crate::response::IntoResponse;
use async_trait::async_trait;
use http::{request::Parts, Request};
use std::convert::Infallible;

pub mod rejection;

mod default_body_limit;
mod from_ref;
mod request_parts;
mod tuple;

pub(crate) use self::default_body_limit::DefaultBodyLimitKind;
pub use self::{default_body_limit::DefaultBodyLimit, from_ref::FromRef};

mod private {
    #[derive(Debug, Clone, Copy)]
    pub enum ViaParts {}

    #[derive(Debug, Clone, Copy)]
    pub enum ViaRequest {}
}

/// Types that can be created from request parts.
///
/// Extractors that implement `FromRequestParts` cannot consume the request body and can thus be
/// run in any order for handlers.
///
/// If your extractor needs to consume the request body then you should implement [`FromRequest`]
/// and not [`FromRequestParts`].
///
/// See [`axum::extract`] for more general docs about extractors.
///
/// [`axum::extract`]: https://docs.rs/axum/0.6.0/axum/extract/index.html
#[async_trait]
#[cfg_attr(
    nightly_error_messages,
    rustc_on_unimplemented(
        note = "Function argument is not a valid axum extractor. \nSee `https://docs.rs/axum/latest/axum/extract/index.html` for details",
    )
)]
pub trait FromRequestParts<S>: Sized {
    /// If the extractor fails it'll use this "rejection" type. A rejection is
    /// a kind of error that can be converted into a response.
    type Rejection: IntoResponse;

    /// Perform the extraction.
    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection>;
}

/// Types that can be created from requests.
///
/// Extractors that implement `FromRequest` can consume the request body and can thus only be run
/// once for handlers.
///
/// If your extractor doesn't need to consume the request body then you should implement
/// [`FromRequestParts`] and not [`FromRequest`].
///
/// See [`axum::extract`] for more general docs about extractors.
///
/// # What is the `B` type parameter?
///
/// `FromRequest` is generic over the request body (the `B` in
/// [`http::Request<B>`]). This is to allow `FromRequest` to be usable with any
/// type of request body. This is necessary because some middleware change the
/// request body, for example to add timeouts.
///
/// If you're writing your own `FromRequest` that wont be used outside your
/// application, and not using any middleware that changes the request body, you
/// can most likely use `axum::body::Body`.
///
/// If you're writing a library that's intended for others to use, it's recommended
/// to keep the generic type parameter:
///
/// ```rust
/// use axum::{
///     async_trait,
///     extract::FromRequest,
///     http::{self, Request},
/// };
///
/// struct MyExtractor;
///
/// #[async_trait]
/// impl<S, B> FromRequest<S, B> for MyExtractor
/// where
///     // these bounds are required by `async_trait`
///     B: Send + 'static,
///     S: Send + Sync,
/// {
///     type Rejection = http::StatusCode;
///
///     async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
///         // ...
///         # unimplemented!()
///     }
/// }
/// ```
///
/// This ensures your extractor is as flexible as possible.
///
/// [`http::Request<B>`]: http::Request
/// [`axum::extract`]: https://docs.rs/axum/0.6.0/axum/extract/index.html
#[async_trait]
#[cfg_attr(
    nightly_error_messages,
    rustc_on_unimplemented(
        note = "Function argument is not a valid axum extractor. \nSee `https://docs.rs/axum/latest/axum/extract/index.html` for details",
    )
)]
pub trait FromRequest<S, B, M = private::ViaRequest>: Sized {
    /// If the extractor fails it'll use this "rejection" type. A rejection is
    /// a kind of error that can be converted into a response.
    type Rejection: IntoResponse;

    /// Perform the extraction.
    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection>;
}

#[async_trait]
impl<S, B, T> FromRequest<S, B, private::ViaParts> for T
where
    B: Send + 'static,
    S: Send + Sync,
    T: FromRequestParts<S>,
{
    type Rejection = <Self as FromRequestParts<S>>::Rejection;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        let (mut parts, _) = req.into_parts();
        Self::from_request_parts(&mut parts, state).await
    }
}

#[async_trait]
impl<S, T> FromRequestParts<S> for Option<T>
where
    T: FromRequestParts<S>,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(
        parts: &mut Parts,
        state: &S,
    ) -> Result<Option<T>, Self::Rejection> {
        Ok(T::from_request_parts(parts, state).await.ok())
    }
}

#[async_trait]
impl<S, T, B> FromRequest<S, B> for Option<T>
where
    T: FromRequest<S, B>,
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request(req: Request<B>, state: &S) -> Result<Option<T>, Self::Rejection> {
        Ok(T::from_request(req, state).await.ok())
    }
}

#[async_trait]
impl<S, T> FromRequestParts<S> for Result<T, T::Rejection>
where
    T: FromRequestParts<S>,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
        Ok(T::from_request_parts(parts, state).await)
    }
}

#[async_trait]
impl<S, T, B> FromRequest<S, B> for Result<T, T::Rejection>
where
    T: FromRequest<S, B>,
    B: Send + 'static,
    S: Send + Sync,
{
    type Rejection = Infallible;

    async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
        Ok(T::from_request(req, state).await)
    }
}
