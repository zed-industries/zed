use crate::extract::FromRequestParts;
use async_trait::async_trait;
use axum_core::response::{IntoResponse, IntoResponseParts, Response, ResponseParts};
use headers::HeaderMapExt;
use http::request::Parts;
use std::convert::Infallible;

/// Extractor and response that works with typed header values from [`headers`].
///
/// # As extractor
///
/// In general, it's recommended to extract only the needed headers via `TypedHeader` rather than
/// removing all headers with the `HeaderMap` extractor.
///
/// ```rust,no_run
/// use axum::{
///     TypedHeader,
///     headers::UserAgent,
///     routing::get,
///     Router,
/// };
///
/// async fn users_teams_show(
///     TypedHeader(user_agent): TypedHeader<UserAgent>,
/// ) {
///     // ...
/// }
///
/// let app = Router::new().route("/users/:user_id/team/:team_id", get(users_teams_show));
/// # async {
/// # axum::Server::bind(&"".parse().unwrap()).serve(app.into_make_service()).await.unwrap();
/// # };
/// ```
///
/// # As response
///
/// ```rust
/// use axum::{
///     TypedHeader,
///     response::IntoResponse,
///     headers::ContentType,
/// };
///
/// async fn handler() -> (TypedHeader<ContentType>, &'static str) {
///     (
///         TypedHeader(ContentType::text_utf8()),
///         "Hello, World!",
///     )
/// }
/// ```
#[cfg(feature = "headers")]
#[derive(Debug, Clone, Copy)]
#[must_use]
pub struct TypedHeader<T>(pub T);

#[async_trait]
impl<T, S> FromRequestParts<S> for TypedHeader<T>
where
    T: headers::Header,
    S: Send + Sync,
{
    type Rejection = TypedHeaderRejection;

    async fn from_request_parts(parts: &mut Parts, _state: &S) -> Result<Self, Self::Rejection> {
        let mut values = parts.headers.get_all(T::name()).iter();
        let is_missing = values.size_hint() == (0, Some(0));
        T::decode(&mut values)
            .map(Self)
            .map_err(|err| TypedHeaderRejection {
                name: T::name(),
                reason: if is_missing {
                    // Report a more precise rejection for the missing header case.
                    TypedHeaderRejectionReason::Missing
                } else {
                    TypedHeaderRejectionReason::Error(err)
                },
            })
    }
}

axum_core::__impl_deref!(TypedHeader);

impl<T> IntoResponseParts for TypedHeader<T>
where
    T: headers::Header,
{
    type Error = Infallible;

    fn into_response_parts(self, mut res: ResponseParts) -> Result<ResponseParts, Self::Error> {
        res.headers_mut().typed_insert(self.0);
        Ok(res)
    }
}

impl<T> IntoResponse for TypedHeader<T>
where
    T: headers::Header,
{
    fn into_response(self) -> Response {
        let mut res = ().into_response();
        res.headers_mut().typed_insert(self.0);
        res
    }
}

/// Rejection used for [`TypedHeader`](super::TypedHeader).
#[cfg(feature = "headers")]
#[derive(Debug)]
pub struct TypedHeaderRejection {
    name: &'static http::header::HeaderName,
    reason: TypedHeaderRejectionReason,
}

impl TypedHeaderRejection {
    /// Name of the header that caused the rejection
    pub fn name(&self) -> &http::header::HeaderName {
        self.name
    }

    /// Reason why the header extraction has failed
    pub fn reason(&self) -> &TypedHeaderRejectionReason {
        &self.reason
    }
}

/// Additional information regarding a [`TypedHeaderRejection`]
#[cfg(feature = "headers")]
#[derive(Debug)]
#[non_exhaustive]
pub enum TypedHeaderRejectionReason {
    /// The header was missing from the HTTP request
    Missing,
    /// An error occured when parsing the header from the HTTP request
    Error(headers::Error),
}

impl IntoResponse for TypedHeaderRejection {
    fn into_response(self) -> Response {
        (http::StatusCode::BAD_REQUEST, self.to_string()).into_response()
    }
}

impl std::fmt::Display for TypedHeaderRejection {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match &self.reason {
            TypedHeaderRejectionReason::Missing => {
                write!(f, "Header of type `{}` was missing", self.name)
            }
            TypedHeaderRejectionReason::Error(err) => {
                write!(f, "{} ({})", err, self.name)
            }
        }
    }
}

impl std::error::Error for TypedHeaderRejection {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match &self.reason {
            TypedHeaderRejectionReason::Error(err) => Some(err),
            TypedHeaderRejectionReason::Missing => None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{response::IntoResponse, routing::get, test_helpers::*, Router};

    #[crate::test]
    async fn typed_header() {
        async fn handle(
            TypedHeader(user_agent): TypedHeader<headers::UserAgent>,
            TypedHeader(cookies): TypedHeader<headers::Cookie>,
        ) -> impl IntoResponse {
            let user_agent = user_agent.as_str();
            let cookies = cookies.iter().collect::<Vec<_>>();
            format!("User-Agent={user_agent:?}, Cookie={cookies:?}")
        }

        let app = Router::new().route("/", get(handle));

        let client = TestClient::new(app);

        let res = client
            .get("/")
            .header("user-agent", "foobar")
            .header("cookie", "a=1; b=2")
            .header("cookie", "c=3")
            .send()
            .await;
        let body = res.text().await;
        assert_eq!(
            body,
            r#"User-Agent="foobar", Cookie=[("a", "1"), ("b", "2"), ("c", "3")]"#
        );

        let res = client.get("/").header("user-agent", "foobar").send().await;
        let body = res.text().await;
        assert_eq!(body, r#"User-Agent="foobar", Cookie=[]"#);

        let res = client.get("/").header("cookie", "a=1").send().await;
        let body = res.text().await;
        assert_eq!(body, "Header of type `user-agent` was missing");
    }
}
