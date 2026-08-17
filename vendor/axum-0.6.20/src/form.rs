use crate::body::HttpBody;
use crate::extract::{rejection::*, FromRequest, RawForm};
use crate::BoxError;
use async_trait::async_trait;
use axum_core::response::{IntoResponse, Response};
use axum_core::RequestExt;
use http::header::CONTENT_TYPE;
use http::{Request, StatusCode};
use serde::de::DeserializeOwned;
use serde::Serialize;

/// URL encoded extractor and response.
///
/// # As extractor
///
/// If used as an extractor `Form` will deserialize the query parameters for `GET` and `HEAD`
/// requests and `application/x-www-form-urlencoded` encoded request bodies for other methods. It
/// supports any type that implements [`serde::Deserialize`].
///
/// ⚠️ Since parsing form data might require consuming the request body, the `Form` extractor must be
/// *last* if there are multiple extractors in a handler. See ["the order of
/// extractors"][order-of-extractors]
///
/// [order-of-extractors]: crate::extract#the-order-of-extractors
///
/// ```rust
/// use axum::Form;
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct SignUp {
///     username: String,
///     password: String,
/// }
///
/// async fn accept_form(Form(sign_up): Form<SignUp>) {
///     // ...
/// }
/// ```
///
/// Note that `Content-Type: multipart/form-data` requests are not supported. Use [`Multipart`]
/// instead.
///
/// # As response
///
/// ```rust
/// use axum::Form;
/// use serde::Serialize;
///
/// #[derive(Serialize)]
/// struct Payload {
///     value: String,
/// }
///
/// async fn handler() -> Form<Payload> {
///     Form(Payload { value: "foo".to_owned() })
/// }
/// ```
///
/// [`Multipart`]: crate::extract::Multipart
#[cfg_attr(docsrs, doc(cfg(feature = "form")))]
#[derive(Debug, Clone, Copy, Default)]
#[must_use]
pub struct Form<T>(pub T);

#[async_trait]
impl<T, S, B> FromRequest<S, B> for Form<T>
where
    T: DeserializeOwned,
    B: HttpBody + Send + 'static,
    B::Data: Send,
    B::Error: Into<BoxError>,
    S: Send + Sync,
{
    type Rejection = FormRejection;

    async fn from_request(req: Request<B>, _state: &S) -> Result<Self, Self::Rejection> {
        let is_get_or_head =
            req.method() == http::Method::GET || req.method() == http::Method::HEAD;

        match req.extract().await {
            Ok(RawForm(bytes)) => {
                let value =
                    serde_urlencoded::from_bytes(&bytes).map_err(|err| -> FormRejection {
                        if is_get_or_head {
                            FailedToDeserializeForm::from_err(err).into()
                        } else {
                            FailedToDeserializeFormBody::from_err(err).into()
                        }
                    })?;
                Ok(Form(value))
            }
            Err(RawFormRejection::BytesRejection(r)) => Err(FormRejection::BytesRejection(r)),
            Err(RawFormRejection::InvalidFormContentType(r)) => {
                Err(FormRejection::InvalidFormContentType(r))
            }
        }
    }
}

impl<T> IntoResponse for Form<T>
where
    T: Serialize,
{
    fn into_response(self) -> Response {
        match serde_urlencoded::to_string(&self.0) {
            Ok(body) => (
                [(CONTENT_TYPE, mime::APPLICATION_WWW_FORM_URLENCODED.as_ref())],
                body,
            )
                .into_response(),
            Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, err.to_string()).into_response(),
        }
    }
}

axum_core::__impl_deref!(Form);

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{
        body::{Empty, Full},
        routing::{on, MethodFilter},
        test_helpers::TestClient,
        Router,
    };
    use bytes::Bytes;
    use http::{header::CONTENT_TYPE, Method, Request};
    use mime::APPLICATION_WWW_FORM_URLENCODED;
    use serde::{Deserialize, Serialize};
    use std::fmt::Debug;

    #[derive(Debug, PartialEq, Serialize, Deserialize)]
    struct Pagination {
        size: Option<u64>,
        page: Option<u64>,
    }

    async fn check_query<T: DeserializeOwned + PartialEq + Debug>(uri: impl AsRef<str>, value: T) {
        let req = Request::builder()
            .uri(uri.as_ref())
            .body(Empty::<Bytes>::new())
            .unwrap();
        assert_eq!(Form::<T>::from_request(req, &()).await.unwrap().0, value);
    }

    async fn check_body<T: Serialize + DeserializeOwned + PartialEq + Debug>(value: T) {
        let req = Request::builder()
            .uri("http://example.com/test")
            .method(Method::POST)
            .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
            .body(Full::<Bytes>::new(
                serde_urlencoded::to_string(&value).unwrap().into(),
            ))
            .unwrap();
        assert_eq!(Form::<T>::from_request(req, &()).await.unwrap().0, value);
    }

    #[crate::test]
    async fn test_form_query() {
        check_query(
            "http://example.com/test",
            Pagination {
                size: None,
                page: None,
            },
        )
        .await;

        check_query(
            "http://example.com/test?size=10",
            Pagination {
                size: Some(10),
                page: None,
            },
        )
        .await;

        check_query(
            "http://example.com/test?size=10&page=20",
            Pagination {
                size: Some(10),
                page: Some(20),
            },
        )
        .await;
    }

    #[crate::test]
    async fn test_form_body() {
        check_body(Pagination {
            size: None,
            page: None,
        })
        .await;

        check_body(Pagination {
            size: Some(10),
            page: None,
        })
        .await;

        check_body(Pagination {
            size: Some(10),
            page: Some(20),
        })
        .await;
    }

    #[crate::test]
    async fn test_incorrect_content_type() {
        let req = Request::builder()
            .uri("http://example.com/test")
            .method(Method::POST)
            .header(CONTENT_TYPE, mime::APPLICATION_JSON.as_ref())
            .body(Full::<Bytes>::new(
                serde_urlencoded::to_string(&Pagination {
                    size: Some(10),
                    page: None,
                })
                .unwrap()
                .into(),
            ))
            .unwrap();
        assert!(matches!(
            Form::<Pagination>::from_request(req, &())
                .await
                .unwrap_err(),
            FormRejection::InvalidFormContentType(InvalidFormContentType)
        ));
    }

    #[tokio::test]
    async fn deserialize_error_status_codes() {
        #[allow(dead_code)]
        #[derive(Deserialize)]
        struct Payload {
            a: i32,
        }

        let app = Router::new().route(
            "/",
            on(
                MethodFilter::GET | MethodFilter::POST,
                |_: Form<Payload>| async {},
            ),
        );

        let client = TestClient::new(app);

        let res = client.get("/?a=false").send().await;
        assert_eq!(res.status(), StatusCode::BAD_REQUEST);

        let res = client
            .post("/")
            .header(CONTENT_TYPE, APPLICATION_WWW_FORM_URLENCODED.as_ref())
            .body("a=false")
            .send()
            .await;
        assert_eq!(res.status(), StatusCode::UNPROCESSABLE_ENTITY);
    }
}
