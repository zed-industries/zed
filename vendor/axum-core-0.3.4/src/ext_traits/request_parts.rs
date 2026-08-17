use crate::extract::FromRequestParts;
use futures_util::future::BoxFuture;
use http::request::Parts;

mod sealed {
    pub trait Sealed {}
    impl Sealed for http::request::Parts {}
}

/// Extension trait that adds additional methods to [`Parts`].
pub trait RequestPartsExt: sealed::Sealed + Sized {
    /// Apply an extractor to this `Parts`.
    ///
    /// This is just a convenience for `E::from_request_parts(parts, &())`.
    ///
    /// # Example
    ///
    /// ```
    /// use axum::{
    ///     extract::{Query, TypedHeader, FromRequestParts},
    ///     response::{Response, IntoResponse},
    ///     headers::UserAgent,
    ///     http::request::Parts,
    ///     RequestPartsExt,
    ///     async_trait,
    /// };
    /// use std::collections::HashMap;
    ///
    /// struct MyExtractor {
    ///     user_agent: String,
    ///     query_params: HashMap<String, String>,
    /// }
    ///
    /// #[async_trait]
    /// impl<S> FromRequestParts<S> for MyExtractor
    /// where
    ///     S: Send + Sync,
    /// {
    ///     type Rejection = Response;
    ///
    ///     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    ///         let user_agent = parts
    ///             .extract::<TypedHeader<UserAgent>>()
    ///             .await
    ///             .map(|user_agent| user_agent.as_str().to_owned())
    ///             .map_err(|err| err.into_response())?;
    ///
    ///         let query_params = parts
    ///             .extract::<Query<HashMap<String, String>>>()
    ///             .await
    ///             .map(|Query(params)| params)
    ///             .map_err(|err| err.into_response())?;
    ///
    ///         Ok(MyExtractor { user_agent, query_params })
    ///     }
    /// }
    /// ```
    fn extract<E>(&mut self) -> BoxFuture<'_, Result<E, E::Rejection>>
    where
        E: FromRequestParts<()> + 'static;

    /// Apply an extractor that requires some state to this `Parts`.
    ///
    /// This is just a convenience for `E::from_request_parts(parts, state)`.
    ///
    /// # Example
    ///
    /// ```
    /// use axum::{
    ///     extract::{FromRef, FromRequestParts},
    ///     response::{Response, IntoResponse},
    ///     http::request::Parts,
    ///     RequestPartsExt,
    ///     async_trait,
    /// };
    ///
    /// struct MyExtractor {
    ///     requires_state: RequiresState,
    /// }
    ///
    /// #[async_trait]
    /// impl<S> FromRequestParts<S> for MyExtractor
    /// where
    ///     String: FromRef<S>,
    ///     S: Send + Sync,
    /// {
    ///     type Rejection = std::convert::Infallible;
    ///
    ///     async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    ///         let requires_state = parts
    ///             .extract_with_state::<RequiresState, _>(state)
    ///             .await?;
    ///
    ///         Ok(MyExtractor { requires_state })
    ///     }
    /// }
    ///
    /// struct RequiresState { /* ... */ }
    ///
    /// // some extractor that requires a `String` in the state
    /// #[async_trait]
    /// impl<S> FromRequestParts<S> for RequiresState
    /// where
    ///     String: FromRef<S>,
    ///     S: Send + Sync,
    /// {
    ///     // ...
    ///     # type Rejection = std::convert::Infallible;
    ///     # async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
    ///     #     unimplemented!()
    ///     # }
    /// }
    /// ```
    fn extract_with_state<'a, E, S>(
        &'a mut self,
        state: &'a S,
    ) -> BoxFuture<'a, Result<E, E::Rejection>>
    where
        E: FromRequestParts<S> + 'static,
        S: Send + Sync;
}

impl RequestPartsExt for Parts {
    fn extract<E>(&mut self) -> BoxFuture<'_, Result<E, E::Rejection>>
    where
        E: FromRequestParts<()> + 'static,
    {
        self.extract_with_state(&())
    }

    fn extract_with_state<'a, E, S>(
        &'a mut self,
        state: &'a S,
    ) -> BoxFuture<'a, Result<E, E::Rejection>>
    where
        E: FromRequestParts<S> + 'static,
        S: Send + Sync,
    {
        E::from_request_parts(self, state)
    }
}

#[cfg(test)]
mod tests {
    use std::convert::Infallible;

    use super::*;
    use crate::{
        ext_traits::tests::{RequiresState, State},
        extract::FromRef,
    };
    use async_trait::async_trait;
    use http::{Method, Request};

    #[tokio::test]
    async fn extract_without_state() {
        let (mut parts, _) = Request::new(()).into_parts();

        let method: Method = parts.extract().await.unwrap();

        assert_eq!(method, Method::GET);
    }

    #[tokio::test]
    async fn extract_with_state() {
        let (mut parts, _) = Request::new(()).into_parts();

        let state = "state".to_owned();

        let State(extracted_state): State<String> = parts
            .extract_with_state::<State<String>, String>(&state)
            .await
            .unwrap();

        assert_eq!(extracted_state, state);
    }

    // this stuff just needs to compile
    #[allow(dead_code)]
    struct WorksForCustomExtractor {
        method: Method,
        from_state: String,
    }

    #[async_trait]
    impl<S> FromRequestParts<S> for WorksForCustomExtractor
    where
        S: Send + Sync,
        String: FromRef<S>,
    {
        type Rejection = Infallible;

        async fn from_request_parts(parts: &mut Parts, state: &S) -> Result<Self, Self::Rejection> {
            let RequiresState(from_state) = parts.extract_with_state(state).await?;
            let method = parts.extract().await?;

            Ok(Self { method, from_state })
        }
    }
}
