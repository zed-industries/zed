use crate::extract::{DefaultBodyLimitKind, FromRequest, FromRequestParts};
use futures_util::future::BoxFuture;
use http::Request;
use http_body::Limited;

mod sealed {
    pub trait Sealed<B> {}
    impl<B> Sealed<B> for http::Request<B> {}
}

/// Extension trait that adds additional methods to [`Request`].
pub trait RequestExt<B>: sealed::Sealed<B> + Sized {
    /// Apply an extractor to this `Request`.
    ///
    /// This is just a convenience for `E::from_request(req, &())`.
    ///
    /// Note this consumes the request. Use [`RequestExt::extract_parts`] if you're not extracting
    /// the body and don't want to consume the request.
    ///
    /// # Example
    ///
    /// ```
    /// use axum::{
    ///     async_trait,
    ///     extract::FromRequest,
    ///     http::{header::CONTENT_TYPE, Request, StatusCode},
    ///     response::{IntoResponse, Response},
    ///     Form, Json, RequestExt,
    /// };
    ///
    /// struct FormOrJson<T>(T);
    ///
    /// #[async_trait]
    /// impl<S, B, T> FromRequest<S, B> for FormOrJson<T>
    /// where
    ///     Json<T>: FromRequest<(), B>,
    ///     Form<T>: FromRequest<(), B>,
    ///     T: 'static,
    ///     B: Send + 'static,
    ///     S: Send + Sync,
    /// {
    ///     type Rejection = Response;
    ///
    ///     async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
    ///         let content_type = req
    ///             .headers()
    ///             .get(CONTENT_TYPE)
    ///             .and_then(|value| value.to_str().ok())
    ///             .ok_or_else(|| StatusCode::BAD_REQUEST.into_response())?;
    ///
    ///         if content_type.starts_with("application/json") {
    ///             let Json(payload) = req
    ///                 .extract::<Json<T>, _>()
    ///                 .await
    ///                 .map_err(|err| err.into_response())?;
    ///
    ///             Ok(Self(payload))
    ///         } else if content_type.starts_with("application/x-www-form-urlencoded") {
    ///             let Form(payload) = req
    ///                 .extract::<Form<T>, _>()
    ///                 .await
    ///                 .map_err(|err| err.into_response())?;
    ///
    ///             Ok(Self(payload))
    ///         } else {
    ///             Err(StatusCode::BAD_REQUEST.into_response())
    ///         }
    ///     }
    /// }
    /// ```
    fn extract<E, M>(self) -> BoxFuture<'static, Result<E, E::Rejection>>
    where
        E: FromRequest<(), B, M> + 'static,
        M: 'static;

    /// Apply an extractor that requires some state to this `Request`.
    ///
    /// This is just a convenience for `E::from_request(req, state)`.
    ///
    /// Note this consumes the request. Use [`RequestExt::extract_parts_with_state`] if you're not
    /// extracting the body and don't want to consume the request.
    ///
    /// # Example
    ///
    /// ```
    /// use axum::{
    ///     async_trait,
    ///     extract::{FromRef, FromRequest},
    ///     http::Request,
    ///     RequestExt,
    /// };
    ///
    /// struct MyExtractor {
    ///     requires_state: RequiresState,
    /// }
    ///
    /// #[async_trait]
    /// impl<S, B> FromRequest<S, B> for MyExtractor
    /// where
    ///     String: FromRef<S>,
    ///     S: Send + Sync,
    ///     B: Send + 'static,
    /// {
    ///     type Rejection = std::convert::Infallible;
    ///
    ///     async fn from_request(req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
    ///         let requires_state = req.extract_with_state::<RequiresState, _, _>(state).await?;
    ///
    ///         Ok(Self { requires_state })
    ///     }
    /// }
    ///
    /// // some extractor that consumes the request body and requires state
    /// struct RequiresState { /* ... */ }
    ///
    /// #[async_trait]
    /// impl<S, B> FromRequest<S, B> for RequiresState
    /// where
    ///     String: FromRef<S>,
    ///     S: Send + Sync,
    ///     B: Send + 'static,
    /// {
    ///     // ...
    ///     # type Rejection = std::convert::Infallible;
    ///     # async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
    ///     #     todo!()
    ///     # }
    /// }
    /// ```
    fn extract_with_state<E, S, M>(self, state: &S) -> BoxFuture<'_, Result<E, E::Rejection>>
    where
        E: FromRequest<S, B, M> + 'static,
        S: Send + Sync;

    /// Apply a parts extractor to this `Request`.
    ///
    /// This is just a convenience for `E::from_request_parts(parts, state)`.
    ///
    /// # Example
    ///
    /// ```
    /// use axum::{
    ///     async_trait,
    ///     extract::FromRequest,
    ///     headers::{authorization::Bearer, Authorization},
    ///     http::Request,
    ///     response::{IntoResponse, Response},
    ///     Json, RequestExt, TypedHeader,
    /// };
    ///
    /// struct MyExtractor<T> {
    ///     bearer_token: String,
    ///     payload: T,
    /// }
    ///
    /// #[async_trait]
    /// impl<S, B, T> FromRequest<S, B> for MyExtractor<T>
    /// where
    ///     B: Send + 'static,
    ///     S: Send + Sync,
    ///     Json<T>: FromRequest<(), B>,
    ///     T: 'static,
    /// {
    ///     type Rejection = Response;
    ///
    ///     async fn from_request(mut req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
    ///         let TypedHeader(auth_header) = req
    ///             .extract_parts::<TypedHeader<Authorization<Bearer>>>()
    ///             .await
    ///             .map_err(|err| err.into_response())?;
    ///
    ///         let Json(payload) = req
    ///             .extract::<Json<T>, _>()
    ///             .await
    ///             .map_err(|err| err.into_response())?;
    ///
    ///         Ok(Self {
    ///             bearer_token: auth_header.token().to_owned(),
    ///             payload,
    ///         })
    ///     }
    /// }
    /// ```
    fn extract_parts<E>(&mut self) -> BoxFuture<'_, Result<E, E::Rejection>>
    where
        E: FromRequestParts<()> + 'static;

    /// Apply a parts extractor that requires some state to this `Request`.
    ///
    /// This is just a convenience for `E::from_request_parts(parts, state)`.
    ///
    /// # Example
    ///
    /// ```
    /// use axum::{
    ///     async_trait,
    ///     extract::{FromRef, FromRequest, FromRequestParts},
    ///     http::{request::Parts, Request},
    ///     response::{IntoResponse, Response},
    ///     Json, RequestExt,
    /// };
    ///
    /// struct MyExtractor<T> {
    ///     requires_state: RequiresState,
    ///     payload: T,
    /// }
    ///
    /// #[async_trait]
    /// impl<S, B, T> FromRequest<S, B> for MyExtractor<T>
    /// where
    ///     String: FromRef<S>,
    ///     Json<T>: FromRequest<(), B>,
    ///     T: 'static,
    ///     S: Send + Sync,
    ///     B: Send + 'static,
    /// {
    ///     type Rejection = Response;
    ///
    ///     async fn from_request(mut req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
    ///         let requires_state = req
    ///             .extract_parts_with_state::<RequiresState, _>(state)
    ///             .await
    ///             .map_err(|err| err.into_response())?;
    ///
    ///         let Json(payload) = req
    ///             .extract::<Json<T>, _>()
    ///             .await
    ///             .map_err(|err| err.into_response())?;
    ///
    ///         Ok(Self {
    ///             requires_state,
    ///             payload,
    ///         })
    ///     }
    /// }
    ///
    /// struct RequiresState {}
    ///
    /// #[async_trait]
    /// impl<S> FromRequestParts<S> for RequiresState
    /// where
    ///     String: FromRef<S>,
    ///     S: Send + Sync,
    /// {
    ///     // ...
    ///     # type Rejection = std::convert::Infallible;
    ///     # async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    ///     #     todo!()
    ///     # }
    /// }
    /// ```
    fn extract_parts_with_state<'a, E, S>(
        &'a mut self,
        state: &'a S,
    ) -> BoxFuture<'a, Result<E, E::Rejection>>
    where
        E: FromRequestParts<S> + 'static,
        S: Send + Sync;

    /// Apply the [default body limit](crate::extract::DefaultBodyLimit).
    ///
    /// If it is disabled, return the request as-is in `Err`.
    fn with_limited_body(self) -> Result<Request<Limited<B>>, Request<B>>;

    /// Consumes the request, returning the body wrapped in [`Limited`] if a
    /// [default limit](crate::extract::DefaultBodyLimit) is in place, or not wrapped if the
    /// default limit is disabled.
    fn into_limited_body(self) -> Result<Limited<B>, B>;
}

impl<B> RequestExt<B> for Request<B>
where
    B: Send + 'static,
{
    fn extract<E, M>(self) -> BoxFuture<'static, Result<E, E::Rejection>>
    where
        E: FromRequest<(), B, M> + 'static,
        M: 'static,
    {
        self.extract_with_state(&())
    }

    fn extract_with_state<E, S, M>(self, state: &S) -> BoxFuture<'_, Result<E, E::Rejection>>
    where
        E: FromRequest<S, B, M> + 'static,
        S: Send + Sync,
    {
        E::from_request(self, state)
    }

    fn extract_parts<E>(&mut self) -> BoxFuture<'_, Result<E, E::Rejection>>
    where
        E: FromRequestParts<()> + 'static,
    {
        self.extract_parts_with_state(&())
    }

    fn extract_parts_with_state<'a, E, S>(
        &'a mut self,
        state: &'a S,
    ) -> BoxFuture<'a, Result<E, E::Rejection>>
    where
        E: FromRequestParts<S> + 'static,
        S: Send + Sync,
    {
        let mut req = Request::new(());
        *req.version_mut() = self.version();
        *req.method_mut() = self.method().clone();
        *req.uri_mut() = self.uri().clone();
        *req.headers_mut() = std::mem::take(self.headers_mut());
        *req.extensions_mut() = std::mem::take(self.extensions_mut());
        let (mut parts, _) = req.into_parts();

        Box::pin(async move {
            let result = E::from_request_parts(&mut parts, state).await;

            *self.version_mut() = parts.version;
            *self.method_mut() = parts.method.clone();
            *self.uri_mut() = parts.uri.clone();
            *self.headers_mut() = std::mem::take(&mut parts.headers);
            *self.extensions_mut() = std::mem::take(&mut parts.extensions);

            result
        })
    }

    fn with_limited_body(self) -> Result<Request<Limited<B>>, Request<B>> {
        // update docs in `axum-core/src/extract/default_body_limit.rs` and
        // `axum/src/docs/extract.md` if this changes
        const DEFAULT_LIMIT: usize = 2_097_152; // 2 mb

        match self.extensions().get::<DefaultBodyLimitKind>().copied() {
            Some(DefaultBodyLimitKind::Disable) => Err(self),
            Some(DefaultBodyLimitKind::Limit(limit)) => {
                Ok(self.map(|b| http_body::Limited::new(b, limit)))
            }
            None => Ok(self.map(|b| http_body::Limited::new(b, DEFAULT_LIMIT))),
        }
    }

    fn into_limited_body(self) -> Result<Limited<B>, B> {
        self.with_limited_body()
            .map(Request::into_body)
            .map_err(Request::into_body)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        ext_traits::tests::{RequiresState, State},
        extract::FromRef,
    };
    use async_trait::async_trait;
    use http::Method;
    use hyper::Body;

    #[tokio::test]
    async fn extract_without_state() {
        let req = Request::new(());

        let method: Method = req.extract().await.unwrap();

        assert_eq!(method, Method::GET);
    }

    #[tokio::test]
    async fn extract_body_without_state() {
        let req = Request::new(Body::from("foobar"));

        let body: String = req.extract().await.unwrap();

        assert_eq!(body, "foobar");
    }

    #[tokio::test]
    async fn extract_with_state() {
        let req = Request::new(());

        let state = "state".to_owned();

        let State(extracted_state): State<String> = req.extract_with_state(&state).await.unwrap();

        assert_eq!(extracted_state, state);
    }

    #[tokio::test]
    async fn extract_parts_without_state() {
        let mut req = Request::builder().header("x-foo", "foo").body(()).unwrap();

        let method: Method = req.extract_parts().await.unwrap();

        assert_eq!(method, Method::GET);
        assert_eq!(req.headers()["x-foo"], "foo");
    }

    #[tokio::test]
    async fn extract_parts_with_state() {
        let mut req = Request::builder().header("x-foo", "foo").body(()).unwrap();

        let state = "state".to_owned();

        let State(extracted_state): State<String> =
            req.extract_parts_with_state(&state).await.unwrap();

        assert_eq!(extracted_state, state);
        assert_eq!(req.headers()["x-foo"], "foo");
    }

    // this stuff just needs to compile
    #[allow(dead_code)]
    struct WorksForCustomExtractor {
        method: Method,
        from_state: String,
        body: String,
    }

    #[async_trait]
    impl<S, B> FromRequest<S, B> for WorksForCustomExtractor
    where
        S: Send + Sync,
        B: Send + 'static,
        String: FromRef<S> + FromRequest<(), B>,
    {
        type Rejection = <String as FromRequest<(), B>>::Rejection;

        async fn from_request(mut req: Request<B>, state: &S) -> Result<Self, Self::Rejection> {
            let RequiresState(from_state) = req.extract_parts_with_state(state).await.unwrap();
            let method = req.extract_parts().await.unwrap();
            let body = req.extract().await?;

            Ok(Self {
                method,
                from_state,
                body,
            })
        }
    }
}
